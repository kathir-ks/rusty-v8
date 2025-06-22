use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::{Duration, Instant};
use std::collections::VecDeque;

// Assuming base::Thread::Priority maps to a similar concept in Rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThreadPriority {
    Normal,
    High, // Or other levels as appropriate
}

// Mock for SourceLocation as it's likely debugging information
#[derive(Debug, Clone)]
pub struct SourceLocation {}

type Task = Box<dyn FnOnce() + Send + 'static>;
type IdleTask = Box<dyn FnOnce() + Send + 'static>;
type TimeFunction = Arc<dyn Fn() -> f64 + Send + Sync>;

struct TaskEntry {
    task: Task,
}

struct DelayedTaskEntry {
    task: Task,
    delay: f64,
    scheduled_time: f64,
}

struct DelayedTaskQueue {
    queue: Mutex<VecDeque<DelayedTaskEntry>>,
    time_function: TimeFunction,
    terminated: Mutex<bool>,
    condvar: Condvar,
}

impl DelayedTaskQueue {
    fn new(time_function: TimeFunction) -> Self {
        DelayedTaskQueue {
            queue: Mutex::new(VecDeque::new()),
            time_function,
            terminated: Mutex::new(false),
            condvar: Condvar::new(),
        }
    }

    fn append(&self, task: Task) {
        self.append_delayed(task, 0.0);
    }

    fn append_delayed(&self, task: Task, delay: f64) {
        let scheduled_time = (self.time_function)();
        let entry = DelayedTaskEntry {
            task,
            delay,
            scheduled_time: scheduled_time + delay,
        };

        let mut queue = self.queue.lock().unwrap();
        queue.push_back(entry);
        self.condvar.notify_one();
    }

    enum MaybeNextTask {
        Task(Task),
        Terminated,
        WaitIndefinite,
        WaitDelayed(f64),
    }

    fn try_get_next(&self) -> MaybeNextTask {
        let mut queue = self.queue.lock().unwrap();
        if *self.terminated.lock().unwrap() {
            return MaybeNextTask::Terminated;
        }

        loop {
            match queue.pop_front() {
                Some(entry) => {
                    let now = (self.time_function)();
                    if entry.scheduled_time <= now {
                        return MaybeNextTask::Task(entry.task);
                    } else {
                        // Re-insert the entry and wait
                        queue.push_front(entry);
                        let wait_time = entry.scheduled_time - now;
                        return MaybeNextTask::WaitDelayed(wait_time);
                    }
                }
                None => {
                    if *self.terminated.lock().unwrap() {
                        return MaybeNextTask::Terminated;
                    }
                    return MaybeNextTask::WaitIndefinite;
                }
            }
        }
    }

    fn terminate(&self) {
        *self.terminated.lock().unwrap() = true;
        self.condvar.notify_all();
    }
}

pub struct DefaultWorkerThreadsTaskRunner {
    queue_: DelayedTaskQueue,
    time_function_: TimeFunction,
    thread_pool_: Mutex<Vec<WorkerThread>>,
    lock_: Mutex<()>,
    idle_threads_: Mutex<Vec<Arc<WorkerThread>>>,
    terminated_: Mutex<bool>,
}

impl DefaultWorkerThreadsTaskRunner {
    pub fn new(thread_pool_size: u32, time_function: TimeFunction, priority: ThreadPriority) -> Self {
        let runner = DefaultWorkerThreadsTaskRunner {
            queue_: DelayedTaskQueue::new(time_function.clone()),
            time_function_: time_function.clone(),
            thread_pool_: Mutex::new(Vec::new()),
            lock_: Mutex::new(()),
            idle_threads_: Mutex::new(Vec::new()),
            terminated_: Mutex::new(false),
        };

        let mut thread_pool = runner.thread_pool_.lock().unwrap();
        for _ in 0..thread_pool_size {
            thread_pool.push(WorkerThread::new(Arc::new(runner.clone()), priority));
        }

        runner
    }

    pub fn monotonically_increasing_time(&self) -> f64 {
        (self.time_function_)()
    }

    pub fn terminate(&self) {
        let _guard = self.lock_.lock().unwrap();
        *self.terminated_.lock().unwrap() = true;
        self.queue_.terminate();
        self.idle_threads_.lock().unwrap().clear();

        // Clearing the thread pool lets all worker threads join.
        self.thread_pool_.lock().unwrap().clear();
    }

