// Converted from V8 C++ source files:
// Header: turbofan-typer.h
// Implementation: turbofan-typer.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::any::Any;
use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt;
use std::mem;
use std::ops::{BitAnd, BitOr, BitXor, Deref, DerefMut, Not};
use std::rc::Rc;
use std::string::String;
use std::sync::{Arc, Mutex, RwLock};

// mod compiler;

pub struct FlagScope {}
pub struct ObjectRef {}
pub struct MapRef {}
pub struct HeapObjectRef {}
pub struct SharedFunctionInfoRef {}
pub struct JSFunctionRef {}
pub struct FeedbackCellRef {}
pub struct OptionalSharedFunctionInfoRef {}
pub struct Heap {}

pub type NodeVector = Vec<*mut Node>;
pub type NodeId = usize; // Assuming NodeId is a usize for simplicity

pub struct JSHeapBroker {}
impl JSHeapBroker {
    pub fn IsJSFunction(&self, object: ObjectRef) -> bool {
        true
    }
}

pub trait Object {}
pub trait Value {}
pub type Local<'a, T> = &'a T;
pub struct JSCreateClosureNode<'a> {
    node: &'a Node,
}
impl<'a> JSCreateClosureNode<'a> {
    pub fn Parameters(&self) -> JSCreateClosureParameters {
        JSCreateClosureParameters {}
    }
}
pub struct JSCreateClosureParameters {}
impl JSCreateClosureParameters {
    pub fn shared_info(&self) -> SharedFunctionInfoRef {
        SharedFunctionInfoRef {}
    }
}
pub trait GraphDecorator {
    fn Decorate(&mut self, node: *mut Node);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct bitset {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Limits {
    lower: f64,
    upper: f64,
}
impl Limits {
    pub fn new(lower: f64, upper: f64) -> Self {
        Limits { lower, upper }
    }
    pub fn Is(self, other: Self) -> bool {
        self == other
    }
}
pub struct TypeCache {}
impl TypeCache {
    pub fn Get() -> &'static TypeCache {
        lazy_static::lazy_static! {
            static ref CACHE: TypeCache = TypeCache {};
        }
        &CACHE
    }

    pub fn kSingletonMinusOne(&self) -> Type {
        Type::Constant(-1.0, &Zone {})
    }
    pub fn kSingletonOne(&self) -> Type {
        Type::Constant(1.0, &Zone {})
    }
    pub fn kInteger(&self) -> Type {
        Type::Range(i64::MIN as f64, i64::MAX as f64, &Zone {})
    }
    pub fn kIntegerOrMinusZeroOrNaN(&self) -> Type {
        Type::Union(Type::Intersect(self.kInteger(), self.kInteger(), &Zone {}),
                    Type::Constant(0.0, &Zone {}), &Zone {})
    }
    pub fn kFixedArrayLengthType(&self) -> Type {
        Type::Range(0.0, FixedArray::kMaxLength as f64, &Zone {})
    }
    pub fn kZeroToThirtyTwo(&self) -> Type {
        Type::Range(0.0, 32.0, &Zone {})
    }
    pub fn kTimeValueType(&self) -> Type {
        Type::Number()
    }
    pub fn kJSDateDayType(&self) -> Type {
        Type::Range(0.0, 6.0, &Zone {})
    }
    pub fn kJSDateWeekdayType(&self) -> Type {
        Type::Range(0.0, 6.0, &Zone {})
    }
    pub fn kJSDateYearType(&self) -> Type {
        Type::Range(0.0, 9999.0, &Zone {})
    }
    pub fn kJSDateHourType(&self) -> Type {
        Type::Range(0.0, 23.0, &Zone {})
    }
    pub fn kJSDateMinuteType(&self) -> Type {
        Type::Range(0.0, 59.0, &Zone {})
    }
    pub fn kJSDateMonthType(&self) -> Type {
        Type::Range(0.0, 11.0, &Zone {})
    }
    pub fn kJSDateSecondType(&self) -> Type {
        Type::Range(0.0, 59.0, &Zone {})
    }
    pub fn kJSDateValueType(&self) -> Type {
        Type::Number()
    }
    pub fn kMinusOneToOneOrMinusZeroOrNaN(&self) -> Type {
        Type::Union(Type::Range(-1.0, 1.0, &Zone {}), Type::NaN(), &Zone {})
    }
    pub fn kUint16(&self) -> Type {
        Type::Range(0.0, u16::MAX as f64, &Zone {})
    }
    pub fn kPositiveSafeInteger(&self) -> Type {
        Type::Range(0.0, kMaxSafeInteger as f64, &Zone {})
    }
    pub fn kStringLengthType(&self) -> Type {
        Type::Range(0.0, String::kMaxLength as f64, &Zone {})
    }
    pub fn kUint32(&self) -> Type {
        Type::Range(0.0, u32::MAX as f64, &Zone {})
    }
    pub fn kSmi(&self) -> Type {
        Type::Range(i32::MIN as f64, i32::MAX as f64, &Zone {})
    }
}
pub struct DirectHandle<T> {
    value: T,
}

impl<T> Deref for DirectHandle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for DirectHandle<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T> DirectHandle<T> {
    pub fn new(value: T) -> Self {
        DirectHandle { value }
    }
}

pub struct Tagged<T> {
    value: T,
}

impl<T> Deref for Tagged<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Tagged<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T> Tagged<T> {
    pub fn new(value: T) -> Self {
        Tagged { value }
    }
}

pub struct TFGraph {}
impl TFGraph {
    pub fn AddDecorator(&mut self, decorator: *mut dyn GraphDecorator) {}
    pub fn RemoveDecorator(&mut self, decorator: *mut dyn GraphDecorator) {}
    pub fn zone(&self) -> &Zone {
        &Zone {}
    }
}
pub struct Zone {}
impl Zone {
    pub fn New<T>(&self, value: T) -> Box<T> {
        Box::new(value)
    }
}

pub type RootIndex = usize;

pub struct Isolate {}

pub mod base {
    pub struct Flags<T: Sized> {
        bits: u8,
        _phantom: std::marker::PhantomData<T>,
    }
    impl<T> Flags<T> {
        pub fn new(bits: u8) -> Self {
            Flags {
                bits,
                _phantom: std::marker::PhantomData,
            }
        }
    }
}

pub mod wasm {
    pub struct CanonicalSig {}
    impl CanonicalSig {
        pub fn return_count(&self) -> i32 {
            0
        }
        pub fn GetReturn(&self) -> i32 {
            0
        }
    }
}

pub mod linkage {
    pub const kJSCallClosureParamIndex: i32 = 0;
    pub const kOsrContextSpillSlotIndex: i32 = 0;
}

pub mod objects {
    pub struct JSFunction {}
    impl JSFunction {
        pub fn shared(&self) -> SharedFunctionInfoRef {
            SharedFunctionInfoRef {}
        }
    }
}

pub mod compiler {
    use super::*;
    use std::any::Any;
    use std::cell::RefCell;
    use std::collections::HashSet;
    use std::fmt;
    use std::mem;
    use std::ops::{BitAnd, BitOr, BitXor, Deref, DerefMut, Not};
    use std::rc::Rc;
    use std::string::String;
    use std::sync::{Arc, Mutex, RwLock};

