use libc::{c_char, c_double, c_int, c_long, c_void, MAP_ANONYMOUS, MAP_FAILED, MAP_FIXED, MAP_PRIVATE, PROT_NONE, O_RDONLY, STDERR_FILENO};
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{Read, Write};
use std::mem;
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::slice;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

//use std::os::raw::{c_char, c_double, c_int, c_long, c_void}; //Not available on my platform

//mod platform;
//mod platform_posix;

const MS_PER_SECOND: f64 = 1000.0;

//pub use platform::OS;
//pub use platform_posix::PosixTimezoneCache;

macro_rules! CHECK {
    ($x:expr) => {
        if !$x {
            panic!("Check failed: {}", stringify!($x));
        }
    };
}

macro_rules! DCHECK_EQ {
    ($x:expr, $y:expr) => {
        assert_eq!($x, $y, "DCHECK_EQ failed: {} != {}", stringify!($x), stringify!($y));
    };
}

macro_rules! DCHECK_NE {
    ($x:expr, $y:expr) => {
        assert_ne!($x, $y, "DCHECK_NE failed: {} == {}", stringify!($x), stringify!($y));
    };
}

macro_rules! DCHECK_NOT_NULL {
    ($x:expr) => {
        assert!(!$x.is_null(), "DCHECK_NOT_NULL failed");
    };
}

// Define structs equivalent to C structs

#[repr(C)]
#[derive(Debug)]
struct timeval {
    tv_sec: libc::time_t,
    tv_usec: libc::suseconds_t,
}

#[repr(C)]
#[derive(Debug)]
struct timezone {
    tz_minuteswest: c_int,
    tz_dsttime: c_int,
}

#[repr(C)]
#[derive(Debug)]
struct tm {
    tm_sec: c_int,
    tm_min: c_int,
    tm_hour: c_int,
    tm_mday: c_int,
    tm_mon: c_int,
    tm_year: c_int,
    tm_wday: c_int,
    tm_yday: c_int,
    tm_isdst: c_int,
    tm_gmtoff: c_long,
    tm_zone: *const c_char,
}

#[repr(C)]
#[derive(Debug)]
struct __pthrdsinfo {
    __pi_flags: c_int,
    __pi_errno: c_int,
    __pi_tracepri: c_int,
    __pi_state: c_int,
    __pi_name: [c_char; 16],
    __pi_stackaddr: *mut c_void,
    __pi_stacksize: usize,
    __pi_stackend: *mut c_void,
}

//Implement base::PosixTimezoneCache class
pub trait TimezoneCache {
  fn local_timezone(&self, time: f64) -> &'static str;
  fn local_time_offset(&self, time_ms: f64, is_utc: bool) -> f64;
}

pub struct PosixTimezoneCache {}
impl PosixTimezoneCache{
  pub fn new() -> Self {
    Self{}
  }
}

//Implement base::Stack
pub struct Stack {
  stack_start: *mut c_void,
}
impl Stack {
    pub fn new(stack_start: *mut c_void) -> Self {
        Self {
            stack_start,
        }
    }

    pub fn obtain_current_thread_stack_start() -> *mut c_void {
        let tid = unsafe { libc::pthread_self() };
        let mut buf: __pthrdsinfo = unsafe { mem::zeroed() };
        let mut regbuf: [c_char; 1] = [0];
        let mut regbufsize: libc::size_t = regbuf.len();

        let rc = unsafe {
            libc::pthread_getthrds_np(
                &tid,
                libc::PTHRDSINFO_QUERY_ALL,
                &mut buf as *mut _ as *mut c_void,
                mem::size_of::<__pthrdsinfo>(),
                regbuf.as_mut_ptr() as *mut c_void,
                &mut regbufsize,
            )
        };
        CHECK!(rc == 0);

        if buf.__pi_stackend.is_null() || buf.__pi_stackaddr.is_null() {
            return ptr::null_mut();
        }

        buf.__pi_stackend
    }
}

pub struct AIXTimezoneCache {}

impl TimezoneCache for AIXTimezoneCache {
    fn local_timezone(&self, time_ms: f64) -> &'static str {
        if time_ms.is_nan() {
            return "";
        }

        let tv = time_ms.floor() / MS_PER_SECOND;
        let tv = tv as libc::time_t;
        let mut tm: tm = unsafe { mem::zeroed() };
        let t = unsafe { libc::localtime_r(&tv, &mut tm) };

        if t.is_null() {
            return "";
        }

        unsafe {
            let tzname_ptr = libc::tzname.as_ptr();
            let tzname_slice = slice::from_raw_parts(tzname_ptr, 2);
            let tzname0 = tzname_slice[0];
            CStr::from_ptr(tzname0).to_str().unwrap()
        }
    }

    fn local_time_offset(&self, time_ms: f64, is_utc: bool) -> f64 {
        let utc = unsafe { libc::time(ptr::null_mut()) };
        DCHECK_NE!(utc, -1);

        let mut tm: tm = unsafe { mem::zeroed() };
        let loc = unsafe { libc::localtime_r(&utc, &mut tm) };
        DCHECK_NOT_NULL!(loc);

        get_gmt_offset(&tm) as f64 * MS_PER_SECOND -
            if tm.tm_isdst > 0 { 3600.0 * MS_PER_SECOND } else { 0.0 }
    }
}

