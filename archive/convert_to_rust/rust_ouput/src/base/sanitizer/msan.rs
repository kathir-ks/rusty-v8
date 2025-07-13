// Converted from V8 C++ source files:
// Header: msan.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod msan {
    use std::{mem, ptr};
    use std::marker::PhantomData;

    #[cfg(not(v8_use_memory_sanitizer))]
    macro_rules! static_assert_pointer_or_address {
        ($start:expr) => {
            let _ = $start; // Prevent unused variable warning
        };
    }

    #[cfg(not(v8_use_memory_sanitizer))]
    macro_rules! static_assert_convertible_to_size_t {
        ($size:expr) => {
            let _ = $size as usize; // Attempt conversion and prevent warning
        };
    }

    #[cfg(not(v8_use_memory_sanitizer))]
    macro_rules! use_macro {
        ($start:expr, $size:expr) => {
            let _ = ($start, $size); // Prevent unused variable warnings
        };
    }

    #[cfg(v8_use_memory_sanitizer)]
    extern "C" {
        fn __msan_allocated_memory(start: *const std::ffi::c_void, size: usize);
        fn __msan_unpoison(start: *const std::ffi::c_void, size: usize);
    }

    #[macro_export]
    macro_rules! msan_allocated_uninitialized_memory {
        ($start:expr, $size:expr) => {
            #[cfg(v8_use_memory_sanitizer)]
            unsafe {
                __msan_allocated_memory($start as *const std::ffi::c_void, $size as usize);
            }
            #[cfg(not(v8_use_memory_sanitizer))]
            {
                static_assert_pointer_or_address!($start);
                static_assert_convertible_to_size_t!($size);
                use_macro!($start, $size);
            }
        };
    }

    #[macro_export]
    macro_rules! msan_memory_is_initialized {
        ($start:expr, $size:expr) => {
            #[cfg(v8_use_memory_sanitizer)]
            unsafe {
                __msan_unpoison($start as *const std::ffi::c_void, $size as usize);
            }
            #[cfg(not(v8_use_memory_sanitizer))]
            {
                $crate::msan::msan_allocated_uninitialized_memory!($start, $size);
            }
        };
    }

    #[macro_export]
    macro_rules! disable_msan {
        () => {
            #[cfg(v8_use_memory_sanitizer)]
            #[inline(never)]
            fn disable_msan<F, R>(f: F) -> R
            where
                F: FnOnce() -> R,
            {
                f()
            }

            #[cfg(not(v8_use_memory_sanitizer))]
            #[inline(always)]
            fn disable_msan<F, R>(f: F) -> R
            where
                F: FnOnce() -> R,
            {
                f()
            }
        };
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_msan_macros() {
            let mut data: Vec<u8> = vec![0; 10];
            let start = data.as_mut_ptr();
            let size = data.len();

            unsafe {
                msan_allocated_uninitialized_memory!(start, size);
                msan_memory_is_initialized!(start, size);
            }

            disable_msan!();
            let result = disable_msan(|| {
                1 + 1
            });
            assert_eq!(result, 2);
        }
    }
}
