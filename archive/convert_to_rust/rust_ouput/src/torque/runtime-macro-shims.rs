// Converted from V8 C++ source files:
// Header: runtime-macro-shims.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod torque_runtime_macro_shims {
    pub mod code_stub_assembler {
        use std::os::raw::c_char;

        pub fn bool_constant(b: bool) -> bool {
            b
        }

        pub fn change_int32_to_intptr(i: i32) -> isize {
            i as isize
        }

        pub fn change_uint32_to_word(u: u32) -> usize {
            u as usize
        }

        pub fn intptr_add(a: isize, b: isize) -> isize {
            a + b
        }

        pub fn intptr_mul(a: isize, b: isize) -> isize {
            a * b
        }

        pub fn intptr_less_than(a: isize, b: isize) -> bool {
            a < b
        }

        pub fn intptr_less_than_or_equal(a: isize, b: isize) -> bool {
            a <= b
        }

        pub fn signed(u: usize) -> isize {
            u as isize
        }

        pub trait Smi {
            fn value(&self) -> i32;
        }

        pub fn smi_untag<T: Smi>(s: T) -> i32 {
            s.value()
        }

        pub fn uintptr_less_than(a: usize, b: usize) -> bool {
            a < b
        }

        pub fn unsigned(s: i32) -> u32 {
            s as u32
        }

        #[cfg(target_pointer_width = "64")]
        pub fn unsigned_intptr(s: isize) -> usize {
            s as usize
        }

        pub fn word32_equal(a: u32, b: u32) -> bool {
            a == b
        }

        pub fn word32_not_equal(a: u32, b: u32) -> bool {
            a != b
        }

        #[derive(Debug, Clone)]
        pub struct IntegerLiteral {
            value: i64,
        }

        impl IntegerLiteral {
            pub fn to<T>(&self) -> T
            where
                T: From<i64>,
            {
                self.value.into()
            }
        }

        pub fn constexpr_integer_literal_to_int32(i: &IntegerLiteral) -> i32 {
            i.to::<i32>()
        }

        pub fn constexpr_integer_literal_to_int31(i: &IntegerLiteral) -> i32 {
            let val = constexpr_integer_literal_to_int32(i);
            if val >= -(1 << 30) && val < (1 << 30) {
                val
            } else {
                // Handle overflow.  This is consistent with the C++ code
                // that does not specify behavior on overflow and relies on
                // the compiler to truncate.
                val & ((1 << 31) - 1)
            }
        }

        pub fn constexpr_integer_literal_to_intptr(i: &IntegerLiteral) -> isize {
            i.to::<isize>()
        }

        pub fn print(str_: *const c_char) {
            unsafe {
                let s = std::ffi::CStr::from_ptr(str_);
                print!("{}", s.to_str().unwrap());
            }
        }
    }
}
