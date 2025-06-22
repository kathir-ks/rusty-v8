// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::sync::atomic::{AtomicU64, Ordering};
use std::marker::PhantomData;
//use std::mem::size_of;

// TODO: Add appropriate Rust crates for memory management, atomics, etc.

//const V8_COMPRESS_POINTERS: bool = true; // Assuming this is enabled. Conditional compilation not fully supported

pub mod internal {
    use super::*;
    pub type Address = u64; // Assuming Address is a 64-bit address

    pub type ExternalPointerHandle = u32;
    pub const kNullExternalPointerHandle: ExternalPointerHandle = 0;

    pub type ExternalPointerTag = u8;
    pub struct ExternalPointerTagRange {
        min: ExternalPointerTag,
        max: ExternalPointerTag,
    }

    impl ExternalPointerTagRange {
        pub fn new(min: ExternalPointerTag, max: ExternalPointerTag) -> Self {
            ExternalPointerTagRange { min, max }
        }
        pub fn Contains(&self, tag: ExternalPointerTag) -> bool {
          tag >= self.min && tag <= self.max
        }
        pub fn ContainsSingle(&self, tag: ExternalPointerTag) -> bool {
          self.min == tag && self.max == tag
        }
    }

    impl ExternalPointerTagRange {
        pub fn new_single(tag: ExternalPointerTag) -> Self {
            ExternalPointerTagRange { min: tag, max: tag }
        }
    }

    impl From<ExternalPointerTag> for ExternalPointerTagRange {
        fn from(tag: ExternalPointerTag) -> Self {
            ExternalPointerTagRange { min: tag, max: tag }
        }
    }
    
    pub const kLastExternalPointerTag: ExternalPointerTag = 0xF; // Example value

    pub const kExternalPointerFreeEntryTag: ExternalPointerTag = 0x1; // Example value
    pub const kExternalPointerEvacuationEntryTag: ExternalPointerTag = 0x2; // Example value

    pub const kExternalPointerTableReservationSize: usize = 1024; // Example value

    pub const kMaxCapacity: usize = 2048; // Example value
    pub const kMaxExternalPointers: usize = kMaxCapacity; // Assuming no LSan
    pub const kSupportsCompaction: bool = true;

    pub fn ExternalPointerCanBeEmpty(tag_range: ExternalPointerTagRange) -> bool {
      true // Placeholder - Replace with actual logic.
    }

    const kExternalPointerTagShift: usize = 56; // Example value
    const kExternalPointerTagMask: Address = 0xFF00000000000000; // Example value
    const kExternalPointerPayloadMask: Address = 0x00FFFFFFFFFFFFFF; // Example value
    const kExternalPointerMarkBit: Address = 0x0100000000000000; // Example value


    /// The entries of an ExternalPointerTable.
    ///
    /// Each entry consists of a single pointer-sized word containing the external
    /// pointer, the marking bit, and a type tag. An entry can either be:
    ///  - A "regular" entry, containing the external pointer together with a type
    ///    tag and the marking bit in the unused upper bits, or
    ///  - A freelist entry, tagged with the kExternalPointerFreeEntryTag and
    ///    containing the index of the next free entry in the lower 32 bits, or
    ///  - An evacuation entry, tagged with the kExternalPointerEvacuationEntryTag
    ///    and containing the address of the ExternalPointerSlot referencing the
    ///    entry that will be evacuated into this entry. See the compaction
    ///    algorithm overview for more details about these entries.
    #[derive(Debug, Clone, Copy)]
    pub struct ExternalPointerTableEntry {
        payload_: AtomicU64,
        #[cfg(feature = "leak_sanitizer")]
        raw_pointer_for_lsan_: Address,
    }

    impl ExternalPointerTableEntry {
        pub enum EvacuateMarkMode {
            kTransferMark,
            kLeaveUnmarked,
            kClearMark,
        }

      #[inline]
      fn get_raw_payload(&self) -> Payload {
          Payload { encoded_word_: self.payload_.load(Ordering::Relaxed) }
      }

      #[inline]
      fn set_raw_payload(&self, new_payload: Payload) {
          self.payload_.store(new_payload.encoded_word_, Ordering::Relaxed);
      }

