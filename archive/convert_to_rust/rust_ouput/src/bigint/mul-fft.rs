// Converted from V8 C++ source files:
// Header: N/A
// Implementation: mul-fft.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// FFT-based multiplication, due to Schönhage and Strassen.
// This implementation mostly follows the description given in:
// Christoph Lüders: Fast Multiplication of Large Integers,
// http://arxiv.org/abs/1503.04955

#![allow(dead_code)]
#![allow(non_snake_case)]

use std::cmp::{max, min};
use std::mem::MaybeUninit;
use std::sync::atomic::AtomicU16;

// use crate::bigint::bigint_internal::*;
// use crate::bigint::digit_arithmetic::*;
// use crate::bigint::util::*;

const kDigitBits: usize = 32;
const kLog2DigitBits: usize = 5;
const kFftInnerThreshold: usize = 128;

type digit_t = u32;
type signed_digit_t = i32;

#[derive(Debug, Clone)]
struct Status {}

#[derive(Debug, Clone)]
struct Digits<'a> {
    ptr: &'a [digit_t],
    len: usize,
}

impl<'a> Digits<'a> {
    fn new(ptr: &'a [digit_t], len: usize) -> Self {
        Digits { ptr, len }
    }

    fn digits(&self) -> &[digit_t] {
        self.ptr
    }

    fn len(&self) -> usize {
        self.len
    }
}

#[derive(Debug, Clone)]
struct RWDigits<'a> {
    ptr: &'a mut [digit_t],
    len: usize,
}

impl<'a> RWDigits<'a> {
    fn new(ptr: &'a mut [digit_t], len: usize) -> Self {
        RWDigits { ptr, len }
    }

    fn digits(&mut self) -> &mut [digit_t] {
        self.ptr
    }

    fn len(&self) -> usize {
        self.len
    }

    fn clear(&mut self) {
        for i in 0..self.len {
            self.ptr[i] = 0;
        }
    }
}

impl<'a> std::ops::Index<usize> for RWDigits<'a> {
    type Output = digit_t;

    fn index(&self, index: usize) -> &Self::Output {
        &self.ptr[index]
    }
}

impl<'a> std::ops::IndexMut<usize> for RWDigits<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.ptr[index]
    }
}

#[derive(Debug, Clone)]
struct ScratchDigits {
    ptr: Box<[digit_t]>,
    len: usize,
}

impl ScratchDigits {
    fn new(len: usize) -> Self {
        let mut data = Vec::with_capacity(len);
        for _ in 0..len {
            data.push(0);
        }
        ScratchDigits {
            ptr: data.into_boxed_slice(),
            len,
        }
    }

    fn get(&mut self) -> &mut [digit_t] {
        &mut self.ptr
    }
}

fn digit_add2(a: digit_t, b: digit_t, carry: &mut digit_t) -> digit_t {
    let sum = a as u64 + b as u64 + *carry as u64;
    *carry = (sum >> kDigitBits) as digit_t;
    (sum as digit_t)
}

fn digit_add3(a: digit_t, b: digit_t, c: digit_t, carry: &mut digit_t) -> digit_t {
    let sum = a as u64 + b as u64 + c as u64 + *carry as u64;
    *carry = (sum >> kDigitBits) as digit_t;
    (sum as digit_t)
}

fn digit_sub(a: digit_t, b: digit_t, borrow: &mut digit_t) -> digit_t {
    let diff = a as u64 - b as u64 - *borrow as u64;
    *borrow = if diff >> kDigitBits != 0 { 1 } else { 0 };
    (diff as digit_t)
}

fn digit_sub2(a: digit_t, b: digit_t, borrow: &mut digit_t, borrow2: &mut digit_t) -> digit_t {
    let diff = a as u64 - b as u64 - *borrow as u64;
    *borrow2 = if diff >> kDigitBits != 0 { 1 } else { 0 };
    (diff as digit_t)
}

fn BitLength(x: usize) -> usize {
    if x == 0 {
        0
    } else {
        kDigitBits - (x as digit_t).leading_zeros() as usize
    }
}

fn CountTrailingZeros(x: usize) -> usize {
    (x as digit_t).trailing_zeros() as usize
}

fn RoundUp(x: usize, k: usize) -> usize {
    (x + k - 1) & !(k - 1)
}

mod bigint {
    use super::*;

    mod internal {
        use super::*;
    }

    mod util {
        use super::*;
    }

