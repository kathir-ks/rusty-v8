// Converted from V8 C++ source files:
// Header: char-predicates-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub type uc32 = u32;

    pub fn IsInRange<T: PartialOrd>(c: T, low: T, high: T) -> bool {
        c >= low && c <= high
    }
}

pub mod internal {
    use super::base;

    // If c is in 'A'-'Z' or 'a'-'z', return its lower-case.
    // Else, return something outside of 'A'-'Z' and 'a'-'z'.
    // Note: it ignores LOCALE.
    pub const fn AsciiAlphaToLower(c: base::uc32) -> base::uc32 {
        c | 0x20
    }

    pub const fn IsCarriageReturn(c: base::uc32) -> bool {
        c == 0x000D
    }

    pub const fn IsLineFeed(c: base::uc32) -> bool {
        c == 0x000A
    }

    pub const fn IsAsciiIdentifier(c: base::uc32) -> bool {
        IsAlphaNumeric(c) || c == '$' || c == '_'
    }

    pub const fn IsAlphaNumeric(c: base::uc32) -> bool {
        base::IsInRange(AsciiAlphaToLower(c), 'a' as u32, 'z' as u32) || IsDecimalDigit(c)
    }

    pub const fn IsDecimalDigit(c: base::uc32) -> bool {
        // ECMA-262, 3rd, 7.8.3 (p 16)
        base::IsInRange(c, '0' as u32, '9' as u32)
    }

    pub const fn IsHexDigit(c: base::uc32) -> bool {
        // ECMA-262, 3rd, 7.6 (p 15)
        IsDecimalDigit(c) || base::IsInRange(AsciiAlphaToLower(c), 'a' as u32, 'f' as u32)
    }

    pub const fn IsOctalDigit(c: base::uc32) -> bool {
        // ECMA-262, 6th, 7.8.3
        base::IsInRange(c, '0' as u32, '7' as u32)
    }

    pub const fn IsNonOctalDecimalDigit(c: base::uc32) -> bool {
        base::IsInRange(c, '8' as u32, '9' as u32)
    }

    pub const fn IsBinaryDigit(c: base::uc32) -> bool {
        // ECMA-262, 6th, 7.8.3
        c == '0' as u32 || c == '1' as u32
    }

    pub const fn IsAscii(c: base::uc32) -> bool {
        !(c & !0x7F)
    }

    pub const fn IsAsciiLower(c: base::uc32) -> bool {
        base::IsInRange(c, 'a' as u32, 'z' as u32)
    }

    pub const fn IsAsciiUpper(c: base::uc32) -> bool {
        base::IsInRange(c, 'A' as u32, 'Z' as u32)
    }

    pub const fn ToAsciiUpper(c: base::uc32) -> base::uc32 {
        c & !((IsAsciiLower(c) as u32) << 5)
    }

    pub const fn ToAsciiLower(c: base::uc32) -> base::uc32 {
        c | ((IsAsciiUpper(c) as u32) << 5)
    }

    pub const fn IsRegExpWord(c: base::uc32) -> bool {
        IsAlphaNumeric(c) || c == '_'
    }

