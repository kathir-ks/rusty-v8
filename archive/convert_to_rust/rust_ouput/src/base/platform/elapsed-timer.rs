// Converted from V8 C++ source files:
// Header: elapsed-timer.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod platform {
        use std::time::{Duration, Instant};

        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
        pub struct TimeTicks {
            instant: Option<Instant>,
        }

        impl TimeTicks {
            pub fn now() -> Self {
                TimeTicks {
                    instant: Some(Instant::now()),
                }
            }

            pub fn is_null(&self) -> bool {
                self.instant.is_none()
            }

            pub fn null() -> Self {
                TimeTicks { instant: None }
            }
        }

        impl std::ops::Sub for TimeTicks {
            type Output = TimeDelta;

            fn sub(self, other: Self) -> Self::Output {
                match (self.instant, other.instant) {
                    (Some(self_instant), Some(other_instant)) => {
                        if self_instant >= other_instant {
                            TimeDelta {
                                duration: self_instant.duration_since(other_instant),
                            }
                        } else {
                             TimeDelta {
                                duration: other_instant.duration_since(self_instant),
                            }
                        }
                    }
                    _ => TimeDelta {
                        duration: Duration::from_secs(0),
                    },
                }
            }
        }

        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
        pub struct TimeDelta {
            duration: Duration,
        }

        impl TimeDelta {
            pub fn in_microseconds(&self) -> i64 {
                self.duration.as_micros() as i64
            }

            pub fn as_duration(&self) -> Duration {
                self.duration
            }
        }
    }

    use self::platform::{TimeDelta, TimeTicks};

    #[derive(Debug)]
    pub struct ElapsedTimer {
        start_ticks_: TimeTicks,
        paused_elapsed_: TimeDelta,
        started_: bool,
        paused_: bool,
    }

    impl ElapsedTimer {
        pub fn new() -> Self {
            ElapsedTimer {
                start_ticks_: TimeTicks::null(),
                paused_elapsed_: TimeDelta {
                    duration: Duration::from_secs(0),
                },
                started_: false,
                paused_: false,
            }
        }

        pub fn start(&mut self) {
            self.start_with_time(TimeTicks::now());
        }

        pub fn start_with_time(&mut self, now: TimeTicks) {
            assert!(!now.is_null());
            assert!(!self.is_started());
            self.set_start_ticks(now);
            self.started_ = true;
            assert!(self.is_started());
        }

        pub fn stop(&mut self) {
            assert!(self.is_started());
            self.set_start_ticks(TimeTicks::null());
            self.started_ = false;
            assert!(!self.is_started());
        }

        pub fn is_started(&self) -> bool {
            assert!(!self.paused_);
            assert_eq!(self.started_, !self.start_ticks_.is_null());
            !self.start_ticks_.is_null()
        }

        #[cfg(debug_assertions)]
        pub fn is_paused(&self) -> bool {
            self.paused_
        }

        pub fn restart(&mut self) -> TimeDelta {
            self.restart_with_time(TimeTicks::now())
        }

        pub fn restart_with_time(&mut self, now: TimeTicks) -> TimeDelta {
            assert!(!now.is_null());
            assert!(self.is_started());
            let elapsed = now - self.start_ticks_;
            assert!(elapsed.in_microseconds() >= 0);
            self.set_start_ticks(now);
            assert!(self.is_started());
            elapsed
        }

        pub fn pause(&mut self) {
            self.pause_with_time(TimeTicks::now());
        }

        pub fn pause_with_time(&mut self, now: TimeTicks) {
            let elapsed = self.elapsed_with_time(now);
            assert!(self.is_started());
            self.paused_ = true;
            self.set_paused_elapsed(elapsed);
        }

        pub fn resume(&mut self) {
            self.resume_with_time(TimeTicks::now());
        }

        pub fn resume_with_time(&mut self, now: TimeTicks) {
            assert!(!now.is_null());
            assert!(self.started_);
            assert!(self.paused_);
            let elapsed = self.paused_elapsed();
            self.paused_ = false;
            self.set_start_ticks(now - elapsed);
            assert!(self.is_started());
        }

        pub fn elapsed(&self) -> TimeDelta {
            self.elapsed_with_time(TimeTicks::now())
        }

        pub fn elapsed_with_time(&self, now: TimeTicks) -> TimeDelta {
            assert!(!now.is_null());
            assert!(self.is_started());
            let elapsed = now - self.start_ticks_;
            assert!(elapsed.in_microseconds() >= 0);
            elapsed
        }

        pub fn has_expired(&self, time_delta: TimeDelta) -> bool {
            assert!(self.is_started());
            self.elapsed() >= time_delta
        }

        fn paused_elapsed(&self) -> TimeDelta {
            assert!(self.paused_);
            assert!(self.started_);
            self.paused_elapsed_
        }

        fn set_paused_elapsed(&mut self, delta: TimeDelta) {
            assert!(self.paused_);
            assert!(self.started_);
            self.paused_elapsed_ = delta;
        }

        fn start_ticks(&self) -> TimeTicks {
            assert!(!self.paused_);
            self.start_ticks_
        }

        fn set_start_ticks(&mut self, start_ticks: TimeTicks) {
            assert!(!self.paused_);
            self.start_ticks_ = start_ticks;
        }
    }

    pub struct ScopedTimer<'a> {
        timer_: ElapsedTimer,
        location_: Option<&'a mut TimeDelta>,
    }

    impl<'a> ScopedTimer<'a> {
        pub fn new(location: Option<&'a mut TimeDelta>) -> Self {
            let mut timer = ElapsedTimer::new();
            if location.is_some() {
                timer.start();
            }
            ScopedTimer {
                timer_: timer,
                location_: location,
            }
        }
    }

    impl<'a> Drop for ScopedTimer<'a> {
        fn drop(&mut self) {
            if let Some(location) = self.location_ {
                *location = TimeDelta {duration: location.duration.clone() + self.timer_.elapsed().duration};
            }
        }
    }
}
