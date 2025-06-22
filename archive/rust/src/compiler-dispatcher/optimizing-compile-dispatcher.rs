// src/compiler-dispatcher/optimizing_compile_dispatcher.rs

use std::cmp;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex, Condvar};
use std::time::Duration;
// use base::{atomicops, fpu, logging}; // Placeholder for base crate
// use base::platform::mutex::Mutex; // Placeholder for base::platform::mutex
// use base::vector::OwnedVector; // Placeholder for base::vector
// use codegen::{compiler, optimized_compilation_info}; // Placeholder for codegen crate
// use execution::{isolate, local_isolate_inl}; // Placeholder for execution crate
// use handles::handles_inl; // Placeholder for handles crate
// use heap::local_heap_inl; // Placeholder for heap crate
// use init::v8; // Placeholder for init crate
// use logging::{counters, log}; // Placeholder for logging crate
// use logging::runtime_call_stats_scope; // Placeholder for logging crate
// use objects::js_function; // Placeholder for objects crate
// use tasks::cancelable_task; // Placeholder for tasks crate
// use tracing::trace_event; // Placeholder for tracing crate

// Placeholder for flags
struct V8Flags {
    concurrent_recompilation: bool,
    concurrent_builtin_generation: bool,
    concurrent_turbofan_max_threads: i32,
    concurrent_recompilation_queue_length: usize,
    concurrent_recompilation_delay: i32,
    trace_concurrent_recompilation: bool,
}

// Mock flags instance
static v8_flags: V8Flags = V8Flags {
    concurrent_recompilation: true,
    concurrent_builtin_generation: true,
    concurrent_turbofan_max_threads: 0,
    concurrent_recompilation_queue_length: 1024,
    concurrent_recompilation_delay: 0,
    trace_concurrent_recompilation: false,
};

