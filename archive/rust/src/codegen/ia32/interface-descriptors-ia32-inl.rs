// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file provides IA32-specific interface descriptors.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

// Placeholder for architecture-specific conditional compilation
#[cfg(target_arch = "x86")]
pub mod interface_descriptors_ia32 {
    use crate::codegen::interface_descriptors::*;

    // Define register constants
    pub const eax: usize = 0;
    pub const ecx: usize = 1;
    pub const edx: usize = 2;
    pub const ebx: usize = 3;
    pub const esp: usize = 4;
    pub const ebp: usize = 5;
    pub const esi: usize = 6;
    pub const edi: usize = 7;

    pub const xmm0: usize = 0;
    pub const xmm1: usize = 1;
    pub const xmm2: usize = 2;
    pub const xmm3: usize = 3;
    pub const xmm4: usize = 4;
    pub const xmm5: usize = 5;
    pub const xmm6: usize = 6;
    pub const xmm7: usize = 7;

    pub const kReturnRegister0: usize = eax;
    pub const kReturnRegister1: usize = ecx;
    pub const kReturnRegister2: usize = edx;
    pub const kFPReturnRegister0: usize = xmm0;
    pub const no_dreg: usize = 8; // Represents no double register

    pub const kInterpreterAccumulatorRegister: usize = eax;
    pub const kInterpreterBytecodeOffsetRegister: usize = ebx;
    pub const kInterpreterBytecodeArrayRegister: usize = esi;
    pub const kInterpreterDispatchTableRegister: usize = edi;

    pub const no_reg: usize = 8; // Represents no register

    impl CallInterfaceDescriptor {
        pub const fn default_register_array() -> RegisterArray {
            let registers = RegisterArray([eax, ecx, edx, edi]);
            assert_eq!(registers.0.len(), kMaxBuiltinRegisterParams);
            registers
        }

        pub const fn default_double_register_array() -> DoubleRegisterArray {
            let registers = DoubleRegisterArray([xmm0, xmm1, xmm2, xmm3, xmm4, xmm5, xmm6]);
            registers
        }

        pub const fn default_return_register_array() -> RegisterArray {
            let registers = RegisterArray([kReturnRegister0, kReturnRegister1, kReturnRegister2]);
            registers
        }

        pub const fn default_return_double_register_array() -> DoubleRegisterArray {
            let registers = DoubleRegisterArray([kFPReturnRegister0, no_dreg, no_dreg]);
            registers
        }
    }

    #[cfg(debug_assertions)]
    pub trait StaticCallInterfaceDescriptorTrait {
        fn verify_argument_register_count(data: &mut CallInterfaceDescriptorData, nof_expected_args: usize);
    }

    #[cfg(debug_assertions)]
    impl<T: CallInterfaceDescriptorTrait> StaticCallInterfaceDescriptorTrait for T {
        fn verify_argument_register_count(data: &mut CallInterfaceDescriptorData, nof_expected_args: usize) {
            let allocatable_regs = data.allocatable_registers;
            if nof_expected_args >= 1 {
                assert!(allocatable_regs & (1 << esi) != 0);
            }
            if nof_expected_args >= 2 {
                assert!(allocatable_regs & (1 << edi) != 0);
            }
            // Additional arguments are passed on the stack.
        }
    }

