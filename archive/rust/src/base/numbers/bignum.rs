// Copyright 2011 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/base/numbers/bignum.h - Module Definition
pub mod bignum {
    use std::{cmp::{max, min}, fmt};
    use std::ops::{Add, Div, Mul, Shl, Shr, Sub};
    use std::mem::size_of;

    const K_BIGIT_SIZE: usize = 32;
    const K_CHUNK_SIZE: usize = 32;
    const K_BIGIT_CAPACITY: usize = 64;

    pub type Chunk = u32;
    pub type DoubleChunk = u64;

    const K_BIGIT_MASK: Chunk = Chunk::MAX;

    /// Represents a large integer.
    #[derive(Clone, Debug)]
    pub struct Bignum {
        bigits_: Vec<Chunk>,
        used_digits_: usize,
        exponent_: i32,
        bigits_buffer_: [Chunk; K_BIGIT_CAPACITY],
    }

    impl Bignum {
        /// Creates a new `Bignum` with initial values set to zero.
        pub fn new() -> Self {
            let mut bigits_buffer_ = [0; K_BIGIT_CAPACITY];
            let bigits_ = bigits_buffer_.to_vec(); //Vec::with_capacity(K_BIGIT_CAPACITY); //bigits_buffer_.to_vec();
            Bignum {
                bigits_,
                used_digits_: 0,
                exponent_: 0,
                bigits_buffer_,
            }
        }

        fn bit_size<S>(_: S) -> usize {
            8 * size_of::<S>()
        }

        /// Assigns a `uint16_t` value to the `Bignum`.
        pub fn assign_u16(&mut self, value: u16) {
            assert!(K_BIGIT_SIZE >= Self::bit_size(value));
            self.zero();
            if value == 0 {
                return;
            }

            self.ensure_capacity(1);
            self.bigits_[0] = value as Chunk;
            self.used_digits_ = 1;
        }

        /// Assigns a `uint64_t` value to the `Bignum`.
        pub fn assign_u64(&mut self, value: u64) {
            const K_UINT64_SIZE: usize = 64;

            self.zero();
            if value == 0 {
                return;
            }

            let needed_bigits = K_UINT64_SIZE / K_BIGIT_SIZE + 1;
            self.ensure_capacity(needed_bigits);
            for i in 0..needed_bigits {
                self.bigits_[i] = (value & K_BIGIT_MASK as u64) as Chunk;
                //value = value.wrapping_shr(K_BIGIT_SIZE as u32);
                value >>= K_BIGIT_SIZE;
            }
            self.used_digits_ = needed_bigits;
            self.clamp();
        }

        /// Assigns another `Bignum`'s value to this `Bignum`.
        pub fn assign_bignum(&mut self, other: &Bignum) {
            self.exponent_ = other.exponent_;
            let other_used_digits = other.used_digits_;
            self.ensure_capacity(other_used_digits);

            for i in 0..other_used_digits {
                self.bigits_[i] = other.bigits_[i];
            }

            // Clear the excess digits (if there were any).
            for i in other_used_digits..self.used_digits_ {
                self.bigits_[i] = 0;
            }
            self.used_digits_ = other_used_digits;
        }

        fn read_u64(buffer: &[u8], from: usize, digits_to_read: usize) -> u64 {
            let mut result: u64 = 0;
            let to = from + digits_to_read;

            for i in from..to {
                let digit = (buffer[i] as char).to_digit(10).unwrap() as u64;
                assert!((0..=9).contains(&(digit as u32)));
                result = result * 10 + digit;
            }
            return result;
        }

        /// Assigns a decimal string representation to the `Bignum`.
        pub fn assign_decimal_string(&mut self, value: &str) {
            // 2^64 = 18446744073709551616 > 10^19
            const K_MAX_UINT64_DECIMAL_DIGITS: usize = 19;

            self.zero();
            let mut length = value.len();
            let mut pos = 0;
            let value_bytes = value.as_bytes();

            // Let's just say that each digit needs 4 bits.
            while length >= K_MAX_UINT64_DECIMAL_DIGITS {
                let digits = Self::read_u64(value_bytes, pos, K_MAX_UINT64_DECIMAL_DIGITS);
                pos += K_MAX_UINT64_DECIMAL_DIGITS;
                length -= K_MAX_UINT64_DECIMAL_DIGITS;
                self.multiply_by_power_of_ten(K_MAX_UINT64_DECIMAL_DIGITS as i32);
                self.add_u64(digits);
            }
            let digits = Self::read_u64(value_bytes, pos, length);
            self.multiply_by_power_of_ten(length as i32);
            self.add_u64(digits);
            self.clamp();
        }

