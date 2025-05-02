// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![cfg(target_arch = "aarch64")]

mod interface_descriptors {
    use std::marker::PhantomData;

    // Placeholder types - replace with actual V8 types
    type Register = u32;
    type DoubleRegister = u32;
    type RegList = u64;

    const NO_REGISTER: Register = 0;
    const NO_DOUBLE_REGISTER: DoubleRegister = 0;

    const X0: Register = 0;
    const X1: Register = 1;
    const X2: Register = 2;
    const X3: Register = 3;
    const X4: Register = 4;
    const X5: Register = 5;
    const X6: Register = 6;
    const X7: Register = 7;
    const X8: Register = 8;
    const X9: Register = 9;

    const D0: DoubleRegister = 0;
    const D1: DoubleRegister = 1;
    const D2: DoubleRegister = 2;
    const D3: DoubleRegister = 3;
    const D4: DoubleRegister = 4;
    const D5: DoubleRegister = 5;
    const D6: DoubleRegister = 6;

    const K_RETURN_REGISTER0: Register = 10;
    const K_RETURN_REGISTER1: Register = 11;
    const K_RETURN_REGISTER2: Register = 12;

    const K_FP_RETURN_REGISTER0: DoubleRegister = 7;
    const NO_DREG: DoubleRegister = NO_DOUBLE_REGISTER;

    const K_INTERPRETER_ACCUMULATOR_REGISTER: Register = 13;
    const K_INTERPRETER_BYTECODE_OFFSET_REGISTER: Register = 14;
    const K_INTERPRETER_BYTECODE_ARRAY_REGISTER: Register = 15;
    const K_INTERPRETER_DISPATCH_TABLE_REGISTER: Register = 16;

    const K_CONTEXT_REGISTER: Register = 17;

    const K_MAX_BUILTIN_REGISTER_PARAMS: usize = 5;

    // --- Helper structs/functions ---

    #[derive(Debug, Clone, Copy)]
    struct RegisterArray<T, const N: usize> {
        registers: [T; N],
    }

    impl<T: Copy, const N: usize> RegisterArray<T, N> {
        const fn new(registers: [T; N]) -> Self {
            RegisterArray { registers }
        }
        const fn size(&self) -> usize {
            N
        }
    }

    macro_rules! register_array {
        ($($reg:expr),*) => {
            {
                #[allow(unused_mut)]
                let mut array = [0; 0];
                #[allow(unused_mut)]
                let mut vec = Vec::new();
                $(vec.push($reg);)*
                array = vec.try_into().unwrap();
                RegisterArray::new(array)
            }
        };
    }

    // --- CallInterfaceDescriptor ---

    pub struct CallInterfaceDescriptor {}

    impl CallInterfaceDescriptor {
        pub const fn default_register_array() -> RegisterArray<Register, K_MAX_BUILTIN_REGISTER_PARAMS> {
            RegisterArray::new([X0, X1, X2, X3, X4])
        }

        pub const fn default_double_register_array() -> RegisterArray<DoubleRegister, 7> {
            RegisterArray::new([D0, D1, D2, D3, D4, D5, D6])
        }

        pub const fn default_return_register_array() -> RegisterArray<Register, 3> {
            RegisterArray::new([K_RETURN_REGISTER0, K_RETURN_REGISTER1, K_RETURN_REGISTER2])
        }

        pub const fn default_return_double_register_array() -> RegisterArray<DoubleRegister, 3> {
            RegisterArray::new([K_FP_RETURN_REGISTER0, NO_DREG, NO_DREG])
        }
    }

    // --- StaticCallInterfaceDescriptor ---

    pub struct StaticCallInterfaceDescriptor<T> {
        _phantom: PhantomData<T>,
    }

