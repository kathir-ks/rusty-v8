// Converted from V8 C++ source files:
// Header: bignum.h
// Implementation: bignum.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
pub mod numbers {

use std::cmp::{max, min};

#[derive(Debug, Clone)]
pub struct Bignum {
    bigits_buffer_: Vec<Chunk>,
    bigits_: Vec<Chunk>,
    used_digits_: i32,
    exponent_: i32,
}

type Chunk = u32;
type DoubleChunk = u64;

impl Bignum {
    const kMaxSignificantBits: i32 = 3584;
    const kChunkSize: i32 = std::mem::size_of::<Chunk>() as i32 * 8;
    const kDoubleChunkSize: i32 = std::mem::size_of::<DoubleChunk>() as i32 * 8;
    const kBigitSize: i32 = 28;
    const kBigitMask: Chunk = (1 << Self::kBigitSize) - 1;
    const kBigitCapacity: i32 = Self::kMaxSignificantBits / Self::kBigitSize;

    pub fn new() -> Self {
        let mut bigits_buffer_ = vec![0; Self::kBigitCapacity as usize];
        let bigits_ = bigits_buffer_.clone();
        let mut result = Self {
            bigits_buffer_,
            bigits_,
            used_digits_: 0,
            exponent_: 0,
        };
        for i in 0..Self::kBigitCapacity {
            result.bigits_buffer_[i as usize] = 0;
        }
        result
    }

    fn ensure_capacity(&mut self, size: i32) {
        if size > Self::kBigitCapacity {
            panic!("UNREACHABLE");
        }
        if size as usize > self.bigits_.len() {
            self.bigits_.resize(size as usize, 0);
        }
    }

    fn align(&mut self, other: &Bignum) {
        if self.exponent_ > other.exponent_ {
            let zero_digits = self.exponent_ - other.exponent_;
            self.ensure_capacity(self.used_digits_ + zero_digits);
            let mut new_bigits = vec![0; (self.used_digits_ + zero_digits) as usize];
            for i in 0..self.used_digits_ {
                new_bigits[(zero_digits + i) as usize] = self.bigits_[i as usize];
            }
            for i in 0..zero_digits {
                new_bigits[i as usize] = 0;
            }
            self.bigits_ = new_bigits;
            self.used_digits_ += zero_digits;
            self.exponent_ -= zero_digits;
        }
    }

    fn clamp(&mut self) {
        while self.used_digits_ > 0 && self.bigits_[(self.used_digits_ - 1) as usize] == 0 {
            self.used_digits_ -= 1;
        }
        if self.used_digits_ == 0 {
            self.exponent_ = 0;
        }
    }

    fn is_clamped(&self) -> bool {
        self.used_digits_ == 0 || self.bigits_[(self.used_digits_ - 1) as usize] != 0
    }

    fn zero(&mut self) {
        for i in 0..self.used_digits_ {
            self.bigits_[i as usize] = 0;
        }
        self.used_digits_ = 0;
        self.exponent_ = 0;
    }

    fn bigits_shift_left(&mut self, shift_amount: i32) {
        assert!(shift_amount < Self::kBigitSize);
        assert!(shift_amount >= 0);
        let mut carry: Chunk = 0;
        for i in 0..self.used_digits_ {
            let new_carry = self.bigits_[i as usize] >> (Self::kBigitSize - shift_amount);
            self.bigits_[i as usize] = ((self.bigits_[i as usize] << shift_amount) + carry) & Self::kBigitMask;
            carry = new_carry;
        }
        if carry != 0 {
            self.bigits_.push(carry);
            self.used_digits_ += 1;
        }
    }

