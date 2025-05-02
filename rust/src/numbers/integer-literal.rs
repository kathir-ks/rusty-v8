// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod integer_literal {
    use std::{
        cmp::Ordering,
        fmt,
        fmt::Display,
        marker::PhantomData,
        num::TryFromIntError,
        ops::{BitOr, Shl, Add},
        str::FromStr,
    };

    /// Represents an integer literal.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct IntegerLiteral {
        negative: bool,
        absolute_value: u64,
    }

    impl IntegerLiteral {
        /// Creates a new `IntegerLiteral`.
        pub fn new(negative: bool, absolute_value: u64) -> Self {
            let mut result = Self {
                negative,
                absolute_value,
            };
            if absolute_value == 0 {
                result.negative = false;
            }
            result
        }

        /// Creates a new `IntegerLiteral` from a value.
        pub fn from<T>(value: T) -> Self
        where
            T: Into<i64>,
        {
            let value_i64: i64 = value.into();
            if value_i64 >= 0 {
                IntegerLiteral::new(false, value_i64 as u64)
            } else {
                let absolute_value = value_i64.abs() as u64;
                IntegerLiteral::new(true, absolute_value)
            }
        }

        /// Returns whether the `IntegerLiteral` is negative.
        pub fn is_negative(&self) -> bool {
            self.negative
        }

        /// Returns the absolute value of the `IntegerLiteral`.
        pub fn absolute_value(&self) -> u64 {
            self.absolute_value
        }

        /// Checks if the `IntegerLiteral` is representable as the given type.
        pub fn is_representable_as<T>(&self) -> bool
        where
            T: std::fmt::Display + std::str::FromStr + std::cmp::PartialOrd,
            <T as FromStr>::Err: std::fmt::Debug,
            T: std::convert::TryFrom<i64>,
        {
            use std::i64;
            let min_value = std::i64::MIN;
            let max_value = std::i64::MAX;

            let min_lit = IntegerLiteral::from(min_value);
            let max_lit = IntegerLiteral::from(max_value);

            self.compare(&min_lit) >= Ordering::Less && self.compare(&max_lit) <= Ordering::Greater
        }

        /// Converts the `IntegerLiteral` to the given type.
        pub fn to<T>(&self) -> T
        where
            T: std::convert::TryFrom<i64>,
            <T as TryFrom<i64>>::Error: std::fmt::Debug,
            T: std::fmt::Display + std::str::FromStr + std::cmp::PartialOrd,
            <T as FromStr>::Err: std::fmt::Debug,
        {
            assert!(self.is_representable_as::<T>());
            let v = if self.negative {
                -(self.absolute_value as i64)
            } else {
                self.absolute_value as i64
            };
            v.try_into().unwrap()
        }

        /// Attempts to convert the `IntegerLiteral` to the given type.
        pub fn try_to<T>(&self) -> Option<T>
        where
            T: std::convert::TryFrom<i64>,
            <T as TryFrom<i64>>::Error: std::fmt::Debug,
             T: std::fmt::Display + std::str::FromStr + std::cmp::PartialOrd,
            <T as FromStr>::Err: std::fmt::Debug,
        {
            if !self.is_representable_as::<T>() {
                return None;
            }
            Some(self.to::<T>())
        }

        /// Compares the `IntegerLiteral` to another `IntegerLiteral`.
        pub fn compare(&self, other: &IntegerLiteral) -> Ordering {
            if self.absolute_value == other.absolute_value {
                if self.absolute_value == 0 || self.negative == other.negative {
                    Ordering::Equal
                } else if self.negative {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            } else if self.absolute_value < other.absolute_value {
                if other.negative {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            } else if self.negative {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }

        /// Converts the `IntegerLiteral` to a string.
        pub fn to_string(&self) -> String {
            if self.negative {
                format!("-{}", self.absolute_value)
            } else {
                self.absolute_value.to_string()
            }
        }
    }

    impl Display for IntegerLiteral {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.to_string())
        }
    }

    impl BitOr for IntegerLiteral {
        type Output = Self;

        fn bitor(self, other: Self) -> Self {
            assert!(!self.is_negative());
            assert!(!other.is_negative());
            IntegerLiteral::new(false, self.absolute_value | other.absolute_value)
        }
    }
    //Implementations for operators are missing due to the dependencies on other V8 modules

}