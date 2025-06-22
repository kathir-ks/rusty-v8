// src/sandbox/hardware_support.rs

// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)] // Suppress warnings about unused code when the feature is disabled

#[cfg(feature = "v8_enable_sandbox_hardware_support")]
mod platform {
    use crate::page_allocator::Permission;
    use std::sync::atomic::{AtomicI32, Ordering};

    #[allow(non_camel_case_types)]
    type Address = usize;
    
    pub struct MemoryProtectionKey {
        key: i32,
    }

    impl MemoryProtectionKey {
        pub const K_NO_MEMORY_PROTECTION_KEY: i32 = 0;
        pub const K_DEFAULT_PROTECTION_KEY: i32 = 1;

        pub fn allocate_key() -> i32 {
            // Simulate key allocation.  In a real implementation, this
            // would interact with the OS.  For this stub, we just return
            // a unique, incrementing number.
            static NEXT_KEY: AtomicI32 = AtomicI32::new(2);
            NEXT_KEY.fetch_add(1, Ordering::Relaxed)
        }

        pub fn set_permissions_and_key(range: (Address, usize), perm: Permission, key: i32) -> bool {
            // Simulate setting permissions. In a real implementation, this
            // would interact with the OS. For this stub, we simply print
            // the operation.
            println!(
                "Setting permissions for range {:?} to {:?} with key {}",
                range, perm, key
            );
            true // Indicate success for the stub
        }
    
        pub fn set_permissions_for_key(key: i32, permission: KeyPermission) {
            println!("Setting permissions for key {} to {:?}", key, permission);
        }
    }

    #[derive(Debug)]
    pub enum KeyPermission {
        NoRestrictions,
        DisableAccess,
    }
}

#[cfg(not(feature = "v8_enable_sandbox_hardware_support"))]
mod platform {
    #[allow(non_camel_case_types)]
    type Address = usize;
    
    #[derive(Debug)]
    pub enum Permission {}

    pub struct MemoryProtectionKey {}

    impl MemoryProtectionKey {
        pub const K_NO_MEMORY_PROTECTION_KEY: i32 = 0;
        pub const K_DEFAULT_PROTECTION_KEY: i32 = 1;

        pub fn allocate_key() -> i32 {
            0
        }
        pub fn set_permissions_and_key(_range: (Address, usize), _perm: Permission, _key: i32) -> bool {
            false
        }

        pub fn set_permissions_for_key(_key: i32, _permission: KeyPermission) {}
    }

    #[derive(Debug)]
    pub enum KeyPermission {
        NoRestrictions,
        DisableAccess,
    }
}

pub mod hardware_support {
    use super::platform;
    use super::platform::{Address, MemoryProtectionKey, KeyPermission};
    #[cfg(feature = "v8_enable_sandbox_hardware_support")]
    use crate::page_allocator::Permission;

    #[cfg(feature = "v8_enable_sandbox_hardware_support")]
    use std::sync::atomic::{AtomicI32, Ordering};

    #[cfg(feature = "v8_enable_sandbox_hardware_support")]
    static PKEY: AtomicI32 = AtomicI32::new(MemoryProtectionKey::K_NO_MEMORY_PROTECTION_KEY);

    /// Provides hardware-based support for sandboxing.
    pub struct SandboxHardwareSupport {}

    impl SandboxHardwareSupport {
        /// Tries to enable hardware-based memory protection for the given address range.
        pub fn try_enable(addr: Address, size: usize) -> bool {
            #[cfg(feature = "v8_enable_sandbox_hardware_support")]
            {
                let pkey = PKEY.load(Ordering::Relaxed);
                if pkey != MemoryProtectionKey::K_NO_MEMORY_PROTECTION_KEY {
                    MemoryProtectionKey::set_permissions_and_key(
                        (addr, size),
                        Permission::NoAccess,
                        pkey,
                    )
                } else {
                    false
                }
            }
            #[cfg(not(feature = "v8_enable_sandbox_hardware_support"))]
            {
                false
            }
        }

