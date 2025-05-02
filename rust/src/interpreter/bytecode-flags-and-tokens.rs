// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/interpreter/bytecode-flags-and-tokens.rs

mod bytecode_flags_and_tokens {
    use bitflags::bitflags;

    /// Flags for `CreateArrayLiteral` bytecode.
    pub struct CreateArrayLiteralFlags {}

    impl CreateArrayLiteralFlags {
        /// Encodes the flags for the `CreateArrayLiteral` bytecode.
        pub fn encode(use_fast_shallow_clone: bool, runtime_flags: i32) -> u8 {
            let mut flags: u8 = 0;

            // Encode runtime_flags in the lower 5 bits
            flags |= (runtime_flags as u8) & 0x1F;

            // Encode fast_clone_supported in bit 5
            if use_fast_shallow_clone {
                flags |= 0x20;
            }

            flags
        }
    }

    /// Flags for `CreateObjectLiteral` bytecode.
    pub struct CreateObjectLiteralFlags {}

    impl CreateObjectLiteralFlags {
        /// Encodes the flags for the `CreateObjectLiteral` bytecode.
        pub fn encode(runtime_flags: i32, fast_clone_supported: bool) -> u8 {
            let mut flags: u8 = 0;

            // Encode runtime_flags in the lower 5 bits
            flags |= (runtime_flags as u8) & 0x1F;

            // Encode fast_clone_supported in bit 5
            if fast_clone_supported {
                flags |= 0x20;
            }

            flags
        }
    }

    /// Flags for `CreateClosure` bytecode.
    pub struct CreateClosureFlags {}

    impl CreateClosureFlags {
        /// Encodes the flags for the `CreateClosure` bytecode.
        pub fn encode(pretenure: bool, is_function_scope: bool, might_always_turbofan: bool) -> u8 {
            let mut flags: u8 = 0;

            if pretenure {
                flags |= 0x01;
            }

            if is_function_scope {
                flags |= 0x02;
            }

            if might_always_turbofan {
                flags |= 0x04;
            }

            flags
        }
    }

    /// Flags for `TestTypeOf` bytecode.
    pub struct TestTypeOfFlags {}

    impl TestTypeOfFlags {
        /// Enum representing the different literal types.
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum LiteralFlag {
            Number,
            String,
            Symbol,
            Boolean,
            BigInt,
            Undefined,
            Function,
            Object,
            Other,
        }

        /// Returns the `LiteralFlag` for a given `Literal`.
        pub fn get_flag_for_literal(_ast_constants: &AstStringConstants, literal: &Literal) -> LiteralFlag {
            match literal {
                Literal::Number(_) => LiteralFlag::Number,
                Literal::String(_) => LiteralFlag::String,
                Literal::Symbol(_) => LiteralFlag::Symbol,
                Literal::Boolean(_) => LiteralFlag::Boolean,
                Literal::BigInt(_) => LiteralFlag::BigInt,
                Literal::Undefined => LiteralFlag::Undefined,
                Literal::Function(_) => LiteralFlag::Function,
                Literal::Object(_) => LiteralFlag::Object,
                Literal::Other => LiteralFlag::Other,
            }
        }

        /// Encodes the `LiteralFlag` into a `u8`.
        pub fn encode(literal_flag: LiteralFlag) -> u8 {
            match literal_flag {
                LiteralFlag::Number => 0,
                LiteralFlag::String => 1,
                LiteralFlag::Symbol => 2,
                LiteralFlag::Boolean => 3,
                LiteralFlag::BigInt => 4,
                LiteralFlag::Undefined => 5,
                LiteralFlag::Function => 6,
                LiteralFlag::Object => 7,
                LiteralFlag::Other => 8,
            }
        }

        /// Decodes a `u8` into a `LiteralFlag`.
        pub fn decode(raw_flag: u8) -> LiteralFlag {
            match raw_flag {
                0 => LiteralFlag::Number,
                1 => LiteralFlag::String,
                2 => LiteralFlag::Symbol,
                3 => LiteralFlag::Boolean,
                4 => LiteralFlag::BigInt,
                5 => LiteralFlag::Undefined,
                6 => LiteralFlag::Function,
                7 => LiteralFlag::Object,
                _ => LiteralFlag::Other,
            }
        }

        /// Returns a string representation of the `LiteralFlag`.
        pub fn to_string(literal_flag: LiteralFlag) -> &'static str {
            match literal_flag {
                LiteralFlag::Number => "Number",
                LiteralFlag::String => "String",
                LiteralFlag::Symbol => "Symbol",
                LiteralFlag::Boolean => "Boolean",
                LiteralFlag::BigInt => "BigInt",
                LiteralFlag::Undefined => "Undefined",
                LiteralFlag::Function => "Function",
                LiteralFlag::Object => "Object",
                LiteralFlag::Other => "Other",
            }
        }
    }

    /// Flags for `StoreLookupSlot` bytecode.
    pub struct StoreLookupSlotFlags {}

    impl StoreLookupSlotFlags {
        /// Encodes the `LanguageMode` and `LookupHoistingMode` into a `u8`.
        pub fn encode(language_mode: LanguageMode, lookup_hoisting_mode: LookupHoistingMode) -> u8 {
            let mut flags: u8 = 0;

            flags |= language_mode as u8;

            if lookup_hoisting_mode == LookupHoistingMode::Hoisting {
                flags |= 0x02;
            }

            flags
        }

        /// Returns the `LanguageMode` from the flags.
        pub fn get_language_mode(flags: u8) -> LanguageMode {
            match flags & 0x01 {
                0 => LanguageMode::Sloppy,
                _ => LanguageMode::Strict,
            }
        }

        /// Checks if the lookup hoisting mode is enabled from the flags.
        pub fn is_lookup_hoisting_mode(flags: u8) -> bool {
            (flags & 0x02) != 0
        }
    }

    /// Enum representing possible values for the try-finally continuation token.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum TryFinallyContinuationToken {
        /// Fallthrough token (-1).
        FallthroughToken = -1,
        /// Rethrow token (0).
        RethrowToken = 0,
    }

    // Dummy implementations of types used in signatures for now
    #[derive(Debug)]
    pub enum Literal {
        Number(i64),
        String(String),
        Symbol(String),
        Boolean(bool),
        BigInt(i64),
        Undefined,
        Function(String),
        Object(String),
        Other,
    }

    #[derive(Debug)]
    pub struct AstStringConstants {}

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum LanguageMode {
        Sloppy,
        Strict,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum LookupHoistingMode {
        Normal,
        Hoisting,
    }

    const LanguageModeSize: usize = 2;
}

pub use bytecode_flags_and_tokens::*;