    mod digit_arithmetic {
        use super::*;
    }

    namespace! {
        // === IMPLEMENTATION CONTENT ===
        // Copyright 2021 the V8 project authors. All rights reserved.
        // Use of this source code is governed by a BSD-style license that can be
        // found in the LICENSE file.

        // FFT-based multiplication, due to Schönhage and Strassen.
        // This implementation mostly follows the description given in:
        // Christoph Lüders: Fast Multiplication of Large Integers,
        // http://arxiv.org/abs/1503.04955

        // #include "src/bigint/bigint-internal.h"
        // #include "src/bigint/digit-arithmetic.h"
        // #include "src/bigint/util.h"

        // namespace v8 {
        // namespace bigint {

        mod _ {}
        // namespace {

        ////////////////////////////////////////////////////////////////////////////////
        // Part 1: Functions for "mod F_n" arithmetic.
        // F_n is of the shape 2^K + 1, and for convenience we use K to count the
        // number of digits rather than the number of bits, so F_n (or K) are implicit
        // and deduced from the length {len} of the digits array.

        // Helper function for {ModFn} below.
        fn ModFn_Helper(x: &mut [digit_t], len: usize, high: signed_digit_t) {
            if high > 0 {
                let mut borrow: digit_t = high as digit_t;
                x[len - 1] = 0;
                for i in 0..len {
                    x[i] = digit_sub(x[i], borrow, &mut borrow);
                    if borrow == 0 {
                        break;
                    }
                }
            } else {
                let mut carry: digit_t = (-high) as digit_t;
                x[len - 1] = 0;
                for i in 0..len {
                    x[i] = digit_add2(x[i], carry, &mut carry);
                    if carry == 0 {
                        break;
                    }
                }
            }
        }

        // {x} := {x} mod F_n, assuming that {x} is "slightly" larger than F_n (e.g.
        // after addition of two numbers that were mod-F_n-normalized before).
        fn ModFn(x: &mut [digit_t], len: usize) {
            let k: usize = len - 1;
            let mut high: signed_digit_t = x[k] as signed_digit_t;
            if high == 0 {
                return;
            }
            ModFn_Helper(x, len, high);
            high = x[k] as signed_digit_t;
            if high == 0 {
                return;
            }
            //DCHECK(high == 1 || high == -1);
            if high != 1 && high != -1 {
                println!("DCHECK failed: high = {}", high);
                panic!("DCHECK failed");
            }
            ModFn_Helper(x, len, high);
            high = x[k] as signed_digit_t;
            if high == -1 {
                ModFn_Helper(x, len, high);
            }
        }

        // {dest} := {src} mod F_n, assuming that {src} is about twice as long as F_n
        // (e.g. after multiplication of two numbers that were mod-F_n-normalized
        // before).
        // {len} is length of {dest}; {src} is twice as long.
        fn ModFnDoubleWidth(dest: &mut [digit_t], src: &[digit_t], len: usize) {
            let k: usize = len - 1;
            let mut borrow: digit_t = 0;
            for i in 0..k {
                dest[i] = digit_sub2(src[i], src[i + k], &mut borrow, &mut borrow);
            }
            dest[k] = digit_sub2(0, src[2 * k], &mut borrow, &mut borrow);
            // {borrow} may be non-zero here, that's OK as {ModFn} will take care of it.
            ModFn(dest, len);
        }

        // Sets {sum} := {a} + {b} and {diff} := {a} - {b}, which is more efficient
        // than computing sum and difference separately. Applies "mod F_n" normalization
        // to both results.
        fn SumDiff(sum: &mut [digit_t], diff: &mut [digit_t], a: &[digit_t], b: &[digit_t], len: usize) {
            let mut carry: digit_t = 0;
            let mut borrow: digit_t = 0;
            for i in 0..len {
                // Read both values first, because inputs and outputs can overlap.
                let ai: digit_t = a[i];
                let bi: digit_t = b[i];
                sum[i] = digit_add3(ai, bi, 0, &mut carry);
                diff[i] = digit_sub2(ai, bi, &mut borrow, &mut borrow);
            }
            ModFn(sum, len);
            ModFn(diff, len);
        }

