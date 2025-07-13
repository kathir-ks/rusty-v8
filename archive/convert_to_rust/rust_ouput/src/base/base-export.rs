// Converted from V8 C++ source files:
// Header: base-export.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#[cfg(target_os = "windows")]
mod windows {
    #[cfg(feature = "building_v8_base_shared")]
    #[macro_export]
    macro_rules! V8_BASE_EXPORT {
        () => {
            #[link(name = "kernel32")]
            extern "system" {
                pub fn GetModuleHandleW(lpModuleName: *const u16) -> *mut std::ffi::c_void;
            }
            #[link(name = "kernel32")]
            extern "system" {
                pub fn GetProcAddress(hModule: *mut std::ffi::c_void, lpProcName: *const i8) -> *mut std::ffi::c_void;
            }
            #[link(name = "kernel32")]
            extern "system" {
                pub fn FreeLibrary(hLibModule: *mut std::ffi::c_void) -> i32;
            }
            #[link(name = "kernel32")]
            extern "system" {
                pub fn LoadLibraryW(lpFileName: *const u16) -> *mut std::ffi::c_void;
            }
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            type __declspec = ();
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            type dllexport = ();
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            type dllimport = ();
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            type V8_BASE_EXPORT = ();
        };
    }

    #[cfg(all(not(feature = "building_v8_base_shared"), feature = "using_v8_base_shared"))]
    #[macro_export]
    macro_rules! V8_BASE_EXPORT {
        () => {
            #[link(name = "kernel32")]
            extern "system" {
                pub fn GetModuleHandleW(lpModuleName: *const u16) -> *mut std::ffi::c_void;
            }
            #[link(name = "kernel32")]
            extern "system" {
                pub fn GetProcAddress(hModule: *mut std::ffi::c_void, lpProcName: *const i8) -> *mut std::ffi::c_void;
            }
            #[link(name = "kernel32")]
            extern "system" {
                pub fn FreeLibrary(hLibModule: *mut std::ffi::c_void) -> i32;
            }
            #[link(name = "kernel32")]
            extern "system" {
                pub fn LoadLibraryW(lpFileName: *const u16) -> *mut std::ffi::c_void;
            }
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            type __declspec = ();
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            type dllexport = ();
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            type dllimport = ();
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            type V8_BASE_EXPORT = ();
        };
    }

    #[cfg(not(any(feature = "building_v8_base_shared", feature = "using_v8_base_shared")))]
    #[macro_export]
    macro_rules! V8_BASE_EXPORT {
        () => {};
    }
}

#[cfg(target_os = "linux")]
mod linux {
    #[cfg(feature = "building_v8_base_shared")]
    #[macro_export]
    macro_rules! V8_BASE_EXPORT {
        () => {
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            type __attribute__ = ();
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            type visibility = ();
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            type V8_BASE_EXPORT = ();
        };
    }

    #[cfg(not(feature = "building_v8_base_shared"))]
    #[macro_export]
    macro_rules! V8_BASE_EXPORT {
        () => {};
    }
}

#[cfg(target_os = "windows")]
pub use windows::V8_BASE_EXPORT;

#[cfg(target_os = "linux")]
pub use linux::V8_BASE_EXPORT;

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
#[macro_export]
macro_rules! V8_BASE_EXPORT {
    () => {};
}
