// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::cmp::max;
use std::collections::BTreeMap;
use std::mem::MaybeUninit;
use std::ptr::NonNull;
use std::{mem, ptr};

//use crate::base::macros::*; // Assuming these are simple constants/macros
//use crate::base::sanitizer::asan::*; // Assuming ASAN functionality is not crucial for core logic
//use crate::heap::cppgc::memory::*; // Assuming MemoryRegion is defined here
//use crate::heap::cppgc::platform::*; // Assuming PageAllocator trait is defined here

const V8_OS_POSIX: bool = cfg!(target_family = "unix"); // Example, adjust as needed

const DEBUG: bool = cfg!(debug_assertions); // Example

const kPageSize: usize = 4096; // Example page size

trait PageAllocator {
    type Error;
    fn allocate_pages(
        &mut self,
        address: Option<*mut u8>,
        size: usize,
        alignment: usize,
        permissions: Permission,
    ) -> Result<*mut u8, Self::Error>;
    fn free_pages(&mut self, address: *mut u8, size: usize) -> Result<(), Self::Error>;
    fn set_permissions(&mut self, address: *mut u8, size: usize, permissions: Permission) -> Result<(), Self::Error>;
    fn discard_system_pages(&mut self, address: *mut u8, size: usize) -> Result<(), Self::Error>;
    fn recommit_pages(&mut self, address: *mut u8, size: usize, permissions: Permission) -> Result<(), Self::Error>;
    fn allocate_page_size(&self) -> usize;
    fn commit_page_size(&self) -> usize;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Permission {
    kNoAccess,
    kReadWrite,
    // Add other permissions as needed
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct MemoryRegion {
    base: *mut u8,
    size: usize,
}

impl MemoryRegion {
    fn new(base: *mut u8, size: usize) -> Self {
        MemoryRegion { base, size }
    }

    fn base(&self) -> *mut u8 {
        self.base
    }

    fn size(&self) -> usize {
        self.size
    }

    fn end(&self) -> *mut u8 {
        unsafe { self.base.add(self.size) }
    }
}

fn try_unprotect<T: PageAllocator>(
    allocator: &mut T,
    memory_region: &MemoryRegion,
) -> Result<(), T::Error> {
    assert_eq!(0, memory_region.size() % allocator.commit_page_size());
    allocator.set_permissions(
        memory_region.base(),
        memory_region.size(),
        Permission::kReadWrite,
    )
}

fn try_discard<T: PageAllocator>(
    allocator: &mut T,
    memory_region: &MemoryRegion,
) -> Result<(), T::Error> {
    assert_eq!(0, memory_region.size() % allocator.commit_page_size());
    allocator.discard_system_pages(memory_region.base(), memory_region.size())
}

fn reserve_memory_region<T: PageAllocator>(
    allocator: &mut T,
    allocation_size: usize,
) -> Result<MemoryRegion, T::Error> {
    let region_memory = allocator.allocate_pages(
        None,
        allocation_size,
        kPageSize,
        Permission::kNoAccess,
    )?;
    let reserved_region = MemoryRegion::new(region_memory, allocation_size);
    assert_eq!(
        (reserved_region.base as usize) + allocation_size,
        reserved_region.end() as usize
    );
    Ok(reserved_region)
}

fn free_memory_region<T: PageAllocator>(
    allocator: &mut T,
    reserved_region: &MemoryRegion,
) -> Result<(), T::Error> {
    //ASAN_UNPOISON_MEMORY_REGION(reserved_region.base(), reserved_region.size()); // Removed ASAN call
    allocator.free_pages(reserved_region.base(), reserved_region.size())
}

fn round_up(value: usize, alignment: usize) -> usize {
    (value + alignment - 1) / alignment * alignment
}

struct PageMemoryRegion<T: PageAllocator> {
    allocator: T,
    reserved_region: MemoryRegion,
}

impl<T: PageAllocator> PageMemoryRegion<T> {
    fn new(allocator: T, reserved_region: MemoryRegion) -> Self {
        PageMemoryRegion {
            allocator,
            reserved_region,
        }
    }

    fn region(&self) -> MemoryRegion {
        self.reserved_region
    }

    fn allocator(&self) -> &T {
        &self.allocator
    }

    fn unprotect_for_testing(&mut self) -> Result<(), T::Error> {
        try_unprotect(&mut self.allocator, &self.region())
    }
}

impl<T: PageAllocator> Drop for PageMemoryRegion<T> {
    fn drop(&mut self) {
        let _ = free_memory_region(&mut self.allocator, &self.region()); //ignore the error
    }
}

struct PageMemoryRegionTree<'a, T: PageAllocator> {
    set: BTreeMap<*mut u8, &'a PageMemoryRegion<T>>,
}

impl<'a, T: PageAllocator> PageMemoryRegionTree<'a, T> {
    fn new() -> Self {
        PageMemoryRegionTree {
            set: BTreeMap::new(),
        }
    }

