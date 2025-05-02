// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Collection of swiss table helpers that are independent from a specific
// container, like SwissNameDictionary. Taken almost in verbatim from Abseil,
// comments in this file indicate what is taken from what Abseil file.

//use std::arch::x86_64::*;
use std::mem;
use std::ops::{BitAnd, BitXor, Not, Shl, Shr, Sub};
use std::{convert::TryInto, u32, u64};

const V8_SWISS_TABLE_HAVE_SSE2_TARGET: bool = cfg!(any(target_arch = "x86", target_arch = "x86_64"));

// Assuming SSE2 and SSSE3 are enabled if the target architecture is x86 or x64
// The following are placeholders, and need to be configured during compilation based on target CPU capabilities
const V8_SWISS_TABLE_HAVE_SSE2_HOST: bool = cfg!(any(target_arch = "x86", target_arch = "x86_64"));
const V8_SWISS_TABLE_HAVE_SSSE3_HOST: bool = cfg!(any(target_arch = "x86", target_arch = "x86_64"));

// Placeholder for V8 target architecture definitions, will need to be configured at compilation.
const V8_TARGET_ARCH_IA32: bool = cfg!(target_arch = "x86");
const V8_TARGET_ARCH_X64: bool = cfg!(target_arch = "x86_64");
const V8_HOST_ARCH_IA32: bool = cfg!(target_arch = "x86");
const V8_HOST_ARCH_X64: bool = cfg!(target_arch = "x86_64");

pub mod swiss_table {
    use super::*;
    use std::ops::{BitAnd, BitXor, Not, Shl, Shr, Sub};

    /// Denotes the group of the control table currently being probed.
    /// Implements quadratic probing by advancing by i groups after the i-th
    /// (unsuccessful) probe.
    pub struct ProbeSequence<const GROUP_SIZE: usize> {
        mask_: u32,
        offset_: u32,
        index_: u32,
    }

    impl<const GROUP_SIZE: usize> ProbeSequence<GROUP_SIZE> {
        pub fn new(hash: u32, mask: u32) -> Self {
            // Mask must be a power of 2 minus 1.
            debug_assert_eq!(0, ((mask + 1) & mask));
            ProbeSequence {
                mask_: mask,
                offset_: hash & mask,
                index_: 0,
            }
        }
        pub fn offset(&self) -> u32 {
            self.offset_
        }
        pub fn offset_i(&self, i: i32) -> u32 {
            (self.offset_ + i as u32) & self.mask_
        }

        pub fn next(&mut self) {
            self.index_ += GROUP_SIZE as u32;
            self.offset_ += self.index_;
            self.offset_ &= self.mask_;
        }

        pub fn index(&self) -> u32 {
            self.index_
        }
    }

    /// An abstraction over a bitmask. It provides an easy way to iterate through the
    /// indexes of the set bits of a bitmask. When Shift=0 (platforms with SSE),
    /// this is a true bitmask.
    /// When Shift=3 (used on non-SSE platforms), we obtain a "byte mask", where each
    /// logical bit is represented by a full byte. The logical bit 0 is represented
    /// as 0x00, whereas 1 is represented as 0x80. Other values must not appear.
    ///
    /// For example:
    ///   for (int i : BitMask<uint32_t, 16>(0x5)) -> yields 0, 2
    ///   for (int i : BitMask<uint64_t, 8, 3>(0x0000000080800000)) -> yields 2, 3
    pub struct BitMask<T, const SIGNIFICANT_BITS: usize, const SHIFT: usize = 0>
    where
        T: std::ops::BitAnd<Output = T>
            + std::ops::BitXor<Output = T>
            + std::ops::Not<Output = T>
            + std::ops::Sub<Output = T>
            + std::marker::Copy
            + std::cmp::PartialEq
            + std::convert::TryInto<usize>,
    {
        mask_: T,
    }

    impl<T, const SIGNIFICANT_BITS: usize, const SHIFT: usize> BitMask<T, SIGNIFICANT_BITS, SHIFT>
    where
        T: std::ops::BitAnd<Output = T>
            + std::ops::BitXor<Output = T>
            + std::ops::Not<Output = T>
            + std::ops::Sub<Output = T>
            + std::marker::Copy
            + std::cmp::PartialEq
            + std::convert::TryInto<usize>,
    {
        pub fn new(mask: T) -> Self {
            //static_assert!(SHIFT == 0 || SHIFT == 3);
            BitMask { mask_: mask }
        }

        pub fn iter(&self) -> BitMaskIterator<'_, T, SIGNIFICANT_BITS, SHIFT> {
            BitMaskIterator { mask: *self }
        }

