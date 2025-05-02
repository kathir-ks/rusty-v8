// Copyright 2014 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Slightly adapted for inclusion in V8.
// Copyright 2014 the V8 project authors. All rights reserved.
// List of adaptations:
// - include guard names
// - wrap in v8 namespace
// - formatting (git cl format)
// - include paths

pub mod base {
    pub mod internal {
        use std::{
            cmp::{max, min},
            convert::TryFrom,
            fmt::Debug,
            marker::PhantomData,
            mem,
            ops::{Neg, Sub},
            sync::atomic::{AtomicUsize, Ordering},
        };

        #[macro_export]
        macro_rules! static_assert {
            ($cond:expr, $msg:expr) => {
                const _: () = assert!($cond, $msg);
            };
        }

        pub trait Numeric:
            Copy
            + Clone
            + Debug
            + PartialEq
            + PartialOrd
            + Sized
            + 'static
        {
            const MIN: Self;
            const MAX: Self;
            const ZERO: Self;

            fn saturating_add(self, other: Self) -> Self;
            fn saturating_sub(self, other: Self) -> Self;
        }

        macro_rules! impl_numeric_for_integer {
            ($type:ty) => {
                impl Numeric for $type {
                    const MIN: Self = Self::MIN;
                    const MAX: Self = Self::MAX;
                    const ZERO: Self = 0;

                    fn saturating_add(self, other: Self) -> Self {
                        self.saturating_add(other)
                    }
                    fn saturating_sub(self, other: Self) -> Self {
                        self.saturating_sub(other)
                    }
                }
            };
        }

        macro_rules! impl_numeric_for_float {
            ($type:ty) => {
                impl Numeric for $type {
                    const MIN: Self = Self::MIN;
                    const MAX: Self = Self::MAX;
                    const ZERO: Self = 0.0;

                    fn saturating_add(self, other: Self) -> Self {
                        if self.is_infinite() && other > 0.0{
                            self
                        } else if self.is_infinite() && other < 0.0 {
                            other
                        } else {
                            self + other
                        }
                    }
                    fn saturating_sub(self, other: Self) -> Self {
                         if self.is_infinite() && other > 0.0{
                            self
                        } else if self.is_infinite() && other < 0.0 {
                            other
                        } else {
                            self - other
                        }
                    }
                }
            };
        }

        impl_numeric_for_integer!(i8);
        impl_numeric_for_integer!(i16);
        impl_numeric_for_integer!(i32);
        impl_numeric_for_integer!(i64);
        impl_numeric_for_integer!(isize);
        impl_numeric_for_integer!(u8);
        impl_numeric_for_integer!(u16);
        impl_numeric_for_integer!(u32);
        impl_numeric_for_integer!(u64);
        impl_numeric_for_integer!(usize);
        impl_numeric_for_float!(f32);
        impl_numeric_for_float!(f64);

        pub trait SafeConvert<T: Numeric>: Sized {
            fn safe_convert(self) -> Result<T, SafeConversionError>;
        }

        #[derive(Debug, PartialEq, Eq)]
        pub enum SafeConversionError {
            Overflow,
            Underflow,
            InvalidValue, // For NaN or other invalid values
        }

        impl<T: Numeric> SafeConvert<T> for T {
            fn safe_convert(self) -> Result<T, SafeConversionError> {
                Ok(self)
            }
        }

        impl<T: Numeric, U: Numeric + TryFrom<T>> SafeConvert<U> for T {
            fn safe_convert(self) -> Result<U, SafeConversionError> {
                match U::try_from(self) {
                    Ok(val) => Ok(val),
                    Err(_) => {
                        if self > T::ZERO {
                            Err(SafeConversionError::Overflow)
                        } else {
                            Err(SafeConversionError::Underflow)
                        }
                    }
                }
            }
        }

        pub trait ArithmeticOrUnderlyingEnum {}

