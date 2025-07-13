// Converted from V8 C++ source files:
// Header: N/A
// Implementation: mul-karatsuba.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Karatsuba multiplication. This is loosely based on Go's implementation
// found at https://golang.org/src/math/big/nat.go, licensed as follows:
//
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file [1].
//
// [1] https://golang.org/LICENSE

use std::cmp;
use std::mem;

use crate::bigint::{
    digit_add2, digit_add, digit_sub2, digit_sub, GreaterThanOrEqual,
    MultiplySingle, MultiplySchoolbook, AddAndReturnOverflow, SubAndReturnBorrow,
    BitLength, RoundUp, ScratchDigits,
};

const V8_ADVANCED_BIGINT_ALGORITHMS: bool = false;
const kKaratsubaThreshold: usize = 32;

macro_rules! maybe_terminate {
    () => {
        if false { // should_terminate() placeholder
            return;
        }
    };
}

pub struct ProcessorImpl {}

impl ProcessorImpl {
    // If Karatsuba is the best supported algorithm, then it must check for
    // termination requests. If there are more advanced algorithms available
    // for larger inputs, then Karatsuba will only be used for sufficiently
    // small chunks that checking for termination requests is not necessary.
    // The Karatsuba algorithm sometimes finishes more quickly when the
    // input length is rounded up a bit. This method encodes some heuristics
    // to accomplish this. The details have been determined experimentally.
    fn round_up_len(len: usize) -> usize {
        if len <= 36 {
            Self::round_up(len, 2)
        } else {
            // Keep the 4 or 5 most significant non-zero bits.
            let mut shift = Self::bit_length(len) - 5;
            if (len >> shift) >= 0x18 {
                shift += 1;
            }
            // Round up, unless we're only just above the threshold. This smoothes
            // the steps by which time goes up as input size increases.
            let additive = ((1 << shift) - 1);
            if shift >= 2 && (len & additive) < (1 << (shift - 2)) {
                return len;
            }
            ((len + additive) >> shift) << shift
        }
    }

    // This method makes the final decision how much to bump up the input size.
    fn karatsuba_length(n: usize) -> usize {
        let mut n = Self::round_up_len(n);
        let mut i = 0;
        while n > kKaratsubaThreshold {
            n >>= 1;
            i += 1;
        }
        n << i
    }

    // Performs the specific subtraction required by {KaratsubaMain} below.
    fn karatsuba_subtraction_helper(
        result: &mut [digit_t],
        x: &[digit_t],
        y: &[digit_t],
        sign: &mut i32,
    ) {
        let mut x_norm = x.to_vec();
        let mut y_norm = y.to_vec();
        while let Some(0) = x_norm.last() { x_norm.pop(); }
        while let Some(0) = y_norm.last() { y_norm.pop(); }
        let x_digits = &x_norm[..];
        let y_digits = &y_norm[..];

        let mut borrow: digit_t = 0;
        let mut i = 0;
        let mut x_copy = x_digits.to_vec();
        let mut y_copy = y_digits.to_vec();
        if !Self::greater_than_or_equal(&x_copy, &y_copy) {
            *sign = -(*sign);
            mem::swap(&mut x_copy, &mut y_copy);
        }
        let x_use = &x_copy[..];
        let y_use = &y_copy[..];
        for i in 0..y_use.len() {
            result[i] = digit_sub2(x_use[i], y_use[i], borrow, &mut borrow);
        }
        for i in y_use.len()..x_use.len() {
            result[i] = digit_sub(x_use[i], borrow, &mut borrow);
        }
        assert!(borrow == 0);
        for i in x_use.len()..result.len() {
            result[i] = 0;
        }
    }

    pub fn multiply_karatsuba(z: &mut [digit_t], x: &[digit_t], y: &[digit_t]) {
        assert!(x.len() >= y.len());
        assert!(y.len() >= kKaratsubaThreshold);
        assert!(z.len() >= x.len() + y.len());
        let k = Self::karatsuba_length(y.len());
        let scratch_len = 4 * k;
        let mut scratch = vec![0 as digit_t; scratch_len];
        Self::karatsuba_start(z, x, y, &mut scratch, k);
    }

