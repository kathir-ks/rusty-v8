// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::{mem, cmp};

macro_rules! foreach_simd_type {
    ($v:ident) => {
        $v!(double, Float64x2, f64x2, 2);
        $v!(float, Float32x4, f32x4, 4);
        $v!(i64, Int64x2, i64x2, 2);
        $v!(i32, Int32x4, i32x4, 4);
        $v!(i16, Int16x8, i16x8, 8);
        $v!(i8, Int8x16, i8x16, 16);
    };
}

pub mod internal {
    use std::{mem, cmp};
    use std::slice;

    macro_rules! define_simd_type {
        ($c_type:ty, $s_type:ident, $name:ident, $k_size:literal) => {
            #[derive(Copy, Clone, Debug)]
            #[repr(C)]
            pub struct $s_type {
                pub val: [$c_type; $k_size],
            }
        };
    }
    foreach_simd_type!(define_simd_type);

    /// A 128-bit SIMD value.
    #[derive(Copy, Clone, Debug)]
    #[repr(align(8))]
    pub struct Simd128 {
        val: [u8; 16],
    }

    impl Simd128 {
        /// Creates a new `Simd128` with all elements set to zero.
        pub fn new() -> Self {
            Simd128 { val: [0; 16] }
        }

        macro_rules! define_simd_type_specific_methods {
            ($c_type:ty, $s_type:ident, $name:ident, $size:literal) => {
                /// Creates a `Simd128` from a `$s_type` value.
                pub fn from_$name(val: $s_type) -> Self {
                    let mut simd = Simd128::new();
                    unsafe {
                        std::ptr::write_unaligned(simd.val.as_mut_ptr() as *mut $s_type, val);
                    }
                    simd
                }

                /// Converts the `Simd128` to a `$s_type` value.
                pub fn to_$name(&self) -> $s_type {
                    unsafe {
                        std::ptr::read_unaligned(self.val.as_ptr() as *const $s_type)
                    }
                }
            };
        }
        foreach_simd_type!(define_simd_type_specific_methods);

        /// Creates a `Simd128` from a byte array.
        pub fn from_bytes(bytes: &[u8]) -> Self {
            let mut simd = Simd128::new();
            simd.val.copy_from_slice(bytes);
            simd
        }

        /// Returns true if this `Simd128` is equal to another.
        pub fn operator_eq(&self, other: &Simd128) -> bool {
            self.val == other.val
        }

        /// Returns a reference to the underlying byte array.
        pub fn bytes(&self) -> &[u8] {
            &self.val
        }

        /// Converts the `Simd128` to a specific SIMD vector type.
        pub fn to<T>(&self) -> T
        where
            Self: ToSimd<T>,
        {
            <Self as ToSimd<T>>::to(self)
        }
    }

    impl Default for Simd128 {
        fn default() -> Self {
            Self::new()
        }
    }

    pub trait ToSimd<T> {
        fn to(self_: &Simd128) -> T;
    }

    macro_rules! declare_cast {
        ($c_type:ty, $s_type:ident, $name:ident, $size:literal) => {
            impl ToSimd<$s_type> for Simd128 {
                fn to(self_: &Simd128) -> $s_type {
                    self_.to_$name()
                }
            }
        };
    }
    foreach_simd_type!(declare_cast);

    // Define concrete SIMD types
    pub type Float64x2 = struct { val: [f64; 2] };
    pub type Float32x4 = struct { val: [f32; 4] };
    pub type Int64x2 = struct { val: [i64; 2] };
    pub type Int32x4 = struct { val: [i32; 4] };
    pub type Int16x8 = struct { val: [i16; 8] };
    pub type Int8x16 = struct { val: [i8; 16] };

    const K_SIMD128_SIZE: usize = 16;
}