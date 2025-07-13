// Converted from V8 C++ source files:
// Header: N/A
// Implementation: fromstring.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use crate::bigint::{
    bigint_internal::{Status, kFromStringLargeThreshold},
    vector_arithmetic::{Add, MultiplySingle}, Digits, digit_t, RWDigits, Compare, digit_mul, digit_add2, AddAndReturnOverflow,
};

fn should_terminate() -> bool {
    false
}

fn bit_length(x: u8) -> i32 {
    let mut len = 0;
    let mut val = x;
    while val > 0 {
        len += 1;
        val >>= 1;
    }
    len
}

fn is_power_of_two(n: u8) -> bool {
    n != 0 && (n & (n - 1)) == 0
}

#[derive(Debug)]
pub enum FromStringError {
    GenericError,
}

struct Storage {
    data: Vec<digit_t>,
}

impl Storage {
    pub fn new(size: usize) -> Self {
        Storage { data: vec![0; size] }
    }

    pub fn get(&mut self) -> *mut digit_t {
        self.data.as_mut_ptr()
    }
}

pub struct FromStringAccumulator<'a> {
    pub stack_parts_: [digit_t; 16],
    pub stack_parts_used_: usize,
    pub heap_parts_: Vec<digit_t>,
    pub max_multiplier_: digit_t,
    pub last_multiplier_: digit_t,
    pub inline_everything_: bool,
    pub radix_: u8,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> FromStringAccumulator<'a> {
    pub fn new() -> Self {
        FromStringAccumulator {
            stack_parts_: [0; 16],
            stack_parts_used_: 0,
            heap_parts_: Vec::new(),
            max_multiplier_: 0,
            last_multiplier_: 0,
            inline_everything_: false,
            radix_: 10,
            _marker: std::marker::PhantomData,
        }
    }
    pub fn ResultLength(&self) -> usize {
        if self.heap_parts_.is_empty() {
            self.stack_parts_used_
        } else {
            self.heap_parts_.len()
        }
    }
}

pub struct Processor {
    status: Status,
    work_estimate: usize,
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            status: Status::Success,
            work_estimate: 0,
        }
    }
    pub fn FromString(
        &mut self,
        Z: RWDigits,
        accumulator: &mut FromStringAccumulator,
    ) -> Status {
        let mut impl_processor = ProcessorImpl {
            processor: self,
        };
        impl_processor.FromString(Z, accumulator);
        impl_processor.processor.status
    }
}

struct ProcessorImpl<'a> {
    processor: &'a mut Processor,
}

impl<'a> ProcessorImpl<'a> {
    fn add_work_estimate(&mut self, work: usize) {
        self.processor.work_estimate += work;
    }

    fn get_and_clear_status(&mut self) -> Status {
        let status = self.processor.status;
        self.processor.status = Status::Success;
        status
    }

    // The classic algorithm: for every part, multiply the accumulator with
    // the appropriate multiplier, and add the part. O(n²) overall.
    fn FromStringClassic(&mut self, Z: RWDigits, accumulator: &mut FromStringAccumulator) {
        // We always have at least one part to process.
        assert!(accumulator.stack_parts_used_ > 0);
        Z[0] = accumulator.stack_parts_[0];
        let mut already_set = RWDigits::new(Z.digits(), 0, 1);
        for i in 1..Z.len() {
            Z[i] = 0;
        }

        // The {FromStringAccumulator} uses stack-allocated storage for the first
        // few parts; if heap storage is used at all then all parts are copied there.
        let num_stack_parts = accumulator.stack_parts_used_;
        if num_stack_parts == 1 {
            return;
        }
        let heap_parts = &accumulator.heap_parts_;
        let num_heap_parts = heap_parts.len();
        // All multipliers are the same, except possibly for the last.
        let max_multiplier = accumulator.max_multiplier_;

        if num_heap_parts == 0 {
            for i in 1..num_stack_parts - 1 {
                MultiplySingle(Z, already_set, max_multiplier);
                Add(Z, accumulator.stack_parts_[i]);
                already_set.set_len(already_set.len() + 1);
            }
            MultiplySingle(Z, already_set, accumulator.last_multiplier_);
            Add(Z, accumulator.stack_parts_[num_stack_parts - 1]);
            return;
        }
        // Parts are stored on the heap.
        for i in 1..num_heap_parts - 1 {
            MultiplySingle(Z, already_set, max_multiplier);
            Add(Z, accumulator.heap_parts_[i]);
            already_set.set_len(already_set.len() + 1);
        }
        MultiplySingle(Z, already_set, accumulator.last_multiplier_);
        Add(Z, *accumulator.heap_parts_.last().unwrap());
    }

