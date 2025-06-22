// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod interface_descriptors {
    use std::{
        mem,
        ops::{BitOr, BitOrAssign},
        ptr,
    };

    //use crate::base::logging; // Assuming a Rust equivalent exists
    //use crate::codegen::machine_type::MachineType; // Assuming a Rust equivalent exists
    //use crate::codegen::register::Register; // Assuming a Rust equivalent exists
    //use crate::codegen::tnode::TNode; // Assuming a Rust equivalent exists
    //use crate::common::globals::*; // Assuming a Rust equivalent exists
    //use crate::execution::isolate::Isolate; // Assuming a Rust equivalent exists

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum StackArgumentOrder {
        KDefault, // Arguments in the stack are pushed in the default/stub order (the
                  // first argument is pushed first).
        KJS,      // Arguments in the stack are pushed in the same order as the one used
                  // by JS-to-JS function calls. This should be used if calling a
                  // JSFunction or if the builtin is expected to be called directly from a
                  // JSFunction. This order is reversed compared to kDefault.
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct Register(usize); // Placeholder

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DoubleRegister(usize); // Placeholder

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct MachineType(usize); // Placeholder

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct CodeEntrypointTag(usize); // Placeholder

    pub const KDefaultCodeEntrypointTag: CodeEntrypointTag = CodeEntrypointTag(0);
    pub const KInvalidEntrypointTag: CodeEntrypointTag = CodeEntrypointTag(1);
    pub const KJSEntrypointTag: CodeEntrypointTag = CodeEntrypointTag(2);
    pub const KRegExpEntrypointTag: CodeEntrypointTag = CodeEntrypointTag(3);
    pub const KStoreTransitionICHandlerEntrypointTag: CodeEntrypointTag = CodeEntrypointTag(4);
    pub const KStoreWithVectorICHandlerEntrypointTag: CodeEntrypointTag = CodeEntrypointTag(5);
    pub const KLoadWithVectorICHandlerEntrypointTag: CodeEntrypointTag = CodeEntrypointTag(6);
    pub const kWasmEntrypointTag: CodeEntrypointTag = CodeEntrypointTag(7);

    // Placeholder types
    pub type RegList = u64; // Placeholder
    pub const no_reg: Register = Register(0);
    pub const kContextRegister: Register = Register(1);

    // Helper function
    pub const fn arraysize<T>(_: &[T]) -> usize {
        mem::size_of::<[T]>() / mem::size_of::<T>()
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Flags(u32);

    impl Flags {
        pub const NO_FLAGS: Flags = Flags(0);
        pub const NO_CONTEXT: Flags = Flags(1 << 0);
        // This indicates that the code uses a special frame that does not scan the
        // stack arguments, e.g. EntryFrame. And this allows the code to use
        // untagged stack arguments.
        pub const NO_STACK_SCAN: Flags = Flags(1 << 1);
        // In addition to the specified parameters, additional arguments can be
        // passed on the stack.
        // This does not indicate if arguments adaption is used or not.
        pub const ALLOW_VAR_ARGS: Flags = Flags(1 << 2);
        // Callee save allocatable_registers.
        pub const CALLEE_SAVE_REGISTERS: Flags = Flags(1 << 3);

        pub fn contains(&self, other: Flags) -> bool {
            (self.0 & other.0) == other.0
        }
    }

    impl BitOr for Flags {
        type Output = Self;

        fn bitor(self, other: Self) -> Self {
            Flags(self.0 | other.0)
        }
    }

    impl BitOrAssign for Flags {
        fn bitor_assign(&mut self, other: Self) {
            self.0 |= other.0;
        }
    }

    #[derive(Debug)]
    pub struct CallInterfaceDescriptorData {
        flags_: Flags,
        tag_: CodeEntrypointTag,
        return_count_: i32,
        param_count_: i32,
        register_param_count_: i32,
        stack_order_: StackArgumentOrder,
        allocatable_registers_: RegList,
        register_params_: *const Register,
        double_register_params_: *const DoubleRegister,
        register_returns_: *const Register,
        double_register_returns_: *const DoubleRegister,
        machine_types_: *mut MachineType,
    }

    impl Default for CallInterfaceDescriptorData {
        fn default() -> Self {
            CallInterfaceDescriptorData {
                flags_: Flags::NO_FLAGS,
                tag_: KDefaultCodeEntrypointTag,
                return_count_: Self::K_UNINITIALIZED_COUNT,
                param_count_: Self::K_UNINITIALIZED_COUNT,
                register_param_count_: Self::K_UNINITIALIZED_COUNT,
                stack_order_: StackArgumentOrder::KDefault,
                allocatable_registers_: 0,
                register_params_: ptr::null(),
                double_register_params_: ptr::null(),
                register_returns_: ptr::null(),
                double_register_returns_: ptr::null(),
                machine_types_: ptr::null_mut(),
            }
        }
    }

    impl CallInterfaceDescriptorData {
        pub const K_UNINITIALIZED_COUNT: i32 = -1;

        pub fn initialize_registers(
            &mut self,
            flags: Flags,
            tag: CodeEntrypointTag,
            return_count: i32,
            parameter_count: i32,
            stack_order: StackArgumentOrder,
            register_parameter_count: i32,
            registers: *const Register,
            double_registers: *const DoubleRegister,
            return_registers: *const Register,
            return_double_registers: *const DoubleRegister,
        ) {
            self.flags_ = flags;
            self.tag_ = tag;
            self.return_count_ = return_count;
            self.param_count_ = parameter_count;
            self.stack_order_ = stack_order;
            self.register_param_count_ = register_parameter_count;
            self.register_params_ = registers;
            self.double_register_params_ = double_registers;
            self.register_returns_ = return_registers;
            self.double_register_returns_ = return_double_registers;
        }

        pub fn initialize_types(&mut self, machine_types: *const MachineType, machine_types_length: usize) {
            if machine_types.is_null() {
                let total_count = self.return_count_ + self.param_count_;
                // Allocate memory for machine_types_ (remember to free it later).
                let layout = std::alloc::Layout::array::<MachineType>(total_count as usize)
                    .expect("Failed to create layout");
                let ptr = unsafe { std::alloc::alloc(layout) } as *mut MachineType;
                if ptr.is_null() {
                    panic!("Memory allocation failed");
                }
                self.machine_types_ = ptr;

                for i in 0..total_count {
                    unsafe {
                        *self.machine_types_.add(i as usize) = MachineType(0); // MachineType::AnyTagged() equivalent
                    }
                }
            } else {
                let total_count = self.return_count_ + self.param_count_;

                let layout = std::alloc::Layout::array::<MachineType>(total_count as usize)
                    .expect("Failed to create layout");
                let ptr = unsafe { std::alloc::alloc(layout) } as *mut MachineType;
                if ptr.is_null() {
                    panic!("Memory allocation failed");
                }
                self.machine_types_ = ptr;

                for i in 0..machine_types_length {
                    unsafe {
                        *self.machine_types_.add(i as usize) = *machine_types.add(i as usize);
                    }
                }

                for i in machine_types_length..total_count as usize {
                    unsafe {
                        *self.machine_types_.add(i as usize) = MachineType(0); // MachineType::AnyTagged() equivalent
                    }
                }
            }
        }

        pub fn reset(&mut self) {
            self.return_count_ = Self::K_UNINITIALIZED_COUNT;
            self.param_count_ = Self::K_UNINITIALIZED_COUNT;
            self.register_param_count_ = Self::K_UNINITIALIZED_COUNT;
            self.flags_ = Flags::NO_FLAGS;
            self.tag_ = KDefaultCodeEntrypointTag;
            self.stack_order_ = StackArgumentOrder::KDefault;
            self.allocatable_registers_ = 0;

            self.register_params_ = ptr::null();
            self.double_register_params_ = ptr::null();
            self.register_returns_ = ptr::null();
            self.double_register_returns_ = ptr::null();
            
            if !self.machine_types_.is_null() {
                let total_count = self.return_count_ + self.param_count_;
                let layout = std::alloc::Layout::array::<MachineType>(total_count as usize)
                    .expect("Failed to create layout");
                unsafe {
                    std::alloc::dealloc(self.machine_types_ as *mut u8, layout);
                }
                self.machine_types_ = ptr::null_mut();
            }
        }

        pub fn is_initialized(&self) -> bool {
            self.is_initialized_registers() && self.is_initialized_types()
        }

        pub fn flags(&self) -> Flags {
            self.flags_
        }
        pub fn tag(&self) -> CodeEntrypointTag {
            self.tag_
        }
        pub fn return_count(&self) -> i32 {
            self.return_count_
        }
        pub fn param_count(&self) -> i32 {
            self.param_count_
        }
        pub fn register_param_count(&self) -> i32 {
            self.register_param_count_
        }
        pub fn register_param(&self, index: i32) -> Register {
            assert!(index >= 0 && index < self.register_param_count_);
            unsafe { *self.register_params_.add(index as usize) }
        }
        pub fn double_register_param(&self, index: i32) -> DoubleRegister {
            assert!(index >= 0 && index < self.register_param_count_);
            unsafe { *self.double_register_params_.add(index as usize) }
        }
        pub fn register_return(&self, index: i32) -> Register {
            assert!(index >= 0 && index < self.return_count_);
            unsafe { *self.register_returns_.add(index as usize) }
        }
        pub fn double_register_return(&self, index: i32) -> DoubleRegister {
            assert!(index >= 0 && index < self.return_count_);
            unsafe { *self.double_register_returns_.add(index as usize) }
        }
        pub fn return_type(&self, index: i32) -> MachineType {
            assert!(index >= 0 && index < self.return_count_);
            unsafe { *self.machine_types_.add(index as usize) }
        }
        pub fn param_type(&self, index: i32) -> MachineType {
            assert!(index >= 0 && index < self.param_count_);
            unsafe { *self.machine_types_.add((self.return_count_ + index) as usize) }
        }
        pub fn stack_order(&self) -> StackArgumentOrder {
            self.stack_order_
        }

        pub fn restrict_allocatable_registers(&mut self, registers: *const Register, num: usize) {
            assert!(self.allocatable_registers_ == 0);
            for i in 0..num {
                unsafe {
                    self.allocatable_registers_ |= 1 << (*registers.add(i)).0; // Assuming register value can be used as bit index
                }
            }
            assert!(self.allocatable_registers_ != 0);
        }

        pub fn allocatable_registers(&self) -> RegList {
            self.allocatable_registers_
        }

        fn is_initialized_registers(&self) -> bool {
            let initialized = self.return_count_ != Self::K_UNINITIALIZED_COUNT
                && self.param_count_ != Self::K_UNINITIALIZED_COUNT
                && (self.register_param_count_ == 0 || !self.register_params_.is_null());
            // Register initialization happens before type initialization.
            initialized
        }

        fn is_initialized_types(&self) -> bool {
            let initialized = !self.machine_types_.is_null();
            // Register initialization happens before type initialization.
            initialized
        }
    }

    #[derive(Debug)]
    pub struct CallDescriptors {
        // Static class, no fields needed
    }

    impl CallDescriptors {
        // CallInterfaceDescriptorData array
        static mut CALL_DESCRIPTOR_DATA: [CallInterfaceDescriptorData; Self::NUMBER_OF_DESCRIPTORS] =
            [CallInterfaceDescriptorData::default(); Self::NUMBER_OF_DESCRIPTORS];

        pub const NUMBER_OF_DESCRIPTORS: usize = 190;

        pub fn call_descriptor_data(key: CallDescriptorsKey) -> *mut CallInterfaceDescriptorData {
            unsafe { &mut Self::CALL_DESCRIPTOR_DATA[key as usize] }
        }

        pub fn get_key(data: *const CallInterfaceDescriptorData) -> CallDescriptorsKey {
            let base_ptr = unsafe { Self::CALL_DESCRIPTOR_DATA.as_ptr() as *const CallInterfaceDescriptorData };
            let index = (data as usize - base_ptr as usize) / mem::size_of::<CallInterfaceDescriptorData>();
            assert!(index >= 0 && index < Self::NUMBER_OF_DESCRIPTORS);
            unsafe { mem::transmute(index) }
        }

        pub fn initialize_once_per_process() {
            // Initialization logic here if needed
            // Example:
            // unsafe { Self::CALL_DESCRIPTOR_DATA[CallDescriptorsKey::Abort as usize].initialize(...) }
        }

        pub fn tear_down() {
            // Teardown logic here, e.g., deallocating memory
            unsafe {
                for data in Self::CALL_DESCRIPTOR_DATA.iter_mut() {
                    data.reset();
                }
            }
        }
    }

    #[repr(usize)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum CallDescriptorsKey {
        Abort,
        Allocate,
        CallApiCallbackGeneric,
        CallApiCallbackOptimized,
        ApiGetter,
        ArrayConstructor,
        ArrayNArgumentsConstructor,
        ArrayNoArgumentConstructor,
        ArraySingleArgumentConstructor,
        AsyncFunctionStackParameter,
        BaselineLeaveFrame,
        BaselineOutOfLinePrologue,
        BigIntToI32Pair,
        BigIntToI64,
        BinaryOp,
        BinaryOp_Baseline,
        BinaryOp_WithFeedback,
        BinarySmiOp_Baseline,
        CallForwardVarargs,
        CallFunctionTemplate,
        CallFunctionTemplateGeneric,
        CallTrampoline,
        CallTrampoline_Baseline,
        CallTrampoline_Baseline_Compact,
        CallTrampoline_WithFeedback,
        CallVarargs,
        CallWithArrayLike,
        CallWithArrayLike_WithFeedback,
        CallWithSpread,
        CallWithSpread_Baseline,
        CallWithSpread_WithFeedback,
        CCall,
        CEntryDummy,
        CEntry1ArgvOnStack,
        CloneObjectBaseline,
        CloneObjectWithVector,
        Compare,
        CompareNoContext,
        StringEqual,
        Compare_Baseline,
        Compare_WithFeedback,
        Construct_Baseline,
        ConstructForwardVarargs,
        ConstructForwardAllArgs,
        ConstructForwardAllArgs_Baseline,
        ConstructForwardAllArgs_WithFeedback,
        ConstructStub,
        ConstructVarargs,
        ConstructWithArrayLike,
        Construct_WithFeedback,
        ConstructWithSpread,
        ConstructWithSpread_Baseline,
        ConstructWithSpread_WithFeedback,
        ContextOnly,
        CopyDataPropertiesWithExcludedProperties,
        CopyDataPropertiesWithExcludedPropertiesOnStack,
        CppBuiltinAdaptor,
        CreateFromSlowBoilerplateHelper,
        DefineKeyedOwn,
        DefineKeyedOwnBaseline,
        DefineKeyedOwnWithVector,
        FastNewObject,
        FindNonDefaultConstructorOrConstruct,
        ForInPrepare,
        GetIteratorStackParameter,
        GetProperty,
        GrowArrayElements,
        I32PairToBigInt,
        I64ToBigInt,
        InterpreterCEntry1,
        InterpreterCEntry2,
        InterpreterDispatch,
        InterpreterPushArgsThenCall,
        InterpreterPushArgsThenConstruct,
        JSTrampoline,
        KeyedHasICBaseline,
        KeyedHasICWithVector,
        KeyedLoad,
        KeyedLoadBaseline,
        EnumeratedKeyedLoadBaseline,
        KeyedLoadWithVector,
        EnumeratedKeyedLoad,
        Load,
        LoadBaseline,
        LoadGlobal,
        LoadGlobalBaseline,
        LoadGlobalNoFeedback,
        LoadGlobalWithVector,
        LoadNoFeedback,
        LoadWithReceiverAndVector,
        LoadWithReceiverBaseline,
        LoadWithVector,
        LookupWithVector,
        LookupTrampoline,
        LookupBaseline,
        MaglevOptimizeCodeOrTailCallOptimizedCodeSlot,
        NewHeapNumber,
        NoContext,
        OnStackReplacement,
        RegExpTrampoline,
        RestartFrameTrampoline,
        ResumeGenerator,
        ResumeGeneratorBaseline,
        RunMicrotasks,
        RunMicrotasksEntry,
        SingleParameterOnStack,
        Store,
        StoreNoFeedback,
        StoreBaseline,
        StoreGlobal,
        StoreGlobalBaseline,
        StoreGlobalWithVector,
        StoreTransition,
        StoreWithVector,
        StringAtAsString,
        StringSubstring,
        SuspendGeneratorBaseline,
        TypeConversion,
        TypeConversion_Baseline,
        TypeConversionNoContext,
        Typeof,
        UnaryOp_Baseline,
        UnaryOp_WithFeedback,
        Void,
        WasmDummy,
        WasmFloat32ToNumber,
        WasmFloat64ToTagged,
        WasmJSToWasmWrapper,
        WasmToJSWrapper,
        WasmSuspend,
        WasmHandleStackOverflow,
        WriteBarrier,
        IndirectPointerWriteBarrier,
        TSANLoad,
        TSANStore,
        Builtin_kAbort,
        Builtin_kAdd,
        Builtin_kArgumentsAdaptorTrampoline,
        Builtin_kArrayIncludes_Slow,
        Builtin_kArrayIndexOf,
        Builtin_kArrayIsArray,
        Builtin_kArrayIteratorNext,
        Builtin_kArrayJoinConcat,
        Builtin_kArrayPop,
        Builtin_kArrayPush,
        Builtin_kArrayShift,
        Builtin_kArraySlice,
        Builtin_kArrayUnshift,
        Builtin_kAsyncFunctionAwaitCaught,
        Builtin_kAsyncFunctionAwaitUncaught,
        Builtin_kAsyncFunctionReject,
        Builtin_kAsyncFunctionResolve,
        Builtin_kAsyncGeneratorAwaitCaught,
        Builtin_kAsyncGeneratorAwaitUncaught,
        Builtin_kAsyncGeneratorReject,
        Builtin_kAsyncGeneratorResolve,
        Builtin_kAsyncGeneratorReturn,
        Builtin_kBigIntAsIntN,
        Builtin_kBigIntAsUintN,
        Builtin_kBigIntDivide,
        Builtin_kBigIntMod,
        Builtin_kBooleanValueOf,
        Builtin_kCall,
        Builtin_kClassStaticBlock,
        Builtin_kCreateIterResultObject,
        Builtin_kDateNow,
        Builtin_kDateValueOf,
        Builtin_kDebugPrint,
        Builtin_kDecrement,
        Builtin_kDefineClass,
        Builtin_kDeleteProperty,
        Builtin_kDiv,
        Builtin_kErrorToString,
        Builtin_kEscape,
        Builtin_kGeneratorClose,
        Builtin_kGeneratorNext,
        Builtin_kGeneratorReturn,
        Builtin_kGetIterator,
        Builtin_kGetSuperConstructor,
        Builtin_kGlobalEval,
        Builtin_kHasProperty,
        Builtin_kIncrement,
        Builtin_kInstanceOf,
        Builtin_kInternalMapGet,
        Builtin_kInternalMapHas,
        Builtin_kInternalMapSet,
        Builtin_kInternalSetAdd,
        Builtin_kInternalWeakMapGet,
        Builtin_kInternalWeakMapHas,
        Builtin_kInternalWeakMapSet,
        Builtin_kInternalWeakSetAdd,
        Builtin_kIsArray,
        Builtin_kIsConstructor,
        Builtin_kIsFinite,
        Builtin_kIsInteger,
        Builtin_kIsNaN,
        Builtin_kJavascriptRuntimeGetHiddenValue,
        Builtin_kJavascriptRuntimeSetHiddenValue,
        Builtin_kJsonParse,
        Builtin_kJsonStringify,
        Builtin_kKeyedHasIC_Megamorphic,
        Builtin_kKeyedLoadIC_Megamorphic,
        Builtin_kKeyedStoreIC_Megamorphic,
        Builtin_kLoadIC_Megamorphic,
        Builtin_kMapClear,
        Builtin_kMapDelete,
        Builtin_kMapGet,
        Builtin_kMapHas,
        Builtin_kMapIteratorNext,
        Builtin_kMapSet,
        Builtin_kMathAcos,
        Builtin_kMathAsin,
        Builtin_kMathAtan,
        Builtin_kMathAtan2,
        Builtin_kMathCeil,
        Builtin_kMathCos,
        Builtin_kMathExp,
        Builtin_kMathExpm1,
        Builtin_kMathFloor,
        Builtin_kMathFround,
        Builtin_kMathLog,
        Builtin_kMathLog1p,
        Builtin_kMathMax,
        Builtin_kMathMin,
        Builtin_kMathPow,
        Builtin_kMathRandom,
        Builtin_kMathRound,
        Builtin_kMathSign,
        Builtin_kMathSin,
        Builtin_kMathSqrt,
        Builtin_kMathTan,
        Builtin_kNumberIsSafeInteger,
        Builtin_kNumberToString,
        Builtin_kNumberValueOf,
        Builtin_kObjectDefineProperties,
        Builtin_kObjectDefineProperty,
        Builtin_kObjectGetOwnPropertyNames,
        Builtin_kObjectGetPrototypeOf,
        Builtin_kObjectIs,
        Builtin_kObjectSetPrototypeOf,
        Builtin_kOrdinaryHasInstance,
        Builtin_kParseInt,
        Builtin_kPromiseGetCapabilitiesExecutor,
        Builtin_kPromiseThen,
        Builtin_kPromiseCatch,
        Builtin_kPromiseFinally,
        Builtin_kPromiseResolve,
        Builtin_kReflectApply,
        Builtin_kRegExpExec,
        Builtin_kRegExpTest,
        Builtin_kSetAdd,
        Builtin_kSetClear,
        Builtin_kSetDelete,
        Builtin_kSetHas,
        Builtin_kSetIteratorNext,
        Builtin_kStringFromSingleCharCode,
        Builtin_kStringReplace,
        Builtin_kStringReplaceAll,
        Builtin_kStringReplaceRegExp,
        Builtin_kStringSplit,
        Builtin_kStringSubstring,
        Builtin_kStringToNumber,
        Builtin_kStringValueOf,
        Builtin_kSub,
        Builtin_kSymbolValueOf,
        Builtin_kThrowIteratorResultNotAnObject,
        Builtin_kToBigInt,
        Builtin_kToBoolean,
        Builtin_kToNumber,
        Builtin_kToObject,
        Builtin_kToString,
        Builtin_kTryCatch,
        Builtin_kTryFinally,
        Builtin_kTypedArrayCopyWithin,
        Builtin_kTypedArrayFill,
        Builtin_kTypedArrayFilter,
        Builtin_kTypedArrayFind,
        Builtin_kTypedArrayFindIndex,
        Builtin_kTypedArrayForEach,
        Builtin_kTypedArrayJoin,
        Builtin_kTypedArrayMap,
        Builtin_kTypedArrayReduce,
        Builtin_kTypedArrayReduceRight,
        Builtin_kTypedArraySet,
        Builtin_kTypedArraySlice,
        Builtin_kTypedArraySort,
        Builtin_kTypedArraySubarray,
        Builtin_kUint8ClampedArraySort,
        Builtin_kWeakMapClear,
        Builtin_kWeakMapDelete,
        Builtin_kWeakMapGet,
        Builtin_kWeakMapHas,
        Builtin_kWeakMapSet,
        Builtin_kWeakSetAdd,
        Builtin_kWeakSetClear,
        Builtin_kWeakSetDelete,
        Builtin_kWeakSetHas,
        Builtin_kWasmCompileLazy,
        Builtin_kWasmInstantiate,
        Builtin_kWasmMemoryGrow,
        Builtin_kWasmRunInterpreter,
        Builtin_kWasmValidate,
        Builtin_kYield,
        Torque_Abort,
        Torque_ArrayIncludes_Slow,
        Torque_ArrayIndexOf,
        Torque_AsyncFunctionAwaitCaught,
        Torque_AsyncFunctionAwaitUncaught,
        Torque_AsyncFunctionReject,
        Torque_AsyncFunctionResolve,
        Torque_AsyncGeneratorAwaitCaught,
        Torque_AsyncGeneratorAwaitUncaught,
        Torque_AsyncGeneratorReject,
        Torque_AsyncGeneratorResolve,
        Torque_AsyncGeneratorReturn,
        Torque_BigIntAsIntN,
        Torque_BigIntAsUintN,
        Torque_BigIntDivide,
        Torque_BigIntMod,
        Torque_Call,
        Torque_ClassStaticBlock,
        Torque_CreateIterResultObject,
        Torque_DateNow,
        Torque_DebugPrint,
        Torque_DefineClass,
        Torque_DeleteProperty,
        Torque_GeneratorClose,
        Torque_GeneratorNext,
        Torque_GeneratorReturn,
        Torque_GetIterator,
        Torque_GetSuperConstructor,
        Torque_GlobalEval,
        Torque_HasProperty,
        Torque_InstanceOf,
        Torque_InternalMapGet,
        Torque_InternalMapHas,
        Torque_InternalMapSet,
        Torque_InternalSetAdd,
        Torque_InternalWeakMapGet,
        Torque_InternalWeakMapHas,
        Torque_InternalWeakMapSet,
        Torque_InternalWeakSetAdd,
        Torque_IsArray,
        Torque_IsConstructor,
        Torque_IsFinite,
        Torque_IsInteger,
        Torque_IsNaN,
        Torque_JavascriptRuntimeGetHiddenValue,
        Torque_JavascriptRuntimeSetHiddenValue,
        Torque_JsonParse,
        Torque_JsonStringify,
        Torque_KeyedHasIC_Megamorphic,
        Torque_KeyedLoadIC_Megamorphic,
        Torque_KeyedStoreIC_Megamorphic,
        Torque_LoadIC_Megamorphic,
        Torque_MapClear,
        Torque_MapDelete,
        Torque_MapGet,
        Torque_MapHas,
        Torque_MapIteratorNext,
        Torque_MapSet,
        Torque_MathAcos,
        Torque_MathAsin,
        Torque_MathAtan,
        Torque_MathAtan2,
        Torque_MathCeil,
        Torque_MathCos,
        Torque_MathExp,
        Torque_MathExpm1,
        Torque_MathFloor,
        Torque_MathFround,
        Torque_MathLog,
        Torque_MathLog1p,
        Torque_MathMax,
        Torque_MathMin,
        Torque_MathPow,
        Torque_MathRandom,
        Torque_MathRound,
        Torque_MathSign,
        Torque_MathSin,
        Torque_MathSqrt,
        Torque_MathTan,
        Torque_NumberIsSafeInteger,
        Torque_NumberToString,
        Torque_ObjectDefineProperties,
        Torque_ObjectDefineProperty,
        Torque_ObjectGetOwnPropertyNames,
        Torque_ObjectGetPrototypeOf,
        Torque_ObjectIs,
        Torque_ObjectSetPrototypeOf,
        Torque_OrdinaryHasInstance,
        Torque_ParseInt,
        Torque_PromiseGetCapabilitiesExecutor,
        Torque_PromiseThen,
        Torque_PromiseCatch,
        Torque_PromiseFinally,
        Torque_PromiseResolve,
        Torque_ReflectApply,
        Torque_RegExpExec,
        Torque_RegExpTest,
        Torque_SetAdd,
        Torque_SetClear,
        Torque_SetDelete,
        Torque_SetHas,
        Torque_SetIteratorNext,
        Torque_StringFromSingleCharCode,
        Torque_StringReplace,
        Torque_StringReplaceAll,
        Torque_StringReplaceRegExp,
        Torque_StringSplit,
        Torque_StringSubstring,
        Torque_StringToNumber,
        Torque_Sub,
        Torque_SymbolValueOf,
        Torque_ThrowIteratorResultNotAnObject,
        Torque_ToBigInt,
        Torque_ToBoolean,
        Torque_ToNumber,
        Torque_ToObject,
        Torque_ToString,
        Torque_TryCatch,
        Torque_TryFinally,
        Torque_TypedArrayCopyWithin,
        Torque_TypedArrayFill,
        Torque_TypedArrayFilter,
        Torque_TypedArrayFind,
        Torque_TypedArrayFindIndex,
        Torque_TypedArrayForEach,
        Torque_TypedArrayJoin,
        Torque_TypedArrayMap,
        Torque_TypedArrayReduce,
        Torque_TypedArrayReduceRight,
        Torque_TypedArraySet,
        Torque_TypedArraySlice,
        Torque_TypedArraySort,
        Torque_TypedArraySubarray,
        Torque_Uint8ClampedArraySort,
        Torque_WeakMapClear,
        Torque_WeakMapDelete,
        Torque_WeakMapGet,
        Torque_WeakMapHas,
        Torque_WeakMapSet,
        Torque_WeakSetAdd,
        Torque_WeakSetClear,
        Torque_WeakSetDelete,
        Torque_WeakSetHas,
        Torque_WasmCompileLazy,
        Torque_WasmInstantiate,
        Torque_WasmMemoryGrow,
        Torque_WasmRunInterpreter,
        Torque_WasmValidate,
        Torque_Yield,
        NUMBER_OF_DESCRIPTORS,
    }

    const K_MAX_BUILTIN_REGISTER_PARAMS: i32 = 5;
    const K_JS_BUILTIN_REGISTER_PARAMS: i32 = 4;
    const K_MAX_TFS_BUILTIN_REGISTER_PARAMS: i32 = 5;

    #[derive(Debug)]
    pub struct CallInterfaceDescriptor {
        data_: *mut CallInterfaceDescriptorData,
    }

    impl CallInterfaceDescriptor {
        pub fn new() -> Self {
            CallInterfaceDescriptor {
                data_: ptr::null_mut(),
            }
        }

        pub fn with_key(key: CallDescriptorsKey) -> Self {
            CallInterfaceDescriptor {
                data_: CallDescriptors::call_descriptor_data(key),
            }
        }

        pub fn flags(&self) -> Flags {
            unsafe { (*self.data_).flags() }
        }

        pub fn tag(&self) -> CodeEntrypointTag {
            unsafe { (*self.data_).tag() }
        }

        pub fn has_context_parameter(&self) -> bool {
            !self.flags().contains(Flags::NO_CONTEXT)
        }

        pub fn allow_var_args(&self) -> bool {
            self.flags().contains(Flags::ALLOW_VAR_ARGS)
        }

        pub fn callee_save_registers(&self) -> bool {
            self.flags().contains(Flags::CALLEE_SAVE_REGISTERS)
        }

        pub fn get_return_count(&self) -> i32 {
            unsafe { (*self.data_).return_count() }
        }

        pub fn get_return_type(&self, index: i32) -> MachineType {
            unsafe { (*self.data_).return_type(index) }
        }

        pub fn get_parameter_count(&self) -> i32 {
            unsafe { (*self.data_).param_count() }
        }

        pub fn get_register_parameter_count(&self) -> i32 {
            unsafe { (*self.data_).register_param_count() }
        }

        pub fn get_stack_parameter_count(&self) -> i32 {
            unsafe { (*self.data_).param_count() - (*self.data_).register_param_count() }
        }

        pub fn get_register_parameter(&self, index: i32) -> Register {
            unsafe { (*self.data_).register_param(index) }
        }

        pub fn get_double_register_parameter(&self, index: i32) -> DoubleRegister {
            unsafe { (*