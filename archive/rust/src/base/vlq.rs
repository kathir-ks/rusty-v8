// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod vlq {
    use std::convert::TryInto;

    const CONTINUE_SHIFT: u32 = 7;
    const CONTINUE_BIT: u32 = 1 << CONTINUE_SHIFT;
    const DATA_MASK: u32 = CONTINUE_BIT - 1;

    /// Encodes an unsigned value using variable-length encoding and stores it using
    /// the passed process_byte function.
    #[inline]
    pub fn vlq_encode_unsigned<F>(mut process_byte: F, value: u32)
    where
        F: FnMut(u8),
    {
        // Write as many bytes as necessary to encode the value, with 7 bits of data
        // per byte (leaving space for one continuation bit).
        const DATA_BITS_PER_BYTE: u32 = CONTINUE_SHIFT;
        if value < 1 << (DATA_BITS_PER_BYTE) {
            return write_one_byte(process_byte, value);
        }
        if value < 1 << (2 * DATA_BITS_PER_BYTE) {
            return write_two_bytes(process_byte, value);
        }
        if value < 1 << (3 * DATA_BITS_PER_BYTE) {
            return write_three_bytes(process_byte, value);
        }
        if value < 1 << (4 * DATA_BITS_PER_BYTE) {
            return write_four_bytes(process_byte, value);
        }

        process_byte((value | CONTINUE_BIT) as u8);
        let value = value >> CONTINUE_SHIFT;

        write_four_bytes(process_byte, value);

        fn write_four_bytes<F: FnMut(u8)>(mut process_byte: F, mut value: u32) {
            process_byte((value | CONTINUE_BIT) as u8);
            value >>= CONTINUE_SHIFT;
            write_three_bytes(process_byte, value);
        }

        fn write_three_bytes<F: FnMut(u8)>(mut process_byte: F, mut value: u32) {
            process_byte((value | CONTINUE_BIT) as u8);
            value >>= CONTINUE_SHIFT;
            write_two_bytes(process_byte, value);
        }

        fn write_two_bytes<F: FnMut(u8)>(mut process_byte: F, mut value: u32) {
            process_byte((value | CONTINUE_BIT) as u8);
            value >>= CONTINUE_SHIFT;
            write_one_byte(process_byte, value);
        }

        fn write_one_byte<F: FnMut(u8)>(mut process_byte: F, value: u32) {
            // The last value written doesn't need a continuation bit.
            process_byte(value as u8);
        }
    }

    #[inline]
    pub fn vlq_convert_to_unsigned(value: i32) -> u32 {
        assert_ne!(value, i32::min_value());
        let is_negative = value < 0;
        // Encode sign in least significant bit.
        let bits = ((if is_negative { -value } else { value }) as u32 * 2) | (is_negative as u32);
        bits
    }

    /// Encodes value using variable-length encoding and stores it using the passed
    /// process_byte function.
    #[inline]
    pub fn vlq_encode<F>(mut process_byte: F, value: i32)
    where
        F: FnMut(u8),
    {
        let bits = vlq_convert_to_unsigned(value);
        vlq_encode_unsigned(process_byte, bits);
    }

    /// Wrapper of vlq_encode for Vec backed storage containers.
    #[inline]
    pub fn vlq_encode_vec(data: &mut Vec<u8>, value: i32) {
        vlq_encode(|byte| data.push(byte), value);
    }

    /// Wrapper of vlq_encode_unsigned for Vec backed storage containers.
    #[inline]
    pub fn vlq_encode_unsigned_vec(data: &mut Vec<u8>, value: u32) {
        vlq_encode_unsigned(|byte| data.push(byte), value);
    }

    /// Decodes a variable-length encoded unsigned value from bytes returned by
    /// successive calls to the given function.
    #[inline]
    pub fn vlq_decode_unsigned<F>(mut get_next: F) -> u32
    where
        F: FnMut() -> u8,
    {
        let cur_byte = get_next();
        // Single byte fast path; no need to mask.
        if cur_byte <= DATA_MASK as u8 {
            return cur_byte as u32;
        }
        let mut bits = (cur_byte & DATA_MASK as u8) as u32;
        let mut shift = CONTINUE_SHIFT;

        loop {
            let cur_byte = get_next();
            bits |= ((cur_byte & DATA_MASK as u8) as u32) << shift;
            if cur_byte <= DATA_MASK as u8 {
                break;
            }
            shift += CONTINUE_SHIFT;
            if shift > 32 {
                break; // prevent infinite loop. Original C++ code does not prevent this
            }
        }

        bits
    }

    /// Decodes a variable-length encoded unsigned value stored in contiguous memory
    /// starting at data_start + index, updating index to where the next encoded
    /// value starts.
    #[inline]
    pub fn vlq_decode_unsigned_slice(data_start: &[u8], index: &mut usize) -> u32 {
        let mut local_index = *index;
        let result = vlq_decode_unsigned(|| {
            let value = data_start[local_index];
            local_index += 1;
            value
        });
        *index = local_index;
        result
    }

    /// Decodes a variable-length encoded value stored in contiguous memory starting
    /// at data_start + index, updating index to where the next encoded value starts.
    #[inline]
    pub fn vlq_decode_slice(data_start: &[u8], index: &mut usize) -> i32 {
        let bits = vlq_decode_unsigned_slice(data_start, index);
        let is_negative = (bits & 1) == 1;
        let result = (bits >> 1) as i32;
        if is_negative {
            -result
        } else {
            result
        }
    }
}