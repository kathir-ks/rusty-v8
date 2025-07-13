// Converted from V8 C++ source files:
// Header: swiss-hash-table-helpers.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod swiss_hash_table_helpers {
    // Collection of swiss table helpers that are independent from a specific
    // container, like SwissNameDictionary. Taken almost in verbatim from Abseil,
    // comments in this file indicate what is taken from what Abseil file.

    use crate::base::bits;
    use std::{mem, ptr};
    use std::ops::BitAnd;

    // The following #defines are taken from Abseil's have_sse.h (but renamed).
    // In Rust, we can use cfg! to check for target features during compilation
    // rather than relying on preprocessor defines.
    const V8_SWISS_TABLE_HAVE_SSE2_HOST: bool = cfg!(target_feature = "sse2");
    const V8_SWISS_TABLE_HAVE_SSSE3_HOST: bool = cfg!(target_feature = "ssse3");

    // Unlike Abseil, we cannot select SSE purely by host capabilities. When
    // creating a snapshot, the group width must be compatible. The SSE
    // implementation uses a group width of 16, whereas the non-SSE version uses 8.
    // Thus we select the group size based on target capabilities and, if the host
    // does not match, select a polyfill implementation. This means, in supported
    // cross-compiling configurations, we must be able to determine matching target
    // capabilities from the host.
    // We can use cfg! to check for target features during compilation.
    const V8_SWISS_TABLE_HAVE_SSE2_TARGET: bool = cfg!(target_feature = "sse2");
    //const V8_SWISS_TABLE_HAVE_SSE2_TARGET: bool = true;

    // All definitions below are taken from Abseil's raw_hash_set.h with only minor
    // changes, like using existing V8 versions of certain helper functions.

    // Denotes the group of the control table currently being probed.
    // Implements quadratic probing by advancing by i groups after the i-th
    // (unsuccessful) probe.
    pub struct ProbeSequence<const GROUP_SIZE: usize> {
        mask_: u32,
        offset_: u32,
        index_: u32,
    }

    impl<const GROUP_SIZE: usize> ProbeSequence<GROUP_SIZE> {
        pub fn new(hash: u32, mask: u32) -> Self {
            // Mask must be a power of 2 minus 1.
            debug_assert_eq!(0, ((mask + 1) & mask));
            let offset_ = hash & mask;
            ProbeSequence {
                mask_: mask,
                offset_: offset_,
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

        pub fn index(&self) -> usize {
            self.index_ as usize
        }
    }

    // An abstraction over a bitmask. It provides an easy way to iterate through the
    // indexes of the set bits of a bitmask. When Shift=0 (platforms with SSE),
    // this is a true bitmask.
    // When Shift=3 (used on non-SSE platforms), we obtain a "byte mask", where each
    // logical bit is represented by a full byte. The logical bit 0 is represented
    // as 0x00, whereas 1 is represented as 0x80. Other values must not appear.
    //
    // For example:
    //   for (int i : BitMask<uint32_t, 16>(0x5)) -> yields 0, 2
    //   for (int i : BitMask<uint64_t, 8, 3>(0x0000000080800000)) -> yields 2, 3
    pub struct BitMask<T, const SIGNIFICANT_BITS: usize, const SHIFT: usize = 0>
    where
        T: BitAnd<Output = T>
            + Copy
            + PartialEq
            + core::ops::Sub<Output = T>
            + core::ops::BitOr<Output = T>
            + core::ops::Not<Output = T>,
    {
        mask_: T,
    }

    impl<T, const SIGNIFICANT_BITS: usize, const SHIFT: usize>
        BitMask<T, SIGNIFICANT_BITS, SHIFT>
    where
        T: BitAnd<Output = T>
            + Copy
            + PartialEq
            + core::ops::Sub<Output = T>
            + core::ops::BitOr<Output = T>
            + core::ops::Not<Output = T>,
    {
        pub fn new(mask: T) -> Self {
            //static_assert(std::is_unsigned<T>::value);
            //static_assert(Shift == 0 || Shift == 3);
            BitMask { mask_: mask }
        }

        pub fn increment(&mut self) -> &mut Self {
            // Clear the least significant bit that is set.
            self.mask_ = self.mask_ & (self.mask_ - Self::one());
            self
        }

        fn one() -> T {
            unsafe { mem::transmute::<u8, T>(1) }
        }

        pub fn to_bool(&self) -> bool {
            self.mask_ != Self::zero()
        }

        fn zero() -> T {
            unsafe { mem::transmute::<u8, T>(0) }
        }
    }

    impl<T, const SIGNIFICANT_BITS: usize, const SHIFT: usize>
        BitMask<T, SIGNIFICANT_BITS, SHIFT>
    where
        T: BitAnd<Output = T>
            + Copy
            + PartialEq
            + core::ops::Sub<Output = T>
            + core::ops::BitOr<Output = T>
            + core::ops::Not<Output = T>
            + num::Integer,
    {
        pub fn lowest_bit_set(&self) -> i32 {
            self.trailing_zeros() as i32
        }
        pub fn highest_bit_set(&self) -> i32 {
            (mem::size_of::<T>() as i32 * 8 - bits::count_leading_zeros(self.mask_) as i32 - 1)
                >> SHIFT
        }

        pub fn trailing_zeros(&self) -> u32 {
            debug_assert_ne!(self.mask_, Self::zero());
            (bits::count_trailing_zeros_nonzero(self.mask_)) >> SHIFT
        }

        pub fn leading_zeros(&self) -> u32 {
            let total_significant_bits = SIGNIFICANT_BITS as u32 * SHIFT as u32;
            let extra_bits = mem::size_of::<T>() as u32 * 8 - total_significant_bits;
            (bits::count_leading_zeros(self.mask_ << extra_bits)) >> SHIFT
        }
    }

    impl<T, const SIGNIFICANT_BITS: usize, const SHIFT: usize> PartialEq
        for BitMask<T, SIGNIFICANT_BITS, SHIFT>
    where
        T: BitAnd<Output = T>
            + Copy
            + PartialEq
            + core::ops::Sub<Output = T>
            + core::ops::BitOr<Output = T>
            + core::ops::Not<Output = T>,
    {
        fn eq(&self, other: &Self) -> bool {
            self.mask_ == other.mask_
        }
    }

    pub type ctrl_t = i8;
    pub type h2_t = u8;

    // The values here are selected for maximum performance. See the static asserts
    // below for details.
    #[allow(dead_code)]
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub enum Ctrl {
        kEmpty = -128,   // 0b10000000
        kDeleted = -2,   // 0b11111110
        kSentinel = -1,  // 0b11111111
    }
    const _ASSERT: () = {
        assert!(
            (Ctrl::kEmpty as i8) & (Ctrl::kDeleted as i8) & (Ctrl::kSentinel as i8) & 0x80 != 0,
            "Special markers need to have the MSB to make checking for them efficient"
        );
        assert!(
            (Ctrl::kEmpty as i8) < (Ctrl::kSentinel as i8)
                && (Ctrl::kDeleted as i8) < (Ctrl::kSentinel as i8),
            "kEmpty and kDeleted must be smaller than kSentinel to make the SIMD test of IsEmptyOrDeleted() efficient"
        );
        assert!(
            (Ctrl::kSentinel as i8) == -1,
            "kSentinel must be -1 to elide loading it from memory into SIMD registers (pcmpeqd xmm, xmm)"
        );
        assert!(
            (Ctrl::kEmpty as i8) == -128,
            "kEmpty must be -128 to make the SIMD check for its existence efficient (psignb xmm, xmm)"
        );
        assert!(
            !(Ctrl::kEmpty as i8) & !(Ctrl::kDeleted as i8) & (Ctrl::kSentinel as i8) & 0x7F != 0,
            "kEmpty and kDeleted must share an unset bit that is not shared by kSentinel to make the scalar test for MatchEmptyOrDeleted() efficient"
        );
        assert!(
            (Ctrl::kDeleted as i8) == -2,
            "kDeleted must be -2 to make the implementation of ConvertSpecialToEmptyAndFullToDeleted efficient"
        );
    };

    // See below for explanation of H2. Just here for documentation purposes, Swiss
    // Table implementations rely on this being 7.
    const kH2Bits: i32 = 7;

    const kNotFullMask: i32 = (1 << kH2Bits);
    const _ASSERT2: () = {
        assert!(
            (Ctrl::kEmpty as i32) & (Ctrl::kDeleted as i32) & (Ctrl::kSentinel as i32)
                & kNotFullMask
                != 0,
            "Special markers need to have the MSB to make checking for them efficient"
        );
    };

    // Extracts H1 from the given overall hash, which means discarding the lowest 7
    // bits of the overall hash. H1 is used to determine the first group to probe.
    #[inline]
    pub fn h1(hash: u32) -> u32 {
        (hash >> kH2Bits)
    }

    // Extracts H2 from the given overall hash, which means using only the lowest 7
    // bits of the overall hash. H2 is stored in the control table byte for each
    // present entry.
    #[inline]
    pub fn h2(hash: u32) -> ctrl_t {
        (hash & ((1 << kH2Bits) - 1)) as i8
    }

    #[cfg(all(
        target_arch = "x86_64",
        V8_SWISS_TABLE_HAVE_SSE2_HOST,
        V8_SWISS_TABLE_HAVE_SSE2_TARGET
    ))]
    mod sse2 {
        use std::arch::x86_64::*;
        use std::mem;
        use crate::internal::swiss_table::{h2_t, ctrl_t, Ctrl, BitMask};

        pub struct GroupSse2Impl {
            pub ctrl: __m128i,
        }

        impl GroupSse2Impl {
            pub const K_WIDTH: usize = 16; // the number of slots per group
            pub fn new(pos: *const ctrl_t) -> Self {
                unsafe {
                    let ctrl = _mm_loadu_si128(pos as *const __m128i);
                    GroupSse2Impl { ctrl }
                }
            }

            // Returns a bitmask representing the positions of slots that match |hash|.
            pub fn match_hash(&self, hash: h2_t) -> BitMask<u32, Self::K_WIDTH> {
                unsafe {
                    let match_ = _mm_set1_epi8(hash as i8);
                    BitMask::new(_mm_movemask_epi8(_mm_cmpeq_epi8(match_, self.ctrl)) as u32)
                }
            }

            // Returns a bitmask representing the positions of empty slots.
            #[cfg(V8_SWISS_TABLE_HAVE_SSSE3_HOST)]
            pub fn match_empty(&self) -> BitMask<u32, Self::K_WIDTH> {
                unsafe {
                    BitMask::new(_mm_movemask_epi8(_mm_sign_epi8(self.ctrl, self.ctrl)) as u32)
                }
            }

            #[cfg(not(V8_SWISS_TABLE_HAVE_SSSE3_HOST))]
            pub fn match_empty(&self) -> BitMask<u32, Self::K_WIDTH> {
                self.match_hash(Ctrl::kEmpty as h2_t)
            }
        }
    }

    // A portable, inefficient version of GroupSse2Impl. This exists so SSE2-less
    // hosts can generate snapshots for SSE2-capable targets.
    #[cfg(all(
        target_arch = "x86_64",
        not(V8_SWISS_TABLE_HAVE_SSE2_HOST),
        V8_SWISS_TABLE_HAVE_SSE2_TARGET
    ))]
    mod sse2_polyfill {
        use crate::internal::swiss_table::{ctrl_t, h2_t, BitMask, Ctrl};
        use std::mem;
        pub struct GroupSse2Polyfill {
            ctrl_: [ctrl_t; Self::K_WIDTH],
        }

        impl GroupSse2Polyfill {
            pub const K_WIDTH: usize = 16; // the number of slots per group
            pub fn new(pos: *const ctrl_t) -> Self {
                let mut ctrl_: [ctrl_t; Self::K_WIDTH] = [0; Self::K_WIDTH];
                unsafe {
                   libc::memcpy(ctrl_.as_mut_ptr() as *mut libc::c_void, pos as *const libc::c_void, Self::K_WIDTH);
                }
                GroupSse2Polyfill { ctrl_ }
            }

            // Returns a bitmask representing the positions of slots that match |hash|.
            pub fn match_hash(&self, hash: h2_t) -> BitMask<u32, Self::K_WIDTH> {
                let mut mask: u32 = 0;
                for i in 0..Self::K_WIDTH {
                    if self.ctrl_[i] as u8 == hash {
                        mask |= 1u32 << i;
                    }
                }
                BitMask::new(mask)
            }

            // Returns a bitmask representing the positions of empty slots.
            pub fn match_empty(&self) -> BitMask<u32, Self::K_WIDTH> {
                self.match_hash(Ctrl::kEmpty as h2_t)
            }
        }

        impl GroupSse2Polyfill {
            fn match_empty_or_deleted_mask(&self) -> u32 {
                let mut mask: u32 = 0;
                for i in 0..Self::K_WIDTH {
                    if self.ctrl_[i] < Ctrl::kSentinel as i8 {
                        mask |= 1u32 << i;
                    }
                }
                mask
            }
        }
    }

    #[cfg(not(all(
        target_arch = "x86_64",
        V8_SWISS_TABLE_HAVE_SSE2_HOST,
        V8_SWISS_TABLE_HAVE_SSE2_TARGET
    )))]
    #[cfg(not(all(
        target_arch = "x86_64",
        not(V8_SWISS_TABLE_HAVE_SSE2_HOST),
        V8_SWISS_TABLE_HAVE_SSE2_TARGET
    )))]
    mod portable {
        use crate::internal::swiss_table::{ctrl_t, h2_t, BitMask, Ctrl};
        use std::mem;

        pub struct GroupPortableImpl {
            pub ctrl: u64,
        }

        impl GroupPortableImpl {
            pub const K_WIDTH: usize = 8; // the number of slots per group

            const K_MSBS: u64 = 0x8080808080808080;
            const K_LSBS: u64 = 0x0101010101010101;

            pub fn new(pos: *const ctrl_t) -> Self {
                let ctrl = unsafe {
                    (pos as *const u64).read_unaligned()
                };
                GroupPortableImpl { ctrl }
            }

            // Returns a bitmask representing the positions of slots that match |hash|.
            pub fn match_hash(&self, hash: h2_t) -> BitMask<u64, Self::K_WIDTH, 3> {
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
                let x = self.ctrl ^ (Self::K_LSBS * hash as u64);
                BitMask::new((x - Self::K_LSBS) & !x & Self::K_MSBS)
            }

            // Returns a bitmask representing the positions of empty slots.
            pub fn match_empty(&self) -> BitMask<u64, Self::K_WIDTH, 3> {
                BitMask::new((self.ctrl & (!self.ctrl << 6)) & Self::K_MSBS)
            }
        }
    }

    // Determine which Group implementation SwissNameDictionary uses.
    #[cfg(all(
        defined(V8_ENABLE_SWISS_NAME_DICTIONARY),
        debug_assertions
    ))]
    type Group = portable::GroupPortableImpl;

    #[cfg(not(all(
        defined(V8_ENABLE_SWISS_NAME_DICTIONARY),
        debug_assertions
    )))]
    #[cfg(all(
        target_arch = "x86_64",
        V8_SWISS_TABLE_HAVE_SSE2_HOST,
        V8_SWISS_TABLE_HAVE_SSE2_TARGET
    ))]
    type Group = sse2::GroupSse2Impl;

    #[cfg(not(all(
        defined(V8_ENABLE_SWISS_NAME_DICTIONARY),
        debug_assertions
    )))]
    #[cfg(all(
        target_arch = "x86_64",
        not(V8_SWISS_TABLE_HAVE_SSE2_HOST),
        V8_SWISS_TABLE_HAVE_SSE2_TARGET
    ))]
    type Group = sse2_polyfill::GroupSse2Polyfill;

    #[cfg(not(all(
        defined(V8_ENABLE_SWISS_NAME_DICTIONARY),
        debug_assertions
    )))]
    #[cfg(not(all(
        target_arch = "x86_64",
        V8_SWISS_TABLE_HAVE_SSE2_HOST,
        V8_SWISS_TABLE_HAVE_SSE2_TARGET
    ))]
    #[cfg(not(all(
        target_arch = "x86_64",
        not(V8_SWISS_TABLE_HAVE_SSE2_HOST),
        V8_SWISS_TABLE_HAVE_SSE2_TARGET
    )))]
    type Group = portable::GroupPortableImpl;
}
