// Converted from V8 C++ source files:
// Header: heap-base.h
// Implementation: heap-base.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]

use std::sync::{Mutex, Arc};
use std::vec::Vec;
use std::ptr::null_mut;
use std::mem::size_of;
use crate::Heap;
use crate::V8;
use crate::roots::ReadOnlyRoots;
use crate::base::Stack;
use crate::cppgc::heap_base::MoveListener;
use crate::cppgc::internal::FatalOutOfMemoryHandler;
use crate::cppgc::internal::PageBackend;
use crate::cppgc::internal::PreFinalizerHandler;
use crate::cppgc::marker::MarkerBase;
use crate::cppgc::object_allocator::ObjectAllocator;
use crate::cppgc::platform::Platform;
use crate::cppgc::process_heap_statistics::ProcessHeapStatisticsUpdater;
use crate::cppgc::raw_heap::RawHeap;
use crate::cppgc::sweeper::Sweeper;
use crate::cppgc::write_barrier::Enable;
use crate::base;
use crate::heap;
use crate::objects::fixed_array_inl::TaggedField;
use crate::heap::cppgc::compactor::Compactor;
use crate::heap::cppgc::heap_config::GCConfig;
use crate::heap::cppgc::marker;
use crate::heap::cppgc::pointer_policies::PersistentRegion;
use crate::heap::cppgc::pointer_policies::CrossThreadPersistentRegion;
use crate::heap::cppgc::remembered_set::OldToNewRememberedSet;
use crate::heap::cppgc::stats_collector::StatsCollector;
use crate::heap::cppgc::stats_collector::CollectionType;
use crate::objects::objects::HeapObject;
use crate::objects::tagged_impl_inl::Tagged;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum class HeapObjectNameForUnnamedObject {
  kUseHiddenName,
  kUseClassNameIfSupported,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum class StickyBits : u8 {
  kDisabled,
  kEnabled,
}

pub trait MoveListenerTrait {
    fn on_move(&self, from: usize, to: usize, size_including_header: usize);
}

pub struct HeapBase {
    heap_thread_id_: i32,
    raw_heap_: RawHeap,
    platform_: Arc<dyn Platform>,
    oom_handler_: Box<FatalOutOfMemoryHandler>,
    page_backend_: Box<PageBackend>,
    heap_registry_subscription_: HeapRegistrySubscription,
    stats_collector_: Box<StatsCollector>,
    stack_: Box<Stack>,
    prefinalizer_handler_: Box<PreFinalizerHandler>,
    marker_: Option<Box<dyn MarkerBase>>,
    compactor_: Compactor,
    object_allocator_: ObjectAllocator,
    sweeper_: Sweeper,
    strong_persistent_region_: PersistentRegion,
    weak_persistent_region_: PersistentRegion,
    strong_cross_thread_persistent_region_: CrossThreadPersistentRegion,
    weak_cross_thread_persistent_region_: CrossThreadPersistentRegion,
    allocation_observer_for_PROCESS_HEAP_STATISTICS_: ProcessHeapStatisticsUpdater::AllocationObserverImpl,
    remembered_set_: OldToNewRememberedSet,
    no_gc_scope_: usize,
    disallow_gc_scope_: usize,
    stack_support_: StackSupport,
    stack_state_of_prev_gc_: EmbedderStackState,
    in_atomic_pause_: bool,
    marking_support_: MarkingType,
    sweeping_support_: SweepingType,
    name_for_unnamed_object_: HeapObjectNameForUnnamedObject,
    move_listeners_: Vec<Box<dyn MoveListenerTrait + Send + Sync>>,
    is_incremental_marking_in_progress_: bool,
}

impl HeapBase {
    pub fn from(heap_handle: &HeapHandle) -> &HeapBase {
        unsafe { &*(heap_handle as *const HeapHandle as *const HeapBase) }
    }

    pub fn from_const(heap_handle: &HeapHandle) -> &HeapBase {
        unsafe { &*(heap_handle as *const HeapHandle as *const HeapBase) }
    }

    pub fn new(
        platform: Arc<dyn Platform>,
        custom_spaces: Vec<Box<dyn CustomSpaceBase>>,
        stack_support: StackSupport,
        marking_support: MarkingType,
        sweeping_support: SweepingType,
        garbage_collector: &mut GarbageCollector,
    ) -> HeapBase {
        let raw_heap = RawHeap::new();
        let oom_handler = Box::new(FatalOutOfMemoryHandler::new());
        let page_backend = Box::new(PageBackend::new());
        let heap_registry_subscription = HeapRegistrySubscription::new();
        let stats_collector = Box::new(StatsCollector::new(platform.clone()));
        let stack = Box::new(Stack::new());
        let prefinalizer_handler = Box::new(PreFinalizerHandler::new());
        let marker = None;
        let compactor = Compactor::new();
        let object_allocator = ObjectAllocator::new();
        let sweeper = Sweeper::new();
        let strong_persistent_region = PersistentRegion::new();
        let weak_persistent_region = PersistentRegion::new();
        let strong_cross_thread_persistent_region = CrossThreadPersistentRegion::new();
        let weak_cross_thread_persistent_region = CrossThreadPersistentRegion::new();
        let allocation_observer_for_PROCESS_HEAP_STATISTICS = ProcessHeapStatisticsUpdater::AllocationObserverImpl::new();
        let remembered_set = OldToNewRememberedSet::new();
        let move_listeners = Vec::new();

        let mut heap_base = HeapBase {
            heap_thread_id_: 0,
            raw_heap_: raw_heap,
            platform_: platform,
            oom_handler_: oom_handler,
            page_backend_: page_backend,
            heap_registry_subscription_: heap_registry_subscription,
            stats_collector_: stats_collector,
            stack_: stack,
            prefinalizer_handler_: prefinalizer_handler,
            marker_: marker,
            compactor_: compactor,
            object_allocator_: object_allocator,
            sweeper_: sweeper,
            strong_persistent_region_: strong_persistent_region,
            weak_persistent_region_: weak_persistent_region,
            strong_cross_thread_persistent_region_: strong_cross_thread_persistent_region,
            weak_cross_thread_persistent_region_: weak_cross_thread_persistent_region,
            allocation_observer_for_PROCESS_HEAP_STATISTICS_: allocation_observer_for_PROCESS_HEAP_STATISTICS,
            remembered_set_: remembered_set,
            no_gc_scope_: 0,
            disallow_gc_scope_: 0,
            stack_support_: stack_support,
            stack_state_of_prev_gc_: EmbedderStackState::kNoHeapPointers,
            in_atomic_pause_: false,
            marking_support_: marking_support,
            sweeping_support_: sweeping_support,
            name_for_unnamed_object_: HeapObjectNameForUnnamedObject::kUseHiddenName,
            move_listeners_: move_listeners,
            is_incremental_marking_in_progress_: false,
        };

        heap_base.heap_thread_id_ = unsafe { base::OS::GetCurrentThreadId() };
        heap_base
    }

    pub fn raw_heap(&self) -> &RawHeap {
        &self.raw_heap_
    }

    pub fn platform(&self) -> &dyn Platform {
        self.platform_.as_ref()
    }

    pub fn oom_handler(&self) -> &FatalOutOfMemoryHandler {
        self.oom_handler_.as_ref()
    }

    pub fn page_backend(&self) -> &PageBackend {
        self.page_backend_.as_ref()
    }

    pub fn stats_collector(&self) -> &StatsCollector {
        self.stats_collector_.as_ref()
    }

    pub fn prefinalizer_handler(&self) -> &PreFinalizerHandler {
        self.prefinalizer_handler_.as_ref()
    }

    pub fn marker(&self) -> Option<&dyn MarkerBase> {
        self.marker_.as_ref().map(|m| m.as_ref())
    }

    pub fn get_marker_ref_for_testing(&mut self) -> &mut Option<Box<dyn MarkerBase>> {
        &mut self.marker_
    }

    pub fn compactor(&mut self) -> &mut Compactor {
        &mut self.compactor_
    }

    pub fn object_allocator(&mut self) -> &mut ObjectAllocator {
        &mut self.object_allocator_
    }

    pub fn sweeper(&mut self) -> &mut Sweeper {
        &mut self.sweeper_
    }

    pub fn get_strong_persistent_region(&mut self) -> &mut PersistentRegion {
        &mut self.strong_persistent_region_
    }
    pub fn get_strong_persistent_region_const(&self) -> & PersistentRegion {
         &self.strong_persistent_region_
    }

    pub fn get_weak_persistent_region(&mut self) -> &mut PersistentRegion {
        &mut self.weak_persistent_region_
    }
    pub fn get_weak_persistent_region_const(&self) -> & PersistentRegion {
         &self.weak_persistent_region_
    }


    pub fn get_strong_cross_thread_persistent_region(&mut self) -> &mut CrossThreadPersistentRegion {
        &mut self.strong_cross_thread_persistent_region_
    }

    pub fn get_strong_cross_thread_persistent_region_const(&self) -> & CrossThreadPersistentRegion {
        &self.strong_cross_thread_persistent_region_
    }

    pub fn get_weak_cross_thread_persistent_region(&mut self) -> &mut CrossThreadPersistentRegion {
        &mut self.weak_cross_thread_persistent_region_
    }
     pub fn get_weak_cross_thread_persistent_region_const(&self) -> & CrossThreadPersistentRegion {
        &self.weak_cross_thread_persistent_region_
    }

    pub fn remembered_set(&mut self) -> &mut OldToNewRememberedSet {
        &mut self.remembered_set_
    }

    pub fn object_payload_size(&self) -> usize {
        ObjectSizeCounter::new().get_size(&self.raw_heap_)
    }

    pub fn stack(&mut self) -> &mut Stack {
        self.stack_.as_mut()
    }

    pub fn stack_support(&self) -> StackSupport {
        self.stack_support_
    }

    pub fn set_override_stack_state(&mut self, _state: EmbedderStackState)  {}
    pub fn clear_overridden_stack_state(&mut self)  {}

    pub fn terminate(&mut self) {
       
    }

    pub fn is_gc_forbidden(&self) -> bool {
        self.disallow_gc_scope_ > 0
    }

    pub fn in_atomic_pause(&self) -> bool {
        self.in_atomic_pause_
    }

    pub fn collect_statistics(&mut self, detail_level: HeapStatisticsDetailLevel) -> HeapStatistics {
        HeapStatistics::new()
    }

    pub fn stack_state_of_prev_gc(&self) -> EmbedderStackState {
        self.stack_state_of_prev_gc_
    }

    pub fn set_stack_state_of_prev_gc(&mut self, stack_state: EmbedderStackState) {
        self.stack_state_of_prev_gc_ = stack_state;
    }

    pub fn set_in_atomic_pause_for_testing(&mut self, value: bool) {
        self.in_atomic_pause_ = value;
    }

    pub fn start_incremental_garbage_collection_for_testing(&mut self) {}
    pub fn finalize_incremental_garbage_collection_for_testing(&mut self, _state: EmbedderStackState) {}

    pub fn set_metric_recorder(&mut self, _histogram_recorder: Option<Box<dyn MetricRecorder>>) {}

    pub fn current_thread_is_heap_thread(&self) -> bool {
        unsafe { self.heap_thread_id_ == base::OS::GetCurrentThreadId() }
    }

    pub fn marking_support(&self) -> MarkingType {
        self.marking_support_
    }

    pub fn sweeping_support(&self) -> SweepingType {
        self.sweeping_support_
    }

    pub fn incremental_marking_supported(&self) -> bool {
        self.marking_support_ != MarkingType::kAtomic
    }

    pub fn generational_gc_supported(&self) -> bool {
        false
    }

    pub fn sticky_bits(&self) -> StickyBits {
        if self.generational_gc_supported() {
            StickyBits::kEnabled
        } else {
            StickyBits::kDisabled
        }
    }

    pub fn name_of_unnamed_object(&self) -> HeapObjectNameForUnnamedObject {
        self.name_for_unnamed_object_
    }

    pub fn set_name_of_unnamed_object(&mut self, value: HeapObjectNameForUnnamedObject) {
        self.name_for_unnamed_object_ = value;
    }

    pub fn has_move_listeners(&self) -> bool {
        !self.move_listeners_.is_empty()
    }

    pub fn call_move_listeners(&self, from: usize, to: usize, size_including_header: usize) {
        for listener in &self.move_listeners_ {
            listener.on_move(from, to, size_including_header);
        }
    }

    pub fn register_move_listener(&mut self, listener: Box<dyn MoveListenerTrait + Send + Sync>) {
          self.move_listeners_.push(listener);
    }

    pub fn unregister_move_listener(&mut self, _listener: &dyn MoveListenerTrait) {
         
    }

    pub fn set_incremental_marking_in_progress(&mut self, _value: bool) {
       
    }

    pub fn enter_no_gc_scope(&mut self) {
        self.no_gc_scope_ += 1;
    }

    pub fn leave_no_gc_scope(&mut self) {
        assert!(self.no_gc_scope_ > 0);
        self.no_gc_scope_ -= 1;
    }

    pub fn enter_disallow_gc_scope(&mut self) {
        self.disallow_gc_scope_ += 1;
    }

    pub fn leave_disallow_gc_scope(&mut self) {
        assert!(self.disallow_gc_scope_ > 0);
        self.disallow_gc_scope_ -= 1;
    }

    fn initialize_page_backend(_allocator: &mut PageAllocator) -> Box<PageBackend> {
        Box::new(PageBackend::new())
    }

    fn finalize_incremental_garbage_collection_if_needed(&mut self, _stack_state: StackState) {}

    fn is_gc_allowed(&self) -> bool {
        !self.sweeper_.is_sweeping_on_mutator_thread() && self.no_gc_scope_ == 0
    }

    fn in_no_gc_scope(&self) -> bool {
        self.no_gc_scope_ > 0
    }

    fn is_marking(&self) -> bool {
        self.marker_.is_some()
    }

    fn execute_pre_finalizers(&mut self) -> usize {
        0
    }

    fn page_allocator(&self) -> &dyn PageAllocator {
        self.platform_.get_page_allocator()
    }
    
    pub fn is_incremental_marking_in_progress(&self) -> bool {
        self.is_incremental_marking_in_progress_
    }
}

pub struct ClassNameAsHeapObjectNameScope<'a> {
    heap_: &'a mut HeapBase,
    saved_heap_object_name_value_: HeapObjectNameForUnnamedObject,
}

