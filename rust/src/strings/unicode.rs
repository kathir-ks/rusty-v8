// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file was generated at 2014-10-08 15:25:47.940335

#![allow(non_camel_case_types)]
#![allow(dead_code)]

// use std::os::raw::{c_int, c_uchar};
use std::convert::TryInto;

#[cfg(V8_ENABLE_WEBASSEMBLY)]
// extern crate utf8_decoder; // Requires creating a rust crate for utf8-decoder

// #[cfg(V8_INTL_SUPPORT)]
// extern crate icu; // Requires adding icu crate

//pub type uchar = u32;
pub type uchar = u16;
pub type Utf8IncrementalBuffer = u32;

pub const kMaxOneByteChar: u8 = 127;
pub const kBadChar: uchar = 0xFFFD; // Replacement character
pub const kBufferEmpty: uchar = 0x0000; // Null character

mod unibrow {
    pub use super::kBadChar;
    pub use super::uchar;
    pub use super::Utf8IncrementalBuffer;
    pub use super::Utf8::State;

    //#[allow(dead_code)]
    pub const kSentinel: uchar = 0xFFFF; //static_cast<uchar>(-1)

    #[derive(Debug, PartialEq)]
    pub enum ConvertError {
        InvalidInput,
    }
}

// TODO: This struct is never directly used, and its static methods are called as functions.
// Could be refactored as standalone functions rather than a struct.
pub struct Uppercase {}

impl Uppercase {
    #[cfg(V8_INTL_SUPPORT)]
    pub fn is(c: uchar) -> bool {
        // TODO: Replace with ICU crate equivalent
        // unsafe { icu::ucasemap_to_upper(0 as *mut _, c as i32, &mut err) == 1 }
        false
    }

    #[cfg(not(V8_INTL_SUPPORT))]
    pub fn is(c: uchar) -> bool {
        let chunk_index = (c >> 13) as usize;
        match chunk_index {
            0 => Self::lookup_predicate(&K_UPPERCASE_TABLE0, K_UPPERCASE_TABLE0_SIZE, c),
            1 => Self::lookup_predicate(&K_UPPERCASE_TABLE1, K_UPPERCASE_TABLE1_SIZE, c),
            5 => Self::lookup_predicate(&K_UPPERCASE_TABLE5, K_UPPERCASE_TABLE5_SIZE, c),
            7 => Self::lookup_predicate(&K_UPPERCASE_TABLE7, K_UPPERCASE_TABLE7_SIZE, c),
            _ => false,
        }
    }

    #[cfg(not(V8_INTL_SUPPORT))]
    fn lookup_predicate(table: &[i32], size: u16, chr: uchar) -> bool {
        const ENTRY_DIST: usize = 1;
        let value = chr & (K_CHUNK_BITS - 1);
        let mut low: usize = 0;
        let mut high: usize = (size - 1) as usize;

        while high != low {
            let mid = low + ((high - low) >> 1);
            let current_value = Self::get_entry(table[ENTRY_DIST * mid]);

            if (current_value <= value)
                && (mid + 1 == size as usize
                    || Self::get_entry(table[ENTRY_DIST * (mid + 1)]) > value)
            {
                low = mid;
                break;
            } else if current_value < value {
                low = mid + 1;
            } else {
                if mid == 0 {
                    break;
                }
                high = mid - 1;
            }
        }

        let field = table[ENTRY_DIST * low];
        let entry = Self::get_entry(field);
        let is_start = Self::is_start(field);

        (entry == value) || (entry < value && is_start)
    }

    #[cfg(not(V8_INTL_SUPPORT))]
    #[inline]
    fn get_entry(entry: i32) -> uchar {
        (entry & (K_START_BIT - 1)) as uchar
    }

    #[cfg(not(V8_INTL_SUPPORT))]
    #[inline]
    fn is_start(entry: i32) -> bool {
        (entry & K_START_BIT) != 0
    }
}

pub struct Letter {}

impl Letter {
    #[cfg(V8_INTL_SUPPORT)]
    pub fn is(c: uchar) -> bool {
        // TODO: Replace with ICU crate equivalent
        // unsafe { icu::ucasemap_to_upper(0 as *mut _, c as i32, &mut err) == 1 }
        false
    }

