// Copyright 2013 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//! MemorySanitizer support.

#![cfg_attr(not(feature = "v8_use_memory_sanitizer"), allow(unused_variables))]
#![cfg_attr(not(feature = "v8_use_memory_sanitizer"), allow(dead_code))]

use std::convert::TryInto;
use std::mem::size_of;
use std::{
    marker::PhantomData,
    ptr::{null_mut, NonNull},
};

// Placeholder for v8::base::Address, which needs more context to be correctly translated.
// For now, just use usize as a stand-in.  A better representation might involve a raw pointer.
pub type Address = usize;

/// Marks a memory range as uninitialized, as if it was allocated here.
#[macro_export]
macro_rules! msan_allocated_uninitialized_memory {
    ($start:expr, $size:expr) => {
        #[cfg(feature = "v8_use_memory_sanitizer")]
        {
            extern "C" {
                fn __msan_allocated_memory(start: *const std::ffi::c_void, size: usize);
            }
            unsafe {
                __msan_allocated_memory($start as *const std::ffi::c_void, $size as usize);
            }
        }
        #[cfg(not(feature = "v8_use_memory_sanitizer"))]
        {
            let _ = $start;
            let _ = $size;
        }
    };
}

/// Marks a memory range as initialized.
#[macro_export]
macro_rules! msan_memory_is_initialized {
    ($start:expr, $size:expr) => {
        #[cfg(feature = "v8_use_memory_sanitizer")]
        {
            extern "C" {
                fn __msan_unpoison(start: *const std::ffi::c_void, size: usize);
            }
            unsafe {
                __msan_unpoison($start as *const std::ffi::c_void, $size as usize);
            }
        }
        #[cfg(not(feature = "v8_use_memory_sanitizer"))]
        {
            $crate::msan_allocated_uninitialized_memory!($start, $size);
        }
    };
}

/// Disables memory sanitization for a function.
#[macro_export]
macro_rules! disable_msan {
    () => {
        #[cfg(feature = "v8_use_memory_sanitizer")]
        {
            #[no_sanitize(memory)]
            fn dummy() {} // Dummy function to attach attribute
            dummy()
        }
        #[cfg(not(feature = "v8_use_memory_sanitizer"))]
        {}
    };
}