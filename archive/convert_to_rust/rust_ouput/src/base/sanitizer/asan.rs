// Converted from V8 C++ source files:
// Header: asan.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod asan {
    // AddressSanitizer support.

    use std::mem::MaybeUninit;

    #[cfg(not(v8_use_address_sanitizer))]
    macro_rules! static_assert {
        ($condition:expr, $message:expr) => {
            const _: () = assert!($condition, $message);
        };
    }

    #[cfg(v8_use_address_sanitizer)]
    extern "C" {
        fn __asan_region_is_poisoned(addr: *const std::ffi::c_void, size: usize) -> i32;
        fn __asan_address_is_poisoned(addr: *const i8) -> i32;
        fn __asan_poison_memory_region(addr: *mut std::ffi::c_void, size: usize);
        fn __asan_unpoison_memory_region(addr: *mut std::ffi::c_void, size: usize);
    }

    #[cfg(v8_use_address_sanitizer)]
    macro_rules! ASAN_POISON_MEMORY_REGION {
        ($start:expr, $size:expr) => {
            unsafe {
                __asan_poison_memory_region($start as *mut std::ffi::c_void, $size as usize);
            }
        };
    }

    #[cfg(not(v8_use_address_sanitizer))]
    macro_rules! ASAN_POISON_MEMORY_REGION {
        ($start:expr, $size:expr) => {
            {
                let start = $start;
                let size = $size;
                static_assert!(std::ptr::eq(std::ptr::null::<*const u8>(), std::ptr::null::<*const u8>()), "static type violation");
                static_assert!(std::mem::size_of_val(&size) == std::mem::size_of::<usize>(), "static type violation");
                crate::base::macros::Use{dummy:0}.Use(start, size);
            }
        };
    }


    #[cfg(v8_use_address_sanitizer)]
    macro_rules! ASAN_UNPOISON_MEMORY_REGION {
        ($start:expr, $size:expr) => {
            unsafe {
                __asan_unpoison_memory_region($start as *mut std::ffi::c_void, $size as usize);
            }
        };
    }

    #[cfg(not(v8_use_address_sanitizer))]
    macro_rules! ASAN_UNPOISON_MEMORY_REGION {
        ($start:expr, $size:expr) => {
            ASAN_POISON_MEMORY_REGION!($start, $size)
        };
    }

    #[cfg(v8_use_address_sanitizer)]
    macro_rules! ASAN_CHECK_WHOLE_MEMORY_REGION_IS_POISONED {
        ($start:expr, $size:expr) => {
            {
                let start = $start;
                let size = $size;
                for i in 0..size {
                    unsafe {
                        assert!(__asan_address_is_poisoned((start as *const u8).offset(i as isize)) != 0);
                    }
                }
            }
        };
    }

    #[cfg(not(v8_use_address_sanitizer))]
    macro_rules! ASAN_CHECK_WHOLE_MEMORY_REGION_IS_POISONED {
        ($start:expr, $size:expr) => {
            ASAN_POISON_MEMORY_REGION!($start, $size)
        };
    }

    #[derive(Debug)]
    pub struct AsanUnpoisonScopeError;

    pub struct AsanUnpoisonScope {
        addr_: *const std::ffi::c_void,
        size_: usize,
        was_poisoned_: bool,
    }

    impl AsanUnpoisonScope {
        #[cfg(v8_use_address_sanitizer)]
        pub fn new(addr: *const std::ffi::c_void, size: usize) -> Self {
            let was_poisoned_ = unsafe { __asan_region_is_poisoned(addr, size) != 0 };
            if was_poisoned_ {
                unsafe {
                    __asan_unpoison_memory_region(addr as *mut std::ffi::c_void, size);
                }
            }
            AsanUnpoisonScope {
                addr_: addr,
                size_: size,
                was_poisoned_: was_poisoned_,
            }
        }

        #[cfg(not(v8_use_address_sanitizer))]
        pub fn new(addr: *const std::ffi::c_void, size: usize) -> Self {
            AsanUnpoisonScope {
                addr_: addr,
                size_: size,
                was_poisoned_: false,
            }
        }
    }

    impl Drop for AsanUnpoisonScope {
        #[cfg(v8_use_address_sanitizer)]
        fn drop(&mut self) {
            if self.was_poisoned_ {
                unsafe {
                    __asan_poison_memory_region(self.addr_ as *mut std::ffi::c_void, self.size_);
                }
            }
        }

        #[cfg(not(v8_use_address_sanitizer))]
        fn drop(&mut self) {}
    }

    #[cfg(not(v8_use_address_sanitizer))]
    macro_rules! DISABLE_ASAN {
        () => {};
    }

    #[cfg(v8_use_address_sanitizer)]
    macro_rules! DISABLE_ASAN {
        () => {
            #[allow(unused_attributes)]
            #[no_sanitize(address)]
            fn dummy() {}
        };
    }

    #[cfg(not(v8_use_hwaddress_sanitizer))]
    macro_rules! DISABLE_HWASAN {
        () => {};
    }

    #[cfg(v8_use_hwaddress_sanitizer)]
    macro_rules! DISABLE_HWASAN {
        () => {
            #[allow(unused_attributes)]
            #[no_sanitize(hwaddress)]
            fn dummy() {}
        };
    }
}

