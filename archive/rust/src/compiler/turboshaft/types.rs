// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::cmp::Ordering;
use std::fmt;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::{f32, f64};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::num::Wrapping;
use std::borrow::Cow;

//use crate::base::container_utils::*; // TODO: Implement equivalent functions
//use crate::base::export_template::*; // TODO: Implement if needed
//use crate::base::logging::*; // TODO: Implement logging macros
//use crate::base::small_vector::*; // TODO: Use smallvec crate?
//use crate::common::globals::*; // TODO: Implement globals if needed
//use crate::compiler::turboshaft::fast_hash::*; // TODO: Implement fast hash
//use crate::numbers::conversions::*; // TODO: Implement conversions if needed
//use crate::objects::turboshaft_types::*; // TODO: Implement turboshaft types if needed
//use crate::utils::ostreams::*; // TODO: Implement ostreams if needed
//use crate::zone::zone_containers::*; // TODO: Implement zone containers

// TODO: Implement tracing macros based on v8_flags.turboshaft_trace_typing

macro_rules! turboshaft_trace_typing {
    ($($arg:tt)*) => {
        // TODO: Implement tracing based on v8_flags.turboshaft_trace_typing
        //if v8_flags.turboshaft_trace_typing {
        //    println!($($arg)*);
        //}
    };
}

macro_rules! turboshaft_trace_typing_with_color {
    ($colorcode:expr, $str:expr, $($arg:tt)*) => {
        // TODO: Implement colored tracing based on v8_flags.turboshaft_trace_typing and v8_flags.log_colour
        //turboshaft_trace_typing!(if v8_flags.log_colour { format!("\x1B[{}m{}\x1B[0m", $colorcode, $str) } else { $str.to_string() }, $($arg)*);
    };
}

macro_rules! turboshaft_trace_typing_ok {
    ($str:expr, $($arg:tt)*) => {
        //turboshaft_trace_typing_with_color!("32", $str, $($arg)*);
    };
}

macro_rules! turboshaft_trace_typing_fail {
    ($str:expr, $($arg:tt)*) => {
        //turboshaft_trace_typing_with_color!("31", $str, $($arg)*);
    };
}

pub mod turboshaft {
    use super::*;

    pub(crate) mod detail {
        use super::*;
        use std::cmp::Ordering;

        pub(crate) fn is_unique_and_sorted<T: Ord>(container: &[T]) -> bool {
            if container.len() <= 1 {
                return true;
            }
            container.windows(2).all(|w| w[0] < w[1])
        }

        pub(crate) fn is_minus_zero<T: std::ops::Neg<Output = T> + PartialEq + Copy>(value: T) -> bool
            where f64: From<T> {
            f64::from(value).is_sign_negative() && value == -value
        }

        pub(crate) fn is_float_special_value<T: std::ops::Neg<Output = T> + PartialEq + Copy>(value: T) -> bool
            where f64: From<T> {
            f64::from(value).is_nan() || is_minus_zero(value)
        }

        pub(crate) trait TypeForBits {
            type UintType;
            type FloatType;
            const NAN: Self::FloatType;
        }

        pub(crate) struct TypeForBits32;
        impl TypeForBits for TypeForBits32 {
            type UintType = u32;
            type FloatType = f32;
            const NAN: Self::FloatType = f32::NAN;
        }

        pub(crate) struct TypeForBits64;
        impl TypeForBits for TypeForBits64 {
            type UintType = u64;
            type FloatType = f64;
            const NAN: Self::FloatType = f64::NAN;
        }

        // gcc versions < 9 may produce the following compilation error:
        // > '<anonymous>' is used uninitialized in this function
        // if Payload_Empty is initialized without any data, link to a relevant bug:
        // https://gcc.gnu.org/bugzilla/show_bug.cgi?id=86465
        // A workaround is to add a dummy value which is zero initialized by default.
        // More information as well as a sample reproducible code can be found at the
        // comment section of this CL crrev.com/c/4057111
        // TODO(nicohartmann@): Remove dummy once all platforms are using gcc >= 9.
        #[derive(Default, Copy, Clone)]
        #[repr(C)]
        pub(crate) struct PayloadEmpty {
            pub(crate) dummy: u8,
        }

        #[derive(Copy, Clone)]
        #[repr(C)]
        pub(crate) struct PayloadRange<T> {
            pub(crate) min: T,
            pub(crate) max: T,
        }

