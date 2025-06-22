// Copyright 2011 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod base {
    use std::{mem, ptr};

    pub type Address = usize;

    /// Provides an interface to 'raw' memory. It encapsulates the casts
    /// that typically are needed when incompatible pointer types are used.
    #[inline]
    pub fn memory<T>(addr: Address) -> &'static mut T {
        assert!(addr % mem::align_of::<T>() == 0);
        unsafe { &mut *(addr as *mut T) }
    }

    #[inline]
    pub fn memory_from_u8<T>(addr: *mut u8) -> &'static mut T {
        memory::<T>(addr as Address)
    }

    /// Reads an unaligned value from memory.
    #[inline]
    pub fn read_unaligned_value<V: Copy>(p: Address) -> V {
        let mut r: V = unsafe { mem::zeroed() };
        unsafe {
            ptr::copy_nonoverlapping(p as *const u8, &mut r as *mut V as *mut u8, mem::size_of::<V>());
        }
        r
    }

    /// Reads an unaligned value from a char array.
    #[inline]
    pub fn read_unaligned_value_from_char_array<V: Copy>(p: &[u8]) -> V {
        assert!(p.len() >= mem::size_of::<V>());
        read_unaligned_value::<V>(p.as_ptr() as Address)
    }

    /// Writes an unaligned value to memory.
    #[inline]
    pub fn write_unaligned_value<V: Copy>(p: Address, value: V) {
        unsafe {
            ptr::copy_nonoverlapping(&value as *const V as *const u8, p as *mut u8, mem::size_of::<V>());
        }
    }

    /// Writes an unaligned value to a char array.
    #[inline]
    pub fn write_unaligned_value_to_char_array<V: Copy>(p: &mut [u8], value: V) {
        assert!(p.len() >= mem::size_of::<V>());
        write_unaligned_value::<V>(p.as_mut_ptr() as Address, value);
    }

    /// Reads a little-endian value from memory.
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
            unsafe {
                for i in 0..mem::size_of::<V>() {
                    *dst.add(i) = *src.add(mem::size_of::<V>() - i - 1);
                }
            }
            ret
        }
    }

    /// Writes a little-endian value to memory.
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
            unsafe {
                for i in 0..mem::size_of::<V>() {
                    *dst.add(i) = *src.add(mem::size_of::<V>() - i - 1);
                }
            }
        }
    }

    /// Reads a little-endian value from a pointer.
    #[inline]
    pub fn read_little_endian_value_from_ptr<V: Copy>(p: *mut V) -> V {
        read_little_endian_value::<V>(p as Address)
    }

    /// Writes a little-endian value to a pointer.
    #[inline]
    pub fn write_little_endian_value_to_ptr<V: Copy>(p: *mut V, value: V) {
        write_little_endian_value::<V>(p as Address, value);
    }
}