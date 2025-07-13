// Converted from V8 C++ source files:
// Header: parallel-work-item.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

use std::sync::atomic::{AtomicBool, Ordering};

pub struct ParallelWorkItem {
    acquire_: AtomicBool,
}

impl ParallelWorkItem {
    pub fn new() -> Self {
        ParallelWorkItem {
            acquire_: AtomicBool::new(false),
        }
    }

    pub fn try_acquire(&self) -> bool {
        self.acquire_
            .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok()
    }

    pub fn is_acquired(&self) -> bool {
        self.acquire_.load(Ordering::Relaxed)
    }
}

impl Default for ParallelWorkItem {
    fn default() -> Self {
        Self::new()
    }
}
