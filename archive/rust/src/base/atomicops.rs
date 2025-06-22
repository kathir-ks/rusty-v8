// Copyright 2010 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// The routines exported by this module are subtle.  If you use them, even if
// you get the code right, it will depend on careful reasoning about atomicity
// and memory ordering; it will be less readable, and harder to maintain.  If
// you plan to use these routines, you should have a good reason, such as solid
// evidence that performance would otherwise suffer, or there being no
// alternative.  You should assume only properties explicitly guaranteed by the
// specifications in this file.  You are almost certainly _not_ writing code
// just for the x86; if you assume x86 semantics, x86 hardware bugs and
// implementations on other archtectures will cause your code to break.  If you
// do not know what you are doing, avoid these routines, and use a Mutex.
//
// It is incorrect to make direct assignments to/from an atomic variable.
// You should use one of the Load or Store routines.  The Relaxed  versions
// are provided when no fences are needed:
//   Relaxed_Store()
//   Relaxed_Load()
// Although there are currently no compiler enforcement, you are encouraged
// to use these.

use std::sync::atomic::{
    AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicPtr, Ordering,
};
use std::{cmp, mem, ptr};

// Placeholder for base-export.h and build_config.h macros
// In a real conversion, these would be replaced with appropriate Rust features,
// conditional compilation, and platform-specific code.
// For example:
// #[cfg(target_os = "linux")]
// const IS_LINUX: bool = true;
// #[cfg(not(target_os = "linux"))]
// const IS_LINUX: bool = false;

#[allow(dead_code)]
mod base {
    // Placeholder for V8_EXPORT macro
    // Replace with #[no_mangle] or pub where necessary
    // Example: #[no_mangle] pub extern "C" fn ...
}

#[allow(dead_code)]
mod macros {
    macro_rules! unreachable {
        () => {
            panic!("UNREACHABLE");
        };
    }

    #[allow(unused_macros)]
    macro_rules! use_expr {
        ($e:expr) => {
            let _ = $e;
        };
    }
}

// Placeholder for starboard atomic definitions
// #[cfg(V8_OS_STARBOARD)]
// mod starboard {
//     pub type Atomic8 = AtomicI8;
//     pub type Atomic32 = AtomicI32;
//     #[cfg(SB_IS_64_BIT)]
//     pub type Atomic64 = AtomicI64;
// }

pub mod v8 {
    pub mod base {
        #[cfg(not(V8_OS_STARBOARD))]
        pub type Atomic8 = i8;
        #[cfg(not(V8_OS_STARBOARD))]
        pub type Atomic16 = i16;
        #[cfg(not(V8_OS_STARBOARD))]
        pub type Atomic32 = i32;

        #[cfg(target_arch = "x86_64")]
        #[cfg(not(V8_OS_STARBOARD))]
        #[cfg(not(__ILP32__))]
        pub type Atomic64 = isize; // Assuming intptr_t is equivalent to isize

        #[cfg(target_arch = "x86_64")]
        #[cfg(not(V8_OS_STARBOARD))]
        #[cfg(__ILP32__)]
        pub type Atomic64 = i64;

        #[cfg(target_arch = "x86_64")]
        pub type AtomicWord = Atomic64;

        #[cfg(not(target_arch = "x86_64"))]
        #[cfg(not(V8_OS_STARBOARD))]
        pub type AtomicWord = Atomic32;

        #[cfg(not(target_arch = "x86_64"))]
        pub type Atomic64 = i64;

        pub(crate) mod helper {
            use std::sync::atomic::{AtomicI8, AtomicI16, AtomicI32, AtomicI64, Ordering};

            #[cfg(not(target_arch = "x86_64"))]
            pub(crate) fn to_std_atomic(ptr: *mut i32) -> *mut AtomicI32 {
                ptr as *mut AtomicI32
            }

            #[cfg(target_arch = "x86_64")]
            pub(crate) fn to_std_atomic(ptr: *mut isize) -> *mut AtomicI64 {
                ptr as *mut AtomicI64
            }

            #[cfg(not(target_arch = "x86_64"))]
            pub(crate) fn to_std_atomic_const(ptr: *const i32) -> *const AtomicI32 {
                ptr as *const AtomicI32
            }

