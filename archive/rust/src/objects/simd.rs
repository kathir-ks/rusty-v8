// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// TODO: Missing equivalent of "src/objects/simd.h" (likely an empty header)

use std::{
    arch::x86_64::{
        __m128d, __m128i, __m256d, __m256i, _CMP_EQ_OQ, _MM_SHUFFLE, _mm256_castsi256_pd,
        _mm256_castsi256_ps, _mm256_cmp_pd, _mm256_cmpeq_epi32, _mm256_cmpeq_epi64,
        _mm256_movemask_pd, _mm256_movemask_ps, _mm256_set1_epi32, _mm256_set1_epi64x,
        _mm256_set1_pd, _mm_add_epi8, _mm_add_epi8, _mm_and_si128, _mm_castsi128_ps,
        _mm_cmpeq_epi32, _mm_cmpgt_epi8, _mm_load_si128, _mm_movemask_pd, _mm_movemask_ps,
        _mm_set1_epi32, _mm_set1_epi64x, _mm_set1_epi8, _mm_set1_pd, _mm_shuffle_epi32,
        _mm_storeu_si128,
    },
    mem::transmute,
};

// TODO: Missing equivalent of "src/base/cpu.h"
// TODO: Missing equivalent of "src/codegen/cpu-features.h"
// TODO: Missing equivalent of "src/objects/compressed-slots.h"
// TODO: Missing equivalent of "src/objects/fixed-array-inl.h"
// TODO: Missing equivalent of "src/objects/heap-number-inl.h"
// TODO: Missing equivalent of "src/objects/smi-inl.h"

#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::{
    vaddq_u8, vandq_u8, vceqq_f64, vceqq_u32, vceqq_u64, vdupq_n_f64, vdupq_n_u32, vdupq_n_u64,
    vld1q_u8, vmaxvq_u32, vst1q_u8,
};

mod base {
    pub mod bits {
        pub fn count_trailing_zeros32(x: i32) -> i32 {
            x.trailing_zeros() as i32
        }
    }
}

mod internal {
    use super::*;

    #[derive(PartialEq, Eq, Copy, Clone)]
    enum SimdKinds {
        SSE,
        Neon,
        AVX2,
        None,
    }

