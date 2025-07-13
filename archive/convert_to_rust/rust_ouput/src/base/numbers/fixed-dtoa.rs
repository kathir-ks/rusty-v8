// Converted from V8 C++ source files:
// Header: fixed-dtoa.h
// Implementation: fixed-dtoa.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
pub mod numbers {
use std::fmt;
#[derive(Debug)]
pub enum FixedDtoaError {
    InvalidInput,
    BufferTooSmall,
}

impl fmt::Display for FixedDtoaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FixedDtoaError::InvalidInput => write!(f, "Invalid input parameters"),
            FixedDtoaError::BufferTooSmall => write!(f, "Buffer is too small to hold the result"),
        }
    }
}

impl std::error::Error for FixedDtoaError {}

// Represents a 128bit type. This class should be replaced by a native type on
// platforms that support 128bit integers.
#[derive(Clone, Copy)]
struct UInt128 {
    high_bits_: u64,
    low_bits_: u64,
}

impl UInt128 {
    pub fn new() -> Self {
        UInt128 {
            high_bits_: 0,
            low_bits_: 0,
        }
    }

    pub fn from_parts(high: u64, low: u64) -> Self {
        UInt128 {
            high_bits_: high,
            low_bits_: low,
        }
    }

    pub fn multiply(&mut self, multiplicand: u32) {
        let kMask32: u64 = 0xFFFFFFFF;
        let mut accumulator: u64;

        accumulator = (self.low_bits_ & kMask32) * (multiplicand as u64);
        let part = (accumulator & kMask32) as u32;
        accumulator >>= 32;
        accumulator = accumulator + (self.low_bits_ >> 32) * (multiplicand as u64);
        self.low_bits_ = (accumulator << 32) + (part as u64);
        accumulator >>= 32;
        accumulator = accumulator + (self.high_bits_ & kMask32) * (multiplicand as u64);
        let part = (accumulator & kMask32) as u32;
        accumulator >>= 32;
        accumulator = accumulator + (self.high_bits_ >> 32) * (multiplicand as u64);
        self.high_bits_ = (accumulator << 32) + (part as u64);
        assert_eq!(accumulator >> 32, 0);
    }

    pub fn shift(&mut self, shift_amount: i32) {
        if shift_amount == 0 {
            return;
        } else if shift_amount == -64 {
            self.high_bits_ = self.low_bits_;
            self.low_bits_ = 0;
        } else if shift_amount == 64 {
            self.low_bits_ = self.high_bits_;
            self.high_bits_ = 0;
        } else if shift_amount <= 0 {
            self.high_bits_ <<= -shift_amount;
            self.high_bits_ += self.low_bits_ >> (64 + shift_amount);
            self.low_bits_ <<= -shift_amount;
        } else {
            self.low_bits_ >>= shift_amount;
            self.low_bits_ += self.high_bits_ << (64 - shift_amount);
            self.high_bits_ >>= shift_amount;
        }
    }

    // Modifies *this to *this MOD (2^power).
    // Returns *this DIV (2^power).
    pub fn div_mod_power_of_2(&mut self, power: i32) -> i32 {
        if power >= 64 {
            let result = (self.high_bits_ >> (power - 64)) as i32;
            self.high_bits_ -= (result as u64) << (power - 64);
            return result;
        } else {
            let part_low = self.low_bits_ >> power;
            let part_high = self.high_bits_ << (64 - power);
            let result = (part_low + part_high) as i32;
            self.high_bits_ = 0;
            self.low_bits_ -= part_low << power;
            return result;
        }
    }

    pub fn is_zero(&self) -> bool {
        self.high_bits_ == 0 && self.low_bits_ == 0
    }

    pub fn bit_at(&self, position: i32) -> i32 {
        if position >= 64 {
            ((self.high_bits_ >> (position - 64)) & 1) as i32
        } else {
            ((self.low_bits_ >> position) & 1) as i32
        }
    }
}

const K_DOUBLE_SIGNIFICAND_SIZE: i32 = 53; // Includes the hidden bit.

fn fill_digits32_fixed_length(
    number: u32,
    requested_length: i32,
    buffer: &mut [char],
    length: &mut i32,
) {
    for i in (0..requested_length).rev() {
        buffer[(*length as usize) + (i as usize)] = char::from_digit((number % 10) as u32, 10).unwrap();
        let mut num = number;
        num /= 10;
    }
    *length += requested_length;
}

