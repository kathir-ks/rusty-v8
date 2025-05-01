// Implementation of ARM64 simulator logic in Rust.

//use std::fmt;
//use std::fmt::Debug;
use std::convert::TryInto;

//mod fp16; // Assuming fp16 crate handles fp16 functionality

//use crate::simulator::FPRounding;

//use std::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr, Add, Sub};

//use num::traits::{Zero, One, PrimInt, Signed, Unsigned};
//use std::mem::size_of;

//use num_traits::ops::checked::{CheckedAdd, CheckedSub, CheckedMul, CheckedShl, CheckedShr};
//use std::num::Wrapping;
//use std::cmp::{min, max};
//use std::ops::Neg;

//use std::intrinsics::size_of;

//use std::cmp::{max, min};

//use bitintr::*;

//mod base {
//    pub fn bit_cast<T, U>(value: T) -> U {
//        unsafe { std::mem::transmute_copy(&value) }
//    }
//}

const kFP16DefaultNaN: u16 = 0x7e00;
const kHQuietNanMask: u16 = 0x0200;

//const kDoubleExponentBits: usize = 11;
//const kDoubleMantissaBits: usize = 52;
//const kFloatExponentBits: usize = 8;
//const kFloatMantissaBits: usize = 23;
//const kFloat16ExponentBits: usize = 5;
//const kFloat16MantissaBits: usize = 10;

//const kFP32PositiveInfinity: f32 = f32::INFINITY;
//const kFP32NegativeInfinity: f32 = f32::NEG_INFINITY;
//const kFP32DefaultNaN: f32 = f32::NAN;
//const kFP64DefaultNaN: f64 = f64::NAN;

//trait FPRoundingTrait {
//    fn fp_default_nan<T>() -> T;
//}

// Implementations will be in Simulator.rs file.

// Implementations will be in Simulator.rs file.
// Implementations will be in Simulator.rs file.
// Implementations will be in Simulator.rs file.
// Implementations will be in Simulator.rs file.
// Implementations will be in Simulator.rs file.
// Implementations will be in Simulator.rs file.

#[derive(Clone, Copy)]
pub struct half {
    bits_: u16,
}

impl half {
    pub fn new() -> Self {
        half { bits_: 0 }
    }

    pub fn from_f32(f: f32) -> Self {
        half { bits_: fp16::from_f32(f).bits() }
    }

    pub fn from_f64(d: f64) -> Self {
        half { bits_: double_to_float16(d) }
    }

    pub fn from_u16(b: u16) -> Self {
        half { bits_: b }
    }

    pub fn bits(&self) -> u16 {
        self.bits_
    }
}

impl From<half> for f32 {
    fn from(h: half) -> Self {
        fp16::to_f32(fp16::FP16 { bits: h.bits_ })
    }
}

//impl Simulator {
//    pub fn fp_default_nan<T>() -> T {
//        // Placeholder, implementations will be added to Simulator.rs
//        unimplemented!()
//    }
//}

// Will be implemented in Simulator.rs.
// Will be implemented in Simulator.rs.

fn to_quiet_nan(num: half) -> half {
    half {
        bits_: (num.bits() | kHQuietNanMask),
    }
}

// Will be implemented in Simulator.rs.
// Will be implemented in Simulator.rs.

fn is_normal_f32(f: f32) -> bool {
    f.is_normal()
}

fn is_normal_f64(f: f64) -> bool {
    f.is_normal()
}

fn is_normal_half(f: half) -> bool {
    fp16::classify(fp16::FP16 { bits: f.bits() }) == fp16::FpClass::Normal
}

// Implementations will be in Simulator.rs file.
// Implementations will be in Simulator.rs file.
// Implementations will be in Simulator.rs file.
// Implementations will be in Simulator.rs file.
// Implementations will be in Simulator.rs file.

// Implementations will be in Simulator.rs file.
// Implementations will be in Simulator.rs file.

fn copysign_f64(a: f64, f: f64) -> f64 {
    a.copysign(f)
}

fn copysign_f32(a: f64, f: f32) -> f32 {
    a.copysign(f as f64) as f32
}

fn copysign_half(a: f64, f: half) -> half {
    half::from_f32(a.copysign(f32::from(f) as f64) as f32)
}

// Implementations will be in Simulator.rs file.
// Implementations will be in Simulator.rs file.
// Implementations will be in Simulator.rs file.
// Implementations will be in Simulator.rs file.
// Implementations will be in Simulator.rs file.

