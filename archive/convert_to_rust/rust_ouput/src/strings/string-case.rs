// Converted from V8 C++ source files:
// Header: string-case.h
// Implementation: string-case.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod string_case {
    use std::mem;
    use std::ops::BitAnd;
    use std::ops::BitXor;
    use std::ops::Shr;

    pub struct DisallowGarbageCollection {}
    impl DisallowGarbageCollection {
        pub fn new() -> Self {
            DisallowGarbageCollection {}
        }
    }

    impl Drop for DisallowGarbageCollection {
        fn drop(&mut self) {}
    }

    fn is_aligned<T>(ptr: *const T, alignment: usize) -> bool {
        (ptr as usize) % alignment == 0
    }

    // Assuming Tagged_t is a usize or u64 based on context.  Using usize for simplicity.
    type TaggedT = usize;
    type WordT = <TaggedT as num::PrimInt>::Unsigned;

    const WORD_T_ALL_BITS_SET: WordT = WordT::max_value();
    const ONE_IN_EVERY_BYTE: WordT = WORD_T_ALL_BITS_SET / 0xFF as WordT;
    const ASCII_MASK: WordT = ONE_IN_EVERY_BYTE << 7;

    #[cfg(debug_assertions)]
    fn check_fast_ascii_convert(
        dst: *mut u8,
        src: *const u8,
        length: u32,
        changed: bool,
        is_to_lower: bool,
    ) -> bool {
        let mut expected_changed = false;
        for i in 0..length {
            let dst_val = unsafe { *dst.add(i as usize) };
            let src_val = unsafe { *src.add(i as usize) };
            if dst_val == src_val {
                continue;
            }
            expected_changed = true;
            if is_to_lower {
                assert!(b'A' <= src_val && src_val <= b'Z');
                assert!(dst_val == src_val + (b'a' - b'A'));
            } else {
                assert!(b'a' <= src_val && src_val <= b'z');
                assert!(dst_val == src_val - (b'a' - b'A'));
            }
        }
        expected_changed == changed
    }

    // Requires: all bytes in the input word and the boundaries must be
    // ASCII (less than 0x7F).
    fn ascii_range_mask(w: WordT, m: char, n: char) -> WordT {
        // Use strict inequalities since in edge cases the function could be
        // further simplified.
        assert!(0 < m as i32 && (m as i32) < (n as i32));
        // Has high bit set in every w byte less than n.
        let tmp1 = ONE_IN_EVERY_BYTE * (0x7F + (n as WordT)) - w;
        // Has high bit set in every w byte greater than m.
        let tmp2 = w + ONE_IN_EVERY_BYTE * (0x7F - (m as WordT));
        tmp1.bitand(tmp2).bitand(ONE_IN_EVERY_BYTE * 0x80)
    }

    pub fn fast_ascii_convert<const IS_LOWER: bool>(
        dst: *mut u8,
        src: *const u8,
        length: u32,
        changed_out: *mut bool,
    ) -> u32 {
        #[cfg(debug_assertions)]
        let saved_dst = dst;

        let saved_src = src;
        let _no_gc = DisallowGarbageCollection::new();

        // We rely on the distance between upper and lower case letters
        // being a known power of 2.
        assert_eq!(b'a' - b'A', 1 << 5);
        // Boundaries for the range of input characters than require conversion.
        let lo = if IS_LOWER { b'A' - 1 } else { b'a' - 1 } as char;
        let hi = if IS_LOWER { b'Z' + 1 } else { b'z' + 1 } as char;

        let mut changed = false;
        let limit = unsafe { src.add(length as usize) };

        // dst is newly allocated and always aligned.
        assert!(is_aligned(dst, mem::size_of::<WordT>()));
        // Only attempt processing one word at a time if src is also aligned.
        if is_aligned(src, mem::size_of::<WordT>()) {
            // Process the prefix of the input that requires no conversion one aligned
            // (machine) word at a time.
            while unsafe { src.add(mem::size_of::<WordT>()) } <= limit {
                let w = unsafe { *(src as *const WordT) };
                if (w & ASCII_MASK) != 0 {
                    return (unsafe { src.offset_from(saved_src) }) as u32;
                }
                if ascii_range_mask(w, lo, hi) != 0 {
                    changed = true;
                    break;
                }
                unsafe { *(dst as *mut WordT) = w };
                unsafe {
                    let size = mem::size_of::<WordT>();
                    src = src.add(size);
                    dst = dst.add(size);
                };
            }
            // Process the remainder of the input performing conversion when
            // required one word at a time.
            while unsafe { src.add(mem::size_of::<WordT>()) } <= limit {
                let w = unsafe { *(src as *const WordT) };
                if (w & ASCII_MASK) != 0 {
                    return (unsafe { src.offset_from(saved_src) }) as u32;
                }
                let m = ascii_range_mask(w, lo, hi);
                // The mask has high (7th) bit set in every byte that needs
                // conversion and we know that the distance between cases is
                // 1 << 5.
                unsafe { *(dst as *mut WordT) = w.bitxor(m.shr(2)) };
                unsafe {
                    let size = mem::size_of::<WordT>();
                    src = src.add(size);
                    dst = dst.add(size);
                };
            }
        }
        // Process the last few bytes of the input (or the whole input if
        // unaligned access is not supported).
        while src < limit {
            let c = unsafe { *src };
            if (c & (ASCII_MASK as u8)) != 0 {
                return (unsafe { src.offset_from(saved_src) }) as u32;
            }
            if (lo as u8) < c && c < (hi as u8) {
                unsafe { *dst = c.bitxor(1 << 5) };
                changed = true;
            } else {
                unsafe { *dst = c };
            }
            unsafe {
                src = src.add(1);
                dst = dst.add(1);
            }
        }
        #[cfg(debug_assertions)]
        assert!(check_fast_ascii_convert(
            saved_dst, saved_src, length, changed, IS_LOWER
        ));

        unsafe { *changed_out = changed };
        length
    }
}
