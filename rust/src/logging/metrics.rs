// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::any::Any;
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

pub mod v8 {
    pub mod metrics {
        pub type Recorder = super::metrics::Recorder;
        pub type ContextId = u32; // Example type, replace with the actual type
    }
    pub trait TaskRunner: Send + Sync {
        fn post_task(&self, task: Box<dyn FnOnce()>);
    }
}

pub mod base {
    pub mod platform {
        use std::time::{Instant, SystemTime, UNIX_EPOCH};
        pub use std::time::Duration;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub struct TimeTicks {
            instant: Option<Instant>,
        }

        impl TimeTicks {
            pub fn now() -> Self {
                TimeTicks { instant: Some(Instant::now()) }
            }

            pub fn min() -> Self {
                TimeTicks { instant: None }
            }

            pub fn is_min(&self) -> bool {
                self.instant.is_none()
            }

            pub fn duration_since(&self, earlier: TimeTicks) -> TimeDelta {
                match (self.instant, earlier.instant) {
                    (Some(later_instant), Some(earlier_instant)) => {
                        TimeDelta { duration: later_instant.duration_since(earlier_instant) }
                    }
                    _ => TimeDelta { duration: Duration::from_secs(0) },
                }
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub struct TimeDelta {
            duration: Duration,
        }

        impl TimeDelta {
            pub fn in_microseconds(&self) -> i64 {
                self.duration.as_micros() as i64
            }
        }

        pub struct Mutex {
            inner: std::sync::Mutex<()>,
        }

        impl Mutex {
            pub fn new() -> Self {
                Mutex { inner: std::sync::Mutex::new(()) }
            }

            pub fn lock(&self) -> MutexGuard<()> {
                self.inner.lock().unwrap()
            }
        }
    }
}

pub mod init {
    pub mod v8 {
        pub fn initialize() {} //Placeholder
    }
}

pub mod internal {
    pub mod metrics {
        use std::any::Any;
        use std::sync::{Arc, Mutex, MutexGuard};
        use crate::base::platform;
        use crate::v8;

        pub struct Recorder {
            lock: Mutex<()>,
            foreground_task_runner: Option<Arc<dyn v8::TaskRunner>>,
            embedder_recorder: Option<Arc<Recorder>>,
            delayed_events: Mutex<VecDeque<Box<dyn DelayedEventBase>>>,
        }

        impl Recorder {
            pub fn new() -> Self {
                Recorder {
                    lock: Mutex::new(()),
                    foreground_task_runner: None,
                    embedder_recorder: None,
                    delayed_events: Mutex::new(VecDeque::new()),
                }
            }

            pub fn set_embedder_recorder(
                &mut self,
                isolate: *mut std::ffi::c_void, // Replace with actual isolate type
                embedder_recorder: Arc<v8::metrics::Recorder>,
            ) {
                // TODO: Use isolate to store the embedder recorder.
                let _ = isolate;
                self.embedder_recorder = Some(embedder_recorder);
            }

            pub fn has_embedder_recorder(&self) -> bool {
                self.embedder_recorder.is_some()
            }

            pub fn notify_isolate_disposal(&self) {
                // Placeholder implementation
                //println!("Isolate disposal notification received.");
            }

            pub fn add_main_thread_event<T: Any + Send + Sync>(
                &self,
                event: &T,
                id: v8::metrics::ContextId,
            ) {
                if let Some(embedder_recorder) = &self.embedder_recorder {
                    embedder_recorder.add_main_thread_event(event, id);
                }
            }

            pub fn delay_main_thread_event<T: Any + Send + Sync>(
                &self,
                event: T,
                id: v8::metrics::ContextId,
            ) {
                if self.embedder_recorder.is_none() {
                    return;
                }

                let delayed_event = DelayedEvent::new(event, id);
                self.delay(Box::new(delayed_event));
            }

            pub fn add_thread_safe_event<T: Any + Send + Sync>(&self, event: &T) {
                if let Some(embedder_recorder) = &self.embedder_recorder {
                    embedder_recorder.add_thread_safe_event(event);
                }
            }

            fn delay(&self, event: Box<dyn DelayedEventBase>) {
                let mut delayed_events = self.delayed_events.lock().unwrap();
                delayed_events.push_back(event);
            }
        }

        trait DelayedEventBase: Send {
            fn run(&self, recorder: &Arc<Recorder>);
        }

        struct DelayedEvent<T: Any + Send + Sync> {
            event: T,
            id: v8::metrics::ContextId,
        }

        impl<T: Any + Send + Sync> DelayedEvent<T> {
            fn new(event: T, id: v8::metrics::ContextId) -> Self {
                DelayedEvent { event, id }
            }
        }

        impl<T: Any + Send + Sync> DelayedEventBase for DelayedEvent<T> {
            fn run(&self, recorder: &Arc<Recorder>) {
                recorder.add_main_thread_event(&self.event, self.id);
            }
        }

        pub struct TimedScope<'a, T, F>
            where
                F: Fn(&platform::TimeDelta) -> i64,
        {
            event: &'a mut T,
            start_time: platform::TimeTicks,
            precision: F,
            _phantom: PhantomData<&'a T>,
        }

        impl<'a, T, F> TimedScope<'a, T, F>
            where
                F: Fn(&platform::TimeDelta) -> i64,
        {
            pub fn new(event: &'a mut T, precision: F) -> Self {
                let mut timed_scope = TimedScope {
                    event,
                    start_time: platform::TimeTicks::min(),
                    precision,
                    _phantom: PhantomData,
                };
                timed_scope.start();
                timed_scope
            }

            fn start(&mut self) {
                self.start_time = platform::TimeTicks::now();
            }

            fn stop(&mut self) {
                if self.start_time.is_min() {
                    return;
                }
                let duration = platform::TimeTicks::now().duration_since(self.start_time);
                self.event.wall_clock_duration_in_us = (self.precision)(&duration);
                self.start_time = platform::TimeTicks::min();
            }
        }

        impl<'a, T, F> Drop for TimedScope<'a, T, F>
            where
                F: Fn(&platform::TimeDelta) -> i64,
        {
            fn drop(&mut self) {
                self.stop();
            }
        }
    }
}