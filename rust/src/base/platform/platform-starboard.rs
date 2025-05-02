// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Platform-specific code for Starboard goes here. Starboard is the platform
// abstraction layer for Cobalt, an HTML5 container used mainly by YouTube
// apps in the living room.

use std::alloc::{alloc, dealloc, Layout};
use std::ffi::CString;
use std::mem;
use std::ops::Deref;
use std::ptr;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

//use libc; // Consider using specific types instead of the whole libc

//use starboard_rs; // Assuming starboard has a Rust crate
//use starboard_rs::eztime;

mod starboard_rs {
    pub mod common {
        pub mod condition_variable {
            // Placeholder for starboard::common::condition_variable functionality
        }
        pub mod log {
            use std::ffi::CString;
            use std::os::raw::c_char;
            use std::sync::Mutex;

            static LOG_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

            pub fn sb_log_raw_format(format: *const c_char, args: *mut core::ffi::VaList) {
                let _lock = LOG_MUTEX.get_or_init(|| Mutex::new(())).lock().unwrap();
                unsafe {
                    let c_format = CString::from_raw(format as *mut c_char);
                    // Not possible to implement VaList in rust.
                    //println!("SbLogRawFormat: {}, VaList is not printed", c_format.to_str().unwrap());
                    println!("SbLogRawFormat: {}", c_format.to_str().unwrap());
                    let _ = CString::into_raw(c_format);
                }
            }
        }
        pub mod string {
            // Placeholder for starboard::common::string functionality
        }
        pub mod time {
            // Placeholder for starboard::common::time functionality
            pub fn current_monotonic_thread_time() -> i64 {
                // Stub Implementation
                0
            }
        }
    }
    pub mod client_porting {
        pub mod eztime {
            // Placeholder for starboard::client_porting::eztime functionality
        }
    }
    pub mod configuration {
        // Placeholder for starboard::configuration functionality
        pub const SB_MEMORY_PAGE_SIZE: usize = 4096;
    }
    pub mod configuration_constants {
        // Placeholder for starboard::configuration_constants functionality
    }
    pub mod time_zone {
        use std::ffi::CString;

        pub fn sb_time_zone_get_name() -> *const i8 {
            //Stub Implementation
            let s = CString::new("UTC").unwrap();
            s.into_raw() as *const i8
        }
        pub fn sb_time_zone_get_current() -> i32 {
            //Stub Implementation
            0
        }
    }
    pub mod thread {
        use std::thread;
        use std::sync::{Mutex, Arc};
        use std::os::raw::c_void;
        use std::ffi::CString;
        use std::collections::HashMap;
        use std::sync::RwLock;

        pub type SbThread = usize;
        pub const SB_THREAD_INVALID: SbThread = 0;
        pub const SB_THREAD_NO_PRIORITY: i32 = 0;
        pub const SB_THREAD_NO_AFFINITY: i32 = 0;

        pub fn sb_thread_create(stack_size: usize, priority: i32, affinity: i32, detached: bool, name: *const i8, entry_point: extern "C" fn(arg: *mut c_void) -> *mut c_void, arg: *mut c_void) -> SbThread {
            unsafe {
                let c_name = CString::from_raw(name as *mut i8);
                let thread_name = c_name.to_str().unwrap().to_string();
                 let _ = CString::into_raw(c_name);

                let builder = thread::Builder::new().name(thread_name.clone());
                let _ = stack_size;
                let _ = priority;
                let _ = affinity;
                let _ = detached;
                let join_handle = builder.spawn(move || {
                    entry_point(arg);
                }).unwrap();

                let thread_id = join_handle.thread().id();
                let thread_id_usize =  unsafe { mem::transmute::<thread::ThreadId, usize>(thread_id)};

                THREAD_MAP.write().unwrap().insert(thread_id_usize, join_handle);

                thread_id_usize as SbThread
            }
        }

        pub fn sb_thread_is_valid(thread: SbThread) -> bool {
           if THREAD_MAP.read().unwrap().contains_key(&thread){
               true
           }
           else {
            thread != SB_THREAD_INVALID
           }
        }

