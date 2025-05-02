// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// The MarkingProgressTracker allows for keeping track of the bytes processed of
/// a single object. It splits marking of large arrays into chunks so that the
/// work can be shared across multiple concurrent markers. The tracker must be
/// enabled before it's used.
///
/// Only large objects use the tracker which is stored in their page metadata.
/// These objects are scanned in increments and concurrently and will be kept
/// black while being scanned. Even if the mutator writes to them they will be
/// kept black and a white to grey transition is performed in the value via
/// regular write barrier.
///
/// The tracker starts as disabled. After enabling (through `Enable()`), it can
/// never be disabled again.
pub struct MarkingProgressTracker {
    overall_chunks_: usize,
    current_chunk_: std::sync::atomic::AtomicUsize,
}

impl MarkingProgressTracker {
    pub const K_CHUNK_SIZE: usize = crate::globals::k_max_regular_heap_object_size;

    /// Enables the tracker with the given size.
    pub fn enable(&mut self, size: usize) {
        debug_assert!(!self.is_enabled());
        self.overall_chunks_ = (size + Self::K_CHUNK_SIZE - 1) / Self::K_CHUNK_SIZE;
        self.current_chunk_.store(0, std::sync::atomic::Ordering::Relaxed);
    }

    /// Returns true if the tracker is enabled.
    pub fn is_enabled(&self) -> bool {
        self.overall_chunks_ != 0
    }

    /// Gets the next chunk to mark.
    pub fn get_next_chunk_to_mark(&self) -> usize {
        let new_chunk = self.current_chunk_.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        debug_assert!(new_chunk < self.overall_chunks_);
        new_chunk
    }

    /// Returns the total number of chunks.
    pub fn total_number_of_chunks(&self) -> usize {
        self.overall_chunks_
    }

    /// Resets the tracker if it is enabled.
    pub fn reset_if_enabled(&mut self) {
        if self.is_enabled() {
            self.current_chunk_.store(0, std::sync::atomic::Ordering::Relaxed);
        }
    }

    /// Returns the current chunk for testing purposes.
    pub fn get_current_chunk_for_testing(&self) -> usize {
        self.current_chunk_.load(std::sync::atomic::Ordering::Relaxed)
    }
}

impl Default for MarkingProgressTracker {
    fn default() -> Self {
        MarkingProgressTracker {
            overall_chunks_: 0,
            current_chunk_: std::sync::atomic::AtomicUsize::new(0),
        }
    }
}