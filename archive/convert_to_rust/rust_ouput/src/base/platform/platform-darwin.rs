// Converted from V8 C++ source files:
// Header: N/A
// Implementation: platform-darwin.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

use std::os::raw::c_void;
use std::ptr;
use std::sync::Mutex;
use std::sync::RwLock;
use libc::{c_char, size_t, ssize_t};

#[derive(Debug)]
enum PlatformError {
    MachError(i32),
    Other(String),
}

impl std::fmt::Display for PlatformError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlatformError::MachError(code) => write!(f, "Mach error: {}", code),
            PlatformError::Other(msg) => write!(f, "Platform error: {}", msg),
        }
    }
}

impl std::error::Error for PlatformError {}

mod mach {
    use std::os::raw::c_void;
    use libc::{size_t, mach_port_t, vm_prot_t, kern_return_t};

    pub type mach_vm_address_t = u64;
    pub type mach_vm_size_t = u64;
    pub type memory_object_offset_t = u64;
    pub type vm_inherit_t = i32;

    extern "C" {
        pub fn mach_task_self() -> mach_port_t;
        pub fn mach_make_memory_entry_64(
            target_task: mach_port_t,
            size: *mut mach_vm_size_t,
            offset: memory_object_offset_t,
            flags: i32,
            out_memory_object: *mut mach_port_t,
            parent_entry: mach_port_t,
        ) -> kern_return_t;
        pub fn mach_port_deallocate(task: mach_port_t, name: mach_port_t) -> kern_return_t;
        pub fn mach_vm_map(
            target: mach_port_t,
            address: *mut mach_vm_address_t,
            size: mach_vm_size_t,
            mask: mach_vm_address_t,
            flags: i32,
            memory_object: mach_port_t,
            offset: memory_object_offset_t,
            copy: i32,
            cur_protection: vm_prot_t,
            max_protection: vm_prot_t,
            inheritance: vm_inherit_t,
        ) -> kern_return_t;
        pub fn mach_vm_remap(
            target_task: mach_port_t,
            target_address: *mut mach_vm_address_t,
            size: mach_vm_size_t,
            offset: mach_vm_address_t,
            flags: i32,
            source_task: mach_port_t,
            source_address: mach_vm_address_t,
            copy: i32,
            cur_protection: *mut vm_prot_t,
            max_protection: *mut vm_prot_t,
            inheritance: vm_inherit_t,
        ) -> kern_return_t;
    }
}

const MAP_MEM_NAMED_CREATE: i32 = 0x00001000;
const VM_PROT_NONE: i32 = 0x00;
const VM_PROT_READ: i32 = 0x01;
const VM_PROT_WRITE: i32 = 0x02;
const VM_PROT_EXECUTE: i32 = 0x04;
const VM_FLAGS_FIXED: i32 = 0x0000;
const VM_FLAGS_ANYWHERE: i32 = 0x0010;
const VM_FLAGS_OVERWRITE: i32 = 0x4000;
const VM_INHERIT_NONE: i32 = 2;
const KERN_SUCCESS: i32 = 0;
const MACH_PORT_NULL: mach::mach_port_t = 0;

pub struct PlatformSharedMemoryHandle {
    port: mach::mach_port_t,
}

const kInvalidSharedMemoryHandle: PlatformSharedMemoryHandle = PlatformSharedMemoryHandle { port: 0 };

impl PlatformSharedMemoryHandle {
    fn is_valid(&self) -> bool {
        self.port != 0
    }
}

fn SharedMemoryHandleFromMachMemoryEntry(port: mach::mach_port_t) -> PlatformSharedMemoryHandle {
    PlatformSharedMemoryHandle { port }
}

