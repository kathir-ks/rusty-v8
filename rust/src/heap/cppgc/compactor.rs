// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::collections::{HashMap, HashSet, BTreeMap};
use std::mem;
use std::ptr;
use std::num::NonZeroUsize;
use std::ops::{Deref, DerefMut};

//use crate::cppgc::macros::*; // Assuming cppgc::macros contains CPPGC_STACK_ALLOCATED
use crate::heap::cppgc::compaction_worklists::{CompactionWorklists, MovableReference};
use crate::heap::cppgc::globals::*; // Assuming globals.h defines kKB
use crate::heap::cppgc::heap_base::HeapBase;
use crate::heap::cppgc::heap_page::{BasePage, NormalPage, HeapObjectHeader};
use crate::heap::cppgc::heap_space::NormalPageSpace;
use crate::heap::cppgc::memory::*;
use crate::heap::cppgc::object_poisoner::UnmarkedObjectsPoisoner;
use crate::heap::cppgc::raw_heap::RawHeap;
use crate::heap::cppgc::stats_collector::StatsCollector;
use crate::support::address::Address;

mod support {
    pub mod address {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
        pub struct Address(pub usize);
    }
}

pub mod cppgc {
    pub mod macros {
        #[macro_export]
        macro_rules! CPPGC_STACK_ALLOCATED {
            () => {
                // Implement stack allocation guard here if needed.
            };
        }
    }
}

pub mod heap {
    pub mod cppgc {
        pub mod compaction_worklists {
            pub type MovableReference = *mut usize;
            pub struct CompactionWorklists {}
            impl CompactionWorklists {
                pub fn new() -> Self {
                    CompactionWorklists {}
                }
                pub fn movable_slots_worklist(&self) -> MovableReferencesWorklist {
                   MovableReferencesWorklist {}
                }
            }

            pub struct MovableReferencesWorklist {}

            impl MovableReferencesWorklist {
                pub fn clear(&mut self) {}
                pub fn local(&self) -> Local {
                    Local{}
                }
            }

            pub struct Local {}

            impl Local {
                pub fn pop(&mut self, slot: &mut MovableReference) -> bool {
                    false
                }
            }
        }
        pub mod globals {
            pub const kKB: usize = 1024;
        }
        pub mod heap_base {
            pub struct HeapBase {}
            impl HeapBase {
                pub fn has_move_listeners(&self) -> bool {
                    false
                }
                pub fn call_move_listeners(&self, from: usize, to: usize, size_including_header: usize) {}
            }
        }
        pub mod heap_page {
            use crate::support::address::Address;

            #[derive(Debug)]
            pub struct HeapObjectHeader {}

            impl HeapObjectHeader {
                pub fn from_object(_object: Address) -> Self {
                    HeapObjectHeader{}
                }
                pub fn allocated_size(&self) -> usize {
                    0
                }
                pub fn is_free(&self) -> bool {
                    false
                }
                pub fn is_marked(&self) -> bool {
                    false
                }
                pub fn finalize(&self) {}
                pub fn unmark(&self) {}
                pub fn object_start(&self) -> usize {
                    0
                }
                pub fn object_size(&self) -> usize {
                    0
                }
            }

            pub struct BasePage {}
            impl BasePage {
                pub fn from_inner_address(_heap: &crate::heap::cppgc::heap_base::HeapBase, _address: *mut usize) -> *mut Self {
                    ptr::null_mut()
                }
                pub fn object_header_from_inner_address(&self, _address: *mut usize) -> HeapObjectHeader {
                    HeapObjectHeader{}
                }
                pub fn is_large(&self) -> bool {
                    false
                }
                pub fn space(&self) -> &crate::heap::cppgc::heap_space::NormalPageSpace {
                    unimplemented!()
                }
            }
            pub struct NormalPage {}
            impl NormalPage {
                pub fn from(_page: *mut BasePage) -> *mut NormalPage {
                    ptr::null_mut()
                }
                pub fn reset_marked_bytes(&self) {}
                pub fn payload_start(&self) -> usize {
                    0
                }
                pub fn payload_end(&self) -> usize {
                    0
                }
                pub fn object_start_bitmap(&self) -> ObjectStartBitmap {
                   ObjectStartBitmap{}
                }
                pub fn payload_size(&self) -> usize {
                    0
                }
                pub fn destroy(_page: *mut NormalPage) {}
            }
            pub struct ObjectStartBitmap {}
            impl ObjectStartBitmap {
                pub fn clear(&self) {}
                pub fn set_bit(&self, _address: usize) {}
                pub fn mark_as_fully_populated(&self) {}
            }
        }
        pub mod heap_space {
            pub struct NormalPageSpace {}
            impl NormalPageSpace {
                pub fn is_compactable(&self) -> bool {
                    false
                }
                pub fn size(&self) -> usize {
                    0
                }
                pub fn free_list(&self) -> FreeList {
                    FreeList{}
                }
                pub fn raw_heap(&self) -> &crate::heap::cppgc::raw_heap::RawHeap {
                    unimplemented!()
                }
                pub fn remove_all_pages(&self) -> Vec<*mut crate::heap::cppgc::heap_page::BasePage> {
                    Vec::new()
                }
                pub fn add_page(&self, _page: *mut crate::heap::cppgc::heap_page::NormalPage) {}
            }

