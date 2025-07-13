// Converted from V8 C++ source files:
// Header: lsan.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// LeakSanitizer support.

// There is no compile time flag for LSan, so enable this whenever ASan is
// enabled. Note that LSan can be used as part of ASan with 'detect_leaks=1'.
// On Windows, LSan is not implemented yet, so disable it there.
#[cfg(all(feature = "address_sanitizer", not(target_os = "windows")))]
mod lsan_impl {
    extern "C" {
        fn __lsan_ignore_object(ptr: *const std::ffi::c_void);
    }

    pub fn ignore_object<T>(ptr: *const T) {
        unsafe {
            __lsan_ignore_object(ptr as *const std::ffi::c_void);
        }
    }
}

#[cfg(not(all(feature = "address_sanitizer", not(target_os = "windows"))))]
mod lsan_impl {
    use std::marker::PhantomData;

    pub fn ignore_object<T>(_ptr: *const T) {
        // This static assertion is performed at compile time.  If the type
        // provided is not a pointer, then this will not compile.
        let _ = PhantomData::<*const T>;
    }
}

pub use lsan_impl::ignore_object as LSAN_IGNORE_OBJECT;
