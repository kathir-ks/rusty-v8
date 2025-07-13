// Converted from V8 C++ source files:
// Header: tick-counter.h
// Implementation: tick-counter.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod macros {
        #[macro_export]
        macro_rules! USE {
            ($x:expr) => {
                {
                    $x;
                }
            };
        }
    }
}

pub mod heap {
    pub struct LocalHeap {}

    impl LocalHeap {
        pub fn Safepoint(&self) {}
    }
}

pub mod codegen {
    use crate::heap::LocalHeap;

    #[derive(Debug)]
    pub enum TickCounterError {
        LocalHeapError,
    }

    pub struct TickCounter {
        ticks_: usize,
        local_heap_: Option<*mut LocalHeap>,
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

            const K_MAX_TICKS: usize = 100_000_000;
            base::macros::USE!(K_MAX_TICKS);
            assert!(self.ticks_ < K_MAX_TICKS);

            if let Some(local_heap_ptr) = self.local_heap_ {
                unsafe {
                    (*local_heap_ptr).Safepoint();
                }
            }
        }

        pub fn attach_local_heap(&mut self, local_heap: *mut LocalHeap) {
            assert!(self.local_heap_.is_none());
            self.local_heap_ = Some(local_heap);
            assert!(self.local_heap_.is_some());
        }

        pub fn detach_local_heap(&mut self) {
            self.local_heap_ = None;
        }

        pub fn current_ticks(&self) -> usize {
            self.ticks_
        }
    }
}
