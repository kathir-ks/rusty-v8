// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// TODO: Add equivalent crates for codegen and assembler
// use crate::codegen::assembler;
// use crate::codegen::reloc_info;
use std::ptr::NonNull;

// TODO: Define these modules/types
// mod heap;
// mod objects;
// use heap::{Heap, NewSpace, PagedSpaceBase, PageMetadata};
// use objects::{HeapObject, Map};
// use crate::heap::visit_object::VisitObject;

macro_rules! CHECK {
    ($condition:expr) => {
        if !$condition {
            panic!("Check failed: {}", stringify!($condition));
        }
    };
}

// TODO: Define v8_flags or equivalent
//extern crate v8_flags;

const VERIFY_HEAP: bool = cfg!(debug_assertions);

#[allow(dead_code)]
mod evacuation_verifier {
    use super::*;
    use std::marker::PhantomData;

    // TODO: Define HeapObject, ObjectSlot, MaybeObjectSlot, FullObjectSlot, InstructionStreamSlot, Code, InstructionStream, RelocInfo, Root, SkipRoot
    #[derive(Debug)]
    pub struct HeapObject {}
    #[derive(Debug)]
    pub struct ObjectSlot {}
    #[derive(Debug)]
    pub struct MaybeObjectSlot {}
    #[derive(Debug)]
    pub struct FullObjectSlot {}
    #[derive(Debug)]
    pub struct InstructionStreamSlot {}
    #[derive(Debug)]
    pub struct Code {}
    #[derive(Debug)]
    pub struct InstructionStream {}
    #[derive(Debug)]
    pub struct RelocInfo {}
    #[derive(Debug)]
    pub enum Root {}
    #[derive(Debug)]
    pub enum SkipRoot {}
    #[derive(Debug)]
    pub struct NewSpace {}
    #[derive(Debug)]
    pub struct PagedSpaceBase {}
    #[derive(Debug)]
    pub struct PageMetadata {}

    #[derive(Debug)]
    pub struct Heap {
        sweeping_in_progress_: bool,
        new_space_: Option<Box<NewSpace>>,
        old_space_: Option<Box<PagedSpaceBase>>,
        code_space_: Option<Box<PagedSpaceBase>>,
        shared_space_: Option<Box<PagedSpaceBase>>,
        isolate_: Box<Isolate>,
    }

    impl Heap {
        pub fn sweeping_in_progress(&self) -> bool {
            self.sweeping_in_progress_
        }

        pub fn new_space(&self) -> Option<&NewSpace> {
            self.new_space_.as_ref().map(|x| &**x)
        }

        pub fn old_space(&self) -> Option<&PagedSpaceBase> {
            self.old_space_.as_ref().map(|x| &**x)
        }

        pub fn code_space(&self) -> Option<&PagedSpaceBase> {
            self.code_space_.as_ref().map(|x| &**x)
        }

        pub fn shared_space(&self) -> Option<&PagedSpaceBase> {
            self.shared_space_.as_ref().map(|x| &**x)
        }
        // TODO: Implement IterateRootsIncludingClients
        pub fn IterateRootsIncludingClients(
            &self,
            visitor: &mut EvacuationVerifier,
            skip_root: base::EnumSet<SkipRoot>,
        ) {
            println!("Heap::IterateRootsIncludingClients called (dummy)");
        }

        pub fn isolate(&self) -> &Isolate {
            &self.isolate_
        }
    }
    
    #[derive(Debug)]
    pub struct Isolate {}

    // TODO: Implement base::EnumSet
    mod base {
        #[derive(Debug)]
        pub struct EnumSet<T> {
            _phantom: std::marker::PhantomData<T>,
        }
        impl<T> EnumSet<T> {
            pub fn new() -> Self {
                EnumSet {
                    _phantom: std::marker::PhantomData,
                }
            }
        }
    }