        pub fn sb_thread_join(thread: SbThread, _out_result: *mut *mut c_void) {
            if let Some(join_handle) = THREAD_MAP.write().unwrap().remove(&thread) {
                join_handle.join().unwrap();
            }
        }

        pub type SbThreadLocalKey = usize;

        static THREAD_LOCAL_MAP: OnceLock<RwLock<HashMap<(usize, SbThreadLocalKey), *mut c_void>>> = OnceLock::new();
        static THREAD_KEY_COUNTER: OnceLock<Mutex<usize>> = OnceLock::new();
        static THREAD_MAP: OnceLock<RwLock<HashMap<usize, thread::JoinHandle<()>>>> = OnceLock::new();


        pub fn sb_thread_create_local_key(_dtor: Option<extern "C" fn(value: *mut c_void)>) -> SbThreadLocalKey {
            let mut counter = THREAD_KEY_COUNTER.get_or_init(|| Mutex::new(0)).lock().unwrap();
            *counter += 1;
            *counter
        }

        pub fn sb_thread_destroy_local_key(key: SbThreadLocalKey) {
            // Logic to destroy a thread local key.  Needs synchronization
             let mut thread_local_map = THREAD_LOCAL_MAP.get_or_init(|| RwLock::new(HashMap::new())).write().unwrap();
             thread_local_map.retain(|&(thread_id, k), _| k != key );
        }

        pub fn sb_thread_get_local_value(key: SbThreadLocalKey) -> *mut c_void {
            let thread_id =  unsafe { mem::transmute::<thread::ThreadId, usize>(thread::current().id())};

            let thread_local_map = THREAD_LOCAL_MAP.get_or_init(|| RwLock::new(HashMap::new())).read().unwrap();
            thread_local_map.get(&(thread_id, key)).map_or(ptr::null_mut(), |&value| value)
        }

        pub fn sb_thread_set_local_value(key: SbThreadLocalKey, value: *mut c_void) -> bool {
            let thread_id =  unsafe { mem::transmute::<thread::ThreadId, usize>(thread::current().id())};

            let mut thread_local_map = THREAD_LOCAL_MAP.get_or_init(|| RwLock::new(HashMap::new())).write().unwrap();
            thread_local_map.insert((thread_id, key), value);
            true
        }

        pub fn sb_thread_get_id() -> i32 {
            let thread_id = thread::current().id();
            let thread_id_usize =  unsafe { mem::transmute::<thread::ThreadId, usize>(thread_id)};
            thread_id_usize as i32
        }

         pub fn sb_thread_set_name(name: *const i8) {
            unsafe {
                let c_name = CString::from_raw(name as *mut i8);
                let thread_name = c_name.to_str().unwrap().to_string();
                thread::current().name().map(|current_name|{
                assert_eq!(thread_name, current_name);
                });
                let _ = CString::into_raw(c_name);
            }
        }
    }
    pub mod system {
        pub fn sb_system_break_into_debugger() {
            // Placeholder for starboard::system::sb_system_break_into_debugger
            println!("SbSystemBreakIntoDebugger called");
            #[cfg(debug_assertions)]
            std::process::abort();
        }
        pub fn sb_system_get_last_error() -> i32 {
            // Placeholder for starboard::system::sb_system_get_last_error
            0 // Or some other default
        }
        pub fn sb_system_get_stack(addresses: *mut *mut std::os::raw::c_void, count: usize) -> usize {
           // Stub Implementation
           0
        }
    }
}

pub mod base {
    use std::time::{Duration, SystemTime};
    pub mod lazy_instance {
        use std::sync::{Mutex, OnceLock};

        pub struct LazyInstance<T> {
            instance: OnceLock<T>,
        }

        impl<T> LazyInstance<T> {
            pub const fn new() -> Self {
                LazyInstance {
                    instance: OnceLock::new(),
                }
            }

            pub fn get<F>(&self, init: F) -> &T
            where
                F: FnOnce() -> T,
            {
                self.instance.get_or_init(init)
            }
        }