    impl<T> StaticCallInterfaceDescriptor<T> {
        #[cfg(debug_assertions)]
        pub fn verify_argument_register_count(_data: &mut CallInterfaceDescriptorData, argc: i32) {
            todo!() // RegList not yet defined
            /*
            let allocatable_regs = data.allocatable_registers();
            if argc >= 1 { assert!(allocatable_regs.has(x0)); }
            if argc >= 2 { assert!(allocatable_regs.has(x1)); }
            if argc >= 3 { assert!(allocatable_regs.has(x2)); }
            if argc >= 4 { assert!(allocatable_regs.has(x3)); }
            if argc >= 5 { assert!(allocatable_regs.has(x4)); }
            if argc >= 6 { assert!(allocatable_regs.has(x5)); }
            if argc >= 7 { assert!(allocatable_regs.has(x6)); }
            if argc >= 8 { assert!(allocatable_regs.has(x7)); }
            */
        }
    }

    // --- WriteBarrierDescriptor ---

    pub struct WriteBarrierDescriptor {}

    impl WriteBarrierDescriptor {
        pub const fn registers() -> RegisterArray<Register, 8> {
            RegisterArray::new([X1, X5, X4, X2, X0, X3, K_CONTEXT_REGISTER, X7])
        }
    }

    // --- LoadDescriptor ---

    pub struct LoadDescriptor {}

    impl LoadDescriptor {
        pub const fn receiver_register() -> Register {
            X1
        }
        pub const fn name_register() -> Register {
            X2
        }
        pub const fn slot_register() -> Register {
            X0
        }
    }

    // --- LoadWithVectorDescriptor ---

    pub struct LoadWithVectorDescriptor {}

    impl LoadWithVectorDescriptor {
        pub const fn vector_register() -> Register {
            X3
        }
    }

    // --- KeyedLoadBaselineDescriptor ---

    pub struct KeyedLoadBaselineDescriptor {}

    impl KeyedLoadBaselineDescriptor {
        pub const fn receiver_register() -> Register {
            X1
        }
        pub const fn name_register() -> Register {
            K_INTERPRETER_ACCUMULATOR_REGISTER
        }
        pub const fn slot_register() -> Register {
            X2
        }
    }

    // --- KeyedLoadWithVectorDescriptor ---

    pub struct KeyedLoadWithVectorDescriptor {}

    impl KeyedLoadWithVectorDescriptor {
        pub const fn vector_register() -> Register {
            X3
        }
    }

    // --- EnumeratedKeyedLoadBaselineDescriptor ---

    pub struct EnumeratedKeyedLoadBaselineDescriptor {}

    impl EnumeratedKeyedLoadBaselineDescriptor {
        pub const fn enum_index_register() -> Register {
            X4
        }

        pub const fn cache_type_register() -> Register {
            X5
        }

        pub const fn slot_register() -> Register {
            X2
        }
    }

    // --- KeyedHasICBaselineDescriptor ---

    pub struct KeyedHasICBaselineDescriptor {}

    impl KeyedHasICBaselineDescriptor {
        pub const fn receiver_register() -> Register {
            K_INTERPRETER_ACCUMULATOR_REGISTER
        }
        pub const fn name_register() -> Register {
            X1
        }
        pub const fn slot_register() -> Register {
            X2
        }
    }

    // --- KeyedHasICWithVectorDescriptor ---

    pub struct KeyedHasICWithVectorDescriptor {}

    impl KeyedHasICWithVectorDescriptor {
        pub const fn vector_register() -> Register {
            X3
        }
    }

    // --- LoadWithReceiverAndVectorDescriptor ---

    pub struct LoadWithReceiverAndVectorDescriptor {}

    impl LoadWithReceiverAndVectorDescriptor {
        pub const fn lookup_start_object_register() -> Register {
            X4
        }
    }

    // --- StoreDescriptor ---

    pub struct StoreDescriptor {}

    impl StoreDescriptor {
        pub const fn receiver_register() -> Register {
            X1
        }
        pub const fn name_register() -> Register {
            X2
        }
        pub const fn value_register() -> Register {
            X0
        }
        pub const fn slot_register() -> Register {
            X4
        }
    }

    // --- StoreWithVectorDescriptor ---

