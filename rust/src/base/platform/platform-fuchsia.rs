// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::mem::size_of;
use std::option::Option;
use std::ptr;
use std::slice;
use std::{ffi::CString, os::raw::c_char};

use fuchsia_component::client::connect_to_protocol;
use fuchsia_zircon as zx;
use fuchsia_zircon::AsHandleRef;

mod base {
    pub mod bits {
        // TODO: Implement equivalent functionality if needed.
    }
    pub mod macros {
        // TODO: Implement equivalent functionality if needed.
        // Examples:
        // #[macro_export]
        // macro_rules! DCHECK { ... }
        // #[macro_export]
        // macro_rules! UNREACHABLE { ... }
    }
    pub mod platform {
        pub mod platform_posix_time {
            pub struct TimezoneCache {}
        }
        pub mod platform_posix {
            pub fn posix_initialize_common(_abort_mode: super::super::os::AbortMode, _gc_fake_mmap: Option<&str>) {}
        }
        pub mod platform {
            use super::super::super::super::zx;
            use super::super::super::super::zx::Rights;
            use std::{ptr, sync::Once};
            use thiserror::Error;

            #[derive(Debug, Error)]
            pub enum MemoryError {
                #[error("Zircon error: {0}")]
                ZirconError(#[from] zx::Status),
                #[error("Invalid alignment specified")]
                InvalidAlignment,
            }

            pub type Result<T> = std::result::Result<T, MemoryError>;

            pub type PlatformSharedMemoryHandle = zx::Handle;
            pub const kInvalidSharedMemoryHandle: PlatformSharedMemoryHandle = zx::Handle::INVALID;

            #[derive(Clone, Copy, PartialEq, Eq, Debug)]
            pub enum MemoryPermission {
                kNoAccess,
                kNoAccessWillJitLater,
                kRead,
                kReadWrite,
                kReadWriteExecute,
                kReadExecute,
            }

            #[derive(Clone, Copy, PartialEq, Eq, Debug)]
            pub enum AbortMode {
                kAbortOnFailedAllocation,
                kReturnNullOnFailedAllocation,
            }

            #[derive(Debug)]
            pub struct AddressSpaceReservation {
                base_: *mut std::ffi::c_void,
                size_: usize,
                vmar_: zx::Handle,
            }

            impl AddressSpaceReservation {
                pub fn new(base_: *mut std::ffi::c_void, size_: usize, vmar_: zx::Handle) -> Self {
                    Self {
                        base_: base_,
                        size_: size_,
                        vmar_: vmar_,
                    }
                }
                pub fn base(&self) -> *mut std::ffi::c_void {
                    self.base_
                }

                pub fn size(&self) -> usize {
                    self.size_
                }
                fn contains(&self, address: *mut std::ffi::c_void, size: usize) -> bool {
                    let start = self.base_ as usize;
                    let end = start + self.size_;
                    let addr_start = address as usize;
                    let addr_end = addr_start + size;
                    addr_start >= start && addr_end <= end
                }

                pub fn create_sub_reservation(
                    &self,
                    address: *mut std::ffi::c_void,
                    size: usize,
                    max_permission: MemoryPermission,
                ) -> Option<AddressSpaceReservation> {
                    if !self.contains(address, size) {
                        return None;
                    }
                    let child_vmar_handle =
                        unsafe { zx::Handle::from_raw(self.vmar_.raw_handle()) };
                    let (child, child_addr) = create_address_space_reservation_internal(
                        &zx::Vmar::from(child_vmar_handle),
                        self.base(),
                        os::allocate_page_size(),
                        address,
                        PlacementMode::kFixed,
                        size,
                        os::allocate_page_size(),
                        max_permission,
                    )
                    .ok()?;
                    assert_eq!(child_addr as *mut std::ffi::c_void, address);
                    Some(AddressSpaceReservation::new(address, size, child.into_raw()))
                }
                pub fn free_sub_reservation(&self, reservation: AddressSpaceReservation) -> bool {
                    os::free_address_space_reservation(reservation);
                    true
                }

