// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod vlq_base64 {
    /// Decodes a VLQ-Base64 character to a digit (for testing purposes).
    ///
    /// This function is designed for testing and debugging VLQ-Base64 decoding.
    pub fn char_to_digit_decode_for_testing(c: u8) -> i8 {
        match c {
            b'A'..=b'Z' => (c - b'A') as i8,
            b'a'..=b'z' => (c - b'a' + 26) as i8,
            b'0'..=b'9' => (c - b'0' + 52) as i8,
            b'+' => 62,
            b'/' => 63,
            _ => -1, // Or handle invalid characters differently, e.g., return an error.
        }
    }

    /// Decodes a VLQ-Base64-encoded string into 32bit digits.
    /// A valid return value is within [-2^31+1, 2^31-1]. This function returns
    /// i32::MIN when bad input `start` is passed.
    ///
    /// # Arguments
    ///
    /// * `start` - A pointer to the beginning of the VLQ-Base64 encoded string.
    /// * `sz` - The size of the string.
    /// * `pos` - A mutable pointer to the current parsing position within the string.
    ///
    /// # Returns
    ///
    /// The decoded 32-bit integer, or `i32::MIN` if an error occurred.
    pub fn vlq_base64_decode(start: &str, sz: usize) -> (i32, usize) {
        let mut result: i32 = 0;
        let mut shift: i32 = 0;
        let mut pos: usize = 0;

        while pos < sz {
            let c = start.as_bytes()[pos];
            let digit = char_to_digit_decode_for_testing(c);

            if digit == -1 {
                return (i32::min_value(), pos);
            }

            result |= ((digit as i32) & 0x1F) << shift;
            shift += 5;
            pos += 1;

            if (digit & 0x20) == 0 {
                if (result & 1) != 0 {
                    return (-(result >> 1), pos);
                } else {
                    return (result >> 1, pos);
                }
            }
        }

        i32::min_value() // Indicate incomplete VLQ sequence.  This behavior is aligned with original code.
    }
}