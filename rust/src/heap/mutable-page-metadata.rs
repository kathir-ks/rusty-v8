// src/heap/mutable_page_metadata.rs

use std::ptr::null_mut;
use std::sync::atomic::{AtomicPtr, Ordering};

// use crate::base::logging; // TODO: Implement logging
use crate::heap::active_system_pages::ActiveSystemPages;
use crate::heap::incremental_marking::IncrementalMarking;
use crate::heap::marking_state::MarkingBitmap;
use crate::heap::memory_allocator::MemoryAllocator;
use crate::heap::memory_chunk::MemoryChunk;
use crate::heap::memory_chunk_layout::PageSize;
use crate::heap::memory_chunk_metadata::MemoryChunkMetadata;
use crate::heap::page_metadata::PageMetadata;
use crate::heap::spaces::BaseSpace;
use crate::objects::heap_object::HeapObject;
use crate::Address;
use crate::common::globals::FLAG_trace_gc_verbose; //TODO: Remove dependency to globals module

// TODO: Define RememberedSetType enum
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RememberedSetType {
    OLD_TO_NEW,
    OLD_TO_NEW_BACKGROUND,
    OLD_TO_OLD,
    TRUSTED_TO_CODE,
    OLD_TO_SHARED,
    TRUSTED_TO_TRUSTED,
    TRUSTED_TO_SHARED_TRUSTED,
    SURVIVOR_TO_EXTERNAL_POINTER,
    NUMBER_OF_REMEMBERED_SET_TYPES,
}

//Helper enum for enum conversions
pub enum Executability {
    EXECUTABLE,
    NOT_EXECUTABLE,
}

pub struct Heap {
    incremental_marking: IncrementalMarking,
}

impl Heap {
    pub fn incremental_marking(&self) -> &IncrementalMarking {
        &self.incremental_marking
    }
}

// TODO: Define SlotSet struct
pub struct SlotSet {}

impl SlotSet {
    pub fn allocate(_buckets_in_slot_set: usize) -> *mut SlotSet {
        Box::into_raw(Box::new(SlotSet {}))
    }
    pub fn delete(slot_set: *mut SlotSet) {
        unsafe {
            if !slot_set.is_null() {
                drop(Box::from_raw(slot_set));
            }
        }
    }
}

// TODO: Define TypedSlotSet struct
pub struct TypedSlotSet {
    chunk_address: Address,
}

impl TypedSlotSet {
    pub fn new(chunk_address: Address) -> Self {
        TypedSlotSet { chunk_address }
    }
}

// TODO: Define FreeListCategory struct
pub struct FreeListCategory {}

//TODO: Replace hardware_destructive_interference_size with constant value because it is not a stable feature
const HARDWARE_DESTRUCTIVE_INTERFERENCE_SIZE: usize = 64;

pub struct MutablePageMetadata {
    memory_chunk_metadata: MemoryChunkMetadata,
    heap_: *mut Heap,
    slot_set_: [AtomicPtr<SlotSet>; RememberedSetType::NUMBER_OF_REMEMBERED_SET_TYPES as usize],
    typed_slot_set_: [AtomicPtr<TypedSlotSet>; RememberedSetType::NUMBER_OF_REMEMBERED_SET_TYPES as usize],
    active_system_pages_: Option<Box<ActiveSystemPages>>,
    possibly_empty_buckets_: PossiblyEmptyBuckets, // TODO: Implement PossiblyEmptyBuckets
    categories_: [*mut FreeListCategory; 16], // TODO: Determine the correct size of this array dynamically, using `owner()->free_list()->last_category()`
}

struct PossiblyEmptyBuckets {} // Dummy struct

