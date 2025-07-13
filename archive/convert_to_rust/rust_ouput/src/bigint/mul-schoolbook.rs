// Converted from V8 C++ source files:
// Header: N/A
// Implementation: mul-schoolbook.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/bigint/bigint-internal.h and src/bigint/digit-arithmetic.h and src/bigint/vector-arithmetic.h are implicitly included here

mod bigint_internal {
    pub type digit_t = u32;
}

use bigint_internal::digit_t;

mod digit_arithmetic {
    use super::bigint_internal::digit_t;

    pub fn digit_mul(x: digit_t, y: digit_t, high: &mut digit_t) -> digit_t {
        let result = (x as u64) * (y as u64);
        *high = (result >> 32) as digit_t;
        (result & 0xFFFFFFFF) as digit_t
    }

    pub fn digit_add2(x: digit_t, y: digit_t, carry: &mut digit_t) -> digit_t {
        let sum = (x as u64) + (y as u64) + (*carry as u64);
        *carry = (sum >> 32) as digit_t;
        (sum & 0xFFFFFFFF) as digit_t
    }

    pub fn digit_add3(x: digit_t, y: digit_t, z: digit_t, carry: &mut digit_t) -> digit_t {
        let sum = (x as u64) + (y as u64) + (z as u64) + (*carry as u64);
        *carry = (sum >> 32) as digit_t;
        (sum & 0xFFFFFFFF) as digit_t
    }
}

mod vector_arithmetic {
    use super::bigint_internal::digit_t;

    #[derive(Debug)]
    pub struct Digits<'a> {
        data: &'a [digit_t],
    }

    impl<'a> Digits<'a> {
        pub fn new(data: &'a [digit_t]) -> Self {
            Digits { data }
        }

        pub fn len(&self) -> usize {
            self.data.len()
        }

        pub fn is_empty(&self) -> bool {
            self.data.is_empty()
        }
    }

    impl<'a> std::ops::Index<usize> for Digits<'a> {
        type Output = digit_t;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    #[derive(Debug)]
    pub struct RWDigits<'a> {
        data: &'a mut [digit_t],
    }

    impl<'a> RWDigits<'a> {
        pub fn new(data: &'a mut [digit_t]) -> Self {
            RWDigits { data }
        }

        pub fn len(&self) -> usize {
            self.data.len()
        }

        pub fn clear(&mut self) {
            for i in 0..self.data.len() {
                self.data[i] = 0;
            }
        }
    }

    impl<'a> std::ops::Index<usize> for RWDigits<'a> {
        type Output = digit_t;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    impl<'a> std::ops::IndexMut<usize> for RWDigits<'a> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.data[index]
        }
    }
}

use digit_arithmetic::{digit_add2, digit_add3, digit_mul};
use vector_arithmetic::{Digits, RWDigits};

fn is_digit_normalized(x: Digits) -> bool {
    !x.is_empty() && x[x.len() - 1] != 0
}

pub struct ProcessorImpl {
    work_estimate: u64,
}

impl ProcessorImpl {
    pub fn new() -> Self {
        ProcessorImpl { work_estimate: 0 }
    }

    fn add_work_estimate(&mut self, work: usize) {
        self.work_estimate += work as u64;
    }

    pub fn multiply_single(&mut self, z: RWDigits, x: Digits, y: digit_t) {
        assert_ne!(y, 0);
        let mut carry: digit_t = 0;
        let mut high: digit_t = 0;
        for i in 0..x.len() {
            let mut new_high: digit_t = 0;
            let low: digit_t = digit_mul(x[i], y, &mut new_high);
            z[i] = digit_add3(low, high, carry, &mut carry);
            high = new_high;
        }
        self.add_work_estimate(x.len());
        z[x.len()] = carry + high;
        for i in x.len() + 1..z.len() {
            z[i] = 0;
        }
    }

