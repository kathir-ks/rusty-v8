// Converted from V8 C++ source files:
// Header: strtod.h
// Implementation: strtod.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/base/numbers/strtod.h
pub mod strtod_rs {
    use crate::base::vector::Vector;

    // The buffer must only contain digits in the range [0-9]. It must not
    // contain a dot or a sign. It must not start with '0', and must not be empty.
    pub fn strtod(buffer: Vector<char>, exponent: i32) -> f64;
}

// src/base/numbers/strtod.cc
pub mod strtod_impl {
    use crate::base::numbers::bignum::Bignum;
    use crate::base::numbers::cached_powers::PowersOfTenCache;
    use crate::base::numbers::double_rs::Double;
    use crate::base::numbers::diy_fp::DiyFp;
    use crate::base::vector::Vector;
    use std::f64;

    // 2^53 = 9007199254740992.
    // Any integer with at most 15 decimal digits will hence fit into a double
    // (which has a 53bit significand) without loss of precision.
    const K_MAX_EXACT_DOUBLE_INTEGER_DECIMAL_DIGITS: i32 = 15;
    // 2^64 = 18446744073709551616 > 10^19
    const K_MAX_UINT64_DECIMAL_DIGITS: i32 = 19;

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

    static EXACT_POWERS_OF_TEN: [f64; 23] = [
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
    const K_MAX_SIGNIFICANT_DECIMAL_DIGITS: i32 = 780;

    fn trim_leading_zeros(buffer: Vector<char>) -> Vector<char> {
        for i in 0..buffer.length() {
            if buffer[i] != '0' {
                return buffer.sub_vector(i, buffer.length());
            }
        }
        Vector::new()
    }

    fn trim_trailing_zeros(buffer: Vector<char>) -> Vector<char> {
        for i in (0..buffer.length()).rev() {
            if buffer[i] != '0' {
                return buffer.sub_vector(0, i + 1);
            }
        }
        Vector::new()
    }

    fn trim_to_max_significant_digits(
        buffer: Vector<char>,
        exponent: i32,
        significant_buffer: &mut [char],
        significant_exponent: &mut i32,
    ) {
        for i in 0..K_MAX_SIGNIFICANT_DECIMAL_DIGITS - 1 {
            significant_buffer[i as usize] = buffer[i as usize];
        }

        assert_ne!(buffer[buffer.length() - 1], '0');

        significant_buffer[(K_MAX_SIGNIFICANT_DECIMAL_DIGITS - 1) as usize] = '1';
        *significant_exponent = exponent + (buffer.length() as i32 - K_MAX_SIGNIFICANT_DECIMAL_DIGITS);
    }

    // Reads digits from the buffer and converts them to a uint64.
    // Reads in as many digits as fit into a uint64.
    // When the string starts with "1844674407370955161" no further digit is read.
    // Since 2^64 = 18446744073709551616 it would still be possible read another
    // digit if it was less or equal than 6, but this would complicate the code.
    fn read_uint64(buffer: Vector<char>, number_of_read_digits: &mut i32) -> u64 {
        let mut result: u64 = 0;
        let mut i: usize = 0;
        while i < buffer.length() && result <= (K_MAX_UINT64 / 10 - 1) {
            let digit = (buffer[i] as u32 - '0' as u32) as i32;
            assert!((0 <= digit) && (digit <= 9));
            result = 10 * result + digit as u64;
            i += 1;
        }
        *number_of_read_digits = i as i32;
        result
    }

    // Reads a DiyFp from the buffer.
    // The returned DiyFp is not necessarily normalized.
    // If remaining_decimals is zero then the returned DiyFp is accurate.
    // Otherwise it has been rounded and has error of at most 1/2 ulp.
    fn read_diy_fp(buffer: Vector<char>, result: &mut DiyFp, remaining_decimals: &mut i32) {
        let mut read_digits: i32 = 0;
        let significand = read_uint64(buffer, &mut read_digits);
        if buffer.length() == read_digits as usize {
            *result = DiyFp::new(significand, 0);
            *remaining_decimals = 0;
        } else {
            // Round the significand.
            if buffer[read_digits as usize] >= '5' {
                *result = DiyFp::new(significand + 1, 0);
            } else {
                *result = DiyFp::new(significand, 0);
            }
            // Compute the binary exponent.
            let exponent = 0;
            *remaining_decimals = buffer.length() as i32 - read_digits;
        }
    }

    fn double_strtod(trimmed: Vector<char>, exponent: i32, result: &mut f64) -> bool {
        if trimmed.length() <= K_MAX_EXACT_DOUBLE_INTEGER_DECIMAL_DIGITS as usize {
            let mut read_digits: i32 = 0;
            if exponent < 0 && -exponent < K_EXACT_POWERS_OF_TEN_SIZE as i32 {
                *result = read_uint64(trimmed, &mut read_digits) as f64;
                assert_eq!(read_digits, trimmed.length() as i32);
                *result /= EXACT_POWERS_OF_TEN[(-exponent) as usize];
                return true;
            }
            if 0 <= exponent && exponent < K_EXACT_POWERS_OF_TEN_SIZE as i32 {
                *result = read_uint64(trimmed, &mut read_digits) as f64;
                assert_eq!(read_digits, trimmed.length() as i32);
                *result *= EXACT_POWERS_OF_TEN[exponent as usize];
                return true;
            }
            let remaining_digits =
                K_MAX_EXACT_DOUBLE_INTEGER_DECIMAL_DIGITS as usize - trimmed.length();
            if (0 <= exponent) && (exponent - remaining_digits as i32 < K_EXACT_POWERS_OF_TEN_SIZE as i32) {
                *result = read_uint64(trimmed, &mut read_digits) as f64;
                assert_eq!(read_digits, trimmed.length() as i32);
                *result *= EXACT_POWERS_OF_TEN[remaining_digits];
                *result *= EXACT_POWERS_OF_TEN[(exponent - remaining_digits as i32) as usize];
                return true;
            }
        }
        false
    }

    // Returns 10^exponent as an exact DiyFp.
    // The given exponent must be in the range [1; kDecimalExponentDistance[.
    fn adjustment_power_of_ten(exponent: i32) -> DiyFp {
        assert!(0 < exponent);
        assert!(exponent < PowersOfTenCache::K_DECIMAL_EXPONENT_DISTANCE as i32);
        assert_eq!(PowersOfTenCache::K_DECIMAL_EXPONENT_DISTANCE, 8);
        match exponent {
            1 => DiyFp::new(0xA000_0000_0000_0000, -60),
            2 => DiyFp::new(0xC800_0000_0000_0000, -57),
            3 => DiyFp::new(0xFA00_0000_0000_0000, -54),
            4 => DiyFp::new(0x9C40_0000_0000_0000, -50),
            5 => DiyFp::new(0xC350_0000_0000_0000, -47),
            6 => DiyFp::new(0xF424_0000_0000_0000, -44),
            7 => DiyFp::new(0x9896_8000_0000_0000, -40),
            _ => panic!("UNREACHABLE"),
        }
    }

    // If the function returns true then the result is the correct double.
    // Otherwise it is either the correct double or the double that is just below
    // the correct double.
    fn diy_fp_strtod(buffer: Vector<char>, exponent: i32, result: &mut f64) -> bool {
        let mut input = DiyFp::new(0, 0);
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

        assert!(exponent <= PowersOfTenCache::K_MAX_DECIMAL_EXPONENT as i32);
        if exponent < PowersOfTenCache::K_MIN_DECIMAL_EXPONENT as i32 {
            *result = 0.0;
            return true;
        }
        let mut cached_power = DiyFp::new(0, 0);
        let mut cached_decimal_exponent: i32 = 0;
        PowersOfTenCache::get_cached_power_for_decimal_exponent(
            exponent as i32,
            &mut cached_power,
            &mut cached_decimal_exponent,
        );

        if cached_decimal_exponent != exponent {
            let adjustment_exponent = exponent - cached_decimal_exponent;
            let adjustment_power = adjustment_power_of_ten(adjustment_exponent);
            input.multiply(adjustment_power);
            if K_MAX_UINT64_DECIMAL_DIGITS as i32 - buffer.length() as i32 >= adjustment_exponent {
            } else {
                error += K_DENOMINATOR as i64 / 2;
            }
        }

        input.multiply(cached_power);
        let error_b = K_DENOMINATOR as i64 / 2;
        let error_ab: i64 = if error == 0 { 0 } else { 1 };
        let fixed_error = K_DENOMINATOR as i64 / 2;
        error += error_b + error_ab + fixed_error;

        let old_e = input.e();
        input.normalize();
        error <<= old_e - input.e();

        let order_of_magnitude = DiyFp::K_SIGNIFICAND_SIZE as i32 + input.e();
        let effective_significand_size = Double::significand_size_for_order_of_magnitude(order_of_magnitude);
        let precision_digits_count = DiyFp::K_SIGNIFICAND_SIZE as i32 - effective_significand_size as i32;
        if precision_digits_count + K_DENOMINATOR_LOG >= DiyFp::K_SIGNIFICAND_SIZE as i32 {
            let shift_amount = (precision_digits_count + K_DENOMINATOR_LOG) - DiyFp::K_SIGNIFICAND_SIZE as i32 + 1;
            input.set_f(input.f() >> shift_amount);
            input.set_e(input.e() + shift_amount);
            error = (error >> shift_amount) + 1 + K_DENOMINATOR as i64;
            let _ = precision_digits_count - shift_amount;
        }

        assert_eq!(DiyFp::K_SIGNIFICAND_SIZE, 64);
        assert!(precision_digits_count < 64);
        let one64: u64 = 1;
        let precision_bits_mask = (one64 << precision_digits_count) - 1;
        let precision_bits = input.f() & precision_bits_mask;
        let half_way = one64 << (precision_digits_count - 1);
        let half_way = half_way * K_DENOMINATOR as u64;
        let precision_bits = precision_bits * K_DENOMINATOR as u64;

        let rounded_input = DiyFp::new(input.f() >> precision_digits_count, input.e() + precision_digits_count);

        if precision_bits >= half_way + error as u64 {
            let mut rounded_input_clone = rounded_input.clone();
            rounded_input_clone.set_f(rounded_input_clone.f() + 1);
        }

        *result = Double::new(rounded_input).value();

        if half_way - error as u64 < precision_bits && precision_bits < half_way + error as u64 {
            return false;
        } else {
            return true;
        }
    }

    // Returns the correct double for the buffer*10^exponent.
    // The variable guess should be a close guess that is either the correct double
    // or its lower neighbor (the nearest double less than the correct one).
    // Preconditions:
    //   buffer.length() + exponent <= kMaxDecimalPower + 1
    //   buffer.length() + exponent > kMinDecimalPower
    //   buffer.length() <= kMaxDecimalSignificantDigits
    fn bignum_strtod(buffer: Vector<char>, exponent: i32, guess: f64) -> f64 {
        if guess == f64::INFINITY {
            return guess;
        }

        let upper_boundary = Double::new(DiyFp::new(guess.to_bits(), 0)).upper_boundary();

        assert!(buffer.length() as i32 + exponent <= K_MAX_DECIMAL_POWER + 1);
        assert!(buffer.length() as i32 + exponent > K_MIN_DECIMAL_POWER);
        assert!(buffer.length() as i32 <= K_MAX_SIGNIFICANT_DECIMAL_DIGITS);
        assert!(
            (K_MAX_DECIMAL_POWER + 1) as f64 * 3.33 / 100.0 < Bignum::K_MAX_SIGNIFICANT_BITS as f64
        );
        let mut input = Bignum::new();
        let mut boundary = Bignum::new();
        input.assign_decimal_string(buffer);
        boundary.assign_u64(upper_boundary.f());
        if exponent >= 0 {
            input.multiply_by_power_of_ten(exponent);
        } else {
            boundary.multiply_by_power_of_ten(-exponent);
        }
        if upper_boundary.e() > 0 {
            boundary.shift_left(upper_boundary.e());
        } else {
            input.shift_left(-upper_boundary.e());
        }
        let comparison = Bignum::compare(input, boundary);
        if comparison < 0 {
            return guess;
        } else if comparison > 0 {
            return Double::new(DiyFp::new(guess.to_bits(), 0)).next_double();
        } else if (Double::new(DiyFp::new(guess.to_bits(), 0)).significand() & 1) == 0 {
            return guess;
        } else {
            return Double::new(DiyFp::new(guess.to_bits(), 0)).next_double();
        }
    }

    pub fn strtod(buffer: Vector<char>, exponent: i32) -> f64 {
        let left_trimmed = trim_leading_zeros(buffer);
        let trimmed = trim_trailing_zeros(left_trimmed);
        let exponent = exponent + (left_trimmed.length() as i32 - trimmed.length() as i32);
        if trimmed.is_empty() {
            return 0.0;
        }
        if exponent + trimmed.length() as i32 - 1 >= K_MAX_DECIMAL_POWER {
            return f64::INFINITY;
        }
        if exponent + trimmed.length() as i32 <= K_MIN_DECIMAL_POWER {
            return 0.0;
        }

        let mut guess = 0.0;
        if double_strtod(trimmed.clone(), exponent, &mut guess) || diy_fp_strtod(trimmed.clone(), exponent, &mut guess) {
            return guess;
        }
        bignum_strtod(trimmed, exponent, guess)
    }
}

pub mod base {
    pub mod vector {
        #[derive(Clone, Debug)]
        pub struct Vector<T> {
            data: Vec<T>,
        }

