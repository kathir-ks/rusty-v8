// Copyright 2011 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod bignum;
mod double_wrapper;

use crate::bignum::Bignum;
use crate::double_wrapper::Double;
use std::cmp::Ordering;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum BignumDtoaMode {
    Shortest,
    Fixed,
    Precision,
}

fn normalized_exponent(significand: u64, exponent: i32) -> i32 {
    assert_ne!(significand, 0);
    let mut sig = significand;
    let mut exp = exponent;
    while (sig & Double::k_hidden_bit()) == 0 {
        sig <<= 1;
        exp -= 1;
    }
    exp
}

// Returns an estimation of k such that 10^(k-1) <= v < 10^k.
fn estimate_power(exponent: i32) -> i32 {
    // This function estimates log10 of v where v = f*2^e (with e == exponent).
    // Note that 10^floor(log10(v)) <= v, but v <= 10^ceil(log10(v)).
    // Note that f is bounded by its container size. Let p = 53 (the double's
    // significand size). Then 2^(p-1) <= f < 2^p.
    //
    // Given that log10(v) == log2(v)/log2(10) and e+(len(f)-1) is quite close
    // to log2(v) the function is simplified to (e+(len(f)-1)/log2(10)).
    // The computed number undershoots by less than 0.631 (when we compute log3
    // and not log10).
    //
    // Optimization: since we only need an approximated result this computation
    // can be performed on 64 bit integers. On x86/x64 architecture the speedup is
    // not really measurable, though.
    //
    // Since we want to avoid overshooting we decrement by 1e10 so that
    // floating-point imprecisions don't affect us.
    //
    // Explanation for v's boundary m+: the computation takes advantage of
    // the fact that 2^(p-1) <= f < 2^p. Boundaries still satisfy this requirement
    // (even for denormals where the delta can be much more important).

    const K1_LOG10: f64 = 0.30102999566398114; // 1/lg(10)

    // For doubles len(f) == 53 (don't forget the hidden bit).
    const K_SIGNIFICAND_SIZE: i32 = 53;
    let estimate =
        (f64::from(exponent + K_SIGNIFICAND_SIZE - 1) * K1_LOG10 - 1e-10).ceil();
    estimate as i32
}

// Computes v / 10^estimated_power exactly, as a ratio of two bignums, numerator
// and denominator.
fn initial_scaled_start_values(
    v: f64,
    estimated_power: i32,
    need_boundary_deltas: bool,
    numerator: &mut Bignum,
    denominator: &mut Bignum,
    delta_minus: &mut Bignum,
    delta_plus: &mut Bignum,
) {
    if Double::new(v).exponent() >= 0 {
        initial_scaled_start_values_positive_exponent(
            v,
            estimated_power,
            need_boundary_deltas,
            numerator,
            denominator,
            delta_minus,
            delta_plus,
        );
    } else if estimated_power >= 0 {
        initial_scaled_start_values_negative_exponent_positive_power(
            v,
            estimated_power,
            need_boundary_deltas,
            numerator,
            denominator,
            delta_minus,
            delta_plus,
        );
    } else {
        initial_scaled_start_values_negative_exponent_negative_power(
            v,
            estimated_power,
            need_boundary_deltas,
            numerator,
            denominator,
            delta_minus,
            delta_plus,
        );
    }
}

// See comments for InitialScaledStartValues.
fn initial_scaled_start_values_positive_exponent(
    v: f64,
    estimated_power: i32,
    need_boundary_deltas: bool,
    numerator: &mut Bignum,
    denominator: &mut Bignum,
    delta_minus: &mut Bignum,
    delta_plus: &mut Bignum,
) {
    // A positive exponent implies a positive power.
    assert!(estimated_power >= 0);
    // Since the estimated_power is positive we simply multiply the denominator
    // by 10^estimated_power.

    // numerator = v.
    numerator.assign_u64(Double::new(v).significand());
    numerator.shift_left(Double::new(v).exponent());
    // denominator = 10^estimated_power.
    denominator.assign_power_u16(10, estimated_power);

    if need_boundary_deltas {
        // Introduce a common denominator so that the deltas to the boundaries are
        // integers.
        denominator.shift_left(1);
        numerator.shift_left(1);
        // Let v = f * 2^e, then m+ - v = 1/2 * 2^e; With the common
        // denominator (of 2) delta_plus equals 2^e.
        delta_plus.assign_u16(1);
        delta_plus.shift_left(Double::new(v).exponent());
        // Same for delta_minus (with adjustments below if f == 2^p-1).
        delta_minus.assign_u16(1);
        delta_minus.shift_left(Double::new(v).exponent());

        // If the significand (without the hidden bit) is 0, then the lower
        // boundary is closer than just half a ulp (unit in the last place).
        // There is only one exception: if the next lower number is a denormal then
        // the distance is 1 ulp. This cannot be the case for exponent >= 0 (but we
        // have to test it in the other function where exponent < 0).
        let v_bits = Double::new(v).as_u64();
        if (v_bits & Double::k_significand_mask()) == 0 {
            // The lower boundary is closer at half the distance of "normal" numbers.
            // Increase the common denominator and adapt all but the delta_minus.
            denominator.shift_left(1); // *2
            numerator.shift_left(1); // *2
            delta_plus.shift_left(1); // *2
        }
    }
}

