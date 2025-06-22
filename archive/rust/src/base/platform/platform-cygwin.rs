use std::{
    alloc::{alloc, dealloc, Layout},
    cmp::{max, min},
    ffi::{CStr, CString},
    fmt,
    fs::File,
    io::{BufRead, BufReader},
    mem::{self, MaybeUninit},
    num::NonZeroUsize,
    os::raw::{c_char, c_int, c_long, c_void},
    path::Path,
    ptr,
    slice,
    sync::atomic::{AtomicPtr, Ordering},
};

use libc::{
    fclose, fopen, free, getc, mktime, snprintf, time, ungetc, FILENAME_MAX,
    MAP_FAILED, MEM_RESET, PAGE_EXECUTE_READ, PAGE_EXECUTE_READWRITE,
    PAGE_NOACCESS, PAGE_READONLY, PAGE_READWRITE, V8PRIxPTR,
};
use winapi::{
    shared::minwindef::{DWORD, LPVOID, PVOID, SIZE_T},
    um::{
        libloaderapi::GetModuleHandleW, memoryapi::VirtualAlloc,
        memoryapi::VirtualFree, processthreadsapi::GetProcAddress,
        winnt::MEM_COMMIT, winnt::MEM_DECOMMIT, winnt::MEM_RELEASE,
        winnt::MEM_RESERVE,
    },
};

const msPerSecond: f64 = 1000.0;

macro_rules! UNREACHABLE {
    () => {
        panic!("This code should be unreachable");
    };
}

// TODO: Implement a proper AlignedAddress function
fn aligned_address(hint: *mut c_void, alignment: usize) -> *mut c_void {
    // For now, just return the hint as is
    hint
}

fn round_up(addr: *mut u8, alignment: usize) -> *mut u8 {
    let addr_int = addr as usize;
    let remainder = addr_int % alignment;
    if remainder == 0 {
        addr
    } else {
        ((addr_int + alignment - remainder) as *mut u8)
    }
}

/// Represents memory permissions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemoryPermission {
    kNoAccess,
    kNoAccessWillJitLater,
    kRead,
    kReadWrite,
    kReadExecute,
    kReadWriteExecute,
}

fn get_protection_from_memory_permission(access: MemoryPermission) -> DWORD {
    match access {
        MemoryPermission::kNoAccess | MemoryPermission::kNoAccessWillJitLater => {
            PAGE_NOACCESS
        }
        MemoryPermission::kRead => PAGE_READONLY,
        MemoryPermission::kReadWrite => PAGE_READWRITE,
        MemoryPermission::kReadWriteExecute => PAGE_EXECUTE_READWRITE,
        MemoryPermission::kReadExecute => PAGE_EXECUTE_READ,
    }
}

fn randomized_virtual_alloc(size: usize, flags: DWORD, protect: DWORD, hint: *mut c_void) -> *mut u8 {
    unsafe {
        let mut base: LPVOID = ptr::null_mut();

        // For executable or reserved pages try to use the address hint.
        if protect != PAGE_READWRITE {
            base = VirtualAlloc(hint, size as SIZE_T, flags, protect);
        }

        // If that fails, let the OS find an address to use.
        if base.is_null() {
            base = VirtualAlloc(ptr::null_mut(), size as SIZE_T, flags, protect);
        }

        base as *mut u8
    }
}

/// Provides platform-specific functionalities.
pub struct OS {}

impl OS {
    /// Allocates memory with the given hint, size, alignment, and memory permission.
    pub fn allocate(
        hint: *mut c_void,
        size: usize,
        alignment: usize,
        access: MemoryPermission,
    ) -> *mut c_void {
        let page_size = OS::allocate_page_size();
        assert_eq!(0, size % page_size);
        assert_eq!(0, alignment % page_size);
        assert!(page_size <= alignment);
        let hint = aligned_address(hint, alignment);

        let flags = if access == MemoryPermission::kNoAccess {
            MEM_RESERVE
        } else {
            MEM_RESERVE | MEM_COMMIT
        };
        let protect = get_protection_from_memory_permission(access);

        // First, try an exact size aligned allocation.
        let mut base = randomized_virtual_alloc(size, flags, protect, hint);
        if base.is_null() {
            return ptr::null_mut(); // Can't allocate, we're OOM.
        }

        // If address is suitably aligned, we're done.
        let aligned_base = round_up(base, alignment);
        if base == aligned_base {
            return base as *mut c_void;
        }

        // Otherwise, free it and try a larger allocation.
        OS::free(base as *mut c_void, size);

        // Clear the hint. It's unlikely we can allocate at this address.
        // hint = ptr::null_mut(); // not needed, it's already dropped in Rust

        // Add the maximum misalignment so we are guaranteed an aligned base address
        // in the allocated region.
        let padded_size = size + (alignment - page_size);
        const K_MAX_ATTEMPTS: usize = 3;
        let mut aligned_base: *mut u8 = ptr::null_mut();
        let mut base: *mut u8 = ptr::null_mut();
        for _i in 0..K_MAX_ATTEMPTS {
            base = randomized_virtual_alloc(padded_size, flags, protect, hint);
            if base.is_null() {
                return ptr::null_mut(); // Can't allocate, we're OOM.
            }

            // Try to trim the allocation by freeing the padded allocation and then
            // calling VirtualAlloc at the aligned base.
            OS::free(base as *mut c_void, padded_size);
            aligned_base = round_up(base, alignment);
            unsafe {
                base = VirtualAlloc(aligned_base as LPVOID, size as SIZE_T, flags, protect) as *mut u8;
            }
            // We might not get the reduced allocation due to a race. In that case,
            // base will be nullptr.
            if !base.is_null() {
                break;
            }
        }

        if !base.is_null() {
            assert_eq!(base, aligned_base);
        }
        base as *mut c_void
    }

