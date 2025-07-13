// Converted from V8 C++ source files:
// Header: utils-arm64.h
// Implementation: utils-arm64.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod bits {
        pub fn IsPowerOfTwo(x: i32) -> bool {
            x > 0 && (x & (x - 1)) == 0
        }

        pub fn CountLeadingZeros64(x: u64) -> i32 {
            x.leading_zeros() as i32
        }
        pub fn CountTrailingZeros(value: u64) -> usize {
            value.trailing_zeros() as usize
        }

        pub fn CountPopulation(x: u64) -> u32 {
            x.count_ones()
        }

    }
}

pub mod v8 {
    pub mod internal {

        use std::convert::TryInto;

        const kFloatExponentBits: u32 = 8;
        const kFloatMantissaBits: u32 = 23;
        const kDoubleExponentBits: u32 = 11;
        const kDoubleMantissaBits: u32 = 52;
        const kDQuietNanMask: u64 = 0x0008000000000000;
        const kSQuietNanMask: u64 = 0x00400000;
        const kFloat16ExponentBits: u32 = 5;
        const kFloat16MantissaBits: u32 = 10;

        pub type float16 = u16;

        pub fn float_sign(val: f32) -> u32 {
            let bits = val.to_bits();
            (bits >> 31) & 1
        }

        pub fn float_exp(val: f32) -> u32 {
            let bits = val.to_bits();
            (bits >> 23) & 0xFF
        }

        pub fn float_mantissa(val: f32) -> u32 {
            let bits = val.to_bits();
            bits & 0x7FFFFF
        }

        pub fn double_sign(val: f64) -> u32 {
            let bits = val.to_bits();
            ((bits >> 63) & 1) as u32
        }

        pub fn double_exp(val: f64) -> u32 {
            let bits = val.to_bits();
            ((bits >> 52) & 0x7FF) as u32
        }

        pub fn double_mantissa(val: f64) -> u64 {
            let bits = val.to_bits();
            bits & 0xFFFFFFFFFFFFF
        }

        pub fn float_pack(sign: u32, exp: u32, mantissa: u32) -> f32 {
            let bits = (sign << kFloatExponentBits) | exp;
            f32::from_bits((bits << kFloatMantissaBits) | mantissa)
        }

        pub fn double_pack(sign: u64, exp: u64, mantissa: u64) -> f64 {
            let bits = (sign << kDoubleExponentBits) | exp;
            f64::from_bits((bits << kDoubleMantissaBits) | mantissa)
        }

        pub fn float16classify(value: float16) -> i32 {
            const EXPONENT_MAX: u16 = (1 << kFloat16ExponentBits) - 1;
            const EXPONENT_MASK: u16 = EXPONENT_MAX << kFloat16MantissaBits;
            const MANTISSA_MASK: u16 = (1 << kFloat16MantissaBits) - 1;

            let exponent = (value & EXPONENT_MASK) >> kFloat16MantissaBits;
            let mantissa = value & MANTISSA_MASK;

            if exponent == 0 {
                if mantissa == 0 {
                    return 0; // FP_ZERO
                }
                return 4; // FP_SUBNORMAL
            } else if exponent == EXPONENT_MAX {
                if mantissa == 0 {
                    return 2; // FP_INFINITE
                }
                return 1; // FP_NAN
            }
            return 3; // FP_NORMAL
        }

        pub fn CountLeadingZeros(value: u64, width: i32) -> i32 {
            if value == 0 {
                return width;
            }
            base::bits::CountLeadingZeros64(value << (64 - width))
        }

        pub fn CountLeadingSignBits(value: i64, width: i32) -> i32 {
            if value >= 0 {
                CountLeadingZeros(value as u64, width) - 1
            } else {
                CountLeadingZeros((!value) as u64, width) - 1
            }
        }

        pub fn CountSetBits(value: u64, width: i32) -> i32 {
            if width == 64 {
                base::bits::CountPopulation(value) as i32
            } else {
                base::bits::CountPopulation((value & 0xFFFFFFFFF) as u32 as u64) as i32
            }
        }

        pub fn LowestSetBitPosition(value: u64) -> i32 {
            base::bits::CountTrailingZeros(value) as i32 + 1
        }

        pub fn HighestSetBitPosition(value: u64) -> i32 {
            63 - CountLeadingZeros(value, 64)
        }

        pub fn LargestPowerOf2Divisor(value: u64) -> u64 {
            value & ((!value).wrapping_add(1))
        }

        pub fn MaskToBit(mask: u64) -> i32 {
            assert_eq!(CountSetBits(mask, 64), 1);
            base::bits::CountTrailingZeros(mask) as i32
        }

