// Copyright 2015 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/heap/scavenger-inl.h

//use std::sync::atomic::{AtomicPtr, Ordering}; //Potentially needed in future implementations
//use std::ptr::NonNull; //Potentially needed in future implementations

mod heap {
    pub mod evacuation_allocator_inl {}
    pub mod heap_layout_inl {}
    pub mod heap_visitor_inl {}
    pub mod incremental_marking_inl {}
    pub mod marking_state_inl {}
    pub mod mutable_page_metadata {
        #[derive(PartialEq)]
        pub enum OwnerIdentity {
            NEW_LO_SPACE,
            // Other variants...
        }
        pub struct MutablePageMetadata {}

        impl MutablePageMetadata {
            pub fn from_heap_object(_object: &super::objects::HeapObject) -> Self {
                MutablePageMetadata {}
            }

            pub fn owner_identity(&self) -> OwnerIdentity {
                OwnerIdentity::NEW_LO_SPACE
            }
        }

    }
    pub mod new_spaces {
        pub struct SemiSpaceNewSpace {}
        impl SemiSpaceNewSpace {
            pub fn from(_heap: &super::Heap) -> Self {
                SemiSpaceNewSpace {}
            }
            pub fn should_be_promoted(&self, _address: usize) -> bool {
                false
            }
        }
    }
    pub mod pretenuring_handler_inl {}
    pub mod scavenger;
    pub mod objects;

    use crate::codegen::assembler_inl::kTaggedSize;
    use crate::heap::mutable_page_metadata::MutablePageMetadata;
    use crate::heap::mutable_page_metadata::OwnerIdentity;
    use crate::heap::new_spaces::SemiSpaceNewSpace;
    use crate::heap::objects::HeapObject;
    use crate::heap::objects::Map;
    use crate::heap::objects::MapWord;
    use crate::heap::objects::Object;
    use crate::heap::objects::ThinString;
    use crate::heap::objects::ConsString;
    use crate::heap::objects::JSArrayBuffer;
    use crate::heap::objects::JSObject;
    use crate::heap::objects::EphemeronHashTable;
    use crate::heap::objects::InternalIndex;
    use crate::heap::objects::String;
    use crate::heap::objects::Heap;
    use crate::heap::objects::ReadOnlyRoots;
    use crate::heap::objects::JSAPIObjectWithEmbedderSlots;

    use crate::heap::objects::VisitorId;
    use crate::heap::objects::ObjectFields;
    use crate::heap::objects::UncheckedCast;
    use crate::heap::objects::GCSafeCast;

    use crate::heap::objects::ObjectSlot;
    use crate::heap::objects::MaybeObjectSlot;
    use crate::heap::objects::FullMaybeObjectSlot;
    use crate::heap::objects::ExternalPointerSlot;
    use crate::heap::objects::ExternalPointerHandle;
    use crate::heap::objects::ExternalPointerTable;
    use crate::heap::objects::IsSharedExternalPointerType;

    use crate::heap::objects::AllocationResult;
    use crate::heap::objects::AllocationAlignment;

    use crate::heap::memory_chunk::MemoryChunk;
    use crate::heap::memory_chunk::MemoryChunkLayout;
    //use crate::heap::incremental_marking::IncrementalMarking;

    use crate::isolate::Isolate;
    use std::collections::HashSet;
    use std::marker::PhantomData;

