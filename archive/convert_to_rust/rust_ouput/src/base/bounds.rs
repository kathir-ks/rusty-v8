// Converted from V8 C++ source files:
// Header: bounds.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
  use std::{
    mem::size_of,
    ops::{Sub, Add},
  };

  // Checks if value is in range [lower_limit, higher_limit] using a single
  // branch.
  pub fn is_in_range<T, U>(value: T, lower_limit: U, higher_limit: U) -> bool
  where
    T: PartialOrd + Sub<U, Output = T> + Copy + Into<u128>,
    U: PartialOrd + Copy + Into<u128>,
  {
    assert!(lower_limit <= higher_limit);
    let value_unsigned: u128 = value.into();
    let lower_limit_unsigned: u128 = lower_limit.into();
    let higher_limit_unsigned: u128 = higher_limit.into();

    (value_unsigned - lower_limit_unsigned) <= (higher_limit_unsigned - lower_limit_unsigned)
  }

  // Like IsInRange but for the half-open range [lower_limit, higher_limit).
  pub fn is_in_half_open_range<T, U>(value: T, lower_limit: U, higher_limit: U) -> bool
  where
    T: PartialOrd + Sub<U, Output = T> + Copy + Into<u128>,
    U: PartialOrd + Copy + Into<u128>,
  {
    assert!(lower_limit <= higher_limit);
    let value_unsigned: u128 = value.into();
    let lower_limit_unsigned: u128 = lower_limit.into();
    let higher_limit_unsigned: u128 = higher_limit.into();

    (value_unsigned - lower_limit_unsigned) < (higher_limit_unsigned - lower_limit_unsigned)
  }

  // Checks if [index, index+length) is in range [0, max). Note that this check
  // works even if {index+length} would wrap around.
  pub fn is_in_bounds<T>(index: T, length: T, max: T) -> bool
  where
    T: PartialOrd + Sub<Output = T> + Copy + Add<Output = T>,
  {
    length <= max && index <= (max - length)
  }

  // Checks if [index, index+length) is in range [0, max). If not, {length} is
  // clamped to its valid range. Note that this check works even if
  // {index+length} would wrap around.
  pub fn clamp_to_bounds<T>(index: T, length: &mut T, max: T) -> bool
  where
    T: PartialOrd + Sub<Output = T> + Copy,
  {
    if index > max {
      *length = T::from(0);
      return false;
    }
    let avail = max - index;
    let oob = *length > avail;
    if oob {
      *length = avail;
    }
    return !oob;
  }
}