            #[cfg(target_arch = "x86_64")]
            pub(crate) fn to_std_atomic_const(ptr: *const isize) -> *const AtomicI64 {
                ptr as *const AtomicI64
            }
        }

        /// Performs a sequentially consistent memory fence.
        #[inline]
        pub fn seq_cst_memory_fence() {
            std::sync::atomic::fence(Ordering::SeqCst);
        }

        /// Atomically compares the value at the pointer with `old_value` and, if they are
        /// equal, replaces the value at the pointer with `new_value`.
        ///
        /// Returns the old value at the pointer, regardless of whether the swap occurred.
        #[inline]
        pub fn relaxed_compare_and_swap(
            ptr: *mut Atomic8,
            old_value: Atomic8,
            new_value: Atomic8,
        ) -> Atomic8 {
            unsafe {
                let atomic_ptr = ptr as *mut AtomicI8;
                let atomic_ref = &mut *atomic_ptr;
                atomic_ref.compare_exchange(
                    old_value,
                    new_value,
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                ).unwrap_or(old_value)
            }
        }

        /// Atomically compares the value at the pointer with `old_value` and, if they are
        /// equal, replaces the value at the pointer with `new_value`.
        ///
        /// Returns the old value at the pointer, regardless of whether the swap occurred.
        #[inline]
        pub fn relaxed_compare_and_swap_i16(
            ptr: *mut Atomic16,
            old_value: Atomic16,
            new_value: Atomic16,
        ) -> Atomic16 {
            unsafe {
                let atomic_ptr = ptr as *mut AtomicI16;
                let atomic_ref = &mut *atomic_ptr;
                atomic_ref.compare_exchange(
                    old_value,
                    new_value,
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                ).unwrap_or(old_value)
            }
        }

        /// Atomically compares the value at the pointer with `old_value` and, if they are
        /// equal, replaces the value at the pointer with `new_value`.
        ///
        /// Returns the old value at the pointer, regardless of whether the swap occurred.
        #[inline]
        pub fn relaxed_compare_and_swap_i32(
            ptr: *mut Atomic32,
            old_value: Atomic32,
            new_value: Atomic32,
        ) -> Atomic32 {
            unsafe {
                let atomic_ptr = ptr as *mut AtomicI32;
                let atomic_ref = &mut *atomic_ptr;
                atomic_ref.compare_exchange(
                    old_value,
                    new_value,
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                ).unwrap_or(old_value)
            }
        }

        /// Atomically exchanges the value at the pointer with `new_value`.
        ///
        /// Returns the old value at the pointer.
        #[inline]
        pub fn relaxed_atomic_exchange_i32(
            ptr: *mut Atomic32,
            new_value: Atomic32,
        ) -> Atomic32 {
            unsafe {
                let atomic_ptr = ptr as *mut AtomicI32;
                let atomic_ref = &mut *atomic_ptr;
                atomic_ref.swap(new_value, Ordering::Relaxed)
            }
        }

        /// Atomically exchanges the value at the pointer with `new_value`.
        ///
        /// Returns the old value at the pointer.
        #[inline]
        pub fn seq_cst_atomic_exchange_i32(
            ptr: *mut Atomic32,
            new_value: Atomic32,
        ) -> Atomic32 {
            unsafe {
                let atomic_ptr = ptr as *mut AtomicI32;
                let atomic_ref = &mut *atomic_ptr;
                atomic_ref.swap(new_value, Ordering::SeqCst)
            }
        }

        /// Atomically increments the value at the pointer by `increment`.
        ///
        /// Returns the old value at the pointer plus increment.
        #[inline]
        pub fn relaxed_atomic_increment_i32(
            ptr: *mut Atomic32,
            increment: Atomic32,
        ) -> Atomic32 {
            unsafe {
                let atomic_ptr = ptr as *mut AtomicI32;
                let atomic_ref = &mut *atomic_ptr;
                atomic_ref.fetch_add(increment, Ordering::Relaxed) + increment
            }
        }

