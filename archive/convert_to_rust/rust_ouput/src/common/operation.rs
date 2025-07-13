// Converted from V8 C++ source files:
// Header: operation.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Operation {
    kAdd,
    kSubtract,
    kMultiply,
    kDivide,
    kModulus,
    kExponentiate,
    kBitwiseAnd,
    kBitwiseOr,
    kBitwiseXor,
    kShiftLeft,
    kShiftRight,
    kShiftRightLogical,
    kBitwiseNot,
    kNegate,
    kIncrement,
    kDecrement,
    kEqual,
    kStrictEqual,
    kLessThan,
    kLessThanOrEqual,
    kGreaterThan,
    kGreaterThanOrEqual,
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::kAdd => write!(f, "Add"),
            Operation::kSubtract => write!(f, "Subtract"),
            Operation::kMultiply => write!(f, "Multiply"),
            Operation::kDivide => write!(f, "Divide"),
            Operation::kModulus => write!(f, "Modulus"),
            Operation::kExponentiate => write!(f, "Exponentiate"),
            Operation::kBitwiseAnd => write!(f, "BitwiseAnd"),
            Operation::kBitwiseOr => write!(f, "BitwiseOr"),
            Operation::kBitwiseXor => write!(f, "BitwiseXor"),
            Operation::kShiftLeft => write!(f, "ShiftLeft"),
            Operation::kShiftRight => write!(f, "ShiftRight"),
            Operation::kShiftRightLogical => write!(f, "ShiftRightLogical"),
            Operation::kBitwiseNot => write!(f, "BitwiseNot"),
            Operation::kNegate => write!(f, "Negate"),
            Operation::kIncrement => write!(f, "Increment"),
            Operation::kDecrement => write!(f, "Decrement"),
            Operation::kEqual => write!(f, "Equal"),
            Operation::kStrictEqual => write!(f, "StrictEqual"),
            Operation::kLessThan => write!(f, "LessThan"),
            Operation::kLessThanOrEqual => write!(f, "LessThanOrEqual"),
            Operation::kGreaterThan => write!(f, "GreaterThan"),
            Operation::kGreaterThanOrEqual => write!(f, "GreaterThanOrEqual"),
        }
    }
}
