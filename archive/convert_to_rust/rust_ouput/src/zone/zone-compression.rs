// Converted from V8 C++ source files:
// Header: zone-compression.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod bits {
        pub fn is_power_of_two(x: usize) -> bool {
            x != 0 && (x & (x - 1)) == 0
        }
    }
}

pub mod common {
    pub mod globals {
        pub const COMPRESS_ZONES_BOOL: bool = true;
    }
}

pub mod internal {
    use crate::common::globals::COMPRESS_ZONES_BOOL;
    use std::assert;

    pub type Address = usize;

    pub struct ZoneCompression {}

    impl ZoneCompression {
        pub const K_RESERVATION_SIZE: usize = 2 * (1 << 30); // 2GB
        pub const K_RESERVATION_ALIGNMENT: usize = if COMPRESS_ZONES_BOOL {
            4 * (1 << 30) // 4GB
        } else {
            1
        };

        pub const K_OFFSET_MASK: usize = ZoneCompression::K_RESERVATION_ALIGNMENT - 1;
        pub const K_NULL_ADDRESS: Address = 0;

        pub fn base_of(zone_pointer: *const std::ffi::c_void) -> Address {
            let ptr_addr = zone_pointer as usize;
            ptr_addr & !ZoneCompression::K_OFFSET_MASK
        }

        pub fn check_same_base(p1: *const std::ffi::c_void, p2: *const std::ffi::c_void) -> bool {
            if p1.is_null() || p2.is_null() {
                return true;
            }

            assert_eq!(ZoneCompression::base_of(p1), ZoneCompression::base_of(p2));
            true
        }

        pub fn compress(value: *const std::ffi::c_void) -> u32 {
            let raw_value = value as usize;
            let compressed_value = (raw_value & ZoneCompression::K_OFFSET_MASK) as u32;

            if compressed_value == 0 {
                assert!(value.is_null());
            }

            assert!(compressed_value as usize <= ZoneCompression::K_RESERVATION_SIZE);
            compressed_value
        }

        pub fn decompress(zone_pointer: *const std::ffi::c_void, compressed_value: u32) -> Address {
            if compressed_value == 0 {
                return ZoneCompression::K_NULL_ADDRESS;
            }
            ZoneCompression::base_of(zone_pointer) + compressed_value as usize
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::ptr;

        #[test]
        fn test_base_of() {
            let addr: usize = 0x123456789ABCDEF0;
            let aligned_addr = addr & !ZoneCompression::K_OFFSET_MASK;
            let ptr = addr as *const std::ffi::c_void;
            assert_eq!(ZoneCompression::base_of(ptr), aligned_addr);
        }

        #[test]
        fn test_check_same_base() {
            let addr1: usize = 0x123456789ABCDEF0;
            let addr2: usize = 0x123456789ABCE000; // Different offset, same base
            let ptr1 = addr1 as *const std::ffi::c_void;
            let ptr2 = addr2 as *const std::ffi::c_void;
            assert!(ZoneCompression::check_same_base(ptr1, ptr2));

            let addr3: usize = 0x987654321FEDCBA0; // Different base
            let ptr3 = addr3 as *const std::ffi::c_void;
           // assert!(!ZoneCompression::check_same_base(ptr1, ptr3));  Will panic in debug mode
           assert!(ZoneCompression::check_same_base(ptr1, ptr::null()));
        }

        #[test]
        fn test_compress_decompress() {
            let base_addr: usize = 0x100000000000;
            let offset: usize = 0x1234;
            let original_addr: usize = base_addr + offset;
            let original_ptr = original_addr as *const std::ffi::c_void;

            let compressed_value = ZoneCompression::compress(original_ptr);
            assert_eq!(compressed_value as usize, offset);

            let decompressed_addr = ZoneCompression::decompress(original_ptr, compressed_value);
            assert_eq!(decompressed_addr, original_addr);

             // Test nullptr compression/decompression
            let null_ptr: *const std::ffi::c_void = ptr::null();
            let compressed_null = ZoneCompression::compress(null_ptr);
            assert_eq!(compressed_null, 0);
            let decompressed_null = ZoneCompression::decompress(null_ptr, compressed_null);
            assert_eq!(decompressed_null, ZoneCompression::K_NULL_ADDRESS);

        }
    }
}
