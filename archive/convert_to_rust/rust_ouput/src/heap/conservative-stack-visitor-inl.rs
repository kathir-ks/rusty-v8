// Converted from V8 C++ source files:
// Header: conservative-stack-visitor-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::atomic::Ordering::Relaxed as kRelaxedLoad;

use crate::heap::safepoint::V8;
use crate::heap::stress_scavenge_observer::v8;

pub struct Isolate {
    isolate_data: IsolateData,
    heap: Heap
}

impl Isolate {
    pub fn code_cage_base(&self) -> PtrComprCageBase {
        PtrComprCageBase{}
    }
    pub fn heap(&self) -> &Heap {
        &self.heap
    }
    pub fn isolate_data(&self) -> &IsolateData {
        &self.isolate_data
    }
}

pub struct Heap {
    memory_allocator: MemoryAllocator,
    code_region : CodeRegion
}

impl Heap{
    pub fn memory_allocator(&self) -> &MemoryAllocator {
        &self.memory_allocator
    }
    pub fn code_region(&self) -> &CodeRegion {
        &self.code_region
    }
}

pub struct IsolateData {
    trusted_cage_base_address: Address,
}

impl IsolateData {
    pub fn trusted_cage_base_address(&self) -> Address {
        self.trusted_cage_base_address
    }
}

pub struct CodeRegion{}