        // {result} := ({input} << shift) mod F_n, where shift >= K.
        fn ShiftModFn_Large(result: &mut [digit_t], input: &[digit_t], digit_shift: usize, bits_shift: usize, K: usize) {
            // If {digit_shift} is greater than K, we use the following transformation
            // (where, since everything is mod 2^K + 1, we are allowed to add or
            // subtract any multiple of 2^K + 1 at any time):
            //      x * 2^{K+m}   mod 2^K + 1
            //   == x * 2^K * 2^m - (2^K + 1)*(x * 2^m)   mod 2^K + 1
            //   == x * 2^K * 2^m - x * 2^K * 2^m - x * 2^m   mod 2^K + 1
            //   == -x * 2^m   mod 2^K + 1
            // So the flow is the same as for m < K, but we invert the subtraction's
            // operands. In order to avoid underflow, we virtually initialize the
            // result to 2^K + 1:
            //   input  =  [ iK ][iK-1] ....  .... [ i1 ][ i0 ]
            //   result =  [   1][0000] ....  .... [0000][0001]
            //            +                  [ iK ] .... [ iX ]
            //            -      [iX-1] .... [ i0 ]
            //DCHECK(digit_shift >= K);
            if digit_shift < K {
                println!("DCHECK failed: digit_shift = {}, K = {}", digit_shift, K);
                panic!("DCHECK failed");
            }
            let mut digit_shift_mut = digit_shift;
            digit_shift_mut -= K;
            let mut borrow: digit_t = 0;
            if bits_shift == 0 {
                let mut carry: digit_t = 1;
                for i in 0..digit_shift_mut {
                    result[i] = digit_add2(input[i + K - digit_shift_mut], carry, &mut carry);
                }
                result[digit_shift_mut] = digit_sub(input[K] + carry, input[0], &mut borrow);
                for i in digit_shift_mut + 1..K {
                    let d: digit_t = input[i - digit_shift_mut];
                    result[i] = digit_sub2(0, d, &mut borrow, &mut borrow);
                }
            } else {
                let mut add_carry: digit_t = 1;
                let mut input_carry: digit_t =
                    input[K - digit_shift_mut - 1] >> (kDigitBits - bits_shift);
                for i in 0..digit_shift_mut {
                    let d: digit_t = input[i + K - digit_shift_mut];
                    let summand: digit_t = (d << bits_shift) | input_carry;
                    result[i] = digit_add2(summand, add_carry, &mut add_carry);
                    input_carry = d >> (kDigitBits - bits_shift);
                }
                {
                    // result[digit_shift] = (add_carry + iK_part) - i0_part
                    let d: digit_t = input[K];
                    let iK_part: digit_t = (d << bits_shift) | input_carry;
                    let mut iK_carry: digit_t = d >> (kDigitBits - bits_shift);
                    let sum: digit_t = digit_add2(add_carry, iK_part, &mut add_carry);
                    // {iK_carry} is less than a full digit, so we can merge {add_carry}
                    // into it without overflow.
                    iK_carry += add_carry;
                    let d: digit_t = input[0];
                    let i0_part: digit_t = d << bits_shift;
                    result[digit_shift_mut] = digit_sub(sum, i0_part, &mut borrow);
                    input_carry = d >> (kDigitBits - bits_shift);
                    if digit_shift_mut + 1 < K {
                        let d: digit_t = input[1];
                        let subtrahend: digit_t = (d << bits_shift) | input_carry;
                        result[digit_shift_mut + 1] =
                            digit_sub2(iK_carry, subtrahend, &mut borrow, &mut borrow);
                        input_carry = d >> (kDigitBits - bits_shift);
                    }
                }
                for i in digit_shift_mut + 2..K {
                    let d: digit_t = input[i - digit_shift_mut];
                    let subtrahend: digit_t = (d << bits_shift) | input_carry;
                    result[i] = digit_sub2(0, subtrahend, &mut borrow, &mut borrow);
                    input_carry = d >> (kDigitBits - bits_shift);
                }
            }
            // The virtual 1 in result[K] should be eliminated by {borrow}. If there
            // is no borrow, then the virtual initialization was too much. Subtract
            // 2^K + 1.
            result[K] = 0;
            if borrow != 1 {
                borrow = 1;
                for i in 0..K {
                    result[i] = digit_sub(result[i], borrow, &mut borrow);
                    if borrow == 0 {
                        break;
                    }
                }
                if borrow != 0 {
                    // The result must be 2^K.
                    for i in 0..K {
                        result[i] = 0;
                    }
                    result[K] = 1;
                }
            }
        }

