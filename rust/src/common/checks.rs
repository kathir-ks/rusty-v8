// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This is a simplified translation of the C++ header file.
// It doesn't include all the original functionality,
// especially the slow DCHECK functionality, which depends
// on global flags.
// For a complete translation, the global flags system and
// logging would need to be implemented in Rust.

/// Asserts that a condition is true during development builds.
#[macro_export]
macro_rules! dcheck {
    ($condition:expr) => {
        #[cfg(debug_assertions)]
        {
            if !($condition) {
                panic!("DCHECK failed: {}", stringify!($condition));
            }
        }
    };
}

/// Asserts that the left-hand side implies the right-hand side.
#[macro_export]
macro_rules! dcheck_implies {
    ($lhs:expr, $rhs:expr) => {
        dcheck!(!($lhs) || ($rhs));
    };
}

pub const K_HEAP_OBJECT_TAG_MASK: usize = 7; // Example value, adapt if needed

/// Asserts that an address is tag-aligned.
#[macro_export]
macro_rules! dcheck_tag_aligned {
    ($address:expr) => {
        dcheck!(($address & crate::K_HEAP_OBJECT_TAG_MASK) == 0);
    };
}

/// Asserts that a size is tag-aligned.
#[macro_export]
macro_rules! dcheck_size_tag_aligned {
    ($size:expr) => {
        dcheck!(($size & crate::K_HEAP_OBJECT_TAG_MASK) == 0);
    };
}