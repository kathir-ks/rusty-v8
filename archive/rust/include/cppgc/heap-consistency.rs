// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod heap_consistency {
    use std::marker::PhantomData;

    pub struct HeapHandle {}

    pub mod subtle {
        use super::HeapHandle;
        // TODO: Import or define TraceCallback
        // type TraceCallback = ...;

        pub struct WriteBarrierParams {}

        #[derive(PartialEq, Eq)]
        pub enum WriteBarrierType {
            NoBarrier,
            // Placeholder for other barrier types
            OtherBarrier,
        }

        pub struct HeapConsistency {}

        impl HeapConsistency {
            pub fn get_write_barrier_type(
                slot: *const std::ffi::c_void,
                value: *const std::ffi::c_void,
                params: &mut WriteBarrierParams,
            ) -> WriteBarrierType {
                internal::write_barrier::get_write_barrier_type(slot, value, params)
            }

            pub fn get_write_barrier_type_basic_member<T, WeaknessTag, WriteBarrierPolicy, CheckingPolicy, StorageType>(
                value: &internal::BasicMember<T, WeaknessTag, WriteBarrierPolicy, CheckingPolicy, StorageType>,
                params: &mut WriteBarrierParams,
            ) -> WriteBarrierType {
                internal::write_barrier::get_write_barrier_type(
                    value.get_raw_slot(),
                    value.get_raw_storage(),
                    params,
                )
            }

            pub fn get_write_barrier_type_with_callback<HeapHandleCallback>(
                slot: *const std::ffi::c_void,
                params: &mut WriteBarrierParams,
                callback: HeapHandleCallback,
            ) -> WriteBarrierType
            where
                HeapHandleCallback: FnOnce() -> HeapHandle,
            {
                internal::write_barrier::get_write_barrier_type_with_callback(slot, params, callback)
            }

            pub fn get_write_barrier_type_no_slot(
                value: *const std::ffi::c_void,
                params: &mut WriteBarrierParams,
            ) -> WriteBarrierType {
                internal::write_barrier::get_write_barrier_type_no_slot(value, params)
            }

            pub fn dijkstra_write_barrier(params: &WriteBarrierParams, object: *const std::ffi::c_void) {
                internal::write_barrier::dijkstra_marking_barrier(params, object);
            }

            // pub fn dijkstra_write_barrier_range(
            //     params: &WriteBarrierParams,
            //     first_element: *const std::ffi::c_void,
            //     element_size: usize,
            //     number_of_elements: usize,
            //     trace_callback: TraceCallback,
            // ) {
            //     internal::write_barrier::dijkstra_marking_barrier_range(
            //         params,
            //         first_element,
            //         element_size,
            //         number_of_elements,
            //         trace_callback,
            //     );
            // }

            pub fn steele_write_barrier(params: &WriteBarrierParams, object: *const std::ffi::c_void) {
                internal::write_barrier::steele_marking_barrier(params, object);
            }

            pub fn generational_barrier(params: &WriteBarrierParams, slot: *const std::ffi::c_void) {
                internal::write_barrier::generational_barrier(
                    params,
                    slot,
                    internal::write_barrier::GenerationalBarrierType::PreciseSlot,
                );
            }

            pub fn generational_barrier_for_uncompressed_slot(
                params: &WriteBarrierParams,
                uncompressed_slot: *const std::ffi::c_void,
            ) {
                internal::write_barrier::generational_barrier(
                    params,
                    uncompressed_slot,
                    internal::write_barrier::GenerationalBarrierType::PreciseUncompressedSlot,
                );
            }

            pub fn generational_barrier_for_source_object(
                params: &WriteBarrierParams,
                inner_pointer: *const std::ffi::c_void,
            ) {
                internal::write_barrier::generational_barrier(
                    params,
                    inner_pointer,
                    internal::write_barrier::GenerationalBarrierType::ImpreciseSlot,
                );
            }
        }

        /// Disallows garbage collection finalizations. Any garbage collection triggers
        /// result in a crash when in this scope.
        ///
        /// Note that the garbage collector already covers paths that can lead to garbage
        /// collections, so user code does not require checking
        /// `is_garbage_collection_allowed()` before allocations.
        pub struct DisallowGarbageCollectionScope<'a> {
            heap_handle_: &'a HeapHandle,
        }

        impl<'a> DisallowGarbageCollectionScope<'a> {
            /// \returns whether garbage collections are currently allowed.
            pub fn is_garbage_collection_allowed(heap_handle: &mut HeapHandle) -> bool {
                internal::disallow_gc_scope::is_garbage_collection_allowed(heap_handle)
            }

            /// Enters a disallow garbage collection scope. Must be paired with `leave()`.
            /// Prefer a scope instance of `DisallowGarbageCollectionScope`.
            ///
            /// \param heap_handle The corresponding heap.
            pub fn enter(heap_handle: &mut HeapHandle) {
                internal::disallow_gc_scope::enter(heap_handle);
            }

            /// Leaves a disallow garbage collection scope. Must be paired with `enter()`.
            /// Prefer a scope instance of `DisallowGarbageCollectionScope`.
            ///
            /// \param heap_handle The corresponding heap.
            pub fn leave(heap_handle: &mut HeapHandle) {
                internal::disallow_gc_scope::leave(heap_handle);
            }

            /// Constructs a scoped object that automatically enters and leaves a disallow
            /// garbage collection scope based on its lifetime.
            ///
            /// \param heap_handle The corresponding heap.
            pub fn new(heap_handle: &'a HeapHandle) -> Self {
                internal::disallow_gc_scope::enter(heap_handle);
                DisallowGarbageCollectionScope {
                    heap_handle_: heap_handle,
                }
            }
        }

        impl<'a> Drop for DisallowGarbageCollectionScope<'a> {
            fn drop(&mut self) {
                internal::disallow_gc_scope::leave(self.heap_handle_);
            }
        }

        /// Avoids invoking garbage collection finalizations. Already running garbage
        /// collection phase are unaffected by this scope.
        ///
        /// Should only be used temporarily as the scope has an impact on memory usage
        /// and follow up garbage collections.
        pub struct NoGarbageCollectionScope<'a> {
            heap_handle_: &'a HeapHandle,
        }

        impl<'a> NoGarbageCollectionScope<'a> {
            /// Enters a no garbage collection scope. Must be paired with `leave()`. Prefer
            /// a scope instance of `NoGarbageCollectionScope`.
            ///
            /// \param heap_handle The corresponding heap.
            pub fn enter(heap_handle: &mut HeapHandle) {
                internal::no_gc_scope::enter(heap_handle);
            }

            /// Leaves a no garbage collection scope. Must be paired with `enter()`. Prefer
            /// a scope instance of `NoGarbageCollectionScope`.
            ///
            /// \param heap_handle The corresponding heap.
            pub fn leave(heap_handle: &mut HeapHandle) {
                internal::no_gc_scope::leave(heap_handle);
            }

            /// Constructs a scoped object that automatically enters and leaves a no
            /// garbage collection scope based on its lifetime.
            ///
            /// \param heap_handle The corresponding heap.
            pub fn new(heap_handle: &'a HeapHandle) -> Self {
                internal::no_gc_scope::enter(heap_handle);
                NoGarbageCollectionScope {
                    heap_handle_: heap_handle,
                }
            }
        }

        impl<'a> Drop for NoGarbageCollectionScope<'a> {
            fn drop(&mut self) {
                internal::no_gc_scope::leave(self.heap_handle_);
            }
        }
    }

    mod internal {
        pub mod write_barrier {
            use super::super::{HeapHandle, WriteBarrierParams, WriteBarrierType};

            pub fn get_write_barrier_type(
                _slot: *const std::ffi::c_void,
                _value: *const std::ffi::c_void,
                _params: &mut WriteBarrierParams,
            ) -> WriteBarrierType {
                WriteBarrierType::NoBarrier // Placeholder implementation
            }

            pub fn get_write_barrier_type_with_callback<HeapHandleCallback>(
                _slot: *const std::ffi::c_void,
                _params: &mut WriteBarrierParams,
                _callback: HeapHandleCallback,
            ) -> WriteBarrierType
            where
                HeapHandleCallback: FnOnce() -> HeapHandle,
            {
                WriteBarrierType::NoBarrier // Placeholder implementation
            }

            pub fn get_write_barrier_type_no_slot(
                _value: *const std::ffi::c_void,
                _params: &mut WriteBarrierParams,
            ) -> WriteBarrierType {
                WriteBarrierType::NoBarrier // Placeholder implementation
            }

            pub fn dijkstra_marking_barrier(_params: &WriteBarrierParams, _object: *const std::ffi::c_void) {} // Placeholder

            // pub fn dijkstra_marking_barrier_range(
            //     _params: &WriteBarrierParams,
            //     _first_element: *const std::ffi::c_void,
            //     _element_size: usize,
            //     _number_of_elements: usize,
            //     _trace_callback: TraceCallback,
            // ) {
            // }

            pub fn steele_marking_barrier(_params: &WriteBarrierParams, _object: *const std::ffi::c_void) {} // Placeholder

            pub enum GenerationalBarrierType {
                PreciseSlot,
                PreciseUncompressedSlot,
                ImpreciseSlot,
            }

            pub fn generational_barrier(
                _params: &WriteBarrierParams,
                _slot: *const std::ffi::c_void,
                _barrier_type: GenerationalBarrierType,
            ) {
            }
        }

        pub mod disallow_gc_scope {
            use super::super::HeapHandle;

            pub fn is_garbage_collection_allowed(_heap_handle: &mut HeapHandle) -> bool {
                true // Placeholder implementation
            }

            pub fn enter(_heap_handle: &HeapHandle) {} // Placeholder implementation

            pub fn leave(_heap_handle: &HeapHandle) {} // Placeholder implementation
        }

        pub mod no_gc_scope {
            use super::super::HeapHandle;

            pub fn enter(_heap_handle: &mut HeapHandle) {} // Placeholder implementation

            pub fn leave(_heap_handle: &mut HeapHandle) {} // Placeholder implementation
        }

        pub struct BasicMember<T, WeaknessTag, WriteBarrierPolicy, CheckingPolicy, StorageType> {
            storage: StorageType,
            _phantom_t: PhantomData<T>,
            _phantom_weakness_tag: PhantomData<WeaknessTag>,
            _phantom_write_barrier_policy: PhantomData<WriteBarrierPolicy>,
            _phantom_checking_policy: PhantomData<CheckingPolicy>,
        }

        impl<T, WeaknessTag, WriteBarrierPolicy, CheckingPolicy, StorageType>
            BasicMember<T, WeaknessTag, WriteBarrierPolicy, CheckingPolicy, StorageType>
        {
            pub fn get_raw_slot(&self) -> *const std::ffi::c_void {
                &self.storage as *const _ as *const std::ffi::c_void
            }

            pub fn get_raw_storage(&self) -> *const std::ffi::c_void {
                &self.storage as *const _ as *const std::ffi::c_void
            }
        }
    }
}