// Copyright 2014 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Slightly adapted for inclusion in V8.
// Copyright 2014 the V8 project authors. All rights reserved.
// List of adaptations:
// - include guard names
// - wrap in v8 namespace
// - formatting (git cl format)

pub mod v8_base_safe_conversions_impl {
    use std::{
        cmp::{max, min},
        fmt,
        marker::PhantomData,
        mem,
        num::Wrapping,
        ops::{Add, BitAnd, BitXor, Not, Sub},
    };
    use std::{
        i16, i32, i64, i8, isize, u16, u32, u64, u8, usize,
    };

    pub mod internal {

        use std::{
            cmp::{max, min},
            fmt,
            marker::PhantomData,
            mem,
            num::Wrapping,
            ops::{Add, BitAnd, BitXor, Not, Sub},
        };
        use std::{
            i16, i32, i64, i8, isize, u16, u32, u64, u8, usize,
        };

        /// The std library doesn't provide a binary max_exponent for integers, however
        /// we can compute an analog using std::numeric_limits<>::digits.
        pub struct MaxExponent<T> {
            _phantom: PhantomData<T>,
        }

        impl<T: num_traits::Float> MaxExponent<T> {
            pub const VALUE: i32 = T::max_exponent();
        }

        impl<T: num_traits::PrimInt> MaxExponent<T> {
            pub const VALUE: i32 = mem::size_of::<T>() as i32 * 8;
        }

        /// The number of bits (including the sign) in an integer. Eliminates sizeof
        /// hacks.
        pub struct IntegerBitsPlusSign<T> {
            _phantom: PhantomData<T>,
        }

        impl<T: num_traits::PrimInt> IntegerBitsPlusSign<T> {
            pub const VALUE: usize = mem::size_of::<T>() * 8;
        }

        /// Helper templates for integer manipulations.
        pub struct PositionOfSignBit<Integer> {
            _phantom: PhantomData<Integer>,
        }

        impl<Integer: num_traits::PrimInt> PositionOfSignBit<Integer> {
            pub const VALUE: usize = IntegerBitsPlusSign::<Integer>::VALUE - 1;
        }

        /// Determines if a numeric value is negative without throwing compiler
        /// warnings on: unsigned(value) < 0.
        pub trait IsNegative {
            fn is_negative(self) -> bool;
        }

        impl<T: num_traits::PrimInt + std::cmp::PartialOrd> IsNegative for T {
            default fn is_negative(self) -> bool {
                self < T::zero()
            }
        }

        impl<T: num_traits::PrimInt + std::cmp::PartialOrd> IsNegative for Wrapping<T> {
            default fn is_negative(self) -> bool {
                self < Wrapping(T::zero())
            }
        }

        impl<T: num_traits::PrimInt + std::cmp::PartialOrd> IsNegative for &T {
            default fn is_negative(self) -> bool {
                *self < T::zero()
            }
        }

        impl<T: num_traits::PrimInt> IsNegative for Wrapping<&T> {
            default fn is_negative(self) -> bool {
                *self < Wrapping(T::zero())
            }
        }

        impl<T: num_traits::PrimInt + std::cmp::PartialOrd> IsNegative for &Wrapping<T> {
            default fn is_negative(self) -> bool {
                *self < Wrapping(T::zero())
            }
        }

        impl<T: num_traits::Unsigned> IsNegative for T {
            fn is_negative(self) -> bool {
                false
            }
        }

        impl<T: num_traits::Unsigned> IsNegative for Wrapping<T> {
            fn is_negative(self) -> bool {
                false
            }
        }

        impl<T: num_traits::Unsigned> IsNegative for &T {
            fn is_negative(self) -> bool {
                false
            }
        }

        impl<T: num_traits::Unsigned> IsNegative for Wrapping<&T> {
            fn is_negative(self) -> bool {
                false
            }
        }

        impl<T: num_traits::Unsigned> IsNegative for &Wrapping<T> {
            fn is_negative(self) -> bool {
                false
            }
        }