        /// Make this entry an external pointer entry containing the given pointer
        /// tagged with the given tag.
        #[inline]
        pub fn make_external_pointer_entry(&self, value: Address, tag: ExternalPointerTag, mark_as_alive: bool) {
            let mut payload = Payload::new(value, tag);
            if mark_as_alive {
                payload.set_mark_bit();
            }
            self.set_raw_payload(payload);
            #[cfg(feature = "leak_sanitizer")]
            {
                self.raw_pointer_for_lsan_ = value;
            }
        }

        /// Load and untag the external pointer stored in this entry.
        /// This entry must be an external pointer entry.
        /// If the specified tag doesn't match the actual tag of this entry, the
        /// resulting pointer will be invalid and cannot be dereferenced.
        #[inline]
        pub fn get_external_pointer(&self, tag_range: ExternalPointerTagRange) -> Address {
            self.get_raw_payload().untag(tag_range)
        }

        /// Tag and store the given external pointer in this entry.
        /// This entry must be an external pointer entry.
        #[inline]
        pub fn set_external_pointer(&self, value: Address, tag: ExternalPointerTag) {
            let payload = Payload::new(value, tag);
            self.set_raw_payload(payload);
            #[cfg(feature = "leak_sanitizer")]
            {
                self.raw_pointer_for_lsan_ = value;
            }
        }

        /// Returns true if this entry contains an external pointer with the given tag.
        #[inline]
        pub fn has_external_pointer(&self, tag_range: ExternalPointerTagRange) -> bool {
            self.get_raw_payload().is_tagged_with_tag_in(tag_range)
        }

        /// Exchanges the external pointer stored in this entry with the provided one.
        /// Returns the old external pointer. This entry must be an external pointer
        /// entry. If the provided tag doesn't match the tag of the old entry, the
        /// returned pointer will be invalid.
        #[inline]
        pub fn exchange_external_pointer(&self, value: Address, tag: ExternalPointerTag) -> Address {
            let old_payload = self.get_raw_payload();
            let new_payload = Payload::new(value, tag);
            let old_address = old_payload.untag(tag);
            self.payload_.store(new_payload.encoded_word_, Ordering::Relaxed);
            #[cfg(feature = "leak_sanitizer")]
            {
                self.raw_pointer_for_lsan_ = value;
            }
            old_address
        }

        /// Load the tag of the external pointer stored in this entry.
        /// This entry must be an external pointer entry.
        #[inline]
        pub fn get_external_pointer_tag(&self) -> ExternalPointerTag {
            self.get_raw_payload().extract_tag()
        }

        /// Returns the address of the managed resource contained in this entry or
        /// nullptr if this entry does not reference a managed resource.
        #[inline]
        pub fn extract_managed_resource_or_null(&self) -> Address {
            // TODO: Implement the logic for extracting a managed resource.
            // This might involve checking the tag and returning the address if it
            // corresponds to a managed resource.
            0 // Placeholder
        }

        /// Invalidate the entry. Any access to a zapped entry will result in an
        /// invalid pointer that will crash upon dereference.
        #[inline]
        pub fn make_zapped_entry(&self) {
            self.set_raw_payload(Payload::new(0, 0)); // Or some other invalid value.
            #[cfg(feature = "leak_sanitizer")]
            {
                self.raw_pointer_for_lsan_ = 0;
            }
        }

        /// Make this entry a freelist entry, containing the index of the next entry
        /// on the freelist.
        #[inline]
        pub fn make_freelist_entry(&self, next_entry_index: u32) {
          let tagged_index = Address::from(next_entry_index) | ((Address::from(kExternalPointerFreeEntryTag)) << kExternalPointerTagShift);
          self.payload_.store(tagged_index, Ordering::Relaxed);
          #[cfg(feature = "leak_sanitizer")]
          {
              self.raw_pointer_for_lsan_ = 0;
          }
        }

        /// Get the index of the next entry on the freelist. This method may be
        /// called even when the entry is not a freelist entry. However, the result
        /// is only valid if this is a freelist entry. This behaviour is required
        /// for efficient entry allocation, see TryAllocateEntryFromFreelist.
        #[inline]
        pub fn get_next_freelist_entry_index(&self) -> u32 {
            self.get_raw_payload().extract_freelist_link()
        }

