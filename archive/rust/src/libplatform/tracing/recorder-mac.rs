// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file provides a macOS-specific implementation of the Recorder class for tracing.

use core::ffi::c_char;
use core::ffi::c_int;
use std::ffi::CString;

#[cfg(target_os = "macos")]
mod os_log {
    use core::ffi::c_void;

    #[repr(C)]
    pub struct os_log_t {
        _private: [u8; 0],
    }

    #[repr(C)]
    pub enum os_log_type_t {
        OS_LOG_TYPE_DEFAULT = 0,
        OS_LOG_TYPE_INFO = 16,
        OS_LOG_TYPE_DEBUG = 32,
        OS_LOG_TYPE_ERROR = 64,
        OS_LOG_TYPE_FAULT = 80,
    }

    extern "C" {
        pub fn os_log_create(subsystem: *const c_char, category: *const c_char) -> *mut os_log_t;
        pub fn os_log_type_enabled(log: *mut os_log_t, type_: os_log_type_t) -> bool;
        pub fn os_signpost_event_emit(
            log: *mut os_log_t,
            signpost_id: u64,
            name: *const c_char,
            format: *const c_char,
            ...
        );
    }
}

pub mod tracing {
    use super::*;

    #[cfg(target_os = "macos")]
    use os_log::*;

    // Define a trait for TraceObject, mirroring the C++ TraceObject interface.
    pub trait TraceObject {
        fn name(&self) -> &str;
        fn cpu_duration(&self) -> i64;
    }

    pub struct Recorder {
        #[cfg(target_os = "macos")]
        v8_provider: *mut os_log_t,
    }

    impl Recorder {
        pub fn new() -> Recorder {
            #[cfg(target_os = "macos")]
            {
                let subsystem = CString::new("v8").unwrap();
                let category = CString::new("").unwrap();
                let v8_provider = unsafe { os_log_create(subsystem.as_ptr(), category.as_ptr()) };
                Recorder { v8_provider }
            }
            #[cfg(not(target_os = "macos"))]
            Recorder {}
        }

        pub fn is_enabled(&self) -> bool {
            #[cfg(target_os = "macos")]
            unsafe {
                os_log_type_enabled(self.v8_provider, OS_LOG_TYPE_DEFAULT)
            }
            #[cfg(not(target_os = "macos"))]
            false
        }

        pub fn is_enabled_with_level(&self, level: u8) -> bool {
            #[cfg(target_os = "macos")]
            {
                use os_log::os_log_type_t::*;
                if level == OS_LOG_TYPE_DEFAULT as u8
                    || level == OS_LOG_TYPE_INFO as u8
                    || level == OS_LOG_TYPE_DEBUG as u8
                    || level == OS_LOG_TYPE_ERROR as u8
                    || level == OS_LOG_TYPE_FAULT as u8
                {
                    unsafe {
                        os_log_type_enabled(
                            self.v8_provider,
                            std::mem::transmute::<u8, os_log_type_t>(level),
                        )
                    }
                } else {
                    false
                }
            }
            #[cfg(not(target_os = "macos"))]
            false
        }

        pub fn add_event(&self, trace_event: &dyn TraceObject) {
            #[cfg(target_os = "macos")]
            {
                let name = CString::new(trace_event.name()).unwrap();
                let format = CString::new("%s, cpu_duration: %d").unwrap();
                unsafe {
                    os_signpost_event_emit(
                        self.v8_provider,
                        0, // OS_SIGNPOST_ID_EXCLUSIVE
                        name.as_ptr(),
                        format.as_ptr(),
                        name.as_ptr(),
                        trace_event.cpu_duration() as c_int,
                    );
                }
            }
            #[cfg(not(target_os = "macos"))]
            {
                // Placeholder for non-macOS platforms
                println!(
                    "Trace event: {}, cpu_duration: {}",
                    trace_event.name(),
                    trace_event.cpu_duration()
                );
            }
        }
    }
}