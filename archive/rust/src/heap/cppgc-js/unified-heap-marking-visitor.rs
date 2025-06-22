// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod unified_heap_marking_visitor {
    use std::ptr::NonNull;

    pub use crate::heap::cppgc::marking_visitor::*;
    pub use crate::heap::cppgc_js::unified_heap_marking_state::*;

    pub use crate::base::macros::*;

    pub use crate::cppgc;
    pub use crate::v8;
    pub use crate::v8::internal::heap::cppgc::marking_state::*;
    pub use crate::v8::internal::heap::cppgc::marking_visitor::JSVisitor;

    pub type TraceDescriptor = cppgc::TraceDescriptor;
    pub type TraceDescriptorCallback = cppgc::TraceDescriptorCallback;
    pub type WeakCallback = cppgc::WeakCallback;
    pub type HeapBase = cppgc::internal::HeapBase;
    pub type MutatorMarkingState = cppgc::internal::MutatorMarkingState;

    pub struct UnifiedHeapMarker {}

    pub struct UnifiedHeapMarkingVisitorBase<'a> {
        marking_state_: &'a mut BasicMarkingState,
        unified_heap_marking_state_: &'a mut UnifiedHeapMarkingState,
    }

    impl<'a> UnifiedHeapMarkingVisitorBase<'a> {
        pub fn new(
            marking_state_: &'a mut BasicMarkingState,
            unified_heap_marking_state_: &'a mut UnifiedHeapMarkingState,
        ) -> Self {
            UnifiedHeapMarkingVisitorBase {
                marking_state_: marking_state_,
                unified_heap_marking_state_: unified_heap_marking_state_,
            }
        }
        
        // C++ handling
        // TODO: implement Visit
        // final keyword cannot be represented in Rust
        pub fn visit(&self, _ptr: *const std::ffi::c_void, _descriptor: TraceDescriptor) {
            todo!()
        }
        // TODO: implement VisitMultipleUncompressedMember
        // final keyword cannot be represented in Rust
        pub fn visit_multiple_uncompressed_member(
            &self,
            _ptr: *const std::ffi::c_void,
            _size: usize,
            _callback: TraceDescriptorCallback,
        ) {
            todo!()
        }
        // TODO: implement VisitMultipleCompressedMember
        // final keyword cannot be represented in Rust
        #[cfg(feature = "cppgc_pointer_compression")]
        pub fn visit_multiple_compressed_member(
            &self,
            _ptr: *const std::ffi::c_void,
            _size: usize,
            _callback: TraceDescriptorCallback,
        ) {
            todo!()
        }
        // TODO: implement VisitWeak
        // final keyword cannot be represented in Rust
        pub fn visit_weak(
            &self,
            _ptr: *const std::ffi::c_void,
            _descriptor: TraceDescriptor,
            _weak_callback: WeakCallback,
            _data: *const std::ffi::c_void,
        ) {
            todo!()
        }
        // TODO: implement VisitEphemeron
        // final keyword cannot be represented in Rust
        pub fn visit_ephemeron(
            &self,
            _key: *const std::ffi::c_void,
            _value: *const std::ffi::c_void,
            _descriptor: TraceDescriptor,
        ) {
            todo!()
        }
        // TODO: implement VisitWeakContainer
        // final keyword cannot be represented in Rust
        pub fn visit_weak_container(
            &self,
            _self_ptr: *const std::ffi::c_void,
            _strong_desc: TraceDescriptor,
            _weak_desc: TraceDescriptor,
            _callback: WeakCallback,
            _data: *const std::ffi::c_void,
        ) {
            todo!()
        }
        // TODO: implement RegisterWeakCallback
        // final keyword cannot be represented in Rust
        pub fn register_weak_callback(&self, _callback: WeakCallback, _data: *const std::ffi::c_void) {
            todo!()
        }
        // TODO: implement HandleMovableReference
        // final keyword cannot be represented in Rust
        pub fn handle_movable_reference(&self, _ptr: *const *const std::ffi::c_void) {
            todo!()
        }
        // JS handling
        // TODO: implement Visit for TracedReferenceBase
        pub fn visit_traced_reference_base(&self, _ref: &TracedReferenceBase) {
            todo!()
        }
    }

    pub struct MutatorUnifiedHeapMarkingVisitor<'a> {
        base: UnifiedHeapMarkingVisitorBase<'a>,
    }

    impl<'a> MutatorUnifiedHeapMarkingVisitor<'a> {
        pub fn new(
            marking_state_: &'a mut MutatorMarkingState,
            unified_heap_marking_state_: &'a mut UnifiedHeapMarkingState,
        ) -> Self {
            let basic_marking_state: &mut BasicMarkingState = unsafe {
                std::mem::transmute::<&mut MutatorMarkingState, &mut BasicMarkingState>(marking_state_)
            };

            MutatorUnifiedHeapMarkingVisitor {
                base: UnifiedHeapMarkingVisitorBase::new(basic_marking_state, unified_heap_marking_state_),
            }
        }
    }

    pub struct ConcurrentUnifiedHeapMarkingVisitor<'a> {
        base: UnifiedHeapMarkingVisitorBase<'a>,
        // TODO: Missing fields and functionality implementation for concurrent marking
        concurrent_unified_heap_marking_state_: UnifiedHeapMarkingState,
    }

    impl<'a> ConcurrentUnifiedHeapMarkingVisitor<'a> {
        pub fn new(
            marking_state_: &'a mut cppgc::internal::ConcurrentMarkingState,
            unified_heap_marking_state_: &'a mut UnifiedHeapMarkingState,
        ) -> Self {
            let basic_marking_state: &mut BasicMarkingState = unsafe {
                std::mem::transmute::<&mut cppgc::internal::ConcurrentMarkingState, &mut BasicMarkingState>(marking_state_)
            };
            ConcurrentUnifiedHeapMarkingVisitor {
                base: UnifiedHeapMarkingVisitorBase::new(basic_marking_state, unified_heap_marking_state_),
                concurrent_unified_heap_marking_state_: UnifiedHeapMarkingState::new(), // Dummy init
            }
        }
        // TODO: implement DeferTraceToMutatorThreadIfConcurrent
        // final keyword cannot be represented in Rust
        pub fn defer_trace_to_mutator_thread_if_concurrent(
            &self,
            _ptr: *const std::ffi::c_void,
            _callback: cppgc::TraceCallback,
            _size: usize,
        ) -> bool {
            todo!()
        }
    }
}

pub mod cppgc {
    pub type TraceDescriptor = u32;
    pub type TraceDescriptorCallback = u32;
    pub type WeakCallback = u32;
    pub type TraceCallback = u32;

    pub mod internal {
        pub struct HeapBase {}
        pub struct BasicMarkingState {}
        pub struct MutatorMarkingState {}

        pub struct ConcurrentMarkingState {}
    }
}

pub mod v8 {
    pub mod internal {
        pub mod heap {
            pub mod cppgc {
                pub mod marking_state {
                    pub struct BasicMarkingState {}
                }
                pub mod marking_visitor {
                    pub struct JSVisitor {}
                }
            }
        }
    }
    pub struct SourceLocation {}
}

pub mod heap {
    pub mod cppgc_js {
        pub mod unified_heap_marking_state {
            pub struct UnifiedHeapMarkingState {}

            impl UnifiedHeapMarkingState {
                pub fn new() -> Self {
                    UnifiedHeapMarkingState {}
                }
            }
        }
    }
}

pub mod base {
    pub mod macros {
        //Empty macro for now
        macro_rules! UNIMPLEMENTED {
            () => {
                todo!()
            };
        }
    }
}