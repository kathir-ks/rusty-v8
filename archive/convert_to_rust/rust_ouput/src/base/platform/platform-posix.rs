// Converted from V8 C++ source files:
// Header: platform-posix.h
// Implementation: platform-posix.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
pub mod platform {
use std::ffi::c_void;
use std::time::Duration;
use std::{
    io,
    sync::{Mutex, MutexGuard},
};

    // Dummy structures and enums to satisfy dependencies.
    pub struct TimezoneCache {}
    pub enum MemoryPermission {
        kNoAccess,
        kNoAccessWillJitLater,
        kRead,
        kReadWrite,
        kReadExecute,
        kReadWriteExecute,
    }
    pub struct PlatformSharedMemoryHandle {}
    pub enum FileMode {
        kReadOnly,
        kReadWrite,
    }

    pub enum AbortMode {
        kDefault,
        kExitWithSuccessAndIgnoreDcheckFailures,
        kExitWithFailureAndIgnoreDcheckFailures,
        kImmediateCrash,
    }

    pub struct AddressSpaceReservation {
        base: *mut c_void,
        size: usize,
    }

    impl AddressSpaceReservation {
        pub fn base(&self) -> *mut c_void {
            self.base
        }
        pub fn size(&self) -> usize {
            self.size
        }
        pub fn contains(&self, address: *mut c_void, size: usize) -> bool {
            let start = self.base as usize;
            let end = start + self.size;
            let region_start = address as usize;
            let region_end = region_start + size;
            region_start >= start && region_end <= end
        }
        pub fn create_sub_reservation(
            &self,
            address: *mut c_void,
            size: usize,
            max_permission: MemoryPermission,
        ) -> Option<AddressSpaceReservation> {
            if self.contains(address, size) {
                Some(AddressSpaceReservation {
                    base: address,
                    size,
                })
            } else {
                None
            }
        }

        pub fn free_sub_reservation(&self, reservation: AddressSpaceReservation) -> bool {
            true
        }

        pub fn allocate(&self, address: *mut c_void, size: usize, access: MemoryPermission) -> bool {
            if self.contains(address, size) {
                unsafe {
                    let prot = get_protection_from_memory_permission(access);
                    let result = libc::mprotect(address, size, prot);
                    return result == 0;
                }
            }
            false
        }

        pub fn free(&self, address: *mut c_void, size: usize) -> bool {
            if self.contains(address, size) {
                unsafe {
                    let result = libc::mmap(
                        address,
                        size,
                        libc::PROT_NONE,
                        libc::MAP_FIXED | libc::MAP_ANONYMOUS | libc::MAP_PRIVATE,
                        -1,
                        0,
                    );
                    return result == address;
                }
            }
            false
        }

        pub fn allocate_shared(
            &self,
            address: *mut c_void,
            size: usize,
            access: MemoryPermission,
            handle: PlatformSharedMemoryHandle,
            offset: u64,
        ) -> bool {
            if self.contains(address, size) {
                unsafe {
                    let prot = get_protection_from_memory_permission(access);
                    let fd = file_descriptor_from_shared_memory_handle(handle);
                    let result = libc::mmap(
                        address,
                        size,
                        prot,
                        libc::MAP_SHARED | libc::MAP_FIXED,
                        fd,
                        offset as i64,
                    );
                    return result != libc::MAP_FAILED;
                }
            }
            false
        }

        pub fn free_shared(&self, address: *mut c_void, size: usize) -> bool {
            if self.contains(address, size) {
                unsafe {
                    let result = libc::mmap(
                        address,
                        size,
                        libc::PROT_NONE,
                        libc::MAP_FIXED | libc::MAP_ANONYMOUS | libc::MAP_PRIVATE,
                        -1,
                        0,
                    );
                    return result == address;
                }
            }
            false
        }

        pub fn set_permissions(&self, address: *mut c_void, size: usize, access: MemoryPermission) -> bool {
            if self.contains(address, size) {
                unsafe {
                    let prot = get_protection_from_memory_permission(access);
                    let result = libc::mprotect(address, size, prot);
                    return result == 0;
                }
            }
            false
        }
        pub fn recommit_pages(&self, address: *mut c_void, size: usize, access: MemoryPermission) -> bool {
            if self.contains(address, size) {
                true
            } else {
                false
            }
        }
        pub fn discard_system_pages(&self, address: *mut c_void, size: usize) -> bool {
            if self.contains(address, size) {
                discard_system_pages(address, size)
            } else {
                false
            }
        }
        pub fn decommit_pages(&self, address: *mut c_void, size: usize) -> bool {
            if self.contains(address, size) {
                unsafe {
                    let result = libc::mmap(
                        address,
                        size,
                        libc::PROT_NONE,
                        libc::MAP_FIXED | libc::MAP_ANONYMOUS | libc::MAP_PRIVATE,
                        -1,
                        0,
                    );
                    return result == address;
                }
            }
            false
        }
    }