    const kMaxSafeInteger: f64 = 9007199254740991.0;
    const V8_INFINITY: f64 = f64::INFINITY;

    // mod common_operator;
    // mod graph_reducer;
    // mod js_heap_broker;
    // mod js_operator;
    // mod linkage;
    // mod loop_variable_optimizer;
    // mod node_properties;
    // mod node;
    // mod opcodes;
    // mod operation_typer;
    // mod simplified_operator;
    // mod type_cache;
    // mod objects;

    pub struct TickCounter {}

    pub enum IrOpcode {
        kStart,
        kIfException,
        kLoop,
        kBranch,
        kIfTrue,
        kIfFalse,
        kIfSuccess,
        kSwitch,
        kIfValue,
        kIfDefault,
        kMerge,
        kDeoptimize,
        kDeoptimizeIf,
        kDeoptimizeUnless,
        kTrapIf,
        kTrapUnless,
        kAssert,
        kReturn,
        kTailCall,
        kTerminate,
        kThrow,
        kEnd,
        kLoopExit,
        kSelect,
        kPhi,
        kEffectPhi,
        kInductionVariablePhi,
        kEnterMachineGraph,
        kExitMachineGraph,
        kEnsureWritableFastElements,
        kMaybeGrowFastElements,
        kTransitionElementsKind,
        kTransitionElementsKindOrCheckMap,
        kCheckpoint,
        kBeginRegion,
        kFinishRegion,
        kFrameState,
        kStateValues,
        kTypedStateValues,
        kObjectId,
        kTypeArgumentsElementsState,
        kTypeArgumentsLengthState,
        kObjectState,
        kTypedObjectState,
        kCall,
        kFastApiCall,
        kJSWasmCall,
        kGetContinuationPreservedEmbedderData,
        kSetContinuationPreservedEmbedderData,
        kParameter,
        kOsrValue,
        kRetain,
        kInt32Constant,
        kInt64Constant,
        kTaggedIndexConstant,
        kRelocatableInt32Constant,
        kRelocatableInt64Constant,
        kFloat32Constant,
        kFloat64Constant,
        kNumberConstant,
        kHeapConstant,
        kCompressedHeapConstant,
        kTrustedHeapConstant,
        kExternalConstant,
        kPointerConstant,
        kProjection,
        kMapGuard,
        kTypeGuard,
        kDead,
        kDeadValue,
        kUnreachable,
        kPlug,
        kStaticAssert,
        kSLVerifierHint,
        kJSEqual,
        JSStrictEqual,
        JSLessThan,
        JSGreaterThan,
        JSLessThanOrEqual,
        JSGreaterThanOrEqual,
        JSBitwiseOr,
        JSBitwiseAnd,
        JSBitwiseXor,
        JSShiftLeft,
        JSShiftRight,
        JSShiftRightLogical,
        JSAdd,
        JSSubtract,
        JSMultiply,
        JSDivide,
        JSModulus,
        JSExponentiate,
        JSBitwiseNot,
        JSDecrement,
        JSIncrement,
        JSNegate,
        JSToLength,
        JSToName,
        JSToNumber,
        JSToNumberConvertBigInt,
        JSToBigInt,
        JSToBigIntConvertNumber,
        JSToNumeric,
        JSToObject,
        JSToString,
        JSTypeOf,
        JSToBoolean,
        JSCreate,
        JSCreateArguments,
        JSCreateArray,
        JSCreateArrayIterator,
        JSCreateAsyncFunctionObject,
        JSCreateCollectionIterator,
        JSCreateBoundFunction,
        JSCreateGeneratorObject,
        JSCreateClosure,
        JSCreateIterResultObject,
        JSCreateStringIterator,
        JSCreateKeyValueArray,
        JSCreateObject,
        JSCreateStringWrapper,
        JSCreatePromise,
        JSCreateTypedArray,
        JSCreateLiteralArray,
        JSCreateEmptyLiteralArray,
        JSCreateArrayFromIterable,
        JSCreateLiteralObject,
        JSCreateEmptyLiteralObject,
        JSCloneObject,
        JSCreateLiteralRegExp,
        JSGetTemplateObject,
        JSLoadProperty,
        JSLoadNamed,
        JSLoadNamedFromSuper,
        JSLoadGlobal,
        JSParseInt,
        JSRegExpTest,
        JSSetKeyedProperty,
        JSDefineKeyedOwnProperty,
        JSSetNamedProperty,
        JSStoreGlobal,
        JSDefineNamedOwnProperty,
        JSDefineKeyedOwnPropertyInLiteral,
        JSStoreInArrayLiteral,
        JSDeleteProperty,
        JSHasProperty,
        JSHasInPrototypeChain,
        JSInstanceOf,
        JSOrdinaryHasInstance,
        JSGetSuperConstructor,
        JSFindNonDefaultConstructorOrConstruct,
        JSHasContextExtension,
        JSLoadContext,
        JSLoadScriptContext,
        JSStoreContext,
        JSStoreScriptContext,
        JSCreateFunctionContext,
        JSCreateCatchContext,
        JSCreateWithContext,
        JSCreateBlockContext,
        JSConstructForwardVarargs,
        JSConstructForwardAllArgs,
        JSConstruct,
        JSConstructWithArrayLike,
        JSConstructWithSpread,
        JSObjectIsArray,
        DateNow,
        DoubleArrayMin,
        DoubleArrayMax,
        Unsigned32Divide,
        JSCallForwardVarargs,
        JSCall,
        JSCallWithArrayLike,
        JSCallWithSpread,
        JSCallRuntime,
        JSForInEnumerate,
        JSForInNext,
        JSForInPrepare,
        JSLoadMessage,
        JSStoreMessage,
        JSLoadModule,
        JSStoreModule,
        JSGetImportMeta,
        JSGeneratorStore,
        JSGeneratorRestoreContinuation,
        JSGeneratorRestoreContext,
        JSGeneratorRestoreRegister,
        JSGeneratorRestoreInputOrDebugPos,
        JSStackCheck,
        JSDebugger,
        JSAsyncFunctionEnter,
        JSAsyncFunctionReject,
        JSAsyncFunctionResolve,
        JSFulfillPromise,
        JSPerformPromiseThen,
        JSPromiseResolve,
        JSRejectPromise,
        JSResolvePromise,
        BooleanNot,
        NumberEqual,
        NumberLessThan,
        NumberLessThanOrEqual,
        SpeculativeNumberEqual,
        SpeculativeNumberLessThan,
        SpeculativeNumberLessThanOrEqual,
        BigIntEqual,
        BigIntLessThan,
        BigIntLessThanOrEqual,
        SpeculativeBigIntEqual,
        SpeculativeBigIntLessThan,
        SpeculativeBigIntLessThanOrEqual,
        StringConcat,
        StringToNumber,
        PlainPrimitiveToNumber,
        PlainPrimitiveToWord32,
        PlainPrimitiveToFloat64,
        ReferenceEqual,
        SameValue,
        SameValueNumbersOnly,
        NumberSameValue,
        StringEqual,
        StringLessThan,
        StringLessThanOrEqual,
        StringFromSingleCharCode,
        StringFromSingleCodePoint,
        StringToLowerCaseIntl,
        StringToUpperCaseIntl,
        StringCharCodeAt,
        StringCodePointAt,
        StringFromCodePointAt,
        StringIndexOf,
        StringLength,
        StringWrapperLength,
        StringSubstring,
        CheckBounds,
        CheckHeapObject,
        CheckIf,
        CheckInternalizedString,
        CheckMaps,
        CompareMaps,
        CheckNumber,
        CheckNumberFitsInt32,
        CheckReceiver,
        CheckReceiverOrNullOrUndefined,
        CheckSmi,
        CheckString,
        CheckStringOrStringWrapper,
        CheckSymbol,
        CheckFloat64Hole,
        ChangeFloat64HoleToTagged,
        CheckNotTaggedHole,
        CheckClosure,
        ConvertReceiver,
        ConvertTaggedHoleToUndefined,
        CheckEqualsInternalizedString,
        CheckEqualsSymbol,
        Allocate,
        AllocateRaw,
        LoadFieldByIndex,
        LoadField,
        LoadElement,
        LoadStackArgument,
        LoadFromObject,
        LoadImmutableFromObject,
        LoadTypedElement,
        LoadDataViewElement,
        StoreField,
        StoreElement,
        StoreToObject,
        InitializeImmutableInObject,
        TransitionAndStoreElement,
        TransitionAndStoreNumberElement,
        TransitionAndStoreNonNumberElement,
        StoreSignedSmallElement,
        StoreTypedElement,
        StoreDataViewElement,
        ObjectIsArrayBufferView,
        ObjectIsBigInt,
        ObjectIsCallable,
        ObjectIsConstructor,
        ObjectIsDetectableCallable,
        ObjectIsMinusZero,
        NumberIsMinusZero,
        NumberIsFloat64Hole,
        NumberIsFinite,
        ObjectIsFiniteNumber,
        NumberIsInteger,
        ObjectIsSafeInteger,
        NumberIsSafeInteger,
        ObjectIsInteger,
        ObjectIsNaN,
        NumberIsNaN,
        ObjectIsNonCallable,
        ObjectIsNumber,
        ObjectIsReceiver,
        ObjectIsSmi,
        ObjectIsString,
        ObjectIsSymbol,
        ObjectIsUndetectable,
        TypeArgumentsLength,
        RestLength,
        TypedArrayLength,
        NewDoubleElements,
        NewSmiOrObjectElements,
        NewArgumentsElements,
        NewConsString,
        FindOrderedHashMapEntry,
        FindOrderedHashMapEntryForInt32Key,
        FindOrderedHashSetEntry,
        RuntimeAbort,
        AssertType,
        VerifyType,
        CheckTurboshaftTypeOf,
        JSGetIterator,
        LoadTrapOnNull,
        StoreTrapOnNull,
        MemoryBarrier,
        SignExtendWord8ToInt32,
        SignExtendWord16ToInt32,
        SignExtendWord8ToInt64,
        SignExtendWord16ToInt64,
        SignExtendWord32ToInt64,
        StackPointerGreaterThan,
        TraceInstruction,
        UnalignedLoad,
        UnalignedStore,
        Int32PairAdd,
        Int32PairSub,
        Int32PairMul,
        Word32PairShl,
        Word32PairShr,
        Word32PairSar,
        ProtectedLoad,
        ProtectedStore,
        LoadImmutable,
        Store,
        StorePair,
        StoreIndirectPointer,
        StackSlot,
        Word32Popcnt,
        Word64Popcnt,
        Word64Clz,
        Word64Ctz,
        Word64ClzLowerable,
        Word64CtzLowerable,
        Word64ReverseBits,
        Word64ReverseBytes,
        Simd128ReverseBytes,
        Int64AbsWithOverflow,
        BitcastTaggedToWord,
        BitcastTaggedToWordForTagAndSmiBits,
        BitcastWordToTagged,
        BitcastWordToTaggedSigned,
        TruncateFloat64ToWord32,
        ChangeFloat32ToFloat64,
        ChangeFloat64ToInt32,
        ChangeFloat64ToInt64,
        ChangeFloat64ToUint32,
        ChangeFloat64ToUint64,
        Float64SilenceNaN,
        TruncateFloat64ToInt64,
        TruncateFloat64ToUint32,
        TruncateFloat32ToInt32,
        TruncateFloat32ToUint32,
        TryTruncateFloat32ToInt64,
        TryTruncateFloat64ToInt64,
        TryTruncateFloat32ToUint64,
        TryTruncateFloat64ToUint64,
        TryTruncateFloat64ToInt32,
        TryTruncateFloat64ToUint32,
        ChangeInt32ToFloat64,
        BitcastWord32ToWord64,
        ChangeInt32ToInt64,
        ChangeInt64ToFloat64,
        ChangeUint32ToFloat64,
        ChangeFloat16RawBitsToFloat64,
        TruncateFloat64ToFloat32,
        TruncateFloat64ToFloat16RawBits,
        TruncateInt64ToInt32,
        RoundFloat64ToInt32,
        RoundInt32ToFloat32,
        RoundInt64ToFloat32,
        RoundInt64ToFloat64,
        RoundUint32ToFloat32,
        RoundUint64ToFloat32,
        RoundUint64ToFloat64,
        BitcastFloat32ToInt32,
        BitcastFloat64ToInt64,
        BitcastInt32ToFloat32,
        BitcastInt64ToFloat64,
        Float64ExtractLowWord32,
        Float64ExtractHighWord32,
        Float64InsertLowWord32,
        Float64InsertHighWord32,
        Word32Select,
        Word64Select,
        Float32Select,
        Float64Select,
        LoadStackCheckOffset,
        LoadFramePointer,
        LoadStackPointer,
        SetStackPointer,
        LoadParentFramePointer,
        LoadRootRegister,
        Word32Or,
        Word32Xor,
        Word32Sar,
        Word32Rol,
        Word32Ror,
        Int32AddWithOverflow,
        Int32SubWithOverflow,
        Int32Mul,
        Int32MulWithOverflow,
        Int32MulHigh,
        Int32Div,
        Int32Mod,
        Uint32Mod,
        Uint32MulHigh,
        Word64Or,
        Word64Xor,
        Word64Sar,
        Word64Rol,
        Word64Ror,
        Word64RolLowerable,
        Word64RorLowerable,
        Int64AddWithOverflow,
        Int64SubWithOverflow,
        Int64Mul,
        Int64MulHigh,
        Int64MulWithOverflow,
        Int64Div,
        Int64Mod,
        Uint64Mod,
        Uint64MulHigh,
        Word64Equal,
        Int32LessThan,
        Int64LessThan,
        Int64LessThanOrEqual,
        Float32Equal,
        Float32LessThan,
        Float32LessThanOrEqual,
        Float64Equal,
        Float64LessThan,
        Float64LessThanOrEqual,
        Float32Add,
        Float32Sub,
        Float32Mul,
        Float32Div,
        Float32Abs,
        Float32Neg,
        Float32Sqrt,
        Float64Add,
        Float64Sub,
        Float64Mul,
        Float64Div,
        Float64Abs,
        Float64Neg,
        Float64Sqrt,
        AtomicLoadInt8,
        AtomicLoadUint8,
        AtomicLoadInt16,
        AtomicLoadUint16,
        AtomicLoadInt32,
        AtomicLoadUint32,
        AtomicLoadInt64,
        AtomicLoadUint64,
        AtomicStoreInt8,
        AtomicStoreUint8,
        AtomicStoreInt16,
        AtomicStoreUint16,
        AtomicStoreInt32,
        AtomicStoreUint32,
        AtomicStoreInt64,
        AtomicStoreUint64,
        AtomicAddInt8,
        AtomicAddUint8,
        AtomicAddInt16,
        AtomicAddUint16,
        AtomicAddInt32,
        AtomicAddUint32,
        AtomicAddInt64,
        AtomicAddUint64,
        AtomicSubInt8,
        AtomicSubUint8,
        AtomicSubInt16,
        AtomicSubUint16,
        AtomicSubInt32,
        AtomicSubUint32,
        AtomicSubInt64,
        AtomicSubUint64,
        AtomicExchangeInt8,
        AtomicExchangeUint8,
        AtomicExchangeInt16,
        AtomicExchangeUint16,
        AtomicExchangeInt32,
        AtomicExchangeUint32,
        AtomicExchangeInt64,
        AtomicExchangeUint64,
        AtomicCompareExchangeInt8,
        AtomicCompareExchangeUint8,
        AtomicCompareExchangeInt16,
        AtomicCompareExchangeUint16,
        AtomicCompareExchangeInt32,
        AtomicCompareExchangeUint32,
        AtomicCompareExchangeInt64,
        AtomicCompareExchangeUint64,
        AbortCSADcheck,
        DebugBreak,
        Comment,
        JSAsyncFunctionAwaitCaught,
        JSAsyncFunctionAwaitUncaught,
        JSAsyncFunctionComplete,
    }
    impl IrOpcode {
        pub fn ValueOutputCount(&self) -> i32 {
            match self {
                IrOpcode::kUnreachable
                | IrOpcode::kUnreachable => 0,
                _ => 1,
            }
        }
    }