        impl<T> Vector<T> {
            pub fn new() -> Self {
                Vector { data: Vec::new() }
            }

            pub fn from_vec(vec: Vec<T>) -> Self {
                Vector { data: vec }
            }

            pub fn len(&self) -> usize {
                self.data.len()
            }

            pub fn is_empty(&self) -> bool {
                self.data.is_empty()
            }

            pub fn get(&self, index: usize) -> Option<&T> {
                self.data.get(index)
            }

            pub fn begin(&self) -> *const T {
                self.data.as_ptr() as *const T
            }

            pub fn end(&self) -> *const T {
                if self.data.is_empty() {
                    self.data.as_ptr() as *const T
                } else {
                    unsafe { self.data.as_ptr().add(self.data.len()) as *const T }
                }
            }

            pub fn sub_vector(&self, start: usize, end: usize) -> Vector<T>
            where
                T: Clone,
            {
                Vector {
                    data: self.data[start..end].to_vec(),
                }
            }

            pub fn length(&self) -> usize {
                self.data.len()
            }
        }

        impl Vector<char> {
            pub fn sub_vector(&self, start: usize, end: usize) -> Vector<char> {
                Vector {
                    data: self.data[start..end].to_vec(),
                }
            }
        }

        impl<T> std::ops::Index<usize> for Vector<T> {
            type Output = T;

