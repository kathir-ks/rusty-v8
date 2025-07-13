// Converted from V8 C++ source files:
// Header: util.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// "Generic" helper functions (not specific to BigInts).

// Integer division, rounding up.
macro_rules! div_ceil {
    ($x:expr, $y:expr) => {
        (($x) - 1) / ($y) + 1
    };
}

pub mod bigint {

    // Rounds up x to a multiple of y.
    pub const fn round_up(x: i32, y: i32) -> i32 {
        (x + y - 1) & -y
    }

    // Different environments disagree on how 64-bit uintptr_t and uint64_t are
    // defined, so we have to use templates to be generic.
    pub fn count_leading_zeros_u64(value: u64) -> i32 {
        if value == 0 {
            64
        } else {
            value.leading_zeros() as i32
        }
    }

    pub fn count_leading_zeros_u32(value: u32) -> i32 {
        if value == 0 {
            32
        } else {
            value.leading_zeros() as i32
        }
    }

    pub fn count_trailing_zeros_u32(value: u32) -> i32 {
        if value == 0 {
            32
        } else {
            value.trailing_zeros() as i32
        }
    }

    pub const fn bit_length(n: i32) -> i32 {
        32 - count_leading_zeros_u32(n as u32)
    }

    pub const fn is_power_of_two(value: i32) -> bool {
        value > 0 && (value & (value - 1)) == 0
    }
}
