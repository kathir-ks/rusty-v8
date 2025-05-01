// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod builtin_call_descriptors {
    use std::marker::PhantomData;

    //use crate::builtins::builtins::Builtin; // Assuming a similar enum exists in Rust
    //use crate::codegen::callable::Callable; // Assuming a similar struct exists in Rust
    //use crate::codegen::interface_descriptors::CallInterfaceDescriptor; // Assuming a similar struct exists in Rust
    //use crate::compiler::frame::Frame; // Assuming a similar struct exists in Rust
    //use crate::compiler::globals::*; // Assuming definitions for globals exist in Rust
    //use crate::compiler::turboshaft::operations::*; // Assuming definitions for operations exist in Rust
    //use crate::compiler::turboshaft::representations::*; // Assuming definitions for representations exist in Rust
    //use crate::compiler::write_barrier_kind::WriteBarrierKind; // Assuming a similar enum exists in Rust
    //use crate::objects::js_function::JSFunction; // Assuming a similar struct exists in Rust

    // Dummy definitions for types that aren't fully convertible without other codebase files.
    pub struct TSCallDescriptor {}
    pub enum Builtin {
        kCheckTurbofanType,
        kToNumber,
        kNonNumberToNumber,
        kToNumeric,
        kNonNumberToNumeric,
        kCopyFastSmiOrObjectElements,
        kDebugPrintFloat64,
        kDebugPrintWordPtr,
        kFindOrderedHashMapEntry,
        kFindOrderedHashSetEntry,
        kGrowFastDoubleElements,
        kGrowFastSmiOrObjectElements,
        kNewSloppyArgumentsElements,
        kNewStrictArgumentsElements,
        kNewRestArgumentsElements,
        kNumberToString,
        kToString,
        kPlainPrimitiveToNumber,
        kSameValue,
        kSameValueNumbersOnly,
        kStringAdd_CheckNone,
        kStringEqual,
        kStringFromCodePointAt,
        kStringIndexOf,
        kStringCompare,
        kStringLessThan,
        kStringLessThanOrEqual,
        kStringSubstring,
        kStringToLowerCaseIntl,
        kStringToNumber,
        kToBoolean,
        kToObject,
        kFastNewFunctionContextFunction,
        kFastNewFunctionContextEval,
        kFastNewClosure,
        kTypeof,
        kCheckTurboshaftWord32Type,
        kCheckTurboshaftWord64Type,
        kCheckTurboshaftFloat32Type,
        kCheckTurboshaftFloat64Type,
        kWasmStringAsWtf8,
        kWasmStringAsWtf16,
        kWasmInt32ToHeapNumber,
        kWasmRefFunc,
        kWasmGetOwnProperty,
        kWasmRethrow,
        kWasmThrowRef,
        kWasmMemoryGrow,
        kWasmStringFromCodePoint,
        kWasmStringNewWtf8Array,
        kWasmStringNewWtf16Array,
        kWasmStringViewWtf8Slice,
        kWasmStringViewWtf16Slice,
        kWasmStringEncodeWtf8Array,
        kWasmStringToUtf8Array,
        kWasmStringEncodeWtf16Array,
        kWasmFloat64ToString,
        kWasmIntToString,
        kWasmStringToDouble,
        kWasmAllocateFixedArray,
        kWasmThrow,
        kWasmI32AtomicWait,
        kWasmI64AtomicWait,
        kWasmFunctionTableGet,
        kWasmTableSetFuncRef,
        kWasmTableSet,
        kWasmTableInit,
        kWasmTableCopy,
        kWasmTableGrow,
        kWasmTableFill,
        kWasmArrayNewSegment,
        kWasmArrayInitSegment,
        kWasmStringNewWtf8,
        kWasmStringNewWtf16,
        kWasmStringFromDataSegment,
        kWasmStringConst,
        kWasmStringMeasureUtf8,
        kWasmStringMeasureWtf8,
        kWasmStringEncodeWtf8,
        kWasmStringEncodeWtf16,
        kWasmStringEqual,
        kWasmStringIsUSVSequence,
        kWasmStringViewWtf8Advance,
        kWasmStringViewWtf8Encode,
        kWasmStringViewWtf16Encode,
        kWasmStringViewWtf16GetCodeUnit,
        kWasmStringCodePointAt,
        kWasmStringAsIter,
        kWasmStringViewIterNext,
        kWasmStringViewIterAdvance,
        kWasmStringViewIterRewind,
        kWasmStringViewIterSlice,
        kWasmStringHash,
        kThrowDataViewDetachedError,
        kThrowDataViewOutOfBounds,
        kThrowDataViewTypeError,
        kThrowIndexOfCalledOnNull,
        kThrowToLowerCaseCalledOnNull,
        kWasmFastApiCallTypeCheckAndUpdateIC,
        kWasmPropagateException,
    }

    pub struct Object {}
    pub struct TurbofanType {}
    pub struct Smi {}
    pub struct Number {}
    pub struct JSAnyNotNumber {}
    pub struct Numeric {}
    pub struct String {}
    pub struct Boolean {}
    pub struct JSPrimitive {}
    pub struct JSReceiver {}
    pub struct ScopeInfo {}
    pub struct Word32 {}
    pub struct WordPtr {}
    pub struct Context {}
    pub struct SharedFunctionInfo {}
    pub struct FeedbackCell {}
    pub struct JSFunction {}
    pub struct Oddball {}
    pub struct Float32 {}
    pub struct Float64 {}
    pub struct TurboshaftWord32Type {}
    pub struct TurboshaftWord64Type {}
    pub struct TurboshaftFloat32Type {}
    pub struct TurboshaftFloat64Type {}
    pub struct ByteArray {}
    pub struct HeapNumber {}
    pub struct WasmFuncRef {}
    pub struct Symbol {}
    pub struct WasmArray {}
    pub struct WasmStringRefNullable {}
    pub struct Map {}
    pub struct WasmStringViewIter {}
    pub struct JSDataView {}
    pub struct BigInt {}
    pub struct FixedArray {}
    pub struct HeapObject {}

    #[derive(Clone, Copy)]
    pub enum CanThrow {
        Yes,
        No,
    }

    // Dummy definitions for traits/types used in V
    pub trait AllowsRepresentation {
        fn allows_representation(_rep: RegisterRepresentation) -> bool;
    }
    #[derive(Debug, Clone, Copy)]
    pub enum RegisterRepresentation {}

    pub struct V<T> {
        _phantom: PhantomData<T>,
    }

    impl<T> V<T> {
        pub fn new() -> Self {
            V { _phantom: PhantomData }
        }
    }

    impl AllowsRepresentation for V<Object> {
        fn allows_representation(_rep: RegisterRepresentation) -> bool { true }
    }
    impl AllowsRepresentation for V<TurbofanType> {
        fn allows_representation(_rep: RegisterRepresentation) -> bool { true }
    }
    impl AllowsRepresentation for V<Smi> {
        fn allows_representation(_rep: RegisterRepresentation) -> bool { true }
    }
    impl AllowsRepresentation for V<Number> {
        fn allows_representation(_rep: RegisterRepresentation) -> bool { true }
    }
    impl AllowsRepresentation for V<JSAnyNotNumber> {
        fn allows_representation(_rep: RegisterRepresentation) -> bool { true }
    }
    impl AllowsRepresentation for V<Numeric> {
        fn allows_representation(_rep: RegisterRepresentation) -> bool { true }
    }
    impl AllowsRepresentation for V<String> {
        fn allows_representation(_rep: RegisterRepresentation) -> bool { true }
    }
    impl AllowsRepresentation for V<Boolean> {
        fn allows_representation(_rep: RegisterRepresentation) -> bool { true }
    }
    impl AllowsRepresentation for V<JSPrimitive> {
        fn allows_representation(_rep: RegisterRepresentation) -> bool { true }
    }
    impl AllowsRepresentation for V<JSReceiver> {
        fn allows_representation(_rep: RegisterRepresentation) -> bool { true }
    }
    impl AllowsRepresentation for V<ScopeInfo> {
        fn allows_representation(_rep: RegisterRepresentation) -> bool { true }
    }
    impl AllowsRepresentation for V<Word32> {
        fn allows_representation(_rep: RegisterRepresentation) -> bool { true }
    }
    impl AllowsRepresentation for V<WordPtr> {
        fn allows_representation(_rep: RegisterRepresentation) -> bool { true }
    }
    impl AllowsRepresentation for V<Context> {
        fn allows_representation(_rep: RegisterRepresentation) -> bool { true }
    }
    impl AllowsRepresentation for V<SharedFunctionInfo> {
        fn allows_representation(_rep: RegisterRepresentation) -> bool { true }
    }
    impl AllowsRepresentation for V<FeedbackCell> {
        fn allows_representation(_rep: RegisterRepresentation) -> bool { true }
    }
    impl AllowsRepresentation for V<JSFunction> {
        fn allows_representation(_rep: RegisterRepresentation) -> bool { true }
    }
    impl AllowsRepresentation for V<Oddball> {
        fn allows_representation(_rep: RegisterRepresentation) -> bool { true }
    }
    impl AllowsRepresentation for V<Float32> {
        fn allows_representation(_rep: RegisterRepresentation) -> bool { true }
    }
    impl AllowsRepresentation for V<Float64> {
        fn allows_representation(_rep: RegisterRepresentation) -> bool { true }
    }
    impl AllowsRepresentation for V<TurboshaftWord32Type> {
        fn allows_representation(_rep: RegisterRepresentation) -> bool { true }
    }
    impl AllowsRepresentation for V<TurboshaftWord64Type> {
        fn allows_representation(_rep: RegisterRepresentation) -> bool { true }
    }
    impl AllowsRepresentation for V<TurboshaftFloat32Type> {
        fn allows_representation(_rep: RegisterRepresentation) -> bool { true }
    }
    impl AllowsRepresentation for V<TurboshaftFloat64Type> {
        fn allows_representation(_rep: RegisterRepresentation) -> bool { true }
    }
    impl AllowsRepresentation for V<ByteArray> {
        fn allows_representation(_rep: RegisterRepresentation) -> bool { true }
    }
    impl AllowsRepresentation for V<HeapNumber> {
        fn allows_representation(_rep: RegisterRepresentation) -> bool { true }
    }
    impl AllowsRepresentation for V<WasmFuncRef> {
        fn allows_representation(_rep: RegisterRepresentation) -> bool { true }
    }
    impl AllowsRepresentation for V<Symbol> {
        fn allows_representation(_rep: RegisterRepresentation) -> bool { true }
    }
    impl AllowsRepresentation for V<WasmArray> {
        fn allows_representation(_rep: RegisterRepresentation) -> bool { true }
    }
    impl AllowsRepresentation for V<WasmStringRefNullable> {
        fn allows_representation(_rep: RegisterRepresentation) -> bool { true }
    }
    impl AllowsRepresentation for V<Map> {
        fn allows_representation(_rep: RegisterRepresentation) -> bool { true }
    }
    impl AllowsRepresentation for V<WasmStringViewIter> {
        fn allows_representation(_rep: RegisterRepresentation) -> bool { true }
    }
    impl AllowsRepresentation for V<JSDataView> {
        fn allows_representation(_rep: RegisterRepresentation) -> bool { true }
    }
    impl AllowsRepresentation for V<BigInt> {
        fn allows_representation(_rep: RegisterRepresentation) -> bool { true }
    }
    impl AllowsRepresentation for V<FixedArray> {
        fn allows_representation(_rep: RegisterRepresentation) -> bool { true }
    }
    impl AllowsRepresentation for V<HeapObject> {
        fn allows_representation(_rep: RegisterRepresentation) -> bool { true }
    }

    pub type OpIndex = usize;
    impl AllowsRepresentation for OpIndex {
        fn allows_representation(_rep: RegisterRepresentation) -> bool { true }
    }

    #[derive(Clone, Copy)]
    pub enum StubCallMode {}
    #[derive(Clone, Copy)]
    pub enum LazyDeoptOnThrow {
        kNo,
    }

    pub struct CallDescriptor {}
    impl CallDescriptor {
        pub const kNeedsFrameState: i32 = 1;
        pub const kNoFlags: i32 = 0;

        pub fn ReturnCount(&self) -> usize { 0 }
        pub fn NeedsFrameState(&self) -> i32 { 0 }
        pub fn properties(&self) -> i32 { 0 }
        pub fn ParameterCount(&self) -> usize { 0 }
        pub fn GetReturnType(&self, _index: usize) -> Type { Type {} }
        pub fn GetParameterType(&self, _index: usize) -> Type { Type {} }
    }

    pub struct Type {}
    impl Type {
        pub fn representation(&self) -> MachineRepresentation { MachineRepresentation::Any }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum MachineRepresentation {
        Any,
    }

    pub struct Zone {}

    pub struct Linkage {}
    impl Linkage {
        pub fn GetStubCallDescriptor(
            _zone: *mut Zone,
            _interface_descriptor: CallInterfaceDescriptor,
            _stack_parameter_count: i32,
            _flags: i32,
            _properties: i32,
            _call_mode: StubCallMode,
        ) -> *mut CallDescriptor {
            // Return a raw pointer.  Memory management handled elsewhere (presumably).
            Box::into_raw(Box::new(CallDescriptor {}))
        }
    }

    pub struct CallInterfaceDescriptor {}

    pub struct Operator {}
    impl Operator {
        pub const kNoThrow: i32 = 1;
        pub const kNoDeopt: i32 = 2;
        pub const kNoWrite: i32 = 4;
        pub const kEliminatable: i32 = 8;
        pub const kPure: i32 = 16;
        pub const kNoProperties: i32 = 0;
    }

    pub struct OpEffects {
        can_depend_on_checks: bool,
        can_read_memory: bool,
        can_write_memory: bool,
        can_allocate: bool,
        can_call_anything: bool,
        can_change_control_flow: bool,
        assumes_consistent_heap: bool,
        can_leave_current_function: bool,
        can_write_heap_memory: bool,
        can_read_heap_memory: bool,
        allocate_without_identity: bool,
        required_when_unused: bool,
    }

    impl OpEffects {
        pub const fn new() -> Self {
            OpEffects {
                can_depend_on_checks: false,
                can_read_memory: false,
                can_write_memory: false,
                can_allocate: false,
                can_call_anything: false,
                can_change_control_flow: false,
                assumes_consistent_heap: false,
                can_leave_current_function: false,
                can_write_heap_memory: false,
                can_read_heap_memory: false,
                allocate_without_identity: false,
                required_when_unused: false,
            }
        }
        pub const fn CanDependOnChecks(mut self) -> Self {
            self.can_depend_on_checks = true;
            self
        }
        pub const fn CanReadMemory(mut self) -> Self {
            self.can_read_memory = true;
            self
        }
        pub const fn CanWriteMemory(mut self) -> Self {
            self.can_write_memory = true;
            self
        }
        pub const fn CanAllocate(mut self) -> Self {
            self.can_allocate = true;
            self
        }
        pub const fn CanCallAnything(mut self) -> Self {
            self.can_call_anything = true;
            self
        }
        pub const fn CanChangeControlFlow(mut self) -> Self {
            self.can_change_control_flow = true;
            self
        }
        pub const fn AssumesConsistentHeap(mut self) -> Self {
            self.assumes_consistent_heap = true;
            self
        }
        pub const fn CanLeaveCurrentFunction(mut self) -> Self {
            self.can_leave_current_function = true;
            self
        }
        pub const fn CanWriteHeapMemory(mut self) -> Self {
            self.can_write_heap_memory = true;
            self
        }
        pub const fn CanReadHeapMemory(mut self) -> Self {
            self.can_read_heap_memory = true;
            self
        }
        pub const fn CanAllocateWithoutIdentity(mut self) -> Self {
            self.allocate_without_identity = true;
            self
        }
        pub const fn RequiredWhenUnused(mut self) -> Self {
            self.required_when_unused = true;
            self
        }
    }

    pub struct BuiltinCallDescriptor {}

    impl BuiltinCallDescriptor {
        const BASE_EFFECTS: OpEffects = OpEffects::new().CanDependOnChecks();

        // TODO(nicohartmann@): Unfortunately, we cannot define builtins with
        // void/never return types properly (e.g. in Torque), but they typically have
        // a JSAny dummy return type. Use Void/Never sentinels to express that in
        // Turboshaft's descriptors. We should find a better way to model this.
        pub type Void = std::tuple<OpIndex>;
        pub type Never = std::tuple<OpIndex>;

        pub struct CheckTurbofanType {}
        impl CheckTurbofanType {
            pub const FUNCTION: Builtin = Builtin::kCheckTurbofanType;
            pub type arguments_t = std::tuple<V<Object>, V<TurbofanType>, V<Smi>>;
            pub type results_t = std::tuple<V<Object>>;

            pub const NEEDS_FRAME_STATE: bool = false;
            pub const NEEDS_CONTEXT: bool = true;
            pub const PROPERTIES: i32 = Operator::kNoThrow | Operator::kNoDeopt;
            pub const EFFECTS: OpEffects =
                BuiltinCallDescriptor::BASE_EFFECTS.CanReadMemory().RequiredWhenUnused();

            pub fn create(_call_mode: StubCallMode, _zone: *mut Zone, _lazy_deopt_on_throw: LazyDeoptOnThrow) -> *mut TSCallDescriptor {
                Box::into_raw(Box::new(TSCallDescriptor {}))
            }
        }

        macro_rules! decl_generic_binop {
            ($name:ident, $builtin:ident) => {
                pub struct $name {}
                impl $name {
                    pub const FUNCTION: Builtin = Builtin::$builtin;
                    pub type arguments_t = std::tuple<V<Object>, V<Object>>;
                    pub type results_t = std::tuple<V<Object>>;

                    pub const NEEDS_FRAME_STATE: bool = true;
                    pub const NEEDS_CONTEXT: bool = true;
                    pub const PROPERTIES: i32 = Operator::kNoProperties;
                    pub const EFFECTS: OpEffects =
                        BuiltinCallDescriptor::BASE_EFFECTS.CanCallAnything();

                        pub fn create(_call_mode: StubCallMode, _zone: *mut Zone, _lazy_deopt_on_throw: LazyDeoptOnThrow) -> *mut TSCallDescriptor {
                            Box::into_raw(Box::new(TSCallDescriptor {}))
                        }
                }
            };
        }

        decl_generic_binop!(Add, Add);
        decl_generic_binop!(Subtract, Subtract);
        decl_generic_binop!(Multiply, Multiply);
        decl_generic_binop!(Divide, Divide);
        decl_generic_binop!(Modulus, Modulus);
        decl_generic_binop!(BitwiseAnd, BitwiseAnd);
        decl_generic_binop!(BitwiseOr, BitwiseOr);
        decl_generic_binop!(BitwiseXor, BitwiseXor);
        decl_generic_binop!(ShiftLeft, ShiftLeft);
        decl_generic_binop!(ShiftRight, ShiftRight);
        decl_generic_binop!(ShiftRightLogical, ShiftRightLogical);
        decl_generic_binop!(Exponentiate, Exponentiate);

        macro_rules! decl_generic_unop {
            ($name:ident, $builtin:ident) => {
                pub struct $name {}
                impl $name {
                    pub const FUNCTION: Builtin = Builtin::$builtin;
                    pub type arguments_t = std::tuple<V<Object>>;
                    pub type results_t = std::tuple<V<Object>>;

                    pub const NEEDS_FRAME_STATE: bool = true;
                    pub const NEEDS_CONTEXT: bool = true;
                    pub const PROPERTIES: i32 = Operator::kNoProperties;
                    pub const EFFECTS: OpEffects =
                        BuiltinCallDescriptor::BASE_EFFECTS.CanCallAnything();

                        pub fn create(_call_mode: StubCallMode, _zone: *mut Zone, _lazy_deopt_on_throw: LazyDeoptOnThrow) -> *mut TSCallDescriptor {
                            Box::into_raw(Box::new(TSCallDescriptor {}))
                        }
                }
            };
        }

        decl_generic_unop!(Decrement, Decrement);
        decl_generic_unop!(Increment, Increment);
        decl_generic_unop!(Negate, Negate);
        decl_generic_unop!(BitwiseNot, BitwiseNot);

        pub struct ToNumber {}
        impl ToNumber {
            pub const FUNCTION: Builtin = Builtin::kToNumber;
            pub type arguments_t = std::tuple<V<Object>>;
            pub type results_t = std::tuple<V<Number>>;

            pub const NEEDS_FRAME_STATE: bool = true;
            pub const NEEDS_CONTEXT: bool = true;
            pub const PROPERTIES: i32 = Operator::kNoProperties;
            pub const EFFECTS: OpEffects =
                BuiltinCallDescriptor::BASE_EFFECTS.CanCallAnything();

            pub fn create(_call_mode: StubCallMode, _zone: *mut Zone, _lazy_deopt_on_throw: LazyDeoptOnThrow) -> *mut TSCallDescriptor {
                Box::into_raw(Box::new(TSCallDescriptor {}))
            }
        }

        pub struct NonNumberToNumber {}
        impl NonNumberToNumber {
            pub const FUNCTION: Builtin = Builtin::kNonNumberToNumber;
            pub type arguments_t = std::tuple<V<JSAnyNotNumber>>;
            pub type results_t = std::tuple<V<Number>>;

            pub const NEEDS_FRAME_STATE: bool = false;
            pub const NEEDS_CONTEXT: bool = true;
            pub const PROPERTIES: i32 = Operator::kNoProperties;
            pub const EFFECTS: OpEffects =
                BuiltinCallDescriptor::BASE_EFFECTS.CanCallAnything();
                
            pub fn create(_call_mode: StubCallMode, _zone: *mut Zone, _lazy_deopt_on_throw: LazyDeoptOnThrow) -> *mut TSCallDescriptor {
                Box::into_raw(Box::new(TSCallDescriptor {}))
            }
        }

        pub struct ToNumeric {}
        impl ToNumeric {
            pub const FUNCTION: Builtin = Builtin::kToNumeric;
            pub type arguments_t = std::tuple<V<Object>>;
            pub type results_t = std::tuple<V<Numeric>>;

            pub const NEEDS_FRAME_STATE: bool = true;
            pub const NEEDS_CONTEXT: bool = true;
            pub const PROPERTIES: i32 = Operator::kNoProperties;
            pub const EFFECTS: OpEffects =
                BuiltinCallDescriptor::BASE_EFFECTS.CanCallAnything();
                
            pub fn create(_call_mode: StubCallMode, _zone: *mut Zone, _lazy_deopt_on_throw: LazyDeoptOnThrow) -> *mut TSCallDescriptor {
                Box::into_raw(Box::new(TSCallDescriptor {}))
            }
        }

        pub struct NonNumberToNumeric {}
        impl NonNumberToNumeric {
            pub const FUNCTION: Builtin = Builtin::kNonNumberToNumeric;
            pub type arguments_t = std::tuple<V<JSAnyNotNumber>>;
            pub type results_t = std::tuple<V<Numeric>>;

            pub const NEEDS_FRAME_STATE: bool = false;
            pub const NEEDS_CONTEXT: bool = true;
            pub const PROPERTIES: i32 = Operator::kNoProperties;
            pub const EFFECTS: OpEffects =
                BuiltinCallDescriptor::BASE_EFFECTS.CanCallAnything();

            pub fn create(_call_mode: StubCallMode, _zone: *mut Zone, _lazy_deopt_on_throw: LazyDeoptOnThrow) -> *mut TSCallDescriptor {
                Box::into_raw(Box::new(TSCallDescriptor {}))
            }
        }

        pub struct CopyFastSmiOrObjectElements {}
        impl CopyFastSmiOrObjectElements {
            pub const FUNCTION: Builtin = Builtin::kCopyFastSmiOrObjectElements;
            pub type arguments_t = std::tuple<V<Object>>;
            pub type results_t = std::tuple<V<Object>>;

            pub const NEEDS_FRAME_STATE: bool = false;
            pub const NEEDS_CONTEXT: bool = false;
            pub const PROPERTIES: i32 = Operator::kEliminatable;
            pub const EFFECTS: OpEffects = BuiltinCallDescriptor::BASE_EFFECTS
                .CanWriteMemory()
                .CanReadMemory()
                .CanAllocate();
            
            pub fn create(_call_mode: StubCallMode, _zone: *mut Zone, _lazy_deopt_on_throw: LazyDeoptOnThrow) -> *mut TSCallDescriptor {
                Box::into_raw(Box::new(TSCallDescriptor {}))
            }
        }

        pub struct DebugPrint<B, Input> {
            _builtin: PhantomData<B>,
            _input: PhantomData<Input>,
        }

        impl<B, Input> DebugPrint<B, Input> {
            pub const FUNCTION: Builtin = match stringify!(B).as_str() {
                "Float64" => Builtin::kDebugPrintFloat64,
                "WordPtr" => Builtin::kDebugPrintWordPtr,
                _ => panic!("Unsupported builtin type for DebugPrint"),
            };

            pub type arguments_t = std::tuple<V<Input>>;
            pub type results_t = std::tuple<V<Object>>;

            pub const NEEDS_FRAME_STATE: bool = false;
            pub const NEEDS_CONTEXT: bool = true;
            pub const PROPERTIES: i32 = Operator::kNoThrow | Operator::kNoDeopt;
            pub const EFFECTS: OpEffects =
                BuiltinCallDescriptor::BASE_EFFECTS.RequiredWhenUnused();
            
            pub fn create(_call_mode: StubCallMode, _zone: *mut Zone, _lazy_deopt_on_throw: LazyDeoptOnThrow) -> *mut TSCallDescriptor {
                Box::into_raw(Box::new(TSCallDescriptor {}))
            }
        }

        pub type DebugPrintFloat64 = DebugPrint<Float64, Float64>;
        pub type DebugPrintWordPtr = DebugPrint<WordPtr, WordPtr>;

        macro_rules! stringify {
            ($tok:tt) => {
                stringify!($tok)
            };
        }

        pub struct FindOrderedHashEntry<B> {
            _builtin: PhantomData<B>,
        }

        impl<B> FindOrderedHashEntry<B> {
            pub const FUNCTION: Builtin = match stringify!(B).as_str() {
                "HashMap" => Builtin::kFindOrderedHashMapEntry,
                "HashSet" => Builtin::kFindOrderedHashSetEntry,
                _ => panic!("Unsupported builtin type for FindOrderedHashEntry"),
            };

            pub type arguments_t = std::tuple<V<Object>, V<Smi>>;
            pub type results_t = std::tuple<V<Smi>>;

            pub const NEEDS_FRAME_STATE: bool = false;
            pub const NEEDS_CONTEXT: bool = true;
            pub const PROPERTIES: i32 = Operator::kEliminatable;
            pub const EFFECTS: OpEffects = BuiltinCallDescriptor::BASE_EFFECTS
                .AssumesConsistentHeap()
                .CanReadMemory()
                .CanAllocate();
            
            pub fn create(_call_mode: StubCallMode, _zone: *mut Zone, _lazy_deopt_on_throw: LazyDeoptOnThrow) -> *mut TSCallDescriptor {
                Box::into_raw(Box::new(TSCallDescriptor {}))
            }
        }

        pub type FindOrderedHashMapEntry = FindOrderedHashEntry<HashMap>;
        pub type FindOrderedHashSetEntry = FindOrderedHashEntry<HashSet>;

        pub struct HashMap {}
        pub struct HashSet {}

        pub struct GrowFastElements<B> {
            _builtin: PhantomData<B>,
        }

        impl<B> GrowFastElements<B> {
            pub const FUNCTION: Builtin = match stringify!(B).as_str() {
                "DoubleElements" => Builtin::kGrowFastDoubleElements,
                "SmiOrObjectElements" => Builtin::kGrowFastSmiOrObjectElements,
                _ => panic!("Unsupported builtin type for GrowFastElements"),
            };
            pub type arguments_t = std::tuple<V<Object>, V<Smi>>;
            pub type results_t = std::tuple<V<Object>>;

            pub const NEEDS_FRAME_STATE: bool = false;
            pub const NEEDS_CONTEXT: bool = false;
            pub const PROPERTIES: i32 = Operator::kEliminatable;
            pub const EFFECTS: OpEffects = BuiltinCallDescriptor::BASE_EFFECTS
                .CanWriteMemory()
                .CanReadMemory()
                .CanAllocate();
            
            pub fn create(_call_mode: StubCallMode, _zone: *mut Zone, _lazy_deopt_on_throw: LazyDeoptOnThrow) -> *mut TSCallDescriptor {
                Box::into_raw(Box::new(TSCallDescriptor {}))
            }
        }

        pub type GrowFastDoubleElements = GrowFastElements<DoubleElements>;
        pub type GrowFastSmiOrObjectElements = GrowFastElements<SmiOrObjectElements>;

        pub struct DoubleElements {}
        pub struct SmiOrObjectElements {}

        pub struct NewArgumentsElements<B> {
            _builtin: PhantomData<B>,
        }

        impl<B> NewArgumentsElements<B> {
            pub const FUNCTION: Builtin = match stringify!(B).as_str() {
                "SloppyArgumentsElements" => Builtin::kNewSloppyArgumentsElements,
                "StrictArgumentsElements" => Builtin::kNewStrictArgumentsElements,
                "RestArgumentsElements" => Builtin::kNewRestArgumentsElements,
                _ => panic!("Unsupported builtin type for NewArgumentsElements"),
            };

            // TODO(nicohartmann@): First argument should be replaced by a proper
            // RawPtr.
            pub type arguments_t = std::tuple<V<WordPtr>, V<WordPtr>, V<Smi>>;
            pub type results_t = std::tuple<V<FixedArray>>;

            pub const NEEDS_FRAME_STATE: bool = false;
            pub const NEEDS_CONTEXT: bool = false;
            pub const PROPERTIES: i32 = Operator::kEliminatable;
            pub const EFFECTS: OpEffects = BuiltinCallDescriptor::BASE_EFFECTS.CanAllocate();
            
            pub fn create(_call_mode: StubCallMode, _zone: *mut Zone, _lazy_deopt_on_throw: LazyDeoptOnThrow) -> *mut TSCallDescriptor {
                Box::into_raw(Box::new(TSCallDescriptor {}))
            }
        }

        pub type NewSloppyArgumentsElements = NewArgumentsElements<SloppyArgumentsElements>;
        pub type NewStrictArgumentsElements = NewArgumentsElements<StrictArgumentsElements>;
        pub type NewRestArgumentsElements = NewArgumentsElements<RestArgumentsElements>;

        pub struct SloppyArgumentsElements {}
        pub struct StrictArgumentsElements {}
        pub struct RestArgumentsElements {}

        pub struct NumberToString {}
        impl NumberToString {
            pub const FUNCTION: Builtin = Builtin::kNumberToString;
            pub type arguments_t = std::tuple<V<Number>>;
            pub type results_t = std::tuple<V<String>>;

            pub const NEEDS_FRAME_STATE: bool = false;
            pub const NEEDS_CONTEXT: bool = false;
            pub const PROPERTIES: i32 = Operator::kEliminatable;
            pub const EFFECTS: OpEffects = BuiltinCallDescriptor::BASE_EFFECTS
                .CanReadMemory()
                .CanAllocateWithoutIdentity();
            
            pub fn create(_call_mode: StubCallMode, _zone: *mut Zone, _lazy_deopt_on_throw: LazyDeoptOnThrow) -> *mut TSCallDescriptor {
                Box::into_raw(Box::new(TSCallDescriptor {}))
            }
        }

        pub struct ToString {}
        impl ToString {
            pub const FUNCTION: Builtin = Builtin::kToString;
            pub type arguments_t = std::tuple<V<Object>>;
            pub type results_t = std::tuple<V<String>>;

            pub const NEEDS_FRAME_STATE: bool = true;
            pub const NEEDS_CONTEXT: bool = true;
            pub const PROPERTIES: i32 = Operator::kNoProperties;
            pub const EFFECTS: OpEffects =
                BuiltinCallDescriptor::BASE_EFFECTS.CanCallAnything();
            
            pub fn create(_call_mode: StubCallMode, _zone: *mut Zone, _lazy_deopt_on_throw: LazyDeoptOnThrow) -> *mut TSCallDescriptor {
                Box::into_raw(Box::new(TSCallDescriptor {}))
            }
        }

        pub struct PlainPrimitiveToNumber {}
        impl PlainPrimitiveToNumber {
            pub const FUNCTION: Builtin = Builtin::kPlainPrimitiveToNumber;
            pub type arguments_t = std::tuple<V<PlainPrimitive>>;
            pub type results_t = std::tuple<V<Number>>;

            pub const NEEDS_FRAME_STATE: bool = false;
            pub const NEEDS_CONTEXT: bool = false;
            pub const PROPERTIES: i32 = Operator::kEliminatable;
            pub const EFFECTS: OpEffects = BuiltinCallDescriptor::BASE_EFFECTS
                .CanReadMemory()
                .CanAllocateWithoutIdentity();
            
            pub fn create(_call_mode: StubCallMode, _zone: *mut Zone, _lazy_deopt_on_throw: LazyDeoptOnThrow) -> *mut TSCallDescriptor {
                Box::into_raw(Box::new(TS