    #[cfg(not(V8_INTL_SUPPORT))]
    pub fn is(c: uchar) -> bool {
        let chunk_index = (c >> 13) as usize;
        match chunk_index {
            0 => Self::lookup_predicate(&K_LETTER_TABLE0, K_LETTER_TABLE0_SIZE, c),
            1 => Self::lookup_predicate(&K_LETTER_TABLE1, K_LETTER_TABLE1_SIZE, c),
            2 => Self::lookup_predicate(&K_LETTER_TABLE2, K_LETTER_TABLE2_SIZE, c),
            3 => Self::lookup_predicate(&K_LETTER_TABLE3, K_LETTER_TABLE3_SIZE, c),
            4 => Self::lookup_predicate(&K_LETTER_TABLE4, K_LETTER_TABLE4_SIZE, c),
            5 => Self::lookup_predicate(&K_LETTER_TABLE5, K_LETTER_TABLE5_SIZE, c),
            6 => Self::lookup_predicate(&K_LETTER_TABLE6, K_LETTER_TABLE6_SIZE, c),
            7 => Self::lookup_predicate(&K_LETTER_TABLE7, K_LETTER_TABLE7_SIZE, c),
            _ => false,
        }
    }

    #[cfg(not(V8_INTL_SUPPORT))]
    fn lookup_predicate(table: &[i32], size: u16, chr: uchar) -> bool {
        const ENTRY_DIST: usize = 1;
        let value = chr & (K_CHUNK_BITS - 1);
        let mut low: usize = 0;
        let mut high: usize = (size - 1) as usize;

        while high != low {
            let mid = low + ((high - low) >> 1);
            let current_value = Self::get_entry(table[ENTRY_DIST * mid]);

            if (current_value <= value)
                && (mid + 1 == size as usize
                    || Self::get_entry(table[ENTRY_DIST * (mid + 1)]) > value)
            {
                low = mid;
                break;
            } else if current_value < value {
                low = mid + 1;
            } else {
                if mid == 0 {
                    break;
                }
                high = mid - 1;
            }
        }

        let field = table[ENTRY_DIST * low];
        let entry = Self::get_entry(field);
        let is_start = Self::is_start(field);

        (entry == value) || (entry < value && is_start)
    }

    #[cfg(not(V8_INTL_SUPPORT))]
    #[inline]
    fn get_entry(entry: i32) -> uchar {
        (entry & (K_START_BIT - 1)) as uchar
    }

    #[cfg(not(V8_INTL_SUPPORT))]
    #[inline]
    fn is_start(entry: i32) -> bool {
        (entry & K_START_BIT) != 0
    }
}

pub struct ID_Start {}

impl ID_Start {
    #[cfg(not(V8_INTL_SUPPORT))]
    pub fn is(c: uchar) -> bool {
        let chunk_index = (c >> 13) as usize;
        match chunk_index {
            0 => Self::lookup_predicate(&K_ID_START_TABLE0, K_ID_START_TABLE0_SIZE, c),
            1 => Self::lookup_predicate(&K_ID_START_TABLE1, K_ID_START_TABLE1_SIZE, c),
            2 => Self::lookup_predicate(&K_ID_START_TABLE2, K_ID_START_TABLE2_SIZE, c),
            3 => Self::lookup_predicate(&K_ID_START_TABLE3, K_ID_START_TABLE3_SIZE, c),
            4 => Self::lookup_predicate(&K_ID_START_TABLE4, K_ID_START_TABLE4_SIZE, c),
            5 => Self::lookup_predicate(&K_ID_START_TABLE5, K_ID_START_TABLE5_SIZE, c),
            6 => Self::lookup_predicate(&K_ID_START_TABLE6, K_ID_START_TABLE6_SIZE, c),
            7 => Self::lookup_predicate(&K_ID_START_TABLE7, K_ID_START_TABLE7_SIZE, c),
            _ => false,
        }
    }

