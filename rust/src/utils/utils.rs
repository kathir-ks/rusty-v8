// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::useless_transmute)]

use std::cmp::{max, min};
use std::f64::NAN;
use std::fmt;
use std::mem::size_of;
use std::ops::{BitAnd, BitOr, BitXor, Shl, Shr, Sub};
use std::os::raw::c_char;
use std::{f64, i32, i64, u32, u64};

// use libc; // Not needed as all OS specific code is removed
use std::string::String;

// use siphash::sip::SipHasher; // Disabled due to missing dependency

// use std::arch::x86_64::*; // Disabled due to SSE3 and AVX features

// Placeholder modules for missing dependencies
mod base {
    pub mod bits {
        pub fn count_leading_zeros(x: usize) -> u32 {
            x.leading_zeros()
        }
    }
    pub trait CheckedNumeric {
        fn safe_add(self, v: Self) -> Option<Self>
        where
            Self: Sized;
        fn safe_sub(self, v: Self) -> Option<Self>
        where
            Self: Sized;
        fn safe_mul(self, v: Self) -> Option<Self>
        where
            Self: Sized;
    }

    impl CheckedNumeric for u64 {
        fn safe_add(self, v: Self) -> Option<Self> {
            self.checked_add(v)
        }
        fn safe_sub(self, v: Self) -> Option<Self> {
            self.checked_sub(v)
        }
        fn safe_mul(self, v: Self) -> Option<Self> {
            self.checked_mul(v)
        }
    }

    pub fn saturated_cast<T>(value: i64) -> T
    where
        T: SaturatedCast<i64>,
    {
        T::saturate(value)
    }

    pub trait SaturatedCast<U> {
        fn saturate(value: U) -> Self;
    }

    macro_rules! impl_saturated_cast {
        ($t:ty) => {
            impl SaturatedCast<i64> for $t {
                fn saturate(value: i64) -> Self {
                    if value > <$t>::MAX as i64 {
                        <$t>::MAX
                    } else if value < <$t>::MIN as i64 {
                        <$t>::MIN
                    } else {
                        value as $t
                    }
                }
            }
        };
    }

    impl_saturated_cast!(i8);
    impl_saturated_cast!(i16);
    impl_saturated_cast!(i32);

    pub type Vector<T> = Vec<T>;
}

mod common {
    pub mod globals {
        pub const kBitsPerByte: usize = 8;
        pub const kMaxUInt64: u64 = u64::MAX;
    }
}

mod wasm {
    pub struct Float16 {}

    impl Float16 {
        pub fn FromFloat32(_f: f32) -> Self {
            Float16 {}
        }
        pub fn ToFloat32(&self) -> f32 {
            0.0
        }
    }
}

// NOTE: Logging and Macros are placeholders and may need better conversion
macro_rules! DCHECK_LE {
    ($left:expr, $right:expr) => {
        if !($left <= $right) {
            panic!("DCHECK_LE failed: {} <= {}", $left, $right);
        }
    };
}

macro_rules! CHECK {
    ($condition:expr) => {
        if !($condition) {
            panic!("CHECK failed: {}", stringify!($condition));
        }
    };
}

macro_rules! UNREACHABLE {
    () => {
        panic!("UNREACHABLE");
    };
}

macro_rules! DCHECK_NOT_NULL {
    ($ptr:expr) => {
        if $ptr.is_null() {
            panic!("DCHECK_NOT_NULL failed: pointer is null");
        }
    };
}

macro_rules! V8_INLINE {
    ($item:item) => {
        #[inline(always)]
        $item
    };
}

macro_rules! V8_CLANG_NO_SANITIZE {
    ($arg:tt) => {
        #[allow(unused_attributes)]
        #[$arg]
    };
}

macro_rules! V8_EXPORT_PRIVATE {
    ($item:item) => {
        $item
    };
}

macro_rules! PRINTF_FORMAT {
    ($($arg:tt)*) => {};
}

macro_rules! V8_NOINLINE {
    () => {
        #[inline(never)]
    }
}

pub mod internal {
    use super::*;
    use std::fmt;
    use std::fmt::Write;
    use std::mem::transmute;
    use std::ops::{BitAnd, BitOr, BitXor, Shl, Shr, Sub};
    use std::os::raw::c_char;
    use std::ptr;
    use std::string::String;