        macro_rules! impl_arithmetic_or_enum {
            ($type:ty) => {
                impl ArithmeticOrUnderlyingEnum for $type {}
            };
        }

        impl_arithmetic_or_enum!(i8);
        impl_arithmetic_or_enum!(i16);
        impl_arithmetic_or_enum!(i32);
        impl_arithmetic_or_enum!(i64);
        impl_arithmetic_or_enum!(isize);
        impl_arithmetic_or_enum!(u8);
        impl_arithmetic_or_enum!(u16);
        impl_arithmetic_or_enum!(u32);
        impl_arithmetic_or_enum!(u64);
        impl_arithmetic_or_enum!(usize);
        impl_arithmetic_or_enum!(f32);
        impl_arithmetic_or_enum!(f64);

        #[repr(C)]
        pub union TransmuteUnion<F, T> {
            from: mem::ManuallyDrop<F>,
            to: mem::ManuallyDrop<T>,
        }

        pub fn as_signed<T: Numeric>(x: T) -> T {
            x
        }

        pub fn as_unsigned<T: Numeric>(x: T) -> T {
            x
        }

        pub fn is_value_negative<T: PartialOrd + Numeric>(value: T) -> bool {
            value < T::ZERO
        }

        pub fn safe_unsigned_abs<T: Numeric + Neg<Output = T>>(x: T) -> T {
            if x < T::ZERO {
                -x
            } else {
                x
            }
        }

        pub struct CheckOnFailure;

        impl CheckOnFailure {
            pub fn handle_failure<Dst>() -> Dst
            where
                Dst: Default + Debug,
            {
                // In C++ this would trigger a CHECK, which typically aborts the program in debug builds.
                // Here, we return a default value and print a warning. This is not ideal, but prevents the program
                // from crashing. Consider using Result<> or panic!() for more robust error handling.
                println!("WARNING: Safe conversion failed. Returning default value.");
                Dst::default()
            }
        }

        pub struct DstRangeRelationToSrcRange<Dst, SaturationHandler = SaturationDefaultLimits<Dst>, Src = Dst> {
            _phantom_data: PhantomData<(Dst, SaturationHandler, Src)>,
            is_overflow: bool,
            is_underflow: bool,
        }

        impl<Dst, SaturationHandler, Src> DstRangeRelationToSrcRange<Dst, SaturationHandler, Src>
        where
            Dst: Numeric,
            Src: Numeric,
        {
            pub fn new(is_overflow: bool, is_underflow: bool) -> Self {
                Self {
                    _phantom_data: PhantomData,
                    is_overflow,
                    is_underflow,
                }
            }

            pub fn is_valid(&self) -> bool {
                !self.is_overflow && !self.is_underflow
            }
        }

        impl<Dst, Src> DstRangeRelationToSrcRange<Dst, SaturationDefaultLimits<Dst>, Src>
        where
            Dst: Numeric,
            Src: Numeric,
        {
             pub fn new_with_value(value: Src) -> Self {
                let dst_min = Dst::MIN;
                let dst_max = Dst::MAX;

                let is_overflow = value > dst_max.safe_convert().unwrap_or(dst_max);
                let is_underflow = value < dst_min.safe_convert().unwrap_or(dst_min);

                Self {
                    _phantom_data: PhantomData,
                    is_overflow,
                    is_underflow,
                }
            }
        }

        impl<Dst, SaturationHandler, Src> RangeCheck
            for DstRangeRelationToSrcRange<Dst, SaturationHandler, Src>
        {
            fn is_overflow_flag_set(&self) -> bool {
                self.is_overflow
            }
            fn is_underflow_flag_set(&self) -> bool {
                self.is_underflow
            }
        }

        pub trait RangeCheck {
            fn is_overflow_flag_set(&self) -> bool;
            fn is_underflow_flag_set(&self) -> bool;
        }

        pub struct UnderlyingType<T>(PhantomData<T>);

