// Converted from V8 C++ source files:
// Header: cancelable-task.h
// Implementation: cancelable-task.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

use std::sync::{Arc, Mutex, Condvar, atomic::{AtomicU64, AtomicUsize, Ordering}, Weak};
use std::collections::HashMap;

pub struct CancelableTaskManager {
    task_id_counter: AtomicU64,
    cancelable_tasks: Mutex<HashMap<u64, Weak<Cancelable>>>,
    cancelable_tasks_barrier: Condvar,
    mutex: Mutex<()>,
    canceled: Mutex<bool>,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum TryAbortResult {
    TaskRemoved,
    TaskRunning,
    TaskAborted,
}

impl CancelableTaskManager {
    pub const K_INVALID_TASK_ID: u64 = 0;

    pub fn new() -> Self {
        CancelableTaskManager {
            task_id_counter: AtomicU64::new(Self::K_INVALID_TASK_ID),
            cancelable_tasks: Mutex::new(HashMap::new()),
            cancelable_tasks_barrier: Condvar::new(),
            mutex: Mutex::new(()),
            canceled: Mutex::new(false),
        }
    }

    pub fn register(&self, task: Arc<Cancelable>) -> u64 {
        let _guard = self.mutex.lock().unwrap();
        if *self.canceled.lock().unwrap() {
            task.cancel();
            return Self::K_INVALID_TASK_ID;
        }

        let mut id = self.task_id_counter.fetch_add(1, Ordering::Relaxed);
        while id == Self::K_INVALID_TASK_ID {
            id = self.task_id_counter.fetch_add(1, Ordering::Relaxed);
        }
        
        let mut tasks = self.cancelable_tasks.lock().unwrap();
        tasks.insert(id, Arc::downgrade(&task));
        id
    }

    pub fn try_abort(&self, id: u64) -> TryAbortResult {
        if id == Self::K_INVALID_TASK_ID {
            return TryAbortResult::TaskRemoved;
        }

        let _guard = self.mutex.lock().unwrap();
        let mut tasks = self.cancelable_tasks.lock().unwrap();
        if let Some(weak_task) = tasks.get(&id) {
            if let Some(task) = weak_task.upgrade() {
                if task.cancel() {
                    tasks.remove(&id);
                    self.cancelable_tasks_barrier.notify_one();
                    return TryAbortResult::TaskAborted;
                } else {
                    return TryAbortResult::TaskRunning;
                }
            } else {
                tasks.remove(&id);
                self.cancelable_tasks_barrier.notify_one();
                return TryAbortResult::TaskRemoved;
            }
        }
        TryAbortResult::TaskRemoved
    }

    pub fn try_abort_all(&self) -> TryAbortResult {
        let _guard = self.mutex.lock().unwrap();
        let mut tasks = self.cancelable_tasks.lock().unwrap();

        if tasks.is_empty() {
            return TryAbortResult::TaskRemoved;
        }

        let mut all_aborted = true;
        tasks.retain(|&id, weak_task| {
            if let Some(task) = weak_task.upgrade() {
                if task.cancel() {
                    false // Remove the task
                } else {
                    all_aborted = false;
                    true // Keep the task
                }
            } else {
                false
            }
        });
        
        if tasks.is_empty() {
            TryAbortResult::TaskAborted
        } else {
            TryAbortResult::TaskRunning
        }
    }

    pub fn cancel_and_wait(&self) {
        let _guard = self.mutex.lock().unwrap();
        *self.canceled.lock().unwrap() = true;

        let mut tasks = self.cancelable_tasks.lock().unwrap();
        while !tasks.is_empty() {
            tasks.retain(|&id, weak_task| {
                if let Some(task) = weak_task.upgrade() {
                    !task.cancel()
                } else {
                    false
                }
            });

            if !tasks.is_empty() {
                self.cancelable_tasks_barrier.wait(&mut self.mutex.lock().unwrap());
            }
        }
    }

    pub fn canceled(&self) -> bool {
        *self.canceled.lock().unwrap()
    }

