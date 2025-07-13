// Converted from V8 C++ source files:
// Header: overflowing-math.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    use std::{
        f32,
        fmt::Display,
        i16, i32, i64, i8,
        marker::PhantomData,
        mem::size_of,
        ops::{Add, Mul, Neg, Shl, Sub},
        result,
    };
    //use crate::base::export::Maybe;
    use std::limits::Limits;
    use std::ops::BitAnd;
    use std::ops::Div;
    use std::ops::Not;

    macro_rules! assert_signed_integer_type {
        ($Type:ty) => {
            static_assert!(
                std::is_signed::<$Type>::is_signed() && std::any::TypeId::of::<$Type>() != std::any::TypeId::of::<bool>(),
                "use this for signed integer types"
            );
        };
    }
    macro_rules! op_with_wraparound {
        ($Name:ident, $op:tt) => {
            pub fn $Name<signed_type>(a: signed_type, b: signed_type) -> signed_type
            where
                signed_type: Sized
                    + std::ops::Add<Output = signed_type>
                    + std::ops::Sub<Output = signed_type>
                    + std::ops::Mul<Output = signed_type>
                    + std::ops::BitAnd<Output = signed_type>
                    + std::marker::Copy,
                unsigned_type: From<signed_type>,
                //unsigned_type: std::ops::Add<Output = unsigned_type> + std::marker::Copy,
                unsigned_type: std::convert::From<signed_type>,
                signed_type: std::convert::From<unsigned_type>,
            {
                assert_signed_integer_type!(signed_type);
                type unsigned_type = <signed_type as std::convert::TryInto<unsigned_type>>::Error;
                let a_unsigned: unsigned_type = a.into();
                let b_unsigned: unsigned_type = b.into();
                let result: unsigned_type = a_unsigned.into() $op b_unsigned.into();
                result.into()
            }
        };
    }
    pub fn AddWithWraparound<T>(a: T, b: T) -> T
    where
        T: Add<Output = T> + From<i8> + Copy,
    {
        a + b
    }
    pub fn SubWithWraparound<T>(a: T, b: T) -> T
    where
        T: Sub<Output = T> + From<i8> + Copy,
    {
        a - b
    }
    pub fn MulWithWraparound<T>(a: T, b: T) -> T
    where
        T: Mul<Output = T> + From<i8> + Copy,
    {
        a * b
    }

    pub fn MulWithWraparound_i16(a: i16, b: i16) -> i16 {
        let a_unsigned: u32 = a as u32;
        let b_unsigned: u32 = b as u32;
        let result: u32 = a_unsigned * b_unsigned;
        (result as u16) as i16
    }
    pub fn NegateWithWraparound<signed_type>(a: signed_type) -> signed_type
    where
        signed_type: Neg<Output = signed_type> + std::cmp::PartialEq + Copy,
        std::primitive::i8: std::convert::TryInto<signed_type>,
    {
        assert_signed_integer_type!(signed_type);
        if a == std::i8::MIN.try_into().unwrap() {
            return a;
        }
        -a
    }

    pub fn ShlWithWraparound<signed_type>(a: signed_type, b: signed_type) -> signed_type
    where
        signed_type: Shl<Output = signed_type> + BitAnd<Output = signed_type> + From<u8> + Copy,
    {
        assert_signed_integer_type!(signed_type);
        type unsigned_type = <signed_type as std::convert::TryInto<u8>>::Error;
        let kMask: u8 = (size_of::<signed_type>() * 8 - 1) as u8;
        (a << (b & kMask.into()))
    }

    pub fn Divide<T>(x: T, y: T) -> T
    where
        T: Div<Output = T>
            + PartialEq
            + Copy
            + std::marker::Sized
            + num_traits::Zero
            + num_traits::Float
            + std::fmt::Debug,
    {
        if y != num_traits::Zero::zero() {
            return x / y;
        }

        if x == num_traits::Zero::zero() || x != x {
            return num_traits::Float::nan();
        }

        if (x >= num_traits::Zero::zero()) == (y.is_sign_positive()) {
            return num_traits::Float::infinity();
        }

        return num_traits::Float::neg_infinity();
    }
    pub fn Recip(a: f32) -> f32 {
        Divide(1.0f32, a)
    }

    pub fn RecipSqrt(a: f32) -> f32 {
        if a != 0.0 {
            return 1.0f32 / a.sqrt();
        }
        if a.is_sign_positive() {
            return f32::INFINITY;
        }
        return f32::NEG_INFINITY;
    }
    trait Limits {
        fn quiet_NaN() -> Self;
    }
}
