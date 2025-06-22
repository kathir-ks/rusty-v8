// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// AddressSanitizer support.

#[cfg(feature = "v8_use_address_sanitizer")]
mod asan_impl {
    extern "C" {
        fn __asan_region_is_poisoned(addr: *const std::ffi::c_void, size: usize) -> i32;
        fn __asan_address_is_poisoned(addr: *const i8) -> i32;
        fn ASAN_POISON_MEMORY_REGION(addr: *const std::ffi::c_void, size: usize);
        fn ASAN_UNPOISON_MEMORY_REGION(addr: *const std::ffi::c_void, size: usize);
    }

    macro_rules! disable_asan {
        () => {
            #[cfg_attr(any(), no_sanitize(address))]
            unsafe fn _disable_asan() {}
        };
    }
    pub(crate) use disable_asan;

    macro_rules! asan_check_whole_memory_region_is_poisoned {
        ($start:expr, $size:expr) => {
            {
                let start_ptr = $start as *const std::ffi::c_void;
                let size_val = $size as usize;

                for i in 0..size_val {
                    let addr = (start_ptr as *const i8).wrapping_add(i);
                    assert_ne!(unsafe { __asan_address_is_poisoned(addr) }, 0);
                }
            }
        };
    }
    pub(crate) use asan_check_whole_memory_region_is_poisoned;

    /// RAII guard to unpoison a memory region.
    pub struct AsanUnpoisonScope {
        addr_: *const std::ffi::c_void,
        size_: usize,
        was_poisoned_: bool,
    }

    impl AsanUnpoisonScope {
        pub fn new(addr: *const std::ffi::c_void, size: usize) -> Self {
            let was_poisoned_ = unsafe { __asan_region_is_poisoned(addr as *mut std::ffi::c_void, size) != 0 };
            if was_poisoned_ {
                unsafe { ASAN_UNPOISON_MEMORY_REGION(addr, size) };
            }

            Self {
                addr_: addr,
                size_: size,
                was_poisoned_: was_poisoned_,
            }
        }
    }

    impl Drop for AsanUnpoisonScope {
        fn drop(&mut self) {
            if self.was_poisoned_ {
                unsafe { ASAN_POISON_MEMORY_REGION(self.addr_, self.size_) };
            }
        }
    }
}

#[cfg(not(feature = "v8_use_address_sanitizer"))]
mod asan_impl {
    macro_rules! disable_asan {
        () => {
            #[cfg_attr(any(), allow(dead_code))]
            unsafe fn _disable_asan() {}
        };
    }
    pub(crate) use disable_asan;

    macro_rules! asan_poison_memory_region {
        ($start:expr, $size:expr) => {
            {
                let _start = $start;
                let _size = $size;
                let _ = (&_start, &_size);
            }
        };
    }

    macro_rules! asan_unpoison_memory_region {
        ($start:expr, $size:expr) => {
           asan_poison_memory_region!($start, $size)
        };
    }

    pub(crate) use asan_poison_memory_region;
    pub(crate) use asan_unpoison_memory_region;


    macro_rules! asan_check_whole_memory_region_is_poisoned {
        ($start:expr, $size:expr) => {
            asan_poison_memory_region!($start, $size)
        };
    }
    pub(crate) use asan_check_whole_memory_region_is_poisoned;

    /// RAII guard to unpoison a memory region. (No-op when ASan is disabled).
    pub struct AsanUnpoisonScope {}

    impl AsanUnpoisonScope {
        pub fn new(_addr: *const std::ffi::c_void, _size: usize) -> Self {
           Self {}
        }
    }
}

pub use asan_impl::*;

#[cfg(feature = "v8_use_hwaddress_sanitizer")]
macro_rules! disable_hwasan {
    () => {
        #[cfg_attr(any(), no_sanitize("hwaddress"))]
        unsafe fn _disable_hwasan() {}
    };
}

#[cfg(not(feature = "v8_use_hwaddress_sanitizer"))]
macro_rules! disable_hwasan {
    () => {
        #[cfg_attr(any(), allow(dead_code))]
        unsafe fn _disable_hwasan() {}
    };
}

pub(crate) use disable_hwasan;