            fn index(&self, index: usize) -> &Self::Output {
                &self.data[index]
            }
        }
    }

    pub mod numbers {
        pub mod bignum {
            #[derive(Clone, Debug)]
            pub struct Bignum {
                data: Vec<u64>,
                exponent: i32,
                positive: bool,
            }

            impl Bignum {
                pub const K_MAX_SIGNIFICANT_BITS: i32 = 1024;
                pub fn new() -> Self {
                    Bignum {
                        data: Vec::new(),
                        exponent: 0,
                        positive: true,
                    }
                }
                pub fn assign_decimal_string(&mut self, buffer: vector::Vector<char>) {
                    self.data = buffer.data.iter().map(|c| (*c as u8 - b'0') as u64).collect();
                    self.exponent = 0;
                    self.positive = true;
                }
                pub fn assign_u64(&mut self, value: u64) {
                    self.data = vec![value];
                    self.exponent = 0;
                    self.positive = true;
                }
                pub fn multiply_by_power_of_ten(&mut self, exponent: i32) {
                    self.exponent += exponent;
                }
                pub fn shift_left(&mut self, bits: i32) {
                    self.exponent += bits;
                }
                pub fn compare(a: Bignum, b: Bignum) -> i32 {
                    if a.positive != b.positive {
                        if a.positive {
                            return 1;
                        } else {
                            return -1;
                        }
                    }
                    if a.data.len() != b.data.len() {
                        if a.data.len() > b.data.len() {
                            return 1;
                        } else {
                            return -1;
                        }
                    }
                    for i in (0..a.data.len()).rev() {
                        if a.data[i] != b.data[i] {
                            if a.data[i] > b.data[i] {
                                return 1;
                            } else {
                                return -1;
                            }
                        }
                    }
                    0
                }
            }
        }
        pub mod cached_powers {
            #[derive(Clone, Copy, Debug)]
            pub struct PowersOfTenCache {}

