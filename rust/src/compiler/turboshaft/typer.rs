// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/compiler/turboshaft/typer.rs

use std::{
    cmp::{max, min},
    f32, f64,
    fmt::{self, Debug, Display},
    limits::*,
    mem::MaybeUninit,
    ops::{Add, Sub},
};

use crate::compiler::turboshaft::{
    operations::*, representations::*, types::*, Word32Type, Word64Type,
};
use v8::base::logging::DCHECK_NE;

pub mod base {
    use std::vec::Vec;

    pub fn sort<T: Ord>(mut vec: Vec<T>) -> Vec<T> {
        vec.sort();
        vec
    }

    pub fn unique<T: Eq>(mut vec: Vec<T>) -> Vec<T> {
        vec.dedup();
        vec
    }

    pub fn erase_if<T, F: FnMut(&T) -> bool>(vec: &mut Vec<T>, mut predicate: F) -> usize {
        let initial_len = vec.len();
        vec.retain(|x| !predicate(x));
        initial_len - vec.len()
    }

    pub fn all_of<T, F: FnMut(&T) -> bool>(vec: &Vec<T>, mut predicate: F) -> bool {
        vec.iter().all(|x| predicate(x))
    }

    // SmallVector isn't available in Rust yet so using Vec here as a placeholder
    pub type SmallVector<T, const N: usize> = Vec<T>;

    // VectorOf isn't available in Rust yet so using Vec here as a placeholder
    pub fn VectorOf<T>(vec: Vec<T>) -> Vec<T> {
        vec
    }
}

pub mod detail {
    use std::num::FpCategory;

    pub fn is_minus_zero<T: num_traits::Float>(f: T) -> bool {
        f.classify() == FpCategory::Zero && f.is_sign_negative()
    }

    pub fn is_unique_and_sorted<T: Ord + Eq>(elements: &[T]) -> bool {
        if elements.is_empty() {
            return true;
        }
        for i in 0..(elements.len() - 1) {
            if elements[i] >= elements[i + 1] {
                return false;
            }
        }
        true
    }
}

fn nan_v<const Bits: usize>() -> f64 {
    match Bits {
        32 => f32::NAN as f64,
        64 => f64::NAN,
        _ => panic!("Unsupported bit width"),
    }
}

fn IsMinusZero<T: num_traits::Float>(v: T) -> bool {
    detail::is_minus_zero(v)
}

fn next_smaller<T: num_traits::Float>(v: T) -> T {
    if v.is_infinite() && v.is_sign_positive() {
        return T::max_value();
    }
    let mut bits = v.to_bits();
    if v.is_sign_positive() {
        if bits == 0 {
            return T::neg_zero(); // 0 -> -0
        }
        bits -= 1;
    } else {
        bits += 1;
    }
    T::from_bits(bits)
}

fn next_larger<T: num_traits::Float>(v: T) -> T {
    if v.is_infinite() && v.is_sign_negative() {
        return T::min_value();
    }
    let mut bits = v.to_bits();
    if v.is_sign_positive() {
        bits += 1;
    } else {
        if bits == T::neg_zero().to_bits() {
            return T::zero(); // -0 -> 0
        }
        bits -= 1;
    }
    T::from_bits(bits)
}

// Returns the array's least element, ignoring NaN.
// There must be at least one non-NaN element.
// Any -0 is converted to 0.
fn array_min<T: num_traits::Float, const N: usize>(a: &[T; N]) -> T {
    DCHECK_NE(0, N);
    let mut x = T::infinity();
    for i in 0..N {
        if !a[i].is_nan() {
            x = min(a[i], x);
        }
    }
    assert!(!x.is_nan());
    if x.is_zero() && x.is_sign_negative() {
        //-0 -> 0
        T::zero()
    } else {
        x
    }
}

// Returns the array's greatest element, ignoring NaN.
// There must be at least one non-NaN element.
// Any -0 is converted to 0.
fn array_max<T: num_traits::Float, const N: usize>(a: &[T; N]) -> T {
    DCHECK_NE(0, N);
    let mut x = T::neg_infinity();
    for i in 0..N {
        if !a[i].is_nan() {
            x = max(a[i], x);
        }
    }
    assert!(!x.is_nan());
    if x.is_zero() && x.is_sign_negative() {
        //-0 -> 0
        T::zero()
    } else {
        x
    }
}

struct WordOperationTyper<const Bits: usize> {
    // This static_assert becomes a compile time check in Rust
    //const _: () = assert!(Bits == 32 || Bits == 64);
}