//static_assert(size_of::<half>() == size_of::<u16>(), "Half must be 16 bit");

//enum FPRounding {
//    NearestEven,
//    Zero,
//    Up,
//    Down,
//    Odd
//}

mod float_utils {
    pub fn double_pack(sign: u64, exponent: u64, mantissa: u64) -> f64 {
        let bits = (sign << 63) | (exponent << 52) | mantissa;
        f64::from_bits(bits)
    }
}

// Implementations will be in Simulator.rs file.

//enum VectorFormat {
//    Undefined,
//    B,
//    H,
//    S,
//    D,
//    Q,
//    Format2D,
//    Format4S,
//    Format8H,
//    Format8B,
//    Format16B
//}

//impl VectorFormat {
//    fn half_width(&self) -> Self {
//        match self {
//            VectorFormat::H => VectorFormat::B,
//            VectorFormat::S => VectorFormat::H,
//            VectorFormat::D => VectorFormat::S,
//            _ => VectorFormat::Undefined
//        }
//    }

//    fn double_width(&self) -> Self {
//        match self {
//            VectorFormat::B => VectorFormat::H,
//            VectorFormat::H => VectorFormat::S,
//            VectorFormat::S => VectorFormat::D,
//            _ => VectorFormat::Undefined
//        }
//    }

//    fn scalar_format(&self) -> Self {
//        match self {
//            VectorFormat::B => VectorFormat::B,
//            VectorFormat::H => VectorFormat::H,
//            VectorFormat::S => VectorFormat::S,
//            VectorFormat::D => VectorFormat::D,
//            _ => VectorFormat::Undefined
//        }
//    }

//    fn lane_count(&self) -> usize {
//        match self {
//            VectorFormat::B => 16,
//            VectorFormat::H => 8,
//            VectorFormat::S => 4,
//            VectorFormat::D => 2,
//            VectorFormat::Q => 1,
//            VectorFormat::Format2D => 2,
//            VectorFormat::Format4S => 4,
//            VectorFormat::Format8H => 8,
//            VectorFormat::Format8B => 8,
//            VectorFormat::Format16B => 16,
//            _ => 0
//        }
//    }

//    fn lane_size_in_bytes(&self) -> usize {
//        match self {
//            VectorFormat::B => 1,
//            VectorFormat::H => 2,
//            VectorFormat::S => 4,
//            VectorFormat::D => 8,
//            VectorFormat::Q => 16,
//            VectorFormat::Format2D => 8,
//            VectorFormat::Format4S => 4,
//            VectorFormat::Format8H => 2,
//            VectorFormat::Format8B => 1,
//            VectorFormat::Format16B => 1,
//            _ => 0
//        }
//    }
//}

//enum Condition {
//    eq,
//    ge,
//    gt,
//    hi,
//    hs,
//    lt,
//    le
//}

//trait LogicVRegisterTrait {
//    fn clear_for_write(&self, vform: VectorFormat);
//    fn read_uint_from_mem(&self, vform: VectorFormat, index: usize, addr: u64);
//    fn write_uint_to_mem(&self, vform: VectorFormat, index: usize, addr: u64);
//    fn int(&self, vform: VectorFormat, i: usize) -> i64;
//    fn uint(&self, vform: VectorFormat, i: usize) -> u64;
//    fn uint_left_justified(&self, vform: VectorFormat, i: usize) -> u64;
//    fn is(&self, other: &Self) -> bool;
//    fn set_uint(&self, vform: VectorFormat, i: usize, value: u64);
//    fn set_int(&self, vform: VectorFormat, i: usize, value: i64);
//    fn set_uint_array(&self, vform: VectorFormat, result: &[u64]);
//    fn set_signed_sat(&self, i: usize, value: bool);
//    fn set_unsigned_sat(&self, i: usize, value: bool);
//    fn set_rounding(&self, i: usize, value: bool);
//}

//trait SimVRegisterTrait {
//    fn signed_saturate(&self, vform: VectorFormat) -> Self;
//    fn unsigned_saturate(&self, vform: VectorFormat) -> Self;
//    fn round(&self, vform: VectorFormat) -> Self;
//}

//impl Simulator {
//    // Functions will be implemented in Simulator.rs
//    pub fn fixed_to_double(src: i64, fbits: i32, round: FPRounding) -> f64 {
//        unimplemented!()
//    }

//    pub fn ufixed_to_double(src: u64, fbits: i32, round: FPRounding) -> f64 {
//        unimplemented!()
//    }

