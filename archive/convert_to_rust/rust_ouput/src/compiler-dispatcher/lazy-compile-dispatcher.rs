// Converted from V8 C++ source files:
// Header: lazy-compile-dispatcher.h
// Implementation: lazy-compile-dispatcher.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod atomic_utils {
        pub struct AtomicValue<T> {
            value: std::sync::atomic::AtomicBool,
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T> AtomicValue<T> {
            pub fn new(initial_value: bool) -> Self {
                AtomicValue {
                    value: std::sync::atomic::AtomicBool::new(initial_value),
                    _phantom: std::marker::PhantomData,
                }
            }

            pub fn Value(&self) -> bool {
                self.value.load(std::sync::atomic::Ordering::Relaxed)
            }

            pub fn SetValue(&self, new_value: bool) {
                self.value.store(new_value, std::sync::atomic::Ordering::Relaxed)
            }
        }
    }
    pub mod platform {
        pub struct ConditionVariable {
            inner: std::sync::Condvar,
        }

        impl ConditionVariable {
            pub fn new() -> Self {
                ConditionVariable {
                    inner: std::sync::Condvar::new(),
                }
            }

            pub fn Wait<'a>(&self, mutex: &'a std::sync::Mutex<()>) {
                let mut guard = mutex.lock().unwrap();
                let _ = self.inner.wait(guard).unwrap();
            }

            pub fn NotifyOne(&self) {
                self.inner.notify_one();
            }
        }
        pub struct Mutex {
            inner: std::sync::Mutex<()>,
        }

        impl Mutex {
            pub fn new() -> Self {
                Mutex {
                    inner: std::sync::Mutex::new(()),
                }
            }

            pub fn lock(&self) -> Result<std::sync::MutexGuard<()>, std::sync::PoisonError<std::sync::MutexGuard<()>>> {
                self.inner.lock()
            }
        }
        pub struct MutexGuard<'a> {
            guard: std::sync::MutexGuard<'a, ()>,
        }

        impl<'a> MutexGuard<'a> {
            pub fn new(guard: std::sync::MutexGuard<'a, ()>) -> Self {
                MutexGuard { guard }
            }
        }
        pub struct Semaphore {
            inner: std::sync::Arc<tokio::sync::Semaphore>,
        }

        impl Semaphore {
            pub fn new(permits: u32) -> Self {
                Semaphore {
                    inner: std::sync::Arc::new(tokio::sync::Semaphore::new(permits as usize)),
                }
            }

            pub fn Wait(&self) {
                let permit = self.inner.clone().acquire_owned();
                tokio::runtime::Runtime::new().unwrap().block_on(permit).unwrap();
            }

            pub fn Signal(&self) {
                self.inner.add_permits(1);
            }
        }
        pub mod time {
            use std::time::{SystemTime, UNIX_EPOCH};

            pub fn MonotonicallyIncreasingTime() -> f64 {
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f64()
            }
        }
    }
}
pub mod common {
    pub mod globals {
        pub const DEBUG_BOOL: bool = true;
    }
}
pub mod utils {
    pub mod identity_map {
        use std::collections::HashMap;

        pub struct IdentityMap<V, A> {
            map: HashMap<usize, V>,
            _phantom: std::marker::PhantomData<A>,
        }

        impl<V, A> IdentityMap<V, A> {
            pub fn new() -> Self {
                IdentityMap {
                    map: HashMap::new(),
                    _phantom: std::marker::PhantomData,
                }
            }

            pub fn insert(&mut self, key: usize, value: V) {
                self.map.insert(key, value);
            }

            pub fn get(&self, key: usize) -> Option<&V> {
                self.map.get(&key)
            }

            pub fn remove(&mut self, key: usize) -> Option<V> {
                self.map.remove(&key)
            }
        }

