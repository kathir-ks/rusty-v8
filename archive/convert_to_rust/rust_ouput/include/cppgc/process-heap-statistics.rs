// Converted from V8 C++ source files:
// Header: process-heap-statistics.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod cppgc {
    pub mod internal {
        pub struct ProcessHeapStatisticsUpdater {}
    }

    use std::sync::atomic::{AtomicUsize, Ordering};

    pub struct ProcessHeapStatistics {}

    impl ProcessHeapStatistics {
        pub fn total_allocated_object_size() -> usize {
            static TOTAL_ALLOCATED_OBJECT_SIZE: AtomicUsize = AtomicUsize::new(0);
            TOTAL_ALLOCATED_OBJECT_SIZE.load(Ordering::Relaxed)
        }

        pub fn total_allocated_space() -> usize {
            static TOTAL_ALLOCATED_SPACE: AtomicUsize = AtomicUsize::new(0);
            TOTAL_ALLOCATED_SPACE.load(Ordering::Relaxed)
        }

        // Note: No public constructor or fields.  Instances cannot be created outside this module.
    }

    // Implementation for the friend class interaction
    impl ProcessHeapStatistics {
        pub(crate) fn update_total_allocated_space(new_value: usize) {
            static TOTAL_ALLOCATED_SPACE: AtomicUsize = AtomicUsize::new(0);
            TOTAL_ALLOCATED_SPACE.store(new_value, Ordering::Relaxed);
        }

        pub(crate) fn update_total_allocated_object_size(new_value: usize) {
            static TOTAL_ALLOCATED_OBJECT_SIZE: AtomicUsize = AtomicUsize::new(0);
            TOTAL_ALLOCATED_OBJECT_SIZE.store(new_value, Ordering::Relaxed);
        }
    }
}
