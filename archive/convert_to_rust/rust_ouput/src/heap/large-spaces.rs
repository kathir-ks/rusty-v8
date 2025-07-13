// Converted from V8 C++ source files:
// Header: large-spaces.h
// Implementation: large-spaces.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::sync::{Mutex, RecursiveMutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::LinkedList;
use crate::V8_EXPORT_PRIVATE;
use crate::V8_WARN_UNUSED_RESULT;
use crate::Isolate;
use crate::LocalHeap;
use crate::Heap;
use crate::Address;
use crate::Tagged;
use crate::HeapObject;
use crate::Space;
use crate::ObjectIterator;
use crate::String;
use crate::MaybeObject;
use crate::PtrComprCageBase;

pub struct LargePageIterator {}
pub struct ConstLargePageIterator {}
pub struct AllocationResult {}
pub struct Executability {}
pub struct AllocationOrigin {}
pub struct GCType {}

enum ExternalBackingStoreType {
    kArrayBuffer,
    kWasmMemory,
    kNumValues,
}

trait ForAllHelper {
    fn for_all<F>(f: F) where F: Fn(ExternalBackingStoreType, i32);
}

impl ForAllHelper for ExternalBackingStoreType {
    fn for_all<F>(f: F)
    where
        F: Fn(ExternalBackingStoreType, i32),
    {
        f(ExternalBackingStoreType::kArrayBuffer, 0);
        f(ExternalBackingStoreType::kWasmMemory, 1);
    }
}

macro_rules! ForAll {
    ($callback:expr) => {
        ExternalBackingStoreType::for_all($callback);
    };
}

struct AllocationCounter {}
impl AllocationCounter {
    fn AddAllocationObserver(&mut self, observer: &AllocationObserver) {}
    fn RemoveAllocationObserver(&mut self, observer: &AllocationObserver) {}
    fn AdvanceAllocationObservers(&mut self, object_size: usize) {}
    fn InvokeAllocationObservers(&mut self, soon_object: Address, object_size: usize, size: usize) {}
    fn NextBytes(&self) -> usize { 0 }
    fn HasAllocationObservers(&self) -> bool { false }
}
struct AllocationObserver {}
pub struct LargePageMetadata {}
impl LargePageMetadata {
    fn GetObject(&self) -> Tagged<HeapObject> { Tagged::<HeapObject>{} }
    fn Chunk(&self) -> *mut MemoryChunk {std::ptr::null_mut()}
    fn area_size(&self) -> usize { 0 }
    fn size(&self) -> usize { 0 }
    fn area_start(&self) -> Address { Address{} }
    fn next_page(&self) -> *mut LargePageMetadata { std::ptr::null_mut() }
    fn set_owner(&mut self, space: *mut LargeObjectSpace) {}
    fn owner_identity(&self) -> AllocationSpace {AllocationSpace::LO_SPACE}
    fn ExternalBackingStoreBytes(&self, _type: ExternalBackingStoreType) -> usize {0}
    fn ClearOutOfLiveRangeSlots(&mut self, _address: Address) {}
    fn set_area_end(&mut self, _address: Address) {}
    fn ClearLiveness(&mut self) {}
    fn ChunkAddress(&self) -> Address { Address{} }
}

struct MemoryChunkMetadata {}
impl MemoryChunkMetadata {
    fn FromHeapObject(_object: Tagged<HeapObject>) -> *mut MemoryChunkMetadata {std::ptr::null_mut()}
    fn owner(&self) -> *mut LargeObjectSpace {std::ptr::null_mut()}
}
struct MemoryChunk {}
impl MemoryChunk {
    fn FromAddress(_address: Address) -> *mut MemoryChunk {std::ptr::null_mut()}
    fn SetFlagNonExecutable(&mut self, _flag: i32) {}
    fn ClearFlagNonExecutable(&mut self, _flag: i32) {}
    fn InitializationMemoryFence(&mut self) {}
    fn IsLargePage(&self) -> bool {false}
    fn SetOldGenerationPageFlags(&mut self, _marking_mode: bool, _lo_space: AllocationSpace) {}
    fn IsFlagSet(&self, _flag: i32) -> bool {false}
    fn address(&self) -> Address { Address{} }
    fn executable(&self) -> Executability { Executability{} }
    fn Offset(&self, _address: Address) -> usize { 0 }
}
struct PageMetadata {}
impl PageMetadata {
    fn FromHeapObject(_object: Tagged<HeapObject>) -> *mut PageMetadata {std::ptr::null_mut()}
    fn area_start(&self) -> Address { Address{} }
}

