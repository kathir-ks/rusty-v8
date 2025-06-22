// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/heap/concurrent-marking.h (Rust module definition - incomplete)
mod concurrent_marking {
    // Placeholder, as the header file's content is implicit in the .cc file
    // Define public interfaces here as needed
}

use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::HashMap;
use std::any::Any;

// Placeholder for v8config.h
const V8_COMPRESS_POINTERS_8GB_BOOL: bool = false;

// Placeholder for globals.h
const KB: usize = 1024;
const kObjectAlignment8GbHeap: usize = 8;
const kNullAddress: usize = 0;

// Placeholder for flags/flags.h
mod flags {
    pub static mut concurrent_marking: bool = false;
    pub static mut concurrent_marking_max_worker_num: i32 = 0;
    pub static mut parallel_marking: bool = false;
    pub static mut concurrent_minor_ms_marking: bool = false;
    pub static mut concurrent_marking_high_priority_threads: bool = false;
    pub static mut trace_concurrent_marking: bool = false;
    pub static mut minor_ms: bool = false;
}

// Placeholder for base/logging.h
macro_rules! DCHECK {
    ($condition:expr) => {
        if !$condition {
            panic!("DCHECK failed: {}", stringify!($condition));
        }
    };
}

macro_rules! DCHECK_EQ {
    ($left:expr, $right:expr) => {
        if $left != $right {
            panic!("DCHECK_EQ failed: {} != {}", stringify!($left), stringify!($right));
        }
    };
}

macro_rules! DCHECK_NE {
    ($left:expr, $right:expr) => {
        if $left == $right {
            panic!("DCHECK_NE failed: {} == {}", stringify!($left), stringify!($right));
        }
    };
}

macro_rules! DCHECK_NULL {
    ($ptr:expr) => {
        if !($ptr as *const _).is_null() {
            panic!("DCHECK_NULL failed: pointer is not null");
        }
    };
}

macro_rules! DCHECK_NOT_NULL {
    ($ptr:expr) => {
        if ($ptr as *const _).is_null() {
            panic!("DCHECK_NOT_NULL failed: pointer is null");
        }
    };
}

macro_rules! DCHECK_IMPLIES {
    ($condition:expr, $implication:expr) => {
        if $condition {
            DCHECK!($implication);
        }
    };
}

// Placeholder for execution/isolate.h
struct Isolate {}

// Placeholder for heap/base/cached-unordered-map.h
mod heap {
    pub mod base {
        use std::collections::HashMap;
        use std::hash::Hash;

        pub struct CachedUnorderedMap<K, V> {
            map: HashMap<K, V>,
        }

        impl<K: Eq + Hash + Copy, V> CachedUnorderedMap<K, V> {
            pub fn new() -> Self {
                CachedUnorderedMap { map: HashMap::new() }
            }

            pub fn get(&self, key: &K) -> Option<&V> {
                self.map.get(key)
            }

            pub fn insert(&mut self, key: K, value: V) {
                self.map.insert(key, value);
            }

            pub fn remove(&mut self, key: &K) -> Option<V> {
                self.map.remove(key)
            }

            pub fn is_empty(&self) -> bool {
                self.map.is_empty()
            }
        }
    }
}

// Placeholder for heap/gc-tracer-inl.h and heap/gc-tracer.h
mod gc_tracer {
    pub enum Scope {
        MC_BACKGROUND_MARKING,
        MINOR_MS_MARK_PARALLEL,
        MINOR_MS_BACKGROUND_MARKING,
        MINOR_MS_BACKGROUND_MARKING_CLOSURE
    }
    pub enum ThreadKind {
        kBackground
    }
    pub struct GCTracer {}
    impl GCTracer {
        pub fn CurrentEpoch(&self, scope: Scope) -> usize { 0 }
    }
}

// Placeholder for heap/heap-inl.h and heap/heap.h
mod heap_mod {
    use super::*;