        // Sets {result} := {input} * 2^{power_of_two} mod 2^{K} + 1.
        // This function is highly relevant for overall performance.
        fn ShiftModFn(result: &mut [digit_t], input: &[digit_t], power_of_two: usize, K: usize, zero_above: usize) {
            // The modulo-reduction amounts to a subtraction, which we combine
            // with the shift as follows:
            //   input  =  [ iK ][iK-1] ....  .... [ i1 ][ i0 ]
            //   result =        [iX-1] .... [ i0 ] <---------- shift by {power_of_two}
            //            -                  [ iK ] .... [ iX ]
            // where "X" is the index "K - digit_shift".
            let mut digit_shift: usize = power_of_two / kDigitBits;
            let bits_shift: usize = power_of_two % kDigitBits;
            // By an analogous construction to the "digit_shift >= K" case,
            // it turns out that:
            //    x * 2^{2K+m} == x * 2^m   mod 2^K + 1.
            while digit_shift >= 2 * K {
                digit_shift -= 2 * K; // Faster than '%'!
            }
            if digit_shift >= K {
                return ShiftModFn_Large(result, input, digit_shift, bits_shift, K);
            }
            let mut borrow: digit_t = 0;
            if bits_shift == 0 {
                // We do a single pass over {input}, starting by copying digits [i1] to
                // [iX-1] to result indices digit_shift+1 to K-1.
                let mut i: usize = 1;
                // Read input digits unless we know they are zero.
                let cap: usize = min(K - digit_shift, zero_above);
                while i < cap {
                    result[i + digit_shift] = input[i];
                    i += 1;
                }
                // Any remaining work can hard-code the knowledge that input[i] == 0.
                while i < K - digit_shift {
                    //DCHECK(input[i] == 0);
                    if input[i] != 0 {
                        println!("DCHECK failed: input[i] = {}", input[i]);
                        panic!("DCHECK failed");
                    }
                    result[i + digit_shift] = 0;
                    i += 1;
                }
                // Second phase: subtract input digits [iX] to [iK] from (virtually) zero-
                // initialized result indices 0 to digit_shift-1.
                let cap: usize = min(K, zero_above);
                while i < cap {
                    let d: digit_t = input[i];
                    result[i - K + digit_shift] = digit_sub2(0, d, &mut borrow, &mut borrow);
                    i += 1;
                }
                // Any remaining work can hard-code the knowledge that input[i] == 0.
                while i < K {
                    //DCHECK(input[i] == 0);
                    if input[i] != 0 {
                        println!("DCHECK failed: input[i] = {}", input[i]);
                        panic!("DCHECK failed");
                    }
                    result[i - K + digit_shift] = digit_sub(0, borrow, &mut borrow);
                    i += 1;
                }
                // Last step: subtract [iK] from [i0] and store at result index digit_shift.
                result[digit_shift] = digit_sub2(input[0], input[K], &mut borrow, &mut borrow);
            } else {
                // Same flow as before, but taking bits_shift != 0 into account.
                // First phase: result indices digit_shift+1 to K.
                let mut carry: digit_t = 0;
                let mut i: usize = 0;
                // Read input digits unless we know they are zero.
                let cap: usize = min(K - digit_shift, zero_above);
                while i < cap {
                    let d: digit_t = input[i];
                    result[i + digit_shift] = (d << bits_shift) | carry;
                    carry = d >> (kDigitBits - bits_shift);
                    i += 1;
                }
                // Any remaining work can hard-code the knowledge that input[i] == 0.
                while i < K - digit_shift {
                    //DCHECK(input[i] == 0);
                    if input[i] != 0 {
                        println!("DCHECK failed: input[i] = {}", input[i]);
                        panic!("DCHECK failed");
                    }
                    result[i + digit_shift] = carry;
                    carry = 0;
                    i += 1;
                }
                // Second phase: result indices 0 to digit_shift - 1.
                let cap: usize = min(K, zero_above);
                while i < cap {
                    let d: digit_t = input[i];
                    result[i - K + digit_shift] =
                        digit_sub2(0, (d << bits_shift) | carry, &mut borrow, &mut borrow);
                    carry = d >> (kDigitBits - bits_shift);
                    i += 1;
                }
                // Any remaining work can hard-code the knowledge that input[i] == 0.
                if i < K {
                    //DCHECK(input[i] == 0);
                    if input[i] != 0 {
                        println!("DCHECK failed: input[i] = {}", input[i]);
                        panic!("DCHECK failed");
                    }
                    result[i - K + digit_shift] = digit_sub2(0, carry, &mut borrow, &mut borrow);
                    carry = 0;
                    i += 1;
                }
                while i < K {
                    //DCHECK(input[i] == 0);
                    if input[i] != 0 {
                        println!("DCHECK failed: input[i] = {}", input[i]);
                        panic!("DCHECK failed");
                    }
                    result[i - K + digit_shift] = digit_sub(0, borrow, &mut borrow);
                    i += 1;
                }
                // Last step: compute result[digit_shift].
                let d: digit_t = input[K];
                result[digit_shift] = digit_sub2(
                    result[digit_shift], (d << bits_shift) | carry, &mut borrow, &mut borrow);
                // No carry left.
                //DCHECK((d >> (kDigitBits - bits_shift)) == 0);
                if (d >> (kDigitBits - bits_shift)) != 0 {
                    println!("DCHECK failed: (d >> (kDigitBits - bits_shift)) = {}", (d >> (kDigitBits - bits_shift)));
                    panic!("DCHECK failed");
                }
            }
            result[K] = 0;
            for i in digit_shift + 1..=K {
                if borrow > 0 {
                    result[i] = digit_sub(result[i], borrow, &mut borrow);
                } else {
                    break;
                }
            }
            if borrow > 0 {
                // Underflow means we subtracted too much. Add 2^K + 1.
                let mut carry: digit_t = 1;
                for i in 0..=K {
                    result[i] = digit_add2(result[i], carry, &mut carry);
                    if carry == 0 {
                        break;
                    }
                }
                result[K] = digit_add2(result[K], 1, &mut carry);
            }
        }

