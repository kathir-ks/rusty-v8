// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod tracing_flags {
    use std::sync::atomic::{AtomicU32, Ordering};

    /// A struct to hold tracing flags.  Mimics the C++ class of the same name.
    pub struct TracingFlags {}

    impl TracingFlags {
        /// Runtime statistics tracing flag.
        pub static runtime_stats: AtomicU32 = AtomicU32::new(0);

        /// Garbage collection tracing flag.
        pub static gc: AtomicU32 = AtomicU32::new(0);

        /// Garbage collection statistics tracing flag.
        pub static gc_stats: AtomicU32 = AtomicU32::new(0);

        /// Inline cache statistics tracing flag.
        pub static ic_stats: AtomicU32 = AtomicU32::new(0);

        /// Zone statistics tracing flag.
        pub static zone_stats: AtomicU32 = AtomicU32::new(0);
    }
}