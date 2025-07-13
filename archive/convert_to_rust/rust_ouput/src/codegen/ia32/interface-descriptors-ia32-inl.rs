// Converted from V8 C++ source files:
// Header: interface-descriptors-ia32-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod interface_descriptors_ia32_inl {
use crate::codegen::interface_descriptors::*;
use crate::codegen::x64::register_x64::*;
use crate::codegen::x64::reglist_x64::*;
use crate::codegen::code_factory::*;
use crate::codegen::assembler_arch::*;

    pub const fn default_register_array() -> RegisterArray {
        RegisterArray {
            registers: [eax, ecx, edx, edi],
        }
    }

    pub const fn default_double_register_array() -> DoubleRegisterArray {
        DoubleRegisterArray {
            registers: [xmm0, xmm1, xmm2, xmm3, xmm4, xmm5, xmm6],
        }
    }

    pub const fn default_return_register_array() -> RegisterArray {
        RegisterArray {
            registers: [kReturnRegister0, kReturnRegister1, kReturnRegister2],
        }
    }

    pub const fn default_return_double_register_array() -> DoubleRegisterArray {
        DoubleRegisterArray {
            registers: [kFPReturnRegister0, no_dreg, no_dreg],
        }
    }

    #[cfg(debug_assertions)]
    pub fn verify_argument_register_count<T>(data: &CallInterfaceDescriptorData, nof_expected_args: i32) {
        let allocatable_regs = data.allocatable_registers();
        if nof_expected_args >= 1 {
            assert!(allocatable_regs.has(esi));
        }
        if nof_expected_args >= 2 {
            assert!(allocatable_regs.has(edi));
        }
    }

    pub mod write_barrier_descriptor {
        use super::*;
        pub const fn registers() -> RegisterArray {
            RegisterArray {
                registers: [edi, ecx, edx, esi, kReturnRegister0],
            }
        }
    }

    pub mod load_descriptor {
        use super::*;
        pub const fn receiver_register() -> Register {
            edx
        }
        pub const fn name_register() -> Register {
            ecx
        }
        pub const fn slot_register() -> Register {
            eax
        }
    }

    pub mod load_with_vector_descriptor {
        use super::*;
        pub const fn vector_register() -> Register {
            no_reg
        }
    }

    pub mod keyed_load_baseline_descriptor {
        use super::*;
        pub const fn receiver_register() -> Register {
            edx
        }
        pub const fn name_register() -> Register {
            kInterpreterAccumulatorRegister
        }
        pub const fn slot_register() -> Register {
            ecx
        }
    }

    pub mod keyed_load_with_vector_descriptor {
        use super::*;
        pub const fn vector_register() -> Register {
            no_reg
        }
    }

    pub mod enumerated_keyed_load_baseline_descriptor {
        use super::*;
        pub const fn enum_index_register() -> Register {
            ecx
        }
        pub const fn cache_type_register() -> Register {
            no_reg
        }
        pub const fn slot_register() -> Register {
            no_reg
        }
    }

    pub mod keyed_has_ic_baseline_descriptor {
        use super::*;
        pub const fn receiver_register() -> Register {
            kInterpreterAccumulatorRegister
        }
        pub const fn name_register() -> Register {
            edx
        }
        pub const fn slot_register() -> Register {
            ecx
        }
    }

    pub mod keyed_has_ic_with_vector_descriptor {
        use super::*;
        pub const fn vector_register() -> Register {
            no_reg
        }
    }

    pub mod load_with_receiver_and_vector_descriptor {
        use super::*;
        pub const fn lookup_start_object_register() -> Register {
            edi
        }
    }

    pub mod store_descriptor {
        use super::*;
        pub const fn receiver_register() -> Register {
            edx
        }
        pub const fn name_register() -> Register {
            ecx
        }
        pub const fn value_register() -> Register {
            no_reg
        }
        pub const fn slot_register() -> Register {
            no_reg
        }
    }

    pub mod store_with_vector_descriptor {
        use super::*;
        pub const fn vector_register() -> Register {
            no_reg
        }
    }

    pub mod define_keyed_own_descriptor {
        use super::*;
        pub const fn flags_register() -> Register {
            no_reg
        }
    }

    pub mod store_transition_descriptor {
        use super::*;
        pub const fn map_register() -> Register {
            edi
        }
    }

    pub mod api_getter_descriptor {
        use super::*;
        pub const fn holder_register() -> Register {
            ecx
        }
        pub const fn callback_register() -> Register {
            eax
        }
    }

    pub mod grow_array_elements_descriptor {
        use super::*;
        pub const fn object_register() -> Register {
            eax
        }
        pub const fn key_register() -> Register {
            ecx
        }
    }

    pub mod baseline_leave_frame_descriptor {
        use super::*;
        pub const fn params_size_register() -> Register {
            esi
        }
        pub const fn weight_register() -> Register {
            edi
        }
    }

    pub mod type_conversion_descriptor {
        use super::*;
        pub const fn argument_register() -> Register {
            eax
        }
    }

    pub mod typeof_descriptor {
        use super::*;
        pub const fn registers() -> RegisterArray {
            RegisterArray {
                registers: [eax],
            }
        }
    }

    pub mod call_trampoline_descriptor {
        use super::*;
        pub const fn registers() -> RegisterArray {
            RegisterArray {
                registers: [edi, eax],
            }
        }
    }

    pub mod copy_data_properties_with_excluded_properties_descriptor {
        use super::*;
        pub const fn registers() -> RegisterArray {
            RegisterArray {
                registers: [edi, eax],
            }
        }
    }

    pub mod copy_data_properties_with_excluded_properties_on_stack_descriptor {
        use super::*;
        pub const fn registers() -> RegisterArray {
            RegisterArray {
                registers: [edi, eax, ecx],
            }
        }
    }

    pub mod call_varargs_descriptor {
        use super::*;
        pub const fn registers() -> RegisterArray {
            RegisterArray {
                registers: [edi, eax, ecx],
            }
        }
    }

    pub mod call_forward_varargs_descriptor {
        use super::*;
        pub const fn registers() -> RegisterArray {
            RegisterArray {
                registers: [edi, eax, ecx],
            }
        }
    }

    pub mod call_function_template_descriptor {
        use super::*;
        pub const fn registers() -> RegisterArray {
            RegisterArray {
                registers: [edx, ecx],
            }
        }
    }

    pub mod call_function_template_generic_descriptor {
        use super::*;
        pub const fn registers() -> RegisterArray {
            RegisterArray {
                registers: [edx, ecx, edi],
            }
        }
    }

    pub mod call_with_spread_descriptor {
        use super::*;
        pub const fn registers() -> RegisterArray {
            RegisterArray {
                registers: [edi, eax, ecx],
            }
        }
    }

    pub mod call_with_array_like_descriptor {
        use super::*;
        pub const fn registers() -> RegisterArray {
            RegisterArray {
                registers: [edi, edx],
            }
        }
    }

    pub mod construct_varargs_descriptor {
        use super::*;
        pub const fn registers() -> RegisterArray {
            RegisterArray {
                registers: [edi, edx, eax, ecx],
            }
        }
    }

    pub mod construct_forward_varargs_descriptor {
        use super::*;
        pub const fn registers() -> RegisterArray {
            RegisterArray {
                registers: [edi, edx, eax, ecx],
            }
        }
    }

    pub mod construct_with_spread_descriptor {
        use super::*;
        pub const fn registers() -> RegisterArray {
            RegisterArray {
                registers: [edi, edx, eax, ecx],
            }
        }
    }

    pub mod construct_with_array_like_descriptor {
        use super::*;
        pub const fn registers() -> RegisterArray {
            RegisterArray {
                registers: [edi, edx, ecx],
            }
        }
    }

    pub mod construct_stub_descriptor {
        use super::*;
        pub const fn registers() -> RegisterArray {
            RegisterArray {
                registers: [edi, edx, eax],
            }
        }
    }

    pub mod abort_descriptor {
        use super::*;
        pub const fn registers() -> RegisterArray {
            RegisterArray {
                registers: [edx],
            }
        }
    }

    pub mod compare_descriptor {
        use super::*;
        pub const fn registers() -> RegisterArray {
            RegisterArray {
                registers: [edx, eax],
            }
        }
    }

    pub mod compare_baseline_descriptor {
        use super::*;
        pub const fn registers() -> RegisterArray {
            RegisterArray {
                registers: [edx, eax, ecx],
            }
        }
    }

    pub mod binary_op_descriptor {
        use super::*;
        pub const fn registers() -> RegisterArray {
            RegisterArray {
                registers: [edx, eax],
            }
        }
    }

    pub mod binary_op_baseline_descriptor {
        use super::*;
        pub const fn registers() -> RegisterArray {
            RegisterArray {
                registers: [edx, eax, ecx],
            }
        }
    }

    pub mod binary_smi_op_baseline_descriptor {
        use super::*;
        pub const fn registers() -> RegisterArray {
            RegisterArray {
                registers: [eax, edx, ecx],
            }
        }
    }

    pub mod call_api_callback_optimized_descriptor {
        use super::*;
        pub const fn api_function_address_register() -> Register {
            eax
        }
        pub const fn actual_arguments_count_register() -> Register {
            ecx
        }
        pub const fn function_template_info_register() -> Register {
            edx
        }
    }

    pub mod call_api_callback_generic_descriptor {
        use super::*;
        pub const fn actual_arguments_count_register() -> Register {
            ecx
        }
        pub const fn topmost_script_having_context_register() -> Register {
            eax
        }
        pub const fn function_template_info_register() -> Register {
            edx
        }
    }

    pub mod interpreter_dispatch_descriptor {
        use super::*;
        pub const fn registers() -> RegisterArray {
            RegisterArray {
                registers: [
                    kInterpreterAccumulatorRegister,
                    kInterpreterBytecodeOffsetRegister,
                    kInterpreterBytecodeArrayRegister,
                    kInterpreterDispatchTableRegister,
                ],
            }
        }
    }

    pub mod interpreter_push_args_then_call_descriptor {
        use super::*;
        pub const fn registers() -> RegisterArray {
            RegisterArray {
                registers: [eax, ecx, edi],
            }
        }
    }

    pub mod interpreter_push_args_then_construct_descriptor {
        use super::*;
        pub const fn registers() -> RegisterArray {
            RegisterArray {
                registers: [eax, ecx],
            }
        }
    }

    pub mod construct_forward_all_args_descriptor {
        use super::*;
        pub const fn registers() -> RegisterArray {
            RegisterArray {
                registers: [edi, edx],
            }
        }
    }

    pub mod resume_generator_descriptor {
        use super::*;
        pub const fn registers() -> RegisterArray {
            RegisterArray {
                registers: [eax, edx],
            }
        }
    }

    pub mod run_microtasks_entry_descriptor {
        use super::*;
        pub const fn registers() -> RegisterArray {
            RegisterArray {
                registers: [],
            }
        }
    }

    pub mod wasm_js_to_wasm_wrapper_descriptor {
        use super::*;
        pub const fn registers() -> RegisterArray {
            RegisterArray {
                registers: [edi],
            }
        }
    }
}