// Placeholder for TimerEventScope
struct TimerEventScope<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> TimerEventScope<T> {
    fn new(_isolate: *mut Isolate) -> Self {
        TimerEventScope {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T> Drop for TimerEventScope<T> {
    fn drop(&mut self) {}
}

// Placeholder for Isolate
struct Isolate {
    thread_id: usize,
    efficiency_mode_enabled_for_tiering: bool,
    optimizing_compile_dispatcher: Mutex<OptimizingCompileDispatcher>,
    is_generating_embedded_builtins: bool,
    stack_guard: StackGuard,
    main_thread_local_isolate: MainThreadLocalIsolate,
}

impl Isolate {
    fn efficiency_mode_enabled_for_tiering(&self) -> bool {
        self.efficiency_mode_enabled_for_tiering
    }
    fn optimizing_compile_dispatcher(&self) -> &Mutex<OptimizingCompileDispatcher> {
        &self.optimizing_compile_dispatcher
    }
    fn is_generating_embedded_builtins(&self) -> bool {
        self.is_generating_embedded_builtins
    }

    fn thread_id(&self) -> usize {
      self.thread_id
    }
    fn stack_guard(&self) -> &StackGuard {
        &self.stack_guard
    }
    fn main_thread_local_isolate(&self) -> &MainThreadLocalIsolate {
        &self.main_thread_local_isolate
    }
}

// Placeholder for LocalIsolate
struct LocalIsolate<'a> {
    isolate: *mut Isolate,
    thread_kind: ThreadKind,
    heap: LocalHeap,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> LocalIsolate<'a> {
    fn new(isolate: *mut Isolate, thread_kind: ThreadKind) -> Self {
        LocalIsolate {
            isolate,
            thread_kind,
            heap: LocalHeap{},
            _phantom: std::marker::PhantomData,
        }
    }

    fn runtime_call_stats(&self) -> RuntimeCallStats {
        RuntimeCallStats {}
    }

    fn heap(&self) -> &LocalHeap {
        &self.heap
    }
}

// Placeholder for RuntimeCallCounterId
enum RuntimeCallCounterId {
    kOptimizeBackgroundTurbofan,
}

// Placeholder for RCS_SCOPE
struct RCS_SCOPE<'a> {
    _local_isolate: &'a LocalIsolate<'a>,
    _id: RuntimeCallCounterId,
}

impl<'a> RCS_SCOPE<'a> {
    fn new(_local_isolate: &'a LocalIsolate<'a>, _id: RuntimeCallCounterId) -> Self {
        RCS_SCOPE {
            _local_isolate,
            _id,
        }
    }
}

impl<'a> Drop for RCS_SCOPE<'a> {
    fn drop(&mut self) {}
}

// Placeholder for JobDelegate
trait JobDelegate {
    fn get_task_id(&self) -> usize;
    fn should_yield(&self) -> bool;
}

// Placeholder for FlushDenormalsScope
struct FlushDenormalsScope {
    _flush_denormals: bool,
}

impl FlushDenormalsScope {
    fn new(_flush_denormals: bool) -> Self {
        FlushDenormalsScope {
            _flush_denormals,
        }
    }
}

impl Drop for FlushDenormalsScope {
    fn drop(&mut self) {}
}

// Placeholder for ConcurrencyMode
enum ConcurrencyMode {
    kBlocking,
    kNonBlocking,
}

// Placeholder for LocalHeap
struct LocalHeap {}

impl LocalHeap {
    fn is_parked(&self) -> bool {
        true
    }
}

// Placeholder for ThreadKind
enum ThreadKind {
    kBackground,
}

// Placeholder for CompilationJob::Status
enum CompilationJobStatus {
    SUCCEEDED,
}

// Placeholder for SharedFunctionInfo
struct SharedFunctionInfo {}

// Placeholder for JSFunction
struct JSFunction {}

// Placeholder for GlobalObject
struct GlobalObject {
    detached: bool,
}

impl GlobalObject {
    fn is_detached(&self) -> bool {
        self.detached
    }
}

// Placeholder for NativeContext
struct NativeContext {
    global_object: GlobalObject,
}

impl NativeContext {
    fn global_object(&self) -> &GlobalObject {
        &self.global_object
    }
}

// Placeholder for Handle
struct DirectHandle<T> {
    value: T,
}

impl DirectHandle<JSFunction> {
    fn new(value: JSFunction, _isolate: *mut Isolate) -> Self {
        DirectHandle {
            value,
        }
    }
}

// Placeholder for OptimizedCompilationInfo
struct OptimizedCompilationInfo {
    shared_info: SharedFunctionInfo,
    closure: JSFunction,
    code_kind: i32,
    is_osr: bool,
}

impl OptimizedCompilationInfo {
    fn shared_info(&self) -> &SharedFunctionInfo {
        &self.shared_info
    }
    fn closure(&self) -> &JSFunction {
        &self.closure
    }
    fn code_kind(&self) -> i32 {
        self.code_kind
    }
    fn is_osr(&self) -> bool {
        self.is_osr
    }
}

// Placeholder for AllowGarbageCollection
struct AllowGarbageCollection {}

impl AllowGarbageCollection {
    fn new() -> Self {
        AllowGarbageCollection {}
    }
}

impl Drop for AllowGarbageCollection {
    fn drop(&mut self) {}
}

// Placeholder for MainThreadLocalIsolate
struct MainThreadLocalIsolate {}

impl MainThreadLocalIsolate {
    fn execute_main_thread_while_parked<F>(&self, f: F)
    where
        F: FnOnce(),
    {
        f();
    }
}

// Placeholder for BlockingBehavior
enum BlockingBehavior {
    kBlock,
    kNonBlock,
}

// Placeholder for StackGuard
struct StackGuard {}

impl StackGuard {
    fn request_install_code(&self) {}
}

// Placeholder for kTaskPriority and kEfficiencyTaskPriority
const K_TASK_PRIORITY: i32 = 0;
const K_EFFICIENCY_TASK_PRIORITY: i32 = 1;

// Placeholder for V8::GetCurrentPlatform()
struct Platform {
    number_of_worker_threads: usize,
}

impl Platform {
    fn number_of_worker_threads(&self) -> usize {
        self.number_of_worker_threads
    }
    fn post_job(&self, priority: i32, job: Box<dyn JobTask>) -> JobHandle {
        JobHandle {
            priority,
            job,
            valid: true,
            update_priority_enabled: true,
        }
    }
}

fn v8_get_current_platform() -> Platform {
    Platform { number_of_worker_threads: 4 }
}

// Placeholder for JobTask
trait JobTask: Send {
    fn run(&mut self, delegate: &dyn JobDelegate);
    fn get_max_concurrency(&self, worker_count: usize) -> usize;
}

// Placeholder for JobHandle
struct JobHandle {
    priority: i32,
    job: Box<dyn JobTask>,
    valid: bool,
    update_priority_enabled: bool,
}

impl JobHandle {
    fn cancel(&mut self) {
        self.valid = false;
    }
    fn is_valid(&self) -> bool {
        self.valid
    }
    fn update_priority_enabled(&self) -> bool {
        self.update_priority_enabled
    }
    fn update_priority(&mut self, _priority: i32) {}
    fn notify_concurrency_increase(&self) {}
}

// Placeholder for V8
struct V8 {}

impl V8 {
    fn get_current_platform() -> Platform {
        v8_get_current_platform()
    }
}

// Placeholder for TurbofanCompilationJob
struct TurbofanCompilationJob {
    isolate_ptr: *mut Isolate,
    compilation_info: OptimizedCompilationInfo,
    trace_id: u32,
    finalize_order: i32,
    status: CompilationJobStatus,
}

impl TurbofanCompilationJob {
    fn new(isolate_ptr: *mut Isolate, compilation_info: OptimizedCompilationInfo) -> Self {
        TurbofanCompilationJob {
            isolate_ptr,
            compilation_info,
            trace_id: 0,
            finalize_order: 0,
            status: CompilationJobStatus::SUCCEEDED,
        }
    }

