// Converted from V8 C++ source files:
// Header: lsan-virtual-address-space.h
// Implementation: lsan-virtual-address-space.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod sanitizer {
        pub mod lsan_virtual_address_space {
            use std::ptr::null_mut;
            use std::sync::Mutex;
            use v8::base::PagePermissions;
            use v8::PlatformSharedMemoryHandle;
            use v8::VirtualAddressSpace;

            pub type Address = usize;

            pub struct LsanVirtualAddressSpace {
                vas_: Box<dyn VirtualAddressSpace>,
            }

            impl LsanVirtualAddressSpace {
                pub fn new(vas: Box<dyn VirtualAddressSpace>) -> Self {
                    let page_size = vas.page_size();
                    let allocation_granularity = vas.allocation_granularity();
                    let base = vas.base();
                    let size = vas.size();
                    let max_page_permissions = vas.max_page_permissions();

                    LsanVirtualAddressSpace { vas_: vas }
                }

                pub fn set_random_seed(&mut self, seed: i64) {
                    self.vas_.set_random_seed(seed);
                }

                pub fn random_page_address(&mut self) -> Address {
                    self.vas_.random_page_address()
                }

                pub fn allocate_pages(
                    &mut self,
                    hint: Address,
                    size: usize,
                    alignment: usize,
                    permissions: PagePermissions,
                ) -> Address {
                    let result = self.vas_.allocate_pages(hint, size, alignment, permissions);
                    #[cfg(leak_sanitizer)]
                    unsafe {
                        if result != 0 {
                            extern "C" {
                                fn __lsan_register_root_region(ptr: *mut std::ffi::c_void, size: usize);
                            }
                            __lsan_register_root_region(result as *mut std::ffi::c_void, size);
                        }
                    }
                    result
                }

                pub fn free_pages(&mut self, address: Address, size: usize) {
                    self.vas_.free_pages(address, size);
                    #[cfg(leak_sanitizer)]
                    unsafe {
                        extern "C" {
                            fn __lsan_unregister_root_region(ptr: *mut std::ffi::c_void, size: usize);
                        }
                        __lsan_unregister_root_region(address as *mut std::ffi::c_void, size);
                    }
                }

                pub fn allocate_shared_pages(
                    &mut self,
                    hint: Address,
                    size: usize,
                    permissions: PagePermissions,
                    handle: PlatformSharedMemoryHandle,
                    offset: u64,
                ) -> Address {
                    let result = self.vas_.allocate_shared_pages(hint, size, permissions, handle, offset);
                    #[cfg(leak_sanitizer)]
                    unsafe {
                        if result != 0 {
                            extern "C" {
                                fn __lsan_register_root_region(ptr: *mut std::ffi::c_void, size: usize);
                            }
                            __lsan_register_root_region(result as *mut std::ffi::c_void, size);
                        }
                    }
                    result
                }

                pub fn free_shared_pages(&mut self, address: Address, size: usize) {
                    self.vas_.free_shared_pages(address, size);
                    #[cfg(leak_sanitizer)]
                    unsafe {
                        extern "C" {
                            fn __lsan_unregister_root_region(ptr: *mut std::ffi::c_void, size: usize);
                        }
                        __lsan_unregister_root_region(address as *mut std::ffi::c_void, size);
                    }
                }

                pub fn set_page_permissions(
                    &mut self,
                    address: Address,
                    size: usize,
                    permissions: PagePermissions,
                ) -> bool {
                    self.vas_.set_page_permissions(address, size, permissions)
                }

                pub fn recommit_pages(
                    &mut self,
                    address: Address,
                    size: usize,
                    permissions: PagePermissions,
                ) -> bool {
                    self.vas_.recommit_pages(address, size, permissions)
                }

                pub fn allocate_guard_region(&mut self, address: Address, size: usize) -> bool {
                    self.vas_.allocate_guard_region(address, size)
                }

                pub fn free_guard_region(&mut self, address: Address, size: usize) {
                    self.vas_.free_guard_region(address, size);
                }

                pub fn can_allocate_subspaces(&self) -> bool {
                    self.vas_.can_allocate_subspaces()
                }

                pub fn allocate_subspace(
                    &mut self,
                    hint: Address,
                    size: usize,
                    alignment: usize,
                    max_page_permissions: PagePermissions,
                ) -> Box<dyn VirtualAddressSpace> {
                    let subspace = self.vas_.allocate_subspace(hint, size, alignment, max_page_permissions);

                    #[cfg(leak_sanitizer)]
                    {
                        return Box::new(LsanVirtualAddressSpace::new(subspace));
                    }

                    subspace
                }

                pub fn discard_system_pages(&mut self, address: Address, size: usize) -> bool {
                    self.vas_.discard_system_pages(address, size)
                }

                pub fn decommit_pages(&mut self, address: Address, size: usize) -> bool {
                    self.vas_.decommit_pages(address, size)
                }
            }

            impl VirtualAddressSpace for LsanVirtualAddressSpace {
                fn page_size(&self) -> usize {
                    self.vas_.page_size()
                }

                fn allocation_granularity(&self) -> usize {
                    self.vas_.allocation_granularity()
                }

                fn base(&self) -> Address {
                    self.vas_.base()
                }

                fn size(&self) -> usize {
                    self.vas_.size()
                }

                fn max_page_permissions(&self) -> PagePermissions {
                    self.vas_.max_page_permissions()
                }

                fn set_random_seed(&mut self, seed: i64) {
                    self.vas_.set_random_seed(seed);
                }

                fn random_page_address(&mut self) -> Address {
                    self.vas_.random_page_address()
                }

                fn allocate_pages(
                    &mut self,
                    hint: Address,
                    size: usize,
                    alignment: usize,
                    permissions: PagePermissions,
                ) -> Address {
                    self.allocate_pages(hint, size, alignment, permissions)
                }

                fn free_pages(&mut self, address: Address, size: usize) {
                    self.free_pages(address, size)
                }

                fn allocate_shared_pages(
                    &mut self,
                    hint: Address,
                    size: usize,
                    permissions: PagePermissions,
                    handle: PlatformSharedMemoryHandle,
                    offset: u64,
                ) -> Address {
                    self.allocate_shared_pages(hint, size, permissions, handle, offset)
                }

                fn free_shared_pages(&mut self, address: Address, size: usize) {
                    self.free_shared_pages(address, size)
                }

                fn set_page_permissions(
                    &mut self,
                    address: Address,
                    size: usize,
                    permissions: PagePermissions,
                ) -> bool {
                    self.set_page_permissions(address, size, permissions)
                }

                fn recommit_pages(
                    &mut self,
                    address: Address,
                    size: usize,
                    permissions: PagePermissions,
                ) -> bool {
                    self.recommit_pages(address, size, permissions)
                }

                fn allocate_guard_region(&mut self, address: Address, size: usize) -> bool {
                    self.allocate_guard_region(address, size)
                }

                fn free_guard_region(&mut self, address: Address, size: usize) {
                    self.free_guard_region(address, size)
                }

                fn can_allocate_subspaces(&self) -> bool {
                    self.can_allocate_subspaces()
                }

                fn allocate_subspace(
                    &mut self,
                    hint: Address,
                    size: usize,
                    alignment: usize,
                    max_page_permissions: PagePermissions,
                ) -> Box<dyn VirtualAddressSpace> {
                    self.allocate_subspace(hint, size, alignment, max_page_permissions)
                }

                fn discard_system_pages(&mut self, address: Address, size: usize) -> bool {
                    self.discard_system_pages(address, size)
                }

                fn decommit_pages(&mut self, address: Address, size: usize) -> bool {
                    self.decommit_pages(address, size)
                }
            }
        }
    }
}

