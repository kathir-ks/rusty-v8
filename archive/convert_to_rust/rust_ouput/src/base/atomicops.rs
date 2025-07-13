// Converted from V8 C++ source files:
// Header: atomicops.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2010 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]

use std::sync::atomic::{
    self, AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicPtr, Ordering,
};
use std::{mem, ptr};

#[cfg(target_os = "starboard")]
mod starboard {
    // Define Starboard atomic types and functions here.
    // This is a placeholder and needs to be replaced with actual Starboard API.
    pub type SbAtomic8 = std::sync::atomic::AtomicI8;
    pub type SbAtomic32 = std::sync::atomic::AtomicI32;

    #[cfg(target_pointer_width = "64")]
    pub type SbAtomic64 = std::sync::atomic::AtomicI64;
}

pub mod base {
    pub use crate::*;

    #[cfg(target_os = "starboard")]
    pub use starboard::*;

    #[cfg(not(target_os = "starboard"))]
    pub type Atomic8 = i8;
    #[cfg(not(target_os = "starboard"))]
    pub type Atomic16 = i16;
    #[cfg(not(target_os = "starboard"))]
    pub type Atomic32 = i32;

    #[cfg(all(target_pointer_width = "64", not(target_env = "__ILP32__")))]
    pub type Atomic64 = isize;

    #[cfg(target_pointer_width = "64")]
    pub type AtomicWord = Atomic64;
    #[cfg(not(target_pointer_width = "64"))]
    pub type AtomicWord = Atomic32;

    pub mod helper {
        use super::*;
        use std::sync::atomic::AtomicI8;
        use std::sync::atomic::AtomicI16;
        use std::sync::atomic::AtomicI32;
        use std::sync::atomic::AtomicI64;

        pub fn to_std_atomic_i8(ptr: *mut Atomic8) -> *mut AtomicI8 {
            ptr as *mut AtomicI8
        }

        pub fn to_std_atomic_i16(ptr: *mut Atomic16) -> *mut AtomicI16 {
            ptr as *mut AtomicI16
        }
        pub fn to_std_atomic_i32(ptr: *mut Atomic32) -> *mut AtomicI32 {
            ptr as *mut AtomicI32
        }
        pub fn to_std_atomic_i64(ptr: *mut Atomic64) -> *mut AtomicI64 {
            ptr as *mut AtomicI64
        }

        pub fn to_std_atomic_i8_const(ptr: *const Atomic8) -> *const AtomicI8 {
            ptr as *const AtomicI8
        }

        pub fn to_std_atomic_i16_const(ptr: *const Atomic16) -> *const AtomicI16 {
            ptr as *const AtomicI16
        }
        pub fn to_std_atomic_i32_const(ptr: *const Atomic32) -> *const AtomicI32 {
            ptr as *const AtomicI32
        }
        pub fn to_std_atomic_i64_const(ptr: *const Atomic64) -> *const AtomicI64 {
            ptr as *const AtomicI64
        }
    }

    #[inline]
    pub fn seq_cst_memory_fence() {
        atomic::fence(Ordering::SeqCst);
    }