        #[derive(Copy, Clone)]
        #[repr(C)]
        pub(crate) struct PayloadInlineSet<T> {
            pub(crate) elements: [T; 2],
        }

        #[derive(Copy, Clone)]
        #[repr(C)]
        pub(crate) struct PayloadOutlineSet<T> {
            pub(crate) array: *mut T, // Using raw pointer for now; needs proper memory management
        }
    }

    fn next_smaller<T>(v: T) -> T
    where
        T: std::ops::Sub<Output = T> + PartialOrd + Copy,
        std::ops::Range<T>: Iterator<Item = T>,
        f64: From<T>
    {
        if num_traits::Float::is_nan(f64::from(v)) {
            panic!("v is NaN");
        }
        if f64::from(std::f64::NEG_INFINITY) >= f64::from(v) {
            panic!("v is less than negative infinity")
        }

        // TODO: Replace this with a direct translation of the C++ code for floating point types
        // For integral types, use v - 1
        // Use nextafter for floating types
        v - T::from(1).unwrap()
    }

    fn next_larger<T>(v: T) -> T
    where
        T: std::ops::Add<Output = T> + PartialOrd + Copy,
        std::ops::Range<T>: Iterator<Item = T>,
        f64: From<T>
    {
        if num_traits::Float::is_nan(f64::from(v)) {
            panic!("v is NaN");
        }
        if f64::from(v) >= f64::from(std::f64::INFINITY) {
            panic!("v is greater than infinity")
        }
        // TODO: Replace this with a direct translation of the C++ code for floating point types
        // For integral types, use v + 1
        // Use nextafter for floating types
        v + T::from(1).unwrap()
    }

    // TODO: Replace these with implementations that use the appropriate trait based on the type.
    // fn next_smaller<T>(v: T) -> T {
    //     v - T::one()
    // }
    //
    // fn next_larger<T>(v: T) -> T {
    //     v + T::one()
    // }

    pub(crate) type UintType32 = <detail::TypeForBits32 as detail::TypeForBits>::UintType;
    pub(crate) type FloatType32 = <detail::TypeForBits32 as detail::TypeForBits>::FloatType;
    pub(crate) const NAN_V32: FloatType32 = <detail::TypeForBits32 as detail::TypeForBits>::NAN;

    pub(crate) type UintType64 = <detail::TypeForBits64 as detail::TypeForBits>::UintType;
    pub(crate) type FloatType64 = <detail::TypeForBits64 as detail::TypeForBits>::FloatType;
    pub(crate) const NAN_V64: FloatType64 = <detail::TypeForBits64 as detail::TypeForBits>::NAN;

    pub struct Word32Type(WordType<32>);
    pub struct Word64Type(WordType<64>);
    pub struct Float32Type(FloatType<32>);
    pub struct Float64Type(FloatType<64>);