fn MachMemoryEntryFromSharedMemoryHandle(handle: PlatformSharedMemoryHandle) -> mach::mach_port_t {
    handle.port
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

fn GetVMProtFromMemoryPermission(access: MemoryPermission) -> i32 {
    match access {
        MemoryPermission::kNoAccess | MemoryPermission::kNoAccessWillJitLater => VM_PROT_NONE,
        MemoryPermission::kRead => VM_PROT_READ,
        MemoryPermission::kReadWrite => VM_PROT_READ | VM_PROT_WRITE,
        MemoryPermission::kReadWriteExecute => {
            VM_PROT_READ | VM_PROT_WRITE | VM_PROT_EXECUTE
        }
        MemoryPermission::kReadExecute => VM_PROT_READ | VM_PROT_EXECUTE,
    }
}

fn mach_vm_map_wrapper(
    address: &mut mach::mach_vm_address_t,
    size: mach::mach_vm_size_t,
    flags: i32,
    port: mach::mach_port_t,
    offset: mach::memory_object_offset_t,
    prot: i32,
) -> Result<(), PlatformError> {
    let mut current_prot = prot;
    let maximum_prot = current_prot;
    let kr = unsafe {
        mach::mach_vm_map(
            mach::mach_task_self(),
            address,
            size,
            0,
            flags,
            port,
            offset,
            0,
            current_prot,
            maximum_prot,
            VM_INHERIT_NONE,
        )
    };

    if kr != KERN_SUCCESS {
        return Err(PlatformError::MachError(kr));
    }

    Ok(())
}

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

pub struct PosixDefaultTimezoneCache {}

impl PosixDefaultTimezoneCache {
    pub fn new() -> Self {
        PosixDefaultTimezoneCache {}
    }
}

pub struct AddressSpaceReservation {
    start: *mut c_void,
    size: usize,
}

impl AddressSpaceReservation {
    pub fn new(start: *mut c_void, size: usize) -> Self {
        AddressSpaceReservation { start, size }
    }

    fn contains(&self, address: *mut c_void, size: usize) -> bool {
        let reservation_start = self.start as usize;
        let reservation_end = reservation_start + self.size;
        let address_start = address as usize;
        let address_end = address_start + size;

        address_start >= reservation_start && address_end <= reservation_end
    }

    fn allocate_shared(
        &self,
        address: *mut c_void,
        size: usize,
        access: MemoryPermission,
        handle: PlatformSharedMemoryHandle,
        offset: u64,
    ) -> Result<(), PlatformError> {
        if !self.contains(address, size) {
            return Err(PlatformError::Other("Address not within reservation".to_string()));
        }

        let prot = GetVMProtFromMemoryPermission(access);
        let mut addr = address as mach::mach_vm_address_t;
        let shared_mem_port = MachMemoryEntryFromSharedMemoryHandle(handle);
        let kr = unsafe {
            mach::mach_vm_map(
                mach::mach_task_self(),
                &mut addr,
                size as u64,
                0,
                VM_FLAGS_FIXED | VM_FLAGS_OVERWRITE,
                shared_mem_port,
                offset,
                0,
                prot,
                prot,
                VM_INHERIT_NONE,
            )
        };

        if kr != KERN_SUCCESS {
            return Err(PlatformError::MachError(kr));
        }

        Ok(())
    }
}

pub struct Stack {
}

impl Stack {
    pub fn ObtainCurrentThreadStackStart() -> *mut c_void{
        pthread_get_stackaddr_np(pthread_self()) as *mut c_void
    }
}

extern "C" {
    fn pthread_self() -> usize;
    fn pthread_get_stackaddr_np(thread: usize) -> *mut c_void;
}

pub struct OS {}

impl OS {
    pub fn get_shared_library_addresses() -> Vec<SharedLibraryAddress> {
        let mut result = Vec::new();
        let images_count = unsafe { _dyld_image_count() };
        for i in 0..images_count {
            let header = unsafe { _dyld_get_image_header(i) };
            if header.is_null() {
                continue;
            }

            let (code_ptr, size) = {
                #[cfg(target_arch = "x86_64")]
                {
                    let header64 = header as *const mach_header_64;
                    let mut size: u64 = 0;
                    let code_ptr = unsafe { getsectiondata(header64, b"__TEXT\0".as_ptr() as *const i8, b"__text\0".as_ptr() as *const i8, &mut size) };
                    (code_ptr, size as usize)
                }
                #[cfg(target_arch = "x86")]
                {
                    let mut size: u64 = 0;
                    let code_ptr = unsafe { getsectiondata(header, b"__TEXT\0".as_ptr() as *const i8, b"__text\0".as_ptr() as *const i8, &mut size) };
                    (code_ptr, size as usize)
                }
            };

            if code_ptr.is_null() {
                continue;
            }

            let slide = unsafe { _dyld_get_image_vmaddr_slide(i) };
            let start = code_ptr as usize;
            let name = unsafe {
                let name_ptr = _dyld_get_image_name(i);
                let name_cstr = std::ffi::CStr::from_ptr(name_ptr);
                name_cstr.to_string_lossy().into_owned()
            };
            result.push(SharedLibraryAddress::new(name, start, start + size, slide));
        }
        result
    }

    pub fn signal_code_moving_gc() {}

    pub fn create_timezone_cache() -> PosixDefaultTimezoneCache {
        PosixDefaultTimezoneCache::new()
    }

    pub fn adjust_scheduling_params() {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        {
            let mut val: u32 = 0;
            let mut val_size = std::mem::size_of::<u32>() as size_t;
            let rc = unsafe {
                sysctlbyname(
                    b"kern.tcsm_available\0".as_ptr() as *const i8,
                    &mut val as *mut u32 as *mut c_void,
                    &mut val_size,
                    ptr::null_mut(),
                    0,
                )
            };

            if rc < 0 || val == 0 {
                return;
            }

            let val: u32 = 1;
            let rc = unsafe {
                sysctlbyname(
                    b"kern.tcsm_enable\0".as_ptr() as *const i8,
                    ptr::null_mut(),
                    ptr::null_mut(),
                    &val as *const u32 as *const c_void,
                    std::mem::size_of::<u32>() as size_t,
                )
            };
            assert!(rc >= 0);
        }
    }

    pub fn get_first_free_memory_range_within(
        _boundary_start: *mut c_void,
        _boundary_end: *mut c_void,
        _minimum_size: usize,
        _alignment: usize,
    ) -> Option<(*mut c_void, usize)> {
        None
    }

    pub fn create_shared_memory_handle_for_testing(size: usize) -> PlatformSharedMemoryHandle {
        let mut vm_size = size as mach::mach_vm_size_t;
        let mut port: mach::mach_port_t = 0;
        let kr = unsafe {
            mach::mach_make_memory_entry_64(
                mach::mach_task_self(),
                &mut vm_size,
                0,
                MAP_MEM_NAMED_CREATE | VM_PROT_READ | VM_PROT_WRITE,
                &mut port,
                MACH_PORT_NULL,
            )
        };

        if kr != KERN_SUCCESS {
            return kInvalidSharedMemoryHandle;
        }

        SharedMemoryHandleFromMachMemoryEntry(port)
    }

    pub fn destroy_shared_memory_handle(handle: PlatformSharedMemoryHandle) {
        assert!(handle.is_valid());
        let port = MachMemoryEntryFromSharedMemoryHandle(handle);
        let kr = unsafe { mach::mach_port_deallocate(mach::mach_task_self(), port) };
        assert_eq!(kr, KERN_SUCCESS);
    }

    pub fn allocate_shared(
        hint: *mut c_void,
        size: usize,
        access: MemoryPermission,
        handle: PlatformSharedMemoryHandle,
        offset: u64,
    ) -> *mut c_void {
        assert_eq!(0, size % Self::allocate_page_size());

        let mut addr = hint as mach::mach_vm_address_t;
        let prot = GetVMProtFromMemoryPermission(access);
        let shared_mem_port = MachMemoryEntryFromSharedMemoryHandle(handle);
        let mut kr = mach_vm_map_wrapper(&mut addr, size as mach::mach_vm_size_t, VM_FLAGS_FIXED, shared_mem_port, offset, prot);

        if kr.is_err() {
            // Retry without hint.
            addr = 0;
            kr = mach_vm_map_wrapper(&mut addr, size as mach::mach_vm_size_t, VM_FLAGS_ANYWHERE, shared_mem_port, offset, prot);
        }

        match kr {
            Ok(_) => addr as *mut c_void,
            Err(_) => ptr::null_mut(),
        }
    }

    pub fn remap_pages(
        address: *const c_void,
        size: usize,
        new_address: *mut c_void,
        access: MemoryPermission,
    ) -> bool {
        assert!(Self::is_aligned(address as usize, Self::allocate_page_size()));
        assert!(Self::is_aligned(new_address as usize, Self::allocate_page_size()));
        assert!(Self::is_aligned(size, Self::allocate_page_size()));

        let mut cur_protection = GetVMProtFromMemoryPermission(access);
        let mut max_protection: i32 = 0;
        let flags = VM_FLAGS_FIXED | VM_FLAGS_OVERWRITE;
        let mut target = new_address as mach::mach_vm_address_t;
        let ret = unsafe {
            mach::mach_vm_remap(
                mach::mach_task_self(),
                &mut target,
                size as mach::mach_vm_size_t,
                0,
                flags,
                mach::mach_task_self(),
                address as mach::mach_vm_address_t,
                0,
                &mut cur_protection,
                &mut max_protection,
                VM_INHERIT_NONE,
            )
        };

        if ret != KERN_SUCCESS {
            return false;
        }

        // Did we get the address we wanted?
        assert_eq!(new_address, target as *mut c_void);

        true
    }

    fn allocate_page_size() -> usize {
        4096
    }

     fn is_aligned(value: usize, alignment: usize) -> bool {
        value % alignment == 0
    }
}

extern "C" {
    fn _dyld_image_count() -> u32;
    fn _dyld_get_image_header(image_index: u32) -> *const mach_header;
    fn _dyld_get_image_name(image_index: u32) -> *const c_char;
    fn _dyld_get_image_vmaddr_slide(image_index: u32) -> i64;

    #[cfg(target_arch = "x86_64")]
    fn getsectiondata(mhp: *const mach_header_64, segname: *const i8, sectname: *const i8, size: *mut u64) -> *mut u8;

    #[cfg(target_arch = "x86")]
    fn getsectiondata(mhp: *const mach_header, segname: *const i8, sectname: *const i8, size: *mut u64) -> *mut u8;

    fn sysctlbyname(name: *const i8, oldp: *mut c_void, oldlenp: *mut size_t, newp: *const c_void, newlen: size_t) -> i32;
}

#[repr(C)]
struct mach_header {
    magic: u32,
    cpu_type: i32,
    cpu_subtype: i32,
    file_type: u32,
    number_of_commands: u32,
    size_of_commands: u32,
    flags: u32,
}

#[repr(C)]
struct mach_header_64 {
    magic: u32,
    cpu_type: i32,
    cpu_subtype: i32,
    file_type: u32,
    number_of_commands: u32,
    size_of_commands: u32,
    flags: u32,
    reserved: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::ptr;

    #[test]
    fn test_shared_memory_handle() {
        let size = 4096;
        let handle = OS::create_shared_memory_handle_for_testing(size);
        assert!(handle.is_valid());
        OS::destroy_shared_memory_handle(handle);
    }

    #[test]
    fn test_allocate_shared() {
        let size = 4096;
        let handle = OS::create_shared_memory_handle_for_testing(size);
        assert!(handle.is_valid());

        let addr = OS::allocate_shared(
            ptr::null_mut(),
            size,
            MemoryPermission::kReadWrite,
            handle,
            0,
        );

        assert!(!addr.is_null());

        // Deallocate.  Since there's no deallocation function, just leak the memory.

        OS::destroy_shared_memory_handle(handle);
    }
}
