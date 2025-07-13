// Converted from V8 C++ source files:
// Header: machine-operator.h
// Implementation: machine-operator.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod machine_operator {
use crate::codegen::atomic_memory_order::AtomicMemoryOrder;
use crate::codegen::machine_type::{
    ElementSizeLog2Of, MachineRepresentation, MachineType,
};
use crate::compiler::operator::Operator;
use crate::compiler::wasm_gc_lowering::MemoryAccessKind;
use std::cell::RefCell;
use std::cmp::PartialEq;
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem;
use std::ops::Deref;
use std::rc::Rc;
#[derive(Debug, Clone)]
pub struct V8_EXPORT_PRIVATE {}
pub struct V8 {}
pub struct v8 {}
pub struct StdoutStream {}
pub struct JSCallReducerAssembler {}
pub struct Flags {}
pub struct MachineOperatorBuilder {}
pub mod base {
pub struct Flags {}
}
pub mod compiler {
pub enum Flag {
}
}
pub mod execution {
pub mod isolate {
pub struct Isolate {}
}
}
pub mod revectorizer {
pub struct StoreRepresentation {}
}
pub mod linkage {
pub struct MachineType {}
}
pub struct StoreRepresentation {}
pub struct JSHeapBroker {}
pub struct HeapObjectRef {}
pub struct RootIndex {}
pub struct StoreRepresentation {}
pub struct Node {}
pub struct NumberMatcher<'a> {}
pub struct Reduction {}
pub enum IdentifyZeros {
    kSignedZeros,
    kUnsignedZeros,
    kBoth,
}
pub enum RoundingMode {
    TIES_EVEN,
    TIES_AWAY_FROM_ZERO,
    TOWARD_ZERO,
    TOWARD_POSITIVE_INFINITY,
    TOWARD_NEGATIVE_INFINITY,
}
pub struct VectorRepresentation {}
pub enum TypeFeedbackSource {}
pub struct MapRef {}
pub mod simplified_operator_reducer {
pub struct NumberMatcher<'a> {}
}
pub mod wasm_gc_lowering {
pub struct MachineRepresentation {}
}
pub mod csa_load_elimination {
pub struct Smi {}
}
pub mod machine_graph {
pub struct Operator {}
}
pub mod operator {
pub struct Operator {}
}
pub enum CallFrequency {}
pub enum StackCheckKind {
    JSFunctionEntry,
    CodeStubAssembler,
    Wasm,
    JSIterationBody,
}
pub enum IrOpcode {
    kStackSlot,
    kLoad,
    kStore,
    kWord32Sar,
    kWord64Sar,
    kTruncateFloat32ToUint32,
    kTruncateFloat64ToInt64,
    kTruncateFloat32ToInt32,
    kWord32Ctz,
    kWord64Ctz,
    kWord32Rol,
    kWord64Rol,
    kWord32ReverseBits,
    kWord64ReverseBits,
    kInt32AbsWithOverflow,
    kInt64AbsWithOverflow,
    kWord32Popcnt,
    kWord64Popcnt,
    kFloat32RoundDown,
    kFloat64RoundDown,
    kFloat32RoundUp,
    kFloat64RoundUp,
    kFloat32RoundTruncate,
    kFloat64RoundTruncate,
    kFloat64RoundTiesAway,
    kFloat32RoundTiesEven,
    kFloat64RoundTiesEven,
    kWord32Select,
    kWord64Select,
    kFloat32Select,
    kFloat64Select,
    kInt32AddWithOverflow,
    kInt32SubWithOverflow,
    kInt32MulWithOverflow,
    kInt64AddWithOverflow,
    kInt64SubWithOverflow,
    kInt64MulWithOverflow,
    kWord32And,
    kWord32Or,
    kWord32Xor,
    kWord32Shl,
    kWord32Shr,
    kWord32Ror,
    kWord32Equal,
    kInt32Add,
    kInt32Sub,
    kInt32Mul,
    kInt32MulHigh,
    kInt32Div,
    kInt32Mod,
    kInt32LessThan,
    kInt32LessThanOrEqual,
    kUint32Div,
    kUint32LessThan,
    kUint32LessThanOrEqual,
    kUint32Mod,
    kUint32MulHigh,
    kWord64And,
    kWord64Or,
    kWord64Xor,
    kWord64Shl,
    kWord64Shr,
    kWord64Ror,
    kWord64RorLowerable,
    kWord64Equal,
    kInt64Add,
    kInt64Sub,
    kInt64Mul,
    kInt64MulHigh,
    kInt64Div,
    kInt64Mod,
    kInt64LessThan,
    kInt64LessThanOrEqual,
    kUint64Div,
    kUint64Mod,
    kUint64LessThan,
    kUint64LessThanOrEqual,
    kUint64MulHigh,
    kWord64Clz,
    kWord64ClzLowerable,
    kWord32Clz,
    kWord32ReverseBytes,
    kWord64ReverseBytes,
    kSimd128ReverseBytes,
    kBitcastTaggedToWordForTagAndSmiBits,
    kBitcastWordToTaggedSigned,
    kTruncateFloat64ToWord32,
    kChangeFloat32ToFloat64,
    kChangeFloat64ToInt32,
    kChangeFloat64ToInt64,
    kChangeFloat64ToUint32,
    kChangeFloat64ToUint64,
    kTruncateFloat64ToUint32,
    kTryTruncateFloat32ToInt64,
    kTryTruncateFloat64ToInt64,
    kTryTruncateFloat32ToUint64,
    kTryTruncateFloat64ToUint64,
    kTryTruncateFloat64ToInt32,
    kTryTruncateFloat64ToUint32,
    kChangeInt32ToFloat64,
    kChangeInt64ToFloat64,
    kFloat64SilenceNaN,
    kRoundFloat64ToInt32,
    kRoundInt32ToFloat32,
    kRoundInt64ToFloat32,
    kRoundInt64ToFloat64,
    kRoundUint32ToFloat32,
    kRoundUint64ToFloat32,
    kRoundUint64ToFloat64,
    kBitcastWord32ToWord64,
    kChangeInt32ToInt64,
    kChangeUint32ToFloat64,
    kChangeUint32ToUint64,
    kTruncateFloat64ToFloat32,
    kTruncateInt64ToInt32,
    kBitcastFloat32ToInt32,
    kBitcastFloat64ToInt64,
    kBitcastInt32ToFloat32,
    kBitcastInt64ToFloat64,
    kSignExtendWord8ToInt32,
    kSignExtendWord16ToInt32,
    kSignExtendWord8ToInt64,
    kSignExtendWord16ToInt64,
    kSignExtendWord32ToInt64,
    kFloat32Abs,
    kFloat32Add,
    kFloat32Sub,
    kFloat32Mul,
    kFloat32Div,
    kFloat32Neg,
    kFloat32Sqrt,
    kFloat32Max,
    kFloat32Min,
    kFloat64Abs,
    kFloat64Acos,
    kFloat64Acosh,
    kFloat64Asin,
    kFloat64Asinh,
    kFloat64Atan,
    kFloat64Atan2,
    kFloat64Atanh,
    kFloat64Cbrt,
    kFloat64Cos,
    kFloat64Cosh,
    kFloat64Exp,
    kFloat64Expm1,
    kFloat64Log,
    kFloat64Log1p,
    kFloat64Log2,
    kFloat64Log10,
    kFloat64Max,
    kFloat64Min,
    kFloat64Neg,
    kFloat64Add,
    kFloat64Sub,
    kFloat64Mul,
    kFloat64Div,
    kFloat64Mod,
    kFloat64Pow,
    kFloat64Sin,
    kFloat64Sinh,
    kFloat64Sqrt,
    kFloat64Tan,
    kFloat64Tanh,
    kFloat32Equal,
    kFloat32LessThan,
    kFloat32LessThanOrEqual,
    kFloat64Equal,
    kFloat64LessThan,
    kFloat64LessThanOrEqual,
    kFloat64ExtractLowWord32,
    kFloat64ExtractHighWord32,
    kFloat64InsertLowWord32,
    kFloat64InsertHighWord32,
    kLoadStackCheckOffset,
    kLoadFramePointer,
    kLoadRootRegister,
    kLoadParentFramePointer,
    kInt32PairAdd,
    kInt32PairSub,
    kInt32PairMul,
    kWord32PairShl,
    kWord32PairShr,
    kWord32PairSar,
    kStackPointerGreaterThan,
    kStorePair,
    kProtectedStore,
    kUnalignedStore,
    kStoreIndirectPointer,
    kComment,
    kAbortCSADcheck,
    kDebugBreak,
    kMemoryBarrier,
    kWord32AtomicLoad,
    kWord64AtomicLoad,
    kWord32AtomicStore,
    kWord64AtomicStore,
    kWord32AtomicExchange,
    kWord64AtomicExchange,
    kWord32AtomicCompareExchange,
    kWord64AtomicCompareExchange,
    kWord32AtomicAdd,
    kWord32AtomicSub,
    kWord32AtomicAnd,
    kWord32AtomicOr,
    kWord32AtomicXor,
    kWord64AtomicAdd,
    kWord64AtomicSub,
    kWord64AtomicAnd,
    kWord64AtomicOr,
    kWord64AtomicXor,
    kWord32AtomicPairLoad,
    kWord32AtomicPairStore,
    kWord32AtomicPairAdd,
    kWord32AtomicPairSub,
    kWord32AtomicPairAnd,
    kWord32AtomicPairOr,
    kWord32AtomicPairXor,
    kWord32AtomicPairExchange,
    kWord32AtomicPairCompareExchange,
    kStoreTrapOnNull,
    kProtectedLoad,
    kLoadTrapOnNull,
    kUnalignedLoad,
    kLoadImmutable,
    kTruncateFloat64ToFloat16RawBits,
    kChangeFloat16RawBitsToFloat64,
    kI8x16Splat,
    kI8x16Eq,
    kI8x16BitMask,
    kF64x2Splat,
    kF64x2Abs,
    kF64x2Neg,
    kF64x2Sqrt,
    kF64x2Add,
    kF64x2Sub,
    kF64x2Mul,
    kF64x2Div,
    kF64x2Min,
    kF64x2Max,
    kF64x2Eq,
    kF64x2Ne,
    kF64x2Lt,
    kF64x2Le,
    kF64x2Qfma,
    kF64x2Qfms,
    kF64x2Pmin,
    kF64x2Pmax,
    kF64x2Ceil,
    kF64x2Floor,
    kF64x2Trunc,
    kF64x2NearestInt,
    kF64x2ConvertLowI32x4S,
    kF64x2ConvertLowI32x4U,
    kF64x2PromoteLowF32x4,
    kF32x4Splat,
    kF32x4Abs,
    kF32x4Neg,
    kF32x4Sqrt,
    kF32x4Add,
    kF32x4Sub,
    kF32x4Mul,
    kF32x4Div,
    kF32x4Min,
    kF32x4Max,
    kF32x4Eq,
    kF32x4Ne,
    kF32x4Lt,
    kF32x4Le,
    kF32x4Qfma,
    kF32x4Qfms,
    kF32x4Pmin,
    kF32x4Pmax,
    kF32x4Ceil,
    kF32x4Floor,
    kF32x4Trunc,
    kF32x4NearestInt,
    kF32x4DemoteF64x2Zero,
    kF16x8Splat,
    kF16x8Abs,
    kF16x8Neg,
    kF16x8Sqrt,
    kF16x8Ceil,
    kF16x8Floor,
    kF16x8Trunc,
    kF16x8NearestInt,
    kF16x8Add,
    kF16x8Sub,
    kF16x8Mul,
    kF16x8Div,
    kF16x8Min,
    kF16x8Max,
    kF16x8Pmin,
    kF16x8Pmax,
    kF16x8Eq,
    kF16x8Ne,
    kF16x8Lt,
    kF16x8Le,
    kF16x8SConvertI16x8,
    kF16x8UConvertI16x8,
    kI16x8SConvertF16x8,
    kI16x8UConvertF16x8,
    kF32x4PromoteLowF16x8,
    kF16x8DemoteF32x4Zero,
    kF16x8DemoteF64x2Zero,
    kF16x8Qfma,
    kF16x8Qfms,
    kI64x2Splat,
    kI64x2SplatI32Pair,
    kI64x2Abs,
    kI64x2Neg,
    kI64x2SConvertI32x4Low,
    kI64x2SConvertI32x4High,
    kI64x2UConvertI32x4Low,
    kI64x2UConvertI32x4High,
    kI64x2BitMask,
    kI64x2Shl,
    kI64x2ShrS,
    kI64x2Add,
    kI64x2Sub,
    kI64x2Mul,
    kI64x2Eq,
    kI64x2Ne,
    kI64x2GtS,
    kI64x2GeS,
    kI64x2ShrU,
    kI64x2ExtMulLowI32x4S,
    kI64x2ExtMulHighI32x4S,
    kI64x2ExtMulLowI32x4U,
    kI64x2ExtMulHighI32x4U,
    kI32x4Splat,
    kI32x4SConvertF32x4,
    kI32x4SConvertI16x8Low,
    kI32x4SConvertI16x8High,
    kI32x4Neg,
    kI32x4Shl,
    kI32x4ShrS,
    kI32x4Add,
    kI32x4Sub,
    kI32x4Mul,
    kI32x4MinS,
    kI32x4MaxS,
    kI32x4Eq,
    kI32x4Ne,
    kI32x4GtS,
    kI32x4GeS,
    kI32x4UConvertF32x4,
    kI32x4UConvertI16x8Low,
    kI32x4UConvertI16x8High,
    kI32x4ShrU,
    kI32x4MinU,
    kI32x4MaxU,
    kI32x4GtU,
    kI32x4GeU,
    kI32x4Abs,
    kI32x4BitMask,
    kI32x4DotI16x8S,
    kI32x4ExtMulLowI16x8S,
    kI32x4ExtMulHighI16x8S,
    kI32x4ExtMulLowI16x8U,
    kI32x4ExtMulHighI16x8U,
    kI32x4ExtAddPairwiseI16x8S,
    kI32x4ExtAddPairwiseI16x8U,
    kI32x4TruncSatF64x2SZero,
    kI32x4TruncSatF64x2UZero,
    kI16x8Splat,
    kI16x8SConvertI8x16Low,
    kI16x8SConvertI8x16High,
    kI16x8Neg,
    kI16x8Shl,
    kI16x8ShrS,
    kI16x8SConvertI32x4,
    kI16x8Add,
    kI16x8AddSatS,
    kI16x8Sub,
    kI16x8SubSatS,
    kI16x8Mul,
    kI16x8MinS,
    kI16x8MaxS,
    kI16x8Eq,
    kI16x8Ne,
    kI16x8GtS,
    kI16x8GeS,
    kI16x8UConvertI8x16Low,
    kI16x8UConvertI8x16High,
    kI16x8ShrU,
    kI16x8UConvertI32x4,
    kI16x8AddSatU,
    kI16x8SubSatU,
    kI16x8MinU,
    kI16x8MaxU,
    kI16x8GtU,
    kI16x8GeU,
    kI16x8RoundingAverageU,
    kI16x8Q15MulRSatS,
    kI16x8Abs,
    kI16x8BitMask,
    kI16x8ExtMulLowI8x16S,
    kI16x8ExtMulHighI8x16S,
    kI16x8ExtMulLowI8x16U,
    kI16x8ExtMulHighI8x16U,
    kI16x8ExtAddPairwiseI8x16S,
    kI16x8ExtAddPairwiseI8x16U,
    kI8x16Splat,
    kI8x16Neg,
    kI8x16Shl,
    kI8x16ShrS,
    kI8x16SConvertI16x8,
    kI8x16Add,
    kI8x16AddSatS,
    kI8x16Sub,
    kI8x16SubSatS,
    kI8x16MinS,
    kI8x16MaxS,
    kI8x16Ne,
    kI8x16GtS,
    kI8x16GeS,
    kI8x16ShrU,
    kI8x16UConvertI16x8,
    kI8x16AddSatU,
    kI8x16SubSatU,
    kI8x16MinU,
    kI8x16MaxU,
    kI8x16GtU,
    kI8x16GeU,
    kI8x16RoundingAverageU,
    kI8x16Popcnt,
    kI8x16Abs,
    kS128Const,
    kS128Zero,
    kS128And,
    kS128Or,
    kS128Xor,
    kS128Not,
    kS128Select,
    kS128AndNot,
    kI8x16Swizzle,
    kI8x16Shuffle,
    kV128AnyTrue,
    kI64x2AllTrue,
    kI32x4AllTrue,
    kI16x8AllTrue,
    kI8x16AllTrue,
    kI8x16RelaxedLaneSelect,
    kI16x8RelaxedLaneSelect,
    kI32x4RelaxedLaneSelect,
    kI64x2RelaxedLaneSelect,
    kF32x4RelaxedMin,
    kF32x4RelaxedMax,
    kF64x2RelaxedMin,
    kF64x2RelaxedMax,
    kI32x4RelaxedTruncF32x4S,
    kI32x4RelaxedTruncF32x4U,
    kI32x4RelaxedTruncF64x2SZero,
    kI32x4RelaxedTruncF64x2UZero,
    kI16x8RelaxedQ15MulRS,
    kI16x8DotI8x16I7x16S,
    kI32x4DotI8x16I7x16AddS,
    kF64x4Min,
    kF64x4Max,
    kF64x4Add,
    kF64x4Abs,
    kF64x4Neg,
    kF64x4Sqrt,
    kF32x8Abs,
    kF32x8Neg,
    kF32x8Sqrt,
    kF32x8Add,
    kI64x4Add,
    kI32x8Add,
    kI16x16Add,
    kI8x32Add,
    kF64x4Sub,
    kF32x8Sub,
    kI64x4Sub,
    kI32x8Sub,
    kI16x16Sub,
    kI8x32Sub,
    kF64x4Mul,
    kF32x8Mul,
    kI64x4Mul,
    kI32x8Mul,
    kI16x16Mul,
    kF64x4Div,
    kF32x8Div,
    kI16x16AddSatS,
    kI8x32AddSatS,
    kI16x16AddSatU,
    kI8x32AddSatU,
    kI16x16SubSatS,
    kI8x32SubSatS,
    kI16x16SubSatU,
    kI8x32SubSatU,
    kF32x8Min,
    kF32x8Max,
    kF32x8Pmin,
    kF32x8Pmax,
    kF32x8Eq,
    kF64x4Eq,
    kI64x4Eq,
    kI32x8Eq,
    kI16x16Eq,
    kI8x32Eq,
    kF32x8Ne,
    kF64x4Ne,
    kI64x4GtS,
    kI32x8GtS,
    kI16x16GtS,
    kI8x32GtS,
    kF64x4Lt,
    kF32x8Lt,
    kF64x4Le,
    kF32x8Le,
    kI32x8MinS,
    kI16x16MinS,
    kI8x32MinS,
    kI32x8MinU,
    kI16x16MinU,
    kI8x32MinU,
    kI32x8MaxS,
    kI16x16MaxS,
    kI8x32MaxS,
    kI32x8MaxU,
    kI16x16MaxU,
    kI8x32MaxU,
    kI64x4Ne,
    kI64x4GeS,
    kI32x8Ne,
    kI32x8GtU,
    kI32x8GeS,
    kI32x8GeU,
    kI16x16Ne,
    kI16x16GtU,
    kI16x16GeS,
    kI16x16GeU,
    kI8x32Ne,
    kI8x32GtU,
    kI8x32GeS,
    kI8x32GeU,
    kI32x8SConvertF32x8,
    kI32x8UConvertF32x8,
    kF64x4ConvertI32x4S,
    kF32x8SConvertI32x8,
    kF32x8UConvertI32x8,
    kF32x4DemoteF64x4,
    kI64x4SConvertI32x4,
    kI64x4UConvertI32x4,
    kI32x8SConvertI16x8,
    kI32x8UConvertI16x8,
    kI16x16SConvertI8x16,
    kI16x16UConvertI8x16,
    kI16x16SConvertI32x8,
    kI16x16UConvertI32x8,
    kI8x32SConvertI16x16,
    kI8x32UConvertI16x16,
    kI32x8Neg,
    kI32x8Abs,
    kI16x16Neg,
    kI16x16Abs,
    kI8x32Neg,
    kI8x32Abs,
    kI64x4Shl,
    kI64x4ShrU,
    kI32x8Shl,
    kI32x8ShrS,
    kI32x8ShrU,
    kI16x16Shl,
    kI16x16ShrS,
    kI16x16ShrU,
    kI32x8DotI16x16S,
    kI16x16RoundingAverageU,
    kI8x32RoundingAverageU,
    kI64x4ExtMulI32x4S,
    kI64x4ExtMulI32x4U,
    kI32x8ExtMulI16x8S,
    kI32x8ExtMulI16x8U,
    kI16x16ExtMulI8x16S,
    kI16x16ExtMulI8x16U,
    kI32x8ExtAddPairwiseI16x16S,
    kI32x8ExtAddPairwiseI16x16U,
    kI16x16ExtAddPairwiseI8x32S,
    kI16x16ExtAddPairwiseI8x32U,
    kS256Const,
    kS256Zero,
    kS256And,
    kS256Or,
    kS256Xor,
    kS256Not,
    kS256Select,
    kS256AndNot,
    kF32x8Qfma,
    kF32x8Qfms,
    kF64x4Qfma,
    kF64x4Qfms,
    kI64x4RelaxedLaneSelect,
    kI32x8RelaxedLaneSelect,
    kI16x16RelaxedLaneSelect,
    kI8x32RelaxedLaneSelect,
    kI32x8DotI8x32I7x32AddS,
    kI16x16DotI8x32I7x32S,
    kF32x8RelaxedMin,
    kF32x8RelaxedMax,
    kF64x4RelaxedMin,
    kF64x4RelaxedMax,
    kI32x8RelaxedTruncF32x8S,
    kI32x8RelaxedTruncF32x8U,
        kExtractF128,
        kLoadLane,
        kStoreLane,
        kSetStackPointer,
        kLoadStackPointer
}

