// Converted from V8 C++ source files:
// Header: N/A
// Implementation: instruction-selector-s390.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]

use std::mem;

use bitflags::bitflags;

//use crate::base::logging::DCHECK;
//use crate::compiler::backend::code_generator_impl::kStackFrameExtraParamSlot;
//use crate::compiler::backend::instruction_selector::FlagsContinuation;
//use crate::compiler::backend::instruction_selector::InstructionSelectorT;
//use crate::compiler::node::Node;
//use crate::compiler::node::NodeId;
//use crate::execution::frame_constants::kStackFrameExtraParamSlot;

use std::convert::TryFrom;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum AddressingMode {
    kMode_MRI,
    kMode_MRRI,
    kMode_MRR,
    kMode_MR,
    kMode_Root,
}

// Ensure AddressingMode can be formatted as Debug
impl fmt::Display for AddressingMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// #[derive(Debug, PartialEq, Eq, Copy, Clone)]
// pub enum ArchOpcode {
//     kS390_Add64,
// }

// Ensure ArchOpcode can be formatted as Debug
// impl fmt::Display for ArchOpcode {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct OperandMode: u32 {
        const kNone = 0;
        // Immediate mode
        const kShift32Imm = 1 << 0;
        const kShift64Imm = 1 << 1;
        const kInt32Imm = 1 << 2;
        const kInt32Imm_Negate = 1 << 3;
        const kUint32Imm = 1 << 4;
        const kInt20Imm = 1 << 5;
        const kUint12Imm = 1 << 6;
        // Instr format
        const kAllowRRR = 1 << 7;
        const kAllowRM = 1 << 8;
        const kAllowRI = 1 << 9;
        const kAllowRRI = 1 << 10;
        const kAllowRRM = 1 << 11;
        // Useful combination
        const kAllowImmediate = Self::kAllowRI.bits() | Self::kAllowRRI.bits();
        const kAllowMemoryOperand = Self::kAllowRM.bits() | Self::kAllowRRM.bits();
        const kAllowDistinctOps = Self::kAllowRRR.bits() | Self::kAllowRRI.bits() | Self::kAllowRRM.bits();
        const kBitWiseCommonMode = Self::kAllowRI.bits();
        const kArithmeticCommonMode = Self::kAllowRM.bits() | Self::kAllowRI.bits();
    }
}

//type OperandModes = base::Flags<OperandMode, u32>;
// DEFINE_OPERATORS_FOR_FLAGS(OperandModes)
const IMMEDIATE_MODE_MASK: OperandMode = OperandMode::kShift32Imm
    .union(OperandMode::kShift64Imm)
    .union(OperandMode::kInt32Imm)
    .union(OperandMode::kInt32Imm_Negate)
    .union(OperandMode::kUint32Imm)
    .union(OperandMode::kInt20Imm);
//.union(OperandMode::kUint12Imm);

// #define AndCommonMode                                                \
//   ((OperandMode::kAllowRM |
//     (CpuFeatures::IsSupported(DISTINCT_OPS) ? OperandMode::kAllowRRR
//                                             : OperandMode::kNone)))
// #define And64OperandMode AndCommonMode
// #define Or64OperandMode And64OperandMode
// #define Xor64OperandMode And64OperandMode

// #define And32OperandMode \
//   (AndCommonMode | OperandMode::kAllowRI | OperandMode::kUint32Imm)
// #define Or32OperandMode And32OperandMode
// #define Xor32OperandMode And32OperandMode

// #define Shift32OperandMode                                   \
//   ((OperandMode::kAllowRI | OperandMode::kShift64Imm |       \
//     (CpuFeatures::IsSupported(DISTINCT_OPS)                  \
//          ? (OperandMode::kAllowRRR | OperandMode::kAllowRRI) \
//          : OperandMode::kNone)))

// #define Shift64OperandMode                             \
//   ((OperandMode::kAllowRI | OperandMode::kShift64Imm | \
//    OperandMode::kAllowRRR | OperandMode::kAllowRRI))

