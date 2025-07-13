// Converted from V8 C++ source files:
// Header: bits.h
// Implementation: bits.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod bits {
        use std::{
            convert::TryInto,
            fmt,
            i32,
            i64,
            marker::PhantomData,
            mem::{self, size_of},
            ops::{BitAnd, BitOr, BitXor, Div, Mul, Neg, Rem, Shl, Shr, Sub},
        };

        // CountPopulation(value) returns the number of bits set in |value|.
        pub const fn count_population<T>(value: T) -> u32
        where
            T: std::marker::Copy,
            T: BitAnd<Output = T>,
            T: Shr<usize, Output = T>,
            T: BitXor<Output = T>,
            T: BitOr<Output = T>,
            T: Sub<Output = T>,
            T: Into<u64>,
        {
            let value: u64 = value.into();
            // Fall back to divide-and-conquer popcount (see "Hacker's Delight" by Henry
            // S. Warren, Jr.), chapter 5-1.
            let mask: [u64; 3] = [
                0x5555555555555555,
                0x3333333333333333,
                0x0f0f0f0f0f0f0f0f,
            ];
            // Start with 64 buckets of 1 bits, holding values from [0,1].
            let mut value = ((value >> 1) & mask[0]) + (value & mask[0]);
            // Having 32 buckets of 2 bits, holding values from [0,2] now.
            value = ((value >> 2) & mask[1]) + (value & mask[1]);
            // Having 16 buckets of 4 bits, holding values from [0,4] now.
            value = ((value >> 4) & mask[2]) + (value & mask[2]);
            // Having 8 buckets of 8 bits, holding values from [0,8] now.
            // From this point on, the buckets are bigger than the number of bits
            // required to hold the values, and the buckets are bigger the maximum
            // result, so there's no need to mask value anymore, since there's no
            // more risk of overflow between buckets.

            value = (value >> (8)) + value;

            // Having 4 buckets of 16 bits, holding values from [0,16] now.
            value = (value >> (16)) + value;

            // Having 2 buckets of 32 bits, holding values from [0,32] now.
            value = (value >> (32)) + value;

            // Having 1 buckets of 64 bits, holding values from [0,64] now.
            (value & 0xff) as u32
        }

        // ReverseBits(value) returns |value| in reverse bit order.
        pub fn reverse_bits<T>(value: T) -> T
        where
            T: std::marker::Copy,
            T: Shl<usize, Output = T>,
            T: Shr<usize, Output = T>,
            T: BitOr<Output = T>,
            T: BitAnd<u8, Output = T>,
            T: From<u8>,
            T: Into<u64>,
        {
            let size = std::mem::size_of::<T>();
            assert!(size == 1 || size == 2 || size == 4 || size == 8);

            let mut value: u64 = value.into();

            let mut result: u64 = 0;
            for _i in 0..(size * 8) {
                result = (result << 1) | (value & 1);
                value >>= 1;
            }
            result.try_into().unwrap()
        }

        // ReverseBytes(value) returns |value| in reverse byte order.
        pub fn reverse_bytes<T>(value: T) -> T
        where
            T: std::marker::Copy,
            T: Shl<usize, Output = T>,
            T: Shr<usize, Output = T>,
            T: BitOr<Output = T>,
            T: BitAnd<u8, Output = T>,
            T: From<u8>,
        {
            let size = std::mem::size_of::<T>();
            assert!(size == 1 || size == 2 || size == 4 || size == 8);

            let mut value_bytes: [u8; 8] = [0; 8];
            unsafe {
                std::ptr::copy_nonoverlapping(
                    &value as *const T as *const u8,
                    value_bytes.as_mut_ptr(),
                    size,
                );
            }

            let mut result_bytes: [u8; 8] = [0; 8];
            for i in 0..size {
                result_bytes[i] = value_bytes[size - 1 - i];
            }

            let mut result: T = unsafe { std::mem::zeroed() };
            unsafe {
                std::ptr::copy_nonoverlapping(
                    result_bytes.as_ptr(),
                    &mut result as *mut T as *mut u8,
                    size,
                );
            }

            result
        }

        pub const fn unsigned<T>(value: T) -> std::num::NonZeroU64
        where
            T: std::marker::Copy,
            T: Into<i64>,
        {
            let val: i64 = value.into();
            std::num::NonZeroU64::new(val as u64).unwrap()
        }
        pub const fn signed<T>(value: std::num::NonZeroU64) -> i64 {
            value.get() as i64
        }

        // CountLeadingZeros(value) returns the number of zero bits following the most
        // significant 1 bit in |value| if |value| is non-zero, otherwise it returns
        // {sizeof(T) * 8}.
        pub const fn count_leading_zeros<T, const BITS: usize>(value: T) -> u32
        where
            T: std::marker::Copy,
            T: std::cmp::PartialEq<T>,
            T: Shr<usize, Output = T>,
            T: BitXor<Output = T>,
            T: Into<u64>,
        {
            if BITS == 0 {
                panic!("invalid instantiation");
            }

            let value: u64 = value.into();

            if value == 0 {
                return BITS as u32;
            }

            let bits = BITS as u32;
            if bits == 1 {
                return (value as u32) ^ 1;
            }

            let upper_half = (value >> (BITS / 2)) as u64;
            let next_value = if upper_half != 0 {
                upper_half
            } else {
                value as u64
            };
            let add = if upper_half != 0 {
                0
            } else {
                (BITS / 2) as u32
            };
            let next_bits = if BITS == 1 { 1 } else { BITS / 2 };
            count_leading_zeros::<u64, next_bits>(next_value) + add
        }

        pub const fn count_leading_zeros32(value: u32) -> u32 {
            count_leading_zeros::<u32, 32>(value)
        }
        pub const fn count_leading_zeros64(value: u64) -> u32 {
            count_leading_zeros::<u64, 64>(value)
        }

        // The number of leading zeros for a positive number,
        // the number of leading ones for a negative number.
        pub const fn count_leading_sign_bits<T>(value: T) -> u32
        where
            T: std::marker::Copy,
            T: std::cmp::PartialOrd<T>,
            T: Neg<Output = T>,
            T: Into<i64>,
        {
            let zero: T = (0 as i32).try_into().unwrap();
            let value_i64: i64 = value.into();

            if value_i64 < 0 {
                let unsigned_value = (!value_i64 as u64).try_into().unwrap();
                count_leading_zeros::<u64, 64>(unsigned_value)
            } else {
                let unsigned_value = (value_i64 as u64).try_into().unwrap();
                count_leading_zeros::<u64, 64>(unsigned_value)
            }
        }

        // CountTrailingZeros(value) returns the number of zero bits preceding the
        // least significant 1 bit in |value| if |value| is non-zero, otherwise it
        // returns {sizeof(T) * 8}.
        // See CountTrailingZerosNonZero for an optimized version for the case that
        // |value| is guaranteed to be non-zero.
        pub const fn count_trailing_zeros<T, const BITS: usize>(value: T) -> u32
        where
            T: std::marker::Copy,
            T: std::cmp::PartialEq<T>,
            T: BitAnd<Output = T>,
            T: BitXor<Output = T>,
            T: BitOr<Output = T>,
            T: Sub<Output = T>,
            T: Into<u64>,
        {
            let value: u64 = value.into();
            if value == 0 {
                return BITS as u32;
            }

            let bits = BITS as u32;
            let u = value;
            count_population((!u) & (u.wrapping_sub(1)))
        }

        pub const fn count_trailing_zeros32(value: u32) -> u32 {
            count_trailing_zeros::<u32, 32>(value)
        }
        pub const fn count_trailing_zeros64(value: u64) -> u32 {
            count_trailing_zeros::<u64, 64>(value)
        }

        // CountTrailingZerosNonZero(value) returns the number of zero bits preceding
        // the least significant 1 bit in |value| if |value| is non-zero, otherwise the
        // behavior is undefined.
        // See CountTrailingZeros for an alternative version that allows |value| == 0.
        pub const fn count_trailing_zeros_non_zero<T, const BITS: usize>(value: T) -> u32
        where
            T: std::marker::Copy,
            T: std::cmp::PartialEq<T>,
            T: BitAnd<Output = T>,
            T: BitXor<Output = T>,
            T: BitOr<Output = T>,
            T: Sub<Output = T>,
            T: Into<u64>,
        {
            let value: u64 = value.into();
            if value == 0 {
                panic!("value should not be zero");
            }

            let bits = BITS as u32;
            let u = value;
            count_population((!u) & (u.wrapping_sub(1)))
        }

        // Returns true iff |value| is a power of 2.
        pub const fn is_power_of_two<T>(value: T) -> bool
        where
            T: std::marker::Copy,
            T: std::cmp::PartialOrd<T>,
            T: BitAnd<Output = T>,
            T: Sub<Output = T>,
            T: Into<u64>,
        {
            let value_u64: u64 = value.into();
            let zero: u64 = 0;

            value_u64 > zero && (value_u64 & (value_u64 - 1)) == 0
        }

        // Identical to {CountTrailingZeros}, but only works for powers of 2.
        pub const fn which_power_of_two<T>(value: T) -> i32
        where
            T: std::marker::Copy,
            T: std::cmp::PartialOrd<T>,
            T: BitAnd<Output = T>,
            T: Sub<Output = T>,
            T: Into<u64>,
        {
            let value_u64: u64 = value.into();
            if !is_power_of_two(value_u64) {
                panic!("value should be power of 2");
            }
            let u = value_u64;
            count_population(u - 1) as i32
        }

        // RoundUpToPowerOfTwo32(value) returns the smallest power of two which is
        // greater than or equal to |value|. If you pass in a |value| that is already a
        // power of two, it is returned as is. |value| must be less than or equal to
        // 0x80000000u. Uses computation based on leading zeros if we have compiler
        // support for that. Falls back to the implementation from "Hacker's Delight" by
        // Henry S. Warren, Jr., figure 3-3, page 48, where the function is called clp2.
        pub const fn round_up_to_power_of_two32(mut value: u32) -> u32 {
            if value > (1u32 << 31) {
                panic!("value must be <= 2^31");
            }
            if value != 0 {
                value -= 1;
            }

            value |= value >> 1;
            value |= value >> 2;
            value |= value >> 4;
            value |= value >> 8;
            value |= value >> 16;
            value + 1
        }
        // Same for 64 bit integers. |value| must be <= 2^63
        pub const fn round_up_to_power_of_two64(mut value: u64) -> u64 {
            if value > (1u64 << 63) {
                panic!("value must be <= 2^63");
            }
            if value != 0 {
                value -= 1;
            }
            value |= value >> 1;
            value |= value >> 2;
            value |= value >> 4;
            value |= value >> 8;
            value |= value >> 16;
            value |= value >> 32;
            value + 1
        }
        // Same for size_t integers.
        pub const fn round_up_to_power_of_two(value: usize) -> usize {
            if size_of::<usize>() == size_of::<u64>() {
                round_up_to_power_of_two64(value as u64) as usize
            } else {
                round_up_to_power_of_two32(value as u32) as usize
            }
        }

        // RoundDownToPowerOfTwo32(value) returns the greatest power of two which is
        // less than or equal to |value|. If you pass in a |value| that is already a
        // power of two, it is returned as is.
        pub fn round_down_to_power_of_two32(value: u32) -> u32 {
            if value > 0x80000000u {
                return 0x80000000u;
            }
            let mut result = round_up_to_power_of_two32(value);
            if result > value {
                result >>= 1;
            }
            result
        }

        // Precondition: 0 <= shift < 32
        pub const fn rotate_right32(value: u32, shift: u32) -> u32 {
            (value >> shift) | (value << ((32 - shift) & 31))
        }

        // Precondition: 0 <= shift < 32
        pub const fn rotate_left32(value: u32, shift: u32) -> u32 {
            (value << shift) | (value >> ((32 - shift) & 31))
        }

        // Precondition: 0 <= shift < 64
        pub const fn rotate_right64(value: u64, shift: u64) -> u64 {
            (value >> shift) | (value << ((64 - shift) & 63))
        }

        // Precondition: 0 <= shift < 64
        pub const fn rotate_left64(value: u64, shift: u64) -> u64 {
            (value << shift) | (value >> ((64 - shift) & 63))
        }

        // SignedAddOverflow32(lhs,rhs,val) performs a signed summation of |lhs| and
        // |rhs| and stores the result into the variable pointed to by |val| and
        // returns true if the signed summation resulted in an overflow.
        pub fn signed_add_overflow32(lhs: i32, rhs: i32, val: &mut i32) -> bool {
            match lhs.overflowing_add(rhs) {
                (result, overflow) => {
                    *val = result;
                    overflow
                }
            }
        }

        // SignedSubOverflow32(lhs,rhs,val) performs a signed subtraction of |lhs| and
        // |rhs| and stores the result into the variable pointed to by |val| and
        // returns true if the signed subtraction resulted in an overflow.
        pub fn signed_sub_overflow32(lhs: i32, rhs: i32, val: &mut i32) -> bool {
            match lhs.overflowing_sub(rhs) {
                (result, overflow) => {
                    *val = result;
                    overflow
                }
            }
        }

        // SignedMulOverflow32(lhs,rhs,val) performs a signed multiplication of |lhs|
        // and |rhs| and stores the result into the variable pointed to by |val| and
        // returns true if the signed multiplication resulted in an overflow.
        pub fn signed_mul_overflow32(lhs: i32, rhs: i32, val: &mut i32) -> bool {
            match lhs.overflowing_mul(rhs) {
                (result, overflow) => {
                    *val = result;
                    overflow
                }
            }
        }

        // SignedAddOverflow64(lhs,rhs,val) performs a signed summation of |lhs| and
        // |rhs| and stores the result into the variable pointed to by |val| and
        // returns true if the signed summation resulted in an overflow.
        pub fn signed_add_overflow64(lhs: i64, rhs: i64, val: &mut i64) -> bool {
            match lhs.overflowing_add(rhs) {
                (result, overflow) => {
                    *val = result;
                    overflow
                }
            }
        }

        // SignedSubOverflow64(lhs,rhs,val) performs a signed subtraction of |lhs| and
        // |rhs| and stores the result into the variable pointed to by |val| and
        // returns true if the signed subtraction resulted in an overflow.
        pub fn signed_sub_overflow64(lhs: i64, rhs: i64, val: &mut i64) -> bool {
            match lhs.overflowing_sub(rhs) {
                (result, overflow) => {
                    *val = result;
                    overflow
                }
            }
        }

        // SignedMulOverflow64(lhs,rhs,val) performs a signed multiplication of |lhs|
        // and |rhs| and stores the result into the variable pointed to by |val| and
        // returns true if the signed multiplication resulted in an overflow.
        pub fn signed_mul_overflow64(lhs: i64, rhs: i64, val: &mut i64) -> bool {
            match lhs.overflowing_mul(rhs) {
                (result, overflow) => {
                    *val = result;
                    overflow
                }
            }
        }

        // SignedMulHigh32(lhs, rhs) multiplies two signed 32-bit values |lhs| and
        // |rhs|, extracts the most significant 32 bits of the result, and returns
        // those.
        pub fn signed_mul_high32(lhs: i32, rhs: i32) -> i32 {
            let value = (lhs as i64) * (rhs as i64);
            (value >> 32) as i32
        }

        // UnsignedMulHigh32(lhs, rhs) multiplies two unsigned 32-bit values |lhs| and
        // |rhs|, extracts the most significant 32 bits of the result, and returns
        // those.
        pub fn unsigned_mul_high32(lhs: u32, rhs: u32) -> u32 {
            let value = (lhs as u64) * (rhs as u64);
            (value >> 32) as u32
        }

        // SignedMulHigh64(lhs, rhs) multiplies two signed 64-bit values |lhs| and
        // |rhs|, extracts the most significant 64 bits of the result, and returns
        // those.
        pub fn signed_mul_high64(u: i64, v: i64) -> i64 {
            let u0 = u & 0xFFFFFFFF;
            let u1 = u >> 32;
            let v0 = v & 0xFFFFFFFF;
            let v1 = v >> 32;

            let w0 = u0 * v0;
            let t = u1 * v0 + (w0 >> 32);
            let w1 = t & 0xFFFFFFFF;
            let w2 = t >> 32;
            let w1 = u0 * v1 + w1;

            u1 * v1 + w2 + (w1 >> 32)
        }

        // UnsignedMulHigh64(lhs, rhs) multiplies two unsigned 64-bit values |lhs| and
        // |rhs|, extracts the most significant 64 bits of the result, and returns
        // those.
        pub fn unsigned_mul_high64(u: u64, v: u64) -> u64 {
            let u0 = u & 0xFFFFFFFF;
            let u1 = u >> 32;
            let v0 = v & 0xFFFFFFFF;
            let v1 = v >> 32;

            let w0 = u0 * v0;
            let t = u1 * v0 + (w0 >> 32);
            let w1 = t & 0xFFFFFFFF;
            let w2 = t >> 32;
            let w1 = u0 * v1 + w1;

            u1 * v1 + w2 + (w1 >> 32)
        }

        // SignedMulHighAndAdd32(lhs, rhs, acc) multiplies two signed 32-bit values
        // |lhs| and |rhs|, extracts the most significant 32 bits of the result, and
        // adds the accumulate value |acc|.
        pub fn signed_mul_high_and_add32(lhs: i32, rhs: i32, acc: i32) -> i32 {
            acc.wrapping_add(signed_mul_high32(lhs, rhs))
        }

        // SignedDiv32(lhs, rhs) divides |lhs| by |rhs| and returns the quotient
        // truncated to int32. If |rhs| is zero, then zero is returned. If |lhs|
        // is minint and |rhs| is -1, it returns minint.
        pub fn signed_div32(lhs: i32, rhs: i32) -> i32 {
            if rhs == 0 {
                return 0;
            }
            if rhs == -1 {
                if lhs == i32::MIN {
                    return lhs;
                } else {
                    return -lhs;
                }
            }
            lhs / rhs
        }

        // SignedDiv64(lhs, rhs) divides |lhs| by |rhs| and returns the quotient
        // truncated to int64. If |rhs| is zero, then zero is returned. If |lhs|
        // is minint and |rhs| is -1, it returns minint.
        pub fn signed_div64(lhs: i64, rhs: i64) -> i64 {
            if rhs == 0 {
                return 0;
            }
            if rhs == -1 {
                if lhs == i64::MIN {
                    return lhs;
                } else {
                    return -lhs;
                }
            }
            lhs / rhs
        }

        // SignedMod32(lhs, rhs) divides |lhs| by |rhs| and returns the remainder
        // truncated to int32. If either |rhs| is zero or |lhs| is minint and |rhs|
        // is -1, it returns zero.
        pub fn signed_mod32(lhs: i32, rhs: i32) -> i32 {
            if rhs == 0 || rhs == -1 {
                return 0;
            }
            lhs % rhs
        }

        // SignedMod64(lhs, rhs) divides |lhs| by |rhs| and returns the remainder
        // truncated to int64. If either |rhs| is zero or |lhs| is minint and |rhs|
        // is -1, it returns zero.
        pub fn signed_mod64(lhs: i64, rhs: i64) -> i64 {
            if rhs == 0 || rhs == -1 {
                return 0;
            }
            lhs % rhs
        }

        // UnsignedAddOverflow32(lhs,rhs,val) performs an unsigned summation of |lhs|
        // and |rhs| and stores the result into the variable pointed to by |val| and
        // returns true if the unsigned summation resulted in an overflow.
        pub fn unsigned_add_overflow32(lhs: u32, rhs: u32, val: &mut u32) -> bool {
            match lhs.overflowing_add(rhs) {
                (result, overflow) => {
                    *val = result;
                    overflow
                }
            }
        }

        // UnsignedDiv32(lhs, rhs) divides |lhs| by |rhs| and returns the quotient
        // truncated to uint32. If |rhs| is zero, then zero is returned.
        pub fn unsigned_div32(lhs: u32, rhs: u32) -> u32 {
            if rhs == 0 {
                0
            } else {
                lhs / rhs
            }
        }

        // UnsignedDiv64(lhs, rhs) divides |lhs| by |rhs| and returns the quotient
        // truncated to uint64. If |rhs| is zero, then zero is returned.
        pub fn unsigned_div64(lhs: u64, rhs: u64) -> u64 {
            if rhs == 0 {
                0
            } else {
                lhs / rhs
            }
        }

        // UnsignedMod32(lhs, rhs) divides |lhs| by |rhs| and returns the remainder
        // truncated to uint32. If |rhs| is zero, then zero is returned.
        pub fn unsigned_mod32(lhs: u32, rhs: u32) -> u32 {
            if rhs == 0 {
                0
            } else {
                lhs % rhs
            }
        }

        // UnsignedMod64(lhs, rhs) divides |lhs| by |rhs| and returns the remainder
        // truncated to uint64. If |rhs| is zero, then zero is returned.
        pub fn unsigned_mod64(lhs: u64, rhs: u64) -> u64 {
            if rhs == 0 {
                0
            } else {
                lhs % rhs
            }
        }

        // Wraparound integer arithmetic without undefined behavior.
        pub fn wraparound_add32(lhs: i32, rhs: i32) -> i32 {
            lhs.wrapping_add(rhs)
        }

        pub fn wraparound_neg32(x: i32) -> i32 {
            x.wrapping_neg()
        }

        // SignedSaturatedAdd64(lhs, rhs) adds |lhs| and |rhs|,
        // checks and returns the result.
        pub fn signed_saturated_add64(lhs: i64, rhs: i64) -> i64 {
            match lhs.checked_add(rhs) {
                Some(result) => result,
                None => {
                    if rhs > 0 {
                        i64::MAX
                    } else {
                        i64::MIN
                    }
                }
            }
        }

        // SignedSaturatedSub64(lhs, rhs) subtracts |lhs| by |rhs|,
        // checks and returns the result.
        pub fn signed_saturated_sub64(lhs: i64, rhs: i64) -> i64 {
            match lhs.checked_sub(rhs) {
                Some(result) => result,
                None => {
                    if rhs > 0 {
                        i64::MIN
                    } else {
                        i64::MAX
                    }
                }
            }
        }

        pub const fn bit_width<T>(x: T) -> i32
        where
            T: std::marker::Copy,
            T: std::cmp::PartialEq<T>,
            T: BitAnd<Output = T>,
            T: Shr<usize, Output = T>,
            T: BitXor<Output = T>,
            T: BitOr<Output = T>,
            T: Sub<Output = T>,
            T: Into<u64>,
            T: std::fmt::Binary,
        {
            let x64 = x.into();
            i64::BITS as i32 - count_leading_zeros::<u64, 64>(x64) as i32
        }
    }
}
