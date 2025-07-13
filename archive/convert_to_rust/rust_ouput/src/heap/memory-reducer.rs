// Converted from V8 C++ source files:
// Header: memory-reducer.h
// Implementation: memory-reducer.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod v8 {
    pub mod platform {
        pub struct TaskRunner {}
    }
}
pub mod internal {
    pub use crate::v8::platform::TaskRunner;
    pub struct CancelableTask {
        isolate: *mut Isolate,
    }

    impl CancelableTask {
        pub fn new(isolate: *mut Isolate) -> Self {
            CancelableTask { isolate }
        }

        pub fn isolate(&self) -> *mut Isolate {
            self.isolate
        }
    }
}

pub mod base {
    pub mod macros {
        #[macro_export]
        macro_rules! UNREACHABLE {
            () => {
                panic!("UNREACHABLE");
            };
        }
    }
}

pub mod common {
    pub mod globals {
        pub type Address = usize;
        pub type SizeT = usize;
    }
}

pub mod tasks {
    pub mod cancelable_task {
        pub struct CancelableTask {}
    }
}

pub mod heap {
    pub struct HeapTester {}
    use crate::v8::platform::TaskRunner;
    use crate::internal::CancelableTask;
    use std::sync::Mutex;

    pub struct Heap {
        foreground_task_runner: std::sync::Arc<Mutex<TaskRunner>>,
    }

    impl Heap {
        pub fn GetForegroundTaskRunner(&self) -> std::sync::Arc<Mutex<TaskRunner>> {
            self.foreground_task_runner.clone()
        }
        pub fn IsTearingDown(&self) -> bool {
            false
        }
        pub fn allocator(&self) -> &Allocator {
            &Allocator {}
        }
        pub fn tracer(&self) -> &GCTracer {
            &GCTracer {}
        }
        pub fn isolate(&self) -> &Isolate {
            &Isolate {}
        }
        pub fn incremental_marking(&self) -> &IncrementalMarking {
            &IncrementalMarking {}
        }
        pub fn CommittedOldGenerationMemory(&self) -> usize {
            1024
        }
        pub fn HasLowAllocationRate(&self) -> bool {
            true
        }
        pub fn ShouldOptimizeForMemoryUsage(&self) -> bool {
            true
        }
        pub fn StartIncrementalMarking(
            &self,
            gc_flags: GCFlags,
            garbage_collection_reason: GarbageCollectionReason,
            gc_callback_flags: GCCallbackFlags,
        ) {
        }
        pub fn HasHighFragmentation(&self) -> bool {
            true
        }
        pub fn MonotonicallyIncreasingTimeInMs(&self) -> f64 {
            1.0
        }
        pub fn NewSpaceAllocationCounter(&self) -> usize {
            1024
        }
        pub fn OldGenerationAllocationCounter(&self) -> usize {
            1024
        }
        pub fn EmbedderAllocationCounter(&self) -> usize {
            1024
        }
    }

    pub struct Allocator {}
    impl Allocator {
        pub fn new_space_allocator(&self) -> &NewSpaceAllocator {
            &NewSpaceAllocator {}
        }
    }
    pub struct NewSpaceAllocator {}
    impl NewSpaceAllocator {
        pub fn FreeLinearAllocationArea(&self) {}
    }
    pub struct GCTracer {}
    impl GCTracer {
        pub fn SampleAllocation(
            &self,
            _now: base::TimeTicks,
            _new_space_allocation_counter: usize,
            _old_generation_allocation_counter: usize,
            _embedder_allocation_counter: usize,
        ) {
        }
    }
    pub struct IncrementalMarking {}
    impl IncrementalMarking {
        pub fn IsStopped(&self) -> bool {
            true
        }
        pub fn CanAndShouldBeStarted(&self) -> bool {
            true
        }
    }
    pub enum GCFlags {
        kReduceMemoryFootprint,
        kNoFlags,
    }
    pub enum GarbageCollectionReason {
        kMemoryReducer,
    }
    pub enum GCCallbackFlags {
        kGCCallbackFlagCollectAllExternalMemory,
    }

    use crate::flags::v8_flags;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    pub struct MemoryReducer {
        heap_: *mut Heap,
        taskrunner_: std::shared_ptr::SharedPtr<TaskRunner>,
        state_: State,
        js_calls_counter_: u32,
        js_calls_sample_time_ms_: f64,
        start_delay_ms_: bool,
    }

