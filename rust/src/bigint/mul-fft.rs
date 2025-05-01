// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// FFT-based multiplication, due to Schönhage and Strassen.
// This implementation mostly follows the description given in:
// Christoph Lüders: Fast Multiplication of Large Integers,
// http://arxiv.org/abs/1503.04955

mod bigint_internal;
mod digit_arithmetic;
mod util;

use crate::bigint::bigint_internal::{Digits, RWDigits};
use crate::bigint::digit_arithmetic::{digit_add2, digit_add3, digit_sub, digit_sub2};
use crate::bigint::util::{BitLength, CountTrailingZeros, RoundUp};
use std::cmp;
use std::mem;

pub const kDigitBits: usize = mem::size_of::<digit_t>() * 8;
pub const kLog2DigitBits: usize = match kDigitBits {
    32 => 5,
    64 => 6,
    _ => panic!("Unsupported digit size"),
};
pub const kFftInnerThreshold: usize = 16; // Chosen empirically.

pub type digit_t = u32;
pub type signed_digit_t = i32;

mod bigint {
    use super::*;

    mod mul_fft {
        use super::*;

        mod internal {
            use super::*;

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
                let k = len - 1;
                let mut high: signed_digit_t = x[k] as signed_digit_t;
                if high == 0 {
                    return;
                }
                ModFn_Helper(x, len, high);
                high = x[k] as signed_digit_t;
                if high == 0 {
                    return;
                }
                debug_assert!(high == 1 || high == -1);
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
                let k = len - 1;
                let mut borrow: digit_t = 0;
                for i in 0..k {
                    dest[i] = digit_sub2(src[i], src[i + k], borrow, &mut borrow);
                }
                dest[k] = digit_sub2(0, src[2 * k], borrow, &mut borrow);
                // {borrow} may be non-zero here, that's OK as {ModFn} will take care of it.
                ModFn(dest, len);
            }

            // Sets {sum} := {a} + {b} and {diff} := {a} - {b}, which is more efficient
            // than computing sum and difference separately. Applies "mod F_n" normalization
            // to both results.
            fn SumDiff(
                sum: &mut [digit_t],
                diff: &mut [digit_t],
                a: &[digit_t],
                b: &[digit_t],
                len: usize,
            ) {
                let mut carry: digit_t = 0;
                let mut borrow: digit_t = 0;
                for i in 0..len {
                    // Read both values first, because inputs and outputs can overlap.
                    let ai: digit_t = a[i];
                    let bi: digit_t = b[i];
                    sum[i] = digit_add3(ai, bi, carry, &mut carry);
                    diff[i] = digit_sub2(ai, bi, borrow, &mut borrow);
                }
                ModFn(sum, len);
                ModFn(diff, len);
            }

