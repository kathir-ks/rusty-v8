// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//use std::mem;
//use std::sync::Arc;

mod bigint_internal;
mod digit_arithmetic;
mod div_helpers;
mod util;
mod vector_arithmetic;

use crate::bigint_internal::*;
use crate::digit_arithmetic::*;
use crate::div_helpers::*;
use crate::util::*;
use crate::vector_arithmetic::*;

//use std::convert::TryInto;
//use std::ptr;

const K_MAX_BITS_PER_CHAR: [u8; 37] = [
    0, 0, 32, 51, 64, 75, 83, 90, 96, // 0..8
    102, 107, 111, 115, 119, 122, 126, 128, // 9..16
    131, 134, 136, 139, 141, 143, 145, 147, // 17..24
    149, 151, 153, 154, 156, 158, 159, 160, // 25..32
    162, 163, 165, 166, // 33..36
];

const K_BITS_PER_CHAR_TABLE_SHIFT: usize = 5;
const K_BITS_PER_CHAR_TABLE_MULTIPLIER: usize = 1 << K_BITS_PER_CHAR_TABLE_SHIFT;

const K_CONVERSION_CHARS: &str = "0123456789abcdefghijklmnopqrstuvwxyz";

/// Raises `base` to the power of `exponent`. Does not check for overflow.
fn digit_pow(base: Digit, exponent: Digit) -> Digit {
    let mut result: Digit = 1;
    let mut b = base;
    let mut exp = exponent;
    while exp > 0 {
        if exp & 1 != 0 {
            result *= b;
        }
        exp >>= 1;
        b *= b;
    }
    result
}

/// Compile-time version of the above.
const fn digit_pow_rec(base: Digit, exponent: Digit) -> Digit {
    if exponent == 1 {
        base
    } else {
        base * digit_pow_rec(base, exponent - 1)
    }
}

/// A variant of ToStringFormatter::BasecaseLast, specialized for a radix
/// known at compile-time.
fn basecase_fixed_last<const RADIX: usize>(chunk: Digit, mut out: &mut [u8]) -> &mut [u8] {
    let conversion_chars = K_CONVERSION_CHARS.as_bytes();
    let mut c = chunk;
    while c != 0 {
        //assert_eq!(out[out.len() - 1], k_string_zap_value);
        if RADIX <= 10 {
            out[out.len() - 1] = b'0' + (c % RADIX as Digit) as u8;
        } else {
            out[out.len() - 1] = conversion_chars[(c % RADIX as Digit) as usize];
        }
        c /= RADIX as Digit;
        out = &mut out[..out.len() - 1];
    }
    out
}

/// By making {radix} a compile-time constant and computing {chunk_divisor}
/// as another compile-time constant from it, we allow the compiler to emit
/// an optimized instruction sequence based on multiplications with "magic"
/// numbers (modular multiplicative inverses) instead of actual divisions.
/// The price we pay is having to work on half digits; the technique doesn't
/// work with twodigit_t-by-digit_t divisions.
/// Includes an equivalent of ToStringFormatter::BasecaseMiddle, accordingly
/// specialized for a radix known at compile time.
fn divide_by_magic<const RADIX: Digit>(
    mut rest: RWDigits,
    input: Digits,
    mut output: &mut [u8],
) -> &mut [u8] {
    const MAX_BITS_PER_CHAR: u8 = K_MAX_BITS_PER_CHAR[RADIX as usize];
    const CHUNK_CHARS: usize =
        (K_HALF_DIGIT_BITS * K_BITS_PER_CHAR_TABLE_MULTIPLIER) / MAX_BITS_PER_CHAR as usize;
    const CHUNK_DIVISOR: Digit = digit_pow_rec(RADIX, CHUNK_CHARS as Digit);
    let conversion_chars = K_CONVERSION_CHARS.as_bytes();

    let mut remainder: Digit = 0;
    for i in (0..input.len()).rev() {
        let d = input[i];
        let upper: Digit = (remainder << K_HALF_DIGIT_BITS) | (d >> K_HALF_DIGIT_BITS);
        let u_result: Digit = upper / CHUNK_DIVISOR;
        remainder = upper % CHUNK_DIVISOR;
        let lower: Digit = (remainder << K_HALF_DIGIT_BITS) | (d & K_HALF_DIGIT_MASK);
        let l_result: Digit = lower / CHUNK_DIVISOR;
        remainder = lower % CHUNK_DIVISOR;
        rest[i] = (u_result << K_HALF_DIGIT_BITS) | l_result;
    }
    // {remainder} is now the current chunk to be written out.
    for _i in 0..CHUNK_CHARS {
        //assert_eq!(output[output.len() - 1], k_string_zap_value);
        if RADIX <= 10 {
            output[output.len() - 1] = b'0' + (remainder % RADIX) as u8;
        } else {
            output[output.len() - 1] = conversion_chars[(remainder % RADIX) as usize];
        }
        remainder /= RADIX;
        output = &mut output[..output.len() - 1];
    }
    assert_eq!(remainder, 0);
    output
}

