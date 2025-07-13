// Converted from V8 C++ source files:
// Header: win32-headers.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file contains defines and typedefs that allow popular Windows types to
// be used without the overhead of including windows.h.
// This file no longer includes windows.h but it still sets the defines that
// tell windows.h to omit some includes so that the V8 source files that do
// include windows.h will still get the minimal version.

// These defines are handled by build config files.
// #[cfg(not(WIN32_LEAN_AND_MEAN))]
// const WIN32_LEAN_AND_MEAN: () = ();
// #[cfg(not(NOMINMAX))]
// const NOMINMAX: () = ();
// #[cfg(not(NOKERNEL))]
// const NOKERNEL: () = ();
// #[cfg(not(NOUSER))]
// const NOUSER: () = ();
// #[cfg(not(NOSERVICE))]
// const NOSERVICE: () = ();
// #[cfg(not(NOSOUND))]
// const NOSOUND: () = ();
// #[cfg(not(NOMCX))]
// const NOMCX: () = ();
// #[cfg(not(_WIN32_WINNT))]
// compile_error!("This should be set in build config files. See build\\config\\win\\BUILD.gn");

use std::os::raw::c_long;
use std::os::raw::c_ulong;
use std::os::raw::c_void;

pub type BOOL = i32;
pub type DWORD = c_ulong;
pub type LONG = c_long;
pub type LPVOID = *mut c_void;
pub type PVOID = *mut c_void;
pub type HANDLE = *mut c_void;

pub type ULONG_PTR = usize;
pub type PULONG_PTR = *mut ULONG_PTR;

#[repr(C)]
pub struct SRWLOCK {
    ptr: *mut c_void, // Placeholder, actual implementation depends on OS
}

#[repr(C)]
pub struct CONDITION_VARIABLE {
    ptr: *mut c_void, // Placeholder, actual implementation depends on OS
}

#[repr(C)]
pub struct CRITICAL_SECTION {
    debug_info: *mut CRITICAL_SECTION_DEBUG,
    lock_count: LONG,
    recursion_count: LONG,
    owning_thread: HANDLE,
    lock_semaphore: HANDLE,
    spin_count: ULONG_PTR,
}

#[repr(C)]
pub struct CRITICAL_SECTION_DEBUG {
    // Placeholder, actual implementation depends on OS
    // This should contain debug info of the critical section
    // For example, possible fields: Type, CheckSum, etc.
    // In this case, keep empty to minimize the dependency
}

pub type PRTL_CRITICAL_SECTION_DEBUG = *mut CRITICAL_SECTION_DEBUG;

#[allow(non_snake_case)]
pub extern "stdcall" fn AcquireSRWLockExclusive(srwlock: *mut SRWLOCK) {
    unsafe {
        std::sync::Mutex::new(()).lock().unwrap(); // Simulate exclusive lock
    }
}

#[allow(non_snake_case)]
pub extern "stdcall" fn ReleaseSRWLockExclusive(srwlock: *mut SRWLOCK) {
    // Release the simulated lock
}

#[allow(non_snake_case)]
pub extern "stdcall" fn AcquireSRWLockShared(srwlock: *mut SRWLOCK) {
    unsafe {
       std::sync::RwLock::new(()).read().unwrap(); // Simulate shared lock
    }
}

#[allow(non_snake_case)]
pub extern "stdcall" fn ReleaseSRWLockShared(srwlock: *mut SRWLOCK) {
    // Release the simulated lock
}

#[allow(non_snake_case)]
pub extern "system" fn InitializeConditionVariable(condition_variable: *mut CONDITION_VARIABLE) {
    // No-op implementation.  The memory is expected to be zeroed before calling.
}

#[allow(non_snake_case)]
pub extern "system" fn WakeConditionVariable(condition_variable: *mut CONDITION_VARIABLE) {
    // No-op implementation
}

