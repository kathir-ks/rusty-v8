// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file defines the counters and histograms used for V8's logging and
// performance monitoring.  It is roughly equivalent to the C++ header file
// `src/logging/counters-definitions.h`.
//
// Note: This is a *partial* translation.  Many of the details relating to
// V8 internals (v8-internal.h) and specific V8 data structures would
// need further elaboration based on the actual implementations.

//include "include/v8-internal.h"
// Placeholder for v8-internal.h - needs definition
// For example, if v8-internal.h defines constants, enums, and structs,
// we would define those here.
const KB: usize = 1024;
const GB: usize = 1024 * 1024 * 1024;
const MILLISECOND: &str = "millisecond";
const MICROSECOND: &str = "microsecond";

const kGarbageCollectionReasonMaxValue: usize = 10; //Example value, real value is from v8-internal.h
const kMaxExternalPointers: usize = 1024; // Example value
const kMaxCodePointers: usize = 1024; // Example value
const kMaxTrustedPointers: usize = 1024; // Example value
const kMaxCppHeapPointers: usize = 1024; // Example value
const kMaxJSDispatchEntries: usize = 1024; // Example value

pub mod logging {
    pub mod counters_definitions {
        // Macro for range histograms
        #[macro_export]
        macro_rules! histogram_range_list {
            ($hr:ident) => {
                $hr!(code_cache_reject_reason, "V8.CodeCacheRejectReason", 1, 9, 9);
                $hr!(errors_thrown_per_context, "V8.ErrorsThrownPerContext", 0, 200, 20);
                $hr!(
                    incremental_marking_reason,
                    "V8.GCIncrementalMarkingReason",
                    0,
                    super::kGarbageCollectionReasonMaxValue,
                    super::kGarbageCollectionReasonMaxValue + 1
                );
                $hr!(
                    incremental_marking_sum,
                    "V8.GCIncrementalMarkingSum",
                    0,
                    10000,
                    101
                );
                $hr!(
                    mark_compact_reason,
                    "V8.GCMarkCompactReason",
                    0,
                    super::kGarbageCollectionReasonMaxValue,
                    super::kGarbageCollectionReasonMaxValue + 1
                );
                $hr!(gc_finalize_clear, "V8.GCFinalizeMC.Clear", 0, 10000, 101);
                $hr!(
                    gc_finalize_epilogue,
                    "V8.GCFinalizeMC.Epilogue",
                    0,
                    10000,
                    101
                );
                $hr!(
                    gc_finalize_evacuate,
                    "V8.GCFinalizeMC.Evacuate",
                    0,
                    10000,
                    101
                );
                $hr!(gc_finalize_finish, "V8.GCFinalizeMC.Finish", 0, 10000, 101);
                $hr!(gc_finalize_mark, "V8.GCFinalizeMC.Mark", 0, 10000, 101);
                $hr!(
                    gc_finalize_prologue,
                    "V8.GCFinalizeMC.Prologue",
                    0,
                    10000,
                    101
                );
                $hr!(gc_finalize_sweep, "V8.GCFinalizeMC.Sweep", 0, 10000, 101);
                $hr!(
                    gc_scavenger_scavenge_main,
                    "V8.GCScavenger.ScavengeMain",
                    0,
                    10000,
                    101
                );
                $hr!(
                    gc_scavenger_scavenge_roots,
                    "V8.GCScavenger.ScavengeRoots",
                    0,
                    10000,
                    101
                );
                /* Asm/Wasm. */
                $hr!(
                    wasm_functions_per_asm_module,
                    "V8.WasmFunctionsPerModule.asm",
                    1,
                    1000000,
                    51
                );
                $hr!(
                    wasm_functions_per_wasm_module,
                    "V8.WasmFunctionsPerModule.wasm",
                    1,
                    1000000,
                    51
                );
                $hr!(
                    array_buffer_big_allocations,
                    "V8.ArrayBufferLargeAllocations",
                    0,
                    4096,
                    13
                );
                $hr!(
                    array_buffer_new_size_failures,
                    "V8.ArrayBufferNewSizeFailures",
                    0,
                    4096,
                    13
                );
                $hr!(
                    shared_array_allocations,
                    "V8.SharedArrayAllocationSizes",
                    0,
                    4096,
                    13
                );
                $hr!(
                    wasm_asm_huge_function_size_bytes,
                    "V8.WasmHugeFunctionSizeBytes.asm",
                    100 * super::KB,
                    super::GB,
                    51
                );
                $hr!(
                    wasm_wasm_huge_function_size_bytes,
                    "V8.WasmHugeFunctionSizeBytes.wasm",
                    100 * super::KB,
                    super::GB,
                    51
                );
                $hr!(
                    wasm_asm_module_size_bytes,
                    "V8.WasmModuleSizeBytes.asm",
                    1,
                    super::GB,
                    51
                );
                $hr!(
                    wasm_wasm_module_size_bytes,
                    "V8.WasmModuleSizeBytes.wasm",
                    1,
                    super::GB,
                    51
                );
                $hr!(
                    wasm_compile_huge_function_peak_memory_bytes,
                    "V8.WasmCompileHugeFunctionPeakMemoryBytes",
                    1,
                    super::GB,
                    51
                );
                $hr!(
                    asm_module_size_bytes,
                    "V8.AsmModuleSizeBytes",
                    1,
                    super::GB,
                    51
                );
                $hr!(
                    compile_script_cache_behaviour,
                    "V8.CompileScript.CacheBehaviour",
                    0,
                    21,
                    22
                );
                $hr!(
                    wasm_memory_allocation_result,
                    "V8.WasmMemoryAllocationResult",
                    0,
                    3,
                    4
                );
                /* Committed code size per module, collected on GC. */
                /* Older histogram, in MiB (0..1024MB). */
                $hr!(
                    wasm_module_code_size_mb,
                    "V8.WasmModuleCodeSizeMiB",
                    0,
                    1024,
                    64
                );
                /* Newer histogram, in KiB (0..100MB). */
                $hr!(
                    wasm_module_code_size_kb,
                    "V8.WasmModuleCodeSizeKiB",
                    0,
                    1024 * 100,
                    101
                );
                /* Metadata size per module, collected on GC. */
                $hr!(
                    wasm_module_metadata_size_kb,
                    "V8.WasmModuleMetadataSizeKiB",
                    0,
                    1024 * 100,
                    101
                );
                /* Metadata of the whole Wasm engine, collected on GC. */
                $hr!(
                    wasm_engine_metadata_size_kb,
                    "V8.WasmEngineMetadataSizeKiB",
                    0,
                    1024 * 100,
                    101
                );
                /* Percent of freed code size per module, collected on GC. */
                $hr!(
                    wasm_module_freed_code_size_percent,
                    "V8.WasmModuleCodeSizePercentFreed",
                    0,
                    100,
                    32
                );
                /* Number of code GCs triggered per native module, collected on code GC. */
                $hr!(
                    wasm_module_num_triggered_code_gcs,
                    "V8.WasmModuleNumberOfCodeGCsTriggered",
                    1,
                    128,
                    20
                );
                /* The amount of executable Liftoff code flushed on emergency GCs for */
                /* allocations and on memory pressure. */
                $hr!(
                    wasm_flushed_liftoff_code_size_bytes,
                    "V8.WasmFlushedLiftoffCodeSizeBytes",
                    0,
                    super::GB,
                    101
                );
                /* The size of flushed Liftoff meta data on emergency GCs for allocations */
                /* and on memory pressure. */
                $hr!(
                    wasm_flushed_liftoff_metadata_size_bytes,
                    "V8.WasmFlushedLiftoffMetadataSizeBytes",
                    0,
                    super::GB,
                    101
                );
                /* Number of code spaces reserved per wasm module. */
                $hr!(
                    wasm_module_num_code_spaces,
                    "V8.WasmModuleNumberOfCodeSpaces",
                    1,
                    128,
                    20
                );
                /* Number of deopts triggered in webassembly code. */
                $hr!(
                    wasm_deopts_executed,
                    "V8.WasmDeoptsExecutedCount",
                    0,
                    10000,
                    51
                );
                $hr!(
                    wasm_deopts_per_function,
                    "V8.WasmDeoptsPerFunction",
                    0,
                    500,
                    21
                );
                /* Number of live modules per isolate. */
                $hr!(
                    wasm_modules_per_isolate,
                    "V8.WasmModulesPerIsolate",
                    1,
                    1024,
                    30
                );
                /* Number of live modules per engine (i.e. whole process). */
                $hr!(
                    wasm_modules_per_engine,
                    "V8.WasmModulesPerEngine",
                    1,
                    1024,
                    30
                );
                /* Bailout reason if Liftoff failed, or {kSuccess} (per function). */
                $hr!(
                    liftoff_bailout_reasons,
                    "V8.LiftoffBailoutReasons",
                    0,
                    20,
                    21
                );
                /* Support for PKEYs/PKU by testing result of pkey_alloc(). */
                $hr!(
                    wasm_memory_protection_keys_support,
                    "V8.WasmMemoryProtectionKeysSupport",
                    0,
                    1,
                    2
                );
                /* Ticks observed in a single Turbofan compilation, in 1K. */
                $hr!(turbofan_ticks, "V8.TurboFan1KTicks", 0, 100000, 200);
                /* Backtracks observed in a single regexp interpreter execution. */
                /* The maximum of 100M backtracks takes roughly 2 seconds on my machine. */
                $hr!(regexp_backtracks, "V8.RegExpBacktracks", 1, 100000000, 50);
                /* Number of times a cache event is triggered for a wasm module. */
                $hr!(wasm_cache_count, "V8.WasmCacheCount", 0, 100, 101);
                /* Number of in-use external pointers in the external pointer table. */
                /* Counted after sweeping the table at the end of mark-compact GC. */
                $hr!(
                    external_pointers_count,
                    "V8.SandboxedExternalPointersCount",
                    0,
                    super::kMaxExternalPointers,
                    101
                );
                $hr!(
                    code_pointers_count,
                    "V8.SandboxedCodePointersCount",
                    0,
                    super::kMaxCodePointers,
                    101
                );
                $hr!(
                    trusted_pointers_count,
                    "V8.SandboxedTrustedPointersCount",
                    0,
                    super::kMaxTrustedPointers,
                    101
                );
                $hr!(
                    cppheap_pointers_count,
                    "V8.SandboxedCppHeapPointersCount",
                    0,
                    super::kMaxCppHeapPointers,
                    101
                );
                $hr!(
                    js_dispatch_table_entries_count,
                    "V8.JSDispatchTableEntriesCount",
                    0,
                    super::kMaxJSDispatchEntries,
                    101
                );
                /* Outcome of external pointer table compaction: kSuccess, */
                /* kPartialSuccessor kAbortedDuringSweeping. See */
                /* ExternalPointerTable::TableCompactionOutcome enum for more details. */
                $hr!(
                    external_pointer_table_compaction_outcome,
                    "V8.ExternalPointerTableCompactionOutcome",
                    0,
                    2,
                    3
                );
                $hr!(
                    wasm_compilation_method,
                    "V8.WasmCompilationMethod",
                    0,
                    4,
                    5
                );
                $hr!(
                    asmjs_instantiate_result,
                    "V8.AsmjsInstantiateResult",
                    0,
                    1,
                    2
                );
            };
        }

