// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod bigint_internal;
mod digit_arithmetic;
mod vector_arithmetic;

use bigint_internal::*;
use digit_arithmetic::*;
use vector_arithmetic::*;

pub mod mul_schoolbook {
    use super::*;

    /// Z := X * y, where y is a single digit.
    pub fn multiply_single(z: &mut [digit_t], x: &[digit_t], y: digit_t) {
        debug_assert!(y != 0);
        let mut carry: digit_t = 0;
        let mut high: digit_t = 0;
        for i in 0..x.len() {
            let mut new_high: digit_t = 0;
            let low = digit_mul(x[i], y, &mut new_high);
            z[i] = digit_add3(low, high, carry, &mut carry);
            high = new_high;
        }
        // AddWorkEstimate(x.len()); // No direct translation in Rust
        z[x.len()] = carry + high;
        for i in x.len() + 1..z.len() {
            z[i] = 0;
        }
    }

    macro_rules! body {
        ($min:expr, $max:expr, $x:expr, $y:expr, $i:expr, $z:expr, $zi:ident, $carry:ident, $next:ident, $next_carry:ident) => {
            for j in $min..=$max {
                let mut high: digit_t = 0;
                let low = digit_mul($x[j], $y[$i - j], &mut high);
                let mut carrybit: digit_t = 0;
                $zi = digit_add2($zi, low, &mut carrybit);
                $carry += carrybit;
                let mut carrybit2: digit_t = 0;
                $next = digit_add2($next, high, &mut carrybit2);
                $next_carry += carrybit2;
            }
            $z[$i] = $zi;
        };
    }

    /// Z := X * Y.
    /// O(n²) "schoolbook" multiplication algorithm. Optimized to minimize
    /// bounds and overflow checks: rather than looping over X for every digit
    /// of Y (or vice versa), we loop over Z. The {BODY} macro above is what
    /// computes one of Z's digits as a sum of the products of relevant digits
    /// of X and Y. This yields a nearly 2x improvement compared to more obvious
    /// implementations.
    /// This method is *highly* performance sensitive even for the advanced
    /// algorithms, which use this as the base case of their recursive calls.
    pub fn multiply_schoolbook(z: &mut [digit_t], x: &[digit_t], y: &[digit_t]) {
        debug_assert!(is_digit_normalized(x));
        debug_assert!(is_digit_normalized(y));
        debug_assert!(x.len() >= y.len());
        debug_assert!(z.len() >= x.len() + y.len());
        if x.len() == 0 || y.len() == 0 {
            z.iter_mut().for_each(|d| *d = 0);
            return;
        }
        let mut next: digit_t;
        let mut next_carry: digit_t = 0;
        let mut carry: digit_t = 0;
        // Unrolled first iteration: it's trivial.
        let mut next_uninit: digit_t = 0;
        z[0] = digit_mul(x[0], y[0], &mut next_uninit);
        next = next_uninit;

        let mut i = 1;
        // Unrolled second iteration: a little less setup.
        if i < y.len() {
            let mut zi = next;
            next = 0;
            body!(0, 1, x, y, i, z, zi, carry, next, next_carry);
            i += 1;
        }
        // Main part: since X.len() >= Y.len() > i, no bounds checks are needed.
        while i < y.len() {
            let mut zi = digit_add2(next, carry, &mut carry);
            next = next_carry + carry;
            carry = 0;
            next_carry = 0;
            body!(0, i, x, y, i, z, zi, carry, next, next_carry);
            // AddWorkEstimate(i); // no direct translation
            i += 1;
        }
        // Last part: i exceeds Y now, we have to be careful about bounds.
        let loop_end = x.len() + y.len() - 2;
        while i <= loop_end {
            let max_x_index = std::cmp::min(i, x.len() - 1);
            let max_y_index = y.len() - 1;
            let min_x_index = i - max_y_index;

            let mut zi = digit_add2(next, carry, &mut carry);
            next = next_carry + carry;
            carry = 0;
            next_carry = 0;
            body!(min_x_index, max_x_index, x, y, i, z, zi, carry, next, next_carry);
            // AddWorkEstimate(max_x_index - min_x_index); // no direct translation
            i += 1;
        }

        // Write the last digit, and zero out any extra space in Z.
        let mut carry_last: digit_t = 0;
        z[i] = digit_add2(next, carry, &mut carry_last);
        debug_assert!(carry_last == 0);
        i += 1;
        for j in i..z.len() {
            z[j] = 0;
        }
    }
}