        pub fn lowest_bit_set(&self) -> usize
        where
            T: std::convert::TryInto<usize>,
        {
            self.trailing_zeros()
        }

        pub fn highest_bit_set(&self) -> usize
        where
            T: Shr<usize, Output = T>,
            usize: std::convert::From<u32>,
            T: std::convert::TryInto<usize>,
        {
            let mask_usize: usize = unsafe { std::mem::transmute_copy(&self.mask_) };
            let leading_zeros = mask_usize.leading_zeros() as usize;
            (std::mem::size_of::<T>() * 8 - leading_zeros - 1) >> SHIFT
        }

        fn trailing_zeros(&self) -> usize
        where
            T: std::convert::TryInto<usize>,
        {
            let mask_usize: usize = unsafe { std::mem::transmute_copy(&self.mask_) };
            debug_assert_ne!(mask_usize, 0);
            mask_usize.trailing_zeros() as usize >> SHIFT
        }

        #[allow(dead_code)]
        fn leading_zeros(&self) -> usize
        where
            T: Shl<usize, Output = T>,
            usize: From<u32>,
            T: std::convert::TryInto<usize>,
        {
            let total_significant_bits = SIGNIFICANT_BITS << SHIFT;
            let extra_bits = std::mem::size_of::<T>() * 8 - total_significant_bits;

            let mask_usize: usize = unsafe { std::mem::transmute_copy(&self.mask_) };
            let shifted_mask = mask_usize << extra_bits;

            (shifted_mask.leading_zeros() as usize) >> SHIFT
        }
    }

    pub struct BitMaskIterator<'a, T, const SIGNIFICANT_BITS: usize, const SHIFT: usize = 0>
    where
        T: std::ops::BitAnd<Output = T>
            + std::ops::BitXor<Output = T>
            + std::ops::Not<Output = T>
            + std::ops::Sub<Output = T>
            + std::marker::Copy
            + std::cmp::PartialEq
            + std::convert::TryInto<usize>,
    {
        mask: BitMask<T, SIGNIFICANT_BITS, SHIFT>,
    }

    impl<'a, T, const SIGNIFICANT_BITS: usize, const SHIFT: usize> Iterator
        for BitMaskIterator<'a, T, SIGNIFICANT_BITS, SHIFT>
    where
        T: std::ops::BitAnd<Output = T>
            + std::ops::BitXor<Output = T>
            + std::ops::Not<Output = T>
            + std::ops::Sub<Output = T>
            + std::marker::Copy
            + std::cmp::PartialEq
            + std::convert::TryInto<usize>,
    {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if unsafe { std::mem::transmute_copy::<T, usize>(&self.mask.mask_) } != 0 {
                let lsb = self.mask.lowest_bit_set();
                self.mask.mask_ = (self.mask.mask_ & (self.mask.mask_ - unsafe { std::mem::transmute::<i32, T>(1) }));
                Some(lsb)
            } else {
                None
            }
        }
    }

    #[allow(non_camel_case_types)]
    pub type ctrl_t = i8;
    #[allow(non_camel_case_types)]
    pub type h2_t = u8;

    /// The values here are selected for maximum performance. See the static asserts
    /// below for details.
    #[allow(non_camel_case_types)]
    #[derive(PartialEq, Eq, Debug, Clone, Copy)]
    pub enum Ctrl {
        kEmpty = -128_i8,   // 0b10000000
        kDeleted = -2_i8, // 0b11111110
        kSentinel = -1_i8,  // 0b11111111
    }
    const _: () = {
        assert!(Ctrl::kEmpty as i8 & Ctrl::kDeleted as i8 & Ctrl::kSentinel as i8 & 0x80 != 0, "Special markers need to have the MSB to make checking for them efficient");
        assert!(Ctrl::kEmpty as i8 < Ctrl::kSentinel as i8 && Ctrl::kDeleted as i8 < Ctrl::kSentinel as i8, "kEmpty and kDeleted must be smaller than kSentinel to make the SIMD test of IsEmptyOrDeleted() efficient");
        assert!(Ctrl::kSentinel as i8 == -1, "kSentinel must be -1 to elide loading it from memory into SIMD registers (pcmpeqd xmm, xmm)");
        assert!(Ctrl::kEmpty as i8 == -128, "kEmpty must be -128 to make the SIMD check for its existence efficient (psignb xmm, xmm)");
        assert!(!(Ctrl::kEmpty as i8) & !(Ctrl::kDeleted as i8) & (Ctrl::kSentinel as i8) & 0x7F != 0, "kEmpty and kDeleted must share an unset bit that is not shared by kSentinel to make the scalar test for MatchEmptyOrDeleted() efficient");
        assert!(Ctrl::kDeleted as i8 == -2, "kDeleted must be -2 to make the implementation of ConvertSpecialToEmptyAndFullToDeleted efficient");
    };

    /// See below for explanation of H2. Just here for documentation purposes, Swiss
    /// Table implementations rely on this being 7.
    pub const K_H2_BITS: i32 = 7;

    pub const K_NOT_FULL_MASK: i32 = (1 << K_H2_BITS);
    const _: () = {
        assert!(Ctrl::kEmpty as i8 & Ctrl::kDeleted as i8 & Ctrl::kSentinel as i8 & (K_NOT_FULL_MASK as i8) != 0, "Special markers need to have the MSB to make checking for them efficient");
    };

    /// Extracts H1 from the given overall hash, which means discarding the lowest 7
    /// bits of the overall hash. H1 is used to determine the first group to probe.
    #[inline]
    pub fn h1(hash: u32) -> u32 {
        hash >> K_H2_BITS
    }

    /// Extracts H2 from the given overall hash, which means using only the lowest 7
    /// bits of the overall hash. H2 is stored in the control table byte for each
    /// present entry.
    #[inline]
    pub fn h2(hash: u32) -> ctrl_t {
        (hash & ((1 << K_H2_BITS) - 1)) as ctrl_t
    }

    #[cfg(all(
        feature = "simd",
        any(target_arch = "x86", target_arch = "x86_64"),
        V8_SWISS_TABLE_HAVE_SSE2_HOST
    ))]
    pub mod sse2 {
        use super::*;
        use std::arch::x86_64::*;
        use std::mem;

        pub struct GroupSse2Impl {
            pub ctrl: __m128i,
        }

        impl GroupSse2Impl {
            pub const WIDTH: usize = 16;

            pub fn new(pos: *const ctrl_t) -> Self {
                unsafe {
                    GroupSse2Impl {
                        ctrl: _mm_loadu_si128(pos as *const __m128i),
                    }
                }
            }

            /// Returns a bitmask representing the positions of slots that match |hash|.
            pub fn match_hash(&self, hash: h2_t) -> BitMask<u32, { Self::WIDTH }> {
                unsafe {
                    let match_ = _mm_set1_epi8(hash as i8);
                    BitMask::new(_mm_movemask_epi8(_mm_cmpeq_epi8(match_, self.ctrl)) as u32)
                }
            }

            /// Returns a bitmask representing the positions of empty slots.
            #[cfg(V8_SWISS_TABLE_HAVE_SSSE3_HOST)]
            pub fn match_empty(&self) -> BitMask<u32, { Self::WIDTH }> {
                // This only works because kEmpty is -128.
                unsafe {
                    BitMask::new(_mm_movemask_epi8(_mm_sign_epi8(self.ctrl, self.ctrl)) as u32)
                }
            }

            /// Returns a bitmask representing the positions of empty slots.
            #[cfg(not(V8_SWISS_TABLE_HAVE_SSSE3_HOST))]
            pub fn match_empty(&self) -> BitMask<u32, { Self::WIDTH }> {
                self.match_hash(Ctrl::kEmpty as h2_t)
            }
        }
    }

    /// A portable, inefficient version of GroupSse2Impl. This exists so SSE2-less
    /// hosts can generate snapshots for SSE2-capable targets.
    #[cfg(all(feature = "simd", V8_SWISS_TABLE_HAVE_SSE2_TARGET))]
    pub mod sse2_polyfill {
        use super::*;

        pub struct GroupSse2Polyfill {
            ctrl_: [ctrl_t; Self::WIDTH],
        }

        impl GroupSse2Polyfill {
            pub const WIDTH: usize = 16; // the number of slots per group

            pub fn new(pos: *const ctrl_t) -> Self {
                let mut ctrl_: [ctrl_t; Self::WIDTH] = [0; Self::WIDTH];
                unsafe {
                    std::ptr::copy_nonoverlapping(pos, ctrl_.as_mut_ptr(), Self::WIDTH);
                }
                GroupSse2Polyfill { ctrl_ }
            }

            /// Returns a bitmask representing the positions of slots that match |hash|.
            pub fn match_hash(&self, hash: h2_t) -> BitMask<u32, { Self::WIDTH }> {
                let mut mask: u32 = 0;
                for i in 0..Self::WIDTH {
                    if self.ctrl_[i] as h2_t == hash {
                        mask |= 1 << i;
                    }
                }
                BitMask::new(mask)
            }

            /// Returns a bitmask representing the positions of empty slots.
            pub fn match_empty(&self) -> BitMask<u32, { Self::WIDTH }> {
                self.match_hash(Ctrl::kEmpty as h2_t)
            }

            #[allow(dead_code)]
            fn match_empty_or_deleted_mask(&self) -> u32 {
                let mut mask: u32 = 0;
                for i in 0..Self::WIDTH {
                    if self.ctrl_[i] < Ctrl::kSentinel as i8 {
                        mask |= 1 << i;
                    }
                }
                mask
            }
        }
    }

    pub mod portable {
        use super::*;
        use std::mem;

        pub struct GroupPortableImpl {
            pub ctrl: u64,
        }

        impl GroupPortableImpl {
            pub const WIDTH: usize = 8; // the number of slots per group

            pub const MSBS: u64 = 0x8080808080808080u64;
            pub const LSBS: u64 = 0x0101010101010101u64;

            pub fn new(pos: *const ctrl_t) -> Self {
                let ctrl = unsafe { mem::transmute::<*const ctrl_t, u64>(pos) };
                GroupPortableImpl { ctrl: ctrl.to_le() }
            }

            /// Returns a bitmask representing the positions of slots that match |hash|.
            pub fn match_hash(&self, hash: h2_t) -> BitMask<u64, { Self::WIDTH }, 3> {
                // For the technique, see:
                // http://graphics.stanford.edu/~seander/bithacks.html##ValueInWord
                // (Determine if a word has a byte equal to n).
                //
                // Caveat: there are false positives but:
                // - they only occur if |hash| actually appears elsewhere in |ctrl|
                // - they never occur on kEmpty, kDeleted, kSentinel
                // - they will be handled gracefully by subsequent checks in code
                //
                // Example:
                //   v = 0x1716151413121110
                //   hash = 0x12
                //   retval = (v - lsbs) & ~v & msbs = 0x0000000080800000
                let x = self.ctrl ^ (Self::LSBS * (hash as u64));
                BitMask::new((x.wrapping_sub(Self::LSBS)) & !x & Self::MSBS)
            }

            /// Returns a bitmask representing the positions of empty slots.
            pub fn match_empty(&self) -> BitMask<u64, { Self::WIDTH }, 3> {
                BitMask::new((self.ctrl & (!self.ctrl << 6)) & Self::MSBS)
            }
        }
    }

    /// Determine which Group implementation SwissNameDictionary uses.
    #[cfg(all(
        feature = "simd",
        any(target_arch = "x86", target_arch = "x86_64"),
        V8_SWISS_TABLE_HAVE_SSE2_HOST
    ))]
    #[allow(dead_code)]
    pub type Group = sse2::GroupSse2Impl;

    #[cfg(all(feature = "simd", V8_SWISS_TABLE_HAVE_SSE2_TARGET, not(V8_SWISS_TABLE_HAVE_SSE2_HOST)))]
    #[allow(dead_code)]
    pub type Group = sse2_polyfill::GroupSse2Polyfill;

    #[cfg(not(any(
        all(
            feature = "simd",
            any(target_arch = "x86", target_arch = "x86_64"),
            V8_SWISS_TABLE_HAVE_SSE2_HOST
        ),
        all(feature = "simd", V8_SWISS_TABLE_HAVE_SSE2_TARGET, not(V8_SWISS_TABLE_HAVE_SSE2_HOST))
    )))]
    #[allow(dead_code)]
    pub type Group = portable::GroupPortableImpl;
}