        /// Atomically compares the value at the pointer with `old_value` and, if they are
        /// equal, replaces the value at the pointer with `new_value`.
        ///
        /// Returns the old value at the pointer, regardless of whether the swap occurred.
        #[inline]
        pub fn acquire_compare_and_swap_i32(
            ptr: *mut Atomic32,
            old_value: Atomic32,
            new_value: Atomic32,
        ) -> Atomic32 {
            unsafe {
                let atomic_ptr = ptr as *mut AtomicI32;
                let atomic_ref = &mut *atomic_ptr;
                atomic_ref.compare_exchange(
                    old_value,
                    new_value,
                    Ordering::Acquire,
                    Ordering::Acquire,
                ).unwrap_or(old_value)
            }
        }

        /// Atomically compares the value at the pointer with `old_value` and, if they are
        /// equal, replaces the value at the pointer with `new_value`.
        ///
        /// Returns the old value at the pointer, regardless of whether the swap occurred.
        #[inline]
        pub fn release_compare_and_swap(
            ptr: *mut Atomic8,
            old_value: Atomic8,
            new_value: Atomic8,
        ) -> Atomic8 {
            unsafe {
                let atomic_ptr = ptr as *mut AtomicI8;
                let atomic_ref = &mut *atomic_ptr;
                let _result = atomic_ref.compare_exchange(
                    old_value,
                    new_value,
                    Ordering::Release,
                    Ordering::Relaxed,
                );
                // macros::use_expr!(_result);
                old_value
            }
        }

        /// Atomically compares the value at the pointer with `old_value` and, if they are
        /// equal, replaces the value at the pointer with `new_value`.
        ///
        /// Returns the old value at the pointer, regardless of whether the swap occurred.
        #[inline]
        pub fn release_compare_and_swap_i32(
            ptr: *mut Atomic32,
            old_value: Atomic32,
            new_value: Atomic32,
        ) -> Atomic32 {
            unsafe {
                let atomic_ptr = ptr as *mut AtomicI32;
                let atomic_ref = &mut *atomic_ptr;
                atomic_ref.compare_exchange(
                    old_value,
                    new_value,
                    Ordering::Release,
                    Ordering::Relaxed,
                ).unwrap_or(old_value)
            }
        }

        /// Atomically compares the value at the pointer with `old_value` and, if they are
        /// equal, replaces the value at the pointer with `new_value`.
        ///
        /// Returns the old value at the pointer, regardless of whether the swap occurred.
        #[inline]
        pub fn acquire_release_compare_and_swap_i32(
            ptr: *mut Atomic32,
            old_value: Atomic32,
            new_value: Atomic32,
        ) -> Atomic32 {
            unsafe {
                let atomic_ptr = ptr as *mut AtomicI32;
                let atomic_ref = &mut *atomic_ptr;
                atomic_ref.compare_exchange(
                    old_value,
                    new_value,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                ).unwrap_or(old_value)
            }
        }

        /// Atomically compares the value at the pointer with `old_value` and, if they are
        /// equal, replaces the value at the pointer with `new_value`.
        ///
        /// Returns the old value at the pointer, regardless of whether the swap occurred.
        #[inline]
        pub fn seq_cst_compare_and_swap_i32(
            ptr: *mut Atomic32,
            old_value: Atomic32,
            new_value: Atomic32,
        ) -> Atomic32 {
            unsafe {
                let atomic_ptr = ptr as *mut AtomicI32;
                let atomic_ref = &mut *atomic_ptr;
                atomic_ref.compare_exchange(
                    old_value,
                    new_value,
                    Ordering::SeqCst,
                    Ordering::SeqCst,
                ).unwrap_or(old_value)
            }
        }

        /// Atomically stores `value` at the pointer, using `Relaxed` ordering.
        #[inline]
        pub fn relaxed_store(ptr: *mut Atomic8, value: Atomic8) {
            unsafe {
                let atomic_ptr = ptr as *mut AtomicI8;
                let atomic_ref = &mut *atomic_ptr;
                atomic_ref.store(value, Ordering::Relaxed);
            }
        }

        /// Atomically stores `value` at the pointer, using `Relaxed` ordering.
        #[inline]
        pub fn relaxed_store_i16(ptr: *mut Atomic16, value: Atomic16) {
            unsafe {
                let atomic_ptr = ptr as *mut AtomicI16;
                let atomic_ref = &mut *atomic_ptr;
                atomic_ref.store(value, Ordering::Relaxed);
            }
        }

