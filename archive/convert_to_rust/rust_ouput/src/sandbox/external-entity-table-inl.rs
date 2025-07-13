// Converted from V8 C++ source files:
// Header: external-entity-table-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
use std::sync::{Mutex, MutexGuard, PoisonError};
    pub struct MutexGuardType {}
    impl MutexGuardType {
        pub fn AssertHeld(&self) {}
    }
    pub struct MutexType {
        pub guard: MutexGuardType,
    }
    impl MutexType {
        pub fn new() -> Self {
            MutexType {
                guard: MutexGuardType {},
            }
        }
        pub fn lock(&self) -> Result<MutexGuard<()>, PoisonError<MutexGuard<()>>> {
            let mutex = Mutex::new(());
            mutex.lock()
        }
    }
pub mod atomicops {
        use std::sync::atomic::{AtomicU32, Ordering};

        pub struct AtomicU32Wrapper {
            inner: AtomicU32,
        }

        impl AtomicU32Wrapper {
            pub fn new(value: u32) -> Self {
                AtomicU32Wrapper {
                    inner: AtomicU32::new(value),
                }
            }

            pub fn load(&self, order: Ordering) -> u32 {
                self.inner.load(order)
            }

            pub fn store(&self, value: u32, order: Ordering) {
                self.inner.store(value, order)
            }

            pub fn compare_exchange_strong(
                &self,
                current: u32,
                new: u32,
                success: Ordering,
            ) -> bool {
                self.inner.compare_exchange_strong(current, new, success, Ordering::Relaxed).is_ok()
            }
        }
    }
    pub mod emulated_virtual_address_subspace {
        use crate::Address;
        #[derive(Debug)]
        pub enum PagePermissions {
            kRead,
            kReadWrite,
            kNone,
        }
        pub struct EmulatedVirtualAddressSubspace {
            base: Address,
            size: usize,
        }

        impl EmulatedVirtualAddressSubspace {
            pub fn new(base: Address, size: usize) -> Self {
                EmulatedVirtualAddressSubspace { base, size }
            }

            pub fn base(&self) -> Address {
                self.base
            }

            pub fn size(&self) -> usize {
                self.size
            }

            pub fn allocate_pages(
                &self,
                base: Address,
                size: usize,
                alignment: usize,
                permissions: PagePermissions,
            ) -> Address {
                // In a real implementation, this would allocate memory with the given
                // permissions.  For this example, we'll just return the base address.
                println!(
                    "allocate_pages: base={:?}, size={}, alignment={}, permissions={:?}",
                    base, size, alignment, permissions
                );
                base
            }

            pub fn free_pages(&self, base: Address, size: usize) {
                // In a real implementation, this would free the memory.
                println!("free_pages: base={:?}, size={}", base, size);
            }

            pub fn set_page_permissions(
                &self,
                base: Address,
                size: usize,
                permissions: PagePermissions,
            ) -> bool {
                // In a real implementation, this would change the page permissions.
                println!(
                    "set_page_permissions: base={:?}, size={}, permissions={:?}",
                    base, size, permissions
                );
                true // Simulate success
            }
        }
    }
    pub mod iterator {
        pub struct Reversed<T> {
            data: Vec<T>,
        }

        impl<T> Reversed<T> {
            pub fn new(data: Vec<T>) -> Self {
                Reversed { data }
            }
        }

        impl<T> IntoIterator for Reversed<T> {
            type Item = T;
            type IntoIter = std::vec::IntoIter<T>;

            fn into_iter(self) -> Self::IntoIter {
                let mut data = self.data;
                data.reverse();
                data.into_iter()
            }
        }

        pub fn reversed<T>(data: &Vec<T>) -> Reversed<T>
        where
            T: Clone,
        {
            Reversed {
                data: data.clone(),
            }
        }
    }
}
pub mod common {
    pub struct AssertScope {}
}
pub mod utils {
    pub mod allocation {
        pub fn get_page_size() -> usize {
            4096
        }
    }
}
use std::sync::{Mutex, MutexGuard, PoisonError};
use std::{cmp, mem, ptr};