fn fill_digits32(number: u32, buffer: &mut [char], length: &mut i32) {
    let mut number_length = 0;
    let mut num = number;
    // We fill the digits in reverse order and exchange them afterwards.
    while num != 0 {
        let digit = num % 10;
        num /= 10;
        buffer[(*length as usize) + (number_length as usize)] = char::from_digit(digit, 10).unwrap();
        number_length += 1;
    }

    // Exchange the digits.
    let mut i = *length;
    let mut j = *length + number_length - 1;
    while i < j {
        let tmp = buffer[i as usize];
        buffer[i as usize] = buffer[j as usize];
        buffer[j as usize] = tmp;
        i += 1;
        j -= 1;
    }
    *length += number_length;
}

fn fill_digits64_fixed_length(
    number: u64,
    requested_length: i32,
    buffer: &mut [char],
    length: &mut i32,
) {
    const K_TEN7: u32 = 10000000;
    // For efficiency cut the number into 3 uint32_t parts, and print those.
    let part2 = (number % (K_TEN7 as u64)) as u32;
    let mut num = number;
    num /= K_TEN7 as u64;
    let part1 = (num % (K_TEN7 as u64)) as u32;
    let part0 = (num / (K_TEN7 as u64)) as u32;

    fill_digits32_fixed_length(part0, 3, buffer, length);
    fill_digits32_fixed_length(part1, 7, buffer, length);
    fill_digits32_fixed_length(part2, 7, buffer, length);
}

fn fill_digits64(number: u64, buffer: &mut [char], length: &mut i32) {
    const K_TEN7: u32 = 10000000;
    // For efficiency cut the number into 3 uint32_t parts, and print those.
    let part2 = (number % (K_TEN7 as u64)) as u32;
    let mut num = number;
    num /= K_TEN7 as u64;
    let part1 = (num % (K_TEN7 as u64)) as u32;
    let part0 = (num / (K_TEN7 as u64)) as u32;

    if part0 != 0 {
        fill_digits32(part0, buffer, length);
        fill_digits32_fixed_length(part1, 7, buffer, length);
        fill_digits32_fixed_length(part2, 7, buffer, length);
    } else if part1 != 0 {
        fill_digits32(part1, buffer, length);
        fill_digits32_fixed_length(part2, 7, buffer, length);
    } else {
        fill_digits32(part2, buffer, length);
    }
}

fn dtoa_round_up(buffer: &mut [char], length: &mut i32, decimal_point: &mut i32) {
    // An empty buffer represents 0.
    if *length == 0 {
        buffer[0] = '1';
        *decimal_point = 1;
        *length = 1;
        return;
    }
    // Round the last digit until we either have a digit that was not '9' or until
    // we reached the first digit.
    buffer[(*length as usize) - 1] = ((buffer[(*length as usize) - 1] as u8) + 1) as char;
    for i in (*length - 1..0).rev() {
        if buffer[i as usize] != char::from_digit(10, 10).unwrap() {
            return;
        }
        buffer[i as usize] = '0';
        buffer[(i - 1) as usize] = ((buffer[(i - 1) as usize] as u8) + 1) as char;
    }
    // If the first digit is now '0' + 10, we would need to set it to '0' and add
    // a '1' in front. However we reach the first digit only if all following
    // digits had been '9' before rounding up. Now all trailing digits are '0' and
    // we simply switch the first digit to '1' and update the decimal-point
    // (indicating that the point is now one digit to the right).
    if buffer[0] == char::from_digit(10, 10).unwrap() {
        buffer[0] = '1';
        *decimal_point += 1;
    }
}

