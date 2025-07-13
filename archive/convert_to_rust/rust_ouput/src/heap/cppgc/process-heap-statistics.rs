// Converted from V8 C++ source files:
// Header: process-heap-statistics.h
// Implementation: process-heap-statistics.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod cppgc {
pub mod internal {
use std::sync::atomic::{AtomicUsize, Ordering};

    pub struct ProcessHeapStatisticsUpdater {}

    impl ProcessHeapStatisticsUpdater {
        pub fn increase_total_allocated_object_size(delta: usize) {
            unsafe {
                super::ProcessHeapStatistics::total_allocated_object_size_
                    .fetch_add(delta, Ordering::Relaxed);
            }
        }

        pub fn decrease_total_allocated_object_size(delta: usize) {
            unsafe {
                super::ProcessHeapStatistics::total_allocated_object_size_
                    .fetch_sub(delta, Ordering::Relaxed);
            }
        }

        pub fn increase_total_allocated_space(delta: usize) {
            unsafe {
                super::ProcessHeapStatistics::total_allocated_space_
                    .fetch_add(delta, Ordering::Relaxed);
            }
        }

        pub fn decrease_total_allocated_space(delta: usize) {
            unsafe {
                super::ProcessHeapStatistics::total_allocated_space_
                    .fetch_sub(delta, Ordering::Relaxed);
            }
        }
    }

    pub struct AllocationObserverImpl {
        object_size_changes_since_last_reset_: usize,
    }

    impl AllocationObserverImpl {
        pub fn new() -> Self {
            AllocationObserverImpl {
                object_size_changes_since_last_reset_: 0,
            }
        }
        pub fn allocated_object_size_increased(&mut self, bytes: usize) {
            ProcessHeapStatisticsUpdater::increase_total_allocated_object_size(bytes);
            self.object_size_changes_since_last_reset_ += bytes;
        }

        pub fn allocated_object_size_decreased(&mut self, bytes: usize) {
            ProcessHeapStatisticsUpdater::decrease_total_allocated_object_size(bytes);
            self.object_size_changes_since_last_reset_ -= bytes;
        }

        pub fn reset_allocated_object_size(&mut self, bytes: usize) {
            ProcessHeapStatisticsUpdater::decrease_total_allocated_object_size(
                self.object_size_changes_since_last_reset_,
            );
            ProcessHeapStatisticsUpdater::increase_total_allocated_object_size(bytes);
            self.object_size_changes_since_last_reset_ = bytes;
        }

        pub fn allocated_size_increased(&mut self, bytes: usize) {
            ProcessHeapStatisticsUpdater::increase_total_allocated_space(bytes);
        }

        pub fn allocated_size_decreased(&mut self, bytes: usize) {
            ProcessHeapStatisticsUpdater::decrease_total_allocated_space(bytes);
        }
    }
}

    pub struct ProcessHeapStatistics {
        pub total_allocated_space_: AtomicUsize,
        pub total_allocated_object_size_: AtomicUsize,
    }

    impl ProcessHeapStatistics {
        pub const fn new() -> Self {
            ProcessHeapStatistics {
                total_allocated_space_: AtomicUsize::new(0),
                total_allocated_object_size_: AtomicUsize::new(0),
            }
        }
    }

    impl Default for ProcessHeapStatistics {
        fn default() -> Self {
            Self::new()
        }
    }

    static mut PROCESS_HEAP_STATISTICS: ProcessHeapStatistics = ProcessHeapStatistics::new();

    impl ProcessHeapStatistics {
        pub fn get() -> &'static mut Self {
            unsafe { &mut PROCESS_HEAP_STATISTICS }
        }
    }

    pub static mut total_allocated_space_: AtomicUsize = AtomicUsize::new(0);
    pub static mut total_allocated_object_size_: AtomicUsize = AtomicUsize::new(0);
}
