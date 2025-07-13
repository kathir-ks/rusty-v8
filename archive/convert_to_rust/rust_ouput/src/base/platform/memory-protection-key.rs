// Converted from V8 C++ source files:
// Header: memory-protection-key.h
// Implementation: memory-protection-key.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
pub mod platform {

#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
pub mod memory_protection_key {
    use std::mem::size_of;
    use std::os::raw::c_int;
    use std::{ffi::c_void, sync::Once};

    use crate::base::AddressRegion;
    use crate::v8::PageAllocator;

    // Define weak linkage attributes.  These are compiler-specific.
    #[cfg(compiler_supports_weak_attributes)]
    macro_rules! weak_linkage {
        () => {
            #[link_section = ".gnu.linkonce.default"]
        };
    }

    #[cfg(not(compiler_supports_weak_attributes))]
    macro_rules! weak_linkage {
        () => {};
    }

    //use libc;
    extern "C" {
        #[cfg_attr(
            all(target_os = "linux", target_arch = "x86_64"),
            link_name = "pkey_mprotect"
        )]
        #[weak_linkage!()]
        static pkey_mprotect: Option<
            unsafe extern "C" fn(addr: *mut c_void, len: usize, prot: c_int, pkey: c_int) -> c_int,
        >;
        #[cfg_attr(all(target_os = "linux", target_arch = "x86_64"), link_name = "pkey_get")]
        #[weak_linkage!()]
        static pkey_get: Option<unsafe extern "C" fn(pkey: c_int) -> c_int>;
        #[cfg_attr(all(target_os = "linux", target_arch = "x86_64"), link_name = "pkey_set")]
        #[weak_linkage!()]
        static pkey_set: Option<unsafe extern "C" fn(pkey: c_int, value: u32) -> c_int>;
        #[cfg_attr(all(target_os = "linux", target_arch = "x86_64"), link_name = "pkey_alloc")]
        #[weak_linkage!()]
        static pkey_alloc: Option<unsafe extern "C" fn(flags: u32, mask: u32) -> c_int>;
    }

    pub struct MemoryProtectionKey {}

    impl MemoryProtectionKey {
        pub const K_NO_MEMORY_PROTECTION_KEY: i32 = -1;
        pub const K_DEFAULT_PROTECTION_KEY: i32 = 0;

        #[derive(Debug, PartialEq, Eq, Copy, Clone)]
        pub enum Permission {
            K_NO_RESTRICTIONS = 0,
            K_DISABLE_ACCESS = 1,
            K_DISABLE_WRITE = 2,
        }

        fn get_protection_from_memory_permission(
            permission: PageAllocator::Permission,
        ) -> i32 {
            match permission {
                PageAllocator::Permission::kNoAccess => libc::PROT_NONE,
                PageAllocator::Permission::kRead => libc::PROT_READ,
                PageAllocator::Permission::kReadWrite => {
                    libc::PROT_READ | libc::PROT_WRITE
                }
                PageAllocator::Permission::kReadWriteExecute => {
                    libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC
                }
            }
        }

        pub fn has_memory_protection_key_support() -> bool {
            unsafe {
                if pkey_mprotect.is_none() {
                    return false;
                }
                true
            }
        }

        pub fn allocate_key() -> i32 {
            unsafe {
                match pkey_alloc {
                    Some(func) => func(0, 0),
                    None => MemoryProtectionKey::K_NO_MEMORY_PROTECTION_KEY,
                }
            }
        }

        pub fn set_permissions_and_key(
            region: AddressRegion,
            page_permissions: v8::PageAllocator::Permission,
            key: i32,
        ) -> bool {
            unsafe {
                if let Some(func) = pkey_mprotect {
                    let address = region.begin() as *mut c_void;
                    let size = region.size();
                    let protection =
                        MemoryProtectionKey::get_protection_from_memory_permission(
                            page_permissions,
                        );
                    func(address, size, protection, key) == 0
                } else {
                    false
                }
            }
        }

        pub fn set_permissions_for_key(key: i32, permissions: Permission) {
            unsafe {
                if let Some(func) = pkey_set {
                    func(key, permissions as u32);
                }
            }
        }

        pub fn get_key_permission(key: i32) -> Permission {
            unsafe {
                if let Some(func) = pkey_get {
                    let permission = func(key);
                    match permission {
                        0 => Permission::K_NO_RESTRICTIONS,
                        1 => Permission::K_DISABLE_ACCESS,
                        2 => Permission::K_DISABLE_WRITE,
                        _ => Permission::K_NO_RESTRICTIONS,
                    }
                } else {
                    Permission::K_NO_RESTRICTIONS
                }
            }
        }
    }
}
}
}

pub mod v8 {
    pub mod PageAllocator {
        #[derive(Debug, PartialEq, Eq, Copy, Clone)]
        pub enum Permission {
            kNoAccess,
            kRead,
            kReadWrite,
            kReadWriteExecute,
        }
    }
}

pub mod base {
    use std::ops::Range;

    #[derive(Debug, Copy, Clone)]
    pub struct AddressRegion {
        range: Range<usize>,
    }

    impl AddressRegion {
        pub fn new(begin: usize, end: usize) -> Self {
            AddressRegion { range: begin..end }
        }

        pub fn begin(&self) -> usize {
            self.range.start
        }

        pub fn end(&self) -> usize {
            self.range.end
        }

        pub fn size(&self) -> usize {
            self.range.end - self.range.start
        }
    }
}
