// Converted from V8 C++ source files:
// Header: external-pointer-table-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_camel_case_types)]
use std::sync::atomic::{AtomicU64, Ordering};

//use crate::sandbox::compactible_external_entity_table_inl::CompactibleExternalEntityTable;
use crate::sandbox::external_pointer_table::{
    ExternalPointerHandle, ExternalPointerTag, ExternalPointerTagRange, EvacuateMarkMode,
    kNullExternalPointerHandle,
};
//use crate::sandbox::external_pointer::ExternalPointer;
use crate::sandbox::isolate::Address;

const kExternalPointerTagAndMarkbitMask: Address = 0;
const kNullAddress: Address = 0;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ExternalPointerError {
    InvalidTag,
    InvalidAddress,
    ContainsPointer,
    NotContainsPointer,
    InvalidHandle,
    SpaceNotBelongs,
}

#[derive(Debug, Copy, Clone)]
struct Payload {
    value: u64,
}

impl Payload {
    fn new(value: Address, tag: ExternalPointerTag) -> Self {
        let value = value as u64 | ((tag as u64) << 48);
        Payload { value }
    }

    fn contains_pointer(&self) -> bool {
        (self.value >> 48) != ExternalPointerTag::kExternalPointerFreeEntryTag as u64
    }

    fn untag(&self, tag_range: ExternalPointerTagRange) -> Address {
        let tag = (self.value >> 48) as ExternalPointerTag;
        if !tag_range.contains(tag) {
            panic!("Unexpected tag {:?} not in range", tag);
        }
        (self.value & 0x0000_FFFF_FFFF_FFFF) as Address
    }

    fn set_mark_bit(&mut self) {
        self.value |= 1 << 63;
    }

    fn clear_mark_bit(&mut self) {
        self.value &= !(1 << 63);
    }

    fn extract_tag(&self) -> ExternalPointerTag {
        (self.value >> 48) as ExternalPointerTag
    }

    fn extract_freelist_link(&self) -> u32 {
        (self.value & 0x0000_FFFF_FFFF_FFFF) as u32
    }

    fn has_mark_bit_set(&self) -> bool {
        (self.value & (1 << 63)) != 0
    }

    fn contains_evacuation_entry(&self) -> bool {
        (self.value >> 48) == ExternalPointerTag::kExternalPointerEvacuationEntryTag as u64
    }
}

struct ExternalPointerTableEntry {
    payload_: AtomicU64,
    raw_pointer_for_lsan_: Address, // For LeakSanitizer, can be removed if LSan is not needed
}

impl ExternalPointerTableEntry {
    fn new() -> Self {
        ExternalPointerTableEntry {
            payload_: AtomicU64::new(0),
            raw_pointer_for_lsan_: 0,
        }
    }
    fn make_external_pointer_entry(
        &mut self,
        value: Address,
        tag: ExternalPointerTag,
        mark_as_alive: bool,
    ) {
        // The 2nd most significant byte must be empty as we store the tag in int.
        assert_eq!(0, value & kExternalPointerTagAndMarkbitMask);
        assert_ne!(tag, ExternalPointerTag::kExternalPointerFreeEntryTag);
        assert_ne!(tag, ExternalPointerTag::kExternalPointerEvacuationEntryTag);

        let mut new_payload = Payload::new(value, tag);
        if mark_as_alive {
            new_payload.set_mark_bit();
        }
        self.payload_.store(new_payload.value, Ordering::Relaxed);
        self.maybe_update_raw_pointer_for_lsan(value);
    }

    fn get_external_pointer(&self, tag_range: ExternalPointerTagRange) -> Address {
        let payload = Payload {
            value: self.payload_.load(Ordering::Relaxed),
        };
        assert!(payload.contains_pointer());
        payload.untag(tag_range)
    }

    fn set_external_pointer(&mut self, value: Address, tag: ExternalPointerTag) {
        // The 2nd most significant byte must be empty as we store the tag in int.
        assert_eq!(0, value & kExternalPointerTagAndMarkbitMask);
        assert!(Payload {
            value: self.payload_.load(Ordering::Relaxed),
        }
        .contains_pointer());

        let mut new_payload = Payload::new(value, tag);
        // Writing an entry currently also marks it as alive. In the future, we might
        // want to drop this and instead use write barriers where necessary.
        new_payload.set_mark_bit();
        self.payload_.store(new_payload.value, Ordering::Relaxed);
        self.maybe_update_raw_pointer_for_lsan(value);
    }