// See comments for InitialScaledStartValues
fn initial_scaled_start_values_negative_exponent_positive_power(
    v: f64,
    estimated_power: i32,
    need_boundary_deltas: bool,
    numerator: &mut Bignum,
    denominator: &mut Bignum,
    delta_minus: &mut Bignum,
    delta_plus: &mut Bignum,
) {
    let significand = Double::new(v).significand();
    let exponent = Double::new(v).exponent();
    // v = f * 2^e with e < 0, and with estimated_power >= 0.
    // This means that e is close to 0 (have a look at how estimated_power is
    // computed).

    // numerator = significand
    //  since v = significand * 2^exponent this is equivalent to
    //  numerator = v * / 2^-exponent
    numerator.assign_u64(significand);
    // denominator = 10^estimated_power * 2^-exponent (with exponent < 0)
    denominator.assign_power_u16(10, estimated_power);
    denominator.shift_left(-exponent);

    if need_boundary_deltas {
        // Introduce a common denominator so that the deltas to the boundaries are
        // integers.
        denominator.shift_left(1);
        numerator.shift_left(1);
        // Let v = f * 2^e, then m+ - v = 1/2 * 2^e; With the common
        // denominator (of 2) delta_plus equals 2^e.
        // Given that the denominator already includes v's exponent the distance
        // to the boundaries is simply 1.
        delta_plus.assign_u16(1);
        // Same for delta_minus (with adjustments below if f == 2^p-1).
        delta_minus.assign_u16(1);

        // If the significand (without the hidden bit) is 0, then the lower
        // boundary is closer than just one ulp (unit in the last place).
        // There is only one exception: if the next lower number is a denormal
        // then the distance is 1 ulp. Since the exponent is close to zero
        // (otherwise estimated_power would have been negative) this cannot happen
        // here either.
        let v_bits = Double::new(v).as_u64();
        if (v_bits & Double::k_significand_mask()) == 0 {
            // The lower boundary is closer at half the distance of "normal" numbers.
            // Increase the denominator and adapt all but the delta_minus.
            denominator.shift_left(1); // *2
            numerator.shift_left(1); // *2
            delta_plus.shift_left(1); // *2
        }
    }
}

// See comments for InitialScaledStartValues
fn initial_scaled_start_values_negative_exponent_negative_power(
    v: f64,
    estimated_power: i32,
    need_boundary_deltas: bool,
    numerator: &mut Bignum,
    denominator: &mut Bignum,
    delta_minus: &mut Bignum,
    delta_plus: &mut Bignum,
) {
    const K_MINIMAL_NORMALIZED_EXPONENT: u64 = 0x0010_0000_0000_0000;
    let significand = Double::new(v).significand();
    let exponent = Double::new(v).exponent();
    // Instead of multiplying the denominator with 10^estimated_power we
    // multiply all values (numerator and deltas) by 10^-estimated_power.

    // Use numerator as temporary container for power_ten.
    let power_ten = numerator;
    power_ten.assign_power_u16(10, -estimated_power);

    if need_boundary_deltas {
        // Since power_ten == numerator we must make a copy of 10^estimated_power
        // before we complete the computation of the numerator.
        // delta_plus = delta_minus = 10^estimated_power
        delta_plus.assign_bignum(&power_ten);
        delta_minus.assign_bignum(&power_ten);
    }

    // numerator = significand * 2 * 10^-estimated_power
    //  since v = significand * 2^exponent this is equivalent to
    // numerator = v * 10^-estimated_power * 2 * 2^-exponent.
    // Remember: numerator has been abused as power_ten. So no need to assign it
    //  to itself.
    //DCHECK(numerator == power_ten);
    numerator.multiply_by_u64(significand);

    // denominator = 2 * 2^-exponent with exponent < 0.
    denominator.assign_u16(1);
    denominator.shift_left(-exponent);

    if need_boundary_deltas {
        // Introduce a common denominator so that the deltas to the boundaries are
        // integers.
        numerator.shift_left(1);
        denominator.shift_left(1);
        // With this shift the boundaries have their correct value, since
        // delta_plus = 10^-estimated_power, and
        // delta_minus = 10^-estimated_power.
        // These assignments have been done earlier.

        // The special case where the lower boundary is twice as close.
        // This time we have to look out for the exception too.
        let v_bits = Double::new(v).as_u64();
        if (v_bits & Double::k_significand_mask()) == 0
            &&
			// The only exception where a significand == 0 has its boundaries at
			// "normal" distances:
			(v_bits & Double::k_exponent_mask()) != K_MINIMAL_NORMALIZED_EXPONENT
        {
            numerator.shift_left(1); // *2
            denominator.shift_left(1); // *2
            delta_plus.shift_left(1); // *2
        }
    }
}

