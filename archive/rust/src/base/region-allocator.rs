// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::cmp::{max, min};
use std::collections::BTreeSet;
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

//use crate::base::bits; // Assuming this is a local module

// If |free_size| < |region_size| * |kMaxLoadFactorForRandomization| stop trying
// to randomize region allocation.
const K_MAX_LOAD_FACTOR_FOR_RANDOMIZATION: f64 = 0.40;

// Max number of attempts to allocate page at random address.
const K_MAX_RANDOMIZATION_ATTEMPTS: i32 = 3;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum RegionState {
    Free,
    Excluded,
    Allocated,
}

impl fmt::Display for RegionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegionState::Free => write!(f, "free"),
            RegionState::Excluded => write!(f, "excluded"),
            RegionState::Allocated => write!(f, "used"),
        }
    }
}

pub type Address = usize; // Assuming Address is a usize

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Region {
    begin: Address,
    size: usize,
    state: RegionState,
}

impl Region {
    pub fn new(begin: Address, size: usize, state: RegionState) -> Self {
        Region { begin, size, state }
    }

    pub fn begin(&self) -> Address {
        self.begin
    }

    pub fn end(&self) -> Address {
        self.begin + self.size
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn set_size(&mut self, size: usize) {
        self.size = size;
    }

    pub fn state(&self) -> RegionState {
        self.state
    }

    pub fn set_state(&mut self, state: RegionState) {
        self.state = state;
    }

    pub fn is_free(&self) -> bool {
        self.state == RegionState::Free
    }

    pub fn is_excluded(&self) -> bool {
        self.state == RegionState::Excluded
    }

    pub fn is_allocated(&self) -> bool {
        self.state == RegionState::Allocated
    }

    pub fn contains(&self, address: Address) -> bool {
        address >= self.begin() && address < self.end()
    }

    pub fn contains_range(&self, address: Address, size: usize) -> bool {
        address >= self.begin() && (address + size) <= self.end()
    }

    pub fn print(&self, os: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            os,
            "[{:#x}, {:#x}), size: {}, {}",
            self.begin(),
            self.end(),
            self.size(),
            self.state()
        )
    }
}

impl Ord for Region {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.end().cmp(&other.end())
    }
}

impl PartialOrd for Region {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub type OnSplitCallback = Option<Box<dyn Fn(Address, usize)>>;
pub type OnMergeCallback = Option<Box<dyn Fn(Address, usize)>>;

pub struct RegionAllocator {
    whole_region_: Region,
    region_size_in_pages_: usize,
    max_load_for_randomization_: usize,
    free_size_: usize,
    page_size_: usize,
    all_regions_: BTreeSet<NonNull<Region>>,
    free_regions_: BTreeSet<NonNull<Region>>,
    on_split_: OnSplitCallback,
    on_merge_: OnMergeCallback,
}

// Helper function to create NonNull<Region> from a Region
fn region_to_nonnull(region: Region) -> NonNull<Region> {
    unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(region))) }
}

// Helper function to extract a Region from NonNull<Region> safely
fn region_from_nonnull(ptr: NonNull<Region>) -> Box<Region> {
    unsafe { Box::from_raw(ptr.as_ptr()) }
}

impl RegionAllocator {
    pub fn new(memory_region_begin: Address, memory_region_size: usize, page_size: usize) -> Self {
        assert!(memory_region_begin < memory_region_begin + memory_region_size);
        //assert!(bits::is_power_of_two(page_size)); // Assuming is_power_of_two functionality
        assert!(memory_region_size % page_size == 0); //IsAligned
        assert!(memory_region_begin % page_size == 0); //IsAligned

        let whole_region_ = Region::new(
            memory_region_begin,
            memory_region_size,
            RegionState::Free,
        );
        let region_size_in_pages_ = memory_region_size / page_size;
        let max_load_for_randomization_ =
            (memory_region_size as f64 * K_MAX_LOAD_FACTOR_FOR_RANDOMIZATION) as usize;
        let mut all_regions_: BTreeSet<NonNull<Region>> = BTreeSet::new();
        let mut free_regions_: BTreeSet<NonNull<Region>> = BTreeSet::new();

        let region = region_to_nonnull(whole_region_);

        all_regions_.insert(region);

        let mut allocator = RegionAllocator {
            whole_region_: Region::new(
                memory_region_begin,
                memory_region_size,
                RegionState::Free,
            ),
            region_size_in_pages_: region_size_in_pages_,
            max_load_for_randomization_: max_load_for_randomization_,
            free_size_: 0,
            page_size_: page_size,
            all_regions_: all_regions_,
            free_regions_: free_regions_,
            on_split_: None,
            on_merge_: None,
        };

        allocator.FreeListAddRegion(unsafe { region.as_ref() });

        allocator
    }