    pub fn post_task(&self, task: Task, _location: &SourceLocation) {
        self.post_task_impl(task, _location);
    }

    fn post_task_impl(&self, task: Task, _location: &SourceLocation) {
        let _guard = self.lock_.lock().unwrap();
        if *self.terminated_.lock().unwrap() {
            return;
        }
        self.queue_.append(task);

        let mut idle_threads = self.idle_threads_.lock().unwrap();
        if let Some(thread) = idle_threads.pop() {
            thread.notify();
        }
    }

    pub fn post_delayed_task(&self, task: Task, delay_in_seconds: f64, _location: &SourceLocation) {
        self.post_delayed_task_impl(task, delay_in_seconds, _location);
    }

    fn post_delayed_task_impl(&self, task: Task, delay_in_seconds: f64, _location: &SourceLocation) {
        let _guard = self.lock_.lock().unwrap();
        if *self.terminated_.lock().unwrap() {
            return;
        }
        self.queue_.append_delayed(task, delay_in_seconds);

        let mut idle_threads = self.idle_threads_.lock().unwrap();
        if let Some(thread) = idle_threads.pop() {
            thread.notify();
        }
    }

    pub fn post_idle_task(&self, _task: IdleTask, _location: &SourceLocation) {
        // There are no idle worker tasks.
        panic!("UNREACHABLE");
    }

    pub fn idle_tasks_enabled(&self) -> bool {
        // There are no idle worker tasks.
        false
    }
}

impl Clone for DefaultWorkerThreadsTaskRunner {
    fn clone(&self) -> Self {
        DefaultWorkerThreadsTaskRunner {
            queue_: DelayedTaskQueue::new(self.time_function_.clone()),
            time_function_: self.time_function_.clone(),
            thread_pool_: Mutex::new(Vec::new()),
            lock_: Mutex::new(()),
            idle_threads_: Mutex::new(Vec::new()),
            terminated_: Mutex::new(false),
        }
    }
}

struct WorkerThread {
    thread: Option<thread::JoinHandle<()>>,
    runner_: Arc<DefaultWorkerThreadsTaskRunner>,
    condition_var_: Arc<Condvar>,
}

impl WorkerThread {
    fn new(runner_: Arc<DefaultWorkerThreadsTaskRunner>, priority: ThreadPriority) -> Self {
        let runner = runner_.clone();
        let condition_var_ = Arc::new(Condvar::new());
        let condition_var = condition_var_.clone();

        let thread = thread::spawn(move || {
            let condvar = condition_var;
            let runner = runner;
            loop {
                let next_task = runner.queue_.try_get_next();
                match next_task {
                    DelayedTaskQueue::MaybeNextTask::Task(task) => {
                        (task)();
                    }
                    DelayedTaskQueue::MaybeNextTask::Terminated => {
                        break;
                    }
                    DelayedTaskQueue::MaybeNextTask::WaitIndefinite => {
                        let mut idle_threads = runner.idle_threads_.lock().unwrap();
                        idle_threads.push(Arc::new(WorkerThread { thread: None, runner_: runner.clone(), condition_var_: condvar.clone() }));
                        drop(idle_threads);

                        let _guard = runner.lock_.lock().unwrap();
                        let _ = condvar.wait(_guard).unwrap();
                    }
                    DelayedTaskQueue::MaybeNextTask::WaitDelayed(wait_time) => {
                        let mut idle_threads = runner.idle_threads_.lock().unwrap();
                        idle_threads.push(Arc::new(WorkerThread { thread: None, runner_: runner.clone(), condition_var_: condvar.clone() }));
                        drop(idle_threads);

                        let duration = Duration::from_secs_f64(wait_time);
                        let _guard = runner.lock_.lock().unwrap();
                        let ( _guard_ret, timeout_result) = condvar.wait_timeout(_guard, duration).unwrap();
                    }
                }
            }
        });

        WorkerThread {
            thread: Some(thread),
            runner_: runner_,
            condition_var_: condition_var_,
        }
    }

    fn notify(&self) {
        self.condition_var_.notify_all();
    }
}

impl Drop for WorkerThread {
    fn drop(&mut self) {
        self.condition_var_.notify_all();
        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
    }
}