        unsafe impl<T> Sync for LazyInstance<T> where T: Sync {}
    }
    pub mod macros {
        macro_rules! sb_notimplemented {
            () => {
                println!("SB_NOTIMPLEMENTED");
                unimplemented!()
            };
        }
        pub(crate) use sb_notimplemented;
    }
    pub mod platform {
        use std::{
            alloc::{alloc, dealloc, Layout},
            ffi::CStr,
            fmt::Write,
            fs::File,
            io,
            mem,
            ops::{Deref, DerefMut},
            os::raw::{c_char, c_int, c_void},
            path::Path,
            ptr,
            slice,
            sync::{Arc, Mutex, OnceLock},
            time::{Duration, SystemTime, UNIX_EPOCH},
        };

        use crate::starboard_rs::common::log::sb_log_raw_format;
        use crate::starboard_rs::system::sb_system_break_into_debugger;
        use crate::starboard_rs::thread::{sb_thread_create, sb_thread_join, sb_thread_is_valid, sb_thread_get_id};
        use crate::starboard_rs::thread::{sb_thread_create_local_key, sb_thread_destroy_local_key, sb_thread_get_local_value, sb_thread_set_local_value, SbThreadLocalKey};
        use crate::starboard_rs::{
            client_porting::eztime::*,
            common::time::current_monotonic_thread_time,
            configuration::SB_MEMORY_PAGE_SIZE,
            time_zone::{sb_time_zone_get_current, sb_time_zone_get_name},
            system::sb_system_get_last_error,
        };

        use super::lazy_instance::LazyInstance;
        use super::macros::sb_notimplemented;
        use super::time::TimeDelta;
        use super::timezone_cache::TimezoneCache;
        use super::utils::random_number_generator::RandomNumberGenerator;

        #[cfg(target_arch = "arm")]
        pub fn arm_using_hard_float() -> bool {
            // This logic is complex and relies on preprocessor defines.
            //  It's highly recommended to test this thoroughly on the target platform.
            // Placeholder implementation:
            false
        }

        static PLATFORM_RANDOM_NUMBER_GENERATOR: LazyInstance<RandomNumberGenerator> =
            LazyInstance::new();
        static RNG_MUTEX: LazyInstance<Mutex<()>> = LazyInstance::new();

        const K_STACK_SIZE: usize = 1;

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum AbortMode {
            Exit,
            // Other modes can be added if needed
        }

        static mut G_ABORT_MODE: AbortMode = AbortMode::Exit;

        pub struct OS {}

        impl OS {
            pub fn initialize(abort_mode: AbortMode, _gc_fake_mmap: Option<&str>) {
                unsafe {
                    G_ABORT_MODE = abort_mode;
                }
            }

            pub fn get_user_time(&self, secs: &mut u32, usecs: &mut u32) -> Result<(), i32> {
                let us_time = current_monotonic_thread_time();
                if us_time == 0 {
                    return Err(-1);
                }
                *secs = (us_time / TimeConstants::K_MICRO_SECONDS_PER_SECOND) as u32;
                *usecs = (us_time % TimeConstants::K_MICRO_SECONDS_PER_SECOND) as u32;
                Ok(())
            }

            pub fn time_current_millis() -> f64 {
                Time::now().to_js_time()
            }

            pub fn activation_frame_alignment() -> usize {
                #[cfg(target_arch = "arm")]
                {
                    8
                }
                #[cfg(target_arch = "mips")]
                {
                    8
                }
                #[cfg(target_arch = "s390x")]
                {
                    8
                }
                #[cfg(not(any(target_arch = "arm", target_arch = "mips", target_arch = "s390x")))]
                {
                    16
                }
            }

            pub fn allocate_page_size() -> usize {
                SB_MEMORY_PAGE_SIZE
            }

            pub fn commit_page_size() -> usize {
                SB_MEMORY_PAGE_SIZE
            }

            pub fn set_random_mmap_seed(_seed: i64) {
                sb_notimplemented!()
            }

