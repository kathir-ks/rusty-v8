pub mod conversions {
    use std::{
        f64,
        i32,
        i64,
        mem,
        u16,
        u32,
        u64,
        usize,
    };

    const V8_INFINITY: f64 = f64::INFINITY;
    const K_MAX_SAFE_INTEGER: f64 = 9007199254740991.0;
    const K_MIN_SAFE_INTEGER: f64 = -9007199254740991.0;
    const K_MAX_INT: f64 = i32::MAX as f64;
    const K_MIN_INT: f64 = i32::MIN as f64;
    const K_MAX_UINT32: f64 = u32::MAX as f64;

    const K_FP64_SIGN_MASK: u64 = 0x8000000000000000;
    const K_FP64_INFINITY: u64 = 0x7FF0000000000000;
    const K_FP16_INFINITY: u16 = 0x7C00;
    const K_FP16_QNAN: u16 = 0x7E00;
    const K_FP64_TO_16_DENORMAL_MAGIC: u64 = 0x4700000000000000;
    const K_FP64_TO_16_REBIAS_EXPONENT_AND_ROUND: u64 = 0x0403000000000000;
    const K_FP64_MANTISSA_BITS: u32 = 52;
    const K_FP16_MANTISSA_BITS: u32 = 10;
    const K_FP16_INFINITY_AND_NAN_INFIMUM: u64 = 0x7c00 << (K_FP64_MANTISSA_BITS - K_FP16_MANTISSA_BITS);
    const K_FP16_DENORMAL_THRESHOLD: u64 = 0x0400 << (K_FP64_MANTISSA_BITS - K_FP16_MANTISSA_BITS);

    pub fn fast_d2ui(x: f64) -> u32 {
        const K2_POW52: f64 = 4503599627370496.0;
        let mut negative = x < 0.0;
        let mut x = x;
        if negative {
            x = -x;
        }
        if x < K2_POW52 {
            x += K2_POW52;
            let result: u32 = unsafe { mem::transmute::<f64, u64>(x) as u32 };
            if negative {
                !result + 1
            } else {
                result
            }
        } else {
            0x80000000
        }
    }

    pub fn double_to_float16(value: f64) -> u16 {
        let mut in_bits: u64 = value.to_bits();
        let mut out: u16 = 0;

        // Take the absolute value of the input.
        let sign: u64 = in_bits & K_FP64_SIGN_MASK;
        in_bits ^= sign;

        if in_bits >= K_FP16_INFINITY_AND_NAN_INFIMUM {
            // Result is infinity or NaN.
            out = if in_bits > K_FP64_INFINITY {
                K_FP16_QNAN // NaN->qNaN
            } else {
                K_FP16_INFINITY // Inf->Inf
            };
        } else {
            // Result is a (de)normalized number or zero.

            if in_bits < K_FP16_DENORMAL_THRESHOLD {
                // Result is a denormal or zero. Use the magic value and FP addition to
                // align 10 mantissa bits at the bottom of the float. Depends on FP
                // addition being round-to-nearest-even.
                let temp = f64::from_bits(in_bits) + f64::from_bits(K_FP64_TO_16_DENORMAL_MAGIC);
                out = (temp.to_bits() - K_FP64_TO_16_DENORMAL_MAGIC) as u16;
            } else {
                // Result is not a denormal.

                // Remember if the result mantissa will be odd before rounding.
                let mant_odd = (in_bits >> (K_FP64_MANTISSA_BITS - K_FP16_MANTISSA_BITS)) & 1;

                // Update the exponent and round to nearest even.
                //
                // Rounding to nearest even is handled in two parts. First, adding
                // kFP64To16RebiasExponentAndRound has the effect of rebiasing the
                // exponent and that if any of the lower 41 bits of the mantissa are set,
                // the 11th mantissa bit from the front becomes set. Second, adding
                // mant_odd ensures ties are rounded to even.
                let mut in_bits = in_bits;
                in_bits += K_FP64_TO_16_REBIAS_EXPONENT_AND_ROUND;
                in_bits += mant_odd;

                out = (in_bits >> (K_FP64_MANTISSA_BITS - K_FP16_MANTISSA_BITS)) as u16;
            }
        }

        out |= (sign >> 48) as u16;
        out
    }

    pub fn double_to_float32(x: f64) -> f32 {
        use std::f32::consts;
        use std::f32::MAX;
        use std::f32::MIN;

        if x > f32::MAX as f64 {
            const K_ROUNDING_THRESHOLD: f64 = 3.4028235677973362e+38;
            if x <= K_ROUNDING_THRESHOLD {
                return MAX;
            }
            return f32::INFINITY;
        }
        if x < f32::MIN as f64 {
            const K_ROUNDING_THRESHOLD: f64 = -3.4028235677973362e+38;
            if x >= K_ROUNDING_THRESHOLD {
                return MIN;
            }
            return f32::NEG_INFINITY;
        }
        x as f32
    }

    pub fn double_to_integer(x: f64) -> f64 {
        if x.is_nan() || x == 0.0 {
            return 0.0;
        }
        if !x.is_finite() {
            return x;
        }
        if x > 0.0 {
            x.floor() + 0.0
        } else {
            x.ceil() + 0.0
        }
    }

    pub fn double_to_i32(x: f64) -> i32 {
        if x.is_finite() && x <= i32::MAX as f64 && x >= i32::MIN as f64 {
            return x as i32;
        }
        let d = Double(x);
        let exponent = d.exponent();
        let bits: u64;
        if exponent < 0 {
            if exponent <= -Double::k_significand_size() as i32 {
                return 0;
            }
            bits = d.significand() >> -exponent;
        } else {
            if exponent > 31 {
                return 0;
            }
            bits = (d.significand() << exponent) & 0xFFFFFFFF;
        }
        (d.sign() as i64 * bits as i64) as i32
    }

    pub fn double_to_webidl_i64(x: f64) -> i64 {
        if x.is_finite() && x <= K_MAX_SAFE_INTEGER && x >= K_MIN_SAFE_INTEGER {
            return x as i64;
        }
        let d = Double(x);
        let exponent = d.exponent();
        let bits: u64;
        if exponent < 0 {
            if exponent <= -Double::k_significand_size() as i32 {
                return 0;
            }
            bits = d.significand() >> -exponent;
        } else {
            if exponent > 63 {
                return 0;
            }
            bits = d.significand() << exponent;
            let bits_int64 = bits as i64;
            if bits_int64 == i64::MIN {
                return bits_int64;
            }
        }
        d.sign() as i64 * bits as i64
    }

    pub fn double_to_webidl_u64(x: f64) -> u64 {
        double_to_webidl_i64(x) as u64
    }

    pub fn double_to_smi_integer(value: f64, smi_int_value: &mut i32) -> bool {
        if !is_smi_double(value) {
            return false;
        }
        *smi_int_value = fast_d2i(value);
        if is_valid_smi(*smi_int_value) {
           return true;
        } else {
            return false;
        }
    }

    fn is_valid_smi(value: i32) -> bool {
        value >= SMI_K_MIN_VALUE && value <= SMI_K_MAX_VALUE
    }

    const SMI_K_MIN_VALUE: i32 = -1073741824;
    const SMI_K_MAX_VALUE: i32 = 1073741823;
   
    fn fast_d2i(x: f64) -> i32 {
        x as i32
    }

    fn fast_i2d(x: i32) -> f64 {
        x as f64
    }
    pub fn is_smi_double(value: f64) -> bool {
        value >= SMI_K_MIN_VALUE as f64
            && value <= SMI_K_MAX_VALUE as f64
            && !is_minus_zero(value)
            && value == fast_i2d(fast_d2i(value))
    }

    pub fn is_int32_double(value: f64) -> bool {
        value >= K_MIN_INT && value <= K_MAX_INT && !is_minus_zero(value) && value == fast_i2d(fast_d2i(value))
    }

    pub fn is_uint32_double(value: f64) -> bool {
        !is_minus_zero(value) && value >= 0.0 && value <= K_MAX_UINT32 && value == fast_ui2d(fast_d2ui(value))
    }

    fn fast_ui2d(x: u32) -> f64 {
        x as f64
    }

    fn is_minus_zero(value: f64) -> bool {
        value == 0.0 && value.is_sign_negative()
    }

    pub fn double_to_uint32_if_equal_to_self(value: f64, uint32_value: &mut u32) -> bool {
        const K2_POW52: f64 = 4503599627370496.0;
        const K_VALID_TOP_BITS: u32 = 0x43300000;
        const K_BOTTOM_BIT_MASK: u64 = 0x00000000FFFFFFFF;

        let shifted_value = value + K2_POW52;

        let result: u64 = shifted_value.to_bits();
        if (result >> 32) as u32 == K_VALID_TOP_BITS {
            *uint32_value = (result & K_BOTTOM_BIT_MASK) as u32;
            return fast_ui2d((result & K_BOTTOM_BIT_MASK) as u32) == value;
        }
        false
    }

    // TODO: Add Tagged<Object>, IsSmi, Cast<HeapNumber> implementations

    // pub fn number_to_i32(number: Tagged<Object>) -> i32 {
    //     if IsSmi(number) {
    //         return Smi::ToInt(number);
    //     }
    //     return double_to_i32(Cast::<HeapNumber>(number).value());
    // }

    // pub fn number_to_u32(number: Tagged<Object>) -> u32 {
    //     if IsSmi(number) {
    //         return Smi::ToInt(number);
    //     }
    //     return double_to_uint32(Cast::<HeapNumber>(number).value());
    // }

    // pub fn positive_number_to_u32(number: Tagged<Object>) -> u32 {
    //     if (number).is_smi() {
    //         let value = number.as_smi_value();
    //         if value <= 0 {
    //             return 0;
    //         }
    //         return value as u32;
    //     }
    //     let value = number.cast_to_heap_number().value();
    //     if !(value >= 1.0) {
    //         return 0;
    //     }
    //     let max = u32::MAX;
    //     if value < max as f64 {
    //         return value as u32;
    //     }
    //     return max;
    // }

    // pub fn number_to_i64(number: Tagged<Object>) -> i64 {
    //     if IsSmi(number) {
    //         return Smi::ToInt(number);
    //     }
    //     let d = Cast::<HeapNumber>(number).value();
    //     if d.is_nan() {
    //         return 0;
    //     }
    //     if d >= i64::MAX as f64 {
    //         return i64::MAX;
    //     }
    //     if d <= i64::MIN as f64 {
    //         return i64::MIN;
    //     }
    //     return d as i64;
    // }

    // pub fn positive_number_to_u64(number: Tagged<Object>) -> u64 {
    //     if IsSmi(number) {
    //         let value = Smi::ToInt(number);
    //         if value <= 0 {
    //             return 0;
    //         }
    //         return value as u64;
    //     }
    //     let value = Cast::<HeapNumber>(number).value();
    //     if !(value >= 1.0) {
    //         return 0;
    //     }
    //     let max = u64::MAX;
    //     if value < max as f64 {
    //         return value as u64;
    //     }
    //     return max;
    // }

    // pub fn try_number_to_size(number: Tagged<Object>, result: &mut usize) -> bool {
    //     if IsSmi(number) {
    //         let value = Smi::ToInt(number);
    //         if value >= 0 {
    //             *result = value as usize;
    //             return true;
    //         }
    //         return false;
    //     } else {
    //         let value = Cast::<HeapNumber>(number).value();
    //         let max_size = usize::MAX as f64;
    //         if value >= 0.0 && value < max_size {
    //             let size = value as usize;
    //             // if size > kMaxSafeBufferSizeForSandbox {  //SANDBOX
    //             //     return false;
    //             // }
    //             *result = size;
    //             return true;
    //         } else {
    //             return false;
    //         }
    //     }
    // }

    // pub fn number_to_size(number: Tagged<Object>) -> usize {
    //     let mut result: usize = 0;
    //     let is_valid = try_number_to_size(number, &mut result);
    //     assert!(is_valid);
    //     return result;
    // }

    pub fn double_to_uint32(x: f64) -> u32 {
        double_to_i32(x) as u32
    }

    struct Double(f64);

    impl Double {
        const K_SIGNIFICAND_SIZE: u32 = 52;

        fn exponent(&self) -> i32 {
            let bits = self.0.to_bits();
            let exponent = ((bits >> Double::K_SIGNIFICAND_SIZE) & 0x7FF) as i32;
            exponent - 1023
        }

        fn significand(&self) -> u64 {
            let bits = self.0.to_bits();
            bits & ((1 << Double::K_SIGNIFICAND_SIZE) - 1)
        }

        fn sign(&self) -> i32 {
            if self.0.is_sign_positive() {
                1
            } else {
                -1
            }
        }

        fn k_significand_size() -> u32 {
            Self::K_SIGNIFICAND_SIZE
        }
    }
}