        impl<T> UnderlyingType<T> {
            pub type type_alias = T;

            pub const is_numeric: bool = {
                use std::any::TypeId;
                let id = TypeId::of::<T>();
                id == TypeId::of::<i8>()
                    || id == TypeId::of::<i16>()
                    || id == TypeId::of::<i32>()
                    || id == TypeId::of::<i64>()
                    || id == TypeId::of::<isize>()
                    || id == TypeId::of::<u8>()
                    || id == TypeId::of::<u16>()
                    || id == TypeId::of::<u32>()
                    || id == TypeId::of::<u64>()
                    || id == TypeId::of::<usize>()
                    || id == TypeId::of::<f32>()
                    || id == TypeId::of::<f64>()
            };
        }

        pub const NUMERIC_RANGE_CONTAINED: i32 = 0;

        pub struct StaticDstRangeRelationToSrcRange<Dst, Src> {
            _phantom_data: PhantomData<(Dst, Src)>,
        }

        impl<Dst, Src> StaticDstRangeRelationToSrcRange<Dst, Src> {
            pub const value: i32 = {
                use std::any::TypeId;
                if TypeId::of::<Dst>() == TypeId::of::<Src>() {
                    NUMERIC_RANGE_CONTAINED
                } else {
                    0
                }
            };
        }

        // The following special case a few specific integer conversions where we can
        // eke out better performance than range checking.
        pub struct IsValueInRangeFastOp<Dst, Src> {
            _phantom_data: PhantomData<(Dst, Src)>,
        }

        impl<Dst, Src> IsValueInRangeFastOp<Dst, Src> {
            pub const is_supported: bool = false;
            pub const UNSUPPORTED_ERR_MSG: &'static str =
                "This operation is not supported for the given types.";

            pub fn do_op(value: Src) -> bool {
                // Force a compile failure if instantiated.
                CheckOnFailure::handle_failure::<bool>()
            }
        }

        impl<Dst, Src> IsValueInRangeFastOp<Dst, Src>
        where
            Dst: Numeric + TryFrom<Src>,
            Src: Numeric,
        {
            pub const is_supported: bool = true;

            pub fn do_op(value: Src) -> bool {
                match Dst::try_from(value) {
                    Ok(converted_value) => {
                        let back_to_src: Result<Src, _> = converted_value.safe_convert();
                        match back_to_src {
                            Ok(original_value) => original_value == value,
                            Err(_) => false,
                        }
                    }
                    Err(_) => false,
                }
            }
        }

        // Convenience function that returns true if the supplied value is in range
        // for the destination type.
        pub fn is_value_in_range_for_numeric_type<Dst, Src>(value: Src) -> bool
        where
            Dst: Numeric,
            Src: Numeric,
        {
            let src_type_value: <UnderlyingType<Src> as UnderlyingType<Src>>::type_alias = value;

            if IsValueInRangeFastOp::<
                Dst,
                <UnderlyingType<Src> as UnderlyingType<Src>>::type_alias,
            >::is_supported
            {
                IsValueInRangeFastOp::<
                    Dst,
                    <UnderlyingType<Src> as UnderlyingType<Src>>::type_alias,
                >::do_op(src_type_value)
            } else {
                DstRangeRelationToSrcRange::<Dst, SaturationDefaultLimits<Dst>, Src>::new_with_value(src_type_value).is_valid()
            }
        }

        // checked_cast<> is analogous to static_cast<> for numeric types,
        // except that it CHECKs that the specified numeric conversion will not
        // overflow or underflow. NaN source will always trigger a CHECK.
        pub fn checked_cast<Dst, Src>(value: Src) -> Dst
        where
            Dst: Numeric,
            Src: Numeric + Copy + Debug,
        {
            // This throws a compile-time error on evaluating the constexpr if it can be
            // determined at compile-time as failing, otherwise it will CHECK at runtime.

            if is_value_in_range_for_numeric_type::<Dst, Src>(value) {
                value.safe_convert().unwrap()
            } else {
                CheckOnFailure::handle_failure::<Dst>()
            }
        }