impl<const Bits: usize> WordOperationTyper<Bits> {
    const _: () = assert!(Bits == 32 || Bits == 64); // Equivalent to static_assert

    type word_t = uint_type<Bits>;
    type type_t = WordType<Bits>;
    type ElementsVector = base::SmallVector<Self::word_t, { <Self::type_t>::K_MAX_SET_SIZE * 2 }>;
    const MAX: Self::word_t = Self::word_t::MAX;

    fn FromElements(elements: Self::ElementsVector, zone: *mut Zone) -> Self::type_t {
        let mut elements = base::sort(elements);
        elements = base::unique(elements);
        assert!(!elements.is_empty());
        if elements.len() <= <Self::type_t>::K_MAX_SET_SIZE {
            return Self::type_t::Set(elements, zone);
        }

        let range = Self::MakeRange(&base::VectorOf(elements));
        let result = Self::type_t::Range(range.0, range.1, zone);
        assert!(base::all_of(&elements, |e| result.Contains(*e)));
        return result;
    }

    fn MakeRange(t: &Self::type_t) -> (Self::word_t, Self::word_t) {
        if t.is_range() {
            return t.range();
        }
        assert!(t.is_set());
        return Self::MakeRange(t.set_elements());
    }

    // This function tries to find a somewhat reasonable range for a given set of
    // values. If the elements span no more than half of the range, we just
    // construct the range from min(elements) to max(elements) Otherwise, we
    // consider a wrapping range because it is likely that there is a larger gap
    // in the middle of the elements. For that, we start with a wrapping range
    // from max(elements) to min(elements) and then incrementally add another
    // element either by increasing the 'to' or decreasing the 'from' of the
    // range, whichever leads to a smaller range.
    fn MakeRange(elements: &Vec<Self::word_t>) -> (Self::word_t, Self::word_t) {
        assert!(!elements.is_empty());
        assert!(detail::is_unique_and_sorted(elements));
        if elements[elements.len() - 1].wrapping_sub(elements[0]) <= Self::MAX / 2 {
            // Construct a non-wrapping range.
            return (elements[0], *elements.last().unwrap());
        }
        // Construct a wrapping range.
        let mut from_index = elements.len() - 1;
        let mut to_index = 0;
        while to_index + 1 < from_index {
            if (elements[to_index + 1].wrapping_sub(elements[to_index]))
                < (elements[from_index].wrapping_sub(elements[from_index - 1]))
            {
                to_index += 1;
            } else {
                from_index -= 1;
            }
        }
        return (elements[from_index], elements[to_index]);
    }

    fn distance(range: &(Self::word_t, Self::word_t)) -> Self::word_t {
        Self::distance_word(range.0, range.1)
    }

    fn distance_word(from: Self::word_t, to: Self::word_t) -> Self::word_t {
        if Self::is_wrapping(from, to) {
            Self::MAX.wrapping_sub(from).wrapping_add(to)
        } else {
            to.wrapping_sub(from)
        }
    }

    fn is_wrapping(range: &(Self::word_t, Self::word_t)) -> bool {
        Self::is_wrapping(range.0, range.1)
    }

    fn is_wrapping(from: Self::word_t, to: Self::word_t) -> bool {
        from > to
    }

    fn Add(lhs: &Self::type_t, rhs: &Self::type_t, zone: *mut Zone) -> Self::type_t {
        if lhs.is_any() || rhs.is_any() {
            return Self::type_t::Any();
        }

        // If both sides are decently small sets, we produce the product set (which
        // we convert to a range if it exceeds the set limit).
        if lhs.is_set() && rhs.is_set() {
            let mut result_elements: Self::ElementsVector = Self::ElementsVector::new();
            for i in 0..lhs.set_size() {
                for j in 0..rhs.set_size() {
                    result_elements.push(lhs.set_element(i).wrapping_add(rhs.set_element(j)));
                }
            }
            return Self::FromElements(result_elements, zone);
        }

        // Otherwise just construct a range.
        let x = Self::MakeRange(lhs);
        let y = Self::MakeRange(rhs);

        // If the result would not be a complete range, we compute it.
        // Check: (lhs.to - lhs.from + 1) + rhs.to - rhs.from < max
        // =====> (lhs.to - lhs.from + 1) < max - rhs.to + rhs.from
        // =====> (lhs.to - lhs.from + 1) < max - (rhs.to - rhs.from)
        if Self::distance(&x).wrapping_add(Self::word_t::from(1))
            < Self::MAX.wrapping_sub(Self::distance(&y))
        {
            return Self::type_t::Range(
                x.0.wrapping_add(y.0),
                x.1.wrapping_add(y.1),
                zone,
            );
        }

        return Self::type_t::Any();
    }