                pub fn allocate(
                    &self,
                    address: *mut std::ffi::c_void,
                    size: usize,
                    access: MemoryPermission,
                ) -> bool {
                    if !self.contains(address, size) {
                        return false;
                    }
                    let allocation = create_and_map_vmo(
                        &zx::Vmar::from(unsafe { zx::Handle::from_raw(self.vmar_.raw_handle()) }),
                        self.base(),
                        os::allocate_page_size(),
                        address,
                        PlacementMode::kFixed,
                        size,
                        os::allocate_page_size(),
                        access,
                    );

                    allocation.map_or(false, |alloc| alloc == address)
                }

                pub fn free(&self, address: *mut std::ffi::c_void, size: usize) -> bool {
                    if !self.contains(address, size) {
                        return false;
                    }
                    unmap_vmo(
                        &zx::Vmar::from(unsafe { zx::Handle::from_raw(self.vmar_.raw_handle()) }),
                        os::allocate_page_size(),
                        address,
                        size,
                    )
                }

                pub fn allocate_shared(
                    &self,
                    address: *mut std::ffi::c_void,
                    size: usize,
                    access: MemoryPermission,
                    handle: PlatformSharedMemoryHandle,
                    offset: u64,
                ) -> bool {
                    if !self.contains(address, size) {
                        return false;
                    }
                    let vmo = unsafe { zx::Vmo::from_raw(handle.raw_handle()) };
                    let mapped = map_vmo(
                        &zx::Vmar::from(unsafe { zx::Handle::from_raw(self.vmar_.raw_handle()) }),
                        self.base(),
                        os::allocate_page_size(),
                        address,
                        &vmo,
                        offset,
                        PlacementMode::kFixed,
                        size,
                        os::allocate_page_size(),
                        access,
                    );
                    zx::Handle::into_raw(vmo.into_handle());
                    mapped.is_some()
                }

                pub fn free_shared(&self, address: *mut std::ffi::c_void, size: usize) -> bool {
                    if !self.contains(address, size) {
                        return false;
                    }
                    unmap_vmo(
                        &zx::Vmar::from(unsafe { zx::Handle::from_raw(self.vmar_.raw_handle()) }),
                        os::allocate_page_size(),
                        address,
                        size,
                    )
                }
                pub fn set_permissions(
                    &self,
                    address: *mut std::ffi::c_void,
                    size: usize,
                    access: MemoryPermission,
                ) -> bool {
                    if !self.contains(address, size) {
                        return false;
                    }
                    set_permissions_internal(
                        &zx::Vmar::from(unsafe { zx::Handle::from_raw(self.vmar_.raw_handle()) }),
                        os::commit_page_size(),
                        address,
                        size,
                        access,
                    )
                }

                pub fn recommit_pages(
                    &self,
                    address: *mut std::ffi::c_void,
                    size: usize,
                    access: MemoryPermission,
                ) -> bool {
                    self.set_permissions(address, size, access)
                }
                pub fn discard_system_pages(&self, address: *mut std::ffi::c_void, size: usize) -> bool {
                    if !self.contains(address, size) {
                        return false;
                    }
                    discard_system_pages_internal(
                        &zx::Vmar::from(unsafe { zx::Handle::from_raw(self.vmar_.raw_handle()) }),
                        os::commit_page_size(),
                        address,
                        size,
                    )
                }

                pub fn decommit_pages(&self, address: *mut std::ffi::c_void, size: usize) -> bool {
                    if !self.contains(address, size) {
                        return false;
                    }
                    self.set_permissions(address, size, MemoryPermission::kNoAccess)
                        && self.discard_system_pages(address, size)
                }
            }

            impl Drop for AddressSpaceReservation {
                fn drop(&mut self) {
                    let vmar = unsafe { zx::Vmar::from(zx::Handle::from_raw(self.vmar_.raw_handle())) };
                    let _ = vmar.destroy();
                }
            }

