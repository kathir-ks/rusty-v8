// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod base {
    use std::{
        fmt,
        marker::Copy,
        ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Sub, SubAssign},
    };

    /// A bit set of enums `E` (without explicit values), fitting into an integral type `T`.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct EnumSet<E, T = i32>
    where
        E: Into<T> + Copy,
        T: Copy + Default + BitAnd<Output = T> + BitOr<Output = T> + Not<Output = T> + From<i32>,
    {
        bits_: T,
        _phantom: std::marker::PhantomData<E>,
    }

    impl<E, T> EnumSet<E, T>
    where
        E: Into<T> + Copy,
        T: Copy
            + Default
            + BitAnd<Output = T>
            + BitOr<Output = T>
            + Not<Output = T>
            + From<i32>
            + BitAndAssign
            + BitOrAssign
            + Sub<Output = T>
            + SubAssign,
    {
        /// Creates an empty `EnumSet`.
        pub const fn new() -> Self {
            EnumSet {
                bits_: T::default(),
                _phantom: std::marker::PhantomData,
            }
        }

        /// Creates an `EnumSet` from an initializer list.
        pub fn from_initializer_list(init: &[E]) -> Self {
            let mut bits = T::default();
            for &e in init {
                bits = bits | Self::mask(e);
            }
            EnumSet {
                bits_: bits,
                _phantom: std::marker::PhantomData,
            }
        }

        /// Returns `true` if the set is empty.
        pub const fn is_empty(&self) -> bool {
            self.bits_ == T::default()
        }

        /// Returns `true` if the set contains the given element.
        pub const fn contains(&self, element: E) -> bool {
            (self.bits_ & Self::mask(element)) != T::default()
        }

        /// Returns `true` if the set contains all elements of the given set.
        pub const fn contains_all(&self, set: Self) -> bool {
            (self.bits_ & set.bits_) == set.bits_
        }

        /// Returns `true` if the set contains any elements of the given set.
        pub const fn contains_any(&self, set: Self) -> bool {
            (self.bits_ & set.bits_) != T::default()
        }

        /// Returns `true` if the set contains only the given element.
        pub const fn contains_only(&self, element: E) -> bool
        where
            T: PartialEq,
        {
            self.bits_ == Self::mask(element)
        }

        /// Returns `true` if the set is a subset of the given set.
        pub const fn is_subset_of(&self, set: Self) -> bool {
            (self.bits_ & set.bits_) == self.bits_
        }

        /// Adds the given element to the set.
        pub fn add(&mut self, element: E) {
            self.bits_ = self.bits_ | Self::mask(element);
        }

        /// Adds all elements from the given set to this set.
        pub fn add_set(&mut self, set: Self) {
            self.bits_ = self.bits_ | set.bits_;
        }

        /// Removes the given element from the set.
        pub fn remove(&mut self, element: E) {
            self.bits_ = self.bits_ & !Self::mask(element);
        }

        /// Removes all elements from the given set from this set.
        pub fn remove_set(&mut self, set: Self) {
            self.bits_ = self.bits_ & !set.bits_;
        }

        /// Removes all elements from the set.
        pub fn remove_all(&mut self) {
            self.bits_ = T::default();
        }

        /// Intersects this set with the given set.
        pub fn intersect(&mut self, set: Self) {
            self.bits_ = self.bits_ & set.bits_;
        }

        /// Converts the set to its integral representation.
        pub const fn to_integral(&self) -> T {
            self.bits_
        }

        fn mask(element: E) -> T
        where
            T: From<i32>,
        {
            let element_val: T = element.into();
            assert!(
                std::mem::size_of::<T>() * 8 > element_val.into(),
                "Enum value exceeds storage capacity"
            );
            T::from(1) << element.into()
        }

        /// Creates a new EnumSet directly from an integral type.
        pub fn from_integral(bits: T) -> Self {
            EnumSet {
                bits_: bits,
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl<E, T> Not for EnumSet<E, T>
    where
        E: Into<T> + Copy,
        T: Copy
            + Default
            + BitAnd<Output = T>
            + BitOr<Output = T>
            + Not<Output = T>
            + From<i32>
            + BitAndAssign
            + BitOrAssign
            + BitXorAssign
            + BitXor<Output = T>
            + Sub<Output = T>
            + SubAssign,
    {
        type Output = Self;

        fn not(self) -> Self::Output {
            EnumSet {
                bits_: !self.bits_,
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl<E, T> BitOr for EnumSet<E, T>
    where
        E: Into<T> + Copy,
        T: Copy
            + Default
            + BitAnd<Output = T>
            + BitOr<Output = T>
            + Not<Output = T>
            + From<i32>
            + BitAndAssign
            + BitOrAssign
            + Sub<Output = T>
            + SubAssign,
    {
        type Output = Self;

        fn bitor(self, rhs: Self) -> Self::Output {
            EnumSet {
                bits_: self.bits_ | rhs.bits_,
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl<E, T> BitAnd for EnumSet<E, T>
    where
        E: Into<T> + Copy,
        T: Copy
            + Default
            + BitAnd<Output = T>
            + BitOr<Output = T>
            + Not<Output = T>
            + From<i32>
            + BitAndAssign
            + BitOrAssign
            + Sub<Output = T>
            + SubAssign,
    {
        type Output = Self;

        fn bitand(self, rhs: Self) -> Self::Output {
            EnumSet {
                bits_: self.bits_ & rhs.bits_,
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl<E, T> Sub for EnumSet<E, T>
    where
        E: Into<T> + Copy,
        T: Copy
            + Default
            + BitAnd<Output = T>
            + BitOr<Output = T>
            + Not<Output = T>
            + From<i32>
            + BitAndAssign
            + BitOrAssign
            + Sub<Output = T>
            + SubAssign,
    {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            EnumSet {
                bits_: self.bits_ & !rhs.bits_,
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl<E, T> BitOrAssign for EnumSet<E, T>
    where
        E: Into<T> + Copy,
        T: Copy
            + Default
            + BitAnd<Output = T>
            + BitOr<Output = T>
            + Not<Output = T>
            + From<i32>
            + BitAndAssign
            + BitOrAssign
            + Sub<Output = T>
            + SubAssign,
    {
        fn bitor_assign(&mut self, rhs: Self) {
            *self = *self | rhs;
        }
    }

    impl<E, T> BitAndAssign for EnumSet<E, T>
    where
        E: Into<T> + Copy,
        T: Copy
            + Default
            + BitAnd<Output = T>
            + BitOr<Output = T>
            + Not<Output = T>
            + From<i32>
            + BitAndAssign
            + BitOrAssign
            + Sub<Output = T>
            + SubAssign,
    {
        fn bitand_assign(&mut self, rhs: Self) {
            *self = *self & rhs;
        }
    }

    impl<E, T> SubAssign for EnumSet<E, T>
    where
        E: Into<T> + Copy,
        T: Copy
            + Default
            + BitAnd<Output = T>
            + BitOr<Output = T>
            + Not<Output = T>
            + From<i32>
            + BitAndAssign
            + BitOrAssign
            + Sub<Output = T>
            + SubAssign,
    {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs;
        }
    }

    impl<E, T> BitOr<E> for EnumSet<E, T>
    where
        E: Into<T> + Copy,
        T: Copy
            + Default
            + BitAnd<Output = T>
            + BitOr<Output = T>
            + Not<Output = T>
            + From<i32>
            + BitAndAssign
            + BitOrAssign
            + Sub<Output = T>
            + SubAssign,
    {
        type Output = Self;

        fn bitor(self, element: E) -> Self::Output {
            EnumSet {
                bits_: self.bits_ | Self::mask(element),
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl<E, T> BitAnd<E> for EnumSet<E, T>
    where
        E: Into<T> + Copy,
        T: Copy
            + Default
            + BitAnd<Output = T>
            + BitOr<Output = T>
            + Not<Output = T>
            + From<i32>
            + BitAndAssign
            + BitOrAssign
            + Sub<Output = T>
            + SubAssign,
    {
        type Output = Self;

        fn bitand(self, element: E) -> Self::Output {
            EnumSet {
                bits_: self.bits_ & Self::mask(element),
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl<E, T> Sub<E> for EnumSet<E, T>
    where
        E: Into<T> + Copy,
        T: Copy
            + Default
            + BitAnd<Output = T>
            + BitOr<Output = T>
            + Not<Output = T>
            + From<i32>
            + BitAndAssign
            + BitOrAssign
            + Sub<Output = T>
            + SubAssign,
    {
        type Output = Self;

        fn sub(self, element: E) -> Self::Output {
            EnumSet {
                bits_: self.bits_ & !Self::mask(element),
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl<E, T> BitOrAssign<E> for EnumSet<E, T>
    where
        E: Into<T> + Copy,
        T: Copy
            + Default
            + BitAnd<Output = T>
            + BitOr<Output = T>
            + Not<Output = T>
            + From<i32>
            + BitAndAssign
            + BitOrAssign
            + Sub<Output = T>
            + SubAssign,
    {
        fn bitor_assign(&mut self, element: E) {
            *self = *self | element;
        }
    }

    impl<E, T> BitAndAssign<E> for EnumSet<E, T>
    where
        E: Into<T> + Copy,
        T: Copy
            + Default
            + BitAnd<Output = T>
            + BitOr<Output = T>
            + Not<Output = T>
            + From<i32>
            + BitAndAssign
            + BitOrAssign
            + Sub<Output = T>
            + SubAssign,
    {
        fn bitand_assign(&mut self, element: E) {
            *self = *self & element;
        }
    }

    impl<E, T> SubAssign<E> for EnumSet<E, T>
    where
        E: Into<T> + Copy,
        T: Copy
            + Default
            + BitAnd<Output = T>
            + BitOr<Output = T>
            + Not<Output = T>
            + From<i32>
            + BitAndAssign
            + BitOrAssign
            + Sub<Output = T>
            + SubAssign,
    {
        fn sub_assign(&mut self, element: E) {
            *self = *self - element;
        }
    }

    impl<E, T> fmt::Debug for EnumSet<E, T>
    where
        E: Into<T> + Copy + fmt::Debug,
        T: Copy
            + Default
            + BitAnd<Output = T>
            + BitOr<Output = T>
            + Not<Output = T>
            + From<i32>
            + BitAndAssign
            + BitOrAssign
            + Sub<Output = T>
            + SubAssign,
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut set = *self;
            let mut elements: Vec<E> = Vec::new();
            while !set.is_empty() {
                let bits = set.to_integral();
                //This is a placeholder as Rust stdlib does not have an equivalent to CountTrailingZerosNonZero for all integer types
                //Replace this with a proper implementation of CountTrailingZerosNonZero for T
                let element_index = trailing_zeros_nonzero(bits.into());
                let element: E = unsafe { std::mem::transmute(element_index as i32) };
                elements.push(element);
                set.remove(element);
            }
            write!(f, "{{")?;
            for (i, element) in elements.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:?}", element)?;
            }
            write!(f, "}}")
        }
    }

    fn trailing_zeros_nonzero(bits: i32) -> u32 {
        bits.trailing_zeros()
    }

    //Implement the trait ToString for EnumSet<E, T>
    impl<E, T> fmt::Display for EnumSet<E, T>
    where
        E: Into<T> + Copy + fmt::Debug,
        T: Copy
            + Default
            + BitAnd<Output = T>
            + BitOr<Output = T>
            + Not<Output = T>
            + From<i32>
            + BitAndAssign
            + BitOrAssign
            + Sub<Output = T>
            + SubAssign,
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut set = *self;
            let mut elements: Vec<E> = Vec::new();
            while !set.is_empty() {
                let bits = set.to_integral();
                //This is a placeholder as Rust stdlib does not have an equivalent to CountTrailingZerosNonZero for all integer types
                //Replace this with a proper implementation of CountTrailingZerosNonZero for T
                let element_index = trailing_zeros_nonzero(bits.into());
                let element: E = unsafe { std::mem::transmute(element_index as i32) };
                elements.push(element);
                set.remove(element);
            }
            write!(f, "{{")?;
            for (i, element) in elements.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:?}", element)?;
            }
            write!(f, "}}")
        }
    }
}