            pub fn get_random_mmap_addr() -> *mut c_void {
                ptr::null_mut()
            }

            pub fn allocate(
                address: *mut c_void,
                size: usize,
                alignment: usize,
                access: MemoryPermission,
            ) -> *mut c_void {
                let page_size = Self::allocate_page_size();
                assert_eq!(0, size % page_size);
                assert_eq!(0, alignment % page_size);
                let address = Self::aligned_address(address, alignment);

                let request_size = size + (alignment - page_size);
                let request_size = round_up(request_size, Self::allocate_page_size());

                let result = allocate_memory(address, request_size, access);
                if result.is_null() {
                    return ptr::null_mut();
                }

                let base = result as *mut u8;
                let aligned_base = round_up(base as usize, alignment) as *mut u8;

                if aligned_base != base {
                    assert!(base < aligned_base);
                    let prefix_size = (aligned_base as usize) - (base as usize);
                    Self::free(base as *mut c_void, prefix_size);
                    // SAFETY: aligned_base is within the original allocation
                    unsafe {
                        deallocate(base, Layout::from_size_align(prefix_size, Self::allocate_page_size()).unwrap());
                    }
                    //request_size -= prefix_size;
                }

                let aligned_base_offset = aligned_base as usize - base as usize;

                if size != request_size {
                     let suffix_size = request_size - size;

                     let suffix_address = (aligned_base as usize + size) as *mut c_void;
                    // SAFETY: suffix_address is within the original allocation and suffix_size is correct
                    unsafe {
                        deallocate(suffix_address as *mut u8, Layout::from_size_align(suffix_size, Self::allocate_page_size()).unwrap());
                    }

                    Self::free(suffix_address, suffix_size);
                }
                aligned_base as *mut c_void
            }

            fn aligned_address(address: *mut c_void, alignment: usize) -> *mut c_void {
                 address
            }

            pub fn free(address: *mut c_void, size: usize) {
                 //SAFETY: The pointer and size must be valid
                  unsafe {
                      deallocate(address as *mut u8, Layout::from_size_align(size, Self::allocate_page_size()).unwrap());
                  }

                let result = unsafe { libc::munmap(address, size as libc::size_t) };
                assert_eq!(result, 0);
            }

            pub fn release(address: *mut c_void, size: usize) {
                 //SAFETY: The pointer and size must be valid
                  unsafe {
                      deallocate(address as *mut u8, Layout::from_size_align(size, Self::allocate_page_size()).unwrap());
                  }

                let result = unsafe { libc::munmap(address, size as libc::size_t) };
                assert_eq!(result, 0);
            }

            pub fn set_permissions(
                address: *mut c_void,
                size: usize,
                access: MemoryPermission,
            ) -> bool {
                let new_protection = match access {
                    MemoryPermission::NoAccess => libc::PROT_NONE,
                    MemoryPermission::Read => libc::PROT_READ,
                    MemoryPermission::ReadWrite => libc::PROT_READ | libc::PROT_WRITE,
                    MemoryPermission::ReadExecute => {
                        #[cfg(feature = "map_executable_memory")]
                        {
                            libc::PROT_READ | libc::PROT_EXEC
                        }
                        #[cfg(not(feature = "map_executable_memory"))]
                        {
                            unsafe { core::hint::unreachable_unchecked() }
                        }
                    }
                    _ => return false, // Other types are not supported by Starboard.
                };

                let result = unsafe { libc::mprotect(address, size as libc::size_t, new_protection) };
                result == 0
            }

            pub fn recommit_pages(
                address: *mut c_void,
                size: usize,
                access: MemoryPermission,
            ) -> bool {
                Self::set_permissions(address, size, access)
            }

            pub fn has_lazy_commits() -> bool {
                sb_notimplemented!()
            }

            pub fn sleep(interval: TimeDelta) {
                crate::starboard_rs::thread::sb_thread_sleep(interval.in_microseconds());
            }

            pub fn abort() {
                sb_system_break_into_debugger();
            }

            pub fn debug_break() {
                sb_system_break_into_debugger();
            }

