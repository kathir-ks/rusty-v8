// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod memcopy {
    use std::{
        cmp,
        mem,
        ptr,
    };
    use std::os::raw::c_void;

    // Replaces V8's Address, which is just a usize/uintptr_t
    pub type Address = usize;

    // Placeholder for architecture-specific initialization.
    pub fn init_memcopy_functions() {}

    // Define architecture-specific implementations and constants.
    #[cfg(target_arch = "x86")]
    pub mod ia32 {
        use super::*;

        // Limit below which the extra overhead of the MemCopy function is likely
        // to outweigh the benefits of faster copying.
        pub const K_MIN_COMPLEX_MEM_COPY: usize = 64;

        // Copy memory area. No restrictions.
        pub fn mem_move(dest: *mut c_void, src: *const c_void, size: usize) {
            unsafe {
                ptr::copy_nonoverlapping(src as *const u8, dest as *mut u8, size);
            }
        }

        // Keep the distinction of "move" vs. "copy" for the benefit of other
        // architectures.
        #[inline]
        pub fn mem_copy(dest: *mut c_void, src: *const c_void, size: usize) {
            mem_move(dest, src, size);
        }
    }

    #[cfg(target_arch = "arm")]
    pub mod arm {
        use super::*;

        // This extern function pointer from the C++ code is replaced with a Rust function.
        // In the original code, the extern function pointer would be initialized by init_memcopy_functions().
        // Here we provide a default implementation. The actual choice of function could be configurable in a real system.
        pub fn memcopy_uint8_function(dest: *mut u8, src: *const u8, size: usize) {
            unsafe {
                ptr::copy_nonoverlapping(src, dest, size);
            }
        }

        #[inline]
        pub fn mem_copy_uint8_wrapper(dest: *mut u8, src: *const u8, chars: usize) {
            unsafe {
                ptr::copy_nonoverlapping(src, dest, chars);
            }
        }

        // For values < 16, the assembler function is slower than the inlined C code.
        pub const K_MIN_COMPLEX_MEM_COPY: usize = 16;

        #[inline]
        pub fn mem_copy(dest: *mut c_void, src: *const c_void, size: usize) {
            unsafe {
                memcopy_uint8_function(dest as *mut u8, src as *const u8, size);
            }
        }

        #[inline]
        pub fn mem_move(dest: *mut c_void, src: *const c_void, size: usize) {
            unsafe {
                ptr::copy(src as *const u8, dest as *mut u8, size);
            }
        }

        // For values < 12, the assembler function is slower than the inlined C code.
        pub const K_MIN_COMPLEX_CONVERT_MEM_COPY: i32 = 12;
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "arm")))]
    pub mod generic {
        use super::*;

        // Implementations for architectures other than ia32 and arm.
        #[cfg(all(feature = "v8_optimize_with_neon", target_arch = "aarch64"))]
        pub mod neon {
            use super::*;
            use std::arch::aarch64::*;

            // We intentionally use misaligned read/writes for NEON intrinsics, disable
            // alignment sanitization explicitly.
            // Overlapping writes help to save instructions, e.g. doing 2 two-byte writes
            // instead 3 one-byte write for count == 3.
            #[inline]
            unsafe fn overlapping_writes<IntType>(dst: *mut c_void, src: *const c_void, count: usize) {
                let dst = dst as *mut IntType;
                let src = src as *const IntType;
                *dst = *src;
                let dst = (dst as *mut u8).add(count - mem::size_of::<IntType>()) as *mut IntType;
                let src = (src as *const u8).add(count - mem::size_of::<IntType>()) as *const IntType;
                *dst = *src;
            }

            #[inline]
            pub fn mem_copy(dst: *mut c_void, src: *const c_void, count: usize) {
                let dst_u = dst as *mut u8;
                let src_u = src as *const u8;

                unsafe {
                    // Common cases. Handle before doing clz.
                    if count == 0 {
                        return;
                    }
                    if count == 1 {
                        *dst_u = *src_u;
                        return;
                    }

                    let order = mem::size_of::<usize>() * 8 - (count - 1).leading_zeros() as usize;
                    match order {
                        1 => {
                            // count: [2, 2]
                            *(dst_u as *mut u16) = *(src_u as *const u16);
                            return;
                        }
                        2 => {
                            // count: [3, 4]
                            overlapping_writes::<u16>(dst, src, count);
                            return;
                        }
                        3 => {
                            // count: [5, 8]
                            overlapping_writes::<u32>(dst, src, count);
                            return;
                        }
                        4 => {
                            // count: [9, 16]
                            overlapping_writes::<u64>(dst, src, count);
                            return;
                        }
                        5 => {
                            // count: [17, 32]
                            vst1q_u8(dst_u, vld1q_u8(src_u));
                            vst1q_u8(dst_u.add(count - 16), vld1q_u8(src_u.add(count - 16)));
                            return;
                        }
                        _ => {
                            // count: [33, ...]
                            vst1q_u8(dst_u, vld1q_u8(src_u));
                            for i in (count % 16..count).step_by(16) {
                                vst1q_u8(dst_u.add(i), vld1q_u8(src_u.add(i)));
                            }
                            return;
                        }
                    }
                }
            }
        }

        #[cfg(all(feature = "v8_optimize_with_neon", target_arch = "aarch64"))]
        use neon::mem_copy;

        #[cfg(not(all(feature = "v8_optimize_with_neon", target_arch = "aarch64")))]
        #[inline]
        pub fn mem_copy(dest: *mut c_void, src: *const c_void, size: usize) {
            unsafe {
                match size {
                    1 => { *(dest as *mut u8) = *(src as *const u8); }
                    2 => { *(dest as *mut u16) = *(src as *const u16); }
                    3 => {
                        let dest = dest as *mut u8;
                        let src = src as *const u8;
                        ptr::copy_nonoverlapping(src, dest, 3);
                    }
                    4 => { *(dest as *mut u32) = *(src as *const u32); }
                    5 => {
                        let dest = dest as *mut u8;
                        let src = src as *const u8;
                        ptr::copy_nonoverlapping(src, dest, 5);
                    }
                    6 => {
                        let dest = dest as *mut u8;
                        let src = src as *const u8;
                        ptr::copy_nonoverlapping(src, dest, 6);
                    }
                    7 => {
                        let dest = dest as *mut u8;
                        let src = src as *const u8;
                        ptr::copy_nonoverlapping(src, dest, 7);
                    }
                    8 => { *(dest as *mut u64) = *(src as *const u64); }
                    9 => {
                        let dest = dest as *mut u8;
                        let src = src as *const u8;
                        ptr::copy_nonoverlapping(src, dest, 9);
                    }
                    10 => {
                        let dest = dest as *mut u8;
                        let src = src as *const u8;
                        ptr::copy_nonoverlapping(src, dest, 10);
                    }
                    11 => {
                        let dest = dest as *mut u8;
                        let src = src as *const u8;
                        ptr::copy_nonoverlapping(src, dest, 11);
                    }
                    12 => {
                        let dest = dest as *mut u8;
                        let src = src as *const u8;
                        ptr::copy_nonoverlapping(src, dest, 12);
                    }
                    13 => {
                        let dest = dest as *mut u8;
                        let src = src as *const u8;
                        ptr::copy_nonoverlapping(src, dest, 13);
                    }
                    14 => {
                        let dest = dest as *mut u8;
                        let src = src as *const u8;
                        ptr::copy_nonoverlapping(src, dest, 14);
                    }
                    15 => {
                        let dest = dest as *mut u8;
                        let src = src as *const u8;
                        ptr::copy_nonoverlapping(src, dest, 15);
                    }
                    16 => {
                        let dest = dest as *mut u8;
                        let src = src as *const u8;
                        ptr::copy_nonoverlapping(src, dest, 16);
                    }
                    _ => {
                        ptr::copy_nonoverlapping(src as *const u8, dest as *mut u8, size);
                    }
                }
            }
        }

        #[cfg(target_endian = "big")]
        pub fn mem_copy_and_switch_endianness(dst: *mut c_void, src: *mut c_void, num_elements: usize, element_size: usize) {
            macro_rules! copy_loop {
                ($type:ty, $reverse:ident) => {
                    unsafe {
                        for i in 0..num_elements {
                            let mut t: $type = 0;
                            let s = (src as *mut $type).add(i);
                            let d = (dst as *mut $type).add(i);
                            ptr::copy_nonoverlapping(s as *const c_void, &mut t as *mut $type as *mut c_void, element_size);
                            t = $reverse(t);
                            ptr::copy_nonoverlapping(&t as *const $type as *const c_void, d as *mut c_void, element_size);
                        }
                        return;
                    }
                };
            }

            match element_size {
                1 => {
                    mem_copy(dst, src, num_elements);
                    return;
                }
                2 => {
                    copy_loop!(u16, u16::swap_bytes);
                }
                4 => {
                    copy_loop!(u32, u32::swap_bytes);
                }
                8 => {
                    copy_loop!(u64, u64::swap_bytes);
                }
                _ => {
                    panic!("unreachable");
                }
            }
        }

        #[inline]
        pub fn mem_move(dest: *mut c_void, src: *const c_void, size: usize) {
            unsafe {
                match size {
                    1 => { *(dest as *mut u8) = *(src as *const u8); }
                    2 => { *(dest as *mut u16) = *(src as *const u16); }
                    3 => {
                        let dest = dest as *mut u8;
                        let src = src as *const u8;
                        ptr::copy(src, dest, 3);
                    }
                    4 => { *(dest as *mut u32) = *(src as *const u32); }
                    5 => {
                        let dest = dest as *mut u8;
                        let src = src as *const u8;
                        ptr::copy(src, dest, 5);
                    }
                    6 => {
                        let dest = dest as *mut u8;
                        let src = src as *const u8;
                        ptr::copy(src, dest, 6);
                    }
                    7 => {
                        let dest = dest as *mut u8;
                        let src = src as *const u8;
                        ptr::copy(src, dest, 7);
                    }
                    8 => { *(dest as *mut u64) = *(src as *const u64); }
                    9 => {
                        let dest = dest as *mut u8;
                        let src = src as *const u8;
                        ptr::copy(src, dest, 9);
                    }
                    10 => {
                        let dest = dest as *mut u8;
                        let src = src as *const u8;
                        ptr::copy(src, dest, 10);
                    }
                    11 => {
                        let dest = dest as *mut u8;
                        let src = src as *const u8;
                        ptr::copy(src, dest, 11);
                    }
                    12 => {
                        let dest = dest as *mut u8;
                        let src = src as *const u8;
                        ptr::copy(src, dest, 12);
                    }
                    13 => {
                        let dest = dest as *mut u8;
                        let src = src as *const u8;
                        ptr::copy(src, dest, 13);
                    }
                    14 => {
                        let dest = dest as *mut u8;
                        let src = src as *const u8;
                        ptr::copy(src, dest, 14);
                    }
                    15 => {
                        let dest = dest as *mut u8;
                        let src = src as *const u8;
                        ptr::copy(src, dest, 15);
                    }
                    16 => {
                        let dest = dest as *mut u8;
                        let src = src as *const u8;
                        ptr::copy(src, dest, 16);
                    }
                    _ => {
                        ptr::copy(src as *const u8, dest as *mut u8, size);
                    }
                }
            }
        }

        pub const K_MIN_COMPLEX_MEM_COPY: usize = 8;
    }

    #[cfg(any(target_arch = "x86", target_arch = "arm"))]
    use self::ia32::mem_copy;
    #[cfg(not(any(target_arch = "x86", target_arch = "arm")))]
    use self::generic::mem_copy;

    // Copies words from |src| to |dst|. The data spans must not overlap.
    // |src| and |dst| must be TWord-size aligned.
    #[inline]
    pub fn copy_impl<const K_BLOCK_COPY_LIMIT: usize, T>(dst_ptr: *mut T, src_ptr: *const T, count: usize) {
        let kt_word_size = mem::size_of::<T>();

        if count == 0 {
            return;
        }

        // Use block copying MemCopy if the segment we're copying is
        // enough to justify the extra call/setup overhead.
        if count < K_BLOCK_COPY_LIMIT {
            let mut dst_ptr = dst_ptr;
            let mut src_ptr = src_ptr;
            for _ in 0..count {
                unsafe {
                    *dst_ptr = *src_ptr;
                    dst_ptr = dst_ptr.add(1);
                    src_ptr = src_ptr.add(1);
                }
            }
        } else {
            mem_copy(
                dst_ptr as *mut c_void,
                src_ptr as *const c_void,
                count * kt_word_size,
            );
        }
    }

    // Copies kSystemPointerSize-sized words from |src| to |dst|. The data spans
    // must not overlap. |src| and |dst| must be kSystemPointerSize-aligned.
    #[inline]
    pub fn copy_words(dst: Address, src: Address, num_words: usize) {
        const K_BLOCK_COPY_LIMIT: usize = 16;
        copy_impl::<K_BLOCK_COPY_LIMIT, Address>(
            dst as *mut Address,
            src as *const Address,
            num_words,
        );
    }

    // Copies data from |src| to |dst|.  The data spans must not overlap.
    #[inline]
    pub fn copy_bytes<T>(dst: *mut T, src: *const T, num_bytes: usize) {
        assert_eq!(mem::size_of::<T>(), 1);
        if num_bytes == 0 {
            return;
        }
        #[cfg(any(target_arch = "x86", target_arch = "arm"))]
        use self::ia32::K_MIN_COMPLEX_MEM_COPY;
        #[cfg(not(any(target_arch = "x86", target_arch = "arm")))]
        use self::generic::K_MIN_COMPLEX_MEM_COPY;
        copy_impl::<{ K_MIN_COMPLEX_MEM_COPY }, T>(dst, src, num_bytes);
    }

    pub fn memset_uint32(dest: *mut u32, value: u32, counter: usize) {
        let mut i = 0;
        while i < counter {
            unsafe {
                *dest.add(i) = value;
            }
            i += 1;
        }
    }

    pub fn memset_pointer(dest: *mut Address, value: Address, counter: usize) {
        let mut i = 0;
        while i < counter {
            unsafe {
                *dest.add(i) = value;
            }
            i += 1;
        }
    }

    pub fn memset_pointer_generic<T, U>(dest: *mut *mut T, value: *mut U, counter: usize) {
        let value_address = value as Address;
        memset_pointer(dest as *mut Address, value_address, counter);
    }

    pub fn memset_pointer_null<T>(dest: *mut *mut T, counter: usize) {
        memset_pointer(dest as *mut Address, 0, counter);
    }

    // Copy from 8bit/16bit chars to 8bit/16bit chars. Values are zero-extended if
    // needed. Ranges are not allowed to overlap.
    pub fn copy_chars<SrcType, DstType>(dst: *mut DstType, src: *const SrcType, count: usize)
        where
            SrcType: std::fmt::Debug + Copy + std::default::Default,
            DstType: std::fmt::Debug + Copy + std::default::Default,
    {
        use std::mem::size_of;

        if count == 0 {
            return;
        }

        #[cfg(all(feature = "v8_optimize_with_neon", target_arch = "aarch64"))]
        {
            if size_of::<DstType>() == 1 && size_of::<SrcType>() == 1 {
                // Use simd optimized memcpy.
                mem_copy(dst as *mut c_void, src as *const c_void, count);
                return;
            }
        }

        let src_slice = unsafe { std::slice::from_raw_parts(src, count) };
        let dst_slice = unsafe { std::slice::from_raw_parts_mut(dst, count) };

        for i in 0..count {
            dst_slice[i] = unsafe { std::mem::transmute_copy(&src_slice[i]) };
        }

    }
}