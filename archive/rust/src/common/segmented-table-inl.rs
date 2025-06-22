// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/common/segmented-table-inl.h

use std::mem;
use std::ptr::NonNull;

// Placeholder for emulated virtual address subspace
// Replace with a proper implementation if needed
mod base {
    pub struct EmulatedVirtualAddressSubspace {}
}

mod common {
    pub mod assert_scope {
        // Placeholder for assert scope
        pub struct AssertScope {}
        impl AssertScope {
            pub fn new(_message: &'static str) -> Self {
                AssertScope {}
            }
        }
    }
}

mod utils {
    pub mod allocation {
        // Placeholder functions, replace with appropriate implementations
        pub fn get_platform_virtual_address_space() -> &'static mut crate::VirtualAddressSpace {
            todo!()
        }

        pub enum PagePermissions {
            NoAccess,
            ReadWrite,
        }
    }
}

mod thread_isolation {
    pub mod write_protect_memory {
        use crate::page_allocator::Permission;
        // Placeholder functions, replace with appropriate implementations
        pub fn write_protect_memory(_base: *mut u8, _size: usize, _permission: Permission) -> bool {
            todo!()
        }
    }
}

mod page_allocator {
    pub enum Permission {
        NoAccess
    }
}

// Placeholder V8 functionality
mod v8 {
    pub fn fatal_process_out_of_memory(_location: Option<&'static str>, _message: &'static str) -> ! {
        panic!("Out of memory: {}", _message)
    }
}

// Placeholder virtual address space
pub struct VirtualAddressSpace {}

impl VirtualAddressSpace {
    pub fn base(&self) -> *mut u8 {
        std::ptr::null_mut()
    }

    pub fn allocate_pages(&mut self, _hint: u32, _size: usize, _alignment: usize, _permissions: utils::allocation::PagePermissions) -> *mut u8 {
        std::ptr::null_mut()
    }

     pub fn allocate_subspace(&mut self, _hint: u32, _reservation_size: usize, _segment_size: usize, _permissions: utils::allocation::PagePermissions) -> Result<Box<VirtualAddressSpace>, ()>{
         Err(())
     }

     pub fn free_pages(&mut self, _address: *mut u8, _size: usize) {}

     pub fn can_allocate_subspaces(&self) -> bool{
        false
     }

     pub const K_NO_HINT: u32 = 0;
}

pub const K_USE_CONTIGUOUS_MEMORY: bool = cfg!(target_pointer_width = "64");
pub const K_IS_WRITE_PROTECTED: bool = false;

#[macro_export]
macro_rules! static_assert {
    ($cond:expr) => {
        const _: [(); 0 - (!($cond) as usize)] = [];
    };
}

/// A table divided into segments for better memory management.
pub struct SegmentedTable<Entry, const SIZE: usize> {
    base_: *mut Entry,
    vas_: *mut VirtualAddressSpace, // Option<Box<VirtualAddressSpace>>
}

impl<Entry, const SIZE: usize> SegmentedTable<Entry, const SIZE> {
    const K_SEGMENT_SIZE: usize = 4096;
    const K_ENTRIES_PER_SEGMENT: usize = Self::K_SEGMENT_SIZE / mem::size_of::<Entry>();
    const K_RESERVATION_SIZE: usize = 16 * 1024 * 1024;

    /// Represents a segment within the segmented table.
    #[derive(Clone, Copy)]
    pub struct Segment {
        number: u32,
    }

    impl Segment {
        /// Creates a Segment from an offset.
        pub fn at(offset: u32) -> Self {
            assert!(offset % Self::k_segment_size() as u32 == 0);
            let number = offset / Self::k_segment_size() as u32;
            Segment { number }
        }

        /// Creates a Segment that contains a given entry index.
        pub fn containing(entry_index: u32) -> Self {
            let number = entry_index / Self::k_entries_per_segment() as u32;
            Segment { number }
        }

        /// Returns the offset of the segment in bytes
        pub fn offset(&self) -> usize {
            (self.number as usize) * Self::k_segment_size()
        }

