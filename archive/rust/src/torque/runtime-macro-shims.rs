// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file contains runtime implementations of a few macros that are defined
// as external in Torque, so that generated runtime code can work.

use std::os::raw::{c_char, c_int, c_uint};

mod integer_literal {
    // Placeholder for IntegerLiteral, as its full implementation isn't provided.
    // Replace with a proper implementation if available.
    #[derive(Debug, Clone, Copy)]
    pub struct IntegerLiteral {
        value: i64,
    }

    impl IntegerLiteral {
        pub fn new(value: i64) -> Self {
            IntegerLiteral { value }
        }

        pub fn to<T: IntegerLiteralConvertible>(&self) -> T {
            T::from_i64(self.value)
        }
    }

    pub trait IntegerLiteralConvertible {
        fn from_i64(value: i64) -> Self;
    }

    impl IntegerLiteralConvertible for i32 {
        fn from_i64(value: i64) -> Self {
            value as i32
        }
    }

    impl IntegerLiteralConvertible for isize {
        fn from_i64(value: i64) -> Self {
            value as isize
        }
    }
}

pub mod torque_runtime_macro_shims {
    pub mod code_stub_assembler {
        use crate::integer_literal::IntegerLiteral;
        use std::os::raw::c_char;

        #[inline]
        pub fn bool_constant(b: bool) -> bool {
            b
        }

        #[inline]
        pub fn change_int32_to_intptr(i: i32) -> isize {
            i as isize
        }

        #[inline]
        pub fn change_uint32_to_word(u: u32) -> usize {
            u as usize
        }

        #[inline]
        pub fn intptr_add(a: isize, b: isize) -> isize {
            a + b
        }

        #[inline]
        pub fn intptr_mul(a: isize, b: isize) -> isize {
            a * b
        }

        #[inline]
        pub fn intptr_less_than(a: isize, b: isize) -> bool {
            a < b
        }

        #[inline]
        pub fn intptr_less_than_or_equal(a: isize, b: isize) -> bool {
            a <= b
        }

        #[inline]
        pub fn signed(u: usize) -> isize {
            u as isize
        }

        #[inline]
        pub fn smi_untag<Smi: SmiValue>(s: Smi) -> i32 {
            s.value()
        }

        pub trait SmiValue {
            fn value(&self) -> i32;
        }

        // Placeholder for Smi type; replace with an actual Smi implementation if available.
        impl SmiValue for i32 {
            fn value(&self) -> i32 {
                *self
            }
        }

        #[inline]
        pub fn uintptr_less_than(a: usize, b: usize) -> bool {
            a < b
        }

        #[inline]
        pub fn unsigned(s: i32) -> u32 {
            s as u32
        }

        #[cfg(target_pointer_width = "64")]
        #[inline]
        pub fn unsigned_intptr(s: isize) -> usize {
            s as usize
        }

        #[inline]
        pub fn word32_equal(a: u32, b: u32) -> bool {
            a == b
        }

        #[inline]
        pub fn word32_not_equal(a: u32, b: u32) -> bool {
            a != b
        }

        #[inline]
        pub fn constexpr_integer_literal_to_int32(i: &IntegerLiteral) -> i32 {
            i.to::<i32>()
        }

        #[inline]
        pub fn constexpr_integer_literal_to_int31(i: &IntegerLiteral) -> i32 {
            i.to::<i32>()
        }

        #[inline]
        pub fn constexpr_integer_literal_to_intptr(i: &IntegerLiteral) -> isize {
            i.to::<isize>()
        }

        #[inline]
        pub fn print(str: *const c_char) {
            // This requires a safe abstraction over C strings
            let c_str = unsafe { std::ffi::CStr::from_ptr(str) };
            if let Ok(string) = c_str.to_str() {
                eprint!("{}", string); // Prints to stderr, similar to PrintF
            } else {
                eprint!("Invalid UTF-8 sequence in C string");
            }
        }
    }
}