    fn has_external_pointer(&self, tag_range: ExternalPointerTagRange) -> bool {
        let payload = Payload {
            value: self.payload_.load(Ordering::Relaxed),
        };
        if !payload.contains_pointer() {
            return false;
        }
        let tag = payload.extract_tag();
        tag_range.contains(tag)
    }

    fn exchange_external_pointer(&self, value: Address, tag: ExternalPointerTag) -> Address {
        // The 2nd most significant byte must be empty as we store the tag in int.
        assert_eq!(0, value & kExternalPointerTagAndMarkbitMask);

        let mut new_payload = Payload::new(value, tag);
        // Writing an entry currently also marks it as alive. In the future, we might
        // want to drop this and instead use write barriers where necessary.
        new_payload.set_mark_bit();
        let old_payload_value =
            self.payload_
                .compare_exchange(self.payload_.load(Ordering::Relaxed),new_payload.value, Ordering::Relaxed, Ordering::Relaxed);
        let old_payload = Payload{value: match old_payload_ {
            Ok(v) => v,
            Err(v) => v,
        }};
        assert!(old_payload.contains_pointer());
        self.maybe_update_raw_pointer_for_lsan(value);
        old_payload.untag(tag)
    }

    fn get_external_pointer_tag(&self) -> ExternalPointerTag {
        let payload = Payload {
            value: self.payload_.load(Ordering::Relaxed),
        };
        assert!(payload.contains_pointer());
        payload.extract_tag()
    }

    fn extract_managed_resource_or_null(&self) -> Address {
        let payload = Payload {
            value: self.payload_.load(Ordering::Relaxed),
        };
        let tag = payload.extract_tag();
        if is_managed_external_pointer_type(tag) {
            payload.untag(tag)
        } else {
            kNullAddress
        }
    }

    fn make_zapped_entry(&mut self) {
        let new_payload = Payload::new(kNullAddress, ExternalPointerTag::kExternalPointerZappedEntryTag);
        self.payload_.store(new_payload.value, Ordering::Relaxed);
    }

    fn make_freelist_entry(&mut self, next_entry_index: u32) {
        // The next freelist entry is stored in the lower bits of the entry.
        assert!(
            kMaxExternalPointers <= std::u32::MAX,
            "kMaxExternalPointers exceeds u32::MAX"
        );
        let new_payload = Payload::new(next_entry_index as Address, ExternalPointerTag::kExternalPointerFreeEntryTag);
        self.payload_.store(new_payload.value, Ordering::Relaxed);
    }

    fn get_next_freelist_entry_index(&self) -> u32 {
        let payload = Payload {
            value: self.payload_.load(Ordering::Relaxed),
        };
        payload.extract_freelist_link()
    }

    fn mark(&self) {
        let old_payload_value = self.payload_.load(Ordering::Relaxed);
        let old_payload = Payload{value: old_payload_value};
        assert!(old_payload.contains_pointer());

        let mut new_payload = old_payload;
        new_payload.set_mark_bit();

        // We don't need to perform the CAS in a loop: if the new value is not equal
        // to the old value, then the mutator must've just written a new value into
        // the entry. This in turn must've set the marking bit already (see e.g.
        // SetExternalPointer), so we don't need to do it again.
        let success = self.payload_.compare_exchange(old_payload_value, new_payload.value, Ordering::Relaxed, Ordering::Relaxed).is_ok();
        assert!(success || old_payload.has_mark_bit_set());
    }

    fn make_evacuation_entry(&mut self, handle_location: Address) {
        let new_payload = Payload::new(handle_location, ExternalPointerTag::kExternalPointerEvacuationEntryTag);
        self.payload_.store(new_payload.value, Ordering::Relaxed);
    }

    fn has_evacuation_entry(&self) -> bool {
        let payload = Payload {
            value: self.payload_.load(Ordering::Relaxed),
        };
        payload.contains_evacuation_entry()
    }