    // Entry point for Karatsuba-based multiplication, takes care of inputs
    // with unequal lengths by chopping the larger into chunks.
    fn karatsuba_start(
        z: &mut [digit_t],
        x: &[digit_t],
        y: &[digit_t],
        scratch: &mut [digit_t],
        k: usize,
    ) {
        Self::karatsuba_main(z, x, y, scratch, k);
        maybe_terminate!();
        for i in 2 * k..z.len() {
            z[i] = 0;
        }
        if k < y.len() || x.len() != y.len() {
            let mut t = vec![0 as digit_t; 2 * k];
            // Add X0 * Y1 * b.
            let x0 = &x[0..cmp::min(k, x.len())];
            let y1 = &y[cmp::min(k, y.len())..];
            if y1.len() > 0 {
                Self::karatsuba_chunk(&mut t, x0, y1, scratch);
                maybe_terminate!();
                Self::add_and_return_overflow(&mut z[k..], &t); // Can't overflow.
            }

            // Add Xi * Y0 << i and Xi * Y1 * b << (i + k).
            let y0 = &y[0..cmp::min(k, y.len())];
            let mut i = k;
            while i < x.len() {
                let xi = &x[i..cmp::min(i + k, x.len())];
                Self::karatsuba_chunk(&mut t, xi, y0, scratch);
                maybe_terminate!();
                Self::add_and_return_overflow(&mut z[i..], &t); // Can't overflow.
                if y1.len() > 0 {
                    Self::karatsuba_chunk(&mut t, xi, y1, scratch);
                    maybe_terminate!();
                    Self::add_and_return_overflow(&mut z[(i + k)..], &t); // Can't overflow.
                }
                i += k;
            }
        }
    }

    // Entry point for chunk-wise multiplications, selects an appropriate
    // algorithm for the inputs based on their sizes.
    fn karatsuba_chunk(z: &mut [digit_t], x: &[digit_t], y: &[digit_t], scratch: &mut [digit_t]) {
        let mut x_norm = x.to_vec();
        let mut y_norm = y.to_vec();
        while let Some(0) = x_norm.last() { x_norm.pop(); }
        while let Some(0) = y_norm.last() { y_norm.pop(); }
        let x_digits = &x_norm[..];
        let y_digits = &y_norm[..];

        if x_digits.len() == 0 || y_digits.len() == 0 {
            z.iter_mut().for_each(|x| *x = 0);
            return;
        }
        if x_digits.len() < y_digits.len() {
            Self::karatsuba_chunk(z, y_digits, x_digits, scratch);
            return;
        }
        if y_digits.len() == 1 {
            Self::multiply_single(z, x_digits, y_digits[0]);
            return;
        }
        if y_digits.len() < kKaratsubaThreshold {
            Self::multiply_schoolbook(z, x_digits, y_digits);
            return;
        }
        let k = Self::karatsuba_length(y_digits.len());
        assert!(scratch.len() >= 4 * k);
        Self::karatsuba_start(z, x_digits, y_digits, scratch, k);
    }

