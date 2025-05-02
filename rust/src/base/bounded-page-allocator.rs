// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::sync::{Mutex, MutexGuard};
use std::ptr::null_mut;

mod page_allocator {
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum Permission {
        kNoAccess,
        kNoAccessWillJitLater,
        kRead,
        kReadWrite,
        kReadExecute,
        kReadWriteExecute,
    }

    pub trait PageAllocator {
        fn AllocatePageSize(&self) -> usize;
        fn CommitPageSize(&self) -> usize;
        fn SetPermissions(&self, address: *mut std::ffi::c_void, size: usize, access: Permission) -> bool;
        fn RecommitPages(&self, address: *mut std::ffi::c_void, size: usize, access: Permission) -> bool;
        fn DiscardSystemPages(&self, address: *mut std::ffi::c_void, size: usize) -> bool;
        fn DecommitPages(&self, address: *mut std::ffi::c_void, size: usize) -> bool;
        fn SealPages(&self, address: *mut std::ffi::c_void, size: usize) -> bool;
    }
}

mod region_allocator {
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum RegionState {
        kFree,
        kAllocated,
        kExcluded,
    }

    pub struct RegionAllocator {
        start: usize,
        size: usize,
        page_size: usize,
    }

    impl RegionAllocator {
        pub const kAllocationFailure: usize = 0;

        pub fn new(start: usize, size: usize, page_size: usize) -> Self {
            RegionAllocator {
                start,
                size,
                page_size,
            }
        }

        pub fn begin(&self) -> usize {
            self.start
        }

        pub fn size(&self) -> usize {
            self.size
        }

        pub fn page_size(&self) -> usize {
            self.page_size
        }

        pub fn contains(&self, address: usize, size: usize) -> bool {
            address >= self.start && (address + size) <= (self.start + self.size)
        }

        pub fn AllocateRegionAt(&mut self, address: usize, size: usize) -> bool {
            // Placeholder implementation
            self.AllocateRegionAt_internal(address, size, RegionState::kAllocated)
        }

        pub fn AllocateRegionAt_internal(&mut self, address: usize, size: usize, state: RegionState) -> bool {
            if !self.contains(address, size) {
                return false;
            }
            // Placeholder: In real implementation, track allocated regions to prevent overlaps
            true
        }

        pub fn AllocateRegionAt_excluded(&mut self, address: usize, size: usize) -> bool {
            self.AllocateRegionAt_internal(address, size, RegionState::kExcluded)
        }

        pub fn AllocateRegion(&mut self, size: usize) -> usize {
            // Placeholder implementation, always returns allocation failure.
            RegionAllocator::kAllocationFailure
        }
        
        pub fn AllocateAlignedRegion(&mut self, size: usize, alignment: usize) -> usize {
            // Placeholder implementation, always returns allocation failure.
            RegionAllocator::kAllocationFailure
        }

        pub fn FreeRegion(&mut self, address: usize) -> usize {
            // Placeholder implementation
            self.page_size()
        }

        pub fn TrimRegion(&mut self, address: usize, new_size: usize) {
            // Placeholder implementation
        }

        #[cfg(debug_assertions)]
        pub fn CheckRegion(&self, address: usize) -> usize {
            // Placeholder implementation, returns page_size always
            self.page_size()
        }
    }
}

pub mod base {
    use super::*;
    use page_allocator::PageAllocator;
    use region_allocator::RegionAllocator;

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum PageInitializationMode {
        kAllocatedPagesCanBeUninitialized,
        kAllocatedPagesMustBeZeroInitialized,
        kRecommitOnly,
    }

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum PageFreeingMode {
        kMakeInaccessible,
        kDiscard,
    }

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum AllocationStatus {
        kSuccess,
        kFailedToCommit,
        kRanOutOfReservation,
        kHintedAddressTakenOrNotFound,
    }

