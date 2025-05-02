// Copyright 2013 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeTicks(Instant);

impl TimeTicks {
    pub fn now() -> Self {
        TimeTicks(Instant::now())
    }

    pub fn is_null(&self) -> bool {
        self.0 == Instant::now() - Duration::from_secs(u64::MAX) // Approximation of a null Instant
    }

    pub fn from_duration_since_epoch(duration: Duration) -> Self {
        TimeTicks(Instant::now() - duration) // Approximation, assumes the epoch is in the past
    }

    pub fn duration_since(&self, earlier: TimeTicks) -> Duration {
        self.0.duration_since(earlier.0)
    }

    // Add other necessary methods from the original TimeTicks class.
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeDelta(Duration);

impl TimeDelta {
    pub fn from_microseconds(microseconds: i64) -> Self {
        if microseconds >= 0 {
            TimeDelta(Duration::from_micros(microseconds as u64))
        } else {
            TimeDelta(Duration::from_micros(0)) // Handle negative values appropriately
        }
    }

    pub fn in_microseconds(&self) -> i64 {
        self.0.as_micros() as i64
    }
}

impl std::ops::Sub for TimeTicks {
    type Output = TimeDelta;

    fn sub(self, other: Self) -> Self::Output {
        TimeDelta(self.0.duration_since(other.0))
    }
}

impl std::ops::Sub<TimeDelta> for TimeTicks {
    type Output = TimeTicks;

    fn sub(self, other: TimeDelta) -> Self::Output {
        TimeTicks(self.0 - other.0)
    }
}

impl std::ops::Add<TimeDelta> for TimeTicks {
    type Output = TimeTicks;

    fn add(self, other: TimeDelta) -> Self::Output {
        TimeTicks(self.0 + other.0)
    }
}

impl std::ops::AddAssign<TimeDelta> for TimeDelta {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl std::cmp::PartialOrd<TimeDelta> for TimeDelta {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl std::cmp::PartialEq<TimeDelta> for TimeDelta {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

/// A timer that measures elapsed time.
#[derive(Debug)]
pub struct ElapsedTimer {
    start_ticks: TimeTicks,
    paused_elapsed: TimeDelta,
    started: bool,
    paused: bool,
}

impl ElapsedTimer {
    /// Creates a new `ElapsedTimer`.
    pub fn new() -> Self {
        ElapsedTimer {
            start_ticks: TimeTicks::from_duration_since_epoch(Duration::from_secs(0)), //Initialize to a null TimeTicks
            paused_elapsed: TimeDelta::from_microseconds(0),
            started: false,
            paused: false,
        }
    }

    /// Starts this timer.
    ///
    /// Once started, a timer can be checked with [`elapsed()`] or [`has_expired()`],
    /// and may be restarted using [`restart()`].
    ///
    /// # Panics
    ///
    /// This method must not be called on an already started timer.
    pub fn start(&mut self) {
        self.start_with_time(TimeTicks::now());
    }

    fn start_with_time(&mut self, now: TimeTicks) {
        assert!(!now.is_null());
        assert!(!self.is_started());
        self.set_start_ticks(now);
        self.started = true;
        assert!(self.is_started());
    }

    /// Stops this timer.
    ///
    /// # Panics
    ///
    /// This method must not be called on a timer that was not started before.
    pub fn stop(&mut self) {
        assert!(self.is_started());
        self.set_start_ticks(TimeTicks::from_duration_since_epoch(Duration::from_secs(0))); //Reset to null TimeTicks
        self.started = false;
        assert!(!self.is_started());
    }

    /// Returns `true` if this timer was started previously.
    pub fn is_started(&self) -> bool {
        assert!(!self.paused);
        assert_eq!(self.started, !self.start_ticks.is_null());
        !self.start_ticks.is_null()
    }

    #[cfg(debug_assertions)]
    /// Returns `true` if this timer is paused.
    pub fn is_paused(&self) -> bool {
        self.paused
    }

