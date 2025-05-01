use std::env;
use std::fs::File;
use std::io::ErrorKind;
use std::mem::size_of;
use std::os::unix::io::AsRawFd;
use std::process;
use std::slice;
use std::sync::atomic::{AtomicU32, Ordering};

use nix::fcntl;
use nix::sys::mman::{mmap, MapFlags, ProtFlags};
use nix::sys::shm::{shm_open, ShmFlags};
use nix::sys::stat::Mode;
use nix::unistd::ftruncate;

const SHM_SIZE: usize = 0x100000;
const MAX_EDGES: usize = ((SHM_SIZE - 4) * 8) as usize;

#[repr(C)]
struct ShmemData {
    num_edges: AtomicU32,
    edges: [u8; SHM_SIZE - 4], // Changed to fixed-size array
}

static mut SHMEM: Option<*mut ShmemData> = None;
static mut EDGES_START: Option<*mut u32> = None;
static mut EDGES_STOP: Option<*mut u32> = None;
static mut BUILTINS_START: u32 = 0;
static mut BUILTINS_EDGE_COUNT: u32 = 0;

/// Resets edge guards by setting them to increasing numbers from 1 to MAX_EDGES.
pub fn sanitizer_cov_reset_edgeguards() {
    unsafe {
        if let (Some(edges_start), Some(edges_stop)) = (EDGES_START, EDGES_STOP) {
            let mut n: u32 = 0;
            let mut x = edges_start;
            while x < edges_stop && n < MAX_EDGES as u32 {
                *x = n + 1;
                n += 1;
                x = x.add(1);
            }
        }
    }
}

/// Initializes the shared memory region for coverage data.
///
/// This function is called by the sanitizer to initialize the coverage tracking.
pub fn __sanitizer_cov_trace_pc_guard_init(start: *mut u32, stop: *mut u32) {
    unsafe {
        if SHMEM.is_some() {
            if let (Some(edges_start), Some(edges_stop)) = (EDGES_START, EDGES_STOP) {
                if edges_start != start || edges_stop != stop {
                    eprintln!(
                        "[COV] Multiple initialization of shmem! This is probably not intended! \
                         Currently only one edge region is supported"
                    );
                    process::exit(-1);
                }
                return;
            }
        }

        let shm_key = env::var("SHM_ID").ok();
        let shmem_ptr = match shm_key {
            Some(key) => {
                let fd = shm_open(
                    key.as_str(),
                    ShmFlags::O_RDWR,
                    Mode::from_bits_truncate(0o600),
                )
                .unwrap_or_else(|_| {
                    eprintln!("[COV] Failed to open shared memory region");
                    process::exit(-1);
                });

                let addr = mmap(
                    None,
                    SHM_SIZE,
                    ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
                    MapFlags::MAP_SHARED,
                    fd,
                    0,
                )
                .unwrap_or_else(|_| {
                    eprintln!("[COV] Failed to mmap shared memory region");
                    process::exit(-1);
                });

                addr as *mut ShmemData
            }
            None => {
                eprintln!("[COV] no shared memory bitmap available, skipping");
                let layout = std::alloc::Layout::new::<ShmemData>();
                let ptr = std::alloc::alloc_zeroed(layout);
                if ptr.is_null() {
                    eprintln!("[COV] Failed to allocate memory for shmem");
                    process::exit(-1);
                }
                ptr as *mut ShmemData
            }
        };

        SHMEM = Some(shmem_ptr);
        EDGES_START = Some(start);
        EDGES_STOP = Some(stop);
        sanitizer_cov_reset_edgeguards();

        let shmem_data = SHMEM.map(|ptr| &*ptr).unwrap();
        let num_edges = (stop as usize - start as usize) / size_of::<u32>();
        shmem_data
            .num_edges
            .store(num_edges as u32, Ordering::Relaxed);
        BUILTINS_START = 1 + shmem_data.num_edges.load(Ordering::Relaxed);

        eprintln!(
            "[COV] edge counters initialized. Shared memory: {:?} with {} edges",
            shm_key,
            shmem_data.num_edges.load(Ordering::Relaxed)
        );
    }
}

/// Counts the number of discovered edges in the shared memory region.
pub fn sanitizer_cov_count_discovered_edges() -> u32 {
    unsafe {
        let shmem_data = SHMEM.map(|ptr| &*ptr).unwrap();
        let mut on_edges_counter: u32 = 0;
        for i in 1..BUILTINS_START {
            let byte_index = (i >> 3) as usize;
            let bit_index = (i & 7) as usize;

            if (shmem_data.edges[byte_index] & (1 << bit_index)) != 0 {
                on_edges_counter += 1;
            }
        }
        on_edges_counter
    }
}

/// Updates the coverage information for a specific edge.
///
/// This function is called by the sanitizer to mark an edge as covered.
pub fn __sanitizer_cov_trace_pc_guard(guard: *mut u32) {
    unsafe {
        let index = *guard;
        if let Some(shmem_ptr) = SHMEM {
            let shmem_data = &mut *shmem_ptr;
            let byte_index = (index / 8) as usize;
            let bit_index = (index % 8) as u8;
            shmem_data.edges[byte_index] |= 1 << bit_index;
            *guard = 0;
        }
    }
}

/// Initializes additional edges for built-in functions.
pub fn cov_init_builtins_edges(num_edges: u32) {
    unsafe {
        let shmem_data = SHMEM.map(|ptr| &*ptr).unwrap();
        if num_edges + shmem_data.num_edges.load(Ordering::Relaxed) > MAX_EDGES as u32 {
            eprintln!(
                "[COV] Error: Insufficient amount of edges left for builtins coverage."
            );
            process::exit(-1);
        }
        BUILTINS_EDGE_COUNT = num_edges;
        BUILTINS_START = 1 + shmem_data.num_edges.load(Ordering::Relaxed);
        shmem_data
            .num_edges
            .fetch_add(BUILTINS_EDGE_COUNT, Ordering::Relaxed);
        eprintln!("[COV] Additional {} edges for builtins initialized.", num_edges);
    }
}

/// Updates the coverage information for built-in functions based on a coverage map.
pub fn cov_update_builtins_basic_block_coverage(cov_map: &Vec<bool>) {
    unsafe {
        let shmem_data = SHMEM.map(|ptr| &*ptr).unwrap();
        if cov_map.len() as u32 != BUILTINS_EDGE_COUNT {
            eprintln!("[COV] Error: Size of builtins cov map changed.");
            process::exit(-1);
        }

        for (i, &covered) in cov_map.iter().enumerate() {
            if covered {
                let i = i as u32;
                let byte_index = ((i + BUILTINS_START) >> 3) as usize;
                let bit_index = ((i + BUILTINS_START) & 7) as u8;

                shmem_data.edges[byte_index] |= (1 << bit_index);
            }
        }
    }
}