// #define AddOperandMode                                            \
//   ((OperandMode::kArithmeticCommonMode | OperandMode::kInt32Imm | \
//     (CpuFeatures::IsSupported(DISTINCT_OPS)                       \
//          ? (OperandMode::kAllowRRR | OperandMode::kAllowRRI)      \
//          : OperandMode::kArithmeticCommonMode)))
// #define SubOperandMode                                                   \
//   ((OperandMode::kArithmeticCommonMode | OperandMode::kInt32Imm_Negate | \
//     (CpuFeatures::IsSupported(DISTINCT_OPS)                              \
//          ? (OperandMode::kAllowRRR | OperandMode::kAllowRRI)             \
//          : OperandMode::kArithmeticCommonMode)))
// #define MulOperandMode \
//   (OperandMode::kArithmeticCommonMode | OperandMode::kInt32Imm)

#[derive(Debug, Default, Copy, Clone)]
struct BaseWithScaledIndexAndDisplacementMatch {
    base: OpIndex,
    index: OpIndex,
    scale: i32,
    displacement: i64,
    displacement_mode: DisplacementMode,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum DisplacementMode {
    kPositiveDisplacement,
    kNegativeDisplacement,
}

struct InstructionSelectorT {}

struct OpIndex {}

struct ConstantOp {
    kind: ConstantKind,
    word32: i32,
    word64: i64,
}

impl ConstantOp {
    fn word32(&self) -> i32 {
        self.word32
    }
    fn word64(&self) -> i64 {
        self.word64
    }
}

enum ConstantKind {
    kWord32,
    kWord64,
}

impl ConstantOp {
    fn try_cast(&self) -> Option<&ConstantOp> {
        Some(self)
    }
}

struct WordBinopOp {}

impl WordBinopOp {
    fn is_commutative(&self) -> bool {
        true
    }
}

struct Operation {}

impl Operation {
    fn try_cast<T>(&self) -> Option<&T> {
        None
    }
    fn is<T>(&self) -> bool {
        false
    }
}

enum WordRepresentation {
    Word32,
    Word64,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum RegisterRepresentation {
    Word32,
    Word64,
    Float32,
    Float64,
    Simd128,
    Tagged,
    Compressed,
}

impl WordRepresentation {
    fn map_tagged_to_word(&self) -> RegisterRepresentation {
        RegisterRepresentation::Word32
    }
}

struct LoadOp {}

struct TurboshaftAdapter {}

struct StoreOp {}

enum MachineRepresentation {
    kFloat64,
    kFloat32,
    kWord64,
    kWord32,
    kInt8,
    kUint8,
    kInt16,
    kUint16,
    kSimd128,
    kAnyTagged,
    kTaggedPointer,
    kTaggedSigned,
    kCompressed,
    kCompressedPointer,
    kAnyUncompressedTagged,
    kUncompressedTaggedPointer,
    kUncompressedTaggedSigned,
    kBit,
    kProtectedPointer,
    kIndirectPointer,
    kSandboxedPointer,
    kSimd256,
    kMapWord,
    kFloat16RawBits,
    kFloat16,
    kNone,
}

struct LoadRepresentation {
  representation: MachineRepresentation,
  is_signed: bool,
}

impl LoadRepresentation {
  fn representation(&self) -> MachineRepresentation {
    self.representation
  }
  fn is_signed(&self) -> bool {
    self.is_signed
  }
}

struct Simd128LaneMemoryOp {}

struct LoadView {
    ts_loaded_rep: MemoryRepresentation,
    ts_result_rep: RegisterRepresentation,
}

impl LoadView {
  fn ts_loaded_rep(&self) -> MemoryRepresentation {
    self.ts_loaded_rep
  }
  fn ts_result_rep(&self) -> RegisterRepresentation {
    self.ts_result_rep
  }
}

impl TurboshaftAdapter {
    fn load_view(&self, node: OpIndex) -> LoadView {
        LoadView {
            ts_loaded_rep: MachineRepresentation::kWord32,
            ts_result_rep: RegisterRepresentation::Word32,
        }
    }
}

struct StoreView {}

impl InstructionSelectorT {
    fn get(&self, node: OpIndex) -> Operation {
        Operation {}
    }
    fn input_at(&self, node: OpIndex, index: usize) -> OpIndex {
        OpIndex {}
    }
    fn can_cover(&self, user: OpIndex, input: OpIndex) -> bool {
        true
    }

    fn load_view(&self, node: OpIndex) -> TurboshaftAdapter::LoadView {
        TurboshaftAdapter {}.load_view(node)
    }

