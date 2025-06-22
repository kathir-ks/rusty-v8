// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// CPU specific code for x64 independent of OS goes here.

#[cfg(target_arch = "x86_64")]
mod cpu_features {
    // The cfg attribute makes sure this module is only compiled for x86_64

    // NOTE: Valgrind related code is conditionally compiled and needs external
    // valgrind libraries. We will emulate them with empty calls, as we don't want
    // to depend on it to make the code compilable.
    #[cfg(not(feature = "valgrind"))]
    mod valgrind {
        pub fn discard_translations(_start: *mut std::ffi::c_void, _size: usize) -> u32 {
            0 // Dummy implementation
        }
    }

    #[cfg(feature = "valgrind")]
    mod valgrind {
        // NOTE: This section requires external valgrind libraries.
        // You'd typically have extern crate declarations, like
        // extern crate valgrind; and calls to valgrind functions.
        // For the purpose of the problem, we're using dummy implementation

        pub fn discard_translations(_start: *mut std::ffi::c_void, _size: usize) -> u32 {
            0 // Dummy implementation
        }

        // In reality this should be a declaration to the valgrind library
        // extern {
        //    fn VALGRIND_DISCARD_TRANSLATIONS(start: *mut std::ffi::c_void, size: usize) -> u32;
        //}

    }

    /// Provides functions to query CPU features.
    pub struct CpuFeatures {}

    impl CpuFeatures {
        /// Flushes the instruction cache for the given memory region.
        ///
        /// On Intel, instruction cache flushing is only necessary when multiple cores
        /// are running the same code simultaneously. V8 (and JavaScript) is single
        /// threaded, and when code is patched on an Intel CPU, the core performing
        /// the patching will have its own instruction cache updated automatically.
        ///
        /// If flushing of the instruction cache becomes necessary, Windows has the
        /// API function `FlushInstructionCache`.
        pub fn flush_icache(start: *mut std::ffi::c_void, size: usize) {
            // No need to flush the instruction cache on Intel.

            // By default, valgrind only checks the stack for writes that might need to
            // invalidate already cached translated code.  This leads to random
            // instability when code patches or moves are sometimes unnoticed.  One
            // solution is to run valgrind with --smc-check=all, but this comes at a big
            // performance cost.  We can notify valgrind to invalidate its cache.

            let res = valgrind::discard_translations(start, size);
            let _ = res; //Suppress unused variable warning. USE(res) in C++ translates to this.
        }
    }
}

#[cfg(target_arch = "x86_64")]
pub use cpu_features::CpuFeatures;