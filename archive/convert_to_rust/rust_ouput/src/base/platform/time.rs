// Converted from V8 C++ source files:
// Header: time.h
// Implementation: time.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_camel_case_types)]

use std::cmp;
use std::fmt;
use std::i64;
use std::io::Write;
use std::mem::transmute;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::sync::{Mutex, MutexGuard};
use std::time::Duration;
use std::{atomic::Ordering, convert::Infallible};

#[cfg(v8_os_win)]
use winapi::shared::minwindef::{DWORD, FILETIME, LARGE_INTEGER, BOOL};
#[cfg(v8_os_win)]
use winapi::um::profileapi::QueryPerformanceCounter;
#[cfg(v8_os_win)]
use winapi::um::sysinfoapi::GetSystemTimeAsFileTime;
#[cfg(v8_os_win)]
use winapi::um::processthreadsapi::GetCurrentThread;
#[cfg(v8_os_win)]
use winapi::shared::ntdef::HANDLE;
#[cfg(v8_os_win)]
use winapi::um::processthreadsapi::QueryThreadCycleTime;
#[cfg(v8_os_win)]
use winapi::um::processthreadsapi::GetThreadTimes;
#[cfg(v8_os_win)]
use winapi::um::processthreadsapi::GetThreadPriority;
#[cfg(v8_os_win)]
use winapi::um::processthreadsapi::SetThreadPriority;
#[cfg(v8_os_win)]
use winapi::um::processthreadsapi::THREAD_PRIORITY_HIGHEST;
#[cfg(v8_os_win)]
use winapi::um::synchapi::Sleep;
#[cfg(v8_os_win)]
use std::sync::atomic::AtomicI32;
#[cfg(v8_os_win)]
use std::sync::atomic::AtomicU64;
#[cfg(v8_os_win)]
use std::sync::atomic;

#[cfg(v8_os_posix)]
use libc::{clock_gettime, CLOCK_MONOTONIC, timespec, timeval, gettimeofday, CLOCK_THREAD_CPUTIME_ID, time_t, suseconds_t};
#[cfg(v8_os_posix)]
use std::ffi::c_void;

#[cfg(v8_os_darwin)]
use libc::{mach_timespec};

use crate::base::bits;
use crate::base::cpu::CPU;
use crate::base::logging::CHECK_EQ;
use crate::base::logging::CHECK_GE;
use crate::base::logging::CHECK_GT;
use crate::base::logging::CHECK_LE;
use crate::base::logging::DCHECK_EQ;
use crate::base::logging::DCHECK_GE;
use crate::base::logging::DCHECK_GT;
use crate::base::logging::DCHECK_LE;
use crate::base::logging::UNREACHABLE;
use crate::base::macros::Use;
use crate::base::safe_conversions::saturated_cast;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU8;

pub struct Time {}
pub struct TimeDelta {}
pub struct TimeTicks {}

pub mod time_internal {
    pub struct TimeBase {}
}

pub struct Clock {}

pub struct ThreadTicks {}

pub struct Counters {}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct V8_BASE_EXPORT {}

impl fmt::Display for V8_BASE_EXPORT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "V8_BASE_EXPORT")
    }
}

pub struct V8 {}

pub mod base {
    pub struct TimeDelta {}
    pub struct TimeTicks {}
    pub struct Time {}
    pub mod time_internal {
        pub struct TimeBase {}
    }
}

mod win32_headers {
    pub type DWORD = u32;
}

pub struct FileEvent {}

pub struct MutexGuard {}