    /// Represents a type in the Turboshaft compiler.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct Type {
        kind: Kind,
        sub_kind: u8,
        set_size: u8,
        reserved: u8,
        bitfield: u32,
        payload: [u64; 2],
    }

    impl Type {
        /// Represents the kind of a type.
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        #[repr(u8)]
        pub enum Kind {
            Invalid,
            None,
            Word32,
            Word64,
            Float32,
            Float64,
            Tuple,
            Any,
        }

        /// Represents the resolution mode for type operations.
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub enum ResolutionMode {
            PreciseOrInvalid,
            OverApproximate,
            GreatestLowerBound,
        }

        /// Creates an invalid type.
        pub fn invalid() -> Self {
            Type {
                kind: Kind::Invalid,
                sub_kind: 0,
                set_size: 0,
                reserved: 0,
                bitfield: 0,
                payload: [0; 2],
            }
        }

        /// Creates a none type.
        pub fn none() -> Self {
            Type {
                kind: Kind::None,
                sub_kind: 0,
                set_size: 0,
                reserved: 0,
                bitfield: 0,
                payload: [0; 2],
            }
        }

        /// Creates an any type.
        pub fn any() -> Self {
            Type {
                kind: Kind::Any,
                sub_kind: 0,
                set_size: 0,
                reserved: 0,
                bitfield: 0,
                payload: [0; 2],
            }
        }

        /// Returns the kind of the type.
        pub fn kind(&self) -> Kind {
            self.kind
        }

        /// Checks if the type is invalid.
        pub fn is_invalid(&self) -> bool {
            self.kind == Kind::Invalid
        }

        /// Checks if the type is none.
        pub fn is_none(&self) -> bool {
            self.kind == Kind::None
        }

        /// Checks if the type is a word32.
        pub fn is_word32(&self) -> bool {
            self.kind == Kind::Word32
        }

        /// Checks if the type is a word64.
        pub fn is_word64(&self) -> bool {
            self.kind == Kind::Word64
        }

        /// Checks if the type is a float32.
        pub fn is_float32(&self) -> bool {
            self.kind == Kind::Float32
        }

        /// Checks if the type is a float64.
        pub fn is_float64(&self) -> bool {
            self.kind == Kind::Float64
        }

        /// Checks if the type is a tuple.
        pub fn is_tuple(&self) -> bool {
            self.kind == Kind::Tuple
        }

        /// Checks if the type is any.
        pub fn is_any(&self) -> bool {
            self.kind == Kind::Any
        }

        /// Checks if the type is a word of a given size.
        pub fn is_word<const B: usize>(&self) -> bool {
            match B {
                32 => self.is_word32(),
                64 => self.is_word64(),
                _ => panic!("B must be 32 or 64"),
            }
        }

        /// Checks if the type is a float of a given size.
        pub fn is_float<const B: usize>(&self) -> bool {
            match B {
                32 => self.is_float32(),
                64 => self.is_float64(),
                _ => panic!("B must be 32 or 64"),
            }
        }

        /// Casts the type to a Word32Type.
        pub fn as_word32(&self) -> &Word32Type {
            assert!(self.is_word32());
            unsafe { &*(self as *const Type as *const Word32Type) }
        }

        /// Casts the type to a Word64Type.
        pub fn as_word64(&self) -> &Word64Type {
            assert!(self.is_word64());
            unsafe { &*(self as *const Type as *const Word64Type) }
        }

        /// Casts the type to a Float32Type.
        pub fn as_float32(&self) -> &Float32Type {
            assert!(self.is_float32());
            unsafe { &*(self as *const Type as *const Float32Type) }
        }

        /// Casts the type to a Float64Type.
        pub fn as_float64(&self) -> &Float64Type {
            assert!(self.is_float64());
            unsafe { &*(self as *const Type as *const Float64Type) }
        }

        /// Casts the type to a TupleType.
        pub fn as_tuple(&self) -> &TupleType {
            assert!(self.is_tuple());
            unsafe { &*(self as *const Type as *const TupleType) }
        }

        /// Casts the type to a WordType of a given size.
        pub fn as_word<const B: usize>(&self) -> &WordType<{B}> {
             match B {
                32 => {
                    assert!(self.is_word32());
                    unsafe { &*(self as *const Type as *const WordType<32>) }
                }
                64 => {
                     assert!(self.is_word64());
                     unsafe { &*(self as *const Type as *const WordType<64>) }
                }
                 _ => panic!("B must be 32 or 64"),
             }
        }

        /// Casts the type to a FloatType of a given size.
        pub fn as_float<const B: usize>(&self) -> &FloatType<{B}> {
            match B {
                32 => {
                    assert!(self.is_float32());
                    unsafe { &*(self as *const Type as *const FloatType<32>) }
                }
                64 => {
                    assert!(self.is_float64());
                    unsafe { &*(self as *const Type as *const FloatType<64>) }
                }
                _ => panic!("B must be 32 or 64"),
            }
        }

        /// Checks if the type is equal to another type.
        pub fn equals(&self, other: &Self) -> bool {
             self.kind == other.kind &&
                self.sub_kind == other.sub_kind &&
                self.set_size == other.set_size &&
                self.reserved == other.reserved &&
                self.bitfield == other.bitfield &&
                self.payload == other.payload
        }

        /// Checks if the type is a subtype of another type.
        pub fn is_subtype_of(&self, other: &Self) -> bool {
            // TODO: Implement subtype checking logic
            // This is a placeholder
            self == other || other.is_any()
        }

        /// Prints the type to a stream.
        pub fn print_to(&self, stream: &mut dyn std::fmt::Write) -> std::fmt::Result {
            write!(stream, "{:?}", self)
        }

        /// Prints the type to stdout.
        pub fn print(&self) {
            println!("{}", self.to_string());
        }

        /// Returns a string representation of the type.
        pub fn to_string(&self) -> String {
            format!("{:?}", self)
        }

        /// Calculates the least upper bound of two types.
        pub fn least_upper_bound(lhs: &Type, rhs: &Type, zone: &mut Zone) -> Self {
            // TODO: Implement least upper bound logic. Need a Zone type.
            if lhs == rhs {
                *lhs
            } else {
                Type::any()
            }
        }

        /// Parses a type from a string.
        pub fn parse_from_string(str: &str, zone: &mut Zone) -> Option<Self> {
            // TODO: Implement parsing logic
            // Need a Zone type.
            None
        }

        /// Allocates the type on the heap.
        pub fn allocate_on_heap(&self, factory: &mut Factory) -> Handle<TurboshaftType> {
            // TODO: Implement allocation logic. Need Factory and TurboshaftType.
            panic!("Allocation on heap not implemented")
        }

        fn new<Payload>(kind: Kind, sub_kind: u8, set_size: u8, bitfield: u32, reserved: u8, payload: Payload) -> Self {
            let mut payload_bytes: [u8; 16] = [0; 16];
            let payload_size = std::mem::size_of::<Payload>();
            let payload_ptr = &payload as *const Payload as *const u8;

            unsafe {
                std::ptr::copy_nonoverlapping(payload_ptr, payload_bytes.as_mut_ptr(), payload_size);
            }

            if payload_size < 16 {
                for i in payload_size..16 {
                    payload_bytes[i] = 0x00;
                }
            }

            Type {
                kind,
                sub_kind,
                set_size,
                reserved,
                bitfield,
                payload: unsafe { std::mem::transmute(payload_bytes) },
            }
        }

        #[inline]
        fn get_payload<Payload>(&self) -> Payload
            where
                Payload: Copy,
        {
            unsafe {
                std::mem::transmute_copy(&self.payload)
            }
        }
    }

    impl Default for Type {
        fn default() -> Self {
            Type::invalid()
        }
    }

    impl fmt::Display for Type {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.print_to(f)
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct WordType<const Bits: usize> {
        inner: Type,
    }

    impl<const Bits: usize> WordType<Bits> {
        const MAX_INLINE_SET_SIZE: usize = 2;
        const KIND: Type::Kind = if Bits == 32 {
            Type::Kind::Word32
        } else if Bits == 64 {
            Type::Kind::Word64
        } else {
            panic!("Bits must be 32 or 64");
        };
        const MAX_SET_SIZE: usize = 8;

        type WordT =  match Bits {
            32 => UintType32,
            64 => UintType64,
            _ => panic!("Bits must be 32 or 64"),
        };

        type ValueType = Self::WordT;

        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        enum SubKind {
            Range,
            Set,
        }

        /// Creates an any word type.
        pub fn any(zone: &mut Zone) -> Self {
             Self::range(0, std::u64::MAX, zone)
        }

        /// Creates a range word type.
        pub fn range(from: u64, to: u64, zone: &mut Zone) -> Self {
            // Normalize ranges smaller than {kMaxSetSize} to sets.
            if to >= from {
                // (to - from + 1) <= kMaxSetSize
                if to - from <= Self::MAX_SET_SIZE as u64 - 1 {
                    // Normalizing non-wrapping ranges to a Set.
                    let mut elements: Vec<u64> = Vec::new();
                    let mut i = from;
                    while i < to {
                        elements.push(i);
                        i += 1;
                    }
                    elements.push(to);
                    return Self::set(&elements, zone);
                }
            } else {
                // (max - from + 1) + (to + 1) <= kMaxSetSize
                if (std::u64::MAX - from + to) <= Self::MAX_SET_SIZE as u64 - 2 {
                    // Normalizing wrapping ranges to a Set.
                    let mut elements: Vec<u64> = Vec::new();
                    let mut i = from;
                    while i < std::u64::MAX {
                        elements.push(i);
                        i += 1;
                    }
                    elements.push(std::u64::MAX);
                    let mut i = 0;
                    while i < to {
                        elements.push(i);
                        i += 1;
                    }
                    elements.push(to);
                    elements.sort();
                    return Self::set(&elements, zone);
                }
            }
             Self {
                inner: Type::new(
                    Self::KIND,
                    Self::SubKind::Range as u8,
                    0,
                    0,
                    0,
                    detail::PayloadRange { min: from, max: to },
                ),
            }
        }

        /// Creates a set word type.
        pub fn set(elements: &[u64], zone: &mut Zone) -> Self {
             assert!(detail::is_unique_and_sorted(elements));
             assert!(elements.len() > 0);
             assert!(elements.len() <= Self::MAX_SET_SIZE);

            if elements.len() <= Self::MAX_INLINE_SET_SIZE {
                // Use inline storage.
                let mut p = detail::PayloadInlineSet {
                    elements: [0; 2]
                };

                p.elements[0] = elements[0];
                if elements.len() > 1 {
                    p.elements[1] = elements[1];
                }

                 Self {
                    inner: Type::new(
                        Self::KIND,
                        Self::SubKind::Set as u8,
                        elements.len() as u8,
                        0,
                        0,
                        p,
                    ),
                }
            } else {
                // Allocate storage in the zone.

                // TODO: Implement Zone allocation
                //let p = detail::PayloadOutlineSet {
                //    array: zone.allocate_array::<Self::WordT>(elements.len()),
                //};
                //for i in 0..elements.len() {
                //    p.array[i] = elements[i];
                //}

                 Self {
                    inner: Type::new(
                        Self::KIND,
                        Self::SubKind::Set as u8,
                        elements.len() as u8,
                        0,
                        0,
                        detail::PayloadOutlineSet { array: std::ptr::null_mut() }
                    ),
                }
            }
        }

        /// Creates a constant word type.
        pub fn constant(constant: u64) -> Self {
            Self::set(&[constant], &mut Zone::default())
        }

        /// Checks if the word type is a range.
        pub fn is_range(&self) -> bool {
            self.sub_kind() == Self::SubKind::Range
        }

        /// Checks if the word type is a set.
        pub fn is_set(&self) -> bool {
            self.sub_kind() == Self::SubKind::Set
        }

        /// Checks if the word type is any.
        pub fn is_any(&self) -> bool {
            self.is_range() && self.range_to() + 1 == self.range_from()
        }

        /// Checks if the word type is constant.
        pub fn is_constant(&self) -> bool {
            assert_eq!(self.inner.set_size > 0, self.is_set());
            self.inner.set_size == 1
        }

        /// Checks if the word type is wrapping.
        pub fn is_wrapping(&self) -> bool {
            self.is_range() && self.range_from() > self.range_to()
        }

        /// Returns the range from value.
        pub fn range_from(&self) -> u64 {
            assert!(self.is_range());
            self.inner.get_payload::<detail::PayloadRange<u64>>().min
        }

        /// Returns the range to value.
        pub fn range_to(&self) -> u64 {
            assert!(self.is_range());
            self.inner.get_payload::<detail::PayloadRange<u64>>().max
        }

        /// Returns the range as a tuple.
        pub fn range(&self) -> (u64, u64) {
            assert!(self.is_range());
            (self.range_from(), self.range_to())
        }

        /// Returns the set size.
        pub fn set_size(&self) -> usize {
            assert!(self.is_set());
            self.inner.set_size as usize
        }

        /// Returns the set element at the given index.
        pub fn set_element(&self, index: usize) -> u64 {
            assert!(self.is_set());
            assert!(index >= 0);
            assert!(index < self.set_size());
            self.set_elements()[index]
        }

        /// Returns the set elements as a slice.
        pub fn set_elements(&self) -> Cow<[u64]> {
            assert!(self.is_set());
            if self.set_size() <= Self::MAX_INLINE_SET_SIZE {
                 let payload = self.inner.get_payload::<detail::PayloadInlineSet<u64>>();
                 Cow::Owned(payload.elements[..self.inner.set_size as usize].to_vec())
            } else {
                // TODO: Implement Zone allocation
                //let payload = self.inner.get_payload::<detail::PayloadOutlineSet<u64>>();
                //Cow::Borrowed(unsafe { std::slice::from_raw_parts(payload.array, self.set_size()) })
                Cow::Owned(vec![])
            }
        }

        /// Tries to get the constant value.
        pub fn try_get_constant(&self) -> Option<u64> {
            if !self.is_constant() {
                return None;
            }
            assert!(self.is_set());
            assert_eq!(self.set_size(), 1);
            Some(self.set_element(0))
        }

        /// Checks if the word type is constant with the given value.
        pub fn is_constant_value(&self, value: u64) -> bool {
            match self.try_get_constant() {
                Some(c) => c == value,
                None => false,
            }
        }

        /// Returns the unsigned minimum value.
        pub fn unsigned_min(&self) -> u64 {
            match self.sub_kind() {
                Self::SubKind::Range => {
                    if self.is_wrapping() {
                        0
                    } else {
                        self.range_from()
                    }
                }
                Self::SubKind::Set => self.set_element(0),
            }
        }

        /// Returns the unsigned maximum value.
        pub fn unsigned_max(&self) -> u64 {
            match self.sub_kind() {
                Self::SubKind::Range => {
                    if self.is_wrapping() {
                        std::u64::MAX
                    } else {
                        self.range_to()
                    }
                }
                Self::SubKind::Set => {
                    assert!(self.set_size() >= 1);
                    self.set_element(self.set_size() - 1)
                }
            }
        }

        /// Checks if the word type contains the given value.
        pub fn contains(&self, value: u64) -> bool {
            match self.sub_kind() {
                Self::SubKind::Range => {
                    if self.is_wrapping() {
                        value >= self.range_from() || value <= self.range_to()
                    } else {
                        value >= self.range_from() && value <= self.range_to()
                    }
                }
                Self::SubKind::Set => self.set_elements().contains(&value),
            }
        }

        /// Checks if the word type is equal to another word type.
        pub fn equals(&self, other: &Self) -> bool {
             self.inner.equals(&other.inner)
        }

        /// Checks if the word type is a subtype of another word type.
        pub fn is_subtype_of(&self, other: &Self) -> bool {
            // TODO: Implement subtype checking logic
            // This is a placeholder
            self.equals(other) || other.is_any()
        }

        /// Calculates the least upper bound of two word types.
        pub fn least_upper_bound(lhs: &Self, rhs: &Self, zone: &mut Zone) -> Self {
            // TODO: Implement least upper bound logic
            // This is a placeholder
            if lhs.equals(rhs) {
                *lhs
            } else {
                Self::any(zone)
            }
        }

        /// Calculates the intersection of two word types.
        pub fn intersect(lhs: &Self, rhs: &Self, resolution_mode: Type::ResolutionMode, zone: &mut Zone) -> Type {
            // TODO: Implement intersection logic
            // This is a placeholder
            if lhs.equals(rhs) {
                lhs.inner
            } else {
                Type::invalid()
            }
        }

        /// Prints the word type to a stream.
        pub fn print_to(&self, stream: &mut dyn fmt::Write) -> fmt::Result {
            write!(stream, "{:?}", self)
        }

        /// Allocates the word type on the heap.
        pub fn allocate_on_heap(&self, factory: &mut Factory) -> Handle<TurboshaftType> {
            // TODO: Implement allocation logic
            // Need Factory and TurboshaftType
            panic!("Allocation on heap not implemented")
        }

        fn sub_kind(&self) -> Self::SubKind {
            match self.inner.sub_kind {
                0 => Self::SubKind::Range,
                1 => Self::SubKind::Set,
                _ => panic!("Unknown subkind"),
            }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct FloatType<const Bits: usize> {
        inner: Type,
    }

    impl<const Bits: usize> FloatType<Bits> {
        const MAX_INLINE_SET_SIZE: usize = 2;
        const MAX_SET_SIZE: usize = 8;
        const KIND: Type::Kind = if Bits == 32 {
            Type::Kind::Float32
        } else if Bits == 64 {
            Type::Kind::Float64
        } else {
            panic!("Bits must be 32 or 64");
        };

        type FloatT =  match Bits {
            32 => FloatType32,
            64 => FloatType64,
            _ => panic!("Bits must be 32 or 64"),
        };

        type ValueType = Self::FloatT;

        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        enum SubKind {
            Range,
            Set,
            OnlySpecialValues,
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub enum Special {
            NoSpecialValues = 0x0,
            NaN = 0x1,
            MinusZero = 0x2,
        }

        /// Creates a float type with only special values.
        pub fn only_special_values(special_values: u32) -> Self {
            assert_ne!(0, special_values);
             Self {
                inner: Type::new(
                    Self::KIND,
                    Self::SubKind::OnlySpecialValues as u8,
                    0,
                    special_values,
                    0,
                    detail::PayloadEmpty::default(),
                ),
            }
        }

        /// Creates a float type with only NaN.
        pub fn nan() -> Self {
             Self {
                inner: Type::new(
                    Self::KIND,
                    Self::SubKind::OnlySpecialValues as u8,
                    0,
                    Self::Special::NaN as u32,
                    0,
                    detail::PayloadEmpty::default(),
                ),
            }
        }

        /// Creates a float type with only minus zero.
        pub fn minus_zero() -> Self {
             Self {
                inner: Type::new(
                    Self::KIND,
                    Self::SubKind::OnlySpecialValues as u8,
                    0,
                    Self::Special::MinusZero as u32,
                    0,
                    detail::PayloadEmpty::default(),
                ),
            }
        }

        /// Creates an any