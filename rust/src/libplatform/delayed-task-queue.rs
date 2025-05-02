// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::sync::Mutex;
use std::time::Duration;

// TODO: Add logging crate
// use log::{debug, error, info, trace, warn};

// Placeholder for base::TimeDelta and TimeConstants
pub mod base {
    #[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
    pub struct TimeDelta {
        pub microseconds: i64,
    }

    impl TimeDelta {
        pub fn from_microseconds(microseconds: i64) -> Self {
            TimeDelta { microseconds }
        }
    }

    pub mod TimeConstants {
        pub const K_MICROSECONDS_PER_SECOND: i64 = 1_000_000;
    }
}

/// Represents a task to be executed.
pub trait Task: Send {
    fn run(&mut self);
}

/// A queue for managing delayed tasks.
pub struct DelayedTaskQueue {
    time_function: Box<dyn Fn() -> f64 + Send + Sync>,
    task_queue: Mutex<Vec<Box<dyn Task>>>,
    delayed_task_queue: Mutex<BinaryHeap<DelayedTask>>,
    terminated: Mutex<bool>,
}

impl DelayedTaskQueue {
    /// Creates a new `DelayedTaskQueue`.
    pub fn new(time_function: impl Fn() -> f64 + Send + Sync + 'static) -> Self {
        DelayedTaskQueue {
            time_function: Box::new(time_function),
            task_queue: Mutex::new(Vec::new()),
            delayed_task_queue: Mutex::new(BinaryHeap::new()),
            terminated: Mutex::new(false),
        }
    }

    /// Returns the current time using the provided time function.
    fn monotonically_increasing_time(&self) -> f64 {
        (self.time_function)()
    }

    /// Appends a task to the queue.
    pub fn append(&self, task: Box<dyn Task>) {
        let terminated = *self.terminated.lock().unwrap();
        assert!(!terminated);
        self.task_queue.lock().unwrap().push(task);
    }

    /// Appends a task to the queue with a specified delay.
    pub fn append_delayed(&self, task: Box<dyn Task>, delay_in_seconds: f64) {
        assert!(delay_in_seconds >= 0.0);
        let deadline = self.monotonically_increasing_time() + delay_in_seconds;
        {
            let terminated = *self.terminated.lock().unwrap();
            assert!(!terminated);
            self.delayed_task_queue
                .lock()
                .unwrap()
                .push(DelayedTask { deadline, task });
        }
    }

    /// Represents the possible results of trying to get the next task.
    pub enum MaybeNextTask {
        /// A task is available.
        Task(Box<dyn Task>),
        /// The queue has been terminated.
        Terminated,
        /// Wait for the specified duration before trying again.
        WaitDelayed(base::TimeDelta),
        /// Wait indefinitely for a new task.
        WaitIndefinite,
    }

    /// Tries to get the next task from the queue.
    pub fn try_get_next(&self) -> MaybeNextTask {
        loop {
            // Move delayed tasks that have hit their deadline to the main queue.
            let now = self.monotonically_increasing_time();
            loop {
                let task = self.pop_task_from_delayed_queue(now);
                if let Some(task) = task {
                    self.task_queue.lock().unwrap().push(task);
                } else {
                    break;
                }
            }

            let mut task_queue = self.task_queue.lock().unwrap();
            if !task_queue.is_empty() {
                let task = task_queue.remove(0);
                return MaybeNextTask::Task(task);
            }

            let terminated = *self.terminated.lock().unwrap();
            if terminated {
                return MaybeNextTask::Terminated;
            }

            let delayed_task_queue = self.delayed_task_queue.lock().unwrap();
            if task_queue.is_empty() && !delayed_task_queue.is_empty() {
                // Wait for the next delayed task or a newly posted task.
                let wait_in_seconds = delayed_task_queue.peek().unwrap().deadline - now;
                return MaybeNextTask::WaitDelayed(base::TimeDelta::from_microseconds(
                    (base::TimeConstants::K_MICROSECONDS_PER_SECOND as f64 * wait_in_seconds) as i64,
                ));
            } else {
                return MaybeNextTask::WaitIndefinite;
            }
        }
    }

    /// Pops a task from the delayed queue if its deadline has passed.
    fn pop_task_from_delayed_queue(&self, now: f64) -> Option<Box<dyn Task>> {
        let mut delayed_task_queue = self.delayed_task_queue.lock().unwrap();
        if delayed_task_queue.is_empty() {
            return None;
        }

        if delayed_task_queue.peek().unwrap().deadline > now {
            return None;
        }

        delayed_task_queue.pop().map(|dt| dt.task)
    }

    /// Terminates the queue.
    pub fn terminate(&self) {
        let mut terminated = self.terminated.lock().unwrap();
        assert!(!*terminated);
        *terminated = true;
    }
}

impl Drop for DelayedTaskQueue {
    fn drop(&mut self) {
        let terminated = *self.terminated.lock().unwrap();
        assert!(terminated);
        assert!(self.task_queue.lock().unwrap().is_empty());
    }
}

/// Represents a delayed task with a deadline.
struct DelayedTask {
    deadline: f64,
    task: Box<dyn Task>,
}

impl PartialEq for DelayedTask {
    fn eq(&self, other: &Self) -> bool {
        self.deadline == other.deadline
    }
}

impl Eq for DelayedTask {}

impl PartialOrd for DelayedTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DelayedTask {
    fn cmp(&self, other: &Self) -> Ordering {
        // BinaryHeap is a max-heap, so we reverse the order to get a min-heap.
        other.deadline.partial_cmp(&self.deadline).unwrap()
    }
}