    fn add(&mut self, region: &'a PageMemoryRegion<T>) {
        let base = region.region().base();
        let result = self.set.insert(base, region);
        assert!(result.is_none());
    }

    fn remove(&mut self, region: &'a PageMemoryRegion<T>) {
        let base = region.region().base();
        let size = self.set.remove(&base);
        assert!(size.is_some());
    }

    fn lookup(&self, base: *mut u8) -> Option<&&PageMemoryRegion<T>> {
        self.set.get(&base)
    }
}

struct PooledPageMemoryRegion<'a, T: PageAllocator> {
    region: &'a PageMemoryRegion<T>,
    is_decommitted: bool,
    is_discarded: bool,
}

impl<'a, T: PageAllocator> PooledPageMemoryRegion<'a, T> {
    fn new(region: &'a PageMemoryRegion<T>) -> Self {
        PooledPageMemoryRegion {
            region,
            is_decommitted: false,
            is_discarded: false,
        }
    }
}

struct NormalPageMemoryPool<'a, T: PageAllocator> {
    pool_: Vec<PooledPageMemoryRegion<'a, T>>,
    decommit_pooled_pages_: bool, //default false, implement setter/getter if needed
}

impl<'a, T: PageAllocator> NormalPageMemoryPool<'a, T> {
    fn new() -> Self {
        NormalPageMemoryPool {
            pool_: Vec::new(),
            decommit_pooled_pages_: false,
        }
    }

    fn add(&mut self, pmr: &'a PageMemoryRegion<T>) {
        assert_eq!(pmr.region().size(), kPageSize);
        // Oilpan requires the pages to be zero-initialized.
        {
            let base = pmr.region().base();
            let size = pmr.region().size();
            //AsanUnpoisonScope unpoison_for_memset(base, size); // Removed ASAN call
            unsafe {
                ptr::write_bytes(base, 0, size);
            }
        }
        self.pool_.push(PooledPageMemoryRegion::new(pmr));
    }

    fn take(&mut self) -> Option<&'a PageMemoryRegion<T>> {
        if self.pool_.is_empty() {
            return None;
        }
        let entry = self.pool_.pop().unwrap();
        let base = entry.region.region().base();
        let size = entry.region.region().size();
        //ASAN_UNPOISON_MEMORY_REGION(base, size); // Removed ASAN call

        assert!(!self.decommit_pooled_pages_ || !entry.is_decommitted);
        if entry.is_decommitted {
            let allocator = entry.region.allocator();
            unsafe {
                let _ = allocator.recommit_pages(base, size, Permission::kReadWrite);
            }
            let allocator = entry.region.allocator();
            unsafe {
                let ok = allocator.set_permissions(base, size, Permission::kReadWrite);
                if ok.is_err() {
                   //GetGlobalOOMHandler()("Cannot change page permissions"); // Removed OOM handler
                    assert!(false);
                }
            }
        }
        if DEBUG {
            unsafe {
                check_memory_is_zero(base, size);
            }
        }
        Some(entry.region)
    }

    fn pooled_memory(&self) -> usize {
        let mut total_size = 0;
        for entry in &self.pool_ {
            if entry.is_decommitted || entry.is_discarded {
                continue;
            }
            total_size += entry.region.region().size();
        }
        total_size
    }

    fn release_pooled_pages<Allocator: PageAllocator>(&mut self, page_allocator: &mut Allocator) -> Result<(), Allocator::Error> {
        for entry in &mut self.pool_ {
            let base = entry.region.region().base();
            let size = entry.region.region().size();
            // Unpoison the memory before giving back to the OS.
            //ASAN_UNPOISON_MEMORY_REGION(base, size); // Removed ASAN call
            if self.decommit_pooled_pages_ {
                if entry.is_decommitted {
                    continue;
                }
                page_allocator.decommit_pages(base, size)?;
                entry.is_decommitted = true;
            } else {
                if entry.is_discarded {
                    continue;
                }
                try_discard(page_allocator, &entry.region.region())?;
                entry.is_discarded = true;
            }
        }
        Ok(())
    }
}

unsafe fn check_memory_is_zero(base: *mut u8, size: usize) {
    for i in 0..size {
        assert_eq!(*base.add(i), 0);
    }
}

