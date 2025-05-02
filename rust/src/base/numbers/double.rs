// Copyright 2011 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/base/numbers/double.rs

use std::mem;

use crate::base::numbers::diy_fp::DiyFp;

pub mod base {
    pub mod numbers {
        pub mod diy_fp;
    }
}

/// Converts a double to its `u64` representation.
#[inline]
pub const fn double_to_u64(d: f64) -> u64 {
    d.to_bits()
}

/// Converts a `u64` representation to a double.
#[inline]
pub const fn u64_to_double(d64: u64) -> f64 {
    f64::from_bits(d64)
}

/// Helper functions for doubles.
#[derive(Clone, Copy, Debug)]
pub struct Double {
    d64_: u64,
}

impl Double {
    pub const K_SIGN_MASK: u64 = 0x8000_0000_0000_0000;
    pub const K_EXPONENT_MASK: u64 = 0x7FF0_0000_0000_0000;
    pub const K_SIGNIFICAND_MASK: u64 = 0x000F_FFFF_FFFF_FFFF;
    pub const K_HIDDEN_BIT: u64 = 0x0010_0000_0000_0000;
    pub const K_PHYSICAL_SIGNIFICAND_SIZE: i32 = 52; // Excludes the hidden bit.
    pub const K_SIGNIFICAND_SIZE: i32 = 53;

    /// Creates a `Double` with the value 0.0.
    pub const fn new() -> Self {
        Double { d64_: 0 }
    }

    /// Creates a `Double` from a `f64`.
    pub const fn from_f64(d: f64) -> Self {
        Double { d64_: double_to_u64(d) }
    }

    /// Creates a `Double` from a `u64`.
    pub const fn from_u64(d64: u64) -> Self {
        Double { d64_: d64 }
    }

    /// Creates a `Double` from a `DiyFp`.
    pub const fn from_diy_fp(diy_fp: DiyFp) -> Self {
        Double {
            d64_: Self::diy_fp_to_u64(diy_fp),
        }
    }

    /// Returns the `DiyFp` representation of the double.
    ///
    /// # Panics
    ///
    /// Panics if the double is not greater or equal to +0.0 or if it is special
    /// (infinity or NaN).
    pub fn as_diy_fp(&self) -> DiyFp {
        assert!(self.sign() > 0);
        assert!(!self.is_special());
        DiyFp::new(self.significand(), self.exponent())
    }

    /// Returns the normalized `DiyFp` representation of the double.
    ///
    /// # Panics
    ///
    /// Panics if the value is not strictly greater than 0.
    pub fn as_normalized_diy_fp(&self) -> DiyFp {
        assert!(self.value() > 0.0);
        let mut f = self.significand();
        let mut e = self.exponent();

        // The current double could be a denormal.
        while (f & Self::K_HIDDEN_BIT) == 0 {
            f <<= 1;
            e -= 1;
        }
        // Do the final shifts in one go.
        f <<= DiyFp::K_SIGNIFICAND_SIZE - Self::K_SIGNIFICAND_SIZE;
        e -= DiyFp::K_SIGNIFICAND_SIZE - Self::K_SIGNIFICAND_SIZE;
        DiyFp::new(f, e)
    }

    /// Returns the double's bit representation as `u64`.
    pub const fn as_u64(&self) -> u64 {
        self.d64_
    }

    /// Returns the next greater double. Returns +infinity on input +infinity.
    pub fn next_double(&self) -> f64 {
        if self.d64_ == Self::K_INFINITY {
            return Double::from_u64(Self::K_INFINITY).value();
        }
        if self.sign() < 0 && self.significand() == 0 {
            // -0.0
            return 0.0;
        }
        if self.sign() < 0 {
            return Double::from_u64(self.d64_ - 1).value();
        } else {
            return Double::from_u64(self.d64_ + 1).value();
        }
    }

    /// Returns the exponent of the double.
    pub const fn exponent(&self) -> i32 {
        if self.is_denormal() {
            return Self::K_DENORMAL_EXPONENT;
        }

        let d64 = self.as_u64();
        let biased_e = ((d64 & Self::K_EXPONENT_MASK) >> Self::K_PHYSICAL_SIGNIFICAND_SIZE) as i32;
        biased_e - Self::K_EXPONENT_BIAS
    }

    /// Returns the significand of the double.
    pub const fn significand(&self) -> u64 {
        let d64 = self.as_u64();
        let significand = d64 & Self::K_SIGNIFICAND_MASK;
        if !self.is_denormal() {
            significand + Self::K_HIDDEN_BIT
        } else {
            significand
        }
    }

    /// Returns `true` if the double is a denormal.
    pub const fn is_denormal(&self) -> bool {
        (self.as_u64() & Self::K_EXPONENT_MASK) == 0
    }

    /// Returns `true` if the double is special (Infinity or NaN).
    pub const fn is_special(&self) -> bool {
        (self.as_u64() & Self::K_EXPONENT_MASK) == Self::K_EXPONENT_MASK
    }

