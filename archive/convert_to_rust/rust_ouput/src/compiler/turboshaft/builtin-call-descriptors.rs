// Converted from V8 C++ source files:
// Header: builtin-call-descriptors.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod builtins {
    pub enum Builtin {
        kCheckTurbofanType,
        kAdd,
        kSubtract,
        kMultiply,
        kDivide,
        kModulus,
        kBitwiseAnd,
        kBitwiseOr,
        kBitwiseXor,
        kShiftLeft,
        kShiftRight,
        kShiftRightLogical,
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
}
pub mod codegen {
    pub struct Callable {}
    pub struct InterfaceDescriptors {}
}
pub mod compiler {
    pub mod frame {
        pub struct Frame {}
    }
    pub mod globals {
        pub struct Globals {}
    }
    pub mod turboshaft {
        pub mod operations {
            pub struct OpEffects {
                can_depend_on_checks: bool,
                can_read_memory: bool,
                can_write_memory: bool,
                can_allocate: bool,
                can_call_anything: bool,
                can_read_heap_memory: bool,
                can_write_heap_memory: bool,
                can_allocate_without_identity: bool,
                can_change_control_flow: bool,
                assumes_consistent_heap: bool,
                required_when_unused: bool,
                can_leave_current_function: bool,
                no_properties: bool,
                no_deopt: bool,
                no_throw: bool,
                no_write: bool,
                eliminatable: bool,
            }

            impl OpEffects {
                pub fn new() -> Self {
                    OpEffects {
                        can_depend_on_checks: false,
                        can_read_memory: false,
                        can_write_memory: false,
                        can_allocate: false,
                        can_call_anything: false,
                        can_read_heap_memory: false,
                        can_write_heap_memory: false,
                        can_allocate_without_identity: false,
                        can_change_control_flow: false,
                        assumes_consistent_heap: false,
                        required_when_unused: false,
                        can_leave_current_function: false,
                        no_properties: false,
                        no_deopt: false,
                        no_throw: false,
                        no_write: false,
                        eliminatable: false,
                    }
                }

                pub fn CanDependOnChecks(mut self) -> Self {
                    self.can_depend_on_checks = true;
                    self
                }

                pub fn CanReadMemory(mut self) -> Self {
                    self.can_read_memory = true;
                    self
                }

                pub fn CanWriteMemory(mut self) -> Self {
                    self.can_write_memory = true;
                    self
                }

                pub fn CanAllocate(mut self) -> Self {
                    self.can_allocate = true;
                    self
                }

                pub fn CanCallAnything(mut self) -> Self {
                    self.can_call_anything = true;
                    self
                }

                pub fn CanReadHeapMemory(mut self) -> Self {
                    self.can_read_heap_memory = true;
                    self
                }

                pub fn CanWriteHeapMemory(mut self) -> Self {
                    self.can_write_heap_memory = true;
                    self
                }

                pub fn CanAllocateWithoutIdentity(mut self) -> Self {
                    self.can_allocate_without_identity = true;
                    self
                }

                pub fn CanChangeControlFlow(mut self) -> Self {
                    self.can_change_control_flow = true;
                    self
                }

                pub fn AssumesConsistentHeap(mut self) -> Self {
                    self.assumes_consistent_heap = true;
                    self
                }

                pub fn RequiredWhenUnused(mut self) -> Self {
                    self.required_when_unused = true;
                    self
                }