    /// Frees the memory at the given address of the given size.
    pub fn free(address: *mut c_void, size: usize) {
        let page_size = OS::allocate_page_size();
        assert_eq!(0, address as usize % page_size);
        assert_eq!(0, size % page_size);
        unsafe {
            assert_ne!(0, VirtualFree(address, 0, MEM_RELEASE));
        }
    }

    /// Releases the memory at the given address of the given size.
    pub fn release(address: *mut c_void, size: usize) {
        let commit_page_size = OS::commit_page_size();
        assert_eq!(0, address as usize % commit_page_size);
        assert_eq!(0, size % commit_page_size);
        unsafe {
            assert_ne!(0, VirtualFree(address, size as SIZE_T, MEM_DECOMMIT));
        }
    }

    /// Sets the permissions of the memory at the given address of the given size.
    pub fn set_permissions(
        address: *mut c_void,
        size: usize,
        access: MemoryPermission,
    ) -> bool {
        let commit_page_size = OS::commit_page_size();
        assert_eq!(0, address as usize % commit_page_size);
        assert_eq!(0, size % commit_page_size);

        if access == MemoryPermission::kNoAccess {
            unsafe {
                return VirtualFree(address, size as SIZE_T, MEM_DECOMMIT) != 0;
            }
        }

        let protect = get_protection_from_memory_permission(access);
        unsafe {
            VirtualAlloc(address, size as SIZE_T, MEM_COMMIT, protect).is_null() == false
        }
    }

    /// Recommits pages.
    pub fn recommit_pages(
        address: *mut c_void,
        size: usize,
        access: MemoryPermission,
    ) -> bool {
        OS::set_permissions(address, size, access)
    }

    /// Discards system pages.
    pub fn discard_system_pages(address: *mut c_void, size: usize) -> bool {
        // On Windows, discarded pages are not returned to the system immediately and
        // not guaranteed to be zeroed when returned to the application.

        type DiscardVirtualMemoryFunction =
            unsafe extern "system" fn(virtualAddress: PVOID, size: SIZE_T) -> DWORD;

        static DISCARD_VIRTUAL_MEMORY: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());