// The classic algorithm must check for interrupt requests if no faster
// algorithm is available.
//
// C++ macro translation:
//
// #if V8_ADVANCED_BIGINT_ALGORITHMS
// #define MAYBE_INTERRUPT(code) ((void)0)
// #else
// #define MAYBE_INTERRUPT(code) code
// #endif
//
// Rust equivalent: Conditional compilation with cfg,
// and inline functions for readability.
//
// NOTE: V8_ADVANCED_BIGINT_ALGORITHMS is always enabled,
// so MAYBE_INTERRUPT will be no-op.
#[inline]
fn maybe_interrupt<F: FnOnce()>(code: F) {
    code();
}

struct RecursionLevel {
    leading_zero_shift_: usize,
    char_count_: usize,
    is_toplevel_: bool,
    next_: Option<Box<RecursionLevel>>,
    divisor_: ScratchDigits,
    inverse_storage_: Option<Box<Storage>>,
    inverse_: Digits,
}

impl RecursionLevel {
    fn new(base_divisor: Digit, base_char_count: usize) -> Self {
        let mut divisor_ = ScratchDigits::new(1);
        divisor_[0] = base_divisor;
        RecursionLevel {
            leading_zero_shift_: 0,
            char_count_: base_char_count,
            is_toplevel_: true,
            next_: None,
            divisor_,
            inverse_storage_: None,
            inverse_: Digits::empty(),
        }
    }

    fn new_from_next(next: Box<RecursionLevel>) -> Self {
        let char_count_ = next.char_count_ * 2;
        let divisor_ = ScratchDigits::new(next.divisor_.len() * 2);
        RecursionLevel {
            leading_zero_shift_: 0,
            char_count_,
            is_toplevel_: false,
            next_: Some(next),
            divisor_,
            inverse_storage_: None,
            inverse_: Digits::empty(),
        }
    }

    fn create_levels(
        base_divisor: Digit,
        base_char_count: usize,
        target_bit_length: usize,
        processor: &mut ProcessorImpl,
    ) -> Option<Box<RecursionLevel>> {
        let mut level = Box::new(RecursionLevel::new(base_divisor, base_char_count));
        // We can stop creating levels when the next level's divisor, which is the
        // square of the current level's divisor, would be strictly bigger (in terms
        // of its numeric value) than the input we're formatting. Since computing that
        // next divisor is expensive, we want to predict the necessity based on bit
        // lengths. Bit lengths are an imperfect predictor of numeric value, so we
        // have to be careful:
        // - since we can't estimate which one of two numbers of equal bit length
        //   is bigger, we have to aim for a strictly bigger bit length.
        // - when squaring, the bit length sometimes doubles (e.g. 0b11² == 0b1001),
        //   but usually we "lose" a bit (e.g. 0b10² == 0b100).
        while BitLength(level.divisor_.as_digits()) * 2 - 1 <= target_bit_length {
            let prev = level;
            level = Box::new(RecursionLevel::new_from_next(prev));

            processor.multiply(
                level.divisor_.as_rw_digits(),
                level.next_.as_ref().unwrap().divisor_.as_digits(),
                level.next_.as_ref().unwrap().divisor_.as_digits(),
            );
            if processor.should_terminate() {
                return None;
            }
            level.divisor_.normalize();
            // Left-shifting the divisor must only happen after it's been used to
            // compute the next divisor.
            level.next_mut().unwrap().left_shift_divisor();
            level.next_mut().unwrap().compute_inverse(processor, 0);
        }
        level.left_shift_divisor();
        // Not calling info->ComputeInverse here so that it can take the input's
        // length into account to save some effort on inverse generation.
        Some(level)
    }

