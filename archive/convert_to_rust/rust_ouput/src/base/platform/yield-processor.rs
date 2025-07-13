// Converted from V8 C++ source files:
// Header: yield-processor.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub use std::time::Duration;
#[cfg(target_thread_local)]
use std::thread;

#[cfg(target_thread_local)]
fn yield_processor() {
    // Mimic the C++ logic based on target architecture and compiler.
    // This is a simplified implementation.  A real implementation
    // would need to consider more factors like compiler and architecture.
    #[cfg(target_arch = "x86_64")]
    {
        unsafe {
            std::arch::x86_64::_mm_pause();
        }
    }
    #[cfg(target_arch = "x86")]
    {
        unsafe {
            std::arch::x86_64::_mm_pause();
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        unsafe {
           std::arch::aarch64::__yield();
        }
    }
    #[cfg(all(target_arch = "arm", __ARM_ARCH >= 6))]
    {
        unsafe {
            std::arch::aarch64::__yield();
        }
    }
    #[cfg(all(target_arch = "mips64el", __mips_isa_rev >= 2))]
    {
        // Not directly supported, using a fallback.
        std::thread::yield_now();
    }
    #[cfg(target_arch = "powerpc64")]
    {
        // No direct equivalent.  Fallback to yield.
        std::thread::yield_now();
    }
    #[cfg(target_os = "android")]
    {
        //If thread sanitizer is enabled, sleep for 1 ms
        thread::sleep(Duration::from_millis(1));
    }
}

#[cfg(not(target_thread_local))]
fn yield_processor() {}
