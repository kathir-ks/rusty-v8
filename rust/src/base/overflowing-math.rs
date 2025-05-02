pub mod base {
    use std::{
        f32,
        fmt::Debug,
        mem,
        ops::{Add, Mul, Neg, Shl, Sub},
        u32,
    };

    /// A trait to assert that a type is a signed integer.
    pub trait AssertSignedIntegerType: Debug {
        // This trait serves as a static assertion, as it is not possible to implement the methods automatically for all types.
        // Instead, the trait must be manually implemented.
        // This trait is sealed and cannot be implemented outside of this module.
    }

    macro_rules! impl_assert_signed_integer_type {
        ($($t:ty),*) => {
            $(
                impl AssertSignedIntegerType for $t {}
            )*
        };
    }

    impl_assert_signed_integer_type!(i8, i16, i32, i64, i128, isize);

    macro_rules! op_with_wraparound {
        ($name:ident, $op:tt) => {
            /// Performs the `$op` operation with wraparound.
            pub fn $name<T>(a: T, b: T) -> T
            where
                T: AssertSignedIntegerType
                    + Add<Output = T>
                    + Sub<Output = T>
                    + Mul<Output = T>
                    + Sized
                    + Copy,
            {
                let a_unsigned = a as <T as std::convert::TryInto<
                    <T as std::convert::TryInto<u128>>::Error,
                >>::Output;
                let b_unsigned = b as <T as std::convert::TryInto<
                    <T as std::convert::TryInto<u128>>::Error,
                >>::Output;

                a_unsigned $op b_unsigned as T
            }
        };
    }

    op_with_wraparound!(add_with_wraparound, +);
    op_with_wraparound!(sub_with_wraparound, -);
    op_with_wraparound!(mul_with_wraparound, *);

    /// Performs multiplication with wraparound for i16.
    pub fn mul_with_wraparound_i16(a: i16, b: i16) -> i16 {
        let a_unsigned = a as u32;
        let b_unsigned = b as u32;
        let result = a_unsigned * b_unsigned;
        (result as u16) as i16
    }

    /// Negates a value with wraparound.
    pub fn negate_with_wraparound<T>(a: T) -> T
    where
        T: AssertSignedIntegerType + Neg<Output = T> + Copy + std::cmp::PartialEq,
        std::ops::RangeInclusive<T>: std::iter::Iterator<Item = T>,
    {
        if a == std::ops::RangeInclusive::new(T::min_value(), T::min_value()).next().unwrap() {
            return a;
        }
        -a
    }

    /// Shifts left with wraparound.
    pub fn shl_with_wraparound<T>(a: T, b: T) -> T
    where
        T: AssertSignedIntegerType + Shl<Output = T> + Copy + std::convert::TryInto<u128>,
        <T as std::convert::TryInto<u128>>::Error: Debug,
    {
        let k_mask = (mem::size_of::<T>() * 8) as u32 - 1;
        (a as u128 << (b as u32 & k_mask)) as T
    }

    /// Divides two values, avoiding undefined behavior if y == 0.
    pub fn divide<T>(x: T, y: T) -> T
    where
        T: PartialEq + std::ops::Div<Output = T> + Copy,
        std::num::FpCategory: From<T>,
    {
        if y != T::default() {
            return x / y;
        }

        if x == T::default() {
           return T::NAN;
        }

        if std::cmp::PartialOrd::ge(&x, &T::default()) == (y.is_sign_positive()) {
            return T::INFINITY;
        }

        T::NEG_INFINITY
    }

    /// Calculates the reciprocal of a float.
    pub fn recip(a: f32) -> f32 {
        divide(1.0f32, a)
    }

    /// Calculates the reciprocal square root of a float.
    pub fn recip_sqrt(a: f32) -> f32 {
        if a != 0.0 {
            return 1.0f32 / a.sqrt();
        }
        if a.is_sign_positive() {
            return f32::INFINITY;
        }
        f32::NEG_INFINITY
    }
}