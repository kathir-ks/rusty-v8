// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// #![allow(dead_code)] // Suppress unused code warnings in the generated code
// #![allow(non_snake_case)] // Allow C++-style names

mod base {
    pub mod numbers {
        pub mod strtod {
            use std::f64;
            use std::mem;

            use super::bignum::Bignum;
            use super::cached_powers::PowersOfTenCache;
            use super::double::Double;

            // 2^53 = 9007199254740992.
            // Any integer with at most 15 decimal digits will hence fit into a double
            // (which has a 53bit significand) without loss of precision.
            const K_MAX_EXACT_DOUBLE_INTEGER_DECIMAL_DIGITS: usize = 15;
            // 2^64 = 18446744073709551616 > 10^19
            const K_MAX_UINT64_DECIMAL_DIGITS: usize = 19;

            // Max double: 1.7976931348623157 x 10^308
            // Min non-zero double: 4.9406564584124654 x 10^-324
            // Any x >= 10^309 is interpreted as +infinity.
            // Any x <= 10^-324 is interpreted as 0.
            // Note that 2.5e-324 (despite being smaller than the min double) will be read
            // as non-zero (equal to the min non-zero double).
            const K_MAX_DECIMAL_POWER: i32 = 309;
            const K_MIN_DECIMAL_POWER: i32 = -324;

            // 2^64 = 18446744073709551616
            const K_MAX_UINT64: u64 = 0xFFFF_FFFF_FFFF_FFFF;

            const EXACT_POWERS_OF_TEN: [f64; 23] = [
                1.0,
                10.0,
                100.0,
                1000.0,
                10000.0,
                100000.0,
                1000000.0,
                10000000.0,
                100000000.0,
                1000000000.0,
                10000000000.0,
                100000000000.0,
                1000000000000.0,
                10000000000000.0,
                100000000000000.0,
                1000000000000000.0,
                10000000000000000.0,
                100000000000000000.0,
                1000000000000000000.0,
                10000000000000000000.0,
                100000000000000000000.0,
                1000000000000000000000.0,
                10000000000000000000000.0,
            ];

            const K_EXACT_POWERS_OF_TEN_SIZE: usize = EXACT_POWERS_OF_TEN.len();

            // Maximum number of significant digits in the decimal representation.
            // In fact the value is 772 (see conversions.cc), but to give us some margin
            // we round up to 780.
            const K_MAX_SIGNIFICANT_DECIMAL_DIGITS: usize = 780;

            fn trim_leading_zeros(buffer: &[u8]) -> &[u8] {
                for i in 0..buffer.len() {
                    if buffer[i] != b'0' {
                        return &buffer[i..];
                    }
                }
                &[]
            }

            fn trim_trailing_zeros(buffer: &[u8]) -> &[u8] {
                for i in (0..buffer.len()).rev() {
                    if buffer[i] != b'0' {
                        return &buffer[..i + 1];
                    }
                }
                &[]
            }

            fn trim_to_max_significant_digits(
                buffer: &[u8],
                exponent: i32,
                significant_buffer: &mut [u8],
                significant_exponent: &mut i32,
            ) {
                for i in 0..K_MAX_SIGNIFICANT_DECIMAL_DIGITS - 1 {
                    significant_buffer[i] = buffer[i];
                }
                // The input buffer has been trimmed. Therefore the last digit must be
                // different from '0'.
                // debug_assert_ne!(buffer[buffer.len() - 1], b'0'); // Removed DCHECK macro
                // Set the last digit to be non-zero. This is sufficient to guarantee
                // correct rounding.
                significant_buffer[K_MAX_SIGNIFICANT_DECIMAL_DIGITS - 1] = b'1';
                *significant_exponent =
                    exponent + (buffer.len() as i32 - K_MAX_SIGNIFICANT_DECIMAL_DIGITS as i32);
            }

            // Reads digits from the buffer and converts them to a uint64.
            // Reads in as many digits as fit into a uint64.
            // When the string starts with "1844674407370955161" no further digit is read.
            // Since 2^64 = 18446744073709551616 it would still be possible read another
            // digit if it was less or equal than 6, but this would complicate the code.
            fn read_uint64(buffer: &[u8], number_of_read_digits: &mut usize) -> u64 {
                let mut result: u64 = 0;
                let mut i: usize = 0;
                while i < buffer.len() && result <= (K_MAX_UINT64 / 10 - 1) {
                    let digit = (buffer[i] - b'0') as u64;
                    // debug_assert!((0..=9).contains(&digit)); // Removed DCHECK macro
                    result = 10 * result + digit;
                    i += 1;
                }
                *number_of_read_digits = i;
                result
            }

