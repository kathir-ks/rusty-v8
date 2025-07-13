// Converted from V8 C++ source files:
// Header: typer.h
// Implementation: typer.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod turboshaft {
    use std::cmp::{max, min};
    use std::f64::INFINITY;
    use std::fmt;
    use std::mem::size_of;
    use std::ops::{BitAnd, BitOr};

    pub use crate::compiler::dead_code_elimination::Type;
    use crate::compiler::loop_unrolling::V8;

    pub struct ConstantOp {
        pub kind: Kind,
        pub storage: Storage,
    }
    impl ConstantOp {
        pub enum Kind {
            kFloat32,
            kFloat64,
            kWord32,
            kWord64,
        }
    }
    pub struct Storage {
        pub float32: Float32Wrapper,
        pub float64: Float64Wrapper,
        pub integral: i64,
    }

    pub struct Float32Wrapper {
        scalar: f32,
    }
    impl Float32Wrapper {
        pub fn get_scalar(&self) -> f32 {
            self.scalar
        }
        pub fn is_nan(&self) -> bool {
            self.scalar.is_nan()
        }
    }

    pub struct Float64Wrapper {
        scalar: f64,
    }
    impl Float64Wrapper {
        pub fn get_scalar(&self) -> f64 {
            self.scalar
        }
        pub fn is_nan(&self) -> bool {
            self.scalar.is_nan()
        }
    }

    pub fn IsMinusZero<T>(v: T) -> bool
    where
        T: Into<f64>,
    {
        let float_value: f64 = v.into();
        float_value == 0.0 && float_value.is_sign_negative()
    }

    pub mod detail {
        pub fn is_minus_zero<T>(v: T) -> bool
        where
            T: Into<f64>,
        {
            let float_value: f64 = v.into();
            float_value == 0.0 && float_value.is_sign_negative()
        }
        pub fn is_unique_and_sorted<T: Ord>(elements: &[T]) -> bool {
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

    pub fn array_min<T: PartialOrd + Copy, const N: usize>(a: &[T; N]) -> T {
        assert_ne!(0, N);
        let mut x = T::infinity();
        for i in 0..N {
            if !a[i].is_nan() {
                x = if a[i] < x { a[i] } else { x };
            }
        }
        assert!(!x.is_nan());
        if x == T::from(0) {
            T::from(0)
        } else {
            x
        } // -0 -> 0

        trait Infinity {
            fn infinity() -> Self;
            fn is_nan(&self) -> bool;
        }

        macro_rules! impl_infinity {
            ($type:ty, $inf:expr) => {
                impl Infinity for $type {
                    fn infinity() -> Self {
                        $inf
                    }
                    fn is_nan(&self) -> bool {
                        Self::is_nan(*self)
                    }
                }
            };
        }

        impl_infinity!(f32, std::f32::INFINITY);
        impl_infinity!(f64, std::f64::INFINITY);

        trait From<T> {
            fn from(_: T) -> Self;
        }

        impl From<i32> for f32 {
            fn from(x: i32) -> Self {
                x as Self
            }
        }
        impl From<i32> for f64 {
            fn from(x: i32) -> Self {
                x as Self
            }
        }
        impl From<f32> for f32 {
            fn from(x: f32) -> Self {
                x
            }
        }
        impl From<f64> for f64 {
            fn from(x: f64) -> Self {
                x
            }
        }
    }

    pub fn array_max<T: PartialOrd + Copy, const N: usize>(a: &[T; N]) -> T {
        assert_ne!(0, N);
        let mut x = T::neg_infinity();
        for i in 0..N {
            if !a[i].is_nan() {
                x = if a[i] > x { a[i] } else { x };
            }
        }
        assert!(!x.is_nan());
        if x == T::from(0) {
            T::from(0)
        } else {
            x
        } // -0 -> 0
        trait NegInfinity {
            fn neg_infinity() -> Self;
            fn is_nan(&self) -> bool;
        }

        macro_rules! impl_neginfinity {
            ($type:ty, $neginf:expr) => {
                impl NegInfinity for $type {
                    fn neg_infinity() -> Self {
                        $neginf
                    }
                    fn is_nan(&self) -> bool {
                        Self::is_nan(*self)
                    }
                }
            };
        }
        impl_neginfinity!(f32, std::f32::NEG_INFINITY);
        impl_neginfinity!(f64, std::f64::NEG_INFINITY);
        trait Infinity {
            fn infinity() -> Self;
            fn is_nan(&self) -> bool;
        }

        macro_rules! impl_infinity {
            ($type:ty, $inf:expr) => {
                impl Infinity for $type {
                    fn infinity() -> Self {
                        $inf
                    }
                    fn is_nan(&self) -> bool {
                        Self::is_nan(*self)
                    }
                }
            };
        }

        impl_infinity!(f32, std::f32::INFINITY);
        impl_infinity!(f64, std::f64::INFINITY);

        trait From<T> {
            fn from(_: T) -> Self;
        }

        impl From<i32> for f32 {
            fn from(x: i32) -> Self {
                x as Self
            }
        }
        impl From<i32> for f64 {
            fn from(x: i32) -> Self {
                x as Self
            }
        }
        impl From<f32> for f32 {
            fn from(x: f32) -> Self {
                x
            }
        }
        impl From<f64> for f64 {
            fn from(x: f64) -> Self {
                x
            }
        }
    }

    pub struct WordOperationTyper<const Bits: usize> {
        _phantom: std::marker::PhantomData<[(); Bits]>,
    }

    impl<const Bits: usize> WordOperationTyper<Bits> {
        pub fn from_elements(
            elements: Vec<u64>,
            zone: *mut Zone,
        ) -> Result<WordType<Bits>, String> {
            if Bits != 32 && Bits != 64 {
                return Err("Bits must be 32 or 64".to_string());
            }
            let mut elements = elements;
            elements.sort();
            elements.dedup();
            if elements.is_empty() {
                return Err("Elements must not be empty".to_string());
            }
            if elements.len() <= WordType::<Bits>::kMaxSetSize as usize {
                Ok(WordType::<Bits>::set(elements, zone))
            } else {
                let range = Self::make_range(&elements);
                Ok(WordType::<Bits>::range(range.0, range.1, zone))
            }
        }

        fn make_range(elements: &[u64]) -> (u64, u64) {
            assert!(!elements.is_empty());
            if elements[elements.len() - 1] - elements[0] <= u64::MAX / 2 {
                (elements[0], elements[elements.len() - 1])
            } else {
                let mut from_index = elements.len() - 1;
                let mut to_index = 0;
                while to_index + 1 < from_index {
                    if (elements[to_index + 1] - elements[to_index])
                        < (elements[from_index] - elements[from_index - 1])
                    {
                        to_index += 1;
                    } else {
                        from_index -= 1;
                    }
                }
                (elements[from_index], elements[to_index])
            }
        }

        fn distance(range: (u64, u64)) -> u64 {
            if range.0 > range.1 {
                (u64::MAX - range.0) + range.1
            } else {
                range.1 - range.0
            }
        }

        fn is_wrapping(range: (u64, u64)) -> bool {
            range.0 > range.1
        }

        pub fn add(lhs: WordType<Bits>, rhs: WordType<Bits>, zone: *mut Zone) -> WordType<Bits> {
            if lhs.is_any() || rhs.is_any() {
                return WordType::<Bits>::any();
            }

            if lhs.is_set() && rhs.is_set() {
                let mut result_elements: Vec<u64> = Vec::new();
                for i in 0..lhs.set_size() {
                    for j in 0..rhs.set_size() {
                        result_elements.push(lhs.set_element(i) + rhs.set_element(j));
                    }
                }

                match Self::from_elements(result_elements, zone) {
                    Ok(res) => return res,
                    Err(_e) => return WordType::<Bits>::any()
                }
            }

            let x = Self::make_range(&(lhs.elements()));
            let y = Self::make_range(&(rhs.elements()));

            if Self::distance(x) + 1 < u64::MAX - Self::distance(y) {
                return WordType::<Bits>::range(x.0 + y.0, x.1 + y.1, zone);
            }

            WordType::<Bits>::any()
        }

        pub fn subtract(lhs: WordType<Bits>, rhs: WordType<Bits>, zone: *mut Zone) -> WordType<Bits> {
            if lhs.is_any() || rhs.is_any() {
                return WordType::<Bits>::any();
            }
            if lhs.is_set() && rhs.is_set() {
                let mut result_elements: Vec<u64> = Vec::new();
                for i in 0..lhs.set_size() {
                    for j in 0..rhs.set_size() {
                        result_elements.push(lhs.set_element(i) - rhs.set_element(j));
                    }
                }

                match Self::from_elements(result_elements, zone) {
                    Ok(res) => return res,
                    Err(_e) => return WordType::<Bits>::any()
                }
            }

            let x = Self::make_range(&(lhs.elements()));
            let y = Self::make_range(&(rhs.elements()));

            if !Self::is_wrapping(x) && !Self::is_wrapping(y) {
                if Self::distance(x) + 1 < u64::MAX - Self::distance(y) {
                    return WordType::<Bits>::range(x.0 - y.1, x.1 - y.0, zone);
                }
            }

            WordType::<Bits>::any()
        }

        pub fn unsigned_less_than(lhs: WordType<Bits>, rhs: WordType<Bits>, zone: *mut Zone) -> Word32Type {
            let can_be_true = lhs.unsigned_min() < rhs.unsigned_max();
            let can_be_false = lhs.unsigned_max() >= rhs.unsigned_min();

            if !can_be_true {
                return Word32Type::constant(0);
            }
            if !can_be_false {
                return Word32Type::constant(1);
            }
            Word32Type::set(vec![0, 1], zone)
        }

        pub fn unsigned_less_than_or_equal(lhs: WordType<Bits>, rhs: WordType<Bits>, zone: *mut Zone) -> Word32Type {
            let can_be_true = lhs.unsigned_min() <= rhs.unsigned_max();
            let can_be_false = lhs.unsigned_max() > rhs.unsigned_min();

            if !can_be_true {
                return Word32Type::constant(0);
            }
            if !can_be_false {
                return Word32Type::constant(1);
            }
            Word32Type::set(vec![0, 1], zone)
        }

        pub fn restriction_for_unsigned_less_than_true(
            lhs: WordType<Bits>,
            rhs: WordType<Bits>,
            zone: *mut Zone,
        ) -> (Type, Type) {
            let restrict_lhs = if rhs.unsigned_max() == 0 {
                Type::None()
            } else {
                Type::Word64(WordType::<64>::range(0, Self::next_smaller(rhs.unsigned_max()), zone))
            };

            let restrict_rhs = if lhs.unsigned_min() == u64::MAX {
                Type::None()
            } else {
                Type::Word64(WordType::<64>::range(Self::next_larger(lhs.unsigned_min()), u64::MAX, zone))
            };

            (restrict_lhs, restrict_rhs)
        }

        pub fn restriction_for_unsigned_less_than_false(
            lhs: WordType<Bits>,
            rhs: WordType<Bits>,
            zone: *mut Zone,
        ) -> (Type, Type) {
            let restrict_lhs = Type::Word64(WordType::<64>::range(rhs.unsigned_min(), u64::MAX, zone));
            let restrict_rhs = Type::Word64(WordType::<64>::range(0, lhs.unsigned_max(), zone));

            (restrict_lhs, restrict_rhs)
        }

        pub fn restriction_for_unsigned_less_than_or_equal_true(
            lhs: WordType<Bits>,
            rhs: WordType<Bits>,
            zone: *mut Zone,
        ) -> (Type, Type) {
            let restrict_lhs = Type::Word64(WordType::<64>::range(0, rhs.unsigned_max(), zone));
            let restrict_rhs = Type::Word64(WordType::<64>::range(lhs.unsigned_min(), u64::MAX, zone));

            (restrict_lhs, restrict_rhs)
        }

        pub fn restriction_for_unsigned_less_than_or_equal_false(
            lhs: WordType<Bits>,
            rhs: WordType<Bits>,
            zone: *mut Zone,
        ) -> (Type, Type) {
            let restrict_lhs = if rhs.unsigned_min() == u64::MAX {
                Type::None()
            } else {
                Type::Word64(WordType::<64>::range(Self::next_larger(rhs.unsigned_min()), u64::MAX, zone))
            };

            let restrict_rhs = if lhs.unsigned_max() == 0 {
                Type::None()
            } else {
                Type::Word64(WordType::<64>::range(0, Self::next_smaller(lhs.unsigned_max()), zone))
            };

            (restrict_lhs, restrict_rhs)
        }
        fn next_smaller(x: u64) -> u64 {
            if x == 0 {
                return 0;
            }
            x - 1
        }

        fn next_larger(x: u64) -> u64 {
            if x == u64::MAX {
                return u64::MAX;
            }
            x + 1
        }

        pub fn widen_maximal(old_type: WordType<Bits>, new_type: WordType<Bits>, zone: *mut Zone) -> WordType<Bits> {
            if new_type.is_any() {
                return new_type;
            }
            if old_type.is_wrapping() || new_type.is_wrapping() {
                return WordType::<Bits>::any();
            }

            let mut result_from = new_type.unsigned_min();
            if result_from < old_type.unsigned_min() {
                result_from = 0;
            }
            let mut result_to = new_type.unsigned_max();
            if result_to > old_type.unsigned_max() {
                result_to = u64::MAX;
            }
            WordType::<Bits>::range(result_from, result_to, zone)
        }
        pub fn widen_exponential(old_type: WordType<Bits>, new_type: WordType<Bits>, zone: *mut Zone) -> WordType<Bits> {
            if new_type.is_any() {
                return new_type;
            }

            let (old_from, old_to) = if old_type.is_set() {
                let old_size = old_type.set_size();
                let (new_from, new_to) = if new_type.is_set() {
                    let new_size = new_type.set_size();
                    if new_size >= 2 * old_size {
                        return new_type;
                    }
                    Self::make_range(&new_type.elements())
                } else {
                    Self::make_range(&new_type.elements())
                };

                if Self::distance((new_from, new_to)) >= 2 * old_size {
                    return WordType::<Bits>::range(new_from, new_to, zone);
                }
                Self::make_range(&old_type.elements())
            } else {
                let (new_from, new_to) = if new_type.is_set() {
                    Self::make_range(&new_type.elements())
                } else {
                    Self::make_range(&new_type.elements())
                };
                Self::make_range(&old_type.elements())
            };

            if Self::distance((old_from, old_to)) >= u64::MAX / 4 {
                return WordType::<Bits>::any();
            }

            let min_size = 2 * (Self::distance((old_from, old_to)) + 1);
            let (new_from, new_to) = Self::make_range(&new_type.elements());

            if Self::distance((new_from, new_to)) >= min_size {
                return WordType::<Bits>::range(new_from, new_to, zone);
            }

            if Self::is_wrapping((old_from, old_to)) {
                assert!(Self::is_wrapping((new_from, new_to)));

                if new_from < old_from {
                    assert!(old_to <= new_to);
                    assert!(new_to < min_size);
                    let result_from = u64::MAX - (min_size - new_to);
                    assert!(result_from < new_from);
                    assert!(min_size <= Self::distance((result_from, new_to)));
                    return WordType::<Bits>::range(result_from, new_to, zone);
                } else {
                    assert_eq!(old_from, new_from);
                    assert!(u64::MAX - new_from < min_size);
                    let result_to = min_size - (u64::MAX - new_from);
                    assert!(result_to > new_to);
                    assert!(min_size <= Self::distance((new_from, result_to)));
                    return WordType::<Bits>::range(new_from, result_to, zone);
                }
            }

            if Self::is_wrapping((new_from, new_to)) {
                if new_to < old_to {
                    assert!(u64::MAX - new_from < min_size);
                    let result_to = min_size - (u64::MAX - new_from);
                    assert!(new_to < result_to);
                    return WordType::<Bits>::range(new_from, result_to, zone);
                } else {
                    assert!(old_from < new_from);
                    assert!(new_to < min_size);
                    let result_from = u64::MAX - (min_size - new_to);
                    assert!(result_from < new_from);
                    return WordType::<Bits>::range(result_from, new_to, zone);
                }
            }

            if new_from < old_from {
                assert!(old_to <= new_to);
                if new_to >= min_size {
                    let result_from = new_to - min_size;
                    assert!(result_from < new_from);
                    return WordType::<Bits>::range(result_from, new_to, zone);
                } else {
                    return WordType::<Bits>::range(0, min_size, zone);
                }
            } else {
                assert_eq!(old_from, new_from);
                if new_from <= u64::MAX - min_size {
                    let result_to = new_from + min_size;
                    assert!(result_to > new_to);
                    return WordType::<Bits>::range(new_from, result_to, zone);
                } else {
                    return WordType::<Bits>::range(u64::MAX - min_size, u64::MAX, zone);
                }
            }
        }
    }

    pub trait TypeProperties {
        fn is_set(&self) -> bool;
        fn is_range(&self) -> bool;
        fn is_any(&self) -> bool;
        fn is_wrapping(&self) -> bool;
    }

    macro_rules! define_word_type {
        ($name:ident, $bits:expr) => {
            #[derive(Debug, Clone, Copy, PartialEq)]
            pub enum $name {
                Invalid,
                Any,
                Range { from: u64, to: u64 },
                Set { elements: [u64; 4] }, // Keep small to fit in registers.
                Constant(u64),
            }
            impl $name {
                pub const kMaxSetSize: i32 = 4;
                pub const kNoSpecialValues: u32 = 0;

                pub fn any() -> Self {
                    Self::Any
                }
                pub fn is_any(&self) -> bool {
                    match self {
                        Self::Any => true,
                        _ => false,
                    }
                }
                pub fn is_set(&self) -> bool {
                    match self {
                        Self::Set { .. } => true,
                        _ => false,
                    }
                }
                pub fn is_range(&self) -> bool {
                    match self {
                        Self::Range { .. } => true,
                        _ => false,
                    }
                }
                pub fn range(from: u64, to: u64, _zone: *mut Zone) -> Self {
                    Self::Range { from, to }
                }
                pub fn constant(value: u64) -> Self {
                    Self::Constant(value)
                }
                pub fn set(elements: Vec<u64>, _zone: *mut Zone) -> Self {
                    assert!(elements.len() <= Self::kMaxSetSize as usize);
                    let mut arr: [u64; 4] = [0; 4];
                    for (i, &elem) in elements.iter().enumerate() {
                        arr[i] = elem;
                    }
                    Self::Set { elements: arr }
                }
                pub fn set_size(&self) -> i32 {
                    match self {
                        Self::Set { .. } => Self::kMaxSetSize,
                        _ => 0,
                    }
                }
                pub fn set_element(&self, index: i32) -> u64 {
                    match self {
                        Self::Set { elements } => elements[index as usize],
                        _ => 0,
                    }
                }

                pub fn elements(&self) -> Vec<u64> {
                    match self {
                        Self::Range { from, to } => vec![*from, *to],
                        Self::Set { elements } => elements.to_vec(),
                        _ => vec![]
                    }
                }

                pub fn unsigned_min(&self) -> u64 {
                    match self {
                        Self::Range { from, .. } => *from,
                        Self::Set { elements } => elements[0],
                        Self::Constant(c) => *c,
                        _ => 0,
                    }
                }

                pub fn unsigned_max(&self) -> u64 {
                    match self {
                        Self::Range { to, .. } => *to,
                        Self::Set { elements } => {
                            elements[3]
                        }
                        Self::Constant(c) => *c,
                        _ => u64::MAX,
                    }
                }
                pub fn try_get_constant(&self) -> Option<&u64> {
                    match self {
                        Self::Constant(val) => Some(val),
                        _ => None,
                    }
                }
                pub fn is_constant(&self) -> bool {
                    match self {
                        Self::Constant(_) => true,
                        _ => false,
                    }
                }
                pub fn is_wrapping(&self) -> bool {
                    match self {
                        Self::Range { from, to } => from > to,
                        _ => false,
                    }
                }
            }
        };
    }

    define_word_type!(Word32Type, 32);
    define_word_type!(Word64Type, 64);

    impl Word32Type {
        pub fn set(elements: Vec<u32>, zone: *mut Zone) -> Self {
            assert!(elements.len() <= Self::kMaxSetSize as usize);
            let mut arr: [u64; 4] = [0; 4];
            for (i, &elem) in elements.iter().enumerate() {
                arr[i] = elem as u64;
            }
            Self::Set { elements: arr }
        }
        pub fn constant(value: i32) -> Self {
            Self::Constant(value as u64)
        }
    }

    pub struct FloatOperationTyper<const Bits: usize> {
        _phantom: std::marker::PhantomData<[(); Bits]>,
    }
    impl<const Bits: usize> FloatOperationTyper<Bits> {
        pub fn range(
            min: f64,
            max: f64,
            special_values: u32,
            zone: *mut Zone,
        ) -> FloatType<Bits> {
            if min == max {
                let mut elements = vec![min + 0.0];
                FloatType::<Bits>::set(elements, special_values, zone)
            } else {
                FloatType::<Bits>::range(min, max, special_values, zone)
            }
        }

        pub fn set(elements: Vec<f64>, special_values: u32, zone: *mut Zone) -> FloatType<Bits> {
            let mut elements = elements;
            elements.sort_by(|a, b| a.partial_cmp(b).unwrap());
            elements.dedup();
            let mut special_values = special_values;

            let mut nans_removed = 0;
            elements.retain(|&v| {
                if v.is_nan() {
                    nans_removed += 1;
                    false
                } else {
                    true
                }
            });
            special_values |= if nans_removed > 0 {
                FloatType::<Bits>::kNaN
            } else {
                0
            };

            let mut minus_zeros_removed = 0;
            elements.retain(|&v| {
                if detail::is_minus_zero(v) {
                    minus_zeros_removed += 1;
                    false
                } else {
                    true
                }
            });
            special_values |= if minus_zeros_removed > 0 {
                FloatType::<Bits>::kMinusZero
            } else {
                0
            };
            if elements.is_empty() {
                assert_ne!(0, special_values);
                return FloatType::<Bits>::only_special_values(special_values);
            }
            FloatType::<Bits>::set(elements, special_values, zone)
        }

        pub fn is_integer_set(t: &FloatType<Bits>) -> bool {
            if !t.is_set() {
                return false;
            }
            let size = t.set_size();
            assert!(0 < size);

            let mut unused_ipart: f64 = 0.0;
            let min = t.set_element(0);
            if std::modf(min).1 != 0.0 {
                return false;
            }
            if min == f64::NEG_INFINITY {
                return false;
            }
            let max = t.set_element(size - 1);
            if std::modf(max).1 != 0.0 {
                return false;
            }
            if max == f64::INFINITY {
                return false;
            }

            for i in 1..size - 1 {
                if std::modf(t.set_element(i)).1 != 0.0 {
                    return false;
                }
            }
            true
        }

        pub fn is_zeroish(l: &FloatType<Bits>) -> bool {
            l.has_nan() || l.has_minus_zero() || l.contains(0.0)
        }
        pub fn product_set(
            l: &FloatType<Bits>,
            r: &FloatType<Bits>,
            special_values: u32,
            zone: *mut Zone,
            combine: fn(f64, f64) -> f64,
        ) -> Result<Type, String> {
            assert!(l.is_set());
            assert!(r.is_set());
            let mut results: Vec<f64> = Vec::new();

            let combine_with_left = |left: f64| {
                for j in 0..r.set_size() {
                    results.push(combine(left, r.set_element(j)));
                }
                if r.has_minus_zero() {
                    results.push(combine(left, -0.0));
                }
                if r.has_nan() {
                    results.push(combine(left, Self::nan_v()));
                }
            };

            for i in 0..l.set_size() {
                combine_with_left(l.set_element(i));
            }
            if l.has_minus_zero() {
                combine_with_left(-0.0);
            }
            if l.has_nan() {
                combine_with_left(Self::nan_v());
            }

            let mut special_values = special_values;
            let mut nans_removed = 0;
            results.retain(|&v| {
                if v.is_nan() {
                    nans_removed += 1;
                    false
                } else {
                    true
                }
            });
            special_values |= if nans_removed > 0 {
                FloatType::<Bits>::kNaN
            } else {
                0
            };
            let mut minus_zeros_removed = 0;
            results.retain(|&v| {
                if detail::is_minus_zero(v) {
                    minus_zeros_removed += 1;
                    false
                } else {
                    true
                }
            });
            special_values |= if minus_zeros_removed > 0 {
                FloatType::<Bits>::kMinusZero
            } else {
                0
            };

            results.sort_by(|a, b| a.partial_cmp(b).unwrap());
            results.dedup();
            if results.len() > FloatType::<Bits>::kMaxSetSize as usize {
                return Err("Type::Invalid".to_string());
            }

            if results.is_empty() {
                return Ok(Type::Float64(FloatType::<64>::only_special_values(special_values)));
            }
            let set = Self::set(results, special_values, zone);
            Ok(Type::Float64(set))
        }

        pub fn add(l: FloatType<Bits>, r: FloatType<Bits>, zone: *mut Zone) -> Type {
            if l.is_only_nan() || r.is_only_nan() {
                return Type::Float64(FloatType::<64>::nan());
            }
            let maybe_nan = l.has_nan() || r.has_nan();
            let mut maybe_minuszero = true;
            let mut l = l;
            let mut r = r;
            if l.has_minus_zero() {
                l = FloatType::<Bits>::least_upper_bound(l, FloatType::<Bits>::constant(0.0), zone);
            } else {
                maybe_minuszero = false;
            }
            if r.has_minus
