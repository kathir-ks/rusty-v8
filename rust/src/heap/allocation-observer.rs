// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/heap/allocation-observer.h (converted to module definition)
pub mod allocation_observer {
    use std::cmp;
    use std::collections::HashSet;
    use std::ops::Deref;

    // Re-export the types used in the public API
    pub use super::heap::Heap;

    #[derive(Debug)]
    struct AllocationObserverCounter<'a> {
        observer_: &'a dyn AllocationObserver,
        prev_counter_: usize,
        next_counter_: usize,
    }

    trait AllocationObserver {
        fn step(&self, current_counter: i32, soon_object: usize, object_size: usize);
        fn get_next_step_size(&self) -> i64;
    }

    #[derive(Debug)]
    pub struct AllocationCounter<'a> {
        observers_: Vec<AllocationObserverCounter<'a>>,
        current_counter_: usize,
        next_counter_: usize,
        step_in_progress_: bool,
        pending_added_: Vec<AllocationObserverCounter<'a>>,
        pending_removed_: HashSet<&'a dyn AllocationObserver>,
    }

    impl<'a> AllocationCounter<'a> {
        pub fn new() -> Self {
            AllocationCounter {
                observers_: Vec::new(),
                current_counter_: 0,
                next_counter_: 0,
                step_in_progress_: false,
                pending_added_: Vec::new(),
                pending_removed_: HashSet::new(),
            }
        }

        pub fn add_allocation_observer(&mut self, observer: &'a dyn AllocationObserver) {
            #[cfg(debug_assertions)]
            {
                assert!(self.observers_.iter().find(|aoc| aoc.observer_ as *const _ == observer as *const _).is_none());
            }

            if self.step_in_progress_ {
                self.pending_added_.push(AllocationObserverCounter {
                    observer_: observer,
                    prev_counter_: 0,
                    next_counter_: 0,
                });
                return;
            }

            let step_size = observer.get_next_step_size();
            let observer_next_counter = self.current_counter_ as i64 + step_size;

            self.observers_.push(AllocationObserverCounter {
                observer_: observer,
                prev_counter_: self.current_counter_,
                next_counter_: observer_next_counter as usize,
            });

            if self.observers_.len() == 1 {
                assert_eq!(self.current_counter_, self.next_counter_);
                self.next_counter_ = observer_next_counter as usize;
            } else {
                let missing_bytes = self.next_counter_ as i64 - self.current_counter_ as i64;
                self.next_counter_ = (self.current_counter_ as i64 + cmp::min(missing_bytes, step_size)) as usize;
            }
        }

        pub fn remove_allocation_observer(&mut self, observer: &'a dyn AllocationObserver) {
            let it = self.observers_.iter().position(|aoc| aoc.observer_ as *const _ == observer as *const _);
            
            match it {
                Some(index) => {
                    if self.step_in_progress_ {
                        assert!(!self.pending_removed_.contains(observer));
                        self.pending_removed_.insert(observer);
                        return;
                    }

                    self.observers_.remove(index);

                    if self.observers_.is_empty() {
                        self.current_counter_ = 0;
                        self.next_counter_ = 0;
                    } else {
                        let mut step_size = 0;

                        for observer_counter in &self.observers_ {
                            let left_in_step = observer_counter.next_counter_ as i64 - self.current_counter_ as i64;
                            assert!(left_in_step > 0);
                            step_size = if step_size == 0 {
                                left_in_step
                            } else {
                                cmp::min(step_size, left_in_step)
                            };
                        }

                        self.next_counter_ = (self.current_counter_ as i64 + step_size) as usize;
                    }
                }
                None => {
                    panic!("Observer not found");
                }
            }
        }

        pub fn advance_allocation_observers(&mut self, allocated: usize) {
            if self.observers_.is_empty() {
                return;
            }
            assert!(!self.step_in_progress_);
            assert!(allocated < self.next_counter_ - self.current_counter_);
            self.current_counter_ += allocated;
        }

        pub fn invoke_allocation_observers(&mut self, soon_object: usize, object_size: usize, aligned_object_size: usize) {
            if self.observers_.is_empty() {
                return;
            }
            assert!(!self.step_in_progress_);
            assert!(aligned_object_size >= self.next_counter_ - self.current_counter_);
            assert!(soon_object != 0);
            let mut step_run = false;
            self.step_in_progress_ = true;
            let mut step_size = 0;

            assert!(self.pending_added_.is_empty());
            assert!(self.pending_removed_.is_empty());

            for aoc in &mut self.observers_ {
                if (aoc.next_counter_ as i64 - self.current_counter_ as i64) <= aligned_object_size as i64 {
                    // {
                    //     DisallowGarbageCollection no_gc;
                    aoc.observer_.step(
                        (self.current_counter_ as i64 - aoc.prev_counter_ as i64) as i32,
                        soon_object,
                        object_size,
                    );
                    // }
                    let observer_step_size = aoc.observer_.get_next_step_size();

                    aoc.prev_counter_ = self.current_counter_;
                    aoc.next_counter_ = (self.current_counter_ as i64 + aligned_object_size as i64 + observer_step_size) as usize;
                    step_run = true;
                }

                let left_in_step = aoc.next_counter_ as i64 - self.current_counter_ as i64;
                step_size = if step_size == 0 {
                    left_in_step
                } else {
                    cmp::min(step_size, left_in_step)
                };
            }

            assert!(step_run);

            // Now process newly added allocation observers.
            for aoc in &mut self.pending_added_ {
                assert_eq!(0, aoc.next_counter_);
                let observer_step_size = aoc.observer_.get_next_step_size();
                aoc.prev_counter_ = self.current_counter_;
                aoc.next_counter_ = (self.current_counter_ as i64 + aligned_object_size as i64 + observer_step_size) as usize;

                assert_ne!(step_size, 0);
                step_size = cmp::min(step_size, aligned_object_size as i64 + observer_step_size);

                self.observers_.push(AllocationObserverCounter {
                    observer_: aoc.observer_,
                    prev_counter_: aoc.prev_counter_,
                    next_counter_: aoc.next_counter_,
                });
            }

            self.pending_added_.clear();

            if !self.pending_removed_.is_empty() {
                self.observers_.retain(|aoc| !self.pending_removed_.contains(aoc.observer_));
                self.pending_removed_.clear();

                // Some observers were removed, recalculate step size.
                step_size = 0;
                for aoc in &self.observers_ {
                    let left_in_step = aoc.next_counter_ as i64 - self.current_counter_ as i64;
                    step_size = if step_size == 0 {
                        left_in_step
                    } else {
                        cmp::min(step_size, left_in_step)
                    };
                }

                if self.observers_.is_empty() {
                    self.next_counter_ = 0;
                    self.current_counter_ = 0;
                    self.step_in_progress_ = false;
                    return;
                }
            }

            self.next_counter_ = (self.current_counter_ as i64 + step_size) as usize;
            self.step_in_progress_ = false;
        }
    }

    /// RAII-style helper to pause allocation observers during GC.
    #[derive(Debug)]
    pub struct PauseAllocationObserversScope<'a> {
        heap_: &'a mut Heap,
    }

    impl<'a> PauseAllocationObserversScope<'a> {
        pub fn new(heap: &'a mut Heap) -> Self {
            // assert_eq!(heap.gc_state(), Heap::NOT_IN_GC);
            heap.allocator().pause_allocation_observers();
            heap.pause_allocation_observers_depth_ += 1;
            PauseAllocationObserversScope { heap_: heap }
        }
    }

    impl<'a> Drop for PauseAllocationObserversScope<'a> {
        fn drop(&mut self) {
            self.heap_.pause_allocation_observers_depth_ -= 1;
            self.heap_.allocator().resume_allocation_observers();
        }
    }

    // Dummy structs and methods for compilation, to be replaced with the
    // actual Heap and Allocator implementations.
    pub mod heap {
        #[derive(Debug)]
        pub struct Heap {
            allocator_: Allocator,
            pub pause_allocation_observers_depth_: i32,
            gc_state_: i32, // Added gc_state_ field
        }
    
        impl Heap {
            pub fn new(allocator: Allocator) -> Self {
                Heap {
                    allocator_: allocator,
                    pause_allocation_observers_depth_: 0,
                    gc_state_: 0, // Initialize gc_state_ to 0
                }
            }
            pub fn allocator(&mut self) -> &mut Allocator {
                &mut self.allocator_
            }
            pub fn gc_state(&self) -> i32 {
                self.gc_state_
            }
        }
    }

    #[derive(Debug)]
    pub struct Allocator {
        paused_: bool,
    }

    impl Allocator {
        pub fn new() -> Self {
            Allocator {
                paused_: false,
            }
        }
        pub fn pause_allocation_observers(&mut self) {
            self.paused_ = true;
        }
        pub fn resume_allocation_observers(&mut self) {
            self.paused_ = false;
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[derive(Debug)]
        struct MockAllocationObserver {
            step_size: i64,
            step_called: bool,
        }
    
        impl MockAllocationObserver {
            fn new(step_size: i64) -> Self {
                MockAllocationObserver {
                    step_size,
                    step_called: false,
                }
            }
        }
    
        impl AllocationObserver for MockAllocationObserver {
            fn step(&self, current_counter: i32, soon_object: usize, object_size: usize) {
                println!("Step called with counter: {}, object: {}, size: {}", current_counter, soon_object, object_size);
                //self.step_called = true; //Does not compile with &self
            }
    
            fn get_next_step_size(&self) -> i64 {
                self.step_size
            }
        }

        #[test]
        fn test_add_remove_observers() {
            let mut counter = AllocationCounter::new();
            let observer1 = MockAllocationObserver::new(100);
            let observer2 = MockAllocationObserver::new(200);

            counter.add_allocation_observer(&observer1);
            counter.add_allocation_observer(&observer2);
            assert_eq!(counter.observers_.len(), 2);

            counter.remove_allocation_observer(&observer1);
            assert_eq!(counter.observers_.len(), 1);

            counter.remove_allocation_observer(&observer2);
            assert_eq!(counter.observers_.len(), 0);
        }

        #[test]
        fn test_invoke_observers() {
            let mut counter = AllocationCounter::new();
            let observer1 = MockAllocationObserver::new(100);
            let soon_object = 0x1000; // Example address
            let object_size = 50;
            let aligned_object_size = 64;

            counter.add_allocation_observer(&observer1);
            counter.invoke_allocation_observers(soon_object, object_size, aligned_object_size);
            //assert!(observer1.step_called);  //Does not compile with &self
        }
    
        #[test]
        fn test_pause_allocation_observers_scope() {
            let allocator = Allocator::new();
            let mut heap = Heap::new(allocator);
    
            {
                let _pause_scope = PauseAllocationObserversScope::new(&mut heap);
                assert_eq!(heap.allocator().paused_, true);
            }
    
            assert_eq!(heap.allocator().paused_, false);
        }
    }
}