            // Reads a DiyFp from the buffer.
            // The returned DiyFp is not necessarily normalized.
            // If remaining_decimals is zero then the returned DiyFp is accurate.
            // Otherwise it has been rounded and has error of at most 1/2 ulp.
            fn read_diy_fp(buffer: &[u8], result: &mut DiyFp, remaining_decimals: &mut i32) {
                let mut read_digits: usize = 0;
                let significand: u64 = read_uint64(buffer, &mut read_digits);
                if buffer.len() == read_digits {
                    *result = DiyFp::new(significand, 0);
                    *remaining_decimals = 0;
                } else {
                    // Round the significand.
                    if buffer[read_digits] >= b'5' {
                        // Wrapping add is fine, this is the last operation and the MSB is always 0.
                        result.f = significand.wrapping_add(1);
                    }
                    // Compute the binary exponent.
                    let exponent: i32 = 0;
                    *result = DiyFp::new(significand, exponent);
                    *remaining_decimals = buffer.len() as i32 - read_digits as i32;
                }
            }

            // If the function returns true then the result is the correct double.
            // Otherwise it is either the correct double or the double that is just below
            // the correct double.
            fn diy_fp_strtod(buffer: &[u8], exponent: i32, result: &mut f64) -> bool {
                let mut input = DiyFp::default();
                let mut remaining_decimals: i32 = 0;
                read_diy_fp(buffer, &mut input, &mut remaining_decimals);
                // Since we may have dropped some digits the input is not accurate.
                // If remaining_decimals is different than 0 than the error is at most
                // .5 ulp (unit in the last place).
                // We don't want to deal with fractions and therefore keep a common
                // denominator.
                const K_DENOMINATOR_LOG: i32 = 3;
                const K_DENOMINATOR: i32 = 1 << K_DENOMINATOR_LOG;
                // Move the remaining decimals into the exponent.
                let mut exponent = exponent + remaining_decimals;
                let mut error: i64 = if remaining_decimals == 0 {
                    0
                } else {
                    K_DENOMINATOR as i64 / 2
                };

                let old_e = input.e();
                input.normalize();
                error <<= old_e - input.e();

                // debug_assert!(exponent <= PowersOfTenCache::K_MAX_DECIMAL_EXPONENT); // Removed DCHECK macro
                if exponent < PowersOfTenCache::K_MIN_DECIMAL_EXPONENT {
                    *result = 0.0;
                    return true;
                }
                let mut cached_power = DiyFp::default();
                let mut cached_decimal_exponent: i32 = 0;
                PowersOfTenCache::get_cached_power_for_decimal_exponent(
                    exponent,
                    &mut cached_power,
                    &mut cached_decimal_exponent,
                );

                if cached_decimal_exponent != exponent {
                    let adjustment_exponent = exponent - cached_decimal_exponent;
                    let adjustment_power = adjustment_power_of_ten(adjustment_exponent);
                    input.multiply(adjustment_power);
                    if K_MAX_UINT64_DECIMAL_DIGITS as i32 - buffer.len() as i32 >= adjustment_exponent {
                        // The product of input with the adjustment power fits into a 64 bit
                        // integer.
                        // debug_assert_eq!(DiyFp::k_significand_size(), 64); // Removed DCHECK macro
                    } else {
                        // The adjustment power is exact. There is hence only an error of 0.5.
                        error += K_DENOMINATOR as i64 / 2;
                    }
                }

                input.multiply(cached_power);
                // The error introduced by a multiplication of a*b equals
                //   error_a + error_b + error_a*error_b/2^64 + 0.5
                // Substituting a with 'input' and b with 'cached_power' we have
                //   error_b = 0.5  (all cached powers have an error of less than 0.5 ulp),
                //   error_ab = 0 or 1 / kDenominator > error_a*error_b/ 2^64
                let error_b = K_DENOMINATOR / 2;
                let error_ab = if error == 0 { 0 } else { 1 }; // We round up to 1.
                let fixed_error = K_DENOMINATOR / 2;
                error += error_b as i64 + error_ab as i64 + fixed_error as i64;

                let old_e = input.e();
                input.normalize();
                error <<= old_e - input.e();

                // See if the double's significand changes if we add/subtract the error.
                let order_of_magnitude = DiyFp::k_significand_size() as i32 + input.e();
                let effective_significand_size =
                    Double::significand_size_for_order_of_magnitude(order_of_magnitude);
                let precision_digits_count =
                    DiyFp::k_significand_size() as i32 - effective_significand_size;
                if precision_digits_count + K_DENOMINATOR_LOG >= DiyFp::k_significand_size() as i32 {
                    // This can only happen for very small denormals. In this case the
                    // half-way multiplied by the denominator exceeds the range of an uint64.
                    // Simply shift everything to the right.
                    let shift_amount = (precision_digits_count + K_DENOMINATOR_LOG)
                        - DiyFp::k_significand_size() as i32
                        + 1;
                    input.f >>= shift_amount;
                    input.e += shift_amount;
                    // We add 1 for the lost precision of error, and kDenominator for
                    // the lost precision of input.f().
                    error = (error >> shift_amount) + 1 + K_DENOMINATOR as i64;
                    // precision_digits_count -= shift_amount; // Removed unused assignment
                }
                // We use uint64_ts now. This only works if the DiyFp uses uint64_ts too.
                // debug_assert_eq!(DiyFp::k_significand_size(), 64); // Removed DCHECK macro
                // debug_assert!(precision_digits_count < 64); // Removed DCHECK macro
                let one64: u64 = 1;
                let precision_bits_mask: u64 = (one64 << precision_digits_count) - 1;
                let precision_bits: u64 = input.f & precision_bits_mask;
                let half_way: u64 = one64 << (precision_digits_count - 1);

                let precision_bits = precision_bits.wrapping_mul(K_DENOMINATOR as u64);
                let half_way = half_way.wrapping_mul(K_DENOMINATOR as u64);
                let mut rounded_input = DiyFp::new(
                    input.f >> precision_digits_count,
                    input.e + precision_digits_count,
                );
                if precision_bits >= half_way.wrapping_add(error as u64) {
                    rounded_input.f = rounded_input.f.wrapping_add(1);
                }
                // If the last_bits are too close to the half-way case than we are too
                // inaccurate and round down. In this case we return false so that we can
                // fall back to a more precise algorithm.

                *result = Double::new(rounded_input).value();
                if (half_way as i64 - error) < precision_bits as i64
                    && (precision_bits as i64) < half_way as i64 + error
                {
                    // Too imprecise. The caller will have to fall back to a slower version.
                    // However the returned number is guaranteed to be either the correct
                    // double, or the next-lower double.
                    return false;
                } else {
                    return true;
                }
            }

