// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod stress_scavenge_observer {
    use std::cell::Cell;

    /// Trait representing the Heap interface required by StressScavengeObserver.
    /// This allows for mocking the Heap in tests.
    pub trait HeapTrait {
        fn scavenge(&mut self, reason: &str); // Changed Reason enum to string slice for simplicity
        fn new_space_capacity(&self) -> usize;
    }

    /// Represents a simple Heap structure for demonstration.  Replace with a proper
    /// implementation if needed.
    pub struct Heap {
        new_space_capacity: usize,
    }

    impl Heap {
        pub fn new(new_space_capacity: usize) -> Self {
            Heap { new_space_capacity }
        }
    }

    impl HeapTrait for Heap {
        fn scavenge(&mut self, reason: &str) {
            println!("Scavenging due to: {}", reason);
        }

        fn new_space_capacity(&self) -> usize {
            self.new_space_capacity
        }
    }

    pub trait AllocationObserver {
        fn step(&mut self, bytes_allocated: i32, soon_object: usize, size: usize);
    }

    /// Observes allocations and triggers scavenges based on configured limits.
    pub struct StressScavengeObserver<'a> {
        heap_: &'a mut dyn HeapTrait,
        limit_percentage_: Cell<i32>,
        has_requested_gc_: Cell<bool>,
        max_new_space_size_reached_: Cell<f64>,
    }

    impl<'a> StressScavengeObserver<'a> {
        /// Creates a new `StressScavengeObserver`.
        pub fn new(heap: &'a mut dyn HeapTrait) -> Self {
            StressScavengeObserver {
                heap_: heap,
                limit_percentage_: Cell::new(0),
                has_requested_gc_: Cell::new(false),
                max_new_space_size_reached_: Cell::new(0.0),
            }
        }

        /// Checks if a GC has been requested.
        pub fn has_requested_gc(&self) -> bool {
            self.has_requested_gc_.get()
        }

        /// Marks a requested GC as done.
        pub fn requested_gc_done(&self) {
            self.has_requested_gc_.set(false);
        }

        /// Gets the maximum new space size reached.
        pub fn max_new_space_size_reached(&self) -> f64 {
            self.max_new_space_size_reached_.get()
        }

        fn next_limit(&self, min: i32) -> i32 {
            // Simple implementation for demonstration.
            let next = (min + 10) % 100;
            next
        }
    }

    impl<'a> AllocationObserver for StressScavengeObserver<'a> {
        /// Called on each allocation step.
        fn step(&mut self, bytes_allocated: i32, _soon_object: usize, _size: usize) {
            let new_space_capacity = self.heap_.new_space_capacity() as f64;
            let limit_percentage = self.limit_percentage_.get() as f64;
            let current_size_reached = (bytes_allocated as f64 / new_space_capacity) * 100.0;

            self.max_new_space_size_reached_.set(
                f64::max(self.max_new_space_size_reached_.get(), current_size_reached)
            );

            if current_size_reached > limit_percentage && !self.has_requested_gc_.get() {
                self.has_requested_gc_.set(true);
                self.heap_.scavenge("StressScavengeObserver");
                let next_limit = self.next_limit(self.limit_percentage_.get());
                self.limit_percentage_.set(next_limit);
            }
        }
    }
}