        /// This performs a fast negation, returning a signed value. It works on unsigned
        /// arguments, but probably doesn't do what you want for any unsigned value
        /// larger than max / 2 + 1 (i.e. signed min cast to unsigned).
        pub fn conditional_negate<T: num_traits::PrimInt>(x: T, is_negative: bool) -> T {
            if is_negative {
                x.wrapping_neg()
            } else {
                x
            }
        }

        /// This performs a safe, absolute value via unsigned overflow.
        pub fn safe_unsigned_abs<T: num_traits::PrimInt>(value: T) -> T {
            if value.is_negative() {
                value.wrapping_neg()
            } else {
                value
            }
        }

        // TODO(jschuh): Switch to std::is_constant_evaluated() once C++20 is supported.
        // Alternately, the usage could be restructured for "consteval if" in C++23.
        macro_rules! is_constant_evaluated {
            () => {
                cfg!(debug_assertions) == false
            };
        }

        // TODO(jschuh): Debug builds don't reliably propagate constants, so we restrict
        // some accelerated runtime paths to release builds until this can be forced
        // with consteval support in C++20 or C++23.
        #[cfg(debug_assertions)]
        const ENABLE_ASM_CODE: bool = false;
        #[cfg(not(debug_assertions))]
        const ENABLE_ASM_CODE: bool = true;

        /// Forces a crash, like a CHECK(false). Used for numeric boundary errors.
        /// Also used in a constexpr template to trigger a compilation failure on
        /// an error condition.
        pub struct CheckOnFailure {}

        impl CheckOnFailure {
            pub fn handle_failure<T>() -> T {
                panic!("CheckOnFailure::HandleFailure");
            }
        }

        #[derive(Debug, PartialEq, Eq)]
        pub enum IntegerRepresentation {
            INTEGER_REPRESENTATION_UNSIGNED,
            INTEGER_REPRESENTATION_SIGNED,
        }

        /// A range for a given nunmeric Src type is contained for a given numeric Dst
        /// type if both numeric_limits<Src>::max() <= numeric_limits<Dst>::max() and
        /// numeric_limits<Src>::lowest() >= numeric_limits<Dst>::lowest() are true.
        /// We implement this as template specializations rather than simple static
        /// comparisons to ensure type correctness in our comparisons.
        #[derive(Debug, PartialEq, Eq)]
        pub enum NumericRangeRepresentation {
            NUMERIC_RANGE_NOT_CONTAINED,
            NUMERIC_RANGE_CONTAINED,
        }

        /// Helper templates to statically determine if our destination type can contain
        /// maximum and minimum values represented by the source type.
        pub struct StaticDstRangeRelationToSrcRange<Dst, Src> {
            _phantom: PhantomData<(Dst, Src)>,
        }

        impl<Dst, Src> StaticDstRangeRelationToSrcRange<Dst, Src> {
            pub const VALUE: NumericRangeRepresentation = {
                if MaxExponent::<Dst>::VALUE >= MaxExponent::<Src>::VALUE {
                    NumericRangeRepresentation::NUMERIC_RANGE_CONTAINED
                } else {
                    NumericRangeRepresentation::NUMERIC_RANGE_NOT_CONTAINED
                }
            };
        }

        // impl<Dst, Src> StaticDstRangeRelationToSrcRange<Dst, Src>
        // where Dst: num_traits::Signed, Src: num_traits::Unsigned
        // {
        //   pub const VALUE: NumericRangeRepresentation = {
        //     if MaxExponent::<Dst>::VALUE > MaxExponent::<Src>::VALUE {
        //       NumericRangeRepresentation::NUMERIC_RANGE_CONTAINED
        //     } else {
        //       NumericRangeRepresentation::NUMERIC_RANGE_NOT_CONTAINED
        //     }
        //   };
        // }

        // impl<Dst, Src> StaticDstRangeRelationToSrcRange<Dst, Src>
        // where Dst: num_traits::Unsigned, Src: num_traits::Signed
        // {
        //   pub const VALUE: NumericRangeRepresentation =
        //     NumericRangeRepresentation::NUMERIC_RANGE_NOT_CONTAINED;
        // }

        /// This class wraps the range constraints as separate booleans so the compiler
        /// can identify constants and eliminate unused code paths.
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct RangeCheck {
            is_underflow_: bool,
            is_overflow_: bool,
        }

