// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod bits {
    use std::{
        convert::TryInto,
        fmt,
        i32,
        i64,
        mem::transmute,
        num::Wrapping,
        ops::{Add, Neg, Sub},
        usize,
    };

    /// CountPopulation(value) returns the number of bits set in |value|.
    pub const fn count_population<T>(value: T) -> u32
    where
        T: num_traits::Unsigned + num_traits::PrimInt,
    {
        if std::mem::size_of::<T>() > 8 {
            panic!("sizeof(T) <= 8");
        }

        #[cfg(target_arch = "x86_64")]
        {
            if std::mem::size_of::<T>() == 8 {
                (value as u64).count_ones()
            } else {
                (value as u32).count_ones()
            }
        }

        #[cfg(not(target_arch = "x86_64"))]
        {
            // Fall back to divide-and-conquer popcount (see "Hacker's Delight" by Henry
            // S. Warren, Jr.), chapter 5-1.
            const MASK: [u64; 3] = [
                0x5555555555555555,
                0x3333333333333333,
                0x0f0f0f0f0f0f0f0f,
            ];

            let mut value = value.to_u64().unwrap();
            // Start with 64 buckets of 1 bits, holding values from [0,1].
            value = ((value >> 1) & MASK[0]) + (value & MASK[0]);
            // Having 32 buckets of 2 bits, holding values from [0,2] now.
            value = ((value >> 2) & MASK[1]) + (value & MASK[1]);
            // Having 16 buckets of 4 bits, holding values from [0,4] now.
            value = ((value >> 4) & MASK[2]) + (value & MASK[2]);
            // Having 8 buckets of 8 bits, holding values from [0,8] now.
            // From this point on, the buckets are bigger than the number of bits
            // required to hold the values, and the buckets are bigger the maximum
            // result, so there's no need to mask value anymore, since there's no
            // more risk of overflow between buckets.
            if std::mem::size_of::<T>() > 1 {
                value = (value >> (if std::mem::size_of::<T>() > 1 {
                    8
                } else {
                    0
                })) + value;
            }
            // Having 4 buckets of 16 bits, holding values from [0,16] now.
            if std::mem::size_of::<T>() > 2 {
                value = (value >> (if std::mem::size_of::<T>() > 2 {
                    16
                } else {
                    0
                })) + value;
            }
            // Having 2 buckets of 32 bits, holding values from [0,32] now.
            if std::mem::size_of::<T>() > 4 {
                value = (value >> (if std::mem::size_of::<T>() > 4 {
                    32
                } else {
                    0
                })) + value;
            }
            // Having 1 buckets of 64 bits, holding values from [0,64] now.
            value as u32 & 0xff
        }
    }

    /// ReverseBits(value) returns |value| in reverse bit order.
    pub fn reverse_bits<T>(value: T) -> T
    where
        T: num_traits::PrimInt + num_traits::Unsigned,
    {
        assert!(
            std::mem::size_of::<T>() == 1
                || std::mem::size_of::<T>() == 2
                || std::mem::size_of::<T>() == 4
                || std::mem::size_of::<T>() == 8
        );
        let mut result: T = T::zero();
        let size = std::mem::size_of::<T>() * 8;
        let mut value = value;
        for _i in 0..size {
            result = (result << 1) | (value & T::one());
            value = value >> 1;
        }
        result
    }

    /// ReverseBytes(value) returns |value| in reverse byte order.
    pub fn reverse_bytes<T>(value: T) -> T
    where
        T: num_traits::PrimInt + num_traits::Unsigned,
    {
        assert!(
            std::mem::size_of::<T>() == 1
                || std::mem::size_of::<T>() == 2
                || std::mem::size_of::<T>() == 4
                || std::mem::size_of::<T>() == 8
        );
        let mut result: T = T::zero();
        let size = std::mem::size_of::<T>();
        let mut value = value;
        for _i in 0..size {
            result = (result << 8) | (value & T::from(0xff).unwrap());
            value = value >> 8;
        }
        result
    }

    pub const fn unsigned<T>(value: T) -> <T as num_traits::AsPrimitive<<T as num_traits::PrimInt>::Unsigned>>::Output
    where T: num_traits::PrimInt + num_traits::Signed,
        <T as num_traits::PrimInt>::Unsigned : num_traits::PrimInt
    {
        value.as_()
    }

    pub const fn signed<T>(value: T) -> <T as num_traits::AsPrimitive<<T as num_traits::PrimInt>::Signed>>::Output
    where T: num_traits::PrimInt + num_traits::Unsigned,
        <T as num_traits::PrimInt>::Signed : num_traits::PrimInt
    {
        value.as_()
    }

    /// CountLeadingZeros(value) returns the number of zero bits following the most
    /// significant 1 bit in |value| if |value| is non-zero, otherwise it returns
    /// {sizeof(T) * 8}.
    pub const fn count_leading_zeros<T, const BITS: u32>(value: T) -> u32
    where
        T: num_traits::Unsigned + num_traits::PrimInt,
    {
        if BITS == 0 {
            panic!("invalid instantiation");
        }
        #[cfg(target_arch = "x86_64")]
        {
            if value == T::zero() {
                BITS
            } else if BITS == 64 {
                (value.to_u64().unwrap()).leading_zeros()
            } else {
                (value.to_u32().unwrap()).leading_zeros() - (32 - BITS)
            }
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            // Binary search algorithm taken from "Hacker's Delight" (by Henry S. Warren,
            // Jr.), figures 5-11 and 5-12.
            if BITS == 1 {
                return (value == T::zero()) as u32 ^ 1;
            }
            let upper_half = value >> (BITS / 2);
            let next_value = if upper_half != T::zero() {
                upper_half
            } else {
                value
            };
            let add = if upper_half != T::zero() {
                0
            } else {
                BITS / 2
            };
            let next_bits = if BITS == 1 { 1 } else { BITS / 2 };
            count_leading_zeros::<T, { next_bits }>(next_value) + add
        }
    }

    pub const fn count_leading_zeros32(value: u32) -> u32 {
        count_leading_zeros::<u32, { (std::mem::size_of::<u32>() * 8) as u32 }>(value)
    }

    pub const fn count_leading_zeros64(value: u64) -> u32 {
        count_leading_zeros::<u64, { (std::mem::size_of::<u64>() * 8) as u32 }>(value)
    }

    /// The number of leading zeros for a positive number,
    /// the number of leading ones for a negative number.
    pub const fn count_leading_sign_bits<T>(value: T) -> u32
    where T: num_traits::PrimInt + num_traits::Signed,
    {
        if value < T::zero() {
            count_leading_zeros::< <T as num_traits::PrimInt>::Unsigned, { (std::mem::size_of::< <T as num_traits::PrimInt>::Unsigned >() * 8) as u32 }>(!unsigned(value))
        } else {
            count_leading_zeros::< <T as num_traits::PrimInt>::Unsigned, { (std::mem::size_of::< <T as num_traits::PrimInt>::Unsigned >() * 8) as u32 }>(unsigned(value))
        }
    }

    /// CountTrailingZeros(value) returns the number of zero bits preceding the
    /// least significant 1 bit in |value| if |value| is non-zero, otherwise it
    /// returns {sizeof(T) * 8}.
    /// See CountTrailingZerosNonZero for an optimized version for the case that
    /// |value| is guaranteed to be non-zero.
    pub const fn count_trailing_zeros<T, const BITS: u32>(value: T) -> u32
    where
        T: num_traits::PrimInt + num_traits::Unsigned,
    {
        #[cfg(target_arch = "x86_64")]
        {
            if value == T::zero() {
                BITS
            } else {
                (value.to_u64().unwrap()).trailing_zeros()
            }
        }

        #[cfg(not(target_arch = "x86_64"))]
        {
            // Fall back to popcount (see "Hacker's Delight" by Henry S. Warren, Jr.),
            // chapter 5-4. On x64, since is faster than counting in a loop and faster
            // than doing binary search.
            let u: <T as num_traits::PrimInt>::Unsigned = value;
            count_population(!(u & (u - T::one())))
        }
    }

    pub const fn count_trailing_zeros32(value: u32) -> u32 {
        count_trailing_zeros::<u32, { (std::mem::size_of::<u32>() * 8) as u32 }>(value)
    }

    pub const fn count_trailing_zeros64(value: u64) -> u32 {
        count_trailing_zeros::<u64, { (std::mem::size_of::<u64>() * 8) as u32 }>(value)
    }

    /// CountTrailingZerosNonZero(value) returns the number of zero bits preceding
    /// the least significant 1 bit in |value| if |value| is non-zero, otherwise the
    /// behavior is undefined.
    /// See CountTrailingZeros for an alternative version that allows |value| == 0.
    pub const fn count_trailing_zeros_non_zero<T, const BITS: u32>(value: T) -> u32
    where
        T: num_traits::PrimInt + num_traits::Unsigned,
    {
        assert_ne!(T::zero(), value);
        #[cfg(target_arch = "x86_64")]
        {
            (value.to_u64().unwrap()).trailing_zeros()
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            count_trailing_zeros::<T, BITS>(value)
        }
    }

    /// Returns true iff |value| is a power of 2.
    pub const fn is_power_of_two<T>(value: T) -> bool
    where
        T: num_traits::PrimInt + num_traits::Unsigned,
    {
        value > T::zero() && (value & (value - T::one())) == T::zero()
    }

    /// Identical to {CountTrailingZeros}, but only works for powers of 2.
    pub fn which_power_of_two<T>(value: T) -> u32
    where
        T: num_traits::PrimInt + num_traits::Unsigned,
    {
        assert!(is_power_of_two(value));
        #[cfg(target_arch = "x86_64")]
        {
            if std::mem::size_of::<T>() == 8 {
                (value as u64).trailing_zeros()
            } else {
                (value as u32).trailing_zeros()
            }
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            let u: <T as num_traits::PrimInt>::Unsigned = value;
            count_population(u - T::one())
        }
    }

    /// RoundUpToPowerOfTwo32(value) returns the smallest power of two which is
    /// greater than or equal to |value|. If you pass in a |value| that is already a
    /// power of two, it is returned as is. |value| must be less than or equal to
    /// 0x80000000u. Uses computation based on leading zeros if we have compiler
    /// support for that. Falls back to the implementation from "Hacker's Delight" by
    /// Henry S. Warren, Jr., figure 3-3, page 48, where the function is called clp2.
    pub const fn round_up_to_power_of_two32(mut value: u32) -> u32 {
        assert!(value <= (1u32 << 31));
        if value != 0 {
            value -= 1;
        }
        #[cfg(target_arch = "x86_64")]
        {
            1u32 << (32 - count_leading_zeros(value))
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            value |= value >> 1;
            value |= value >> 2;
            value |= value >> 4;
            value |= value >> 8;
            value |= value >> 16;
            value + 1
        }
    }

    /// Same for 64 bit integers. |value| must be <= 2^63
    pub const fn round_up_to_power_of_two64(mut value: u64) -> u64 {
        assert!(value <= (1u64 << 63));
        if value != 0 {
            value -= 1;
        }
        #[cfg(target_arch = "x86_64")]
        {
            1u64 << (64 - count_leading_zeros(value))
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            value |= value >> 1;
            value |= value >> 2;
            value |= value >> 4;
            value |= value >> 8;
            value |= value >> 16;
            value |= value >> 32;
            value + 1
        }
    }

    /// Same for size_t integers.
    pub const fn round_up_to_power_of_two(value: usize) -> usize {
        if std::mem::size_of::<usize>() == std::mem::size_of::<u64>() {
            round_up_to_power_of_two64(value as u64) as usize
        } else {
            round_up_to_power_of_two32(value as u32) as usize
        }
    }

    /// RoundDownToPowerOfTwo32(value) returns the greatest power of two which is
    /// less than or equal to |value|. If you pass in a |value| that is already a
    /// power of two, it is returned as is.
    pub fn round_down_to_power_of_two32(value: u32) -> u32 {
        if value > 0x80000000u32 {
            return 0x80000000u32;
        }
        let mut result = round_up_to_power_of_two32(value);
        if result > value {
            result >>= 1;
        }
        result
    }

    /// Precondition: 0 <= shift < 32
    pub const fn rotate_right32(value: u32, shift: u32) -> u32 {
        (value >> shift) | (value << ((32 - shift) & 31))
    }

    /// Precondition: 0 <= shift < 32
    pub const fn rotate_left32(value: u32, shift: u32) -> u32 {
        (value << shift) | (value >> ((32 - shift) & 31))
    }

    /// Precondition: 0 <= shift < 64
    pub const fn rotate_right64(value: u64, shift: u64) -> u64 {
        (value >> shift) | (value << ((64 - shift) & 63))
    }

    /// Precondition: 0 <= shift < 64
    pub const fn rotate_left64(value: u64, shift: u64) -> u64 {
        (value << shift) | (value >> ((64 - shift) & 63))
    }

    /// SignedAddOverflow32(lhs,rhs,val) performs a signed summation of |lhs| and
    /// |rhs| and stores the result into the variable pointed to by |val| and
    /// returns true if the signed summation resulted in an overflow.
    pub fn signed_add_overflow32(lhs: i32, rhs: i32) -> (i32, bool) {
        let (res, overflow) = lhs.overflowing_add(rhs);
        (res, overflow)
    }

    /// SignedSubOverflow32(lhs,rhs,val) performs a signed subtraction of |lhs| and
    /// |rhs| and stores the result into the variable pointed to by |val| and
    /// returns true if the signed subtraction resulted in an overflow.
    pub fn signed_sub_overflow32(lhs: i32, rhs: i32) -> (i32, bool) {
        let (res, overflow) = lhs.overflowing_sub(rhs);
        (res, overflow)
    }

    /// SignedMulOverflow32(lhs,rhs,val) performs a signed multiplication of |lhs|
    /// and |rhs| and stores the result into the variable pointed to by |val| and
    /// returns true if the signed multiplication resulted in an overflow.
    pub fn signed_mul_overflow32(lhs: i32, rhs: i32) -> (i32, bool) {
        let (res, overflow) = lhs.overflowing_mul(rhs);
        (res, overflow)
    }

    /// SignedAddOverflow64(lhs,rhs,val) performs a signed summation of |lhs| and
    /// |rhs| and stores the result into the variable pointed to by |val| and
    /// returns true if the signed summation resulted in an overflow.
    pub fn signed_add_overflow64(lhs: i64, rhs: i64) -> (i64, bool) {
        let (res, overflow) = lhs.overflowing_add(rhs);
        (res, overflow)
    }

    /// SignedSubOverflow64(lhs,rhs,val) performs a signed subtraction of |lhs| and
    /// |rhs| and stores the result into the variable pointed to by |val| and
    /// returns true if the signed subtraction resulted in an overflow.
    pub fn signed_sub_overflow64(lhs: i64, rhs: i64) -> (i64, bool) {
        let (res, overflow) = lhs.overflowing_sub(rhs);
        (res, overflow)
    }

    /// SignedMulOverflow64(lhs,rhs,val) performs a signed multiplication of |lhs|
    /// and |rhs| and stores the result into the variable pointed to by |val| and
    /// returns true if the signed multiplication resulted in an overflow.
    pub fn signed_mul_overflow64(lhs: i64, rhs: i64) -> (i64, bool) {
        let (res, overflow) = lhs.overflowing_mul(rhs);
        (res, overflow)
    }

    // SignedMulHigh32(lhs, rhs) multiplies two signed 32-bit values |lhs| and
    // |rhs|, extracts the most significant 32 bits of the result, and returns
    // those.
    #[cfg(target_arch = "x86_64")]
    #[link(name = "v8_base")]
    extern "C" {
        pub fn SignedMulHigh32(lhs: i32, rhs: i32) -> i32;
    }

    #[cfg(not(target_arch = "x86_64"))]
    pub fn SignedMulHigh32(lhs: i32, rhs: i32) -> i32 {
        let result: i64 = i64::from(lhs) * i64::from(rhs);
        (result >> 32) as i32
    }

    // UnsignedMulHigh32(lhs, rhs) multiplies two unsigned 32-bit values |lhs| and
    // |rhs|, extracts the most significant 32 bits of the result, and returns
    // those.
    #[cfg(target_arch = "x86_64")]
    #[link(name = "v8_base")]
    extern "C" {
        pub fn UnsignedMulHigh32(lhs: u32, rhs: u32) -> u32;
    }

    #[cfg(not(target_arch = "x86_64"))]
    pub fn UnsignedMulHigh32(lhs: u32, rhs: u32) -> u32 {
        let result: u64 = u64::from(lhs) * u64::from(rhs);
        (result >> 32) as u32
    }

    // SignedMulHigh64(lhs, rhs) multiplies two signed 64-bit values |lhs| and
    // |rhs|, extracts the most significant 64 bits of the result, and returns
    // those.
    #[cfg(target_arch = "x86_64")]
    #[link(name = "v8_base")]
    extern "C" {
        pub fn SignedMulHigh64(lhs: i64, rhs: i64) -> i64;
    }

    #[cfg(not(target_arch = "x86_64"))]
    pub fn SignedMulHigh64(lhs: i64, rhs: i64) -> i64 {
        let lhs_high = lhs >> 32;
        let lhs_low = lhs & 0xFFFFFFFF;
        let rhs_high = rhs >> 32;
        let rhs_low = rhs & 0xFFFFFFFF;

        let cross1 = lhs_high * rhs_low;
        let cross2 = lhs_low * rhs_high;
        let high = lhs_high * rhs_high;

        let sum = cross1.wrapping_add(cross2).wrapping_add(high << 32);
        sum >> 32
    }

    // UnsignedMulHigh64(lhs, rhs) multiplies two unsigned 64-bit values |lhs| and
    // |rhs|, extracts the most significant 64 bits of the result, and returns
    // those.
    #[cfg(target_arch = "x86_64")]
    #[link(name = "v8_base")]
    extern "C" {
        pub fn UnsignedMulHigh64(lhs: u64, rhs: u64) -> u64;
    }

    #[cfg(not(target_arch = "x86_64"))]
    pub fn UnsignedMulHigh64(lhs: u64, rhs: u64) -> u64 {
        let lhs_high = lhs >> 32;
        let lhs_low = lhs & 0xFFFFFFFF;
        let rhs_high = rhs >> 32;
        let rhs_low = rhs & 0xFFFFFFFF;

        let cross1 = lhs_high * rhs_low;
        let cross2 = lhs_low * rhs_high;
        let high = lhs_high * rhs_high;

        let sum = cross1.wrapping_add(cross2).wrapping_add(high << 32);
        sum >> 32
    }

    // SignedMulHighAndAdd32(lhs, rhs, acc) multiplies two signed 32-bit values
    // |lhs| and |rhs|, extracts the most significant 32 bits of the result, and
    // adds the accumulate value |acc|.
    #[cfg(target_arch = "x86_64")]
    #[link(name = "v8_base")]
    extern "C" {
        pub fn SignedMulHighAndAdd32(lhs: i32, rhs: i32, acc: i32) -> i32;
    }

    #[cfg(not(target_arch = "x86_64"))]
    pub fn SignedMulHighAndAdd32(lhs: i32, rhs: i32, acc: i32) -> i32 {
        SignedMulHigh32(lhs, rhs).wrapping_add(acc)
    }

    // SignedDiv32(lhs, rhs) divides |lhs| by |rhs| and returns the quotient
    // truncated to int32. If |rhs| is zero, then zero is returned. If |lhs|
    // is minint and |rhs| is -1, it returns minint.
    #[cfg(target_arch = "x86_64")]
    #[link(name = "v8_base")]
    extern "C" {
        pub fn SignedDiv32(lhs: i32, rhs: i32) -> i32;
    }

    #[cfg(not(target_arch = "x86_64"))]
    pub fn SignedDiv32(lhs: i32, rhs: i32) -> i32 {
        if rhs == 0 {
            return 0;
        }
        if lhs == i32::MIN && rhs == -1 {
            return i32::MIN;
        }
        lhs / rhs
    }

    // SignedDiv64(lhs, rhs) divides |lhs| by |rhs| and returns the quotient
    // truncated to int64. If |rhs| is zero, then zero is returned. If |lhs|
    // is minint and |rhs| is -1, it returns minint.
    #[cfg(target_arch = "x86_64")]
    #[link(name = "v8_base")]
    extern "C" {
        pub fn SignedDiv64(lhs: i64, rhs: i64) -> i64;
    }

    #[cfg(not(target_arch = "x86_64"))]
    pub fn SignedDiv64(lhs: i64, rhs: i64) -> i64 {
        if rhs == 0 {
            return 0;
        }
        if lhs == i64::MIN && rhs == -1 {
            return i64::MIN;
        }
        lhs / rhs
    }

    // SignedMod32(lhs, rhs) divides |lhs| by |rhs| and returns the remainder
    // truncated to int32. If either |rhs| is zero or |lhs| is minint and |rhs|
    // is -1, it returns zero.
    #[cfg(target_arch = "x86_64")]
    #[link(name = "v8_base")]
    extern "C" {
        pub fn SignedMod32(lhs: i32, rhs: i32) -> i32;
    }

    #[cfg(not(target_arch = "x86_64"))]
    pub fn SignedMod32(lhs: i32, rhs: i32) -> i32 {
        if rhs == 0 || (lhs == i32::MIN && rhs == -1) {
            return 0;
        }
        lhs % rhs
    }

    // SignedMod64(lhs, rhs) divides |lhs| by |rhs| and returns the remainder
    // truncated to int64. If either |rhs| is zero or |lhs| is minint and |rhs|
    // is -1, it returns zero.
    #[cfg(target_arch = "x86_64")]
    #[link(name = "v8_base")]
    extern "C" {
        pub fn SignedMod64(lhs: i64, rhs: i64) -> i64;
    }

    #[cfg(not(target_arch = "x86_64"))]
    pub fn SignedMod64(lhs: i64, rhs: i64) -> i64 {
        if rhs == 0 || (lhs == i64::MIN && rhs == -1) {
            return 0;
        }
        lhs % rhs
    }

    /// UnsignedAddOverflow32(lhs,rhs,val) performs an unsigned summation of |lhs|
    /// and |rhs| and stores the result into the variable pointed to by |val| and
    /// returns true if the unsigned summation resulted in an overflow.
    pub fn unsigned_add_overflow32(lhs: u32, rhs: u32) -> (u32, bool) {
        lhs.overflowing_add(rhs)
    }

    /// UnsignedDiv32(lhs, rhs) divides |lhs| by |rhs| and returns the quotient
    /// truncated to uint32. If |rhs| is zero, then zero is returned.
    pub fn unsigned_div32(lhs: u32, rhs: u32) -> u32 {
        if rhs == 0 {
            0
        } else {
            lhs / rhs
        }
    }

    /// UnsignedDiv64(lhs, rhs) divides |lhs| by |rhs| and returns the quotient
    /// truncated to uint64. If |rhs| is zero, then zero is returned.
    pub fn unsigned_div64(lhs: u64, rhs: u64) -> u64 {
        if rhs == 0 {
            0
        } else {
            lhs / rhs
        }
    }

    /// UnsignedMod32(lhs, rhs) divides |lhs| by |rhs| and returns the remainder
    /// truncated to uint32. If |rhs| is zero, then zero is returned.
    pub fn unsigned_mod32(lhs: u32, rhs: u32) -> u32 {
        if rhs == 0 {
            0
        } else {
            lhs % rhs
        }
    }

    /// UnsignedMod64(lhs, rhs) divides |lhs| by |rhs| and returns the remainder
    /// truncated to uint64. If |rhs| is zero, then zero is returned.
    pub fn unsigned_mod64(lhs: u64, rhs: u64) -> u64 {
        if rhs == 0 {
            0
        } else {
            lhs % rhs
        }
    }

    /// Wraparound integer arithmetic without undefined behavior.
    pub fn wraparound_add32(lhs: i32, rhs: i32) -> i32 {
        (lhs as u32).wrapping_add(rhs as u32) as i32
    }

    pub fn wraparound_neg32(x: i32) -> i32 {
        (-(x as u32)) as i32
    }

    // SignedSaturatedAdd64(lhs, rhs) adds |lhs| and |rhs|,
    // checks and returns the result.
    #[cfg(target_arch = "x86_64")]
    #[link(name = "v8_base")]
    extern "C" {
        pub fn SignedSaturatedAdd64(lhs: i64, rhs: i64) -> i64;
    }

    #[cfg(not(target_arch = "x86_64"))]
    pub fn SignedSaturatedAdd64(lhs: i64, rhs: i64) -> i64 {
        let (result, overflow) = lhs.overflowing_add(rhs);
        if overflow {
            if rhs > 0 {
                i64::MAX
            } else {
                i64::MIN
            }
        } else {
            result
        }
    }

    // SignedSaturatedSub64(lhs, rhs) subtracts |lhs| by |rhs|,
    // checks and returns the result.
    #[cfg(target_arch = "x86_64")]
    #[link(name = "v8_base")]
    extern "C" {
        pub fn SignedSaturatedSub64(lhs: i64, rhs: i64) -> i64;
    }

    #[cfg(not(target_arch = "x86_64"))]
    pub