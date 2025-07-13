// Converted from V8 C++ source files:
// Header: digit-arithmetic.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Helper functions that operate on individual digits.

// use crate::bigint::bigint::digit_t; // Assuming digit_t is defined here
// use crate::bigint::util::*; // Assuming util.h is converted to util.rs

pub mod digit_arithmetic {
    use std::mem::size_of;

    pub type digit_t = u32; // Assuming digit_t is an unsigned 32-bit integer
    pub const kDigitBits: i32 = size_of::<digit_t>() as i32 * 8;

    pub const kHalfDigitBits: i32 = kDigitBits / 2;
    pub const kHalfDigitBase: digit_t = 1 << kHalfDigitBits;
    pub const kHalfDigitMask: digit_t = kHalfDigitBase - 1;

    pub const fn digit_ismax(x: digit_t) -> bool {
        x.wrapping_neg() == 0
    }

    // {carry} will be set to 0 or 1.
    pub fn digit_add2(a: digit_t, b: digit_t, carry: &mut digit_t) -> digit_t {
        let result = (a as u64) + (b as u64);
        *carry = (result >> kDigitBits) as digit_t;
        result as digit_t
    }

    // This compiles to slightly better machine code than repeated invocations
    // of {digit_add2}.
    pub fn digit_add3(a: digit_t, b: digit_t, c: digit_t, carry: &mut digit_t) -> digit_t {
        let result = (a as u64) + (b as u64) + (c as u64);
        *carry = (result >> kDigitBits) as digit_t;
        result as digit_t
    }

    // {borrow} will be set to 0 or 1.
    pub fn digit_sub(a: digit_t, b: digit_t, borrow: &mut digit_t) -> digit_t {
        if a < b {
            *borrow = 1;
            a.wrapping_sub(b)
        } else {
            *borrow = 0;
            a - b
        }
    }

    // {borrow_out} will be set to 0 or 1.
    pub fn digit_sub2(a: digit_t, b: digit_t, borrow_in: digit_t, borrow_out: &mut digit_t) -> digit_t {
        let subtrahend = (b as u64) + (borrow_in as u64);
        if (a as u64) < subtrahend {
            *borrow_out = 1;
            (a as u64).wrapping_sub(subtrahend) as digit_t
        } else {
            *borrow_out = 0;
            ((a as u64) - subtrahend) as digit_t
        }
    }

    // Returns the low half of the result. High half is in {high}.
    pub fn digit_mul(a: digit_t, b: digit_t, high: &mut digit_t) -> digit_t {
        let result = (a as u64) * (b as u64);
        *high = (result >> kDigitBits) as digit_t;
        result as digit_t
    }

    // Returns the quotient.
    // quotient = (high << kDigitBits + low - remainder) / divisor
    pub fn digit_div(high: digit_t, low: digit_t, divisor: digit_t, remainder: &mut digit_t) -> digit_t {
        if high >= divisor {
            panic!("high must be less than divisor");
        }
        if divisor == 0 {
            panic!("divisor cannot be 0");
        }

        let combined = ((high as u64) << kDigitBits) | (low as u64);
        *remainder = (combined % (divisor as u64)) as digit_t;
        (combined / (divisor as u64)) as digit_t
    }
}
