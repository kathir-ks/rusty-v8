// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This Rust code is a conversion of the C++ header file
// `include/cppgc/internal/base-page-handle.h` from the V8 JavaScript engine.

// This module defines the BasePageHandle struct and its associated methods
// for handling memory pages within the V8 garbage collection system.

pub mod internal {
    use std::mem;

    /// Placeholder for HeapHandle (as the original C++ refers to an external type).
    /// Replace with a real implementation if needed.
    #[derive(Debug)]
    pub struct HeapHandle {}

    impl HeapHandle {
        pub fn new() -> Self {
            HeapHandle {}
        }
    }

    // api_constants::kPageSize is assumed to be a constant representing the page size.
    // This is a placeholder; replace with the actual page size value.
    pub const K_PAGE_SIZE: usize = 4096;

    /// Represents a handle to a memory page.  Corresponds to `BasePageHandle` in C++.
    #[derive(Debug)]
    pub struct BasePageHandle {
        heap_handle: HeapHandle,
    }

    impl BasePageHandle {
        /// Calculates the `BasePageHandle` from a payload pointer.
        ///
        /// # Safety
        ///
        /// The `payload` pointer must point to a valid memory location within a managed page.
        #[inline]
        pub unsafe fn from_payload(payload: *mut std::ffi::c_void) -> *mut BasePageHandle {
            let address = payload as usize;
            let aligned_address = address & !(K_PAGE_SIZE - 1);
            aligned_address as *mut BasePageHandle
        }

        /// Calculates the `BasePageHandle` from a const payload pointer.
        ///
        /// # Safety
        ///
        /// The `payload` pointer must point to a valid memory location within a managed page.
        #[inline]
        pub unsafe fn from_payload_const(payload: *const std::ffi::c_void) -> *const BasePageHandle {
            BasePageHandle::from_payload(payload as *mut std::ffi::c_void) as *const BasePageHandle
        }

        /// Returns a mutable reference to the associated `HeapHandle`.
        #[inline]
        pub fn heap_handle(&mut self) -> &mut HeapHandle {
            &mut self.heap_handle
        }

        /// Returns a const reference to the associated `HeapHandle`.
        #[inline]
        pub fn heap_handle_const(&self) -> &HeapHandle {
            &self.heap_handle
        }
    }

    impl BasePageHandle {
        /// Creates a new `BasePageHandle`.
        ///
        /// # Safety
        ///
        /// The `heap_handle` must be a valid HeapHandle. The address of the constructed `BasePageHandle`
        /// must be a multiple of the page size.  This is a precondition that needs to be verified during
        /// construction.  In C++, this is done via a `CPPGC_DCHECK`.  Rust does not offer the exact
        /// same debugging facilities at compile time, so a run-time assert is used here for similar
        /// debugging purposes.
        pub unsafe fn new(heap_handle: &mut HeapHandle) -> Self {
            let handle = BasePageHandle {
                heap_handle: HeapHandle { ..*heap_handle },
            };
            assert!(mem::transmute::<&BasePageHandle, usize>(&handle) % K_PAGE_SIZE == 0);
            handle
        }
    }
}