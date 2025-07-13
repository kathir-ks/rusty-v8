// Converted from V8 C++ source files:
// Header: cov.h
// Implementation: cov.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use std::sync::Mutex;

const SHM_SIZE: usize = 0x100000;
const MAX_EDGES: usize = (SHM_SIZE - 4) * 8;

struct shmem_data {
    num_edges: u32,
    edges: [u8; SHM_SIZE - 4], // Assuming edges fills the remaining space
}

static mut SHMEM: *mut shmem_data = std::ptr::null_mut();
static SHMEM_MUTEX: Mutex<()> = Mutex::new(());

static mut EDGES_START: *mut u32 = std::ptr::null_mut();
static mut EDGES_STOP: *mut u32 = std::ptr::null_mut();
static mut BUILTINS_START: u32 = 0;
static mut BUILTINS_EDGE_COUNT: u32 = 0;

pub fn sanitizer_cov_reset_edgeguards() {
    unsafe {
        let mut n: u32 = 0;
        let mut x = EDGES_START;
        while x < EDGES_STOP && n < MAX_EDGES as u32 {
            *x = n + 1;
            n += 1;
            x = x.add(1);
        }
    }
}

#[no_mangle]
pub extern "C" fn __sanitizer_cov_trace_pc_guard_init(start: *mut u32, stop: *mut u32) {
    unsafe {
        let _lock = SHMEM_MUTEX.lock().unwrap();

        if !SHMEM.is_null() {
            if EDGES_START == start && EDGES_STOP == stop {
                return; // Already initialized
            } else {
                eprintln!(
                    "[COV] Multiple initialization of shmem! This is probably not intended! Currently only one edge region is supported"
                );
                std::process::exit(-1);
            }
        }

        let shm_key = std::env::var("SHM_ID").ok();

        if shm_key.is_none() {
            eprintln!("[COV] no shared memory bitmap available, skipping");
            SHMEM = Box::into_raw(Box::new(shmem_data {
                num_edges: 0,
                edges: [0u8; SHM_SIZE - 4],
            }));
        } else {
            use std::os::unix::io::AsRawFd;
            use std::fs::OpenOptions;
            use std::io::Write;
            use std::mem::size_of;
            use std::slice;

            let shm_key = shm_key.unwrap();
            let fd_result = shm_open(&shm_key, O_RDWR, S_IREAD | S_IWRITE);

            let fd = match fd_result {
                Ok(fd) => fd,
                Err(_) => {
                  eprintln!("[COV] Failed to open shared memory region");
                  std::process::exit(-1);
                }
            };
            
            let mmap_result = mmap(
                std::ptr::null_mut(),
                SHM_SIZE,
                PROT_READ | PROT_WRITE,
                MAP_SHARED,
                fd,
                0,
            );
            
            SHMEM = match mmap_result {
                Ok(ptr) => ptr as *mut shmem_data,
                Err(_) => {
                    eprintln!("[COV] Failed to mmap shared memory region");
                    std::process::exit(-1);
                }
            };
        }

        EDGES_START = start;
        EDGES_STOP = stop;
        sanitizer_cov_reset_edgeguards();

        (*SHMEM).num_edges = stop.offset_from(start) as u32;
        BUILTINS_START = 1 + (*SHMEM).num_edges;
        eprintln!(
            "[COV] edge counters initialized. Shared memory: {:?} with {} edges",
            std::env::var("SHM_ID"),
            (*SHMEM).num_edges
        );
    }
}

pub fn sanitizer_cov_count_discovered_edges() -> u32 {
    unsafe {
        let mut on_edges_counter: u32 = 0;
        let mut i: u32 = 1;
        while i < BUILTINS_START {
            let byte_index = (i >> 3) as usize;
            let bit_index = (i & 7) as u8;

            if ((*SHMEM).edges[byte_index] & (1 << bit_index)) != 0 {
                on_edges_counter += 1;
            }
            i += 1;
        }
        on_edges_counter
    }
}