// For operators that are not supported on all platforms.
pub struct OptionalOperator {
    supported_: bool,
    op_: *const Operator,
}

impl OptionalOperator {
    pub fn new(supported: bool, op: *const Operator) -> Self {
        OptionalOperator { supported_: supported, op_: op }
    }

    pub fn is_supported(&self) -> bool {
        self.supported_
    }
    // Gets the operator only if it is supported.
    pub fn op(&self) -> *const Operator {
        assert!(self.supported_);
        self.op_
    }
    // Always gets the operator, even for unsupported operators. This is useful to
    // use the operator as a placeholder in a graph, for instance.
    pub fn placeholder(&self) -> *const Operator {
        self.op_
    }
}

// A Load needs a MachineType.
pub type LoadRepresentation = MachineType;

pub fn load_representation_of(op: *const Operator) -> LoadRepresentation {
    LoadRepresentation {}
}

// A Word(32|64)AtomicLoad needs both a LoadRepresentation and a memory
// order.
#[derive(Clone, Copy, Debug)]
pub struct AtomicLoadParameters {
    representation_: LoadRepresentation,
    order_: AtomicMemoryOrder,
    kind_: MemoryAccessKind,
}

impl AtomicLoadParameters {
    pub fn new(
        representation: LoadRepresentation,
        order: AtomicMemoryOrder,
        kind: MemoryAccessKind,
    ) -> Self {
        AtomicLoadParameters {
            representation_: representation,
            order_: order,
            kind_: kind,
        }
    }