    // The top level might get by with a smaller inverse than we could maximally
    // compute, so the caller should provide the dividend length.
    fn compute_inverse(&mut self, processor: &mut ProcessorImpl, dividend_length: usize) {
        let mut inverse_len = self.divisor_.len();
        if dividend_length != 0 {
            inverse_len = dividend_length - self.divisor_.len();
            assert!(inverse_len <= self.divisor_.len());
        }
        let scratch_len = InvertScratchSpace(inverse_len);
        let mut scratch = ScratchDigits::new(scratch_len);
        let inv_storage = Box::new(Storage::new(inverse_len + 1));
        self.inverse_storage_ = Some(inv_storage);

        let inverse_initializer =
            RWDigits::new(self.inverse_storage_.as_mut().unwrap().get(), inverse_len + 1);
        let input = Digits::new(
            self.divisor_.as_digits().digits().as_ptr(),
            self.divisor_.len() - inverse_len,
            inverse_len,
        );
        processor.invert(inverse_initializer, input, &mut scratch);
        inverse_initializer.trim_one();
        self.inverse_ = inverse_initializer.as_digits();
    }

    fn get_inverse(&self, dividend_length: usize) -> Digits {
        assert!(self.inverse_.len() != 0);
        let inverse_len = dividend_length - self.divisor_.len();
        // If the bits in memory are reliable, then we always have enough digits
        // in the inverse available. This is a Release-mode CHECK because malicious
        // concurrent heap mutation can throw off the decisions made by the recursive
        // procedure, and this is a good bottleneck to catch them.
        assert!(inverse_len <= self.inverse_.len());
        self.inverse_
            .offset(self.inverse_.len() - inverse_len)
    }

    fn left_shift_divisor(&mut self) {
        self.leading_zero_shift_ = CountLeadingZeros(self.divisor_.msd());
        LeftShift(
            self.divisor_.as_rw_digits(),
            self.divisor_.as_digits(),
            self.leading_zero_shift_,
        );
    }

    fn next_mut(&mut self) -> Option<&mut RecursionLevel> {
        self.next_.as_mut().map(|boxed| &mut **boxed)
    }
}

struct ToStringFormatter<'a> {
    digits_: Digits,
    radix_: i32,
    max_bits_per_char_: usize,
    chunk_chars_: usize,
    sign_: bool,
    out_start_: *mut u8,
    out_end_: *mut u8,
    out_: *mut u8,
    chunk_divisor_: Digit,
    processor_: &'a mut ProcessorImpl,
}

impl<'a> ToStringFormatter<'a> {
    fn new(
        digits_: Digits,
        radix_: i32,
        sign_: bool,
        out: *mut u8,
        chars_available: u32,
        processor_: &'a mut ProcessorImpl,
    ) -> Self {
        let mut s = ToStringFormatter {
            digits_,
            radix_: radix_,
            max_bits_per_char_: 0,
            chunk_chars_: 0,
            sign_: sign_,
            out_start_: out,
            out_end_: unsafe { out.add(chars_available as usize) },
            out_: unsafe { out.add(chars_available as usize) },
            chunk_divisor_: 0,
            processor_: processor_,
        };
        s.digits_.normalize();
        assert!(chars_available >= ToStringResultLength(s.digits_, s.radix_, s.sign_));
        s
    }