    #[cfg(not(V8_INTL_SUPPORT))]
    fn lookup_predicate(table: &[i32], size: u16, chr: uchar) -> bool {
        const ENTRY_DIST: usize = 1;
        let value = chr & (K_CHUNK_BITS - 1);
        let mut low: usize = 0;
        let mut high: usize = (size - 1) as usize;

        while high != low {
            let mid = low + ((high - low) >> 1);
            let current_value = Self::get_entry(table[ENTRY_DIST * mid]);

            if (current_value <= value)
                && (mid + 1 == size as usize
                    || Self::get_entry(table[ENTRY_DIST * (mid + 1)]) > value)
            {
                low = mid;
                break;
            } else if current_value < value {
                low = mid + 1;
            } else {
                if mid == 0 {
                    break;
                }
                high = mid - 1;
            }
        }

        let field = table[ENTRY_DIST * low];
        let entry = Self::get_entry(field);
        let is_start = Self::is_start(field);

        (entry == value) || (entry < value && is_start)
    }

    #[cfg(not(V8_INTL_SUPPORT))]
    #[inline]
    fn get_entry(entry: i32) -> uchar {
        (entry & (K_START_BIT - 1)) as uchar
    }

    #[cfg(not(V8_INTL_SUPPORT))]
    #[inline]
    fn is_start(entry: i32) -> bool {
        (entry & K_START_BIT) != 0
    }
}

pub struct ID_Continue {}

impl ID_Continue {
    #[cfg(not(V8_INTL_SUPPORT))]
    pub fn is(c: uchar) -> bool {
        let chunk_index = (c >> 13) as usize;
        match chunk_index {
            0 => Self::lookup_predicate(&K_ID_CONTINUE_TABLE0, K_ID_CONTINUE_TABLE0_SIZE, c),
            1 => Self::lookup_predicate(&K_ID_CONTINUE_TABLE1, K_ID_CONTINUE_TABLE1_SIZE, c),
            5 => Self::lookup_predicate(&K_ID_CONTINUE_TABLE5, K_ID_CONTINUE_TABLE5_SIZE, c),
            7 => Self::lookup_predicate(&K_ID_CONTINUE_TABLE7, K_ID_CONTINUE_TABLE7_SIZE, c),
            _ => false,
        }
    }

    #[cfg(not(V8_INTL_SUPPORT))]
    fn lookup_predicate(table: &[i32], size: u16, chr: uchar) -> bool {
        const ENTRY_DIST: usize = 1;
        let value = chr & (K_CHUNK_BITS - 1);
        let mut low: usize = 0;
        let mut high: usize = (size - 1) as usize;

        while high != low {
            let mid = low + ((high - low) >> 1);
            let current_value = Self::get_entry(table[ENTRY_DIST * mid]);

            if (current_value <= value)
                && (mid + 1 == size as usize
                    || Self::get_entry(table[ENTRY_DIST * (mid + 1)]) > value)
            {
                low = mid;
                break;
            } else if current_value < value {
                low = mid + 1;
            } else {
                if mid == 0 {
                    break;
                }
                high = mid - 1;
            }
        }

        let field = table[ENTRY_DIST * low];
        let entry = Self::get_entry(field);
        let is_start = Self::is_start(field);

        (entry == value) || (entry < value && is_start)
    }

    #[cfg(not(V8_INTL_SUPPORT))]
    #[inline]
    fn get_entry(entry: i32) -> uchar {
        (entry & (K_START_BIT - 1)) as uchar
    }

    #[cfg(not(V8_INTL_SUPPORT))]
    #[inline]
    fn is_start(entry: i32) -> bool {
        (entry & K_START_BIT) != 0
    }
}

pub struct WhiteSpace {}

impl WhiteSpace {
    #[cfg(not(V8_INTL_SUPPORT))]
    pub fn is(c: uchar) -> bool {
        let chunk_index = (c >> 13) as usize;
        match chunk_index {
            0 => Self::lookup_predicate(&K_WHITE_SPACE_TABLE0, K_WHITE_SPACE_TABLE0_SIZE, c),
            1 => Self::lookup_predicate(&K_WHITE_SPACE_TABLE1, K_WHITE_SPACE_TABLE1_SIZE, c),
            7 => Self::lookup_predicate(&K_WHITE_SPACE_TABLE7, K_WHITE_SPACE_TABLE7_SIZE, c),
            _ => false,
        }
    }

