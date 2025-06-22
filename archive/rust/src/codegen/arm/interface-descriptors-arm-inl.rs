// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file is a translation of the C++ header file:
// /home/kathirks_gc/v8_go/codebase/src/codegen/arm/interface-descriptors-arm-inl.h

// Note: This is a simplified translation and may not be fully functional
// due to missing definitions for V8-specific types and functionality.

// Placeholder for V8_TARGET_ARCH_ARM macro.  Assuming ARM target.
const V8_TARGET_ARCH_ARM: bool = true;

#[cfg(all(V8_TARGET_ARCH_ARM))]
pub mod interface_descriptors_arm {
    use std::array;

    // Placeholder for Register, DoubleRegister, RegisterArray
    // These would need to be defined based on the ARM architecture.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Register(u32);
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct DoubleRegister(u32);

    // Placeholder functions for RegisterArray
    pub type RegisterArray<const N: usize> = array::FixedSizeArray<Register>;
    pub type DoubleRegisterArray<const N: usize> = array::FixedSizeArray<DoubleRegister>;

    // Placeholder for RegList
    #[derive(Debug, Copy, Clone)]
    pub struct RegList(u32);

    impl RegList {
        pub fn has(&self, reg: Register) -> bool {
            (self.0 & (1 << reg.0)) != 0
        }
    }

    // Placeholder register definitions
    pub const r0: Register = Register(0);
    pub const r1: Register = Register(1);
    pub const r2: Register = Register(2);
    pub const r3: Register = Register(3);
    pub const r4: Register = Register(4);
    pub const r5: Register = Register(5);
    pub const r6: Register = Register(6);
    pub const r7: Register = Register(7);
    pub const r8: Register = Register(8);

    pub const kReturnRegister0: Register = r0;
    pub const kReturnRegister1: Register = r1;
    pub const kReturnRegister2: Register = r2;
    pub const kContextRegister: Register = r6;
    pub const kInterpreterAccumulatorRegister: Register = r0;
    pub const kInterpreterBytecodeOffsetRegister: Register = r1;
    pub const kInterpreterBytecodeArrayRegister: Register = r2;
    pub const kInterpreterDispatchTableRegister: Register = r3;

    pub const d0: DoubleRegister = DoubleRegister(0);
    pub const d1: DoubleRegister = DoubleRegister(1);
    pub const d2: DoubleRegister = DoubleRegister(2);
    pub const d3: DoubleRegister = DoubleRegister(3);
    pub const d4: DoubleRegister = DoubleRegister(4);
    pub const d5: DoubleRegister = DoubleRegister(5);
    pub const d6: DoubleRegister = DoubleRegister(6);

    pub const kFPReturnRegister0: DoubleRegister = d0;
    pub const no_dreg: DoubleRegister = DoubleRegister(u32::MAX); // Represents a missing DoubleRegister

    // Placeholder for CallInterfaceDescriptorData
    pub struct CallInterfaceDescriptorData {
        allocatable_registers: RegList,
    }

    impl CallInterfaceDescriptorData {
        pub fn allocatable_registers(&self) -> RegList {
            self.allocatable_registers
        }
    }

    pub const kMaxBuiltinRegisterParams: usize = 5;

    pub struct CallInterfaceDescriptor {}

    impl CallInterfaceDescriptor {
        pub const fn default_register_array() -> RegisterArray<kMaxBuiltinRegisterParams> {
            [r0, r1, r2, r3, r4]
        }

        pub const fn default_double_register_array() -> [DoubleRegister; 7] {
            [d0, d1, d2, d3, d4, d5, d6]
        }

        pub const fn default_return_register_array() -> RegisterArray<3> {
            [kReturnRegister0, kReturnRegister1, kReturnRegister2]
        }

        pub const fn default_return_double_register_array() -> [DoubleRegister; 3] {
            [kFPReturnRegister0, no_dreg, no_dreg]
        }
    }