struct PageBackend<'a, NormalAllocator: PageAllocator, LargeAllocator: PageAllocator> {
    normal_page_allocator: &'a mut NormalAllocator,
    large_page_allocator: &'a mut LargeAllocator,
    page_pool: NormalPageMemoryPool<'a, NormalAllocator>,
    page_memory_region_tree: PageMemoryRegionTree<'a, NormalAllocator>,
    normal_page_memory_regions: BTreeMap<*const PageMemoryRegion<NormalAllocator>, Box<PageMemoryRegion<NormalAllocator>>>,
    large_page_memory_regions: BTreeMap<*const PageMemoryRegion<LargeAllocator>, Box<PageMemoryRegion<LargeAllocator>>>,
    mutex: std::sync::Mutex<()>, //Use a simple mutex, replace if needed
}

impl<'a, NormalAllocator: PageAllocator, LargeAllocator: PageAllocator>
    PageBackend<'a, NormalAllocator, LargeAllocator>
{
    fn new(
        normal_page_allocator: &'a mut NormalAllocator,
        large_page_allocator: &'a mut LargeAllocator,
    ) -> Self {
        PageBackend {
            normal_page_allocator,
            large_page_allocator,
            page_pool: NormalPageMemoryPool::new(),
            page_memory_region_tree: PageMemoryRegionTree::new(),
            normal_page_memory_regions: BTreeMap::new(),
            large_page_memory_regions: BTreeMap::new(),
            mutex: std::sync::Mutex::new(()),
        }
    }

    fn try_allocate_normal_page_memory(&mut self) -> Result<*mut u8, NormalAllocator::Error> {
        let _guard = self.mutex.lock().unwrap();
        if let Some(cached) = self.page_pool.take() {
            let region = cached.region();
            let end = self.normal_page_memory_regions.get_key_value(&cached);
            assert!(end.is_some());
            self.page_memory_region_tree.add(cached);
            return Ok(region.base());
        }

        let mut pmr = reserve_memory_region(self.normal_page_allocator, kPageSize).map(|region| PageMemoryRegion::new(self.normal_page_allocator, region))?;
        
        let memory_region = pmr.region();
        if try_unprotect(self.normal_page_allocator, &memory_region).is_ok() {
            let ptr = Box::new(pmr);
            let raw_ptr: *const PageMemoryRegion<NormalAllocator> = &*ptr;
            self.page_memory_region_tree.add(unsafe {&*raw_ptr});
            self.normal_page_memory_regions.insert(raw_ptr, ptr);
            return Ok(memory_region.base());
        }
        //If allocation fails, the `Drop` for `pmr` will be called, releasing the reserved memory.
        Err(self.normal_page_allocator.allocate_pages(None, kPageSize, kPageSize, Permission::kNoAccess).err().unwrap())//TODO: Proper Error handling in case unprotect fails
    }

    fn free_normal_page_memory(&mut self, writeable_base: *mut u8) {
        let _guard = self.mutex.lock().unwrap();
        let pmr_ref = self.page_memory_region_tree.lookup(writeable_base);
        let pmr = pmr_ref.expect("pmr should not be null");

        self.page_memory_region_tree.remove(pmr);
        self.page_pool.add(pmr);
    }

    fn try_allocate_large_page_memory(&mut self, size: usize) -> Result<*mut u8, LargeAllocator::Error> {
        let _guard = self.mutex.lock().unwrap();

        let mut pmr = reserve_memory_region(self.large_page_allocator, round_up(size, self.large_page_allocator.allocate_page_size())).map(|region| PageMemoryRegion::new(self.large_page_allocator, region))?;

        let memory_region = pmr.region();
        if try_unprotect(self.large_page_allocator, &memory_region).is_ok() {
            let ptr = Box::new(pmr);
            let raw_ptr: *const PageMemoryRegion<LargeAllocator> = &*ptr;
            self.page_memory_region_tree.add(unsafe {&*raw_ptr});
            self.large_page_memory_regions.insert(raw_ptr, ptr);
            return Ok(memory_region.base());
        }
        //If allocation fails, the `Drop` for `pmr` will be called, releasing the reserved memory.
        Err(self.large_page_allocator.allocate_pages(None, size, kPageSize, Permission::kNoAccess).err().unwrap()) //TODO: Proper Error handling in case unprotect fails
    }

    fn free_large_page_memory(&mut self, writeable_base: *mut u8) {
        let _guard = self.mutex.lock().unwrap();
        let pmr_ref = self.page_memory_region_tree.lookup(writeable_base);
        let pmr = pmr_ref.expect("pmr should not be null");
        self.page_memory_region_tree.remove(pmr);
        let size = self.large_page_memory_regions.remove(&pmr);
        assert!(size.is_some());
    }

    fn release_pooled_pages(&mut self) -> Result<(), NormalAllocator::Error> {
        self.page_pool.release_pooled_pages(self.normal_page_allocator)
    }
}