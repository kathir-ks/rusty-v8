// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::time::{Duration, Instant};

use v8::{IdleTask, Task, TaskRunner};

pub mod v8 {
    pub struct Task {
        // Opaque Task struct
    }
    pub struct IdleTask {
        // Opaque IdleTask struct
    }
    pub trait TaskRunner {
        fn idle_tasks_enabled(&self) -> bool;
        fn non_nestable_tasks_enabled(&self) -> bool;
        fn post_task(&self, task: Box<dyn FnOnce()>); // Example
    }

    #[derive(Clone, Copy)]
    pub enum MessageLoopBehavior {
        //Placeholder
        Default,
    }
}

pub mod base {
    pub mod platform {
        use std::sync::{Condvar, Mutex};

        pub struct MutexWrapper {
            mutex: Mutex<()>,
        }

        impl MutexWrapper {
            pub fn new() -> Self {
                MutexWrapper {
                    mutex: Mutex::new(()),
                }
            }

            pub fn lock(&self) -> MutexGuard<()> {
                self.mutex.lock().unwrap()
            }
        }

        pub struct ConditionVariable {
            condvar: Condvar,
        }

        impl ConditionVariable {
            pub fn new() -> Self {
                ConditionVariable {
                    condvar: Condvar::new(),
                }
            }

            pub fn notify_one(&self) {
                self.condvar.notify_one();
            }

            pub fn wait<'a, T>(&self, guard: MutexGuard<'a, T>) -> MutexGuard<'a, T> {
                self.condvar.wait(guard).unwrap()
            }
        }
    }
}

pub mod libplatform {
    pub enum IdleTaskSupport {
        Enabled,
        Disabled,
    }
}

use libplatform::IdleTaskSupport;

pub struct SourceLocation {} // Placeholder

#[derive(Clone)]
pub struct DefaultForegroundTaskRunner {
    inner: Arc<DefaultForegroundTaskRunnerInner>,
}

struct DefaultForegroundTaskRunnerInner {
    terminated: Mutex<bool>,
    mutex: base::platform::MutexWrapper,
    event_loop_control: base::platform::ConditionVariable,
    nesting_depth: Mutex<i32>,
    task_queue: Mutex<VecDeque<TaskQueueEntry>>,
    idle_task_support: IdleTaskSupport,
    idle_task_queue: Mutex<Vec<Box<dyn FnOnce() + Send + Sync>>>, // IdleTask>,  //TODO Replace Box<dyn FnOnce...> with correct type
    delayed_task_queue: Mutex<BinaryHeap<DelayedEntry>>,
    time_function: fn() -> f64,
}

type TaskQueueEntry = (Nestability, Box<dyn FnOnce() + Send + Sync>); //Task>>
#[derive(PartialEq, Eq, Copy, Clone)]
enum Nestability {
    Nestable,
    NonNestable,
}

impl DefaultForegroundTaskRunner {
    pub type TimeFunction = fn() -> f64;

    pub struct RunTaskScope {
        task_runner: DefaultForegroundTaskRunner,
    }

    impl RunTaskScope {
        pub fn new(task_runner: DefaultForegroundTaskRunner) -> Self {
            let mut nesting_depth = task_runner.inner.nesting_depth.lock().unwrap();
            *nesting_depth += 1;
            RunTaskScope { task_runner }
        }
    }

    impl Drop for RunTaskScope {
        fn drop(&mut self) {
            let mut nesting_depth = self.task_runner.inner.nesting_depth.lock().unwrap();
            *nesting_depth -= 1;
        }
    }

    pub fn new(idle_task_support: IdleTaskSupport, time_function: fn() -> f64) -> Self {
        DefaultForegroundTaskRunner {
            inner: Arc::new(DefaultForegroundTaskRunnerInner {
                terminated: Mutex::new(false),
                mutex: base::platform::MutexWrapper::new(),
                event_loop_control: base::platform::ConditionVariable::new(),
                nesting_depth: Mutex::new(0),
                task_queue: Mutex::new(VecDeque::new()),
                idle_task_support,
                idle_task_queue: Mutex::new(Vec::new()),
                delayed_task_queue: Mutex::new(BinaryHeap::new()),
                time_function,
            }),
        }
    }

