// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Note: This Rust code is a translation of the C++ header file.
// Some parts might require further adaptation based on the complete V8 codebase.
// Especially, the `Register` type and related constants/operations need careful
// definition based on their usage in the V8 context.  Assumed `Register` is `u32`

#![allow(dead_code)]

// Conditional compilation based on V8_TARGET_ARCH_S390X
#[cfg(target_arch = "s390x")]
pub mod s390 {
    use std::marker::PhantomData;

    // Placeholder for architectural definitions.  Replace with actual register definitions.
    pub type Register = u32;
    pub type DoubleRegister = u32;

    // Placeholder values for registers - replace with actual S390 register IDs
    pub const r2: Register = 2;
    pub const r3: Register = 3;
    pub const r4: Register = 4;
    pub const r5: Register = 5;
    pub const r6: Register = 6;
    pub const r7: Register = 7;
    pub const r8: Register = 8;
    pub const r9: Register = 9;
    pub const kReturnRegister0: Register = 10;
    pub const kReturnRegister1: Register = 11;
    pub const kReturnRegister2: Register = 12;
    pub const kContextRegister: Register = 13;
    pub const kInterpreterAccumulatorRegister: Register = 14;
    pub const kInterpreterBytecodeOffsetRegister: Register = 15;
    pub const kInterpreterBytecodeArrayRegister: Register = 16;
    pub const kInterpreterDispatchTableRegister: Register = 17;

    pub const d1: DoubleRegister = 1;
    pub const d2: DoubleRegister = 2;
    pub const d3: DoubleRegister = 3;
    pub const d4: DoubleRegister = 4;
    pub const d5: DoubleRegister = 5;
    pub const d6: DoubleRegister = 6;
    pub const d7: DoubleRegister = 7;
    pub const kFPReturnRegister0: DoubleRegister = 8;
    pub const no_dreg: DoubleRegister = 0xFFFFFFFF;

    pub const MAX_BUILTIN_REGISTER_PARAMS: usize = 5;

    pub struct RegisterArray<const N: usize> {
        registers: [Register; N],
    }

    impl<const N: usize> RegisterArray<N> {
        pub const fn new(registers: [Register; N]) -> Self {
            RegisterArray { registers }
        }

        pub const fn size(&self) -> usize {
            N
        }
    }

    pub struct DoubleRegisterArray<const N: usize> {
        registers: [DoubleRegister; N],
    }

    impl<const N: usize> DoubleRegisterArray<N> {
        pub const fn new(registers: [DoubleRegister; N]) -> Self {
            DoubleRegisterArray { registers }
        }
    }

    pub struct CallInterfaceDescriptor;

    impl CallInterfaceDescriptor {
        pub const fn default_register_array() -> RegisterArray<MAX_BUILTIN_REGISTER_PARAMS> {
            let registers = [r2, r3, r4, r5, r6];
            RegisterArray::new(registers)
        }

        pub const fn default_double_register_array() -> DoubleRegisterArray<7> {
            let registers = [d1, d2, d3, d4, d5, d6, d7];
            DoubleRegisterArray::new(registers)
        }

        pub const fn default_return_register_array() -> RegisterArray<3> {
            let registers = [kReturnRegister0, kReturnRegister1, kReturnRegister2];
            RegisterArray::new(registers)
        }

        pub const fn default_return_double_register_array() -> DoubleRegisterArray<3> {
            let registers = [kFPReturnRegister0, no_dreg, no_dreg];
            DoubleRegisterArray::new(registers)
        }
    }

    pub struct StaticCallInterfaceDescriptor<T> {
        _phantom: PhantomData<T>,
    }

