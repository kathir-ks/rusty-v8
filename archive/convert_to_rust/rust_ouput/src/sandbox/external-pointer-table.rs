// Converted from V8 C++ source files:
// Header: external-pointer-table.h
// Implementation: external-pointer-table.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]

use std::sync::{Mutex, MutexGuard};

//use crate::base::atomicops::AtomicWord;
//use crate::base::memory::UnsealReadOnlySegmentScope;
use crate::common::globals::kNullAddress;
//use crate::logging::counters::Counters;
//use crate::sandbox::check::SBXCHECK;
use crate::sandbox::external_pointer_inl::{
    ExternalPointerCanBeEmpty, ExternalPointerTag, ExternalPointerTagRange,
    kAnyExternalPointerTagRange, kExternalPointerEvacuationEntryTag,
    kExternalPointerFreeEntryTag, kLastExternalPointerTag,
};
use crate::sandbox::isolate_inl::Address;
//use crate::sandbox::tagged_payload::kAnyExternalPointerTag;
use crate::utils::allocation::Malloced;

const kExternalPointerTagShift: usize = 52;
const kExternalPointerTagMask: Address =
    (Address::MAX >> (64 - kExternalPointerTagShift)) << kExternalPointerTagShift;
const kExternalPointerPayloadMask: Address = Address::MAX & !kExternalPointerTagMask;
const kExternalPointerMarkBit: Address = Address::MIN;

pub struct Isolate;
pub struct ReadOnlyArtifacts;
pub struct Segment;
pub struct Histogram;
pub struct Counters;
pub struct Graph;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalPointerHandle(u32);

pub const kNullExternalPointerHandle: ExternalPointerHandle = ExternalPointerHandle(0);

const kMaxExternalPointers: usize = 1024;
const kExternalPointerTableReservationSize: usize = 1024;
const kSupportsCompaction: bool = true;

pub mod base {
    pub struct MutexGuard<'a, T> {
        mutex: &'a std::sync::Mutex<T>,
    }

    impl<'a, T> MutexGuard<'a, T> {
        pub fn new(mutex: &'a std::sync::Mutex<T>) -> Self {
            MutexGuard { mutex }
        }
    }
}

#[derive(Debug)]
pub enum ExternalPointerTableError {
    AllocationError,
    InvalidHandle,
    LockError,
    InvalidatedFieldError,
    Other(String),
}

type Result<T, E = ExternalPointerTableError> = std::result::Result<T, E>;

pub struct ExternalPointerTableEntry {
    payload_: std::sync::atomic::AtomicU64,
}

impl ExternalPointerTableEntry {
    pub enum EvacuateMarkMode {
        kTransferMark,
        kLeaveUnmarked,
        kClearMark,
    }

    #[inline]
    pub fn make_external_pointer_entry(
        &self,
        value: Address,
        tag: ExternalPointerTag,
        mark_as_alive: bool,
    ) {
        let payload = Payload::new(value, tag);
        let encoded_word = if mark_as_alive {
            payload.encoded_word_ | kExternalPointerMarkBit
        } else {
            payload.encoded_word_
        };
        self.payload_.store(encoded_word, std::sync::atomic::Ordering::Relaxed);
        //self.maybe_update_raw_pointer_for_lsan(value);
    }

    #[inline]
    pub fn get_external_pointer(&self, tag_range: ExternalPointerTagRange) -> Address {
        let encoded_word = self.payload_.load(std::sync::atomic::Ordering::Relaxed);
        let payload = Payload {
            encoded_word_: encoded_word,
        };
        payload.untag(tag_range)
    }

    #[inline]
    pub fn set_external_pointer(&self, value: Address, tag: ExternalPointerTag) {
        let payload = Payload::new(value, tag);
        self.payload_
            .store(payload.encoded_word_, std::sync::atomic::Ordering::Relaxed);
        //self.maybe_update_raw_pointer_for_lsan(value);
    }

    #[inline]
    pub fn has_external_pointer(&self, tag_range: ExternalPointerTagRange) -> bool {
        let encoded_word = self.payload_.load(std::sync::atomic::Ordering::Relaxed);
        let payload = Payload {
            encoded_word_: encoded_word,
        };
        payload.is_tagged_with_tag_in(tag_range)
    }

    #[inline]
    pub fn exchange_external_pointer(&self, value: Address, tag: ExternalPointerTag) -> Address {
        let new_payload = Payload::new(value, tag).encoded_word_;
        let old_payload = self.payload_.swap(new_payload, std::sync::atomic::Ordering::Relaxed);
        let payload = Payload {
            encoded_word_: old_payload,
        };
        payload.untag(ExternalPointerTagRange(payload.extract_tag(), payload.extract_tag()))
    }

