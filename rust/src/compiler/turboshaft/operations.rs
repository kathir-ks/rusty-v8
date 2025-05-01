// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unit_arg)]

use std::any::Any;
use std::cmp::min;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem;
use std::num::Wrapping;
use std::ops::{BitAnd, BitOr};
use std::sync::Mutex;

use lazy_static::lazy_static;

// Placeholder for src/base/logging.h
macro_rules! CHECK {
    ($cond:expr) => {
        if !$cond {
            panic!("Check failed: {}", stringify!($cond));
        }
    };
}

macro_rules! CHECK_EQ {
    ($left:expr, $right:expr) => {
        if $left != $right {
            panic!("Check failed: {} == {}. Left: {:?}. Right: {:?}", stringify!($left), stringify!($right), $left, $right);
        }
    };
}

macro_rules! CHECK_NE {
    ($left:expr, $right:expr) => {
        if $left == $right {
            panic!("Check failed: {} != {}. Left: {:?}. Right: {:?}", stringify!($left), stringify!($right), $left, $right);
        }
    };
}

macro_rules! DCHECK {
    ($cond:expr) => {
        if cfg!(debug_assertions) && !$cond {
            panic!("DCheck failed: {}", stringify!($cond));
        }
    };
}

macro_rules! DCHECK_EQ {
    ($left:expr, $right:expr) => {
        if cfg!(debug_assertions) && $left != $right {
            panic!("DCheck failed: {} == {}. Left: {:?}. Right: {:?}", stringify!($left), stringify!($right), $left, $right);
        }
    };
}

macro_rules! DCHECK_NE {
    ($left:expr, $right:expr) => {
        if cfg!(debug_assertions) && $left == $right {
            panic!("DCheck failed: {} != {}. Left: {:?}. Right: {:?}", stringify!($left), stringify!($right), $left, $right);
        }
    };
}

macro_rules! DCHECK_IMPLIES {
    ($condition:expr, $implication:expr) => {
        if cfg!(debug_assertions) && $condition {
            DCHECK!($implication);
        }
    };
}

macro_rules! UNREACHABLE {
    () => {
        panic!("Unreachable code reached");
    };
}

macro_rules! arraysize {
    ($arr:expr) => {
        ($arr).len()
    };
}

// Placeholder for src/base/macros.h
macro_rules! USE {
    ($x:expr) => {
        let _ = $x;
    };
}

// Placeholder for src/common/globals.h
const V8_COMPRESS_POINTERS: bool = false;
const Is64Bit: bool = cfg!(target_pointer_width = "64");

// Placeholder for src/flags/flags.h
const DEBUG_BOOL: bool = true;

// Placeholder for IF_WASM macro. Since wasm is always enabled, no need for the macro.
macro_rules! IF_WASM {
    ($($tokens:tt)*) => {
        $($tokens)*
    }
}

mod base {
    pub mod small_vector;
    pub mod template_utils;
    pub mod vector;
    pub mod platform {
        pub mod mutex {
            use std::sync::Mutex;
            pub struct MutexGuard<'a> {
                mutex: &'a Mutex<()>,
            }

            impl<'a> MutexGuard<'a> {
                pub fn new(mutex: &'a Mutex<()>) -> Self {
                    mutex.lock().unwrap();
                    MutexGuard { mutex }
                }
                pub fn Pointer(&self) -> &'a Mutex<()> {
                    self.mutex
                }
            }
        }
    }
}
mod codegen {
    pub mod external_reference;
}
mod common {
    pub mod globals;
}
mod compiler {
    pub mod common_operator;
    pub mod fast_api_calls;
    pub mod globals;
    pub mod simplified_operator;
    pub mod turboshaft {
        pub mod deopt_data;
        pub mod fast_hash;
        pub mod index;
        pub mod representations;
        pub mod snapshot_table;
        pub mod types;
        pub mod utils;
        pub mod zone_with_name;
    }
    pub mod write_barrier_kind;
}

#[cfg(V8_ENABLE_WEBASSEMBLY)]
mod wasm {
    pub mod wasm_module;
    pub mod wasm_objects;
}

pub mod internal {
    use std::fmt;

    pub struct HeapObject; // Placeholder

    #[derive(Debug)]
    pub enum AbortReason {
        // Placeholder for abort reasons
        GenericAbort,
    }

