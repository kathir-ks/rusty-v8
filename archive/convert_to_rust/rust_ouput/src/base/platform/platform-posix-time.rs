// Converted from V8 C++ source files:
// Header: platform-posix-time.h
// Implementation: platform-posix-time.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

use std::time::{SystemTime, UNIX_EPOCH};
use std::os::raw::c_char;
use std::ffi::{CStr, CString};
use std::ptr;
use std::sync::Mutex;
use lazy_static::lazy_static;

#[link(name = "c")]
extern "C" {
    fn time(timer: *mut time_t) -> time_t;
    fn localtime_r(timer: *const time_t, result: *mut tm) -> *mut tm;
}

type time_t = i64;

#[repr(C)]
struct tm {
    tm_sec: i32,         // seconds (0-60)
    tm_min: i32,         // minutes (0-59)
    tm_hour: i32,        // hours (0-23)
    tm_mday: i32,        // day of the month (1-31)
    tm_mon: i32,         // month (0-11)
    tm_year: i32,        // year - 1900
    tm_wday: i32,        // day of the week (0-6, Sunday = 0)
    tm_yday: i32,        // day in the year (0-365, 1 Jan = 0)
    tm_isdst: i32,       // daylight saving time
    tm_gmtoff: i64,      // offset from UTC in seconds
    tm_zone: *const c_char, // timezone abbreviation
}

pub struct PosixDefaultTimezoneCache {}

impl PosixDefaultTimezoneCache {
    pub fn new() -> Self {
        PosixDefaultTimezoneCache {}
    }

    pub fn local_timezone(&self, time_ms: f64) -> Result<String, String> {
        if time_ms.is_nan() {
            return Ok("".to_string());
        }
        let tv = (time_ms / 1000.0).floor() as time_t;
        let mut tm_struct: tm = unsafe { std::mem::zeroed() };
        let t = unsafe { localtime_r(&tv, &mut tm_struct) };

        if t.is_null() || tm_struct.tm_zone.is_null() {
            return Ok("".to_string());
        }

        let zone = unsafe {
            CStr::from_ptr(tm_struct.tm_zone)
                .to_string_lossy()
                .into_owned()
        };
        Ok(zone)
    }

    pub fn local_time_offset(&self, _time_ms: f64, _is_utc: bool) -> Result<f64, String> {
        let mut tv: time_t = 0;
        unsafe {
            time(&mut tv);
        }
        let mut tm_struct: tm = unsafe { std::mem::zeroed() };
        let t = unsafe { localtime_r(&tv, &mut tm_struct) };

        if t.is_null() {
            return Err("localtime_r failed".to_string());
        }

        let offset = tm_struct.tm_gmtoff as f64 * 1000.0 -
                     (if tm_struct.tm_isdst > 0 { 3600.0 * 1000.0 } else { 0.0 });

        Ok(offset)
    }
}

const MS_PER_SECOND: f64 = 1000.0;