#[no_mangle]
pub extern "C" fn __sanitizer_cov_trace_pc_guard(guard: *mut u32) {
    unsafe {
        let index = *guard;
        let byte_index = (index / 8) as usize;
        let bit_index = (index % 8) as u8;

        (*SHMEM).edges[byte_index] |= 1 << bit_index;
        *guard = 0;
    }
}

pub fn cov_init_builtins_edges(num_edges: u32) {
    unsafe {
        if num_edges + (*SHMEM).num_edges > MAX_EDGES as u32 {
            eprintln!(
                "[COV] Error: Insufficient amount of edges left for builtins coverage."
            );
            std::process::exit(-1);
        }
        BUILTINS_EDGE_COUNT = num_edges;
        BUILTINS_START = 1 + (*SHMEM).num_edges;
        (*SHMEM).num_edges += BUILTINS_EDGE_COUNT;
        eprintln!("[COV] Additional {} edges for builtins initialized.", num_edges);
    }
}

pub fn cov_update_builtins_basic_block_coverage(cov_map: &Vec<bool>) {
    unsafe {
        if cov_map.len() != BUILTINS_EDGE_COUNT as usize {
            eprintln!("[COV] Error: Size of builtins cov map changed.");
            std::process::exit(-1);
        }

        for i in 0..cov_map.len() {
            if cov_map[i] {
                let byte_index = ((i as u32 + BUILTINS_START) >> 3) as usize;
                let bit_index = ((i as u32 + BUILTINS_START) & 7) as u8;

                (*SHMEM).edges[byte_index] |= 1 << bit_index;
            }
        }
    }
}

extern "C" {
    fn shm_open(name: *const std::ffi::c_char, oflag: i32, mode: i32) -> Result<i32, i32>;
    fn mmap(addr: *mut std::ffi::c_void, len: usize, prot: i32, flags: i32, fd: i32, offset: i64) -> Result<*mut std::ffi::c_void, i32>;
}

const O_RDWR: i32 = 0x0002;
const S_IREAD: i32 = 0o0400;
const S_IWRITE: i32 = 0o0200;
const PROT_READ: i32 = 0x1;
const PROT_WRITE: i32 = 0x2;
const MAP_SHARED: i32 = 0x0001;

#[cfg(unix)]
mod unix {
    use std::ffi::CString;
    use std::io;
    use std::os::unix::io::RawFd;
    use std::ptr;

    pub fn shm_open(name: &str, oflag: i32, mode: i32) -> Result<RawFd, io::Error> {
        let c_name = CString::new(name).map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid shm name"))?;
        let fd = unsafe { libc::shm_open(c_name.as_ptr(), oflag, mode) };
        if fd < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(fd)
        }
    }

    pub fn mmap(addr: *mut std::ffi::c_void, len: usize, prot: i32, flags: i32, fd: RawFd, offset: i64) -> Result<*mut std::ffi::c_void, io::Error> {
        let ptr = unsafe { libc::mmap(addr, len, prot, flags, fd, offset) };
        if ptr == libc::MAP_FAILED {
            Err(io::Error::last_os_error())
        } else {
            Ok(ptr)
        }
    }
}

#[cfg(unix)]
use unix::*;

#[cfg(not(unix))]
mod windows {
    use std::io;
    use std::os::windows::io::RawHandle;
    use std::ptr;
    
    pub fn shm_open(name: &str, oflag: i32, mode: i32) -> Result<RawHandle, io::Error> {
        // Placeholder implementation for Windows.
        // You'd need to use Windows API functions like CreateFileMapping, OpenFileMapping, etc.
        // This example returns an error.
        Err(io::Error::new(io::ErrorKind::Other, "shm_open not implemented for Windows"))
    }
    
    pub fn mmap(addr: *mut std::ffi::c_void, len: usize, prot: i32, flags: i32, fd: RawHandle, offset: i64) -> Result<*mut std::ffi::c_void, io::Error> {
        // Placeholder implementation for Windows.
        // You'd need to use Windows API functions like MapViewOfFile.
        // This example returns an error.
        Err(io::Error::new(io::ErrorKind::Other, "mmap not implemented for Windows"))
    }
}

#[cfg(not(unix))]
use windows::*;
