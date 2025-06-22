// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// #![allow(dead_code)] // Suppress warnings about unused code

use std::{
    marker::PhantomData,
    mem::{size_of, MaybeUninit},
    num::NonZeroU32,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicU32, Ordering},
};

const KB: usize = 1024;

trait FreelistEntry {
    /// Must implement `IsWriteProtected` as a const boolean.
    const IS_WRITE_PROTECTED: bool;

    /// Sets the `next_entry_index` for this entry, encoding it into the entry itself.
    fn make_freelist_entry(&mut self, next_entry_index: u32);

    /// Retrieves the `next_entry_index` from this entry.  Should return 0 if this is the end of the list.
    fn get_next_freelist_entry(&self) -> u32;
}

/// A thread-safe table with a fixed maximum size split into segments.
///
/// The table provides thread-safe methods to allocate and free of segments and
/// an inline freelist implementation. Allocation and Freeing of entries is
/// implemented in subclasses since it depends on if the table is manually
/// managed or GCed.
///
/// For the purpose of memory management, the table is partitioned into Segments
/// (for example 64kb memory chunks) that are grouped together in "Spaces". All
/// segments in a space share a freelist, and so entry allocation and garbage
/// collection happen on the level of spaces.
///
/// The Entry type defines how the freelist is represented.
pub struct SegmentedTable<Entry: FreelistEntry, const SIZE: usize> {
    base_: *mut Entry,
    vas_: VirtualAddressSpace,
    _phantom: PhantomData<Entry>,
}

impl<Entry: FreelistEntry, const SIZE: usize> SegmentedTable<Entry, SIZE> {
    const K_IS_WRITE_PROTECTED: bool = Entry::IS_WRITE_PROTECTED;
    const K_ENTRY_SIZE: usize = size_of::<Entry>();

    #[cfg(target_arch = "x86_64")]
    const K_USE_CONTIGUOUS_MEMORY: bool = true;
    #[cfg(not(target_arch = "x86_64"))]
    const K_USE_CONTIGUOUS_MEMORY: bool = false;

    #[cfg(target_arch = "x86_64")]
    const K_RESERVATION_SIZE: usize = SIZE;
    #[cfg(target_arch = "x86_64")]
    const K_MAX_CAPACITY: usize = Self::K_RESERVATION_SIZE / Self::K_ENTRY_SIZE;

    const K_SEGMENT_SIZE: usize = 64 * KB;
    const K_ENTRIES_PER_SEGMENT: usize = Self::K_SEGMENT_SIZE / Self::K_ENTRY_SIZE;

