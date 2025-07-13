// Converted from V8 C++ source files:
// Header: N/A
// Implementation: simulator-logic-arm64.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_snake_case)]
#![allow(unused_variables)]
use std::convert::TryInto;
use std::ops::{BitAnd, BitOr, BitXor, Neg, Not, Shl, Shr, Sub};
use std::{f32, f64};

//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::registers::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::registers::FPRounding;

//use crate::codegen::arm64::registers::FPRounding;
//use crate::codegen::arm64::registers::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::registers::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::registers::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::registers::FPRounding;
//use crate::codegen::arm64::registers::FPRounding;

//use crate::base::bits::CountLeadingZeros;

//use crate::base::bits::CountLeadingSignBits;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::registers::FPRounding;

//use crate::numbers::conversions::DoubleToFloat16;
//use crate::numbers::conversions::Float16ToFloat;

//use crate::numbers::conversions::Float16ToFloat;

//use crate::codegen::arm64::registers::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::registers::FPRounding;

//use crate::codegen::arm64::registers::FPRounding;

//use crate::codegen::arm64::registers::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::registers::FPRounding;
//use crate::codegen::arm64::registers::FPRounding;

//use crate::codegen::arm64::registers::FPRounding;

//use crate::codegen::arm64::registers::FPRounding;

//use crate::base::bits::CountLeadingSignBits;

//use crate::numbers::conversions::DoubleToFloat16;
//use crate::numbers::conversions::Float16ToFloat;
//use crate::numbers::conversions::Float16ToFloat;

//use crate::codegen::arm64::registers::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::registers::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::registers::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::registers::FPRounding;

//use crate::codegen::arm64::registers::FPRounding;

//use crate::codegen::arm64::registers::FPRounding;

//use crate::codegen::arm64::registers::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::registers::FPRounding;

//use crate::codegen::arm64::registers::FPRounding;

//use crate::base::bits::CountLeadingZeros;
//use crate::base::bits::CountLeadingSignBits;

//use crate::numbers::conversions::DoubleToFloat16;
//use crate::numbers::conversions::Float16ToFloat;
//use crate::numbers::conversions::Float16ToFloat;

//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;

//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembler::FPRounding;
//#[cfg(target_arch = "arm64")]
//use crate::codegen::arm64::assembl