        /// Atomically stores `value` at the pointer, using `Relaxed` ordering.
        #[inline]
        pub fn relaxed_store_i32(ptr: *mut Atomic32, value: Atomic32) {
            unsafe {
                let atomic_ptr = ptr as *mut AtomicI32;
                let atomic_ref = &mut *atomic_ptr;
                atomic_ref.store(value, Ordering::Relaxed);
            }
        }

        /// Atomically stores `value` at the pointer, using `Release` ordering.
        #[inline]
        pub fn release_store(ptr: *mut Atomic8, value: Atomic8) {
            unsafe {
                let atomic_ptr = ptr as *mut AtomicI8;
                let atomic_ref = &mut *atomic_ptr;
                atomic_ref.store(value, Ordering::Release);
            }
        }

        /// Atomically stores `value` at the pointer, using `Release` ordering.
        #[inline]
        pub fn release_store_i16(ptr: *mut Atomic16, value: Atomic16) {
            unsafe {
                let atomic_ptr = ptr as *mut AtomicI16;
                let atomic_ref = &mut *atomic_ptr;
                atomic_ref.store(value, Ordering::Release);
            }
        }

        /// Atomically stores `value` at the pointer, using `Release` ordering.
        #[inline]
        pub fn release_store_i32(ptr: *mut Atomic32, value: Atomic32) {
            unsafe {
                let atomic_ptr = ptr as *mut AtomicI32;
                let atomic_ref = &mut *atomic_ptr;
                atomic_ref.store(value, Ordering::Release);
            }
        }

        /// Atomically stores `value` at the pointer, using `SeqCst` ordering.
        #[inline]
        pub fn seq_cst_store(ptr: *mut Atomic8, value: Atomic8) {
            unsafe {
                let atomic_ptr = ptr as *mut AtomicI8;
                let atomic_ref = &mut *atomic_ptr;
                atomic_ref.store(value, Ordering::SeqCst);
            }
        }

        /// Atomically stores `value` at the pointer, using `SeqCst` ordering.
        #[inline]
        pub fn seq_cst_store_i16(ptr: *mut Atomic16, value: Atomic16) {
            unsafe {
                let atomic_ptr = ptr as *mut AtomicI16;
                let atomic_ref = &mut *atomic_ptr;
                atomic_ref.store(value, Ordering::SeqCst);
            }
        }

        /// Atomically stores `value` at the pointer, using `SeqCst` ordering.
        #[inline]
        pub fn seq_cst_store_i32(ptr: *mut Atomic32, value: Atomic32) {
            unsafe {
                let atomic_ptr = ptr as *mut AtomicI32;
                let atomic_ref = &mut *atomic_ptr;
                atomic_ref.store(value, Ordering::SeqCst);
            }
        }

        /// Atomically loads the value at the pointer, using `Relaxed` ordering.
        #[inline]
        pub fn relaxed_load(ptr: *const Atomic8) -> Atomic8 {
            unsafe {
                let atomic_ptr = ptr as *const AtomicI8;
                let atomic_ref = &*atomic_ptr;
                atomic_ref.load(Ordering::Relaxed)
            }
        }

        /// Atomically loads the value at the pointer, using `Relaxed` ordering.
        #[inline]
        pub fn relaxed_load_i16(ptr: *const Atomic16) -> Atomic16 {
            unsafe {
                let atomic_ptr = ptr as *const AtomicI16;
                let atomic_ref = &*atomic_ptr;
                atomic_ref.load(Ordering::Relaxed)
            }
        }

        /// Atomically loads the value at the pointer, using `Relaxed` ordering.
        #[inline]
        pub fn relaxed_load_i32(ptr: *const Atomic32) -> Atomic32 {
            unsafe {
                let atomic_ptr = ptr as *const AtomicI32;
                let atomic_ref = &*atomic_ptr;
                atomic_ref.load(Ordering::Relaxed)
            }
        }

        /// Atomically loads the value at the pointer, using `Acquire` ordering.
        #[inline]
        pub fn acquire_load(ptr: *const Atomic8) -> Atomic8 {
            unsafe {
                let atomic_ptr = ptr as *const AtomicI8;
                let atomic_ref = &*atomic_ptr;
                atomic_ref.load(Ordering::Acquire)
            }
        }

