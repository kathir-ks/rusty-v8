// Converted from V8 C++ source files:
// Header: diy-fp.h
// Implementation: diy-fp.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/base/numbers/diy-fp.h
use std::ops::{Add, Mul, Sub};

// src/base/logging.h - Assuming logging is done through println! for now

// This "Do It Yourself Floating Point" class implements a floating-point number
// with a uint64 significand and an int exponent. Normalized DiyFp numbers will
// have the most significant bit of the significand set.
// Multiplication and Subtraction do not normalize their results.
// DiyFp are not designed to contain special doubles (NaN and Infinity).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DiyFp {
    f_: u64,
    e_: i32,
}

impl DiyFp {
    pub const kSignificandSize: i32 = 64;

    pub const fn new(f: u64, e: i32) -> Self {
        Self { f_: f, e_: e }
    }

    // this = this - other.
    // The exponents of both numbers must be the same and the significand of this
    // must be bigger than the significand of other.
    // The result will not be normalized.
    pub fn subtract(&mut self, other: &DiyFp) {
        if self.e_ != other.e_ {
            panic!("Exponents must be the same");
        }
        if self.f_ < other.f_ {
            panic!("Significand of this must be bigger than significand of other");
        }
        self.f_ -= other.f_;
    }

    // Returns a - b.
    // The exponents of both numbers must be the same and this must be bigger
    // than other. The result will not be normalized.
    pub fn minus(a: &DiyFp, b: &DiyFp) -> DiyFp {
        let mut result = *a;
        result.subtract(b);
        result
    }

    // this = this * other.
    pub fn multiply(&mut self, other: &DiyFp) {
        // Simply "emulates" a 128 bit multiplication.
        // However: the resulting number only contains 64 bits. The least
        // significant 64 bits are only used for rounding the most significant 64
        // bits.
        const K_M32: u64 = 0xFFFFFFFFu64;
        let a = self.f_ >> 32;
        let b = self.f_ & K_M32;
        let c = other.f_ >> 32;
        let d = other.f_ & K_M32;
        let ac = a * c;
        let bc = b * c;
        let ad = a * d;
        let bd = b * d;
        let mut tmp = (bd >> 32).wrapping_add(ad & K_M32).wrapping_add(bc & K_M32);
        // By adding 1U << 31 to tmp we round the final result.
        // Halfway cases will be round up.
        tmp = tmp.wrapping_add(1u64 << 31);
        let result_f = ac.wrapping_add(ad >> 32).wrapping_add(bc >> 32).wrapping_add(tmp >> 32);
        self.e_ += other.e_ + 64;
        self.f_ = result_f;
    }

    // returns a * b;
    pub fn times(a: &DiyFp, b: &DiyFp) -> DiyFp {
        let mut result = *a;
        result.multiply(b);
        result
    }

    pub fn normalize(&mut self) {
        if self.f_ == 0 {
            return;
        }
        let mut f = self.f_;
        let mut e = self.e_;

        // This method is mainly called for normalizing boundaries. In general
        // boundaries need to be shifted by 10 bits. We thus optimize for this case.
        const K10_MS_BITS: u64 = (0x3FFu64) << 54;
        while (f & K10_MS_BITS) == 0 {
            f <<= 10;
            e -= 10;
        }
        while (f & Self::kUint64MSB) == 0 {
            f <<= 1;
            e -= 1;
        }
        self.f_ = f;
        self.e_ = e;
    }

    pub fn normalize_copy(a: &DiyFp) -> DiyFp {
        let mut result = *a;
        result.normalize();
        result
    }

    pub const fn f(&self) -> u64 {
        self.f_
    }
    pub const fn e(&self) -> i32 {
        self.e_
    }

    pub const fn set_f(&mut self, new_value: u64) {
        self.f_ = new_value;
    }
    pub const fn set_e(&mut self, new_value: i32) {
        self.e_ = new_value;
    }

    const kUint64MSB: u64 = 1u64 << 63;
}
