// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::cmp;
use std::collections::HashSet;
use std::mem;
use std::time::{Duration, Instant};

use crate::base::platform::TimeTicks;
use crate::heap::base::incremental_marking_schedule::IncrementalMarkingSchedule;
use crate::heap::cppgc::globals;
use crate::heap::cppgc::heap_config::CollectionType;
use crate::heap::cppgc::heap_config::MarkingConfig;
use crate::heap::cppgc::heap_config::StackState;
use crate::heap::cppgc::heap_object_header::HeapObjectHeader;
use crate::heap::cppgc::heap_page::BasePage;
use crate::heap::cppgc::heap::HeapBase;
use crate::heap::cppgc::liveness_broker::LivenessBroker;
use crate::heap::cppgc::marking_state::BasicMarkingState;
use crate::heap::cppgc::marking_visitor::MarkingVisitor;
use crate::heap::cppgc::marking_worklists::MarkingWorklists;
use crate::heap::cppgc::process_heap::ProcessGlobalLock;
use crate::heap::cppgc::root_marking_visitor::RootMarkingVisitor;
use crate::heap::cppgc::stats_collector::StatsCollector;
use crate::heap::cppgc::write_barrier::WriteBarrier;

// TODO(Rust): These includes require proper Rust implementation of these features
// #if defined(CPPGC_CAGED_HEAP)
// #include "include/cppgc/internal/caged-heap-local-data.h"
// #endif

pub mod internal {
    use super::*;

    mod private {
        use super::*;

        fn enter_incremental_marking_if_needed(config: MarkingConfig, heap: &mut HeapBase) -> bool {
            if config.marking_type == MarkingConfig::MarkingType::Incremental
                || config.marking_type == MarkingConfig::MarkingType::IncrementalAndConcurrent
            {
                WriteBarrier::FlagUpdater::enter();
                heap.set_incremental_marking_in_progress(true);
                true
            } else {
                false
            }
        }

        fn exit_incremental_marking_if_needed(config: MarkingConfig, heap: &mut HeapBase) -> bool {
            if config.marking_type == MarkingConfig::MarkingType::Incremental
                || config.marking_type == MarkingConfig::MarkingType::IncrementalAndConcurrent
            {
                WriteBarrier::FlagUpdater::exit();
                heap.set_incremental_marking_in_progress(false);
                true
            } else {
                false
            }
        }

        const DEFAULT_DEADLINE_CHECK_INTERVAL: usize = 150;

        fn drain_worklist_with_bytes_and_time_deadline<
            const SCOPE_ID: usize,
            const DEADLINE_CHECK_INTERVAL: usize,
            WorklistLocal,
            Callback,
        >(
            stats_collector: &mut StatsCollector,
            marking_state: &mut dyn BasicMarkingState,
            marked_bytes_deadline: usize,
            time_deadline: TimeTicks,
            worklist_local: &mut WorklistLocal,
            callback: Callback,
        ) -> bool
        where
            WorklistLocal: DrainableWorklist,
            Callback: FnMut(&mut WorklistLocal::Item) -> (),
        {
            drain_worklist_with_predicate::<DEADLINE_CHECK_INTERVAL, WorklistLocal, Callback>(
                || {
                    marked_bytes_deadline <= marking_state.marked_bytes()
                        || time_deadline <= TimeTicks::now()
                },
                || StatsCollector::DisabledScope::new(stats_collector, SCOPE_ID),
                worklist_local,
                callback,
            )
        }

        trait DrainableWorklist {
            type Item;
            type Local<'a>: LocalWorklist<'a, Item = Self::Item>
            where
                Self: 'a;
            fn local(&mut self) -> Self::Local<'_>;
        }

        trait LocalWorklist<'a> {
            type Item;
            fn pop(&mut self) -> Option<Self::Item>;
        }

