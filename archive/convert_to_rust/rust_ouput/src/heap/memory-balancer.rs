// Converted from V8 C++ source files:
// Header: memory-balancer.h
// Implementation: memory-balancer.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod platform {
        pub struct TimeTicks {
            pub time: u64,
        }

        impl TimeTicks {
            pub fn Now() -> Self {
                Self {
                    time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64,
                }
            }

            pub fn to_rfc3339(&self) -> String {
                let seconds = self.time / 1000;
                let nanos = (self.time % 1000) * 1_000_000;
                let datetime = chrono::NaiveDateTime::from_timestamp_opt(seconds as i64, nanos as u32).unwrap();
                datetime.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
            }

            pub fn duration_since(&self, earlier: &TimeTicks) -> TimeDelta {
                TimeDelta {
                    milliseconds: (self.time - earlier.time) as f64,
                }
            }
        }
        #[derive(Debug, Clone, Copy)]
        pub struct TimeDelta {
            pub milliseconds: f64,
        }

        impl TimeDelta {
            pub fn InMillisecondsF(&self) -> f64 {
                self.milliseconds
            }
        }
    }
}

pub mod tasks {
    use std::sync::Mutex;

    pub struct CancelableTask {
        isolate: *mut super::internal::Isolate,
        is_cancelled: Mutex<bool>,
    }

    impl CancelableTask {
        pub fn new(isolate: *mut super::internal::Isolate) -> Self {
            CancelableTask {
                isolate,
                is_cancelled: Mutex::new(false),
            }
        }

        pub fn cancel(&self) {
            let mut cancelled = self.is_cancelled.lock().unwrap();
            *cancelled = true;
        }

        pub fn is_cancelled(&self) -> bool {
            *self.is_cancelled.lock().unwrap()
        }

        pub fn run_internal(&self) {}
    }
}

pub mod internal {
    use std::{
        cell::RefCell,
        rc::Rc,
        sync::{Arc, Mutex},
    };

    use crate::base::platform::{TimeDelta, TimeTicks};

    #[derive(Debug, Clone, Copy)]
    pub struct Isolate {}