    // The fast algorithm: combine parts in a balanced-binary-tree like order:
    // Multiply-and-add neighboring pairs of parts, then loop, until only one
    // part is left. The benefit is that the multiplications will have inputs of
    // similar sizes, which makes them amenable to fast multiplication algorithms.
    // We have to do more multiplications than the classic algorithm though,
    // because we also have to multiply the multipliers.
    // Optimizations:
    // - We can skip the multiplier for the first part, because we never need it.
    // - Most multipliers are the same; we can avoid repeated multiplications and
    //   just copy the previous result. (In theory we could even de-dupe them, but
    //   as the parts/multipliers grow, we'll need most of the memory anyway.)
    //   Copied results are marked with a * below.
    // - We can reuse memory using a system of three buffers whose usage rotates:
    //   - one is considered empty, and is overwritten with the new parts,
    //   - one holds the multipliers (and will be "empty" in the next round), and
    //   - one initially holds the parts and is overwritten with the new multipliers
    //   Parts and multipliers both grow in each iteration, and get fewer, so we
    //   use the space of two adjacent old chunks for one new chunk.
    //   Since the {heap_parts_} vectors has the right size, and so does the
    //   result {Z}, we can use that memory, and only need to allocate one scratch
    //   vector. If the final result ends up in the wrong bucket, we have to copy it
    //   to the correct one.
    // - We don't have to keep track of the positions and sizes of the chunks,
    //   because we can deduce their precise placement from the iteration index.
    //
    // Example, assuming digit_t is 4 bits, fitting one decimal digit:
    // Initial state:
    // parts_:        1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
    // multipliers_: 10 10 10 10 10 10 10 10 10 10 10 10 10 10 10
    // After the first iteration of the outer loop:
    // parts:         12    34    56    78    90    12    34    5
    // multipliers:        100  *100  *100  *100  *100  *100   10
    // After the second iteration:
    // parts:         1234        5678        9012        345
    // multipliers:              10000      *10000       1000
    // After the third iteration:
    // parts:         12345678                9012345
    // multipliers:                          10000000
    // And then there's an obvious last iteration.
    fn FromStringLarge(&mut self, Z: RWDigits, accumulator: &mut FromStringAccumulator) {
        let num_parts = accumulator.heap_parts_.len();
        assert!(num_parts >= 2);
        assert!(Z.len() >= num_parts);
        let mut parts = RWDigits::new(accumulator.heap_parts_.as_mut_ptr(), 0, num_parts);
        let mut multipliers_storage = Storage::new(num_parts);
        let multipliers = RWDigits::new(multipliers_storage.get(), 0, num_parts);
        let temp = RWDigits::new(Z.digits(), 0, num_parts);
        // Unrolled and specialized first iteration: part_len == 1, so instead of
        // Digits sub-vectors we have individual digit_t values, and the multipliers
        // are known up front.
        {
            let max_multiplier = accumulator.max_multiplier_;
            let last_multiplier = accumulator.last_multiplier_;
            let mut new_parts = temp;
            let mut new_multipliers = parts;
            let mut i = 0;
            while i + 1 < num_parts {
                let p_in = parts[i];
                let p_in2 = parts[i + 1];
                let m_in = max_multiplier;
                let m_in2 = if i == num_parts - 2 {
                    last_multiplier
                } else {
                    max_multiplier
                };
                // p[j] = p[i] * m[i+1] + p[i+1]
                let mut p_high = 0;
                let p_low = digit_mul(p_in, m_in2, &mut p_high);
                let mut carry = 0;
                new_parts[i] = digit_add2(p_low, p_in2, &mut carry);
                new_parts[i + 1] = p_high + carry;
                // m[j] = m[i] * m[i+1]
                if i > 0 {
                    if i > 2 && m_in2 != last_multiplier {
                        new_multipliers[i] = new_multipliers[i - 2];
                        new_multipliers[i + 1] = new_multipliers[i - 1];
                    } else {
                        let mut m_high = 0;
                        new_multipliers[i] = digit_mul(m_in, m_in2, &mut m_high);
                        new_multipliers[i + 1] = m_high;
                    }
                }
                i += 2;
            }
            // Trailing last part (if {num_parts} was odd).
            if i < num_parts {
                new_parts[i] = parts[i];
                new_multipliers[i] = last_multiplier;
                i += 2;
            }
            let num_parts_new = i >> 1;
            parts = multipliers;
            multipliers = temp;
            let num_parts_i32 = num_parts_new as i32;

            new_multipliers = parts;
            new_parts = multipliers;

            let tmp = temp;
            self.add_work_estimate(num_parts_new);
            parts = tmp;

             let num_parts_new = i >> 1;

            parts = multipliers;
            multipliers = temp;


             let num_parts_new = i >> 1;
             let tmp = temp;

        }
        let mut part_len = 2;

        // Remaining iterations.
        while num_parts > 1 {
            let mut new_parts = temp;
            let mut new_multipliers = parts;
            let new_part_len = part_len * 2;
            let mut i = 0;
            while i + 1 < num_parts {
                let start = i * part_len;

                let p_in = Digits::new(parts, start, part_len);
                let p_in2 = Digits::new(parts, start + part_len, part_len);
                let m_in = Digits::new(multipliers, start, part_len);
                let m_in2 = Digits::new(multipliers, start + part_len, part_len);
                let p_out = RWDigits::new(new_parts.digits(), start, new_part_len);
                let m_out = RWDigits::new(new_multipliers.digits(), start, new_part_len);
                // p[j] = p[i] * m[i+1] + p[i+1]

                // Multiply(p_out, p_in, m_in2);
                 let mut x = Vec::new();
                 for k in 0..p_in.len() {
                     x.push(p_in[k]);
                 }
                  let mut y = Vec::new();
                  for k in 0..m_in2.len() {
                      y.push(m_in2[k]);
                  }

                 // Multiply(p_out, p_in, m_in2);

                  let mut x = Vec::new();
                  for k in 0..p_in.len() {
                      x.push(p_in[k]);
                  }
                  let mut y = Vec::new();
                  for k in 0..m_in2.len() {
                      y.push(m_in2[k]);
                  }

                if should_terminate() {
                     return;
                 }
                let overflow = AddAndReturnOverflow(p_out, p_in2);
                assert!(overflow == 0);
                // m[j] = m[i] * m[i+1]
                if i > 0 {
                    let mut copied = false;
                    if i > 2 {
                        let prev_start = (i - 2) * part_len;

                        let m_in_prev = Digits::new(multipliers, prev_start, part_len);
                        let m_in2_prev = Digits::new(multipliers, prev_start + part_len, part_len);
                        if Compare(m_in, m_in_prev) == 0 && Compare(m_in2, m_in2_prev) == 0 {
                            copied = true;
                            let m_out_prev = RWDigits::new(new_multipliers.digits(), prev_start, new_part_len);
                            for k in 0..new_part_len {
                                m_out[k] = m_out_prev[k];
                            }
                        }
                    }
                    if !copied {
                         let mut x = Vec::new();
                         for k in 0..m_in.len() {
                             x.push(m_in[k]);
                         }
                          let mut y = Vec::new();
                          for k in 0..m_in2.len() {
                              y.push(m_in2[k]);
                          }
                       // Multiply(m_out, m_in, m_in2);
                        if should_terminate() {
                             return;
                         }
                    }
                }
                i += 2;
            }
            // Trailing last part (if {num_parts} was odd).
            if i < num_parts {
                let p_in = Digits::new(parts, i * part_len, part_len);
                let m_in = Digits::new(multipliers, i * part_len, part_len);
                let p_out = RWDigits::new(new_parts.digits(), i * part_len, new_part_len);
                let m_out = RWDigits::new(new_multipliers.digits(), i * part_len, new_part_len);
                let mut k = 0;
                while k < p_in.len() {
                    p_out[k] = p_in[k];
                    k += 1;
                }
                while k < p_out.len() {
                    p_out[k] = 0;
                    k += 1;
                }
                k = 0;
                while k < m_in.len() {
                    m_out[k] = m_in[k];
                    k += 1;
                }
                while k < m_out.len() {
                    m_out[k] = 0;
                    k += 1;
                }
                i += 2;
            }
            let num_parts_new = i >> 1;
            part_len = new_part_len;

            parts = multipliers;
            multipliers = temp;
           // std::mem::swap(&mut temp, &mut multipliers);
            //parts = temp;

        }
        // Copy the result to Z, if it doesn't happen to be there already.
        if parts.digits() != Z.digits() {
            let mut i = 0;
            while i < parts.len() {
                Z[i] = parts[i];
                i += 1;
            }
            // Z might be bigger than we requested; be robust towards that.
            while i < Z.len() {
                Z[i] = 0;
                i += 1;
            }
        }
    }

