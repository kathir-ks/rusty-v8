// Converted from V8 C++ source files:
// Header: memory.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod bits {
        pub fn IsPowerOfTwo(x: usize) -> bool {
            (x != 0) && ((x & (x - 1)) == 0)
        }
    }

    pub mod logging {
        #[macro_export]
        macro_rules! CHECK_NE {
            ($a:expr, $b:expr) => {
                if $a == $b {
                    panic!("CHECK_NE failed: {} == {}", $a, $b);
                }
            };
        }

        #[macro_export]
        macro_rules! DCHECK_LE {
            ($a:expr, $b:expr) => {
                if $a > $b {
                    panic!("DCHECK_LE failed: {} > {}", $a, $b);
                }
            };
        }
        #[macro_export]
        macro_rules! DCHECK {
            ($a:expr) => {
                if !$a {
                    panic!("DCHECK failed: {}", stringify!($a));
                }
            };
        }
    }
    use std::alloc::{alloc, dealloc, Layout};
    use std::ffi::c_void;
    use std::mem;

    #[cfg(target_os = "windows")]
    extern "system" {
        fn _aligned_malloc(size: usize, alignment: usize) -> *mut c_void;
        fn _aligned_free(ptr: *mut c_void);
        fn _msize(ptr: *mut c_void) -> usize;
    }

    #[cfg(target_os = "macos")]
    extern "C" {
        fn malloc_size(ptr: *mut c_void) -> usize;
    }

    #[cfg(not(target_os = "windows"))]
    extern "C" {
        fn posix_memalign(memptr: *mut *mut c_void, alignment: usize, size: usize) -> i32;
    }

    #[cfg(target_os = "android")]
    extern "C" {
        fn memalign(alignment: usize, size: usize) -> *mut c_void;
    }

    pub fn Malloc(size: usize) -> *mut c_void {
        unsafe { libc::malloc(size) }
    }

    pub fn Realloc(memory: *mut c_void, size: usize) -> *mut c_void {
        crate::base::logging::CHECK_NE!(0, size);
        unsafe { libc::realloc(memory, size) }
    }

    pub fn Free(memory: *mut c_void) {
        unsafe { libc::free(memory) }
    }

    pub fn Calloc(count: usize, size: usize) -> *mut c_void {
        unsafe { libc::calloc(count, size) }
    }

    pub fn AlignedAlloc(size: usize, alignment: usize) -> *mut c_void {
        crate::base::logging::DCHECK_LE!(std::mem::align_of::<*mut c_void>(), alignment);
        crate::base::logging::DCHECK!(crate::base::bits::IsPowerOfTwo(alignment));

        #[cfg(target_os = "windows")]
        unsafe {
            return _aligned_malloc(size, alignment);
        }

        #[cfg(target_os = "android")]
        unsafe {
            return memalign(alignment, size);
        }

        #[cfg(not(any(target_os = "windows", target_os = "android")))]
        {
            let mut ptr: *mut c_void = std::ptr::null_mut();
            let result = unsafe { posix_memalign(&mut ptr, alignment, size) };
            if result != 0 {
                ptr = std::ptr::null_mut();
            }
            return ptr;
        }
    }

    pub fn AlignedFree(ptr: *mut c_void) {
        #[cfg(target_os = "windows")]
        unsafe {
            _aligned_free(ptr);
        }
        #[cfg(not(target_os = "windows"))]
        unsafe {
            Free(ptr);
        }
    }

    pub fn MallocUsableSize(ptr: *mut c_void) -> usize {
        if ptr.is_null() {
            return 0;
        }
        #[cfg(target_os = "windows")]
        unsafe {
            return _msize(ptr);
        }

        #[cfg(target_os = "macos")]
        unsafe {
            return malloc_size(ptr);
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        unsafe {
            return libc::malloc_usable_size(ptr);
        }
    }

    pub struct AllocationResult<T> {
        pub ptr: *mut T,
        pub count: usize,
    }

    impl<T> AllocationResult<T> {
        pub fn default() -> Self {
            AllocationResult {
                ptr: std::ptr::null_mut(),
                count: 0,
            }
        }
    }

    pub fn AllocateAtLeast<T>(n: usize) -> AllocationResult<T> {
        let min_wanted_size = n * std::mem::size_of::<T>();
        let memory = Malloc(min_wanted_size) as *mut T;

        let usable_size = MallocUsableSize(memory as *mut c_void);
        let memory = if usable_size != min_wanted_size && !memory.is_null() {
            Realloc(memory as *mut c_void, usable_size) as *mut T
        } else {
            memory
        };
        AllocationResult {
            ptr: memory,
            count: usable_size,
        }
    }
}
