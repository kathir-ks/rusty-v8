pub mod cppgc {
    pub mod subtle {
        use crate::cppgc::internal::HeapBase;
        use crate::cppgc::HeapHandle;
        use std::ops::{Deref, DerefMut};

        /// RAII helper to disallow garbage collections for the duration of the
        /// object's lifetime.
        pub struct DisallowGarbageCollectionScope<'a> {
            heap_handle_: &'a HeapHandle,
        }

        impl<'a> DisallowGarbageCollectionScope<'a> {
            pub fn new(heap_handle: &'a HeapHandle) -> Self {
                Self::enter(heap_handle);
                DisallowGarbageCollectionScope {
                    heap_handle_: heap_handle,
                }
            }

            /// static
            pub fn is_garbage_collection_allowed(heap_handle: &HeapHandle) -> bool {
                let heap_base = HeapBase::from(heap_handle);
                !heap_base.is_gc_forbidden()
            }

            /// static
            pub fn enter(heap_handle: &HeapHandle) {
                let mut heap_base = HeapBase::from(heap_handle);
                heap_base.enter_disallow_gc_scope();
            }

            /// static
            pub fn leave(heap_handle: &HeapHandle) {
                let mut heap_base = HeapBase::from(heap_handle);
                heap_base.leave_disallow_gc_scope();
            }
        }

        impl<'a> Drop for DisallowGarbageCollectionScope<'a> {
            fn drop(&mut self) {
                Self::leave(self.heap_handle_);
            }
        }

        /// RAII helper to completely disable garbage collections for the duration of
        /// the object's lifetime. This can be useful for benchmarking purposes or
        /// when consistency is paramount.
        pub struct NoGarbageCollectionScope<'a> {
            heap_handle_: &'a HeapHandle,
        }

        impl<'a> NoGarbageCollectionScope<'a> {
            pub fn new(heap_handle: &'a HeapHandle) -> Self {
                Self::enter(heap_handle);
                NoGarbageCollectionScope {
                    heap_handle_: heap_handle,
                }
            }

            /// static
            pub fn enter(heap_handle: &HeapHandle) {
                let mut heap_base = HeapBase::from(heap_handle);
                heap_base.enter_no_gc_scope();
            }

            /// static
            pub fn leave(heap_handle: &HeapHandle) {
                let mut heap_base = HeapBase::from(heap_handle);
                heap_base.leave_no_gc_scope();
            }
        }

        impl<'a> Drop for NoGarbageCollectionScope<'a> {
            fn drop(&mut self) {
                Self::leave(self.heap_handle_);
            }
        }
    }

    // Dummy definition for HeapHandle as it's not provided in the original code.
    #[derive(Debug)]
    pub struct HeapHandle {}

    pub mod internal {
        use crate::cppgc::HeapHandle;

        #[derive(Debug)]
        pub struct HeapBase {}

        impl HeapBase {
            pub fn from(_heap_handle: &HeapHandle) -> HeapBase {
                HeapBase {}
            }

            pub fn is_gc_forbidden(&self) -> bool {
                false // Dummy implementation
            }

            pub fn enter_disallow_gc_scope(&mut self) {
                // Dummy implementation
            }

            pub fn leave_disallow_gc_scope(&mut self) {
                // Dummy implementation
            }

            pub fn enter_no_gc_scope(&mut self) {
                // Dummy implementation
            }

            pub fn leave_no_gc_scope(&mut self) {
                // Dummy implementation
            }
        }
    }
}