        impl RangeCheck {
            pub const fn new(is_in_lower_bound: bool, is_in_upper_bound: bool) -> Self {
                RangeCheck {
                    is_underflow_: !is_in_lower_bound,
                    is_overflow_: !is_in_upper_bound,
                }
            }
            pub const fn default() -> Self {
                RangeCheck {
                    is_underflow_: false,
                    is_overflow_: false,
                }
            }
            pub const fn is_valid(&self) -> bool {
                !self.is_overflow_ && !self.is_underflow_
            }
            pub const fn is_invalid(&self) -> bool {
                self.is_overflow_ && self.is_underflow_
            }
            pub const fn is_overflow(&self) -> bool {
                self.is_overflow_ && !self.is_underflow_
            }
            pub const fn is_underflow(&self) -> bool {
                !self.is_overflow_ && self.is_underflow_
            }
            pub const fn is_overflow_flag_set(&self) -> bool {
                self.is_overflow_
            }
            pub const fn is_underflow_flag_set(&self) -> bool {
                self.is_underflow_
            }
        }

        /// The following helper template addresses a corner case in range checks for
        /// conversion from a floating-point type to an integral type of smaller range
        /// but larger precision (e.g. float -> unsigned). The problem is as follows:
        ///   1. Integral maximum is always one less than a power of two, so it must be
        ///      truncated to fit the mantissa of the floating point. The direction of
        ///      rounding is implementation defined, but by default it's always IEEE
        ///      floats, which round to nearest and thus result in a value of larger
        ///      magnitude than the integral value.
        ///      Example: float f = UINT_MAX; // f is 4294967296f but UINT_MAX
        ///                                   // is 4294967295u.
        ///   2. If the floating point value is equal to the promoted integral maximum
        ///      value, a range check will erroneously pass.
        ///      Example: (4294967296f <= 4294967295u) // This is true due to a precision
        ///                                            // loss in rounding up to float.
        ///   3. When the floating point value is then converted to an integral, the
        ///      resulting value is out of range for the target integral type and
        ///      thus is implementation defined.
        ///      Example: unsigned u = (float)INT_MAX; // u will typically overflow to 0.
        /// To fix this bug we manually truncate the maximum value when the destination
        /// type is an integral of larger precision than the source floating-point type,
        /// such that the resulting maximum is represented exactly as a floating point.
        pub struct NarrowingRange<Dst, Src, Bounds> {
            _phantom: PhantomData<(Dst, Src, Bounds)>,
        }

        impl<Dst, Src, Bounds> NarrowingRange<Dst, Src, Bounds> {
            pub const K_SHIFT: i32 = {
                if MaxExponent::<Src>::VALUE > MaxExponent::<Dst>::VALUE &&
                    std::mem::size_of::<Src>() * 8 < std::mem::size_of::<Dst>() * 8
                {
                    (std::mem::size_of::<Dst>() * 8 - std::mem::size_of::<Src>() * 8) as i32
                } else {
                    0
                }
            };

            pub const fn adjust(value: Dst) -> Dst
            where
                Dst: num_traits::PrimInt,
                Bounds: BoundsTrait<Dst>,
            {
                if Self::K_SHIFT < (std::mem::size_of::<Dst>() * 8) as i32 {
                    let mask: Dst = !(Dst::one().wrapping_shl(Self::K_SHIFT as u32) - Dst::one());
                    conditional_negate(value.bitand(mask), value.is_negative())
                } else {
                    value
                }
            }

            pub const fn adjust_float(value: Dst) -> Dst
            where
                Dst: num_traits::Float,
                Bounds: BoundsTrait<Dst>,
            {
                assert_eq!(Self::K_SHIFT, 0);
                value
            }

            pub const fn max<B>() -> Dst
            where
                Dst: num_traits::PrimInt,
                B: BoundsTrait<Dst>,
            {
                Self::adjust(B::max())
            }

            pub const fn lowest<B>() -> Dst
            where
                Dst: num_traits::PrimInt,
                B: BoundsTrait<Dst>,
            {
                Self::adjust(B::lowest())
            }
        }

        pub trait BoundsTrait<T> {
            fn max() -> T;
            fn lowest() -> T;
        }

