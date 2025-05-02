// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file is a Rust translation of the C++ header file
// `src/codegen/ppc/interface-descriptors-ppc-inl.h` from the V8 JavaScript engine codebase.

#![cfg(target_arch = "powerpc64")]

mod frames;
mod interface_descriptors;

pub mod ppc {
    use super::*;
    use interface_descriptors::*;

    pub type Register = u32; // Placeholder for Register type
    pub type DoubleRegister = u32; // Placeholder for DoubleRegister type
    pub type RegList = u64;   // Placeholder for RegList type

    pub const kMaxBuiltinRegisterParams: usize = 5;

    // Placeholder registers.  Need to replace with actual register definitions.
    pub const r3: Register = 3;
    pub const r4: Register = 4;
    pub const r5: Register = 5;
    pub const r6: Register = 6;
    pub const r7: Register = 7;
    pub const r8: Register = 8;
    pub const r9: Register = 9;
    pub const r10: Register = 10;
    pub const r14: Register = 14;
    pub const kContextRegister: Register = 15;  // Example Context Register Value

    pub const d1: DoubleRegister = 1;
    pub const d2: DoubleRegister = 2;
    pub const d3: DoubleRegister = 3;
    pub const d4: DoubleRegister = 4;
    pub const d5: DoubleRegister = 5;
    pub const d6: DoubleRegister = 6;
    pub const d7: DoubleRegister = 7;
    pub const no_dreg: DoubleRegister = 0; // Placeholder

    pub const kReturnRegister0: Register = 3;   // Example return register
    pub const kReturnRegister1: Register = 4;   // Example return register
    pub const kReturnRegister2: Register = 5;   // Example return register
    pub const kFPReturnRegister0: DoubleRegister = 1;

    pub const kInterpreterAccumulatorRegister: Register = 16;  //Example value
    pub const kInterpreterBytecodeOffsetRegister: Register = 17; //Example value
    pub const kInterpreterBytecodeArrayRegister: Register = 18; //Example value
    pub const kInterpreterDispatchTableRegister: Register = 19; //Example value

    impl CallInterfaceDescriptor {
        pub const fn default_register_array() -> RegisterArray<kMaxBuiltinRegisterParams> {
            RegisterArray::new([r3, r4, r5, r6, r7])
        }

        pub const fn default_double_register_array() -> DoubleRegisterArray<7> {
            DoubleRegisterArray::new([d1, d2, d3, d4, d5, d6, d7])
        }

        pub const fn default_return_register_array() -> RegisterArray<3> {
            RegisterArray::new([kReturnRegister0, kReturnRegister1, kReturnRegister2])
        }

        pub const fn default_return_double_register_array() -> DoubleRegisterArray<3> {
            DoubleRegisterArray::new([kFPReturnRegister0, no_dreg, no_dreg])
        }
    }

    #[cfg(debug_assertions)]
    pub trait StaticCallInterfaceDescriptorTrait {
        fn verify_argument_register_count(data: &mut CallInterfaceDescriptorData, argc: i32);
    }

    #[cfg(debug_assertions)]
    impl<T: CallInterfaceDescriptorTrait> StaticCallInterfaceDescriptorTrait for T {
        fn verify_argument_register_count(data: &mut CallInterfaceDescriptorData, argc: i32) {
            let allocatable_regs = data.allocatable_registers();
            if argc >= 1 {
                assert!(allocatable_regs & (1 << r3) != 0);
            }
            if argc >= 2 {
                assert!(allocatable_regs & (1 << r4) != 0);
            }
            if argc >= 3 {
                assert!(allocatable_regs & (1 << r5) != 0);
            }
            if argc >= 4 {
                assert!(allocatable_regs & (1 << r6) != 0);
            }
            if argc >= 5 {
                assert!(allocatable_regs & (1 << r7) != 0);
            }
            if argc >= 6 {
                assert!(allocatable_regs & (1 << r8) != 0);
            }
            if argc >= 7 {
                assert!(allocatable_regs & (1 << r9) != 0);
            }
            if argc >= 8 {
                assert!(allocatable_regs & (1 << r10) != 0);
            }
            // Additional arguments are passed on the stack.
        }
    }

    impl WriteBarrierDescriptor {
        pub const fn registers() -> RegisterArray<7> {
            RegisterArray::new([r4, r8, r7, r5, r3, r6, kContextRegister])
        }
    }

    impl LoadDescriptor {
        pub const fn receiver_register() -> Register {
            r4
        }
        pub const fn name_register() -> Register {
            r5
        }
        pub const fn slot_register() -> Register {
            r3
        }
    }

