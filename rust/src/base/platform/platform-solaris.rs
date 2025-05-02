// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Platform-specific code for Solaris 10 goes here. For the POSIX-compatible
// parts, the implementation is in platform-posix.rs.

#[cfg(target_arch = "sparc")]
compile_error!("V8 does not support the SPARC CPU architecture.");

use libc::{localtime_r, time_t, tzname, tzset, timezone};
use std::os::raw::c_char;
use std::ptr;
use std::time::Duration;
use std::{mem, thread};

use crate::base::platform::platform_posix::PosixTimezoneCache;
use crate::base::platform::platform::TimezoneCache;
use crate::base::macros::*;

pub mod platform {
    pub use super::*;
}

mod macros {
    #[macro_export]
    macro_rules! CHECK {
        ($cond:expr) => {
            if !$cond {
                panic!("Check failed: {}", stringify!($cond));
            }
        };
    }
}

pub mod base {
    pub mod platform {
        pub mod platform {
            use super::super::*;
            pub trait TimezoneCache {
                fn local_timezone(&self, time: f64) -> String;
                fn local_time_offset(&self, time: f64, is_utc: bool) -> f64;
            }

            pub struct SharedLibraryAddress {}

            pub enum Address {}

            pub struct MemoryRange {}
        }
        pub mod platform_posix {
            use super::super::*;

            pub struct PosixTimezoneCache {}
            impl PosixTimezoneCache {
                pub fn new() -> Self {
                    PosixTimezoneCache {}
                }
            }
        }
    }
    pub mod macros;
}

const MS_PER_SECOND: f64 = 1000.0;

struct SolarisTimezoneCache {
    posix_cache: PosixTimezoneCache,
}

impl SolarisTimezoneCache {
    fn new() -> Self {
        SolarisTimezoneCache {
            posix_cache: PosixTimezoneCache::new(),
        }
    }
}

impl TimezoneCache for SolarisTimezoneCache {
    fn local_timezone(&self, time: f64) -> String {
        if time.is_nan() {
            return "".to_string();
        }
        let tv = time.floor() / MS_PER_SECOND;
        let tv = tv as time_t;
        let mut tm: libc::tm = unsafe { mem::zeroed() };

        let t = unsafe { localtime_r(&tv, &mut tm) };
        if t.is_null() {
            return "".to_string();
        }

        let tz = unsafe { tzname[0] };
        let c_str = unsafe { std::ffi::CStr::from_ptr(tz) };
        c_str.to_str().unwrap().to_string()
    }

    fn local_time_offset(&self, time: f64, is_utc: bool) -> f64 {
        unsafe { tzset() };
        unsafe { -(timezone as f64) * MS_PER_SECOND }
    }
}

pub struct OS {}

impl OS {
    pub fn create_timezone_cache() -> Box<dyn TimezoneCache> {
        Box::new(SolarisTimezoneCache::new())
    }

    pub fn get_shared_library_addresses() -> Vec<base::platform::platform::SharedLibraryAddress> {
        Vec::new()
    }

    pub fn signal_code_moving_gc() {}

    pub fn adjust_scheduling_params() {}

    pub fn get_first_free_memory_range_within(
        boundary_start: *mut libc::c_void,
        boundary_end: *mut libc::c_void,
        minimum_size: usize,
        alignment: usize,
    ) -> Option<base::platform::platform::MemoryRange> {
        None
    }
}

pub struct Stack {}

impl Stack {
    pub fn obtain_current_thread_stack_start() -> *mut libc::c_void {
        let mut attr: libc::pthread_attr_t = unsafe { mem::zeroed() };
        let error = unsafe { libc::pthread_attr_init(&mut attr) };
        if error != 0 {
            return ptr::null_mut();
        }
        let thread = unsafe { libc::pthread_self() };
        let error = unsafe { libc::pthread_getattr_np(thread, &mut attr) };
        if error != 0 {
            unsafe { libc::pthread_attr_destroy(&mut attr) };
            return ptr::null_mut();
        }

        let mut base: *mut libc::c_void = ptr::null_mut();
        let mut size: libc::size_t = 0;

        let error = unsafe { libc::pthread_attr_getstack(&mut attr, &mut base, &mut size) };
        CHECK!(error == 0);

        unsafe { libc::pthread_attr_destroy(&mut attr) };
        unsafe { base.add(size) }
    }
}