    pub fn begin(&self) -> Address {
        self.whole_region_.begin()
    }

    pub fn end(&self) -> Address {
        self.whole_region_.end()
    }

    pub fn size(&self) -> usize {
        self.whole_region_.size()
    }

    pub fn page_size(&self) -> usize {
        self.page_size_
    }

    pub fn free_size(&self) -> usize {
        self.free_size_
    }

    fn find_region(&self, address: Address) -> Option<NonNull<Region>> {
        if !self.whole_region_.contains(address) {
            return None;
        }

        let key = Region::new(address, 0, RegionState::Free);
        let mut iter = self.all_regions_.range(key..); // Use range instead of upper_bound

        // Find the first region that *ends* after the address
        while let Some(&region_ptr) = iter.next() {
            let region = unsafe { region_ptr.as_ref() };
            if region.contains(address) {
                return Some(region_ptr);
            }
        }

        None
    }

    fn FreeListAddRegion(&mut self, region: &Region) {
        self.free_size_ += region.size();
        self.free_regions_.insert(unsafe {NonNull::new_unchecked(region as *const Region as *mut Region)});
    }

    fn FreeListFindRegion(&self, size: usize) -> Option<NonNull<Region>> {
        let key = Region::new(0, size, RegionState::Free);
        let mut iter = self.free_regions_.range(key..);

        if let Some(&region_ptr) = iter.next() {
            let region = unsafe{region_ptr.as_ref()};
            if region.size() >= size {
                return Some(region_ptr);
            }
        }
        None
    }

    fn FreeListRemoveRegion(&mut self, region: &Region) {
        assert!(region.is_free());

        // Create a NonNull<Region> from the raw pointer of the region
        let region_ptr = unsafe { NonNull::new_unchecked(region as *const Region as *mut Region) };

        if let Some(iter) = self.free_regions_.take(&region_ptr) {
            let region = unsafe { iter.as_ref() };
            assert!(region.size() <= self.free_size_);
            self.free_size_ -= region.size();
        }
    }

    fn Split(&mut self, region_ptr: NonNull<Region>, new_size: usize) -> NonNull<Region> {
        let region = unsafe { region_ptr.as_mut() };
        assert!(self.IsAligned(new_size, self.page_size_));
        assert_ne!(new_size, 0);
        assert!(region.size() > new_size);

        if let Some(ref on_split) = self.on_split_ {
            on_split(region.begin(), new_size);
        }

        assert!(!region.is_excluded());
        let state = region.state();
        let new_region_begin = region.begin() + new_size;
        let new_region_size = region.size() - new_size;
        let new_region = Region::new(new_region_begin, new_region_size, state);
        let new_region_ptr = region_to_nonnull(new_region);

        if state == RegionState::Free {
            self.FreeListRemoveRegion(region);
        }
        region.set_size(new_size);

        self.all_regions_.insert(new_region_ptr);

        if state == RegionState::Free {
            self.FreeListAddRegion(region);
            self.FreeListAddRegion(unsafe {new_region_ptr.as_ref()});
        }

        region_ptr
    }

    fn Merge(&mut self, prev_iter: NonNull<Region>, next_iter: NonNull<Region>) {
        let prev = unsafe { prev_iter.as_mut() };
        let next = unsafe { next_iter.as_mut() };
        assert_eq!(prev.end(), next.begin());

        if let Some(ref on_merge) = self.on_merge_ {
            on_merge(prev.begin(), prev.size() + next.size());
        }

        prev.set_size(prev.size() + next.size());
        self.all_regions_.remove(&next_iter);

        assert_eq!(self.free_regions_.get(&next_iter), None);
    }

    pub fn AllocateRegion(&mut self, size: usize) -> Address {
        assert_ne!(size, 0);
        assert!(self.IsAligned(size, self.page_size_));

        let region_ptr = self.FreeListFindRegion(size);

        let region_ptr = match region_ptr {
            Some(ptr) => ptr,
            None => return 0, //kAllocationFailure, assuming 0 represents failure
        };

        let region = unsafe{region_ptr.as_mut()};

        if region.size() != size {
            self.Split(region_ptr, size);
        }
        assert!(self.IsAligned(region.begin(), self.page_size_));
        assert_eq!(region.size(), size);

        self.FreeListRemoveRegion(region);
        region.set_state(RegionState::Allocated);
        region.begin()
    }

