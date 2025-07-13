// Converted from V8 C++ source files:
// Header: instruction-codes.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(non_camel_case_types)]

use std::fmt;

use crate::compiler::WriteBarrierKind;
use crate::codegen::atomic_memory_order::AtomicMemoryOrder;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RecordWriteMode {
    kValueIsMap,
    kValueIsPointer,
    kValueIsIndirectPointer,
    kValueIsEphemeronKey,
    kValueIsAny,
}

pub fn WriteBarrierKindToRecordWriteMode(write_barrier_kind: WriteBarrierKind) -> RecordWriteMode {
    match write_barrier_kind {
        WriteBarrierKind::kMapWriteBarrier => RecordWriteMode::kValueIsMap,
        WriteBarrierKind::kPointerWriteBarrier => RecordWriteMode::kValueIsPointer,
        WriteBarrierKind::kIndirectPointerWriteBarrier => RecordWriteMode::kValueIsIndirectPointer,
        WriteBarrierKind::kEphemeronKeyWriteBarrier => RecordWriteMode::kValueIsEphemeronKey,
        WriteBarrierKind::kFullWriteBarrier => RecordWriteMode::kValueIsAny,
        WriteBarrierKind::kNoWriteBarrier => {
            panic!("Should not be passed as argument.");
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ArchOpcode {
    kArchTailCallCodeObject,
    kArchTailCallAddress,
    kArchTailCallWasm,
    kArchTailCallWasmIndirect,
    kArchCallCodeObject,
    kArchCallJSFunction,
    kArchCallWasmFunction,
    kArchCallWasmFunctionIndirect,
    kArchCallBuiltinPointer,
    kArchPrepareCallCFunction,
    kArchSaveCallerRegisters,
    kArchRestoreCallerRegisters,
    kArchCallCFunction,
    kArchCallCFunctionWithFrameState,
    kArchPrepareTailCall,
    kArchJmp,
    kArchBinarySearchSwitch,
    kArchTableSwitch,
    kArchNop,
    kArchAbortCSADcheck,
    kArchDebugBreak,
    kArchComment,
    kArchThrowTerminator,
    kArchDeoptimize,
    kArchRet,
    kArchFramePointer,
    kArchStackPointer,
    kArchSetStackPointer,
    kArchParentFramePointer,
    kArchTruncateDoubleToI,
    kArchStackSlot,
    kArchStackPointerGreaterThan,
    kArchStackCheckOffset,
    kIeee754Float64Acos,
    kIeee754Float64Acosh,
    kIeee754Float64Asin,
    kIeee754Float64Asinh,
    kIeee754Float64Atan,
    kIeee754Float64Atanh,
    kIeee754Float64Atan2,
    kIeee754Float64Cbrt,
    kIeee754Float64Cos,
    kIeee754Float64Cosh,
    kIeee754Float64Exp,
    kIeee754Float64Expm1,
    kIeee754Float64Log,
    kIeee754Float64Log1p,
    kIeee754Float64Log10,
    kIeee754Float64Log2,
    kIeee754Float64Pow,
    kIeee754Float64Sin,
    kIeee754Float64Sinh,
    kIeee754Float64Tan,
    kIeee754Float64Tanh,
    kAtomicExchangeInt8,
    kAtomicExchangeUint8,
    kAtomicExchangeInt16,
    kAtomicExchangeUint16,
    kAtomicExchangeWord32,
    kAtomicCompareExchangeInt8,
    kAtomicCompareExchangeUint8,
    kAtomicCompareExchangeInt16,
    kAtomicCompareExchangeUint16,
    kAtomicCompareExchangeWord32,
    kAtomicAddInt8,
    kAtomicAddUint8,
    kAtomicAddInt16,
    kAtomicAddUint16,
    kAtomicAddWord32,
    kAtomicSubInt8,
    kAtomicSubUint8,
    kAtomicSubInt16,
    kAtomicSubUint16,
    kAtomicSubWord32,
    kAtomicAndInt8,
    kAtomicAndUint8,
    kAtomicAndInt16,
    kAtomicAndUint16,
    kAtomicAndWord32,
    kAtomicOrInt8,
    kAtomicOrUint8,
    kAtomicOrInt16,
    kAtomicOrUint16,
    kAtomicOrWord32,
    kAtomicXorInt8,
    kAtomicXorUint8,
    kAtomicXorInt16,
    kAtomicXorUint16,
    kAtomicXorWord32,
    kArchStoreWithWriteBarrier,
    kArchAtomicStoreWithWriteBarrier,
    kArchStoreIndirectWithWriteBarrier,
    kAtomicLoadInt8,
    kAtomicLoadUint8,
    kAtomicLoadInt16,
    kAtomicLoadUint16,
    kAtomicLoadWord32,
    kAtomicStoreWord8,
    kAtomicStoreWord16,
    kAtomicStoreWord32,
    kLastArchOpcode,
}

impl fmt::Display for ArchOpcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AddressingMode {
    kMode_None,
    kLastAddressingMode,
}

impl fmt::Display for AddressingMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FlagsMode {
    kFlags_none,
    kFlags_branch,
    kFlags_deoptimize,
    kFlags_set,
    kFlags_trap,
    kFlags_select,
    kFlags_conditional_set,
    kFlags_conditional_branch,
}

impl fmt::Display for FlagsMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FlagsCondition {
    kEqual,
    kNotEqual,
    kSignedLessThan,
    kSignedGreaterThanOrEqual,
    kSignedLessThanOrEqual,
    kSignedGreaterThan,
    kUnsignedLessThan,
    kUnsignedGreaterThanOrEqual,
    kUnsignedLessThanOrEqual,
    kUnsignedGreaterThan,
    kFloatLessThanOrUnordered,
    kFloatGreaterThanOrEqual,
    kFloatLessThanOrEqual,
    kFloatGreaterThanOrUnordered,
    kFloatLessThan,
    kFloatGreaterThanOrEqualOrUnordered,
    kFloatLessThanOrEqualOrUnordered,
    kFloatGreaterThan,
    kUnorderedEqual,
    kUnorderedNotEqual,
    kOverflow,
    kNotOverflow,
    kPositiveOrZero,
    kNegative,
    kIsNaN,
    kIsNotNaN,
}

pub const kStackPointerGreaterThanCondition: FlagsCondition = FlagsCondition::kUnsignedGreaterThan;

pub fn NegateFlagsCondition(condition: FlagsCondition) -> FlagsCondition {
    match condition {
        FlagsCondition::kEqual => FlagsCondition::kNotEqual,
        FlagsCondition::kNotEqual => FlagsCondition::kEqual,
        FlagsCondition::kSignedLessThan => FlagsCondition::kSignedGreaterThanOrEqual,
        FlagsCondition::kSignedGreaterThanOrEqual => FlagsCondition::kSignedLessThan,
        FlagsCondition::kSignedLessThanOrEqual => FlagsCondition::kSignedGreaterThan,
        FlagsCondition::kSignedGreaterThan => FlagsCondition::kSignedLessThanOrEqual,
        FlagsCondition::kUnsignedLessThan => FlagsCondition::kUnsignedGreaterThanOrEqual,
        FlagsCondition::kUnsignedGreaterThanOrEqual => FlagsCondition::kUnsignedLessThan,
        FlagsCondition::kUnsignedLessThanOrEqual => FlagsCondition::kUnsignedGreaterThan,
        FlagsCondition::kUnsignedGreaterThan => FlagsCondition::kUnsignedLessThanOrEqual,
        FlagsCondition::kFloatLessThanOrUnordered => FlagsCondition::kFloatGreaterThanOrEqual,
        FlagsCondition::kFloatGreaterThanOrEqual => FlagsCondition::kFloatLessThanOrUnordered,
        FlagsCondition::kFloatLessThanOrEqual => FlagsCondition::kFloatGreaterThanOrUnordered,
        FlagsCondition::kFloatGreaterThanOrUnordered => FlagsCondition::kFloatLessThanOrEqual,
        FlagsCondition::kFloatLessThan => FlagsCondition::kFloatGreaterThanOrEqualOrUnordered,
        FlagsCondition::kFloatGreaterThanOrEqualOrUnordered => FlagsCondition::kFloatLessThan,
        FlagsCondition::kFloatLessThanOrEqualOrUnordered => FlagsCondition::kFloatGreaterThan,
        FlagsCondition::kFloatGreaterThan => FlagsCondition::kFloatLessThanOrEqualOrUnordered,
        FlagsCondition::kUnorderedEqual => FlagsCondition::kUnorderedNotEqual,
        FlagsCondition::kUnorderedNotEqual => FlagsCondition::kUnorderedEqual,
        FlagsCondition::kOverflow => FlagsCondition::kNotOverflow,
        FlagsCondition::kNotOverflow => FlagsCondition::kOverflow,
        FlagsCondition::kPositiveOrZero => FlagsCondition::kNegative,
        FlagsCondition::kNegative => FlagsCondition::kPositiveOrZero,
        FlagsCondition::kIsNaN => FlagsCondition::kIsNotNaN,
        FlagsCondition::kIsNotNaN => FlagsCondition::kIsNaN,
    }
}

pub fn CommuteFlagsCondition(condition: FlagsCondition) -> FlagsCondition {
    match condition {
        FlagsCondition::kSignedLessThan => FlagsCondition::kSignedGreaterThan,
        FlagsCondition::kSignedGreaterThan => FlagsCondition::kSignedLessThan,
        FlagsCondition::kSignedLessThanOrEqual => FlagsCondition::kSignedGreaterThanOrEqual,
        FlagsCondition::kSignedGreaterThanOrEqual => FlagsCondition::kSignedLessThanOrEqual,
        FlagsCondition::kUnsignedLessThan => FlagsCondition::kUnsignedGreaterThan,
        FlagsCondition::kUnsignedGreaterThan => FlagsCondition::kUnsignedLessThan,
        FlagsCondition::kUnsignedLessThanOrEqual => FlagsCondition::kUnsignedGreaterThanOrEqual,
        FlagsCondition::kUnsignedGreaterThanOrEqual => FlagsCondition::kUnsignedLessThanOrEqual,
        FlagsCondition::kFloatLessThan => FlagsCondition::kFloatGreaterThan,
        FlagsCondition::kFloatGreaterThan => FlagsCondition::kFloatLessThan,
        FlagsCondition::kFloatLessThanOrEqual => FlagsCondition::kFloatGreaterThanOrEqual,
        FlagsCondition::kFloatGreaterThanOrEqual => FlagsCondition::kFloatLessThanOrEqual,
        _ => condition,
    }
}

impl fmt::Display for FlagsCondition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MemoryAccessMode {
    kMemoryAccessDirect,
    kMemoryAccessProtectedMemOutOfBounds,
    kMemoryAccessProtectedNullDereference,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AtomicWidth {
    kWord32,
    kWord64,
}

pub fn AtomicWidthSize(width: AtomicWidth) -> usize {
    match width {
        AtomicWidth::kWord32 => 4,
        AtomicWidth::kWord64 => 8,
    }
}

pub const kLazyDeoptOnThrowSentinel: i32 = -1;

pub type InstructionCode = u32;

pub struct ArchOpcodeField {}
impl ArchOpcodeField {
    const kShift: u32 = 0;
    const kWidth: u32 = 9;
    const kMax: ArchOpcode = ArchOpcode::kLastArchOpcode;
    fn is_valid(opcode: ArchOpcode) -> bool {
        opcode as i32 >= 0 && (opcode as i32) <= Self::kMax as i32
    }
}

pub struct AddressingModeField {}
impl AddressingModeField {
    const kShift: u32 = ArchOpcodeField::kShift + ArchOpcodeField::kWidth;
    const kWidth: u32 = 5;
    const kMax: AddressingMode = AddressingMode::kLastAddressingMode;
    fn is_valid(mode: AddressingMode) -> bool {
        mode as i32 >= 0 && (mode as i32) <= Self::kMax as i32
    }
}

pub struct FlagsModeField {}
impl FlagsModeField {
    const kShift: u32 = AddressingModeField::kShift + AddressingModeField::kWidth;
    const kWidth: u32 = 3;
}

pub struct FlagsConditionField {}
impl FlagsConditionField {
    const kShift: u32 = FlagsModeField::kShift + FlagsModeField::kWidth;
    const kWidth: u32 = 5;
}

pub struct AtomicWidthField {}
impl AtomicWidthField {
    const kShift: u32 = FlagsConditionField::kShift + FlagsConditionField::kWidth;
    const kWidth: u32 = 2;
}

pub struct AtomicMemoryOrderField {}
impl AtomicMemoryOrderField {
    const kShift: u32 = AtomicWidthField::kShift + AtomicWidthField::kWidth;
    const kWidth: u32 = 2;
}

pub struct AtomicStoreRecordWriteModeField {}
impl AtomicStoreRecordWriteModeField {
    const kShift: u32 = AtomicMemoryOrderField::kShift + AtomicMemoryOrderField::kWidth;
    const kWidth: u32 = 4;
    const kLastUsedBit: u32 = Self::kShift + Self::kWidth - 1;
}

pub struct RecordWriteModeField {}
impl RecordWriteModeField {
    const kShift: u32 = FlagsConditionField::kShift + FlagsConditionField::kWidth;
    const kWidth: u32 = 3;
}

pub struct LaneSizeField {}
impl LaneSizeField {
    const kShift: u32 = FlagsConditionField::kShift + FlagsConditionField::kWidth;
    const kWidth: u32 = 2;
}

pub struct VectorLengthField {}
impl VectorLengthField {
    const kShift: u32 = LaneSizeField::kShift + LaneSizeField::kWidth;
    const kWidth: u32 = 2;
}

pub struct AccessModeField {}
impl AccessModeField {
    const kShift: u32 = AtomicStoreRecordWriteModeField::kShift + AtomicStoreRecordWriteModeField::kWidth;
    const kWidth: u32 = 2;
}

pub fn HasMemoryAccessMode(opcode: ArchOpcode) -> bool {
    match opcode {
        ArchOpcode::kAtomicExchangeInt8 => true,
        ArchOpcode::kAtomicExchangeUint8 => true,
        ArchOpcode::kAtomicExchangeInt16 => true,
        ArchOpcode::kAtomicExchangeUint16 => true,
        ArchOpcode::kAtomicExchangeWord32 => true,
        ArchOpcode::kAtomicCompareExchangeInt8 => true,
        ArchOpcode::kAtomicCompareExchangeUint8 => true,
        ArchOpcode::kAtomicCompareExchangeInt16 => true,
        ArchOpcode::kAtomicCompareExchangeUint16 => true,
        ArchOpcode::kAtomicCompareExchangeWord32 => true,
        ArchOpcode::kAtomicAddInt8 => true,
        ArchOpcode::kAtomicAddUint8 => true,
        ArchOpcode::kAtomicAddInt16 => true,
        ArchOpcode::kAtomicAddUint16 => true,
        ArchOpcode::kAtomicAddWord32 => true,
        ArchOpcode::kAtomicSubInt8 => true,
        ArchOpcode::kAtomicSubUint8 => true,
        ArchOpcode::kAtomicSubInt16 => true,
        ArchOpcode::kAtomicSubUint16 => true,
        ArchOpcode::kAtomicSubWord32 => true,
        ArchOpcode::kAtomicAndInt8 => true,
        ArchOpcode::kAtomicAndUint8 => true,
        ArchOpcode::kAtomicAndInt16 => true,
        ArchOpcode::kAtomicAndUint16 => true,
        ArchOpcode::kAtomicAndWord32 => true,
        ArchOpcode::kAtomicOrInt8 => true,
        ArchOpcode::kAtomicOrUint8 => true,
        ArchOpcode::kAtomicOrInt16 => true,
        ArchOpcode::kAtomicOrUint16 => true,
        ArchOpcode::kAtomicOrWord32 => true,
        ArchOpcode::kAtomicXorInt8 => true,
        ArchOpcode::kAtomicXorUint8 => true,
        ArchOpcode::kAtomicXorInt16 => true,
        ArchOpcode::kAtomicXorUint16 => true,
        ArchOpcode::kAtomicXorWord32 => true,
        ArchOpcode::kArchStoreWithWriteBarrier => true,
        ArchOpcode::kArchAtomicStoreWithWriteBarrier => true,
        ArchOpcode::kArchStoreIndirectWithWriteBarrier => true,
        ArchOpcode::kAtomicLoadInt8 => true,
        ArchOpcode::kAtomicLoadUint8 => true,
        ArchOpcode::kAtomicLoadInt16 => true,
        ArchOpcode::kAtomicLoadUint16 => true,
        ArchOpcode::kAtomicLoadWord32 => true,
        ArchOpcode::kAtomicStoreWord8 => true,
        ArchOpcode::kAtomicStoreWord16 => true,
        ArchOpcode::kAtomicStoreWord32 => true,
        _ => false,
    }
}

pub struct DeoptImmedArgsCountField {}
impl DeoptImmedArgsCountField {
    const kShift: u32 = FlagsConditionField::kShift + FlagsConditionField::kWidth;
    const kWidth: u32 = 2;
}

pub struct DeoptFrameStateOffsetField {}
impl DeoptFrameStateOffsetField {
    const kShift: u32 = DeoptImmedArgsCountField::kShift + DeoptImmedArgsCountField::kWidth;
    const kWidth: u32 = 8;
}

pub struct ParamField {}
impl ParamField {
    const kShift: u32 = FlagsConditionField::kShift + FlagsConditionField::kWidth;
    const kWidth: u32 = 5;
}

pub struct FPParamField {}
impl FPParamField {
    const kShift: u32 = ParamField::kShift + ParamField::kWidth;
    const kWidth: u32 = 5;
}

pub struct MiscField {}
impl MiscField {
    const kShift: u32 = FlagsConditionField::kShift + FlagsConditionField::kWidth;
    const kWidth: u32 = 10;
}
