// Copyright 2013 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::cmp;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::{collections::HashMap, thread};

//use crate::base::debug::stack_trace::StackTrace; // Assuming a similar Rust implementation
//use crate::base::logging; // Assuming a similar Rust logging implementation
//use crate::base::page_allocator::PageAllocator; // Assuming a similar Rust implementation
//use crate::base::platform::platform; // Assuming a similar Rust platform abstraction
//use crate::base::platform::time; // Assuming a similar Rust time abstraction
//use crate::base::sys_info; // Assuming a similar Rust system info abstraction
//use crate::libplatform::default_foreground_task_runner::DefaultForegroundTaskRunner; // Assuming a similar Rust implementation
//use crate::libplatform::default_job::DefaultJob; // Assuming a similar Rust implementation
//use crate::libplatform::default_worker_threads_task_runner::DefaultWorkerThreadsTaskRunner; // Assuming a similar Rust implementation

/// Represents a task to be executed.
trait Task {
    fn run(&mut self);
}

trait IdleTask {
    fn run(&mut self, deadline_in_seconds: f64);
}

trait JobTask {
    fn run(&mut self, job_handle: &dyn JobHandle, worker_id: usize);
}

trait TaskRunner {
    fn post_task(&self, task: Box<dyn Task + Send + Sync>);
    fn post_delayed_task(&self, task: Box<dyn Task + Send + Sync>, delay_in_seconds: f64);
}

trait JobHandle {
    fn is_cancelled(&self) -> bool;
    fn cancel(&self);
    fn join(&self);
}

enum TaskPriority {
    kBackground,
    kUserVisible,
    kBestEffort,
}

enum IdleTaskSupport {
    kEnabled,
    kDisabled,
}

enum InProcessStackDumping {
    kEnabled,
    kDisabled,
}

enum MessageLoopBehavior {
    kWaitForWork,
    kDoNotWait,
}

enum PriorityMode {
    kNormal,
    kHigh,
}

// Placeholder for v8::Isolate
struct Isolate {}

// Placeholder for v8::TracingController
struct TracingController {}

struct SourceLocation {}

type StackTracePrinter = fn();

// Assuming this is a redefinition of v8::Platform
trait Platform {
    fn get_foreground_task_runner(&self, isolate: &Isolate, priority: TaskPriority) -> Arc<dyn TaskRunner + Send + Sync>;
    fn post_task_on_worker_thread(&self, priority: TaskPriority, task: Box<dyn Task + Send + Sync>, location: SourceLocation);
    fn post_delayed_task_on_worker_thread(&self, priority: TaskPriority, task: Box<dyn Task + Send + Sync>, delay_in_seconds: f64, location: SourceLocation);
    fn idle_tasks_enabled(&self, isolate: &Isolate) -> bool;
    fn create_job(&self, priority: TaskPriority, job_task: Box<dyn JobTask + Send + Sync>, location: SourceLocation) -> Box<dyn JobHandle>;
    fn monotonically_increasing_time(&self) -> f64;
    fn current_clock_time_millis(&self) -> f64;
    fn get_tracing_controller(&self) -> &TracingController;
    fn set_tracing_controller(&mut self, tracing_controller: Box<TracingController>);
    fn number_of_worker_threads(&self) -> usize;
    fn get_stack_trace_printer(&self) -> StackTracePrinter;
    //fn get_page_allocator(&self) -> &PageAllocator;
    //fn get_thread_isolated_allocator(&self) -> Option<&ThreadIsolatedAllocator>;
    fn notify_isolate_shutdown(&self, isolate: &Isolate);
}

const K_MAX_THREAD_POOL_SIZE: usize = 16;

fn get_actual_thread_pool_size(thread_pool_size: i32) -> usize {
    assert!(thread_pool_size >= 0);
    let mut thread_pool_size = thread_pool_size as usize;
    if thread_pool_size < 1 {
        //thread_pool_size = sys_info::number_of_processors() - 1;
        thread_pool_size = num_cpus::get() - 1; //Using num_cpus crate as alternative
    }
    cmp::max(cmp::min(thread_pool_size, K_MAX_THREAD_POOL_SIZE), 1)
}