    pub enum OS_Error {
        IO(io::Error),
        Other(String),
    }

    impl From<io::Error> for OS_Error {
        fn from(err: io::Error) -> Self {
            OS_Error::IO(err)
        }
    }
    extern "C" {
        fn getpid() -> i32;
    }

    static mut g_abort_mode: AbortMode = AbortMode::kDefault;
    static mut g_gc_fake_mmap: *const i8 = std::ptr::null();

    pub fn posix_initialize_common(abort_mode: AbortMode, gc_fake_mmap: *const i8) {
        unsafe {
            g_abort_mode = abort_mode;
            g_gc_fake_mmap = gc_fake_mmap;
        }
    }

    pub fn initialize(abort_mode: AbortMode, gc_fake_mmap: *const i8) {
        posix_initialize_common(abort_mode, gc_fake_mmap);
    }

    pub fn is_hardware_enforced_shadow_stacks_enabled() -> bool {
        false
    }

    pub fn activation_frame_alignment() -> i32 {
        16
    }

    pub fn allocate_page_size() -> usize {
        unsafe { libc::sysconf(libc::_SC_PAGESIZE) as usize }
    }

    pub fn commit_page_size() -> usize {
        allocate_page_size()
    }

    pub fn set_random_mmap_seed(seed: i64) {
    }

    pub fn get_random_mmap_addr() -> *mut c_void {
        std::ptr::null_mut()
    }

    unsafe fn get_flags_for_memory_permission(access: MemoryPermission, page_type: PageType) -> i32 {
        let mut flags = libc::MAP_ANONYMOUS;
        flags |= match page_type {
            PageType::kShared => libc::MAP_SHARED,
            PageType::kPrivate => libc::MAP_PRIVATE,
        };

        if access == MemoryPermission::kNoAccess || access == MemoryPermission::kNoAccessWillJitLater {
                flags |= libc::MAP_NORESERVE;
        }
    
        #[cfg(target_os = "darwin")]
        if access == MemoryPermission::kNoAccessWillJitLater || access == MemoryPermission::kReadWriteExecute {
            flags |= libc::MAP_JIT;
        }
    
        flags
    }

    pub fn allocate(
        hint: *mut c_void,
        size: usize,
        alignment: usize,
        access: MemoryPermission,
    ) -> Result<*mut c_void, String> {
        let page_size = allocate_page_size();
        if size % page_size != 0 || alignment % page_size != 0 {
            return Err("Size and alignment must be multiples of page size".to_string());
        }
        let aligned_hint = align_address(hint, alignment);
        let request_size = size + (alignment - page_size);
        let request_size = round_up(request_size, allocate_page_size());

        unsafe {
                let prot = get_protection_from_memory_permission(access);
                let flags = get_flags_for_memory_permission(access, PageType::kPrivate);
                let result = libc::mmap(aligned_hint, request_size, prot, flags, -1, 0);

                if result == libc::MAP_FAILED {
                    return Err("mmap failed".to_string());
                }

                let base = result as *mut u8;
                let aligned_base = round_up(base as usize, alignment) as *mut u8;

                if aligned_base != base {
                    let prefix_size = aligned_base.offset_from(base) as usize;
                    free_internal(base as *mut c_void, prefix_size);
                }

                if size != request_size {
                    let suffix_size = request_size - size;
                    free_internal((aligned_base as *mut u8).add(size) as *mut c_void, suffix_size);
                }
            
                Ok(aligned_base as *mut c_void)
        }
    }

    pub fn allocate_shared(size: usize, access: MemoryPermission) -> Result<*mut c_void, String> {
        if size % allocate_page_size() != 0 {
            return Err("Size must be a multiple of page size".to_string());
        }

        unsafe {
            let prot = get_protection_from_memory_permission(access);
            let flags = get_flags_for_memory_permission(access, PageType::kShared);
            let result = libc::mmap(
                std::ptr::null_mut(),
                size,
                prot,
                flags,
                -1,
                0,
            );

            if result == libc::MAP_FAILED {
                return Err("mmap failed".to_string());
            }

            Ok(result)
        }
    }

