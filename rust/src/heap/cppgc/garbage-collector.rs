// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod garbage_collector {
    use std::option::Option;

    /// GC configuration.  Defined elsewhere.  Using a dummy definition here.
    #[derive(Clone, Copy)]
    pub struct GCConfig {}

    /// Embedder stack state. Defined elsewhere.  Using a dummy definition here.
    #[derive(Clone, Copy)]
    pub enum EmbedderStackState {}

    /// GC interface that allows abstraction over the actual GC invocation. This is
    /// needed to mock/fake GC for testing.
    pub trait GarbageCollector {
        /// Executes a garbage collection specified in config.
        fn collect_garbage(&mut self, config: GCConfig);

        /// Starts an incremental garbage collection specified in config.
        fn start_incremental_garbage_collection(&mut self, config: GCConfig);

        /// The current epoch that the GC maintains. The epoch is increased on every
        /// GC invocation.
        fn epoch(&self) -> usize;

        /// Returns if the stack state is overridden.
        fn overridden_stack_state(&self) -> Option<EmbedderStackState>;

        /// These methods are also present in class HeapBase.
        fn set_override_stack_state(&mut self, state: EmbedderStackState);
        fn clear_overridden_stack_state(&mut self);

        #[cfg(feature = "allocation_timeout")]
        /// Returns value for Allocation Timeout.
        fn update_allocation_timeout(&mut self) -> Option<i32>;
    }
}