pub enum RegionState {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SyncTag {}

pub struct String {}

pub struct Isolate {}

pub struct MarkBit {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Bytecode {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Condition {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MachineType {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum GCConfigMarkingType {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum OpIndex {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Register {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operand {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AbortReason {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Immediate {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Address {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operation {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Block {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CpuFeature {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Display {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum JSPluralRules {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CFunction {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Int64Representation {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum OpIndex {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Word32 {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Flag {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HeapObject {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HeapObject {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MarkBit {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HeapObject {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DirectHandle<T> {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Managed<T> {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CallFrequency {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MarkBit {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PageMemoryRegion<'static> {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Heap {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HeapObject {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MarkBit {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RegExpDataWrapper {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum JSFunction {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum InstructionOperand {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum IrregexpImplementation {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Context {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Expression {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operation {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MarkBit {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum String {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FPURegister {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Flag {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CallSiteInfo {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CFIMetadataWriteScope<'a> {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum T {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DirectHandle<T> {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FileEvent {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum V<T> {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum VisitResult {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AssemblerOptions {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MarkingWorklist {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MarkingWorklistsLocal {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum StackFrame {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Intrinsic {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AbortReason {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum JSReceiver {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CodeKind {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tagged<T> {}

pub trait Platform { }

pub struct Label {}

pub struct Operation { }

pub type digit_t = u32;

pub enum Status {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ImmediateMode {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Builtin {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Constant {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Instruction {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CallContext {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Name {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MarkBit {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CodeAddress {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum JSTypedArray {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tagged<String> {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaggedObject {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Scope {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum VisitResult {}

pub struct Bignum {}

pub struct DiyFp {}

pub struct JSPluralRulesInternal {}

pub struct JSPluralRules {}

pub struct MachineRepresentation {}

pub struct Callbacks {}

pub struct MarkingVisitor {}

pub struct Common {}

impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "String")
    }
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Context")
    }
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Display")
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Address {}

impl TimeConstants {
    pub const HOURS_PER_DAY: i64 = 24;
    pub const MILLISECONDS_PER_SECOND: i64 = 1000;
    pub const MILLISECONDS_PER_DAY: i64 =
        TimeConstants::MILLISECONDS_PER_SECOND * 60 * 60 * TimeConstants::HOURS_PER_DAY;
    pub const MICROSECONDS_PER_MILLISECOND: i64 = 1000;
    pub const MICROSECONDS_PER_SECOND: i64 =
        TimeConstants::MICROSECONDS_PER_MILLISECOND * TimeConstants::MILLISECONDS_PER_SECOND;
    pub const MICROSECONDS_PER_MINUTE: i64 = TimeConstants::MICROSECONDS_PER_SECOND * 60;
    pub const MICROSECONDS_PER_HOUR: i64 = TimeConstants::MICROSECONDS_PER_MINUTE * 60;
    pub const MICROSECONDS_PER_DAY: i64 = TimeConstants::MICROSECONDS_PER_HOUR * TimeConstants::HOURS_PER_DAY;
    pub const MICROSECONDS_PER_WEEK: i64 = TimeConstants::MICROSECONDS_PER_DAY * 7;
    pub const NANOSECONDS_PER_MICROSECOND: i64 = 1000;
    pub const NANOSECONDS_PER_SECOND: i64 =
        TimeConstants::NANOSECONDS_PER_MICROSECOND * TimeConstants::MICROSECONDS_PER_SECOND;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeDelta {
    delta_ : i64,
}

impl TimeDelta {
    pub const fn new() -> Self {
        TimeDelta { delta_: 0 }
    }

    pub const fn from_days(days: i32) -> Self {
        TimeDelta {
            delta_: days as i64 * TimeConstants::MICROSECONDS_PER_DAY,
        }
    }

    pub const fn from_hours(hours: i32) -> Self {
        TimeDelta {
            delta_: hours as i64 * TimeConstants::MICROSECONDS_PER_HOUR,
        }
    }

    pub const fn from_minutes(minutes: i32) -> Self {
        TimeDelta {
            delta_: minutes as i64 * TimeConstants::MICROSECONDS_PER_MINUTE,
        }
    }

    pub const fn from_seconds(seconds: i64) -> Self {
        TimeDelta {
            delta_: seconds * TimeConstants::MICROSECONDS_PER_SECOND,
        }
    }

