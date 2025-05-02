// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Note: This is a partial translation. Many parts of the original C++
// header rely on other V8 internal components and require further elaboration.

use std::sync::{Mutex, MutexGuard, Arc};
use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, AtomicU8, AtomicBool, Ordering};
use std::time::{Duration, Instant};
use std::ops::{Add, Sub, BitOr, BitAnd, BitXor, Not, Shl, Shr};
use std::fmt;

// Minimal definitions for include files
mod v8_callbacks {
    pub type NearHeapLimitCallback = fn(data: *mut std::ffi::c_void);
}

mod v8_embedder_heap {
    // Placeholder
}

mod v8_internal {
    // Placeholder
}

mod v8_isolate {
    // Placeholder
    pub type GCCallbackWithData = fn(data: *mut std::ffi::c_void);
    pub enum UseCounterFeature {}
    pub enum MemoryPressureLevel {
        kNone,
        kModerate,
        kCritical,
    }
}

mod src_base {
    pub mod atomic_utils {
        // Placeholder
    }
    pub mod enum_set {
        use std::ops::{BitOr, BitAnd, BitXor, Not};
        use std::fmt;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct EnumSet<T: Copy + Eq + std::hash::Hash, U: Copy> {
            bits: U,
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T: Copy + Eq + std::hash::Hash, U: Copy + Default> EnumSet<T, U> {
            pub fn new() -> Self {
                EnumSet {
                    bits: U::default(),
                    _phantom: std::marker::PhantomData,
                }
            }
        }

        impl<T: Copy + Eq + std::hash::Hash, U: Copy + BitOr<Output = U> + BitAnd<Output = U> + BitXor<Output = U> + Not<Output = U> + PartialEq + From<u8> + Into<u8> + Default> EnumSet<T, U> {
            pub fn contains(&self, flag: T) -> bool {
                // Requires From and Into implementations for U to u8
                let flag_u8: u8 = unsafe { std::mem::transmute_copy(&flag) };
                let bits_u8: u8 = self.bits.into();
                (bits_u8 & flag_u8) != 0
            }

            pub fn insert(&mut self, flag: T) {
                let flag_u8: u8 = unsafe { std::mem::transmute_copy(&flag) };
                let bits_u8: u8 = self.bits.into();
                self.bits = (bits_u8 | flag_u8).into();
            }

            pub fn remove(&mut self, flag: T) {
                let flag_u8: u8 = unsafe { std::mem::transmute_copy(&flag) };
                let bits_u8: u8 = self.bits.into();
                self.bits = (bits_u8 & !flag_u8).into();
            }
        }

        impl<T: Copy + Eq + std::hash::Hash, U: Copy + BitOr<Output = U> + BitAnd<Output = U> + BitXor<Output = U> + Not<Output = U> + PartialEq + From<u8> + Into<u8> + Default> BitOr for EnumSet<T, U> {
            type Output = Self;

            fn bitor(self, other: Self) -> Self {
                let bits_u8_self: u8 = self.bits.into();
                let bits_u8_other: u8 = other.bits.into();
                EnumSet {
                    bits: (bits_u8_self | bits_u8_other).into(),
                    _phantom: std::marker::PhantomData,
                }
            }
        }

        // Implement other bitwise operators similarly (BitAnd, BitXor, Not)
        macro_rules! define_operators_for_flags {
            ($flags_type:ident) => {
                impl std::ops::BitOr for $flags_type {
                    type Output = Self;
                    fn bitor(self, other: Self) -> Self {
                        $flags_type(self.0 | other.0)
                    }
                }

                impl std::ops::BitAnd for $flags_type {
                    type Output = Self;
                    fn bitand(self, other: Self) -> Self {
                        $flags_type(self.0 & other.0)
                    }
                }
            }
        }
    }
    pub mod platform {
        pub struct Mutex {
            inner: std::sync::Mutex<()>,
        }

        impl Mutex {
            pub fn new() -> Self {
                Mutex { inner: std::sync::Mutex::new(()) }
            }

            pub fn lock(&self) -> Result<MutexGuard<()>, std::sync::PoisonError<MutexGuard<()>>> {
                self.inner.lock()
            }
        }
        pub struct ConditionVariable {
            inner: std::sync::Condvar,
        }