            pub struct FreeList {}
            impl FreeList {
                pub fn size(&self) -> usize {
                    0
                }
                pub fn clear(&mut self) {}
                pub fn add(&mut self, _entry: FreeListEntry) {}
            }

            pub struct FreeListEntry {
                pub start: usize,
                pub size: usize
            }
        }
        pub mod memory {
            pub fn set_memory_inaccessible(_start: usize, _size: usize) {}
            pub fn zap_memory(_start: usize, _size: usize) {}
        }
        pub mod object_poisoner {
            pub struct UnmarkedObjectsPoisoner {}
            impl UnmarkedObjectsPoisoner {
                pub fn traverse(&self, _space: &crate::heap::cppgc::heap_space::NormalPageSpace) {}
            }
        }
        pub mod raw_heap {
            pub struct RawHeap {}
            impl RawHeap {
                pub fn heap(&self) -> &crate::heap::cppgc::heap_base::HeapBase {
                    unimplemented!()
                }
                pub fn iter(&self) -> std::slice::Iter<std::option::Option<std::boxed::Box<crate::heap::cppgc::heap_space::NormalPageSpace>>> {
                    unimplemented!()
                }
            }
        }
        pub mod stats_collector {
            pub struct StatsCollector {}
            impl StatsCollector {
                pub const kAtomicCompact: usize = 0;
                pub fn enabled_scope(&self, _heap: &StatsCollector, _compact: usize) -> EnabledScope {
                   EnabledScope{}
                }
            }
            pub struct EnabledScope {}
            impl Drop for EnabledScope {
                fn drop(&mut self) {}
            }
        }
    }
}

const K_FREE_LIST_SIZE_THRESHOLD: usize = 512 * 1024;

struct MovableReferences<'a> {
    heap_: &'a HeapBase,
    movable_references_: HashMap<usize, *mut usize>,
    interior_movable_references_: BTreeMap<*mut usize, usize>,
    heap_has_move_listeners_: bool,
    #[cfg(debug_assertions)]
    moved_objects_: HashSet<usize>,
    #[cfg(debug_assertions)]
    interior_slot_to_object_: HashMap<*mut usize, usize>,
}

impl<'a> MovableReferences<'a> {
    fn new(heap: &'a HeapBase) -> Self {
        MovableReferences {
            heap_: heap,
            movable_references_: HashMap::new(),
            interior_movable_references_: BTreeMap::new(),
            heap_has_move_listeners_: heap.has_move_listeners(),
            #[cfg(debug_assertions)]
            moved_objects_: HashSet::new(),
            #[cfg(debug_assertions)]
            interior_slot_to_object_: HashMap::new(),
        }
    }

    fn add_or_filter(&mut self, slot: *mut usize) {
        unsafe {
            let slot_page = BasePage::from_inner_address(self.heap_, slot);
            if slot_page.is_null() { return; }

            let value = *slot;
            if value == 0 { return; }

            let slot_header = (*slot_page).object_header_from_inner_address(slot);
            if !slot_header.is_marked() { return; }

            let value_page = BasePage::from_inner_address(self.heap_, value as *mut usize);
            if value_page.is_null() { return; }

            if (*value_page).is_large() || !(*value_page).space().is_compactable() { return; }

            let value_header = (*value_page).object_header_from_inner_address(value as *mut usize);
            assert!(value_header.is_marked());

            if let Some(reference_it) = self.movable_references_.get(&(value as usize)) {
                assert_eq!(slot, *reference_it);
                return;
            }

            self.movable_references_.insert(value as usize, slot);

            let slot_page = BasePage::from_inner_address(self.heap_, slot);
            if slot_page.is_null() { return; }

            if !(*slot_page).space().is_compactable() { return; }

            assert!(self.interior_movable_references_.get(&slot).is_none());
            self.interior_movable_references_.insert(slot, 0);

            #[cfg(debug_assertions)]
            self.interior_slot_to_object_.insert(slot, slot_header.object_start());
        }
    }