            fn double_strtod(trimmed: &[u8], exponent: i32, result: &mut f64) -> bool {
                if trimmed.len() <= K_MAX_EXACT_DOUBLE_INTEGER_DECIMAL_DIGITS {
                    let mut read_digits: usize = 0;
                    // The trimmed input fits into a double.
                    // If the 10^exponent (resp. 10^-exponent) fits into a double too then we
                    // can compute the result-double simply by multiplying (resp. dividing) the
                    // two numbers.
                    // This is possible because IEEE guarantees that floating-point operations
                    // return the best possible approximation.
                    if exponent < 0 && -exponent < K_EXACT_POWERS_OF_TEN_SIZE as i32 {
                        // 10^-exponent fits into a double.
                        *result = read_uint64(trimmed, &mut read_digits) as f64;
                        // debug_assert_eq!(read_digits, trimmed.len()); // Removed DCHECK macro
                        *result /= EXACT_POWERS_OF_TEN[(-exponent) as usize];
                        return true;
                    }
                    if 0 <= exponent && exponent < K_EXACT_POWERS_OF_TEN_SIZE as i32 {
                        // 10^exponent fits into a double.
                        *result = read_uint64(trimmed, &mut read_digits) as f64;
                        // debug_assert_eq!(read_digits, trimmed.len()); // Removed DCHECK macro
                        *result *= EXACT_POWERS_OF_TEN[exponent as usize];
                        return true;
                    }
                    let remaining_digits =
                        K_MAX_EXACT_DOUBLE_INTEGER_DECIMAL_DIGITS as i32 - trimmed.len() as i32;
                    if (0 <= exponent)
                        && (exponent - remaining_digits < K_EXACT_POWERS_OF_TEN_SIZE as i32)
                    {
                        // The trimmed string was short and we can multiply it with
                        // 10^remaining_digits. As a result the remaining exponent now fits
                        // into a double too.
                        *result = read_uint64(trimmed, &mut read_digits) as f64;
                        // debug_assert_eq!(read_digits, trimmed.len()); // Removed DCHECK macro
                        *result *= EXACT_POWERS_OF_TEN[remaining_digits as usize];
                        *result *=
                            EXACT_POWERS_OF_TEN[(exponent - remaining_digits) as usize];
                        return true;
                    }
                }
                false
            }

