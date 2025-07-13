// Converted from V8 C++ source files:
// Header: bounded-page-allocator.h
// Implementation: bounded-page-allocator.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod base {
  use std::sync::Mutex;

  // Defines the page initialization mode of a BoundedPageAllocator.
  #[derive(PartialEq, Eq, Clone, Copy, Debug)]
  pub enum PageInitializationMode {
    // The contents of allocated pages must be zero initialized. This causes any
    // committed pages to be decommitted during FreePages and ReleasePages.
    kAllocatedPagesMustBeZeroInitialized,
    // Allocated pages do not have to be be zero initialized and can contain old
    // data. This is slightly faster as comitted pages are not decommitted
    // during FreePages and ReleasePages, but only made inaccessible.
    kAllocatedPagesCanBeUninitialized,
    // Assume pages are in discarded state and already have the right page
    // permissions. Using this mode requires PageFreeingMode::kDiscard.
    kRecommitOnly,
  }

  // Defines how BoundedPageAllocator frees pages when FreePages or ReleasePages
  // is requested.
  #[derive(PartialEq, Eq, Clone, Copy, Debug)]
  pub enum PageFreeingMode {
    // Pages are freed/released by setting permissions to kNoAccess. This is the
    // preferred mode when current platform/configuration allows any page
    // permissions reconfiguration.
    kMakeInaccessible,

    // Pages are freed/released by using DiscardSystemPages of the underlying
    // page allocator. This mode should be used for the cases when page permission
    // reconfiguration is not allowed. In particular, on MacOS on ARM64 ("Apple
    // M1"/Apple Silicon) it's not allowed to reconfigure RWX pages to anything
    // else.
    // This mode is not compatible with kAllocatedPagesMustBeZeroInitialized
    // page initialization mode.
    kDiscard,
  }

  pub enum Permission {
      kNoAccess,
      kReadWrite,
      kReadExecute,
      kReadWriteExecute,
      kNoAccessWillJitLater,
  }

  pub trait PageAllocator {
      fn AllocatePageSize(&self) -> usize;
      fn CommitPageSize(&self) -> usize;
      fn SetRandomMmapSeed(&mut self, seed: i64);
      fn GetRandomMmapAddr(&self) -> *mut std::ffi::c_void;
      fn AllocatePages(&mut self, hint: *mut std::ffi::c_void, size: usize, alignment: usize, access: Permission) -> *mut std::ffi::c_void;
      fn ReserveForSharedMemoryMapping(&mut self, address: *mut std::ffi::c_void, size: usize) -> bool;
      fn AllocatePagesAt(&mut self, address: usize, size: usize, access: Permission) -> bool;
      fn FreePages(&mut self, address: *mut std::ffi::c_void, size: usize) -> bool;
      fn ReleasePages(&mut self, address: *mut std::ffi::c_void, size: usize, new_size: usize) -> bool;
      fn SetPermissions(&mut self, address: *mut std::ffi::c_void, size: usize, access: Permission) -> bool;
      fn RecommitPages(&mut self, address: *mut std::ffi::c_void, size: usize, access: Permission) -> bool;
      fn DiscardSystemPages(&mut self, address: *mut std::ffi::c_void, size: usize) -> bool;
      fn DecommitPages(&mut self, address: *mut std::ffi::c_void, size: usize) -> bool;
      fn SealPages(&mut self, address: *mut std::ffi::c_void, size: usize) -> bool;
  }

  #[derive(Debug)]
  pub enum RegionAllocatorError {
      OutOfMemory,
      AddressTaken,
  }

  #[derive(Debug)]
  pub struct Region {
      pub start: usize,
      pub size: usize,
      pub state: RegionState,
  }

  #[derive(Debug, PartialEq, Eq)]
  pub enum RegionState {
      Free,
      Allocated,
      Excluded,
  }

  #[derive(Debug)]
  pub struct RegionAllocator {
      start: usize,
      size: usize,
      page_size: usize,
      regions: Vec<Region>,
  }

  impl RegionAllocator {
      pub const kAllocationFailure: usize = usize::MAX;

      pub fn new(start: usize, size: usize, page_size: usize) -> Self {
          RegionAllocator {
              start,
              size,
              page_size,
              regions: vec![Region {
                  start,
                  size,
                  state: RegionState::Free,
              }],
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

      pub fn contains(&self, address: usize) -> bool {
          address >= self.start && address < self.start + self.size
      }

      pub fn contains_with_size(&self, address: usize, size: usize) -> bool {
          self.contains(address) && (address + size) <= (self.start + self.size)
      }

      pub fn allocate_region(&mut self, size: usize) -> Result<usize, RegionAllocatorError> {
          self.allocate_aligned_region(size, self.page_size)
      }

      pub fn allocate_aligned_region(&mut self, size: usize, alignment: usize) -> Result<usize, RegionAllocatorError> {
          for i in 0..self.regions.len() {
              if self.regions[i].state == RegionState::Free {
                  let aligned_start = align_up(self.regions[i].start, alignment);
                  if aligned_start >= self.start && (aligned_start + size) <= (self.start + self.size) && (aligned_start + size) <= (self.regions[i].start + self.regions[i].size) {
                      return self.allocate_region_at(aligned_start, size);
                  }
              }
          }
          Err(RegionAllocatorError::OutOfMemory)
      }

      pub fn allocate_region_at(&mut self, address: usize, size: usize) -> Result<usize, RegionAllocatorError> {
          if !self.contains_with_size(address, size) {
              return Err(RegionAllocatorError::OutOfMemory);
          }

          let mut found = false;
          let mut region_index = 0;
          for i in 0..self.regions.len() {
              if self.regions[i].start == address && self.regions[i].state == RegionState::Free{
                  found = true;
                  region_index = i;
                  break;
              }
          }
          if !found {
              return Err(RegionAllocatorError::AddressTaken);
          }
          
          let region = &mut self.regions[region_index];
          if region.start != address {
              return Err(RegionAllocatorError::AddressTaken); //Address taken already
          }

          if region.size < size {
              return Err(RegionAllocatorError::OutOfMemory); //Not enough space in region
          }
          
          if region.size == size{
              region.state = RegionState::Allocated;
              return Ok(address);
          }

          //Split the region
          let new_region = Region {
              start: address + size,
              size: region.size - size,
              state: RegionState::Free,
          };
          region.size = size;
          region.state = RegionState::Allocated;
          self.regions.insert(region_index + 1, new_region);
          
          Ok(address)
      }
    
      pub fn allocate_region_at_with_state(&mut self, address: usize, size: usize, state: RegionState) -> Result<usize, RegionAllocatorError> {
          if !self.contains_with_size(address, size) {
              return Err(RegionAllocatorError::OutOfMemory);
          }

          let mut found = false;
          let mut region_index = 0;
          for i in 0..self.regions.len() {
              if self.regions[i].start == address && self.regions[i].state == RegionState::Free{
                  found = true;
                  region_index = i;
                  break;
              }
          }
          if !found {
              return Err(RegionAllocatorError::AddressTaken);
          }
          
          let region = &mut self.regions[region_index];
          if region.start != address {
              return Err(RegionAllocatorError::AddressTaken); //Address taken already
          }

          if region.size < size {
              return Err(RegionAllocatorError::OutOfMemory); //Not enough space in region
          }
          
          if region.size == size{
              region.state = state;
              return Ok(address);
          }

          //Split the region
          let new_region = Region {
              start: address + size,
              size: region.size - size,
              state: RegionState::Free,
          };
          region.size = size;
          region.state = state;
          self.regions.insert(region_index + 1, new_region);
          
          Ok(address)
      }
      
      pub fn free_region(&mut self, address: usize) -> usize {
          let mut freed_size = 0;
          let mut region_index = 0;
          let mut found = false;
          for i in 0..self.regions.len() {
              if self.regions[i].start == address && self.regions[i].state == RegionState::Allocated{
                  found = true;
                  region_index = i;
                  break;
              }
          }
          if !found {
              return 0; //Region wasn't allocated
          }
          self.regions[region_index].state = RegionState::Free;
          freed_size = self.regions[region_index].size;

          //Try merging with the next region
          if region_index + 1 < self.regions.len() && self.regions[region_index+1].state == RegionState::Free{
              self.regions[region_index].size += self.regions[region_index+1].size;
              self.regions.remove(region_index+1);
          }

          //Try merging with the previous region
          if region_index > 0 && self.regions[region_index-1].state == RegionState::Free{
              self.regions[region_index-1].size += self.regions[region_index].size;
              self.regions.remove(region_index);
          }
          
          freed_size
      }

      pub fn trim_region(&mut self, address: usize, new_size: usize) {
          let mut found = false;
          let mut region_index = 0;
          for i in 0..self.regions.len() {
              if self.regions[i].start == address && self.regions[i].state == RegionState::Allocated{
                  found = true;
                  region_index = i;
                  break;
              }
          }
          if !found {
              return; //Region wasn't allocated
          }

          let region = &mut self.regions[region_index];
          if region.size <= new_size {
              return; //Nothing to trim
          }

          let free_size = region.size - new_size;
          region.size = new_size;

          let new_region = Region {
              start: address + new_size,
              size: free_size,
              state: RegionState::Free,
          };
          self.regions.insert(region_index + 1, new_region);
      }
      
      #[allow(dead_code)]
      pub fn check_region(&self, address: usize) -> usize {
          for i in 0..self.regions.len() {
              if self.regions[i].start == address && self.regions[i].state == RegionState::Allocated{
                  return self.regions[i].size;
              }
          }
          0
      }
  }

  fn align_up(address: usize, alignment: usize) -> usize {
      (address + alignment - 1) & !(alignment - 1)
  }

  // This is a v8::PageAllocator implementation that allocates pages within the
  // pre-reserved region of virtual space. This class requires the virtual space
  // to be kept reserved during the lifetime of this object.
  // The main application of bounded page allocator are
  //  - V8 heap pointer compression which requires the whole V8 heap to be
  //    allocated within a contiguous range of virtual address space,
  //  - executable page allocation, which allows to use PC-relative 32-bit code
  //    displacement on certain 64-bit platforms.
  // Bounded page allocator uses other page allocator instance for doing actual
  // page allocations.
  // The implementation is thread-safe.
  pub struct BoundedPageAllocator<'a> {
      mutex_: Mutex<()>,
      allocate_page_size_: usize,
      commit_page_size_: usize,
      page_allocator_: &'a mut dyn PageAllocator,
      region_allocator_: RegionAllocator,
      page_initialization_mode_: PageInitializationMode,
      page_freeing_mode_: PageFreeingMode,
      allocation_status_: AllocationStatus,
  }

  impl<'a> BoundedPageAllocator<'a> {
      pub fn new(
          page_allocator: &'a mut dyn PageAllocator,
          start: usize,
          size: usize,
          allocate_page_size: usize,
          page_initialization_mode: PageInitializationMode,
          page_freeing_mode: PageFreeingMode,
      ) -> Self {
          assert_ne!(page_allocator as *mut dyn PageAllocator as *mut std::ffi::c_void, std::ptr::null_mut());
          assert!(Self::is_aligned(allocate_page_size, page_allocator.AllocatePageSize()));
          assert!(Self::is_aligned(allocate_page_size, page_allocator.CommitPageSize()));

          BoundedPageAllocator {
              mutex_: Mutex::new(()),
              allocate_page_size_: allocate_page_size,
              commit_page_size_: page_allocator.CommitPageSize(),
              page_allocator_: page_allocator,
              region_allocator_: RegionAllocator::new(start, size, allocate_page_size),
              page_initialization_mode_: page_initialization_mode,
              page_freeing_mode_: page_freeing_mode,
              allocation_status_: AllocationStatus::kSuccess,
          }
      }

      fn is_aligned(value: usize, alignment: usize) -> bool {
        alignment != 0 && (value % alignment == 0)
      }
      
      pub fn begin(&self) -> usize {
          self.region_allocator_.begin()
      }

      pub fn size(&self) -> usize {
          self.region_allocator_.size()
      }

      // Returns true if given address is in the range controlled by the bounded
      // page allocator instance.
      pub fn contains(&self, address: usize) -> bool {
          self.region_allocator_.contains(address)
      }

      pub fn get_last_allocation_status(&self) -> AllocationStatus {
          self.allocation_status_
      }

      pub fn allocation_status_to_string(allocation_status: AllocationStatus) -> &'static str {
          match allocation_status {
              AllocationStatus::kSuccess => "Success",
              AllocationStatus::kFailedToCommit => "Failed to commit",
              AllocationStatus::kRanOutOfReservation => "Ran out of reservation",
              AllocationStatus::kHintedAddressTakenOrNotFound => "Hinted address was taken or not found",
          }
      }
  }

  impl<'a> PageAllocator for BoundedPageAllocator<'a> {
      fn AllocatePageSize(&self) -> usize {
          self.allocate_page_size_
      }

      fn CommitPageSize(&self) -> usize {
          self.commit_page_size_
      }

      fn SetRandomMmapSeed(&mut self, seed: i64) {
          self.page_allocator_.SetRandomMmapSeed(seed);
      }

      fn GetRandomMmapAddr(&self) -> *mut std::ffi::c_void {
          self.page_allocator_.GetRandomMmapAddr()
      }

      fn AllocatePages(
          &mut self,
          hint: *mut std::ffi::c_void,
          size: usize,
          alignment: usize,
          access: Permission,
      ) -> *mut std::ffi::c_void {
          let _guard = self.mutex_.lock().unwrap();
          assert!(Self::is_aligned(alignment, self.region_allocator_.page_size()));
          assert!(Self::is_aligned(alignment, self.allocate_page_size_));

          let mut address = RegionAllocator::kAllocationFailure;
          let hint_address = hint as usize;
          if hint_address != 0 && Self::is_aligned(hint_address, alignment) && self.region_allocator_.contains_with_size(hint_address, size) {
              match self.region_allocator_.allocate_region_at(hint_address, size) {
                  Ok(addr) => {
                      address = addr;
                  }
                  Err(_) => {},
              }
          }

          if address == RegionAllocator::kAllocationFailure {
              if alignment <= self.allocate_page_size_ {
                  match self.region_allocator_.allocate_region(size) {
                      Ok(addr) => {
                          address = addr;
                      }
                      Err(_) => {},
                  }
              } else {
                  match self.region_allocator_.allocate_aligned_region(size, alignment) {
                      Ok(addr) => {
                          address = addr;
                      }
                      Err(_) => {},
                  }
              }
          }

          if address == RegionAllocator::kAllocationFailure {
              self.allocation_status_ = AllocationStatus::kRanOutOfReservation;
              return std::ptr::null_mut();
          }

          let ptr = address as *mut std::ffi::c_void;

          if access == Permission::kNoAccess || access == Permission::kNoAccessWillJitLater {
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
          assert_eq!(self.region_allocator_.free_region(address), size);
          self.allocation_status_ = AllocationStatus::kFailedToCommit;
          std::ptr::null_mut()
      }

      fn AllocatePagesAt(
          &mut self,
          address: usize,
          size: usize,
          access: Permission,
      ) -> bool {
          let _guard = self.mutex_.lock().unwrap();

          assert!(Self::is_aligned(address, self.allocate_page_size_));
          assert!(Self::is_aligned(size, self.allocate_page_size_));
          assert!(self.region_allocator_.contains_with_size(address, size));

          match self.region_allocator_.allocate_region_at(address, size) {
              Ok(_) => {
                  let ptr = address as *mut std::ffi::c_void;
                  if !self.page_allocator_.SetPermissions(ptr, size, access) {
                      // This most likely means that we ran out of memory.
                      assert_eq!(self.region_allocator_.free_region(address), size);
                      self.allocation_status_ = AllocationStatus::kFailedToCommit;
                      return false;
                  }

                  self.allocation_status_ = AllocationStatus::kSuccess;
                  true
              }
              Err(_) => {
                  self.allocation_status_ = AllocationStatus::kHintedAddressTakenOrNotFound;
                  false
              }
          }
      }

      fn ReserveForSharedMemoryMapping(
          &mut self,
          ptr: *mut std::ffi::c_void,
          size: usize,
      ) -> bool {
          let _guard = self.mutex_.lock().unwrap();

          let address = ptr as usize;
          assert!(Self::is_aligned(address, self.allocate_page_size_));
          assert!(Self::is_aligned(size, self.commit_page_size_));
          assert!(self.region_allocator_.contains_with_size(address, size));

          // Region allocator requires page size rather than commit size so just over-
          // allocate there since any extra space couldn't be used anyway.
          let region_size = (size + self.allocate_page_size_ - 1) & !(self.allocate_page_size_ - 1);
          match self.region_allocator_.allocate_region_at_with_state(address, region_size, RegionState::Excluded) {
              Ok(_) => {
                  let success = self.page_allocator_.SetPermissions(
                      ptr,
                      size,
                      Permission::kNoAccess,
                  );
                  if success {
                      self.allocation_status_ = AllocationStatus::kSuccess;
                  } else {
                      self.allocation_status_ = AllocationStatus::kFailedToCommit;
                  }
                  success
              }
              Err(_) => {
                  self.allocation_status_ = AllocationStatus::kHintedAddressTakenOrNotFound;
                  false
              }
          }
      }

      fn FreePages(&mut self, raw_address: *mut std::ffi::c_void, size: usize) -> bool {
          // Careful: we are not locked here, do not touch BoundedPageAllocator
          // metadata.
          let mut success = false;
          let address = raw_address as usize;

          if self.page_initialization_mode_ == PageInitializationMode::kAllocatedPagesMustBeZeroInitialized {
              assert_ne!(self.page_freeing_mode_, PageFreeingMode::kDiscard);
              // When we are required to return zero-initialized pages, we decommit the
              // pages here, which will cause any wired pages to be removed by the OS.
              success = self.page_allocator_.DecommitPages(raw_address, size);
          } else {
              match self.page_freeing_mode_ {
                  PageFreeingMode::kMakeInaccessible => {
                      assert_eq!(
                          self.page_initialization_mode_,
                          PageInitializationMode::kAllocatedPagesCanBeUninitialized
                      );
                      success = self.page_allocator_.SetPermissions(raw_address, size, Permission::kNoAccess);
                  }

                  PageFreeingMode::kDiscard => {
                      success = self.page_allocator_.DiscardSystemPages(raw_address, size);
                  }
              }
          }
          let _guard = self.mutex_.lock().unwrap();
          assert_eq!(size, self.region_allocator_.free_region(address));

          success
      }

      fn ReleasePages(&mut self, raw_address: *mut std::ffi::c_void, size: usize, new_size: usize) -> bool {
          let address = raw_address as usize;
          assert!(Self::is_aligned(address, self.allocate_page_size_));
          assert!(new_size < size);
          assert!(Self::is_aligned(size - new_size, self.commit_page_size_));

          // This must be held until the page permissions are updated.
          let _guard = self.mutex_.lock().unwrap();

          // Check if we freed any allocatable pages by this release.
          let allocated_size = (size + self.allocate_page_size_ - 1) & !(self.allocate_page_size_ - 1);
          let new_allocated_size = (new_size + self.allocate_page_size_ - 1) & !(self.allocate_page_size_ - 1);

          if new_allocated_size < allocated_size {
              self.region_allocator_.trim_region(address, new_allocated_size);
          }

          // Keep the region in "used" state just uncommit some pages.
          let free_address = (address + new_size) as *mut std::ffi::c_void;
          let free_size = size - new_size;
          if self.page_initialization_mode_ == PageInitializationMode::kAllocatedPagesMustBeZeroInitialized {
              assert_ne!(self.page_freeing_mode_, PageFreeingMode::kDiscard);
              // See comment in FreePages().
              return self.page_allocator_.DecommitPages(free_address, free_size);
          }
          if self.page_freeing_mode_ == PageFreeingMode::kMakeInaccessible {
              assert_eq!(
                  self.page_initialization_mode_,
                  PageInitializationMode::kAllocatedPagesCanBeUninitialized
              );
              return self.page_allocator_.SetPermissions(free_address, free_size, Permission::kNoAccess);
          }
          assert_eq!(self.page_freeing_mode_, PageFreeingMode::kDiscard);
          self.page_allocator_.DiscardSystemPages(free_address, free_size)
      }

      fn SetPermissions(&mut self, address: *mut std::ffi::c_void, size: usize, access: Permission) -> bool {
          assert!(Self::is_aligned(address as usize, self.commit_page_size_));
          assert!(Self::is_aligned(size, self.commit_page_size_));
          assert!(self.region_allocator_.contains_with_size(address as usize, size));
          let success = self.page_allocator_.SetPermissions(address, size, access);
          if !success {
              self.allocation_status_ = AllocationStatus::kFailedToCommit;
          }
          success
      }

      fn RecommitPages(&mut self, address: *mut std::ffi::c_void, size: usize, access: Permission) -> bool {
          assert!(Self::is_aligned(address as usize, self.commit_page_size_));
          assert!(Self::is_aligned(size, self.commit_page_size_));
          assert!(self.region_allocator_.contains_with_size(address as usize, size));
          let success = self.page_allocator_.RecommitPages(address, size, access);
          if !success {
              self.allocation_status_ = AllocationStatus::kFailedToCommit;
          }
          success
      }

      fn DiscardSystemPages(&mut self, address: *mut std::ffi::c_void, size: usize) -> bool {
          self.page_allocator_.DiscardSystemPages(address, size)
      }

      fn DecommitPages(&mut self, address: *mut std::ffi::c_void, size: usize) -> bool {
          self.page_allocator_.DecommitPages(address, size)
      }

      fn SealPages(&mut self, address: *mut std::ffi::c_void, size: usize) -> bool {
          self.page_allocator_.SealPages(address, size)
      }
  }

  #[derive(PartialEq, Eq, Clone, Copy, Debug)]
  pub enum AllocationStatus {
      kSuccess,
      kFailedToCommit,
      kRanOutOfReservation,
      kHintedAddressTakenOrNotFound,
  }
}
