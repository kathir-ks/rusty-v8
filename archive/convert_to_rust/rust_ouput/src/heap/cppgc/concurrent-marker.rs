// Converted from V8 C++ source files:
// Header: concurrent-marker.h
// Implementation: concurrent-marker.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod cppgc {
pub mod internal {
use std::sync::atomic::{AtomicUsize, Ordering};
use std::ptr::null_mut;
use std::sync::Mutex;
use crate::heap::cppgc::marker::Visitor;
use crate::heap::cppgc::marker::ConcurrentMarkingVisitor;
use crate::heap::cppgc::marking_state::ConcurrentMarkingState;
use crate::heap::cppgc::marking_worklists::MarkingWorklists;
use crate::heap::base::incremental_marking_schedule::IncrementalMarkingSchedule;
use crate::include::cppgc::platform::Platform;
use crate::heap::cppgc::heap_base::HeapBase;
use crate::heap::cppgc::stats_collector::StatsCollector;
use crate::heap::cppgc::heap_object_header::HeapObjectHeader;
use crate::heap::cppgc::heap_object_header::BasePage;

trait JobDelegate {
    fn should_yield(&self) -> bool;
}

struct DummyJobDelegate {
    yield_counter: Mutex<i32>,
    yield_threshold: i32,
}

impl DummyJobDelegate {
    fn new(yield_threshold: i32) -> Self {
        DummyJobDelegate {
            yield_counter: Mutex::new(0),
            yield_threshold,
        }
    }

    fn increment_counter(&self) {
        let mut counter = self.yield_counter.lock().unwrap();
        *counter += 1;
    }
}

impl JobDelegate for DummyJobDelegate {
    fn should_yield(&self) -> bool {
        let counter = self.yield_counter.lock().unwrap();
        if *counter >= self.yield_threshold {
            return true;
        }
        self.increment_counter();
        false
    }
}

pub struct V8_EXPORT_PRIVATE {}
pub struct JobHandle {
    is_valid: bool,
}

impl JobHandle {
    fn new() -> Self {
        JobHandle { is_valid: true }
    }

    fn is_valid(&self) -> bool {
        self.is_valid
    }

    fn join(&mut self) {
        self.is_valid = false;
    }

    fn cancel(&mut self) {
        self.is_valid = false;
    }

    fn notify_concurrency_increase(&self) {}

    fn update_priority(&self, _priority: TaskPriority) {}

    fn update_priority_enabled(&self) -> bool {
        false
    }
}

pub enum TaskPriority {
    kUserVisible,
    kUserBlocking,
}

struct ConcurrentMarkerBase {
    heap_: HeapBase,
    marking_worklists_: MarkingWorklists,
    incremental_marking_schedule_: IncrementalMarkingSchedule,
    platform_: *mut Platform,
    concurrent_marking_handle_: Mutex<Option<Box<JobHandle>>>,
    concurrently_marked_bytes_: AtomicUsize,
    concurrent_marking_priority_increased_: Mutex<bool>,
}

impl ConcurrentMarkerBase {
    fn new(
        heap: HeapBase,
        marking_worklists: MarkingWorklists,
        incremental_marking_schedule: IncrementalMarkingSchedule,
        platform: *mut Platform,
    ) -> Self {
        ConcurrentMarkerBase {
            heap_: heap,
            marking_worklists_: marking_worklists,
            incremental_marking_schedule_: incremental_marking_schedule,
            platform_: platform,
            concurrent_marking_handle_: Mutex::new(None),
            concurrently_marked_bytes_: AtomicUsize::new(0),
            concurrent_marking_priority_increased_: Mutex::new(false),
        }
    }

    fn start(&self) {
        if self.platform_.is_null() {
            return;
        }

        let mut handle = self.concurrent_marking_handle_.lock().unwrap();
        if handle.is_some() {
            return;
        }

        let task = ConcurrentMarkingTask::new(self);
        let mut platform = unsafe { &mut *self.platform_ };
        let job_handle = platform.post_job(TaskPriority::kUserVisible, Box::new(task));
        *handle = Some(Box::new(job_handle));

        self.incremental_marking_schedule_
            .notify_concurrent_marking_start();
    }

    fn join(&self) -> bool {
        let mut handle = self.concurrent_marking_handle_.lock().unwrap();
        if let Some(mut job_handle) = handle.take() {
            if job_handle.is_valid() {
                job_handle.join();
                return true;
            }
        }
        false
    }