    pub const fn from_milliseconds(milliseconds: i64) -> Self {
        TimeDelta {
            delta_: milliseconds * TimeConstants::MICROSECONDS_PER_MILLISECOND,
        }
    }

    pub const fn from_microseconds(microseconds: i64) -> Self {
        TimeDelta { delta_: microseconds }
    }

    pub const fn from_nanoseconds(nanoseconds: i64) -> Self {
        TimeDelta {
            delta_: nanoseconds / TimeConstants::NANOSECONDS_PER_MICROSECOND,
        }
    }

    pub const fn from_seconds_d(seconds: f64) -> Self {
        TimeDelta::from_double(seconds * TimeConstants::MICROSECONDS_PER_SECOND as f64)
    }

    pub const fn from_milliseconds_d(milliseconds: f64) -> Self {
        TimeDelta::from_double(milliseconds * TimeConstants::MICROSECONDS_PER_MILLISECOND as f64)
    }

    pub const fn max() -> Self {
        TimeDelta {
            delta_: i64::MAX,
        }
    }

    pub const fn min() -> Self {
        TimeDelta {
            delta_: i64::MIN,
        }
    }

    pub const fn is_zero(&self) -> bool {
        self.delta_ == 0
    }

    pub const fn is_max(&self) -> bool {
        self.delta_ == i64::MAX
    }

    pub const fn is_min(&self) -> bool {
        self.delta_ == i64::MIN
    }

    pub fn in_days(&self) -> i32 {
        if self.is_max() {
            i32::MAX
        } else {
            (self.delta_ / Time::MICROSECONDS_PER_DAY) as i32
        }
    }

    pub fn in_hours(&self) -> i32 {
        if self.is_max() {
            i32::MAX
        } else {
            (self.delta_ / Time::MICROSECONDS_PER_HOUR) as i32
        }
    }

    pub fn in_minutes(&self) -> i32 {
        if self.is_max() {
            i32::MAX
        } else {
            (self.delta_ / Time::MICROSECONDS_PER_MINUTE) as i32
        }
    }

    pub fn in_seconds_f(&self) -> f64 {
        if self.is_max() {
            f64::INFINITY
        } else {
            self.delta_ as f64 / Time::MICROSECONDS_PER_SECOND as f64
        }
    }

    pub fn in_seconds(&self) -> i64 {
        if self.is_max() {
            i64::MAX
        } else {
            self.delta_ / Time::MICROSECONDS_PER_SECOND
        }
    }

    pub fn in_milliseconds_f(&self) -> f64 {
        if self.is_max() {
            f64::INFINITY
        } else {
            self.delta_ as f64 / Time::MICROSECONDS_PER_MILLISECOND as f64
        }
    }

    pub fn in_milliseconds(&self) -> i64 {
        if self.is_max() {
            i64::MAX
        } else {
            self.delta_ / Time::MICROSECONDS_PER_MILLISECOND
        }
    }

    pub fn in_milliseconds_rounded_up(&self) -> i64 {
        if self.is_max() {
            i64::MAX
        } else {
            (self.delta_ + Time::MICROSECONDS_PER_MILLISECOND - 1) / Time::MICROSECONDS_PER_MILLISECOND
        }
    }

    pub fn in_microseconds(&self) -> i64 {
        if self.is_max() {
            i64::MAX
        } else {
            self.delta_
        }
    }

    pub fn in_nanoseconds(&self) -> i64 {
        if self.is_max() {
            i64::MAX
        } else {
            self.delta_ * Time::NANOSECONDS_PER_MICROSECOND
        }
    }

    #[cfg(v8_os_darwin)]
    pub fn from_mach_timespec(ts: mach_timespec) -> Self {
        DCHECK_GE!(ts.tv_nsec, 0);
        DCHECK_LT!(ts.tv_nsec, Time::NANOSECONDS_PER_SECOND);
        TimeDelta {
            delta_: (ts.tv_sec * Time::MICROSECONDS_PER_SECOND) + (ts.tv_nsec / Time::NANOSECONDS_PER_MICROSECOND)
        }
    }

