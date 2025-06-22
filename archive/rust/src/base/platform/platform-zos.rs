// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Platform-specific code for z/OS goes here. For the POSIX-compatible
// parts, the implementation is in platform-posix.rs.

// TODO(gabylb): zos - most OS class members here will be removed once mmap
// is fully implemented on z/OS, after which those in platform-posix.rs will
// be used.

use libc::{
    c_char, c_double, c_int, c_long, c_size_t, c_void, fclose, fdopen, fileno, fopen, fread,
    fwrite, gmtime_r, lseek, localtime_r, mmap, munmap, open, perror, time, time_t, tm, O_RDONLY,
    O_RDWR, PROT_NONE, PROT_READ, PROT_WRITE, SEEK_END, MAP_FAILED, MAP_FIXED, MAP_PRIVATE,
    MAP_SHARED,
};
use std::{
    ffi::{CStr, CString},
    fs::File,
    io,
    mem::MaybeUninit,
    os::unix::io::FromRawFd,
    ptr::{null_mut, NonNull},
    slice,
    sync::Mutex,
};

mod platform_posix;

extern "C" {
    fn init_zoslib_config(config: *mut zoslib_config_t);
    fn init_zoslib(config: zoslib_config_t);
    fn __zfree(address: *mut c_void, size: c_size_t) -> c_int;
    fn __zalloc(size: c_size_t, alignment: c_size_t) -> *mut c_void;
    fn __zalloc_for_fd(size: c_size_t, name: *const c_char, fd: c_int, offset: c_long)
        -> *mut c_void;
}

#[repr(C)]
struct zoslib_config_t {}

static INIT_ZOSLIB: Mutex<bool> = Mutex::new(false);

fn initialize_zoslib() {
    let mut guard = INIT_ZOSLIB.lock().unwrap();
    if !*guard {
        let mut config = zoslib_config_t {};
        unsafe {
            init_zoslib_config(&mut config);
            init_zoslib(config);
        }
        *guard = true;
    }
}

#[allow(non_snake_case)]
pub mod base {
    pub mod platform {
        use super::super::*;
        use libc::{c_char, c_int, c_size_t, c_void, time, tm};
        use std::{f64::NAN, fs::File, io, ptr::null_mut};

        pub enum MemoryPermission {
            Read,
            ReadWrite,
            ReadExecute,
            ReadWriteExecute,
        }

        pub type PlatformSharedMemoryHandle = c_int;

        pub fn file_descriptor_from_shared_memory_handle(handle: PlatformSharedMemoryHandle) -> c_int {
            handle
        }

        pub fn get_protection_from_memory_permission(access: MemoryPermission) -> c_int {
            match access {
                MemoryPermission::Read => libc::PROT_READ,
                MemoryPermission::ReadWrite => libc::PROT_READ | libc::PROT_WRITE,
                MemoryPermission::ReadExecute => libc::PROT_READ | libc::PROT_EXEC,
                MemoryPermission::ReadWriteExecute => {
                    libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC
                }
            }
        }

        pub struct OS {}

        impl OS {
            pub fn free(address: *mut c_void, size: c_size_t) {
                assert_eq!(
                    (address as usize) % Self::allocate_page_size(),
                    0
                );
                assert_eq!(size % Self::allocate_page_size(), 0);
                let result = unsafe { __zfree(address, size) };
                assert_eq!(result, 0);
            }

            pub fn release(address: *mut c_void, size: c_size_t) {
                assert_eq!(
                    (address as usize) % Self::commit_page_size(),
                    0
                );
                assert_eq!(size % Self::commit_page_size(), 0);
                let result = unsafe { __zfree(address, size) };
                assert_eq!(result, 0);
            }

            pub fn allocate(
                hint: *mut c_void,
                size: c_size_t,
                alignment: c_size_t,
                _access: MemoryPermission,
            ) -> *mut c_void {
                unsafe { __zalloc(size, alignment) }
            }

            pub fn create_timezone_cache() -> Box<dyn TimezoneCache> {
                Box::new(ZOSTimezoneCache {})
            }

            pub fn allocate_shared(
                hint: *mut c_void,
                size: c_size_t,
                access: MemoryPermission,
                handle: PlatformSharedMemoryHandle,
                offset: u64,
            ) -> *mut c_void {
                assert_eq!(size % Self::allocate_page_size(), 0);
                let prot = get_protection_from_memory_permission(access);
                let fd = file_descriptor_from_shared_memory_handle(handle);
                unsafe { mmap(hint, size, prot as i32, MAP_SHARED, fd, offset as i64) }
            }

            pub fn free_shared(address: *mut c_void, size: c_size_t) {
                assert_eq!(size % Self::allocate_page_size(), 0);
                let result = unsafe { munmap(address, size) };
                assert_eq!(result, 0);
            }

            pub fn get_shared_library_addresses() -> Vec<SharedLibraryAddress> {
                Vec::new()
            }

            pub fn signal_code_moving_gc() {}

            pub fn adjust_scheduling_params() {}

