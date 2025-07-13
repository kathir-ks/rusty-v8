// Converted from V8 C++ source files:
// Header: allocation-observer.h
// Implementation: allocation-observer.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/heap/allocation-observer.h
use std::collections::HashSet;
use std::sync::{Mutex, MutexGuard};

// src/common/globals.h
// Defined a dummy struct Address and Heap for compilation purposes.
#[derive(Debug, Clone, Copy)]
pub struct Address {
    address: usize,
}

#[derive(Debug)]
pub struct Heap {
    allocator_: Allocator,
    gc_state_: i32,
    pause_allocation_observers_depth_: i32,
}

impl Heap {
    pub const NOT_IN_GC: i32 = 0;
    pub fn gc_state(&self) -> i32 {
        self.gc_state_
    }

    pub fn allocator(&mut self) -> &mut Allocator {
        &mut self.allocator_
    }
}

// Dummy implementation for Allocator
#[derive(Debug)]
pub struct Allocator {
    allocation_observers_paused_: bool,
}

impl Allocator {
    pub fn pause_allocation_observers(&mut self) {
        self.allocation_observers_paused_ = true;
    }

    pub fn resume_allocation_observers(&mut self) {
        self.allocation_observers_paused_ = false;
    }
}

#[derive(Debug)]
pub struct DisallowGarbageCollection {}

impl DisallowGarbageCollection {
    pub fn new() -> Self {
        DisallowGarbageCollection {}
    }
}

impl Drop for DisallowGarbageCollection {
    fn drop(&mut self) {}
}
pub trait Step {
    fn step(&mut self, bytes_allocated: i32, soon_object: Address, size: usize);
}

pub struct AllocationObserver {
    step_size_: i64,
}

impl AllocationObserver {
    pub const K_NOT_USING_FIXED_STEP_SIZE: i64 = -1;

    pub fn new(step_size: i64) -> Self {
        AllocationObserver { step_size_: step_size }
    }

    pub fn get_next_step_size(&self) -> i64 {
        assert_ne!(Self::K_NOT_USING_FIXED_STEP_SIZE, self.step_size_);
        self.step_size_
    }
}

// #[derive(Debug)]
// pub struct AllocationObserverCounter<'a> {
//     observer_: &'a mut dyn Step,
//     prev_counter_: usize,
//     next_counter_: usize,
// }

// impl<'a> AllocationObserverCounter<'a> {
//     pub fn new(observer: &'a mut dyn Step, prev_counter: usize, next_counter: usize) -> Self {
//         AllocationObserverCounter {
//             observer_: observer,
//             prev_counter_: prev_counter,
//             next_counter_: next_counter,
//         }
//     }
// }

#[derive(Debug)]
pub struct AllocationObserverCounter {
    observer_: *mut dyn Step,
    prev_counter_: usize,
    next_counter_: usize,
}

impl AllocationObserverCounter {
    pub fn new(observer: *mut dyn Step, prev_counter: usize, next_counter: usize) -> Self {
        AllocationObserverCounter {
            observer_: observer,
            prev_counter_: prev_counter,
            next_counter_: next_counter,
        }
    }
}

#[derive(Debug)]
pub struct AllocationCounter {
    observers_: Vec<AllocationObserverCounter>,
    pending_added_: Vec<AllocationObserverCounter>,
    pending_removed_: HashSet<*mut dyn Step>,
    current_counter_: usize,
    next_counter_: usize,
    step_in_progress_: bool,
    mutex: Mutex<()>,
}

impl AllocationCounter {
    pub fn new() -> Self {
        AllocationCounter {
            observers_: Vec::new(),
            pending_added_: Vec::new(),
            pending_removed_: HashSet::new(),
            current_counter_: 0,
            next_counter_: 0,
            step_in_progress_: false,
            mutex: Mutex::new(()),
        }
    }

    pub fn add_allocation_observer(&mut self, observer: *mut dyn Step) {
        let _lock: MutexGuard<'_, ()> = self.mutex.lock().unwrap();

        #[cfg(debug_assertions)]
        {
            if self
                .observers_
                .iter()
                .any(|aoc| aoc.observer_ as *mut dyn Step == observer)
            {
                panic!("Observer already exists");
            }
        }

        if self.step_in_progress_ {
            self.pending_added_.push_back(AllocationObserverCounter::new(
                observer,
                0,
                0,
            ));
            return;
        }

        let step_size = unsafe { (*(observer)).step_size() } as usize;
        let observer_next_counter = self.current_counter_ + step_size;

        self.observers_.push_back(AllocationObserverCounter::new(
            observer,
            self.current_counter_,
            observer_next_counter,
        ));

        if self.observers_.len() == 1 {
            assert_eq!(self.current_counter_, self.next_counter_);
            self.next_counter_ = observer_next_counter;
        } else {
            let missing_bytes = self.next_counter_ - self.current_counter_;
            self.next_counter_ = self.current_counter_
                + std::cmp::min(missing_bytes as usize, step_size);
        }
    }