//    pub fn fixed_to_float(src: i64, fbits: i32, round: FPRounding) -> f32 {
//        unimplemented!()
//    }

//    pub fn ufixed_to_float(src: u64, fbits: i32, round: FPRounding) -> f32 {
//        unimplemented!()
//    }

//    pub fn fixed_to_float16(src: i64, fbits: i32, round: FPRounding) -> half {
//        unimplemented!()
//    }

//    pub fn ufixed_to_float16(src: u64, fbits: i32, round: FPRounding) -> half {
//        unimplemented!()
//    }

//    pub fn fp_to_double(value: f32) -> f64 {
//        unimplemented!()
//    }

//    pub fn fp_to_float(value: half) -> f32 {
//        unimplemented!()
//    }

//    pub fn fp_to_float16(value: f32, round_mode: FPRounding) -> half {
//        unimplemented!()
//    }

//    pub fn fp_to_float16_double(value: f64, round_mode: FPRounding) -> half {
//        unimplemented!()
//    }

//    pub fn fp_to_float_double(value: f64, round_mode: FPRounding) -> f32 {
//        unimplemented!()
//    }

//    pub fn ld1(&self, vform: VectorFormat, dst: &LogicVRegister, addr: u64) {
//        unimplemented!()
//    }

//    pub fn ld1_indexed(&self, vform: VectorFormat, dst: &LogicVRegister, index: i32, addr: u64) {
//        unimplemented!()
//    }

//    pub fn ld1r(&self, vform: VectorFormat, dst: &LogicVRegister, addr: u64) {
//        unimplemented!()
//    }

//    pub fn ld2(&self, vform: VectorFormat, dst1: &LogicVRegister, dst2: &LogicVRegister, addr1: u64) {
//        unimplemented!()
//    }

//    pub fn ld2_indexed(&self, vform: VectorFormat, dst1: &LogicVRegister, dst2: &LogicVRegister, index: i32, addr1: u64) {
//        unimplemented!()
//    }

//    pub fn ld2r(&self, vform: VectorFormat, dst1: &LogicVRegister, dst2: &LogicVRegister, addr: u64) {
//        unimplemented!()
//    }

//    pub fn ld3(&self, vform: VectorFormat, dst1: &LogicVRegister, dst2: &LogicVRegister, dst3: &LogicVRegister, addr1: u64) {
//        unimplemented!()
//    }

//    pub fn ld3_indexed(&self, vform: VectorFormat, dst1: &LogicVRegister, dst2: &LogicVRegister, dst3: &LogicVRegister, index: i32, addr1: u64) {
//        unimplemented!()
//    }

//    pub fn ld3r(&self, vform: VectorFormat, dst1: &LogicVRegister, dst2: &LogicVRegister, dst3: &LogicVRegister, addr: u64) {
//        unimplemented!()
//    }

//    pub fn ld4(&self, vform: VectorFormat, dst1: &LogicVRegister, dst2: &LogicVRegister, dst3: &LogicVRegister, dst4: &LogicVRegister, addr1: u64) {
//        unimplemented!()
//    }

//    pub fn ld4_indexed(&self, vform: VectorFormat, dst1: &LogicVRegister, dst2: &LogicVRegister, dst3: &LogicVRegister, dst4: &LogicVRegister, index: i32, addr1: u64) {
//        unimplemented!()
//    }

//    pub fn ld4r(&self, vform: VectorFormat, dst1: &LogicVRegister, dst2: &LogicVRegister, dst3: &LogicVRegister, dst4: &LogicVRegister, addr: u64) {
//        unimplemented!()
//    }

//    pub fn st1(&self, vform: VectorFormat, src: &LogicVRegister, addr: u64) {
//        unimplemented!()
//    }

//    pub fn st1_indexed(&self, vform: VectorFormat, src: &LogicVRegister, index: i32, addr: u64) {
//        unimplemented!()
//    }

//    pub fn st2(&self, vform: VectorFormat, dst: &LogicVRegister, dst2: &LogicVRegister, addr: u64) {
//        unimplemented!()
//    }

//    pub fn st2_indexed(&self, vform: VectorFormat, dst: &LogicVRegister, dst2: &LogicVRegister, index: i32, addr: u64) {
//        unimplemented!()
//    }

//    pub fn st3(&self, vform: VectorFormat, dst: &LogicVRegister, dst2: &LogicVRegister, dst3: &LogicVRegister, addr: u64) {
//        unimplemented!()
//    }