impl MutablePageMetadata {
    pub fn new(
        heap: *mut Heap,
        space: *mut BaseSpace,
        chunk_size: usize,
        area_start: Address,
        area_end: Address,
        reservation: VirtualMemory,
        page_size: PageSize,
    ) -> Self {
        //DCHECK_NE(space.identity(), RO_SPACE);
        let mut active_system_pages = None;
        if page_size == PageSize::kRegular {
            let mut asp = Box::new(ActiveSystemPages::new());
            asp.init(
                std::mem::size_of::<MemoryChunk>(),
                MemoryAllocator::get_commit_page_size_bits(),
                chunk_size,
            );
            active_system_pages = Some(asp);
        }

        // We do not track active system pages for large pages and use this fact for
        // `IsLargePage()`.
        //DCHECK_EQ(page_size == PageSize::kLarge, IsLargePage());

        let mut metadata = MutablePageMetadata {
            memory_chunk_metadata: MemoryChunkMetadata::new(heap, space, chunk_size, area_start, area_end, reservation),
            heap_: heap,
            slot_set_: [
                AtomicPtr::new(null_mut());
                RememberedSetType::NUMBER_OF_REMEMBERED_SET_TYPES as usize
            ],
            typed_slot_set_: [
                AtomicPtr::new(null_mut());
                RememberedSetType::NUMBER_OF_REMEMBERED_SET_TYPES as usize
            ],
            active_system_pages_: active_system_pages,
            possibly_empty_buckets_: PossiblyEmptyBuckets {},
            categories_: [null_mut(); 16], // Dummy initialization
        };

        // TODO(sroettger): The following fields are accessed most often (AFAICT) and
        // are moved to the end to occupy the same cache line as the slot set array.
        // Without this change, there was a 0.5% performance impact after cache line
        // aligning the metadata on x64 (before, the metadata started at offset 0x10).
        // After reordering, the impact is still 0.1%/0.2% on jetstream2/speedometer3,
        // so there should be some more optimization potential here.
        // TODO(mlippautz): Replace 64 below with
        // `hardware_destructive_interference_size` once supported.
        // const K_OFFSET_OF_FIRST_FAST_FIELD: usize = offsetof!(MutablePageMetadata, heap_);
        // const K_OFFSET_OF_LAST_FAST_FIELD: usize = offsetof!(MutablePageMetadata, slot_set_) +
        //     size_of::<*mut SlotSet>() * RememberedSetType::OLD_TO_NEW as usize;
        // This assert is merely necessary but not sufficient to guarantee that the
        // fields sit on the same cacheline as the metadata object itself is
        // dynamically allocated without alignment restrictions.
        // static_assert(K_OFFSET_OF_FIRST_FAST_FIELD / 64 == K_OFFSET_OF_LAST_FAST_FIELD / 64);

        metadata
    }

    pub fn initial_flags(&self, executable: Executability) -> MemoryChunk::MainThreadFlags {
        let mut flags = MemoryChunk::MainThreadFlags::NO_FLAGS;

        let owner_identity = unsafe { (*self.memory_chunk_metadata.owner()).identity() };

        if owner_identity == Identity::NEW_SPACE || owner_identity == Identity::NEW_LO_SPACE {
            flags |= MemoryChunk::young_generation_page_flags(unsafe {
                (*(*self.heap_).incremental_marking()).marking_mode()
            });
        } else {
            flags |= MemoryChunk::old_generation_page_flags(
                unsafe { (*(*self.heap_).incremental_marking()).marking_mode() },
                owner_identity,
            );
        }

        if executable == Executability::EXECUTABLE {
            flags |= MemoryChunk::MainThreadFlags::IS_EXECUTABLE;
            // Executable chunks are also trusted as they contain machine code and live
            // outside the sandbox (when it is enabled). While mostly symbolic, this is
            // needed for two reasons:
            // 1. We have the invariant that IsTrustedObject(obj) implies
            //    IsTrustedSpaceObject(obj), where IsTrustedSpaceObject checks the
            //   MemoryChunk::IS_TRUSTED flag on the host chunk. As InstructionStream
            //   objects are
            //    trusted, their host chunks must also be marked as such.
            // 2. References between trusted objects must use the TRUSTED_TO_TRUSTED
            //    remembered set. However, that will only be used if both the host
            //    and the value chunk are marked as IS_TRUSTED.
            flags |= MemoryChunk::MainThreadFlags::IS_TRUSTED;
        }

        // All pages of a shared heap need to be marked with this flag.
        if self.in_shared_space() {
            flags |= MemoryChunk::MainThreadFlags::IN_WRITABLE_SHARED_SPACE;
        }

        // All pages belonging to a trusted space need to be marked with this flag.
        if self.in_trusted_space() {
            flags |= MemoryChunk::MainThreadFlags::IS_TRUSTED;
        }

        // "Trusted" chunks should never be located inside the sandbox as they
        // couldn't be trusted in that case.
        //DCHECK_IMPLIES(flags & MemoryChunk::IS_TRUSTED,
        //               !InsideSandbox(ChunkAddress()));

        flags
    }

