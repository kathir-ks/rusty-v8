pub mod v8_metrics {
    use std::time::Duration;

    #[derive(Default, Debug, Clone, Copy)]
    pub struct GarbageCollectionPhases {
        pub total_wall_clock_duration_in_us: i64,
        pub compact_wall_clock_duration_in_us: i64,
        pub mark_wall_clock_duration_in_us: i64,
        pub sweep_wall_clock_duration_in_us: i64,
        pub weak_wall_clock_duration_in_us: i64,
    }

    impl GarbageCollectionPhases {
        pub fn new() -> Self {
            GarbageCollectionPhases {
                total_wall_clock_duration_in_us: -1,
                compact_wall_clock_duration_in_us: -1,
                mark_wall_clock_duration_in_us: -1,
                sweep_wall_clock_duration_in_us: -1,
                weak_wall_clock_duration_in_us: -1,
            }
        }
    }

    #[derive(Default, Debug, Clone, Copy)]
    pub struct GarbageCollectionSizes {
        pub bytes_before: i64,
        pub bytes_after: i64,
        pub bytes_freed: i64,
    }

    impl GarbageCollectionSizes {
        pub fn new() -> Self {
            GarbageCollectionSizes {
                bytes_before: -1,
                bytes_after: -1,
                bytes_freed: -1,
            }
        }
    }

    #[derive(Default, Debug, Clone)]
    pub struct GarbageCollectionFullCycle {
        pub reason: i32,
        // The priority of the isolate during the GC cycle. A None value denotes a
        // mixed priority cycle, meaning the Isolate's priority was changed while the
        // cycle was in progress.
        pub priority: Option<i32>, // Using i32 as a placeholder for v8::Isolate::Priority
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

    impl GarbageCollectionFullCycle {
        pub fn new() -> Self {
            GarbageCollectionFullCycle {
                reason: -1,
                priority: None,
                total: GarbageCollectionPhases::new(),
                total_cpp: GarbageCollectionPhases::new(),
                main_thread: GarbageCollectionPhases::new(),
                main_thread_cpp: GarbageCollectionPhases::new(),
                main_thread_atomic: GarbageCollectionPhases::new(),
                main_thread_atomic_cpp: GarbageCollectionPhases::new(),
                main_thread_incremental: GarbageCollectionPhases::new(),
                main_thread_incremental_cpp: GarbageCollectionPhases::new(),
                objects: GarbageCollectionSizes::new(),
                objects_cpp: GarbageCollectionSizes::new(),
                memory: GarbageCollectionSizes::new(),
                memory_cpp: GarbageCollectionSizes::new(),
                collection_rate_in_percent: -1.0,
                collection_rate_cpp_in_percent: -1.0,
                efficiency_in_bytes_per_us: -1.0,
                efficiency_cpp_in_bytes_per_us: -1.0,
                main_thread_efficiency_in_bytes_per_us: -1.0,
                main_thread_efficiency_cpp_in_bytes_per_us: -1.0,
                collection_weight_in_percent: -1.0,
                collection_weight_cpp_in_percent: -1.0,
                main_thread_collection_weight_in_percent: -1.0,
                main_thread_collection_weight_cpp_in_percent: -1.0,
                incremental_marking_start_stop_wall_clock_duration_in_us: -1,
            }
        }
    }

    #[derive(Default, Debug, Clone, Copy)]
    pub struct GarbageCollectionFullMainThreadIncrementalMark {
        pub wall_clock_duration_in_us: i64,
        pub cpp_wall_clock_duration_in_us: i64,
    }

    impl GarbageCollectionFullMainThreadIncrementalMark {
        pub fn new() -> Self {
            GarbageCollectionFullMainThreadIncrementalMark {
                wall_clock_duration_in_us: -1,
                cpp_wall_clock_duration_in_us: -1,
            }
        }
    }

    #[derive(Default, Debug, Clone, Copy)]
    pub struct GarbageCollectionFullMainThreadIncrementalSweep {
        pub wall_clock_duration_in_us: i64,
        pub cpp_wall_clock_duration_in_us: i64,
    }

    impl GarbageCollectionFullMainThreadIncrementalSweep {
        pub fn new() -> Self {
            GarbageCollectionFullMainThreadIncrementalSweep {
                wall_clock_duration_in_us: -1,
                cpp_wall_clock_duration_in_us: -1,
            }
        }
    }

    #[derive(Default, Debug, Clone)]
    pub struct GarbageCollectionBatchedEvents<EventType> {
        pub events: Vec<EventType>,
    }

    pub type GarbageCollectionFullMainThreadBatchedIncrementalMark =
        GarbageCollectionBatchedEvents<GarbageCollectionFullMainThreadIncrementalMark>;
    pub type GarbageCollectionFullMainThreadBatchedIncrementalSweep =
        GarbageCollectionBatchedEvents<GarbageCollectionFullMainThreadIncrementalSweep>;

    #[derive(Default, Debug, Clone)]
    pub struct GarbageCollectionYoungCycle {
        pub reason: i32,
        // The priority of the isolate during the GC cycle. A None value denotes a
        // mixed priority cycle, meaning the Isolate's priority was changed while the
        // cycle was in progress.
        pub priority: Option<i32>, // Using i32 as a placeholder for v8::Isolate::Priority
        pub total_wall_clock_duration_in_us: i64,
        pub main_thread_wall_clock_duration_in_us: i64,
        pub collection_rate_in_percent: f64,
        pub efficiency_in_bytes_per_us: f64,
        pub main_thread_efficiency_in_bytes_per_us: f64,
        #[cfg(feature = "cppgc_young_generation")]
        pub total_cpp: GarbageCollectionPhases,
        #[cfg(feature = "cppgc_young_generation")]
        pub objects_cpp: GarbageCollectionSizes,
        #[cfg(feature = "cppgc_young_generation")]
        pub memory_cpp: GarbageCollectionSizes,
        #[cfg(feature = "cppgc_young_generation")]
        pub collection_rate_cpp_in_percent: f64,
        #[cfg(feature = "cppgc_young_generation")]
        pub efficiency_cpp_in_bytes_per_us: f64,
        #[cfg(feature = "cppgc_young_generation")]
        pub main_thread_efficiency_cpp_in_bytes_per_us: f64,
    }

    impl GarbageCollectionYoungCycle {
        pub fn new() -> Self {
            GarbageCollectionYoungCycle {
                reason: -1,
                priority: None,
                total_wall_clock_duration_in_us: -1,
                main_thread_wall_clock_duration_in_us: -1,
                collection_rate_in_percent: -1.0,
                efficiency_in_bytes_per_us: -1.0,
                main_thread_efficiency_in_bytes_per_us: -1.0,
                #[cfg(feature = "cppgc_young_generation")]
                total_cpp: GarbageCollectionPhases::new(),
                #[cfg(feature = "cppgc_young_generation")]
                objects_cpp: GarbageCollectionSizes::new(),
                #[cfg(feature = "cppgc_young_generation")]
                memory_cpp: GarbageCollectionSizes::new(),
                #[cfg(feature = "cppgc_young_generation")]
                collection_rate_cpp_in_percent: -1.0,
                #[cfg(feature = "cppgc_young_generation")]
                efficiency_cpp_in_bytes_per_us: -1.0,
                #[cfg(feature = "cppgc_young_generation")]
                main_thread_efficiency_cpp_in_bytes_per_us: -1.0,
            }
        }
    }

    #[derive(Default, Debug, Clone, Copy)]
    pub struct WasmModuleDecoded {
        pub async_flag: bool,
        pub streamed: bool,
        pub success: bool,
        pub module_size_in_bytes: usize,
        pub function_count: usize,
        pub wall_clock_duration_in_us: i64,
    }

    impl WasmModuleDecoded {
        pub fn new(
            async_flag: bool,
            streamed: bool,
            success: bool,
            module_size_in_bytes: usize,
            function_count: usize,
            wall_clock_duration_in_us: i64,
        ) -> Self {
            WasmModuleDecoded {
                async_flag,
                streamed,
                success,
                module_size_in_bytes,
                function_count,
                wall_clock_duration_in_us,
            }
        }
    }

    #[derive(Default, Debug, Clone, Copy)]
    pub struct WasmModuleCompiled {
        pub async_flag: bool,
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
            async_flag: bool,
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
                async_flag,
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

    #[derive(Default, Debug, Clone, Copy)]
    pub struct WasmModuleInstantiated {
        pub async_flag: bool,
        pub success: bool,
        pub imported_function_count: usize,
        pub wall_clock_duration_in_us: i64,
    }

    impl WasmModuleInstantiated {
        pub fn new(
            async_flag: bool,
            success: bool,
            imported_function_count: usize,
            wall_clock_duration_in_us: i64,
        ) -> Self {
            WasmModuleInstantiated {
                async_flag,
                success,
                imported_function_count,
                wall_clock_duration_in_us,
            }
        }
    }

    #[derive(Default, Debug, Clone, Copy)]
    pub struct WasmModulesPerIsolate {
        pub count: usize,
    }

    impl WasmModulesPerIsolate {
        pub fn new() -> Self {
            WasmModulesPerIsolate { count: 0 }
        }
    }

    /// This class serves as a base class for recording event-based metrics.
    pub trait Recorder {
        /// Adds a main thread event.
        fn add_main_thread_event_garbage_collection_full_cycle(&mut self, event: &GarbageCollectionFullCycle, context_id: ContextId) {}
        /// Adds a main thread event.
        fn add_main_thread_event_garbage_collection_full_main_thread_incremental_mark(&mut self, event: &GarbageCollectionFullMainThreadIncrementalMark, context_id: ContextId) {}
        /// Adds a main thread event.
        fn add_main_thread_event_garbage_collection_full_main_thread_batched_incremental_mark(&mut self, event: &GarbageCollectionFullMainThreadBatchedIncrementalMark, context_id: ContextId) {}
        /// Adds a main thread event.
        fn add_main_thread_event_garbage_collection_full_main_thread_incremental_sweep(&mut self, event: &GarbageCollectionFullMainThreadIncrementalSweep, context_id: ContextId) {}
        /// Adds a main thread event.
        fn add_main_thread_event_garbage_collection_full_main_thread_batched_incremental_sweep(&mut self, event: &GarbageCollectionFullMainThreadBatchedIncrementalSweep, context_id: ContextId) {}
        /// Adds a main thread event.
        fn add_main_thread_event_garbage_collection_young_cycle(&mut self, event: &GarbageCollectionYoungCycle, context_id: ContextId) {}
        /// Adds a main thread event.
        fn add_main_thread_event_wasm_module_decoded(&mut self, event: &WasmModuleDecoded, context_id: ContextId) {}
        /// Adds a main thread event.
        fn add_main_thread_event_wasm_module_compiled(&mut self, event: &WasmModuleCompiled, context_id: ContextId) {}
        /// Adds a main thread event.
        fn add_main_thread_event_wasm_module_instantiated(&mut self, event: &WasmModuleInstantiated, context_id: ContextId) {}

        /// Adds a thread-safe event.
        fn add_thread_safe_event_wasm_modules_per_isolate(&mut self, event: &WasmModulesPerIsolate) {}

        /// Notifies isolate disposal.
        fn notify_isolate_disposal(&mut self) {}

        // These functions are not directly translatable without `v8::Isolate` and `v8::Context`
        // They are left as unimplemented stubs.
        // Return the context with the given id or an empty handle if the context
        // was already garbage collected.
        //static MaybeLocal<Context> GetContext(Isolate* isolate, ContextId id);
        // Return the unique id corresponding to the given context.
        //static ContextId GetContextId(Local<Context> context);
    }

    /// A unique identifier for a context in this Isolate.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct ContextId {
        id_: usize,
    }

    impl ContextId {
        pub fn new() -> Self {
            ContextId { id_: Self::k_empty_id() }
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

        // Private constructor only used within the v8 module
        // pub(crate) fn from_usize(id: usize) -> Self {
        //     ContextId { id_: id }
        // }

        // Method to get the internal id_
        pub fn get_id(&self) -> usize {
            self.id_
        }
    }

    impl Default for ContextId {
        fn default() -> Self {
            Self::new()
        }
    }

    /// Experimental API intended for the LongTasks UKM (crbug.com/1173527).
    #[derive(Default, Debug, Clone, Copy)]
    pub struct LongTaskStats {
        pub gc_full_atomic_wall_clock_duration_us: i64,
        pub gc_full_incremental_wall_clock_duration_us: i64,
        pub gc_young_wall_clock_duration_us: i64,
        // Only collected with --slow-histograms
        pub v8_execute_us: i64,
    }

    impl LongTaskStats {
        /// Resets durations of V8 work for the new task.
        // V8_INLINE static void Reset(Isolate* isolate) {
        //   v8::internal::Internals::IncrementLongTasksStatsCounter(isolate);
        // }
        // Requires v8::internal::Internals and Isolate*, so this is a placeholder
        pub fn reset() {}

        /// Returns durations of V8 work that happened since the last Reset().
        //static LongTaskStats Get(Isolate* isolate);
        // Requires Isolate*, so this is a placeholder
        pub fn get() -> Self {
            LongTaskStats {
                gc_full_atomic_wall_clock_duration_us: 0,
                gc_full_incremental_wall_clock_duration_us: 0,
                gc_young_wall_clock_duration_us: 0,
                v8_execute_us: 0,
            }
        }
    }
}