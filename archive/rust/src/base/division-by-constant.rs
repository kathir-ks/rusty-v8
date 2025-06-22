// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// Magic numbers for division via multiplication.
/// See Warren's "Hacker's Delight", chapter 10.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct MagicNumbersForDivision<T> {
    /// The multiplier.
    pub multiplier: T,
    /// The shift value.
    pub shift: u32,
    /// Whether to add before shifting.
    pub add: bool,
}

impl<T> MagicNumbersForDivision<T> {
    /// Creates a new `MagicNumbersForDivision` instance.
    pub fn new(multiplier: T, shift: u32, add: bool) -> Self {
        Self {
            multiplier,
            shift,
            add,
        }
    }
}

/// Calculates the multiplier and shift for signed division via multiplication.
/// The divisor must not be -1, 0 or 1 when interpreted as a signed value.
pub fn signed_division_by_constant<T>(d: T) -> MagicNumbersForDivision<T>
where
    T: num_traits::Unsigned + num_traits::PrimInt,
{
    unsigned_division_by_constant(d, 0)
}

/// Calculates the multiplier and shift for unsigned division via multiplication.
/// See Warren's "Hacker's Delight", chapter 10. The divisor must not be 0 and
/// leading_zeros can be used to speed up the calculation if the given number of
/// upper bits of the dividend value are known to be zero.
pub fn unsigned_division_by_constant<T>(
    d: T,
    leading_zeros: u32,
) -> MagicNumbersForDivision<T>
where
    T: num_traits::Unsigned + num_traits::PrimInt,
{
    // Implementation based on Hacker's Delight.
    let b = T::from(1).unsigned_shl(T::from(T::BITS as u64 - 1 - leading_zeros).to_u64().unwrap());
    let nc = b / d;
    let p = b % d;

    let mut s = T::BITS as u32 - 1;
    while (p >= b / T::from(2)) && (s > 0) {
        s -= 1;
        p += p;
    }

    MagicNumbersForDivision::new(nc + T::from(1).unsigned_shl(s as u32) / d, s, false)
}

pub mod explicit_instantiations {
    use super::*;

    // Explicit instantiation declarations.
    pub type MagicNumbersForDivisionU32 = MagicNumbersForDivision<u32>;
    pub type MagicNumbersForDivisionU64 = MagicNumbersForDivision<u64>;

    pub fn signed_division_by_constant_u32(d: u32) -> MagicNumbersForDivisionU32 {
        signed_division_by_constant(d)
    }

    pub fn signed_division_by_constant_u64(d: u64) -> MagicNumbersForDivisionU64 {
        signed_division_by_constant(d)
    }

    pub fn unsigned_division_by_constant_u32(
        d: u32,
        leading_zeros: u32,
    ) -> MagicNumbersForDivisionU32 {
        unsigned_division_by_constant(d, leading_zeros)
    }

    pub fn unsigned_division_by_constant_u64(
        d: u64,
        leading_zeros: u32,
    ) -> MagicNumbersForDivisionU64 {
        unsigned_division_by_constant(d, leading_zeros)
    }
}