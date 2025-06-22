// Copyright 2013 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_macros)]

#[cfg(target_os = "windows")]
extern crate winapi;

#[cfg(target_os = "linux")]
extern crate libc;

use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{
    fmt,
    i64,
    sync::{Mutex, OnceLock},
};

#[cfg(target_os = "windows")]
use std::sync::atomic::{AtomicI32, Ordering};

#[cfg(target_os = "windows")]
use winapi::shared::minwindef::{DWORD, FILETIME, ULONG, ULONGLONG};
#[cfg(target_os = "windows")]
use winapi::um::profileapi::QueryPerformanceCounter;
#[cfg(target_os = "windows")]
use winapi::um::sysinfoapi::GetSystemTimeAsFileTime;
#[cfg(target_os = "windows")]
use winapi::um::timeapi::timeGetTime;

// Constants
const K_NANOSECONDS_PER_MICROSECOND: i64 = 1000;
const K_MICROSECONDS_PER_MILLISECOND: i64 = 1000;
const K_MICROSECONDS_PER_SECOND: i64 = 1_000_000;
const K_NANOSECONDS_PER_SECOND: i64 = 1_000_000_000;
const K_MICROSECONDS_PER_MINUTE: i64 = 60 * K_MICROSECONDS_PER_SECOND;
const K_MICROSECONDS_PER_HOUR: i64 = 60 * K_MICROSECONDS_PER_MINUTE;
const K_MICROSECONDS_PER_DAY: i64 = 24 * K_MICROSECONDS_PER_HOUR;

macro_rules! DCHECK_EQ {
    ($left:expr, $right:expr) => {
        if $left != $right {
            panic!("DCHECK_EQ failed: {} != {}", $left, $right);
        }
    };
}

macro_rules! CHECK_EQ {
    ($left:expr, $right:expr) => {
        if $left != $right {
            panic!("CHECK_EQ failed: {} != {}", $left, $right);
        }
    };
}

macro_rules! CHECK_LE {
    ($left:expr, $right:expr) => {
        if $left > $right {
            panic!("CHECK_LE failed: {} > {}", $left, $right);
        }
    };
}

macro_rules! CHECK_GT {
    ($left:expr, $right:expr) => {
        if $left <= $right {
            panic!("CHECK_GT failed: {} <= {}", $left, $right);
        }
    };
}

macro_rules! DCHECK_GE {
    ($left:expr, $right:expr) => {
        if $left < $right {
            panic!("DCHECK_GE failed: {} < {}", $left, $right);
        }
    };
}

macro_rules! DCHECK_LT {
    ($left:expr, $right:expr) => {
        if $left >= $right {
            panic!("DCHECK_LT failed: {} >= {}", $left, $right);
        }
    };
}

macro_rules! USE {
    ($x:expr) => {
        let _ = $x;
    };
}

macro_rules! UNREACHABLE {
    () => {
        panic!("UNREACHABLE");
    };
}

pub mod base {
    use super::*;

    /// Represents a time interval in microseconds.
    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
    pub struct TimeDelta {
        delta_: i64, // Time in microseconds
    }

    impl TimeDelta {
        /// Creates a new `TimeDelta` with the specified number of microseconds.
        pub const fn from_microseconds(delta: i64) -> Self {
            TimeDelta { delta_: delta }
        }

        pub const fn from_nanoseconds(delta: i64) -> Self {
            TimeDelta {
                delta_: delta / K_NANOSECONDS_PER_MICROSECOND,
            }
        }

        pub const fn from_milliseconds(ms: i64) -> Self {
            TimeDelta {
                delta_: ms * K_MICROSECONDS_PER_MILLISECOND,
            }
        }

        pub const fn from_seconds(s: i64) -> Self {
            TimeDelta {
                delta_: s * K_MICROSECONDS_PER_SECOND,
            }
        }

        pub const fn from_minutes(m: i64) -> Self {
            TimeDelta {
                delta_: m * K_MICROSECONDS_PER_MINUTE,
            }
        }

