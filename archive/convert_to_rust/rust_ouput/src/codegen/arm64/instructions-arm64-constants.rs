// Converted from V8 C++ source files:
// Header: N/A
// Implementation: instructions-arm64-constants.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod constants_arm64 {
    use crate::codegen::arm64::instructions_arm64::float16;
    use crate::codegen::arm64::macro_assembler_arm64::V8_EXPORT_PRIVATE;
    use std::mem::transmute;

    pub mod integer_constants {
        pub const K_FP16_POSITIVE_INFINITY: u16 = 0x7C00;
        pub const K_FP16_NEGATIVE_INFINITY: u16 = 0xFC00;
        pub const K_FP32_POSITIVE_INFINITY: u32 = 0x7F800000;
        pub const K_FP32_NEGATIVE_INFINITY: u32 = 0xFF800000;
        pub const K_FP64_POSITIVE_INFINITY: u64 = 0x7FF0000000000000;
        pub const K_FP64_NEGATIVE_INFINITY: u64 = 0xFFF0000000000000;

        // This value is a signalling NaN as both a double and as a float (taking the
        // least-significant word).
        pub const K_FP64_SIGNALLING_NAN: u64 = 0x7FF000007F800001;
        pub const K_FP32_SIGNALLING_NAN: u32 = 0x7F800001;

        // A similar value, but as a quiet NaN.
        pub const K_FP64_QUIET_NAN: u64 = 0x7FF800007FC00001;
        pub const K_FP32_QUIET_NAN: u32 = 0x7FC00001;

        // The default NaN values (for FPCR.DN=1).
        pub const K_FP64_DEFAULT_NAN: u64 = 0x7FF8000000000000;
        pub const K_FP32_DEFAULT_NAN: u32 = 0x7FC00000;
        pub const K_FP16_DEFAULT_NAN: u16 = 0x7E00;
    }

    pub static K_FP16_POSITIVE_INFINITY: float16 =
        unsafe { transmute(integer_constants::K_FP16_POSITIVE_INFINITY) };
    pub static K_FP16_NEGATIVE_INFINITY: float16 =
        unsafe { transmute(integer_constants::K_FP16_NEGATIVE_INFINITY) };
    pub static K_FP32_POSITIVE_INFINITY: f32 =
        unsafe { transmute(integer_constants::K_FP32_POSITIVE_INFINITY) };
    pub static K_FP32_NEGATIVE_INFINITY: f32 =
        unsafe { transmute(integer_constants::K_FP32_NEGATIVE_INFINITY) };
    pub static K_FP64_POSITIVE_INFINITY: f64 =
        unsafe { transmute(integer_constants::K_FP64_POSITIVE_INFINITY) };
    pub static K_FP64_NEGATIVE_INFINITY: f64 =
        unsafe { transmute(integer_constants::K_FP64_NEGATIVE_INFINITY) };

    pub static K_FP64_SIGNALLING_NAN: f64 =
        unsafe { transmute(integer_constants::K_FP64_SIGNALLING_NAN) };
    pub static K_FP32_SIGNALLING_NAN: f32 =
        unsafe { transmute(integer_constants::K_FP32_SIGNALLING_NAN) };

    pub static K_FP64_QUIET_NAN: f64 =
        unsafe { transmute(integer_constants::K_FP64_QUIET_NAN) };
    pub static K_FP32_QUIET_NAN: f32 =
        unsafe { transmute(integer_constants::K_FP32_QUIET_NAN) };

    pub static K_FP64_DEFAULT_NAN: f64 =
        unsafe { transmute(integer_constants::K_FP64_DEFAULT_NAN) };
    pub static K_FP32_DEFAULT_NAN: f32 =
        unsafe { transmute(integer_constants::K_FP32_DEFAULT_NAN) };
    pub static K_FP16_DEFAULT_NAN: float16 =
        unsafe { transmute(integer_constants::K_FP16_DEFAULT_NAN) };
}