            // Returns 10^exponent as an exact DiyFp.
            // The given exponent must be in the range [1; kDecimalExponentDistance[.
            fn adjustment_power_of_ten(exponent: i32) -> DiyFp {
                // debug_assert!(0 < exponent); // Removed DCHECK macro
                // debug_assert!(exponent < PowersOfTenCache::K_DECIMAL_EXPONENT_DISTANCE); // Removed DCHECK macro
                // Simply hardcode the remaining powers for the given decimal exponent
                // distance.
                // debug_assert_eq!(PowersOfTenCache::K_DECIMAL_EXPONENT_DISTANCE, 8); // Removed DCHECK macro
                match exponent {
                    1 => DiyFp::new(0xA000_0000_0000_0000, -60),
                    2 => DiyFp::new(0xC800_0000_0000_0000, -57),
                    3 => DiyFp::new(0xFA00_0000_0000_0000, -54),
                    4 => DiyFp::new(0x9C40_0000_0000_0000, -50),
                    5 => DiyFp::new(0xC350_0000_0000_0000, -47),
                    6 => DiyFp::new(0xF424_0000_0000_0000, -44),
                    7 => DiyFp::new(0x9896_8000_0000_0000, -40),
                    _ => panic!("UNREACHABLE"), //UNREACHABLE(); // Replaced with panic!
                }
            }

            // Returns the correct double for the buffer*10^exponent.
            // The variable guess should be a close guess that is either the correct double
            // or its lower neighbor (the nearest double less than the correct one).
            // Preconditions:
            //   buffer.length() + exponent <= kMaxDecimalPower + 1
            //   buffer.length() + exponent > kMinDecimalPower
            //   buffer.length() <= kMaxDecimalSignificantDigits
            fn bignum_strtod(buffer: &[u8], exponent: i32, guess: f64) -> f64 {
                if guess == f64::INFINITY {
                    return guess;
                }

                let upper_boundary = Double::new(guess).upper_boundary();

                // debug_assert!(buffer.len() as i32 + exponent <= K_MAX_DECIMAL_POWER + 1); // Removed DCHECK macro
                // debug_assert!(buffer.len() as i32 + exponent > K_MIN_DECIMAL_POWER); // Removed DCHECK macro
                // debug_assert!(buffer.len() <= K_MAX_SIGNIFICANT_DECIMAL_DIGITS); // Removed DCHECK macro
                // Make sure that the Bignum will be able to hold all our numbers.
                // Our Bignum implementation has a separate field for exponents. Shifts will
                // consume at most one bigit (< 64 bits).
                // ln(10) == 3.3219...
                // debug_assert!(((K_MAX_DECIMAL_POWER + 1) * 333 / 100) < Bignum::k_max_significant_bits()); // Removed DCHECK macro
                let mut input = Bignum::default();
                let mut boundary = Bignum::default();
                input.assign_decimal_string(buffer);
                boundary.assign_u64(upper_boundary.f);
                if exponent >= 0 {
                    input.multiply_by_power_of_ten(exponent);
                } else {
                    boundary.multiply_by_power_of_ten(-exponent);
                }
                if upper_boundary.e > 0 {
                    boundary.shift_left(upper_boundary.e);
                } else {
                    input.shift_left(-upper_boundary.e);
                }
                let comparison = Bignum::compare(input, boundary);
                if comparison < 0 {
                    return guess;
                } else if comparison > 0 {
                    return Double::new(guess).next_double();
                } else if (Double::new(guess).significand() & 1) == 0 {
                    // Round towards even.
                    return guess;
                } else {
                    return Double::new(guess).next_double();
                }
            }

            pub fn strtod(buffer: &[u8], exponent: i32) -> f64 {
                let left_trimmed = trim_leading_zeros(buffer);
                let trimmed = trim_trailing_zeros(left_trimmed);
                let exponent = exponent + (left_trimmed.len() as i32 - trimmed.len() as i32);
                if trimmed.is_empty() {
                    return 0.0;
                }
                if trimmed.len() > K_MAX_SIGNIFICANT_DECIMAL_DIGITS {
                    let mut significant_buffer: [u8; K_MAX_SIGNIFICANT_DECIMAL_DIGITS] =
                        [0; K_MAX_SIGNIFICANT_DECIMAL_DIGITS];
                    let mut significant_exponent: i32 = 0;
                    trim_to_max_significant_digits(
                        trimmed,
                        exponent,
                        &mut significant_buffer,
                        &mut significant_exponent,
                    );
                    return strtod(&significant_buffer, significant_exponent);
                }
                if exponent + trimmed.len() as i32 - 1 >= K_MAX_DECIMAL_POWER {
                    return f64::INFINITY;
                }
                if exponent + trimmed.len() as i32 <= K_MIN_DECIMAL_POWER {
                    return 0.0;
                }

                let mut guess: f64 = 0.0;
                if double_strtod(trimmed, exponent, &mut guess)
                    || diy_fp_strtod(trimmed, exponent, &mut guess)
                {
                    return guess;
                }
                bignum_strtod(trimmed, exponent, guess)
            }

