// Converted from V8 C++ source files:
// Header: v8-string-conversions.h
// Implementation: v8-string-conversions.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::char;

// Conversion routines between UT8 and UTF16, used by string-16.{h,cc}. You may
// want to use string-16.h directly rather than these.
pub mod v8_inspector {
    use std::string::String;
    use std::char;

    pub fn utf8_to_utf16(string_start: &str) -> Vec<u16> {
        string_start.encode_utf16().collect()
    }

    pub fn utf16_to_utf8(string_start: &[u16]) -> String {
        char::decode_utf16(string_start.iter().cloned())
            .map(|r| r.unwrap_or(char::REPLACEMENT_CHARACTER))
            .collect::<String>()
    }
}

// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod v8_inspector_impl {
    use super::v8_inspector;
    use std::string::String;
    use std::convert::TryInto;

    #[derive(Debug, PartialEq)]
    enum ConversionResult {
        ConversionOk,
        SourceExhausted,
        TargetExhausted,
        SourceIllegal,
    }

    fn is_ascii(c: u16) -> bool {
        !(c & !0x7F) != 0
    }

    const REPLACEMENT_CHARACTER: u16 = 0xFFFD;

    fn inline_utf8_sequence_length_non_ascii(b0: u8) -> i32 {
        if (b0 & 0xC0) != 0xC0 {
            return 0;
        }
        if (b0 & 0xE0) == 0xC0 {
            return 2;
        }
        if (b0 & 0xF0) == 0xE0 {
            return 3;
        }
        if (b0 & 0xF8) == 0xF0 {
            return 4;
        }
        0
    }

    fn inline_utf8_sequence_length(b0: u8) -> i32 {
        if is_ascii(b0 as u16) {
            1
        } else {
            inline_utf8_sequence_length_non_ascii(b0)
        }
    }

    // Once the bits are split out into bytes of UTF-8, this is a mask OR-ed
    // into the first byte, depending on how many bytes follow.  There are
    // as many entries in this table as there are UTF-8 sequence types.
    // (I.e., one byte sequence, two byte... etc.). Remember that sequences
    // for *legal* UTF-8 will be 4 or fewer bytes total.
    const FIRST_BYTE_MARK: [u8; 7] = [0x00, 0x00, 0xC0, 0xE0, 0xF0, 0xF8, 0xFC];

    fn convert_utf16_to_utf8(
        source_start: &mut &[u16],
        source_end: &[u16],
        target_start: &mut &mut [u8],
        target_end: &mut [u8],
        strict: bool,
    ) -> ConversionResult {
        let mut result = ConversionResult::ConversionOk;
        let mut source = *source_start;
        let mut target = *target_start;

        while !source.is_empty() {
            let mut ch: u32;
            let mut bytes_to_write: u32 = 0;
            let byte_mask: u32 = 0xBF;
            let byte_mark: u32 = 0x80;

            let old_source = source;

            ch = source[0] as u32;
            source = &source[1..];

            // If we have a surrogate pair, convert to UChar32 first.
            if (ch >= 0xD800) && (ch <= 0xDBFF) {
                // If the 16 bits following the high surrogate are in the source buffer...
                if !source.is_empty() {
                    let ch2 = source[0] as u32;
                    // If it's a low surrogate, convert to UChar32.
                    if (ch2 >= 0xDC00) && (ch2 <= 0xDFFF) {
                        ch = ((ch - 0xD800) << 10) + (ch2 - 0xDC00) + 0x0010000;
                        source = &source[1..];
                    } else if strict {
                        // it's an unpaired high surrogate
                        source = old_source; // return to the illegal value itself
                        result = ConversionResult::SourceIllegal;
                        break;
                    }
                } else {
                    // We don't have the 16 bits following the high surrogate.
                    source = old_source; // return to the high surrogate
                    result = ConversionResult::SourceExhausted;
                    break;
                }
            } else if strict {
                // UTF-16 surrogate values are illegal in UTF-32
                if (ch >= 0xDC00) && (ch <= 0xDFFF) {
                    source = old_source; // return to the illegal value itself
                    result = ConversionResult::SourceIllegal;
                    break;
                }
            }

            // Figure out how many bytes the result will require
            if ch < 0x80 {
                bytes_to_write = 1;
            } else if ch < 0x800 {
                bytes_to_write = 2;
            } else if ch < 0x10000 {
                bytes_to_write = 3;
            } else if ch < 0x110000 {
                bytes_to_write = 4;
            } else {
                bytes_to_write = 3;
                ch = REPLACEMENT_CHARACTER as u32;
            }

            if (target.len() < bytes_to_write as usize) {
                source = old_source;
                result = ConversionResult::TargetExhausted;
                break;
            }

            let mut target_temp = target;
            target = &mut target_temp[bytes_to_write as usize..];

            match bytes_to_write {
                4 => {
                    target_temp[3] = ((ch | byte_mark) & byte_mask) as u8;
                    ch >>= 6;
                    target_temp[2] = ((ch | byte_mark) & byte_mask) as u8;
                    ch >>= 6;
                    target_temp[1] = ((ch | byte_mark) & byte_mask) as u8;
                    ch >>= 6;
                    target_temp[0] = (ch | FIRST_BYTE_MARK[bytes_to_write as usize]) as u8;
                }
                3 => {
                    target_temp[2] = ((ch | byte_mark) & byte_mask) as u8;
                    ch >>= 6;
                    target_temp[1] = ((ch | byte_mark) & byte_mask) as u8;
                    ch >>= 6;
                    target_temp[0] = (ch | FIRST_BYTE_MARK[bytes_to_write as usize]) as u8;
                }
                2 => {
                    target_temp[1] = ((ch | byte_mark) & byte_mask) as u8;
                    ch >>= 6;
                    target_temp[0] = (ch | FIRST_BYTE_MARK[bytes_to_write as usize]) as u8;
                }
                1 => {
                    target_temp[0] = (ch | FIRST_BYTE_MARK[bytes_to_write as usize]) as u8;
                }
                _ => {}
            }
        }

        *source_start = source;
        *target_start = target;

        result
    }

    const U_IS_BMP_MASK: u32 = 0xFFFF;
    const U_IS_SUPPLEMENTARY_MASK: u32 = 0xFFFFF;

    fn is_legal_utf8(source: &[u8], length: i32) -> bool {
        let mut srcptr = source;

        if length > 4 {
            return false;
        }

        match length {
            4 => {
                let a = srcptr[2];
                if a < 0x80 || a > 0xBF {
                    return false;
                }
            }
            3 => {
                let a = srcptr[1];
                if a < 0x80 || a > 0xBF {
                    return false;
                }
            }
            2 => {
                let a = srcptr[0];
                if a > 0xBF {
                    return false;
                }

                match source[0] {
                    0xE0 => {
                        if a < 0xA0 {
                            return false;
                        }
                    }
                    0xED => {
                        if a > 0x9F {
                            return false;
                        }
                    }
                    0xF0 => {
                        if a < 0x90 {
                            return false;
                        }
                    }
                    0xF4 => {
                        if a > 0x8F {
                            return false;
                        }
                    }
                    _ => {
                        if a < 0x80 {
                            return false;
                        }
                    }
                }
            }
            1 => {
                if source[0] >= 0x80 && source[0] < 0xC2 {
                    return false;
                }
            }
            _ => return false,
        }

        if source[0] > 0xF4 {
            return false;
        }
        true
    }

    const OFFSETS_FROM_UTF8: [u32; 6] = [0x00000000, 0x00003080, 0x000E2080, 0x03C82080, 0xFA082080, 0x82082080];

    fn read_utf8_sequence(sequence: &mut &[u8], length: i32) -> u32 {
        let mut character: u32 = 0;
        let mut seq = *sequence;

        match length {
            6 => {
                character += seq[0] as u32;
                character <<= 6;
                seq = &seq[1..];
            }
            5 => {
                character += seq[0] as u32;
                character <<= 6;
                seq = &seq[1..];
            }
            4 => {
                character += seq[0] as u32;
                character <<= 6;
                seq = &seq[1..];
            }
            3 => {
                character += seq[0] as u32;
                character <<= 6;
                seq = &seq[1..];
            }
            2 => {
                character += seq[0] as u32;
                character <<= 6;
                seq = &seq[1..];
            }
            1 => {
                character += seq[0] as u32;
                seq = &seq[1..];
            }
            _ => {}
        }

        *sequence = seq;
        character - OFFSETS_FROM_UTF8[length as usize - 1]
    }

    fn convert_utf8_to_utf16(
        source_start: &mut &[u8],
        source_end: &[u8],
        target_start: &mut &mut [u16],
        target_end: &mut [u16],
        source_all_ascii: Option<&mut bool>,
        strict: bool,
    ) -> ConversionResult {
        let mut result = ConversionResult::ConversionOk;
        let mut source = *source_start;
        let mut target = *target_start;
        let mut or_all_data: u16 = 0;

        while !source.is_empty() {
            let utf8_sequence_length = inline_utf8_sequence_length(source[0] as u8);

            if (source_end.len() as isize) < (source.len() as isize) {
              result = ConversionResult::SourceExhausted;
              break;
            }
            let source_len = source.len() as i32;
            if source_len < utf8_sequence_length {
                result = ConversionResult::SourceExhausted;
                break;
            }
            if !is_legal_utf8(source, utf8_sequence_length) {
                result = ConversionResult::SourceIllegal;
                break;
            }

            let mut source_copy = source;
            let character = read_utf8_sequence(&mut source_copy, utf8_sequence_length);
            source = &source[utf8_sequence_length as usize..];

            if target.is_empty() {
                source = &source[-(utf8_sequence_length as isize) as usize..];
                result = ConversionResult::TargetExhausted;
                break;
            }

            if character <= 0xFFFF {
                if (character >= 0xD800) && (character <= 0xDFFF) {
                    if strict {
                        source = &source[-(utf8_sequence_length as isize) as usize..];
                        result = ConversionResult::SourceIllegal;
                        break;
                    }

                    target[0] = REPLACEMENT_CHARACTER;
                    or_all_data |= REPLACEMENT_CHARACTER;
                    target = &mut target[1..];
                } else {
                    target[0] = character as u16;
                    or_all_data |= character as u16;
                    target = &mut target[1..];
                }
            } else if character <= 0x10FFFF {
                if target.len() < 2 {
                    source = &source[-(utf8_sequence_length as isize) as usize..];
                    result = ConversionResult::TargetExhausted;
                    break;
                }
                target[0] = (((character >> 10) & 0x3FF) + 0xD800) as u16;
                target[1] = ((character & 0x3FF) + 0xDC00) as u16;
                or_all_data = 0xFFFF;
                target = &mut target[2..];
            } else {
                if strict {
                    source = &source[-(utf8_sequence_length as isize) as usize..];
                    result = ConversionResult::SourceIllegal;
                    break;
                } else {
                    target[0] = REPLACEMENT_CHARACTER;
                    or_all_data |= REPLACEMENT_CHARACTER;
                    target = &mut target[1..];
                }
            }
        }

        *source_start = source;
        *target_start = target;

        if let Some(mut all_ascii) = source_all_ascii {
            *all_ascii = (or_all_data & !0x7F) == 0;
        }

        result
    }

    fn put_utf8_triple(buffer: &mut &mut [u8], ch: u16) {
        let mut buf = *buffer;
        buf[0] = (((ch >> 12) & 0x0F) | 0xE0) as u8;
        buf[1] = (((ch >> 6) & 0x3F) | 0x80) as u8;
        buf[2] = ((ch & 0x3F) | 0x80) as u8;
        *buffer = &mut buf[3..];
    }

    pub fn utf16_to_utf8(string_start: &[u16], length: usize) -> String {
        if string_start.is_empty() || length == 0 {
            return String::new();
        }

        if length > (usize::MAX / 3) {
            return String::new();
        }

        let mut output: Vec<u8> = vec![0; length * 3];
        let mut characters = string_start;
        let characters_end = &string_start[length..];
        let mut buffer = &mut output[..];
        let buffer_end = &mut buffer[length*3..];
        let mut characters_temp = characters;

        while !characters_temp.is_empty() {
            let mut buffer_temp = buffer;
            let conversion_result = convert_utf16_to_utf8(
                &mut characters_temp,
                characters_end,
                &mut buffer_temp,
                buffer_end,
                true,
            );
            characters = characters_temp;
            buffer = buffer_temp;

            if conversion_result != ConversionResult::ConversionOk {
                if string_start[0] >= 0xD800 && string_start[0] <= 0xDFFF {
                    if buffer.len() < 3 {
                      continue;
                    }
                    put_utf8_triple(&mut buffer, REPLACEMENT_CHARACTER);
                    characters = &characters[1..];
                }
            }
        }

        output.resize(output.len() - buffer.len());
        String::from_utf8(output).unwrap()
    }

    pub fn utf8_to_utf16(string_start: &str, length: usize) -> Vec<u16> {
        if string_start.is_empty() || length == 0 {
            return Vec::new();
        }

        let mut buffer: Vec<u16> = vec![0; length];
        let mut buffer_start = buffer.as_mut_slice();

        let string_start_bytes = string_start.as_bytes();
        let mut string_current = string_start_bytes;
        let string_end = &string_start_bytes[length..];

        let mut buff_cur = buffer_start;
        let conversion_result = convert_utf8_to_utf16(
          &mut string_current,
            string_end,
            &mut buff_cur,
            buffer_start,
            None,
            true,
        );

        if conversion_result != ConversionResult::ConversionOk {
            return Vec::new();
        }
        let utf16_length = buffer_start.len() - buff_cur.len();
        buffer[..utf16_length].to_vec()
    }
}
