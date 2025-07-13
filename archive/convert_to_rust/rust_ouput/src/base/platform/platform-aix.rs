// Converted from V8 C++ source files:
// Header: N/A
// Implementation: platform-aix.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

use std::cell::RefCell;
use std::cmp::min;
use std::ffi::c_void;
use std::io::{Error, Read};
use std::mem;
use std::os::raw::c_char;
use std::ptr;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

extern crate libc;
use libc::{
    c_int, c_long, c_uchar, close, fcntl, getpagesize, gettimeofday, localtime_r, mmap, munmap,
    open, pthread_getthrds_np, pthread_self, read, strtol, time, timezone, tm, O_RDONLY,
    PROT_NONE, MAP_ANONYMOUS, MAP_FIXED, MAP_PRIVATE, MAP_FAILED,
    __pthrdsinfo, PTHRDSINFO_QUERY_ALL,
};

#[allow(non_camel_case_types)]
pub struct Address {}

#[allow(non_camel_case_types)]
pub struct Stack {}

#[allow(non_camel_case_types)]
pub struct Isolate {}

#[allow(non_camel_case_types)]
pub struct String {}

#[allow(non_camel_case_types)]
pub struct Object {}

#[allow(non_camel_case_types)]
pub struct Map {}

#[allow(non_camel_case_types)]
pub struct JSFunction {}

#[allow(non_camel_case_types)]
pub struct Tagged<T> {
    _phantom: std::marker::PhantomData<T>,
}

#[allow(non_camel_case_types)]
pub struct DirectHandle<T> {
    _phantom: std::marker::PhantomData<T>,
}

#[allow(non_camel_case_types)]
pub struct IndirectHandle<T> {
    _phantom: std::marker::PhantomData<T>,
}

#[allow(non_camel_case_types)]
pub struct ValueType {}

#[allow(non_camel_case_types)]
pub struct Operand {}

#[allow(non_camel_case_types)]
pub struct Register {}

#[allow(non_camel_case_types)]
pub struct Condition {}

#[allow(non_camel_case_types)]
pub struct Bytecode {}

#[allow(non_camel_case_types)]
pub struct JSArrayBuffer {}

#[allow(non_camel_case_types)]
pub struct MachineType {}

#[allow(non_camel_case_types)]
pub struct IrregexpImplementation {}

#[allow(non_camel_case_types)]
pub struct OpIndex {}

#[derive(Debug)]
pub enum BaseError {
    TimeError,
    IOError(std::io::Error),
    MemoryError,
}

impl From<std::io::Error> for BaseError {
    fn from(err: std::io::Error) -> Self {
        BaseError::IOError(err)
    }
}

pub mod base {
    use super::*;
    use std::ffi::{CString, CStr};

    const MS_PER_SECOND: f64 = 1000.0;

    fn get_gmt_offset(localtm: &libc::tm) -> i64 {
        let mut tv = libc::timeval {
            tv_sec: 0,
            tv_usec: 0,
        };
        let mut tz = libc::timezone {
            tz_minuteswest: 0,
            tz_dsttime: 0,
        };
        let ret_code = unsafe { libc::gettimeofday(&mut tv, &mut tz) };
        assert_ne!(ret_code, -1);
        if ret_code == -1 {
            return 0;
        }
        (-i64::from(tz.tz_minuteswest) * 60) + (if localtm.tm_isdst > 0 { 3600 } else { 0 })
    }

    trait TimezoneCacheTrait {
        fn local_timezone(&mut self, time: f64) -> &str;
        fn local_time_offset(&mut self, time_ms: f64, is_utc: bool) -> f64;
    }

    struct AIXTimezoneCache {
        tzname: [*mut libc::c_char; 2],
    }

    impl AIXTimezoneCache {
        fn new() -> Self {
            AIXTimezoneCache {
                tzname: [ptr::null_mut(); 2],
            }
        }
    }

    impl TimezoneCacheTrait for AIXTimezoneCache {
        fn local_timezone(&mut self, time_ms: f64) -> &str {
            if time_ms.is_nan() {
                return "";
            }
            let tv = (time_ms / MS_PER_SECOND).floor() as time_t;
            let mut tm: tm = unsafe { mem::zeroed() };
            let t = unsafe { localtime_r(&tv, &mut tm) };
            if t.is_null() {
                return "";
            }

            unsafe {
                let c_str = CStr::from_ptr(*self.tzname.get_unchecked(0));
                c_str.to_str().unwrap()
            }
        }

        fn local_time_offset(&mut self, time_ms: f64, is_utc: bool) -> f64 {
            let utc = unsafe { time(ptr::null_mut()) };
            assert_ne!(utc, -1);
            let mut tm: tm = unsafe { mem::zeroed() };
            let loc = unsafe { localtime_r(&utc, &mut tm) };
            assert!(!loc.is_null());
            f64::from(get_gmt_offset(&tm)) * MS_PER_SECOND -
                (if tm.tm_isdst > 0 { 3600 * MS_PER_SECOND } else { 0.0 })
        }
    }

    pub struct OS {}

    impl OS {
        pub fn create_timezone_cache() -> Box<dyn TimezoneCacheTrait> {
            Box::new(AIXTimezoneCache::new())
        }