        /// Atomically loads the value at the pointer, using `Acquire` ordering.
        #[inline]
        pub fn acquire_load_i32(ptr: *const Atomic32) -> Atomic32 {
            unsafe {
                let atomic_ptr = ptr as *const AtomicI32;
                let atomic_ref = &*atomic_ptr;
                atomic_ref.load(Ordering::Acquire)
            }
        }

        /// Atomically loads the value at the pointer, using `SeqCst` ordering.
        #[inline]
        pub fn seq_cst_load(ptr: *const Atomic8) -> Atomic8 {
            unsafe {
                let atomic_ptr = ptr as *const AtomicI8;
                let atomic_ref = &*atomic_ptr;
                atomic_ref.load(Ordering::SeqCst)
            }
        }

        /// Atomically loads the value at the pointer, using `SeqCst` ordering.
        #[inline]
        pub fn seq_cst_load_i32(ptr: *const Atomic32) -> Atomic32 {
            unsafe {
                let atomic_ptr = ptr as *const AtomicI32;
                let atomic_ref = &*atomic_ptr;
                atomic_ref.load(Ordering::SeqCst)
            }
        }

        #[cfg(target_arch = "x86_64")]
        mod atomic64 {
            use super::{Atomic64, helper};
            use std::sync::atomic::{AtomicI64, Ordering};

            #[inline]
            pub fn relaxed_compare_and_swap(
                ptr: *mut Atomic64,
                old_value: Atomic64,
                new_value: Atomic64,
            ) -> Atomic64 {
                unsafe {
                    let atomic_ptr = ptr as *mut AtomicI64;
                    let atomic_ref = &mut *atomic_ptr;
                    atomic_ref.compare_exchange(
                        old_value,
                        new_value,
                        Ordering::Relaxed,
                        Ordering::Relaxed,
                    ).unwrap_or(old_value)
                }
            }

            #[inline]
            pub fn relaxed_atomic_exchange(
                ptr: *mut Atomic64,
                new_value: Atomic64,
            ) -> Atomic64 {
                unsafe {
                    let atomic_ptr = ptr as *mut AtomicI64;
                    let atomic_ref = &mut *atomic_ptr;
                    atomic_ref.swap(new_value, Ordering::Relaxed)
                }
            }

            #[inline]
            pub fn seq_cst_atomic_exchange(
                ptr: *mut Atomic64,
                new_value: Atomic64,
            ) -> Atomic64 {
                unsafe {
                    let atomic_ptr = ptr as *mut AtomicI64;
                    let atomic_ref = &mut *atomic_ptr;
                    atomic_ref.swap(new_value, Ordering::SeqCst)
                }
            }

            #[inline]
            pub fn relaxed_atomic_increment(
                ptr: *mut Atomic64,
                increment: Atomic64,
            ) -> Atomic64 {
                unsafe {
                    let atomic_ptr = ptr as *mut AtomicI64;
                    let atomic_ref = &mut *atomic_ptr;
                    atomic_ref.fetch_add(increment, Ordering::Relaxed) + increment
                }
            }

            #[inline]
            pub fn acquire_compare_and_swap(
                ptr: *mut Atomic64,
                old_value: Atomic64,
                new_value: Atomic64,
            ) -> Atomic64 {
                unsafe {
                    let atomic_ptr = ptr as *mut AtomicI64;
                    let atomic_ref = &mut *atomic_ptr;
                    atomic_ref.compare_exchange(
                        old_value,
                        new_value,
                        Ordering::Acquire,
                        Ordering::Acquire,
                    ).unwrap_or(old_value)
                }
            }

            #[inline]
            pub fn release_compare_and_swap(
                ptr: *mut Atomic64,
                old_value: Atomic64,
                new_value: Atomic64,
            ) -> Atomic64 {
                unsafe {
                    let atomic_ptr = ptr as *mut AtomicI64;
                    let atomic_ref = &mut *atomic_ptr;
                    atomic_ref.compare_exchange(
                        old_value,
                        new_value,
                        Ordering::Release,
                        Ordering::Relaxed,
                    ).unwrap_or(old_value)
                }
            }

