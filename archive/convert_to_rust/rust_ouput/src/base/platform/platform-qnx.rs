// Converted from V8 C++ source files:
// Header: N/A
// Implementation: platform-qnx.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

use std::os::raw::{c_char, c_int, c_void};
use std::path::PathBuf;
use std::{mem, ptr, io};
use std::fs::File;
use std::io::Read;
use std::ffi::{CString, CStr};
use std::sync::Mutex;

//use crate::base::platform::platform_posix::PosixDefaultTimezoneCache;
use libc::{PATH_MAX, O_RDONLY, getpid, close, malloc, free, snprintf, devctl, EOK, MAP_ELF, sysconf};
use std::ptr::null_mut;
use std::convert::TryInto;

#[link(name = "c")]
extern "C" {
    fn pthread_self() -> libc::pthread_t;
}

pub mod base {
    pub mod platform {
        pub use crate::*;
        use std::path::PathBuf;
        use std::{mem, ptr, io};
        use std::fs::File;
        use std::io::Read;
        use std::ffi::{CString, CStr};
        use std::sync::Mutex;

        //use crate::base::platform::platform_posix::PosixDefaultTimezoneCache;
        use libc::{PATH_MAX, O_RDONLY, getpid, close, malloc, free, snprintf, devctl, EOK, MAP_ELF, sysconf};
        use std::ptr::null_mut;
        use std::convert::TryInto;

        #[link(name = "c")]
        extern "C" {
            fn pthread_self() -> libc::pthread_t;
        }
        
        pub struct PosixDefaultTimezoneCache {}
        impl PosixDefaultTimezoneCache {
            pub fn new() -> Self {
                PosixDefaultTimezoneCache{}
            }
        }

        // Mocked types and functions
        pub struct MemoryMappedFile {}
        impl MemoryMappedFile {
            pub enum FileMode {
                ReadOnly,
                ReadWrite,
            }
        }

        pub struct OS {}

        impl OS {
            #[cfg(target_arch = "arm")]
            pub fn ArmUsingHardFloat() -> bool {
                // Adapted from the C++ code
                let gcc_version =
                    env!("GCC_VERSION_MAJOR").parse::<i32>().unwrap() * 10000
                    + env!("GCC_VERSION_MINOR").parse::<i32>().unwrap() * 100
                    + env!("GCC_PATCHLEVEL").parse::<i32>().unwrap();

                if gcc_version >= 40600 {
                    if cfg!(defined(__ARM_PCS_VFP)) {
                        return true;
                    } else {
                        return false;
                    }
                } else if gcc_version < 40500 {
                    return false;
                } else {
                    if cfg!(defined(__ARM_PCS_VFP)) {
                        return true;
                    } else if cfg!(defined(__ARM_PCS)) || cfg!(defined(__SOFTFP__))
                        || cfg!(defined(__SOFTFP)) || !cfg!(defined(__VFP_FP__))
                    {
                        return false;
                    } else {
                        panic!("Your version of GCC does not report the FP ABI compiled for.");
                    }
                }
            }

            #[cfg(not(target_arch = "arm"))]
            pub fn ArmUsingHardFloat() -> bool {
                false
            }

            pub fn CreateTimezoneCache() -> PosixDefaultTimezoneCache {
                PosixDefaultTimezoneCache::new()
            }

            #[derive(Debug)]
            pub struct SharedLibraryAddress {
                path: String,
                start: usize,
                end: usize,
            }

            impl SharedLibraryAddress {
                pub fn new(path: String, start: usize, end: usize) -> Self {
                    SharedLibraryAddress { path, start, end }
                }
            }

            pub fn GetSharedLibraryAddresses() -> Vec<SharedLibraryAddress> {
                let mut result = Vec::new();
                let mut mapinfos: *mut procfs_mapinfo = ptr::null_mut();
                let mut num: c_int = 0;

                #[repr(C)]
                struct Map {
                    info: procfs_debuginfo,
                    buff: [c_char; PATH_MAX as usize],
                }

                let mut map: Map = Map {
                    info: procfs_debuginfo { vaddr: 0, path: [0; PATH_MAX as usize] },
                    buff: [0; PATH_MAX as usize],
                };

                let mut buf: [c_char; PATH_MAX as usize + 1] = [0; PATH_MAX as usize + 1];
                let pid = unsafe { getpid() };
                let as_path = format!("/proc/{}/as", pid);
                let as_path_cstr = CString::new(as_path).unwrap();
                unsafe {
                    snprintf(
                        buf.as_mut_ptr(),
                        buf.len(),
                        as_path_cstr.as_ptr(),
                    );
                }

                let proc_fd = unsafe { libc::open(buf.as_ptr(), O_RDONLY) };

                if proc_fd == -1 {
                   unsafe { close(proc_fd) };
                    return result;
                }

                /* Get the number of map entries.  */
                if unsafe { devctl(proc_fd, DCMD_PROC_MAPINFO, ptr::null_mut(), 0, &mut num) } != EOK {
                    unsafe { close(proc_fd) };
                    return result;
                }

                unsafe {
                    mapinfos = malloc((num as usize) * mem::size_of::<procfs_mapinfo>()) as *mut procfs_mapinfo;
                }
                if mapinfos.is_null() {
                    unsafe { close(proc_fd) };
                    return result;
                }

                /* Fill the map entries.  */
                if unsafe { devctl(proc_fd, DCMD_PROC_PAGEDATA, mapinfos as *mut c_void,
                                 (num as usize) * mem::size_of::<procfs_mapinfo>(), &mut num) } != EOK {
                    unsafe {
                        free(mapinfos as *mut c_void);
                        close(proc_fd);
                    }
                    return result;
                }

                for i in 0..num {
                    unsafe {
                        let mapinfo = mapinfos.add(i as usize);
                        if (*mapinfo).flags & MAP_ELF != 0 {
                            map.info.vaddr = (*mapinfo).vaddr;
                            if devctl(proc_fd, DCMD_PROC_MAPDEBUG, &mut map as *mut Map as *mut c_void, mem::size_of::<Map>() as u32, 0) != EOK {
                                continue;
                            }
                            let path = CStr::from_ptr(map.info.path.as_ptr()).to_string_lossy().into_owned();
                            result.push(SharedLibraryAddress::new(
                                path,
                                (*mapinfo).vaddr,
                                (*mapinfo).vaddr + (*mapinfo).size,
                            ));
                        }
                    }
                }
                unsafe {
                    free(mapinfos as *mut c_void);
                    close(proc_fd);
                }
                result
            }

            pub fn SignalCodeMovingGC() {}

            pub fn AdjustSchedulingParams() {}

            #[derive(Debug, PartialEq)]
            pub struct MemoryRange {
                pub start: usize,
                pub end: usize,
            }

            pub fn GetFirstFreeMemoryRangeWithin(
                boundary_start: usize,
                boundary_end: usize,
                minimum_size: usize,
                alignment: usize,
            ) -> Option<MemoryRange> {
                None
            }
        }

        #[repr(C)]
        pub struct procfs_mapinfo {
            pub vaddr: usize,
            pub size: usize,
            pub flags: c_int,
            pub inode: u64,
            pub device: u32,
            pub next: usize,
        }

        #[repr(C)]
        pub struct procfs_debuginfo {
            pub vaddr: usize,
            pub path: [c_char; PATH_MAX as usize],
        }

        const DCMD_PROC_MAPINFO: c_int = 64240;
        const DCMD_PROC_PAGEDATA: c_int = 64241;
        const DCMD_PROC_MAPDEBUG: c_int = 64243;
    }
}