    fn Subtract(lhs: &Self::type_t, rhs: &Self::type_t, zone: *mut Zone) -> Self::type_t {
        if lhs.is_any() || rhs.is_any() {
            return Self::type_t::Any();
        }

        // If both sides are decently small sets, we produce the product set (which
        // we convert to a range if it exceeds the set limit).
        if lhs.is_set() && rhs.is_set() {
            let mut result_elements: Self::ElementsVector = Self::ElementsVector::new();
            for i in 0..lhs.set_size() {
                for j in 0..rhs.set_size() {
                    result_elements.push(lhs.set_element(i).wrapping_sub(rhs.set_element(j)));
                }
            }
            return Self::FromElements(result_elements, zone);
        }

        // Otherwise just construct a range.
        let x = Self::MakeRange(lhs);
        let y = Self::MakeRange(rhs);

        if !Self::is_wrapping(&x) && !Self::is_wrapping(&y) {
            // If the result would not be a complete range, we compute it.
            // Check: (lhs.to - lhs.from + 1) + rhs.to - rhs.from < max
            // =====> (lhs.to - lhs.from + 1) < max - rhs.to + rhs.from
            // =====> (lhs.to - lhs.from + 1) < max - (rhs.to - rhs.from)
            if Self::distance(&x).wrapping_add(Self::word_t::from(1))
                < Self::MAX.wrapping_sub(Self::distance(&y))
            {
                return Self::type_t::Range(
                    x.0.wrapping_sub(y.1),
                    x.1.wrapping_sub(y.0),
                    zone,
                );
            }
        }

        // TODO(nicohartmann@): Improve the wrapping cases.
        return Self::type_t::Any();
    }

    fn UnsignedLessThan(lhs: &Self::type_t, rhs: &Self::type_t, zone: *mut Zone) -> Word32Type {
        let can_be_true = lhs.unsigned_min() < rhs.unsigned_max();
        let can_be_false = lhs.unsigned_max() >= rhs.unsigned_min();

        if !can_be_true {
            return Word32Type::Constant(0);
        }
        if !can_be_false {
            return Word32Type::Constant(1);
        }
        return Word32Type::Set(vec![0, 1], zone);
    }

    fn UnsignedLessThanOrEqual(
        lhs: &Self::type_t,
        rhs: &Self::type_t,
        zone: *mut Zone,
    ) -> Word32Type {
        let can_be_true = lhs.unsigned_min() <= rhs.unsigned_max();
        let can_be_false = lhs.unsigned_max() > rhs.unsigned_min();

        if !can_be_true {
            return Word32Type::Constant(0);
        }
        if !can_be_false {
            return Word32Type::Constant(1);
        }
        return Word32Type::Set(vec![0, 1], zone);
    }

    // Computes the ranges to which the sides of the unsigned comparison (lhs <
    // rhs) can be restricted when the comparison is true. When the comparison is
    // true, we learn: lhs cannot be >= rhs.max and rhs cannot be <= lhs.min.
    fn RestrictionForUnsignedLessThan_True(
        lhs: &Self::type_t,
        rhs: &Self::type_t,
        zone: *mut Zone,
    ) -> (Type, Type) {
        let restrict_lhs: Type;
        if rhs.unsigned_max() == Self::word_t::from(0) {
            // There is no value for lhs that could make (lhs < 0) true.
            restrict_lhs = Type::None();
        } else {
            restrict_lhs = Type::Word {
                word: Box::new(WordType::<Bits>::Range(
                    Self::word_t::from(0),
                    next_smaller(rhs.unsigned_max()),
                    zone,
                )),
            };
        }

        let restrict_rhs: Type;
        if lhs.unsigned_min() == Self::MAX {
            // There is no value for rhs that could make (max < rhs) true.
            restrict_rhs = Type::None();
        } else {
            restrict_rhs = Type::Word {
                word: Box::new(WordType::<Bits>::Range(
                    next_larger(lhs.unsigned_min()),
                    Self::MAX,
                    zone,
                )),
            };
        }

        return (restrict_lhs, restrict_rhs);
    }

