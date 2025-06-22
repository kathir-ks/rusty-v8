// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod base {
    /// Checks if value is in range [lower_limit, higher_limit] using a single
    /// branch.
    #[inline]
    pub const fn is_in_range<T, U>(value: T, lower_limit: U, higher_limit: U) -> bool
    where
        T: Copy,
        U: Copy + PartialOrd,
        T: std::convert::TryFrom<U>,
        <T as std::convert::TryFrom<U>>::Error: std::fmt::Debug,
    {
        if lower_limit > higher_limit {
            panic!("lower_limit must be less than or equal to higher_limit");
        }

        let value_unsigned = value.try_into().unwrap();
        let lower_limit_unsigned = lower_limit.try_into().unwrap();
        let higher_limit_unsigned = higher_limit.try_into().unwrap();

        value_unsigned >= lower_limit_unsigned && value_unsigned <= higher_limit_unsigned
    }

    /// Like IsInRange but for the half-open range [lower_limit, higher_limit).
    #[inline]
    pub const fn is_in_half_open_range<T, U>(value: T, lower_limit: U, higher_limit: U) -> bool
    where
        T: Copy,
        U: Copy + PartialOrd,
        T: std::convert::TryFrom<U>,
        <T as std::convert::TryFrom<U>>::Error: std::fmt::Debug,
    {
        if lower_limit > higher_limit {
            panic!("lower_limit must be less than or equal to higher_limit");
        }

        let value_unsigned = value.try_into().unwrap();
        let lower_limit_unsigned = lower_limit.try_into().unwrap();
        let higher_limit_unsigned = higher_limit.try_into().unwrap();

        value_unsigned >= lower_limit_unsigned && value_unsigned < higher_limit_unsigned
    }

    /// Checks if [index, index+length) is in range [0, max). Note that this check
    /// works even if {index+length} would wrap around.
    #[inline]
    pub const fn is_in_bounds<T>(index: T, length: T, max: T) -> bool
    where
        T: std::ops::Sub<Output = T> + PartialOrd + Copy,
    {
        length <= max && index <= (max - length)
    }

    /// Checks if [index, index+length) is in range [0, max). If not, {length} is
    /// clamped to its valid range. Note that this check works even if
    /// {index+length} would wrap around.
    pub fn clamp_to_bounds<T>(index: T, length: &mut T, max: T) -> bool
    where
        T: PartialOrd + Copy + std::ops::Sub<Output = T>,
    {
        if index > max {
            *length = T::from(0u8);
            return false;
        }
        let avail = max - index;
        let oob = *length > avail;
        if oob {
            *length = avail;
        }
        !oob
    }
}