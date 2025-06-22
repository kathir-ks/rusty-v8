// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

/// Incremental marking schedule that assumes a fixed time window for scheduling
/// incremental marking steps.
///
/// Usage:
/// 1. `notify_incremental_marking_start()`
/// 2. Any combination of:
///   -> `update_mutator_thread_marked_bytes(mutator_marked_bytes)`
///   -> `add_concurrently_marked_bytes(concurrently_marked_bytes_delta)`
///   -> `mark_synchronously(get_next_incremental_step_duration(estimated_live_size))`
pub struct IncrementalMarkingSchedule {
    incremental_marking_start_time: Option<Instant>,
    mutator_thread_marked_bytes: usize,
    concurrently_marked_bytes: AtomicUsize,
    last_estimated_live_bytes: usize,
    ephemeron_pairs_flushing_ratio_target: f64,
    current_step: StepInfo,
    min_marked_bytes_per_step: usize,
    predictable_schedule: bool,
    elapsed_time_override: Option<Duration>,
    last_concurrently_marked_bytes: usize,
    last_concurrently_marked_bytes_update: Option<Instant>,
}

/// Holds information about a step in the incremental marking schedule.
#[derive(Clone, Copy)]
pub struct StepInfo {
    pub mutator_marked_bytes: usize,
    pub concurrent_marked_bytes: usize,
    pub estimated_live_bytes: usize,
    pub expected_marked_bytes: usize,
    pub elapsed_time: Duration,
}

impl StepInfo {
    /// Returns the sum of mutator and concurrent marked bytes.
    pub fn marked_bytes(&self) -> usize {
        self.mutator_marked_bytes + self.concurrent_marked_bytes
    }

    /// Returns the schedule delta in bytes. Positive and negative delta values
    /// indicate that the marked bytes are ahead and behind the expected
    /// schedule, respectively.
    pub fn scheduled_delta_bytes(&self) -> i64 {
        self.marked_bytes() as i64 - self.expected_marked_bytes as i64
    }

    /// Returns whether the schedule is behind the expectation.
    pub fn is_behind_expectation(&self) -> bool {
        self.scheduled_delta_bytes() < 0
    }
}

impl Default for StepInfo {
    fn default() -> Self {
        StepInfo {
            mutator_marked_bytes: 0,
            concurrent_marked_bytes: 0,
            estimated_live_bytes: 0,
            expected_marked_bytes: 0,
            elapsed_time: Duration::from_millis(0),
        }
    }
}

impl IncrementalMarkingSchedule {
    /// Estimated walltime duration of incremental marking per GC cycle. This value
    /// determines how the mutator thread will try to catch up on incremental
    /// marking steps.
    pub const ESTIMATED_MARKING_TIME: Duration = Duration::from_millis(500);

    /// Step size used when no progress is being made. This step size should allow
    /// for finalizing marking.
    pub const STEP_SIZE_WHEN_NOT_MAKING_PROGRESS: usize = 64 * 1024;

    const EPHEMERON_PAIRS_FLUSHING_RATIO_INCREMENTS: f64 = 0.25;

    fn new(min_marked_bytes_per_step: usize, predictable_schedule: bool) -> Self {
        IncrementalMarkingSchedule {
            incremental_marking_start_time: None,
            mutator_thread_marked_bytes: 0,
            concurrently_marked_bytes: AtomicUsize::new(0),
            last_estimated_live_bytes: 0,
            ephemeron_pairs_flushing_ratio_target: Self::EPHEMERON_PAIRS_FLUSHING_RATIO_INCREMENTS,
            current_step: StepInfo::default(),
            min_marked_bytes_per_step,
            predictable_schedule,
            elapsed_time_override: None,
            last_concurrently_marked_bytes: 0,
            last_concurrently_marked_bytes_update: None,
        }
    }

    /// Creates a new IncrementalMarkingSchedule.
    pub fn create(predictable_schedule: bool) -> Box<Self> {
        Box::new(Self::new(0, predictable_schedule))
    }

    /// Creates a new IncrementalMarkingSchedule with a specific minimum marked bytes per step for testing.
    pub fn create_with_marked_bytes_per_step_for_testing(
        min_marked_bytes_per_step: usize,
        predictable_schedule: bool,
    ) -> Box<Self> {
        Box::new(Self::new(min_marked_bytes_per_step, predictable_schedule))
    }

    /// Notifies the schedule that incremental marking has been started. Can be
    /// called multiple times and is a nop after the first notification.
    pub fn notify_incremental_marking_start(&mut self) {
        if self.incremental_marking_start_time.is_none() {
            self.incremental_marking_start_time = Some(Instant::now());
        }
    }

    /// Notifies the schedule that concurrent marking has been started. Can be
    /// called multiple times and is a nop after the first notification.
    pub fn notify_concurrent_marking_start(&mut self) {
        // No-op in this implementation.
    }