    // Computes the ranges to which the sides of the unsigned comparison (lhs <
    // rhs) can be restricted when the comparison is false. When the comparison is
    // false, we learn: lhs cannot be < rhs.min and rhs cannot be > lhs.max.
    fn RestrictionForUnsignedLessThan_False(
        lhs: &Self::type_t,
        rhs: &Self::type_t,
        zone: *mut Zone,
    ) -> (Type, Type) {
        return (
            Type::Word {
                word: Box::new(WordType::<Bits>::Range(rhs.unsigned_min(), Self::MAX, zone)),
            },
            Type::Word {
                word: Box::new(WordType::<Bits>::Range(Self::word_t::from(0), lhs.unsigned_max(), zone)),
            },
        );
    }

    // Computes the ranges to which the sides of the unsigned comparison (lhs <=
    // rhs) can be restricted when the comparison is true. When the comparison is
    // true, we learn: lhs cannot be > rhs.max and rhs cannot be < lhs.min.
    fn RestrictionForUnsignedLessThanOrEqual_True(
        lhs: &Self::type_t,
        rhs: &Self::type_t,
        zone: *mut Zone,
    ) -> (Type, Type) {
        return (
            Type::Word {
                word: Box::new(WordType::<Bits>::Range(
                    Self::word_t::from(0),
                    rhs.unsigned_max(),
                    zone,
                )),
            },
            Type::Word {
                word: Box::new(WordType::<Bits>::Range(
                    lhs.unsigned_min(),
                    Self::MAX,
                    zone,
                )),
            },
        );
    }

    // Computes the ranges to which the sides of the unsigned comparison (lhs <=
    // rhs) can be restricted when the comparison is false. When the comparison is
    // false, we learn: lhs cannot be <= rhs.min and rhs cannot be >= lhs.max.
    fn RestrictionForUnsignedLessThanOrEqual_False(
        lhs: &Self::type_t,
        rhs: &Self::type_t,
        zone: *mut Zone,
    ) -> (Type, Type) {
        let restrict_lhs: Type;
        if rhs.unsigned_min() == Self::MAX {
            // There is no value for lhs that could make (lhs <= max) false.
            restrict_lhs = Type::None();
        } else {
            restrict_lhs = Type::Word {
                word: Box::new(WordType::<Bits>::Range(
                    next_larger(rhs.unsigned_min()),
                    Self::MAX,
                    zone,
                )),
            };
        }

        let restrict_rhs: Type;
        if lhs.unsigned_max() == Self::word_t::from(0) {
            // There is no value for rhs that could make (0 <= rhs) false.
            restrict_rhs = Type::None();
        } else {
            restrict_rhs = Type::Word {
                word: Box::new(WordType::<Bits>::Range(
                    Self::word_t::from(0),
                    next_smaller(lhs.unsigned_max()),
                    zone,
                )),
            };
        }

        return (restrict_lhs, restrict_rhs);
    }

    // WidenMaximal widens one of the boundary to the extreme immediately.
    fn WidenMaximal(old_type: &Self::type_t, new_type: &Self::type_t, zone: *mut Zone) -> Self::type_t {
        if new_type.is_any() {
            return new_type.clone();
        }
        if old_type.is_wrapping() || new_type.is_wrapping() {
            return Self::type_t::Any();
        }

        let mut result_from = new_type.unsigned_min();
        if result_from < old_type.unsigned_min() {
            result_from = Self::word_t::from(0);
        }
        let mut result_to = new_type.unsigned_max();
        if result_to > old_type.unsigned_max() {
            result_to = Self::MAX;
        }
        return Self::type_t::Range(result_from, result_to, zone);
    }

