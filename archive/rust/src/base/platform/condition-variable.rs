// Copyright 2013 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::time::Duration;
use std::sync::{Condvar, Mutex, MutexGuard};

/// Represents a time duration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeDelta {
    nanos: u64,
}

impl TimeDelta {
    /// Creates a new `TimeDelta` from nanoseconds.
    pub fn from_nanos(nanos: u64) -> Self {
        TimeDelta { nanos }
    }

    /// Returns the duration as nanoseconds.
    pub fn in_nanoseconds(&self) -> u64 {
        self.nanos
    }
}

/// A condition variable that can be used to synchronize threads.
pub struct ConditionVariable {
    native_handle: Condvar,
}

impl ConditionVariable {
    /// Creates a new `ConditionVariable`.
    pub fn new() -> Self {
        ConditionVariable {
            native_handle: Condvar::new(),
        }
    }

    /// Wakes up one thread that is waiting on this condition variable.
    pub fn notify_one(&self) {
        self.native_handle.notify_one();
    }

    /// Wakes up all threads that are waiting on this condition variable.
    pub fn notify_all(&self) {
        self.native_handle.notify_all();
    }

    /// Waits for a notification on this condition variable.  The mutex must be locked
    /// when calling this function, and it will be unlocked while waiting.
    pub fn wait<'a>(&self, mutex: MutexGuard<'a, ()>) -> MutexGuard<'a, ()> {
        self.native_handle.wait(mutex).unwrap()
    }

    /// Waits for a notification on this condition variable, with a timeout. The mutex must be locked
    /// when calling this function, and it will be unlocked while waiting.
    ///
    /// Returns `true` if the wait was successful (i.e., the condition variable was notified
    /// before the timeout expired), and `false` if the timeout expired.
    pub fn wait_for<'a>(&self, mutex: MutexGuard<'a, ()>, rel_time: TimeDelta) -> Result<MutexGuard<'a, ()>, ()> {
        let duration = Duration::from_nanos(rel_time.in_nanoseconds());
        match self.native_handle.wait_timeout(mutex, duration) {
            Ok((mutex_guard, timeout_result)) => {
                if timeout_result.timed_out() {
                    Err(())
                } else {
                    Ok(mutex_guard)
                }
            },
            Err(_) => {
                // Handle potential errors (e.g., poisoned mutex)
                Err(())
            }
        }
    }
}

impl Default for ConditionVariable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc};
    use std::thread;

    #[test]
    fn test_condition_variable() {
        let mutex = Arc::new(Mutex::new(()));
        let condvar = Arc::new(ConditionVariable::new());

        let mutex_clone = Arc::clone(&mutex);
        let condvar_clone = Arc::clone(&condvar);

        let handle = thread::spawn(move || {
            let mut guard = mutex_clone.lock().unwrap();
            let result = condvar_clone.wait_for(guard, TimeDelta::from_nanos(100_000_000)); // 100ms timeout
            match result {
                Ok(_) => {
                    // Successfully waited
                    assert!(true);
                },
                Err(_) => {
                    // Timed out
                    assert!(false); // Should not timeout in this test
                }
            }
        });

        // Give the spawned thread some time to start waiting
        thread::sleep(Duration::from_millis(50));

        // Notify the waiting thread
        condvar.notify_one();

        handle.join().unwrap();
    }

    #[test]
    fn test_condition_variable_timeout() {
        let mutex = Arc::new(Mutex::new(()));
        let condvar = Arc::new(ConditionVariable::new());

        let mutex_clone = Arc::clone(&mutex);
        let condvar_clone = Arc::clone(&condvar);

        let handle = thread::spawn(move || {
            let guard = mutex_clone.lock().unwrap();
            let result = condvar_clone.wait_for(guard, TimeDelta::from_nanos(10_000_000)); // 10ms timeout
            match result {
                Ok(_) => {
                    // Successfully waited
                    assert!(false); // Should timeout in this test
                },
                Err(_) => {
                    // Timed out
                    assert!(true);
                }
            }
        });

        handle.join().unwrap();
    }
}