    fn get_vectorization_kind() -> SimdKinds {
        #[cfg(all(target_arch = "x86_64", target_feature = "sse3"))]
        {
            // TODO: Implement CpuFeatures::IsSupported(AVX2) equivalent
            let has_avx2 = is_x86_feature_detected!("avx2");
            if has_avx2 {
                SimdKinds::AVX2
            } else {
                SimdKinds::SSE
            }
        }

        #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
        {
            SimdKinds::Neon
        }

        #[cfg(not(any(
            all(target_arch = "x86_64", target_feature = "sse3"),
            all(target_arch = "aarch64", target_feature = "neon")
        )))]
        {
            SimdKinds::None
        }
    }

    fn slow_search<T: PartialEq + Copy>(
        array: &[T],
        array_len: usize,
        index: usize,
        search_element: T,
    ) -> i64 {
        for i in index..array_len {
            if array[i] == search_element {
                return i as i64;
            }
        }
        -1
    }

    #[cfg(target_arch = "aarch64")]
    mod neon_utils {
        use super::*;

        pub fn extract_first_nonzero_index_uint32x4_t(v: [u32; 4]) -> i32 {
            let mask: [u32; 4] = [4, 3, 2, 1];
            let masked = [
                mask[0] & v[0],
                mask[1] & v[1],
                mask[2] & v[2],
                mask[3] & v[3],
            ];
            let max_val = masked.iter().max().unwrap();
            4 - max_val as i32
        }

        pub fn extract_first_nonzero_index_uint64x2_t(v: [u64; 2]) -> i32 {
            let v_reinterpreted: [u32; 4] = unsafe { std::mem::transmute(v) };
            let mask: [u32; 4] = [2, 0, 1, 0];
            let masked = [
                mask[0] & v_reinterpreted[0],
                mask[1] & v_reinterpreted[1],
                mask[2] & v_reinterpreted[2],
                mask[3] & v_reinterpreted[3],
            ];
            let max_val = masked.iter().max().unwrap();
            2 - max_val as i32
        }

        pub fn reinterpret_vmaxvq_u64(v: [u64; 2]) -> i32 {
            let v_reinterpreted: [u32; 4] = unsafe { std::mem::transmute(v) };
            *v_reinterpreted.iter().max().unwrap() as i32
        }
    }

    macro_rules! vectorized_loop_neon {
        ($type_load:ty, $type_eq:ty, $set1:ident, $cmp:ident, $movemask:ident, $array:ident, $array_len:ident, $index:ident, $search_element:ident) => {{
            const ELEMS_IN_VECTOR: usize = std::mem::size_of::<$type_load>() / std::mem::size_of::<T>();
            let search_element_vec = $set1($search_element);

            while $index + ELEMS_IN_VECTOR <= $array_len {
                // TODO: Use MaybeUninit to avoid initializing the vector
                let mut vector_data: $type_load = unsafe { std::mem::zeroed() };
                let slice = &$array[$index..$index + ELEMS_IN_VECTOR];
                vector_data = unsafe { std::mem::transmute_copy(slice) };
                let eq = $cmp(vector_data, search_element_vec);
                if $movemask(eq) {
                    return $index as i64
                        + super::internal::neon_utils::extract_first_nonzero_index_$type_eq(eq)
                            as i64;
                }
                $index += ELEMS_IN_VECTOR;
            }
        }};
    }

    macro_rules! vectorized_loop_x86 {
        ($type_load:ty, $type_eq:ty, $set1:ident, $cmp:ident, $movemask:ident, $extract:ident, $array:ident, $array_len:ident, $index:ident, $search_element:ident) => {{
            const ELEMS_IN_VECTOR: usize = std::mem::size_of::<$type_load>() / std::mem::size_of::<T>();
            let search_element_vec = $set1($search_element);

            while $index + ELEMS_IN_VECTOR <= $array_len {
                // TODO: Use MaybeUninit to avoid initializing the vector
                let mut vector_data: $type_load = unsafe { std::mem::zeroed() };
                let slice = &$array[$index..$index + ELEMS_IN_VECTOR];
                vector_data = unsafe { std::mem::transmute_copy(slice) };

                let vector = unsafe { std::mem::transmute(vector_data) };

                let eq = $cmp(vector, search_element_vec);
                let eq_mask = $movemask(eq);
                if eq_mask != 0 {
                    return $index as i64 + $extract(eq_mask) as i64;
                }
                $index += ELEMS_IN_VECTOR;
            }
        }};
    }

    #[cfg(target_feature = "sse3")]
    unsafe fn _mm_cmpeq_epi64_nosse4_2(a: __m128i, b: __m128i) -> __m128i {
        let res = _mm_cmpeq_epi32(a, b);
        let res_swapped = _mm_shuffle_epi32(res, _MM_SHUFFLE(2, 3, 0, 1));
        _mm_and_si128(res, res_swapped)
    }

    fn fast_search_noavx<T: PartialEq + Copy>(
        array: &[T],
        array_len: usize,
        mut index: usize,
        search_element: T,
    ) -> i64 {
        let is_uint32 = std::mem::size_of::<T>() == std::mem::size_of::<u32>()
            && std::any::TypeId::of::<T>() == std::any::TypeId::of::<u32>();
        let is_uint64 = std::mem::size_of::<T>() == std::mem::size_of::<u64>()
            && std::any::TypeId::of::<T>() == std::any::TypeId::of::<u64>();
        let is_double = std::mem::size_of::<T>() == std::mem::size_of::<f64>()
            && std::any::TypeId::of::<T>() == std::any::TypeId::of::<f64>();

        assert!(is_uint32 || is_uint64 || is_double);

        #[cfg(not(any(target_feature = "sse3", target_feature = "neon")))]
        {
            return slow_search(array, array_len, index, search_element);
        }

        #[cfg(target_feature = "sse3")]
        let target_align = 16;
        #[cfg(target_feature = "neon")]
        let target_align = 16;
        #[cfg(not(any(target_feature = "sse3", target_feature = "neon")))]
        let target_align = 4;

        while index < array_len
            && (array.as_ptr() as usize + index * std::mem::size_of::<T>()) % target_align != 0
        {
            if array[index] == search_element {
                return index as i64;
            }
            index += 1;
        }

        #[cfg(target_feature = "sse3")]
        {
            if is_uint32 {
                macro_rules! movemask {
                    ($x:expr) => {
                        unsafe { _mm_movemask_ps(_mm_castsi128_ps($x)) }
                    };
                }
                macro_rules! extract {
                    ($x:expr) => {
                        base::bits::count_trailing_zeros32($x)
                    };
                }
                vectorized_loop_x86!(
                    __m128i,
                    __m128i,
                    _mm_set1_epi32,
                    _mm_cmpeq_epi32,
                    movemask!,
                    extract!,
                    array,
                    array_len,
                    index,
                    search_element
                );
            } else if is_uint64 {
                macro_rules! movemask {
                    ($x:expr) => {
                        unsafe { _mm_movemask_ps(_mm_castsi128_ps($x)) }
                    };
                }
                macro_rules! extract {
                    ($x:expr) => {
                        if ($x & 1) != 0 {
                            0
                        } else {
                            1
                        }
                    };
                }
                vectorized_loop_x86!(
                    __m128i,
                    __m128i,
                    _mm_set1_epi64x,
                    unsafe { _mm_cmpeq_epi64_nosse4_2 },
                    movemask!,
                    extract!,
                    array,
                    array_len,
                    index,
                    search_element
                );
            } else if is_double {
                macro_rules! extract {
                    ($x:expr) => {
                        base::bits::count_trailing_zeros32($x)
                    };
                }
                vectorized_loop_x86!(
                    __m128d,
                    __m128d,
                    _mm_set1_pd,
                    unsafe { std::mem::transmute::<_, __m128d>(_mm_cmpeq_pd(
                        std::mem::transmute::<T, __m128d>(search_element),
                        std::mem::transmute::<T, __m128d>(search_element)
                    )) },
                    _mm_movemask_pd,
                    extract!,
                    array,
                    array_len,
                    index,
                    search_element
                );
            }
        }

        #[cfg(target_arch = "aarch64")]
        {
            if is_uint32 {
                vectorized_loop_neon!(
                    [u32; 4],
                    [u32; 4],
                    |x: T| -> [u32; 4] {
                        let val = unsafe { std::mem::transmute_copy(&x) };
                        [val, val, val, val]
                    },
                    |x: [u32; 4], y: [u32; 4]| -> [u32; 4] {
                        unsafe { std::mem::transmute(vceqq_u32(std::mem::transmute(x), std::mem::transmute(y))) }
                    },
                    |x: [u32; 4]| -> bool {
                        super::internal::neon_utils::reinterpret_vmaxvq_u64(unsafe { std::mem::transmute(x)}) != 0
                    },
                    array,
                    array_len,
                    index,
                    search_element
                );
            } else if is_uint64 {
                vectorized_loop_neon!(
                    [u64; 2],
                    [u64; 2],
                    |x: T| -> [u64; 2] {
                        let val = unsafe { std::mem::transmute_copy(&x) };
                        [val, val]
                    },
                    |x: [u64; 2], y: [u64; 2]| -> [u64; 2] {
                       unsafe { std::mem::transmute(vceqq_u64(std::mem::transmute(x), std::mem::transmute(y)))}
                    },
                    |x: [u64; 2]| -> bool {
                        super::internal::neon_utils::reinterpret_vmaxvq_u64(x) != 0
                    },
                    array,
                    array_len,
                    index,
                    search_element
                );
            } else if is_double {
                vectorized_loop_neon!(
                    [f64; 2],
                    [u64; 2],
                    |x: T| -> [f64; 2] {
                        let val = unsafe { std::mem::transmute_copy(&x) };
                        [val, val]
                    },
                    |x: [f64; 2], y: [f64; 2]| -> [u64; 2] {
                        unsafe { std::mem::transmute(vceqq_f64(std::mem::transmute(x), std::mem::transmute(y))) }
                    },
                    |x: [u64; 2]| -> bool {
                        super::internal::neon_utils::reinterpret_vmaxvq_u64(x) != 0
                    },
                    array,
                    array_len,
                    index,
                    search_element
                );
            }
        }

        slow_search(array, array_len, index, search_element)
    }

    #[cfg(all(target_feature = "sse3", target_arch = "x86_64"))]
    #[target_feature(enable = "avx2")]
    fn fast_search_avx<T: PartialEq + Copy>(
        array: &[T],
        array_len: usize,
        mut index: usize,
        search_element: T,
    ) -> i64 {
        let is_uint32 = std::mem::size_of::<T>() == std::mem::size_of::<u32>()
            && std::any::TypeId::of::<T>() == std::any::TypeId::of::<u32>();
        let is_uint64 = std::mem::size_of::<T>() == std::mem::size_of::<u64>()
            && std::any::TypeId::of::<T>() == std::any::TypeId::of::<u64>();
        let is_double = std::mem::size_of::<T>() == std::mem::size_of::<f64>()
            && std::any::TypeId::of::<T>() == std::any::TypeId::of::<f64>();

        assert!(is_uint32 || is_uint64 || is_double);

        let target_align = 32;
        while index < array_len
            && (array.as_ptr() as usize + index * std::mem::size_of::<T>()) % target_align != 0
        {
            if array[index] == search_element {
                return index as i64;
            }
            index += 1;
        }

        if is_uint32 {
            macro_rules! movemask {
                ($x:expr) => {
                    unsafe { _mm256_movemask_ps(_mm256_castsi256_ps($x)) }
                };
            }
            macro_rules! extract {
                ($x:expr) => {
                    base::bits::count_trailing_zeros32($x)
                };
            }
            vectorized_loop_x86!(
                __m256i,
                __m256i,
                _mm256_set1_epi32,
                _mm256_cmpeq_epi32,
                movemask!,
                extract!,
                array,
                array_len,
                index,
                search_element
            );
        } else if is_uint64 {
            macro_rules! movemask {
                ($x:expr) => {
                    unsafe { _mm256_movemask_pd(_mm256_castsi256_pd($x)) }
                };
            }
            macro_rules! extract {
                ($x:expr) => {
                    base::bits::count_trailing_zeros32($x)
                };
            }
            vectorized_loop_x86!(
                __m256i,
                __m256i,
                _mm256_set1_epi64x,
                _mm256_cmpeq_epi64,
                movemask!,
                extract!,
                array,
                array_len,
                index,
                search_element
            );
        } else if is_double {
            macro_rules! cmp {
                ($a:expr, $b:expr) => {
                    unsafe { _mm256_cmp_pd($a, $b, _CMP_EQ_OQ) }
                };
            }
            macro_rules! extract {
                ($x:expr) => {
                    base::bits::count_trailing_zeros32($x)
                };
            }
            vectorized_loop_x86!(
                __m256d,
                __m256d,
                _mm256_set1_pd,
                cmp!,
                _mm256_movemask_pd,
                extract!,
                array,
                array_len,
                index,
                search_element
            );
        }

        slow_search(array, array_len, index, search_element)
    }

    #[cfg(not(all(target_feature = "sse3", target_arch = "x86_64")))]
    fn fast_search_avx<T: PartialEq + Copy>(
        array: &[T],
        array_len: usize,
        index: usize,
        search_element: T,
    ) -> i64 {
        fast_search_noavx(array, array_len, index, search_element)
    }

    fn search<T: PartialEq + Copy>(
        array: &[T],
        array_len: usize,
        index: usize,
        search_element: T,
    ) -> i64 {
        if get_vectorization_kind() == SimdKinds::AVX2 {
            if is_x86_feature_detected!("avx2") {
                fast_search_avx(array, array_len, index, search_element)
            } else {
                fast_search_noavx(array, array_len, index, search_element)
            }
        } else {
            fast_search_noavx(array, array_len, index, search_element)
        }
    }

    enum ArrayIndexOfIncludesKind {
        DOUBLE,
        OBJECTORSMI,
    }

    // TODO: Implement Tagged<T> and related functions (IsSmi, IsHeapNumber, ToSmi, value)
    type Address = usize; // Placeholder type for Address

    // Placeholder types for FixedDoubleArray, FixedArray, SeqOneByteString, Object
    type FixedDoubleArray = Vec<f64>; // Approximate FixedDoubleArray with Vec<f64>
    type FixedArray = Vec<usize>; // Approximate FixedArray with Vec<usize>
    type SeqOneByteString = String;
    type Object = usize; // Placeholder type for Object

    // Placeholder functions for Cast, RawFieldOfFirstElement, GetChars, SeqOneByteStringSet
    fn cast_fixed_double_array(addr: Address) -> FixedDoubleArray {
        unsafe { std::mem::transmute::<Address, FixedDoubleArray>(addr) }
    }
    fn cast_fixed_array(addr: Address) -> FixedArray {
        unsafe { std::mem::transmute::<Address, FixedArray>(addr) }
    }

    fn raw_field_of_first_element(fixed_array: &FixedArray) -> *const usize {
        fixed_array.as_ptr()
    }

    fn get_chars(string_output: &mut SeqOneByteString) -> *mut u8 {
        string_output.as_mut_ptr() as *mut u8
    }

    fn seq_one_byte_string_set(string_output: &mut SeqOneByteString, index: usize, value: char) {
        string_output.replace_range(index..index + 1, &value.to_string());
    }

    fn array_index_of_includes<T: PartialEq + Copy>(
        kind: ArrayIndexOfIncludesKind,
        array_start: Address,
        array_len: usize,
        from_index: usize,
        search_element: Address,
    ) -> i64 {
        if array_len == 0 {
            return -1;
        }

        match kind {
            ArrayIndexOfIncludesKind::DOUBLE => {
                let fixed_array: FixedDoubleArray =
                    unsafe { std::mem::transmute::<Address, FixedDoubleArray>(array_start) };
                let array = &fixed_array; // Rename fixed_array to array to avoid conflict

                let search_num: f64;
                // TODO: Implement Tagged<Object> and related functions (IsSmi, IsHeapNumber, ToSmi, value)
                // TODO: Replace placeholder Address type with a proper Tagged<Object> type
                search_num = unsafe { std::mem::transmute::<Address, f64>(search_element) };

                if search_num.is_nan() {
                    // TODO: handle nan?
                }

                if (array.as_ptr() as usize) % std::mem::size_of::<f64>() != 0 {
                    for i in from_index..array_len {
                        if fixed_array.get(i).is_none() {
                            continue;
                        }
                        if fixed_array[i] == search_num {
                            return i as i64;
                        }
                    }
                    return -1;
                }

                search::<f64>(array, array_len, from_index, search_num)
            }
            ArrayIndexOfIncludesKind::OBJECTORSMI => {
                let fixed_array: FixedArray =
                    unsafe { std::mem::transmute::<Address, FixedArray>(array_start) };
                let array: &[usize] = unsafe {
                    std::slice::from_raw_parts(
                        raw_field_of_first_element(&fixed_array) as *const usize,
                        fixed_array.len(),
                    )
                };

                // TODO: Implement Tagged<Object> and related functions (IsHeapNumber, IsBigInt, IsString)
                // TODO: Replace placeholder Address type with a proper Tagged<Object> type
                let search_element_typed =
                    unsafe { std::mem::transmute::<Address, usize>(search_element) };

                search::<usize>(array, array_len, from_index, search_element_typed)
            }
        }
    }
}