    fn isolate(&self) -> *mut Isolate {
        self.isolate_ptr
    }

    fn compilation_info(&self) -> &OptimizedCompilationInfo {
        &self.compilation_info
    }

    fn trace_id(&self) -> u32 {
        self.trace_id
    }

    fn execute_job(&mut self, _runtime_call_stats: RuntimeCallStats, _local_isolate: &LocalIsolate) -> CompilationJobStatus {
        CompilationJobStatus::SUCCEEDED
    }

    fn finalize_order(&self) -> i32 {
        self.finalize_order
    }

    fn finalize_job(&mut self, _isolate: *mut Isolate) -> CompilationJobStatus {
        CompilationJobStatus::SUCCEEDED
    }

    fn set_finalize_order(&mut self, finalize_order: i32) {
      self.finalize_order = finalize_order;
    }
}

// Placeholder for RuntimeCallStats
struct RuntimeCallStats {}

// OptimizingCompileTaskState struct
#[derive(Default)]
struct OptimizingCompileTaskState {
    isolate: *mut Isolate,
}

// OptimizingCompileTaskExecutor struct
pub struct OptimizingCompileTaskExecutor {
    input_queue_: OptimizingCompileInputQueue,
    recompilation_delay_: i32,
    task_states_: Arc<Mutex<Vec<OptimizingCompileTaskState>>>,
    job_handle_: Option<JobHandle>,
    is_initialized_: bool,
}

impl OptimizingCompileTaskExecutor {
    pub fn new() -> Self {
        OptimizingCompileTaskExecutor {
            input_queue_: OptimizingCompileInputQueue::new(v8_flags.concurrent_recompilation_queue_length),
            recompilation_delay_: v8_flags.concurrent_recompilation_delay,
            task_states_: Arc::new(Mutex::new(Vec::new())),
            job_handle_: None,
            is_initialized_: false,
        }
    }

    pub fn ensure_initialized(&mut self) {
        if self.is_initialized_ {
            return;
        }
        self.is_initialized_ = true;

        if v8_flags.concurrent_recompilation || v8_flags.concurrent_builtin_generation {
            let max_tasks = if v8_flags.concurrent_turbofan_max_threads == 0 {
                V8::get_current_platform().number_of_worker_threads()
            } else {
                v8_flags.concurrent_turbofan_max_threads as usize
            };

            let mut task_states = self.task_states_.lock().unwrap();
            *task_states = vec![OptimizingCompileTaskState::default(); max_tasks];
            drop(task_states);

            let task_states_clone = Arc::clone(&self.task_states_);

            let compile_task = CompileTask {
                task_executor_: self,
                task_states_: task_states_clone,
            };

            self.job_handle_ = Some(V8::get_current_platform().post_job(K_TASK_PRIORITY, Box::new(compile_task)));
        }
    }

    fn next_input(&self, task_state: &mut OptimizingCompileTaskState) -> Option<Box<TurbofanCompilationJob>> {
        self.input_queue_.dequeue(task_state)
    }

    fn next_input_if_isolate_matches(&self, isolate: *mut Isolate) -> Option<Box<TurbofanCompilationJob>> {
        self.input_queue_.dequeue_if_isolate_matches(isolate)
    }