    impl Isolate {
        pub fn PrintWithTimestamp(&self, format: &str, args: ...) {
            println!("{}", format);
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Heap {
        isolate: *mut Isolate,
        old_generation_size_of_objects: usize,
        max_old_generation_size: usize,
        min_old_generation_size: usize,
        foreground_task_runner: Rc<ForegroundTaskRunner>,
    }

    impl Heap {
        pub fn new(
            isolate: *mut Isolate,
            old_generation_size_of_objects: usize,
            max_old_generation_size: usize,
            min_old_generation_size: usize,
        ) -> Self {
            Heap {
                isolate,
                old_generation_size_of_objects,
                max_old_generation_size,
                min_old_generation_size,
                foreground_task_runner: Rc::new(ForegroundTaskRunner::new()),
            }
        }
        pub fn OldGenerationSizeOfObjects(&self) -> usize {
            self.old_generation_size_of_objects
        }

        pub fn max_old_generation_size(&self) -> usize {
            self.max_old_generation_size
        }

        pub fn min_old_generation_size(&self) -> usize {
            self.min_old_generation_size
        }

        pub fn isolate(&self) -> *mut Isolate {
            self.isolate
        }

        pub fn SetOldGenerationAndGlobalAllocationLimit(&mut self, new_limit: usize, global_allocation_limit: usize) {
            self.old_generation_size_of_objects = new_limit;
        }

        pub fn GetForegroundTaskRunner(&self) -> Rc<ForegroundTaskRunner> {
            self.foreground_task_runner.clone()
        }
    }

    pub struct ForegroundTaskRunner {}

    impl ForegroundTaskRunner {
        pub fn new() -> Self {
            ForegroundTaskRunner {}
        }

        pub fn PostDelayedTask(&self, task: std::boxed::Box<HeartbeatTask>, delay_in_ms: i32) {
            std::thread::sleep(std::time::Duration::from_millis(delay_in_ms as u64));
            task.RunInternal();
        }
    }

    const KB: f64 = 1024.0;
    const MB: usize = 1024 * 1024;

    lazy_static::lazy_static! {
        pub static ref v8_flags: V8Flags = V8Flags {
            trace_memory_balancer: false,
            memory_balancer_c_value: 1.0,
        };
    }

    pub struct V8Flags {
        pub trace_memory_balancer: bool,
        pub memory_balancer_c_value: f64,
    }

    pub struct MemoryBalancer {
        heap_: *mut Heap,
        live_memory_after_gc_: usize,
        embedder_allocation_limit_: usize,
        major_allocation_rate_: Option<SmoothedBytesAndDuration>,
        major_gc_speed_: Option<SmoothedBytesAndDuration>,
        last_measured_memory_: usize,
        last_measured_at_: TimeTicks,
        heartbeat_task_started_: bool,
    }

    impl MemoryBalancer {
        pub fn new(heap: *mut Heap, startup_time: TimeTicks) -> Self {
            MemoryBalancer {
                heap_: heap,
                live_memory_after_gc_: 0,
                embedder_allocation_limit_: 0,
                major_allocation_rate_: None,
                major_gc_speed_: None,
                last_measured_memory_: 0,
                last_measured_at_: startup_time,
                heartbeat_task_started_: false,
            }
        }

        pub fn RecomputeLimits(&mut self, embedder_allocation_limit: usize, time: TimeTicks) {
            self.embedder_allocation_limit_ = embedder_allocation_limit;
            unsafe {
                self.last_measured_memory_ = (*self.heap_).OldGenerationSizeOfObjects();
                self.live_memory_after_gc_ = (*self.heap_).OldGenerationSizeOfObjects();
            }
            self.last_measured_at_ = time;
            self.RefreshLimit();
            self.PostHeartbeatTask();
        }

        fn RefreshLimit(&mut self) {
            if self.major_allocation_rate_.is_none() || self.major_gc_speed_.is_none() {
                return;
            }

            let major_allocation_rate = self.major_allocation_rate_.as_ref().unwrap();
            let major_gc_speed = self.major_gc_speed_.as_ref().unwrap();

            let computed_limit = self.live_memory_after_gc_ as f64
                + (self.live_memory_after_gc_ as f64
                    * (major_allocation_rate.rate() / major_gc_speed.rate() / v8_flags.memory_balancer_c_value))
                    .sqrt();
            let computed_limit = computed_limit as usize;

            let k_min_heap_extra_space = 2 * MB;
            let minimum_limit = self.live_memory_after_gc_ + k_min_heap_extra_space;

            let mut new_limit = std::cmp::max(minimum_limit, computed_limit);

            unsafe {
                new_limit = std::cmp::min(new_limit, (*self.heap_).max_old_generation_size());
                new_limit = std::cmp::max(new_limit, (*self.heap_).min_old_generation_size());
            }
            if v8_flags.trace_memory_balancer {
                unsafe {
                    ((*self.heap_).isolate() as *mut Isolate).as_mut().unwrap().PrintWithTimestamp(
                        "MemoryBalancer: allocation-rate=%.1lfKB/ms gc-speed=%.1lfKB/ms \
                         minium-limit=%.1lfM computed-limit=%.1lfM new-limit=%.1lfM\n",
                        major_allocation_rate.rate() / KB,
                        major_gc_speed.rate() / KB,
                        minimum_limit as f64 / MB as f64,
                        computed_limit as f64 / MB as f64,
                        new_limit as f64 / MB as f64,
                    );
                }
            }

            unsafe {
                (*self.heap_).SetOldGenerationAndGlobalAllocationLimit(
                    new_limit,
                    new_limit + self.embedder_allocation_limit_,
                );
            }
        }

        pub fn UpdateGCSpeed(&mut self, major_gc_bytes: usize, major_gc_duration: TimeDelta) {
            if self.major_gc_speed_.is_none() {
                self.major_gc_speed_ = Some(SmoothedBytesAndDuration::new(
                    major_gc_bytes,
                    major_gc_duration.InMillisecondsF(),
                ));
            } else {
                self.major_gc_speed_.as_mut().unwrap().Update(
                    major_gc_bytes,
                    major_gc_duration.InMillisecondsF(),
                    Self::K_MAJOR_GC_DECAY_RATE,
                );
            }
        }

        pub fn UpdateAllocationRate(
            &mut self,
            major_allocation_bytes: usize,
            major_allocation_duration: TimeDelta,
        ) {
            if self.major_allocation_rate_.is_none() {
                self.major_allocation_rate_ = Some(SmoothedBytesAndDuration::new(
                    major_allocation_bytes,
                    major_allocation_duration.InMillisecondsF(),
                ));
            } else {
                self.major_allocation_rate_.as_mut().unwrap().Update(
                    major_allocation_bytes,
                    major_allocation_duration.InMillisecondsF(),
                    Self::K_MAJOR_ALLOCATION_DECAY_RATE,
                );
            }
        }

        pub fn HeartbeatUpdate(&mut self) {
            self.heartbeat_task_started_ = false;
            let time = TimeTicks::Now();
            unsafe {
                let memory = (*self.heap_).OldGenerationSizeOfObjects();
                let duration = time.duration_since(&self.last_measured_at_);
                let allocated_bytes = if memory > self.last_measured_memory_ {
                    memory - self.last_measured_memory_
                } else {
                    0
                };

                self.UpdateAllocationRate(allocated_bytes, duration);

                self.last_measured_memory_ = memory;
                self.last_measured_at_ = time;
                self.RefreshLimit();
                self.PostHeartbeatTask();
            }
        }

        fn PostHeartbeatTask(&mut self) {
            if self.heartbeat_task_started_ {
                return;
            }
            self.heartbeat_task_started_ = true;
            unsafe {
                let task = HeartbeatTask::new((*self.heap_).isolate(), self);
                (*self.heap_).GetForegroundTaskRunner().PostDelayedTask(
                    std::boxed::Box::new(task),
                    1,
                );
            }
        }

        const K_MAJOR_ALLOCATION_DECAY_RATE: f64 = 0.95;
        const K_MAJOR_GC_DECAY_RATE: f64 = 0.5;
    }

    struct SmoothedBytesAndDuration {
        bytes_: f64,
        duration_: f64,
    }

    impl SmoothedBytesAndDuration {
        fn new(bytes: usize, duration: f64) -> Self {
            SmoothedBytesAndDuration {
                bytes_: bytes as f64,
                duration_: duration,
            }
        }

        fn Update(&mut self, bytes: usize, duration: f64, decay_rate: f64) {
            self.bytes_ = self.bytes_ * decay_rate + bytes as f64 * (1.0 - decay_rate);
            self.duration_ = self.duration_ * decay_rate + duration * (1.0 - decay_rate);
        }

        fn rate(&self) -> f64 {
            self.bytes_ / self.duration_
        }
    }

    pub struct HeartbeatTask {
        isolate: *mut Isolate,
        mb_: *mut MemoryBalancer,
    }

    impl HeartbeatTask {
        pub fn new(isolate: *mut Isolate, mb: *mut MemoryBalancer) -> Self {
            HeartbeatTask { isolate, mb_: mb }
        }

        pub fn RunInternal(&self) {
            unsafe {
                (*self.mb_).HeartbeatUpdate();
            }
        }
    }
}
