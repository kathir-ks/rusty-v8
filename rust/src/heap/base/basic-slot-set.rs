// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::sync::atomic::{AtomicPtr, AtomicU32, Ordering};
use std::{mem, ptr};

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum SlotCallbackResult {
    KEEP_SLOT,
    REMOVE_SLOT,
}

pub mod atomic_utils {
    use std::sync::atomic::{AtomicU32, Ordering};

    pub struct AsAtomic32 {}

    impl AsAtomic32 {
        #[inline]
        pub fn acquire_load(atomic: &AtomicU32) -> u32 {
            atomic.load(Ordering::Acquire)
        }

        #[inline]
        pub fn release_store(atomic: &AtomicU32, value: u32) {
            atomic.store(value, Ordering::Release);
        }

        #[inline]
        pub fn release_set_bits(atomic: &AtomicU32, value: u32, mask: u32) {
            let mut current = atomic.load(Ordering::Acquire);
            loop {
                let new_value = (current & !mask) | value;
                match atomic.compare_exchange_weak(
                    current,
                    new_value,
                    Ordering::Release,
                    Ordering::Acquire,
                ) {
                    Ok(_) => break,
                    Err(x) => current = x,
                }
            }
        }
    }

    pub struct AsAtomicPointer {}

    impl AsAtomicPointer {
        #[inline]
        pub fn acquire_load<T>(atomic: &AtomicPtr<T>) -> *mut T {
            atomic.load(Ordering::Acquire)
        }

        #[inline]
        pub fn release_store<T>(atomic: &AtomicPtr<T>, value: *mut T) {
            atomic.store(value, Ordering::Release);
        }

        #[inline]
        pub fn release_compare_and_swap<T>(
            atomic: &AtomicPtr<T>,
            current: *mut T,
            new: *mut T,
        ) -> *mut T {
            match atomic.compare_exchange(current, new, Ordering::Release, Ordering::Relaxed) {
                Ok(_) => current,
                Err(x) => x,
            }
        }
    }
}

pub mod bits {
    pub fn count_trailing_zeros(x: u32) -> usize {
        x.trailing_zeros() as usize
    }
}

pub mod platform {
    pub mod memory {
        use std::alloc::{alloc, dealloc, Layout};
        use std::ptr;

        pub unsafe fn aligned_alloc(size: usize, alignment: usize) -> *mut u8 {
            let layout = Layout::from_size_align(size, alignment).unwrap();
            if layout.size() == 0 {
                return alignment as *mut u8;
            }
            let ptr = alloc(layout);
            if ptr.is_null() {
                panic!("Allocation failed");
            }
            ptr
        }

        pub unsafe fn aligned_free(ptr: *mut u8) {
            if ptr.is_null() {
                return;
            }

            //This is a dummy aligned_free since we don't have the layout information
            //from the allocation.  This is ONLY safe because the allocator is never used
            //in the current code.

            //TODO: Properly implement aligned_free.
        }
    }
}

pub mod base {
    use super::*;
    use std::sync::atomic::AtomicPtr;

    #[derive(Debug)]
    pub struct AlignedAllocError;

    pub unsafe fn aligned_alloc(size: usize, alignment: usize) -> Result<*mut u8, AlignedAllocError> {
        let ptr = platform::memory::aligned_alloc(size, alignment);
        if ptr.is_null() {
            return Err(AlignedAllocError);
        }
        Ok(ptr)
    }

    pub unsafe fn aligned_free(ptr: *mut u8) {
        platform::memory::aligned_free(ptr);
    }
}

pub mod internal {
    pub struct WriteBarrierCodeStubAssembler {}
}

pub mod heap_base {
    use super::*;
    use std::sync::atomic::AtomicPtr;

    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum EmptyBucketMode {
        FREE_EMPTY_BUCKETS,
        KEEP_EMPTY_BUCKETS,
    }