    pub struct Heap {
        cpp_heap_: *mut CppHeap, // Raw pointer to CppHeap
        isolate_: *mut Isolate,
        tracer_: *mut gc_tracer::GCTracer,
        new_space_: bool,
        allocator_: *mut Allocator,
        mark_compact_collector_: *mut MarkCompactCollector,
        minor_mark_sweep_collector_: *mut MinorMarkSweepCollector,
        new_lo_space_: *mut NewLargeObjectSpace,
        pretenuring_handler_: *mut PretenuringHandler,
    }
    impl Heap {
        pub fn cpp_heap(&self) -> *mut CppHeap {
            self.cpp_heap_
        }
        pub fn isolate(&self) -> *mut Isolate {
            self.isolate_
        }
        pub fn tracer(&self) -> *mut gc_tracer::GCTracer {
            self.tracer_
        }
        pub fn use_new_space(&self) -> bool {
            self.new_space_
        }
        pub fn allocator(&self) -> *mut Allocator {
            self.allocator_
        }
         pub fn mark_compact_collector(&self) -> *mut MarkCompactCollector {
            self.mark_compact_collector_
        }
        pub fn minor_mark_sweep_collector(&self) -> *mut MinorMarkSweepCollector {
            self.minor_mark_sweep_collector_
        }
        pub fn new_lo_space(&self) -> *mut NewLargeObjectSpace {
            self.new_lo_space_
        }
        pub fn pretenuring_handler(&self) -> *mut PretenuringHandler {
            self.pretenuring_handler_
        }
        pub fn ShouldOptimizeForBattery(&self) -> bool {
            false // Placeholder
        }
        pub fn IsTearingDown(&self) -> bool {
            false // Placeholder
        }
        pub fn ShouldCurrentGCKeepAgesUnchanged(&self) -> bool {
            false // Placeholder
        }

    }
    pub struct Allocator {
        new_space_allocator_: *mut MainAllocator,
    }
    impl Allocator {
        pub fn new_space_allocator(&self) -> *mut MainAllocator {
            self.new_space_allocator_
        }
    }
    // Other heap related structs would go here
}

// Placeholder for heap/heap-layout-inl.h
mod heap_layout {
    use super::*;
    pub fn InReadOnlySpace<T>(_obj: T) -> bool {
        false // Placeholder
    }
}

// Placeholder for heap/heap-utils-inl.h
mod heap_utils {
    use super::*;
    pub fn GetOwnerHeap<T>(_obj: T) -> *mut heap_mod::Heap {
        std::ptr::null_mut() // Placeholder
    }
}

// Placeholder for heap/heap-visitor-inl.h and heap/heap-visitor.h
mod heap_visitor {
    // Placeholder
}

// Placeholder for heap/mark-compact-inl.h and heap/mark-compact.h
mod mark_compact {
    use super::*;

    pub struct MarkCompactCollector {}
    impl MarkCompactCollector {
        pub fn UseBackgroundThreadsInCycle(&self) -> bool {
            true // Placeholder
        }
        pub fn epoch(&self) -> u32 { 0 }
        pub fn code_flush_mode(&self) -> base::EnumSet<CodeFlushMode> { base::EnumSet::new() }
        pub fn local_marking_worklists(&self) -> *mut MarkingWorklists { std::ptr::null_mut() } // FIXME
    }
    
    pub mod marking {
        use super::*;
        pub struct MarkingWorklists {
           shared_: *mut SharedWorklist,
           other_: *mut OtherWorklist,
           context_worklists_: Vec<ContextWorklist>
        }
        impl MarkingWorklists {
            pub fn shared(&self) -> &SharedWorklist {
                unsafe { &*self.shared_}
            }
            pub fn other(&self) -> &OtherWorklist {
                unsafe { &*self.other_}
            }
            pub fn context_worklists(&self) -> &Vec<ContextWorklist> {
                &self.context_worklists_
            }
        }

        pub struct SharedWorklist{}
        impl SharedWorklist {
            pub fn IsEmpty(&self) -> bool { true }
            pub fn Size(&self) -> usize { 0 }
        }
        pub struct OtherWorklist{}
        impl OtherWorklist {
            pub fn IsEmpty(&self) -> bool { true }
            pub fn Size(&self) -> usize { 0 }
        }
        pub struct ContextWorklist {
            pub worklist: *mut InnerWorklist
        }
        pub struct InnerWorklist {}
        impl InnerWorklist {
             pub fn Size(&self) -> usize { 0 }
        }

    }
    pub unsafe trait AddressTrait {}

}

// Placeholder for heap/marking-state-inl.h
mod marking_state {
    // Placeholder
}

