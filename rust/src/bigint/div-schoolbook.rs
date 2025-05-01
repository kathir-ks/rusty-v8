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
use std::ops::{Add, AddAssign, Sub, SubAssign};

mod bigint_internal {
    // Placeholder module.  Add necessary internal definitions here.
    pub type Digit = u32; // Assuming digit_t is u32 for example
}

mod digit_arithmetic {
    use super::bigint_internal::Digit;

    #[inline]
    pub fn digit_mul(x: Digit, y: Digit, high: &mut Digit) -> Digit {
        let res = (x as u64) * (y as u64);
        *high = (res >> 32) as Digit; // Assuming u32
        (res & 0xFFFFFFFF) as Digit  // Assuming u32
    }

    #[inline]
    pub fn digit_div(hi: Digit, lo: Digit, divisor: Digit, remainder: &mut Digit) -> Digit {
        let dividend = ((hi as u64) << 32) | (lo as u64);
        *remainder = (dividend % (divisor as u64)) as Digit;
        (dividend / (divisor as u64)) as Digit
    }
}

mod div_helpers {
    use super::bigint_internal::Digit;

    pub trait DigitsTrait {
        fn len(&self) -> usize;
        fn get(&self, index: usize) -> Digit;
    }

    impl<'a> DigitsTrait for &'a [Digit] {
        fn len(&self) -> usize {
            self.len()
        }

        fn get(&self, index: usize) -> Digit {
            self[index]
        }
    }

    impl<'a> DigitsTrait for &'a mut [Digit] {
        fn len(&self) -> usize {
            self.len()
        }

        fn get(&self, index: usize) -> Digit {
            self[index]
        }
    }
}

mod util {
    // Placeholder module for utility functions, may need adjustments
    use super::bigint_internal::Digit;

    #[inline]
    pub fn k_digit_bits() -> usize {
        32 // Assuming Digit is u32
    }

    pub fn greater_than_or_equal(a: &[Digit], b: &[Digit]) -> bool {
        if a.len() != b.len() {
            return a.len() > b.len();
        }

        for i in (0..a.len()).rev() {
            if a[i] > b[i] {
                return true;
            } else if a[i] < b[i] {
                return false;
            }
        }

        true
    }

    pub fn left_shift(dst: &mut [Digit], src: &[Digit], shift: usize) {
        if shift == 0 {
            dst[..src.len()].copy_from_slice(src);
            return;
        }
        let digit_shift = shift % k_digit_bits();
        let carry_shift = k_digit_bits() - digit_shift;
        let num_digits = src.len();
        let mut carry = 0;
        for i in 0..num_digits {
            let digit = src[i];
            dst[i] = (digit << digit_shift) | carry;
            carry = digit >> carry_shift;
        }
        dst[num_digits] = carry;
    }

    pub fn right_shift(dst: &mut [Digit], src: &[Digit], shift: usize) {
         if shift == 0 {
            dst[..src.len()].copy_from_slice(src);
            return;
        }

        let digit_shift = shift % k_digit_bits();
        let carry_shift = k_digit_bits() - digit_shift;

        let num_digits = src.len();
        let mut carry = 0;
        for i in (0..num_digits).rev() {
            let digit = src[i];
            dst[i] = (digit >> digit_shift) | carry;
            carry = digit << carry_shift;
        }
    }
}

mod vector_arithmetic {
    use super::bigint_internal::Digit;

    // Add Z += X and returns carry
    pub fn add_and_return_carry(z: &mut [Digit], z_in: &[Digit], x: &[Digit]) -> Digit {
        let n = x.len();
        let mut carry: Digit = 0;
        for i in 0..n {
            let sum = (z_in[i] as u64) + (x[i] as u64) + (carry as u64);
            z[i] = (sum & 0xFFFFFFFF) as Digit; // Assuming u32
            carry = (sum >> 32) as Digit;       // Assuming u32
        }
        carry
    }

    // Subtract Z -= X and returns borrow
    pub fn subtract_and_return_borrow(z: &mut [Digit], z_in: &[Digit], x: &[Digit]) -> Digit {
        let n = x.len();
        let mut borrow: Digit = 0;
        for i in 0..n {
            let diff = (z_in[i] as i64) - (x[i] as i64) - (borrow as i64);
            if diff < 0 {
                z[i] = (diff + (1i64 << 32)) as Digit; // Assuming u32
                borrow = 1;
            } else {
                z[i] = diff as Digit;
                borrow = 0;
            }
        }
        borrow
    }

    pub fn multiply_single(dst: &mut [Digit], src: &[Digit], factor: Digit) {
        let n = src.len();
        let mut carry: Digit = 0;
        for i in 0..n {
            let product = (src[i] as u64) * (factor as u64) + (carry as u64);
            dst[i] = (product & 0xFFFFFFFF) as Digit; // Assuming u32
            carry = (product >> 32) as Digit;       // Assuming u32
        }
        dst[n] = carry;
    }

