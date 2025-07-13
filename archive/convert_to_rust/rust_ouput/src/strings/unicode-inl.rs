// Converted from V8 C++ source files:
// Header: unicode-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod unibrow {
    use crate::State;
    use std::fmt;

    pub const kBadChar: u16 = 0xFFFD;
    pub const kMaxOneByteChar: u16 = 0x7F;
    pub const kMaxTwoByteChar: u16 = 0x7FF;
    pub const kMaxThreeByteChar: u16 = 0xFFFF;
    pub const kNoPreviousCharacter: i32 = -1;
    pub const kSizeOfUnmatchedSurrogate: usize = 3;
    pub const kBytesSavedByCombiningSurrogates: usize = 1;

    pub trait UnibrowPredicate {
        fn is(code_point: u16) -> bool;
    }

    pub struct Predicate<T: UnibrowPredicate, const S: usize> {
        entries_: [CacheEntry; 256], // Assuming 256 is a reasonable size
    }

    impl<T: UnibrowPredicate, const S: usize> Predicate<T, S> {
        const kMask: usize = 255; // Assuming 255 is a reasonable mask

        pub fn new() -> Self {
            Predicate {
                entries_: [CacheEntry {
                    code_point_: 0,
                    value_: false,
                }; 256],
            }
        }

        pub fn get(&self, code_point: u8) -> bool {
            let entry = self.entries_[code_point as usize & Self::kMask];
            if entry.code_point() == code_point as u16 {
                return entry.value();
            }
            self.calculate_value(code_point)
        }

        fn calculate_value(&self, code_point: u8) -> bool {
            let result = T::is(code_point as u16);
            let mut entry = CacheEntry::new(code_point as u16, result);
            self.entries_[code_point as usize & Self::kMask] = entry;
            result
        }
    }

    #[derive(Clone, Copy)]
    struct CacheEntry {
        code_point_: u16,
        value_: bool,
    }

    impl CacheEntry {
        fn new(code_point: u16, value: bool) -> Self {
            CacheEntry {
                code_point_: code_point,
                value_: value,
            }
        }

        fn code_point(&self) -> u16 {
            self.code_point_
        }

        fn value(&self) -> bool {
            self.value_
        }
    }

    pub trait UnibrowMapping {
        fn convert(c: u8, n: u8, result: &mut [u8], allow_caching: &mut bool) -> i32;
    }

    pub struct Mapping<T: UnibrowMapping, const S: usize> {
        entries_: [CacheEntryMapping; 256], // Assuming 256 is a reasonable size
    }

    impl<T: UnibrowMapping, const S: usize> Mapping<T, S> {
        const kMask: usize = 255; // Assuming 255 is a reasonable mask

        pub fn new() -> Self {
            Mapping {
                entries_: [CacheEntryMapping {
                    code_point_: 0,
                    offset_: 0,
                }; 256],
            }
        }

        pub fn get(&self, c: u8, n: u8, result: &mut [u8]) -> i32 {
            let entry = self.entries_[c as usize & Self::kMask];
            if entry.code_point() == c {
                if entry.offset() == 0 {
                    return 0;
                } else {
                    result[0] = (c as i32 + entry.offset()) as u8;
                    return 1;
                }
            } else {
                self.calculate_value(c, n, result)
            }
        }

        fn calculate_value(&self, c: u8, n: u8, result: &mut [u8]) -> i32 {
            let mut allow_caching = true;
            let length = T::convert(c, n, result, &mut allow_caching);
            if allow_caching {
                if length == 1 {
                    let offset = result[0] as i32 - c as i32;
                    self.entries_[c as usize & Self::kMask] =
                        CacheEntryMapping::new(c, offset);
                    return 1;
                } else {
                    self.entries_[c as usize & Self::kMask] = CacheEntryMapping::new(c, 0);
                    return 0;
                }
            } else {
                length
            }
        }
    }

    #[derive(Clone, Copy)]
    struct CacheEntryMapping {
        code_point_: u8,
        offset_: i32,
    }

    impl CacheEntryMapping {
        fn new(code_point: u8, offset: i32) -> Self {
            CacheEntryMapping {
                code_point_: code_point,
                offset_: offset,
            }
        }

        fn code_point(&self) -> u8 {
            self.code_point_
        }

        fn offset(&self) -> i32 {
            self.offset_
        }
    }

    pub struct Utf16 {}

    impl Utf16 {
        pub fn is_lead_surrogate(code_unit: i32) -> bool {
            (code_unit >= 0xD800) && (code_unit <= 0xDBFF)
        }

        pub fn is_trail_surrogate(code_unit: i32) -> bool {
            (code_unit >= 0xDC00) && (code_unit <= 0xDFFF)
        }

        pub fn combine_surrogate_pair(lead: i32, trail: i32) -> u16 {
            (((lead - 0xD800) << 10) + (trail - 0xDC00) + 0x10000) as u16
        }

        pub fn has_unpaired_surrogate(code_units: &[u16], length: usize) -> bool {
            let mut i = 0;
            while i < length {
                let code_unit = code_units[i] as i32;
                if Self::is_lead_surrogate(code_unit) {
                    if i == length - 1 {
                        return true;
                    }
                    if !Self::is_trail_surrogate(code_units[i + 1] as i32) {
                        return true;
                    }
                    i += 1;
                } else if Self::is_trail_surrogate(code_unit) {
                    return true;
                }
                i += 1;
            }
            false
        }
    }

    pub struct Utf8 {}

    pub type Uchar = u16;

    #[derive(Debug, PartialEq, Eq)]
    pub enum Utf8Error {
        InvalidByteSequence,
        IncompleteByteSequence,
    }

    impl fmt::Display for Utf8Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Utf8Error::InvalidByteSequence => write!(f, "Invalid UTF-8 byte sequence"),
                Utf8Error::IncompleteByteSequence => write!(f, "Incomplete UTF-8 byte sequence"),
            }
        }
    }

    impl Utf8 {
        pub fn value_of_incremental(
            cursor: &mut &[u8],
            state: &mut State,
            buffer: &mut Utf8IncrementalBuffer,
        ) -> Uchar {
            let old_state = *state;
            if cursor.is_empty() {
                return kBadChar;
            }
            let next = cursor[0];
            *cursor = &cursor[1..];

            if next <= kMaxOneByteChar as u8 && old_state == State::kAccept {
                *buffer = 0;
                return next as Uchar;
            }

            Utf8DfaDecoder::decode(next, state, buffer);

            match *state {
                State::kAccept => {
                    let t = *buffer as Uchar;
                    *buffer = 0;
                    return t;
                }

                State::kReject => {
                    *state = State::kAccept;
                    *buffer = 0;

                    if old_state != State::kAccept {
                        *cursor = &cursor[..];
                    }
                    kBadChar
                }

                _ => kBadChar, // Incomplete is handled by the caller.
            }
        }

        pub fn encode_one_byte(str_: &mut [u8], c: u8) -> usize {
            const K_MASK: i32 = !(1 << 6);
            if c <= kMaxOneByteChar as u8 {
                str_[0] = c;
                1
            } else {
                str_[0] = 0xC0 | (c >> 6);
                str_[1] = 0x80 | (c as i32 & K_MASK) as u8;
                2
            }
        }

        pub fn encode(
            str_: &mut [u8],
            c: Uchar,
            previous: i32,
            replace_invalid: bool,
        ) -> usize {
            const K_MASK: i32 = !(1 << 6);
            if c <= kMaxOneByteChar {
                str_[0] = c as u8;
                1
            } else if c <= kMaxTwoByteChar {
                str_[0] = 0xC0 | (c >> 6) as u8;
                str_[1] = 0x80 | (c as i32 & K_MASK) as u8;
                2
            } else if c <= kMaxThreeByteChar {
                if Utf16::is_surrogate_pair(previous, c as i32) {
                    let unmatched_size = kSizeOfUnmatchedSurrogate;
                    return Self::encode(
                        &mut str_[-unmatched_size..],
                        Utf16::combine_surrogate_pair(previous, c as i32),
                        kNoPreviousCharacter,
                        replace_invalid,
                    ) - unmatched_size;
                } else if replace_invalid && (Utf16::is_lead_surrogate(c as i32) || Utf16::is_trail_surrogate(c as i32)) {
                    Self::encode(str_, kBadChar, previous, replace_invalid)
                } else {
                    str_[0] = 0xE0 | (c >> 12) as u8;
                    str_[1] = 0x80 | ((c >> 6) as i32 & K_MASK) as u8;
                    str_[2] = 0x80 | (c as i32 & K_MASK) as u8;
                    3
                }
            } else {
                str_[0] = 0xF0 | (c >> 18) as u8;
                str_[1] = 0x80 | ((c >> 12) as i32 & K_MASK) as u8;
                str_[2] = 0x80 | ((c >> 6) as i32 & K_MASK) as u8;
                str_[3] = 0x80 | (c as i32 & K_MASK) as u8;
                4
            }
        }

        pub fn value_of(bytes: &[u8], length: usize, cursor: &mut usize) -> Uchar {
            if length == 0 {
                return kBadChar;
            }
            let first = bytes[0];
            if first <= kMaxOneByteChar as u8 {
                *cursor += 1;
                return first as Uchar;
            }
            Self::calculate_value(bytes, length, cursor)
        }

        fn calculate_value(bytes: &[u8], length: usize, cursor: &mut usize) -> Uchar {
            if length < 2 {
                return kBadChar;
            }

            let first = bytes[0];

            if (first & 0xE0) == 0xC0 {
                if length < 2 {
                    return kBadChar; // Incomplete
                }
                let second = bytes[1];
                if (second & 0xC0) != 0x80 {
                    *cursor += 1;
                    return kBadChar; // Invalid
                }
                let code_point = ((first as u16 & 0x1F) << 6) | (second as u16 & 0x3F);
                if code_point <= kMaxOneByteChar {
                    *cursor += 1;
                    return kBadChar;
                }
                *cursor += 2;
                return code_point;
            } else if (first & 0xF0) == 0xE0 {
                if length < 3 {
                    return kBadChar; // Incomplete
                }
                let second = bytes[1];
                let third = bytes[2];
                if (second & 0xC0) != 0x80 || (third & 0xC0) != 0x80 {
                    *cursor += 1;
                    return kBadChar; // Invalid
                }
                let code_point = ((first as u16 & 0x0F) << 12)
                    | ((second as u16 & 0x3F) << 6)
                    | (third as u16 & 0x3F);
                if code_point <= kMaxTwoByteChar {
                    *cursor += 1;
                    return kBadChar;
                }
                *cursor += 3;
                return code_point;
            } else if (first & 0xF8) == 0xF0 {
                if length < 4 {
                    return kBadChar; // Incomplete
                }
                let second = bytes[1];
                let third = bytes[2];
                let fourth = bytes[3];
                if (second & 0xC0) != 0x80 || (third & 0xC0) != 0x80 || (fourth & 0xC0) != 0x80 {
                    *cursor += 1;
                    return kBadChar; // Invalid
                }
                let code_point = ((first as u16 & 0x07) << 18)
                    | ((second as u16 & 0x3F) << 12)
                    | ((third as u16 & 0x3F) << 6)
                    | (fourth as u16 & 0x3F);
                if code_point <= kMaxThreeByteChar {
                    *cursor += 1;
                    return kBadChar;
                }
                *cursor += 4;
                return code_point;
            } else {
                *cursor += 1;
                return kBadChar; // Invalid
            }
        }

        pub fn length_one_byte(c: u8) -> usize {
            if c <= kMaxOneByteChar as u8 {
                1
            } else {
                2
            }
        }

        pub fn length(c: Uchar, previous: i32) -> usize {
            if c <= kMaxOneByteChar {
                1
            } else if c <= kMaxTwoByteChar {
                2
            } else if c <= kMaxThreeByteChar {
                if Utf16::is_surrogate_pair(previous, c as i32) {
                    kSizeOfUnmatchedSurrogate - kBytesSavedByCombiningSurrogates
                } else {
                    3
                }
            } else {
                4
            }
        }

        pub fn is_valid_character(c: Uchar) -> bool {
            c < 0xD800 || (c >= 0xE000 && c < 0xFDD0) || (c > 0xFDEF && c <= 0x10FFFF && (c & 0xFFFE) != 0xFFFE && c != kBadChar)
        }

        pub fn encode_str(s: &str) -> Vec<u8> {
            let mut result = Vec::new();
            for c in s.chars() {
                let mut buf = [0u8; 4];
                let len = c.encode_utf8(&mut buf).len();
                result.extend_from_slice(&buf[..len]);
            }
            result
        }

        pub fn decode_str(bytes: &[u8]) -> String {
            String::from_utf8_lossy(bytes).to_string()
        }

        pub fn encode<Char>(
            string: v8::base::Vector<const Char>,
            buffer: &mut [char],
            capacity: usize,
            write_null: bool,
            replace_invalid_utf8: bool,
        ) -> EncodingResult
        where
            Char: Copy,
            u8: TryFrom<Char>,
        {
            let k_source_is_one_byte = std::mem::size_of::<Char>() == 1;

            let mut replace_invalid_utf8_local = replace_invalid_utf8;

            if k_source_is_one_byte {
                replace_invalid_utf8_local = false;
            }

            let mut write_index = 0;
            let characters = string.begin();
            let content_capacity = capacity - if write_null { 1 } else { 0 };

            let mut last: u16 = Utf16::kNoPreviousCharacter as u16;
            let mut read_index = 0;
            while read_index < string.size() {
                let character = characters[read_index];
                let character_u8: u8 = match TryFrom::try_from(character) {
                    Ok(c) => c,
                    Err(_) => {
                        return EncodingResult {
                            bytes_written: write_index,
                            characters_processed: read_index,
                        }
                    }
                };

                let required_capacity;
                if k_source_is_one_byte {
                    required_capacity = Utf8::length_one_byte(character_u8);
                } else {
                    required_capacity = Utf8::length(character_u8 as u16, last as i32);
                }

                let remaining_capacity = content_capacity - write_index;
                if remaining_capacity < required_capacity {
                    if replace_invalid_utf8_local && Utf16::is_lead_surrogate(last as i32) {
                        assert!(write_index >= kSizeOfUnmatchedSurrogate);
                        write_index -= kSizeOfUnmatchedSurrogate;
                    }
                    break;
                }

                if k_source_is_one_byte {
                    write_index += Utf8::encode_one_byte(
                        &mut unsafe { std::mem::transmute::<&mut [char], &mut [u8]>(buffer) }
                            [write_index..],
                        character_u8,
                    );
                } else {
                   // write_index += Utf8::encode(
                   //     &mut unsafe { std::mem::transmute::<&mut [char], &mut [u8]>(buffer) }
                   //         [write_index..],
                   //     character_u8 as Uchar,
                   //     last as i32,
                   //     replace_invalid_utf8_local,
                   // );
                }

                last = character_u8 as u16;
                read_index += 1;
            }
            assert!(write_index <= capacity);
            if write_null {
                assert!(write_index < capacity);
                buffer[write_index] = '\0';
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

    pub struct EncodingResult {
        pub bytes_written: usize,
        pub characters_processed: usize,
    }

    pub type Utf8IncrementalBuffer = u32;

    // Mock implementation for Utf8DfaDecoder
    pub struct Utf8DfaDecoder {}

    impl Utf8DfaDecoder {
        pub fn decode(next: u8, state: &mut State, buffer: &mut Utf8IncrementalBuffer) {
            match *state {
                State::kAccept => {
                    if (next & 0x80) == 0x00 {
                        *buffer = next as u32;
                        *state = State::kAccept;
                    } else if (next & 0xE0) == 0xC0 {
                        *buffer = (next as u32 & 0x1F) << 6;
                        *state = State::kUtf82;
                    } else if (next & 0xF0) == 0xE0 {
                        *buffer = (next as u32 & 0x0F) << 12;
                        *state = State::kUtf83;
                    } else if (next & 0xF8) == 0xF0 {
                        *buffer = (next as u32 & 0x07) << 18;
                        *state = State::kUtf84;
                    } else {
                        *state = State::kReject;
                    }
                }
                State::kUtf82 => {
                    if (next & 0xC0) == 0x80 {
                        *buffer |= (next as u32 & 0x3F);
                        *state = State::kAccept;
                    } else {
                        *state = State::kReject;
                    }
                }
                State::kUtf83 => {
                    if (next & 0xC0) == 0x80 {
                        *buffer |= (next as u32 & 0x3F) << 6;
                        *state = State::kUtf8_3cont;
                    } else {
                        *state = State::kReject;
                    }
                }
                State::kUtf84 => {
                    if (next & 0xC0) == 0x80 {
                        *buffer |= (next as u32 & 0x3F) << 12;
                        *state = State::kUtf8_4cont2;
                    } else {
                        *state = State::kReject;
                    }
                }
                State::kUtf8_3cont => {
                    if (next & 0xC0) == 0x80 {
                        *buffer |= (next as u32 & 0x3F);
                        *state = State::kAccept;
                    } else {
                        *state = State::kReject;
                    }
                }
                State::kUtf8_4cont2 => {
                    if (next & 0xC0) == 0x80 {
                        *buffer |= (next as u32 & 0x3F) << 6;
                        *state = State::kUtf8_4cont3;
                    } else {
                        *state = State::kReject;
                    }
                }
                State::kUtf8_4cont3 => {
                    if (next & 0xC0) == 0x80 {
                        *buffer |= (next as u32 & 0x3F);
                        *state = State::kAccept;
                    } else {
                        *state = State::kReject;
                    }
                }
                _ => *state = State::kReject, // Handle other states, reject by default.
            }
        }
    }
} // namespace unibrow
pub mod v8 {
    pub mod base {
        pub struct Vector<T> {
            ptr: *const T,
            size: usize,
        }

        impl<T> Vector<T> {
            pub fn new(ptr: *const T, size: usize) -> Self {
                Vector { ptr, size }
            }

            pub fn begin(&self) -> *const T {
                self.ptr
            }

            pub fn size(&self) -> usize {
                self.size
            }
        }
    }
}
