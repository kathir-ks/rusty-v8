// Converted from V8 C++ source files:
// Header: N/A
// Implementation: cpu-mips64.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

use std::os::raw::c_void;

#[cfg(target_arch = "mips64")]
mod cpu_features {
    use std::arch::asm;
    use std::io::Error;
    use std::mem;
    use std::os::raw::c_void;

    pub struct CpuFeatures {}

    impl CpuFeatures {
        pub fn flush_icache(start: *mut c_void, size: usize) -> Result<(), Error> {
            if size == 0 {
                return Ok(());
            }
            unsafe {
                // syscall(__NR_cacheflush, start, size, ICACHE);
                let result: i64;
                asm!(
                    "syscall",
                    in("a0") start,
                    in("a1") size,
                    in("a2") 3, //ICACHE,
                    in("v0") 4125, // __NR_cacheflush
                    lateout("v0") result,
                );
                if result != 0 {
                    Err(Error::last_os_error())
                } else {
                    Ok(())
                }
            }
        }
    }
}