            pub fn set_permissions(
                address: *mut c_void,
                size: c_size_t,
                _access: MemoryPermission,
            ) -> bool {
                assert_eq!(
                    (address as usize) % Self::commit_page_size(),
                    0
                );
                assert_eq!((address as usize) % Self::commit_page_size(), 0);
                assert_eq!(size % Self::commit_page_size(), 0);
                true
            }

            pub fn set_data_read_only(address: *mut c_void, size: c_size_t) {
                assert_eq!(
                    (address as usize) % Self::commit_page_size(),
                    0
                );
                assert_eq!(size % Self::commit_page_size(), 0);
            }

            pub fn recommit_pages(
                address: *mut c_void,
                size: c_size_t,
                access: MemoryPermission,
            ) -> bool {
                assert_eq!(
                    (address as usize) % Self::commit_page_size(),
                    0
                );
                assert_eq!(size % Self::commit_page_size(), 0);
                Self::set_permissions(address, size, access)
            }

            pub fn discard_system_pages(address: *mut c_void, size: c_size_t) -> bool {
                assert_eq!(
                    (address as usize) % Self::commit_page_size(),
                    0
                );
                assert_eq!(size % Self::commit_page_size(), 0);
                true
            }

            pub fn decommit_pages(address: *mut c_void, size: c_size_t) -> bool {
                assert_eq!(
                    (address as usize) % Self::commit_page_size(),
                    0
                );
                assert_eq!(size % Self::commit_page_size(), 0);
                true
            }

            pub fn has_lazy_commits() -> bool {
                false
            }

            const fn allocate_page_size() -> usize {
                4096
            }

            const fn commit_page_size() -> usize {
                4096
            }

            // Placeholder function for obtaining a random mmap address.  The actual implementation
            // would need to consider platform-specific constraints and randomness sources.
            pub fn get_random_mmap_addr() -> *mut c_void {
                null_mut()
            }

        }

        pub struct AddressSpaceReservation {}

        impl AddressSpaceReservation {
            pub fn contains(&self, _address: *mut c_void, _size: c_size_t) -> bool {
                true // Replace with actual logic
            }
            pub fn allocate_shared(
                &self,
                address: *mut c_void,
                size: c_size_t,
                access: MemoryPermission,
                handle: PlatformSharedMemoryHandle,
                offset: u64,
            ) -> bool {
                assert!(self.contains(address, size));
                let prot = get_protection_from_memory_permission(access);
                let fd = file_descriptor_from_shared_memory_handle(handle);
                unsafe {
                    mmap(
                        address,
                        size,
                        prot as i32,
                        MAP_SHARED | MAP_FIXED,
                        fd,
                        offset as i64,
                    ) != MAP_FAILED
                }
            }
            pub fn free_shared(&self, address: *mut c_void, size: c_size_t) -> bool {
                assert!(self.contains(address, size));
                unsafe {
                    mmap(address, size, PROT_NONE, MAP_FIXED | MAP_PRIVATE, -1, 0) != MAP_FAILED
                }
            }
        }

        pub struct SharedLibraryAddress {}

        pub trait TimezoneCache {
            fn local_timezone(&self, time: f64) -> String;
            fn local_time_offset(&self, time_ms: f64, is_utc: bool) -> f64;
        }

        struct ZOSTimezoneCache {}

        impl ZOSTimezoneCache {
            const MS_PER_SECOND: f64 = 1000.0;
        }

        impl TimezoneCache for ZOSTimezoneCache {
            fn local_timezone(&self, time: f64) -> String {
                if time.is_nan() {
                    return "".to_string();
                }
                let tv = time.floor() / Self::MS_PER_SECOND;
                let tv = tv as time_t;
                let mut tm = MaybeUninit::<tm>::uninit();
                let t = unsafe { localtime_r(&tv, tm.as_mut_ptr()) };
                if t.is_null() {
                    return "".to_string();
                }

                // TODO: Find safe alternative to tzname
                let tzname: [*mut c_char; 2] = [null_mut(), null_mut()];

                unsafe {
                    if !tzname[0].is_null() {
                        CStr::from_ptr(tzname[0]).to_string_lossy().into_owned()
                    } else {
                        "".to_string()
                    }
                }
            }

            fn local_time_offset(&self, time_ms: f64, is_utc: bool) -> f64 {
                let tv = unsafe { time(null_mut()) };
                let mut tmv = MaybeUninit::<tm>::uninit();
                let gmt = unsafe { gmtime_r(&tv, tmv.as_mut_ptr()) };
                let gm_secs = unsafe {
                    (*gmt).tm_sec as f64
                        + ((*gmt).tm_min as f64 * 60.0)
                        + ((*gmt).tm_hour as f64 * 3600.0)
                };
                let mut tmv = MaybeUninit::<tm>::uninit();
                let localt = unsafe { localtime_r(&tv, tmv.as_mut_ptr()) };
                let local_secs = unsafe {
                    (*localt).tm_sec as f64
                        + ((*localt).tm_min as f64 * 60.0)
                        + ((*localt).tm_hour as f64 * 3600.0)
                };
                let is_dst = unsafe { (*localt).tm_isdst };
                (local_secs - gm_secs) * Self::MS_PER_SECOND
                    - (if is_dst > 0 { 3600.0 * Self::MS_PER_SECOND } else { 0.0 })
            }
        }