        ////////////////////////////////////////////////////////////////////////////////
        // Part 2: FFT-based multiplication is very sensitive to appropriate choice
        // of parameters. The following functions choose the parameters that the
        // subsequent actual computation will use. This is partially based on formal
        // constraints and partially on experimentally-determined heuristics.

        #[derive(Clone, Copy, Debug)]
        struct Parameters {
            // We never use the default values, but skipping zero-initialization
            // of these fields saddens and confuses MSan.
            m: i32,
            K: i32,
            n: i32,
            s: i32,
            r: i32,
        }

        impl Parameters {
            fn new() -> Self {
                Parameters {
                    m: 0,
                    K: 0,
                    n: 0,
                    s: 0,
                    r: 0,
                }
            }
        }

        // Computes parameters for the main calculation, given a bit length {N} and
        // an {m}. See the paper for details.
        fn ComputeParameters(N: i32, m: i32, params: &mut Parameters) {
            let mut N_mut = N * kDigitBits as i32;
            let n: i32 = 1 << m; // 2^m
            let nhalf: i32 = n >> 1;
            let mut s: i32 = (N_mut + n - 1) >> m; // ceil(N/n)
            s = RoundUp(s as usize, kDigitBits) as i32;
            let mut K: i32 = m + 2 * s + 1; // K must be at least this big...
            K = RoundUp(K as usize, nhalf as usize) as i32; // ...and a multiple of n/2.
            let mut r: i32 = K >> (m - 1); // Which multiple?

            // We want recursive calls to make progress, so force K to be a multiple
            // of 8 if it's above the recursion threshold. Otherwise, K must be a
            // multiple of kDigitBits.
            let threshold: i32 = if (K + 1 >= kFftInnerThreshold as i32 * kDigitBits as i32) {
                3 + kLog2DigitBits as i32
            } else {
                kLog2DigitBits as i32
            };
            let mut K_tz: i32 = CountTrailingZeros(K as usize) as i32;
            while K_tz < threshold {
                K += (1 << K_tz);
                r = K >> (m - 1);
                K_tz = CountTrailingZeros(K as usize) as i32;
            }

            //DCHECK(K % kDigitBits == 0);
            if K % kDigitBits as i32 != 0 {
                println!("DCHECK failed: K = {}, kDigitBits = {}", K, kDigitBits);
                panic!("DCHECK failed");
            }
            //DCHECK(s % kDigitBits == 0);
            if s % kDigitBits as i32 != 0 {
                println!("DCHECK failed: s = {}, kDigitBits = {}", s, kDigitBits);
                panic!("DCHECK failed");
            }
            params.K = K / kDigitBits as i32;
            params.s = s / kDigitBits as i32;
            params.n = n;
            params.r = r;
        }