// Placeholder for heap/marking-visitor-inl.h and heap/marking-visitor.h
mod marking_visitor {
    use super::*;
    
    pub struct MarkingVisitorBaseType {}
    impl MarkingVisitorBaseType {
        pub fn VisitMapPointerIfNeeded(_map:usize, _object:usize) -> usize {0}
    }
}

// Placeholder for heap/memory-chunk-metadata.h and heap/memory-chunk.h
mod memory_chunk {
    use super::*;
    pub struct MutablePageMetadata {}
    impl MutablePageMetadata {
        pub fn IncrementLiveBytesAtomically(&self, _bytes: i64) {}
    }
    pub struct MemoryChunkMetadata {}
    impl MemoryChunkMetadata {
        pub fn FromHeapObject<T>(_object: T) -> *mut MutablePageMetadata {
            std::ptr::null_mut() // Placeholder
        }
    }

    pub unsafe trait AddressTrait {}
}

// Placeholder for heap/memory-measurement-inl.h and heap/memory-measurement.h
mod memory_measurement {
    // Placeholder
}

// Placeholder for heap/minor-mark-sweep-inl.h and heap/minor-mark-sweep.h
mod minor_mark_sweep {
    use super::*;

    pub struct MinorMarkSweepCollector {}
    impl MinorMarkSweepCollector {
        pub fn UseBackgroundThreadsInCycle(&self) -> bool {
            true // Placeholder
        }
        pub fn remembered_sets_marking_handler(&self) -> *mut YoungGenerationRememberedSetsMarkingWorklist {
           std::ptr::null_mut()
        }
        pub fn is_in_atomic_pause(&self) -> bool { false }
        pub fn RequestGC(&self) {}
    }
}

// Placeholder for heap/mutable-page-metadata.h
mod mutable_page_metadata {
    // Placeholder
}

// Placeholder for heap/object-lock.h
mod object_lock {
    // Placeholder
}

// Placeholder for heap/pretenuring-handler.h
mod pretenuring_handler {
    use std::collections::HashMap;

    pub struct PretenuringHandler {

    }
    impl PretenuringHandler {
        pub fn MergeAllocationSitePretenuringFeedback(&self, _feedback: HashMap<usize,usize>) {}
    }
}

// Placeholder for heap/weak-object-worklists.h
mod weak_object_worklists {
    use super::*;
    pub struct WeakObjects {
        current_ephemerons: EphemeronWorklist
    }
    impl WeakObjects {
        pub fn new() -> Self {
            WeakObjects { current_ephemerons: EphemeronWorklist {} }
        }
    }
    pub struct EphemeronWorklist{}
    impl EphemeronWorklist {
        pub fn IsEmpty(&self) -> bool { true }
        pub fn Size(&self) -> usize { 0 }
    }
    
}

// Placeholder for heap/young-generation-marking-visitor.h
mod young_generation_marking_visitor {
    use super::*;

    pub enum YoungGenerationMarkingVisitationMode {
        kParallel,
        kConcurrent
    }
    pub struct YoungGenerationRememberedSetsMarkingWorklist {}
    impl YoungGenerationRememberedSetsMarkingWorklist {
        pub fn RemainingRememberedSetsMarkingIteams(&self) -> usize { 0 }
    }
    pub struct YoungGenerationMarkingVisitorType {}
    impl YoungGenerationMarkingVisitorType {
        pub fn IncrementLiveBytesCached(_chunk: usize, _by: usize) {}
    }
}

// Placeholder for init/v8.h
mod v8_mod {
    use super::*;
    pub struct Platform {}
    impl Platform {
        pub fn NumberOfWorkerThreads(&self) -> i32 { 4 }
        pub fn PostJob(_priority:TaskPriority, _job: Box<dyn JobTask>) -> Box<JobHandle> {
            Box::new(JobHandle{})
        }
    }
    pub fn GetCurrentPlatform() -> *mut Platform {
        std::ptr::null_mut() // Placeholder
    }

}

// Placeholder for objects/data-handler-inl.h, objects/embedder-data-array-inl.h, objects/hash-table-inl.h, objects/js-array-buffer-inl.h, objects/slots-inl.h, objects/transitions-inl.h, objects/heap-object.h
mod objects {
    use super::*;
    pub struct HeapObject {}
    impl HeapObject {
        pub fn map(&self, _isolate:usize, _mode:usize) -> *mut Map {
            std::ptr::null_mut() // Placeholder
        }
        pub fn address(&self) -> usize { 0 }
    }
    pub struct Map {}
}