        pub struct FreeStoreAllocationPolicy;
    }
}
pub mod testing {
    pub mod gtest {
        pub mod include {
            pub mod gtest {
                #[macro_export]
                macro_rules! FRIEND_TEST {
                    ($test_suite_name:ident, $test_name:ident) => {
                        #[cfg(test)]
                        impl $test_suite_name {
                            #[allow(dead_code)]
                            pub fn $test_name() {
                                super::$test_name();
                            }
                        }
                    };
                }
            }
        }
    }
}
pub mod v8 {
    pub struct JobDelegate {}
    impl JobDelegate {
        pub fn ShouldYield(&self) -> bool {
            false
        }
    }
    pub struct JobHandle {
        is_valid: bool,
    }
    impl JobHandle {
        pub fn IsValid(&self) -> bool {
            self.is_valid
        }
        pub fn Cancel(&mut self) {
            self.is_valid = false;
        }
        pub fn NotifyConcurrencyIncrease(&self) {}
    }
    pub enum class TaskPriority {
        kUserVisible,
    }
    pub struct Platform {}
    impl Platform {
        pub fn GetForegroundTaskRunner(&self, _isolate: *mut v8::Isolate) -> std::shared_ptr<TaskRunner> {
            std::shared_ptr::new(TaskRunner {})
        }
        pub fn PostJob(&self, _task_priority: TaskPriority, _job_task: std::unique_ptr<JobTask>) -> std::unique_ptr<JobHandle> {
            std::unique_ptr::new(JobHandle { is_valid: true })
        }