    pub struct BasicSlotSet<const SLOT_GRANULARITY: usize> {
        num_buckets: usize,
        buckets: Vec<*mut Bucket>,
    }

    impl<const SLOT_GRANULARITY: usize> BasicSlotSet<SLOT_GRANULARITY> {
        const K_SYSTEM_POINTER_SIZE: usize = mem::size_of::<usize>();
        const K_CELLS_PER_BUCKET: usize = 32;
        const K_CELLS_PER_BUCKET_LOG2: usize = 5;
        const K_CELL_SIZE_BYTES_LOG2: usize = 2;
        const K_CELL_SIZE_BYTES: usize = 1 << Self::K_CELL_SIZE_BYTES_LOG2;
        const K_BITS_PER_CELL: usize = 32;
        const K_BITS_PER_CELL_LOG2: usize = 5;
        const K_BITS_PER_BUCKET: usize = Self::K_CELLS_PER_BUCKET * Self::K_BITS_PER_CELL;
        const K_BITS_PER_BUCKET_LOG2: usize =
            Self::K_CELLS_PER_BUCKET_LOG2 + Self::K_BITS_PER_CELL_LOG2;
        const K_NUM_BUCKETS_SIZE: usize = mem::size_of::<usize>();

        pub fn allocate(buckets: usize) -> Box<Self> {
            let mut v = Vec::with_capacity(buckets);
            v.resize_with(buckets, || std::ptr::null_mut());

            Box::new(Self {
                num_buckets: buckets,
                buckets: v,
            })
        }

        pub fn delete(slot_set: Box<Self>) {
            //Drop will handle this for now.
        }

        pub const fn buckets_for_size(size: usize) -> usize {
            (size + (SLOT_GRANULARITY * Self::K_BITS_PER_BUCKET) - 1)
                / (SLOT_GRANULARITY * Self::K_BITS_PER_BUCKET)
        }

        pub const fn bucket_for_slot(slot_offset: usize) -> usize {
            assert!(Self::is_aligned(slot_offset as u64, SLOT_GRANULARITY as u64));
            slot_offset / (SLOT_GRANULARITY * Self::K_BITS_PER_BUCKET)
        }

        pub const fn offset_for_bucket(bucket_index: usize) -> usize {
            bucket_index * SLOT_GRANULARITY * Self::K_BITS_PER_BUCKET
        }

        fn is_aligned(value: u64, alignment: u64) -> bool {
            value & (alignment - 1) == 0
        }

        pub fn insert<const ACCESS_MODE: u8>(&mut self, slot_offset: usize) {
            let (bucket_index, cell_index, bit_index) = self.slot_to_indices(slot_offset);

            let bucket_ptr = self.load_bucket::<ACCESS_MODE>(bucket_index);
            let mut bucket: Box<Bucket>;

            if bucket_ptr.is_null() {
                bucket = Box::new(Bucket::default());
                let bucket_raw = Box::into_raw(bucket);

                unsafe {
                    if !self.swap_in_new_bucket::<ACCESS_MODE>(bucket_index, bucket_raw) {
                        drop(Box::from_raw(bucket_raw)); //Drop the bucket
                    }
                }

                bucket_ptr = self.load_bucket::<ACCESS_MODE>(bucket_index);
            }

            assert!(!bucket_ptr.is_null());

            let bucket = unsafe {&mut *bucket_ptr};

            let mask = 1u32 << bit_index;
            if (bucket.load_cell::<ACCESS_MODE>(cell_index) & mask) == 0 {
                bucket.set_cell_bits::<ACCESS_MODE>(cell_index, mask);
            }
        }

        pub fn contains(&self, slot_offset: usize) -> bool {
            let (bucket_index, cell_index, bit_index) = self.slot_to_indices(slot_offset);
            let bucket = self.load_bucket::<0>(bucket_index);
            if bucket.is_null() {
                return false;
            }
            let bucket = unsafe { &*bucket };
            (bucket.load_cell::<0>(cell_index) & (1u32 << bit_index)) != 0
        }

