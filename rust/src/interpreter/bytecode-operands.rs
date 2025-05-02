// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/interpreter/bytecode-operands.rs

use std::fmt;

/// Represents the implicit register use of a bytecode.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ImplicitRegisterUse {
    None,
    ReadAccumulator,
    WriteAccumulator,
    ClobberAccumulator,
    WriteShortStar,
    ReadAndClobberAccumulator,
    ReadWriteAccumulator,
    ReadAccumulatorWriteShortStar,
}

impl fmt::Display for ImplicitRegisterUse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ImplicitRegisterUse::None => "None",
            ImplicitRegisterUse::ReadAccumulator => "ReadAccumulator",
            ImplicitRegisterUse::WriteAccumulator => "WriteAccumulator",
            ImplicitRegisterUse::ClobberAccumulator => "ClobberAccumulator",
            ImplicitRegisterUse::WriteShortStar => "WriteShortStar",
            ImplicitRegisterUse::ReadAndClobberAccumulator => "ReadAndClobberAccumulator",
            ImplicitRegisterUse::ReadWriteAccumulator => "ReadWriteAccumulator",
            ImplicitRegisterUse::ReadAccumulatorWriteShortStar => "ReadAccumulatorWriteShortStar",
        };
        write!(f, "{}", s)
    }
}

/// Represents the type of an operand.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OperandType {
    Flag8,
    UImm8,
    UImm16,
    UImm32,
    Imm8,
    Imm16,
    Imm32,
    ImmSByte,
    Idx8,
    Idx16,
    Idx32,
    Reg8,
    Reg16,
    Reg32,
    RegPair8,
    RegPair16,
    RegPair32,
    RegList8,
    RegList16,
    RegList32,
    BytecodeHandler,
    Intrinsic,
    NativeContextIndex,
    CallFeedbackSlot,
    DebugInfoFlags,
    SmiLiteral,
    HeapObject,
    Undefined,
    Null,
    TheHole,
    Uninitialized,
    True,
    False,
    ContextLocalIndex,
    ConstPoolIndex,
    VectorSlotPair,
}

impl fmt::Display for OperandType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            OperandType::Flag8 => "Flag8",
            OperandType::UImm8 => "UImm8",
            OperandType::UImm16 => "UImm16",
            OperandType::UImm32 => "UImm32",
            OperandType::Imm8 => "Imm8",
            OperandType::Imm16 => "Imm16",
            OperandType::Imm32 => "Imm32",
            OperandType::ImmSByte => "ImmSByte",
            OperandType::Idx8 => "Idx8",
            OperandType::Idx16 => "Idx16",
            OperandType::Idx32 => "Idx32",
            OperandType::Reg8 => "Reg8",
            OperandType::Reg16 => "Reg16",
            OperandType::Reg32 => "Reg32",
            OperandType::RegPair8 => "RegPair8",
            OperandType::RegPair16 => "RegPair16",
            OperandType::RegPair32 => "RegPair32",
            OperandType::RegList8 => "RegList8",
            OperandType::RegList16 => "RegList16",
            OperandType::RegList32 => "RegList32",
            OperandType::BytecodeHandler => "BytecodeHandler",
            OperandType::Intrinsic => "Intrinsic",
            OperandType::NativeContextIndex => "NativeContextIndex",
            OperandType::CallFeedbackSlot => "CallFeedbackSlot",
            OperandType::DebugInfoFlags => "DebugInfoFlags",
            OperandType::SmiLiteral => "SmiLiteral",
            OperandType::HeapObject => "HeapObject",
            OperandType::Undefined => "Undefined",
            OperandType::Null => "Null",
            OperandType::TheHole => "TheHole",
            OperandType::Uninitialized => "Uninitialized",
            OperandType::True => "True",
            OperandType::False => "False",
            OperandType::ContextLocalIndex => "ContextLocalIndex",
            OperandType::ConstPoolIndex => "ConstPoolIndex",
            OperandType::VectorSlotPair => "VectorSlotPair",
        };
        write!(f, "{}", s)
    }
}

/// Represents the scale of an operand.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OperandScale {
    Byte,
    Short,
    Quad,
}

impl fmt::Display for OperandScale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            OperandScale::Byte => "Byte",
            OperandScale::Short => "Short",
            OperandScale::Quad => "Quad",
        };
        write!(f, "{}", s)
    }
}

/// Represents the size of an operand.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OperandSize {
    None,
    Byte,
    Short,
    Quad,
}

impl fmt::Display for OperandSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            OperandSize::None => "None",
            OperandSize::Byte => "Byte",
            OperandSize::Short => "Short",
            OperandSize::Quad => "Quad",
        };
        write!(f, "{}", s)
    }
}