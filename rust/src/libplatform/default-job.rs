// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::sync::{Arc, Mutex, Condvar, Weak};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::thread;
use std::time::Duration;
use std::u8;
use std::any::Any;

pub trait JobDelegate {
    fn notify_concurrency_increase(&mut self);
    fn should_yield(&mut self) -> bool;
    fn get_task_id(&self) -> u8;
    fn is_joining_thread(&self) -> bool;
}

pub trait JobTask {
    fn run(&mut self, delegate: &mut dyn JobDelegate);
}

pub trait Platform {
    fn call_on_worker_thread(&mut self, priority: TaskPriority, task: Box<dyn Task>);
    fn get_max_concurrency(&self) -> usize;
}

pub trait Task {
    fn run(&mut self);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
  Highest,
  High,
  Normal,
  Low,
  Lowest,
}

pub trait JobHandle {
    fn notify_concurrency_increase(&mut self);
    fn join(&mut self);
    fn cancel(&mut self);
    fn cancel_and_detach(&mut self);
    fn is_active(&self) -> bool;
    fn is_valid(&self) -> bool;
    fn update_priority_enabled(&self) -> bool;
    fn update_priority(&mut self, priority: TaskPriority);
}

pub struct DefaultJobState {
    platform: Box<dyn Platform>,
    job_task: Mutex<Option<Box<dyn JobTask + Send>>>,
    mutex: Mutex<DefaultJobStateInternal>,
    priority: Mutex<TaskPriority>,
    is_canceled: AtomicBool,
    num_worker_threads: usize,
    worker_released_condition: Condvar,
    assigned_task_ids: AtomicU32,
}

struct DefaultJobStateInternal {
  active_workers: usize,
  pending_tasks: usize,
}

impl DefaultJobState {
    pub fn new(
        platform: Box<dyn Platform>,
        job_task: Box<dyn JobTask + Send>,
        priority: TaskPriority,
        num_worker_threads: usize,
    ) -> Arc<Self> {
        Arc::new(Self {
            platform,
            job_task: Mutex::new(Some(job_task)),
            mutex: Mutex::new(DefaultJobStateInternal { active_workers: 0, pending_tasks: 0 }),
            priority: Mutex::new(priority),
            is_canceled: AtomicBool::new(false),
            num_worker_threads,
            worker_released_condition: Condvar::new(),
            assigned_task_ids: AtomicU32::new(0),
        })
    }

    fn notify_concurrency_increase(&self) {
        let mut mutex = self.mutex.lock().unwrap();
        mutex.pending_tasks += 1;
        self.post_worker_task();
    }

    fn acquire_task_id(&self) -> u8 {
        let id = self.assigned_task_ids.fetch_add(1, Ordering::Relaxed);
        (id % (std::u8::MAX as u32)) as u8
    }

    fn release_task_id(&self, _task_id: u8) {
        //Not doing anything. Task ids are reused automatically.
    }

    fn join(&self) {
        let mut mutex = self.mutex.lock().unwrap();
        while mutex.active_workers > 0 || mutex.pending_tasks > 0 {
            mutex = self.worker_released_condition.wait(mutex).unwrap();
        }
    }

    fn cancel_and_wait(&self) {
        self.is_canceled.store(true, Ordering::Relaxed);
        self.join();
    }

    fn cancel_and_detach(&self) {
        self.is_canceled.store(true, Ordering::Relaxed);
        // Detach means doing nothing more.
    }

    fn is_active(&self) -> bool {
        let mutex = self.mutex.lock().unwrap();
        mutex.active_workers > 0 || mutex.pending_tasks > 0
    }

    fn can_run_first_task(&self) -> bool {
        let mut mutex = self.mutex.lock().unwrap();
        if self.is_canceled.load(Ordering::Relaxed) {
            return false;
        }

        if mutex.pending_tasks == 0 {
            return false;
        }

        mutex.pending_tasks -= 1;
        mutex.active_workers += 1;
        true
    }

    fn did_run_task(&self) -> bool {
        let mut mutex = self.mutex.lock().unwrap();
        if self.is_canceled.load(Ordering::Relaxed) {
            mutex.active_workers -= 1;
            self.worker_released_condition.notify_one();
            return false;
        }

        let capped_max_concurrency = self.capped_max_concurrency(self.num_worker_threads);
        if mutex.active_workers >= capped_max_concurrency {
            mutex.active_workers -= 1;
            self.worker_released_condition.notify_one();
            return false;
        }

        self.post_worker_task();
        true
    }

    fn update_priority(&self, priority: TaskPriority) {
        let mut current_priority = self.priority.lock().unwrap();
        *current_priority = priority;
    }

