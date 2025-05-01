// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Note: `v8config.h` is assumed to be handled by build configuration or feature flags.

pub mod cppgc {
    pub struct HeapHandle;

    pub mod subtle {
        use super::HeapHandle;

        /// Helpers to peek into heap-internal state.
        pub struct HeapState;

        impl HeapState {
            /// Returns whether the garbage collector is marking. This API is experimental
            /// and is expected to be removed in future.
            ///
            /// `heap_handle` The corresponding heap.
            /// returns true if the garbage collector is currently marking, and false
            ///   otherwise.
            pub fn is_marking(_heap_handle: &HeapHandle) -> bool {
                // This is a stub. Needs implementation details from the original V8 codebase.
                // Accessing internal heap state is not possible without deeper integration.
                false
            }

            /// Returns whether the garbage collector is sweeping. This API is experimental
            /// and is expected to be removed in future.
            ///
            /// `heap_handle` The corresponding heap.
            /// returns true if the garbage collector is currently sweeping, and false
            ///   otherwise.
            pub fn is_sweeping(_heap_handle: &HeapHandle) -> bool {
                // This is a stub. Needs implementation details from the original V8 codebase.
                // Accessing internal heap state is not possible without deeper integration.
                false
            }

            /// Returns whether the garbage collector is currently sweeping on the thread
            /// owning this heap. This API allows the caller to determine whether it has
            /// been called from a destructor of a managed object. This API is experimental
            /// and may be removed in future.
            ///
            /// `heap_handle` The corresponding heap.
            /// returns true if the garbage collector is currently sweeping on this
            ///   thread, and false otherwise.
            pub fn is_sweeping_on_owning_thread(_heap_handle: &HeapHandle) -> bool {
                // This is a stub. Needs implementation details from the original V8 codebase.
                // Accessing internal heap state is not possible without deeper integration.
                false
            }

            /// Returns whether the garbage collector is in the atomic pause, i.e., the
            /// mutator is stopped from running. This API is experimental and is expected
            /// to be removed in future.
            ///
            /// `heap_handle` The corresponding heap.
            /// returns true if the garbage collector is currently in the atomic pause,
            ///   and false otherwise.
            pub fn is_in_atomic_pause(_heap_handle: &HeapHandle) -> bool {
                // This is a stub. Needs implementation details from the original V8 codebase.
                // Accessing internal heap state is not possible without deeper integration.
                false
            }

            /// Returns whether the last garbage collection was finalized conservatively
            /// (i.e., with a non-empty stack). This API is experimental and is expected to
            /// be removed in future.
            ///
            /// `heap_handle` The corresponding heap.
            /// returns true if the last garbage collection was finalized conservatively,
            /// and false otherwise.
            pub fn previous_gc_was_conservative(_heap_handle: &HeapHandle) -> bool {
                // This is a stub. Needs implementation details from the original V8 codebase.
                // Accessing internal heap state is not possible without deeper integration.
                false
            }
        }
    }
}