    pub struct StaticCallInterfaceDescriptor<T> {
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> StaticCallInterfaceDescriptor<T> {
        #[cfg(debug_assertions)]
        pub fn verify_argument_register_count(data: &mut CallInterfaceDescriptorData, argc: i32) {
            let allocatable_regs = data.allocatable_registers();
            if argc >= 1 {
                debug_assert!(allocatable_regs.has(r0));
            }
            if argc >= 2 {
                debug_assert!(allocatable_regs.has(r1));
            }
            if argc >= 3 {
                debug_assert!(allocatable_regs.has(r2));
            }
            if argc >= 4 {
                debug_assert!(allocatable_regs.has(r3));
            }
            if argc >= 5 {
                debug_assert!(allocatable_regs.has(r4));
            }
            if argc >= 6 {
                debug_assert!(allocatable_regs.has(r5));
            }
            if argc >= 7 {
                debug_assert!(allocatable_regs.has(r6));
            }
            if argc >= 8 {
                debug_assert!(allocatable_regs.has(r7));
            }
            // Additional arguments are passed on the stack.
        }
    }

    pub struct WriteBarrierDescriptor {}

    impl WriteBarrierDescriptor {
        pub const fn registers() -> RegisterArray<7> {
            [r1, r5, r4, r2, r0, r3, kContextRegister]
        }
    }

    pub struct LoadDescriptor {}

    impl LoadDescriptor {
        pub const fn receiver_register() -> Register {
            r1
        }
        pub const fn name_register() -> Register {
            r2
        }
        pub const fn slot_register() -> Register {
            r0
        }
    }

    pub struct LoadWithVectorDescriptor {}

    impl LoadWithVectorDescriptor {
        pub const fn vector_register() -> Register {
            r3
        }
    }

    pub struct KeyedLoadBaselineDescriptor {}

    impl KeyedLoadBaselineDescriptor {
        pub const fn receiver_register() -> Register {
            r1
        }
        pub const fn name_register() -> Register {
            kInterpreterAccumulatorRegister
        }
        pub const fn slot_register() -> Register {
            r2
        }
    }

    pub struct KeyedLoadWithVectorDescriptor {}

    impl KeyedLoadWithVectorDescriptor {
        pub const fn vector_register() -> Register {
            r3
        }
    }

    pub struct EnumeratedKeyedLoadBaselineDescriptor {}

    impl EnumeratedKeyedLoadBaselineDescriptor {
        pub const fn enum_index_register() -> Register {
            r4
        }

        pub const fn cache_type_register() -> Register {
            r5
        }

        pub const fn slot_register() -> Register {
            r2
        }
    }

    pub struct KeyedHasICBaselineDescriptor {}

    impl KeyedHasICBaselineDescriptor {
        pub const fn receiver_register() -> Register {
            kInterpreterAccumulatorRegister
        }
        pub const fn name_register() -> Register {
            r1
        }
        pub const fn slot_register() -> Register {
            r2
        }
    }

    pub struct KeyedHasICWithVectorDescriptor {}

    impl KeyedHasICWithVectorDescriptor {
        pub const fn vector_register() -> Register {
            r3
        }
    }

    pub struct LoadWithReceiverAndVectorDescriptor {}

    impl LoadWithReceiverAndVectorDescriptor {
        pub const fn lookup_start_object_register() -> Register {
            r4
        }
    }

    pub struct StoreDescriptor {}

    impl StoreDescriptor {
        pub const fn receiver_register() -> Register {
            r1
        }
        pub const fn name_register() -> Register {
            r2
        }
        pub const fn value_register() -> Register {
            r0
        }
        pub const fn slot_register() -> Register {
            r4
        }
    }

    pub struct StoreWithVectorDescriptor {}

    impl StoreWithVectorDescriptor {
        pub const fn vector_register() -> Register {
            r3
        }
    }

    pub struct DefineKeyedOwnDescriptor {}

    impl DefineKeyedOwnDescriptor {
        pub const fn flags_register() -> Register {
            r5
        }
    }

    pub struct StoreTransitionDescriptor {}

    impl StoreTransitionDescriptor {
        pub const fn map_register() -> Register {
            r5
        }
    }

    pub struct ApiGetterDescriptor {}

    impl ApiGetterDescriptor {
        pub const fn holder_register() -> Register {
            r0
        }
        pub const fn callback_register() -> Register {
            r3
        }
    }

    pub struct GrowArrayElementsDescriptor {}

    impl GrowArrayElementsDescriptor {
        pub const fn object_register() -> Register {
            r0
        }
        pub const fn key_register() -> Register {
            r3
        }
    }

    pub struct BaselineLeaveFrameDescriptor {}

