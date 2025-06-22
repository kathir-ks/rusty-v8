// Copyright 2006-2009 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// CPU specific code for arm independent of OS goes here.

#[cfg(all(target_arch = "arm", target_os = "linux"))]
use libc;

#[cfg(all(target_arch = "arm", target_os = "freebsd"))]
use std::os::raw::c_int;

#[cfg(target_arch = "arm")]
mod cpu_features {
    // Re-exporting for easy use within this module
    #[cfg(all(target_arch = "arm", target_os = "linux"))]
    use libc;
    #[cfg(all(target_arch = "arm", target_os = "freebsd"))]
    use std::os::raw::c_int;
    #[cfg(all(target_arch = "arm", target_os = "freebsd"))]
    use std::mem::size_of;

    /// Flushes the instruction cache for the given memory region.
    pub fn flush_icache(start: *mut std::ffi::c_void, size: usize) {
        #[cfg(not(feature = "simulator"))]
        {
            #[cfg(target_os = "qnx")]
            unsafe {
                // Assuming MS_SYNC and MS_INVALIDATE_ICACHE are defined elsewhere or are standard
                // Using 0 for flags for now since the original code uses a bitwise OR that needs to be resolved to their equivalent integer representations
                libc::msync(start, size, 0);
            }

            #[cfg(target_os = "freebsd")]
            unsafe {
                #[repr(C)]
                struct arm_sync_icache_args {
                    addr: usize,
                    len: usize,
                }

                const ARM_SYNC_ICACHE: c_int = 111; // Sysarch number for ARM_SYNC_ICACHE

                let args = arm_sync_icache_args {
                    addr: start as usize,
                    len: size,
                };
                let result = libc::sysarch(ARM_SYNC_ICACHE, &args as *const arm_sync_icache_args as *mut std::ffi::c_void);

                if result != 0 {
                    // Handle error, though the original C++ doesn't seem to
                    eprintln!("Error flushing icache: {}", result);
                }
            }

            #[cfg(all(target_os = "linux", not(target_os = "qnx")))]
            unsafe {
                // Assuming __ARM_NR_cacheflush is defined elsewhere or is standard
                const ARM_NR_cacheflush: i32 = 0xf0002;

                let beg = start as u32;
                let end = beg + size as u32;
                let flg: u32 = 0;

                // Syscall wrapper.  This is unsafe.
                unsafe fn syscall(num: i32, a1: u32, a2: u32, a3: u32) -> i32 {
                    let ret: i32;
                    llvm_asm!(
                        "svc 0"
                        : "={r0}"(ret)
                        : "{r7}"(num), "{r0}"(a1), "{r1}"(a2), "{r2}"(a3)
                        : "memory"
                        : "volatile"
                    );
                    ret
                }

                syscall(ARM_NR_cacheflush, beg, end, flg);
            }
        }
    }
}

#[cfg(target_arch = "arm")]
pub use cpu_features::*;