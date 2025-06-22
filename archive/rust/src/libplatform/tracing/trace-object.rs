// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::os::raw::{c_char, c_int, c_uchar, c_uint, c_ulonglong, c_void};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{mem, ptr, slice, str};

// Placeholder for v8::ConvertableToTraceFormat (cannot be directly translated)
// It represents a trait or abstract class in C++
trait ConvertableToTraceFormat {}

// Define constants
const TRACE_VALUE_TYPE_CONVERTABLE: u8 = 3;
const TRACE_VALUE_TYPE_STRING: u8 = 4;
const TRACE_VALUE_TYPE_COPY_STRING: u8 = 5;
const TRACE_EVENT_FLAG_COPY: u32 = 1;
const K_TRACE_MAX_NUM_ARGS: usize = 2; // Example Value, replace with actual value if known

// Get current process ID (platform-specific)
#[cfg(target_os = "linux")]
fn get_current_process_id() -> i32 {
    unsafe { libc::getpid() }
}

#[cfg(target_os = "macos")]
fn get_current_process_id() -> i32 {
    unsafe { libc::getpid() }
}

#[cfg(target_os = "windows")]
fn get_current_process_id() -> i32 {
    unsafe { kernel32::GetCurrentProcessId() as i32 }
}

#[cfg(target_os = "windows")]
extern "system" {
    fn kernel32::GetCurrentProcessId() -> u32;
}

// Get current thread ID (platform-specific)
#[cfg(target_os = "linux")]
fn get_current_thread_id() -> i32 {
    unsafe { libc::syscall(libc::SYS_gettid) as i32 }
}

#[cfg(target_os = "macos")]
fn get_current_thread_id() -> i32 {
    unsafe { libc::pthread_mach_thread_np(libc::pthread_self()) as i32 }
}

#[cfg(target_os = "windows")]
fn get_current_thread_id() -> i32 {
    unsafe { kernel32::GetCurrentThreadId() as i32 }
}

#[cfg(target_os = "windows")]
extern "system" {
    fn kernel32::GetCurrentThreadId() -> u32;
}

#[cfg(all(target_os = "macos", feature = "use_mach"))]
extern "C" {
    fn pthread_mach_thread_np(thread: libc::pthread_t) -> libc::mach_port_t;
}

#[derive(Clone, Copy)]
#[repr(C)]
union ArgValue {
    as_uint: u64,
    as_string: *const c_char,
}

/// Represents a trace object.
pub struct TraceObject {
    pid_: i32,
    tid_: i32,
    phase_: c_char,
    category_enabled_flag_: *const u8,
    name_: *const c_char,
    scope_: *const c_char,
    id_: u64,
    bind_id_: u64,
    flags_: u32,
    ts_: i64,
    tts_: i64,
    duration_: i64,
    cpu_duration_: i64,
    num_args_: usize,
    arg_names_: [*const c_char; K_TRACE_MAX_NUM_ARGS],
    arg_values_: [ArgValue; K_TRACE_MAX_NUM_ARGS],
    arg_types_: [u8; K_TRACE_MAX_NUM_ARGS],
    arg_convertables_: [Option<Box<dyn ConvertableToTraceFormat>>; K_TRACE_MAX_NUM_ARGS],
    parameter_copy_storage_: *mut c_char,
}

impl TraceObject {
    /// Creates a new `TraceObject`.
    pub fn new() -> Self {
        TraceObject {
            pid_: 0,
            tid_: 0,
            phase_: 0,
            category_enabled_flag_: ptr::null(),
            name_: ptr::null(),
            scope_: ptr::null(),
            id_: 0,
            bind_id_: 0,
            flags_: 0,
            ts_: 0,
            tts_: 0,
            duration_: 0,
            cpu_duration_: 0,
            num_args_: 0,
            arg_names_: [ptr::null(); K_TRACE_MAX_NUM_ARGS],
            arg_values_: [ArgValue { as_uint: 0 }; K_TRACE_MAX_NUM_ARGS],
            arg_types_: [0; K_TRACE_MAX_NUM_ARGS],
            arg_convertables_: [None; K_TRACE_MAX_NUM_ARGS],
            parameter_copy_storage_: ptr::null_mut(),
        }
    }