    #[cfg(v8_os_darwin)]
    pub fn to_mach_timespec(&self) -> mach_timespec {
        let mut ts: mach_timespec = unsafe { std::mem::zeroed() };
        DCHECK_GE!(self.delta_, 0);
        ts.tv_sec = (self.delta_ / Time::MICROSECONDS_PER_SECOND) as i64;
        ts.tv_nsec = (self.delta_ % Time::MICROSECONDS_PER_SECOND) * Time::NANOSECONDS_PER_MICROSECOND;
        ts
    }

    #[cfg(v8_os_posix)]
    pub fn from_timespec(ts: timespec) -> Self {
        DCHECK_GE!(ts.tv_nsec, 0);
        DCHECK_LT!(ts.tv_nsec, Time::NANOSECONDS_PER_SECOND);
        TimeDelta {
            delta_: (ts.tv_sec * Time::MICROSECONDS_PER_SECOND) + (ts.tv_nsec / Time::NANOSECONDS_PER_MICROSECOND)
        }
    }

    #[cfg(v8_os_posix)]
    pub fn to_timespec(&self) -> timespec {
        let mut ts: timespec = unsafe { std::mem::zeroed() };
        ts.tv_sec = (self.delta_ / Time::MICROSECONDS_PER_SECOND) as i64;
        ts.tv_nsec = (self.delta_ % Time::MICROSECONDS_PER_SECOND) * Time::NANOSECONDS_PER_MICROSECOND;
        ts
    }

    fn from_double(value: f64) -> Self {
        TimeDelta {
            delta_: saturated_cast::<i64>(value),
        }
    }

    pub fn times_of(&self, other: &TimeDelta) -> f64 {
        self.delta_ as f64 / other.delta_ as f64
    }

    pub fn percent_of(&self, other: &TimeDelta) -> f64 {
        self.times_of(other) * 100.0
    }
}

impl Add for TimeDelta {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        TimeDelta {
            delta_: self.delta_ + other.delta_,
        }
    }
}

impl Sub for TimeDelta {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        TimeDelta {
            delta_: self.delta_ - other.delta_,
        }
    }
}

impl Mul<i64> for TimeDelta {
    type Output = Self;

    fn mul(self, a: i64) -> Self {
        TimeDelta { delta_: self.delta_ * a }
    }
}

impl Div<i64> for TimeDelta {
    type Output = Self;

    fn div(self, a: i64) -> Self {
        TimeDelta { delta_: self.delta_ / a }
    }
}

impl Div<TimeDelta> for TimeDelta {
    type Output = i64;

    fn div(self, other: TimeDelta) -> i64 {
        self.delta_ / other.delta_
    }
}

impl Neg for TimeDelta {
    type Output = Self;

    fn neg(self) -> Self {
        TimeDelta { delta_: -self.delta_ }
    }
}

impl AddAssign for TimeDelta {
    fn add_assign(&mut self, other: Self) {
        self.delta_ += other.delta_;
    }
}

impl SubAssign for TimeDelta {
    fn sub_assign(&mut self, other: Self) {
        self.delta_ -= other.delta_;
    }
}

impl MulAssign<i64> for TimeDelta {
    fn mul_assign(&mut self, a: i64) {
        self.delta_ *= a;
    }
}

impl DivAssign<i64> for TimeDelta {
    fn div_assign(&mut self, a: i64) {
        self.delta_ /= a;
    }
}

impl Time {
    pub const HOURS_PER_DAY: i64 = 24;
    pub const MILLISECONDS_PER_SECOND: i64 = 1000;
    pub const MILLISECONDS_PER_DAY: i64 = Time::MILLISECONDS_PER_SECOND * 60 * 60 * Time::HOURS_PER_DAY;
    pub const MICROSECONDS_PER_MILLISECOND: i64 = 1000;
    pub const MICROSECONDS_PER_SECOND: i64 = Time::MICROSECONDS_PER_MILLISECOND * Time::MILLISECONDS_PER_SECOND;
    pub const MICROSECONDS_PER_MINUTE: i64 = Time::MICROSECONDS_PER_SECOND * 60;
    pub const MICROSECONDS_PER_HOUR: i64 = Time::MICROSECONDS_PER_MINUTE * 60;
    pub const MICROSECONDS_PER_DAY: i64 = Time::MICROSECONDS_PER_HOUR * Time::HOURS_PER_DAY;
    pub const MICROSECONDS_PER_WEEK: i64 = Time::MICROSECONDS_PER_DAY * 7;
    pub const NANOSECONDS_PER_MICROSECOND: i64 = 1000;
    pub const NANOSECONDS_PER_SECOND: i64 = Time::NANOSECONDS_PER_MICROSECOND * Time::MICROSECONDS_PER_SECOND;

}

impl Time {
    pub const fn new() -> Self {
        Time {
            us_: 0,
        }
    }

