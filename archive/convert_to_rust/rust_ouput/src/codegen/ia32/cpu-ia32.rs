// Converted from V8 C++ source files:
// Header: N/A
// Implementation: cpu-ia32.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
// CPU specific code for ia32 independent of OS goes here.

#[cfg(target_arch = "x86")]
mod cpu_ia32 {
    use std::mem;
    //use super::*;
    //use crate::codegen::cpu_features;
    //use crate::init::v8;
    //use crate::base::logging;
    //use crate::base::bits;

    pub struct CpuFeatures {}

    impl CpuFeatures {
        pub fn FlushICache(start: *mut std::ffi::c_void, size: usize) {
            // No need to flush the instruction cache on Intel. On Intel instruction
            // cache flushing is only necessary when multiple cores running the same
            // code simultaneously. V8 (and JavaScript) is single threaded and when code
            // is patched on an intel CPU the core performing the patching will have its
            // own instruction cache updated automatically.

            // If flushing of the instruction cache becomes necessary Windows has the
            // API function FlushInstructionCache.

            // By default, valgrind only checks the stack for writes that might need to
            // invalidate already cached translated code.  This leads to random
            // instability when code patches or moves are sometimes unnoticed.  One
            // solution is to run valgrind with --smc-check=all, but this comes at a big
            // performance cost.  We can notify valgrind to invalidate its cache.
            #[cfg(feature = "valgrind")]
            unsafe {
                let res = libc::syscall(libc::SYS_membarrier, libc::MEMBARRIER_CMD_PRIVATE_EXPEDITED, 0);
                if res != 0 {
                    eprintln!("VALGRIND_DISCARD_TRANSLATIONS failed with error: {}", res);
                }
                let _ = start;
                let _ = size;
            }
        }
    }
}
#[cfg(target_arch = "x86")]
pub use cpu_ia32::CpuFeatures;
#[cfg(not(target_arch = "x86"))]
pub struct CpuFeatures {}

#[cfg(not(target_arch = "x86"))]
impl CpuFeatures {
    pub fn FlushICache(start: *mut std::ffi::c_void, size: usize) {
                let _ = start;
                let _ = size;
    }
}