    fn evacuate(&mut self, dest: &mut ExternalPointerTableEntry, mode: EvacuateMarkMode) {
        let payload = Payload {
            value: self.payload_.load(Ordering::Relaxed),
        };
        // We expect to only evacuate entries containing external pointers.
        assert!(payload.contains_pointer());

        let mut payload = Payload {
            value: self.payload_.load(Ordering::Relaxed),
        };
        match mode {
            EvacuateMarkMode::kTransferMark => {}
            EvacuateMarkMode::kLeaveUnmarked => {
                assert!(!payload.has_mark_bit_set());
            }
            EvacuateMarkMode::kClearMark => {
                assert!(payload.has_mark_bit_set());
                payload.clear_mark_bit();
            }
        }

        dest.payload_.store(payload.value, Ordering::Relaxed);
        dest.raw_pointer_for_lsan_ = self.raw_pointer_for_lsan_;

        // The destination entry takes ownership of the pointer.
        self.make_zapped_entry();
    }

    fn maybe_update_raw_pointer_for_lsan(&mut self, _value: Address) {
        //Placeholder for when LeakSanitizer is active
    }
}

struct ExternalPointerTable {
    spaces: Vec<Space>,
}

impl ExternalPointerTable {
    fn new(num_spaces: usize) -> Self {
        let mut spaces = Vec::with_capacity(num_spaces);
        for _ in 0..num_spaces {
            spaces.push(Space::new());
        }
        ExternalPointerTable { spaces }
    }

    fn get(
        &self,
        handle: ExternalPointerHandle,
        tag_range: ExternalPointerTagRange,
    ) -> Address {
        let index = Self::handle_to_index(handle);
        assert!(index == 0 || self.at(index).has_external_pointer(tag_range));
        self.at(index).get_external_pointer(tag_range)
    }

    fn set(
        &self,
        handle: ExternalPointerHandle,
        value: Address,
        tag: ExternalPointerTag,
        space_index: usize,
    ) {
        assert_ne!(kNullExternalPointerHandle, handle);
        let index = Self::handle_to_index(handle);

        self.free_managed_resource_if_present(index);
        self.take_ownership_of_managed_resource_if_necessary(value, handle, tag);

        let mut entry = self.at(index);
        entry.set_external_pointer(value, tag);
    }

    fn exchange(&self, handle: ExternalPointerHandle, value: Address, tag: ExternalPointerTag) -> Address {
        assert_ne!(kNullExternalPointerHandle, handle);
        assert!(!is_managed_external_pointer_type(tag));
        let index = Self::handle_to_index(handle);
        self.at(index).exchange_external_pointer(value, tag)
    }

    fn get_tag(&self, handle: ExternalPointerHandle) -> ExternalPointerTag {
        let index = Self::handle_to_index(handle);
        self.at(index).get_external_pointer_tag()
    }

    fn zap(&self, handle: ExternalPointerHandle) {
        // Zapping the null entry is a nop. This is useful as we reset the handle of
        // managed resources to the kNullExternalPointerHandle when the entry is
        // deleted. See SweepAndCompact.
        if handle == kNullExternalPointerHandle {
            return;
        }
        let index = Self::handle_to_index(handle);
        let mut entry = self.at(index);
        entry.make_zapped_entry();
    }

    fn allocate_and_initialize_entry(
        &self,
        space_index: usize,
        initial_value: Address,
        tag: ExternalPointerTag,
    ) -> ExternalPointerHandle {
        let space = &self.spaces[space_index];
        assert!(space.belongs_to(self));

        let index = self.allocate_entry(space);
        let mut entry = self.at(index);

        entry.make_external_pointer_entry(initial_value, tag, space.allocate_black());
        let handle = Self::index_to_handle(index);
        self.take_ownership_of_managed_resource_if_necessary(initial_value, handle, tag);
        handle
    }

    fn mark(&self, space_index: usize, handle: ExternalPointerHandle, handle_location: Address) {
        let space = &self.spaces[space_index];
        assert!(space.belongs_to(self));

        // The handle_location must always contain the given handle. Except if the
        // slot is lazily-initialized. In that case, the handle may transition from
        // the null handle to a valid handle. However, in that case the
        // newly-allocated entry will already have been marked as alive during
        // allocation, and so we don't need to do anything here.

        // If the handle is null, it doesn't have an EPT entry; no mark is needed.
        if handle == kNullExternalPointerHandle {
            return;
        }

        let index = Self::handle_to_index(handle);
        assert!(space.contains(index));

        // If the table is being compacted and the entry is inside the evacuation
        // area, then allocate and set up an evacuation entry for it.
        self.maybe_create_evacuation_entry(space_index, index, handle_location);

        // Even if the entry is marked for evacuation, it still needs to be marked as
        // alive as it may be visited during sweeping before being evacuation.
        let mut entry = self.at(index);
        entry.mark();
    }