        // Default boundaries for integral/float: max/infinity, lowest/-infinity, 0/NaN.
        // You may provide your own limits (e.g. to saturated_cast) so long as you
        // implement all of the static constexpr member functions in the class below.
        pub struct SaturationDefaultLimits<T> {
            _phantom_data: PhantomData<T>,
        }

        impl<T> SaturationDefaultLimits<T> {
            pub fn nan() -> T
            where
                T: Numeric,
            {
                T::ZERO
            }
            pub fn max() -> T
            where
                T: Numeric,
            {
                T::MAX
            }
            pub fn overflow() -> T
            where
                T: Numeric,
            {
                T::MAX
            }
            pub fn lowest() -> T
            where
                T: Numeric,
            {
                T::MIN
            }
            pub fn underflow() -> T
            where
                T: Numeric,
            {
                T::MIN
            }
        }

        pub fn saturated_cast_impl<Dst, SaturationHandler, Src>(
            value: Src,
            constraint: impl RangeCheck,
        ) -> Dst
        where
            Dst: Numeric,
            Src: Numeric,
            SaturationHandler: Sized,
        {
            if !constraint.is_overflow_flag_set() {
                if !constraint.is_underflow_flag_set() {
                    value.safe_convert().unwrap()
                } else {
                    SaturationHandler::underflow()
                }
            }
            // Skip this check for integral Src, which cannot be NaN.
            else if !constraint.is_underflow_flag_set() {
                SaturationHandler::overflow()
            } else {
                SaturationHandler::nan()
            }
        }

        pub struct SaturateFastOp<Dst, Src> {
            _phantom_data: PhantomData<(Dst, Src)>,
        }

        impl<Dst, Src> SaturateFastOp<Dst, Src> {
            pub const is_supported: bool = false;
            pub fn do_op(value: Src) -> Dst
            where
                Dst: Numeric,
                Src: Numeric,
            {
                // Force a compile failure if instantiated.
                CheckOnFailure::handle_failure::<Dst>()
            }
        }

        // saturated_cast<> is analogous to static_cast<> for numeric types, except
        // that the specified numeric conversion will saturate by default rather than
        // overflow or underflow, and NaN assignment to an integral will return 0.
        // All boundary condition behaviors can be overridden with a custom handler.
        pub fn saturated_cast<Dst, SaturationHandler, Src>(value: Src) -> Dst
        where
            Dst: Numeric,
            Src: Numeric,
        {
            let value = value;
            if !is_constant_evaluated() && SaturateFastOp::<Dst, Src>::is_supported
                && std::any::TypeId::of::<SaturationHandler>()
                    == std::any::TypeId::of::<SaturationDefaultLimits<Dst>>()
            {
                SaturateFastOp::<Dst, Src>::do_op(value)
            } else {
                let range_check =
                    DstRangeRelationToSrcRange::<Dst, SaturationHandler, Src>::new_with_value(value);
                saturated_cast_impl::<Dst, SaturationHandler, Src>(value, range_check)
            }
        }

        // strict_cast<> is analogous to static_cast<> for numeric types, except that
        // it will cause a compile failure if the destination type is not large enough
        // to contain any value in the source type. It performs no runtime checking.
        pub fn strict_cast<Dst, Src>(value: Src) -> Dst
        where
            Dst: Numeric + From<Src>,
            Src: Numeric,
        {
            value.into()
        }

        // Some wrappers to statically check that a type is in range.
        pub struct IsNumericRangeContained<Dst, Src> {
            _phantom_data: PhantomData<(Dst, Src)>,
        }

        impl<Dst, Src> IsNumericRangeContained<Dst, Src> {
            pub const value: bool = {
                StaticDstRangeRelationToSrcRange::<Dst, Src>::value == NUMERIC_RANGE_CONTAINED
            };
        }