        unsafe {
            if DISCARD_VIRTUAL_MEMORY.load(Ordering::Relaxed).is_null() {
                let module_name = "Kernel32.dll".encode_utf16().collect::<Vec<_>>();
                let module_handle = GetModuleHandleW(module_name.as_ptr());
                let proc_name = CString::new("DiscardVirtualMemory").unwrap();
                let proc_addr = GetProcAddress(module_handle, proc_name.as_ptr());
                DISCARD_VIRTUAL_MEMORY.store(proc_addr as *mut c_void, Ordering::Relaxed);
            }

            let discard_function = DISCARD_VIRTUAL_MEMORY.load(Ordering::Relaxed) as DiscardVirtualMemoryFunction;
            if !discard_function.is_null() {
                let ret = discard_function(address as PVOID, size as SIZE_T);
                if ret == 0 {
                    return true;
                }
            }

            // DiscardVirtualMemory is buggy in Win10 SP0, so fall back to MEM_RESET on failure.
            let ptr = VirtualAlloc(address, size as SIZE_T, MEM_RESET, PAGE_READWRITE);
            assert!(!ptr.is_null());
            ptr.is_null() == false
        }
    }

    /// Seals pages.
    pub fn seal_pages(_address: *mut c_void, _size: usize) -> bool {
        false
    }

    /// Checks if lazy commits are enabled.
    pub fn has_lazy_commits() -> bool {
        // TODO(alph): implement for the platform.
        false
    }

    /// Gets the shared library addresses.
    pub fn get_shared_library_addresses() -> Vec<SharedLibraryAddress> {
        let mut result = Vec::new();

        // This function assumes that the layout of the file is as follows:
        // hex_start_addr-hex_end_addr rwxp <unused data> [binary_file_name]
        // If we encounter an unexpected situation we abort scanning further entries.
        let file = unsafe { fopen(CString::new("/proc/self/maps").unwrap().as_ptr(), CString::new("r").unwrap().as_ptr()) };
        if file.is_null() {
            return result;
        }

        // Allocate enough room to be able to store a full file name.
        let k_lib_name_len = (FILENAME_MAX + 1) as usize;
        let mut lib_name_buffer: Vec<u8> = vec![0; k_lib_name_len];
        let lib_name = lib_name_buffer.as_mut_ptr() as *mut c_char;

        // This loop will terminate once the scanning hits an EOF.
        loop {
            let mut start: u64 = 0;
            let mut end: u64 = 0;
            let mut attr_r: c_char = 0;
            let mut attr_w: c_char = 0;
            let mut attr_x: c_char = 0;
            let mut attr_p: c_char = 0;

            // Parse the addresses and permission bits at the beginning of the line.
            unsafe {
                if libc::fscanf(file, b"%lx-%lx\0" as *const _ as *const c_char, &mut start, &mut end) != 2 {
                    break;
                }
                if libc::fscanf(file, b" %c%c%c%c\0" as *const _ as *const c_char, &mut attr_r, &mut attr_w, &mut attr_x, &mut attr_p) != 4 {
                    break;
                }
            }

            let mut c: c_int;
            if attr_r == 'r' as i8 && attr_w != 'w' as i8 && attr_x == 'x' as i8 {
                // Found a read-only executable entry. Skip characters until we reach
                // the beginning of the filename or the end of the line.
                loop {
                    unsafe {
                        c = getc(file);
                    }
                    if c == libc::EOF || c == '\n' as i32 || c == '/' as i32 {
                        break;
                    }
                }
                if c == libc::EOF {
                    break; // EOF: Was unexpected, just exit.
                }

                // Process the filename if found.
                if c == '/' as i32 {
                    unsafe {
                        ungetc(c, file);  // Push the '/' back into the stream to be read below.
                    }

                    // Read to the end of the line. Exit if the read fails.
                    let read_result = unsafe { libc::fgets(lib_name, k_lib_name_len as c_int, file) };
                    if read_result.is_null() {
                        break;
                    }

                    // Drop the newline character read by fgets. We do not need to check
                    // for a zero-length string because we know that we at least read the
                    // '/' character.
                    let len = unsafe { libc::strlen(lib_name) };
                    unsafe {
                        *lib_name.add(len - 1) = 0;
                    }
                } else {
                    // No library name found, just record the raw address range.
                    let format_str = CString::new(format!("%08x-%08x", start, end)).unwrap();
                     unsafe {
                        libc::snprintf(lib_name, k_lib_name_len as usize, format_str.as_ptr());
                    }
                }
                let cstr = unsafe { CStr::from_ptr(lib_name) };
                let lib_name_string = cstr.to_string_lossy().into_owned();

                result.push(SharedLibraryAddress {
                    name: lib_name_string,
                    start,
                    end,
                });
            } else {
                // Entry not describing executable data. Skip to end of line to set up
                // reading the next entry.
                loop {
                    unsafe {
                        c = getc(file);
                    }
                    if c == libc::EOF || c == '\n' as i32 {
                        break;
                    }
                }
                if c == libc::EOF {
                    break;
                }
            }
        }
         unsafe {
             libc::free(lib_name as *mut c_void);
            fclose(file);
        }
        result
    }

    /// Signals code moving GC.
    pub fn signal_code_moving_gc() {
        // Nothing to do on Cygwin.
    }

    /// Adjusts scheduling parameters.
    pub fn adjust_scheduling_params() {}

    /// Gets the first free memory range within the given boundaries.
    pub fn get_first_free_memory_range_within(
        _boundary_start: *mut c_void,
        _boundary_end: *mut c_void,
        _minimum_size: usize,
        _alignment: usize,
    ) -> Option<MemoryRange> {
        None
    }

    /// Gets the allocate page size.
    pub fn allocate_page_size() -> usize {
        4096
    }

    /// Gets the commit page size.
    pub fn commit_page_size() -> usize {
        4096
    }
}

/// Represents a shared library address.
#[derive(Debug)]
pub struct SharedLibraryAddress {
    pub name: String,
    pub start: u64,
    pub end: u64,
}

/// Represents a memory range.
#[derive(Debug)]
pub struct MemoryRange {
    pub start: *mut c_void,
    pub size: usize,
}

// TODO: Implement timezone cache functionality.  This is a stub for now.
struct CygwinTimezoneCache {}

impl CygwinTimezoneCache {
    fn local_timezone(&self, _time: f64) -> &str {
        ""
    }

    fn local_time_offset(&self, _time_ms: f64, _is_utc: bool) -> f64 {
        0.0
    }
}