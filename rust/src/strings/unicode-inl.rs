// Copyright 2007-2010 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::convert::TryInto;

mod base {
    pub mod logging {
        #[macro_export]
        macro_rules! DCHECK_NOT_NULL {
            ($ptr:expr) => {
                assert!(!$ptr.is_null());
            };
        }
        #[macro_export]
        macro_rules! CHECK_LE {
            ($left:expr, $right:expr) => {
                assert!($left <= $right);
            };
        }
        #[macro_export]
        macro_rules! DCHECK_LE {
            ($left:expr, $right:expr) => {
                assert!($left <= $right);
            };
        }

        #[macro_export]
        macro_rules! DCHECK_EQ {
            ($left:expr, $right:expr) => {
                assert_eq!($left, $right);
            };
        }
    }
}

mod strings {
    pub mod unicode {
        pub type uchar = u32; // Assuming uchar is a 32-bit unsigned integer for Unicode code points

        pub const kBadChar: uchar = 0xFFFDu32; // Unicode Replacement Character
        pub const kMaxOneByteChar: uchar = 0x7Fu32;
        pub const kMaxTwoByteChar: uchar = 0x7FFu32;
        pub const kMaxThreeByteChar: uchar = 0xFFFFu32;

        pub const kNoPreviousCharacter: i32 = -1;

        pub const kSizeOfUnmatchedSurrogate: usize = 3;
        pub const kBytesSavedByCombiningSurrogates: usize = 1;

        pub fn is_lead_surrogate(code_unit: i32) -> bool {
            (0xD800..=0xDBFF).contains(&code_unit)
        }

        pub fn is_trail_surrogate(code_unit: i32) -> bool {
            (0xDC00..=0xDFFF).contains(&code_unit)
        }

        pub fn is_surrogate_pair(previous: i32, c: uchar) -> bool {
            is_lead_surrogate(previous) && is_trail_surrogate(c as i32)
        }

        pub fn combine_surrogate_pair(previous: i32, c: uchar) -> uchar {
            let high = (previous - 0xD800) as u32;
            let low = (c - 0xDC00) as u32;
            ((high << 10) + low + 0x10000) as uchar
        }

        #[derive(Debug, PartialEq, Eq)]
        pub enum Utf8EncodingError {
            InvalidUtf8,
            InsufficientCapacity,
        }

        pub struct EncodingResult {
            pub bytes_written: usize,
            pub characters_processed: usize,
        }
    }
}

mod utils {
    pub mod utils {
        #[macro_export]
        macro_rules! V8_LIKELY {
            ($e:expr) => {
                $e
            };
        }
    }
}

pub mod unibrow {
    use super::*;
    use crate::base::logging::{DCHECK_EQ, DCHECK_LE, CHECK_LE};
    use crate::strings::unicode::*;
    use crate::utils::utils::V8_LIKELY;

    const kMask: usize = 255; // Assuming the mask is 255 based on typical usage
    
    // The `Predicate` and `Mapping` structs, along with their methods, are
    // disabled because `V8_INTL_SUPPORT` is not defined. These structures appear
    // to be related to internationalization support, and without the macro
    // defined, the code should not be included. If `V8_INTL_SUPPORT` is
    // defined, the template code would need to be converted to Rust,
    // potentially using traits and associated types to mimic the template
    // behavior.

    pub struct Utf16 {}

    impl Utf16 {
        pub fn has_unpaired_surrogate(code_units: &[u16], length: usize) -> bool {
            for i in 0..length {
                let code_unit = code_units[i] as i32;
                if is_lead_surrogate(code_unit) {
                    if i == length - 1 {
                        return true;
                    }
                    if !is_trail_surrogate(code_units[i + 1] as i32) {
                        return true;
                    }
                    continue; // Skip the paired trailing surrogate
                } else if is_trail_surrogate(code_unit) {
                    return true;
                }
            }
            false
        }
    }

