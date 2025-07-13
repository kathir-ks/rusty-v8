// Converted from V8 C++ source files:
// Header: mark-sweep-utilities.h
// Implementation: mark-sweep-utilities.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod mark_sweep_utilities {
    use std::collections::HashSet;
    use std::marker::PhantomData;
    use crate::heap::heap::{Heap, V8};
    use crate::heap::marking_state::MarkingState;
    use crate::heap::memory_chunk::{Address, MarkingMode, MutablePageMetadata, PageMetadata};
    use crate::heap::spaces::{NewSpace, PagedSpaceBase, LargeObjectSpace};
    use crate::objects::string_forwarding_table::StringForwardingTable;
    use crate::objects::visitors::{ObjectSlot, MaybeObjectSlot, InstructionStreamSlot, FullObjectSlot};
    use crate::objects::map::Map;
    use crate::objects::heap_object::HeapObject;
    use crate::heap::marking_bitmap::MarkingBitmap;
    use crate::heap::live_object_range::*;
    use crate::heap::visit_object::*;
    use crate::isolate::Isolate;
    use crate::objects::string::String;
    use crate::objects::thin_string::ThinString;
    use crate::objects::external_string::ExternalString;
    use crate::heap::heap_layout::*;
    use crate::objects::object::*;
    use crate::read_only_roots::ReadOnlyRoots;
    use crate::heap::marking_helper::MarkingHelper;
    use crate::heap::marking_worklist::MarkingWorklists;
    use crate::heap::cppgc_js::cpp_heap::CppHeap;
    use crate::heap::memory_chunk_iterator::*;
    use crate::heap::base::EnumSet;
    use crate::heap::base::SkipRoot;
    use crate::flags::v8_flags;
    use crate::heap::allocation_space::*;
    use crate::heap::garbage_collector::*;
    use crate::heap::old_generation_memory_chunk_iterator::*;
    use crate::heap::access_mode::AccessMode;
    use crate::heap::typed_slot_set::*;

    #[cfg(debug_assertions)]
    use std::ptr::null_mut;

    const kNullAddress: Address = 0;

    #[cfg(feature = "verify_heap")]
    pub struct MarkingVerifierBase<'a> {
        heap_: &'a mut Heap,
        cage_base_: usize,
        phantom: PhantomData<&'a Heap>,
    }

    #[cfg(feature = "verify_heap")]
    impl<'a> MarkingVerifierBase<'a> {
        pub fn new(heap: &'a mut Heap) -> Self {
            MarkingVerifierBase {
                heap_: heap,
                cage_base_: 0,
                phantom: PhantomData,
            }
        }

        pub fn set_cage_base(&mut self, cage_base: usize) {
            self.cage_base_ = cage_base;
        }

        pub fn run(&mut self) {
            todo!()
        }

        fn bitmap(&self, chunk: *const MutablePageMetadata) -> &MarkingBitmap {
            unsafe { &(*chunk).marking_bitmap }
        }

        fn verify_map(&self, map: Tagged<Map>) {
            todo!()
        }

        fn verify_pointers(&self, start: ObjectSlot, end: ObjectSlot) {
            todo!()
        }

        fn verify_pointers_maybe_object(&self, start: MaybeObjectSlot, end: MaybeObjectSlot) {
            todo!()
        }

        fn verify_code_pointer(&self, slot: InstructionStreamSlot) {
            todo!()
        }

        fn verify_root_pointers(&self, start: FullObjectSlot, end: FullObjectSlot) {
            todo!()
        }

        fn is_marked(&self, object: Tagged<HeapObject>) -> bool {
            todo!()
        }

        fn visit_pointers(&mut self, host: Tagged<HeapObject>, start: ObjectSlot, end: ObjectSlot) {
            self.verify_pointers(start, end);
        }

        fn visit_pointers_maybe_object(&mut self, host: Tagged<HeapObject>, start: MaybeObjectSlot, end: MaybeObjectSlot) {
            self.verify_pointers_maybe_object(start, end);
        }

        fn visit_instruction_stream_pointer(&mut self, host: Tagged<code::Code>, slot: InstructionStreamSlot) {
            self.verify_code_pointer(slot);
        }

        fn visit_root_pointers(&mut self, root: Root, description: &str, start: FullObjectSlot, end: FullObjectSlot) {
            self.verify_root_pointers(start, end);
        }

        fn visit_map_pointer(&mut self, object: Tagged<HeapObject>) {
            self.verify_map(object.map(self.cage_base_));
        }

        fn verify_roots(&mut self) {
            self.heap_.iterate_roots_including_clients(self, EnumSet::new());
        }

        fn verify_marking_on_page(&mut self, page: &PageMetadata, start: Address, end: Address) {
            let mut next_object_must_be_here_or_later = start;

            for live_object in LiveObjectRange::new(page) {
                let (object, size) = live_object;
                let current = object.address();
                if current < start {
                    continue;
                }
                if current >= end {
                    break;
                }
                assert!(self.is_marked(object));
                assert!(current >= next_object_must_be_here_or_later);
                visit_object(self.heap_.isolate(), object, self);
                next_object_must_be_here_or_later = current + size;

                let page_metadata = page as *const PageMetadata as *const MutablePageMetadata;
                let page_metadata = unsafe { &*page_metadata };

                let bitmap = self.bitmap(page_metadata);

                assert!(bitmap.all_bits_set_in_range(
                    MarkingBitmap::address_to_index(current),
                    MarkingBitmap::limit_address_to_index(next_object_must_be_here_or_later)
                ) || bitmap.all_bits_clear_in_range(
                    MarkingBitmap::address_to_index(current) + 1,
                    MarkingBitmap::limit_address_to_index(next_object_must_be_here_or_later)
                ));
                current = next_object_must_be_here_or_later;
            }
        }

        fn verify_marking_new_space(&mut self, space: &mut NewSpace) {
            if v8_flags.minor_ms {
                self.verify_marking_paged_new_space(PagedNewSpace::from(space));
                return;
            }
            for page in space.iter_mut() {
                self.verify_marking_on_page(page, page.area_start(), page.area_end());
            }
        }

        fn verify_marking_paged_new_space(&mut self, space: &mut PagedSpaceBase) {
            for page in space.iter_mut() {
                self.verify_marking_on_page(page, page.area_start(), page.area_end());
            }
        }

        fn verify_marking_paged_space(&mut self, space: &mut PagedSpaceBase) {
            for page in space.iter_mut() {
                self.verify_marking_on_page(page, page.area_start(), page.area_end());
            }
        }

        fn verify_marking_large_object_space(&mut self, lo_space: &mut LargeObjectSpace) {
            let mut it = LargeObjectSpaceObjectIterator {
                space: lo_space,
                current: Address::default(),
                limit: Address::default(),
                chunk_iterator: lo_space.chunk_list_.head_.map(|c| unsafe { &mut *c }),
                object: Tagged::<HeapObject>::null(),
            };

            while let Some(obj) = it.next() {
                if self.is_marked(obj) {
                    visit_object(self.heap_.isolate(), obj, self);
                }
            }
        }
    }

    #[derive(PartialEq, Eq)]
    pub enum ExternalStringTableCleaningMode {
        kAll,
        kYoungOnly,
    }

    pub struct ExternalStringTableCleanerVisitor<'a, const MODE: ExternalStringTableCleaningMode> {
        heap_: &'a mut Heap,
        phantom: PhantomData<&'a Heap>,
    }

    impl<'a, const MODE: ExternalStringTableCleaningMode> ExternalStringTableCleanerVisitor<'a, MODE> {
        pub fn new(heap: &'a mut Heap) -> Self {
            ExternalStringTableCleanerVisitor {
                heap_: heap,
                phantom: PhantomData,
            }
        }

        pub fn visit_root_pointers(&mut self, root: Root, description: &str, start: FullObjectSlot, end: FullObjectSlot) {
            assert_eq!(root, Root::kExternalStringsTable);
            let marking_state = self.heap_.non_atomic_marking_state();
            let the_hole = ReadOnlyRoots::create(self.heap_).the_hole_value();

            for p in start..end {
                let o = *p;
                if !o.is_heap_object() {
                    continue;
                }
                let heap_object = unsafe { o.unchecked_cast::<HeapObject>() };

                if MarkingHelper::is_marked_or_always_live(self.heap_, marking_state, heap_object) {
                    continue;
                }

                if MODE == ExternalStringTableCleaningMode::kYoungOnly && !HeapLayout::is_in_young_generation(heap_object) {
                    continue;
                }

                if o.is_external_string() {
                    self.heap_.finalize_external_string(unsafe { o.unchecked_cast::<String>() });
                } else {
                    assert!(o.is_thin_string());
                }

                *p = the_hole;
            }
        }
    }

    pub struct StringForwardingTableCleanerBase<'a> {
        isolate_: &'a mut Isolate,
        marking_state_: &'a mut MarkingState,
        disposed_resources_: HashSet<Address>,
    }

    impl<'a> StringForwardingTableCleanerBase<'a> {
        pub fn new(heap: &'a mut Heap) -> Self {
            StringForwardingTableCleanerBase {
                isolate_: heap.isolate(),
                marking_state_: heap.non_atomic_marking_state(),
                disposed_resources_: HashSet::new(),
            }
        }

        fn dispose_external_resource(&mut self, record: &mut StringForwardingTable::Record) {
            let resource = record.external_resource_address();
            if resource != kNullAddress && !self.disposed_resources_.contains(&resource) {
                record.dispose_external_resource();
                self.disposed_resources_.insert(resource);
            }
        }
    }

    pub fn is_cpp_heap_marking_finished(heap: &mut Heap, local_marking_worklists: &mut MarkingWorklists) -> bool {
        let cpp_heap = CppHeap::from(heap.cpp_heap());
        if cpp_heap.is_none() {
            return true;
        }
        cpp_heap.unwrap().is_marking_done() && local_marking_worklists.is_wrapper_empty()
    }

    #[cfg(debug_assertions)]
    pub fn verify_remembered_sets_after_evacuation(heap: &mut Heap, garbage_collector: GarbageCollector) {
        let new_space_is_empty = heap.new_space().is_none() || heap.new_space().as_ref().unwrap().size() == 0;
        assert!(!(garbage_collector == GarbageCollector::MARK_COMPACTOR) || new_space_is_empty);

        let mut chunk_iterator = MemoryChunkIterator::new(heap);

        while let Some(chunk) = chunk_iterator.next() {
            assert!(chunk.slot_set::<OLD_TO_OLD, AccessMode::ATOMIC>().is_none());
            assert!(chunk.slot_set::<TRUSTED_TO_TRUSTED, AccessMode::ATOMIC>().is_none());
            assert!(chunk.typed_slot_set::<OLD_TO_OLD, AccessMode::ATOMIC>().is_none());
            assert!(chunk.typed_slot_set::<TRUSTED_TO_TRUSTED, AccessMode::ATOMIC>().is_none());

            if new_space_is_empty && garbage_collector == GarbageCollector::MARK_COMPACTOR {
                assert!(chunk.slot_set::<OLD_TO_NEW, AccessMode::ATOMIC>().is_none());
                assert!(chunk.typed_slot_set::<OLD_TO_NEW, AccessMode::ATOMIC>().is_none());
                assert!(chunk.slot_set::<OLD_TO_NEW_BACKGROUND, AccessMode::ATOMIC>().is_none());
                assert!(chunk.typed_slot_set::<OLD_TO_NEW_BACKGROUND, AccessMode::ATOMIC>().is_none());
            }

            let id = chunk.owner_identity();
            if is_any_shared_space(id) || is_any_new_space(id) {
                assert!(chunk.slot_set::<OLD_TO_SHARED, AccessMode::ATOMIC>().is_none());
                assert!(chunk.typed_slot_set::<OLD_TO_SHARED, AccessMode::ATOMIC>().is_none());
                assert!(chunk.slot_set::<TRUSTED_TO_SHARED_TRUSTED, AccessMode::ATOMIC>().is_none());
            }
            assert!(chunk.typed_slot_set::<TRUSTED_TO_SHARED_TRUSTED>().is_none());
        }

        if v8_flags.sticky_mark_bits {
            OldGenerationMemoryChunkIterator::for_all(heap, |chunk| {
                assert!(!chunk.contains_slots::<OLD_TO_NEW>());
                assert!(!chunk.contains_slots::<OLD_TO_NEW_BACKGROUND>());
            });
        }
    }
}

use crate::heap::mark_sweep_utilities::*;