    // Performs exponential widening, which means that the number of values
    // described by the resulting type is at least doubled with respect to the
    // {old_type}. If {new_type} is already twice the size of {old_type},
    // {new_type} may be returned directly.
    fn WidenExponential(
        old_type: &Self::type_t,
        new_type: Self::type_t,
        zone: *mut Zone,
    ) -> Self::type_t {
        if new_type.is_any() {
            return new_type;
        }
        let (old_from, old_to, new_from, new_to): (
            Self::word_t,
            Self::word_t,
            Self::word_t,
            Self::word_t,
        );
        if old_type.is_set() {
            let old_size = old_type.set_size() as Self::word_t;
            if new_type.is_set() {
                let new_size = new_type.set_size() as Self::word_t;
                if new_size >= 2 * old_size {
                    return new_type;
                }
                (new_from, new_to) = Self::MakeRange(&new_type);
            } else {
                assert!(new_type.is_range());
                (new_from, new_to) = new_type.range();
            }
            if Self::distance_word(new_from, new_to) >= 2 * old_size {
                return Self::type_t::Range(new_from, new_to, zone);
            }
            (old_from, old_to) = Self::MakeRange(old_type);
        } else {
            assert!(old_type.is_range());
            (old_from, old_to) = old_type.range();
            if new_type.is_set() {
                (new_from, new_to) = Self::MakeRange(&new_type);
            } else {
                assert!(new_type.is_range());
                (new_from, new_to) = new_type.range();
            }
        }

        // If the old type is already quite large, we go to full range.
        if Self::distance_word(old_from, old_to) >= Self::MAX / 4 {
            return Self::type_t::Any();
        }

        let min_size = 2 * (Self::distance_word(old_from, old_to).wrapping_add(Self::word_t::from(1)));
        if Self::distance_word(new_from, new_to) >= min_size {
            return Self::type_t::Range(new_from, new_to, zone);
        }

        // If old is wrapping (and so is new).
        if Self::is_wrapping(old_from, old_to) {
            assert!(Self::is_wrapping(new_from, new_to));
            if new_from < old_from {
                assert!(old_to <= new_to);
                // We widen the `from` (although `to` might have grown, too).
                assert!(new_to < min_size);
                let result_from =
                    Self::MAX.wrapping_sub(min_size.wrapping_sub(new_to));
                assert!(result_from < new_from);
                assert!(min_size <= Self::distance_word(result_from, new_to));
                return Self::type_t::Range(result_from, new_to, zone);
            } else {
                assert_eq!(old_from, new_from);
                // We widen the `to`.
                assert!(Self::MAX.wrapping_sub(new_from) < min_size);
                let result_to =
                    min_size.wrapping_sub(Self::MAX.wrapping_sub(new_from));
                assert!(result_to > new_to);
                assert!(min_size <= Self::distance_word(new_from, result_to));
                return Self::type_t::Range(new_from, result_to, zone);
            }
        }

        // If old is not wrapping, but new is.
        if Self::is_wrapping(new_from, new_to) {
            if new_to < old_to {
                // If wrapping was caused by to growing over max, grow `to` further
                // (although `from` might have grown, too).
                assert!(Self::MAX.wrapping_sub(new_from) < min_size);
                let result_to =
                    min_size.wrapping_sub(Self::MAX.wrapping_sub(new_from));
                assert!(new_to < result_to);
                return Self::type_t::Range(new_from, result_to, zone);
            } else {
                assert!(old_from < new_from);
                // If wrapping was caused by `from` growing below 0, grow `from`
                // further.
                assert!(new_to < min_size);
                let result_from =
                    Self::MAX.wrapping_sub(min_size.wrapping_sub(new_to));
                assert!(result_from < new_from);
                return Self::type_t::Range(result_from, new_to, zone);
            }
        }

        // Neither old nor new is wrapping.
        if new_from < old_from {
            assert!(old_to <= new_to);
            // Check if we can widen the `from`.
            if new_to >= min_size {
                // We can decrease `from` without going below 0.
                let result_from = new_to.wrapping_sub(min_size);
                assert!(result_from < new_from);
                return Self::type_t::Range(result_from, new_to, zone);
            } else {
                // We cannot grow `from` enough, so we also have to grow `to`.
                return Self::type_t::Range(Self::word_t::from(0), min_size, zone);
            }
        } else {
            assert_eq!(old_from, new_from);
            // Check if we can widen the `to`.
            if new_from <= Self::MAX.wrapping_sub(min_size) {
                // We can increase `to` without going above max.
                let result_to = new_from.wrapping_add(min_size);
                assert!(result_to > new_to);
                return Self::type_t::Range(new_from, result_to, zone);
            } else {
                // We cannot grow `to` enough, so we also have to grow `from`.
                return Self::type_t::Range(Self::MAX.wrapping_sub(min_size), Self::MAX, zone);
            }
        }
    }
}

struct FloatOperationTyper<const Bits: usize> {
    // static_assert(Bits == 32 || Bits == 64);
}

impl<const Bits: usize> FloatOperationTyper<Bits> {
    const _: () = assert!(Bits == 32 || Bits == 64);

    type float_t = f64; //std::conditional_t<Bits == 32, f32, f64>;  // TODO: Fix conditional type

