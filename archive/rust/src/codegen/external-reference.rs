// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod external_reference {
    use std::fmt;
    //use std::os::raw::c_char;
    use std::hash::{Hash, Hasher};

    pub type Address = usize;
    pub const kNullAddress: Address = 0;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    impl Default for Type {
        fn default() -> Self {
            Type::BUILTIN_CALL
        }
    }

    macro_rules! external_reference_list_with_isolate {
        ($V:ident) => {
            $V(isolate_address, "isolate");
            $V(handle_scope_implementer_address, "Isolate::handle_scope_implementer_address");
            $V(address_of_interpreter_entry_trampoline_instruction_start, "Address of the InterpreterEntryTrampoline instruction start");
            $V(interpreter_dispatch_counters, "Interpreter::dispatch_counters");
            $V(interpreter_dispatch_table_address, "Interpreter::dispatch_table_address");
            $V(date_cache_stamp, "date_cache_stamp");
            $V(stress_deopt_count, "Isolate::stress_deopt_count_address()");
            $V(force_slow_path, "Isolate::force_slow_path_address()");
            $V(isolate_root, "Isolate::isolate_root()");
            $V(allocation_sites_list_address, "Heap::allocation_sites_list_address()");
            $V(address_of_jslimit, "StackGuard::address_of_jslimit()");
            $V(address_of_no_heap_write_interrupt_request, "StackGuard::address_of_interrupt_request(StackGuard::InterruptLevel::kNoHeapWrites)");
            $V(address_of_real_jslimit, "StackGuard::address_of_real_jslimit()");
            $V(heap_is_marking_flag_address, "heap_is_marking_flag_address");
            $V(heap_is_minor_marking_flag_address, "heap_is_minor_marking_flag_address");
            $V(is_shared_space_isolate_flag_address, "is_shared_space_isolate_flag_address");
            $V(new_space_allocation_top_address, "Heap::NewSpaceAllocationTopAddress()");
            $V(new_space_allocation_limit_address, "Heap::NewSpaceAllocationLimitAddress()");
            $V(old_space_allocation_top_address, "Heap::OldSpaceAllocationTopAddress");
            $V(old_space_allocation_limit_address, "Heap::OldSpaceAllocationLimitAddress");
            $V(array_buffer_max_allocation_address, "Heap::ArrayBufferMaxAllocationAddress");
            $V(handle_scope_level_address, "HandleScope::level");
            $V(handle_scope_next_address, "HandleScope::next");
            $V(handle_scope_limit_address, "HandleScope::limit");
            $V(exception_address, "Isolate::exception");
            $V(address_of_pending_message, "address_of_pending_message");
            $V(promise_hook_flags_address, "Isolate::promise_hook_flags_address()");
            $V(promise_hook_address, "Isolate::promise_hook_address()");
            $V(async_event_delegate_address, "Isolate::async_event_delegate_address()");
            $V(debug_is_active_address, "Debug::is_active_address()");
            $V(debug_hook_on_function_call_address, "Debug::hook_on_function_call_address()");
            $V(runtime_function_table_address, "Runtime::runtime_function_table_address()");
            $V(debug_suspended_generator_address, "Debug::step_suspended_generator_address()");
            $V(context_address, "Isolate::context_address()");
            $V(address_of_regexp_stack_limit_address, "RegExpStack::limit_address_address()");
            $V(address_of_regexp_stack_memory_top_address, "RegExpStack::memory_top_address_address()");
            $V(address_of_regexp_stack_stack_pointer, "RegExpStack::stack_pointer_address()");
            $V(address_of_regexp_static_result_offsets_vector, "Isolate::address_of_regexp_static_result_offsets_vector");
            $V(thread_in_wasm_flag_address_address, "Isolate::thread_in_wasm_flag_address_address");
            external_reference_list_with_isolate_sandbox!($V);
        };
    }

    macro_rules! external_reference_list_with_isolate_sandbox {
        ($V:ident) => {
            #[cfg(feature = "v8_enable_sandbox")]
            {
                $V(external_pointer_table_address, "Isolate::external_pointer_table_address()");
                $V(shared_external_pointer_table_address_address, "Isolate::shared_external_pointer_table_address_address()");
                $V(trusted_pointer_table_base_address, "Isolate::trusted_pointer_table_base_address()");
                $V(shared_trusted_pointer_table_base_address, "Isolate::shared_trusted_pointer_table_base_address()");
                $V(code_pointer_table_base_address, "Isolate::code_pointer_table_base_address()");
            }
            #[cfg(not(feature = "v8_enable_sandbox"))]
            {}
        };
    }

    macro_rules! external_reference_list {
        ($V:ident) => {
            $V(abort_with_reason, "abort_with_reason");
            $V(address_of_log_or_trace_osr, "v8_flags.log_or_trace_osr");
            $V(address_of_builtin_subclassing_flag, "v8_flags.builtin_subclassing");
            $V(address_of_double_abs_constant, "double_absolute_constant");
            $V(address_of_double_neg_constant, "double_negate_constant");
            $V(address_of_enable_experimental_regexp_engine, "address_of_enable_experimental_regexp_engine");
            $V(address_of_fp16_abs_constant, "fp16_absolute_constant");
            $V(address_of_fp16_neg_constant, "fp16_negate_constant");
            $V(address_of_float_abs_constant, "float_absolute_constant");
            $V(address_of_float_neg_constant, "float_negate_constant");
            $V(address_of_log10_offset_table, "log10_offset_table");
            $V(address_of_min_int, "LDoubleConstant::min_int");
            $V(address_of_mock_arraybuffer_allocator_flag, "v8_flags.mock_arraybuffer_allocator");
            $V(address_of_one_half, "LDoubleConstant::one_half");
            $V(address_of_runtime_stats_flag, "TracingFlags::runtime_stats");
            $V(address_of_shared_string_table_flag, "v8_flags.shared_string_table");
            $V(address_of_the_hole_nan, "the_hole_nan");
            $V(address_of_uint32_bias, "uint32_bias");
            $V(allocate_and_initialize_young_external_pointer_table_entry, "AllocateAndInitializeYoungExternalPointerTableEntry");
            $V(baseline_pc_for_bytecode_offset, "BaselinePCForBytecodeOffset");
            $V(baseline_pc_for_next_executed_bytecode, "BaselinePCForNextExecutedBytecode");
            $V(bytecode_size_table_address, "Bytecodes::bytecode_size_table_address");
            $V(check_object_type, "check_object_type");
            $V(compute_integer_hash, "ComputeSeededHash");
            $V(compute_output_frames_function, "Deoptimizer::ComputeOutputFrames()");
            $V(copy_fast_number_jsarray_elements_to_typed_array, "copy_fast_number_jsarray_elements_to_typed_array");
            $V(copy_typed_array_elements_slice, "copy_typed_array_elements_slice");
            $V(copy_typed_array_elements_to_typed_array, "copy_typed_array_elements_to_typed_array");
            $V(cpu_features, "cpu_features");
            $V(debug_break_at_entry_function, "DebugBreakAtEntry");
            $V(debug_get_coverage_info_function, "DebugGetCoverageInfo");
            $V(delete_handle_scope_extensions, "HandleScope::DeleteExtensions");
            $V(ephemeron_key_write_barrier_function, "Heap::EphemeronKeyWriteBarrierFromCode");
            $V(f64_acos_wrapper_function, "f64_acos_wrapper");
            $V(f64_asin_wrapper_function, "f64_asin_wrapper");
            $V(f64_mod_wrapper_function, "f64_mod_wrapper");
            $V(get_date_field_function, "JSDate::GetField");
            $V(get_or_create_hash_raw, "get_or_create_hash_raw");
            $V(gsab_byte_length, "GsabByteLength");
            $V(ieee754_fp64_to_fp16_raw_bits, "ieee754_fp64_to_fp16_raw_bits");
            $V(ieee754_fp64_raw_bits_to_fp16_raw_bits_for_32bit_arch, "ieee754_fp64_raw_bits_to_fp16_raw_bits_for_32bit_arch");
            $V(ieee754_fp16_raw_bits_to_fp32_raw_bits, "ieee754_fp16_raw_bits_to_fp32_raw_bits");
            $V(ieee754_acos_function, "base::ieee754::acos");
            $V(ieee754_acosh_function, "base::ieee754::acosh");
            $V(ieee754_asin_function, "base::ieee754::asin");
            $V(ieee754_asinh_function, "base::ieee754::asinh");
            $V(ieee754_atan_function, "base::ieee754::atan");
            $V(ieee754_atan2_function, "base::ieee754::atan2");
            $V(ieee754_atanh_function, "base::ieee754::atanh");
            $V(ieee754_cbrt_function, "base::ieee754::cbrt");
            $V(ieee754_cos_function, "base::ieee754::cos");
            $V(ieee754_cosh_function, "base::ieee754::cosh");
            $V(ieee754_exp_function, "base::ieee754::exp");
            $V(ieee754_expm1_function, "base::ieee754::expm1");
            $V(ieee754_log_function, "base::ieee754::log");
            $V(ieee754_log10_function, "base::ieee754::log10");
            $V(ieee754_log1p_function, "base::ieee754::log1p");
            $V(ieee754_log2_function, "base::ieee754::log2");
            $V(ieee754_pow_function, "math::pow");
            $V(ieee754_sin_function, "base::ieee754::sin");
            $V(ieee754_sinh_function, "base::ieee754::sinh");
            $V(ieee754_tan_function, "base::ieee754::tan");
            $V(ieee754_tanh_function, "base::ieee754::tanh");
            $V(insert_remembered_set_function, "Heap::InsertIntoRememberedSetFromCode");
            $V(invalidate_prototype_chains_function, "JSObject::InvalidatePrototypeChains()");
            $V(invoke_accessor_getter_callback, "InvokeAccessorGetterCallback");
            $V(invoke_function_callback_generic, "InvokeFunctionCallbackGeneric");
            $V(invoke_function_callback_optimized, "InvokeFunctionCallbackOptimized");
            $V(jsarray_array_join_concat_to_sequential_string, "jsarray_array_join_concat_to_sequential_string");
            $V(jsreceiver_create_identity_hash, "jsreceiver_create_identity_hash");
            $V(libc_memchr_function, "libc_memchr");
            $V(libc_memcpy_function, "libc_memcpy");
            $V(libc_memmove_function, "libc_memmove");
            $V(libc_memset_function, "libc_memset");
            $V(relaxed_memcpy_function, "relaxed_memcpy");
            $V(relaxed_memmove_function, "relaxed_memmove");
            $V(mod_two_doubles_operation, "mod_two_doubles");
            $V(mutable_big_int_absolute_add_and_canonicalize_function, "MutableBigInt_AbsoluteAddAndCanonicalize");
            $V(mutable_big_int_absolute_compare_function, "MutableBigInt_AbsoluteCompare");
            $V(mutable_big_int_absolute_sub_and_canonicalize_function, "MutableBigInt_AbsoluteSubAndCanonicalize");
            $V(mutable_big_int_absolute_mul_and_canonicalize_function, "MutableBigInt_AbsoluteMulAndCanonicalize");
            $V(mutable_big_int_absolute_div_and_canonicalize_function, "MutableBigInt_AbsoluteDivAndCanonicalize");
            $V(mutable_big_int_absolute_mod_and_canonicalize_function, "MutableBigInt_AbsoluteModAndCanonicalize");
            $V(mutable_big_int_bitwise_and_pp_and_canonicalize_function, "MutableBigInt_BitwiseAndPosPosAndCanonicalize");
            $V(mutable_big_int_bitwise_and_nn_and_canonicalize_function, "MutableBigInt_BitwiseAndNegNegAndCanonicalize");
            $V(mutable_big_int_bitwise_and_pn_and_canonicalize_function, "MutableBigInt_BitwiseAndPosNegAndCanonicalize");
            $V(mutable_big_int_bitwise_or_pp_and_canonicalize_function, "MutableBigInt_BitwiseOrPosPosAndCanonicalize");
            $V(mutable_big_int_bitwise_or_nn_and_canonicalize_function, "MutableBigInt_BitwiseOrNegNegAndCanonicalize");
            $V(mutable_big_int_bitwise_or_pn_and_canonicalize_function, "MutableBigInt_BitwiseOrPosNegAndCanonicalize");
            $V(mutable_big_int_bitwise_xor_pp_and_canonicalize_function, "MutableBigInt_BitwiseXorPosPosAndCanonicalize");
            $V(mutable_big_int_bitwise_xor_nn_and_canonicalize_function, "MutableBigInt_BitwiseXorNegNegAndCanonicalize");
            $V(mutable_big_int_bitwise_xor_pn_and_canonicalize_function, "MutableBigInt_BitwiseXorPosNegAndCanonicalize");
            $V(mutable_big_int_left_shift_and_canonicalize_function, "MutableBigInt_LeftShiftAndCanonicalize");
            $V(big_int_right_shift_result_length_function, "RightShiftResultLength");
            $V(mutable_big_int_right_shift_and_canonicalize_function, "MutableBigInt_RightShiftAndCanonicalize");
            $V(new_deoptimizer_function, "Deoptimizer::New()");
            $V(orderedhashmap_gethash_raw, "orderedhashmap_gethash_raw");
            $V(printf_function, "printf");
            $V(refill_math_random, "MathRandom::RefillCache");
            $V(search_string_raw_one_one, "search_string_raw_one_one");
            $V(search_string_raw_one_two, "search_string_raw_one_two");
            $V(search_string_raw_two_one, "search_string_raw_two_one");
            $V(search_string_raw_two_two, "search_string_raw_two_two");
            $V(string_write_to_flat_one_byte, "string_write_to_flat_one_byte");
            $V(string_write_to_flat_two_byte, "string_write_to_flat_two_byte");
            $V(script_context_mutable_heap_number_flag, "v8_flags.script_context_mutable_heap_number");
            $V(script_context_mutable_heap_int32_flag, "v8_flags.script_context_mutable_heap_int32");
            $V(additive_safe_int_feedback_flag, "v8_flags.additive_safe_int_feedback");
            $V(external_one_byte_string_get_chars, "external_one_byte_string_get_chars");
            $V(external_two_byte_string_get_chars, "external_two_byte_string_get_chars");
            $V(smi_lexicographic_compare_function, "smi_lexicographic_compare_function");
            $V(string_to_array_index_function, "String::ToArrayIndex");
            $V(array_indexof_includes_smi_or_object, "array_indexof_includes_smi_or_object");
            $V(array_indexof_includes_double, "array_indexof_includes_double");
            $V(has_unpaired_surrogate, "Utf16::HasUnpairedSurrogate");
            $V(replace_unpaired_surrogates, "Utf16::ReplaceUnpairedSurrogates");
            $V(try_string_to_index_or_lookup_existing, "try_string_to_index_or_lookup_existing");
            $V(string_from_forward_table, "string_from_forward_table");
            $V(raw_hash_from_forward_table, "raw_hash_from_forward_table");
            $V(name_dictionary_lookup_forwarded_string, "name_dictionary_lookup_forwarded_string");
            $V(name_dictionary_find_insertion_entry_forwarded_string, "name_dictionary_find_insertion_entry_forwarded_string");
            $V(global_dictionary_lookup_forwarded_string, "global_dictionary_lookup_forwarded_string");
            $V(global_dictionary_find_insertion_entry_forwarded_string, "global_dictionary_find_insertion_entry_forwarded_string");
            $V(name_to_index_hashtable_lookup_forwarded_string, "name_to_index_hashtable_lookup_forwarded_string");
            $V(name_to_index_hashtable_find_insertion_entry_forwarded_string, "name_to_index_hashtable_find_insertion_entry_forwarded_string");
            if_wasm!($V, wasm_switch_stacks, "wasm_switch_stacks");
            if_wasm!($V, wasm_return_switch, "wasm_return_switch");
            if_wasm!($V, wasm_switch_to_the_central_stack, "wasm::switch_to_the_central_stack");
            if_wasm!($V, wasm_switch_from_the_central_stack, "wasm::switch_from_the_central_stack");
            if_wasm!($V, wasm_switch_to_the_central_stack_for_js, "wasm::switch_to_the_central_stack_for_js");
            if_wasm!($V, wasm_switch_from_the_central_stack_for_js, "wasm::switch_from_the_central_stack_for_js");
            if_wasm!($V, wasm_code_pointer_table, "GetProcessWideWasmCodePointerTable()");
            if_wasm!($V, wasm_grow_stack, "wasm::grow_stack");
            if_wasm!($V, wasm_shrink_stack, "wasm::shrink_stack");
            if_wasm!($V, wasm_load_old_fp, "wasm::load_old_fp");
            if_wasm!($V, wasm_f32_ceil, "wasm::f32_ceil_wrapper");
            if_wasm!($V, wasm_f32_floor, "wasm::f32_floor_wrapper");
            if_wasm!($V, wasm_f32_nearest_int, "wasm::f32_nearest_int_wrapper");
            if_wasm!($V, wasm_f32_trunc, "wasm::f32_trunc_wrapper");
            if_wasm!($V, wasm_f64_ceil, "wasm::f64_ceil_wrapper");
            if_wasm!($V, wasm_f64_floor, "wasm::f64_floor_wrapper");
            if_wasm!($V, wasm_f64_nearest_int, "wasm::f64_nearest_int_wrapper");
            if_wasm!($V, wasm_f64_trunc, "wasm::f64_trunc_wrapper");
            if_wasm!($V, wasm_float32_to_int64, "wasm::float32_to_int64_wrapper");
            if_wasm!($V, wasm_float32_to_uint64, "wasm::float32_to_uint64_wrapper");
            if_wasm!($V, wasm_float32_to_int64_sat, "wasm::float32_to_int64_sat_wrapper");
            if_wasm!($V, wasm_float32_to_uint64_sat, "wasm::float32_to_uint64_sat_wrapper");
            if_wasm!($V, wasm_float64_pow, "wasm::float64_pow");
            if_wasm!($V, wasm_float64_to_int64, "wasm::float64_to_int64_wrapper");
            if_wasm!($V, wasm_float64_to_uint64, "wasm::float64_to_uint64_wrapper");
            if_wasm!($V, wasm_float64_to_int64_sat, "wasm::float64_to_int64_sat_wrapper");
            if_wasm!($V, wasm_float64_to_uint64_sat, "wasm::float64_to_uint64_sat_wrapper");
            if_wasm!($V, wasm_float16_to_float32, "wasm::float16_to_float32_wrapper");
            if_wasm!($V, wasm_float32_to_float16, "wasm::float32_to_float16_wrapper");
            if_wasm!($V, wasm_int64_div, "wasm::int64_div");
            if_wasm!($V, wasm_int64_mod, "wasm::int64_mod");
            if_wasm!($V, wasm_int64_to_float32, "wasm::int64_to_float32_wrapper");
            if_wasm!($V, wasm_int64_to_float64, "wasm::int64_to_float64_wrapper");
            if_wasm!($V, wasm_uint64_div, "wasm::uint64_div");
            if_wasm!($V, wasm_uint64_mod, "wasm::uint64_mod");
            if_wasm!($V, wasm_uint64_to_float32, "wasm::uint64_to_float32_wrapper");
            if_wasm!($V, wasm_uint64_to_float64, "wasm::uint64_to_float64_wrapper");
            if_wasm!($V, wasm_word32_ctz, "wasm::word32_ctz");
            if_wasm!($V, wasm_word32_popcnt, "wasm::word32_popcnt");
            if_wasm!($V, wasm_word32_rol, "wasm::word32_rol");
            if_wasm!($V, wasm_word32_ror, "wasm::word32_ror");
            if_wasm!($V, wasm_word64_rol, "wasm::word64_rol");
            if_wasm!($V, wasm_word64_ror, "wasm::word64_ror");
            if_wasm!($V, wasm_word64_ctz, "wasm::word64_ctz");
            if_wasm!($V, wasm_word64_popcnt, "wasm::word64_popcnt");
            if_wasm!($V, wasm_f64x2_ceil, "wasm::f64x2_ceil_wrapper");
            if_wasm!($V, wasm_f64x2_floor, "wasm::f64x2_floor_wrapper");
            if_wasm!($V, wasm_f64x2_trunc, "wasm::f64x2_trunc_wrapper");
            if_wasm!($V, wasm_f64x2_nearest_int, "wasm::f64x2_nearest_int_wrapper");
            if_wasm!($V, wasm_f32x4_ceil, "wasm::f32x4_ceil_wrapper");
            if_wasm!($V, wasm_f32x4_floor, "wasm::f32x4_floor_wrapper");
            if_wasm!($V, wasm_f32x4_trunc, "wasm::f32x4_trunc_wrapper");
            if_wasm!($V, wasm_f32x4_nearest_int, "wasm::f32x4_nearest_int_wrapper");
            if_wasm!($V, wasm_f16x8_abs, "wasm::f16x8_abs_wrapper");
            if_wasm!($V, wasm_f16x8_neg, "wasm::f16x8_neg_wrapper");
            if_wasm!($V, wasm_f16x8_sqrt, "wasm::f16x8_sqrt_wrapper");
            if_wasm!($V, wasm_f16x8_ceil, "wasm::f16x8_ceil_wrapper");
            if_wasm!($V, wasm_f16x8_floor, "wasm::f16x8_floor_wrapper");
            if_wasm!($V, wasm_f16x8_trunc, "wasm::f16x8_trunc_wrapper");
            if_wasm!($V, wasm_f16x8_nearest_int, "wasm::f16x8_nearest_int_wrapper");
            if_wasm!($V, wasm_f16x8_eq, "wasm::f16x8_eq_wrapper");
            if_wasm!($V, wasm_f16x8_ne, "wasm::f16x8_ne_wrapper");
            if_wasm!($V, wasm_f16x8_lt, "wasm::f16x8_lt_wrapper");
            if_wasm!($V, wasm_f16x8_le, "wasm::f16x8_le_wrapper");
            if_wasm!($V, wasm_f16x8_add, "wasm::f16x8_add_wrapper");
            if_wasm!($V, wasm_f16x8_sub, "wasm::f16x8_sub_wrapper");
            if_wasm!($V, wasm_f16x8_mul, "wasm::f16x8_mul_wrapper");
            if_wasm!($V, wasm_f16x8_div, "wasm::f16x8_div_wrapper");
            if_wasm!($V, wasm_f16x8_min, "wasm::f16x8_min_wrapper");
            if_wasm!($V, wasm_f16x8_max, "wasm::f16x8_max_wrapper");
            if_wasm!($V, wasm_f16x8_pmin, "wasm::f16x8_pmin_wrapper");
            if_wasm!($V, wasm_f16x8_pmax, "wasm::f16x8_pmax_wrapper");
            if_wasm!($V, wasm_i16x8_sconvert_f16x8, "wasm::i16x8_sconvert_f16x8_wrapper");
            if_wasm!($V, wasm_i16x8_uconvert_f16x8, "wasm::i16x8_uconvert_f16x8_wrapper");
            if_wasm!($V, wasm_f16x8_sconvert_i16x8, "wasm::f16x8_sconvert_i16x8_wrapper");
            if_wasm!($V, wasm_f16x8_uconvert_i16x8, "wasm::f16x8_uconvert_i16x8_wrapper");
            if_wasm!($V, wasm_f32x4_promote_low_f16x8, "wasm::f32x4_promote_low_f16x8_wrapper");
            if_wasm!($V, wasm_f16x8_demote_f32x4_zero, "wasm::f16x8_demote_f32x4_zero_wrapper");
            if_wasm!($V, wasm_f16x8_demote_f64x2_zero, "wasm::f16x8_demote_f64x2_zero_wrapper");
            if_wasm!($V, wasm_f16x8_qfma, "wasm::f16x8_qfma_wrapper");
            if_wasm!($V, wasm_f16x8_qfms, "wasm::f16x8_qfms_wrapper");
            if_wasm!($V, wasm_memory_init, "wasm::memory_init");
            if_wasm!($V, wasm_memory_copy, "wasm::memory_copy");
            if_wasm!($V, wasm_memory_fill, "wasm::memory_fill");
            if_wasm!($V, wasm_array_copy, "wasm::array_copy");
            if_wasm!($V, wasm_array_fill, "wasm::array_fill");
            if_wasm!($V, wasm_string_to_f64, "wasm_string_to_f64");
            if_wasm!($V, wasm_atomic_notify, "wasm_atomic_notify");
            if_wasm!($V, wasm_WebAssemblyCompile, "wasm::WebAssemblyCompile");
            if_wasm!($V, wasm_WebAssemblyException, "wasm::WebAssemblyException");
            if_wasm!($V, wasm_WebAssemblyExceptionGetArg, "wasm::WebAssemblyExceptionGetArg");
            if_wasm!($V, wasm_WebAssemblyExceptionIs, "wasm::WebAssemblyExceptionIs");
            if_wasm!($V, wasm_WebAssemblyGlobal, "wasm::WebAssemblyGlobal");
            if_wasm!($V, wasm_WebAssemblyGlobalGetValue, "wasm::WebAssemblyGlobalGetValue");
            if_wasm!($V, wasm_WebAssemblyGlobalSetValue, "wasm::WebAssemblyGlobalSetValue");
            if_wasm!($V, wasm_WebAssemblyGlobalValueOf, "wasm::WebAssemblyGlobalValueOf");
            if_wasm!($V, wasm_WebAssemblyInstance, "wasm::WebAssemblyInstance");
            if_wasm!($V, wasm_WebAssemblyInstanceGetExports, "wasm::WebAssemblyInstanceGetExports");
            if_wasm!($V, wasm_WebAssemblyInstantiate, "wasm::WebAssemblyInstantiate");
            if_wasm!($V, wasm_WebAssemblyMemory, "wasm