        impl<T: num_traits::PrimInt> BoundsTrait<T> for std::num::Wrapping<T> {
            fn max() -> T {
                std::num::Wrapping::<T>::max_value().0
            }

            fn lowest() -> T {
                std::num::Wrapping::<T>::min_value().0
            }
        }

        impl<T: num_traits::Float> BoundsTrait<T> for std::num::Wrapping<T> {
            fn max() -> T {
                T::max_value()
            }

            fn lowest() -> T {
                T::min_value()
            }
        }

        pub struct DstRangeRelationToSrcRangeImpl<Dst, Src, Bounds> {
            _phantom: PhantomData<(Dst, Src, Bounds)>,
        }

        impl<Dst, Src, Bounds> DstRangeRelationToSrcRangeImpl<Dst, Src, Bounds> {
            pub const fn check(value: Src) -> RangeCheck
            where
                Dst: num_traits::PrimInt,
                Src: num_traits::PrimInt,
                Bounds: BoundsTrait<Dst>,
            {
                let dst_lowest = NarrowingRange::<Dst, Src, Bounds>::lowest::<Bounds>();
                let dst_max = NarrowingRange::<Dst, Src, Bounds>::max::<Bounds>();
                let src_lowest = Src::min_value();
                let src_max = Src::max_value();

                let is_in_lower_bound = (src_lowest >= dst_lowest) || (value >= dst_lowest);
                let is_in_upper_bound = (src_max <= dst_max) || (value <= dst_max);

                RangeCheck::new(is_in_lower_bound, is_in_upper_bound)
            }
        }

        // Simple wrapper for statically checking if a type's range is contained.
        pub struct IsTypeInRangeForNumericType<Dst, Src> {
            _phantom: PhantomData<(Dst, Src)>,
        }

        impl<Dst, Src> IsTypeInRangeForNumericType<Dst, Src> {
            pub const VALUE: bool =
                StaticDstRangeRelationToSrcRange::<Dst, Src>::VALUE == NumericRangeRepresentation::NUMERIC_RANGE_CONTAINED;
        }

        pub const fn dst_range_relation_to_src_range<Dst, Src, Bounds>(value: Src) -> RangeCheck
        where
            Dst: num_traits::PrimInt,
            Src: num_traits::PrimInt,
            Bounds: BoundsTrait<Dst>,
        {
            DstRangeRelationToSrcRangeImpl::<Dst, Src, Bounds>::check(value)
        }

        // Integer promotion templates used by the portable checked integer arithmetic.
        pub struct IntegerForDigitsAndSign<const Size: usize, const IsSigned: bool>;

        macro_rules! integer_for_digits_and_sign {
            ($I:ty) => {
                impl IntegerForDigitsAndSign<{ IntegerBitsPlusSign::<$I>::VALUE }, { std::any::TypeId::of::<i32>() != std::any::TypeId::of::<$I>() }>
                    for IntegerForDigitsAndSign<{ IntegerBitsPlusSign::<$I>::VALUE }, { std::any::TypeId::of::<i32>() != std::any::TypeId::of::<$I>() }> {
                    type type_ = $I;
                }
            };
        }

        integer_for_digits_and_sign!(i8);
        integer_for_digits_and_sign!(u8);
        integer_for_digits_and_sign!(i16);
        integer_for_digits_and_sign!(u16);
        integer_for_digits_and_sign!(i32);
        integer_for_digits_and_sign!(u32);
        integer_for_digits_and_sign!(i64);
        integer_for_digits_and_sign!(u64);

        /// WARNING: We have no IntegerForSizeAndSign<16, *>. If we ever add one to
        /// support 128-bit math, then the ArithmeticPromotion template below will need
        /// to be updated (or more likely replaced with a decltype expression).
        // static_assert(IntegerBitsPlusSign<intmax_t>::value == 64,
        //               "Max integer size not supported for this toolchain.");

        pub struct TwiceWiderInteger<Integer, const IsSigned: bool> {
            _phantom: PhantomData<Integer>,
        }

        impl<Integer, const IsSigned: bool> TwiceWiderInteger<Integer, IsSigned> {
            type type_ = Integer;
        }