            impl PowersOfTenCache {
                pub const K_DECIMAL_EXPONENT_DISTANCE: usize = 8;
                pub const K_MIN_DECIMAL_EXPONENT: i32 = -340;
                pub const K_MAX_DECIMAL_EXPONENT: i32 = 340;
                pub fn get_cached_power_for_decimal_exponent(
                    exponent: i32,
                    power: &mut super::diy_fp::DiyFp,
                    decimal_exponent: &mut i32,
                ) {
                    *decimal_exponent = exponent;
                    *power = super::diy_fp::DiyFp::new(1, 0);
                }
            }
        }
        pub mod double_rs {
            use super::diy_fp::DiyFp;

            #[derive(Clone, Copy, Debug)]
            pub struct Double {
                value: f64,
            }

            impl Double {
                pub fn new(fp: DiyFp) -> Self {
                    Double {
                        value: f64::from_bits(fp.f()),
                    }
                }

                pub fn value(&self) -> f64 {
                    self.value
                }

                pub fn upper_boundary(&self) -> DiyFp {
                    DiyFp::new(self.value.to_bits(), 0)
                }

                pub fn next_double(&self) -> f64 {
                    let bits = self.value.to_bits();
                    let next_bits = if self.value.is_sign_positive() {
                        bits + 1
                    } else {
                        bits - 1
                    };
                    f64::from_bits(next_bits)
                }

