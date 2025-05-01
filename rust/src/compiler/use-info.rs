// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

pub mod base {
    pub fn hash_combine<T: Hash>(seed: u64, value: T) -> u64 {
        let mut s = std::collections::hash_map::DefaultHasher::new();
        seed.hash(&mut s);
        value.hash(&mut s);
        s.finish()
    }
}

pub mod codegen {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum MachineRepresentation {
        None,
        Word8,
        Word16,
        Word32,
        Word64,
        Float32,
        Float64,
        Simd128,
        Tagged,
        TaggedSigned,
        TaggedPointer,
        Bit,
        Float16,
        Float16RawBits,
    }

    impl fmt::Display for MachineRepresentation {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum MachineType {
        Int8,
        Uint8,
        Int16,
        Uint16,
        Int32,
        Uint32,
        Int64,
        Uint64,
        Float32,
        Float64,
        Simd128,
        Pointer,
        AnyTagged,
        AnyTaggedSigned,
        AnyTaggedPointer,
        Bit,
        Float16,
        Float16RawBits,
    }

    impl MachineType {
        pub fn pointer_representation() -> MachineRepresentation {
            MachineRepresentation::Word64
        }
    }
}

pub mod compiler {
    use super::*;
    use codegen::MachineRepresentation;
    use std::fmt;
    use std::hash::{Hash, Hasher};

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum CheckForMinusZeroMode {
        kCheckForMinusZero,
        kDontCheckForMinusZero,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum IdentifyZeros {
        kIdentifyZeros,
        kDistinguishZeros,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Truncation {
        kind_: TruncationKind,
        identify_zeros_: IdentifyZeros,
    }

    impl Truncation {
        pub fn none() -> Self {
            Truncation {
                kind_: TruncationKind::kNone,
                identify_zeros_: IdentifyZeros::kIdentifyZeros,
            }
        }
        pub fn bool() -> Self {
            Truncation {
                kind_: TruncationKind::kBool,
                identify_zeros_: IdentifyZeros::kIdentifyZeros,
            }
        }
        pub fn word32() -> Self {
            Truncation {
                kind_: TruncationKind::kWord32,
                identify_zeros_: IdentifyZeros::kIdentifyZeros,
            }
        }
        pub fn word64() -> Self {
            Truncation {
                kind_: TruncationKind::kWord64,
                identify_zeros_: IdentifyZeros::kIdentifyZeros,
            }
        }
        pub fn oddball_and_big_int_to_number(
            identify_zeros: IdentifyZeros,
        ) -> Self {
            Truncation {
                kind_: TruncationKind::kOddballAndBigIntToNumber,
                identify_zeros_: identify_zeros,
            }
        }
        pub fn any(identify_zeros: IdentifyZeros) -> Self {
            Truncation {
                kind_: TruncationKind::kAny,
                identify_zeros_: identify_zeros,
            }
        }

        pub fn generalize(t1: Truncation, t2: Truncation) -> Truncation {
            Truncation {
                kind_: Self::generalize_kind(t1.kind(), t2.kind()),
                identify_zeros_: Self::generalize_identify_zeros(
                    t1.identify_zeros(),
                    t2.identify_zeros(),
                ),
            }
        }

        pub fn is_unused(&self) -> bool {
            self.kind_ == TruncationKind::kNone
        }
        pub fn is_used_as_bool(&self) -> bool {
            Self::less_general(self.kind_, TruncationKind::kBool)
        }
        pub fn is_used_as_word32(&self) -> bool {
            Self::less_general(self.kind_, TruncationKind::kWord32)
        }
        pub fn is_used_as_word64(&self) -> bool {
            //DCHECK(Is64());
            Self::less_general(self.kind_, TruncationKind::kWord64)
        }
        pub fn truncates_oddball_and_big_int_to_number(&self) -> bool {
            Self::less_general(self.kind_, TruncationKind::kOddballAndBigIntToNumber)
        }
        pub fn identifies_undefined_and_zero(&self) -> bool {
            Self::less_general(self.kind_, TruncationKind::kWord32)
                || Self::less_general(self.kind_, TruncationKind::kBool)
        }
        pub fn identifies_zero_and_minus_zero(&self) -> bool {
            self.identify_zeros() == IdentifyZeros::kIdentifyZeros
        }

        pub fn description(&self) -> &'static str {
            match self.kind_ {
                TruncationKind::kNone => "None",
                TruncationKind::kBool => "Bool",
                TruncationKind::kWord32 => "Word32",
                TruncationKind::kWord64 => "Word64",
                TruncationKind::kOddballAndBigIntToNumber => "OddballAndBigIntToNumber",
                TruncationKind::kAny => "Any",
            }
        }

        pub fn is_less_general_than(&self, other: Truncation) -> bool {
            Self::less_general(self.kind(), other.kind())
                && Self::less_general_identify_zeros(
                    self.identify_zeros(),
                    other.identify_zeros(),
                )
        }

        pub fn identify_zeros(&self) -> IdentifyZeros {
            self.identify_zeros_
        }

        fn kind(&self) -> TruncationKind {
            self.kind_
        }

        fn generalize_kind(rep1: TruncationKind, rep2: TruncationKind) -> TruncationKind {
            use TruncationKind::*;
            match (rep1, rep2) {
                (kNone, _) => rep2,
                (_, kNone) => rep1,
                (kAny, _) | (_, kAny) => kAny,
                (kOddballAndBigIntToNumber, _) | (_, kOddballAndBigIntToNumber) => kOddballAndBigIntToNumber,
                (kWord64, _) | (_, kWord64) => kWord64,
                (kWord32, _) | (_, kWord32) => kWord32,
                (kBool, _) | (_, kBool) => kBool,
            }
        }

        fn generalize_identify_zeros(i1: IdentifyZeros, i2: IdentifyZeros) -> IdentifyZeros {
            if i1 == IdentifyZeros::kDistinguishZeros || i2 == IdentifyZeros::kDistinguishZeros {
                IdentifyZeros::kDistinguishZeros
            } else {
                IdentifyZeros::kIdentifyZeros
            }
        }

        fn less_general(rep1: TruncationKind, rep2: TruncationKind) -> bool {
            use TruncationKind::*;
            match (rep1, rep2) {
                (kNone, _) => true,
                (_, kNone) => false,
                (kBool, kWord32) | (kBool, kWord64) | (kBool, kOddballAndBigIntToNumber) | (kBool, kAny) => true,
                (kWord32, kWord64) | (kWord32, kOddballAndBigIntToNumber) | (kWord32, kAny) => true,
                (kWord64, kOddballAndBigIntToNumber) | (kWord64, kAny) => true,
                (kOddballAndBigIntToNumber, kAny) => true,
                (_, _) if rep1 == rep2 => true,
                (_, _) => false,
            }
        }

        fn less_general_identify_zeros(u1: IdentifyZeros, u2: IdentifyZeros) -> bool {
            u1 == u2 || u2 == IdentifyZeros::kDistinguishZeros
        }
    }

    impl fmt::Display for Truncation {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.description())
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum TruncationKind {
        kNone,
        kBool,
        kWord32,
        kWord64,
        kOddballAndBigIntToNumber,
        kAny,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum TypeCheckKind {
        kNone,
        kSignedSmall,
        kSigned32,
        kSigned64,
        kAdditiveSafeInteger,
        kNumber,
        kNumberOrBoolean,
        kNumberOrOddball,
        kHeapObject,
        kBigInt,
        kBigInt64,
        kArrayIndex,
    }

    impl fmt::Display for TypeCheckKind {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                TypeCheckKind::kNone => write!(f, "None"),
                TypeCheckKind::kSignedSmall => write!(f, "SignedSmall"),
                TypeCheckKind::kSigned32 => write!(f, "Signed32"),
                TypeCheckKind::kSigned64 => write!(f, "Signed64"),
                TypeCheckKind::kAdditiveSafeInteger => write!(f, "AdditiveSafeInteger"),
                TypeCheckKind::kNumber => write!(f, "Number"),
                TypeCheckKind::kNumberOrBoolean => write!(f, "NumberOrBoolean"),
                TypeCheckKind::kNumberOrOddball => write!(f, "NumberOrOddball"),
                TypeCheckKind::kHeapObject => write!(f, "HeapObject"),
                TypeCheckKind::kBigInt => write!(f, "BigInt"),
                TypeCheckKind::kBigInt64 => write!(f, "BigInt64"),
                TypeCheckKind::kArrayIndex => write!(f, "ArrayIndex"),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct FeedbackSource {}

    impl fmt::Display for FeedbackSource {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "FeedbackSource")
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct UseInfo {
        representation_: MachineRepresentation,
        truncation_: Truncation,
        type_check_: TypeCheckKind,
        feedback_: FeedbackSource,
    }

    impl UseInfo {
        pub fn new(
            representation: MachineRepresentation,
            truncation: Truncation,
            type_check: TypeCheckKind,
            feedback: FeedbackSource,
        ) -> Self {
            UseInfo {
                representation_: representation,
                truncation_: truncation,
                type_check_: type_check,
                feedback_: feedback,
            }
        }

        pub fn truncating_word32() -> Self {
            UseInfo {
                representation_: MachineRepresentation::Word32,
                truncation_: Truncation::word32(),
                type_check_: TypeCheckKind::kNone,
                feedback_: FeedbackSource {},
            }
        }

        pub fn truncating_word64() -> Self {
            UseInfo {
                representation_: MachineRepresentation::Word64,
                truncation_: Truncation::word64(),
                type_check_: TypeCheckKind::kNone,
                feedback_: FeedbackSource {},
            }
        }

        pub fn checked_big_int_truncating_word64(feedback: FeedbackSource) -> Self {
            //DCHECK(Is64());
            UseInfo {
                representation_: MachineRepresentation::Word64,
                truncation_: Truncation::word64(),
                type_check_: TypeCheckKind::kBigInt,
                feedback_: feedback,
            }
        }

        pub fn checked_big_int64_as_word64(feedback: FeedbackSource) -> Self {
            //DCHECK(Is64());
            UseInfo {
                representation_: MachineRepresentation::Word64,
                truncation_: Truncation::any(IdentifyZeros::kDistinguishZeros),
                type_check_: TypeCheckKind::kBigInt64,
                feedback_: feedback,
            }
        }

        pub fn word64(identify_zeros: IdentifyZeros) -> Self {
            UseInfo {
                representation_: MachineRepresentation::Word64,
                truncation_: Truncation::any(identify_zeros),
                type_check_: TypeCheckKind::kNone,
                feedback_: FeedbackSource {},
            }
        }

        pub fn word() -> Self {
            UseInfo {
                representation_: codegen::MachineType::pointer_representation(),
                truncation_: Truncation::any(IdentifyZeros::kDistinguishZeros),
                type_check_: TypeCheckKind::kNone,
                feedback_: FeedbackSource {},
            }
        }

        pub fn bool() -> Self {
            UseInfo {
                representation_: MachineRepresentation::Bit,
                truncation_: Truncation::bool(),
                type_check_: TypeCheckKind::kNone,
                feedback_: FeedbackSource {},
            }
        }

        pub fn float32() -> Self {
            UseInfo {
                representation_: MachineRepresentation::Float32,
                truncation_: Truncation::any(IdentifyZeros::kDistinguishZeros),
                type_check_: TypeCheckKind::kNone,
                feedback_: FeedbackSource {},
            }
        }

        pub fn float16_raw_bits() -> Self {
            UseInfo {
                representation_: MachineRepresentation::Float16RawBits,
                truncation_: Truncation::any(IdentifyZeros::kDistinguishZeros),
                type_check_: TypeCheckKind::kNone,
                feedback_: FeedbackSource {},
            }
        }

        pub fn float64() -> Self {
            UseInfo {
                representation_: MachineRepresentation::Float64,
                truncation_: Truncation::any(IdentifyZeros::kDistinguishZeros),
                type_check_: TypeCheckKind::kNone,
                feedback_: FeedbackSource {},
            }
        }

        pub fn truncating_float64(identify_zeros: IdentifyZeros) -> Self {
            UseInfo {
                representation_: MachineRepresentation::Float64,
                truncation_: Truncation::oddball_and_big_int_to_number(identify_zeros),
                type_check_: TypeCheckKind::kNone,
                feedback_: FeedbackSource {},
            }
        }

        pub fn truncating_float16_raw_bits(identify_zeros: IdentifyZeros) -> Self {
            UseInfo {
                representation_: MachineRepresentation::Float16,
                truncation_: Truncation::oddball_and_big_int_to_number(identify_zeros),
                type_check_: TypeCheckKind::kNone,
                feedback_: FeedbackSource {},
            }
        }

        pub fn checked_safe_int_truncating_word32(feedback: FeedbackSource) -> Self {
            //DCHECK(Is64());
            UseInfo {
                representation_: MachineRepresentation::Word32,
                truncation_: Truncation::word32(),
                type_check_: TypeCheckKind::kAdditiveSafeInteger,
                feedback_: feedback,
            }
        }

        pub fn checked_safe_int_as_word64(feedback: FeedbackSource) -> Self {
            //DCHECK(Is64());
            UseInfo {
                representation_: MachineRepresentation::Word64,
                truncation_: Truncation::any(IdentifyZeros::kDistinguishZeros),
                type_check_: TypeCheckKind::kAdditiveSafeInteger,
                feedback_: feedback,
            }
        }

        pub fn any_tagged() -> Self {
            UseInfo {
                representation_: MachineRepresentation::Tagged,
                truncation_: Truncation::any(IdentifyZeros::kDistinguishZeros),
                type_check_: TypeCheckKind::kNone,
                feedback_: FeedbackSource {},
            }
        }

        pub fn tagged_signed() -> Self {
            UseInfo {
                representation_: MachineRepresentation::TaggedSigned,
                truncation_: Truncation::any(IdentifyZeros::kDistinguishZeros),
                type_check_: TypeCheckKind::kNone,
                feedback_: FeedbackSource {},
            }
        }

        pub fn tagged_pointer() -> Self {
            UseInfo {
                representation_: MachineRepresentation::TaggedPointer,
                truncation_: Truncation::any(IdentifyZeros::kDistinguishZeros),
                type_check_: TypeCheckKind::kNone,
                feedback_: FeedbackSource {},
            }
        }

        pub fn checked_tagged_as_array_index(feedback: FeedbackSource) -> Self {
            UseInfo {
                representation_: codegen::MachineType::pointer_representation(),
                truncation_: Truncation::any(IdentifyZeros::kIdentifyZeros),
                type_check_: TypeCheckKind::kArrayIndex,
                feedback_: feedback,
            }
        }

        pub fn checked_heap_object_as_tagged_pointer(feedback: FeedbackSource) -> Self {
            UseInfo {
                representation_: MachineRepresentation::TaggedPointer,
                truncation_: Truncation::any(IdentifyZeros::kDistinguishZeros),
                type_check_: TypeCheckKind::kHeapObject,
                feedback_: feedback,
            }
        }

        pub fn checked_big_int_as_tagged_pointer(feedback: FeedbackSource) -> Self {
            UseInfo {
                representation_: MachineRepresentation::TaggedPointer,
                truncation_: Truncation::any(IdentifyZeros::kDistinguishZeros),
                type_check_: TypeCheckKind::kBigInt,
                feedback_: feedback,
            }
        }

        pub fn checked_signed_small_as_tagged_signed(
            feedback: FeedbackSource,
            identify_zeros: IdentifyZeros,
        ) -> Self {
            UseInfo {
                representation_: MachineRepresentation::TaggedSigned,
                truncation_: Truncation::any(identify_zeros),
                type_check_: TypeCheckKind::kSignedSmall,
                feedback_: feedback,
            }
        }

        pub fn checked_signed_small_as_word32(
            identify_zeros: IdentifyZeros,
            feedback: FeedbackSource,
        ) -> Self {
            UseInfo {
                representation_: MachineRepresentation::Word32,
                truncation_: Truncation::any(identify_zeros),
                type_check_: TypeCheckKind::kSignedSmall,
                feedback_: feedback,
            }
        }

        pub fn checked_signed32_as_word32(
            identify_zeros: IdentifyZeros,
            feedback: FeedbackSource,
        ) -> Self {
            UseInfo {
                representation_: MachineRepresentation::Word32,
                truncation_: Truncation::any(identify_zeros),
                type_check_: TypeCheckKind::kSigned32,
                feedback_: feedback,
            }
        }

        pub fn checked_signed64_as_word64(
            identify_zeros: IdentifyZeros,
            feedback: FeedbackSource,
        ) -> Self {
            UseInfo {
                representation_: MachineRepresentation::Word64,
                truncation_: Truncation::any(identify_zeros),
                type_check_: TypeCheckKind::kSigned64,
                feedback_: feedback,
            }
        }

        pub fn checked_number_as_float64(
            identify_zeros: IdentifyZeros,
            feedback: FeedbackSource,
        ) -> Self {
            UseInfo {
                representation_: MachineRepresentation::Float64,
                truncation_: Truncation::any(identify_zeros),
                type_check_: TypeCheckKind::kNumber,
                feedback_: feedback,
            }
        }

        pub fn checked_number_as_word32(feedback: FeedbackSource) -> Self {
            UseInfo {
                representation_: MachineRepresentation::Word32,
                truncation_: Truncation::word32(),
                type_check_: TypeCheckKind::kNumber,
                feedback_: feedback,
            }
        }

        pub fn checked_number_or_boolean_as_float64(
            identify_zeros: IdentifyZeros,
            feedback: FeedbackSource,
        ) -> Self {
            UseInfo {
                representation_: MachineRepresentation::Float64,
                truncation_: Truncation::any(identify_zeros),
                type_check_: TypeCheckKind::kNumberOrBoolean,
                feedback_: feedback,
            }
        }

        pub fn checked_number_or_oddball_as_float64(
            identify_zeros: IdentifyZeros,
            feedback: FeedbackSource,
        ) -> Self {
            UseInfo {
                representation_: MachineRepresentation::Float64,
                truncation_: Truncation::any(identify_zeros),
                type_check_: TypeCheckKind::kNumberOrOddball,
                feedback_: feedback,
            }
        }

        pub fn checked_number_or_oddball_as_word32(feedback: FeedbackSource) -> Self {
            UseInfo {
                representation_: MachineRepresentation::Word32,
                truncation_: Truncation::word32(),
                type_check_: TypeCheckKind::kNumberOrOddball,
                feedback_: feedback,
            }
        }

        pub fn any() -> Self {
            UseInfo {
                representation_: MachineRepresentation::None,
                truncation_: Truncation::any(IdentifyZeros::kDistinguishZeros),
                type_check_: TypeCheckKind::kNone,
                feedback_: FeedbackSource {},
            }
        }

        pub fn any_truncating_to_bool() -> Self {
            UseInfo {
                representation_: MachineRepresentation::None,
                truncation_: Truncation::bool(),
                type_check_: TypeCheckKind::kNone,
                feedback_: FeedbackSource {},
            }
        }

        pub fn none() -> Self {
            UseInfo {
                representation_: MachineRepresentation::None,
                truncation_: Truncation::none(),
                type_check_: TypeCheckKind::kNone,
                feedback_: FeedbackSource {},
            }
        }

        pub fn representation(&self) -> MachineRepresentation {
            self.representation_
        }
        pub fn truncation(&self) -> Truncation {
            self.truncation_
        }
        pub fn type_check(&self) -> TypeCheckKind {
            self.type_check_
        }
        pub fn minus_zero_check(&self) -> CheckForMinusZeroMode {
            if self.truncation().identifies_zero_and_minus_zero() {
                CheckForMinusZeroMode::kDontCheckForMinusZero
            } else {
                CheckForMinusZeroMode::kCheckForMinusZero
            }
        }
        pub fn feedback(&self) -> &FeedbackSource {
            &self.feedback_
        }
    }

    impl fmt::Display for UseInfo {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{}, {}, {}, {}",
                self.representation(),
                self.truncation(),
                self.type_check(),
                self.feedback()
            )
        }
    }
}