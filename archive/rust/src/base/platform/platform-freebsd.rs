use libc::{c_char, c_int, c_uint, c_void, getpid, size_t, strtol};
use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;
use std::slice;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{thread, time};

mod base {
    pub mod platform {
        pub mod platform_posix_time;
        pub mod platform_posix;
        pub mod platform;
    }
}

use base::platform::platform::{SharedLibraryAddress, TimezoneCache, OS};
use base::platform::platform_posix_time::PosixDefaultTimezoneCache;

extern "C" {
    pub fn sysctl(
        name: *mut c_int,
        namelen: u32,
        oldp: *mut c_void,
        oldlenp: *mut size_t,
        newp: *mut c_void,
        newlen: size_t,
    ) -> c_int;
    pub fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    pub fn pthread_self() -> usize;
    pub fn pthread_attr_init(attr: *mut pthread_attr_t) -> c_int;
    pub fn pthread_attr_destroy(attr: *mut pthread_attr_t) -> c_int;
    pub fn pthread_attr_get_np(thread: usize, attr: *mut pthread_attr_t) -> c_int;
    pub fn pthread_attr_getstack(
        attr: *const pthread_attr_t,
        stackaddr: *mut *mut c_void,
        stacksize: *mut size_t,
    ) -> c_int;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pthread_attr_t {
    __sig: i32,
    __opaque: [u8; 56],
}

const CTL_KERN: c_int = 1;
const KERN_PROC: c_int = 14;
const KERN_PROC_VMMAP: c_int = 40;
const KVME_PROT_READ: c_int = 0x0001;
const KVME_PROT_EXEC: c_int = 0x0008;
const ENOMEM: c_int = 12;

#[repr(C)]
struct kinfo_vmentry {
    kve_structsize: u16,
    kve_type: u8,
    kve_flags: u8,
    kve_resident: i64,
    kve_private_resident: i64,
    kve_shadow_resident: i64,
    kve_ref_count: i32,
    kve_depth: i32,
    kve_offset: u64,
    kve_vn_fileid: u64,
    kve_path: [c_char; 1024],
    kve_start: u64,
    kve_end: u64,
    kve_wired_count: i64,
    kve_prot: i32,
    kve_max_prot: i32,
    kve_advice: i32,
    kve_shm_id: i32,
    kve_shm_nattch: i32,
    kve_shm_segsz: i64,
    kve_shm_pages: i64,
    kve_shm_lock: i32,
    kve_shm_flags: i32,
}

impl OS {
    pub fn create_timezone_cache() -> Box<dyn TimezoneCache> {
        Box::new(PosixDefaultTimezoneCache::new())
    }

    fn string_to_long(buffer: *mut c_char) -> u32 {
        unsafe { strtol(buffer, ptr::null_mut(), 16) as u32 }
    }

    pub fn get_shared_library_addresses() -> Vec<SharedLibraryAddress> {
        let mut result = Vec::new();
        let mut mib: [c_int; 4] = [CTL_KERN, KERN_PROC, KERN_PROC_VMMAP, unsafe { getpid() }];
        let miblen = (mem::size_of_val(&mib) / mem::size_of::<c_int>()) as u32;
        let mut buffer_size: size_t = 0;

        unsafe {
            if sysctl(
                mib.as_mut_ptr(),
                miblen,
                ptr::null_mut(),
                &mut buffer_size,
                ptr::null_mut(),
                0,
            ) == 0
            {
                buffer_size = buffer_size * 4 / 3;
                let mut buffer: Vec<u8> = Vec::with_capacity(buffer_size as usize);
                buffer.resize(buffer_size as usize, 0);

                let ret = sysctl(
                    mib.as_mut_ptr(),
                    miblen,
                    buffer.as_mut_ptr() as *mut c_void,
                    &mut buffer_size,
                    ptr::null_mut(),
                    0,
                );

                if ret == 0 || (ret == -1 && libc::errno == ENOMEM) {
                    let mut start = buffer.as_mut_ptr();
                    let end = start.add(buffer_size as usize);

                    while start < end {
                        let map = start as *mut kinfo_vmentry;
                        let ssize = (*map).kve_structsize as usize;
                        let path = (*map).kve_path.as_ptr();

                        if ssize == 0 {
                            break;
                        }

                        if ((*map).kve_prot & KVME_PROT_READ != 0)
                            && ((*map).kve_prot & KVME_PROT_EXEC != 0)
                            && *path != 0
                        {
                            let path_cstr = CStr::from_ptr(path);
                            if let Ok(path_str) = path_cstr.to_str() {
                                let sep = path_str.rfind('/');
                                let lib_name = match sep {
                                    Some(index) => path_str[index + 1..].to_string(),
                                    None => path_str.to_string(),
                                };

                                result.push(SharedLibraryAddress {
                                    name: lib_name,
                                    start: (*map).kve_start as usize,
                                    end: (*map).kve_end as usize,
                                });
                            }
                        }
                        start = start.add(ssize);
                    }
                }
            }
        }

        result
    }

    pub fn signal_code_moving_gc() {}

    pub fn adjust_scheduling_params() {}

    pub fn get_first_free_memory_range_within(
        boundary_start: usize,
        boundary_end: usize,
        minimum_size: usize,
        alignment: usize,
    ) -> Option<base::platform::platform::MemoryRange> {
        None
    }
}

pub mod stack {
    use libc::{pthread_attr_destroy, pthread_attr_get_np, pthread_attr_getstack, pthread_attr_init, pthread_self};
    use crate::pthread_attr_t;
    use std::ptr;
    use std::mem;

    pub type StackSlot = *mut u8;

    pub struct Stack {}

    impl Stack {
        pub fn obtain_current_thread_stack_start() -> StackSlot {
            unsafe {
                let mut attr: pthread_attr_t = mem::zeroed();
                let error = pthread_attr_init(&mut attr);
                if error == 0 {
                    let error = pthread_attr_get_np(pthread_self(), &mut attr);
                    if error == 0 {
                        let mut base: *mut libc::c_void = ptr::null_mut();
                        let mut size: libc::size_t = 0;
                        let error = pthread_attr_getstack(&attr, &mut base, &mut size);
                        if error == 0 {
                            pthread_attr_destroy(&mut attr);
                            return (base as *mut u8).add(size);
                        }
                    }
                }
                pthread_attr_destroy(&mut attr);
                ptr::null_mut()
            }
        }
    }
}