    #[cfg(not(V8_INTL_SUPPORT))]
    fn lookup_predicate(table: &[i32], size: u16, chr: uchar) -> bool {
        const ENTRY_DIST: usize = 1;
        let value = chr & (K_CHUNK_BITS - 1);
        let mut low: usize = 0;
        let mut high: usize = (size - 1) as usize;

        while high != low {
            let mid = low + ((high - low) >> 1);
            let current_value = Self::get_entry(table[ENTRY_DIST * mid]);

            if (current_value <= value)
                && (mid + 1 == size as usize
                    || Self::get_entry(table[ENTRY_DIST * (mid + 1)]) > value)
            {
                low = mid;
                break;
            } else if current_value < value {
                low = mid + 1;
            } else {
                if mid == 0 {
                    break;
                }
                high = mid - 1;
            }
        }

        let field = table[ENTRY_DIST * low];
        let entry = Self::get_entry(field);
        let is_start = Self::is_start(field);

        (entry == value) || (entry < value && is_start)
    }

    #[cfg(not(V8_INTL_SUPPORT))]
    #[inline]
    fn get_entry(entry: i32) -> uchar {
        (entry & (K_START_BIT - 1)) as uchar
    }

    #[cfg(not(V8_INTL_SUPPORT))]
    #[inline]
    fn is_start(entry: i32) -> bool {
        (entry & K_START_BIT) != 0
    }
}

/// UTF-8 related functions.
pub mod Utf8 {
    use super::*;
    use std::mem::MaybeUninit;

    /// Represents the state of the UTF-8 decoder.
    #[derive(PartialEq, Eq, Debug, Clone, Copy)]
    pub enum State {
        kAccept = 0,
        kS2 = 1,
        kS3FirstByte = 2,
        kS3SecondByte = 3,
        kS4FirstByte = 4,
        kS4SecondByte = 5,
        kS4ThirdByte = 6,
        kReject = 7,
    }

    /// Calculates the UTF-8 value from a byte slice.
    ///
    /// # Arguments
    ///
    /// * `str` - A pointer to the byte slice.
    /// * `max_length` - The maximum length of the byte slice to read.
    /// * `cursor` - A mutable reference to the cursor position.
    pub fn calculate_value(str: &[u8], max_length: usize, cursor: &mut usize) -> uchar {
        debug_assert!(max_length > 0);
        debug_assert!(str[0] > kMaxOneByteChar);

        let mut state = State::kAccept;
        let mut buffer: Utf8IncrementalBuffer = 0;

        let start = *cursor;
        let mut current_cursor = 0;

        while current_cursor < max_length {
            let t = value_of_incremental(
                unsafe { str.as_ptr().add(current_cursor) },
                &mut state,
                &mut buffer,
            );

            if t != unibrow::kSentinel {
                if t != unibrow::kBadChar {
                    *cursor += current_cursor + 1;
                    return t;
                } else {
                    *cursor += current_cursor + 1;
                    return unibrow::kBadChar;
                }
            }
            current_cursor += 1;
        }

        *cursor += current_cursor;
        if state == State::kAccept {
            unibrow::kBufferEmpty
        } else {
            unibrow::kBadChar
        }
    }

    /// Finishes the incremental decoding, ensuring that if an unfinished sequence is left that it is replaced by a replacement char.
    pub fn value_of_incremental_finish(state: &mut State) -> uchar {
        if *state == State::kAccept {
            return unibrow::kBufferEmpty;
        } else {
            debug_assert!(*state as i32 > State::kAccept as i32);
            *state = State::kAccept;
            unibrow::kBadChar
        }
    }