        fn hex_char_value(c: char) -> i32 {
            if ('0'..='9').contains(&c) {
                return (c as i32) - ('0' as i32);
            }
            if ('a'..='f').contains(&c) {
                return 10 + (c as i32) - ('a' as i32);
            }
            if ('A'..='F').contains(&c) {
                return 10 + (c as i32) - ('A' as i32);
            }
            panic!("Unreachable"); //UNREACHABLE();
        }

        /// Assigns a hexadecimal string representation to the `Bignum`.
        pub fn assign_hex_string(&mut self, value: &str) {
            self.zero();
            let length = value.len();
            let value_bytes = value.as_bytes();

            let needed_bigits = length * 4 / K_BIGIT_SIZE + 1;
            self.ensure_capacity(needed_bigits);
            let mut string_index = length - 1;
            for i in 0..needed_bigits - 1 {
                // These bigits are guaranteed to be "full".
                let mut current_bigit: Chunk = 0;
                for j in 0..K_BIGIT_SIZE / 4 {
                    current_bigit += (Self::hex_char_value(value_bytes[string_index] as char) as Chunk) << (j * 4);
                    if string_index > 0 {
                      string_index -= 1;
                    }
                }
                self.bigits_[i] = current_bigit;
            }
            self.used_digits_ = needed_bigits - 1;

            let mut most_significant_bigit: Chunk = 0; // Could be = 0;
            while string_index >= 0 {
                most_significant_bigit <<= 4;
                most_significant_bigit += Self::hex_char_value(value_bytes[string_index] as char) as Chunk;
                if string_index > 0 {
                  string_index -= 1;
                } else {
                  break;
                }
            }

            if most_significant_bigit != 0 {
                self.bigits_[self.used_digits_] = most_significant_bigit;
                self.used_digits_ += 1;
            }
            self.clamp();
        }

        /// Adds a `uint64_t` value to the `Bignum`.
        pub fn add_u64(&mut self, operand: u64) {
            if operand == 0 {
                return;
            }
            let mut other = Bignum::new();
            other.assign_u64(operand);
            self.add_bignum(&other);
        }

        /// Adds another `Bignum` to this `Bignum`.
        pub fn add_bignum(&mut self, other: &Bignum) {
            assert!(self.is_clamped());
            assert!(other.is_clamped());

            // If this has a greater exponent than other append zero-bigits to this.
            // After this call exponent_ <= other.exponent_.
            self.align(other);

            // There are two possibilities:
            //   aaaaaaaaaaa 0000  (where the 0s represent a's exponent)
            //     bbbbb 00000000
            //   ----------------
            //   ccccccccccc 0000
            // or
            //    aaaaaaaaaa 0000
            //  bbbbbbbbb 0000000
            //  -----------------
            //  cccccccccccc 0000
            // In both cases we might need a carry bigit.

            self.ensure_capacity(1 + max(self.bigit_length(), other.bigit_length()) - self.exponent_);
            let mut carry: DoubleChunk = 0;
            let mut bigit_pos = (other.exponent_ - self.exponent_) as usize;
            assert!(bigit_pos >= 0);
            for i in 0..other.used_digits_ {
                let sum = (self.bigits_[bigit_pos] as DoubleChunk) + (other.bigits_[i] as DoubleChunk) + carry;
                self.bigits_[bigit_pos] = (sum & K_BIGIT_MASK as DoubleChunk) as Chunk;
                carry = sum >> K_BIGIT_SIZE;
                bigit_pos += 1;
            }

            while carry != 0 {
                let sum = (self.bigits_[bigit_pos] as DoubleChunk) + carry;
                self.bigits_[bigit_pos] = (sum & K_BIGIT_MASK as DoubleChunk) as Chunk;
                carry = sum >> K_BIGIT_SIZE;
                bigit_pos += 1;
            }
            self.used_digits_ = max(bigit_pos, self.used_digits_);
            assert!(self.is_clamped());
        }