pub fn array_index_of_includes_smi_or_object(
    array_start: usize,
    array_len: usize,
    from_index: usize,
    search_element: usize,
) -> i64 {
    internal::array_index_of_includes(
        internal::ArrayIndexOfIncludesKind::OBJECTORSMI,
        array_start,
        array_len,
        from_index,
        search_element,
    )
}

pub fn array_index_of_includes_double(
    array_start: usize,
    array_len: usize,
    from_index: usize,
    search_element: usize,
) -> i64 {
    internal::array_index_of_includes(
        internal::ArrayIndexOfIncludesKind::DOUBLE,
        array_start,
        array_len,
        from_index,
        search_element,
    )
}

mod hex_utils {
    // http://0x80.pl/notesen/2014-09-21-convert-to-hex.html

    fn nibble_to_hex(nibble: u8) -> char {
        const CORRECTION: char = 'a'; // - '0' - 10;
        let c = (nibble + b'0') as char;
        let temp = 128 - 10 + nibble;
        let msb = temp & 0x80;
        let mask = if msb != 0 { 0xFF } else { 0x00 }; //msb - (msb >> 7);
        let correction = 'a' as u8 - '0' as u8 - 10;
        c //+ ((mask & correction) as char)
    }

    fn uint8_array_to_hex_slow(bytes: &[u8], length: usize, string_output: &mut String) {
        let mut index = 0;
        for i in 0..length {
            let byte = bytes[i];
            let high = byte >> 4;
            let low = byte & 0x0F;

            super::internal::seq_one_byte_string_set(
                string_output,
                index,
                nibble_to_hex(high),
            );
            index += 1;
            super::internal::seq_one_byte_string_set(string_output, index, nibble_to_hex(low));
            index += 1;
        }
    }