    /// Struct representing a segment of the table.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Segment {
        number_: u32,
    }

    impl Segment {
        /// Initialize a segment given its number.
        pub fn new(number: u32) -> Self {
            Segment { number_: number }
        }

        /// Returns the segment starting at the specified offset from the base of the
        /// table.
        pub fn at(offset: u32) -> Self {
            Segment {
                number_: offset / Self::K_SEGMENT_SIZE as u32,
            }
        }

        /// Returns the segment containing the entry at the given index.
        pub fn containing(entry_index: u32) -> Self {
            Segment {
                number_: entry_index / Self::K_ENTRIES_PER_SEGMENT as u32,
            }
        }

        /// The segments of a table are numbered sequentially. This method returns
        /// the number of this segment.
        pub fn number(&self) -> u32 {
            self.number_
        }

        /// Returns the offset of this segment from the table base.
        pub fn offset(&self) -> u32 {
            self.number_ * Self::K_SEGMENT_SIZE as u32
        }

        /// Returns the index of the first entry in this segment.
        pub fn first_entry(&self) -> u32 {
            self.number_ * Self::K_ENTRIES_PER_SEGMENT as u32
        }

        /// Return the index of the last entry in this segment.
        pub fn last_entry(&self) -> u32 {
            self.first_entry() + Self::K_ENTRIES_PER_SEGMENT as u32 - 1
        }
    }

    /// Struct representing the head of the freelist.
    #[derive(Debug, Clone, Copy)]
    pub struct FreelistHead {
        next_: u32,
        length_: u32,
    }

    impl FreelistHead {
        pub const fn new(next: u32, length: u32) -> Self {
            FreelistHead { next_: next, length_: length }
        }

        /// Returns the index of the next entry on the freelist.
        /// If the freelist is empty, this returns zero.
        pub fn next(&self) -> u32 {
            self.next_
        }

        /// Returns the total length of the freelist.
        pub fn length(&self) -> u32 {
            self.length_
        }

        pub fn is_empty(&self) -> bool {
            self.length_ == 0
        }
    }

    /// This Iterator also acts as a scope object to temporarily lift any
    /// write-protection (if kIsWriteProtected is true).
    pub struct WriteIterator<'a, E: FreelistEntry> {
        base_: *mut E,
        index_: u32,
        write_scope_: WriteScope<E>,
        _phantom: PhantomData<&'a mut E>,
        #[cfg(debug_assertions)]
        crossed_segment_: bool,
    }

    impl<'a, E: FreelistEntry> WriteIterator<'a, E> {
        pub fn new(base: *mut E, index: u32) -> Self {
            WriteIterator {
                base_: base,
                index_: index,
                write_scope_: WriteScope::new(base),
                _phantom: PhantomData,
                #[cfg(debug_assertions)]
                crossed_segment_: false,
            }
        }

        pub fn index(&self) -> u32 {
            self.index_
        }
    }

    impl<'a, E: FreelistEntry> Deref for WriteIterator<'a, E> {
        type Target = E;

        fn deref(&self) -> &Self::Target {
            #[cfg(debug_assertions)]
            assert!(!self.crossed_segment_);
            unsafe { &*self.base_.add(self.index_ as usize) }
        }
    }

    impl<'a, E: FreelistEntry> DerefMut for WriteIterator<'a, E> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            #[cfg(debug_assertions)]
            assert!(!self.crossed_segment_);
            unsafe { &mut *self.base_.add(self.index_ as usize) }
        }
    }

    impl<'a, E: FreelistEntry> Drop for WriteIterator<'a, E> {
        fn drop(&mut self) {
            // The WriteScope is dropped here, releasing the write protection
        }
    }

    impl<'a, E: FreelistEntry> WriteIterator<'a, E> {
        pub fn increment(&mut self) -> &mut Self {
            self.index_ += 1;
            #[cfg(debug_assertions)]
            {
                if self.index_ % Self::K_ENTRIES_PER_SEGMENT as u32 == 0 {
                    self.crossed_segment_ = true;
                }
            }
            self
        }

        pub fn decrement(&mut self) -> &mut Self {
            assert!(self.index_ > 0);
            #[cfg(debug_assertions)]
            {
                if self.index_ % Self::K_ENTRIES_PER_SEGMENT as u32 == 0 {
                    self.crossed_segment_ = true;
                }
            }
            self.index_ -= 1;
            self
        }
    }

    pub fn new() -> Self {
        SegmentedTable {
            base_: std::ptr::null_mut(),
            vas_: VirtualAddressSpace::new(),
            _phantom: PhantomData,
        }
    }

    /// Access the entry at the specified index.
    pub fn at(&self, index: u32) -> &Entry {
        assert!(self.is_initialized());
        unsafe { &*self.base_.add(index as usize) }
    }

    /// Access the entry at the specified index.
    pub fn at_mut(&mut self, index: u32) -> &mut Entry {
        assert!(self.is_initialized());
        unsafe { &mut *self.base_.add(index as usize) }
    }

    /// Returns an iterator that can be used to perform multiple write operations
    /// without switching the write-protections all the time (if kIsWriteProtected
    /// is true).
    pub fn iter_at(&mut self, index: u32) -> WriteIterator<Entry> {
        assert!(self.is_initialized());
        WriteIterator::new(self.base_, index)
    }

    /// Returns true if this table has been initialized.
    pub fn is_initialized(&self) -> bool {
        !self.base_.is_null()
    }

    /// Returns the base address of this table.
    pub fn base(&self) -> *mut Entry {
        self.base_
    }

    /// Allocate a new segment in this table.
    ///
    /// The segment is initialized with freelist entries.
    pub fn allocate_and_initialize_segment(&mut self) -> (Segment, FreelistHead) {
        self.try_allocate_and_initialize_segment()
            .expect("Failed to allocate and initialize segment")
    }

    /// Same as above but fails if there is no space left.
    pub fn try_allocate_and_initialize_segment(&mut self) -> Option<(Segment, FreelistHead)> {
        let segment_number = self.vas_.allocate_segment(Self::K_SEGMENT_SIZE)?;
        let segment = Segment::new(segment_number);
        let freelist_head = self.initialize_free_list(segment, 0);
        Some((segment, freelist_head))
    }

    /// Initialize a table segment with a freelist.
    ///
    /// Note that you don't need to call this function on segments allocated with
    /// `allocate_and_initialize_segment()` since those already get initialized.
    pub fn initialize_free_list(&mut self, segment: Segment, start_offset: u32) -> FreelistHead {
        let first_entry = segment.first_entry() + start_offset;
        let last_entry = segment.last_entry();

        if first_entry > last_entry {
            return FreelistHead::new(0, 0); // Empty freelist if range is invalid
        }

        // Initialize the freelist entries
        for i in first_entry..last_entry {
            let next_index = i + 1;
            self.at_mut(i).make_freelist_entry(next_index);
        }
        // Last entry points to null
        self.at_mut(last_entry).make_freelist_entry(0);

        FreelistHead::new(first_entry, last_entry - first_entry + 1)
    }

    /// Free the specified segment of this table.
    ///
    /// The memory of this segment will afterwards be inaccessible.
    pub fn free_table_segment(&mut self, segment: Segment) {
        self.vas_.free_segment(segment.number(), Self::K_SEGMENT_SIZE);
    }

    /// Initializes the table by reserving the backing memory, allocating an
    /// initial segment, and populating the freelist.
    pub fn initialize(&mut self) {
        self.vas_.reserve(if Self::K_USE_CONTIGUOUS_MEMORY {
            Self::K_RESERVATION_SIZE
        } else {
            0
        });
        self.base_ = self.vas_.base() as *mut Entry;

        if self.try_allocate_and_initialize_segment().is_none() {
            panic!("Failed to allocate initial segment");
        }
    }

    /// Deallocates all memory associated with this table.
    pub fn tear_down(&mut self) {
        self.vas_.release();
        self.base_ = std::ptr::null_mut();
    }
}