        #[derive(Debug, PartialEq, Eq)]
        pub enum ArithmeticPromotionCategory {
            LEFT_PROMOTION,  // Use the type of the left-hand argument.
            RIGHT_PROMOTION, // Use the type of the right-hand argument.
        }

        /// Determines the type that can represent the largest positive value.
        pub struct MaxExponentPromotion<Lhs, Rhs> {
            _phantom: PhantomData<(Lhs, Rhs)>,
        }

        impl<Lhs, Rhs> MaxExponentPromotion<Lhs, Rhs> {
            type type_ = Lhs;
        }

        /// Determines the type that can represent the lowest arithmetic value.
        pub struct LowestValuePromotion<Lhs, Rhs> {
            _phantom: PhantomData<(Lhs, Rhs)>,
        }

        impl<Lhs, Rhs> LowestValuePromotion<Lhs, Rhs> {
            type type_ = Lhs;
        }

        /// Determines the type that is best able to represent an arithmetic result.
        pub struct BigEnoughPromotion<Lhs, Rhs> {
            _phantom: PhantomData<(Lhs, Rhs)>,
        }

        impl<Lhs, Rhs> BigEnoughPromotion<Lhs, Rhs> {
            type type_ = Lhs;
            const is_contained: bool = true;
        }

        /// We can statically check if operations on the provided types can wrap, so we
        /// can skip the checked operations if they're not needed. So, for an integer we
        /// care if the destination type preserves the sign and is twice the width of
        /// the source.
        pub struct IsIntegerArithmeticSafe<T, Lhs, Rhs> {
            _phantom: PhantomData<(T, Lhs, Rhs)>,
        }

        impl<T, Lhs, Rhs> IsIntegerArithmeticSafe<T, Lhs, Rhs> {
            const value: bool = false;
        }

        /// Promotes to a type that can represent any possible result of a binary
        /// arithmetic operation with the source types.
        pub struct FastIntegerArithmeticPromotion<Lhs, Rhs> {
            _phantom: PhantomData<(Lhs, Rhs)>,
        }

        impl<Lhs, Rhs> FastIntegerArithmeticPromotion<Lhs, Rhs> {
            type type_ = Lhs;
            const is_contained: bool = false;
        }

        /// Extracts the underlying type from an enum.
        pub struct ArithmeticOrUnderlyingEnum<T> {
            _phantom: PhantomData<T>,
        }

        impl<T> ArithmeticOrUnderlyingEnum<T> {
            type type_ = T;
            const value: bool = false;
        }

        // /// The following are helper templates used in the CheckedNumeric class.
        pub struct CheckedNumeric<T> {
            _phantom: PhantomData<T>,
        }

        // /// The following are helper templates used in the ClampedNumeric class.
        pub struct ClampedNumeric<T> {
            _phantom: PhantomData<T>,
        }

        // /// The following are helper templates used in the StrictNumeric class.
        pub struct StrictNumeric<T> {
            _phantom: PhantomData<T>,
        }

        /// Used to treat CheckedNumeric and arithmetic underlying types the same.
        pub struct UnderlyingType<T> {
            _phantom: PhantomData<T>,
        }

        impl<T> UnderlyingType<T> {
            type type_ = T;
            const is_numeric: bool = std::any::TypeId::of::<i32>() == std::any::TypeId::of::<T>();
            const is_checked: bool = false;
            const is_clamped: bool = false;
            const is_strict: bool = false;
        }

        pub struct IsCheckedOp<L, R> {
            _phantom: PhantomData<(L, R)>,
        }

        impl<L, R> IsCheckedOp<L, R> {
            const value: bool = false;
        }

        pub struct IsClampedOp<L, R> {
            _phantom: PhantomData<(L, R)>,
        }

        impl<L, R> IsClampedOp<L, R> {
            const value: bool = false;
        }

        pub struct IsStrictOp<L, R> {
            _phantom: PhantomData<(L, R)>,
        }

        impl<L, R> IsStrictOp<L, R> {
            const value: bool = false;
        }

        /// as_signed<> returns the supplied integral value (or integral castable
        /// Numeric template) cast as a signed integral of equivalent precision.
        /// I.e. it's mostly an alias for: static_cast<std::make_signed<T>::type>(t)
        pub fn as_signed<Src: num_traits::PrimInt>(value: Src) -> Src {
            value
        }