    #[cfg(v8_os_win)]
    pub fn now() -> Self {
        Clock::now()
    }

    #[cfg(not(v8_os_win))]
    pub fn now() -> Self {
        let tv = Time::gettimeofday_wrapper();
        Time::from_timeval(tv)
    }

    #[cfg(v8_os_win)]
    pub fn now_from_system_time() -> Self {
        Clock::now_from_system_time()
    }

    #[cfg(not(v8_os_win))]
    pub fn now_from_system_time() -> Self {
        Time::now()
    }

    pub fn unix_epoch() -> Self {
        Time { us_: 0 }
    }

    #[cfg(v8_os_posix)]
    pub fn from_timespec(ts: timespec) -> Self {
        DCHECK_GE!(ts.tv_nsec, 0);
        DCHECK_LT!(ts.tv_nsec, Time::NANOSECONDS_PER_SECOND);
        if ts.tv_nsec == 0 && ts.tv_sec == 0 {
            return Time::new();
        }
        if ts.tv_nsec == (Time::NANOSECONDS_PER_SECOND - 1) &&
            ts.tv_sec == i64::MAX as i64 {
            return Time::max();
        }
        Time {
            us_: ts.tv_sec * Time::MICROSECONDS_PER_SECOND + ts.tv_nsec / Time::NANOSECONDS_PER_MICROSECOND
        }
    }

    #[cfg(v8_os_posix)]
    pub fn to_timespec(&self) -> timespec {
        let mut ts: timespec = unsafe { std::mem::zeroed() };
        if self.is_null() {
            ts.tv_sec = 0;
            ts.tv_nsec = 0;
            return ts;
        }
        if self.is_max() {
            ts.tv_sec = i64::MAX as time_t;
            ts.tv_nsec = (Time::NANOSECONDS_PER_SECOND - 1) as libc::c_long;
            return ts;
        }
        ts.tv_sec = (self.us_ / Time::MICROSECONDS_PER_SECOND) as i64;
        ts.tv_nsec = (self.us_ % Time::MICROSECONDS_PER_SECOND) * Time::NANOSECONDS_PER_MICROSECOND;
        ts
    }

    #[cfg(v8_os_posix)]
    pub fn from_timeval(tv: timeval) -> Self {
        DCHECK_GE!(tv.tv_usec, 0);
        DCHECK_LT!(tv.tv_usec, Time::MICROSECONDS_PER_SECOND);
        if tv.tv_usec == 0 && tv.tv_sec == 0 {
            return Time::new();
        }
        if tv.tv_usec == (Time::MICROSECONDS_PER_SECOND - 1) &&
            tv.tv_sec == i64::MAX as i64 {
            return Time::max();
        }
        Time {
            us_: tv.tv_sec * Time::MICROSECONDS_PER_SECOND + tv.tv_usec
        }
    }

