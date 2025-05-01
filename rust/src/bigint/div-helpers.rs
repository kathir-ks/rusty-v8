// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/bigint/div-helpers.h (Inferred Rust module definition)
pub mod div_helpers {
    use crate::bigint::bigint_internal::*;

    /// Shifts the digits of a BigInt to the left by a specified amount.
    ///
    /// This function shifts the digits of the BigInt `X` to the left by `shift` bits and stores the result in `Z`.
    /// `Z` and `X` may alias for an in-place shift.
    ///
    /// # Arguments
    ///
    /// * `Z`: A mutable slice of digits representing the destination BigInt.
    /// * `X`: A slice of digits representing the source BigInt.
    /// * `shift`: The number of bits to shift the digits to the left.
    pub fn left_shift(z: &mut [digit_t], x: &[digit_t], shift: usize) {
        debug_assert!(shift >= 0);
        debug_assert!(shift < k_digit_bits);
        debug_assert!(z.len() >= x.len());

        if shift == 0 {
            copy(z, x);
            return;
        }

        let mut carry: digit_t = 0;
        let mut i: usize = 0;

        for &d in x.iter() {
            z[i] = (d << shift) | carry;
            carry = d >> (k_digit_bits - shift);
            i += 1;
        }

        if i < z.len() {
            z[i] = carry;
            i += 1;
        } else {
            debug_assert!(carry == 0);
        }

        for j in i..z.len() {
            z[j] = 0;
        }
    }

    /// Shifts the digits of a BigInt to the right by a specified amount.
    ///
    /// This function shifts the digits of the BigInt `X` to the right by `shift` bits and stores the result in `Z`.
    /// `Z` and `X` may alias for an in-place shift.
    ///
    /// # Arguments
    ///
    /// * `Z`: A mutable slice of digits representing the destination BigInt.
    /// * `X`: A slice of digits representing the source BigInt.
    /// * `shift`: The number of bits to shift the digits to the right.
    pub fn right_shift(z: &mut [digit_t], x: &[digit_t], shift: usize) {
        debug_assert!(shift >= 0);
        debug_assert!(shift < k_digit_bits);
        let mut x_normalized = x.to_vec();
        normalize(&mut x_normalized);

        debug_assert!(z.len() >= x.len());

        if shift == 0 {
            copy(z, &x_normalized);
            return;
        }

        let mut i: usize = 0;

        if !x_normalized.is_empty() {
            let mut carry: digit_t = x_normalized[0] >> shift;
            let last: usize = x_normalized.len() - 1;

            for j in 0..last {
                let d: digit_t = x_normalized[j + 1];
                z[i] = (d << (k_digit_bits - shift)) | carry;
                carry = d >> shift;
                i += 1;
            }

            z[i] = carry;
            i += 1;
        }

        for j in i..z.len() {
            z[j] = 0;
        }
    }

    fn copy(z: &mut [digit_t], x: &[digit_t]) {
        if z as *mut _ == x as *const _ as *mut _ {
            return;
        }

        let mut i: usize = 0;
        for &digit in x.iter() {
            z[i] = digit;
            i += 1;
        }

        for j in i..z.len() {
            z[j] = 0;
        }
    }
}

// src/bigint/bigint-internal.h (Inferred Rust module definition)
pub mod bigint_internal {
    pub type digit_t = u32;
    pub const k_digit_bits: usize = 32; // Assuming digit_t is u32
    
    // A struct to mimic the RWDigits and Digits types from C++.
    // In Rust we can use slices. This definition is not strictly necessary
    // but helps make the code more similar to the C++ version.
    //
    // struct Digits<'a> {
    //     data: &'a [digit_t],
    // }
    //
    // impl<'a> Digits<'a> {
    //     fn len(&self) -> usize {
    //         self.data.len()
    //     }
    //     fn get(&self, index: usize) -> Option<&digit_t> {
    //         self.data.get(index)
    //     }
    // }
    //
    // struct RWDigits<'a> {
    //     data: &'a mut [digit_t],
    // }
    //
    // impl<'a> RWDigits<'a> {
    //     fn len(&self) -> usize {
    //         self.data.len()
    //     }
    //
    //     fn get(&self, index: usize) -> Option<&digit_t> {
    //         self.data.get(index)
    //     }
    //
    //     fn get_mut(&mut self, index: usize) -> Option<&mut digit_t> {
    //         self.data.get_mut(index)
    //     }
    // }

    //Mimics Normalize method, assuming that "Digits" is just a vector of digits
    pub fn normalize(digits: &mut Vec<digit_t>) {
        while let Some(last) = digits.last() {
            if *last == 0 && digits.len() > 1 {
                digits.pop();
            } else {
                break;
            }
        }
    }
}