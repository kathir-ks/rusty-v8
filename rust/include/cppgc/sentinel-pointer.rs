// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod internal {
    /// Special tag type used to denote some sentinel member. The semantics of the
    /// sentinel is defined by the embedder.
    #[derive(Copy, Clone)]
    pub struct SentinelPointer {}

    impl SentinelPointer {
        #[cfg(feature = "pointer_compression")]
        pub const SENTINEL_VALUE: usize = 1 << crate::api_constants::POINTER_COMPRESSION_SHIFT;
        #[cfg(not(feature = "pointer_compression"))]
        pub const SENTINEL_VALUE: usize = 0b10;

        pub fn to_raw_ptr<T>(&self) -> *mut T {
            Self::SENTINEL_VALUE as *mut T
        }
    }

    impl PartialEq for SentinelPointer {
        fn eq(&self, _other: &Self) -> bool {
            true
        }
    }

    impl Eq for SentinelPointer {}
}

pub const K_SENTINEL_POINTER: internal::SentinelPointer = internal::SentinelPointer {};

// Placeholder module for api-constants.h.  In a full conversion, this
// would be replaced with the actual Rust implementation.
mod api_constants {
    pub const POINTER_COMPRESSION_SHIFT: usize = 3; // Example value
}