    pub fn terminate(&self) {
        let lock = self.inner.mutex.lock();
        let mut terminated = self.inner.terminated.lock().unwrap();
        *terminated = true;
        self.inner.event_loop_control.notify_one();
        drop(lock);
    }

    pub fn pop_task_from_queue(
        self: &DefaultForegroundTaskRunner,
        wait_for_work: v8::MessageLoopBehavior,
    ) -> Option<Box<dyn FnOnce() + Send + Sync>> { //Task>> {
        let lock = self.inner.mutex.lock();
        let task = self.pop_task_from_queue_locked(wait_for_work, lock);
        task
    }

    fn pop_task_from_queue_locked(
        &self,
        wait_for_work: v8::MessageLoopBehavior,
        mut lock: std::sync::MutexGuard<()>,
    ) -> Option<Box<dyn FnOnce() + Send + Sync>> { //Task>> {
        loop {
            if let Some(task) = self.pop_task_from_delayed_queue_locked(&mut Nestability::Nestable) {
                return Some(task);
            }

            if self.has_poppable_task_in_queue() {
                let mut task_queue = self.inner.task_queue.lock().unwrap();
                let (nestability, task) = task_queue.pop_front().unwrap();
                return Some(task);
            }

            let terminated = *self.inner.terminated.lock().unwrap();
            if terminated {
                return None;
            }

            match wait_for_work {
                v8::MessageLoopBehavior::Default => {
                    lock = self.inner.event_loop_control.wait(lock);
                }
            }
        }
    }

    pub fn pop_task_from_idle_queue(&self) -> Option<Box<dyn FnOnce() + Send + Sync>> { //IdleTask>> {
        let mut idle_task_queue = self.inner.idle_task_queue.lock().unwrap();
        idle_task_queue.pop()
    }

    pub fn monotonically_increasing_time(&self) -> f64 {
        (self.inner.time_function)()
    }

    fn wait_for_task_locked(&self) {
        let lock = self.inner.mutex.lock();
        let terminated = *self.inner.terminated.lock().unwrap();
        if terminated {
            return;
        }
        self.inner.event_loop_control.wait(lock);
    }

    fn post_task_locked(
        &self,
        task: Box<dyn FnOnce() + Send + Sync>, //Task>,
        nestability: Nestability,
    ) -> Option<Box<dyn FnOnce() + Send + Sync>> {  //Task>> {
        let mut terminated = self.inner.terminated.lock().unwrap();
        if *terminated {
            return Some(task);
        }

        let mut task_queue = self.inner.task_queue.lock().unwrap();
        task_queue.push_back((nestability, task));
        self.inner.event_loop_control.notify_one();
        None
    }

    fn post_delayed_task_locked(
        &self,
        task: Box<dyn FnOnce() + Send + Sync>, //Task>,
        delay_in_seconds: f64,
        nestability: Nestability,
    ) {
        let timeout_time = self.monotonically_increasing_time() + delay_in_seconds;
        let mut delayed_task_queue = self.inner.delayed_task_queue.lock().unwrap();
        delayed_task_queue.push(DelayedEntry {
            timeout_time,
            nestability,
            task,
        });
    }

    fn pop_task_from_delayed_queue_locked(
        &self,
        nestability: &mut Nestability,
    ) -> Option<Box<dyn FnOnce() + Send + Sync>> { //Task>> {
        let now = self.monotonically_increasing_time();
        let mut delayed_task_queue = self.inner.delayed_task_queue.lock().unwrap();

        if let Some(entry) = delayed_task_queue.peek() {
            if entry.timeout_time <= now {
                let entry = delayed_task_queue.pop().unwrap();
                *nestability = entry.nestability;
                return Some(entry.task);
            }
        }
        None
    }