    fn cancel(&self) -> bool {
         let mut handle = self.concurrent_marking_handle_.lock().unwrap();
        if let Some(mut job_handle) = handle.take() {
            if job_handle.is_valid() {
                job_handle.cancel();
                return true;
            }
        }
        false
    }

    fn notify_incremental_mutator_step_completed(&self) {
        if has_work_for_concurrent_marking(&self.marking_worklists_) {
            self.increase_marking_priority_if_needed();
            let handle = self.concurrent_marking_handle_.lock().unwrap();
            if let Some(job_handle) = handle.as_ref() {
                job_handle.notify_concurrency_increase();
            }
        }
    }

    fn notify_of_work_if_needed(&self, priority: TaskPriority) {
        let handle = self.concurrent_marking_handle_.lock().unwrap();
        if has_work_for_concurrent_marking(&self.marking_worklists_) {
            if let Some(job_handle) = handle.as_ref() {
                job_handle.update_priority(priority);
                job_handle.notify_concurrency_increase();
            }
        }
    }

    fn is_active(&self) -> bool {
         let handle = self.concurrent_marking_handle_.lock().unwrap();
        if let Some(job_handle) = handle.as_ref() {
             job_handle.is_valid()
        } else {
            false
        }
    }

    fn heap(&self) -> &HeapBase {
        &self.heap_
    }

    fn marking_worklists(&self) -> &MarkingWorklists {
        &self.marking_worklists_
    }

    fn incremental_marking_schedule(&self) -> &IncrementalMarkingSchedule {
        &self.incremental_marking_schedule_
    }

    fn add_concurrently_marked_bytes(&self, marked_bytes: usize) {
        self.concurrently_marked_bytes_
            .fetch_add(marked_bytes, Ordering::Relaxed);
        self.incremental_marking_schedule_
            .add_concurrently_marked_bytes(marked_bytes);
    }

    fn concurrently_marked_bytes(&self) -> usize {
        self.concurrently_marked_bytes_.load(Ordering::Relaxed)
    }

    fn increase_marking_priority_if_needed(&self) {
        let handle = self.concurrent_marking_handle_.lock().unwrap();
         if let Some(job_handle) = handle.as_ref() {
            if !job_handle.update_priority_enabled() {
                return;
            }
        }

        let mut priority_increased = self.concurrent_marking_priority_increased_.lock().unwrap();
        if *priority_increased {
            return;
        }

        let time_delta = self
            .incremental_marking_schedule_
            .get_time_since_last_concurrent_marking_update();
        if !time_delta.is_zero()
            && (time_delta.in_milliseconds_f()
                > (IncrementalMarkingSchedule::kEstimatedMarkingTime
                    .in_milliseconds_f()
                    * kMarkingScheduleRatioBeforeConcurrentPriorityIncrease))
        {
             if let Some(job_handle) = handle.as_ref() {
                job_handle.update_priority(TaskPriority::kUserBlocking);
             }
            *priority_increased = true;
        }
    }

    fn create_concurrent_marking_visitor(&self, marking_state: &mut ConcurrentMarkingState) -> Box<dyn Visitor> {
        Box::new(ConcurrentMarkingVisitor::new(&self.heap_, marking_state))
    }
}

unsafe impl Send for ConcurrentMarkerBase {}
unsafe impl Sync for ConcurrentMarkerBase {}

impl Drop for ConcurrentMarkerBase {
    fn drop(&mut self) {
         let handle = self.concurrent_marking_handle_.lock().unwrap();
        if let Some(job_handle) = handle.as_ref() {
            assert!(!job_handle.is_valid());
        }
    }
}

struct ConcurrentMarker {
    base: ConcurrentMarkerBase,
}

impl ConcurrentMarker {
    fn new(
        heap: HeapBase,
        marking_worklists: MarkingWorklists,
        incremental_marking_schedule: IncrementalMarkingSchedule,
        platform: *mut Platform,
    ) -> Self {
        ConcurrentMarker {
            base: ConcurrentMarkerBase::new(
                heap,
                marking_worklists,
                incremental_marking_schedule,
                platform,
            ),
        }
    }