        fn string_to_long(buffer: *mut c_char) -> Result<u32, BaseError> {
            let buffer_cstr = unsafe { CStr::from_ptr(buffer) };
            let buffer_str = buffer_cstr.to_str().map_err(|_| BaseError::IOError(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8")))?;
            u32::from_str_radix(buffer_str, 16).map_err(|_| BaseError::IOError(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid hex string")))
        }

        pub fn get_shared_library_addresses() -> Result<Vec<SharedLibraryAddress>, BaseError> {
            let mut result = Vec::new();
            const MAP_LENGTH: usize = 1024;
            let fd = unsafe { open(CString::new("/proc/self/maps").unwrap().as_ptr(), O_RDONLY) };
            if fd < 0 {
                return Ok(result);
            }

            loop {
                let mut addr_buffer: [c_char; 11] = ['\0' as c_char; 11];
                addr_buffer[0] = '0' as c_char;
                addr_buffer[1] = 'x' as c_char;

                let rc = unsafe { read(fd, addr_buffer[2..].as_mut_ptr() as *mut c_void, 8) };
                if rc < 8 {
                    break;
                }

                let start = match OS::string_to_long(addr_buffer.as_mut_ptr()) {
                    Ok(value) => value,
                    Err(_) => break,
                };

                let mut separator: [c_char; 1] = ['\0' as c_char; 1];
                let rc = unsafe { read(fd, separator.as_mut_ptr() as *mut c_void, 1) };
                if rc < 1 {
                    break;
                }
                if separator[0] != '-' as c_char {
                    break;
                }

                let rc = unsafe { read(fd, addr_buffer[2..].as_mut_ptr() as *mut c_void, 8) };
                if rc < 8 {
                    break;
                }
                let end = match OS::string_to_long(addr_buffer.as_mut_ptr()) {
                    Ok(value) => value,
                    Err(_) => break,
                };

                let mut buffer: [c_char; MAP_LENGTH] = ['\0' as c_char; MAP_LENGTH];
                let mut bytes_read: i32 = -1;

                loop {
                    bytes_read += 1;
                    if bytes_read >= (MAP_LENGTH - 1) as i32 {
                        break;
                    }

                    let rc = unsafe { read(fd, buffer[bytes_read as usize..].as_mut_ptr() as *mut c_void, 1) };
                    if rc < 1 {
                        break;
                    }

                    if buffer[bytes_read as usize] == '\n' as c_char {
                        break;
                    }
                }
                buffer[bytes_read as usize] = '\0' as c_char;

                if buffer[3] != 'x' as c_char {
                    continue;
                }

                let start_of_path = match buffer.iter().position(|&x| x == '/' as c_char) {
                    Some(index) => index,
                    None => continue,
                };

                let path = unsafe { CStr::from_ptr(buffer[start_of_path..].as_ptr()) }.to_string_lossy().into_owned();

                result.push(SharedLibraryAddress {
                    name: path,
                    start: start as usize,
                    end: end as usize,
                });
            }

            unsafe {
                close(fd);
            }

            Ok(result)
        }

        pub fn signal_code_moving_gc() {}
        pub fn adjust_scheduling_params() {}

        pub fn get_first_free_memory_range_within(
            _boundary_start: usize,
            _boundary_end: usize,
            _minimum_size: usize,
            _alignment: usize,
        ) -> Option<MemoryRange> {
            None
        }

        pub fn decommit_pages(address: *mut c_void, size: usize) -> bool {
            let page_size = OS::commit_page_size();
            if (address as usize) % page_size != 0 || size % page_size != 0 {
                return false;
            }

            unsafe {
                let ptr = mmap(
                    address,
                    size,
                    PROT_NONE,
                    MAP_FIXED | MAP_ANONYMOUS | MAP_PRIVATE,
                    -1,
                    0,
                );
                if ptr != address {
                    if ptr == MAP_FAILED {
                        if munmap(address, size) != 0 {
                            return false;
                        }
                        let ptr = mmap(
                            address,
                            size,
                            PROT_NONE,
                            MAP_FIXED | MAP_ANONYMOUS | MAP_PRIVATE,
                            -1,
                            0,
                        );
                        if ptr != address {
                            panic!("Failed to decommit pages after unmapping");
                        }
                    } else {
                        panic!("Unexpected mmap return value");
                    }
                }
            }
            true
        }

        pub fn commit_page_size() -> usize {
          unsafe { getpagesize() as usize }
        }
    }

    pub struct SharedLibraryAddress {
        pub name: String,
        pub start: usize,
        pub end: usize,
    }

    pub struct MemoryRange {
        pub start: usize,
        pub size: usize,
    }

    impl Stack {
      pub fn obtain_current_thread_stack_start() -> *mut c_void {
          let tid = unsafe { pthread_self() };
          let mut buf: __pthrdsinfo = unsafe { mem::zeroed() };
          let mut regbuf: [c_char; 1] = [0 as c_char; 1];
          let regbufsize = mem::size_of_val(&regbuf);

          let rc = unsafe {
              pthread_getthrds_np(
                  &tid,
                  PTHRDSINFO_QUERY_ALL,
                  &mut buf as *mut __pthrdsinfo as *mut c_void,
                  mem::size_of::<__pthrdsinfo>(),
                  regbuf.as_mut_ptr() as *mut c_char,
                  &regbufsize as *const usize as *mut i32,
              )
          };
          assert_eq!(rc, 0);

          if buf.__pi_stackend.is_null() || buf.__pi_stackaddr.is_null() {
              return ptr::null_mut();
          }

          buf.__pi_stackend as *mut c_void
      }
    }
}