        /// as_unsigned<> returns the supplied integral value (or integral castable
        /// Numeric template) cast as an unsigned integral of equivalent precision.
        /// I.e. it's mostly an alias for: static_cast<std::make_unsigned<T>::type>(t)
        pub fn as_unsigned<Src: num_traits::PrimInt>(value: Src) -> Src {
            value
        }

        pub fn is_less_impl<L: num_traits::PrimInt, R: num_traits::PrimInt>(
            lhs: L,
            rhs: R,
            l_range: RangeCheck,
            r_range: RangeCheck,
        ) -> bool {
            l_range.is_underflow()
                || r_range.is_overflow()
                || (l_range == r_range && (lhs.wrapping_add(&rhs)) < (rhs.wrapping_add(&lhs)))
        }

        pub struct IsLess<L, R> {
            _phantom: PhantomData<(L, R)>,
        }

        impl<L: num_traits::PrimInt, R: num_traits::PrimInt> IsLess<L, R> {
            pub fn test(lhs: L, rhs: R) -> bool {
                is_less_impl(
                    lhs,
                    rhs,
                    dst_range_relation_to_src_range::<R, L, std::num::Wrapping<L>>(lhs),
                    dst_range_relation_to_src_range::<L, R, std::num::Wrapping<R>>(rhs),
                )
            }
        }

        pub fn is_less_or_equal_impl<L: num_traits::PrimInt, R: num_traits::PrimInt>(
            lhs: L,
            rhs: R,
            l_range: RangeCheck,
            r_range: RangeCheck,
        ) -> bool {
            l_range.is_underflow()
                || r_range.is_overflow()
                || (l_range == r_range && (lhs.wrapping_add(&rhs)) <= (rhs.wrapping_add(&lhs)))
        }

        pub struct IsLessOrEqual<L, R> {
            _phantom: PhantomData<(L, R)>,
        }

        impl<L: num_traits::PrimInt, R: num_traits::PrimInt> IsLessOrEqual<L, R> {
            pub fn test(lhs: L, rhs: R) -> bool {
                is_less_or_equal_impl(
                    lhs,
                    rhs,
                    dst_range_relation_to_src_range::<R, L, std::num::Wrapping<L>>(lhs),
                    dst_range_relation_to_src_range::<L, R, std::num::Wrapping<R>>(rhs),
                )
            }
        }

        pub fn is_greater_impl<L: num_traits::PrimInt, R: num_traits::PrimInt>(
            lhs: L,
            rhs: R,
            l_range: RangeCheck,
            r_range: RangeCheck,
        ) -> bool {
            l_range.is_overflow()
                || r_range.is_underflow()
                || (l_range == r_range && (lhs.wrapping_add(&rhs)) > (rhs.wrapping_add(&lhs)))
        }

        pub struct IsGreater<L, R> {
            _phantom: PhantomData<(L, R)>,
        }

        impl<L: num_traits::PrimInt, R: num_traits::PrimInt> IsGreater<L, R> {
            pub fn test(lhs: L, rhs: R) -> bool {
                is_greater_impl(
                    lhs,
                    rhs,
                    dst_range_relation_to_src_range::<R, L, std::num::Wrapping<L>>(lhs),
                    dst_range_relation_to_src_range::<L, R, std::num::Wrapping<R>>(rhs),
                )
            }
        }

        pub fn is_greater_or_equal_impl<L: num_traits::PrimInt, R: num_traits::PrimInt>(
            lhs: L,
            rhs: R,
            l_range: RangeCheck,
            r_range: RangeCheck,
        ) -> bool {
            l_range.is_overflow()
                || r_range.is_underflow()
                || (l_range == r_range && (lhs.wrapping_add(&rhs)) >= (rhs.wrapping_add(&lhs)))
        }

        pub struct IsGreaterOrEqual<L, R> {
            _phantom: PhantomData<(L, R)>,
        }

