// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod internal {
    use std::{convert::TryInto, f32, f64, hash::{Hash, Hasher}};

    /// Safety wrapper for a 32-bit floating-point value to make sure we don't lose
    /// the exact bit pattern during deoptimization when passing this value.
    #[derive(Copy, Clone, Debug)]
    #[repr(C)]
    pub struct Float32 {
        bit_pattern_: u32,
    }

    impl Float32 {
        pub fn new(value: f32) -> Self {
            if value.is_nan() {
                panic!("Float32 does not guarantee preservation of NaN bit patterns");
            }
            Float32 {
                bit_pattern_: value.to_bits(),
            }
        }

        pub fn get_bits(&self) -> u32 {
            self.bit_pattern_
        }

        pub fn get_scalar(&self) -> f32 {
            f32::from_bits(self.bit_pattern_)
        }

        pub fn is_nan(&self) -> bool {
            self.get_scalar().is_nan()
        }

        // Return a pointer to the field storing the bit pattern. Used in code
        // generation tests to store generated values there directly.
        pub fn get_bits_address(&mut self) -> *mut u32 {
            &mut self.bit_pattern_
        }

        pub const fn from_bits(bits: u32) -> Self {
            Float32 { bit_pattern_: bits }
        }
    }

    impl Default for Float32 {
        fn default() -> Self {
            Float32 { bit_pattern_: 0 }
        }
    }


    /// Safety wrapper for a 64-bit floating-point value to make sure we don't lose
    /// the exact bit pattern during deoptimization when passing this value.
    #[derive(Copy, Clone, Debug)]
    #[repr(C)]
    pub struct Float64 {
        bit_pattern_: u64,
    }

    impl Float64 {
        pub fn new(value: f64) -> Self {
             if value.is_nan() {
                panic!("Float64 does not guarantee preservation of NaN bit patterns");
            }
            Float64 {
                bit_pattern_: value.to_bits(),
            }
        }

        pub fn from_double(value: base::Double) -> Self {
            Float64 {
                bit_pattern_: value.as_u64(),
            }
        }

        pub fn get_bits(&self) -> u64 {
            self.bit_pattern_
        }

        pub fn get_scalar(&self) -> f64 {
            f64::from_bits(self.bit_pattern_)
        }

        pub fn is_hole_nan(&self) -> bool {
            self.bit_pattern_ == kHoleNanInt64
        }

        pub fn is_nan(&self) -> bool {
            self.get_scalar().is_nan()
        }

        // Return a pointer to the field storing the bit pattern. Used in code
        // generation tests to store generated values there directly.
        pub fn get_bits_address(&mut self) -> *mut u64 {
            &mut self.bit_pattern_
        }

        pub const fn from_bits(bits: u64) -> Self {
            Float64 { bit_pattern_: bits }
        }
    }

    impl Default for Float64 {
        fn default() -> Self {
            Float64 { bit_pattern_: 0 }
        }
    }

    impl PartialEq for Float64 {
        // Unlike doubles, equality is defined as equally behaving as far as the
        // optimizers are concerned. I.e., two NaN's are equal as long as they are
        // both the hole nor not.
        fn eq(&self, other: &Self) -> bool {
            if self.is_nan() && other.is_nan() {
                self.is_hole_nan() == other.is_hole_nan()
            } else {
                self.get_scalar() == other.get_scalar()
            }
        }
    }

    impl Eq for Float64 {}

    impl Hash for Float64 {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.bit_pattern_.hash(state);
        }
    }

    pub const kHoleNanInt64: u64 = 0x7ffc000000000000;
}

pub mod base {
    use std::convert::TryInto;
    use std::hash::{Hash, Hasher};

    #[derive(Copy, Clone, Debug)]
    pub struct Double {
        value: f64,
    }

    impl Double {
        pub fn new(value: f64) -> Self {
            Double { value }
        }

        pub fn as_u64(&self) -> u64 {
            self.value.to_bits()
        }
    }

    impl Hash for internal::Float64 {
        fn hash<H: Hasher>(&self, state: &mut H) {
            if self.is_nan() {
                self.is_hole_nan().hash(state);
            } else {
                self.get_bits().hash(state);
            }
        }
    }
}