pub struct LargeObjectSpace {
    heap_: *mut Heap,
    id_: AllocationSpace,
    memory_chunk_list_: LinkedList<*mut LargePageMetadata>,
    size_: AtomicUsize,
    page_count_: i32,
    objects_size_: AtomicUsize,
    allocation_mutex_: RecursiveMutex,
    pending_object_: AtomicUsize,
    pending_allocation_mutex_: Mutex<()>,
    allocation_counter_: AllocationCounter,
}

impl LargeObjectSpace {
    pub fn new(heap: *mut Heap, id: AllocationSpace) -> Self {
        LargeObjectSpace {
            heap_: heap,
            id_: id,
            memory_chunk_list_: LinkedList::new(),
            size_: AtomicUsize::new(0),
            page_count_: 0,
            objects_size_: AtomicUsize::new(0),
            allocation_mutex_: RecursiveMutex::new(),
            pending_object_: AtomicUsize::new(0),
            pending_allocation_mutex_: Mutex::new(()),
            allocation_counter_: AllocationCounter {},
        }
    }

    fn heap(&self) -> &mut Heap {
        unsafe { &mut *self.heap_ }
    }

    fn identity(&self) -> AllocationSpace {
        self.id_
    }

    pub fn Available(&self) -> usize {
        0
    }

    pub fn TearDown(&mut self) {
        while let Some(page) = self.memory_chunk_list_.pop_front() {
            let page_metadata = unsafe { &mut *page };

            self.heap().memory_allocator().Free(MemoryAllocator::FreeMode::kImmediately, page_metadata);
        }
    }

    pub fn Size(&self) -> usize {
        self.size_.load(Ordering::Relaxed)
    }

    pub fn SizeOfObjects(&self) -> usize {
        self.objects_size_.load(Ordering::Relaxed)
    }

    pub fn CommittedPhysicalMemory(&self) -> usize {
        self.CommittedMemory()
    }

    pub fn PageCount(&self) -> i32 {
        self.page_count_
    }

    pub fn ShrinkPageToObjectSize(&mut self, page: *mut LargePageMetadata, object: Tagged<HeapObject>, object_size: usize) {
        let chunk = unsafe { (*page).Chunk() };
        let isolate = self.heap().isolate();
        let cage_base = PtrComprCageBase(isolate);
    }

    pub fn Contains(&self, obj: Tagged<HeapObject>) -> bool {
        let chunk = unsafe {MemoryChunkMetadata::FromHeapObject(obj)};

        let owned = unsafe { (*chunk).owner() == self as *const _ as *mut _ };

        owned
    }

    pub fn ContainsSlow(&self, addr: Address) -> bool {
        unsafe {
            for page in self.begin()..self.end() {
                if (*MemoryChunk::FromAddress(addr)).address().address() == (*(*page).Chunk()).address().address() {
                    return true;
                }
            }
        }
        false
    }

    pub fn IsEmpty(&self) -> bool {
        self.first_page().is_null()
    }

    pub fn AddPage(&mut self, page: *mut LargePageMetadata, object_size: usize) {
        unsafe {
            self.size_.fetch_add((*page).size(), Ordering::Relaxed);
            self.AccountCommitted((*page).size());
            self.objects_size_.fetch_add(object_size, Ordering::Relaxed);
            self.page_count_ += 1;
            self.memory_chunk_list_.push_back(page);
            (*page).set_owner(self as *mut _);
            ForAll::<_>(|type_, index| {
                self.IncrementExternalBackingStoreBytes(type_, (*page).ExternalBackingStoreBytes(type_));
            });
        }
    }

