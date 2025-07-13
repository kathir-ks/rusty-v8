// Converted from V8 C++ source files:
// Header: lsan-page-allocator.h
// Implementation: lsan-page-allocator.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
pub mod sanitizer {
use std::sync::Mutex;
use std::collections::HashSet;
use std::ptr::null_mut;

    pub struct LsanPageAllocator<'a> {
        page_allocator: &'a mut dyn v8::PageAllocator,
        allocate_page_size: usize,
        commit_page_size: usize,
        #[cfg(leak_sanitizer)]
        not_registered_regions_mutex: Mutex<HashSet<*mut std::ffi::c_void>>,
        #[cfg(leak_sanitizer)]
        not_registered_regions: HashSet<*mut std::ffi::c_void>,
    }

    impl<'a> LsanPageAllocator<'a> {
        pub fn new(page_allocator: &'a mut dyn v8::PageAllocator) -> Self {
            let allocate_page_size = page_allocator.allocate_page_size();
            let commit_page_size = page_allocator.commit_page_size();
            LsanPageAllocator {
                page_allocator,
                allocate_page_size,
                commit_page_size,
                #[cfg(leak_sanitizer)]
                not_registered_regions_mutex: Mutex::new(HashSet::new()),
                #[cfg(leak_sanitizer)]
                not_registered_regions: HashSet::new(),
            }
        }

        pub fn allocate_page_size(&self) -> usize {
            self.allocate_page_size
        }

        pub fn commit_page_size(&self) -> usize {
            self.commit_page_size
        }

        pub fn set_random_mmap_seed(&mut self, seed: i64) {
            self.page_allocator.set_random_mmap_seed(seed);
        }

        pub fn get_random_mmap_addr(&mut self) -> *mut std::ffi::c_void {
            self.page_allocator.get_random_mmap_addr()
        }

        pub fn allocate_pages(
            &mut self,
            hint: *mut std::ffi::c_void,
            size: usize,
            alignment: usize,
            access: v8::PageAllocatorPermission,
        ) -> *mut std::ffi::c_void {
            let result = self.page_allocator.allocate_pages(hint, size, alignment, access);

            #[cfg(leak_sanitizer)]
            {
                if !result.is_null() {
                    if access != v8::PageAllocatorPermission::kNoAccessWillJitLater {
                        unsafe {
                            __lsan_register_root_region(result, size);
                        }
                    } else {
                        let mut lock = self.not_registered_regions_mutex.lock().unwrap();
                        if !self.not_registered_regions.contains(&result) {
                            self.not_registered_regions.insert(result);
                        }
                        drop(lock);
                    }
                }
            }
            result
        }

        pub fn allocate_shared_pages(
            &mut self,
            size: usize,
            original_address: *const std::ffi::c_void,
        ) -> Result<Box<dyn v8::PageAllocatorSharedMemory>, String> {
            let result = self.page_allocator.allocate_shared_pages(size, original_address);

            match result {
                Ok(shared_memory) => {
                    #[cfg(leak_sanitizer)]
                    {
                        unsafe {
                            __lsan_register_root_region(shared_memory.get_memory(), size);
                        }
                    }
                    Ok(shared_memory)
                }
                Err(e) => Err(e),
            }
        }

        pub fn can_allocate_shared_pages(&self) -> bool {
            self.page_allocator.can_allocate_shared_pages()
        }

        pub fn free_pages(&mut self, address: *mut std::ffi::c_void, size: usize) -> bool {
            #[cfg(leak_sanitizer)]
            {
                let mut lock = self.not_registered_regions_mutex.lock().unwrap();
                if !self.not_registered_regions.contains(&address) {
                    unsafe {
                        __lsan_unregister_root_region(address, size);
                    }
                } else {
                    self.not_registered_regions.remove(&address);
                }
                drop(lock);
            }

            if self.page_allocator.free_pages(address, size) {
                return true;
            }

            false
        }

        pub fn release_pages(&mut self, address: *mut std::ffi::c_void, size: usize, new_size: usize) -> bool {
            #[cfg(leak_sanitizer)]
            {
                let mut lock = self.not_registered_regions_mutex.lock().unwrap();
                if !self.not_registered_regions.contains(&address) {
                    unsafe {
                        __lsan_unregister_root_region(address, size);
                        __lsan_register_root_region(address, new_size);
                    }
                }
                drop(lock);
            }

            self.page_allocator.release_pages(address, size, new_size)
        }

        pub fn set_permissions(
            &mut self,
            address: *mut std::ffi::c_void,
            size: usize,
            access: v8::PageAllocatorPermission,
        ) -> bool {
            self.page_allocator.set_permissions(address, size, access)
        }

        pub fn recommit_pages(
            &mut self,
            address: *mut std::ffi::c_void,
            size: usize,
            access: v8::PageAllocatorPermission,
        ) -> bool {
            self.page_allocator.recommit_pages(address, size, access)
        }

        pub fn discard_system_pages(&mut self, address: *mut std::ffi::c_void, size: usize) -> bool {
            self.page_allocator.discard_system_pages(address, size)
        }

        pub fn decommit_pages(&mut self, address: *mut std::ffi::c_void, size: usize) -> bool {
            self.page_allocator.decommit_pages(address, size)
        }
    }
}
}

pub mod v8 {
    pub trait PageAllocator {
        fn allocate_page_size(&self) -> usize;
        fn commit_page_size(&self) -> usize;
        fn set_random_mmap_seed(&mut self, seed: i64);
        fn get_random_mmap_addr(&mut self) -> *mut std::ffi::c_void;
        fn allocate_pages(
            &mut self,
            hint: *mut std::ffi::c_void,
            size: usize,
            alignment: usize,
            access: PageAllocatorPermission,
        ) -> *mut std::ffi::c_void;
        fn allocate_shared_pages(
            &mut self,
            size: usize,
            original_address: *const std::ffi::c_void,
        ) -> Result<Box<dyn PageAllocatorSharedMemory>, String>;
        fn can_allocate_shared_pages(&self) -> bool;
        fn free_pages(&mut self, address: *mut std::ffi::c_void, size: usize) -> bool;
        fn release_pages(&mut self, address: *mut std::ffi::c_void, size: usize, new_size: usize) -> bool;
        fn set_permissions(
            &mut self,
            address: *mut std::ffi::c_void,
            size: usize,
            access: PageAllocatorPermission,
        ) -> bool;
        fn recommit_pages(
            &mut self,
            address: *mut std::ffi::c_void,
            size: usize,
            access: PageAllocatorPermission,
        ) -> bool;
        fn discard_system_pages(&mut self, address: *mut std::ffi::c_void, size: usize) -> bool;
        fn decommit_pages(&mut self, address: *mut std::ffi::c_void, size: usize) -> bool;
    }

    pub trait PageAllocatorSharedMemory {
        fn get_memory(&self) -> *mut std::ffi::c_void;
    }

    #[derive(PartialEq, Eq, Debug)]
    pub enum PageAllocatorPermission {
        kNoAccess,
        kReadWrite,
        kReadExecute,
        kReadWriteExecute,
        kNoAccessWillJitLater,
    }
}

#[cfg(leak_sanitizer)]
extern "C" {
    pub fn __lsan_register_root_region(begin: *mut std::ffi::c_void, size: usize);
    pub fn __lsan_unregister_root_region(begin: *mut std::ffi::c_void, size: usize);
}
