// Converted from V8 C++ source files:
// Header: semaphore.h
// Implementation: semaphore.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
pub mod platform {

use std::time::Duration;
use std::result;
use std::sync::{Arc, Mutex, Condvar};
use std::sync::atomic::{AtomicI32, Ordering};

#[cfg(target_os = "windows")]
use winapi::um::synchapi::{CreateSemaphoreA, ReleaseSemaphore, WaitForSingleObject};
#[cfg(target_os = "windows")]
use winapi::shared::minwindef::{DWORD, LONG};
#[cfg(target_os = "windows")]
use winapi::shared::ntdef::HANDLE;
#[cfg(target_os = "windows")]
use winapi::um::errhandlingapi::GetLastError;
#[cfg(target_os = "windows")]
use winapi::um::winbase::{WAIT_OBJECT_0, WAIT_TIMEOUT};
#[cfg(target_os = "windows")]
use winapi::um::timezoneapi::GetTimeZoneInformation;
#[cfg(target_os = "windows")]
use winapi::shared::basetsd::LONG_PTR;

#[cfg(target_os = "macos")]
use core_foundation::base::TCFType;
#[cfg(target_os = "macos")]
use core_foundation::string::CFString;

#[cfg(target_os = "linux")]
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(target_os = "starboard")]
use starboard::common::semaphore::Semaphore as StarboardSemaphore;

pub struct TimeDelta {
    nanoseconds: i64,
}

impl TimeDelta {
    pub fn from_nanoseconds(nanoseconds: i64) -> Self {
        TimeDelta { nanoseconds }
    }

    pub fn in_nanoseconds(&self) -> i64 {
        self.nanoseconds
    }

    pub fn in_microseconds(&self) -> i64 {
        self.nanoseconds / 1000
    }

    pub fn in_milliseconds(&self) -> i64 {
        self.nanoseconds / 1_000_000
    }

    pub fn in_seconds(&self) -> f64 {
        self.nanoseconds as f64 / 1_000_000_000.0
    }
}

#[derive(Debug)]
pub enum SemaphoreError {
    CreationError,
    WaitError,
    TimedOut,
}

pub struct Semaphore {
    #[cfg(target_os = "macos")]
    native_handle_: dispatch::Semaphore,
    #[cfg(target_os = "linux")]
    native_handle_: Arc<(Mutex<i32>, Condvar)>,
    #[cfg(target_os = "windows")]
    native_handle_: HANDLE,
    #[cfg(target_os = "starboard")]
    native_handle_: StarboardSemaphore,
    count: AtomicI32,
    max: i32,
}

impl Semaphore {
    pub fn new(count: i32) -> Self {
        #[cfg(target_os = "macos")]
        {
            let handle = dispatch::Semaphore::new(count);
            Semaphore {
                native_handle_: handle,
                count: AtomicI32::new(count),
                max: i32::MAX,
            }
        }
        #[cfg(target_os = "linux")]
        {
            Semaphore {
                native_handle_: Arc::new((Mutex::new(count), Condvar::new())),
                count: AtomicI32::new(count),
                max: i32::MAX,
            }
        }
        #[cfg(target_os = "windows")]
        {
            unsafe {
                let handle = CreateSemaphoreA(
                    std::ptr::null_mut(),
                    count,
                    i32::MAX,
                    std::ptr::null_mut(),
                );
                if handle.is_null() {
                    panic!("Failed to create semaphore: {:?}", GetLastError());
                }
                Semaphore {
                    native_handle_: handle,
                    count: AtomicI32::new(count),
                    max: i32::MAX,
                }
            }
        }
        #[cfg(target_os = "starboard")]
        {
            Semaphore {
                native_handle_: StarboardSemaphore::new(count),
                count: AtomicI32::new(count),
                max: i32::MAX,
            }
        }
    }

