// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Platform-specific code shared between macOS and iOS.

use libc::{
    mach_port_t, mach_vm_address_t, mach_vm_size_t, memory_object_offset_t, vm_prot_t, vm_inherit_t,
    pthread_t, size_t, vm_flags_t,
};

use std::{
    ptr,
    sync::atomic::{AtomicBool, Ordering},
};

//use std::os::raw::c_void; // Assuming c_void is used where void* was used in C++
//use std::ffi::CStr; // For converting C-style strings to Rust strings

#[cfg(target_os = "macos")]
mod macos {
    use libc::{c_int, sysctlbyname};

    extern "C" {
        pub fn pthread_get_stackaddr_np(pthread: pthread_t) -> *mut libc::c_void;
        // pub fn pthread_jit_write_protect_np(enable: c_int) -> c_int; //This function is missing on some platforms, making it impossible to translate
    }
    // Function is not available on all platforms, this is a placeholder
    pub fn pthread_jit_write_protect_np(enable: i32) -> Result<(), i32> {
        // This function is available only on macOS 10.14 and later
        // and requires including <pthread.h>.
        // It's also marked with __attribute__((availability(...)))
        // and is not present when building on older macOS versions, or for
        // cross-compilation.
        //
        // Returning an error to indicate the function is not implemented.
        Err(-1) // Placeholder error code
    }

    pub fn adjust_scheduling_params() {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        {
            // Check availability of scheduling params.
            let mut val: u32 = 0;
            let mut val_size: size_t = std::mem::size_of::<u32>();
            let rc: i32;

            unsafe {
                rc = sysctlbyname(
                    "kern.tcsm_available\0".as_ptr() as *const i8,
                    &mut val as *mut u32 as *mut libc::c_void,
                    &mut val_size,
                    ptr::null_mut(),
                    0,
                );
            }

            if rc < 0 || val == 0 {
                return;
            }

            // Adjust scheduling params.
            let val: u32 = 1;
            let rc: i32;
            unsafe {
                rc = sysctlbyname(
                    "kern.tcsm_enable\0".as_ptr() as *const i8,
                    ptr::null_mut(),
                    ptr::null_mut(),
                    &val as *const u32 as *const libc::c_void,
                    std::mem::size_of::<u32>(),
                );
            }

            assert!(rc >= 0);
        }
    }
}

#[cfg(target_os = "ios")]
mod ios {
    // Placeholder for iOS specific code
}

mod platform_shared {
    use super::*;
    use libc::{KERN_SUCCESS, mach_make_memory_entry_64, mach_port_deallocate, mach_task_self, MAP_MEM_NAMED_CREATE, VM_PROT_READ, VM_PROT_WRITE, MACH_PORT_NULL, vm_map, VM_FLAGS_FIXED, VM_FLAGS_ANYWHERE, vm_remap, VM_INHERIT_NONE, vm_address_t};

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct PlatformSharedMemoryHandle(u32);

    pub const K_INVALID_SHARED_MEMORY_HANDLE: PlatformSharedMemoryHandle = PlatformSharedMemoryHandle(0);

    pub fn shared_memory_handle_from_mach_memory_entry(port: mach_port_t) -> PlatformSharedMemoryHandle {
        PlatformSharedMemoryHandle(port)
    }

    pub fn mach_memory_entry_from_shared_memory_handle(handle: PlatformSharedMemoryHandle) -> mach_port_t {
        handle.0
    }

    fn mach_vm_map_wrapper(address: &mut mach_vm_address_t,
                           size: mach_vm_size_t,
                           flags: i32,
                           port: mach_port_t,
                           offset: memory_object_offset_t,
                           prot: vm_prot_t) -> Result<(), i32> {
        let mut current_prot = prot;
        let maximum_prot = current_prot;
        let kr: i32;
        unsafe {
            kr = vm_map(mach_task_self(),
                            address,
                            size,
                            0,
                            flags,
                            port,
                            offset,
                            0,
                            current_prot,
                            maximum_prot,
                            VM_INHERIT_NONE);
        }
        if kr == KERN_SUCCESS {
            Ok(())
        } else {
            Err(kr)
        }
    }