    // Constexpr cache table for character flags.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum OneByteCharFlags {
        kIsIdentifierStart = 1 << 0,
        kIsIdentifierPart = 1 << 1,
        kIsWhiteSpace = 1 << 2,
        kIsWhiteSpaceOrLineTerminator = 1 << 3,
        kMaybeLineEnd = 1 << 4,
    }

    // See http://www.unicode.org/Public/UCD/latest/ucd/DerivedCoreProperties.txt
    // ID_Start. Additionally includes '_' and '$'.
    pub const fn IsOneByteIDStart(c: base::uc32) -> bool {
        c == 0x0024 || (c >= 0x0041 && c <= 0x005A) || c == 0x005F ||
            (c >= 0x0061 && c <= 0x007A) || c == 0x00AA || c == 0x00B5 ||
            c == 0x00BA || (c >= 0x00C0 && c <= 0x00D6) ||
            (c >= 0x00D8 && c <= 0x00F6) || (c >= 0x00F8 && c <= 0x00FF)
    }

    // See http://www.unicode.org/Public/UCD/latest/ucd/DerivedCoreProperties.txt
    // ID_Continue. Additionally includes '_' and '$'.
    pub const fn IsOneByteIDContinue(c: base::uc32) -> bool {
        c == 0x0024 || (c >= 0x0030 && c <= 0x0039) || c == 0x005F ||
            (c >= 0x0041 && c <= 0x005A) || (c >= 0x0061 && c <= 0x007A) ||
            c == 0x00AA || c == 0x00B5 || c == 0x00B7 || c == 0x00BA ||
            (c >= 0x00C0 && c <= 0x00D6) || (c >= 0x00D8 && c <= 0x00F6) ||
            (c >= 0x00F8 && c <= 0x00FF)
    }

    pub const fn IsOneByteWhitespace(c: base::uc32) -> bool {
        c == '\t' as u32 || c == '\v' as u32 || c == '\f' as u32 || c == ' ' as u32 || c == 0xA0
    }

    pub const fn BuildOneByteCharFlags(c: base::uc32) -> u8 {
        let mut result: u8 = 0;
        if IsOneByteIDStart(c) || c == '\\' as u32 { result |= OneByteCharFlags::kIsIdentifierStart as u8; }
        if IsOneByteIDContinue(c) || c == '\\' as u32 { result |= OneByteCharFlags::kIsIdentifierPart as u8; }
        if IsOneByteWhitespace(c) {
            result |= OneByteCharFlags::kIsWhiteSpace as u8 | OneByteCharFlags::kIsWhiteSpaceOrLineTerminator as u8;
        }
        if c == '\r' as u32 || c == '\n' as u32 {
            result |= OneByteCharFlags::kIsWhiteSpaceOrLineTerminator as u8 | OneByteCharFlags::kMaybeLineEnd as u8;
        }
        // Add markers to identify 0x2028 and 0x2029.
        if c == 0x2028 || c == 0x2029 {
            result |= OneByteCharFlags::kMaybeLineEnd as u8;
        }
        result
    }

    pub const kOneByteCharFlags: [u8; 256] = {
        let mut flags = [0u8; 256];
        let mut i = 0;
        while i < 256 {
            flags[i] = BuildOneByteCharFlags(i as u32);
            i += 1;
        }
        flags
    };

    pub fn IsIdentifierStart(c: base::uc32) -> bool {
        if !base::IsInRange(c, 0, 255) { return IsIdentifierStartSlow(c); }
        //DCHECK_EQ(IsIdentifierStartSlow(c),
        //            static_cast<bool>(kOneByteCharFlags[c] & kIsIdentifierStart));
        kOneByteCharFlags[c as usize] & OneByteCharFlags::kIsIdentifierStart as u8 != 0
    }

    pub fn IsIdentifierPart(c: base::uc32) -> bool {
        if !base::IsInRange(c, 0, 255) { return IsIdentifierPartSlow(c); }
        //DCHECK_EQ(IsIdentifierPartSlow(c),
        //            static_cast<bool>(kOneByteCharFlags[c] & kIsIdentifierPart));
        kOneByteCharFlags[c as usize] & OneByteCharFlags::kIsIdentifierPart as u8 != 0
    }

    pub fn IsWhiteSpace(c: base::uc32) -> bool {
        if !base::IsInRange(c, 0, 255) { return IsWhiteSpaceSlow(c); }
        //DCHECK_EQ(IsWhiteSpaceSlow(c),
        //            static_cast<bool>(kOneByteCharFlags[c] & kIsWhiteSpace));
        kOneByteCharFlags[c as usize] & OneByteCharFlags::kIsWhiteSpace as u8 != 0
    }

    pub fn IsWhiteSpaceOrLineTerminator(c: base::uc32) -> bool {
        if (!base::IsInRange(c, 0, 255)) { return IsWhiteSpaceOrLineTerminatorSlow(c); }
        //DCHECK_EQ(
        //    IsWhiteSpaceOrLineTerminatorSlow(c),
        //    static_cast<bool>(kOneByteCharFlags[c] & kIsWhiteSpaceOrLineTerminator));
        kOneByteCharFlags[c as usize] & OneByteCharFlags::kIsWhiteSpaceOrLineTerminator as u8 != 0
    }

    pub fn IsLineTerminatorSequence(c: base::uc32, next: base::uc32) -> bool {
        if kOneByteCharFlags[c as usize] & OneByteCharFlags::kMaybeLineEnd as u8 != 0 {
            if c == '\n' as u32 { return true; }
            if c == '\r' as u32 { return next != '\n' as u32; }
            return base::IsInRange(c as u32, 0x2028, 0x2029);
        }
        false
    }

    // Slow path implementations (can be customized if needed)
    fn IsIdentifierStartSlow(c: base::uc32) -> bool {
        // Placeholder implementation
        c >= 0x80 && c <= 0x10FFFF // Example: Check if outside ASCII range
    }

    fn IsIdentifierPartSlow(c: base::uc32) -> bool {
        // Placeholder implementation
        c >= 0x80 && c <= 0x10FFFF // Example: Check if outside ASCII range
    }

    fn IsWhiteSpaceSlow(c: base::uc32) -> bool {
        // Placeholder implementation
        c == 0x2003 // Example: Em-space
    }

    fn IsWhiteSpaceOrLineTerminatorSlow(c: base::uc32) -> bool {
        // Placeholder implementation
        c == 0x2003 || c == '\r' as u32 || c == '\n' as u32 // Example: Em-space, CR, LF
    }
}
