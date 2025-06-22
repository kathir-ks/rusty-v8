// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/inspector/crc32.rs

/// Computes the CRC32 checksum of a UTF-16 encoded string.
///
/// # Arguments
///
/// * `string`: The string to compute the checksum for.
///
/// # Returns
///
/// The CRC32 checksum as a signed 32-bit integer.
pub fn compute_crc32(string: &[u16]) -> i32 {
    let mut crc = 0xFFFFFFFF;

    for &word in string {
        let mut byte = (word & 0xFF) as u8;
        crc ^= byte as u32;

        for _ in 0..8 {
            if (crc & 1) != 0 {
                crc = (crc >> 1) ^ 0xEDB88320;
            } else {
                crc >>= 1;
            }
        }

        byte = ((word >> 8) & 0xFF) as u8;
        crc ^= byte as u32;

        for _ in 0..8 {
            if (crc & 1) != 0 {
                crc = (crc >> 1) ^ 0xEDB88320;
            } else {
                crc >>= 1;
            }
        }
    }

    !crc as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        let s: [u16;0] = [];
        assert_eq!(compute_crc32(&s), -306674912);
    }

    #[test]
    fn test_simple_string() {
        let s: [u16; 5] = [72, 101, 108, 108, 111]; // "Hello" in UTF-16
        assert_eq!(compute_crc32(&s), 987687600);
    }

    #[test]
    fn test_string_with_unicode() {
        let s: [u16; 2] = [0x4E16, 0x754C]; // "世界" in UTF-16
        assert_eq!(compute_crc32(&s), -1270408702);
    }
}