    pub fn create_shared_memory_handle_for_testing(size: size_t) -> PlatformSharedMemoryHandle {
        let vm_size: mach_vm_size_t = size as mach_vm_size_t;
        let mut port: mach_port_t = 0;
        let kr: i32;

        unsafe {
            kr = mach_make_memory_entry_64(
                mach_task_self(),
                &mut vm_size as *mut mach_vm_size_t,
                0,
                MAP_MEM_NAMED_CREATE | VM_PROT_READ | VM_PROT_WRITE,
                &mut port as *mut mach_port_t,
                MACH_PORT_NULL,
            );
        }

        if kr != KERN_SUCCESS {
            return K_INVALID_SHARED_MEMORY_HANDLE;
        }

        shared_memory_handle_from_mach_memory_entry(port)
    }

    pub fn destroy_shared_memory_handle(handle: PlatformSharedMemoryHandle) -> Result<(), i32> {
        assert_ne!(K_INVALID_SHARED_MEMORY_HANDLE, handle);
        let port = mach_memory_entry_from_shared_memory_handle(handle);
        let kr: i32;
        unsafe {
            kr = mach_port_deallocate(mach_task_self(), port);
        }
        if kr == KERN_SUCCESS {
            Ok(())
        } else {
            Err(kr)
        }
    }

    pub fn allocate_shared(hint: *mut libc::c_void,
                           size: size_t,
                           access: MemoryPermission,
                           handle: PlatformSharedMemoryHandle,
                           offset: u64) -> *mut libc::c_void {
        assert_eq!(0, size % crate::base::os::allocate_page_size());

        let mut addr = hint as mach_vm_address_t;
        let prot = get_vm_prot_from_memory_permission(access);
        let shared_mem_port = mach_memory_entry_from_shared_memory_handle(handle);

        let kr: i32 = unsafe {
            mach_vm_map_wrapper(&mut addr,
                                      size as mach_vm_size_t,
                                      VM_FLAGS_FIXED,
                                      shared_mem_port,
                                      offset as memory_object_offset_t,
                                      prot).err().unwrap_or(0)
        };

        if kr != KERN_SUCCESS {
            // Retry without hint.
            unsafe {
                mach_vm_map_wrapper(&mut addr,
                                          size as mach_vm_size_t,
                                          VM_FLAGS_ANYWHERE,
                                          shared_mem_port,
                                          offset as memory_object_offset_t,
                                          prot).err().unwrap_or(0);
            }
        }

        if kr != KERN_SUCCESS {
            return ptr::null_mut();
        }
        addr as *mut libc::c_void
    }

    pub fn remap_pages(address: *const libc::c_void,
                       size: size_t,
                       new_address: *mut libc::c_void,
                       access: MemoryPermission) -> bool {
        assert!(crate::base::os::is_aligned(address as usize, crate::base::os::allocate_page_size()));
        assert!(crate::base::os::is_aligned(new_address as usize, crate::base::os::allocate_page_size()));
        assert!(crate::base::os::is_aligned(size, crate::base::os::allocate_page_size()));

        let cur_protection = get_vm_prot_from_memory_permission(access);
        let mut max_protection: vm_prot_t = 0; // Initialize to a safe value
                                                   // Asks the kernel to remap *on top* of an existing mapping, rather than
                                                   // copying the data.
        let flags = libc::VM_FLAGS_FIXED as i32 | libc::VM_FLAGS_OVERWRITE as i32; //VM_FLAGS_FIXED | VM_FLAGS_OVERWRITE;
        let mut target = new_address as mach_vm_address_t;
        let ret: i32;

        unsafe {
            ret = vm_remap(mach_task_self(),
                                &mut target as *mut vm_address_t,
                                size as mach_vm_size_t,
                                0,
                                flags,
                                mach_task_self(),
                                address as mach_vm_address_t,
                                0,
                                &mut cur_protection as *mut vm_prot_t,
                                &mut max_protection as *mut vm_prot_t,
                                VM_INHERIT_NONE);
        }

        if ret != KERN_SUCCESS {
            return false;
        }

        // Did we get the address we wanted?
        assert_eq!(new_address, target as *mut libc::c_void);

        true
    }

