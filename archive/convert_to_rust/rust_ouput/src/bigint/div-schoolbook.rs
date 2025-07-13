// Converted from V8 C++ source files:
// Header: N/A
// Implementation: div-schoolbook.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// "Schoolbook" division. This is loosely based on Go's implementation
// found at https://golang.org/src/math/big/nat.go, licensed as follows:
//
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file [1].
//
// [1] https://golang.org/LICENSE

use std::cmp::Ordering;
use std::ops::{Add, Sub};

use crate::bigint::{
    digit_arithmetic::digit_mul, digit_div, vector_arithmetic,
    vector_arithmetic::AddAndReturnCarry, vector_arithmetic::LeftShift,
    vector_arithmetic::RightShift, vector_arithmetic::SubtractAndReturnBorrow, Digits,
    RWDigits, ShiftedDigits,
};

pub struct ProcessorImpl {}

impl ProcessorImpl {
    // Computes Q(uotient) and remainder for A/b, such that
    // Q = (A - remainder) / b, with 0 <= remainder < b.
    // If Q.len == 0, only the remainder will be returned.
    // Q may be the same as A for an in-place division.
    pub fn divide_single(q: RWDigits, remainder: &mut digit_t, a: Digits, b: digit_t) {
        assert_ne!(b, 0);
        assert!(a.len() > 0);
        *remainder = 0;
        let length = a.len();
        if q.len() != 0 {
            if a[length - 1] >= b {
                assert!(q.len() >= a.len());
                for i in (0..length).rev() {
                    q[i] = digit_div(*remainder, a[i], b, remainder);
                }
                for i in length..q.len() {
                    q[i] = 0;
                }
            } else {
                assert!(q.len() >= a.len() - 1);
                *remainder = a[length - 1];
                for i in (0..length - 1).rev() {
                    q[i] = digit_div(*remainder, a[i], b, remainder);
                }
                for i in length - 1..q.len() {
                    q[i] = 0;
                }
            }
        } else {
            for i in (0..length).rev() {
                digit_div(*remainder, a[i], b, remainder);
            }
        }
    }

    // Computes Q(uotient) and R(emainder) for A/B, such that
    // Q = (A - R) / B, with 0 <= R < B.
    // Both Q and R are optional: callers that are only interested in one of them
    // can pass the other with len == 0.
    // If Q is present, its length must be at least A.len - B.len + 1.
    // If R is present, its length must be at least B.len.
    // See Knuth, Volume 2, section 4.3.1, Algorithm D.
    pub fn divide_schoolbook(q: RWDigits, r: RWDigits, a: Digits, b: Digits) {
        assert!(b.len() >= 2); // Use DivideSingle otherwise.
        assert!(a.len() >= b.len()); // No-op otherwise.
                                      // The unusual variable names inside this function are consistent with
                                      // Knuth's book, as well as with Go's implementation of this algorithm.
                                      // Maintaining this consistency is probably more useful than trying to
                                      // come up with more descriptive names for them.
        let n = b.len();
        let m = a.len() - n;

        // In each iteration, {qhatv} holds {divisor} * {current quotient digit}.
        // "v" is the book's name for {divisor}, "qhat" the current quotient digit.
        let mut qhatv = ScratchDigits::new(n + 1);

        // D1.
        // Left-shift inputs so that the divisor's MSB is set. This is necessary
        // to prevent the digit-wise divisions (see digit_div call below) from
        // overflowing (they take a two digits wide input, and return a one digit
        // result).
        let b_normalized = ShiftedDigits::new(b);
        let b = b_normalized.digits;
        // U holds the (continuously updated) remaining part of the dividend, which
        // eventually becomes the remainder.
        let mut u = ScratchDigits::new(a.len() + 1);
        LeftShift(&mut u, a, b_normalized.shift());

        // D2.
        // Iterate over the dividend's digits (like the "grad school" algorithm).
        // {vn1} is the divisor's most significant digit.
        let vn1 = b[n - 1];
        for j in (0..=m).rev() {
            // D3.
            // Estimate the current iteration's quotient digit (see Knuth for details).
            // {qhat} is the current quotient digit.
            let mut qhat = std::u32::MAX as digit_t;
            // {ujn} is the dividend's most significant remaining digit.
            let ujn = u[j + n];
            if ujn != vn1 {
                // {rhat} is the current iteration's remainder.
                let mut rhat: digit_t = 0;
                // Estimate the current quotient digit by dividing the most significant
                // digits of dividend and divisor. The result will not be too small,
                // but could be a bit too large.

                qhat = digit_div(ujn, u[j + n - 1], vn1, &mut rhat);

                // Decrement the quotient estimate as needed by looking at the next
                // digit, i.e. by testing whether
                // qhat * v_{n-2} > (rhat << kDigitBits) + u_{j+n-2}.
                let vn2 = b[n - 2];
                let ujn2 = u[j + n - 2];
                while product_greater_than(qhat, vn2, rhat, ujn2) {
                    qhat -= 1;
                    let prev_rhat = rhat;
                    rhat += vn1;
                    // v[n-1] >= 0, so this tests for overflow.
                    if rhat < prev_rhat {
                        break;
                    }
                }
            }

            // D4.
            // Multiply the divisor with the current quotient digit, and subtract
            // it from the dividend. If there was "borrow", then the quotient digit
            // was one too high, so we must correct it and undo one subtraction of
            // the (shifted) divisor.
            if qhat == 0 {
                qhatv.clear();
            } else {
                multiply_single(&mut qhatv, b, qhat);
            }
            let c = inplace_sub(&mut u.digits[j..], &qhatv.digits[..n + 1]);
            if c != 0 {
                let c = inplace_add(&mut u.digits[j..], &b);
                u[j + n] = u[j + n].wrapping_add(c);
            
                qhat -= 1;
            }

            if q.len() != 0 {
                if j >= q.len() {
                    assert_eq!(qhat, 0);
                } else {
                    q[j] = qhat;
                }
            }
        }
        if r.len() != 0 {
            RightShift(r, &u, b_normalized.shift());
        }
        // If Q has extra storage, clear it.
        for i in m + 1..q.len() {
            q[i] = 0;
        }
    }
}

// Z += X. Returns the "carry" (0 or 1) after adding all of X's digits.
fn inplace_add(z: &mut [digit_t], x: &[digit_t]) -> digit_t {
    AddAndReturnCarry(z, z, x)
}

// Z -= X. Returns the "borrow" (0 or 1) after subtracting all of X's digits.
fn inplace_sub(z: &mut [digit_t], x: &[digit_t]) -> digit_t {
    SubtractAndReturnBorrow(z, z, x)
}

// Returns whether (factor1 * factor2) > (high << kDigitBits) + low.
fn product_greater_than(factor1: digit_t, factor2: digit_t, high: digit_t, low: digit_t) -> bool {
    let mut result_high: digit_t = 0;
    let result_low: digit_t = digit_mul(factor1, factor2, &mut result_high);
    result_high > high || (result_high == high && result_low > low)
}

// Function to multiply a digit with the digits
fn multiply_single(out: &mut ScratchDigits, inp: Digits, digit: digit_t) {
    let mut carry: digit_t = 0;
    for i in 0..inp.len() {
        out[i] = digit_mul(inp[i], digit, &mut carry);
    }
    out[inp.len()] = carry;
}