        /// Subtracts another `Bignum` from this `Bignum`.
        pub fn subtract_bignum(&mut self, other: &Bignum) {
            assert!(self.is_clamped());
            assert!(other.is_clamped());
            // We require this to be bigger than other.
            assert!(self.less_equal(other));

            self.align(other);

            let offset = (other.exponent_ - self.exponent_) as usize;
            let mut borrow: Chunk = 0;
            let mut i: usize = 0;
            for i in 0..other.used_digits_ {
                assert!((borrow == 0) || (borrow == 1));
                let difference = self.bigits_[i + offset] as i64 - other.bigits_[i] as i64 - borrow as i64;
                self.bigits_[i + offset] = (difference & K_BIGIT_MASK as i64) as Chunk;
                borrow = (difference >> (K_CHUNK_SIZE - 1)) as Chunk;
            }
            while borrow != 0 {
                let difference = self.bigits_[i + offset] as i64 - borrow as i64;
                self.bigits_[i + offset] = (difference & K_BIGIT_MASK as i64) as Chunk;
                borrow = (difference >> (K_CHUNK_SIZE - 1)) as Chunk;
                i += 1;
            }
            self.clamp();
        }

        /// Shifts the `Bignum` to the left by a specified amount.
        pub fn shift_left(&mut self, shift_amount: i32) {
            if self.used_digits_ == 0 {
                return;
            }
            self.exponent_ += shift_amount / K_BIGIT_SIZE as i32;
            let local_shift = (shift_amount % K_BIGIT_SIZE as i32) as i32;
            self.ensure_capacity(self.used_digits_ + 1);
            self.bigits_shift_left(local_shift);
        }

        /// Multiplies the `Bignum` by a `uint32_t` factor.
        pub fn multiply_by_u32(&mut self, factor: u32) {
            if factor == 1 {
                return;
            }
            if factor == 0 {
                self.zero();
                return;
            }
            if self.used_digits_ == 0 {
                return;
            }

            // The product of a bigit with the factor is of size kBigitSize + 32.
            // Assert that this number + 1 (for the carry) fits into double chunk.
            assert!(size_of::<DoubleChunk>() >= size_of::<Chunk>() + 4 + 1);
            let mut carry: DoubleChunk = 0;
            for i in 0..self.used_digits_ {
                let product = (factor as DoubleChunk) * (self.bigits_[i] as DoubleChunk) + carry;
                self.bigits_[i] = (product & K_BIGIT_MASK as DoubleChunk) as Chunk;
                carry = product >> K_BIGIT_SIZE;
            }
            while carry != 0 {
                self.ensure_capacity(self.used_digits_ + 1);
                self.bigits_[self.used_digits_] = (carry & K_BIGIT_MASK as DoubleChunk) as Chunk;
                self.used_digits_ += 1;
                carry >>= K_BIGIT_SIZE;
            }
        }

        /// Multiplies the `Bignum` by a `uint64_t` factor.
        pub fn multiply_by_u64(&mut self, factor: u64) {
            if factor == 1 {
                return;
            }
            if factor == 0 {
                self.zero();
                return;
            }
            assert!(K_BIGIT_SIZE < 32);
            let mut carry: u64 = 0;
            let low: u64 = factor & 0xFFFFFFFF;
            let high: u64 = factor >> 32;
            for i in 0..self.used_digits_ {
                let product_low: u64 = low * (self.bigits_[i] as u64);
                let product_high: u64 = high * (self.bigits_[i] as u64);
                let tmp: u64 = (carry & K_BIGIT_MASK as u64) + product_low;
                self.bigits_[i] = (tmp & K_BIGIT_MASK as u64) as Chunk;
                carry = (carry >> K_BIGIT_SIZE) + (tmp >> K_BIGIT_SIZE) + (product_high << (32 - K_BIGIT_SIZE));
            }
            while carry != 0 {
                self.ensure_capacity(self.used_digits_ + 1);
                self.bigits_[self.used_digits_] = (carry & K_BIGIT_MASK as u64) as Chunk;
                self.used_digits_ += 1;
                carry >>= K_BIGIT_SIZE;
            }
        }

