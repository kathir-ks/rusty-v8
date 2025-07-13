// Converted from V8 C++ source files:
// Header: boxed-float.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]

use std::{
    f32, f64,
    hash::{Hash, Hasher},
    mem::transmute,
};

use crate::base::numbers::double;

// Safety wrapper for a 32-bit floating-point value to make sure we don't lose
// the exact bit pattern during deoptimization when passing this value.
#[derive(Clone, Copy, Debug)]
pub struct Float32 {
    bit_pattern_: u32,
}

impl Float32 {
    pub fn default() -> Self {
        Float32 { bit_pattern_: 0 }
    }

    // This constructor does not guarantee that bit pattern of the input value
    // is preserved if the input is a NaN.
    pub fn new(value: f32) -> Self {
        assert!(!value.is_nan());
        Float32 {
            bit_pattern_: value.to_bits(),
        }
    }

    pub fn get_bits(&self) -> u32 {
        self.bit_pattern_
    }

    pub fn get_scalar(&self) -> f32 {
        f32::from_bits(self.bit_pattern_)
    }

    pub fn is_nan(&self) -> bool {
        self.get_scalar().is_nan()
    }

    // Return a pointer to the field storing the bit pattern. Used in code
    // generation tests to store generated values there directly.
    pub fn get_bits_address(&mut self) -> *mut u32 {
        &mut self.bit_pattern_ as *mut u32
    }

    pub const fn from_bits(bits: u32) -> Self {
        Float32 { bit_pattern_: bits }
    }
}

// Safety wrapper for a 64-bit floating-point value to make sure we don't lose
// the exact bit pattern during deoptimization when passing this value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Float64 {
    bit_pattern_: u64,
}

const K_HOLE_NAN_INT64: u64 = 0x7ff8000000000000;

impl Float64 {
    pub fn default() -> Self {
        Float64 { bit_pattern_: 0 }
    }

    // This constructor does not guarantee that bit pattern of the input value
    // is preserved if the input is a NaN.
    pub fn new(value: f64) -> Self {
        assert!(!value.is_nan());
        Float64 {
            bit_pattern_: value.to_bits(),
        }
    }

    pub fn from_double(value: double::Double) -> Self {
        Float64 {
            bit_pattern_: value.as_uint64(),
        }
    }

    pub fn get_bits(&self) -> u64 {
        self.bit_pattern_
    }

    pub fn get_scalar(&self) -> f64 {
        f64::from_bits(self.bit_pattern_)
    }

    pub fn is_hole_nan(&self) -> bool {
        self.bit_pattern_ == K_HOLE_NAN_INT64
    }

    pub fn is_nan(&self) -> bool {
        self.get_scalar().is_nan()
    }

    // Return a pointer to the field storing the bit pattern. Used in code
    // generation tests to store generated values there directly.
    pub fn get_bits_address(&mut self) -> *mut u64 {
        &mut self.bit_pattern_ as *mut u64
    }

    pub const fn from_bits(bits: u64) -> Self {
        Float64 { bit_pattern_: bits }
    }

    // Unlike doubles, equality is defined as equally behaving as far as the
    // optimizers are concerned. I.e., two NaN's are equal as long as they are
    // both the hole nor not.
    pub fn eq(&self, other: &Float64) -> bool {
        if self.is_nan() && other.is_nan() {
            return self.is_hole_nan() == other.is_hole_nan();
        }
        self.get_scalar() == other.get_scalar()
    }
}

impl Hash for Float64 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bit_pattern_.hash(state);
    }
}

pub mod base {
    use super::Float64;
    use std::hash::{Hash, Hasher};

    pub fn hash_value(f64: &Float64) -> u64 {
        if f64.is_nan() {
            if f64.is_hole_nan() {
                1u64
            } else {
                0u64
            }
        } else {
            f64.get_bits()
        }
    }
}