    pub struct StoreWithVectorDescriptor {}

    impl StoreWithVectorDescriptor {
        pub const fn vector_register() -> Register {
            X3
        }
    }

    // --- DefineKeyedOwnDescriptor ---

    pub struct DefineKeyedOwnDescriptor {}

    impl DefineKeyedOwnDescriptor {
        pub const fn flags_register() -> Register {
            X5
        }
    }

    // --- StoreTransitionDescriptor ---

    pub struct StoreTransitionDescriptor {}

    impl StoreTransitionDescriptor {
        pub const fn map_register() -> Register {
            X5
        }
    }

    // --- ApiGetterDescriptor ---

    pub struct ApiGetterDescriptor {}

    impl ApiGetterDescriptor {
        pub const fn holder_register() -> Register {
            X0
        }
        pub const fn callback_register() -> Register {
            X3
        }
    }

    // --- GrowArrayElementsDescriptor ---

    pub struct GrowArrayElementsDescriptor {}

    impl GrowArrayElementsDescriptor {
        pub const fn object_register() -> Register {
            X0
        }
        pub const fn key_register() -> Register {
            X3
        }
    }

    // --- BaselineLeaveFrameDescriptor ---

    pub struct BaselineLeaveFrameDescriptor {}

    impl BaselineLeaveFrameDescriptor {
        pub const fn params_size_register() -> Register {
            X3
        }
        pub const fn weight_register() -> Register {
            X4
        }
    }

    // --- TypeConversionDescriptor ---

    pub struct TypeConversionDescriptor {}

    impl TypeConversionDescriptor {
        pub const fn argument_register() -> Register {
            X0
        }
    }

    // --- MaglevOptimizeCodeOrTailCallOptimizedCodeSlotDescriptor ---

    pub struct MaglevOptimizeCodeOrTailCallOptimizedCodeSlotDescriptor {}

    impl MaglevOptimizeCodeOrTailCallOptimizedCodeSlotDescriptor {
        pub const fn flags_register() -> Register {
            X8
        }
        pub const fn feedback_vector_register() -> Register {
            X9
        }
        pub const fn temporary_register() -> Register {
            X5
        }
    }

    // --- TypeofDescriptor ---

    pub struct TypeofDescriptor {}

    impl TypeofDescriptor {
        pub const fn registers() -> RegisterArray<Register, 1> {
            RegisterArray::new([X0])
        }
    }

    // --- CallTrampolineDescriptor ---

    pub struct CallTrampolineDescriptor {}

    impl CallTrampolineDescriptor {
        pub const fn registers() -> RegisterArray<Register, 2> {
            RegisterArray::new([X1, X0])
        }
    }

    // --- CopyDataPropertiesWithExcludedPropertiesDescriptor ---

    pub struct CopyDataPropertiesWithExcludedPropertiesDescriptor {}

    impl CopyDataPropertiesWithExcludedPropertiesDescriptor {
        pub const fn registers() -> RegisterArray<Register, 2> {
            RegisterArray::new([X1, X0])
        }
    }

    // --- CopyDataPropertiesWithExcludedPropertiesOnStackDescriptor ---

    pub struct CopyDataPropertiesWithExcludedPropertiesOnStackDescriptor {}

    impl CopyDataPropertiesWithExcludedPropertiesOnStackDescriptor {
        pub const fn registers() -> RegisterArray<Register, 3> {
            RegisterArray::new([X1, X0, X2])
        }
    }

    // --- CallVarargsDescriptor ---

    pub struct CallVarargsDescriptor {}

    impl CallVarargsDescriptor {
        pub const fn registers() -> RegisterArray<Register, 4> {
            RegisterArray::new([X1, X0, X4, X2])
        }
    }

    // --- CallForwardVarargsDescriptor ---

    pub struct CallForwardVarargsDescriptor {}

    impl CallForwardVarargsDescriptor {
        pub const fn registers() -> RegisterArray<Register, 3> {
            RegisterArray::new([X1, X0, X2])
        }
    }