    /// Simulates arithmetic right shift.
    pub fn arithmetic_shift_right<T>(x: T, shift: i32) -> T
    where
        T: Copy
            + PartialOrd
            + BitAnd<Output = T>
            + Shr<i32, Output = T>
            + BitOr<Output = T>
            + From<u8>,
        u64: From<T>,
    {
        DCHECK_LE!(0, shift);
        if x < T::from(0) {
            // Right shift of signed values is implementation defined. Simulate a
            // true arithmetic right shift by adding leading sign bits.
            let mask: u64 = !0 >> shift;
            let unsigned_x: u64 = u64::from(x);
            ((unsigned_x >> shift) | mask) as T
        } else {
            x >> shift
        }
    }

    /// Returns the maximum of the two parameters according to JavaScript semantics.
    pub fn js_max<T>(x: T, y: T) -> T
    where
        T: Copy + PartialOrd + std::fmt::Debug,
    {
        if x != x {
            return x;
        }
        if y != y {
            return y;
        }
        if (x < T::from(0) && y >= T::from(0)) {
            return x;
        }
        if x > y {
            x
        } else {
            y
        }
    }

    /// Returns the minimum of the two parameters according to JavaScript semantics.
    pub fn js_min<T>(x: T, y: T) -> T
    where
        T: Copy + PartialOrd + std::fmt::Debug,
    {
        if x != x {
            return x;
        }
        if y != y {
            return y;
        }
        if (x < T::from(0) && y >= T::from(0)) {
            return y;
        }
        if x > y {
            y
        } else {
            x
        }
    }

    /// Returns the absolute value of its argument.
    pub fn abs<T>(a: T) -> T
    where
        T: Copy + std::ops::Shr<usize, Output = T> + std::ops::BitXor<Output = T> + std::ops::Sub<Output = T> + std::convert::TryInto<usize> + std::convert::From<u8>,
    {
        // This is a branch-free implementation of the absolute value function and is
        // described in Warren's "Hacker's Delight", chapter 2. It avoids undefined
        // behavior with the arithmetic negation operation on signed values as well.
        let size_of_t = std::mem::size_of::<T>();
        let shift_amount: usize = (size_of_t * 8 - 1).try_into().unwrap();
        let x = a;
        let y = a >> shift_amount;
        (x ^ y) - y
    }

    pub fn modulo(x: f64, y: f64) -> f64 {
        #[cfg(target_os = "windows")]
        {
            // Workaround MS fmod bugs. ECMA-262 says:
            // dividend is finite and divisor is an infinity => result equals dividend
            // dividend is a zero and divisor is nonzero finite => result equals dividend
            if !(x.is_finite() && (!y.is_finite() && !y.is_nan()))
                && !(x == 0.0 && (y != 0.0 && y.is_finite()))
            {
                let mut result = x % y;
                // Workaround MS bug in VS CRT in some OS versions, https://crbug.com/915045
                // fmod(-17, +/-1) should equal -0.0 but now returns 0.0.
                if x < 0.0 && result == 0.0 {
                    result = -0.0;
                }
                result
            } else {
                x
            }
        }
        #[cfg(target_os = "aix")]
        {
            // AIX raises an underflow exception for (Number.MIN_VALUE % Number.MAX_VALUE)
            // NOTE: `feclearexcept` and `fetestexcept` are not available in Rust standard library.
            // A direct translation would require using `libc` and unsafe code.

            // This is a placeholder implementation that always returns `std::fmod(x, y)`.
            // A complete implementation would require handling floating-point exceptions,
            // which are platform-specific and require `unsafe` code.
            x % y
        }
        #[cfg(not(any(target_os = "windows", target_os = "aix")))]
        {
            x % y
        }
    }

    pub fn saturate_add<T>(a: T, b: T) -> T
    where
        T: Copy + std::ops::Add<Output = T> + PartialOrd + std::fmt::Debug,
    {
        if a > T::from(0) && b > T::from(0) {
            if a > T::max_value() - b {
                return T::max_value();
            }
        } else if a < T::from(0) && b < T::from(0) {
            if a < T::min_value() - b {
                return T::min_value();
            }
        }
        a + b
    }