pub mod v8 {
    use std::ptr::null_mut;
    use std::sync::Mutex;

    pub type Address = usize;
    pub struct PlatformSharedMemoryHandle {}
    pub enum PagePermissions {
        None,
    }

    pub trait VirtualAddressSpace {
        fn page_size(&self) -> usize;
        fn allocation_granularity(&self) -> usize;
        fn base(&self) -> Address;
        fn size(&self) -> usize;
        fn max_page_permissions(&self) -> PagePermissions;
        fn set_random_seed(&mut self, seed: i64);
        fn random_page_address(&mut self) -> Address;
        fn allocate_pages(
            &mut self,
            hint: Address,
            size: usize,
            alignment: usize,
            permissions: PagePermissions,
        ) -> Address;
        fn free_pages(&mut self, address: Address, size: usize);
        fn allocate_shared_pages(
            &mut self,
            hint: Address,
            size: usize,
            permissions: PagePermissions,
            handle: PlatformSharedMemoryHandle,
            offset: u64,
        ) -> Address;
        fn free_shared_pages(&mut self, address: Address, size: usize);
        fn set_page_permissions(
            &mut self,
            address: Address,
            size: usize,
            permissions: PagePermissions,
        ) -> bool;
        fn recommit_pages(
            &mut self,
            address: Address,
            size: usize,
            permissions: PagePermissions,
        ) -> bool;
        fn allocate_guard_region(&mut self, address: Address, size: usize) -> bool;
        fn free_guard_region(&mut self, address: Address, size: usize);
        fn can_allocate_subspaces(&self) -> bool;
        fn allocate_subspace(
            &mut self,
            hint: Address,
            size: usize,
            alignment: usize,
            max_page_permissions: PagePermissions,
        ) -> Box<dyn VirtualAddressSpace>;
        fn discard_system_pages(&mut self, address: Address, size: usize) -> bool;
        fn decommit_pages(&mut self, address: Address, size: usize) -> bool;
    }
}
