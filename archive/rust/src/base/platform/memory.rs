// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(non_snake_case)]

use std::alloc::{alloc, alloc_zeroed, dealloc, Layout};
use std::mem;
use std::ptr;

#[cfg(target_os = "windows")]
use winapi::um::malloc::{_aligned_free, _aligned_malloc, _msize};

#[cfg(target_os = "macos")]
use libc::malloc_size;

// Define a macro for platform-specific malloc usable size
#[cfg(all(
    target_family = "unix",
    not(target_os = "aix"),
    not(target_os = "solaris"),
    not(target_os = "hermit"),
    not(target_os = "openbsd"),
    not(target_os = "zos")
))]
use libc::malloc_usable_size;

pub mod base {
    use super::*;
    use std::alloc::{alloc, dealloc, Layout};
    use std::mem;
    use std::ptr;

    //use crate::base::bits; // Assuming bits.h is converted to bits.rs
    //use crate::base::logging; // Assuming logging.h is converted to logging.rs
    //use crate::base::macros; // Assuming macros.h is converted to macros.rs

    #[inline]
    pub fn Malloc(size: usize) -> *mut u8 {
        #[cfg(feature = "starboard")]
        {
            // Assuming SbMemoryAllocate is available via FFI
            extern "C" {
                fn SbMemoryAllocate(size: usize) -> *mut u8;
            }
            unsafe { SbMemoryAllocate(size) }
        }
        #[cfg(all(target_os = "aix", target_env = "gnu"))]
        {
            // Assuming __linux_malloc is available via FFI
            extern "C" {
                fn __linux_malloc(size: usize) -> *mut u8;
            }
            unsafe { __linux_malloc(size) }
        }
        #[cfg(not(any(feature = "starboard", all(target_os = "aix", target_env = "gnu"))))]
        {
            unsafe { alloc(Layout::from_size_align(size, 1).unwrap()) }
        }
    }

    #[inline]
    pub fn Realloc(memory: *mut u8, size: usize) -> *mut u8 {
        assert_ne!(0, size);
        #[cfg(feature = "starboard")]
        {
            // Assuming SbMemoryReallocate is available via FFI
            extern "C" {
                fn SbMemoryReallocate(memory: *mut u8, size: usize) -> *mut u8;
            }
            unsafe { SbMemoryReallocate(memory, size) }
        }
        #[cfg(all(target_os = "aix", target_env = "gnu"))]
        {
            // Assuming __linux_realloc is available via FFI
            extern "C" {
                fn __linux_realloc(memory: *mut u8, size: usize) -> *mut u8;
            }
            unsafe { __linux_realloc(memory, size) }
        }
        #[cfg(not(any(feature = "starboard", all(target_os = "aix", target_env = "gnu"))))]
        {
            unsafe {
                let layout = Layout::for_value(memory as &u8);
                alloc::realloc(memory, layout, size)
            }
        }
    }

    #[inline]
    pub fn Free(memory: *mut u8) {
        #[cfg(feature = "starboard")]
        {
            // Assuming SbMemoryDeallocate is available via FFI
            extern "C" {
                fn SbMemoryDeallocate(memory: *mut u8);
            }
            unsafe {
                SbMemoryDeallocate(memory);
            }
        }
        #[cfg(not(feature = "starboard"))]
        {
            if memory.is_null() {
                return;
            }
            unsafe {
                let layout = Layout::for_value(memory as &u8);
                dealloc(memory, layout);
            }
        }
    }

    #[inline]
    pub fn Calloc(count: usize, size: usize) -> *mut u8 {
        #[cfg(feature = "starboard")]
        {
            // Assuming SbMemoryCalloc is available via FFI
            extern "C" {
                fn SbMemoryCalloc(count: usize, size: usize) -> *mut u8;
            }
            unsafe { SbMemoryCalloc(count, size) }
        }
        #[cfg(all(target_os = "aix", target_env = "gnu"))]
        {
            // Assuming __linux_calloc is available via FFI
            extern "C" {
                fn __linux_calloc(count: usize, size: usize) -> *mut u8;
            }
            unsafe { __linux_calloc(count, size) }
        }
        #[cfg(not(any(feature = "starboard", all(target_os = "aix", target_env = "gnu"))))]
        {
            unsafe { alloc_zeroed(Layout::from_size_align(count * size, 1).unwrap()) }
        }
    }

