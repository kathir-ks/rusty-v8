// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::sync::atomic::{AtomicBool, Ordering};

pub mod internal {
    /// Represents a parallel work item that can be acquired for processing.
    pub struct ParallelWorkItem {
        acquire: AtomicBool,
    }

    impl ParallelWorkItem {
        /// Creates a new `ParallelWorkItem` in an unacquired state.
        pub fn new() -> Self {
            ParallelWorkItem {
                acquire: AtomicBool::new(false),
            }
        }

        /// Attempts to acquire the work item.
        ///
        /// Returns `true` if the acquisition was successful (i.e., the item was previously unacquired),
        /// and `false` otherwise.
        pub fn try_acquire(&self) -> bool {
            self.acquire
                .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
                .is_ok()
        }

        /// Checks if the work item has been acquired.
        ///
        /// Returns `true` if the item is currently acquired, and `false` otherwise.
        pub fn is_acquired(&self) -> bool {
            self.acquire.load(Ordering::Relaxed)
        }
    }

    impl Default for ParallelWorkItem {
        fn default() -> Self {
            Self::new()
        }
    }
} // namespace internal