    // Specialized algorithms for power-of-two radixes. Designed to work with
    // {ParsePowerTwo}: {max_multiplier_} isn't saved, but {radix_} is, and
    // {last_multiplier_} has special meaning, namely the number of unpopulated bits
    // in the last part.
    // For these radixes, {parts} already is a list of correct bit sequences, we
    // just have to put them together in the right way:
    // - The parts are currently in reversed order. The highest-index parts[i]
    //   will go into Z[0].
    // - All parts, possibly except for the last, are maximally populated.
    // - A maximally populated part stores a non-fractional number of characters,
    //   i.e. the largest fitting multiple of {char_bits} of it is populated.
    // - The populated bits in a part are at the low end.
    // - The number of unused bits in the last part is stored in
    //   {accumulator->last_multiplier_}.
    //
    // Example: Given the following parts vector, where letters are used to
    // label bits, bit order is big endian (i.e. [00000101] encodes "5"),
    // 'x' means "unpopulated", kDigitBits == 8, radix == 8, and char_bits == 3:
    //
    //     parts[0] -> [xxABCDEF][xxGHIJKL][xxMNOPQR][xxxxxSTU] <- parts[3]
    //
    // We have to assemble the following result:
    //
    //         Z[0] -> [NOPQRSTU][FGHIJKLM][xxxABCDE] <- Z[2]
    //
    fn FromStringBasePowerOfTwo(&mut self, Z: RWDigits, accumulator: &mut FromStringAccumulator) {
        let num_parts = accumulator.ResultLength();
        assert!(num_parts >= 1);
        assert!(Z.len() >= num_parts);
        let parts = Digits::new(
            if accumulator.heap_parts_.is_empty() {
                accumulator.stack_parts_.as_ptr() as *mut digit_t
            } else {
                accumulator.heap_parts_.as_ptr() as *mut digit_t
            },
            0,
            num_parts,
        );
        let radix = accumulator.radix_;
        assert!(
            radix == 2 || radix == 4 || radix == 8 || radix == 16 || radix == 32
        );
        let char_bits = bit_length(radix - 1);
        let unused_last_part_bits = accumulator.last_multiplier_ as i32;
        let unused_part_bits = (std::mem::size_of::<digit_t>() * 8) as i32 % char_bits;
        let max_part_bits = (std::mem::size_of::<digit_t>() * 8) as i32 - unused_part_bits;
        let mut z_index = 0;
        let mut part_index = num_parts - 1;

        // If the last part is fully populated, then all parts must be, and we can
        // simply copy them (in reversed order).
        if unused_last_part_bits == 0 {
            assert!((std::mem::size_of::<digit_t>() * 8) as i32 % char_bits == 0);
            while part_index >= 0 {
                Z[z_index] = parts[part_index];
                z_index += 1;
                part_index -= 1;
            }
            while z_index < Z.len() {
                Z[z_index] = 0;
                z_index += 1;
            }
            return;
        }

        // Otherwise we have to shift parts contents around as needed.
        // Holds the next Z digit that we want to store...
        let mut digit = parts[part_index];
        part_index -= 1;
        // ...and the number of bits (at the right end) we already know.
        let mut digit_bits = (std::mem::size_of::<digit_t>() * 8) as i32 - unused_last_part_bits;
        while part_index >= 0 {
            // Holds the last part that we read from {parts}...
            let mut part;
            // ...and the number of bits (at the right end) that we haven't used yet.
            let mut part_bits;
            while digit_bits < (std::mem::size_of::<digit_t>() * 8) as i32 {
                part = parts[part_index];
                part_index -= 1;
                part_bits = max_part_bits;
                digit |= part << digit_bits;
                let part_shift = (std::mem::size_of::<digit_t>() * 8) as i32 - digit_bits;
                if part_shift > part_bits {
                    digit_bits += part_bits;
                    part = 0;
                    part_bits = 0;
                    if part_index < 0 {
                        break;
                    }
                } else {
                    digit_bits = (std::mem::size_of::<digit_t>() * 8) as i32;
                    part >>= part_shift;
                    part_bits -= part_shift;
                }
            }
            Z[z_index] = digit;
            z_index += 1;
            digit = part;
            digit_bits = part_bits;
        }
        if digit_bits > 0 {
            Z[z_index] = digit;
            z_index += 1;
        }
        while z_index < Z.len() {
            Z[z_index] = 0;
            z_index += 1;
        }
    }

    fn FromString(&mut self, Z: RWDigits, accumulator: &mut FromStringAccumulator) {
        if accumulator.inline_everything_ {
            let mut i = 0;
            while i < accumulator.stack_parts_used_ {
                Z[i] = accumulator.stack_parts_[i];
                i += 1;
            }
            while i < Z.len() {
                Z[i] = 0;
                i += 1;
            }
        } else if accumulator.stack_parts_used_ == 0 {
            for i in 0..Z.len() {
                Z[i] = 0;
            }
        } else if is_power_of_two(accumulator.radix_) {
            self.FromStringBasePowerOfTwo(Z, accumulator);
        } else if accumulator.ResultLength() < kFromStringLargeThreshold {
            self.FromStringClassic(Z, accumulator);
        } else {
            self.FromStringLarge(Z, accumulator);
        }
    }
}