    impl LoadWithVectorDescriptor {
        pub const fn vector_register() -> Register {
            r6
        }
    }

    impl KeyedLoadBaselineDescriptor {
        pub const fn receiver_register() -> Register {
            r4
        }
        pub const fn name_register() -> Register {
            kInterpreterAccumulatorRegister
        }
        pub const fn slot_register() -> Register {
            r5
        }
    }

    impl KeyedLoadWithVectorDescriptor {
        pub const fn vector_register() -> Register {
            r6
        }
    }

    impl EnumeratedKeyedLoadBaselineDescriptor {
        pub const fn enum_index_register() -> Register {
            r7
        }

        pub const fn cache_type_register() -> Register {
            r8
        }

        pub const fn slot_register() -> Register {
            r5
        }
    }

    impl KeyedHasICBaselineDescriptor {
        pub const fn receiver_register() -> Register {
            kInterpreterAccumulatorRegister
        }
        pub const fn name_register() -> Register {
            r4
        }
        pub const fn slot_register() -> Register {
            r5
        }
    }

    impl KeyedHasICWithVectorDescriptor {
        pub const fn vector_register() -> Register {
            r6
        }
    }

    impl LoadWithReceiverAndVectorDescriptor {
        pub const fn lookup_start_object_register() -> Register {
            r7
        }
    }

    impl StoreDescriptor {
        pub const fn receiver_register() -> Register {
            r4
        }
        pub const fn name_register() -> Register {
            r5
        }
        pub const fn value_register() -> Register {
            r3
        }
        pub const fn slot_register() -> Register {
            r7
        }
    }

    impl StoreWithVectorDescriptor {
        pub const fn vector_register() -> Register {
            r6
        }
    }

    impl DefineKeyedOwnDescriptor {
        pub const fn flags_register() -> Register {
            r8
        }
    }

    impl StoreTransitionDescriptor {
        pub const fn map_register() -> Register {
            r8
        }
    }

    impl ApiGetterDescriptor {
        pub const fn holder_register() -> Register {
            r3
        }
        pub const fn callback_register() -> Register {
            r6
        }
    }

    impl GrowArrayElementsDescriptor {
        pub const fn object_register() -> Register {
            r3
        }
        pub const fn key_register() -> Register {
            r6
        }
    }

    impl BaselineLeaveFrameDescriptor {
        pub const fn params_size_register() -> Register {
            r6
        }
        pub const fn weight_register() -> Register {
            r7
        }
    }

    impl TypeConversionDescriptor {
        pub const fn argument_register() -> Register {
            r3
        }
    }

    impl TypeofDescriptor {
        pub const fn registers() -> RegisterArray<1> {
            RegisterArray::new([r3])
        }
    }

    impl CallTrampolineDescriptor {
        pub const fn registers() -> RegisterArray<2> {
            // r3 : number of arguments
            // r4 : the target to call
            RegisterArray::new([r4, r3])
        }
    }

    impl CopyDataPropertiesWithExcludedPropertiesDescriptor {
        pub const fn registers() -> RegisterArray<2> {
            // r4 : the source
            // r3 : the excluded property count
            RegisterArray::new([r4, r3])
        }
    }

