// Converted from V8 C++ source files:
// Header: bytecode-flags-and-tokens.h
// Implementation: bytecode-flags-and-tokens.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub struct BitField8<T, const OFFSET: usize, const SIZE: usize> {
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T, const OFFSET: usize, const SIZE: usize> BitField8<T, OFFSET, SIZE> {
        pub fn encode(value: T) -> u8
        where
            T: Into<u8>,
        {
            let value_u8: u8 = value.into();
            (value_u8 << OFFSET) & ((1 << SIZE) - 1 << OFFSET)
        }

        pub fn decode(raw_value: u8) -> T
        where
            T: From<u8>,
        {
            let masked_value = (raw_value >> OFFSET) & ((1 << SIZE) - 1);
            T::from(masked_value)
        }

        pub fn next<U, const NEXT_SIZE: usize>() -> BitField8<U, { OFFSET + SIZE }, NEXT_SIZE> {
            BitField8 {
                _phantom: std::marker::PhantomData,
            }
        }
    }
}

pub mod common {
    pub const kMaxUInt8: usize = 255;
}

pub mod interpreter {
    use super::base;
    use super::LanguageMode;
    use std::marker::PhantomData;

    pub struct CreateArrayLiteralFlags {}

    impl CreateArrayLiteralFlags {
        pub type FlagsBits = base::BitField8<i32, 0, 5>;
        pub type FastCloneSupportedBit = base::BitField8<bool, 5, 1>;

        pub fn encode(use_fast_shallow_clone: bool, runtime_flags: i32) -> u8 {
            let mut result = Self::FlagsBits::encode(runtime_flags);
            result |= Self::FastCloneSupportedBit::encode(use_fast_shallow_clone);
            result
        }
    }

    pub struct CreateObjectLiteralFlags {}

    impl CreateObjectLiteralFlags {
        pub type FlagsBits = base::BitField8<i32, 0, 5>;
        pub type FastCloneSupportedBit = base::BitField8<bool, 5, 1>;

        pub fn encode(runtime_flags: i32, fast_clone_supported: bool) -> u8 {
            let mut result = Self::FlagsBits::encode(runtime_flags);
            result |= Self::FastCloneSupportedBit::encode(fast_clone_supported);
            result
        }
    }

    pub struct CreateClosureFlags {}

    impl CreateClosureFlags {
        pub type PretenuredBit = base::BitField8<bool, 0, 1>;
        pub type FastNewClosureBit = base::BitField8<bool, 1, 1>;

        pub fn encode(pretenure: bool, is_function_scope: bool, might_always_turbofan: bool) -> u8 {
            let mut result = Self::PretenuredBit::encode(pretenure);
            if !might_always_turbofan && !pretenure && is_function_scope {
                result |= Self::FastNewClosureBit::encode(true);
            }
            result
        }
    }

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

    impl From<u8> for LiteralFlag {
        fn from(value: u8) -> Self {
            match value {
                0 => LiteralFlag::Number,
                1 => LiteralFlag::String,
                2 => LiteralFlag::Symbol,
                3 => LiteralFlag::Boolean,
                4 => LiteralFlag::BigInt,
                5 => LiteralFlag::Undefined,
                6 => LiteralFlag::Function,
                7 => LiteralFlag::Object,
                8 => LiteralFlag::Other,
                _ => LiteralFlag::Other,
            }
        }
    }

