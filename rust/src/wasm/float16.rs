// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// The fp16 crate provides the necessary functionality for half-precision floating-point numbers.
use fp16::FP16;

mod base {
    use std::{mem, ptr};

    // Helper function to read a value from memory, emulating base::ReadUnalignedValue
    pub fn read_unaligned<T: Copy>(source: *const u8) -> T {
        unsafe {
            let mut value: T = mem::zeroed();
            ptr::copy_nonoverlapping(source as *const T, &mut value as *mut T, 1);
            value
        }
    }

    // Helper function to write a value to memory, emulating base::WriteUnalignedValue
    pub fn write_unaligned<T>(destination: *mut u8, value: T) {
        unsafe {
            ptr::copy_nonoverlapping(&value as *const T, destination as *mut T, 1);
        }
    }

    pub type Address = *mut u8;
}

pub mod internal {
    use super::*;

    /// Represents a 16-bit floating-point number.
    #[derive(Clone, Copy, Debug)]
    pub struct Float16 {
        bits: u16,
    }

    impl Float16 {
        /// Creates a new `Float16` with a value of zero.
        pub fn new() -> Self {
            Float16 { bits: 0 }
        }

        /// Reads a `Float16` from a raw memory address.
        pub fn read(source: base::Address) -> Self {
            Float16 {
                bits: base::read_unaligned::<u16>(source as *const u8),
            }
        }

        /// Writes the `Float16` to a raw memory address.
        pub fn write(&self, destination: base::Address) {
            base::write_unaligned::<u16>(destination, self.bits);
        }

        /// Converts a 32-bit float to a `Float16`.
        pub fn from_float32(f32: f32) -> Self {
            Float16 {
                bits: FP16::from_f32(f32).to_bits(),
            }
        }

        /// Converts the `Float16` to a 32-bit float.
        pub fn to_float32(&self) -> f32 {
            FP16::from_bits(self.bits).to_f32()
        }
    }

    // Compile-time assertion to check the size of Float16
    const _: () = assert!(std::mem::size_of::<Float16>() == std::mem::size_of::<u16>());
} // namespace internal