    pub fn remove_allocation_observer(&mut self, observer: *mut dyn Step) {
        let _lock: MutexGuard<'_, ()> = self.mutex.lock().unwrap();

        let it = self
            .observers_
            .iter()
            .position(|aoc| aoc.observer_ as *mut dyn Step == observer);
        assert!(it.is_some());

        if self.step_in_progress_ {
            assert!(!self.pending_removed_.contains(&observer));
            self.pending_removed_.insert(observer);
            return;
        }

        self.observers_.remove(it.unwrap());

        if self.observers_.is_empty() {
            self.current_counter_ = self.next_counter_ = 0;
        } else {
            let mut step_size = 0;

            for observer_counter in &self.observers_ {
                let left_in_step = observer_counter.next_counter_ - self.current_counter_;
                assert!(left_in_step > 0);
                step_size = if step_size == 0 {
                    left_in_step
                } else {
                    std::cmp::min(step_size, left_in_step)
                };
            }

            self.next_counter_ = self.current_counter_ + step_size;
        }
    }

    pub fn advance_allocation_observers(&mut self, allocated: usize) {
        let _lock: MutexGuard<'_, ()> = self.mutex.lock().unwrap();

        if self.observers_.is_empty() {
            return;
        }
        assert!(!self.step_in_progress_);
        assert!(allocated < self.next_counter_ - self.current_counter_);
        self.current_counter_ += allocated;
    }

    pub fn invoke_allocation_observers(
        &mut self,
        soon_object: Address,
        object_size: usize,
        aligned_object_size: usize,
    ) {
        let _lock: MutexGuard<'_, ()> = self.mutex.lock().unwrap();

        if self.observers_.is_empty() {
            return;
        }
        assert!(!self.step_in_progress_);
        assert!(aligned_object_size >= self.next_counter_ - self.current_counter_);
        assert!(soon_object.address != 0);

        let mut step_run = false;
        self.step_in_progress_ = true;
        let mut step_size = 0;

        assert!(self.pending_added_.is_empty());
        assert!(self.pending_removed_.is_empty());

        for aoc in &mut self.observers_ {
            if aoc.next_counter_ - self.current_counter_ <= aligned_object_size {
                {
                    let _no_gc = DisallowGarbageCollection::new();
                    unsafe {
                        (*(aoc.observer_)).step(
                            (self.current_counter_ - aoc.prev_counter_) as i32,
                            soon_object,
                            object_size,
                        );
                    }
                }

                let observer_step_size = unsafe { (*(aoc.observer_)).step_size() } as usize;

                aoc.prev_counter_ = self.current_counter_;
                aoc.next_counter_ =
                    self.current_counter_ + aligned_object_size + observer_step_size;

                step_run = true;
            }

            let left_in_step = aoc.next_counter_ - self.current_counter_;
            step_size = if step_size == 0 {
                left_in_step
            } else {
                std::cmp::min(step_size, left_in_step)
            };
        }

        assert!(step_run);

        // Now process newly added allocation observers.
        for aoc in &mut self.pending_added_ {
            assert_eq!(0, aoc.next_counter_);
            let observer_step_size = unsafe { (*(aoc.observer_)).step_size() } as usize;
            aoc.prev_counter_ = self.current_counter_;
            aoc.next_counter_ =
                self.current_counter_ + aligned_object_size + observer_step_size;

            assert_ne!(step_size, 0);
            step_size = std::cmp::min(
                step_size,
                aligned_object_size + observer_step_size,
            );

            self.observers_.push(aoc.clone());
        }

        self.pending_added_.clear();

        if !self.pending_removed_.is_empty() {
            self.observers_.retain(|aoc| {
                !self.pending_removed_.contains(&(aoc.observer_ as *mut dyn Step))
            });
            self.pending_removed_.clear();

            // Some observers were removed, recalculate step size.
            step_size = 0;
            for aoc in &self.observers_ {
                let left_in_step = aoc.next_counter_ - self.current_counter_;
                step_size = if step_size == 0 {
                    left_in_step
                } else {
                    std::cmp::min(step_size, left_in_step)
                };
            }

            if self.observers_.is_empty() {
                self.next_counter_ = self.current_counter_ = 0;
                self.step_in_progress_ = false;
                return;
            }
        }

        self.next_counter_ = self.current_counter_ + step_size;
        self.step_in_progress_ = false;
    }

    pub fn is_step_in_progress(&self) -> bool {
        self.step_in_progress_
    }

    pub fn next_bytes(&self) -> usize {
        if self.observers_.is_empty() {
            usize::MAX
        } else {
            self.next_counter_ - self.current_counter_
        }
    }

    #[cfg(debug_assertions)]
    pub fn has_allocation_observers(&self) -> bool {
        !self.observers_.is_empty()
            || !self.pending_added_.is_empty()
            || !self.pending_removed_.is_empty()
    }
}

impl Clone for AllocationObserverCounter {
    fn clone(&self) -> Self {
        AllocationObserverCounter {
            observer_: self.observer_,
            prev_counter_: self.prev_counter_,
            next_counter_: self.next_counter_,
        }
    }
}

pub struct PauseAllocationObserversScope<'a> {
    heap_: &'a mut Heap,
}

impl<'a> PauseAllocationObserversScope<'a> {
    pub fn new(heap: &'a mut Heap) -> Self {
        assert_eq!(heap.gc_state(), Heap::NOT_IN_GC);
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

// Dummy trait and impls to make AllocationObserverCounter work
pub trait AllocationStep {
    fn step_size(&self) -> i64;
}

impl dyn Step {
    pub fn step_size(&self) -> i64 {
        0 // Provide a default value
    }
}