    /// Initializes the `TraceObject` with the given parameters.
    pub fn initialize(
        &mut self,
        phase: c_char,
        category_enabled_flag: *const u8,
        name: *const c_char,
        scope: *const c_char,
        id: u64,
        bind_id: u64,
        num_args: c_int,
        arg_names: &[*const c_char],
        arg_types: &[u8],
        arg_values: &[u64],
        arg_convertables: &mut [Option<Box<dyn ConvertableToTraceFormat>>],
        flags: c_uint,
        timestamp: i64,
        cpu_timestamp: i64,
    ) {
        self.pid_ = get_current_process_id();
        self.tid_ = get_current_thread_id();
        self.phase_ = phase;
        self.category_enabled_flag_ = category_enabled_flag;
        self.name_ = name;
        self.scope_ = scope;
        self.id_ = id;
        self.bind_id_ = bind_id;
        self.flags_ = flags;
        self.ts_ = timestamp;
        self.tts_ = cpu_timestamp;
        self.duration_ = 0;
        self.cpu_duration_ = 0;

        let num_args = num_args as usize;

        // Clamp num_args
        self.num_args_ = if num_args > K_TRACE_MAX_NUM_ARGS {
            K_TRACE_MAX_NUM_ARGS
        } else {
            num_args
        };

        for i in 0..self.num_args_ {
            self.arg_names_[i] = arg_names[i];
            self.arg_values_[i].as_uint = arg_values[i];
            self.arg_types_[i] = arg_types[i];
            if arg_types[i] == TRACE_VALUE_TYPE_CONVERTABLE {
                self.arg_convertables_[i] = arg_convertables[i].take();
            } else {
                self.arg_convertables_[i] = None;
            }
        }

        let copy = (flags & TRACE_EVENT_FLAG_COPY) != 0;

        let mut alloc_size: usize = 0;
        if copy {
            alloc_size += get_alloc_length(self.name_);
            alloc_size += get_alloc_length(self.scope_);
            for i in 0..self.num_args_ {
                alloc_size += get_alloc_length(self.arg_names_[i]);
                if self.arg_types_[i] == TRACE_VALUE_TYPE_STRING {
                    self.arg_types_[i] = TRACE_VALUE_TYPE_COPY_STRING;
                }
            }
        }

        let mut arg_is_copy = [false; K_TRACE_MAX_NUM_ARGS];
        for i in 0..self.num_args_ {
            arg_is_copy[i] = self.arg_types_[i] == TRACE_VALUE_TYPE_COPY_STRING;
            if arg_is_copy[i] {
                unsafe {
                    alloc_size += get_alloc_length(self.arg_values_[i].as_string);
                }
            }
        }

        if alloc_size > 0 {
            // Since TraceObject can be initialized multiple times, we might need
            // to free old memory.
            if !self.parameter_copy_storage_.is_null() {
                unsafe {
                    drop(Vec::from_raw_parts(
                        self.parameter_copy_storage_ as *mut u8,
                        0,
                        alloc_size,
                    ));
                }
            }

            let mut buffer = Vec::with_capacity(alloc_size);
            unsafe {
                buffer.set_len(alloc_size);
            }
            let mut ptr = buffer.as_mut_ptr() as *mut c_char;
            self.parameter_copy_storage_ = ptr;

            if copy {
                copy_trace_object_parameter(&mut ptr, &mut self.name_);
                copy_trace_object_parameter(&mut ptr, &mut self.scope_);
                for i in 0..self.num_args_ {
                    copy_trace_object_parameter(&mut ptr, &mut self.arg_names_[i]);
                }
            }

            for i in 0..self.num_args_ {
                if arg_is_copy[i] {
                    unsafe {
                        copy_trace_object_parameter(&mut ptr, &mut self.arg_values_[i].as_string);
                    }
                }
            }

            mem::forget(buffer);
        }
    }

    /// Updates the duration of the trace object.
    pub fn update_duration(&mut self, timestamp: i64, cpu_timestamp: i64) {
        self.duration_ = timestamp - self.ts_;
        self.cpu_duration_ = cpu_timestamp - self.tts_;
    }

    /// Initializes the `TraceObject` for testing purposes.
    pub fn initialize_for_testing(
        &mut self,
        phase: c_char,
        category_enabled_flag: *const u8,
        name: *const c_char,
        scope: *const c_char,
        id: u64,
        bind_id: u64,
        num_args: c_int,
        arg_names: &[*const c_char],
        arg_types: &[u8],
        arg_values: &[u64],
        arg_convertables: &mut [Option<Box<dyn ConvertableToTraceFormat>>],
        flags: c_uint,
        pid: i32,
        tid: i32,
        ts: i64,
        tts: i64,
        duration: u64,
        cpu_duration: u64,
    ) {
        self.pid_ = pid;
        self.tid_ = tid;
        self.phase_ = phase;
        self.category_enabled_flag_ = category_enabled_flag;
        self.name_ = name;
        self.scope_ = scope;
        self.id_ = id;
        self.bind_id_ = bind_id;
        self.num_args_ = num_args as usize;
        self.flags_ = flags;
        self.ts_ = ts;
        self.tts_ = tts;
        self.duration_ = duration as i64;
        self.cpu_duration_ = cpu_duration as i64;
    }
}

impl Drop for TraceObject {
    fn drop(&mut self) {
        if !self.parameter_copy_storage_.is_null() {
            unsafe {
                libc::free(self.parameter_copy_storage_ as *mut c_void);
            }
        }
    }
}

/// Calculates the allocation length for a C-style string.
fn get_alloc_length(str: *const c_char) -> usize {
    if str.is_null() {
        0
    } else {
        unsafe { strlen(str) + 1 }
    }
}

extern "C" {
    fn strlen(s: *const c_char) -> usize;
}

/// Copies a trace object parameter.
fn copy_trace_object_parameter(buffer: &mut *mut c_char, member: &mut *const c_char) {
    if member.is_null() {
        return;
    }
    let length = unsafe { strlen(*member) + 1 };
    unsafe {
        libc::memcpy(*buffer as *mut c_void, *member as *const c_void, length);
        *member = *buffer;
        *buffer = (*buffer).add(length);
    }
}