//    pub fn st3_indexed(&self, vform: VectorFormat, dst: &LogicVRegister, dst2: &LogicVRegister, dst3: &LogicVRegister, index: i32, addr: u64) {
//        unimplemented!()
//    }

//    pub fn st4(&self, vform: VectorFormat, dst: &LogicVRegister, dst2: &LogicVRegister, dst3: &LogicVRegister, dst4: &LogicVRegister, addr: u64) {
//        unimplemented!()
//    }

//    pub fn st4_indexed(&self, vform: VectorFormat, dst: &LogicVRegister, dst2: &LogicVRegister, dst3: &LogicVRegister, dst4: &LogicVRegister, index: i32, addr: u64) {
//        unimplemented!()
//    }

//    pub fn cmp(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister, cond: Condition) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn cmp_imm(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, imm: i32, cond: Condition) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn cmptst(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn add(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn addp(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn mla(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn mls(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn mul(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn mul_indexed(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister, index: i32) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn mla_indexed(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister, index: i32) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn mls_indexed(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister, index: i32) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn smull(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister, index: i32) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn smull2(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister, index: i32) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn umull(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister, index: i32) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn umull2(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister, index: i32) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn smlal(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister, index: i32) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn smlal2(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister, index: i32) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn umlal(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister, index: i32) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn umlal2(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister, index: i32) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn smlsl(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister, index: i32) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn smlsl2(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister, index: i32) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn umlsl(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister, index: i32) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn umlsl2(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister, index: i32) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn sqdmull(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister, index: i32) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn sqdmull2(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister, index: i32) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn sqdmlal(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister, index: i32) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn sqdmlal2(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister, index: i32) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn sqdmlsl(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister, index: i32) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn sqdmlsl2(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister, index: i32) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn sqdmulh(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister, index: i32) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn sqrdmulh(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister, index: i32) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn polynomial_mult(&self, op1: u8, op2: u8) -> u16 {
//        unimplemented!()
//    }

//    pub fn pmul(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn pmull(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn pmull2(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn sub(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn and_op(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn orr(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn orn(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn eor(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn bic(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn bic_imm(&self, vform: VectorFormat, dst: &LogicVRegister, src: &LogicVRegister, imm: u64) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn bif(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn bit(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn bsl(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn sminmax(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister, max: bool) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn smax(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn smin(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn sminmaxp(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister, max: bool) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn smaxp(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn sminp(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn addp_src(&self, vform: VectorFormat, dst: &LogicVRegister, src: &LogicVRegister) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn addv(&self, vform: VectorFormat, dst: &LogicVRegister, src: &LogicVRegister) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn saddlv(&self, vform: VectorFormat, dst: &LogicVRegister, src: &LogicVRegister) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn uaddlv(&self, vform: VectorFormat, dst: &LogicVRegister, src: &LogicVRegister) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn sminmaxv(&self, vform: VectorFormat, dst: &LogicVRegister, src: &LogicVRegister, max: bool) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn smaxv(&self, vform: VectorFormat, dst: &LogicVRegister, src: &LogicVRegister) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn sminv(&self, vform: VectorFormat, dst: &LogicVRegister, src: &LogicVRegister) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn uminmax(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister, max: bool) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn umax(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn umin(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn uminmaxp(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister, max: bool) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn umaxp(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn uminp(&self, vform: VectorFormat, dst: &LogicVRegister, src1: &LogicVRegister, src2: &LogicVRegister) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn uminmaxv(&self, vform: VectorFormat, dst: &LogicVRegister, src: &LogicVRegister, max: bool) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn umaxv(&self, vform: VectorFormat, dst: &LogicVRegister, src: &LogicVRegister) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn uminv(&self, vform: VectorFormat, dst: &LogicVRegister, src: &LogicVRegister) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn shl(&self, vform: VectorFormat, dst: &LogicVRegister, src: &LogicVRegister, shift: i32) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn sshll(&self, vform: VectorFormat, dst: &LogicVRegister, src: &LogicVRegister, shift: i32) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn sshll2(&self, vform: VectorFormat, dst: &LogicVRegister, src: &LogicVRegister, shift: i32) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn shll(&self, vform: VectorFormat, dst: &LogicVRegister, src: &LogicVRegister) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn shll2(&self, vform: VectorFormat, dst: &LogicVRegister, src: &LogicVRegister) -> &LogicVRegister {
//        unimplemented!()
//    }

//    pub fn ushll(&self, v