    /// Returns `true` if the double is infinite.
    pub const fn is_infinite(&self) -> bool {
        let d64 = self.as_u64();
        ((d64 & Self::K_EXPONENT_MASK) == Self::K_EXPONENT_MASK)
            && ((d64 & Self::K_SIGNIFICAND_MASK) == 0)
    }

    /// Returns the sign of the double (1 for positive, -1 for negative).
    pub const fn sign(&self) -> i32 {
        let d64 = self.as_u64();
        if (d64 & Self::K_SIGN_MASK) == 0 {
            1
        } else {
            -1
        }
    }

    /// Returns the upper boundary of the double.
    ///
    /// # Panics
    ///
    /// Panics if the sign is not positive.
    pub fn upper_boundary(&self) -> DiyFp {
        assert!(self.sign() > 0);
        DiyFp::new(self.significand() * 2 + 1, self.exponent() - 1)
    }

    /// Returns the normalized boundaries of the double.
    ///
    /// The bigger boundary (`m_plus`) is normalized. The lower boundary has the same
    /// exponent as `m_plus`.
    ///
    /// # Panics
    ///
    /// Panics if the value is not greater than 0.
    pub fn normalized_boundaries(&self, out_m_minus: &mut DiyFp, out_m_plus: &mut DiyFp) {
        assert!(self.value() > 0.0);
        let v = self.as_diy_fp();
        let mut m_plus = DiyFp::normalize(DiyFp::new((v.f() << 1) + 1, v.e() - 1));
        let mut m_minus;
        if (self.as_u64() & Self::K_SIGNIFICAND_MASK) == 0 && v.e() != Self::K_DENORMAL_EXPONENT {
            // The boundary is closer. Think of v = 1000e10 and v- = 9999e9.
            // Then the boundary (== (v - v-)/2) is not just at a distance of 1e9 but
            // at a distance of 1e8.
            // The only exception is for the smallest normal: the largest denormal is
            // at the same distance as its successor.
            // Note: denormals have the same exponent as the smallest normals.
            m_minus = DiyFp::new((v.f() << 2) - 1, v.e() - 2);
        } else {
            m_minus = DiyFp::new((v.f() << 1) - 1, v.e() - 1);
        }
        m_minus.set_f(m_minus.f() << (m_minus.e() - m_plus.e()));
        m_minus.set_e(m_plus.e());
        *out_m_plus = m_plus;
        *out_m_minus = m_minus;
    }

    /// Returns the value of the double as `f64`.
    pub const fn value(&self) -> f64 {
        u64_to_double(self.d64_)
    }

    /// Returns the significand size for a given order of magnitude.
    ///
    /// If v = f*2^e with 2^p-1 <= f <= 2^p then p+e is v's order of magnitude.
    /// This function returns the number of significant binary digits v will have
    /// once its encoded into a double. In almost all cases this is equal to
    /// `K_SIGNIFICAND_SIZE`. The only exception are denormals. They start with leading
    /// zeroes and their effective significand-size is hence smaller.
    pub const fn significand_size_for_order_of_magnitude(order: i32) -> i32 {
        if order >= (Self::K_DENORMAL_EXPONENT + Self::K_SIGNIFICAND_SIZE) {
            return Self::K_SIGNIFICAND_SIZE;
        }
        if order <= Self::K_DENORMAL_EXPONENT {
            return 0;
        }
        order - Self::K_DENORMAL_EXPONENT
    }

    const K_EXPONENT_BIAS: i32 = 0x3FF + Self::K_PHYSICAL_SIGNIFICAND_SIZE;
    const K_DENORMAL_EXPONENT: i32 = -Self::K_EXPONENT_BIAS + 1;
    const K_MAX_EXPONENT: i32 = 0x7FF - Self::K_EXPONENT_BIAS;
    const K_INFINITY: u64 = 0x7FF0_0000_0000_0000;

    const fn diy_fp_to_u64(diy_fp: DiyFp) -> u64 {
        let mut significand = diy_fp.f();
        let mut exponent = diy_fp.e();
        while significand > Self::K_HIDDEN_BIT + Self::K_SIGNIFICAND_MASK {
            significand >>= 1;
            exponent += 1;
        }
        if exponent >= Self::K_MAX_EXPONENT {
            return Self::K_INFINITY;
        }
        if exponent < Self::K_DENORMAL_EXPONENT {
            return 0;
        }
        while exponent > Self::K_DENORMAL_EXPONENT && (significand & Self::K_HIDDEN_BIT) == 0 {
            significand <<= 1;
            exponent -= 1;
        }
        let biased_exponent;
        if exponent == Self::K_DENORMAL_EXPONENT && (significand & Self::K_HIDDEN_BIT) == 0 {
            biased_exponent = 0;
        } else {
            biased_exponent = (exponent + Self::K_EXPONENT_BIAS) as u64;
        }
        (significand & Self::K_SIGNIFICAND_MASK) | (biased_exponent << Self::K_PHYSICAL_SIGNIFICAND_SIZE)
    }
}