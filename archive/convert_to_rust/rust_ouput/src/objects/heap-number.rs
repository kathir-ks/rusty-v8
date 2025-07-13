// Converted from V8 C++ source files:
// Header: heap-number.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(non_camel_case_types)]

use std::io::Write;

//use crate::objects::object_macros::*;
use crate::objects::primitive_heap_object::PrimitiveHeapObject;
use crate::objects::tagged_field::UnalignedDoubleMember;
use crate::objects::map::Map;

pub struct HeapNumber {
    value_: UnalignedDoubleMember,
}

impl HeapNumber {
    pub fn value(&self) -> f64 {
        self.value_.get()
    }

    pub fn set_value(&mut self, value: f64) {
        self.value_.set(value);
    }

    pub fn value_as_bits(&self) -> u64 {
        self.value().to_bits()
    }

    pub fn set_value_as_bits(&mut self, bits: u64) {
        self.set_value(f64::from_bits(bits));
    }

    pub const K_SIGN_MASK: u32 = 0x80000000u32;
    pub const K_EXPONENT_MASK: u32 = 0x7ff00000u32;
    pub const K_MANTISSA_MASK: u32 = 0xfffffu32;
    pub const K_MANTISSA_BITS: i32 = 52;
    pub const K_EXPONENT_BITS: i32 = 11;
    pub const K_EXPONENT_BIAS: i32 = 1023;
    pub const K_EXPONENT_SHIFT: i32 = 20;
    pub const K_INFINITY_OR_NAN_EXPONENT: i32 =
        (HeapNumber::K_EXPONENT_MASK >> HeapNumber::K_EXPONENT_SHIFT) as i32 - HeapNumber::K_EXPONENT_BIAS;
    pub const K_MANTISSA_BITS_IN_TOP_WORD: i32 = 20;
    pub const K_NON_MANTISSA_BITS_IN_TOP_WORD: i32 = 12;

    pub fn heap_number_print(&self, mut os: &mut dyn Write) {
        write!(os, "HeapNumber: {}", self.value()).unwrap();
    }

    pub fn heap_number_verify(&self) -> bool {
        true // Placeholder implementation
    }

    pub fn heap_number_short_print(&self, mut os: &mut dyn Write) {
        write!(os, "{}", self.value()).unwrap();
    }

    pub fn required_alignment(_map: *mut Map) -> AllocationAlignment {
        AllocationAlignment::kDoubleAligned
    }
}

#[derive(Debug)]
pub enum HeapNumberError {
    GenericError,
}

// Implement Display for the error type
impl std::fmt::Display for HeapNumberError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HeapNumberError::GenericError => write!(f, "Generic HeapNumber error"),
        }
    }
}

// Implement Error for the error type
impl std::error::Error for HeapNumberError {}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AllocationAlignment {
    kWordAligned,
    kDoubleAligned,
}
