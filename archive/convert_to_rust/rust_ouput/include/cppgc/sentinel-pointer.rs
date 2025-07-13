// Converted from V8 C++ source files:
// Header: sentinel-pointer.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod api_constants {
    #[cfg(feature = "CPPGC_POINTER_COMPRESSION")]
    pub const kPointerCompressionShift: i32 = 3; // Example value
}

pub mod internal {
    // Special tag type used to denote some sentinel member. The semantics of the
    // sentinel is defined by the embedder.
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct SentinelPointer {}

    impl SentinelPointer {
        #[cfg(feature = "CPPGC_POINTER_COMPRESSION")]
        pub const K_SENTINEL_VALUE: isize = 1 << api_constants::kPointerCompressionShift;

        #[cfg(not(feature = "CPPGC_POINTER_COMPRESSION"))]
        pub const K_SENTINEL_VALUE: isize = 0b10;

        pub fn value(&self) -> isize {
            SentinelPointer::K_SENTINEL_VALUE
        }
    }

    impl SentinelPointer {
        pub fn to_raw_ptr<T>(&self) -> *mut T {
            SentinelPointer::K_SENTINEL_VALUE as *mut T
        }
    }
}

impl SentinelPointer {
    pub fn to_raw_ptr<T>(&self) -> *mut T {
        internal::SentinelPointer::K_SENTINEL_VALUE as *mut T
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SentinelPointer(internal::SentinelPointer);

impl SentinelPointer {
    pub const fn new() -> Self {
        SentinelPointer(internal::SentinelPointer {})
    }
}
impl Default for SentinelPointer {
    fn default() -> Self {
        Self::new()
    }
}