    fn subtract_times(&mut self, other: &Bignum, factor: i32) {
        assert!(self.exponent_ <= other.exponent_);
        if factor < 3 {
            for _ in 0..factor {
                self.subtract_bignum(other);
            }
            return;
        }
        let mut borrow: Chunk = 0;
        let exponent_diff = other.exponent_ - self.exponent_;
        for i in 0..other.used_digits_ {
            let product: DoubleChunk = factor as DoubleChunk * other.bigits_[i as usize] as DoubleChunk;
            let remove: DoubleChunk = borrow as DoubleChunk + product;
            let difference: Chunk = self.bigits_[(i + exponent_diff) as usize] - (remove & Self::kBigitMask as DoubleChunk) as Chunk;
            self.bigits_[(i + exponent_diff) as usize] = difference & Self::kBigitMask;
            borrow = ((difference as i64 >> (Self::kChunkSize - 1)) + (remove >> Self::kBigitSize)) as Chunk;
        }
        for i in (other.used_digits_ + exponent_diff) as i32..self.used_digits_ {
            if borrow == 0 {
                return;
            }
            let difference = self.bigits_[i as usize] - borrow;
            self.bigits_[i as usize] = difference & Self::kBigitMask;
            borrow = difference >> (Self::kChunkSize - 1);
        }
        self.clamp();
    }

    pub fn assign_uint16(&mut self, value: u16) {
        assert!(Self::kBigitSize >= Self::bit_size(value));
        self.zero();
        if value == 0 {
            return;
        }
        self.ensure_capacity(1);
        self.bigits_[0] = value as Chunk;
        self.used_digits_ = 1;
    }

    pub fn assign_uint64(&mut self, value: u64) {
        const K_UINT64_SIZE: i32 = 64;
        self.zero();
        if value == 0 {
            return;
        }

        let needed_bigits = K_UINT64_SIZE / Self::kBigitSize + 1;
        self.ensure_capacity(needed_bigits);

        for i in 0..needed_bigits {
            self.bigits_[i as usize] = (value & Self::kBigitMask as u64) as Chunk;
            value >>= Self::kBigitSize;
        }
        self.used_digits_ = needed_bigits;
        self.clamp();
    }

    pub fn assign_bignum(&mut self, other: &Bignum) {
        self.exponent_ = other.exponent_;
        for i in 0..other.used_digits_ {
            self.bigits_[i as usize] = other.bigits_[i as usize];
        }
        for i in other.used_digits_..self.used_digits_ {
            self.bigits_[i as usize] = 0;
        }
        self.used_digits_ = other.used_digits_;
    }

    fn read_uint64(value: &[u8], from: i32, digits_to_read: i32) -> u64 {
        let mut result: u64 = 0;
        let to = from + digits_to_read;
        for i in from..to {
            let digit = (value[i as usize] as char).to_digit(10).unwrap() as u64;
            assert!((0..=9).contains(&digit));
            result = result * 10 + digit;
        }
        result
    }

    pub fn assign_decimal_string(&mut self, value: &[u8]) {
        const K_MAX_UINT64_DECIMAL_DIGITS: i32 = 19;
        self.zero();
        let mut length = value.len() as i32;
        let mut pos: i32 = 0;
        while length >= K_MAX_UINT64_DECIMAL_DIGITS {
            let digits = Self::read_uint64(value, pos, K_MAX_UINT64_DECIMAL_DIGITS);
            pos += K_MAX_UINT64_DECIMAL_DIGITS;
            length -= K_MAX_UINT64_DECIMAL_DIGITS;
            self.multiply_by_power_of_ten(K_MAX_UINT64_DECIMAL_DIGITS);
            self.add_uint64(digits);
        }
        let digits = Self::read_uint64(value, pos, length);
        self.multiply_by_power_of_ten(length);
        self.add_uint64(digits);
        self.clamp();
    }

    fn hex_char_value(c: char) -> i32 {
        if ('0'..='9').contains(&c) {
            return c as i32 - '0' as i32;
        }
        if ('a'..='f').contains(&c) {
            return 10 + c as i32 - 'a' as i32;
        }
        if ('A'..='F').contains(&c) {
            return 10 + c as i32 - 'A' as i32;
        }
        panic!("UNREACHABLE");
    }