    impl BaselineLeaveFrameDescriptor {
        pub const fn params_size_register() -> Register {
            r3
        }
        pub const fn weight_register() -> Register {
            r4
        }
    }

    pub struct TypeConversionDescriptor {}

    impl TypeConversionDescriptor {
        pub const fn argument_register() -> Register {
            r0
        }
    }

    pub struct TypeofDescriptor {}

    impl TypeofDescriptor {
        pub const fn registers() -> RegisterArray<1> {
            [r0]
        }
    }

    pub struct MaglevOptimizeCodeOrTailCallOptimizedCodeSlotDescriptor {}

    impl MaglevOptimizeCodeOrTailCallOptimizedCodeSlotDescriptor {
        pub const fn flags_register() -> Register {
            r2
        }
        pub const fn feedback_vector_register() -> Register {
            r5
        }
        pub const fn temporary_register() -> Register {
            r4
        }
    }

    pub struct CallTrampolineDescriptor {}

    impl CallTrampolineDescriptor {
        pub const fn registers() -> RegisterArray<2> {
            // r0 : number of arguments
            // r1 : the target to call
            [r1, r0]
        }
    }

    pub struct CopyDataPropertiesWithExcludedPropertiesDescriptor {}

    impl CopyDataPropertiesWithExcludedPropertiesDescriptor {
        pub const fn registers() -> RegisterArray<2> {
            // r0 : the source
            // r1 : the excluded property count
            [r1, r0]
        }
    }

    pub struct CopyDataPropertiesWithExcludedPropertiesOnStackDescriptor {}