    #[cfg(v8_os_posix)]
    pub fn to_timeval(&self) -> timeval {
        let mut tv: timeval = unsafe { std::mem::zeroed() };
        if self.is_null() {
            tv.tv_sec = 0;
            tv.tv_usec = 0;
            return tv;
        }
        if self.is_max() {
            tv.tv_sec = i64::MAX as libc::time_t;
            tv.tv_usec = (Time::MICROSECONDS_PER_SECOND - 1) as libc::suseconds_t;
            return tv;
        }
        tv.tv_sec = (self.us_ / Time::MICROSECONDS_PER_SECOND) as libc::time_t;
        tv.tv_usec = (self.us_ % Time::MICROSECONDS_PER_SECOND) as libc::suseconds_t;
        tv
    }

    #[cfg(v8_os_win)]
    pub fn from_filetime(ft: FILETIME) -> Self {
        if ft.dwLowDateTime == 0 && ft.dwHighDateTime == 0 {
            return Time::new();
        }
        if ft.dwLowDateTime == u32::MAX &&
            ft.dwHighDateTime == u32::MAX {
            return Time::max();
        }

        let us = ((ft.dwLowDateTime as u64) +
                      ((ft.dwHighDateTime as u64) << 32)) / 10;
        Time {
            us_: (us - TIME_TO_EPOCH_IN_MICROSECONDS) as i64
        }
    }

    #[cfg(v8_os_win)]
    pub fn to_filetime(&self) -> FILETIME {
        let mut ft: FILETIME = unsafe { std::mem::zeroed() };

        if self.is_null() {
            return ft;
        }

        if self.is_max() {
            ft.dwLowDateTime = u32::MAX;
            ft.dwHighDateTime = u32::MAX;
            return ft;
        }

        let us = (self.us_ as u64 + TIME_TO_EPOCH_IN_MICROSECONDS) * 10;
        ft.dwLowDateTime = (us & 0xFFFFFFFF) as u32;
        ft.dwHighDateTime = (us >> 32) as u32;
        ft
    }

    pub fn from_js_time(ms_since_epoch: f64) -> Self {
        if ms_since_epoch == f64::MAX {
            return Time::max();
        }

        Time {
            us_: (ms_since_epoch * Time::MICROSECONDS_PER_MILLISECOND as f64) as i64,
        }
    }

    pub fn to_js_time(&self) -> f64 {
        if self.is_null() {
            return 0.0;
        }
        if self.is_max() {
            return f64::MAX;
        }
        self.us_ as f64 / Time::MICROSECONDS_PER_MILLISECOND as f64
    }

    pub fn is_null(&self) -> bool {
        self.us_ == 0
    }

    pub fn is_max(&self) -> bool {
        self.us_ == i64::MAX
    }

    pub fn is_min(&self) -> bool {
        self.us_ == i64::MIN
    }

    pub fn max() -> Self {
        Time {
            us_: i64::MAX
        }
    }

    #[cfg(v8_os_posix)]
    fn gettimeofday_wrapper() -> timeval {
        let mut tv: timeval = unsafe { std::mem::zeroed() };
        let result = unsafe { gettimeofday(&mut tv, std::ptr::null_mut()) };
        DCHECK_EQ!(0, result);
        tv
    }

}

static TIME_TO_EPOCH_IN_MICROSECONDS: u64 = 11644473600000000;

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_js_time())
    }
}

impl TimeTicks {
    pub const fn new() -> Self {
        TimeTicks {
            us_: 0,
        }
    }

    #[cfg(v8_os_win)]
    pub fn now() -> Self {
        let ticks = Clock::time_ticks_now();
        DCHECK!(!ticks.is_null());
        ticks
    }

    #[cfg(not(v8_os_win))]
    pub fn now() -> Self {
        let ticks = TimeTicks::now_ticks();
        TimeTicks {
            us_: (ticks + 1) as i64
        }
    }

    #[cfg(v8_os_win)]
    pub fn is_high_resolution() -> bool {
        Clock::is_high_resolution()
    }

    #[cfg(not(v8_os_win))]
    pub fn is_high_resolution() -> bool {
        true
    }

    pub