    pub fn assign_hex_string(&mut self, value: &[u8]) {
        self.zero();
        let length = value.len() as i32;
        let needed_bigits = length * 4 / Self::kBigitSize + 1;
        self.ensure_capacity(needed_bigits);
        let mut string_index = length - 1;
        for i in 0..needed_bigits - 1 {
            let mut current_bigit: Chunk = 0;
            for j in 0..Self::kBigitSize / 4 {
                current_bigit += (Self::hex_char_value(value[string_index as usize] as char) << (j * 4)) as Chunk;
                string_index -= 1;
            }
            self.bigits_[i as usize] = current_bigit;
        }
        self.used_digits_ = needed_bigits - 1;

        let mut most_significant_bigit: Chunk = 0;
        for j in 0..=string_index {
            most_significant_bigit <<= 4;
            most_significant_bigit += Self::hex_char_value(value[j as usize] as char) as Chunk;
        }
        if most_significant_bigit != 0 {
            self.bigits_[self.used_digits_ as usize] = most_significant_bigit;
            self.used_digits_ += 1;
        }
        self.clamp();
    }

    pub fn add_uint64(&mut self, operand: u64) {
        if operand == 0 {
            return;
        }
        let mut other = Bignum::new();
        other.assign_uint64(operand);
        self.add_bignum(&other);
    }

    pub fn add_bignum(&mut self, other: &Bignum) {
        assert!(self.is_clamped());
        assert!(other.is_clamped());
        self.align(other);
        self.ensure_capacity(1 + max(self.bigit_length(), other.bigit_length()) - self.exponent_);

        let mut carry: Chunk = 0;
        let mut bigit_pos = other.exponent_ - self.exponent_;
        assert!(bigit_pos >= 0);
        for i in 0..other.used_digits_ {
            let sum = self.bigits_[bigit_pos as usize] as u64 + other.bigits_[i as usize] as u64 + carry as u64;
            self.bigits_[bigit_pos as usize] = (sum & Self::kBigitMask as u64) as Chunk;
            carry = (sum >> Self::kBigitSize) as Chunk;
            bigit_pos += 1;
        }

        while carry != 0 {
            let sum = self.bigits_[bigit_pos as usize] as u64 + carry as u64;
            self.bigits_[bigit_pos as usize] = (sum & Self::kBigitMask as u64) as Chunk;
            carry = (sum >> Self::kBigitSize) as Chunk;
            bigit_pos += 1;
        }
        self.used_digits_ = max(bigit_pos, self.used_digits_);
        assert!(self.is_clamped());
    }

    pub fn subtract_bignum(&mut self, other: &Bignum) {
        assert!(self.is_clamped());
        assert!(other.is_clamped());
        assert!(Bignum::less_equal(other, self));

        self.align(other);

        let offset = other.exponent_ - self.exponent_;
        let mut borrow: Chunk = 0;
        for i in 0..other.used_digits_ {
            assert!(borrow == 0 || borrow == 1);
            let difference = self.bigits_[(i + offset) as usize] as i64 - other.bigits_[i as usize] as i64 - borrow as i64;
            self.bigits_[(i + offset) as usize] = (difference & Self::kBigitMask as i64) as Chunk;
            borrow = (difference >> (Self::kChunkSize - 1)) as Chunk;
        }
        let mut i = other.used_digits_;
        while borrow != 0 {
            let difference = self.bigits_[(i + offset) as usize] as i64 - borrow as i64;
            self.bigits_[(i + offset) as usize] = (difference & Self::kBigitMask as i64) as Chunk;
            borrow = (difference >> (Self::kChunkSize - 1)) as Chunk;
            i += 1;
        }
        self.clamp();
    }

    pub fn shift_left(&mut self, shift_amount: i32) {
        if self.used_digits_ == 0 {
            return;
        }
        self.exponent_ += shift_amount / Self::kBigitSize;
        let local_shift = shift_amount % Self::kBigitSize;
        self.ensure_capacity(self.used_digits_ + 1);
        self.bigits_shift_left(local_shift);
    }

    pub fn multiply_by_uint32(&mut self, factor: u32) {
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
        assert!(Self::kDoubleChunkSize >= Self::kBigitSize + 32 + 1);
        let mut carry: DoubleChunk = 0;
        for i in 0..self.used_digits_ {
            let product = factor as DoubleChunk * self.bigits_[i as usize] as DoubleChunk + carry;
            self.bigits_[i as usize] = (product & Self::kBigitMask as DoubleChunk) as Chunk;
            carry = product >> Self::kBigitSize;
        }

        while carry != 0 {
            self.ensure_capacity(self.used_digits_ + 1);
            self.bigits_.push((carry & Self::kBigitMask as DoubleChunk) as Chunk);
            self.used_digits_ += 1;
            carry >>= Self::kBigitSize;
        }
    }

