// Converted from V8 C++ source files:
// Header: N/A
// Implementation: cpu-arm.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_camel_case_types)]

#[cfg(target_arch = "arm")]
mod arm_cpu {
    use std::mem::size_of;
    use std::os::raw::c_void;

    #[cfg(target_os = "qnx")]
    extern "C" {
        fn msync(addr: *mut c_void, len: usize, flags: i32) -> i32;
    }

    #[cfg(target_os = "freebsd")]
    extern "C" {
        fn sysarch(op: i32, parms: *mut c_void) -> i32;
    }

    #[cfg(not(any(target_os = "qnx", target_os = "freebsd", target_os = "starboard")))]
    extern "C" {
        fn syscall(num: i64, ...) -> i64;
    }

    pub struct CpuFeatures {}

    impl CpuFeatures {
        pub fn flush_icache(start: *mut c_void, size: usize) {
            #[cfg(not(feature = "simulator"))]
            {
                #[cfg(target_os = "qnx")]
                unsafe {
                    const MS_SYNC: i32 = 0x00001000;
                    const MS_INVALIDATE_ICACHE: i32 = 0x20000000;
                    msync(start, size, MS_SYNC | MS_INVALIDATE_ICACHE);
                }

                #[cfg(target_os = "freebsd")]
                unsafe {
                    #[repr(C)]
                    struct arm_sync_icache_args {
                        addr: usize,
                        len: usize,
                    }

                    const ARM_SYNC_ICACHE: i32 = 113;
                    let args = arm_sync_icache_args {
                        addr: start as usize,
                        len: size,
                    };
                    sysarch(ARM_SYNC_ICACHE, &args as *const _ as *mut c_void);
                }

                #[cfg(not(any(target_os = "qnx", target_os = "freebsd", target_os = "starboard")))]
                unsafe {
                    const __ARM_NR_cacheflush: i64 = 0x0f0002;
                    syscall(
                        __ARM_NR_cacheflush,
                        start as i64,
                        (start as usize + size) as i64,
                        0,
                    );
                }
            }
        }
    }
}
