// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Note: Corresponding Rust module definitions for header files
// are placed in separate files, for example, page_metadata_inl.rs

use std::ptr::NonNull;

//use crate::heap::page_metadata_inl::*; // Assuming this exists as a Rust module
use crate::heap::heap::*;
use crate::heap::incremental_marking::*;
use crate::heap::paged_spaces::*;
use crate::base::*;
use crate::objects::*;
use crate::marking::*;

pub struct PageMetadata {
    mutable_page_metadata: MutablePageMetadata,
    categories_: Vec<Option<Box<FreeListCategory>>>,
}

impl PageMetadata {
    pub fn new(heap: &mut Heap, space: *mut BaseSpace, size: usize,
               area_start: Address, area_end: Address,
               reservation: VirtualMemory) -> Self {
        let mutable_page_metadata = MutablePageMetadata::new(
            heap,
            space,
            size,
            area_start,
            area_end,
            reservation,
            PageSize::KRegular,
        );

        PageMetadata {
            mutable_page_metadata,
            categories_: Vec::new(),
        }
    }

    pub fn allocate_free_list_categories(&mut self) {
        assert!(self.categories_.is_empty());
        let num_categories = self.owner().free_list().number_of_categories();

        self.categories_.resize(num_categories, None);

        for i in k_first_category()..=self.owner().free_list().last_category() {
            assert!(self.categories_[i].is_none());
            self.categories_[i] = Some(Box::new(FreeListCategory::new()));
        }
    }

    pub fn initialize_free_list_categories(&mut self) {
        for i in k_first_category()..=self.owner().free_list().last_category() {
            if let Some(category) = &mut self.categories_[i] {
                category.initialize(i as FreeListCategoryType);
            }
        }
    }

    pub fn release_free_list_categories(&mut self) {
        if !self.categories_.is_empty() {
            for i in k_first_category()..=self.owner().free_list().last_category() {
                self.categories_[i] = None;
            }
            self.categories_.clear();
        }
    }

    pub fn convert_new_to_old(old_page: &mut PageMetadata) -> *mut PageMetadata {
        assert!(old_page != (0 as *mut PageMetadata));
        let chunk = old_page.chunk();
        assert!(chunk.in_new_space());
        old_page.reset_age_in_new_space();

        let old_space = old_page.heap().old_space();
        old_page.set_owner(old_space);
        chunk.clear_flags_non_executable(MemoryChunk::K_ALL_FLAGS_MASK);
        assert_ne!(old_space.identity(), SpaceIdentity::SharedSpace);

        chunk.set_old_generation_page_flags(
            old_page.heap().incremental_marking().marking_mode(),
            OldGenerationPageFlag::OldSpace
        );

        let new_page = old_space.initialize_page(old_page);
        old_space.add_promoted_page(new_page);

        new_page
    }

    pub fn available_in_free_list(&self) -> usize {
        let mut sum = 0;
        self.for_all_free_list_categories(|category| sum += category.available());
        sum
    }

    pub fn mark_never_allocate_for_testing(&mut self) {
        let chunk = self.chunk();
        assert!(self.owner_identity() != SpaceIdentity::NewSpace);
        assert!(!chunk.is_flag_set(MemoryChunk::NEVER_ALLOCATE_ON_PAGE));

        chunk.set_flag_slow(MemoryChunk::NEVER_ALLOCATE_ON_PAGE);
        chunk.set_flag_slow(MemoryChunk::NEVER_EVACUATE);
        let paged_space = unsafe { &mut *(self.owner() as *mut BaseSpace as *mut PagedSpace) };
        paged_space.free_list().evict_free_list_items(self);
    }

    #[cfg(debug_assertions)]
    fn skip_fillers(cage_base: PtrComprCageBase, mut filler: Tagged<HeapObject>, end: Address) -> Address {
        let mut addr = filler.address();
        while addr < end {
            filler = HeapObject::from_address(addr);
            assert!(is_free_space_or_filler(filler, cage_base));
            addr = filler.address() + filler.size(cage_base);
        }
        addr
    }