    pub fn multiply_by_uint64(&mut self, factor: u64) {
        if factor == 1 {
            return;
        }
        if factor == 0 {
            self.zero();
            return;
        }
        assert!(Self::kBigitSize < 32);
        let mut carry: u64 = 0;
        let low: u64 = factor & 0xFFFFFFFF;
        let high: u64 = factor >> 32;

        for i in 0..self.used_digits_ {
            let product_low: u64 = low * self.bigits_[i as usize] as u64;
            let product_high: u64 = high * self.bigits_[i as usize] as u64;
            let tmp: u64 = (carry & Self::kBigitMask as u64) + product_low;
            self.bigits_[i as usize] = (tmp & Self::kBigitMask as u64) as Chunk;
            carry = (carry >> Self::kBigitSize) + (tmp >> Self::kBigitSize) + (product_high << (32 - Self::kBigitSize));
        }

        while carry != 0 {
            self.ensure_capacity(self.used_digits_ + 1);
            self.bigits_.push((carry & Self::kBigitMask as u64) as Chunk);
            self.used_digits_ += 1;
            carry >>= Self::kBigitSize;
        }
    }

    pub fn multiply_by_power_of_ten(&mut self, exponent: i32) {
        const K_FIVE27: u64 = 0x6765_C793_FA10_079D;
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
        let mut remaining_exponent = exponent;
        while remaining_exponent >= 27 {
            self.multiply_by_uint64(K_FIVE27);
            remaining_exponent -= 27;
        }

        while remaining_exponent >= 13 {
            self.multiply_by_uint32(K_FIVE13);
            remaining_exponent -= 13;
        }

        if remaining_exponent > 0 {
            self.multiply_by_uint32(K_FIVE1_TO_12[(remaining_exponent - 1) as usize]);
        }
        self.shift_left(exponent);
    }

    pub fn square(&mut self) {
        assert!(self.is_clamped());
        let product_length = 2 * self.used_digits_;
        self.ensure_capacity(product_length);
        if (1 << (2 * (Self::kChunkSize - Self::kBigitSize))) <= self.used_digits_ {
            panic!("UNIMPLEMENTED");
        }

        let mut accumulator: DoubleChunk = 0;

        let copy_offset = self.used_digits_;
        let mut copy = vec![0; self.used_digits_ as usize];
        for i in 0..self.used_digits_ {
           copy[i as usize] = self.bigits_[i as usize];
        }
        self.bigits_.resize((copy_offset + self.used_digits_) as usize, 0);
        for i in 0..self.used_digits_ {
            self.bigits_[(copy_offset + i) as usize] = copy[i as usize];
        }

        for i in 0..self.used_digits_ {
            let mut bigit_index1 = i;
            let mut bigit_index2 = 0;
            while bigit_index1 >= 0 {
                let chunk1 = self.bigits_[(copy_offset + bigit_index1) as usize];
                let chunk2 = self.bigits_[(copy_offset + bigit_index2) as usize];
                accumulator += chunk1 as DoubleChunk * chunk2 as DoubleChunk;
                bigit_index1 -= 1;
                bigit_index2 += 1;
            }
            self.bigits_[i as usize] = (accumulator & Self::kBigitMask as DoubleChunk) as Chunk;
            accumulator >>= Self::kBigitSize;
        }

        for i in self.used_digits_..product_length {
            let mut bigit_index1 = self.used_digits_ - 1;
            let mut bigit_index2 = i - bigit_index1;
            while bigit_index2 < self.used_digits_ {
                let chunk1 = self.bigits_[(copy_offset + bigit_index1) as usize];
                let chunk2 = self.bigits_[(copy_offset + bigit_index2) as usize];
                accumulator += chunk1 as DoubleChunk * chunk2 as DoubleChunk;
                bigit_index1 -= 1;
                bigit_index2 += 1;
            }
            self.bigits_[i as usize] = (accumulator & Self::kBigitMask as DoubleChunk) as Chunk;
            accumulator >>= Self::kBigitSize;
        }
        assert_eq!(accumulator, 0);
        self.used_digits_ = product_length;
        self.exponent_ *= 2;
        self.clamp();
    }