    pub struct Utf8 {}

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum State {
        kAccept,
        kReject,
        kS2,
        kS3,
        kS4,
        kS5,
        kS6,
        kS7,
        kS8,
        kS9,
        kS10,
        kS11,
        kS12,
    }

    pub struct Utf8IncrementalBuffer {
        value: u32,
    }

    impl Utf8IncrementalBuffer {
        pub fn new() -> Self {
            Utf8IncrementalBuffer { value: 0 }
        }

        pub fn get(&self) -> u32 {
            self.value
        }

        pub fn set(&mut self, value: u32) {
            self.value = value;
        }
    }

    impl Default for Utf8IncrementalBuffer {
        fn default() -> Self {
            Self::new()
        }
    }

    pub struct Utf8DfaDecoder {}

    impl Utf8DfaDecoder {
        pub fn decode(next: u8, state: &mut State, buffer: &mut Utf8IncrementalBuffer) {
            match *state {
                State::kAccept => {
                    if next < 0x80 {
                        buffer.set(next as u32);
                        *state = State::kAccept;
                    } else if next < 0xE0 {
                        buffer.set((next as u32 & 0x1F) << 6);
                        *state = State::kS2;
                    } else if next < 0xF0 {
                        buffer.set((next as u32 & 0x0F) << 12);
                        *state = State::kS3;
                    } else if next < 0xF8 {
                        buffer.set((next as u32 & 0x07) << 18);
                        *state = State::kS4;
                    } else {
                        *state = State::kReject;
                    }
                }
                State::kS2 => {
                    if next >= 0x80 && next < 0xC0 {
                        let t = buffer.get() | ((next as u32 & 0x3F));
                        buffer.set(t);
                        *state = State::kAccept;
                    } else {
                        *state = State::kReject;
                    }
                }
                State::kS3 => {
                    if next >= 0x80 && next < 0xC0 {
                        let t = buffer.get() | ((next as u32 & 0x3F) << 6);
                        buffer.set(t);
                        *state = State::kS5;
                    } else {
                        *state = State::kReject;
                    }
                }
                State::kS4 => {
                    if next >= 0x80 && next < 0xC0 {
                        let t = buffer.get() | ((next as u32 & 0x3F) << 12);
                        buffer.set(t);
                        *state = State::kS6;
                    } else {
                        *state = State::kReject;
                    }
                }
                State::kS5 => {
                    if next >= 0x80 && next < 0xC0 {
                        let t = buffer.get() | ((next as u32 & 0x3F));
                        buffer.set(t);
                        *state = State::kAccept;
                    } else {
                        *state = State::kReject;
                    }
                }
                State::kS6 => {
                    if next >= 0x80 && next < 0xC0 {
                        let t = buffer.get() | ((next as u32 & 0x3F) << 6);
                        buffer.set(t);
                        *state = State::kS7;
                    } else {
                        *state = State::kReject;
                    }
                }
                State::kS7 => {
                    if next >= 0x80 && next < 0xC0 {
                        let t = buffer.get() | ((next as u32 & 0x3F));
                        buffer.set(t);
                        *state = State::kAccept;
                    } else {
                        *state = State::kReject;
                    }
                }
                _ => {
                    *state = State::kReject; // Handle other states by rejecting.
                }
            }
        }
    }

    impl Utf8 {
        pub fn value_of_incremental(
            cursor: &mut &[u8],
            state: &mut State,
            buffer: &mut Utf8IncrementalBuffer,
        ) -> uchar {
            base::logging::DCHECK_NOT_NULL!(buffer);
            let old_state = *state;
            if cursor.is_empty() {
                return kBadChar; // Or handle end of stream differently
            }

            let next = cursor[0];
            *cursor = &cursor[1..];

            if V8_LIKELY!(next <= kMaxOneByteChar as u8 && old_state == State::kAccept) {
                DCHECK_EQ!(0u32, buffer.get());
                return next as uchar;
            }

            Utf8DfaDecoder::decode(next, state, buffer);

            match *state {
                State::kAccept => {
                    let t = buffer.get() as uchar;
                    buffer.set(0);
                    return t;
                }
                State::kReject => {
                    *state = State::kAccept;
                    buffer.set(0);

                    if old_state != State::kAccept {
                        *cursor = &cursor[..cursor.len() + 1]; // Unconsume the byte
                    }
                    return kBadChar;
                }
                _ => {
                    return kBadChar; //kIncomplete;
                }
            }
        }