        /// Multiplies the `Bignum` by a power of ten.
        pub fn multiply_by_power_of_ten(&mut self, exponent: i32) {
            const K_FIVE27: u64 = 0x6765C793FA10079D;
            const K_FIVE1: u16 = 5;
            const K_FIVE2: u16 = K_FIVE1 * 5;
            const K_FIVE3: u16 = K_FIVE2 * 5;
            const K_FIVE4: u16 = K_FIVE3 * 5;
            const K_FIVE5: u16 = K_FIVE4 * 5;
            const K_FIVE6: u16 = K_FIVE5 * 5;
            const K_FIVE7: u32 = K_FIVE6 as u32 * 5;
            const K_FIVE8: u32 = K_FIVE7 * 5;
            const K_FIVE9: u32 = K_FIVE8 * 5;
            const K_FIVE10: u32 = K_FIVE9 * 5;
            const K_FIVE11: u32 = K_FIVE10 * 5;
            const K_FIVE12: u32 = K_FIVE11 * 5;
            const K_FIVE13: u32 = K_FIVE12 * 5;
            const K_FIVE1_TO_12: [u32; 12] = [K_FIVE1 as u32, K_FIVE2 as u32, K_FIVE3 as u32, K_FIVE4 as u32,
                                               K_FIVE5 as u32, K_FIVE6 as u32, K_FIVE7, K_FIVE8,
                                               K_FIVE9, K_FIVE10, K_FIVE11, K_FIVE12];

            assert!(exponent >= 0);
            if exponent == 0 {
                return;
            }
            if self.used_digits_ == 0 {
                return;
            }

            // We shift by exponent at the end just before returning.
            let mut remaining_exponent = exponent;
            while remaining_exponent >= 27 {
                self.multiply_by_u64(K_FIVE27);
                remaining_exponent -= 27;
            }
            while remaining_exponent >= 13 {
                self.multiply_by_u32(K_FIVE13);
                remaining_exponent -= 13;
            }
            if remaining_exponent > 0 {
                self.multiply_by_u32(K_FIVE1_TO_12[(remaining_exponent - 1) as usize]);
            }
            self.shift_left(exponent);
        }

        /// Squares the `Bignum`.
        pub fn square(&mut self) {
            assert!(self.is_clamped());
            let product_length = 2 * self.used_digits_;
            self.ensure_capacity(product_length);

            // Comba multiplication: compute each column separately.
            // Example: r = a2a1a0 * b2b1b0.
            //    r =  1    * a0b0 +
            //        10    * (a1b0 + a0b1) +
            //        100   * (a2b0 + a1b1 + a0b2) +
            //        1000  * (a2b1 + a1b2) +
            //        10000 * a2b2
            //
            // In the worst case we have to accumulate nb-digits products of digit*digit.
            //
            // Assert that the additional number of bits in a DoubleChunk are enough to
            // sum up used_digits of Bigit*Bigit.
            if (1 << (2 * (size_of::<Chunk>() - size_of::<Chunk>()))) <= self.used_digits_ {
                panic!("UNIMPLEMENTED()");
            }
            let mut accumulator: DoubleChunk = 0;
            // First shift the digits so we don't overwrite them.
            let copy_offset = self.used_digits_;
            for i in 0..self.used_digits_ {
                self.bigits_[copy_offset + i] = self.bigits_[i];
            }
            // We have two loops to avoid some 'if's in the loop.
            for i in 0..self.used_digits_ {
                // Process temporary digit i with power i.
                // The sum of the two indices must be equal to i.
                let mut bigit_index1: i32 = i as i32;
                let mut bigit_index2: i32 = 0;
                // Sum all of the sub-products.
                while bigit_index1 >= 0 {
                    let chunk1 = self.bigits_[copy_offset + bigit_index1 as usize];
                    let chunk2 = self.bigits_[copy_offset + bigit_index2 as usize];
                    accumulator += (chunk1 as DoubleChunk) * (chunk2 as DoubleChunk);
                    bigit_index1 -= 1;
                    bigit_index2 += 1;
                }
                self.bigits_[i] = (accumulator & K_BIGIT_MASK as DoubleChunk) as Chunk;
                accumulator >>= K_BIGIT_SIZE;
            }
            for i in self.used_digits_..product_length {
                let mut bigit_index1: i32 = (self.used_digits_ - 1) as i32;
                let mut bigit_index2: i32 = (i - self.used_digits_) as i32;
                // Invariant: sum of both indices is again equal to i.
                // Inner loop runs 0 times on last iteration, emptying accumulator.
                while bigit_index2 < self.used_digits_ as i32 {
                    let chunk1 = self.bigits_[copy_offset + bigit_index1 as usize];
                    let chunk2 = self.bigits_[copy_offset + bigit_index2 as usize];
                    accumulator += (chunk1 as DoubleChunk) * (chunk2 as DoubleChunk);
                    bigit_index1 -= 1;
                    bigit_index2 += 1;
                }
                // The overwritten bigits_[i] will never be read in further loop iterations,
                // because bigit_index1 and bigit_index2 are always greater
                // than i - used_digits_.
                self.bigits_[i] = (accumulator & K_BIGIT_MASK as DoubleChunk) as Chunk;
                accumulator >>= K_BIGIT_SIZE;
            }
            // Since the result was guaranteed to lie inside the number the
            // accumulator must be 0 now.
            assert_eq!(accumulator, 0);

            // Don't forget to update the used_digits and the exponent.
            self.used_digits_ = product_length;
            self.exponent_ *= 2;
            self.clamp();
        }