        pub fn remove(&mut self, slot_offset: usize) {
            let (bucket_index, cell_index, bit_index) = self.slot_to_indices(slot_offset);
            let bucket = self.load_bucket::<0>(bucket_index);
            if !bucket.is_null() {
                let bucket = unsafe { &mut *bucket };
                let cell = bucket.load_cell::<0>(cell_index);
                let bit_mask = 1u32 << bit_index;
                if cell & bit_mask != 0 {
                    bucket.clear_cell_bits::<0>(cell_index, bit_mask);
                }
            }
        }

        pub fn remove_range(
            &mut self,
            start_offset: usize,
            end_offset: usize,
            buckets: usize,
            mode: EmptyBucketMode,
        ) {
            assert!(end_offset <= buckets * Self::K_BITS_PER_BUCKET * SLOT_GRANULARITY);
            assert!(start_offset <= end_offset);

            let (mut start_bucket, mut start_cell, mut start_bit) = self.slot_to_indices(start_offset);
            let (end_bucket, end_cell, mut end_bit) =
                self.slot_to_indices(end_offset - SLOT_GRANULARITY);

            end_bit += 1;
            if end_bit >= Self::K_BITS_PER_CELL {
                end_bit = 0;
                start_cell += 1;
                if start_cell >= Self::K_CELLS_PER_BUCKET {
                    start_cell = 0;
                    start_bucket += 1;
                }
            }

            let start_mask = (1u32 << start_bit) - 1;
            let end_mask = !((1u32 << end_bit) - 1);
            let mut bucket: *mut Bucket;

            if start_bucket == end_bucket && start_cell == end_cell {
                bucket = self.load_bucket::<0>(start_bucket);
                if !bucket.is_null() {
                  let bucket = unsafe {&mut *bucket};
                    bucket.clear_cell_bits::<0>(start_cell, !(start_mask | end_mask));
                }
                return;
            }

            let mut current_bucket = start_bucket;
            let mut current_cell = start_cell;

            bucket = self.load_bucket::<0>(current_bucket);
            if !bucket.is_null() {
              let bucket = unsafe {&mut *bucket};
              bucket.clear_cell_bits::<0>(current_cell, !start_mask);
            }

            current_cell += 1;

            if current_bucket < end_bucket {
                if !bucket.is_null() {
                  let bucket = unsafe {&mut *bucket};
                    self.clear_bucket(bucket, current_cell, Self::K_CELLS_PER_BUCKET);
                }
                current_bucket += 1;
                current_cell = 0;
            }

            assert!(current_bucket == end_bucket || (current_bucket < end_bucket && current_cell == 0));

            while current_bucket < end_bucket {
                if mode == EmptyBucketMode::FREE_EMPTY_BUCKETS {
                    self.release_bucket::<0>(current_bucket);
                } else {
                    assert!(mode == EmptyBucketMode::KEEP_EMPTY_BUCKETS);
                    bucket = self.load_bucket::<0>(current_bucket);
                    if !bucket.is_null() {
                      let bucket = unsafe {&mut *bucket};
                        self.clear_bucket(bucket, 0, Self::K_CELLS_PER_BUCKET);
                    }
                }
                current_bucket += 1;
            }

            assert!(current_bucket == end_bucket);
            if current_bucket == buckets {
                return;
            }

            bucket = self.load_bucket::<0>(current_bucket);
            assert!(current_cell <= end_cell);
            if bucket.is_null() {
                return;
            }

            let bucket = unsafe {&mut *bucket};

            while current_cell < end_cell {
                bucket.store_cell(current_cell, 0);
                current_cell += 1;
            }

            assert!(current_bucket == end_bucket && current_cell == end_cell);
            bucket.clear_cell_bits::<0>(end_cell, !end_mask);
        }