    fn evacuate(
        &self,
        from_space_index: usize,
        to_space_index: usize,
        handle: ExternalPointerHandle,
        handle_location: Address,
        mode: EvacuateMarkMode,
    ) {
        let from_space = &self.spaces[from_space_index];
        let to_space = &self.spaces[to_space_index];
        assert!(from_space.belongs_to(self));
        assert!(to_space.belongs_to(self));

        assert!(Self::is_valid_handle(handle));

        // If the handle is null, it doesn't have an EPT entry; no evacuation is
        // needed.
        if handle == kNullExternalPointerHandle {
            return;
        }

        let from_index = Self::handle_to_index(handle);
        assert!(from_space.contains(from_index));

        let to_index = self.allocate_entry(to_space);

        let mut from_entry = self.at(from_index);
        let mut to_entry = self.at(to_index);

        from_entry.evacuate(&mut to_entry, mode);
        let new_handle = Self::index_to_handle(to_index);

        if let Some(addr) = self.extract_managed_resource_or_null(to_index) {
            let resource = addr as *mut ManagedResource;
            unsafe {
                (*resource).ept_entry_ = new_handle;
            }
        }

        // Update slot to point to new handle.
        unsafe {
            *(handle_location as *mut ExternalPointerHandle) = new_handle;
        }
    }

    // static
    fn is_valid_handle(handle: ExternalPointerHandle) -> bool {
        let index = handle >> kExternalPointerIndexShift;
        handle == index << kExternalPointerIndexShift
    }

    // static
    fn handle_to_index(handle: ExternalPointerHandle) -> usize {
        assert!(Self::is_valid_handle(handle));
        let mut index = (handle >> kExternalPointerIndexShift) as usize;
        // When LSan is active, we use "fat" entries that also store the raw pointer
        // to that LSan can find live references. However, we do this transparently:
        // we simply multiply the handle by two so that `(handle >> index_shift) * 8`
        // still produces the correct offset of the entry in the table. However, this
        // is not secure as an attacker could reference the raw pointer instead of
        // the encoded pointer in an entry, thereby bypassing the type checks. As
        // such, this mode must only be used in testing environments. Alternatively,
        // all places that access external pointer table entries must be made aware
        // that the entries are 16 bytes large when LSan is active.
        index /= 2;
        assert!(index <= kMaxExternalPointers as usize);
        index
    }

    // static
    fn index_to_handle(index: u32) -> ExternalPointerHandle {
        assert!(index <= kMaxExternalPointers);
        let mut handle = index << kExternalPointerIndexShift;
        // When LSan is active, we use "fat" entries that also store the raw pointer
        // to that LSan can find live references. However, we do this transparently:
        // we simply multiply the handle by two so that `(handle >> index_shift) * 8`
        // still produces the correct offset of the entry in the table. However, this
        // is not secure as an attacker could reference the raw pointer instead of
        // the encoded pointer in an entry, thereby bypassing the type checks. As
        // such, this mode must only be used in testing environments. Alternatively,
        // all places that access external pointer table entries must be made aware
        // that the entries are 16 bytes large when LSan is active.
        handle *= 2;
        assert_ne!(handle, kNullExternalPointerHandle);
        handle
    }

    fn contains(&self, space_index: usize, handle: ExternalPointerHandle) -> bool {
        let space = &self.spaces[space_index];
        assert!(space.belongs_to(self));
        space.contains(Self::handle_to_index(handle))
    }

    fn take_ownership_of_managed_resource_if_necessary(
        &self,
        value: Address,
        handle: ExternalPointerHandle,
        tag: ExternalPointerTag,
    ) {
        if is_managed_external_pointer_type(tag) && value != kNullAddress {
            let resource = value as *mut ManagedResource;
            unsafe {
                assert_eq!((*resource).ept_entry_, kNullExternalPointerHandle);
                (*resource).owning_table_ = self;
                (*resource).ept_entry_ = handle;
            }
        }
    }

