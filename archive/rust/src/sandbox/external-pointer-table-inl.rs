// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod external_pointer_table {
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::{marker::PhantomData, num::NonZeroU32};

    use crate::sandbox::compactible_external_entity_table::*;
    use crate::sandbox::external_pointer::*;

    const V8_COMPRESS_POINTERS: bool = true;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ExternalPointerHandle(u32);

    impl ExternalPointerHandle {
        pub fn is_null(&self) -> bool {
            self.0 == kNullExternalPointerHandle.0
        }
    }

    impl From<u32> for ExternalPointerHandle {
        fn from(value: u32) -> Self {
            ExternalPointerHandle(value)
        }
    }

    const kExternalPointerIndexShift: usize = 3; // Assuming this based on usage
    const kMaxExternalPointers: usize = 1 << 20; // Example value
    const kNullExternalPointerHandle: ExternalPointerHandle = ExternalPointerHandle(0);

    const kExternalPointerTagAndMarkbitMask: u64 = 0x00000000FF000000; // Example
    const kNullAddress: Address = 0;

    #[derive(Debug, Copy, Clone)]
    #[repr(transparent)]
    struct Payload(u64);

    impl Payload {
        fn new(value: Address, tag: ExternalPointerTag) -> Self {
            let tagged_value = (value as u64) | ((tag as u64) << 32); // Assuming tag is stored in upper 32 bits
            Payload(tagged_value)
        }

        fn contains_pointer(&self) -> bool {
            // Assuming a way to determine if payload contains a pointer
            (self.0 & 0x8000000000000000) == 0 // Example check; adjust as needed
        }

        fn contains_evacuation_entry(&self) -> bool {
            self.extract_tag() == kExternalPointerEvacuationEntryTag
        }

        fn untag(&self, _tag_range: ExternalPointerTagRange) -> Address {
            (self.0 & 0x00000000FFFFFFFF) as Address // Mask out the tag
        }

        fn extract_tag(&self) -> ExternalPointerTag {
            ((self.0 >> 32) & 0xFFFFFFFF) as ExternalPointerTag // Extract the tag from upper 32 bits
        }

        fn extract_freelist_link(&self) -> u32 {
            (self.0 & 0x00000000FFFFFFFF) as u32
        }

        fn set_mark_bit(&mut self) {
            self.0 |= 0x0000000100000000; // Example mark bit
        }

        fn has_mark_bit_set(&self) -> bool {
            (self.0 & 0x0000000100000000) != 0 // Example check
        }

        fn clear_mark_bit(&mut self) {
            self.0 &= !0x0000000100000000; // Example clear
        }

        fn is_tagged_with_tag_in(&self, tag_range: ExternalPointerTagRange) -> bool {
            tag_range.contains(self.extract_tag())
        }
    }

    #[derive(Debug)]
    pub struct ExternalPointerTableEntry {
        payload_: AtomicU64, // Changed from Payload to AtomicU64
        #[cfg(feature = "leak_sanitizer")]
        raw_pointer_for_lsan_: Address,
    }

    impl ExternalPointerTableEntry {
        pub fn new() -> Self {
            ExternalPointerTableEntry {
                payload_: AtomicU64::new(0),
                #[cfg(feature = "leak_sanitizer")]
                raw_pointer_for_lsan_: 0,
            }
        }

        pub fn make_external_pointer_entry(
            &self,
            value: Address,
            tag: ExternalPointerTag,
            mark_as_alive: bool,
        ) {
            // The 2nd most significant byte must be empty as we store the tag in int.
            debug_assert_eq!(0, value & kExternalPointerTagAndMarkbitMask);
            debug_assert_ne!(tag, kExternalPointerFreeEntryTag);
            debug_assert_ne!(tag, kExternalPointerEvacuationEntryTag);

            let mut new_payload = Payload::new(value, tag);
            if mark_as_alive {
                new_payload.set_mark_bit();
            }
            self.payload_.store(new_payload.0, Ordering::Relaxed);
            self.maybe_update_raw_pointer_for_lsan(value);
        }

        pub fn get_external_pointer(&self, tag_range: ExternalPointerTagRange) -> Address {
            let payload = Payload(self.payload_.load(Ordering::Relaxed));
            debug_assert!(payload.contains_pointer());
            payload.untag(tag_range)
        }

        pub fn set_external_pointer(&self, value: Address, tag: ExternalPointerTag) {
            // The 2nd most significant byte must be empty as we store the tag in int.
            debug_assert_eq!(0, value & kExternalPointerTagAndMarkbitMask);
            debug_assert!(Payload(self.payload_.load(Ordering::Relaxed)).contains_pointer());

            let mut new_payload = Payload::new(value, tag);
            // Writing an entry currently also marks it as alive. In the future, we might
            // want to drop this and instead use write barriers where necessary.
            new_payload.set_mark_bit();
            self.payload_.store(new_payload.0, Ordering::Relaxed);
            self.maybe_update_raw_pointer_for_lsan(value);
        }

        pub fn has_external_pointer(&self, tag_range: ExternalPointerTagRange) -> bool {
            let payload = Payload(self.payload_.load(Ordering::Relaxed));
            if !payload.contains_pointer() {
                return false;
            }
            payload.is_tagged_with_tag_in(tag_range)
        }

        pub fn exchange_external_pointer(&self, value: Address, tag: ExternalPointerTag) -> Address {
            // The 2nd most significant byte must be empty as we store the tag in int.
            debug_assert_eq!(0, value & kExternalPointerTagAndMarkbitMask);

            let mut new_payload = Payload::new(value, tag);
            // Writing an entry currently also marks it as alive. In the future, we might
            // want to drop this and instead use write barriers where necessary.
            new_payload.set_mark_bit();
            let old_payload = Payload(self.payload_.swap(new_payload.0, Ordering::Relaxed));
            debug_assert!(old_payload.contains_pointer());
            self.maybe_update_raw_pointer_for_lsan(value);
            old_payload.untag(tag)
        }

        pub fn get_external_pointer_tag(&self) -> ExternalPointerTag {
            let payload = Payload(self.payload_.load(Ordering::Relaxed));
            debug_assert!(payload.contains_pointer());
            payload.extract_tag()
        }

        pub fn extract_managed_resource_or_null(&self) -> Address {
            let payload = Payload(self.payload_.load(Ordering::Relaxed));
            let tag = payload.extract_tag();
            if is_managed_external_pointer_type(tag) {
                return payload.untag(tag);
            }
            kNullAddress
        }

        pub fn make_zapped_entry(&self) {
            let new_payload = Payload::new(kNullAddress, kExternalPointerZappedEntryTag);
            self.payload_.store(new_payload.0, Ordering::Relaxed);
        }

        pub fn make_freelist_entry(&self, next_entry_index: u32) {
            // The next freelist entry is stored in the lower bits of the entry.
            assert!(kMaxExternalPointers <= u32::MAX as usize);
            let new_payload = Payload::new(next_entry_index as Address, kExternalPointerFreeEntryTag);
            self.payload_.store(new_payload.0, Ordering::Relaxed);
        }

        pub fn get_next_freelist_entry_index(&self) -> u32 {
            let payload = Payload(self.payload_.load(Ordering::Relaxed));
            payload.extract_freelist_link()
        }

        pub fn mark(&self) {
            let old_payload = Payload(self.payload_.load(Ordering::Relaxed));
            debug_assert!(old_payload.contains_pointer());

            let mut new_payload = old_payload;
            new_payload.set_mark_bit();

            // We don't need to perform the CAS in a loop: if the new value is not equal
            // to the old value, then the mutator must've just written a new value into
            // the entry. This in turn must've set the marking bit already (see e.g.
            // SetExternalPointer), so we don't need to do it again.

            // Using compare_exchange_weak instead of compare_exchange_strong for relaxed memory ordering.
            let success = self.payload_.compare_exchange_weak(
                old_payload.0,
                new_payload.0,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ).is_ok();

            debug_assert!(success || old_payload.has_mark_bit_set());
            std::hint::black_box(success);
        }

        pub fn make_evacuation_entry(&self, handle_location: Address) {
            let new_payload = Payload::new(handle_location, kExternalPointerEvacuationEntryTag);
            self.payload_.store(new_payload.0, Ordering::Relaxed);
        }

        pub fn has_evacuation_entry(&self) -> bool {
            let payload = Payload(self.payload_.load(Ordering::Relaxed));
            payload.contains_evacuation_entry()
        }

        pub fn evacuate(&self, dest: &ExternalPointerTableEntry, mode: EvacuateMarkMode) {
            let mut payload = Payload(self.payload_.load(Ordering::Relaxed));
            // We expect to only evacuate entries containing external pointers.
            debug_assert!(payload.contains_pointer());

            match mode {
                EvacuateMarkMode::kTransferMark => {}
                EvacuateMarkMode::kLeaveUnmarked => {
                    debug_assert!(!payload.has_mark_bit_set());
                }
                EvacuateMarkMode::kClearMark => {
                    debug_assert!(payload.has_mark_bit_set());
                    payload.clear_mark_bit();
                }
            }

            dest.payload_.store(payload.0, Ordering::Relaxed);

            #[cfg(feature = "leak_sanitizer")]
            {
                dest.raw_pointer_for_lsan_ = self.raw_pointer_for_lsan_;
            }

            // The destination entry takes ownership of the pointer.
            self.make_zapped_entry();
        }

        #[cfg(feature = "leak_sanitizer")]
        fn maybe_update_raw_pointer_for_lsan(&self, value: Address) {
            self.raw_pointer_for_lsan_ = value;
        }

        #[cfg(not(feature = "leak_sanitizer"))]
        fn maybe_update_raw_pointer_for_lsan(&self, _value: Address) {}
    }

    #[derive(Debug)]
    pub struct ExternalPointerTable {
        // Assuming a vector for simplicity; can be changed to a more appropriate data structure
        entries: Vec<ExternalPointerTableEntry>,
        free_list_head: AtomicU32,
        spaces: Vec<Space>, // Added to manage spaces
        phantom: PhantomData<*const ()>,
    }

    impl ExternalPointerTable {
        pub fn new() -> Self {
            let mut table = ExternalPointerTable {
                entries: Vec::new(),
                free_list_head: AtomicU32::new(0),
                spaces: Vec::new(),
                phantom: PhantomData,
            };
            table.grow(16); // initial allocation
            table
        }

        fn grow(&mut self, count: usize) {
             let start = self.entries.len();
            self.entries.extend(
                 (0..count).map(|_| ExternalPointerTableEntry::new())
            );

            for i in start..(start + count -1) {
                self.entries[i].make_freelist_entry((i + 1) as u32);
            }

            if start + count > 0 {
                 self.entries[start + count - 1].make_freelist_entry(0);
            }

            if start == 0 {
                self.free_list_head.store(1, Ordering::Relaxed);
            }
        }

        #[inline]
        fn at(&self, index: usize) -> &ExternalPointerTableEntry {
            &self.entries[index]
        }

        pub fn get(
            &self,
            handle: ExternalPointerHandle,
            tag_range: ExternalPointerTagRange,
        ) -> Address {
            let index = Self::handle_to_index(handle);
            debug_assert!(index == 0 || self.at(index).has_external_pointer(tag_range));
            self.at(index).get_external_pointer(tag_range)
        }

        pub fn set(&self, handle: ExternalPointerHandle, value: Address, tag: ExternalPointerTag) {
            debug_assert_ne!(kNullExternalPointerHandle, handle);
            let index = Self::handle_to_index(handle);
            // TODO(saelo): This works for now, but once we actually free the external
            // object here, this will probably become awkward: it's likely not intuitive
            // that a set_foo() call on some object causes another object to be freed.
            // Probably at that point we should instead just forbid re-setting the
            // external pointers if they are managed (via a DCHECK).
            self.free_managed_resource_if_present(index);
            self.take_ownership_of_managed_resource_if_necessary(value, handle, tag);
            self.at(index).set_external_pointer(value, tag);
        }

        pub fn exchange(
            &self,
            handle: ExternalPointerHandle,
            value: Address,
            tag: ExternalPointerTag,
        ) -> Address {
            debug_assert_ne!(kNullExternalPointerHandle, handle);
            debug_assert!(!is_managed_external_pointer_type(tag));
            let index = Self::handle_to_index(handle);
            self.at(index).exchange_external_pointer(value, tag)
        }

        pub fn get_tag(&self, handle: ExternalPointerHandle) -> ExternalPointerTag {
            let index = Self::handle_to_index(handle);
            self.at(index).get_external_pointer_tag()
        }

        pub fn zap(&self, handle: ExternalPointerHandle) {
            // Zapping the null entry is a nop. This is useful as we reset the handle of
            // managed resources to the kNullExternalPointerHandle when the entry is
            // deleted. See SweepAndCompact.
            if handle == kNullExternalPointerHandle {
                return;
            }
            let index = Self::handle_to_index(handle);
            self.at(index).make_zapped_entry();
        }

        pub fn allocate_and_initialize_entry(
            &mut self,
            space: &mut Space,
            initial_value: Address,
            tag: ExternalPointerTag,
        ) -> ExternalPointerHandle {
            debug_assert!(space.belongs_to(self));
            let index = self.allocate_entry(space);
            self.at(index)
                .make_external_pointer_entry(initial_value, tag, space.allocate_black());
            let handle = Self::index_to_handle(index);
            self.take_ownership_of_managed_resource_if_necessary(initial_value, handle, tag);
            handle
        }

        pub fn mark(&self, space: &mut Space, handle: ExternalPointerHandle, handle_location: Address) {
            debug_assert!(space.belongs_to(self));

            // The handle_location must always contain the given handle. Except if the
            // slot is lazily-initialized. In that case, the handle may transition from
            // the null handle to a valid handle. However, in that case the
            // newly-allocated entry will already have been marked as alive during
            // allocation, and so we don't need to do anything here.

            // Not able to convert base::AsAtomic32::Acquire_Load since it's architecture specific.  Need further information.

            // If the handle is null, it doesn't have an EPT entry; no mark is needed.
            if handle == kNullExternalPointerHandle {
                return;
            }

            let index = Self::handle_to_index(handle);
            debug_assert!(space.contains(index));

            // If the table is being compacted and the entry is inside the evacuation
            // area, then allocate and set up an evacuation entry for it.
            self.maybe_create_evacuation_entry(space, index, handle_location);

            // Even if the entry is marked for evacuation, it still needs to be marked as
            // alive as it may be visited during sweeping before being evacuation.
            self.at(index).mark();
        }

        pub fn evacuate(
            &self,
            from_space: &mut Space,
            to_space: &mut Space,
            handle: ExternalPointerHandle,
            handle_location: Address,
            mode: EvacuateMarkMode,
        ) {
            debug_assert!(from_space.belongs_to(self));
            debug_assert!(to_space.belongs_to(self));

            debug_assert!(Self::is_valid_handle(handle));

            // Not able to convert base::AsAtomic32::Relaxed_Store since it's architecture specific.  Need further information.

            // If the handle is null, it doesn't have an EPT entry; no evacuation is
            // needed.
            if handle == kNullExternalPointerHandle {
                return;
            }

            let from_index = Self::handle_to_index(handle);
            debug_assert!(from_space.contains(from_index));
            let to_index = self.allocate_entry(to_space);

            self.at(from_index).evacuate(self.at(to_index), mode);
            let new_handle = Self::index_to_handle(to_index);

            if let Some(addr) = NonZeroU32::new(self.at(to_index).extract_managed_resource_or_null() as u32) {
                let resource = addr.as_ptr() as *mut ManagedResource;
                unsafe {
                    debug_assert_eq!((*resource).ept_entry_, handle);
                    (*resource).ept_entry_ = new_handle;
                }
            }

            // Update slot to point to new handle.
            // Not implemented AsAtomic32::Relaxed_Store.  Need further information.
            // base::AsAtomic32::Relaxed_Store(handle_ptr, new_handle);
        }

        // static
        pub fn is_valid_handle(handle: ExternalPointerHandle) -> bool {
            let index = handle.0 >> kExternalPointerIndexShift;
            handle.0 == index << kExternalPointerIndexShift
        }

        // static
        pub fn handle_to_index(handle: ExternalPointerHandle) -> usize {
            debug_assert!(Self::is_valid_handle(handle));
            let mut index = handle.0 >> kExternalPointerIndexShift;

            #[cfg(feature = "leak_sanitizer")]
            {
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
            } // LEAK_SANITIZER
            debug_assert!(index as usize <= kMaxExternalPointers);
            index as usize
        }

        // static
        pub fn index_to_handle(index: usize) -> ExternalPointerHandle {
            debug_assert!(index <= kMaxExternalPointers);
            let mut handle = (index << kExternalPointerIndexShift) as u32;

            #[cfg(feature = "leak_sanitizer")]
            {
                handle *= 2;
            } // LEAK_SANITIZER
            debug_assert_ne!(handle, kNullExternalPointerHandle.0);
            ExternalPointerHandle(handle)
        }

        pub fn contains(&self, space: &Space, handle: ExternalPointerHandle) -> bool {
            debug_assert!(space.belongs_to(self));
            space.contains(Self::handle_to_index(handle))
        }

        pub fn take_ownership_of_managed_resource_if_necessary(
            &self,
            value: Address,
            handle: ExternalPointerHandle,
            tag: ExternalPointerTag,
        ) {
            if is_managed_external_pointer_type(tag) && value != kNullAddress {
                let resource = value as *mut ManagedResource;
                unsafe {
                    debug_assert_eq!((*resource).ept_entry_, kNullExternalPointerHandle);
                    (*resource).owning_table_ = Some(self as *const _ as *mut Self);
                    (*resource).ept_entry_ = handle;
                }
            }
        }

        pub fn free_managed_resource_if_present(&self, entry_index: usize) {
            // In the future, this would be where we actually delete the external
            // resource. Currently, the deletion still happens elsewhere, and so here we
            // instead set the resource's handle to the null handle so that the resource
            // does not attempt to zap its entry when it is eventually destroyed.
            if self.at(entry_index).extract_managed_resource_or_null() != kNullAddress {
                let addr = self.at(entry_index).extract_managed_resource_or_null();
                let resource = addr as *mut ManagedResource;
                unsafe {
                    debug_assert_eq!((*resource).ept_entry_, Self::index_to_handle(entry_index));
                    (*resource).ept_entry_ = kNullExternalPointerHandle;
                }
            }
        }

        fn allocate_entry(&mut self, space: &mut Space) -> usize {
             if let Some(index) = self.try_allocate_entry() {
                space.add_index(index);
                return index;
             }
             let old_len = self.entries.len();
             self.grow(16);
             let index = old_len;
             space.add_index(index);
             return index;
        }

        fn try_allocate_entry(&self) -> Option<usize> {
            loop {
                let current_head = self.free_list_head.load(Ordering::Relaxed);
                if current_head == 0 {
                    return None;
                }
                let next_head = self.at(current_head as usize).get_next_freelist_entry_index();

                if self.free_list_head.compare_exchange_weak(
                    current_head,
                    next_head,
                    Ordering::Relaxed,
                    Ordering::Relaxed
                ).is_ok() {
                    return Some(current_head as usize);
                }
            }
        }

        fn maybe_create_evacuation_entry(&self, space: &mut Space, index: usize, handle_location: Address) {
            // unimplemented!()
        }

    }

    #[derive(Debug)]
    pub struct Space {
        table: *const ExternalPointerTable, // Raw pointer to the table
        contained_indices: Vec<usize>,      // Indices contained within this space
        invalidated_fields: Vec<Address>,
        allocate_black: bool,
    }

    impl Space {
        pub fn new(table: &ExternalPointerTable, allocate_black: bool) -> Self {
            Space {
                table,
                contained_indices: Vec::new(),
                invalidated_fields: Vec::new(),
                allocate_black,
            }
        }

        pub fn belongs_to(&self, table: &ExternalPointerTable) -> bool {
            self.table as *const _ == table as *const _
        }

        pub fn contains(&self, index: usize) -> bool {
            self.contained_indices.contains(&index)
        }

        pub fn add_index(&mut self, index: usize) {
            self.contained_indices.push(index);
        }

        pub fn allocate_black(&self) -> bool {
            self.allocate_black
        }

        pub fn notify_external_pointer_field_invalidated(
            &mut self,
            field_address: Address,
            tag_range: ExternalPointerTagRange,
        ) {
            // We do not currently support invalidating fields containing managed
            // external pointers. If this is ever needed, we would probably need to free
            // the managed object here as we may otherwise fail to do so during sweeping.
            debug_assert!(!is_managed_external_pointer_type(tag_range));
            // base::AsAtomic32::Acquire_Load is not implemented.  Need further information.

            self.add_invalidated_field(field_address);
        }

        fn add_invalidated_field(&mut self, field_address: Address) {
            self.invalidated_fields.push(field_address);
        }
    }

    #[derive(Debug)]
    #[repr(C)]
    pub struct ManagedResource {
        ept_entry_: ExternalPointerHandle,
        owning_table_: Option<*mut ExternalPointerTable>,
    }

    impl ManagedResource {
        pub fn new() -> Self {
            ManagedResource {
                ept_entry_: kNullExternalPointerHandle,
                owning_table_: None,
            }
        }

        pub fn zap_external_pointer_table_entry(&mut self) {
            if let Some(owning_table_ptr) = self.owning_table_ {
                let owning_table = unsafe { &mut *owning_table_ptr };
                owning_table.zap(self.ept_entry_);
            }
            self.ept_entry_ = kNullExternalPointerHandle;
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub enum EvacuateMarkMode {
        kTransferMark,
        kLeaveUnmarked,
        kClearMark,
    }
}

pub mod compactible_external_entity_table {
    use crate::sandbox::external_pointer::*;
    pub type Address = u64;

    pub type ExternalPointerTag = u32;

    pub type ExternalPointerTagRange = std::ops::RangeInclusive<ExternalPointerTag>;

    pub const kExternalPointerFreeEntryTag: ExternalPointerTag = 0;
    pub const kExternalPointerZappedEntryTag: ExternalPointerTag = 1;
    pub const kExternalPointerEvacuationEntryTag: ExternalPointerTag = 2;

    pub fn is_managed_external_pointer_type(_tag: ExternalPointerTagRange) -> bool {
        false
    }

    pub fn is_managed_external_pointer_type(_tag: ExternalPointerTag) -> bool {
        false
    }
}

pub mod external_pointer {
    pub type Address = u64;
}