            #[derive(Default, Copy, Clone)]
            struct DiyFp {
                f: u64,
                e: i32,
            }

            impl DiyFp {
                const fn k_significand_size() -> usize {
                    64
                }

                const SIGNIFICAND_MASK: u64 = 0xFFFF_FFFF_FFFF_FFFF;

                fn new(f: u64, e: i32) -> Self {
                    DiyFp { f, e }
                }

                fn significand(&self) -> u64 {
                    self.f
                }
                fn exponent(&self) -> i32 {
                    self.e
                }

                fn set_f(&mut self, value: u64) {
                    self.f = value;
                }

                fn set_e(&mut self, value: i32) {
                    self.e = value;
                }

                fn f(&self) -> u64 {
                    self.f
                }

                fn e(&self) -> i32 {
                    self.e
                }

                fn normalize(&mut self) {
                    // If f is zero it does not matter what e is.
                    if self.f == 0 {
                        self.e = 0;
                        return;
                    }
                    let mut f = self.f;
                    let mut e = self.e;

                    // Count the number of leading zeros.
                    let leading_zeros = f.leading_zeros();
                    f <<= leading_zeros;
                    e -= leading_zeros as i32;

                    self.f = f;
                    self.e = e;
                }

                fn multiply(&mut self, other: DiyFp) {
                    self.f = self.f.wrapping_mul(other.f);
                    self.e += other.e;
                }
            }
        }

        pub mod bignum {
            // Placeholder for Bignum implementation (implementation not provided in the original file)
            #[derive(Default, Copy, Clone)]
            pub struct Bignum {}

            impl Bignum {
                pub const fn k_max_significant_bits() -> i32 {
                    10000 // Arbitrary large number
                }
                pub fn assign_decimal_string(&mut self, _buffer: &[u8]) {}
                pub fn assign_u64(&mut self, _value: u64) {}
                pub fn multiply_by_power_of_ten(&mut self, _exponent: i32) {}
                pub fn shift_left(&mut self, _bits: i32) {}
                pub fn compare(_a: Bignum, _b: Bignum) -> i32 {
                    0
                }
            }
        }

        pub mod cached_powers {
            // Placeholder for PowersOfTenCache implementation (implementation not provided in the original file)
            pub struct PowersOfTenCache {}
            impl PowersOfTenCache {
                pub const K_MIN_DECIMAL_EXPONENT: i32 = -340;
                pub const K_MAX_DECIMAL_EXPONENT: i32 = 340;
                pub const K_DECIMAL_EXPONENT_DISTANCE: i32 = 8;
                pub fn get_cached_power_for_decimal_exponent(
                    _exponent: i32,
                    _cached_power: &mut super::strtod::DiyFp,
                    _cached_decimal_exponent: &mut i32,
                ) {
                }
            }
        }

        pub mod double {
            use std::f64;

            use super::strtod::DiyFp;

            #[derive(Default, Copy, Clone)]
            pub struct Double {
                value: f64,
            }

            impl Double {
                pub fn new(diy_fp: DiyFp) -> Self {
                    let value: f64 = f64::from_bits(
                        ((diy_fp.f & 0x000F_FFFF_FFFF_FFFF) | (1023 + diy_fp.e) as u64 * (1 << 52))
                            .into(),
                    );
                    Double { value }
                }

                pub fn value(&self) -> f64 {
                    self.value
                }

                pub fn significand(&self) -> u64 {
                    self.value.to_bits() & 0x000F_FFFF_FFFF_FFFF
                }
                pub fn next_double(&self) -> f64 {
                    let mut bits = self.value.to_bits();
                    if self.value.is_sign_negative() {
                        bits = bits.wrapping_sub(1);
                    } else {
                        bits = bits.wrapping_add(1);
                    }
                    f64::from_bits(bits)
                }

                pub fn upper_boundary(&self) -> DiyFp {
                    //TODO: implement correctly
                    DiyFp {
                        f: self.value.to_bits(),
                        e: 0,
                    } // Placeholder
                }

                pub fn significand_size_for_order_of_magnitude(_order_of_magnitude: i32) -> i32 {
                    52 // Placeholder
                }
            }
        }
    }
}