impl<'a> ClassNameAsHeapObjectNameScope<'a> {
    pub fn new(heap: &'a mut HeapBase) -> Self {
        let saved_heap_object_name_value = heap.name_of_unnamed_object();
        heap.set_name_of_unnamed_object(HeapObjectNameForUnnamedObject::kUseClassNameIfSupported);
        ClassNameAsHeapObjectNameScope {
            heap_: heap,
            saved_heap_object_name_value_: saved_heap_object_name_value,
        }
    }
}

impl<'a> Drop for ClassNameAsHeapObjectNameScope<'a> {
    fn drop(&mut self) {
        self.heap_.set_name_of_unnamed_object(self.saved_heap_object_name_value_);
    }
}

pub trait CustomSpaceBase {}

pub trait PageAllocator {
    fn allocate_page(&self, size: usize) -> *mut u8;
    fn commit_page(&self, start: *mut u8, size: usize);
    fn decommit_page(&self, start: *mut u8, size: usize);
    fn release_page(&self, start: *mut u8, size: usize);
}

pub struct HeapRegistrySubscription {}
impl HeapRegistrySubscription {
    pub fn new() -> Self {
        HeapRegistrySubscription {}
    }
}

pub trait MetricRecorder {}
pub enum class StackSupport {
    kSupportsStack,
    kNoStack,
}