// This routine multiplies numerator/denominator so that its values lies in the
// range 1-10. That is after a call to this function we have:
//    1 <= (numerator + delta_plus) /denominator < 10.
// Let numerator the input before modification and numerator' the argument
// after modification, then the output-parameter decimal_point is such that
//  numerator / denominator * 10^estimated_power ==
//    numerator' / denominator' * 10^(decimal_point - 1)
// In some cases estimated_power was too low, and this is already the case. We
// then simply adjust the power so that 10^(k-1) <= v < 10^k (with k ==
// estimated_power) but do not touch the numerator or denominator.
// Otherwise the routine multiplies the numerator and the deltas by 10.
fn fixup_multiply10(
    estimated_power: i32,
    is_even: bool,
    decimal_point: &mut i32,
    numerator: &mut Bignum,
    denominator: &mut Bignum,
    delta_minus: &mut Bignum,
    delta_plus: &mut Bignum,
) {
    let in_range = if is_even {
        Bignum::plus_compare(numerator, delta_plus, denominator) >= Ordering::Equal
    } else {
        Bignum::plus_compare(numerator, delta_plus, denominator) > Ordering::Equal
    };

    if in_range {
        // Since numerator + delta_plus >= denominator we already have
        // 1 <= numerator/denominator < 10. Simply update the estimated_power.
        *decimal_point = estimated_power + 1;
    } else {
        *decimal_point = estimated_power;
        numerator.times10();
        if Bignum::equal(delta_minus, delta_plus) {
            delta_minus.times10();
            delta_plus.assign_bignum(delta_minus);
        } else {
            delta_minus.times10();
            delta_plus.times10();
        }
    }
}

// Generates digits from the left to the right and stops when the generated
// digits yield the shortest decimal representation of v.
fn generate_shortest_digits(
    numerator: &mut Bignum,
    denominator: &mut Bignum,
    delta_minus: &mut Bignum,
    delta_plus: &mut Bignum,
    is_even: bool,
    buffer: &mut [char],
    length: &mut usize,
) {
    // Small optimization: if delta_minus and delta_plus are the same just reuse
    // one of the two bignums.
    if Bignum::equal(delta_minus, delta_plus) {
        // SAFETY: This is safe because delta_plus is only ever read from, and we're reassigning the mutable reference
        // to point to the same Bignum as delta_minus.
        // delta_plus = delta_minus;
    }
    *length = 0;
    loop {
        let digit = numerator.divide_modulo_int_bignum(denominator);
        assert!(digit <= 9); // digit is a u16 and therefore always positive.
                              // digit = numerator / denominator (integer division).
                              // numerator = numerator % denominator.
        buffer[*length] = char::from_digit(u32::from(digit), 10).unwrap();
        *length += 1;

        // Can we stop already?
        // If the remainder of the division is less than the distance to the lower
        // boundary we can stop. In this case we simply round down (discarding the
        // remainder).
        // Similarly we test if we can round up (using the upper boundary).
        let in_delta_room_minus = if is_even {
            Bignum::less_equal(numerator, delta_minus)
        } else {
            Bignum::less(numerator, delta_minus)
        };
        let in_delta_room_plus = if is_even {
            Bignum::plus_compare(numerator, delta_plus, denominator) >= Ordering::Equal
        } else {
            Bignum::plus_compare(numerator, delta_plus, denominator) > Ordering::Equal
        };

        if !in_delta_room_minus && !in_delta_room_plus {
            // Prepare for next iteration.
            numerator.times10();
            delta_minus.times10();
            // We optimized delta_plus to be equal to delta_minus (if they share the
            // same value). So don't multiply delta_plus if they point to the same
            // object.
            if !std::ptr::eq(delta_minus, delta_plus) {
                delta_plus.times10();
            }
        } else if in_delta_room_minus && in_delta_room_plus {
            // Let's see if 2*numerator < denominator.
            // If yes, then the next digit would be < 5 and we can round down.
            let compare = Bignum::plus_compare(numerator, numerator, denominator);
            if compare < Ordering::Equal {
                // Remaining digits are less than .5. -> Round down (== do nothing).
            } else if compare > Ordering::Equal {
                // Remaining digits are more than .5 of denominator. -> Round up.
                // Note that the last digit could not be a '9' as otherwise the whole
                // loop would have stopped earlier.
                // We still have an assert here in case the preconditions were not
                // satisfied.
                assert_ne!(buffer[*length - 1], '9');
                buffer[*length - 1] = (buffer[*length - 1] as u8 + 1) as char;
            } else {
                // Halfway case.
                // TODO(floitsch): need a way to solve half-way cases.
                //   For now let's round towards even (since this is what Gay seems to
                //   do).

                if (buffer[*length - 1] as i32 - '0' as i32) % 2 == 0 {
                    // Round down => Do nothing.
                } else {
                    assert_ne!(buffer[*length - 1], '9');
                    buffer[*length - 1] = (buffer[*length - 1] as u8 + 1) as char;
                }
            }
            return;
        } else if in_delta_room_minus {
            // Round down (== do nothing).
            return;
        } else {
            // in_delta_room_plus
            // Round up.
            // Note again that the last digit could not be '9' since this would have
            // stopped the loop earlier.
            // We still have an assert here, in case the preconditions were not
            // satisfied.
            assert_ne!(buffer[*length - 1], '9');
            buffer[*length - 1] = (buffer[*length - 1] as u8 + 1) as char;
            return;
        }
    }
}

