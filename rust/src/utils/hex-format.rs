// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod hex_format {
    /// Formats a byte array into a hexadecimal string.
    ///
    /// # Arguments
    ///
    /// * `formatted` - A mutable slice of characters where the formatted hexadecimal string will be written.
    /// * `val` - A slice of bytes to be formatted.
    ///
    /// # Panics
    ///
    /// Panics if the length of `val` is greater than or equal to 0x20000000.
    /// Panics if the length of `formatted` is less than twice the length of `val`.
    pub fn format_bytes_to_hex(formatted: &mut [char], val: &[u8]) {
        // Prevent overflow by ensuring that the value can't exceed
        // 0x20000000 in length, which would be 0x40000000 when formatted
        assert!(val.len() < 0x20000000);
        assert!(formatted.len() >= val.len() * 2);

        for (index, &byte) in val.iter().enumerate() {
            let dest_index = index * 2;
            let hex_string = format!("{:02x}", byte);
            formatted[dest_index] = hex_string.chars().nth(0).unwrap();
            formatted[dest_index + 1] = hex_string.chars().nth(1).unwrap();
        }
    }
}