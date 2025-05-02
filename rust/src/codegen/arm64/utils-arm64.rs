// Copyright 2013 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// #![cfg(target_arch = "aarch64")] // Enable only for ARM64 architecture

use std::mem::transmute;

/// Module containing utility functions for ARM64 architecture.
pub mod utils_arm64 {
    use std::mem::transmute;

    const FLOAT_EXPONENT_BITS: u32 = 8;
    const FLOAT_MANTISSA_BITS: u32 = 23;
    const DOUBLE_EXPONENT_BITS: u64 = 11;
    const DOUBLE_MANTISSA_BITS: u64 = 52;
    const FLOAT16_EXPONENT_BITS: u32 = 5;
    const FLOAT16_MANTISSA_BITS: u32 = 10;

    /// Extracts the sign bit from a float.
    pub fn float_sign(val: f32) -> u32 {
        let bits: u32 = unsafe { transmute(val) };
        (bits >> 31) & 1
    }

    /// Extracts the exponent bits from a float.
    pub fn float_exp(val: f32) -> u32 {
        let bits: u32 = unsafe { transmute(val) };
        (bits >> 23) & 0xFF
    }

    /// Extracts the mantissa bits from a float.
    pub fn float_mantissa(val: f32) -> u32 {
        let bits: u32 = unsafe { transmute(val) };
        bits & 0x7FFFFF
    }

    /// Extracts the sign bit from a double.
    pub fn double_sign(val: f64) -> u32 {
        let bits: u64 = unsafe { transmute(val) };
        ((bits >> 63) & 1) as u32
    }

    /// Extracts the exponent bits from a double.
    pub fn double_exp(val: f64) -> u32 {
        let bits: u64 = unsafe { transmute(val) };
        ((bits >> 52) & 0x7FF) as u32
    }

    /// Extracts the mantissa bits from a double.
    pub fn double_mantissa(val: f64) -> u64 {
        let bits: u64 = unsafe { transmute(val) };
        bits & 0xFFFFFFFFFFFFF
    }

    /// Packs sign, exponent, and mantissa bits into a float.
    pub fn float_pack(sign: u32, exp: u32, mantissa: u32) -> f32 {
        let bits = (sign << FLOAT_EXPONENT_BITS) | exp;
        unsafe { transmute::<u32, f32>((bits << FLOAT_MANTISSA_BITS) | mantissa) }
    }

    /// Packs sign, exponent, and mantissa bits into a double.
    pub fn double_pack(sign: u64, exp: u64, mantissa: u64) -> f64 {
        let bits = (sign << DOUBLE_EXPONENT_BITS) | exp;
        unsafe { transmute::<u64, f64>((bits << DOUBLE_MANTISSA_BITS) | mantissa) }
    }

    pub type Float16 = u16;

    pub const FP_ZERO: i32 = 0;
    pub const FP_SUBNORMAL: i32 = 1;
    pub const FP_NORMAL: i32 = 2;
    pub const FP_INFINITE: i32 = 3;
    pub const FP_NAN: i32 = 4;

    /// Classifies a float16 value.
    pub fn float16classify(value: Float16) -> i32 {
        let exponent_max: u16 = (1 << FLOAT16_EXPONENT_BITS) - 1;
        let exponent_mask: u16 = exponent_max << FLOAT16_MANTISSA_BITS;
        let mantissa_mask: u16 = (1 << FLOAT16_MANTISSA_BITS) - 1;

        let exponent: u16 = (value & exponent_mask) >> FLOAT16_MANTISSA_BITS;
        let mantissa: u16 = value & mantissa_mask;
        if exponent == 0 {
            if mantissa == 0 {
                return FP_ZERO;
            }
            return FP_SUBNORMAL;
        } else if exponent == exponent_max {
            if mantissa == 0 {
                return FP_INFINITE;
            }
            return FP_NAN;
        }
        FP_NORMAL
    }

    /// Counts the number of leading sign bits in a value.
    pub fn count_leading_sign_bits(value: i64, width: i32) -> i32 {
        assert!(width.is_power_of_two() && (width <= 64));
        if value >= 0 {
            count_leading_zeros(value as u64, width as u32) as i32 - 1
        } else {
            count_leading_zeros((!value) as u64, width as u32) as i32 - 1
        }
    }

    /// Counts the number of set bits in a value.
    pub fn count_set_bits(value: u64, width: i32) -> i32 {
        assert!((width == 32) || (width == 64));
        if width == 64 {
            value.count_ones() as i32
        } else {
            (value & 0xFFFFFFFF).count_ones() as i32
        }
    }

    /// Finds the position of the lowest set bit in a value.
    pub fn lowest_set_bit_position(value: u64) -> i32 {
        assert_ne!(value, 0);
        value.trailing_zeros() as i32 + 1
    }

    /// Finds the position of the highest set bit in a value.
    pub fn highest_set_bit_position(value: u64) -> i32 {
        assert_ne!(value, 0);
        63 - count_leading_zeros(value, 64) as i32
    }

    /// Converts a mask to a bit position.
    pub fn mask_to_bit(mask: u64) -> i32 {
        assert_eq!(count_set_bits(mask, 64), 1);
        mask.trailing_zeros() as i32
    }

    /// Counts the number of leading zeros in a value.
    fn count_leading_zeros(value: u64, width: u32) -> u32 {
        if width == 64 {
            value.leading_zeros()
        } else {
            (value & ((1u64 << width) - 1)).leading_zeros()
        }
    }
}