                pub fn CanLeaveCurrentFunction(mut self) -> Self {
                    self.can_leave_current_function = true;
                    self
                }
            }
        }
        pub mod representations {
            pub struct Representations {}
        }
    }
    pub mod write_barrier_kind {
        pub struct WriteBarrierKind {}
    }
}
pub mod objects {
    pub mod js_function {
        pub struct JSFunction {}
    }
}
pub mod builtins {
    pub enum Builtin {
        kCheckTurbofanType,
        kAdd,
        kSubtract,
        kMultiply,
        kDivide,
        kModulus,
        kBitwiseAnd,
        kBitwiseOr,
        kBitwiseXor,
        kShiftLeft,
        kShiftRight,
        kShiftRightLogical,
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
}
pub mod codegen {
    pub struct Callable {}
    pub struct InterfaceDescriptors {}
}
pub mod compiler {
    pub mod frame {
        pub struct Frame {}
    }
    pub mod globals {
        pub struct Globals {}
    }
    pub mod turboshaft {
        pub mod operations {
            pub struct OpEffects {
                can_depend_on_checks: bool,
                can_read_memory: bool,
                can_write_memory: bool,
                can_allocate: bool,
                can_call_anything: bool,
                can_read_heap_memory: bool,
                can_write_heap_memory: bool,
                can_allocate_without_identity: bool,
                can_change_control_flow: bool,
                assumes_consistent_heap: bool,
                required_when_unused: bool,
                can_leave_current_function: bool,
                no_properties: bool,
                no_deopt: bool,
                no_throw: bool,
                no_write: bool,
                eliminatable: bool,
            }

            impl OpEffects {
                pub fn new() -> Self {
                    OpEffects {
                        can_depend_on_checks: false,
                        can_read_memory: false,
                        can_write_memory: false,
                        can_allocate: false,
                        can_call_anything: false,
                        can_read_heap_memory: false,
                        can_write_heap_memory: false,
                        can_allocate_without_identity: false,
                        can_change_control_flow: false,
                        assumes_consistent_heap: false,
                        required_when_unused: false,
                        can_leave_current_function: false,
                        no_properties: false,
                        no_deopt: false,
                        no_throw: false,
                        no_write: false,
                        eliminatable: false,
                    }
                }

                pub fn CanDependOnChecks(mut self) -> Self {
                    self.can_depend_on_checks = true;
                    self
                }

                pub fn CanReadMemory(mut self) -> Self {
                    self.can_read_memory = true;
                    self
                }

                pub fn CanWriteMemory(mut self) -> Self {
                    self.can_write_memory = true;
                    self
                }

                pub fn CanAllocate(mut self) -> Self {
                    self.can_allocate = true;
                    self
                }

                pub fn CanCallAnything(mut self) -> Self {
                    self.can_call_anything = true;
                    self
                }

                pub fn CanReadHeapMemory(mut self) -> Self {
                    self.can_read_heap_memory = true;
                    self
                }

                pub fn CanWriteHeapMemory(mut self) -> Self {
                    self.can_write_heap_memory = true;
                    self
                }

                pub fn CanAllocateWithoutIdentity(mut self) -> Self {
                    self.can_allocate_without_identity = true;
                    self
                }

                pub fn CanChangeControlFlow(mut self) -> Self {
                    self.can_change_control_flow = true;
                    self
                }

                pub fn AssumesConsistentHeap(mut self) -> Self {
                    self.assumes_consistent_heap = true;
                    self
                }

                pub fn RequiredWhenUnused(mut self) -> Self {
                    self.required_when_unused = true;
                    self
                }