    // The main recursive Karatsuba method.
    fn karatsuba_main(
        z: &mut [digit_t],
        x: &[digit_t],
        y: &[digit_t],
        scratch: &mut [digit_t],
        n: usize,
    ) {
        if n < kKaratsubaThreshold {
            let mut x_norm = x.to_vec();
            let mut y_norm = y.to_vec();
            while let Some(0) = x_norm.last() { x_norm.pop(); }
            while let Some(0) = y_norm.last() { y_norm.pop(); }
            let x_digits = &x_norm[..];
            let y_digits = &y_norm[..];
            if x_digits.len() >= y_digits.len() {
                Self::multiply_schoolbook(&mut z[0..2 * n], x_digits, y_digits);
            } else {
                Self::multiply_schoolbook(&mut z[0..2 * n], y_digits, x_digits);
            }
            return;
        }
        assert!(scratch.len() >= 4 * n);
        assert!((n & 1) == 0);
        let n2 = n >> 1;
        let x0 = &x[0..n2];
        let x1 = &x[n2..n];
        let y0 = &y[0..n2];
        let y1 = &y[n2..n];
        let (scratch_for_recursion, rest) = scratch.split_at_mut(2 * n);
        let (p0, scratch_rest) = rest.split_at_mut(n);

        Self::karatsuba_main(p0, x0, y0, scratch_for_recursion, n2);
        maybe_terminate!();
        for i in 0..n {
            z[i] = p0[i];
        }

        let (p2, _) = scratch_rest.split_at_mut(n);

        Self::karatsuba_main(p2, x1, y1, scratch_for_recursion, n2);
        maybe_terminate!();
        let z2 = &mut z[n..];
        let end = cmp::min(z2.len(), p2.len());
        for i in 0..end {
            z2[i] = p2[i];
        }
        for i in end..n {
            assert!(p2[i] == 0);
        }

        // The intermediate result can be one digit too large; the subtraction
        // below will fix this.
        let mut overflow: digit_t = Self::add_and_return_overflow(&mut z[n2..], p0);
        overflow += Self::add_and_return_overflow(&mut z[n2..], p2);

        let (x_diff, rest2) = scratch.split_at_mut(n2);
        let (y_diff, _) = rest2.split_at_mut(n2);

        let mut sign = 1;
        Self::karatsuba_subtraction_helper(x_diff, x1, x0, &mut sign);
        Self::karatsuba_subtraction_helper(y_diff, y0, y1, &mut sign);

        let (p1, _) = scratch_rest.split_at_mut(n);

        Self::karatsuba_main(p1, x_diff, y_diff, scratch_for_recursion, n2);

        if sign > 0 {
            overflow += Self::add_and_return_overflow(&mut z[n2..], p1);
        } else {
            overflow -= Self::sub_and_return_borrow(&mut z[n2..], p1);
        }
        // The intermediate result may have been bigger, but the final result fits.
        assert!(overflow == 0);
        std::hint::black_box(overflow);
    }

    fn bit_length(x: usize) -> usize {
        if x == 0 {
            return 0;
        }
        usize::BITS as usize - x.leading_zeros() as usize
    }

    fn round_up(x: usize, multiple: usize) -> usize {
        (x + multiple - 1) / multiple * multiple
    }

    fn greater_than_or_equal(x: &[digit_t], y: &[digit_t]) -> bool {
        if x.len() != y.len() {
            return x.len() > y.len();
        }
        for i in (0..x.len()).rev() {
            if x[i] != y[i] {
                return x[i] > y[i];
            }
        }
        true
    }

    fn add_and_return_overflow(a: &mut [digit_t], b: &[digit_t]) -> digit_t {
        let mut carry: digit_t = 0;
        let len = cmp::min(a.len(), b.len());
        for i in 0..len {
            carry = digit_add2(a[i], b[i], carry, &mut a[i]);
        }
        if b.len() > a.len() {
          for i in len..b.len() {
            let new_carry = digit_add(b[i], carry, &mut carry);
          }
        }
        carry
    }

    fn sub_and_return_borrow(a: &mut [digit_t], b: &[digit_t]) -> digit_t {
        let mut borrow: digit_t = 0;
        let len = cmp::min(a.len(), b.len());
        for i in 0..len {
            borrow = digit_sub2(a[i], b[i], borrow, &mut a[i]);
        }
        borrow
    }

    fn multiply_single(z: &mut [digit_t], x: &[digit_t], y: digit_t) {
        let mut carry: digit_t = 0;
        for i in 0..x.len() {
            let product = (x[i] as u64) * (y as u64) + (carry as u64);
            z[i] = (product & ((1 << 32) - 1)) as digit_t;
            carry = (product >> 32) as digit_t;
        }
        if z.len() > x.len() {
            z[x.len()] = carry;
            for i in (x.len() + 1)..z.len() {
              z[i] = 0;
            }
        }
    }

    fn multiply_schoolbook(z: &mut [digit_t], x: &[digit_t], y: &[digit_t]) {
        for i in 0..z.len() { z[i] = 0; }

        for i in 0..y.len() {
            let mut carry: digit_t = 0;
            for j in 0..x.len() {
                let product: u64 = (x[j] as u64) * (y[i] as u64) + (z[i + j] as u64) + (carry as u64);
                z[i + j] = (product & ((1u64 << 32) - 1)) as digit_t;
                carry = (product >> 32) as digit_t;
            }
            if i + x.len() < z.len() {
                z[i + x.len()] = carry;
            }
        }
    }
}

type digit_t = u32;