    pub fn shrink_to_high_water_mark(&mut self) -> usize {
        let reservation = self.reserved_memory();
        if !reservation.is_reserved() {
            return 0;
        }

        let filler = HeapObject::from_address(self.high_water_mark());
        if filler.address() == self.area_end() {
            return 0;
        }
        let cage_base = PtrComprCageBase::new(self.heap().isolate());
        assert!(is_free_space_or_filler(filler, cage_base));
        assert_eq!(self.area_end(), Self::skip_fillers(cage_base, filler, self.area_end()));
        assert_eq!(0, self.available_in_free_list());

        assert!(self.slot_set::<OLD_TO_NEW>().is_none());
        assert!(self.slot_set::<OLD_TO_NEW_BACKGROUND>().is_none());
        assert!(self.slot_set::<OLD_TO_OLD>().is_none());

        let unused = round_down(
            (self.area_end() - filler.address()) as usize,
            MemoryAllocator::get_commit_page_size()
        );

        if unused > 0 {
            assert_eq!(0, unused % MemoryAllocator::get_commit_page_size());

            if v8_flags::trace_gc_verbose {
                print_isolate(
                    self.heap().isolate(),
                    format!("Shrinking page {:p}: end {:p} -> {:p}\n",
                            self, self.area_end(), self.area_end() - unused).as_str()
                );
            }

            self.heap().create_filler_object_at(
                filler.address(),
                (self.area_end() - filler.address() - unused) as i32
            );

            self.heap().memory_allocator().partial_free_memory(
                self,
                self.chunk_address() + self.size() - unused,
                unused,
                self.area_end() - unused
            );

            if filler.address() != self.area_end() {
                assert!(is_free_space_or_filler(filler, cage_base));
                assert_eq!(filler.address() + filler.size(cage_base), self.area_end());
            }
        }
        unused
    }

    pub fn create_black_area(&mut self, start: Address, end: Address) {
        assert!(!v8_flags::black_allocated_pages());
        assert_ne!(SpaceIdentity::NewSpace, self.owner_identity());
        assert!(v8_flags::sticky_mark_bits() || self.heap().incremental_marking().black_allocation());
        assert_eq!(PageMetadata::from_address(start), self);
        assert!(start < end);
        assert_eq!(PageMetadata::from_address(end - 1), self);

        self.marking_bitmap().set_range::<AccessMode::Atomic>(
            MarkingBitmap::address_to_index(start),
            MarkingBitmap::limit_address_to_index(end)
        );

        self.increment_live_bytes_atomically((end - start) as isize);
        self.owner().notify_black_area_created(end - start);
    }

    pub fn destroy_black_area(&mut self, start: Address, end: Address) {
        assert!(!v8_flags::black_allocated_pages());
        assert_ne!(SpaceIdentity::NewSpace, self.owner_identity());
        assert!(v8_flags::sticky_mark_bits() || self.heap().incremental_marking().black_allocation());
        assert_eq!(PageMetadata::from_address(start), self);
        assert!(start < end);
        assert_eq!(PageMetadata::from_address(end - 1), self);

        self.marking_bitmap().clear_range::<AccessMode::Atomic>(
            MarkingBitmap::address_to_index(start),
            MarkingBitmap::limit_address_to_index(end)
        );

        self.increment_live_bytes_atomically(-(end - start) as isize);
        self.owner().notify_black_area_destroyed(end - start);
    }

    fn for_all_free_list_categories<F>(&self, mut f: F)
        where F: FnMut(&FreeListCategory) {
        for i in k_first_category()..=self.owner().free_list().last_category() {
            if let Some(category) = &self.categories_[i] {
                f(category);
            }
        }
    }

    // Proxy methods to the MutablePageMetadata
    pub fn owner(&self) -> &mut BaseSpace {
        unsafe {&mut *self.mutable_page_metadata.owner }
    }

    pub fn owner_identity(&self) -> SpaceIdentity {
        self.mutable_page_metadata.owner_identity()
    }

    pub fn set_owner(&mut self, owner: *mut BaseSpace) {
        self.mutable_page_metadata.set_owner(owner);
    }

    pub fn chunk(&mut self) -> &mut MemoryChunk {
        self.mutable_page_metadata.chunk()
    }

    pub fn reset_age_in_new_space(&mut self) {
        self.mutable_page_metadata.reset_age_in_new_space();
    }

    pub fn heap(&mut self) -> &mut Heap {
        self.mutable_page_metadata.heap
    }

    pub fn high_water_mark(&self) -> Address {
        self.mutable_page_metadata.high_water_mark
    }

    pub fn area_end(&self) -> Address {
        self.mutable_page_metadata.area_end
    }

    pub fn slot_set<T: SlotSetType>(&self) -> Option<&SlotSet> {
        self.mutable_page_metadata.slot_set::<T>()
    }

    pub fn reserved_memory(&self) -> &VirtualMemory {
        &self.mutable_page_metadata.reservation
    }

    pub fn chunk_address(&self) -> Address {
        self.mutable_page_metadata.area_start
    }

    pub fn size(&self) -> usize {
        self.mutable_page_metadata.size
    }

    pub fn marking_bitmap(&mut self) -> &mut MarkingBitmap<AccessMode::Atomic> {
        self.mutable_page_metadata.marking_bitmap()
    }

    pub fn increment_live_bytes_atomically(&mut self, bytes: isize) {
        self.mutable_page_metadata.increment_live_bytes_atomically(bytes);
    }

