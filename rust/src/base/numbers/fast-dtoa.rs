// Copyright 2011 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/base/numbers/fast-dtoa.rs

use std::convert::TryInto;
use std::fmt;

mod cached_powers;
mod diy_fp;
mod double;

use cached_powers::PowersOfTenCache;
use diy_fp::DiyFp;
use double::Double;

const K_MINIMAL_TARGET_EXPONENT: i32 = -60;
const K_MAXIMAL_TARGET_EXPONENT: i32 = -32;

/// Adjusts the last digit of the generated number, and screens out generated
/// solutions that may be inaccurate.
///
/// Returns `true` if the buffer is guaranteed to contain the closest
/// representable number to the input. Modifies the generated digits in the
/// buffer to approach (round towards) w.
fn round_weed(
    last_digit: &mut char,
    distance_too_high_w: u64,
    unsafe_interval: u64,
    rest: u64,
    ten_kappa: u64,
    unit: u64,
) -> bool {
    let small_distance = distance_too_high_w - unit;
    let big_distance = distance_too_high_w + unit;

    if rest < small_distance
        && unsafe_interval - rest >= ten_kappa
        && (rest + ten_kappa < small_distance
            || small_distance - rest >= rest + ten_kappa - small_distance)
    {
        *last_digit = char::from_digit((*last_digit as u32 - '0' as u32 - 1).try_into().unwrap(), 10).unwrap();
        
        rest += ten_kappa;
    }

    if rest < big_distance
        && unsafe_interval - rest >= ten_kappa
        && (rest + ten_kappa < big_distance
            || big_distance - rest > rest + ten_kappa - big_distance)
    {
        return false;
    }

    2 * unit <= rest && rest <= unsafe_interval - 4 * unit
}

/// Rounds the buffer upwards if the result is closer to v by possibly adding
/// 1 to the buffer. If the precision of the calculation is not sufficient to
/// round correctly, return false.
fn round_weed_counted(
    buffer: &mut [char],
    length: usize,
    rest: u64,
    ten_kappa: u64,
    unit: u64,
    kappa: &mut i32,
) -> bool {
    if unit >= ten_kappa {
        return false;
    }
    if ten_kappa - unit <= unit {
        return false;
    }

    if (ten_kappa - rest > rest) && (ten_kappa - 2 * rest >= 2 * unit) {
        return true;
    }

    if (rest > unit) && (ten_kappa - (rest - unit) <= (rest - unit)) {
        buffer[length - 1] = char::from_digit((buffer[length - 1] as u32 - '0' as u32 + 1).try_into().unwrap(), 10).unwrap();
        
        for i in (0..length - 1).rev() {
            if buffer[i + 1] != '0' {
                break;
            }
            buffer[i + 1] = '0';
            buffer[i] = char::from_digit((buffer[i] as u32 - '0' as u32 + 1).try_into().unwrap(), 10).unwrap();
            
        }

        if buffer[0] == char::from_digit(10, 10).unwrap() {
            buffer[0] = '1';
            *kappa += 1;
        }
        return true;
    }

    false
}

const K_TEN4: u32 = 10000;
const K_TEN5: u32 = 100000;
const K_TEN6: u32 = 1000000;
const K_TEN7: u32 = 10000000;
const K_TEN8: u32 = 100000000;
const K_TEN9: u32 = 1000000000;

#[derive(Clone, Copy)]
struct DivMagic {
    mul: u32,
    shift: u32,
}

static DIV: [DivMagic; 9] = [
    DivMagic { mul: 0, shift: 0 },
    DivMagic {
        mul: 0x9999999a,
        shift: 3,
    },
    DivMagic {
        mul: 0x47ae147b,
        shift: 6,
    },
    DivMagic {
        mul: 0x0624dd30,
        shift: 9,
    },
    DivMagic {
        mul: 0xa36e2eb2,
        shift: 13,
    },
    DivMagic {
        mul: 0x4f8b588f,
        shift: 16,
    },
    DivMagic {
        mul: 0x0c6f7a0c,
        shift: 19,
    },
    DivMagic {
        mul: 0xad7f29ac,
        shift: 23,
    },
    DivMagic {
        mul: 0x5798ee24,
        shift: 26,
    },
];

/// Returns *val / divisor, and does *val %= divisor. d must be the DivMagic
/// corresponding to the divisor.
fn fast_divmod(val: &mut u32, divisor: u32, d: &DivMagic) -> u32 {
    if divisor == 1 {
        let digit = *val;
        *val = 0;
        return digit;
    } else {
        let q = ((*val as u64) * (d.mul as u64)) >> 32;
        let t = ((*val - q) >> 1) + q;
        let digit = t >> d.shift;
        *val -= digit * divisor;
        return digit;
    }
}

