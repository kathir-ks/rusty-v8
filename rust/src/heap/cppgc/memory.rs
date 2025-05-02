// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod memory {
    use std::mem::size_of;
    use std::ptr::NonNull;

    /// A zapped value used to fill deallocated memory. The lowest bit
    /// should be 0 to prevent zapped objects from being viewed as fully
    /// constructed.
    const ZAPPED_VALUE: u8 = 0xdc;

    /// Fills the memory at the given address with the zapped value.
    ///
    /// # Safety
    ///
    /// The `address` must be a valid pointer to a memory region of at least
    /// `size` bytes.
    #[inline]
    pub unsafe fn zap_memory(address: *mut u8, size: usize) {
        std::ptr::write_bytes(address, ZAPPED_VALUE, size);
    }

    /// Checks if the memory at the given address is filled with the zapped value.
    ///
    /// # Safety
    ///
    /// The `address` must be a valid pointer to a memory region of at least
    /// `size` bytes.
    #[inline]
    pub unsafe fn check_memory_is_zapped(address: *const u8, size: usize) {
        for i in 0..size {
            assert_eq!(*address.add(i), ZAPPED_VALUE);
        }
    }

    /// Checks if the memory at the given address is filled with zeros.
    ///
    /// # Safety
    ///
    /// The `address` must be a valid pointer to a memory region of at least
    /// `size` bytes.
    #[inline]
    pub unsafe fn check_memory_is_zero(address: *const u8, size: usize) {
        for i in 0..size {
            assert_eq!(*address.add(i), 0);
        }
    }

    /// Marks the memory at the given address as accessible.
    ///
    /// This function may be a no-op in release builds.  In debug builds, it
    /// might interact with memory sanitizers to track memory access.
    ///
    /// # Safety
    ///
    /// The `address` must be a valid pointer to a memory region of at least
    /// `size` bytes.
    #[inline]
    pub unsafe fn set_memory_accessible(address: *mut u8, size: usize) {}

    /// Marks the memory at the given address as inaccessible.
    ///
    /// This function may be a no-op in release builds. In debug builds, it
    /// might interact with memory sanitizers to track memory access. It's used when an object
    /// is deallocated.
    ///
    /// # Safety
    ///
    /// The `address` must be a valid pointer to a memory region of at least
    /// `size` bytes.
    #[inline]
    pub unsafe fn set_memory_inaccessible(address: *mut u8, size: usize) {
        std::ptr::write_bytes(address, 0, size);
    }

    /// Checks if the memory at the given address is inaccessible.
    ///
    /// This function may be a no-op.
    ///
    /// # Safety
    ///
    /// The `address` must be a valid pointer to a memory region of at least
    /// `size` bytes.
    #[inline]
    pub unsafe fn check_memory_is_inaccessible(address: *const u8, size: usize) {}

    /// Returns true if `CheckMemoryIsInaccessible` is a no-op.
    #[inline]
    pub const fn check_memory_is_inaccessible_is_noop() -> bool {
        true
    }
}