    impl Heap {
        pub fn new() -> Self {
            Heap {
                sweeping_in_progress_: false,
                new_space_: Some(Box::new(NewSpace {})),
                old_space_: Some(Box::new(PagedSpaceBase {})),
                code_space_: Some(Box::new(PagedSpaceBase {})),
                shared_space_: Some(Box::new(PagedSpaceBase {})),
                isolate_: Box::new(Isolate{}),
            }
        }
    }
    
    impl NewSpace {
        // TODO: Implement PagedNewSpace and From
        pub fn from(_space: &NewSpace) -> PagedNewSpace {
            PagedNewSpace{}
        }
    }

    #[derive(Debug)]
    pub struct PagedNewSpace {}

    impl PagedNewSpace {
        // TODO: Implement paged_space()
        pub fn paged_space(&self) -> &PagedSpaceBase {
            unimplemented!()
        }
    }

    impl From<&NewSpace> for PagedNewSpace {
        fn from(_space: &NewSpace) -> Self {
            PagedNewSpace {}
        }
    }

    impl PagedSpaceBase {
        //TODO: Implement iter()
        pub fn iter(&self) -> std::slice::Iter<'static, PageMetadata> {
            [].iter()
        }
    }

    impl PageMetadata {
        pub fn area_start(&self) -> usize {
            0
        }
        pub fn area_end(&self) -> usize {
            0
        }
        // TODO: Implement Chunk
        pub fn Chunk(&self) -> Chunk {
            Chunk {}
        }
    }

    #[derive(Debug)]
    pub struct Chunk {}

    impl Chunk {
        //TODO: Implement IsEvacuationCandidate
        pub fn IsEvacuationCandidate(&self) -> bool {
            false
        }
    }

    // TODO: Implement ObjectVisitorWithCageBases
    #[derive(Debug)]
    pub struct ObjectVisitorWithCageBases<'a> {
        heap_: &'a Heap,
    }

    impl<'a> ObjectVisitorWithCageBases<'a> {
        pub fn new(heap: &'a Heap) -> Self {
            ObjectVisitorWithCageBases { heap_: heap }
        }
        pub fn cage_base(&self) -> usize {
            0
        }
    }

    #[derive(Debug)]
    pub struct EvacuationVerifier<'a> {
        inner: ObjectVisitorWithCageBases<'a>,
        heap_: &'a Heap,
    }

    impl<'a> EvacuationVerifier<'a> {
        pub fn new(heap: &'a Heap) -> Self {
            EvacuationVerifier {
                inner: ObjectVisitorWithCageBases::new(heap),
                heap_: heap,
            }
        }

        pub fn run(&mut self) {
            CHECK!(!self.heap_.sweeping_in_progress());
            self.verify_roots();
            self.verify_evacuation(self.heap_.new_space());
            self.verify_evacuation(self.heap_.old_space());
            self.verify_evacuation(self.heap_.code_space());
            if self.heap_.shared_space().is_some() {
                self.verify_evacuation(self.heap_.shared_space());
            }
        }

        pub fn visit_pointers(&mut self, host: Tagged<HeapObject>, start: ObjectSlot, end: ObjectSlot) {
            self.verify_pointers_impl(start, end);
        }

        pub fn visit_pointers_maybe(&mut self, host: Tagged<HeapObject>, start: MaybeObjectSlot, end: MaybeObjectSlot) {
            self.verify_pointers_impl(start, end);
        }

        pub fn visit_instruction_stream_pointer(
            &mut self,
            host: Tagged<Code>,
            slot: InstructionStreamSlot,
        ) {
            //TODO: Implement slot.load
            let maybe_code: Tagged<Object> = Tagged::Object(HeapObject{}); //slot.load(self.inner.cage_base());
            let code_opt: Option<Tagged<HeapObject>> = match maybe_code {
                Tagged::HeapObject(ho) => Some(Tagged::HeapObject(ho)),
                _ => None,
            };
            // The slot might contain smi during Code creation, so skip it.
            if let Some(code) = code_opt {
                self.verify_heap_object_impl(code);
            }
        }

        pub fn visit_root_pointers(
            &mut self,
            _root: Root,
            _description: &str,
            start: FullObjectSlot,
            end: FullObjectSlot,
        ) {
            self.verify_pointers_impl(start, end);
        }

        pub fn visit_map_pointer(&mut self, object: Tagged<HeapObject>) {
            //TODO: Implement object.map
            let map = HeapObject{};//object.map(self.inner.cage_base());
            self.verify_heap_object_impl(Tagged::HeapObject(map));
        }

        pub fn visit_code_target(
            &mut self,
            host: Tagged<InstructionStream>,
            rinfo: &RelocInfo,
        ) {
            //TODO: Implement InstructionStream::FromTargetAddress
            let target = InstructionStream{};//InstructionStream::FromTargetAddress(rinfo.target_address());
            self.verify_heap_object_impl(Tagged::HeapObject(target));
        }

        pub fn visit_embedded_pointer(
            &mut self,
            host: Tagged<InstructionStream>,
            rinfo: &RelocInfo,
        ) {
            //TODO: Implement rinfo.target_object
            let target = HeapObject{};//rinfo.target_object(self.inner.cage_base());
            self.verify_heap_object_impl(Tagged::HeapObject(target));
        }

        fn verify_roots(&mut self) {
            self.heap_.IterateRootsIncludingClients(
                self,
                base::EnumSet::<SkipRoot>::new(),//base::EnumSet::<SkipRoot>{},
            );
        }

        fn verify_evacuation_on_page(&mut self, start: usize, end: usize) {
            let mut current = start;
            while current < end {
                let object = HeapObject::from_address(current);
                //TODO: Implement IsFreeSpaceOrFiller, VisitObject, ALIGN_TO_ALLOCATION_ALIGNMENT
                if true {//!is_free_space_or_filler(object, self.inner.cage_base()) {
                    println!("VisitObject");
                    //visit_object(self.heap_.isolate(), object, self);
                }
                current += 1;//align_to_allocation_alignment(object.size(self.inner.cage_base()));
            }
        }

        fn verify_evacuation(&mut self, space_option: Option<&NewSpace>) {
            let space = match space_option {
                Some(s) => s,
                None => return,
            };

            //if v8_flags.minor_ms {
            //    self.verify_evacuation(PagedNewSpace::from(space).paged_space());
            //    return;
            //}

            //TODO: Implement iter
            for p in [].iter() {//space.iter() {
                self.verify_evacuation_on_page(p.area_start(), p.area_end());
            }
        }

        fn verify_evacuation_paged(&mut self, space_option: Option<&PagedSpaceBase>) {
            let space = match space_option {
                Some(s) => s,
                None => return,
            };

            for p in space.iter() {
                if p.Chunk().IsEvacuationCandidate() {
                    continue;
                }
                self.verify_evacuation_on_page(p.area_start(), p.area_end());
            }
        }

        fn verify_pointers_impl<T>(&mut self, _start: T, _end: T) {
            //TODO: Implement
            println!("VerifyPointersImpl called (dummy)");
        }

        fn verify_heap_object_impl(&mut self, _object: Tagged<HeapObject>) {
            //TODO: Implement
            println!("VerifyHeapObjectImpl called (dummy)");
        }
    }

    impl HeapObject {
        fn from_address(_address: usize) -> Self {
            HeapObject {}
        }

        fn size(&self, _cage_base: usize) -> usize {
            1
        }
    }

    // TODO: Implement Tagged<T>
    #[derive(Debug)]
    pub enum Tagged<T> {
        HeapObject(HeapObject),
        Code(Code),
        InstructionStream(InstructionStream),
        Object(HeapObject),
        _Phantom(PhantomData<T>),
    }
}

fn main() {
    if VERIFY_HEAP {
        let heap = evacuation_verifier::Heap::new();
        let mut verifier = evacuation_verifier::EvacuationVerifier::new(&heap);
        verifier.run();
    } else {
        println!("VERIFY_HEAP is not enabled.");
    }
}