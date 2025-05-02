// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//use std::mem;

#[allow(dead_code)]
pub mod constants_arm64 {
    /// ISA constants.
    pub mod integer_constants {
        pub const FP16_POSITIVE_INFINITY: u16 = 0x7C00;
        pub const FP16_NEGATIVE_INFINITY: u16 = 0xFC00;
        pub const FP32_POSITIVE_INFINITY: u32 = 0x7F800000;
        pub const FP32_NEGATIVE_INFINITY: u32 = 0xFF800000;
        pub const FP64_POSITIVE_INFINITY: u64 = 0x7FF0000000000000;
        pub const FP64_NEGATIVE_INFINITY: u64 = 0xFFF0000000000000;

        // This value is a signalling NaN as both a double and as a float (taking the
        // least-significant word).
        pub const FP64_SIGNALLING_NAN: u64 = 0x7FF000007F800001;
        pub const FP32_SIGNALLING_NAN: u32 = 0x7F800001;

        // A similar value, but as a quiet NaN.
        pub const FP64_QUIET_NAN: u64 = 0x7FF800007FC00001;
        pub const FP32_QUIET_NAN: u32 = 0x7FC00001;

        // The default NaN values (for FPCR.DN=1).
        pub const FP64_DEFAULT_NAN: u64 = 0x7FF8000000000000;
        pub const FP32_DEFAULT_NAN: u32 = 0x7FC00000;
        pub const FP16_DEFAULT_NAN: u16 = 0x7E00;
    }

    // The 'float16' type is not available in Rust's standard library.
    // Using u16 as a placeholder for float16 for now, and bitcasting to f16
    // in the getter functions. A proper float16 type will need to be implemented.
    //extern "C" {

    use half::f16;

    #[inline]
    pub fn fp16_positive_infinity() -> f16 {
        f16::from_bits(integer_constants::FP16_POSITIVE_INFINITY)
    }

    #[inline]
    pub fn fp16_negative_infinity() -> f16 {
        f16::from_bits(integer_constants::FP16_NEGATIVE_INFINITY)
    }

    #[inline]
    pub fn fp32_positive_infinity() -> f32 {
        f32::from_bits(integer_constants::FP32_POSITIVE_INFINITY)
    }

    #[inline]
    pub fn fp32_negative_infinity() -> f32 {
        f32::from_bits(integer_constants::FP32_NEGATIVE_INFINITY)
    }

    #[inline]
    pub fn fp64_positive_infinity() -> f64 {
        f64::from_bits(integer_constants::FP64_POSITIVE_INFINITY)
    }

    #[inline]
    pub fn fp64_negative_infinity() -> f64 {
        f64::from_bits(integer_constants::FP64_NEGATIVE_INFINITY)
    }

    #[inline]
    pub fn fp64_signalling_nan() -> f64 {
        f64::from_bits(integer_constants::FP64_SIGNALLING_NAN)
    }

    #[inline]
    pub fn fp32_signalling_nan() -> f32 {
        f32::from_bits(integer_constants::FP32_SIGNALLING_NAN)
    }

    #[inline]
    pub fn fp64_quiet_nan() -> f64 {
        f64::from_bits(integer_constants::FP64_QUIET_NAN)
    }

    #[inline]
    pub fn fp32_quiet_nan() -> f32 {
        f32::from_bits(integer_constants::FP32_QUIET_NAN)
    }

    #[inline]
    pub fn fp64_default_nan() -> f64 {
        f64::from_bits(integer_constants::FP64_DEFAULT_NAN)
    }

    #[inline]
    pub fn fp32_default_nan() -> f32 {
        f32::from_bits(integer_constants::FP32_DEFAULT_NAN)
    }

    #[inline]
    pub fn fp16_default_nan() -> f16 {
        f16::from_bits(integer_constants::FP16_DEFAULT_NAN)
    }
    //} // extern "C"
}