// Placeholder for utils/utils-inl.h
mod utils {
    pub fn IsAligned<T>(value: T, alignment: T) -> bool
    where
        T: std::ops::Rem<Output = T> + PartialEq + From<u8>,
    {
        value % alignment == T::from(0)
    }
}

// Implementations

use std::array;
use std::unique_ptr::UniquePtr;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};

struct Entry {
    page: *mut MutablePageMetadata,
    live_bytes: isize,
}

/// This class caches page live bytes during concurrent marking. This
/// avoids costly CAS operations on MutablePageMetadata::live_byte_count_ for
/// each traced object.
///
/// Page live bytes are cached in a fixed-size hash map. In the case of
/// collisions the existing entry is simply written back to
/// MutablePageMetadata::live_byte_count_ with a CAS. Afterwards it can be
/// replaced with the new entry.
struct MemoryChunkLiveBytesMap {
    map_: array::IntoIter<Entry, 32>,
}

impl MemoryChunkLiveBytesMap {
    const K_TABLE_SIZE: usize = 32;

    fn new() -> Self {
        MemoryChunkLiveBytesMap {
            map_: array::IntoIter::new([])
        }
    }

    fn increment(&mut self, page: *mut MutablePageMetadata, live: isize) {
        todo!()
        // let entry = self.lookup_entry(page);
        // if entry.page == page {
        //     entry.live_bytes += live;
        // } else if entry.page.is_null() {
        //     entry.page = page;
        //     entry.live_bytes = live;
        // } else {
        //     // Write back the existing entry.
        //     unsafe { (*entry.page).IncrementLiveBytesAtomically(entry.live_bytes) };
        //     // Now just replace it with the new entry.
        //     entry.page = page;
        //     entry.live_bytes = live;
        // }
    }

    fn flush_and_clear(&mut self) {
        todo!()
        // for entry in &mut self.map_ {
        //     if !entry.page.is_null() {
        //         unsafe { (*entry.page).IncrementLiveBytesAtomically(entry.live_bytes) };
        //         entry.page = std::ptr::null_mut();
        //         entry.live_bytes = 0;
        //     }
        // }
    }

    fn erase(&mut self, page: *mut MutablePageMetadata) {
        todo!()
        // let entry = self.lookup_entry(page);
        // if entry.page == page {
        //     entry.page = std::ptr::null_mut();
        //     entry.live_bytes = 0;
        // }
    }

    #[cfg(debug_assertions)]
    fn assert_empty(&self) {
        todo!()
        // for entry in &self.map_ {
        //     DCHECK_NULL!(entry.page);
        //     DCHECK_EQ!(entry.live_bytes, 0);
        // }
    }

    fn lookup_entry(&mut self, _page: *mut MutablePageMetadata) -> &mut Entry {
        todo!()
        // let hash = {
        //     let mut hasher = DefaultHasher::new();
        //     page.hash(&mut hasher);
        //     hasher.finish() as usize
        // };
        // let index = hash % Self::K_TABLE_SIZE;
        // &mut self.map_[index]
    }
}

type MemoryChunkTypedSlotsMap = HashMap<*mut MutablePageMetadata, UniquePtr<TypedSlots>>;

// Placeholder for TypedSlots and TypedSlots::Insert
struct TypedSlots {}
impl TypedSlots {
    fn Insert(&mut self, _slot_type:usize, _offset: usize) {}
}

struct ConcurrentMarkingVisitor<'a> {
    local_marking_worklists: *mut MarkingWorklistsLocal,
    local_weak_objects: *mut WeakObjectsLocal,
    heap: *mut heap_mod::Heap,
    mark_compact_epoch: u32,
    code_flush_mode: base::EnumSet<CodeFlushMode>,
    should_keep_ages_unchanged: bool,
    code_flushing_increase: u16,
    memory_chunk_live_bytes_map: &'a mut MemoryChunkLiveBytesMap,
    memory_chunk_typed_slots_map: &'a mut MemoryChunkTypedSlotsMap,
}

impl<'a> ConcurrentMarkingVisitor<'a> {
    const fn enable_concurrent_visitation() -> bool {
        true
    }

