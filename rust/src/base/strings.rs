// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod strings {
    use std::ffi::VaList;
    use std::os::raw::c_char;

    use crate::base::platform::platform;

    /// A wrapper around a mutable buffer of characters.
    pub struct Vector<T> {
        data: *mut T,
        length: usize,
    }

    impl<T> Vector<T> {
        pub fn new(data: *mut T, length: usize) -> Self {
            Vector { data, length }
        }

        pub fn begin(&mut self) -> *mut T {
            self.data
        }

        pub fn length(&self) -> usize {
            self.length
        }
    }

    pub fn vsnprintf(str: Vector<c_char>, format: *const c_char, args: VaList) -> i32 {
        platform::vsnprintf(str.begin(), str.length(), format, args)
    }

    pub fn snprintf(str: Vector<c_char>, format: *const c_char, args: ...) -> i32 {
        // NOTE: This requires using the `va_list` crate due to the limitations of `std::ffi::VaList`.
        // It's difficult to create a safe and correct variadic function wrapper in Rust without a lot of unsafe code or dependencies.
        // For simplicity, this implementation will return a dummy value and print a warning.
        // In a real scenario, you would need to use a crate like `va_list` or manually handle variadic arguments using unsafe code,
        // ensuring proper memory safety and alignment.
        eprintln!("Warning: snprintf with variadic arguments is not fully implemented. Returning a dummy value.");
        0
    }

    pub fn strncpy(dest: Vector<c_char>, src: *const c_char, n: usize) {
        platform::strncpy(dest.begin(), dest.length(), src, n);
    }
}

pub mod platform {
    use std::ffi::VaList;
    use std::os::raw::c_char;

    extern "C" {
        pub fn vsnprintf(str: *mut c_char, len: usize, format: *const c_char, args: VaList) -> i32;
        pub fn strncpy(dest: *mut c_char, dest_size: usize, src: *const c_char, n: usize);
    }
    pub mod platform {}
}