    pub fn saturate_sub<T>(a: T, b: T) -> T
    where
        T: Copy + std::ops::Sub<Output = T> + PartialOrd + std::fmt::Debug,
    {
        if a >= T::from(0) && b < T::from(0) {
            if a > T::max_value() + b {
                return T::max_value();
            }
        } else if a < T::from(0) && b > T::from(0) {
            if a < T::min_value() + b {
                return T::min_value();
            }
        } else if a < b {
            return T::from(0);
        }
        a - b
    }

    pub fn saturate_rounding_qmul<T>(a: T, b: T) -> T
    where
        T: Copy
            + std::ops::Mul<Output = i64>
            + std::ops::AddAssign
            + std::ops::Shr<usize, Output = i64>
            + std::fmt::Debug,
    {
        // Saturating rounding multiplication for Q-format numbers. See
        // https://en.wikipedia.org/wiki/Q_(number_format) for a description.
        // Specifically this supports Q7, Q15, and Q31. This follows the
        // implementation in simulator-logic-arm64.cc (sqrdmulh) to avoid overflow
        // when a == b == int32 min.

        let size_in_bits = std::mem::size_of::<T>() * 8;
        let round_const: i64 = 1 << (size_in_bits - 2);
        let mut product: i64 = (a as i64) * (b as i64);
        product += round_const;
        product >>= size_in_bits - 1;
        base::saturated_cast(product)
    }

    /// Multiply two numbers, returning a result that is twice as wide, no overflow.
    /// Put Wide first so we can use function template argument deduction for Narrow,
    /// and callers can provide only Wide.
    pub fn multiply_long<Wide, Narrow>(a: Narrow, b: Narrow) -> Wide
    where
        Wide: From<Narrow> + std::ops::Mul<Output = Wide>,
        Narrow: Copy,
    {
        (Wide::from(a)) * (Wide::from(b))
    }

    /// Add two numbers, returning a result that is twice as wide, no overflow.
    /// Put Wide first so we can use function template argument deduction for Narrow,
    /// and callers can provide only Wide.
    pub fn add_long<Wide, Narrow>(a: Narrow, b: Narrow) -> Wide
    where
        Wide: From<Narrow> + std::ops::Add<Output = Wide>,
        Narrow: Copy,
    {
        (Wide::from(a)) + (Wide::from(b))
    }

    pub fn rounding_average_unsigned<T>(a: T, b: T) -> T
    where
        T: Copy + Into<u64>,
    {
        ((a.into() as u64) + (b.into() as u64) + 1) as T >> 1
    }

    // Helper macros for defining a contiguous sequence of field offset constants.
    // Example: (backslashes at the ends of respective lines of this multi-line
    // macro definition are omitted here to please the compiler)
    //
    // #define MAP_FIELDS(V)
    //   V(kField1Offset, kTaggedSize)
    //   V(kField2Offset, kIntSize)
    //   V(kField3Offset, kIntSize)
    //   V(kField4Offset, kSystemPointerSize)
    //   V(kSize, 0)
    //
    // DEFINE_FIELD_OFFSET_CONSTANTS(HeapObject::kHeaderSize, MAP_FIELDS)
    //
    macro_rules! define_one_field_offset {
        ($Name:ident, $Size:expr) => {
            pub const $Name: usize = 0; // Placeholder value
            pub const $Name##End: usize = $Name + ($Size) - 1;
        };
    }

    macro_rules! define_field_offset_constants {
        ($StartOffset:expr, $list_macro:ident) => {
            pub const $list_macro##_StartOffset: usize = $StartOffset - 1;
            $list_macro!(define_one_field_offset);
        };
    }

    macro_rules! define_one_field_offset_pure_name {
        ($CamelName:ident, $Size:expr) => {
            pub const k##$CamelName##Offset: usize = 0; // Placeholder value
            pub const k##$CamelName##OffsetEnd: usize = k##$CamelName##Offset + ($Size) - 1;
        };
    }

    macro_rules! define_field_offset_constants_with_pure_name {
        ($StartOffset:expr, $list_macro:ident) => {
            pub const $list_macro##_StartOffset: usize = $StartOffset - 1;
            $list_macro!(define_one_field_offset_pure_name);
        };
    }

    // Size of the field defined by DEFINE_FIELD_OFFSET_CONSTANTS
    macro_rules! field_size {
        ($Name:ident) => {
            $Name##End + 1 - $Name
        };
    }