    pub fn from_address(address: Address) -> *mut PageMetadata {
        MutablePageMetadata::from_address(address) as *mut PageMetadata
    }

    //Note: Implement getters for other MutablePageMetadata fields if needed

}

trait SlotSetType {}

struct OLD_TO_NEW;
impl SlotSetType for OLD_TO_NEW {}

struct OLD_TO_NEW_BACKGROUND;
impl SlotSetType for OLD_TO_NEW_BACKGROUND {}

struct OLD_TO_OLD;
impl SlotSetType for OLD_TO_OLD {}

// Dummy implementations for dependencies
mod base {
    pub type Address = usize;
    pub fn round_down(value: usize, alignment: usize) -> usize {
        value & !(alignment - 1)
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum AccessMode {
        ATOMIC
    }
}

mod objects {
    use crate::base::Address;

    #[derive(Clone, Copy)]
    pub struct Tagged<T> {
        address: Address,
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> Tagged<T> {
        pub fn from_address(address: Address) -> Self {
            Tagged { address, _phantom: std::marker::PhantomData }
        }

        pub fn address(&self) -> Address {
            self.address
        }

        pub fn size(&self, _cage_base: PtrComprCageBase) -> Address {
            // Dummy implementation, replace with actual logic
            8
        }
    }

    pub struct HeapObject {}
}

mod heap {
    use crate::base::Address;
    use crate::objects::Tagged;

    pub struct Heap {
        old_space_: OldSpace,
        incremental_marking_: IncrementalMarking,
        memory_allocator_: MemoryAllocator,
        isolate_: Isolate,
    }

    impl Heap {
        pub fn old_space(&mut self) -> &mut OldSpace {
            &mut self.old_space_
        }

        pub fn incremental_marking(&mut self) -> &IncrementalMarking {
            &self.incremental_marking_
        }

        pub fn memory_allocator(&mut self) -> &mut MemoryAllocator {
            &mut self.memory_allocator_
        }

        pub fn create_filler_object_at(&mut self, _address: Address, _size: i32) {
            // Implementation for creating filler object
        }

        pub fn isolate(&mut self) -> &mut Isolate {
            &mut self.isolate_
        }
    }

    pub struct OldSpace {}

    impl OldSpace {
        pub fn identity(&self) -> SpaceIdentity {
            SpaceIdentity::OldSpace //Dummy
        }

        pub fn initialize_page(&mut self, _old_page: &mut super::PageMetadata) -> *mut super::PageMetadata {
           0 as *mut super::PageMetadata // Dummy
        }

        pub fn add_promoted_page(&mut self, _new_page: *mut super::PageMetadata) {
            //Dummy impl
        }
    }

    pub struct IncrementalMarking {
        marking_mode_: MarkingMode
    }

    impl IncrementalMarking {
        pub fn marking_mode(&self) -> MarkingMode {
            self.marking_mode_ //Dummy
        }

        pub fn black_allocation(&self) -> bool {
            false //Dummy
        }
    }

    pub struct MemoryAllocator {}

    impl MemoryAllocator {
        pub fn get_commit_page_size() -> usize {
            4096 //Dummy
        }

        pub fn partial_free_memory(
            &mut self,
            _page: &mut super::PageMetadata,
            _address: Address,
            _size: usize,
            _new_end: Address
        ) {
            //Dummy impl
        }
    }

    pub struct FreeList {
        number_of_categories_: usize,
        last_category_: usize
    }

    impl FreeList {
        pub fn number_of_categories(&self) -> usize {
            self.number_of_categories_ //Dummy
        }

        pub fn last_category(&self) -> usize {
            self.last_category_ //Dummy
        }

        pub fn evict_free_list_items(&mut self, _page: &mut super::PageMetadata) {
            //Dummy impl
        }
    }

    pub struct Isolate {}

    pub fn print_isolate(_isolate: &mut Isolate, _format_string: &str) {
        //Dummy impl
    }
}

mod incremental_marking {
    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum MarkingMode {
        kIncrementalMarking
    }
}

mod paged_spaces {
    use crate::heap::FreeList;

    pub struct PagedSpace {
        free_list_: FreeList
    }

    impl PagedSpace {
        pub fn free_list(&mut self) -> &mut FreeList {
            &mut self.free_list_ //Dummy
        }
    }
}

mod marking {
    use crate::base::{Address, AccessMode};

    pub struct MarkingBitmap<const MODE: AccessMode> {}

    impl<const MODE: AccessMode> MarkingBitmap<MODE> {
        pub fn set_range<const MODE2: AccessMode>(
            &mut self,
            _start: usize,
            _end: usize
        ) {
            //Dummy impl
        }