            // {result} := ({input} << shift) mod F_n, where shift >= K.
            fn ShiftModFn_Large(
                result: &mut [digit_t],
                input: &[digit_t],
                digit_shift: usize,
                bits_shift: usize,
                k: usize,
            ) {
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
                debug_assert!(digit_shift >= k);
                let mut digit_shift = digit_shift - k;
                let mut borrow: digit_t = 0;
                if bits_shift == 0 {
                    let mut carry: digit_t = 1;
                    for i in 0..digit_shift {
                        result[i] = digit_add2(input[i + k - digit_shift], carry, &mut carry);
                    }
                    result[digit_shift] = digit_sub(
                        input[k].wrapping_add(carry),
                        input[0],
                        &mut borrow,
                    );
                    for i in digit_shift + 1..k {
                        let d: digit_t = input[i - digit_shift];
                        result[i] = digit_sub2(0, d, borrow, &mut borrow);
                    }
                } else {
                    let mut add_carry: digit_t = 1;
                    let mut input_carry: digit_t =
                        input[k - digit_shift - 1] >> (kDigitBits - bits_shift);
                    for i in 0..digit_shift {
                        let d: digit_t = input[i + k - digit_shift];
                        let summand: digit_t = (d << bits_shift) | input_carry;
                        result[i] = digit_add2(summand, add_carry, &mut add_carry);
                        input_carry = d >> (kDigitBits - bits_shift);
                    }
                    {
                        // result[digit_shift] = (add_carry + iK_part) - i0_part
                        let d: digit_t = input[k];
                        let iK_part: digit_t = (d << bits_shift) | input_carry;
                        let mut iK_carry: digit_t = d >> (kDigitBits - bits_shift);
                        let mut sum: digit_t = digit_add2(add_carry, iK_part, &mut add_carry);
                        // {iK_carry} is less than a full digit, so we can merge {add_carry}
                        // into it without overflow.
                        iK_carry = iK_carry.wrapping_add(add_carry);
                        let d: digit_t = input[0];
                        let i0_part: digit_t = d << bits_shift;
                        result[digit_shift] = digit_sub(sum, i0_part, &mut borrow);
                        input_carry = d >> (kDigitBits - bits_shift);
                        if digit_shift + 1 < k {
                            let d: digit_t = input[1];
                            let subtrahend: digit_t = (d << bits_shift) | input_carry;
                            result[digit_shift + 1] =
                                digit_sub2(iK_carry, subtrahend, borrow, &mut borrow);
                            input_carry = d >> (kDigitBits - bits_shift);
                        }
                    }
                    for i in digit_shift + 2..k {
                        let d: digit_t = input[i - digit_shift];
                        let subtrahend: digit_t = (d << bits_shift) | input_carry;
                        result[i] = digit_sub2(0, subtrahend, borrow, &mut borrow);
                        input_carry = d >> (kDigitBits - bits_shift);
                    }
                }
                // The virtual 1 in result[K] should be eliminated by {borrow}. If there
                // is no borrow, then the virtual initialization was too much. Subtract
                // 2^K + 1.
                result[k] = 0;
                if borrow != 1 {
                    borrow = 1;
                    for i in 0..k {
                        result[i] = digit_sub(result[i], borrow, &mut borrow);
                        if borrow == 0 {
                            break;
                        }
                    }
                    if borrow != 0 {
                        // The result must be 2^K.
                        for i in 0..k {
                            result[i] = 0;
                        }
                        result[k] = 1;
                    }
                }
            }