     fn create_concurrent_marking_visitor(&self, marking_state: &mut ConcurrentMarkingState) -> Box<dyn Visitor> {
        Box::new(ConcurrentMarkingVisitor::new(&self.base.heap(), marking_state))
    }
}

struct ConcurrentMarkingTask {
    concurrent_marker_: *const ConcurrentMarkerBase,
}

impl ConcurrentMarkingTask {
    fn new(concurrent_marker: &ConcurrentMarkerBase) -> Self {
        ConcurrentMarkingTask {
            concurrent_marker_: concurrent_marker,
        }
    }
}

impl v8::JobTask for ConcurrentMarkingTask {
    fn run(&mut self, delegate: &dyn JobDelegate) {
        let concurrent_marker = unsafe { &*self.concurrent_marker_ };
        let stats_collector = concurrent_marker.heap().stats_collector();
        let _stats_scope = StatsCollector::EnabledConcurrentScope(stats_collector, StatsCollector::kConcurrentMark);

        if !has_work_for_concurrent_marking(&concurrent_marker.marking_worklists()) {
            return;
        }

        let mut concurrent_marking_state = ConcurrentMarkingState::new(
            concurrent_marker.heap(),
            concurrent_marker.marking_worklists(),
            concurrent_marker.heap().compactor().compaction_worklists(),
        );

        let mut concurrent_marking_visitor = concurrent_marker.create_concurrent_marking_visitor(&mut concurrent_marking_state);
        self.process_worklists(
            delegate,
            &mut concurrent_marking_state,
            concurrent_marking_visitor.as_mut(),
        );

        concurrent_marker.add_concurrently_marked_bytes(
            concurrent_marking_state.recently_marked_bytes(),
        );
        concurrent_marking_state.publish();
    }

    fn get_max_concurrency(&self, current_worker_count: usize) -> usize {
        let concurrent_marker = unsafe { &*self.concurrent_marker_ };
        work_size_for_concurrent_marking(&concurrent_marker.marking_worklists())
            + current_worker_count
    }
}

impl ConcurrentMarkingTask {
    fn process_worklists(
        &mut self,
        job_delegate: &dyn JobDelegate,
        concurrent_marking_state: &mut ConcurrentMarkingState,
        concurrent_marking_visitor: &mut dyn Visitor,
    ) {
        let concurrent_marker = unsafe { &*self.concurrent_marker_ };
        let stats_collector = concurrent_marker.heap().stats_collector();

        loop {
            if !drain_worklist_with_yielding::<
                StatsCollector::kConcurrentMarkProcessNotFullyconstructedWorklist,
            >(
                job_delegate,
                stats_collector,
                concurrent_marking_state,
                concurrent_marker,
                &mut concurrent_marking_state
                    .previously_not_fully_constructed_worklist(),
                |header| {
                    unsafe { BasePage::from_payload(header) }.synchronized_load();
                    concurrent_marking_state.account_marked_bytes(*header);
                    dynamically_trace_marked_object::<AccessMode::kAtomic>(
                        concurrent_marking_visitor,
                        *header,
                    );
                },
            ) {
                return;
            }

            if !drain_worklist_with_yielding::<
                StatsCollector::kConcurrentMarkProcessMarkingWorklist,
            >(
                job_delegate,
                stats_collector,
                concurrent_marking_state,
                concurrent_marker,
                &mut concurrent_marking_state.marking_worklist(),
                |item| {
                    unsafe { BasePage::from_payload(item.base_object_payload) }.synchronized_load();
                    let header =
                        unsafe { HeapObjectHeader::from_object(item.base_object_payload) };
                    assert!(!header.is_in_construction::<AccessMode::kAtomic>());
                    assert!(header.is_marked::<AccessMode::kAtomic>());
                    concurrent_marking_state.account_marked_bytes(header);
                    (item.callback)(concurrent_marking_visitor, item.base_object_payload);
                },
            ) {
                return;
            }

            if !drain_worklist_with_yielding::<
                StatsCollector::kConcurrentMarkProcessWriteBarrierWorklist,
            >(
                job_delegate,
                stats_collector,
                concurrent_marking_state,
                concurrent_marker,
                &mut concurrent_marking_state.write_barrier_worklist(),
                |header| {
                    unsafe { BasePage::from_payload(header) }.synchronized_load();
                    concurrent_marking_state.account_marked_bytes(*header);
                    dynamically_trace_marked_object::<AccessMode::kAtomic>(
                        concurrent_marking_visitor,
                        *header,
                    );
                },
            ) {
                return;
            }

            if !drain_worklist_with_yielding::<
                StatsCollector::kConcurrentMarkProcessEphemeronWorklist,
            >(
                job_delegate,
                stats_collector,
                concurrent_marking_state,
                concurrent_marker,
                &mut concurrent_marking_state.ephemeron_pairs_for_processing_worklist(),
                |item| {
                    concurrent_marking_state.process_ephemeron(
                        item.key,
                        item.value,
                        item.value_desc,
                        concurrent_marking_visitor,
                    );
                },
            ) {
                return;
            }

            if concurrent_marking_state.marking_worklist().is_local_and_global_empty() {
                break;
            }
        }
    }
}

trait V8JobTask {
    fn run(&mut self, delegate: &dyn JobDelegate);
    fn get_max_concurrency(&self, current_worker_count: usize) -> usize;
}

pub trait JobTask {
    fn run(&mut self, delegate: &dyn JobDelegate);
    fn get_max_concurrency(&self, current_worker_count: usize) -> usize;
}

impl<T: V8JobTask> JobTask for T {
    fn run(&mut self, delegate: &dyn JobDelegate) {
        V8JobTask::run(self, delegate);
    }