    impl<T> StaticCallInterfaceDescriptor<T> {
        #[cfg(debug_assertions)]
        pub fn verify_argument_register_count(_data: &CallInterfaceDescriptorData, argc: i32) {
            // Incomplete translation:  Requires RegList and allocatable_registers() from C++
            // and corresponding Rust equivalents.
            // The DCHECK macro is also not directly translatable; consider using assert! in debug mode.
            if argc >= 1 { /* assert!(allocatable_regs.has(r2)); */ }
            if argc >= 2 { /* assert!(allocatable_regs.has(r3)); */ }
            if argc >= 3 { /* assert!(allocatable_regs.has(r4)); */ }
            if argc >= 4 { /* assert!(allocatable_regs.has(r5)); */ }
            if argc >= 5 { /* assert!(allocatable_regs.has(r6)); */ }
            if argc >= 6 { /* assert!(allocatable_regs.has(r7)); */ }
            if argc >= 7 { /* assert!(allocatable_regs.has(r8)); */ }
            if argc >= 8 { /* assert!(allocatable_regs.has(r9)); */ }
            // Additional arguments are passed on the stack.
        }
    }

    pub struct WriteBarrierDescriptor;

    impl WriteBarrierDescriptor {
        pub const fn registers() -> RegisterArray<7> {
            RegisterArray::new([r3, r7, r6, r4, r2, r5, kContextRegister])
        }
    }

    pub struct LoadDescriptor;

    impl LoadDescriptor {
        pub const fn receiver_register() -> Register {
            r3
        }
        pub const fn name_register() -> Register {
            r4
        }
        pub const fn slot_register() -> Register {
            r2
        }
    }

    pub struct LoadWithVectorDescriptor;

    impl LoadWithVectorDescriptor {
        pub const fn vector_register() -> Register {
            r5
        }
    }

    pub struct KeyedLoadBaselineDescriptor;

    impl KeyedLoadBaselineDescriptor {
        pub const fn receiver_register() -> Register {
            r3
        }
        pub const fn name_register() -> Register {
            kInterpreterAccumulatorRegister
        }
        pub const fn slot_register() -> Register {
            r4
        }
    }

    pub struct KeyedLoadWithVectorDescriptor;

    impl KeyedLoadWithVectorDescriptor {
        pub const fn vector_register() -> Register {
            r5
        }
    }

    pub struct EnumeratedKeyedLoadBaselineDescriptor;

    impl EnumeratedKeyedLoadBaselineDescriptor {
        pub const fn enum_index_register() -> Register {
            r6
        }

        pub const fn cache_type_register() -> Register {
            r7
        }

        pub const fn slot_register() -> Register {
            r4
        }
    }

    pub struct KeyedHasICBaselineDescriptor;

    impl KeyedHasICBaselineDescriptor {
        pub const fn receiver_register() -> Register {
            kInterpreterAccumulatorRegister
        }
        pub const fn name_register() -> Register {
            r3
        }
        pub const fn slot_register() -> Register {
            r4
        }
    }

    pub struct KeyedHasICWithVectorDescriptor;

    impl KeyedHasICWithVectorDescriptor {
        pub const fn vector_register() -> Register {
            r5
        }
    }

    pub struct LoadWithReceiverAndVectorDescriptor;

    impl LoadWithReceiverAndVectorDescriptor {
        pub const fn lookup_start_object_register() -> Register {
            r6
        }
    }

    pub struct StoreDescriptor;

    impl StoreDescriptor {
        pub const fn receiver_register() -> Register {
            r3
        }
        pub const fn name_register() -> Register {
            r4
        }
        pub const fn value_register() -> Register {
            r2
        }
        pub const fn slot_register() -> Register {
            r6
        }
    }

    pub struct StoreWithVectorDescriptor;

    impl StoreWithVectorDescriptor {
        pub const fn vector_register() -> Register {
            r5
        }
    }

    pub struct DefineKeyedOwnDescriptor;

    impl DefineKeyedOwnDescriptor {
        pub const fn flags_register() -> Register {
            r7
        }
    }

    pub struct StoreTransitionDescriptor;

    impl StoreTransitionDescriptor {
        pub const fn map_register() -> Register {
            r7
        }
    }

    pub struct ApiGetterDescriptor;