struct VirtualAddressSpace {
    base_: *mut u8,
    size_: usize,
}

impl VirtualAddressSpace {
    fn new() -> Self {
        VirtualAddressSpace {
            base_: std::ptr::null_mut(),
            size_: 0,
        }
    }

    fn reserve(&mut self, size: usize) {
        assert!(self.base_.is_null());
        if size > 0 {
            // Simulate reserving memory (in a real implementation, this would
            // map pages in the virtual address space).
            let layout = std::alloc::Layout::from_size_align(size, 1).unwrap();
            self.base_ = unsafe { std::alloc::alloc(layout) };
            if self.base_.is_null() {
                panic!("Failed to reserve virtual address space");
            }
            self.size_ = size;
        }
    }

    fn release(&mut self) {
        if !self.base_.is_null() {
            let layout = std::alloc::Layout::from_size_align(self.size_, 1).unwrap();
            unsafe { std::alloc::dealloc(self.base_, layout) };
            self.base_ = std::ptr::null_mut();
            self.size_ = 0;
        }
    }

    fn allocate_segment(&mut self, size: usize) -> Option<u32> {
        // In a real implementation, this would map a segment of the virtual
        // address space (or reuse a previously freed segment).
        if self.base_.is_null() || size > self.size_ {
            return None;
        }

        // For simplicity, always allocate from the start
        let offset = 0;
        Some(0) // returns segment number
    }

    fn free_segment(&mut self, segment_number: u32, size: usize) {
        // In a real implementation, this would unmap a segment of the virtual
        // address space (or mark it as available for reuse).
        // For now, we do nothing
    }

    fn base(&self) -> *mut u8 {
        self.base_
    }
}

// Dummy implementation for CFIMetadataWriteScope/NopRwxMemoryWriteScope
enum WriteScope<E: FreelistEntry> {
    Protected(CFIMetadataWriteScope),
    Unprotected(NopRwxMemoryWriteScope),
    _Phantom(PhantomData<E>),
}

impl<E: FreelistEntry> WriteScope<E> {
    fn new(base: *mut E) -> Self {
        if E::IS_WRITE_PROTECTED {
            WriteScope::Protected(CFIMetadataWriteScope {})
        } else {
            WriteScope::Unprotected(NopRwxMemoryWriteScope {})
        }
    }
}

struct CFIMetadataWriteScope {} // Dummy
impl Drop for CFIMetadataWriteScope {
    fn drop(&mut self) {}
}

struct NopRwxMemoryWriteScope {} // Dummy
impl Drop for NopRwxMemoryWriteScope {
    fn drop(&mut self) {}
}