// Converted from V8 C++ source files:
// Header: bignum-dtoa.h
// Implementation: bignum-dtoa.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod numbers {
        use std::cmp::Ordering;

        pub enum BignumDtoaMode {
            BIGNUM_DTOA_SHORTEST,
            BIGNUM_DTOA_FIXED,
            BIGNUM_DTOA_PRECISION,
        }

        pub fn BignumDtoa(
            v: f64,
            mode: BignumDtoaMode,
            requested_digits: i32,
            buffer: &mut [char],
            length: &mut usize,
            decimal_point: &mut i32,
        ) {
            if v <= 0.0 {
                panic!("Input v must be > 0");
            }

            if v.is_nan() || v.is_infinite() {
                panic!("Input v must be finite");
            }

            let significand = Double(v).significand();
            let is_even = (significand & 1) == 0;
            let exponent = Double(v).exponent();
            let normalized_exponent = NormalizedExponent(significand, exponent);
            let estimated_power = EstimatePower(normalized_exponent);

            if let BignumDtoaMode::BIGNUM_DTOA_FIXED = mode {
                if -estimated_power - 1 > requested_digits {
                    buffer[0] = '\0';
                    *length = 0;
                    *decimal_point = -requested_digits;
                    return;
                }
            }

            let mut numerator = Bignum::new();
            let mut denominator = Bignum::new();
            let mut delta_minus = Bignum::new();
            let mut delta_plus = Bignum::new();

            let need_boundary_deltas = matches!(mode, BignumDtoaMode::BIGNUM_DTOA_SHORTEST);

            InitialScaledStartValues(
                v,
                estimated_power,
                need_boundary_deltas,
                &mut numerator,
                &mut denominator,
                &mut delta_minus,
                &mut delta_plus,
            );

            FixupMultiply10(
                estimated_power,
                is_even,
                decimal_point,
                &mut numerator,
                &mut denominator,
                &mut delta_minus,
                &mut delta_plus,
            );

            match mode {
                BignumDtoaMode::BIGNUM_DTOA_SHORTEST => {
                    GenerateShortestDigits(
                        &mut numerator,
                        &mut denominator,
                        &mut delta_minus,
                        &mut delta_plus,
                        is_even,
                        buffer,
                        length,
                    );
                }
                BignumDtoaMode::BIGNUM_DTOA_FIXED => {
                    BignumToFixed(
                        requested_digits,
                        decimal_point,
                        &mut numerator,
                        &mut denominator,
                        buffer,
                        length,
                    );
                }
                BignumDtoaMode::BIGNUM_DTOA_PRECISION => {
                    GenerateCountedDigits(
                        requested_digits,
                        decimal_point,
                        &mut numerator,
                        &mut denominator,
                        buffer,
                        length,
                    );
                }
            }
            if *length < buffer.len() {
                buffer[*length] = '\0';
            }
        }

        fn NormalizedExponent(significand: u64, exponent: i32) -> i32 {
            if significand == 0 {
                panic!("Significand must not be zero");
            }

            let mut s = significand;
            let mut e = exponent;
            while (s & Double::kHiddenBit) == 0 {
                s <<= 1;
                e -= 1;
            }
            e
        }

        fn EstimatePower(exponent: i32) -> i32 {
            const K1_LOG10: f64 = 0.30102999566398114;
            const K_SIGNIFICAND_SIZE: i32 = 53;
            let estimate =
                (exponent as f64 + K_SIGNIFICAND_SIZE as f64 - 1.0) * K1_LOG10 - 1e-10;
            estimate.ceil() as i32
        }

        fn InitialScaledStartValues(
            v: f64,
            estimated_power: i32,
            need_boundary_deltas: bool,
            numerator: &mut Bignum,
            denominator: &mut Bignum,
            delta_minus: &mut Bignum,
            delta_plus: &mut Bignum,
        ) {
            if Double(v).exponent() >= 0 {
                InitialScaledStartValuesPositiveExponent(
                    v,
                    estimated_power,
                    need_boundary_deltas,
                    numerator,
                    denominator,
                    delta_minus,
                    delta_plus,
                );
            } else if estimated_power >= 0 {
                InitialScaledStartValuesNegativeExponentPositivePower(
                    v,
                    estimated_power,
                    need_boundary_deltas,
                    numerator,
                    denominator,
                    delta_minus,
                    delta_plus,
                );
            } else {
                InitialScaledStartValuesNegativeExponentNegativePower(
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

        fn InitialScaledStartValuesPositiveExponent(
            v: f64,
            estimated_power: i32,
            need_boundary_deltas: bool,
            numerator: &mut Bignum,
            denominator: &mut Bignum,
            delta_minus: &mut Bignum,
            delta_plus: &mut Bignum,
        ) {
            if estimated_power < 0 {
                panic!("Estimated power must be positive");
            }

            numerator.assign_u64(Double(v).significand());
            numerator.shift_left(Double(v).exponent());

            denominator.assign_power_u16(10, estimated_power as u16);

            if need_boundary_deltas {
                denominator.shift_left(1);
                numerator.shift_left(1);

                delta_plus.assign_u16(1);
                delta_plus.shift_left(Double(v).exponent());

                delta_minus.assign_u16(1);
                delta_minus.shift_left(Double(v).exponent());

                let v_bits = Double(v).as_u64();
                if (v_bits & Double::kSignificandMask) == 0 {
                    denominator.shift_left(1);
                    numerator.shift_left(1);
                    delta_plus.shift_left(1);
                }
            }
        }

        fn InitialScaledStartValuesNegativeExponentPositivePower(
            v: f64,
            estimated_power: i32,
            need_boundary_deltas: bool,
            numerator: &mut Bignum,
            denominator: &mut Bignum,
            delta_minus: &mut Bignum,
            delta_plus: &mut Bignum,
        ) {
            let significand = Double(v).significand();
            let exponent = Double(v).exponent();

            numerator.assign_u64(significand);
            denominator.assign_power_u16(10, estimated_power as u16);
            denominator.shift_left(-exponent);

            if need_boundary_deltas {
                denominator.shift_left(1);
                numerator.shift_left(1);

                delta_plus.assign_u16(1);
                delta_minus.assign_u16(1);

                let v_bits = Double(v).as_u64();
                if (v_bits & Double::kSignificandMask) == 0 {
                    denominator.shift_left(1);
                    numerator.shift_left(1);
                    delta_plus.shift_left(1);
                }
            }
        }

        fn InitialScaledStartValuesNegativeExponentNegativePower(
            v: f64,
            estimated_power: i32,
            need_boundary_deltas: bool,
            numerator: &mut Bignum,
            denominator: &mut Bignum,
            delta_minus: &mut Bignum,
            delta_plus: &mut Bignum,
        ) {
            const K_MINIMAL_NORMALIZED_EXPONENT: u64 = 0x0010_0000_0000_0000;
            let significand = Double(v).significand();
            let exponent = Double(v).exponent();

            let power_ten = numerator;
            power_ten.assign_power_u16(10, (-estimated_power) as u16);

            if need_boundary_deltas {
                delta_plus.assign_bignum(power_ten);
                delta_minus.assign_bignum(power_ten);
            }

            numerator.multiply_by_u64(significand);

            denominator.assign_u16(1);
            denominator.shift_left(-exponent);

            if need_boundary_deltas {
                numerator.shift_left(1);
                denominator.shift_left(1);

                let v_bits = Double(v).as_u64();
                if (v_bits & Double::kSignificandMask) == 0 &&
                   (v_bits & Double::kExponentMask) != K_MINIMAL_NORMALIZED_EXPONENT
                {
                    numerator.shift_left(1);
                    denominator.shift_left(1);
                    delta_plus.shift_left(1);
                }
            }
        }

        fn FixupMultiply10(
            estimated_power: i32,
            is_even: bool,
            decimal_point: &mut i32,
            numerator: &mut Bignum,
            denominator: &mut Bignum,
            delta_minus: &mut Bignum,
            delta_plus: &mut Bignum,
        ) {
            let in_range = if is_even {
                Bignum::plus_compare(numerator, delta_plus, denominator) >= 0
            } else {
                Bignum::plus_compare(numerator, delta_plus, denominator) > 0
            };

            if in_range {
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

        fn GenerateShortestDigits(
            numerator: &mut Bignum,
            denominator: &mut Bignum,
            delta_minus: &mut Bignum,
            delta_plus: &mut Bignum,
            is_even: bool,
            buffer: &mut [char],
            length: &mut usize,
        ) {
            if Bignum::equal(delta_minus, delta_plus) {
                *delta_plus = *delta_minus;
            }

            *length = 0;
            loop {
                let digit = numerator.divide_modulo_int_bignum(denominator) as u8;
                if digit > 9 {
                    panic!("Digit must be less than or equal to 9");
                }

                buffer[*length] = (digit + b'0') as char;
                *length += 1;

                let in_delta_room_minus = if is_even {
                    Bignum::less_equal(numerator, delta_minus)
                } else {
                    Bignum::less(numerator, delta_minus)
                };

                let in_delta_room_plus = if is_even {
                    Bignum::plus_compare(numerator, delta_plus, denominator) >= 0
                } else {
                    Bignum::plus_compare(numerator, delta_plus, denominator) > 0
                };

                if !in_delta_room_minus && !in_delta_room_plus {
                    numerator.times10();
                    delta_minus.times10();
                    if !Bignum::equal(delta_minus, delta_plus) {
                        delta_plus.times10();
                    }
                } else if in_delta_room_minus && in_delta_room_plus {
                    let compare = Bignum::plus_compare(numerator, numerator, denominator);
                    if compare < 0 {

                    } else if compare > 0 {
                        if buffer[*length - 1] == '9' {
                            panic!("Digit must not be 9");
                        }
                        buffer[*length - 1] = ((buffer[*length - 1] as u8) + 1) as char;
                    } else {
                        if ((buffer[*length - 1] as u8) - b'0') % 2 == 0 {

                        } else {
                            if buffer[*length - 1] == '9' {
                                panic!("Digit must not be 9");
                            }
                            buffer[*length - 1] = ((buffer[*length - 1] as u8) + 1) as char;
                        }
                    }
                    return;
                } else if in_delta_room_minus {
                    return;
                } else {
                    if buffer[*length - 1] == '9' {
                        panic!("Digit must not be 9");
                    }
                    buffer[*length - 1] = ((buffer[*length - 1] as u8) + 1) as char;
                    return;
                }
            }
        }

        fn GenerateCountedDigits(
            count: i32,
            decimal_point: &mut i32,
            numerator: &mut Bignum,
            denominator: &mut Bignum,
            buffer: &mut [char],
            length: &mut usize,
        ) {
            if count < 0 {
                panic!("Count must be non-negative");
            }

            for i in 0..(count - 1) {
                let digit = numerator.divide_modulo_int_bignum(denominator) as u8;
                if digit > 9 {
                    panic!("Digit must be less than or equal to 9");
                }

                buffer[i as usize] = (digit + b'0') as char;
                numerator.times10();
            }

            let mut digit = numerator.divide_modulo_int_bignum(denominator) as u8;
            if Bignum::plus_compare(numerator, numerator, denominator) >= 0 {
                digit += 1;
            }
            buffer[(count - 1) as usize] = (digit + b'0') as char;

            for i in (1..count).rev() {
                if buffer[i as usize] != ('0' as u8 + 10) as char {
                    break;
                }
                buffer[i as usize] = '0';
                buffer[(i - 1) as usize] = ((buffer[(i - 1) as usize] as u8) + 1) as char;
            }

            if buffer[0] == ('0' as u8 + 10) as char {
                buffer[0] = '1';
                *decimal_point += 1;
            }

            *length = count as usize;
        }

        fn BignumToFixed(
            requested_digits: i32,
            decimal_point: &mut i32,
            numerator: &mut Bignum,
            denominator: &mut Bignum,
            buffer: &mut [char],
            length: &mut usize,
        ) {
            if -(*decimal_point) > requested_digits {
                *decimal_point = -requested_digits;
                *length = 0;
                return;
            } else if -(*decimal_point) == requested_digits {
                denominator.times10();
                if Bignum::plus_compare(numerator, numerator, denominator) >= 0 {
                    buffer[0] = '1';
                    *length = 1;
                    *decimal_point += 1;
                } else {
                    *length = 0;
                }
                return;
            } else {
                let needed_digits = (*decimal_point) + requested_digits;
                GenerateCountedDigits(
                    needed_digits,
                    decimal_point,
                    numerator,
                    denominator,
                    buffer,
                    length,
                );
            }
        }

        #[derive(Clone, Copy)]
        pub struct Double(f64);

        impl Double {
            pub const kSignificandSize: i32 = 52;
            pub const kExponentMask: u64 = 0x7FF0000000000000;
            pub const kSignificandMask: u64 = 0x000FFFFFFFFFFFFF;
            pub const kHiddenBit: u64 = 0x0010000000000000;

            pub fn new(value: f64) -> Self {
                Double(value)
            }
            pub fn AsUint64(self) -> u64 {
                self.0.to_bits()
            }

            pub fn Significand(self) -> u64 {
                self.0.to_bits() & Self::kSignificandMask | Self::kHiddenBit
            }

            pub fn exponent(self) -> i32 {
                ((self.0.to_bits() & Self::kExponentMask) >> Self::kSignificandSize) as i32 - 1023
            }

            pub fn Exponent(self) -> i32 {
                ((self.0.to_bits() & Self::kExponentMask) >> Self::kSignificandSize) as i32 - 1023
            }

            pub fn IsSpecial(self) -> bool {
                self.0.is_nan() || self.0.is_infinite()
            }

            pub fn as_u64(self) -> u64 {
                self.0.to_bits()
            }
        }

        impl From<f64> for Double {
            fn from(value: f64) -> Self {
                Double(value)
            }
        }

        #[derive(Clone, Debug)]
        pub struct Bignum {
            digits: Vec<u16>,
            k: i32,
        }

        impl Bignum {
            const K_MAX_SIGNIFICANT_BITS: i32 = 2048;
            const K_DIGIT_BASE: u32 = 10000;
            const K_TEN_K: u32 = 10000;

            pub fn new() -> Self {
                Bignum {
                    digits: Vec::new(),
                    k: 0,
                }
            }

            pub fn equal(a: &Bignum, b: &Bignum) -> bool {
                if a.digits.len() != b.digits.len() {
                    return false;
                }
                for i in 0..a.digits.len() {
                    if a.digits[i] != b.digits[i] {
                        return false;
                    }
                }
                true
            }

            pub fn less_equal(a: &Bignum, b: &Bignum) -> bool {
                match a.digits.len().cmp(&b.digits.len()) {
                    Ordering::Less => return true,
                    Ordering::Greater => return false,
                    Ordering::Equal => {}
                }

                for i in (0..a.digits.len()).rev() {
                    match a.digits[i].cmp(&b.digits[i]) {
                        Ordering::Less => return true,
                        Ordering::Greater => return false,
                        Ordering::Equal => {}
                    }
                }

                true
            }

            pub fn less(a: &Bignum, b: &Bignum) -> bool {
                match a.digits.len().cmp(&b.digits.len()) {
                    Ordering::Less => return true,
                    Ordering::Greater => return false,
                    Ordering::Equal => {}
                }

                for i in (0..a.digits.len()).rev() {
                    match a.digits[i].cmp(&b.digits[i]) {
                        Ordering::Less => return true,
                        Ordering::Greater => return false,
                        Ordering::Equal => {}
                    }
                }

                false
            }

            pub fn plus_compare(a: &Bignum, b: &Bignum, c: &Bignum) -> i32 {
                let mut sum_digits = Vec::new();
                let mut carry: u32 = 0;
                let mut i = 0;
                let mut j = 0;

                while i < a.digits.len() || j < b.digits.len() {
                    let digit_a = if i < a.digits.len() {
                        a.digits[i] as u32
                    } else {
                        0
                    };
                    let digit_b = if j < b.digits.len() {
                        b.digits[j] as u32
                    } else {
                        0
                    };

                    let sum = digit_a + digit_b + carry;
                    sum_digits.push((sum % Self::K_DIGIT_BASE) as u16);
                    carry = sum / Self::K_DIGIT_BASE;

                    i += 1;
                    j += 1;
                }

                if carry > 0 {
                    sum_digits.push(carry as u16);
                }

                match sum_digits.len().cmp(&c.digits.len()) {
                    Ordering::Less => return -1,
                    Ordering::Greater => return 1,
                    Ordering::Equal => {}
                }

                for i in (0..sum_digits.len()).rev() {
                    match sum_digits[i].cmp(&c.digits[i]) {
                        Ordering::Less => return -1,
                        Ordering::Greater => return 1,
                        Ordering::Equal => {}
                    }
                }

                0
            }

            pub fn assign_u64(&mut self, value: u64) {
                self.digits.clear();
                self.k = 0;

                let mut v = value;
                while v > 0 {
                    self.digits.push((v % Self::K_DIGIT_BASE as u64) as u16);
                    v /= Self::K_DIGIT_BASE as u64;
                }
            }

            pub fn assign_u16(&mut self, value: u16) {
                self.digits.clear();
                self.k = 0;
                self.digits.push(value);
            }

            pub fn assign_bignum(&mut self, other: &Bignum) {
                self.digits.clear();
                self.k = other.k;
                for &digit in &other.digits {
                    self.digits.push(digit);
                }
            }

            pub fn assign_power_u16(&mut self, base: u16, exponent: u16) {
                self.digits.clear();
                self.k = 0;

                if exponent == 0 {
                    self.digits.push(1);
                    return;
                }

                self.digits.push(base);

                for _ in 1..exponent {
                    self.times(base as u32);
                }
            }

            fn times(&mut self, multiplier: u32) {
                let mut carry: u32 = 0;
                for i in 0..self.digits.len() {
                    let product = self.digits[i] as u32 * multiplier + carry;
                    self.digits[i] = (product % Self::K_DIGIT_BASE) as u16;
                    carry = product / Self::K_DIGIT_BASE;
                }

                if carry > 0 {
                    while carry > 0 {
                        self.digits.push((carry % Self::K_DIGIT_BASE) as u16);
                        carry /= Self::K_DIGIT_BASE;
                    }
                }
            }

            pub fn shift_left(&mut self, bits: i32) {
                if bits <= 0 {
                    return;
                }
                self.times((1u64 << bits) as u32);
            }

            pub fn divide_modulo_int_bignum(&mut self, divisor: &Bignum) -> u16 {
                let mut remainder = Bignum::new();
                let mut quotient = Bignum::new();

                for i in (0..self.digits.len()).rev() {
                    let mut digit = self.digits[i] as u64;
                    let mut j = 0;
                    while j < remainder.digits.len() {
                        digit *= Self::K_DIGIT_BASE as u64;
                        j += 1;
                    }
                    digit += remainder.digits[i] as u64;
                }

                let mut quotient_digit: u32 = 0;
                for i in (0..self.digits.len()).rev() {
                    let mut digit_u64: u64 = self.digits[i] as u64;
                    for k in 0..quotient.digits.len() {
                        digit_u64 *= Self::K_DIGIT_BASE as u64;
                    }

                }
                0
            }
            
            pub fn kMaxSignificantBits() -> i32 {
                Self::K_MAX_SIGNIFICANT_BITS
            }

            pub fn times10(&mut self) {
                self.times(10);
            }

            pub fn multiply_by_u64(&mut self, value: u64) {
                let mut carry = 0u64;
                for i in 0..self.digits.len() {
                    let product = (self.digits[i] as u64) * value + carry;
                    self.digits[i] = (product % Self::K_DIGIT_BASE as u64) as u16;
                    carry = product / (Self::K_DIGIT_BASE as u64);
                }
                if carry > 0 {
                    let mut temp = carry;
                    while temp > 0 {
                        self.digits.push((temp % (Self::K_DIGIT_BASE as u64)) as u16);
                        temp /= Self::K_DIGIT_BASE as u64;
                    }
                }
            }

            fn assign_power_uint16(&mut self, base: u16, exponent: u16) {
                 self.digits.clear();
                 self.k = 0;

                if exponent == 0 {
                    self.digits.push(1);
                    return;
                }

                self.digits.push(base);

                for _ in 1..exponent {
                    self.times(base as u32);
                }
            }

            pub fn assignUInt16(&mut self, value: u16) {
                self.digits.clear();
                self.k = 0;
                self.digits.push(value);
            }

             fn AssignUInt64(&mut self, value: u64) {
                self.digits.clear();
                self.k = 0;

                let mut v = value;
                while v > 0 {
                    self.digits.push((v % Self::K_DIGIT_BASE as u64) as u16);
                    v /= Self::K_DIGIT_BASE as u64;
                }
             }

             fn AssignBignum(&mut self, other: &Bignum) {
                self.digits.clear();
                self.k = other.k;
                for &digit in &other.digits {
                    self.digits.push(digit);
                }
             }
        }
    }
}
