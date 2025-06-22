// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// Integer division, rounding up.
macro_rules! div_ceil {
    ($x:expr, $y:expr) => {
        (($x) - 1) / ($y) + 1
    };
}

pub mod bigint {
    /// Rounds up x to a multiple of y.
    #[inline]
    pub const fn round_up(x: i32, y: i32) -> i32 {
        (x + y - 1) & -y
    }

    /// Counts the number of leading zeros in a given unsigned integer.
    pub trait CountLeadingZeros {
        fn count_leading_zeros(self) -> u32;
    }

    impl CountLeadingZeros for u64 {
        #[cfg(target_arch = "x86_64")]
        fn count_leading_zeros(self) -> u32 {
            if self == 0 {
                64
            } else {
                self.leading_zeros()
            }
        }

        #[cfg(not(target_arch = "x86_64"))]
        fn count_leading_zeros(self) -> u32 {
            if self == 0 {
                64
            } else {
                self.leading_zeros()
            }
        }
    }

    impl CountLeadingZeros for u32 {
        #[cfg(target_arch = "x86_64")]
        fn count_leading_zeros(self) -> u32 {
            if self == 0 {
                32
            } else {
                self.leading_zeros()
            }
        }

        #[cfg(not(target_arch = "x86_64"))]
        fn count_leading_zeros(self) -> u32 {
            if self == 0 {
                32
            } else {
                self.leading_zeros()
            }
        }
    }

    /// Counts the number of trailing zeros in a given unsigned 32-bit integer.
    #[inline]
    pub fn count_trailing_zeros(value: u32) -> u32 {
        if value == 0 {
            32
        } else {
            value.trailing_zeros()
        }
    }

    /// Calculates the bit length of an integer.
    #[inline]
    pub const fn bit_length(n: i32) -> u32 {
        32 - CountLeadingZeros::count_leading_zeros(n as u32)
    }

    /// Checks if a value is a power of two.
    #[inline]
    pub const fn is_power_of_two(value: i32) -> bool {
        value > 0 && (value & (value - 1)) == 0
    }
}