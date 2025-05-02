// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//! Conversion routines between UTF8 and UTF16.

pub mod v8_inspector {
    /// Converts a UTF-8 string to a UTF-16 string.
    ///
    /// # Arguments
    ///
    /// * `string_start` - A pointer to the start of the UTF-8 string.
    /// * `length` - The length of the UTF-8 string in bytes.
    ///
    /// # Returns
    ///
    /// A UTF-16 string.
    pub fn utf8_to_utf16(string_start: &[u8]) -> Vec<u16> {
        string_start
            .iter()
            .map(|&byte| byte as u16)
            .collect()
    }

    /// Converts a UTF-16 string to a UTF-8 string.
    ///
    /// # Arguments
    ///
    /// * `string_start` - A pointer to the start of the UTF-16 string.
    /// * `length` - The length of the UTF-16 string in 16-bit code units.
    ///
    /// # Returns
    ///
    /// A UTF-8 string.
    pub fn utf16_to_utf8(string_start: &[u16]) -> String {
        string_start
            .iter()
            .map(|&code_unit| {
                char::from_u32(code_unit as u32).unwrap_or(char::REPLACEMENT_CHARACTER)
            })
            .collect::<String>()
    }
}