    pub struct OperationTyper<'a> {
        broker_: &'a JSHeapBroker,
        zone_: &'a Zone,
    }
    impl<'a> OperationTyper<'a> {
        pub fn new(broker: &'a JSHeapBroker, zone: &'a Zone) -> Self {
            OperationTyper {
                broker_: broker,
                zone_: zone,
            }
        }
        pub fn singleton_false(&self) -> Type {
            Type::Constant(false, self.zone_)
        }
        pub fn singleton_true(&self) -> Type {
            Type::Constant(true, self.zone_)
        }
        pub fn Name(&self, _type: Type) -> Type {
            Type::None()
        }
        pub fn Name(&self, _lhs: Type, _rhs: Type) -> Type {
            Type::None()
        }
        pub fn ToBoolean(&self, _type: Type) -> Type {
            Type::Boolean()
        }
        pub fn SpeculativeToNumber(&self, _type: Type) -> Type {
            Type::Number()
        }
        pub fn ToNumber(&self, _type: Type) -> Type {
            Type::Number()
        }
        pub fn ToNumberConvertBigInt(&self, _type: Type) -> Type {
            Type::Number()
        }
        pub fn ToBigInt(&self, _type: Type) -> Type {
            Type::BigInt()
        }
        pub fn ToBigIntConvertNumber(&self, _type: Type) -> Type {
            Type::BigInt()
        }
        pub fn ToNumeric(&self, _type: Type) -> Type {
            Type::Numeric()
        }
        pub fn StrictEqual(&self, _lhs: Type, _rhs: Type) -> Type {
            Type::Boolean()
        }
        pub fn TypeTypeGuard(&self, _op: &dyn Any, _type: Type) -> Type {
            Type::Boolean()
        }
        pub fn CheckBounds(&self, _arg0: Type, _arg1: Type) -> Type {
            Type::Boolean()
        }
        pub fn CheckNumber(&self, _operand: Type) -> Type {
            Type::Boolean()
        }
        pub fn CheckNumberFitsInt32(&self, _operand: Type) -> Type {
            Type::Boolean()
        }
        pub fn CheckFloat64Hole(&self, _operand: Type) -> Type {
            Type::Boolean()
        }
        pub fn ConvertReceiver(&self, _arg: Type) -> Type {
            Type::Receiver()
        }
        pub fn ConvertTaggedHoleToUndefined(&self, _type: Type) -> Type {
            Type::Receiver()
        }
        pub fn SameValue(&self, _lhs: Type, _rhs: Type) -> Type {
            Type::Boolean()
        }
        pub fn SameValueNumbersOnly(&self, _lhs: Type, _rhs: Type) -> Type {
            Type::Boolean()
        }
    }