    pub fn committed_physical_memory(&self) -> usize {
        if !has_lazy_commits() || self.chunk().is_large_page() {
            return self.size();
        }
        self.active_system_pages_.as_ref().unwrap().size(MemoryAllocator::get_commit_page_size_bits())
    }

    pub fn release_allocated_memory_needed_for_writable_chunk(&mut self) {
        //DCHECK(SweepingDone());
        if FLAG_trace_gc_verbose {
            println!("release_allocated_memory_needed_for_writable_chunk");
        }
        self.active_system_pages_ = None;

        self.possibly_empty_buckets_.release();
        self.release_slot_set(RememberedSetType::OLD_TO_NEW);
        self.release_slot_set(RememberedSetType::OLD_TO_NEW_BACKGROUND);
        self.release_slot_set(RememberedSetType::OLD_TO_OLD);
        self.release_slot_set(RememberedSetType::TRUSTED_TO_CODE);
        self.release_slot_set(RememberedSetType::OLD_TO_SHARED);
        self.release_slot_set(RememberedSetType::TRUSTED_TO_TRUSTED);
        self.release_slot_set(RememberedSetType::TRUSTED_TO_SHARED_TRUSTED);
        self.release_slot_set(RememberedSetType::SURVIVOR_TO_EXTERNAL_POINTER);
        self.release_typed_slot_set(RememberedSetType::OLD_TO_NEW);
        self.release_typed_slot_set(RememberedSetType::OLD_TO_OLD);
        self.release_typed_slot_set(RememberedSetType::OLD_TO_SHARED);

        if !self.chunk().is_large_page() {
            let page = self as *mut Self as *mut PageMetadata;
            unsafe {
                (*page).release_free_list_categories();
            }
        }
    }

    pub fn release_all_allocated_memory(&mut self) {
        self.release_allocated_memory_needed_for_writable_chunk();
    }

    pub fn allocate_slot_set(&self, type_: RememberedSetType) -> *mut SlotSet {
        let new_slot_set = SlotSet::allocate(self.buckets_in_slot_set());
        let old_slot_set = unsafe {
            self.slot_set_[type_ as usize].compare_exchange(
                null_mut(),
                new_slot_set,
                Ordering::AcqRel,
                Ordering::Relaxed,
            )
        };
        match old_slot_set {
            Ok(_) => {}
            Err(old_slot_set) => {
                SlotSet::delete(new_slot_set);
                return old_slot_set;
            }
        }
        //DCHECK_NOT_NULL(new_slot_set);
        new_slot_set
    }

    pub fn release_slot_set(&self, type_: RememberedSetType) {
        let slot_set = self.slot_set_[type_ as usize].load(Ordering::Acquire);
        if !slot_set.is_null() {
            self.slot_set_[type_ as usize].store(null_mut(), Ordering::Release);
            SlotSet::delete(slot_set);
        }
    }

    pub fn allocate_typed_slot_set(&self, type_: RememberedSetType) -> *mut TypedSlotSet {
        let typed_slot_set = Box::into_raw(Box::new(TypedSlotSet::new(self.chunk_address())));
        let old_value = unsafe {
            self.typed_slot_set_[type_ as usize].compare_exchange(
                null_mut(),
                typed_slot_set,
                Ordering::AcqRel,
                Ordering::Relaxed,
            )
        };

        match old_value {
            Ok(_) => {}
            Err(old_value) => {
                unsafe { drop(Box::from_raw(typed_slot_set)); }
                return old_value;
            }
        }

        //DCHECK(typed_slot_set);
        typed_slot_set
    }

    pub fn release_typed_slot_set(&self, type_: RememberedSetType) {
        let typed_slot_set = self.typed_slot_set_[type_ as usize].load(Ordering::Acquire);
        if !typed_slot_set.is_null() {
            self.typed_slot_set_[type_ as usize].store(null_mut(), Ordering::Release);
            unsafe { drop(Box::from_raw(typed_slot_set)); }
        }
    }