    pub fn AllocateRegionRng(
        &mut self,
        rng: &mut dyn RandomNumberGenerator,
        size: usize,
    ) -> Address {
        if self.free_size() >= self.max_load_for_randomization_ {
            let mut random = 0u64;

            for _i in 0..K_MAX_RANDOMIZATION_ATTEMPTS {
                let random_bytes = rng.next_bytes(8);
                random = u64::from_ne_bytes(random_bytes.try_into().unwrap());
                let random_offset = self.page_size_ * (random as usize % self.region_size_in_pages_);
                let address = self.begin() + random_offset;
                if self.AllocateRegionAt(address, size, RegionState::Allocated) {
                    return address;
                }
            }
        }
        self.AllocateRegion(size)
    }

    pub fn AllocateRegionAt(
        &mut self,
        requested_address: Address,
        size: usize,
        region_state: RegionState,
    ) -> bool {
        assert!(self.IsAligned(requested_address, self.page_size_));
        assert_ne!(size, 0);
        assert!(self.IsAligned(size, self.page_size_));
        assert_ne!(region_state, RegionState::Free);

        let requested_end = requested_address + size;
        assert!(requested_end <= self.end());

        let region_ptr = self.find_region(requested_address);

        let region_ptr = match region_ptr {
            Some(ptr) => ptr,
            None => return false,
        };

        let region = unsafe { region_ptr.as_ref() };

        if !region.is_free() || region.end() < requested_end {
            return false;
        }

        let region = unsafe { region_ptr.as_mut() };
        if region.begin() != requested_address {
            let new_size = requested_address - region.begin();
            assert!(self.IsAligned(new_size, self.page_size_));
            self.Split(region_ptr, new_size);
        }

        let region = unsafe { region_ptr.as_mut() };
        if region.end() != requested_end {
            self.Split(region_ptr, size);
        }

        let region = unsafe { region_ptr.as_mut() };

        assert_eq!(region.begin(), requested_address);
        assert_eq!(region.size(), size);

        self.FreeListRemoveRegion(region);
        region.set_state(region_state);
        true
    }

    pub fn AllocateAlignedRegion(&mut self, size: usize, alignment: usize) -> Address {
        assert!(self.IsAligned(size, self.page_size_));
        assert!(self.IsAligned(alignment, self.page_size_));
        assert!(alignment >= self.page_size_);

        let padded_size = size + alignment - self.page_size_;
        let region_ptr = self.FreeListFindRegion(padded_size);

        let region_ptr = match region_ptr {
            Some(ptr) => ptr,
            None => {
                let region_ptr = self.FreeListFindRegion(size);
                match region_ptr{
                    Some(ptr) => ptr,
                    None => return 0
                }
            }
        };

        let region = unsafe{region_ptr.as_mut()};

        if !self.IsAligned(region.begin(), alignment) {
            let start = self.RoundUp(region.begin(), alignment);
            if start + size > region.end() {
                return 0;
            }
            self.Split(region_ptr, start - region.begin());
        }

        let region = unsafe{region_ptr.as_mut()};
        assert!(self.IsAligned(region.begin(), alignment));

        if region.size() != size {
            self.Split(region_ptr, size);
        }

        let region = unsafe{region_ptr.as_mut()};
        assert!(self.IsAligned(region.begin(), alignment));
        assert_eq!(region.size(), size);

        self.FreeListRemoveRegion(region);
        region.set_state(RegionState::Allocated);
        region.begin()
    }

    pub fn AllocateRegionHint(
        &mut self,
        hint: Address,
        size: usize,
        alignment: usize,
    ) -> Address {
        assert!(self.IsAligned(alignment, self.page_size()));
        assert!(self.IsAligned(hint, alignment));

        if hint != 0 && self.contains(hint, size) {
            if self.AllocateRegionAt(hint, size, RegionState::Allocated) {
                return hint;
            }
        }

        let address = if alignment <= self.page_size() {
            self.AllocateRegion(size)
        } else {
            self.AllocateAlignedRegion(size, alignment)
        };

        address
    }

