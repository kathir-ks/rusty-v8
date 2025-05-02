// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file is a Rust translation of the C++ header file
// `src/codegen/x64/interface-descriptors-x64-inl.h` from the V8 JavaScript engine codebase.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[cfg(target_arch = "x86_64")]
pub mod interface_descriptors_x64 {
    use crate::codegen::interface_descriptors::*;
    use crate::register::*;

    pub mod register {
        pub use crate::register::*;
    }

    // TODO: Define these based on target architecture.  Using placeholders
    // to make the code compile. These constants should correspond to the
    // hardware registers used in x64 architecture.
    const rax: Register = Register(0);
    const rbx: Register = Register(1);
    const rcx: Register = Register(2);
    const rdx: Register = Register(3);
    const rdi: Register = Register(4);
    const rsi: Register = Register(5);
    const r8: Register = Register(8);
    const r9: Register = Register(9);
    const r11: Register = Register(11);

    const xmm0: DoubleRegister = DoubleRegister(0);
    const xmm1: DoubleRegister = DoubleRegister(1);
    const xmm2: DoubleRegister = DoubleRegister(2);
    const xmm3: DoubleRegister = DoubleRegister(3);
    const xmm4: DoubleRegister = DoubleRegister(4);
    const xmm5: DoubleRegister = DoubleRegister(5);
    const xmm6: DoubleRegister = DoubleRegister(6);

    const no_dreg: DoubleRegister = DoubleRegister(-1);

    impl CallInterfaceDescriptor {
        pub const fn default_register_array() -> RegisterArray<kMaxBuiltinRegisterParams> {
            let registers = RegisterArray([rax, rbx, rcx, rdx, rdi]);
            assert_eq!(registers.0.len(), kMaxBuiltinRegisterParams);
            registers
        }

        pub const fn default_double_register_array() -> DoubleRegisterArray<7> {
            DoubleRegisterArray([xmm0, xmm1, xmm2, xmm3, xmm4, xmm5, xmm6])
        }

        pub const fn default_return_register_array() -> RegisterArray<3> {
            let registers = RegisterArray([kReturnRegister0, kReturnRegister1, kReturnRegister2]);
            registers
        }

        pub const fn default_return_double_register_array() -> DoubleRegisterArray<3> {
            // Padding to have as many double return registers as GP return registers.
            let registers = DoubleRegisterArray([kFPReturnRegister0, no_dreg, no_dreg]);
            registers
        }
    }

    #[cfg(debug_assertions)]
    pub trait StaticCallInterfaceDescriptorTrait {
        fn verify_argument_register_count(data: &mut CallInterfaceDescriptorData, nof_expected_args: i32);
    }

    #[cfg(debug_assertions)]
    impl<DerivedDescriptor> StaticCallInterfaceDescriptorTrait for StaticCallInterfaceDescriptor<DerivedDescriptor> {
        fn verify_argument_register_count(data: &mut CallInterfaceDescriptorData, nof_expected_args: i32) {
            let allocatable_regs = data.allocatable_registers();
            if nof_expected_args >= 1 {
                assert!(allocatable_regs.has(kCArgRegs[0]));
            }
            if nof_expected_args >= 2 {
                assert!(allocatable_regs.has(kCArgRegs[1]));
            }
            if nof_expected_args >= 3 {
                assert!(allocatable_regs.has(kCArgRegs[2]));
            }
            if nof_expected_args >= 4 {
                assert!(allocatable_regs.has(kCArgRegs[3]));
            }
            // Additional arguments are passed on the stack.
        }
    }

    impl WriteBarrierDescriptor {
        pub const fn registers() -> RegisterArray<7> {
            #[cfg(target_os = "windows")]
            {
                RegisterArray([rdi, r8, rcx, rax, r9, rdx, rsi])
            }
            #[cfg(not(target_os = "windows"))]
            {
                RegisterArray([rdi, rbx, rdx, rcx, rax, rsi])
            }
        }
    }

