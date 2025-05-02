// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(unused)]

/// Simple wrapper around an entry (which is notably different from "index" for
/// dictionary backing stores). Most code should treat this as an opaque
/// wrapper: get it via GetEntryForIndex, pass it on to consumers.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct InternalIndex {
    entry_: usize,
}

impl InternalIndex {
    /// Creates a new `InternalIndex` from a raw `usize` value.
    pub const fn new(raw: usize) -> Self {
        InternalIndex { entry_: raw }
    }

    /// Returns a `InternalIndex` representing "not found".
    pub fn not_found() -> Self {
        InternalIndex { entry_: Self::K_NOT_FOUND }
    }

    /// Adjusts the `InternalIndex` down by subtracting a value.
    #[must_use]
    pub fn adjust_down(self, subtract: usize) -> Self {
        debug_assert!(self.entry_ >= subtract);
        InternalIndex {
            entry_: self.entry_ - subtract,
        }
    }

    /// Adjusts the `InternalIndex` up by adding a value.
    #[must_use]
    pub fn adjust_up(self, add: usize) -> Self {
        debug_assert!(self.entry_ <= usize::MAX - add);
        InternalIndex {
            entry_: self.entry_ + add,
        }
    }

    /// Checks if the `InternalIndex` is found (not equal to `kNotFound`).
    pub fn is_found(self) -> bool {
        self.entry_ != Self::K_NOT_FOUND
    }

    /// Checks if the `InternalIndex` is not found (equal to `kNotFound`).
    pub fn is_not_found(self) -> bool {
        self.entry_ == Self::K_NOT_FOUND
    }

    /// Returns the raw `usize` value of the `InternalIndex`.
    pub fn raw_value(self) -> usize {
        self.entry_
    }

    /// Returns the `InternalIndex` as a `u32`.  Panics if the value is too large.
    pub fn as_uint32(self) -> u32 {
        debug_assert!(self.entry_ <= u32::MAX as usize);
        self.entry_ as u32
    }

    /// Returns the `InternalIndex` as an `i32`. Panics if the value is too large.
    pub fn as_int(self) -> i32 {
        debug_assert!(self.entry_ <= i32::MAX as usize);
        self.entry_ as i32
    }

    const K_NOT_FOUND: usize = usize::MAX;
}

impl std::ops::Deref for InternalIndex {
    type Target = InternalIndex;

    fn deref(&self) -> &Self::Target {
        self
    }
}

impl std::ops::Add<usize> for InternalIndex {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        self.adjust_up(rhs)
    }
}

impl std::ops::Sub<usize> for InternalIndex {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        self.adjust_down(rhs)
    }
}

impl std::ops::Not for InternalIndex {
    type Output = bool;

    fn not(self) -> Self::Output {
        self.is_not_found()
    }
}

impl std::ops::AddAssign<usize> for InternalIndex {
    fn add_assign(&mut self, rhs: usize) {
        *self = self.adjust_up(rhs);
    }
}

impl std::ops::SubAssign<usize> for InternalIndex {
    fn sub_assign(&mut self, rhs: usize) {
        *self = self.adjust_down(rhs);
    }
}

impl std::iter::Iterator for InternalIndex {
    type Item = InternalIndex;

    fn next(&mut self) -> Option<Self::Item> {
        if self.entry_ != Self::K_NOT_FOUND {
            let current = *self;
            self.entry_ += 1;
            Some(current)
        } else {
            None
        }
    }
}

impl InternalIndex {
    /// Represents a range of `InternalIndex` values.
    #[derive(Debug, Copy, Clone)]
    pub struct Range {
        min_: usize,
        max_: usize,
    }

    impl Range {
        /// Creates a new `Range` with a min value of 0 and the given max.
        pub fn new(max: usize) -> Self {
            Range { min_: 0, max_ }
        }

        /// Creates a new `Range` with the given min and max values.
        pub fn with_min(min: usize, max: usize) -> Self {
            Range { min_, max_ }
        }

        /// Returns the starting `InternalIndex` of the range.
        pub fn begin(self) -> InternalIndex {
            InternalIndex::new(self.min_)
        }

        /// Returns the ending `InternalIndex` of the range.
        pub fn end(self) -> InternalIndex {
            InternalIndex::new(self.max_)
        }

        /// Returns an iterator for the range.
        pub fn iter(self) -> impl Iterator<Item = InternalIndex> {
            (self.min_..self.max_).map(InternalIndex::new)
        }
    }
}