        impl ConditionVariable {
            pub fn new() -> Self {
                ConditionVariable { inner: std::sync::Condvar::new() }
            }

            pub fn wait<'a, T>(&self, guard: MutexGuard<'a, T>) -> Result<MutexGuard<'a, T>, std::sync::PoisonError<MutexGuard<'a, T>>> {
                // This requires more sophisticated handling in a real implementation.
                // The Rust Condvar API differs significantly from the C++ one.
                // This placeholder returns the guard immediately, which is incorrect.
                Ok(guard)
            }
        }
    }
    pub mod small_vector {
        // Placeholder
    }
}

mod src_builtins {
    pub mod accessors {
        // Placeholder
    }
}

mod src_common {
    pub mod assert_scope {
        // Placeholder
    }
    pub mod code_memory_access {
        // Placeholder
    }
    pub mod globals {
        // Placeholder
        pub const KB: usize = 1024;
        pub const MB: usize = 1024 * KB;
        pub const kTaggedSize: usize = 8;
        pub const kSystemPointerSize: usize = 8;
    }
}

mod src_heap {
    pub mod allocation_observer {
        // Placeholder
    }
    pub mod allocation_result {
        // Placeholder
        pub enum AllocationResult {
            Success,
            Failure,
        }
    }
    pub mod gc_callbacks {
        // Placeholder
    }
    pub mod heap_allocator {
        // Placeholder
    }
    pub mod marking_state {
        // Placeholder
    }
    pub mod minor_gc_job {
        // Placeholder
    }
    pub mod pretenuring_handler {
        // Placeholder
    }
    pub mod sweeper {
        // Placeholder
    }
}

mod src_init {
    pub mod heap_symbols {
        // Placeholder
    }
}

mod src_objects {
    pub mod allocation_site {
        // Placeholder
    }
    pub mod fixed_array {
        // Placeholder
        pub struct FixedArrayBase {}
    }
    pub mod hash_table {
        // Placeholder
    }
    pub mod heap_object {
        // Placeholder
        use std::hash::{Hasher, BuildHasher};
        use std::any::Any;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct HeapObject {}

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct MaybeObject {}

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct Tagged<T> {
            pub ptr: *mut T,
            _phantom: PhantomData<T>,
        }

        impl<T> Tagged<T> {
            pub fn new(ptr: *mut T) -> Self {
                Tagged { ptr, _phantom: PhantomData }
            }

            pub fn from_raw(ptr: *mut T) -> Self {
              Tagged { ptr, _phantom: PhantomData }
            }

            pub fn as_ptr(&self) -> *mut T {
                self.ptr
            }
        }

        impl Tagged<HeapObject> {
          pub fn unsafe_from_address(address: usize) -> Self {
              Tagged::new(address as *mut HeapObject)
          }
        }

        pub trait Object : Any {
            fn is_identical_to(&self, other: &dyn Object) -> bool;
            fn get_hash_code(&self) -> usize;
        }

        pub struct Hasher {}

        impl Hasher {
            pub fn finish(&self) -> u64 { 0 }
            pub fn write(&mut self, bytes: &[u8]) {}
            pub fn write_u8(&mut self, i: u8) {}
            pub fn write_u32(&mut self, i: u32) {}
            pub fn write_u64(&mut self, i: u64) {}
        }

        pub trait KeyEqualSafe {
            fn key_equal_safe(a: &Tagged<HeapObject>, b: &Tagged<HeapObject>) -> bool;
        }

        impl Object {
          pub fn Hasher() -> Hasher {
              Hasher {}
          }
          pub fn KeyEqualSafe(a: &Tagged<HeapObject>, b: &Tagged<HeapObject>) -> bool {
              a.ptr == b.ptr
          }
        }
    }
    pub mod js_array_buffer {
        // Placeholder
    }
    pub mod objects {
        // Placeholder
    }
    pub mod smi {
        // Placeholder
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct Smi(i32);

        impl Smi {
            pub fn zero() -> Self {
                Smi(0)
            }
        }
    }
    pub mod visitors {
        // Placeholder
    }
}

mod src_roots {
    pub mod roots {
        // Placeholder
        pub struct RootsTable {}
    }
}

