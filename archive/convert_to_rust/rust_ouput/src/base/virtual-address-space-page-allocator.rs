// Converted from V8 C++ source files:
// Header: virtual-address-space-page-allocator.h
// Implementation: virtual-address-space-page-allocator.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    use std::collections::HashMap;
    use std::sync::Mutex as StdMutex;

    use crate::sandbox::VirtualAddressSpace;

    pub trait PageAllocator {
        fn allocate_page_size(&self) -> usize;
        fn commit_page_size(&self) -> usize;
        fn set_random_mmap_seed(&mut self, seed: i64);
        fn get_random_mmap_addr(&self) -> *mut std::ffi::c_void;
        fn allocate_pages(
            &mut self,
            hint: *mut std::ffi::c_void,
            size: usize,
            alignment: usize,
            access: Permission,
        ) -> *mut std::ffi::c_void;
        fn free_pages(&mut self, address: *mut std::ffi::c_void, size: usize) -> bool;
        fn release_pages(
            &mut self,
            address: *mut std::ffi::c_void,
            size: usize,
            new_size: usize,
        ) -> bool;
        fn set_permissions(
            &mut self,
            address: *mut std::ffi::c_void,
            size: usize,
            access: Permission,
        ) -> bool;
        fn recommit_pages(
            &mut self,
            address: *mut std::ffi::c_void,
            size: usize,
            access: Permission,
        ) -> bool;
        fn discard_system_pages(&mut self, address: *mut std::ffi::c_void, size: usize) -> bool;
        fn decommit_pages(&mut self, address: *mut std::ffi::c_void, size: usize) -> bool;
        fn seal_pages(&mut self, address: *mut std::ffi::c_void, size: usize) -> bool;
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Permission {
        None,
        Read,
        ReadWrite,
        ReadExecute,
        ReadWriteExecute,
    }

    impl From<Permission> for PagePermissions {
        fn from(perm: Permission) -> Self {
            match perm {
                Permission::None => PagePermissions::None,
                Permission::Read => PagePermissions::Read,
                Permission::ReadWrite => PagePermissions::ReadWrite,
                Permission::ReadExecute => PagePermissions::ReadExecute,
                Permission::ReadWriteExecute => PagePermissions::ReadWriteExecute,
            }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum PagePermissions {
        None,
        Read,
        ReadWrite,
        ReadExecute,
        ReadWriteExecute,
    }

    pub struct VirtualAddressSpacePageAllocator {
        vas_: *mut VirtualAddressSpace,
        resized_allocations_: StdMutex<HashMap<usize, usize>>,
        mutex_: StdMutex<()>,
    }

    impl VirtualAddressSpacePageAllocator {
        pub fn new(vas: *mut VirtualAddressSpace) -> Self {
            VirtualAddressSpacePageAllocator {
                vas_: vas,
                resized_allocations_: StdMutex::new(HashMap::new()),
                mutex_: StdMutex::new(()),
            }
        }
    }

    impl PageAllocator for VirtualAddressSpacePageAllocator {
        fn allocate_page_size(&self) -> usize {
            unsafe { (*self.vas_).allocation_granularity() }
        }

        fn commit_page_size(&self) -> usize {
            unsafe { (*self.vas_).page_size() }
        }

        fn set_random_mmap_seed(&mut self, seed: i64) {
            unsafe { (*self.vas_).SetRandomSeed(seed) }
        }

        fn get_random_mmap_addr(&self) -> *mut std::ffi::c_void {
            unsafe { (*self.vas_).RandomPageAddress() as *mut std::ffi::c_void }
        }

        fn allocate_pages(
            &mut self,
            hint: *mut std::ffi::c_void,
            size: usize,
            alignment: usize,
            access: Permission,
        ) -> *mut std::ffi::c_void {
            let address = unsafe {
                (*self.vas_).AllocatePages(
                    hint as usize,
                    size,
                    alignment,
                    access.into(),
                )
            };
            address as *mut std::ffi::c_void
        }

        fn free_pages(&mut self, address: *mut std::ffi::c_void, size: usize) -> bool {
            let _guard = self.mutex_.lock().unwrap();
            let mut resized_allocations = self.resized_allocations_.lock().unwrap();
            let address_usize = address as usize;

            if let Some(&original_size) = resized_allocations.get(&address_usize) {
                unsafe { (*self.vas_).FreePages(address_usize, original_size) };
                resized_allocations.remove(&address_usize);
            } else {
                unsafe { (*self.vas_).FreePages(address_usize, size) };
            }

            true
        }

        fn release_pages(
            &mut self,
            address: *mut std::ffi::c_void,
            size: usize,
            new_size: usize,
        ) -> bool {
            let _guard = self.mutex_.lock().unwrap();
            let mut resized_allocations = self.resized_allocations_.lock().unwrap();
            let address_usize = address as usize;

            if resized_allocations.contains_key(&address_usize) {
                return false;
            }

            let decommit_address = address as usize + new_size;
            let decommit_size = size - new_size;

            resized_allocations.insert(address_usize, size);
            let result = unsafe { (*self.vas_).DecommitPages(decommit_address, decommit_size) };
            result
        }

        fn set_permissions(
            &mut self,
            address: *mut std::ffi::c_void,
            size: usize,
            access: Permission,
        ) -> bool {
            unsafe {
                (*self.vas_).SetPagePermissions(
                    address as usize,
                    size,
                    access.into(),
                )
            }
        }

        fn recommit_pages(
            &mut self,
            address: *mut std::ffi::c_void,
            size: usize,
            access: Permission,
        ) -> bool {
            unsafe {
                (*self.vas_).RecommitPages(
                    address as usize,
                    size,
                    access.into(),
                )
            }
        }

        fn discard_system_pages(&mut self, address: *mut std::ffi::c_void, size: usize) -> bool {
            unsafe { (*self.vas_).DiscardSystemPages(address as usize, size) }
        }

        fn decommit_pages(&mut self, address: *mut std::ffi::c_void, size: usize) -> bool {
            unsafe { (*self.vas_).DecommitPages(address as usize, size) }
        }

        fn seal_pages(&mut self, address: *mut std::ffi::c_void, size: usize) -> bool {
            false
        }
    }
}
