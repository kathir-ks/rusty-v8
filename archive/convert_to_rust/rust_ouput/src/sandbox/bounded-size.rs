// Converted from V8 C++ source files:
// Header: bounded-size.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(non_snake_case)]

use std::mem;

pub type Address = usize;

mod common {
    pub mod globals {
        pub const kMaxSafeBufferSizeForSandbox: usize = usize::MAX >> 1; // A reasonable value
    }
}

mod v8 {
    pub mod internal {
        use super::super::Address;
        use super::super::common::globals::kMaxSafeBufferSizeForSandbox;

        #[inline]
        pub fn ReadBoundedSizeField(field_address: Address) -> usize {
            // This is inherently unsafe as it reads raw memory.  We assume that
            // the address is valid.
            unsafe {
                let ptr = field_address as *const usize;
                *ptr
            }
        }

        #[inline]
        pub fn WriteBoundedSizeField(field_address: Address, value: usize) {
            if value > kMaxSafeBufferSizeForSandbox {
                panic!("Value exceeds maximum safe buffer size for sandbox");
            }
            // This is inherently unsafe as it writes raw memory. We assume that
            // the address is valid.
            unsafe {
                let ptr = field_address as *mut usize;
                *ptr = value;
            }
        }
    }
}

pub use v8::internal::*;