        #[macro_export]
        macro_rules! histogram_range_list_slow {
            ($hr:ident) => {
                /* Percentage (*1000) of time spent running Wasm jitted code. */
                $hr!(
                    wasm_jit_execution_ratio,
                    "V8.JitWasmExecutionPercentage",
                    0,
                    100000,
                    101
                );
                $hr!(
                    wasm_jit_execution_too_slow,
                    "V8.JitWasmExecutionTooSlow",
                    0,
                    100000,
                    101
                );
                /* Percentage (*1000) of time spent running in the Wasm interpreter. */
                $hr!(
                    wasm_jitless_execution_ratio,
                    "V8.JitlessWasmExecutionPercentage",
                    0,
                    100000,
                    101
                );
                $hr!(
                    wasm_jitless_execution_too_slow,
                    "V8.JitlessWasmExecutionTooSlow",
                    0,
                    100000,
                    101
                );
            };
        }

        // Like TIMED_HISTOGRAM_LIST, but allows the use of NestedTimedHistogramScope.
        #[macro_export]
        macro_rules! nested_timed_histogram_list {
            ($ht:ident) => {
                /* Garbage collection timers. */
                $ht!(
                    gc_incremental_marking,
                    "V8.GCIncrementalMarking",
                    10000,
                    super::MILLISECOND
                );
                $ht!(
                    gc_incremental_marking_start,
                    "V8.GCIncrementalMarkingStart",
                    10000,
                    super::MILLISECOND
                );
                $ht!(
                    gc_minor_incremental_marking_start,
                    "V8.GCMinorIncrementalMarkingStart",
                    10000,
                    super::MILLISECOND
                );
                $ht!(
                    gc_low_memory_notification,
                    "V8.GCLowMemoryNotification",
                    10000,
                    super::MILLISECOND
                );
                /* Compilation times. */
                $ht!(
                    collect_source_positions,
                    "V8.CollectSourcePositions",
                    1000000,
                    super::MICROSECOND
                );
                $ht!(compile, "V8.CompileMicroSeconds", 1000000, super::MICROSECOND);
                $ht!(
                    compile_eval,
                    "V8.CompileEvalMicroSeconds",
                    1000000,
                    super::MICROSECOND
                );
                /* Serialization as part of compilation (code caching). */
                $ht!(
                    compile_serialize,
                    "V8.CompileSerializeMicroSeconds",
                    100000,
                    super::MICROSECOND
                );
                $ht!(
                    compile_deserialize,
                    "V8.CompileDeserializeMicroSeconds",
                    1000000,
                    super::MICROSECOND
                );
                /* Snapshot. */
                $ht!(
                    snapshot_decompress,
                    "V8.SnapshotDecompressMicroSeconds",
                    1000000,
                    super::MICROSECOND
                );
                $ht!(
                    snapshot_deserialize_rospace,
                    "V8.SnapshotDeserializeRoSpaceMicroSeconds",
                    1000000,
                    super::MICROSECOND
                );
                $ht!(
                    snapshot_deserialize_isolate,
                    "V8.SnapshotDeserializeIsolateMicroSeconds",
                    1000000,
                    super::MICROSECOND
                );
                $ht!(
                    snapshot_deserialize_context,
                    "V8.SnapshotDeserializeContextMicroSeconds",
                    1000000,
                    super::MICROSECOND
                );
                /* ... and also see compile_deserialize above. */
                /* Total compilation time incl. caching/parsing. */
                $ht!(
                    compile_script,
                    "V8.CompileScriptMicroSeconds",
                    1000000,
                    super::MICROSECOND
                );
            };
        }