    pub fn representation(&self) -> LoadRepresentation {
        self.representation_.clone()
    }
    pub fn order(&self) -> AtomicMemoryOrder {
        self.order_
    }
    pub fn kind(&self) -> MemoryAccessKind {
        self.kind_
    }
}

impl PartialEq for AtomicLoadParameters {
    fn eq(&self, other: &Self) -> bool {
        self.representation() == other.representation()
            && self.order() == other.order()
            && self.kind() == other.kind()
    }
}

impl Eq for AtomicLoadParameters {}

impl Hash for AtomicLoadParameters {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.representation().hash(state);
        self.order().hash(state);
        self.kind().hash(state);
    }
}

impl fmt::Display for AtomicLoadParameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {:?}", self.representation(), self.order())
    }
}

pub fn atomic_load_parameters_of(op: *const Operator) -> AtomicLoadParameters {
    AtomicLoadParameters::new(
        LoadRepresentation {},
        AtomicMemoryOrder::Relaxed,
        MemoryAccessKind::kNormal,
    )
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AtomicOpParameters {
    type_: MachineType,
    kind_: MemoryAccessKind,
}

impl AtomicOpParameters {
    pub fn new(type_: MachineType, kind: MemoryAccessKind) -> Self {
        AtomicOpParameters { type_: type_, kind_: kind }
    }

    pub fn type_(&self) -> MachineType {
        self.type_.