    impl WriteBarrierDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray([edi, ecx, edx, esi, kReturnRegister0])
        }
    }

    impl LoadDescriptor {
        pub const fn receiver_register() -> usize {
            edx
        }
        pub const fn name_register() -> usize {
            ecx
        }
        pub const fn slot_register() -> usize {
            eax
        }
    }

    impl LoadWithVectorDescriptor {
        pub const fn vector_register() -> usize {
            no_reg
        }
    }

    impl KeyedLoadBaselineDescriptor {
        pub const fn receiver_register() -> usize {
            edx
        }
        pub const fn name_register() -> usize {
            kInterpreterAccumulatorRegister
        }
        pub const fn slot_register() -> usize {
            ecx
        }
    }

    impl KeyedLoadWithVectorDescriptor {
        pub const fn vector_register() -> usize {
            no_reg
        }
    }

    impl EnumeratedKeyedLoadBaselineDescriptor {
        pub const fn enum_index_register() -> usize {
            ecx
        }

        pub const fn cache_type_register() -> usize {
            no_reg
        }

        pub const fn slot_register() -> usize {
            no_reg
        }
    }

    impl KeyedHasICBaselineDescriptor {
        pub const fn receiver_register() -> usize {
            kInterpreterAccumulatorRegister
        }
        pub const fn name_register() -> usize {
            edx
        }
        pub const fn slot_register() -> usize {
            ecx
        }
    }

    impl KeyedHasICWithVectorDescriptor {
        pub const fn vector_register() -> usize {
            no_reg
        }
    }

    impl LoadWithReceiverAndVectorDescriptor {
        pub const fn lookup_start_object_register() -> usize {
            edi
        }
    }

    impl StoreDescriptor {
        pub const fn receiver_register() -> usize {
            edx
        }
        pub const fn name_register() -> usize {
            ecx
        }
        pub const fn value_register() -> usize {
            no_reg
        }
        pub const fn slot_register() -> usize {
            no_reg
        }
    }

    impl StoreWithVectorDescriptor {
        pub const fn vector_register() -> usize {
            no_reg
        }
    }

    impl DefineKeyedOwnDescriptor {
        pub const fn flags_register() -> usize {
            no_reg
        }
    }

    impl StoreTransitionDescriptor {
        pub const fn map_register() -> usize {
            edi
        }
    }

    impl ApiGetterDescriptor {
        pub const fn holder_register() -> usize {
            ecx
        }
        pub const fn callback_register() -> usize {
            eax
        }
    }

    impl GrowArrayElementsDescriptor {
        pub const fn object_register() -> usize {
            eax
        }
        pub const fn key_register() -> usize {
            ecx
        }
    }

    impl BaselineLeaveFrameDescriptor {
        pub const fn params_size_register() -> usize {
            esi
        }
        pub const fn weight_register() -> usize {
            edi
        }
    }

    impl TypeConversionDescriptor {
        pub const fn argument_register() -> usize {
            eax
        }
    }

    impl TypeofDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray([eax])
        }
    }

    impl CallTrampolineDescriptor {
        pub const fn registers() -> RegisterArray {
            // eax : number of arguments
            // edi : the target to call
            RegisterArray([edi, eax])
        }
    }

    impl CopyDataPropertiesWithExcludedPropertiesDescriptor {
        pub const fn registers() -> RegisterArray {
            // edi : the source
            // eax : the excluded property count
            RegisterArray([edi, eax])
        }
    }

    impl CopyDataPropertiesWithExcludedPropertiesOnStackDescriptor {
        pub const fn registers() -> RegisterArray {
            // edi : the source
            // eax : the excluded property count
            // ecx : the excluded property base
            RegisterArray([edi, eax, ecx])
        }
    }

    impl CallVarargsDescriptor {
        pub const fn registers() -> RegisterArray {
            // eax : number of arguments (on the stack)
            // edi : the target to call
            // ecx : arguments list length (untagged)
            // On the stack : arguments list (FixedArray)
            RegisterArray([edi, eax, ecx])
        }
    }

    impl CallForwardVarargsDescriptor {
        pub const fn registers() -> RegisterArray {
            // eax : number of arguments
            // ecx : start index (to support rest parameters)
            // edi : the target to call
            RegisterArray([edi, eax, ecx])
        }
    }

    impl CallFunctionTemplateDescriptor {
        pub const fn registers() -> RegisterArray {
            // edx : function template info
            // ecx : number of arguments (on the stack)
            RegisterArray([edx, ecx])
        }
    }

    impl CallFunctionTemplateGenericDescriptor {
        pub const fn registers() -> RegisterArray {
            // edx: the function template info
            // ecx: number of arguments (on the stack)
            // edi: topmost script-having context
            RegisterArray([edx, ecx, edi])
        }
    }

    impl CallWithSpreadDescriptor {
        pub const fn registers() -> RegisterArray {
            // eax : number of arguments (on the stack)
            // edi : the target to call
            // ecx : the object to spread
            RegisterArray([edi, eax, ecx])
        }
    }

    impl CallWithArrayLikeDescriptor {
        pub const fn registers() -> RegisterArray {
            // edi : the target to call
            // edx : the arguments list
            RegisterArray([edi, edx])
        }
    }

    impl ConstructVarargsDescriptor {
        pub const fn registers() -> RegisterArray {
            // eax : number of arguments (on the stack)
            // edi : the target to call
            // edx : the new target
            // ecx : arguments list length (untagged)
            // On the stack : arguments list (FixedArray)
            RegisterArray([edi, edx, eax, ecx])
        }
    }

    impl ConstructForwardVarargsDescriptor {
        pub const fn registers() -> RegisterArray {
            // eax : number of arguments
            // edx : the new target
            // ecx : start index (to support rest parameters)
            // edi : the target to call
            RegisterArray([edi, edx, eax, ecx])
        }
    }

    impl ConstructWithSpreadDescriptor {
        pub const fn registers() -> RegisterArray {
            // eax : number of arguments (on the stack)
            // edi : the target to call
            // edx : the new target
            // ecx : the object to spread
            RegisterArray([edi, edx, eax, ecx])
        }
    }

    impl ConstructWithArrayLikeDescriptor {
        pub const fn registers() -> RegisterArray {
            // edi : the target to call
            // edx : the new target
            // ecx : the arguments list
            RegisterArray([edi, edx, ecx])
        }
    }

    impl ConstructStubDescriptor {
        pub const fn registers() -> RegisterArray {
            // eax : number of arguments
            // edx : the new target
            // edi : the target to call
            RegisterArray([edi, edx, eax])
        }
    }

    impl AbortDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray([edx])
        }
    }

    impl CompareDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray([edx, eax])
        }
    }

    impl Compare_BaselineDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray([edx, eax, ecx])
        }
    }

    impl BinaryOpDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray([edx, eax])
        }
    }

    impl BinaryOp_BaselineDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray([edx, eax, ecx])
        }
    }

    impl BinarySmiOp_BaselineDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray([eax, edx, ecx])
        }
    }

    impl CallApiCallbackOptimizedDescriptor {
        pub const fn api_function_address_register() -> usize {
            eax
        }
        pub const fn actual_arguments_count_register() -> usize {
            ecx
        }
        pub const fn function_template_info_register() -> usize {
            edx
        }
    }

    impl CallApiCallbackGenericDescriptor {
        pub const fn actual_arguments_count_register() -> usize {
            ecx
        }
        pub const fn topmost_script_having_context_register() -> usize {
            eax
        }
        pub const fn function_template_info_register() -> usize {
            edx
        }
    }

    impl InterpreterDispatchDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray([
                kInterpreterAccumulatorRegister,
                kInterpreterBytecodeOffsetRegister,
                kInterpreterBytecodeArrayRegister,
                kInterpreterDispatchTableRegister,
            ])
        }
    }

    impl InterpreterPushArgsThenCallDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray([
                eax, // argument count
                ecx, // address of first argument
                edi, // the target callable to be call
            ])
        }
    }

    impl InterpreterPushArgsThenConstructDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray([
                eax, // argument count
                ecx, // address of first argument
            ])
        }
    }

    impl ConstructForwardAllArgsDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray([
                edi, // the constructor
                edx, // the new target
            ])
        }
    }

    impl ResumeGeneratorDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray([
                eax, // the value to pass to the generator
                edx, // the JSGeneratorObject to resume
            ])
        }
    }

    impl RunMicrotasksEntryDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray([])
        }
    }

    impl WasmJSToWasmWrapperDescriptor {
        pub const fn registers() -> RegisterArray {
            // Arbitrarily picked register.
            RegisterArray([edi])
        }
    }
}