impl CodeRegion {
    pub fn contains(&self, _address: Address) -> bool{
        true
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Address(usize);

impl Address {
    pub fn is_null(&self) -> bool {
        self.0 == 0
    }

    pub fn offset(&self, offset: usize) -> Address {
        Address(self.0.wrapping_add(offset))
    }
}

const kNullAddress: Address = Address(0);

pub struct RootVisitor {}

impl RootVisitor {
    pub fn VisitRootPointer(&mut self, _root: Root, _description: *mut (), _slot: FullObjectSlot) {}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Root {
    kStackRoots,
}

pub struct FullObjectSlot(*mut Tagged<Object>);

pub struct MemoryAllocator {}

impl MemoryAllocator {
    pub fn LookupChunkContainingAddressInSafepoint(
        &self,
        _address: Address,
    ) -> *const MemoryChunk {
        std::ptr::null()
    }

    pub fn IsOutsideAllocatedSpace(&self, _address: Address) -> bool {
        false
    }
}

pub struct ConservativeStackVisitorBase<ConcreteVisitor> {
    cage_base_: PtrComprCageBase,
    #[cfg(V8_EXTERNAL_CODE_SPACE)]
    code_cage_base_: PtrComprCageBase,
    #[cfg(V8_EXTERNAL_CODE_SPACE)]
    code_address_region_: CodeRegion,
    #[cfg(V8_ENABLE_SANDBOX)]
    trusted_cage_base_: PtrComprCageBase,
    root_visitor_: *mut RootVisitor,
    allocator_: *mut MemoryAllocator,
    _concrete_visitor: PhantomData<ConcreteVisitor>,
}

impl<ConcreteVisitor> ConservativeStackVisitorBase<ConcreteVisitor> {
    fn new(isolate: *mut Isolate, root_visitor: *mut RootVisitor) -> Self {
        let isolate_ref = unsafe { &*isolate };
        ConservativeStackVisitorBase {
            cage_base_: PtrComprCageBase {},
            #[cfg(V8_EXTERNAL_CODE_SPACE)]
            code_cage_base_: isolate_ref.code_cage_base(),
            #[cfg(V8_EXTERNAL_CODE_SPACE)]
            code_address_region_: isolate_ref.heap().code_region().clone(),
            #[cfg(V8_ENABLE_SANDBOX)]
            trusted_cage_base_: PtrComprCageBase {},
            root_visitor_: root_visitor,
            allocator_: unsafe { (*isolate).heap().memory_allocator() as *mut MemoryAllocator },
            _concrete_visitor: PhantomData,
        }
    }

    #[cfg(V8_COMPRESS_POINTERS)]
    fn IsInterestingCage(&self, cage_base: PtrComprCageBase) -> bool {
        if cage_base == self.cage_base_ {
            return true;
        }
        #[cfg(V8_EXTERNAL_CODE_SPACE)]
        if cage_base == self.code_cage_base_ {
            return true;
        }
        #[cfg(V8_ENABLE_SANDBOX)]
        if cage_base == self.trusted_cage_base_ {
            return true;
        }
        false
    }

    fn FindBasePtr(&self, maybe_inner_ptr: Address, cage_base: PtrComprCageBase) -> Address {
        #[cfg(V8_COMPRESS_POINTERS)]
        debug_assert!(self.IsInterestingCage(cage_base));

        let allocator = unsafe { &*self.allocator_ };
        let chunk = allocator.LookupChunkContainingAddressInSafepoint(maybe_inner_ptr);
        if chunk.is_null() {
            return kNullAddress;
        }

        let chunk_metadata = unsafe { (*chunk).Metadata() };
        debug_assert!(unsafe { (*chunk_metadata).Contains(maybe_inner_ptr) });

        if !ConcreteVisitor::FilterPage(unsafe { &*chunk }) {
            return kNullAddress;
        }

        if unsafe { (*chunk).IsLargePage() } {
            let obj = unsafe {
                Tagged::<HeapObject>::new((*(chunk_metadata as *const LargePageMetadata)).GetObject())
            };
            let map_word = obj.map_word(cage_base, kRelaxedLoad);
            return if !ConcreteVisitor::FilterLargeObject(obj, map_word)
                || InstanceTypeChecker::IsFreeSpaceOrFiller(map_word.ToMap())
            {
                kNullAddress
            } else {
                obj.address()
            };
        }

        let page = unsafe { &*(chunk_metadata as *const MemoryChunkMetadata as *const PageMetadata) };
        let base_ptr = MarkingBitmap::FindPreviousValidObject(page, maybe_inner_ptr);
        debug_assert!(base_ptr.0 <= maybe_inner_ptr.0);

        let bitmap = unsafe {
            const_cast(page.marking_bitmap() as *const MarkingBitmap) as *mut MarkingBitmap
        };

        let mut current_base_ptr = base_ptr;
        loop {
            let obj = Tagged::<HeapObject>::new(HeapObject::FromAddress(current_base_ptr));
            let map_word = obj.map_word(cage_base, kRelaxedLoad);
            if !ConcreteVisitor::FilterNormalObject(obj, map_word, unsafe{&mut *bitmap}) {
                return kNullAddress;
            }

            let size = obj.SizeFromMap(map_word.ToMap());
            debug_assert!(size > 0);

            if maybe_inner_ptr.0 < current_base_ptr.0 + size {
                ConcreteVisitor::HandleObjectFound(obj, size, unsafe{&mut *bitmap});
                return if IsFreeSpaceOrFiller(obj, cage_base) {
                    kNullAddress
                } else {
                    current_base_ptr
                };
            }

            current_base_ptr = Address(current_base_ptr.0.wrapping_add(ALIGN_TO_ALLOCATION_ALIGNMENT(size)));
            debug_assert!(current_base_ptr.0 < page.area_end().0);
        }
    }

    fn VisitPointer(&self, pointer: *const void) {
        let address = Address(pointer as usize);
        #[cfg(V8_COMPRESS_POINTERS)]
        {
            V8HeapCompressionScheme::ProcessIntermediatePointers(
                self.cage_base_,
                address,
                |ptr| self.VisitConservativelyIfPointer(ptr, self.cage_base_),
            );
            if ConcreteVisitor::kOnlyVisitMainV8Cage {
                return;
            }
            #[cfg(V8_EXTERNAL_CODE_SPACE)]
            {
                ExternalCodeCompressionScheme::ProcessIntermediatePointers(
                    self.code_cage_base_,
                    address,
                    |ptr| self.VisitConservativelyIfPointer(ptr, self.code_cage_base_),
                );
            }
            #[cfg(V8_ENABLE_SANDBOX)]
            {
                TrustedSpaceCompressionScheme::ProcessIntermediatePointers(
                    self.trusted_cage_base_,
                    address,
                    |ptr| self.VisitConservativelyIfPointer(ptr, self.trusted_cage_base_),
                );
            }
        }
        #[cfg(not(V8_COMPRESS_POINTERS))]
        self.VisitConservativelyIfPointer(address);
    }

    fn VisitConservativelyIfPointer(&self, address: Address) {
        #[cfg(V8_COMPRESS_POINTERS)]
        {
            if V8HeapCompressionScheme::GetPtrComprCageBaseAddress(address) == self.cage_base_.address() {
                self.VisitConservativelyIfPointer(address, self.cage_base_);
            } else if ConcreteVisitor::kOnlyVisitMainV8Cage {
                return;
            }
            #[cfg(V8_EXTERNAL_CODE_SPACE)]
            {
                if self.code_address_region_.contains(address) {
                    self.VisitConservativelyIfPointer(address, self.code_cage_base_);
                }
            }
            #[cfg(V8_ENABLE_SANDBOX)]
            {
                if TrustedSpaceCompressionScheme::GetPtrComprCageBaseAddress(address)
                    == self.trusted_cage_base_.address()
                {
                    self.VisitConservativelyIfPointer(address, self.trusted_cage_base_);
                }
            }
        }
        #[cfg(not(V8_COMPRESS_POINTERS))]
        self.VisitConservativelyIfPointer(address, self.cage_base_);
    }

    fn VisitConservativelyIfPointer(&self, address: Address, cage_base: PtrComprCageBase) {
        let allocator = unsafe { &*self.allocator_ };
        if allocator.IsOutsideAllocatedSpace(address) {
            debug_assert_eq!(
                std::ptr::null(),
                allocator.LookupChunkContainingAddressInSafepoint(address)
            );
            return;
        }

        let base_ptr = self.FindBasePtr(address, cage_base);
        if base_ptr.is_null() {
            return;
        }

        let obj = HeapObject::FromAddress(base_ptr);
        let mut root = obj;
        let root_visitor = unsafe { &mut *self.root_visitor_ };
        root_visitor.VisitRootPointer(
            Root::kStackRoots,
            std::ptr::null_mut(),
            FullObjectSlot(&mut root),
        );

        debug_assert_eq!(root, obj);
    }
}

pub trait ConservativeStackVisitor {
    const kOnlyVisitMainV8Cage: bool;
    fn FilterPage(_chunk: &MemoryChunk) -> bool;
    fn FilterLargeObject(_obj: Tagged<HeapObject>, _map_word: MapWord) -> bool;
    fn FilterNormalObject(_obj: Tagged<HeapObject>, _map_word: MapWord, _bitmap: &mut MarkingBitmap) -> bool;
    fn HandleObjectFound(_obj: Tagged<HeapObject>, _size: usize, _bitmap: &mut MarkingBitmap);
}

pub struct MemoryChunk {}

impl MemoryChunk {
    pub fn Metadata(&self) -> *mut MemoryChunkMetadata {
        std::ptr::null_mut()
    }
    pub fn IsLargePage(&self) -> bool {
        false
    }
}

pub struct MemoryChunkMetadata {}

impl MemoryChunkMetadata {
    pub fn Contains(&self, _address: Address) -> bool {
        false
    }
}

pub struct LargePageMetadata {}

impl LargePageMetadata{
    pub fn GetObject(&self) -> Address {
        Address(0)
    }
}

pub struct PageMetadata { }

impl PageMetadata {
    pub fn marking_bitmap(&self) -> &MarkingBitmap {
        todo!()
    }

    pub fn area_end(&self) -> Address {
        Address(0)
    }
}

pub struct MarkingBitmap {}

impl MarkingBitmap {
    pub fn FindPreviousValidObject(_page: &PageMetadata, _address: Address) -> Address {
        Address(0)
    }
}

pub struct HeapObject {}

impl HeapObject {
    pub fn FromAddress(_address: Address) -> Self {
        HeapObject {}
    }

    pub fn map_word(&self, _cage_base: PtrComprCageBase, _mode: std::sync::atomic::Ordering) -> MapWord {
        MapWord {}
    }

    pub fn SizeFromMap(&self, _map: *mut Map) -> usize {
        0
    }
}

pub struct Object {}

pub struct Tagged<T> {
    _phantom: PhantomData<T>,
    address : Address
}

impl<T> Tagged<T> {
    pub fn new(obj : HeapObject) -> Self{
        Tagged{
            _phantom: PhantomData,
            address : Address(0)
        }
    }
    pub fn address(&self) -> Address {
        self.address
    }
}

impl<T> Copy for Tagged<T> {}

impl<T> Clone for Tagged<T> {
    fn clone(&self) -> Self {
        *self
    }
}

fn IsFreeSpaceOrFiller(_obj: Tagged<HeapObject>, _cage_base: PtrComprCageBase) -> bool {
    false
}

fn ALIGN_TO_ALLOCATION_ALIGNMENT(size: usize) -> usize {
    size
}

pub struct Map {}

pub struct InstanceTypeChecker {}

impl InstanceTypeChecker {
    pub fn IsFreeSpaceOrFiller(_map: *mut Map) -> bool {
        false
    }
}

pub struct V8HeapCompressionScheme {}

impl V8HeapCompressionScheme {
    pub fn ProcessIntermediatePointers(
        _cage_base: PtrComprCageBase,
        _address: Address,
        _callback: impl FnMut(Address),
    ) {
    }

    pub fn GetPtrComprCageBaseAddress(_address: Address) -> Address {
        Address(0)
    }
}

pub struct ExternalCodeCompressionScheme {}

impl ExternalCodeCompressionScheme {
    pub fn ProcessIntermediatePointers(
        _code_cage_base: PtrComprCageBase,
        _address: Address,
        _callback: impl FnMut(Address),
    ) {
    }
}

pub struct TrustedSpaceCompressionScheme {}

impl TrustedSpaceCompressionScheme {
    pub fn ProcessIntermediatePointers(
        _trusted_cage_base: PtrComprCageBase,
        _address: Address,
        _callback: impl FnMut(Address),
    ) {
    }

    pub fn GetPtrComprCageBaseAddress(_address: Address) -> Address {
        Address(0)
    }
}
pub struct PtrComprCageBase {}

impl PartialEq for PtrComprCageBase {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl PtrComprCageBase {
    pub fn address(&self) -> Address {
        Address(0)
    }
}

unsafe fn const_cast<T>(ptr: *const T) -> *mut T {
    ptr as *mut T
}