// Generates 'requested_digits' after the decimal point.
fn bignum_to_fixed(
    requested_digits: i32,
    decimal_point: &mut i32,
    numerator: &mut Bignum,
    denominator: &mut Bignum,
    buffer: &mut [char],
    length: &mut usize,
) {
    // Note that we have to look at more than just the requested_digits, since
    // a number could be rounded up. Example: v=0.5 with requested_digits=0.
    // Even though the power of v equals 0 we can't just stop here.
    if -(*decimal_point) > requested_digits {
        // The number is definitively too small.
        // Ex: 0.001 with requested_digits == 1.
        // Set decimal-point to -requested_digits. This is what Gay does.
        // Note that it should not have any effect anyways since the string is
        // empty.
        *decimal_point = -requested_digits;
        *length = 0;
        return;
    } else if -(*decimal_point) == requested_digits {
        // We only need to verify if the number rounds down or up.
        // Ex: 0.04 and 0.06 with requested_digits == 1.
        assert_eq!(*decimal_point, -requested_digits);
        // Initially the fraction lies in range (1, 10]. Multiply the denominator
        // by 10 so that we can compare more easily.
        denominator.times10();
        if Bignum::plus_compare(numerator, numerator, denominator) >= Ordering::Equal {
            // If the fraction is >= 0.5 then we have to include the rounded
            // digit.
            buffer[0] = '1';
            *length = 1;
            (*decimal_point) += 1;
        } else {
            // Note that we caught most of similar cases earlier.
            *length = 0;
        }
        return;
    } else {
        // The requested digits correspond to the digits after the point.
        // The variable 'needed_digits' includes the digits before the point.
        let needed_digits = (*decimal_point + requested_digits) as usize;
        generate_counted_digits(
            needed_digits,
            decimal_point,
            numerator,
            denominator,
            buffer,
            length,
        );
    }
}

