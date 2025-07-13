// Converted from V8 C++ source files:
// Header: marking-progress-tracker.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

use std::sync::atomic::{AtomicUsize, Ordering};

const kMaxRegularHeapObjectSize: usize = 1024; // Example value

#[derive(Default)]
pub struct MarkingProgressTracker {
  overall_chunks_: usize,
  current_chunk_: AtomicUsize,
}

impl MarkingProgressTracker {
  pub const kChunkSize: usize = kMaxRegularHeapObjectSize;

  pub fn enable(&mut self, size: usize) {
    assert!(!self.is_enabled());
    self.overall_chunks_ = (size + Self::kChunkSize - 1) / Self::kChunkSize;
    self.current_chunk_.store(0, Ordering::Relaxed);
  }

  pub fn is_enabled(&self) -> bool {
    self.overall_chunks_ != 0
  }

  pub fn get_next_chunk_to_mark(&self) -> usize {
    let new_chunk = self.current_chunk_.fetch_add(1, Ordering::AcqRel);
    assert!(new_chunk < self.overall_chunks_);
    new_chunk
  }

  pub fn total_number_of_chunks(&self) -> usize {
    self.overall_chunks_
  }

  pub fn reset_if_enabled(&mut self) {
    if self.is_enabled() {
      self.current_chunk_.store(0, Ordering::Relaxed);
    }
  }

  pub fn get_current_chunk_for_testing(&self) -> usize {
    self.current_chunk_.load(Ordering::Relaxed)
  }
}