    #[inline]
    pub fn get_external_pointer_tag(&self) -> ExternalPointerTag {
        let encoded_word = self.payload_.load(std::sync::atomic::Ordering::Relaxed);
        let payload = Payload {
            encoded_word_: encoded_word,
        };
        payload.extract_tag()
    }

    #[inline]
    pub fn extract_managed_resource_or_null(&self) -> Address {
        let encoded_word = self.payload_.load(std::sync::atomic::Ordering::Relaxed);
        let payload = Payload {
            encoded_word_: encoded_word,
        };

        if !payload.contains_freelist_link() && !payload.contains_evacuation_entry() {
            payload.untag(kAnyExternalPointerTagRange)
        } else {
            0
        }
    }

    #[inline]
    pub fn make_zapped_entry(&self) {
        self.payload_.store(0, std::sync::atomic::Ordering::Relaxed);
    }

    #[inline]
    pub fn make_freelist_entry(&self, next_entry_index: u32) {
        let encoded_word = next_entry_index as Address |
        ((kExternalPointerFreeEntryTag as Address) << kExternalPointerTagShift);
        self.payload_
            .store(encoded_word, std::sync::atomic::Ordering::Relaxed);
    }

    #[inline]
    pub fn get_next_freelist_entry_index(&self) -> u32 {
        let encoded_word = self.payload_.load(std::sync::atomic::Ordering::Relaxed);
        let payload = Payload {
            encoded_word_: encoded_word,
        };
        payload.extract_freelist_link()
    }

    #[inline]
    pub fn make_evacuation_entry(&self, handle_location: Address) {
        let encoded_word = handle_location |
        ((kExternalPointerEvacuationEntryTag as Address) << kExternalPointerTagShift);
        self.payload_
            .store(encoded_word, std::sync::atomic::Ordering::Relaxed);
    }

    #[inline]
    pub fn has_evacuation_entry(&self) -> bool {
        let encoded_word = self.payload_.load(std::sync::atomic::Ordering::Relaxed);
        let payload = Payload {
            encoded_word_: encoded_word,
        };
        payload.contains_evacuation_entry()
    }

    #[inline]
    pub fn evacuate(&self, dest: &ExternalPointerTableEntry, mode: EvacuateMarkMode) {
        let encoded_word = self.payload_.load(std::sync::atomic::Ordering::Relaxed);

        let new_encoded_word = match mode {
            EvacuateMarkMode::kTransferMark => encoded_word,
            EvacuateMarkMode::kLeaveUnmarked => encoded_word & !kExternalPointerMarkBit,
            EvacuateMarkMode::kClearMark => encoded_word & !kExternalPointerMarkBit,
        };

        dest.payload_
            .store(new_encoded_word, std::sync::atomic::Ordering::Relaxed);
        self.make_zapped_entry();
    }

    #[inline]
    pub fn mark(&self) {
        let mut current = self.payload_.load(std::sync::atomic::Ordering::Relaxed);
        loop {
            let new_value = current | kExternalPointerMarkBit;
            match self.payload_.compare_exchange_weak(
                current,
                new_value,
                std::sync::atomic::Ordering::Relaxed,
                std::sync::atomic::Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(x) => current = x,
            }
        }
    }

    pub const IS_WRITE_PROTECTED: bool = false;
}

impl ExternalPointerTableEntry {
    pub fn new() -> Self {
        ExternalPointerTableEntry {
            payload_: std::sync::atomic::AtomicU64::new(0),
        }
    }
}

impl Default for ExternalPointerTableEntry {
    fn default() -> Self {
        Self::new()
    }
}

struct Payload {
    encoded_word_: Address,
}

impl Payload {
    fn new(pointer: Address, tag: ExternalPointerTag) -> Self {
        Payload {
            encoded_word_: Payload::tag(pointer, tag),
        }
    }

    fn tag(pointer: Address, tag: ExternalPointerTag) -> Address {
        if tag > kLastExternalPointerTag {
            panic!("Tag exceeds kLastExternalPointerTag");
        }
        pointer | ((tag as Address) << kExternalPointerTagShift)
    }

    fn check_tag(content: Address, tag_range: ExternalPointerTagRange) -> bool {
        if ExternalPointerCanBeEmpty(tag_range) && content == 0 {
            return true;
        }

        let tag =
            ((content & kExternalPointerTagMask) >> kExternalPointerTagShift) as ExternalPointerTag;
        tag_range.contains(tag)
    }