        /// Assigns a base raised to a power exponent to the `Bignum`.
        pub fn assign_power_u16(&mut self, base: u16, power_exponent: i32) {
            assert_ne!(base, 0);
            assert!(power_exponent >= 0);
            if power_exponent == 0 {
                self.assign_u16(1);
                return;
            }
            self.zero();
            let mut shifts = 0;
            // We expect base to be in range 2-32, and most often to be 10.
            // It does not make much sense to implement different algorithms for counting
            // the bits.
            let mut base = base;
            while (base & 1) == 0 {
                base >>= 1;
                shifts += 1;
            }
            let mut bit_size = 0;
            let mut tmp_base = base;
            while tmp_base != 0 {
                tmp_base >>= 1;
                bit_size += 1;
            }
            let final_size = bit_size * power_exponent;
            // 1 extra bigit for the shifting, and one for rounded final_size.
            self.ensure_capacity(final_size / K_BIGIT_SIZE + 2);

            // Left to Right exponentiation.
            let mut mask = 1;
            while power_exponent >= mask { mask <<= 1; }

            // The mask is now pointing to the bit above the most significant 1-bit of
            // power_exponent.
            // Get rid of first 1-bit;
            mask >>= 2;
            let mut this_value: u64 = base as u64;

            let mut delayed_multipliciation = false;
            const MAX_32BITS: u64 = 0xFFFFFFFF;
            while mask != 0 && this_value <= MAX_32BITS {
                this_value = this_value * this_value;
                // Verify that there is enough space in this_value to perform the
                // multiplication.  The first bit_size bits must be 0.
                if (power_exponent & mask) != 0 {
                    let base_bits_mask: u64 = !((1 as u64 << (64 - bit_size)) - 1);
                    let high_bits_zero = (this_value & base_bits_mask) == 0;
                    if high_bits_zero {
                        this_value *= base as u64;
                    } else {
                        delayed_multipliciation = true;
                    }
                }
                mask >>= 1;
            }
            self.assign_u64(this_value);
            if delayed_multipliciation {
                self.multiply_by_u32(base as u32);
            }

            // Now do the same thing as a bignum.
            while mask != 0 {
                self.square();
                if (power_exponent & mask) != 0 {
                    self.multiply_by_u32(base as u32);
                }
                mask >>= 1;
            }

            // And finally add the saved shifts.
            self.shift_left(shifts * power_exponent);
        }

        /// Divides the `Bignum` modulo another `Bignum` (both < 16bit).
        pub fn divide_modulo_int_bignum(&mut self, other: &Bignum) -> u16 {
            assert!(self.is_clamped());
            assert!(other.is_clamped());
            assert!(other.used_digits_ > 0);

            // Easy case: if we have less digits than the divisor than the result is 0.
            // Note: this handles the case where this == 0, too.
            if self.bigit_length() < other.bigit_length() {
                return 0;
            }

            self.align(other);

            let mut result: u16 = 0;

            // Start by removing multiples of 'other' until both numbers have the same
            // number of digits.
            while self.bigit_length() > other.bigit_length() {
                // This naive approach is extremely inefficient if the this divided other
                // might be big. This function is implemented for doubleToString where
                // the result should be small (less than 10).
                assert!(other.bigits_[other.used_digits_ - 1] >= ((1 << K_BIGIT_SIZE) / 16) as Chunk);
                // Remove the multiples of the first digit.
                // Example this = 23 and other equals 9. -> Remove 2 multiples.
                result += self.bigits_[self.used_digits_ - 1] as u16;
                self.subtract_times(other, self.bigits_[self.used_digits_ - 1] as i32);
            }

            assert!(self.bigit_length() == other.bigit_length());

            // Both bignums are at the same length now.
            // Since other has more than 0 digits we know that the access to
            // bigits_[used_digits_ - 1] is safe.
            let this_bigit = self.bigits_[self.used_digits_ - 1];
            let other_bigit = other.bigits_[other.used_digits_ - 1];

            if other.used_digits_ == 1 {
                // Shortcut for easy (and common) case.
                let quotient = this_bigit / other_bigit;
                self.bigits_[self.used_digits_ - 1] = this_bigit - other_bigit * quotient;
                result += quotient as u16;
                self.clamp();
                return result;
            }

            let division_estimate = this_bigit / (other_bigit + 1);
            result += division_estimate as u16;
            self.subtract_times(other, division_estimate as i32);

            if other_bigit as u64 * (division_estimate as u64 + 1) > this_bigit as u64 {
                // No need to even try to subtract. Even if other's remaining digits were 0
                // another subtraction would be too much.
                return result;
            }

            while self.less_equal(other) {
                self.subtract_bignum(other);
                result += 1;
            }
            return result;
        }