    impl fmt::Display for AbortReason {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    pub mod compiler {
        pub struct CallDescriptor; // Placeholder
        pub struct JSWasmCallParameters; // Placeholder
        pub struct DeoptimizeParameters; // Placeholder
        pub struct FrameStateInfo; // Placeholder
        pub struct Node; // Placeholder

        #[derive(Debug)]
        pub enum TrapId {
            // Placeholder for TrapId
            GenericTrap,
        }
    }
}

pub mod compiler {
    pub mod turboshaft {
        use crate::{
            base::{
                platform::mutex::{MutexGuard},
                small_vector::SmallVector,
                template_utils::any_of,
                vector::{Vector, VectorOf},
            },
            codegen::external_reference::ExternalReference,
            compiler::{
                simplified_operator::CheckForMinusZeroMode,
                turboshaft::{
                    deopt_data::LazyDeoptOnThrow,
                    index::{OpIndex, OptionalOpIndex, ShadowyOpIndexVectorWrapper},
                    representations::{
                        FloatRepresentation, MaybeRegisterRepresentation, RegisterRepresentation,
                        UntaggedRepresentation, WordRepresentation,
                    },
                    snapshot_table::{SnapshotTable},
                    types::{Object},
                    zone_with_name::{Zone},
                },
                write_barrier_kind::FeedbackSource,
            },
            internal::{
                compiler::FrameStateInfo,
                HeapObject,
            },
            Is64Bit, DEBUG_BOOL,
            lazy_static::lazy_static,
            std::{
                any::Any,
                cmp::min,
                fmt,
                fmt::Debug,
                hash::{Hash, Hasher},
                mem,
                num::Wrapping,
                ops::{BitAnd, BitOr},
                sync::Mutex,
            },
            CHECK, CHECK_EQ, arraysize, DCHECK, DCHECK_EQ, DCHECK_IMPLIES, IF_WASM, USE,
            UNREACHABLE,
        };

        //#[cfg(V8_ENABLE_WEBASSEMBLY)]
        //use crate::wasm::{wasm_module::WasmModule, wasm_objects::WasmObjects};

        pub const K_COMPILATION_ZONE_NAME: &str = "compilation-zone";

        pub struct Block; // Placeholder
        pub struct FrameStateData; // Placeholder
        pub struct Graph; // Placeholder
        pub struct FrameStateOp; // Placeholder

        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub enum HashingStrategy {
            kDefault,
            kMakeSnapshotStable,
        }

        #[derive(Debug, Clone)]
        pub struct VariableData {
            pub rep: MaybeRegisterRepresentation,
            pub loop_invariant: bool,
            pub active_loop_variables_index: Index, //IntrusiveSetIndex,
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub struct Index {} // Placeholder for IntrusiveSetIndex

