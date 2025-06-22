use std::sync::Mutex;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::boxed::Box;

/// A trait representing a task that can be run.
pub trait Task: Send + 'static {
    fn run(self: Box<Self>);
}

/// A queue for tasks to be executed by worker threads.
pub struct TaskQueue {
    queue: Arc<Mutex<Vec<Box<dyn Task>>>>,
}

impl TaskQueue {
    /// Creates a new TaskQueue.
    pub fn new() -> Self {
        TaskQueue {
            queue: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Adds a task to the queue.
    pub fn add_task(&self, task: Box<dyn Task>) {
        let mut queue = self.queue.lock().unwrap();
        queue.push(task);
    }

    /// Retrieves the next task from the queue.  Returns `None` if the queue is empty.
    pub fn get_next(&self) -> Option<Box<dyn Task>> {
        let mut queue = self.queue.lock().unwrap();
        if queue.is_empty() {
            None
        } else {
            Some(queue.remove(0))
        }
    }
}

/// A worker thread that executes tasks from a TaskQueue.
pub struct WorkerThread {
    thread: Option<JoinHandle<()>>,
    queue: Arc<TaskQueue>,
}

impl WorkerThread {
    /// Creates a new WorkerThread that executes tasks from the given TaskQueue.
    pub fn new(queue: Arc<TaskQueue>) -> Self {
        let queue_clone = queue.clone();
        let thread = thread::spawn(move || {
            loop {
                match queue_clone.get_next() {
                    Some(task) => {
                        task.run();
                    }
                    None => {
                        //Queue is empty so thread stops here
                        break;
                    }
                }
            }
        });

        WorkerThread {
            thread: Some(thread),
            queue,
        }
    }

    /// Waits for the worker thread to finish executing all tasks in the queue.
    pub fn join(mut self) {
        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
    }
}