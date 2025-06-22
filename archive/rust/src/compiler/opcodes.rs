// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/compiler/opcodes.rs

/// Defines the IrOpcode enum and related functions for V8's compiler.
mod opcodes {
    /// An enum representing the different opcodes in V8's intermediate representation.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Value {
        I32Add,
        I32Sub,
        I32Mul,
        I32Div,
        I32Mod,
        I32And,
        I32Or,
        I32Xor,
        I32Shl,
        I32Shr,
        I32Ushr,
        I32Equal,
        I32NotEqual,
        I32LessThan,
        I32LessThanOrEqual,
        I32GreaterThan,
        I32GreaterThanOrEqual,
        I32Clz,
        I32Ctz,
        I32Popcnt,
        I64Add,
        I64Sub,
        I64Mul,
        I64Div,
        I64Mod,
        I64And,
        I64Or,
        I64Xor,
        I64Shl,
        I64Shr,
        I64Ushr,
        I64Equal,
        I64NotEqual,
        I64LessThan,
        I64LessThanOrEqual,
        I64GreaterThan,
        I64GreaterThanOrEqual,
        I64Clz,
        I64Ctz,
        I64Popcnt,
        F32Add,
        F32Sub,
        F32Mul,
        F32Div,
        F32Mod,
        F32Min,
        F32Max,
        F32Abs,
        F32Neg,
        F32Sqrt,
        F32Floor,
        F32Ceil,
        F32Trunc,
        F32RoundTiesEven,
        F64Add,
        F64Sub,
        F64Mul,
        F64Div,
        F64Mod,
        F64Min,
        F64Max,
        F64Abs,
        F64Neg,
        F64Sqrt,
        F64Floor,
        F64Ceil,
        F64Trunc,
        F64RoundTiesEven,
        ChangeFloat32ToFloat64,
        ChangeFloat64ToInt32,
        ChangeFloat64ToUint32,
        ChangeInt32ToFloat64,
        ChangeUint32ToFloat64,
        ChangeInt32ToInt64,
        ChangeUint32ToUint64,
        ChangeInt64ToInt32,
        ChangeUint64ToUint32,
        ChangeInt64ToFloat64,
        ChangeUint64ToFloat64,
        ChangeBoolToBit,
        Phi,
        Select,
        Call,
        Parameter,
        Constant,
        Return,
        Unreachable,
        Deoptimize,
        Load,
        Store,
        StackSlot,
        LoadElement,
        StoreElement,
        LoadField,
        StoreField,
        Poisoned,
        DebugBreak,
        // Add more opcodes as needed.  This is a subset of ALL_OP_LIST from the original C++ code
        // Many opcodes are omitted for brevity in this example.  The real implementation
        // would include all of them.
        Last,
    }

    const K_MNEMONICS: [&str; 79] = [
        "I32Add",
        "I32Sub",
        "I32Mul",
        "I32Div",
        "I32Mod",
        "I32And",
        "I32Or",
        "I32Xor",
        "I32Shl",
        "I32Shr",
        "I32Ushr",
        "I32Equal",
        "I32NotEqual",
        "I32LessThan",
        "I32LessThanOrEqual",
        "I32GreaterThan",
        "I32GreaterThanOrEqual",
        "I32Clz",
        "I32Ctz",
        "I32Popcnt",
        "I64Add",
        "I64Sub",
        "I64Mul",
        "I64Div",
        "I64Mod",
        "I64And",
        "I64Or",
        "I64Xor",
        "I64Shl",
        "I64Shr",
        "I64Ushr",
        "I64Equal",
        "I64NotEqual",
        "I64LessThan",
        "I64LessThanOrEqual",
        "I64GreaterThan",
        "I64GreaterThanOrEqual",
        "I64Clz",
        "I64Ctz",
        "I64Popcnt",
        "F32Add",
        "F32Sub",
        "F32Mul",
        "F32Div",
        "F32Mod",
        "F32Min",
        "F32Max",
        "F32Abs",
        "F32Neg",
        "F32Sqrt",
        "F32Floor",
        "F32Ceil",
        "F32Trunc",
        "F32RoundTiesEven",
        "F64Add",
        "F64Sub",
        "F64Mul",
        "F64Div",
        "F64Mod",
        "F64Min",
        "F64Max",
        "F64Abs",
        "F64Neg",
        "F64Sqrt",
        "F64Floor",
        "F64Ceil",
        "F64Trunc",
        "F64RoundTiesEven",
        "ChangeFloat32ToFloat64",
        "ChangeFloat64ToInt32",
        "ChangeFloat64ToUint32",
        "ChangeInt32ToFloat64",
        "ChangeUint32ToFloat64",
        "ChangeInt32ToInt64",
        "ChangeUint32ToUint64",
        "ChangeInt64ToInt32",
        "ChangeUint64ToUint32",
        "ChangeInt64ToFloat64",
        "ChangeUint64ToFloat64",
        "ChangeBoolToBit",
        "Phi",
        "Select",
        "Call",
        "Parameter",
        "Constant",
        "Return",
        "Unreachable",
        "Deoptimize",
        "Load",
        "Store",
        "StackSlot",
        "LoadElement",
        "StoreElement",
        "LoadField",
        "StoreField",
        "Poisoned",
        "DebugBreak",
    ];

    impl Value {
        /// Returns the mnemonic for the given IrOpcode::Value.
        pub fn mnemonic(&self) -> &'static str {
            let index = *self as usize;
            if index < K_MNEMONICS.len() {
                K_MNEMONICS[index]
            } else {
                "UnknownOpcode"
            }
        }
    }

    impl std::fmt::Display for Value {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.mnemonic())
        }
    }

    /// Represents the IrOpcode.
    pub struct IrOpcode {}

    impl IrOpcode {
        /// Returns the mnemonic for the given IrOpcode::Value.
        pub fn mnemonic(value: Value) -> &'static str {
            value.mnemonic()
        }
    }
}

use opcodes::{IrOpcode, Value};