    unsafe fn free_internal(address: *mut c_void, size: usize) {
        let result = libc::munmap(address, size);
        if result != 0 {
           eprintln!("munmap failed: {}", std::io::Error::last_os_error());
        }
    }

    pub fn free(address: *mut c_void, size: usize) {
        unsafe {
            free_internal(address, size);
        }
    }

    pub fn allocate_shared_with_handle(
        hint: *mut c_void,
        size: usize,
        access: MemoryPermission,
        handle: PlatformSharedMemoryHandle,
        offset: u64,
    ) -> Result<*mut c_void, String> {
        unsafe {
            let prot = get_protection_from_memory_permission(access);
            let fd = file_descriptor_from_shared_memory_handle(handle);
            let result = libc::mmap(hint, size, prot, libc::MAP_SHARED, fd, offset as i64);
            if result == libc::MAP_FAILED {
                return Err("mmap failed".to_string());
            }
            Ok(result)
        }
    }

    pub fn free_shared(address: *mut c_void, size: usize) {
        unsafe {
            let result = libc::munmap(address, size);
            if result != 0 {
                eprintln!("munmap failed: {}", std::io::Error::last_os_error());
            }
        }
    }

    pub fn release(address: *mut c_void, size: usize) {
        unsafe {
            let result = libc::munmap(address, size);
            if result != 0 {
                eprintln!("munmap failed: {}", std::io::Error::last_os_error());
            }
        }
    }

    pub fn set_permissions(address: *mut c_void, size: usize, access: MemoryPermission) -> bool {
        unsafe {
            let prot = get_protection_from_memory_permission(access);
            let result = libc::mprotect(address, size, prot);
            result == 0
        }
    }

    pub fn set_data_read_only(address: *mut c_void, size: usize) {
        unsafe {
            if libc::mprotect(address, size, libc::PROT_READ) != 0 {
                panic!(
                    "Failed to protect data memory at {:p} +{}; error {}",
                    address,
                    size,
                    std::io::Error::last_os_error()
                );
            }
        }
    }

    pub fn recommit_pages(address: *mut c_void, size: usize, access: MemoryPermission) -> bool {
        true
    }

    pub fn discard_system_pages(address: *mut c_void, size: usize) -> bool {
        discard_system_pages(address, size)
    }
    
    fn discard_system_pages(address: *mut c_void, size: usize) -> bool {
        unsafe {
            let ret = libc::madvise(address, size, libc::MADV_DONTNEED);
            ret == 0
        }
    }

    pub fn decommit_pages(address: *mut c_void, size: usize) -> bool {
        unsafe {
            let ret = libc::mmap(
                address,
                size,
                libc::PROT_NONE,
                libc::MAP_FIXED | libc::MAP_ANONYMOUS | libc::MAP_PRIVATE,
                -1,
                0,
            );
            if ret == libc::MAP_FAILED {
                return false;
            }
            ret == address
        }
    }
    
    pub fn seal_pages(address: *mut c_void, size: usize) -> bool {
        false
    }
    
    pub fn can_reserve_address_space() -> bool {
        true
    }
    
    pub fn create_address_space_reservation(
        hint: *mut c_void,
        size: usize,
        alignment: usize,
        max_permission: MemoryPermission,
    ) -> Option<AddressSpaceReservation> {
        let permission = if max_permission == MemoryPermission::kReadWriteExecute {
            MemoryPermission::kNoAccessWillJitLater
        } else {
            MemoryPermission::kNoAccess
        };
        
        match allocate(hint, size, alignment, permission) {
            Ok(reservation) => Some(AddressSpaceReservation {
                base: reservation,
                size,
            }),
            Err(_) => None,
        }
    }

    pub fn free_address_space_reservation(reservation: AddressSpaceReservation) {
        free(reservation.base(), reservation.size());
    }
    
    pub fn has_lazy_commits() -> bool {
        true
    }

    pub fn get_gc_fake_mmap_file() -> *const i8 {
        unsafe { g_gc_fake_mmap }
    }

    pub fn sleep(interval: Duration) {
        unsafe {
            libc::usleep(interval.as_micros() as u32);
        }
    }

