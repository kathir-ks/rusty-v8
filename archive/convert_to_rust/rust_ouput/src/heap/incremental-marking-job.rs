// Converted from V8 C++ source files:
// Header: incremental-marking-job.h
// Implementation: incremental-marking-job.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/heap/incremental-marking-job.h
pub mod incremental_marking_job {
    use std::time::Duration;
    use std::sync::Mutex;
    use crate::heap::heap::Heap;
    use v8::TaskPriority;
    use std::option::Option;
    use v8::base::TimeDelta;
    use v8::base::TimeTicks;
    use std::sync::Arc;
    use v8::TaskRunner;

    pub struct IncrementalMarkingJob {
        heap_: *mut Heap,
        user_blocking_task_runner_: Arc<dyn v8::TaskRunner>,
        user_visible_task_runner_: Arc<dyn v8::TaskRunner>,
        mutex_: Mutex<IncrementalMarkingJobState>,
    }

    struct IncrementalMarkingJobState {
        scheduled_time_: TimeTicks,
        pending_task_: bool,
    }

    impl IncrementalMarkingJob {
        pub fn new(heap: *mut Heap) -> IncrementalMarkingJob {
            let user_blocking_task_runner_ = unsafe { (*heap).GetForegroundTaskRunner(TaskPriority::kUserBlocking) };
            let user_visible_task_runner_ = unsafe { (*heap).GetForegroundTaskRunner(TaskPriority::kUserVisible) };

            IncrementalMarkingJob {
                heap_: heap,
                user_blocking_task_runner_: user_blocking_task_runner_,
                user_visible_task_runner_: user_visible_task_runner_,
                mutex_: Mutex::new(IncrementalMarkingJobState {
                    scheduled_time_: TimeTicks::Now(),
                    pending_task_: false,
                }),
            }
        }

        pub fn schedule_task(&self, priority: TaskPriority) {
            use crate::flags::flags;
            use crate::heap::incremental_marking::IncrementalMarking;
            use crate::execution::isolate::Isolate;
            use crate::heap::heap::HeapIncrementalMarkingLimit;
            use crate::heap::gc_tracer::GarbageCollectionReason;
            use crate::heap::gc_tracer::kGCCallbackScheduleIdleGarbageCollection;
            use crate::execution::stack_guard::StackState;
            use crate::execution::embedder_state::EmbedderStackStateScope;
            use crate::execution::embedder_state::EmbedderStackStateOrigin;
            use crate::base::macros::V8_UNLIKELY;

            let mut guard = self.mutex_.lock().unwrap();

            if guard.pending_task_ || unsafe { (*self.heap_).IsTearingDown() } {
                return;
            }

            let incremental_marking = unsafe { (*self.heap_).incremental_marking() };

            let task_runner = if flags::incremental_marking_start_user_visible() && incremental_marking.IsStopped() && (priority != TaskPriority::kUserBlocking) {
                self.user_visible_task_runner_.clone()
            } else {
                self.user_blocking_task_runner_.clone()
            };

            let task_runner_ref = task_runner.as_ref();
            let non_nestable_tasks_enabled = task_runner_ref.non_nestable_tasks_enabled();
            let task = Task::new(unsafe { (*self.heap_).isolate() }, self, if non_nestable_tasks_enabled { StackState::kNoHeapPointers } else { StackState::kMayContainHeapPointers });

            if non_nestable_tasks_enabled {
                task_runner.post_non_nestable_task(Box::new(task));
            } else {
                task_runner.post_task(Box::new(task));
            }

            guard.pending_task_ = true;
            guard.scheduled_time_ = TimeTicks::Now();

            if V8_UNLIKELY(flags::trace_incremental_marking()) {
                unsafe { (*(*self.heap_).isolate()).PrintWithTimestamp("[IncrementalMarking] Job: Schedule\n"); }
            }
        }

        pub fn average_time_to_task(&self) -> Option<TimeDelta> {
            unsafe { (*self.heap_).tracer().AverageTimeToIncrementalMarkingTask() }
        }

        pub fn current_time_to_task(&self) -> Option<TimeDelta> {
            let guard = self.mutex_.lock().unwrap();
            if guard.pending_task_ {
                let now = TimeTicks::Now();
                assert!(now >= guard.scheduled_time_);
                Some(now - guard.scheduled_time_)
            } else {
                None
            }
        }
    }

    trait CancelableTaskTrait {
        fn run_internal(&mut self);
        fn isolate(&self) -> *mut Isolate;
    }
    
    pub struct Task {
        isolate_: *mut Isolate,
        job_: *const IncrementalMarkingJob,
        stack_state_: StackState,
    }
    
    impl Task {
        fn new(isolate_: *mut Isolate, job_: &IncrementalMarkingJob, stack_state_: StackState) -> Self {
            Task {
                isolate_: isolate_,
                job_: job_,
                stack_state_: stack_state_,
            }
        }
    }
    
    use crate::execution::isolate::Isolate;
    use crate::execution::vm_state::VMState;
    use crate::execution::vm_state::GC;
    use crate::execution::stack_guard::StackState;
    use crate::heap::heap::Heap;
    use crate::flags;
    use crate::heap::incremental_marking::IncrementalMarking;
    use crate::heap::heap::HeapIncrementalMarkingLimit;
    use crate::heap::gc_tracer::GarbageCollectionReason;
    use crate::heap::gc_tracer::kGCCallbackScheduleIdleGarbageCollection;
    use crate::execution::embedder_state::EmbedderStackStateScope;
    use crate::execution::embedder_state::EmbedderStackStateOrigin;
    use crate::base::macros::V8_UNLIKELY;
    use crate::ptr_compr::PtrComprCageAccessScope;

