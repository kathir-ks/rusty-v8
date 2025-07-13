// Converted from V8 C++ source files:
// Header: incremental-marking.h
// Implementation: incremental-marking.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub struct TimeDelta {}
    impl TimeDelta {
        pub fn FromMilliseconds(ms: i64) -> TimeDelta {
            TimeDelta {}
        }
        pub fn Max() -> TimeDelta {
            TimeDelta {}
        }
        pub fn InMillisecondsF(&self) -> f64 {
            0.0
        }
        pub fn FromMillisecondsD(ms: f64) -> TimeDelta {
            TimeDelta {}
        }
    }
    pub struct TimeTicks {}
    impl TimeTicks {
        pub fn Now() -> TimeTicks {
            TimeTicks {}
        }
    }
    pub mod hashing {
        pub fn hash<T>(obj: &T) -> usize {
            0
        }
    }
    pub mod platform {
        pub struct Mutex {}
        impl Mutex {
            pub fn new() -> Mutex {
                Mutex {}
            }
            pub fn lock(&self) -> MutexGuard {
                MutexGuard {}
            }
        }
        pub struct MutexGuard {}
    }
}
pub mod common {
    pub mod globals {
        pub const KB: usize = 1024;
        pub const MB: usize = 1024 * KB;
    }
}
pub mod heap {
    pub struct Heap {}
    impl Heap {
        pub fn gc_state(&self) -> HeapState {
            HeapState::NOT_IN_GC
        }
        pub fn deserialization_complete(&self) -> bool {
            true
        }
        pub fn old_generation_allocation_limit(&self) -> usize {
            0
        }
        pub fn global_allocation_limit(&self) -> usize {
            0
        }
        pub fn IsTearingDown(&self) -> bool {
            false
        }
        pub fn allocator(&self) -> &Allocator {
            &Allocator {}
        }
        pub fn safepoint(&self) -> &Safepoint {
            &Safepoint {}
        }
        pub fn cpp_heap(&self) -> *mut CppHeap {
            std::ptr::null_mut()
        }
        pub fn lo_space(&self) -> &LoSpace {
            &LoSpace {}
        }
        pub fn code_lo_space(&self) -> &CodeLoSpace {
            &CodeLoSpace {}
        }
        pub fn shared_lo_space(&self) -> Option<&SharedLoSpace> {
            None
        }
        pub fn tracer(&self) -> &GCTracer {
            &GCTracer {}
        }
        pub fn mark_compact_collector(&self) -> &MarkCompactCollector {
            &MarkCompactCollector {}
        }
        pub fn concurrent_marking(&self) -> &ConcurrentMarking {
            &ConcurrentMarking {}
        }
        pub fn old_external_pointer_space(&self) -> &OldExternalPointerSpace {
            &OldExternalPointerSpace {}
        }
        pub fn young_external_pointer_space(&self) -> &YoungExternalPointerSpace {
            &YoungExternalPointerSpace {}
        }
        pub fn code_pointer_space(&self) -> &CodePointerSpace {
            &CodePointerSpace {}
        }
        pub fn trusted_pointer_space(&self) -> &TrustedPointerSpace {
            &TrustedPointerSpace {}
        }
        pub fn minor_mark_sweep_collector(&self) -> &MinorMarkSweepCollector {
            &MinorMarkSweepCollector {}
        }
        pub fn IsInYoungGeneration(obj: &HeapObject) -> bool {
            false
        }
        pub fn sweeper(&self) -> &Sweeper {
            &Sweeper {}
        }
        pub fn heap(&self) -> &Heap {
            self
        }
        pub fn global_handles(&self) -> &GlobalHandles {
            &GlobalHandles {}
        }
        pub fn is_shared(&self) -> bool {
            false
        }
        pub fn iterate(&self, f: &dyn Fn(&HeapObject)) {}
        pub fn SetIsMarkingFlag(&mut self, is_marking: bool) {}
        pub fn SetIsMinorMarkingFlag(&mut self, is_minor_marking: bool) {}
        pub fn minor_gc(&mut self, gc_reason: GarbageCollectionReason) {}
        pub fn IsLargeObject(obj: &HeapObject) -> bool {
            false
        }
        pub fn InFromPage(obj: Tagged<HeapObject>) -> bool {
            false
        }
        pub fn InToPage(obj: Tagged<HeapObject>) -> bool {
            false
        }
        pub fn InWritableSharedSpace(dest: Tagged<HeapObject>) -> bool {
            false
        }
        pub fn IsFreeSpaceOrFiller(obj: Tagged<HeapObject>, cage_base: PtrComprCageBase) -> bool {
            false
        }
        pub fn ShouldUseIncrementalMarking(&self) -> bool {
            false
        }
        pub fn EmbedderSizeOfObjects(&self) -> usize {
            0
        }
        pub fn InvokeIncrementalMarkingPrologueCallbacks(&self) {}
        pub fn FreeLinearAllocationAreas(&self) {}
        pub fn FreeSharedLinearAllocationAreasAndResetFreeLists(&self) {}
        pub fn MarkLinearAllocationAreasBlack(&self) {}
        pub fn UnmarkLinearAllocationsArea(&self) {}
        pub fn InvokeIncrementalMarkingEpilogueCallbacks(&self) {}
        pub fn FinalizeIncrementalMarkingAtomically(&self, reason: GarbageCollectionReason) {}
        pub fn OldGenerationSizeOfObjects(&self) -> usize {
            0
        }
        pub fn OldGenerationWastedBytes(&self) -> usize {
            0
        }
        pub fn IsTearingDown(&self) -> bool {
            false
        }
        pub fn IsLargeObject(obj: Tagged<HeapObject>) -> bool {
            false
        }
        pub fn IsLargeObject(obj: Tagged<HeapObject>) -> bool {
            false
        }
        pub fn DeserializationComplete() -> bool {
            true
        }
        pub fn GlobalSizeOfObjects(&self) -> usize {
            0
        }
        pub fn GlobalWastedBytes(&self) -> usize {
            0
        }
        pub fn tracer(&self) -> &GCTracer {
            &GCTracer {}
        }
        pub fn allocator(&self) -> &Allocator {
            &Allocator {}
        }
        pub fn marking_state(&self) -> &MarkingState {
            &MarkingState {}
        }
        pub fn IteratedRoots(&self, root_visitor: &mut RootMarkingVisitor, skip_root: base::EnumSet<SkipRoot>) {}
    }
    pub struct IncrementalMarkingJob {
        heap: *mut Heap
    }
    impl IncrementalMarkingJob {
        pub fn ScheduleTask(&self) {}
        pub fn AverageTimeToTask(&self) -> Option<base::TimeDelta> {
            None
        }
        pub fn CurrentTimeToTask(&self) -> Option<base::TimeDelta> {
            None
        }
    }
    pub struct MarkCompactCollector {}
    impl MarkCompactCollector {
        pub fn StartCompaction(&self, mode: StartCompactionMode) -> bool {
            false
        }
        pub fn StartMarking(&mut self) {}
        pub fn ProcessMarkingWorklist(&mut self, max_duration: base::TimeDelta, marked_bytes_limit: usize) -> (usize, bool) {
            (0, false)
        }
        pub fn local_marking_worklists(&self) -> &MarkingWorklists {
            &MarkingWorklists {}
        }
    }
    pub struct MinorMarkSweepCollector {}
    impl MinorMarkSweepCollector {
        pub fn StartMarking(&mut self, use_background_threads: bool) {}
        pub fn local_marking_worklists(&self) -> &MarkingWorklists {
            &MarkingWorklists {}
        }
    }
    pub struct MarkingWorklists {}
    impl MarkingWorklists {
        pub fn IsEmpty(&self) -> bool {
            false
        }
    }
    pub struct LoSpace {}
    pub struct CodeLoSpace {}
    pub struct SharedLoSpace {}
    pub struct Allocator {}
    impl Allocator {
        pub fn AddAllocationObserver(&self, old_generation_observer: &Observer, new_generation_observer: &Observer) {}
        pub fn RemoveAllocationObserver(&self, old_generation_observer: &Observer, new_generation_observer: &Observer) {}
        pub fn FreeLinearAllocationAreasAndResetFreeLists(&self) {}
    }
    pub struct Safepoint {}
    impl Safepoint {
        pub fn IterateLocalHeaps(&self, f: fn(&LocalHeap)) {}
    }
    pub struct LocalHeap {}
    impl LocalHeap {
        pub fn FreeLinearAllocationAreasAndResetFreeLists(&self) {}
        pub fn MarkLinearAllocationAreasBlack(&self) {}
        pub fn UnmarkLinearAllocationsArea(&self) {}
    }
    pub struct GCTracer {}
    impl GCTracer {
        pub fn CurrentEpoch(&self, scope_id: GCTracerScope) -> i32 {
            0
        }
        pub fn NotifyIncrementalMarkingStart(&self) {}
        pub fn AddIncrementalMarkingStep(&self, duration: f64, marked_bytes: usize) {}
        pub fn IncrementalMarkingSpeedInBytesPerMillisecond(&self) -> f64 {
            0.0
        }
    }
    pub struct ConcurrentMarking {}
    impl ConcurrentMarking {
        pub fn TryScheduleJob(&self, garbage_collector: GarbageCollector) {}
        pub fn TotalMarkedBytes(&self) -> usize {
            0
        }
        pub fn RescheduleJobIfNeeded(&self, garbage_collector: GarbageCollector) {}
    }
    pub struct OldExternalPointerSpace {}
    pub struct YoungExternalPointerSpace {}
    pub struct CodePointerSpace {}
    pub struct TrustedPointerSpace {}
    pub struct Sweeper {}
    impl Sweeper {
        pub fn PauseMajorSweepingScope(&self) -> PauseMajorSweepingScope {
            PauseMajorSweepingScope {}
        }
    }
    pub struct PauseMajorSweepingScope {}
    pub struct GlobalHandles {}
    impl GlobalHandles {
        pub fn IterateYoungStrongAndDependentRoots(&self, root_visitor: &YoungGenerationRootMarkingVisitor) {}
    }
    pub struct MarkingBarrier {}
    impl MarkingBarrier {
        pub fn ActivateAll(heap: &Heap, is_compacting: bool) {}
        pub fn ActivateYoung(heap: &Heap) {}
    }
    pub struct HeapObject {}
    impl HeapObject {
        pub fn map_word(&self, cage_base: PtrComprCageBase, mode: std::sync::atomic::Ordering) -> MapWord {
            MapWord {}
        }
        pub fn Size(&self) -> i32 {
            0
        }
        pub fn map(&self, cage_base: PtrComprCageBase) -> Tagged<Map> {
            Tagged{}
        }
        pub fn address(&self) -> Address {
            Address{}
        }
    }
    pub struct Tagged<T> {dummy: i32}
    pub struct Map {}
    pub struct HeapState {}
    impl HeapState {
        pub const NOT_IN_GC: HeapState = HeapState {};
    }
    pub struct Address {}
    pub struct PtrComprCageBase {}
    pub struct MapWord {}
    pub struct RootMarkingVisitor {}
    pub struct YoungGenerationRootMarkingVisitor {}
    pub struct Observer {}
    impl Observer {
        pub fn Step(&mut self, bytes_allocated: i32, address: Address, size: usize) {}
    }
    pub struct PagedSpaceIterator {}
    impl PagedSpaceIterator {
        pub fn Next(&self) -> *mut PagedSpace {
            std::ptr::null_mut()
        }
    }
    pub struct PagedSpace {}
    impl PagedSpace {
        pub fn identity(&self) -> SpaceIdentity {
            SpaceIdentity::NEW_SPACE
        }
        pub fn SizeOfObjects(&self) -> usize {
            0
        }
    }
    pub enum SpaceIdentity {
        NEW_SPACE,
        SHARED_SPACE,
    }
    pub enum StartCompactionMode {
        kIncremental
    }
    pub struct MarkingState {}
    impl MarkingState {
        pub fn TryMark(&self, obj: Tagged<HeapObject>) -> bool {
            true
        }
        pub fn IsUnmarked(&self, obj: Tagged<HeapObject>) -> bool {
            false
        }
    }
    pub struct MutablePageMetadata {}
    impl MutablePageMetadata {
        pub fn FromHeapObject(obj: &HeapObject) -> *mut MutablePageMetadata {
            std::ptr::null_mut()
        }
        pub fn IncrementLiveBytesAtomically(&self, bytes: i64) {}
    }
    pub struct MarkingBitmap {}
    impl MarkingBitmap {
        pub fn FindPreviousValidObject(metadata: *const PageMetadata, old_handle_location: Address) -> Address {
            Address {}
        }
    }
    pub struct ExternalPointerHandle {}
    pub struct MemoryChunk {}
    impl MemoryChunk {
        pub fn FromAddress(address: Address) -> *const MemoryChunk {
            std::ptr::null()
        }
        pub fn Metadata(&self) -> *const PageMetadata {
            std::ptr::null()
        }
        pub fn InYoungGeneration(&self) -> bool {
            false
        }
    }
    pub struct PageMetadata {}
    pub mod base {
        pub struct EnumSet<T> {dummy: i32}
    }
    pub enum SkipRoot {
        kStack,
        kMainThreadHandles,
        kTracedHandles,
        kWeak,
        kReadOnlyBuiltins,
        kExternalStringTable,
        kGlobalHandles,
        kOldGeneration
    }
    pub struct WeakObjects {}
}
pub mod tasks {
    pub struct CancelableTask {}
}
pub mod v8 {
    pub struct Isolate {}
    impl Isolate {
        pub fn serializer_enabled(&self) -> bool {
            false
        }
        pub fn stack_guard(&self) -> &StackGuard {
            &StackGuard {}
        }
        pub fn has_shared_space(&self) -> bool {
            false
        }
        pub fn is_shared_space_isolate(&self) -> bool {
            false
        }
        pub fn shared_space_isolate(&self) -> &Isolate {
            self
        }
        pub fn counters(&self) -> &Counters {
            &Counters {}
        }
        pub fn traced_handles(&self) -> &TracedHandles {
            &TracedHandles {}
        }
        pub fn PrintWithTimestamp(&self, message: &str, args: ...) {}
        pub fn serializer_enabled(&self) -> bool {
            false
        }
        pub fn PrintWithTimestamp(&self, message: &str, args: ...) {}
    }
    pub struct StackGuard {}
    impl StackGuard {
        pub fn RequestGC(&self) {}
        pub fn ClearGC(&self) {}
    }
    pub struct Counters {}
    impl Counters {
        pub fn incremental_marking_reason(&self) -> &IncrementalMarkingReason {
            &IncrementalMarkingReason {}
        }
        pub fn gc_incremental_marking_start(&self) -> &NestedTimedHistogram {
            &NestedTimedHistogram {}
        }
        pub fn gc_minor_incremental_marking_start(&self) -> &NestedTimedHistogram {
            &NestedTimedHistogram {}
        }
        pub fn gc_incremental_marking(&self) -> &NestedTimedHistogram {
            &NestedTimedHistogram {}
        }
    }
    pub struct IncrementalMarkingReason {}
    impl IncrementalMarkingReason {
        pub fn AddSample(&self, reason: i32) {}
    }
    pub struct NestedTimedHistogram {}
    pub struct TracedHandles {}
    impl TracedHandles {
        pub fn SetIsMarking(&self, is_marking: bool) {}
    }
}
pub mod flags {
    pub mod flags {
        pub static incremental_marking: bool = false;
        pub static trace_incremental_marking: bool = false;
        pub static incremental_marking_task: bool = false;
        pub static predictable: bool = false;
        pub static concurrent_marking: bool = false;
        pub static separate_gc_phases: bool = false;
        pub static minor_ms: bool = false;
        pub static concurrent_minor_ms_marking: bool = false;
        pub static black_allocated_pages: bool = false;
        pub static incremental_marking_unified_schedule: bool = false;
    }
}
pub mod execution {
    pub struct VMState<T> {dummy: i32}
    impl<T> VMState<T> {
        pub fn new(isolate: &v8::Isolate) -> VMState<T> {
            VMState{dummy: 0}
        }
    }
    pub struct SafepointScope {dummy: i32}
}
pub mod logging {
    pub struct RuntimeCallStatsScope {dummy: i32}
}
pub mod numbers {
    pub struct conversions {}
}
pub mod objects {
    pub struct slots_inl {}
    pub struct visitors {}
    pub struct data_handler_inl {}
}
pub mod tracing {
    pub struct trace_event {}
}
pub mod utils {
    pub struct utils {}
}
pub mod init {
    pub struct v8 {}
}
pub mod handles {
    pub struct global_handles {}
}
pub enum GarbageCollector {
    MARK_COMPACTOR,
    MINOR_MARK_SWEEPER
}
pub enum GarbageCollectionReason {
    kFinalizeMarkingViaTask,
    kFinalizeMarkingViaStackGuard
}
pub struct CppHeap {}
impl CppHeap {
    pub fn From(heap: *mut CppHeap) -> *mut CppHeap {
        heap
    }
    pub fn incremental_marking_supported(&self) -> bool {
        false
    }
    pub fn AdvanceMarking(&self, max_duration: base::TimeDelta, marked_bytes_limit: usize) {}
    pub fn last_bytes_marked(&self) -> usize {
        0
    }
    pub fn ShouldFinalizeIncrementalMarking(&self) -> bool {
        false
    }
    pub fn used_size(&self) -> usize {
        0
    }
    pub fn StartMarking(&self) {}
}
pub mod heap {
    pub mod base {
        pub struct IncrementalMarkingSchedule {}
        impl IncrementalMarkingSchedule {
            pub fn Create(predictable: bool) -> std::shared_ptr<IncrementalMarkingSchedule> {
                std::shared_ptr::new(IncrementalMarkingSchedule {})
            }
            pub fn NotifyIncrementalMarkingStart(&self) {}
            pub fn GetNextIncrementalStepDuration(&self, estimated_live_bytes: usize) -> usize {
                0
            }
            pub fn RemoveMutatorThreadMarkedBytes(&self, dead_bytes_marked: usize) {}
            pub fn AddConcurrentlyMarkedBytes(&self, delta: usize) {}
            pub fn GetCurrentStepInfo(&self) -> StepInfo {
                StepInfo {}
            }
            pub const kStepSizeWhenNotMakingProgress: usize = 8 * 1024 * 1024;
        }
        pub struct StepInfo {
            pub elapsed_time: super::super::base::TimeDelta,
            pub marked_bytes: usize,
            pub mutator_marked_bytes: usize,
            pub concurrent_marked_bytes: usize,
            pub expected_marked_bytes: usize,
            pub estimated_live_bytes: usize,
            pub scheduled_delta_bytes: i64,
        }
    }
}
pub mod base {
    pub struct EnumSet<T> {
        dummy: i32,
    }
}
pub mod v8 {
    pub struct AllowGarbageCollection {}
}
pub mod execution {
    pub enum SafepointKind {
        kIsolate
    }
    pub struct SafepointScope {
        dummy: i32,
    }
    impl SafepointScope {
        pub fn new(isolate: &super::v8::Isolate, kind: SafepointKind) -> Self {
            Self { dummy: 0 }
        }
    }
}
pub mod internal {
    use crate::base;
    use crate::common;
    use crate::flags::flags;
    use crate::heap;
    use crate::v8;
    use crate::execution;
    use crate::logging;
    use crate::numbers;
    use crate::objects;
    use crate::tracing;
    use crate::utils;
    use crate::init;
    use crate::handles;
    use crate::heap::base::IncrementalMarkingSchedule;
    use std::sync::atomic::Ordering;