        pub fn lookup(&self, slot_offset: usize) -> bool {
            let (bucket_index, cell_index, bit_index) = self.slot_to_indices(slot_offset);
            let bucket = self.load_bucket::<0>(bucket_index);
            if bucket.is_null() {
                return false;
            }
            let bucket = unsafe {&*bucket};
            (bucket.load_cell::<0>(cell_index) & (1u32 << bit_index)) != 0
        }

        pub fn iterate<const ACCESS_MODE: u8, F>(
            &mut self,
            chunk_start: usize,
            start_bucket: usize,
            end_bucket: usize,
            callback: F,
            mode: EmptyBucketMode,
        ) -> usize
        where
            F: Fn(usize) -> SlotCallbackResult,
        {
            let empty_bucket_callback = |bucket_index: usize| {
                if mode == EmptyBucketMode::FREE_EMPTY_BUCKETS {
                    self.release_bucket::<0>(bucket_index);
                }
            };
            self.iterate_internal::<ACCESS_MODE, F, _>(
                chunk_start,
                start_bucket,
                end_bucket,
                callback,
                empty_bucket_callback,
            )
        }

        fn iterate_internal<const ACCESS_MODE: u8, F, E>(
            &mut self,
            chunk_start: usize,
            start_bucket: usize,
            end_bucket: usize,
            callback: F,
            empty_bucket_callback: E,
        ) -> usize
        where
            F: Fn(usize) -> SlotCallbackResult,
            E: Fn(usize),
        {
            let mut new_count = 0;
            for bucket_index in start_bucket..end_bucket {
                let bucket_ptr = self.load_bucket::<ACCESS_MODE>(bucket_index);
                if !bucket_ptr.is_null() {
                  let bucket = unsafe {&mut *bucket_ptr};
                    let mut in_bucket_count = 0;
                    let cell_offset = bucket_index << Self::K_BITS_PER_BUCKET_LOG2;
                    for i in 0..Self::K_CELLS_PER_BUCKET {
                        let cell_offset_local = cell_offset + (i << Self::K_BITS_PER_CELL_LOG2);
                        let cell = bucket.load_cell::<ACCESS_MODE>(i);
                        if cell != 0 {
                            let mut old_cell = cell;
                            let mut mask = 0;
                            let mut cell_local = cell;
                            while cell_local != 0 {
                                let bit_offset = bits::count_trailing_zeros(cell_local);
                                let bit_mask = 1u32 << bit_offset;
                                let slot = (cell_offset_local + bit_offset) * SLOT_GRANULARITY;
                                if callback(chunk_start + slot) == SlotCallbackResult::KEEP_SLOT {
                                    in_bucket_count += 1;
                                } else {
                                    mask |= bit_mask;
                                }
                                cell_local ^= bit_mask;
                            }
                            let new_cell = old_cell & !mask;
                            if old_cell != new_cell {
                                bucket.clear_cell_bits::<ACCESS_MODE>(i, mask);
                            }
                        }
                    }
                    if in_bucket_count == 0 {
                        empty_bucket_callback(bucket_index);
                    }
                    new_count += in_bucket_count;
                }
            }
            new_count
        }

        fn free_bucket_if_empty(&mut self, bucket_index: usize) -> bool {
            let bucket = self.load_bucket::<0>(bucket_index);
            if !bucket.is_null() {
                let bucket = unsafe {&*bucket};
                if bucket.is_empty() {
                    self.release_bucket::<0>(bucket_index);
                } else {
                    return false;
                }
            }

            true
        }

        fn clear_bucket(&mut self, bucket: &mut Bucket, start_cell: usize, end_cell: usize) {
            assert!(start_cell >= 0);
            assert!(end_cell <= Self::K_CELLS_PER_BUCKET);
            let mut current_cell = start_cell;
            while current_cell < Self::K_CELLS_PER_BUCKET {
                bucket.store_cell(current_cell, 0);
                current_cell += 1;
            }
        }

