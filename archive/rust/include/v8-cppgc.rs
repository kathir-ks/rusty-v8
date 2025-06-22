// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod cppgc {
    pub mod common {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum EmbedderStackState {
            NoHeapStack,
            MayContainHeapPointers,
        }
    }

    pub mod custom_space {
        use std::any::Any;

        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub struct CustomSpaceIndex(pub usize);

        pub trait CustomSpaceBase : Any {
            fn as_any(&self) -> &dyn Any;
        }
    }

    pub mod heap_statistics {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum DetailLevel {
            Brief,
            Detailed,
        }

        #[derive(Debug, Default, Clone)]
        pub struct HeapStatistics {
            pub total_heap_size: usize,
            pub total_heap_capacity: usize,
            pub external_memory: usize,
            pub detail_level: DetailLevel,
        }
    }

    pub mod visitor {
        pub struct Visitor {
            key: Key,
        }

        impl Visitor {
            pub fn new(key: Key) -> Self {
                Visitor { key }
            }

            pub fn visit(&mut self, _obj: &dyn std::any::Any) {} // Dummy visit method
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct Key(pub usize);
    }

    pub mod internal {
        //Dummy struct
        pub struct CppHeap {}
    }

    pub mod alloc {
        pub struct AllocationHandle {}
    }

    pub mod heap {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum MarkingType {
            kIncrementalAndConcurrent,
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum SweepingType {
            kIncrementalAndConcurrent,
        }

        pub struct Heap {}
    }

    pub use common::EmbedderStackState;
    pub use custom_space::CustomSpaceBase;
    pub use custom_space::CustomSpaceIndex;
    pub use heap_statistics::DetailLevel;
    pub use heap_statistics::HeapStatistics;
    pub use visitor::Key;
    pub use visitor::Visitor;
    pub use alloc::AllocationHandle;
    pub use heap::Heap;
    pub use heap::MarkingType;
    pub use heap::SweepingType;

    pub struct HeapHandle {}
}

pub mod v8 {
    use std::mem::MaybeUninit;
    use std::ptr::NonNull;

    pub use crate::cppgc;
    use std::any::Any;

    pub trait Platform {}

    pub struct Isolate {}

    pub struct Object {}

    pub struct TracedReference<T> {
        inner: MaybeUninit<NonNull<T>>,
    }

    impl<T> TracedReference<T> {
        pub fn new() -> Self {
            TracedReference {
                inner: MaybeUninit::uninit()
            }
        }
        pub fn is_empty_thread_safe(&self) -> bool {
            unsafe { self.inner.assume_init().is_null() }
        }
    }

    pub trait TracedReferenceBase {
        fn is_empty_thread_safe(&self) -> bool;
    }

    impl<T> TracedReferenceBase for TracedReference<T> {
        fn is_empty_thread_safe(&self) -> bool {
            self.is_empty_thread_safe()
        }
    }

    pub struct CppHeapCreateParams {
        pub custom_spaces: Vec<Box<dyn cppgc::CustomSpaceBase>>,
        pub marking_support: cppgc::Heap::MarkingType,
        pub sweeping_support: cppgc::Heap::SweepingType,
    }

    impl CppHeapCreateParams {
        pub fn new(custom_spaces: Vec<Box<dyn cppgc::CustomSpaceBase>>) -> Self {
            CppHeapCreateParams {
                custom_spaces,
                marking_support: cppgc::Heap::MarkingType::kIncrementalAndConcurrent,
                sweeping_support: cppgc::Heap::SweepingType::kIncrementalAndConcurrent,
            }
        }
    }

    pub trait CustomSpaceStatisticsReceiver {
        fn allocated_bytes(&mut self, space_index: cppgc::CustomSpaceIndex, bytes: usize);
    }

    pub struct CppHeap {
        // Fields would go here, but they're private in the C++ version.
        allocation_handle: cppgc::AllocationHandle,
        heap_handle: cppgc::HeapHandle,
    }

    impl CppHeap {
        pub fn create(_platform: &dyn Platform, params: &CppHeapCreateParams) -> Box<CppHeap> {
            // Implementation would go here to instantiate a CppHeap.
            // This is just a placeholder.
            Box::new(CppHeap {
                allocation_handle: cppgc::AllocationHandle {},
                heap_handle: cppgc::HeapHandle {},
            })
        }

        pub fn get_allocation_handle(&self) -> &cppgc::AllocationHandle {
            &self.allocation_handle
        }

        pub fn get_heap_handle(&self) -> &cppgc::HeapHandle {
            &self.heap_handle
        }

        #[deprecated(note = "Terminate gets automatically called in the CppHeap destructor")]
        pub fn terminate(&self) {
            // No-op in Rust, as destructors handle this automatically.
        }

        pub fn collect_statistics(&self, detail_level: cppgc::DetailLevel) -> cppgc::HeapStatistics {
            // Placeholder implementation.
            cppgc::HeapStatistics {
                total_heap_size: 0,
                total_heap_capacity: 0,
                external_memory: 0,
                detail_level,
            }
        }

        pub fn collect_custom_space_statistics_at_last_gc(
            &self,
            custom_spaces: Vec<cppgc::CustomSpaceIndex>,
            mut receiver: Box<dyn CustomSpaceStatisticsReceiver>,
        ) {
            for &space_index in &custom_spaces {
                receiver.allocated_bytes(space_index, 0); // Placeholder value.
            }
        }

        pub fn enable_detached_garbage_collections_for_testing(&mut self) {
            // Implementation would go here.
        }

        pub fn collect_garbage_for_testing(&self, _stack_state: cppgc::EmbedderStackState) {
            // Implementation would go here.
        }

        pub fn collect_garbage_in_young_generation_for_testing(
            &self,
            _stack_state: cppgc::EmbedderStackState,
        ) {
            // Implementation would go here.
        }
    }

    pub struct JSVisitor {
        visitor: cppgc::Visitor,
    }

    impl JSVisitor {
        pub fn new(key: cppgc::Key) -> Self {
            JSVisitor { visitor: cppgc::Visitor::new(key) }
        }

        pub fn trace<T>(&mut self, ref_: &TracedReference<T>) {
            if ref_.is_empty_thread_safe() {
                return;
            }
            self.visit(ref_);
        }

        fn visit<T>(&mut self, _ref: &TracedReference<T>) {}
    }

}

pub mod internal {
    pub use crate::cppgc;
}

mod trace_trait_impl {
    use crate::cppgc;
    use crate::v8;
    use std::any::Any;

    pub struct TraceTrait;

    impl TraceTrait {
        pub fn get_trace_descriptor<T>(_self: *const v8::TracedReference<T>) -> cppgc::visitor::Key {
            cppgc::visitor::Key(0)
        }

        pub fn trace<T>(visitor: &mut cppgc::Visitor, self_: *const v8::TracedReference<T>) {
            // Dummy implementation
            visitor.visit(unsafe { &*self_ as &dyn Any });
        }
    }
}