            // Sets {result} := {input} * 2^{power_of_two} mod 2^{K} + 1.
            // This function is highly relevant for overall performance.
            fn ShiftModFn(
                result: &mut [digit_t],
                input: &[digit_t],
                power_of_two: usize,
                k: usize,
                zero_above: usize,
            ) {
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
                while digit_shift >= 2 * k {
                    digit_shift -= 2 * k; // Faster than '%'!
                }
                if digit_shift >= k {
                    return ShiftModFn_Large(result, input, digit_shift, bits_shift, k);
                }
                let mut borrow: digit_t = 0;
                if bits_shift == 0 {
                    // We do a single pass over {input}, starting by copying digits [i1] to
                    // [iX-1] to result indices digit_shift+1 to K-1.
                    let mut i: usize = 1;
                    // Read input digits unless we know they are zero.
                    let cap: usize = cmp::min(k - digit_shift, zero_above);
                    while i < cap {
                        result[i + digit_shift] = input[i];
                        i += 1;
                    }
                    // Any remaining work can hard-code the knowledge that input[i] == 0.
                    while i < k - digit_shift {
                        debug_assert!(input[i] == 0);
                        result[i + digit_shift] = 0;
                        i += 1;
                    }
                    // Second phase: subtract input digits [iX] to [iK] from (virtually) zero-
                    // initialized result indices 0 to digit_shift-1.
                    let cap: usize = cmp::min(k, zero_above);
                    while i < cap {
                        let d: digit_t = input[i];
                        result[i - k + digit_shift] = digit_sub2(0, d, borrow, &mut borrow);
                        i += 1;
                    }
                    // Any remaining work can hard-code the knowledge that input[i] == 0.
                    while i < k {
                        debug_assert!(input[i] == 0);
                        result[i - k + digit_shift] = digit_sub(0, borrow, &mut borrow);
                        i += 1;
                    }
                    // Last step: subtract [iK] from [i0] and store at result index digit_shift.
                    result[digit_shift] = digit_sub2(input[0], input[k], borrow, &mut borrow);
                } else {
                    // Same flow as before, but taking bits_shift != 0 into account.
                    // First phase: result indices digit_shift+1 to K.
                    let mut carry: digit_t = 0;
                    let mut i: usize = 0;
                    // Read input digits unless we know they are zero.
                    let cap: usize = cmp::min(k - digit_shift, zero_above);
                    while i < cap {
                        let d: digit_t = input[i];
                        result[i + digit_shift] = (d << bits_shift) | carry;
                        carry = d >> (kDigitBits - bits_shift);
                        i += 1;
                    }
                    // Any remaining work can hard-code the knowledge that input[i] == 0.
                    while i < k - digit_shift {
                        debug_assert!(input[i] == 0);
                        result[i + digit_shift] = carry;
                        carry = 0;
                        i += 1;
                    }
                    // Second phase: result indices 0 to digit_shift - 1.
                    let cap: usize = cmp::min(k, zero_above);
                    while i < cap {
                        let d: digit_t = input[i];
                        result[i - k + digit_shift] =
                            digit_sub2(0, (d << bits_shift) | carry, borrow, &mut borrow);
                        carry = d >> (kDigitBits - bits_shift);
                        i += 1;
                    }
                    // Any remaining work can hard-code the knowledge that input[i] == 0.
                    if i < k {
                        debug_assert!(input[i] == 0);
                        result[i - k + digit_shift] = digit_sub2(0, carry, borrow, &mut borrow);
                        carry = 0;
                        i += 1;
                    }
                    while i < k {
                        debug_assert!(input[i] == 0);
                        result[i - k + digit_shift] = digit_sub(0, borrow, &mut borrow);
                        i += 1;
                    }
                    // Last step: compute result[digit_shift].
                    let d: digit_t = input[k];
                    result[digit_shift] = digit_sub2(
                        result[digit_shift],
                        (d << bits_shift) | carry,
                        borrow,
                        &mut borrow,
                    );
                    // No carry left.
                    debug_assert!((d >> (kDigitBits - bits_shift)) == 0);
                }
                result[k] = 0;
                for i in digit_shift + 1..=k {
                    if borrow > 0 {
                        result[i] = digit_sub(result[i], borrow, &mut borrow);
                    } else {
                        break;
                    }
                }
                if borrow > 0 {
                    // Underflow means we subtracted too much. Add 2^K + 1.
                    let mut carry: digit_t = 1;
                    for i in 0..=k {
                        result[i] = digit_add2(result[i], carry, &mut carry);
                        if carry == 0 {
                            break;
                        }
                    }
                    result[k] = digit_add2(result[k], 1, &mut carry);
                }
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
            k: i32,
            n: i32,
            s: i32,
            r: i32,
        }

        impl Parameters {
            fn new() -> Self {
                Parameters {
                    m: 0,
                    k: 0,
                    n: 0,
                    s: 0,
                    r: 0,
                }
            }
        }

        // Computes parameters for the main calculation, given a bit length {N} and
        // an {m}. See the paper for details.
        fn ComputeParameters(N: i32, m: i32, params: &mut Parameters) {
            let mut N: i32 = N * (kDigitBits as i32);
            let n: i32 = 1 << m; // 2^m
            let nhalf: i32 = n >> 1;
            let mut s: i32 = (N + n - 1) >> m; // ceil(N/n)
            s = RoundUp(s as usize, kDigitBits) as i32;
            let mut k: i32 = m + 2 * s + 1; // K must be at least this big...
            k = RoundUp(k as usize, nhalf as usize) as i32; // ...and a multiple of n/2.
            let mut r: i32 = k >> (m - 1); // Which multiple?

            // We want recursive calls to make progress, so force K to be a multiple
            // of 8 if it's above the recursion threshold. Otherwise, K must be a
            // multiple of kDigitBits.
            let threshold: i32 = if k + 1 >= (kFftInnerThreshold as i32) * (kDigitBits as i32) {
                3 + (kLog2DigitBits as i32)
            } else {
                kLog2DigitBits as i32
            };
            let mut k_tz: i32 = CountTrailingZeros(k as usize) as i32;
            while k_tz < threshold {
                k += 1 << k_tz;
                r = k >> (m - 1);
                k_tz = CountTrailingZeros(k as usize) as i32;
            }

            debug_assert!(k % (kDigitBits as i32) == 0);
            debug_assert!(s % (kDigitBits as i32) == 0);
            params.k = k / (kDigitBits as i32);
            params.s = s / (kDigitBits as i32);
            params.n = n;
            params.r = r;
        }