struct DefaultPlatform {
    thread_pool_size: usize,
    idle_task_support: IdleTaskSupport,
    tracing_controller: Box<TracingController>,
    //page_allocator: Box<PageAllocator>,
    priority_mode: PriorityMode,
    foreground_task_runner_map: Mutex<HashMap<*const Isolate, Arc<DefaultForegroundTaskRunner>>>,
    worker_threads_task_runners: [Option<Arc<DefaultWorkerThreadsTaskRunner>>; 4], // Assuming 4 priority levels
    lock: Mutex<()>,
    time_function_for_testing: Option<fn() -> f64>,
    //thread_isolated_allocator: ThreadIsolatedAllocator, //Not implemented
}

impl DefaultPlatform {
    fn new(
        thread_pool_size: i32,
        idle_task_support: IdleTaskSupport,
        tracing_controller: Box<TracingController>,
        priority_mode: PriorityMode,
    ) -> Self {
        let thread_pool_size = get_actual_thread_pool_size(thread_pool_size);
        let mut platform = DefaultPlatform {
            thread_pool_size,
            idle_task_support,
            tracing_controller,
            //page_allocator: Box::new(PageAllocator::new()), // Assuming a constructor
            priority_mode,
            foreground_task_runner_map: Mutex::new(HashMap::new()),
            worker_threads_task_runners: [None, None, None, None],
            lock: Mutex::new(()),
            time_function_for_testing: None,
            //thread_isolated_allocator: ThreadIsolatedAllocator::new(), //Not implemented
        };
        //controller->Initialize(nullptr);  // Assuming tracing controller initialization is handled elsewhere or not needed.
        if platform.thread_pool_size > 0 {
            platform.ensure_background_task_runner_initialized();
        }
        platform
    }

    fn new_single_threaded(
        idle_task_support: IdleTaskSupport,
        tracing_controller: Box<TracingController>,
    ) -> Self {
        DefaultPlatform {
            thread_pool_size: 0,
            idle_task_support,
            tracing_controller,
            //page_allocator: Box::new(PageAllocator::new()), // Assuming a constructor
            priority_mode: PriorityMode::kNormal,
            foreground_task_runner_map: Mutex::new(HashMap::new()),
            worker_threads_task_runners: [None, None, None, None],
            lock: Mutex::new(()),
            time_function_for_testing: None,
            //thread_isolated_allocator: ThreadIsolatedAllocator::new(), //Not implemented
        }
    }

    fn pump_message_loop(&self, isolate: &Isolate, wait_for_work: MessageLoopBehavior) -> bool {
        let failed_result = matches!(wait_for_work, MessageLoopBehavior::kWaitForWork);
        let task_runner: Arc<DefaultForegroundTaskRunner>;
        {
            let guard = self.foreground_task_runner_map.lock().unwrap();
            let isolate_ptr = isolate as *const Isolate;
            match guard.get(&isolate_ptr) {
                Some(tr) => task_runner = tr.clone(),
                None => return failed_result,
            }
        }

        let mut task = task_runner.pop_task_from_queue(wait_for_work);
        if task.is_none() {
            return failed_result;
        }

        let _scope = DefaultForegroundTaskRunner::RunTaskScope {task_runner: task_runner.clone()};
        task.unwrap().run();
        true
    }

    fn run_idle_tasks(&self, isolate: &Isolate, idle_time_in_seconds: f64) {
        match self.idle_task_support {
            IdleTaskSupport::kDisabled => return,
            IdleTaskSupport::kEnabled => {}
        }

        let task_runner: Arc<DefaultForegroundTaskRunner>;
        {
            let guard = self.foreground_task_runner_map.lock().unwrap();
            let isolate_ptr = isolate as *const Isolate;
            match guard.get(&isolate_ptr) {
                Some(tr) => task_runner = tr.clone(),
                None => return,
            }
        }

        let deadline_in_seconds = self.monotonically_increasing_time() + idle_time_in_seconds;

        while deadline_in_seconds > self.monotonically_increasing_time() {
            let mut task = task_runner.pop_task_from_idle_queue();
            if task.is_none() {
                return;
            }
            let _scope = DefaultForegroundTaskRunner::RunTaskScope { task_runner: task_runner.clone() };
            task.unwrap().run(deadline_in_seconds);
        }
    }

