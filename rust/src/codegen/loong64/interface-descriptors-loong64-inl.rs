// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This Rust code is an adaptation of the C++ header file
// `src/codegen/loong64/interface-descriptors-loong64-inl.h` from the V8
// JavaScript engine. It aims to provide equivalent functionality in Rust.

#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

// The following cfg attribute simulates the V8_TARGET_ARCH_LOONG64 macro.
#[cfg(target_arch = "loongarch64")]
pub mod loong64_interface_descriptors {
    use crate::codegen::interface_descriptors::*;
    use crate::execution::frames::*;

    // Assuming Register, DoubleRegister, RegList, and other types are defined elsewhere,
    // possibly in the files interface-descriptors.h and frames.h

    impl CallInterfaceDescriptor {
        pub const fn default_register_array() -> RegisterArray {
            let registers = RegisterArray([a0, a1, a2, a3, a4]);
            assert_eq!(registers.0.len(), kMaxBuiltinRegisterParams);
            registers
        }

        pub const fn default_double_register_array() -> DoubleRegisterArray {
            DoubleRegisterArray([f0, f1, f2, f3, f4, f5, f6])
        }

        pub const fn default_return_register_array() -> RegisterArray {
            RegisterArray([kReturnRegister0, kReturnRegister1, kReturnRegister2])
        }

        pub const fn default_return_double_register_array() -> DoubleRegisterArray {
            DoubleRegisterArray([kFPReturnRegister0, no_dreg, no_dreg])
        }
    }

    #[cfg(debug_assertions)]
    pub trait StaticCallInterfaceDescriptorTrait {
        fn verify_argument_register_count(data: &mut CallInterfaceDescriptorData, argc: i32);
    }

    #[cfg(debug_assertions)]
    impl<T: StaticCallInterfaceDescriptorTrait> StaticCallInterfaceDescriptorTrait for T {
        fn verify_argument_register_count(data: &mut CallInterfaceDescriptorData, argc: i32) {
            let allocatable_regs = data.allocatable_registers();
            if argc >= 1 {
                assert!(allocatable_regs.has(a0));
            }
            if argc >= 2 {
                assert!(allocatable_regs.has(a1));
            }
            if argc >= 3 {
                assert!(allocatable_regs.has(a2));
            }
            if argc >= 4 {
                assert!(allocatable_regs.has(a3));
            }
            if argc >= 5 {
                assert!(allocatable_regs.has(a4));
            }
            if argc >= 6 {
                assert!(allocatable_regs.has(a5));
            }
            if argc >= 7 {
                assert!(allocatable_regs.has(a6));
            }
            if argc >= 8 {
                assert!(allocatable_regs.has(a7));
            }
            // Additional arguments are passed on the stack.
        }
    }

