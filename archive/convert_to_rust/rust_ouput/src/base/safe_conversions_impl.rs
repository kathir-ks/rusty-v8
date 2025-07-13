// Converted from V8 C++ source files:
// Header: safe_conversions_impl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod internal {
    use std::{
        isize,
        i16, i32, i64, i8,
        marker::Copy,
        u16, u32, u64, u8,
    };

    pub struct MaxExponent<NumericType> {
        _phantom: std::marker::PhantomData<NumericType>,
    }

    impl<NumericType> MaxExponent<NumericType> {
        pub const value: i32 = if std::any::TypeId::of::<NumericType>() == std::any::TypeId::of::<f32>()
            || std::any::TypeId::of::<NumericType>() == std::any::TypeId::of::<f64>()
        {
            std::f32::MAX_EXP as i32
        } else {
            (std::mem::size_of::<NumericType>() * 8) as i32
        };
    }

    pub struct IntegerBitsPlusSign<NumericType> {
        _phantom: std::marker::PhantomData<NumericType>,
    }

    impl<NumericType> IntegerBitsPlusSign<NumericType> {
        pub const value: i32 = (std::mem::size_of::<NumericType>() * 8) as i32;
    }

    pub struct PositionOfSignBit<Integer> {
        _phantom: std::marker::PhantomData<Integer>,
    }

    impl<Integer> PositionOfSignBit<Integer> {
        pub const value: usize = IntegerBitsPlusSign::<Integer>::value as usize - 1;
    }

    pub const fn is_value_negative<T>(value: T) -> bool
    where
        T: std::cmp::PartialOrd + std::ops::Neg<Output = T> + Copy,
    {
        value < -num_traits::zero()
    }

    pub const fn conditional_negate<T>(x: T, is_negative: bool) -> T
    where
        T: std::ops::BitXor<Output = T>
            + std::ops::Add<Output = T>
            + num_traits::Zero
            + num_traits::One
            + std::convert::TryFrom<bool>,
        T: Copy,
    {
        if is_negative {
            x ^ (!T::zero()) + T::one()
        } else {
            x
        }
    }

    pub const fn safe_unsigned_abs<T>(value: T) -> T
    where
        T: std::ops::Neg<Output = T> + Copy,
    {
        if is_value_negative(value) {
            -value
        } else {
            value
        }
    }

    pub fn is_constant_evaluated() -> bool {
        true
    }

    pub const k_enable_asm_code: bool = true;

    pub struct CheckOnFailure {}

    impl CheckOnFailure {
        pub fn handle_failure<T>() -> T
        where
            T: Default,
        {
            std::process::abort();
        }
    }

    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum IntegerRepresentation {
        INTEGER_REPRESENTATION_UNSIGNED,
        INTEGER_REPRESENTATION_SIGNED,
    }

    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum NumericRangeRepresentation {
        NUMERIC_RANGE_NOT_CONTAINED,
        NUMERIC_RANGE_CONTAINED,
    }

    pub struct StaticDstRangeRelationToSrcRange<Dst, Src> {
        _phantom: std::marker::PhantomData<(Dst, Src)>,
    }

    impl<Dst, Src> StaticDstRangeRelationToSrcRange<Dst, Src> {
        pub const value: NumericRangeRepresentation =
            if MaxExponent::<Dst>::value >= MaxExponent::<Src>::value {
                NUMERIC_RANGE_CONTAINED
            } else {
                NUMERIC_RANGE_NOT_CONTAINED
            };
    }

    #[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
    pub struct RangeCheck {
        is_underflow_: bool,
        is_overflow_: bool,
    }

    impl RangeCheck {
        pub const fn new(is_in_lower_bound: bool, is_in_upper_bound: bool) -> Self {
            Self {
                is_underflow_: !is_in_lower_bound,
                is_overflow_: !is_in_upper_bound,
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

    pub struct NarrowingRange<Dst, Src, Bounds> {
        _phantom: std::marker::PhantomData<(Dst, Src, Bounds)>,
    }

    impl<Dst, Src, Bounds> NarrowingRange<Dst, Src, Bounds> {
        pub const k_shift: i32 = 0;

        pub fn adjust<T>(_value: T) -> T {
            T::default()
        }

        pub fn max() -> i32 {
            0
        }

        pub fn lowest() -> i32 {
            0
        }
    }

    pub struct DstRangeRelationToSrcRangeImpl<Dst, Src, Bounds> {
        _phantom: std::marker::PhantomData<(Dst, Src, Bounds)>,
    }

    impl<Dst, Src, Bounds> DstRangeRelationToSrcRangeImpl<Dst, Src, Bounds> {
        pub const fn check(_value: Src) -> RangeCheck {
            RangeCheck::default()
        }
    }

    pub struct IsTypeInRangeForNumericType<Dst, Src> {
        _phantom: std::marker::PhantomData<(Dst, Src)>,
    }

    impl<Dst, Src> IsTypeInRangeForNumericType<Dst, Src> {
        pub const value: bool =
            StaticDstRangeRelationToSrcRange::<Dst, Src>::value == NUMERIC_RANGE_CONTAINED;
    }

    pub const fn dst_range_relation_to_src_range<Dst, Src, Bounds>(_value: Src) -> RangeCheck {
        RangeCheck::default()
    }

    pub struct IntegerForDigitsAndSign<const Size: usize, const IsSigned: bool>;

    macro_rules! integer_for_digits_and_sign {
        ($I:ty) => {
            impl IntegerForDigitsAndSign<{ IntegerBitsPlusSign::<$I>::value as usize }, { <$I>::MIN > 0 }> {
                pub type type_ = $I;
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

    pub struct TwiceWiderInteger<Integer, const IsSigned: bool> {
        _phantom: std::marker::PhantomData<Integer>,
    }

    impl<Integer, const IsSigned: bool> TwiceWiderInteger<Integer, IsSigned> {
        pub type type_ = i64;
    }

    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum ArithmeticPromotionCategory {
        LEFT_PROMOTION,
        RIGHT_PROMOTION,
    }

    pub struct MaxExponentPromotion<Lhs, Rhs> {
        _phantom: std::marker::PhantomData<(Lhs, Rhs)>,
    }

    impl<Lhs, Rhs> MaxExponentPromotion<Lhs, Rhs> {
        pub type type_ = Lhs;
    }

    pub struct LowestValuePromotion<Lhs, Rhs> {
        _phantom: std::marker::PhantomData<(Lhs, Rhs)>,
    }

    impl<Lhs, Rhs> LowestValuePromotion<Lhs, Rhs> {
        pub type type_ = Lhs;
    }

    pub struct BigEnoughPromotion<Lhs, Rhs> {
        _phantom: std::marker::PhantomData<(Lhs, Rhs)>,
    }

    impl<Lhs, Rhs> BigEnoughPromotion<Lhs, Rhs> {
        pub type type_ = Lhs;
        pub const is_contained: bool = false;
    }

    pub struct IsIntegerArithmeticSafe<T, Lhs, Rhs> {
        _phantom: std::marker::PhantomData<(T, Lhs, Rhs)>,
    }

    impl<T, Lhs, Rhs> IsIntegerArithmeticSafe<T, Lhs, Rhs> {
        pub const value: bool = false;
    }

    pub struct FastIntegerArithmeticPromotion<Lhs, Rhs> {
        _phantom: std::marker::PhantomData<(Lhs, Rhs)>,
    }

    impl<Lhs, Rhs> FastIntegerArithmeticPromotion<Lhs, Rhs> {
        pub type type_ = Lhs;
        pub const is_contained: bool = false;
    }

    pub struct ArithmeticOrUnderlyingEnum<T> {
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> ArithmeticOrUnderlyingEnum<T> {
        pub type type_ = T;
        pub const value: bool = false;
    }

    pub struct UnderlyingType<T> {
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> UnderlyingType<T> {
        pub type type_ = T;
        pub const is_numeric: bool = false;
        pub const is_checked: bool = false;
        pub const is_clamped: bool = false;
        pub const is_strict: bool = false;
    }

    pub struct IsCheckedOp<L, R> {
        _phantom: std::marker::PhantomData<(L, R)>,
    }

    impl<L, R> IsCheckedOp<L, R> {
        pub const value: bool = false;
    }

    pub struct IsClampedOp<L, R> {
        _phantom: std::marker::PhantomData<(L, R)>,
    }

    impl<L, R> IsClampedOp<L, R> {
        pub const value: bool = false;
    }

    pub struct IsStrictOp<L, R> {
        _phantom: std::marker::PhantomData<(L, R)>,
    }

    impl<L, R> IsStrictOp<L, R> {
        pub const value: bool = false;
    }

    pub const fn as_signed<Src>(value: Src) -> Src
    where Src: Copy {
        value
    }

    pub const fn as_unsigned<Src>(value: Src) -> Src
    where Src: Copy {
        value
    }

    pub const fn is_less_impl<L, R>(lhs: L, rhs: R, l_range: RangeCheck, r_range: RangeCheck) -> bool
    where
        L: PartialOrd + Copy,
        R: PartialOrd + Copy,
    {
        l_range.is_underflow() || r_range.is_overflow() || (l_range == r_range && lhs < rhs)
    }

    pub struct IsLess<L, R> {
        _phantom: std::marker::PhantomData<(L, R)>,
    }

    impl<L, R> IsLess<L, R> {
        pub const fn test(lhs: L, rhs: R) -> bool
        where
            L: PartialOrd + Copy,
            R: PartialOrd + Copy,
        {
            is_less_impl(
                lhs,
                rhs,
                RangeCheck::default(),
                RangeCheck::default()
            )
        }
    }

    pub const fn is_less_or_equal_impl<L, R>(
        lhs: L,
        rhs: R,
        l_range: RangeCheck,
        r_range: RangeCheck,
    ) -> bool
    where
        L: PartialOrd + Copy,
        R: PartialOrd + Copy,
    {
        l_range.is_underflow() || r_range.is_overflow() || (l_range == r_range && lhs <= rhs)
    }

    pub struct IsLessOrEqual<L, R> {
        _phantom: std::marker::PhantomData<(L, R)>,
    }

    impl<L, R> IsLessOrEqual<L, R> {
        pub const fn test(lhs: L, rhs: R) -> bool
        where
            L: PartialOrd + Copy,
            R: PartialOrd + Copy,
        {
            is_less_or_equal_impl(
                lhs,
                rhs,
                RangeCheck::default(),
                RangeCheck::default()
            )
        }
    }

    pub const fn is_greater_impl<L, R>(
        lhs: L,
        rhs: R,
        l_range: RangeCheck,
        r_range: RangeCheck,
    ) -> bool
    where
        L: PartialOrd + Copy,
        R: PartialOrd + Copy,
    {
        l_range.is_overflow() || r_range.is_underflow() || (l_range == r_range && lhs > rhs)
    }

    pub struct IsGreater<L, R> {
        _phantom: std::marker::PhantomData<(L, R)>,
    }

    impl<L, R> IsGreater<L, R> {
        pub const fn test(lhs: L, rhs: R) -> bool
        where
            L: PartialOrd + Copy,
            R: PartialOrd + Copy,
        {
            is_greater_impl(
                lhs,
                rhs,
                RangeCheck::default(),
                RangeCheck::default()
            )
        }
    }

    pub const fn is_greater_or_equal_impl<L, R>(
        lhs: L,
        rhs: R,
        l_range: RangeCheck,
        r_range: RangeCheck,
    ) -> bool
    where
        L: PartialOrd + Copy,
        R: PartialOrd + Copy,
    {
        l_range.is_overflow() || r_range.is_underflow() || (l_range == r_range && lhs >= rhs)
    }

    pub struct IsGreaterOrEqual<L, R> {
        _phantom: std::marker::PhantomData<(L, R)>,
    }

    impl<L, R> IsGreaterOrEqual<L, R> {
        pub const fn test(lhs: L, rhs: R) -> bool
        where
            L: PartialOrd + Copy,
            R: PartialOrd + Copy,
        {
            is_greater_or_equal_impl(
                lhs,
                rhs,
                RangeCheck::default(),
                RangeCheck::default()
            )
        }
    }

    pub struct IsEqual<L, R> {
        _phantom: std::marker::PhantomData<(L, R)>,
    }

    impl<L, R> IsEqual<L, R> {
        pub const fn test(lhs: L, rhs: R) -> bool
        where
            L: PartialEq + Copy,
            R: PartialEq + Copy,
        {
            lhs == rhs
        }
    }

    pub struct IsNotEqual<L, R> {
        _phantom: std::marker::PhantomData<(L, R)>,
    }

    impl<L, R> IsNotEqual<L, R> {
        pub const fn test(lhs: L, rhs: R) -> bool
        where
            L: PartialEq + Copy,
            R: PartialEq + Copy,
        {
            lhs != rhs
        }
    }

    pub const fn safe_compare<C, L, R>(lhs: L, rhs: R) -> bool
    where
        L: PartialOrd + Copy,
        R: PartialOrd + Copy,
    {
        C::test(lhs, rhs)
    }

    pub const fn is_max_in_range_for_numeric_type<Dst, Src>() -> bool
    where
        Dst: PartialOrd + Copy,
        Src: PartialOrd + Copy,
    {
        IsGreaterOrEqual::<Dst, Src>::test(
            std::i32::MAX,
            std::i32::MAX
        )
    }

    pub const fn is_min_in_range_for_numeric_type<Dst, Src>() -> bool
    where
        Dst: PartialOrd + Copy,
        Src: PartialOrd + Copy,
    {
        IsLessOrEqual::<Dst, Src>::test(std::i32::MIN, std::i32::MIN)
    }

    pub const fn common_max<Dst, Src>() -> Dst
    where
        Dst: PartialOrd + Copy + From<i32>,
        Src: PartialOrd + Copy + From<i32>
    {
        if !is_max_in_range_for_numeric_type::<Dst, Src>() {
            Dst::from(std::i32::MAX)
        } else {
            Dst::from(std::i32::MAX)
        }
    }

    pub const fn common_min<Dst, Src>() -> Dst
    where
        Dst: PartialOrd + Copy + From<i32>,
        Src: PartialOrd + Copy + From<i32>
    {
        if !is_min_in_range_for_numeric_type::<Dst, Src>() {
            Dst::from(std::i32::MIN)
        } else {
            Dst::from(std::i32::MIN)
        }
    }

    pub const fn common_max_or_min<Dst, Src>(is_min: bool) -> Dst
    where
        Dst: PartialOrd + Copy + From<i32>,
        Src: PartialOrd + Copy + From<i32>
    {
        if is_min {
            common_min::<Dst, Src>()
        } else {
            common_max::<Dst, Src>()
        }
    }
}
