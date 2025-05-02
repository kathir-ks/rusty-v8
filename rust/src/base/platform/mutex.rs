// Copyright 2013 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::sync::{Mutex as StdMutex, TryLockError};
use std::thread;

#[cfg(debug_assertions)]
use std::sync::atomic::{AtomicUsize, Ordering};

mod platform {
    // Placeholder for platform specific functionality
    #[cfg(target_os = "linux")]
    pub fn get_current_thread_id() -> usize {
        unsafe { libc::syscall(libc::SYS_gettid) as usize }
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_current_thread_id() -> usize {
        // Implement for other platforms if needed
        thread_id::get() as usize
    }
}

mod thread_id {
    use std::thread;
    use std::sync::OnceLock;

    thread_local! {
        static ID: usize = rand::random();
    }

    pub fn get() -> usize {
        *ID.with(|id| id)
    }
}

/// A recursive mutex.
pub struct RecursiveMutex {
    mutex_: StdMutex<()>,
    thread_id_: usize,
    level_: usize,
}

impl RecursiveMutex {
    /// Creates a new recursive mutex.
    pub fn new() -> Self {
        RecursiveMutex {
            mutex_: StdMutex::new(()),
            thread_id_: 0,
            level_: 0,
        }
    }

    /// Locks the mutex.
    pub fn lock(&mut self) {
        let own_id = platform::get_current_thread_id();
        if self.thread_id_ == own_id {
            self.level_ += 1;
            return;
        }
        self.mutex_.lock().unwrap();
        debug_assert_eq!(0, self.level_);
        self.thread_id_ = own_id;
        self.level_ = 1;
    }

    /// Unlocks the mutex.
    pub fn unlock(&mut self) {
        #[cfg(debug_assertions)]
        {
            let own_id = platform::get_current_thread_id();
            assert_eq!(self.thread_id_, own_id);
        }
        self.level_ -= 1;
        if self.level_ == 0 {
            self.thread_id_ = 0;
            self.mutex_.unlock().unwrap();
        }
    }

    /// Tries to lock the mutex.
    pub fn try_lock(&mut self) -> bool {
        let own_id = platform::get_current_thread_id();
        if self.thread_id_ == own_id {
            self.level_ += 1;
            return true;
        }
        match self.mutex_.try_lock() {
            Ok(_guard) => {
                debug_assert_eq!(0, self.level_);
                self.thread_id_ = own_id;
                self.level_ = 1;
                true
            }
            Err(_try_lock_error) => false,
        }
    }
}

impl Drop for RecursiveMutex {
    fn drop(&mut self) {
        debug_assert_eq!(0, self.level_);
    }
}


/// A basic mutex.
pub struct Mutex {
    native_handle_: StdMutex<()>,
    #[cfg(debug_assertions)]
    level_: AtomicUsize,
}

impl Mutex {
    /// Creates a new mutex.
    pub fn new() -> Self {
        Mutex {
            native_handle_: StdMutex::new(()),
            #[cfg(debug_assertions)]
            level_: AtomicUsize::new(0),
        }
    }

    /// Locks the mutex.
    pub fn lock(&self) {
        self.native_handle_.lock().unwrap();
        self.assert_unheld_and_mark();
    }

    /// Unlocks the mutex.
    pub fn unlock(&self) {
        self.assert_held_and_unmark();
        self.native_handle_.unlock().unwrap();
    }

    /// Tries to lock the mutex.
    pub fn try_lock(&self) -> bool {
        match self.native_handle_.try_lock() {
            Ok(_guard) => {
                self.assert_unheld_and_mark();
                true
            }
            Err(_try_lock_error) => false,
        }
    }

    #[cfg(debug_assertions)]
    fn assert_held_and_unmark(&self) {
        assert_eq!(self.level_.load(Ordering::Relaxed), 1);
        self.level_.store(0, Ordering::Relaxed);
    }

    #[cfg(debug_assertions)]
    fn assert_unheld_and_mark(&self) {
        assert_eq!(self.level_.load(Ordering::Relaxed), 0);
        self.level_.store(1, Ordering::Relaxed);
    }

    #[cfg(not(debug_assertions))]
    fn assert_held_and_unmark(&self) {}

    #[cfg(not(debug_assertions))]
    fn assert_unheld_and_mark(&self) {}
}

impl Drop for Mutex {
    fn drop(&mut self) {
        #[cfg(debug_assertions)]
        debug_assert_eq!(self.level_.load(Ordering::Relaxed), 0);
    }
}