// Converted from V8 C++ source files:
// Header: runtime.h
// Implementation: runtime.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub struct CustomMatcherHashMap {}
}
pub mod execution {
    pub struct Isolate {}
}
pub mod runtime {
    pub struct ObjectPair {}
}
pub mod strings {
    pub struct StringHasher {}
}

use std::sync::Once;

pub struct AllStatic {}

pub mod v8 {
    pub mod internal {
        use crate::execution::Isolate;
        use crate::runtime::ObjectPair;
        use crate::strings::StringHasher;
        use std::ffi::CString;
        use std::os::raw::c_char;
        use std::sync::Once;
        use std::{
            borrow::Cow,
            fmt,
            ops::{BitAnd, BitOr},
        };

        pub struct JSReceiver {}
        pub struct JSAny {}
        pub struct String {}
        pub struct Object {}
        pub struct Heap {}
        pub struct Context {}
        pub struct JSObject {}
        pub struct WasmTableObject {}
        pub struct Code {}
        pub struct Expression {}
        pub struct Operand {}
        pub struct Simulator {}
        pub struct EphemeronHashTable {}
        pub struct Label {}
        pub struct DirectHandle<T> {
            _phantom: std::marker::PhantomData<T>,
        }
        impl<T> DirectHandle<T> {
            pub fn new() -> Self {
                DirectHandle {
                    _phantom: std::marker::PhantomData,
                }
            }
        }
        impl<T> Clone for DirectHandle<T> {
            fn clone(&self) -> Self {
                DirectHandle::new()
            }
        }
        impl<T> Copy for DirectHandle<T> {}
        pub struct Tagged<T> {
            _phantom: std::marker::PhantomData<T>,
        }
        impl<T> Tagged<T> {
            pub fn new() -> Self {
                Tagged {
                    _phantom: std::marker::PhantomData,
                }
            }
        }

        pub enum LanguageMode {}
        pub enum StoreOrigin {}
        pub enum ShouldThrow {}
        pub enum FPUControlRegister {}
        pub enum Condition {}
        pub enum AbortReason {}
        pub enum ValueType {}
        pub enum VisitResult {}
        pub enum Bytecode {}
        pub enum OperandScale {}
        pub enum AsmType {}
        pub enum AsmType {}
        pub enum IrregexpImplementation {}
        pub enum JSThreadState {}
        pub enum HeapObject {}
        pub enum TaskPriority {}
        pub enum MemoryRepresentation {}

        pub struct InstructionOperand {}
        pub struct OpIndex {}
        pub struct Register {}
        pub struct Immediate {}
        pub struct Zone {}
        pub struct UnoptimizedCompileFlags {}
        pub struct Heap {}
        pub struct LocalHeap {}
        pub struct FPUCRegister {}
        pub struct AssemblerBase {}
        pub struct Exceptions {}
        pub struct Simulator {}
        pub struct WasmArrayObject {}

        pub mod base {
            pub struct CustomMatcherHashMap {}
        }
        pub mod execution {
            pub struct Isolate {}
        }
        pub mod runtime {
            pub struct ObjectPair {}
        }
        pub mod strings {
            pub struct StringHasher {}
        }

