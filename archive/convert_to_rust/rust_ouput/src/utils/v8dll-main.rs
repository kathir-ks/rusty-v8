// Converted from V8 C++ source files:
// Header: N/A
// Implementation: v8dll-main.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_snake_case)]

#[cfg(target_os = "windows")]
use std::os::windows::raw::HANDLE;

#[cfg(target_os = "windows")]
extern "system" {
    fn GetModuleHandleW(lpModuleName: *const u16) -> HANDLE;
}

#[cfg(target_os = "windows")]
#[no_mangle]
pub extern "system" fn DllMain(
    hinstDLL: HANDLE,
    dwReason: u32,
    lpvReserved: *mut std::ffi::c_void,
) -> i32 {
    // Do nothing.
    1
}