    /// Restarts the timer and returns the time elapsed since the previous start.
    ///
    /// This method is equivalent to obtaining the elapsed time with [`elapsed()`]
    /// and then starting the timer again, but does so in one single operation,
    /// avoiding the need to obtain the clock value twice. It may only be called
    /// on a previously started timer.
    ///
    /// # Panics
    ///
    /// This method panics if the timer was not started before.
    pub fn restart(&mut self) -> TimeDelta {
        self.restart_with_time(TimeTicks::now())
    }

    fn restart_with_time(&mut self, now: TimeTicks) -> TimeDelta {
        assert!(!now.is_null());
        assert!(self.is_started());
        let elapsed = self.elapsed_with_time(now);
        self.set_start_ticks(now);
        assert!(elapsed.in_microseconds() >= 0);
        assert!(self.is_started());
        elapsed
    }

    /// Pauses the timer.
    pub fn pause(&mut self) {
        self.pause_with_time(TimeTicks::now());
    }

    fn pause_with_time(&mut self, now: TimeTicks) {
        let elapsed = self.elapsed_with_time(now);
        assert!(self.is_started());
        self.paused = true;
        self.set_paused_elapsed(elapsed);
    }

    /// Resumes the timer.
    pub fn resume(&mut self) {
        self.resume_with_time(TimeTicks::now());
    }

    fn resume_with_time(&mut self, now: TimeTicks) {
        assert!(!now.is_null());
        assert!(self.started);
        assert!(self.paused);
        let elapsed = self.paused_elapsed();
        self.paused = false;
        self.set_start_ticks(now - elapsed);
        assert!(self.is_started());
    }

    /// Returns the time elapsed since the previous start.
    ///
    /// # Panics
    ///
    /// This method panics if the timer was not started before.
    pub fn elapsed(&self) -> TimeDelta {
        self.elapsed_with_time(TimeTicks::now())
    }

    fn elapsed_with_time(&self, now: TimeTicks) -> TimeDelta {
        assert!(!now.is_null());
        assert!(self.is_started());
        let elapsed = now - self.start_ticks;
        assert!(elapsed.in_microseconds() >= 0);
        elapsed
    }

    /// Returns `true` if the specified `time_delta` has elapsed since the
    /// previous start, or `false` if not.
    ///
    /// # Panics
    ///
    /// This method panics if the timer was not started before.
    pub fn has_expired(&self, time_delta: TimeDelta) -> bool {
        assert!(self.is_started());
        self.elapsed() >= time_delta
    }

    fn paused_elapsed(&self) -> TimeDelta {
        assert!(self.paused);
        assert!(self.started);
        self.paused_elapsed
    }

    fn set_paused_elapsed(&mut self, delta: TimeDelta) {
        assert!(self.paused);
        assert!(self.started);
        self.paused_elapsed = delta;
    }

    fn start_ticks(&self) -> TimeTicks {
        assert!(!self.paused);
        self.start_ticks
    }

    fn set_start_ticks(&mut self, start_ticks: TimeTicks) {
        assert!(!self.paused);
        self.start_ticks = start_ticks;
    }
}

impl Default for ElapsedTimer {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper that times a scoped region and records the elapsed time.
pub struct ScopedTimer<'a> {
    timer: ElapsedTimer,
    location: Option<&'a mut TimeDelta>,
}

impl<'a> ScopedTimer<'a> {
    /// Creates a new `ScopedTimer`.
    pub fn new(location: Option<&'a mut TimeDelta>) -> Self {
        let mut scoped_timer = ScopedTimer {
            timer: ElapsedTimer::new(),
            location,
        };
        if let Some(_loc) = scoped_timer.location {
            scoped_timer.timer.start();
        }
        scoped_timer
    }
}

impl<'a> Drop for ScopedTimer<'a> {
    fn drop(&mut self) {
        if let Some(location) = self.location {
            *location += self.timer.elapsed();
        }
    }
}