    pub struct Typer {
        flags_: Flags,
        graph_: *mut TFGraph,
        decorator_: *mut Decorator,
        cache_: &'static TypeCache,
        broker_: *mut JSHeapBroker,
        operation_typer_: OperationTyper<'static>,
        tick_counter_: *mut TickCounter,

        singleton_false_: Type,
        singleton_true_: Type,
    }

    impl Typer {
        pub fn new(
            broker: *mut JSHeapBroker,
            flags: Flags,
            graph: *mut TFGraph,
            tick_counter: *mut TickCounter,
        ) -> Typer {
            unsafe {
                let zone = (*graph).zone();
                let mut operation_typer = OperationTyper::new(&*(broker as *const JSHeapBroker), zone);

                let singleton_false_ = operation_typer.singleton_false();
                let singleton_true_ = operation_typer.singleton_true();

                let mut typer = Typer {
                    flags_: flags,
                    graph_: graph,
                    decorator_: std::ptr::null_mut(),
                    cache_: TypeCache::Get(),
                    broker_: broker,
                    operation_typer_: operation_typer,
                    tick_counter_: tick_counter,

                    singleton_false_: singleton_false_,
                    singleton_true_: singleton_true_,
                };

                let decorator = zone.New(Decorator::new(&mut typer));
                typer.decorator_ = Box::into_raw(decorator);
                (*graph).AddDecorator(typer.decorator_ as *mut dyn GraphDecorator);

                typer
            }
        }

