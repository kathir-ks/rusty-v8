// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod internal {
    use std::sync::atomic::{AtomicU32, AtomicPtr, Ordering};
    use std::{mem, ptr};

    pub enum WriteBarrierSlotType {
        kCompressed,
        kUncompressed,
    }

    #[cfg(feature = "cppgc_pointer_compression")]
    pub mod pointer_compression {
        use super::*;
        // Note: CPPGC_CONST and CPPGC_REQUIRE_CONSTANT_INIT are handled through const evaluation in Rust.
        //  No macro is needed.

        const K_LOWER_HALF_WORD_MASK: usize =
            (super::api_constants::K_CAGED_HEAP_RESERVATION_ALIGNMENT - 1) as usize;

        #[repr(align(64))] // api_constants::kCachelineSize
        struct Base {
            base: usize,
            cache_line: [u8; super::api_constants::K_CACHELINE_SIZE as usize],
        }

        impl Base {
            const fn new() -> Self {
                Self {
                    base: 0,
                    cache_line: [0; super::api_constants::K_CACHELINE_SIZE as usize],
                }
            }
        }

        static mut G_BASE: Base = Base::new();

        pub struct CageBaseGlobal {}

        impl CageBaseGlobal {
            #[inline]
            pub fn get() -> usize {
                debug_assert!(Self::is_base_consistent());
                unsafe { G_BASE.base }
            }

            #[inline]
            pub fn is_set() -> bool {
                debug_assert!(Self::is_base_consistent());
                (unsafe { G_BASE.base } & !K_LOWER_HALF_WORD_MASK) != 0
            }

            #[inline]
            fn is_base_consistent() -> bool {
                unsafe { K_LOWER_HALF_WORD_MASK == (G_BASE.base & K_LOWER_HALF_WORD_MASK) }
            }
        }

        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
        #[repr(transparent)]
        pub struct CompressedPointer {
            value_: u32,
        }

        impl CompressedPointer {
            pub struct AtomicInitializerTag {}

            pub type IntegralType = u32;
            pub const K_WRITE_BARRIER_SLOT_TYPE: WriteBarrierSlotType =
                WriteBarrierSlotType::kCompressed;

            #[inline]
            pub fn new() -> Self {
                Self { value_: 0u32 }
            }

            #[inline]
            pub fn from_ptr_atomic(value: *const std::ffi::c_void, _tag: AtomicInitializerTag) -> Self {
                let mut compressed = Self::new();
                compressed.store_atomic(value);
                compressed
            }

            #[inline]
            pub fn from_ptr(ptr: *const std::ffi::c_void) -> Self {
                Self {
                    value_: Self::compress(ptr),
                }
            }

            #[inline]
            pub fn from_nullptr() -> Self {
                Self { value_: 0u32 }
            }

            #[inline]
            pub fn from_sentinel() -> Self {
                Self {
                    value_: K_COMPRESSED_SENTINEL,
                }
            }

            #[inline]
            pub fn load(&self) -> *const std::ffi::c_void {
                Self::decompress(self.value_)
            }

            #[inline]
            pub fn load_atomic(&self) -> *const std::ffi::c_void {
                let atomic_value = unsafe {
                    (self as *const Self as *const AtomicU32).as_ref().unwrap().load(Ordering::Relaxed)
                };
                Self::decompress(atomic_value)
            }

            #[inline]
            pub fn store(&mut self, ptr: *const std::ffi::c_void) {
                self.value_ = Self::compress(ptr);
            }

            #[inline]
            pub fn store_atomic(&mut self, value: *const std::ffi::c_void) {
                let atomic_value = unsafe {
                    (self as *mut Self as *mut AtomicU32).as_mut().unwrap()
                };
                atomic_value.store(Self::compress(value), Ordering::Relaxed);
            }

            #[inline]
            pub fn clear(&mut self) {
                self.value_ = 0u32;
            }

            #[inline]
            pub fn is_cleared(&self) -> bool {
                self.value_ == 0u32
            }

            #[inline]
            pub fn is_sentinel(&self) -> bool {
                self.value_ == K_COMPRESSED_SENTINEL
            }

            #[inline]
            pub fn get_as_integer(&self) -> u32 {
                self.value_
            }

            const K_GIGA_CAGE_MASK: usize =
                !(super::api_constants::K_CAGED_HEAP_RESERVATION_ALIGNMENT - 1) as usize;
            const K_POINTER_COMPRESSION_SHIFT_MASK: usize =
                (1 << super::api_constants::K_POINTER_COMPRESSION_SHIFT) - 1;

            #[inline]
            fn compress(ptr: *const std::ffi::c_void) -> u32 {
                debug_assert!(
                    super::sentinel_pointer::KSENTINEL_VALUE ==
                        1 << super::api_constants::K_POINTER_COMPRESSION_SHIFT,
                    "The compression scheme relies on the sentinel encoded as 1 << kPointerCompressionShift"
                );

                debug_assert!(CageBaseGlobal::is_set());
                let base = CageBaseGlobal::get();
                debug_assert!(
                    ptr.is_null()
                        || ptr as usize == super::sentinel_pointer::KSENTINEL_POINTER as usize
                        || (base & Self::K_GIGA_CAGE_MASK)
                            == (ptr as usize & Self::K_GIGA_CAGE_MASK)
                );
                debug_assert!(
                    (ptr as usize & Self::K_POINTER_COMPRESSION_SHIFT_MASK) == 0
                );

                let uptr = ptr as usize;
                // Shift the pointer and truncate.
                let compressed =
                    (uptr >> super::api_constants::K_POINTER_COMPRESSION_SHIFT) as u32;
                // Normal compressed pointers must have the MSB set. This is guaranteed by
                // the cage alignment.
                debug_assert!(
                    compressed == 0
                        || compressed == K_COMPRESSED_SENTINEL
                        || (compressed & (1 << 31)) != 0
                );
                compressed
            }

            #[inline]
            fn decompress(ptr: u32) -> *const std::ffi::c_void {
                debug_assert!(CageBaseGlobal::is_set());
                let base = CageBaseGlobal::get();
                Self::decompress_with_base(ptr, base)
            }

            #[inline]
            fn decompress_with_base(ptr: u32, base: usize) -> *const std::ffi::c_void {
                debug_assert!(CageBaseGlobal::is_set());
                debug_assert!(base == CageBaseGlobal::get());
                // Sign-extend compressed pointer to full width. This ensure that normal
                // pointers have only 1s in the base part of the address. It's also
                // important to shift the unsigned value, as otherwise it would result in
                // undefined behavior.
                let mask = (ptr as i32 as i64 as u64) << super::api_constants::K_POINTER_COMPRESSION_SHIFT;
                // Set the base part of the address for normal compressed pointers. Note
                // that nullptr and the sentinel value do not have 1s in the base part and
                // remain as-is in this operation.
                (mask & base as u64) as *const std::ffi::c_void
            }

            // For a given memory `address`, this method iterates all possible pointers
            // that can be reasonably recovered with the current compression scheme and
            // passes them to `callback`.
            #[inline]
            pub fn visit_possible_pointers<F>(address: *const std::ffi::c_void, callback: F)
            where
                F: Fn(*const std::ffi::c_void),
            {
                let base = CageBaseGlobal::get();
                debug_assert!(base != 0);
                // We may have random compressed pointers on stack (e.g. due to inlined
                // collections). These could be present in both halfwords.
                let compressed_low = (address as usize) as u32;
                callback(Self::decompress_with_base(compressed_low, base));
                let compressed_high = ((address as usize) >> (mem::size_of::<u32>() * 8)) as u32;
                callback(Self::decompress_with_base(compressed_high, base));
                // Iterate possible intermediate values, see `Decompress()`. The intermediate
                // value of decompressing is a 64-bit value where 35 bits are the offset. We
                // don't assume sign extension is stored and recover that part.
                //
                // Note that this case conveniently also recovers the full pointer.
                const K_BIT_FOR_INTERMEDIATE_VALUE: usize =
                    (mem::size_of::<u32>() * 8) + super::api_constants::K_POINTER_COMPRESSION_SHIFT;
                const K_SIGN_EXTENSION_MASK: usize =
                    !((1 as usize) << K_BIT_FOR_INTERMEDIATE_VALUE).wrapping_sub(1);
                let intermediate_sign_extended = (address as usize) | K_SIGN_EXTENSION_MASK;
                callback((intermediate_sign_extended & base) as *const std::ffi::c_void);
            }
        }

        const K_COMPRESSED_SENTINEL: u32 =
            (super::sentinel_pointer::KSENTINEL_VALUE >> super::api_constants::K_POINTER_COMPRESSION_SHIFT) as u32;

    }

    pub mod raw_pointer {
        use super::*;

        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
        #[repr(transparent)]
        pub struct RawPointer {
            ptr_: *const std::ffi::c_void,
        }

        impl RawPointer {
            pub struct AtomicInitializerTag {}

            pub type IntegralType = usize;
            pub const K_WRITE_BARRIER_SLOT_TYPE: WriteBarrierSlotType =
                WriteBarrierSlotType::kUncompressed;

            #[inline]
            pub fn new() -> Self {
                Self { ptr_: ptr::null() }
            }

            #[inline]
            pub fn from_ptr_atomic(ptr: *const std::ffi::c_void, _tag: AtomicInitializerTag) -> Self {
                let mut raw = Self::new();
                raw.store_atomic(ptr);
                raw
            }

            #[inline]
            pub fn from_ptr(ptr: *const std::ffi::c_void) -> Self {
                Self { ptr_: ptr }
            }

            #[inline]
            pub fn load(&self) -> *const std::ffi::c_void {
                self.ptr_
            }

            #[inline]
            pub fn load_atomic(&self) -> *const std::ffi::c_void {
                unsafe {
                    (self as *const Self as *const AtomicPtr<std::ffi::c_void>).as_ref().unwrap().load(Ordering::Relaxed) as *const std::ffi::c_void
                }
            }

            #[inline]
            pub fn store(&mut self, ptr: *const std::ffi::c_void) {
                self.ptr_ = ptr;
            }

            #[inline]
            pub fn store_atomic(&mut self, ptr: *const std::ffi::c_void) {
                 let atomic_ptr = unsafe {
                    (self as *mut Self as *mut AtomicPtr<std::ffi::c_void>).as_mut().unwrap()
                 };
                 atomic_ptr.store(ptr as *mut std::ffi::c_void, Ordering::Relaxed);
            }

            #[inline]
            pub fn clear(&mut self) {
                self.ptr_ = ptr::null();
            }

            #[inline]
            pub fn is_cleared(&self) -> bool {
                self.ptr_.is_null()
            }

            #[inline]
            pub fn is_sentinel(&self) -> bool {
                self.ptr_ as usize == super::sentinel_pointer::KSENTINEL_POINTER as usize
            }

            #[inline]
            pub fn get_as_integer(&self) -> usize {
                self.ptr_ as usize
            }

            #[inline]
            pub fn visit_possible_pointers<F>(address: *const std::ffi::c_void, callback: F)
            where
                F: Fn(*const std::ffi::c_void),
            {
                // Pass along the full pointer.
                callback(address as *mut std::ffi::c_void);
            }
        }
    }

    #[cfg(feature = "cppgc_pointer_compression")]
    pub type DefaultMemberStorage = pointer_compression::CompressedPointer;
    #[cfg(not(feature = "cppgc_pointer_compression"))]
    pub type DefaultMemberStorage = raw_pointer::RawPointer;
}

pub mod api_constants {
    pub const K_CACHELINE_SIZE: i32 = 64;
    pub const K_CAGED_HEAP_RESERVATION_ALIGNMENT: i32 = 1 << 30;
    pub const K_POINTER_COMPRESSION_SHIFT: usize = 12;
}

pub mod sentinel_pointer {
    pub const KSENTINEL_VALUE: usize = 0x4000;
    pub const KSENTINEL_POINTER: *const std::ffi::c_void = KSENTINEL_VALUE as *const std::ffi::c_void;
}