    pub fn abort() -> ! {
        unsafe {
            match g_abort_mode {
                AbortMode::kExitWithSuccessAndIgnoreDcheckFailures => libc::_exit(0),
                AbortMode::kExitWithFailureAndIgnoreDcheckFailures => libc::_exit(-1),
                AbortMode::kImmediateCrash => std::process::abort(),
                AbortMode::kDefault => {}
            }
            libc::abort();
        }
    }

    pub fn debug_break() {}

    pub fn get_current_process_id() -> i32 {
        unsafe { getpid() }
    }

    pub fn get_current_thread_id_internal() -> i32 {
         0
    }

    pub fn exit_process(exit_code: i32) -> ! {
        unsafe {
            libc::_exit(exit_code);
        }
    }

    pub fn get_user_time(secs: &mut u32, usecs: &mut u32) -> i32 {
        0
    }

    pub fn get_peak_memory_usage_kb() -> i32 {
        -1
    }

    pub fn time_current_millis() -> f64 {
        0.0
    }

    pub fn get_last_error() -> i32 {
        unsafe { libc::errno }
    }

    pub fn fopen(path: &str, mode: &str) -> *mut libc::FILE {
        unsafe {
            let c_path = std::ffi::CString::new(path).unwrap();
            let c_mode = std::ffi::CString::new(mode).unwrap();
            libc::fopen(c_path.as_ptr(), c_mode.as_ptr())
        }
    }

    pub fn remove(path: &str) -> bool {
        unsafe {
            let c_path = std::ffi::CString::new(path).unwrap();
            libc::remove(c_path.as_ptr()) == 0
        }
    }

    pub fn directory_separator() -> char {
        '/'
    }

    pub fn is_directory_separator(ch: char) -> bool {
        ch == directory_separator()
    }

    pub fn open_temporary_file() -> *mut libc::FILE {
        unsafe { libc::tmpfile() }
    }

    pub const LOG_FILE_OPEN_MODE: &str = "w+";

    pub fn print(format: &str, args: Vec<std::ffi::CString>) {}

    pub fn vprint(format: &str, args: Vec<*mut i8>) {}

    pub fn fprintf(out: *mut libc::FILE, format: &str, args: Vec<std::ffi::CString>) {}

    pub fn vfprintf(out: *mut libc::FILE, format: &str, args: *mut *mut i8) {}

    pub fn printerror(format: &str, args: Vec<std::ffi::CString>) {}

    pub fn vprinterror(format: &str, args: *mut *mut i8) {}

    pub fn snprintf(str: &mut [u8], format: &str, args: Vec<std::ffi::CString>) -> i32 {
        0
    }

    pub fn vsnprintf(str: &mut [u8], format: &str, args: *mut *mut i8) -> i32 {
        0
    }

    pub fn strncpy(dest: &mut [u8], src: &str, n: usize) {}

    #[allow(dead_code)]
    enum PageType {
        kShared,
        kPrivate,
    }

    unsafe fn get_protection_from_memory_permission(access: MemoryPermission) -> i32 {
        match access {
            MemoryPermission::kNoAccess => libc::PROT_NONE,
            MemoryPermission::kNoAccessWillJitLater => libc::PROT_NONE,
            MemoryPermission::kRead => libc::PROT_READ,
            MemoryPermission::kReadWrite => libc::PROT_READ | libc::PROT_WRITE,
            MemoryPermission::kReadWriteExecute => {
                libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC
            }
            MemoryPermission::kReadExecute => libc::PROT_READ | libc::PROT_EXEC,
        }
    }

    fn align_address(hint: *mut c_void, alignment: usize) -> *mut c_void {
        if hint.is_null() {
            std::ptr::null_mut()
        } else {
            round_up(hint as usize, alignment) as *mut c_void
        }
    }

    fn round_up(x: usize, alignment: usize) -> usize {
        (x + alignment - 1) & !(alignment - 1)
    }

    fn file_descriptor_from_shared_memory_handle(handle: PlatformSharedMemoryHandle) -> i32 {
        0
    }
}

pub mod time {
    pub struct TimeDelta {}
    impl TimeDelta {
        pub fn in_microseconds(&self) -> i64 {
            0
        }
    }

    pub struct Time {}
    impl Time {
        pub fn now() -> Self {
            Time {}
        }
        pub fn to_js_time(&self) -> f64 {
            0.0
        }
    }
}

pub mod utils {
    pub struct RandomNumberGenerator {}
    impl RandomNumberGenerator {
        pub fn next_bytes(&mut self, raw_addr: &mut usize, size: usize) {}
        pub fn set_seed(&mut self, seed: i64) {}
    }
}
}