    fn has_poppable_task_in_queue(&self) -> bool {
        let nesting_depth = *self.inner.nesting_depth.lock().unwrap();
        let task_queue = self.inner.task_queue.lock().unwrap();

        task_queue.iter().any(|(nestability, _)| match nestability {
            Nestability::Nestable => true,
            Nestability::NonNestable => nesting_depth == 0,
        })
    }

    fn move_expired_delayed_tasks_locked(&self) -> Vec<Box<dyn FnOnce() + Send + Sync>> { //Task>> {
        let now = self.monotonically_increasing_time();
        let mut delayed_task_queue = self.inner.delayed_task_queue.lock().unwrap();
        let mut expired_tasks = Vec::new();

        while let Some(entry) = delayed_task_queue.peek() {
            if entry.timeout_time <= now {
                let entry = delayed_task_queue.pop().unwrap();
                let terminated = *self.inner.terminated.lock().unwrap();
                if terminated {
                    expired_tasks.push(entry.task);
                } else {
                    let mut task_queue = self.inner.task_queue.lock().unwrap();
                    task_queue.push_back((entry.nestability, entry.task));
                }
            } else {
                break;
            }
        }

        expired_tasks
    }
}

impl TaskRunner for DefaultForegroundTaskRunner {
    fn idle_tasks_enabled(&self) -> bool {
        match self.inner.idle_task_support {
            IdleTaskSupport::Enabled => true,
            IdleTaskSupport::Disabled => false,
        }
    }

    fn non_nestable_tasks_enabled(&self) -> bool {
        true // Placeholder
    }

    fn post_task(&self, task: Box<dyn FnOnce()>) { // Placeholder, this is just an example of what a real implementation might look like.
        let lock = self.inner.mutex.lock();
        self.post_task_locked(Box::new(move || task()), Nestability::Nestable);
    }
}

impl DefaultForegroundTaskRunner {
    fn post_task_impl(
        &self,
        task: Box<dyn FnOnce() + Send + Sync>, //Task>,
        location: &SourceLocation,
    ) {
        let lock = self.inner.mutex.lock();
        self.post_task_locked(task, Nestability::Nestable);
    }

    fn post_delayed_task_impl(
        &self,
        task: Box<dyn FnOnce() + Send + Sync>, //Task>,
        delay_in_seconds: f64,
        location: &SourceLocation,
    ) {
        let lock = self.inner.mutex.lock();
        self.post_delayed_task_locked(task, delay_in_seconds, Nestability::Nestable);
    }

    fn post_idle_task_impl(
        &self,
        task: Box<dyn FnOnce() + Send + Sync>, //IdleTask>,
        location: &SourceLocation,
    ) {
        let mut idle_task_queue = self.inner.idle_task_queue.lock().unwrap();
        idle_task_queue.push(task);
    }

    fn post_non_nestable_task_impl(
        &self,
        task: Box<dyn FnOnce() + Send + Sync>, //Task>,
        location: &SourceLocation,
    ) {
        let lock = self.inner.mutex.lock();
        self.post_task_locked(task, Nestability::NonNestable);
    }

    fn post_non_nestable_delayed_task_impl(
        &self,
        task: Box<dyn FnOnce() + Send + Sync>, //Task>,
        delay_in_seconds: f64,
        location: &SourceLocation,
    ) {
        let lock = self.inner.mutex.lock();
        self.post_delayed_task_locked(task, delay_in_seconds, Nestability::NonNestable);
    }
}

#[derive(PartialEq, PartialOrd)]
struct DelayedEntry {
    timeout_time: f64,
    nestability: Nestability,
    task: Box<dyn FnOnce() + Send + Sync>, //Task>,
}

impl Eq for DelayedEntry {}

impl Ord for DelayedEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse the order to make it a min-heap
        self.timeout_time.partial_cmp(&other.timeout_time).unwrap_or(Ordering::Equal).reverse()
    }
}