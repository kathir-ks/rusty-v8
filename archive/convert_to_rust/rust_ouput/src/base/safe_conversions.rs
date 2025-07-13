// Converted from V8 C++ source files:
// Header: safe_conversions.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
pub mod internal {
use std::cmp::{max, min};
use std::fmt::Debug;
use std::marker::Copy;
use std::mem::size_of;
use std::ops::{Neg, Not};

pub struct RangeCheck {
    overflow: bool,
    underflow: bool,
}

impl RangeCheck {
    pub const fn new() -> Self {
        Self {
            overflow: false,
            underflow: false,
        }
    }
    pub const fn is_overflow(&self) -> bool {
        self.overflow
    }
    pub const fn is_underflow(&self) -> bool {
        self.underflow
    }
}
pub enum NumericRangeRelation {
    Below,
    Contained,
    Above,
    Spanning,
}
pub struct CheckOnFailure {}

impl CheckOnFailure {
    pub fn template_handle_failure<Dst>() -> Dst
    where
        Dst: Default + Copy + Debug,
    {
        println!("Check failed, returning default value");
        Dst::default()
    }
}
pub trait ArithmeticOrUnderlyingEnumT {}
impl<T> ArithmeticOrUnderlyingEnumT for T where T: std::ops::Add {}

pub struct StaticDstRangeRelationToSrcRange<Dst, Src> {
    _phantom: std::marker::PhantomData<(Dst, Src)>,
}

impl<Dst, Src> StaticDstRangeRelationToSrcRange<Dst, Src> {
    pub const value: NumericRangeRelation = NumericRangeRelation::Contained;
}

pub trait IsTypeInRangeForNumericTypeT<Dst, Src> {
    const value: bool;
}

impl<Dst, Src> IsTypeInRangeForNumericTypeT<Dst, Src> for StaticDstRangeRelationToSrcRange<Dst, Src> {
    const value: bool = false;
}

pub struct IsValueInRangeFastOp<Dst, Src> {
    _phantom: std::marker::PhantomData<(Dst, Src)>,
}

impl<Dst, Src> IsValueInRangeFastOp<Dst, Src> {
    pub const is_supported: bool = false;

    pub const fn do_it(_value: Src) -> bool {
        CheckOnFailure::template_handle_failure::<bool>()
    }
}
pub struct DstRangeRelationToSrcRange<Dst, Src> {
    _phantom: std::marker::PhantomData<(Dst, Src)>,
}

impl<Dst, Src> DstRangeRelationToSrcRange<Dst, Src> {
    pub const fn new(_value: Src) -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }

    pub const fn is_valid(&self) -> bool {
        true
    }
}

pub trait UnderlyingTypeT {}
impl<T> UnderlyingTypeT for T {}

pub struct UnderlyingType<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> UnderlyingType<T> {
    pub type type_ = T;
    pub const is_numeric: bool = true;
}

pub fn as_signed<T: num::ToPrimitive>(value: T) -> i64 {
    value.to_i64().unwrap()
}

pub fn as_unsigned<T: num::ToPrimitive>(value: T) -> u64 {
    value.to_u64().unwrap()
}
pub const NUMERIC_RANGE_CONTAINED: NumericRangeRelation = NumericRangeRelation::Contained;

pub fn is_value_negative<T: PartialOrd + Default>(value: T) -> bool {
    value < T::default()
}
pub fn safe_unsigned_abs<T: Neg<Output = T> + PartialOrd + Copy>(value: T) -> T {
    if value < T::default() {
        -value
    } else {
        value
    }
}

pub fn common_max_or_min<Dst, Src>(condition: bool) -> Dst
where
    Dst: num::Bounded + Copy,
{
    if condition {
        Dst::max_value()
    } else {
        Dst::min_value()
    }
}

pub trait IsMinInRangeForNumericTypeT<Dst, Src> {
    const value: bool;
}

impl<Dst, Src> IsMinInRangeForNumericTypeT<Dst, Src> for StaticDstRangeRelationToSrcRange<Dst, Src> {
    const value: bool = false;
}

pub fn common_max<Src: num::Bounded + Copy, Dst: num::Bounded + Copy>() -> Src{
    if size_of::<Src>() < size_of::<Dst>(){
        Src::max_value()
    } else {
        Src::max_value()
    }
}

pub trait IsMaxInRangeForNumericTypeT<Dst, Src> {
    const value: bool;
}

impl<Dst, Src> IsMaxInRangeForNumericTypeT<Dst, Src> for StaticDstRangeRelationToSrcRange<Dst, Src> {
    const value: bool = false;
}

pub struct SaturateFastAsmOp<Dst, Src> {
  _phantom: std::marker::PhantomData<(Dst, Src)>,
}

impl<Dst, Src> SaturateFastAsmOp<Dst, Src> {
  pub const is_supported: bool = false;
  pub const fn do_it(_value: Src) -> Dst {
    CheckOnFailure::template_handle_failure::<Dst>()
  }
}

}  // namespace internal

use internal::*;

pub fn is_value_in_range_for_numeric_type<Dst, Src>(value: Src) -> bool
where
    Dst: num::Num + Copy + PartialOrd,
    Src: num::ToPrimitive + Copy,
{
    let value: <internal::UnderlyingType<Src> as UnderlyingTypeT>::type_ = value;
    if internal::IsValueInRangeFastOp::<Dst, _>::is_supported {
        let value_copy = value;
        internal::IsValueInRangeFastOp::<Dst, _>::do_it(value_copy)
    } else {
        let value_copy = value;
        internal::DstRangeRelationToSrcRange::<Dst, _>::new(value_copy).is_valid()
    }
}

