// src/base/sys_info.rs

#![allow(non_snake_case)]

#[cfg(target_os = "openbsd")]
use libc::{sysctl, CTL_HW, HW_NCPU};

#[cfg(target_os = "zos")]
extern "C" {
    fn __get_num_online_cpus() -> i32;
    fn __get_num_frames() -> i32;
}

#[cfg(target_family = "unix")]
use libc::{sysconf, _SC_NPROCESSORS_ONLN, _SC_PHYS_PAGES, _SC_PAGESIZE};

#[cfg(target_os = "windows")]
use winapi::um::sysinfoapi::{GetNativeSystemInfo, SYSTEM_INFO};

#[cfg(target_os = "starboard")]
extern "C" {
    fn SbSystemGetNumberOfProcessors() -> i32;
    fn SbSystemGetTotalCPUMemory() -> i64;
}

#[cfg(target_os = "darwin")]
use libc::{sysctlbyname, CTL_HW, HW_MEMSIZE};

#[cfg(target_os = "freebsd")]
use libc::sysctlbyname;

#[cfg(any(target_os = "cygwin", target_os = "windows"))]
use winapi::um::sysinfoapi::{GlobalMemoryStatusEx, MEMORYSTATUSEX};

#[cfg(target_os = "qnx")]
use libc::stat;

#[cfg(target_os = "aix")]
use libc::sysconf;
#[cfg(target_os = "aix")]
use libc::_SC_AIX_REALMEM;


#[cfg(target_family = "unix")]
use libc::{getrlimit, rlimit, RLIMIT_DATA, RLIM_INFINITY};

use std::mem;
use std::ptr;
use std::i64;


pub struct SysInfo {}

impl SysInfo {
    /// Returns the number of processors.
    pub fn number_of_processors() -> i32 {
        #[cfg(target_os = "openbsd")]
        {
            let mut mib: [i32; 2] = [CTL_HW, HW_NCPU];
            let mut ncpu: i32 = 0;
            let mut len = mem::size_of::<i32>() as libc::size_t;

            unsafe {
                if sysctl(mib.as_mut_ptr(), 2, &mut ncpu as *mut i32 as *mut libc::c_void, &mut len, ptr::null_mut(), 0) != 0 {
                    return 1;
                }
            }
            ncpu
        }
        #[cfg(target_os = "zos")]
        {
            unsafe { __get_num_online_cpus() }
        }
        #[cfg(target_family = "unix")]
        {
            let result = unsafe { sysconf(_SC_NPROCESSORS_ONLN) };
            if result == -1 {
                return 1;
            }
            result as i32
        }
        #[cfg(target_os = "windows")]
        {
            let mut system_info: SYSTEM_INFO = unsafe { mem::zeroed() };
            unsafe { GetNativeSystemInfo(&mut system_info) };
            system_info.dwNumberOfProcessors as i32
        }
        #[cfg(target_os = "starboard")]
        {
            unsafe { SbSystemGetNumberOfProcessors() }
        }
    }

    /// Returns the amount of physical memory in bytes.
    pub fn amount_of_physical_memory() -> i64 {
        #[cfg(target_os = "darwin")]
        {
            let mut mib: [i32; 2] = [CTL_HW, HW_MEMSIZE];
            let mut memsize: i64 = 0;
            let mut len = mem::size_of::<i64>() as libc::size_t;

            unsafe {
                if sysctl(mib.as_mut_ptr(), 2, &mut memsize as *mut i64 as *mut libc::c_void, &mut len, ptr::null_mut(), 0) != 0 {
                    return 0;
                }
            }
            memsize
        }
        #[cfg(target_os = "freebsd")]
        {
            let mut pages: i32 = 0;
            let mut page_size: i32 = 0;
            let mut size = mem::size_of::<i32>() as libc::size_t;

            unsafe {
                sysctlbyname(
                    b"vm.stats.vm.v_page_count\0".as_ptr() as *const i8,
                    &mut pages as *mut i32 as *mut libc::c_void,
                    &mut size,
                    ptr::null_mut(),
                    0,
                );
                sysctlbyname(
                    b"vm.stats.vm.v_page_size\0".as_ptr() as *const i8,
                    &mut page_size as *mut i32 as *mut libc::c_void,
                    &mut size,
                    ptr::null_mut(),
                    0,
                );
            }
            if pages == -1 || page_size == -1 {
                return 0;
            }
            pages as i64 * page_size as i64
        }
        #[cfg(any(target_os = "cygwin", target_os = "windows"))]
        {
            let mut memory_info: MEMORYSTATUSEX = unsafe { mem::zeroed() };
            memory_info.dwLength = mem::size_of::<MEMORYSTATUSEX>() as u32;
            if unsafe { GlobalMemoryStatusEx(&mut memory_info) } == 0 {
                return 0;
            }
            let mut result = memory_info.ullTotalPhys as i64;
            if result < 0 {
                result = i64::MAX;
            }
            result
        }
        #[cfg(target_os = "qnx")]
        {
            let mut stat_buf: libc::stat = unsafe { mem::zeroed() };
            if unsafe { stat(b"/proc\0".as_ptr() as *const i8, &mut stat_buf) } != 0 {
                return 0;
            }
            stat_buf.st_size as i64
        }
        #[cfg(target_os = "aix")]
        {
          let result = unsafe { sysconf(_SC_AIX_REALMEM) };
          result as i64 * 1024
        }
        #[cfg(target_os = "zos")]
        {
            let pages = unsafe { __get_num_frames() };
            let page_size = unsafe { sysconf(_SC_PAGESIZE) };
            pages as u64 * page_size
        }
        #[cfg(target_family = "unix")]
        {
            let pages = unsafe { sysconf(_SC_PHYS_PAGES) };
            let page_size = unsafe { sysconf(_SC_PAGESIZE) };
            if pages == -1 || page_size == -1 {
                return 0;
            }
            pages as i64 * page_size as i64
        }
        #[cfg(target_os = "starboard")]
        {
            unsafe { SbSystemGetTotalCPUMemory() }
        }
    }

    /// Returns the amount of virtual memory in bytes.
    pub fn amount_of_virtual_memory() -> i64 {
        #[cfg(any(target_os = "windows", target_os = "fuchsia"))]
        {
            0
        }
        #[cfg(target_family = "unix")]
        {
            let mut rlim: rlimit = unsafe { mem::zeroed() };
            let result = unsafe { getrlimit(RLIMIT_DATA, &mut rlim) };
            if result != 0 {
                return 0;
            }
            if rlim.rlim_cur == RLIM_INFINITY {
                0
            } else {
                rlim.rlim_cur as i64
            }
        }
        #[cfg(target_os = "starboard")]
        {
            0
        }
    }

    /// Returns the end of the address space.
    pub fn address_space_end() -> usize {
        #[cfg(target_os = "windows")]
        {
            let mut info: SYSTEM_INFO = unsafe { mem::zeroed() };
            unsafe { winapi::um::sysinfoapi::GetSystemInfo(&mut info) };
            let max_address = info.lpMaximumApplicationAddress as usize;
            max_address + 1
        }
        #[cfg(not(target_os = "windows"))]
        {
            usize::MAX
        }
    }
}