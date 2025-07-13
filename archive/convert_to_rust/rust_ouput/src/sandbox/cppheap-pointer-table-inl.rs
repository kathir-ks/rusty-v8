// Converted from V8 C++ source files:
// Header: cppheap-pointer-table-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod cppheap_pointer_table_inl {
use std::sync::atomic::{AtomicU64, Ordering};

use crate::sandbox::cppheap_pointer_table::{
    CppHeapPointerHandle, CppHeapPointerTable, CppHeapPointerTableEntry,
    CppHeapPointerTag, CppHeapPointerTagRange, kBitsPerSystemPointer,
    kCppHeapPointerIndexShift, kCppHeapPointerPayloadShift,
    kMaxCppHeapPointers, kNullCppHeapPointerHandle,
};
use crate::sandbox::isolate::Address;
use crate::sandbox::space::Space;

#[derive(Debug, Copy, Clone)]
pub struct Payload(u64);

impl Payload {
    pub fn new(value: u64, tag: CppHeapPointerTag) -> Self {
        let tag_bits = tag as u64;
        Payload((value << 2) | tag_bits)
    }

    pub fn value(&self) -> u64 {
        self.0 >> 2
    }

    pub fn tag(&self) -> CppHeapPointerTag {
        match self.0 & 0x3 {
            0 => CppHeapPointerTag::kNormal,
            1 => CppHeapPointerTag::kZappedEntryTag,
            2 => CppHeapPointerTag::kFreeEntryTag,
            3 => CppHeapPointerTag::kEvacuationEntryTag,
            _ => panic!("Invalid tag"),
        }
    }

    pub fn contains_pointer(&self) -> bool {
        !matches!(self.tag(), CppHeapPointerTag::kFreeEntryTag)
    }

    pub fn untag(&self, tag_range: CppHeapPointerTagRange) -> Address {
        if self.is_tagged_with_tag_in(tag_range) {
            self.value() as Address
        } else {
            0 as Address // Or some other appropriate default
        }
    }

    pub fn is_tagged_with_tag_in(&self, tag_range: CppHeapPointerTagRange) -> bool {
        tag_range.contains(self.tag())
    }
    
    pub fn extract_freelist_link(&self) -> u32 {
        self.value() as u32
    }
    
    pub fn has_mark_bit_set(&self) -> bool {
        false // Implement the logic if needed.  Assuming mark bit is not used.
    }
    
    pub fn set_mark_bit(&mut self) {
        // Implement the logic if needed.  Assuming mark bit is not used.
    }
    
    pub fn contains_evacuation_entry(&self) -> bool {
        self.tag() == CppHeapPointerTag::kEvacuationEntryTag
    }
}

impl CppHeapPointerTableEntry {
    pub fn make_pointer_entry(&self, value: Address, tag: CppHeapPointerTag, mark_as_alive: bool) {
        assert_eq!(0, value >> (kBitsPerSystemPointer - kCppHeapPointerPayloadShift));
        assert_ne!(tag, CppHeapPointerTag::kFreeEntryTag);
        assert_ne!(tag, CppHeapPointerTag::kEvacuationEntryTag);

        let mut new_payload = Payload::new(value as u64, tag);

        if mark_as_alive {
            new_payload.set_mark_bit();
        }
        self.payload_.store(new_payload.0, Ordering::Relaxed);
    }

    pub fn get_pointer(&self, tag_range: CppHeapPointerTagRange) -> Address {
        let payload = Payload(self.payload_.load(Ordering::Relaxed));
        assert!(payload.contains_pointer());
        payload.untag(tag_range)
    }

    pub fn set_pointer(&self, value: Address, tag: CppHeapPointerTag) {
        assert_eq!(0, value >> (kBitsPerSystemPointer - kCppHeapPointerPayloadShift));
        assert_ne!(tag, CppHeapPointerTag::kFreeEntryTag);
        assert_ne!(tag, CppHeapPointerTag::kEvacuationEntryTag);
        assert!(Payload(self.payload_.load(Ordering::Relaxed)).contains_pointer());

        let new_payload = Payload::new(value as u64, tag);
        self.payload_.store(new_payload.0, Ordering::Relaxed);
    }

    pub fn has_pointer(&self, tag_range: CppHeapPointerTagRange) -> bool {
        let payload = Payload(self.payload_.load(Ordering::Relaxed));
        payload.is_tagged_with_tag_in(tag_range)
    }

    pub fn make_zapped_entry(&self) {
        let new_payload = Payload::new(0, CppHeapPointerTag::kZappedEntryTag);
        self.payload_.store(new_payload.0, Ordering::Relaxed);
    }