                pub fn CanLeaveCurrentFunction(mut self) -> Self {
                    self.can_leave_current_function = true;
                    self
                }
            }
        }
        pub mod representations {
            pub struct Representations {}
        }
    }
    pub mod write_barrier_kind {
        pub struct WriteBarrierKind {}
    }
}
pub mod objects {
    pub mod js_function {
        pub struct JSFunction {}
    }
}
use std::marker::PhantomData;
use std::mem::size_of;
pub struct BuiltinCallDescriptor {}
pub enum StubCallMode {
    kRegular,
    kWithSelf,
}
pub enum LazyDeoptOnThrow {
    kYes,
    kNo,
}
pub struct TSCallDescriptor {}
pub struct CallInterfaceDescriptor {}
pub struct CallDescriptor {}
pub struct Linkage {}
pub struct Zone {}
impl Linkage {
    pub fn GetStubCallDescriptor(
        zone: *mut Zone,
        interface_descriptor: CallInterfaceDescriptor,
        stack_parameter_count: i32,
        needs_frame_state: i32,
        properties: i32,
        call_mode: StubCallMode,
    ) -> *mut CallDescriptor {
        Box::into_raw(Box::new(CallDescriptor {}))
    }
}
impl TSCallDescriptor {
    pub fn Create(
        descriptor: *mut CallDescriptor,
        can_throw: CanThrow,
        lazy_deopt_on_throw: LazyDeoptOnThrow,
        zone: *mut Zone,
    ) -> *mut TSCallDescriptor {
        Box::into_raw(Box::new(TSCallDescriptor {}))
    }
}
pub enum CanThrow {
    kYes,
    kNo,
}
macro_rules! CHECK_EQ {
    ($left:expr, $right:expr) => {
        if $left != $right {
            panic!("CHECK_EQ failed: {} != {}", $left, $right);
        }
    };
}
macro_rules! DCHECK {
    ($condition:expr) => {
        if !$condition {
            panic!("DCHECK failed: {}", stringify!($condition));
        }
    };
}
pub struct Operator {}
impl Operator {
    pub const kNoThrow: i32 = 1;
    pub const kNoDeopt: i32 = 2;
    pub const kNoWrite: i32 = 4;
    pub const kEliminatable: i32 = 8;
    pub const kPure: i32 = 16;
    pub const kNoProperties: i32 = 0;
}
pub struct V<T> {
    _phantom: PhantomData<T>,
}
impl<T> V<T> {
    fn allows_representation(_rep: i32) -> bool {
        true
    }
}
pub struct Object {}
pub struct TurbofanType {}
pub struct Smi {}
pub struct Number {}
pub struct JSAnyNotNumber {}
pub struct Numeric {}
pub struct Float64 {}
pub struct WordPtr {}
pub struct Word32 {}
pub struct FixedArray {}
pub struct String {}
pub struct PlainPrimitive {}
pub struct Boolean {}
pub struct JSPrimitive {}
pub struct JSReceiver {}
pub struct ScopeInfo {}
pub struct Context {}
pub struct SharedFunctionInfo {}
pub struct FeedbackCell {}
pub struct JSFunction {}
pub struct Oddball {}
pub struct TurboshaftWord32Type {}
pub struct TurboshaftWord64Type {}
pub struct Float32 {}
pub struct TurboshaftFloat64Type {}
pub struct ByteArray {}
pub struct HeapNumber {}
pub struct WasmFuncRef {}
pub struct Symbol {}
pub struct WasmArray {}
pub struct WasmStringRefNullable {}
pub struct WasmStringViewIter {}
pub struct Map {}
pub struct HeapObject {}
pub struct BigInt {}
pub struct JSDataView {}
impl BuiltinCallDescriptor {
    const base_effects: operations::OpEffects = operations::OpEffects {
        can_depend_on_checks: true,
        can_read_memory: false,
        can_write_memory: false,
        can_allocate: false,
        can_call_anything: false,
        can_read_heap_memory: false,
        can_write_heap_memory: false,
        can_allocate_without_identity: false,
        can_change_control_flow: false,
        assumes_consistent_heap: false,
        required_when_unused: false,
        can_leave_current_function: false,
        no_properties: false,
        no_deopt: false,
        no_throw: false,
        no_write: false,
        eliminatable: false,
    };
    pub struct CheckTurbofanType {}
    impl CheckTurbofanType {
        pub const kFunction: builtins::Builtin = builtins::Builtin::kCheckTurbofanType;
        pub type arguments_t = std::tuple::<(V<Object>, V<TurbofanType>, V<Smi>)>;
        pub type results_t = std::tuple::<V<Object>>;
        pub const kNeedsFrameState: bool = false;
        pub const kNeedsContext: bool = true;
        pub const kProperties: i32 = Operator::kNoThrow | Operator::kNoDeopt;
        pub const kEffects: operations::OpEffects = {
            let mut effects = BuiltinCallDescriptor::base_effects;
            effects.can_read_memory = true;
            effects.required_when_unused = true;
            effects
        };
    }
    macro_rules! DECL_GENERIC_BINOP {
        ($Name:ident) => {
            pub struct $Name {}
            impl $Name {
                pub const kFunction: builtins::Builtin = builtins::Builtin::kAdd;
                pub type arguments_t = std::tuple::<(V<Object>, V<Object>)>;
                pub type results_t = std::tuple::<V<Object>>;
                pub const kNeedsFrameState: bool = true;
                pub const kNeedsContext: bool = true;
                pub const kProperties: i32 = Operator::kNoProperties;
                pub const kEffects: operations::OpEffects = {
                    let mut effects = BuiltinCallDescriptor::base_effects;
                    effects.can_call_anything = true;
                    effects
                };
            }
        };
    }
    macro_rules! GENERIC_BINOP_LIST {
        ($macro:ident) => {
            $macro!(Add);
            $macro!(Subtract);
            $macro!(Multiply);
            $macro!(Divide);
            $macro!(Modulus);
            $macro!(BitwiseAnd);
            $macro!(BitwiseOr);
            $macro!(BitwiseXor);
            $macro!(ShiftLeft);
            $macro!(ShiftRight);
            $macro!(ShiftRightLogical);
        };
    }
    GENERIC_BINOP_LIST!(DECL_GENERIC_BINOP);
    macro_rules! DECL_GENERIC_UNOP {
        ($Name:ident) => {
            pub struct $Name {}
            impl $Name {
                pub const kFunction: builtins::Builtin = builtins::Builtin::kAdd;
                pub type arguments_t = std::tuple::<V<Object>>;
                pub type results_t = std::tuple::<V<Object>>;
                pub const kNeedsFrameState: bool = true;
                pub const kNeedsContext: bool = true;
                pub const kProperties: i32 = Operator::kNoProperties;
                pub const kEffects: operations::OpEffects = {
                    let mut effects = BuiltinCallDescriptor::base_effects;
                    effects.can_call_anything = true;
                    effects
                };
            }
        };
    }
    macro_rules! GENERIC_UNOP_LIST {
        ($macro:ident) => {};
    }
    GENERIC_UNOP_LIST!(DECL_GENERIC_UNOP);
    pub struct ToNumber {}
    impl ToNumber {
        pub const kFunction: builtins::Builtin = builtins::Builtin::kToNumber;
        pub type arguments_t = std::tuple::<V<Object>>;
        pub type results_t = std::tuple::<V<Number>>;
        pub const kNeedsFrameState: bool = true;
        pub const kNeedsContext: bool = true;
        pub const kProperties: i32 = Operator::kNoProperties;
        pub const kEffects: operations::OpEffects = {
            let mut effects = BuiltinCallDescriptor::base_effects;
            effects.can_call_anything = true;
            effects
        };
    }
    pub struct NonNumberToNumber {}
    impl NonNumberToNumber {
        pub const kFunction: builtins::Builtin = builtins::Builtin::kNonNumberToNumber;
        pub type arguments_t = std::tuple::<V<JSAnyNotNumber>>;
        pub type results_t = std::tuple::<V<Number>>;
        pub const kNeedsFrameState: bool = false;
        pub const kNeedsContext: bool = true;
        pub const kProperties: i32 = Operator::kNoProperties;
        pub const kEffects: operations::OpEffects = {
            let mut effects = BuiltinCallDescriptor::base_effects;
            effects.can_call_anything = true;
            effects
        };
    }
    pub struct ToNumeric {}
    impl ToNumeric {
        pub const kFunction: builtins::Builtin = builtins::Builtin::kToNumeric;
        pub type arguments_t = std::tuple::<V<Object>>;
        pub type results_t = std::tuple::<V<Numeric>>;
        pub const kNeedsFrameState: bool = true;
        pub const kNeedsContext: bool = true;
        pub const kProperties: i32 = Operator::kNoProperties;
        pub const kEffects: operations::OpEffects = {
            let mut effects = BuiltinCallDescriptor::base_effects;
            effects.can_call_anything = true;
            effects
        };
    }
    pub struct NonNumberToNumeric {}
    impl NonNumberToNumeric {
        pub const kFunction: builtins::Builtin = builtins::Builtin::kNonNumberToNumeric;
        pub type arguments_t = std::tuple::<V<JSAnyNotNumber>>;
        pub type results_t = std::tuple::<V<Numeric>>;
        pub const kNeedsFrameState: bool = false;
        pub const kNeedsContext: bool = true;
        pub const kProperties: i32 = Operator::kNoProperties;
        pub const kEffects: operations::OpEffects = {
            let mut effects = BuiltinCallDescriptor::base_effects;
            effects.can_call_anything = true;
            effects
        };
    }
    pub struct CopyFastSmiOrObjectElements {}
    impl CopyFastSmiOrObjectElements {
        pub const kFunction: builtins::Builtin = builtins::Builtin::kCopyFastSmiOrObjectElements;
        pub type arguments_t = std::tuple::<V<Object>>;
        pub type results_t = std::tuple::<V<Object>>;
        pub const kNeedsFrameState: bool = false;
        pub const kNeedsContext: bool = false;
        pub const kProperties: i32 = Operator::kEliminatable;
        pub const kEffects: operations::OpEffects = {
            let mut effects = BuiltinCallDescriptor::base_effects;
            effects.can_write_memory = true;
            effects.can_read_memory = true;
            effects.can_allocate = true;
            effects
        };
    }
    pub struct DebugPrint<B, Input> {
        _builtin: PhantomData<B>,
        _input: PhantomData<Input>,
    }
    impl<B, Input> DebugPrint<B, Input> {
        pub const kFunction: builtins::Builtin = builtins::Builtin::kDebugPrintFloat64;
        pub type arguments_t = std::tuple::<V<Input>>;
        pub type results_t = std::tuple::<V<Object>>;
        pub const kNeedsFrameState: bool = false;
        pub const kNeedsContext: bool = true;
        pub const kProperties: i32 = Operator::kNoThrow | Operator::kNoDeopt;
        pub const kEffects: operations::OpEffects = {
            let mut effects = BuiltinCallDescriptor::base_effects;
            effects.required_when_unused = true;
            effects
        };
    }
    pub type DebugPrintFloat64 = DebugPrint<builtins::Builtin::kDebugPrintFloat64, Float64>;
    pub type DebugPrintWordPtr = DebugPrint<builtins::Builtin::kDebugPrintWordPtr, WordPtr>;
    pub struct FindOrderedHashEntry<B> {
        _builtin: PhantomData<B>,
    }
    impl<B> FindOrderedHashEntry<B> {
        pub const kFunction: builtins::Builtin = builtins::Builtin::kFindOrderedHashMapEntry;
        pub type arguments_t = std::tuple::<(V<Object>, V<Smi>)>;
        pub type results_t = std::tuple::<V<Smi>>;
        pub const kNeedsFrameState: bool = false;
        pub const kNeedsContext: bool = true;
        pub const kProperties: i32 = Operator::kEliminatable;
        pub const kEffects: operations::OpEffects = {
            let mut effects = BuiltinCallDescriptor::base_effects;
            effects.assumes_consistent_heap = true;
            effects.can_read_memory = true;
            effects.can_allocate = true;
            effects
        };
    }
    pub type FindOrderedHashMapEntry = FindOrderedHashEntry<builtins::Builtin::kFindOrderedHashMapEntry>;
    pub type FindOrderedHashSetEntry = FindOrderedHashEntry<builtins::Builtin::kFindOrderedHashSetEntry>;
    pub struct GrowFastElements<B> {
        _builtin: PhantomData<B>,
    }
    impl<B> GrowFastElements<B> {
        pub const kFunction: builtins::Builtin = builtins::Builtin::kGrowFastDoubleElements;
        pub type arguments_t = std::tuple::<(V<Object>, V<Smi>)>;
        pub type results_t = std::tuple::<V<Object>>;
        pub const kNeedsFrameState: bool = false;
        pub const kNeedsContext: bool = false;
        pub const kProperties: i32 = Operator::kEliminatable;
        pub const kEffects: operations::OpEffects = {
            let mut effects = BuiltinCallDescriptor::base_effects;
            effects.can_write_memory = true;
            effects.can_read_memory = true;
            effects.can_allocate = true;
            effects
        };
    }
    pub type GrowFastDoubleElements = GrowFastElements<builtins::Builtin::kGrowFastDoubleElements>;
    pub type GrowFastSmiOrObjectElements =
        GrowFastElements<builtins::Builtin::kGrowFastSmiOrObjectElements>;
    pub struct NewArgumentsElements<B> {
        _builtin: PhantomData<B>,
    }
    impl<B> NewArgumentsElements<B> {
        pub const kFunction: builtins::Builtin = builtins::Builtin::kNewSloppyArgumentsElements;
        pub type arguments_t = std::tuple::<(V<WordPtr>, V<WordPtr>, V<Smi>)>;
        pub type results_t = std::tuple::<V<FixedArray>>;
        pub const kNeedsFrameState: bool = false;
        pub const kNeedsContext: bool = false;
        pub const kProperties: i32 = Operator::kEliminatable;
        pub const kEffects: operations::OpEffects = {
            let mut effects = BuiltinCallDescriptor::base_effects;
            effects.can_allocate = true;
            effects
        };
    }
    pub type NewSloppyArgumentsElements =
        NewArgumentsElements<builtins::Builtin::kNewSloppyArgumentsElements>;
    pub type NewStrictArgumentsElements =
        NewArgumentsElements<builtins::Builtin::kNewStrictArgumentsElements>;
    pub type NewRestArgumentsElements =
        NewArgumentsElements<builtins::Builtin::kNewRestArgumentsElements>;
    pub struct NumberToString {}
    impl NumberToString {
        pub const kFunction: builtins::Builtin = builtins::Builtin::kNumberToString;
        pub type arguments_t = std::tuple::<V<Number>>;
        pub type results_t = std::tuple::<V<String>>;
        pub const kNeedsFrameState: bool = false;
        pub const kNeedsContext: bool = false;
        pub const kProperties: i32 = Operator::kEliminatable;
        pub const kEffects: operations::OpEffects = {
            let mut effects = BuiltinCallDescriptor::base_effects;
            effects.can_read_memory = true;
            effects.can_allocate_without_identity = true;
            effects
        };
    }
    pub struct ToString {}
    impl ToString {
        pub const kFunction: builtins::Builtin = builtins::Builtin::kToString;
        pub type arguments_t = std::tuple::<V<Object>>;
        pub type results_t = std::tuple::<V<String>>;
        pub const kNeedsFrameState: bool = true;
        pub const kNeedsContext: bool = true;
        pub const kProperties: i32 = Operator::kNoProperties;
        pub const kEffects: operations::OpEffects = {
            let mut effects = BuiltinCallDescriptor::base_effects;
            effects.can_call_anything = true;
            effects
        };
    }
    pub struct PlainPrimitiveToNumber {}
    impl PlainPrimitiveToNumber {
        pub const kFunction: builtins::Builtin = builtins::Builtin::kPlainPrimitiveToNumber;
        pub type arguments_t = std::tuple::<V<PlainPrimitive>>;
        pub type results_t = std::tuple::<V<Number>>;
        pub const kNeedsFrameState: bool = false;
        pub const kNeedsContext: bool = false;
        pub const kProperties: i32 = Operator::kEliminatable;
        pub const kEffects: operations::OpEffects = {
            let mut effects = BuiltinCallDescriptor::base_effects;
            effects.can_read_memory = true;
            effects.can_allocate_without_identity = true;
            effects
        };
    }
    pub struct SameValue {}
    impl SameValue {
        pub const kFunction: builtins::Builtin = builtins::Builtin::kSameValue;
        pub type arguments_t = std::tuple::<(V<Object>, V<Object>)>;
        pub type results_t = std::tuple::<V<Boolean>>;
        pub const kNeedsFrameState: bool = false;
        pub const kNeedsContext: bool = false;
        pub const kProperties: i32 = Operator::kEliminatable;
        pub const kEffects: operations::OpEffects = {
            let mut effects = BuiltinCallDescriptor::base_effects;
            effects.can_read_memory = true;
            effects.can_allocate = true;
            effects
        };
    }
    pub struct SameValueNumbersOnly {}
    impl SameValueNumbersOnly {
        pub const kFunction: builtins::Builtin = builtins::Builtin::kSameValueNumbersOnly;
        pub type
