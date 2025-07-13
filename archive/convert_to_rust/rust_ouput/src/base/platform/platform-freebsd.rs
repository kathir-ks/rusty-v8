// Converted from V8 C++ source files:
// Header: N/A
// Implementation: platform-freebsd.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

use std::ffi::CString;
use std::io;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::slice;
use std::str;
use std::sync::Mutex;
//use libc::{pthread_attr_t, pthread_self};

use std::ffi::CStr;
use std::sync::Arc;

//use crate::base::platform::platform_posix::{PosixDefaultTimezoneCache, TimezoneCache};
//use crate::base::platform::platform::{OS, SharedLibraryAddress, TimezoneCache};
//use crate::base::platform::time::TimezoneCache;
//use crate::base::platform::time::TimezoneCache;

pub struct PosixDefaultTimezoneCache {}
pub trait TimezoneCache {
    fn get_cached_timezone(&self, time: i64) -> String;
}
impl TimezoneCache for PosixDefaultTimezoneCache {
    fn get_cached_timezone(&self, time: i64) -> String {
        "UTC".to_string()
    }
}
pub struct OS {}

impl OS {
    pub fn CreateTimezoneCache() -> Box<dyn TimezoneCache> {
        Box::new(PosixDefaultTimezoneCache {})
    }

    pub fn GetSharedLibraryAddresses() -> Vec<SharedLibraryAddress> {
        let mut result = Vec::new();
        let mib: [i32; 4] = [libc::CTL_KERN, libc::KERN_PROC, libc::KERN_PROC_VMMAP, unsafe { libc::getpid() }];
        let miblen = mib.len();
        let mut buffer_size: usize = 0;

        unsafe {
            if libc::sysctl(
                mib.as_ptr() as *mut i32,
                miblen as u32,
                ptr::null_mut(),
                &mut buffer_size,
                ptr::null_mut(),
                0,
            ) == 0
            {
                // Overallocate the buffer by 1/3 to account for concurrent
                // kinfo_vmentry change. 1/3 is an arbitrary constant that
                // works in practice.
                buffer_size = buffer_size * 4 / 3;
                let mut buffer: Vec<u8> = Vec::with_capacity(buffer_size);
                buffer.set_len(buffer_size);

                let ret = libc::sysctl(
                    mib.as_ptr() as *mut i32,
                    miblen as u32,
                    buffer.as_mut_ptr() as *mut c_void,
                    &mut buffer_size,
                    ptr::null_mut(),
                    0,
                );

                if ret == 0 || (ret == -1 && libc::errno == libc::ENOMEM) {
                    let mut start = buffer.as_mut_ptr();
                    let end = start.add(buffer_size);

                    while start < end {
                        let map = start as *mut libc::kinfo_vmentry;
                        let ssize = (*map).kve_structsize as usize;
                        let path = (*map).kve_path.as_ptr();

                        if ssize == 0 {
                            break;
                        }

                        if ((*map).kve_protection & libc::KVME_PROT_READ != 0)
                            && ((*map).kve_protection & libc::KVME_PROT_EXEC != 0)
                            && (*path) != 0
                        {
                            let path_cstr = CStr::from_ptr(path);
                            if let Ok(path_str) = path_cstr.to_str() {
                                if let Some(sep) = path_str.rfind('/') {
                                    let lib_name = path_str[sep + 1..].to_string();
                                    result.push(SharedLibraryAddress {
                                        name: lib_name,
                                        start: (*map).kve_start as usize,
                                        end: (*map).kve_end as usize,
                                    });
                                } else {
                                    let lib_name = path_str.to_string();
                                    result.push(SharedLibraryAddress {
                                        name: lib_name,
                                        start: (*map).kve_start as usize,
                                        end: (*map).kve_end as usize,
                                    });
                                }
                            }
                        }
                        start = start.add(ssize);
                    }
                }
            }
        }
        result
    }

    pub fn SignalCodeMovingGC() {}

    pub fn AdjustSchedulingParams() {}

    pub fn GetFirstFreeMemoryRangeWithin(
        boundary_start: usize,
        boundary_end: usize,
        minimum_size: usize,
        alignment: usize,
    ) -> Option<MemoryRange> {
        None
    }
}

#[derive(Debug)]
pub struct SharedLibraryAddress {
    pub name: String,
    pub start: usize,
    pub end: usize,
}

pub struct MemoryRange {
    pub start: usize,
    pub size: usize,
}
pub struct Stack {}
impl Stack {
    pub type StackSlot = *mut u8;
    pub fn ObtainCurrentThreadStackStart() -> Stack::StackSlot {
        let mut attr: libc::pthread_attr_t = unsafe { mem::zeroed() };
        let error: i32;

        unsafe {
            error = libc::pthread_attr_init(&mut attr);
            if error != 0 {
                return ptr::null_mut();
            }

            let error = libc::pthread_getattr_np(libc::pthread_self(), &mut attr);
            if error != 0 {
                libc::pthread_attr_destroy(&mut attr);
                return ptr::null_mut();
            }

            let mut base: *mut c_void = ptr::null_mut();
            let mut size: usize = 0;
            let error = libc::pthread_attr_getstack(&attr, &mut base, &mut size);
            if error != 0 {
                libc::pthread_attr_destroy(&mut attr);
                return ptr::null_mut();
            }

            libc::pthread_attr_destroy(&mut attr);
            return (base as *mut u8).add(size);
        }
    }
}