  fn store_view(&self, node: OpIndex) -> StoreView {
    StoreView {}
  }

}

fn try_match_base_with_scaled_index_and_displacement64(
    selector: &InstructionSelectorT,
    node: OpIndex,
) -> Result<BaseWithScaledIndexAndDisplacementMatch, String> {
    // The BaseWithIndexAndDisplacementMatcher canonicalizes the order of
    // displacements and scale factors that are used as inputs, so instead of
    // enumerating all possible patterns by brute force, checking for node
    // clusters using the following templates in the following order suffices
    // to find all of the interesting cases (S = index * scale, B = base
    // input, D = displacement input):
    //
    // (S + (B + D))
    // (S + (B + B))
    // (S + D)
    // (S + B)
    // ((S + D) + B)
    // ((S + B) + D)
    // ((B + D) + B)
    // ((B + B) + D)
    // (B + D)
    // (B + B)
    let mut result = BaseWithScaledIndexAndDisplacementMatch::default();
    result.displacement_mode = DisplacementMode::kPositiveDisplacement;

    //TODO
    Err("unimplemented".to_string())
}

struct S390OperandGeneratorT {
  selector: *mut InstructionSelectorT,
}

impl S390OperandGeneratorT {
    fn use_register(&self, node: OpIndex) -> InstructionOperand {
        InstructionOperand {}
    }

    fn can_be_immediate(&self, value: i64, mode: OperandMode) -> bool {
        if mode.contains(OperandMode::kShift32Imm) {
            return value >= 0 && value < 32;
        } else if mode.contains(OperandMode::kShift64Imm) {
            return value >= 0 && value < 64;
        } else if mode.contains(OperandMode::kInt32Imm) {
            return value >= i32::MIN as i64 && value <= i32::MAX as i64;
        } else if mode.contains(OperandMode::kInt32Imm_Negate) {
            return (-value) >= i32::MIN as i64 && (-value) <= i32::MAX as i64;
        } else if mode.contains(OperandMode::kUint32Imm) {
            return value >= 0 && value <= u32::MAX as i64;
        } else if mode.contains(OperandMode::kInt20Imm) {
            return value >= -524288 && value <= 524287;
        } else if mode.contains(OperandMode::kUint12Imm) {
            return value >= 0 && value <= 4095;
        } else {
            return false;
        }
    }