        /// Initializes the hardware support before thread creation.
        pub fn initialize_before_thread_creation() {
            #[cfg(feature = "v8_enable_sandbox_hardware_support")]
            {
                assert_eq!(PKEY.load(Ordering::Relaxed), MemoryProtectionKey::K_NO_MEMORY_PROTECTION_KEY);
                let new_key = MemoryProtectionKey::allocate_key();
                PKEY.store(new_key, Ordering::Relaxed);
            }
            #[cfg(not(feature = "v8_enable_sandbox_hardware_support"))]
            {}
        }

        /// Sets default permissions for the signal handler.
        pub fn set_default_permissions_for_signal_handler() {
            #[cfg(feature = "v8_enable_sandbox_hardware_support")]
            {
                let pkey = PKEY.load(Ordering::Relaxed);
                if pkey != MemoryProtectionKey::K_NO_MEMORY_PROTECTION_KEY {
                    MemoryProtectionKey::set_permissions_for_key(pkey, KeyPermission::NoRestrictions);
                }
            }
            #[cfg(not(feature = "v8_enable_sandbox_hardware_support"))]
            {}
        }

        /// Notifies the hardware support that a read-only page has been created.
        pub fn notify_read_only_page_created(addr: Address, size: usize, perm: Permission) {
            #[cfg(feature = "v8_enable_sandbox_hardware_support")]
            {
                let pkey = PKEY.load(Ordering::Relaxed);
                if pkey != MemoryProtectionKey::K_NO_MEMORY_PROTECTION_KEY {
                    MemoryProtectionKey::set_permissions_and_key(
                        (addr, size),
                        perm,
                        MemoryProtectionKey::K_DEFAULT_PROTECTION_KEY,
                    );
                }
            }
            #[cfg(not(feature = "v8_enable_sandbox_hardware_support"))]
            {}
        }

        /// Returns a scope that potentially blocks access to memory.
        pub fn maybe_block_access() -> BlockAccessScope {
            #[cfg(feature = "v8_enable_sandbox_hardware_support")]
            {
                BlockAccessScope::new(PKEY.load(Ordering::Relaxed))
            }
            #[cfg(not(feature = "v8_enable_sandbox_hardware_support"))]
            {
                BlockAccessScope::new(MemoryProtectionKey::K_NO_MEMORY_PROTECTION_KEY)
            }
        }
    }

    /// A scope that temporarily blocks access to memory.
    pub struct BlockAccessScope {
        #[cfg(feature = "v8_enable_sandbox_hardware_support")]
        pkey: i32,
    }

    impl BlockAccessScope {
        #[cfg(feature = "v8_enable_sandbox_hardware_support")]
        fn new(pkey: i32) -> Self {
            if pkey != MemoryProtectionKey::K_NO_MEMORY_PROTECTION_KEY {
                MemoryProtectionKey::set_permissions_for_key(pkey, KeyPermission::DisableAccess);
            }
            BlockAccessScope { pkey }
        }

        #[cfg(not(feature = "v8_enable_sandbox_hardware_support"))]
        fn new(_pkey: i32) -> Self {
            BlockAccessScope {}
        }
    }

    impl Drop for BlockAccessScope {
        fn drop(&mut self) {
            #[cfg(feature = "v8_enable_sandbox_hardware_support")]
            {
                if self.pkey != MemoryProtectionKey::K_NO_MEMORY_PROTECTION_KEY {
                    MemoryProtectionKey::set_permissions_for_key(self.pkey, KeyPermission::NoRestrictions);
                }
            }
            #[cfg(not(feature = "v8_enable_sandbox_hardware_support"))]
            {}
        }
    }
}

#[cfg(feature = "v8_enable_sandbox_hardware_support")]
mod page_allocator {
    #[derive(Debug, Copy, Clone)]
    pub enum Permission {
        NoAccess,
        ReadWrite, // Add ReadWrite permission type
        ReadOnly // Add ReadOnly permission type
    }
}

#[cfg(not(feature = "v8_enable_sandbox_hardware_support"))]
mod page_allocator {
    #[derive(Debug, Copy, Clone)]
    pub enum Permission {
        NoAccess,
        ReadWrite, // Add ReadWrite permission type
        ReadOnly // Add ReadOnly permission type
    }
}