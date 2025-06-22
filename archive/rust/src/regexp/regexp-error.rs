// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// Defines the set of possible regular expression errors.
pub mod regexp_error {
    /// Defines all possible regular expression error messages.
    macro_rules! regexp_error_messages {
        ($T:ident) => {
            $T!(None, "")
            $T!(StackOverflow, "Maximum call stack size exceeded")
            $T!(AnalysisStackOverflow, "Stack overflow")
            $T!(TooLarge, "Regular expression too large")
            $T!(UnterminatedGroup, "Unterminated group")
            $T!(UnmatchedParen, "Unmatched ')'")
            $T!(EscapeAtEndOfPattern, "\\ at end of pattern")
            $T!(InvalidPropertyName, "Invalid property name")
            $T!(InvalidEscape, "Invalid escape")
            $T!(InvalidDecimalEscape, "Invalid decimal escape")
            $T!(InvalidUnicodeEscape, "Invalid Unicode escape")
            $T!(NothingToRepeat, "Nothing to repeat")
            $T!(LoneQuantifierBrackets, "Lone quantifier brackets")
            $T!(RangeOutOfOrder, "numbers out of order in {} quantifier")
            $T!(IncompleteQuantifier, "Incomplete quantifier")
            $T!(InvalidQuantifier, "Invalid quantifier")
            $T!(InvalidGroup, "Invalid group")
            $T!(MultipleFlagDashes, "Multiple dashes in flag group")
            $T!(NotLinear, "Cannot be executed in linear time")
            $T!(RepeatedFlag, "Repeated flag in flag group")
            $T!(InvalidFlagGroup, "Invalid flag group")
            $T!(TooManyCaptures, "Too many captures")
            $T!(InvalidCaptureGroupName, "Invalid capture group name")
            $T!(DuplicateCaptureGroupName, "Duplicate capture group name")
            $T!(InvalidNamedReference, "Invalid named reference")
            $T!(InvalidNamedCaptureReference, "Invalid named capture referenced")
            $T!(InvalidClassPropertyName, "Invalid property name in character class")
            $T!(InvalidCharacterClass, "Invalid character class")
            $T!(UnterminatedCharacterClass, "Unterminated character class")
            $T!(OutOfOrderCharacterClass, "Range out of order in character class")
            $T!(InvalidClassSetOperation, "Invalid set operation in character class")
            $T!(InvalidCharacterInClass, "Invalid character in character class")
            $T!(NegatedCharacterClassWithStrings,
                "Negated character class may contain strings")
        };
    }

    /// Represents the different kinds of regular expression errors.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(u32)]
    pub enum RegExpError {
        #[allow(dead_code)]
        None,
        StackOverflow,
        AnalysisStackOverflow,
        TooLarge,
        UnterminatedGroup,
        UnmatchedParen,
        EscapeAtEndOfPattern,
        InvalidPropertyName,
        InvalidEscape,
        InvalidDecimalEscape,
        InvalidUnicodeEscape,
        NothingToRepeat,
        LoneQuantifierBrackets,
        RangeOutOfOrder,
        IncompleteQuantifier,
        InvalidQuantifier,
        InvalidGroup,
        MultipleFlagDashes,
        NotLinear,
        RepeatedFlag,
        InvalidFlagGroup,
        TooManyCaptures,
        InvalidCaptureGroupName,
        DuplicateCaptureGroupName,
        InvalidNamedReference,
        InvalidNamedCaptureReference,
        InvalidClassPropertyName,
        InvalidCharacterClass,
        UnterminatedCharacterClass,
        OutOfOrderCharacterClass,
        InvalidClassSetOperation,
        InvalidCharacterInClass,
        NegatedCharacterClassWithStrings,
        NumErrors,
    }

    impl RegExpError {
        pub fn to_string(&self) -> &'static str {
            match self {
                RegExpError::None => "",
                RegExpError::StackOverflow => "Maximum call stack size exceeded",
                RegExpError::AnalysisStackOverflow => "Stack overflow",
                RegExpError::TooLarge => "Regular expression too large",
                RegExpError::UnterminatedGroup => "Unterminated group",
                RegExpError::UnmatchedParen => "Unmatched ')'",
                RegExpError::EscapeAtEndOfPattern => "\\ at end of pattern",
                RegExpError::InvalidPropertyName => "Invalid property name",
                RegExpError::InvalidEscape => "Invalid escape",
                RegExpError::InvalidDecimalEscape => "Invalid decimal escape",
                RegExpError::InvalidUnicodeEscape => "Invalid Unicode escape",
                RegExpError::NothingToRepeat => "Nothing to repeat",
                RegExpError::LoneQuantifierBrackets => "Lone quantifier brackets",
                RegExpError::RangeOutOfOrder => "numbers out of order in {} quantifier",
                RegExpError::IncompleteQuantifier => "Incomplete quantifier",
                RegExpError::InvalidQuantifier => "Invalid quantifier",
                RegExpError::InvalidGroup => "Invalid group",
                RegExpError::MultipleFlagDashes => "Multiple dashes in flag group",
                RegExpError::NotLinear => "Cannot be executed in linear time",
                RegExpError::RepeatedFlag => "Repeated flag in flag group",
                RegExpError::InvalidFlagGroup => "Invalid flag group",
                RegExpError::TooManyCaptures => "Too many captures",
                RegExpError::InvalidCaptureGroupName => "Invalid capture group name",
                RegExpError::DuplicateCaptureGroupName => "Duplicate capture group name",
                RegExpError::InvalidNamedReference => "Invalid named reference",
                RegExpError::InvalidNamedCaptureReference => "Invalid named capture referenced",
                RegExpError::InvalidClassPropertyName => "Invalid property name in character class",
                RegExpError::InvalidCharacterClass => "Invalid character class",
                RegExpError::UnterminatedCharacterClass => "Unterminated character class",
                RegExpError::OutOfOrderCharacterClass => "Range out of order in character class",
                RegExpError::InvalidClassSetOperation => "Invalid set operation in character class",
                RegExpError::InvalidCharacterInClass => "Invalid character in character class",
                RegExpError::NegatedCharacterClassWithStrings =>
                    "Negated character class may contain strings",
                RegExpError::NumErrors => "[Invalid RegExpError]", // Should not occur.
            }
        }
    }

    /// Extern function to get the string representation of a RegExpError.
    // The following extern function requires unsafe code.  It would be preferable to get the string name directly from the enum variant
    // but that would require changes in other parts of the V8 codebase and is outside the scope of this conversion.
    // extern "C" {
    //     pub fn RegExpErrorString(error: RegExpError) -> *const std::os::raw::c_char;
    // }

    /// Returns true if the given error is a stack overflow error.
    #[inline]
    pub const fn regexp_error_is_stack_overflow(error: RegExpError) -> bool {
        error == RegExpError::StackOverflow || error == RegExpError::AnalysisStackOverflow
    }
}