    // --- CallFunctionTemplateDescriptor ---

    pub struct CallFunctionTemplateDescriptor {}

    impl CallFunctionTemplateDescriptor {
        pub const fn registers() -> RegisterArray<Register, 2> {
            RegisterArray::new([X1, X2])
        }
    }

    // --- CallFunctionTemplateGenericDescriptor ---

    pub struct CallFunctionTemplateGenericDescriptor {}

    impl CallFunctionTemplateGenericDescriptor {
        pub const fn registers() -> RegisterArray<Register, 3> {
            RegisterArray::new([X1, X2, X3])
        }
    }

    // --- CallWithSpreadDescriptor ---

    pub struct CallWithSpreadDescriptor {}

    impl CallWithSpreadDescriptor {
        pub const fn registers() -> RegisterArray<Register, 3> {
            RegisterArray::new([X1, X0, X2])
        }
    }

    // --- CallWithArrayLikeDescriptor ---

    pub struct CallWithArrayLikeDescriptor {}

    impl CallWithArrayLikeDescriptor {
        pub const fn registers() -> RegisterArray<Register, 2> {
            RegisterArray::new([X1, X2])
        }
    }

    // --- ConstructVarargsDescriptor ---

    pub struct ConstructVarargsDescriptor {}

    impl ConstructVarargsDescriptor {
        pub const fn registers() -> RegisterArray<Register, 5> {
            RegisterArray::new([X1, X3, X0, X4, X2])
        }
    }

    // --- ConstructForwardVarargsDescriptor ---

    pub struct ConstructForwardVarargsDescriptor {}

    impl ConstructForwardVarargsDescriptor {
        pub const fn registers() -> RegisterArray<Register, 4> {
            RegisterArray::new([X1, X3, X0, X2])
        }
    }

    // --- ConstructWithSpreadDescriptor ---

    pub struct ConstructWithSpreadDescriptor {}

    impl ConstructWithSpreadDescriptor {
        pub const fn registers() -> RegisterArray<Register, 4> {
            RegisterArray::new([X1, X3, X0, X2])
        }
    }

    // --- ConstructWithArrayLikeDescriptor ---

    pub struct ConstructWithArrayLikeDescriptor {}

    impl ConstructWithArrayLikeDescriptor {
        pub const fn registers() -> RegisterArray<Register, 3> {
            RegisterArray::new([X1, X3, X2])
        }
    }

    // --- ConstructStubDescriptor ---

    pub struct ConstructStubDescriptor {}

    impl ConstructStubDescriptor {
        pub const fn registers() -> RegisterArray<Register, 3> {
            RegisterArray::new([X1, X3, X0])
        }
    }

    // --- AbortDescriptor ---

    pub struct AbortDescriptor {}

    impl AbortDescriptor {
        pub const fn registers() -> RegisterArray<Register, 1> {
            RegisterArray::new([X1])
        }
    }

    // --- CompareDescriptor ---

    pub struct CompareDescriptor {}

    impl CompareDescriptor {
        pub const fn registers() -> RegisterArray<Register, 2> {
            RegisterArray::new([X1, X0])
        }
    }

    // --- Compare_BaselineDescriptor ---

    pub struct Compare_BaselineDescriptor {}

    impl Compare_BaselineDescriptor {
        pub const fn registers() -> RegisterArray<Register, 3> {
            RegisterArray::new([X1, X0, X2])
        }
    }

    // --- BinaryOpDescriptor ---

    pub struct BinaryOpDescriptor {}

    impl BinaryOpDescriptor {
        pub const fn registers() -> RegisterArray<Register, 2> {
            RegisterArray::new([X1, X0])
        }
    }

    // --- BinaryOp_BaselineDescriptor ---

    pub struct BinaryOp_BaselineDescriptor {}

    impl BinaryOp_BaselineDescriptor {
        pub const fn registers() -> RegisterArray<Register, 3> {
            RegisterArray::new([X1, X0, X2])
        }
    }