    #[cfg(feature = "tsan")]
    impl TSANStoreDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            RegisterArray([kCArgRegs[0], kCArgRegs[1], kReturnRegister0])
        }
    }

    #[cfg(feature = "tsan")]
    impl TSANLoadDescriptor {
        pub const fn registers() -> RegisterArray<2> {
            RegisterArray([kCArgRegs[0], kReturnRegister0])
        }
    }

    impl LoadDescriptor {
        pub const fn receiver_register() -> Register {
            rdx
        }
        pub const fn name_register() -> Register {
            rcx
        }
        pub const fn slot_register() -> Register {
            rax
        }
    }

    impl LoadWithVectorDescriptor {
        pub const fn vector_register() -> Register {
            rbx
        }
    }

    impl KeyedLoadBaselineDescriptor {
        pub const fn receiver_register() -> Register {
            rdx
        }
        pub const fn name_register() -> Register {
            kInterpreterAccumulatorRegister
        }
        pub const fn slot_register() -> Register {
            rcx
        }
    }

    impl KeyedLoadWithVectorDescriptor {
        pub const fn vector_register() -> Register {
            rbx
        }
    }

    impl EnumeratedKeyedLoadBaselineDescriptor {
        pub const fn enum_index_register() -> Register {
            rdi
        }

        pub const fn cache_type_register() -> Register {
            r8
        }

        pub const fn slot_register() -> Register {
            rcx
        }
    }

    impl KeyedHasICBaselineDescriptor {
        pub const fn receiver_register() -> Register {
            kInterpreterAccumulatorRegister
        }
        pub const fn name_register() -> Register {
            rdx
        }
        pub const fn slot_register() -> Register {
            rcx
        }
    }

    impl KeyedHasICWithVectorDescriptor {
        pub const fn vector_register() -> Register {
            rbx
        }
    }

    impl LoadWithReceiverAndVectorDescriptor {
        pub const fn lookup_start_object_register() -> Register {
            rdi
        }
    }

    impl StoreDescriptor {
        pub const fn receiver_register() -> Register {
            rdx
        }
        pub const fn name_register() -> Register {
            rcx
        }
        pub const fn value_register() -> Register {
            rax
        }
        pub const fn slot_register() -> Register {
            rdi
        }
    }

    impl StoreWithVectorDescriptor {
        pub const fn vector_register() -> Register {
            rbx
        }
    }

    impl DefineKeyedOwnDescriptor {
        pub const fn flags_register() -> Register {
            r11
        }
    }

    impl StoreTransitionDescriptor {
        pub const fn map_register() -> Register {
            r11
        }
    }

    impl ApiGetterDescriptor {
        pub const fn holder_register() -> Register {
            rcx
        }
        pub const fn callback_register() -> Register {
            rbx
        }
    }

    impl GrowArrayElementsDescriptor {
        pub const fn object_register() -> Register {
            rax
        }
        pub const fn key_register() -> Register {
            rbx
        }
    }

    impl BaselineLeaveFrameDescriptor {
        pub const fn params_size_register() -> Register {
            rbx
        }
        pub const fn weight_register() -> Register {
            rcx
        }
    }

    impl MaglevOptimizeCodeOrTailCallOptimizedCodeSlotDescriptor {
        pub const fn flags_register() -> Register {
            r8
        }
        pub const fn feedback_vector_register() -> Register {
            r9
        }
        pub const fn temporary_register() -> Register {
            r11
        }
    }

    impl TypeConversionDescriptor {
        pub const fn argument_register() -> Register {
            rax
        }
    }

    impl TypeofDescriptor {
        pub const fn registers() -> RegisterArray<1> {
            RegisterArray([rax])
        }
    }

    impl CallTrampolineDescriptor {
        pub const fn registers() -> RegisterArray<2> {
            // rax : number of arguments
            // rdi : the target to call
            RegisterArray([rdi, rax])
        }
    }
    impl CopyDataPropertiesWithExcludedPropertiesDescriptor {
        pub const fn registers() -> RegisterArray<2> {
            // rdi : the source
            // rax : the excluded property count
            RegisterArray([rdi, rax])
        }
    }

    impl CopyDataPropertiesWithExcludedPropertiesOnStackDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            // rdi : the source
            // rax : the excluded property count
            // rcx : the excluded property base
            RegisterArray([rdi, rax, rcx])
        }
    }

    impl CallVarargsDescriptor {
        pub const fn registers() -> RegisterArray<4> {
            // rax : number of arguments (on the stack)
            // rdi : the target to call
            // rcx : arguments list length (untagged)
            // rbx : arguments list (FixedArray)
            RegisterArray([rdi, rax, rcx, rbx])
        }
    }

    impl CallForwardVarargsDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            // rax : number of arguments
            // rcx : start index (to support rest parameters)
            // rdi : the target to call
            RegisterArray([rdi, rax, rcx])
        }
    }

    impl CallFunctionTemplateDescriptor {
        pub const fn registers() -> RegisterArray<2> {
            // rdx: the function template info
            // rcx: number of arguments (on the stack)
            RegisterArray([rdx, rcx])
        }
    }

    impl CallFunctionTemplateGenericDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            // rdx: the function template info
            // rcx: number of arguments (on the stack)
            // rdi: topmost script-having context
            RegisterArray([rdx, rcx, rdi])
        }
    }

    impl CallWithSpreadDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            // rax : number of arguments (on the stack)
            // rdi : the target to call
            // rbx : the object to spread
            RegisterArray([rdi, rax, rbx])
        }
    }

    impl CallWithArrayLikeDescriptor {
        pub const fn registers() -> RegisterArray<2> {
            // rdi : the target to call
            // rbx : the arguments list
            RegisterArray([rdi, rbx])
        }
    }

    impl ConstructVarargsDescriptor {
        pub const fn registers() -> RegisterArray<5> {
            // rax : number of arguments (on the stack)
            // rdi : the target to call
            // rdx : the new target
            // rcx : arguments list length (untagged)
            // rbx : arguments list (FixedArray)
            RegisterArray([rdi, rdx, rax, rcx, rbx])
        }
    }

    impl ConstructForwardVarargsDescriptor {
        pub const fn registers() -> RegisterArray<4> {
            // rax : number of arguments
            // rdx : the new target
            // rcx : start index (to support rest parameters)
            // rdi : the target to call
            RegisterArray([rdi, rdx, rax, rcx])
        }
    }

    impl ConstructWithSpreadDescriptor {
        pub const fn registers() -> RegisterArray<4> {
            // rax : number of arguments (on the stack)
            // rdi : the target to call
            // rdx : the new target
            // rbx : the object to spread
            RegisterArray([rdi, rdx, rax, rbx])
        }
    }

    impl ConstructWithArrayLikeDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            // rdi : the target to call
            // rdx : the new target
            // rbx : the arguments list
            RegisterArray([rdi, rdx, rbx])
        }
    }

    impl ConstructStubDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            // rax : number of arguments
            // rdx : the new target
            // rdi : the target to call
            RegisterArray([rdi, rdx, rax])
        }
    }

    impl AbortDescriptor {
        pub const fn registers() -> RegisterArray<1> {
            RegisterArray([rdx])
        }
    }

    impl CompareDescriptor {
        pub const fn registers() -> RegisterArray<2> {
            RegisterArray([rdx, rax])
        }
    }

    impl BinaryOpDescriptor {
        pub const fn registers() -> RegisterArray<2> {
            RegisterArray([rdx, rax])
        }
    }

    impl Compare_BaselineDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            RegisterArray([rdx, rax, rbx])
        }
    }

    impl BinaryOp_BaselineDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            RegisterArray([rdx, rax, rbx])
        }
    }

    impl BinarySmiOp_BaselineDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            RegisterArray([rax, rdx, rbx])
        }
    }

    impl CallApiCallbackOptimizedDescriptor {
        pub const fn api_function_address_register() -> Register {
            rdx
        }
        pub const fn actual_arguments_count_register() -> Register {
            rcx
        }
        pub const fn function_template_info_register() -> Register {
            rbx
        }
    }

    impl CallApiCallbackGenericDescriptor {
        pub const fn actual_arguments_count_register() -> Register {
            rcx
        }
        pub const fn function_template_info_register() -> Register {
            rbx
        }
        pub const fn topmost_script_having_context_register() -> Register {
            rdx
        }
    }

    impl InterpreterDispatchDescriptor {
        pub const fn registers() -> RegisterArray<4> {
            RegisterArray([
                kInterpreterAccumulatorRegister,
                kInterpreterBytecodeOffsetRegister,
                kInterpreterBytecodeArrayRegister,
                kInterpreterDispatchTableRegister,
            ])
        }
    }

    impl InterpreterPushArgsThenCallDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            RegisterArray([
                rax, // argument count
                rbx, // address of first argument
                rdi, // the target callable to be call
            ])
        }
    }

    impl InterpreterPushArgsThenConstructDescriptor {
        pub const fn registers() -> RegisterArray<5> {
            RegisterArray([
                rax, // argument count
                rcx, // address of first argument
                rdi, // constructor to call
                rdx, // new target
                rbx, // allocation site feedback if available, undefined otherwise
            ])
        }
    }

    impl ConstructForwardAllArgsDescriptor {
        pub const fn registers() -> RegisterArray<2> {
            RegisterArray([
                rdi, // constructor to call
                rdx, // new target
            ])
        }
    }

    impl ResumeGeneratorDescriptor {
        pub const fn registers() -> RegisterArray<2> {
            RegisterArray([
                rax, // the value to pass to the generator
                rdx, // the JSGeneratorObject / JSAsyncGeneratorObject to resume
            ])
        }
    }

    impl RunMicrotasksEntryDescriptor {
        pub const fn registers() -> RegisterArray<2> {
            RegisterArray([kCArgRegs[0], kCArgRegs[1]])
        }
    }

    impl WasmJSToWasmWrapperDescriptor {
        pub const fn registers() -> RegisterArray<1> {
            // Arbitrarily picked register.
            RegisterArray([rdi])
        }
    }
}

pub mod register {
    pub use crate::codegen::register::*;
}

mod codegen {
    pub mod interface_descriptors {
        pub use crate::interface_descriptors::*;
    }
    pub mod register {
        pub use crate::register::*;
    }
}

mod interface_descriptors {
    pub use crate::interface_descriptors::*;
}

mod register {
    pub use crate::register::*;
}