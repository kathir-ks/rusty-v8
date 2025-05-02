pub mod testing {
    use crate::heap::cppgc::internal;
    use crate::heap::cppgc::internal::HeapBase;
    use crate::heap::cppgc::EmbedderStackState;
    use crate::heap::cppgc::HeapHandle;
    use crate::heap::cppgc::internal::HeapObjectHeader;

    /// Scope to override the embedder stack state for testing.
    pub struct OverrideEmbedderStackStateScope<'a> {
        heap_handle_: &'a HeapHandle,
    }

    impl<'a> OverrideEmbedderStackStateScope<'a> {
        /// Constructor. Overrides the stack state.
        pub fn new(heap_handle: &'a HeapHandle, state: EmbedderStackState) -> Self {
            HeapBase::from(heap_handle).set_override_stack_state(state);
            Self { heap_handle_: heap_handle }
        }
    }

    impl<'a> Drop for OverrideEmbedderStackStateScope<'a> {
        /// Destructor. Clears the overridden stack state.
        fn drop(&mut self) {
            HeapBase::from(self.heap_handle_).clear_overridden_stack_state();
        }
    }

    /// A standalone heap for testing purposes.
    pub struct StandaloneTestingHeap<'a> {
        heap_handle_: &'a HeapHandle,
    }

    impl<'a> StandaloneTestingHeap<'a> {
        /// Constructor.
        pub fn new(heap_handle: &'a HeapHandle) -> Self {
            Self { heap_handle_: heap_handle }
        }

        /// Starts a garbage collection cycle.
        pub fn start_garbage_collection(&self) {
            HeapBase::from(self.heap_handle_)
                .start_incremental_garbage_collection_for_testing();
        }

        /// Performs a marking step of garbage collection.
        pub fn perform_marking_step(&self, stack_state: EmbedderStackState) -> bool {
            HeapBase::from(self.heap_handle_)
                .marker()
                .incremental_marking_step_for_testing(stack_state)
        }

        /// Finalizes a garbage collection cycle.
        pub fn finalize_garbage_collection(&self, stack_state: EmbedderStackState) {
            HeapBase::from(self.heap_handle_)
                .finalize_incremental_garbage_collection_for_testing(stack_state);
        }

        /// Toggles whether main thread marking is disabled.
        pub fn toggle_main_thread_marking(&self, should_mark: bool) {
            HeapBase::from(self.heap_handle_)
                .marker()
                .set_main_thread_marking_disabled_for_testing(!should_mark);
        }

        /// Forces compaction for the next garbage collection.
        pub fn force_compaction_for_next_garbage_collection(&self) {
            HeapBase::from(self.heap_handle_)
                .compactor()
                .enable_for_next_gc_for_testing();
        }
    }

    /// Determines if a heap object is old.
    pub fn is_heap_object_old(object: *mut std::ffi::c_void) -> bool {
        #[cfg(feature = "cppgc_young_generation")]
        {
            unsafe { HeapObjectHeader::from_object(object).is_marked() }
        }
        #[cfg(not(feature = "cppgc_young_generation"))]
        {
            true
        }
    }
}