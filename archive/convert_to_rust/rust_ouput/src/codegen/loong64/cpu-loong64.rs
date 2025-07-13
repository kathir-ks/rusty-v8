// Converted from V8 C++ source files:
// Header: N/A
// Implementation: cpu-loong64.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::mem;

pub struct CpuFeatures {}

impl CpuFeatures {
    pub fn FlushICache(start: *mut std::ffi::c_void, size: usize) {
        #[cfg(all(
            target_arch = "loongarch64",
            target_os = "android",
            target_pointer_width = "32"
        ))]
        unsafe {
            let end = (start as usize + size) as *mut std::ffi::c_void;
            cacheflush(
                start as usize as i32,
                end as usize as i32,
                0,
            );
        }
        #[cfg(target_arch = "loongarch64")]
        #[cfg(not(all(
            target_arch = "loongarch64",
            target_os = "android",
            target_pointer_width = "32"
        )))]
        unsafe {
            core::arch::asm!("ibar 0");
        }
    }
}

extern "C" {
    fn cacheflush(start: i32, end: i32, flags: i32);
}
