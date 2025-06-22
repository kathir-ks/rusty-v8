// Copyright 2013 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Platform-specific code for QNX goes here. For the POSIX-compatible
// parts the implementation is in platform-posix.rs.

use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_int, c_void};
use std::path::PathBuf;
use std::ptr;

use libc::{devctl, free, getpid, open, snprintf, close, EOK, O_RDONLY, PATH_MAX};
use libc::{malloc, MAP_ELF};

#[allow(dead_code)]
#[cfg(target_arch = "arm")]
mod arm;

mod platform_posix_time;
mod platform_posix;
mod platform;

//use backtrace::Backtrace; //Feature not implemented in Rust - backtrace
//use std::thread; // Implemented by default in Rust

//use std::sync::{Arc, Mutex}; //Implemented by default in Rust

// QNX requires memory pages to be marked as executable.
// Otherwise, the OS raises an exception when executing code in that page.
//use std::io; //Implemented by default in Rust
//use std::fs::File; //Implemented by default in Rust
//use std::os::unix::io::AsRawFd; //Implemented by default in Rust
//use std::io::Read; //Implemented by default in Rust
//use std::io::BufReader; //Implemented by default in Rust

//use std::io::Error; //Implemented by default in Rust
//use std::num::ParseIntError; //Implemented by default in Rust

//use std::collections::HashMap; //Implemented by default in Rust
//use std::time::Duration; //Implemented by default in Rust

//use std::sync::mpsc::{channel, Sender, Receiver}; //Implemented by default in Rust

const K_NO_THREAD: pthread_t = 0;

type pthread_t = usize;

#[repr(C)]
struct procfs_mapinfo {
    vaddr: usize,
    size: usize,
    flags: c_int,
    // ... other fields
}

#[repr(C)]
struct procfs_debuginfo {
    vaddr: usize,
    path: [c_char; PATH_MAX],
    // ... other fields
}

#[allow(dead_code)]
extern "C" {
    fn snprintf(buf: *mut c_char, size: libc::size_t, format: *const c_char, ...) -> c_int;
    fn devctl(fd: c_int, dcmd: c_int, in_data: *mut c_void, in_size: libc::size_t, out_size_or_null: *mut c_int) -> c_int;
}

const DCMD_PROC_MAPINFO: c_int = 1; // Assuming a value, needs to be defined based on QNX API
const DCMD_PROC_PAGEDATA: c_int = 2; // Assuming a value, needs to be defined based on QNX API
const DCMD_PROC_MAPDEBUG: c_int = 3; // Assuming a value, needs to be defined based on QNX API

pub mod base {
    use super::*;
    use super::platform_posix_time::PosixDefaultTimezoneCache;
    use super::platform::OS;
    use std::path::PathBuf;
    use std::ffi::CStr;

    impl OS {
        #[cfg(target_arch = "arm")]
        pub fn arm_using_hard_float() -> bool {
            arm::arm_using_hard_float()
        }

        #[cfg(not(target_arch = "arm"))]
        pub fn arm_using_hard_float() -> bool {
            false
        }

        pub fn create_timezone_cache() -> Box<dyn platform::TimezoneCache> {
            Box::new(PosixDefaultTimezoneCache::new())
        }

        pub fn get_shared_library_addresses() -> Vec<platform::SharedLibraryAddress> {
            let mut result = Vec::new();
            let mut mapinfos: *mut procfs_mapinfo = ptr::null_mut();
            let mut proc_fd: c_int;
            let mut num: c_int = 0;
            let mut i: c_int;

            #[repr(C)]
            struct Map {
                info: procfs_debuginfo,
                buff: [c_char; PATH_MAX],
            }

            let mut map: Map = Map {
                info: procfs_debuginfo {
                    vaddr: 0,
                    path: [0; PATH_MAX],
                },
                buff: [0; PATH_MAX],
            };

            let mut buf: [c_char; PATH_MAX + 1] = [0; PATH_MAX + 1];
            unsafe {
                snprintf(
                    buf.as_mut_ptr(),
                    (PATH_MAX + 1) as libc::size_t,
                    b"/proc/%d/as\0".as_ptr() as *const i8,
                    getpid(),
                );
            }

            let c_buf = unsafe { CStr::from_ptr(buf.as_ptr()) };
            let path = c_buf.to_str().unwrap();

            unsafe {
                proc_fd = open(path.as_ptr() as *const i8, O_RDONLY);
                if proc_fd == -1 {
                  close(proc_fd);
                    return result;
                }

                /* Get the number of map entries.  */
                if devctl(proc_fd, DCMD_PROC_MAPINFO, ptr::null_mut(), 0, &mut num) != EOK {
                    close(proc_fd);
                    return result;
                }

                mapinfos = malloc((num as usize) * mem::size_of::<procfs_mapinfo>()) as *mut procfs_mapinfo;
                if mapinfos == ptr::null_mut() {
                    close(proc_fd);
                    return result;
                }

                /* Fill the map entries.  */
                if devctl(proc_fd, DCMD_PROC_PAGEDATA, mapinfos as *mut c_void,
                           (num as usize) * mem::size_of::<procfs_mapinfo>(), &mut num) != EOK {
                    free(mapinfos as *mut c_void);
                    close(proc_fd);
                    return result;
                }

                for i in 0..num {
                    let mapinfo = mapinfos.add(i as usize);
                    if (*mapinfo).flags & MAP_ELF != 0 {
                        map.info.vaddr = (*mapinfo).vaddr;
                        if devctl(proc_fd, DCMD_PROC_MAPDEBUG, &mut map as *mut Map as *mut c_void, mem::size_of::<Map>(), ptr::null_mut()) != EOK {
                            continue;
                        }
                        let path_cstr = CStr::from_ptr(map.info.path.as_ptr());
                        let path_str = path_cstr.to_str().unwrap_or("");
                        result.push(platform::SharedLibraryAddress {
                            name: PathBuf::from(path_str),
                            start: (*mapinfo).vaddr as usize,
                            end: ((*mapinfo).vaddr + (*mapinfo).size) as usize,
                        });
                    }
                }
                free(mapinfos as *mut c_void);
                close(proc_fd);
                return result;
            }
        }

        pub fn signal_code_moving_gc() {
            // This function is empty on QNX.
        }

        pub fn adjust_scheduling_params() {
            // This function is empty on QNX.
        }

        pub fn get_first_free_memory_range_within(
            _boundary_start: usize,
            _boundary_end: usize,
            _minimum_size: usize,
            _alignment: usize,
        ) -> Option<platform::MemoryRange> {
            None
        }
    }
}