    fn get_vm_prot_from_memory_permission(access: MemoryPermission) -> vm_prot_t {
        match access {
            MemoryPermission::kNoAccess => libc::VM_PROT_NONE,
            MemoryPermission::kNoAccessWillJitLater => libc::VM_PROT_NONE,
            MemoryPermission::kRead => libc::VM_PROT_READ,
            MemoryPermission::kReadWrite => libc::VM_PROT_READ | libc::VM_PROT_WRITE,
            MemoryPermission::kReadWriteExecute => {
                libc::VM_PROT_READ | libc::VM_PROT_WRITE | libc::VM_PROT_EXECUTE
            }
            MemoryPermission::kReadExecute => libc::VM_PROT_READ | libc::VM_PROT_EXECUTE,
        }
    }
}

pub mod base {
    pub mod os {
        use std::ffi::{CStr, CString};
        use std::mem;
        use libc::{c_char, c_void, sysconf, _SC_PAGESIZE, size_t, vm_address_t, vm_size_t};
        use super::super::platform_shared::*;

        extern "C" {
            pub fn _dyld_image_count() -> u32;
            pub fn _dyld_get_image_header(image_index: u32) -> *const c_void;
            pub fn _dyld_get_image_name(image_index: u32) -> *const c_char;
            pub fn _dyld_get_image_vmaddr_slide(image_index: u32) -> i64;
            // pub fn getsectiondata(header: *const mach_header, segname: *const c_char, sectname: *const c_char, size: *mut size_t) -> *mut u8; // needs mach-o headers

            // placeholder for now
            fn getsectiondata(header: *const c_void, segname: *const c_char, sectname: *const c_char, size: *mut size_t) -> *mut u8;
        }

        #[derive(Debug)]
        pub struct SharedLibraryAddress {
            name: String,
            start: usize,
            end: usize,
            slide: i64,
        }

        impl SharedLibraryAddress {
            pub fn new(name: String, start: usize, end: usize, slide: i64) -> Self {
                SharedLibraryAddress { name, start, end, slide }
            }
        }

        pub fn get_shared_library_addresses() -> Vec<SharedLibraryAddress> {
            let images_count = unsafe { _dyld_image_count() };
            let mut result = Vec::new();

            for i in 0..images_count {
                let header = unsafe { _dyld_get_image_header(i) };
                if header.is_null() {
                    continue;
                }

                let mut size: size_t = 0;
                let code_ptr: *mut u8;

                #[cfg(target_pointer_width = "32")]
                unsafe {
                    let seg_name = CString::new("__TEXT").unwrap();
                    let sect_name = CString::new("__text").unwrap();
                    code_ptr = getsectiondata(
                        header,
                        seg_name.as_ptr(),
                        sect_name.as_ptr(),
                        &mut size as *mut size_t,
                    );
                }
                #[cfg(target_pointer_width = "64")]
                unsafe {
                    let seg_name = CString::new("__TEXT").unwrap();
                    let sect_name = CString::new("__text").unwrap();

                    code_ptr = getsectiondata(
                        header,
                        seg_name.as_ptr(),
                        sect_name.as_ptr(),
                        &mut size as *mut size_t,
                    );
                }

                if code_ptr.is_null() {
                    continue;
                }

                let slide = unsafe { _dyld_get_image_vmaddr_slide(i) };
                let start = code_ptr as usize;

                let name_ptr = unsafe { _dyld_get_image_name(i) };
                let name = unsafe {
                    CStr::from_ptr(name_ptr)
                        .to_string_lossy()
                        .into_owned()
                };

                result.push(SharedLibraryAddress::new(name, start, start + size, slide));
            }

            result
        }

        pub fn signal_code_moving_gc() {
            // No-op for now
        }