    /// Validates the UTF-8 encoding of a byte slice.
    pub fn validate_encoding(bytes: &[u8], length: usize) -> bool {
        let mut state = State::kAccept;
        let mut throw_away: Utf8IncrementalBuffer = 0;

        for i in 0..length {
            if state == State::kReject {
                break;
            }
            utf8dfadecoder::decode(bytes[i], &mut state, &mut throw_away);
        }

        state == State::kAccept
    }

    /// Incremental UTF-8 decoder.
    ///
    /// Based on Table 3.7 from the Unicode Standard, Version 15.0.
    /// https://www.unicode.org/versions/Unicode15.0.0/ch03.pdf
    ///
    /// DFA-based decoder:
    /// ```
    ///        |           Input byte           |
    /// State  |----------------------------------|  Output
    ///        | 0x00..0x7F | 0x80..0xBF | 0xC2..0xDF | 0xE0    | 0xE1..0xEC | 0xED    | 0xEE..0xEF | 0xF0    | 0xF1..0xF3 | 0xF4    | 0xF5..0xFF |
    /// ------ + ---------- + ---------- + ---------- + -------- + ---------- + -------- + ---------- + -------- + ---------- + -------- + ---------- + ------
    /// Accept |  U+00..U+7F|   Reject   |  State_S2 | State_S3A|  State_S3 | State_S3B|  State_S3 | State_S4A|  State_S4 | State_S4B|   Reject   | U+00..U+7F
    /// S2     |   Reject   | U+0080..U+7FF           |   Reject   |   Reject |   Reject |   Reject |   Reject |   Reject |   Reject |   Reject |   Reject   |   Reject
    /// S3     |   Reject   | U+0800..U+D7FF,        |   Reject   |   Reject |   Reject |   Reject |   Reject |   Reject |   Reject |   Reject |   Reject   |   Reject
    ///        |           U+E000..U+FFFF           |
    /// S3A    |   Reject   | U+0800..U+D7FF,        |   Reject   |   Reject |   Reject |   Reject |   Reject |   Reject |   Reject |   Reject |   Reject   |   Reject
    ///        |           U+E000..U+FFFF           |
    /// S3B    |   Reject   | U+0800..U+D7FF,        |   Reject   |   Reject |   Reject |   Reject |   Reject |   Reject |   Reject |   Reject |   Reject   |   Reject
    ///        |           U+E000..U+FFFF           |
    /// S4     |   Reject   | U+10000..U+10FFFF       |   Reject   |   Reject |   Reject |   Reject |   Reject |   Reject |   Reject |   Reject |   Reject   |   Reject
    /// S4A    |   Reject   | U+10000..U+10FFFF       |   Reject   |   Reject |   Reject |   Reject |   Reject |   Reject |   Reject |   Reject |   Reject   |   Reject
    /// S4B    |   Reject   | U+10000..U+10FFFF       |   Reject   |   Reject |   Reject |   Reject |   Reject |   Reject |   Reject |   Reject |   Reject   |   Reject
    /// ```
    pub mod utf8dfadecoder {
        use super::*;

