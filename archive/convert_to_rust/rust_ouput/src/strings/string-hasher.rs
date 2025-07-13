// Converted from V8 C++ source files:
// Header: string-hasher.h
// Implementation: string-hasher.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
mod std {
    pub use core::ffi;
    pub use core::mem;
    pub use core::ops;
    pub use std::*;
}
pub mod v8 {
    pub mod base {
        pub struct Vector<T> {
            data: Vec<T>,
        }
        impl<T> Vector<T> {
            pub fn new() -> Self {
                Vector { data: Vec::new() }
            }
        }
    }
    pub mod internal {
        use std::mem;
        pub struct RunningStringHasher {
            running_hash_: u32,
        }
        impl RunningStringHasher {
            pub fn new(seed: u32) -> Self {
                RunningStringHasher { running_hash_: seed }
            }
            #[inline]
            pub fn add_character(&mut self, c: u16) {
                self.running_hash_ = self.running_hash_.wrapping_add(c as u32);
                self.running_hash_ = self.running_hash_.wrapping_add(self.running_hash_ << 10);
                self.running_hash_ ^= self.running_hash_ >> 6;
            }
            #[inline]
            pub fn finalize(&mut self) -> u32 {
                self.running_hash_ = self.running_hash_.wrapping_add(self.running_hash_ << 3);
                self.running_hash_ ^= self.running_hash_ >> 11;
                self.running_hash_ = self.running_hash_.wrapping_add(self.running_hash_ << 15);
                self.running_hash_
            }
        }
        pub struct StringHasher {}
        impl StringHasher {
            #[inline]
            pub fn hash_sequential_string<T>(chars: &[T], length: u32, seed: u64) -> u32
            where
                T: Copy,
                T: std::fmt::Debug,
                T: AsRef<[u8]>,
            {
                let mut hasher = RunningStringHasher::new(seed as u32);
                for i in 0..length {
                    let char_bytes = chars[i as usize].as_ref();
                    if char_bytes.len() == 1 {
                        hasher.add_character(char_bytes[0] as u16);
                    } else if char_bytes.len() == 2 {
                        hasher.add_character(((char_bytes[0] as u16) << 8) | char_bytes[1] as u16);
                    } else {
                        panic!("Character size not supported");
                    }
                }
                let mut result = hasher.finalize();
                if result == 0 {
                    result = StringHasher::k_zero_hash as u32;
                }
                result
            }
            #[inline]
            pub fn make_array_index_hash(value: u32, length: u32) -> u32 {
                let mut hasher = RunningStringHasher::new(0);
                let value_str = value.to_string();
                for c in value_str.chars() {
                    hasher.add_character(c as u16);
                }
                hasher.finalize()
            }
            pub const k_zero_hash: i32 = 27;
            #[inline]
            pub fn get_trivial_hash(length: u32) -> u32 {
                length
            }
        }
        pub struct SeededStringHasher {
            hashseed_: u64,
        }
        impl SeededStringHasher {
            pub fn new(hashseed: u64) -> Self {
                SeededStringHasher { hashseed_: hashseed }
            }
            #[inline]
            pub fn call(&self, name: *const i8) -> usize {
                let name_str = unsafe {
                    let mut len = 0;
                    while *name.offset(len) != 0 {
                        len += 1;
                    }
                    let slice = std::slice::from_raw_parts(name as *const u8, len as usize);
                    std::str::from_utf8(slice).unwrap()
                };
                let hash = StringHasher::hash_sequential_string(
                    name_str.as_bytes(),
                    name_str.len() as u32,
                    self.hashseed_,
                );
                hash as usize
            }
        }
        pub struct StringEquals {}
        impl StringEquals {
            pub fn new() -> Self {
                StringEquals {}
            }
            pub fn call(&self, name1: *const i8, name2: *const i8) -> bool {
                let name1_str = unsafe {
                    let mut len = 0;
                    while *name1.offset(len) != 0 {
                        len += 1;
                    }
                    let slice = std::slice::from_raw_parts(name1 as *const u8, len as usize);
                    std::str::from_utf8(slice).unwrap()
                };
                let name2_str = unsafe {
                    let mut len = 0;
                    while *name2.offset(len) != 0 {
                        len += 1;
                    }
                    let slice = std::slice::from_raw_parts(name2 as *const u8, len as usize);
                    std::str::from_utf8(slice).unwrap()
                };
                name1_str == name2_str
            }
        }
        struct ConvertTo8BitHashReader {}
        impl ConvertTo8BitHashReader {
            const K_COMPRESSION_FACTOR: usize = 2;
            const K_EXPANSION_FACTOR: usize = 1;
            #[inline]
            fn read64(ptr: *const u8) -> u64 {
                let p = ptr as *const u16;
                let p_array = unsafe { [
                    *p.offset(0),
                    *p.offset(1),
                    *p.offset(2),
                    *p.offset(3),
                    *p.offset(4),
                    *p.offset(5),
                    *p.offset(6),
                    *p.offset(7)
                ]};
                assert!(p_array[0] <= 0xff);
                assert!(p_array[1] <= 0xff);
                assert!(p_array[2] <= 0xff);
                assert!(p_array[3] <= 0xff);
                assert!(p_array[4] <= 0xff);
                assert!(p_array[5] <= 0xff);
                assert!(p_array[6] <= 0xff);
                assert!(p_array[7] <= 0xff);
                (p_array[0] as u64) | ((p_array[1] as u64) << 8) | ((p_array[2] as u64) << 16) |
                       ((p_array[3] as u64) << 24) | ((p_array[4] as u64) << 32) |
                       ((p_array[5] as u64) << 40) | ((p_array[6] as u64) << 48) |
                       ((p_array[7] as u64) << 56)
            }
            #[inline]
            fn read32(ptr: *const u8) -> u64 {
                let p = ptr as *const u16;
                let p_array = unsafe { [
                    *p.offset(0),
                    *p.offset(1),
                    *p.offset(2),
                    *p.offset(3)
                ]};
                assert!(p_array[0] <= 0xff);
                assert!(p_array[1] <= 0xff);
                assert!(p_array[2] <= 0xff);
                assert!(p_array[3] <= 0xff);
                (p_array[0] as u64) | ((p_array[1] as u64) << 8) | ((p_array[2] as u64) << 16) |
                       ((p_array[3] as u64) << 24)
            }
            #[inline]
            fn read_small(ptr: *const u8, k: usize) -> u64 {
                let p = ptr as *const u16;
                let p_array = unsafe { [
                    *p.offset(0),
                    *p.offset((k >> 1) as isize),
                    *p.offset((k - 1) as isize)
                ]};
                assert!(p_array[0] <= 0xff);
                assert!(p_array[1] <= 0xff);
                assert!(p_array[2] <= 0xff);
                ((p_array[0] as u64) << 56) | ((p_array[1] as u64) << 32) | (p_array[2] as u64)
            }
        }
        mod detail {
            use super::*;
            pub fn hash_converting_to8bit(chars: *const u16, length: u32, seed: u64) -> u64 {
                let slice = unsafe { std::slice::from_raw_parts(chars as *const u8, (length as usize) * 2) };
                rapidhash::<ConvertTo8BitHashReader>(slice, length, seed)
            }
        }
        fn rapidhash<T>(chars: &[u8], length: u32, seed: u64) -> u64 {
            let mut a = 0x4942d66c55d08a7a ^ seed;
            let mut b = 0x6996c53aca5e485c ^ seed;
            let mut c = 0x4a3c9b45bc882eb5 ^ seed;
            let len = length as usize;
            let mut i = 0;
            while i + 32 <= len {
                let t1 = T::read64(chars.as_ptr().wrapping_add(i));
                let t2 = T::read64(chars.as_ptr().wrapping_add(i + 8));
                let t3 = T::read64(chars.as_ptr().wrapping_add(i + 16));
                let t4 = T::read64(chars.as_ptr().wrapping_add(i + 24));
                a = a.wrapping_add(t1);
                b = b.wrapping_add(t2);
                c = c.wrapping_add(t3);
                mix64(&mut a, &mut b, &mut c);
                a = a.wrapping_add(t4);
                mix64(&mut a, &mut b, &mut c);
                i += 32;
            }
            if i + 16 <= len {
                let t1 = T::read64(chars.as_ptr().wrapping_add(i));
                let t2 = T::read64(chars.as_ptr().wrapping_add(i + 8));
                a = a.wrapping_add(t1);
                b = b.wrapping_add(t2);
                mix64(&mut a, &mut b, &mut c);
                i += 16;
            }
            if i + 8 <= len {
                let t1 = T::read64(chars.as_ptr().wrapping_add(i));
                a = a.wrapping_add(t1);
                mix64(&mut a, &mut b, &mut c);
                i += 8;
            }
            if i + 4 <= len {
                let t1 = T::read32(chars.as_ptr().wrapping_add(i)) as u64;
                a = a.wrapping_add(t1);
                mix64(&mut a, &mut b, &mut c);
                i += 4;
            }
            if i < len {
                let t1 = T::read_small(chars.as_ptr().wrapping_add(i), len - i);
                a = a.wrapping_add(t1);
                mix64(&mut a, &mut b, &mut c);
            }
            finalize64(&mut a, &mut b, &mut c);
            c
        }
        #[inline(always)]
        fn mix64(a: &mut u64, b: &mut u64, c: &mut u64) {
            *a = a.wrapping_sub(*b);
            *a ^= *c >> 43;
            *b = b.wrapping_sub(*c);
            *b ^= *a << 9;
            *c = c.wrapping_sub(*a);
            *c ^= *b >> 8;
            *a = a.wrapping_sub(*b);
            *a ^= *c >> 38;
            *b = b.wrapping_sub(*c);
            *b ^= *a << 23;
            *c = c.wrapping_sub(*a);
            *c ^= *b >> 5;
            *a = a.wrapping_sub(*b);
            *a ^= *c >> 35;
            *b = b.wrapping_sub(*c);
            *b ^= *a << 49;
            *c = c.wrapping_sub(*a);
            *c ^= *b >> 3;
            *a = a.wrapping_sub(*b);
            *a ^= *c >> 43;
            *b = b.wrapping_sub(*c);
            *b ^= *a << 9;
            *c = c.wrapping_sub(*a);
            *c ^= *b >> 8;
        }
        #[inline(always)]
        fn finalize64(a: &mut u64, b: &mut u64, c: &mut u64) {
            *c ^= *b;
            *c = c.wrapping_sub(*b << 45 | *b >> 19);
            *a ^= *c;
            *a = a.wrapping_sub(*c << 14 | *c >> 50);
            *b ^= *a;
            *b = b.wrapping_sub(*a << 4 | *a >> 60);
            *c ^= *b;
            *c = c.wrapping_sub(*b << 33 | *b >> 31);
            *a ^= *c;
            *a = a.wrapping_sub(*c << 11 | *c >> 53);
            *b ^= *a;
            *b = b.wrapping_sub(*a << 38 | *a >> 26);
            *c ^= *b;
            *c = c.wrapping_sub(*b << 22 | *b >> 42);
            *a ^= *c;
            *a = a.wrapping_sub(*c << 16 | *c >> 48);
            *b ^= *a;
            *b = b.wrapping_sub(*a << 44 | *a >> 20);
        }
    }
}
