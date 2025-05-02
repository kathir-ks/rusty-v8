use std::sync::{Arc, Mutex, Condvar};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread;
use std::time::Duration;

/// A task to be executed.  Corresponds to `v8::Task`.
pub trait Task: Send {
    fn run(&mut self);
}

pub struct TaskQueue {
    lock: Arc<Mutex<TaskQueueInternal>>,
    process_queue_semaphore: Arc<Condvar>,
}

struct TaskQueueInternal {
    task_queue: Vec<Box<dyn Task>>,
    terminated: bool,
}

impl TaskQueue {
    pub fn new() -> Self {
        TaskQueue {
            lock: Arc::new(Mutex::new(TaskQueueInternal {
                task_queue: Vec::new(),
                terminated: false,
            })),
            process_queue_semaphore: Arc::new(Condvar::new()),
        }
    }

    pub fn append(&self, task: Box<dyn Task>) {
        let mut guard = self.lock.lock().unwrap();
        assert!(!guard.terminated);
        guard.task_queue.push(task);
        self.process_queue_semaphore.notify_one();
    }

    pub fn get_next(&self) -> Option<Box<dyn Task>> {
        loop {
            let mut guard = self.lock.lock().unwrap();
            if !guard.task_queue.is_empty() {
                let result = guard.task_queue.remove(0);
                return Some(result);
            }
            if guard.terminated {
                self.process_queue_semaphore.notify_one();
                return None;
            }

            guard = self.process_queue_semaphore.wait(guard).unwrap();
        }
    }

    pub fn terminate(&self) {
        let mut guard = self.lock.lock().unwrap();
        assert!(!guard.terminated);
        guard.terminated = true;
        self.process_queue_semaphore.notify_one();
    }

    pub fn block_until_queue_empty_for_testing(&self) {
        loop {
            {
                let guard = self.lock.lock().unwrap();
                if guard.task_queue.is_empty() {
                    return;
                }
            }
            thread::sleep(Duration::from_millis(5));
        }
    }
}

impl Drop for TaskQueue {
    fn drop(&mut self) {
        let mut guard = self.lock.lock().unwrap();
        assert!(guard.terminated);
        assert!(guard.task_queue.is_empty());
    }
}