mod src_sandbox {
    pub mod code_pointer_table {
        // Placeholder
    }
    pub mod external_pointer_table {
        // Placeholder
    }
    pub mod js_dispatch_table {
        // Placeholder
    }
    pub mod trusted_pointer_table {
        // Placeholder
    }
}

mod src_utils {
    pub mod allocation {
        // Placeholder
    }
}

// Testing only
mod testing {
    pub mod gtest {
        pub mod include {
            pub mod gtest {
                // Placeholder
            }
        }
    }
}

mod cppgc {
    pub mod internal {
        pub enum HeapObjectNameForUnnamedObject { }
        pub struct ClassNameAsHeapObjectNameScope {}
    }
}

mod heap_base {
    pub mod stack {
        pub struct Stack {}
        pub struct StackVisitor {}
    }
}

mod debug {
    pub type OutOfMemoryCallback = fn(data: *mut std::ffi::c_void);
}

mod heap {
    pub struct HeapTester {}
    pub struct TestMemoryAllocatorScope {}
}

mod internal {
    pub struct ArrayBufferCollector {}
    pub struct ArrayBufferSweeper {}
    pub struct BackingStore {}
    pub struct MemoryChunkMetadata {}

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Boolean {}
    pub struct CodeLargeObjectSpace {}
    pub struct CodeRange {}
    pub struct CollectionBarrier {}
    pub struct ConcurrentMarking {}
    pub struct CppHeap {}
    pub struct EphemeronRememberedSet {}
    pub struct GCTracer {}
    pub struct IncrementalMarking {}
    pub struct IsolateSafepoint {}
    pub struct HeapObjectAllocationTracker {}
    pub struct HeapObjectsFilter {}
    pub struct HeapProfiler {}
    pub struct HeapStats {}
    pub struct Isolate {}
    pub struct JSArrayBuffer {}
    pub struct JSFinalizationRegistry {}
    pub struct JSPromise {}
    pub struct LinearAllocationArea {}
    pub struct LocalHeap {}
    pub struct MemoryAllocator {}
    pub struct MemoryBalancer {}
    pub struct MutablePageMetadata {}
    pub struct MemoryMeasurement {}
    pub struct MemoryReducer {}
    pub struct MinorMarkSweepCollector {}
    pub struct NativeContext {}
    pub struct NopRwxMemoryWriteScope {}
    pub struct ObjectIterator {}
    pub struct ObjectStats {}
    pub struct PageMetadata {}
    pub struct PagedSpace {}
    pub struct PagedNewSpace {}
    pub struct ReadOnlyHeap {}
    pub struct RootVisitor {}
    pub struct RwxMemoryWriteScope {}
    pub struct SafepointScope {}
    pub struct Scavenger {}
    pub struct ScavengerCollector {}
    pub struct SemiSpaceNewSpace {}
    pub struct SharedLargeObjectSpace {}
    pub struct SharedReadOnlySpace {}
    pub struct SharedSpace {}
    pub struct SharedTrustedLargeObjectSpace {}
    pub struct SharedTrustedSpace {}
    pub struct Space {}
    pub struct StickySpace {}
    pub struct StressScavengeObserver {}
    pub struct TimedHistogram {}
    pub struct TrustedLargeObjectSpace {}
    pub struct TrustedRange {}
    pub struct TrustedSpace {}
    pub struct WeakObjectRetainer {}
    pub struct GlobalHandleVector<T> {
        _phantom: PhantomData<T>
    }
    impl<T> GlobalHandleVector<T> {
        pub fn new() -> Self {
          GlobalHandleVector{_phantom: PhantomData}
        }
    }

    use src_objects::heap_object::{Tagged, HeapObject, Object};
    use src_base::enum_set::EnumSet;
    use v8_isolate::{MemoryPressureLevel};
    use std::cell::Cell;

    pub enum ClearRecordedSlots { kYes, kNo }

    pub enum InvalidateRecordedSlots { kYes, kNo }

    pub enum InvalidateExternalPointerSlots { kYes, kNo }

    pub enum ClearFreedMemoryMode { kClearFreedMemory, kDontClearFreedMemory }

