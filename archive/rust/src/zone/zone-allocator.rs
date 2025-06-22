// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::{
    alloc::{alloc, dealloc, Layout},
    marker::PhantomData,
    ptr::NonNull,
};

//use crate::base::compiler_specific::unreachable;
//use crate::base::logging::dcheck_imples;
//use crate::common::globals::is_compressed_pointer;
//use crate::zone::zone::Zone;

/// A zone allocator that allocates memory from a given zone.
pub struct ZoneAllocator<'zone, T> {
    zone: &'zone Zone,
    _phantom: PhantomData<T>,
}

impl<'zone, T> ZoneAllocator<'zone, T> {
    /// Creates a new `ZoneAllocator` that allocates memory from the given zone.
    pub fn new(zone: &'zone Zone) -> Self {
        // If we are going to allocate compressed pointers in the zone it must
        // support compression.
        // DCHECK_IMPLIES(is_compressed_pointer<T>::value,
        //                zone_->supports_compression());
        ZoneAllocator {
            zone,
            _phantom: PhantomData,
        }
    }

    /// Allocates a new array of `T` of the given length from the zone.
    pub fn allocate(&self, length: usize) -> NonNull<T> {
        self.zone.allocate_array(length)
    }

    /// Deallocates the given array of `T` of the given length from the zone.
    ///
    /// # Safety
    ///
    /// The pointer `p` must have been allocated by this allocator and the
    /// length must be the same as the length that was used to allocate the
    /// array.
    pub unsafe fn deallocate(&self, p: NonNull<T>, length: usize) {
        self.zone.deallocate_array(p, length)
    }

    /// Returns the zone that this allocator allocates from.
    pub fn zone(&self) -> &Zone {
        self.zone
    }
}

impl<'zone, T> Clone for ZoneAllocator<'zone, T> {
    fn clone(&self) -> Self {
        ZoneAllocator {
            zone: self.zone,
            _phantom: PhantomData,
        }
    }
}

impl<'zone, T> Copy for ZoneAllocator<'zone, T> {}

impl<'zone, T> PartialEq for ZoneAllocator<'zone, T> {
    fn eq(&self, other: &Self) -> bool {
        self.zone as *const _ == other.zone as *const _
    }
}

impl<'zone, T> Eq for ZoneAllocator<'zone, T> {}

/// A recycling zone allocator that maintains a free list of deallocated chunks
/// to reuse on subsequent allocations.
pub struct RecyclingZoneAllocator<'zone, T> {
    zone_allocator: ZoneAllocator<'zone, T>,
    free_list: Option<NonNull<FreeBlock<T>>>,
}

impl<'zone, T> RecyclingZoneAllocator<'zone, T> {
    /// Creates a new `RecyclingZoneAllocator` that allocates memory from the given zone.
    pub fn new(zone: &'zone Zone) -> Self {
        RecyclingZoneAllocator {
            zone_allocator: ZoneAllocator::new(zone),
            free_list: None,
        }
    }

    /// Allocates a new array of `T` of the given length from the zone, reusing
    /// a free block if possible.
    pub fn allocate(&mut self, n: usize) -> NonNull<T> {
        // Only check top block in free list, since this will be equal to or larger
        // than the other blocks in the free list.
        if let Some(mut free_block) = self.free_list {
            let free_block_ref = unsafe { free_block.as_mut() };
            if free_block_ref.size >= n {
                let return_val = free_block.cast::<T>();
                self.free_list = free_block_ref.next;
                return return_val;
            }
        }
        self.zone_allocator.allocate(n)
    }

    /// Deallocates the given array of `T` of the given length from the zone,
    /// adding it to the free list if it is large enough.
    ///
    /// # Safety
    ///
    /// The pointer `p` must have been allocated by this allocator and the
    /// length must be the same as the length that was used to allocate the
    /// array.
    pub unsafe fn deallocate(&mut self, p: NonNull<T>, n: usize) {
        if std::mem::size_of::<T>() * n < std::mem::size_of::<FreeBlock<T>>() {
            return;
        }

        // Only add block to free_list if it is equal or larger than previous block
        // so that allocation stays O(1) only having to look at the top block.
        if self.free_list.is_none()
            || self.free_list.map_or(true, |free_list| {
                let free_list_ref = unsafe { free_list.as_ref() };
                free_list_ref.size <= n
            })
        {
            // Store the free-list within the block being deallocated.
            // DCHECK((sizeof(T) * n >= sizeof(FreeBlock)));
            let new_free_block_ptr = p.cast::<FreeBlock<T>>();
            let mut new_free_block = new_free_block_ptr.as_mut();

            new_free_block.size = n;
            new_free_block.next = self.free_list;
            self.free_list = Some(new_free_block_ptr);
        }
    }

    /// Returns the zone that this allocator allocates from.
    pub fn zone(&self) -> &Zone {
        self.zone_allocator.zone()
    }
}

#[derive(Debug)]
struct FreeBlock<T> {
    next: Option<NonNull<FreeBlock<T>>>,
    size: usize,
}

/// A zone allocator for booleans.
pub type ZoneBoolAllocator<'zone> = ZoneAllocator<'zone, bool>;

/// A zone allocator for integers.
pub type ZoneIntAllocator<'zone> = ZoneAllocator<'zone, i32>;

/// Dummy zone for compilation.  The real `Zone` and its methods are not provided.
#[derive(Debug)]
pub struct Zone {}

impl Zone {
    pub fn new() -> Self {
        Zone {}
    }
    pub fn allocate_array<T>(&self, length: usize) -> NonNull<T> {
        let layout = Layout::array::<T>(length).unwrap();
        unsafe {
            let ptr = alloc(layout) as *mut T;
            NonNull::new(ptr).unwrap()
        }
    }

    pub unsafe fn deallocate_array<T>(&self, ptr: NonNull<T>, length: usize) {
        let layout = Layout::array::<T>(length).unwrap();
        dealloc(ptr.as_ptr() as *mut u8, layout);
    }

    pub fn supports_compression(&self) -> bool {
        true
    }
}