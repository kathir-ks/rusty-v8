// Copyright 2013 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/base/platform/semaphore.h

use std::time::Duration;

#[cfg(target_os = "macos")]
use dispatch::Semaphore as NativeSemaphore;

#[cfg(target_os = "linux")]
use libc::sem_t as NativeSemaphore;

#[cfg(target_os = "windows")]
use winapi::shared::ntdef::HANDLE as NativeSemaphore;

// TODO: V8_OS_ZOS, V8_OS_STARBOARD implementations

/// A semaphore object is a synchronization object that maintains a count.
/// The count is decremented each time a thread completes a wait for the semaphore
/// object and incremented each time a thread signals the semaphore. When the
/// count reaches zero,  threads waiting for the semaphore blocks until the
/// count becomes non-zero.
pub struct Semaphore {
    native_handle_: NativeSemaphore,
}

impl Semaphore {
    /// Creates a new semaphore with the given initial count.
    pub fn new(count: i32) -> Self {
        #[cfg(target_os = "macos")]
        {
            let handle = dispatch::Semaphore::new(count as isize);
            Semaphore {
                native_handle_: handle,
            }
        }

        #[cfg(target_os = "linux")]
        {
            use libc::{sem_init};
            use std::ptr::null_mut;
            let mut handle: libc::sem_t = unsafe { std::mem::zeroed() };
            let result = unsafe {sem_init(&mut handle, 0, count as u32)};
            if result != 0 {
                panic!("Failed to initialize semaphore: {}", result); //TODO: replace with Result
            }
            Semaphore {
                native_handle_: handle,
            }
        }

        #[cfg(target_os = "windows")]
        {
            use winapi::um::synchapi::CreateSemaphoreA;
            use winapi::shared::minwindef::{LPSECURITY_ATTRIBUTES, FALSE, LONG};
            use std::ptr::null_mut;

            let handle = unsafe { CreateSemaphoreA(null_mut() as LPSECURITY_ATTRIBUTES, count as LONG, i32::MAX as LONG, null_mut()) };
            if handle.is_null() {
                panic!("Failed to create semaphore"); //TODO: replace with Result
            }
            Semaphore {
                native_handle_: handle,
            }
        }

        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
        {
            panic!("Semaphore is not implemented for this platform");
        }
    }

    /// Increments the semaphore counter.
    pub fn signal(&self) {
        #[cfg(target_os = "macos")]
        {
            self.native_handle_.signal();
        }

        #[cfg(target_os = "linux")]
        {
            use libc::{sem_post};
            let result = unsafe {sem_post(&self.native_handle_)};
            if result != 0 {
                panic!("Failed to post to semaphore: {}", result); //TODO: replace with Result
            }
        }

        #[cfg(target_os = "windows")]
        {
            use winapi::um::synchapi::ReleaseSemaphore;
            use winapi::shared::minwindef::{LONG};
            use std::ptr::null_mut;

            let result = unsafe { ReleaseSemaphore(self.native_handle_, 1 as LONG, null_mut()) };
            if result == 0 {
                panic!("Failed to release semaphore"); //TODO: replace with Result
            }
        }

        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
        {
            panic!("Semaphore signal is not implemented for this platform");
        }
    }

    /// Decrements the semaphore counter if it is positive, or blocks until it
    /// becomes positive and then decrements the counter.
    pub fn wait(&self) {
        #[cfg(target_os = "macos")]
        {
            self.native_handle_.wait();
        }

        #[cfg(target_os = "linux")]
        {
            use libc::{sem_wait};
            let result = unsafe {sem_wait(&self.native_handle_)};
            if result != 0 {
                panic!("Failed to wait on semaphore: {}", result); //TODO: replace with Result
            }
        }

        #[cfg(target_os = "windows")]
        {
            use winapi::um::synchapi::WaitForSingleObject;
            use winapi::shared::winobj::INFINITE;
            use winapi::shared::minwindef::DWORD;

            let result = unsafe { WaitForSingleObject(self.native_handle_, INFINITE) };
            if result != 0 {
                 panic!("Failed to wait on semaphore"); //TODO: replace with Result
            }
        }

        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
        {
            panic!("Semaphore wait is not implemented for this platform");
        }
    }

    /// Like wait() but returns after rel_time time has passed. If the timeout
    /// happens the return value is false and the counter is unchanged. Otherwise
    /// the semaphore counter is decremented and true is returned.
    pub fn wait_for(&self, rel_time: Duration) -> bool {
        #[cfg(target_os = "macos")]
        {
            self.native_handle_.wait_timeout(rel_time)
        }

        #[cfg(target_os = "linux")]
        {
            use libc::{sem_timedwait, timespec};
            use std::time::{SystemTime, UNIX_EPOCH};

            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            let tv_sec = now.as_secs() + rel_time.as_secs();
            let tv_nsec = (now.subsec_nanos() + rel_time.subsec_nanos()) as i64;
            let (tv_sec, tv_nsec) = ((tv_sec + (tv_nsec / 1_000_000_000)) as libc::time_t, (tv_nsec % 1_000_000_000) as libc::c_long);

            let abstime = timespec {
                tv_sec: tv_sec,
                tv_nsec: tv_nsec,
            };
            let result = unsafe {sem_timedwait(&self.native_handle_, &abstime)};
            result == 0
        }

        #[cfg(target_os = "windows")]
        {
            use winapi::um::synchapi::WaitForSingleObject;
            use winapi::shared::minwindef::DWORD;

            let result = unsafe { WaitForSingleObject(self.native_handle_, rel_time.as_millis() as DWORD) };
            result == 0
        }

        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
        {
            panic!("Semaphore wait_for is not implemented for this platform");
        }
    }

    pub fn native_handle(&self) -> &NativeSemaphore {
        &self.native_handle_
    }

}

impl Drop for Semaphore {
    fn drop(&mut self) {
        #[cfg(target_os = "linux")]
        {
            use libc::{sem_destroy};
            let result = unsafe {sem_destroy(&mut self.native_handle_)};
            if result != 0 {
                panic!("Failed to destroy semaphore: {}", result); //TODO: replace with Result
            }
        }
        #[cfg(target_os = "windows")]
        {
            use winapi::um::handleapi::CloseHandle;
            unsafe { CloseHandle(self.native_handle_) };
        }
    }
}

// POD Semaphore initialized lazily (i.e. the first time Pointer() is called).
// Usage:
//   // The following semaphore starts at 0.
//   static LazySemaphore<0>::type my_semaphore = LAZY_SEMAPHORE_INITIALIZER;
//
//   void my_function() {
//     // Do something with my_semaphore.Pointer().
//   }
//
// TODO: Implement lazy semaphore

//template <int N>
//struct CreateSemaphoreTrait {
//  static Semaphore* Create() {
//    return new Semaphore(N);
//  }
//};
//
//template <int N>
//struct LazySemaphore {
//  using typename LazyDynamicInstance<Semaphore, CreateSemaphoreTrait<N>,
//                                     ThreadSafeInitOnceTrait>::type;
//};
//
//#define LAZY_SEMAPHORE_INITIALIZER LAZY_DYNAMIC_INSTANCE_INITIALIZER