    // --- BinarySmiOp_BaselineDescriptor ---

    pub struct BinarySmiOp_BaselineDescriptor {}

    impl BinarySmiOp_BaselineDescriptor {
        pub const fn registers() -> RegisterArray<Register, 3> {
            RegisterArray::new([X0, X1, X2])
        }
    }

    // --- CallApiCallbackOptimizedDescriptor ---

    pub struct CallApiCallbackOptimizedDescriptor {}

    impl CallApiCallbackOptimizedDescriptor {
        pub const fn api_function_address_register() -> Register {
            X1
        }
        pub const fn actual_arguments_count_register() -> Register {
            X2
        }
        pub const fn function_template_info_register() -> Register {
            X3
        }
    }

    // --- CallApiCallbackGenericDescriptor ---

    pub struct CallApiCallbackGenericDescriptor {}

    impl CallApiCallbackGenericDescriptor {
        pub const fn actual_arguments_count_register() -> Register {
            X2
        }
        pub const fn topmost_script_having_context_register() -> Register {
            X1
        }
        pub const fn function_template_info_register() -> Register {
            X3
        }
    }

    // --- InterpreterDispatchDescriptor ---

    pub struct InterpreterDispatchDescriptor {}

    impl InterpreterDispatchDescriptor {
        pub const fn registers() -> RegisterArray<Register, 4> {
            RegisterArray::new([
                K_INTERPRETER_ACCUMULATOR_REGISTER,
                K_INTERPRETER_BYTECODE_OFFSET_REGISTER,
                K_INTERPRETER_BYTECODE_ARRAY_REGISTER,
                K_INTERPRETER_DISPATCH_TABLE_REGISTER,
            ])
        }
    }

    // --- InterpreterPushArgsThenCallDescriptor ---

    pub struct InterpreterPushArgsThenCallDescriptor {}

    impl InterpreterPushArgsThenCallDescriptor {
        pub const fn registers() -> RegisterArray<Register, 3> {
            RegisterArray::new([
                X0, // argument count
                X2, // address of first argument
                X1, // the target callable to be call
            ])
        }
    }

    // --- InterpreterPushArgsThenConstructDescriptor ---

    pub struct InterpreterPushArgsThenConstructDescriptor {}

    impl InterpreterPushArgsThenConstructDescriptor {
        pub const fn registers() -> RegisterArray<Register, 5> {
            RegisterArray::new([
                X0, // argument count
                X4, // address of the first argument
                X1, // constructor to call
                X3, // new target
                X2, // allocation site feedback if available, undefined otherwise
            ])
        }
    }

    // --- ConstructForwardAllArgsDescriptor ---

    pub struct ConstructForwardAllArgsDescriptor {}

    impl ConstructForwardAllArgsDescriptor {
        pub const fn registers() -> RegisterArray<Register, 2> {
            RegisterArray::new([
                X1, // constructor to call
                X3, // new target
            ])
        }
    }

    // --- ResumeGeneratorDescriptor ---

    pub struct ResumeGeneratorDescriptor {}

    impl ResumeGeneratorDescriptor {
        pub const fn registers() -> RegisterArray<Register, 2> {
            RegisterArray::new([
                X0, // the value to pass to the generator
                X1, // the JSGeneratorObject to resume
            ])
        }
    }

    // --- RunMicrotasksEntryDescriptor ---

    pub struct RunMicrotasksEntryDescriptor {}

    impl RunMicrotasksEntryDescriptor {
        pub const fn registers() -> RegisterArray<Register, 2> {
            RegisterArray::new([X0, X1])
        }
    }

    // --- WasmJSToWasmWrapperDescriptor ---
    pub struct WasmJSToWasmWrapperDescriptor {}

    impl WasmJSToWasmWrapperDescriptor {
        pub const fn registers() -> RegisterArray<Register, 1> {
            RegisterArray::new([X8])
        }
    }

    // Placeholder for CallInterfaceDescriptorData (if needed)
    pub struct CallInterfaceDescriptorData {}
}