    use std::sync::Mutex;
    use std::collections::HashMap;

    pub enum class StepOrigin {
        kV8,
        kTask
    }

    impl StepOrigin {
        fn ToString(&self) -> &'static str {
            match self {
                StepOrigin::kV8 => "V8",
                StepOrigin::kTask => "task"
            }
        }
    }

    pub struct IncrementalMarking {
        heap_: *mut heap::Heap,
        major_collector_: *mut heap::MarkCompactCollector,
        minor_collector_: *mut heap::MinorMarkSweepCollector,
        weak_objects_: *mut heap::WeakObjects,
        current_local_marking_worklists_: *mut MarkingWorklistsLocal,
        marking_state_: *mut heap::MarkingState,
        start_time_: base::TimeTicks,
        main_thread_marked_bytes_: usize,
        bytes_marked_concurrently_: usize,
        marking_mode_: MarkingMode,
        is_compacting_: bool,
        black_allocation_: bool,
        completion_task_scheduled_: bool,
        completion_task_timeout_: base::TimeTicks,
        major_collection_requested_via_stack_guard_: bool,
        incremental_marking_job_: Option<Box<IncrementalMarkingJob>>,
        new_generation_observer_: Observer,
        old_generation_observer_: Observer,
        background_live_bytes_mutex_: Mutex,
        background_live_bytes_: HashMap<*mut MutablePageMetadata, i64>,
        schedule_: Option<std::shared_ptr<IncrementalMarkingSchedule>>,
        current_trace_id_: Option<u64>
    }

    impl IncrementalMarking {
        pub fn new(heap: *mut heap::Heap, weak_objects: *mut heap::WeakObjects) -> IncrementalMarking {
            let major_collector = unsafe { (*heap).mark_compact_collector() as *const _ as *mut heap::MarkCompactCollector };
            let minor_collector = unsafe { (*heap).minor_mark_sweep_collector() as *const _ as *mut heap::MinorMarkSweepCollector };
            let marking_state = unsafe { (*heap).marking_state() as *const _ as *mut heap::MarkingState };
            IncrementalMarking {
                heap_: heap,
                major_collector_: major_collector,
                minor_collector_: minor_collector,
                weak_objects_: weak_objects,
                current_local_marking_worklists_: std::ptr::null_mut(),
                marking_state_: marking_state,
                start_time_: base::TimeTicks::Now(),
                main_thread_marked_bytes_: 0,
                bytes_marked_concurrently_: 0,
                marking_mode_: MarkingMode::kNoMarking,
                is_compacting_: false,
                black_allocation_: false,
                completion_task_scheduled_: false,
                completion_task_timeout_: base::TimeTicks::Now(),
                major_collection_requested_via_stack_guard_: false,
                incremental_marking_job_: if flags::incremental_marking_task {
                    unsafe { Some(Box::new(IncrementalMarkingJob{heap: heap})) }
                } else {
                    None
                },
                new_generation_observer_: Observer::new( &IncrementalMarking{
                    heap_: heap,
                    major_collector_: major_collector,
                    minor_collector_: minor_collector,
                    weak_objects_: weak_objects,
                    current_local_marking_worklists_: std::ptr::null_mut(),
                    marking_state_: marking_state,
                    start_time_: base::TimeTicks::Now(),
                    main_thread_marked_bytes_: 0,
                    bytes_marked_concurrently_: 0,
                    marking_mode_: MarkingMode::kNoMarking,
                    is_compacting_: false,
                    black_allocation_: false,
                    completion_task_scheduled_: false,
                    completion_task_timeout_: base::TimeTicks::Now(),
                    major_collection_requested_via_stack_guard_: false,
                    incremental_marking_job_: if flags::incremental_marking_task {
                        unsafe { Some(Box::new(IncrementalMarkingJob{heap: heap})) }
                    } else {
                        None
                    },
                    new_generation_observer_: Observer::new( &IncrementalMarking{
                        heap_: heap,
                        major_collector_: major_collector,
                        minor_collector_: minor_collector,
                        weak_objects_: weak_objects,
                        current_local_marking_worklists_: std::ptr::null_mut(),
                        marking_state_: marking_state,
                        start_time_: base::TimeTicks::Now(),
                        main_thread_marked_bytes_: 0,
                        bytes_marked_concurrently_: 0,
                        marking_mode_: MarkingMode::kNoMarking,
                        is_compacting_: false,
                        black_allocation_: false,
                        completion_task_scheduled_: false,
                        completion_task_timeout_: base::TimeTicks::Now(),
                        major_collection_requested_via_stack_guard_: false,
                        incremental_marking_job_: if flags::incremental_marking_task {
                            unsafe { Some(Box::new(IncrementalMarkingJob{heap: heap})) }
                        } else {
                            None
                        },
                        new_generation_observer_: Observer::new( &IncrementalMarking{
                            heap_: heap,
                            major_collector_: major_collector,
                            minor_collector_: minor_collector,
                            weak_objects_: weak_objects,
                            current_local_marking_worklists_: std::ptr::null_mut(),
                            marking_state_: marking_state,
                            start_time_: base::TimeTicks::Now(),
                            main_thread_marked_bytes_: 0,
                            bytes_marked_concurrently_: 0,
                            marking_mode_: MarkingMode::kNoMarking,
                            is_compacting_: false,
                            black_allocation_: false,
                            completion_task_scheduled_: false,
                            completion_task_timeout_: base::TimeTicks::Now(),
                            major_collection_requested_via_stack_guard_: false,
                            incremental_marking_job_: if flags::incremental_marking_task {
                                unsafe { Some(Box::new(IncrementalMarkingJob{heap: heap})) }
                            } else {
                                None
                            },
                            new_generation_observer_: Observer::new( &IncrementalMarking{
                                heap_: heap,
                                major_collector_: major_collector,
                                minor_collector_: minor_collector,
                                weak_objects_: weak_objects,
                                current_local_marking_worklists_: std::ptr::null_mut(),
                                marking_state_: marking_state,
                                start_time_: base::TimeTicks::Now(),
                                main_thread_marked_bytes_: 0,
                                bytes_marked_concurrently_: 0,
                                marking_mode_: MarkingMode::kNoMarking,
                                is_compacting_: false,
                                black_allocation_: false,
                                completion_task_scheduled_: false,
                                completion_task_timeout_: base::TimeTicks::Now(),
                                major_collection_requested_via_stack_guard_: false,
                                incremental_marking_job_: if flags::incremental_marking_task {
                                    unsafe { Some(Box::new(IncrementalMarkingJob{heap: heap})) }
                                } else {
                                    None
                                },
                                new_generation_observer_: Observer::new( &IncrementalMarking{
                                    heap_: heap,
                                    major_collector_: major_collector,
                                    minor_collector_: minor_collector,
                                    weak_objects_: weak_objects,
                                    current_local_marking_worklists_: std::ptr::null_mut(),
                                    marking_state_: marking_state,
                                    start_time_: base::TimeTicks::Now(),
                                    main_thread_marked_bytes_: 0,
                                    bytes_marked_concurrently_: 0,
                                    marking_mode_: MarkingMode::kNoMarking,
                                    is_compacting_: false,
                                    black_allocation_: false,
                                    completion_task_scheduled_: false,
                                    completion_task_timeout_: base::TimeTicks::Now(),
                                    major_collection_requested_via_stack_guard_: false,
                                    incremental_marking_job_: if flags::incremental_marking_task {
                                        unsafe { Some(Box::new(IncrementalMarkingJob{heap: heap})) }
                                    } else {
                                        None
                                    },
                                    new_generation_observer_: Observer::new( &IncrementalMarking{
                                        heap_: heap,
                                        major_collector_: major_collector,
                                        minor_collector_: minor_collector,
                                        weak_objects_: weak_objects,
                                        current_local_marking_worklists_: std::ptr::null_mut(),
                                        marking_state_: marking_state,
                                        start_time_: base::TimeTicks::Now(),
                                        main_thread_marked_bytes_: 0,
                                        bytes_marked_concurrently_: 0,
                                        marking_mode_: MarkingMode::kNoMarking,
                                        is_compacting_: false,
                                        black_allocation_: false,
                                        completion_task_scheduled_: false,
                                        completion_task_timeout_: base::TimeTicks::Now(),
                                        major_collection_requested_via_stack_guard_: false,
                                        incremental_marking_job_: if flags::incremental_marking_task {
                                            unsafe { Some(Box::new(IncrementalMarkingJob{heap: heap})) }
                                        } else {
                                            None
                                        },
                                        new_generation_observer_: Observer::new( &IncrementalMarking{
                                            heap_: heap,
                                            major_collector_: major_collector,
                                            minor_collector_: minor_collector,
                                            weak_objects_: weak_objects,
                                            current_local_marking_worklists_: std::ptr::null_mut(),
                                            marking_state_: marking_state,
                                            start_time_: base::TimeTicks::Now(),
                                            main_thread_marked_bytes_: 0,
                                            bytes_marked_concurrently_: 0,
                                            marking_mode_: MarkingMode::kNoMarking,
                                            is_compacting_: false,
                                            black_allocation_: false,
                                            completion_task_scheduled_: false,
                                            completion_task_timeout_: base::TimeTicks::Now(),
                                            major_collection_requested_via_stack_guard_: false,
                                            incremental_marking_job_: if flags::incremental_marking_task {
                                                unsafe { Some(Box::new(IncrementalMarkingJob{heap: heap})) }
                                            } else {
                                                None
                                            },
                                            new_generation_observer_: Observer::new( &IncrementalMarking{
                                                heap_: heap,
                                                major_collector_: major_collector,
                                                minor_collector_: minor_collector,
                                                weak_objects_: weak_objects,
                                                current_local_marking_worklists_: std::ptr::null_mut(),
                                                marking_state_: marking_state,
                                                start_time_: base::TimeTicks::Now(),
                                                main_thread_marked_bytes_: 0,
                                                bytes_marked_concurrently_: 0,
                                                marking_mode_: MarkingMode::kNoMarking,
                                                is_compacting_: false,
                                                black_allocation_: false,
                                                completion_task_scheduled_: false,
                                                completion_task_timeout_: base::TimeTicks::Now(),
                                                major_collection_requested_via_stack_guard_: false,
                                                incremental_marking_job_: if flags::incremental_marking_task {
                                                    unsafe { Some(Box::new(IncrementalMarkingJob{heap: heap})) }
                                                } else {
                                                    None
                                                },
                                                new_generation_observer_: Observer::new( &IncrementalMarking{
                                                    heap_: heap,
                                                    major_collector_: major_collector,
                                                    minor_collector_: minor_collector,
                                                    weak_objects_: weak_objects,
                                                    current_local_marking_worklists_: std::ptr::null_mut(),
                                                    marking_state_: marking_state,
                                                    start_time_: base::TimeTicks::Now(),
                                                    main_thread_marked_bytes_: 0,
                                                    bytes_marked_concurrently_: 0,
                                                    marking_mode_: MarkingMode::kNoMarking,
                                                    is_compacting_: false,
                                                    black_allocation_: false,
                                                    completion_task_scheduled_: false,
                                                    completion_task_timeout_: base::TimeTicks::Now(),
                                                    major_collection_requested_via_stack_guard_: false,
                                                    incremental_marking_job_: if flags::incremental_marking_task {
                                                        unsafe { Some(Box::new(IncrementalMarkingJob{heap: heap})) }
                                                    } else {
                                                        None
                                                    },
                                                    new_generation_observer_: Observer::new( &IncrementalMarking{
                                                        heap_: heap,
                                                        major_collector_: major_collector,
                                                        minor_collector_: minor_collector,
                                                        weak_objects_: weak_objects,
                                                        current_local_marking_worklists_: std::ptr::null_mut(),
                                                        marking_state_: marking_state,
                                                        start_time_: base::TimeTicks::Now(),
                                                        main_thread_marked_bytes_: 0,
                                                        bytes_marked_concurrently_: 0,
                                                        marking_mode_: MarkingMode::kNoMarking,
                                                        is_compacting_: false,
                                                        black_allocation_: false,
                                                        completion_task_scheduled_: false,
                                                        completion_task_timeout_: base::TimeTicks::Now(),
                                                        major_collection_requested_via_stack_guard_: false,
                                                        incremental_marking_job_: if flags::incremental_marking_task {
                                                            unsafe { Some(Box::new(IncrementalMarkingJob{
