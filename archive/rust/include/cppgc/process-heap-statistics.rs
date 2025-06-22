// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::sync::atomic::{AtomicUsize, Ordering};

pub mod internal {
    // The C++ code has a forward declaration to internal::ProcessHeapStatisticsUpdater
    // which is not provided. Thus this is just an empty module definition.
    // This module would contain Rust equivalent of the ProcessHeapStatisticsUpdater.
}

/// Provides process heap statistics such as total allocated object size and space.
pub struct ProcessHeapStatistics {}

impl ProcessHeapStatistics {
    static TOTAL_ALLOCATED_OBJECT_SIZE: AtomicUsize = AtomicUsize::new(0);
    static TOTAL_ALLOCATED_SPACE: AtomicUsize = AtomicUsize::new(0);

    /// Returns the total allocated object size.
    pub fn total_allocated_object_size() -> usize {
        Self::TOTAL_ALLOCATED_OBJECT_SIZE.load(Ordering::Relaxed)
    }

    /// Returns the total allocated space.
    pub fn total_allocated_space() -> usize {
        Self::TOTAL_ALLOCATED_SPACE.load(Ordering::Relaxed)
    }

    // The friend class `internal::ProcessHeapStatisticsUpdater` would have access
    // to modify the atomic variables. In Rust, the typical pattern would be
    // to expose a mutable API through a separate function, possibly gated behind
    // an internal module or a specific feature. Since the updater class is not
    // defined, we leave this as a comment.

    /// Updates the total allocated space.
    #[allow(dead_code)] // Removing the dead_code warning as this might be used in the future.
    pub(crate) fn update_total_allocated_space(new_size: usize) {
        Self::TOTAL_ALLOCATED_SPACE.store(new_size, Ordering::Relaxed);
    }

    /// Updates the total allocated object size.
    #[allow(dead_code)] // Removing the dead_code warning as this might be used in the future.
    pub(crate) fn update_total_allocated_object_size(new_size: usize) {
        Self::TOTAL_ALLOCATED_OBJECT_SIZE.store(new_size, Ordering::Relaxed);
    }
}