    fn get(&self, base: OpIndex) -> Operation {
      Operation {}
    }
    fn temp_immediate(&self, i: i32) -> InstructionOperand {
        InstructionOperand {}
    }
    fn use_any_except_immediate(&self, right: OpIndex) -> InstructionOperand {
        InstructionOperand {}
    }
  fn use_immediate(&self, right: OpIndex) -> InstructionOperand {
        InstructionOperand {}
    }
    fn DefineSameAsFirst(&self, node: OpIndex) -> InstructionOperand {
        InstructionOperand {}
    }
    fn DefineAsRegister(&self, node: OpIndex) -> InstructionOperand {
        InstructionOperand {}
    }
    fn use_operand(&self, rhs: OpIndex, mode: OperandMode) -> InstructionOperand {
        InstructionOperand {}
    }
    fn no_output(&self) -> InstructionOperand {
        InstructionOperand {}
    }
  fn use_fixed(&self, node: OpIndex, r3: i32) -> InstructionOperand {
        InstructionOperand {}
    }
  fn define_as_fixed(&self, node: OpIndex, d1: i32) -> InstructionOperand {
        InstructionOperand {}
    }
    fn temp_register(&self) -> InstructionOperand {
        InstructionOperand {}
    }
    fn define_as_register(&self, node: OpIndex) -> InstructionOperand {
        InstructionOperand {}
    }
  fn DefineAsUniqueRegister(&self, value: OpIndex) -> InstructionOperand {
        InstructionOperand {}
  }
  fn use_unique_register(&self, value: OpIndex) -> InstructionOperand {
        InstructionOperand {}
  }
  fn Get(&self, left: OpIndex) -> Operation {
        Operation {}
    }
    fn CanBeImmediate(&self, right: OpIndex, mode: OperandMode) -> bool {
      true
    }
    fn use_register_with_mode(&self, value: OpIndex, register_mode: OperandGeneratorRegisterUseKind) -> InstructionOperand {
      InstructionOperand {}
    }
  fn is_load_or_load_immutable(&self, input: OpIndex) -> bool {
        true
    }
    fn can_be_memory_operand(&self, opcode: InstructionCode, node: OpIndex, rhs: OpIndex, effect_level: i32) -> bool {
        true
    }
    fn get_effective_address_memory_operand(&self, right: OpIndex, inputs: &mut [InstructionOperand], input_count: &mut usize, immediate_mode: OperandMode) -> AddressingMode {
        AddressingMode::kMode_MRI
    }
    fn generate_memory_operand_inputs(&self, index: OptionalOpIndex, base: OpIndex, displacement: i64, displacement_mode: DisplacementMode, inputs: &mut [InstructionOperand], input_count: &mut usize, reg_kind: RegisterUseKind) -> AddressingMode {
      AddressingMode::kMode_MRI
    }
    fn value(&self, index: OpIndex) -> OpIndex {
        OpIndex {}
    }
    fn Turboshaft_graph(&self) -> Operation {
      Operation {}
    }
    fn is_live(&self, node: OpIndex) -> bool {
        true
    }
    fn temp_simd128_register(&self) -> InstructionOperand {
      InstructionOperand {}
    }
}

enum Opcode {
  kWordBinop,
  kWordUnary,
  kChange,
  kShift,
  kOverflowCheckedBinop,
  kLoad,
}

enum ChangeKind {
  kExtractLowHalf,
  kExtractHighHalf,
  kBitcast,
  kSignedFloatTruncateOverflowToMin,
  kUnsignedFloatTruncateOverflowToMin
}

enum FloatRepresentation {
    Float64,
    Float32,
}

struct ChangeOp {
  kind: ChangeKind,
  from: FloatRepresentation,
  to: WordRepresentation,
}

impl ChangeOp {
    fn try_cast(&self) -> Option<&ChangeOp> {
        Some(self)
    }
}

enum WordBinopOpKind {
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

struct WordUnaryOp {
  kind: WordUnaryOpKind,
  rep: WordRepresentation,
}

impl WordUnaryOp {
    fn try_cast(&self) -> Option<&WordUnaryOp> {
        Some(self)
    }
}

enum WordUnaryOpKind {
  kCountLeadingZeros,
  kPopCount,
  kSignExtend8,
  kSignExtend16,
}

enum ShiftOpKind {
    kShiftRightArithmetic,
    kShiftRightLogical,
    kShiftLeft,
    kRotateRight,
    kShiftRightArithmeticShiftOutZeros,
}

struct ShiftOp {
  kind: ShiftOpKind,
  rep: WordRepresentation,
}

impl ShiftOp {
  fn rep(&self) -> WordRepresentation {
        self.rep
    }
    fn right(&self) -> OpIndex {
        OpIndex {}
    }
    fn left(&self) -> OpIndex {
        OpIndex {}
    }

    fn try_cast(&self) -> Option<&ShiftOp> {
        Some(self)
    }
}

enum OverflowCheckedBinopOpKind {
  kSignedAdd,
  kSignedSub,
  kSignedMul,
}

struct OverflowCheckedBinopOp {
  kind: OverflowCheckedBinopOpKind,
  rep: WordRepresentation,
}

impl OverflowCheckedBinopOp {
  fn rep(&self) -> WordRepresentation {
        self.rep
    }