// The given fractionals number represents a fixed-point number with binary
// point at bit (-exponent).
// Preconditions:
//   -128 <= exponent <= 0.
//   0 <= fractionals * 2^exponent < 1
//   The buffer holds the result.
// The function will round its result. During the rounding-process digits not
// generated by this function might be updated, and the decimal-point variable
// might be updated. If this function generates the digits 99 and the buffer
// already contained "199" (thus yielding a buffer of "19999") then a
// rounding-up will change the contents of the buffer to "20000".
fn fill_fractionals(
    fractionals: u64,
    exponent: i32,
    fractional_count: i32,
    buffer: &mut [char],
    length: &mut i32,
    decimal_point: &mut i32,
) {
    assert!(-128 <= exponent && exponent <= 0);
    // 'fractionals' is a fixed-point number, with binary point at bit
    // (-exponent). Inside the function the non-converted remainder of fractionals
    // is a fixed-point number, with binary point at bit 'point'.
    if -exponent <= 64 {
        // One 64 bit number is sufficient.
        assert_eq!(fractionals >> 56, 0);
        let mut point = -exponent;
        let mut fractionals_mut = fractionals;

        for _i in 0..fractional_count {
            if fractionals_mut == 0 {
                break;
            }
            // Instead of multiplying by 10 we multiply by 5 and adjust the point
            // location. This way the fractionals variable will not overflow.
            // Invariant at the beginning of the loop: fractionals < 2^point.
            // Initially we have: point <= 64 and fractionals < 2^56
            // After each iteration the point is decremented by one.
            // Note that 5^3 = 125 < 128 = 2^7.
            // Therefore three iterations of this loop will not overflow fractionals
            // (even without the subtraction at the end of the loop body). At this
            // time point will satisfy point <= 61 and therefore fractionals < 2^point
            // and any further multiplication of fractionals by 5 will not overflow.
            fractionals_mut *= 5;
            point -= 1;
            let digit = (fractionals_mut >> point) as i32;
            buffer[*length as usize] = char::from_digit(digit as u32, 10).unwrap();
            *length += 1;
            fractionals_mut -= (digit as u64) << point;
        }
        // If the first bit after the point is set we have to round up.
        if point > 0 && ((fractionals_mut >> (point - 1)) & 1) == 1 {
            dtoa_round_up(buffer, length, decimal_point);
        }
    } else {
        // We need 128 bits.
        assert!(64 < -exponent && -exponent <= 128);
        let mut fractionals128 = UInt128::from_parts(fractionals, 0);
        fractionals128.shift(-exponent - 64);
        let mut point = 128;
        for _i in 0..fractional_count {
            if fractionals128.is_zero() {
                break;
            }
            // As before: instead of multiplying by 10 we multiply by 5 and adjust the
            // point location.
            // This multiplication will not overflow for the same reasons as before.
            fractionals128.multiply(5);
            point -= 1;
            let digit = fractionals128.div_mod_power_of_2(point);
            buffer[*length as usize] = char::from_digit(digit as u32, 10).unwrap();
            *length += 1;
        }
        if fractionals128.bit_at(point - 1) == 1 {
            dtoa_round_up(buffer, length, decimal_point);
        }
    }
}

// Removes leading and trailing zeros.
// If leading zeros are removed then the decimal point position is adjusted.
fn trim_zeros(buffer: &mut [char], length: &mut i32, decimal_point: &mut i32) {
    while *length > 0 && buffer[(*length as usize) - 1] == '0' {
        *length -= 1;
    }
    let mut first_non_zero = 0;
    while (first_non_zero as i32) < *length && buffer[first_non_zero as usize] == '0' {
        first_non_zero += 1;
    }
    if first_non_zero != 0 {
        for i in first_non_zero as i32..*length {
            buffer[(i - first_non_zero as i32) as usize] = buffer[i as usize];
        }
        *length -= first_non_zero as i32;
        *decimal_point -= first_non_zero as i32;
    }
}