            pub fn memory_mapped_file_open(_name: &str, _mode: FileMode) -> Option<MemoryMappedFile> {
                sb_notimplemented!()
            }

            pub fn memory_mapped_file_create(
                _name: &str,
                _size: usize,
                _initial: *mut c_void,
            ) -> Option<MemoryMappedFile> {
                sb_notimplemented!()
            }

            pub fn get_current_process_id() -> i32 {
                sb_notimplemented!()
            }

            pub fn get_current_thread_id_internal() -> i32 {
                crate::starboard_rs::thread::sb_thread_get_id()
            }

            pub fn get_last_error() -> i32 {
                sb_system_get_last_error()
            }

            pub fn fopen(_path: &str, _mode: &str) -> Option<File> {
                sb_notimplemented!()
            }

            pub fn remove(_path: &str) -> bool {
                sb_notimplemented!()
            }

            pub fn directory_separator() -> char {
                '/'
            }

            pub fn is_directory_separator(ch: char) -> bool {
                ch == Self::directory_separator()
            }

            pub fn open_temporary_file() -> Option<File> {
                sb_notimplemented!()
            }

            pub const LOG_FILE_OPEN_MODE: &'static str = "\0";

            pub fn print(format: &str, args: core::fmt::Arguments) {
                let c_string = CString::new(format).expect("CString::new failed");
                unsafe {
                    sb_log_raw_format(c_string.as_ptr(), args as *mut _);
                }
            }

            pub fn vprint(format: &str, args: core::fmt::Arguments) {
                let c_string = CString::new(format).expect("CString::new failed");
                unsafe {
                    sb_log_raw_format(c_string.as_ptr(), args as *mut _);
                }
            }

            pub fn fprint(_out: &mut File, format: &str, args: core::fmt::Arguments) {
                let c_string = CString::new(format).expect("CString::new failed");
                unsafe {
                    sb_log_raw_format(c_string.as_ptr(), args as *mut _);
                }
            }

            pub fn vfprint(_out: &mut File, format: &str, args: core::fmt::Arguments) {
                let c_string = CString::new(format).expect("CString::new failed");
                unsafe {
                    sb_log_raw_format(c_string.as_ptr(), args as *mut _);
                }
            }

            pub fn printerror(format: &str, args: core::fmt::Arguments) {
                let c_string = CString::new(format).expect("CString::new failed");
                unsafe {
                    sb_log_raw_format(c_string.as_ptr(), args as *mut _);
                }
            }

            pub fn vprinterror(format: &str, args: core::fmt::Arguments) {
                let c_string = CString::new(format).expect("CString::new failed");
                unsafe {
                    sb_log_raw_format(c_string.as_ptr(), args as *mut _);
                }
            }

            pub fn snprintf(str: &mut [u8], format: &str, args: core::fmt::Arguments) -> i32 {
                let format_c_str = CString::new(format).unwrap();
                let len = str.len();
                let result = unsafe {
                    libc::vsnprintf(
                        str.as_mut_ptr() as *mut libc::c_char,
                        len as libc::size_t,
                        format_c_str.as_ptr() as *const libc::c_char,
                        args as *mut core::ffi::VaList,
                    )
                };

                if result < 0 || result as usize >= len {
                    if len > 0 {
                        str[len - 1] = 0;
                    }
                    -1
                } else {
                    result as i32
                }
            }

            pub fn vscnprintf(str: &mut [u8], format: &str, args: core::fmt::Arguments) -> i32 {
                let format_c_str = CString::new(format).unwrap();
                let len = str.len();
                let result = unsafe {
                    libc::vsnprintf(
                        str.as_mut_ptr() as *mut libc::c_char,
                        len as libc::size_t,
                        format_c_str.as_ptr() as *const libc::c_char,
                        args as *mut core::ffi::VaList,
                    )
                };

                if result < 0 || result as usize >= len {
                    if len > 0 {
                        str[len - 1] = 0;
                    }
                    -1
                } else {
                    result as i32
                }
            }