    pub struct BoundedPageAllocator<'a> {
        allocate_page_size_: usize,
        commit_page_size_: usize,
        page_allocator_: &'a dyn PageAllocator,
        region_allocator_: RegionAllocator,
        page_initialization_mode_: PageInitializationMode,
        page_freeing_mode_: PageFreeingMode,
        mutex_: Mutex<()>,
        allocation_status_: AllocationStatus,
    }

    impl<'a> BoundedPageAllocator<'a> {
        pub fn new(
            page_allocator: &'a dyn PageAllocator,
            start: usize,
            size: usize,
            allocate_page_size: usize,
            page_initialization_mode: PageInitializationMode,
            page_freeing_mode: PageFreeingMode,
        ) -> Self {
            assert!(is_aligned(allocate_page_size, page_allocator.AllocatePageSize()));
            assert!(is_aligned(allocate_page_size, page_allocator.CommitPageSize()));

            BoundedPageAllocator {
                allocate_page_size_: allocate_page_size,
                commit_page_size_: page_allocator.CommitPageSize(),
                page_allocator_: page_allocator,
                region_allocator_: RegionAllocator::new(start, size, allocate_page_size),
                page_initialization_mode_: page_initialization_mode,
                page_freeing_mode_: page_freeing_mode,
                mutex_: Mutex::new(()),
                allocation_status_: AllocationStatus::kSuccess,
            }
        }

        pub fn begin(&self) -> usize {
            self.region_allocator_.begin()
        }

        pub fn size(&self) -> usize {
            self.region_allocator_.size()
        }

        pub fn AllocatePages(
            &mut self,
            hint: *mut std::ffi::c_void,
            size: usize,
            alignment: usize,
            access: page_allocator::Permission,
        ) -> *mut std::ffi::c_void {
            let _guard = self.mutex_.lock().unwrap();
            assert!(is_aligned(alignment, self.region_allocator_.page_size()));
            assert!(is_aligned(alignment, self.allocate_page_size_));

            let mut address = RegionAllocator::kAllocationFailure;

            let hint_address = hint as usize;
            if hint_address != 0 && is_aligned(hint_address, alignment)
                && self.region_allocator_.contains(hint_address, size)
            {
                if self.region_allocator_.AllocateRegionAt(hint_address, size) {
                    address = hint_address;
                }
            }

            if address == RegionAllocator::kAllocationFailure {
                if alignment <= self.allocate_page_size_ {
                    // TODO(ishell): Consider using randomized version here.
                    address = self.region_allocator_.AllocateRegion(size);
                } else {
                    address = self.region_allocator_.AllocateAlignedRegion(size, alignment);
                }
            }

            if address == RegionAllocator::kAllocationFailure {
                self.allocation_status_ = AllocationStatus::kRanOutOfReservation;
                return null_mut();
            }

            let ptr = address as *mut std::ffi::c_void;
            // It's assumed that free regions are in kNoAccess/kNoAccessWillJitLater
            // state.
            if access == page_allocator::Permission::kNoAccess ||
                access == page_allocator::Permission::kNoAccessWillJitLater {
                self.allocation_status_ = AllocationStatus::kSuccess;
                return ptr;
            }

            if self.page_initialization_mode_ == PageInitializationMode::kRecommitOnly {
                if self.page_allocator_.RecommitPages(ptr, size, access) {
                    self.allocation_status_ = AllocationStatus::kSuccess;
                    return ptr;
                }
            } else {
                if self.page_allocator_.SetPermissions(ptr, size, access) {
                    self.allocation_status_ = AllocationStatus::kSuccess;
                    return ptr;
                }
            }

            // This most likely means that we ran out of memory.
            assert_eq!(self.region_allocator_.FreeRegion(address), size);
            self.allocation_status_ = AllocationStatus::kFailedToCommit;
            null_mut()
        }

        pub fn AllocatePagesAt(
            &mut self,
            address: usize,
            size: usize,
            access: page_allocator::Permission,
        ) -> bool {
            let _guard = self.mutex_.lock().unwrap();

            assert!(is_aligned(address, self.allocate_page_size_));
            assert!(is_aligned(size, self.allocate_page_size_));

            assert!(self.region_allocator_.contains(address, size));

            if !self.region_allocator_.AllocateRegionAt(address, size) {
                self.allocation_status_ = AllocationStatus::kHintedAddressTakenOrNotFound;
                return false;
            }

            let ptr = address as *mut std::ffi::c_void;
            if !self.page_allocator_.SetPermissions(ptr, size, access) {
                // This most likely means that we ran out of memory.
                assert_eq!(self.region_allocator_.FreeRegion(address), size);
                self.allocation_status_ = AllocationStatus::kFailedToCommit;
                return false;
            }

            self.allocation_status_ = AllocationStatus::kSuccess;
            true
        }

        pub fn ReserveForSharedMemoryMapping(
            &mut self,
            ptr: *mut std::ffi::c_void,
            size: usize,
        ) -> bool {
            let _guard = self.mutex_.lock().unwrap();

            let address = ptr as usize;
            assert!(is_aligned(address, self.allocate_page_size_));
            assert!(is_aligned(size, self.commit_page_size_));

            assert!(self.region_allocator_.contains(address, size));

            // Region allocator requires page size rather than commit size so just over-
            // allocate there since any extra space couldn't be used anyway.
            let region_size = round_up(size, self.allocate_page_size_);
            if !self.region_allocator_.AllocateRegionAt_excluded(address, region_size) {
                self.allocation_status_ = AllocationStatus::kHintedAddressTakenOrNotFound;
                return false;
            }

            let success = self.page_allocator_.SetPermissions(
                ptr,
                size,
                page_allocator::Permission::kNoAccess,
            );
            if success {
                self.allocation_status_ = AllocationStatus::kSuccess;
            } else {
                self.allocation_status_ = AllocationStatus::kFailedToCommit;
            }
            success
        }

        pub fn FreePages(&mut self, raw_address: *mut std::ffi::c_void, size: usize) -> bool {
            // Careful: we are not locked here, do not touch BoundedPageAllocator
            // metadata.
            let mut success: bool;
            let address = raw_address as usize;

            // The operations below can be expensive, don't hold the lock while they
            // happen. There is still potentially contention in the kernel, but at least
            // we don't need to hold the V8-side lock.
            if self.page_initialization_mode_ ==
                PageInitializationMode::kAllocatedPagesMustBeZeroInitialized {
                assert_ne!(self.page_freeing_mode_, PageFreeingMode::kDiscard);
                // When we are required to return zero-initialized pages, we decommit the
                // pages here, which will cause any wired pages to be removed by the OS.
                success = self.page_allocator_.DecommitPages(raw_address, size);
            } else {
                match self.page_freeing_mode_ {
                    PageFreeingMode::kMakeInaccessible => {
                        assert_eq!(self.page_initialization_mode_,
                                   PageInitializationMode::kAllocatedPagesCanBeUninitialized);
                        success = self.page_allocator_.SetPermissions(raw_address, size,
                                                                      page_allocator::Permission::kNoAccess);
                    }

                    PageFreeingMode::kDiscard => {
                        success = self.page_allocator_.DiscardSystemPages(raw_address, size);
                    }
                }
            }

            let _guard = self.mutex_.lock().unwrap();
            assert_eq!(size, self.region_allocator_.FreeRegion(address));

            success
        }

        pub fn ReleasePages(
            &mut self,
            raw_address: *mut std::ffi::c_void,
            size: usize,
            new_size: usize,
        ) -> bool {
            let address = raw_address as usize;
            assert!(is_aligned(address, self.allocate_page_size_));

            assert!(new_size < size);
            assert!(is_aligned(size - new_size, self.commit_page_size_));

            // This must be held until the page permissions are updated.
            let _guard = self.mutex_.lock().unwrap();

            // Check if we freed any allocatable pages by this release.
            let allocated_size = round_up(size, self.allocate_page_size_);
            let new_allocated_size = round_up(new_size, self.allocate_page_size_);

            #[cfg(debug_assertions)]
            {
                // There must be an allocated region at given |address| of a size not
                // smaller than |size|.
                assert_eq!(allocated_size, self.region_allocator_.CheckRegion(address));
            }

            if new_allocated_size < allocated_size {
                self.region_allocator_.TrimRegion(address, new_allocated_size);
            }

            // Keep the region in "used" state just uncommit some pages.
            let free_address = (address + new_size) as *mut std::ffi::c_void;
            let free_size = size - new_size;
            if self.page_initialization_mode_ ==
                PageInitializationMode::kAllocatedPagesMustBeZeroInitialized {
                assert_ne!(self.page_freeing_mode_, PageFreeingMode::kDiscard);
                // See comment in FreePages().
                return self.page_allocator_.DecommitPages(free_address, free_size);
            }
            if self.page_freeing_mode_ == PageFreeingMode::kMakeInaccessible {
                assert_eq!(self.page_initialization_mode_,
                           PageInitializationMode::kAllocatedPagesCanBeUninitialized);
                return self.page_allocator_.SetPermissions(free_address, free_size,
                                                           page_allocator::Permission::kNoAccess);
            }
            assert_eq!(self.page_freeing_mode_, PageFreeingMode::kDiscard);
            self.page_allocator_.DiscardSystemPages(free_address, free_size)
        }

        pub fn SetPermissions(
            &mut self,
            address: *mut std::ffi::c_void,
            size: usize,
            access: page_allocator::Permission,
        ) -> bool {
            assert!(is_aligned(address as usize, self.commit_page_size_));
            assert!(is_aligned(size, self.commit_page_size_));
            assert!(self.region_allocator_.contains(address as usize, size));
            let success = self.page_allocator_.SetPermissions(address, size, access);
            if !success {
                self.allocation_status_ = AllocationStatus::kFailedToCommit;
            }
            success
        }

        pub fn RecommitPages(
            &mut self,
            address: *mut std::ffi::c_void,
            size: usize,
            access: page_allocator::Permission,
        ) -> bool {
            assert!(is_aligned(address as usize, self.commit_page_size_));
            assert!(is_aligned(size, self.commit_page_size_));
            assert!(self.region_allocator_.contains(address as usize, size));
            let success = self.page_allocator_.RecommitPages(address, size, access);
            if !success {
                self.allocation_status_ = AllocationStatus::kFailedToCommit;
            }
            success
        }

        pub fn DiscardSystemPages(&self, address: *mut std::ffi::c_void, size: usize) -> bool {
            self.page_allocator_.DiscardSystemPages(address, size)
        }

        pub fn DecommitPages(&self, address: *mut std::ffi::c_void, size: usize) -> bool {
            self.page_allocator_.DecommitPages(address, size)
        }

        pub fn SealPages(&self, address: *mut std::ffi::c_void, size: usize) -> bool {
            self.page_allocator_.SealPages(address, size)
        }

        pub fn AllocationStatusToString(allocation_status: AllocationStatus) -> &'static str {
            match allocation_status {
                AllocationStatus::kSuccess => "Success",
                AllocationStatus::kFailedToCommit => "Failed to commit",
                AllocationStatus::kRanOutOfReservation => "Ran out of reservation",
                AllocationStatus::kHintedAddressTakenOrNotFound => "Hinted address was taken or not found",
            }
        }
    }

    const fn is_aligned(value: usize, alignment: usize) -> bool {
        (value & (alignment - 1)) == 0
    }

    const fn round_up(value: usize, alignment: usize) -> usize {
        (value + alignment - 1) & !(alignment - 1)
    }
}