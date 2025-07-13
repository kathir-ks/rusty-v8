// Converted from V8 C++ source files:
// Header: N/A
// Implementation: cpu-ppc.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#[cfg(target_arch = "powerpc64")]
mod cpu_ppc {
    pub struct CpuFeatures {}

    impl CpuFeatures {
        pub fn FlushICache(buffer: *mut std::ffi::c_void, size: usize) {
            #[cfg(not(feature = "simulator"))]
            unsafe {
                llvm_asm!(
                    "sync",
                    "icbi 0, $0",
                    "isync",
                    in("$0") buffer,
                    options(nostack, nomem, preserves_flags)
                );
            }
        }
    }
}