        // This function is a direct translation of the V8 C++ code with similar performance.
        pub fn decode(byte: u8, state: &mut State, buffer: &mut Utf8IncrementalBuffer) -> uchar {
            match *state {
                State::kAccept => {
                    if byte < 0x80 {
                        // 0x00..0x7F
                        *buffer = byte as Utf8IncrementalBuffer;
                        unibrow::kSentinel
                    } else if byte < 0xC0 {
                        // 0x80..0xBF
                        *state = State::kReject;
                        *buffer = unibrow::kBadChar as Utf8IncrementalBuffer;
                        unibrow::kSentinel
                    } else if byte < 0xE0 {
                        // 0xC2..0xDF
                        *state = State::kS2;
                        *buffer = (byte as Utf8IncrementalBuffer & 0x1F) << 6;
                        unibrow::kSentinel
                    } else if byte == 0xE0 {
                        // 0xE0
                        *state = State::kS3FirstByte;
                        *buffer = (byte as Utf8IncrementalBuffer & 0x0F) << 12;
                        unibrow::kSentinel
                    } else if byte < 0xED {
                        // 0xE1..0xEC
                        *state = State::kS3;
                        *buffer = (byte as Utf8IncrementalBuffer & 0x0F) << 12;
                        unibrow::kSentinel
                    } else if byte == 0xED {
                        // 0xED
                        *state = State::kS3SecondByte;
                        *buffer = (byte as Utf8IncrementalBuffer & 0x0F) << 12;
                        unibrow::kSentinel
                    } else if byte < 0xF0 {
                        // 0xEE..0xEF
                        *state = State::kS3;
                        *buffer = (byte as Utf8IncrementalBuffer & 0x0F) << 12;
                        unibrow::kSentinel
                    } else if byte == 0xF0 {
                        // 0xF0
                        *state = State::kS4FirstByte;
                        *buffer = (byte as Utf8IncrementalBuffer & 0x07) << 18;
                        unibrow::kSentinel
                    } else if byte < 0xF4 {
                        // 0xF1..0xF3
                        *state = State::kS4;
                        *buffer = (byte as Utf8IncrementalBuffer & 0x07) << 18;
                        unibrow::kSentinel
                    } else if byte == 0xF4 {
                        // 0xF4
                        *state = State::kS4SecondByte;
                        *buffer = (byte as Utf8IncrementalBuffer & 0x07) << 18;
                        unibrow::kSentinel
                    } else {
                        // 0xF5..0xFF
                        *state = State::kReject;
                        *buffer = unibrow::kBadChar as Utf8IncrementalBuffer;
                        unibrow::kSentinel
                    }
                }
                State::kS2 => {
                    if byte < 0x80 || byte > 0xBF {
                        *state = State::kReject;
                        *buffer = unibrow::kBadChar as Utf8IncrementalBuffer;
                        unibrow::kSentinel
                    } else {
                        *state = State::kAccept;
                        *buffer |= byte as Utf8IncrementalBuffer & 0x3F;
                        (*buffer).try_into().unwrap()
                    }
                }
                State::kS3 | State::kS3FirstByte | State::kS3SecondByte => {
                    if byte < 0x80 || byte > 0xBF {
                        *state = State::kReject;
                        *buffer = unibrow::kBadChar as Utf8IncrementalBuffer;
                        unibrow::kSentinel
                    } else {
                        *buffer |= (byte as Utf8IncrementalBuffer & 0x3F) << 6;
                        if matches!(*state, State::kS3FirstByte | State::kS3SecondByte) {
                            *state = State::kS3;
                            unibrow::kSentinel
                        } else {
                            if *buffer >= 0xD800 && *buffer <= 0xDFFF {
                                *state = State::kReject;
                                *buffer = unibrow::kBadChar as Utf8IncrementalBuffer;
                                unibrow::kSentinel
                            } else {
                                *state = State::kAccept;
                                *buffer |= byte as Utf8IncrementalBuffer & 0x3F;
                                (*buffer).try_into().unwrap()
                            }
                        }
                    }
                }
                State::kS4 | State::kS4FirstByte | State::kS4SecondByte => {
                    if byte < 0x80 || byte > 0xBF {
                        *state = State::kReject;
                        *buffer = unibrow::kBadChar as Utf8IncrementalBuffer;
                        unibrow::kSentinel
                    } else {
                        *buffer |= (byte as Utf8IncrementalBuffer & 0x3F) << 12;

                        if matches!(*state, State::kS4FirstByte | State::kS4SecondByte) {
                            *state = State::kS4;
                            unibrow::kSentinel
                        } else {
                            *state = State::kAccept;
                            *buffer |= byte as Utf8IncrementalBuffer & 0x3F;
                            (*buffer).try_into().unwrap()
                        }
                    }
                }
                State::kReject => {
                    unibrow::kBadChar
                }
            }
        }
    }

    /// Helper function for incremental decoding.
    fn value_of_incremental(str: *const u8, state: &mut State, buffer: &mut Utf8IncrementalBuffer) -> uchar {
        let byte: u8 = unsafe { *str };

        match *state {
            State::kAccept => {
                if byte < 0x80 {
                    *buffer = byte as Utf8IncrementalBuffer;
                    return byte as uchar;
                } else {
                    return utf8dfadecoder::decode(byte, state, buffer);
                }
            }
            _ => {
                return utf8dfadecoder::decode(byte, state, buffer);
            }
        }
    }
}

