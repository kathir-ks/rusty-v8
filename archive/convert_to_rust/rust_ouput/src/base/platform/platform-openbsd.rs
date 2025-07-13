// Converted from V8 C++ source files:
// Header: N/A
// Implementation: platform-openbsd.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process;
use std::ptr;
use std::sync::atomic::Ordering;
use std::{ffi::CString, mem};
use std::io;

use libc::{
    fclose, fileno, fopen, free, fseek, ftell, getc, malloc, mmap, munmap, sysconf, ungetc, MAP_FAILED, MAP_PRIVATE, PROT_EXEC, PROT_READ, _SC_PAGESIZE,
};

#[cfg(target_os = "openbsd")]
use libc::FILENAME_MAX;
#[cfg(target_os = "netbsd")]
use libc::FILENAME_MAX;

use crate::base::platform::platform_posix_time::PosixDefaultTimezoneCache;
use crate::base::platform::platform_posix::OS;
use crate::base::macros::Use;

pub mod base {
    pub mod platform {
        pub mod platform_posix_time;
        pub mod platform_posix;
    }
    pub mod macros;
}

pub mod v8 {
    pub mod base {
        pub struct TimezoneCache {}
    }
}

impl OS {
    pub fn create_timezone_cache() -> Box<PosixDefaultTimezoneCache> {
        Box::new(PosixDefaultTimezoneCache::new())
    }

    pub fn get_shared_library_addresses() -> Vec<SharedLibraryAddress> {
        let mut result = Vec::new();
        if let Ok(file) = File::open("/proc/self/maps") {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(line) = line {
                    if let Some(address) = parse_map_entry(&line) {
                        result.push(address);
                    }
                }
            }
        }
        result
    }

    pub fn signal_code_moving_gc() {
        let size = unsafe { sysconf(_SC_PAGESIZE) } as usize;
        let filename = OS::get_gc_fake_mmap_file();
        match File::create(&filename) {
            Ok(f) => {
                let fd = f.as_raw_fd();
                let addr = unsafe {
                    mmap(
                        ptr::null_mut(),
                        size,
                        PROT_READ | PROT_EXEC,
                        MAP_PRIVATE,
                        fd,
                        0,
                    )
                };
                if addr == MAP_FAILED {
                    eprintln!("mmap failed: {}", io::Error::last_os_error());
                    process::abort();
                }
                if unsafe { munmap(addr, size) } != 0 {
                    eprintln!("munmap failed: {}", io::Error::last_os_error());
                }
            }
            Err(e) => {
                eprintln!("Failed to open {}: {}", filename.display(), e);
                process::abort();
            }
        }
    }

    pub fn adjust_scheduling_params() {}

    pub fn get_first_free_memory_range_within(
        boundary_start: usize,
        boundary_end: usize,
        minimum_size: usize,
        alignment: usize,
    ) -> Option<MemoryRange> {
        None
    }
}

#[cfg(target_os = "openbsd")]
use std::os::unix::io::AsRawFd;
#[cfg(target_os = "netbsd")]
use std::os::unix::io::AsRawFd;

#[derive(Debug)]
pub struct SharedLibraryAddress {
    name: String,
    start: usize,
    end: usize,
}

impl SharedLibraryAddress {
    pub fn new(name: String, start: usize, end: usize) -> Self {
        SharedLibraryAddress { name, start, end }
    }
}

fn parse_map_entry(line: &str) -> Option<SharedLibraryAddress> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 4 {
        return None;
    }

    let address_range = parts[0];
    let permissions = parts[1];

    if permissions.len() != 4 {
        return None;
    }

    if permissions.starts_with("r") && !permissions.contains("w") && permissions.ends_with("x") {
        let addresses: Vec<&str> = address_range.split("-").collect();
        if addresses.len() != 2 {
            return None;
        }

        let start = usize::from_str_radix(addresses[0], 16).ok()?;
        let end = usize::from_str_radix(addresses[1], 16).ok()?;

        let mut name = String::new();
        if parts.len() > 5 {
            name = parts[5..].join(" ");
        } else if parts.len() == 5 {
            name = parts[4].to_string();
        } else {
            name = format!("{:x}-{:x}", start, end);
        }

        return Some(SharedLibraryAddress::new(name, start, end));
    }

    None
}

#[derive(Debug)]
pub struct MemoryRange {
    start: usize,
    end: usize,
}

impl MemoryRange {
    pub fn new(start: usize, end: usize) -> Self {
        MemoryRange { start, end }
    }
}