    #[inline]
    pub fn relaxed_compare_and_swap(ptr: *mut Atomic8, old_value: Atomic8, new_value: Atomic8) -> Atomic8 {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i8(ptr) as *mut AtomicI8) };
        atomic_ptr.compare_exchange(old_value, new_value, Ordering::Relaxed, Ordering::Relaxed).unwrap_or(old_value)
    }

    #[inline]
    pub fn relaxed_compare_and_swap_i16(ptr: *mut Atomic16, old_value: Atomic16, new_value: Atomic16) -> Atomic16 {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i16(ptr) as *mut AtomicI16) };
        atomic_ptr.compare_exchange(old_value, new_value, Ordering::Relaxed, Ordering::Relaxed).unwrap_or(old_value)
    }

    #[inline]
    pub fn relaxed_compare_and_swap_i32(ptr: *mut Atomic32, old_value: Atomic32, new_value: Atomic32) -> Atomic32 {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i32(ptr) as *mut AtomicI32) };
        atomic_ptr.compare_exchange(old_value, new_value, Ordering::Relaxed, Ordering::Relaxed).unwrap_or(old_value)
    }

    #[inline]
    pub fn relaxed_atomic_exchange_i32(ptr: *mut Atomic32, new_value: Atomic32) -> Atomic32 {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i32(ptr) as *mut AtomicI32) };
        atomic_ptr.swap(new_value, Ordering::Relaxed)
    }

    #[inline]
    pub fn seq_cst_atomic_exchange_i32(ptr: *mut Atomic32, new_value: Atomic32) -> Atomic32 {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i32(ptr) as *mut AtomicI32) };
        atomic_ptr.swap(new_value, Ordering::SeqCst)
    }

    #[inline]
    pub fn relaxed_atomic_increment_i32(ptr: *mut Atomic32, increment: Atomic32) -> Atomic32 {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i32(ptr) as *mut AtomicI32) };
        atomic_ptr.fetch_add(increment, Ordering::Relaxed) + increment
    }

    #[inline]
    pub fn acquire_compare_and_swap_i32(ptr: *mut Atomic32, old_value: Atomic32, new_value: Atomic32) -> Atomic32 {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i32(ptr) as *mut AtomicI32) };
        atomic_ptr.compare_exchange(old_value, new_value, Ordering::Acquire, Ordering::Acquire).unwrap_or(old_value)
    }

    #[inline]
    pub fn release_compare_and_swap_i8(ptr: *mut Atomic8, old_value: Atomic8, new_value: Atomic8) -> Atomic8 {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i8(ptr) as *mut AtomicI8) };
        atomic_ptr.compare_exchange(old_value, new_value, Ordering::Release, Ordering::Relaxed).unwrap_or(old_value)
    }

    #[inline]
    pub fn release_compare_and_swap_i32(ptr: *mut Atomic32, old_value: Atomic32, new_value: Atomic32) -> Atomic32 {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i32(ptr) as *mut AtomicI32) };
        atomic_ptr.compare_exchange(old_value, new_value, Ordering::Release, Ordering::Relaxed).unwrap_or(old_value)
    }

    #[inline]
    pub fn acquire_release_compare_and_swap_i32(ptr: *mut Atomic32, old_value: Atomic32, new_value: Atomic32) -> Atomic32 {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i32(ptr) as *mut AtomicI32) };
        atomic_ptr.compare_exchange(old_value, new_value, Ordering::AcqRel, Ordering::Acquire).unwrap_or(old_value)
    }

    #[inline]
    pub fn seq_cst_compare_and_swap_i32(ptr: *mut Atomic32, old_value: Atomic32, new_value: Atomic32) -> Atomic32 {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i32(ptr) as *mut AtomicI32) };
        atomic_ptr.compare_exchange(old_value, new_value, Ordering::SeqCst, Ordering::SeqCst).unwrap_or(old_value)
    }

    #[inline]
    pub fn relaxed_store_i8(ptr: *mut Atomic8, value: Atomic8) {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i8(ptr) as *mut AtomicI8) };
        atomic_ptr.store(value, Ordering::Relaxed);
    }

    #[inline]
    pub fn relaxed_store_i16(ptr: *mut Atomic16, value: Atomic16) {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i16(ptr) as *mut AtomicI16) };
        atomic_ptr.store(value, Ordering::Relaxed);
    }

    #[inline]
    pub fn relaxed_store_i32(ptr: *mut Atomic32, value: Atomic32) {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i32(ptr) as *mut AtomicI32) };
        atomic_ptr.store(value, Ordering::Relaxed);
    }

    #[inline]
    pub fn release_store_i8(ptr: *mut Atomic8, value: Atomic8) {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i8(ptr) as *mut AtomicI8) };
        atomic_ptr.store(value, Ordering::Release);
    }

    #[inline]
    pub fn release_store_i16(ptr: *mut Atomic16, value: Atomic16) {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i16(ptr) as *mut AtomicI16) };
        atomic_ptr.store(value, Ordering::Release);
    }

    #[inline]
    pub fn release_store_i32(ptr: *mut Atomic32, value: Atomic32) {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i32(ptr) as *mut AtomicI32) };
        atomic_ptr.store(value, Ordering::Release);
    }

    #[inline]
    pub fn seq_cst_store_i8(ptr: *mut Atomic8, value: Atomic8) {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i8(ptr) as *mut AtomicI8) };
        atomic_ptr.store(value, Ordering::SeqCst);
    }

    #[inline]
    pub fn seq_cst_store_i16(ptr: *mut Atomic16, value: Atomic16) {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i16(ptr) as *mut AtomicI16) };
        atomic_ptr.store(value, Ordering::SeqCst);
    }

    #[inline]
    pub fn seq_cst_store_i32(ptr: *mut Atomic32, value: Atomic32) {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i32(ptr) as *mut AtomicI32) };
        atomic_ptr.store(value, Ordering::SeqCst);
    }

    #[inline]
    pub fn relaxed_load_i8(ptr: *const Atomic8) -> Atomic8 {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i8_const(ptr) as *const AtomicI8) };
        atomic_ptr.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn relaxed_load_i16(ptr: *const Atomic16) -> Atomic16 {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i16_const(ptr) as *const AtomicI16) };
        atomic_ptr.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn relaxed_load_i32(ptr: *const Atomic32) -> Atomic32 {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i32_const(ptr) as *const AtomicI32) };
        atomic_ptr.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn acquire_load_i8(ptr: *const Atomic8) -> Atomic8 {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i8_const(ptr) as *const AtomicI8) };
        atomic_ptr.load(Ordering::Acquire)
    }

    #[inline]
    pub fn acquire_load_i32(ptr: *const Atomic32) -> Atomic32 {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i32_const(ptr) as *const AtomicI32) };
        atomic_ptr.load(Ordering::Acquire)
    }

    #[inline]
    pub fn seq_cst_load_i8(ptr: *const Atomic8) -> Atomic8 {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i8_const(ptr) as *const AtomicI8) };
        atomic_ptr.load(Ordering::SeqCst)
    }

    #[inline]
    pub fn seq_cst_load_i32(ptr: *const Atomic32) -> Atomic32 {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i32_const(ptr) as *const AtomicI32) };
        atomic_ptr.load(Ordering::SeqCst)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    pub fn relaxed_compare_and_swap_i64(ptr: *mut Atomic64, old_value: Atomic64, new_value: Atomic64) -> Atomic64 {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i64(ptr) as *mut AtomicI64) };
        atomic_ptr.compare_exchange(old_value, new_value, Ordering::Relaxed, Ordering::Relaxed).unwrap_or(old_value)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    pub fn relaxed_atomic_exchange_i64(ptr: *mut Atomic64, new_value: Atomic64) -> Atomic64 {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i64(ptr) as *mut AtomicI64) };
        atomic_ptr.swap(new_value, Ordering::Relaxed)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    pub fn seq_cst_atomic_exchange_i64(ptr: *mut Atomic64, new_value: Atomic64) -> Atomic64 {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i64(ptr) as *mut AtomicI64) };
        atomic_ptr.swap(new_value, Ordering::SeqCst)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    pub fn relaxed_atomic_increment_i64(ptr: *mut Atomic64, increment: Atomic64) -> Atomic64 {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i64(ptr) as *mut AtomicI64) };
        atomic_ptr.fetch_add(increment, Ordering::Relaxed) + increment
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    pub fn acquire_compare_and_swap_i64(ptr: *mut Atomic64, old_value: Atomic64, new_value: Atomic64) -> Atomic64 {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i64(ptr) as *mut AtomicI64) };
        atomic_ptr.compare_exchange(old_value, new_value, Ordering::Acquire, Ordering::Acquire).unwrap_or(old_value)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    pub fn release_compare_and_swap_i64(ptr: *mut Atomic64, old_value: Atomic64, new_value: Atomic64) -> Atomic64 {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i64(ptr) as *mut AtomicI64) };
        atomic_ptr.compare_exchange(old_value, new_value, Ordering::Release, Ordering::Relaxed).unwrap_or(old_value)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    pub fn acquire_release_compare_and_swap_i64(ptr: *mut Atomic64, old_value: Atomic64, new_value: Atomic64) -> Atomic64 {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i64(ptr) as *mut AtomicI64) };
        atomic_ptr.compare_exchange(old_value, new_value, Ordering::AcqRel, Ordering::Acquire).unwrap_or(old_value)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    pub fn seq_cst_compare_and_swap_i64(ptr: *mut Atomic64, old_value: Atomic64, new_value: Atomic64) -> Atomic64 {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i64(ptr) as *mut AtomicI64) };
        atomic_ptr.compare_exchange(old_value, new_value, Ordering::SeqCst, Ordering::SeqCst).unwrap_or(old_value)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    pub fn relaxed_store_i64(ptr: *mut Atomic64, value: Atomic64) {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i64(ptr) as *mut AtomicI64) };
        atomic_ptr.store(value, Ordering::Relaxed);
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    pub fn release_store_i64(ptr: *mut Atomic64, value: Atomic64) {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i64(ptr) as *mut AtomicI64) };
        atomic_ptr.store(value, Ordering::Release);
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    pub fn seq_cst_store_i64(ptr: *mut Atomic64, value: Atomic64) {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i64(ptr) as *mut AtomicI64) };
        atomic_ptr.store(value, Ordering::SeqCst);
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    pub fn relaxed_load_i64(ptr: *const Atomic64) -> Atomic64 {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i64_const(ptr) as *const AtomicI64) };
        atomic_ptr.load(Ordering::Relaxed)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    pub fn acquire_load_i64(ptr: *const Atomic64) -> Atomic64 {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i64_const(ptr) as *const AtomicI64) };
        atomic_ptr.load(Ordering::Acquire)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    pub fn seq_cst_load_i64(ptr: *const Atomic64) -> Atomic64 {
        let atomic_ptr = unsafe { &*(helper::to_std_atomic_i64_const(ptr) as *const AtomicI64) };
        atomic_ptr.load(Ordering::SeqCst)
    }

    #[inline]
    pub fn relaxed_memcpy(dst: *mut Atomic8, src: *const Atomic8, bytes: usize) {
        let mut dst_ptr = dst;
        let mut src_ptr = src;
        let atomic_word_size = mem::size_of::<AtomicWord>();

        while bytes > 0 && !(dst_ptr as usize).is_aligned_to(atomic_word_size) {
            unsafe {
                relaxed_store_i8(dst_ptr, relaxed_load_i8(src_ptr));
                dst_ptr = dst_ptr.add(1);
                src_ptr = src_ptr.add(1);
            }
            let bytes_ptr = bytes - 1;
            // bytes -= 1;
            // bytes_ptr = bytes;
        }

        if (src as usize).is_aligned_to(atomic_word_size) && (dst as usize).is_aligned_to(atomic_word_size) {
            while bytes >= atomic_word_size {
                let dst_word_ptr = dst_ptr as *mut AtomicWord;
                let src_word_ptr = src_ptr as *const AtomicWord;
                unsafe {
                    relaxed_store(dst_word_ptr, relaxed_load(src_word_ptr));
                    dst_ptr = dst_ptr.add(atomic_word_size);
                    src_ptr = src_ptr.add(atomic_word_size);
                }
                let bytes_ptr = bytes - atomic_word_size;
                // bytes -= atomic_word_size;
                // bytes_ptr = bytes;
            }
        }

        while bytes > 0 {
            unsafe {
                relaxed_store_i8(dst_ptr, relaxed_load_i8(src_ptr));
                dst_ptr = dst_ptr.add(1);
                src_ptr = src_ptr.add(1);
            }
            let bytes_ptr = bytes - 1;
            // bytes -= 1;
            // bytes_ptr = bytes;
        }

        #[inline]
        fn relaxed_store(ptr: *mut AtomicWord, value: AtomicWord) {
            let atomic_ptr = ptr as *mut AtomicWord;
            unsafe {
                (atomic_ptr as *mut AtomicWord).write_volatile(value);
            }
        }

        #[inline]
        fn relaxed_load(ptr: *const AtomicWord) -> AtomicWord {
            let atomic_ptr = ptr as *const AtomicWord;
            unsafe { (atomic_ptr as *const AtomicWord).read_volatile() }
        }
    }

    #[inline]
    pub fn relaxed_memmove(dst: *mut Atomic8, src: *const Atomic8, bytes: usize) {
        if (dst as usize).wrapping_sub(src as usize) >= bytes {
            relaxed_memcpy(dst, src, bytes);
            return;
        }

        let mut dst_ptr = unsafe { dst.add(bytes) };
        let mut src_ptr = unsafe { src.add(bytes) };
        let atomic_word_size = mem::size_of::<AtomicWord>();

        while bytes > 0 && !(dst_ptr as usize).is_aligned_to(atomic_word_size) {
            unsafe {
                dst_ptr = dst_ptr.sub(1);
                src_ptr = src_ptr.sub(1);
                relaxed_store_i8(dst_ptr, relaxed_load_i8(src_ptr));
            }
            let bytes_ptr = bytes - 1;
            // bytes -= 1;
            // bytes_ptr = bytes;
        }

        if (src as usize).is_aligned_to(atomic_word_size) && (dst as usize).is_aligned_to(atomic_word_size) {
            while bytes >= atomic_word_size {
                unsafe {
                    dst_ptr = dst_ptr.sub(atomic_word_size);
                    src_ptr = src_ptr.sub(atomic_word_size);
                    relaxed_store(dst_ptr as *mut AtomicWord, relaxed_load(src_ptr as *const AtomicWord));
                }
                let bytes_ptr = bytes - atomic_word_size;
                // bytes -= atomic_word_size;
                // bytes_ptr = bytes;
            }
        }

        while bytes > 0 {
            unsafe {
                dst_ptr = dst_ptr.sub(1);
                src_ptr = src_ptr.sub(1);
                relaxed_store_i8(dst_ptr, relaxed_load_i8(src_ptr));
            }
            let bytes_ptr = bytes - 1;
            // bytes -= 1;
            // bytes_ptr = bytes;
        }

        #[inline]
        fn relaxed_store(ptr: *mut AtomicWord, value: AtomicWord) {
            let atomic_ptr = ptr as *mut AtomicWord;
            unsafe {
                (atomic_ptr as *mut AtomicWord).write_volatile(value);
            }
        }

        #[inline]
        fn relaxed_load(ptr: *const AtomicWord) -> AtomicWord {
            let atomic_ptr = ptr as *const AtomicWord;
            unsafe { (atomic_ptr as *const AtomicWord).read_volatile() }
        }
    }

    pub mod helper_memcmp {
        use super::*;

        #[inline]
        pub fn memcmp_not_equal_fundamental_i8(u1: Atomic8, u2: Atomic8) -> i32 {
            debug_assert_ne!(u1, u2);
            if u1 < u2 {
                -1
            } else {
                1
            }
        }

        #[inline]
        pub fn memcmp_not_equal_fundamental_word(u1: AtomicWord, u2: AtomicWord) -> i32 {
            debug_assert_ne!(u1, u2);
            #[cfg(target_endian = "big")]
            {
                if u1 < u2 {
                    -1
                } else {
                    1
                }
            }
            #[cfg(target_endian = "little")]
            {
                for i in 0..mem::size_of::<AtomicWord>() {
                    let byte1 = (u1 & 0xFF) as u8;
                    let byte2 = (u2 & 0xFF) as u8;
                    if byte1 != byte2 {
                        return if byte1 < byte2 { -1 } else { 1 };
                    }
                    // u1 >>= 8;
                    // u2 >>= 8;
                    let u1 = u1 >> 8;
                    let u2 = u2 >> 8;
                }
                unreachable!();
            }
        }
    }

    #[inline]
    pub fn relaxed_memcmp(s1: *const Atomic8, s2: *const Atomic8, len: usize) -> i32 {
        let mut s1_ptr = s1;
        let mut s2_ptr = s2;
        let atomic_word_size = mem::size_of::<AtomicWord>();

        while len > 0 && !((s1 as usize).is_aligned_to(atomic_word_size) && (s2 as usize).is_aligned_to(atomic_word_size)) {
            unsafe {
                let u1 = relaxed_load_i8(s1_ptr);
                s1_ptr = s1_ptr.add(1);
                let u2 = relaxed_load_i8(s2_ptr);
                s2_ptr = s2_ptr.add(1);
                if u1 != u2 {
                    return helper_memcmp::memcmp_not_equal_fundamental_i8(u1, u2);
                }
            }
            let len_ptr = len - 1;
            // len -= 1;
            // len_ptr = len;
        }

        if (s1 as usize).is_aligned_to(atomic_word_size) && (s2 as usize).is_aligned_to(atomic_word_size) {
            while len >= atomic_word_size {
                unsafe {
                    let u1 = relaxed_load(s1_ptr as *const AtomicWord);
                    let u2 = relaxed_load(s2_ptr as *const AtomicWord);
                    if u1 != u2 {
                        return helper_memcmp::memcmp_not_equal_fundamental_word(u1, u2);
                    }
                    s1_ptr = s1_ptr.add(atomic_word_size);
                    s2_ptr = s2_ptr.add(atomic_word_size);
                }
                let len_ptr = len - atomic_word_size;
                // len -= atomic_word_size;
                // len_ptr = len;
            }
        }

        while len > 0 {
            unsafe {
                let u1 = relaxed_load_i8(s1_ptr);
                s1_ptr = s1_ptr.add(1);
                let u2 = relaxed_load_i8(s2_ptr);
                s2_ptr = s2_ptr.add(1);
                if u1 != u2 {
                    return helper_memcmp::memcmp_not_equal_fundamental_i8(u1, u2);
                }
            }
            let len_ptr = len - 1;
            // len -= 1;
            // len_ptr = len;
        }

        return 0;

        #[inline]
        fn relaxed_load(ptr: *const AtomicWord) -> AtomicWord {
            let atomic_ptr = ptr as *const AtomicWord;
            unsafe { (atomic_ptr as *const AtomicWord).read_volatile() }
        }
    }

    trait Aligned {
        fn is_aligned_to(&self, align: usize) -> bool;
    }

    impl Aligned for usize {
        fn is_aligned_to(&self, align: usize) -> bool {
            (self & (align - 1)) == 0
        }
    }
}
