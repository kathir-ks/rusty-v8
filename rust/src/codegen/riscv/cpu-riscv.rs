// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// CPU specific code for arm independent of OS goes here.

#[cfg(target_os = "linux")]
use std::arch::asm;

pub mod cpu_features {
    /// Flushes the instruction cache for a given memory region.
    ///
    /// This function uses the `riscv_flush_icache` syscall on Linux to
    /// ensure that the CPU's instruction cache is synchronized with the
    /// contents of memory after code has been generated or modified.
    ///
    /// # Arguments
    ///
    /// * `start`: A raw pointer to the beginning of the memory region to flush.
    /// * `size`: The size, in bytes, of the memory region to flush.
    pub fn flush_icache(start: *mut std::ffi::c_void, size: usize) {
        #[cfg(all(target_os = "linux", not(feature = "simulator")))]
        unsafe {
            let end = (start as usize).wrapping_add(size) as *mut std::ffi::c_void;
            // SYS_riscv_flush_icache is a symbolic constant used in user-space code to
            // identify the flush_icache system call, while __NR_riscv_flush_icache is the
            // corresponding system call number used in the kernel to dispatch the system
            // call.
            // The flag set to zero will flush all cpu cores.

            // The specific syscall number is likely to be architecture and kernel version dependent
            // and might not be exposed directly in libc.  Inline assembly is needed for direct syscall access.
            let syscall_number: i64 = 291; //TODO: Confirm this syscall number!  (May need conditional compilation based on kernel version)
            asm!(
                "syscall",
                in("a0") start,
                in("a1") end,
                in("a2") 0, // Flag set to zero will flush all cpu cores.
                in("a7") syscall_number,  // Syscall number for riscv_flush_icache
                options(nostack, preserves_flags)
            );
        }
        #[cfg(not(all(target_os = "linux", not(feature = "simulator"))))]
        {
            // Dummy implementation if not on linux or if the simulator feature is enabled.
            // This prevents compilation errors on other platforms.
            eprintln!("flush_icache is not implemented for this platform or when using the simulator.");
        }
    }
}