        /// Make this entry an evacuation entry containing the address of the handle to
        /// the entry being evacuated.
        #[inline]
        pub fn make_evacuation_entry(&self, handle_location: Address) {
          let tagged_handle = handle_location | ((Address::from(kExternalPointerEvacuationEntryTag)) << kExternalPointerTagShift);
          self.payload_.store(tagged_handle, Ordering::Relaxed);
          #[cfg(feature = "leak_sanitizer")]
          {
              self.raw_pointer_for_lsan_ = 0;
          }
        }

        /// Returns true if this entry contains an evacuation entry.
        #[inline]
        pub fn has_evacuation_entry(&self) -> bool {
            self.get_raw_payload().contains_evacuation_entry()
        }

        /// Move the content of this entry into the provided entry, possibly clearing
        /// the marking bit. Used during table compaction and during promotion.
        /// Invalidates the source entry.
        #[inline]
        pub fn evacuate(&self, dest: &ExternalPointerTableEntry, mode: EvacuateMarkMode) {
            let payload = self.get_raw_payload();
            match mode {
                EvacuateMarkMode::kTransferMark => {
                    // Keep the mark bit as is.
                    dest.set_raw_payload(payload);
                }
                EvacuateMarkMode::kLeaveUnmarked => {
                    // Copy and clear mark bit
                    let mut new_payload = payload;
                    new_payload.clear_mark_bit();
                    dest.set_raw_payload(new_payload);

                }
                EvacuateMarkMode::kClearMark => {
                    // Clear mark bit.
                    let mut new_payload = payload;
                    new_payload.clear_mark_bit();
                    dest.set_raw_payload(new_payload);
                }
            }

            self.make_zapped_entry(); // Invalidate the source entry.

            #[cfg(feature = "leak_sanitizer")]
            {
                dest.raw_pointer_for_lsan_ = self.raw_pointer_for_lsan_;
            }
        }

        /// Mark this entry as alive during table garbage collection.
        #[inline]
        pub fn mark(&self) {
            let mut payload = self.get_raw_payload();
            payload.set_mark_bit();
            self.set_raw_payload(payload);
        }

        pub const IS_WRITE_PROTECTED: bool = false;
    }

    impl Default for ExternalPointerTableEntry {
        fn default() -> Self {
            ExternalPointerTableEntry {
                payload_: AtomicU64::new(0),
                #[cfg(feature = "leak_sanitizer")]
                raw_pointer_for_lsan_: 0,
            }
        }
    }

    /// Helper struct to manage the tagged payload within an ExternalPointerTableEntry.
    #[derive(Debug, Clone, Copy)]
    struct Payload {
        encoded_word_: Address,
    }

    impl Payload {
        fn new(pointer: Address, tag: ExternalPointerTag) -> Self {
            Payload {
                encoded_word_: Self::tag(pointer, tag),
            }
        }

        fn tag(pointer: Address, tag: ExternalPointerTag) -> Address {
            assert!(tag <= kLastExternalPointerTag);
            pointer | ((Address::from(tag)) << kExternalPointerTagShift)
        }

        fn check_tag(content: Address, tag_range: ExternalPointerTagRange) -> bool {
            if ExternalPointerCanBeEmpty(tag_range) && content == 0 {
                return true;
            }

            let tag = (content & kExternalPointerTagMask) >> kExternalPointerTagShift;
            tag_range.Contains(tag as ExternalPointerTag)
        }

        fn untag(&self, tag_range: ExternalPointerTagRange) -> Address {
            let content = self.encoded_word_;
            assert!(Self::check_tag(content, tag_range));
            content & kExternalPointerPayloadMask
        }

        fn is_tagged_with_tag_in(&self, tag_range: ExternalPointerTagRange) -> bool {
            Self::check_tag(self.encoded_word_, tag_range)
        }

        fn contains_freelist_link(&self) -> bool {
            self.is_tagged_with_tag_in(kExternalPointerFreeEntryTag.into())
        }

        fn contains_evacuation_entry(&self) -> bool {
            self.is_tagged_with_tag_in(kExternalPointerEvacuationEntryTag.into())
        }

        fn extract_evacuation_entry_handle_location(&self) -> Address {
            self.untag(kExternalPointerEvacuationEntryTag.into())
        }

        fn extract_freelist_link(&self) -> u32 {
            self.encoded_word_ as u32
        }

        fn extract_tag(&self) -> ExternalPointerTag {
            ((self.encoded_word_ & kExternalPointerTagMask) >> kExternalPointerTagShift) as ExternalPointerTag
        }