    fn start(&mut self) {
        self.max_bits_per_char_ = K_MAX_BITS_PER_CHAR[self.radix_ as usize] as usize;
        self.chunk_chars_ =
            (K_DIGIT_BITS * K_BITS_PER_CHAR_TABLE_MULTIPLIER) / self.max_bits_per_char_;
        self.chunk_divisor_ = digit_pow(self.radix_ as Digit, self.chunk_chars_ as Digit);
        // By construction of chunk_chars_, there can't have been overflow.
        assert!(self.chunk_divisor_ != 0);
    }

    fn finish(&mut self) -> i32 {
        assert!(self.out_ as usize >= self.out_start_ as usize);
        assert!(self.out_ as usize < self.out_end_ as usize); // At least one character was written.

        // Convert raw pointers to slices to perform the calculations.
        let out_start_usize = self.out_start_ as usize;
        let out_end_usize = self.out_end_ as usize;
        let out_usize = self.out_ as usize;

        let out_start_ptr = self.out_start_;
        let out_end_ptr = self.out_end_;
        let out_ptr = self.out_;

        // Calculate the slice lengths based on the raw pointer values.
        let out_start_to_out_len = (out_usize - out_start_usize);
        let out_to_out_end_len = (out_end_usize - out_usize);
        let total_len = (out_end_usize - out_start_usize);

        let out_start_slice = unsafe { std::slice::from_raw_parts_mut(out_start_ptr, total_len) };
        let out_slice = unsafe { std::slice::from_raw_parts_mut(out_ptr, out_to_out_end_len) };

        // The original logic uses `while` loop, convert to loop here to achieve the same.
        let mut slice_start = 0;
        while slice_start < out_to_out_end_len && out_slice[slice_start] == b'0' {
            slice_start += 1;
        }

        let out_slice = &out_slice[slice_start..];

        //Adjust `out_usize`, `out_len`, and `out_start_usize`.
        let mut excess = 0;

        if self.sign_ {
            // Adjusted pointer arithmetic and slice length calculation.
            let new_out_ptr = unsafe { out_ptr.sub(1) };
            let new_out_usize = new_out_ptr as usize;
            let new_out_to_out_end_len = out_to_out_end_len + 1;
            let new_out_slice = unsafe { std::slice::from_raw_parts_mut(new_out_ptr, new_out_to_out_end_len) };

            // Write the '-' sign and update the slice.
            new_out_slice[0] = b'-';

            self.out_ = new_out_ptr;
        }

        if (out_usize as usize) > (out_start_usize as usize) {
            let actual_length = out_to_out_end_len;
            excess = (out_usize as usize) - (out_start_usize as usize);

            // Use memmove, because slices can overlap.
            unsafe {
                std::ptr::copy(out_ptr, out_start_ptr, actual_length);
            }
        }

        excess as i32
    }

