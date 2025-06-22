// Copyright 2011 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod conversions {
    use std::string::String;
    use std::{f32, f64};
    use std::{fmt, mem};
    use std::num::ParseFloatError;
    use std::ptr;
    use std::borrow::Cow;

    //use crate::bigint::BigInt; // Assuming BigInt is defined in bigint.rs
    //use crate::isolate::Isolate; // Assuming Isolate is defined in isolate.rs

    // uint64_t constants prefixed with kFP64 are bit patterns of doubles.
    // uint64_t constants prefixed with kFP16 are bit patterns of doubles encoding
    // limits of half-precision floating point values.
    pub const K_FP64_EXPONENT_BITS: i32 = 11;
    pub const K_FP64_MANTISSA_BITS: i32 = 52;
    pub const K_FP64_EXPONENT_BIAS: u64 = 1023;
    pub const K_FP64_SIGN_MASK: u64 = 1u64 << (K_FP64_EXPONENT_BITS + K_FP64_MANTISSA_BITS);
    pub const K_FP64_INFINITY: u64 = 2047u64 << K_FP64_MANTISSA_BITS;
    pub const K_FP16_INFINITY_AND_NAN_INFIMUM: u64 = (K_FP64_EXPONENT_BIAS + 16) << K_FP64_MANTISSA_BITS;
    pub const K_FP16_MIN_EXPONENT: u64 = K_FP64_EXPONENT_BIAS - 14;
    pub const K_FP16_DENORMAL_THRESHOLD: u64 = K_FP16_MIN_EXPONENT << K_FP64_MANTISSA_BITS;

    pub const K_FP16_MANTISSA_BITS: i32 = 10;
    pub const K_FP16_Q_NAN: u16 = 0x7e00;
    pub const K_FP16_INFINITY: u16 = 0x7c00;

    // A value that, when added, has the effect that if any of the lower 41 bits of
    // the mantissa are set, the 11th mantissa bit from the front becomes set. Used
    // for rounding when converting from double to half-precision.
    pub const K_FP64_TO16_ROUNDING_ADDEND: u64 =
        (1u64 << ((K_FP64_MANTISSA_BITS - K_FP16_MANTISSA_BITS) - 1)) - 1;
    // A value that, when added, rebiases the exponent of a double to the range of
    // the half precision and performs rounding as described above in
    // kFP64To16RoundingAddend. Note that 15-kFP64ExponentBias overflows into the
    // sign bit, but that bit is implicitly cut off when assigning the 64-bit double
    // to a 16-bit output.
    pub const K_FP64_TO16_REBIAS_EXPONENT_AND_ROUND: u64 =
        ((15u64 - K_FP64_EXPONENT_BIAS) << K_FP64_MANTISSA_BITS) +
        K_FP64_TO16_ROUNDING_ADDEND;
    // A magic value that aligns 10 mantissa bits at the bottom of the double when
    // added to a double using floating point addition. Depends on floating point
    // addition being round-to-nearest-even.
    pub const K_FP64_TO16_DENORMAL_MAGIC: u64 =
        (K_FP16_MIN_EXPONENT + (K_FP64_MANTISSA_BITS - K_FP16_MANTISSA_BITS))
        << K_FP64_MANTISSA_BITS;

    pub const K_FP32_WITHOUT_SIGN_MASK: u32 = 0x7fffffff;
    pub const K_FP32_MIN_FP16_ZERO_REPRESENTABLE: u32 = 0x33000000;
    pub const K_FP32_MAX_FP16_REPRESENTABLE: u32 = 0x47800000;
    pub const K_FP32_SUBNORMAL_THRESHOLD_OF_FP16: u32 = 0x38800000;

    // The limit for the the fractionDigits/precision for toFixed, toPrecision
    // and toExponential.
    pub const K_MAX_FRACTION_DIGITS: i32 = 100;
    pub const K_DOUBLE_TO_FIXED_MAX_DIGITS_BEFORE_POINT: i32 = 21;
    // Leave room in the result for appending a minus and a period.
    pub const K_DOUBLE_TO_FIXED_MAX_CHARS: i32 =
        K_DOUBLE_TO_FIXED_MAX_DIGITS_BEFORE_POINT + K_MAX_FRACTION_DIGITS + 2;
    // Leave room in the result for appending a minus, for a period, up to 5 zeros
    // padding after the period and a zero in front of the period.
    pub const K_DOUBLE_TO_PRECISION_MAX_CHARS: i32 = K_MAX_FRACTION_DIGITS + 8;
    // Leave room in the result for one digit before the period, a minus, a period,
    // the letter 'e', a minus or a plus depending on the exponent, and a three
    // digit exponent.
    pub const K_DOUBLE_TO_EXPONENTIAL_MAX_CHARS: i32 = K_MAX_FRACTION_DIGITS + 8;
    // The algorithm starts with the decimal point in the middle and writes to the
    // left for the integer part and to the right for the fractional part.
    // 1024 characters for the exponent and 52 for the mantissa either way, with
    // additional space for sign and decimal point.
    pub const K_DOUBLE_TO_RADIX_MAX_CHARS: i32 = 2200;

    // The fast double-to-(unsigned-)int conversion routine does not guarantee
    // rounding towards zero.
    // If x is NaN, the result is i32::MIN. Otherwise the result is the argument x,
    // clamped to [i32::MIN, i32::MAX] and then rounded to an integer.
    pub fn fast_d2i_checked(x: f64) -> i32 {
        if !(x >= i32::MIN as f64) { return i32::MIN; }  // Negation to catch NaNs.
        if x > i32::MAX as f64 { return i32::MAX; }
        x as i32
    }

    // The fast double-to-(unsigned-)int conversion routine does not guarantee
    // rounding towards zero.
    // The result is undefined if x is infinite or NaN, or if the rounded
    // integer value is outside the range of type int.
    pub fn fast_d2i(x: f64) -> i32 {
        //DCHECK(x <= INT_MAX);
        //DCHECK(x >= INT_MIN);
        x as i32
    }

    pub fn fast_d2ui(x: f64) -> u32 {
        x as u32
    }

    pub fn fast_i2d(x: i32) -> f64 {
        // There is no rounding involved in converting an integer to a
        // double, so this code should compile to a few instructions without
        // any FPU pipeline stalls.
        x as f64
    }

    pub fn fast_ui2d(x: u32) -> f64 {
        // There is no rounding involved in converting an unsigned integer to a
        // double, so this code should compile to a few instructions without
        // any FPU pipeline stalls.
        x as f64
    }

    // This function should match the exact semantics of ECMA-262 20.2.2.17.
    pub fn double_to_float32(x: f64) -> f32 {
        x as f32
    }

    pub fn double_to_float32_no_inline(x: f64) -> f32 {
        x as f32
    }

    // This function should match the exact semantics of truncating x to
    // IEEE 754-2019 binary16 format using roundTiesToEven mode.
    pub fn double_to_float16(x: f64) -> u16 {
        // Implementation detail needed
        let bits: u64 = x.to_bits();

        let rebiased_bits = bits.wrapping_add(K_FP64_TO16_REBIAS_EXPONENT_AND_ROUND);
        rebiased_bits as u16
    }

    // This function should match the exact semantics of ECMA-262 9.4.
    pub fn double_to_integer(x: f64) -> f64 {
        if x.is_nan() {
            return f64::NAN;
        }
        if x.is_infinite() {
            return x;
        }
        x.trunc()
    }

    // This function should match the exact semantics of ECMA-262 9.5.
    pub fn double_to_int32(x: f64) -> i32 {
        if x.is_nan() {
            return 0;
        }
        let integer = double_to_integer(x);

        if integer.is_infinite() {
            if integer.is_sign_positive() {
                return (i32::MAX) as i32;
            } else {
                return (i32::MIN) as i32;
            }
        }

        if integer > i32::MAX as f64 {
            return (i32::MAX) as i32;
        } else if integer < i32::MIN as f64 {
            return (i32::MIN) as i32;
        }
        integer as i32
    }

    pub fn double_to_int32_no_inline(x: f64) -> i32 {
        double_to_int32(x)
    }

    // This function should match the exact semantics of ECMA-262 9.6.
    pub fn double_to_uint32(x: f64) -> u32 {
        if x.is_nan() {
            return 0;
        }
        let integer = double_to_integer(x);
        if integer.is_infinite() {
            return (u32::MAX) as u32;
        }

        if integer > u32::MAX as f64 {
            return (u32::MAX) as u32;
        } else if integer < 0.0 {
            return 0;
        }
        integer as u32
    }

    // These functions have similar semantics as the ones above, but are
    // added for 64-bit integer types.
    pub fn double_to_int64(x: f64) -> i64 {
         if x.is_nan() {
            return 0;
        }
        let integer = double_to_integer(x);

        if integer.is_infinite() {
            if integer.is_sign_positive() {
                return (i64::MAX) as i64;
            } else {
                return (i64::MIN) as i64;
            }
        }

        if integer > i64::MAX as f64 {
            return (i64::MAX) as i64;
        } else if integer < i64::MIN as f64 {
            return (i64::MIN) as i64;
        }
        integer as i64
    }

    pub fn double_to_uint64(x: f64) -> u64 {
         if x.is_nan() {
            return 0;
        }
        let integer = double_to_integer(x);
        if integer.is_infinite() {
            return (u64::MAX) as u64;
        }

        if integer > u64::MAX as f64 {
            return (u64::MAX) as u64;
        } else if integer < 0.0 {
            return 0;
        }
        integer as u64
    }

    // Enumeration for allowing radix prefixes or ignoring junk when converting
    // strings to numbers. We never need to be able to allow both.
    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum ConversionFlag {
        NoConversionFlag,
        AllowNonDecimalPrefix,
        AllowTrailingJunk,
    }

    impl Default for ConversionFlag {
        fn default() -> Self {
            ConversionFlag::NoConversionFlag
        }
    }

    // Converts a string into a double value according to ECMA-262 9.3.1
    pub fn string_to_double(str: &[u8], flag: ConversionFlag, empty_string_val: f64) -> f64 {
        if str.is_empty() {
            return empty_string_val;
        }

        let s = String::from_utf8_lossy(str);
        string_to_double_utf16(&s, flag, empty_string_val)
    }

    pub fn string_to_double_utf16(str: &str, flag: ConversionFlag, empty_string_val: f64) -> f64 {
        if str.is_empty() {
            return empty_string_val;
        }

        let mut s = str.trim();

        if s.is_empty() {
            return empty_string_val;
        }

        let (radix, s) = match flag {
            ConversionFlag::AllowNonDecimalPrefix => {
                if s.starts_with("0x") || s.starts_with("0X") {
                    (16, &s[2..])
                } else if s.starts_with("0o") || s.starts_with("0O") {
                    (8, &s[2..])
                } else if s.starts_with("0b") || s.starts_with("0B") {
                    (2, &s[2..])
                } else {
                    (10, s)
                }
            }
            _ => (10, s),
        };

        let result = if radix == 10 {
            s.parse::<f64>()
        } else {
            let without_junk = match flag {
                ConversionFlag::AllowTrailingJunk => {
                    let mut end = s.len();
                    for (i, c) in s.chars().enumerate().rev() {
                        let valid = match radix {
                            2 => c >= '0' && c <= '1',
                            8 => c >= '0' && c <= '7',
                            16 => c.is_digit(16),
                            _ => true, // Should not happen
                        };
                        if !valid {
                            end = i + 1;
                            break;
                        }
                    }
                    &s[..end]
                }
                _ => s
            };

            let value = u64::from_str_radix(without_junk, radix)
                        .map(|x| x as f64);
            match value {
                Ok(v) => Ok(v),
                Err(_) => Err(ParseFloatError::Invalid)
            }

        };

        match result {
            Ok(v) => v,
            Err(_) => {
                if s.to_lowercase() == "infinity" || s == "+infinity" {
                    f64::INFINITY
                } else if s == "-infinity" {
                    f64::NEG_INFINITY
                } else if s == "nan" {
                    f64::NAN
                } else if flag == ConversionFlag::AllowTrailingJunk {
                    f64::NAN
                } else {
                    f64::NAN
                }
            }
        }
    }

    // This version expects a zero-terminated character array.
    pub fn string_to_double_cstr(str: &str, flag: ConversionFlag, empty_string_val: f64) -> f64 {
        string_to_double(str.as_bytes(), flag, empty_string_val)
    }

    // Converts a binary string (of the form `0b[0-1]*`) into a double value
    // according to https://tc39.es/ecma262/#sec-numericvalue
    pub fn binary_string_to_double(str: &[u8]) -> f64 {
        let s = String::from_utf8_lossy(str);
        if s.starts_with("0b") || s.starts_with("0B") {
            let without_prefix = &s[2..];
            if without_prefix.is_empty() {
                return f64::NAN;
            }
            match u64::from_str_radix(without_prefix, 2) {
                Ok(value) => value as f64,
                Err(_) => f64::NAN,
            }
        } else {
            f64::NAN
        }
    }

    // Converts an octal string (of the form `0o[0-8]*`) into a double value
    // according to https://tc39.es/ecma262/#sec-numericvalue
    pub fn octal_string_to_double(str: &[u8]) -> f64 {
        let s = String::from_utf8_lossy(str);
        if s.starts_with("0o") || s.starts_with("0O") {
            let without_prefix = &s[2..];
            if without_prefix.is_empty() {
                return f64::NAN;
            }
            match u64::from_str_radix(without_prefix, 8) {
                Ok(value) => value as f64,
                Err(_) => f64::NAN,
            }
        } else {
            f64::NAN
        }
    }

    // Converts a hex string (of the form `0x[0-9a-f]*`) into a double value
    // according to https://tc39.es/ecma262/#sec-numericvalue
    pub fn hex_string_to_double(str: &[u8]) -> f64 {
        let s = String::from_utf8_lossy(str);
        if s.starts_with("0x") || s.starts_with("0X") {
            let without_prefix = &s[2..];
            if without_prefix.is_empty() {
                return f64::NAN;
            }
            match u64::from_str_radix(without_prefix, 16) {
                Ok(value) => value as f64,
                Err(_) => f64::NAN,
            }
        } else {
            f64::NAN
        }
    }

    // Converts an implicit octal string (a.k.a. LegacyOctalIntegerLiteral, of the
    // form `0[0-7]*`) into a double value according to
    // https://tc39.es/ecma262/#sec-numericvalue
    pub fn implicit_octal_string_to_double(str: &[u8]) -> f64 {
        let s = String::from_utf8_lossy(str);
        if s.starts_with('0') && s.len() > 1 {
            match u64::from_str_radix(&s[1..], 8) {
                Ok(value) => value as f64,
                Err(_) => f64::NAN,
            }
        } else {
            f64::NAN
        }
    }

    // TODO: Implement StringToInt
    // pub fn string_to_int(isolate: &Isolate, string: &String, radix: i32) -> f64 {
    //     unimplemented!()
    // }

    // TODO: Implement StringToBigInt
    // pub fn string_to_bigint(isolate: &Isolate, string: &String) -> Option<BigInt> {
    //     unimplemented!()
    // }

    // TODO: Implement BigIntLiteral
    // pub fn bigint_literal<T>(isolate: &T, string: &str) -> Option<BigInt> {
    //     unimplemented!()
    // }

    pub const K_DOUBLE_TO_STRING_MIN_BUFFER_SIZE: i32 = 100;

    // Converts a double to a string value according to ECMA-262 9.8.1.
    // The buffer should be large enough for any floating point number.
    // 100 characters is enough.
    // Note: The returned string_view is not necessarily pointing inside the
    // provided buffer.
    pub fn double_to_string_view(value: f64, buffer: &mut [char]) -> String {
        // Implementation detail needed
        format!("{}", value)
    }

    // TODO: Implement BigIntLiteralToDecimal
    // pub fn bigint_literal_to_decimal(isolate: &Isolate, literal: &[u8]) -> Vec<u8> {
    //     unimplemented!()
    // }

    // Convert an int to string value. The returned string is located inside the
    // buffer, but not necessarily at the start.
    pub fn int_to_string_view(n: i32, buffer: &mut [char]) -> String {
        // Implementation detail needed
        format!("{}", n)
    }

    // Additional number to string conversions for the number type.
    pub fn double_to_fixed_string_view(value: f64, f: i32, buffer: &mut [char]) -> String {
        // Implementation detail needed
        format!("{:.f$}", value, f = f as usize)
    }

    pub fn double_to_exponential_string_view(value: f64, f: i32, buffer: &mut [char]) -> String {
        // Implementation detail needed
        format!("{:e}", value)
    }

    pub fn double_to_precision_string_view(value: f64, f: i32, buffer: &mut [char]) -> String {
        // Implementation detail needed
        format!("{:.p$}", value, p = f as usize)
    }

    pub fn double_to_radix_string_view(value: f64, radix: i32, buffer: &mut [char]) -> String {
        // Implementation detail needed
        format!("{}", value) // Placeholder: Radix conversion not directly supported in Rust format
    }

    pub fn is_minus_zero(value: f64) -> bool {
        value.to_bits() == (-0.0f64).to_bits()
    }

    // Returns true if value can be converted to a SMI, and returns the resulting
    // integer value of the SMI in |smi_int_value|.
    pub fn double_to_smi_integer(value: f64, smi_int_value: &mut i32) -> bool {
        if value > ((1 << 30) - 1) as f64 || value < -(1 << 30) as f64 {
            return false;
        }
        let int_value = value as i32;
        if value == int_value as f64 {
            *smi_int_value = int_value;
            true
        } else {
            false
        }
    }

    pub fn is_smi_double(value: f64) -> bool {
        let mut _unused: i32 = 0;
        double_to_smi_integer(value, &mut _unused)
    }

    // Integer32 is an integer that can be represented as a signed 32-bit
    // integer. It has to be in the range [-2^31, 2^31 - 1].
    // We also have to check for negative 0 as it is not an Integer32.
    pub fn is_int32_double(value: f64) -> bool {
        value >= i32::MIN as f64 && value <= i32::MAX as f64 && value.fract() == 0.0 && !is_minus_zero(value)
    }

    // UInteger32 is an integer that can be represented as an unsigned 32-bit
    // integer. It has to be in the range [0, 2^32 - 1].
    // We also have to check for negative 0 as it is not a UInteger32.
    pub fn is_uint32_double(value: f64) -> bool {
        value >= 0.0 && value <= u32::MAX as f64 && value.fract() == 0.0 && !is_minus_zero(value)
    }

    // Tries to convert |value| to a uint32, setting the result in |uint32_value|.
    // If the output does not compare equal to the input, returns false and the
    // value in |uint32_value| is left unspecified.
    // Used for conversions such as in ECMA-262 15.4.2.2, which check "ToUint32(len)
    // is equal to len".
    pub fn double_to_uint32_if_equal_to_self(value: f64, uint32_value: &mut u32) -> bool {
        let converted = double_to_uint32(value) as f64;
        if converted == value {
            *uint32_value = converted as u32;
            true
        } else {
            false
        }
    }

    // TODO: Implement PositiveNumberToUint32
    // Convert from Number object to C integer.
    // pub fn positive_number_to_uint32(number: &Object) -> u32 {
    //     unimplemented!()
    // }

    // TODO: Implement NumberToInt32
    // pub fn number_to_int32(number: &Object) -> i32 {
    //     unimplemented!()
    // }

    // TODO: Implement NumberToUint32
    // pub fn number_to_uint32(number: &Object) -> u32 {
    //     unimplemented!()
    // }

    // TODO: Implement NumberToInt64
    // pub fn number_to_int64(number: &Object) -> i64 {
    //     unimplemented!()
    // }

    // TODO: Implement PositiveNumberToUint64
    // pub fn positive_number_to_uint64(number: &Object) -> u64 {
    //     unimplemented!()
    // }

    // TODO: Implement StringToDouble (with isolate and string)
    // pub fn string_to_double_with_isolate(isolate: &Isolate, string: &String, flags: ConversionFlag, empty_string_val: f64) -> f64 {
    //     unimplemented!()
    // }

    // TODO: Implement FlatStringToDouble
    // pub fn flat_string_to_double(string: &String, flags: ConversionFlag, empty_string_val: f64) -> f64 {
    //     unimplemented!()
    // }

    // String to double helper without heap allocation.
    // Returns None if the string is longer than
    // {max_length_for_conversion}. 23 was chosen because any representable double
    // can be represented using a string of length 23.
    pub fn try_string_to_double(isolate: &(), object: &str, max_length_for_conversion: u32) -> Option<f64> {
        if object.len() > max_length_for_conversion as usize {
            return None;
        }
        match object.parse::<f64>() {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }

    // Return None if the string is longer than 20.
    pub fn try_string_to_int(isolate: &(), object: &str, radix: i32) -> Option<f64> {
        if object.len() > 20 {
            return None;
        }
        match i64::from_str_radix(object, radix as u32) {
            Ok(value) => Some(value as f64),
            Err(_) => None,
        }
    }

    // TODO: Implement TryNumberToSize
    // pub fn try_number_to_size(number: &Object, result: &mut usize) -> bool {
    //     unimplemented!()
    // }

    // TODO: Implement NumberToSize
    // Converts a number into size_t.
    // pub fn number_to_size(number: &Object) -> usize {
    //     unimplemented!()
    // }

     // TODO: Implement IsSpecialIndex (with SharedStringAccessGuardIfNeeded)
    // pub fn is_special_index_with_guard(string: &String, access_guard: &SharedStringAccessGuardIfNeeded) -> bool {
    //     unimplemented!()
    // }

    // TODO: Implement IsSpecialIndex (without SharedStringAccessGuardIfNeeded)
    // pub fn is_special_index(string: &String) -> bool {
    //     unimplemented!()
    // }
}