// Generates 'count' digits of numerator/denominator.
// Once 'count' digits have been produced rounds the result depending on the
// remainder (remainders of exactly .5 round upwards). Might update the
// decimal_point when rounding up (for example for 0.9999).
fn generate_counted_digits(
    count: usize,
    decimal_point: &mut i32,
    numerator: &mut Bignum,
    denominator: &mut Bignum,
    buffer: &mut [char],
    length: &mut usize,
) {
    assert!(count >= 0);
    for i in 0..count - 1 {
        let digit = numerator.divide_modulo_int_bignum(denominator);
        assert!(digit <= 9); // digit is a u16 and therefore always positive.
                              // digit = numerator / denominator (integer division).
                              // numerator = numerator % denominator.
        buffer[i] = char::from_digit(u32::from(digit), 10).unwrap();
        // Prepare for next iteration.
        numerator.times10();
    }
    // Generate the last digit.
    let mut digit = numerator.divide_modulo_int_bignum(denominator);
    if Bignum::plus_compare(numerator, numerator, denominator) >= Ordering::Equal {
        digit += 1;
    }
    buffer[count - 1] = char::from_digit(u32::from(digit), 10).unwrap();

    // Correct bad digits (in case we had a sequence of '9's). Propagate the
    // carry until we hat a non-'9' or til we reach the first digit.
    for i in (0..count).rev() {
        if buffer[i] != char::from_digit(10, 10).unwrap() {
            break;
        }
        buffer[i] = '0';
        if i > 0 {
            buffer[i - 1] = (buffer[i - 1] as u8 + 1) as char;
        }
    }
    if buffer[0] == char::from_digit(10, 10).unwrap() {
        // Propagate a carry past the top place.
        buffer[0] = '1';
        (*decimal_point) += 1;
    }
    *length = count;
}

/// Converts a double-precision floating-point number to its shortest decimal representation.
///
/// # Arguments
///
/// * `v` - The double-precision floating-point number to convert. Must be positive and not special (NaN, Infinity).
/// * `mode` - The desired conversion mode (shortest, fixed, or precision).
/// * `requested_digits` - The number of digits requested after the decimal point (only relevant for fixed and precision modes).
/// * `buffer` - A mutable vector of characters to store the resulting decimal representation.  Must be large enough.
/// * `length` - A mutable reference to an integer that will store the length of the resulting decimal representation.
/// * `decimal_point` - A mutable reference to an integer that will store the position of the decimal point.
pub fn bignum_dtoa(
    v: f64,
    mode: BignumDtoaMode,
    requested_digits: i32,
    buffer: &mut [char],
    length: &mut usize,
    decimal_point: &mut i32,
) {
    assert!(v > 0.0);
    assert!(!Double::new(v).is_special());

    let significand = Double::new(v).significand();
    let is_even = (significand & 1) == 0;
    let exponent = Double::new(v).exponent();
    let normalized_exponent = normalized_exponent(significand, exponent);
    // estimated_power might be too low by 1.
    let estimated_power = estimate_power(normalized_exponent);

    // Shortcut for Fixed.
    // The requested digits correspond to the digits after the point. If the
    // number is much too small, then there is no need in trying to get any
    // digits.
    if mode == BignumDtoaMode::Fixed && -estimated_power - 1 > requested_digits {
        buffer[0] = '\0';
        *length = 0;
        // Set decimal-point to -requested_digits. This is what Gay does.
        // Note that it should not have any effect anyways since the string is
        // empty.
        *decimal_point = -requested_digits;
        return;
    }

    let mut numerator = Bignum::new();
    let mut denominator = Bignum::new();
    let mut delta_minus = Bignum::new();
    let mut delta_plus = Bignum::new();
    // Make sure the bignum can grow large enough. The smallest double equals
    // 4e-324. In this case the denominator needs fewer than 324*4 binary digits.
    // The maximum double is 1.7976931348623157e308 which needs fewer than
    // 308*4 binary digits.
    //DCHECK_GE(Bignum::kMaxSignificantBits, 324 * 4);
    let need_boundary_deltas = mode == BignumDtoaMode::Shortest;
    initial_scaled_start_values(
        v,
        estimated_power,
        need_boundary_deltas,
        &mut numerator,
        &mut denominator,
        &mut delta_minus,
        &mut delta_plus,
    );
    // We now have v = (numerator / denominator) * 10^estimated_power.
    fixup_multiply10(
        estimated_power,
        is_even,
        decimal_point,
        &mut numerator,
        &mut denominator,
        &mut delta_minus,
        &mut delta_plus,
    );
    // We now have v = (numerator / denominator) * 10^(decimal_point-1), and
    //  1 <= (numerator + delta_plus) / denominator < 10
    match mode {
        BignumDtoaMode::Shortest => generate_shortest_digits(
            &mut numerator,
            &mut denominator,
            &mut delta_minus,
            &mut delta_plus,
            is_even,
            buffer,
            length,
        ),
        BignumDtoaMode::Fixed => bignum_to_fixed(
            requested_digits,
            decimal_point,
            &mut numerator,
            &mut denominator,
            buffer,
            length,
        ),
        BignumDtoaMode::Precision => generate_counted_digits(
            requested_digits as usize,
            decimal_point,
            &mut numerator,
            &mut denominator,
            buffer,
            length,
        ),
    }
    buffer[*length] = '\0';
}