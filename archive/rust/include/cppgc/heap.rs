// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod common;
pub mod platform;

/// cppgc - A C++ garbage collection library.
pub mod cppgc {
    use std::borrow::BorrowMut;
    use std::rc::Rc;
    use std::{any::Any, vec};

    pub use crate::common::EmbedderStackState as StackState;
    pub use crate::platform::Platform;

    pub trait CustomSpaceBase: Any {
        fn as_any(&self) -> &dyn Any;
        fn as_any_mut(&mut self) -> &mut dyn Any;
    }

    pub struct AllocationHandle {}
    pub struct HeapHandle {}

    mod internal {
        pub struct Heap {}
    }

    /// Specifies whether conservative stack scanning is supported.
    #[derive(Clone, Copy)]
    pub enum StackSupport {
        /// Conservative stack scan is supported.
        SupportsConservativeStackScan,
        /// Conservative stack scan is not supported. Embedders may use this option
        /// when using custom infrastructure that is unsupported by the library.
        NoConservativeStackScan,
    }

    /// Specifies supported marking types.
    #[derive(Clone, Copy)]
    pub enum MarkingType {
        /// Atomic stop-the-world marking. This option does not require any write
        /// barriers but is the most intrusive in terms of jank.
        Atomic,
        /// Incremental marking interleaves marking with the rest of the application
        /// workload on the same thread.
        Incremental,
        /// Incremental and concurrent marking.
        IncrementalAndConcurrent,
    }

    /// Specifies supported sweeping types.
    #[derive(Clone, Copy)]
    pub enum SweepingType {
        /// Atomic stop-the-world sweeping. All of sweeping is performed at once.
        Atomic,
        /// Incremental sweeping interleaves sweeping with the rest of the
        /// application workload on the same thread.
        Incremental,
        /// Incremental and concurrent sweeping. Sweeping is split and interleaved
        /// with the rest of the application.
        IncrementalAndConcurrent,
    }

    /// Constraints for a Heap setup.
    #[derive(Default)]
    pub struct ResourceConstraints {
        /// Allows the heap to grow to some initial size in bytes before triggering
        /// garbage collections. This is useful when it is known that applications
        /// need a certain minimum heap to run to avoid repeatedly invoking the
        /// garbage collector when growing the heap.
        pub initial_heap_size_bytes: usize,
    }

    /// Options specifying Heap properties (e.g. custom spaces) when initializing a
    /// heap through `Heap::Create()`.
    #[derive(Default)]
    pub struct HeapOptions {
        /// Custom spaces added to heap are required to have indices forming a
        /// numbered sequence starting at 0, i.e., their `kSpaceIndex` must
        /// correspond to the index they reside in the vector.
        pub custom_spaces: Vec<Box<dyn CustomSpaceBase>>,

        /// Specifies whether conservative stack scan is supported. When conservative
        /// stack scan is not supported, the collector may try to invoke
        /// garbage collections using non-nestable task, which are guaranteed to have
        /// no interesting stack, through the provided Platform. If such tasks are
        /// not supported by the Platform, the embedder must take care of invoking
        /// the GC through `ForceGarbageCollectionSlow()`.
        pub stack_support: StackSupport,

        /// Specifies which types of marking are supported by the heap.
        pub marking_support: MarkingType,

        /// Specifies which types of sweeping are supported by the heap.
        pub sweeping_support: SweepingType,

        /// Resource constraints specifying various properties that the internal
        /// GC scheduler follows.
        pub resource_constraints: ResourceConstraints,
    }

    impl HeapOptions {
        /// Creates reasonable defaults for instantiating a Heap.
        ///
        /// \returns the HeapOptions that can be passed to `Heap::Create()`.
        pub fn default() -> Self {
            HeapOptions {
                stack_support: StackSupport::SupportsConservativeStackScan,
                marking_support: MarkingType::IncrementalAndConcurrent,
                sweeping_support: SweepingType::IncrementalAndConcurrent,
                ..Default::default()
            }
        }
    }

    pub struct Heap {
        allocation_handle: AllocationHandle,
        heap_handle: HeapHandle,
    }

    impl Heap {
        /// Creates a new heap that can be used for object allocation.
        ///
        /// \param platform implemented and provided by the embedder.
        /// \param options HeapOptions specifying various properties for the Heap.
        /// \returns a new Heap instance.
        pub fn create(platform: Rc<dyn Platform>, options: HeapOptions) -> Box<Heap> {
            Box::new(Heap {
                allocation_handle: AllocationHandle {},
                heap_handle: HeapHandle {},
            })
        }

        /// Forces garbage collection.
        ///
        /// \param source String specifying the source (or caller) triggering a
        ///   forced garbage collection.
        /// \param reason String specifying the reason for the forced garbage
        ///   collection.
        /// \param stack_state The embedder stack state, see StackState.
        pub fn force_garbage_collection_slow(
            &mut self,
            source: &str,
            reason: &str,
            stack_state: StackState,
        ) {
            // Implementation of garbage collection is not provided.
            println!(
                "Forcing garbage collection: source={}, reason={}, stack_state={:?}",
                source, reason, stack_state
            );
        }

        /// \returns the opaque handle for allocating objects using
        /// `MakeGarbageCollected()`.
        pub fn get_allocation_handle(&mut self) -> &AllocationHandle {
            &self.allocation_handle
        }

        /// \returns the opaque heap handle which may be used to refer to this heap in
        ///   other APIs. Valid as long as the underlying `Heap` is alive.
        pub fn get_heap_handle(&mut self) -> &HeapHandle {
            &self.heap_handle
        }
    }
}