        // StrictNumeric implements compile time range checking between numeric types by
        // wrapping assignment operations in a strict_cast. This class is intended to be
        // used for function arguments and return types, to ensure the destination type
        // can always contain the source type. This is essentially the same as enforcing
        // -Wconversion in gcc and C4302 warnings on MSVC, but it can be applied
        // incrementally at API boundaries, making it easier to convert code so that it
        // compiles cleanly with truncation warnings enabled.
        // This template should introduce no runtime overhead, but it also provides no
        // runtime checking of any of the associated mathematical operations. Use
        // CheckedNumeric for runtime range checks of the actual value being assigned.
        #[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
        pub struct StrictNumeric<T: Numeric> {
            value_: T,
        }

        impl<T: Numeric> StrictNumeric<T> {
            pub fn new(value: T) -> Self {
                StrictNumeric { value_: value }
            }
        }

        impl<T: Numeric> Default for StrictNumeric<T> {
            fn default() -> Self {
                StrictNumeric { value_: T::ZERO }
            }
        }

        impl<T: Numeric> From<T> for StrictNumeric<T> {
            fn from(value: T) -> Self {
                StrictNumeric { value_: value }
            }
        }

        impl<T: Numeric> StrictNumeric<T> {
            // Copy constructor.
            pub fn copy_from<Src: Numeric>(rhs: &StrictNumeric<Src>) -> Self
            where
                T: From<Src>,
            {
                StrictNumeric {
                    value_: strict_cast::<T, Src>(rhs.value_),
                }
            }

            // Strictly speaking, this is not necessary, but declaring this allows class
            // template argument deduction to be used so that it is possible to simply
            // write `StrictNumeric(777)` instead of `StrictNumeric<int>(777)`.
            pub fn from_numeric(value: T) -> Self {
                StrictNumeric { value_: value }
            }

            // This is not an explicit constructor because we implicitly upgrade regular
            // numerics to StrictNumerics to make them easier to use.
            pub fn from_src<Src: Numeric>(value: Src) -> Self
            where
                T: From<Src>,
            {
                StrictNumeric {
                    value_: strict_cast::<T, Src>(value),
                }
            }
        }

        impl<T: Numeric> From<StrictNumeric<T>> for T {
            fn from(val: StrictNumeric<T>) -> Self {
                val.value_
            }
        }

        // Convenience wrapper returns a StrictNumeric from the provided arithmetic
        // type.
        pub fn make_strict_num<T: Numeric>(value: T) -> StrictNumeric<T> {
            StrictNumeric { value_: value }
        }

        #[macro_export]
        macro_rules! base_numeric_comparison_operators {
            ($class:ident, $name:ident, $op:tt) => {
                impl<L, R> std::ops:: $class <StrictNumeric<R>> for StrictNumeric<L>
                where
                    L: Numeric,
                    R: Numeric,
                {
                    type Output = bool;

                    fn $name (self, other: StrictNumeric<R>) -> Self::Output {
                        self.value_ $op other.value_
                    }
                }

            };
        }

        base_numeric_comparison_operators!(PartialEq, eq, ==);
        base_numeric_comparison_operators!(PartialOrd, partial_cmp, >=);
        base_numeric_comparison_operators!(PartialOrd, partial_cmp, <=);
        base_numeric_comparison_operators!(PartialOrd, partial_cmp, >);
        base_numeric_comparison_operators!(PartialOrd, partial_cmp, <);

        thread_local! {
            static IS_CONSTANT_EVALUATED: AtomicUsize = AtomicUsize::new(0);
        }

        pub fn is_constant_evaluated() -> bool {
            IS_CONSTANT_EVALUATED.with(|c| c.load(Ordering::Relaxed)) != 0
        }

        pub fn common_max<T: PartialOrd + Copy>(a: T, b: T) -> T {
          if a > b {
            a
          } else {
            b
          }
        }

