// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::cmp::{max, min};
use std::time::{Duration, Instant};

//use base::TimeTicks;  // Assuming a simple Instant replacement
//use base::TimeDelta;  // Assuming a simple Duration replacement

const KB: usize = 1024;
const MB: usize = 1024 * KB;

mod flags {
    pub static mut trace_memory_balancer: bool = false;
    pub static mut memory_balancer_c_value: f64 = 1.0; // Default value
}

struct SmoothedBytesAndDuration {
    bytes: usize,
    duration_ms: f64,
}

impl SmoothedBytesAndDuration {
    fn new(bytes: usize, duration_ms: f64) -> Self {
        SmoothedBytesAndDuration {
            bytes,
            duration_ms,
        }
    }

    fn update(&mut self, new_bytes: usize, new_duration_ms: f64, decay_rate: f64) {
        self.bytes = (decay_rate * self.bytes as f64 + (1.0 - decay_rate) * new_bytes as f64) as usize;
        self.duration_ms = (decay_rate * self.duration_ms + (1.0 - decay_rate) * new_duration_ms) as f64;
    }

    fn rate(&self) -> f64 {
        if self.duration_ms == 0.0 {
            0.0
        } else {
            self.bytes as f64 / self.duration_ms
        }
    }
}

trait ForegroundTaskRunner {
    fn post_delayed_task(&self, task: Box<dyn FnOnce() + Send + 'static>, delay: Duration);
}

trait Isolate {
    fn print_with_timestamp(&self, format: &str, args: &[f64]);
}

trait Heap {
    fn old_generation_size_of_objects(&self) -> usize;
    fn max_old_generation_size(&self) -> usize;
    fn min_old_generation_size(&self) -> usize;
    fn set_old_generation_and_global_allocation_limit(&self, limit: usize, global_limit: usize);
    fn get_foreground_task_runner(&self) -> &dyn ForegroundTaskRunner;
    fn isolate(&self) -> &dyn Isolate;
}

const kMajorGCDecayRate: f64 = 0.9;
const kMajorAllocationDecayRate: f64 = 0.9;

pub struct MemoryBalancer<'a> {
    heap_: &'a dyn Heap,
    last_measured_at_: Instant,
    last_measured_memory_: usize,
    live_memory_after_gc_: usize,
    embedder_allocation_limit_: usize,
    major_allocation_rate_: Option<SmoothedBytesAndDuration>,
    major_gc_speed_: Option<SmoothedBytesAndDuration>,
    heartbeat_task_started_: bool,
}

impl<'a> MemoryBalancer<'a> {
    pub fn new(heap_: &'a dyn Heap, startup_time: Instant) -> Self {
        MemoryBalancer {
            heap_,
            last_measured_at_: startup_time,
            last_measured_memory_: 0,
            live_memory_after_gc_: 0,
            embedder_allocation_limit_: 0,
            major_allocation_rate_: None,
            major_gc_speed_: None,
            heartbeat_task_started_: false,
        }
    }

    pub fn recompute_limits(&mut self, embedder_allocation_limit: usize, time: Instant) {
        self.embedder_allocation_limit_ = embedder_allocation_limit;
        self.last_measured_memory_ = self.heap_.old_generation_size_of_objects();
        self.live_memory_after_gc_ = self.last_measured_memory_;
        self.last_measured_at_ = time;
        self.refresh_limit();
        self.post_heartbeat_task();
    }