    pub enum SkipRoot {
        kExternalStringTable,
        kGlobalHandles,
        kTracedHandles,
        kOldGeneration,
        kStack,
        kMainThreadHandles,
        kUnserializable,
        kWeak,
        kConservativeStack,
        kReadOnlyBuiltins,
    }

    pub enum EmbedderStackStateOrigin {
      kImplicitThroughTask,
      kExplicitInvocation,
    }

    pub struct StrongRootsEntry {
        label: &'static str,
        start: *mut HeapObject, //Placeholder
        end: *mut HeapObject,   //Placeholder
        prev: *mut StrongRootsEntry,
        next: *mut StrongRootsEntry,
    }

    impl StrongRootsEntry {
        fn new(label: &'static str) -> Self {
            StrongRootsEntry {
                label,
                start: std::ptr::null_mut(),
                end: std::ptr::null_mut(),
                prev: std::ptr::null_mut(),
                next: std::ptr::null_mut(),
            }
        }
    }

    pub type UnorderedHeapObjectMap<T> = HashMap<Tagged<HeapObject>, T>;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum GCFlag {
        kNoFlags = 0,
        kReduceMemoryFootprint = 1 << 0,
        kForced = 1 << 1,
        kLastResort = 1 << 2,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct GCFlags(u8);

    impl GCFlags {
        pub const kNoFlags: GCFlags = GCFlags(0);

        pub fn contains(&self, flag: GCFlag) -> bool {
            (self.0 & (flag as u8)) != 0
        }
    }

    impl From<GCFlag> for GCFlags {
        fn from(flag: GCFlag) -> Self {
            GCFlags(flag as u8)
        }
    }

    impl std::ops::BitOr for GCFlags {
        type Output = Self;

        fn bitor(self, other: Self) -> Self {
            GCFlags(self.0 | other.0)
        }
    }

    // Placeholder for Allocationspace and GarbageCollector enums
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum AllocationSpace {}
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum GarbageCollector {
        SCAVENGER,
        MARK_COMPACTOR,
        MINOR_MARK_SWEEPER
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum GarbageCollectionReason {}

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct GCCallbackFlags {}

    impl GCCallbackFlags {
        pub const kNoGCCallbackFlags: Self = GCCallbackFlags{};
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum AllocationAlignment {}

    pub enum AllocationType {
      kYoung,
      kOld
    }

    pub enum AllocationOrigin {
      kRuntime
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum InstanceType {}

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum ElementsKind {}

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum WriteBarrierMode {}

    pub enum DevToolsTraceEventScope {}

    pub struct WritableFreeSpace {}

    pub struct Code {}

    pub struct GcSafeCode {}

    pub struct InstructionStream {}

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Map {}

    pub struct ArrayList {}
    pub struct ByteArray {}
    pub struct FixedArray {}
    pub struct FixedDoubleArray {}
    pub struct TransitionArray {}
    pub struct WeakFixedArray {}

    pub enum TaskPriority {}

    pub struct VirtualMemory {}

    pub struct DirectHandle<T> {
      _phantom: PhantomData<T>
    }
    impl<T> DirectHandle<T> {
      pub fn new() -> Self {
        DirectHandle{_phantom: PhantomData}
      }
    }

    pub struct MaybeDirectHandle<T> {
      _phantom: PhantomData<T>
    }
    impl<T> MaybeDirectHandle<T> {
      pub fn new() -> Self {
        MaybeDirectHandle{_phantom: PhantomData}
      }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum StackState {
      kMayContainHeapPointers
    }
    pub struct EmbedderRootsHandler {}

    pub enum ExternalBackingStoreType {}

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Address(usize);

    impl Address {
        pub fn from_usize(address: usize) -> Self {
            Address(address)
        }
        pub fn as_usize(&self) -> usize {
            self.0
        }
    }
    pub struct ResourceConstraints {}

    pub struct WasmCanonicalRttsAndJSToWasmWrappers {}

    pub struct Heap {
        relocation_mutex_: Mutex<()>,
        isolate_: *mut Isolate,
        heap_allocator_: *mut MemoryAllocator,
        code_range_size_: usize,
        max_semi_space_size_: usize,
        min_semi_space_size_: usize,
        initial_semispace_size_: usize,
        min_old_generation_size_: usize,
        max_old_generation_size_: AtomicU64,
        min_global_memory_size_: usize,
        max_global_memory_size_: usize,
        initial_max_old_generation_size_: usize,
        initial_max_old_generation_size_threshold_: usize,
        initial_old_generation_size_: usize,
        using_initial_limit_: AtomicBool,
        initial_size_overwritten_: bool,
        maximum_committed_: usize,
        old_generation_capacity_after_bootstrap_: usize,
        backing_store_bytes_: AtomicU64,
        survived_since_last_expansion_: usize,
        always_allocate_scope_count_: AtomicU64,
        memory_pressure_level_: AtomicU8,
        near_heap_limit_callbacks_: Vec<(*mut v8_callbacks::NearHeapLimitCallback, *mut std::ffi::c_void)>,
        contexts_disposed_: i32,
        new_space_: *mut NewSpace,
        old_space_: *mut OldSpace,
        code_space_: *mut CodeSpace,
        shared_space_: *mut SharedSpace,
        lo_space_: *mut OldLargeObjectSpace,
        code_lo_space_: *mut CodeLargeObjectSpace,
        new_lo_space_: *mut NewLargeObjectSpace,
        shared_lo_space_: *mut SharedLargeObjectSpace,
        read_only_space_: *mut ReadOnlySpace,
        trusted_space_: *mut TrustedSpace,
        shared_trusted_space_: *mut SharedTrustedSpace,
        trusted_lo_space_: *mut TrustedLargeObjectSpace,
        shared_trusted_lo_space_: *mut SharedTrustedLargeObjectSpace,
        shared_allocation_space_: *mut PagedSpace,
        shared_lo_allocation_space_: *mut OldLargeObjectSpace,
        shared_trusted_allocation_space_: *mut SharedTrustedSpace,
        shared_trusted_lo_allocation_space_: *mut SharedTrustedLargeObjectSpace,
        space_: [*mut Space; 18], // LAST_SPACE + 1 = 17, but Space type may require 18?
        main_thread_local_heap_: *mut LocalHeap,
        gc_state_: AtomicU8,
        stress_marking_percentage_: i32,
        stress_scavenge_observer_: *mut StressScavengeObserver,
        max_marking_limit_reached_: AtomicU64,
        ms_count_: u32,
        gc_count_: u32,
        consecutive_ineffective_mark_compacts_: i32,
        mmap_region_base_: usize,
        remembered_unmapped_pages_index_: i32,
        remembered_unmapped_pages_: [Address; 128], // kRememberedUnmappedPages = 128
        old_generation_allocation_limit_: AtomicU64,
        global_allocation_limit_: AtomicU64,
        native_contexts_list_: AtomicU64,
        allocation_sites_list_: Tagged<Object>,
        dirty_js_finalization_registries_list_: Tagged<Object>,
        dirty_js_finalization_registries_list_tail_: Tagged<Object>,
        gc_prologue_callbacks_: GCCallbacks,
        gc_epilogue_callbacks_: GCCallbacks,
        external_memory_callback_: *mut std::ffi::c_void, // Placeholder
        deferred_counters_: Vec<v8_isolate::UseCounterFeature>,
        promoted_objects_size_: usize,
        promotion_ratio_: f64,
        promotion_rate_: f64,
        new_space_surviving_object_size_: usize,
        previous_new_space_surviving_object_size_: usize,
        new_space_surviving_rate_: f64,
        nodes_died_in_new_space_: i32,
        nodes_copied_in_new_space_: i32,
        nodes_promoted_: i32,
        total_gc_time_ms_: Duration,
        last_gc_time_: f64,
        tracer_: *mut GCTracer,
        sweeper_: *mut Sweeper,
        mark_compact_collector_: *mut MarkCompactCollector,
        minor_mark_sweep_collector_: *mut MinorMarkSweepCollector,
        scavenger_collector_: *mut ScavengerCollector,
        array_buffer_sweeper_: *mut ArrayBufferSweeper,
        memory_allocator_: *mut MemoryAllocator,
        incremental_marking_: *mut IncrementalMarking,
        concurrent_marking_: *mut ConcurrentMarking,
        memory_measurement_: *mut MemoryMeasurement,
        memory_reducer_: *mut MemoryReducer,
        live_object_stats_: *mut ObjectStats,
        dead_object_stats_: *mut ObjectStats,
        minor_gc_job_: *mut MinorGCJob,
        stress_concurrent_allocation_observer_: *mut AllocationObserver,
        allocation_tracker_for_debugging_: *mut AllocationTrackerForDebugging,
        ephemeron_remembered_set_: *mut EphemeronRememberedSet,
        heap_profiler_: *mut HeapProfiler,
        task_runner_: *mut std::ffi::c_void, //Placeholder
        code_range_: *mut CodeRange,
        owning_cpp_heap_: *mut CppHeap,
        cpp_heap_: *mut CppHeap,
        embedder_roots_handler_: *mut EmbedderRootsHandler,
        embedder_stack_state_: StackState,
        embedder_stack_state_origin_: Option<EmbedderStackStateOrigin>,
        strong_roots_head_: *mut StrongRootsEntry,
        strong_roots_mutex_: Mutex<()>,
        heap_expansion_mutex_: Mutex<()>,
        need_to_remove_stress_concurrent_allocation_observer_: bool,
        new_space_allocation_counter_: usize,
        old_generation_allocation_counter_at_last_gc_: usize,
        old_generation_size_at_last_gc_: usize,
        old_generation_wasted_at_last_gc_: usize,
        embedder_size_at_last_gc_: usize,
        trace_ring_buffer_: [u8; 512], // kTraceRingBufferSize = 512
        ring_buffer_full_: bool,
        ring_buffer_end_: usize,
        configured_: bool,
        current_gc_flags_: GCFlags,
        current_gc_callback_flags_: GCCallbackFlags,
        safepoint_: *mut IsolateSafepoint,
        is_current_gc_forced_: bool,
        is_current_gc_for_heap_profiler_: bool,
        current_or_last_garbage_collector_: GarbageCollector,
        external_string_table_: ExternalStringTable,
        allocation_type_for_in_place_internalizable_strings_: AllocationType,
        relocation_mutex_: Mutex<()>,
        collection_barrier_: *mut CollectionBarrier,
        ignore_local_gc_requests_depth_: i32,
        gc_callbacks_depth_: i32,
        deserialization_complete_: bool,
        max_regular_code_object_size_: i32,
        inline_allocation_enabled_: bool,
        pause_allocation_observers_depth_: i32,
        force_oom_: bool,
        force_gc_on_next_allocation_: bool,
        delay_sweeper_tasks_for_testing_: bool,
        allocation_trackers_: Vec<*mut HeapObjectAllocationTracker>,
        is_finalization_registry_cleanup_task_posted_: bool,
        marking_state_: MarkingState,
        non_atomic_marking_state_: NonAtomicMarkingState,
        pretenuring_handler_: PretenuringHandler,
        resize_new_space_mode_: ResizeNewSpaceMode,
        mb_: *mut MemoryBalancer,
        load_start_time_ms_: AtomicU64,
        update_allocation_limits_after_loading_: bool,
        is_full_gc_during_loading_: bool,
    }

    impl Heap {
        const kPointerMultiplier: usize = 2;
        const kHeapLimitMultiplier: usize = 2;
        const kMaxInitialOldGenerationSize: usize = 256 * 1024 * 1024 * Heap::kHeapLimitMultiplier;
        const kPhysicalMemoryToOldGenerationRatio: usize = 4;
        const kOldGenerationLowMemory: usize = 128 * 1024 * 1024 * Heap::kHeapLimitMultiplier;
        const kNewLargeObjectSpaceToSemiSpaceRatio: usize = 1;
        const kTraceRingBufferSize: usize = 512;
        const kStacktraceBufferSize: usize = 512;
        const kMinObjectSizeInTaggedWords: usize = 2;
        const kInitialEvalCacheSize: usize = 64;
        const kInitialNumberStringCacheSize: usize = 256;
        const kRememberedUnmappedPages: usize = 128;
        const kYoungSurvivalRateHighThreshold: i32 = 90;
        const kYoungSurvivalRateAllowedDeviation: i32 = 15;
        const kOldSurvivalRateLowThreshold: i32 = 10;
        const kMaxMarkCompactsInIdleRound: i32 = 7;
        const kRetainMapEntrySize: i32 = 2;
        const kLoadTimeNotLoading: f64 = -1.0;

        pub fn new() -> Self {
            Heap {
                relocation_mutex_: Mutex::new(()),
                isolate_: std::ptr::null_mut(),
                heap_allocator_: std::ptr::null_mut(),
                code_range_size_: 0,
                max_semi_space_size_: 0,
                min_semi_space_size_: 0,
                initial_semispace_size_: 0,
                min_old_generation_size_: 0,
                max_old_generation_size_: AtomicU64::new(0),
                min_global_memory_size_: 0,
                max_global_memory_size_: 0,
                initial_max_old_generation_size_: 0,
                initial_max_old_generation_size_threshold_: 0,
                initial_old_generation_size_: 0,
                using_initial_limit_: AtomicBool::new(true),
                initial_size_overwritten_: false,
                maximum_committed_: 0,
                old_generation_capacity_after_bootstrap_: 0,
                backing_store_bytes_: AtomicU64::new(0),
                survived_since_last_expansion_: 0,
                always_allocate_scope_count_: AtomicU64::new(0),
                memory_pressure_level_: AtomicU8::new(0),
                near_heap_limit_callbacks_: Vec::new(),
                contexts_disposed_: 0,
                new_space_: std::ptr::null_mut(),
                old_space_: std::ptr::null_mut(),
                code_space_: std::ptr::null_mut(),
                shared_space_: std::ptr::null_mut(),
                lo_space_: std::ptr::null_mut(),
                code_lo_space_: std::ptr::null_mut(),
                new_lo_space_: std::ptr::null_mut(),
                shared_lo_space_: std::ptr::null_mut(),
                read_only_space_: std::ptr::null_mut(),
                trusted_space_: std::ptr::null_mut(),
                shared_trusted_space_: std::ptr::null_mut(),
                trusted_lo_space_: std::ptr::null_mut(),
                shared_trusted_lo_space_: std::ptr::null_mut(),
                shared_allocation_space_: std::ptr::null_mut(),
                shared_lo_allocation_space_: std::ptr::null_mut(),
                shared_trusted_allocation_space_: std::ptr::null_mut(),
                shared_trusted_lo_allocation_space_: std::ptr::null_mut(),
                space_: [std::ptr::null_mut(); 18],
                main_thread_local_heap_: std::ptr::null_mut(),
                gc_state_: AtomicU8::new(0),
                stress_marking_percentage_: 0,
                stress_scavenge_observer_: std::ptr::null_mut(),
                max_marking_limit_reached_: AtomicU64::new(0),
                ms_count_: 0,
                gc_count_: 0,
                consecutive_ineffective_mark_compacts_: 0,
                mmap_region_base_: 0,
                remembered_unmapped_pages_index_: 0,
                remembered_unmapped_pages_: [Address::from_usize(0); 128],
                old_generation_allocation_limit_: AtomicU64::new(0),
                global_allocation_limit_: AtomicU64::new(0),
                native_contexts_list_: AtomicU64::new(0),
                allocation_sites_list_: Tagged { ptr: std::ptr::null_mut(), _phantom: PhantomData },
                dirty_js_finalization_registries_list_: Tagged { ptr: std::ptr::null_mut(), _phantom: PhantomData },
                dirty_js_finalization_registries_list_tail_: Tagged { ptr: std::ptr::null_mut(), _phantom: PhantomData },
                gc_prologue_callbacks_: GCCallbacks::new(),
                gc_epilogue_callbacks_: GCCallbacks::new(),
                external_memory_callback_: std::ptr::null_mut(),
                deferred_counters_: Vec::new(),
                promoted_objects_size_: 0,
                promotion_ratio_: 0.0,
                promotion_rate_: 0.0,
                new_space_surviving_object_size_: 0,
                previous_new_space_surviving_object_size_: 0,
                new_space_surviving_rate_: 0.0,
                nodes_died_in_new_space_: 0,
                nodes_copied_in_new_space_: 0,
                nodes_promoted_: 0,
                total_gc_time_ms_: Duration::from_millis(0),
                last_gc_time_: 0.0,
                tracer_: std::ptr::null_mut(),
                sweeper_: std::ptr::null_mut(),
                mark_compact_