    pub fn assign_power_uint16(&mut self, base: u16, power_exponent: i32) {
        assert!(base != 0);
        assert!(power_exponent >= 0);
        if power_exponent == 0 {
            self.assign_uint16(1);
            return;
        }
        self.zero();
        let mut shifts = 0;
        let mut base_val = base;
        while (base_val & 1) == 0 {
            base_val >>= 1;
            shifts += 1;
        }

        let mut bit_size: i32 = 0;
        let mut tmp_base: i32 = base_val as i32;
        while tmp_base != 0 {
            tmp_base >>= 1;
            bit_size += 1;
        }

        let final_size = bit_size * power_exponent;
        self.ensure_capacity(final_size / Self::kBigitSize + 2);

        let mut mask = 1;
        while power_exponent >= mask {
            mask <<= 1;
        }

        mask >>= 2;
        let mut this_value: u64 = base_val as u64;

        let mut delayed_multiplication = false;
        const MAX_32BITS: u64 = 0xFFFFFFFF;

        while mask != 0 && this_value <= MAX_32BITS {
            this_value = this_value * this_value;
            if (power_exponent & mask) != 0 {
                let base_bits_mask = !((1u64 << (64 - bit_size)) - 1);
                let high_bits_zero = (this_value & base_bits_mask) == 0;
                if high_bits_zero {
                    this_value *= base_val as u64;
                } else {
                    delayed_multiplication = true;
                }
            }
            mask >>= 1;
        }
        self.assign_uint64(this_value);
        if delayed_multiplication {
            self.multiply_by_uint32(base_val as u32);
        }

        while mask != 0 {
            self.square();
            if (power_exponent & mask) != 0 {
                self.multiply_by_uint32(base_val as u32);
            }
            mask >>= 1;
        }
        self.shift_left(shifts * power_exponent);
    }

    pub fn divide_modulo_int_bignum(&mut self, other: &Bignum) -> u16 {
        assert!(self.is_clamped());
        assert!(other.is_clamped());
        assert!(other.used_digits_ > 0);

        if self.bigit_length() < other.bigit_length() {
            return 0;
        }

        self.align(other);
        let mut result: u16 = 0;

        while self.bigit_length() > other.bigit_length() {
            assert!(other.bigits_[(other.used_digits_ - 1) as usize] >= ((1 << Self::kBigitSize) / 16) as Chunk);
            result += self.bigits_[(self.used_digits_ - 1) as usize] as u16;
            self.subtract_times(other, self.bigits_[(self.used_digits_ - 1) as usize] as i32);
        }
        assert!(self.bigit_length() == other.bigit_length());
        let this_bigit = self.bigits_[(self.used_digits_ - 1) as usize];
        let other_bigit = other.bigits_[(other.used_digits_ - 1) as usize];
        if other.used_digits_ == 1 {
            let quotient = this_bigit / other_bigit;
            self.bigits_[(self.used_digits_ - 1) as usize] = this_bigit - other_bigit * quotient;
            result += quotient as u16;
            self.clamp();
            return result;
        }
        let division_estimate = this_bigit / (other_bigit + 1);
        result += division_estimate as u16;
        self.subtract_times(other, division_estimate as i32);
        if other_bigit * (division_estimate + 1) > this_bigit {
            return result;
        }
        while Bignum::less_equal(other, self) {
            self.subtract_bignum(other);
            result += 1;
        }
        result
    }

    fn size_in_hex_chars(number: u64) -> i32 {
        assert!(number > 0);
        let mut result = 0;
        let mut num = number;
        while num != 0 {
            num >>= 4;
            result += 1;
        }
        result
    }

