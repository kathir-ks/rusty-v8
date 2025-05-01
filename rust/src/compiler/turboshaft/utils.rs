// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::{
    fmt,
    marker::PhantomData,
    mem,
    ops::Deref,
    ops::DerefMut,
    u64,
};

macro_rules! any_of {
    ($($arg:expr),*) => {
        AnyOf::new($($arg),*)
    };
}

macro_rules! all_of {
    ($($arg:expr),*) => {
        AllOf::new($($arg),*)
    };
}

pub struct AnyOf<T> {
    values: Vec<T>,
}

impl<T: PartialEq + fmt::Debug> AnyOf<T> {
    pub fn new(values: T) -> Self {
        AnyOf { values: vec![values] }
    }

    pub fn contains(&self, value: &T) -> bool {
        self.values.iter().any(|v| v == value)
    }

    pub fn print_to(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "any_of(")?;
        let mut first = true;
        for value in &self.values {
            if !first {
                write!(f, ", ")?;
            }
            first = false;
            write!(f, "{:?}", value)?;
        }
        write!(f, ")")
    }
}

impl<T: PartialEq + fmt::Debug> PartialEq<T> for AnyOf<T> {
    fn eq(&self, other: &T) -> bool {
        self.contains(other)
    }
}

impl<T: PartialEq + fmt::Debug> fmt::Display for AnyOf<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.print_to(f)
    }
}

pub struct AllOf<T> {
    values: Vec<T>,
}

impl<T: PartialEq + fmt::Debug> AllOf<T> {
    pub fn new(values: T) -> Self {
        AllOf { values: vec![values] }
    }

    pub fn all_equal_to(&self, value: &T) -> bool {
        self.values.iter().all(|v| v == value)
    }

    pub fn print_to(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "all_of(")?;
        let mut first = true;
        for value in &self.values {
            if !first {
                write!(f, ", ")?;
            }
            first = false;
            write!(f, "{:?}", value)?;
        }
        write!(f, ")")
    }
}

impl<T: PartialEq + fmt::Debug> PartialEq<AllOf<T>> for T {
    fn eq(&self, other: &AllOf<T>) -> bool {
        other.all_equal_to(self)
    }
}

impl<T: PartialEq + fmt::Debug> fmt::Display for AllOf<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.print_to(f)
    }
}

#[cfg(debug_assertions)]
extern "C" {
    fn ShouldSkipOptimizationStep() -> bool;
}

#[cfg(not(debug_assertions))]
#[inline(always)]
pub fn should_skip_optimization_step() -> bool {
    false
}

/// Set `*ptr` to `new_value` while the scope is active, reset to the previous
/// value upon destruction.
pub struct ScopedModification<'a, T> {
    ptr: &'a mut T,
    old_value: T,
}

impl<'a, T> ScopedModification<'a, T> {
    pub fn new(ptr: &'a mut T, new_value: T) -> Self {
        let old_value = mem::replace(ptr, new_value);
        ScopedModification { ptr, old_value }
    }

    pub fn old_value(&self) -> &T {
        &self.old_value
    }
}

impl<'a, T> Drop for ScopedModification<'a, T> {
    fn drop(&mut self) {
        let _ = mem::replace(self.ptr, self.old_value);
    }
}

// The `multi`-switch mechanism helps to switch on multiple values at the same
// time. Example:
//
//   match multi(change.from, change.to) {
//     multi(Word32(), Float32()) => ...
//     multi(Word32(), Float64()) => ...
//     multi(Word64(), Float32()) => ...
//     multi(Word64(), Float64()) => ...
//     ...
//   }
//
// This works for an arbitrary number of dimensions and arbitrary types as long
// as they can be encoded into an integral value and their combination fits into
// a uint64_t. For types to be used, they need to provide a specialization of
// MultiSwitch<T> with this signature:
//
//   impl MultiSwitch for T {
//     const MAX_VALUE: u64 = ...
//     fn encode(value: T) -> u64 { ... }
//   }
//
// For `max_value` choose a value that is larger than all encoded values. Choose
// this as small as possible to make jump tables more dense. If a type's value
// count is somewhat close to a multiple of two, consider using this, as this
// might lead to slightly faster encoding. The encoding follows this formula:
//
//   multi(v1, v2, v3) =
//     let t1 = MultiSwitch<T3>::encode(v3) in
//     let t2 = (t1 * MultiSwitch<T2>::max_value)
//              + MultiSwitch<T2>::encode(v2) in
//     (t2 * MultiSwitch<T1>::max_value) + MultiSwitch<T1>::encode(v1)
//
// For integral types (like enums), use
//
//   DEFINE_MULTI_SWITCH_INTEGRAL(MyType, MaxValue)

pub trait MultiSwitch {
    const MAX_VALUE: u64;
    fn encode(self) -> u64;
}

macro_rules! define_multi_switch_integral {
    ($name:ty, $max_value:expr) => {
        impl MultiSwitch for $name {
            const MAX_VALUE: u64 = $max_value;
            fn encode(self) -> u64 {
                self as u64
            }
        }
    };
}

define_multi_switch_integral!(bool, 2);

mod detail {
    use super::MultiSwitch;

    #[inline]
    pub const fn multi_encode<T: MultiSwitch>(value: T) -> u64 {
        value.encode()
    }

    #[inline]
    pub const fn multi_encode_recursive<Head: MultiSwitch, Next: MultiSwitch, Rest: MultiSwitch>
    (head: Head, next: Next, rest: Rest) -> u64
    {
        let v = multi_encode_recursive(next, rest);
        let max_value = Head::MAX_VALUE;
        let head_encode = head.encode();

        // Ensure the maximum value for u64 isn't reached, panic if so
        // const assertions aren't possible with math operations so we resort to panicking
        if v > (u64::MAX / max_value) {
            panic!("Max value is too large");
        }

        (v * max_value) + head_encode
    }
}

pub const fn multi<T: MultiSwitch>(value: T) -> u64 {
    detail::multi_encode(value)
}

pub const fn multi_recursive<Head: MultiSwitch, Next: MultiSwitch, Rest: MultiSwitch>
(head: Head, next: Next, rest: Rest) -> u64 {
    detail::multi_encode_recursive(head, next, rest)
}