    fn classic(&mut self) {
        if self.digits_.len() == 0 {
            unsafe {
                *self.out_.sub(1) = b'0';
                self.out_ = self.out_.sub(1);
            }
            return;
        }
        if self.digits_.len() == 1 {
            unsafe {
                let digits_val = self.digits_[0];
                let out_slice = std::slice::from_raw_parts_mut(self.out_ as *mut u8, 0);
                self.out_ = self.basecase_last(digits_val, out_slice).as_mut_ptr();
            }
            return;
        }
        // {rest} holds the part of the BigInt that we haven't looked at yet.
        // Not to be confused with "remainder"!
        let mut rest = ScratchDigits::new(self.digits_.len());
        // In the first round, divide the input, allocating a new BigInt for
        // the result == rest; from then on divide the rest in-place.
        let mut dividend = self.digits_;
        loop {
            if self.radix_ == 10 {
                // Faster but costs binary size, so we optimize the most common case.
                unsafe {
                    let out_slice = std::slice::from_raw_parts_mut(self.out_ as *mut u8, 0);
                    self.out_ = divide_by_magic::<10>(rest.as_rw_digits(), dividend, out_slice)
                        .as_mut_ptr();
                }

                maybe_interrupt(|| self.processor_.add_work_estimate(rest.len() * 2));
            } else {
                let mut chunk: Digit = 0;
                self.processor_.divide_single(
                    rest.as_rw_digits(),
                    &mut chunk,
                    dividend,
                    self.chunk_divisor_,
                );
                unsafe {
                    let out_slice = std::slice::from_raw_parts_mut(self.out_ as *mut u8, 0);
                    self.out_ = self.basecase_middle(chunk, out_slice).as_mut_ptr();
                }

                // Assume that a division is about ten times as expensive as a
                // multiplication.
                maybe_interrupt(|| self.processor_.add_work_estimate(rest.len() * 10));
            }

            if self.processor_.should_terminate() {
                return;
            }
            rest.normalize();
            dividend = rest.as_digits();
            if rest.len() <= 1 {
                break;
            }
        }
        unsafe {
            let out_slice = std::slice::from_raw_parts_mut(self.out_ as *mut u8, 0);
            self.out_ = self.basecase_last(rest[0], out_slice).as_mut_ptr();
        }
    }

    fn base_power_of_two(&mut self) {
        let bits_per_char = CountTrailingZeros(self.radix_ as Digit) as usize;
        let char_mask = self.radix_ - 1;
        let mut digit: Digit = 0;
        // Keeps track of how many unprocessed bits there are in {digit}.
        let mut available_bits = 0;
        let conversion_chars = K_CONVERSION_CHARS.as_bytes();
        for i in 0..self.digits_.len() - 1 {
            let new_digit = self.digits_[i];
            // Take any leftover bits from the last iteration into account.
            let mut current = (digit | (new_digit << available_bits)) & char_mask as Digit;

            unsafe {
                *self.out_.sub(1) = conversion_chars[current as usize];
                self.out_ = self.out_.sub(1);
            }

            let consumed_bits = bits_per_char - available_bits;
            digit = new_digit >> consumed_bits;
            available_bits = K_DIGIT_BITS - consumed_bits;
            while available_bits >= bits_per_char {
                unsafe {
                    *self.out_.sub(1) = conversion_chars[(digit & char_mask as Digit) as usize];
                    self.out_ = self.out_.sub(1);
                }
                digit >>= bits_per_char;
                available_bits -= bits_per_char;
            }
        }
        // Take any leftover bits from the last iteration into account.
        let msd = self.digits_.msd();
        let mut current = (digit | (msd << available_bits)) & char_mask as Digit;
        unsafe {
            *self.out_.sub(1) = conversion_chars[current as usize];
            self.out_ = self.out_.sub(1);
        }
        digit = msd >> (bits_per_char - available_bits);
        while digit != 0 {
            unsafe {
                *self.out_.sub(1) = conversion_chars[(digit & char_mask as Digit) as usize];
                self.out_ = self.out_.sub(1);
            }
            digit >>= bits_per_char;
        }
    }

    fn fast(&mut self) {
        // As a sandbox proofing measure, we round up here. Using {BitLength(digits_)}
        // would be technically optimal, but vulnerable to a malicious worker that
        // uses an in-sandbox corruption primitive to concurrently toggle the MSD bits
        // between the invocations of {CreateLevels} and {ProcessLevel}.
        let target_bit_length = self.digits_.len() * K_DIGIT_BITS;
        let recursion_levels = RecursionLevel::create_levels(
            self.chunk_divisor_,
            self.chunk_chars_,
            target_bit_length,
            self.processor_,
        );
        if self.processor_.should_terminate() {
            return;
        }

        //Convert unsafe raw pointer to slice
        let out_slice = unsafe { std::slice::from_raw_parts_mut(self.out_ as *mut u8, 0) };
        if let Some(level) = recursion_levels {
            unsafe {
                self.out_ =
                    self.process_level(&*level, self.digits_, out_slice, true)
                        .as_mut_ptr();
            }
        }
    }

