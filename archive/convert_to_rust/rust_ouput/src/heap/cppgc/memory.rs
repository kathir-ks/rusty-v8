// Converted from V8 C++ source files:
// Header: memory.h
// Implementation: memory.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod cppgc {
    pub mod internal {
        use std::mem;
        use std::os::raw::c_char;

        pub const kZappedValue: u8 = 0xdc;

        #[inline]
        pub fn zap_memory(address: *mut std::ffi::c_void, size: usize) {
            unsafe {
                std::ptr::write_bytes(address as *mut u8, kZappedValue, size);
            }
        }

        #[inline]
        pub fn check_memory_is_zapped(address: *const std::ffi::c_void, size: usize) {
            let ptr = address as *const u8;
            for i in 0..size {
                unsafe {
                    assert_eq!(kZappedValue, *ptr.add(i));
                }
            }
        }

        #[inline]
        pub fn check_memory_is_zero(address: *const std::ffi::c_void, size: usize) {
            let ptr = address as *const u8;
            for i in 0..size {
                unsafe {
                    assert_eq!(0, *ptr.add(i));
                }
            }
        }

        pub fn no_sanitize_memset(address: *mut std::ffi::c_void, c: c_char, bytes: usize) {
            let base = address as *mut u8;
            for i in 0..bytes {
                unsafe {
                    *base.add(i) = c as u8;
                }
            }
        }

        #[cfg(any(
            feature = "v8_use_memory_sanitizer",
            feature = "v8_use_address_sanitizer",
            debug_assertions
        ))]
        pub mod sanitizer {
            use super::*;

            extern "C" {
                fn __msan_check_mem_is_initialized(ptr: *const std::ffi::c_void, size: usize);
                fn __msan_poison(ptr: *mut std::ffi::c_void, size: usize);
                fn __msan_unpoison(ptr: *mut std::ffi::c_void, size: usize);

                fn __asan_poison_memory_region(addr: *mut std::ffi::c_void, size: usize);
                fn __asan_unpoison_memory_region(addr: *mut std::ffi::c_void, size: usize);
                fn __asan_check_region_is_poisoned(addr: *const std::ffi::c_void, size: usize);
            }

            pub fn set_memory_accessible(address: *mut std::ffi::c_void, size: usize) {
                #[cfg(feature = "v8_use_memory_sanitizer")]
                unsafe {
                    __msan_check_mem_is_initialized(address, size);
                }

                #[cfg(feature = "v8_use_address_sanitizer")]
                unsafe {
                    __asan_unpoison_memory_region(address, size);
                }

                #[cfg(all(
                    not(feature = "v8_use_memory_sanitizer"),
                    not(feature = "v8_use_address_sanitizer"),
                    debug_assertions
                ))]
                unsafe {
                    std::ptr::write_bytes(address as *mut u8, 0, size);
                }
            }

            pub fn set_memory_inaccessible(address: *mut std::ffi::c_void, size: usize) {
                #[cfg(feature = "v8_use_memory_sanitizer")]
                {
                    unsafe {
                        std::ptr::write_bytes(address as *mut u8, 0, size);
                        __msan_poison(address, size);
                    }
                }

                #[cfg(feature = "v8_use_address_sanitizer")]
                {
                    super::no_sanitize_memset(address, 0, size);
                    unsafe {
                        __asan_poison_memory_region(address, size);
                    }
                }

                #[cfg(all(
                    not(feature = "v8_use_memory_sanitizer"),
                    not(feature = "v8_use_address_sanitizer"),
                    debug_assertions
                ))]
                {
                    super::zap_memory(address, size);
                }
            }

            pub fn check_memory_is_inaccessible(address: *const std::ffi::c_void, size: usize) {
                #[cfg(feature = "v8_use_memory_sanitizer")]
                {
                    assert!(check_memory_is_inaccessible_is_noop());
                }

                #[cfg(feature = "v8_use_address_sanitizer")]
                {
                    assert!(!check_memory_is_inaccessible_is_noop());

                    #[cfg(target_pointer_width = "64")]
                    unsafe {
                        __asan_check_region_is_poisoned(address, size);
                    }
                    unsafe {
                        __asan_unpoison_memory_region(address as *mut std::ffi::c_void, size);
                    }
                    super::check_memory_is_zero(address, size);
                    unsafe {
                        __asan_poison_memory_region(address as *mut std::ffi::c_void, size);
                    }
                }

                #[cfg(all(
                    not(feature = "v8_use_memory_sanitizer"),
                    not(feature = "v8_use_address_sanitizer"),
                    debug_assertions
                ))]
                {
                    assert!(!check_memory_is_inaccessible_is_noop());
                    super::check_memory_is_zapped(address, size);
                }
            }

            pub const fn check_memory_is_inaccessible_is_noop() -> bool {
                #[cfg(feature = "v8_use_memory_sanitizer")]
                {
                    true
                }

                #[cfg(feature = "v8_use_address_sanitizer")]
                {
                    false
                }

                #[cfg(all(
                    not(feature = "v8_use_memory_sanitizer"),
                    not(feature = "v8_use_address_sanitizer"),
                    debug_assertions
                ))]
                {
                    false
                }
            }
        }

        #[cfg(not(any(
            feature = "v8_use_memory_sanitizer",
            feature = "v8_use_address_sanitizer",
            debug_assertions
        )))]
        pub mod no_sanitizer {
            use super::*;

            #[inline]
            pub fn set_memory_accessible(address: *mut std::ffi::c_void, size: usize) {}

            #[inline]
            pub fn check_memory_is_inaccessible(address: *const std::ffi::c_void, size: usize) {}

            #[inline]
            pub const fn check_memory_is_inaccessible_is_noop() -> bool {
                true
            }

            #[inline]
            pub fn set_memory_inaccessible(address: *mut std::ffi::c_void, size: usize) {
                unsafe {
                    std::ptr::write_bytes(address as *mut u8, 0, size);
                }
            }
        }

        #[cfg(any(
            feature = "v8_use_memory_sanitizer",
            feature = "v8_use_address_sanitizer",
            debug_assertions
        ))]
        pub use sanitizer::*;

        #[cfg(not(any(
            feature = "v8_use_memory_sanitizer",
            feature = "v8_use_address_sanitizer",
            debug_assertions
        )))]
        pub use no_sanitizer::*;
    }
}
