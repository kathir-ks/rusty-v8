// Converted from V8 C++ source files:
// Header: flag-definitions.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod flag_definitions {
    use std::{ffi::c_char, io::Write, rc::Rc, sync::Arc};

    use crate::{
        compiler::{
            backend::riscv::instruction_selector_riscv::InstructionOperand,
            js_heap_broker::BrokerMode, simplified_lowering_verifier::OpIndex,
            turboshaft::build_graph_phase::Operation, wasm_gc_operator_reducer::NodeId,
        },
        execution::isolate::Isolate,
        init::{bootstrapper::SharedFunctionInfo, isolate_group::Flags},
    };

    macro_rules! DEFINE_IMPLICATION {
        ($whenflag:ident, $thenflag:ident) => {
            DEFINE_VALUE_IMPLICATION!($whenflag, $thenflag, true);
        };
    }

    macro_rules! DEFINE_WEAK_IMPLICATION {
        ($whenflag:ident, $thenflag:ident) => {
            DEFINE_WEAK_VALUE_IMPLICATION!($whenflag, $thenflag, true);
        };
    }

    macro_rules! DEFINE_WEAK_NEG_IMPLICATION {
        ($whenflag:ident, $thenflag:ident) => {
            DEFINE_WEAK_VALUE_IMPLICATION!($whenflag, $thenflag, false);
        };
    }

    macro_rules! DEFINE_NEG_IMPLICATION {
        ($whenflag:ident, $thenflag:ident) => {
            DEFINE_VALUE_IMPLICATION!($whenflag, $thenflag, false);
        };
    }

    macro_rules! DEFINE_NEG_NEG_IMPLICATION {
        ($whenflag:ident, $thenflag:ident) => {
            DEFINE_NEG_VALUE_IMPLICATION!($whenflag, $thenflag, false);
        };
    }

    #[macro_export]
    macro_rules! FLAG {
        (BOOL, $ctype:ty, $nam:ident, $def:expr, $cmt:expr) => {
            #[derive(Clone, Copy, Debug)]
            pub struct $nam {
                pub value: $ctype,
            }
            impl $nam {
                pub const fn new() -> Self {
                    Self { value: $def }
                }
                pub fn get(&self) -> $ctype {
                    self.value
                }
                pub fn set(&mut self, new_value: $ctype) {
                    self.value = new_value;
                }
            }
        };
        (INT, $ctype:ty, $nam:ident, $def:expr, $cmt:expr) => {
            #[derive(Clone, Copy, Debug)]
            pub struct $nam {
                pub value: $ctype,
            }
            impl $nam {
                pub const fn new() -> Self {
                    Self { value: $def }
                }
                pub fn get(&self) -> $ctype {
                    self.value
                }
                pub fn set(&mut self, new_value: $ctype) {
                    self.value = new_value;
                }
            }
        };
        (UINT, $ctype:ty, $nam:ident, $def:expr, $cmt:expr) => {
            #[derive(Clone, Copy, Debug)]
            pub struct $nam {
                pub value: $ctype,
            }
            impl $nam {
                pub const fn new() -> Self {
                    Self { value: $def }
                }
                pub fn get(&self) -> $ctype {
                    self.value
                }
                pub fn set(&mut self, new_value: $ctype) {
                    self.value = new_value;
                }
            }
        };
        (UINT64, $ctype:ty, $nam:ident, $def:expr, $cmt:expr) => {
            #[derive(Clone, Copy, Debug)]
            pub struct $nam {
                pub value: $ctype,
            }
            impl $nam {
                pub const fn new() -> Self {
                    Self { value: $def }
                }
                pub fn get(&self) -> $ctype {
                    self.value
                }
                pub fn set(&mut self, new_value: $ctype) {
                    self.value = new_value;
                }
            }
        };
        (FLOAT, $ctype:ty, $nam:ident, $def:expr, $cmt:expr) => {
            #[derive(Clone, Copy, Debug)]
            pub struct $nam {
                pub value: $ctype,
            }
            impl $nam {
                pub const fn new() -> Self {
                    Self { value: $def }
                }
                pub fn get(&self) -> $ctype {
                    self.value
                }
                pub fn set(&mut self, new_value: $ctype) {
                    self.value = new_value;
                }
            }
        };
        (SIZE_T, $ctype:ty, $nam:ident, $def:expr, $cmt:expr) => {
            #[derive(Clone, Copy, Debug)]
            pub struct $nam {
                pub value: $ctype,
            }
            impl $nam {
                pub const fn new() -> Self {
                    Self { value: $def }
                }
                pub fn get(&self) -> $ctype {
                    self.value
                }
                pub fn set(&mut self, new_value: $ctype) {
                    self.value = new_value;
                }
            }
        };
        (STRING, $ctype:ty, $nam:ident, $def:expr, $cmt:expr) => {
            #[derive(Clone, Copy, Debug)]
            pub struct $nam {
                pub value: $ctype,
            }
            impl $nam {
                pub const fn new() -> Self {
                    Self { value: $def }
                }
                pub fn get(&self) -> $ctype {
                    self.value
                }
                pub fn set(&mut self, new_value: $ctype) {
                    self.value = new_value;
                }
            }
        };
        (MAYBE_BOOL, $ctype:ty, $nam:ident, $def:expr, $cmt:expr) => {
            #[derive(Clone, Copy, Debug)]
            pub struct $nam {
                pub value: $ctype,
            }
            impl $nam {
                pub const fn new() -> Self {
                    Self { value: $def }
                }
                pub fn get(&self) -> $ctype {
                    match self.value {
                        Some(b) => b,
                        None => false,
                    }
                }
                pub fn set(&mut self, new_value: $ctype) {
                    self.value = Some(new_value);
                }
            }
        };
    }

    macro_rules! FLAG_READONLY {
        (BOOL, $ctype:ty, $nam:ident, $def:expr, $cmt:expr) => {
            pub const $nam: FlagValue<$ctype> = FlagValue::<$ctype> { value: $def };
        };
        (INT, $ctype:ty, $nam:ident, $def:expr, $cmt:expr) => {
            pub const $nam: FlagValue<$ctype> = FlagValue::<$ctype> { value: $def };
        };
        (UINT, $ctype:ty, $nam:ident, $def:expr, $cmt:expr) => {
            pub const $nam: FlagValue<$ctype> = FlagValue::<$ctype> { value: $def };
        };
        (UINT64, $ctype:ty, $nam:ident, $def:expr, $cmt:expr) => {
            pub const $nam: FlagValue<$ctype> = FlagValue::<$ctype> { value: $def };
        };
        (FLOAT, $ctype:ty, $nam:ident, $def:expr, $cmt:expr) => {
            pub const $nam: FlagValue<$ctype> = FlagValue::<$ctype> { value: $def };
        };
        (SIZE_T, $ctype:ty, $nam:ident, $def:expr, $cmt:expr) => {
            pub const $nam: FlagValue<$ctype> = FlagValue::<$ctype> { value: $def };
        };
        (STRING, $ctype:ty, $nam:ident, $def:expr, $cmt:expr) => {
            pub const $nam: FlagValue<$ctype> = FlagValue::<$ctype> { value: $def };
        };
    }

    macro_rules! FLAG_ALIAS {
        (BOOL, $ctype:ty, $alias:ident, $nam:ident) => {};
    }

    #[derive(Clone, Copy, Debug)]
    pub struct FlagValue<T> {
        pub value: T,
    }

    impl<T> FlagValue<T> {
        pub const fn new(value: T) -> Self {
            FlagValue { value }
        }
    }

    macro_rules! DEFINE_VALUE_IMPLICATION {
        ($whenflag:ident, $thenflag:ident, $value:expr) => {};
    }

    macro_rules! DEFINE_WEAK_VALUE_IMPLICATION {
        ($whenflag:ident, $thenflag:ident, $value:expr) => {};
    }

    macro_rules! DEFINE_GENERIC_IMPLICATION {
        ($whenflag:ident, $statement:expr) => {};
    }

    macro_rules! DEFINE_REQUIREMENT {
        ($statement:expr) => {};
    }

    macro_rules! DEFINE_NEG_VALUE_IMPLICATION {
        ($whenflag:ident, $thenflag:ident, $value:expr) => {};
    }

    macro_rules! DEFINE_NEG_VALUE_VALUE_IMPLICATION {
        ($whenflag:ident, $whenvalue:ident, $thenflag:ident, $thenvalue:ident) => {};
    }

    macro_rules! DEFINE_MIN_VALUE_IMPLICATION {
        ($flag:ident, $min_value:ident) => {};
    }

    macro_rules! DEFINE_DISABLE_FLAG_IMPLICATION {
        ($whenflag:ident, $thenflag:ident) => {};
    }

    macro_rules! DEFINE_EXPERIMENTAL_FEATURE {
        ($nam:ident, $cmt:expr) => {
            FLAG!(BOOL, bool, $nam, false, $cmt);
            DEFINE_IMPLICATION!($nam, experimental);
        };
    }

    macro_rules! DEFINE_DEBUG_BOOL {
        ($nam:ident, $def:expr, $cmt:expr) => {
            FLAG!(BOOL, bool, $nam, $def, $cmt);
        };
    }

    pub use DEFINE_DEBUG_BOOL as DEFINE_BOOL;
    pub use DEFINE_EXPERIMENTAL_FEATURE as FLAG;

    FLAG!(BOOL, bool, experimental, false, "");
    DEFINE_BOOL!(
        abort_on_contradictory_flags,
        false,
        "Disallow flags or implications overriding each other."
    );
    DEFINE_NEG_IMPLICATION!(fuzzing, abort_on_contradictory_flags);
    DEFINE_BOOL!(
        exit_on_contradictory_flags,
        false,
        "Exit with return code 0 on contradictory flags."
    );
    DEFINE_WEAK_IMPLICATION!(exit_on_contradictory_flags, abort_on_contradictory_flags);
    DEFINE_BOOL!(
        allow_overwriting_for_next_flag,
        false,
        "temporary disable flag contradiction to allow overwriting just "
    );
    DEFINE_BOOL!(use_strict, false, "enforce strict mode");
    DEFINE_BOOL!(trace_temporal, false, "trace temporal code");
    DEFINE_BOOL!(harmony, false, "enable all completed harmony features");
    DEFINE_BOOL!(harmony_shipping, true, "enable all shipped harmony features");
    DEFINE_BOOL!(js_staging, false, "enable all completed JavaScript features");
    DEFINE_BOOL!(js_shipping, true, "enable all shipped JavaScript features");

    DEFINE_EXPERIMENTAL_FEATURE!(harmony_temporal, "Temporal");
    DEFINE_EXPERIMENTAL_FEATURE!(harmony_shadow_realm, "harmony ShadowRealm");
    DEFINE_EXPERIMENTAL_FEATURE!(
        harmony_struct,
        "harmony structs, shared structs, and shared arrays"
    );
    DEFINE_EXPERIMENTAL_FEATURE!(js_decorators, "decorators");
    DEFINE_EXPERIMENTAL_FEATURE!(js_source_phase_imports, "source phase imports");
    DEFINE_EXPERIMENTAL_FEATURE!(js_base_64, "Uint8Array to/from base64 and hex");

    DEFINE_EXPERIMENTAL_FEATURE!(
        harmony_intl_best_fit_matcher,
        "Intl BestFitMatcher"
    );

    DEFINE_BOOL!(js_explicit_resource_management, false, "explicit resource management");
    DEFINE_IMPLICATION!(harmony, js_explicit_resource_management);
    DEFINE_IMPLICATION!(js_staging, js_explicit_resource_management);

    DEFINE_BOOL!(
        js_float16array,
        false,
        "Float16Array, Math.f16round, DataView.getFloat16, DataView.setFloat16"
    );
    DEFINE_IMPLICATION!(harmony, js_float16array);
    DEFINE_IMPLICATION!(js_staging, js_float16array);

    DEFINE_BOOL!(js_regexp_duplicate_named_groups, true, "RegExp duplicate named groups");
    DEFINE_NEG_NEG_IMPLICATION!(harmony_shipping, js_regexp_duplicate_named_groups);
    DEFINE_NEG_NEG_IMPLICATION!(js_shipping, js_regexp_duplicate_named_groups);

    DEFINE_BOOL!(js_regexp_modifiers, true, "RegExp modifiers");
    DEFINE_NEG_NEG_IMPLICATION!(harmony_shipping, js_regexp_modifiers);
    DEFINE_NEG_NEG_IMPLICATION!(js_shipping, js_regexp_modifiers);

    DEFINE_BOOL!(js_promise_try, true, "Promise.try");
    DEFINE_NEG_NEG_IMPLICATION!(harmony_shipping, js_promise_try);
    DEFINE_NEG_NEG_IMPLICATION!(js_shipping, js_promise_try);

    DEFINE_BOOL!(js_atomics_pause, true, "Atomics.pause");
    DEFINE_NEG_NEG_IMPLICATION!(harmony_shipping, js_atomics_pause);
    DEFINE_NEG_NEG_IMPLICATION!(js_shipping, js_atomics_pause);

    DEFINE_BOOL!(js_error_iserror, true, "Error.isError");
    DEFINE_NEG_NEG_IMPLICATION!(harmony_shipping, js_error_iserror);
    DEFINE_NEG_NEG_IMPLICATION!(js_shipping, js_error_iserror);

    DEFINE_BOOL!(js_regexp_escape, true, "RegExp.escape");
    DEFINE_NEG_NEG_IMPLICATION!(harmony_shipping, js_regexp_escape);
    DEFINE_NEG_NEG_IMPLICATION!(js_shipping, js_regexp_escape);

    DEFINE_BOOL!(harmony_iterator_helpers, true, "JavaScript iterator helpers");
    DEFINE_NEG_NEG_IMPLICATION!(harmony_shipping, harmony_iterator_helpers);
    DEFINE_NEG_NEG_IMPLICATION!(js_shipping, harmony_iterator_helpers);

    DEFINE_BOOL!(harmony_set_methods, true, "harmony Set Methods");
    DEFINE_NEG_NEG_IMPLICATION!(harmony_shipping, harmony_set_methods);
    DEFINE_NEG_NEG_IMPLICATION!(js_shipping, harmony_set_methods);

    DEFINE_BOOL!(harmony_import_attributes, true, "harmony import attributes");
    DEFINE_NEG_NEG_IMPLICATION!(harmony_shipping, harmony_import_attributes);
    DEFINE_NEG_NEG_IMPLICATION!(js_shipping, harmony_import_attributes);

    DEFINE_BOOL!(builtin_subclassing, true, "subclassing support in built-in methods");
    DEFINE_BOOL!(
        enable_sharedarraybuffer_per_context,
        false,
        "enable the SharedArrayBuffer constructor per context"
    );
    DEFINE_BOOL!(icu_timezone_data, true, "get information about timezones from ICU");
    pub const enable_double_const_store_check: FlagValue<bool> =
        FlagValue::<bool> { value: true };
    pub const lazy_source_positions: FlagValue<bool> = FlagValue::<bool> { value: true };
    DEFINE_BOOL!(stress_snapshot, false, "disables sharing of the read-only heap for testing");
    DEFINE_NEG_IMPLICATION!(stress_snapshot, incremental_marking);
    pub const lite_mode: FlagValue<bool> = FlagValue::<bool> { value: false };
    DEFINE_IMPLICATION!(lite_mode, jitless);
    DEFINE_IMPLICATION!(lite_mode, optimize_for_size);
    pub const enable_allocation_folding: FlagValue<bool> =
        FlagValue::<bool> { value: true };
    DEFINE_NEG_NEG_IMPLICATION!(enable_allocation_folding, turbo_allocation_folding);
    pub const disable_write_barriers: FlagValue<bool> =
        FlagValue::<bool> { value: true };
    DEFINE_NEG_IMPLICATION!(disable_write_barriers, incremental_marking);
    DEFINE_NEG_IMPLICATION!(disable_write_barriers, concurrent_marking);
    DEFINE_NEG_IMPLICATION!(disable_write_barriers, cppheap_incremental_marking);
    DEFINE_NEG_IMPLICATION!(disable_write_barriers, cppheap_concurrent_marking);
    pub const enable_unconditional_write_barriers: FlagValue<bool> =
        FlagValue::<bool> { value: true };
    pub const single_generation: FlagValue<bool> = FlagValue::<bool> { value: true };
    DEFINE_BOOL!(
        conservative_stack_scanning,
        true,
        "use conservative stack scanning"
    );
    DEFINE_IMPLICATION!(conservative_stack_scanning, minor_ms);
    DEFINE_NEG_IMPLICATION!(conservative_stack_scanning, compact_with_stack);
    pub const direct_handle: FlagValue<bool> = FlagValue::<bool> { value: true };
    DEFINE_NEG_NEG_IMPLICATION!(conservative_stack_scanning, direct_handle);

    DEFINE_EXPERIMENTAL_FEATURE!(
        scavenger_conservative_object_pinning,
        "Objects reachable from the native stack during scavenge will be pinned and won't move."
    );
    DEFINE_IMPLICATION!(scavenger_conservative_object_pinning, separate_gc_phases);
    DEFINE_BOOL!(
        stress_scavenger_conservative_object_pinning,
        false,
        "Treat some precise references as conservative references to stress test object pinning in Scavenger"
    );
    DEFINE_IMPLICATION!(stress_scavenger_conservative_object_pinning, scavenger_conservative_object_pinning);
    DEFINE_NEG_IMPLICATION!(stress_scavenger_conservative_object_pinning, minor_gc_task);
    DEFINE_VALUE_IMPLICATION!(stress_scavenger_conservative_object_pinning, scavenger_max_new_space_capacity_mb, 1);
    DEFINE_BOOL!(
        stress_scavenger_conservative_object_pinning_random,
        false,
        "Enables random stressing of object pinning in Scavenger, such that each GC would randomly pick a subset of the precise references to treat conservatively"
    );
    DEFINE_IMPLICATION!(stress_scavenger_conservative_object_pinning_random, stress_scavenger_conservative_object_pinning);

    DEFINE_EXPERIMENTAL_FEATURE!(
        scavenger_precise_object_pinning,
        "Objects reachable from handles during scavenge will be pinned and won't move."
    );
    DEFINE_IMPLICATION!(scavenger_precise_object_pinning, separate_gc_phases);

    DEFINE_EXPERIMENTAL_FEATURE!(
        precise_object_pinning,
        "Objects reachable from handles during GC will be pinned and won't move."
    );
    DEFINE_IMPLICATION!(precise_object_pinning, scavenger_precise_object_pinning);

    DEFINE_BOOL!(scavenger_promote_quarantined_pages, true, "Quarantined pages in the intermediate generation will be promoted to old space");
    DEFINE_BOOL!(
        local_off_stack_check,
        true,
        "check for off-stack allocation of v8::Local"
    );
    DEFINE_BOOL!(future, false, "");

    DEFINE_BOOL!(force_emit_interrupt_budget_checks, false, "");

    DEFINE_BOOL!(maglev, true, "enable the maglev optimizing compiler");
    DEFINE_EXPERIMENTAL_FEATURE!(maglev_future, "enable maglev features");
    DEFINE_IMPLICATION!(maglev_future, maglev);
    DEFINE_BOOL!(
        optimize_on_next_call_optimizes_to_maglev,
        false,
        ""
    );
    DEFINE_BOOL!(stress_maglev, false, "trigger maglev compilation earlier");
    DEFINE_IMPLICATION!(stress_maglev, maglev);
    DEFINE_WEAK_VALUE_IMPLICATION!(stress_maglev, invocation_count_for_maglev, 4);

    DEFINE_BOOL!(maglev_inlining, true, "enable inlining in the maglev optimizing compiler");
    DEFINE_BOOL!(maglev_loop_peeling, true, "enable loop peeling in the maglev optimizing compiler");
    DEFINE_BOOL!(maglev_optimistic_peeled_loops, true, "enable aggressive optimizations for loops (loop SPeeling) in the maglev optimizing compiler");
    DEFINE_INT!(maglev_loop_peeling_max_size, 400, "max loop size for loop peeling in the maglev optimizing compiler");
    DEFINE_INT!(
        maglev_loop_peeling_max_size_cumulative,
        900,
        "max cumulative size for loop peeling in the maglev optimizing compiler"
    );
    DEFINE_BOOL!(maglev_deopt_data_on_background, true, "Generate deopt data on background thread");
    DEFINE_BOOL!(maglev_build_code_on_background, true, "Generate code on background thread");
    DEFINE_WEAK_IMPLICATION!(maglev_build_code_on_background, maglev_deopt_data_on_background);
    DEFINE_BOOL!(maglev_destroy_on_background, true, "Destroy compilation jobs on background thread");
    DEFINE_BOOL!(maglev_inline_api_calls, false, "Inline CallApiCallback builtin into generated code");
    DEFINE_BOOL!(maglev_cons_string_elision, false, "Native support for cons strings and their elision in maglev.");
    DEFINE_BOOL!(
        maglev_pretenure_store_values,
        false,
        "Recursively pretenure values which are stored into pretenured allocation sites."
    );
    DEFINE_EXPERIMENTAL_FEATURE!(maglev_licm, "loop invariant code motion");
    DEFINE_BOOL!(turbolev, true, "use Maglev as a frontend for Turboshaft");
    DEFINE_EXPERIMENTAL_FEATURE!(maglev_object_tracking, "track object changes to avoid escaping them");

    DEFINE_UINT!(
        concurrent_maglev_max_threads,
        2,
        "max number of threads that concurrent Maglev can use (0 for unbounded)"
    );
    DEFINE_BOOL!(
        concurrent_maglev_high_priority_threads,
        false,
        "use high priority compiler threads for concurrent Maglev"
    );
    DEFINE_INT!(
        max_maglev_inline_depth,
        1,
        "max depth of functions that Maglev will inline excl. small functions"
    );
    DEFINE_INT!(
        max_maglev_hard_inline_depth,
        10,
        "max depth of functions that Maglev will inline incl. small functions"
    );
    DEFINE_INT!(
        max_maglev_inlined_bytecode_size,
        460,
        "maximum size of bytecode for a single inlining"
    );
    DEFINE_INT!(
        max_maglev_inlined_bytecode_size_cumulative,
        920,
        "maximum cumulative size of bytecode considered for inlining excl. small functions"
    );
    DEFINE_INT!(
        max_maglev_inlined_bytecode_size_small,
        27,
        "maximum size of bytecode considered for small function inlining"
    );
    DEFINE_FLOAT!(
        min_maglev_inlining_frequency,
        0.10,
        "minimum frequency for inlining"
    );
    DEFINE_BOOL!(
        maglev_reuse_stack_slots,
        true,
        "reuse stack slots in the maglev optimizing compiler"
    );
    DEFINE_BOOL!(
        maglev_untagged_phis,
        true,
        "enable phi untagging in the maglev optimizing compiler"
    );
    DEFINE_BOOL!(maglev_hoist_osr_value_phi_untagging, true, "enable phi untagging to hoist untagging of osr values");
    DEFINE_EXPERIMENTAL_FEATURE!(maglev_speculative_hoist_phi_untagging, "enable phi untagging to hoist untagging of loop phi inputs (could still cause deopt loops)");
    DEFINE_BOOL!(maglev_cse, true, "common subexpression elimination");
    DEFINE_EXPERIMENTAL_FEATURE!(
        maglev_non_eager_inlining,
        "enable Maglev non-eager inlining"
    );

    DEFINE_STRING!(maglev_filter, "*", "optimization filter for the maglev compiler");
    DEFINE_STRING!(
        maglev_print_filter,
        "*",
        "filter for maglev's tracing/printing options"
    );
    DEFINE_BOOL!(maglev_assert, false, "insert extra assertion in maglev code");
    DEFINE_DEBUG_BOOL!(
        maglev_assert_stack_size,
        true,
        "insert stack size checks before every IR node"
    );
    DEFINE_BOOL!(maglev_break_on_entry, false, "insert an int3 on maglev entries");
    DEFINE_BOOL!(
        maglev_print_feedback,
        true,
        "print feedback vector for maglev compiled code"
    );
    DEFINE_BOOL!(
        maglev_print_inlined,
        true,
        "print bytecode / feedback vectors also for inlined code"
    );

    DEFINE_BOOL!(print_maglev_code, false, "print maglev code");
    DEFINE_BOOL!(
        trace_maglev_graph_building,
        false,
        "trace maglev graph building"
    );
    DEFINE_BOOL!(
        trace_maglev_loop_speeling,
        false,
        "trace maglev loop SPeeling"
    );
    DEFINE_WEAK_IMPLICATION!(trace_maglev_graph_building, trace_maglev_loop_speeling);
    DEFINE_BOOL!(trace_maglev_inlining, false, "trace maglev inlining");
    DEFINE_BOOL!(
        trace_maglev_inlining_verbose,
        false,
        "trace maglev inlining (verbose)"
    );
    DEFINE_IMPLICATION!(trace_maglev_inlining_verbose, trace_maglev_inlining);

    DEFINE_BOOL!(maglev_stats, false, "print Maglev statistics");
    DEFINE_BOOL!(
        maglev_stats_nvp,
        false,
        "print Maglev statistics in machine-readable format"
    );
    DEFINE_BOOL!(maglev_function_context_specialization, true, "enable function context specialization in maglev");
    DEFINE_BOOL!(maglev_skip_migration_check_for_polymorphic_access, false, "skip generating a migration check when some maps of polymorpic property access are migration targets");

    DEFINE_BOOL!(additive_safe_int_feedback, false, "Enable the use of AdditiveSafeInteger feedback");

    DEFINE_BOOL!(
        enable_enumerated_keyed_access_bytecode,
        true,
        "enable generating GetEnumeratedKeyedProperty bytecode for keyed access"
    );
    pub const dict_property_const_tracking: FlagValue<bool> =
        FlagValue::<bool> { value: true };

    DEFINE_BOOL!(
        const_tracking_let,
        true,
        "Use const tracking on top-level `let` variables"
    );
    DEFINE_BOOL!(
        script_context_mutable_heap_number,
        true,
        "Use mutable heap numbers in script contexts"
    );
    DEFINE_BOOL!(
        script_context_mutable_heap_int32,
        true,
        "Use mutable heap int32 number in script contexts"
    );
    DEFINE_WEAK_IMPLICATION!(
        script_context_mutable_heap_int32,
        script_context_mutable_heap_number
    );
    DEFINE_BOOL!(
        empty_context_extension_dep,
        true,
        "Use compilation dependency to avoid dynamic checks for non-empty context extensions"
    );
    DEFINE_BOOL!(
        json_stringify_fast_path,
        false,
        "Enable JSON.stringify fast-path"
    );
    DEFINE_BOOL!(
        extensible_ro_snapshot,
        true,
        "Whether custom embedder snapshots may extend ReadOnlySpace"
    );
    DEFINE_UINT!(
        max_opt,
        999,
        "Set the maximal optimisation tier: > 3 == any, 0 == ignition/interpreter, 1 == sparkplug/baseline, 2 == maglev, 3 == turbofan"
    );
    DEFINE_MAYBE_BOOL!(
        efficiency_mode,
        "Forces efficiency mode on or off, disregarding any dynamic signals. Efficiency mode is optimized for situations with no latency requirements and uses fewer threads."
    );
    DEFINE_MAYBE_BOOL!(
        battery_saver_mode,
        "Forces battery saver mode on or off, disregarding any dynamic signals. Battery saver tries to conserve overal cpu cycles spent."
    );
    DEFINE_MAYBE_BOOL!(
        memory_saver_mode,
        "Forces memory saver mode on or off, disregarding any dynamic signals. Memory saver tries to keep memory footprint low at the expense of extra cpu cycles."
    );
    DEFINE_BOOL!(
        efficiency_mode_for_tiering_heuristics,
        true,
        "Use efficiency mode in tiering heuristics."
    );
    DEFINE_BOOL!(
        efficiency_mode_disable_turbofan,
        false,
        "Defer tier-up to turbofan while in efficiency mode."
    );
    DEFINE_INT!(
        efficiency_mode_delay_turbofan,
        15000,
        "Delay tier-up to turbofan to a certain invocation count while in efficiency mode."
    );
    DEFINE_STRING!(
        wasm_trace_native,
        std::ptr::null::<c_char>() as *const c_char,
        "Select which native code sequence to use for wasm trace instruction: default or cpuid"
    );
    DEFINE_BOOL!(jitless, false, "Disable runtime allocation of executable memory.");
    DEFINE_NEG_IMPLICATION!(jitless, track_field_types);
    DEFINE_NEG_IMPLICATION!(jitless, script_context_mutable_heap_number);
    DEFINE_IMPLICATION!(jitless, regexp_interpret_all);
    DEFINE_NEG_IMPLICATION!(jitless, turbofan);
    DEFINE_NEG_IMPLICATION!(jitless, interpreted_frames_native_stack);
    DEFINE_BOOL!(
        disable_optimizing_compilers,
        false,
        "Disable all optimizing compilers while leaving baseline compilers enabled"
    );
    DEFINE_NEG_IMPLICATION!(disable_optimizing_compilers, turbofan);
    DEFINE_NEG_IMPLICATION!(disable_optimizing_compilers, maglev);
    DEFINE_NEG_IMPLICATION!(disable_optimizing_compilers, wasm_tier_up);
    DEFINE_NEG_IMPLICATION!(disable_optimizing_compilers, wasm_dynamic_tiering);
    DEFINE_NEG_IMPLICATION!(disable_optimizing_compilers, validate_asm);
    DEFINE_NEG_IMPLICATION!(disable_optimizing_compilers, track_field_types);
    DEFINE_NEG_IMPLICATION!(disable_optimizing_compilers, script_context_mutable_heap_number);
    DEFINE_BOOL!(memory_protection_keys, true, "protect code memory with PKU if available");
    DEFINE_BOOL!(assert_types, false, "generate runtime type assertions to test the typer");
    DEFINE_NEG_IMPLICATION!(assert_types, concurrent_recompilation);
    DEFINE_NEG_IMPLICATION!(assert_types, concurrent_builtin_generation);
    DEFINE_BOOL!(
        verify_simplified_lowering,
        false,
        "verify graph generated by simplified lowering"
    );
    DEFINE_BOOL!(trace_compilation_dependencies, false, "trace code dependencies");
    DEFINE_IMPLICATION!(trace_compilation_dependencies, trace_deopt_verbose);
    DEFINE_BOOL!(allocation_site_tracking, true, "Enable allocation site tracking");
    DEFINE_NEG_NEG_IMPLICATION!(allocation_site_tracking, allocation_site_pretenuring);
    DEFINE_BOOL!(allocation_site_pretenuring, true, "pretenure with allocation sites");
    DEFINE_BOOL!(page_promotion, true, "promote pages based on utilization");
    DEFINE_INT!(
        page_promotion_threshold,
        70,
        "min percentage of live bytes on a page to enable fast evacuation in full GCs"
    );
    DEFINE_INT!(
        minor_ms_page_promotion_threshold,
        50,
        "min percentage of live bytes on a page to enable fast evacuation in MinorMS"
    );
    DEFINE_INT!(
        minor_ms_page_promotion_max_lab_threshold,
        30,
        "max percentage of labs out of a page to still be considered for page promotion"
    );
    DEFINE_UINT!(
        minor_ms_max_page_age,
        4,
        "max age for a page after which it is force promoted to old space"
    );
    DEFINE_UINT!(
        minor_ms_max_new_space_capacity_mb,
        72,
        "max new space capacity in MBs when using MinorMS. When pointer compression is disabled, twice the capacity is used."
    );
    DEFINE_BOOL!(
        trace_page_promotions,
        false,
        "trace page promotion decisions"
    );
    DEFINE_BOOL!(
        trace_pretenuring,
        false,
        "trace pretenuring decisions of HAllocate instructions"
    );
    DEFINE_BOOL!(
        trace_pretenuring_statistics,
        false,
        "trace allocation site pretenuring statistics"
    );
    DEFINE_BOOL!(track_field_types, true, "track field types");
    DEFINE_BOOL!(trace
