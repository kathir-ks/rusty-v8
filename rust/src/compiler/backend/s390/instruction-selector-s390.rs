// Copyright 2015 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::ops::{BitOr, BitAnd, BitXor};
use std::convert::TryFrom;

//use crate::base::logging; // Assuming logging is handled via standard Rust means
//use crate::compiler::backend::instruction_selector_adapter; // Assuming these are custom V8 modules
//use crate::compiler::backend::instruction_selector_impl; // Assuming these are custom V8 modules
//use crate::compiler::turboshaft::operations; // Assuming these are custom V8 modules
//use crate::compiler::turboshaft::opmasks; // Assuming these are custom V8 modules
//use crate::execution::frame_constants; // Assuming these are custom V8 modules

// Define dummy structs for V8 types that are not available in Rust
// These need to be replaced with actual implementations for the code to work
pub struct InstructionOperand {}
pub struct OpIndex {}
pub struct Operation {
    pub opcode: Opcode,
}

impl Operation {
    pub fn TryCast<T>(&self) -> Option<&T> {
        None // Placeholder, replace with actual casting logic
    }
    pub fn Is<T>(&self) -> bool {
        false // Placeholder, replace with actual type checking logic
    }

    pub fn Cast<T>(&self) -> &Self {
        self  // Placeholder. Always returns self for compilation, replace in full implementation.
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Opcode {
  kWordBinop,
  kWordUnary,
  kChange,
  kShift,
  kOverflowCheckedBinop,
  kLoad,
  kStackSlot,
  kArchAbortCSADcheck,
  kStore,
  kProtectedStore,
  kUnalignedLoad,
  kUnalignedStore,
  kStackPointerGreaterThan,
  kWord64And,
  kWord64Shl,
  kWord64Shr,
  kWord32Rol,
  kWord64Rol,
  kWord32Ctz,
  kWord64Ctz,
  kWord32ReverseBits,
  kWord64ReverseBits,
  kInt32AbsWithOverflow,
  kInt64AbsWithOverflow,
  kWord64ReverseBytes,
  kWord32ReverseBytes,
  kSimd128ReverseBytes,
  kWord32RolOver,
  kSwitch,
  kWord32Equal,
  kInt32LessThan,
  kInt32LessThanOrEqual,
  kUint32LessThan,
  kUint32LessThanOrEqual,
  kWord64Equal,
  kInt64LessThan,
  kInt64LessThanOrEqual,
  kUint64LessThan,
  kUint64LessThanOrEqual,
  kTruncateFloat64ToFloat16RawBits,
  kChangeFloat16RawBitsToFloat64,
  kFloat32Equal,
  kFloat32LessThan,
  kFloat32LessThanOrEqual,
  kFloat64Equal,
  kFloat64LessThan,
  kFloat64LessThanOrEqual,
  kBitcastWord32PairToFloat64
}

pub struct ConstantOp {
  pub kind: ConstantOpKind,
  word32_: i32,
  word64_: i64,
}

impl ConstantOp {
  pub fn word32(&self) -> i32 {
    self.word32_
  }
  pub fn word64(&self) -> i64 {
      self.word64_
  }
}
pub enum ConstantOpKind {
    kWord32,
    kWord64,
}

pub struct LoadOp {
  base_: OpIndex,
  index_: Option<OpIndex>,
  element_size_log2_: i32,
  offset_: i32,
  kind_: LoadKind,
}

impl LoadOp {
    pub fn base(&self) -> OpIndex {
        self.base_
    }
    pub fn index(&self) -> Option<OpIndex> {
        self.index_
    }
    pub fn element_size_log2(&self) -> i32 {
        self.element_size_log2_
    }
    pub fn offset(&self) -> i32 {
        self.offset_
    }
}

pub struct StoreOp {
  base_: OpIndex,
  index_: Option<OpIndex>,
  element_size_log2_: i32,
  offset_: i32,
  kind_: LoadKind,
  value_: OpIndex,
}

impl StoreOp {
    pub fn base(&self) -> OpIndex {
        self.base_
    }
    pub fn index(&self) -> Option<OpIndex> {
        self.index_
    }
    pub fn element_size_log2(&self) -> i32 {
        self.element_size_log2_
    }
    pub fn offset(&self) -> i32 {
        self.offset_
    }
    pub fn value(&self) -> OpIndex {
        self.value_
    }
}

pub struct ShiftOp {
  rep_: WordRepresentation,
  kind_: ShiftOpKind,
  left_: OpIndex,
  right_: OpIndex,
}

impl ShiftOp {
  pub fn rep(&self) -> WordRepresentation {
    self.rep_
  }
  pub fn kind(&self) -> ShiftOpKind {
    self.kind_
  }
  pub fn left(&self) -> OpIndex {
    self.left_
  }
  pub fn right(&self) -> OpIndex {
    self.right_
  }
}

pub struct Simd128LaneMemoryOp {
  base_: OpIndex,
  index_: OpIndex,
  kind_: LoadKind,
}

impl Simd128LaneMemoryOp {
    pub fn base(&self) -> OpIndex {
        self.base_
    }
    pub fn index(&self) -> OpIndex {
        self.index_
    }
}

pub struct Simd128LoadTransformOp {
  base_: OpIndex,
  index_: OpIndex,
  load_kind_: LoadKind,
  offset_: i32,
}

impl Simd128LoadTransformOp {
    pub fn base(&self) -> OpIndex {
        self.base_
    }
    pub fn index(&self) -> OpIndex {
        self.index_
    }
}

pub struct WordBinopOp {
    pub rep: WordRepresentation,
    pub kind: WordBinopOpKind,
}
impl WordBinopOp {
  pub fn left(&self) -> OpIndex {
    OpIndex{}
  }

  pub fn right(&self) -> OpIndex {
    OpIndex{}
  }
  
  pub fn IsCommutative(kind: WordBinopOpKind) -> bool {
    match kind {
      WordBinopOpKind::kAdd => true,
      _ => false
    }
  }
}

pub enum WordBinopOpKind {
  kAdd,
  kSub,
  kMul,
  kSignedDiv,
  kUnsignedDiv,
  kSignedMod,
  kUnsignedMod,
  kBitwiseAnd,
  kBitwiseOr,
  kBitwiseXor,
  kSignedMulOverflownBits,
  kUnsignedMulOverflownBits,
}
pub struct OverflowCheckedBinopOp {
    pub rep: WordRepresentation,
    pub kind: OverflowCheckedBinopOpKind,
}

pub enum OverflowCheckedBinopOpKind {
  kSignedAdd,
  kSignedSub,
  kSignedMul,
}

pub struct WordUnaryOp {
    pub rep: WordRepresentation,
    pub kind: WordUnaryOpKind,
}

pub enum WordUnaryOpKind {
  kCountLeadingZeros,
  kPopCount,
  kSignExtend8,
  kSignExtend16,
}

pub struct ChangeOp {
  pub from: FloatRepresentation,
  pub to: WordRepresentation,
  pub kind: ChangeOpKind,
}

pub enum ChangeOpKind {
  kExtractLowHalf,
  kExtractHighHalf,
  kBitcast,
  kSignedFloatTruncateOverflowToMin,
  kUnsignedFloatTruncateOverflowToMin,
}

pub struct StackSlotOp {
  pub size: i32,
  pub alignment: i32,
  pub is_tagged: bool,
}

pub struct StackPointerGreaterThanOp {
  pub kind: StackCheckKind,
  stack_limit_: OpIndex,
}

impl StackPointerGreaterThanOp {
  pub fn stack_limit(&self) -> OpIndex {
    self.stack_limit_
  }
}

pub struct ComparisonOp {
  pub rep: RegisterRepresentation,
  pub kind: ComparisonOpKind,
}

impl ComparisonOp {
    pub fn right(&self) -> OpIndex {
      OpIndex{}
    }
    pub fn left(&self) -> OpIndex {
      OpIndex{}
    }
}

pub enum ComparisonOpKind {
  kEqual,
  kSignedLessThan,
  kSignedLessThanOrEqual
}

pub struct ProjectionOp {
    pub index: u32,
}

impl ProjectionOp {
    pub fn input(&self) -> OpIndex {
      OpIndex{}
    }
}

pub struct OverflowCheckedUnaryOp {
    pub rep: WordRepresentation,
    pub kind: OverflowCheckedUnaryOpKind,
}

pub enum OverflowCheckedUnaryOpKind {
  kAbs,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WordRepresentation {
  Word32,
  Word64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FloatRepresentation {
  Float32,
  Float64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RegisterRepresentation {
  Word32,
  Word64,
  Float32,
  Float64,
  Tagged
}

impl RegisterRepresentation {
  pub fn MapTaggedToWord(&self) -> Option<Self> {
    Some(*self)
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ShiftOpKind {
  kShiftRightArithmetic,
  kShiftRightLogical,
  kShiftRightArithmeticShiftOutZeros,
  kShiftLeft,
  kRotateRight,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LoadKind {

}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StackCheckKind {
  kJSFunctionEntry,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MachineRepresentation {
  Int8,
  Uint8,
  Int16,
  Uint16,
  Int32,
  Uint32,
  Int64,
  Uint64,
  Float16,
  Float32,
  Float64,
  AnyTagged,
  TaggedPointer,
  TaggedSigned,
  AnyUncompressedTagged,
  UncompressedTaggedPointer,
  UncompressedTaggedSigned,
  Simd128,
  ProtectedPointer,
  IndirectPointer,
  SandboxedPointer,
  Simd256,
  Bit,
  Word8,
  Word16,
  Word32,
  CompressedPointer,
  Compressed,
  TaggedSignedLoad,
  TaggedPointerLoad,
  TaggedLoad,
  Word64Load,
  Float32Load,
  Float64Load,
  MapWord,
  Float16RawBits,
  kNone
}

impl MachineRepresentation {
    fn is_signed(&self) -> bool {
        match self {
            MachineRepresentation::Int8 | MachineRepresentation::Int16 | MachineRepresentation::Int32 | MachineRepresentation::Int64 => true,
            _ => false,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ArchOpcode {
  kArchStackSlot,
  kArchAbortCSADcheck,
  kS390_LoadWordS8,
  kS390_LoadWordU8,
  kS390_LoadWordS16,
  kS390_LoadWordU16,
  kS390_LoadWordU32,
  kS390_LoadWord64,
  kS390_LoadFloat32,
  kS390_LoadDouble,
  kS390_LoadDecompressTagged,
  kS390_LoadDecompressTaggedSigned,
  kS390_LoadSimd128,
  kArchStoreWithWriteBarrier,
  kS390_StoreWord8,
  kS390_StoreWord16,
  kS390_StoreWord32,
  kS390_StoreWord64,
  kS390_StoreFloat32,
  kS390_StoreDouble,
  kS390_StoreCompressTagged,
  kS390_StoreSimd128,
  kArchStackPointerGreaterThan,
  kS390_RotLeftAndClearLeft64,
  kS390_RotLeftAndClearRight64,
  kS390_RotLeftAndClear64,
  kS390_SignExtendWord16ToInt32,
  kS390_SignExtendWord8ToInt32,
  kS390_Abs32,
  kS390_Abs64,
  kS390_LoadReverse64RR,
  kS390_LoadReverse32RR,
  kS390_LoadReverseSimd128RR,
  kS390_Neg32,
  kS390_Neg64,
  kS390_ShiftLeft32,
  kS390_ShiftLeft64,
  kS390_ShiftRight32,
  kS390_ShiftRight64,
  kS390_Mul32,
  kS390_SignExtendWord32ToInt64,
  kS390_SignExtendWord16ToInt64,
  kS390_SignExtendWord8ToInt64,
  kS390_Popcnt32,
  kS390_Cntlz32,
  kS390_BitcastInt32ToFloat32,
  kS390_Uint32ToDouble,
  kS390_Uint32ToFloat32,
  kS390_Int32ToFloat32,
  kS390_Int32ToDouble,
  kS390_SignExtendWord32ToInt64Convert,
  kS390_Uint32ToUint64,
  kS390_DoubleInsertHighWord32,
  kS390_DoubleInsertLowWord32,
  kS390_Sub32,
  kS390_MulHighU32,
  kS390_ModU32,
  kS390_DivU32,
  kS390_Mod32,
  kS390_Div32,
  kS390_MulHigh32,
  kS390_And32,
  kS390_Xor32,
  kS390_RotRight32,
  kS390_Or32,
  kS390_ShiftRightArith32,
  kS390_Add32,
  kS390_Mul32WithOverflow,
  kS390_And64,
  kS390_MulHighS64,
  kS390_Mod64,
  kS390_Div64,
  kS390_Or64,
  kS390_Xor64,
  kS390_RotRight64,
  kS390_DoubleToUint32,
  kS390_Float64SilenceNaN,
  kS390_SqrtDouble,
  kS390_CeilDouble,
  kS390_TruncateDouble,
  kS390_DoubleNearestInt,
  kS390_RoundDouble,
  kS390_FloorDouble,
  kS390_NegDouble,
  kS390_AbsDouble,
  kS390_SqrtFloat,
  kS390_CeilFloat,
  kS390_TruncateFloat,
  kS390_FloatNearestInt,
  kS390_FloorFloat,
  kS390_NegFloat,
  kS390_AbsFloat,
  kS390_BitcastDoubleToInt64,
  kS390_DoubleExtractHighWord32,
  kS390_DoubleExtractLowWord32,
  kS390_DoubleToUint64,
  kS390_DoubleToInt64,
  kS390_DoubleToFloat32,
  kArchTruncateDoubleToI,
  kS390_Float32ToDouble,
  kS390_Int64ToInt32,
  kS390_Cntlz64,
  kS390_Popcnt64,
  kS390_BitcastInt64ToDouble,
  kS390_Int64ToDouble,
  kS390_Uint64ToDouble,
  kS390_Uint64ToFloat32,
  kS390_Int64ToFloat32,
  kS390_Lay,
  kS390_Mul64,
  kS390_DoubleFromWord32Pair,
  kS390_MinDouble,
  kS390_MaxDouble,
  kS390_MinFloat,
  kS390_MaxFloat,
  kS390_DivFloat,
  kS390_MulFloat,
  kS390_SubFloat,
  kS390_AddFloat,
  kS390_SubDouble,
  kS390_DivDouble,
  kS390_BitcastFloat32ToInt32,
  kS390_Tst32,
  kS390_Tst64,
  kS390_Cmp32,
  kS390_Cmp64,
  kS390_CmpFloat,
  kS390_CmpDouble,
  kS390_LoadAndTestWord32,
  kS390_LoadAndTestWord64,
  kS390_Mul64WithOverflow,
  kS390_ModDouble,
  kA
  kS390_LoadReverse32,
  kS390_LoadReverse64,
  kS390_StoreReverse32,
  kS390_StoreReverse64,
  kS390_StoreReverseSimd128,
  kS390_LoadReverseSimd128
}

pub struct AddressingModeField {}

impl AddressingModeField {
    pub fn encode(mode: AddressingMode) -> ArchOpcode {
        // Dummy implementation. Replace with actual encoding
        match mode {
            AddressingMode::kMode_MRI => ArchOpcode::kArchStackSlot,
            AddressingMode::kMode_MRR => ArchOpcode::kArchStackSlot,
            AddressingMode::kMode_MR => ArchOpcode::kArchStackSlot,
            AddressingMode::kMode_Root => ArchOpcode::kArchStackSlot,
            _ => ArchOpcode::kArchStackSlot
        }
    }
}

pub struct MiscField {}

impl MiscField {
  pub fn encode(value: i32) -> ArchOpcode {
    ArchOpcode::kArchStackSlot  // Dummy implementation. Replace with actual encoding
  }
}

pub struct ArchOpcodeField {}

impl ArchOpcodeField {
    pub fn decode(op: InstructionCode) -> ArchOpcode {
        // Dummy implementation. Replace with actual decoding logic
        ArchOpcode::kArchStackSlot
    }
}

pub struct RecordWriteModeField {}

impl RecordWriteModeField {
  pub fn encode(mode: RecordWriteMode) -> ArchOpcode {
    ArchOpcode::kArchStackSlot  // Dummy implementation. Replace with actual encoding
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AddressingMode {
  kMode_MRI,
  kMode_MRRI,
  kMode_MRR,
  kMode_MR,
  kMode_Root
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DisplacementMode {
  kPositiveDisplacement,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WriteBarrierKind {
  kNoWriteBarrier,
  kFullWriteBarrier
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RecordWriteMode {
  kNoRecordWrite,
  kAddress,
  kMap,
  kStub
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FlagsCondition {
  kEqual,
  kNotEqual,
  kSignedLessThan,
  kSignedLessThanOrEqual,
  kUnsignedLessThan,
  kUnsignedLessThanOrEqual,
  kUnsignedGreaterThan,
  kUnsignedGreaterThanOrEqual,
  kOverflow,
  kStackPointerGreaterThanCondition
}

// TODO: Replace with actual implementation
pub fn WriteBarrierKindToRecordWriteMode(kind: WriteBarrierKind) -> RecordWriteMode {
  RecordWriteMode::kNoRecordWrite
}

// TODO: Replace with actual implementation
pub fn GetComparisonFlagCondition(comparison: ComparisonOp) -> FlagsCondition {
  FlagsCondition::kEqual
}

pub struct InstructionCode(u32);

impl InstructionCode {
  pub fn new(value: u32) -> Self {
    InstructionCode(value)
  }
}

impl BitOr<ArchOpcode> for InstructionCode {
    type Output = Self;

    fn bitor(self, rhs: ArchOpcode) -> Self {
        InstructionCode(self.0 | (rhs as u32))
    }
}

impl BitOr<AddressingMode> for InstructionCode {
    type Output = Self;

    fn bitor(self, rhs: AddressingMode) -> Self {
        InstructionCode(self.0 | (rhs as u32)) // Placeholder, needs proper encoding
    }
}

impl BitOr<FlagsCondition> for InstructionCode {
    type Output = Self;

    fn bitor(self, rhs: FlagsCondition) -> Self {
        InstructionCode(self.0 | (rhs as u32)) // Placeholder, needs proper encoding
    }
}

impl From<ArchOpcode> for InstructionCode {
    fn from(opcode: ArchOpcode) -> Self {
        InstructionCode(opcode as u32)
    }
}

// Placeholder for CPU feature support
#[allow(dead_code)]
mod CpuFeatures {
    pub fn IsSupported(_feature: Feature) -> bool {
        false
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum Feature {
    DISTINCT_OPS,
    GENERAL_INSTR_EXT,
    MISC_INSTR_EXT2,
}

// Placeholder
fn IsAnyTagged(_rep: MachineRepresentation) -> bool {
  false
}

// Placeholder
fn IsAnyCompressed(_rep: MachineRepresentation) -> bool {
    false
}

// Placeholder
fn CanBeTaggedOrCompressedPointer(_rep: MachineRepresentation) -> bool {
    false
}

// Placeholder
fn ElementSizeInBits(_rep: MachineRepresentation) -> i32 {
    0
}

// Placeholder
pub struct LoadRepresentation {}

impl LoadRepresentation {
    pub fn representation(&self) -> MachineRepresentation {
        MachineRepresentation::kNone
    }

    pub fn IsSigned(&self) -> bool {
        false
    }
}

pub struct StoreRepresentation {}

impl StoreRepresentation {
    pub fn write_barrier_kind(&self) -> WriteBarrierKind {
        WriteBarrierKind::kNoWriteBarrier
    }

    pub fn representation(&self) -> MachineRepresentation {
        MachineRepresentation::kNone
    }
}

// Placeholder
fn CanDoBranchIfOverflowFusion(_node: OpIndex) -> bool {
  false
}

// Placeholder
pub struct CallDescriptor {}

impl CallDescriptor {
  pub fn IsCFunctionCall(&self) -> bool {
    false
  }
}

// Placeholder
pub struct LinkageLocation {}

// Placeholder
pub fn ConsumeEqualZero(user: &mut OpIndex, value: &mut OpIndex, cont: &mut FlagsContinuationT) {

}

// Enums and constants
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum OperandMode {
  kNone = 0,
  // Immediate mode
  kShift32Imm = 1 << 0,
  kShift64Imm = 1 << 1,
  kInt32Imm = 1 << 2,
  kInt32Imm_Negate = 1 << 3,
  kUint32Imm = 1 << 4,
  kInt20Imm = 1 << 5,
  kUint12Imm = 1 << 6,
  // Instr format
  kAllowRRR = 1 << 7,
  kAllowRM = 1 << 8,
  kAllowRI = 1 << 9,
  kAllowRRI = 1 << 10,
  kAllowRRM = 1 << 11,
  // Useful combination
  kAllowImmediate = kAllowRI as u32 | kAllowRRI as u32,
  kAllowMemoryOperand = kAllowRM as u32 | kAllowRRM as u32,
  kAllowDistinctOps = kAllowRRR as u32 | kAllowRRI as u32 | kAllowRRM as u32,
  kBitWiseCommonMode = kAllowRI as u32,
  kArithmeticCommonMode = kAllowRM as u32 | kAllowRI as u32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct OperandModes(u32);

impl OperandModes {
    pub fn contains(&self, mode: OperandMode) -> bool {
        (self.0 & (mode as u32)) != 0
    }
}

impl From<OperandMode> for OperandModes {
    fn from(mode: OperandMode) -> Self {
        OperandModes(mode as u32)
    }
}

impl BitOr for OperandModes {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        OperandModes(self.0 | other.0)
    }
}

impl BitOr<OperandMode> for OperandModes {
    type Output = Self;

    fn bitor(self, other: OperandMode) -> Self {
        OperandModes(self.0 | (other as u32))
    }
}

impl BitAnd for OperandModes {
  type Output = Self;

  fn bitand(self, other: Self) -> Self {
    OperandModes(self.0 & other.0)
  }
}

impl BitAnd<OperandMode> for OperandModes {
  type Output = Self;

  fn bitand(self, other: OperandMode) -> Self {
    OperandModes(self.0 & (other as u32))
  }
}

impl BitXor for OperandModes {
  type Output = Self;

  fn bitxor(self, other: Self) -> Self {
    OperandModes(self.0 ^ other.0)
  }
}

impl BitXor<OperandMode> for OperandModes {
  type Output = Self;

  fn bitxor(self, other: OperandMode) -> Self {
    OperandModes(self.0 ^ (other as u32))
  }
}

macro_rules! define_operators_for_flags {
    ($flags_type:ident, $underlying_type:ty) => {
        impl std::ops::BitOr for $flags_type {
            type Output = Self;

            fn bitor(self, other: Self) -> Self {
                $flags_type(self.0 | other.0)
            }
        }

        impl std::ops::BitAnd for $flags_type {
            type Output = Self;

            fn bitand(self, other: Self) -> Self {
                $flags_type(self.0 & other.0)
            }
        }

        impl std::ops::BitXor for $flags_type {
            type Output = Self;

            fn bitxor(self, other: Self) -> Self {
                $flags_type(self.0 ^ other.0)
            }
        }

        impl $flags_type {
            pub fn contains(&self, other: Self) -> bool {
                (self.0 & other.0) == other.0
            }
        }
    };
}

//define_operators_for_flags!(OperandModes, u32); //This is replaced by impls above.

const immediateModeMask: OperandModes = OperandModes(
  OperandMode::kShift32Imm as u32 | OperandMode::kShift64Imm as u32 |
  OperandMode::kInt32Imm as u32 | OperandMode::kInt32Imm_Negate as u32 |
  OperandMode::kUint32Imm as u32 | OperandMode::kInt20Imm as u32
);

macro_rules! and_common_mode {
  () => {
    OperandMode::kAllowRM as u32 |
    (if CpuFeatures::IsSupported(Feature::DISTINCT_OPS) { OperandMode::kAllowRRR as u32 } else { 0 })
  };
}

macro_rules! and64_operand_mode {
  () => {
    OperandModes(and_common_mode!())
  };
}
const And64OperandMode: OperandModes = and64_operand_mode!();

macro_rules! or64_operand_mode {
  () => {
    OperandModes(and_common_mode!())
  };
}
const Or64OperandMode: OperandModes = or64_operand_mode!();

macro_rules! xor64_operand_mode {
  () => {
    OperandModes(and_common_mode!())
  };
}
const Xor64OperandMode: OperandModes = xor64_operand_mode!();

macro_rules! and32_operand_mode {
  () => {
    OperandModes(and_common_mode!() | OperandMode::kAllowRI as u32 | OperandMode::kUint32Imm as u32)
  };
}
const And32OperandMode: OperandModes = and32_operand_mode!();

macro_rules! or32_operand_mode {
  () => {
    OperandModes(and_common_mode!() | OperandMode::kAllowRI as u32 | OperandMode::kUint32Imm as u32)
  };
}
const Or32OperandMode: OperandModes = or32_operand_mode!();

macro_rules! xor32_operand_mode {
  () => {
    OperandModes(and_common_mode!() | OperandMode::kAllowRI as u32 | OperandMode::kUint32Imm as u32)
  };
}
const Xor32OperandMode: OperandModes = xor32_operand_mode!();

macro_rules! shift32_operand_mode {
  () => {
    OperandModes(OperandMode::kAllowRI as u32 | OperandMode::kShift64Imm as u32 |
      (if CpuFeatures::IsSupported(Feature::DISTINCT_OPS) { (OperandMode::kAllowRRR as u32 | OperandMode::kAllowRRI as u32) } else { 0 }))
  };
}
const Shift32OperandMode: OperandModes = shift32_operand_mode!();

macro_rules! shift64_operand_mode {
  () => {
    OperandModes(OperandMode::kAllowRI as u32 | OperandMode::kShift64Imm as u32 | OperandMode::kAllowRRR as u32 | OperandMode::kAllowRRI as u32)
  };
}
const Shift64OperandMode: OperandModes = shift64_operand_mode!();

macro_rules! add_operand_mode {
  () => {
    OperandModes(OperandMode::kArithmeticCommonMode as u32 | OperandMode::kInt32Imm as u32 |
      (if CpuFeatures::IsSupported(Feature::DISTINCT_OPS) { (OperandMode::kAllowRRR as u32 | OperandMode::kAllowRRI as u32) } else { OperandMode::kArithmeticCommonMode as u32 }))
  };
}
const AddOperandMode: OperandModes = add_operand_mode!();

macro_rules! sub_operand_mode {
  () => {
    OperandModes(OperandMode::kArithmeticCommonMode as u32 | OperandMode::kInt32Imm_Negate as u32 |
      (if CpuFeatures::IsSupported(Feature::DISTINCT_OPS) { (OperandMode::kAllowRRR as u32 | OperandMode::kAllowRRI as u32) } else { OperandMode::kArithmeticCommonMode as u32 }))
  };
}
const SubOperandMode: OperandModes = sub_operand_mode!();

macro_rules! mul_operand_mode {
  () => {
    OperandModes(OperandMode::kArithmeticCommonMode as u32 | OperandMode::kInt32Imm as u32)
  };
}
const MulOperandMode: OperandModes = mul_operand_mode!();

// Structs
#[derive(Debug)]
struct BaseWithScaledIndexAndDisplacementMatch {