            pub fn strncpy(dest: &mut [u8], src: &str, n: usize) {
                let src_bytes = src.as_bytes();
                let len = std::cmp::min(src_bytes.len(), n);
                dest[..len].copy_from_slice(&src_bytes[..len]);
                if len < n {
                    dest[len] = 0;
                }
            }

            // No direct equivalent for POSIX threads in core Rust.  Using std::thread.
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum MemoryPermission {
            NoAccess,
            Read,
            ReadWrite,
            ReadExecute,
        }

        fn allocate_memory(
            address: *mut c_void,
            size: usize,
            access: MemoryPermission,
        ) -> *mut c_void {
            let prot_flags = match access {
                MemoryPermission::NoAccess => libc::PROT_NONE,
                MemoryPermission::ReadWrite => libc::PROT_READ | libc::PROT_WRITE,
                _ => {
                    println!(
                        "The requested memory allocation access is not implemented for Starboard"
                    );
                    return ptr::null_mut();
                }
            };

            let result = unsafe {
                libc::mmap(
                    address,
                    size as libc::size_t,
                    prot_flags,
                    libc::MAP_PRIVATE | libc::MAP_ANON,
                    -1,
                    0,
                )
            };
            if result == libc::MAP_FAILED {
                ptr::null_mut()
            } else {
                result
            }
        }