            #[inline]
            pub fn acquire_release_compare_and_swap(
                ptr: *mut Atomic64,
                old_value: Atomic64,
                new_value: Atomic64,
            ) -> Atomic64 {
                unsafe {
                    let atomic_ptr = ptr as *mut AtomicI64;
                    let atomic_ref = &mut *atomic_ptr;
                    atomic_ref.compare_exchange(
                        old_value,
                        new_value,
                        Ordering::AcqRel,
                        Ordering::Acquire,
                    ).unwrap_or(old_value)
                }
            }

            #[inline]
            pub fn seq_cst_compare_and_swap(
                ptr: *mut Atomic64,
                old_value: Atomic64,
                new_value: Atomic64,
            ) -> Atomic64 {
                unsafe {
                    let atomic_ptr = ptr as *mut AtomicI64;
                    let atomic_ref = &mut *atomic_ptr;
                    atomic_ref.compare_exchange(
                        old_value,
                        new_value,
                        Ordering::SeqCst,
                        Ordering::SeqCst,
                    ).unwrap_or(old_value)
                }
            }

            #[inline]
            pub fn relaxed_store(ptr: *mut Atomic64, value: Atomic64) {
                unsafe {
                    let atomic_ptr = ptr as *mut AtomicI64;
                    let atomic_ref = &mut *atomic_ptr;
                    atomic_ref.store(value, Ordering::Relaxed);
                }
            }

            #[inline]
            pub fn release_store(ptr: *mut Atomic64, value: Atomic64) {
                unsafe {
                    let atomic_ptr = ptr as *mut AtomicI64;
                    let atomic_ref = &mut *atomic_ptr;
                    atomic_ref.store(value, Ordering::Release);
                }
            }

            #[inline]
            pub fn seq_cst_store(ptr: *mut Atomic64, value: Atomic64) {
                unsafe {
                    let atomic_ptr = ptr as *mut AtomicI64;
                    let atomic_ref = &mut *atomic_ptr;
                    atomic_ref.store(value, Ordering::SeqCst);
                }
            }

            #[inline]
            pub fn relaxed_load(ptr: *const Atomic64) -> Atomic64 {
                unsafe {
                    let atomic_ptr = ptr as *const AtomicI64;
                    let atomic_ref = &*atomic_ptr;
                    atomic_ref.load(Ordering::Relaxed)
                }
            }

            #[inline]
            pub fn acquire_load(ptr: *const Atomic64) -> Atomic64 {
                unsafe {
                    let atomic_ptr = ptr as *const AtomicI64;
                    let atomic_ref = &*atomic_ptr;
                    atomic_ref.load(Ordering::Acquire)
                }
            }

            #[inline]
            pub fn seq_cst_load(ptr: *const Atomic64) -> Atomic64 {
                unsafe {
                    let atomic_ptr = ptr as *const AtomicI64;
                    let atomic_ref = &*atomic_ptr;
                    atomic_ref.load(Ordering::SeqCst)
                }
            }
        }

        #[cfg(target_arch = "x86_64")]
        pub use atomic64::*;

        /// Copies `bytes` from `src` to `dst`, using `Relaxed` ordering for each individual
        /// load and store.
        #[inline]
        pub fn relaxed_memcpy(dst: *mut Atomic8, src: *const Atomic8, bytes: usize) {
            let k_atomic_word_size = mem::size_of::<AtomicWord>();
            let mut dst_ptr = dst;
            let mut src_ptr = src;
            let mut remaining_bytes = bytes;

            while remaining_bytes > 0
                && !is_aligned(dst_ptr as usize, k_atomic_word_size)
            {
                unsafe {
                    relaxed_store(dst_ptr, relaxed_load(src_ptr));
                    dst_ptr = dst_ptr.add(1);
                    src_ptr = src_ptr.add(1);
                }
                remaining_bytes -= 1;
            }

            if is_aligned(src_ptr as usize, k_atomic_word_size)
                && is_aligned(dst_ptr as usize, k_atomic_word_size)
            {
                while remaining_bytes >= k_atomic_word_size {
                    unsafe {
                        let dst_word_ptr = dst_ptr as *mut AtomicWord;
                        let src_word_ptr = src_ptr as *const AtomicWord;
                        relaxed_store(
                            dst_word_ptr as *mut Atomic8,
                            relaxed_load(src_word_ptr as *const Atomic8),
                        );
                        dst_ptr = dst_ptr.add(k_atomic_word_size);
                        src_ptr = src_ptr.add(k_atomic_word_size);
                    }
                    remaining_bytes -= k_atomic_word_size;
                }
            }

            while remaining_bytes > 0 {
                unsafe {
                    relaxed_store(dst_ptr, relaxed_load(src_ptr));
                    dst_ptr = dst_ptr.add(1);
                    src_ptr = src_ptr.add(1);
                }
                remaining_bytes -= 1;
            }
        }

