// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// Opaque handle used for additional heap APIs.
pub struct HeapHandle {
    is_incremental_marking_in_progress: bool,
    is_young_generation_enabled: bool,
}

impl HeapHandle {
    /// Creates a new `HeapHandle`.  This is private and should only
    /// be called by internal cppgc components.
    fn new() -> Self {
        HeapHandle {
            is_incremental_marking_in_progress: false,
            is_young_generation_enabled: false,
        }
    }

    #[inline]
    pub fn is_incremental_marking_in_progress(&self) -> bool {
        self.is_incremental_marking_in_progress
    }

    #[inline]
    pub fn is_young_generation_enabled(&self) -> bool {
        self.is_young_generation_enabled
    }

    // These functions are intentionally private and can only be set by
    // internal cppgc components.
    fn set_incremental_marking_in_progress(&mut self, value: bool) {
        self.is_incremental_marking_in_progress = value;
    }

    fn set_young_generation_enabled(&mut self, value: bool) {
        self.is_young_generation_enabled = value;
    }
}

mod internal {
    pub(crate) use super::HeapHandle;

    pub struct HeapBase {}

    impl HeapBase {
        // Empty impl to mimic c++ HeapBase definition
    }

    pub struct WriteBarrierTypeForCagedHeapPolicy {}

    impl WriteBarrierTypeForCagedHeapPolicy {
        // Empty impl to mimic c++ WriteBarrierTypeForCagedHeapPolicy definition
    }

    pub struct WriteBarrierTypeForNonCagedHeapPolicy {}

    impl WriteBarrierTypeForNonCagedHeapPolicy {
        // Empty impl to mimic c++ WriteBarrierTypeForNonCagedHeapPolicy definition
    }
}