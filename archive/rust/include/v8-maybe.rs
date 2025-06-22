// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::marker::PhantomData;
use std::mem::MaybeUninit;

// Placeholder for cppgc::internal::ConditionalStackAllocatedBase
// In a real conversion, this would be handled by a corresponding Rust struct/trait
// #[derive(Default)]
// struct ConditionalStackAllocatedBase {}
//
// trait ConditionalStackAllocated {}
// impl<T> ConditionalStackAllocated for T {}
//
// struct Maybe<T> {
//   base: ConditionalStackAllocatedBase,
//   value: Option<T>,
// }

// Placeholder for V8_EXPORT
// In a real conversion, this would be handled by conditional compilation or FFI
extern "C" {
    fn FromJustIsNothing(); // Placeholder function
}

/// A simple Maybe type, representing an object which may or may not have a
/// value, see https://hackage.haskell.org/package/base/docs/Data-Maybe.html.
///
/// If an API method returns a Maybe<>, the API method can potentially fail
/// either because an exception is thrown, or because an exception is pending,
/// e.g. because a previous API call threw an exception that hasn't been caught
/// yet, or because a TerminateExecution exception was thrown. In that case, a
/// "Nothing" value is returned.
#[derive(Debug, Clone, PartialEq)]
pub enum Maybe<T> {
    Nothing,
    Just(T),
}

impl<T> Maybe<T> {
    /// Returns `true` if the maybe is a `Nothing` value.
    #[inline]
    pub fn is_nothing(&self) -> bool {
        match self {
            Maybe::Nothing => true,
            Maybe::Just(_) => false,
        }
    }

    /// Returns `true` if the maybe is a `Just` value.
    #[inline]
    pub fn is_just(&self) -> bool {
        !self.is_nothing()
    }

    /// An alias for |FromJust|. Will panic if the Maybe<> is nothing.
    #[inline]
    pub fn to_checked(&self) -> &T {
        self.from_just()
    }

    /// Short-hand for ToChecked(), which doesn't return a value. To be used, where
    /// the actual value of the Maybe is not needed like Object::Set.
    #[inline]
    pub fn check(&self) {
        if self.is_nothing() {
            unsafe { crate::FromJustIsNothing() };
        }
    }

    /// Converts this Maybe<> to a value of type T. If this Maybe<> is
    /// nothing (empty), |false| is returned and |out| is left untouched.
    #[inline]
    pub fn to(&self, out: &mut MaybeUninit<T>) -> bool {
        match self {
            Maybe::Just(value) => {
                unsafe { out.as_mut_ptr().write(value.clone()); } // Use clone to avoid move
                true
            }
            Maybe::Nothing => false,
        }
    }

    /// Converts this Maybe<> to a value of type T. If this Maybe<> is
    /// nothing (empty), V8 will panic the process.
    #[inline]
    pub fn from_just(&self) -> &T {
        match self {
            Maybe::Just(value) => value,
            Maybe::Nothing => {
                unsafe { crate::FromJustIsNothing() };
                panic!("FromJust called on Nothing");
            }
        }
    }

    /// Converts this Maybe<> to a value of type T. If this Maybe<> is
    /// nothing (empty), V8 will panic the process.
    #[inline]
    pub fn from_just_owned(self) -> T {
        match self {
            Maybe::Just(value) => value,
            Maybe::Nothing => {
                unsafe { crate::FromJustIsNothing() };
                panic!("FromJust called on Nothing");
            }
        }
    }

    /// Converts this Maybe<> to a value of type T, using a default value if this
    /// Maybe<> is nothing (empty).
    #[inline]
    pub fn from_maybe(&self, default_value: &T) -> T
    where
        T: Clone,
    {
        match self {
            Maybe::Just(value) => value.clone(),
            Maybe::Nothing => default_value.clone(),
        }
    }
}

/// Creates a `Nothing` value of type `Maybe<T>`.
#[inline]
pub fn nothing<T>() -> Maybe<T> {
    Maybe::Nothing
}

/// Creates a `Just` value of type `Maybe<T>` containing the given value.
#[inline]
pub fn just<T>(t: T) -> Maybe<T> {
    Maybe::Just(t)
}

// A template specialization of Maybe<T> for the case of T = void.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MaybeVoid {
    Nothing,
    Just,
}

impl MaybeVoid {
    /// Returns `true` if the maybe is a `Nothing` value.
    #[inline]
    pub fn is_nothing(&self) -> bool {
        match self {
            MaybeVoid::Nothing => true,
            MaybeVoid::Just => false,
        }
    }

    /// Returns `true` if the maybe is a `Just` value.
    #[inline]
    pub fn is_just(&self) -> bool {
        !self.is_nothing()
    }
}

/// Creates a `Nothing` value of type `MaybeVoid`.
#[inline]
pub fn nothing_void() -> MaybeVoid {
    MaybeVoid::Nothing
}

/// Creates a `Just` value of type `MaybeVoid`.
#[inline]
pub fn just_void() -> MaybeVoid {
    MaybeVoid::Just
}