    // Compare two offsets with static cast
    macro_rules! static_assert_field_offsets_equal {
        ($Offset1:ident, $Offset2:expr) => {
            const _: () = assert!(
                $Offset1 as usize == $Offset2,
                "Field offsets are not equal"
            );
        };
    }
    // ----------------------------------------------------------------------------
    // Hash function.

    pub const K_ZERO_HASH_SEED: u64 = 0;

    // Thomas Wang, Integer Hash Functions.
    // http://www.concentric.net/~Ttwang/tech/inthash.htm`
    #[inline]
    pub fn compute_unseeded_hash(key: u32) -> u32 {
        let mut hash = key;
        hash = !hash + (hash << 15); // hash = (hash << 15) - hash - 1;
        hash = hash ^ (hash >> 12);
        hash = hash + (hash << 2);
        hash = hash ^ (hash >> 4);
        hash = hash * 2057; // hash = (hash + (hash << 3)) + (hash << 11);
        hash = hash ^ (hash >> 16);
        hash & 0x3fffffff
    }

    #[inline]
    pub fn compute_long_hash(key: u64) -> u32 {
        let mut hash = key;
        hash = !hash + (hash << 18); // hash = (hash << 18) - hash - 1;
        hash = hash ^ (hash >> 31);
        hash = hash * 21; // hash = (hash + (hash << 2)) + (hash << 4);
        hash = hash ^ (hash >> 11);
        hash = hash + (hash << 6);
        hash = hash ^ (hash >> 22);
        (hash & 0x3fffffff) as u32
    }

    #[inline]
    pub fn compute_seeded_hash(key: u32, seed: u64) -> u32 {
        // #[cfg(feature = "V8_USE_SIPHASH")]
        // {
        //     halfsiphash(key, seed)
        // }
        // #[cfg(not(feature = "V8_USE_SIPHASH"))]
        {
            compute_long_hash((key as u64) ^ seed)
        }
    }

    #[inline]
    pub fn compute_pointer_hash(ptr: *mut std::ffi::c_void) -> u32 {
        compute_unseeded_hash(ptr as usize as u32)
    }

    #[inline]
    pub fn compute_address_hash(address: usize) -> u32 {
        compute_unseeded_hash((address & 0xFFFFFFFF) as u32)
    }

    // ----------------------------------------------------------------------------
    // Miscellaneous

    // Memory offset for lower and higher bits in a 64 bit integer.
    #[cfg(target_endian = "little")]
    pub const K_INT64_LOWER_HALF_MEMORY_OFFSET: i32 = 0;
    #[cfg(target_endian = "little")]
    pub const K_INT64_UPPER_HALF_MEMORY_OFFSET: i32 = 4;

    #[cfg(target_endian = "big")]
    pub const K_INT64_LOWER_HALF_MEMORY_OFFSET: i32 = 4;
    #[cfg(target_endian = "big")]
    pub const K_INT64_UPPER_HALF_MEMORY_OFFSET: i32 = 0;

    /// A pointer that can only be set once and doesn't allow NULL values.
    pub struct SetOncePointer<T> {
        pointer_: *mut T,
    }

    impl<T> SetOncePointer<T> {
        pub fn new() -> Self {
            SetOncePointer {
                pointer_: ptr::null_mut(),
            }
        }

        pub fn is_set(&self) -> bool {
            !self.pointer_.is_null()
        }

        pub fn get(&self) -> &mut T {
            DCHECK_NOT_NULL!(self.pointer_);
            unsafe { &mut *self.pointer_ }
        }

        pub fn set(&mut self, value: *mut T) {
            if !self.pointer_.is_null() {
                panic!("Pointer was already set");
            }
            if value.is_null() {
                panic!("Value cannot be NULL");
            }
            self.pointer_ = value;
        }
    }