    fn remove_finished_task(&self, id: u64) {
        if id == Self::K_INVALID_TASK_ID {
            return;
        }

        let _guard = self.mutex.lock().unwrap();
        let mut tasks = self.cancelable_tasks.lock().unwrap();
        let removed = tasks.remove(&id).is_some();
        
        if removed {
            self.cancelable_tasks_barrier.notify_one();
        }
    }
}

use std::sync::atomic::AtomicU8;

pub struct Cancelable {
    parent: Arc<CancelableTaskManager>,
    status: AtomicU8,
    id: u64,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Status {
    Waiting,
    Canceled,
    Running,
}

impl Cancelable {
    pub fn new(parent: Arc<CancelableTaskManager>) -> Arc<Self> {
        let cancelable = Arc::new(Cancelable {
            parent: parent.clone(),
            status: AtomicU8::new(Status::Waiting as u8),
            id: 0,
        });
        let id = parent.register(cancelable.clone());
        Arc::get_mut(&mut Arc::try_unwrap(cancelable).unwrap()).unwrap().id = id;
        let cancelable = Arc::new(Cancelable {
            parent: parent.clone(),
            status: AtomicU8::new(Status::Waiting as u8),
            id: id,
        });
        let id = parent.register(cancelable.clone());
        Arc::get_mut(&mut Arc::try_unwrap(cancelable).unwrap()).unwrap().id = id;
        let cancelable = Arc::new(Cancelable {
            parent: parent.clone(),
            status: AtomicU8::new(Status::Waiting as u8),
            id: id,
        });
         Arc::new(Cancelable {
            parent: parent.clone(),
            status: AtomicU8::new(Status::Waiting as u8),
            id: id,
        })
    }
    
    fn id(&self) -> u64 {
        self.id
    }

    fn try_run(&self) -> bool {
        let mut current = Status::Waiting as u8;
        let desired = Status::Running as u8;
        self.status.compare_exchange(current, desired, Ordering::AcqRel, Ordering::Acquire).is_ok()
    }

    fn cancel(&self) -> bool {
        let mut current = Status::Waiting as u8;
        let desired = Status::Canceled as u8;
        self.status.compare_exchange(current, desired, Ordering::AcqRel, Ordering::Acquire).is_ok()
    }
}

impl Drop for Cancelable {
    fn drop(&mut self) {
        let mut previous = Status::Waiting;
        if self.try_run() || previous == Status::Running {
            self.parent.remove_finished_task(self.id);
        }
    }
}

pub trait Task {
    fn run(&self);
}

pub struct CancelableTask<T: Task> {
    cancelable: Arc<Cancelable>,
    task: T,
}

impl<T: Task> CancelableTask<T> {
    pub fn new(manager: Arc<CancelableTaskManager>, task: T) -> Self {
        let cancelable = Cancelable::new(manager);
        CancelableTask {
            cancelable,
            task,
        }
    }
    
    pub fn run_internal(&self) {
        self.task.run();
    }
}

impl<T: Task> Task for CancelableTask<T> {
    fn run(&self) {
        if self.cancelable.try_run() {
            self.run_internal();
        }
    }
}

pub trait IdleTask {
    fn run(&self, deadline_in_seconds: f64);
}

pub struct CancelableIdleTask<T: IdleTask> {
    cancelable: Arc<Cancelable>,
    idle_task: T,
}

impl<T: IdleTask> CancelableIdleTask<T> {
    pub fn new(manager: Arc<CancelableTaskManager>, idle_task: T) -> Self {
        let cancelable = Cancelable::new(manager);
        CancelableIdleTask {
            cancelable,
            idle_task,
        }
    }

    fn run_internal(&self, deadline_in_seconds: f64) {
        self.idle_task.run(deadline_in_seconds);
    }
}

impl<T: IdleTask> IdleTask for CancelableIdleTask<T> {
    fn run(&self, deadline_in_seconds: f64) {
        if self.cancelable.try_run() {
            self.run_internal(deadline_in_seconds);
        }
    }
}