        pub fn common_min<T: PartialOrd + Copy>(a: T, b: T) -> T {
          if a < b {
            a
          } else {
            b
          }
        }

        pub fn common_max_or_min<T: PartialOrd + Copy>(condition: bool) -> T
        where T:Numeric
        {
            if condition {
              T::MAX
            } else {
              T::MIN
            }
        }
    } // namespace internal

    pub use internal::as_signed;
    pub use internal::as_unsigned;
    pub use internal::checked_cast;
    pub use internal::is_value_in_range_for_numeric_type as IsValueInRangeForNumericType;
    pub use internal::is_value_negative as IsValueNegative;
    pub use internal::make_strict_num as MakeStrictNum;
    pub use internal::safe_unsigned_abs as SafeUnsignedAbs;
    pub use internal::saturated_cast;
    pub use internal::strict_cast;
    pub use internal::StrictNumeric;

    // Explicitly make a shorter size_t alias for convenience.
    pub type SizeT = StrictNumeric<usize>;

    // floating -> integral conversions that saturate and thus can actually return
    // an integral type.
    //
    // Generally, what you want is saturated_cast<Dst>(std::nearbyint(x)), which
    // rounds correctly according to IEEE-754 (round to nearest, ties go to nearest
    // even number; this avoids bias). If your code is performance-critical
    // and you are sure that you will never overflow, you can use std::lrint()
    // or std::llrint(), which return a long or long long directly.
    //
    // Below are convenience functions around similar patterns, except that
    // they round in nonstandard directions and will generally be slower.

    // Rounds towards negative infinity (i.e., down).
    pub fn clamp_floor<Dst, Src>(value: Src) -> Dst
    where
        Dst: Numeric,
        Src: Numeric,
    {
        saturated_cast::<Dst, internal::SaturationDefaultLimits<Dst>, Src>(value.floor())
    }

    pub trait Floor {
      fn floor(self) -> Self;
    }

    impl Floor for f32 {
        fn floor(self) -> Self {
            self.floor()
        }
    }

    impl Floor for f64 {
        fn floor(self) -> Self {
            self.floor()
        }
    }


    // Rounds towards positive infinity (i.e., up).
    pub fn clamp_ceil<Dst, Src>(value: Src) -> Dst
    where
        Dst: Numeric,
        Src: Numeric,
    {
        saturated_cast::<Dst, internal::SaturationDefaultLimits<Dst>, Src>(value.ceil())
    }

    pub trait Ceil {
      fn ceil(self) -> Self;
    }

    impl Ceil for f32 {
        fn ceil(self) -> Self {
            self.ceil()
        }
    }

    impl Ceil for f64 {
        fn ceil(self) -> Self {
            self.ceil()
        }
    }

    // Rounds towards nearest integer, with ties away from zero.
    // This means that 0.5 will be rounded to 1 and 1.5 will be rounded to 2.
    // Similarly, -0.5 will be rounded to -1 and -1.5 will be rounded to -2.
    //
    // This is normally not what you want accuracy-wise (it introduces a small bias
    // away from zero), and it is not the fastest option, but it is frequently what
    // existing code expects. Compare with saturated_cast<Dst>(std::nearbyint(x))
    // or std::lrint(x), which would round 0.5 and -0.5 to 0 but 1.5 to 2 and
    // -1.5 to -2.
    pub fn clamp_round<Dst, Src>(value: Src) -> Dst
    where
        Dst: Numeric,
        Src: Numeric,
    {
        let rounded = value.round();
        saturated_cast::<Dst, internal::SaturationDefaultLimits<Dst>, Src>(rounded)
    }

    pub trait Round {
      fn round(self) -> Self;
    }

    impl Round for f32 {
        fn round(self) -> Self {
            self.round()
        }
    }

    impl Round for f64 {
        fn round(self) -> Self {
            self.round()
        }
    }
} // namespace v8::base