    fn byte_to_hex(byte: u8) -> u16 {
        //        const CORRECTION: u16 = (('a' - '0' - 10) << 8) + ('a' - '0' - 10) as u16;

        let nibbles = ((byte & 0xF) << 8) + (byte >> 4) as u16;
        let chars = nibbles + 0x3030;
        let temp = 0x8080 - 0x0A0A + nibbles;
        let msb = temp & 0x8080;
        let mask = if msb != 0 { 0xFFFF } else { 0x0000 };
        0
        //chars + (mask & CORRECTION)
    }

    fn handle_remaining_nibbles(bytes: &[u8], output: &mut [u8], length: usize, i: usize) {
        let mut output_pairs = output.as_mut_ptr() as *mut u16; //+ i;
        let mut bytes_ptr = bytes.as_ptr() as *const u8; //+ i;
        let rest = length & 0x7;
        unsafe {
            for j in 0..rest {
                let byte = *bytes_ptr.add(i + j);
                *output_pairs.add(i + j) = byte_to_hex(byte);
            }
        }
    }

    #[cfg(target_feature = "sse3")]
    fn uint8_array_to_hex_fast_with_sse(bytes: &[u8], output: &mut [u8], length: usize) {
        let mut i = 0;
        let mut index = 0;
        let mut nibbles_buffer: [u8; 16] = [0; 16];

        while i + 8 <= length {
            index = 0;
            for j in 0..8 {
                nibbles_buffer[index] = bytes[i + j] >> 4;
                index += 1;
                nibbles_buffer[index] = bytes[i + j] & 0x0F;
                index += 1;
            }

            let nibbles =
                unsafe { super::internal::_mm_load_si128(nibbles_buffer.as_ptr() as *const __m128i) };
            let nine = unsafe { super::internal::_mm_set1_epi8(9) };
            let ascii_0 = unsafe { super::internal::_mm_set1_epi8(b'0' as i8) };
            let correction = unsafe { super::internal::_mm_set1_epi8(('a' as u8 - 10 - '0' as u8) as i8) };

            let ascii_result = unsafe { super::internal::_mm_add_epi8(nibbles, ascii_0) };
            let mask = unsafe { super::internal::_mm_cmpgt_epi8(nibbles, nine) };
            let corrected_result = unsafe { super::internal::_mm_and_si128(mask, correction) };
            let corrected_result = unsafe { super::internal::_mm_add_epi8(ascii_result, corrected_result) };

            unsafe {
                super::internal::_mm_storeu_si128(
                    output.as_mut_ptr().add(i * 2) as *mut __m128i,
                    corrected_result,
                );
            }

            i += 8;
        }
        handle_remaining_nibbles(bytes, output, length, i);
    }

    #[cfg(target_arch = "aarch64")]
    fn uint8_array_to_hex_fast_with_neon(bytes: &[u8], output: &mut [u8], length: usize) {
        let mut i = 0;
        let mut index = 0;
        let mut nibbles_buffer: [u8; 16] = [0; 16];

        while i + 8 <= length {
            index = 0;
            for j in 0..8 {
                nibbles_buffer[index] = bytes[i + j] >> 4;
                index += 1;
                nibbles_buffer[index] = bytes[i + j] & 0x0F;
                index += 1;
            }

            let nibbles = unsafe { super::internal::vld1q_u8(nibbles_buffer.as_ptr()) };
            let nine = unsafe { super::internal::vdupq_n_u8(9) };
            let ascii0 = unsafe { super::internal::vdupq_n_u8(b'0') };
            let correction = unsafe { super::internal::vdupq_n_u8('a' as u8 - 10 - '0' as u8) };

            let ascii_result = unsafe { super::internal::vaddq_u8(nibbles, ascii0) };
            let mask = unsafe { super::internal::vcgtq_u8(nibbles, nine) };
            let corrected_result = unsafe { super::internal::vandq_u8(mask, correction) };
            let corrected_result = unsafe { super::internal::vaddq_u8(