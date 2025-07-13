// Converted from V8 C++ source files:
// Header: interface-descriptors-loong64-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod loong64_interface_descriptors_loong64_inl {
use std::fmt;

use crate::codegen::interface_descriptors::*;
use crate::codegen::loong64::assembler_loong64::*;
use crate::codegen::loong64::register_loong64::*;
use crate::execution::frames::*;

  
    
        
            
                
                    
                        
                            
                                
                                
                                    
                                    

pub type RegisterArray = [Register; 5];
pub type DoubleRegisterArray = [DoubleRegister; 7];

pub const fn default_register_array() -> RegisterArray {
    [a0, a1, a2, a3, a4]
}

pub const fn default_double_register_array() -> DoubleRegisterArray {
    [
        DoubleRegister { code_: 0 },
        DoubleRegister { code_: 1 },
        DoubleRegister { code_: 2 },
        DoubleRegister { code_: 3 },
        DoubleRegister { code_: 4 },
        DoubleRegister { code_: 5 },
        DoubleRegister { code_: 6 },
    ]
}

pub const fn default_return_register_array() -> RegisterArray {
    [kReturnRegister0, kReturnRegister1, kReturnRegister2]
}

pub const fn default_return_double_register_array() -> DoubleRegisterArray {
    [
        kFPReturnRegister0,
        DoubleRegister { code_: -1 },
        DoubleRegister { code_: -1 },
    ]
}

#[cfg(debug_assertions)]
pub fn verify_argument_register_count(data: &CallInterfaceDescriptorData, argc: i32) {
    let allocatable_regs = data.allocatable_registers();
    if argc >= 1 {
        assert!(allocatable_regs.contains(&a0));
    }
    if argc >= 2 {
        assert!(allocatable_regs.contains(&a1));
    }
    if argc >= 3 {
        assert!(allocatable_regs.contains(&a2));
    }
    if argc >= 4 {
        assert!(allocatable_regs.contains(&a3));
    }
    if argc >= 5 {
        assert!(allocatable_regs.contains(&a4));
    }
    if argc >= 6 {
        assert!(allocatable_regs.contains(&a5));
    }
    if argc >= 7 {
        assert!(allocatable_regs.contains(&a6));
    }
    if argc >= 8 {
        assert!(allocatable_regs.contains(&a7));
    }
}

pub const fn write_barrier_registers() -> RegisterArray {
    [
        a1,
        a5,
        a4,
        a2,
        a0,
        a3,
        kContextRegister,
    ]
}

pub const fn load_receiver_register() -> Register {
    a1
}

pub const fn load_name_register() -> Register {
    a2
}

pub const fn load_slot_register() -> Register {
    a0
}

pub const fn load_with_vector_vector_register() -> Register {
    a3
}

pub const fn keyed_load_baseline_receiver_register() -> Register {
    a1
}

pub const fn keyed_load_baseline_name_register() -> Register {
    kInterpreterAccumulatorRegister
}

pub const fn keyed_load_baseline_slot_register() -> Register {
    a2
}

pub const fn keyed_load_with_vector_vector_register() -> Register {
    a3
}

pub const fn enumerated_keyed_load_baseline_enum_index_register() -> Register {
    a4
}

pub const fn enumerated_keyed_load_baseline_cache_type_register() -> Register {
    a5
}

pub const fn enumerated_keyed_load_baseline_slot_register() -> Register {
    a2
}

pub const fn keyed_has_ic_baseline_receiver_register() -> Register {
    kInterpreterAccumulatorRegister
}

pub const fn keyed_has_ic_baseline_name_register() -> Register {
    a1
}

pub const fn keyed_has_ic_baseline_slot_register() -> Register {
    a2
}

pub const fn keyed_has_ic_with_vector_vector_register() -> Register {
    a3
}

pub const fn load_with_receiver_and_vector_lookup_start_object_register() -> Register {
    a4
}

pub const fn store_receiver_register() -> Register {
    a1
}

pub const fn store_name_register() -> Register {
    a2
}

pub const fn store_value_register() -> Register {
    a0
}

pub const fn store_slot_register() -> Register {
    a4
}

pub const fn store_with_vector_vector_register() -> Register {
    a3
}

