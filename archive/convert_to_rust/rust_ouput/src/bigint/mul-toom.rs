// Converted from V8 C++ source files:
// Header: N/A
// Implementation: mul-toom.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Toom-Cook multiplication.
// Reference: https://en.wikipedia.org/wiki/Toom%E2%80%93Cook_multiplication

use std::cmp::max;

//use crate::bigint::bigint_internal::*;
//use crate::bigint::digit_arithmetic::*;
//use crate::bigint::vector_arithmetic::*;

mod bigint_internal {
    pub type digit_t = u32;
    pub const kDigitBits: usize = 32;
    pub const kHalfDigitBits: usize = 16;
    pub const kHalfDigitMask: digit_t = (1 << kHalfDigitBits) - 1;

    #[derive(Debug, Clone, Copy)]
    pub struct Digits<'a> {
        ptr: &'a [digit_t],
        start: usize,
        len: usize,
    }

    impl<'a> Digits<'a> {
        pub fn new(ptr: &'a [digit_t], start: usize, len: usize) -> Self {
            Digits { ptr, start, len }
        }

        pub fn len(&self) -> usize {
            self.len
        }

        pub fn is_empty(&self) -> bool {
            self.len == 0
        }

        pub fn get(&self, index: usize) -> Option<&digit_t> {
            if index < self.len {
                self.ptr.get(self.start + index)
            } else {
                None
            }
        }
    }

    impl<'a> std::ops::Index<usize> for Digits<'a> {
        type Output = digit_t;

        fn index(&self, index: usize) -> &Self::Output {
            &self.ptr[self.start + index]
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct RWDigits<'a> {
        ptr: &'a mut [digit_t],
        start: usize,
        len: usize,
    }

    impl<'a> RWDigits<'a> {
        pub fn new(ptr: &'a mut [digit_t], start: usize, len: usize) -> Self {
            RWDigits { ptr, start, len }
        }

        pub fn len(&self) -> usize {
            self.len
        }

        pub fn is_empty(&self) -> bool {
            self.len == 0
        }

        pub fn get(&self, index: usize) -> Option<&digit_t> {
            if index < self.len {
                self.ptr.get(self.start + index)
            } else {
                None
            }
        }

        pub fn get_mut(&mut self, index: usize) -> Option<&mut digit_t> {
            if index < self.len {
                self.ptr.get_mut(self.start + index)
            } else {
                None
            }
        }

        pub fn normalize(&mut self) {
            while self.len > 0 && self[self.len - 1] == 0 {
                self.len -= 1;
            }
        }
    }

    impl<'a> std::ops::Index<usize> for RWDigits<'a> {
        type Output = digit_t;

        fn index(&self, index: usize) -> &Self::Output {
            &self.ptr[self.start + index]
        }
    }

    impl<'a> std::ops::IndexMut<usize> for RWDigits<'a> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.ptr[self.start + index]
        }
    }
}

mod digit_arithmetic {
    use super::bigint_internal::digit_t;
    use super::bigint_internal::kDigitBits;

    pub fn add(z: &mut [digit_t], x: &[digit_t], y: &[digit_t]) {
        let len = x.len().max(y.len());
        let mut carry: digit_t = 0;

        for i in 0..len {
            let x_val = if i < x.len() { x[i] as u64 } else { 0 };
            let y_val = if i < y.len() { y[i] as u64 } else { 0 };

            let sum = x_val + y_val + carry as u64;
            z[i] = (sum & ((1u64 << kDigitBits) - 1)) as digit_t;
            carry = (sum >> kDigitBits) as digit_t;
        }

        if carry != 0 {
            z[len] = carry;
        }
    }

    pub fn multiply(z: &mut [digit_t], x: &[digit_t], y: &[digit_t]) {
        for i in 0..x.len() {
            for j in 0..y.len() {
                let product = (x[i] as u64) * (y[j] as u64);
                let index = i + j;
                let mut carry: digit_t = (product >> super::bigint_internal::kDigitBits) as digit_t;
                z[index] += (product & ((1u64 << super::bigint_internal::kDigitBits) - 1)) as digit_t;
                if z[index] < (product & ((1u64 << super::bigint_internal::kDigitBits) - 1)) as digit_t {
                  carry += 1;
                }
                if carry > 0 {
                    let mut k = index + 1;
                    while carry > 0 {
                        z[k] += carry;
                        if z[k] < carry {
                            carry = 1;
                        } else {
                            carry = 0;
                        }
                        k += 1;
                    }
                }

            }
        }
    }