    fn compile_next(&self, isolate: *mut Isolate, local_isolate: &LocalIsolate, job: Box<TurbofanCompilationJob>) {
        // The function may have already been optimized by OSR.  Simply continue.
        let mut_job = job;
        let status = mut_job.execute_job(local_isolate.runtime_call_stats(), local_isolate);

        unsafe {
            (*isolate).optimizing_compile_dispatcher().lock().unwrap().queue_finished_job(*mut_job);
        }
    }

    fn is_task_running_for_isolate(&self, isolate: *mut Isolate) -> bool {
        self.input_queue_.is_task_running_for_isolate(isolate)
    }

    fn has_compilation_jobs_for_isolate(&self, isolate: *mut Isolate) -> bool {
        self.input_queue_.has_job_for_isolate(isolate) || self.is_task_running_for_isolate(isolate)
    }

    fn clear_task_state(&self, task_state: &mut OptimizingCompileTaskState) {
        self.input_queue_.clear_task_state(task_state);
    }

    pub fn try_queue_for_optimization(&self, job: Box<TurbofanCompilationJob>) -> bool {
        unsafe {
            let isolate = (*job).isolate();
            if self.input_queue_.enqueue(job) {
                if let Some(ref job_handle) = self.job_handle_ {
                    if job_handle.update_priority_enabled() {
                        // TODO: Implement EfficiencyModeEnabledForTiering()
                        // job_handle.update_priority(isolate.EfficiencyModeEnabledForTiering() ? kEfficiencyTaskPriority : kTaskPriority);
                    }
                }

                if let Some(ref job_handle) = self.job_handle_ {
                    job_handle.notify_concurrency_increase();
                }
                true
            } else {
                false
            }
        }
    }

    pub fn wait_until_compilation_jobs_done_for_isolate(&self, isolate: *mut Isolate) {
        self.input_queue_.wait_until_compilation_jobs_done_for_isolate(isolate);
    }
}

impl Drop for OptimizingCompileTaskExecutor {
    fn drop(&mut self) {
        //DCHECK_EQ(input_queue_.Length(), 0);
        let queue_length = self.input_queue_.queue_.lock().unwrap().len();
        assert_eq!(queue_length, 0);

        if let Some(job_handle) = &mut self.job_handle_ {
            //DCHECK(job_handle_->IsValid());
            assert!(job_handle.is_valid());

            // Wait for the job handle to complete, so that we know the queue
            // pointers are safe.
            job_handle.cancel();
        }
    }
}

// CompileTask struct
struct CompileTask<'a> {
    task_executor_: &'a OptimizingCompileTaskExecutor,
    task_states_: Arc<Mutex<Vec<OptimizingCompileTaskState>>>,
}

impl JobTask for CompileTask<'_> {
    fn run(&mut self, delegate: &dyn JobDelegate) {
        //TRACE_EVENT0(TRACE_DISABLED_BY_DEFAULT("v8.compile"), "V8.TurbofanTask");
        //DCHECK_LT(delegate->GetTaskId(), task_executor_->task_states_.size());
        let task_id = delegate.get_task_id();

        let mut should_yield = delegate.should_yield();

        while !should_yield {
            // NextInput() sets the isolate for task_state to job->isolate() while
            // holding the lock.
            let mut task_states = self.task_states_.lock().unwrap();
            if task_id >= task_states.len() {
                println!("Task ID {} out of bounds for task_states of size {}", task_id, task_states.len());
                return;
            }
            let task_state = &mut task_states[task_id];
            drop(task_states);

            let job = match self.task_executor_.next_input(task_state) {
                Some(job) => job,
                None => break,
            };

            unsafe {
                let isolate = (*job).isolate();

                let _flush_denormals_scope = FlushDenormalsScope::new(false); // TODO: replace isolate->flush_denormals() with actual value

                // Note that LocalIsolate's lifetime is shorter than the isolate value
                // in task_state which is only cleared after this LocalIsolate instance
                // was destroyed.
                let local_isolate = LocalIsolate::new(isolate, ThreadKind::kBackground);
                //DCHECK(local_isolate.heap()->IsParked());
                assert!(local_isolate.heap().is_parked());

                loop {
                    self.run_compilation_job(isolate, &local_isolate, job);

                    should_yield = delegate.should_yield();
                    if should_yield {
                        break;
                    }

                    // Reuse the LocalIsolate if the next worklist item has the same
                    // isolate.
                    let job = self.task_executor_.next_input_if_isolate_matches(isolate);
                    match job {
                        Some(job) => {
                            self.run_compilation_job(isolate, &local_isolate, job);
                        },
                        None => break,
                    }
                }

                // Reset the isolate in the task state to nullptr. Only do this after the
                // LocalIsolate was destroyed. This invariant is used by
                // WaitUntilTasksStoppedForIsolate() to ensure all tasks are stopped for
                // an isolate.
                let mut task_states = self.task_states_.lock().unwrap();
                let task_state = &mut task_states[task_id];
                self.task_executor_.clear_task_state(task_state);
                drop(task_states);
            }
        }

        // Here we are allowed to read the isolate without holding a lock because
        // only this thread here will ever change this field and the main thread
        // will only ever read it.
        let task_states = self.task_states_.lock().unwrap();
        let task_state = &task_states[delegate.get_task_id()];
        drop(task_states);
        unsafe {
            assert!(task_state.isolate.is_null());
        }
    }

    fn get_max_concurrency(&self, worker_count: usize) -> usize {
        let num_tasks = self.task_executor_.input_queue_.length() + worker_count;
        let task_states_lock = self.task_executor_.task_states_.lock().unwrap();
        let task_states_size = task_states_lock.len();
        drop(task_states_lock);
        cmp::min(num_tasks, task_states_size)
    }
}

