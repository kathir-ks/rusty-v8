// Converted from V8 C++ source files:
// Header: marking-state.h
// Implementation: marking-state.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod marking_state {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::{
        cell::{Cell, RefCell},
        collections::HashMap,
        mem::MaybeUninit,
        ptr::NonNull,
        rc::Rc,
        sync::{Mutex, RwLock},
    };

    use crate::heap::base::cached_unordered_map::CachedUnorderedMap;
    use crate::heap::base::stack::Stack;
    use crate::heap::cppgc::compaction_worklists::CompactionWorklists;
    use crate::heap::cppgc::globals::GlobalGCInfoTable;
    use crate::heap::cppgc::heap_object_header::HeapObjectHeader;
    use crate::heap::cppgc::heap_page::{BasePage, HeapBase, LargePage};
    use crate::heap::cppgc::liveness_broker::LivenessBrokerFactory;
    use crate::heap::cppgc::marking_worklists::{MarkingWorklists, AccessMode};
    use crate::heap::cppgc::stats_collector::StatsCollector;
    use crate::objects::objects::HeapObject;
    use crate::objects::objects::Visitor;

    pub struct MarkingStateBase<'a> {
        heap_: *mut HeapBase,
        marking_worklist_:
            MarkingWorklists<'a>::MarkingWorklist<'a>::Local,
        not_fully_constructed_worklist_:
            *mut MarkingWorklists<'a>::NotFullyConstructedWorklist<'a>,
    }

    impl<'a> MarkingStateBase<'a> {
        pub fn new(heap: *mut HeapBase, marking_worklists: &'a mut MarkingWorklists<'a>) -> Self {
            MarkingStateBase {
                heap_: heap,
                marking_worklist_: marking_worklists.marking_worklist().clone(),
                not_fully_constructed_worklist_:
                    marking_worklists.not_fully_constructed_worklist(),
            }
        }

        pub fn mark_and_push(&mut self, object: *const std::ffi::c_void, desc: TraceDescriptor) {
            assert!(!object.is_null());
            let header = unsafe {
                HeapObjectHeader::from_object(desc.base_object_payload as *mut std::ffi::c_void)
            };
            self.mark_and_push_header(header, desc);
        }

        pub fn mark_and_push_header(&mut self, header: &mut HeapObjectHeader) {
            self.mark_and_push_header(
                header,
                TraceDescriptor {
                    base_object_payload: header.object_start() as *mut std::ffi::c_void,
                    callback: unsafe {
                        GlobalGCInfoTable::gc_info_from_index(header.get_gc_info_index()).trace
                    },
                },
            );
        }

        pub fn push_marked(&mut self, header: &mut HeapObjectHeader, desc: TraceDescriptor) {
            assert!(header.is_marked::<AccessMode::kAtomic>());
            assert!(!header.is_in_construction::<AccessMode::kAtomic>());
            assert!(!desc.callback.is_null());

            self.marking_worklist_.push(desc);
        }

        pub fn publish(&mut self) {
            self.marking_worklist_.publish();
        }

        pub fn marking_worklist(
            &mut self,
        ) -> &mut MarkingWorklists<'a>::MarkingWorklist<'a>::Local {
            &mut self.marking_worklist_
        }

        pub fn not_fully_constructed_worklist(
            &mut self,
        ) -> &mut MarkingWorklists<'a>::NotFullyConstructedWorklist<'a> {
            unsafe { &mut *self.not_fully_constructed_worklist_ }
        }
    }

    impl<'a> MarkingStateBase<'a> {
        fn mark_and_push_header(&mut self, header: &mut HeapObjectHeader, desc: TraceDescriptor) {
            assert!(!desc.callback.is_null());

            if header.is_in_construction::<AccessMode::kAtomic>() {
                unsafe {
                    (&mut *self.not_fully_constructed_worklist_)
                        .push::<AccessMode::kAtomic>(header)
                };
            } else if self.mark_no_push(header) {
                self.push_marked(header, desc);
            }
        }

        fn mark_no_push(&mut self, header: &mut HeapObjectHeader) -> bool {
            unsafe {
                assert_eq!((&mut *self.heap_), &BasePage::from_payload(header).heap());
                assert!(!header.is_free::<AccessMode::kAtomic>());
                header.try_mark_atomic()
            }
        }
    }

    pub struct BasicMarkingState<'a> {
        marking_state_base: MarkingStateBase<'a>,
        previously_not_fully_constructed_worklist_:
            MarkingWorklists<'a>::PreviouslyNotFullyConstructedWorklist<'a>::Local,
        weak_container_callback_worklist_:
            MarkingWorklists<'a>::WeakCallbackWorklist<'a>::Local,
        parallel_weak_callback_worklist_:
            MarkingWorklists<'a>::WeakCallbackWorklist<'a>::Local,
        weak_custom_callback_worklist_:
            MarkingWorklists<'a>::WeakCustomCallbackWorklist<'a>::Local,
        write_barrier_worklist_: MarkingWorklists<'a>::WriteBarrierWorklist<'a>::Local,
        concurrent_marking_bailout_worklist_:
            MarkingWorklists<'a>::ConcurrentMarkingBailoutWorklist<'a>::Local,
        discovered_ephemeron_pairs_worklist_:
            MarkingWorklists<'a>::EphemeronPairsWorklist<'a>::Local,
        ephemeron_pairs_for_processing_worklist_:
            MarkingWorklists<'a>::EphemeronPairsWorklist<'a>::Local,
        weak_containers_worklist_: *mut MarkingWorklists<'a>::WeakContainersWorklist,
        movable_slots_worklist_:
            Option<Box<CompactionWorklists<'a>::MovableReferencesWorklist<'a>::Local>>,
        marked_bytes_: usize,
        last_marked_bytes_: usize,
        in_ephemeron_processing_: bool,
        discovered_new_ephemeron_pairs_: bool,
        in_atomic_pause_: bool,
        marked_bytes_map_: CachedUnorderedMap<
            *mut BasePage,
            i64,
            v8::base::hash::DefaultHasher,
        >,
    }

    impl<'a> BasicMarkingState<'a> {
        pub fn new(
            heap: *mut HeapBase,
            marking_worklists: &'a mut MarkingWorklists<'a>,
            compaction_worklists: Option<&'a mut CompactionWorklists<'a>>,
        ) -> Self {
            let mut movable_slots_worklist: Option<
                Box<CompactionWorklists<'a>::MovableReferencesWorklist<'a>::Local>,
            > = None;
            if let Some(cw) = compaction_worklists {
                movable_slots_worklist = Some(Box::new(
                    cw.movable_slots_worklist().clone(),
                ));
            }
            BasicMarkingState {
                marking_state_base: MarkingStateBase::new(heap, marking_worklists),
                previously_not_fully_constructed_worklist_:
                    marking_worklists.previously_not_fully_constructed_worklist().clone(),
                weak_container_callback_worklist_:
                    marking_worklists.weak_container_callback_worklist().clone(),
                parallel_weak_callback_worklist_:
                    marking_worklists.parallel_weak_callback_worklist().clone(),
                weak_custom_callback_worklist_:
                    marking_worklists.weak_custom_callback_worklist().clone(),
                write_barrier_worklist_: marking_worklists.write_barrier_worklist().clone(),
                concurrent_marking_bailout_worklist_:
                    marking_worklists.concurrent_marking_bailout_worklist().clone(),
                discovered_ephemeron_pairs_worklist_:
                    marking_worklists.discovered_ephemeron_pairs_worklist().clone(),
                ephemeron_pairs_for_processing_worklist_:
                    marking_worklists.ephemeron_pairs_for_processing_worklist().clone(),
                weak_containers_worklist_: marking_worklists.weak_containers_worklist(),
                movable_slots_worklist_: movable_slots_worklist,
                marked_bytes_: 0,
                last_marked_bytes_: 0,
                in_ephemeron_processing_: false,
                discovered_new_ephemeron_pairs_: false,
                in_atomic_pause_: false,
                marked_bytes_map_: CachedUnorderedMap::new(),
            }
        }

        pub fn register_weak_reference_if_needed(
            &mut self,
            object: *const std::ffi::c_void,
            desc: TraceDescriptor,
            weak_callback: WeakCallback,
            parameter: *const std::ffi::c_void,
        ) {
            let header = unsafe {
                HeapObjectHeader::from_object(desc.base_object_payload as *mut std::ffi::c_void)
            };
            if !header.is_in_construction::<AccessMode::kAtomic>()
                && header.is_marked::<AccessMode::kAtomic>()
            {
                return;
            }
            self.parallel_weak_callback_worklist_.push(WeakCallbackParameter {
                callback: weak_callback,
                parameter: parameter,
            });
        }

        pub fn register_weak_container_callback(
            &mut self,
            callback: WeakCallback,
            object: *const std::ffi::c_void,
        ) {
            assert!(!callback.is_null());
            self.weak_container_callback_worklist_.push(WeakCallbackParameter {
                callback: callback,
                parameter: object,
            });
        }

        pub fn register_weak_custom_callback(
            &mut self,
            callback: WeakCallback,
            object: *const std::ffi::c_void,
        ) {
            assert!(!callback.is_null());
            self.weak_custom_callback_worklist_.push(WeakCallbackParameter {
                callback: callback,
                parameter: object,
            });
        }

        pub fn register_movable_reference(&mut self, slot: *const *const std::ffi::c_void) {
            if self.movable_slots_worklist_.is_none() {
                return;
            }
            if cfg!(feature = "CPPGC_CAGED_HEAP") {
                todo!()
            } else {
                if unsafe { crate::heap::base::stack::is_on_stack(slot as usize) } {
                    return;
                }
            }
            if let Some(ref mut worklist) = self.movable_slots_worklist_ {
                worklist.push(slot as *mut *const std::ffi::c_void);
            }
        }

        pub fn process_weak_container(
            &mut self,
            object: *const std::ffi::c_void,
            desc: TraceDescriptor,
            callback: WeakCallback,
            data: *const std::ffi::c_void,
        ) {
            assert!(!object.is_null());

            let mut header = unsafe {
                HeapObjectHeader::from_object(object as *mut std::ffi::c_void)
            };

            if header.is_in_construction::<AccessMode::kAtomic>() {
                unsafe {
                    (&mut *self.marking_state_base.not_fully_constructed_worklist_)
                        .push::<AccessMode::kAtomic>(&mut header);
                }
                return;
            }

            self.register_weak_container(header);

            if !self.marking_state_base.mark_no_push(header) {
                return;
            }

            self.register_weak_container_callback(callback, data);

            if !desc.callback.is_null() {
                self.marking_state_base.push_marked(header, desc);
            } else {
                self.account_marked_bytes(header);
            }
        }

        pub fn process_ephemeron(
            &mut self,
            key: *const std::ffi::c_void,
            value: *const std::ffi::c_void,
            value_desc: TraceDescriptor,
            visitor: &mut Visitor,
        ) {
            assert!(!self.in_ephemeron_processing_);
            self.in_ephemeron_processing_ = true;

            let key_header = unsafe { HeapObjectHeader::from_object(key as *mut std::ffi::c_void) };
            let key_in_construction = key_header.is_in_construction::<AccessMode::kAtomic>();
            let key_considered_as_live = if key_in_construction {
                self.in_atomic_pause_
            } else {
                key_header.is_marked::<AccessMode::kAtomic>()
            };

            if key_considered_as_live {
                if !value_desc.base_object_payload.is_null() {
                    self.marking_state_base
                        .mark_and_push(value_desc.base_object_payload, value_desc);
                } else {
                    unsafe { value_desc.callback.unwrap()(visitor, value) };
                }
            } else {
                self.discovered_ephemeron_pairs_worklist_.push(EphemeronPair {
                    key: key,
                    value: value,
                    value_desc: value_desc,
                });
                self.discovered_new_ephemeron_pairs_ = true;
            }

            self.in_ephemeron_processing_ = false;
        }

        pub fn account_marked_bytes(&mut self, header: &HeapObjectHeader) {
            let marked_bytes = if header.is_large_object::<AccessMode::kAtomic>() {
                unsafe {
                    (BasePage::from_payload(header) as *const BasePage as *const LargePage)
                        .as_ref()
                        .unwrap()
                        .payload_size()
                }
            } else {
                header.allocated_size::<AccessMode::kAtomic>()
            };

            let base_page = unsafe {
                BasePage::from_payload(&mut *(header as *const _ as *mut HeapObjectHeader))
            };
            self.account_marked_bytes_page(base_page, marked_bytes);
        }

        pub fn account_marked_bytes_page(&mut self, base_page: *mut BasePage, marked_bytes: usize) {
            self.marked_bytes_ += marked_bytes;
            self.marked_bytes_map_
                .insert(base_page, marked_bytes as i64);
        }

        pub fn marked_bytes(&self) -> usize {
            self.marked_bytes_
        }

        pub fn recently_marked_bytes(&mut self) -> usize {
            let result = self.marked_bytes_ - self.last_marked_bytes_;
            self.last_marked_bytes_ = self.marked_bytes_;
            result
        }

        pub fn publish(&mut self) {
            self.marking_state_base.publish();
            self.previously_not_fully_constructed_worklist_.publish();
            self.weak_container_callback_worklist_.publish();
            self.parallel_weak_callback_worklist_.publish();
            self.weak_custom_callback_worklist_.publish();
            self.write_barrier_worklist_.publish();
            self.concurrent_marking_bailout_worklist_.publish();
            self.discovered_ephemeron_pairs_worklist_.publish();
            self.ephemeron_pairs_for_processing_worklist_.publish();
            if let Some(ref mut worklist) = self.movable_slots_worklist_ {
                worklist.publish();
            }

            for entry in self.marked_bytes_map_.take() {
                unsafe {
                    entry
                        .0
                        .as_mut()
                        .unwrap()
                        .increment_marked_bytes(entry.1 as usize)
                };
            }
        }

        pub fn previously_not_fully_constructed_worklist(
            &mut self,
        ) -> &mut MarkingWorklists<'a>::PreviouslyNotFullyConstructedWorklist<'a>::Local {
            &mut self.previously_not_fully_constructed_worklist_
        }

        pub fn weak_container_callback_worklist(
            &mut self,
        ) -> &mut MarkingWorklists<'a>::WeakCallbackWorklist<'a>::Local {
            &mut self.weak_container_callback_worklist_
        }

        pub fn parallel_weak_callback_worklist(
            &mut self,
        ) -> &mut MarkingWorklists<'a>::WeakCallbackWorklist<'a>::Local {
            &mut self.parallel_weak_callback_worklist_
        }

        pub fn weak_custom_callback_worklist(
            &mut self,
        ) -> &mut MarkingWorklists<'a>::WeakCustomCallbackWorklist<'a>::Local {
            &mut self.weak_custom_callback_worklist_
        }

        pub fn write_barrier_worklist(
            &mut self,
        ) -> &mut MarkingWorklists<'a>::WriteBarrierWorklist<'a>::Local {
            &mut self.write_barrier_worklist_
        }

        pub fn concurrent_marking_bailout_worklist(
            &mut self,
        ) -> &mut MarkingWorklists<'a>::ConcurrentMarkingBailoutWorklist<'a>::Local {
            &mut self.concurrent_marking_bailout_worklist_
        }

        pub fn discovered_ephemeron_pairs_worklist(
            &mut self,
        ) -> &mut MarkingWorklists<'a>::EphemeronPairsWorklist<'a>::Local {
            &mut self.discovered_ephemeron_pairs_worklist_
        }

        pub fn ephemeron_pairs_for_processing_worklist(
            &mut self,
        ) -> &mut MarkingWorklists<'a>::EphemeronPairsWorklist<'a>::Local {
            &mut self.ephemeron_pairs_for_processing_worklist_
        }

        pub fn weak_containers_worklist(
            &mut self,
        ) -> &mut MarkingWorklists<'a>::WeakContainersWorklist {
            unsafe { &mut *self.weak_containers_worklist_ }
        }

        pub fn movable_slots_worklist(
            &mut self,
        ) -> Option<&mut CompactionWorklists<'a>::MovableReferencesWorklist<'a>::Local> {
            self.movable_slots_worklist_.as_mut().map(|x| &mut **x)
        }

        pub fn did_discover_new_ephemeron_pairs(&self) -> bool {
            self.discovered_new_ephemeron_pairs_
        }

        pub fn reset_did_discover_new_ephemeron_pairs(&mut self) {
            self.discovered_new_ephemeron_pairs_ = false;
        }

        pub fn set_in_atomic_pause(&mut self) {
            self.in_atomic_pause_ = true;
        }
    }

    impl<'a> BasicMarkingState<'a> {
        fn register_weak_container(&mut self, header: &mut HeapObjectHeader) {
            unsafe {
                (&mut *self.weak_containers_worklist_)
                    .push::<AccessMode::kAtomic>(header);
            }
        }
    }

    pub struct MutatorMarkingState<'a> {
        basic_marking_state: BasicMarkingState<'a>,
        retrace_marked_objects_worklist_:
            MarkingWorklists<'a>::RetraceMarkedObjectsWorklist<'a>::Local,
        recently_retraced_weak_containers_: RecentlyRetracedWeakContainers,
    }

    impl<'a> MutatorMarkingState<'a> {
        pub fn new(
            heap: *mut HeapBase,
            marking_worklists: &'a mut MarkingWorklists<'a>,
            compaction_worklists: Option<&'a mut CompactionWorklists<'a>>,
        ) -> Self {
            MutatorMarkingState {
                basic_marking_state: BasicMarkingState::new(heap, marking_worklists, compaction_worklists),
                retrace_marked_objects_worklist_:
                    marking_worklists.retrace_marked_objects_worklist().clone(),
                recently_retraced_weak_containers_: RecentlyRetracedWeakContainers::new(),
            }
        }

        pub fn mark_no_push(&mut self, header: &mut HeapObjectHeader) -> bool {
            self.basic_marking_state.marking_state_base.mark_no_push(header)
        }

        pub fn re_trace_marked_weak_container(
            &mut self,
            visitor: &mut Visitor,
            header: &mut HeapObjectHeader,
        ) {
            let weak_containers_worklist = unsafe { &*self.basic_marking_state.weak_containers_worklist_ };
            assert!(weak_containers_worklist.contains::<AccessMode::kAtomic>(header));
            self.recently_retraced_weak_containers_.insert(header);
            self.retrace_marked_objects_worklist_.push(header);
        }

        pub fn dynamically_mark_address(&mut self, address: usize) {
            let mut header = unsafe {
                BasePage::from_payload(address as *const std::ffi::c_void)
                    .object_header_from_inner_address(address as *mut std::ffi::c_void)
            };
            assert!(!header.is_in_construction());
            if self.mark_no_push(header) {
                self.basic_marking_state.marking_state_base.marking_worklist_.push(
                    TraceDescriptor {
                        base_object_payload: header.object_start() as *mut std::ffi::c_void,
                        callback: unsafe {
                            GlobalGCInfoTable::gc_info_from_index(header.get_gc_info_index()).trace
                        },
                    },
                );
            }
        }

        pub fn flush_not_fully_constructed_objects(&mut self) {
            let objects = unsafe {
                (&mut *self.basic_marking_state.marking_state_base.not_fully_constructed_worklist_)
                    .extract::<AccessMode::kAtomic>()
            };
            for object in objects {
                if self.mark_no_push(object) {
                    self.basic_marking_state
                        .previously_not_fully_constructed_worklist_
                        .push(object);
                }
            }
        }

        pub fn flush_discovered_ephemeron_pairs(&mut self) {
            let stats_scope = unsafe {
                StatsCollector::EnabledScope::new(
                    (&*self.basic_marking_state.marking_state_base.heap_)
                        .stats_collector(),
                    StatsCollector::kMarkFlushEphemerons,
                )
            };
            self.basic_marking_state
                .discovered_ephemeron_pairs_worklist_
                .publish();
            if !self
                .basic_marking_state
                .discovered_ephemeron_pairs_worklist_
                .is_global_empty()
            {
                self.basic_marking_state
                    .ephemeron_pairs_for_processing_worklist_
                    .merge(self.basic_marking_state.discovered_ephemeron_pairs_worklist_.clone());
            }
        }

        pub fn invoke_weak_roots_callback_if_needed(
            &mut self,
            object: *const std::ffi::c_void,
            desc: TraceDescriptor,
            weak_callback: WeakCallback,
            parameter: *const std::ffi::c_void,
        ) {
            let header = unsafe {
                HeapObjectHeader::from_object(desc.base_object_payload as *mut std::ffi::c_void)
            };
            if cfg!(debug_assertions) {
                if header.is_in_construction() {
                    assert!(header.is_marked::<AccessMode::kAtomic>());
                }
            }
            unsafe { weak_callback(LivenessBrokerFactory::create(), parameter) };
        }

        pub fn is_marked_weak_container(&mut self, header: &mut HeapObjectHeader) -> bool {
            let weak_containers_worklist = unsafe { &*self.basic_marking_state.weak_containers_worklist_ };
            let result =
                weak_containers_worklist.contains::<AccessMode::kAtomic>(header)
                    && !self.recently_retraced_weak_containers_.contains(header);
            if cfg!(debug_assertions) {
                assert!(!result || header.is_marked::<AccessMode::kAtomic>());
                assert!(!result || !header.is_in_construction());
            }
            result
        }

        pub fn publish(&mut self) {
            self.basic_marking_state.publish();
            self.retrace_marked_objects_worklist_.publish();
        }

        pub fn retrace_marked_objects_worklist(
            &mut self,
        ) -> &mut MarkingWorklists<'a>::RetraceMarkedObjectsWorklist<'a>::Local {
            &mut self.retrace_marked_objects_worklist_
        }
    }

    struct RecentlyRetracedWeakContainers {
        recently_retraced_cache_: Vec<*mut HeapObjectHeader>,
        last_used_index_: isize,
    }

    impl RecentlyRetracedWeakContainers {
        const K_MAX_CACHE_SIZE: usize = 8;

        fn new() -> Self {
            RecentlyRetracedWeakContainers {
                recently_retraced_cache_: Vec::with_capacity(Self::K_MAX_CACHE_SIZE),
                last_used_index_: -1,
            }
        }

        fn contains(&self, header: &HeapObjectHeader) -> bool {
            self.recently_retraced_cache_
                .iter()
                .any(|&cached_header| cached_header == header as *const _ as *mut _)
        }

        fn insert(&mut self, header: &mut HeapObjectHeader) {
            self.last_used_index_ = (self.last_used_index_ + 1) % Self::K_MAX_CACHE_SIZE as isize;
            if self.recently_retraced_cache_.len() <= self.last_used_index_ as usize {
                self.recently_retraced_cache_.push(header);
            } else {
                self.recently_retraced_cache_[self.last_used_index_ as usize] = header;
            }
        }
    }

    pub struct ConcurrentMarkingState<'a> {
        basic_marking_state: BasicMarkingState<'a>,
    }

    impl<'a> ConcurrentMarkingState<'a> {
        pub fn new(
            heap: *mut HeapBase,
            marking_worklists: &'a mut MarkingWorklists<'a>,
            compaction_worklists: Option<&'a mut CompactionWorklists<'a>>,
        ) -> Self {
            ConcurrentMarkingState {
                basic_marking_state: BasicMarkingState::new(heap, marking_worklists, compaction_worklists),
            }
        }

        pub fn account_deferred_marked_bytes(&mut self, base_page: *mut BasePage, deferred_bytes: usize) {
            assert!(deferred_bytes <= self.basic_marking_state.marked_bytes_);
            self.basic_marking_state.marked_bytes_ -= deferred_bytes;
            self.basic_marking_state
                .marked_bytes_map_
                .insert(base_page, -(deferred_bytes as i64));
        }
    }

    pub fn drain_worklist_with_predicate<
        Predicate: Fn() -> bool,
        CreateStatsScopeCallback: Fn() -> StatsCollector::EnabledScope,
        WorklistLocal: MarkingWorklistsLocal,
        ProcessWorklistItemCallback: Fn(WorklistLocal::ItemType),
    >(
        should_yield: Predicate,
        create_stats_scope: CreateStatsScopeCallback,
        worklist_local: &mut WorklistLocal,
        process_worklist_item: ProcessWorklistItemCallback,
        k_deadline_check_interval: usize,
    ) -> bool {
        if worklist_local.is_local_and_global_empty() {
            return true;
        }
        if should_yield() {
            return false;
        }
        let stats_scope = create_stats_scope();
        let mut processed_callback_count = k_deadline_check_interval;
        while let Some(item) = worklist_local.pop() {
            process_worklist_item(item);
            if processed_callback_count == 0 {
                if should_yield() {
                    return false;
                }
                processed_callback_count = k_deadline_check_interval;
            }
            processed_callback_count -= 1;
        }
        true
    }

    pub unsafe fn dynamically_trace_marked_object<
        mode: crate::heap::cppgc::marking_worklists::AccessMode,
    >(
        visitor: &mut Visitor,
        header: &HeapObjectHeader,
    ) {
        assert!(!header.is_in_construction::<{ mode }>());
        assert!(header.is_marked::<AccessMode::kAtomic>());
        header.trace::<{ mode }>(visitor);
    }

    pub trait MarkingWorklistsLocal {
        type ItemType;
        fn is_local_and_global_empty(&self) -> bool;
        fn pop(&mut self) -> Option<Self::ItemType>;
    }

    impl<'a> MarkingWorklistsLocal for MarkingWorklists<'a>::MarkingWorklist<'a>::Local {
        type ItemType = TraceDescriptor;

        fn is_local_and_global_empty(&self) -> bool {
            self.is_local_and_global_empty()
        }

        fn pop(&mut self) -> Option<Self::ItemType> {
            self.pop()
        }
    }

    impl<'a> MarkingWorklistsLocal
        for MarkingWorklists<'a>::PreviouslyNotFullyConstructedWorklist<'a>::Local
    {
        type ItemType = *mut HeapObjectHeader;

        fn is_local_and_global_empty(&self) -> bool {
            self.is_local_and_global_empty()
        }

        fn pop(&mut self) -> Option<Self::ItemType> {
            self.pop()
        }
    }

    impl<'a> MarkingWorklistsLocal for MarkingWorklists<'a>::WeakCallbackWorklist<'a>::Local {
        type ItemType = WeakCallbackParameter;

        fn is_local_and_global_empty(&self) -> bool {
            self.is_local_and_global_empty()
        }

        fn pop(&mut self) -> Option<Self::ItemType> {
            self.pop()
        }
    }

    impl<'a> MarkingWorklistsLocal for MarkingWorklists<'a>::WeakCustomCallbackWorklist<'a>::Local {
        type ItemType = WeakCallbackParameter;

        fn is_local_and_global_empty(&self) -> bool {
            self.is_local_and_global_empty()
        }

        fn pop(&mut self) -> Option<Self::ItemType> {
            self.pop()
        }
    }

    impl<'a> MarkingWorklistsLocal for MarkingWorklists<'a>::WriteBarrierWorklist<'a>::Local {
        type ItemType = *mut HeapObjectHeader;

        fn is_local_and_global_empty(&self) -> bool {
            self.is_local_and_global_empty()
        }

        fn pop(&mut self) -> Option<Self::ItemType> {
            self.pop()
        }
    }

    impl<'a> MarkingWorklistsLocal
        for MarkingWorklists<'a>::ConcurrentMarkingBailoutWorklist<'a>::Local
    {
        type ItemType = *mut HeapObjectHeader;

        fn is_local_and_global_empty(&self) -> bool {
            self.is_local_and_global_empty()
        }

        fn pop(&mut self) -> Option<Self::ItemType> {
            self.pop()
        }
    }

    impl<'a> MarkingWorklistsLocal for MarkingWorklists<'a>::EphemeronPairsWorklist<'a>::Local {
        type ItemType = EphemeronPair;

        fn is_local_and_global_empty(&self) -> bool {
            self.is_local_and_global_empty()
        }

        fn pop(&mut self) -> Option<Self::ItemType> {
            self.pop()
        }
    }

    impl<'a> MarkingWorklistsLocal for MarkingWorklists<'a>::RetraceMarkedObjectsWorklist<'a>::Local {
        type ItemType = *mut HeapObjectHeader;

        fn is_local_and_global_empty(&self) -> bool {
            self.is_local_and_global_empty()
        }

        fn pop(&mut self) -> Option<Self::ItemType> {
            self.pop()
        }
    }

    #[derive(Clone, Copy)]
    pub struct TraceDescriptor {
        pub base_object_payload: *mut std::ffi::c_void,
        pub callback: WeakCallback,
    }

    pub type WeakCallback = Option<
        unsafe extern "C" fn(
            broker: *mut crate::heap::cppgc::liveness_broker::LivenessBroker,
            object: *const std::ffi::c_void,
        ),
    >;

    #[derive(Clone, Copy)]
    pub struct WeakCallbackParameter {
        pub callback: WeakCallback,
        pub parameter: *const std::ffi::c_void,
    }

    #[derive(Clone, Copy)]
    pub struct EphemeronPair {
        pub key: *const std::ffi::c_void,
        pub value: *const std::ffi::c_void,
        pub value_desc: TraceDescriptor,
    }
}