        pub fn create_timezone_cache() -> PosixDefaultTimezoneCache {
            PosixDefaultTimezoneCache {}
        }

        use super::super::macos::adjust_scheduling_params;
        pub fn adjust_scheduling_params_wrapper() {
             adjust_scheduling_params();
        }


        pub fn get_first_free_memory_range_within(
            boundary_start: usize,
            boundary_end: usize,
            minimum_size: usize,
            alignment: usize,
        ) -> Option<()> {
            None
        }

        pub fn allocate_page_size() -> usize {
            unsafe { sysconf(_SC_PAGESIZE) as usize }
        }

        pub fn is_aligned(address: usize, alignment: usize) -> bool {
            address % alignment == 0
        }
    }

    pub mod platform {
        use super::super::macos::pthread_get_stackaddr_np;
        use libc::{pthread_self};
        use std::os::raw::c_void;

        pub struct Stack {}

        impl Stack {
            pub fn obtain_current_thread_stack_start() -> *mut c_void {
                unsafe { pthread_get_stackaddr_np(pthread_self()) }
            }
        }
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum MemoryPermission {
            kNoAccess,
            kNoAccessWillJitLater,
            kRead,
            kReadWrite,
            kReadWriteExecute,
            kReadExecute,
        }
    }
}

struct PosixDefaultTimezoneCache {}

struct AddressSpaceReservation {
    base: *mut libc::c_void,
    size: usize,
}

impl AddressSpaceReservation {
    fn contains(&self, address: *mut libc::c_void, size: usize) -> bool {
        let start = self.base as usize;
        let end = start + self.size;
        let addr_start = address as usize;
        let addr_end = addr_start + size;
        addr_start >= start && addr_end <= end
    }

    fn allocate_shared(&self, address: *mut libc::c_void, size: usize, access: base::platform::MemoryPermission, handle: platform_shared::PlatformSharedMemoryHandle, offset: u64) -> bool {
        assert!(self.contains(address, size));
        let addr = address as mach_vm_address_t;
        let shared_mem_port = platform_shared::mach_memory_entry_from_shared_memory_handle(handle);
        let prot = get_vm_prot_from_memory_permission(access);
        let flags = libc::VM_FLAGS_FIXED as i32 | libc::VM_FLAGS_OVERWRITE as i32;

        let kr: i32;
        unsafe {
            kr = vm_map(mach_task_self(),
                            &addr as *const mach_vm_address_t as *mut mach_vm_address_t, // Corrected: Use pointer to addr for in-out parameter
                            size as mach_vm_size_t,
                            0,
                            flags,
                            shared_mem_port,
                            offset as memory_object_offset_t,
                            0,
                            prot,
                            prot,
                            VM_INHERIT_NONE);
        }

        kr == libc::KERN_SUCCESS
    }
}

extern "C" {
    fn vm_map(target: mach_port_t, address: *mut mach_vm_address_t, size: mach_vm_size_t, mask: memory_object_offset_t, flags: i32, object: mach_port_t, offset: memory_object_offset_t, copy: i32, cur_protection: vm_prot_t, max_protection: vm_prot_t, inheritance: vm_inherit_t) -> i32;
}

fn get_vm_prot_from_memory_permission(access: base::platform::MemoryPermission) -> vm_prot_t {
    match access {
        base::platform::MemoryPermission::kNoAccess => libc::VM_PROT_NONE,
        base::platform::MemoryPermission::kNoAccessWillJitLater => libc::VM_PROT_NONE,
        base::platform::MemoryPermission::kRead => libc::VM_PROT_READ,
        base::platform::MemoryPermission::kReadWrite => libc::VM_PROT_READ | libc::VM_PROT_WRITE,
        base::platform::MemoryPermission::kReadWriteExecute => {
            libc::VM_PROT_READ | libc::VM_PROT_WRITE | libc::VM_PROT_EXECUTE
        }
        base::platform::MemoryPermission::kReadExecute => {
            libc::VM_PROT_READ | libc::VM_PROT_EXECUTE
        }
    }
}