        #[macro_export]
        macro_rules! nested_timed_histogram_list_slow {
            ($ht:ident) => {
                /* Total V8 time (including JS and runtime calls, exluding callbacks). */
                $ht!(
                    execute,
                    "V8.ExecuteMicroSeconds",
                    1000000,
                    super::MICROSECOND
                );
            };
        }

        // Timer histograms, thread safe: HT(name, caption, max, unit)
        #[macro_export]
        macro_rules! timed_histogram_list {
            ($ht:ident) => {
                /* Garbage collection timers. */
                $ht!(
                    gc_finalize_incremental_regular,
                    "V8.GC.Event.MainThread.Full.Finalize.Incremental.Regular",
                    10000,
                    super::MILLISECOND
                );
                $ht!(
                    gc_finalize_incremental_regular_foreground,
                    "V8.GC.Event.MainThread.Full.Finalize.Incremental.Regular.Foreground",
                    10000,
                    super::MILLISECOND
                );
                $ht!(
                    gc_finalize_incremental_regular_background,
                    "V8.GC.Event.MainThread.Full.Finalize.Incremental.Regular.Background",
                    10000,
                    super::MILLISECOND
                );
                $ht!(
                    gc_finalize_incremental_memory_reducing,
                    "V8.GC.Event.MainThread.Full.Finalize.Incremental.ReduceMemory",
                    10000,
                    super::MILLISECOND
                );
                $ht!(
                    gc_finalize_incremental_memory_reducing_foreground,
                    "V8.GC.Event.MainThread.Full.Finalize.Incremental.ReduceMemory.Foreground",
                    10000,
                    super::MILLISECOND
                );
                $ht!(
                    gc_finalize_incremental_memory_reducing_background,
                    "V8.GC.Event.MainThread.Full.Finalize.Incremental.ReduceMemory.Background",
                    10000,
                    super::MILLISECOND
                );
                $ht!(
                    gc_finalize_incremental_memory_measure,
                    "V8.GC.Event.MainThread.Full.Finalize.Incremental.MeasureMemory",
                    10000,
                    super::MILLISECOND
                );
                $ht!(
                    gc_finalize_incremental_memory_measure_foreground,
                    "V8.GC.Event.MainThread.Full.Finalize.Incremental.MeasureMemory\
                     .Foreground",
                    10000,
                    super::MILLISECOND
                );
                $ht!(
                    gc_finalize_incremental_memory_measure_background,
                    "V8.GC.Event.MainThread.Full.Finalize.Incremental.MeasureMemory\
                     .Background",
                    10000,
                    super::MILLISECOND
                );
                $ht!(
                    gc_finalize_non_incremental_regular,
                    "V8.GC.Event.MainThread.Full.Finalize.NonIncremental.Regular",
                    10000,
                    super::MILLISECOND
                );
                $ht!(
                    gc_finalize_non_incremental_regular_foreground,
                    "V8.GC.Event.MainThread.Full.Finalize.NonIncremental.Regular.Foreground",
                    10000,
                    super::MILLISECOND
                );
                $ht!(
                    gc_finalize_non_incremental_regular_background,
                    "V8.GC.Event.MainThread.Full.Finalize.NonIncremental.Regular.Background",
                    10000,
                    super::MILLISECOND
                );
                $ht!(
                    gc_finalize_non_incremental_memory_reducing,
                    "V8.GC.Event.MainThread.Full.Finalize.NonIncremental.ReduceMemory",
                    10000,
                    super::MILLISECOND
                );
                $ht!(
                    gc_finalize_non_incremental_memory_reducing_foreground,
                    "V8.GC.Event.MainThread.Full.Finalize.NonIncremental.ReduceMemory\
                     .Foreground",
                    10000,
                    super::MILLISECOND
                );
                $ht!(
                    gc_finalize_non_incremental_memory_reducing_background,
                    "V8.GC.Event.MainThread.Full.Finalize.NonIncremental.ReduceMemory\
                     .Background",
                    10000,
                    super::MILLISECOND
                );
                $ht!(
                    gc_finalize_non_incremental_memory_measure,
                    "V8.GC.Event.MainThread.Full.Finalize.NonIncremental.MeasureMemory",
                    10000,
                    super::MILLISECOND
                );
                $ht!(
                    gc_finalize_non_incremental_memory_measure_foreground,
                    "V8.GC.Event.MainThread.Full.Finalize.NonIncremental.MeasureMemory\
                     .Foreground",
                    10000,
                    super::MILLISECOND
                );
                $ht!(
                    gc_finalize_non_incremental_memory_measure_background,
                    "V8.GC.Event.MainThread.Full.Finalize.NonIncremental.MeasureMemory\
                     .Background",
                    10000,
                    super::MILLISECOND
                );
                $ht!(
                    measure_memory_delay_ms,
                    "V8.MeasureMemoryDelayMilliseconds",
                    100000,
                    super::MILLISECOND
                );
                $ht!(
                    gc_time_to_global_safepoint,
                    "V8.GC.TimeToGlobalSafepoint",
                    10000000,
                    super::MICROSECOND
                );
                $ht!(
                    gc_time_to_safepoint,
                    "V8.GC.TimeToSafepoint",
                    10000000,
                    super::MICROSECOND
                );
                $ht!(
                    gc_time_to_collection_on_background,
                    "V8.GC.TimeToCollectionOnBackground",
                    10000000,
                    super::MICROSECOND
                );
                /* Maglev timers. */
                $ht!(
                    maglev_optimize_prepare,
                    "V8.MaglevOptimizePrepare",
                    100000,
                    super::MICROSECOND
                );
                $ht!(
                    maglev_optimize_execute,
                    "V8.MaglevOptimizeExecute",
                    100000,
                    super::MICROSECOND
                );
                $ht!(
                    maglev_optimize_finalize,
                    "V8.MaglevOptimizeFinalize",
                    100000,
                    super::MICROSECOND
                );
                $ht!(
                    maglev_optimize_total_time,
                    "V8.MaglevOptimizeTotalTime",
                    1000000,
                    super::MICROSECOND
                );
                /* TurboFan timers. */
                $ht!(
                    turbofan_optimize_prepare,
                    "V8.TurboFanOptimizePrepare",
                    1000000,
                    super::MICROSECOND
                );
                $ht!(
                    turbofan_optimize_execute,
                    "V8.TurboFanOptimizeExecute",
                    1000000,
                    super::MICROSECOND
                );
                $ht!(
                    turbofan_optimize_finalize,
                    "V8.TurboFanOptimizeFinalize",
                    1000000,
                    super::MICROSECOND
                );
                $ht!(
                    turbofan_optimize_total_foreground,
                    "V8.TurboFanOptimizeTotalForeground",
                    10000000,
                    super::MICROSECOND
                );
                $ht!(
                    turbofan_optimize_total_background,
                    "V8.TurboFanOptimizeTotalBackground",
                    10000000,
                    super::MICROSECOND
                );
                $ht!(
                    turbofan_optimize_total_time,
                    "V8.TurboFanOptimizeTotalTime",
                    10000000,
                    super::MICROSECOND
                );
                $ht!(
                    turbofan_optimize_non_concurrent_total_time,
                    "V8.TurboFanOptimizeNonConcurrentTotalTime",
                    10000000,
                    super::MICROSECOND
                );
                $ht!(
                    turbofan_optimize_concurrent_total_time,
                    "V8.TurboFanOptimizeConcurrentTotalTime",
                    10000000,
                    super::MICROSECOND
                );
                $ht!(
                    turbofan_osr_prepare,
                    "V8.TurboFanOptimizeForOnStackReplacementPrepare",
                    1000000,
                    super::MICROSECOND
                );
                $ht!(
                    turbofan_osr_execute,
                    "V8.TurboFanOptimizeForOnStackReplacementExecute",
                    1000000,
                    super::MICROSECOND
                );
                $ht!(
                    turbofan_osr_finalize,
                    "V8.TurboFanOptimizeForOnStackReplacementFinalize",
                    1000000,
                    super::MICROSECOND
                );
                $ht!(
                    turbofan_osr_total_time,
                    "V8.TurboFanOptimizeForOnStackReplacementTotalTime",
                    10000000,
                    super::MICROSECOND
                );
                /* Wasm timers. */
                $ht!(
                    wasm_compile_asm_module_time,
                    "V8.WasmCompileModuleMicroSeconds.asm",
                    10000000,
                    super::MICROSECOND
                );
                $ht!(
                    wasm_compile_wasm_module_time,
                    "V8.WasmCompileModuleMicroSeconds.wasm",
                    10000000,
                    super::MICROSECOND
                );
                $ht!(
                    wasm_async_compile_wasm_module_time,
                    "V8.WasmCompileModuleAsyncMicroSeconds",
                    100000000,
                    super::MICROSECOND
                );
                $ht!(
                    wasm_streaming_compile_wasm_module_time,
                    "V8.WasmCompileModuleStreamingMicroSeconds",
                    100000000,
                    super::MICROSECOND
                );
                $ht!(
                    wasm_streaming_finish_wasm_module_time,
                    "V8.WasmFinishModuleStreamingMicroSeconds",
                    100000000,
                    super::MICROSECOND
                );
                $ht!(
                    wasm_deserialization_time,
                    "V8.WasmDeserializationTimeMilliSeconds",
                    10000,
                    super::MILLISECOND
                );
                $ht!(
                    wasm_compile_asm_function_time,
                    "V8.WasmCompileFunctionMicroSeconds.asm",
                    1000000,
                    super::MICROSECOND
                );
                $ht!(
                    wasm_compile_wasm_function_time,
                    "V8.WasmCompileFunctionMicroSeconds.wasm",
                    1000000,
                    super::MICROSECOND
                );
                $ht!(
                    wasm_compile_huge_function_time,
                    "V8.WasmCompileHugeFunctionMilliSeconds",
                    100000,
                    super::MILLISECOND
                );
                $ht!(
                    wasm_instantiate_wasm_module_time,
                    "V8.WasmInstantiateModuleMicroSeconds.wasm",
                    10000000,
                    super::MICROSECOND
                );
                $ht!(
                    wasm_instantiate_asm_module_time,
                    "V8.WasmInstantiateModuleMicroSeconds.asm",
                    10000000,
                    super::MICROSECOND
                );
                $ht!(
                    wasm_lazy_compile_time,
                    "V8.WasmLazyCompileTimeMicroSeconds",
                    100000000,
                    super::MICROSECOND
                );
                $ht!(
                    wasm_compile_after_deserialize,
                    "V8.WasmCompileAfterDeserializeMilliSeconds",
                    1000000,
                    super::MILLISECOND
                );
                /* Total compilation time incl. caching/parsing for various cache states. */
                $ht!(
                    compile_script_with_produce_cache,
                    "V8.CompileScriptMicroSeconds.ProduceCache",
                    1000000,
                    super::MICROSECOND
                );
                $ht!(
                    compile_script_with_isolate_cache_hit,
                    "V8.CompileScriptMicroSeconds.IsolateCacheHit",
                    1000000,
                    super::MICROSECOND
                );
                $ht!(
                    compile_script_with_consume_cache,
                    "V8.CompileScriptMicroSeconds.ConsumeCache",
                    1000000,
                    super::MICROSECOND
                );
                $ht!(
                    compile_script_consume_failed,
                    "V8.CompileScriptMicroSeconds.ConsumeCache.Failed",
                    1000000,
                    super::MICROSECOND
                );
                $ht!(
                    compile_script_no_cache_other,
                    "V8.CompileScriptMicroSeconds.NoCache.Other",
                    1000000,
                    super::MICROSECOND
                );
                $ht!(
                    compile_script_no_cache_because_inline_script,
                    "V8.CompileScriptMicroSeconds.NoCache.InlineScript",
                    1000000,
                    super::MICROSECOND
                );
                $ht!(
                    compile_script_no_cache_because_script_too_small,
                    "V8.CompileScriptMicroSeconds.NoCache.ScriptTooSmall",
                    1000000,
                    super::MICROSECOND
                );
                $ht!(
                    compile_script_no_cache_because_cache_too_cold,
                    "V8.CompileScriptMicroSeconds.NoCache.CacheTooCold",
                    1000000,
                    super::MICROSECOND
                );
                $ht!(
                    compile_script_streaming_finalization,
                    "V8.CompileScriptMicroSeconds.StreamingFinalization",
                    1000000,
                    super::MICROSECOND
                );
                $ht!(
                    compile_script_on_background,
                    "V8.CompileScriptMicroSeconds.BackgroundThread",
                    1000000,
                    super::MICROSECOND
                );
                $ht!(
                    compile_function_on_background,
                    "V8.CompileFunctionMicroSeconds.BackgroundThread",
                    1000000,
                    super::MICROSECOND
                );
                $ht!(
                    deserialize_script_on_background,
                    "V8.CompileScriptMicroSeconds.ConsumeCache.BackgroundThread",
                    1000000,
                    super::MICROSECOND
                );
                /* Debugger timers. */
                $ht!(
                    debug_pause_to_paused_event,
                    "V8.DebugPauseToPausedEventMilliSeconds",
                    1000000,
                    super::MILLISECOND
                );
            };
        }

        #[macro_export]
        macro_rules! aggregatable_histogram_timer_list {
            ($aht:ident) => {
                $aht!(compile_lazy, "V8.CompileLazyMicroSeconds");
            };
        }

        #[macro_export]
        macro_rules! histogram_percentage_list {
            ($hp:ident) => {
                /* Heap fragmentation. */
                $hp!(
                    external_fragmentation_total,
                    "V8.MemoryExternalFragmentationTotal"
                );
                $hp!(
                    external_fragmentation_old_space,
                    "V8.MemoryExternalFragmentationOldSpace"
                );
                $hp!(
                    external_fragmentation_code_space,
                    "V8.MemoryExternalFragmentationCodeSpace"
                );
                $hp!(
                    external_fragmentation_map_space,
                    "V8.MemoryExternalFragmentationMapSpace"
                );
                $hp!(
                    external_fragmentation_lo_space,
                    