        fn release_bucket<const ACCESS_MODE: u8>(&mut self, bucket_index: usize) {
            let bucket_ptr = self.load_bucket::<ACCESS_MODE>(bucket_index);
            self.store_bucket::<ACCESS_MODE>(bucket_index, std::ptr::null_mut());
            if !bucket_ptr.is_null() {
                unsafe {
                    drop(Box::from_raw(bucket_ptr));
                }
            }
        }

        fn load_bucket<const ACCESS_MODE: u8>(&self, bucket_index: usize) -> *mut Bucket {
                self.buckets[bucket_index]
        }

        fn store_bucket<const ACCESS_MODE: u8>(&mut self, bucket_index: usize, value: *mut Bucket) {
                self.buckets[bucket_index] = value;
        }

        unsafe fn swap_in_new_bucket<const ACCESS_MODE: u8>(
            &mut self,
            bucket_index: usize,
            value: *mut Bucket,
        ) -> bool {
            if self.buckets[bucket_index].is_null() {
                self.buckets[bucket_index] = value;
                true
            } else {
                false
            }
        }

        fn slot_to_indices(&self, slot_offset: usize) -> (usize, usize, usize) {
            assert!(Self::is_aligned(slot_offset as u64, SLOT_GRANULARITY as u64));
            let slot = slot_offset / SLOT_GRANULARITY;
            let bucket_index = slot >> Self::K_BITS_PER_BUCKET_LOG2;
            assert!(bucket_index < self.num_buckets);
            let cell_index =
                ((slot >> Self::K_BITS_PER_CELL_LOG2) & (Self::K_CELLS_PER_BUCKET - 1)) as usize;
            let bit_index = (slot & (Self::K_BITS_PER_CELL - 1)) as usize;
            (bucket_index, cell_index, bit_index)
        }
    }

    impl<const SLOT_GRANULARITY: usize> Drop for BasicSlotSet<SLOT_GRANULARITY> {
        fn drop(&mut self) {
            for i in 0..self.num_buckets {
                self.release_bucket::<0>(i);
            }
        }
    }

    #[derive(Default)]
    pub struct Bucket {
        cells: [AtomicU32; BasicSlotSet::<1>::K_CELLS_PER_BUCKET],
    }

    impl Bucket {
        pub fn cells(&mut self) -> &mut [AtomicU32; BasicSlotSet::<1>::K_CELLS_PER_BUCKET] {
            &mut self.cells
        }

        pub fn cell(&mut self, cell_index: usize) -> &mut AtomicU32 {
            &mut self.cells[cell_index]
        }

        pub fn load_cell<const ACCESS_MODE: u8>(&self, cell_index: usize) -> u32 {
            assert!(cell_index < BasicSlotSet::<1>::K_CELLS_PER_BUCKET);
                atomic_utils::AsAtomic32::acquire_load(&self.cells[cell_index])
        }

        pub fn set_cell_bits<const ACCESS_MODE: u8>(&mut self, cell_index: usize, mask: u32) {
                atomic_utils::AsAtomic32::release_set_bits(&self.cells[cell_index], mask, mask);
        }

        pub fn clear_cell_bits<const ACCESS_MODE: u8>(&mut self, cell_index: usize, mask: u32) {
                atomic_utils::AsAtomic32::release_set_bits(&self.cells[cell_index], 0u32, mask);
        }

        pub fn store_cell(&mut self, cell_index: usize, value: u32) {
            atomic_utils::AsAtomic32::release_store(&self.cells[cell_index], value);
        }

        pub fn is_empty(&self) -> bool {
            for i in 0..BasicSlotSet::<1>::K_CELLS_PER_BUCKET {
                if self.cells[i].load(Ordering::Relaxed) != 0 {
                    return false;
                }
            }
            true
        }
    }
}