    pub fn to_hex_string(&self, buffer: &mut [u8]) -> bool {
        assert!(self.is_clamped());
        assert_eq!(Self::kBigitSize % 4, 0);
        let hex_chars_per_bigit = Self::kBigitSize / 4;
        if self.used_digits_ == 0 {
            if buffer.len() < 2 {
                return false;
            }
            buffer[0] = '0' as u8;
            buffer[1] = '\0' as u8;
            return true;
        }
        let needed_chars = (self.bigit_length() - 1) * hex_chars_per_bigit +
                            Self::size_in_hex_chars(self.bigits_[(self.used_digits_ - 1) as usize] as u64) + 1;
        if needed_chars > buffer.len() as i32 {
            return false;
        }
        let mut string_index = needed_chars - 1;
        buffer[string_index as usize] = '\0' as u8;
        string_index -= 1;
        for _ in 0..self.exponent_ {
            for _ in 0..hex_chars_per_bigit {
                buffer[string_index as usize] = '0' as u8;
                string_index -= 1;
            }
        }

        for i in 0..self.used_digits_ - 1 {
            let mut current_bigit = self.bigits_[i as usize];
            for _ in 0..hex_chars_per_bigit {
                buffer[string_index as usize] = Self::hex_char_of_value((current_bigit & 0xF) as i32) as u8;
                current_bigit >>= 4;
                string_index -= 1;
            }
        }

        let mut most_significant_bigit = self.bigits_[(self.used_digits_ - 1) as usize];
        while most_significant_bigit != 0 {
            buffer[string_index as usize] = Self::hex_char_of_value((most_significant_bigit & 0xF) as i32) as u8;
            most_significant_bigit >>= 4;
            string_index -= 1;
        }
        return true;
    }

    fn bigit_at(&self, index: i32) -> Chunk {
        if index >= self.bigit_length() {
            return 0;
        }
        if index < self.exponent_ {
            return 0;
        }
        self.bigits_[(index - self.exponent_) as usize]
    }

    pub fn compare(a: &Bignum, b: &Bignum) -> i32 {
        assert!(a.is_clamped());
        assert!(b.is_clamped());
        let bigit_length_a = a.bigit_length();
        let bigit_length_b = b.bigit_length();
        if bigit_length_a < bigit_length_b {
            return -1;
        }
        if bigit_length_a > bigit_length_b {
            return 1;
        }

        for i in (min(a.exponent_, b.exponent_)..bigit_length_a).rev() {
            let bigit_a = a.bigit_at(i);
            let bigit_b = b.bigit_at(i);
            if bigit_a < bigit_b {
                return -1;
            }
            if bigit_a > bigit_b {
                return 1;
            }
        }
        0
    }

    pub fn plus_compare(a: &Bignum, b: &Bignum, c: &Bignum) -> i32 {
        assert!(a.is_clamped());
        assert!(b.is_clamped());
        assert!(c.is_clamped());

        if a.bigit_length() < b.bigit_length() {
            return Self::plus_compare(b, a, c);
        }

        if a.bigit_length() + 1 < c.bigit_length() {
            return -1;
        }
        if a.bigit_length() > c.bigit_length() {
            return 1;
        }

        if a.exponent_ >= b.bigit_length() && a.bigit_length() < c.bigit_length() {
            return -1;
        }

        let mut borrow: Chunk = 0;
        let min_exponent = min(min(a.exponent_, b.exponent_), c.exponent_);
        for i in (min_exponent..c.bigit_length()).rev() {
            let chunk_a = a.bigit_at(i);
            let chunk_b = b.bigit_at(i);
            let chunk_c = c.bigit_at(i);
            let sum = chunk_a as u64 + chunk_b as u64;
            if sum > (chunk_c as u64 + borrow as u64) {
                return 1;
            } else {
                borrow = (chunk_c as u64 + borrow as u64 - sum) as Chunk;
                if borrow > 1 {
                    return -1;
                }
                borrow <<= Self::kBigitSize;
            }
        }

        if borrow == 0 {
            return 0;
        }

        return -1;
    }

    fn bit_size<S>(value: S) -> i32 {
        std::mem::size_of::<S>() as i32 * 8
    }

    fn hex_char_of_value(value: i32) -> char