    fn capped_max_concurrency(&self, worker_count: usize) -> usize {
        std::cmp::min(self.platform.get_max_concurrency(), worker_count)
    }

    fn call_on_worker_thread(&self, priority: TaskPriority, task: Box<dyn Task>) {
        self.platform.call_on_worker_thread(priority, task);
    }

    fn post_worker_task(&self) {
        let job_task_mutex = self.job_task.lock().unwrap();
        let job_task = match job_task_mutex.as_ref() {
            Some(task) => task,
            None => return
        };
        let priority = *self.priority.lock().unwrap();
        let weak_self = Arc::downgrade(self);

        let worker = DefaultJobWorker::new(weak_self, job_task);
        self.call_on_worker_thread(priority, Box::new(worker));
    }
}

impl Drop for DefaultJobState {
    fn drop(&mut self) {
        // Ensure the job is canceled before dropping to avoid dangling pointers.
        self.cancel_and_wait();
    }
}

impl DefaultJobState {
    pub fn get_delegate(self: &Arc<Self>, is_joining_thread: bool) -> DefaultJobStateJobDelegate {
        DefaultJobStateJobDelegate::new(Arc::clone(self), is_joining_thread)
    }
}

pub struct DefaultJobStateJobDelegate {
    outer: Arc<DefaultJobState>,
    task_id: u8,
    is_joining_thread: bool,
    was_told_to_yield: Mutex<bool>
}

const K_INVALID_TASK_ID: u8 = std::u8::MAX;

impl DefaultJobStateJobDelegate {
    pub fn new(outer: Arc<DefaultJobState>, is_joining_thread: bool) -> Self {
        DefaultJobStateJobDelegate {
            outer,
            task_id: K_INVALID_TASK_ID,
            is_joining_thread,
            was_told_to_yield: Mutex::new(false),
        }
    }
}

impl JobDelegate for DefaultJobStateJobDelegate {
    fn notify_concurrency_increase(&mut self) {
        self.outer.notify_concurrency_increase();
    }

    fn should_yield(&mut self) -> bool {
        let mut was_told_to_yield = self.was_told_to_yield.lock().unwrap();

        if *was_told_to_yield {
           panic!("ShouldYield called twice")
        }

        *was_told_to_yield |= self.outer.is_canceled.load(Ordering::Relaxed);
        *was_told_to_yield
    }

    fn get_task_id(&self) -> u8 {
        self.task_id
    }

    fn is_joining_thread(&self) -> bool {
        self.is_joining_thread
    }
}

impl Drop for DefaultJobStateJobDelegate {
    fn drop(&mut self) {
        if self.task_id != K_INVALID_TASK_ID {
            self.outer.release_task_id(self.task_id);
        }
    }
}

pub struct DefaultJobHandle {
    state: Arc<DefaultJobState>,
}

impl DefaultJobHandle {
    pub fn new(state: Arc<DefaultJobState>) -> Self {
        DefaultJobHandle { state }
    }
}

impl JobHandle for DefaultJobHandle {
    fn notify_concurrency_increase(&mut self) {
        self.state.notify_concurrency_increase();
    }

    fn join(&mut self) {
        self.state.join();
    }

    fn cancel(&mut self) {
        self.state.cancel_and_wait();
    }

    fn cancel_and_detach(&mut self) {
        self.state.cancel_and_detach();
    }

    fn is_active(&self) -> bool {
        self.state.is_active()
    }

    fn is_valid(&self) -> bool {
        true // Always valid as long as the struct exists
    }

    fn update_priority_enabled(&self) -> bool {
        true
    }

    fn update_priority(&mut self, priority: TaskPriority) {
        self.state.update_priority(priority);
    }
}

pub struct DefaultJobWorker {
    state: Weak<DefaultJobState>,
    job_task: *mut dyn JobTask,
}

impl DefaultJobWorker {
    pub fn new(state: Weak<DefaultJobState>, job_task: &dyn JobTask) -> Self {
        DefaultJobWorker {
            state,
            job_task: job_task as *const dyn JobTask as *mut dyn JobTask,
        }
    }
}

impl Task for DefaultJobWorker {
    fn run(&mut self) {
        let shared_state = match self.state.upgrade() {
            Some(state) => state,
            None => return,
        };

        if !shared_state.can_run_first_task() {
            return;
        }

        loop {
            let mut delegate = shared_state.get_delegate(false);
            unsafe {
                (*self.job_task).run(&mut delegate);
            }

            if !shared_state.did_run_task() {
                break;
            }
        }
    }
}

unsafe impl Send for DefaultJobWorker {}
unsafe impl Sync for DefaultJobWorker {}