impl AIXTimezoneCache {
    pub fn new() -> Self {
        AIXTimezoneCache {}
    }
}

fn get_gmt_offset(localtm: &tm) -> i64 {
    let mut tv: timeval = unsafe { mem::zeroed() };
    let mut tz: timezone = unsafe { mem::zeroed() };
    let ret_code = unsafe { libc::gettimeofday(&mut tv, &mut tz) };
    DCHECK_NE!(ret_code, -1);
    if ret_code == -1 {
        return 0;
    }
    (-tz.tz_minuteswest as i64 * 60) + (if localtm.tm_isdst > 0 { 3600 } else { 0 })
}

pub struct OS {}

impl OS {
    pub fn create_timezone_cache() -> Box<dyn TimezoneCache> {
        Box::new(AIXTimezoneCache::new())
    }

    fn string_to_long(buffer: &[u8]) -> Result<u32, std::num::ParseIntError> {
        let s = unsafe { std::str::from_utf8_unchecked(buffer) };
        u32::from_str_radix(s, 16)
    }

    pub fn get_shared_library_addresses() -> Vec<SharedLibraryAddress> {
        let mut result = Vec::new();
        const MAP_LENGTH: usize = 1024;

        let fd = unsafe { libc::open("/proc/self/maps\0".as_ptr() as *const i8, O_RDONLY) };
        if fd < 0 {
            return result;
        }

        let mut addr_buffer = [0u8; 11];
        addr_buffer[0] = b'0';
        addr_buffer[1] = b'x';
        addr_buffer[10] = 0;

        loop {
            let rc = unsafe { libc::read(fd, addr_buffer[2..].as_mut_ptr() as *mut c_void, 8) };
            if rc < 8 {
                break;
            }

            let start = match Self::string_to_long(&addr_buffer[..10]) {
                Ok(val) => val,
                Err(_) => break,
            };

            let rc = unsafe { libc::read(fd, addr_buffer[2..].as_mut_ptr() as *mut c_void, 1) };
            if rc < 1 {
                break;
            }

            if addr_buffer[2] != b'-' {
                break;
            }

            let rc = unsafe { libc::read(fd, addr_buffer[2..].as_mut_ptr() as *mut c_void, 8) };
            if rc < 8 {
                break;
            }

            let end = match Self::string_to_long(&addr_buffer[..10]) {
                Ok(val) => val,
                Err(_) => break,
            };

            let mut buffer = [0u8; MAP_LENGTH];
            let mut bytes_read = -1;

            loop {
                bytes_read += 1;
                if bytes_read >= (MAP_LENGTH - 1) as i32 {
                    break;
                }

                let rc = unsafe { libc::read(fd, buffer[bytes_read as usize..].as_mut_ptr() as *mut c_void, 1) };
                if rc < 1 {
                    break;
                }

                if buffer[bytes_read as usize] == b'\n' {
                    break;
                }
            }
            buffer[bytes_read as usize] = 0;

            if buffer[3] != b'x' {
                continue;
            }

            let start_of_path = buffer.iter().position(|&x| x == b'/');

            match start_of_path {
                Some(index) => {
                    let path = &buffer[index..bytes_read as usize];
                    let path_str = String::from_utf8_lossy(path).to_string();

                    result.push(SharedLibraryAddress {
                        name: path_str,
                        start: start as usize,
                        end: end as usize,
                    });
                }
                None => continue,
            }
        }

        unsafe { libc::close(fd) };
        result
    }

    pub fn signal_code_moving_gc() {}

    pub fn adjust_scheduling_params() {}

    pub fn get_first_free_memory_range_within(
        boundary_start: usize,
        boundary_end: usize,
        minimum_size: usize,
        alignment: usize,
    ) -> Option<MemoryRange> {
        None
    }

    pub fn decommit_pages(address: *mut c_void, size: usize) -> bool {
        DCHECK_EQ!(address as usize % Self::commit_page_size(), 0);
        DCHECK_EQ!(size % Self::commit_page_size(), 0);

        let ptr = unsafe {
            libc::mmap(
                address,
                size,
                PROT_NONE,
                MAP_FIXED | MAP_ANONYMOUS | MAP_PRIVATE,
                -1,
                0,
            )
        };

        if ptr != address {
            DCHECK_EQ!(ptr, MAP_FAILED);

            if unsafe { libc::munmap(address, size) } != 0 {
                return false;
            }

            let ptr = unsafe {
                libc::mmap(
                    address,
                    size,
                    PROT_NONE,
                    MAP_FIXED | MAP_ANONYMOUS | MAP_PRIVATE,
                    -1,
                    0,
                )
            };

            CHECK_EQ!(ptr, address);
        }

        true
    }

    const fn commit_page_size() -> usize {
      4096
    }
}

#[derive(Debug)]
pub struct SharedLibraryAddress {
    pub name: String,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub struct MemoryRange {
    pub start: usize,
    pub size: usize,
}