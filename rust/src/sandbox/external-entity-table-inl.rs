// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod external_entity_table {
    use std::sync::{Mutex, MutexGuard};
    use std::collections::HashSet;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::mem;
    use std::cmp;

    // Placeholder for EmulatedVirtualAddressSubspace
    pub struct EmulatedVirtualAddressSubspace {}
    impl EmulatedVirtualAddressSubspace {
        pub fn allocate_pages(&self, base: Address, size: usize, alignment: usize, permissions: PagePermissions) -> Address {
            // Placeholder implementation
            base
        }
        pub fn free_pages(&self, base: Address, size: usize) {
            // Placeholder implementation
        }
        pub fn set_page_permissions(&self, base: Address, size: usize, permissions: PagePermissions) -> bool {
            // Placeholder implementation
            true
        }
        pub fn base(&self) -> Address { Address(0) }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct Address(usize);

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum PagePermissions {
        KRead,
        KReadWrite,
    }

    pub struct Segment {
        number: u32,
    }

    impl Segment {
        pub fn at(offset: usize) -> Self {
            Segment { number: (offset / kSegmentSize) as u32 }
        }

        pub fn containing(index: u32) -> Self {
            Segment { number: (index / kEntriesPerSegment) as u32 }
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

        pub fn offset(&self) -> usize {
            (self.number as usize) * kSegmentSize
        }
    }

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

    // Constants
    const kSegmentSize: usize = 4096; // Example size
    const kEntriesPerSegment: u32 = 1024; // Example size
    const kInternalReadOnlySegmentOffset: usize = 0;
    const kInternalNullEntryIndex: u32 = 0;
    const kEntryAllocationIsForbiddenMarker: FreelistHead = FreelistHead { next: u32::MAX, length: u32::MAX };
    const kNullAddress: Address = Address(0);
    const kEntrySize: usize = 8;

    // Base trait (not fully translated, functionality may differ)
    pub trait BaseTable {
        fn initialize(&mut self);
        fn tear_down(&mut self);
    }

    pub struct UnsealReadOnlySegmentScope<'a, Entry, const SIZE: usize> {
        table: &'a mut ExternalEntityTable<Entry, SIZE>,
    }

    impl<'a, Entry, const SIZE: usize> UnsealReadOnlySegmentScope<'a, Entry, SIZE> {
        pub fn new(table: &'a mut ExternalEntityTable<Entry, SIZE>) -> Self {
            table.unseal_read_only_segment();
            UnsealReadOnlySegmentScope { table }
        }
    }

    impl<'a, Entry, const SIZE: usize> Drop for UnsealReadOnlySegmentScope<'a, Entry, SIZE> {
        fn drop(&mut self) {
            self.table.seal_read_only_segment();
        }
    }

    /// A table for managing external entities.
    pub struct ExternalEntityTable<Entry, const SIZE: usize> {
        vas_: Box<EmulatedVirtualAddressSubspace>,
        is_initialized_: bool,
        // Placeholder for Base class
    }

    impl<Entry, const SIZE: usize> ExternalEntityTable<Entry, const SIZE> {
        pub const kUseContiguousMemory: bool = true;

        pub fn new() -> Self {
            ExternalEntityTable {
                vas_: Box::new(EmulatedVirtualAddressSubspace {}),
                is_initialized_: false,
            }
        }

        pub fn initialize(&mut self) {
            // Base::Initialize();
            self.is_initialized_ = true;

            if !ExternalEntityTable::<Entry, SIZE>::kUseContiguousMemory {
                return;
            }

            let first_segment = self.vas_.allocate_pages(
                self.vas_.base(),
                kSegmentSize,
                kSegmentSize,
                PagePermissions::KRead,
            );

            if first_segment != self.vas_.base() {
                panic!("ExternalEntityTable::InitializeTable (first segment allocation)");
            }

            assert_eq!((first_segment.0 as isize) - (self.vas_.base().0 as isize), kInternalReadOnlySegmentOffset as isize);
        }

        pub fn tear_down(&mut self) {
            assert!(self.is_initialized());

            if ExternalEntityTable::<Entry, SIZE>::kUseContiguousMemory {
                self.vas_.free_pages(self.vas_.base(), kSegmentSize);
            }

            //Base::TearDown();
            self.is_initialized_ = false;
        }

        pub fn is_initialized(&self) -> bool {
            self.is_initialized_
        }

        pub fn at(&self, index: u32) -> &Entry {
            unsafe {
                let address = self.vas_.base().0 + (index * mem::size_of::<Entry>() as u32) as usize;
                &*(address as *const Entry)
            }
        }

        pub fn initialize_space(&self, space: &mut Space<Entry, SIZE>) {
            space.owning_table = Some(self);
        }

        pub fn tear_down_space(&self, space: &mut Space<Entry, SIZE>) {
            assert!(self.is_initialized());
            assert!(space.belongs_to(self));
            for segment in space.segments_.iter() {
                self.free_table_segment(*segment);
            }
            space.segments_.clear();
        }

        pub fn attach_space_to_read_only_segment(&mut self, space: &mut Space<Entry, SIZE>) {
            assert!(ExternalEntityTable::<Entry, SIZE>::kUseContiguousMemory);
            assert!(self.is_initialized());
            assert!(space.belongs_to(self));

            assert!(!space.is_internal_read_only_space());
            space.is_internal_read_only_space_ = true;

            let _unseal_scope = UnsealReadOnlySegmentScope::new(self);

            let mut freelist: FreelistHead = FreelistHead { next: 0, length: 0 };
            {
                let mut guard = space.mutex_.lock().unwrap();
                assert_eq!(space.segments_.len(), 0);
                let segment = Segment::at(kInternalReadOnlySegmentOffset);
                assert_eq!(segment.first_entry(), kInternalNullEntryIndex);

                freelist = self.initialize_free_list(segment, 1);

                self.extend(space, segment, freelist);
            }

            assert!(!freelist.is_empty());
            assert_eq!(freelist.next(), kInternalNullEntryIndex + 1);
            assert!(space.contains(freelist.next()));
        }

        pub fn detach_space_from_read_only_segment(&self, space: &mut Space<Entry, SIZE>) {
            assert!(self.is_initialized());
            assert!(space.belongs_to(self));

            let mut guard = space.mutex_.lock().unwrap();
            assert_eq!(space.segments_.len(), 1);
            space.segments_.clear();
        }

        pub fn unseal_read_only_segment(&mut self) {
            assert!(self.is_initialized());
            let success = self.vas_.set_page_permissions(
                self.vas_.base(),
                kSegmentSize,
                PagePermissions::KReadWrite,
            );
            assert!(success);
        }

        pub fn seal_read_only_segment(&mut self) {
            assert!(self.is_initialized());
            let success = self.vas_.set_page_permissions(
                self.vas_.base(),
                kSegmentSize,
                PagePermissions::KRead,
            );
            assert!(success);
        }

        pub fn allocate_entry(&mut self, space: &mut Space<Entry, SIZE>) -> u32 {
            match self.try_allocate_entry(space) {
                Some(res) => res,
                None => panic!("ExternalEntityTable::AllocateEntry"), // Simulate FatalProcessOutOfMemory
            }
        }

        pub fn try_allocate_entry(&mut self, space: &mut Space<Entry, SIZE>) -> Option<u32> {
            //DisallowGarbageCollection no_gc; // Placeholder - Rust's ownership prevents unexpected GC

            let mut freelist: FreelistHead;
            loop {
                freelist = space.freelist_head_.load(Ordering::Acquire);

                if freelist.is_empty() {
                    let mut guard = space.mutex_.lock().unwrap();
                    freelist = space.freelist_head_.load(Ordering::Relaxed);

                    if freelist.is_empty() {
                        if let Some(maybe_freelist) = self.try_extend(space) {
                            freelist = maybe_freelist;
                        } else {
                            return None;
                        }
                        assert_eq!(freelist.length(), kEntriesPerSegment);
                    }
                }

                if self.try_allocate_entry_from_freelist(space, freelist) {
                    break;
                }
            }

            let allocated_entry = freelist.next();
            assert!(space.contains(allocated_entry));
            if !space.is_internal_read_only_space() {
                assert_ne!(allocated_entry, 0);
            }
            Some(allocated_entry)
        }

        pub fn allocate_entry_below(&mut self, space: &mut Space<Entry, SIZE>, threshold_index: u32) -> u32 {
            assert!(self.is_initialized());

            let mut freelist: FreelistHead;
            let mut success = false;
            while !success {
                freelist = space.freelist_head_.load(Ordering::Acquire);
                if freelist.is_empty() || freelist.next() >= threshold_index {
                    return 0;
                }

                success = self.try_allocate_entry_from_freelist(space, freelist);
            }

            let allocated_entry = freelist.next();
            assert!(space.contains(allocated_entry));
            assert_ne!(allocated_entry, 0);
            assert!(allocated_entry < threshold_index);
            return allocated_entry;
        }

        fn try_allocate_entry_from_freelist(&self, space: &mut Space<Entry, SIZE>, freelist: FreelistHead) -> bool {
            assert!(!freelist.is_empty());
            assert!(space.contains(freelist.next()));

            let freelist_entry = self.at(freelist.next()); // This line may need a workaround because of mutability, since at returns a shared reference
            let next_freelist_entry = freelist_entry.get_next_freelist_entry_index(); // Assuming get_next_freelist_entry_index is defined on Entry
            let new_freelist = FreelistHead::new(next_freelist_entry, freelist.length() - 1);

            let success = space.freelist_head_.compare_exchange(
                freelist,
                new_freelist,
                Ordering::Relaxed,
                Ordering::Relaxed, // failure ordering can be relaxed as well
            ).is_ok();

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

        fn try_extend(&mut self, space: &mut Space<Entry, SIZE>) -> Option<FreelistHead> {
            assert_eq!(space.freelist_length(), 0);
            let _guard = space.mutex_.lock().unwrap();
            assert!(!space.is_internal_read_only_space());

            match self.try_allocate_and_initialize_segment() {
                Some((segment, freelist_head)) => {
                    self.extend(space, segment, freelist_head);
                    Some(freelist_head)
                }
                None => None,
            }
        }

        fn extend(&self, space: &mut Space<Entry, SIZE>, segment: Segment, freelist: FreelistHead) {
            assert_eq!(space.freelist_length(), 0);
            let _guard = space.mutex_.lock().unwrap();

            space.segments_.insert(segment);
            if ExternalEntityTable::<Entry, SIZE>::kUseContiguousMemory {
                assert_ne!(segment.number(), 0);
            }
            assert_eq!(space.is_internal_read_only_space(), segment.number() == 0);
            assert_eq!(space.is_internal_read_only_space(), segment.offset() == kInternalReadOnlySegmentOffset);

            if space.is_internal_read_only_space() {
                // Null entry handling
                // Assuming the memory is zeroed on allocation in `EmulatedVirtualAddressSubspace`.
                // Verification skipped.
            }

            space.freelist_head_.store(freelist, Ordering::Release);
        }

        fn generic_sweep(&mut self, space: &mut Space<Entry, SIZE>) -> u32 {
            self.generic_sweep_with_callback(space, |_entry| {})
        }

        fn generic_sweep_with_callback<Callback>(&mut self, space: &mut Space<Entry, SIZE>, callback: Callback) -> u32
        where
            Callback: FnMut(&mut Entry), // Pass entry by mutable reference
        {
            assert!(space.belongs_to(self));

            let _guard = space.mutex_.lock().unwrap();

            space.freelist_head_.store(kEntryAllocationIsForbiddenMarker, Ordering::Relaxed);

            let mut current_freelist_head = 0;
            let mut current_freelist_length = 0;
            let mut segments_to_deallocate: Vec<Segment> = Vec::new();

            for segment in space.segments_.iter().rev() {
                let previous_freelist_head = current_freelist_head;
                let previous_freelist_length = current_freelist_length;

                // Iterate through entries in reverse order within the segment
                let mut it = WriteIterator::new(self, segment.last_entry());
                while it.index() >= segment.first_entry() {
                    if !it.is_marked() {
                        it.make_freelist_entry(current_freelist_head);
                        current_freelist_head = it.index();
                        current_freelist_length += 1;
                    } else {
                        //The callback should accept a mutable reference to the Entry
                        callback(it.get_mut());
                        it.unmark();
                    }
                    it.decrement();
                }

                let free_entries = current_freelist_length - previous_freelist_length;
                let segment_is_empty = free_entries == kEntriesPerSegment;

                if segment_is_empty {
                    segments_to_deallocate.push(*segment); // Dereference to copy Segment
                    current_freelist_head = previous_freelist_head;
                    current_freelist_length = previous_freelist_length;
                }
            }

            for segment in segments_to_deallocate {
                assert_ne!(segment.number(), 0);
                self.free_table_segment(segment);
                space.segments_.remove(&segment);
            }

            let new_freelist = FreelistHead::new(current_freelist_head, current_freelist_length);
            space.freelist_head_.store(new_freelist, Ordering::Release);
            assert_eq!(space.freelist_length(), current_freelist_length);

            let num_live_entries = space.capacity() - current_freelist_length;
            num_live_entries
        }

        fn iterate_entries_in<Callback>(&self, space: &Space<Entry, SIZE>, callback: Callback)
        where
            Callback: FnMut(u32),
        {
            let _guard = space.mutex_.lock().unwrap();

            for segment in &space.segments_ {
                for i in segment.first_entry()..=segment.last_entry() {
                    callback(i);
                }
            }
        }

        fn try_allocate_and_initialize_segment(&mut self) -> Option<(Segment, FreelistHead)> {
            // Placeholder implementation.  Needs proper memory allocation and initialization.
            // Returns a dummy segment for now.
            let num_segments = 0;
            let segment = Segment { number: num_segments + 1 };

            let freelist_head = self.initialize_free_list(segment, 0); // Changed offset to 0

            Some((segment, freelist_head))
        }

        fn free_table_segment(&self, segment: Segment) {
            // Placeholder implementation.  Needs proper memory deallocation.
        }

        fn initialize_free_list(&self, segment: Segment, start_index: u32) -> FreelistHead {
            let mut current_index = start_index;
            let end_index = segment.last_entry();

            if current_index > end_index {
                return FreelistHead { next: 0, length: 0 }; //Empty list
            }

            let mut length = 0;
            while current_index <= end_index {
                if current_index < end_index {
                    let next_index = current_index + 1;
                    self.at(current_index).set_next_freelist_entry_index(next_index);
                }

                length += 1;
                current_index += 1;
            }

            FreelistHead::new(start_index, length)
        }
    }

    impl<Entry, const SIZE: usize> Default for ExternalEntityTable<Entry, SIZE> {
        fn default() -> Self {
            Self::new()
        }
    }
    
    /// Represents a memory space within the external entity table.
    pub struct Space<Entry, const SIZE: usize> {
        mutex_: Mutex<()>,
        segments_: HashSet<Segment>,
        freelist_head_: AtomicU32,
        owning_table: Option<*const ExternalEntityTable<Entry, SIZE>>,
        is_internal_read_only_space_: bool,
    }

    impl<Entry, const SIZE: usize> Space<Entry, SIZE> {
        pub fn new() -> Self {
            Space {
                mutex_: Mutex::new(()),
                segments_: HashSet::new(),
                freelist_head_: AtomicU32::new(0),
                owning_table: None,
                is_internal_read_only_space_: false,
            }
        }

        pub fn freelist_length(&self) -> u32 {
            let freelist = self.freelist_head_.load(Ordering::Relaxed);
            if freelist == kEntryAllocationIsForbiddenMarker.next() {
                0
            } else {
                let head = unsafe {&*(self.freelist_head_.as_ptr() as *const FreelistHead)};
                head.length()
            }
        }

        pub fn num_segments(&self) -> u32 {
            let guard = self.mutex_.lock().unwrap();
            self.segments_.len() as u32
        }

        pub fn contains(&self, index: u32) -> bool {
            let guard = self.mutex_.lock().unwrap();
            let segment = Segment::containing(index);
            self.segments_.contains(&segment)
        }

        pub fn belongs_to(&self, table: &ExternalEntityTable<Entry, SIZE>) -> bool {
            match self.owning_table {
                Some(ptr) => ptr == table,
                None => false,
            }
        }

        pub fn is_internal_read_only_space(&self) -> bool {
            self.is_internal_read_only_space_
        }

        pub fn capacity(&self) -> u32 {
            let mut capacity = 0;
            let _guard = self.mutex_.lock().unwrap();
            for segment in &self.segments_ {
                capacity += kEntriesPerSegment;
            }
            capacity
        }
    }

    impl<Entry, const SIZE: usize> Drop for Space<Entry, SIZE> {
        fn drop(&mut self) {
            assert!(self.segments_.is_empty());
        }
    }

    // Placeholder for Entry methods (adapt based on the Entry struct's actual members)
    pub trait EntryMethods {
        fn is_marked(&self) -> bool;
        fn unmark(&mut self);
        fn make_freelist_entry(&mut self, next: u32);
        fn set_next_freelist_entry_index(&self, index: u32);
        fn get_next_freelist_entry_index(&self) -> u32;
    }

    // Dummy implementation for testing. Replace this with the real Entry struct
    #[derive(Copy, Clone)]
    pub struct DummyEntry {
        marked: bool,
        next_freelist_entry: u32,
    }

    impl DummyEntry {
        pub fn new() -> Self {
            DummyEntry {
                marked: false,
                next_freelist_entry: 0,
            }
        }
    }

    impl EntryMethods for DummyEntry {
        fn is_marked(&self) -> bool {
            self.marked
        }

        fn unmark(&mut self) {
            self.marked = false;
        }

        fn make_freelist_entry(&mut self, next: u32) {
            self.next_freelist_entry = next;
        }

        fn set_next_freelist_entry_index(&self, index: u32) {
            self.next_freelist_entry = index;
        }

        fn get_next_freelist_entry_index(&self) -> u32 {
            self.next_freelist_entry
        }
    }

    // WriteIterator struct
    pub struct WriteIterator<'a, Entry, const SIZE: usize> {
        table: &'a mut ExternalEntityTable<Entry, SIZE>,
        index: u32,
    }

    impl<'a, Entry, const SIZE: usize> WriteIterator<'a, Entry, SIZE> {
        pub fn new(table: &'a mut ExternalEntityTable<Entry, SIZE>, index: u32) -> Self {
            WriteIterator { table, index }
        }

        pub fn index(&self) -> u32 {
            self.index
        }

        pub fn get_mut(&mut self) -> &mut Entry {
          unsafe {
              let address = self.table.vas_.base().0 + (self.index * mem::size_of::<Entry>() as u32) as usize;
              &mut *(address as *mut Entry)
          }
        }

        pub fn is_marked(&self) -> bool {
          unsafe {
              let address = self.table.vas_.base().0 + (self.index * mem::size_of::<Entry>() as u32) as usize;
              (*(address as *const Entry)).is_marked()
          }
        }

        pub fn unmark(&mut self) {
          unsafe {
              let address = self.table.vas_.base().0 + (self.index * mem::size_of::<Entry>() as u32) as usize;
              (*(address as *mut Entry)).unmark();
          }
        }

        pub fn make_freelist_entry(&mut self, next: u32) {
          unsafe {
              let address = self.table.vas_.base().0 + (self.index * mem::size_of::<Entry>() as u32) as usize;
              (*(address as *mut Entry)).make_freelist_entry(next);
          }
        }

        pub fn decrement(&mut self) {
            self.index = self.index.saturating_sub(1);
        }
    }
}