        pub fn ReverseBytes<T>(value: T, block_bytes_log2: i32) -> T
        where
            T: std::fmt::Debug + Copy + Sized,
            <T as TryInto<u64>>::Error: std::fmt::Debug,
        {
            let value_size = std::mem::size_of::<T>();
            assert!(value_size == 4 || value_size == 8);
            assert!((1 << block_bytes_log2) <= value_size);

            let value_u64: u64 = match value_size {
                4 => (value as i32).try_into().unwrap() as u64,
                8 => (value as i64).try_into().unwrap() as u64,
                _ => panic!("Unsupported size"),
            };

            let mut bytes: [u8; 8] = [0; 8];
            let mut mask: u64 = 0xff00000000000000;
            for i in (0..8).rev() {
                bytes[i] = ((value_u64 & mask) >> (i * 8)) as u8;
                mask >>= 8;
            }

            const PERMUTE_TABLE: [[[u8; 8]; 3]] = [
                [[6, 7, 4, 5, 2, 3, 0, 1], [6, 7, 4, 5, 2, 3, 0, 1], [6, 7, 4, 5, 2, 3, 0, 1]],
                [[4, 5, 6, 7, 0, 1, 2, 3], [4, 5, 6, 7, 0, 1, 2, 3], [4, 5, 6, 7, 0, 1, 2, 3]],
                [[0, 1, 2, 3, 4, 5, 6, 7], [0, 1, 2, 3, 4, 5, 6, 7], [0, 1, 2, 3, 4, 5, 6, 7]],
            ];

            assert!(0 < block_bytes_log2 && block_bytes_log2 < 4);
            let permute_table = &PERMUTE_TABLE[block_bytes_log2 as usize - 1];
            let mut result: u64 = 0;

            for i in 0..8 {
                result <<= 8;
                result |= bytes[permute_table[0][i] as usize] as u64;
            }

            match value_size {
                4 => result as i32 as T,
                8 => result as i64 as T,
                _ => panic!("Unsupported size"),
            }
        }

        pub fn IsSignallingNaN(num: f64) -> bool {
            if num.is_nan() {
                let raw = num.to_bits();
                (raw & kDQuietNanMask) == 0
            } else {
                false
            }
        }

        pub fn IsSignallingNaN_float(num: f32) -> bool {
            if num.is_nan() {
                let raw = num.to_bits();
                ((raw as u64) & kSQuietNanMask) == 0
            } else {
                false
            }
        }

        pub fn IsSignallingNaN_float16(num: float16) -> bool {
            const K_FP16_QUIET_NAN_MASK: u16 = 0x0200;
            float16classify(num) == 1 && ((num & K_FP16_QUIET_NAN_MASK) == 0)
        }

        pub fn IsQuietNaN<T: std::fmt::Debug>(num: T) -> bool
        where
            T: std::fmt::Debug,
        {
            match std::any::TypeId::of::<T>() {
                x if x == std::any::TypeId::of::<f64>() => {
                    let num = unsafe { std::mem::transmute_copy::<T, f64>(&num) };
                    num.is_nan() && !IsSignallingNaN(num)
                }
                x if x == std::any::TypeId::of::<f32>() => {
                    let num = unsafe { std::mem::transmute_copy::<T, f32>(&num) };
                    num.is_nan() && !IsSignallingNaN_float(num)
                }
                _ => {
                    eprintln!("Type {:?} is not supported by IsQuietNaN, returning false", std::any::TypeId::of::<T>());
                    false
                }
            }
        }

        pub fn ToQuietNaN(num: f64) -> f64 {
            assert!(num.is_nan());
            f64::from_bits(num.to_bits() | kDQuietNanMask)
        }

        pub fn ToQuietNaN_float(num: f32) -> f32 {
            assert!(num.is_nan());
            f32::from_bits(num.to_bits() | kSQuietNanMask as u32)
        }

        pub fn FusedMultiplyAdd(op1: f64, op2: f64, a: f64) -> f64 {
            op1.mul_add(op2, a)
        }

        pub fn FusedMultiplyAdd_float(op1: f32, op2: f32, a: f32) -> f32 {
            op1.mul_add(op2, a)
        }

        fn unsigned_bitextract_32(high: u32, low: u32, bits: u32) -> u32 {
            let mask = (1 << (high - low + 1)) - 1;
            (bits >> low) & mask
        }

        fn unsigned_bitextract_64(high: u32, low: u32, bits: u64) -> u64 {
            let mask = (1 << (high - low + 1)) - 1;
            (bits >> low) & mask
        }
    }
}
