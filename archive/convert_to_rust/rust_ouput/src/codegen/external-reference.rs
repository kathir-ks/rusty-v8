// Converted from V8 C++ source files:
// Header: external-reference.h
// Implementation: external-reference.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod external_reference {
    use std::sync::Mutex;
    use std::sync::Arc;
    use std::ops::Deref;
    use std::ptr::null_mut;

    //use crate::v8::internal::Address;
    //use crate::v8::internal::Isolate;
    //use crate::v8::internal::LocalIsolate;
    //use crate::v8::internal::StatsCounter;

    pub struct ApiFunction {}
    pub struct CFunctionInfo {}

    pub struct V8_EXPORT_PRIVATE {}

    pub struct Simulator {}

    pub struct ExternalReference {
        raw_: usize,
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum Type {
        BUILTIN_CALL,
        BUILTIN_CALL_PAIR,
        BUILTIN_COMPARE_CALL,
        BUILTIN_FP_FP_CALL,
        BUILTIN_FP_CALL,
        BUILTIN_FP_INT_CALL,
        BUILTIN_FP_POINTER_CALL,
        BUILTIN_INT_FP_CALL,
        DIRECT_API_CALL,
        DIRECT_GETTER_CALL,
        FAST_C_CALL,
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum IsolateAddressId {
        kIsolateAddress,
        kJsLimitAddress,
        kCodePointerTableBaseAddress,
        // Add other IDs here
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum RuntimeFunctionId {
        kAbort,
        kAllocateBuffer,
        kDeallocateBuffer,
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum IsolateFieldId {
        kIsolateAddress,
        kHandleScopeImplementerAddress,
        kInterpreterEntryTrampoline,
        kInterpreterDispatchCounters,
        kInterpreterDispatchTableAddress,
        kDateCacheStamp,
        kStressDeoptCount,
        kForceSlowPath,
        kIsolateRoot,
        kAllocationSitesListAddress,
        kJsLimitAddress,
        kNoHeapWritesInterruptRequest,
        kRealJsLimitAddress,
        kHeapIsMarkingFlagAddress,
        kHeapIsMinorMarkingFlagAddress,
        kIsSharedSpaceIsolateFlagAddress,
        kNewSpaceAllocationTopAddress,
        kNewSpaceAllocationLimitAddress,
        kOldSpaceAllocationTopAddress,
        kOldSpaceAllocationLimitAddress,
        kArrayBufferMaxAllocationAddress,
        kHandleScopeLevelAddress,
        kHandleScopeNextAddress,
        kHandleScopeLimitAddress,
        kExceptionAddress,
        kPendingMessageAddress,
        kPromiseHookFlagsAddress,
        kPromiseHookAddress,
        kAsyncEventDelegateAddress,
        kDebugIsActiveAddress,
        kDebugHookOnFunctionCallAddress,
        kRuntimeFunctionTableAddress,
        kDebugSuspendedGeneratorAddress,
        kContextAddress,
        kRegExpStackLimitAddress,
        kRegExpStackMemoryTopAddress,
        kRegExpStackStackPointer,
        kRegExpStaticResultOffsetsVector,
        kThreadInWasmFlagAddressAddress,
        kExternalPointerTableAddress,
        kSharedExternalPointerTableAddressAddress,
        kTrustedPointerTableBaseAddress,
        kSharedTrustedPointerTableBaseAddress,
        kCodePointerTableBaseAddress,
        kNumIsolateFieldIds,
    }
    
    impl IsolateFieldId {
        pub const kNumIsolateFieldIds: usize = 50; // Approximate the number of enums
    }

    // Mock Address type for usize
    pub type Address = usize;

    // Placeholder types for V8 internal structs
    pub struct Isolate {}
    pub struct LocalIsolate {}
    pub struct StatsCounter {}
    pub struct SCTableReference {}
    pub struct HandleScopeImplementer {}
    pub struct SharedFunctionInfo {}
    pub struct Code {}
    pub struct BytecodeArray {}
    pub struct DebugInfo {}
    pub struct JSReceiver {}
    pub struct NameDictionary {}
    pub struct GlobalDictionary {}
    pub struct NameToIndexHashTable {}
    pub struct String {}
    pub struct ExternalOneByteString {}
    pub struct ExternalTwoByteString {}
    pub struct Map {}
    pub struct NativeContext {}
    pub struct HandleScope {}
    pub struct RegExpStack {}
    pub struct ExternalPointerHandle {}
    pub struct AbortReason {}
    pub struct CallApiCallbackMode {}
    pub struct FutexEmulation {}
    pub struct MemoryChunk {}
    pub struct Sandbox {}
    pub struct IsolateGroup {}
    pub struct Heap {}
    pub struct StackGuard {}
    pub struct MicrotaskQueue {}
    pub struct TaskQueue {}
    pub struct TracingFlags {}
    pub struct Smi {}
    pub struct RegExp {}
    pub struct RegExpResultVector {}
    pub struct StringForwardingTable {}
    pub struct unibrow {}
    pub struct JSArrayBuffer {}
    pub struct OrderedHashMap {}

    impl ExternalReference {
        pub fn address_of_pending_message(_local_isolate: *mut LocalIsolate) -> ExternalReference {
            ExternalReference { raw_: 0 } // Replace with a proper address if needed
        }

        pub fn new() -> Self {
            Self { raw_: 0 } // Or some default value
        }

        pub fn Create(table_ref: &SCTableReference) -> Self {
            ExternalReference { raw_: 0 } // Replace with actual logic
        }

        pub fn Create_stats_counter(counter: *mut StatsCounter) -> Self {
            ExternalReference { raw_: 0 } // Replace with actual logic
        }

        pub fn Create_api_function(ptr: *mut ApiFunction, type_: Type) -> Self {
            ExternalReference { raw_: 0 } // Replace with actual logic
        }

        pub fn Create_isolate(
            _isolate: *mut Isolate,
            _ptr: *mut ApiFunction,
            _type_: Type,
            _c_functions: *mut Address,
            _c_signatures: *const *const CFunctionInfo,
            _num_functions: u32,
        ) -> Self {
            ExternalReference { raw_: 0 } // Replace with actual logic
        }

        pub fn Create_runtime_function(f: usize) -> Self {
            ExternalReference { raw_: 0 } // Replace with actual logic
        }

        pub fn Create_runtime_function_id(_id: RuntimeFunctionId) -> Self {
            ExternalReference { raw_: 0 } // Replace with actual logic
        }

        pub fn Create_isolate_field_id(_id: IsolateFieldId) -> Self {
            ExternalReference { raw_: 0 } // Replace with actual logic
        }

        pub fn Create_address(address: Address, type_: Type) -> Self {
            ExternalReference { raw_: address }
        }

        pub fn search_string_raw<SubjectChar, PatternChar>() -> Self {
            ExternalReference { raw_: 0 } // Replace with actual logic
        }

        pub fn FromRawAddress(address: Address) -> Self {
            ExternalReference { raw_: address }
        }

        pub fn abort_with_reason() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn address_of_log_or_trace_osr() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn address_of_builtin_subclassing_flag() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn address_of_double_abs_constant() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn address_of_double_neg_constant() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn address_of_enable_experimental_regexp_engine() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn address_of_fp16_abs_constant() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn address_of_fp16_neg_constant() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn address_of_float_abs_constant() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn address_of_float_neg_constant() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn address_of_log10_offset_table() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn address_of_min_int() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn address_of_mock_arraybuffer_allocator_flag() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn address_of_one_half() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn address_of_runtime_stats_flag() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn address_of_shared_string_table_flag() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn address_of_the_hole_nan() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn address_of_uint32_bias() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn allocate_and_initialize_young_external_pointer_table_entry() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn baseline_pc_for_bytecode_offset() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn baseline_pc_for_next_executed_bytecode() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn bytecode_size_table_address() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn check_object_type() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn compute_integer_hash() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn compute_output_frames_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn copy_fast_number_jsarray_elements_to_typed_array() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn copy_typed_array_elements_slice() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn copy_typed_array_elements_to_typed_array() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn cpu_features() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn debug_break_at_entry_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn debug_get_coverage_info_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn delete_handle_scope_extensions() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn ephemeron_key_write_barrier_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn f64_acos_wrapper_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn f64_asin_wrapper_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn f64_mod_wrapper_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn get_date_field_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn get_or_create_hash_raw() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn gsab_byte_length() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn ieee754_fp64_to_fp16_raw_bits() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn ieee754_fp64_raw_bits_to_fp16_raw_bits_for_32bit_arch() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn ieee754_fp16_raw_bits_to_fp32_raw_bits() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn ieee754_acos_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn ieee754_acosh_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn ieee754_asin_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn ieee754_asinh_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn ieee754_atan_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn ieee754_atan2_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn ieee754_atanh_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn ieee754_cbrt_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn ieee754_cos_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn ieee754_cosh_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn ieee754_exp_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn ieee754_expm1_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn ieee754_log_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn ieee754_log10_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn ieee754_log1p_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn ieee754_log2_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn ieee754_pow_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn ieee754_sin_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn ieee754_sinh_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn ieee754_tan_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn ieee754_tanh_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn insert_remembered_set_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn invalidate_prototype_chains_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn invoke_accessor_getter_callback() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn invoke_function_callback_generic() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn invoke_function_callback_optimized() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn jsarray_array_join_concat_to_sequential_string() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn jsreceiver_create_identity_hash() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn libc_memchr_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn libc_memcpy_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn libc_memmove_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn libc_memset_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn relaxed_memcpy_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn relaxed_memmove_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn mod_two_doubles_operation() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn mutable_big_int_absolute_add_and_canonicalize_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn mutable_big_int_absolute_compare_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn mutable_big_int_absolute_sub_and_canonicalize_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn mutable_big_int_absolute_mul_and_canonicalize_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn mutable_big_int_absolute_div_and_canonicalize_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn mutable_big_int_absolute_mod_and_canonicalize_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn mutable_big_int_bitwise_and_pp_and_canonicalize_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn mutable_big_int_bitwise_and_nn_and_canonicalize_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn mutable_big_int_bitwise_and_pn_and_canonicalize_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn mutable_big_int_bitwise_or_pp_and_canonicalize_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn mutable_big_int_bitwise_or_nn_and_canonicalize_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn mutable_big_int_bitwise_or_pn_and_canonicalize_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn mutable_big_int_bitwise_xor_pp_and_canonicalize_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn mutable_big_int_bitwise_xor_nn_and_canonicalize_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn mutable_big_int_bitwise_xor_pn_and_canonicalize_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn mutable_big_int_left_shift_and_canonicalize_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn big_int_right_shift_result_length_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn mutable_big_int_right_shift_and_canonicalize_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn new_deoptimizer_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn orderedhashmap_gethash_raw() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn printf_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn refill_math_random() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn search_string_raw_one_one() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn search_string_raw_one_two() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn search_string_raw_two_one() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn search_string_raw_two_two() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn string_write_to_flat_one_byte() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn string_write_to_flat_two_byte() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn script_context_mutable_heap_number_flag() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn script_context_mutable_heap_int32_flag() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn additive_safe_int_feedback_flag() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn external_one_byte_string_get_chars() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn external_two_byte_string_get_chars() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn smi_lexicographic_compare_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn string_to_array_index_function() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn array_indexof_includes_smi_or_object() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn array_indexof_includes_double() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn has_unpaired_surrogate() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn replace_unpaired_surrogates() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn try_string_to_index_or_lookup_existing() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn string_from_forward_table() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn raw_hash_from_forward_table() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn name_dictionary_lookup_forwarded_string() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn name_dictionary_find_insertion_entry_forwarded_string() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn global_dictionary_lookup_forwarded_string() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn global_dictionary_find_insertion_entry_forwarded_string() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn name_to_index_hashtable_lookup_forwarded_string() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn name_to_index_hashtable_find_insertion_entry_forwarded_string() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_switch_stacks() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_return_switch() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_switch_to_the_central_stack() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_switch_from_the_central_stack() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_switch_to_the_central_stack_for_js() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_switch_from_the_central_stack_for_js() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_code_pointer_table() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_grow_stack() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_shrink_stack() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_load_old_fp() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f32_ceil() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f32_floor() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f32_nearest_int() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f32_trunc() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f64_ceil() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f64_floor() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f64_nearest_int() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f64_trunc() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_float32_to_int64() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_float32_to_uint64() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_float32_to_int64_sat() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_float32_to_uint64_sat() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_float64_pow() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_float64_to_int64() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_float64_to_uint64() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_float64_to_int64_sat() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_float64_to_uint64_sat() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_float16_to_float32() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_float32_to_float16() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_int64_div() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_int64_mod() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_int64_to_float32() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_int64_to_float64() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_uint64_div() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_uint64_mod() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_uint64_to_float32() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_uint64_to_float64() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_word32_ctz() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_word32_popcnt() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_word32_rol() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_word32_ror() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_word64_rol() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_word64_ror() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_word64_ctz() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_word64_popcnt() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f64x2_ceil() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f64x2_floor() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f64x2_trunc() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f64x2_nearest_int() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f32x4_ceil() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f32x4_floor() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f32x4_trunc() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f32x4_nearest_int() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f16x8_abs() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f16x8_neg() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f16x8_sqrt() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f16x8_ceil() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f16x8_floor() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f16x8_trunc() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f16x8_nearest_int() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f16x8_eq() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f16x8_ne() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f16x8_lt() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f16x8_le() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f16x8_add() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f16x8_sub() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f16x8_mul() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f16x8_div() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f16x8_min() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f16x8_max() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f16x8_pmin() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f16x8_pmax() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_i16x8_sconvert_f16x8() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_i16x8_uconvert_f16x8() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f16x8_sconvert_i16x8() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f16x8_uconvert_i16x8() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f32x4_promote_low_f16x8() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f16x8_demote_f32x4_zero() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f16x8_demote_f64x2_zero() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f16x8_qfma() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_f16x8_qfms() -> Self {
            ExternalReference { raw_: 0 }
        }

        pub fn wasm_