pub fn fast_fixed_dtoa(
    v: f64,
    fractional_count: i32,
    buffer: &mut [char],
    length: &mut i32,
    decimal_point: &mut i32,
) -> Result<bool, FixedDtoaError> {
    const K_MAX_UINT32: u32 = 0xFFFFFFFF;
    let double_val = Double(v);
    let significand = double_val.significand();
    let exponent = double_val.exponent();

    // v = significand * 2^exponent (with significand a 53bit integer).
    // If the exponent is larger than 20 (i.e. we may have a 73bit number) then we
    // don't know how to compute the representation. 2^73 ~= 9.5*10^21.
    // If necessary this limit could probably be increased, but we don't need
    // more.
    if exponent > 20 {
        return Err(FixedDtoaError::InvalidInput);
    }
    if fractional_count > 20 {
        return Err(FixedDtoaError::InvalidInput);
    }

    *length = 0;
    // At most kDoubleSignificandSize bits of the significand are non-zero.
    // Given a 64 bit integer we have 11 0s followed by 53 potentially non-zero
    // bits:  0..11*..0xxx..53*..xx
    if exponent + K_DOUBLE_SIGNIFICAND_SIZE > 64 {
        // The exponent must be > 11.
        //
        // We know that v = significand * 2^exponent.
        // And the exponent > 11.
        // We simplify the task by dividing v by 10^17.
        // The quotient delivers the first digits, and the remainder fits into a 64
        // bit number.
        // Dividing by 10^17 is equivalent to dividing by 5^17*2^17.
        const K_FIVE17: u64 = 0xB1A2BC2EC5; // 5^17
        let divisor = K_FIVE17;
        let divisor_power = 17;
        let dividend = significand;
        let quotient: u32;
        let remainder: u64;

        // Let v = f * 2^e with f == significand and e == exponent.
        // Then need q (quotient) and r (remainder) as follows:
        //   v            = q * 10^17       + r
        //   f * 2^e      = q * 10^17       + r
        //   f * 2^e      = q * 5^17 * 2^17 + r
        // If e > 17 then
        //   f * 2^(e-17) = q * 5^17        + r/2^17
        // else
        //   f  = q * 5^17 * 2^(17-e) + r/2^e
        if exponent > divisor_power {
            // We only allow exponents of up to 20 and therefore (17 - e) <= 3
            let dividend = dividend << (exponent - divisor_power);
            quotient = (dividend / divisor) as u32;
            remainder = (dividend % divisor) << divisor_power;
        } else {
            let divisor = divisor << (divisor_power - exponent);
            quotient = (dividend / divisor) as u32;
            remainder = (dividend % divisor) << exponent;
        }

        fill_digits32(quotient, buffer, length);
        fill_digits64_fixed_length(remainder, divisor_power, buffer, length);
        *decimal_point = *length;
    } else if exponent >= 0 {
        // 0 <= exponent <= 11
        let significand = significand << exponent;
        fill_digits64(significand, buffer, length);
        *decimal_point = *length;
    } else if exponent > -K_DOUBLE_SIGNIFICAND_SIZE {
        // We have to cut the number.
        let integrals = significand >> -exponent;
        let fractionals = significand - (integrals << -exponent);
        if integrals > (K_MAX_UINT32 as u64) {
            fill_digits64(integrals, buffer, length);
        } else {
            fill_digits32(integrals as u32, buffer, length);
        }
        *decimal_point = *length;
        fill_fractionals(
            fractionals,
            exponent,
            fractional_count,
            buffer,
            length,
            decimal_point,
        );
    } else if exponent < -128 {
        // This configuration (with at most 20 digits) means that all digits must be
        // 0.
        assert!(fractional_count <= 20);
        buffer[0] = '\0';
        *length = 0;
        *decimal_point = -fractional_count;
    } else {
        *decimal_point = 0;
        fill_fractionals(
            significand,
            exponent,
            fractional_count,
            buffer,
            length,
            decimal_point,
        );
    }

    trim_zeros(buffer, length, decimal_point);
    if (*length as usize) < buffer.len() {
      buffer[*length as usize] = '\0';
    }
    else {
      return Err(FixedDtoaError::BufferTooSmall);
    }

    if *length == 0 {
        // The string is empty and the decimal_point thus has no importance. Mimick
        // Gay's dtoa and and set it to -fractional_count.
        *decimal_point = -fractional_count;
    }

    Ok(true)
}
#[derive(Clone, Copy)]
struct Double {
    value: u64,
}

impl Double {
    fn new(value: f64) -> Self {
        Double {
            value: value.to_bits(),
        }
    }

    fn significand(&self) -> u64 {
        self.value & ((1u64 << 52) - 1) | (1u64 << 52)
    }

    fn exponent(&self) -> i32 {
        ((self.value >> 52) & ((1u64 << 11) - 1)) as i32 - 1023 - 52
    }
}

impl From<f64> for Double {
    fn from(f: f64) -> Self {
        Double::new(f)
    }
}
}
}
