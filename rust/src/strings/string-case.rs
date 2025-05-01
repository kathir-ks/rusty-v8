// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/strings/string-case.h (Rust module definition)
pub mod string_case {
    use std::{
        convert::TryInto,
        limits::*,
        mem::{self, size_of},
        sync::atomic::{AtomicBool, Ordering},
    };

    /// FastAsciiConvert tries to do character processing on a word_t basis if
    /// source and destination strings are properly aligned. Natural alignment of
    /// string data depends on kTaggedSize so we define word_t via Tagged_t.
    type word_t = u64; // Assuming Tagged_t is 64-bit.  Needs to be verified.

    const K_WORD_T_ALL_BITS_SET: word_t = word_t::MAX;
    const K_ONE_IN_EVERY_BYTE: word_t = K_WORD_T_ALL_BITS_SET / 0xFF;
    const K_ASCII_MASK: word_t = K_ONE_IN_EVERY_BYTE << 7;

    #[cfg(debug_assertions)]
    fn check_fast_ascii_convert(
        dst: &mut [u8],
        src: &[u8],
        length: u32,
        changed: bool,
        is_to_lower: bool,
    ) -> bool {
        let mut expected_changed = false;
        for i in 0..length {
            if dst[i as usize] == src[i as usize] {
                continue;
            }
            expected_changed = true;
            if is_to_lower {
                assert!(b'A' <= src[i as usize] && src[i as usize] <= b'Z');
                assert!(dst[i as usize] == src[i as usize] + (b'a' - b'A'));
            } else {
                assert!(b'a' <= src[i as usize] && src[i as usize] <= b'z');
                assert!(dst[i as usize] == src[i as usize] - (b'a' - b'A'));
            }
        }
        expected_changed == changed
    }

    /// Given a word and two range boundaries returns a word with high bit
    /// set in every byte iff the corresponding input byte was strictly in
    /// the range (m, n). All the other bits in the result are cleared.
    /// This function is only useful when it can be inlined and the
    /// boundaries are statically known.
    /// Requires: all bytes in the input word and the boundaries must be
    /// ASCII (less than 0x7F).
    #[inline]
    fn ascii_range_mask(w: word_t, m: u8, n: u8) -> word_t {
        // Use strict inequalities since in edge cases the function could be
        // further simplified.
        assert!(0 < m && m < n);
        // Has high bit set in every w byte less than n.
        let tmp1 = K_ONE_IN_EVERY_BYTE * (0x7F + n as word_t) - w;
        // Has high bit set in every w byte greater than m.
        let tmp2 = w + K_ONE_IN_EVERY_BYTE * (0x7F - m as word_t);
        tmp1 & tmp2 & (K_ONE_IN_EVERY_BYTE * 0x80)
    }

    /// Converts ASCII strings to lowercase or uppercase.
    ///
    /// # Type Parameters
    ///
    /// * `is_lower`: A boolean indicating whether to convert to lowercase (true)
    ///   or uppercase (false).
    ///
    /// # Arguments
    ///
    /// * `dst`: A mutable slice to store the converted string.
    /// * `src`: An immutable slice representing the original string.
    /// * `length`: The length of the string to convert.
    /// * `changed_out`: A mutable reference to a boolean that will be set to true
    ///   if any characters were changed during the conversion.
    ///
    /// # Returns
    ///
    /// The number of bytes processed from the source string.
    pub fn fast_ascii_convert<const IS_LOWER: bool>(
        dst: &mut [u8],
        src: &[u8],
        length: u32,
        changed_out: &mut bool,
    ) -> u32 {
        let saved_src = src;
        let mut changed = false;
        // We rely on the distance between upper and lower case letters
        // being a known power of 2.
        assert_eq!(b'a' - b'A', 1 << 5);
        // Boundaries for the range of input characters than require conversion.
        let lo = if IS_LOWER { b'A' - 1 } else { b'a' - 1 };
        let hi = if IS_LOWER { b'Z' + 1 } else { b'z' + 1 };
        let limit = src.as_ptr() as usize + length as usize;
        let mut src_ptr = src.as_ptr() as usize;
        let mut dst_ptr = dst.as_mut_ptr() as usize;

        // dst is newly allocated and always aligned.
        assert!(dst_ptr % size_of::<word_t>() == 0);
        // Only attempt processing one word at a time if src is also aligned.
        if src_ptr % size_of::<word_t>() == 0 {
            // Process the prefix of the input that requires no conversion one aligned
            // (machine) word at a time.
            while src_ptr <= limit - size_of::<word_t>() {
                let w = unsafe { *(src_ptr as *const word_t) };
                if (w & K_ASCII_MASK) != 0 {
                    return (src_ptr - saved_src.as_ptr() as usize) as u32;
                }
                if ascii_range_mask(w, lo, hi) != 0 {
                    changed = true;
                    break;
                }
                unsafe { *(dst_ptr as *mut word_t) = w };
                src_ptr += size_of::<word_t>();
                dst_ptr += size_of::<word_t>();
            }
            // Process the remainder of the input performing conversion when
            // required one word at a time.
            while (src_ptr <= limit - size_of::<word_t>()) {
                let w = unsafe { *(src_ptr as *const word_t) };
                if (w & K_ASCII_MASK) != 0 {
                    return (src_ptr - saved_src.as_ptr() as usize) as u32;
                }
                let m = ascii_range_mask(w, lo, hi);
                // The mask has high (7th) bit set in every byte that needs
                // conversion and we know that the distance between cases is
                // 1 << 5.
                unsafe { *(dst_ptr as *mut word_t) = w ^ (m >> 2) };
                src_ptr += size_of::<word_t>();
                dst_ptr += size_of::<word_t>();
            }
        }

        let mut src_slice = unsafe { std::slice::from_raw_parts(src_ptr as *const u8, limit - src_ptr) };
        let mut dst_slice = unsafe { std::slice::from_raw_parts_mut(dst_ptr as *mut u8, limit - src_ptr) };

        // Process the last few bytes of the input (or the whole input if
        // unaligned access is not supported).
        for (i, c) in src_slice.iter().enumerate() {
            if (c & (K_ASCII_MASK as u8)) != 0 {
                return (src_ptr - saved_src.as_ptr() as usize) as u32;
            }
            let mut c_mut = *c;
            if lo < *c && *c < hi {
                c_mut ^= 1 << 5;
                changed = true;
            }
            dst_slice[i] = c_mut;
            src_ptr += 1;
            dst_ptr += 1;
        }

        #[cfg(debug_assertions)]
        assert!(check_fast_ascii_convert(
            dst,
            saved_src,
            length,
            changed,
            IS_LOWER
        ));

        *changed_out = changed;
        (src_ptr - saved_src.as_ptr() as usize) as u32
    }
}