    fn ensure_background_task_runner_initialized(&mut self) {
        assert!(self.worker_threads_task_runners[0].is_none());
        for i in 0..self.num_worker_runners() {
            let priority = self.priority_from_index(i);
            self.worker_threads_task_runners[i] = Some(Arc::new(DefaultWorkerThreadsTaskRunner::new(
                self.thread_pool_size,
                self.time_function_for_testing.unwrap_or(default_time_function),
                priority,
            )));
        }
        assert!(self.worker_threads_task_runners[0].is_some());
    }

    fn set_time_function_for_testing(&mut self, time_function: fn() -> f64) {
        let _guard = self.lock.lock().unwrap();
        self.time_function_for_testing = Some(time_function);
        assert!(self.foreground_task_runner_map.lock().unwrap().is_empty());
    }

    fn num_worker_runners(&self) -> usize {
        4 // Assuming 4 priority levels
    }

    fn priority_from_index(&self, index: usize) -> TaskPriority {
        match index {
            0 => TaskPriority::kBackground,
            1 => TaskPriority::kUserVisible,
            2 => TaskPriority::kBestEffort,
            _ => TaskPriority::kBestEffort, // Default case, adjust as needed
        }
    }

    fn priority_to_index(&self, priority: TaskPriority) -> usize {
        match priority {
            TaskPriority::kBackground => 0,
            TaskPriority::kUserVisible => 1,
            TaskPriority::kBestEffort => 2,
        }
    }

    fn post_task_on_worker_thread_impl(
        &self,
        priority: TaskPriority,
        task: Box<dyn Task + Send + Sync>,
        _location: SourceLocation,
    ) {
        let index = self.priority_to_index(priority);
        let runner = self.worker_threads_task_runners[index].as_ref().unwrap();
        runner.post_task(task);
    }

    fn post_delayed_task_on_worker_thread_impl(
        &self,
        priority: TaskPriority,
        task: Box<dyn Task + Send + Sync>,
        delay_in_seconds: f64,
        _location: SourceLocation,
    ) {
        let index = self.priority_to_index(priority);
        let runner = self.worker_threads_task_runners[index].as_ref().unwrap();
        runner.post_delayed_task(task, delay_in_seconds);
    }
}

impl Drop for DefaultPlatform {
    fn drop(&mut self) {
        let _guard = self.lock.lock().unwrap();
        if self.worker_threads_task_runners[0].is_some() {
            for i in 0..self.num_worker_runners() {
                if let Some(runner) = &self.worker_threads_task_runners[i] {
                    runner.terminate();
                }
            }
        }

        let mut foreground_runners = self.foreground_task_runner_map.lock().unwrap();
        for (_, runner) in foreground_runners.iter() {
            runner.terminate();
        }
    }
}

fn default_time_function() -> f64 {
    let now = Instant::now(); // Replace with a more precise time source if needed
    let duration = now.elapsed();
    duration.as_secs_f64()
}

impl Platform for DefaultPlatform {
    fn get_foreground_task_runner(&self, isolate: &Isolate, priority: TaskPriority) -> Arc<dyn TaskRunner + Send + Sync> {
        let mut guard = self.foreground_task_runner_map.lock().unwrap();
        let isolate_ptr = isolate as *const Isolate;
        if !guard.contains_key(&isolate_ptr) {
            let task_runner = Arc::new(DefaultForegroundTaskRunner::new(
                self.idle_task_support,
                self.time_function_for_testing.unwrap_or(default_time_function),
            ));
            guard.insert(isolate_ptr, task_runner.clone());
        }
        guard.get(&isolate_ptr).unwrap().clone()
    }

    fn post_task_on_worker_thread(&self, priority: TaskPriority, task: Box<dyn Task + Send + Sync>, location: SourceLocation) {
        self.post_task_on_worker_thread_impl(priority, task, location);
    }

    fn post_delayed_task_on_worker_thread(&self, priority: TaskPriority, task: Box<dyn Task + Send + Sync>, delay_in_seconds: f64, location: SourceLocation) {
        self.post_delayed_task_on_worker_thread_impl(priority, task, delay_in_seconds, location);
    }

    fn idle_tasks_enabled(&self, _isolate: &Isolate) -> bool {
        matches!(self.idle_task_support, IdleTaskSupport::kEnabled)
    }