#[allow(non_snake_case)]
pub extern "system" fn SleepConditionVariableSRW(
    condition_variable: *mut CONDITION_VARIABLE,
    srwlock: *mut SRWLOCK,
    timeout: DWORD,
    flags: DWORD,
) -> BOOL {
    // No-op implementation
    1 // Return non-zero to indicate success.
}

#[allow(non_snake_case)]
pub extern "system" fn InitializeCriticalSection(critical_section: *mut CRITICAL_SECTION) -> BOOL {
    // Initialize fields of CRITICAL_SECTION
    unsafe {
        (*critical_section).debug_info = std::ptr::null_mut();
        (*critical_section).lock_count = -1;
        (*critical_section).recursion_count = 0;
        (*critical_section).owning_thread = std::ptr::null_mut();
        (*critical_section).lock_semaphore = std::ptr::null_mut();
        (*critical_section).spin_count = 0;
    }
    1 // Success
}

#[allow(non_snake_case)]
pub extern "system" fn EnterCriticalSection(critical_section: *mut CRITICAL_SECTION) {
    unsafe {
        // Simulate locking behavior.  Use a std::sync::Mutex internally.
        static mut MUTEX: Option<std::sync::Mutex<()>> = None;
        let mutex = MUTEX.get_or_insert_with(|| std::sync::Mutex::new(()));
        mutex.lock().unwrap();

        (*critical_section).lock_count = 0; // Update lock count (simulation)
    }
}

#[allow(non_snake_case)]
pub extern "system" fn LeaveCriticalSection(critical_section: *mut CRITICAL_SECTION) {
    unsafe {
        // Simulate unlocking behavior.  Release the Mutex.
        static mut MUTEX: Option<std::sync::Mutex<()>> = None;
        let mutex = MUTEX.get_or_insert_with(|| std::sync::Mutex::new(()));
        mutex.unlock().unwrap();

        (*critical_section).lock_count = -1; // Update lock count (simulation)
    }
}

#[allow(non_snake_case)]
pub extern "system" fn DeleteCriticalSection(critical_section: *mut CRITICAL_SECTION) {
    // This function does nothing. The resources used by a
    // critical section are released when the process terminates.
    // https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-deletecriticalsection
}

#[allow(non_snake_case)]
pub type WINAPI = extern "stdcall";

#[repr(C)]
pub struct V8_SRWLOCK {
    pub ptr: PVOID,
}

#[repr(C)]
pub struct V8_CONDITION_VARIABLE {
    pub ptr: PVOID,
}

#[repr(C)]
pub struct V8_CRITICAL_SECTION {
    pub debug_info: PRTL_CRITICAL_SECTION_DEBUG,
    pub lock_count: LONG,
    pub recursion_count: LONG,
    pub owning_thread: HANDLE,
    pub lock_semaphore: HANDLE,
    pub spin_count: ULONG_PTR,
}

#[inline]
pub fn v8_to_windows_type(p: *mut V8_SRWLOCK) -> *mut SRWLOCK {
    p as *mut SRWLOCK
}

#[inline]
pub fn v8_to_windows_type_const(p: *const V8_SRWLOCK) -> *const SRWLOCK {
    p as *const SRWLOCK
}

#[inline]
pub fn v8_to_windows_type_condition_variable(p: *mut V8_CONDITION_VARIABLE) -> *mut CONDITION_VARIABLE {
    p as *mut CONDITION_VARIABLE
}

#[inline]
pub fn v8_to_windows_type_condition_variable_const(
    p: *const V8_CONDITION_VARIABLE,
) -> *const CONDITION_VARIABLE {
    p as *const CONDITION_VARIABLE
}

#[inline]
pub fn v8_to_windows_type_critical_section(p: *mut V8_CRITICAL_SECTION) -> *mut CRITICAL_SECTION {
    p as *mut CRITICAL_SECTION
}

#[inline]
pub fn v8_to_windows_type_critical_section_const(
    p: *const V8_CRITICAL_SECTION,
) -> *const CRITICAL_SECTION {
    p as *const CRITICAL_SECTION
}