        fn set_mark_bit(&mut self) {
            self.encoded_word_ |= kExternalPointerMarkBit;
        }

        fn clear_mark_bit(&mut self) {
            self.encoded_word_ &= !kExternalPointerMarkBit;
        }

        fn contains_pointer(&self) -> bool {
          !self.contains_freelist_link() && !self.contains_evacuation_entry()
        }
    }

    impl PartialEq for Payload {
        fn eq(&self, other: &Self) -> bool {
            self.encoded_word_ == other.encoded_word_
        }
    }

    impl Eq for Payload {}

    // TODO: Convert CompactibleExternalEntityTable to Rust
    pub struct CompactibleExternalEntityTable<T, const RESERVATION_SIZE: usize> {
        //Base functionality here
        phantom: PhantomData<T>,
    }
    impl<T, const RESERVATION_SIZE: usize> CompactibleExternalEntityTable<T,RESERVATION_SIZE> {
      pub fn dummy() {}
    }

    // TODO: Implement the Space struct and its methods

    pub struct Space {
      segments_: Vec<usize>,
      allocate_black_: bool
    }

    impl Space {
        pub fn new() -> Self {
          Space {
            segments_: Vec::new(),
            allocate_black_: false,
          }
        }
        pub fn notify_external_pointer_field_invalidated(&self, field_address: Address, tag_range: ExternalPointerTagRange) {}
        pub fn assert_empty(&self) {}
        pub fn allocate_black(&self) -> bool {self.allocate_black_}
        pub fn set_allocate_black(& mut self, allocate_black: bool) {
            self.allocate_black_ = allocate_black;
        }
    }

    /// A table storing pointers to objects outside the V8 heap.
    ///
    /// When V8_ENABLE_SANDBOX, its primary use is for pointing to objects outside
    /// the sandbox, as described below.
    /// When V8_COMPRESS_POINTERS, external pointer tables are also used to ease
    /// alignment requirements in heap object fields via indirection.
    ///
    /// A table's role for the V8 Sandbox:
    /// --------------------------------
    /// An external pointer table provides the basic mechanisms to ensure
    /// memory-safe access to objects located outside the sandbox, but referenced
    /// from within it. When an external pointer table is used, objects located
    /// inside the sandbox reference outside objects through indices into the table.
    ///
    /// Type safety can be ensured by using type-specific tags for the external
    /// pointers. These tags will be ORed into the unused top bits of the pointer
    /// when storing them and will be ANDed away when loading the pointer later
    /// again. If a pointer of the wrong type is accessed, some of the top bits will
    /// remain in place, rendering the pointer inaccessible.
    ///
    /// Temporal memory safety is achieved through garbage collection of the table,
    /// which ensures that every entry is either an invalid pointer or a valid
    /// pointer pointing to a live object.
    ///
    /// Spatial memory safety can, if necessary, be ensured either by storing the
    /// size of the referenced object together with the object itself outside the
    /// sandbox, or by storing both the pointer and the size in one (double-width)
    /// table entry.
    ///
    /// Table memory management:
    /// ------------------------
    /// The garbage collection algorithm works as follows:
    ///  - One bit of every entry is reserved for the marking bit.
    ///  - Every store to an entry automatically sets the marking bit when ORing
    ///    with the tag. This avoids the need for write barriers.
    ///  - Every load of an entry automatically removes the marking bit when ANDing
    ///    with the inverted tag.
    ///  - When the GC marking visitor finds a live object with an external pointer,
    ///    it marks the corresponding entry as alive through Mark(), which sets the
    ///    marking bit using an atomic CAS operation.
    ///  - When marking is finished, SweepAndCompact() iterates over a Space once
    ///    while the mutator is stopped and builds a freelist from all dead entries
    ///    while also possibly clearing the marking bit from any live entry.
    ///
    /// Generational collection for tables:
    /// -----------------------------------
    /// Young-generation objects with external pointer slots allocate their
    /// ExternalPointerTable entries in a spatially partitioned young external
    /// pointer space.  There are two different mechanisms:
    ///  - When using the semi-space nursery, promoting an object evacuates its EPT
    ///    entries to the old external pointer space.
    ///  - For the in-place MinorMS nursery, possibly-concurrent marking populates
    ///    the SURVIVOR_TO_EXTERNAL_POINTER remembered sets.  In the pause, promoted
    ///    objects use this remembered set to evacuate their EPT entries to the old
    ///    external pointer space.  Survivors have their EPT entries are left in
    ///    place.
    /// In a full collection, segments from the young EPT space are eagerly promoted
    /// during the pause, leaving the young generation empty.
    ///
    /// Table compaction:
    /// -----------------
    /// Additionally, the external pointer table supports compaction.
    /// For details about the compaction algorithm see the
    /// CompactibleExternalEntityTable class.
    pub struct ExternalPointerTable {
        base: CompactibleExternalEntityTable<ExternalPointerTableEntry, kExternalPointerTableReservationSize>,
        // TODO: Add fields for the actual table storage and freelist
    }

