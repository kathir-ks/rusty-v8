// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// Defines a set of operations used in the V8 engine.

macro_rules! define_arithmetic_operations {
    ($($name:ident,)*) => {
        $(
            $name,
        )*
    }
}

macro_rules! define_unary_operations {
    ($($name:ident,)*) => {
        $(
            $name,
        )*
    }
}

macro_rules! define_comparison_operations {
    ($($name:ident,)*) => {
        $(
            $name,
        )*
    }
}

macro_rules! define_operations {
    () => {
        Add,
        Subtract,
        Multiply,
        Divide,
        Modulus,
        Exponentiate,
        BitwiseAnd,
        BitwiseOr,
        BitwiseXor,
        ShiftLeft,
        ShiftRight,
        ShiftRightLogical,
        BitwiseNot,
        Negate,
        Increment,
        Decrement,
        Equal,
        StrictEqual,
        LessThan,
        LessThanOrEqual,
        GreaterThan,
        GreaterThanOrEqual,
    };
}

/// Represents different types of operations.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Exponentiate,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    ShiftLeft,
    ShiftRight,
    ShiftRightLogical,
    BitwiseNot,
    Negate,
    Increment,
    Decrement,
    Equal,
    StrictEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Add => write!(f, "Add"),
            Operation::Subtract => write!(f, "Subtract"),
            Operation::Multiply => write!(f, "Multiply"),
            Operation::Divide => write!(f, "Divide"),
            Operation::Modulus => write!(f, "Modulus"),
            Operation::Exponentiate => write!(f, "Exponentiate"),
            Operation::BitwiseAnd => write!(f, "BitwiseAnd"),
            Operation::BitwiseOr => write!(f, "BitwiseOr"),
            Operation::BitwiseXor => write!(f, "BitwiseXor"),
            Operation::ShiftLeft => write!(f, "ShiftLeft"),
            Operation::ShiftRight => write!(f, "ShiftRight"),
            Operation::ShiftRightLogical => write!(f, "ShiftRightLogical"),
            Operation::BitwiseNot => write!(f, "BitwiseNot"),
            Operation::Negate => write!(f, "Negate"),
            Operation::Increment => write!(f, "Increment"),
            Operation::Decrement => write!(f, "Decrement"),
            Operation::Equal => write!(f, "Equal"),
            Operation::StrictEqual => write!(f, "StrictEqual"),
            Operation::LessThan => write!(f, "LessThan"),
            Operation::LessThanOrEqual => write!(f, "LessThanOrEqual"),
            Operation::GreaterThan => write!(f, "GreaterThan"),
            Operation::GreaterThanOrEqual => write!(f, "GreaterThanOrEqual"),
        }
    }
}