use self::base::atomicops::AtomicU32Wrapper;
use self::base::emulated_virtual_address_subspace::{EmulatedVirtualAddressSubspace, PagePermissions};
use self::base::iterator;
use self::base::iterator::Reversed;
use self::common::AssertScope;
use self::utils::allocation::get_page_size;
use crate::{Address, DisallowGarbageCollection, If, Set, V8};

const kEntrySize: usize = 4;
const kEntriesPerSegment: u32 = 2048;
const kSegmentSize: usize = kEntriesPerSegment as usize * kEntrySize;
const kInternalReadOnlySegmentOffset: Address = Address {};
const kInternalNullEntryIndex: u32 = 0;
const kEntryAllocationIsForbiddenMarker: u32 = u32::MAX - 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Segment {
    number: u32,
}

impl Segment {
    pub fn Containing(index: u32) -> Self {
        Segment {
            number: index / kEntriesPerSegment,
        }
    }
    pub fn At(offset: Address) -> Self {
        Segment { number: 0 }
    }
    pub fn first_entry(&self) -> u32 {
        self.number * kEntriesPerSegment
    }
    pub fn last_entry(&self) -> u32 {
        (self.number + 1) * kEntriesPerSegment - 1
    }
    pub fn number(&self) -> u32 {
        self.number
    }
    pub fn offset(&self) -> Address {
        Address {}
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FreelistHead {
    next: u32,
    length: u32,
}

impl FreelistHead {
    pub fn new(next: u32, length: u32) -> Self {
        FreelistHead { next, length }
    }
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
    pub fn next(&self) -> u32 {
        self.next
    }
    pub fn length(&self) -> u32 {
        self.length
    }
}

pub struct ExternalEntityTable<Entry, const SIZE: usize> {
    vas_: Box<EmulatedVirtualAddressSubspace>,
    spaces_: Vec<Space<Entry, SIZE>>,
    next_table_segment_number_: u32,
}

impl<Entry, const SIZE: usize> ExternalEntityTable<Entry, SIZE>
where
    Entry: ExternalEntity,
{
    const kUseContiguousMemory: bool = true;

    pub fn new() -> Self {
        ExternalEntityTable {
            vas_: Box::new(EmulatedVirtualAddressSubspace::new(Address {}, SIZE)),
            spaces_: Vec::new(),
            next_table_segment_number_: 1,
        }
    }

    fn at(&self, _index: u32) -> Entry {
        Entry::default()
    }

    fn iter_at(&self, _index: u32) -> WriteIterator<Entry, SIZE> {
        WriteIterator::new()
    }

    pub fn is_initialized(&self) -> bool {
        true
    }

    pub fn lock(&self) -> Result<MutexGuard<()>, PoisonError<MutexGuard<()>>> {
        let mutex = Mutex::new(());
        mutex.lock()
    }

    pub fn Initialize(&mut self) {
        if !ExternalEntityTable::<Entry, SIZE>::kUseContiguousMemory {
            return;
        }
        let first_segment = self.vas_.allocate_pages(
            self.vas_.base(),
            kSegmentSize,
            kSegmentSize,
            PagePermissions::kRead,
        );
        if first_segment != self.vas_.base() {
            V8::FatalProcessOutOfMemory(
                ptr::null_mut(),
                "ExternalEntityTable::InitializeTable (first segment allocation)",
            );
        }
    }

    pub fn TearDown(&mut self) {
        if ExternalEntityTable::<Entry, SIZE>::kUseContiguousMemory {
            self.vas_.free_pages(self.vas_.base(), kSegmentSize);
        }
    }

    pub fn InitializeSpace(&self, space: &mut Space<Entry, SIZE>) {
    }

    pub fn TearDownSpace(&self, space: &mut Space<Entry, SIZE>) {
        for segment in space.segments_.clone() {
            self.FreeTableSegment(segment);
        }
        space.segments_.clear();
    }

    pub fn AttachSpaceToReadOnlySegment(&self, space: &mut Space<Entry, SIZE>) {
        assert!(ExternalEntityTable::<Entry, SIZE>::kUseContiguousMemory);
        assert!(self.is_initialized());

        assert!(!space.is_internal_read_only_space());
        space.is_internal_read_only_space_ = true;

        let _unseal_scope = UnsealReadOnlySegmentScope { table: self };

        let mut freelist = FreelistHead { next: 0, length: 0 };
        {
            let _guard = space.mutex_.lock().unwrap();
            assert_eq!(space.segments_.len(), 0);
            let segment = Segment::At(kInternalReadOnlySegmentOffset);

            freelist = Self::InitializeFreeList(segment, 1);

            self.Extend(space, segment, freelist);
        }
    }

    pub fn DetachSpaceFromReadOnlySegment(&self, space: &mut Space<Entry, SIZE>) {
        let _guard = space.mutex_.lock().unwrap();
        assert_eq!(space.segments_.len(), 1);
        space.segments_.clear();
    }

    pub fn UnsealReadOnlySegment(&self) {
        let success = self.vas_.set_page_permissions(
            self.vas_.base(),
            kSegmentSize,
            PagePermissions::kReadWrite,
        );
        assert!(success);
    }

    pub fn SealReadOnlySegment(&self) {
        let success = self.vas_.set_page_permissions(
            self.vas_.base(),
            kSegmentSize,
            PagePermissions::kRead,
        );
        assert!(success);
    }

    pub fn AllocateEntry(&self, space: &mut Space<Entry, SIZE>) -> u32 {
        match self.TryAllocateEntry(space) {
            Some(res) => res,
            None => {
                V8::FatalProcessOutOfMemory(
                    ptr::null_mut(),
                    "ExternalEntityTable::AllocateEntry",
                );
                0
            }
        }
    }

    pub fn TryAllocateEntry(&self, space: &mut Space<Entry, SIZE>) -> Option<u32> {
        let _no_gc = DisallowGarbageCollection {};

        loop {
            let freelist = space.freelist_head_.load(std::sync::atomic::Ordering::Acquire);
            if freelist.is_empty() {
                let guard = space.mutex_.lock().unwrap();
                let freelist = space.freelist_head_.load(std::sync::atomic::Ordering::Relaxed);

                if freelist.is_empty() {
                    let maybe_freelist = self.TryExtend(space);
                    let freelist = match maybe_freelist {
                        Some(freelist) => freelist,
                        None => return None,
                    };
                }
            }

            let freelist = space.freelist_head_.load(std::sync::atomic::Ordering::Acquire);

            if self.TryAllocateEntryFromFreelist(space, freelist) {
                break;
            }
        }

        let freelist = space.freelist_head_.load(std::sync::atomic::Ordering::Acquire);
        let allocated_entry = freelist.next();
        assert!(space.Contains(allocated_entry));
        if !space.is_internal_read_only_space() {
            assert_ne!(allocated_entry, 0);
        }
        Some(allocated_entry)
    }

    pub fn AllocateEntryBelow(&self, space: &mut Space<Entry, SIZE>, threshold_index: u32) -> u32 {
        let mut success = false;
        let mut freelist = FreelistHead { next: 0, length: 0 };
        while !success {
            freelist = space.freelist_head_.load(std::sync::atomic::Ordering::Acquire);
            if freelist.is_empty() || freelist.next() >= threshold_index {
                return 0;
            }

            success = self.TryAllocateEntryFromFreelist(space, freelist);
        }

        let allocated_entry = freelist.next();
        assert!(space.Contains(allocated_entry));
        assert_ne!(allocated_entry, 0);
        assert!(allocated_entry < threshold_index);
        allocated_entry
    }

    pub fn TryAllocateEntryFromFreelist(&self, space: &mut Space<Entry, SIZE>, freelist: FreelistHead) -> bool {
        assert!(!freelist.is_empty());
        assert!(space.Contains(freelist.next()));

        let freelist_entry = self.at(freelist.next());
        let next_freelist_entry = freelist_entry.GetNextFreelistEntryIndex();
        let new_freelist = FreelistHead::new(next_freelist_entry, freelist.length() - 1);

        let success = space.freelist_head_.compare_exchange_strong(
            freelist,
            new_freelist,
            std::sync::atomic::Ordering::Relaxed,
        );

        if success {
            if freelist.length() > 1 {
                assert!(!new_freelist.is_empty());
            }
            if freelist.length() == 1 {
                assert!(new_freelist.is_empty());
            }
        }

        success
    }

    pub fn TryExtend(&self, space: &mut Space<Entry, SIZE>) -> Option<FreelistHead> {
        assert_eq!(space.freelist_length(), 0);
        let _guard = space.mutex_.lock().unwrap();
        assert!(!space.is_internal_read_only_space());

        let extended = self.TryAllocateAndInitializeSegment();
        if extended.is_none() {
            return None;
        }

        let (segment, freelist_head) = extended.unwrap();
        self.Extend(space, segment, freelist_head);
        Some(freelist_head)
    }

    pub fn Extend(&self, space: &mut Space<Entry, SIZE>, segment: Segment, freelist: FreelistHead) {
        assert_eq!(space.freelist_length(), 0);
        let _guard = space.mutex_.lock().unwrap();

        space.segments_.insert(segment);
        if !ExternalEntityTable::<Entry, SIZE>::kUseContiguousMemory {
            assert_ne!(segment.number(), 0);
        }
        assert_eq!(space.is_internal_read_only_space(), segment.number() == 0);

        if space.is_internal_read_only_space() {
        }

        space.freelist_head_.store(freelist, std::sync::atomic::Ordering::Release);
    }

    pub fn GenericSweep(&self, space: &mut Space<Entry, SIZE>) -> u32 {
        self.GenericSweepWithCallback(space, |_entry| {})
    }

    pub fn GenericSweepWithCallback<Callback>(&self, space: &mut Space<Entry, SIZE>, callback: Callback) -> u32
    where
        Callback: FnMut(&mut Entry),
    {
        assert!(space.BelongsTo(self));

        let _guard = space.mutex_.lock().unwrap();
        space.freelist_head_.store(
            kEntryAllocationIsForbiddenMarker,
            std::sync::atomic::Ordering::Relaxed,
        );

        let mut current_freelist_head = 0;
        let mut current_freelist_length = 0;
        let mut segments_to_deallocate: Vec<Segment> = Vec::new();

        for segment in base::iterator::reversed(&space.segments_.iter().cloned().collect()) {
            let previous_freelist_head = current_freelist_head;
            let previous_freelist_length = current_freelist_length;

            for index in (segment.first_entry()..=segment.last_entry()).rev() {
                let mut it = self.iter_at(index);
                if !it.IsMarked() {
                    it.MakeFreelistEntry(current_freelist_head);
                    current_freelist_head = it.index();
                    current_freelist_length += 1;
                } else {
                    //callback(it.entry_mut());
                    it.Unmark();
                }
            }

            let free_entries = current_freelist_length - previous_freelist_length;
            let segment_is_empty = free_entries == kEntriesPerSegment;

            if segment_is_empty {
                segments_to_deallocate.push(segment);
                current_freelist_head = previous_freelist_head;
                current_freelist_length = previous_freelist_length;
            }
        }

        for segment in segments_to_deallocate {
            assert_ne!(segment.number(), 0);
            self.FreeTableSegment(segment);
            space.segments_.remove(&segment);
        }

        let new_freelist = FreelistHead::new(current_freelist_head, current_freelist_length);
        space.freelist_head_.store(new_freelist, std::sync::atomic::Ordering::Release);
        assert_eq!(space.freelist_length(), current_freelist_length);

        let num_live_entries = space.capacity() - current_freelist_length;
        num_live_entries
    }

    pub fn IterateEntriesIn<Callback>(&self, space: &mut Space<Entry, SIZE>, callback: Callback)
    where
        Callback: FnMut(u32),
    {
        let _guard = space.mutex_.lock().unwrap();
        for segment in space.segments_.clone() {
            for i in segment.first_entry()..=segment.last_entry() {
                //callback(i);
            }
        }
    }

    fn InitializeFreeList(segment: Segment, start_index: u32) -> FreelistHead {
        FreelistHead::new(start_index, kEntriesPerSegment - start_index)
    }

    fn TryAllocateAndInitializeSegment(&self) -> Option<(Segment, FreelistHead)> {
        let segment_number = self.next_table_segment_number_;
        self.next_table_segment_number_ += 1;
        let segment = Segment {
            number: segment_number,
        };
        let freelist_head = Self::InitializeFreeList(segment, 0);
        Some((segment, freelist_head))
    }

    fn FreeTableSegment(&self, _segment: Segment) {}
}

pub trait ExternalEntity: Default {
    fn GetNextFreelistEntryIndex(&self) -> u32;
    fn SetNextFreelistEntryIndex(&mut self, index: u32);
    fn IsMarked(&self) -> bool;
    fn Mark(&mut self);
    fn Unmark(&mut self);
}

pub struct Space<Entry, const SIZE: usize> {
    mutex_: Mutex<()>,
    freelist_head_: AtomicU32Wrapper,
    segments_: Set<Segment>,
    owning_table_: *mut ExternalEntityTable<Entry, SIZE>,
    is_internal_read_only_space_: bool,
}

impl<Entry, const SIZE: usize> Space<Entry, SIZE>
where
    Entry: ExternalEntity,
{
    pub fn new() -> Self {
        Space {
            mutex_: Mutex::new(()),
            freelist_head_: AtomicU32Wrapper::new(0),
            segments_: Set::new(),
            owning_table_: ptr::null_mut(),
            is_internal_read_only_space_: false,
        }
    }

    pub fn freelist_length(&self) -> u32 {
        let freelist = self.freelist_head_.load(std::sync::atomic::Ordering::Relaxed);
        let _next = freelist.length();
        freelist.length()
    }

    pub fn num_segments(&self) -> u32 {
        let _guard = self.mutex_.lock().unwrap();
        self.segments_.len() as u32
    }

    pub fn Contains(&self, index: u32) -> bool {
        let _guard = self.mutex_.lock().unwrap();
        let segment = Segment::Containing(index);
        self.segments_.contains(&segment)
    }

    pub fn BelongsTo(&self, _table: &ExternalEntityTable<Entry, SIZE>) -> bool {
        true
    }
    pub fn is_internal_read_only_space(&self) -> bool {
        self.is_internal_read_only_space_
    }
    pub fn capacity(&self) -> u32 {
        let mut capacity = 0;
        for segment in self.segments_.clone() {
            capacity += kEntriesPerSegment;
        }
        capacity
    }
}

impl<Entry, const SIZE: usize> Drop for Space<Entry, SIZE> {
    fn drop(&mut self) {
    }
}

struct UnsealReadOnlySegmentScope<'a, Entry, const SIZE: usize> {
    table: &'a ExternalEntityTable<Entry, SIZE>,
}

impl<'a, Entry, const SIZE: usize> UnsealReadOnlySegmentScope<'a, Entry, SIZE> {
}

impl<'a, Entry, const SIZE: usize> Drop for UnsealReadOnlySegmentScope<'a, Entry, SIZE> {
    fn drop(&mut self) {
    }
}

struct WriteIterator<Entry, const SIZE: usize> {
    index: u32,
    _phantom: std::marker::PhantomData<Entry>,
}

impl<Entry, const SIZE: usize> WriteIterator<Entry, SIZE> {
    fn new() -> Self {
        WriteIterator {
            index: 0,
            _phantom: std::marker::PhantomData,
        }
    }

    fn index(&self) -> u32 {
        self.index
    }

    fn IsMarked(&self) -> bool {
        false
    }

    fn MakeFreelistEntry(&mut self, _next: u32) {}

    fn Unmark(&mut self) {}
}