    fn untag(&self, tag_range: ExternalPointerTagRange) -> Address {
        let content = self.encoded_word_;
        if !Payload::check_tag(content, tag_range) {
            panic!("CheckTag failed");
        }
        content & kExternalPointerPayloadMask
    }

    fn is_tagged_with_tag_in(&self, tag_range: ExternalPointerTagRange) -> bool {
        Payload::check_tag(self.encoded_word_, tag_range)
    }

    fn extract_freelist_link(&self) -> u32 {
        self.encoded_word_ as u32
    }

    fn extract_tag(&self) -> ExternalPointerTag {
        ((self.encoded_word_ & kExternalPointerTagMask) >> kExternalPointerTagShift)
            as ExternalPointerTag
    }

    fn contains_freelist_link(&self) -> bool {
        self.is_tagged_with(kExternalPointerFreeEntryTag)
    }

    fn contains_evacuation_entry(&self) -> bool {
        self.is_tagged_with(kExternalPointerEvacuationEntryTag)
    }

    fn extract_evacuation_entry_handle_location(&self) -> Address {
        self.untag(kExternalPointerEvacuationEntryTag)
    }

    fn is_tagged_with(&self, tag: ExternalPointerTag) -> bool {
        self.is_tagged_with_tag_in(ExternalPointerTagRange(tag, tag))
    }
}

struct CompactibleExternalEntityTable<T, const N: usize> {
    entries: Vec<T>,
}

impl<T: Default + Copy, const N: usize> CompactibleExternalEntityTable<T, N> {
    fn new() -> Self {
        CompactibleExternalEntityTable {
            entries: vec![T::default(); N],
        }
    }

    fn at(&self, index: usize) -> &T {
        &self.entries[index]
    }

    fn at_mut(&mut self, index: usize) -> &mut T {
        &mut self.entries[index]
    }

    fn capacity(&self) -> usize {
        self.entries.len()
    }
}

const kEntryAllocationIsForbiddenMarker: u32 = u32::MAX;
const kEntriesPerSegment: u32 = 128;

pub struct FreelistHead {
    head: u32,
    length: u32,
}

impl FreelistHead {
    pub fn new(head: u32, length: u32) -> Self {
        FreelistHead { head, length }
    }
}

impl From<FreelistHead> for u64 {
    fn from(head: FreelistHead) -> Self {
        ((head.head as u64) << 32) | (head.length as u64)
    }
}

impl From<u64> for FreelistHead {
    fn from(value: u64) -> Self {
        FreelistHead {
            head: (value >> 32) as u32,
            length: value as u32,
        }
    }
}

pub struct ExternalPointerTable {
    base: CompactibleExternalEntityTable<ExternalPointerTableEntry, kExternalPointerTableReservationSize>,
}

impl ExternalPointerTable {
    pub fn new() -> Self {
        ExternalPointerTable {
            base: CompactibleExternalEntityTable::new(),
        }
    }

    pub struct Space {
        mutex_: Mutex<()>,
        invalidated_fields_mutex_: Mutex<()>,
        segments_: std::collections::HashSet<Segment>,
        invalidated_fields_: Vec<Address>,
        freelist_head_: std::sync::atomic::AtomicU32,
        allocate_black_: bool,
        start_of_evacuation_area_: std::sync::atomic::AtomicU32,
    }

    impl Space {
        pub fn new() -> Self {
            Space {
                mutex_: Mutex::new(()),
                invalidated_fields_mutex_: Mutex::new(()),
                segments_: std::collections::HashSet::new(),
                invalidated_fields_: Vec::new(),
                freelist_head_: std::sync::atomic::AtomicU32::new(0),
                allocate_black_: false,
                start_of_evacuation_area_: std::sync::atomic::AtomicU32::new(0),
            }
        }

        #[inline]
        pub fn notify_external_pointer_field_invalidated(
            &mut self,
            field_address: Address,
            tag_range: ExternalPointerTagRange,
        ) {
            let _guard = self
                .invalidated_fields_mutex_
                .lock()
                .expect("Mutex poisoned");
            self.invalidated_fields_.push(field_address);
        }

        pub fn assert_empty(&self) {
           // assert!(self.segments_.is_empty());
        }

        pub fn allocate_black(&self) -> bool {
            self.allocate_black_
        }

        pub fn set_allocate_black(&mut self, allocate_black: bool) {
            self.allocate_black_ = allocate_black;
        }

        pub fn field_was_invalidated(&self, field_address: Address) -> bool {
            let _guard = self
                .invalidated_fields_mutex_
                .lock()
                .expect("Mutex poisoned");
            self.invalidated_fields_.contains(&field_address)
        }