    pub fn make_freelist_entry(&self, next_entry_index: u32) {
        assert!(kMaxCppHeapPointers <= u32::MAX as usize);
        let new_payload = Payload::new(next_entry_index as u64, CppHeapPointerTag::kFreeEntryTag);
        self.payload_.store(new_payload.0, Ordering::Relaxed);
    }

    pub fn get_next_freelist_entry_index(&self) -> u32 {
        let payload = Payload(self.payload_.load(Ordering::Relaxed));
        payload.extract_freelist_link()
    }

    pub fn mark(&self) {
        let old_payload_bits = self.payload_.load(Ordering::Relaxed);
        let old_payload = Payload(old_payload_bits);
        assert!(old_payload.contains_pointer());

        let mut new_payload = old_payload;
        new_payload.set_mark_bit();

        self.payload_.compare_exchange_strong(
            old_payload_bits,
            new_payload.0,
            Ordering::Relaxed,
            Ordering::Relaxed,
        );
    }

    pub fn make_evacuation_entry(&self, handle_location: Address) {
        let new_payload = Payload::new(handle_location as u64, CppHeapPointerTag::kEvacuationEntryTag);
        self.payload_.store(new_payload.0, Ordering::Relaxed);
    }

    pub fn has_evacuation_entry(&self) -> bool {
        let payload = Payload(self.payload_.load(Ordering::Relaxed));
        payload.contains_evacuation_entry()
    }

    pub fn evacuate(&self, dest: &CppHeapPointerTableEntry) {
        let payload = Payload(self.payload_.load(Ordering::Relaxed));
        assert!(payload.contains_pointer());
        assert!(!payload.has_mark_bit_set());

        dest.payload_.store(payload.0, Ordering::Relaxed);

        self.make_zapped_entry();
    }
}

impl CppHeapPointerTable {
    pub fn get(&self, handle: CppHeapPointerHandle, tag_range: CppHeapPointerTagRange) -> Address {
        let index = Self::handle_to_index(handle);
        assert!(index == 0 || self.at(index).has_pointer(tag_range));
        self.at(index).get_pointer(tag_range)
    }

    pub fn set(&self, handle: CppHeapPointerHandle, value: Address, tag: CppHeapPointerTag) {
        assert_ne!(kNullCppHeapPointerHandle, handle);
        let index = Self::handle_to_index(handle);
        self.at(index).set_pointer(value, tag);
    }

    pub fn allocate_and_initialize_entry(
        &self,
        space: &mut Space,
        initial_value: Address,
        tag: CppHeapPointerTag,
    ) -> CppHeapPointerHandle {
        assert!(space.belongs_to(self));
        let index = self.allocate_entry(space);
        self.at(index)
            .make_pointer_entry(initial_value, tag, space.allocate_black());

        let handle = Self::index_to_handle(index);
        handle
    }

    pub fn mark(
        &self,
        space: &mut Space,
        handle: CppHeapPointerHandle,
        handle_location: Address,
    ) {
        assert!(space.belongs_to(self));

        // The handle_location must always contain the given handle. Except if the
        // slot is lazily-initialized. In that case, the handle may transition from
        // the null handle to a valid handle. However, in that case the
        // newly-allocated entry will already have been marked as alive during
        // allocation, and so we don't need to do anything here.
        
        let index = Self::handle_to_index(handle);
        assert!(space.contains(index));

        // If the table is being compacted and the entry is inside the evacuation
        // area, then allocate and set up an evacuation entry for it.
        self.maybe_create_evacuation_entry(space, index, handle_location);

        // Even if the entry is marked for evacuation, it still needs to be marked as
        // alive as it may be visited during sweeping before being evacuation.
        self.at(index).mark();
    }

    pub fn is_valid_handle(handle: CppHeapPointerHandle) -> bool {
        let index = handle >> kCppHeapPointerIndexShift;
        handle == index << kCppHeapPointerIndexShift
    }

    pub fn handle_to_index(handle: CppHeapPointerHandle) -> u32 {
        assert!(Self::is_valid_handle(handle));
        let index = handle >> kCppHeapPointerIndexShift;
        assert!(index <= kMaxCppHeapPointers as u32);
        index
    }

    pub fn index_to_handle(index: u32) -> CppHeapPointerHandle {
        assert!(index <= kMaxCppHeapPointers as u32);
        let handle = index << kCppHeapPointerIndexShift;
        assert_ne!(handle, kNullCppHeapPointerHandle);
        handle
    }

    pub fn contains(&self, space: &Space, handle: CppHeapPointerHandle) -> bool {
        assert!(space.belongs_to(self));
        space.contains(Self::handle_to_index(handle))
    }
}
}