    #[inline]
    pub fn AlignedAlloc(size: usize, alignment: usize) -> *mut u8 {
        assert!(alignment >= mem::align_of::<*mut u8>());
        //assert!(bits::IsPowerOfTwo(alignment)); // Assuming bits.h provides this function

        #[cfg(target_os = "windows")]
        unsafe {
            _aligned_malloc(size, alignment) as *mut u8
        }
        #[cfg(all(target_os = "android", target_env = "gnu"))]
        {
            // Bionic doesn't expose posix_memalign
            unsafe { libc::memalign(alignment, size) as *mut u8 }
        }
        #[cfg(target_os = "zos")]
        {
            extern "C" {
                fn __aligned_malloc(size: usize, alignment: usize) -> *mut u8;
            }
            unsafe { __aligned_malloc(size, alignment) }
        }
        #[cfg(not(any(target_os = "windows", all(target_os = "android", target_env = "gnu"), target_os = "zos")))]
        {
            let mut ptr: *mut std::ffi::c_void = ptr::null_mut();
            let result = unsafe { libc::posix_memalign(&mut ptr, alignment, size) };
            if result != 0 {
                ptr::null_mut::<u8>() as *mut u8
            } else {
                ptr as *mut u8
            }
        }
    }

    #[inline]
    pub fn AlignedFree(ptr: *mut u8) {
        #[cfg(target_os = "windows")]
        unsafe {
            _aligned_free(ptr);
        }
        #[cfg(target_os = "zos")]
        {
            extern "C" {
                fn __aligned_free(ptr: *mut u8);
            }
            unsafe {
                __aligned_free(ptr);
            }
        }
        #[cfg(not(any(target_os = "windows", target_os = "zos")))]
        {
            // Correct on V8_LIBC_BIONIC and other platforms where aligned alloc use regular free
            Free(ptr);
        }
    }

    #[cfg(any(
        target_os = "windows",
        target_os = "macos",
        all(
            target_family = "unix",
            not(target_os = "aix"),
            not(target_os = "solaris"),
            not(target_os = "hermit"),
            not(target_os = "openbsd"),
            not(target_os = "zos")
        )
    ))]
    pub fn MallocUsableSize(ptr: *mut u8) -> usize {
        #[cfg(target_os = "windows")]
        {
            if ptr.is_null() {
                return 0;
            }
            unsafe { _msize(ptr as *mut std::ffi::c_void) as usize }
        }
        #[cfg(target_os = "macos")]
        unsafe {
            malloc_size(ptr as *mut std::ffi::c_void) as usize
        }
        #[cfg(all(
            target_family = "unix",
            not(target_os = "aix"),
            not(target_os = "solaris"),
            not(target_os = "hermit"),
            not(target_os = "openbsd"),
            not(target_os = "zos")
        ))]
        unsafe {
            malloc_usable_size(ptr as *mut std::ffi::c_void) as usize
        }
    }

    // Mimics C++23 `allocation_result`.
    #[derive(Debug)]
    pub struct AllocationResult<T> {
        pub ptr: *mut T,
        pub count: usize,
    }

    // Allocates at least `n * sizeof(T)` uninitialized storage but may allocate
    // more which is indicated by the return value. Mimics C++23
    // `allocate_at_least()`.
    pub fn AllocateAtLeast<T>(n: usize) -> AllocationResult<T> {
        let min_wanted_size = n * mem::size_of::<T>();
        let memory = Malloc(min_wanted_size) as *mut T;

        #[cfg(not(any(
            target_os = "windows",
            target_os = "macos",
            all(
                target_family = "unix",
                not(target_os = "aix"),
                not(target_os = "solaris"),
                not(target_os = "hermit"),
                not(target_os = "openbsd"),
                not(target_os = "zos")
            )
        )))]
        {
            return AllocationResult {
                ptr: memory,
                count: min_wanted_size,
            };
        }

        #[cfg(any(
            target_os = "windows",
            target_os = "macos",
            all(
                target_family = "unix",
                not(target_os = "aix"),
                not(target_os = "solaris"),
                not(target_os = "hermit"),
                not(target_os = "openbsd"),
                not(target_os = "zos")
            )
        ))]
        {
            let usable_size = MallocUsableSize(memory as *mut u8);
            #[cfg(feature = "use_ubsan")]
            {
                if memory.is_null() {
                    return AllocationResult { ptr: ptr::null_mut(), count: 0 };
                }

                if usable_size != min_wanted_size {
                    let memory_realloc = Realloc(memory as *mut u8, usable_size) as *mut T;
                    return AllocationResult {
                        ptr: memory_realloc,
                        count: usable_size,
                    };
                }
            }
            return AllocationResult {
                ptr: memory,
                count: usable_size,
            };
        }
    }
}