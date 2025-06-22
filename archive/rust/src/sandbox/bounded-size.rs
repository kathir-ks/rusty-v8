// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod bounded_size {
    // use crate::common::globals::*; // Assuming globals are defined elsewhere, adapt as needed

    // Placeholder for Address type, assuming it's a memory address
    pub type Address = usize;

    /// Reads a BoundedSize from the given memory address.
    #[inline]
    pub fn read_bounded_size_field(field_address: Address) -> usize {
        // SAFETY: This operation is inherently unsafe as it dereferences a raw pointer.
        // The caller must ensure that `field_address` is a valid memory location
        // containing a size_t value.
        unsafe { *(field_address as *const usize) }
    }

    /// Writes a BoundedSize to the given memory address.
    #[inline]
    pub fn write_bounded_size_field(field_address: Address, value: usize) {
        // SAFETY: This operation is inherently unsafe as it dereferences a raw pointer.
        // The caller must ensure that `field_address` is a valid memory location
        // and that writing to it is safe.
        unsafe {
            *(field_address as *mut usize) = value;
        }
    }

} // mod bounded_size