    impl<T> std::ops::Deref for SetOncePointer<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            self.get()
        }
    }

    impl<T> std::ops::DerefMut for SetOncePointer<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { &mut *self.pointer_ }
        }
    }

    impl<T> PartialEq<std::ffi::c_void> for SetOncePointer<T> {
        fn eq(&self, other: &std::ffi::c_void) -> bool {
            self.pointer_ == other
        }
    }

    impl<T> Eq for SetOncePointer<T> {}

    impl<T> fmt::Debug for SetOncePointer<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SetOncePointer")
                .field("pointer_", &self.pointer_)
                .finish()
        }
    }

    // #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    // {
    //     fn simd_mem_equal<Char>(lhs: *const Char, rhs: *const Char, count: usize, order: usize) -> bool {
    //         false
    //     }
    // }
    //
    // #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
    // {
    //     fn simd_mem_equal<Char>(lhs: *const Char, rhs: *const Char, count: usize, order: usize) -> bool {
    //         false
    //     }
    // }

    /// Compare 8bit/16bit chars to 8bit/16bit chars.
    pub fn compare_chars_equal_unsigned<lchar, rchar>(lhs: *const lchar, rhs: *const rchar, chars: usize) -> bool {
        if std::mem::size_of::<lchar>() == std::mem::size_of::<rchar>() {
            // memcmp compares byte-by-byte, but for equality it doesn't matter whether
            // two-byte char comparison is little- or big-endian.
            unsafe {
                std::slice::from_raw_parts(lhs as *const u8, chars * std::mem::size_of::<lchar>())
                    == std::slice::from_raw_parts(rhs as *const u8, chars * std::mem::size_of::<rchar>())
            }
        } else {
            for i in 0..chars {
                unsafe {
                    let l = *lhs.add(i);
                    let r = *rhs.add(i);
                    if l as u32 != r as u32 {
                        return false;
                    }
                }
            }
            true
        }
    }

    pub fn compare_chars_equal<lchar, rchar>(lhs: *const lchar, rhs: *const rchar, chars: usize) -> bool {
        compare_chars_equal_unsigned(lhs as *const lchar, rhs as *const rchar, chars)
    }

    /// Compare 8bit/16bit chars to 8bit/16bit chars.
    pub fn compare_chars_unsigned<lchar, rchar>(lhs: *const lchar, rhs: *const rchar, chars: usize) -> i32 {
        if std::mem::size_of::<lchar>() == std::mem::size_of::<char>() {
            // memcmp compares byte-by-byte, yielding wrong results for two-byte
            // strings on little-endian systems.
            let l_slice = unsafe { std::slice::from_raw_parts(lhs as *const u8, chars) };
            let r_slice = unsafe { std::slice::from_raw_parts(rhs as *const u8, chars) };
            for i in 0..chars {
                let r = (l_slice[i] as i32) - (r_slice[i] as i32);
                if r != 0 {
                    return r;
                }
            }
            0
        } else {
            for i in 0..chars {
                unsafe {
                    let l = *lhs.add(i);
                    let r = *rhs.add(i);
                    let result = (l as i32) - (r as i32);
                    if result != 0 {
                        return result;
                    }
                }
            }
            0
        }
    }

    pub fn compare_chars<lchar, rchar>(lhs: *const lchar, rhs: *const rchar, chars: usize) -> i32 {
        compare_chars_unsigned(lhs as *const lchar, rhs as *const rchar, chars)
    }

    /// Calculate 10^exponent.
    pub const fn ten_to_the(exponent: u32) -> u64 {
        DCHECK_LE!(exponent, 19);
        DCHECK_LE!(0, exponent);
        let mut answer: u64 = 1;
        let mut i: u32 = 0;
        while i < exponent {
            answer *= 10;
            i += 1;
        }
        answer
    }

    // Bit field extraction.
    #[inline]
    pub fn unsigned_bitextract_32(msb: i32, lsb: i32, x: u32) -> u32 {
        (x >> lsb) & ((1 << (1 + msb - lsb)) - 1)
    }

    #[inline]
    pub fn unsigned_bitextract_64(msb: i32, lsb: i32, x: u64) -> u64 {
        (x >> lsb) & (((1 as u64) << (1 + msb - lsb)) - 1)
    }

    #[inline]
    pub fn signed_bitextract_32(msb: i32, lsb: i32, x: u32) -> i32 {
        (x << (31 - msb)) as i32 >> (lsb + 31 - msb)
    }

    /// Check number width.
    pub const fn is_intn(x: i64, n: u32) -> bool {
        DCHECK_LE!(1, n);
        DCHECK_LE!(n, 63);
        let limit: i64 = 1 << (n - 1);
        (-limit <= x) && (x < limit)
    }

    pub const fn is_uintn(x: i64, n: u32) -> bool {
        DCHECK_LE!(1, n);
        DCHECK_LE!(n, (std::mem::size_of::<i64>() * 8) as u32);
        !(x >> n != 0)
    }

    pub fn truncate_to_intn<T>(x: T, n: u32) -> T
    where
        T: BitAnd<Output = T> + From<i64>,
    {
        DCHECK_LE!(1, n);
        DCHECK_LE!(n, (std::mem::size_of::<T>() * 8) as u32);
        x & (((1 as i64) << n) - 1).into()
    }

    macro_rules! declare_is_int_n {
        ($n:expr) => {
            pub const fn is_int##$n(x: i64) -> bool {
                is_intn(x, $n)
            }
        };
    }

    macro_rules! declare_is_uint_n {
        ($n:expr) => {
            pub const fn is_uint##$n<T>(x: T) -> bool
            where
                T: Into<i64> + Copy,
            {
                is_uintn(x.into(), $n)
            }
        };
    }

    macro_rules! declare_truncate_to_int_n {
        ($n:expr) => {
            pub fn truncate_to_int##$n<T>(x: T) -> T
            where
                T: BitAnd<Output = T> + From<i64> + Copy,
            {
                truncate_to_intn(x, $n)
            }
        };
    }

    macro_rules! declare_checked_truncate_to_int_n {
        ($n:expr) => {
            pub fn checked_truncate_to_int##$n<T>(x: T) -> T
            where
                T: BitAnd<Output = T> + From<i64> + Copy + std::fmt::Debug,
            {
                CHECK!(is_intn(x.into(), $n));
                truncate_to_intn(x, $n)
            }
        };
    }

    macro_rules! int_1_to_63_list {
        ($macro:ident) => {
            $macro!(1);
            $macro!(2);
            $macro!(3);
            $macro!(4);
            $macro!(5);
            $macro!(6);
            $macro!(7);
            $macro!(8);
            $macro!(9);
            $macro!(10);
            $macro!(11);
            $macro!(12);
            $macro!(13);
            $macro!(14);
            $macro!(15);
            $macro!(16);
            $macro!(17);
            $macro!(18);
            $macro!(19);
            $macro!(20);
            $macro!(21);
            $macro!(22);
            $macro!(23);
            $macro!(24);
            $macro!(25);
            $macro!(26);
            $macro!(27);
            $macro!(28);
            $macro!(29);
            $macro!(30);
            $macro!(31);
            $macro!(32);
            $macro!(33);
            $macro!(34);
            $macro!(35);
            $macro!(36);
            $macro!(37);
            $macro!(38);
            $macro!(39);
            $macro!(40);
            $macro!(41);
            $macro!(42);
            $macro!(43);
            $macro!(44);
            $macro!(45);
            $macro!(46);
            $macro!(47);
            $macro!(48);
            $macro!(49);
            $macro!(50);
            $macro!(51);
            $macro!(52);
            $macro!(53);
            $macro!(54);
            $macro!(55);
            $macro!(56);
            $macro!(57);
            $macro!(58);
            $macro!(59);
            $macro!(60);
            $macro!(61);
            $macro!(62);
            $macro!(63);
        };
    }

    int_1_to_63_list!(declare_is_int_n);
    int_1_to_63_list!(declare_is_uint_n);
    int_1_to_63_list!(declare_truncate_to_int_n);
    int_1_to_63_list!(declare_checked_truncate_to_int_n);

    macro_rules! int_0_to_127_list {
        ($macro:ident) => {
            $macro!(0);
            $macro!(1);
            $macro!(2);
            $macro!(3);
            $macro!(4);
            $macro!(5);
            $macro!(6);
            $macro!(7);
            $macro!(8);
            $macro!(9);
            $macro!(10);
            $macro!(11);
            $macro!(12);
            $macro!(13);
            $macro!(14);
            $macro!(15);
            $macro!(16);
            $macro!(17);
            $macro!(18);
            $macro!(19);
            $macro!(20);
            $macro!(21);
            $macro!(22);
            $macro!(23);
            $macro!(24);
            $macro!(25);
            $macro!(26);
            $macro!(27);
            $macro!(28);
            $macro!(29);
            $macro!(30);
            $macro!(31);
            $macro!(32);
            $macro!(33);
            $macro!(34);
            $macro!(35);
            $macro!(36);
            $macro!(37);
            $macro!(38);
            $macro!(39);
            $macro!(40);
            $macro!(41);
            $macro!(42);
            $macro!(43);