        pub fn encode_one_byte(str_: &mut [char], c: u8) -> usize {
            const kMask: i32 = !(1 << 6);
            if c <= kMaxOneByteChar as u8 {
                str_[0] = c as char;
                1
            } else {
                str_[0] = (0xC0 | (c >> 6)) as char;
                str_[1] = (0x80 | (c as i32 & kMask)) as char;
                2
            }
        }

        pub fn encode(
            str_: &mut [u8],
            c: uchar,
            previous: i32,
            replace_invalid: bool,
        ) -> usize {
            const kMask: i32 = !(1 << 6);
            if c <= kMaxOneByteChar {
                str_[0] = c as u8;
                1
            } else if c <= kMaxTwoByteChar {
                str_[0] = (0xC0 | (c >> 6)) as u8;
                str_[1] = (0x80 | (c as i32 & kMask)) as u8;
                2
            } else if c <= kMaxThreeByteChar {
                if is_surrogate_pair(previous, c) {
                    let kUnmatchedSize = kSizeOfUnmatchedSurrogate;
                    return Utf8::encode(
                        &mut str_[-kUnmatchedSize..],
                        combine_surrogate_pair(previous, c),
                        kNoPreviousCharacter,
                        replace_invalid,
                    ) - kUnmatchedSize;
                } else if replace_invalid && (is_lead_surrogate(c as i32) || is_trail_surrogate(c as i32)) {
                    let c = kBadChar;
                    str_[0] = (0xE0 | (c >> 12)) as u8;
                    str_[1] = (0x80 | ((c >> 6) & kMask as u32)) as u8;
                    str_[2] = (0x80 | (c & kMask as u32)) as u8;
                    3
                } else {
                    str_[0] = (0xE0 | (c >> 12)) as u8;
                    str_[1] = (0x80 | ((c >> 6) & kMask as u32)) as u8;
                    str_[2] = (0x80 | (c & kMask as u32)) as u8;
                    3
                }
            } else {
                str_[0] = (0xF0 | (c >> 18)) as u8;
                str_[1] = (0x80 | ((c >> 12) & kMask as u32)) as u8;
                str_[2] = (0x80 | ((c >> 6) & kMask as u32)) as u8;
                str_[3] = (0x80 | (c & kMask as u32)) as u8;
                4
            }
        }

        pub fn value_of(bytes: &[u8], length: usize, cursor: &mut usize) -> uchar {
            if length == 0 {
                return kBadChar;
            }
            let first = bytes[0];

            if V8_LIKELY!(first <= kMaxOneByteChar as u8) {
                *cursor += 1;
                return first as uchar;
            }
            Utf8::calculate_value(bytes, length, cursor)
        }

        fn calculate_value(bytes: &[u8], length: usize, cursor: &mut usize) -> uchar {
            if length < 2 {
                *cursor += 1;
                return kBadChar;
            }

            let first = bytes[0];

            if first < 0xE0 {
                if length < 2 {
                    *cursor += 1;
                    return kBadChar;
                }
                let second = bytes[1];
                if (second & 0xC0) != 0x80 {
                    *cursor += 1;
                    return kBadChar;
                }
                *cursor += 2;
                return ((first as u32 & 0x1F) << 6 | (second as u32 & 0x3F)) as uchar;
            }

            if first < 0xF0 {
                if length < 3 {
                    *cursor += 1;
                    return kBadChar;
                }
                let second = bytes[1];
                let third = bytes[2];
                if (second & 0xC0) != 0x80 || (third & 0xC0) != 0x80 {
                    *cursor += 1;
                    return kBadChar;
                }
                *cursor += 3;
                return ((first as u32 & 0x0F) << 12 | (second as u32 & 0x3F) << 6 | (third as u32 & 0x3F)) as uchar;
            }

            if first < 0xF8 {
                if length < 4 {
                    *cursor += 1;
                    return kBadChar;
                }
                let second = bytes[1];
                let third = bytes[2];
                let fourth = bytes[3];
                if (second & 0xC0) != 0x80 || (third & 0xC0) != 0x80 || (fourth & 0xC0) != 0x80 {
                    *cursor += 1;
                    return kBadChar;
                }
                *cursor += 4;
                return ((first as u32 & 0x07) << 18 | (second as u32 & 0x3F) << 12 | (third as u32 & 0x3F) << 6 | (fourth as u32 & 0x3F)) as uchar;
            }
            *cursor += 1;
            kBadChar
        }

