// Copyright 2016 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Slightly adapted for inclusion in V8.
// Copyright 2016 the V8 project authors. All rights reserved.

pub mod base {
    /// Function object which invokes 'free' on its parameter, which must be
    /// a pointer. Can be used to store malloc-allocated pointers in
    /// `std::unique_ptr` (Box in Rust equivalent):
    ///
    /// ```rust
    /// use std::ptr;
    /// use v8::base::FreeDeleter;
    ///
    /// // This example allocates memory using libc::malloc and ensures it's freed
    /// // when the `boxed_int` variable goes out of scope, via FreeDeleter.
    /// let memory = unsafe { libc::malloc(std::mem::size_of::<i32>()) as *mut i32 };
    /// if memory.is_null() {
    ///     panic!("Memory allocation failed");
    /// }
    ///
    /// let boxed_int = unsafe { Box::from_raw_in(memory, FreeDeleter) };
    /// *boxed_int = 42;
    /// assert_eq!(*boxed_int, 42);
    ///
    /// // boxed_int is dropped here, and the allocated memory is freed.
    /// ```
    #[derive(Copy, Clone, Debug, Default)]
    pub struct FreeDeleter;

    impl FreeDeleter {
        #[inline]
        pub fn call(&self, ptr: *mut std::ffi::c_void) {
            unsafe {
                if !ptr.is_null() {
                   libc::free(ptr);
                }
            }
        }
    }

    impl Drop for FreeDeleter {
        fn drop(&mut self) {
           // Intentionally empty drop implementation to satisfy Drop trait requirement
           // when used with from_raw_in in a Box.
        }
    }
} // namespace base