    pub fn TrimRegion(&mut self, address: Address, new_size: usize) -> usize {
        assert!(self.IsAligned(new_size, self.page_size_));

        let region_ptr = self.find_region(address);

        let region_ptr = match region_ptr {
            Some(ptr) => ptr,
            None => return 0,
        };

        let region = unsafe { region_ptr.as_mut() };

        if region.begin() != address || !region.is_allocated() {
            return 0;
        }

        assert_eq!(self.free_regions_.get(&region_ptr), None);

        let region = unsafe{region_ptr.as_mut()};
        let mut region_iter = region_ptr;

        if new_size > 0 {
            self.Split(region_ptr, new_size);
            region_iter = unsafe {
              let mut regions = self.all_regions_.iter();
              while let Some(&r) = regions.next(){
                if unsafe {r.as_ref().begin()} == unsafe {region_ptr.as_ref().end()} {
                  break;
                }
              }
              let reg = regions.next().unwrap();
              *reg
            }; // Increment the iterator
        }

        let region = unsafe { region_iter.as_mut() };

        let size = region.size();
        region.set_state(RegionState::Free);

        if region.end() != self.whole_region_.end() {
            let mut next_iter_found = false;
            let mut next_iter:NonNull<Region> = NonNull::dangling();

            let mut regions = self.all_regions_.iter();
            while let Some(&r) = regions.next(){
              if unsafe {r.as_ref().begin()} == unsafe {region_iter.as_ref().end()} {
                next_iter_found = true;
                next_iter = *regions.next().unwrap();
                break;
              }
            }

            if next_iter_found{
              let next = unsafe {next_iter.as_ref()};
              if next.is_free() {
                  self.FreeListRemoveRegion(next);
                  self.Merge(region_iter, next_iter);
              }
            }
        }

        if new_size == 0 && region.begin() != self.whole_region_.begin() {
            let mut prev_iter_found = false;
            let mut prev_iter:NonNull<Region> = NonNull::dangling();

            let mut regions = self.all_regions_.iter().rev();
            while let Some(&r) = regions.next(){
              if unsafe {r.as_ref().end()} == unsafe {region_iter.as_ref().begin()} {
                prev_iter_found = true;
                prev_iter = *regions.next().unwrap();
                break;
              }
            }

            if prev_iter_found{
              let prev = unsafe {prev_iter.as_ref()};
              if prev.is_free() {
                  self.FreeListRemoveRegion(prev);
                  self.Merge(prev_iter, region_iter);
                  region_iter = prev_iter;
              }
            }
        }
        let region = unsafe{region_iter.as_mut()};

        self.FreeListAddRegion(region);
        size
    }

    pub fn CheckRegion(&self, address: Address) -> usize {
        let region_ptr = self.find_region(address);

        let region_ptr = match region_ptr {
            Some(ptr) => ptr,
            None => return 0,
        };

        let region = unsafe { region_ptr.as_ref() };

        if region.begin() != address || region.is_free() {
            return 0;
        }
        region.size()
    }

    pub fn IsFree(&self, address: Address, size: usize) -> bool {
        assert!(self.contains(address, size));
        let region_ptr = self.find_region(address);

        let region_ptr = match region_ptr {
            Some(ptr) => ptr,
            None => return true,
        };

        let region = unsafe { region_ptr.as_ref() };
        region.is_free() && region.contains_range(address, size)
    }

    pub fn contains(&self, address: Address, size: usize) -> bool {
        self.whole_region_.contains_range(address, size)
    }

    pub fn set_on_split_callback<F>(&mut self, callback: F)
        where F: Fn(Address, usize) + 'static {
        self.on_split_ = Some(Box::new(callback));
    }

    pub fn set_on_merge_callback<F>(&mut self, callback: F)
        where F: Fn(Address, usize) + 'static {
        self.on_merge_ = Some(Box::new(callback));
    }

    fn IsAligned(&self, value: usize, alignment: usize) -> bool {
        value & (alignment - 1) == 0
    }

    fn RoundUp(&self, value: Address, alignment: usize) -> Address {
        (value + alignment - 1) & !(alignment - 1)
    }
}

impl Drop for RegionAllocator {
    fn drop(&mut self) {
        for region_ptr in &self.all_regions_ {
            let _region = region_from_nonnull(*region_ptr); //Deallocates the box
        }
    }
}

impl fmt::Display for RegionAllocator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "RegionAllocator: [{:#x}, {:#x})",
            self.begin(),
            self.end()
        )?;
        writeln!(f, "size: {}", self.size())?;
        writeln!(f, "free_size: {}", self.free_size())?;
        writeln!(f, "page_size: {}", self.page_size())?;

        writeln!(f, "all regions:")?;
        for region_ptr in &self.all_regions_ {
            let region = unsafe { region_ptr.as_ref() };
            write!(f, "  ")?;
            region.print(f)?;
            writeln!(f)?;
        }

        writeln!(f, "free regions:")?;
        for region_ptr in &self.free_regions_ {
            let region = unsafe { region_ptr.as_ref() };
            write!(f, "  ")?;
            region.print(f)?;
            writeln!(f)?;
        }
        Ok(())
    }
}

pub trait RandomNumberGenerator {
    fn next_bytes(&mut self, num_bytes: usize) -> Vec<u8>;
}

pub const K_ALLOCATION_FAILURE: Address = 0;