    fn create_job(&self, priority: TaskPriority, job_task: Box<dyn JobTask + Send + Sync>, location: SourceLocation) -> Box<dyn JobHandle> {
        let mut num_worker_threads = self.number_of_worker_threads();
        if matches!(priority, TaskPriority::kBestEffort) && num_worker_threads > 2 {
            num_worker_threads = 2;
        }
        new_default_job_handle(self, priority, job_task, num_worker_threads)
    }

    fn monotonically_increasing_time(&self) -> f64 {
        match self.time_function_for_testing {
            Some(f) => f(),
            None => default_time_function(),
        }
    }

    fn current_clock_time_millis(&self) -> f64 {
        let now = Instant::now(); // Replace with a more accurate clock if needed
        let duration = now.elapsed();
        duration.as_millis() as f64
    }

    fn get_tracing_controller(&self) -> &TracingController {
        &self.tracing_controller
    }

    fn set_tracing_controller(&mut self, tracing_controller: Box<TracingController>) {
        self.tracing_controller = tracing_controller;
    }

    fn number_of_worker_threads(&self) -> usize {
        self.thread_pool_size
    }

    fn get_stack_trace_printer(&self) -> StackTracePrinter {
        print_stack_trace //Assuming print_stack_trace is defined elsewhere.
    }
    /*
    fn get_page_allocator(&self) -> &PageAllocator {
        &self.page_allocator
    }
    */
    //fn get_thread_isolated_allocator(&self) -> Option<&ThreadIsolatedAllocator> { //Not implemented

    fn notify_isolate_shutdown(&self, isolate: &Isolate) {
        let mut taskrunner: Option<Arc<DefaultForegroundTaskRunner>> = None;
        {
            let mut guard = self.foreground_task_runner_map.lock().unwrap();
            let isolate_ptr = isolate as *const Isolate;
            if guard.contains_key(&isolate_ptr) {
                taskrunner = guard.remove(&isolate_ptr);
            }
        }
        if let Some(runner) = taskrunner {
            runner.terminate();
        }
    }
}

fn new_default_platform(
    thread_pool_size: i32,
    idle_task_support: IdleTaskSupport,
    _in_process_stack_dumping: InProcessStackDumping,
    tracing_controller: Box<TracingController>,
    priority_mode: PriorityMode,
) -> Box<dyn Platform> {
    //if in_process_stack_dumping == InProcessStackDumping::kEnabled {
    //    base::debug::EnableInProcessStackDumping(); // Assuming a similar Rust implementation
    //}
    let platform = DefaultPlatform::new(
        thread_pool_size,
        idle_task_support,
        tracing_controller,
        priority_mode,
    );
    Box::new(platform)
}

fn new_single_threaded_default_platform(
    idle_task_support: IdleTaskSupport,
    _in_process_stack_dumping: InProcessStackDumping,
    tracing_controller: Box<TracingController>,
) -> Box<dyn Platform> {
    //if in_process_stack_dumping == InProcessStackDumping::kEnabled {
    //    base::debug::EnableInProcessStackDumping(); // Assuming a similar Rust implementation
    //}
    let platform = DefaultPlatform::new_single_threaded(idle_task_support, tracing_controller);
    Box::new(platform)
}

struct DefaultJobState {
    platform: *const DefaultPlatform, // raw pointer to avoid lifetime issues with Platform trait
    job_task: Box<dyn JobTask + Send + Sync>,
    priority: TaskPriority,
    num_worker_threads: usize,
    cancelled: Arc<Mutex<bool>>,
    join_handle: Option<thread::JoinHandle<()>>,
}

impl DefaultJobState {
    fn new(
        platform: &dyn Platform,
        job_task: Box<dyn JobTask + Send + Sync>,
        priority: TaskPriority,
        num_worker_threads: usize,
    ) -> Self {
        DefaultJobState {
            platform: platform as *const dyn Platform as *const DefaultPlatform,
            job_task,
            priority,
            num_worker_threads,
            cancelled: Arc::new(Mutex::new(false)),
            join_handle: None,
        }
    }
}

struct DefaultJobHandle {
    state: Arc<DefaultJobState>,
}