pub fn checked_cast<Dst, Src>(value: Src) -> Dst
where
    Dst: num::Num + Copy + PartialOrd + Default + Debug,
    Src: num::ToPrimitive + Copy,
{
    if is_value_in_range_for_numeric_type::<Dst, Src>(value) {
        value.to_primitive().unwrap()
    } else {
        CheckOnFailure::template_handle_failure::<Dst>()
    }
}

pub struct SaturationDefaultLimits<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> SaturationDefaultLimits<T> {
    pub fn nan() -> T
    where
        T: num::Float,
    {
        T::nan()
    }

    pub fn max() -> T
    where
        T: num::Bounded,
    {
        T::max_value()
    }

    pub fn overflow() -> T
    where
        T: num::Float,
    {
        T::infinity()
    }

    pub fn lowest() -> T
    where
        T: num::Bounded,
    {
        T::min_value()
    }

    pub fn underflow() -> T
    where
        T: num::Float,
    {
        T::neg_infinity()
    }
}

fn saturated_cast_impl<Dst, S, Src>(value: Src, constraint: RangeCheck) -> Dst
where
    Dst: num::Num + Copy + PartialOrd + Default + Debug,
    Src: num::ToPrimitive + Copy,
    S: SaturationHandler<Dst = Dst>,
{
    if !constraint.is_overflow() {
        if !constraint.is_underflow() {
            value.to_primitive().unwrap()
        } else {
            S::underflow()
        }
    } else {
        if std::any::TypeId::of::<Src>() == std::any::TypeId::of::<i32>() || !constraint.is_underflow() {
            S::overflow()
        } else {
            S::nan()
        }
    }
}
pub trait SaturationHandler {
    type Dst;
    fn nan() -> Self::Dst;
    fn max() -> Self::Dst;
    fn overflow() -> Self::Dst;
    fn lowest() -> Self::Dst;
    fn underflow() -> Self::Dst;
}

impl<T> SaturationHandler for SaturationDefaultLimits<T> where T: num::Float {
    type Dst = T;
    fn nan() -> Self::Dst {
        Self::nan()
    }
    fn max() -> Self::Dst {
        Self::max()
    }
    fn overflow() -> Self::Dst {
        Self::overflow()
    }
    fn lowest() -> Self::Dst {
        Self::lowest()
    }
    fn underflow() -> Self::Dst {
        Self::underflow()
    }
}

pub struct SaturateFastOp<Dst, Src> {
  _phantom: std::marker::PhantomData<(Dst, Src)>,
}

impl<Dst, Src> SaturateFastOp<Dst, Src> {
  pub const is_supported: bool = false;
  pub const fn do_it(_value: Src) -> Dst {
    CheckOnFailure::template_handle_failure::<Dst>()
  }
}

pub fn saturated_cast<Dst, S, Src>(value: Src) -> Dst
where
    Dst: num::Num + Copy + PartialOrd + Default + Debug,
    Src: num::ToPrimitive + Copy,
    S: SaturationHandler<Dst = Dst>,
{
    let value: <internal::UnderlyingType<Src> as UnderlyingTypeT>::type_ = value;
    if std::any::TypeId::of::<bool>() == std::any::TypeId::of::<bool>() && SaturateFastOp::<Dst, _>::is_supported &&
        std::any::TypeId::of::<S>() == std::any::TypeId::of::<SaturationDefaultLimits<Dst>>() {
        let value_copy = value;
        SaturateFastOp::<Dst, _>::do_it(value_copy)
    } else {
        let value_copy = value;
        let constraint = DstRangeRelationToSrcRange::<Dst, _, _>::new(value_copy);
        saturated_cast_impl::<Dst, S, _>(value_copy, constraint)
    }
}

pub fn strict_cast<Dst, Src>(value: Src) -> Dst
where
    Dst: num::Num + Copy + PartialOrd + Default + Debug,
    Src: num::ToPrimitive + Copy,
{
    value.to_primitive().unwrap()
}
pub struct IsNumericRangeContained<Dst, Src> {
    _phantom: std::marker::PhantomData<(Dst, Src)>,
}

impl<Dst, Src> IsNumericRangeContained<Dst, Src> {
    pub const value: bool = false;
}

pub struct StrictNumeric<T> {
    value_: T,
}

impl<T> StrictNumeric<T> {
    pub const fn new(value: T) -> Self {
        Self { value_: value }
    }
}

pub fn make_strict_num<T>(value: T) -> StrictNumeric<<internal::UnderlyingType<T> as UnderlyingTypeT>::type_>
where
    T: num::Num + Copy + PartialOrd + Default + Debug,
{
    StrictNumeric { value_: value }
}

pub type SizeT = StrictNumeric<usize>;

pub fn clamp_floor<Dst, Src>(value: Src) -> Dst
where
    Dst: num::Integer + Copy + PartialOrd + Default + Debug,
    Src: num::Float + num::ToPrimitive,
{
    saturated_cast::<Dst, SaturationDefaultLimits<Dst>, _>(value.floor())
}

pub fn clamp_ceil<Dst, Src>(value: Src) -> Dst
where
    Dst: num::Integer + Copy + PartialOrd + Default + Debug,
    Src: num::Float + num::ToPrimitive,
{
    saturated_cast::<Dst, SaturationDefaultLimits<Dst>, _>(value.ceil())
}

pub fn clamp_round<Dst, Src>(value: Src) -> Dst
where
    Dst: num::Integer + Copy + PartialOrd + Default + Debug,
    Src: num::Float + num::ToPrimitive,
{
    let rounded: Src = value.round();
    saturated_cast::<Dst, SaturationDefaultLimits<Dst>, _>(rounded)
}
}  // namespace v8::base