        pub fn flags(&self) -> Flags {
            self.flags_
        }
        pub fn graph(&self) -> *mut TFGraph {
            self.graph_
        }
        pub fn zone(&self) -> &Zone {
            unsafe { (*self.graph_).zone() }
        }
        pub fn operation_typer(&mut self) -> &mut OperationTyper<'static> {
            &mut self.operation_typer_
        }
        pub fn broker(&self) -> *mut JSHeapBroker {
            self.broker_
        }

        pub fn run(&mut self) {
            self.run_with_roots(NodeVector::new(), std::ptr::null_mut());
        }

        pub fn run_with_roots(&mut self, roots: NodeVector, induction_vars: *mut LoopVariableOptimizer) {
            unsafe {
                if !induction_vars.is_null() {
                    (*induction_vars).ChangeToInductionVariablePhis();
                }

                let mut visitor = Visitor::new(self, induction_vars);
                let mut graph_reducer = GraphReducer::new(
                    self.zone(),
                    self.graph_,
                    self.tick_counter_,
                    self.broker_,
                );

                graph_reducer.AddReducer(&mut visitor);

                for &root in &roots {
                    graph_reducer.ReduceNode(root);
                }

                graph_reducer.ReduceGraph();

                if !induction_vars.is_null() {
                    // Validate the types computed by TypeInductionVariablePhi.
                    for entry in (*induction_vars).induction_variables().iter() {
                        let induction_var = entry.1;
                        if (*induction_var).phi().opcode() == IrOpcode::kInductionVariablePhi {
                            assert!(visitor.InductionVariablePhiTypeIsPrefixedPoint(induction_var));
                        }
                    }

                    (*induction_vars).ChangeToPhisAndInsertGuards();
                }
            }
        }
    }

    impl Drop for Typer {
        fn drop(&mut self) {
            unsafe {
                (*self.graph_).RemoveDecorator(self.decorator_ as *mut dyn GraphDecorator);
                let _ = Box::from_raw(self.decorator_);
            }
        }
    }

    #[derive(Clone, Copy)]
    pub enum Flag {
        kNoFlags = 0,
        kThisIsReceiver = 1 << 0,
        kNewTargetIsReceiver = 1 << 1,
    }

    impl Flag {
        pub fn to_u8(self) -> u8 {
            self as u8
        }
    }

    pub struct Flags(u8);
    impl Flags {
        pub fn contains(&self, flag: Flag) -> bool {
            self.0 & (flag.to_u8()) != 0
        }
        pub fn new() -> Self {
            Flags(0)
        }
    }
    impl std::ops::BitOr<Flag> for Flags {
        type Output = Self;
        fn bitor(self, rhs: Flag) -> Self::Output {
            Flags(self.0 | rhs.to_u8())
        }
    }

    struct Decorator {
        typer_: *mut Typer,
    }

    impl Decorator {
        fn new(typer: *mut Typer) -> Self {
            Decorator { typer_: typer }
        }
    }

    impl GraphDecorator for Decorator {
        fn Decorate(&mut self, node: *mut Node) {
            unsafe {
                if (*node).op.ValueOutputCount() > 0 {
                    // Only eagerly type-decorate nodes with known input types.
                    // Other cases will generally require a proper fixpoint iteration with Run.
                    let is_typed = NodeProperties::IsTyped(node);
                    if is_typed || NodeProperties::AllValueInputsAreTyped(node