impl<'a> CompileTask<'a> {
    fn run_compilation_job(&self, isolate: *mut Isolate, local_isolate: &LocalIsolate, job: Box<TurbofanCompilationJob>) {
        //TRACE_EVENT_WITH_FLOW0(TRACE_DISABLED_BY_DEFAULT("v8.compile"), "V8.OptimizeBackground", job->trace_id(), TRACE_EVENT_FLAG_FLOW_IN | TRACE_EVENT_FLAG_FLOW_OUT);
        unsafe {
            let _timer = TimerEventScope::<TimerEventRecompileConcurrent>::new(isolate);

            if self.task_executor_.recompilation_delay_ != 0 {
                std::thread::sleep(Duration::from_millis(self.task_executor_.recompilation_delay_ as u64));
            }

            let _rcs_scope = RCS_SCOPE::new(local_isolate, RuntimeCallCounterId::kOptimizeBackgroundTurbofan);

            self.task_executor_.compile_next(isolate, local_isolate, job);
        }
    }
}

// Placeholder for TimerEventRecompileConcurrent
struct TimerEventRecompileConcurrent {}

// OptimizingCompileInputQueue struct
struct OptimizingCompileInputQueue {
    queue_: Arc<Mutex<VecDeque<Box<TurbofanCompilationJob>>>>,
    mutex_: Mutex<()>,
    task_finished_: Condvar,
    capacity_: usize,
}

impl OptimizingCompileInputQueue {
    fn new(capacity: usize) -> Self {
        OptimizingCompileInputQueue {
            queue_: Arc::new(Mutex::new(VecDeque::new())),
            mutex_: Mutex::new(()),
            task_finished_: Condvar::new(),
            capacity_: capacity,
        }
    }

    fn prioritize(&self, isolate: *mut Isolate, function: SharedFunctionInfo) {
        // Ensure that we only run this method on the main thread. This makes sure
        // that we never dereference handles during a safepoint.
        //DCHECK_EQ(isolate->thread_id(), ThreadId::Current());
        unsafe {
            let isolate_thread_id = (*isolate).thread_id();
            assert_eq!(isolate_thread_id, 0); // Assume thread id is 0
        }
        let access = self.mutex_.lock().unwrap();
        let mut queue = self.queue_.lock().unwrap();

        if let Some(it) = queue.iter().position(|job| {
            unsafe {
                // Early bailout to avoid dereferencing handles from other
                // isolates. The other isolate could be in a safepoint/GC
                // and dereferencing the handle is therefore invalid.
                if (*job).isolate() != isolate {
                    return false;
                }
                return (*job).compilation_info().shared_info() == &function;
            }
        }) {
            if let Some(first_for_isolate) = queue.iter().position(|job| {
                unsafe {
                    (*job).isolate() == isolate
                }
            }) {
                queue.swap(it, first_for_isolate);
            }
        }
        drop(queue);
        drop(access);
    }