pub struct Utf16 {}

impl Utf16 {
    /// Replaces unpaired surrogates with the replacement character (U+FFFD).
    pub fn replace_unpaired_surrogates(source_code_units: &[u16], dest_code_units: &mut [u16]) {
        // U+FFFD (REPLACEMENT CHARACTER)
        const K_REPLACEMENT: u16 = 0xFFFD;

        assert_eq!(source_code_units.len(), dest_code_units.len());
        let length = source_code_units.len();

        for i in 0..length {
            let source_code_unit = source_code_units[i];
            let copy_index = i;
            let mut dest_code_unit = source_code_unit;

            if Self::is_lead_surrogate(source_code_unit) {
                // The current code unit is a leading surrogate. If it's not followed by a
                // trailing surrogate, replace it with the replacement character.
                if i == length - 1 || !Self::is_trail_surrogate(source_code_units[i + 1]) {
                    dest_code_unit = K_REPLACEMENT;
                } else {
                    // Copy the paired trailing surrogate. The paired leading surrogate will
                    // be copied below.
                    dest_code_units[i + 1] = source_code_units[i + 1];
                }
            } else if Self::is_trail_surrogate(source_code_unit) {
                // All paired trailing surrogates are skipped above, so this branch is
                // only for those that are unpaired.
                dest_code_unit = K_REPLACEMENT;
            }

            dest_code_units[copy_index] = dest_code_unit;
        }
    }

    #[inline]
    pub fn is_lead_surrogate(code_unit: u16) -> bool {
        (0xD800..=0xDBFF).contains(&code_unit)
    }

    #[inline]
    pub fn is_trail_surrogate(code_unit: u16) -> bool {
        (0xDC00..=0xDFFF).contains(&code_unit)
    }
}

#[cfg(V8_ENABLE_WEBASSEMBLY)]
pub struct Wtf8 {}

#[cfg(V8_ENABLE_WEBASSEMBLY)]
impl Wtf8 {
    //Requires porting the generalized-utf8-decoder.h
    pub fn validate_encoding(bytes: &[u8], length: usize) -> bool {
        use super::Utf16;
        //use generalized_utf8_decoder::GeneralizedUtf8DfaDecoder::State; //Requires making a rust crate
        enum State { //Using basic state for compilation reasons.
            kAccept,
            kReject
        }

        let mut state = State::kAccept;
        let mut current: u32 = 0;
        let mut previous: u32 = 0;
        for i in 0..length {
            //generalized_utf8_decoder::GeneralizedUtf8DfaDecoder::Decode(bytes[i], &mut state, &mut current);
            //Requires making a rust crate.
            if matches!(state, State::kReject) { return false; }
            if matches!(state, State::kAccept) {
                if Utf16::is_trail_surrogate(current as u16) &&
                    Utf16::is_lead_surrogate(previous as u16) {
                    return false;
                }
                previous = current;
                current = 0;
            }
        }
        matches!(state, State::kAccept)
    }

    pub fn scan_for_surrogates(wtf8: &[u8], surrogate_offsets: &mut Vec<usize>) {
        const K_WTF8_SURROGATE_FIRST_BYTE: u8 = 0xED;
        const K_WTF8_SURROGATE_SECOND_BYTE_HIGH_BIT: u8 = 0x20;

        for i in 0..wtf8.len() {
            if wtf8[i] == K_WTF8_SURROGATE_FIRST_BYTE &&
                (wtf8.get(i + 1).map_or(0, |&x| x) & K_WTF8_SURROGATE_SECOND_BYTE_HIGH_BIT) != 0 {
                surrogate_offsets.push(i);
            }
        }
    }
}

// Constants for Unicode tables
const K_START_BIT: i32 = (1 << 30);
const K_CHUNK_BITS: uchar = (1 << 13) as uchar;

// Uppercase Table Constants
const K_UPPERCASE_TABLE0_SIZE: u16 = 455;
const K_UPPERCASE_TABLE1_SIZE