    // Implements ephemeron semantics: Marks value if key is already reachable.
    // Returns true if value was actually marked.
    fn process_ephemeron(&mut self, _key:usize, _value:usize) -> bool {
        todo!()
        // if self.marking_state().is_marked(key) {
        //     if let Some(target_worklist) = MarkingHelper::should_mark_object(self.heap, value) {
        //         if self.mark_object(key, value, target_worklist) {
        //             return true;
        //         }
        //     }
        // } else if self.marking_state().is_unmarked(value) {
        //     self.local_weak_objects.next_ephemerons_local.push(Ephemeron { key, value });
        // }
        // false
    }

    fn record_slot<TSlot>(&mut self, _object: usize, _slot: TSlot, _target:usize) {
        todo!()
        // MarkCompactCollector::RecordSlot(object, slot, target);
    }

    fn increment_live_bytes_cached(&mut self, chunk: *mut MutablePageMetadata, by: isize) {
         unsafe {
             if V8_COMPRESS_POINTERS_8GB_BOOL {
                DCHECK!(utils::IsAligned(by, kObjectAlignment8GbHeap as isize));
             }
        }
        self.memory_chunk_live_bytes_map.increment(chunk, by);
    }

    fn record_reloc_slot(&mut self, _host: usize, _rinfo:usize, _target: usize) {
        todo!()
        // if !MarkCompactCollector::ShouldRecordRelocSlot(host, rinfo, target) {
        //     return;
        // }

        // let info = MarkCompactCollector::ProcessRelocInfo(host, rinfo, target);

        // let typed_slots = (*self.memory_chunk_typed_slots_map).entry(info.page_metadata).or_insert_with(|| UniquePtr::new(TypedSlots::new()));
        // typed_slots.Insert(info.slot_type, info.offset);
    }
}

// Placeholder for MarkingWorklistsLocal
struct MarkingWorklistsLocal {}

// Placeholder for WeakObjectsLocal
struct WeakObjectsLocal {}

// Placeholder for CodeFlushMode
mod base {
    use std::ops::{BitOr, BitAnd, Not};
    use std::marker::Copy;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct EnumSet<T: Copy + Into<u32> + TryFrom<u32>> {
        bits: u32,
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T: Copy + Into<u32> + TryFrom<u32>> EnumSet<T> {
        pub const fn new() -> Self {
            EnumSet { bits: 0, _phantom: std::marker::PhantomData }
        }

        pub fn insert(&mut self, value: T) {
            self.bits |= value.into();
        }

        pub fn contains(&self, value: T) -> bool {
            (self.bits & value.into()) != 0
        }

        pub fn remove(&mut self, value: T) {
            self.bits &= !(value.into());
        }

        pub fn is_empty(&self) -> bool {
            self.bits == 0
        }
    }
    
    impl<T: Copy + Into<u32> + TryFrom<u32>> BitOr for EnumSet<T> {
        type Output = Self;
        fn bitor(self, rhs: Self) -> Self::Output {
            EnumSet { bits: self.bits | rhs.bits, _phantom: self._phantom }
        }
    }

    impl<T: Copy + Into<u32> + TryFrom<u32>> BitAnd for EnumSet<T> {
        type Output = Self;
        fn bitand(self, rhs: Self) -> Self::Output {
            EnumSet { bits: self.bits & rhs.bits, _phantom: self._phantom }
        }
    }

    impl<T: Copy + Into<u32> + TryFrom<u32>> Not for EnumSet<T> {
        type Output = Self;
        fn not(self) -> Self::Output {
            EnumSet { bits: !self.bits, _phantom: self._phantom }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum CodeFlushMode {
    kNoCodeFlushing,
    kEagerCodeFlushing,
    kFullCodeFlushing,
}

impl Into<u32> for CodeFlushMode {
    fn into(self) -> u32 {
        match self {
            CodeFlushMode::kNoCodeFlushing => 0,
            CodeFlushMode::kEagerCodeFlushing => 1,
            CodeFlushMode::kFullCodeFlushing => 2,
        }
    }
}

impl TryFrom<u32> for CodeFlushMode {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CodeFlushMode::kNoCodeFlushing),
            1 => Ok(CodeFlushMode::kEagerCodeFlushing),
            2 => Ok(CodeFlushMode::kFullCodeFlushing),
            _ => Err(()),
        }
    }
}

// Placeholder for NativeContextInferrer and NativeContextStats
struct NativeContextInferrer {}
impl NativeContextInferrer {
    fn Infer(&self, _cage_base:usize, _map:*mut objects::Map, _object: usize, _context: *mut usize) -> bool {
        false
    }
}
struct NativeContextStats {}
impl NativeContextStats {
    fn IncrementSize(&mut self, _context:usize, _map:*mut objects::Map, _object: usize, _visited_size: usize) {}
    fn Empty(&self) -> bool { true }
    fn Clear(&mut self) {}
    fn Merge(&mut self, _other:NativeContextStats) {}
}

// Placeholder for Ephemeron
struct Ephemeron {
    key: usize,
    value: usize,
}

// Placeholder for MainAllocator
struct MainAllocator {}
impl MainAllocator {
    fn original_top_acquire(&self) -> usize { 0 }
    fn original_limit_relaxed(&self) -> usize { 0 }
}

// Placeholder for NewLargeObjectSpace
struct NewLargeObjectSpace {}
impl NewLargeObjectSpace {
    fn pending_object(&self) -> usize { 0 }
}

// Placeholder for PtrComprCageBase
struct PtrComprCageBase {}

// Placeholder for TimedScope
struct TimedScope<'a> {
    time_ms: &'a mut f64,
}

