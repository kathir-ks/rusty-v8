use std::fs::File;
use std::io::{BufRead, BufReader};
use std::os::unix::io::AsRawFd;
use std::path::Path;
use std::ptr;
use std::{mem, str};
use nix::sys::mman::{mmap, munmap, MapFlags, ProtFlags};
use nix::unistd::{sysconf, SysconfVar};
use std::ffi::{CString, CStr};
use std::os::raw::c_void;

pub mod base {
    pub mod platform {
        pub mod platform_posix_time;
        pub mod platform_posix;
        pub mod platform;
    }
}

use base::platform::platform_posix_time::PosixDefaultTimezoneCache;
use base::platform::platform::{TimezoneCache, SharedLibraryAddress, OS, Address, MemoryRange};

impl OS {
    pub fn create_timezone_cache() -> Box<dyn TimezoneCache> {
        Box::new(PosixDefaultTimezoneCache::new())
    }

    pub fn get_shared_library_addresses() -> Vec<SharedLibraryAddress> {
        let mut result = Vec::new();

        let file = match File::open("/proc/self/maps") {
            Ok(file) => file,
            Err(_) => return result,
        };

        let reader = BufReader::new(file);
        for line_result in reader.lines() {
            let line = match line_result {
                Ok(line) => line,
                Err(_) => break,
            };

            let parts: Vec<&str> = line.splitn(2, ' ').collect();
            if parts.len() < 1 {
                continue;
            }

            let address_and_perms = parts[0];
            let address_and_perms_parts: Vec<&str> = address_and_perms.split('-').collect();

            if address_and_perms_parts.len() != 2 {
                continue;
            }
            
            let start_str = address_and_perms_parts[0];
            let end_str = address_and_perms_parts[1];

            let start = match usize::from_str_radix(start_str, 16) {
                Ok(start) => start,
                Err(_) => continue,
            };

            let end = match usize::from_str_radix(end_str, 16) {
                Ok(end) => end,
                Err(_) => continue,
            };
            
            let mut perms = "";
            if parts.len() > 1 {
                let perms_and_path = parts[1];
                let perms_and_path_parts: Vec<&str> = perms_and_path.splitn(2, ' ').collect();

                if perms_and_path_parts.len() > 0 {
                   perms = perms_and_path_parts[0];
                }
            }
            

            if perms.len() >= 4 {
                let r = perms.chars().nth(0).unwrap();
                let w = perms.chars().nth(1).unwrap();
                let x = perms.chars().nth(2).unwrap();

                if r == 'r' && w != 'w' && x == 'x' {
                    let mut lib_name = String::new();

                    if parts.len() > 1 {
                        let perms_and_path = parts[1];
                        let perms_and_path_parts: Vec<&str> = perms_and_path.splitn(2, ' ').collect();
                        if perms_and_path_parts.len() > 1 {
                            let path = perms_and_path_parts[1].trim();
                            lib_name = path.to_string();
                        } else {
                            lib_name = format!("{:x}-{:x}", start, end);
                        }
                    } else {
                        lib_name = format!("{:x}-{:x}", start, end);
                    }

                    result.push(SharedLibraryAddress {
                        name: lib_name,
                        start: start as Address,
                        end: end as Address,
                    });
                }
            }
        }

        result
    }

    pub fn signal_code_moving_gc() {
        // Support for ll_prof.py.
        //
        // The Linux profiler built into the kernel logs all mmap's with
        // PROT_EXEC so that analysis tools can properly attribute ticks. We
        // do a mmap with a name known by ll_prof.py and immediately munmap
        // it. This injects a GC marker into the stream of events generated
        // by the kernel and allows us to synchronize V8 code log and the
        // kernel log.
        let page_size = sysconf(SysconfVar::PAGE_SIZE).unwrap().unwrap() as usize;

        let file_path = OS::get_gc_fake_mmap_file();

        let file = match File::create(&file_path) {
            Ok(f) => f,
            Err(e) => {
                OS::print_error(&format!("Failed to open {}: {}", file_path, e));
                OS::abort();
                unreachable!();
            }
        };

        let addr = unsafe {
            mmap(
                ptr::null_mut(),
                page_size,
                ProtFlags::PROT_READ | ProtFlags::PROT_EXEC,
                MapFlags::MAP_PRIVATE,
                file.as_raw_fd(),
                0,
            ).unwrap()
        };

        OS::free(addr as *mut c_void, page_size);
        drop(file);
    }

    pub fn adjust_scheduling_params() {}

    pub fn get_first_free_memory_range_within(
        _boundary_start: Address,
        _boundary_end: Address,
        _minimum_size: usize,
        _alignment: usize,
    ) -> Option<MemoryRange> {
        None
    }

    fn print_error(message: &str) {
        eprintln!("{}", message);
    }

    fn abort() {
        panic!("Aborted");
    }

    fn free(addr: *mut c_void, size: usize) {
        unsafe {
            munmap(addr, size).unwrap();
        }
    }
}