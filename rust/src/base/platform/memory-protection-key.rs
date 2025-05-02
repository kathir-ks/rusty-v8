// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/base/platform/memory-protection-key.rs

#[cfg(v8_has_pku_jit_write_protect)]
pub mod memory_protection_key {
    use crate::base::address_region::AddressRegion;
    use v8::PageAllocator;

    /// Sentinel value if there is no PKU support or allocation of a key failed.
    /// This is also the return value on an error of pkey_alloc() and has the
    /// benefit that calling pkey_mprotect() with -1 behaves the same as regular
    /// mprotect().
    pub const K_NO_MEMORY_PROTECTION_KEY: i32 = -1;

    /// The default ProtectionKey can be used to remove pkey assignments.
    pub const K_DEFAULT_PROTECTION_KEY: i32 = 0;

    /// Permissions for memory protection keys on top of the page's permissions.
    /// NOTE: Since there is no executable bit, the executable permission cannot be
    /// withdrawn by memory protection keys.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Permission {
        KNoRestrictions = 0,
        KDisableAccess = 1,
        KDisableWrite = 2,
    }

    // If sys/mman.h has PKEY support (on newer Linux distributions), ensure that
    // our definitions of the permissions is consistent with the ones in glibc.
    // NOTE: These asserts are not possible in Rust without direct access to the glibc headers
    // or a comparable mechanism. They must be checked through other means.

    /// This class has static methods for the different platform specific
    /// functions related to memory protection key support.
    pub struct MemoryProtectionKey {}

    impl MemoryProtectionKey {
        /// Call exactly once per process to determine if PKU is supported on this
        /// platform and initialize global data structures.
        pub fn has_memory_protection_key_support() -> bool {
            // TODO(sroettger): Implement the check for PKU support.
            // This likely involves checking for the presence of the pkey_* functions.
            // Placeholder: Assume PKU is supported if the feature flag is enabled.
            true
        }

        /// Allocates a new key. Returns -1 on error.
        pub fn allocate_key() -> i32 {
            // TODO(sroettger): Implement key allocation using pkey_alloc().
            // Returns -1 on error.
            // Placeholder: Always return -1 for now.
            -1
        }

        /// Associates a memory protection {key} with the given {region}.
        /// If {key} is {K_NO_MEMORY_PROTECTION_KEY} this behaves like "plain"
        /// {SetPermissions()} and associates the default key to the region. That is,
        /// explicitly calling with {K_NO_MEMORY_PROTECTION_KEY} can be used to
        /// disassociate any protection key from a region. This also means "plain"
        /// {SetPermissions()} disassociates the key from a region, making the key's
        /// access restrictions irrelevant/inactive for that region. Returns true if
        /// changing permissions and key was successful. (Returns a bool to be
        /// consistent with {SetPermissions()}). The {page_permissions} are the
        /// permissions of the page, not the key. For changing the permissions of the
        /// key, use {SetPermissionsForKey()} instead.
        pub fn set_permissions_and_key(
            region: AddressRegion,
            page_permissions: PageAllocator::Permission,
            key: i32,
        ) -> bool {
            // TODO(sroettger): Implement permission and key setting using pkey_mprotect().
            // Placeholder: Always return false for now.
            false
        }

        /// Set the key's permissions. {key} must be valid, i.e. not
        /// {K_NO_MEMORY_PROTECTION_KEY}.
        pub fn set_permissions_for_key(key: i32, permissions: Permission) {
            // TODO(sroettger): Implement key permission setting using pkey_set().
        }

        /// Get the permissions of the protection key {key} for the current thread.
        pub fn get_key_permission(key: i32) -> Permission {
            // TODO(sroettger): Implement key permission retrieval using pkey_get().
            // Placeholder: Return kNoRestrictions for now.
            Permission::KNoRestrictions
        }
    }
}

#[cfg(not(v8_has_pku_jit_write_protect))]
pub mod memory_protection_key {
    /// Sentinel value if there is no PKU support or allocation of a key failed.
    /// This is also the return value on an error of pkey_alloc() and has the
    /// benefit that calling pkey_mprotect() with -1 behaves the same as regular
    /// mprotect().
    pub const K_NO_MEMORY_PROTECTION_KEY: i32 = -1;

    /// The default ProtectionKey can be used to remove pkey assignments.
    pub const K_DEFAULT_PROTECTION_KEY: i32 = 0;

    /// Permissions for memory protection keys on top of the page's permissions.
    /// NOTE: Since there is no executable bit, the executable permission cannot be
    /// withdrawn by memory protection keys.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Permission {
        KNoRestrictions = 0,
        KDisableAccess = 1,
        KDisableWrite = 2,
    }
    pub struct MemoryProtectionKey {}

    impl MemoryProtectionKey {
        pub fn has_memory_protection_key_support() -> bool {
            false
        }

        pub fn allocate_key() -> i32 {
            K_NO_MEMORY_PROTECTION_KEY
        }

        pub fn set_permissions_and_key(
            _region: crate::base::address_region::AddressRegion,
            _page_permissions: v8::PageAllocator::Permission,
            _key: i32,
        ) -> bool {
            false
        }

        pub fn set_permissions_for_key(_key: i32, _permissions: Permission) {}

        pub fn get_key_permission(_key: i32) -> Permission {
            Permission::KNoRestrictions
        }
    }
}