    fn refresh_limit(&mut self) {
        assert!(self.major_allocation_rate_.is_some());
        assert!(self.major_gc_speed_.is_some());

        let computed_limit = (self.live_memory_after_gc_ as f64 +
            (self.live_memory_after_gc_ as f64 * self.major_allocation_rate_.as_ref().unwrap().rate() /
             self.major_gc_speed_.as_ref().unwrap().rate() / unsafe { flags::memory_balancer_c_value })
                .sqrt()) as usize;

        const K_MIN_HEAP_EXTRA_SPACE: usize = 2 * MB;
        let minimum_limit = self.live_memory_after_gc_ + K_MIN_HEAP_EXTRA_SPACE;

        let mut new_limit = max(minimum_limit, computed_limit);
        new_limit = min(new_limit, self.heap_.max_old_generation_size());
        new_limit = max(new_limit, self.heap_.min_old_generation_size());

        if unsafe { flags::trace_memory_balancer } {
            self.heap_.isolate().print_with_timestamp(
                "MemoryBalancer: allocation-rate=%.1lfKB/ms gc-speed=%.1lfKB/ms minium-limit=%.1lfM computed-limit=%.1lfM new-limit=%.1lfM\n",
                &[
                    self.major_allocation_rate_.as_ref().unwrap().rate() / KB as f64,
                    self.major_gc_speed_.as_ref().unwrap().rate() / KB as f64,
                    minimum_limit as f64 / MB as f64,
                    computed_limit as f64 / MB as f64,
                    new_limit as f64 / MB as f64,
                ],
            );
        }

        self.heap_.set_old_generation_and_global_allocation_limit(
            new_limit,
            new_limit + self.embedder_allocation_limit_,
        );
    }

    pub fn update_gc_speed(&mut self, major_gc_bytes: usize, major_gc_duration: Duration) {
        if self.major_gc_speed_.is_none() {
            self.major_gc_speed_ = Some(SmoothedBytesAndDuration::new(
                major_gc_bytes,
                major_gc_duration.as_millis() as f64,
            ));
        } else {
            self.major_gc_speed_.as_mut().unwrap().update(
                major_gc_bytes,
                major_gc_duration.as_millis() as f64,
                kMajorGCDecayRate,
            );
        }
    }

    pub fn update_allocation_rate(
        &mut self,
        major_allocation_bytes: usize,
        major_allocation_duration: Duration,
    ) {
        if self.major_allocation_rate_.is_none() {
            self.major_allocation_rate_ = Some(SmoothedBytesAndDuration::new(
                major_allocation_bytes,
                major_allocation_duration.as_millis() as f64,
            ));
        } else {
            self.major_allocation_rate_.as_mut().unwrap().update(
                major_allocation_bytes,
                major_allocation_duration.as_millis() as f64,
                kMajorAllocationDecayRate,
            );
        }
    }

    fn heartbeat_update(&mut self) {
        self.heartbeat_task_started_ = false;
        let time = Instant::now();
        let memory = self.heap_.old_generation_size_of_objects();

        let duration = time.duration_since(self.last_measured_at_);
        let allocated_bytes = if memory > self.last_measured_memory_ {
            memory - self.last_measured_memory_
        } else {
            0
        };
        self.update_allocation_rate(allocated_bytes, duration);

        self.last_measured_memory_ = memory;
        self.last_measured_at_ = time;
        self.refresh_limit();
        self.post_heartbeat_task();
    }

    fn post_heartbeat_task(&mut self) {
        if self.heartbeat_task_started_ {
            return;
        }
        self.heartbeat_task_started_ = true;
        let isolate = self.heap_.isolate();
        let mb_ptr = self as *mut Self;
        self.heap_.get_foreground_task_runner().post_delayed_task(
            Box::new(move || {
                //This is unsafe, but it's mirroring the original C++ code's behavior
                // of passing a raw pointer to the task.
                unsafe {
                    (*mb_ptr).heartbeat_update();
                }
            }),
            Duration::from_millis(1),
        );
    }
}

struct HeartbeatTask<'a> {
    isolate: &'a dyn Isolate,
    mb_: *mut MemoryBalancer<'a>, //raw pointer mirrors C++ version.  Rust usually avoids this.
}

impl<'a> HeartbeatTask<'a> {
    fn new(isolate: &'a dyn Isolate, mb_: &mut MemoryBalancer<'a>) -> Self {
        HeartbeatTask { isolate, mb_: mb_ }
    }

    fn run_internal(&mut self) {
        unsafe {
            (*self.mb_).heartbeat_update();
        }
    }
}