    fn free_managed_resource_if_present(&self, entry_index: usize) {
        // In the future, this would be where we actually delete the external
        // resource. Currently, the deletion still happens elsewhere, and so here we
        // instead set the resource's handle to the null handle so that the resource
        // does not attempt to zap its entry when it is eventually destroyed.
        if let Some(addr) = self.extract_managed_resource_or_null(entry_index) {
            let resource = addr as *mut ManagedResource;
            unsafe {
                assert_eq!((*resource).ept_entry_, Self::index_to_handle(entry_index as u32));
                (*resource).ept_entry_ = kNullExternalPointerHandle;
            }
        }
    }

    fn extract_managed_resource_or_null(&self, entry_index: usize) -> Option<Address> {
        let entry = self.at(entry_index);
        let addr = entry.extract_managed_resource_or_null();
        if addr == kNullAddress {
            None
        } else {
            Some(addr)
        }
    }

    const NUM_ENTRIES: usize = 1024; // Adjust as needed

    fn at(&self, index: usize) -> &mut ExternalPointerTableEntry {
        let space_index = 0; // for now only using space_index 0
        &mut self.spaces[space_index].entries[index]
    }

    fn allocate_entry(&self, space: &Space) -> usize {
        space.allocate_entry()
    }

    fn maybe_create_evacuation_entry(&self, _space_index: usize, _index: usize, _handle_location: Address) {
        // placeholder
    }
}

#[derive(Default)]
struct Space {
    entries: Vec<ExternalPointerTableEntry>,
    free_list_head: AtomicU64,
    invalidated_fields: Vec<Address>,
    is_black_allocation: bool,
}

impl Space {
    fn new() -> Self {
        let mut entries = Vec::with_capacity(ExternalPointerTable::NUM_ENTRIES);
        for _ in 0..ExternalPointerTable::NUM_ENTRIES {
            entries.push(ExternalPointerTableEntry::new());
        }
        let mut space = Space {
            entries,
            free_list_head: AtomicU64::new(0),
            invalidated_fields: Vec::new(),
            is_black_allocation: false,
        };
        space.initialize_free_list();
        space
    }

    fn initialize_free_list(&mut self) {
        for i in 0..(ExternalPointerTable::NUM_ENTRIES - 1) {
            self.entries[i].make_freelist_entry((i + 1) as u32);
        }
        self.entries[ExternalPointerTable::NUM_ENTRIES - 1].make_freelist_entry(0);
    }

    fn belongs_to(&self, _table: &ExternalPointerTable) -> bool {
        true // Assuming all spaces belong to the given table
    }

    fn contains(&self, index: usize) -> bool {
        index < ExternalPointerTable::NUM_ENTRIES
    }

    fn allocate_entry(&self) -> usize {
        // get current head
        let current_head = self.free_list_head.load(Ordering::Relaxed);
        // get next free head from entries
        let next_head: u64 = if current_head != 0 {
                self.entries[current_head as usize].get_next_freelist_entry_index() as u64
            } else {
                panic!("OOM");
            };
        // try to CAS the free list head to point to the next free index
        if self.free_list_head.compare_exchange(current_head, next_head, Ordering::Relaxed, Ordering::Relaxed).is_ok() {
            current_head as usize
        } else {
            self.allocate_entry() // try again
        }
    }

    fn allocate_black(&self) -> bool {
        self.is_black_allocation
    }

    fn add_invalidated_field(&mut self, field_address: Address) {
        self.invalidated_fields.push(field_address);
    }

    fn notify_external_pointer_field_invalidated(
        &mut self,
        field_address: Address,
        tag_range: ExternalPointerTagRange,
    ) {
        // We do not currently support invalidating fields containing managed
        // external pointers. If this is ever needed, we would probably need to free
        // the managed object here as we may otherwise fail to do so during sweeping.
        assert!(!is_managed_external_pointer_type(tag_range));

        self.add_invalidated_field(field_address);
    }
}

#[derive(Debug)]
struct ManagedResource {
    owning_table_: *const ExternalPointerTable,
    ept_entry_: ExternalPointerHandle,
}

impl ManagedResource {
    fn zap_external_pointer_table_entry(&mut self) {
        if !self.owning_table_.is_null() {
            unsafe {
                (*self.owning_table_).zap(self.ept_entry_);
            }
        }
        self.ept_entry_ = kNullExternalPointerHandle;
    }
}

fn is_managed_external_pointer_type(_tag: ExternalPointerTag) -> bool {
    false
}

const kMaxExternalPointers: u32 = 2048; // Adjust as needed
const kExternalPointerIndexShift: u32 = 3; // Assuming a shift of 3, adjust if needed