        pub const fn from_hours(h: i64) -> Self {
            TimeDelta {
                delta_: h * K_MICROSECONDS_PER_HOUR,
            }
        }

        pub const fn from_days(d: i64) -> Self {
            TimeDelta {
                delta_: d * K_MICROSECONDS_PER_DAY,
            }
        }

        /// Returns a `TimeDelta` representing the maximum possible duration.
        pub const fn max() -> Self {
            TimeDelta {
                delta_: i64::max_value(),
            }
        }

        /// Checks if this `TimeDelta` represents the maximum possible duration.
        pub const fn is_max(&self) -> bool {
            self.delta_ == i64::max_value()
        }

        /// Returns the number of days in this `TimeDelta`.
        pub fn in_days(&self) -> i32 {
            if self.is_max() {
                i32::max_value()
            } else {
                (self.delta_ / K_MICROSECONDS_PER_DAY) as i32
            }
        }

        /// Returns the number of hours in this `TimeDelta`.
        pub fn in_hours(&self) -> i32 {
            if self.is_max() {
                i32::max_value()
            } else {
                (self.delta_ / K_MICROSECONDS_PER_HOUR) as i32
            }
        }

        /// Returns the number of minutes in this `TimeDelta`.
        pub fn in_minutes(&self) -> i32 {
            if self.is_max() {
                i32::max_value()
            } else {
                (self.delta_ / K_MICROSECONDS_PER_MINUTE) as i32
            }
        }

        /// Returns the number of seconds in this `TimeDelta` as a floating-point number.
        pub fn in_seconds_f(&self) -> f64 {
            if self.is_max() {
                f64::infinity()
            } else {
                self.delta_ as f64 / K_MICROSECONDS_PER_SECOND as f64
            }
        }

        /// Returns the number of seconds in this `TimeDelta`.
        pub fn in_seconds(&self) -> i64 {
            if self.is_max() {
                i64::max_value()
            } else {
                self.delta_ / K_MICROSECONDS_PER_SECOND
            }
        }

        /// Returns the number of milliseconds in this `TimeDelta` as a floating-point number.
        pub fn in_milliseconds_f(&self) -> f64 {
            if self.is_max() {
                f64::infinity()
            } else {
                self.delta_ as f64 / K_MICROSECONDS_PER_MILLISECOND as f64
            }
        }

        /// Returns the number of milliseconds in this `TimeDelta`.
        pub fn in_milliseconds(&self) -> i64 {
            if self.is_max() {
                i64::max_value()
            } else {
                self.delta_ / K_MICROSECONDS_PER_MILLISECOND
            }
        }

        /// Returns the number of milliseconds in this `TimeDelta`, rounded up.
        pub fn in_milliseconds_rounded_up(&self) -> i64 {
            if self.is_max() {
                i64::max_value()
            } else {
                (self.delta_ + K_MICROSECONDS_PER_MILLISECOND - 1) / K_MICROSECONDS_PER_MILLISECOND
            }
        }

        /// Returns the number of microseconds in this `TimeDelta`.
        pub fn in_microseconds(&self) -> i64 {
            if self.is_max() {
                i64::max_value()
            } else {
                self.delta_
            }
        }

        /// Returns the number of nanoseconds in this `TimeDelta`.
        pub fn in_nanoseconds(&self) -> i64 {
            if self.is_max() {
                i64::max_value()
            } else {
                self.delta_ * K_NANOSECONDS_PER_MICROSECOND
            }
        }

        #[cfg(target_os = "darwin")]
        fn from_mach_timespec(ts: mach_timespec) -> Self {
            DCHECK_GE!(ts.tv_nsec, 0);
            DCHECK_LT!(ts.tv_nsec, K_NANOSECONDS_PER_SECOND as i64);
            TimeDelta::from_microseconds(
                ts.tv_sec * K_MICROSECONDS_PER_SECOND + ts.tv_nsec / K_NANOSECONDS_PER_MICROSECOND,
            )
        }