    pub fn RemovePage(&mut self, page: *mut LargePageMetadata) {
        unsafe {
            self.size_.fetch_sub((*page).size(), Ordering::Relaxed);
            self.AccountUncommitted((*page).size());
            self.page_count_ -= 1;
            self.memory_chunk_list_.retain(|&x| x != page);
            (*page).set_owner(std::ptr::null_mut());
            ForAll::<_>(|type_, index| {
                self.DecrementExternalBackingStoreBytes(type_, (*page).ExternalBackingStoreBytes(type_));
            });
        }
    }

    pub fn first_page(&self) -> *mut LargePageMetadata {
        match self.memory_chunk_list_.front() {
            Some(&page) => page,
            None => std::ptr::null_mut(),
        }
    }

    pub fn begin(&self) -> LargePageIterator {
        LargePageIterator{}
    }

    pub fn end(&self) -> LargePageIterator {
        LargePageIterator{}
    }

    pub fn GetObjectIterator(&self, heap: *mut Heap) -> Box<dyn ObjectIterator> {
        Box::new(LargeObjectSpaceObjectIterator::new(self as *const _ as *mut _))
    }

    pub fn AddAllocationObserver(&mut self, observer: *mut AllocationObserver) {
        self.allocation_counter_.AddAllocationObserver(unsafe { &*observer });
    }

    pub fn RemoveAllocationObserver(&mut self, observer: *mut AllocationObserver) {
        self.allocation_counter_.RemoveAllocationObserver(unsafe { &*observer });
    }

    pub fn pending_object(&self) -> Address {
        Address{}
    }

    pub fn ResetPendingObject(&self) {}

    pub fn pending_allocation_mutex(&self) -> &Mutex<()> {
        &self.pending_allocation_mutex_
    }

    pub fn set_objects_size(&mut self, objects_size: usize) {
        self.objects_size_.store(objects_size, Ordering::Relaxed);
    }
    fn AllocateLargePage(&mut self, _object_size: i32, _executable: Executability) -> *mut LargePageMetadata {std::ptr::null_mut()}
    fn UpdatePendingObject(&mut self, _object: Tagged<HeapObject>) {}
    fn AdvanceAndInvokeAllocationObservers(&mut self, _soon_object: Address, _object_size: usize) {}
    fn CommittedMemory(&self) -> usize { 0 }
    fn AccountCommitted(&mut self, _size: usize) {}
    fn AccountUncommitted(&mut self, _size: usize) {}
    fn IncrementExternalBackingStoreBytes(&mut self, _type: ExternalBackingStoreType, _bytes: usize) {}
    fn DecrementExternalBackingStoreBytes(&mut self, _type: ExternalBackingStoreType, _bytes: usize) {}