        // Computes parameters for recursive invocations ("inner layer").
        fn ComputeParameters_Inner(N: i32, params: &mut Parameters) {
            let max_m: i32 = CountTrailingZeros(N as usize) as i32;
            let N_bits: i32 = BitLength(N as usize) as i32;
            let mut m: i32 = N_bits - 4; // Don't let s get too small.
            m = min(max_m, m);
            let mut N_mut = N * kDigitBits as i32;
            let n: i32 = 1 << m; // 2^m
                                 // We can't round up s in the inner layer, because N = n*s is fixed.
            let s: i32 = N_mut >> m;
            //DCHECK(N == s * n);
            if N != s * n {
                println!("DCHECK failed: N = {}, s = {}, n = {}", N, s, n);
                panic!("DCHECK failed");
            }
            let mut K: i32 = m + 2 * s + 1; // K must be at least this big...
            K = RoundUp(K as usize, n as usize) as i32; // ...and a multiple of n and kDigitBits.
            K = RoundUp(K as usize, kDigitBits) as i32;
            params.r = K >> m; // Which multiple?
                               //DCHECK(K % kDigitBits == 0);
            if K % kDigitBits as i32 != 0 {
                println!("DCHECK failed: K = {}, kDigitBits = {}", K, kDigitBits);
                panic!("DCHECK failed");
            }
            //DCHECK(s % kDigitBits == 0);
            if s % kDigitBits as i32 != 0 {
                println!("DCHECK failed: s = {}, kDigitBits = {}", s, kDigitBits);
                panic!("DCHECK failed");
            }
            params.K = K / kDigitBits as i32;
            params.s = s / kDigitBits as i32;
            params.n = n;
            params.m = m;
        }

        fn PredictInnerK(N: i32) -> i32 {
            let mut params: Parameters = Parameters::new();
            ComputeParameters_Inner(N, &mut params);
            params.K
        }

        // Applies heuristics to decide whether {m} should be decremented, by looking
        // at what would happen to {K} and {s} if {m} was decremented.
        fn ShouldDecrementM(current: &Parameters, next: &Parameters, after_next: &Parameters) -> bool {
            // K == 64 seems to work particularly well.
            if current.K == 64 && next.K >= 112 {
                return false;
            }
            // Small values for s are never efficient.
            if current.s < 6 {
                return true;
            }
            // The time is roughly determined by K * n. When we decrement m, then
            // n always halves, and K usually gets bigger, by up to 2x.
            // For not-quite-so-small s, look at how much bigger K would get: if
            // the K increase is small enough, making n smaller is worth it.
            // Empirically, it's most meaningful to look at the K *after* next.
            // The specific threshold values have been chosen by running many
            // benchmarks on inputs of many sizes, and manually selecting thresholds
            // that seemed to produce good results.
            let factor: f64 = after_next.K as f64 / current.K as f64;
            if (current.s == 6 && factor < 3.85) || // --
               (current.s == 7 && factor < 3.73) || // --
               (current.s == 8 && factor < 3.55) || // --
               (current.s == 9 && factor < 3.50) || // --
               factor < 3.4
            {
                return true;
            }
            // If K is just below the recursion threshold, make sure we do recurse,
            // unless doing so would be particularly inefficient (large inner_K).
            // If K is just above the recursion threshold, doubling it often makes
            // the inner call more efficient.
            if current.K >= 160 && current.K < 250 && PredictInnerK(next.K) < 28 {
                return true;
            }
            // If we found no reason to decrement, keep m as large as possible.
            return false;
        }

        // Decides what parameters to use for a given input bit length {N}.
        // Returns the chosen m.
        fn GetParameters(N: i32, params: &mut Parameters) -> i32 {
            let N_bits: i32 = BitLength(N as usize) as i32;
            let mut max_m: i32 = N_bits - 3; // Larger m make s too small.
            max_m = max(kLog2DigitBits as i32, max_m); // Smaller m break the logic below.
            let mut m: i32 = max_m;
            let mut current: Parameters = Parameters::new();
            ComputeParameters(N, m, &mut current);
            let mut next: Parameters = Parameters::new();
            ComputeParameters(N, m - 1, &mut next);
            while m > 2 {
                let mut after_next: Parameters = Parameters::new();
                ComputeParameters(N, m - 2, &mut after_next);
                if ShouldDecrementM(&current, &next, &after_next) {
                    m -= 1;
                    current = next;
                    next = after_next;
                } else {
                    break;
                }
            }
            *params = current;
            return m;
        }

        ////////////////////////////////////////////////////////////////////////////////
        // Part 3