    fn flush_jobs_for_isolate(&self, isolate: *mut Isolate) {
        let access = self.mutex_.lock().unwrap();
        let mut queue = self.queue_.lock().unwrap();
        queue.retain(|job| {
            unsafe {
                if (*job).isolate() != isolate {
                    return true;
                }

                false
            }
        });
        drop(queue);
        drop(access);
    }

    fn has_job_for_isolate(&self, isolate: *mut Isolate) -> bool {
        let _access = self.mutex_.lock().unwrap();
        let queue = self.queue_.lock().unwrap();
        let has_job = queue.iter().any(|job| {
            unsafe {
                (*job).isolate() == isolate
            }
        });
        drop(queue);
        has_job
    }

    fn dequeue(&self, task_state: &mut OptimizingCompileTaskState) -> Option<Box<TurbofanCompilationJob>> {
        let _access = self.mutex_.lock().unwrap();
        task_state.isolate = std::ptr::null_mut();

        let mut queue = self.queue_.lock().unwrap();
        let job = queue.pop_front();
        drop(queue);

        if let Some(ref job_ptr) = job {
            unsafe {
                task_state.isolate = (**job_ptr).isolate();
            }
        }
        job
    }

    fn dequeue_if_isolate_matches(&self, isolate: *mut Isolate) -> Option<Box<TurbofanCompilationJob>> {
        let _access = self.mutex_.lock().unwrap();

        let mut queue = self.queue_.lock().unwrap();
        if let Some(job) = queue.front() {
            unsafe {
                if (*job).isolate() != isolate {
                    return None;
                }
            }
            return queue.pop_front();
        }
        None
    }

    fn enqueue(&self, job: Box<TurbofanCompilationJob>) -> bool {
        let _access = self.mutex_.lock().unwrap();
        let mut queue = self.queue_.lock().unwrap();
        if queue.len() < self.capacity_ {
            queue.push_back(job);
            true
        } else {
            false
        }
    }

    fn length(&self) -> usize {
        let _access = self.mutex_.lock().unwrap();
        let queue = self.queue_.lock().unwrap();
        queue.len()
    }

    fn is_task_running_for_isolate(&self, isolate: *mut Isolate) -> bool {
        //self.mutex_.AssertHeld();  // MutexGuard does not have AssertHeld
        let _access = self.mutex_.lock().unwrap();
        let task_states = self.queue_.lock().unwrap();
        let is_running = task_states.iter().any(|job| {
            unsafe {
                (*job).isolate() == isolate
            }
        });
        drop(task_states);
        is_running
    }

    fn clear_task_state(&self, task_state: &mut OptimizingCompileTaskState) {
        let _access = self.mutex_.lock().unwrap();
        unsafe {
            assert!(!task_state.isolate.is_null());
            task_state.isolate = std::ptr::null_mut();
        }
        self.task_finished_.notify_all();
    }

    fn wait_until_compilation_jobs_done_for_isolate(&self, isolate: *mut Isolate) {
        // Once we have ensured that no task is working on the given isolate, we also
        // know that there are no more LocalHeaps for this isolate from CompileTask.
        // This is because CompileTask::Run() only updates the isolate once the
        // LocalIsolate/LocalHeap for it was destroyed.
        let access = self.mutex_.lock().unwrap();

        loop {
            if !self.has_job_for_isolate(isolate) && !self.is_task_running_for_isolate(isolate) {
                break;
            }
            self.task_finished_.wait(access);
        }
    }
}

// OptimizingCompileOutputQueue struct
struct OptimizingCompileOutputQueue {
    queue_: Mutex<Vec<Box<TurbofanCompilationJob>>>,
    mutex_: Mutex<()>,
}

impl OptimizingCompileOutputQueue {
    fn new() -> Self {
        OptimizingCompileOutputQueue {
            queue_: Mutex::new(Vec::new()),
            mutex_: Mutex::new(()),
        }
    }

    fn enqueue(&self, job: Box<TurbofanCompilationJob>) {
        let _guard = self.mutex_.lock().unwrap();
        let mut queue = self.queue_.lock().unwrap();
        queue.push(job);
    }

    fn dequeue(&self) -> Option<Box<TurbofanCompilationJob>> {
        let _guard = self.mutex_.lock().unwrap();
        let mut queue = self.queue_.lock().unwrap();
        if queue.is_empty() {
            None
        } else {
            queue.remove(0).into()
        }
    }

    fn size(&self) -> usize {
        let _guard = self.mutex_.lock().unwrap();
        let queue = self.queue_.lock().unwrap();
        queue.len()
    }