    pub fn clear(digits: &mut [Digit]) {
        for digit in digits.iter_mut() {
            *digit = 0;
        }
    }
}

pub mod bigint {
    use super::bigint_internal::Digit;
    use super::digit_arithmetic;
    use super::div_helpers::DigitsTrait;
    use super::util;
    use super::vector_arithmetic;
    use std::cmp::Ordering;
    use std::fmt;
    use std::fmt::Debug;
    use std::u32;
    use std::vec;

    #[derive(Debug, PartialEq, Eq)]
    pub struct Digits {
        data: Vec<Digit>,
    }

    impl Digits {
        pub fn new(data: Vec<Digit>) -> Self {
            Digits { data }
        }

        pub fn len(&self) -> usize {
            self.data.len()
        }

        pub fn get(&self, index: usize) -> Digit {
            self.data[index]
        }

        pub fn as_slice(&self) -> &[Digit] {
            &self.data
        }

        pub fn as_mut_slice(&mut self) -> &mut [Digit] {
            &mut self.data
        }

        pub fn from_slice(slice: &[Digit]) -> Self {
            Digits {
                data: slice.to_vec(),
            }
        }
    }

    impl<'a> From<&'a [Digit]> for Digits {
        fn from(slice: &'a [Digit]) -> Self {
            Digits::from_slice(slice)
        }
    }

    impl<'a> DigitsTrait for Digits {
        fn len(&self) -> usize {
            self.len()
        }