    fn isolate(&self) -> &mut Isolate {
        unsafe { &mut *((*self.heap_).isolate()) }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum AllocationSpace {
    NEW_SPACE,
    OLD_SPACE,
    CODE_SPACE,
    LO_SPACE,
    MAP_SPACE,
    NEW_LO_SPACE,
    OLD_LO_SPACE,
    CODE_LO_SPACE,
    SHARED_LO_SPACE,
    SHARED_SPACE,
    READ_ONLY_SPACE,
    TRUSTED_SPACE,
    TRUSTED_LO_SPACE,
    SHARED_TRUSTED_LO_SPACE,
    kNumSpaces
}

impl Space for LargeObjectSpace {}

pub struct OldLargeObjectSpace {
    base: LargeObjectSpace,
}

impl OldLargeObjectSpace {
    pub fn new(heap: *mut Heap) -> Self {
        OldLargeObjectSpace {
            base: LargeObjectSpace::new(heap, AllocationSpace::LO_SPACE),
        }
    }

    pub fn AllocateRaw(&mut self, local_heap: *mut LocalHeap, object_size: i32) -> AllocationResult {
        self.AllocateRaw_executable(local_heap, object_size, Executability{})
    }

    fn AllocateRaw_executable(&mut self, local_heap: *mut LocalHeap, object_size: i32, executable: Executability) -> AllocationResult {
        AllocationResult{}
    }

    pub fn PromoteNewLargeObject(&mut self, page: *mut LargePageMetadata) {}
}

pub struct SharedLargeObjectSpace {
    base: OldLargeObjectSpace,
}

impl SharedLargeObjectSpace {
    pub fn new(heap: *mut Heap) -> Self {
        SharedLargeObjectSpace {
            base: OldLargeObjectSpace::new(heap),
        }
    }
}

pub struct TrustedLargeObjectSpace {
    base: OldLargeObjectSpace,
}

impl TrustedLargeObjectSpace {
    pub fn new(heap: *mut Heap) -> Self {
        TrustedLargeObjectSpace {
            base: OldLargeObjectSpace::new(heap),
        }
    }
}

pub struct SharedTrustedLargeObjectSpace {
    base: OldLargeObjectSpace,
}

impl SharedTrustedLargeObjectSpace {
    pub fn new(heap: *mut Heap) -> Self {
        SharedTrustedLargeObjectSpace {
            base: OldLargeObjectSpace::new(heap),
        }
    }
}

pub struct NewLargeObjectSpace {
    base: LargeObjectSpace,
    capacity_: usize,
}

impl NewLargeObjectSpace {
    pub fn new(heap: *mut Heap, capacity: usize) -> Self {
        NewLargeObjectSpace {
            base: LargeObjectSpace::new(heap, AllocationSpace::NEW_LO_SPACE),
            capacity_: capacity,
        }
    }

    pub fn AllocateRaw(&mut self, local_heap: *mut LocalHeap, object_size: i32) -> AllocationResult {
        AllocationResult{}
    }

    pub fn Available(&self) -> usize {
        self.capacity_ - self.base.SizeOfObjects()
    }

    pub fn Flip(&mut self) {}

    pub fn FreeDeadObjects(&mut self, is_dead: &dyn Fn(Tagged<HeapObject>) -> bool) {}

    pub fn SetCapacity(&mut self, capacity: usize) {
        self.capacity_ = std::cmp::max(capacity, self.base.SizeOfObjects());
    }
}

pub struct CodeLargeObjectSpace {
    base: OldLargeObjectSpace,
}

impl CodeLargeObjectSpace {
    pub fn new(heap: *mut Heap) -> Self {
        CodeLargeObjectSpace {
            base: OldLargeObjectSpace::new(heap),
        }
    }

   pub fn AllocateRaw(&mut self, local_heap: *mut LocalHeap, object_size: i32) -> AllocationResult {
        AllocationResult{}
    }
}

impl Space for OldLargeObjectSpace {}
impl Space for NewLargeObjectSpace {}
impl Space for CodeLargeObjectSpace {}
impl Space for SharedLargeObjectSpace {}
impl Space for TrustedLargeObjectSpace {}
impl Space for SharedTrustedLargeObjectSpace {}

pub struct LargeObjectSpaceObjectIterator {
    current_: *mut LargePageMetadata,
}

impl LargeObjectSpaceObjectIterator {
    pub fn new(space: *mut LargeObjectSpace) -> Self {
        LargeObjectSpaceObjectIterator {
            current_: unsafe { (*space).first_page() },
        }
    }
}

impl ObjectIterator for LargeObjectSpaceObjectIterator {
    fn Next(&mut self) -> Tagged<HeapObject> {
        if self.current_.is_null() {
            return Tagged::<HeapObject>{};
        }
        unsafe {
            let object = (*self.current_).GetObject();
            self.current_ = (*self.current_).next_page();
            object
        }
    }
}

trait MutablePageMetadata {}

fn IsFreeSpaceOrFiller(_object: Tagged<HeapObject>) -> bool { false }
fn ALIGN_TO_ALLOCATION_ALIGNMENT(size: i32) -> i32 {size}

impl Address {
    fn address(&self) -> usize { 0 }
}