            // TODO: Consider using atomic types.
            static mut G_VMEX_RESOURCE: zx::Handle = zx::Handle::INVALID;
            static VMEX_RESOURCE_INIT: Once = Once::new();
            static mut G_ROOT_VMAR_BASE: *mut std::ffi::c_void = ptr::null_mut();

            fn set_vmex_resource() {
                unsafe {
                    if G_VMEX_RESOURCE != zx::Handle::INVALID {
                        return;
                    }
                }

                let vmex_resource_client = connect_to_protocol::<fuchsia_kernel::VmexResource>();

                if let Ok(vmex_resource_client) = vmex_resource_client {
                    let result = vmex_resource_client.get();
                    match result {
                        Ok(res) => unsafe {
                            G_VMEX_RESOURCE = res.resource;
                        },
                        Err(_) => {}
                    }
                }
            }

            fn get_protection_from_memory_permission(
                access: MemoryPermission,
            ) -> zx::VmarFlags {
                match access {
                    MemoryPermission::kNoAccess => zx::VmarFlags::empty(),
                    MemoryPermission::kNoAccessWillJitLater => zx::VmarFlags::empty(),
                    MemoryPermission::kRead => zx::VmarFlags::READ,
                    MemoryPermission::kReadWrite => zx::VmarFlags::READ | zx::VmarFlags::WRITE,
                    MemoryPermission::kReadWriteExecute => {
                        zx::VmarFlags::READ | zx::VmarFlags::WRITE | zx::VmarFlags::EXECUTE
                    }
                    MemoryPermission::kReadExecute => {
                        zx::VmarFlags::READ | zx::VmarFlags::EXECUTE
                    }
                }
            }

            fn get_alignment_option_from_alignment(alignment: usize) -> Result<zx::VmarFlags> {
                // The alignment must be one of the ZX_VM_ALIGN_X constants.
                // See zircon/system/public/zircon/types.h.
                const ZX_VM_ALIGN_BASE: usize = 10;
                const ZX_VM_ALIGN_1KB: usize = 10 << ZX_VM_ALIGN_BASE;
                const ZX_VM_ALIGN_4GB: usize = 32 << ZX_VM_ALIGN_BASE;

                assert_eq!(ZX_VM_ALIGN_1KB, (10 << ZX_VM_ALIGN_BASE));
                assert_eq!(ZX_VM_ALIGN_4GB, (32 << ZX_VM_ALIGN_BASE));

                let alignment_log2 = match alignment {
                    _ if alignment == (1 << 10) => 10,
                    _ if alignment == (1 << 11) => 11,
                    _ if alignment == (1 << 12) => 12,
                    _ if alignment == (1 << 13) => 13,
                    _ if alignment == (1 << 14) => 14,
                    _ if alignment == (1 << 15) => 15,
                    _ if alignment == (1 << 16) => 16,
                    _ if alignment == (1 << 17) => 17,
                    _ if alignment == (1 << 18) => 18,
                    _ if alignment == (1 << 19) => 19,
                    _ if alignment == (1 << 20) => 20,
                    _ if alignment == (1 << 21) => 21,
                    _ if alignment == (1 << 22) => 22,
                    _ if alignment == (1 << 23) => 23,
                    _ if alignment == (1 << 24) => 24,
                    _ if alignment == (1 << 25) => 25,
                    _ if alignment == (1 << 26) => 26,
                    _ if alignment == (1 << 27) => 27,
                    _ if alignment == (1 << 28) => 28,
                    _ if alignment == (1 << 29) => 29,
                    _ if alignment == (1 << 30) => 30,
                    _ if alignment == (1 << 31) => 31,
                    _ if alignment == (1 << 32) => 32,
                    _ => return Err(MemoryError::InvalidAlignment),
                };

                Ok(zx::VmarFlags::from_bits_truncate(
                    (alignment_log2 << ZX_VM_ALIGN_BASE) as u32,
                ))
            }

            #[derive(PartialEq, Eq, Copy, Clone, Debug)]
            enum PlacementMode {
                kUseHint,
                kAnywhere,
                kFixed,
            }