    pub fn subtract(z: &mut [digit_t], x: &[digit_t], y: &[digit_t]) -> bool {
        let mut borrow: digit_t = 0;
        for i in 0..x.len().max(y.len()) {
            let x_val = if i < x.len() { x[i] as u64 } else { 0 };
            let y_val = if i < y.len() { y[i] as u64 } else { 0 };

            let diff = (x_val as u64) - (y_val as u64) - (borrow as u64);
            if diff & (1u64 << 63) != 0 {
                z[i] = ((1u64 << super::bigint_internal::kDigitBits) + diff) as digit_t;
                borrow = 1;
            } else {
                z[i] = diff as digit_t;
                borrow = 0;
            }
        }
        borrow != 0
    }
}

mod vector_arithmetic {
    use super::bigint_internal::{digit_t, Digits, RWDigits};
    use super::digit_arithmetic;

    pub fn add(dest: RWDigits, x: Digits, y: Digits) {
        let x_slice = &x.ptr[x.start..x.start + x.len];
        let y_slice = &y.ptr[y.start..y.start + y.len];
        let mut dest_slice = dest.ptr[dest.start..dest.start + dest.len].to_vec(); // Create a mutable copy
        digit_arithmetic::add(&mut dest_slice, x_slice, y_slice);
        for i in 0..dest.len() {
            dest.ptr[dest.start + i] = dest_slice[i];
        }
    }

    pub fn multiply(dest: RWDigits, x: Digits, y: Digits) {
        let x_slice = &x.ptr[x.start..x.start + x.len];
        let y_slice = &y.ptr[y.start..y.start + y.len];
        let mut dest_slice = vec![0; dest.len()];
        digit_arithmetic::multiply(&mut dest_slice, x_slice, y_slice);
         for i in 0..dest.len() {
             dest.ptr[dest.start + i] = dest_slice[i];
         }
    }

    pub fn subtract_signed(dest: RWDigits, x: RWDigits, x_sign: bool, y: Digits, y_sign: bool) -> bool {
        if x_sign == y_sign {
            let x_slice = &x.ptr[x.start..x.start + x.len];
            let y_slice = &y.ptr[y.start..y.start + y.len];
            let mut dest_slice = x_slice.to_vec(); // Create a mutable copy of x
            let borrow = digit_arithmetic::subtract(&mut dest_slice, x_slice, y_slice);
             for i in 0..dest.len() {
                 dest.ptr[dest.start + i] = dest_slice[i];
             }
            borrow
        } else {
             add(dest, Digits::new(x.ptr,x.start, x.len), y);
             false // Result will have the sign of X.  No overflow for addition.
        }
    }

    pub fn add_signed(dest: RWDigits, x: RWDigits, x_sign: bool, y: Digits, y_sign: bool) -> bool {
        if x_sign == y_sign {
            add(dest, Digits::new(x.ptr, x.start, x.len), y);
            false // Result will have the same sign. No overflow for addition.
        } else {
            let x_slice = &x.ptr[x.start..x.start + x.len];
            let y_slice = &y.ptr[y.start..y.start + y.len];
            let mut dest_slice = x_slice.to_vec(); // Create a mutable copy of x
            let borrow = digit_arithmetic::subtract(&mut dest_slice, x_slice, y_slice);
            for i in 0..dest.len() {
                dest.ptr[dest.start + i] = dest_slice[i];
            }
            borrow
        }
    }
}

pub mod mul_toom {
    use super::*;
    use bigint_internal::{digit_t, Digits, RWDigits, kDigitBits, kHalfDigitBits, kHalfDigitMask};
    use digit_arithmetic;
    use vector_arithmetic;

    fn div_ceil(a: usize, b: usize) -> usize {
        (a + b - 1) / b
    }

    fn times_two(x: &mut RWDigits) {
        let mut carry: digit_t = 0;
        for i in 0..x.len() {
            let d = x[i];
            x[i] = (d << 1) | carry;
            carry = d >> (kDigitBits - 1);
        }
    }

    fn divide_by_two(x: &mut RWDigits) {
        let mut carry: digit_t = 0;
        for i in (0..x.len()).rev() {
            let d = x[i];
            x[i] = (d >> 1) | carry;
            carry = d << (kDigitBits - 1);
        }
    }

    fn divide_by_three(x: &mut RWDigits) {
        let mut remainder: digit_t = 0;
        for i in (0..x.len()).rev() {
            let d = x[i];
            let upper = (remainder << kHalfDigitBits) | (d >> kHalfDigitBits);
            let u_result = upper / 3;
            remainder = upper - 3 * u_result;
            let lower = (remainder << kHalfDigitBits) | (d & kHalfDigitMask);
            let l_result = lower / 3;
            remainder = lower - 3 * l_result;
            x[i] = (u_result << kHalfDigitBits) | l_result;
        }
    }

