// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod base {
    pub mod macros {
        #[macro_export]
        macro_rules! use_ {
            ($x:expr) => {
                let _ = $x;
            };
        }
    }
}

mod heap {
    pub mod local_heap {
        pub struct LocalHeap {}
        impl LocalHeap {
            pub fn safepoint(&self) {}
        }
    }
}

use crate::base::macros::use_;
use crate::heap::local_heap::LocalHeap;

/// This method generates a tick. Also makes the current thread to enter a
/// safepoint iff it was required to do so. The tick is used as a deterministic
/// correlate of time to detect performance or divergence bugs in Turbofan.
/// TickAndMaybeEnterSafepoint() should be called frequently thoughout the
/// compilation.
pub struct TickCounter {
    ticks_: usize,
    local_heap_: Option<Box<LocalHeap>>,
}

impl TickCounter {
    pub fn new() -> Self {
        TickCounter {
            ticks_: 0,
            local_heap_: None,
        }
    }
    pub fn tick_and_maybe_enter_safepoint(&mut self) {
        self.ticks_ += 1;
        // Magical number to detect performance bugs or compiler divergence.
        // Selected as being roughly 10x of what's needed frequently.
        const K_MAX_TICKS: usize = 100000000;
        use_!(K_MAX_TICKS);
        debug_assert!(self.ticks_ < K_MAX_TICKS);

        if let Some(local_heap) = &self.local_heap_ {
            local_heap.safepoint();
        }
    }

    pub fn attach_local_heap(&mut self, local_heap: Box<LocalHeap>) {
        self.local_heap_ = Some(local_heap);
    }

    pub fn detach_local_heap(&mut self) {
        self.local_heap_ = None;
    }

    pub fn current_ticks(&self) -> usize {
        self.ticks_
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::heap::local_heap::LocalHeap;

    #[test]
    fn test_tick_counter() {
        let mut tick_counter = TickCounter::new();
        assert_eq!(tick_counter.current_ticks(), 0);

        tick_counter.tick_and_maybe_enter_safepoint();
        assert_eq!(tick_counter.current_ticks(), 1);

        let local_heap = Box::new(LocalHeap {});
        tick_counter.attach_local_heap(local_heap);
        tick_counter.tick_and_maybe_enter_safepoint();
        assert_eq!(tick_counter.current_ticks(), 2);

        tick_counter.detach_local_heap();
        tick_counter.tick_and_maybe_enter_safepoint();
        assert_eq!(tick_counter.current_ticks(), 3);
    }
}