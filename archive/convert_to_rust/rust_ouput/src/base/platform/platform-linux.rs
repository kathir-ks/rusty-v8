// Converted from V8 C++ source files:
// Header: platform-linux.h
// Implementation: platform-linux.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod platform_linux {
    use std::ffi::CString;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::mem::MaybeUninit;
    use std::num::ParseIntError;
    use std::os::unix::fs::MetadataExt;
    use std::path::Path;
    use std::ptr;
    use std::sync::Mutex;
    use std::{fmt, io, mem, str};

    use libc::{
        c_char, c_int, dev_t, fclose, feof, fileno, fopen, fprintf, fstat,
        fscanf, getc, makedev, mmap, munmap, open, prctl, sysconf, MAP_FAILED,
        MAP_FIXED, MAP_PRIVATE, MREMAP_FIXED, MREMAP_MAYMOVE, O_RDONLY, PROT_EXEC,
        PROT_READ, rusage, sem_t, sigset_t, stat, syscall, timeval,
        RLIMIT_NOFILE, SC_PAGESIZE, SYS_gettid,
    };

    //use crate::base::platform::platform::{Mutex};
    use crate::base::logging::DCHECK_NE;
    use crate::base::macros::DCHECK;
    use crate::base::platform::platform::{
        AllocatePageSize, Free, GetGCFakeMMapFile, GetRandomMmapAddr, MemoryPermission,
        OSSharedLibraryAddress,
    };
    use crate::base::platform::platform_posix::{
        GetProtectionFromMemoryPermission, PosixDefaultTimezoneCache,
    };
    use crate::base::platform::platform_posix_time::TimezoneCache;
    use crate::base::time::V8;

    pub struct MemoryRegion {
        pub start: usize,
        pub end: usize,
        pub permissions: [c_char; 5],
        pub offset: usize,
        pub dev: dev_t,
        pub inode: u64,
        pub pathname: String,
    }

    impl MemoryRegion {
        // |line| must not contains the tail '\n'.
        pub fn from_maps_line(line: &str) -> Option<MemoryRegion> {
            let mut region = MemoryRegion {
                start: 0,
                end: 0,
                permissions: [0; 5],
                offset: 0,
                dev: 0,
                inode: 0,
                pathname: String::new(),
            };

            // Split the line into its components
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 5 {
                return None; // Invalid format
            }

            // Parse the address range
            let address_range: Vec<&str> = parts[0].split('-').collect();
            if address_range.len() != 2 {
                return None;
            }
            region.start = match usize::from_str_radix(address_range[0], 16) {
                Ok(val) => val,
                Err(_) => return None,
            };
            region.end = match usize::from_str_radix(address_range[1], 16) {
                Ok(val) => val,
                Err(_) => return None,
            };

            // Copy permissions
            let permissions = parts[1].as_bytes();
            if permissions.len() != 4 {
                return None;
            }
            for i in 0..4 {
                region.permissions[i] = permissions[i] as c_char;
            }
            region.permissions[4] = 0;

            // Parse offset
            region.offset = match usize::from_str_radix(parts[2], 16) {
                Ok(val) => val,
                Err(_) => return None,
            };

            // Parse device
            let device: Vec<&str> = parts[3].split(':').collect();
            if device.len() != 2 {
                return None;
            }
            let dev_major = match u32::from_str_radix(device[0], 16) {
                Ok(val) => val,
                Err(_) => return None,
            };
            let dev_minor = match u32::from_str_radix(device[1], 16) {
                Ok(val) => val,
                Err(_) => return None,
            };
            region.dev = unsafe { makedev(dev_major, dev_minor) };

            // Parse inode
            region.inode = match u64::from_str_radix(parts[4], 10) {
                Ok(val) => val,
                Err(_) => return None,
            };

            // Copy pathname if present
            if parts.len() > 5 {
                region.pathname = parts[5..].join(" ");
            }

            Some(region)
        }
    }

    pub fn get_shared_library_addresses(fp: *mut libc::FILE) -> Vec<OSSharedLibraryAddress> {
        let regions = parse_proc_self_maps(
            fp,
            |region: &MemoryRegion| {
                region.permissions[0] == 'r' as i8
                    && region.permissions[1] == '-' as i8
                    && region.permissions[2] == 'x' as i8
            },
            false,
        );

        match regions {
            Some(regions) => {
                let mut result: Vec<OSSharedLibraryAddress> = Vec::new();
                for region in regions.iter() {
                    let mut start = region.start;
                    start -= region.offset;
                    result.push(OSSharedLibraryAddress {
                        name: region.pathname.clone(),
                        start: start,
                        end: region.end,
                    });
                }
                result
            }
            None => Vec::new(),
        }
    }

    pub mod OS {
        use super::*;

        pub fn create_timezone_cache() -> *mut TimezoneCache {
            let cache = Box::new(PosixDefaultTimezoneCache::new());
            Box::into_raw(cache)
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
            let size = unsafe { sysconf(SC_PAGESIZE) } as usize;
            let filename = GetGCFakeMMapFile();
            let filename_cstr = CString::new(filename).unwrap();

            let f = unsafe { fopen(filename_cstr.as_ptr(), CString::new("w+").unwrap().as_ptr()) };
            if f.is_null() {
                eprintln!("Failed to open {}", filename);
                abort();
            }

            let addr = unsafe {
                mmap(
                    GetRandomMmapAddr() as *mut libc::c_void,
                    size,
                    PROT_READ | PROT_EXEC,
                    MAP_PRIVATE,
                    fileno(f),
                    0,
                )
            };

            DCHECK_NE!(addr, MAP_FAILED);
            unsafe { Free(addr, size) };
            unsafe { fclose(f) };
        }

        pub fn adjust_scheduling_params() {}

        pub fn remap_shared(old_address: *mut libc::c_void, new_address: *mut libc::c_void, size: usize) -> *mut libc::c_void {
            unsafe {
                let result = mremap(
                    old_address,
                    0,
                    size,
                    MREMAP_FIXED | MREMAP_MAYMOVE,
                    new_address,
                );
                if result == MAP_FAILED {
                    return ptr::null_mut();
                }
                DCHECK!(result == new_address);
                result
            }
        }

        fn round_up(value: usize, alignment: usize) -> usize {
            (value + alignment - 1) & !(alignment - 1)
        }

        fn round_down(value: usize, alignment: usize) -> usize {
            value & !(alignment - 1)
        }

        pub fn get_first_free_memory_range_within(
            boundary_start: usize,
            boundary_end: usize,
            minimum_size: usize,
            alignment: usize,
        ) -> Option<(usize, usize)> {
            let mut result: Option<(usize, usize)> = None;
            let file = File::open("/proc/self/maps").ok()?;
            let reader = BufReader::new(file);

            let mut gap_start: usize = 0;
            let mut gap_end: usize = 0;

            for line_result in reader.lines() {
                let line = line_result.ok()?;
                let parts: Vec<&str> = line.split_whitespace().collect();

                if parts.len() < 2 {
                    continue;
                }

                let range_parts: Vec<&str> = parts[0].split('-').collect();
                if range_parts.len() != 2 {
                    continue;
                }

                let vm_start = usize::from_str_radix(range_parts[0], 16).ok()?;
                let vm_end = usize::from_str_radix(range_parts[1], 16).ok()?;

                if gap_start >= boundary_end {
                    break;
                }

                gap_end = vm_start;

                if gap_end > boundary_start {
                    let overlap_start =
                        round_up(std::cmp::max(gap_start, boundary_start), alignment);
                    let overlap_end =
                        round_down(std::cmp::min(gap_end, boundary_end), alignment);

                    if overlap_start < overlap_end && (overlap_end - overlap_start) >= minimum_size {
                        result = Some((overlap_start, overlap_end));
                        break;
                    }
                }

                gap_start = vm_end;
            }

            result
        }

        pub fn get_shared_library_addresses() -> Vec<OSSharedLibraryAddress> {
            super::get_shared_library_addresses(ptr::null_mut())
        }

        pub fn remap_pages(
            address: *const std::ffi::c_void,
            size: usize,
            new_address: *mut std::ffi::c_void,
            access: MemoryPermission,
        ) -> bool {
            let address_addr = address as usize;

            DCHECK!(is_aligned(address_addr, AllocatePageSize()));
            DCHECK!(is_aligned(new_address as usize, AllocatePageSize()));
            DCHECK!(is_aligned(size, AllocatePageSize()));

            let enclosing_region = find_enclosing_mapping(address_addr, size);

            if enclosing_region.start == 0 {
                return false;
            }

            if enclosing_region.pathname.is_empty() {
                return false;
            }

            let filename = enclosing_region.pathname.clone();
            let filename_cstr = CString::new(filename).unwrap();
            let fd = unsafe { open(filename_cstr.as_ptr(), O_RDONLY) };

            if fd == -1 {
                return false;
            }

            let mut stat_buf: stat = unsafe { mem::zeroed() };
            if unsafe { fstat(fd, &mut stat_buf) } != 0 {
                unsafe { close(fd) };
                return false;
            }

            if stat_buf.st_dev != enclosing_region.dev || stat_buf.st_ino != enclosing_region.inode {
                unsafe { close(fd) };
                return false;
            }

            let offset_in_mapping = address_addr - enclosing_region.start;
            let offset_in_file = enclosing_region.offset + offset_in_mapping;
            let protection = GetProtectionFromMemoryPermission(access);

            let mapped_address = unsafe {
                mmap(
                    new_address,
                    size,
                    protection,
                    MAP_FIXED | MAP_PRIVATE,
                    fd,
                    offset_in_file as i64,
                )
            };

            unsafe { close(fd) };

            if mapped_address != new_address {
                eprintln!("Mmap failed. Should not happen, MAP_FIXED should always map where we want");
                return false;
            }
            true
        }
    }

    fn parse_proc_self_maps(
        fp: *mut libc::FILE,
        predicate: impl Fn(&MemoryRegion) -> bool,
        early_stopping: bool,
    ) -> Option<Vec<MemoryRegion>> {
        let mut file: *mut libc::FILE = fp;
        if fp.is_null() {
            let filename = CString::new("/proc/self/maps").unwrap();
            file = unsafe { fopen(filename.as_ptr(), CString::new("r").unwrap().as_ptr()) };
            if file.is_null() {
                return None;
            }
        }

        const K_MAX_LINE_LENGTH: usize = 2 * 4096; //FILENAME_MAX;
        let mut line_buffer: Vec<u8> = vec![0; K_MAX_LINE_LENGTH];
        let mut result: Vec<MemoryRegion> = Vec::new();
        let mut error: bool = false;

        loop {
            error = true;
            let read_result = unsafe {
                fgets(
                    line_buffer.as_mut_ptr() as *mut libc::c_char,
                    K_MAX_LINE_LENGTH as i32,
                    file,
                )
            };
            if read_result.is_null() {
                if unsafe { feof(file) } != 0 {
                    error = false;
                }
                break;
            }

            let line = unsafe {
                let len = libc::strlen(line_buffer.as_ptr() as *const i8) as usize;
                str::from_utf8(&line_buffer[..len]).unwrap()
            };

            let region_option = MemoryRegion::from_maps_line(line);
            if region_option.is_none() {
                break;
            }
            let region = region_option.unwrap();

            error = false;

            if predicate(&region) {
                result.push(region);
                if early_stopping {
                    break;
                }
            }
        }

        if fp.is_null() {
            unsafe { fclose(file) };
        }

        if !error && !result.is_empty() {
            return Some(result);
        }

        None
    }

    fn find_enclosing_mapping(target_start: usize, size: usize) -> MemoryRegion {
        let result = parse_proc_self_maps(
            ptr::null_mut(),
            |region: &MemoryRegion| {
                region.start <= target_start && target_start + size < region.end
            },
            true,
        );
        match result {
            Some(regions) => regions[0].clone(),
            None => MemoryRegion {
                start: 0,
                end: 0,
                permissions: [0; 5],
                offset: 0,
                dev: 0,
                inode: 0,
                pathname: String::new(),
            },
        }
    }

    fn is_aligned(address: usize, alignment: usize) -> bool {
        address % alignment == 0
    }

    extern "C" {
        fn fgets(buf: *mut c_char, size: c_int, fp: *mut libc::FILE) -> *mut c_char;
        fn close(fd: c_int) -> c_int;
        fn strlen(s: *const i8) -> usize;
        fn abort() -> !;
    }
}
