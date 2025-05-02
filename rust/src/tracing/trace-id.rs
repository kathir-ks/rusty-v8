// Copyright 2025 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod tracing {
    use std::sync::atomic::{AtomicU64, Ordering};

    /// Generates a unique trace ID.
    #[inline]
    pub fn trace_id() -> u64 {
        static SEQUENCE_NUMBER: AtomicU64 = AtomicU64::new(0);
        SEQUENCE_NUMBER.fetch_add(1, Ordering::Relaxed)
    }
} // namespace tracing