// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file provides a Rust equivalent of the C++
// HighAllocationThroughputScope.

use std::ops::{Drop};

/// Observer trait for high allocation throughput events.
///
/// This trait should mirror the `v8::Platform::HighAllocationThroughputObserver` interface.
pub trait HighAllocationThroughputObserver {
    /// Called when entering a section with high allocation throughput.
    fn enter_section(&mut self);
    /// Called when leaving a section with high allocation throughput.
    fn leave_section(&mut self);
}

/// Platform trait to fetch the HighAllocationThroughputObserver.
///
/// This trait mirrors the `v8::Platform` to retrieve the `HighAllocationThroughputObserver`.
pub trait Platform {
    /// Returns a mutable reference to the `HighAllocationThroughputObserver`.
    fn get_high_allocation_throughput_observer(&mut self) -> &mut dyn HighAllocationThroughputObserver;
}

/// Scope that notifies embedder's observer about entering sections with high
/// throughput of malloc/free operations.
pub struct HighAllocationThroughputScope<'a> {
    observer: &'a mut dyn HighAllocationThroughputObserver,
}

impl<'a> HighAllocationThroughputScope<'a> {
    /// Creates a new `HighAllocationThroughputScope`.
    ///
    /// # Arguments
    ///
    /// * `platform`: A mutable reference to the `Platform` trait object.
    pub fn new<P: Platform>(platform: &'a mut P) -> Self {
        let observer = platform.get_high_allocation_throughput_observer();
        observer.leave_section();

        HighAllocationThroughputScope { observer }
    }
}

impl<'a> Drop for HighAllocationThroughputScope<'a> {
    fn drop(&mut self) {
        self.observer.enter_section();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockObserver {
        enter_count: usize,
        leave_count: usize,
    }

    impl HighAllocationThroughputObserver for MockObserver {
        fn enter_section(&mut self) {
            self.enter_count += 1;
        }

        fn leave_section(&mut self) {
            self.leave_count += 1;
        }
    }

    struct MockPlatform {
        observer: MockObserver,
    }

    impl Platform for MockPlatform {
        fn get_high_allocation_throughput_observer(&mut self) -> &mut dyn HighAllocationThroughputObserver {
            &mut self.observer
        }
    }

    #[test]
    fn test_scope() {
        let mut mock_observer = MockObserver { enter_count: 0, leave_count: 0 };
        let mut mock_platform = MockPlatform { observer: mock_observer };

        assert_eq!(mock_platform.observer.enter_count, 0);
        assert_eq!(mock_platform.observer.leave_count, 0);

        {
            let _scope = HighAllocationThroughputScope::new(&mut mock_platform);
            assert_eq!(mock_platform.observer.enter_count, 0);
            assert_eq!(mock_platform.observer.leave_count, 1);
        }

        assert_eq!(mock_platform.observer.enter_count, 1);
        assert_eq!(mock_platform.observer.leave_count, 1);
    }
}