    impl ApiGetterDescriptor {
        pub const fn holder_register() -> Register {
            r2
        }
        pub const fn callback_register() -> Register {
            r5
        }
    }

    pub struct GrowArrayElementsDescriptor;

    impl GrowArrayElementsDescriptor {
        pub const fn object_register() -> Register {
            r2
        }
        pub const fn key_register() -> Register {
            r5
        }
    }

    pub struct BaselineLeaveFrameDescriptor;

    impl BaselineLeaveFrameDescriptor {
        pub const fn params_size_register() -> Register {
            // TODO(v8:11421): Implement on this platform.
            r5
        }
        pub const fn weight_register() -> Register {
            // TODO(v8:11421): Implement on this platform.
            r6
        }
    }

    pub struct TypeConversionDescriptor;

    impl TypeConversionDescriptor {
        pub const fn argument_register() -> Register {
            r2
        }
    }

    pub struct TypeofDescriptor;

    impl TypeofDescriptor {
        pub const fn registers() -> RegisterArray<1> {
            RegisterArray::new([r2])
        }
    }

    pub struct MaglevOptimizeCodeOrTailCallOptimizedCodeSlotDescriptor;

    impl MaglevOptimizeCodeOrTailCallOptimizedCodeSlotDescriptor {
        pub const fn flags_register() -> Register {
            r4
        }
        pub const fn feedback_vector_register() -> Register {
            r7
        }
        pub const fn temporary_register() -> Register {
            r6
        }
    }

    pub struct CallTrampolineDescriptor;

    impl CallTrampolineDescriptor {
        pub const fn registers() -> RegisterArray<2> {
            RegisterArray::new([r3, r2])
        }
    }

    pub struct CopyDataPropertiesWithExcludedPropertiesDescriptor;

    impl CopyDataPropertiesWithExcludedPropertiesDescriptor {
        pub const fn registers() -> RegisterArray<2> {
            RegisterArray::new([r3, r2])
        }
    }

    pub struct CopyDataPropertiesWithExcludedPropertiesOnStackDescriptor;