    fn relocate(&mut self, from: usize, to: usize, size_including_header: usize) {
        #[cfg(debug_assertions)]
        self.moved_objects_.insert(from);

        if self.heap_has_move_listeners_ {
            self.heap_.call_move_listeners(from - mem::size_of::<HeapObjectHeader>(), to - mem::size_of::<HeapObjectHeader>(), size_including_header);
        }

        if !self.interior_movable_references_.is_empty() {
            unsafe {
                let header = HeapObjectHeader::from_object(to as support::address::Address);
                let size = header.object_size();
                self.relocate_interior_references(from, to, size);
            }
        }

        if self.movable_references_.get(&(from as usize)).is_none() {
            return;
        }

        let slot = *self.movable_references_.get(&(from as usize)).unwrap();

        if let Some(interior_it) = self.interior_movable_references_.get(&slot) {
            let slot_location = *interior_it as *mut usize;
            if slot_location.is_null() {
                self.interior_movable_references_.insert(slot, to);

                #[cfg(debug_assertions)] {
                    let reverse_it = self.interior_slot_to_object_.get(&slot);
                    assert!(reverse_it.is_some());
                    assert!(!self.moved_objects_.contains(reverse_it.unwrap()));
                }
            } else {
                //slot = slot_location as *mut usize; // This is an address, casting is problematic
            }
        }

        unsafe {
            assert_eq!(from, *slot);
            *slot = to;
        }
    }

    fn relocate_interior_references(&mut self, from: usize, to: usize, size: usize) {
        let mut interior_it = self.interior_movable_references_.range(
            (ptr::null_mut::<usize>(), (from as *mut usize)..).
        );

        if let Some((&key, &value)) = interior_it.next() {
            let mut offset = (key as usize) - from;
            while offset < size {
                let value = *self.interior_movable_references_.get(&key).unwrap();
                if value == 0 {
                   // Update the interior reference value, so that when the object the slot
                    // is pointing to is moved, it can reuse this value.
                    let reference = to + offset;
                    self.interior_movable_references_.insert(key, reference);

                    // If the |slot|'s content is pointing into the region [from, from +
                    // size) we are dealing with an interior pointer that does not point to
                    // a valid HeapObjectHeader. Such references need to be fixed up
                    // immediately.
                    unsafe {
                        let reference_contents = reference as *mut usize;
                        if *reference_contents > from && *reference_contents < (from + size) {
                            *reference_contents = *reference_contents - from + to;
                        }
                    }
                }
                if let Some((&next_key, &next_value)) = interior_it.next() {
                    offset = (next_key as usize) - from;
                } else {
                    return;
                }
            }
        } else {
           return;
        }
    }

    fn update_callbacks(&mut self) {}
}

struct CompactionState<'a> {
    space_: &'a mut NormalPageSpace,
    movable_references_: &'a mut MovableReferences<'a>,
    current_page_: Option<*mut NormalPage>,
    used_bytes_in_current_page_: usize,
    available_pages_: Vec<*mut NormalPage>,
}

impl<'a> CompactionState<'a> {
    fn new(space: &'a mut NormalPageSpace, movable_references: &'a mut MovableReferences<'a>) -> Self {
        CompactionState {
            space_: space,
            movable_references_: movable_references,
            current_page_: None,
            used_bytes_in_current_page_: 0,
            available_pages_: Vec::new(),
        }
    }

    fn add_page(&mut self, page: *mut NormalPage) {
        //assert_eq!(self.space_ as *mut _ , &mut (*page).space() as *mut _);

        if self.current_page_.is_none() {
            self.current_page_ = Some(page);
        } else {
            self.available_pages_.push(page);
        }
    }

    fn relocate_object(&mut self, page: *mut NormalPage, header: usize, size: usize) {
        unsafe {
            let current_page = self.current_page_.unwrap();
            let compact_frontier = (*current_page).payload_start() + self.used_bytes_in_current_page_;
            if compact_frontier + size > (*current_page).payload_end() {
                self.return_current_page_to_space();

                let current_page = self.available_pages_.pop().unwrap();
                self.current_page_ = Some(current_page);
                self.used_bytes_in_current_page_ = 0;
                let compact_frontier = (*current_page).payload_start();
            }

            let current_page = self.current_page_.unwrap();

            if compact_frontier != header {
                if current_page == page {
                    ptr::copy_nonoverlapping(header as *const u8, compact_frontier as *mut u8, size);
                } else {
                    ptr::copy_nonoverlapping(header as *const u8, compact_frontier as *mut u8, size);
                }
                self.movable_references_.relocate(header + mem::size_of::<HeapObjectHeader>(), compact_frontier + mem::size_of::<HeapObjectHeader>(), size);
            }

            (*current_page).object_start_bitmap().set_bit(compact_frontier);
            self.used_bytes_in_current_page_ += size;
            assert!(self.used_bytes_in_current_page_ <= (*current_page).payload_size());
        }
    }