            fn map_vmo(
                vmar: &zx::Vmar,
                vmar_base: *mut std::ffi::c_void,
                page_size: usize,
                address: *mut std::ffi::c_void,
                vmo: &zx::Vmo,
                offset: u64,
                placement: PlacementMode,
                size: usize,
                alignment: usize,
                access: MemoryPermission,
            ) -> Option<*mut std::ffi::c_void> {
                assert_eq!(0, size % page_size);
                assert_eq!(0, (address as usize) % page_size);
                if placement != PlacementMode::kAnywhere {
                    assert!(!address.is_null());
                }

                let mut options = get_protection_from_memory_permission(access);

                let alignment_option =
                    get_alignment_option_from_alignment(alignment).ok()?;
                options |= alignment_option;

                let mut vmar_offset: usize = 0;
                if placement != PlacementMode::kAnywhere {
                    let target_addr = address as usize;
                    let base = vmar_base as usize;
                    assert!(target_addr >= base);
                    vmar_offset = target_addr - base;
                    options |= zx::VmarFlags::SPECIFIC;
                }

                let result = vmar.map(vmar_offset, vmo, offset, size as u64, options);
                match result {
                    Ok(result) => Some(result as *mut std::ffi::c_void),
                    Err(status) => {
                        if status == zx::Status::ALREADY_EXISTS
                            && placement == PlacementMode::kUseHint
                        {
                            let options = options & !(zx::VmarFlags::SPECIFIC);
                            let result = vmar.map(0, vmo, offset, size as u64, options);
                            match result {
                                Ok(result) => Some(result as *mut std::ffi::c_void),
                                Err(_) => None,
                            }
                        } else {
                            None
                        }
                    }
                }
            }

            fn create_and_map_vmo(
                vmar: &zx::Vmar,
                vmar_base: *mut std::ffi::c_void,
                page_size: usize,
                address: *mut std::ffi::c_void,
                placement: PlacementMode,
                size: usize,
                alignment: usize,
                access: MemoryPermission,
            ) -> Option<*mut std::ffi::c_void> {
                let vmo_result = zx::Vmo::create(size as u64, zx::VmoOptions::empty());
                let mut vmo = match vmo_result {
                    Ok(vmo) => vmo,
                    Err(_) => return None,
                };

                const VIRTUAL_MEMORY_NAME: &str = "v8-virtualmem";
                let name_cstring = CString::new(VIRTUAL_MEMORY_NAME).unwrap();
                let name_bytes = name_cstring.as_bytes_with_nul();
                let name_slice = name_bytes.split_last().unwrap().1;

                let _ = vmo.set_name(VIRTUAL_MEMORY_NAME);

                unsafe {
                    VMEX_RESOURCE_INIT.call_once(|| {
                        set_vmex_resource();
                    });

                    if G_VMEX_RESOURCE != zx::Handle::INVALID {
                        let exec_result = vmo.replace_as_executable(&zx::Resource::from(zx::Handle::duplicate_handle(&G_VMEX_RESOURCE, Rights::SAME_RIGHTS).unwrap()));
                        if exec_result.is_err() {
                            return None;
                        }
                    }
                }
                let result = map_vmo(
                    vmar,
                    vmar_base,
                    page_size,
                    address,
                    &vmo,
                    0,
                    placement,
                    size,
                    alignment,
                    access,
                );
                zx::Handle::into_raw(vmo.into_handle());
                result
            }

            fn unmap_vmo(vmar: &zx::Vmar, page_size: usize, address: *mut std::ffi::c_void, size: usize) -> bool {
                assert_eq!(0, (address as usize) % page_size);
                assert_eq!(0, size % page_size);
                vmar.unmap(address as usize, size as u64).is_ok()
            }

