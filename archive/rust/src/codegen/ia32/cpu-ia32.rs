// Copyright 2011 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// CPU specific code for ia32 independent of OS goes here.

#[cfg(target_arch = "x86")]
mod cpu_ia32 {
    //use std::arch::x86::*; // Consider enabling specific x86 features if needed
    //use std::mem; //Consider using for manipulating memory directly
    //use std::ptr;

    /// Provides CPU feature detection and utilities.
    pub struct CpuFeatures {}

    impl CpuFeatures {
        /// Flushes the instruction cache.
        ///
        /// This function currently does nothing on Intel architectures, as instruction
        /// cache flushing is generally not required for single-threaded code patching.
        ///
        /// # Arguments
        ///
        /// * `start` - A pointer to the start of the memory region to flush.
        /// * `size` - The size of the memory region to flush, in bytes.
        pub fn flush_icache(start: *mut std::ffi::c_void, size: usize) {
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
            // The below valgrind code is conditionally included based on the presence of `valgrind` and the `VALGRIND_DISCARD_TRANSLATIONS` macro.
            // Since a direct Rust equivalent is not readily available (and valgrind is an external tool), we omit the functionality
            // and leave a comment to acknowledge the omitted section.

            // #ifdef VALGRIND_DISCARD_TRANSLATIONS
            // unsigned res = VALGRIND_DISCARD_TRANSLATIONS(start, size);
            // USE(res);
            // #endif
            #[cfg(feature = "valgrind")]
            {
                extern "C" {
                    fn VALGRIND_DISCARD_TRANSLATIONS(addr: *mut std::ffi::c_void, len: usize) -> std::os::raw::c_uint;
                }
                unsafe {
                    let _res = VALGRIND_DISCARD_TRANSLATIONS(start, size);
                    //placeholder
                }

            }
        }
    }
}
#[cfg(target_arch = "x86")]
pub use cpu_ia32::*;