    impl MemoryReducer {
        pub const kLongDelayMs: i32 = 8000;
        pub const kShortDelayMs: i32 = 500;
        pub const kWatchdogDelayMs: i32 = 100000;
        pub const kCommittedMemoryFactor: f64 = 1.1;
        pub const kCommittedMemoryDelta: usize = 10 * 1024 * 1024;

        pub fn new(heap: *mut Heap) -> Self {
            assert!(v8_flags.incremental_marking);
            assert!(v8_flags.memory_reducer);
            let heap_ref = unsafe { &*heap };
            MemoryReducer {
                heap_: heap,
                taskrunner_: heap_ref.GetForegroundTaskRunner().clone(),
                state_: State::CreateUninitialized(),
                js_calls_counter_: 0,
                js_calls_sample_time_ms_: 0.0,
                start_delay_ms_: false,
            }
        }
        pub fn heap(&self) -> &Heap {
            unsafe { &*self.heap_ }
        }

        pub fn NotifyMarkCompact(&mut self, committed_memory_before: usize) {
            if !v8_flags.incremental_marking {
                return;
            }
            let committed_memory = self.heap().CommittedOldGenerationMemory();
            let event = MemoryReducer::Event {
                type_: MemoryReducer::EventType::kMarkCompact,
                time_ms: self.heap().MonotonicallyIncreasingTimeInMs(),
                committed_memory,
                next_gc_likely_to_collect_more: (committed_memory_before
                    > committed_memory + 1024 * 1024)
                    || self.heap().HasHighFragmentation(),
                should_start_incremental_gc: false,
                can_start_incremental_gc: false,
                is_frozen: MemoryReducer::IsFrozen(self.heap()),
            };
            let old_state = self.state_.clone();
            self.state_ = MemoryReducer::Step(self.state_.clone(), event);
            if old_state.id() != Id::kWait && self.state_.id() == Id::kWait {
                self.ScheduleTimer(self.state_.next_gc_start_ms() - event.time_ms);
            }
            if old_state.id() == Id::kRun && v8_flags.trace_memory_reducer {
                self.heap()
                    .isolate()
                    .PrintWithTimestamp(format!(
                        "Memory reducer: finished GC #{} ({})",
                        old_state.started_gcs(),
                        if self.state_.id() == Id::kWait {
                            "will do more"
                        } else {
                            "done"
                        }
                    )
                    .as_str());
            }
        }

