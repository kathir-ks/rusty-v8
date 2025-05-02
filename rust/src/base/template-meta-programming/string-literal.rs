// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod tmp {
    //use std::array;
    use std::convert::TryInto;
    use std::marker::PhantomData;

    // Mimic HAS_CPP_CLASS_TYPES_AS_TEMPLATE_ARGS behavior.  Always enabled for now.
    // In C++, this would gate the feature based on compiler capabilities.

    // Mimic __cpp_lib_to_array.  Rust has into_array.

    /// Represents a compile-time string literal.
    ///
    /// This struct provides a way to pass compile-time string literals to
    /// generics. The size `N` includes the null terminator.
    #[derive(Copy, Clone)]
    pub struct StringLiteral<const N: usize> {
        data: [u8; N],
    }

    impl<const N: usize> StringLiteral<N> {
        /// Constructs a `StringLiteral` from a string literal.
        ///
        /// # Panics
        ///
        /// Panics if the provided string literal is not null-terminated, or has an invalid length.
        pub const fn new(s: &str) -> StringLiteral<N> {
            assert!(s.len() + 1 == N, "String literal must be N-1 bytes long");

            let mut data: [u8; N] = [0; N];
            let s_bytes = s.as_bytes();
            let mut i = 0;
            while i < s_bytes.len() {
                data[i] = s_bytes[i];
                i += 1;
            }
            data[N - 1] = 0; // Null terminate

            StringLiteral { data }
        }

        /// Returns the size of the string literal, excluding the null terminator.
        pub const fn size(&self) -> usize {
            assert!(self.data[N - 1] == 0, "String must be null terminated");
            N - 1
        }

        /// Returns a pointer to the beginning of the literal's data.
        pub fn as_ptr(&self) -> *const u8 {
            self.data.as_ptr()
        }

        /// Returns a raw pointer to the underlying data.
        pub fn c_str(&self) -> *const i8 {
            self.data.as_ptr() as *const i8
        }
    }

    impl<const N: usize> std::fmt::Debug for StringLiteral<N> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let s = unsafe { std::ffi::CStr::from_ptr(self.c_str()) };
            f.debug_struct("StringLiteral")
                .field("data", &s)
                .finish()
        }
    }
}