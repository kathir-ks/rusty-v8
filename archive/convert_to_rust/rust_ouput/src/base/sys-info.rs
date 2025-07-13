// Converted from V8 C++ source files:
// Header: sys-info.h
// Implementation: sys-info.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/base/sys-info.h
pub mod sys_info {
    use std::ffi;

    pub struct SysInfo {}

    impl SysInfo {
        pub fn number_of_processors() -> i32 {
            #[cfg(target_os = "openbsd")]
            {
                use std::os::raw::c_int;
                let mib: [c_int; 2] = [libc::CTL_HW, libc::HW_NCPU];
                let mut ncpu: c_int = 0;
                let mut len = std::mem::size_of::<c_int>() as libc::size_t;
                unsafe {
                    if libc::sysctl(
                        mib.as_ptr() as *mut c_int,
                        mib.len() as u32,
                        &mut ncpu as *mut c_int as *mut ffi::c_void,
                        &mut len,
                        std::ptr::null_mut(),
                        0,
                    ) != 0
                    {
                        return 1;
                    }
                }
                ncpu as i32
            }
            #[cfg(target_os = "zos")]
            {
                extern "C" {
                    fn __get_num_online_cpus() -> i32;
                }
                unsafe { __get_num_online_cpus() }
            }
            #[cfg(all(target_family = "unix", not(target_os = "fuchsia"), not(target_os = "zos"), not(target_os = "openbsd"), not(target_os = "starboard")))]
            {
                unsafe {
                    let result = libc::sysconf(libc::_SC_NPROCESSORS_ONLN);
                    if result == -1 {
                        return 1;
                    }
                    result as i32
                }
            }
            #[cfg(target_os = "windows")]
            {
                use winapi::um::sysinfoapi::{GetNativeSystemInfo, SYSTEM_INFO};
                let mut system_info: SYSTEM_INFO = unsafe { std::mem::zeroed() };
                unsafe {
                    GetNativeSystemInfo(&mut system_info);
                }
                system_info.dwNumberOfProcessors as i32
            }
            #[cfg(target_os = "starboard")]
            {
                extern "C" {
                    fn SbSystemGetNumberOfProcessors() -> i32;
                }
                unsafe { SbSystemGetNumberOfProcessors() }
            }
        }

        pub fn amount_of_physical_memory() -> i64 {
            #[cfg(target_os = "darwin")]
            {
                use std::os::raw::c_int;
                let mut mib: [c_int; 2] = [libc::CTL_HW, libc::HW_MEMSIZE];
                let mut memsize: i64 = 0;
                let mut len = std::mem::size_of::<i64>() as libc::size_t;
                unsafe {
                    if libc::sysctl(
                        mib.as_ptr() as *mut c_int,
                        mib.len() as u32,
                        &mut memsize as *mut i64 as *mut ffi::c_void,
                        &mut len,
                        std::ptr::null_mut(),
                        0,
                    ) != 0
                    {
                        return 0;
                    }
                }
                memsize
            }
            #[cfg(target_os = "freebsd")]
            {
                use std::ffi::CString;
                let mut pages: i32 = 0;
                let mut page_size: i32 = 0;
                let mut size = std::mem::size_of::<i32>() as libc::size_t;

                unsafe {
                    let vm_page_count = CString::new("vm.stats.vm.v_page_count").unwrap();
                    libc::sysctlbyname(
                        vm_page_count.as_ptr(),
                        &mut pages as *mut i32 as *mut ffi::c_void,
                        &mut size,
                        std::ptr::null_mut(),
                        0,
                    );

                    let vm_page_size = CString::new("vm.stats.vm.v_page_size").unwrap();
                    libc::sysctlbyname(
                        vm_page_size.as_ptr(),
                        &mut page_size as *mut i32 as *mut ffi::c_void,
                        &mut size,
                        std::ptr::null_mut(),
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
                use winapi::um::sysinfoapi::{GlobalMemoryStatusEx, MEMORYSTATUSEX};

                let mut memory_info: MEMORYSTATUSEX = unsafe { std::mem::zeroed() };
                memory_info.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;
                unsafe {
                    if GlobalMemoryStatusEx(&mut memory_info) == 0 {
                        return 0;
                    }
                }
                let result = memory_info.ullTotalPhys as i64;
                if result < 0 {
                    std::i64::MAX
                } else {
                    result
                }
            }
            #[cfg(target_os = "qnx")]
            {
                let mut stat_buf: libc::stat = unsafe { std::mem::zeroed() };
                unsafe {
                    if libc::stat("/proc".as_ptr() as *const i8, &mut stat_buf) != 0 {
                        return 0;
                    }
                    stat_buf.st_size as i64
                }
            }
            #[cfg(target_os = "aix")]
            {
                unsafe {
                    let result = libc::sysconf(libc::_SC_AIX_REALMEM);
                    result as i64 * 1024
                }
            }
            #[cfg(target_os = "zos")]
            {
                extern "C" {
                    fn __get_num_frames() -> i32;
                }
                unsafe {
                    let pages = __get_num_frames();
                    let page_size = libc::sysconf(libc::_SC_PAGESIZE);
                    pages as u64 * page_size as u64 as u64 as i64
                }
            }
            #[cfg(all(target_family = "unix", not(target_os = "fuchsia"), not(target_os = "aix"), not(target_os = "qnx"), not(target_os = "cygwin"), not(target_os = "darwin"), not(target_os = "freebsd"), not(target_os = "starboard"), not(target_os = "zos")))]
            {
                unsafe {
                    let pages = libc::sysconf(libc::_SC_PHYS_PAGES);
                    let page_size = libc::sysconf(libc::_SC_PAGESIZE);
                    if pages == -1 || page_size == -1 {
                        return 0;
                    }
                    pages as i64 * page_size as i64
                }
            }
            #[cfg(target_os = "starboard")]
            {
                extern "C" {
                    fn SbSystemGetTotalCPUMemory() -> i64;
                }
                unsafe { SbSystemGetTotalCPUMemory() }
            }
        }

        pub fn amount_of_virtual_memory() -> i64 {
            #[cfg(any(target_os = "windows", target_os = "fuchsia"))]
            {
                0
            }
            #[cfg(all(target_family = "unix", not(target_os = "fuchsia"), not(target_os = "starboard")))]
            {
                let mut rlim: libc::rlimit = unsafe { std::mem::zeroed() };
                let result = unsafe { libc::getrlimit(libc::RLIMIT_DATA, &mut rlim) };
                if result != 0 {
                    return 0;
                }
                if rlim.rlim_cur == libc::RLIM_INFINITY {
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

        pub fn address_space_end() -> usize {
            #[cfg(target_os = "windows")]
            {
                use winapi::um::sysinfoapi::{GetSystemInfo, SYSTEM_INFO};
                let mut info: SYSTEM_INFO = unsafe { std::mem::zeroed() };
                unsafe {
                    GetSystemInfo(&mut info);
                }
                let max_address = info.lpMaximumApplicationAddress as usize;
                max_address + 1
            }
            #[cfg(not(target_os = "windows"))]
            {
                std::usize::MAX
            }
        }
    }
}