        pub type Variable = SnapshotTable<OpIndex, VariableData>::Key;

        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub enum Opcode {
            kCheckException,
            kGoto,
            kTailCall,
            kUnreachable,
            kReturn,
            kBranch,
            kSwitch,
            kDeoptimize,
            kWasmStackCheck,
            kGlobalGet,
            kGlobalSet,
            kRootConstant,
            kIsRootConstant,
            kNull,
            kIsNull,
            kAssertNotNull,
            kRttCanon,
            kWasmTypeCheck,
            kWasmTypeCast,
            kAnyConvertExtern,
            kExternConvertAny,
            kWasmTypeAnnotation,
            kStructGet,
            kStructSet,
            kArrayGet,
            kArraySet,
            kArrayLength,
            kWasmAllocateArray,
            kWasmAllocateStruct,
            kWasmRefFunc,
            kStringAsWtf16,
            kStringPrepareForGetCodeUnit,
            kSimd128Constant,
            kSimd128Binop,
            kSimd128Unary,
            kSimd128Reduce,
            kSimd128Shift,
            kSimd128Test,
            kSimd128Splat,
            kSimd128Ternary,
            kSimd128ExtractLane,
            kSimd128ReplaceLane,
            kSimd128LaneMemory,
            kSimd128LoadTransform,
            kSimd128Shuffle,
            kArgumentsLength,
            kBigIntBinop,
            kBigIntComparison,
            kBigIntUnary,
            kCheckedClosure,
            kWordBinopDeoptOnOverflow,
            kCheckEqualsInternalizedString,
            kCheckMaps,
            kCompareMaps,
            kFloat64Is,
            kObjectIs,
            kObjectIsNumericValue,
            kFloat64SameValue,
            kSameValue,
            kChangeOrDeopt,
            kConvert,
            kConvertJSPrimitiveToObject,
            kConvertJSPrimitiveToUntagged,
            kConvertJSPrimitiveToUntaggedOrDeopt,
            kConvertUntaggedToJSPrimitive,
            kConvertUntaggedToJSPrimitiveOrDeopt,
            kTruncateJSPrimitiveToUntagged,
            kTruncateJSPrimitiveToUntaggedOrDeopt,
            kDoubleArrayMinMax,
            kEnsureWritableFastElements,
            kFastApiCall,
            kFindOrderedHashEntry,
            kLoadDataViewElement,
            kLoadFieldByIndex,
            kLoadMessage,
            kLoadStackArgument,
            kLoadTypedElement,
            kStoreDataViewElement,
            kStoreMessage,
            kStoreTypedElement,
            kMaybeGrowFastElements,
            kNewArgumentsElements,
            kNewArray,
            kRuntimeAbort,
            kStaticAssert,
            kStringAt,
            kStringComparison,
            kStringConcat,
            kStringFromCodePointAt,
            kStringIndexOf,
            kStringLength,
            kTypedArrayLength,
            kStringSubstring,
            kNewConsString,
            kTransitionAndStoreArrayElement,
            kTransitionElementsKind,
            kTransitionElementsKindOrCheckMap,
            kDebugPrint,
            kCheckTurboshaftTypeOf,
            kWord32SignHint,
            kWordBinop,
            kFloatBinop,
            kWord32PairBinop,
            kOverflowCheckedBinop,
            kWordUnary,
            kOverflowCheckedUnary,
            kFloatUnary,
            kShift,
            kComparison,
            kChange,
            kTryChange,
            kBitcastWord32PairToFloat64,
            kTaggedBitcast,
            kSelect,
            kPendingLoopPhi,
            kConstant,
            kLoadRootRegister,
            kLoad,
            kStore,
            kRetain,
            kParameter,
            kOsrValue,
            kStackPointerGreaterThan,
            kStackSlot,
            kFrameConstant,
            kDeoptimizeIf,
            kTrapIf,
            kLoadStackPointer,
            kSetStackPointer,
            kPhi,
            kFrameState,
            kCall,
            kCatchBlockBegin,
            kDidntThrow,
            kTuple,
            kProjection,
            kDebugBreak,
            kAssumeMap,
            kAtomicRMW,
            kAtomicWord32Pair,
            kMemoryBarrier,
            kComment,
            kDead,
            kAbortCSADcheck,
            kAllocate,
            kDecodeExternalPointer,
            kJSStackCheck,
            kGenericBinop,
            kGenericUnop,
            kToNumberOrNumeric,
            kStringToCaseIntl, // Added for V8_INTL_SUPPORT
            kGetContinuationPreservedEmbedderData,
            kSetContinuationPreservedEmbedderData,
            kSimd256Constant,
            kSimd256Extract128Lane,
            kSimd256LoadTransform,
            kSimd256Unary,
            kSimd256Binop,
            kSimd256Shift,
            kSimd256Ternary,
            kSimd256Splat,
            kSimdPack128To256,
            kSimd256Shufd,
            kSimd256Shufps,
            kSimd256Unpack,
        }

        impl Opcode {
            pub fn name(&self) -> &'static str {
                OpcodeName(*self)
            }
        }

