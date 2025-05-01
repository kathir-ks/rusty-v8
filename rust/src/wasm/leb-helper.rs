// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This header should only be included if WebAssembly is enabled.
// #[cfg(not(feature = "webassembly"))]
// compile_error!("This header should only be included if WebAssembly is enabled.");

pub mod wasm {
    /// Size of a padded varint32.
    pub const K_PADDED_VAR_INT32_SIZE: usize = 5;
    /// Maximum size of a varint32.
    pub const K_MAX_VAR_INT32_SIZE: usize = 5;
    /// Maximum size of a varint64.
    pub const K_MAX_VAR_INT64_SIZE: usize = 10;

    /// Helper struct for LEB encoding and decoding.
    pub struct LEBHelper {}

    impl LEBHelper {
        /// Write a 32-bit unsigned LEB to the destination, updating the destination
        /// to point after the last byte written. No safety checks.
        pub fn write_u32v(dest: &mut &mut [u8], mut val: u32) {
            while val >= 0x80 {
                **dest.get_mut(0).unwrap() = (0x80 | (val & 0x7F)) as u8;
                *dest = &mut dest[1..];
                val >>= 7;
            }
            **dest.get_mut(0).unwrap() = (val & 0x7F) as u8;
            *dest = &mut dest[1..];
        }

        /// Write a 32-bit signed LEB to the destination, updating the destination
        /// to point after the last byte written. No safety checks.
        pub fn write_i32v(dest: &mut &mut [u8], mut val: i32) {
            if val >= 0 {
                while val >= 0x40 {
                    **dest.get_mut(0).unwrap() = (0x80 | (val & 0x7F)) as u8;
                    *dest = &mut dest[1..];
                    val >>= 7;
                }
                **dest.get_mut(0).unwrap() = (val & 0xFF) as u8;
                *dest = &mut dest[1..];
            } else {
                while (val >> 6) != -1 {
                    **dest.get_mut(0).unwrap() = (0x80 | (val & 0x7F)) as u8;
                    *dest = &mut dest[1..];
                    val >>= 7;
                }
                **dest.get_mut(0).unwrap() = (val & 0x7F) as u8;
                *dest = &mut dest[1..];
            }
        }

        /// Write a 64-bit unsigned LEB to the destination, updating the destination
        /// to point after the last byte written. No safety checks.
        pub fn write_u64v(dest: &mut &mut [u8], mut val: u64) {
            while val >= 0x80 {
                **dest.get_mut(0).unwrap() = (0x80 | (val & 0x7F)) as u8;
                *dest = &mut dest[1..];
                val >>= 7;
            }
            **dest.get_mut(0).unwrap() = (val & 0x7F) as u8;
            *dest = &mut dest[1..];
        }

        /// Write a 64-bit signed LEB to the destination, updating the destination
        /// to point after the last byte written. No safety checks.
        pub fn write_i64v(dest: &mut &mut [u8], mut val: i64) {
            if val >= 0 {
                while val >= 0x40 {
                    **dest.get_mut(0).unwrap() = (0x80 | (val & 0x7F)) as u8;
                    *dest = &mut dest[1..];
                    val >>= 7;
                }
                **dest.get_mut(0).unwrap() = (val & 0xFF) as u8;
                *dest = &mut dest[1..];
            } else {
                while (val >> 6) != -1 {
                    **dest.get_mut(0).unwrap() = (0x80 | (val & 0x7F)) as u8;
                    *dest = &mut dest[1..];
                    val >>= 7;
                }
                **dest.get_mut(0).unwrap() = (val & 0x7F) as u8;
                *dest = &mut dest[1..];
            }
        }

        /// Compute the size of {val} if emitted as an LEB32.
        pub fn sizeof_u32v(mut val: usize) -> usize {
            let mut size = 0;
            loop {
                size += 1;
                val = val >> 7;
                if val == 0 {
                    break;
                }
            }
            size
        }

        /// Compute the size of {val} if emitted as an LEB32.
        pub fn sizeof_i32v(mut val: i32) -> usize {
            let mut size = 1;
            if val >= 0 {
                while val >= 0x40 {
                    size += 1;
                    val >>= 7;
                }
            } else {
                while (val >> 6) != -1 {
                    size += 1;
                    val >>= 7;
                }
            }
            size
        }

        /// Compute the size of {val} if emitted as an unsigned LEB64.
        pub fn sizeof_u64v(mut val: u64) -> usize {
            let mut size = 0;
            loop {
                size += 1;
                val = val >> 7;
                if val == 0 {
                    break;
                }
            }
            size
        }

        /// Compute the size of {val} if emitted as a signed LEB64.
        pub fn sizeof_i64v(mut val: i64) -> usize {
            let mut size = 1;
            if val >= 0 {
                while val >= 0x40 {
                    size += 1;
                    val >>= 7;
                }
            } else {
                while (val >> 6) != -1 {
                    size += 1;
                    val >>= 7;
                }
            }
            size
        }
    }
}