    /// Adds bytes marked on the mutator thread. Must be called from the
    /// thread owning the schedule. The schedule supports marked bytes being
    /// adjusted downwards, i.e., going backwards in the schedule.
    pub fn add_mutator_thread_marked_bytes(&mut self, bytes: usize) {
        self.mutator_thread_marked_bytes += bytes;
    }

    /// Removes bytes marked on the mutator thread.
    pub fn remove_mutator_thread_marked_bytes(&mut self, bytes: usize) {
        if bytes > self.mutator_thread_marked_bytes {
            self.mutator_thread_marked_bytes = 0;
        } else {
            self.mutator_thread_marked_bytes -= bytes;
        }
    }

    /// Adds concurrently marked bytes. May be called from any thread. Not required
    /// to be complete, i.e., it is okay to not report bytes already marked for the
    /// schedule.
    pub fn add_concurrently_marked_bytes(&self, bytes: usize) {
        self.concurrently_marked_bytes.fetch_add(bytes, Ordering::Relaxed);
        self.last_concurrently_marked_bytes_update = Some(Instant::now());
    }

    /// Computes the next step duration based on reported marked bytes and the
    /// current `estimated_live_bytes`.
    pub fn get_next_incremental_step_duration(&mut self, estimated_live_bytes: usize) -> usize {
        let start_time = self.incremental_marking_start_time.unwrap_or_else(|| Instant::now());
        let elapsed_time = self.elapsed_time_override.unwrap_or_else(|| start_time.elapsed());
        self.current_step.elapsed_time = elapsed_time;
        self.current_step.estimated_live_bytes = estimated_live_bytes;

        let overall_marked_bytes = self.get_overall_marked_bytes();
        let expected_marked_bytes = (elapsed_time.as_secs_f64()
            / Self::ESTIMATED_MARKING_TIME.as_secs_f64()
            * estimated_live_bytes as f64) as usize;

        self.current_step.mutator_marked_bytes = self.mutator_thread_marked_bytes;
        self.current_step.concurrent_marked_bytes =
            self.concurrently_marked_bytes.load(Ordering::Relaxed);
        self.current_step.expected_marked_bytes = expected_marked_bytes;

        self.elapsed_time_override = None;

        std::cmp::max(
            self.min_marked_bytes_per_step,
            Self::STEP_SIZE_WHEN_NOT_MAKING_PROGRESS,
        )
    }

    /// Returns the step info for the current step. This function is most useful
    /// after calling `get_next_incremental_step_duration()` to report scheduling
    /// details.
    pub fn get_current_step_info(&self) -> StepInfo {
        self.current_step
    }

    /// Returns whether locally cached ephemerons should be flushed and made
    /// available globally. Will only return true once every
    /// `EPHEMERON_PAIRS_FLUSHING_RATIO_INCREMENTS` percent of overall marked bytes.
    pub fn should_flush_ephemeron_pairs(&mut self) -> bool {
        let overall_marked_bytes = self.get_overall_marked_bytes();
        let target = (self.ephemeron_pairs_flushing_ratio_target * self.last_estimated_live_bytes as f64) as usize;

        if overall_marked_bytes >= target {
            self.ephemeron_pairs_flushing_ratio_target += Self::EPHEMERON_PAIRS_FLUSHING_RATIO_INCREMENTS;
            return true;
        }

        false
    }

    /// Returns the time span since concurrent marking has added marked bytes via
    /// `add_concurrently_marked_bytes()`. Note that instead of updating the time on
    /// adding bytes we only update time in
    /// `get_time_since_last_concurrent_marking_update()`.
    pub fn get_time_since_last_concurrent_marking_update(&self) -> Duration {
        self.last_concurrently_marked_bytes_update.map_or(Duration::from_secs(0), |t| t.elapsed())
    }

    /// The minimum marked bytes per step. This is a lower bound for all the step
    /// sizes returned from `get_next_incremental_step_duration()`.
    pub fn min_marked_bytes_per_step(&self) -> usize {
        self.min_marked_bytes_per_step
    }

    /// Sets the elapsed time for testing purposes. Is reset after calling
    /// `get_next_incremental_step_duration()`.
    pub fn set_elapsed_time_for_testing(&mut self, elapsed_time: Duration) {
        self.elapsed_time_override = Some(elapsed_time);
    }

    fn get_elapsed_time_since_marking_start(&self) -> Duration {
        self.incremental_marking_start_time.map_or(Duration::from_secs(0), |t| t.elapsed())
    }

    /// Returns the reported overall marked bytes including those marked by the
    /// mutator and concurrently.
    fn get_overall_marked_bytes(&self) -> usize {
        self.mutator_thread_marked_bytes + self.get_concurrently_marked_bytes()
    }

    /// Returns the reported concurrently marked bytes. Only as accurate as
    /// `add_concurrently_marked_bytes()` is.
    fn get_concurrently_marked_bytes(&self) -> usize {
        self.concurrently_marked_bytes.load(Ordering::Relaxed)
    }
}