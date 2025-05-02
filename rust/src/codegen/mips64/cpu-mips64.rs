// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// CPU specific code for arm independent of OS goes here.

#[cfg(target_arch = "mips64")]
pub mod cpu_features {
    use std::arch::asm;
    use std::ffi::c_void;

    /// Provides functionality for flushing the instruction cache (I-Cache).
    pub struct CpuFeatures {}

    impl CpuFeatures {
        /// Flushes the instruction cache for a given memory region.
        ///
        /// # Arguments
        ///
        /// * `start` - A pointer to the starting address of the memory region.
        /// * `size` - The size of the memory region in bytes.
        pub fn flush_i_cache(start: *mut c_void, size: usize) {
            #[cfg(not(feature = "use_simulator"))]
            {
                if size == 0 {
                    return;
                }

                #[cfg(all(target_os = "android", target_pointer_width = "32"))]
                {
                    // Bionic cacheflush can typically run in userland, avoiding kernel call.
                    let end = (start as usize) + size;
                    unsafe {
                        // TODO: Find a proper Rust equivalent for cacheflush
                        // cacheflush(start as i32, end as i32, 0);

                         // Placeholder for cacheflush.  This will need to be
                         // replaced with the appropriate Android cache flushing
                         // mechanism using libc or similar.  The C++ code
                         // used cacheflush which is not directly available in Rust.

                        eprintln!("Warning: cacheflush is not implemented for Android 32-bit. The I-cache might not be flushed.");

                    }
                }
                #[cfg(not(all(target_os = "android", target_pointer_width = "32")))]
                {
                    let res: i64;
                    let icache = 2; //ICACHE defined as 2 on linux syscall.h
                    unsafe {
                        // Using inline assembly to perform the syscall.
                        asm!(
                            "syscall",
                            in("v0") 4137, // __NR_cacheflush = 4137 on mips64 linux
                            in("a0") start as usize,
                            in("a1") size,
                            in("a2") icache,
                            lateout("a3") res,
                            options(nostack, preserves_flags)
                        );
                        if res != 0 {
                            panic!("Failed to flush the instruction cache");
                        }
                    }
                }
            }
        }
    }
}