        pub fn NotifyPossibleGarbage(&mut self) {
            if !v8_flags.incremental_marking {
                return;
            }
            let event = MemoryReducer::Event {
                type_: MemoryReducer::EventType::kPossibleGarbage,
                time_ms: self.heap().MonotonicallyIncreasingTimeInMs(),
                committed_memory: 0,
                next_gc_likely_to_collect_more: false,
                should_start_incremental_gc: false,
                can_start_incremental_gc: false,
                is_frozen: MemoryReducer::IsFrozen(self.heap()),
            };
            let old_action = self.state_.id();
            self.state_ = MemoryReducer::Step(self.state_.clone(), event);
            if old_action != Id::kWait && self.state_.id() == Id::kWait {
                self.ScheduleTimer(self.state_.next_gc_start_ms() - event.time_ms);
            }
        }
        fn NotifyTimer(&mut self, event: &Event) {
            if self.state_.id() != Id::kWait {
                return;
            }
            assert_eq!(EventType::kTimer, event.type_);
            self.state_ = MemoryReducer::Step(self.state_.clone(), event);
            if self.state_.id() == Id::kRun {
                assert!(self.heap().incremental_marking().IsStopped());
                assert!(v8_flags.incremental_marking);
                if v8_flags.trace_memory_reducer {
                    self.heap().isolate().PrintWithTimestamp(
                        format!("Memory reducer: started GC #{}", self.state_.started_gcs())
                            .as_str(),
                    );
                }
                let gc_flags = if v8_flags.memory_reducer_favors_memory {
                    GCFlags::kReduceMemoryFootprint
                } else {
                    GCFlags::kNoFlags
                };
                self.heap().StartIncrementalMarking(
                    gc_flags,
                    GarbageCollectionReason::kMemoryReducer,
                    GCCallbackFlags::kGCCallbackFlagCollectAllExternalMemory,
                );
            } else if self.state_.id() == Id::kWait {
                self.ScheduleTimer(self.state_.next_gc_start_ms() - event.time_ms);
                if (v8_flags.trace_memory_reducer) {
                    self.heap().isolate().PrintWithTimestamp(
                        format!(
                            "Memory reducer: waiting for {} ms",
                            self.state_.next_gc_start_ms() - event.time_ms
                        )
                        .as_str(),
                    );
                }
            }
        }
        fn ScheduleTimer(&self, delay_ms: f64) {
            assert!(0.0 < delay_ms);
            if self.heap().IsTearingDown() {
                return;
            }
            let kSlackMs = 100.0;
            let taskrunner = self.taskrunner_.clone();
            let memory_reducer = unsafe { &mut *(self as *const Self as *mut Self) };
            taskrunner.lock().unwrap().PostDelayedTask(
                std::shared_ptr::SharedPtr::new(TimerTask::new(memory_reducer)),
                (delay_ms + kSlackMs) / 1000.0,
            );
        }
        fn Step(state: State, event: Event) -> State {
            assert!(v8_flags.memory_reducer);
            assert!(v8_flags.incremental_marking);

            match state.id() {
                Id::kUninit | Id::kDone => {
                    if event.type_ == EventType::kTimer {
                        return state;
                    } else if event.type_ == EventType::kMarkCompact {
                        if event.committed_memory
                            < std::cmp::max(
                                (state.committed_memory_at_last_run() as f64
                                    * MemoryReducer::kCommittedMemoryFactor)
                                    as usize,
                                state.committed_memory_at_last_run()
                                    + MemoryReducer::kCommittedMemoryDelta,
                            )
                        {
                            return state;
                        } else {
                            return State::CreateWait(
                                0,
                                event.time_ms + MemoryReducer::kLongDelayMs as f64,
                                event.time_ms,
                            );
                        }
                    } else {
                        assert_eq!(EventType::kPossibleGarbage, event.type_);
                        return State::CreateWait(
                            0,
                            event.time_ms + v8_flags.gc_memory_reducer_start_delay_ms as f64,
                            state.last_gc_time_ms(),
                        );
                    }
                }
                Id::kWait => {
                    assert!(state.started_gcs() <= MemoryReducer::MaxNumberOfGCs());
                    match event.type_ {
                        EventType::kPossibleGarbage => return state,
                        EventType::kTimer => {
                            if event.is_frozen || state.started_gcs() >= MemoryReducer::MaxNumberOfGCs() {
                                return State::CreateDone(
                                    state.last_gc_time_ms(),
                                    event.committed_memory,
                                );
                            } else if event.can_start_incremental_gc
                                && (event.should_start_incremental_gc
                                    || MemoryReducer::WatchdogGC(&state, &event))
                            {
                                if state.next_gc_start_ms() <= event.time_ms {
                                    return State::CreateRun(state.started_gcs() + 1);
                                } else {
                                    return state;
                                }
                            } else {
                                return State::CreateWait(
                                    state.started_gcs(),
                                    event.time_ms + MemoryReducer::kLongDelayMs as f64,
                                    state.last_gc_time_ms(),
                                );
                            }
                        }
                        EventType::kMarkCompact => {
                            return State::CreateWait(
                                state.started_gcs(),
                                event.time_ms + MemoryReducer::kLongDelayMs as f64,
                                event.time_ms,
                            );
                        }
                    }
                }
                Id::kRun => {
                    assert!(state.started_gcs() <= MemoryReducer::MaxNumberOfGCs());
                    if event.type_ == EventType::kMarkCompact {
                        if !event.is_frozen
                            && state.started_gcs() < MemoryReducer::MaxNumberOfGCs()
                            && (event.next_gc_likely_to_collect_more || state.started_gcs() == 1)
                        {
                            return State::CreateWait(
                                state.started_gcs(),
                                event.time_ms + MemoryReducer::kShortDelayMs as f64,
                                event.time_ms,
                            );
                        } else {
                            return State::CreateDone(event.time_ms, event.committed_memory);
                        }
                    } else {
                        return state;
                    }
                }
            }
            unreachable!()
        }
        fn WatchdogGC(state: &State, event: &Event) -> bool {
            state.last_gc_time_ms() != 0.0
                && event.time_ms > state.last_gc_time_ms() + MemoryReducer::kWatchdogDelayMs as f64
        }

        pub fn TearDown(&mut self) {
            self.state_ = State::CreateUninitialized();
        }