        #[cfg(target_os = "darwin")]
        fn to_mach_timespec(&self) -> mach_timespec {
            let mut ts = mach_timespec {
                tv_sec: 0,
                tv_nsec: 0,
            };
            DCHECK_GE!(self.delta_, 0);
            ts.tv_sec = (self.delta_ / K_MICROSECONDS_PER_SECOND) as i64;
            ts.tv_nsec = (self.delta_ % K_MICROSECONDS_PER_SECOND) * K_NANOSECONDS_PER_MICROSECOND;
            ts
        }

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "emscripten"))]
        fn from_timespec(ts: libc::timespec) -> Self {
            DCHECK_GE!(ts.tv_nsec, 0);
            DCHECK_LT!(ts.tv_nsec, K_NANOSECONDS_PER_SECOND as i64);
            TimeDelta::from_microseconds(
                ts.tv_sec * K_MICROSECONDS_PER_SECOND + ts.tv_nsec / K_NANOSECONDS_PER_MICROSECOND,
            )
        }

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "emscripten"))]
        fn to_timespec(&self) -> libc::timespec {
            let mut ts = libc::timespec {
                tv_sec: 0,
                tv_nsec: 0,
            };
            ts.tv_sec = (self.delta_ / K_MICROSECONDS_PER_SECOND) as i64;
            ts.tv_nsec = (self.delta_ % K_MICROSECONDS_PER_SECOND) * K_NANOSECONDS_PER_MICROSECOND;
            ts
        }
    }

    impl std::ops::Add for TimeDelta {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            TimeDelta::from_microseconds(self.in_microseconds() + other.in_microseconds())
        }
    }

    impl std::ops::Sub for TimeDelta {
        type Output = Self;

        fn sub(self, other: Self) -> Self {
            TimeDelta::from_microseconds(self.in_microseconds() - other.in_microseconds())
        }
    }

    /// Represents a specific point in time.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Time {
        us_: i64, // Time in microseconds since the epoch
    }

    impl Time {
        /// Creates a `Time` object representing the null time.
        pub const fn null() -> Self {
            Time { us_: 0 }
        }

        /// Creates a `Time` object representing the maximum time.
        pub const fn max() -> Self {
            Time {
                us_: i64::max_value(),
            }
        }

        /// Creates a `Time` object from microseconds since the epoch.
        pub const fn from_microseconds(us: i64) -> Self {
            Time { us_: us }
        }

        /// Checks if this `Time` represents the null time.
        pub const fn is_null(&self) -> bool {
            self.us_ == 0
        }

        /// Checks if this `Time` represents the maximum time.
        pub const fn is_max(&self) -> bool {
            self.us_ == i64::max_value()
        }

        #[cfg(target_os = "windows")]
        fn from_filetime(ft: FILETIME) -> Self {
            if ft.dwLowDateTime == 0 && ft.dwHighDateTime == 0 {
                return Time::null();
            }
            if ft.dwLowDateTime == std::u32::MAX && ft.dwHighDateTime == std::u32::MAX {
                return Time::max();
            }
            let us = ((ft.dwLowDateTime as u64) + ((ft.dwHighDateTime as u64) << 32)) / 10;
            Time::from_microseconds((us - K_TIME_TO_EPOCH_IN_MICROSECONDS) as i64)
        }

        #[cfg(target_os = "windows")]
        fn to_filetime(&self) -> FILETIME {
            DCHECK_GE!(self.us_, 0);
            let mut ft = FILETIME {
                dwLowDateTime: 0,
                dwHighDateTime: 0,
            };
            if self.is_null() {
                return ft;
            }
            if self.is_max() {
                ft.dwLowDateTime = std::u32::MAX;
                ft.dwHighDateTime = std::u32::MAX;
                return ft;
            }

            let us = (self.us_ as u64 + K_TIME_TO_EPOCH_IN_MICROSECONDS as u64) * 10;
            ft.dwLowDateTime = us as u32;
            ft.dwHighDateTime = (us >> 32) as u32;
            ft
        }

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "emscripten"))]
        fn from_timespec(ts: libc::timespec) -> Self {
            DCHECK_GE!(ts.tv_nsec, 0);
            DCHECK_LT!(ts.tv_nsec, K_NANOSECONDS_PER_SECOND as i64);
            if ts.tv_nsec == 0 && ts.tv_sec == 0 {
                return Time::null();
            }
            if ts.tv_nsec == (K_NANOSECONDS_PER_SECOND - 1) && ts.tv_sec == i64::max_value() {
                return Time::max();
            }
            Time::from_microseconds(
                ts.tv_sec * K_MICROSECONDS_PER_SECOND + ts.tv_nsec / K_NANOSECONDS_PER_MICROSECOND,
            )
        }

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "emscripten"))]
        fn to_timespec(&self) -> libc::timespec {
            let mut ts = libc::timespec {
                tv_sec: 0,
                tv_nsec: 0,
            };
            if self.is_null() {
                return ts;
            }
            if self.is_max() {
                ts.tv_sec = i64::max_value();
                ts.tv_nsec = (K_NANOSECONDS_PER_SECOND - 1) as i64;
                return ts;
            }
            ts.tv_sec = (self.us_ / K_MICROSECONDS_PER_SECOND) as i64;
            ts.tv_nsec = (self.us_ % K_MICROSECONDS_PER_SECOND) * K_NANOSECONDS_PER_MICROSECOND;
            ts
        }

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "emscripten"))]
        fn from_timeval(tv: libc::timeval) -> Self {
            DCHECK_GE!(tv.tv_usec, 0);
            DCHECK!(tv.tv_usec < K_MICROSECONDS_PER_SECOND as i32);
            if tv.tv_usec == 0 && tv.tv_sec == 0 {
                return Time::null();
            }
            if tv.tv_usec == (K_MICROSECONDS_PER_SECOND - 1) as i32 && tv.tv_sec == i64::max_value()
            {
                return Time::max();
            }
            Time::from_microseconds(tv.tv_sec * K_MICROSECONDS_PER_SECOND + tv.tv_usec as i64)
        }

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "emscripten"))]
        fn to_timeval(&self) -> libc::timeval {
            let mut tv = libc::timeval {
                tv_sec: 0,
                tv_usec: 0,
            };
            if self.is_null() {
                tv.tv_sec = 0;
                tv.tv_usec = 0;
                return tv;
            }
            if self.is_max() {
                tv.tv_sec = i64::max_value();
                tv.tv_usec = (K_MICROSECONDS_PER_SECOND - 1) as i32;
                return tv;
            }
            tv.tv_sec = (self.us_ / K_MICROSECONDS_PER_SECOND) as i64;
            tv.tv_usec = (self.us_ % K_MICROSECONDS_PER_SECOND) as i32;
            tv
        }

        /// Creates a `Time` object from JavaScript time (milliseconds since the epoch).
        pub fn from_js_time(ms_since_epoch: f64) -> Self {
            if ms_since_epoch == f64::INFINITY {
                return Time::max();
            }
            Time::from_microseconds((ms_since_epoch * K_MICROSECONDS_PER_MILLISECOND as f64) as i64)
        }

        /// Converts this `Time` object to JavaScript time (milliseconds since the epoch).
        pub fn to_js_time(&self) -> f64 {
            if self.is_null() {
                return 0.0;
            }
            if self.is_max() {
                return f64::INFINITY;
            }
            self.us_ as f64 / K_MICROSECONDS_PER_MILLISECOND as f64
        }

        #[cfg(target_os = "windows")]
        fn now() -> Self {
            GetClock::get().Now()
        }

        #[cfg(not(target_os = "windows"))]
        fn now() -> Self {
            let now = SystemTime::now();
            let duration = now.duration_since(UNIX_EPOCH).unwrap();
            Time::from_microseconds(duration.as_micros() as i64)
        }

        #[cfg(target_os = "windows")]
        fn now_from_system_time() -> Self {
            GetClock::get().NowFromSystemTime()
        }

        #[cfg(not(target_os = "windows"))]
        fn now_from_system_time() -> Self {
            Time::now()
        }
    }

    impl fmt::Display for Time {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.to_js_time())
        }
    }

    impl std::ops::Add<TimeDelta> for Time {
        type Output = Self;

        fn add(self, delta: TimeDelta) -> Self {
            Time::from_microseconds(self.us_ + delta.in_microseconds())
        }
    }

    impl std::ops::Sub<TimeDelta> for Time {
        type Output = Self;

        fn sub(self, delta: TimeDelta) -> Self {
            Time::from_microseconds(self.us_ - delta.in_microseconds())
        }
    }

    /// Represents a timestamp taken from a high-resolution clock.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct TimeTicks {
        ticks_: i64,
    }

    impl TimeTicks {
        const K_QPC_OVERFLOW_THRESHOLD: i64 = i64::max_value() / 1000;
        const K_MICROSECONDS_PER_SECOND: i64 = 1_000_000;

        /// Creates a `TimeTicks` object with the specified number of ticks.
        pub const fn from_ticks(ticks: i64) -> Self {
            TimeTicks { ticks_: ticks }
        }

        /// Returns a `TimeTicks` object representing the current time.
        #[cfg(target_os = "windows")]
        pub fn now() -> Self {
            // Make sure we never return 0 here.
            let ticks = g_time_ticks_now_function
                .get()
                .expect("TimeTicks::Now: g_time_ticks_now_function not initialized")();
            DCHECK!(!ticks.is_null());
            ticks
        }

        #[cfg(not(target_os = "windows"))]
        pub fn now() -> Self {
            let ticks: i64;

            #[cfg(target_os = "darwin")]
            {
                let mut info = DarwinTimebaseInfo::new();
                ticks = ((unsafe { mach_absolute_time() } / Time::kNanosecondsPerMicrosecond)
                    * info.numer
                    / info.denom);
            }

            #[cfg(target_os = "fuchsia")]
            {
                ticks = unsafe { zx_clock_get_monotonic() } / Time::kNanosecondsPerMicrosecond;
            }

            #[cfg(any(target_os = "linux", target_os = "android", target_os = "emscripten"))]
            {
                ticks = clock_now(CLOCK_MONOTONIC);
            }

            // Make sure we never return 0 here.
            TimeTicks::from_ticks(ticks + 1)
        }

        /// Checks if this `TimeTicks` represents the null time.
        pub const fn is_null(&self) -> bool {
            self.ticks_ == 0
        }

        /// Checks if the high-resolution clock is being used.
        #[cfg(target_os = "windows")]
        pub fn is_high_resolution() -> bool {
            if g_time_ticks_now_function.get().is_none() {
                initial_time_ticks_now_function();
            }
            g_time_ticks_now_function.get() == Some(&qpc_now)
        }

        #[cfg(not(target_os = "windows"))]
        pub fn is_high_resolution() -> bool {
            #[cfg(target_os = "darwin")]
            {
                true
            }

            #[cfg(target_os = "fuchsia")]
            {
                true
            }

            #[cfg(any(target_os = "linux", target_os = "android", target_os = "emscripten"))]
            {
                static IS_HIGH_RESOLUTION: OnceLock<bool> = OnceLock::new();
                *IS_HIGH_RESOLUTION.get_or_init(|| is_high_resolution_timer(CLOCK_MONOTONIC))
            }
        }
    }

    impl std::ops::Add<TimeDelta> for TimeTicks {
        type Output = Self;

        fn add(self, delta: TimeDelta) -> Self {
            TimeTicks::from_ticks(self.ticks_ + delta.in_microseconds())
        }
    }

    impl std::ops::Sub for TimeTicks {
        type Output = TimeDelta;

        fn sub(self, other: Self) -> TimeDelta {
            TimeDelta::from_microseconds(self.ticks_ - other.ticks_)
        }
    }

    /// Represents CPU usage time for a thread.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ThreadTicks {
        ticks_: i64,
    }

    impl ThreadTicks {
        /// Creates a `ThreadTicks` object with the specified number of ticks.
        pub const fn from_ticks(ticks: i64) -> Self {
            ThreadTicks { ticks_: ticks }
        }

        /// Checks if thread ticks are supported on the current platform.
        pub fn is_supported() -> bool {
            #[cfg(any(target_os = "linux", target_os = "android", target_os = "emscripten"))]
            {
                true
            }

            #[cfg(target_os = "darwin")]
            {
                true
            }

            #[cfg(target_os = "windows")]
            {
                is_supported_win()
            }
        }

        /// Returns a `ThreadTicks` object representing the current thread's CPU usage time.
        pub fn now() -> Self {
            #[cfg(target_os = "darwin")]
            {
                ThreadTicks::from_ticks(compute_thread_ticks())
            }

            #[cfg(target_os = "fuchsia")]
            {
                ThreadTicks::from_ticks(get_fuchsia_thread_ticks())
            }

            #[cfg(any(target_os = "linux", target_os = "android", target_os = "emscripten"))]
            {
                ThreadTicks::from_ticks(clock_now(CLOCK_THREAD_CPUTIME_ID))
            }

            #[cfg(target_os = "windows")]
            {
                Self::get_for_thread(unsafe { GetCurrentThread() })
            }
        }

        #[cfg(target_os = "windows")]
        fn get_for_thread(thread_handle: *mut winapi::ctypes::c_void) -> Self {
            DCHECK!(Self::is_supported());

            #[cfg(target_arch = "aarch64")]
            {
                // QueryThreadCycleTime versus TSCTicksPerSecond doesn't have much relation to
                // actual elapsed time on Windows on Arm, because QueryThreadCycleTime is
                // backed by the actual number of CPU cycles executed, rather than a
                // constant-rate timer like Intel. To work around this, use GetThreadTimes
                // (which isn't as accurate but is meaningful as a measure of elapsed
                // per-thread time).
                let mut creation_time: FILETIME = FILETIME {
                    dwLowDateTime: 0,
                    dwHighDateTime: 0,
                };
                let mut exit_time: FILETIME = FILETIME {
                    dwLowDateTime: 0,
                    dwHighDateTime: 0,
                };
                let mut kernel_time: FILETIME = FILETIME {
                    dwLowDateTime: 0,
                    dwHighDateTime: 0,
                };
                let mut user_time: FILETIME = FILETIME {
                    dwLowDateTime: 0,
                    dwHighDateTime: 0,
                };

                unsafe {
                    GetThreadTimes(
                        thread_handle,
                        &mut creation_time,
                        &mut exit_time,
                        &mut kernel_time,
                        &mut user_time,
                    )
                };

                let us = file_time_to_microseconds(user_time);
                return ThreadTicks::from_ticks(us);
            }

            #[cfg(not(target_arch = "aarch64"))]
            {
                // Get the number of TSC ticks used by the current thread.
                let mut thread_cycle_time: ULONGLONG = 0;
                unsafe { QueryThreadCycleTime(thread_handle, &mut thread_cycle_time) };

                // Get the frequency of the TSC.
                let tsc_ticks_per_second = tsc_ticks_per_second();
                if tsc_ticks_per_second == 0.0 {
                    return ThreadTicks::from_ticks(0);
                }

                // Return the CPU time of the current thread.
                let thread_time_seconds = thread_cycle_time as f64 / tsc_ticks_per_second;
                return ThreadTicks::from_ticks(
                    (thread_time_seconds * K_MICROSECONDS_PER_SECOND as f64) as i64,
                );
            }
        }

        #[cfg(target_os = "windows")]
        fn wait_until_initialized_win() {
            #[cfg(not(target_arch = "aarch64"))]
            while tsc_ticks_per_second() == 0.0 {
                unsafe { Sleep(10) };
            }
        }
    }

    //
    // Platform Specific Implementations
    //

    #[cfg(target_os = "darwin")]
    extern "C" {
        fn mach_absolute_time() -> u64;
    }

    #[cfg(target_os = "fuchsia")]
    extern "C" {
        fn zx_clock_get_monotonic() -> i64;
    }

    #[cfg(any(target_os = "linux", target_os = "android", target_os = "emscripten"))]
    extern "C" {
        fn clock_gettime(clockid: libc::clockid_t, tp: *mut libc::timespec) -> libc::c_int;
    }

    #[cfg(target_os = "windows")]
    extern "system" {
        fn GetCurrentThread() -> *mut winapi::ctypes::c_void;
        fn QueryThreadCycleTime(
            ThreadHandle: *mut winapi::ctypes::c_void,
            CycleTime: *mut ULONGLONG,
        ) -> winapi::shared::minwindef::BOOL;
        fn Sleep(dwMilliseconds: DWORD);
        fn GetThreadTimes(
            hThread: *mut winapi::ctypes::c_void,
            lpCreationTime: *mut FILETIME,
            lpExitTime: *mut FILETIME,
            lpKernelTime: *mut FILETIME,
            lpUserTime: *mut FILETIME,
        ) -> winapi::shared::minwindef::BOOL;
    }

    //
    // Platform Specific Structures
    //

    #[cfg(target_os = "darwin")]
    #[repr(C)]
    struct mach_timespec {
        tv_sec: i64,
        tv_nsec: i64,
    }

    //
    // Darwin Specific Code
    //

    #[cfg(target_os = "darwin")]
    struct DarwinTimebaseInfo {
        numer: u32,
        denom: u32,
    }

    #[cfg(target_os = "darwin")]
    impl DarwinTimebaseInfo {
        fn new() -> Self {
            use std::mem;
            use std::ptr;

            let mut info = DarwinTimebaseInfo { numer: 0, denom: 0 };
            let info_ptr = &mut info as *mut DarwinTimebaseInfo as *mut _;

            extern "C" {
                fn mach_timebase_info(info: *mut u32) -> i32;
            }

            unsafe {
                mach_timebase_info(info_ptr);
            }

            info
        }
    }

    #[cfg(target_os = "darwin")]
    fn compute_thread_ticks() -> i64 {
        // Missing implementations
        0
    }

    //
    // Fuchsia Specific Code
    //

    #[cfg(target_os = "fuchsia")]
    fn get_fuchsia_thread_ticks() -> i64 {
        // Missing implementations
        0
    }

    //
    // Linux Specific Code
    //

    #[cfg(any(target_os = "linux", target_os = "android", target_os = "emscripten"))]
    const CLOCK_MONOTONIC: libc::clockid_t = libc::CLOCK_MONOTONIC;

    #[cfg(any(target_os = "linux", target_os = "android", target_os = "emscripten"))]
    const CLOCK_THREAD_CPUTIME_ID: libc::clockid_t = libc::CLOCK_THREAD_CPUTIME_ID;

    #[cfg(any(target_os = "linux", target_os = "android", target_os = "emscripten"))]
    fn clock_now(clk_id: libc::clockid_t) -> i64 {
        let mut ts = libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };

        unsafe {
            if clock_gettime(clk_id, &mut ts) != 0 {
                UNREACHABLE!();
            }
        }

        let k_seconds_limit: i64 = (i64::max_value() / K_MICROSECONDS_PER_SECOND) - 1;
        CHECK_GT!(k_seconds_limit, ts.tv_sec);
        let result = i64::from(ts.tv_sec) * K_MICROSECONDS_PER_SECOND
            + (ts.tv_nsec / K_NANOSECONDS_PER_MICROSECOND);
        result
    }

    #[cfg(any(target_os = "linux", target_os = "android", target_os = "emscripten"))]
    fn nanoseconds_now() -> i64 {
        let mut ts = libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        unsafe {
            clock_