    impl Into<u8> for LiteralFlag {
        fn into(self) -> u8 {
            match self {
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
    }

    pub struct TestTypeOfFlags {}

    impl TestTypeOfFlags {
        pub fn get_flag_for_literal(
            ast_constants: &AstStringConstants,
            literal: &Literal,
        ) -> LiteralFlag {
            let raw_literal = literal.as_raw_string();
            if raw_literal == ast_constants.number_string() {
                return LiteralFlag::Number;
            } else if raw_literal == ast_constants.string_string() {
                return LiteralFlag::String;
            } else if raw_literal == ast_constants.symbol_string() {
                return LiteralFlag::Symbol;
            } else if raw_literal == ast_constants.boolean_string() {
                return LiteralFlag::Boolean;
            } else if raw_literal == ast_constants.bigint_string() {
                return LiteralFlag::BigInt;
            } else if raw_literal == ast_constants.undefined_string() {
                return LiteralFlag::Undefined;
            } else if raw_literal == ast_constants.function_string() {
                return LiteralFlag::Function;
            } else if raw_literal == ast_constants.object_string() {
                return LiteralFlag::Object;
            } else {
                return LiteralFlag::Other;
            }
        }

        pub fn encode(literal_flag: LiteralFlag) -> u8 {
            literal_flag.into()
        }

        pub fn decode(raw_flag: u8) -> LiteralFlag {
            raw_flag.into()
        }

        pub fn to_string(literal_flag: LiteralFlag) -> &'static str {
            match literal_flag {
                LiteralFlag::Number => "number",
                LiteralFlag::String => "string",
                LiteralFlag::Symbol => "symbol",
                LiteralFlag::Boolean => "boolean",
                LiteralFlag::BigInt => "bigint",
                LiteralFlag::Undefined => "undefined",
                LiteralFlag::Function => "function",
                LiteralFlag::Object => "object",
                LiteralFlag::Other => "other",
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum LookupHoistingMode {
        kNormal,
        kLegacySloppy,
    }

    pub struct StoreLookupSlotFlags {}

    impl StoreLookupSlotFlags {
        pub type LanguageModeBit = base::BitField8<LanguageMode, 0, 1>;
        pub type LookupHoistingModeBit = base::BitField8<bool, 1, 1>;

        pub fn encode(language_mode: LanguageMode, lookup_hoisting_mode: LookupHoistingMode) -> u8 {
            let mut result = Self::LanguageModeBit::encode(language_mode);
            result |= Self::LookupHoistingModeBit::encode(lookup_hoisting_mode == LookupHoistingMode::kLegacySloppy);
            result
        }

        pub fn get_language_mode(flags: u8) -> LanguageMode {
            Self::LanguageModeBit::decode(flags)
        }

        pub fn is_lookup_hoisting_mode(flags: u8) -> bool {
            Self::LookupHoistingModeBit::decode(flags)
        }
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum TryFinallyContinuationToken {
        kFallthroughToken = -1,
        kRethrowToken = 0,
    }
}

pub struct AstStringConstants {
    number_string: String,
    string_string: String,
    symbol_string: String,
    boolean_string: String,
    bigint_string: String,
    undefined_string: String,
    function_string: String,
    object_string: String,
}

impl AstStringConstants {
    pub fn new() -> AstStringConstants {
        AstStringConstants {
            number_string: "number".to_string(),
            string_string: "string".to_string(),
            symbol_string: "symbol".to_string(),
            boolean_string: "boolean".to_string(),
            bigint_string: "bigint".to_string(),
            undefined_string: "undefined".to_string(),
            function_string: "function".to_string(),
            object_string: "object".to_string(),
        }
    }

    pub fn number_string(&self) -> &String {
        &self.number_string
    }

    pub fn string_string(&self) -> &String {
        &self.string_string
    }

    pub fn symbol_string(&self) -> &String {
        &self.symbol_string
    }

    pub fn boolean_string(&self) -> &String {
        &self.boolean_string
    }

    pub fn bigint_string(&self) -> &String {
        &self.bigint_string
    }

    pub fn undefined_string(&self) -> &String {
        &self.undefined_string
    }

    pub fn function_string(&self) -> &String {
        &self.function_string
    }

    pub fn object_string(&self) -> &String {
        &self.object_string
    }
}

pub struct Literal {
    raw_string: String,
}

impl Literal {
    pub fn new(raw_string: String) -> Literal {
        Literal { raw_string }
    }

    pub fn as_raw_string(&self) -> &String {
        &self.raw_string
    }
}