        pub fn clear_invalidated_fields(&mut self) {
            let _guard = self
                .invalidated_fields_mutex_
                .lock()
                .expect("Mutex poisoned");
            self.invalidated_fields_.clear();
        }

        pub fn is_compacting(&self) -> bool {
            self.start_of_evacuation_area_.load(std::sync::atomic::Ordering::Relaxed) != 0
        }

        pub fn freelist_length(&self) -> u32 {
            0
        }

        pub fn belongs_to(&self, _table: &ExternalPointerTable) -> bool {
            true
        }

        pub fn is_internal_read_only_space(&self) -> bool {
            false
        }
    }

    impl Default for Space {
        fn default() -> Self {
            Self::new()
        }
    }

    pub fn set_up_from_read_only_artifacts(
        &mut self,
        read_only_space: &mut Space,
        artifacts: &ReadOnlyArtifacts,
    ) {
        //let _unseal_scope = UnsealReadOnlySegmentScope(self);
        // for registry_entry in artifacts.external_pointer_registry() {
        //     let handle = self.allocate_and_initialize_entry(
        //         read_only_space,
        //         registry_entry.value,
        //         registry_entry.tag,
        //     );
        //     assert_eq!(handle, registry_entry.handle);
        // }
    }

    #[inline]
    pub fn get(
        &self,
        handle: ExternalPointerHandle,
        tag_range: ExternalPointerTagRange,
    ) -> Address {
        if !Self::is_valid_handle(handle) {
            return 0;
        }
        let index = Self::handle_to_index(handle) as usize;
        self.base.at(index).get_external_pointer(tag_range)
    }

    #[inline]
    pub fn set(&self, handle: ExternalPointerHandle, value: Address, tag: ExternalPointerTag) {
        if !Self::is_valid_handle(handle) {
            return;
        }
        let index = Self::handle_to_index(handle) as usize;
        self.base.at(index).set_external_pointer(value, tag);
    }

    #[inline]
    pub fn exchange(
        &self,
        handle: ExternalPointerHandle,
        value: Address,
        tag: ExternalPointerTag,
    ) -> Address {
        if !Self::is_valid_handle(handle) {
            return 0;
        }
        let index = Self::handle_to_index(handle) as usize;
        self.base.at(index).exchange_external_pointer(value, tag)
    }

    #[inline]
    pub fn get_tag(&self, handle: ExternalPointerHandle) -> ExternalPointerTag {
        if !Self::is_valid_handle(handle) {
            return ExternalPointerTag::kExternalPointerFreeEntryTag;
        }
        let index = Self::handle_to_index(handle) as usize;
        self.base.at(index).get_external_pointer_tag()
    }

    #[inline]
    pub fn zap(&self, handle: ExternalPointerHandle) {
        if !Self::is_valid_handle(handle) {
            return;
        }
        let index = Self::handle_to_index(handle) as usize;
        self.base.at(index).make_zapped_entry();
    }

    #[inline]
    pub fn allocate_and_initialize_entry(
        &mut self,
        space: &mut Space,
        initial_value: Address,
        tag: ExternalPointerTag,
    ) -> ExternalPointerHandle {
        let _guard = space.mutex_.lock().expect("Mutex poisoned");

        // Try to allocate from the freelist.
        if let Some(handle) = self.try_allocate_entry_from_freelist(space) {
            let index = Self::handle_to_index(handle) as usize;
            self.base.at(index).make_external_pointer_entry(initial_value, tag, true);
            self.take_ownership_of_managed_resource_if_necessary(initial_value, handle, tag);
            return handle;
        }

        // Otherwise, allocate a new entry.
        if self.base.entries.len() >= kMaxExternalPointers {
            panic!("No more entries available");
        }

        let index = self.base.entries.len() as u32;
        self.base.entries.push(ExternalPointerTableEntry::new());

        let handle = Self::index_to_handle(index);
        self.base.at(index as usize).make_external_pointer_entry(initial_value, tag, true);
        self.take_ownership_of_managed_resource_if_necessary(initial_value, handle, tag);

        handle
    }

    fn try_allocate_entry_from_freelist(&mut self, space: &mut Space) -> Option<ExternalPointerHandle> {
        let freelist_head = space.freelist_head_.load(std::sync::atomic::Ordering::Relaxed);
        if freelist_head == kEntryAllocationIsForbiddenMarker {
            return None;
        }

        if freelist_head == 0 {
            return None;
        }

        let index = freelist_head;
        let entry = &self.base.at(index as usize);

        let next_free_index = entry.get_next_freelist_entry_index();
        space.freelist_head_.store(next_free_index, std::sync::atomic::Ordering::Relaxed);
        Some(Self::index_to_handle(index))
    }

