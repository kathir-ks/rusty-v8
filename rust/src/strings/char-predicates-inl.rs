// Copyright 2011 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod char_predicates {
    // use crate::base::bounds::IsInRange; // Assuming base::bounds is mapped to this Rust module
    // use crate::strings::char_predicates::*; // Assuming strings::char_predicates is mapped to this Rust module
    // use crate::utils::utils::*; // Assuming utils::utils is mapped to this Rust module

    /// If `c` is in 'A'-'Z' or 'a'-'z', return its lower-case.
    /// Else, return something outside of 'A'-'Z' and 'a'-'z'.
    /// Note: it ignores LOCALE.
    #[inline]
    pub const fn ascii_alpha_to_lower(c: u32) -> u32 {
        c | 0x20
    }

    #[inline]
    pub const fn is_carriage_return(c: u32) -> bool {
        c == 0x000D
    }

    #[inline]
    pub const fn is_line_feed(c: u32) -> bool {
        c == 0x000A
    }

    #[inline]
    pub const fn is_ascii_identifier(c: u32) -> bool {
        is_alpha_numeric(c) || c == '$' as u32 || c == '_' as u32
    }

    #[inline]
    pub const fn is_alpha_numeric(c: u32) -> bool {
        is_in_range(ascii_alpha_to_lower(c), 'a' as u32, 'z' as u32) || is_decimal_digit(c)
    }

    #[inline]
    pub const fn is_decimal_digit(c: u32) -> bool {
        is_in_range(c, '0' as u32, '9' as u32)
    }

    #[inline]
    pub const fn is_hex_digit(c: u32) -> bool {
        is_decimal_digit(c) || is_in_range(ascii_alpha_to_lower(c), 'a' as u32, 'f' as u32)
    }

    #[inline]
    pub const fn is_octal_digit(c: u32) -> bool {
        is_in_range(c, '0' as u32, '7' as u32)
    }

    #[inline]
    pub const fn is_non_octal_decimal_digit(c: u32) -> bool {
        is_in_range(c, '8' as u32, '9' as u32)
    }

    #[inline]
    pub const fn is_binary_digit(c: u32) -> bool {
        c == '0' as u32 || c == '1' as u32
    }

    #[inline]
    pub const fn is_ascii(c: u32) -> bool {
        (c & !0x7F) == 0
    }

    #[inline]
    pub const fn is_ascii_lower(c: u32) -> bool {
        is_in_range(c, 'a' as u32, 'z' as u32)
    }

    #[inline]
    pub const fn is_ascii_upper(c: u32) -> bool {
        is_in_range(c, 'A' as u32, 'Z' as u32)
    }

    #[inline]
    pub const fn to_ascii_upper(c: u32) -> u32 {
        c & !(if is_ascii_lower(c) { 1 << 5 } else { 0 })
    }

    #[inline]
    pub const fn to_ascii_lower(c: u32) -> u32 {
        c | (if is_ascii_upper(c) { 1 << 5 } else { 0 })
    }

    #[inline]
    pub const fn is_reg_exp_word(c: u32) -> bool {
        is_alpha_numeric(c) || c == '_' as u32
    }

    // Constexpr cache table for character flags.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct OneByteCharFlags(u8);

    impl OneByteCharFlags {
        pub const IS_IDENTIFIER_START: Self = Self(1 << 0);
        pub const IS_IDENTIFIER_PART: Self = Self(1 << 1);
        pub const IS_WHITESPACE: Self = Self(1 << 2);
        pub const IS_WHITESPACE_OR_LINE_TERMINATOR: Self = Self(1 << 3);
        pub const MAYBE_LINE_END: Self = Self(1 << 4);
    }

    impl std::ops::BitOr for OneByteCharFlags {
        type Output = Self;

        fn bitor(self, other: Self) -> Self {
            Self(self.0 | other.0)
        }
    }

    impl std::ops::BitAnd for OneByteCharFlags {
        type Output = Self;

        fn bitand(self, other: Self) -> Self {
            Self(self.0 & other.0)
        }
    }

    // See http://www.unicode.org/Public/UCD/latest/ucd/DerivedCoreProperties.txt
    // ID_Start. Additionally includes '_' and '$'.
    pub const fn is_one_byte_id_start(c: u32) -> bool {
        c == '$' as u32
            || (c >= 'A' as u32 && c <= 'Z' as u32)
            || c == '_' as u32
            || (c >= 'a' as u32 && c <= 'z' as u32)
            || c == 0x00AA
            || c == 0x00B5
            || c == 0x00BA
            || (c >= 0x00C0 && c <= 0x00D6)
            || (c >= 0x00D8 && c <= 0x00F6)
            || (c >= 0x00F8 && c <= 0x00FF)
    }

    // See http://www.unicode.org/Public/UCD/latest/ucd/DerivedCoreProperties.txt
    // ID_Continue. Additionally includes '_' and '$'.
    pub const fn is_one_byte_id_continue(c: u32) -> bool {
        c == '$' as u32
            || (c >= '0' as u32 && c <= '9' as u32)
            || c == '_' as u32
            || (c >= 'A' as u32 && c <= 'Z' as u32)
            || (c >= 'a' as u32 && c <= 'z' as u32)
            || c == 0x00AA
            || c == 0x00B5
            || c == 0x00B7
            || c == 0x00BA
            || (c >= 0x00C0 && c <= 0x00D6)
            || (c >= 0x00D8 && c <= 0x00F6)
            || (c >= 0x00F8 && c <= 0x00FF)
    }

    pub const fn is_one_byte_whitespace(c: u32) -> bool {
        c == '\t' as u32 || c == '\x0B' as u32 || c == '\x0C' as u32 || c == ' ' as u32 || c == 0xA0
    }

    pub const fn build_one_byte_char_flags(c: u32) -> OneByteCharFlags {
        let mut result = OneByteCharFlags(0);
        if is_one_byte_id_start(c) || c == '\\' as u32 {
            result = result | OneByteCharFlags::IS_IDENTIFIER_START;
        }
        if is_one_byte_id_continue(c) || c == '\\' as u32 {
            result = result | OneByteCharFlags::IS_IDENTIFIER_PART;
        }
        if is_one_byte_whitespace(c) {
            result = result | OneByteCharFlags::IS_WHITESPACE | OneByteCharFlags::IS_WHITESPACE_OR_LINE_TERMINATOR;
        }
        if c == '\r' as u32 || c == '\n' as u32 {
            result = result | OneByteCharFlags::IS_WHITESPACE_OR_LINE_TERMINATOR | OneByteCharFlags::MAYBE_LINE_END;
        }
        // Add markers to identify 0x2028 and 0x2029.
        if c == 0x2028 || c == 0x2029 {
            result = result | OneByteCharFlags::MAYBE_LINE_END;
        }
        result
    }

    const ONE_BYTE_CHAR_FLAGS: [OneByteCharFlags; 256] = {
        const fn build_array() -> [OneByteCharFlags; 256] {
            let mut arr = [OneByteCharFlags(0); 256];
            let mut n = 0;
            while n < 256 {
                arr[n] = build_one_byte_char_flags(n as u32);
                n += 1;
            }
            arr
        }
        build_array()
    };

    pub fn is_identifier_start(c: u32) -> bool {
        if !is_in_range(c, 0, 255) {
            return is_identifier_start_slow(c);
        }
        let flag = ONE_BYTE_CHAR_FLAGS[c as usize] & OneByteCharFlags::IS_IDENTIFIER_START;

        is_identifier_start_slow(c) == (flag != OneByteCharFlags(0));

        flag != OneByteCharFlags(0)
    }

    pub fn is_identifier_part(c: u32) -> bool {
        if !is_in_range(c, 0, 255) {
            return is_identifier_part_slow(c);
        }

        let flag = ONE_BYTE_CHAR_FLAGS[c as usize] & OneByteCharFlags::IS_IDENTIFIER_PART;

        is_identifier_part_slow(c) == (flag != OneByteCharFlags(0));

        flag != OneByteCharFlags(0)
    }

    pub fn is_white_space(c: u32) -> bool {
        if !is_in_range(c, 0, 255) {
            return is_white_space_slow(c);
        }

        let flag = ONE_BYTE_CHAR_FLAGS[c as usize] & OneByteCharFlags::IS_WHITESPACE;
        is_white_space_slow(c) == (flag != OneByteCharFlags(0));
        flag != OneByteCharFlags(0)
    }

    pub fn is_white_space_or_line_terminator(c: u32) -> bool {
        if !is_in_range(c, 0, 255) {
            return is_white_space_or_line_terminator_slow(c);
        }

        let flag = ONE_BYTE_CHAR_FLAGS[c as usize] & OneByteCharFlags::IS_WHITESPACE_OR_LINE_TERMINATOR;

        is_white_space_or_line_terminator_slow(c) == (flag != OneByteCharFlags(0));

        flag != OneByteCharFlags(0)
    }

    pub fn is_line_terminator_sequence(c: u32, next: u32) -> bool {
        if (ONE_BYTE_CHAR_FLAGS[c as usize] & OneByteCharFlags::MAYBE_LINE_END) != OneByteCharFlags(0) {
            if c == '\n' as u32 {
                return true;
            }
            if c == '\r' as u32 {
                return next != '\n' as u32;
            }
            return (0x2028..=0x2029).contains(&c);
        }
        false
    }

    // Dummy implementations for slow versions since it's outside the scope
    fn is_identifier_start_slow(_c: u32) -> bool {
        false
    }
    fn is_identifier_part_slow(_c: u32) -> bool {
        false
    }
    fn is_white_space_slow(_c: u32) -> bool {
        false
    }
    fn is_white_space_or_line_terminator_slow(_c: u32) -> bool {
        false
    }

    const fn is_in_range(c: u32, low: u32, high: u32) -> bool {
        c >= low && c <= high
    }
}