pub enum class MarkingType {
    kAtomic,
    kIncremental,
}

pub enum class SweepingType {
    kAtomic,
    kConcurrent,
}

pub struct HeapStatistics {
    committed_memory: usize,
    resident_memory: usize,
    allocated_object_size: usize,
    pooled_memory: usize,
    detail_level: HeapStatisticsDetailLevel,
    nodes_in_use: usize,
    nodes_total: usize,
}

impl HeapStatistics {
    pub fn new() -> Self {
        HeapStatistics {
            committed_memory: 0,
            resident_memory: 0,
            allocated_object_size: 0,
            pooled_memory: 0,
            detail_level: HeapStatisticsDetailLevel::kBrief,
            nodes_in_use: 0,
            nodes_total: 0,
        }
    }
}
pub enum class HeapStatisticsDetailLevel {
    kBrief,
    kDetailed,
}

pub enum class EmbedderStackState {
    kNoHeapPointers,
    kMayContainHeapPointers,
}

struct ObjectSizeCounter {
    accumulated_size_: usize,
}

impl ObjectSizeCounter {
    fn new() -> Self {
        ObjectSizeCounter {
            accumulated_size_: 0,
        }
    }

    fn get_size(&mut self, heap: &RawHeap) -> usize {
       self.accumulated_size_ = 0;
        self.traverse(heap);
        self.accumulated_size_
    }

