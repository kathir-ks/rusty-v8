extern crate icu;

use icu::properties::BinaryProperty;
use icu::char::{props::is_id_continue, props::is_id_start, char_type};
use icu::char::CharType;

mod base {
    pub type uc32 = u32;
}

pub mod strings {
    pub mod char_predicates {
        use super::super::base::uc32;
        use icu::properties::BinaryProperty;
        use icu::char::{props::is_id_continue, props::is_id_start, char_type};
        use icu::char::CharType;

        /// ES#sec-names-and-keywords Names and Keywords
        /// UnicodeIDStart, '$', '_' and '\'
        pub fn is_identifier_start_slow(c: uc32) -> bool {
            // cannot use u_isIDStart because it does not work for
            // Other_ID_Start characters.
            is_id_start(c) || (c < 0x60 && (c == '$' as uc32 || c == '\\' as uc32 || c == '_' as uc32))
        }

        /// ES#sec-names-and-keywords Names and Keywords
        /// UnicodeIDContinue, '$', '_', '\', ZWJ, and ZWNJ
        pub fn is_identifier_part_slow(c: uc32) -> bool {
            // Can't use u_isIDPart because it does not work for
            // Other_ID_Continue characters.
            is_id_continue(c) || (c < 0x60 && (c == '$' as uc32 || c == '\\' as uc32 || c == '_' as uc32)) || c == 0x200C ||
            c == 0x200D
        }

        /// ES#sec-white-space White Space
        /// gC=Zs, U+0009, U+000B, U+000C, U+FEFF
        pub fn is_white_space_slow(c: uc32) -> bool {
            (char_type(c) == CharType::SpaceSeparator) ||
            (c < 0x0D && (c == 0x09 || c == 0x0B || c == 0x0C)) || c == 0xFEFF
        }
    }
}