    impl CopyDataPropertiesWithExcludedPropertiesOnStackDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            // r4 : the source
            // r3 : the excluded property count
            // r5 : the excluded property base
            RegisterArray::new([r4, r3, r5])
        }
    }

    impl CallVarargsDescriptor {
        pub const fn registers() -> RegisterArray<4> {
            // r3 : number of arguments (on the stack)
            // r4 : the target to call
            // r7 : arguments list length (untagged)
            // r5 : arguments list (FixedArray)
            RegisterArray::new([r4, r3, r7, r5])
        }
    }

    impl CallForwardVarargsDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            // r3 : number of arguments
            // r5 : start index (to support rest parameters)
            // r4 : the target to call
            RegisterArray::new([r4, r3, r5])
        }
    }

    impl CallFunctionTemplateDescriptor {
        pub const fn registers() -> RegisterArray<2> {
            // r4 : function template info
            // r5 : number of arguments (on the stack)
            RegisterArray::new([r4, r5])
        }
    }

    impl CallFunctionTemplateGenericDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            // r4 : function template info
            // r5 : number of arguments (on the stack)
            // r6 : topmost script-having context
            RegisterArray::new([r4, r5, r6])
        }
    }

    impl CallWithSpreadDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            // r3 : number of arguments (on the stack)
            // r4 : the target to call
            // r5 : the object to spread
            RegisterArray::new([r4, r3, r5])
        }
    }

    impl CallWithArrayLikeDescriptor {
        pub const fn registers() -> RegisterArray<2> {
            // r4 : the target to call
            // r5 : the arguments list
            RegisterArray::new([r4, r5])
        }
    }

    impl ConstructVarargsDescriptor {
        pub const fn registers() -> RegisterArray<5> {
            // r3 : number of arguments (on the stack)
            // r4 : the target to call
            // r6 : the new target
            // r7 : arguments list length (untagged)
            // r5 : arguments list (FixedArray)
            RegisterArray::new([r4, r6, r3, r7, r5])
        }
    }

    impl ConstructForwardVarargsDescriptor {
        pub const fn registers() -> RegisterArray<4> {
            // r3 : number of arguments
            // r6 : the new target
            // r5 : start index (to support rest parameters)
            // r4 : the target to call
            RegisterArray::new([r4, r6, r3, r5])
        }
    }

    impl ConstructWithSpreadDescriptor {
        pub const fn registers() -> RegisterArray<4> {
            // r3 : number of arguments (on the stack)
            // r4 : the target to call
            // r6 : the new target
            // r5 : the object to spread
            RegisterArray::new([r4, r6, r3, r5])
        }
    }

    impl ConstructWithArrayLikeDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            // r4 : the target to call
            // r6 : the new target
            // r5 : the arguments list
            RegisterArray::new([r4, r6, r5])
        }
    }

    impl ConstructStubDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            // r3 : number of arguments
            // r4 : the target to call
            // r6 : the new target
            RegisterArray::new([r4, r6, r3])
        }
    }

    impl AbortDescriptor {
        pub const fn registers() -> RegisterArray<1> {
            RegisterArray::new([r4])
        }
    }

    impl CompareDescriptor {
        pub const fn registers() -> RegisterArray<2> {
            RegisterArray::new([r4, r3])
        }
    }

    impl Compare_BaselineDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            RegisterArray::new([r4, r3, r5])
        }
    }

    impl BinaryOpDescriptor {
        pub const fn registers() -> RegisterArray<2> {
            RegisterArray::new([r4, r3])
        }
    }

    impl BinaryOp_BaselineDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            RegisterArray::new([r4, r3, r5])
        }
    }

    impl BinarySmiOp_BaselineDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            RegisterArray::new([r3, r4, r5])
        }
    }

    impl CallApiCallbackOptimizedDescriptor {
        pub const fn api_function_address_register() -> Register {
            r4
        }
        pub const fn actual_arguments_count_register() -> Register {
            r5
        }
        pub const fn function_template_info_register() -> Register {
            r6
        }
    }

    impl CallApiCallbackGenericDescriptor {
        pub const fn actual_arguments_count_register() -> Register {
            r5
        }
        pub const fn topmost_script_having_context_register() -> Register {
            r4
        }
        pub const fn function_template_info_register() -> Register {
            r6
        }
    }

    impl InterpreterDispatchDescriptor {
        pub const fn registers() -> RegisterArray<4> {
            RegisterArray::new([
                kInterpreterAccumulatorRegister,
                kInterpreterBytecodeOffsetRegister,
                kInterpreterBytecodeArrayRegister,
                kInterpreterDispatchTableRegister,
            ])
        }
    }

    impl InterpreterPushArgsThenCallDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            RegisterArray::new([
                r3, // argument count
                r5, // address of first argument
                r4, // the target callable to be call
            ])
        }
    }

    impl InterpreterPushArgsThenConstructDescriptor {
        pub const fn registers() -> RegisterArray<5> {
            RegisterArray::new([
                r3, // argument count
                r7, // address of the first argument
                r4, // constructor to call
                r6, // new target
                r5, // allocation site feedback if available, undefined otherwise
            ])
        }
    }

    impl ConstructForwardAllArgsDescriptor {
        pub const fn registers() -> RegisterArray<2> {
            RegisterArray::new([
                r4, // constructor to call
                r6, // new target
            ])
        }
    }

    impl ResumeGeneratorDescriptor {
        pub const fn registers() -> RegisterArray<2> {
            RegisterArray::new([
                r3, // the value to pass to the generator
                r4, // the JSGeneratorObject to resume
            ])
        }
    }

    impl RunMicrotasksEntryDescriptor {
        pub const fn registers() -> RegisterArray<2> {
            RegisterArray::new([r3, r4])
        }
    }

    impl WasmJSToWasmWrapperDescriptor {
        pub const fn registers() -> RegisterArray<1> {
            // Arbitrarily picked register.
            RegisterArray::new([r14])
        }
    }
}