    pub fn signal(&self) {
        #[cfg(target_os = "macos")]
        {
            self.native_handle_.signal();
            self.count.fetch_add(1, Ordering::Relaxed);
        }
        #[cfg(target_os = "linux")]
        {
            let (lock, cvar) = &*self.native_handle_;
            let mut count = lock.lock().unwrap();
            *count += 1;
            cvar.notify_one();
            self.count.fetch_add(1, Ordering::Relaxed);
        }
        #[cfg(target_os = "windows")]
        {
            unsafe {
                ReleaseSemaphore(self.native_handle_, 1, std::ptr::null_mut());
                self.count.fetch_add(1, Ordering::Relaxed);
            }
        }
        #[cfg(target_os = "starboard")]
        {
            self.native_handle_.put();
            self.count.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn wait(&self) {
        #[cfg(target_os = "macos")]
        {
            self.native_handle_.wait();
            self.count.fetch_sub(1, Ordering::Relaxed);
        }
        #[cfg(target_os = "linux")]
        {
            let (lock, cvar) = &*self.native_handle_;
            let mut count = lock.lock().unwrap();
            while *count <= 0 {
                count = cvar.wait(count).unwrap();
            }
            *count -= 1;
            self.count.fetch_sub(1, Ordering::Relaxed);
        }
        #[cfg(target_os = "windows")]
        {
            unsafe {
                let result = WaitForSingleObject(self.native_handle_, winapi::um::winbase::INFINITE);
                assert_eq!(result, WAIT_OBJECT_0);
                self.count.fetch_sub(1, Ordering::Relaxed);
            }
        }
        #[cfg(target_os = "starboard")]
        {
            self.native_handle_.take();
            self.count.fetch_sub(1, Ordering::Relaxed);
        }
    }

    pub fn wait_for(&self, rel_time: &TimeDelta) -> bool {
        #[cfg(target_os = "macos")]
        {
            let timeout = dispatch::DispatchTime::now() + dispatch::TimeDelta::from_nanoseconds(rel_time.in_nanoseconds());
            let result = self.native_handle_.wait_timeout(timeout);
            if result.is_ok() {
                self.count.fetch_sub(1, Ordering::Relaxed);
                return true;
            }
            return false;
        }
        #[cfg(target_os = "linux")]
        {
            let (lock, cvar) = &*self.native_handle_;
            let mut count = lock.lock().unwrap();
            let timeout = Duration::from_nanos(rel_time.in_nanoseconds() as u64);
            let result = cvar.wait_timeout(count, timeout).unwrap();
            if result.1.timed_out() {
                return false;
            } else {
                *result.0 -= 1;
                self.count.fetch_sub(1, Ordering::Relaxed);
                return true;
            }
        }
        #[cfg(target_os = "windows")]
        {
            unsafe {
                let msec = rel_time.in_milliseconds();
                let result = WaitForSingleObject(
                    self.native_handle_,
                    if msec >= 0 {
                        msec as DWORD
                    } else {
                        0
                    },
                );
                if result == WAIT_OBJECT_0 {
                    self.count.fetch_sub(1, Ordering::Relaxed);
                    return true;
                } else {
                    return false;
                }
            }
        }
        #[cfg(target_os = "starboard")]
        {
            let microseconds = rel_time.in_microseconds();
            let result = self.native_handle_.take_wait(microseconds);
            if result {
                self.count.fetch_sub(1, Ordering::Relaxed);
            }
            return result;
        }
    }

    #[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
    pub fn native_handle(&self) -> &HANDLE {
        &self.native_handle_
    }

    #[cfg(target_os = "starboard")]
    pub fn native_handle(&self) -> &StarboardSemaphore {
        &self.native_handle_
    }
}

impl Drop for Semaphore {
    fn drop(&mut self) {
        #[cfg(target_os = "windows")]
        unsafe {
            CloseHandle(self.native_handle_);
        }
    }
}

#[cfg(target_os = "macos")]
mod dispatch {
    use core_foundation::base::TCFType;
    use dispatch::*;
    use std::time::Duration;

    pub struct Semaphore(dispatch_semaphore_t);

    impl Semaphore {
        pub fn new(count: i32) -> Self {
            unsafe { Semaphore(dispatch_semaphore_create(count as i64)) }
        }

        pub fn signal(&self) {
            unsafe { dispatch_semaphore_signal(self.0) };
        }

        pub fn wait(&self) {
            unsafe { dispatch_semaphore_wait(self.0, DISPATCH_TIME_FOREVER) };
        }

        pub fn wait_timeout(&self, timeout: DispatchTime) -> Result<(), ()> {
            unsafe {
                if dispatch_semaphore_wait(self.0, timeout.0) == 0 {
                    Ok(())
                } else {
                    Err(())
                }
            }
        }
    }

    impl Drop for Semaphore {
        fn drop(&mut self) {
            unsafe { dispatch_release(self.0 as dispatch_object_t) }
        }
    }

    #[derive(Clone, Copy)]
    pub struct DispatchTime(dispatch_time_t);

    impl DispatchTime {
        pub fn now() -> Self {
            DispatchTime(DISPATCH_TIME_NOW)
        }
    }

    pub struct TimeDelta {
        nanoseconds: i64,
    }

    impl TimeDelta {
        pub fn from_nanoseconds(nanoseconds: i64) -> Self {
            TimeDelta { nanoseconds }
        }
    }

    impl std::ops::Add<TimeDelta> for DispatchTime {
        type Output = Self;

        fn add(self, delta: TimeDelta) -> Self {
            unsafe { DispatchTime(dispatch_time(DISPATCH_TIME_NOW, delta.nanoseconds)) }
        }
    }
}

pub struct TimeTicks {}

impl TimeTicks {
    #[cfg(target_os = "windows")]
    pub fn now() -> Self {
        TimeTicks {}
    }
}

pub struct Time {}

impl Time {
    #[cfg(target_os = "linux")]
    pub fn now_from_system_time() -> Self {
        Time {}
    }

    #[cfg(target_os = "linux")]
    pub fn to_timespec(&self) -> timespec {
        timespec {
            tv_sec: 0,
            tv_nsec: 0,
        }
    }
}

#[cfg(target_os = "linux")]
pub struct timespec {
    tv_sec: i64,
    tv_nsec: i64,
}

}  // namespace platform

//POD Semaphore initialized lazily.
pub struct LazySemaphore<const N: i32> {
    semaphore: Semaphore,
}

impl<const N: i32> LazySemaphore<N> {
    pub const fn new() -> Self {
        LazySemaphore {
            semaphore: Semaphore::new(N),
        }
    }

    pub fn pointer(&self) -> &Semaphore {
        &self.semaphore
    }
}
}  // namespace base