                pub fn significand(&self) -> i32 {
                    self.value.to_bits() as i32
                }

                pub fn significand_size_for_order_of_magnitude(order_of_magnitude: i32) -> i32 {
                    order_of_magnitude
                }
            }
        }
        pub mod diy_fp {
            #[derive(Clone, Copy, Debug)]
            pub struct DiyFp {
                f_: u64,
                e_: i32,
            }

            impl DiyFp {
                pub const K_SIGNIFICAND_SIZE: usize = 64;
                pub fn new(f_: u64, e_: i32) -> Self {
                    DiyFp { f_, e_ }
                }

                pub fn f(&self) -> u64 {
                    self.f_
                }

                pub fn e(&self) -> i32 {
                    self.e_
                }

                pub fn set_f(&mut self, f_: u64) {
                    self.f_ = f_;
                }
                pub fn set_e(&mut self, e_: i32) {
                    self.e_ = e_;
                }

                pub fn normalize(&mut self) {
                    while (self.f_ & 0x8000_0000_0000_0000) == 0 {
                        self.f_ <<= 1;
                        self.e_ -= 1;
                    }
                }

                pub fn multiply(&mut self, other: DiyFp) {
                    self.f_ *= other.f_;
                    self.e_ += other.e_;
                }
            }
        }
    }
}

pub mod base_numbers {
    pub use crate::strtod_rs::strtod;
}
