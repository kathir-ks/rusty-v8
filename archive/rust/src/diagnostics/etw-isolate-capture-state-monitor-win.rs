// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod etw_jit_interface {
    use std::sync::{Arc, Condvar, Mutex, MutexGuard};
    use std::time::{Duration, Instant};

    /// A monitor for tracking the capture state of isolates in ETW.
    pub struct EtwIsolateCaptureStateMonitor {
        mutex: Arc<Mutex<()>>,
        pending_isolate_count: Mutex<usize>,
        isolates_ready_cv: Condvar,
        wait_started: Mutex<Option<Instant>>,
    }

    impl EtwIsolateCaptureStateMonitor {
        /// Creates a new `EtwIsolateCaptureStateMonitor`.
        ///
        /// # Arguments
        ///
        /// * `mutex`: A mutex to protect access to the monitor's state.
        /// * `pending_isolate_count`: The initial number of isolates pending capture.
        pub fn new(mutex: Arc<Mutex<()>>, pending_isolate_count: usize) -> Self {
            EtwIsolateCaptureStateMonitor {
                mutex,
                pending_isolate_count: Mutex::new(pending_isolate_count),
                isolates_ready_cv: Condvar::new(),
                wait_started: Mutex::new(None),
            }
        }

        /// Waits for the specified duration or until `notify` is called `pending_isolate_count` times.
        ///
        /// # Arguments
        ///
        /// * `delta`: The maximum duration to wait.
        pub fn wait(&self, delta: Duration) -> bool {
            let lock = self.mutex.lock().unwrap();
            let mut wait_started = self.wait_started.lock().unwrap();
            *wait_started = Some(Instant::now());

            let mut pending_count = self.pending_isolate_count.lock().unwrap();
            while *pending_count > 0 {
                let start_time = wait_started.unwrap();
                let elapsed = start_time.elapsed();

                if elapsed >= delta {
                    return false;
                }

                let remaining = delta - elapsed;

                let (mut lock, timeout_result) = self
                    .isolates_ready_cv
                    .wait_timeout(lock, remaining)
                    .unwrap();
                
                if *pending_count == 0{
                    return true;
                }

                if timeout_result.timed_out(){
                    return false;
                }
            }

            true
        }

        /// Notifies the monitor that an isolate has been captured.
        pub fn notify(&self) {
            let lock = self.mutex.lock().unwrap();
            let mut pending_count = self.pending_isolate_count.lock().unwrap();
            if *pending_count > 0 {
                *pending_count -= 1;
                if *pending_count == 0 {
                    self.isolates_ready_cv.notify_one();
                }
            }
        }
    }
}