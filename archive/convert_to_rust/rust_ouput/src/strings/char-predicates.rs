// Converted from V8 C++ source files:
// Header: char-predicates.h
// Implementation: char-predicates.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/strings/char-predicates.h
pub mod char_predicates {
    use crate::base::strings::uc32;

    pub const fn AsciiAlphaToLower(c: uc32) -> i32 {
        if c >= 'A' as uc32 && c <= 'Z' as uc32 {
            (c + 32) as i32
        } else {
            c as i32
        }
    }

    pub const fn IsCarriageReturn(c: uc32) -> bool {
        c == 0x0D
    }

    pub const fn IsLineFeed(c: uc32) -> bool {
        c == 0x0A
    }

    pub const fn IsAsciiIdentifier(c: uc32) -> bool {
        IsAsciiAlpha(c) || IsDecimalDigit(c) || c == 0x24 || c == 0x5F
    }

    pub const fn IsAlphaNumeric(c: uc32) -> bool {
        IsAsciiAlpha(c) || IsDecimalDigit(c)
    }

    pub const fn IsDecimalDigit(c: uc32) -> bool {
        c >= '0' as uc32 && c <= '9' as uc32
    }

    pub const fn IsHexDigit(c: uc32) -> bool {
        IsDecimalDigit(c) || (c >= 'A' as uc32 && c <= 'F' as uc32) || (c >= 'a' as uc32 && c <= 'f' as uc32)
    }

    pub const fn IsOctalDigit(c: uc32) -> bool {
        c >= '0' as uc32 && c <= '7' as uc32
    }

    pub const fn IsBinaryDigit(c: uc32) -> bool {
        c == '0' as uc32 || c == '1' as uc32
    }

    pub const fn IsRegExpWord(c: uc32) -> bool {
        IsAsciiAlpha(c) || IsDecimalDigit(c) || c == 0x5F
    }

    pub const fn IsAsciiLower(ch: uc32) -> bool {
        ch >= 'a' as uc32 && ch <= 'z' as uc32
    }

    pub const fn IsAsciiUpper(ch: uc32) -> bool {
        ch >= 'A' as uc32 && ch <= 'Z' as uc32
    }

    pub const fn ToAsciiUpper(ch: uc32) -> uc32 {
        if IsAsciiLower(ch) {
            ch - 32
        } else {
            ch
        }
    }

    pub const fn ToAsciiLower(ch: uc32) -> uc32 {
        if IsAsciiUpper(ch) {
            ch + 32
        } else {
            ch
        }
    }
    pub const fn IsAsciiAlpha(c: uc32) -> bool {
        (c >= 'A' as uc32 && c <= 'Z' as uc32) || (c >= 'a' as uc32 && c <= 'z' as uc32)
    }

    // ES#sec-names-and-keywords
    // This includes '_', '$' and '\', and ID_Start according to
    // http://www.unicode.org/reports/tr31/, which consists of categories
    // 'Lu', 'Ll', 'Lt', 'Lm', 'Lo', 'Nl', but excluding properties
    // 'Pattern_Syntax' or 'Pattern_White_Space'.
    pub fn IsIdentifierStart(c: uc32) -> bool {
        IsIdentifierStartSlow(c)
    }
    #[cfg(feature = "intl")]
    extern "C" {
        fn u_hasBinaryProperty(c: i32, property: i32) -> bool;
        fn u_charType(c: i32) -> i32;
    }
    #[cfg(feature = "intl")]
    pub fn IsIdentifierStartSlow(c: uc32) -> bool {
        unsafe {
            if cfg!(feature = "intl") {
                const UCHAR_ID_START: i32 = 0;
                u_hasBinaryProperty(c as i32, UCHAR_ID_START)
                    || (c < 0x60 && (c == '$' as uc32 || c == '\\' as uc32 || c == '_' as uc32))
            } else {
                (c <= 0xFFFF)
            }
        }
    }

    #[cfg(not(feature = "intl"))]
    pub fn IsIdentifierStartSlow(c: uc32) -> bool {
        (c <= 0xFFFF)
    }

    // ES#sec-names-and-keywords
    // This includes \u200c and \u200d, and ID_Continue according to
    // http://www.unicode.org/reports/tr31/, which consists of ID_Start,
    // the categories 'Mn', 'Mc', 'Nd', 'Pc', but excluding properties
    // 'Pattern_Syntax' or 'Pattern_White_Space'.
    pub fn IsIdentifierPart(c: uc32) -> bool {
        IsIdentifierPartSlow(c)
    }
    #[cfg(feature = "intl")]
    pub fn IsIdentifierPartSlow(c: uc32) -> bool {
        unsafe {
            if cfg!(feature = "intl") {
                const UCHAR_ID_CONTINUE: i32 = 1;
                u_hasBinaryProperty(c as i32, 1)
                    || (c < 0x60 && (c == '$' as uc32 || c == '\\' as uc32 || c == '_' as uc32))
                    || c == 0x200C || c == 0x200D
            } else {
                c <= 0xFFFF
            }
        }
    }
    #[cfg(not(feature = "intl"))]
    pub fn IsIdentifierPartSlow(c: uc32) -> bool {
        c <= 0xFFFF
    }

    // ES6 draft section 11.2
    // This includes all code points of Unicode category 'Zs'.
    // Further included are \u0009, \u000b, \u000c, and \ufeff.
    pub fn IsWhiteSpace(c: uc32) -> bool {
        IsWhiteSpaceSlow(c)
    }
    #[cfg(feature = "intl")]
    pub fn IsWhiteSpaceSlow(c: uc32) -> bool {
        unsafe {
            if cfg!(feature = "intl") {
                const U_SPACE_SEPARATOR: i32 = 12;
                (u_charType(c as i32) == U_SPACE_SEPARATOR)
                    || (c < 0x0D && (c == 0x09 || c == 0x0B || c == 0x0C))
                    || c == 0xFEFF
            } else {
                false
            }
        }
    }
    #[cfg(not(feature = "intl"))]
    pub fn IsWhiteSpaceSlow(c: uc32) -> bool {
        false
    }

    // WhiteSpace and LineTerminator according to ES6 draft section 11.2 and 11.3
    // This includes all the characters with Unicode category 'Z' (= Zs+Zl+Zp)
    // as well as \u0009 - \u000d and \ufeff.
    pub fn IsWhiteSpaceOrLineTerminator(c: uc32) -> bool {
        IsWhiteSpaceOrLineTerminatorSlow(c)
    }
    pub fn IsWhiteSpaceOrLineTerminatorSlow(c: uc32) -> bool {
        IsWhiteSpaceSlow(c) || IsLineTerminator(c)
    }

    pub fn IsLineTerminatorSequence(c: uc32, next: uc32) -> bool {
        c == 0x0D && next == 0x0A
    }

    pub fn IsLineTerminator(c: uc32) -> bool {
        c == 0x0A || c == 0x0D || c == 0x2028 || c == 0x2029
    }
}

// src/strings/char-predicates.cc
#[cfg(feature = "intl")]
extern "C" {
    fn u_hasBinaryProperty(c: i32, property: i32) -> bool;
    fn u_charType(c: i32) -> i32;
}