    pub const kPromotedListSegmentSize: usize = 1024; // Example value, adjust as needed

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum CopyAndForwardResult {
        SUCCESS_YOUNG_GENERATION,
        SUCCESS_OLD_GENERATION,
        FAILURE,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum SlotCallbackResult {
        KEEP_SLOT,
        REMOVE_SLOT,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum PromotionHeapChoice {
        kPromoteIntoLocalHeap,
        kPromoteIntoSharedHeap,
    }

    pub struct Scavenger {
        is_logging_: bool,
        is_incremental_marking_: bool,
        is_compacting_: bool,
        shortcut_strings_: bool,
        shared_string_table_: bool,
        mark_shared_heap_: bool,
        allocator_: Allocator,
        local_promoted_list_: LocalPromotedList,
        local_copied_list_: LocalCopiedList,
        copied_size_: usize,
        promoted_size_: usize,
        local_pretenuring_feedback_: LocalPretenuringFeedback,
        local_surviving_new_large_objects_: HashSet<(HeapObject, Map)>,
        heap_: *mut Heap //Raw pointer example
    }

    impl Scavenger {
        pub fn new(heap: *mut Heap) -> Self {
            Scavenger {
                is_logging_: false,
                is_incremental_marking_: false,
                is_compacting_: false,
                shortcut_strings_: false,
                shared_string_table_: false,
                mark_shared_heap_: false,
                allocator_: Allocator {},
                local_promoted_list_: LocalPromotedList {},
                local_copied_list_: LocalCopiedList {},
                copied_size_: 0,
                promoted_size_: 0,
                local_pretenuring_feedback_: LocalPretenuringFeedback {},
                local_surviving_new_large_objects_: HashSet::new(),
                heap_: heap,
            }
        }
        pub fn should_eagerly_process_promoted_list(&self) -> bool {
            const K_PROCESS_PROMOTED_LIST_THRESHOLD: usize = kPromotedListSegmentSize / 2;
            self.local_promoted_list_.push_segment_size() >= K_PROCESS_PROMOTED_LIST_THRESHOLD
        }

        pub fn synchronize_page_access(&self, object: &MaybeObject) {
            #[cfg(thread_sanitizer)]
            {
                if let Some(heap_object) = object.get_heap_object() {
                    MemoryChunk::from_heap_object(&heap_object).synchronized_load();
                }
            }
        }

        pub fn migrate_object(
            &self,
            map: &Map,
            source: &HeapObject,
            target: &HeapObject,
            size: usize,
            promotion_heap_choice: PromotionHeapChoice,
        ) -> bool {
            if !source.relaxed_compare_and_swap_map_word_forwarded(
                MapWord::from_map(map),
                target,
            ) {
                return false;
            }

            target.set_map_word(map, StoreOrdering::Relaxed);
            unsafe {
                let source_ptr = source.address() + kTaggedSize;
                let target_ptr = target.address() + kTaggedSize;
                std::ptr::copy_nonoverlapping(source_ptr as *const u8, target_ptr as *mut u8, size - kTaggedSize);
            }

            if self.is_logging_ {
                self.on_move_event(source, target, size);
            }

            if self.is_incremental_marking_
                && (promotion_heap_choice != PromotionHeapChoice::kPromoteIntoSharedHeap
                    || self.mark_shared_heap_)
            {
               self.transfer_color(source, target); // Placeholder;
            }
            self.update_allocation_site(map, source, size, &self.local_pretenuring_feedback_);
            true
        }

        fn on_move_event(&self, _source: &HeapObject, _target: &HeapObject, _size: usize) {
            unsafe {
               ((*self.heap_)).on_move_event(_source, _target, _size);
            }
        }

        fn transfer_color(&self, _source: &HeapObject, _target: &HeapObject){
            unsafe {
                ((*self.heap_)).incremental_marking().transfer_color(_source, _target);
            }
        }

        fn update_allocation_site(&self, _map: &Map, _source: &HeapObject, _size: usize, _feedback: &LocalPretenuringFeedback){
            unsafe {
                ((*self.heap_)).pretenuring_handler().update_allocation_site(_map, _source, _size, &self.local_pretenuring_feedback_);
            }
        }

        fn fatal_process_out_of_memory(&self, message: &str){
            unsafe {
                ((*self.heap_)).fatal_process_out_of_memory(message);
            }
        }

        fn heap(&self) -> &Heap {
            unsafe {
                &*self.heap_
            }
        }

        fn non_atomic_marking_state(&self) -> &MarkingState {
            unsafe{
                ((*self.heap_)).non_atomic_marking_state()
            }
        }

        fn marking_state(&self) -> &MarkingState {
            unsafe{
                ((*self.heap_)).marking_state()
            }
        }

        fn incremental_marking(&self) -> &IncrementalMarking{
            unsafe {
                ((*self.heap_)).incremental_marking()
            }
        }

        fn pretenuring_handler(&self) -> &PretenuringHandler{
            unsafe {
                ((*self.heap_)).pretenuring_handler()
            }
        }

        fn copy_block(&self, target_address: usize, source_address: usize, size: usize){
            unsafe {
                ((*self.heap_)).copy_block(target_address, source_address, size);
            }
        }


        fn evacuate_object<THeapObjectSlot>(&self, slot: THeapObjectSlot, map: &Map, source: &HeapObject) -> SlotCallbackResult
        where
            THeapObjectSlot: HeapObjectSlotTrait
        {
            SLOW_DCHECK(Heap::in_from_page(source));
            SLOW_DCHECK(!MapWord::from_map(map).is_forwarding_address());
            let size = source.size_from_map(map);

            let visitor_id = map.visitor_id();
            match visitor_id {
                VisitorId::kVisitThinString => {
                    if slot.is_weak() {
                        panic!("DCHECK failed: (!(*slot).IsWeak())")
                    }
                    self.evacuate_thin_string(map, slot, UncheckedCast::<ThinString>(source), size)
                }
                VisitorId::kVisitShortcutCandidate => {
                    if slot.is_weak() {
                        panic!("DCHECK failed: (!(*slot).IsWeak())")
                    }
                    self.evacuate_shortcut_candidate(map, slot, UncheckedCast::<ConsString>(source), size)
                }
                VisitorId::kVisitSeqOneByteString | VisitorId::kVisitSeqTwoByteString => {
                    if !String::is_in_place_internalizable(map.instance_type()) {
                        panic!("DCHECK failed: (String::IsInPlaceInternalizable(map->instance_type()))")
                    }

                    self.evacuate_in_place_internalizable_string(
                        map,
                        slot,
                        UncheckedCast::<String>(source),
                        size,
                        Map::object_fields_from(VisitorId::kVisitSeqOneByteString),
                    )
                }
                _ => {
                    self.evacuate_object_default(map, slot, source, size, Map::object_fields_from(visitor_id))
                }
            }
        }

        fn evacuate_object_default<THeapObjectSlot>(&self, map: &Map, slot: THeapObjectSlot, object: &HeapObject, object_size: usize, object_fields: ObjectFields) -> SlotCallbackResult
        where
            THeapObjectSlot: HeapObjectSlotTrait
        {
            self.evacuate_object_default_impl::<THeapObjectSlot, PromotionHeapChoice>(map, slot, object, object_size, object_fields, PromotionHeapChoice::kPromoteIntoLocalHeap)
        }

        fn evacuate_object_default_impl<THeapObjectSlot, const PROMOTION_HEAP_CHOICE: PromotionHeapChoice>(&self, map: &Map, slot: THeapObjectSlot, object: &HeapObject, object_size: usize, object_fields: ObjectFields, promotion_heap_choice: PromotionHeapChoice) -> SlotCallbackResult
        where
            THeapObjectSlot: HeapObjectSlotTrait,
        {
            SLOW_DCHECK(object.size_from_map(map) == object_size);
            let mut result: CopyAndForwardResult;

            if self.handle_large_object(map, object, object_size, object_fields) {
                return SlotCallbackResult::KEEP_SLOT;
            }

            SLOW_DCHECK(object_size <= MemoryChunkLayout::allocatable_memory_in_data_page());
            let new_space = unsafe {
                SemiSpaceNewSpace::from(&*((*self.heap_)))
            };
            if !new_space.should_be_promoted(object.address()) {
                result = self.semi_space_copy_object(map, slot, object, object_size, object_fields);
                if result != CopyAndForwardResult::FAILURE {
                    return self.remembered_set_entry_needed(result);
                }
            }

            result = self.promote_object::<THeapObjectSlot, PROMOTION_HEAP_CHOICE>(map, slot, object, object_size, object_fields);
            if result != CopyAndForwardResult::FAILURE {
                return self.remembered_set_entry_needed(result);
            }

            result = self.semi_space_copy_object(map, slot, object, object_size, object_fields);
            if result != CopyAndForwardResult::FAILURE {
                return self.remembered_set_entry_needed(result);
            }

            self.fatal_process_out_of_memory("Scavenger: semi-space copy");
            unreachable!();
        }

        fn semi_space_copy_object<THeapObjectSlot>(&self, map: &Map, slot: THeapObjectSlot, object: &HeapObject, object_size: usize, object_fields: ObjectFields) -> CopyAndForwardResult
        where
            THeapObjectSlot: HeapObjectSlotTrait,
        {
            if !self.heap().allowed_to_be_migrated(map, object, Space::NEW_SPACE) {
                panic!("DCHECK failed: (heap()->AllowedToBeMigrated(map, object, NEW_SPACE))");
            }
            let alignment = HeapObject::required_alignment(map);
            let allocation = self.allocator_.allocate(Space::NEW_SPACE, object_size, alignment);

            if let Some(target) = allocation.to_heap_object() {
                if !self.marking_state().is_unmarked(&target) {
                    panic!("DCHECK failed: (heap()->marking_state()->IsUnmarked(target))");
                }

                let self_success = self.migrate_object(map, object, &target, object_size, PromotionHeapChoice::kPromoteIntoLocalHeap);
                if !self_success {
                    self.allocator_.free_last(Space::NEW_SPACE, &target, object_size);

                    let map_word = object.map_word(LoadOrdering::Relaxed);
                    slot.update_heap_object_reference_slot(map_word.to_forwarding_address(object));
                    self.synchronize_page_access(slot.deref());
                    if Heap::in_from_page(slot.deref()) {
                        panic!("DCHECK failed: (!Heap::InFromPage(*slot))");
                    }
                    return if Heap::in_to_page(slot.deref()) {
                        CopyAndForwardResult::SUCCESS_YOUNG_GENERATION
                    } else {
                        CopyAndForwardResult::SUCCESS_OLD_GENERATION
                    };
                }

                slot.update_heap_object_reference_slot(&target);

                if object_fields == ObjectFields::kMaybePointers {
                    self.local_copied_list_.push(target);
                }
                self.copied_size_ += object_size;
                return CopyAndForwardResult::SUCCESS_YOUNG_GENERATION;
            }
            CopyAndForwardResult::FAILURE
        }

        fn promote_object<THeapObjectSlot, const PROMOTION_HEAP_CHOICE: PromotionHeapChoice>(&self, map: &Map, slot: THeapObjectSlot, object: &HeapObject, object_size: usize, object_fields: ObjectFields) -> CopyAndForwardResult
        where
            THeapObjectSlot: HeapObjectSlotTrait,
        {
            if object_size < Heap::K_MIN_OBJECT_SIZE_IN_TAGGED_WORDS * kTaggedSize {
                panic!("DCHECK_GE(object_size, Heap::kMinObjectSizeInTaggedWords * kTaggedSize)");
            }
            let alignment = HeapObject::required_alignment(map);
            let space = if PROMOTION_HEAP_CHOICE == PromotionHeapChoice::kPromoteIntoLocalHeap {
                Space::OLD_SPACE
            } else {
                Space::SHARED_SPACE
            };
            let allocation = self.allocator_.allocate(space, object_size, alignment);

            if let Some(target) = allocation.to_heap_object() {
                if !self.non_atomic_marking_state().is_unmarked(&target) {
                    panic!("DCHECK failed: (heap()->non_atomic_marking_state()->IsUnmarked(target))");
                }
                let self_success = self.migrate_object(map, object, &target, object_size, PromotionHeapChoice::kPromoteIntoLocalHeap);
                if !self_success {
                    let space = if PROMOTION_HEAP_CHOICE == PromotionHeapChoice::kPromoteIntoLocalHeap {
                        Space::OLD_SPACE
                    } else {
                        Space::SHARED_SPACE
                    };
                    self.allocator_.free_last(space, &target, object_size);

                    let map_word = object.map_word(LoadOrdering::Relaxed);
                    slot.update_heap_object_reference_slot(map_word.to_forwarding_address(object));
                    self.synchronize_page_access(slot.deref());
                    if Heap::in_from_page(slot.deref()) {
                        panic!("DCHECK failed: (!Heap::InFromPage(*slot))");
                    }
                    return if Heap::in_to_page(slot.deref()) {
                        CopyAndForwardResult::SUCCESS_YOUNG_GENERATION
                    } else {
                        CopyAndForwardResult::SUCCESS_OLD_GENERATION
                    };
                }
                slot.update_heap_object_reference_slot(&target);

                if object_fields == ObjectFields::kMaybePointers || self.is_compacting_ {
                    self.local_promoted_list_.push(PromotedEntry {
                        target,
                        map: *map,
                        object_size,
                    });
                }
                self.promoted_size_ += object_size;
                return CopyAndForwardResult::SUCCESS_OLD_GENERATION;
            }
            CopyAndForwardResult::FAILURE
        }

        fn remembered_set_entry_needed(&self, result: CopyAndForwardResult) -> SlotCallbackResult {
            if result == CopyAndForwardResult::FAILURE {
                panic!("DCHECK failed: (CopyAndForwardResult::FAILURE != result)");
            }
            if result == CopyAndForwardResult::SUCCESS_YOUNG_GENERATION {
                SlotCallbackResult::KEEP_SLOT
            } else {
                SlotCallbackResult::REMOVE_SLOT
            }
        }

        fn handle_large_object(&self, map: &Map, object: &HeapObject, object_size: usize, object_fields: ObjectFields) -> bool {
            let owner_identity = MutablePageMetadata::from_heap_object(object).owner_identity();
            if owner_identity == OwnerIdentity::NEW_LO_SPACE {
                unsafe {
                    if !MemoryChunk::from_heap_object(object).in_new_large_object_space() {
                        panic!("DCHECK failed: (MemoryChunk::FromHeapObject(object)->InNewLargeObjectSpace())");
                    }
                }

                if object.relaxed_compare_and_swap_map_word_forwarded(MapWord::from_map(map), object) {
                    self.local_surviving_new_large_objects_.insert((object.clone(), map.clone()));
                    self.promoted_size_ += object_size;
                    if object_fields == ObjectFields::kMaybePointers {
                        self.local_promoted_list_.push(PromotedEntry {
                            target: *object,
                            map: *map,
                            object_size,
                        });
                    }
                }
                return true;
            }
            false
        }

        fn evacuate_thin_string<THeapObjectSlot>(&self, map: &Map, slot: THeapObjectSlot, object: &ThinString, object_size: usize) -> SlotCallbackResult
        where
            THeapObjectSlot: HeapObjectSlotTrait,
        {
            if self.shortcut_strings_ {
                let actual = object.actual();
                if Heap::layout_in_young_generation(actual) {
                    panic!("DCHECK failed: (!HeapLayout::InYoungGeneration(actual))");
                }
                slot.update_heap_object_reference_slot(actual);
                return SlotCallbackResult::REMOVE_SLOT;
            }

            if Map::object_fields_from(map.visitor_id()) != ObjectFields::kMaybePointers {
                panic!("DCHECK failed: (ObjectFields::kMaybePointers == Map::ObjectFieldsFrom(map->visitor_id()))");
            }
            self.evacuate_object_default(map, slot, object, object_size, ObjectFields::kMaybePointers)
        }

        fn evacuate_shortcut_candidate<THeapObjectSlot>(&self, map: &Map, slot: THeapObjectSlot, object: &ConsString, object_size: usize) -> SlotCallbackResult
        where
            THeapObjectSlot: HeapObjectSlotTrait,
        {
            if !Self::is_shortcut_candidate(map.instance_type()) {
                panic!("DCHECK failed: (IsShortcutCandidate(map->instance_type()))");
            }

            if self.shortcut_strings_
                && object.unchecked_second() == self.read_only_roots().empty_string()
            {
                let first = UncheckedCast::<HeapObject>(object.unchecked_first());

                slot.update_heap_object_reference_slot(first);

                if !Heap::layout_in_young_generation(first) {
                    object.set_map_word_forwarded(first, StoreOrdering::Relaxed);
                    return SlotCallbackResult::REMOVE_SLOT;
                }

                let first_word = first.map_word(LoadOrdering::Relaxed);
                if first_word.is_forwarding_address() {
                    let target = first_word.to_forwarding_address(first);

                    slot.update_heap_object_reference_slot(target);
                    self.synchronize_page_access(target);
                    object.set_map_word_forwarded(target, StoreOrdering::Relaxed);
                    return if Heap::layout_in_young_generation(target) {
                        SlotCallbackResult::KEEP_SLOT
                    } else {
                        SlotCallbackResult::REMOVE_SLOT
                    };
                }
                let first_map = first_word.to_map();
                let result = self.evacuate_object_default(
                    first_map,
                    slot,
                    first,
                    first.size_from_map(first_map),
                    Map::object_fields_from(first_map.visitor_id()),
                );
                object.set_map_word_forwarded(slot.to_heap_object(), StoreOrdering::Relaxed);
                return result;
            }
            if Map::object_fields_from(map.visitor_id()) != ObjectFields::kMaybePointers {
                panic!("DCHECK failed: (ObjectFields::kMaybePointers == Map::ObjectFieldsFrom(map->visitor_id()))");
            }
            self.evacuate_object_default(map, slot, object, object_size, ObjectFields::kMaybePointers)
        }

        fn evacuate_in_place_internalizable_string<THeapObjectSlot>(
            &self,
            map: &Map,
            slot: THeapObjectSlot,
            object: &String,
            object_size: usize,
            object_fields: ObjectFields,
        ) -> SlotCallbackResult
        where
            THeapObjectSlot: HeapObjectSlotTrait,
        {
            if !String::is_in_place_internalizable(map.instance_type()) {
                panic!("DCHECK failed: (String::IsInPlaceInternalizable(map->instance_type()))");
            }
            if object_fields != Map::object_fields_from(map.visitor_id()) {
                panic!(
                    "DCHECK failed: (object_fields == Map::ObjectFieldsFrom(map->visitor_id()))"
                );
            }
            if self.shared_string_table_ {
                self.evacuate_object_default_impl::<THeapObjectSlot, {PromotionHeapChoice::kPromoteIntoSharedHeap}>(map, slot, object, object_size, object_fields, PromotionHeapChoice::kPromoteIntoSharedHeap)
            } else {
                self.evacuate_object_default(map, slot, object, object_size, object_fields)
            }
        }

        fn scavenge_object<THeapObjectSlot>(&self, p: THeapObjectSlot, object: &HeapObject) -> SlotCallbackResult
        where
            THeapObjectSlot: HeapObjectSlotTrait,
        {
            if !Heap::in_from_page(object) {
                panic!("DCHECK failed: (Heap::InFromPage(object))");
            }

            let first_word = object.map_word(LoadOrdering::Relaxed);

            if first_word.is_forwarding_address() {
                let dest = first_word.to_forwarding_address(object);
                p.update_heap_object_reference_slot(dest);
                self.synchronize_page_access(dest);
                unsafe {
                    if Heap::layout_in_young_generation(dest) {
                        if ! (Heap::in_to_page(dest) || Heap::is_large_object(dest) || MemoryChunk::from_heap_object(dest).is_quarantined()) {
                           println!("destination address: {}", dest.address());
                           println!("is in to page: {}", Heap::in_to_page(dest));
                           println!("is large object: {}", Heap::is_large_object(dest));
                           println!("is quarantined: {}", MemoryChunk::from_heap_object(dest).is_quarantined());
                        }
                        if !(Heap::in_to_page(dest) || Heap::is_large_object(dest) || MemoryChunk::from_heap_object(dest).is_quarantined()) {
                            panic!("DCHECK_IMPLIES(HeapLayout::InYoungGeneration(dest),\n                   Heap::InToPage(dest) || Heap::IsLargeObject(dest) ||\n                       MemoryChunk::FromHeapObject(dest)->IsQuarantined())");
                        }
                    }
                }

                return if Heap::layout_in_young_generation(dest) {
                    SlotCallbackResult::KEEP_SLOT
                } else {
                    SlotCallbackResult::REMOVE_SLOT
                };
            }

            let map = first_word.to_map();
            if map == self.read_only_roots().allocation_memento_map() {
                panic!("DCHECK_NE(ReadOnlyRoots(heap()).allocation_memento_map(), map)");
            }

            self.evacuate_object(p, map, object)
        }

        pub fn check_and_scavenge_object<TSlot>(heap: *mut Heap, slot: TSlot) -> SlotCallbackResult
        where
            TSlot: MaybeObjectSlotTrait,
        {
            let object = slot.deref();

            if Heap::in_from_page(object) {
                let heap_object = object.get_heap_object();
                let result = Self::new(heap).scavenge_object(slot.to_heap_object_slot(), &heap_object);
                if result == SlotCallbackResult::REMOVE_SLOT && Heap::layout_in_young_generation(slot.deref().get_heap_object()) {
                    panic!("DCHECK_IMPLIES(result == REMOVE_SLOT,\n                   !HeapLayout::InYoungGeneration((*slot).GetHeapObject()))");
                }
                return result;
            } else if Heap::in_to_page(object) {
                return SlotCallbackResult::KEEP_SLOT;
            }

            SlotCallbackResult::REMOVE_SLOT
        }

        fn read_only_roots(&self) -> &ReadOnlyRoots{
            unsafe {
                ((*self.heap_)).read_only_roots()
            }
        }

        pub fn add_ephemeron_hash_table(&self, _table: &EphemeronHashTable){
           //Implementation will be added
        }
        fn is_shortcut_candidate(instance_type: InstanceType) -> bool {
           instance_type == InstanceType::ConsString
        }
    }

    unsafe trait FullHeapObjectSlotTrait {
        fn update_heap_object_reference_slot(&self, object: &HeapObject);
        fn is_weak(&self) -> bool;
        fn deref(&self) -> &MaybeObject;
        fn to_heap_object(&self) -> FullHeapObjectSlot;
        fn to_heap_object(&self) -> HeapObjectSlot;
        fn to_heap_object(&self) -> &HeapObject;

    }

    unsafe trait HeapObjectSlotTrait {
        fn update_heap_object_reference_slot(&self, object: &HeapObject);
        fn is_weak(&self) -> bool;
        fn deref(&self) -> &MaybeObject;
        fn to_heap_object(&self) -> FullHeapObjectSlot;
        fn to_heap_object(&self) -> HeapObjectSlot;
        fn to_heap_object(&self) -> &HeapObject;
    }

    unsafe trait MaybeObjectSlotTrait {
        fn deref(&self) -> &MaybeObject;
        type THeapObjectSlot: HeapObjectSlotTrait;
        fn to_heap_object_slot(&self) -> Self::THeapObjectSlot;
    }

    struct FullHeapObjectSlot {
        address: usize
    }
    impl FullHeapObjectSlot {
        fn new(address: usize) -> Self {
           FullHeapObjectSlot { address }
        }
    }
    unsafe impl FullHeapObjectSlotTrait for FullHeapObjectSlot {
        fn update_heap_object_reference_slot(&self, _object: &HeapObject) {
            // Implementation will be added
        }
        fn is_weak(&self) -> bool {
            // Implementation will be added
            false
        }
        fn deref(&self) -> &MaybeObject {
           //Implementation will be added
           panic!("");
        }
        fn to_heap_object(&self) -> FullHeapObjectSlot {
            FullHeapObjectSlot { address: self.address }
        }
        fn to_heap_object(&self) -> HeapObjectSlot {
            HeapObjectSlot { address: self.address }
        }
        fn to_heap_object(&self) -> &HeapObject {
            panic!("Cannot use this for heap object conversion");
        }
    }
    struct HeapObjectSlot {
        address: usize
    }
    impl HeapObjectSlot {
        fn new(address: usize) -> Self {
           HeapObjectSlot { address }
        }
    }
    unsafe impl HeapObjectSlotTrait for HeapObjectSlot {
        fn update_heap_object_reference_slot(&self, _object: &HeapObject) {
            // Implementation will be added
        }
        fn is_weak(&self) -> bool {
            // Implementation will be added
            false
        }
        fn deref(&self) -> &MaybeObject {
           //Implementation will be added
           panic!("");
        }
        fn to_heap_object(&self) -> FullHeapObjectSlot {
            FullHeapObjectSlot { address: self.address }
        }
        fn to_heap_object(&self) -> HeapObjectSlot {
            HeapObjectSlot { address: self.address }
        }
        fn to_heap_object(&self) -> &HeapObject {
            panic!("Cannot use this for heap object conversion");
        }
    }

    struct FullMaybeObjectSlot {
        address: usize,
    }
    impl FullMaybeObjectSlot {
        fn new(address: usize) -> Self {
            FullMaybeObjectSlot {address}
        }
    }
    unsafe impl MaybeObjectSlotTrait for FullMaybeObjectSlot {
        fn deref(&self) -> &MaybeObject {
            panic!("Implementation will be added")
        }
        type THeapObjectSlot = FullHeapObjectSlot;
        fn to_heap_object_slot(&self) -> Self::THeapObjectSlot {
            FullHeapObjectSlot { address: self.address }
        }
    }

    struct MaybeObject {
        address: usize,
    }

    impl MaybeObject {
        fn get_heap_object(&self) -> Option<&HeapObject> {
            unsafe {
                Some(&*(self.address as *const HeapObject))
            }
        }
    }

    enum Space {
        NEW_SPACE,
        OLD_SPACE,
        SHARED_SPACE,
        NEW_LO_SPACE,
    }

    impl Space {
        fn to_usize(&self) -> usize {
            match self {
                Space::NEW_SPACE => 0,
                Space::OLD_SPACE => 1,
                Space::SHARED_SPACE => 2,
                Space::NEW_LO_SPACE => 3,
            }
        }
    }
    
    struct Allocator {}

    impl Allocator {
        fn allocate(&self, _space: Space, _object_size: usize, _alignment: AllocationAlignment) -> AllocationResult {
            AllocationResult::Failure
        }
        fn free_last(&self, _space: Space, _target: &HeapObject, _object_size: usize) {}