impl DefaultJobHandle {
    fn new(state: Arc<DefaultJobState>) -> Self {
        let cancelled = state.cancelled.clone();
        let num_worker_threads = state.num_worker_threads;
        let platform_ptr = state.platform;
        let job_task = &mut state.job_task;
        let priority = state.priority;

        let join_handle = thread::spawn(move || {
            let platform = unsafe { &*platform_ptr }; // Dereference the raw pointer

            let mut handles = Vec::new();
            for worker_id in 0..num_worker_threads {
                let cancelled_clone = cancelled.clone();
                let mut job_task_clone = || {
                    let platform = platform;
                    (unsafe { &mut *(job_task as *const _ as *mut dyn JobTask) }).run(&DefaultJobHandle{state: state.clone()}, worker_id);

                };
                let join_handle = thread::spawn(move ||{
                    job_task_clone();
                });
                handles.push(join_handle);
            }
            for handle in handles {
                handle.join().unwrap();
            }

        });

        DefaultJobHandle {
            state: state.clone(),
        }
    }

}

impl JobHandle for DefaultJobHandle {
    fn is_cancelled(&self) -> bool {
        *self.state.cancelled.lock().unwrap()
    }

    fn cancel(&self) {
        *self.state.cancelled.lock().unwrap() = true;
    }

    fn join(&self) {
        if let Some(handle) = &self.state.join_handle {
            handle.join().unwrap();
        }
    }
}

fn new_default_job_handle(
    platform: &dyn Platform,
    priority: TaskPriority,
    job_task: Box<dyn JobTask + Send + Sync>,
    num_worker_threads: usize,
) -> Box<dyn JobHandle> {
    let state = Arc::new(DefaultJobState::new(platform, job_task, priority, num_worker_threads));
    Box::new(DefaultJobHandle::new(state))
}

fn pump_message_loop(
    platform: &dyn Platform,
    isolate: &Isolate,
    behavior: MessageLoopBehavior,
) -> bool {
    let platform = unsafe { &*(platform as *const dyn Platform as *const DefaultPlatform) }; // Type cast
    platform.pump_message_loop(isolate, behavior)
}

fn run_idle_tasks(platform: &dyn Platform, isolate: &Isolate, idle_time_in_seconds: f64) {
    let platform = unsafe { &*(platform as *const dyn Platform as *const DefaultPlatform) }; // Type cast
    platform.run_idle_tasks(isolate, idle_time_in_seconds);
}

fn notify_isolate_shutdown(platform: &dyn Platform, isolate: &Isolate) {
    let platform = unsafe { &*(platform as *const dyn Platform as *const DefaultPlatform) }; // Type cast
    platform.notify_isolate_shutdown(isolate);
}

fn print_stack_trace() {
    //crate::base::debug::stack_trace::StackTrace::new().print();
    // Implement stack trace printing here, possibly using backtrace crate
    println!("Stack trace printing not yet implemented.");
}

//Dummy structs for types that were missing. These will need implementing if the above code is to work.
struct DefaultForegroundTaskRunner{
    idle_task_support: IdleTaskSupport,
    time_function: fn() -> f64,
}

impl DefaultForegroundTaskRunner{
    fn new(idle_task_support: IdleTaskSupport, time_function: fn() -> f64) -> Self{
        DefaultForegroundTaskRunner{
            idle_task_support,
            time_function,
        }
    }

    fn pop_task_from_queue(&self, wait_for_work: MessageLoopBehavior) -> Option<Box<dyn Task>>{
        //Dummy implementation returning None
        None
    }

    fn pop_task_from_idle_queue(&self) -> Option<Box<dyn IdleTask>>{
        //Dummy implementation returning None
        None
    }

    fn terminate(&self){
        //Dummy implementation returning None
    }

    struct RunTaskScope{
        task_runner: Arc<DefaultForegroundTaskRunner>,
    }
}

struct DefaultWorkerThreadsTaskRunner{
    thread_pool_size: usize,
    time_function: fn() -> f64,
    priority: TaskPriority,
}

impl DefaultWorkerThreadsTaskRunner{
    fn new(thread_pool_size: usize, time_function: fn() -> f64, priority: TaskPriority) -> Self{
        DefaultWorkerThreadsTaskRunner{
            thread_pool_size,
            time_function,
            priority,
        }
    }

    fn post_task(&self, task: Box<dyn Task + Send + Sync>){
        //Dummy implementation returning None
    }

    fn post_delayed_task(&self, task: Box<dyn Task + Send + Sync>, delay_in_seconds: f64){
        //Dummy implementation returning None
    }

    fn terminate(&self){
        //Dummy implementation returning None
    }

}