        // Define macros as functions
        pub fn array_includes_slow(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn array_index_of(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn array_is_array(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn array_species_constructor(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn grow_array_elements(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn is_array(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn new_array(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn normalize_elements(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn transition_elements_kind(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn transition_elements_kind_with_kind(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        // Atomics
        }
        pub fn atomics_load64(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn atomics_store64(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn atomics_add(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn atomics_and(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn atomics_compare_exchange(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn atomics_exchange(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn atomics_num_waiters_for_testing(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn atomics_num_unresolved_async_promises_for_testing(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn atomics_or(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn atomics_sub(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn atomics_xor(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn set_allow_atomics_wait(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn atomics_load_shared_struct_or_array(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn atomics_store_shared_struct_or_array(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn atomics_exchange_shared_struct_or_array(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn atomics_compare_exchange_shared_struct_or_array(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn atomics_synchronization_primitive_num_waiters_for_testing(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn atomics_sychronization_num_async_waiters_in_isolate_for_testing(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        //BigInt
        }
        pub fn big_int_compare_to_number(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn big_int_compare_to_string(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn big_int_equal_to_big_int(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn big_int_equal_to_number(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn big_int_equal_to_string(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn big_int_exponentiate(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn big_int_max_length_bits(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn big_int_to_number(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn big_int_unary_op(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn to_big_int(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn to_big_int_convert_number(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        //Classes
        }
        pub fn throw_constructor_non_callable_error(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn throw_not_super_constructor(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn throw_static_prototype_error(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn throw_super_already_called_error(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn throw_super_not_called(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn throw_unsupported_super_error(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn define_class(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn load_from_super(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn load_keyed_from_super(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn store_keyed_to_super(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn store_to_super(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        //Collections
        }
        pub fn map_grow(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn map_shrink(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn ordered_hash_set_grow(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn set_grow(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn set_shrink(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn ordered_hash_set_shrink(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn the_hole(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn weak_collection_delete(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn weak_collection_set(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn ordered_hash_map_grow(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        //Compiler
        }
        pub fn compile_optimized_osr(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn compile_optimized_osr_from_maglev(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn compile_optimized_osr_from_maglev_inlined(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn log_or_trace_optimized_osr_entry(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn compile_lazy(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn compile_baseline(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn install_baseline_code(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn install_sfi_code(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn instantiate_asm_js(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn notify_deoptimized(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn observe_node(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn resolve_possibly_direct_eval(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn verify_type(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn check_turboshaft_type_of(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        //Tiering
        }
        pub fn function_log_next_execution(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn optimize_maglev_eager(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn start_maglev_optimize_job(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn optimize_turbofan_eager(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn start_turbofan_optimize_job(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn mark_lazy_deoptimized(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        //HealOptimizedCodeSlot
        }
        pub fn heal_optimized_code_slot(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn compile_optimized(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        //Date
        pub fn date_current_time(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        //Debug
        pub fn clear_stepping(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn collect_garbage(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn debug_async_function_suspended(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn debug_break_at_entry(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn debug_collect_coverage(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn debug_get_loaded_script_ids(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn debug_on_function_call(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn debug_prepare_step_in_suspended_generator(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn debug_promise_then(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn debug_toggle_block_coverage(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn debug_toggle_precise_coverage(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn function_get_inferred_name(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn get_break_locations(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn get_generator_scope_count(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn get_generator_scope_details(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn handle_debugger_statement(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn is_break_on_exception(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn live_edit_patch_script(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn profile_create_snapshot_data_blob(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn schedule_break(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn script_location_from_line2(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn set_generator_scope_variable_value(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn inc_block_counter(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        //ForIn
        }
        pub fn for_in_enumerate(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn for_in_has_property(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        //Trace
        }
        pub fn trace_unoptimized_bytecode_entry(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn trace_unoptimized_bytecode_exit(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        //TraceFeedback
        }
        pub fn trace_update_feedback(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        //Function
        }
        pub fn call(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn function_get_script_source(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn function_get_script_id(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn function_get_script_source_position(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn function_get_source_code(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn function_is_api_function(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        //Generator
        }
        pub fn async_function_await(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn async_function_enter(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn async_function_reject(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn async_function_resolve(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn async_generator_await(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn async_generator_reject(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn async_generator_resolve(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn async_generator_yield_with_await(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn create_js_generator_object(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn generator_close(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn generator_get_function(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn generator_get_resume_mode(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        //Intl
        }
        pub fn format_list(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn format_list_to_parts(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn string_to_lower_case_intl(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn string_to_locale_lower_case(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn string_to_upper_case_intl(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        //Internal
        }
        pub fn throw(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn throw_apply_non_function(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn throw_called_non_callable(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn throw_constructed_non_constructable(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn throw_constructor_returned_non_object(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn throw_invalid_string_length(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn throw_invalid_typed_array_alignment(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn throw_iterator_error(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn throw_spread_arg_error(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn throw_iterator_result_not_an_object(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn throw_no_access(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn throw_not_constructor(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn throw_pattern_assignment_non_coercible(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn throw_range_error(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn throw_reference_error(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn throw_accessed_uninitialized_variable(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn throw_stack_overflow(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn throw_symbol_async_iterator_invalid(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn throw_symbol_iterator_invalid(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn throw_throw_method_missing(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn throw_type_error(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn throw_type_error_if_strict(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn re_throw(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn re_throw_with_message(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        //InternalNoThrow
        }
        pub fn access_check(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn allocate_byte_array(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn allocate_in_young_generation(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn allocate_in_old_generation(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn allow_dynamic_function(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn create_async_from_sync_iterator(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> Address {
            Address {}
        }
        pub fn create_list_from_array_like(args_length: i32, args_object: *