    impl CopyDataPropertiesWithExcludedPropertiesOnStackDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            RegisterArray::new([r3, r2, r4])
        }
    }

    pub struct CallVarargsDescriptor;

    impl CallVarargsDescriptor {
        pub const fn registers() -> RegisterArray<4> {
            RegisterArray::new([r3, r2, r6, r4])
        }
    }

    pub struct CallForwardVarargsDescriptor;

    impl CallForwardVarargsDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            RegisterArray::new([r3, r2, r4])
        }
    }

    pub struct CallFunctionTemplateDescriptor;

    impl CallFunctionTemplateDescriptor {
        pub const fn registers() -> RegisterArray<2> {
            RegisterArray::new([r3, r4])
        }
    }

    pub struct CallFunctionTemplateGenericDescriptor;

    impl CallFunctionTemplateGenericDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            RegisterArray::new([r3, r4, r5])
        }
    }

    pub struct CallWithSpreadDescriptor;

    impl CallWithSpreadDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            RegisterArray::new([r3, r2, r4])
        }
    }

    pub struct CallWithArrayLikeDescriptor;

    impl CallWithArrayLikeDescriptor {
        pub const fn registers() -> RegisterArray<2> {
            RegisterArray::new([r3, r4])
        }
    }

    pub struct ConstructVarargsDescriptor;

    impl ConstructVarargsDescriptor {
        pub const fn registers() -> RegisterArray<5> {
            RegisterArray::new([r3, r5, r2, r6, r4])
        }
    }

    pub struct ConstructForwardVarargsDescriptor;

    impl ConstructForwardVarargsDescriptor {
        pub const fn registers() -> RegisterArray<4> {
            RegisterArray::new([r3, r5, r2, r4])
        }
    }

    pub struct ConstructWithSpreadDescriptor;

    impl ConstructWithSpreadDescriptor {
        pub const fn registers() -> RegisterArray<4> {
            RegisterArray::new([r3, r5, r2, r4])
        }
    }

    pub struct ConstructWithArrayLikeDescriptor;

    impl ConstructWithArrayLikeDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            RegisterArray::new([r3, r5, r4])
        }
    }

    pub struct ConstructStubDescriptor;

    impl ConstructStubDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            RegisterArray::new([r3, r5, r2])
        }
    }

    pub struct AbortDescriptor;

    impl AbortDescriptor {
        pub const fn registers() -> RegisterArray<1> {
            RegisterArray::new([r3])
        }
    }

    pub struct CompareDescriptor;

    impl CompareDescriptor {
        pub const fn registers() -> RegisterArray<2> {
            RegisterArray::new([r3, r2])
        }
    }

    pub struct Compare_BaselineDescriptor;

    impl Compare_BaselineDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            RegisterArray::new([r3, r2, r4])
        }
    }

    pub struct BinaryOpDescriptor;

    impl BinaryOpDescriptor {
        pub const fn registers() -> RegisterArray<2> {
            RegisterArray::new([r3, r2])
        }
    }

    pub struct BinaryOp_BaselineDescriptor;

    impl BinaryOp_BaselineDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            RegisterArray::new([r3, r2, r4])
        }
    }

    pub struct BinarySmiOp_BaselineDescriptor;

    impl BinarySmiOp_BaselineDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            RegisterArray::new([r2, r3, r4])
        }
    }

    pub struct CallApiCallbackOptimizedDescriptor;

    impl CallApiCallbackOptimizedDescriptor {
        pub const fn api_function_address_register() -> Register {
            r3
        }
        pub const fn actual_arguments_count_register() -> Register {
            r4
        }
        pub const fn function_template_info_register() -> Register {
            r5
        }
    }

    pub struct CallApiCallbackGenericDescriptor;

    impl CallApiCallbackGenericDescriptor {
        pub const fn actual_arguments_count_register() -> Register {
            r4
        }
        pub const fn topmost_script_having_context_register() -> Register {
            r3
        }
        pub const fn function_template_info_register() -> Register {
            r5
        }
    }

    pub struct InterpreterDispatchDescriptor;

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

    pub struct InterpreterPushArgsThenCallDescriptor;

    impl InterpreterPushArgsThenCallDescriptor {
        pub const fn registers() -> RegisterArray<3> {
            RegisterArray::new([
                r2, // argument count
                r4, // address of first argument
                r3, // the target callable to be call
            ])
        }
    }

    pub struct InterpreterPushArgsThenConstructDescriptor;

    impl InterpreterPushArgsThenConstructDescriptor {
        pub const fn registers() -> RegisterArray<5> {
            RegisterArray::new([
                r2, // argument count
                r6, // address of the first argument
                r3, // constructor to call
                r5, // new target
                r4, // allocation site feedback if available, undefined otherwise
            ])
        }
    }

    pub struct ConstructForwardAllArgsDescriptor;

    impl ConstructForwardAllArgsDescriptor {
        pub const fn registers() -> RegisterArray<2> {
            RegisterArray::new([
                r3, // constructor to call
                r5, // new target
            ])
        }
    }

    pub struct ResumeGeneratorDescriptor;

    impl ResumeGeneratorDescriptor {
        pub const fn registers() -> RegisterArray<2> {
            RegisterArray::new([
                r2, // the value to pass to the generator
                r3, // the JSGeneratorObject to resume
            ])
        }
    }

    pub struct RunMicrotasksEntryDescriptor;

    impl RunMicrotasksEntryDescriptor {
        pub const fn registers() -> RegisterArray<2> {
            RegisterArray::new([r2, r3])
        }
    }

    pub struct WasmJSToWasmWrapperDescriptor;

    impl WasmJSToWasmWrapperDescriptor {
        pub const fn registers() -> RegisterArray<1> {
            RegisterArray::new([r8])
        }
    }
}
