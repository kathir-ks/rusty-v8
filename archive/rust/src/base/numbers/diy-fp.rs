// Copyright 2011 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod diy_fp {
    /// This "Do It Yourself Floating Point" struct implements a floating-point number
    /// with a u64 significand and an i32 exponent. Normalized DiyFp numbers will
    /// have the most significant bit of the significand set.
    /// Multiplication and Subtraction do not normalize their results.
    /// DiyFp are not designed to contain special doubles (NaN and Infinity).
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct DiyFp {
        f: u64,
        e: i32,
    }

    impl DiyFp {
        pub const SIGNIFICAND_SIZE: i32 = 64;
        const K_UINT64_MSB: u64 = 1 << 63;

        /// Creates a new DiyFp with f = 0 and e = 0.
        pub const fn new() -> Self {
            DiyFp { f: 0, e: 0 }
        }

        /// Creates a new DiyFp with the given significand and exponent.
        pub const fn new_with_values(f: u64, e: i32) -> Self {
            DiyFp { f, e }
        }

        /// Subtracts `other` from `self`.
        ///
        /// The exponents of both numbers must be the same and the significand of `self`
        /// must be bigger than the significand of `other`.
        /// The result will not be normalized.
        pub fn subtract(&mut self, other: &DiyFp) {
            debug_assert_eq!(self.e, other.e);
            debug_assert!(self.f >= other.f);
            self.f -= other.f;
        }

        /// Returns `a - b`.
        ///
        /// The exponents of both numbers must be the same and `a` must be bigger
        /// than `b`. The result will not be normalized.
        pub fn minus(a: &DiyFp, b: &DiyFp) -> DiyFp {
            let mut result = *a;
            result.subtract(b);
            result
        }

        /// Multiplies `self` by `other`.
        pub fn multiply(&mut self, other: &DiyFp) {
            let a = self.f;
            let b = other.f;

            let (hi, lo) = multiply_u64_by_u64(a, b);

            self.f = hi + (lo >> 63);
            self.e += other.e + 64;
        }

        /// Returns `a * b`.
        pub fn times(a: &DiyFp, b: &DiyFp) -> DiyFp {
            let a_f = a.f;
            let b_f = b.f;

            let (hi, lo) = multiply_u64_by_u64(a_f, b_f);

            DiyFp {
                f: hi + (lo >> 63),
                e: a.e + b.e + 64,
            }
        }

        /// Normalizes the DiyFp number.
        ///
        /// Requires that the significand is not zero.
        pub fn normalize(&mut self) {
            debug_assert_ne!(self.f, 0);
            let mut f = self.f;
            let mut e = self.e;

            // This method is mainly called for normalizing boundaries. In general
            // boundaries need to be shifted by 10 bits. We thus optimize for this case.
            const K10_MS_BITS: u64 = 0x3FF << 54;
            while (f & K10_MS_BITS) == 0 {
                f <<= 10;
                e -= 10;
            }
            while (f & Self::K_UINT64_MSB) == 0 {
                f <<= 1;
                e -= 1;
            }
            self.f = f;
            self.e = e;
        }

        /// Normalizes the DiyFp number and returns a new DiyFp.
        pub fn normalize_new(a: &DiyFp) -> DiyFp {
            let mut result = *a;
            result.normalize();
            result
        }

        /// Returns the significand.
        pub const fn f(&self) -> u64 {
            self.f
        }

        /// Returns the exponent.
        pub const fn e(&self) -> i32 {
            self.e
        }

        /// Sets the significand.
        pub const fn set_f(&mut self, new_value: u64) {
            self.f = new_value;
        }

        /// Sets the exponent.
        pub const fn set_e(&mut self, new_value: i32) {
            self.e = new_value;
        }
    }

    fn multiply_u64_by_u64(a: u64, b: u64) -> (u64, u64) {
        let a_hi = a >> 32;
        let a_lo = a & 0xFFFFFFFF;
        let b_hi = b >> 32;
        let b_lo = b & 0xFFFFFFFF;

        let hi_hi = a_hi * b_hi;
        let hi_lo = a_hi * b_lo;
        let lo_hi = a_lo * b_hi;
        let lo_lo = a_lo * b_lo;

        let mut carry = 0;
        let lo = lo_lo.wrapping_add((hi_lo << 32) & 0xFFFFFFFF00000000);
        if lo < lo_lo {
            carry += 1;
        }

        let lo = lo.wrapping_add((lo_hi << 32) & 0xFFFFFFFF00000000);
        if lo < (lo - ((lo_hi << 32) & 0xFFFFFFFF00000000)) {
            carry += 1;
        }

        let hi = hi_hi + (hi_lo >> 32) + (lo_hi >> 32) + carry;

        (hi, lo)
    }
}