        fn drain_worklist_with_predicate<
            const DEADLINE_CHECK_INTERVAL: usize,
            WorklistLocal,
            Callback,
        >(
            predicate: impl Fn() -> bool,
            scope_factory: impl Fn() -> StatsCollector::DisabledScope,
            worklist_local: &mut WorklistLocal,
            mut callback: Callback,
        ) -> bool
        where
            WorklistLocal: DrainableWorklist,
            Callback: FnMut(&mut WorklistLocal::Item) -> (),
        {
            let mut i = 0;
            while let Some(mut item) = worklist_local.local().pop() {
                if i == DEADLINE_CHECK_INTERVAL {
                    if predicate() {
                        return false;
                    }
                    i = 0;
                }
                let _scope = scope_factory();
                callback(&mut item);
                i += 1;
            }
            true
        }

        fn get_next_incremental_step_duration(
            schedule: &mut IncrementalMarkingSchedule,
            heap: &HeapBase,
        ) -> usize {
            schedule.get_next_incremental_step_duration(heap.stats_collector().allocated_object_size())
        }
    }

    pub use private::*;

    const MIN_ALLOCATED_BYTES_PER_STEP: usize = 1024 * 1024;

    pub struct MarkerBase {
        heap_: HeapBase,
        config_: MarkingConfig,
        platform_: *mut dyn Platform, //TODO(Rust): Add Platform trait
        foreground_task_runner_: Option<Box<dyn TaskRunner>>, //TODO(Rust): Add TaskRunner trait
        incremental_marking_allocation_observer_: IncrementalMarkingAllocationObserver,
        mutator_marking_state_: MutatorMarkingState,
        marking_worklists_: MarkingWorklists,
        incremental_marking_handle_: IncrementalMarkingTaskHandle,
        main_marking_disabled_for_testing_: bool,
        last_bytes_marked_: usize,
        processed_cross_thread_weakness_: bool,
        visited_cross_thread_persistents_in_atomic_pause_: bool,
    }

    impl MarkerBase {
        pub const MAXIMUM_INCREMENTAL_STEP_DURATION: Duration = Duration::from_secs(3);

        pub fn new(heap: HeapBase, platform: *mut dyn Platform, config: MarkingConfig) -> Self {
            let foreground_task_runner = unsafe { (*platform).get_foreground_task_runner() };
            MarkerBase {
                heap_: heap,
                config_: config,
                platform_: platform,
                foreground_task_runner_: foreground_task_runner,
                incremental_marking_allocation_observer_:
                    IncrementalMarkingAllocationObserver::new(),
                mutator_marking_state_: MutatorMarkingState::new(),
                marking_worklists_: MarkingWorklists::new(),
                incremental_marking_handle_: IncrementalMarkingTaskHandle::Empty,
                main_marking_disabled_for_testing_: false,
                last_bytes_marked_: 0,
                processed_cross_thread_weakness_: false,
                visited_cross_thread_persistents_in_atomic_pause_: false,
            }
        }

        pub fn heap(&self) -> &HeapBase {
            &self.heap_
        }

        pub fn schedule(&mut self) -> &mut IncrementalMarkingSchedule {
            todo!() //&mut self.schedule_
        }

        pub fn concurrent_marker(&self) -> &ConcurrentMarkerBase {
            todo!() //&self.concurrent_marker_
        }

        pub fn stats_collector(&self) -> &StatsCollector {
            todo!() //self.heap_.stats_collector()
        }

        pub fn AdvanceMarkingWithLimitsEpilogue(&self) {
            todo!()
        }

        pub fn AdvanceMarkingOnAllocationImpl(&self) {
            todo!()
        }

        pub fn conservative_visitor(&self) -> &ConservativeMarkingVisitor {
            todo!()
        }
        pub fn visitor(&self) -> &MarkingVisitor {
            todo!()
        }

        pub fn mutator_marking_state(&self) -> &MutatorMarkingState {
            &self.mutator_marking_state_
        }
    }

    impl Drop for MarkerBase {
        fn drop(&mut self) {
            // The fixed point iteration may have found not-fully-constructed objects.
            // Such objects should have already been found through the stack scan though
            // and should thus already be marked.
            if !self
                .marking_worklists_
                .not_fully_constructed_worklist()
                .is_empty()
            {
                //TODO(Rust): Add DEBUG check
                // #if DEBUG
                //     DCHECK_NE(StackState::kNoHeapPointers, config_.stack_state);
                //     std::unordered_set<HeapObjectHeader*> objects =
                //         mutator_marking_state_.not_fully_constructed_worklist().Extract();
                //     for (HeapObjectHeader* object : objects) DCHECK(object->IsMarked());
                // #else
                self.marking_worklists_
                    .not_fully_constructed_worklist()
                    .clear();
                // #endif
            }

            // |discovered_ephemeron_pairs_worklist_| may still hold ephemeron pairs with
            // dead keys.
            if !self
                .marking_worklists_
                .discovered_ephemeron_pairs_worklist()
                .is_empty()
            {
                //TODO(Rust): Add DEBUG check
                // #if DEBUG
                // MarkingWorklists::EphemeronPairItem item;
                // while (mutator_marking_state_.discovered_ephemeron_pairs_worklist().Pop(
                //     &item)) {
                //   DCHECK(!HeapObjectHeader::FromObject(item.key).IsMarked());
                // }
                // #else
                self.marking_worklists_
                    .discovered_ephemeron_pairs_worklist()
                    .clear();
                // #endif
            }

            self.marking_worklists_.weak_containers_worklist().clear();
        }
    }

    // TODO(Rust): Needs proper task/task runner implementations.
    #[derive(Debug)]
    struct IncrementalMarkingTask {}

    impl IncrementalMarkingTask {
        fn post(_runner: &dyn TaskRunner, _marker: &MarkerBase) -> IncrementalMarkingTaskHandle {
            IncrementalMarkingTaskHandle::Empty
        }
        fn run(&self, _marker: &mut MarkerBase, _stack_state: StackState) {}
    }

    #[derive(Debug)]
    enum IncrementalMarkingTaskHandle {
        Empty,
        //TODO(Rust): Add CancelableTask
        NonEmpty,
    }

    impl IncrementalMarkingTaskHandle {
        fn cancel_if_non_empty(&mut self) {
            *self = IncrementalMarkingTaskHandle::Empty;
        }

        fn is_canceled(&self) -> bool {
            match self {
                IncrementalMarkingTaskHandle::Empty => true,
                IncrementalMarkingTaskHandle::NonEmpty => false,
            }
        }
    }

    struct IncrementalMarkingAllocationObserver {
        current_allocated_size_: usize,
    }

    impl IncrementalMarkingAllocationObserver {
        fn new() -> Self {
            IncrementalMarkingAllocationObserver {
                current_allocated_size_: 0,
            }
        }
        fn allocated_object_size_increased(&mut self, marker: &mut MarkerBase, delta: usize) {
            self.current_allocated_size_ += delta;
            if self.current_allocated_size_ > MIN_ALLOCATED_BYTES_PER_STEP {
                marker.AdvanceMarkingOnAllocation();
                self.current_allocated_size_ = 0;
            }
        }
    }

    impl MarkerBase {
        pub fn start_marking(&mut self) {
            if self.is_marking_ {
                return;
            }
            let stats_scope = StatsCollector::EnabledScope::new(
                self.heap().stats_collector(),
                if self.config_.marking_type == MarkingConfig::MarkingType::Atomic {
                    StatsCollector::ATOMIC_MARK
                } else {
                    StatsCollector::INCREMENTAL_MARK
                },
            );

            self.heap().stats_collector().notify_marking_started(
                self.config_.collection_type,
                self.config_.marking_type,
                self.config_.is_forced_gc,
            );

            self.is_marking_ = true;
            if enter_incremental_marking_if_needed(self.config_, &mut self.heap_) {
                let inner_stats_scope = StatsCollector::EnabledScope::new(
                    self.heap().stats_collector(),
                    StatsCollector::MARK_INCREMENTAL_START,
                );

                // Performing incremental or concurrent marking.
                self.schedule().notify_incremental_marking_start();
                // Scanning the stack is expensive so we only do it at the atomic pause.
                self.VisitLocalRoots(StackState::kNoHeapPointers);
                self.ScheduleIncrementalMarkingTask();
                if self.config_.marking_type == MarkingConfig::MarkingType::IncrementalAndConcurrent {
                    self.mutator_marking_state_.Publish();
                    self.concurrent_marker().Start();
                }
                self.MarkStrongCrossThreadRoots();
                todo!()//self.heap().stats_collector().RegisterObserver(
                //     &incremental_marking_allocation_observer_,
                // );
            }
        }

        pub fn handle_not_fully_constructed_objects(&mut self) {
            if self.config_.stack_state == StackState::kNoHeapPointers {
                self.mutator_marking_state_.FlushNotFullyConstructedObjects();
            } else {
                self.MarkNotFullyConstructedObjects();
            }
        }

        pub fn enter_atomic_pause(&mut self, stack_state: StackState) {
            let top_stats_scope = StatsCollector::EnabledScope::new(
                self.heap().stats_collector(),
                StatsCollector::ATOMIC_MARK,
            );
            let stats_scope = StatsCollector::EnabledScope::new(
                self.heap().stats_collector(),
                StatsCollector::MARK_ATOMIC_PROLOGUE,
            );

            let old_marking_type = self.config_.marking_type;

            if exit_incremental_marking_if_needed(self.config_, &mut self.heap_) {
                // Cancel remaining incremental tasks. Concurrent marking jobs are left to
                // run in parallel with the atomic pause until the mutator thread runs out
                // of work.
                self.incremental_marking_handle_.cancel_if_non_empty();
                //TODO(Rust): Add StatsCollector::UnregisterObserver
                //self.heap().stats_collector().UnregisterObserver(
                //    &incremental_marking_allocation_observer_,
                //);
            }
            self.config_.stack_state = stack_state;
            self.config_.marking_type = MarkingConfig::MarkingType::Atomic;
            self.mutator_marking_state_.set_in_atomic_pause();

            {
                // VisitLocalRoots() also resets the LABs.
                self.VisitLocalRoots(self.config_.stack_state);
                // Early marking of strong cross-thread roots before parallel marking. Helps
                // avoiding long single-threaded marking phases.
                self.MarkStrongCrossThreadRoots();
                self.handle_not_fully_constructed_objects();
            }
            if old_marking_type == MarkingConfig::MarkingType::IncrementalAndConcurrent {
                // Start parallel marking.
                self.mutator_marking_state_.Publish();
                let marker = self.concurrent_marker();
                if marker.IsActive() {
                    marker.NotifyIncrementalMutatorStepCompleted();
                } else {
                    marker.Start();
                }
            }
        }

        pub fn re_enable_concurrent_marking(&mut self) {
            if !self.is_marking_ {
                return;
            }

            if self.config_.marking_type == MarkingConfig::MarkingType::Atomic {
                return;
            }

            assert_eq!(
                self.config_.marking_type,
                MarkingConfig::MarkingType::Incremental
            );
            self.config_.marking_type = MarkingConfig::MarkingType::IncrementalAndConcurrent;
            self.mutator_marking_state_.Publish();
            let marker = self.concurrent_marker();
            assert!(!marker.IsActive());
            marker.Start();
            assert!(marker.IsActive());
        }

        pub fn leave_atomic_pause(&mut self) {
            {
                let top_stats_scope = StatsCollector::EnabledScope::new(
                    self.heap().stats_collector(),
                    StatsCollector::ATOMIC_MARK,
                );
                let stats_scope = StatsCollector::EnabledScope::new(
                    self.heap().stats_collector(),
                    StatsCollector::MARK_ATOMIC_EPILOGUE,
                );
                assert!(self.incremental_marking_handle_.is_canceled());
                let overall_marked_bytes = self.mutator_marking_state_.marked_bytes()
                    + self.concurrent_marker().concurrently_marked_bytes();
                self.heap()
                    .stats_collector()
                    .notify_marking_completed(overall_marked_bytes);
                self.is_marking_ = false;
            }
            self.ProcessWeakness();
            todo!()//self.heap().SetStackStateOfPrevGC(self.config_.stack_state);
        }

        pub fn enter_process_global_atomic_pause(&mut self) {
            self.VisitCrossThreadRoots();
        }

        pub fn finish_marking(&mut self, stack_state: StackState) {
            assert!(self.is_marking_);
            self.enter_atomic_pause(stack_state);
            self.enter_process_global_atomic_pause();
            {
                let stats_scope = StatsCollector::EnabledScope::new(
                    self.heap().stats_collector(),
                    StatsCollector::ATOMIC_MARK,
                );
                assert!(self.AdvanceMarkingWithLimits(
                    Duration::MAX,
                    usize::MAX
                ));
                if self.JoinConcurrentMarkingIfNeeded() {
                    assert!(self.AdvanceMarkingWithLimits(
                        Duration::MAX,
                        usize::MAX
                    ));
                }
                self.mutator_marking_state_.Publish();
            }
            self.leave_atomic_pause();
        }

        pub fn process_cross_thread_weakness_if_needed(&mut self) {
            assert_eq!(
                self.config_.marking_type,
                MarkingConfig::MarkingType::Atomic
            );

            if self.processed_cross_thread_weakness_ {
                return;
            }

            let stats_scope = StatsCollector::EnabledScope::new(
                self.heap().stats_collector(),
                StatsCollector::ATOMIC_WEAK,
            );
            // Weakness callbacks are forbidden from allocating objects.
            //TODO(Rust): Add DisallowGarbageCollectionScope
            // cppgc::subtle::DisallowGarbageCollectionScope disallow_gc_scope(heap_);

            let mut root_marking_visitor =
                RootMarkingVisitor::new(self.mutator_marking_state());

            // Processing cross-thread roots requires taking the global process lock.
            // Process these weak roots first to minimize the time the lock is held.
            ProcessGlobalLock::AssertHeld();
            assert!(self.visited_cross_thread_persistents_in_atomic_pause_);
            todo!()//self.heap()
            //    .GetWeakCrossThreadPersistentRegion()
            //    .Iterate(root_marking_visitor);
            ProcessGlobalLock::Unlock::<ProcessGlobalLock::Reason>(
                ProcessGlobalLock::Reason::kForGC,
            );
            self.processed_cross_thread_weakness_ = true;
        }

        fn ProcessWeakness(&mut self) {
            assert_eq!(
                self.config_.marking_type,
                MarkingConfig::MarkingType::Atomic
            );

            self.process_cross_thread_weakness_if_needed();

            // Weakness callbacks are forbidden from allocating objects.
            //TODO(Rust): Add DisallowGarbageCollectionScope
            //cppgc::subtle::DisallowGarbageCollectionScope disallow_gc_scope(heap_);

            let stats_scope = StatsCollector::EnabledScope::new(
                self.heap().stats_collector(),
                StatsCollector::ATOMIC_WEAK,
            );

            let mut root_marking_visitor =
                RootMarkingVisitor::new(self.mutator_marking_state());

            // Launch the parallel job before anything else to provide the maximum time
            // slice for processing.
            let broker = LivenessBroker::new();
            let mut job_handle: Option<Box<dyn JobHandle>> = None;
            //TODO(Rust): Add Heap::MarkingType
            //if self.heap().marking_support() == cppgc::Heap::MarkingType::kIncrementalAndConcurrent {
            //     job_handle = platform_->PostJob(
            //         cppgc::TaskPriority::kUserBlocking,
            //         std::make_unique<WeakCallbackJobTask>(
            //             this, marking_worklists_.parallel_weak_callback_worklist(),
            //             broker));
            //}

            // Process same-thread roots.
            todo!()//self.heap().GetWeakPersistentRegion().Iterate(root_marking_visitor);

            // Call weak callbacks on objects that may now be pointing to dead objects.
            //TODO(Rust): Add CPPGC_YOUNG_GENERATION
            // #if defined(CPPGC_YOUNG_GENERATION)
            // if (heap().generational_gc_supported()) {
            //   auto& remembered_set = heap().remembered_set();
            //   if (config_.collection_type == CollectionType::kMinor) {
            //     // Custom callbacks assume that untraced pointers point to not yet freed
            //     // objects. They must make sure that upon callback completion no
            //     // UntracedMember points to a freed object. This may not hold true if a
            //     // custom callback for an old object operates with a reference to a young
            //     // object that was freed on a minor collection cycle. To maintain the
            //     // invariant that UntracedMembers always point to valid objects, execute
            //     // custom callbacks for old objects on each minor collection cycle.
            //     remembered_set.ExecuteCustomCallbacks(broker);
            //   } else {
            //     // For major GCs, just release all the remembered weak callbacks.
            //     remembered_set.ReleaseCustomCallbacks();
            //   }
            // }
            // #endif  // defined(CPPGC_YOUNG_GENERATION)

            {
                // First, process weak container callbacks.
                let inner_stats_scope = StatsCollector::EnabledScope::new(
                    self.heap().stats_collector(),
                    StatsCollector::WEAK_CONTAINER_CALLBACKS_PROCESSING,
                );
                todo!()
                // MarkingWorklists::WeakCallbackItem item;
                // MarkingWorklists::WeakCallbackWorklist::Local& collections_local =
                //     mutator_marking_state_.weak_container_callback_worklist();
                // while (collections_local.Pop(&item)) {
                //   item.callback(broker, item.parameter);
                // }
            }
            {
                // Then, process custom weak callbacks.
                let inner_stats_scope = StatsCollector::EnabledScope::new(
                    self.heap().stats_collector(),
                    StatsCollector::CUSTOM_CALLBACKS_PROCESSING,
                );
                todo!()
                // MarkingWorklists::WeakCallbackItem item;
                // MarkingWorklists::WeakCustomCallbackWorklist::Local& custom_callbacks =
                //     mutator_marking_state_.weak_custom_callback_worklist();
                // while (custom_callbacks.Pop(&item)) {
                //   item.callback(broker, item.parameter);
                // #if defined(CPPGC_YOUNG_GENERATION)
                //   if (heap().generational_gc_supported())
                //     heap().remembered_set().AddWeakCallback(item);
                // #endif  // defined(CPPGC_YOUNG_GENERATION)
                // }
            }

            if let Some(handle) = job_handle {
                handle.Join();
            } else {
                todo!()
                // MarkingWorklists::WeakCallbackItem item;
                // MarkingWorklists::WeakCallbackWorklist::Local& local =
                //     mutator_marking_state_.parallel_weak_callback_worklist();
                // while (local.Pop(&item)) {
                //   item.callback(broker, item.parameter);
                // }
            }

            // Weak callbacks should not add any new objects for marking.
            assert!(self.marking_worklists_.marking_worklist().is_empty());
        }

        fn VisitLocalRoots(&mut self, stack_state: StackState) {
            let stats_scope = StatsCollector::EnabledScope::new(
                self.heap().stats_collector(),
                StatsCollector::MARK_VISIT_ROOTS,
            );

            // Reset LABs before scanning roots. LABs are cleared to allow
            // ObjectStartBitmap handling without considering LABs.
            todo!()//self.heap().object_allocator().ResetLinearAllocationBuffers();

            {
                let inner_stats_scope = StatsCollector::DisabledScope::new(
                    self.heap().stats_collector(),
                    StatsCollector::MARK_VISIT_PERSISTENTS,
                );
                let mut root_marking_visitor =
                    RootMarkingVisitor::new(self.mutator_marking_state());
                todo!()//self.heap()
                //    .GetStrongPersistentRegion()
                //    .Iterate(root_marking_visitor);
            }

            if stack_state != StackState::kNoHeapPointers {
                let stack_stats_scope = StatsCollector::DisabledScope::new(
                    self.heap().stats_collector(),
                    StatsCollector::MARK_VISIT_STACK,
                );
                todo!()//self.heap().stack().SetMarkerIfNeededAndCallback([this]() {
                //  heap().stack().IteratePointersUntilMarker(&stack_visitor());
                //});
            }

            //TODO(Rust): Add CPPGC_YOUNG_GENERATION
            // #if defined(CPPGC_YOUNG_GENERATION)
            // if (config_.collection_type == CollectionType::kMinor) {
            //   StatsCollector::EnabledScope inner_stats_scope(
            //       heap().stats_collector(), StatsCollector::kMarkVisitRememberedSets);
            //   heap().remembered_set().Visit(visitor(), conservative_visitor(),
            //                                 mutator_marking_state_);
            // }
            // #endif  // defined(CPPGC_YOUNG_GENERATION)
        }

        fn VisitCrossThreadRoots(&mut self) {
            let inner_stats_scope = StatsCollector::DisabledScope::new(
                self.heap().stats_collector(),
                StatsCollector::MARK_VISIT_CROSS_THREAD_PERSISTENTS,
            );
            assert_eq!(
                self.config_.marking_type,
                MarkingConfig::MarkingType::Atomic
            );
            assert!(!self.visited_cross_thread_persistents_in_atomic_pause_);
            // Lock guards against changes to {Weak}CrossThreadPersistent handles, that
            // may conflict with marking. E.g., a WeakCrossThreadPersistent may be
            // converted into a CrossThreadPersistent which requires that the handle
            // is either cleared or the object is retained.
            ProcessGlobalLock::Lock::<ProcessGlobalLock::Reason>(
                ProcessGlobalLock::Reason::kForGC,
            );
            let mut root_marking_visitor =
                RootMarkingVisitor::new(self.mutator_marking_state());
            todo!()//self.heap()
            //    .GetStrongCrossThreadPersistentRegion()
            //    .Iterate(root_marking_visitor);
            self.visited_cross_thread_persistents_in_atomic_pause_ = true;
        }

        fn MarkStrongCrossThreadRoots(&mut self) {
            ProcessGlobalLock::Lock::<ProcessGlobalLock::Reason>(
                ProcessGlobalLock::Reason::kForGC,
            );
            let mut root_marking_visitor =
                RootMarkingVisitor::new(self.mutator_marking_state());
            todo!()//self.heap()
            //    .GetStrongCrossThreadPersistentRegion()
            //    .Iterate(root_marking_visitor);
            ProcessGlobalLock::Unlock::<ProcessGlobalLock::Reason>(
                ProcessGlobalLock::Reason::kForGC,
            );
        }

        fn ScheduleIncrementalMarkingTask(&mut self) {
            if self.platform_.is_null() || self.foreground_task_runner_.is_none()
            {
                return;
            }
            if !self.incremental_marking_handle_.is_canceled() {
                return;
            }

            let runner = self.foreground_task_runner_.as_ref().unwrap();
            let task_handle = IncrementalMarkingTask::post(runner.as_ref(), self);
            self.incremental_marking_handle_ = task_handle;
        }

        pub fn IncrementalMarkingStepForTesting(&mut self, stack_state: StackState) -> bool {
            self.IncrementalMarkingStep(stack_state)
        }

        fn IncrementalMarkingStep(&mut self, stack_state: StackState) -> bool {
            if stack_state == StackState::kNoHeapPointers {
                self.mutator_marking_state_.FlushNotFullyConstructedObjects();
            }
            self.config_.stack_state = stack_state;

            self.AdvanceMarkingWithLimits()
        }

        fn AdvanceMarkingOnAllocation(&mut self) {
            let stats_scope = StatsCollector::EnabledScope::new(
                self.heap().stats_collector(),
                StatsCollector::INCREMENTAL_MARK,
            );
            let nested_scope = StatsCollector::EnabledScope::new(
                self.heap().stats_collector(),
                StatsCollector::MARK_ON_ALLOCATION,
            );
            self.AdvanceMarkingOnAllocationImpl();
        }

        fn JoinConcurrentMarkingIfNeeded(&mut self) -> bool {
            if self.config_.marking_type != MarkingConfig::MarkingType::Atomic
                || !self.concurrent_marker().Join()
            {
                return false;
            }

            // Concurrent markers may have pushed some "leftover" in-construction objects
            // after flushing in EnterAtomicPause.
            self.handle_not_fully_constructed_objects();
            assert!(self
                .marking_worklists_
                .not_fully_constructed_worklist()
                .is_empty());
            true
        }

        fn NotifyConcurrentMarkingOfWorkIfNeeded(&self, _priority: TaskPriority) {
            let marker = self.concurrent_marker();
            if marker.IsActive() {
                marker.NotifyOfWorkIfNeeded(_priority);
            }
        }

        fn AdvanceMarkingWithLimits(&mut self) -> bool {
            self.AdvanceMarkingWithLimits(Duration::MAX, 0)
        }

        fn AdvanceMarkingWithLimits(
            &mut self,
            max_duration: Duration,
            marked_bytes_limit: usize,
        ) -> bool {
            if self.main_marking_disabled_for_testing_ {
                self.AdvanceMarkingWithLimitsEpilogue();
                return false;
            }

            let deadline_scope = StatsCollector::EnabledScope::new(
                self.heap().stats_collector(),
                StatsCollector::MARK_TRANSITIVE_CLOSURE_WITH_DEADLINE,
            );
            self.last_bytes_marked_ = 0;
            let deadline = TimeTicks::now() + max_duration;
            let mut marked_bytes_limit = marked_bytes_limit;
            if marked_bytes_limit == 0 {
                marked_bytes_limit =
                    todo!()//get_next_incremental_step_duration(self.schedule(), &self.heap_);
            }
            // `ProcessWorklistsWithDeadline()` below checks against `marked_bytes()`
            // which are never reset.
            let mut marked_bytes_deadline =
                marked_bytes_limit + self.mutator_marking_state_.marked_bytes();
            if marked_bytes_deadline < marked_bytes_limit {
                marked_bytes_deadline = usize::MAX;
            }
            let is_done = self.ProcessWorklistsWithDeadline(marked_bytes_deadline, deadline);
            self.last_bytes_marked_ = self.mutator_marking_state_.RecentlyMarkedBytes();
            todo!()//self.schedule_
            //    .AddMutatorThreadMarkedBytes(self.last_bytes_marked_);
            self.mutator_marking_state_.Publish();
            if !is_done {
                self.AdvanceMarkingWithLimitsEpilogue();
            }
            is_done
        }

        fn ProcessWorklistsWithDeadline(
            &mut self,
            marked_bytes_deadline: usize,
            time_deadline: TimeTicks,
        ) -> bool {
            let stats_collector = self.heap().stats_collector();
            let stats_scope = StatsCollector::EnabledScope::new(
                stats_collector,
                StatsCollector::MARK_TRANSITIVE_CLOSURE,
            );
            let mut saved_did_discover_new_ephemeron_pairs: bool;
            loop {
                self.mutator_marking_state_.ResetDidDiscoverNewEphemeronPairs();
                if (self.config_.marking_type == MarkingConfig::MarkingType::Atomic)
                    || todo!()//self.schedule_.ShouldFlushEphemeronPairs()
                {
                    self.mutator_marking_state_.FlushDiscoveredEphemeronPairs();
                }

                // Bailout objects may be complicated to trace and thus might take longer
                // than other objects. Therefore we reduce the interval between deadline
                // checks to guarantee the deadline is not exceeded.
                if !todo!() /*drain_worklist_with_bytes_and_time_deadline::<
                    StatsCollector::MARK_PROCESS_BAIL_OUT_OBJECTS,
                    DEFAULT_DEADLINE_CHECK_INTERVAL / 5,
                >(
                    stats_collector,
                    self.mutator_marking_state_,
                    usize::MAX,
                    time_deadline,
                    self.mutator_marking_state_.concurrent_