    pub fn multiply_schoolbook(&mut self, mut z: RWDigits, x: Digits, y: Digits) {
        assert!(is_digit_normalized(x));
        assert!(is_digit_normalized(y));
        assert!(x.len() >= y.len());
        assert!(z.len() >= x.len() + y.len());

        if x.len() == 0 || y.len() == 0 {
            z.clear();
            return;
        }

        let mut next: digit_t;
        let mut next_carry: digit_t = 0;
        let mut carry: digit_t = 0;

        // Unrolled first iteration: it's trivial.
        next = digit_mul(x[0], y[0], &mut next);
        z[0] = next;

        let mut i = 1;

        // Unrolled second iteration: a little less setup.
        if i < y.len() {
            let mut zi = next;
            next = 0;

            for j in 0..=1 {
                let mut high: digit_t = 0;
                let low: digit_t = digit_mul(x[j], y[i - j], &mut high);
                let mut carrybit: digit_t;
                zi = digit_add2(zi, low, &mut carrybit);
                carry += carrybit;
                next = digit_add2(next, high, &mut carrybit);
                next_carry += carrybit;
            }
            z[i] = zi;
            i += 1;
        }

        // Main part: since X.len() >= Y.len() > i, no bounds checks are needed.
        for (; i < y.len(); i++) {
            let mut zi: digit_t;
            let mut carrybit: digit_t;

            zi = digit_add2(next, carry, &mut carrybit);
            next = next_carry + carry;
            carry = 0;
            next_carry = 0;
            carry = carrybit;

            for j in 0..=i {
                let mut high: digit_t = 0;
                let low: digit_t = digit_mul(x[j], y[i - j], &mut high);
                let mut carrybit: digit_t;
                zi = digit_add2(zi, low, &mut carrybit);
                carry += carrybit;
                next = digit_add2(next, high, &mut carrybit);
                next_carry += carrybit;
            }
            z[i] = zi;

            self.add_work_estimate(i);
        }

        // Last part: i exceeds Y now, we have to be careful about bounds.
        let loop_end = x.len() + y.len() - 2;
        for (; i <= loop_end; i++) {
            let max_x_index = std::cmp::min(i, x.len() - 1);
            let max_y_index = y.len() - 1;
            let min_x_index = i - max_y_index;

            let mut zi: digit_t;
            let mut carrybit: digit_t;

            zi = digit_add2(next, carry, &mut carrybit);
            next = next_carry + carry;
            carry = 0;
            next_carry = 0;
            carry = carrybit;
            for j in min_x_index..=max_x_index {
                let mut high: digit_t = 0;
                let low: digit_t = digit_mul(x[j], y[i - j], &mut high);
                let mut carrybit: digit_t;
                zi = digit_add2(zi, low, &mut carrybit);
                carry += carrybit;
                next = digit_add2(next, high, &mut carrybit);
                next_carry += carrybit;
            }
            z[i] = zi;
            self.add_work_estimate(max_x_index - min_x_index);
        }

        // Write the last digit, and zero out any extra space in Z.
        let mut carrybit: digit_t;
        z[i] = digit_add2(next, carry, &mut carrybit);
        i += 1;
        assert_eq!(carrybit, 0);
        for (; i < z.len(); i++) {
            z[i] = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vector_arithmetic::{Digits, RWDigits};

    #[test]
    fn test_multiply_single() {
        let mut processor = ProcessorImpl::new();
        let mut z_data = vec![0u32; 5];
        let x_data = vec![1u32, 2u32, 3u32];
        let y: digit_t = 5;

        let z = RWDigits::new(&mut z_data);
        let x = Digits::new(&x_data);

        processor.multiply_single(z, x, y);

        assert_eq!(z_data[0], 5);
        assert_eq!(z_data[1], 10);
        assert_eq!(z_data[2], 15);
        assert_eq!(z_data[3], 0);
        assert_eq!(z_data[4], 0);
    }

    #[test]
    fn test_multiply_schoolbook() {
        let mut processor = ProcessorImpl::new();
        let mut z_data = vec![0u32; 5];
        let x_data = vec![1u32, 2u32];
        let y_data = vec![3u32, 4u32];

        let z = RWDigits::new(&mut z_data);
        let x = Digits::new(&x_data);
        let y = Digits::new(&y_data);

        processor.multiply_schoolbook(z, x, y);

        assert_eq!(z_data[0], 3);
        assert_eq!(z_data[1], 10);
        assert_eq!(z_data[2], 8);
        assert_eq!(z_data[3], 0);
        assert_eq!(z_data[4], 0);
    }
}