    type type_t = FloatType<Bits>;
    const INF: Self::float_t = f64::INFINITY; //std::numeric_limits<Self::float_t>::infinity();
    const K_SET_THRESHOLD: usize = <Self::type_t>::K_MAX_SET_SIZE;

    fn Range(
        min: Self::float_t,
        max: Self::float_t,
        special_values: u32,
        zone: *mut Zone,
    ) -> Self::type_t {
        assert!(min <= max);
        if detail::is_minus_zero(min) {
            assert!((special_values & <Self::type_t>::K_MINUS_ZERO) != 0);
        }
        if detail::is_minus_zero(max) {
            assert!((special_values & <Self::type_t>::K_MINUS_ZERO) != 0);
        }
        if min == max {
            return Self::Set(vec![min + Self::float_t::from(0)], special_values, zone);
        }
        return <Self::type_t>::Range(min, max, special_values, zone);
    }

    fn Set(
        mut elements: Vec<Self::float_t>,
        special_values: u32,
        zone: *mut Zone,
    ) -> Self::type_t {
        elements.sort_by(|a, b| a.partial_cmp(b).unwrap()); // TODO: Handle NaN
        elements.dedup();
        let mut special_values = special_values;
        if base::erase_if(&mut elements, |v| v.is_nan()) > 0 {
            special_values |= <Self::type_t>::K_NAN;
        }
        if base::erase_if(&mut elements, |v| IsMinusZero(*v)) > 0 {
            special_values |= <Self::type_t>::K_MINUS_ZERO;
        }
        if elements.is_empty() {
            assert_ne!(0, special_values);
            return <Self::type_t>::OnlySpecialValues(special_values);
        }
        return <Self::type_t>::Set(elements, special_values, zone);
    }

    // Check if the elements in the set are all integers. This ignores special
    // values (NaN, -0)!
    fn IsIntegerSet(t: &Self::type_t) -> bool {
        if !t.is_set() {
            return false;
        }
        let size = t.set_size();
        assert!(size > 0);

        let mut unused_ipart = 0.0; //f64
        let min = t.set_element(0);
        if min.fract() != 0.0 {
            return false;
        }
        if min == -Self::INF {
            return false;
        }
        let max = t.set_element(size - 1);
        if max.fract() != 0.0 {
            return false;
        }
        if max == Self::INF {
            return false;
        }

        for i in 1..size - 1 {
            if t.set_element(i).fract() != 0.0 {
                return false;
            }
        }
        return true;
    }

    fn IsZeroish(l: &Self::type_t) -> bool {
        l.has_nan() || l.has_minus_zero() || l.Contains(0.0)
    }

    // Tries to construct the product of two sets where values are generated using
    // {combine}. Returns Type::Invalid() if a set cannot be constructed (e.g.
    // because the result exceeds the maximal number of set elements).
    fn ProductSet(
        l: &Self::type_t,
        r: &Self::type_t,
        special_values: u32,
        zone: *mut Zone,
        combine: impl Fn(Self::float_t, Self::float_t) -> Self::float_t,
    ) -> Type {
        assert!(l.is_set());
        assert!(r.is_set());

        let mut results: Vec<Self::float_t> = Vec::new();
        let mut special_values = special_values;

        let CombineWithLeft = |left: Self::float_t| {
            for j in 0..r.set_size() {
                results.push(combine(left, r.set_element(j)));
            }
            if r.has_minus_zero() {
                results.push(combine(left, -0.0));
            }
            if r.has_nan() {
                results.push(combine(left, nan_v::<Bits>()));
            }
        };

        for i in 0..l.set_size() {
            CombineWithLeft(l.set_element(i));
        }
        if l.has_minus_zero() {
            CombineWithLeft(-0.0);
        }
        if l.has_nan() {
            CombineWithLeft(nan_v::<Bits>());
        }

        if base::erase_if(&mut results, |v| v.is_nan()) > 0 {
            special_values |= <Self::type_t>::K_NAN;
        }
        if base::erase_if(&mut results, |v| IsMinusZero(*v)) > 0 {
            special_values |= <Self::type_t>::K_MINUS_ZERO;
        }
        results.sort_by(|a, b| a.partial_cmp(b).unwrap()); // TODO: Handle NaN
        results.dedup();
        if results.len() > Self::K_SET_THRESHOLD {
            return Type::Invalid();
        }
        results.shrink_to_fit();
        if results.is_empty() {
            return Self::type_t::