        impl<L: num_traits::PrimInt, R: num_traits::PrimInt> IsGreaterOrEqual<L, R> {
            pub fn test(lhs: L, rhs: R) -> bool {
                is_greater_or_equal_impl(
                    lhs,
                    rhs,
                    dst_range_relation_to_src_range::<R, L, std::num::Wrapping<L>>(lhs),
                    dst_range_relation_to_src_range::<L, R, std::num::Wrapping<R>>(rhs),
                )
            }
        }

        pub struct IsEqual<L, R> {
            _phantom: PhantomData<(L, R)>,
        }

        impl<L: num_traits::PrimInt, R: num_traits::PrimInt> IsEqual<L, R> {
            pub fn test(lhs: L, rhs: R) -> bool {
                dst_range_relation_to_src_range::<R, L, std::num::Wrapping<L>>(lhs)
                    == dst_range_relation_to_src_range::<L, R, std::num::Wrapping<R>>(rhs)
                    && (lhs.wrapping_add(&rhs)) == (rhs.wrapping_add(&lhs))
            }
        }

        pub struct IsNotEqual<L, R> {
            _phantom: PhantomData<(L, R)>,
        }

        impl<L: num_traits::PrimInt, R: num_traits::PrimInt> IsNotEqual<L, R> {
            pub fn test(lhs: L, rhs: R) -> bool {
                dst_range_relation_to_src_range::<R, L, std::num::Wrapping<L>>(lhs)
                    != dst_range_relation_to_src_range::<L, R, std::num::Wrapping<R>>(rhs)
                    || (lhs.wrapping_add(&rhs)) != (rhs.wrapping_add(&lhs))
            }
        }

        /// These perform the actual math operations on the CheckedNumerics.
        /// Binary arithmetic operations.
        pub fn safe_compare<C, L, R>(lhs: L, rhs: R) -> bool
        where
            L: num_traits::PrimInt,
            R: num_traits::PrimInt,
        {
            let _promotion: BigEnoughPromotion<L, R> = BigEnoughPromotion {
                _phantom: PhantomData,
            };
            // using Promotion = BigEnoughPromotion<L, R>;
            // using BigType = typename Promotion::type;
            // Promotion::is_contained

            // // Force to a larger type for speed if both are contained.
            // ? C<BigType, BigType>::Test(
            //     static_cast<BigType>(static_cast<L>(lhs)),
            //     static_cast<BigType>(static_cast<R>(rhs)))
            // // Let the template functions figure it out for mixed types.
            // : C<L, R>::Test(lhs, rhs);
            C::test(lhs, rhs)
        }

        pub fn is_max_in_range_for_numeric_type<Dst, Src>() -> bool
        where
            Dst: num_traits::PrimInt,
            Src: num_traits::PrimInt,
        {
            IsGreaterOrEqual::<Dst, Src>::test(Dst::max_value(), Src::max_value())
        }

        pub fn is_min_in_range_for_numeric_type<Dst, Src>() -> bool
        where
            Dst: num_traits::PrimInt,
            Src: num_traits::PrimInt,
        {
            IsLessOrEqual::<Dst, Src>::test(Dst::min_value(), Src::min_value())
        }

        pub fn common_max<Dst, Src>() -> Dst
        where
            Dst: num_traits::PrimInt,
            Src: num_traits::PrimInt,
        {
            if !is_max_in_range_for_numeric_type::<Dst, Src>() {
                Dst::max_value()
            } else {
                Src::max_value()
            }
        }

        pub fn common_min<Dst, Src>() -> Dst
        where
            Dst: num_traits::PrimInt,
            Src: num_traits::PrimInt,
        {
            if !is_min_in_range_for_numeric_type::<Dst, Src>() {
                Dst::min_value()
            } else {
                Src::min_value()
            }
        }

        /// This is a wrapper to generate return the max or min for a supplied type.
        /// If the argument is false, the returned value is the maximum. If true the
        /// returned value is the minimum.
        pub fn common_max_or_min<Dst, Src>(is_min: bool) -> Dst
        where
            Dst: num_traits::PrimInt,
            Src: num_traits::PrimInt,
        {
            if is_min {
                common_min::<Dst, Src>()
            } else {
                common_max::<Dst, Src>()
            }
        }
    } // namespace v8::base::internal
} // mod