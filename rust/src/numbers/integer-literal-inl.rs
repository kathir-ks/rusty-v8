// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod integer_literal {
    use std::fmt;
    use std::ops::{Add, Shl};

    const BITS_PER_BYTE: usize = 8;

    /// Represents an integer literal.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct IntegerLiteral {
        negative: bool,
        absolute_value: u64,
    }

    impl IntegerLiteral {
        /// Creates a new `IntegerLiteral`.
        pub fn new(negative: bool, absolute_value: u64) -> Self {
            Self {
                negative,
                absolute_value,
            }
        }

        /// Returns whether the literal is negative.
        pub fn is_negative(&self) -> bool {
            self.negative
        }

        /// Returns the absolute value of the literal.
        pub fn absolute_value(&self) -> u64 {
            self.absolute_value
        }

        /// Returns a string representation of the literal.
        pub fn to_string(&self) -> String {
            if self.negative {
                format!("-{}", self.absolute_value)
            } else {
                self.absolute_value.to_string()
            }
        }
    }

    impl fmt::Display for IntegerLiteral {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.to_string())
        }
    }

    impl Shl<IntegerLiteral> for IntegerLiteral {
        type Output = Self;

        /// Implements the left shift operator.
        ///
        /// # Panics
        ///
        /// Panics if the right-hand side is negative or greater than or equal to
        /// the number of bits in a `u64`.
        fn shl(self, other: IntegerLiteral) -> Self {
            assert!(!other.is_negative());
            assert!(other.absolute_value < (std::mem::size_of::<u64>() * BITS_PER_BYTE) as u64);
            IntegerLiteral::new(self.negative, self.absolute_value << other.absolute_value)
        }
    }

    impl Add<IntegerLiteral> for IntegerLiteral {
        type Output = Self;

        /// Implements the addition operator.
        fn add(self, other: IntegerLiteral) -> Self {
            if self.negative == other.negative {
                //debug_assert!(self.absolute_value().wrapping_add(other.absolute_value()) >= self.absolute_value());
                IntegerLiteral::new(self.negative, self.absolute_value.wrapping_add(other.absolute_value))
            } else {
                if self.absolute_value >= other.absolute_value {
                    IntegerLiteral::new(self.negative, self.absolute_value - other.absolute_value)
                } else {
                    IntegerLiteral::new(!self.negative, other.absolute_value - self.absolute_value)
                }
            }
        }
    }
}