impl<'a> TimedScope<'a> {
    fn new(time_ms: &'a mut f64) -> Self {
        *time_ms = 0.0; // Simulate time measurement
        TimedScope { time_ms }
    }
}

impl<'a> Drop for TimedScope<'a> {
    fn drop(&mut self) {
        // Simulate time measurement
        *self.time_ms = 1.0;
    }
}

// Placeholder for set_another_ephemeron_iteration
fn set_another_ephemeron_iteration(_value: bool) {}

// Placeholder for NativeContextInferrer::Infer

// Placeholder for CppHeap
struct CppHeap {}
impl CppHeap {
    fn From(_heap: *mut CppHeap) -> *mut CppHeap { _heap }
    fn CreateCppMarkingState(&self) -> usize { 0 }
}

// Actual implementation of ConcurrentMarking

struct ConcurrentMarking {
    heap_: *mut heap_mod::Heap,
    weak_objects_: *mut weak_object_worklists::WeakObjects,
    task_state_: Vec<UniquePtr<TaskState>>,
    job_handle_: Option<Box<dyn JobHandle>>,
    garbage_collector_: Option<GarbageCollector>,
    marking_worklists_: *mut mark_compact::marking::MarkingWorklists,
    total_marked_bytes_: AtomicUsize,
    current_job_trace_id_: Option<u64>,
    estimate_concurrency_: AtomicUsize,
    minor_marking_state_: Option<Box<MinorMarkingState>>,
}

struct TaskState {
    marked_bytes: AtomicUsize,
    memory_chunk_live_bytes_map: MemoryChunkLiveBytesMap,
    memory_chunk_typed_slots_map: MemoryChunkTypedSlotsMap,
    native_context_stats: NativeContextStats,
    local_pretenuring_feedback: HashMap<usize, usize>,
}

impl ConcurrentMarking {
    fn new(heap: *mut heap_mod::Heap, weak_objects: *mut weak_object_worklists::WeakObjects) -> Self {
        let max_tasks = if unsafe { flags::concurrent_marking_max_worker_num } == 0 {
            unsafe { v8_mod::GetCurrentPlatform() }.as_ref().map(|p| (*p).NumberOfWorkerThreads()).unwrap_or(1)
        } else {
            unsafe { flags::concurrent_marking_max_worker_num }
        };

        let mut task_state_ = Vec::with_capacity((max_tasks + 1) as usize);
        for _ in 0..=max_tasks {
            task_state_.push(UniquePtr::new(TaskState {
                marked_bytes: AtomicUsize::new(0),
                memory_chunk_live_bytes_map: MemoryChunkLiveBytesMap::new(),
                memory_chunk_typed_slots_map: HashMap::new(),
                native_context_stats: NativeContextStats {},
                local_pretenuring_feedback: HashMap::new(),
            }));
        }

        ConcurrentMarking {
            heap_: heap,
            weak_objects_: weak_objects,
            task_state_: task_state_,
            job_handle_: None,
            garbage_collector_: None,
            marking_worklists_: std::ptr::null_mut(),
            total_marked_bytes_: AtomicUsize::new(0),
            current_job_trace_id_: None,
            estimate_concurrency_: AtomicUsize::new(0),
            minor_marking_state_: None,
        }
    }