        pub fn MonotonicallyIncreasingTime(&self) -> f64 {
            base::platform::time::MonotonicallyIncreasingTime()
        }
    }
    pub struct TaskRunner {
    idle_tasks_enabled: bool,
    }
    impl TaskRunner {
        pub fn IdleTasksEnabled(&self) -> bool {
            self.idle_tasks_enabled
        }
        pub fn PostIdleTask(&self, _task: Box<dyn FnOnce(f64)>) {}
    }
    pub enum class MemoryPressureLevel {}
    pub struct Isolate {}
}
pub mod internal {
    use crate::base::atomic_utils::AtomicValue;
    use crate::base::platform::{ConditionVariable, Mutex, MutexGuard, Semaphore};
    use crate::common::globals::DEBUG_BOOL;
    use crate::testing::gtest::include::gtest::FRIEND_TEST;
    use crate::v8;
    use crate::v8::{JobDelegate, JobHandle, Platform, TaskPriority, TaskRunner, Isolate};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    pub struct AstRawString {}
    pub struct AstValueFactory {}
    pub struct BackgroundCompileTask {
        shared_info: DirectHandle<SharedFunctionInfo>,
        character_stream: std::unique_ptr<Utf16CharacterStream>,
    }
    impl BackgroundCompileTask {
        pub fn Run(&mut self, _isolate: &LocalIsolate, _reusable_state: &ReusableUnoptimizedCompileState) {}
        pub fn RunOnMainThread(&mut self, _isolate: *mut Isolate) {}
        pub fn AbortFunction(&mut self) {}
    }
    pub struct CancelableTaskManager {
        aborted: Arc<AtomicUsize>,
    }
    impl CancelableTaskManager {
        pub fn new() -> Self {
            CancelableTaskManager {
                aborted: Arc::new(AtomicUsize::new(0)),
            }
        }
        pub fn TryAbortAll(&self) {
            self.aborted.fetch_add(1, Ordering::SeqCst);
        }
        pub fn CancelAndWait(&self) {}
    }
    pub struct UnoptimizedCompileJob {}
    pub struct UnoptimizedCompileState {}
    pub struct FunctionLiteral {}
    pub struct ParseInfo {}
    pub struct ProducedPreparseData {}
    pub struct SharedFunctionInfo {}
    pub struct TimedHistogram {}
    pub struct Utf16CharacterStream {}
    pub struct WorkerThreadRuntimeCallStats {}
    pub struct Zone {}
    pub struct LocalIsolate {
        thread_kind: ThreadKind,
    }
    impl LocalIsolate {
        pub fn new(_isolate: *mut Isolate, thread_kind: ThreadKind) -> Self {
            LocalIsolate {
                thread_kind,
            }
        }
        pub fn factory(&self) -> Factory {
            Factory {}
        }
    }
    pub struct Factory {}
    impl Factory {
        pub fn NewUncompiledDataWithPreparseDataAndJob(
            &self,
            _inferred_name: Handle<String>,
            _start_position: i32,
            _end_position: i32,
            _preparse_data: Handle<PreparseData>,
        ) -> DirectHandle<UncompiledDataWithPreparseDataAndJob> {
            DirectHandle::new(UncompiledDataWithPreparseDataAndJob {})
        }
        pub fn NewUncompiledDataWithoutPreparseDataWithJob(
            &self,
            _inferred_name: Handle<String>,
            _start_position: i32,
            _end_position: i32,
        ) -> DirectHandle<UncompiledDataWithoutPreparseDataWithJob> {
            DirectHandle::new(UncompiledDataWithoutPreparseDataWithJob {})
        }
    }
    pub struct UnparkedScope<'a> {
        _isolate: &'a LocalIsolate,
    }
    impl<'a> UnparkedScope<'a> {
        pub fn new(_isolate: &'a LocalIsolate) -> Self {
            UnparkedScope { _isolate }
        }
    }
    pub struct LocalHandleScope<'a> {
        _isolate: &'a LocalIsolate,
    }
    impl<'a> LocalHandleScope<'a> {
        pub fn new(_isolate: &'a LocalIsolate) -> Self {
            LocalHandleScope { _isolate }
        }
    }
    pub struct ReusableUnoptimizedCompileState<'a> {
        _isolate: &'a LocalIsolate,
    }
    impl<'a> ReusableUnoptimizedCompileState<'a> {
        pub fn new(_isolate: &'a LocalIsolate) -> Self {
            ReusableUnoptimizedCompileState { _isolate }
        }
    }
    pub struct Handle<T> {
        _phantom: std::marker::PhantomData<T>,
    }
    pub struct DirectHandle<T> {
        _phantom: std::marker::PhantomData<T>,
    }
    impl<T> DirectHandle<T> {
        pub fn new(_value: T) -> Self {
            DirectHandle {
                _phantom: std::marker::PhantomData,
            }
        }
    }
    pub struct String {}
    pub struct PreparseData {}
    pub struct UncompiledData {}
    pub struct UncompiledDataWithPreparseDataAndJob {}
    pub struct UncompiledDataWithoutPreparseDataWithJob {}
    pub enum ThreadKind {
        kBackground,
        kForeground,
    }
    pub struct Flags {}
    impl Flags {
        pub lazy_compile_dispatcher_max_threads: usize,
        pub trace_compiler_dispatcher: bool,
    }
    pub mod counters {
        pub struct Counters {}
        impl Counters {
            pub fn worker_thread_runtime_call_stats(&self) -> WorkerThreadRuntimeCallStats {
                WorkerThreadRuntimeCallStats {}
            }
            pub fn compile_function_on_background(&self) -> TimedHistogram {
                TimedHistogram {}
            }
        }
    }
    pub enum RuntimeCallCounterId {
        kCompileEnqueueOnDispatcher,
        kCompileFinishNowOnDispatcher,
        kCompileWaitForDispatcher,
    }
    pub struct RuntimeCallStatsScope<'a> {
        _isolate: &'a LocalIsolate,
    }
    impl<'a> RuntimeCallStatsScope<'a> {
        pub fn new(_isolate: &'a LocalIsolate, _id: RuntimeCallCounterId) -> Self {
            RuntimeCallStatsScope { _isolate }
        }
    }
    macro_rules! RCS_SCOPE {
        ($isolate:expr, $id:expr) => {
            let _rcs_scope = RuntimeCallStatsScope::new($isolate, $id);
        };
    }

    pub struct InstanceType {}
    pub struct ObjectsInl {}
    pub mod parsing {
        pub struct ParseInfo {}
        pub struct Scanner {}
    }
    pub mod tasks {
        pub struct CancelableTask {}
    }
    pub mod zone {
        pub struct ZoneListInl {}
    }
    pub struct Compiler {}
    impl Compiler {
        pub fn FinalizeBackgroundCompileTask(
            _task: &mut BackgroundCompileTask,
            _isolate: *mut Isolate,
            _clear_exception: i32,
        ) -> bool {
            true
        }
        pub const KEEP_EXCEPTION: i32 = 0;
        pub const CLEAR_EXCEPTION: i32 = 0;
    }
    pub fn IsUncompiledDataWithPreparseDataAndJob(_data: UncompiledData) -> bool {
        false
    }
    pub fn IsUncompiledDataWithoutPreparseDataWithJob(_data: UncompiledData) -> bool {
        false
    }
    pub fn IsUncompiledDataWithoutPreparseData(_data: UncompiledData) -> bool {
        false
    }
    pub trait TracedValue {}

    // The LazyCompileDispatcher uses a combination of idle tasks and background
    // tasks to parse and compile lazily parsed functions.
    //
    // As both parsing and compilation currently requires a preparation and
    // finalization step that happens on the main thread, every task has to be
    // advanced during idle time first. Depending on the properties of the task, it
    // can then be parsed or compiled on either background threads, or during idle
    // time. Last, it has to be finalized during idle time again.
    //
    // LazyCompileDispatcher::jobs_ maintains the list of all
    // LazyCompilerDispatcherJobs the LazyCompileDispatcher knows about.
    //
    // LazyCompileDispatcher::pending_background_jobs_ contains the set of
    // LazyCompilerDispatcherJobs that can be processed on a background thread.
    //
    // LazyCompileDispatcher::running_background_jobs_ contains the set of
    // LazyCompilerDispatcherJobs that are currently being processed on a background
    // thread.
    //
    // LazyCompileDispatcher::DoIdleWork tries to advance as many jobs out of jobs_
    // as possible during idle time. If a job can't be advanced, but is suitable for
    // background processing, it fires off background threads.
    //
    // LazyCompileDispatcher::DoBackgroundWork advances one of the pending jobs,
    // and then spins of another idle task to potentially do the final step on the
    // main thread.
    pub struct LazyCompileDispatcher {
        isolate_: *mut Isolate,
        worker_thread_runtime_call_stats_: WorkerThreadRuntimeCallStats,
        background_compile_timer_: TimedHistogram,
        taskrunner_: std::shared_ptr<TaskRunner>,
        platform_: *mut Platform,
        max_stack_size_: usize,
        job_handle_: std::unique_ptr<JobHandle>,
        trace_compiler_dispatcher_: bool,
        idle_task_manager_: std::unique_ptr<CancelableTaskManager>,
        idle_task_scheduled_: bool,
        mutex_: Mutex,
        pending_background_jobs_: Vec<*mut Job>,
        finalizable_jobs_: Vec<*mut Job>,
        num_jobs_for_background_: AtomicUsize,
        all_jobs_: std::collections::HashSet<*mut Job>,
        jobs_to_dispose_: Vec<*mut Job>,
        main_thread_blocking_on_job_: *mut Job,
        main_thread_blocking_signal_: ConditionVariable,
        block_for_testing_: AtomicValue<bool>,
        semaphore_for_testing_: Semaphore,
    }

    impl LazyCompileDispatcher {
        pub fn new(isolate: *mut Isolate, platform: *mut Platform, max_stack_size: usize) -> Self {
            let flags = Flags {
                lazy_compile_dispatcher_max_threads: 1,
                trace_compiler_dispatcher: false,
            };
            let isolate_obj = unsafe { &*isolate };
            let counters = isolate_obj.counters();
            LazyCompileDispatcher {
                isolate_: isolate,
                worker_thread_runtime_call_stats_: counters.worker_thread_runtime_call_stats(),
                background_compile_timer_: counters.compile_function_on_background(),
                taskrunner_: unsafe { (*platform).GetForegroundTaskRunner(isolate) },
                platform_: platform,
                max_stack_size_: max_stack_size,
                job_handle_: unsafe { (*platform).PostJob(TaskPriority::kUserVisible, std::unique_ptr::new(JobTask::new(std::ptr::null_mut()))) },
                trace_compiler_dispatcher_: flags.trace_compiler_dispatcher,
                idle_task_manager_: std::unique_ptr::new(CancelableTaskManager::new()),
                idle_task_scheduled_: false,
                mutex_: Mutex::new(),
                pending_background_jobs_: Vec::new(),
                finalizable_jobs_: Vec::new(),
                num_jobs_for_background_: AtomicUsize::new(0),
                all_jobs_: std::collections::HashSet::new(),
                jobs_to_dispose_: Vec::new(),
                main_thread_blocking_on_job_: std::ptr::null_mut(),
                main_thread_blocking_signal_: ConditionVariable::new(),
                block_for_testing_: AtomicValue::new(false),
                semaphore_for_testing_: Semaphore::new(0),
            }
        }

        pub fn Enqueue(
            &mut self,
            isolate: &mut LocalIsolate,
            shared_info: Handle<SharedFunctionInfo>,
            character_stream: std::unique_ptr<Utf16CharacterStream>,
        ) {
            println!("LazyCompileDispatcher::Enqueue");
            let job = Box::new(Job {
                task: std::unique_ptr::new(BackgroundCompileTask {
                    shared_info: DirectHandle::new(SharedFunctionInfo {}),
                    character_stream: std::move(character_stream),
                }),
                state: Job::State::kPending,
            });
            let job_ptr = Box::into_raw(job);

            {
                let lock = self.mutex_.lock().unwrap();
                if self.trace_compiler_dispatcher_ {
                    println!("LazyCompileDispatcher: enqueued job");
                }

                self.all_jobs_.insert(job_ptr);
                self.pending_background_jobs_.push(job_ptr);
                self.NotifyAddedBackgroundJob(&lock);
            }
            unsafe { (*self.job_handle_.as_mut().unwrap()).NotifyConcurrencyIncrease() };
        }

        // Returns true if there is a pending job registered for the given function.
        pub fn IsEnqueued(&self, _function: DirectHandle<SharedFunctionInfo>) -> bool {
            false
        }

        // Blocks until the given function is compiled (and does so as fast as
        // possible). Returns true if the compile job was successful.
        pub fn FinishNow(&mut self, _function: DirectHandle<SharedFunctionInfo>) -> bool {
            true
        }

        // Aborts compilation job for the given function.
        pub fn AbortJob(&mut self, _function: DirectHandle<SharedFunctionInfo>) {}

        // Aborts all jobs, blocking until all jobs are aborted.
        pub fn AbortAll(&mut self) {}

        fn WaitForJobIfRunningOnBackground(&mut self, job: *mut Job, _lock: &std::sync::MutexGuard<()>) {
            if unsafe { !(*job).is_running_on_background() } {
                return;
            }
            self.main_thread_blocking_on_job_ = job;
            while self.main_thread_blocking_on_job_ != std::ptr::null_mut() {
                self.main_thread_blocking_signal_.Wait(&self.mutex_.inner);
            }
        }

        fn GetJobFor(
            &self,
            _shared: DirectHandle<SharedFunctionInfo>,
            _lock: &std::sync::MutexGuard<()>,
        ) -> *mut Job {
            std::ptr::null_mut()
        }

        fn PopSingleFinalizeJob(&mut self) -> *mut Job {
            let lock = self.mutex_.lock().unwrap();
            if self.finalizable_jobs_.is_empty() {
                return std::ptr::null_mut();
            }

            let job = self.finalizable_jobs_.pop().unwrap();
            drop(lock);
            job
        }

        fn ScheduleIdleTaskFromAnyThread(&mut self, _lock: &std::sync::MutexGuard<()>) {}

        fn FinalizeSingleJob(&mut self) -> bool {
            let job = self.PopSingleFinalizeJob();
            if job == std::ptr::null_mut() {
                return false;
            }
            true
        }

        fn DoBackgroundWork(&mut self, _delegate: *mut JobDelegate) {}

        fn DoIdleWork(&mut self, _deadline_in_seconds: f64) {}

        fn DeleteJob(&mut self, job: *mut Job) {
            let lock = self.mutex_.lock().unwrap();
            self.DeleteJob_locked(job, &lock);
        }

        fn DeleteJob_locked(&mut self, job: *mut Job, _lock: &std::sync::MutexGuard<()>) {
            self.all_jobs_.remove(&job);
            self.jobs_to_dispose_.push(job);
            if self.jobs_to_dispose_.len() == 1 {
                self.num_jobs_for_background_.fetch_add(1, Ordering::Relaxed);
            }
        }

        fn NotifyAddedBackgroundJob(&mut self, lock: &std::sync::MutexGuard<()>) {
            self.num_jobs_for_background_.fetch_add(1, Ordering::Relaxed);
            self.VerifyBackgroundTaskCount(lock);
        }
        fn NotifyRemovedBackgroundJob(&mut self, lock: &std::sync::MutexGuard<()>) {
            self.num_jobs_for_background_.fetch_sub(1, Ordering::Relaxed);
            self.VerifyBackgroundTaskCount(lock);
        }

        fn VerifyBackgroundTaskCount(&mut self, _lock: &std::sync::MutexGuard<()>) {}
    }

    impl Drop for LazyCompileDispatcher {
        fn drop(&mut self) {
            assert!(!unsafe { (*self.job_handle_.as_mut().unwrap()).IsValid() });
        }
    }

    impl LazyCompileDispatcher {
        // JobTask for PostJob API.
        struct JobTask {
            lazy_compile_dispatcher_: *mut LazyCompileDispatcher,
        }
        impl JobTask {
            fn new(lazy_compile_dispatcher: *mut LazyCompileDispatcher) -> Self {
                JobTask {
                    lazy_compile_dispatcher_: lazy_compile_dispatcher,
                }
            }
        }
        impl v8::JobTask for JobTask {
            fn Run(&mut self, delegate: *mut JobDelegate) {
                if let Some(dispatcher) = unsafe { self.lazy_compile_dispatcher_.as_mut() } {
                    dispatcher.DoBackgroundWork(delegate);
                }
            }
            fn GetMaxConcurrency(&self, worker_count: usize) -> usize {
                if let Some(dispatcher) = unsafe { self.lazy_compile_dispatcher_.as_ref() } {
                    let n = dispatcher.num_jobs_for_background_.load(Ordering::Relaxed);
                    if true {
                        return n;
                    }
                }
                0
            }
        }

        struct Job {
            task: std::unique_ptr<BackgroundCompileTask>,
            state: Job::State,
        }
        impl Job {
            fn is_running_on_background(&self) -> bool {
                self.state == Job::State::kRunning || self.state == Job::State::kAbortRequested
            }
        }
        impl Drop for Job {
            fn drop(&mut self) {}
        }
        impl Job {
            #[allow(dead_code)]
            #[derive(PartialEq, Eq)]
            enum State {
                // Background thread states (Enqueue + DoBackgroundWork)
                // ---

                // In the pending task queue.
                kPending,
                // Currently running on a background thread.
                kRunning,
                kAbortRequested,  // ... but we want to drop the result.
                // In the finalizable task queue.
                kReadyToFinalize,
                kAborted,

                // Main thread states (FinishNow and FinalizeSingleJob)
                // ---

                // Popped off the pending task queue.
                kPendingToRunOnForeground,
                // Popped off the finalizable task queue.
                kFinalizingNow,
                kAbortingNow,  // ... and we want to abort

                // Finished finalizing, ready for deletion.
                kFinalized,
            }
        }
    }
}
