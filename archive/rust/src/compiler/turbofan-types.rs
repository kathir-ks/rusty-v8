// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::{
    cmp::{max, min},
    f64::{INFINITY, NAN},
    fmt,
    mem::size_of,
};

// use crate::compiler::js_heap_broker::JSHeapBroker;  // Placeholder: Define JSHeapBroker if needed
// use crate::numbers::conversions_inl::IsUint32Double; // Placeholder: Implement if needed.
// use crate::objects::elements_kind::ElementsKind;    // Placeholder: Define ElementsKind enum
// use crate::objects::instance_type::InstanceType;    // Placeholder: Define InstanceType enum
// use crate::objects::turbofan_types::TurbofanType;   // Placeholder: Define TurbofanType if needed
// use crate::utils::ostreams::StdoutStream;           // Placeholder: Define StdoutStream if needed

macro_rules! proper_bitset_type_list {
    ($macro:ident) => {
        $macro!(None, 0);
        $macro!(Any, !0);
        $macro!(Hole, 1 << 0);
        $macro!(Undefined, 1 << 1);
        $macro!(Null, 1 << 2);
        $macro!(Boolean, 1 << 3);
        $macro!(String, 1 << 4);
        $macro!(Symbol, 1 << 5);
        $macro!(BigInt, 1 << 6);
        $macro!(Number, 1 << 7);
        $macro!(OtherNumber, 1 << 8);
        $macro!(NaN, 1 << 9);
        $macro!(MinusZero, 1 << 10);
        $macro!(PlainNumber, 1 << 11);
        $macro!(Signed32, 1 << 12);
        $macro!(Unsigned32, 1 << 13);
        $macro!(OtherObject, 1 << 14);
        $macro!(Array, 1 << 15);
        $macro!(StringWrapper, 1 << 16);
        $macro!(TypedArray, 1 << 17);
        $macro!(OtherCallable, 1 << 18);
        $macro!(OtherUndetectable, 1 << 19);
        $macro!(BoundFunction, 1 << 20);
        $macro!(CallableProxy, 1 << 21);
        $macro!(OtherProxy, 1 << 22);
        $macro!(CallableFunction, 1 << 23);
        $macro!(ClassConstructor, 1 << 24);
        $macro!(InternalizedString, 1 << 25);
    };
}

macro_rules! internal_bitset_type_list {
    ($macro:ident) => {
        $macro!(Negative32, 1 << 26);
        $macro!(Unsigned31, 1 << 27);
        $macro!(OtherSigned32, 1 << 28);
        $macro!(OtherUnsigned31, 1 << 29);
        $macro!(OtherUnsigned32, 1 << 30);
        $macro!(OtherInternal, 1 << 31);
        $macro!(WasmObject, 1 << 32);
    };
}

macro_rules! proper_atomic_bitset_type_low_list {
    ($macro:ident) => {
        $macro!(Signed31, 1 << 33);
        $macro!(Unsigned30, 1 << 34);
    };
}

