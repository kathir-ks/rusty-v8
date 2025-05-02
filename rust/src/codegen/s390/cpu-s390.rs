// Copyright 2015 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// CPU specific code for s390 independent of OS goes here.
#[cfg(target_arch = "s390x")]
pub mod cpu_features {
    /// Provides CPU feature detection and cache management for the s390x architecture.
    pub struct CpuFeatures {}

    impl CpuFeatures {
        /// Flushes the instruction cache for the given buffer.
        ///
        /// On z/Architecture, due to the strong memory model and the single-threaded nature of V8,
        /// instruction cache flushing is not necessary. The architecture guarantees that if a core
        /// patches its own instruction cache, the updated instructions will be reflected automatically.
        pub fn flush_icache(_buffer: *mut std::ffi::c_void, _size: usize) {
            // No-op on s390x because of the strong memory model.
        }
    }
}