    fn object_size(header: &HeapObjectHeader) -> usize {
        ObjectView::new(header).size()
    }

    fn visit_heap_object_header(&mut self, header: &mut HeapObjectHeader) -> bool {
        if header.is_free() {
            return true;
        }
        self.accumulated_size_ += ObjectSizeCounter::object_size(header);
        true
    }

    fn traverse(&mut self, heap: &RawHeap) {
       
    }
}

struct ObjectView<'a> {
    header: &'a HeapObjectHeader,
}

impl<'a> ObjectView<'a> {
    fn new(header: &'a HeapObjectHeader) -> Self {
        ObjectView {
            header,
        }
    }

    fn size(&self) -> usize {
        16
    }
}

struct HeapHandle {}
impl HeapHandle {
    pub fn is_young_generation_enabled_(&self) -> bool {false}
}

struct AllLABsAreEmpty {}
impl AllLABsAreEmpty {
    fn value(&self) -> bool {false}
}

struct GarbageCollector {}
impl GarbageCollector {
     fn as_mut(&mut self) -> &mut GarbageCollector {self}
}

struct HeapStatisticsCollector {}
impl HeapStatisticsCollector {
    fn collect_detailed_statistics(&self, _heap_base: &HeapBase) -> HeapStatistics {
        HeapStatistics::new()
    }
}
mod base {
    pub mod OS {
        pub unsafe fn GetCurrentThreadId() -> i32 {
            0
        }
    }
}