            fn set_permissions_internal(
                vmar: &zx::Vmar,
                page_size: usize,
                address: *mut std::ffi::c_void,
                size: usize,
                access: MemoryPermission,
            ) -> bool {
                assert_eq!(0, (address as usize) % page_size);
                assert_eq!(0, size % page_size);
                let prot = get_protection_from_memory_permission(access);
                let status = vmar.protect(address as usize, size as u64, prot);
                status.is_ok()
            }

            fn discard_system_pages_internal(
                vmar: &zx::Vmar,
                page_size: usize,
                address: *mut std::ffi::c_void,
                size: usize,
            ) -> bool {
                assert_eq!(0, (address as usize) % page_size);
                assert_eq!(0, size % page_size);
                let address_int = address as u64;
                vmar.op_range(
                    zx::VmoOp::Decommit,
                    address_int,
                    size as u64,
                )
                .is_ok()
            }

            fn create_address_space_reservation_internal(
                vmar: &zx::Vmar,
                vmar_base: *mut std::ffi::c_void,
                page_size: usize,
                address: *mut std::ffi::c_void,
                placement: PlacementMode,
                size: usize,
                alignment: usize,
                max_permission: MemoryPermission,
            ) -> Result<(zx::Vmar, usize)> {
                assert_eq!(0, size % page_size);
                assert_eq!(0, alignment % page_size);
                assert_eq!(0, (address as usize) % alignment);
                if placement != PlacementMode::kAnywhere {
                    assert!(!address.is_null());
                }

                let mut options = zx::VmarFlags::CAN_MAP_READ
                    | zx::VmarFlags::CAN_MAP_WRITE
                    | zx::VmarFlags::CAN_MAP_EXECUTE
                    | zx::VmarFlags::SPECIFIC;

                let alignment_option =
                    get_alignment_option_from_alignment(alignment)?;
                options |= alignment_option;

                let mut vmar_offset: usize = 0;
                if placement != PlacementMode::kAnywhere {
                    let target_addr = address as usize;
                    let base = vmar_base as usize;
                    assert!(target_addr >= base);
                    vmar_offset = target_addr - base;
                    options |= zx::VmarFlags::SPECIFIC;
                }
                let (child_vmar, child_addr) = vmar
                    .allocate(vmar_offset, size as u64, options)
                    .map_err(MemoryError::ZirconError)?;

                if placement == PlacementMode::kUseHint
                    && child_addr as *mut std::ffi::c_void != address
                {
                    let options = options & !(zx::VmarFlags::SPECIFIC);
                    let (child_vmar, child_addr) = vmar
                        .allocate(0, size as u64, options)
                        .map_err(MemoryError::ZirconError)?;
                    return Ok((child_vmar, child_addr));
                }

                Ok((child_vmar, child_addr))
            }

            pub mod os {
                use super::*;
                use super::super::platform_posix_time::TimezoneCache;
                use super::super::platform_posix;
                use fuchsia_zircon::{HandleBased, Rights};

                pub fn create_timezone_cache() -> Box<TimezoneCache> {
                    Box::new(TimezoneCache {})
                }

                pub fn initialize(abort_mode: AbortMode, gc_fake_mmap: Option<&str>) {
                    platform_posix::posix_initialize_common(abort_mode, gc_fake_mmap);
                    unsafe {
                         let vmar_result = zx::Vmar::root_self();
                        let info = vmar_result.get_info()?;
                        G_ROOT_VMAR_BASE = info.base as *mut std::ffi::c_void;

                        VMEX_RESOURCE_INIT.call_once(|| {
                            super::set_vmex_resource();
                        });
                    }
                }

                pub fn allocate(
                    address: *mut std::ffi::c_void,
                    size: usize,
                    alignment: usize,
                    access: MemoryPermission,
                ) -> Option<*mut std::ffi::c_void> {
                    let placement = if !address.is_null() {
                        PlacementMode::kUseHint
                    } else {
                        PlacementMode::kAnywhere
                    };
                    unsafe {
                    let vmar_result = zx::Vmar::root_self();
                    let vmar = vmar_result;
                        create_and_map_vmo(
                            &vmar,
                            G_ROOT_VMAR_BASE,
                            allocate_page_size(),
                            address,
                            placement,
                            size,
                            alignment,
                            access,
                        )
                    }
                }