    impl ExternalPointerTable {
        pub type EvacuateMarkMode = ExternalPointerTableEntry::EvacuateMarkMode;

        pub fn new() -> Self {
            ExternalPointerTable {
                base: CompactibleExternalEntityTable {
                    phantom: PhantomData,
                },
            }
        }

        // Initializes all slots in the RO space from pre-existing artifacts.
        pub fn set_up_from_read_only_artifacts(
            &self,
            read_only_space: &Space,
            artifacts: &ReadOnlyArtifacts,
        ) {
            // TODO: Implement initialization from read-only artifacts.
        }

        /// Retrieves the entry referenced by the given handle.
        ///
        /// This method is atomic and can be called from background threads.
        #[inline]
        pub fn get(
            &self,
            handle: ExternalPointerHandle,
            tag_range: ExternalPointerTagRange,
        ) -> Address {
            assert!(Self::is_valid_handle(handle));
            //TODO: Implement table access logic
            ExternalPointerTableEntry::default().get_external_pointer(tag_range)
        }

        /// Sets the entry referenced by the given handle.
        ///
        /// This method is atomic and can be called from background threads.
        #[inline]
        pub fn set(&self, handle: ExternalPointerHandle, value: Address, tag: ExternalPointerTag) {
            assert!(Self::is_valid_handle(handle));
            //TODO: Implement table access logic
            ExternalPointerTableEntry::default().set_external_pointer(value, tag)
        }

        /// Exchanges the entry referenced by the given handle with the given value,
        /// returning the previous value. The same tag is applied both to decode the
        /// previous value and encode the given value.
        ///
        /// This method is atomic and can be called from background threads.
        #[inline]
        pub fn exchange(
            &self,
            handle: ExternalPointerHandle,
            value: Address,
            tag: ExternalPointerTag,
        ) -> Address {
            assert!(Self::is_valid_handle(handle));
            //TODO: Implement table access logic
            ExternalPointerTableEntry::default().exchange_external_pointer(value, tag)
        }

        /// Retrieves the tag used for the entry referenced by the given handle.
        ///
        /// This method is atomic and can be called from background threads.
        #[inline]
        pub fn get_tag(&self, handle: ExternalPointerHandle) -> ExternalPointerTag {
            assert!(Self::is_valid_handle(handle));
            //TODO: Implement table access logic
            ExternalPointerTableEntry::default().get_external_pointer_tag()
        }

        /// Invalidates the entry referenced by the given handle.
        #[inline]
        pub fn zap(&self, handle: ExternalPointerHandle) {
            assert!(Self::is_valid_handle(handle));
            //TODO: Implement table access logic
            ExternalPointerTableEntry::default().make_zapped_entry()
        }

        /// Allocates a new entry in the given space. The caller must provide the
        /// initial value and tag for the entry.
        ///
        /// This method is atomic and can be called from background threads.
        #[inline]
        pub fn allocate_and_initialize_entry(
            &self,
            space: &mut Space,
            initial_value: Address,
            tag: ExternalPointerTag,
        ) -> ExternalPointerHandle {
            //TODO: Implement entry allocation logic
            let handle = 1; // Placeholder
            self.take_ownership_of_managed_resource_if_necessary(initial_value, handle, tag);
            handle
        }