        pub fn clear_range<const MODE2: AccessMode>(
            &mut self,
            _start: usize,
            _end: usize
        ) {
            //Dummy impl
        }

        pub fn address_to_index(_address: Address) -> usize {
            0 //Dummy
        }

        pub fn limit_address_to_index(_address: Address) -> usize {
            0 //Dummy
        }
    }
}

mod v8_flags {
    pub fn black_allocated_pages() -> bool {
        false //Dummy
    }

    pub fn sticky_mark_bits() -> bool {
        false //Dummy
    }

    pub fn trace_gc_verbose() -> bool {
        false //Dummy
    }
}

mod page_metadata_inl {
    use crate::heap::Heap;
    use crate::base::Address;
    use crate::marking::*;
    use crate::base::AccessMode;

    pub struct MutablePageMetadata {
        pub heap: *mut Heap,
        pub owner: *mut super::BaseSpace,
        pub size: usize,
        pub area_start: Address,
        pub area_end: Address,
        pub high_water_mark: Address,
        pub reservation: VirtualMemory,
        //NOTE: Add all fields as they appear in the original C++ code.
    }

    impl MutablePageMetadata {
        pub fn new(heap: &mut Heap, owner: *mut super::BaseSpace, size: usize,
               area_start: Address, area_end: Address,
               reservation: VirtualMemory, page_size: PageSize) -> Self {
                MutablePageMetadata{
                    heap,
                    owner,
                    size,
                    area_start,
                    area_end,
                    high_water_mark: area_start,
                    reservation,
                }
        }

        pub fn owner_identity(&self) -> SpaceIdentity {
            SpaceIdentity::OldSpace
        }

        pub fn set_owner(&mut self, owner: *mut super::BaseSpace) {
            self.owner = owner;
        }

        pub fn chunk(&mut self) -> &mut super::MemoryChunk {
            //Dummy impl
            unsafe { &mut *(0 as *mut super::MemoryChunk) }
        }

        pub fn reset_age_in_new_space(&mut self) {
            //Dummy impl
        }

        pub fn slot_set<T: super::SlotSetType>(&self) -> Option<&SlotSet> {
            None //Dummy impl
        }

        pub fn increment_live_bytes_atomically(&mut self, _bytes: isize) {
            //Dummy impl
        }

        pub fn marking_bitmap(&mut self) -> &mut MarkingBitmap<AccessMode::ATOMIC> {
            //Dummy impl
            unsafe { &mut *(0 as *mut MarkingBitmap<AccessMode::ATOMIC>)}
        }

        pub fn from_address(address: Address) -> *mut MutablePageMetadata {
            address as *mut MutablePageMetadata
        }
    }

    pub enum PageSize {
        KRegular
    }

    pub struct VirtualMemory {
    }

    impl VirtualMemory {
        pub fn is_reserved(&self) -> bool {
            false // Dummy impl
        }
    }

    pub struct SlotSet {}
}

// Dummy implementations
#[derive(PartialEq, Eq)]
pub enum SpaceIdentity {
    NewSpace,
    OldSpace,
    SharedSpace
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum OldGenerationPageFlag {
    OldSpace
}

pub struct MemoryChunk {}

impl MemoryChunk {
    pub const K_ALL_FLAGS_MASK: i32 = 0;
    pub const NEVER_ALLOCATE_ON_PAGE: i32 = 0;
    pub const NEVER_EVACUATE: i32 = 0;

    pub fn in_new_space(&self) -> bool {
        false //Dummy
    }

    pub fn clear_flags_non_executable(&mut self, _mask: i32) {
        //Dummy
    }

    pub fn set_old_generation_page_flags(&mut self, _marking_mode: incremental_marking::MarkingMode, _old_space: OldGenerationPageFlag) {
        //Dummy
    }

    pub fn is_flag_set(&self, _flag: i32) -> bool {
        false //Dummy
    }

    pub fn set_flag_slow(&mut self, _flag: i32) {
        //Dummy
    }
}

pub struct FreeListCategory {}

impl FreeListCategory {
    pub fn new() -> Self {
        FreeListCategory {}
    }
    pub fn initialize(&mut self, _category_type: FreeListCategoryType) {
        //Dummy impl
    }
    pub fn available(&self) -> usize {
        0 //Dummy impl
    }
}

pub type FreeListCategoryType = usize;

fn is_free_space_or_filler(_object: Tagged<HeapObject>, _cage_base: PtrComprCageBase) -> bool {
    false //Dummy impl
}

pub struct PtrComprCageBase {
    isolate: *mut Heap::Isolate
}

impl PtrComprCageBase {
    pub fn new(isolate: *mut Heap::Isolate) -> Self {
        PtrComprCageBase{isolate}
    }
}

// Dummy constants
const fn k_first_category() -> usize {
    0
}