    fn finish_compacting_space(&mut self) {
        if self.used_bytes_in_current_page_ == 0 {
            self.available_pages_.push(self.current_page_.unwrap());
        } else {
            self.return_current_page_to_space();
        }

        for page in &self.available_pages_ {
            unsafe {
                set_memory_inaccessible((*page).payload_start(), (*page).payload_size());
                NormalPage::destroy(*page);
            }
        }
    }

    fn finish_compacting_page(&self, page: *mut NormalPage) {
        #[cfg(any(debug_assertions, feature = "v8_use_memory_sanitizer", feature = "v8_use_address_sanitizer"))]
        unsafe {
            if self.current_page_ != Some(page) {
                zap_memory((*page).payload_start(), (*page).payload_size());
            } else {
                let current_page = self.current_page_.unwrap();
                zap_memory((*page).payload_start() + self.used_bytes_in_current_page_, (*page).payload_size() - self.used_bytes_in_current_page_);
            }
        }
        unsafe {
            (*page).object_start_bitmap().mark_as_fully_populated();
        }
    }

    fn return_current_page_to_space(&mut self) {
        unsafe {
            let current_page = self.current_page_.unwrap();
           // assert_eq!(&mut (*current_page).space() as *mut _, self.space_ as *mut _);
            self.space_.add_page(current_page);

            if self.used_bytes_in_current_page_ != (*current_page).payload_size() {
                let freed_size = (*current_page).payload_size() - self.used_bytes_in_current_page_;
                let payload = (*current_page).payload_start();
                let free_start = payload + self.used_bytes_in_current_page_;
                set_memory_inaccessible(free_start, freed_size);
                self.space_.free_list().add(crate::heap::cppgc::heap_space::FreeListEntry{start: free_start, size: freed_size});
                (*current_page).object_start_bitmap().set_bit(free_start);
            }
        }
    }
}

#[derive(PartialEq, Eq)]
enum StickyBits {
    kEnabled,
    kDisabled
}

fn compact_page(page: *mut NormalPage, compaction_state: &mut CompactionState, sticky_bits: StickyBits) {
    unsafe {
        compaction_state.add_page(page);
        (*page).object_start_bitmap().clear();

        let mut header_address = (*page).payload_start();
        while header_address < (*page).payload_end() {
            let header = header_address as *mut HeapObjectHeader;
            let size = (*header).allocated_size();
            assert!(size > 0);
            assert!(size < kPageSize);

            if (*header).is_free() {
                //ASAN_UNPOISON_MEMORY_REGION(header_address, size);
                header_address += size;
                continue;
            }

            if !(*header).is_marked() {
                (*header).finalize();

                #[cfg(any(debug_assertions, feature = "v8_use_memory_sanitizer", feature = "v8_use_address_sanitizer"))]
                zap_memory(header_address, size);
                header_address += size;
                continue;
            }

            // Object is marked.
            #[cfg(feature = "cppgc_young_generation")]
            if sticky_bits == StickyBits::kDisabled { (*header).unmark(); }
            #[cfg(not(feature = "cppgc_young_generation"))]
            (*header).unmark();

            //ASAN_UNPOISON_MEMORY_REGION((*header).object_start(), (*header).object_size());
            compaction_state.relocate_object(page, header_address, size);
            header_address += size;
        }

        compaction_state.finish_compacting_page(page);
    }
}

fn compact_space(space: &mut NormalPageSpace, movable_references: &mut MovableReferences, sticky_bits: StickyBits) {
    //type Pages = NormalPageSpace::Pages; // Assuming Pages is a type alias for Vec<BasePage*>

    #[cfg(feature = "v8_use_address_sanitizer")]
    UnmarkedObjectsPoisoner{}.traverse(space);

    assert!(space.is_compactable());

    space.free_list().clear();

    let pages = space.remove_all_pages();
    if pages.is_empty() { return; }

    let mut compaction_state = CompactionState::new(space, movable_references);
    for page in pages {
        unsafe {
            (*page).reset_marked_bytes();
            compact_page(crate::heap::cppgc::heap_page::NormalPage::from(page), &mut compaction_state, sticky_bits);
        }
    }

    compaction_state.finish_compacting_space();
}