    fn run_major(&mut self, _delegate: &dyn JobDelegate, code_flush_mode: base::EnumSet<CodeFlushMode>, mark_compact_epoch: u32, should_keep_ages_unchanged: bool) {
       todo!()
        // let k_bytes_until_interrupt_check = 64 * KB;
        // let k_objects_until_interrupt_check = 1000;
        // let task_id = delegate.get_task_id() + 1;
        // let task_state = self.task_state_[task_id as usize].as_mut().unwrap();
        // let cpp_heap = unsafe { CppHeap::From((*self.heap_).cpp_heap()) };

        // let mut local_marking_worklists = MarkingWorklistsLocal::new(self.marking_worklists_, cpp_heap.map_or(0, |_| 1));
        // let mut local_weak_objects = WeakObjectsLocal::new(self.weak_objects_);
        // let mut visitor = ConcurrentMarkingVisitor {
        //     local_marking_worklists: &mut local_marking_worklists,
        //     local_weak_objects: &mut local_weak_objects,
        //     heap: self.heap_,
        //     mark_compact_epoch,
        //     code_flush_mode,
        //     should_keep_ages_unchanged,
        //     code_flushing_increase: unsafe { (*self.heap_).tracer().CodeFlushingIncrease() },
        //     memory_chunk_live_bytes_map: &mut task_state.memory_chunk_live_bytes_map,
        //     memory_chunk_typed_slots_map: &mut task_state.memory_chunk_typed_slots_map,
        // };
        // let mut native_context_inferrer = NativeContextInferrer::new();
        // let native_context_stats = &mut task_state.native_context_stats;
        // let mut time_ms = 0.0;
        // let mut marked_bytes = 0;
        // let isolate = unsafe { (*self.heap_).isolate() };
        // let mut another_ephemeron_iteration = false;
        // let new_space_allocator = unsafe { (*self.heap_).allocator().new_space_allocator() };

        // {
        //     let scope = TimedScope::new(&mut time_ms);

        //     {
        //         while let Some(ephemeron) = local_weak_objects.current_ephemerons_local.pop() {
        //             if visitor.process_ephemeron(ephemeron.key, ephemeron.value) {
        //                 another_ephemeron_iteration = true;
        //             }
        //         }
        //     }

        //     // Placeholder for PtrComprCageBase
        //     let cage_base = PtrComprCageBase::new();
        //     let is_per_context_mode = local_marking_worklists.is_per_context_mode();
        //     let mut done = false;

        //     while !done {
        //         let mut current_marked_bytes = 0;
        //         let mut objects_processed = 0;

        //         while current_marked_bytes < k_bytes_until_interrupt_check && objects_processed < k_objects_until_interrupt_check {
        //             if let Some(object) = local_marking_worklists.pop() {
        //                 objects_processed += 1;
        //                 let addr = object.address();
        //                 let new_space_top = if !new_space_allocator.is_null() { unsafe { (*new_space_allocator).original_top_acquire() } } else { 0 };
        //                 let new_space_limit = if !new_space_allocator.is_null() { unsafe { (*new_space_allocator).original_limit_relaxed() } } else { 0 };
        //                 let new_large_object = if unsafe { (*self.heap_).new_lo_space() }.is_null() { 0 } else { unsafe { (*(*self.heap_).new_lo_space()).pending_object() } };
        //                 if (new_space_top <= addr && addr < new_space_limit) || addr == new_large_object {
        //                     local_marking_worklists.push_on_hold(object);
        //                 } else {
        //                     let map = unsafe { (*object).map() };
        //                     if is_per_context_mode {
        //                         let mut context = 0;
        //                         if native_context_inferrer.infer(cage_base, map, object, &mut context) {
        //                             local_marking_worklists.switch_to_context(context);
        //                         }
        //                     }
        //                     let visited_size = visitor.visit(map, object);
        //                     visitor.increment_live_bytes