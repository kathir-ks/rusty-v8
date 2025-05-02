// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/objects/visitors-inl.h

pub mod visitors_inl {
    use crate::codegen::reloc_info::RelocInfo;
    use crate::common::globals::*;
    use crate::execution::isolate::Isolate;
    use crate::heap::heap_layout_inl::*;
    use crate::objects::map::Map;
    use crate::objects::visitors::*;

    //use std::marker::PhantomData;

    #[derive(Clone, Copy)]
    pub struct ObjectVisitorWithCageBases {
        #[cfg(v8_compress_pointers)]
        cage_base_: PtrComprCageBase,
        #[cfg(all(v8_compress_pointers, v8_external_code_space))]
        code_cage_base_: PtrComprCageBase,
    }

    impl ObjectVisitorWithCageBases {
        #[cfg(v8_compress_pointers)]
        pub fn new(cage_base: PtrComprCageBase, code_cage_base: PtrComprCageBase) -> Self {
            ObjectVisitorWithCageBases {
                cage_base_: cage_base,
                #[cfg(v8_external_code_space)]
                code_cage_base_: code_cage_base,
            }
        }

        #[cfg(not(v8_compress_pointers))]
        pub fn new() -> Self {
            ObjectVisitorWithCageBases {}
        }

        #[cfg(v8_compress_pointers)]
        pub fn from_isolate(isolate: &Isolate) -> Self {
            ObjectVisitorWithCageBases::new(
                PtrComprCageBase(isolate.cage_base()),
                PtrComprCageBase(isolate.code_cage_base()),
            )
        }

        #[cfg(not(v8_compress_pointers))]
        pub fn from_isolate(_isolate: &Isolate) -> Self {
            ObjectVisitorWithCageBases {}
        }

        pub fn from_heap(heap: &Heap) -> Self {
            ObjectVisitorWithCageBases::from_isolate(Isolate::from_heap(heap))
        }
    }

    pub struct ClientRootVisitor<Visitor> {
        actual_visitor_: Visitor,
    }

    impl<Visitor> ClientRootVisitor<Visitor> {
        pub fn new(actual_visitor: Visitor) -> Self {
            ClientRootVisitor {
                actual_visitor_: actual_visitor,
            }
        }
        #[allow(unused_variables)]
        pub fn visit_running_code(&self, code_slot: FullObjectSlot, maybe_istream_slot: FullObjectSlot) {
            #[cfg(debug_assertions)]
            {
                //DCHECK(!HeapLayout::InWritableSharedSpace(Cast::<HeapObject>(&*code_slot)));
                //let maybe_istream = *maybe_istream_slot;
                //DCHECK(maybe_istream == Smi::zero() ||
                //       !HeapLayout::InWritableSharedSpace(Cast::<HeapObject>(&maybe_istream)));
            }
        }

        pub fn is_shared_heap_object(object: Tagged<Object>) -> bool {
            if is_heap_object(object) {
                //HeapLayout::InWritableSharedSpace(Cast::<HeapObject>(object))
                false // Placeholder: Replace with actual implementation
            } else {
                false
            }
        }
    }

    pub struct ClientObjectVisitor<Visitor> {
        actual_visitor_: Visitor,
        cage_base_: PtrComprCageBase,
        code_cage_base_: PtrComprCageBase,
    }

    impl<Visitor> ClientObjectVisitor<Visitor> {
        pub fn new(actual_visitor: Visitor, cage_base: PtrComprCageBase, code_cage_base: PtrComprCageBase) -> Self {
            ClientObjectVisitor {
                actual_visitor_: actual_visitor,
                cage_base_: cage_base,
                code_cage_base_: code_cage_base,
            }
        }

        #[allow(unused_variables)]
        pub fn visit_map_pointer(&self, host: Tagged<HeapObject>) {
            if !Self::is_shared_heap_object(host.map(self.cage_base_)) {
                return;
            }
            //self.actual_visitor_.VisitMapPointer(host); //TODO add a VisitMapPointer method in the visitor trait
        }

        #[allow(unused_variables)]
        pub fn visit_instruction_stream_pointer(&self, host: Tagged<Code>, slot: InstructionStreamSlot) {
            #[cfg(debug_assertions)]
            {
                //let istream_object = slot.load(self.code_cage_base_);
                //let mut istream: Tagged<InstructionStream> = Tagged::<InstructionStream>::default(); //TODO implement Default for Tagged<T>
                //if istream_object.GetHeapObject(&mut istream) {
                //    DCHECK(!HeapLayout::InWritableSharedSpace(istream));
                //}
            }
        }

        #[allow(unused_variables)]
        pub fn visit_code_target(&self, host: Tagged<InstructionStream>, rinfo: &mut RelocInfo) {
            #[cfg(debug_assertions)]
            {
                //let target = InstructionStream::FromTargetAddress(rinfo.target_address());
                //DCHECK(!HeapLayout::InWritableSharedSpace(target));
            }
        }

        #[allow(unused_variables)]
        pub fn visit_embedded_pointer(&self, host: Tagged<InstructionStream>, rinfo: &mut RelocInfo) {
            if !Self::is_shared_heap_object(rinfo.target_object(self.cage_base_)) {
                return;
            }
            //self.actual_visitor_.VisitEmbeddedPointer(host, rinfo); //TODO add a VisitEmbeddedPointer method in the visitor trait
        }

        pub fn is_shared_heap_object(object: Tagged<Object>) -> bool {
            if is_heap_object(object) {
                //HeapLayout::InWritableSharedSpace(Cast::<HeapObject>(object))
                false // Placeholder: Replace with actual implementation
            } else {
                false
            }
        }
    }
}