    fn fill_with_zeros(
        &mut self,
        level: Option<&RecursionLevel>,
        right_boundary: *mut u8,
        mut out: *mut u8,
        is_last_on_level: bool,
    ) -> *mut u8 {
        // Fill up with zeros up to the character count expected to be generated
        // on this level; unless this is the left edge of the result.
        if is_last_on_level {
            return out;
        }

        let chunk_chars = match level {
            Some(l) => l.char_count_ * 2,
            None => self.chunk_chars_,
        };

        // Calculate end_ptr
        let end_ptr = unsafe { right_boundary.sub(chunk_chars) };

        // Adjust out_ptr until it reaches end_ptr or the start of slice
        while out as *mut u8 > end_ptr {
            unsafe {
                *out.sub(1) = b'0';
                out = out.sub(1);
            }
        }
        out
    }

    fn process_level(
        &mut self,
        level: &RecursionLevel,
        chunk: Digits,
        mut out: &mut [u8],
        is_last_on_level: bool,
    ) -> &mut [u8] {
        // Step 0: if only one digit is left, bail out to the base case.
        let mut normalized = chunk;
        normalized.normalize();

        if normalized.len() <= 1 {
            let right_boundary = out;
            if normalized.len() == 1 {
                out = self.basecase_last(normalized[0], out);
            }

            let ptr = out;

            // Get out_ptr from the reference
            out = &mut [0];

            let filled_ptr = self.fill_with_zeros(
                None,
                right_boundary.as_mut_ptr(),
                ptr.as_mut_ptr(),
                is_last_on_level,
            )
                .as_mut_ptr();

            unsafe {
                let dist = ptr.as_mut_ptr() as usize - filled_ptr as usize;

                //Update the out-pointer by given distance.
                let new_out_ptr = ptr.as_mut_ptr().sub(dist);
                out = &mut *(new_out_ptr as *mut [u8]);
            }

            return out;
        }

        // Step 1: If the chunk is guaranteed to remain smaller than the divisor
        // even after left-shifting, fall through to the next level immediately.
        if normalized.len() < level.divisor_.len() {
            let right_boundary = out;
            if let Some(next_level) = level.next_.as_ref() {
                out = self.process_level(next_level, chunk, out, is_last_on_level);
            }
            let ptr = out;

            // Get out_ptr from the reference
            out = &mut [0];

            let filled_ptr = self.fill_with_zeros(
                None,
                right_boundary.as_mut_ptr(),
                ptr.as_mut_ptr(),
                is_last_on_level,
            )
                .as_mut_ptr();

            unsafe {
                let dist = ptr.as_mut_ptr() as usize - filled_ptr as usize;

                //Update the out-pointer by given distance.
                let new_out_ptr = ptr.as_mut_ptr().sub(dist);
                out = &mut *(new_out_ptr as *mut [u8]);
            }

            return out;
        }
        // Step 2: Prepare the chunk.
        let allow_inplace_modification = chunk.digits() != self.digits_.digits();
        let original_chunk = chunk;
        let chunk_shifted = ShiftedDigits::new(
            chunk,
            level.leading_zero_shift_,
            allow_inplace_modification,
        );
        let mut chunk = chunk_shifted.as_digits();
        chunk.normalize();
        // Check (now precisely) if the chunk is smaller than the divisor.
        let comparison = Compare(chunk, level.divisor_.as_digits());
        if comparison <= 0 {
            let right_boundary = out;
            if comparison < 0 {
                // If the chunk is strictly smaller than the divisor, we can process
                // it directly on the next level as the right half, and know that the
                // left half is all '0'.
                // In case we shifted {chunk} in-place, we must undo that
                // before the call...
                // chunk_shifted.Reset();  //TODO: Implement reset if needed
                // ...and otherwise undo the {chunk = chunk_shifted} assignment above.
                let out_temp = out;

                // Get out_ptr from the reference
                out = &mut [0];

                // chunk = original_chunk;
                if let Some(next_level) = level.next_.as_ref() {
                    out = self.process_level(next_level, original_chunk, out, is_last_on_level);
                }

                let out_ptr = out;
                out = out_temp;

                let filled_ptr = self.fill_with_zeros(
                    None,
                    right_boundary.as_mut_ptr(),
                    out_ptr.as_mut_ptr(),
                    is_last_on_level,
                )
                    .as_mut_ptr();

                unsafe {
                    let dist = out_ptr.as_mut_ptr() as usize - filled_ptr as usize;

                    //Update the out-pointer by given distance.
                    let new_out_ptr = out_ptr.as_mut_ptr().sub(dist);
                    out = &mut *(new_out_ptr as *mut [u8]);
                }
            } else {
                assert_eq!(comparison, 0);
                // If the chunk is equal to the divisor, we know that the right half
                // is all '0', and the left half is '...0001'.
                // Handling this case specially is an optimization; we could also
                // fall through to the generic "chunk > divisor" path below.

                let ptr = out;

                // Get out_ptr from the reference
                out = &mut [0];

                let filled_ptr = self.fill_with_zeros(
                    None,
                    right_boundary.as_mut_ptr(),
                    ptr.as_mut_ptr(),
                    false,
                )
                    .as_mut_ptr();

                unsafe {
                    let dist = ptr.as_mut_ptr() as usize - filled_ptr as usize;

                    //Update the out-pointer by given distance.
                    let new_out_ptr = ptr.as_mut_ptr().sub(dist);
                    out = &mut *(new_out_ptr as *mut [u8]);
                }
                *(--out) = b'1';
            }
            // In both cases, make sure the left half is fully written.
            let ptr = out;

            // Get out_ptr from the reference
            out = &mut [0];

            let filled_ptr = self.fill_with_zeros(
                None,
                right_boundary.as_mut_ptr(),
                ptr.as_mut_ptr(),
                is_last_on_level,
            )
                .as_mut_ptr();

            unsafe {
                let dist = ptr.as_mut_ptr() as usize - filled_ptr as usize;

                //Update the out-pointer by given distance.
                let new_out_ptr = ptr.as_mut_ptr().sub(dist);
                out = &mut *(new_out_ptr as *mut [u8]);
            }

            return out;
        }
        // Step 3: Allocate space for the results.
        // Allocate one extra digit so the next level can left-shift in-place.
        let mut right = ScratchDigits::new(level.divisor_.len() + 1);
        // Allocate one extra digit because DivideBarrett requires it.
        let mut left = ScratchDigits::new(chunk.len() - level.divisor_.len() + 1);

        // Step 4: Divide to split {chunk} into {left} and {right}.
        let inverse_len = chunk.len() - level.divisor_.len();
        if inverse_len == 0 {
            self.processor_.divide_schoolbook(
                left.as_rw_digits(),
                right.as_rw_digits(),
                chunk,
                level.divisor_.as_digits(),
            );
        } else if level.divisor_.len() == 1 {
            let right_digit = right.as_rw_digits();
            self.processor_.divide_single(
                left.as_rw_digits(),
                right_digit,
                chunk,
                level.divisor_[0],
            );
            for i in 1..right.len() {
                right[i] = 0;
            }
        } else {
            let mut scratch = ScratchDigits::new(DivideBarrettScratchSpace(chunk.len()));
            // The top level only computes its inverse when {chunk.len()} is
            // available. Other levels have precomputed theirs.
            if level.is_toplevel_ {
                // TODO: Implement this function for processor
                level.compute_inverse(self.processor_, chunk.len());
                if self.processor_.should_terminate() {
                    return out;
                }
            }
            let inverse = level.get_inverse(chunk.len());
            self.processor_.divide_barrett(
                left.as_rw_digits(),
                right.as_rw_digits(),
                chunk,
                level.divisor_.as_digits(),
                inverse,
                &mut scratch,
            );

            if self.processor_.should_terminate() {
                return out;
            }
        }
        