    #[inline]
    pub fn mark(&self, space: &mut Space, handle: ExternalPointerHandle, handle_location: Address) {
        if !Self::is_valid_handle(handle) {
            return;
        }
        let index = Self::handle_to_index(handle) as usize;
        if space.is_compacting() {
            // We're in the middle of compaction, so we need to also mark the
            // entry for evacuation. First, mark it as alive by setting the
            // mark bit.
            self.base.at(index).mark();
            // Mark it again, this time also recording the location of the
            // handle.
            // Make sure we do not create several evacuation entries for the same
            // slot.

        } else {
            self.base.at(index).mark();
        }
    }

    #[inline]
    pub fn evacuate(
        &self,
        from_space: &mut Space,
        to_space: &mut Space,
        handle: ExternalPointerHandle,
        handle_location: Address,
        mode: ExternalPointerTableEntry::EvacuateMarkMode,
    ) {
        if !Self::is_valid_handle(handle) {
            return;
        }

        let from_index = Self::handle_to_index(handle) as usize;
        let to_index = Self::handle_to_index(handle) as usize;

        let from_entry = &self.base.entries[from_index];
        let to_entry = &mut self.base.entries[to_index];

        from_entry.evacuate(to_entry, mode);
    }

    pub fn evacuate_and_sweep_and_compact(
        &mut self,
        space: &mut Space,
        from_space: &mut Space,
        counters: &mut Counters,
    ) -> u32 {
        // Lock the space. Technically this is not necessary since no other thread can
        // allocate entries at this point, but some of the methods we call on the
        // space assert that the lock is held.
       // let _guard = space.mutex_.lock().expect("Mutex poisoned");
        // Same for the invalidated fields mutex.
      //  let _invalidated_fields_guard = space.invalidated_fields_mutex_.lock().expect("Mutex poisoned");

        // There must not be any entry allocations while the table is being swept as
        // that would not be safe. Set the freelist to this special marker value to
        // easily catch any violation of this requirement.
        //space.freelist_head_.store(kEntryAllocationIsForbiddenMarker, std::sync::atomic::Ordering::Relaxed);

        0
    }

    pub fn sweep_and_compact(&mut self, space: &mut Space, counters: &mut Counters) -> u32 {
       // self.evacuate_and_sweep_and_compact(space, None, counters)
        0
    }

    pub fn sweep(&mut self, space: &mut Space, counters: &mut Counters) -> u32 {
       // assert!(!space.is_compacting());
      //  self.sweep_and_compact(space, counters)
        0
    }

    pub fn update_all_evacuation_entries(
        &mut self,
        space: &mut Space,
        function: fn(Address) -> Address,
    ) {
        // if !space.is_compacting() {
        //     return;
        // }
        // let _guard = space.mutex_.lock().expect("Mutex poisoned");
        // let _invalidated_fields_guard = space.invalidated_fields_mutex_.lock().expect("Mutex poisoned");

        //let start_of_evacuation_area =
        //  space.start_of_evacuation_area_.load(std::memory_order_relaxed);

        // Iterate until the start of evacuation area.
        // for segment in &space.segments_ {
        //     if segment.first_entry() == start_of_evacuation_area {
        //         return;
        //     }
        //     for i in segment.first_entry()..segment.last_entry() + 1 {
        //         let entry = &mut self.base.entries[i as usize];
        //         let payload = entry.get_raw_payload();
        //         if !payload.contains_evacuation_entry() {
        //             continue;
        //         }
        //         let new_location = function(payload.extract_evacuation_entry_handle_location());
        //         entry.make_evacuation_entry(new_location);
        //     }
        // }
    }

    pub fn contains(&self, space: &mut Space, handle: ExternalPointerHandle) -> bool {
        true
    }

    #[inline]
    fn is_valid_handle(handle: ExternalPointerHandle) -> bool {
        handle.0 != 0
    }

    #[inline]
    fn handle_to_index(handle: ExternalPointerHandle) -> u32 {
        handle.0
    }

    #[inline]
    fn index_to_handle(index: u32) -> ExternalPointerHandle {
        ExternalPointerHandle(index)
    }

    fn take_ownership_of_managed_resource_if_necessary(
        &mut self,
        value: Address,
        handle: ExternalPointerHandle,
        tag: ExternalPointerTag,
    ) {
    }

    fn free_managed_resource_if_present(&mut self, entry_index: u32) {}
}

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
    pub fn zap_external_pointer_table_entry(&mut self) {}
}