        pub fn OpcodeName(opcode: Opcode) -> &'static str {
            match opcode {
                Opcode::kCheckException => "CheckException",
                Opcode::kGoto => "Goto",
                Opcode::kTailCall => "TailCall",
                Opcode::kUnreachable => "Unreachable",
                Opcode::kReturn => "Return",
                Opcode::kBranch => "Branch",
                Opcode::kSwitch => "Switch",
                Opcode::kDeoptimize => "Deoptimize",
                Opcode::kWasmStackCheck => "WasmStackCheck",
                Opcode::kGlobalGet => "GlobalGet",
                Opcode::kGlobalSet => "GlobalSet",
                Opcode::kRootConstant => "RootConstant",
                Opcode::kIsRootConstant => "IsRootConstant",
                Opcode::kNull => "Null",
                Opcode::kIsNull => "IsNull",
                Opcode::kAssertNotNull => "AssertNotNull",
                Opcode::kRttCanon => "RttCanon",
                Opcode::kWasmTypeCheck => "WasmTypeCheck",
                Opcode::kWasmTypeCast => "WasmTypeCast",
                Opcode::kAnyConvertExtern => "AnyConvertExtern",
                Opcode::kExternConvertAny => "ExternConvertAny",
                Opcode::kWasmTypeAnnotation => "WasmTypeAnnotation",
                Opcode::kStructGet => "StructGet",
                Opcode::kStructSet => "StructSet",
                Opcode::kArrayGet => "ArrayGet",
                Opcode::kArraySet => "ArraySet",
                Opcode::kArrayLength => "ArrayLength",
                Opcode::kWasmAllocateArray => "WasmAllocateArray",
                Opcode::kWasmAllocateStruct => "WasmAllocateStruct",
                Opcode::kWasmRefFunc => "WasmRefFunc",
                Opcode::kStringAsWtf16 => "StringAsWtf16",
                Opcode::kStringPrepareForGetCodeUnit => "StringPrepareForGetCodeUnit",
                Opcode::kSimd128Constant => "Simd128Constant",
                Opcode::kSimd128Binop => "Simd128Binop",
                Opcode::kSimd128Unary => "Simd128Unary",
                Opcode::kSimd128Reduce => "Simd128Reduce",
                Opcode::kSimd128Shift => "Simd128Shift",
                Opcode::kSimd128Test => "Simd128Test",
                Opcode::kSimd128Splat => "Simd128Splat",
                Opcode::kSimd128Ternary => "Simd128Ternary",
                Opcode::kSimd128ExtractLane => "Simd128ExtractLane",
                Opcode::kSimd128ReplaceLane => "Simd128ReplaceLane",
                Opcode::kSimd128LaneMemory => "Simd128LaneMemory",
                Opcode::kSimd128LoadTransform => "Simd128LoadTransform",
                Opcode::kSimd128Shuffle => "Simd128Shuffle",
                Opcode::kArgumentsLength => "ArgumentsLength",
                Opcode::kBigIntBinop => "BigIntBinop",
                Opcode::kBigIntComparison => "BigIntComparison",
                Opcode::kBigIntUnary => "BigIntUnary",
                Opcode::kCheckedClosure => "CheckedClosure",
                Opcode::kWordBinopDeoptOnOverflow => "WordBinopDeoptOnOverflow",
                Opcode::kCheckEqualsInternalizedString => "CheckEqualsInternalizedString",
                Opcode::kCheckMaps => "CheckMaps",
                Opcode::kCompareMaps => "CompareMaps",
                Opcode::kFloat64Is => "Float64Is",
                Opcode::kObjectIs => "ObjectIs",
                Opcode::kObjectIsNumericValue => "ObjectIsNumericValue",
                Opcode::kFloat64SameValue => "Float64SameValue",
                Opcode::kSameValue => "SameValue",
                Opcode::kChangeOrDeopt => "ChangeOrDeopt",
                Opcode::kConvert => "Convert",
                Opcode::kConvertJSPrimitiveToObject => "ConvertJSPrimitiveToObject",
                Opcode::kConvertJSPrimitiveToUntagged => "ConvertJSPrimitiveToUntagged",
                Opcode::kConvertJSPrimitiveToUntaggedOrDeopt => "ConvertJSPrimitiveToUntaggedOrDeopt",
                Opcode::kConvertUntaggedToJSPrimitive => "ConvertUntaggedToJSPrimitive",
                Opcode::kConvertUntaggedToJSPrimitiveOrDeopt => "ConvertUntaggedToJSPrimitiveOrDeopt",
                Opcode::kTruncateJSPrimitiveToUntagged => "TruncateJSPrimitiveToUntagged",
                Opcode::kTruncateJSPrimitiveToUntaggedOrDeopt => "TruncateJSPrimitiveToUntaggedOrDeopt",
                Opcode::kDoubleArrayMinMax => "DoubleArrayMinMax",
                Opcode::kEnsureWritableFastElements => "EnsureWritableFastElements",
                Opcode::kFastApiCall => "FastApiCall",
                Opcode::kFindOrderedHashEntry => "FindOrderedHashEntry",
                Opcode::kLoadDataViewElement => "LoadDataViewElement",
                Opcode::kLoadFieldByIndex => "LoadFieldByIndex",
                Opcode::kLoadMessage => "LoadMessage",
                Opcode::kLoadStackArgument => "LoadStackArgument",
                Opcode::kLoadTypedElement => "LoadTypedElement",
                Opcode::kStoreDataViewElement => "StoreDataViewElement",
                Opcode::kStoreMessage => "StoreMessage",
                Opcode::kStoreTypedElement => "StoreTypedElement",
                Opcode::kMaybeGrowFastElements => "MaybeGrowFastElements",
                Opcode::kNewArgumentsElements => "NewArgumentsElements",
                Opcode::kNewArray => "NewArray",
                Opcode::kRuntimeAbort => "RuntimeAbort",
                Opcode::kStaticAssert => "StaticAssert",
                Opcode::kStringAt => "StringAt",
                Opcode::kStringComparison => "StringComparison",
                Opcode::kStringConcat => "StringConcat",
                Opcode::kStringFromCodePointAt => "StringFromCodePointAt",
                Opcode::kStringIndexOf => "StringIndexOf",
                Opcode::kStringLength => "StringLength",
                Opcode::kTypedArrayLength => "TypedArrayLength",
                Opcode::kStringSubstring => "StringSubstring",
                Opcode::kNewConsString => "NewConsString",
                Opcode::kTransitionAndStoreArrayElement => "TransitionAndStoreArrayElement",
                Opcode::kTransitionElementsKind => "TransitionElementsKind",
                Opcode::kTransitionElementsKindOrCheckMap => "TransitionElementsKindOrCheckMap",
                Opcode::kDebugPrint => "DebugPrint",
                Opcode::kCheckTurboshaftTypeOf => "CheckTurboshaftTypeOf",
                Opcode::kWord32SignHint => "Word32SignHint",
                Opcode::kWordBinop => "WordBinop",
                Opcode::kFloatBinop => "FloatBinop",
                Opcode::kWord32PairBinop => "Word32PairBinop",
                Opcode::kOverflowCheckedBinop => "OverflowCheckedBinop",
                Opcode::kWordUnary => "WordUnary",
                Opcode::kOverflowCheckedUnary => "OverflowCheckedUnary",
                Opcode::kFloatUnary => "FloatUnary",
                Opcode::kShift => "Shift",
                Opcode::kComparison => "Comparison",
                Opcode::kChange => "Change",
                Opcode::kTryChange => "TryChange",
                Opcode::kBitcastWord32PairToFloat64 => "BitcastWord32PairToFloat64",
                Opcode::kTaggedBitcast => "TaggedBitcast",
                Opcode::kSelect => "Select",
                Opcode::kPendingLoopPhi => "PendingLoopPhi",
                Opcode::kConstant => "Constant",
                Opcode::kLoadRootRegister => "LoadRootRegister",
                Opcode::kLoad => "Load",
                Opcode::kStore => "Store",
                Opcode::kRetain => "Retain",
                Opcode::kParameter => "Parameter",
                Opcode::kOsrValue => "OsrValue",
                Opcode::kStackPointerGreaterThan => "StackPointerGreaterThan",
                Opcode::kStackSlot => "StackSlot",
                Opcode::kFrameConstant => "FrameConstant",
                Opcode::kDeoptimizeIf => "DeoptimizeIf",
                Opcode::kTrapIf => "TrapIf",
                Opcode::kLoadStackPointer => "LoadStackPointer",
                Opcode::kSetStackPointer => "SetStackPointer",
                Opcode::kPhi => "Phi",
                Opcode::kFrameState => "FrameState",
                Opcode::kCall => "Call",
                Opcode::kCatchBlockBegin => "CatchBlockBegin",
                Opcode::kDidntThrow => "DidntThrow",
                Opcode::kTuple => "Tuple",
                Opcode::kProjection => "Projection",
                Opcode::kDebugBreak => "DebugBreak",
                Opcode::kAssumeMap => "AssumeMap",
                Opcode::kAtomicRMW => "AtomicRMW",
                Opcode::kAtomicWord32Pair => "AtomicWord32Pair",
                Opcode::kMemoryBarrier => "MemoryBarrier",
                Opcode::kComment => "Comment",
                Opcode::kDead => "Dead",
                Opcode::kAbortCSADcheck => "AbortCSADcheck",
                Opcode::kAllocate => "Allocate",
                Opcode::kDecodeExternalPointer => "DecodeExternalPointer",
                Opcode::kJSStackCheck => "JSStackCheck",
                Opcode::kGenericBinop => "GenericBinop",
                Opcode::kGenericUnop => "GenericUnop",
                Opcode::kToNumberOrNumeric => "ToNumberOrNumeric",
                Opcode::kStringToCaseIntl => "StringToCaseIntl",
                Opcode::kGetContinuationPreservedEmbedderData => {
                    "GetContinuationPreservedEmbedderData"
                }
                Opcode::kSetContinuationPreservedEmbedderData => {
                    "SetContinuationPreservedEmbedderData"
                }
                Opcode::kSimd256Constant => "Simd256Constant",
                Opcode::kSimd256Extract128Lane => "Simd256Extract128Lane",
                Opcode::kSimd256LoadTransform => "Simd256LoadTransform",
                Opcode::kSimd256Unary => "Simd256Unary",
                Opcode::kSimd256Binop => "Simd256Binop",
                Opcode::kSimd256Shift => "Simd256Shift",
                Opcode::kSimd256Ternary => "Simd256Ternary",
                Opcode::kSimd256Splat => "Simd256Splat",
                Opcode::kSimdPack128To256 => "SimdPack128To256",
                Opcode::kSimd256Shufd => "Simd256Shufd",
                Opcode::kSimd256Shufps => "Simd256Shufps",
                Opcode::kSimd256Unpack => "Simd256Unpack",
            }
        }

