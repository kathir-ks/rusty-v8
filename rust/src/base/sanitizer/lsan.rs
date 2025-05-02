// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// LeakSanitizer support.
mod lsan {
    #[cfg(all(feature = "address_sanitizer", not(target_os = "windows")))]
    mod inner {
        extern "C" {
            fn __lsan_ignore_object(ptr: *const std::ffi::c_void);
        }

        /// Ignore an object for LeakSanitizer.
        ///
        /// # Safety
        ///
        /// The pointer must be valid and point to a memory region that is managed by the allocator
        /// under LeakSanitizer's control.
        pub unsafe fn ignore_object<T>(ptr: *const T) {
            __lsan_ignore_object(ptr as *const std::ffi::c_void);
        }
    }

    #[cfg(not(all(feature = "address_sanitizer", not(target_os = "windows"))))]
    mod inner {
        /// Ignore an object for LeakSanitizer.
        ///
        /// This is a no-op when LeakSanitizer is not enabled, but it checks at compile time
        /// that the argument is a pointer.
        pub fn ignore_object<T>(_ptr: *const T) {
            // Compile-time check that the argument is a pointer type.
            // This check emulates the static_assert from the C++ code.
        }
    }

    /// Ignore an object for LeakSanitizer.
    pub use inner::ignore_object;
}

pub use lsan::*;