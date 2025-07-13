// Converted from V8 C++ source files:
// Header: caged-heap.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_upper_case_globals)]

pub mod api_constants {
    pub const kCagedHeapReservationAlignment: usize = 1 << 20;
    pub const kPointerCompressionShift: usize = 0;
    pub const kCagedHeapMaxReservationSize: usize = 1 << 30;
}

pub mod internal {
    use crate::api_constants;
    use std::sync::Mutex;

    pub struct CagedHeapBase {
    }

    impl CagedHeapBase {
        #[inline]
        pub fn offset_from_address(address: *const std::ffi::c_void) -> usize {
            let address_usize = address as usize;
            address_usize & (api_constants::kCagedHeapReservationAlignment - 1)
        }

        #[inline]
        pub fn is_within_cage(address: *const std::ffi::c_void) -> bool {
            unsafe {
                if g_heap_base.lock().unwrap().is_none() {
                    return false;
                }
                let heap_base = g_heap_base.lock().unwrap().unwrap();

                (address as usize & !(api_constants::kCagedHeapReservationAlignment - 1)) == heap_base
            }
        }

        #[inline]
        pub fn are_within_cage(addr1: *const std::ffi::c_void, addr2: *const std::ffi::c_void) -> bool {
            unsafe {
                if g_heap_base.lock().unwrap().is_none() {
                    return false;
                }
                let heap_base = g_heap_base.lock().unwrap().unwrap();

                let k_heap_base_shift: usize = std::mem::size_of::<u32>() * 8;

                if (1usize << k_heap_base_shift) != api_constants::kCagedHeapMaxReservationSize {
                    panic!("Assertion failed: (static_cast<size_t>(1) << kHeapBaseShift) == api_constants::kCagedHeapMaxReservationSize");
                }

                !(((addr1 as usize ^ heap_base) | (addr2 as usize ^ heap_base)) >> k_heap_base_shift != 0)
            }
        }

        #[inline]
        pub fn get_base() -> usize {
            unsafe {
                if g_heap_base.lock().unwrap().is_none() {
                    return 0;
                }
                g_heap_base.lock().unwrap().unwrap()
            }
        }

        #[inline]
        pub fn get_age_table_size() -> usize {
            unsafe {
                g_age_table_size.lock().unwrap().clone()
            }
        }
    }

    lazy_static::lazy_static! {
        static ref g_heap_base: Mutex<Option<usize>> = Mutex::new(None);
        static ref g_age_table_size: Mutex<usize> = Mutex::new(0);
    }

    pub fn set_g_heap_base(base: usize) {
        let mut guard = g_heap_base.lock().unwrap();
        *guard = Some(base);
    }

    pub fn set_g_age_table_size(size: usize) {
        let mut guard = g_age_table_size.lock().unwrap();
        *guard = size;
    }
}