        fn round_up(x: usize, multiple: usize) -> usize {
            (x + multiple - 1) & !(multiple - 1)
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum FileMode {
            ReadOnly,
            ReadWrite,
            // Add other modes as necessary
        }

        pub struct MemoryMappedFile {}

        impl MemoryMappedFile {
            pub fn open(_name: &str, _mode: FileMode) -> Option<Self> {
                sb_notimplemented!()
            }
            pub fn create(_name: &str, _size: usize, _initial: *mut c_void) -> Option<Self> {
                sb_notimplemented!()
            }
            pub fn memory(&self) -> *mut c_void {
                sb_notimplemented!()
            }
            pub fn size(&self) -> usize {
                sb_notimplemented!()
            }
        }

        pub struct Thread {
            data_: Box<PlatformData>,
            stack_size_: usize,
            name_: [i8; 64],
            start_semaphore_: *mut c_void, // Assuming c_void* is appropriate
        }

        struct PlatformData {
            thread_: usize,
            thread_creation_mutex_: Mutex<()>,
        }

        impl Thread {
            pub fn new(options: &Options) -> Self {
                let name_bytes = options.name().as_bytes();
                let mut name_: [i8; 64] = [0; 64];

                for (i, &byte) in name_bytes.iter().enumerate().take(63) {
                    name_[i] = byte as i8;
                }

                Self {
                    data_: Box::new(PlatformData {
                        thread_: 0,
                        thread_creation_mutex_: Mutex::new(()),
                    }),
                    stack_size_: options.stack_size(),
                    name_: name_,
                    start_semaphore_: ptr::null_mut(),
                }
            }

            fn set_name(&mut self, name: &str) {
                let name_bytes = name.as_bytes();
                for (i, &byte) in name_bytes.iter().enumerate().take(63) {
                    self.name_[i] = byte as i8;
                }
                self.name_[std::cmp::min(name_bytes.len(), 63)] = 0;
            }

            pub fn start(&mut self) -> bool {
                unsafe {
                     let c_name = CString::new(self.name_.iter().map(|&c| c as u8).collect::<Vec<u8>>()).unwrap();
                     let name_ptr = c_name.as_ptr();

                    let entry_point: extern "C" fn(*mut c_void) -> *mut c_void =
                        Self::thread_entry;

                    self.data_.thread_ = sb_thread_create(
                        self.stack_size_,
                        crate::starboard_rs::thread::SB_THREAD_NO_PRIORITY,
                        crate::starboard_rs::thread::SB_THREAD_NO_AFFINITY,
                        true,
                        name_ptr,
                        entry_point,
                        self as *mut Self as *mut c_void,
                    );
                    let _ = CString::into_raw(c_name);
                }
                crate::starboard_rs::thread::sb_thread_is_valid(self.data_.thread_)
            }

            extern "C" fn thread_entry(arg: *mut c_void) -> *mut c_void {
                let thread = unsafe { &mut *(arg as *mut Thread) };
                let _lock = thread.data_.thread_creation_mutex_.lock().unwrap();

                unsafe {
                    let c_name = CString::new(thread.name_.iter().map(|&c| c as u8).collect::<Vec<u8>>()).unwrap();
                    crate::starboard_rs::thread::sb_thread_set_name(c_name.as_ptr());
                     let _ = CString::into_raw(c_name);
                }
                thread.notify_started_and_run();
                ptr::null_mut()
            }

            pub fn join(&mut self) {
                crate::starboard_rs::thread::sb_thread_join(self.data_.thread_, ptr::null_mut());
            }

            fn notify_started_and_run(&mut self) {
                // Placeholder for notify_started_and_run functionality.
                // This would likely involve signaling a semaphore or other synchronization primitive.
            }

             pub type LocalStorageKey = usize;

            pub fn create_thread_local_key() -> LocalStorageKey {
                crate::starboard_rs::thread::sb_thread_create_local_key(None)
            }

            pub fn delete_thread_local_key(key: LocalStorageKey) {
                crate::starboard_rs::thread::sb_thread_destroy_local_key(key);
            }

            pub fn get_thread_local(key: LocalStorageKey) -> *mut c_void {
                crate::starboard_rs::thread::sb_thread_get_local_value(key)
            }

            pub fn set_thread_local(key: LocalStorageKey, value: *mut c_void) {
                crate::starboard_rs::thread::sb_thread_set_local_value(key, value);
            }
        }

        impl Drop for Thread {
            fn drop(&mut self) {
                // Handle resource cleanup here, if needed.
            }
        }

        pub struct Options {
            name: String,
            stack_size: usize,
        }

        impl Options {
            pub fn new() -> Self {
                Options {
                    name: String::new(),
                    stack_size: 0,
                }
            }

            pub fn name(&self) -> &str {
                &self.name
            }

            pub fn stack_size(&self) -> usize {
                self.stack_size
            }

            pub fn set_name(&mut self, name: String) -> &mut Self {
                self.name = name;
                self
            }

            pub fn set_stack_size(&mut self, stack_size: usize) -> &mut Self {
                self.stack_size = stack_size;
                self
            }
        }

        pub struct StarboardMemoryMappedFile {}

        impl Drop for StarboardMemoryMappedFile {
            fn drop(&mut self) {
                sb_notimplemented!()
            }
        }

        impl Drop for MemoryMappedFile {
           fn drop(&mut self){
            //Drop Implementation
           }
        }

        pub enum TimeZoneDetection {
            //TODO - Implement
            System,
            None,
        }

        pub fn create_timezone_cache() -> Box<dyn TimezoneCache> {
            Box::new(StarboardDefaultTimezoneCache {})
        }

        pub fn get_shared_library_addresses() -> Vec<SharedLibraryAddress> {
            sb_notimplemented!()
        }

        pub fn signal_code_moving_gc() {
            sb_notimplemented!()
        }

        pub fn adjust_scheduling_params() {}

        pub fn get_first_free_memory_range_within(
            _boundary_start: *mut c_void,
            _boundary_end: *mut c_void,
            _minimum_size: usize,
            _alignment: usize,
        ) -> Option<MemoryRange> {
            None
        }

        pub fn discard_system_pages(address: *mut c_void, size: usize) -> bool {
            // Starboard API does not support this function yet.
            true
        }

        pub fn stack_get_stack_start() -> *mut c_void {
           sb_notimplemented!();
        }

        pub fn stack_get_current_stack_position() -> *mut c_void {
            let mut addresses: [*mut c_void; K_STACK_SIZE] = [ptr::null_mut(); K_STACK_SIZE];
            let count = crate::starboard_rs::system::sb_system_get_stack(addresses.as_mut_ptr(), K_STACK_SIZE);