/// Returns the biggest power of ten that is less than or equal than the given
/// number.
fn biggest_power_ten(number: u32, number_bits: i32, power: &mut u32, exponent: &mut i32) {
    match number_bits {
        32..=i32::MAX | 31 | 30 => {
            if K_TEN9 <= number {
                *power = K_TEN9;
                *exponent = 9;
            } else {
                match 29..=i32::MAX | 28 | 27 {
                    _ => {
                        if K_TEN8 <= number {
                            *power = K_TEN8;
                            *exponent = 8;
                        } else {
                            match 26..=i32::MAX | 25 | 24 {
                                _ => {
                                    if K_TEN7 <= number {
                                        *power = K_TEN7;
                                        *exponent = 7;
                                    } else {
                                        match 23..=i32::MAX | 22 | 21 | 20 {
                                            _ => {
                                                if K_TEN6 <= number {
                                                    *power = K_TEN6;
                                                    *exponent = 6;
                                                } else {
                                                    match 19..=i32::MAX | 18 | 17 {
                                                        _ => {
                                                            if K_TEN5 <= number {
                                                                *power = K_TEN5;
                                                                *exponent = 5;
                                                            } else {
                                                                match 16..=i32::MAX | 15 | 14 {
                                                                    _ => {
                                                                        if K_TEN4 <= number {
                                                                            *power = K_TEN4;
                                                                            *exponent = 4;
                                                                        } else {
                                                                            match 13..=i32::MAX | 12 | 11
                                                                                | 10
                                                                            {
                                                                                _ => {
                                                                                    if 1000 <= number {
                                                                                        *power = 1000;
                                                                                        *exponent = 3;
                                                                                    } else {
                                                                                        match 9..=i32::MAX | 8 | 7 {
                                                                                            _ => {
                                                                                                if 100 <= number {
                                                                                                    *power = 100;
                                                                                                    *exponent = 2;
                                                                                                } else {
                                                                                                    match 6..=i32::MAX | 5 | 4 {
                                                                                                        _ => {
                                                                                                            if 10 <= number {
                                                                                                                *power = 10;
                                                                                                                *exponent = 1;
                                                                                                            } else {
                                                                                                                match 3..=i32::MAX | 2 | 1 {
                                                                                                                    _ => {
                                                                                                                        if 1 <= number {
                                                                                                                            *power = 1;
                                                                                                                            *exponent = 0;
                                                                                                                        } else {
                                                                                                                            match 0 {
                                                                                                                                _ => {
                                                                                                                                    *power = 0;
                                                                                                                                    *exponent = -1;
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {
            *power = 0;
            *exponent = 0;
            // unreachable!()
        }
    }
}

/// Generates the digits of input number w.
fn digit_gen(
    low: DiyFp,
    w: DiyFp,
    high: DiyFp,
    outptr: &mut &mut [u8],
    kappa: &mut i32,
) -> bool {
    assert_eq!(low.e(), w.e());
    assert_eq!(w.e(), high.e());
    assert!(low.f() + 1 <= high.f() - 1);
    assert!(K_MINIMAL_TARGET_EXPONENT <= w.e() && w.e() <= K_MAXIMAL_TARGET_EXPONENT);

    let unit = 1;
    let too_low = DiyFp::new(low.f() - unit, low.e());
    let too_high = DiyFp::new(high.f() + unit, high.e());

    let unsafe_interval = DiyFp::minus(too_high, too_low);

    let one = DiyFp::new((1u64 << -w.e()) as u64, w.e());

    let integrals = (too_high.f() >> -one.e()) as u32;
    let fractionals = too_high.f() & (one.f() - 1);

    let mut divisor = 0;
    let mut divisor_exponent: i32 = 0;
    biggest_power_ten(
        integrals,
        DiyFp::k_significand_size() as i32 - (-one.e()),
        &mut divisor,
        &mut divisor_exponent,
    );
    *kappa = divisor_exponent + 1;

    let mut integrals_mut = integrals;

    while *kappa > 0 {
        let digit = fast_divmod(&mut integrals_mut, divisor, &DIV[divisor_exponent as usize]);
        **outptr = b'0' + digit as u8;
        *outptr = &mut (*outptr)[1..];
        *kappa -= 1;

        let rest = ((integrals_mut as u64) << -one.e()) + fractionals;

        if rest < unsafe_interval.f() {
            let last_digit = (**outptr.split_at(0).0.last().unwrap()) as char;

            return round_weed(
                &mut (last_digit),
                DiyFp::minus(too_high, w).f(),
                unsafe_interval.f(),
                rest,
                (divisor as u64) << -one.e(),
                unit,
            );
        }

        if *kappa <= 0 {
            break;
        }

        divisor /= 10;
        divisor_exponent -= 1;
    }

    assert!(one.e() >= -60);
    assert!(fractionals < one.f());
    assert!(0xFFFF_FFFF_FFFF_FFFF / 10 >= one.f());

    let mut fractionals_mut = fractionals;
    let mut unit_mut = unit;
    let mut unsafe_interval_mut = unsafe_interval;

    loop {
        fractionals_mut *= 10;
        unit_mut *= 10;
        unsafe_interval_mut.set_f(unsafe_interval_mut.f() * 10);

        let digit = (fractionals_mut >> -one.e()) as i32;

        **outptr = b'0' + digit as u8;
        *outptr = &mut (*outptr)[1..];
        *kappa -= 1;
        fractionals_mut &= one.f() - 1; // Modulo by one

        if fractionals_mut < unsafe_interval_mut.f() {

            let last_digit = (**outptr.split_at(0).0.last().unwrap()) as char;
            return round_weed(
                &mut (last_digit),
                DiyFp::minus(too_high, w).f() * unit_mut,
                unsafe_interval_mut.f(),
                fractionals_mut,
                one.f(),
                unit_mut,
            );
        }
    }
}

/// Generates (at most) requested_digits of input number w.
fn digit_gen_counted(
    w: DiyFp,
    requested_digits: i32,
    buffer: &mut [char],
    length: &mut usize,
    kappa: &mut i32,
) -> bool {
    assert!(K_MINIMAL_TARGET_EXPONENT <= w.e() && w.e() <= K_MAXIMAL_TARGET_EXPONENT);
    assert!(K_MINIMAL_TARGET_EXPONENT >= -60);
    assert!(K_MAXIMAL_TARGET_EXPONENT <= -32);

    let mut w_error = 1;

    let one = DiyFp::new((1u64 << -w.e()) as u64, w.e());

    let integrals = (w.f() >> -one.e()) as u32;
    let fractionals = w.f() & (one.f() - 1);

    let mut divisor = 0;
    let mut divisor_exponent: i32 = 0;

    biggest_power_ten(
        integrals,
        DiyFp::k_significand_size() as i32 - (-one.e()),
        &mut divisor,
        &mut divisor_exponent,
    );

    *kappa = divisor_exponent + 1;
    *length = 0;

    let mut integrals_mut = integrals;
    let mut requested_digits_mut = requested_digits;

    while *kappa > 0 {
        let digit = fast_divmod(&mut integrals_mut, divisor, &DIV[divisor_exponent as usize]);
        buffer[*length] = char::from_digit(digit, 10).unwrap();
        *length += 1;
        requested_digits_mut -= 1;
        *kappa -= 1;

        if requested_digits_mut == 0 {
            break;
        }

        divisor /= 10;
        divisor_exponent -= 1;
    }

    if requested_digits_mut == 0 {
        let rest = ((integrals_mut as u64) << -one.e()) + fractionals;
        return round_weed_counted(
            buffer,
            *length,
            rest,
            (divisor as u64) << -one.e(),
            w_error,
            kappa,
        );
    }

    assert!(one.e() >= -60);
    assert!(fractionals < one.f());
    assert!(0xFFFF_FFFF_FFFF_FFFF / 10 >= one.f());

    let mut fractionals_mut = fractionals;
    let mut w_error_mut = w_error;

    while requested_digits_mut > 0 && fractionals_mut > w_error_mut {
        fractionals_mut *= 10;
        w_error_mut *= 10;

        let digit = (fractionals_mut >> -one.e()) as i32;
        buffer[*length] = char::from_digit(digit, 10).unwrap();
        *length += 1;
        requested_digits_mut -= 1;
        fractionals_mut &= one.f() - 1; // Modulo by one
        *kappa -= 1;
    }

    if requested_digits_mut != 0 {
        return false;
    }

    round_weed_counted(buffer, *length, fractionals_mut, one.f(), w_error_mut, kappa)
}

/// Provides a decimal representation of v.
fn grisu3(v: f64, outptr: &mut &mut [u8], decimal_exponent: &mut i32) -> bool {
    let w = Double::new(v).as_normalized_diy_fp();

    let mut boundary_minus = DiyFp::new(0,0);
    let mut boundary_plus = DiyFp::new(0,0);
    Double::new(v).normalized_boundaries(&mut boundary_minus, &mut boundary_plus);

    assert_eq!(boundary_plus.e(), w.e());

    let mut ten_mk = DiyFp::new(0,0);
    let mut mk: i32 = 0;

    let ten_mk_minimal_binary_exponent =
        K_MINIMAL_TARGET_EXPONENT - (w.e() + DiyFp::k_significand_size() as i32);
    let ten_mk_maximal_binary_exponent =
        K_MAXIMAL_TARGET_EXPONENT - (w.e() + DiyFp::k_significand_size() as i32);

    PowersOfTenCache::get_cached_power_for_binary_exponent_range(
        ten_mk_minimal_binary_exponent,
        ten_mk_maximal_binary_exponent,
        &mut ten_mk,
        &mut mk,
    );

    assert!(
        K_MINIMAL_TARGET_EXPONENT
            <= w.e() + ten_mk.e() + DiyFp::k_significand_size() as i32
            && K_MAXIMAL_TARGET_EXPONENT
                >= w.e() + ten_mk.e() + DiyFp::k_significand_size() as i32
    );

    let scaled_w = DiyFp::times(w, ten_mk);
    assert_eq!(
        scaled_w.e(),
        boundary_plus.e() + ten_mk.e() + DiyFp::k_significand_size() as i32
    );

    let scaled_boundary_minus = DiyFp::times(boundary_minus, ten_mk);
    let scaled_boundary_plus = DiyFp::times(boundary_plus, ten_mk);

    let mut kappa: i32 = 0;
    let result = digit_gen(
        scaled_boundary_minus,
        scaled_w,
        scaled_boundary_plus,
        outptr,
        &mut kappa,
    );
    *decimal_exponent = -mk + kappa;
    result
}

/// The "counted" version of grisu3 (see above) only generates requested_digits
/// number of digits.
fn grisu3_counted(
    v: f64,
    requested_digits: i32,
    buffer: &mut [char],
    length: &mut usize,
    decimal_exponent: &mut i32,
) -> bool {
    let w = Double::new(v).as_normalized_diy_fp();

    let mut ten_mk = DiyFp::new(0,0);
    let mut mk: i32 = 0;

    let ten_mk_minimal_binary_exponent =
        K_MINIMAL_TARGET_EXPONENT - (w.e() + DiyFp::k_significand_size() as i32);
    let ten_mk_maximal_binary_exponent =
        K_MAXIMAL_TARGET_EXPONENT - (w.e() + DiyFp::k_significand_size() as i32);

    PowersOfTenCache::get_cached_power_for_binary_exponent_range(
        ten_mk_minimal_binary_exponent,
        ten_mk_maximal_binary_exponent,
        &mut ten_mk,
        &mut mk,
    );

    assert!(
        K_MINIMAL_TARGET_EXPONENT
            <= w.e() + ten_mk.e() + DiyFp::k_significand_size() as i32
            && K_MAXIMAL_TARGET_EXPONENT
                >= w.e() + ten_mk.e() + DiyFp::k_significand_size() as i32
    );

    let scaled_w = DiyFp::times(w, ten_mk);

    let mut kappa: i32 = 0;
    let result = digit_gen_counted(scaled_w, requested_digits, buffer, length, &mut kappa);
    *decimal_exponent = -mk + kappa;
    result
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FastDtoaMode {
    /// Generate the shortest representation possible.
    Shortest,
    /// Generate a representation with the specified number of digits.
    Precision,
}

/// Provides a decimal representation of v.
pub fn fast_dtoa(
    v: f64,
    mode: FastDtoaMode,
    requested_digits: i32,
    buffer: &mut [char],
    length: &mut usize,
    decimal_point: &mut i32,
) -> bool {
    assert!(v > 0.0);
    assert!(!Double::new(v).is_special());

    let mut result = false;
    let mut decimal_exponent = 0;

    match mode {
        FastDtoaMode::Shortest => {
            let mut outptr_bytes = buffer.iter_mut().map(|c| *c as u8).collect::<Vec<u8>>();
            let mut outptr_slice: &mut [u8] = outptr_bytes.as_mut_slice();
            let mut outptr: &mut &mut [u8] = &mut outptr_slice;
            
            result = grisu3(v, outptr, &mut decimal_exponent);

            let len = buffer.len() - outptr.len();
            *length = len;

             
        }
        FastDtoaMode::Precision => {
            let mut local_length = 0;
            result = grisu3_counted(
                v,
                requested_digits,
                buffer,
                &mut local_length,
                &mut decimal_exponent,
            );
            *length = local_length;
        }
    }

    if result {
        *decimal_point = *length as i32 + decimal_exponent;
        // buffer[*length] = '\0'; // String termination is not required in Rust
    }
    result
}