    fn get_max_concurrency(&self, current_worker_count: usize) -> usize {
        V8JobTask::get_max_concurrency(self, current_worker_count)
    }
}

const kMarkingScheduleRatioBeforeConcurrentPriorityIncrease: f64 = 0.5;
const kDefaultDeadlineCheckInterval: usize = 750;

fn drain_worklist_with_yielding<
    const SCOPE_ID: StatsCollector::ConcurrentScopeId,
    const DEADLINE_CHECK_INTERVAL: usize = kDefaultDeadlineCheckInterval,
>(
    job_delegate: &dyn JobDelegate,
    stats_collector: &StatsCollector,
    marking_state: &mut ConcurrentMarkingState,
    concurrent_marker: &ConcurrentMarkerBase,
    worklist_local: &mut dyn MarkingWorklistTrait,
    callback: impl FnMut(&mut HeapObjectHeader),
) -> bool {
    let mut adapted_callback = |item: &crate::heap::cppgc::marking_worklists::MarkingWorklists::MarkingItem| {
        let header = unsafe { HeapObjectHeader::from_object(item.base_object_payload) };
         unsafe { BasePage::from_payload(item.base_object_payload) }.synchronized_load();
         (callback)(&mut header);
    };
    drain_worklist_with_predicate::<DEADLINE_CHECK_INTERVAL>(
        || {
            concurrent_marker.add_concurrently_marked_bytes(
                marking_state.recently_marked_bytes(),
            );
            job_delegate.should_yield()
        },
        || StatsCollector::DisabledConcurrentScope(stats_collector, SCOPE_ID),
        worklist_local,
        adapted_callback,
    )
}

trait MarkingWorklistTrait {
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;
    fn pop(&mut self) -> Option<crate::heap::cppgc::marking_worklists::MarkingWorklists::MarkingItem>;
}

impl MarkingWorklistTrait for MarkingWorklists {
    fn is_empty(&self) -> bool {
        self.marking_worklist().is_empty()
    }
    fn size(&self) -> usize {
        self.marking_worklist().size()
    }
    fn pop(&mut self) -> Option<crate::heap::cppgc::marking_worklists::MarkingWorklists::MarkingItem> {
        self.marking_worklist().pop()
    }
}

fn drain_worklist_with_predicate<const DEADLINE_CHECK_INTERVAL: usize>(
    should_yield: impl FnMut() -> bool,
    get_disabled_scope: impl FnOnce() -> StatsCollector::DisabledConcurrentScope,
    worklist_local: &mut dyn MarkingWorklistTrait,
    mut callback: impl FnMut(&crate::heap::cppgc::marking_worklists::MarkingWorklists::MarkingItem),
) -> bool {
    let _disabled_scope = get_disabled_scope();
    let mut yield_counter = 0;
    while !worklist_local.is_empty() {
        if yield_counter >= DEADLINE_CHECK_INTERVAL {
            if should_yield() {
                return false;
            }
            yield_counter = 0;
        }
        if let Some(item) = worklist_local.pop() {
            callback(&item);
        }
        yield_counter += 1;
    }
    true
}

fn work_size_for_concurrent_marking(marking_worklists: &MarkingWorklists) -> usize {
    marking_worklists.marking_worklist().size()
        + marking_worklists.write_barrier_worklist().size()
        + marking_worklists
            .previously_not_fully_constructed_worklist()
            .size()
}

fn has_work_for_concurrent_marking(marking_worklists: &MarkingWorklists) -> bool {
    !marking_worklists.marking_worklist().is_empty()
        || !marking_worklists.write_barrier_worklist().is_empty()
        || !marking_worklists
            .previously_not_fully_constructed_worklist()
            .is_empty()
}

pub mod v8 {
    pub trait JobTask {
        fn run(&mut self, delegate: &dyn super::JobDelegate);
        fn get_max_concurrency(&self, current_worker_count: usize) -> usize;
    }
}
}  // namespace internal
}  // namespace cppgc
