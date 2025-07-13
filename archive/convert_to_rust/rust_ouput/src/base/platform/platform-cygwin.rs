// Converted from V8 C++ source files:
// Header: N/A
// Implementation: platform-cygwin.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

use std::ffi::c_void;
use std::sync::atomic::Ordering;
use std::{mem, ptr, io, ffi::CString};
use std::os::raw::{c_char, c_int};
use winapi::shared::minwindef::{DWORD, LPVOID, SIZE_T};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::memoryapi::{VirtualAlloc, VirtualFree, GetProcAddress, DiscardVirtualMemory};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::shared::ntdef::NULL;
use winapi::um::winnt::{PAGE_NOACCESS, PAGE_READONLY, PAGE_READWRITE, PAGE_EXECUTE_READWRITE, PAGE_EXECUTE_READ, MEM_RESERVE, MEM_COMMIT, MEM_RELEASE, MEM_DECOMMIT, MEM_RESET};
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::GetCurrentProcess;
use winapi::um::psapi::{GetModuleFileNameExW, K32GetModuleInformation, MODULEINFO};
use widestring::U16CString;

use crate::v8::base::PosixTimezoneCache;

pub mod base {
    pub struct SharedLibraryAddresses {
        name: String,
        start: usize,
        end: usize,
    }

    impl SharedLibraryAddresses {
        pub fn new(name: String, start: usize, end: usize) -> Self {
            SharedLibraryAddresses { name, start, end }
        }

        pub fn name(&self) -> &str {
            &self.name
        }

        pub fn start(&self) -> usize {
            self.start
        }

        pub fn end(&self) -> usize {
            self.end
        }
    }
}

