// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// NOTE: This translation is incomplete as it requires substantial parts of the V8 codebase.
//       Many types and functions are assumed or stubbed.
//       This code is intended as a starting point and needs further refinement.

use std::{
    any::Any,
    ops::{BitAnd, BitOr, BitXor},
    ptr::NonNull,
};

//use bitflags::bitflags;
//use num_traits::{PrimInt, Unsigned};

macro_rules! arraysize {
    ($arr:expr) => {
        {
            let arr: &[_; _] = $arr;
            arr.len()
        }
    };
}

macro_rules! UNREACHABLE {
    () => {
        panic!("UNREACHABLE");
    };
}

macro_rules! DCHECK {
    ($cond:expr) => {
        if !($cond) {
            panic!("DCHECK failed: {}", stringify!($cond));
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        if !($cond) {
            panic!("DCHECK failed: {}: {}", stringify!($cond), format_args!($($arg)+));
        }
    };
}

macro_rules! CHECK {
    ($cond:expr) => {
        if !($cond) {
            panic!("CHECK failed: {}", stringify!($cond));
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        if !($cond) {
            panic!("CHECK failed: {}: {}", stringify!($cond), format_args!($($arg)+));
        }
    };
}

macro_rules! IF_WASM {
    ($($tokens:tt)*) => {
        $($tokens)*
    }
}

const COMPRESS_POINTERS_BOOL: bool = false;
const V8_STATIC_ROOTS_BOOL: bool = false;
const V8_COMPRESS_POINTERS: bool = false;
const V8_ENABLE_SANDBOX_BOOL: bool = false;

mod flags {
    pub static mut disable_write_barriers: bool = false;
    pub static mut enable_unconditional_write_barriers: bool = false;
}

const kNumCcmpOperands: usize = 5;

const kCcmpOffsetOfOpcode: usize = 0;
const kCcmpOffsetOfLhs: usize = 1;
const kCcmpOffsetOfRhs: usize = 2;
const kCcmpOffsetOfDefaultFlags: usize = 3;
const kCcmpOffsetOfCompareCondition: usize = 4;

// Stubs and assumptions.  These would need to be fleshed out based on the broader V8 codebase.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum ArchOpcode {
    kArchNop,
    kArchStackSlot,
    kArchAbortCSADcheck,
    kArchTruncateDoubleToI,
    kArchStackPointerGreaterThan,
    kArm64Ldr,
    kArm64LdrW,
    kArm64Ldrb,
    kArm64Ldrh,
    kArm64LdrH,
    kArm64LdrS,
    kArm64LdrD,
    kArm64Ldrsb,
    kArm64LdrsbW,
    kArm64Ldrsh,
    kArm64LdrshW,
    kArm64Ldrsw,
    kArm64Strb,
    kArm64Strh,
    kArm64StrW,
    kArm64Str,
    kArm64StrH,
    kArm64StrS,
    kArm64StrD,
    kArm64Add,
    kArm64Add32,
    kArm64Sub,
    kArm64Sub32,
    kArm64And,
    kArm64And32,
    kArm64Or,
    kArm64Or32,
    kArm64Eor,
    kArm64Eor32,
    kArm64Cmp,
    kArm64Cmp32,
    kArm64Tst,
    kArm64Tst32,
    kArm64Idiv32,
    kArm64Idiv,
    kArm64Udiv32,
    kArm64Udiv,
    kArm64Imod32,
    kArm64Imod,
    kArm64Umod32,
    kArm64Umod,
    kArm64Float32Add,
    kArm64Float64Add,
    kArm64Float32Sub,
    kArm64Float64Sub,
    kArm64Float32Div,
    kArm64Float64Div,
    kArm64Float32Max,
    kArm64Float64Max,
    kArm64Float32Min,
    kArm64Float64Min,
    kArm64Float32Sqrt,
    kArm64Float64Sqrt,
    kArm64Int32ToFloat64,
    kArm64Float64ToInt32,
    kArm64Float32ToFloat64,
    kArm64Int32ToFloat32,
    kArm64Uint32ToFloat32,
    kArm64Int64ToFloat64,
    kArm64Uint32ToFloat64,
    kArm64Float64ToUint32,
    kArm64Float64ToUint64,
    kArm64Int64ToFloat32,
    kArm64Int64ToFloat64,
    kArm64Uint64ToFloat32,
    kArm64Uint64ToFloat64,
    kArm64Float32RoundDown,
    kArm64Float64RoundDown,
    kArm64Float32RoundUp,
    kArm64Float64RoundUp,
    kArm64Float32RoundTruncate,
    kArm64Float64RoundTruncate,
    kArm64Float64RoundTiesAway,
    kArm64Float32RoundTiesEven,
    kArm64Float64RoundTiesEven,
    kArm64Float64SilenceNaN,
    kArm64Float64ExtractLowWord32,
    kArm64Float64ExtractHighWord32,
    kArm64Clz,
    kArm64Clz32,
    kArm64Cnt32,
    kArm64Cnt64,
    kArm64Rbit32,
    kArm64Rbit,
    kArm64Rev32,
    kArm64Rev,
    kArm64Lsl32,
    kArm64Lsl,
    kArm64Lsr32,
    kArm64Lsr,
    kArm64Asr32,
    kArm64Asr,
    kArm64Ror32,
    kArm64Ror,
    kArm64Ubfx32,
    kArm64Ubfx,
    kArm64Sbfx32,
    kArm64Sbfx,
    kArm64Ubfiz32,
    kArm64Bic32,
    kArm64Bic,
    kArm64Orn32,
    kArm64Orn,
    kArm64Eon32,
    kArm64Eon,
    kArm64LoadLane,
    kArm64StoreLane,
    kArm64LoadSplat,
    kArm64S128Load8x8S,
    kArm64S128Load8x8U,
    kArm64S128Load16x4S,
    kArm64S128Load16x4U,
    kArm64S128Load32x2S,
    kArm64S128Load32x2U,
    kArm64LdrS,
    kArm64LdrD,
    kArm64StPair,
    kArm64LdrQ,
    kArm64StrQ,
    kArm64Umull,
    kArm64Smull,
    kArm64Madd,
    kArm64Madd32,
    kArm64Msub,
    kArm64Msub32,
    kArm64Mneg,
    kArm64Mneg32,
    kArm64Cmn32,
    kArm64Cmn,
    kArm64LoadCompressTagged,
    kArm64LdrDecompressTagged,
    kArm64LdrDecompressTaggedSigned,
    kArm64LdrDecompressProtected,
    kArm64StrCompressTagged,
    kArm64StrPair,
    kArm64StrWPair,
    kArm64LdrEncodeSandboxedPointer,
    kArm64LdrDecodeSandboxedPointer,
    kArm64StrEncodeSandboxedPointer,
    kArm64StrIndirectPointer,
    kArchStoreIndirectWithWriteBarrier,
    kArchStoreWithWriteBarrier,
    kFloat16RoundUp,
    kFloat16RoundDown,
    kFloat16RoundTruncate,
    kFloat16RoundTiesEven,
    kI8x16Swizzle,
    kFloat64MoveU64,
    kU64MoveFloat64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum InstructionCode {
    kArchNop,
    kArchStackSlot,
    kArchAbortCSADcheck,
    kArchTruncateDoubleToI,
    kArchStackPointerGreaterThan,
    kArm64Ldr,
    kArm64LdrW,
    kArm64Ldrb,
    kArm64Ldrh,
    kArm64LdrH,
    kArm64LdrS,
    kArm64LdrD,
    kArm64Ldrsb,
    kArm64LdrsbW,
    kArm64Ldrsh,
    kArm64LdrshW,
    kArm64Ldrsw,
    kArm64Strb,
    kArm64Strh,
    kArm64StrW,
    kArm64Str,
    kArm64StrH,
    kArm64StrS,
    kArm64StrD,
    kArm64Add,
    kArm64Add32,
    kArm64Sub,
    kArm64Sub32,
    kArm64And,
    kArm64And32,
    kArm64Or,
    kArm64Or32,
    kArm64Eor,
    kArm64Eor32,
    kArm64Cmp,
    kArm64Cmp32,
    kArm64Tst,
    kArm64Tst32,
    kArm64Idiv32,
    kArm64Idiv,
    kArm64Udiv32,
    kArm64Udiv,
    kArm64Imod32,
    kArm64Imod,
    kArm64Umod32,
    kArm64Umod,
    kArm64Float32Add,
    kArm64Float64Add,
    kArm64Float32Sub,
    kArm64Float64Sub,
    kArm64Float32Div,
    kArm64Float64Div,
    kArm64Float32Max,
    kArm64Float64Max,
    kArm64Float32Min,
    kArm64Float64Min,
    kArm64Float32Sqrt,
    kArm64Float64Sqrt,
    kArm64Int32ToFloat64,
    kArm64Float64ToInt32,
    kArm64Float32ToFloat64,
    kArm64Int32ToFloat32,
    kArm64Uint32ToFloat32,
    kArm64Int64ToFloat64,
    kArm64Uint32ToFloat64,
    kArm64Float64ToUint32,
    kArm64Float64ToUint64,
    kArm64Int64ToFloat32,
    kArm64Int64ToFloat64,
    kArm64Uint64ToFloat32,
    kArm64Uint64ToFloat64,
    kArm64Float32RoundDown,
    kArm64Float64RoundDown,
    kArm64Float32RoundUp,
    kArm64Float64RoundUp,
    kArm64Float32RoundTruncate,
    kArm64Float64RoundTruncate,
    kArm64Float64RoundTiesAway,
    kArm64Float32RoundTiesEven,
    kArm64Float64RoundTiesEven,
    kArm64Float64SilenceNaN,
    kArm64Float64ExtractLowWord32,
    kArm64Float64ExtractHighWord32,
    kArm64Clz,
    kArm64Clz32,
    kArm64Cnt32,
    kArm64Cnt64,
    kArm64Rbit32,
    kArm64Rbit,
    kArm64Rev32,
    kArm64Rev,
    kArm64Lsl32,
    kArm64Lsl,
    kArm64Lsr32,
    kArm64Lsr,
    kArm64Asr32,
    kArm64Asr,
    kArm64Ror32,
    kArm64Ror,
    kArm64Ubfx32,
    kArm64Ubfx,
    kArm64Sbfx32,
    kArm64Sbfx,
    kArm64Ubfiz32,
    kArm64Bic32,
    kArm64Bic,
    kArm64Orn32,
    kArm64Orn,
    kArm64Eon32,
    kArm64Eon,
    kArm64LoadLane,
    kArm64StoreLane,
    kArm64LoadSplat,
    kArm64S128Load8x8S,
    kArm64S128Load8x8U,
    kArm64S128Load16x4S,
    kArm64S128Load16x4U,
    kArm64S128Load32x2S,
    kArm64S128Load32x2U,
    kArm64LdrS,
    kArm64LdrD,
    kArm64StPair,
    kArm64LdrQ,
    kArm64StrQ,
    kArm64Umull,
    kArm64Smull,
    kArm64Madd,
    kArm64Madd32,
    kArm64Msub,
    kArm64Msub32,
    kArm64Mneg,
    kArm64Mneg32,
    kArm64Cmn32,
    kArm64Cmn,
    kArm64LoadCompressTagged,
    kArm64LdrDecompressTagged,
    kArm64LdrDecompressTaggedSigned,
    kArm64LdrDecompressProtected,
    kArm64StrCompressTagged,
    kArm64StrPair,
    kArm64StrWPair,
    kArm64LdrEncodeSandboxedPointer,
    kArm64LdrDecodeSandboxedPointer,
    kArm64StrEncodeSandboxedPointer,
    kArm64StrIndirectPointer,
    kArchStoreIndirectWithWriteBarrier,
    kArchStoreWithWriteBarrier,
    kFloat16RoundUp,
    kFloat16RoundDown,
    kFloat16RoundTruncate,
    kFloat16RoundTiesEven,
    kI8x16Swizzle,
    kFloat64MoveU64,
    kU64MoveFloat64,
    kArm64StPairPost,
    kArm64LdPairPost,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum ImmediateMode {
    kArithmeticImm,  // 12 bit unsigned immediate shifted left 0 or 12 bits
    kShift32Imm,     // 0 - 31
    kShift64Imm,     // 0 - 63
    kLogical32Imm,
    kLogical64Imm,
    kLoadStoreImm8,  // signed 8 bit or 12 bit unsigned scaled by access size
    kLoadStoreImm16,
    kLoadStoreImm32,
    kLoadStoreImm64,
    kConditionalCompareImm,
    kNoImmediate,
}

type OpIndex = usize; // Using usize for OpIndex
type InstructionOperand = usize;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum RegisterRepresentation {
    Word32,
    Word64,
    Float32,
    Float64,
    Tagged,
    Compressed,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum MemoryRepresentation {
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
    ProtectedPointer,
    IndirectPointer,
    SandboxedPointer,
    Simd128,
    Simd256,
    Float16RawBits,
    Bit,
    MapWord,
    None,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum WriteBarrierKind {
    kNoWriteBarrier,
    kFullWriteBarrier,
    kMapWriteBarrier,
    kPointerWriteBarrier,
    kIndirectPointerWriteBarrier,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum RecordWriteMode {
    kNoRecordWrite,
    kRecordWriteWithoutSmiCheck,
    kRecordWriteWithSmiCheck,
}

fn WriteBarrierKindToRecordWriteMode(kind: WriteBarrierKind) -> RecordWriteMode {
    match kind {
        WriteBarrierKind::kNoWriteBarrier => RecordWriteMode::kNoRecordWrite,
        _ => RecordWriteMode::kRecordWriteWithSmiCheck,
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum FlagsCondition {
    kEqual,
    kNotEqual,
    kSignedLessThan,
    kSignedLessThanOrEqual,
    kUnsignedLessThan,
    kUnsignedLessThanOrEqual,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum StackCheckKind {
    kJSFunctionEntry,
    kOther,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum AddressingMode {
    kMode_MRI,
    kMode_MRR,
    kMode_Root,
    kMode_Operand2_R_LSL_I,
    kMode_Operand2_R_LSR_I,
    kMode_Operand2_R_ASR_I,
    kMode_Operand2_R_ROR_I,
    kMode_Operand2_R_UXTB,
    kMode_Operand2_R_UXTH,
    kMode_Operand2_R_SXTW,
    kMode_Operand2_R_SXTB,
    kMode_Operand2_R_SXTH,
}

macro_rules! AddressingModeField {
    ($mode:expr) => {
        $mode as i32
    };
    (encode, $mode:expr) => {
        AddressingModeField!($mode)
    };
    (decode, $value:expr) => {
        $value as AddressingMode
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum MemoryAccessKind {
    kNormalMemoryAccess,
    kProtectedByTrapHandler,
}

impl MemoryAccessKind {
    fn is_protected(&self) -> bool {
        *self == MemoryAccessKind::kProtectedByTrapHandler
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum MemoryAccessProtection {
    kMemoryAccessProtectedMemOutOfBounds,
    kMemoryAccessProtectedNullDereference,
}

macro_rules! AccessModeField {
    ($mode:expr) => {
        $mode as i32
    };
    (encode, $mode:expr) => {
        AccessModeField!($mode)
    };
    (decode, $value:expr) => {
        $value as MemoryAccessProtection
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum LaneSize {
    LaneSize8,
    LaneSize16,
    LaneSize32,
    LaneSize64,
}

macro_rules! LaneSizeField {
    ($size:expr) => {
        $size as i32
    };
    (encode, $size:expr) => {
        LaneSizeField!($size)
    };
    (decode, $value:expr) => {
        $value as LaneSize
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum MachineSemantic {
    kInt32,
    kNumber,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct LoadRepresentation {
    representation_: MachineRepresentation,
    semantic_: MachineSemantic,
    is_unsigned_: bool,
}

impl LoadRepresentation {
    fn representation(&self) -> MachineRepresentation {
        self.representation_
    }

    fn semantic(&self) -> MachineSemantic {
        self.semantic_
    }

    fn IsUnsigned(&self) -> bool {
        self.is_unsigned_
    }
}

macro_rules! MiscField {
    ($misc:expr) => {
        $misc as i32
    };
    (encode, $misc:expr) => {
        MiscField!($misc)
    };
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum IndirectPointerTag {
    kIndirectPointerTag_0,
    kIndirectPointerTag_1,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct MachineType {
    representation: MachineRepresentation,
}

impl MachineType {
    fn representation(&self) -> MachineRepresentation {
        self.representation
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Constant {
    value: i64,
}

impl Constant {
    fn new(value: i64) -> Self {
        Constant { value }
    }
}

fn NegateFlagsCondition(condition: FlagsCondition) -> FlagsCondition {
    match condition {
        FlagsCondition::kEqual => FlagsCondition::kNotEqual,
        FlagsCondition::kNotEqual => FlagsCondition::kEqual,
        FlagsCondition::kSignedLessThan => FlagsCondition::kSignedLessThanOrEqual,
        FlagsCondition::kSignedLessThanOrEqual => FlagsCondition::kSignedLessThan,
        FlagsCondition::kUnsignedLessThan => FlagsCondition::kUnsignedLessThanOrEqual,
        FlagsCondition::kUnsignedLessThanOrEqual => FlagsCondition::kUnsignedLessThan,
    }
}

fn CommuteFlagsCondition(condition: FlagsCondition) -> FlagsCondition {
    match condition {
        FlagsCondition::kSignedLessThan => FlagsCondition::kSignedLessThanOrEqual,
        FlagsCondition::kSignedLessThanOrEqual => FlagsCondition::kSignedLessThan,
        FlagsCondition::kUnsignedLessThan => FlagsCondition::kUnsignedLessThanOrEqual,
        FlagsCondition::kUnsignedLessThanOrEqual => FlagsCondition::kUnsignedLessThan,
        _ => condition,
    }
}

fn IsLoadStoreImmediate(value: i64, size: usize) -> bool {
    // Placeholder implementation, adapt from internal::Assembler::IsImmLSScaled and IsImmLSUnscaled
    value >= -256 && value <= 255
}

fn CanBeTaggedOrCompressedPointer(rep: MachineRepresentation) -> bool {
    rep == MachineRepresentation::AnyTagged
        || rep == MachineRepresentation::TaggedPointer
        || rep == MachineRepresentation::TaggedSigned
        || rep == MachineRepresentation::SandboxedPointer
}

fn CanBeTaggedOrCompressedOrIndirectPointer(rep: MachineRepresentation) -> bool {
    CanBeTaggedOrCompressedPointer(rep) || rep == MachineRepresentation::IndirectPointer
}

fn is_int32(value: i64) -> bool {
    value >= std::i32::MIN as i64 && value <= std::i32::MAX as i64
}

fn is_uint5(value: i64) -> bool {
    value >= 0 && value <= 31
}

fn ElementSizeLog2Of(rep: MachineRepresentation) -> usize {
    match rep {
        MemoryRepresentation::Int8 | MemoryRepresentation::Uint8 => 0,
        MemoryRepresentation::Int16 | MemoryRepresentation::Uint16 | MemoryRepresentation::Float16 => 1,
        MemoryRepresentation::Int32 | MemoryRepresentation::Uint32 | MemoryRepresentation::Float32 => 2,
        MemoryRepresentation::Int64 | MemoryRepresentation::Uint64 | MemoryRepresentation::Float64 => 3,
        MemoryRepresentation::AnyTagged
        | MemoryRepresentation::TaggedPointer
        | MemoryRepresentation::TaggedSigned => {
            if V8_COMPRESS_POINTERS {
                2
            } else {
                3
            }
        },
        _ => unimplemented!("ElementSizeLog2Of for {:?}", rep),
    }
}

// Forward declarations of structs
struct InstructionSelectorT;
struct OperandGeneratorT;

/// Adds Arm64-specific methods for generating operands.
struct Arm64OperandGeneratorT<'a> {
    selector: &'a InstructionSelectorT,
}

impl<'a> Arm64OperandGeneratorT<'a> {
    /// Constructs an `Arm64OperandGeneratorT` with the given `selector`.
    fn new(selector: &'a InstructionSelectorT) -> Self {
        Arm64OperandGeneratorT { selector }
    }

    /// Returns an `InstructionOperand` representing the use of the value of the
    /// specified `node`, either as an immediate value (if possible) or as a
    /// register.
    fn UseOperand(&self, node: OpIndex, mode: ImmediateMode) -> InstructionOperand {
        if self.CanBeImmediate(node, mode) {
            self.UseImmediate(node)
        } else {
            self.UseRegister(node)
        }
    }

    /// Returns true if the value of the specified `node` is an immediate zero.
    fn IsImmediateZero(&self, node: OpIndex) -> bool {
        let operation = self.selector.get(node);
        if let Some(constant) = operation.try_cast::<ConstantOp>() {
            if constant.IsIntegral() && constant.integral() == 0 {
                return true;
            }
            if constant.kind == ConstantOp::Kind::kFloat32 {
                return constant.float32().get_bits() == 0;
            }
            if constant.kind == ConstantOp::Kind::kFloat64 {
                return constant.float64().get_bits() == 0;
            }
        }
        false
    }

    /// Returns an `InstructionOperand` representing the use of the value of the
    /// specified `node`, using the zero register if the node has the immediate
    /// value zero, otherwise assigning a register.
    fn UseRegisterOrImmediateZero(&self, node: OpIndex) -> InstructionOperand {
        if self.IsImmediateZero(node) {
            self.UseImmediate(node)
        } else {
            self.UseRegister(node)
        }
    }

    /// Returns an `InstructionOperand` representing the use of the value of the
    /// specified `node`, using the zero register if the node has the immediate
    /// value zero, otherwise assigning a register, keeping it alive for the whole
    /// sequence of continuation instructions.
    fn UseRegisterAtEndOrImmediateZero(&self, node: OpIndex) -> InstructionOperand {
        if self.IsImmediateZero(node) {
            self.UseImmediate(node)
        } else {
            self.UseRegisterAtEnd(node) // Assuming UseRegisterAtEnd exists in OperandGeneratorT
        }
    }

    /// Returns an `InstructionOperand` representing the use of the specified
    /// `node`, using the provided value if it has the required value, or creating
    /// a TempImmediate otherwise.
    fn UseImmediateOrTemp(&self, node: OpIndex, value: i32) -> InstructionOperand {
        if self
            .selector
            .get(node)
            .cast::<ConstantOp>()
            .signed_integral()
            == value as i64
        {
            self.UseImmediate(node)
        } else {
            self.TempImmediate(value)
        }
    }

    /// Returns true if the specified `node` is an integer constant.
    fn IsIntegerConstant(&self, node: OpIndex) -> bool {
        let mut unused: i64 = 0;
        self.selector.MatchSignedIntegralConstant(node, &mut unused)
    }

    /// Returns an optional i64 if the given `operation` is a signed integral constant.
    fn GetOptionalIntegerConstant(&self, operation: OpIndex) -> Option<i64> {
        let mut constant: i64 = 0;
        if self.selector.MatchSignedIntegralConstant(operation, &mut constant) {
            Some(constant)
        } else {
            None
        }
    }

    /// Returns true if the specified `node` can be represented as an immediate
    /// value, depending on the `mode`.
    fn CanBeImmediate(&self, node: OpIndex, mode: ImmediateMode) -> bool {
        let operation = self.selector.get(node);

        if let Some(constant) = operation.try_cast::<ConstantOp>() {
            if constant.kind == ConstantOp::Kind::kCompressedHeapObject {
                if !COMPRESS_POINTERS_BOOL {
                    return false;
                }
                // For builtin code we need static roots
                // TODO: Port bootstrapper and roots table
                //if self.selector.isolate().bootstrapper() && !V8_STATIC_ROOTS_BOOL {
                //    return false;
                //}

                //TODO: RootsTable logic here
                //const RootsTable& roots_table = selector()->isolate()->roots_table();
                //RootIndex root_index;
                //Handle<HeapObject> value = constant->handle();
                //if (roots_table.IsRootHandle(value, &root_index)) {
                //  if (!RootsTable::IsReadOnly(root_index)) return false;
                //  return CanBeImmediate(MacroAssemblerBase::ReadOnlyRootPtr(
                //                            root_index, selector()->isolate()),
                //                        mode);
                //}
                return false;
            }

            let mut value: i64 = 0;
            self.selector.MatchSignedIntegralConstant(node, &mut value)
                && self.CanBeImmediateValue(value, mode)
        } else {
            false
        }
    }

    /// Returns true if the specified `value` can be represented as an immediate
    /// value, depending on the `mode`.
    fn CanBeImmediateValue(&self, value: i64, mode: ImmediateMode) -> bool {
        let mut ignored: u32 = 0;
        match mode {
            ImmediateMode::kLogical32Imm => {
                // TODO(dcarney): some unencodable values can be handled by
                // switching instructions.
                // TODO: Add Assembler::IsImmLogical implementation
                false //internal::Assembler::IsImmLogical(value as u32, 32, &mut ignored, &mut ignored, &mut ignored)
            }
            ImmediateMode::kLogical64Imm => {
                // TODO: Add Assembler::IsImmLogical implementation
                false //internal::Assembler::IsImmLogical(value as u64, 64, &mut ignored, &mut ignored, &mut ignored)
            }
            ImmediateMode::kArithmeticImm => {
                // TODO: Add Assembler::IsImmAddSub implementation
                false //internal::Assembler::IsImmAddSub(value)
            }
            ImmediateMode::kLoadStoreImm8 => IsLoadStoreImmediate(value, 0),
            ImmediateMode::kLoadStoreImm16 => IsLoadStoreImmediate(value, 1),
            ImmediateMode::kLoadStoreImm32 => IsLoadStoreImmediate(value, 2),
            ImmediateMode::kLoadStoreImm64 => IsLoadStoreImmediate(value, 3),
            ImmediateMode::kNoImmediate => false,
            ImmediateMode::kConditionalCompareImm => {
                // TODO: Add Assembler::IsImmConditionalCompare implementation
                false //internal::Assembler::IsImmConditionalCompare(value)
            }
            ImmediateMode::kShift32Imm | ImmediateMode::kShift64Imm => {
                // Shift operations only observe the bottom 5 or 6 bits of the value.
                // All possible shifts can be encoded by discarding bits which have no
                // effect.
                true
            }
        }
    }

    /// Returns true if the specified `node` can be represented as a shift
    /// immediate value for load/store operations, depending on the `rep`.
    fn CanBeLoadStoreShiftImmediate(&self, node: OpIndex, rep: MachineRepresentation) -> bool {