        pub fn MaxNumberOfGCs() -> i32 {
            assert!(v8_flags.memory_reducer_gc_count > 0);
            return v8_flags.memory_reducer_gc_count;
        }
        pub fn IsFrozen(heap: &Heap) -> bool {
            return v8_flags.memory_reducer_respects_frozen_state && heap.isolate().IsFrozen();
        }

        pub fn ShouldGrowHeapSlowly(&self) -> bool {
            self.state_.id() == Id::kDone
        }
    }

    #[derive(Clone, Copy, PartialEq, Debug)]
    pub enum Id {
        kUninit,
        kDone,
        kWait,
        kRun,
    }

    #[derive(Clone, Copy, Debug)]
    pub struct State {
        id_: Id,
        started_gcs_: i32,
        next_gc_start_ms_: f64,
        last_gc_time_ms_: f64,
        committed_memory_at_last_run_: usize,
    }

    impl State {
        pub fn CreateUninitialized() -> Self {
            State {
                id_: Id::kUninit,
                started_gcs_: 0,
                next_gc_start_ms_: 0.0,
                last_gc_time_ms_: 0.0,
                committed_memory_at_last_run_: 0,
            }
        }

        pub fn CreateDone(last_gc_time_ms: f64, committed_memory: usize) -> Self {
            State {
                id_: Id::kDone,
                started_gcs_: 0,
                next_gc_start_ms_: 0.0,
                last_gc_time_ms,
                committed_memory_at_last_run_: committed_memory,
            }
        }

        pub fn CreateWait(started_gcs: i32, next_gc_time_ms: f64, last_gc_time_ms: f64) -> Self {
            State {
                id_: Id::kWait,
                started_gcs,
                next_gc_start_ms: next_gc_time_ms,
                last_gc_time_ms,
                committed_memory_at_last_run_: 0,
            }
        }

        pub fn CreateRun(started_gcs: i32) -> Self {
            State {
                id_: Id::kRun,
                started_gcs,
                next_gc_start_ms_: 0.0,
                last_gc_time_ms_: 0.0,
                committed_memory_at_last_run_: 0,
            }
        }

        pub fn id(&self) -> Id {
            self.id_
        }

        pub fn started_gcs(&self) -> i32 {
            assert!(self.id() == Id::kWait || self.id() == Id::kRun);
            self.started_gcs_
        }

        pub fn next_gc_start_ms(&self) -> f64 {
            assert_eq!(self.id(), Id::kWait);
            self.next_gc_start_ms_
        }