pub mod codegen {
    pub mod interface_descriptors {
        //Dummy definitions to avoid errors
        pub struct CallInterfaceDescriptor {
        }
        pub struct RegisterArray(pub [usize; 4]);
        pub struct DoubleRegisterArray(pub [usize; 7]);
        pub const kMaxBuiltinRegisterParams: usize = 4;

        pub struct CallInterfaceDescriptorData {
            pub allocatable_registers: usize,
        }

        pub struct WriteBarrierDescriptor{}
        pub struct LoadDescriptor{}
        pub struct LoadWithVectorDescriptor{}
        pub struct KeyedLoadBaselineDescriptor{}
        pub struct KeyedLoadWithVectorDescriptor{}
        pub struct EnumeratedKeyedLoadBaselineDescriptor{}
        pub struct KeyedHasICBaselineDescriptor{}
        pub struct KeyedHasICWithVectorDescriptor{}
        pub struct LoadWithReceiverAndVectorDescriptor{}
        pub struct StoreDescriptor{}
        pub struct StoreWithVectorDescriptor{}
        pub struct DefineKeyedOwnDescriptor{}
        pub struct StoreTransitionDescriptor{}
        pub struct ApiGetterDescriptor{}
        pub struct GrowArrayElementsDescriptor{}
        pub struct BaselineLeaveFrameDescriptor{}
        pub struct TypeConversionDescriptor{}
        pub struct TypeofDescriptor{}
        pub struct CallTrampolineDescriptor{}
        pub struct CopyDataPropertiesWithExcludedPropertiesDescriptor{}
        pub struct CopyDataPropertiesWithExcludedPropertiesOnStackDescriptor{}
        pub struct CallVarargsDescriptor{}
        pub struct CallForwardVarargsDescriptor{}
        pub struct CallFunctionTemplateDescriptor{}
        pub struct CallFunctionTemplateGenericDescriptor{}
        pub struct CallWithSpreadDescriptor{}
        pub struct CallWithArrayLikeDescriptor{}
        pub struct ConstructVarargsDescriptor{}
        pub struct ConstructForwardVarargsDescriptor{}
        pub struct ConstructWithSpreadDescriptor{}
        pub struct ConstructWithArrayLikeDescriptor{}
        pub struct ConstructStubDescriptor{}
        pub struct AbortDescriptor{}
        pub struct CompareDescriptor{}
        pub struct Compare_BaselineDescriptor{}
        pub struct BinaryOpDescriptor{}
        pub struct BinaryOp_BaselineDescriptor{}
        pub struct BinarySmiOp_BaselineDescriptor{}
        pub struct CallApiCallbackOptimizedDescriptor{}
        pub struct CallApiCallbackGenericDescriptor{}
        pub struct InterpreterDispatchDescriptor{}
        pub struct InterpreterPushArgsThenCallDescriptor{}
        pub struct InterpreterPushArgsThenConstructDescriptor{}
        pub struct ConstructForwardAllArgsDescriptor{}
        pub struct ResumeGeneratorDescriptor{}
        pub struct RunMicrotasksEntryDescriptor{}
        pub struct WasmJSToWasmWrapperDescriptor{}
    }
}