macro_rules! proper_atomic_bitset_type_high_list {
    ($macro:ident) => {};
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Limits {
    min: f64,
    max: f64,
}

impl Limits {
    fn new(min: f64, max: f64) -> Self {
        Limits { min, max }
    }
    fn empty() -> Self {
        Limits {
            min: INFINITY,
            max: -INFINITY,
        }
    }
    fn is_empty(&self) -> bool {
        self.min > self.max
    }

    fn intersect(lhs: Limits, rhs: Limits) -> Limits {
        let mut result = lhs;
        result.min = f64::max(lhs.min, rhs.min);
        result.max = f64::min(lhs.max, rhs.max);
        result
    }

    fn union(lhs: Limits, rhs: Limits) -> Limits {
        if lhs.is_empty() {
            return rhs;
        }
        if rhs.is_empty() {
            return lhs;
        }
        let mut result = lhs;
        result.min = f64::min(lhs.min, rhs.min);
        result.max = f64::max(lhs.max, rhs.max);
        result
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RangeType {
    limits: Limits,
}

impl RangeType {
    fn new(min: f64, max: f64) -> Self {
        RangeType {
            limits: Limits::new(min, max),
        }
    }
    fn new_from_limits(limits: Limits) -> Self {
        RangeType { limits }
    }

    fn min(&self) -> f64 {
        self.limits.min
    }

    fn max(&self) -> f64 {
        self.limits.max
    }

    fn lub(&self) -> BitsetType {
        BitsetType::Glb(self.min(), self.max())
    }

    fn is_integer(value: f64) -> bool {
        value.fract() == 0.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BitsetType(u64);

impl BitsetType {
    const K_NONE: u64 = 0;
    const K_ANY: u64 = !0;
    const K_HOLE: u64 = 1 << 0;
    const K_UNDEFINED: u64 = 1 << 1;
    const K_NULL: u64 = 1 << 2;
    const K_BOOLEAN: u64 = 1 << 3;
    const K_STRING: u64 = 1 << 4;
    const K_SYMBOL: u64 = 1 << 5;
    const K_BIGINT: u64 = 1 << 6;
    const K_NUMBER: u64 = 1 << 7;
    const K_OTHER_NUMBER: u64 = 1 << 8;
    const K_NAN: u64 = 1 << 9;
    const K_MINUS_ZERO: u64 = 1 << 10;
    const K_PLAIN_NUMBER: u64 = 1 << 11;
    const K_SIGNED32: u64 = 1 << 12;
    const K_UNSIGNED32: u64 = 1 << 13;
    const K_OTHER_OBJECT: u64 = 1 << 14;
    const K_ARRAY: u64 = 1 << 15;
    const K_STRING_WRAPPER: u64 = 1 << 16;
    const K_TYPED_ARRAY: u64 = 1 << 17;
    const K_OTHER_CALLABLE: u64 = 1 << 18;
    const K_OTHER_UNDETECTABLE: u64 = 1 << 19;
    const K_BOUND_FUNCTION: u64 = 1 << 20;
    const K_CALLABLE_PROXY: u64 = 1 << 21;
    const K_OTHER_PROXY: u64 = 1 << 22;
    const K_CALLABLE_FUNCTION: u64 = 1 << 23;
    const K_CLASS_CONSTRUCTOR: u64 = 1 << 24;
    const K_INTERNALIZED_STRING: u64 = 1 << 25;
    const K_NEGATIVE32: u64 = 1 << 26;
    const K_UNSIGNED31: u64 = 1 << 27;
    const K_OTHER_SIGNED32: u64 = 1 << 28;
    const K_OTHER_UNSIGNED31: u64 = 1 << 29;
    const K_OTHER_UNSIGNED32: u64 = 1 << 30;
    const K_OTHER_INTERNAL: u64 = 1 << 31;
    const K_WASM_OBJECT: u64 = 1 << 32;
    const K_SIGNED31: u64 = 1 << 33;
    const K_UNSIGNED30: u64 = 1 << 34;

    fn new(bits: u64) -> Self {
        BitsetType(bits)
    }

    fn is(&self, other: BitsetType) -> bool {
        (self.0 & other.0) == other.0
    }

    fn is_none(&self) -> bool {
        self.0 == 0
    }

    fn expand_internals(&self) -> BitsetType {
        let mut bits = self.0;
        if (bits & BitsetType::K_OTHER_STRING) != 0 {
            bits |= BitsetType::K_STRING;
        }

        if (bits & BitsetType::K_PLAIN_NUMBER) == 0 {
            return BitsetType(bits);
        }

        for boundary in Self::Boundaries() {
            if (bits & boundary.internal) != 0 {
                bits |= boundary.external;
            }
        }
        BitsetType(bits)
    }

    fn lub(min: f64, max: f64) -> BitsetType {
        let mut lub = 0;
        let mins = Self::Boundaries();

        for i in 1..Self::BoundariesSize() {
            if min < mins[i].min {
                lub |= mins[i - 1].internal;
                if max < mins[i].min {
                    return BitsetType(lub);
                }
            }
        }
        BitsetType(lub | mins[Self::BoundariesSize() - 1].internal)
    }

    fn number_bits(&self) -> BitsetType {
        BitsetType(self.0 & Self::K_PLAIN_NUMBER)
    }

    fn glb(min: f64, max: f64) -> BitsetType {
        let mut glb = 0;
        let mins = Self::Boundaries();

        if max < -1.0 || min > 0.0 {
            return BitsetType(glb);
        }

        for i in 1..(Self::BoundariesSize() - 1) {
            if min <= mins[i].min {
                if max + 1.0 < mins[i + 1].min {
                    break;
                }
                glb |= mins[i].external;
            }
        }
        BitsetType(glb & !Self::K_OTHER_NUMBER)
    }

    fn min(bits: BitsetType) -> f64 {
        if !Type::new_bitset(bits).is_number() {
            panic!("Type is not a Number");
        }
        if Type::new_bitset(bits).is_nan() {
            panic!("Type is NaN");
        }

        let mins = Self::Boundaries();
        let mz = (bits.0 & Self::K_MINUS_ZERO) != 0;

        for i in 0..Self::BoundariesSize() {
            if (bits.0 & mins[i].internal) != 0 {
                return if mz {
                    min(0.0, mins[i].min)
                } else {
                    mins[i].min
                };
            }
        }
        assert!(mz);
        0.0
    }

    fn max(bits: BitsetType) -> f64 {
        if !Type::new_bitset(bits).is_number() {
            panic!("Type is not a Number");
        }
        if Type::new_bitset(bits).is_nan() {
            panic!("Type is NaN");
        }

        let mins = Self::Boundaries();
        let mz = (bits.0 & Self::K_MINUS_ZERO) != 0;

        if (bits.0 & mins[Self::BoundariesSize() - 1].internal) != 0 {
            return INFINITY;
        }

        for i in (0..(Self::BoundariesSize() - 1)).rev() {
            if (bits.0 & mins[i].internal) != 0 {
                return if mz {
                    max(0.0, mins[i + 1].min - 1.0)
                } else {
                    mins[i + 1].min - 1.0
                };
            }
        }
        assert!(mz);
        0.0
    }

    const fn boundaries() -> &'static [Boundary] {
        &[
            Boundary {
                internal: Self::K_OTHER_NUMBER,
                external: Self::K_PLAIN_NUMBER,
                min: -INFINITY,
            },
            Boundary {
                internal: Self::K_OTHER_SIGNED32,
                external: Self::K_NEGATIVE32,
                min: i32::MIN as f64,
            },
            Boundary {
                internal: Self::K_NEGATIVE31,
                external: Self::K_NEGATIVE31,
                min: -0x40000000_f64,
            },
            Boundary {
                internal: Self::K_UNSIGNED30,
                external: Self::K_UNSIGNED30,
                min: 0.0,
            },
            Boundary {
                internal: Self::K_OTHER_UNSIGNED31,
                external: Self::K_UNSIGNED31,
                min: 0x40000000_f64,
            },
            Boundary {
                internal: Self::K_OTHER_UNSIGNED32,
                external: Self::K_UNSIGNED32,
                min: 0x80000000_f64,
            },
            Boundary {
                internal: Self::K_OTHER_NUMBER,
                external: Self::K_PLAIN_NUMBER,
                min: (u32::MAX as f64) + 1.0,
            },
        ]
    }

    const fn boundaries_size() -> usize {
        7
    }

    fn signed_small() -> Self {
        if Self::smi_values_are_31_bits() {
            BitsetType(Self::K_SIGNED31)
        } else {
            BitsetType(Self::K_SIGNED32)
        }
    }

    fn unsigned_small() -> Self {
        if Self::smi_values_are_31_bits() {
            BitsetType(Self::K_UNSIGNED30)
        } else {
            BitsetType(Self::K_UNSIGNED31)
        }
    }

    const fn smi_values_are_31_bits() -> bool {
        size_of::<usize>() == 4
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Boundary {
    internal: u64,
    external: u64,
    min: f64,
}

impl fmt::Display for BitsetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = Self::name(self.0);
        if let Some(name) = name {
            return write!(f, "{}", name);
        }

        let named_bitsets = [
            BitsetType::K_NEGATIVE32,
            BitsetType::K_UNSIGNED31,
            BitsetType::K_OTHER_SIGNED32,
            BitsetType::K_OTHER_UNSIGNED31,
            BitsetType::K_OTHER_UNSIGNED32,
            BitsetType::K_OTHER_INTERNAL,
            BitsetType::K_WASM_OBJECT,
            BitsetType::K_NONE,
            BitsetType::K_ANY,
            BitsetType::K_HOLE,
            BitsetType::K_UNDEFINED,
            BitsetType::K_NULL,
            BitsetType::K_BOOLEAN,
            BitsetType::K_STRING,
            BitsetType::K_SYMBOL,
            BitsetType::K_BIGINT,
            BitsetType::K_NUMBER,
            BitsetType::K_OTHER_NUMBER,
            BitsetType::K_NAN,
            BitsetType::K_MINUS_ZERO,
            BitsetType::K_PLAIN_NUMBER,
            BitsetType::K_SIGNED32,
            BitsetType::K_UNSIGNED32,
            BitsetType::K_OTHER_OBJECT,
            BitsetType::K_ARRAY,
            BitsetType::K_STRING_WRAPPER,
            BitsetType::K_TYPED_ARRAY,
            BitsetType::K_OTHER_CALLABLE,
            BitsetType::K_OTHER_UNDETECTABLE,
            BitsetType::K_BOUND_FUNCTION,
            BitsetType::K_CALLABLE_PROXY,
            BitsetType::K_OTHER_PROXY,
            BitsetType::K_CALLABLE_FUNCTION,
            BitsetType::K_CLASS_CONSTRUCTOR,
            BitsetType::K_INTERNALIZED_STRING,
            BitsetType::K_SIGNED31,
            BitsetType::K_UNSIGNED30,
        ];

        let mut bits = self.0;
        let mut is_first = true;
        write!(f, "(")?;
        for i in (0..named_bitsets.len()).rev() {
            let subset = named_bitsets[i];
            if (bits & subset) == subset {
                if !is_first {
                    write!(f, " | ")?;
                }
                is_first = false;
                if let Some(name) = Self::name(subset) {
                    write!(f, "{}", name)?;
                }
                bits -= subset;
            }
        }
        write!(f, ")")
    }
}

impl BitsetType {
    fn name(bits: u64) -> Option<&'static str> {
        match bits {
            0 => Some("None"),
            !0 => Some("Any"),
            BitsetType::K_HOLE => Some("Hole"),
            BitsetType::K_UNDEFINED => Some("Undefined"),
            BitsetType::K_NULL => Some("Null"),
            BitsetType::K_BOOLEAN => Some("Boolean"),
            BitsetType::K_STRING => Some("String"),
            BitsetType::K_SYMBOL => Some("Symbol"),
            BitsetType::K_BIGINT => Some("BigInt"),
            BitsetType::K_NUMBER => Some("Number"),
            BitsetType::K_OTHER_NUMBER => Some("OtherNumber"),
            BitsetType::K_NAN => Some("NaN"),
            BitsetType::K_MINUS_ZERO => Some("MinusZero"),
            BitsetType::K_PLAIN_NUMBER => Some("PlainNumber"),
            BitsetType::K_SIGNED32 => Some("Signed32"),
            BitsetType::K_UNSIGNED32 => Some("Unsigned32"),
            BitsetType::K_OTHER_OBJECT => Some("OtherObject"),
            BitsetType::K_ARRAY => Some("Array"),
            BitsetType::K_STRING_WRAPPER => Some("StringWrapper"),
            BitsetType::K_TYPED_ARRAY => Some("TypedArray"),
            BitsetType::K_OTHER_CALLABLE => Some("OtherCallable"),
            BitsetType::K_OTHER_UNDETECTABLE => Some("OtherUndetectable"),
            BitsetType::K_BOUND_FUNCTION => Some("BoundFunction"),
            BitsetType::K_CALLABLE_PROXY => Some("CallableProxy"),
            BitsetType::K_OTHER_PROXY => Some("OtherProxy"),
            BitsetType::K_CALLABLE_FUNCTION => Some("CallableFunction"),
            BitsetType::K_CLASS_CONSTRUCTOR => Some("ClassConstructor"),
            BitsetType::K_INTERNALIZED_STRING => Some("InternalizedString"),
            BitsetType::K_NEGATIVE32 => Some("Negative32"),
            BitsetType::K_UNSIGNED31 => Some("Unsigned31"),
            BitsetType::K_OTHER_SIGNED32 => Some("OtherSigned32"),
            BitsetType::K_OTHER_UNSIGNED31 => Some("OtherUnsigned31"),
            BitsetType::K_OTHER_UNSIGNED32 => Some("OtherUnsigned32"),
            BitsetType::K_OTHER_INTERNAL => Some("OtherInternal"),
            BitsetType::K_WASM_OBJECT => Some("WasmObject"),
            BitsetType::K_SIGNED31 => Some("Signed31"),
            BitsetType::K_UNSIGNED30 => Some("Unsigned30"),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HeapConstantType {
    bitset: BitsetType,
    // heap_ref: HeapObjectRef, // Placeholder: Define HeapObjectRef
    value: usize, // address of heap object
}

impl HeapConstantType {
    fn new(bitset: BitsetType, value: usize) -> Self {
        HeapConstantType { bitset, value }
    }

    fn value(&self) -> usize {
        self.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OtherNumberConstantType {
    value: f64,
}

impl OtherNumberConstantType {
    fn new(value: f64) -> Self {
        OtherNumberConstantType { value }
    }

    fn value(&self) -> f64 {
        self.value
    }

    fn is_other_number_constant(value: f64) -> bool {
        !value.is_nan() && !RangeType::is_integer(value) && !is_minus_zero(value)
    }

    fn lub(&self) -> BitsetType {
        // Placeholder: Implement lub logic for other number constant.
        BitsetType(0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TupleType {
    elements: Vec<Type>,
}

impl TupleType {
    fn new(arity: usize) -> Self {
        TupleType {
            elements: Vec::with_capacity(arity),
        }
    }

    fn init_element(&mut self, index: usize, element: Type) {
        if self.elements.len() <= index {
            self.elements.resize(index + 1, Type::None()); // Replace Type::None with appropriate default if needed.
        }
        self.elements[index] = element;
    }

    fn arity(&self) -> usize {
        self.elements.len()
    }

    fn element(&self, index: usize) -> Type {
        self.elements[index]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnionType {
    types: Vec<Type>,
}

impl UnionType {
    fn new(capacity: usize) -> Self {
        UnionType {
            types: Vec::with_capacity(capacity),
        }
    }

    fn get(&self, index: usize) -> Type {
        self.types[index].clone()
    }

    fn set(&mut self, index: usize, value: Type) {
        if self.types.len() <= index {
            self.types.resize(index + 1, Type::None());  // Replace Type::None with appropriate default if needed.
        }
        self.types[index] = value;
    }

    fn length(&self) -> usize {
        self.types.len()
    }

    fn shrink(&mut self, new_size: usize) {
        self.types.truncate(new_size);
    }

    fn wellformed(&self) -> bool {
        if self.types.len() < 2 {
            return false;
        }
        if !self.types[0].is_bitset() {
            return false;
        }

        for i in 0..self.types.len() {
            if i != 0 && self.types[i].is_bitset() {
                return false;
            }
            if i != 1 && self.types[i].is_range() {
                return false;
            }
            if self.types[i].is_union() {
                return false;
            }
            for j in 0..self.types.len() {
                if i != j && i != 0 && self.types[i].is(self.types[j].clone()) {
                    return false;
                }
            }
        }

        if self.types[1].is_range()
            && (self.types[0].as_bitset().number_bits().0 != BitsetType::K_NONE)
        {
            return false;
        }
        true
    }
}

// #[cfg(V8_ENABLE_WEBASSEMBLY)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WasmType {
    value_type: u32,             // wasm::ValueType, Placeholder: Define wasm::ValueType
    module: *const u8, // *const wasm::WasmModule, Placeholder: Define wasm::WasmModule
}

// #[cfg(V8_ENABLE_WEBASSEMBLY)]
impl WasmType {
    fn new(value_type: u32, module: *const u8) -> Self {
        WasmType { value_type, module }
    }

    fn value_type(&self) -> u32 {
        self.value_type
    }

    fn module(&self) -> *const u8 {
        self.module
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeBase {
    Bitset(BitsetType),
    HeapConstant(HeapConstantType),
    OtherNumberConstant(OtherNumberConstantType),
    Range(RangeType),
    Tuple(TupleType),
    Union(UnionType),
    Wasm(WasmType),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    base: TypeBase,
}

impl Type {
    fn new_bitset(bits: BitsetType) -> Self {
        Type {
            base: TypeBase::Bitset(bits),
        }
    }

    fn new_heap_constant(heap_constant: HeapConstantType) -> Self {
        Type {
            base: TypeBase::HeapConstant(heap_constant),
        }
    }

    fn new_other_number_constant(other_number_constant: OtherNumberConstantType) -> Self {
        Type {
            base: TypeBase::OtherNumberConstant(other_number_constant),
        }
    }

    fn new_range(range: RangeType) -> Self {
        Type {
            base: TypeBase::Range(range),
        }
    }

    fn new_tuple(tuple: TupleType) -> Self {
        Type {
            base: TypeBase::Tuple(tuple),
        }
    }

    fn new_union(union: UnionType) -> Self {
        Type {
            base: TypeBase::Union(union),
        }
    }

    fn new_wasm(wasm: WasmType) -> Self {
        Type { base: TypeBase::Wasm(wasm) }
    }

    fn none() -> Self {
        Type::new_bitset(BitsetType(0))
    }

    fn any() -> Self {
        Type::new_bitset(BitsetType(!0))
    }

    fn hole() -> Self {
        Type::new_bitset(BitsetType(BitsetType::K_HOLE))
    }

    fn undefined() -> Self {
        Type::new_bitset(BitsetType(BitsetType::K_UNDEFINED))
    }

    fn null_type() -> Self {
        Type::new_bitset(BitsetType(BitsetType::K_NULL))
    }

    fn boolean() -> Self {
        Type::new_bitset(BitsetType(BitsetType::K_BOOLEAN))
    }

    fn string() -> Self {
        Type::new_bitset(BitsetType(BitsetType::K_STRING))
    }

    fn symbol() -> Self {
        Type::new_bitset(BitsetType(BitsetType::K_SYMBOL))
    }

    fn bigint() -> Self {
        Type::new_bitset(BitsetType(BitsetType::K_BIGINT))
    }

    fn number() -> Self {
        Type::new_bitset(BitsetType(BitsetType::K_NUMBER))
    }

    fn other_number() -> Self {
        Type::new_bitset(BitsetType(BitsetType::K_OTHER_NUMBER))
    }

    fn nan() -> Self {
        Type::new_bitset(BitsetType(BitsetType::K_NAN))
    }

    fn minus_zero() -> Self {
        Type::new_bitset(BitsetType(BitsetType::K_MINUS_ZERO))
    }

    fn plain_number() -> Self {
        Type::new_bitset(BitsetType(BitsetType::K_PLAIN_NUMBER))
    }

    fn signed32() -> Self {
        Type::new_bitset(BitsetType(BitsetType::K_SIGNED32))
    }

    fn unsigned32() -> Self {
        Type::new_bitset(BitsetType(BitsetType::K_UNSIGNED32))
    }

    fn other_object() -> Self {
        Type::new_bitset(BitsetType(BitsetType::K_OTHER_OBJECT))
    }

    fn array() -> Self {
        Type::new_bitset(BitsetType(BitsetType::K_ARRAY))
    }

    fn string_wrapper() -> Self {
        Type::new_bitset(BitsetType(BitsetType::K_STRING_WRAPPER))
    }

    fn typed_array() -> Self {
        Type::new_bitset(BitsetType(BitsetType::K_TYPED_ARRAY))
    }

    fn other_callable() -> Self {
        Type::new_bitset(BitsetType(BitsetType::K_OTHER_CALLABLE))
    }

    fn other_undetectable() -> Self {
        Type::new_bitset(BitsetType(BitsetType::K_OTHER_UNDETECTABLE))
    }

    fn bound_function() -> Self {
        Type::new_bitset(BitsetType(BitsetType::K_BOUND_FUNCTION))
    }

    fn callable_proxy() -> Self {
        Type::new_bitset(BitsetType(BitsetType::K_CALLABLE_PROXY))
    }

    fn other_proxy() -> Self {
        Type::new_bitset(BitsetType(BitsetType::K_OTHER_PROXY))
    }

    fn callable_function() -> Self {
        Type::new_bitset(BitsetType(BitsetType::K_CALLABLE_FUNCTION))
    }

    fn class_constructor() -> Self {
        Type::new_bitset(BitsetType(BitsetType::K_CLASS_CONSTRUCTOR))
    }

    fn internalized_string() -> Self {
        Type::new_bitset(BitsetType(BitsetType::K_INTERNALIZED_STRING))
    }

    fn negative32() -> Self {
        Type::new_bitset(BitsetType(BitsetType::K_NEGATIVE32))
    }

    fn unsigned31() -> Self {
        Type::new_bitset(BitsetType(BitsetType::K_UNSIGNED31))
    }

    fn other_signed32() -> Self {
        Type::new_bitset(BitsetType(BitsetType::K_OTHER_SIGNED32))
    }

    fn other_unsigned31() -> Self {
        Type::new_bitset(BitsetType(BitsetType::K_OTHER_UNSIGNED31))
    }

    fn other_unsigned32() -> Self {
        Type::new_bitset(BitsetType(BitsetType::K_OTHER_UNSIGNED32))
    }

    fn other_internal() -> Self {
        Type::new_bitset(BitsetType(BitsetType::K_OTHER_INTERNAL))
    }

    fn wasm_object() -> Self {
        Type::new_bitset(BitsetType(BitsetType::K_WASM_OBJECT))
    }

    fn signed31() -> Self {
        Type::new_bitset(BitsetType(BitsetType::K_SIGNED31))
    }

    fn unsigned30() -> Self {
        Type::new_bitset(BitsetType(BitsetType::K_UNSIGNED30))
    }

    fn is_bitset(&self) -> bool {
        matches!(self.base, TypeBase::Bitset(_))
    }

    fn is_heap_constant(&self) -> bool {
        matches!(self.base, TypeBase::HeapConstant(_))
    }

    fn is_other_number_constant(&self) -> bool {
        matches!(self.base, TypeBase::OtherNumberConstant(_))
    }

    fn is_range(&self) -> bool {
        matches!(self.base, TypeBase::Range(_))
    }

    fn is_tuple(&self) -> bool {
        matches!(self.base, TypeBase::Tuple(_))
    }

    fn is_union(&self) -> bool {
        matches!(self.base, TypeBase::Union(_))
    }

    fn is_wasm(&self) -> bool {
        matches!(self.base, TypeBase::Wasm(_))
    }

    fn as_bitset(&self) -> BitsetType {
        if let TypeBase::Bitset(bitset) = &self.base {
            *bitset
        } else {
            panic!("Type is not a Bitset");
        }
    }

    fn as_heap_constant(&self) -> &HeapConstantType {
        if let TypeBase::HeapConstant(heap_constant) = &self.base {
            heap_constant
        } else {
            panic!("Type is not a HeapConstant");
        }
    }

    fn as_other_number_constant(&self) -> &OtherNumberConstantType {
        if let TypeBase::OtherNumberConstant(other_number_constant) = &self.base {
            other_number_constant
        } else {
            panic!("Type is not an OtherNumberConstant");
        }
    }

    fn as_range(&self) -> &Range