        // Computes parameters for recursive invocations ("inner layer").
        fn ComputeParameters_Inner(N: i32, params: &mut Parameters) {
            let max_m: i32 = CountTrailingZeros(N as usize) as i32;
            let N_bits: i32 = BitLength(N as usize) as i32;
            let mut m: i32 = N_bits - 4; // Don't let s get too small.
            m = cmp::min(max_m, m);
            let mut N: i32 = N * (kDigitBits as i32);
            let n: i32 = 1 << m; // 2^m
            // We can't round up s in the inner layer, because N = n*s is fixed.
            let s: i32 = N >> m;
            debug_assert!(N == s * n);
            let mut k: i32 = m + 2 * s + 1; // K must be at least this big...
            k = RoundUp(k as usize, n as usize) as i32; // ...and a multiple of n and kDigitBits.
            k = RoundUp(k as usize, kDigitBits) as i32;
            params.r = k >> m; // Which multiple?
            debug_assert!(k % (kDigitBits as i32) == 0);
            debug_assert!(s % (kDigitBits as i32) == 0);
            params.k = k / (kDigitBits as i32);
            params.s = s / (kDigitBits as i32);
            params.n = n;
            params.m = m;
        }

        fn PredictInnerK(N: i32) -> i32 {
            let mut params = Parameters::new();
            ComputeParameters_Inner(N, &mut params);
            params.k
        }

