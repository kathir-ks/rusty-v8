// Converted from V8 C++ source files:
// Header: memory.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2011 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]

use std::mem;
use std::ptr;

pub mod base {
    use std::{mem, ptr};

    pub type Address = usize;

    // Memory provides an interface to 'raw' memory. It encapsulates the casts
    // that typically are needed when incompatible pointer types are used.
    #[inline]
    pub fn memory<T>(addr: Address) -> &'static mut T {
        assert!(is_aligned(addr, mem::align_of::<T>()));
        unsafe { &mut *(addr as *mut T) }
    }

    #[inline]
    pub fn memory_from_u8<T>(addr: *mut u8) -> &'static mut T {
        memory::<T>(addr as Address)
    }

    #[inline]
    pub fn read_unaligned_value<V: Copy>(p: Address) -> V {
        let mut r: V = unsafe { mem::zeroed() };
        unsafe {
            ptr::copy_nonoverlapping(
                p as *const u8,
                &mut r as *mut V as *mut u8,
                mem::size_of::<V>(),
            );
        }
        r
    }

    #[inline]
    pub fn read_unaligned_value_from_array<V: Copy>(p: &[u8]) -> V {
        read_unaligned_value::<V>(p.as_ptr() as Address)
    }

    #[inline]
    pub fn write_unaligned_value<V: Copy>(p: Address, value: V) {
        unsafe {
            ptr::copy_nonoverlapping(
                &value as *const V as *const u8,
                p as *mut u8,
                mem::size_of::<V>(),
            );
        }
    }

    #[inline]
    pub fn write_unaligned_value_to_array<V: Copy>(p: &mut [u8], value: V) {
        write_unaligned_value::<V>(p.as_mut_ptr() as Address, value)
    }

    #[inline]
    pub fn read_little_endian_value<V: Copy>(p: Address) -> V {
        #[cfg(target_endian = "little")]
        {
            read_unaligned_value::<V>(p)
        }
        #[cfg(target_endian = "big")]
        {
            let mut ret: V = unsafe { mem::zeroed() };
            let src = p as *const u8;
            let dst = &mut ret as *mut V as *mut u8;
            for i in 0..mem::size_of::<V>() {
                unsafe {
                    *dst.add(i) = *src.add(mem::size_of::<V>() - i - 1);
                }
            }
            ret
        }
    }

    #[inline]
    pub fn write_little_endian_value<V: Copy>(p: Address, value: V) {
        #[cfg(target_endian = "little")]
        {
            write_unaligned_value::<V>(p, value);
        }
        #[cfg(target_endian = "big")]
        {
            let src = &value as *const V as *const u8;
            let dst = p as *mut u8;
            for i in 0..mem::size_of::<V>() {
                unsafe {
                    *dst.add(i) = *src.add(mem::size_of::<V>() - i - 1);
                }
            }
        }
    }

    #[inline]
    pub fn read_little_endian_value_ptr<V: Copy>(p: *mut V) -> V {
        read_little_endian_value::<V>(p as Address)
    }

    #[inline]
    pub fn write_little_endian_value_ptr<V: Copy>(p: *mut V, value: V) {
        write_little_endian_value::<V>(p as Address, value);
    }

    fn is_aligned(addr: Address, align: usize) -> bool {
        addr % align == 0
    }
}