        impl From<Opcode> for u8 {
            fn from(opcode: Opcode) -> Self {
                match opcode {
                    Opcode::kCheckException => 0,
                    Opcode::kGoto => 1,
                    Opcode::kTailCall => 2,
                    Opcode::kUnreachable => 3,
                    Opcode::kReturn => 4,
                    Opcode::kBranch => 5,
                    Opcode::kSwitch => 6,
                    Opcode::kDeoptimize => 7,
                    Opcode::kWasmStackCheck => 8,
                    Opcode::kGlobalGet => 9,
                    Opcode::kGlobalSet => 10,
                    Opcode::kRootConstant => 11,
                    Opcode::kIsRootConstant => 12,
                    Opcode::kNull => 13,
                    Opcode::kIsNull => 14,
                    Opcode::kAssertNotNull => 15,
                    Opcode::kRttCanon => 16,
                    Opcode::kWasmTypeCheck => 17,
                    Opcode::kWasmTypeCast => 18,
                    Opcode::kAnyConvertExtern => 19,
                    Opcode::kExternConvertAny => 20,
                    Opcode::kWasmTypeAnnotation => 21,
                    Opcode::kStructGet => 22,
                    Opcode::kStructSet => 23,
                    Opcode::kArrayGet => 24,
                    Opcode::kArraySet => 25,
                    Opcode::kArrayLength => 26,
                    Opcode::kWasmAllocateArray => 27,
                    Opcode::kWasmAllocateStruct => 28,
                    Opcode::kWasmRefFunc => 29,
                    Opcode::kStringAsWtf16 => 30,
                    Opcode::kStringPrepareForGetCodeUnit => 31,
                    Opcode::kSimd128Constant => 32,
                    Opcode::kSimd128Binop => 33,
                    Opcode::kSimd128Unary => 34,
                    Opcode::kSimd128Reduce => 35,
                    Opcode::kSimd128Shift => 36,
                    Opcode::kSimd128Test => 37,
                    Opcode::kSimd128Splat => 38,
                    Opcode::kSimd128Ternary => 39,
                    Opcode::kSimd128ExtractLane => 40,
                    Opcode::kSimd128ReplaceLane => 41,
                    Opcode::kSimd128LaneMemory => 42,
                    Opcode::kSimd128LoadTransform => 43,
                    Opcode::kSimd128Shuffle => 44,
                    Opcode::kArgumentsLength => 45,
                    Opcode::kBigIntBinop => 46,
                    Opcode::kBigIntComparison => 47,
                    Opcode::kBigIntUnary => 48,
                    Opcode::kCheckedClosure => 49,
                    Opcode::kWordBinopDeoptOnOverflow => 50,
                    Opcode::kCheckEqualsInternalizedString => 51,
                    Opcode::kCheckMaps => 52,
                    Opcode::kCompareMaps => 53,
                    Opcode::kFloat64Is => 54,
                    Opcode::kObjectIs => 55,
                    Opcode::kObjectIsNumericValue => 56,
                    Opcode::kFloat64SameValue => 57,
                    Opcode::kSameValue => 58,
                    Opcode::kChangeOrDeopt => 59,
                    Opcode::kConvert => 60,
                    Opcode::kConvertJSPrimitiveToObject => 61,
                    Opcode::kConvertJSPrimitiveToUntagged => 62,
                    Opcode::kConvertJSPrimitiveToUntaggedOrDeopt => 63,
                    Opcode::kConvertUntaggedToJSPrimitive => 64,
                    Opcode::kConvertUntaggedToJSPrimitiveOrDeopt => 65,
                    Opcode::kTruncateJSPrimitiveToUntagged => 66,
                    Opcode::kTruncateJSPrimitiveToUntaggedOrDeopt => 67,
                    Opcode::kDoubleArrayMinMax => 68,
                    Opcode::kEnsureWritableFastElements => 69,
                    Opcode::kFastApiCall => 70,
                    Opcode::kFindOrderedHashEntry => 71,
                    Opcode::kLoadDataViewElement => 72,
                    Opcode::kLoadFieldByIndex => 73,
                    Opcode::kLoadMessage => 74,
                    Opcode::kLoadStackArgument => 75,
                    Opcode::kLoadTypedElement => 76,
                    Opcode::kStoreDataViewElement => 77,
                    Opcode::kStoreMessage => 78,
                    Opcode::kStoreTypedElement => 79,
                    Opcode::kMaybeGrowFastElements => 80,
                    Opcode::kNewArgumentsElements => 81,
                    Opcode::kNewArray => 82,
                    Opcode::kRuntimeAbort => 83,
                    Opcode::kStaticAssert => 84,
                    Opcode::kStringAt => 85,
                    Opcode::kStringComparison => 86,
                    Opcode::kStringConcat => 87,
                    Opcode::kStringFromCodePointAt => 88,
                    Opcode::kStringIndexOf => 89,
                    Opcode::kStringLength => 90,
                    Opcode::kTypedArrayLength => 91,
                    Opcode::kStringSubstring => 92,
                    Opcode::kNewConsString => 93,
                    Opcode::kTransitionAndStoreArrayElement => 94,
                    Opcode::kTransitionElementsKind => 95,
                    Opcode::kTransitionElementsKindOrCheckMap => 96,
                    Opcode::kDebugPrint => 97,
                    Opcode::kCheckTurboshaftTypeOf => 98,
                    Opcode::kWord32SignHint => 99,
                    Opcode::kWordBinop => 100,
                    Opcode::kFloatBinop => 101,
                    Opcode::kWord32PairBinop => 102,
                    Opcode::kOverflowCheckedBinop => 103,
                    Opcode::kWordUnary => 104,
                    Opcode::kOverflowCheckedUnary => 105,
                    Opcode::kFloatUnary => 106,
                    Opcode::kShift => 107,
                    Opcode::kComparison => 108,
                    Opcode::kChange => 109,
                    Opcode::kTryChange => 110,
                    Opcode::kBitcastWord32PairToFloat64 => 111,
                    Opcode::kTaggedBitcast => 112,
                    Opcode::kSelect => 113,
                    Opcode::kPendingLoopPhi => 114,
                    Opcode::kConstant => 115,
                    Opcode::kLoadRootRegister => 116,
                    Opcode::kLoad => 117,
                    Opcode::kStore => 118,
                    Opcode::kRetain => 119,
                    Opcode::kParameter => 120,
                    Opcode::kOsrValue => 121,
                    Opcode::kStackPointerGreaterThan => 122,
                    Opcode::kStackSlot => 123,
                    Opcode::kFrameConstant => 124,
                    Opcode::kDeoptimizeIf => 125,
                    Opcode::kTrapIf => 126,
                    Opcode::kLoadStackPointer => 127,
                    Opcode::kSetStackPointer => 128,
                    Opcode::kPhi => 129,
                    Opcode::kFrameState => 130,
                    Opcode::kCall => 131,
                    Opcode::kCatchBlockBegin => 132,
                    Opcode::kDidntThrow => 133,
                    Opcode::kTuple => 134,
                    Opcode::kProjection => 135,
                    Opcode::kDebugBreak => 136,
                    Opcode::kAssumeMap => 137,
                    Opcode::kAtomicRMW => 138,
                    Opcode::kAtomicWord32Pair => 139,
                    Opcode::kMemoryBarrier => 140,
                    Opcode::kComment => 141,
                    Opcode::kDead => 