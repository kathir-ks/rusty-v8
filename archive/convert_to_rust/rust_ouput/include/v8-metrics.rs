// Converted from V8 C++ source files:
// Header: v8-metrics.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod metrics {
    use std::time::Duration;
    use std::option::Option;

    #[derive(Default, Clone, Copy)]
    pub struct GarbageCollectionPhases {
        pub total_wall_clock_duration_in_us: i64,
        pub compact_wall_clock_duration_in_us: i64,
        pub mark_wall_clock_duration_in_us: i64,
        pub sweep_wall_clock_duration_in_us: i64,
        pub weak_wall_clock_duration_in_us: i64,
    }

    #[derive(Default, Clone, Copy)]
    pub struct GarbageCollectionSizes {
        pub bytes_before: i64,
        pub bytes_after: i64,
        pub bytes_freed: i64,
    }

    #[derive(Default, Clone)]
    pub struct GarbageCollectionFullCycle {
        pub reason: i32,
        // The priority of the isolate during the GC cycle. A nullopt value denotes a
        // mixed priority cycle, meaning the Isolate's priority was changed while the
        // cycle was in progress.
        pub priority: Option<Priority>,
        pub total: GarbageCollectionPhases,
        pub total_cpp: GarbageCollectionPhases,
        pub main_thread: GarbageCollectionPhases,
        pub main_thread_cpp: GarbageCollectionPhases,
        pub main_thread_atomic: GarbageCollectionPhases,
        pub main_thread_atomic_cpp: GarbageCollectionPhases,
        pub main_thread_incremental: GarbageCollectionPhases,
        pub main_thread_incremental_cpp: GarbageCollectionPhases,
        pub objects: GarbageCollectionSizes,
        pub objects_cpp: GarbageCollectionSizes,
        pub memory: GarbageCollectionSizes,
        pub memory_cpp: GarbageCollectionSizes,
        pub collection_rate_in_percent: f64,
        pub collection_rate_cpp_in_percent: f64,
        pub efficiency_in_bytes_per_us: f64,
        pub efficiency_cpp_in_bytes_per_us: f64,
        pub main_thread_efficiency_in_bytes_per_us: f64,
        pub main_thread_efficiency_cpp_in_bytes_per_us: f64,
        pub collection_weight_in_percent: f64,
        pub collection_weight_cpp_in_percent: f64,
        pub main_thread_collection_weight_in_percent: f64,
        pub main_thread_collection_weight_cpp_in_percent: f64,
        pub incremental_marking_start_stop_wall_clock_duration_in_us: i64,
    }

    #[derive(Default, Clone, Copy)]
    pub struct GarbageCollectionFullMainThreadIncrementalMark {
        pub wall_clock_duration_in_us: i64,
        pub cpp_wall_clock_duration_in_us: i64,
    }

    #[derive(Default, Clone, Copy)]
    pub struct GarbageCollectionFullMainThreadIncrementalSweep {
        pub wall_clock_duration_in_us: i64,
        pub cpp_wall_clock_duration_in_us: i64,
    }

    #[derive(Default, Clone)]
    pub struct GarbageCollectionBatchedEvents<EventType> {
        pub events: Vec<EventType>,
    }

    pub type GarbageCollectionFullMainThreadBatchedIncrementalMark =
        GarbageCollectionBatchedEvents<GarbageCollectionFullMainThreadIncrementalMark>;
    pub type GarbageCollectionFullMainThreadBatchedIncrementalSweep =
        GarbageCollectionBatchedEvents<GarbageCollectionFullMainThreadIncrementalSweep>;

    #[derive(Default, Clone)]
    pub struct GarbageCollectionYoungCycle {
        pub reason: i32,
        // The priority of the isolate during the GC cycle. A nullopt value denotes a
        // mixed priority cycle, meaning the Isolate's priority was changed while the
        // cycle was in progress.
        pub priority: Option<Priority>,
        pub total_wall_clock_duration_in_us: i64,
        pub main_thread_wall_clock_duration_in_us: i64,
        pub collection_rate_in_percent: f64,
        pub efficiency_in_bytes_per_us: f64,
        pub main_thread_efficiency_in_bytes_per_us: f64,
        #[cfg(feature = "CPPGC_YOUNG_GENERATION")]
        pub total_cpp: GarbageCollectionPhases,
        #[cfg(feature = "CPPGC_YOUNG_GENERATION")]
        pub objects_cpp: GarbageCollectionSizes,
        #[cfg(feature = "CPPGC_YOUNG_GENERATION")]
        pub memory_cpp: GarbageCollectionSizes,
        #[cfg(feature = "CPPGC_YOUNG_GENERATION")]
        pub collection_rate_cpp_in_percent: f64,
        #[cfg(feature = "CPPGC_YOUNG_GENERATION")]
        pub efficiency_cpp_in_bytes_per_us: f64,
        #[cfg(feature = "CPPGC_YOUNG_GENERATION")]
        pub main_thread_efficiency_cpp_in_bytes_per_us: f64,
    }

    #[derive(Default, Clone, Copy)]
    pub struct WasmModuleDecoded {
        pub async_: bool,
        pub streamed: bool,
        pub success: bool,
        pub module_size_in_bytes: usize,
        pub function_count: usize,
        pub wall_clock_duration_in_us: i64,
    }

    impl WasmModuleDecoded {
        pub fn new(
            async_: bool,
            streamed: bool,
            success: bool,
            module_size_in_bytes: usize,
            function_count: usize,
            wall_clock_duration_in_us: i64,
        ) -> Self {
            WasmModuleDecoded {
                async_,
                streamed,
                success,
                module_size_in_bytes,
                function_count,
                wall_clock_duration_in_us,
            }
        }
    }

    #[derive(Default, Clone, Copy)]
    pub struct WasmModuleCompiled {
        pub async_: bool,
        pub streamed: bool,
        pub cached: bool,
        pub deserialized: bool,
        pub lazy: bool,
        pub success: bool,
        pub code_size_in_bytes: usize,
        pub liftoff_bailout_count: usize,
        pub wall_clock_duration_in_us: i64,
    }

    impl WasmModuleCompiled {
        pub fn new(
            async_: bool,
            streamed: bool,
            cached: bool,
            deserialized: bool,
            lazy: bool,
            success: bool,
            code_size_in_bytes: usize,
            liftoff_bailout_count: usize,
            wall_clock_duration_in_us: i64,
        ) -> Self {
            WasmModuleCompiled {
                async_,
                streamed,
                cached,
                deserialized,
                lazy,
                success,
                code_size_in_bytes,
                liftoff_bailout_count,
                wall_clock_duration_in_us,
            }
        }
    }

    #[derive(Default, Clone, Copy)]
    pub struct WasmModuleInstantiated {
        pub async_: bool,
        pub success: bool,
        pub imported_function_count: usize,
        pub wall_clock_duration_in_us: i64,
    }

    #[derive(Default, Clone, Copy)]
    pub struct WasmModulesPerIsolate {
        pub count: usize,
    }

    pub trait RecorderInterface {
        fn add_main_thread_event_garbage_collection_full_cycle(
            &mut self,
            event: &GarbageCollectionFullCycle,
            context_id: ContextId,
        );
        fn add_main_thread_event_garbage_collection_full_main_thread_incremental_mark(
            &mut self,
            event: &GarbageCollectionFullMainThreadIncrementalMark,
            context_id: ContextId,
        );
        fn add_main_thread_event_garbage_collection_full_main_thread_batched_incremental_mark(
            &mut self,
            event: &GarbageCollectionFullMainThreadBatchedIncrementalMark,
            context_id: ContextId,
        );
        fn add_main_thread_event_garbage_collection_full_main_thread_incremental_sweep(
            &mut self,
            event: &GarbageCollectionFullMainThreadIncrementalSweep,
            context_id: ContextId,
        );
        fn add_main_thread_event_garbage_collection_full_main_thread_batched_incremental_sweep(
            &mut self,
            event: &GarbageCollectionFullMainThreadBatchedIncrementalSweep,
            context_id: ContextId,
        );
        fn add_main_thread_event_garbage_collection_young_cycle(
            &mut self,
            event: &GarbageCollectionYoungCycle,
            context_id: ContextId,
        );
        fn add_main_thread_event_wasm_module_decoded(
            &mut self,
            event: &WasmModuleDecoded,
            context_id: ContextId,
        );
        fn add_main_thread_event_wasm_module_compiled(
            &mut self,
            event: &WasmModuleCompiled,
            context_id: ContextId,
        );
        fn add_main_thread_event_wasm_module_instantiated(
            &mut self,
            event: &WasmModuleInstantiated,
            context_id: ContextId,
        );
        fn add_thread_safe_event_wasm_modules_per_isolate(
            &mut self,
            event: &WasmModulesPerIsolate,
        );
        fn notify_isolate_disposal(&mut self);
        // Return the context with the given id or an empty handle if the context
        // was already garbage collected.
        fn get_context(isolate: *mut Isolate, id: ContextId) -> MaybeLocal<Context>;
        // Return the unique id corresponding to the given context.
        fn get_context_id(context: Local<Context>) -> ContextId;
    }

    /**
     * This class serves as a base class for recording event-based metrics in V8.
     * There a two kinds of metrics, those which are expected to be thread-safe and
     * whose implementation is required to fulfill this requirement and those whose
     * implementation does not have that requirement and only needs to be
     * executable on the main thread. If such an event is triggered from a
     * background thread, it will be delayed and executed by the foreground task
     * runner.
     *
     * The embedder is expected to call v8::Isolate::SetMetricsRecorder()
     * providing its implementation and have the virtual methods overwritten
     * for the events it cares about.
     */
    pub struct Recorder {}

    impl Recorder {
        pub fn new() -> Self {
            Recorder {}
        }
    }

    impl RecorderInterface for Recorder {
        fn add_main_thread_event_garbage_collection_full_cycle(
            &mut self,
            _event: &GarbageCollectionFullCycle,
            _context_id: ContextId,
        ) {
        }
        fn add_main_thread_event_garbage_collection_full_main_thread_incremental_mark(
            &mut self,
            _event: &GarbageCollectionFullMainThreadIncrementalMark,
            _context_id: ContextId,
        ) {
        }
        fn add_main_thread_event_garbage_collection_full_main_thread_batched_incremental_mark(
            &mut self,
            _event: &GarbageCollectionFullMainThreadBatchedIncrementalMark,
            _context_id: ContextId,
        ) {
        }
        fn add_main_thread_event_garbage_collection_full_main_thread_incremental_sweep(
            &mut self,
            _event: &GarbageCollectionFullMainThreadIncrementalSweep,
            _context_id: ContextId,
        ) {
        }
        fn add_main_thread_event_garbage_collection_full_main_thread_batched_incremental_sweep(
            &mut self,
            _event: &GarbageCollectionFullMainThreadBatchedIncrementalSweep,
            _context_id: ContextId,
        ) {
        }
        fn add_main_thread_event_garbage_collection_young_cycle(
            &mut self,
            _event: &GarbageCollectionYoungCycle,
            _context_id: ContextId,
        ) {
        }
        fn add_main_thread_event_wasm_module_decoded(
            &mut self,
            _event: &WasmModuleDecoded,
            _context_id: ContextId,
        ) {
        }
        fn add_main_thread_event_wasm_module_compiled(
            &mut self,
            _event: &WasmModuleCompiled,
            _context_id: ContextId,
        ) {
        }
        fn add_main_thread_event_wasm_module_instantiated(
            &mut self,
            _event: &WasmModuleInstantiated,
            _context_id: ContextId,
        ) {
        }
        fn add_thread_safe_event_wasm_modules_per_isolate(
            &mut self,
            _event: &WasmModulesPerIsolate,
        ) {
        }
        fn notify_isolate_disposal(&mut self) {}

        // Return the context with the given id or an empty handle if the context
        // was already garbage collected.
        fn get_context(_isolate: *mut Isolate, _id: ContextId) -> MaybeLocal<Context> {
            MaybeLocal::Empty()
        }
        // Return the unique id corresponding to the given context.
        fn get_context_id(_context: Local<Context>) -> ContextId {
            ContextId::Empty()
        }
    }

    // A unique identifier for a context in this Isolate.
    // It is guaranteed to not be reused throughout the lifetime of the Isolate.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ContextId {
        id_: usize,
    }

    impl ContextId {
        pub fn new(id: usize) -> Self {
            ContextId { id_: id }
        }

        pub fn is_empty(&self) -> bool {
            self.id_ == Self::k_empty_id()
        }
        pub fn empty() -> Self {
            ContextId { id_: Self::k_empty_id() }
        }

        const fn k_empty_id() -> usize {
            0
        }
    }

    /**
     * Experimental API intended for the LongTasks UKM (crbug.com/1173527).
     * The Reset() method should be called at the start of a potential
     * long task. The Get() method returns durations of V8 work that
     * happened during the task.
     *
     * This API is experimental and may be removed/changed in the future.
     */
    #[derive(Default, Clone, Copy)]
    pub struct LongTaskStats {
        pub gc_full_atomic_wall_clock_duration_us: i64,
        pub gc_full_incremental_wall_clock_duration_us: i64,
        pub gc_young_wall_clock_duration_us: i64,
        // Only collected with --slow-histograms
        pub v8_execute_us: i64,
    }

    impl LongTaskStats {
        /**
         * Resets durations of V8 work for the new task.
         */
        #[inline]
        pub fn reset(isolate: *mut Isolate) {
            unsafe {
                Internals::increment_long_tasks_stats_counter(isolate);
            }
        }

        /**
         * Returns durations of V8 work that happened since the last Reset().
         */
        pub fn get(isolate: *mut Isolate) -> Self {
            // In a real implementation, you would need to access the isolate's
            // internal state to retrieve the accumulated durations.
            // This is a placeholder.
            LongTaskStats::default()
        }
    }

    // Mock V8 types to allow compilation
    pub struct Isolate {}
    pub struct Context {}
    pub struct Local<'a, T> {
        _phantom: std::marker::PhantomData<&'a T>,
    }

    impl<'a, T> Local<'a, T> {
        pub fn empty() -> Self {
            Local {
                _phantom: std::marker::PhantomData,
            }
        }

        pub fn new() -> Self {
            Local {
                _phantom: std::marker::PhantomData,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub enum Error {
        GenericError,
    }

    pub struct MaybeLocal<'a, T> {
        _phantom: std::marker::PhantomData<&'a T>,
        is_empty: bool,
    }

    impl<'a, T> MaybeLocal<'a, T> {
        pub fn Empty() -> Self {
            MaybeLocal {
                _phantom: std::marker::PhantomData,
                is_empty: true,
            }
        }

        pub fn IsEmpty(&self) -> bool {
            self.is_empty
        }
    }

    #[allow(dead_code)]
    #[repr(C)]
    pub enum Priority {
        RealTime,
        High,
        Normal,
        Low,
        Background,
    }

    mod v8_internal {
        pub struct Internals {}
        impl Internals {
            pub unsafe fn increment_long_tasks_stats_counter(_isolate: *mut super::Isolate) {}
        }
    }

    use v8_internal::Internals;
}