    pub fn contains_any_slots(&self) -> bool {
        for rs_type in 0..RememberedSetType::NUMBER_OF_REMEMBERED_SET_TYPES as usize {
            if !self.slot_set_[rs_type].load(Ordering::Acquire).is_null() || !self.typed_slot_set_[rs_type].load(Ordering::Acquire).is_null() {
                return true;
            }
        }
        false
    }

    pub fn compute_free_lists_length(&self) -> i32 {
        let mut length: i32 = 0;
        let owner = self.owner();
        let first_category = 0; //TODO: kFirstCategory
        let last_category = unsafe { (*owner).free_list().last_category() };

        for cat in first_category..=last_category {
            if !self.categories_[cat as usize].is_null() {
                unsafe {
                    length += (*self.categories_[cat as usize]).free_list_length();
                }
            }
        }
        length
    }

    pub fn is_liveness_clear(&self) -> bool {
        //CHECK_IMPLIES(marking_bitmap()->IsClean(), live_bytes() == 0);
        //TODO: implement IsClean()
        self.marking_bitmap().is_clean()
    }

    pub fn owner(&self) -> *mut BaseSpace {
        self.memory_chunk_metadata.owner()
    }

    pub fn chunk_address(&self) -> Address {
        self.memory_chunk_metadata.chunk_address()
    }

    fn in_shared_space(&self) -> bool {
        // Dummy implementation
        false
    }

    fn in_trusted_space(&self) -> bool {
        // Dummy implementation
        false
    }

    fn chunk(&self) -> &MemoryChunk {
        self.memory_chunk_metadata.chunk()
    }

    fn size(&self) -> usize {
        self.memory_chunk_metadata.size()
    }

    fn buckets_in_slot_set(&self) -> usize {
        0 //Dummy Implementation
    }

    fn marking_bitmap(&self) -> &MarkingBitmap {
        self.memory_chunk_metadata.marking_bitmap() // Dummy implementation
    }
}

// Implement traits from MemoryChunkMetadata
impl MutablePageMetadata {
    fn identity(&self) -> Identity {
        self.memory_chunk_metadata.identity()
    }
}

// TODO: Implement ActiveSystemPages struct and methods
// struct ActiveSystemPages {}

// TODO: Implement PossiblyEmptyBuckets methods
impl PossiblyEmptyBuckets {
    fn release(&mut self) {}
}

// TODO: Implement FreeListCategory methods
impl FreeListCategory {
    fn free_list_length(&self) -> i32 {
        0
    }
}

// Dummy Implementations for supporting structs
struct VirtualMemory {}

// Implement traits for supporting structs
impl MemoryChunk {
    pub enum MainThreadFlags {
        NO_FLAGS = 0,
        IS_EXECUTABLE = 1,
        IS_TRUSTED = 1 << 1,
        IN_WRITABLE_SHARED_SPACE = 1 << 2,
    }

    fn young_generation_page_flags(_marking_mode: i32) -> Self::MainThreadFlags {
        Self::MainThreadFlags::NO_FLAGS
    }

    fn old_generation_page_flags(_marking_mode: i32, _owner_identity: Identity) -> Self::MainThreadFlags {
        Self::MainThreadFlags::NO_FLAGS
    }

    fn is_large_page(&self) -> bool {
        false
    }
}

impl MemoryChunkMetadata {
    fn owner(&self) -> *mut BaseSpace {
        null_mut() // Dummy implementation
    }

    fn chunk(&self) -> &MemoryChunk {
        todo!()
    }
}

pub enum Identity {
    NEW_SPACE,
    NEW_LO_SPACE,
    OLD_SPACE,
    CODE_SPACE,
    MAP_SPACE,
    LO_SPACE,
    RO_SPACE,
}

impl BaseSpace {
    fn free_list(&self) -> &FreeList {
        todo!()
    }

    fn identity(&self) -> Identity {
        todo!()
    }
}

pub struct FreeList {}

impl FreeList {
    fn last_category(&self) -> i32 {
        0
    }
}

impl MarkingBitmap {
    fn is_clean(&self) -> bool {
        false
    }
}

impl PageMetadata {
    unsafe fn release_free_list_categories(&mut self) {}
}

fn has_lazy_commits() -> bool {
    false
}