        /// Moves `bytes` from `src` to `dst`, using `Relaxed` ordering for each individual
        /// load and store.  Handles overlapping memory regions correctly.
        #[inline]
        pub fn relaxed_memmove(dst: *mut Atomic8, src: *const Atomic8, bytes: usize) {
            if (dst as usize).wrapping_sub(src as usize) >= bytes {
                relaxed_memcpy(dst, src, bytes);
                return;
            }

            let mut dst_ptr = unsafe { dst.add(bytes) };
            let mut src_ptr = unsafe { src.add(bytes) };
            let mut remaining_bytes = bytes;

            let k_atomic_word_size = mem::size_of::<AtomicWord>();

            while remaining_bytes > 0
                && !is_aligned(dst_ptr as usize, k_atomic_word_size)
            {
                unsafe {
                    dst_ptr = dst_ptr.sub(1);
                    src_ptr = src_ptr.sub(1);
                    relaxed_store(dst_ptr, relaxed_load(src_ptr));
                }
                remaining_bytes -= 1;
            }

            if is_aligned(src_ptr as usize, k_atomic_word_size)
                && is_aligned(dst_ptr as usize, k_atomic_word_size)
            {
                while remaining_bytes >= k_atomic_word_size {
                    unsafe {
                        dst_ptr = dst_ptr.sub(k_atomic_word_size);
                        src_ptr = src_ptr.sub(k_atomic_word_size);

                        let dst_word_ptr = dst_ptr as *mut AtomicWord;
                        let src_word_ptr = src_ptr as *const AtomicWord;
                        relaxed_store(
                            dst_word_ptr as *mut Atomic8,
                            relaxed_load(src_word_ptr as *const Atomic8),
                        );
                    }
                    remaining_bytes -= k_atomic_word_size;
                }
            }

            while remaining_bytes > 0 {
                unsafe {
                    dst_ptr = dst_ptr.sub(1);
                    src_ptr = src_ptr.sub(1);
                    relaxed_store(dst_ptr, relaxed_load(src_ptr));
                }
                remaining_bytes -= 1;
            }
        }

        pub(crate) mod helper {
            use super::{Atomic8, AtomicWord};
            use std::cmp::Ordering;

            #[inline]
            pub fn memcmp_not_equal_fundamental(u1: Atomic8, u2: Atomic8) -> i32 {
                if u1 < u2 {
                    -1
                } else {
                    1
                }
            }

            #[inline]
            pub fn memcmp_not_equal_fundamental_word(u1: AtomicWord, u2: AtomicWord) -> i32 {
                if u1 < u2 {
                    -1
                } else {
                    1
                }
            }
        }

        /// Compares `len` bytes from `s1` to `s2`, using `Relaxed` ordering for each
        /// individual load.
        #[inline]
        pub fn relaxed_memcmp(
            s1: *const Atomic8,
            s2: *const Atomic8,
            len: usize,
        ) -> i32 {
            let k_atomic_word_size = mem::size_of::<AtomicWord>();
            let mut s1_ptr = s1;
            let mut s2_ptr = s2;
            let mut remaining_len = len;

            while remaining_len > 0
                && !(is_aligned(s1_ptr as usize, k_atomic_word_size)
                    && is_aligned(s2_ptr as usize, k_atomic_word_size))
            {
                unsafe {
                    let u1 = relaxed_load(s1_ptr);
                    s1_ptr = s1_ptr.add(1);
                    let u2 = relaxed_load(s2_ptr);
                    s2_ptr = s2_ptr.add(1);

                    if u1 != u2 {
                        return helper::memcmp_not_equal_fundamental(u1, u2);
                    }
                }
                remaining_len -= 1;
            }

            if is_aligned(s1_ptr as usize, k_atomic_word_size)
                && is_aligned(s2_ptr as usize, k_atomic_word_size)
            {
                while remaining_len >= k_atomic_word_