        // Applies heuristics to decide whether {m} should be decremented, by looking
        // at what would happen to {K} and {s} if {m} was decremented.
        fn ShouldDecrementM(
            current: &Parameters,
            next: &Parameters,
            after_next: &Parameters,
        ) -> bool {
            // K == 64 seems to work particularly well.
            if current.k == 64 && next.k >= 112 {
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
            let factor: f64 = (after_next.k as f64) / (current.k as f64);
            if (current.s == 6 && factor < 3.85)
                || // --
                (current.s == 7 && factor < 3.73)
                || // --
                (current.s == 8 && factor < 3.55)
                || // --
                (current.s == 9 && factor < 3.50)
                || // --
                factor < 3.4
            {
                return true;
            }
            // If K is just below the recursion threshold, make sure we do recurse,
            // unless doing so would be particularly inefficient (large inner_K).
            // If K is just above the recursion threshold, doubling it often makes
            // the inner call more efficient.
            if current.k >= 160 && current.k < 250 && PredictInnerK(next.k) < 28 {
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
            max_m = cmp::max(kLog2DigitBits as i32, max_m); // Smaller m break the logic below.
            let mut m: i32 = max_m;
            let mut current = Parameters::new();
            ComputeParameters(N, m, &mut current);
            let mut next = Parameters::new();
            ComputeParameters(N, m - 1, &mut next);
            while m > 2 {
                let mut after_next = Parameters::new();
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
        // Part 3: Fast Fourier Transformation.

        pub struct FFTContainer<'a> {
            n_: usize,       // Number of parts.
            k_: usize,       // Always length_ - 1.
            length_: usize,  // Length of each part, in digits.
            processor_: &'a mut ProcessorImpl,
            storage_: Box<[digit_t]>, // Combined storage of all parts.
            part_: Vec<&'a mut [digit_t]>,    // Pointers to each part.
            temp_: Box<[digit_t]>,     // Temporary storage with size 2 * length_.
        }

        impl<'a> FFTContainer<'a> {
            // {n} is the number of chunks, whose length is {K}+1.
            // {K} determines F_n = 2^(K * kDigitBits) + 1.
            pub fn new(n: usize, k: usize, processor: &'a mut ProcessorImpl) -> Self {
                let length_ = k + 1;
                let storage_ = vec![0; length_ * n].into_boxed_slice();
                let mut part_: Vec<&mut [digit_t]> = Vec::with_capacity(n);
                let mut ptr = storage_.as_ptr() as *mut digit_t;
                unsafe {
                    for _ in 0..n {
                        part_.push(std::slice::from_raw_parts_mut(ptr, length_));
                        ptr = ptr.add(length_);
                    }
                }
                let temp_ = vec![0; length_ * 2].into_boxed_slice();
                FFTContainer {
                    n_: n,
                    k_: k,
                    length_: length_,
                    processor_: processor,
                    storage_: storage_,
                    part_: part_,
                    temp_: temp_,
                }
            }

            pub fn start_default(&mut self, X: Digits, chunk_size: i32, theta: i32, omega: i32) {
                self.start(X, chunk_size, theta, omega, true);
            }

            pub fn start(&mut self, X: Digits, chunk_size: i32, theta: i32, omega: i32) {
                self.start(X, chunk_size, theta, omega, false);
            }

            fn start(
                &mut self,
                X: Digits,
                mut chunk_size: i32,
                theta: i32,
                omega: i32,
                is_default: bool,
            ) {
                let mut len: i32 = X.len() as i32;
                let pointer: *const digit_t = X.digits();
                let part_length_in_bytes: usize = self.length_ * std::mem::size_of::<digit_t>();

                if !is_default && len > (self.n_ as i32) * chunk_size / 2 {
                    return self.start_default(X, chunk_size, theta, omega);
                }

                let mut current_theta = 0;
                let mut i = 0;
                while i < self.n_ && len > 0 {
                    chunk_size = cmp::min(chunk_size, len);
                    if i == self.n_ - 1 && len == chunk_size + 1 {
                        debug_assert!(X[self.n_ * chunk_size as usize] <= 1);
                        debug_assert!(self.length_ >= chunk_size as usize + 1);
                        chunk_size += 1;
                    }
                    let current_part = &mut self.part_[i];
                    if current_theta != 0 {
                        let mut temp_slice: &mut [digit_t] =
                            unsafe { self.temp_.as_mut_ptr().as_mut().unwrap() };
                        CopyAndZeroExtend(
                            temp_slice,
                            unsafe { std::slice::from_raw_parts(pointer, chunk_size as usize) },
                            chunk_size as usize,
                            part_length_in_bytes,
                        );
                        let temp_digits = Digits::new(temp_slice);
                        internal::ShiftModFn(
                            current_part,
                            temp_digits.digits(),
                            current_theta as usize,
                            self.k_,
                            chunk_size as usize,
                        );
                    } else {
                        CopyAndZeroExtend(
                            current_part,
                            unsafe { std::slice::from_raw_parts(pointer, chunk_size as usize) },
                            chunk_size as usize,
                            part_length_in_bytes,
                        );
                    }
                    unsafe {
                        len -= chunk_size;
                    }
                    i += 1;
                    unsafe {
                        current_theta += theta;
                    }
                }
                debug_assert!(len == 0);
                while i < self.n_ {
                    self.part_[i].fill(0);
                    i += 1;
                }
                self.fft_return_shuffled_threadsafe(0, self.n_, omega, &mut self.temp_);
            }

            pub fn normalize_and_recombine(&mut self, omega: i32, m: i32, Z: &mut RWDigits, chunk_size: i32) {
                Z.Clear();
                let mut z_index = 0;
                let shift = (self.n_ as i32) * omega - m;
                for i in 0..self.n_ {
                    let part = self.part_[i];
                    unsafe {
                      let mut temp_slice: &mut [digit_t] =
                            self.temp_.as_mut_ptr().as_mut().unwrap();
                       internal::ShiftModFn(temp_slice, part, shift as usize, self.k_, part.len());
                    }
                    let mut carry = 0;
                    let mut zi = z_index;
                    let mut j = 0;
                    while j < self.length_ && zi < Z.len() {
                        Z[zi] = digit_add3(Z[zi], self.temp_[j], carry, &mut carry);
                        zi += 1;
                        j += 1;
                    }
                    while j < self.length_ {
                        debug_assert!(self.temp_[j] == 0);
                        j += 1;
                    }
                    if carry != 0 {
                        debug_assert!(zi < Z.len());
                        Z[zi] = carry;
                    }
                    z_index += chunk_size as usize;
                }
            }

            pub fn counterweight_and_recombine(&mut self, theta: i32, m: i32, Z: &mut RWDigits, s: i32) {
                Z.Clear();
                let mut z_index = 0;
                for k in 0..self.n_ {
                    let mut shift = -(theta * (k as i32)) - m;
                    if shift < 0 {
                        shift += 2 * (self.n_ as i32) * theta;
                    }
                    debug_assert!(shift >= 0);
                    let input = self.part_[k];

                    unsafe {
                      let mut temp_slice: &mut [digit_t] =
                            self.temp_.as_mut_ptr().as_mut