        /// Returns the first entry index of the segment.
        pub fn first_entry(&self) -> u32 {
            self.number * Self::k_entries_per_segment() as u32
        }

        /// Returns the last entry index of the segment.
        pub fn last_entry(&self) -> u32 {
            self.first_entry() + Self::k_entries_per_segment() as u32 - 1
        }

        const fn k_segment_size() -> usize {
             SegmentedTable::<Entry, SIZE>::K_SEGMENT_SIZE
        }

        const fn k_entries_per_segment() -> usize {
            SegmentedTable::<Entry, SIZE>::K_ENTRIES_PER_SEGMENT
        }
    }

    /// An iterator for writing to the segmented table.
    pub struct WriteIterator<'a> {
        base_: *mut Entry,
        index_: u32,
        write_scope_: common::assert_scope::AssertScope,
        _phantom: std::marker::PhantomData<&'a mut Entry>,
    }

    impl<'a> WriteIterator<'a> {
        /// Creates a new WriteIterator.
        pub fn new(base: *mut Entry, index: u32) -> Self {
            WriteIterator {
                base_: base,
                index_: index,
                write_scope_: common::assert_scope::AssertScope::new("pointer table write"),
                _phantom: std::marker::PhantomData,
            }
        }

        /// Returns the current index of the iterator.
        pub fn index(&self) -> u32 {
            self.index_
        }
    }

    impl<'a> std::ops::Deref for WriteIterator<'a> {
        type Target = Entry;

        fn deref(&self) -> &Self::Target {
            unsafe { &*self.base_.add(self.index_ as usize) }
        }
    }

    impl<'a> std::ops::DerefMut for WriteIterator<'a> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { &mut *self.base_.add(self.index_ as usize) }
        }
    }

    impl<'a> WriteIterator<'a> {
        fn increment(&mut self) {
            self.index_ += 1;
        }
    }

    /// Head of a free list.
    #[derive(Clone, Copy)]
    pub struct FreelistHead {
        index: u32,
        count: u32,
    }

    impl FreelistHead {
        /// Creates a new FreelistHead.
        pub fn new(index: u32, count: u32) -> Self {
            FreelistHead { index, count }
        }
    }

    /// Returns a reference to the entry at the given index.
    pub fn at(&self, index: u32) -> &Entry {
        unsafe { &*self.base_.add(index as usize) }
    }

    /// Returns a mutable reference to the entry at the given index.
    pub fn at_mut(&mut self, index: u32) -> &mut Entry {
        unsafe { &mut *self.base_.add(index as usize) }
    }

    /// Creates a WriteIterator at the given index.
    pub fn iter_at(&mut self, index: u32) -> WriteIterator {
        WriteIterator::new(self.base_, index)
    }

    /// Checks if the table is initialized.
    pub fn is_initialized(&self) -> bool {
        //DCHECK(!base_ || reinterpret_cast<Address>(base_) == vas_->base());
        if self.base_.is_null() {
            self.vas_.is_null()
        } else {
            unsafe { (*self.vas_).base() == self.base_ as *mut u8 }
        }
    }

    /// Returns the base address of the table.
    pub fn base(&self) -> *mut u8 {
        assert!(self.is_initialized());
        self.base_ as *mut u8
    }

    /// Initializes the table.
    pub fn initialize(&mut self) {
        assert!(!self.is_initialized());

        let root_space = utils::allocation::get_platform_virtual_address_space();

        if cfg!(target_pointer_width = "64") {
            //static_assert!(K_USE_CONTIGUOUS_MEMORY);
            assert!(Self::K_RESERVATION_SIZE % root_space.allocation_granularity() == 0);

            // Mimic unique_ptr::release, since we are transfering ownership
            if root_space.can_allocate_subspaces() {
                match root_space.allocate_subspace(VirtualAddressSpace::K_NO_HINT, Self::K_RESERVATION_SIZE, Self::K_SEGMENT_SIZE, utils::allocation::PagePermissions::ReadWrite) {
                    Ok(subspace) => {
                        self.vas_ = Box::into_raw(subspace);
                    }
                    Err(_e) => {
                        self.vas_ = std::ptr::null_mut();
                    }
                }
            } else {
                // This may be required on old Windows versions that don't support
                // VirtualAlloc2, which is required for subspaces. In that case, just use a
                // fully-backed emulated subspace.
                let reservation_base = root_space.allocate_pages(VirtualAddressSpace::K_NO_HINT, Self::K_RESERVATION_SIZE, Self::K_SEGMENT_SIZE, utils::allocation::PagePermissions::NoAccess);
                if !reservation_base.is_null() {
                    //self.vas_ = Box::new(base::EmulatedVirtualAddressSubspace{}); // TODO
                    // Currently, we cannot fully convert this part as it requires implementing `EmulatedVirtualAddressSubspace`
                    // We will leave it as a placeholder and handle this conversion later
                    todo!();
                }
            }

            if self.vas_.is_null() {
                v8::fatal_process_out_of_memory(None, "SegmentedTable::InitializeTable (subspace allocation)");
            }
        } else {
            //static_assert!(!K_USE_CONTIGUOUS_MEMORY);
            self.vas_ = root_space;
        }

        self.base_ = unsafe { (*self.vas_).base() as *mut Entry };

        if K_USE_CONTIGUOUS_MEMORY && K_IS_WRITE_PROTECTED {
            assert!(thread_isolation::write_protect_memory::write_protect_memory(
                self.base() as *mut u8,
                SIZE,
                page_allocator::Permission::NoAccess
            ));
        }
    }

    /// Tears down the table, freeing allocated memory.
    pub fn tear_down(&mut self) {
        assert!(self.is_initialized());

        self.base_ = std::ptr::null_mut();

        if cfg!(target_pointer_width = "64") {
            unsafe {
                drop(Box::from_raw(self.vas_));
            }
        }

        self.vas_ = std::ptr::null_mut();
    }

    /// Initializes the free list for a given segment.
    pub fn initialize_free_list(&mut self, segment: Segment, start_offset: u32) -> FreelistHead {
        assert!(start_offset < Self::K_ENTRIES_PER_SEGMENT as u32);
        let num_entries = Self::K_ENTRIES_PER_SEGMENT as u32 - start_offset;

        let first = segment.first_entry() + start_offset;
        let last = segment.last_entry();
        {
            let mut it = self.iter_at(first);
            while it.index() != last {
                // TODO: implement MakeFreelistEntry
                //it.MakeFreelistEntry(it.index() + 1);
                it.increment();
            }
            // TODO: implement MakeFreelistEntry
            //it.MakeFreelistEntry(0);
        }

        FreelistHead::new(first, num_entries)
    }

    /// Allocates and initializes a new segment.
    pub fn allocate_and_initialize_segment(&mut self) -> (Segment, FreelistHead) {
        match self.try_allocate_and_initialize_segment() {
            Some(res) => res,
            None => v8::fatal_process_out_of_memory(None, "SegmentedTable::AllocateAndInitializeSegment"),
        }
    }

    /// Tries to allocate and initialize a new segment.
    pub fn try_allocate_and_initialize_segment(&mut self) -> Option<(Segment, FreelistHead)> {
        let start = unsafe {
            (*self.vas_).allocate_pages(VirtualAddressSpace::K_NO_HINT, Self::K_SEGMENT_SIZE, Self::K_SEGMENT_SIZE, utils::allocation::PagePermissions::ReadWrite)
        };
        if start.is_null() {
            return None;
        }

        let offset = unsafe { (start as usize - (*self.vas_).base() as usize) as u32 };
        let segment = Segment::at(offset);

        let freelist = self.initialize_free_list(segment, 0);

        Some((segment, freelist))
    }

    /// Frees a table segment.
    pub fn free_table_segment(&mut self, segment: Segment) {
        let segment_start = unsafe { (*self.vas_).base().add(segment.offset()) };
        unsafe { (*self.vas_).free_pages(segment_start, Self::K_SEGMENT_SIZE) };
    }
}