                pub fn free(address: *mut std::ffi::c_void, size: usize) {
                    unsafe {
                        let vmar_result = zx::Vmar::root_self();
                        let vmar = vmar_result;
                        assert!(unmap_vmo(&vmar, allocate_page_size(), address, size));
                    }
                }

                pub fn allocate_shared(
                    address: *mut std::ffi::c_void,
                    size: usize,
                    access: MemoryPermission,
                    handle: PlatformSharedMemoryHandle,
                    offset: u64,
                ) -> Option<*mut std::ffi::c_void> {
                    let placement = if !address.is_null() {
                        PlacementMode::kUseHint
                    } else {
                        PlacementMode::kAnywhere
                    };
                    unsafe {
                        let vmo = zx::Vmo::from_raw(handle.raw_handle());
                        let vmar_result = zx::Vmar::root_self();
                        let vmar = vmar_result;
                        let result = map_vmo(
                            &vmar,
                            G_ROOT_VMAR_BASE,
                            allocate_page_size(),
                            address,
                            &vmo,
                            offset,
                            placement,
                            size,
                            allocate_page_size(),
                            access,
                        );
                        zx::Handle::into_raw(vmo.into_handle());
                        result
                    }
                }

                pub fn free_shared(address: *mut std::ffi::c_void, size: usize) {
                    unsafe {
                        let vmar_result = zx::Vmar::root_self();
                         let vmar = vmar_result;
                        assert!(unmap_vmo(&vmar, allocate_page_size(), address, size));
                    }
                }

                pub fn release(address: *mut std::ffi::c_void, size: usize) {
                    free(address, size);
                }

                pub fn set_permissions(
                    address: *mut std::ffi::c_void,
                    size: usize,
                    access: MemoryPermission,
                ) -> bool {
                    unsafe {
                        let vmar_result = zx::Vmar::root_self();
                         let vmar = vmar_result;
                        set_permissions_internal(
                            &vmar,
                            commit_page_size(),
                            address,
                            size,
                            access,
                        )
                    }
                }

                pub fn set_data_read_only(address: *mut std::ffi::c_void, size: usize) {
                    assert!(set_permissions(address, size, MemoryPermission::kRead));
                }

                pub fn recommit_pages(
                    address: *mut std::ffi::c_void,
                    size: usize,
                    access: MemoryPermission,
                ) -> bool {
                    set_permissions(address, size, access)
                }

                pub fn discard_system_pages(address: *mut std::ffi::c_void, size: usize) -> bool {
                    unsafe {
                        let vmar_result = zx::Vmar::root_self();
                         let vmar = vmar_result;
                        discard_system_pages_internal(
                            &vmar,
                            commit_page_size(),
                            address,
                            size,
                        )
                    }
                }

                pub fn decommit_pages(address: *mut std::ffi::c_void, size: usize) -> bool {
                    set_permissions(address, size, MemoryPermission::kNoAccess)
                        && discard_system_pages(address, size)
                }

                pub fn seal_pages(_address: *mut std::ffi::c_void, _size: usize) -> bool {
                    false
                }

                pub fn can_reserve_address_space() -> bool {
                    true
                }

                pub fn create_address_space_reservation(
                    hint: *mut std::ffi::c_void,
                    size: usize,
                    alignment: usize,
                    max_permission: MemoryPermission,
                ) -> Option<AddressSpaceReservation> {
                    assert_eq!((hint as usize) % alignment, 0);

                    let placement = if !hint.is_null() {
                        PlacementMode::kUseHint
                    } else {
                        PlacementMode::kAnywhere
                    };
                    unsafe {
                        let vmar_result = zx::Vmar::root_self();
                         let vmar = vmar_result;

                        let (child, child_addr) =
                            super::create_address_space_reservation_internal(
                                &vmar,
                                G_ROOT_VMAR_BASE,
                                allocate_page_size(),
                                hint,
                                placement,
                                size,
                                alignment,
                                max_permission,
                            )
                            .ok()?;

                        Some(AddressSpaceReservation::new(
                            child_addr as *mut std::ffi::c_void,
                            size,
                            child.into_raw()
                        ))
                    }
                }