        pub fn last_gc_time_ms(&self) -> f64 {
            assert!(
                self.id() == Id::kWait || self.id() == Id::kDone || self.id() == Id::kUninit
            );
            self.last_gc_time_ms_
        }
        pub fn committed_memory_at_last_run(&self) -> usize {
            assert!(self.id() == Id::kUninit || self.id() == Id::kDone);
            self.committed_memory_at_last_run_
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub enum EventType {
        kTimer,
        kMarkCompact,
        kPossibleGarbage,
    }

    #[derive(Clone, Copy, Debug)]
    pub struct Event {
        pub type_: EventType,
        pub time_ms: f64,
        pub committed_memory: usize,
        pub next_gc_likely_to_collect_more: bool,
        pub should_start_incremental_gc: bool,
        pub can_start_incremental_gc: bool,
        pub is_frozen: bool,
    }

    struct TimerTask {
        memory_reducer_: *mut MemoryReducer,
    }

    impl TimerTask {
        fn new(memory_reducer: *mut MemoryReducer) -> Self {
            TimerTask {
                memory_reducer_: memory_reducer,
            }
        }
    }

    impl internal::CancelableTask {
        fn RunInternal(&mut self) {
            let memory_reducer = unsafe { &mut *(*self).memory_reducer_ };
            let heap = memory_reducer.heap();
            let time_ms = heap.MonotonicallyIncreasingTimeInMs();

            heap.allocator()
                .new_space_allocator()
                .FreeLinearAllocationArea();
            heap.tracer().SampleAllocation(
                base::TimeTicks::Now(),
                heap.NewSpaceAllocationCounter(),
                heap.OldGenerationAllocationCounter(),
                heap.EmbedderAllocationCounter(),
            );
            let low_allocation_rate = heap.HasLowAllocationRate();
            let optimize_for_memory = heap.ShouldOptimizeForMemoryUsage();

            if v8_flags.trace_memory_reducer {
                heap.isolate().PrintWithTimestamp(
                    format!(
                        "Memory reducer: {}, {}",
                        if low_allocation_rate {
                            "low alloc"
                        } else {
                            "high alloc"
                        },
                        if optimize_for_memory {
                            "background"
                        } else {
                            "foreground"
                        }
                    )
                    .as_str(),
                );
            }

            let event = Event {
                type_: EventType::kTimer,
                time_ms,
                committed_memory: heap.CommittedOldGenerationMemory(),
                next_gc_likely_to_collect_more: false,
                should_start_incremental_gc: low_allocation_rate || optimize_for_memory,
                can_start_incremental_gc: heap.incremental_marking().IsStopped()
                    && heap.incremental_marking().CanAndShouldBeStarted(),
                is_frozen: MemoryReducer::IsFrozen(heap),
            };

            memory_reducer.NotifyTimer(&event);
        }
    }
}

pub mod flags {
    pub struct Flags {}
    impl Flags {
        pub fn new() -> Self {
            Flags {}
        }
    }
    pub static mut v8_flags: Flags = Flags {};
}

pub mod init {
    pub mod v8 {
        pub fn Initialize() {}
    }
}

pub mod utils {
    pub mod utils {
        pub fn Divisor(value: usize, divisor: usize) -> usize {
            value / divisor
        }
    }
}

pub mod isolate {
    pub struct Isolate {}
}

pub mod base {
    pub mod time {
        pub struct TimeTicks {}
    }
}

pub mod i {
    pub use crate::isolate::Isolate;
    pub type IsolateForSandbox = Isolate;
}

impl v8::platform::TaskRunner {
    fn PostDelayedTask<T>(&mut self, task: std::shared_ptr::SharedPtr<T>, delay_in_seconds: f64) {
    }
}

impl MemoryReducer::TimerTask {
    fn RunInternal(&mut self) {
        let memory_reducer = unsafe { &mut *(*self).memory_reducer_ };
        let heap = memory_reducer.heap();
        let time_ms = heap.MonotonicallyIncreasingTimeInMs();

        heap.allocator()
            .new_space_allocator()
            .FreeLinearAllocationArea();
        heap.tracer().SampleAllocation(
            base::TimeTicks::Now(),
            heap.NewSpaceAllocationCounter(),
            heap.OldGenerationAllocationCounter(),
            heap.EmbedderAllocationCounter(),
        );
        let low_allocation_rate = heap.HasLowAllocationRate();
        let optimize_for_memory = heap.ShouldOptimizeForMemoryUsage();

        if v8_flags.trace_memory_reducer {
            heap.isolate().PrintWithTimestamp(
                format!(
                    "Memory reducer: {}, {}",
                    if low_allocation_rate {
                        "low alloc"
                    } else {
                        "high alloc"
                    },
                    if optimize_for_memory {
                        "background"
                    } else {
                        "foreground"
                    }
                )
                .as_str(),
            );
        }

        let event = Event {
            type_: EventType::kTimer,
            time_ms,
            committed_memory: heap.CommittedOldGenerationMemory(),
            next_gc_likely_to_collect_more: false,
            should_start_incremental_gc: low_allocation_rate || optimize_for_memory,
            can_start_incremental_gc: heap.incremental_marking().IsStopped()
                && heap.incremental_marking().CanAndShouldBeStarted(),
            is_frozen: MemoryReducer::IsFrozen(heap),
        };

        memory_reducer.NotifyTimer(&event);
    }
}

impl internal::CancelableTask {
    fn isolate(&self) -> &Isolate {
        unsafe { &*self.isolate }
    }
}

impl Default for flags::Flags {
    fn default() -> Self {
        Self::new()
    }
}

impl Heap {
    pub fn new() -> Self {
        Heap {
            foreground_task_runner: std::sync::Arc::new(Mutex::new(v8::platform::TaskRunner {})),
        }
    }
}

impl Default for Heap {
    fn default() -> Self {
        Self::new()
    }
}

impl base::time::TimeTicks {
    pub fn Now() -> Self {
        Self {}
    }
}

impl i::Isolate {
    fn IsFrozen(&self) -> bool {
        true
    }

    fn PrintWithTimestamp(&self, message: &str) {
        println!("{}", message);
    }

    fn PrintWithTimestamp(&self, message: &str, arg1: &str, arg2: &str) {
        println!("{}", message);
    }
}

pub mod objects {}
