// Converted from V8 C++ source files:
// Header: page-memory.h
// Implementation: page-memory.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod page_memory {
    use std::collections::HashMap;
    use std::collections::BTreeMap;
    use std::sync::Mutex;
    use std::mem::size_of;

    pub struct MemoryRegion {
        base_: usize,
        size_: usize,
    }

    impl MemoryRegion {
        pub fn new() -> MemoryRegion {
            MemoryRegion {
                base_: 0,
                size_: 0,
            }
        }

        pub fn with_params(base: usize, size: usize) -> MemoryRegion {
            if base == 0 || size == 0 {
                panic!("Base and size must be non-zero");
            }
            MemoryRegion {
                base_: base,
                size_: size,
            }
        }

        pub fn base(&self) -> usize {
            self.base_
        }

        pub fn size(&self) -> usize {
            self.size_
        }

        pub fn end(&self) -> usize {
            self.base_ + self.size_
        }

        pub fn contains(&self, addr: usize) -> bool {
            (addr - self.base_) < self.size_
        }

        pub fn contains_other(&self, other: &MemoryRegion) -> bool {
            self.base_ <= other.base() && other.end() <= self.end()
        }
    }

    pub struct PageAllocator {}
    impl PageAllocator {
        pub fn new() -> PageAllocator {
            PageAllocator {}
        }
        pub fn CommitPageSize(&self) -> usize {
            4096 // Default page size
        }
        pub fn SetPermissions(&self, _base: usize, _size: usize, _permission: i32) -> bool {
            true
        }
         pub fn AllocatePages(&self, _hint: *mut usize, size: usize, _alignment: usize, _permission: i32) -> *mut usize {
             let layout = std::alloc::Layout::from_size_align(size, 1).unwrap();
             unsafe { std::alloc::alloc(layout) as *mut usize }
         }
         pub fn FreePages(&self, ptr: usize, size: usize) {
             let layout = std::alloc::Layout::from_size_align(size, 1).unwrap();
             unsafe { std::alloc::dealloc(ptr as *mut u8, layout) }
         }
        pub fn AllocatePageSize(&self) -> usize {
            4096 // Default page size
        }
        pub fn DiscardSystemPages(&self, _base: usize, _size: usize) -> bool {
            true
        }
        pub fn RecommitPages(&self, _base: *mut usize, _size: usize, _permission: i32) -> bool{
            true
        }
        pub fn DecommitPages(&self, _base: *mut usize, _size: usize) -> bool {
            true
        }
    }
    pub mod Permission {
        pub const kReadWrite: i32 = 0;
        pub const kNoAccess: i32 = 1;
    }

    pub struct PageMemoryRegion<'a> {
        allocator_: &'a PageAllocator,
        reserved_region_: MemoryRegion,
    }

    impl<'a> PageMemoryRegion<'a> {
        pub fn new(allocator: &'a PageAllocator, reserved_region: MemoryRegion) -> PageMemoryRegion<'a> {
            PageMemoryRegion {
                allocator_: allocator,
                reserved_region_: reserved_region,
            }
        }

        pub fn region(&self) -> &MemoryRegion {
            &self.reserved_region_
        }

        pub fn lookup(&self, address: usize) -> Option<usize> {
            if self.region().contains(address) {
                Some(self.region().base())
            } else {
                None
            }
        }

        pub fn allocator(&self) -> &PageAllocator {
            self.allocator_
        }

        pub fn unprotect_for_testing(&self) {
            let _ = self.allocator_.SetPermissions(self.region().base(), self.region().size(), Permission::kReadWrite);
        }
    }

    impl Drop for PageMemoryRegion<'_> {
        fn drop(&mut self) {
             self.allocator_.FreePages(self.reserved_region_.base_, self.reserved_region_.size_);
        }
    }

    pub struct PageMemoryRegionTree {
        set_: BTreeMap<usize, *mut PageMemoryRegion<'static>>,
    }

    impl PageMemoryRegionTree {
        pub fn new() -> PageMemoryRegionTree {
            PageMemoryRegionTree {
                set_: BTreeMap::new(),
            }
        }

        pub fn add(&mut self, region: *mut PageMemoryRegion<'static>) {
            if region.is_null() {
                panic!("Region must not be null");
            }

            unsafe {
                let base = (*region).region().base();
                self.set_.insert(base, region);
            }
        }

        pub fn remove(&mut self, region: *mut PageMemoryRegion<'static>) {
            if region.is_null() {
                panic!("Region must not be null");
            }
            unsafe {
                let base = (*region).region().base();
                self.set_.remove(&base);
            }
        }

        pub fn lookup(&self, address: usize) -> Option<*mut PageMemoryRegion<'static>> {
            let mut iter = self.set_.range(..=address);
            if let Some((&key, &value)) = iter.next_back() {
                unsafe {
                    if address < (*value).region().end() {
                        return Some(value);
                    }
                }
            }
            None
        }
    }

    pub struct NormalPageMemoryPool {
        pool_: Vec<PooledPageMemoryRegion>,
        decommit_pooled_pages_: bool,
    }

    struct PooledPageMemoryRegion {
        region: *mut PageMemoryRegion<'static>,
        is_decommitted: bool,
        is_discarded: bool,
    }

    impl NormalPageMemoryPool {
        pub const kDefaultDecommitPooledPage: bool = false;

        pub fn new() -> NormalPageMemoryPool {
            NormalPageMemoryPool {
                pool_: Vec::new(),
                decommit_pooled_pages_: Self::kDefaultDecommitPooledPage,
            }
        }

        pub fn add(&mut self, pmr: *mut PageMemoryRegion<'static>) {
            if pmr.is_null() {
                panic!("pmr must not be null");
            }

            unsafe {
                if (*pmr).region().size() != 4096 {
                    panic!("Size must be equal to kPageSize");
                }
                let base = (*pmr).region().base() as *mut u8;
                let size = (*pmr).region().size();

                std::ptr::write_bytes(base, 0, size);

                self.pool_.push(PooledPageMemoryRegion {
                    region: pmr,
                    is_decommitted: false,
                    is_discarded: false,
                });
            }
        }

        pub fn take(&mut self) -> Option<*mut PageMemoryRegion<'static>> {
            if self.pool_.is_empty() {
                return None;
            }

            let entry = self.pool_.pop().unwrap();
            let pmr = entry.region;
            unsafe {
                if !pmr.is_null() {
                    let base = (*pmr).region().base() as *mut u8;
                    let size = (*pmr).region().size();
                   std::arch::asm!("", options(nostack));
                }

                if !self.decommit_pooled_pages_ && entry.is_decommitted {
                     (*pmr).allocator().RecommitPages((*pmr).region().base() as *mut usize, (*pmr).region().size(), Permission::kReadWrite);
                     (*pmr).allocator().SetPermissions((*pmr).region().base(), (*pmr).region().size(), Permission::kReadWrite);
                }
            }
            Some(pmr)
        }

        pub fn pooled(&self) -> usize {
            self.pool_.len()
        }

        pub fn pooled_memory(&self) -> usize {
            let mut total_size = 0;
            for entry in &self.pool_ {
                if entry.is_decommitted || entry.is_discarded {
                    continue;
                }
                unsafe {
                    total_size += (*entry.region).region().size();
                }
            }
            total_size
        }

        pub fn release_pooled_pages(&mut self, page_allocator: &PageAllocator) {
            for entry in &mut self.pool_ {
                unsafe {
                    let base = (*entry.region).region().base() as *mut usize;
                    let size = (*entry.region).region().size();
                    if self.decommit_pooled_pages_ {
                        if !entry.is_decommitted {
                           page_allocator.DecommitPages(base, size);
                            entry.is_decommitted = true;
                        }
                    } else {
                        if !entry.is_discarded {
                            let region = (*entry.region).region();
                            page_allocator.DiscardSystemPages(region.base(), region.size());
                            entry.is_discarded = true;
                        }
                    }
                }
            }
        }

        pub fn get_raw_pool_for_testing(&mut self) -> &mut Vec<PooledPageMemoryRegion> {
            &mut self.pool_
        }

        pub fn set_decommit_pooled_pages(&mut self, value: bool) {
            self.decommit_pooled_pages_ = value;
        }
    }

    pub struct PageBackend<'a> {
        mutex_: Mutex<()>,
        normal_page_allocator_: &'a PageAllocator,
        large_page_allocator_: &'a PageAllocator,
        page_pool_: NormalPageMemoryPool,
        page_memory_region_tree_: PageMemoryRegionTree,
        normal_page_memory_regions_: HashMap<*mut PageMemoryRegion<'static>, Box<PageMemoryRegion<'static>>>,
        large_page_memory_regions_: HashMap<*mut PageMemoryRegion<'static>, Box<PageMemoryRegion<'static>>>,
    }

    impl<'a> PageBackend<'a> {
        pub fn new(normal_page_allocator: &'a PageAllocator, large_page_allocator: &'a PageAllocator) -> PageBackend<'a> {
            PageBackend {
                mutex_: Mutex::new(()),
                normal_page_allocator_: normal_page_allocator,
                large_page_allocator_: large_page_allocator,
                page_pool_: NormalPageMemoryPool::new(),
                page_memory_region_tree_: PageMemoryRegionTree::new(),
                normal_page_memory_regions_: HashMap::new(),
                large_page_memory_regions_: HashMap::new(),
            }
        }

        pub fn try_allocate_normal_page_memory(&mut self) -> Option<usize> {
            let _guard = self.mutex_.lock().unwrap();
            if let Some(cached) = self.page_pool_.take() {
                unsafe {
                    let region = (*cached).region();
                    self.page_memory_region_tree_.add(cached);
                    return Some(region.base());
                }
            }

            let mut pmr = Box::new(PageMemoryRegion::new(self.normal_page_allocator_,
                                                          MemoryRegion::with_params(0, 0)));
            let region_memory = self.normal_page_allocator_.AllocatePages(
                std::ptr::null_mut(),
                4096, // kPageSize
                4096, // kPageSize
                Permission::kNoAccess
            );

            if region_memory.is_null() {
                return None;
            }

            let memory_region = MemoryRegion::with_params(region_memory as usize, 4096);

            *pmr = PageMemoryRegion::new(self.normal_page_allocator_, memory_region);

            if self.normal_page_allocator_.SetPermissions(pmr.region().base(), pmr.region().size(), Permission::kReadWrite) {
                let pmr_ptr = Box::into_raw(pmr);
                self.page_memory_region_tree_.add(pmr_ptr);
                self.normal_page_memory_regions_.insert(pmr_ptr, unsafe { Box::from_raw(pmr_ptr) });
                unsafe { Some((*pmr_ptr).region().base()) }
            } else {
                None
            }
        }

        pub fn free_normal_page_memory(&mut self, writeable_base: usize) {
            let _guard = self.mutex_.lock().unwrap();
            let pmr = self.page_memory_region_tree_.lookup(writeable_base).unwrap();
            self.page_memory_region_tree_.remove(pmr);

            self.page_pool_.add(pmr);
        }

        pub fn try_allocate_large_page_memory(&mut self, size: usize) -> Option<usize> {
            let _guard = self.mutex_.lock().unwrap();

            let mut pmr = Box::new(PageMemoryRegion::new(self.large_page_allocator_, MemoryRegion::with_params(0, 0)));

            let region_memory = self.large_page_allocator_.AllocatePages(
                std::ptr::null_mut(),
                size,
                4096, // kPageSize
                Permission::kNoAccess
            );

            if region_memory.is_null() {
                return None;
            }

            let memory_region = MemoryRegion::with_params(region_memory as usize, size);
            *pmr = PageMemoryRegion::new(self.large_page_allocator_, memory_region);

            if self.large_page_allocator_.SetPermissions(pmr.region().base(), pmr.region().size(), Permission::kReadWrite) {
                let pmr_ptr = Box::into_raw(pmr);
                self.page_memory_region_tree_.add(pmr_ptr);
                self.large_page_memory_regions_.insert(pmr_ptr, unsafe {Box::from_raw(pmr_ptr)});
                unsafe { Some((*pmr_ptr).region().base()) }
            } else {
                None
            }
        }

        pub fn free_large_page_memory(&mut self, writeable_base: usize) {
            let _guard = self.mutex_.lock().unwrap();
            let pmr = self.page_memory_region_tree_.lookup(writeable_base).unwrap();
            self.page_memory_region_tree_.remove(pmr);

            let _ = self.large_page_memory_regions_.remove(&pmr);
        }

        pub fn lookup(&self, address: usize) -> Option<usize> {
            let _guard = self.mutex_.lock().unwrap();
            match self.page_memory_region_tree_.lookup(address) {
                Some(pmr) => unsafe { (*pmr).lookup(address) },
                None => None,
            }
        }

        pub fn release_pooled_pages(&mut self) {
            self.page_pool_.release_pooled_pages(self.normal_page_allocator_);
        }

        pub fn get_page_memory_region_tree_for_testing(&mut self) -> &mut PageMemoryRegionTree {
            &mut self.page_memory_region_tree_
        }

        pub fn page_pool(&mut self) -> &mut NormalPageMemoryPool {
            &mut self.page_pool_
        }
    }
}
