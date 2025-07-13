// Converted from V8 C++ source files:
// Header: type-hints.h
// Implementation: type-hints.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod type_hints {
    use std::fmt;
    use std::hash::{Hash, Hasher};

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum BinaryOperationHint {
        kNone,
        kSignedSmall,
        kSignedSmallInputs,
        kAdditiveSafeInteger,
        kNumber,
        kNumberOrOddball,
        kString,
        kStringOrStringWrapper,
        kBigInt,
        kBigInt64,
        kAny,
    }

    impl BinaryOperationHint {
        
    }

    impl fmt::Display for BinaryOperationHint {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                BinaryOperationHint::kNone => write!(f, "None"),
                BinaryOperationHint::kSignedSmall => write!(f, "SignedSmall"),
                BinaryOperationHint::kSignedSmallInputs => write!(f, "SignedSmallInputs"),
                BinaryOperationHint::kAdditiveSafeInteger => write!(f, "AdditiveSafeInteger"),
                BinaryOperationHint::kNumber => write!(f, "Number"),
                BinaryOperationHint::kNumberOrOddball => write!(f, "NumberOrOddball"),
                BinaryOperationHint::kString => write!(f, "String"),
                BinaryOperationHint::kStringOrStringWrapper => write!(f, "StringOrStringWrapper"),
                BinaryOperationHint::kBigInt => write!(f, "BigInt"),
                BinaryOperationHint::kBigInt64 => write!(f, "BigInt64"),
                BinaryOperationHint::kAny => write!(f, "Any"),
            }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum CompareOperationHint {
        kNone,
        kSignedSmall,
        kNumber,
        kNumberOrBoolean,
        kNumberOrOddball,
        kInternalizedString,
        kString,
        kSymbol,
        kBigInt,
        kBigInt64,
        kReceiver,
        kReceiverOrNullOrUndefined,
        kAny,
    }

    impl CompareOperationHint {
        
    }

    impl fmt::Display for CompareOperationHint {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                CompareOperationHint::kNone => write!(f, "None"),
                CompareOperationHint::kSignedSmall => write!(f, "SignedSmall"),
                CompareOperationHint::kNumber => write!(f, "Number"),
                CompareOperationHint::kNumberOrBoolean => write!(f, "NumberOrBoolean"),
                CompareOperationHint::kNumberOrOddball => write!(f, "NumberOrOddball"),
                CompareOperationHint::kInternalizedString => write!(f, "InternalizedString"),
                CompareOperationHint::kString => write!(f, "String"),
                CompareOperationHint::kSymbol => write!(f, "Symbol"),
                CompareOperationHint::kBigInt => write!(f, "BigInt"),
                CompareOperationHint::kBigInt64 => write!(f, "BigInt64"),
                CompareOperationHint::kReceiver => write!(f, "Receiver"),
                CompareOperationHint::kReceiverOrNullOrUndefined => write!(f, "ReceiverOrNullOrUndefined"),
                CompareOperationHint::kAny => write!(f, "Any"),
            }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum ForInHint {
        kNone,
        kEnumCacheKeysAndIndices,
        kEnumCacheKeys,
        kAny,
    }

    impl ForInHint {
        
    }

    impl fmt::Display for ForInHint {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ForInHint::kNone => write!(f, "None"),
                ForInHint::kEnumCacheKeys => write!(f, "EnumCacheKeys"),
                ForInHint::kEnumCacheKeysAndIndices => write!(f, "EnumCacheKeysAndIndices"),
                ForInHint::kAny => write!(f, "Any"),
            }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum StringAddFlags {
        STRING_ADD_CHECK_NONE,
        STRING_ADD_CONVERT_LEFT,
        STRING_ADD_CONVERT_RIGHT,
    }

    impl fmt::Display for StringAddFlags {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                StringAddFlags::STRING_ADD_CHECK_NONE => write!(f, "CheckNone"),
                StringAddFlags::STRING_ADD_CONVERT_LEFT => write!(f, "ConvertLeft"),
                StringAddFlags::STRING_ADD_CONVERT_RIGHT => write!(f, "ConvertRight"),
            }
        }
    }
}
