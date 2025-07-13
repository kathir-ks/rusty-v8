// Converted from V8 C++ source files:
// Header: interface-descriptors-x64-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod interface_descriptors_x64_inl {
use crate::codegen::x64::assembler_x64::V8;
use crate::codegen::interface_descriptors::*;
use crate::codegen::x64::register_x64::*;
use crate::codegen::x64::assembler_x64::*;
use crate::codegen::x64::reglist_x64::*;

  pub const fn default_register_array() -> RegisterArray {
    RegisterArray {
      r0: rax,
      r1: rbx,
      r2: rcx,
      r3: rdx,
      r4: rdi,
      r5: Register { code: 0 },
      r6: Register { code: 0 },
      r7: Register { code: 0 },
      r8: Register { code: 0 },
      r9: Register { code: 0 },
      r10: Register { code: 0 },
      r11: Register { code: 0 },
      r12: Register { code: 0 },
      r13: Register { code: 0 },
      r14: Register { code: 0 },
      r15: Register { code: 0 },
    }
  }

  pub const fn default_double_register_array() -> DoubleRegisterArray {
    DoubleRegisterArray {
      d0: xmm0,
      d1: xmm1,
      d2: xmm2,
      d3: xmm3,
      d4: xmm4,
      d5: xmm5,
      d6: xmm6,
      d7: DoubleRegister { code: 0 },
      d8: DoubleRegister { code: 0 },
      d9: DoubleRegister { code: 0 },
      d10: DoubleRegister { code: 0 },
      d11: DoubleRegister { code: 0 },
      d12: DoubleRegister { code: 0 },
      d13: DoubleRegister { code: 0 },
      d14: DoubleRegister { code: 0 },
      d15: DoubleRegister { code: 0 },
    }
  }

  pub const fn default_return_register_array() -> RegisterArray {
    RegisterArray {
      r0: kReturnRegister0,
      r1: kReturnRegister1,
      r2: kReturnRegister2,
      r3: Register { code: 0 },
      r4: Register { code: 0 },
      r5: Register { code: 0 },
      r6: Register { code: 0 },
      r7: Register { code: 0 },
      r8: Register { code: 0 },
      r9: Register { code: 0 },
      r10: Register { code: 0 },
      r11: Register { code: 0 },
      r12: Register { code: 0 },
      r13: Register { code: 0 },
      r14: Register { code: 0 },
      r15: Register { code: 0 },
    }
  }

  pub const fn default_return_double_register_array() -> DoubleRegisterArray {
    DoubleRegisterArray {
      d0: kFPReturnRegister0,
      d1: no_dreg,
      d2: no_dreg,
      d3: DoubleRegister { code: 0 },
      d4: DoubleRegister { code: 0 },
      d5: DoubleRegister { code: 0 },
      d6: DoubleRegister { code: 0 },
      d7: DoubleRegister { code: 0 },
      d8: DoubleRegister { code: 0 },
      d9: DoubleRegister { code: 0 },
      d10: DoubleRegister { code: 0 },
      d11: DoubleRegister { code: 0 },
      d12: DoubleRegister { code: 0 },
      d13: DoubleRegister { code: 0 },
      d14: DoubleRegister { code: 0 },
      d15: DoubleRegister { code: 0 },
    }
  }

  pub fn verify_argument_register_count<T: StaticCallInterfaceDescriptorTrait>(
    data: &CallInterfaceDescriptorData,
    nof_expected_args: i32,
  ) {
    let allocatable_regs = data.allocatable_registers();
    if nof_expected_args >= 1 {
      assert!(allocatable_regs.has(&k_c_arg_regs[0]));
    }
    if nof_expected_args >= 2 {
      assert!(allocatable_regs.has(&k_c_arg_regs[1]));
    }
    if nof_expected_args >= 3 {
      assert!(allocatable_regs.has(&k_c_arg_regs[2]));
    }
    if nof_expected_args >= 4 {
      assert!(allocatable_regs.has(&k_c_arg_regs[3]));
    }
  }

  pub const fn write_barrier_registers() -> RegisterArray {
    RegisterArray {
      r0: rdi,
      r1: rbx,
      r2: rdx,
      r3: rcx,
      r4: rax,
      r5: rsi,
      r6: Register { code: 0 },
      r7: Register { code: 0 },
      r8: Register { code: 0 },
      r9: Register { code: 0 },
      r10: Register { code: 0 },
      r11: Register { code: 0 },
      r12: Register { code: 0 },
      r13: Register { code: 0 },
      r14: Register { code: 0 },
      r15: Register { code: 0 },
    }
  }

  pub const fn tsan_store_registers() -> RegisterArray {
    RegisterArray {
      r0: k_c_arg_regs[0],
      r1: k_c_arg_regs[1],
      r2: kReturnRegister0,
      r3: Register { code: 0 },
      r4: Register { code: 0 },
      r5: Register { code: 0 },
      r6: Register { code: 0 },
      r7: Register { code: 0 },
      r8: Register { code: 0 },
      r9: Register { code: 0 },
      r10: Register { code: 0 },
      r11: Register { code: 0 },
      r12: Register { code: 0 },
      r13: Register { code: 0 },
      r14: Register { code: 0 },
      r15: Register { code: 0 },
    }
  }

  pub const fn tsan_load_registers() -> RegisterArray {
    RegisterArray {
      r0: k_c_arg_regs[0],
      r1: kReturnRegister0,
      r2: Register { code: 0 },
      r3: Register { code: 0 },
      r4: Register { code: 0 },
      r5: Register { code: 0 },
      r6: Register { code: 0 },
      r7: Register { code: 0 },
      r8: Register { code: 0 },
      r9: Register { code: 0 },
      r10: Register { code: 0 },
      r11: Register { code: 0 },
      r12: Register { code: 0 },
      r13: Register { code: 0 },
      r14: Register { code: 0 },
      r15: Register { code: 0 },
    }
  }

  pub const fn load_receiver_register() -> Register {
    rdx
  }

  pub const fn load_name_register() -> Register {
    rcx
  }

  pub const fn load_slot_register() -> Register {
    rax
  }

  pub const fn load_with_vector_vector_register() -> Register {
    rbx
  }

  pub const fn keyed_load_baseline_receiver_register() -> Register {
    rdx
  }

  pub const fn keyed_load_baseline_name_register() -> Register {
    kInterpreterAccumulatorRegister
  }

  pub const fn keyed_load_baseline_slot_register() -> Register {
    rcx
  }

  pub const fn keyed_load_with_vector_vector_register() -> Register {
    rbx
  }

  pub const fn enumerated_keyed_load_baseline_enum_index_register() -> Register {
    rdi
  }

  pub const fn enumerated_keyed_load_baseline_cache_type_register() -> Register {
    r8
  }

  pub const fn enumerated_keyed_load_baseline_slot_register() -> Register {
    rcx
  }

  pub const fn keyed_has_ic_baseline_receiver_register() -> Register {
    kInterpreterAccumulatorRegister
  }

  pub const fn keyed_has_ic_baseline_name_register() -> Register {
    rdx
  }

  pub const fn keyed_has_ic_baseline_slot_register() -> Register {
    rcx
  }

  pub const fn keyed_has_ic_with_vector_vector_register() -> Register {
    rbx
  }

  pub const fn load_with_receiver_and_vector_lookup_start_object_register() -> Register {
    rdi
  }

  pub const fn store_receiver_register() -> Register {
    rdx
  }

  pub const fn store_name_register() -> Register {
    rcx
  }

  pub const fn store_value_register() -> Register {
    rax
  }

  pub const fn store_slot_register() -> Register {
    rdi
  }

  pub const fn store_with_vector_vector_register() -> Register {
    rbx
  }

  pub const fn define_keyed_own_flags_register() -> Register {
    r11
  }

  pub const fn store_transition_map_register() -> Register {
    r11
  }

  pub const fn api_getter_holder_register() -> Register {
    rcx
  }

  pub const fn api_getter_callback_register() -> Register {
    rbx
  }

  pub const fn grow_array_elements_object_register() -> Register {
    rax
  }

  pub const fn grow_array_elements_key_register() -> Register {
    rbx
  }

  pub const fn baseline_leave_frame_params_size_register() -> Register {
    rbx
  }

  pub const fn baseline_leave_frame_weight_register() -> Register {
    rcx
  }

  pub const fn maglev_optimize_code_or_tail_call_optimized_code_slot_flags_register() -> Register {
    r8
  }

  pub const fn maglev_optimize_code_or_tail_call_optimized_code_slot_feedback_vector_register() -> Register {
    r9
  }

  pub const fn maglev_optimize_code_or_tail_call_optimized_code_slot_temporary_register() -> Register {
    r11
  }

  pub const fn type_conversion_argument_register() -> Register {
    rax
  }

  pub const fn typeof_registers() -> RegisterArray {
    RegisterArray {
      r0: rax,
      r1: Register { code: 0 },
      r2: Register { code: 0 },
      r3: Register { code: 0 },
      r4: Register { code: 0 },
      r5: Register { code: 0 },
      r6: Register { code: 0 },
      r7: Register { code: 0 },
      r8: Register { code: 0 },
      r9: Register { code: 0 },
      r10: Register { code: 0 },
      r11: Register { code: 0 },
      r12: Register { code: 0 },
      r13: Register { code: 0 },
      r14: Register { code: 0 },
      r15: Register { code: 0 },
    }
  }

  pub const fn call_trampoline_registers() -> RegisterArray {
    RegisterArray {
      r0: rdi,
      r1: rax,
      r2: Register { code: 0 },
      r3: Register { code: 0 },
      r4: Register { code: 0 },
      r5: Register { code: 0 },
      r6: Register { code: 0 },
      r7: Register { code: 0 },
      r8: Register { code: 0 },
      r9: Register { code: 0 },
      r10: Register { code: 0 },
      r11: Register { code: 0 },
      r12: Register { code: 0 },
      r13: Register { code: 0 },
      r14: Register { code: 0 },
      r15: Register { code: 0 },
    }
  }

  pub const fn copy_data_properties_with_excluded_properties_registers() -> RegisterArray {
    RegisterArray {
      r0: rdi,
      r1: rax,
      r2: Register { code: 0 },
      r3: Register { code: 0 },
      r4: Register { code: 0 },
      r5: Register { code: 0 },
      r6: Register { code: 0 },
      r7: Register { code: 0 },
      r8: Register { code: 0 },
      r9: Register { code: 0 },
      r10: Register { code: 0 },
      r11: Register { code: 0 },
      r12: Register { code: 0 },
      r13: Register { code: 0 },
      r14: Register { code: 0 },
      r15: Register { code: 0 },
    }
  }

  pub const fn copy_data_properties_with_excluded_properties_on_stack_registers() -> RegisterArray {
    RegisterArray {
      r0: rdi,
      r1: rax,
      r2: rcx,
      r3: Register { code: 0 },
      r4: Register { code: 0 },
      r5: Register { code: 0 },
      r6: Register { code: 0 },
      r7: Register { code: 0 },
      r8: Register { code: 0 },
      r9: Register { code: 0 },
      r10: Register { code: 0 },
      r11: Register { code: 0 },
      r12: Register { code: 0 },
      r13: Register { code: 0 },
      r14: Register { code: 0 },
      r15: Register { code: 0 },
    }
  }

  pub const fn call_varargs_registers() -> RegisterArray {
    RegisterArray {
      r0: rdi,
      r1: rax,
      r2: rcx,
      r3: rbx,
      r4: Register { code: 0 },
      r5: Register { code: 0 },
      r6: Register { code: 0 },
      r7: Register { code: 0 },
      r8: Register { code: 0 },
      r9: Register { code: 0 },
      r10: Register { code: 0 },
      r11: Register { code: 0 },
      r12: Register { code: 0 },
      r13: Register { code: 0 },
      r14: Register { code: 0 },
      r15: Register { code: 0 },
    }
  }

  pub const fn call_forward_varargs_registers() -> RegisterArray {
    RegisterArray {
      r0: rdi,
      r1: rax,
      r2: rcx,
      r3: Register { code: 0 },
      r4: Register { code: 0 },
      r5: Register { code: 0 },
      r6: Register { code: 0 },
      r7: Register { code: 0 },
      r8: Register { code: 0 },
      r9: Register { code: 0 },
      r10: Register { code: 0 },
      r11: Register { code: 0 },
      r12: Register { code: 0 },
      r13: Register { code: 0 },
      r14: Register { code: 0 },
      r15: Register { code: 0 },
    }
  }

  pub const fn call_function_template_registers() -> RegisterArray {
    RegisterArray {
      r0: rdx,
      r1: rcx,
      r2: Register { code: 0 },
      r3: Register { code: 0 },
      r4: Register { code: 0 },
      r5: Register { code: 0 },
      r6: Register { code: 0 },
      r7: Register { code: 0 },
      r8: Register { code: 0 },
      r9: Register { code: 0 },
      r10: Register { code: 0 },
      r11: Register { code: 0 },
      r12: Register { code: 0 },
      r13: Register { code: 0 },
      r14: Register { code: 0 },
      r15: Register { code: 0 },
    }
  }

  pub const fn call_function_template_generic_registers() -> RegisterArray {
    RegisterArray {
      r0: rdx,
      r1: rcx,
      r2: rdi,
      r3: Register { code: 0 },
      r4: Register { code: 0 },
      r5: Register { code: 0 },
      r6: Register { code: 0 },
      r7: Register { code: 0 },
      r8: Register { code: 0 },
      r9: Register { code: 0 },
      r10: Register { code: 0 },
      r11: Register { code: 0 },
      r12: Register { code: 0 },
      r13: Register { code: 0 },
      r14: Register { code: 0 },
      r15: Register { code: 0 },
    }
  }

  pub const fn call_with_spread_registers() -> RegisterArray {
    RegisterArray {
      r0: rdi,
      r1: rax,
      r2: rbx,
      r3: Register { code: 0 },
      r4: Register { code: 0 },
      r5: Register { code: 0 },
      r6: Register { code: 0 },
      r7: Register { code: 0 },
      r8: Register { code: 0 },
      r9: Register { code: 0 },
      r10: Register { code: 0 },
      r11: Register { code: 0 },
      r12: Register { code: 0 },
      r13: Register { code: 0 },
      r14: Register { code: 0 },
      r15: Register { code: 0 },
    }
  }

  pub const fn call_with_array_like_registers() -> RegisterArray {
    RegisterArray {
      r0: rdi,
      r1: rbx,
      r2: Register { code: 0 },
      r3: Register { code: 0 },
      r4: Register { code: 0 },
      r5: Register { code: 0 },
      r6: Register { code: 0 },
      r7: Register { code: 0 },
      r8: Register { code: 0 },
      r9: Register { code: 0 },
      r10: Register { code: 0 },
      r11: Register { code: 0 },
      r12: Register { code: 0 },
      r13: Register { code: 0 },
      r14: Register { code: 0 },
      r15: Register { code: 0 },
    }
  }

  pub const fn construct_varargs_registers() -> RegisterArray {
    RegisterArray {
      r0: rdi,
      r1: rdx,
      r2: rax,
      r3: rcx,
      r4: rbx,
      r5: Register { code: 0 },
      r6: Register { code: 0 },
      r7: Register { code: 0 },
      r8: Register { code: 0 },
      r9: Register { code: 0 },
      r10: Register { code: 0 },
      r11: Register { code: 0 },
      r12: Register { code: 0 },
      r13: Register { code: 0 },
      r14: Register { code: 0 },
      r15: Register { code: 0 },
    }
  }

  pub const fn construct_forward_varargs_registers() -> RegisterArray {
    RegisterArray {
      r0: rdi,
      r1: rdx,
      r2: rax,
      r3: rcx,
      r4: Register { code: 0 },
      r5: Register { code: 0 },
      r6: Register { code: 0 },
      r7: Register { code: 0 },
      r8: Register { code: 0 },
      r9: Register { code: 0 },
      r10: Register { code: 0 },
      r11: Register { code: 0 },
      r12: Register { code: 0 },
      r13: Register { code: 0 },
      r14: Register { code: 0 },
      r15: Register { code: 0 },
    }
  }

  pub const fn construct_with_spread_registers() -> RegisterArray {
    RegisterArray {
      r0: rdi,
      r1: rdx,
      r2: rax,
      r3: rbx,
      r4: Register { code: 0 },
      r5: Register { code: 0 },
      r6: Register { code: 0 },
      r7: Register { code: 0 },
      r8: Register { code: 0 },
      r9: Register { code: 0 },
      r10: Register { code: 0 },
      r11: Register { code: 0 },
      r12: Register { code: 0 },
      r13: Register { code: 0 },
      r14: Register { code: 0 },
      r15: Register { code: 0 },
    }
  }

  pub const fn construct_with_array_like_registers() -> RegisterArray {
    RegisterArray {
      r0: rdi,
      r1: rdx,
      r2: rbx,
      r3: Register { code: 0 },
      r4: Register { code: 0 },
      r5: Register { code: 0 },
      r6: Register { code: 0 },
      r7: Register { code: 0 },
      r8: Register { code: 0 },
      r9: Register { code: 0 },
      r10: Register { code: 0 },
      r11: Register { code: 0 },
      r12: Register { code: 0 },
      r13: Register { code: 0 },
      r14: Register { code: 0 },
      r15: Register { code: 0 },
    }
  }

  pub const fn construct_stub_registers() -> RegisterArray {
    RegisterArray {
      r0: rdi,
      r1: rdx,
      r2: rax,
      r3: Register { code: 0 },
      r4: Register { code: 0 },
      r5: Register { code: 0 },
      r6: Register { code: 0 },
      r7: Register { code: 0 },
      r8: Register { code: 0 },
      r9: Register { code: 0 },
      r10: Register { code: 0 },
      r11: Register { code: 0 },
      r12: Register { code: 0 },
      r13: Register { code: 0 },
      r14: Register { code: 0 },
      r15: Register { code: 0 },
    }
  }

  pub const fn abort_registers() -> RegisterArray {
    RegisterArray {
      r0: rdx,
      r1: Register { code: 0 },
      r2: Register { code: 0 },
      r3: Register { code: 0 },
      r4: Register { code: 0 },
      r5: Register { code: 0 },
      r6: Register { code: 0 },
      r7: Register { code: 0 },
      r8: Register { code: 0 },
      r9: Register { code: 0 },
      r10: Register { code: 0 },
      r11: Register { code: 0 },
      r12: Register { code: 0 },
      r13: Register { code: 0 },
      r14: Register { code: 0 },
      r15: Register { code: 0 },
    }
  }

  pub const fn compare_registers() -> RegisterArray {
    RegisterArray {
      r0: rdx,
      r1: rax,
      r2: Register { code: 0 },
      r3: Register { code: 0 },
      r4: Register { code: 0 },
      r5: Register { code: 0 },
      r6: Register { code: 0 },
      r7: Register { code: 0 },
      r8: Register { code: 0 },
      r9: Register { code: 0 },
      r10: Register { code: 0 },
      r11: Register { code: 0 },
      r12: Register { code: 0 },
      r13: Register { code: 0 },
      r14: Register { code: 0 },
      r15: Register { code: 0 },
    }
  }

  pub const fn binary_op_registers() -> RegisterArray {
    RegisterArray {
      r0: rdx,
      r1: rax,
      r2: Register { code: 0 },
      r3: Register { code: 0 },
      r4: Register { code: 0 },
      r5: Register { code: 0 },
      r6: Register { code: 0 },
      r7: Register { code: 0 },
      r8: Register { code: 0 },
      r9: Register { code: 0 },
      r10: Register { code: 0 },
      r11: Register { code: 0 },
      r12: Register { code: 0 },
      r13: Register { code: 0 },
      r14: Register { code: 0 },
      r15: Register { code: 0 },
    }
  }

  pub const fn compare_baseline_registers() -> RegisterArray {
    RegisterArray {
      r0: rdx,
      r1: rax,
      r2: rbx,
      r3: Register { code: 0 },
      r4: Register { code: 0 },
      r5: Register { code: 0 },
      r6: Register { code: 0 },
      r7: Register { code: 0 },
      r8: Register { code: 0 },
      r9: Register { code: 0 },
      r10: Register { code: 0 },
      r11: Register { code: 0 },
      r12: Register { code: 0 },
      r13: Register { code: 0 },
      r14: Register { code: 0 },
      r15: Register { code: 0 },
    }
  }

  pub const fn binary_op_baseline_registers() -> RegisterArray {
    RegisterArray {
      r0: rdx,
      r1: rax,
      r2: rbx,
      r3: Register { code: 0 },
      r4: Register { code: 0 },
      r5: Register { code: 0 },
      r6: Register { code: 0 },
      r7: Register { code: 0 },
      r8: Register { code: 0 },
      r9: Register { code: 0 },
      r10: Register { code: 0 },
      r11: Register { code: 0 },
      r12: Register { code: 0 },
      r13: Register { code: 0 },
      r14: Register { code: 0 },
      r15: Register { code: 0 },
    }
  }

  pub const fn binary_smi_op_baseline_registers() -> RegisterArray {
    RegisterArray {
      r0: rax,
      r1: rdx,
      r2: rbx,
      r3: Register { code: 0 },
      r4: Register { code: 0 },
      r5: Register { code: 0 },
      r6: Register { code: 0 },
      r7: Register { code: 0 },
      r8: Register { code: 0 },
      r9: Register { code: 0 },
      r10: Register { code: 0 },
      r11: Register { code: 0 },
      r12: Register { code: 0 },
      r13: Register { code: 0 },
      r14: Register { code: 0 },
      r15: Register { code: 0 },
    }
  }

  pub const fn call_api_callback_optimized_api_function_address_register() -> Register {
    rdx
  }

  pub const fn call_api_callback_optimized_actual_arguments_count_register() -> Register {
    rcx
  }

  pub const fn call_api_callback_optimized_function_template_info_register() -> Register {
    rbx
  }

  pub const fn call_api_callback_generic_actual_arguments_count_register() -> Register {
    rcx
  }

  pub const fn call_api_callback_generic_function_template_info_register() -> Register {
    rbx
  }

  pub const fn call_api_callback_generic_topmost_script_having_context_register() -> Register {
    rdx
  }

  pub const fn interpreter_dispatch_registers() -> RegisterArray {
    RegisterArray {
      r0: kInterpreterAccumulatorRegister,
      r1: kInterpreterBytecodeOffsetRegister,
      r2: kInterpreterBytecodeArrayRegister,
      r3: kInterpreterDispatchTableRegister,
      r4: Register { code: 0 },
      r5: Register { code: 0 },
      r6: Register { code: 0 },
      r7: Register { code: 0 },
      r8: Register { code: 0 },
      r9: Register { code: 0 },
      r10: Register { code: 0 },
      r11: Register { code: 0 },
      r12: Register { code: 0 },
      r13: Register { code: 0 },
      r14: Register { code: 0 },
      r15: Register { code: 0 },
    }
  }

  pub const fn interpreter_push_args_then_call_registers() -> RegisterArray {
    RegisterArray {
      r0: rax,
      r1: rbx,
      r2: rdi,
      r3: Register { code: 0 },
      r4: Register { code: 0 },
      r5: Register { code: 0 },
      r6: Register { code: 0 },
      r7: Register { code: 0 },
      r8: Register { code: 0 },
      r9: Register { code: 0 },
      r10: Register { code: 0 },
      r11: Register { code: 0 },
      r12: Register { code: 0 },
      r13: Register { code: 0 },
      r14: Register { code: 0 },
      r15: Register { code: 0 },
    }
  }

  pub const fn interpreter_push_args_then_construct_registers() -> RegisterArray {
    RegisterArray {
      r0: rax,
      r1: rcx,
      r2: rdi,
      r3: rdx,
      r4: rbx,
      r5: Register { code: 0 },
      r6: Register { code: 0 },
      r7: Register { code: 0 },
      r8: Register { code: 0 },
      r9: Register { code: 0 },
      r10: Register { code: 0 },
      r11: Register { code: 0 },
      r12: Register { code: 0 },
      r13: Register { code: 0 },
      r14: Register { code: 0 },
      r15: Register { code: 0 },
    }
  }

  pub const fn construct_forward_all_args_registers() -> RegisterArray {
    RegisterArray {
      r0: rdi,
      r1: rdx,
      r2: Register { code: 0 },
      r3: Register { code: 0 },
      r4: Register { code: 0 },
      r5: Register { code: 0 },
      r6: Register { code: 0 },
      r7: Register { code: 0 },
      r8: Register { code: 0 },