    impl CopyDataPropertiesWithExcludedPropertiesOnStackDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            // r0 : the source
            // r1 : the excluded property count
            // r2 : the excluded property base
            [r1, r0, r2]
        }
    }

    pub struct CallVarargsDescriptor {}

    impl CallVarargsDescriptor {
        pub const fn registers() -> RegisterArray<4> {
            // r0 : number of arguments (on the stack)
            // r1 : the target to call
            // r4 : arguments list length (untagged)
            // r2 : arguments list (FixedArray)
            [r1, r0, r4, r2]
        }
    }

    pub struct CallForwardVarargsDescriptor {}

    impl CallForwardVarargsDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            // r0 : number of arguments
            // r2 : start index (to support rest parameters)
            // r1 : the target to call
            [r1, r0, r2]
        }
    }

    pub struct CallFunctionTemplateDescriptor {}

    impl CallFunctionTemplateDescriptor {
        pub const fn registers() -> RegisterArray<2> {
            // r1 : function template info
            // r2 : number of arguments (on the stack)
            [r1, r2]
        }
    }

    pub struct CallFunctionTemplateGenericDescriptor {}

    impl CallFunctionTemplateGenericDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            // r1 : function template info
            // r2 : number of arguments (on the stack)
            // r3 : topmost script-having context
            [r1, r2, r3]
        }
    }

    pub struct CallWithSpreadDescriptor {}

    impl CallWithSpreadDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            // r0 : number of arguments (on the stack)
            // r1 : the target to call
            // r2 : the object to spread
            [r1, r0, r2]
        }
    }

    pub struct CallWithArrayLikeDescriptor {}

    impl CallWithArrayLikeDescriptor {
        pub const fn registers() -> RegisterArray<2> {
            // r1 : the target to call
            // r2 : the arguments list
            [r1, r2]
        }
    }

    pub struct ConstructVarargsDescriptor {}

    impl ConstructVarargsDescriptor {
        pub const fn registers() -> RegisterArray<5> {
            // r0 : number of arguments (on the stack)
            // r1 : the target to call
            // r3 : the new target
            // r4 : arguments list length (untagged)
            // r2 : arguments list (FixedArray)
            [r1, r3, r0, r4, r2]
        }
    }

    pub struct ConstructForwardVarargsDescriptor {}

    impl ConstructForwardVarargsDescriptor {
        pub const fn registers() -> RegisterArray<4> {
            // r0 : number of arguments
            // r3 : the new target
            // r2 : start index (to support rest parameters)
            // r1 : the target to call
            [r1, r3, r0, r2]
        }
    }

    pub struct ConstructWithSpreadDescriptor {}

    impl ConstructWithSpreadDescriptor {
        pub const fn registers() -> RegisterArray<4> {
            // r0 : number of arguments (on the stack)
            // r1 : the target to call
            // r3 : the new target
            // r2 : the object to spread
            [r1, r3, r0, r2]
        }
    }

    pub struct ConstructWithArrayLikeDescriptor {}

    impl ConstructWithArrayLikeDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            // r1 : the target to call
            // r3 : the new target
            // r2 : the arguments list
            [r1, r3, r2]
        }
    }

    pub struct ConstructStubDescriptor {}

    impl ConstructStubDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            // r0 : number of arguments
            // r1 : the target to call
            // r3 : the new target
            [r1, r3, r0]
        }
    }

    pub struct AbortDescriptor {}

    impl AbortDescriptor {
        pub const fn registers() -> RegisterArray<1> {
            [r1]
        }
    }

    pub struct CompareDescriptor {}

    impl CompareDescriptor {
        pub const fn registers() -> RegisterArray<2> {
            [r1, r0]
        }
    }

    pub struct Compare_BaselineDescriptor {}

    impl Compare_BaselineDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            // r1: left operand
            // r0: right operand
            // r2: feedback slot
            [r1, r0, r2]
        }
    }

    pub struct BinaryOpDescriptor {}

    impl BinaryOpDescriptor {
        pub const fn registers() -> RegisterArray<2> {
            [r1, r0]
        }
    }

    pub struct BinaryOp_BaselineDescriptor {}

    impl BinaryOp_BaselineDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            // r1: left operand
            // r0: right operand
            // r2: feedback slot
            [r1, r0, r2]
        }
    }

    pub struct BinarySmiOp_BaselineDescriptor {}

    impl BinarySmiOp_BaselineDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            // r0: left operand
            // r1: right operand
            // r2: feedback slot
            [r0, r1, r2]
        }
    }

    pub struct CallApiCallbackOptimizedDescriptor {}

    impl CallApiCallbackOptimizedDescriptor {
        pub const fn api_function_address_register() -> Register {
            r1
        }
        pub const fn actual_arguments_count_register() -> Register {
            r2
        }
        pub const fn function_template_info_register() -> Register {
            r3
        }
    }

    pub struct CallApiCallbackGenericDescriptor {}

    impl CallApiCallbackGenericDescriptor {
        pub const fn actual_arguments_count_register() -> Register {
            r2
        }
        pub const fn topmost_script_having_context_register() -> Register {
            r1
        }
        pub const fn function_template_info_register() -> Register {
            r3
        }
    }

    pub struct InterpreterDispatchDescriptor {}

    impl InterpreterDispatchDescriptor {
        pub const fn registers() -> RegisterArray<4> {
            [
                kInterpreterAccumulatorRegister,
                kInterpreterBytecodeOffsetRegister,
                kInterpreterBytecodeArrayRegister,
                kInterpreterDispatchTableRegister,
            ]
        }
    }

    pub struct InterpreterPushArgsThenCallDescriptor {}

    impl InterpreterPushArgsThenCallDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            [
                r0, // argument count
                r2, // address of first argument
                r1, // the target callable to be call
            ]
        }
    }

    pub struct InterpreterPushArgsThenConstructDescriptor {}

    impl InterpreterPushArgsThenConstructDescriptor {
        pub const fn registers() -> RegisterArray<5> {
            [
                r0, // argument count
                r4, // address of the first argument
                r1, // constructor to call
                r3, // new target
                r2, // allocation site feedback if available, undefined otherwise
            ]
        }
    }

    pub struct ConstructForwardAllArgsDescriptor {}

    impl ConstructForwardAllArgsDescriptor {
        pub const fn registers() -> RegisterArray<2> {
            [r1, r3] // constructor to call, new target
        }
    }

    pub struct ResumeGeneratorDescriptor {}

    impl ResumeGeneratorDescriptor {
        pub const fn registers() -> RegisterArray<2> {
            [r0, r1] // the value to pass to the generator, the JSGeneratorObject to resume
        }
    }

    pub struct RunMicrotasksEntryDescriptor {}

    impl RunMicrotasksEntryDescriptor {
        pub const fn registers() -> RegisterArray<2> {
            [r0, r1]
        }
    }

    pub struct WasmJSToWasmWrapperDescriptor {}

    impl WasmJSToWasmWrapperDescriptor {
        pub const fn registers() -> RegisterArray<1> {
            // Arbitrarily picked register.
            [r8]
        }
    }
}