    fn try_cast(&self) -> Option<&OverflowCheckedBinopOp> {
        Some(self)
    }
}

struct LoadRootRegisterOp {}

pub struct OptionalOpIndex {}

struct StoreRepresentation {}

enum RecordWriteMode {}

enum WriteBarrierKind {
  kNoWriteBarrier
}

fn can_be_tagged_or_compressed_pointer(rep: MachineRepresentation) -> bool {
    true
}

enum StackCheckKind {
    kJSFunctionEntry,
}

struct StackPointerGreaterThanOp {}

impl TurboshaftAdapter {
    fn simd_shuffle_view(&self, node: OpIndex) -> SimdShuffleView {
        SimdShuffleView {}
    }
}

struct SimdShuffleView {}

impl SimdShuffleView {
    fn input(&self, index: usize) -> OpIndex {
        OpIndex {}
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum ComparisonOpKind {
    kEqual,
    kSignedLessThan,
    kSignedLessThanOrEqual
}

struct ComparisonOp {
    kind: ComparisonOpKind,
    right: i32,
    rep: RegisterRepresentation
}

impl ComparisonOp {
    fn try_cast(&self) -> Option<&ComparisonOp> {
        Some(self)
    }

    fn right(&self) -> i32 {
      self.right
    }
}

fn get_comparison_flag_condition(comparison: &ComparisonOp) -> i32 {
  0
}

struct ProjectionOp {}

impl ProjectionOp {
    fn try_cast(&self) -> Option<&ProjectionOp> {
        Some(self)
    }
    fn input(&self) -> OpIndex {
        OpIndex {}
    }
}

enum Opmask {
    kWord32Sub,
    kWord32BitwiseAnd,
    kWord64Sub,
    kWord64BitwiseAnd,
    kWord32BitwiseOr,
    kWord32BitwiseXor,
    kWord64BitwiseOr,
    kWord64BitwiseXor,
}

fn can_do_branch_if_overflow_fusion(node: OpIndex) -> bool {
    true
}

enum OverflowCheckedUnaryOpKind {
    kAbs,
}

struct OverflowCheckedUnaryOp {
    kind: OverflowCheckedUnaryOpKind,
    rep: WordRepresentation
}

impl OverflowCheckedUnaryOp {
    fn try_cast(&self) -> Option<&OverflowCheckedUnaryOp> {
        Some(self)
    }
}

fn is_uint32(value: i64) -> bool {
    true
}
fn is_int32(value: i64) -> bool {
    true
}
fn is_uint12(value: i64) -> bool {
    true
}
fn is_int20(value: i64) -> bool {
    true
}

enum RegisterUseKind {
  kUseRegister
}

enum FlagsCondition {
  kUnsignedLessThan,
  kUnsignedLessThanOrEqual,
  kEqual,
  kStackPointerGreaterThanCondition,
  kSignedLessThan,
  kSignedLessThanOrEqual,
  kOverflow,
  kNotEqual
}

struct FlagsContinuationT {}

//impl FlagsContinuationT {
    //fn for_set(overflow: FlagsCondition, value: OpIndex) -> FlagsContinuationT {
        //TODO
        //FlagsContinuationT {}
    //}

    //fn is_deoptimize(&self) -> bool {
        //TODO
        //false
    //}
//}

enum ArchOpcode {
  kS390_Add64,
  kS390_Sub64,
  kS390_Abs32,
  kS390_Abs64
}

struct InstructionOperand {}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InstructionCode {}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AtomicStoreParameters {}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AtomicWidth {}
fn select_load_opcode(rep: MemoryRepresentation) -> ArchOpcode {
  ArchOpcode::kS390_Add64
}

struct AtomicRMWOp {
  memory_rep: MemoryRepresentation
}

enum AtomicMemoryOrder {}

enum AccessKind {}

struct DirectHandle<T> {}
impl<T> DirectHandle<T> {
}

mod base {
    pub mod bits {
        pub fn count_population(value: u64) -> i32 {
            0
        }
        pub fn count_leading_zeros32(value: u32) -> i32 {
            0
        }
        pub fn count_trailing_zeros32(value: u32) -> i32 {
            0
        }
        pub fn count_leading_zeros64(value: u64) -> i32 {
            0
        }
        pub fn count_trailing_zeros64(value: u64) -> i32 {
            0
        }
        pub fn is_power_of_two(value: i64) -> bool {
            true
        }
    }
}

struct LinkageLocation {}

pub struct InstructionSequence {}
impl InstructionSequence {
  pub fn AddImmediate(&self, constant: i32) -> i32 {
    0
  }
}
struct Frame {}

impl Frame {
    fn allocate_spill_slot(&self, size: i32, alignment: i32, is_tagged: bool) -> i32 {
        0
    }
}

//mod turboshaft {

  pub mod operations {

    pub struct Simd128ConstantOp {
      pub value: *const u8,
    }

      pub fn cast<T>(&self) -> &Self {
          self
      }
        pub struct Simd128ExtractLaneOp {
          pub lane: i32
        }
        impl Simd128ExtractLaneOp {}
      pub mod opmasks {}

      pub mod opmasks {}

      pub mod opmasks {}

      pub mod opmasks {}

      pub mod opmasks {}

      pub mod opmasks {}
  }

  pub struct Simd128ConstantOp {
    pub value: *const u8
  }
//}
fn sign_extend_word8_to_int32(
    selector: &InstructionSelectorT,
    result: OpIndex,
    input: OpIndex,
  ) -> Result<(), String> {
        Err("unimplemented".to_string())
  }

trait IsInRange<T> {
    fn is_in_range(&self, min: T, max: T) -> bool;
}
impl IsInRange<i32> for i64 {
  fn is_in_range(&self, min: i32, max: i32) -> bool {
    true
  }
}

fn wasm_simd_shuffle_pack4_lanes(shuffle: &[u8]) -> i32 {
  0
}