        fn size_in_hex_chars<S>(number: S) -> i32
        where S: Shr<i32, Output = S> + PartialEq + From<u8> + Copy {
            assert!(number != S::from(0));
            let mut result: i32 = 0;
            let mut num_copy = number;
            while num_copy != S::from(0) {
                num_copy = num_copy >> 4;
                result += 1;
            }
            return result;
        }

        fn hex_char_of_value(value: Chunk) -> char {
            match value {
                0..=9 => char::from_digit(value as u32, 10).unwrap(),
                10..=15 => char::from_u32((value - 10) as u32 + 'a' as u32).unwrap(),
                _ => panic!("Invalid hex value"),
            }
        }

        /// Converts the `Bignum` to a hexadecimal string representation.
        pub fn to_hex_string(&self) -> Option<String> {
            assert!(self.is_clamped());
            // Each bigit must be printable as separate hex-character.
            assert_eq!(K_BIGIT_SIZE % 4, 0);
            const K_HEX_CHARS_PER_BIGIT: usize = K_BIGIT_SIZE / 4;

            if self.used_digits_ == 0 {
                return Some("0".to_string());
            }
            // We add 1 for the terminating '\0' character.
            let needed_chars = (self.bigit_length() - 1) * K_HEX_CHARS_PER_BIGIT +
                                Self::size_in_hex_chars(self.bigits_[self.used_digits_ - 1]) as usize + 1;

            let mut buffer: Vec<char> = Vec::with_capacity(needed_chars);
             unsafe {
               buffer.set_len(needed_chars);
             }

            let mut string_index = needed_chars - 1;
            buffer[string_index] = '\0';
            string_index -= 1;

            for _ in 0..self.exponent_ {
                for _ in 0..K_HEX_CHARS_PER_BIGIT {
                    buffer[string_index] = '0';
                    string_index -= 1;
                }
            }
            for i in 0..self.used_digits_ - 1 {
                let mut current_bigit = self.bigits_[i];
                for _ in 0..K_HEX_CHARS_PER_BIGIT {
                    buffer[string_index] = Self::hex_char_of_value(current_bigit & 0xF);
                    string_index -= 1;
                    current_bigit >>= 4;
                }
            }
            // And finally the last bigit.
            let mut most_significant_bigit = self.bigits_[self.used_digits_ - 1];
            while most_significant_bigit != 0 {
                buffer[string_index] = Self::hex_char_of_value(most_significant_bigit & 0xF);
                string_index -= 1;
                most_significant_bigit >>= 4;
            }

            Some(buffer.iter().collect())
        }

        /// Returns the bigit at a specific index.
        pub fn bigit_at(&self, index: i32) -> Chunk {
            if index >= self.bigit_length() {
                return 0;
            }
            if index < self.exponent_ {
                return 0;
            }
            return self.bigits_[(index - self.exponent_) as usize];
        }

        /// Compares two `Bignum`s.
        pub fn compare(a: &Bignum, b: &Bignum) -> i32 {
            assert!(a.is_clamped());
            assert!(b.is_clamped());
            let bigit_length_a = a.bigit_length();
            let bigit_length_b = b.bigit_length();
            if bigit_length_a < bigit_length_b { return -1; }
            if bigit_length_a > bigit_length_b { return 1; }
            for i in (min(a.exponent_, b.exponent_)..bigit_length_a).rev() {
                let bigit_a = a.bigit_at(i);
                let bigit_b = b.bigit_at(i