    impl WriteBarrierDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray([a1, a5, a4, a2, a0, a3, kContextRegister])
        }
    }

    impl LoadDescriptor {
        pub const fn receiver_register() -> Register {
            a1
        }
        pub const fn name_register() -> Register {
            a2
        }
        pub const fn slot_register() -> Register {
            a0
        }
    }

    impl LoadWithVectorDescriptor {
        pub const fn vector_register() -> Register {
            a3
        }
    }

    impl KeyedLoadBaselineDescriptor {
        pub const fn receiver_register() -> Register {
            a1
        }
        pub const fn name_register() -> Register {
            kInterpreterAccumulatorRegister
        }
        pub const fn slot_register() -> Register {
            a2
        }
    }

    impl KeyedLoadWithVectorDescriptor {
        pub const fn vector_register() -> Register {
            a3
        }
    }

    impl EnumeratedKeyedLoadBaselineDescriptor {
        pub const fn enum_index_register() -> Register {
            a4
        }

        pub const fn cache_type_register() -> Register {
            a5
        }

        pub const fn slot_register() -> Register {
            a2
        }
    }

    impl KeyedHasICBaselineDescriptor {
        pub const fn receiver_register() -> Register {
            kInterpreterAccumulatorRegister
        }
        pub const fn name_register() -> Register {
            a1
        }
        pub const fn slot_register() -> Register {
            a2
        }
    }

    impl KeyedHasICWithVectorDescriptor {
        pub const fn vector_register() -> Register {
            a3
        }
    }

    impl LoadWithReceiverAndVectorDescriptor {
        pub const fn lookup_start_object_register() -> Register {
            a4
        }
    }

    impl StoreDescriptor {
        pub const fn receiver_register() -> Register {
            a1
        }
        pub const fn name_register() -> Register {
            a2
        }
        pub const fn value_register() -> Register {
            a0
        }
        pub const fn slot_register() -> Register {
            a4
        }
    }

    impl StoreWithVectorDescriptor {
        pub const fn vector_register() -> Register {
            a3
        }
    }

    impl DefineKeyedOwnDescriptor {
        pub const fn flags_register() -> Register {
            a5
        }
    }

    impl StoreTransitionDescriptor {
        pub const fn map_register() -> Register {
            a5
        }
    }

    impl ApiGetterDescriptor {
        pub const fn holder_register() -> Register {
            a0
        }
        pub const fn callback_register() -> Register {
            a3
        }
    }

    impl GrowArrayElementsDescriptor {
        pub const fn object_register() -> Register {
            a0
        }
        pub const fn key_register() -> Register {
            a3
        }
    }

    impl BaselineLeaveFrameDescriptor {
        pub const fn params_size_register() -> Register {
            a2
        }
    }

    impl BaselineLeaveFrameDescriptor {
        pub const fn weight_register() -> Register {
            a3
        }
    }

    impl TypeConversionDescriptor {
        pub const fn argument_register() -> Register {
            a0
        }
    }

    impl TypeofDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray([a0])
        }
    }

    impl CallTrampolineDescriptor {
        pub const fn registers() -> RegisterArray {
            // a1: target
            // a0: number of arguments
            RegisterArray([a1, a0])
        }
    }

    impl CopyDataPropertiesWithExcludedPropertiesDescriptor {
        pub const fn registers() -> RegisterArray {
            // a1 : the source
            // a0 : the excluded property count
            RegisterArray([a1, a0])
        }
    }

    impl CopyDataPropertiesWithExcludedPropertiesOnStackDescriptor {
        pub const fn registers() -> RegisterArray {
            // a1 : the source
            // a0 : the excluded property count
            // a2 : the excluded property base
            RegisterArray([a1, a0, a2])
        }
    }

    impl CallVarargsDescriptor {
        pub const fn registers() -> RegisterArray {
            // a0 : number of arguments (on the stack)
            // a1 : the target to call
            // a4 : arguments list length (untagged)
            // a2 : arguments list (FixedArray)
            RegisterArray([a1, a0, a4, a2])
        }
    }

    impl CallForwardVarargsDescriptor {
        pub const fn registers() -> RegisterArray {
            // a1: the target to call
            // a0: number of arguments
            // a2: start index (to support rest parameters)
            RegisterArray([a1, a0, a2])
        }
    }

    impl CallFunctionTemplateDescriptor {
        pub const fn registers() -> RegisterArray {
            // a1 : function template info
            // a0 : number of arguments (on the stack)
            RegisterArray([a1, a0])
        }
    }

    impl CallFunctionTemplateGenericDescriptor {
        pub const fn registers() -> RegisterArray {
            // a1 : function template info
            // a2 : number of arguments (on the stack)
            // a3 : topmost script-having context
            RegisterArray([a1, a2, a3])
        }
    }

    impl CallWithSpreadDescriptor {
        pub const fn registers() -> RegisterArray {
            // a0 : number of arguments (on the stack)
            // a1 : the target to call
            // a2 : the object to spread
            RegisterArray([a1, a0, a2])
        }
    }

    impl CallWithArrayLikeDescriptor {
        pub const fn registers() -> RegisterArray {
            // a1 : the target to call
            // a2 : the arguments list
            RegisterArray([a1, a2])
        }
    }

    impl ConstructVarargsDescriptor {
        pub const fn registers() -> RegisterArray {
            // a0 : number of arguments (on the stack)
            // a1 : the target to call
            // a3 : the new target
            // a4 : arguments list length (untagged)
            // a2 : arguments list (FixedArray)
            RegisterArray([a1, a3, a0, a4, a2])
        }
    }

    impl ConstructForwardVarargsDescriptor {
        pub const fn registers() -> RegisterArray {
            // a1: the target to call
            // a3: new target
            // a0: number of arguments
            // a2: start index (to support rest parameters)
            RegisterArray([a1, a3, a0, a2])
        }
    }

    impl ConstructWithSpreadDescriptor {
        pub const fn registers() -> RegisterArray {
            // a0 : number of arguments (on the stack)
            // a1 : the target to call
            // a3 : the new target
            // a2 : the object to spread
            RegisterArray([a1, a3, a0, a2])
        }
    }

    impl ConstructWithArrayLikeDescriptor {
        pub const fn registers() -> RegisterArray {
            // a1 : the target to call
            // a3 : the new target
            // a2 : the arguments list
            RegisterArray([a1, a3, a2])
        }
    }

    impl ConstructStubDescriptor {
        pub const fn registers() -> RegisterArray {
            // a1: target
            // a3: new target
            // a0: number of arguments
            RegisterArray([a1, a3, a0])
        }
    }

    impl AbortDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray([a0])
        }
    }

    impl CompareDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray([a1, a0])
        }
    }

    impl Compare_BaselineDescriptor {
        pub const fn registers() -> RegisterArray {
            // a1: left operand
            // a0: right operand
            // a2: feedback slot
            RegisterArray([a1, a0, a2])
        }
    }

    impl BinaryOpDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray([a1, a0])
        }
    }

    impl BinaryOp_BaselineDescriptor {
        pub const fn registers() -> RegisterArray {
            // a1: left operand
            // a0: right operand
            // a2: feedback slot
            RegisterArray([a1, a0, a2])
        }
    }

    impl BinarySmiOp_BaselineDescriptor {
        pub const fn registers() -> RegisterArray {
            // a0: left operand
            // a1: right operand
            // a2: feedback slot
            RegisterArray([a0, a1, a2])
        }
    }

    impl CallApiCallbackOptimizedDescriptor {
        pub const fn api_function_address_register() -> Register {
            a1
        }
        pub const fn actual_arguments_count_register() -> Register {
            a2
        }
        pub const fn function_template_info_register() -> Register {
            a3
        }
    }

    impl CallApiCallbackGenericDescriptor {
        pub const fn topmost_script_having_context_register() -> Register {
            a1
        }
        pub const fn actual_arguments_count_register() -> Register {
            a2
        }
        pub const fn function_template_info_register() -> Register {
            a3
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
            // a0 : argument count (not including receiver)
            // a2 : address of first argument
            // a1 : the target callable to be call
            RegisterArray([a0, a2, a1])
        }
    }

    impl InterpreterPushArgsThenConstructDescriptor {
        pub const fn registers() -> RegisterArray {
            // a0 : argument count
            // a4 : address of the first argument
            // a1 : constructor to call
            // a3 : new target
            // a2 : allocation site feedback if available, undefined otherwise
            RegisterArray([a0, a4, a1, a3, a2])
        }
    }

    impl ConstructForwardAllArgsDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray([
                a1, // constructor to call
                a3, // new target
            ])
        }
    }

    impl ResumeGeneratorDescriptor {
        pub const fn registers() -> RegisterArray {
            // v0 : the value to pass to the generator
            // a1 : the JSGeneratorObject to resume
            RegisterArray([a0, a1])
        }
    }

    impl RunMicrotasksEntryDescriptor {
        pub const fn registers() -> RegisterArray {
            RegisterArray([a0, a1])
        }
    }

    impl WasmJSToWasmWrapperDescriptor {
        pub const fn registers() -> RegisterArray {
            // Arbitrarily picked register.
            RegisterArray([t0])
        }
    }

    // Dummy Definitions
    #[derive(Debug, Copy, Clone)]
    pub struct Register(u32);
    #[derive(Debug, Copy, Clone)]
    pub struct DoubleRegister(u32);

    #[derive(Debug, Copy, Clone)]
    pub struct RegisterArray([Register; 7]);

    #[derive(Debug, Copy, Clone)]
    pub struct DoubleRegisterArray([DoubleRegister; 7]);
    impl DoubleRegisterArray{
        pub const fn new() -> Self {
            DoubleRegisterArray([DoubleRegister(0);7])
        }
    }

    impl RegisterArray {
        pub const fn new() -> Self {
            RegisterArray([Register(0); 7])
        }
    }

    impl RegisterArray {
        pub fn has(&self, reg: Register) -> bool {
            self.0.iter().any(|&r| r.0 == reg.0)
        }
    }

    pub struct CallInterfaceDescriptor;
    pub struct CallInterfaceDescriptorData;
    pub struct WriteBarrierDescriptor;
    pub struct LoadDescriptor;
    pub struct LoadWithVectorDescriptor;
    pub struct KeyedLoadBaselineDescriptor;
    pub struct KeyedLoadWithVectorDescriptor;
    pub struct EnumeratedKeyedLoadBaselineDescriptor;
    pub struct KeyedHasICBaselineDescriptor;
    pub struct KeyedHasICWithVectorDescriptor;
    pub struct LoadWithReceiverAndVectorDescriptor;
    pub struct StoreDescriptor;
    pub struct StoreWithVectorDescriptor;
    pub struct DefineKeyedOwnDescriptor;
    pub struct StoreTransitionDescriptor;
    pub struct ApiGetterDescriptor;
    pub struct GrowArrayElementsDescriptor;
    pub struct BaselineLeaveFrameDescriptor;
    pub struct TypeConversionDescriptor;
    pub struct TypeofDescriptor;
    pub struct CallTrampolineDescriptor;
    pub struct CopyDataPropertiesWithExcludedPropertiesDescriptor;
    pub struct CopyDataPropertiesWithExcludedPropertiesOnStackDescriptor;
    pub struct CallVarargsDescriptor;
    pub struct CallForwardVarargsDescriptor;
    pub struct CallFunctionTemplateDescriptor;
    pub struct CallFunctionTemplateGenericDescriptor;
    pub struct CallWithSpreadDescriptor;
    pub struct CallWithArrayLikeDescriptor;
    pub struct ConstructVarargsDescriptor;
    pub struct ConstructForwardVarargsDescriptor;
    pub struct ConstructWithSpreadDescriptor;
    pub struct ConstructWithArrayLikeDescriptor;
    pub struct ConstructStubDescriptor;
    pub struct AbortDescriptor;
    pub struct CompareDescriptor;
    pub struct Compare_BaselineDescriptor;
    pub struct BinaryOpDescriptor;
    pub struct BinaryOp_BaselineDescriptor;
    pub struct BinarySmiOp_BaselineDescriptor;
    pub struct CallApiCallbackOptimizedDescriptor;
    pub struct CallApiCallbackGenericDescriptor;
    pub struct InterpreterDispatchDescriptor;
    pub struct InterpreterPushArgsThenCallDescriptor;
    pub struct InterpreterPushArgsThenConstructDescriptor;
    pub struct ConstructForwardAllArgsDescriptor;
    pub struct ResumeGeneratorDescriptor;
    pub struct RunMicrotasksEntryDescriptor;
    pub struct WasmJSToWasmWrapperDescriptor;

    const a0: Register = Register(0);
    const a1: Register = Register(1);
    const a2: Register = Register(2);
    const a3: Register = Register(3);
    const a4: Register = Register(4);
    const a5: Register = Register(5);
    const a6: Register = Register(6);
    const a7: Register = Register(7);
    const t0: Register = Register(8);

    const f0: DoubleRegister = DoubleRegister(0);
    const f1: DoubleRegister = DoubleRegister(1);
    const f2: DoubleRegister = DoubleRegister(2);
    const f3: DoubleRegister = DoubleRegister(3);
    const f4: DoubleRegister = DoubleRegister(4);
    const f5: DoubleRegister = DoubleRegister(5);
    const f6: DoubleRegister = DoubleRegister(6);
    const no_dreg: DoubleRegister = DoubleRegister(100);

    const kReturnRegister0: Register = Register(9);
    const kReturnRegister1: Register = Register(10);
    const kReturnRegister2: Register = Register(11);
    const kFPReturnRegister0: DoubleRegister = DoubleRegister(7);
    const kContextRegister: Register = Register(12);
    const kInterpreterAccumulatorRegister: Register = Register(13);
    const kInterpreterBytecodeOffsetRegister: Register = Register(14);
    const kInterpreterBytecodeArrayRegister: Register = Register(15);
    const kInterpreterDispatchTableRegister: Register = Register(16);
    const kMaxBuiltinRegisterParams: usize = 5;
}

pub mod codegen {
    pub mod interface_descriptors {
        pub struct InterfaceDescriptorData {}
        pub struct CallInterfaceDescriptorData {
            allocatable_registers: RegList,
        }

        impl CallInterfaceDescriptorData {
            pub fn allocatable_registers(&mut self) -> &RegList {
                &self.allocatable_registers
            }
        }

        pub struct RegList(u64);

        impl RegList {
            pub fn has(&self, reg: crate::loong64_interface_descriptors::Register) -> bool {
                (self.0 & (1 << reg.0)) != 0
            }
        }
    }
}

pub mod execution {
    pub mod frames {
        pub struct Frame {}
    }
}