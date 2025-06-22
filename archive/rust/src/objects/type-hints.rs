// Copyright 2015 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/objects/type-hints.h equivalent
pub mod type_hints {
    use std::fmt;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum BinaryOperationHint {
        None,
        SignedSmall,
        SignedSmallInputs,
        AdditiveSafeInteger,
        Number,
        NumberOrOddball,
        String,
        StringOrStringWrapper,
        BigInt,
        BigInt64,
        Any,
    }

    impl fmt::Display for BinaryOperationHint {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                BinaryOperationHint::None => write!(f, "None"),
                BinaryOperationHint::SignedSmall => write!(f, "SignedSmall"),
                BinaryOperationHint::SignedSmallInputs => write!(f, "SignedSmallInputs"),
                BinaryOperationHint::AdditiveSafeInteger => write!(f, "AdditiveSafeInteger"),
                BinaryOperationHint::Number => write!(f, "Number"),
                BinaryOperationHint::NumberOrOddball => write!(f, "NumberOrOddball"),
                BinaryOperationHint::String => write!(f, "String"),
                BinaryOperationHint::StringOrStringWrapper => write!(f, "StringOrStringWrapper"),
                BinaryOperationHint::BigInt => write!(f, "BigInt"),
                BinaryOperationHint::BigInt64 => write!(f, "BigInt64"),
                BinaryOperationHint::Any => write!(f, "Any"),
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum CompareOperationHint {
        None,
        SignedSmall,
        Number,
        NumberOrBoolean,
        NumberOrOddball,
        InternalizedString,
        String,
        Symbol,
        BigInt,
        BigInt64,
        Receiver,
        ReceiverOrNullOrUndefined,
        Any,
    }

    impl fmt::Display for CompareOperationHint {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                CompareOperationHint::None => write!(f, "None"),
                CompareOperationHint::SignedSmall => write!(f, "SignedSmall"),
                CompareOperationHint::Number => write!(f, "Number"),
                CompareOperationHint::NumberOrBoolean => write!(f, "NumberOrBoolean"),
                CompareOperationHint::NumberOrOddball => write!(f, "NumberOrOddball"),
                CompareOperationHint::InternalizedString => write!(f, "InternalizedString"),
                CompareOperationHint::String => write!(f, "String"),
                CompareOperationHint::Symbol => write!(f, "Symbol"),
                CompareOperationHint::BigInt => write!(f, "BigInt"),
                CompareOperationHint::BigInt64 => write!(f, "BigInt64"),
                CompareOperationHint::Receiver => write!(f, "Receiver"),
                CompareOperationHint::ReceiverOrNullOrUndefined => write!(f, "ReceiverOrNullOrUndefined"),
                CompareOperationHint::Any => write!(f, "Any"),
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ForInHint {
        None,
        EnumCacheKeys,
        EnumCacheKeysAndIndices,
        Any,
    }

    impl fmt::Display for ForInHint {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ForInHint::None => write!(f, "None"),
                ForInHint::EnumCacheKeys => write!(f, "EnumCacheKeys"),
                ForInHint::EnumCacheKeysAndIndices => write!(f, "EnumCacheKeysAndIndices"),
                ForInHint::Any => write!(f, "Any"),
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum StringAddFlags {
        CheckNone,
        ConvertLeft,
        ConvertRight,
    }

    impl fmt::Display for StringAddFlags {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                StringAddFlags::CheckNone => write!(f, "CheckNone"),
                StringAddFlags::ConvertLeft => write!(f, "ConvertLeft"),
                StringAddFlags::ConvertRight => write!(f, "ConvertRight"),
            }
        }
    }
}

// src/objects/type-hints.cc equivalent
pub mod objects {
    pub mod type_hints {
        use std::fmt;
        use super::super::type_hints::*;

        // Ideally, the following `impl` blocks would be located inside of
        // the `type_hints` module above, but Rust doesn't allow you to
        // implement `fmt::Display` on a foreign type (like `std::fmt::Formatter`)
        // from outside of the crate where that type is defined.

        /*
        // The `operator<<` implementations from the C++ code are implemented
        // using the `fmt::Display` trait in Rust.
        // See the `impl fmt::Display for ...` blocks in the `type_hints` module
        // above.
        */

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_binary_operation_hint_display() {
                assert_eq!(format!("{}", BinaryOperationHint::None), "None");
                assert_eq!(format!("{}", BinaryOperationHint::SignedSmall), "SignedSmall");
            }

            #[test]
            fn test_compare_operation_hint_display() {
                assert_eq!(format!("{}", CompareOperationHint::None), "None");
                assert_eq!(format!("{}", CompareOperationHint::String), "String");
            }

            #[test]
            fn test_for_in_hint_display() {
                assert_eq!(format!("{}", ForInHint::None), "None");
                assert_eq!(format!("{}", ForInHint::EnumCacheKeys), "EnumCacheKeys");
            }

            #[test]
            fn test_string_add_flags_display() {
                assert_eq!(format!("{}", StringAddFlags::CheckNone), "CheckNone");
                assert_eq!(format!("{}", StringAddFlags::ConvertLeft), "ConvertLeft");
            }
        }
    }
}