                pub fn free_address_space_reservation(reservation: AddressSpaceReservation) {
                     // Destroy the vmar and release the handle.
                    let _ = unsafe { zx::Vmar::from(zx::Handle::from_raw(reservation.vmar_.raw_handle())).destroy() };
                }

                pub fn create_shared_memory_handle_for_testing(size: usize) -> PlatformSharedMemoryHandle {
                    let vmo_result = zx::Vmo::create(size as u64, zx::VmoOptions::empty());
                    match vmo_result {
                        Ok(vmo) => SharedMemoryHandleFromVMO(vmo.into_handle()),
                        Err(_) => kInvalidSharedMemoryHandle,
                    }
                }

                pub fn destroy_shared_memory_handle(handle: PlatformSharedMemoryHandle) {
                    assert_ne!(kInvalidSharedMemoryHandle, handle);
                    let vmo = VMOFromSharedMemoryHandle(handle);
                     unsafe { zx::Handle::from_raw(vmo.raw_handle()) }.close();
                }

                pub fn has_lazy_commits() -> bool {
                    true
                }

                pub fn get_shared_library_addresses() -> Vec<SharedLibraryAddress> {
                    unreachable!(); // TODO(scottmg): Port, https://crbug.com/731217.
                }

                pub fn signal_code_moving_gc() {
                    unreachable!(); // TODO(scottmg): Port, https://crbug.com/731217.
                }

                pub fn get_user_time(secs: &mut u32, usecs: &mut u32) -> i32 {
                    const NANOS_PER_MICROSECOND: u64 = 1000;
                    const MICROS_PER_SECOND: u64 = 1000000;

                    let mut info: zx::ThreadStats = Default::default();
                    let thread = zx::Thread::self();

                    let status = thread.get_info(&mut info);

                    if status.is_err() {
                        return -1;
                    }

                    let micros_since_thread_started =
                        (info.total_runtime + NANOS_PER_MICROSECOND - 1) / NANOS_PER_MICROSECOND;

                    *secs = (micros_since_thread_started / MICROS_PER_SECOND) as u32;
                    *usecs = (micros_since_thread_started % MICROS_PER_SECOND) as u32;

                    0
                }

                pub fn adjust_scheduling_params() {}

                pub fn get_first_free_memory_range_within(
                    _boundary_start: *mut std::ffi::c_void,
                    _boundary_end: *mut std::ffi::c_void,
                    _minimum_size: usize,
                    _alignment: usize,
                ) -> Option<MemoryRange> {
                    None
                }

                // Helper functions (platform-specific)
                #[cfg(target_os = "fuchsia")]
                pub fn allocate_page_size() -> usize {
                    zx::system_get_page_size() as usize
                }

                #[cfg(target_os = "fuchsia")]
                pub fn commit_page_size() -> usize {
                    zx::system_get_page_size() as usize
                }
            }

            #[derive(Debug)]
            pub struct SharedLibraryAddress {}

            #[derive(Debug)]
            pub struct MemoryRange {}

            pub fn SharedMemoryHandleFromVMO(vmo: zx::Handle) -> PlatformSharedMemoryHandle {
                vmo
            }

            pub fn VMOFromSharedMemoryHandle(handle: PlatformSharedMemoryHandle) -> zx::Handle {
                handle
            }

            unsafe impl Send for AddressSpaceReservation {}
            unsafe impl Sync for AddressSpaceReservation {}
        }
    }
    pub mod os {
        pub use super::platform::platform::os::*;
        pub use super::platform::platform::*;
    }
}

pub mod v8 {
    pub use super::base::os;
    pub use super::base::platform::platform::AddressSpaceReservation;
    pub use super::base::platform::platform::MemoryPermission;
}