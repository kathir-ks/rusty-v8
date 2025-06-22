// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/wasm/simd-shuffle.h equivalent
pub mod simd_shuffle {
    /// Represents a canonical shuffle operation.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum CanonicalShuffle {
        kIdentity,
        kS64x2Even,
        kS64x2Odd,
        kS64x2Reverse,
        kS32x4InterleaveEven,
        kS32x4InterleaveOdd,
        kS32x4InterleaveLowHalves,
        kS32x4InterleaveHighHalves,
        kS32x4Reverse,
        kS32x4TransposeEven,
        kS32x4TransposeOdd,
        kS32x2Reverse,
        kS16x8InterleaveEven,
        kS16x8InterleaveOdd,
        kS16x8InterleaveLowHalves,
        kS16x8InterleaveHighHalves,
        kS16x8TransposeEven,
        kS16x8TransposeOdd,
        kS16x2Reverse,
        kS16x4Reverse,
        kS64x2ReverseBytes,
        kS32x4ReverseBytes,
        kS16x8ReverseBytes,
        kS8x16InterleaveLowHalves,
        kS8x16InterleaveHighHalves,
        kS8x16InterleaveEven,
        kS8x16InterleaveOdd,
        kS8x16TransposeEven,
        kS8x16TransposeOdd,
        kUnknown,
        kMaxShuffles, // This is not a shuffle itself, but the number of shuffles + 1
    }

    pub const K_SIMD128_SIZE: usize = 16;

    pub struct SimdShuffle {}

    impl SimdShuffle {
        pub type ShuffleArray = [u8; K_SIMD128_SIZE];

        /// Tries to match a given shuffle array against a list of canonical shuffles.
        pub fn try_match_canonical(shuffle: &ShuffleArray) -> CanonicalShuffle {
            const fn expand<const N: usize>(input: [u8; N]) -> [u8; K_SIMD128_SIZE] {
                let mut res = [0u8; K_SIMD128_SIZE];
                let lane_bytes = 16 / N;
                let mut i = 0;
                while i < N {
                    let mut j = 0;
                    while j < lane_bytes {
                        res[i * lane_bytes + j] = (lane_bytes * input[i] + j) as u8;
                        j += 1;
                    }
                    i += 1;
                }
                res
            }

            const CANONICAL_SHUFFLE_LIST: [([u8; K_SIMD128_SIZE], CanonicalShuffle); 29] = [
                (expand::<2>([0, 1]), CanonicalShuffle::kIdentity),
                (expand::<2>([0, 2]), CanonicalShuffle::kS64x2Even),
                (expand::<2>([1, 3]), CanonicalShuffle::kS64x2Odd),
                (expand::<2>([1, 0]), CanonicalShuffle::kS64x2Reverse),
                (expand::<4>([0, 2, 4, 6]), CanonicalShuffle::kS32x4InterleaveEven),
                (expand::<4>([1, 3, 5, 7]), CanonicalShuffle::kS32x4InterleaveOdd),
                (expand::<4>([0, 4, 1, 5]), CanonicalShuffle::kS32x4InterleaveLowHalves),
                (expand::<4>([2, 6, 3, 7]), CanonicalShuffle::kS32x4InterleaveHighHalves),
                (expand::<4>([3, 2, 1, 0]), CanonicalShuffle::kS32x4Reverse),
                (expand::<4>([0, 4, 2, 6]), CanonicalShuffle::kS32x4TransposeEven),
                (expand::<4>([1, 5, 3, 7]), CanonicalShuffle::kS32x4TransposeOdd),
                (expand::<4>([1, 0, 3, 2]), CanonicalShuffle::kS32x2Reverse),
                (expand::<8>([0, 2, 4, 6, 8, 10, 12, 14]), CanonicalShuffle::kS16x8InterleaveEven),
                (expand::<8>([1, 3, 5, 7, 9, 11, 13, 15]), CanonicalShuffle::kS16x8InterleaveOdd),
                (expand::<8>([0, 8, 1, 9, 2, 10, 3, 11]), CanonicalShuffle::kS16x8InterleaveLowHalves),
                (expand::<8>([4, 12, 5, 13, 6, 14, 7, 15]), CanonicalShuffle::kS16x8InterleaveHighHalves),
                (expand::<8>([0, 8, 2, 10, 4, 12, 6, 14]), CanonicalShuffle::kS16x8TransposeEven),
                (expand::<8>([1, 9, 3, 11, 5, 13, 7, 15]), CanonicalShuffle::kS16x8TransposeOdd),
                (expand::<8>([1, 0, 3, 2, 5, 4, 7, 6]), CanonicalShuffle::kS16x2Reverse),
                (expand::<8>([3, 2, 1, 0, 7, 6, 5, 4]), CanonicalShuffle::kS16x4Reverse),
                ([7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8], CanonicalShuffle::kS64x2ReverseBytes),
                ([3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12], CanonicalShuffle::kS32x4ReverseBytes),
                ([1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14], CanonicalShuffle::kS16x8ReverseBytes),
                ([0, 16, 1, 17, 2, 18, 3, 19, 4, 20, 5, 21, 6, 22, 7, 23], CanonicalShuffle::kS8x16InterleaveLowHalves),
                ([8, 24, 9, 25, 10, 26, 11, 27, 12, 28, 13, 29, 14, 30, 15, 31], CanonicalShuffle::kS8x16InterleaveHighHalves),
                ([0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30], CanonicalShuffle::kS8x16InterleaveEven),
                ([1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31], CanonicalShuffle::kS8x16InterleaveOdd),
                ([0, 16, 2, 18, 4, 20, 6, 22, 8, 24, 10, 26, 12, 28, 14, 30], CanonicalShuffle::kS8x16TransposeEven),
                ([1, 17, 3, 19, 5, 21, 7, 23, 9, 25, 11, 27, 13, 29, 15, 31], CanonicalShuffle::kS8x16TransposeOdd),
            ];

            for ( (lanes, canonical) in CANONICAL_SHUFFLE_LIST.iter() {
                if lanes == shuffle {
                    return *canonical;
                }
            }
            CanonicalShuffle::kUnknown
        }

        /// Tries to match if a given shuffle is an identity shuffle.
        pub fn try_match_identity(shuffle: &[u8]) -> bool {
            for i in 0..K_SIMD128_SIZE {
                if shuffle[i] != i as u8 {
                    return false;
                }
            }
            true
        }

        /// Tries to match a 32x4 rotate shuffle.
        pub fn try_match_32x4_rotate(shuffle: &[u8], shuffle32x4: &mut [u8; 4], is_swizzle: bool) -> bool {
            let mut offset: u8 = 0;
            let is_concat = Self::try_match_concat(shuffle, &mut offset);
            if offset == 0 {
                return false;
            } // 0 is identity, it should not be matched.
              // Since we already have a concat shuffle, we know that the indices goes from:
              // [ offset, ..., 15, 0, ... ], it suffices to check that the offset points
              // to the low byte of a 32x4 element.
            if !is_concat || !is_swizzle || offset % 4 != 0 {
                return false;
            }

            let offset_32 = offset / 4;
            for i in 0..4 {
                shuffle32x4[i] = ((offset_32 as u32 + i as u32) % 4) as u8;
            }
            true
        }

        /// Tries to match a 32x4 reverse shuffle.
        pub fn try_match_32x4_reverse(shuffle32x4: &[u8; 4]) -> bool {
            shuffle32x4[0] == 3 && shuffle32x4[1] == 2 && shuffle32x4[2] == 1 && shuffle32x4[3] == 0
        }

        /// Tries to match a 32x4 one-lane swizzle shuffle.
        pub fn try_match_32x4_one_lane_swizzle(
            shuffle32x4: &[u8; 4],
            from_lane: &mut u8,
            to_lane: &mut u8,
        ) -> bool {
            const PATTERNS: [u32; 12] = [
                0x30200000,  // 0 -> 1
                0x30000100,  // 0 -> 2
                0x00020100,  // 0 -> 3
                0x03020101,  // 1 -> 0
                0x03010100,  // 1 -> 2
                0x01020100,  // 1 -> 3
                0x03020102,  // 2 -> 0
                0x03020200,  // 2 -> 1
                0x02020100,  // 2 -> 3
                0x03020103,  // 3 -> 0
                0x03020300,  // 3 -> 1
                0x03030100, // 3 -> 2
            ];

            let mut pattern_idx = 0;
            //FIXME: Handle endianness correctly.
            let shuffle = u32::from_ne_bytes(shuffle32x4.clone().try_into().unwrap());

            for from in 0..4 {
                for to in 0..4 {
                    if from == to {
                        continue;
                    }
                    if shuffle == PATTERNS[pattern_idx] {
                        *from_lane = from as u8;
                        *to_lane = to as u8;
                        return true;
                    }
                    pattern_idx += 1;
                }
            }
            false
        }

        /// Tries to match a 64x2 shuffle.
        pub fn try_match_64x2_shuffle(shuffle: &[u8], shuffle64x2: &mut [u8; 2]) -> bool {
            const ELEMENT_PATTERNS: [[u8; 8]; 4] = [
                [0, 1, 2, 3, 4, 5, 6, 7],
                [8, 9, 10, 11, 12, 13, 14, 15],
                [16, 17, 18, 19, 20, 21, 22, 23],
                [24, 25, 26, 27, 28, 29, 30, 31],
            ];

            for i in 0..2 {
                let element = u64::from_ne_bytes(shuffle[i * 8..(i * 8 + 8)].try_into().unwrap());
                for j in 0..4 {
                    let pattern = u64::from_ne_bytes(ELEMENT_PATTERNS[j].try_into().unwrap());
                    if pattern == element {
                        shuffle64x2[i] = j as u8;
                        break;
                    }
                    if j == 3 {
                        return false;
                    }
                }
            }
            true
        }

        fn match_helper<const LANES: usize, const LANE_BYTES: usize>(input: &[u8], output: &mut [u8; LANES]) -> bool {
            for i in 0..LANES {
                if input[i * LANE_BYTES] % LANE_BYTES as u8 != 0 {
                    return false;
                }
                for j in 1..LANE_BYTES {
                    if input[i * LANE_BYTES + j] as i32 - input[i * LANE_BYTES + j - 1] as i32 != 1 {
                        return false;
                    }
                }
                output[i] = (input[i * LANE_BYTES] / LANE_BYTES as u8) as u8;
            }
            true
        }

        /// Tries to match a 64x1 shuffle.
        pub fn try_match_64x1_shuffle(shuffle: &[u8], shuffle64x1: &mut [u8; 1]) -> bool {
            Self::match_helper::<1, 8>(shuffle, shuffle64x1)
        }

        /// Tries to match a 32x1 shuffle.
        pub fn try_match_32x1_shuffle(shuffle: &[u8], shuffle32x1: &mut [u8; 1]) -> bool {
            Self::match_helper::<1, 4>(shuffle, shuffle32x1)
        }

        /// Tries to match a 32x2 shuffle.
        pub fn try_match_32x2_shuffle(shuffle: &[u8], shuffle32x2: &mut [u8; 2]) -> bool {
            Self::match_helper::<2, 4>(shuffle, shuffle32x2)
        }

        /// Tries to match a 32x4 shuffle.
        pub fn try_match_32x4_shuffle(shuffle: &[u8], shuffle32x4: &mut [u8; 4]) -> bool {
            Self::match_helper::<4, 4>(shuffle, shuffle32x4)
        }

        /// Tries to match a 32x8 shuffle.
        pub fn try_match_32x8_shuffle(shuffle: &[u8], shuffle32x8: &mut [u8; 8]) -> bool {
            Self::match_helper::<8, 4>(shuffle, shuffle32x8)
        }

        /// Tries to match a 16x1 shuffle.
        pub fn try_match_16x1_shuffle(shuffle: &[u8], shuffle16x1: &mut [u8; 1]) -> bool {
            Self::match_helper::<1, 2>(shuffle, shuffle16x1)
        }

        /// Tries to match a 16x2 shuffle.
        pub fn try_match_16x2_shuffle(shuffle: &[u8], shuffle16x2: &mut [u8; 2]) -> bool {
            Self::match_helper::<2, 2>(shuffle, shuffle16x2)
        }

        /// Tries to match a 16x4 shuffle.
        pub fn try_match_16x4_shuffle(shuffle: &[u8], shuffle16x4: &mut [u8; 4]) -> bool {
            Self::match_helper::<4, 2>(shuffle, shuffle16x4)
        }

        /// Tries to match a 16x8 shuffle.
        pub fn try_match_16x8_shuffle(shuffle: &[u8], shuffle16x8: &mut [u8; 8]) -> bool {
            Self::match_helper::<8, 2>(shuffle, shuffle16x8)
        }

        /// Tries to match a concat shuffle.
        pub fn try_match_concat(shuffle: &[u8], offset: &mut u8) -> bool {
            // Don't match the identity shuffle (e.g. [0 1 2 ... 15]).
            let start = shuffle[0];
            if start == 0 {
                return false;
            }
            if K_SIMD128_SIZE as u8 <= start {
                return false;
            } // The shuffle should be canonicalized.
              // A concatenation is a series of consecutive indices, with at most one jump
              // in the middle from the last lane to the first.
            for i in 1..K_SIMD128_SIZE {
                if (shuffle[i] as i32) != ((shuffle[i - 1] as i32 + 1)) {
                    if shuffle[i - 1] != 15 {
                        return false;
                    }
                    if shuffle[i] % K_SIMD128_SIZE as u8 != 0 {
                        return false;
                    }
                }
            }
            *offset = start;
            true
        }

        /// Tries to match a blend shuffle.
        pub fn try_match_blend(shuffle: &[u8]) -> bool {
            for i in 0..16 {
                if (shuffle[i] & 0xF) != i as u8 {
                    return false;
                }
            }
            true
        }

        /// Tries to match a byte-to-dword zero-extend shuffle.
        pub fn try_match_byte_to_dword_zero_extend(shuffle: &[u8]) -> bool {
            for i in 0..16 {
                if (i % 4 != 0) && (shuffle[i] < 16) {
                    return false;
                }
                if (i % 4 == 0) && (shuffle[i] > 15 || (shuffle[i] != shuffle[0] + (i / 4) as u8)) {
                    return false;
                }
            }
            true
        }

        /// Tries to match an 8x16 upper-to-lower reduce shuffle.
        pub fn try_match_8x16_upper_to_lower_reduce(
            shuffle1: &[u8],
            shuffle2: &[u8],
            shuffle3: &[u8],
            shuffle4: &[u8],
        ) -> bool {
            Self::try_match_upper_to_lower_first(shuffle1)
                && Self::try_match_upper_to_lower_second(shuffle2)
                && Self::try_match_upper_to_lower_third(shuffle3)
                && Self::try_match_upper_to_lower_fourth(shuffle4)
        }

        /// Tries to match a 16x8 upper-to-lower reduce shuffle.
        pub fn try_match_16x8_upper_to_lower_reduce(shuffle1: &[u8], shuffle2: &[u8], shuffle3: &[u8]) -> bool {
            Self::try_match_upper_to_lower_first(shuffle1)
                && Self::try_match_upper_to_lower_second(shuffle2)
                && Self::try_match_upper_to_lower_third(shuffle3)
        }

        /// Tries to match a 32x4 upper-to-lower reduce shuffle.
        pub fn try_match_32x4_upper_to_lower_reduce(shuffle1: &[u8], shuffle2: &[u8]) -> bool {
            Self::try_match_upper_to_lower_first(shuffle1) && Self::try_match_upper_to_lower_second(shuffle2)
        }

        /// Tries to match a 32x4 pairwise reduce shuffle.
        pub fn try_match_32x4_pairwise_reduce(shuffle1: &[u8], shuffle2: &[u8]) -> bool {
            Self::try_match_32x4_pairwise(shuffle1) && Self::try_match_32x2_pairwise(shuffle2)
        }

        /// Tries to match a 64x2 reduce shuffle.
        pub fn try_match_64x2_reduce(shuffle64x2: &[u8]) -> bool {
            shuffle64x2[0] == 1
        }

        fn try_match_32x4_pairwise(shuffle: &[u8]) -> bool {
            // Pattern to select 32-bit element 1.
            const LOW_PATTERN_ARR: [u8; 4] = [4, 5, 6, 7];
            // And we'll check that element 1 is shuffled into element 0.
            let low_shuffle = u32::from_ne_bytes(shuffle[0..4].try_into().unwrap());
            // Pattern to select 32-bit element 3.
            const HIGH_PATTERN_ARR: [u8; 4] = [12, 13, 14, 15];
            // And we'll check that element 3 is shuffled into element 2.
            let high_shuffle = u32::from_ne_bytes(shuffle[8..12].try_into().unwrap());
            let low_pattern = u32::from_ne_bytes(LOW_PATTERN_ARR.try_into().unwrap());
            let high_pattern = u32::from_ne_bytes(HIGH_PATTERN_ARR.try_into().unwrap());
            low_shuffle == low_pattern && high_shuffle == high_pattern
        }

        fn try_match_32x2_pairwise(shuffle: &[u8]) -> bool {
            // Pattern to select 32-bit element 2.
            const PATTERN_ARR: [u8; 4] = [8, 9, 10, 11];
            // And we'll check that element 2 is shuffled to element 0.
            let low_shuffle = u32::from_ne_bytes(shuffle[0..4].try_into().unwrap());
            let pattern = u32::from_ne_bytes(PATTERN_ARR.try_into().unwrap());
            low_shuffle == pattern
        }

        fn try_match_upper_to_lower_first(shuffle: &[u8]) -> bool {
            // There's 16 'active' bytes, so the pattern to select the upper half starts
            // at byte 8.
            const LOW_PATTERN_ARR: [u8; 8] = [8, 9, 10, 11, 12, 13, 14, 15];
            // And we'll check that the top half is shuffled into the lower.
            let low_shuffle = u64::from_ne_bytes(shuffle[0..8].try_into().unwrap());
            let low_pattern = u64::from_ne_bytes(LOW_PATTERN_ARR.try_into().unwrap());
            low_shuffle == low_pattern
        }

        fn try_match_upper_to_lower_second(shuffle: &[u8]) -> bool {
            // There's 8 'active' bytes, so the pattern to select the upper half starts
            // at byte 4.
            const LOW_PATTERN_ARR: [u8; 4] = [4, 5, 6, 7];
            // And we'll check that the top half is shuffled into the lower.
            let low_shuffle = u32::from_ne_bytes(shuffle[0..4].try_into().unwrap());
            let low_pattern = u32::from_ne_bytes(LOW_PATTERN_ARR.try_into().unwrap());
            low_shuffle == low_pattern
        }

        fn try_match_upper_to_lower_third(shuffle: &[u8]) -> bool {
            // The vector now has 4 'active' bytes, select the top two.
            const LOW_PATTERN_ARR: [u8; 2] = [2, 3];
            // And check they're shuffled to the lower half.
            let low_shuffle = u16::from_ne_bytes(shuffle[0..2].try_into().unwrap());
            let low_pattern = u16::from_ne_bytes(LOW_PATTERN_ARR.try_into().unwrap());
            low_shuffle == low_pattern
        }

        fn try_match_upper_to_lower_fourth(shuffle: &[u8]) -> bool {
            shuffle[0] == 1
        }

        /// Packs a 4-lane shuffle into a single byte.
        pub fn pack_shuffle4(shuffle: &mut [u8]) -> u8 {
            (shuffle[0] & 3) | ((shuffle[1] & 3) << 2) | ((shuffle[2] & 3) << 4) | ((shuffle[3] & 3) << 6)
        }

        /// Packs an 8-lane blend into a single byte.
        pub fn pack_blend8(shuffle16x8: &[u8]) -> u8 {
            let mut result: i8 = 0;
            for i in 0..8 {
                result |= (if shuffle16x8[i] >= 8 { 1 } else { 0 }) << i;
            }
            result as u8
        }

        /// Packs a 4-lane blend into a single byte.
        pub fn pack_blend4(shuffle32x4: &[u8]) -> u8 {
            let mut result: i8 = 0;
            for i in 0..4 {
                result |= (if shuffle32x4[i] >= 4 { 0x3 } else { 0 }) << (i * 2);
            }
            result as u8
        }

        /// Packs two lanes into a 32-bit integer.
        pub fn pack_2_lanes(shuffle: &[u8; 2]) -> i32 {
            let mut result: i32 = 0;
            for i in (0..2).rev() {
                result <<= 8;
                result |= shuffle[i] as i32;
            }
            result
        }

        /// Packs four lanes into a 32-bit integer.
        pub fn pack_4_lanes(shuffle: &[u8]) -> i32 {
            let mut result: i32 = 0;
            for i in (0..4).rev() {
                result <<= 8;
                result |= shuffle[i] as i32;
            }
            result
        }

        /// Packs 16 lanes into an array of four 32-bit integers.
        pub fn pack_16_lanes(dst: &mut [u32], shuffle: &[u8]) {
            for i in 0..4 {
                dst[i] = Self::pack_4_lanes(&shuffle[i * 4..]);
            }
        }

        #[cfg(target_arch = "x86_64")]
        /// Tries to match a vpshufd shuffle.
        pub fn try_match_vpshufd(shuffle32x8: &[u8], control: &mut u8) -> bool {
            *control = 0;
            for i in 0..4 {
                let mask: u8;
                if shuffle32x8[i] < 4 && shuffle32x8[i + 4] - shuffle32x8[i] == 4 {
                    mask = shuffle32x8[i];
                    *control |= mask << (2 * i);
                    continue;
                }
                return false;
            }
            true
        }

        #[cfg(target_arch = "x86_64")]
        /// Tries to match a shufps256 shuffle.
        pub fn try_match_shufps256(shuffle32x8: &[u8], control: &mut u8) -> bool {
            *control = 0;
            for i in 0..4 {
                // low 128-bits and high 128-bits should have the same shuffle order.
                if shuffle32x8[i + 4] - shuffle32x8[i] == 4 {
                    // [63:0]   bits select from SRC1,
                    // [127:64] bits select from SRC2
                    if (i < 2 && shuffle32x8[i] < 4) || (i >= 2 && shuffle32x8[i] >= 8 && shuffle32x8[i] < 12) {
                        *control |= (shuffle32x8[i] % 4) << (2 * i);
                        continue;
                    }
                    return false;
                }
                return false;
            }
            true
        }
    }

    pub struct SimdSwizzle {}

    impl SimdSwizzle {
        /// Checks if all values in the shuffle are either in range or have the top bit set.
        pub fn all_in_range_or_top_bit_set(shuffle: [u8; K_SIMD128_SIZE]) -> bool {
            shuffle.iter().all(|&i| (i < K_SIMD128_SIZE as u8) || (i & 0x80 != 0))
        }
    }
}