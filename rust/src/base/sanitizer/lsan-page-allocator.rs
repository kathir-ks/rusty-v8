// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//use std::sync::{Mutex, MutexGuard};
//use std::collections::HashSet;

use std::sync::Mutex;

pub mod base {
    use std::sync::MutexGuard;
    use std::{collections::HashSet, ptr::NonNull, sync::Arc};

    use v8::SharedMemory;
    use v8::PageAllocator;

    pub struct LsanPageAllocator {
        page_allocator: Box<dyn PageAllocator>,
        allocate_page_size: usize,
        commit_page_size: usize,
        #[cfg(leak_sanitizer)]
        not_registered_regions_mutex: Mutex<HashSet<*mut std::ffi::c_void>>,
        #[cfg(not(leak_sanitizer))]
        not_registered_regions_mutex: Mutex<()>, // Dummy mutex when leak sanitizer is disabled
    }

    impl LsanPageAllocator {
        pub fn new(page_allocator: Box<dyn PageAllocator>) -> Self {
            LsanPageAllocator {
                page_allocator,
                allocate_page_size: page_allocator.allocate_page_size(),
                commit_page_size: page_allocator.commit_page_size(),
                #[cfg(leak_sanitizer)]
                not_registered_regions_mutex: Mutex::new(HashSet::new()),
                #[cfg(not(leak_sanitizer))]
                not_registered_regions_mutex: Mutex::new(()),
            }
        }

        pub fn allocate_page_size(&self) -> usize {
            self.allocate_page_size
        }

        pub fn commit_page_size(&self) -> usize {
            self.commit_page_size
        }

        pub fn set_random_mmap_seed(&self, seed: i64) {
            self.page_allocator.set_random_mmap_seed(seed);
        }

        pub fn get_random_mmap_addr(&self) -> *mut std::ffi::c_void {
            self.page_allocator.get_random_mmap_addr()
        }

        pub fn allocate_pages(
            &self,
            address: *mut std::ffi::c_void,
            size: usize,
            alignment: usize,
            access: v8::PageAllocatorPermission,
        ) -> *mut std::ffi::c_void {
            // TODO(https://github.com/rust-lang/rust-bindgen/issues/1670):
            // Use `NonNull::dangling().as_ptr()` instead of `0 as *mut _`

            let ptr = self.page_allocator.allocate_pages(address, size, alignment, access);

            #[cfg(leak_sanitizer)]
            {
                let mut lock = self.not_registered_regions_mutex.lock().unwrap();
                lock.insert(ptr);
            }

            ptr
        }

        pub fn allocate_shared_pages(
            &self,
            size: usize,
            original_address: *const std::ffi::c_void,
        ) -> Option<Arc<dyn SharedMemory>> {
            self.page_allocator.allocate_shared_pages(size, original_address)
        }

        pub fn can_allocate_shared_pages(&self) -> bool {
            self.page_allocator.can_allocate_shared_pages()
        }

        pub fn free_pages(&self, address: *mut std::ffi::c_void, size: usize) -> bool {
            #[cfg(leak_sanitizer)]
            {
                let mut lock = self.not_registered_regions_mutex.lock().unwrap();
                lock.remove(&address);
            }

            self.page_allocator.free_pages(address, size)
        }

        pub fn release_pages(&self, address: *mut std::ffi::c_void, size: usize, new_size: usize) -> bool {
          self.page_allocator.release_pages(address, size, new_size)
        }

        pub fn set_permissions(
            &self,
            address: *mut std::ffi::c_void,
            size: usize,
            access: v8::PageAllocatorPermission,
        ) -> bool {
            self.page_allocator.set_permissions(address, size, access)
        }

        pub fn recommit_pages(
            &self,
            address: *mut std::ffi::c_void,
            size: usize,
            access: v8::PageAllocatorPermission,
        ) -> bool {
            self.page_allocator.recommit_pages(address, size, access)
        }

        pub fn discard_system_pages(&self, address: *mut std::ffi::c_void, size: usize) -> bool {
            self.page_allocator.discard_system_pages(address, size)
        }

        pub fn decommit_pages(&self, address: *mut std::ffi::c_void, size: usize) -> bool {
            self.page_allocator.decommit_pages(address, size)
        }
    }
}

mod v8 {
    pub trait PageAllocator {
        fn allocate_page_size(&self) -> usize;
        fn commit_page_size(&self) -> usize;
        fn set_random_mmap_seed(&self, seed: i64);
        fn get_random_mmap_addr(&self) -> *mut std::ffi::c_void;
        fn allocate_pages(
            &self,
            address: *mut std::ffi::c_void,
            size: usize,
            alignment: usize,
            access: PageAllocatorPermission,
        ) -> *mut std::ffi::c_void;
        fn allocate_shared_pages(
            &self,
            size: usize,
            original_address: *const std::ffi::c_void,
        ) -> Option<std::sync::Arc<dyn SharedMemory>>;
        fn can_allocate_shared_pages(&self) -> bool;
        fn free_pages(&self, address: *mut std::ffi::c_void, size: usize) -> bool;
        fn release_pages(&self, address: *mut std::ffi::c_void, size: usize, new_size: usize) -> bool;
        fn set_permissions(
            &self,
            address: *mut std::ffi::c_void,
            size: usize,
            access: PageAllocatorPermission,
        ) -> bool;
        fn recommit_pages(
            &self,
            address: *mut std::ffi::c_void,
            size: usize,
            access: PageAllocatorPermission,
        ) -> bool;
        fn discard_system_pages(&self, address: *mut std::ffi::c_void, size: usize) -> bool;
        fn decommit_pages(&self, address: *mut std::ffi::c_void, size: usize) -> bool;
    }
    
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum PageAllocatorPermission {
      NoAccess,
      ReadWrite,
      ReadExecute,
      ReadWriteExecute,
    }

    pub trait SharedMemory: Send + Sync {
        fn size(&self) -> usize;
        fn base(&self) -> *mut std::ffi::c_void;
    }
}