        fn get(&self, index: usize) -> Digit {
            self.get(index)
        }
    }

    #[derive(Debug)]
    pub struct ShiftedDigits {
        digits: Digits,
        shift: usize,
    }

    impl ShiftedDigits {
        pub fn new(digits: Digits, shift: usize) -> Self {
            ShiftedDigits { digits, shift }
        }
        pub fn shift(&self) -> usize {
            self.shift
        }
    }

    impl<'a> From<&'a Digits> for ShiftedDigits {
        fn from(digits: &'a Digits) -> Self {
            let mut leading_zeros: usize = 0;
            if digits.len() > 0 {
                let most_significant_digit = digits.get(digits.len() - 1);
                if most_significant_digit == 0 {
                    leading_zeros = 32;
                } else {
                    leading_zeros = most_significant_digit.leading_zeros() as usize;
                }
            }

            ShiftedDigits::new(Digits::from_slice(digits.as_slice()), leading_zeros)
        }
    }

    impl std::ops::Deref for ShiftedDigits {
        type Target = Digits;

        fn deref(&self) -> &Self::Target {
            &self.digits
        }
    }

    pub struct ScratchDigits {
        digits: Vec<Digit>,
    }

    impl ScratchDigits {
        pub fn new(size: usize) -> Self {
            ScratchDigits {
                digits: vec![0; size],
            }
        }

        pub fn clear(&mut self) {
            self.digits.iter_mut().for_each(|x| *x = 0);
        }

        pub fn resize(&mut self, new_size: usize) {
            self.digits.resize(new_size, 0);
        }
    }

    impl std::ops::Deref for ScratchDigits {
        type Target = Vec<Digit>;

        fn deref(&self) -> &Self::Target {
            &self.digits
        }
    }

    impl std::ops::DerefMut for ScratchDigits {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.digits
        }
    }

    pub struct ProcessorImpl {}

    impl ProcessorImpl {
        pub fn new() -> Self {
            ProcessorImpl {}
        }

        /// Computes Q(uotient) and remainder for A/b, such that
        /// Q = (A - remainder) / b, with 0 <= remainder < b.
        /// If Q.len == 0, only the remainder will be returned.
        /// Q may be the same as A for an in-place division.
        pub fn divide_single(
            &self,
            q: &mut [Digit],
            remainder: &mut Digit,
            a: &Digits,
            b: Digit,
        ) {
            assert!(b != 0);
            assert!(a.len() > 0);

            *remainder = 0;
            let length = a.len();
            if q.len() != 0 {
                if a.get(length - 1) >= b {
                    assert!(q.len() >= a.len());
                    for i in (0..length).rev() {
                        let temp_rem = *remainder;
                        q[i] = digit_arithmetic::digit_div(temp_rem, a.get(i), b, remainder);
                    }
                    for i in length..q.len() {
                        q[i] = 0;
                    }
                } else {
                    assert!(q.len() >= a.len() - 1);
                    *remainder = a.get(length - 1);
                    for i in (0..length - 1).rev() {
                        let temp_rem = *remainder;
                        q[i] = digit_arithmetic::digit_div(temp_rem, a.get(i), b, remainder);
                    }
                    for i in length - 1..q.len() {
                        q[i] = 0;
                    }
                }
            } else {
                for i in (0..length).rev() {
                    let mut temp_rem = *remainder;
                    digit_arithmetic::digit_div(temp_rem, a.get(i), b, remainder);
                }
            }
        }

        /// Computes Q(uotient) and R(emainder) for A/B, such that
        /// Q = (A - R) / B, with 0 <= R < B.
        /// Both Q and R are optional: callers that are only interested in one of them
        /// can pass the other with len == 0.
        /// If Q is present, its length must be at least A.len - B.len + 1.
        /// If R is present, its length must be at least B.len.
        /// See Knuth, Volume 2, section 4.3.1, Algorithm D.
        pub fn divide_schoolbook(
            &self,
            q: &mut [Digit],
            r: &mut [Digit],
            a: &Digits,
            b: &Digits,
        ) {
            assert!(b.len() >= 2); // Use DivideSingle otherwise.
            assert!(a.len() >= b.len()); // No-op otherwise.
            //DCHECK(Q.len() == 0 || QLengthOK(Q, A, B));
            //DCHECK(R.len() == 0 || R.len() >= B.len());
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
            let b_normalized = ShiftedDigits::from(b);
            let b_shifted = &b_normalized.digits;
            // U holds the (continuously updated) remaining part of the dividend, which
            // eventually becomes the remainder.
            let mut u = ScratchDigits::new(a.len() + 1);
            util::left_shift(u.as_mut_slice(), a.as_slice(), b_normalized.shift());

            // D2.
            // Iterate over the dividend's digits (like the "grad school" algorithm).
            // {vn1} is the divisor's most significant digit.
            let vn1 = b_shifted.get(n - 1);
            for j in (0..=m).rev() {
                // D3.
                // Estimate the current iteration's quotient digit (see Knuth for details).
                // {qhat} is the current quotient digit.
                let mut qhat = u32::MAX;
                // {ujn} is the dividend's most significant remaining digit.
                let ujn = u[j + n];
                if ujn != vn1 {
                    // {rhat} is the current iteration's remainder.
                    let mut rhat: Digit = 0;
                    // Estimate the current quotient digit by dividing the most significant
                    // digits of dividend and divisor. The result will not be too small,
                    // but could be a bit too large.
                    qhat = digit_arithmetic::digit_div(ujn, u[j + n - 1], vn1, &mut rhat);

                    // Decrement the quotient estimate as needed by looking at the next
                    // digit, i.e. by testing whether
                    // qhat * v_{n-2} > (rhat << kDigitBits) + u_{j+n-2}.
                    let vn2 = b_shifted.get(n - 2);
                    let ujn2 = u[j + n - 2];
                    while self.product_greater_than(qhat, vn2, rhat, ujn2) {
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
                    vector_arithmetic::clear(&mut qhatv);
                } else {
                    vector_arithmetic::multiply_single(qhatv.as_mut_slice(), b_shifted.as_slice(), qhat);
                }
                let c = self.inplace_sub(&mut u[j..], &Digits::from_slice(&qhatv[..n + 1]));
                if c != 0 {
                    let mut borrow = self.inplace_add(&mut u[j..], b_shifted);
                    u[j + n] = u[j + n] + borrow;
                    qhat -= 1;
                }

                if q.len() != 0 {
                    if j >= q.len() {
                        assert!(qhat == 0);
                    } else {
                        q[j] = qhat;
                    }
                }
            }
            if r.len() != 0 {
                util::right_shift(r, u.as_slice(), b_normalized.shift());
            }
            // If Q has extra storage, clear it.
            for i in m + 1..q.len() {
                q[i] = 0;
            }
        }

        fn product_greater_than(
            &self,
            factor1: Digit,
            factor2: Digit,
            high: Digit,
            low: Digit,
        ) -> bool {
            let mut result_high: Digit = 0;
            let result_low = digit_arithmetic::digit_mul(factor1, factor2, &mut result_high);
            result_high > high || (result_high == high && result_low > low)
        }

        // Z += X. Returns the "carry" (0 or 1) after adding all of X's digits.
        #[inline]
        fn inplace_add(&self, z: &mut [Digit], x: &Digits) -> Digit {
            vector_arithmetic::add_and_return_carry(z, z, x.as_slice())
        }

        // Z -= X. Returns the "borrow" (0 or 1) after subtracting all of X's digits.
        #[inline]
        fn inplace_sub(&self, z: &mut [Digit], x: &Digits) -> Digit {
            vector_arithmetic::subtract_and_return_borrow(z, z, x.as_slice())
        }
    }
}