        pub fn length_one_byte(c: u8) -> usize {
            if c <= kMaxOneByteChar as u8 {
                1
            } else {
                2
            }
        }

        pub fn length(c: uchar, previous: i32) -> usize {
            if c <= kMaxOneByteChar {
                1
            } else if c <= kMaxTwoByteChar {
                2
            } else if c <= kMaxThreeByteChar {
                if is_surrogate_pair(previous, c) {
                    kSizeOfUnmatchedSurrogate - kBytesSavedByCombiningSurrogates
                } else {
                    3
                }
            } else {
                4
            }
        }

        pub fn is_valid_character(c: uchar) -> bool {
            c < 0xD800 || (c >= 0xE000 && c < 0xFDD0) || (c > 0xFDEF && c <= 0x10FFFF && (c & 0xFFFE) != 0xFFFE && c != kBadChar)
        }

        pub fn encode_vec<Char: Into<u32> + Copy>(
            string: &[Char],
            buffer: &mut [u8],
            capacity: usize,
            write_null: bool,
            replace_invalid_utf8: bool,
        ) -> EncodingResult {
            let kSourceIsOneByte = std::mem::size_of::<Char>() == 1;

            let mut replace_invalid_utf8 = replace_invalid_utf8; // Make mutable

            if kSourceIsOneByte {
                replace_invalid_utf8 = false;
            }

            let mut write_index = 0;
            let content_capacity = capacity - if write_null { 1 } else { 0 };
            CHECK_LE!(content_capacity, capacity);
            let mut last = kNoPreviousCharacter;
            let mut read_index = 0;

            for &character in string.iter() {
                let character: u32 = character.into();
                let required_capacity;

                if kSourceIsOneByte {
                    required_capacity = Utf8::length_one_byte(character as u8);
                } else {
                    required_capacity = Utf8::length(character, last);
                }

                let remaining_capacity = content_capacity - write_index;
                if remaining_capacity < required_capacity {
                    if replace_invalid_utf8 && is_lead_surrogate(last) {
                        DCHECK_LE!(write_index, kSizeOfUnmatchedSurrogate);
                        write_index -= kSizeOfUnmatchedSurrogate;
                    }
                    break;
                }

                if kSourceIsOneByte {
                    let char_arr = &mut [0 as char; 10];
                    let written = Utf8::encode_one_byte(char_arr, character as u8);
                    for i in 0..written {
                        buffer[write_index+i] = char_arr[i] as u8;
                    }
                    write_index += written;
                } else {
                    let char_arr = &mut [0 as u8; 10];
                    let written = Utf8::encode(
                        char_arr,
                        character,
                        last,
                        replace_invalid_utf8,
                    );
                    for i in 0..written {
                        buffer[write_index+i] = char_arr[i];
                    }
                    write_index += written;

                }
                last = character as i32;
                read_index += 1;
            }

            DCHECK_LE!(write_index, capacity);

            if write_null {
                DCHECK_LE!(write_index, capacity);
                buffer[write_index] = 0;
                write_index += 1;
            }

            let bytes_written = write_index;
            let characters_processed = read_index;

            EncodingResult {
                bytes_written,
                characters_processed,
            }
        }
    }
}