pub struct Error {
    message: String,
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error {
            message: format!("{}", err),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MemoryPermission {
    kNoAccess,
    kNoAccessWillJitLater,
    kRead,
    kReadWrite,
    kReadWriteExecute,
    kReadExecute,
}

mod platform_constants {
    pub const KB: usize = 1024;
    pub const MB: usize = 1024 * KB;
    pub const GB: usize = 1024 * MB;
}

mod memory_utils {
    use super::*;

    pub fn aligned_address(address: *mut c_void, alignment: usize) -> *mut c_void {
        let address_int = address as usize;
        let aligned_address_int = (address_int + (alignment - 1)) & !(alignment - 1);
        aligned_address_int as *mut c_void
    }

    pub fn round_up(value: usize, alignment: usize) -> usize {
        (value + (alignment - 1)) & !(alignment - 1)
    }

    pub fn round_down(value: usize, alignment: usize) -> usize {
        value & !(alignment - 1)
    }
}

fn get_protection_from_memory_permission(access: MemoryPermission) -> DWORD {
    match access {
        MemoryPermission::kNoAccess => PAGE_NOACCESS,
        MemoryPermission::kNoAccessWillJitLater => PAGE_NOACCESS,
        MemoryPermission::kRead => PAGE_READONLY,
        MemoryPermission::kReadWrite => PAGE_READWRITE,
        MemoryPermission::kReadWriteExecute => PAGE_EXECUTE_READWRITE,
        MemoryPermission::kReadExecute => PAGE_EXECUTE_READ,
    }
}

fn randomized_virtual_alloc(size: usize, flags: DWORD, protect: DWORD, hint: *mut c_void) -> *mut u8 {
    let mut base: LPVOID = std::ptr::null_mut();

    if protect != PAGE_READWRITE {
        base = unsafe { VirtualAlloc(hint, size as SIZE_T, flags, protect) };
    }

    if base.is_null() {
        base = unsafe { VirtualAlloc(std::ptr::null_mut(), size as SIZE_T, flags, protect) };
    }

    base as *mut u8
}

struct CygwinTimezoneCache {}

impl CygwinTimezoneCache {
    fn LocalTimezone(time: f64) -> String {
        if time.is_nan() {
            return "".to_string();
        }
        let tv = time.floor() as i64;
        
        //TODO: Implement timezone lookup.
        "UTC".to_string()
    }

    fn LocalTimeOffset(time_ms: f64, is_utc: bool) -> f64 {
        //TODO: Implement timezone offset calculation.
        0.0
    }
}

pub struct OS {}

impl OS {
    pub fn allocate(hint: *mut c_void, size: usize, alignment: usize, access: MemoryPermission) -> *mut c_void {
        let page_size = OS::allocate_page_size();
        assert_eq!(0, size % page_size);
        assert_eq!(0, alignment % page_size);
        assert!(page_size <= alignment);
        let hint = memory_utils::aligned_address(hint, alignment);

        let flags = if access == MemoryPermission::kNoAccess {
            MEM_RESERVE
        } else {
            MEM_RESERVE | MEM_COMMIT
        };
        let protect = get_protection_from_memory_permission(access);

        let mut base = randomized_virtual_alloc(size, flags, protect, hint);
        if base.is_null() {
            return std::ptr::null_mut();
        }

        let aligned_base = memory_utils::aligned_address(base as *mut c_void, alignment) as *mut u8;
        if base == aligned_base {
            return base as *mut c_void;
        }

        OS::free(base as *mut c_void, size);

        let mut hint = std::ptr::null_mut();

        let padded_size = size + (alignment - page_size);
        const MAX_ATTEMPTS: i32 = 3;
        let mut aligned_base: *mut u8 = std::ptr::null_mut();

        for _i in 0..MAX_ATTEMPTS {
            base = randomized_virtual_alloc(padded_size, flags, protect, hint);
            if base.is_null() {
                return std::ptr::null_mut();
            }

            OS::free(base as *mut c_void, padded_size);
            aligned_base = memory_utils::aligned_address(base as *mut c_void, alignment) as *mut u8;

            let new_base = unsafe { VirtualAlloc(aligned_base as *mut c_void, size as SIZE_T, flags, protect) };

            if !new_base.is_null() {
                base = new_base as *mut u8;
                break;
            }
        }

        if base.is_null() {
          return std::ptr::null_mut();
        }

        base as *mut c_void
    }

    pub fn free(address: *mut c_void, size: usize) {
        assert_eq!(0, address as usize % OS::allocate_page_size());
        assert_eq!(0, size % OS::allocate_page_size());

        let result = unsafe { VirtualFree(address, 0, MEM_RELEASE) };
        assert_ne!(0, result);
    }

    pub fn release(address: *mut c_void, size: usize) {
        assert_eq!(0, address as usize % OS::commit_page_size());
        assert_eq!(0, size % OS::commit_page_size());

        let result = unsafe { VirtualFree(address, size as SIZE_T, MEM_DECOMMIT) };
        assert_ne!(0, result);
    }

    pub fn set_permissions(address: *mut c_void, size: usize, access: MemoryPermission) -> bool {
        assert_eq!(0, address as usize % OS::commit_page_size());
        assert_eq!(0, size % OS::commit_page_size());

        if access == MemoryPermission::kNoAccess {
            return unsafe { VirtualFree(address, size as SIZE_T, MEM_DECOMMIT) != 0 };
        }

        let protect = get_protection_from_memory_permission(access);
        unsafe { VirtualAlloc(address, size as SIZE_T, MEM_COMMIT, protect) != NULL }
    }

    pub fn recommit_pages(address: *mut c_void, size: usize, access: MemoryPermission) -> bool {
        OS::set_permissions(address, size, access)
    }

    pub fn discard_system_pages(address: *mut c_void, size: usize) -> bool {
        type DiscardVirtualMemoryFunction = unsafe extern "system" fn(virtual_address: *mut c_void, size: SIZE_T) -> DWORD;
        static mut DISCARD_VIRTUAL_MEMORY: Option<DiscardVirtualMemoryFunction> = None;

        unsafe {
            if DISCARD_VIRTUAL_MEMORY.is_none() {
                let module_name = U16CString::from_str("Kernel32.dll").unwrap();
                let module_handle = GetModuleHandleW(module_name.as_ptr());
                
                let proc_name = CString::new("DiscardVirtualMemory").unwrap();
                let function_ptr = GetProcAddress(module_handle, proc_name.as_ptr());

                DISCARD_VIRTUAL_MEMORY = function_ptr.map(|ptr| mem::transmute(ptr));
            }

            if let Some(discard_function) = DISCARD_VIRTUAL_MEMORY {
                let ret = discard_function(address, size as SIZE_T);
                if ret == 0 {
                    return true;
                }
            }

            let ptr = VirtualAlloc(address, size as SIZE_T, MEM_RESET, PAGE_READWRITE);
            assert!(!ptr.is_null());
            !ptr.is_null()
        }
    }

    pub fn seal_pages(address: *mut c_void, size: usize) -> bool {
        false
    }

    pub fn has_lazy_commits() -> bool {
        false
    }

    pub fn get_shared_library_addresses() -> Vec<base::SharedLibraryAddresses> {
        let mut result = Vec::new();
        
        if let Ok(file) = std::fs::File::open("/proc/self/maps") {
            let reader = std::io::BufReader::new(file);

            for line_result in std::io::BufRead::lines(reader) {
                if let Ok(line) = line_result {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() < 5 {
                        continue;
                    }
                    
                    let address_range = parts[0];
                    let permissions = parts[1];

                    if permissions.starts_with("r") && !permissions.contains("w") && permissions.contains("x") {
                        let address_parts: Vec<&str> = address_range.split("-").collect();
                        if address_parts.len() == 2 {
                            if let (Ok(start), Ok(end)) = (usize::from_str_radix(address_parts[0], 16), usize::from_str_radix(address_parts[1], 16)) {
                                let lib_name = if parts.len() > 5 {
                                    parts[5..].join(" ")
                                } else {
                                    format!("{:x}-{:x}", start, end)
                                };
                                result.push(base::SharedLibraryAddresses::new(lib_name, start, end));
                            }
                        }
                    }
                }
            }
        }
        result
    }

    pub fn signal_code_moving_gc() {}

    pub fn adjust_scheduling_params() {}

    pub fn get_first_free_memory_range_within(boundary_start: *mut c_void, boundary_end: *mut c_void, minimum_size: usize, alignment: usize) -> Option<(*mut c_void, *mut c_void)> {
        None
    }

    fn allocate_page_size() -> usize {
        4096
    }

    fn commit_page_size() -> usize {
        4096
    }
}