    impl CancelableTaskTrait for Task {
        fn run_internal(&mut self) {
            unsafe {
                let isolate_ = self.isolate_ as *mut Isolate;
                let state = VMState::<GC>::new(isolate_);
                let ptr_compr_cage_access_scope = PtrComprCageAccessScope::new(isolate_);

                (*isolate_).stack_guard().ClearStartIncrementalMarking();

                let heap = (*isolate_).heap();

                {
                    let job = &(*self.job_);
                    let mut guard = job.mutex_.lock().unwrap();
                    (*heap).tracer().RecordTimeToIncrementalMarkingTask(TimeTicks::Now() - guard.scheduled_time_);
                    guard.scheduled_time_ = TimeTicks::Now();
                }

                let scope = EmbedderStackStateScope::new(heap, EmbedderStackStateOrigin::kImplicitThroughTask, self.stack_state_);

                let incremental_marking = (*heap).incremental_marking();
                if incremental_marking.IsStopped() {
                    if (*heap).IncrementalMarkingLimitReached() != HeapIncrementalMarkingLimit::kNoLimit {
                        (*heap).StartIncrementalMarking((*heap).GCFlagsForIncrementalMarking(), GarbageCollectionReason::kTask, kGCCallbackScheduleIdleGarbageCollection);
                    } else if flags::minor_ms() && flags::concurrent_minor_ms_marking() {
                        (*heap).StartMinorMSIncrementalMarkingIfNeeded();
                    }
                }

                {
                    let job = &(*self.job_);
                    let mut guard = job.mutex_.lock().unwrap();
                    if V8_UNLIKELY(flags::trace_incremental_marking()) {
                        (*(*job.heap_).isolate()).PrintWithTimestamp("[IncrementalMarking] Job: Run\n");
                    }
                    guard.pending_task_ = false;
                }

                if incremental_marking.IsMajorMarking() {
                    (*heap).incremental_marking().AdvanceAndFinalizeIfComplete();
                    if incremental_marking.IsMajorMarking() {
                        if V8_UNLIKELY(flags::trace_incremental_marking()) {
                            (*isolate_).PrintWithTimestamp("[IncrementalMarking] Using regular task based on flags\n");
                        }
                        let job = &(*self.job_);
                        job.schedule_task(v8::TaskPriority::kUserBlocking);
                    }
                }
            }
        }

        fn isolate(&self) -> *mut Isolate {
            self.isolate_
        }
    }
    
    impl v8::Task for Task {
        fn run(&mut self) {
            let mut trait_object: Box<dyn CancelableTaskTrait> = Box::new(Task {
                isolate_: self.isolate_,
                job_: self.job_,
                stack_state_: self.stack_state_,
            });
            trait_object.run_internal();
        }
    }
}

pub mod v8 {
    pub trait Task {
        fn run(&mut self);
    }
    
    pub trait TaskRunner {
        fn NonNestableTasksEnabled(&self) -> bool;
        fn post_task(&self, task: Box<dyn Task>);
        fn post_non_nestable_task(&self, task: Box<dyn Task>);
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum TaskPriority {
        k низкой,
        kUserBlocking,
        kUserVisible,
        kBestEffort,
    }
}

pub mod base {
    #[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
    pub struct TimeDelta {
        micros: i64,
    }

    impl TimeDelta {
        pub fn new(micros: i64) -> Self {
            TimeDelta { micros }
        }
    }

    use std::time::{SystemTime, UNIX_EPOCH};

    #[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
    pub struct TimeTicks {
        micros: i64,
    }

    impl TimeTicks {
        pub fn Now() -> Self {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            TimeTicks {
                micros: now.as_micros() as i64,
            }
        }
    }

    impl std::ops::Sub for TimeTicks {
        type Output = TimeDelta;

        fn sub(self, other: Self) -> Self::Output {
            TimeDelta::new(self.micros - other.micros)
        }
    }

    impl std::ops::Add<TimeDelta> for TimeTicks {
        type Output = TimeTicks;

        fn add(self, other: TimeDelta) -> Self::Output {
            TimeTicks {
                micros: self.micros + other.micros,
            }
        }
    }

    impl std::ops::Sub<TimeDelta> for TimeTicks {
        type Output = TimeTicks;

        fn sub(self, other: TimeDelta) -> Self::Output {
            TimeTicks {
                micros: self.micros - other.micros,
            }
        }
    }
}

pub mod flags {
    pub fn incremental_marking_start_user_visible() -> bool {
        true
    }
    pub fn minor_ms() -> bool {
        true
    }
    pub fn concurrent_minor_ms_marking() -> bool {
        true
    }
    pub fn trace_incremental_marking() -> bool {
        false
    }
}

pub mod ptr_compr {
    pub struct PtrComprCageAccessScope {}

    impl PtrComprCageAccessScope {
        pub fn new(_isolate: *mut crate::execution::isolate::Isolate) -> Self {
            PtrComprCageAccessScope {}
        }
    }
}