        pub enum FileMode {
            ReadOnly,
            ReadWrite,
        }

        pub trait MemoryMappedFileTrait {
            fn memory(&self) -> *mut c_void;
            fn size(&self) -> c_size_t;
        }

        pub struct MemoryMappedFile {
            inner: Box<dyn MemoryMappedFileTrait>,
        }

        impl MemoryMappedFile {
            pub fn open(name: &str, mode: FileMode) -> Option<Self> {
                let name_cstr = CString::new(name).ok()?;
                let fopen_mode = match mode {
                    FileMode::ReadOnly => "r",
                    FileMode::ReadWrite => "r+",
                };
                let open_mode = match mode {
                    FileMode::ReadOnly => O_RDONLY,
                    FileMode::ReadWrite => O_RDWR,
                };
                let mut memory: *mut c_void = null_mut();

                unsafe {
                    let fd = open(name_cstr.as_ptr(), open_mode);
                    if fd > 0 {
                        let file = fdopen(fd, CString::new(fopen_mode).unwrap().as_ptr());
                        let size = lseek(fd, 0, SEEK_END);
                        if size == 0 {
                            return Some(MemoryMappedFile {
                                inner: Box::new(PosixMemoryMappedFile {
                                    file: file,
                                    memory: null_mut(),
                                    size: 0,
                                    ismmap: false,
                                }),
                            });
                        }

                        if size > 0 {
                            let prot = match mode {
                                FileMode::ReadOnly => PROT_READ,
                                FileMode::ReadWrite => PROT_READ | PROT_WRITE,
                            };
                            let mut flags = MAP_PRIVATE;
                            let mut ismmap = false;

                            if let FileMode::ReadWrite = mode {
                                flags = MAP_SHARED;
                                memory = mmap(
                                    OS::get_random_mmap_addr(),
                                    size as size_t,
                                    prot,
                                    flags,
                                    fd,
                                    0,
                                );
                                ismmap = true;
                            } else {
                                memory = __zalloc_for_fd(size as size_t, name_cstr.as_ptr(), fd, 0);
                                ismmap = false;
                            }

                            if memory != MAP_FAILED {
                                return Some(MemoryMappedFile {
                                    inner: Box::new(PosixMemoryMappedFile {
                                        file: file,
                                        memory: memory,
                                        size: size as size_t,
                                        ismmap: ismmap,
                                    }),
                                });
                            }
                        } else {
                            perror(b"lseek\0".as_ptr() as *mut i8);
                        }
                        fclose(file);
                    }

                }
                None
            }

            pub fn create(name: &str, size: usize, initial: *mut c_void) -> Option<Self> {
                let name_cstr = CString::new(name).ok()?;
                unsafe {
                    let file = fopen(name_cstr.as_ptr(), CString::new("w+").unwrap().as_ptr());
                    if !file.is_null() {
                        if size == 0 {
                            return Some(MemoryMappedFile {
                                inner: Box::new(PosixMemoryMappedFile {
                                    file: file,
                                    memory: null_mut(),
                                    size: 0,
                                    ismmap: false,
                                }),
                            });
                        }
                        let result = fwrite(initial, 1, size, file);
                        if result == size {
                            let memory = mmap(
                                OS::get_random_mmap_addr(),
                                result,
                                PROT_READ | PROT_WRITE,
                                MAP_SHARED,
                                fileno(file),
                                0,
                            );
                            if memory != MAP_FAILED {
                                return Some(MemoryMappedFile {
                                    inner: Box::new(PosixMemoryMappedFile {
                                        file: file,
                                        memory: memory,
                                        size: result,
                                        ismmap: true,
                                    }),
                                });
                            }
                        }
                        fclose(file);
                    }
                }
                None
            }

            pub fn memory(&self) -> *mut c_void {
                self.inner.memory()
            }

            pub fn size(&self) -> c_size_t {
                self.inner.size()
            }
        }

        struct PosixMemoryMappedFile {
            file: *mut libc::FILE,
            memory: *mut c_void,
            size: c_size_t,
            ismmap: bool,
        }

        impl MemoryMappedFileTrait for PosixMemoryMappedFile {
            fn memory(&self) -> *mut c_void {
                self.memory
            }
            fn size(&self) -> c_size_t {
                self.size
            }
        }

        impl Drop for PosixMemoryMappedFile {
            fn drop(&mut self) {
                unsafe {
                    if !self.memory.is_null() {
                        if self.ismmap {
                            munmap(self.memory, round_up(self.size, OS::allocate_page_size()));
                        } else {
                            __zfree(self.memory, self.size);
                        }
                    }
                    fclose(self.file);
                }
            }
        }

        fn round_up(x: usize, multiple: usize) -> usize {
            (x + multiple - 1) / multiple * multiple
        }

    }
}