    fn empty(&self) -> bool {
        let _guard = self.mutex_.lock().unwrap();
        let queue = self.queue_.lock().unwrap();
        queue.is_empty()
    }

    fn install_generated_builtins(&self, isolate: *mut Isolate, mut installed_count: i32) -> i32 {
        // Builtin generation needs to be deterministic, meaning heap allocations must
        // happen in a deterministic order. To ensure determinism with concurrent
        // compilation, only finalize contiguous builtins in ascending order of their
        // finalization order, which is set at job creation time.

        unsafe {
            assert!((*isolate).is_generating_embedded_builtins());
        }

        let _guard = self.mutex_.lock().unwrap();
        let mut queue = self.queue_.lock().unwrap();

        queue.sort_by_key(|job| job.finalize_order());

        while !queue.is_empty() {
            let current = queue[0].finalize_order();
            assert_eq!(installed_count, current);

            let mut job = queue.remove(0);

            unsafe {
              assert_eq!(CompilationJobStatus::SUCCEEDED, (*job).finalize_job(isolate));
            }
            installed_count = current + 1;
        }

        installed_count
    }
}

// OptimizingCompileDispatcher struct
pub struct OptimizingCompileDispatcher {
    isolate_: *mut Isolate,
    task_executor_: *mut OptimizingCompileTaskExecutor,
    output_queue_: OptimizingCompileOutputQueue,
}

impl OptimizingCompileDispatcher {
    pub fn new(isolate: *mut Isolate, task_executor: *mut OptimizingCompileTaskExecutor) -> Self {
        OptimizingCompileDispatcher {
            isolate_: isolate,
            task_executor_: task_executor,
            output_queue_: OptimizingCompileOutputQueue::new(),
        }
    }

    fn queue_finished_job(&mut self, job: Box<TurbofanCompilationJob>) {
        unsafe {
            assert_eq!(self.isolate_, (*job).isolate());
        }
        self.output_queue_.enqueue(job);

        //if (finalize()) isolate_->stack_guard()->RequestInstallCode();
        unsafe {
          (*self.isolate_).stack_guard().request_install_code();
        }
    }

    fn flush_output_queue(&mut self) {
        loop {
            let job = self.output_queue_.dequeue();
            match job {
                Some(job) => {
                    //Compiler::DisposeTurbofanCompilationJob(isolate_, job.get());
                    //println!("Disposing job");
                }
                None => break,
            }
        }
    }

    fn finish_tear_down(&mut self) {
        unsafe {
            (*self.task_executor_).wait_until_compilation_jobs_done_for_isolate(self.isolate_);
        }
        //HandleScope handle_scope(isolate_);
        self.flush_output_queue();
    }

    fn flush_input_queue(&mut self) {
        //input_queue().FlushJobsForIsolate(isolate_);
        unsafe {
            (*self.task_executor_).input_queue_.flush_jobs_for_isolate(self.isolate_);
        }
    }

    fn wait_until_compilation_jobs_done(&mut self) {
        //AllowGarbageCollection allow_before_parking;
        let _allow = AllowGarbageCollection::new();
        //isolate_->main_thread_local_isolate()->ExecuteMainThreadWhileParked([this]() {
        //  task_executor_->WaitUntilCompilationJobsDoneForIsolate(isolate_);
        //});
        unsafe {
          (*self.isolate_).main_thread_local_isolate().execute_main_thread_while_parked(|| {
            (*self.task_executor_).wait_until_compilation_jobs_done_for_isolate(self.isolate_);
          });
        }
    }

    fn flush_queues(&mut self, blocking_behavior: BlockingBehavior) {
        self.flush_input_queue();
        if let BlockingBehavior::kBlock = blocking_behavior {
            self.wait_until_compilation_jobs_done();
        }
        self.flush_output_queue();
    }

    fn flush(&mut self, blocking_behavior: BlockingBehavior) {
        //HandleScope handle_scope(isolate_);
        //TODO add handlescope equivalent here

        self.flush_queues(blocking_behavior);

        if v8_flags.trace_concurrent_recompilation {
            println!(
                "  ** Flushed concurrent recompilation queues. (mode: {})",
                match blocking_behavior {
                    BlockingBehavior::kBlock => "blocking",
                    BlockingBehavior::kNonBlock => "non blocking",
                }
            );
        }
    }

    fn start_tear_down(&mut self) {
        //HandleScope handle_scope(