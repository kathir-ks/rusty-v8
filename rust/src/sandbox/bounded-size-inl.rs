// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/sandbox/bounded-size-inl.h

// Include the non-inl header before the rest of the headers.
mod bounded_size;

//use v8_rs::internal::*; // Assuming a crate that provides similar functionality to v8-internal.h
//use v8_rs::common::ptr_compr::*; // Assuming a crate that provides similar functionality to ptr-compr-inl.h
//use v8_rs::sandbox::*; // Assuming a crate that provides similar functionality to sandbox.h
//use v8_rs::sandbox::sandboxed_pointer::*; // Assuming a crate that provides similar functionality to sandboxed-pointer.h

// Assuming base::ReadUnalignedValue and base::WriteUnalignedValue are similar to reading and writing raw bytes

use std::mem::size_of;

const K_BOUNDED_SIZE_SHIFT: usize = 0; // Placeholder value, replace with actual value from C++
const K_MAX_SAFE_BUFFER_SIZE_FOR_SANDBOX: usize = usize::MAX >> K_BOUNDED_SIZE_SHIFT; // Placeholder value, replace with actual value from C++

#[inline]
pub fn read_bounded_size_field(field_address: usize) -> usize {
    #[cfg(feature = "v8_enable_sandbox")]
    {
        let raw_value = unsafe { (field_address as *const usize).read_unaligned() };
        raw_value >> K_BOUNDED_SIZE_SHIFT
    }
    #[cfg(not(feature = "v8_enable_sandbox"))]
    {
       unsafe { (field_address as *const usize).read_unaligned() }
    }
}

#[inline]
pub fn write_bounded_size_field(field_address: usize, value: usize) {
    #[cfg(feature = "v8_enable_sandbox")]
    {
        debug_assert!(value <= K_MAX_SAFE_BUFFER_SIZE_FOR_SANDBOX);
        let raw_value = value << K_BOUNDED_SIZE_SHIFT;
        unsafe { (field_address as *mut usize).write_unaligned(raw_value) };
    }
    #[cfg(not(feature = "v8_enable_sandbox"))]
    {
        unsafe { (field_address as *mut usize).write_unaligned(value) };
    }
}