        /// Marks the specified entry as alive.
        ///
        /// If the space to which the entry belongs is currently being compacted, this
        /// may also mark the entry for evacuation for which the location of the
        /// handle is required. See the comments about the compaction algorithm for
        /// more details.
        ///
        /// This method is atomic and can be called from background threads.
        #[inline]
        pub fn mark(
            &self,
            space: &mut Space,
            handle: ExternalPointerHandle,
            handle_location: Address,
        ) {
            assert!(Self::is_valid_handle(handle));
            //TODO: Implement marking logic
            ExternalPointerTableEntry::default().mark()
        }

        /// Evacuate the specified entry from one space to another, updating the handle
        /// location in place.
        ///
        /// This method is not atomic and can be called only when the mutator is
        /// paused.
        #[inline]
        pub fn evacuate(
            &self,
            from_space: &mut Space,
            to_space: &mut Space,
            handle: ExternalPointerHandle,
            handle_location: Address,
            mode: EvacuateMarkMode,
        ) {
            assert!(Self::is_valid_handle(handle));
            //TODO: Implement evacuation logic
            let dest = ExternalPointerTableEntry::default();
            ExternalPointerTableEntry::default().evacuate(&dest, mode)
        }

        /// Evacuate all segments from from_space to to_space, leaving from_space empty
        /// with an empty free list.  Then free unmarked entries, finishing compaction
        /// if it was running, and collecting freed entries onto to_space's free list.
        ///
        /// The from_space will be left empty with an empty free list.
        ///
        /// This method must only be called while mutator threads are stopped as it is
        /// not safe to allocate table entries while the table is being swept.
        ///
        /// SweepAndCompact is the same as EvacuateAndSweepAndCompact, except without
        /// the evacuation phase.
        ///
        /// Sweep is the same as SweepAndCompact, but assumes that compaction was not
        /// running.
        ///
        /// Returns the number of live entries after sweeping.
        pub fn evacuate_and_sweep_and_compact(
            &self,
            to_space: &mut Space,
            from_space: &mut Space,
            counters: &mut Counters,
        ) -> u32 {
            // TODO: Implement the full logic here.
            0 // Placeholder
        }
        pub fn sweep_and_compact(&self, space: &mut Space, counters: &mut Counters) -> u32 {
            // TODO: Implement the full logic here.
            0 // Placeholder
        }
        pub fn sweep(&self, space: &mut Space, counters: &mut Counters) -> u32 {
            // TODO: Implement the full logic here.
            0 // Placeholder
        }

        /// Updates all evacuation entries with new handle locations. The function
        /// takes the old hanlde location and returns the new one.
        pub fn update_all_evacuation_entries(
            &self,
            space: &mut Space,
            update_function: impl Fn(Address) -> Address,
        ) {
            // TODO: Implement the logic here.
        }

        pub fn contains(&self, space: &mut Space, handle: ExternalPointerHandle) -> bool {
            // TODO: Implement the logic here.
            true // Placeholder
        }

        fn is_valid_handle(handle: ExternalPointerHandle) -> bool {
            handle != kNullExternalPointerHandle
        }

        fn handle_to_index(handle: ExternalPointerHandle) -> u32 {
            handle // Assuming handle is the index
        }

        fn index_to_handle(index: u32) -> ExternalPointerHandle {
            index // Assuming handle is the index
        }

        fn take_ownership_of_managed_resource_if_necessary(
            &self,
            value: Address,
            handle: ExternalPointerHandle,
            tag: ExternalPointerTag,
        ) {
            // TODO: Implement this logic
        }

        fn free_managed_resource_if_present(&self, entry_index: u32) {
            // TODO: Implement this logic
        }

        fn resolve_evacuation_entry_during_sweeping(
            &self,
            index: u32,
            handle_location: &mut ExternalPointerHandle,
            start_of_evacuation_area: u32,
        ) {
            // TODO: Implement this logic
        }
    }

    // TODO: Implement Isolate and Counters classes
    pub struct Isolate {}
    pub struct Counters {}
    pub struct ReadOnlyArtifacts {}

    impl Default for ExternalPointerTable {
        fn default() -> Self {
            Self::new()
        }
    }

    pub struct ManagedResource {
        owning_table_: *mut ExternalPointerTable,
        ept_entry_: ExternalPointerHandle,
    }

    impl ManagedResource {
        pub fn zap_external_pointer_table_entry(&mut self) {
            unsafe {
                if self.owning_table_.is_null() {
                    return;
                }
                (*self.owning_table_).zap(self.ept_entry_);
            }
        }
    }

} // end of mod internal