pub const fn define_keyed_own_flags_register() -> Register {
    a5
}

pub const fn store_transition_map_register() -> Register {
    a5
}

pub const fn api_getter_holder_register() -> Register {
    a0
}

pub const fn api_getter_callback_register() -> Register {
    a3
}

pub const fn grow_array_elements_object_register() -> Register {
    a0
}

pub const fn grow_array_elements_key_register() -> Register {
    a3
}

pub const fn baseline_leave_frame_params_size_register() -> Register {
    a2
}

pub const fn baseline_leave_frame_weight_register() -> Register {
    a3
}

pub const fn type_conversion_argument_register() -> Register {
    a0
}

pub const fn typeof_registers() -> RegisterArray {
    [a0]
}

pub const fn call_trampoline_registers() -> RegisterArray {
    [a1, a0]
}

pub const fn copy_data_properties_with_excluded_properties_registers() -> RegisterArray {
    [a1, a0]
}

pub const fn copy_data_properties_with_excluded_properties_on_stack_registers() -> RegisterArray {
    [a1, a0, a2]
}

pub const fn call_varargs_registers() -> RegisterArray {
    [a1, a0, a4, a2]
}

pub const fn call_forward_varargs_registers() -> RegisterArray {
    [a1, a0, a2]
}

pub const fn call_function_template_registers() -> RegisterArray {
    [a1, a0]
}

pub const fn call_function_template_generic_registers() -> RegisterArray {
    [a1, a2, a3]
}

pub const fn call_with_spread_registers() -> RegisterArray {
    [a1, a0, a2]
}

pub const fn call_with_array_like_registers() -> RegisterArray {
    [a1, a2]
}

pub const fn construct_varargs_registers() -> RegisterArray {
    [a1, a3, a0, a4, a2]
}

pub const fn construct_forward_varargs_registers() -> RegisterArray {
    [a1, a3, a0, a2]
}

pub const fn construct_with_spread_registers() -> RegisterArray {
    [a1, a3, a0, a2]
}

pub const fn construct_with_array_like_registers() -> RegisterArray {
    [a1, a3, a2]
}

pub const fn construct_stub_registers() -> RegisterArray {
    [a1, a3, a0]
}

pub const fn abort_registers() -> RegisterArray {
    [a0]
}

pub const fn compare_registers() -> RegisterArray {
    [a1, a0]
}

pub const fn compare_baseline_registers() -> RegisterArray {
    [a1, a0, a2]
}

pub const fn binary_op_registers() -> RegisterArray {
    [a1, a0]
}

pub const fn binary_op_baseline_registers() -> RegisterArray {
    [a1, a0, a2]
}

pub const fn binary_smi_op_baseline_registers() -> RegisterArray {
    [a0, a1, a2]
}

pub const fn call_api_callback_optimized_api_function_address_register() -> Register {
    a1
}

pub const fn call_api_callback_optimized_actual_arguments_count_register() -> Register {
    a2
}

pub const fn call_api_callback_generic_topmost_script_having_context_register() -> Register {
    a1
}

pub const fn call_api_callback_optimized_function_template_info_register() -> Register {
    a3
}

pub const fn call_api_callback_generic_actual_arguments_count_register() -> Register {
    a2
}

pub const fn call_api_callback_generic_function_template_info_register() -> Register {
    a3
}

pub const fn interpreter_dispatch_registers() -> RegisterArray {
    [
        kInterpreterAccumulatorRegister,
        kInterpreterBytecodeOffsetRegister,
        kInterpreterBytecodeArrayRegister,
        kInterpreterDispatchTableRegister,
    ]
}

pub const fn interpreter_push_args_then_call_registers() -> RegisterArray {
    [a0, a2, a1]
}

pub const fn interpreter_push_args_then_construct_registers() -> RegisterArray {
    [a0, a4, a1, a3, a2]
}

pub const fn construct_forward_all_args_registers() -> RegisterArray {
    [a1, a3]
}

pub const fn resume_generator_registers() -> RegisterArray {
    [a0, a1]
}

pub const fn run_microtasks_entry_registers() -> RegisterArray {
    [a0, a1]
}

pub const fn wasm_js_to_wasm_wrapper_registers() -> RegisterArray {
    [t0]
}
}