fn update_heap_residency(spaces: &Vec<&mut NormalPageSpace>) -> usize {
    spaces.iter().fold(0, |acc, space| {
        assert!(space.is_compactable());
        if space.size() == 0 { return acc; }
        acc + space.free_list().size()
    })
}

pub struct Compactor<'a> {
    heap_: &'a mut RawHeap,
    compactable_spaces_: Vec<&'a mut NormalPageSpace>,
    compaction_worklists_: Option<CompactionWorklists>,
    is_enabled_: bool,
    is_cancelled_: bool,
    enable_for_next_gc_for_testing_: bool,
}

impl<'a> Compactor<'a> {
    pub fn new(heap: &'a mut RawHeap) -> Self {
        let mut compactable_spaces_ = Vec::new();
        for space in heap.iter() {
            if let Some(s) = space {
                if s.is_compactable() {
                   // assert_eq!(heap, s.raw_heap());
                    compactable_spaces_.push(unsafe { &mut **s}); // Downcast to NormalPageSpace*
                }
            }
        }
        Compactor {
            heap_: heap,
            compactable_spaces_: compactable_spaces_,
            compaction_worklists_: None,
            is_enabled_: false,
            is_cancelled_: false,
            enable_for_next_gc_for_testing_: false,
        }
    }

    fn should_compact(&self, marking_type: MarkingType, stack_state: StackState) -> bool {
        if self.compactable_spaces_.is_empty() ||
           (marking_type == MarkingType::Atomic && stack_state == StackState::MayContainHeapPointers) {
            assert!(!self.enable_for_next_gc_for_testing_);
            return false;
        }

        if self.enable_for_next_gc_for_testing_ {
            return true;
        }

        let free_list_size = update_heap_residency(&self.compactable_spaces_);
        free_list_size > K_FREE_LIST_SIZE_THRESHOLD
    }

    pub fn initialize_if_should_compact(&mut self, marking_type: MarkingType, stack_state: StackState) {
        assert!(!self.is_enabled_);

        if !self.should_compact(marking_type, stack_state) { return; }

        self.compaction_worklists_ = Some(CompactionWorklists::new());

        self.is_enabled_ = true;
        self.is_cancelled_ = false;
    }

    pub fn cancel_if_should_not_compact(&mut self, marking_type: MarkingType, stack_state: StackState) {
        if !self.is_enabled_ || self.should_compact(marking_type, stack_state) { return; }

        self.is_cancelled_ = true;
        self.is_enabled_ = false;
    }

    pub fn compact_spaces_if_enabled(&mut self) -> CompactableSpaceHandling {
        if self.is_cancelled_ && self.compaction_worklists_.is_some() {
            //let worklist = self.compaction_worklists_.as_mut().unwrap();
           // worklist.movable_slots_worklist().clear(); // Need to implement clear

            self.compaction_worklists_ = None;
        }
        if !self.is_enabled_ { return CompactableSpaceHandling::Sweep; }

        let stats_scope = StatsCollector{}.enabled_scope(&StatsCollector{}, StatsCollector::kAtomicCompact);

        let heap = self.heap_.heap();
        let mut movable_references = MovableReferences::new(heap);
        if let Some(ref worklists) = self.compaction_worklists_ {
            let mut local = worklists.movable_slots_worklist().local();
            let mut slot: *mut usize = 0 as *mut usize;
            while local.pop(&mut slot) {
                movable_references.add_or_filter(slot);
            }
        }
        self.compaction_worklists_ = None;

        let sticky_bits = match self.heap_.heap().sticky_bits() {
            _ => StickyBits::kEnabled
        };

        for space in &mut self.compactable_spaces_ {
            compact_space(space, &mut movable_references, sticky_bits);
        }

        self.enable_for_next_gc_for_testing_ = false;
        self.is_enabled_ = false;
        CompactableSpaceHandling::Ignore
    }

    pub fn enable_for_next_gc_for_testing(&mut self) {
        assert!(self.heap_.heap().marker().is_none());
        self.enable_for_next_gc_for_testing_ = true;
    }
}

#[derive(PartialEq, Eq)]
pub enum CompactableSpaceHandling {
    Sweep,
    Ignore
}

#[derive(PartialEq, Eq)]
pub enum MarkingType {
   Atomic
}

#[derive(PartialEq, Eq)]
pub enum StackState {
   MayContainHeapPointers
}

// Added a stub marker function as it was called in assert in enable_for_next_gc_for_testing
impl RawHeap {
    fn marker(&self) -> Option<usize> {
        None
    }
}

// Added a stub sticky_bits function
impl HeapBase {
    fn sticky_bits(&self) -> usize {
        0
    }
}