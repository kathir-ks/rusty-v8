// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod free_list {
    use std::sync::atomic::{AtomicUsize, Ordering};

    //use crate::base::macros::*; // Assuming macros are not crucial for the basic structure
    //use crate::common::globals::*; // Assuming globals are handled elsewhere or replaced with constants
    //use crate::heap::allocation_result::*; // Assuming AllocationResult is handled elsewhere
    //use crate::heap::mutable_page_metadata::*; // Assuming MutablePageMetadata is handled elsewhere
    //use crate::objects::free_space::*; // Assuming FreeSpace and WritableFreeSpace are defined elsewhere
    //use crate::objects::map::*; // Assuming Map is defined elsewhere
    //use crate::utils::utils::*; // Assuming utils are not crucial for the basic structure
    //use testing::gtest::include::gtest_prod::*;  // nogncheck - Ignoring testing-related imports

    // Placeholder types - replace with actual definitions if available
    pub type Tagged<T> = *mut T; // Replace with a proper tagged pointer type if needed
    pub type Heap = (); // Replace with a proper Heap type if needed
    pub type PageMetadata = (); // Replace with a proper PageMetadata type if needed
    pub type AllocationOrigin = (); // Replace with a proper AllocationOrigin type if needed

    pub type FreeListCategoryType = i32;

    pub const K_FIRST_CATEGORY: FreeListCategoryType = 0;
    pub const K_INVALID_CATEGORY: FreeListCategoryType = -1;

    pub enum FreeMode {
        LinkCategory,
        DoNotLinkCategory,
    }

    pub struct FreeListCategory {
        type_: FreeListCategoryType,
        available_: u32,
        top_: Tagged<FreeSpace>,
        prev_: *mut FreeListCategory,
        next_: *mut FreeListCategory,
    }

    impl FreeListCategory {
        pub fn initialize(&mut self, type_: FreeListCategoryType) {
            self.type_ = type_;
            self.available_ = 0;
            self.prev_ = std::ptr::null_mut();
            self.next_ = std::ptr::null_mut();
        }

        // Unlinks the category from the freelist.
        pub fn unlink(&mut self, owner: &mut FreeList) {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }

        // Resets all the fields of the category.
        pub fn reset(&mut self, owner: &mut FreeList) {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }

        pub fn repair_free_list(&mut self, heap: &mut Heap) {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }

        // Relinks the category into the currently owning free list. Requires that the
        // category is currently unlinked.
        pub fn relink(&mut self, owner: &mut FreeList) {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }

        pub fn free(&mut self, writable_free_space: &WritableFreeSpace, mode: FreeMode, owner: &mut FreeList) {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }

        // Performs a single try to pick a node of at least |minimum_size| from the
        // category. Stores the actual size in |node_size|. Returns nullptr if no
        // node is found.
        pub fn pick_node_from_list(
            &mut self,
            minimum_size: usize,
            node_size: &mut usize,
        ) -> Tagged<FreeSpace> {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }

        // Picks a node of at least |minimum_size| from the category. Stores the
        // actual size in |node_size|. Returns nullptr if no node is found.
        pub fn search_for_node_in_list(
            &mut self,
            minimum_size: usize,
            node_size: &mut usize,
        ) -> Tagged<FreeSpace> {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }

        #[inline]
        pub fn is_linked(&self, owner: &FreeList) -> bool {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }

        pub fn is_empty(&self) -> bool {
            self.top().is_null()
        }

        pub fn available(&self) -> u32 {
            self.available_
        }

        pub fn sum_free_list(&self) -> usize {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }

        pub fn free_list_length(&self) -> i32 {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }

        pub fn iterate_nodes_for_testing<F>(&self, callback: F)
        where
            F: Fn(Tagged<FreeSpace>),
        {
            let mut cur_node = self.top();
            while !cur_node.is_null() {
                // Safe as long as FreeSpace has the next field.
                let next_node: Tagged<FreeSpace> = unsafe { (*cur_node).next() };
                callback(cur_node);
                cur_node = next_node;
            }
        }

        fn top(&self) -> Tagged<FreeSpace> {
            self.top_
        }
        fn set_top(&mut self, top: Tagged<FreeSpace>) {
            self.top_ = top;
        }
        fn prev(&self) -> *mut FreeListCategory {
            self.prev_
        }
        fn set_prev(&mut self, prev: *mut FreeListCategory) {
            self.prev_ = prev;
        }
        fn next(&self) -> *mut FreeListCategory {
            self.next_
        }
        fn set_next(&mut self, next: *mut FreeListCategory) {
            self.next_ = next;
        }
    }

    // Placeholder types - replace with actual definitions if available
    pub struct WritableFreeSpace {}

    pub trait FreeSpace {
        fn next(&self) -> Tagged<FreeSpace>;
    }

    // A free list maintains free blocks of memory. The free list is organized in
    // a way to encourage objects allocated around the same time to be near each
    // other. The normal way to allocate is intended to be by bumping a 'top'
    // pointer until it hits a 'limit' pointer.  When the limit is hit we need to
    // find a new space to allocate from. This is done with the free list, which is
    // divided up into rough categories to cut down on waste. Having finer
    // categories would scatter allocation more.
    pub struct FreeList {
        number_of_categories_: i32,
        last_category_: FreeListCategoryType,
        min_block_size_: usize,
        categories_: Vec<*mut FreeListCategory>,
        available_: usize,
        wasted_bytes_: AtomicUsize,
    }

    impl FreeList {
        // Creates a Freelist of the default class.
        pub fn create_free_list() -> Box<FreeList> {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }
        // Creates a Freelist for new space.
        pub fn create_free_list_for_new_space() -> Box<FreeList> {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }

        pub fn new(number_of_categories: i32, min_block_size: usize) -> Self {
            FreeList {
                number_of_categories_: number_of_categories,
                last_category_: 0,
                min_block_size_: min_block_size,
                categories_: vec![std::ptr::null_mut(); number_of_categories as usize],
                available_: 0,
                wasted_bytes_: AtomicUsize::new(0),
            }
        }

        // Adds a node on the free list. The block of size {size_in_bytes} starting
        // at {start} is placed on the free list. The return value is the number of
        // bytes that were not added to the free list, because the freed memory block
        // was too small. Bookkeeping information will be written to the block, i.e.,
        // its contents will be destroyed. The start address should be word aligned,
        // and the size should be a non-zero multiple of the word size.
        pub fn free(&mut self, free_space: &WritableFreeSpace, mode: FreeMode) -> usize {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }

        // Allocates a free space node from the free list of at least size_in_bytes
        // bytes. Returns the actual node size in node_size which can be bigger than
        // size_in_bytes. This method returns null if the allocation request cannot be
        // handled by the free list.
        pub fn allocate(
            &mut self,
            size_in_bytes: usize,
            node_size: &mut usize,
            origin: AllocationOrigin,
        ) -> Tagged<FreeSpace> {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }

        // Returns a page containing an entry for a given type, or nullptr otherwise.
        pub fn get_page_for_size(&self, size_in_bytes: usize) -> *mut PageMetadata {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }

        pub fn reset(&mut self) {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }
        pub fn reset_for_non_black_allocated_pages(&mut self) {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }

        // Return the number of bytes available on the free list.
        pub fn available(&self) -> usize {
            self.verify_available();
            self.available_
        }

        // Update number of available  bytes on the Freelists.
        pub fn increase_available_bytes(&mut self, bytes: usize) {
            self.available_ += bytes;
        }
        pub fn decrease_available_bytes(&mut self, bytes: usize) {
            self.available_ -= bytes;
        }

        pub fn wasted_bytes(&self) -> usize {
            self.wasted_bytes_.load(Ordering::Relaxed)
        }
        pub fn increase_wasted_bytes(&self, bytes: usize) {
            self.wasted_bytes_.fetch_add(bytes, Ordering::Relaxed);
        }
        pub fn decrease_wasted_bytes(&self, bytes: usize) {
            self.wasted_bytes_.fetch_sub(bytes, Ordering::Relaxed);
        }

        pub fn is_empty(&self) -> bool {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }

        // Used after booting the VM.
        pub fn repair_lists(&mut self, heap: &mut Heap) {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }

        pub fn evict_free_list_items(&mut self, page: *mut PageMetadata) {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }

        pub fn number_of_categories(&self) -> i32 {
            self.number_of_categories_
        }
        pub fn last_category(&self) -> FreeListCategoryType {
            self.last_category_
        }

        pub fn min_block_size(&self) -> usize {
            self.min_block_size_
        }

        pub fn for_all_free_list_categories<F>(&self, type_: FreeListCategoryType, callback: F)
        where
            F: Fn(*mut FreeListCategory),
        {
            let mut current = self.categories_[type_ as usize];
            while !current.is_null() {
                let next: *mut FreeListCategory = unsafe { (*current).next() };
                callback(current);
                current = next;
            }
        }

        pub fn for_all_free_list_categories_all<F>(&self, callback: F)
        where
            F: Fn(*mut FreeListCategory),
        {
            for i in K_FIRST_CATEGORY..self.number_of_categories() {
                self.for_all_free_list_categories(i as FreeListCategoryType, &callback);
            }
        }

        pub fn add_category(&mut self, category: *mut FreeListCategory) -> bool {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }
        pub fn remove_category(&mut self, category: *mut FreeListCategory) {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }
        pub fn print_categories(&self, type_: FreeListCategoryType) {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }

        #[cfg(debug_assertions)]
        pub fn sum_free_lists(&self) -> usize {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }

        #[cfg(debug_assertions)]
        pub fn is_very_long(&self) -> bool {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }

        fn verify_available(&self) {
            #[cfg(debug_assertions)]
            {
                assert!(self.is_very_long() || self.available_ == self.sum_free_lists());
            }
        }

        // Tries to retrieve a node from the first category in a given |type|.
        // Returns nullptr if the category is empty or the top entry is smaller
        // than minimum_size.
        fn try_find_node_in(
            &self,
            type_: FreeListCategoryType,
            minimum_size: usize,
            node_size: &mut usize,
        ) -> Tagged<FreeSpace> {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }

        // Searches a given |type| for a node of at least |minimum_size|.
        fn search_for_node_in_list(
            &self,
            type_: FreeListCategoryType,
            minimum_size: usize,
            node_size: &mut usize,
        ) -> Tagged<FreeSpace> {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }

        // Returns the smallest category in which an object of |size_in_bytes| could
        // fit.
        fn select_free_list_category_type(&self, size_in_bytes: usize) -> FreeListCategoryType {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }

        fn top(&self, type_: FreeListCategoryType) -> *mut FreeListCategory {
            self.categories_[type_ as usize]
        }

        fn get_page_for_category_type(&self, type_: FreeListCategoryType) -> *mut PageMetadata {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }
    }

    pub struct FreeListCategoryIterator<'a> {
        current_: *mut FreeListCategory,
        _free_list: &'a FreeList, // Added to avoid lifetime issues.
    }

    impl<'a> FreeListCategoryIterator<'a> {
        pub fn new(free_list: &'a FreeList, type_: FreeListCategoryType) -> Self {
            FreeListCategoryIterator {
                current_: free_list.categories_[type_ as usize],
                _free_list: free_list,
            }
        }

        pub fn has_next(&self) -> bool {
            !self.current_.is_null()
        }

        pub fn next(&mut self) -> *mut FreeListCategory {
            assert!(self.has_next());
            let tmp = self.current_;
            // Safe as long as FreeListCategory has the next pointer.
            self.current_ = unsafe { (*self.current_).next() };
            tmp
        }
    }

    // Use 24 Freelists: on per 16 bytes between 24 and 256, and then a few ones for
    // larger sizes. See the variable |categories_min| for the size of each
    // Freelist.  Allocation is done using a best-fit strategy (considering only the
    // first element of each category though).
    // Performances are expected to be worst than FreeListLegacy, but memory
    // consumption should be lower (since fragmentation should be lower).
    pub struct FreeListMany {
        base: FreeList,
    }

    impl FreeListMany {
        pub fn new() -> Self {
            FreeListMany {
                base: FreeList::new(Self::K_NUMBER_OF_CATEGORIES, Self::K_MIN_BLOCK_SIZE),
            }
        }
    }

    impl FreeListMany {
        pub const K_MIN_BLOCK_SIZE: usize = 3 * std::mem::size_of::<usize>();

        // This is a conservative upper bound. The actual maximum block size takes
        // padding and alignment of data and code pages into account.
        pub const K_MAX_BLOCK_SIZE: usize = 4096; //MutablePageMetadata::kPageSize; //Assuming page size is 4096
        // Largest size for which categories are still precise, and for which we can
        // therefore compute the category in constant time.
        pub const K_PRECISE_CATEGORY_MAX_SIZE: usize = 256;

        // Categories boundaries generated with:
        // perl -E '
        //      @cat = (24, map {$_*16} 2..16, 48, 64);
        //      while ($cat[-1] <= 32768) {
        //        push @cat, $cat[-1]*2
        //      }
        //      say join ", ", @cat;
        //      say "\n", scalar @cat'
        pub const K_NUMBER_OF_CATEGORIES: i32 = 24;
        pub const CATEGORIES_MIN: [u32; Self::K_NUMBER_OF_CATEGORIES as usize] = [
            24, 32, 48, 64, 80, 96, 112, 128, 144, 160, 176, 192, 208, 224, 240, 256, 512, 1024,
            2048, 4096, 8192, 16384, 32768, 65536,
        ];

        // Return the smallest category that could hold |size_in_bytes| bytes.
        fn select_free_list_category_type(&self, size_in_bytes: usize) -> FreeListCategoryType {
            if size_in_bytes <= Self::K_PRECISE_CATEGORY_MAX_SIZE {
                if size_in_bytes < Self::CATEGORIES_MIN[1] as usize {
                    return 0;
                }
                return ((size_in_bytes >> 4) - 1) as FreeListCategoryType;
            }
            for cat in (Self::K_PRECISE_CATEGORY_MAX_SIZE >> 4) - 1..self.base.last_category_ {
                if size_in_bytes < Self::CATEGORIES_MIN[(cat + 1) as usize] as usize {
                    return cat;
                }
            }
            self.base.last_category_
        }

        pub fn get_page_for_size(&self, size_in_bytes: usize) -> *mut PageMetadata {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }

        pub fn allocate(
            &mut self,
            size_in_bytes: usize,
            node_size: &mut usize,
            origin: AllocationOrigin,
        ) -> Tagged<FreeSpace> {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }
    }

    // Same as FreeListMany but uses a cache to know which categories are empty.
    // The cache (|next_nonempty_category|) is maintained in a way such that for
    // each category c, next_nonempty_category[c] contains the first non-empty
    // category greater or equal to c, that may hold an object of size c.
    // Allocation is done using the same strategy as FreeListMany (ie, best fit).
    pub struct FreeListManyCached {
        base: FreeListMany,
        next_nonempty_category: [i32; (FreeListMany::K_NUMBER_OF_CATEGORIES + 1) as usize],
    }

    impl FreeListManyCached {
        pub fn new() -> Self {
            let mut new_self = FreeListManyCached {
                base: FreeListMany::new(),
                next_nonempty_category: [0; (FreeListMany::K_NUMBER_OF_CATEGORIES + 1) as usize],
            };
            new_self.reset_cache();
            new_self
        }

        pub fn allocate(
            &mut self,
            size_in_bytes: usize,
            node_size: &mut usize,
            origin: AllocationOrigin,
        ) -> Tagged<FreeSpace> {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }

        pub fn free(&mut self, free_space: &WritableFreeSpace, mode: FreeMode) -> usize {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }

        pub fn reset(&mut self) {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }
        pub fn reset_for_non_black_allocated_pages(&mut self) {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }

        pub fn add_category(&mut self, category: *mut FreeListCategory) -> bool {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }
        pub fn remove_category(&mut self, category: *mut FreeListCategory) {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }

        // Updates the cache after adding something in the category |cat|.
        fn update_cache_after_addition(&mut self, cat: FreeListCategoryType) {
            for i in (K_FIRST_CATEGORY..=cat).rev() {
                if self.next_nonempty_category[i as usize] > cat {
                    self.next_nonempty_category[i as usize] = cat;
                }
            }
        }

        // Updates the cache after emptying category |cat|.
        fn update_cache_after_removal(&mut self, cat: FreeListCategoryType) {
            for i in (K_FIRST_CATEGORY..=cat).rev() {
                if self.next_nonempty_category[i as usize] == cat {
                    self.next_nonempty_category[i as usize] = self.next_nonempty_category[(cat + 1) as usize];
                }
            }
        }

        #[cfg(debug_assertions)]
        fn check_cache_integrity(&self) {
            for i in 0..=self.base.base.last_category_ {
                assert!(
                    self.next_nonempty_category[i as usize] == self.base.base.last_category_ + 1
                        || !self.base.base.categories_[self.next_nonempty_category[i as usize] as usize].is_null()
                );
                for j in i..self.next_nonempty_category[i as usize] {
                    assert!(self.base.base.categories_[j as usize].is_null());
                }
            }
        }

        fn reset_cache(&mut self) {
            for i in 0..FreeListMany::K_NUMBER_OF_CATEGORIES {
                self.next_nonempty_category[i as usize] = FreeListMany::K_NUMBER_OF_CATEGORIES;
            }
            // Setting the after-last element as well, as explained in the cache's
            // declaration.
            self.next_nonempty_category[FreeListMany::K_NUMBER_OF_CATEGORIES as usize] = FreeListMany::K_NUMBER_OF_CATEGORIES;
        }
    }

    // Same as FreeListManyCached but uses a fast path.
    // The fast path overallocates by at least 1.85k bytes. The idea of this 1.85k
    // is: we want the fast path to always overallocate, even for larger
    // categories. Therefore, we have two choices: either overallocate by
    // "size_in_bytes * something" or overallocate by "size_in_bytes +
    // something". We choose the later, as the former will tend to overallocate too
    // much for larger objects. The 1.85k (= 2048 - 128) has been chosen such that
    // for tiny objects (size <= 128 bytes), the first category considered is the
    // 36th (which holds objects of 2k to 3k), while for larger objects, the first
    // category considered will be one that guarantees a 1.85k+ bytes
    // overallocation. Using 2k rather than 1.85k would have resulted in either a
    // more complex logic for SelectFastAllocationFreeListCategoryType, or the 36th
    // category (2k to 3k) not being used; both of which are undesirable.
    // A secondary fast path is used for tiny objects (size <= 128), in order to
    // consider categories from 256 to 2048 bytes for them.
    // Note that this class uses a precise GetPageForSize (inherited from
    // FreeListMany), which makes its fast path less fast in the Scavenger. This is
    // done on purpose, since this class's only purpose is to be used by
    // FreeListManyCachedOrigin, which is precise for the scavenger.
    pub struct FreeListManyCachedFastPathBase {
        base: FreeListManyCached,
        small_blocks_mode_: SmallBlocksMode,
    }

    impl FreeListManyCachedFastPathBase {
        pub fn new(small_blocks_mode: SmallBlocksMode) -> Self {
            let mut new_self = FreeListManyCachedFastPathBase {
                base: FreeListManyCached::new(),
                small_blocks_mode_: small_blocks_mode,
            };

            if let SmallBlocksMode::Prohibit = new_self.small_blocks_mode_ {
                // Assuming v8_flags and KB are defined elsewhere. Using placeholder values.
                let minor_ms = false;
                let minor_ms_min_lab_size_kb = 0;
                new_self.base.base.base.base.min_block_size_ = if minor_ms && minor_ms_min_lab_size_kb > 0 {
                    minor_ms_min_lab_size_kb * 1024 // KB
                } else {
                    Self::K_FAST_PATH_START
                };
            }
            new_self
        }

        pub fn allocate(
            &mut self,
            size_in_bytes: usize,
            node_size: &mut usize,
            origin: AllocationOrigin,
        ) -> Tagged<FreeSpace> {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }

        // Objects in the 18th category are at least 2048 bytes
        const K_FAST_PATH_FIRST_CATEGORY: FreeListCategoryType = 18;
        const K_FAST_PATH_START: usize = 2048;
        const K_TINY_OBJECT_MAX_SIZE: usize = 128;
        const K_FAST_PATH_OFFSET: usize = Self::K_FAST_PATH_START - Self::K_TINY_OBJECT_MAX_SIZE;
        // Objects in the 15th category are at least 256 bytes
        const K_FAST_PATH_FALL_BACK_TINY: FreeListCategoryType = 15;

        fn select_fast_allocation_free_list_category_type(&self, size_in_bytes: usize) -> FreeListCategoryType {
            assert!(size_in_bytes < FreeListMany::K_MAX_BLOCK_SIZE);

            if size_in_bytes >= FreeListMany::CATEGORIES_MIN[(self.base.base.base.base.last_category_) as usize] as usize {
                return self.base.base.base.base.last_category_;
            }

            let mut size_in_bytes = size_in_bytes + Self::K_FAST_PATH_OFFSET;
            for cat in Self::K_FAST_PATH_FIRST_CATEGORY..self.base.base.base.base.last_category_ {
                if size_in_bytes <= FreeListMany::CATEGORIES_MIN[cat as usize] as usize {
                    return cat;
                }
            }
            self.base.base.base.base.last_category_
        }
    }

    #[derive(PartialEq, Eq)]
    pub enum SmallBlocksMode {
        Allow,
        Prohibit,
    }

    pub struct FreeListManyCachedFastPath {
        base: FreeListManyCachedFastPathBase,
    }

    impl FreeListManyCachedFastPath {
        pub fn new() -> Self {
            FreeListManyCachedFastPath {
                base: FreeListManyCachedFastPathBase::new(SmallBlocksMode::Allow),
            }
        }

        pub fn allocate(
            &mut self,
            size_in_bytes: usize,
            node_size: &mut usize,
            origin: AllocationOrigin,
        ) -> Tagged<FreeSpace> {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }
    }

    pub struct FreeListManyCachedFastPathForNewSpace {
        base: FreeListManyCachedFastPathBase,
    }

    impl FreeListManyCachedFastPathForNewSpace {
        pub fn new() -> Self {
            FreeListManyCachedFastPathForNewSpace {
                base: FreeListManyCachedFastPathBase::new(SmallBlocksMode::Prohibit),
            }
        }

        pub fn allocate(
            &mut self,
            size_in_bytes: usize,
            node_size: &mut usize,
            origin: AllocationOrigin,
        ) -> Tagged<FreeSpace> {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }
    }

    // Uses FreeListManyCached if in the GC; FreeListManyCachedFastPath otherwise.
    // The reasoning behind this FreeList is the following: the GC runs in
    // parallel, and therefore, more expensive allocations there are less
    // noticeable. On the other hand, the generated code and runtime need to be very
    // fast. Therefore, the strategy for the former is one that is not very
    // efficient, but reduces fragmentation (FreeListManyCached), while the strategy
    // for the later is one that is very efficient, but introduces some
    // fragmentation (FreeListManyCachedFastPath).
    pub struct FreeListManyCachedOrigin {
        base: FreeListManyCachedFastPath,
    }

    impl FreeListManyCachedOrigin {
        pub fn allocate(
            &mut self,
            size_in_bytes: usize,
            node_size: &mut usize,
            origin: AllocationOrigin,
        ) -> Tagged<FreeSpace> {
            // Implementation detail is not exposed in header.
            unimplemented!()
        }
    }
}