    #[derive(Debug)]
    struct Storage {
        data: Vec<digit_t>,
    }

    impl Storage {
        fn new(size: usize) -> Self {
            Storage { data: vec![0; size] }
        }

        fn get(&mut self) -> *mut digit_t {
            self.data.as_mut_ptr()
        }
    }

    macro_rules! mark_invalid {
        ($d:expr) => {
            $d = RWDigits::new(&mut [0; 1], 0, 1);
        };
    }

    pub struct ProcessorImpl {}

    impl ProcessorImpl {
        pub fn toom3_main(&self, z: &mut RWDigits, x: Digits, y: Digits) {
            assert!(z.len() >= x.len() + y.len());

            // Phase 1: Splitting.
            let i = div_ceil(max(x.len(), y.len()), 3);
            let x0 = Digits::new(x.ptr, x.start, i);
            let x1 = Digits::new(x.ptr, x.start + i, i);
            let x2 = Digits::new(x.ptr, x.start + 2 * i, i);
            let y0 = Digits::new(y.ptr, y.start, i);
            let y1 = Digits::new(y.ptr, y.start + i, i);
            let y2 = Digits::new(y.ptr, y.start + 2 * i, i);

            // Temporary storage.
            let p_len = i + 1;
            let r_len = 2 * p_len;
            let mut temp_storage = Storage::new(4 * r_len);
            let t = temp_storage.get();

            let mut po = RWDigits::new(unsafe { std::slice::from_raw_parts_mut(t, p_len) }, 0, p_len);
            let mut qo = RWDigits::new(unsafe { std::slice::from_raw_parts_mut(t.add(p_len), p_len) }, 0, p_len);
            let mut p_1 = RWDigits::new(unsafe { std::slice::from_raw_parts_mut(t.add(r_len), p_len) }, 0, p_len);
            let mut q_1 = RWDigits::new(unsafe { std::slice::from_raw_parts_mut(t.add(r_len + p_len), p_len) }, 0, p_len);
            let mut r_1 = RWDigits::new(unsafe { std::slice::from_raw_parts_mut(t.add(2 * r_len), r_len) }, 0, r_len);
            let mut r_m1 = RWDigits::new(unsafe { std::slice::from_raw_parts_mut(t.add(3 * r_len), r_len) }, 0, r_len);

            assert!(z.len() >= r_len);
            let mut r_0 = RWDigits::new(z.ptr, z.start, r_len);

            // Phase 2a: Evaluation, steps 0, 1, m1.
            // po = X0 + X2
            vector_arithmetic::add(po, x0, x2);
            // p_0 = X0
            // p_1 = po + X1
            vector_arithmetic::add(p_1, Digits::new(po.ptr, po.start, po.len), x1);
            // p_m1 = po - X1
            let mut p_m1 = RWDigits::new(po.ptr, po.start, po.len);
            let p_m1_sign = vector_arithmetic::subtract_signed(
                RWDigits::new(p_m1.ptr, p_m1.start, p_m1.len),
                RWDigits::new(po.ptr, po.start, po.len),
                false,
                x1,
                false,
            );
            mark_invalid!(po);

            // qo = Y0 + Y2
            vector_arithmetic::add(qo, y0, y2);
            // q_0 = Y0
            // q_1 = qo + Y1
            vector_arithmetic::add(q_1, Digits::new(qo.ptr, qo.start, qo.len), y1);
            // q_m1 = qo - Y1
            let mut q_m1 = RWDigits::new(qo.ptr, qo.start, qo.len);
            let q_m1_sign = vector_arithmetic::subtract_signed(
                RWDigits::new(q_m1.ptr, q_m1.start, q_m1.len),
                RWDigits::new(qo.ptr, qo.start, qo.len),
                false,
                y1,
                false,
            );
            mark_invalid!(qo);

            // Phase 3a: Pointwise multiplication, steps 0, 1, m1.
            vector_arithmetic::multiply(RWDigits::new(r_0.ptr, r_0.start, r_0.len), x0, y0);
            vector_arithmetic::multiply(RWDigits::new(r_1.ptr, r_1.start, r_1.len), Digits::new(p_1.ptr, p_1.start, p_1.len), Digits::new(q_1.ptr, q_1.start, q_1.len));
            vector_arithmetic::multiply(RWDigits::new(r_m1.ptr, r_m1.start, r_m1.len), Digits::new(p_m1.ptr, p_m1.start, p_m1.len), Digits::new(q_m1.ptr, q_m1.start, q_m1.len));
            let r_m1_sign = p_m1_sign != q_m1_sign;

            // Phase 2b: Evaluation, steps m2 and inf.
            // p_m2 = (p_m1 + X2) * 2 - X0
            let mut p_m2 = RWDigits::new(p_1.ptr, p_1.start, p_1.len);
            mark_invalid!(p_1);
            let mut p_m2_sign = vector_arithmetic::add_signed(
                RWDigits::new(p_m2.ptr, p_m2.start, p_m2.len),
                RWDigits::new(p_m1.ptr, p_m1.start, p_m1.len),
                p_m1_sign,
                x2,
                false,
            );
            times_two(&mut p_m2);
            p_m2_sign = vector_arithmetic::subtract_signed(
                RWDigits::new(p_m2.ptr, p_m2.start, p_m2.len),
                RWDigits::new(p_m2.ptr, p_m2.start, p_m2.len),
                p_m2_sign,
                x0,
                false,
            );
            // p_inf = X2

            // q_m2 = (q_m1 + Y2) * 2 - Y0
            let mut q_m2 = RWDigits::new(q_1.ptr, q_1.start, q_1.len);
            mark_invalid!(q_1);
            let mut q_m2_sign = vector_arithmetic::add_signed(
                RWDigits::new(q_m2.ptr, q_m2.start, q_m2.len),
                RWDigits::new(q_m1.ptr, q_m1.start, q_m1.len),
                q_m1_sign,
                y2,
                false,
            );
            times_two(&mut q_m2);
            q_m2_sign = vector_arithmetic::subtract_signed(
                RWDigits::new(q_m2.ptr, q_m2.start, q_m2.len),
                RWDigits::new(q_m2.ptr, q_m2.start, q_m2.len),
                q_m2_sign,
                y0,
                false,
            );
            // q_inf = Y2

            // Phase 3b: Pointwise multiplication, steps m2 and inf.
            let mut r_m2 = RWDigits::new(unsafe { std::slice::from_raw_parts_mut(t, r_len) }, 0, r_len);
            mark_invalid!(p_m1);
            mark_invalid!(q_m1);
            vector_arithmetic::multiply(RWDigits::new(r_m2.ptr, r_m2.start, r_m2.len), Digits::new(p_m2.ptr, p_m2.start, p_m2.len), Digits::new(q_m2.ptr, q_m2.start, q_m2.len));
            let r_m2_sign = p_m2_sign != q_m2_sign;

            let mut r_inf = RWDigits::new(unsafe { std::slice::from_raw_parts_mut(t.add(r_len), r_len) }, 0, r_len);
            mark_invalid!(p_m2);
            mark_invalid!(q_m2);
            vector_arithmetic::multiply(RWDigits::new(r_inf.ptr, r_inf.start, r_inf.len), x2, y2);

            // Phase 4: Interpolation.
            let r0 = Digits::new(r_0.ptr, r_0.start, r_0.len);
            let r4 = Digits::new(r_inf.ptr, r_inf.start, r_inf.len);
            // R3 <- (r_m2 - r_1) / 3
            let mut r3 = RWDigits::new(r_m2.ptr, r_m2.start, r_m2.len);
            let mut r3_sign = vector_arithmetic::subtract_signed(
                RWDigits::new(r3.ptr, r3.start, r3.len),
                RWDigits::new(r_m2.ptr, r_m2.start, r_m2.len),
                r_m2_sign,
                Digits::new(r_1.ptr, r_1.start, r_1.len),
                false,
            );
            divide_by_three(&mut r3);
            // R1 <- (r_1 - r_m1) / 2
            let mut r1 = RWDigits::new(r_1.ptr, r_1.start, r_1.len);
            let mut r1_sign = vector_arithmetic::subtract_signed(
                RWDigits::new(r1.ptr, r1.start, r1.len),
                RWDigits::new(r_1.ptr, r_1.start, r_1.len),
                false,
                Digits::new(r_m1.ptr, r_m1.start, r_m1.len),
                r_m1_sign,
            );
            divide_by_two(&mut r1);
            // R2 <- r_m1 - r_0
            let mut r2 = RWDigits::new(r_m1.ptr, r_m1.start, r_m1.len);
            let mut r2_sign = vector_arithmetic::subtract_signed(
                RWDigits::new(r2.ptr, r2.start, r2.len),
                RWDigits::new(r_m1.ptr, r_m1.start, r_m1.len),
                r_m1_sign,
                r0,
                false,
            );
            // R3 <- (R2 - R3) / 2 + 2 * r_inf
            r3_sign = vector_arithmetic::subtract_signed(
                RWDigits::new(r3.ptr, r3.start, r3.len),
                RWDigits::new(r2.ptr, r2.start, r2.len),
                r2_sign,
                Digits::new(r3.ptr, r3.start, r3.len),
                r3_sign,
            );
            divide_by_two(&mut r3);
            r3_sign = vector_arithmetic::add_signed(
                RWDigits::new(r3.ptr, r3.start, r3.len),
                RWDigits::new(r3.ptr, r3.start, r3.len),
                r3_sign,
                r4,
                false,
            );
            r3_sign = vector_arithmetic::add_signed(
                RWDigits::new(r3.ptr, r3.start, r3.len),
                RWDigits::new(r3.ptr, r3.start, r3.len),
                r3_sign,
                r4,
                false,
            );
            // R2 <- R2 + R1 - R4
            r2_sign = vector_arithmetic::add_signed(
                RWDigits::new(r2.ptr, r2.start, r2.len),
                RWDigits::new(r2.ptr, r2.start, r2.len),
                r2_sign,
                Digits::new(r1.ptr, r1.start, r1.len),
                r1_sign,
            );
            r2_sign = vector_arithmetic::subtract_signed(
                RWDigits::new(r2.ptr, r2.start, r2.len),
                RWDigits::new(r2.ptr, r2.start, r2.len),
                r2_sign,
                r4,
                false,
            );
            // R1 <- R1 - R3
            r1_sign = vector_arithmetic::subtract_signed(
                RWDigits::new(r1.ptr, r1.start, r1.len),
                RWDigits::new(r1.ptr, r1.start, r1.len),
                r1_sign,
                Digits::new(r3.ptr, r3.start, r3.len),
                r3_sign,
            );

            r1.normalize();
            r2.normalize();
            r3.normalize();
            assert!(r1_sign == false || r1.len() == 0);
            assert!(r2_sign == false || r2.len() == 0);
            assert!(r3_sign == false || r3.len() == 0);

            // Phase 5: Recomposition. R0 is already in place. Overflow can't happen.
            for j in r0.len()..z.len() {
                z[j] = 0;
            }
            add_and_return_overflow(
                RWDigits::new(z.ptr, z.start + i, z.len - i),
                Digits::new(r1.ptr, r1.start, r1.len),
            );
            add_and_return_overflow(
                RWDigits::new(z.ptr, z.start + 2 * i, z.len - 2 * i),
                Digits::new(r2.ptr, r2.start, r2.len),
            );
            add_and_return_overflow(
                RWDigits::new(z.ptr, z.start + 3 * i, z.len - 3 * i),
                Digits::new(r3.ptr, r3.start, r3.len),
            );
            add_and_return_overflow(
                RWDigits::new(z.ptr, z.start + 4 * i, z.len - 4 * i),
                r4,
            );
        }

        pub fn multiply_toom_cook(&self, z: &mut RWDigits, x: Digits, y: Digits) {
            assert!(x.len() >= y.len());
            let k = y.len();
            let x0 = Digits::new(x.ptr, x.start, k);
            self.toom3_main(z, x0, y);
            if x.len() > y.len() {
                let mut t_data = vec![0; 2 * k];
                let mut t = RWDigits::new(&mut t_data, 0, 2 * k);
                for i in (k..x.len()).step_by(k) {
                    let xi = Digits::new(x.ptr, x.start + i, k.min(x.len() - i));
                    self.toom3_main(&mut t, xi, y);
                    add_and_return_overflow(
                        RWDigits::new(z.ptr, z.start + i, z.len() - i),
                        Digits::new(t.ptr, t.start, t.len),
                    );
                }
            }
        }
    }

    pub struct ScratchDigits {
        data: Vec<digit_t>,
    }

    impl ScratchDigits {
        pub fn new(size: usize) -> Self {
            ScratchDigits { data: vec![0; size] }
        }
    }

    fn add_and_return_overflow(dest: RWDigits, source: Digits) -> bool {
         let mut carry: digit_t = 0;
         for i in 0..dest.len().max(source.len()) {
            let dest_val = if i < dest.len() { dest[i] as u64} else { 0 };
            let source_val = if i < source.len() { source[i] as u64} else { 0 };

             let sum = dest_val + source_val + (carry as u64);
             if i < dest.len() {
                 dest.ptr[dest.start + i] = (sum & ((1u64 << kDigitBits) - 1)) as digit_t;
             }
             carry = (sum >> kDigitBits) as digit_t;
         }
         if carry > 0 {
             return true;
         }
         return false;
    }
}
