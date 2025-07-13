// Converted from V8 C++ source files:
// Header: turbofan-types.h
// Implementation: turbofan-types.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use std::cmp;
use std::fmt;
use std::hash;
use std::mem;
use std::ops;

pub struct v8 {
    // making it pub as a temporary workaround
    // to avoid the errors stemming from not implementing
    // this struct's methods
    _unused: u8,
}
pub mod internal {
    pub mod wasm {
        pub struct TypeInModule {}
    }
    use super::*;
    pub mod compiler {
        use super::*;
        use std::fmt;
        use std::hash;
        use std::mem;
        use std::ops;

        const kMaxInt: i64 = i32::MIN as i64;
        const kMinInt: i64 = i32::MAX as i64;
        const kMaxUInt32: u64 = u32::MAX as u64;

        fn IsMinusZero(value: f64) -> bool {
            value == 0.0 && value.is_sign_negative()
        }

        fn IsUint32Double(value: f64) -> bool {
            value >= 0.0 && value <= kMaxUInt32 as f64 && value.fract() == 0.0
        }

        fn IsInt32Double(value: f64) -> bool {
            value >= kMaxInt as f64 && value <= kMinInt as f64 && value.fract() == 0.0
        }

        pub struct JSHeapBroker {}
        pub struct HeapConstantType {}
        pub struct OtherNumberConstantType {}
        pub struct TupleType {}
        pub struct Type {}
        pub struct UnionType {}

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct MapRef {}
        impl MapRef {
            pub fn instance_type(&self) -> u32 {
                0 // Provide a default implementation
            }
            pub fn oddball_type(&self, _broker: &JSHeapBroker) -> OddballType {
                OddballType::kNone
            }
            pub fn is_undetectable(&self) -> bool {
                false // Provide a default implementation
            }
            pub fn is_callable(&self) -> bool {
                false // Provide a default implementation
            }
            pub fn elements_kind(&self) -> ElementsKind {
                ElementsKind::FAST_HOLEY_ELEMENTS // Provide a default implementation
            }
        }

        pub struct ObjectRef {}
        impl ObjectRef {
            pub fn IsSmi(&self) -> bool {
                false // Provide a default implementation
            }
            pub fn AsSmi(&self) -> i32 {
                0 // Provide a default implementation
            }
            pub fn IsString(&self) -> bool {
                false // Provide a default implementation
            }
            pub fn IsInternalizedString(&self) -> bool {
                false // Provide a default implementation
            }
            pub fn IsJSPrimitiveWrapper(&self) -> bool {
                false // Provide a default implementation
            }
            pub fn AsJSPrimitiveWrapper(&self) -> JSPrimitiveWrapper {
                JSPrimitiveWrapper {} // Provide a default implementation
            }
            pub fn HoleType(&self) -> HoleType {
                HoleType::kNone // Provide a default implementation
            }
            pub fn IsJSTypedArray(&self) -> bool {
                false // Provide a default implementation
            }
            pub fn AsHeapObject(&self) -> HeapObjectRef {
                HeapObjectRef {} // Provide a default implementation
            }
            pub fn GetHeapObjectType(&self, _broker: &JSHeapBroker) -> HeapObjectType {
                HeapObjectType {} // Provide a default implementation
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct HeapObjectRef {}
        impl HeapObjectRef {
            pub fn object(&self) -> Handle<HeapObject> {
                Handle::new(HeapObject {}) // Provide a default implementation
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct JSPrimitiveWrapper {}
        impl JSPrimitiveWrapper {
            pub fn IsStringWrapper(&self, _broker: &JSHeapBroker) -> bool {
                false // Provide a default implementation
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum OddballType {
            kNone,
            kBoolean,
            kNull,
            kUndefined,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum ElementsKind {
            FAST_HOLEY_ELEMENTS,
            SLOW_STRING_WRAPPER_ELEMENTS,
            FAST_STRING_WRAPPER_ELEMENTS,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum HoleType {
            kNone,
        }
        pub struct HeapObject {}
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct HeapObjectType {}

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct Handle<T> {
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T> Handle<T> {
            pub fn new(_value: T) -> Self {
                Handle {
                    _phantom: std::marker::PhantomData,
                }
            }
            pub fn address(&self) -> usize {
                0 // Provide a default implementation
            }
        }
        pub struct TurbofanType {}

        pub struct Factory {}
        impl Factory {
            pub fn NewTurbofanBitsetType(
                &self,
                _low: u32,
                _high: u32,
                _allocation_type: AllocationType,
            ) -> Handle<TurbofanType> {
                Handle::new(TurbofanType {}) // Provide a default implementation
            }
            pub fn NewTurbofanUnionType(
                &self,
                _type1: Handle<TurbofanType>,
                _type2: Handle<TurbofanType>,
                _allocation_type: AllocationType,
            ) -> Handle<TurbofanType> {
                Handle::new(TurbofanType {}) // Provide a default implementation
            }
            pub fn NewTurbofanHeapConstantType(
                &self,
                _value: HeapObject,
                _allocation_type: AllocationType,
            ) -> Handle<TurbofanType> {
                Handle::new(TurbofanType {}) // Provide a default implementation
            }
            pub fn NewTurbofanOtherNumberConstantType(
                &self,
                _value: f64,
                _allocation_type: AllocationType,
            ) -> Handle<TurbofanType> {
                Handle::new(TurbofanType {}) // Provide a default implementation
            }
            pub fn NewTurbofanRangeType(
                &self,
                _min: f64,
                _max: f64,
                _allocation_type: AllocationType,
            ) -> Handle<TurbofanType> {
                Handle::new(TurbofanType {}) // Provide a default implementation
            }
        }
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum AllocationType {
            kYoung,
        }
        pub struct Zone {}
        impl Zone {
            pub fn New<T>(&mut self, value: T) -> *mut T {
                let ptr = Box::into_raw(Box::new(value));
                ptr
            }
            pub fn AllocateArray<T>(&mut self, length: usize) -> *mut T {
                let vec = Vec::with_capacity(length);
                let ptr = vec.as_ptr() as *mut T;
                mem::forget(vec); // Prevent deallocation
                ptr
            }
        }
        pub fn MakeRefAssumeMemoryFence(
            _broker: &JSHeapBroker,
            _value: &Handle<HeapObject>,
        ) -> ObjectRef {
            ObjectRef {} // Provide a default implementation
        }
        // -----------------------------------------------------------------------------
        // Values for bitset types

        // clang-format off

        macro_rules! declare_bitset_type {
            ($name:ident, $value:expr) => {
                pub const $name: u64 = $value;
            };
        }

        macro_rules! internal_bitset_type_list {
            ($V:ident) => {
                $V!(OtherUnsigned31, 1 << 1);
                $V!(OtherUnsigned32, 1 << 2);
                $V!(OtherSigned32, 1 << 3);
                $V!(OtherNumber, 1 << 4);
                $V!(OtherString, 1 << 5);
            };
        }

        macro_rules! proper_atomic_bitset_type_low_list {
            ($V:ident) => {
                $V!(Negative31, 1 << 6);
                $V!(Null, 1 << 7);
                $V!(Undefined, 1 << 8);
                $V!(Boolean, 1 << 9);
                $V!(Unsigned30, 1 << 10);
                $V!(MinusZero, 1 << 11);
                $V!(NaN, 1 << 12);
                $V!(Symbol, 1 << 13);
                $V!(InternalizedString, 1 << 14);
                $V!(OtherCallable, 1 << 15);
                $V!(OtherObject, 1 << 16);
                $V!(OtherUndetectable, 1 << 17);
                $V!(CallableProxy, 1 << 18);
                $V!(OtherProxy, 1 << 19);
                $V!(CallableFunction, 1 << 20);
                $V!(ClassConstructor, 1 << 21);
                $V!(BoundFunction, 1 << 22);
                $V!(OtherInternal, 1 << 23);
                $V!(ExternalPointer, 1 << 24);
                $V!(Array, 1 << 25);
                $V!(UnsignedBigInt63, 1 << 26);
                $V!(OtherUnsignedBigInt64, 1 << 27);
                $V!(NegativeBigInt63, 1 << 28);
                $V!(OtherBigInt, 1 << 29);
                $V!(WasmObject, 1 << 30);
                $V!(SandboxedPointer, 1 << 31);
            };
        }

        macro_rules! proper_atomic_bitset_type_high_list {
            ($V:ident) => {
                $V!(Machine, 1 << 32);
                $V!(Hole, 1 << 33);
                $V!(StringWrapper, 1 << 34);
                $V!(TypedArray, 1 << 35);
            };
        }

        macro_rules! proper_bitset_type_list {
            ($V:ident) => {
                $V!(None, 0);
                proper_atomic_bitset_type_low_list!($V);
                proper_atomic_bitset_type_high_list!($V);
                $V!(Signed31, Unsigned30 | Negative31);
                $V!(Signed32, Signed31 | OtherUnsigned31 | OtherSigned32);
                $V!(Signed32OrMinusZero, Signed32 | MinusZero);
                $V!(Signed32OrMinusZeroOrNaN, Signed32 | MinusZero | NaN);
                $V!(Negative32, Negative31 | OtherSigned32);
                $V!(Unsigned31, Unsigned30 | OtherUnsigned31);
                $V!(Unsigned32, Unsigned30 | OtherUnsigned31 | OtherUnsigned32);
                $V!(Unsigned32OrMinusZero, Unsigned32 | MinusZero);
                $V!(Unsigned32OrMinusZeroOrNaN, Unsigned32 | MinusZero | NaN);
                $V!(Integral32, Signed32 | Unsigned32);
                $V!(Integral32OrMinusZero, Integral32 | MinusZero);
                $V!(Integral32OrMinusZeroOrNaN, Integral32OrMinusZero | NaN);
                $V!(PlainNumber, Integral32 | OtherNumber);
                $V!(OrderedNumber, PlainNumber | MinusZero);
                $V!(MinusZeroOrNaN, MinusZero | NaN);
                $V!(Number, OrderedNumber | NaN);
                $V!(SignedBigInt64, UnsignedBigInt63 | NegativeBigInt63);
                $V!(UnsignedBigInt64, UnsignedBigInt63 | OtherUnsignedBigInt64);
                $V!(BigInt, SignedBigInt64 | OtherUnsignedBigInt64 | OtherBigInt);
                $V!(Numeric, Number | BigInt);
                $V!(String, InternalizedString | OtherString);
                $V!(StringOrStringWrapper, String | StringWrapper);
                $V!(UniqueName, Symbol | InternalizedString);
                $V!(Name, Symbol | String);
                $V!(InternalizedStringOrNull, InternalizedString | Null);
                $V!(BooleanOrNumber, Boolean | Number);
                $V!(BooleanOrNullOrNumber, BooleanOrNumber | Null);
                $V!(BooleanOrNullOrUndefined, Boolean | Null | Undefined);
                $V!(NullOrNumber, Null | Number);
                $V!(NullOrUndefined, Null | Undefined);
                $V!(Undetectable, NullOrUndefined | OtherUndetectable);
                $V!(NumberOrHole, Number | Hole);
                $V!(NumberOrOddball, Number | BooleanOrNullOrUndefined);
                $V!(NumberOrOddballOrHole, NumberOrOddball | Hole);
                $V!(NumericOrString, Numeric | String);
                $V!(NumberOrUndefined, Number | Undefined);
                $V!(PlainPrimitive, Number | String | Boolean | NullOrUndefined);
                $V!(NonBigIntPrimitive, Symbol | PlainPrimitive);
                $V!(Primitive, BigInt | NonBigIntPrimitive);
                $V!(OtherUndetectableOrUndefined, OtherUndetectable | Undefined);
                $V!(Proxy, CallableProxy | OtherProxy);
                $V!(ArrayOrOtherObject, Array | OtherObject);
                $V!(ArrayOrProxy, Array | Proxy);
                $V!(StringWrapperOrOtherObject, StringWrapper | OtherObject);
                $V!(Function, CallableFunction | ClassConstructor);
                $V!(DetectableCallable, Function | BoundFunction | OtherCallable | CallableProxy);
                $V!(Callable, DetectableCallable | OtherUndetectable);
                $V!(NonCallable, Array | StringWrapper | TypedArray | OtherObject | OtherProxy | WasmObject);
                $V!(NonCallableOrNull, NonCallable | Null);
                $V!(DetectableObject, Array | Function | BoundFunction | StringWrapper | TypedArray | OtherCallable | OtherObject);
                $V!(DetectableReceiver, DetectableObject | Proxy | WasmObject);
                $V!(DetectableReceiverOrNull, DetectableReceiver | Null);
                $V!(Object, DetectableObject | OtherUndetectable);
                $V!(Receiver, Object | Proxy | WasmObject);
                $V!(ReceiverOrUndefined, Receiver | Undefined);
                $V!(ReceiverOrNull, Receiver | Null);
                $V!(ReceiverOrNullOrUndefined, Receiver | Null | Undefined);
                $V!(SymbolOrReceiver, Symbol | Receiver);
                $V!(StringOrReceiver, String | Receiver);
                $V!(Unique, Boolean | UniqueName | Null | Undefined | Hole | Receiver);
                $V!(Internal, Hole | ExternalPointer | SandboxedPointer | OtherInternal);
                $V!(NonInternal, Primitive | Receiver);
                $V!(NonBigInt, NonBigIntPrimitive | Receiver);
                $V!(NonNumber, BigInt | Unique | String | Internal);
                $V!(Any, 0xfffffffffffffffe);
            };
        }

        macro_rules! bitset_type_list {
            ($V:ident) => {
                internal_bitset_type_list!($V);
                proper_bitset_type_list!($V);
            };
        }

        macro_rules! define_type_constructor {
            ($type:ident, $value:expr) => {
                #[allow(non_snake_case)]
                pub fn $type() -> Type {
                    Type::NewBitset(BitsetType::k##$type)
                }
            };
        }
        // clang-format on

        /*
         * The following diagrams show how integers (in the mathematical sense) are
         * divided among the different atomic numerical types.
         *
         *   ON    OS32     N31     U30     OU31    OU32     ON
         * ______[_______[_______[_______[_______[_______[_______
         *     -2^31   -2^30     0      2^30    2^31    2^32
         *
         * E.g., OtherUnsigned32 (OU32) covers all integers from 2^31 to 2^32-1.
         *
         * Some of the atomic numerical bitsets are internal only (see
         * INTERNAL_BITSET_TYPE_LIST).  To a types user, they should only occur in
         * union with certain other bitsets.  For instance, OtherNumber should only
         * occur as part of PlainNumber.
         */

        pub struct BitsetType {
            // Internal
        }

        impl BitsetType {
            pub type bitset = u64;

            bitset_type_list!(declare_bitset_type);

            pub fn SignedSmall() -> bitset {
                if SmiValuesAre31Bits() {
                    Self::kSigned31
                } else {
                    Self::kSigned32
                }
            }

            pub fn UnsignedSmall() -> bitset {
                if SmiValuesAre31Bits() {
                    Self::kUnsigned30
                } else {
                    Self::kUnsigned31
                }
            }

            pub fn IsNone(bits: bitset) -> bool {
                bits == Self::kNone
            }

            pub fn Is(bits1: bitset, bits2: bitset) -> bool {
                (bits1 | bits2) == bits2
            }

            pub fn Min(bits: bitset) -> f64 {
                Self::min(bits)
            }
            fn min(bits: bitset) -> f64 {
                DisallowGarbageCollection {};
                assert!(Self::Is(bits, Self::kNumber));
                assert!(!Self::Is(bits, Self::kNaN));
                let mins = Self::Boundaries();
                let mz = bits & Self::kMinusZero != 0;

                for i in 0..Self::BoundariesSize() {
                    if Self::Is(mins[i].internal, bits) {
                        return if mz {
                            f64::min(0.0, mins[i].min)
                        } else {
                            mins[i].min
                        };
                    }
                }
                assert!(mz);
                return 0.0;
            }

            pub fn Max(bits: bitset) -> f64 {
                Self::max(bits)
            }

            fn max(bits: bitset) -> f64 {
                DisallowGarbageCollection {};
                assert!(Self::Is(bits, Self::kNumber));
                assert!(!Self::Is(bits, Self::kNaN));
                let mins = Self::Boundaries();
                let mz = bits & Self::kMinusZero != 0;
                if Self::Is(mins[Self::BoundariesSize() - 1].internal, bits) {
                    return f64::INFINITY;
                }
                for i in (0..Self::BoundariesSize() - 1).rev() {
                    if Self::Is(mins[i].internal, bits) {
                        return if mz {
                            f64::max(0.0, mins[i + 1].min - 1.0)
                        } else {
                            mins[i + 1].min - 1.0
                        };
                    }
                }
                assert!(mz);
                return 0.0;
            }

            pub fn Glb(min: f64, max: f64) -> bitset {
                DisallowGarbageCollection {};
                let mut glb = Self::kNone;
                let mins = Self::Boundaries();

                // If the range does not touch 0, the bound is empty.
                if max < -1.0 || min > 0.0 {
                    return glb;
                }

                for i in 1..(Self::BoundariesSize() - 1) {
                    if min <= mins[i].min {
                        if max + 1.0 < mins[i + 1].min {
                            break;
                        }
                        glb |= mins[i].external;
                    }
                }
                // OtherNumber also contains float numbers, so it can never be
                // in the greatest lower bound.
                return glb & !Self::kOtherNumber;
            }
            pub fn Lub<T: Lubable>(map: T, broker: &JSHeapBroker) -> bitset {
                T::lub(map, broker)
            }
            pub fn Lub_double(value: f64) -> bitset {
                DisallowGarbageCollection {};
                if IsMinusZero(value) {
                    return Self::kMinusZero;
                }
                if value.is_nan() {
                    return Self::kNaN;
                }
                if IsUint32Double(value) || IsInt32Double(value) {
                    return Self::Lub(value, value);
                }
                return Self::kOtherNumber;
            }

            pub fn Lub(min: f64, max: f64) -> bitset {
                Self::lub(min, max)
            }
            fn lub(min: f64, max: f64) -> bitset {
                DisallowGarbageCollection {};
                let mut lub = Self::kNone;
                let mins = Self::Boundaries();

                for i in 1..Self::BoundariesSize() {
                    if min < mins[i].min {
                        lub |= mins[i - 1].internal;
                        if max < mins[i].min {
                            return lub;
                        }
                    }
                }
                return lub | mins[Self::BoundariesSize() - 1].internal;
            }

            pub fn ExpandInternals(bits: bitset) -> bitset {
                if !(bits & Self::kOtherString == 0) && !(bits & Self::kString == Self::kString)
                {
                    panic!();
                }
                DisallowGarbageCollection {};
                if bits & Self::kPlainNumber == 0 {
                    return bits; // Shortcut.
                }
                let boundaries = Self::Boundaries();
                for i in 0..Self::BoundariesSize() {
                    if !Self::Is(boundaries[i].internal, boundaries[i].external) {
                        panic!()
                    };
                    if bits & boundaries[i].internal != 0 {
                        bits |= boundaries[i].external;
                    }
                }
                return bits;
            }

            pub fn Name(bits: bitset) -> &'static str {
                match bits {
                    Self::kNone => "None",
                    Self::kNegative31 => "Negative31",
                    Self::kNull => "Null",
                    Self::kUndefined => "Undefined",
                    Self::kBoolean => "Boolean",
                    Self::kUnsigned30 => "Unsigned30",
                    Self::kMinusZero => "MinusZero",
                    Self::kNaN => "NaN",
                    Self::kSymbol => "Symbol",
                    Self::kInternalizedString => "InternalizedString",
                    Self::kOtherCallable => "OtherCallable",
                    Self::kOtherObject => "OtherObject",
                    Self::kOtherUndetectable => "OtherUndetectable",
                    Self::kCallableProxy => "CallableProxy",
                    Self::kOtherProxy => "OtherProxy",
                    Self::kCallableFunction => "CallableFunction",
                    Self::kClassConstructor => "ClassConstructor",
                    Self::kBoundFunction => "BoundFunction",
                    Self::kOtherInternal => "OtherInternal",
                    Self::kExternalPointer => "ExternalPointer",
                    Self::kArray => "Array",
                    Self::kUnsignedBigInt63 => "UnsignedBigInt63",
                    Self::kOtherUnsignedBigInt64 => "OtherUnsignedBigInt64",
                    Self::kNegativeBigInt63 => "NegativeBigInt63",
                    Self::kOtherBigInt => "OtherBigInt",
                    Self::kWasmObject => "WasmObject",
                    Self::SandboxedPointer => "SandboxedPointer",
                    Self::Machine => "Machine",
                    Self::Hole => "Hole",
                    Self::StringWrapper => "StringWrapper",
                    Self::TypedArray => "TypedArray",
                    Self::Signed31 => "Signed31",
                    Self::Signed32 => "Signed32",
                    Self::Signed32OrMinusZero => "Signed32OrMinusZero",
                    Self::Signed32OrMinusZeroOrNaN => "Signed32OrMinusZeroOrNaN",
                    Self::Negative32 => "Negative32",
                    Self::Unsigned31 => "Unsigned31",
                    Self::Unsigned32 => "Unsigned32",
                    Self::Unsigned32OrMinusZero => "Unsigned32OrMinusZero",
                    Self::Unsigned32OrMinusZeroOrNaN => "Unsigned32OrMinusZeroOrNaN",
                    Self::Integral32 => "Integral32",
                    Self::Integral32OrMinusZero => "Integral32OrMinusZero",
                    Self::Integral32OrMinusZeroOrNaN => "Integral32OrMinusZeroOrNaN",
                    Self::PlainNumber => "PlainNumber",
                    Self::OrderedNumber => "OrderedNumber",
                    Self::MinusZeroOrNaN => "MinusZeroOrNaN",
                    Self::Number => "Number",
                    Self::SignedBigInt64 => "SignedBigInt64",
                    Self::UnsignedBigInt64 => "UnsignedBigInt64",
                    Self::BigInt => "BigInt",
                    Self::Numeric => "Numeric",
                    Self::String => "String",
                    Self::StringOrStringWrapper => "StringOrStringWrapper",
                    Self::UniqueName => "UniqueName",
                    Self::Name => "Name",
                    Self::InternalizedStringOrNull => "InternalizedStringOrNull",
                    Self::BooleanOrNumber => "BooleanOrNumber",
                    Self::BooleanOrNullOrNumber => "BooleanOrNullOrNumber",
                    Self::BooleanOrNullOrUndefined => "BooleanOrNullOrUndefined",
                    Self::NullOrNumber => "NullOrNumber",
                    Self::NullOrUndefined => "NullOrUndefined",
                    Self::Undetectable => "Undetectable",
                    Self::NumberOrHole => "NumberOrHole",
                    Self::NumberOrOddball => "NumberOrOddball",
                    Self::NumberOrOddballOrHole => "NumberOrOddballOrHole",
                    Self::NumericOrString => "NumericOrString",
                    Self::NumberOrUndefined => "NumberOrUndefined",
                    Self::PlainPrimitive => "PlainPrimitive",
                    Self::NonBigIntPrimitive => "NonBigIntPrimitive",
                    Self::Primitive => "Primitive",
                    Self::OtherUndetectableOrUndefined => "OtherUndetectableOrUndefined",
                    Self::Proxy => "Proxy",
                    Self::ArrayOrOtherObject => "ArrayOrOtherObject",
                    Self::ArrayOrProxy => "ArrayOrProxy",
                    Self::StringWrapperOrOtherObject => "StringWrapperOrOtherObject",
                    Self::Function => "Function",
                    Self::DetectableCallable => "DetectableCallable",
                    Self::Callable => "Callable",
                    Self::NonCallable => "NonCallable",
                    Self::NonCallableOrNull => "NonCallableOrNull",
                    Self::DetectableObject => "DetectableObject",
                    Self::DetectableReceiver => "DetectableReceiver",
                    Self::DetectableReceiverOrNull => "DetectableReceiverOrNull",
                    Self::Object => "Object",
                    Self::Receiver => "Receiver",
                    Self::ReceiverOrUndefined => "ReceiverOrUndefined",
                    Self::ReceiverOrNull => "ReceiverOrNull",
                    Self::ReceiverOrNullOrUndefined => "ReceiverOrNullOrUndefined",
                    Self::SymbolOrReceiver => "SymbolOrReceiver",
                    Self::StringOrReceiver => "StringOrReceiver",
                    Self::Unique => "Unique",
                    Self::Internal => "Internal",
                    Self::NonInternal => "NonInternal",
                    Self::NonBigInt => "NonBigInt",
                    Self::NonNumber => "NonNumber",
                    Self::Any => "Any",

                    Self::OtherUnsigned31 => "OtherUnsigned31",
                    Self::OtherUnsigned32 => "OtherUnsigned32",
                    Self::OtherSigned32 => "OtherSigned32",
                    Self::OtherNumber => "OtherNumber",
                    Self::OtherString => "OtherString",

                    _ => return "unknown",
                }
            }

            pub fn Print(os: &mut std::fmt::Formatter, bits: bitset) -> std::fmt::Result {
                let name = Self::Name(bits);
                if name != "unknown" {
                    write!(os, "{}", name)?;
                    return Ok(());
                }

                // clang-format off
                const NAMED_BITSETS: [bitset; 44] = [
                    Self::OtherUnsigned31,
                    Self::OtherUnsigned32,
                    Self::OtherSigned32,
                    Self::OtherNumber,
                    Self::OtherString,
                    Self::Negative31,
                    Self::Null,
                    Self::Undefined,
                    Self::Boolean,
                    Self::Unsigned30,
                    Self::MinusZero,
                    Self::NaN,
                    Self::Symbol,
                    Self::InternalizedString,
                    Self::OtherCallable,
                    Self::OtherObject,
                    Self::OtherUndetectable,
                    Self::CallableProxy,
                    Self::OtherProxy,
                    Self::CallableFunction,
                    Self::ClassConstructor,
                    Self::BoundFunction,
                    Self::OtherInternal,
                    Self::ExternalPointer,
                    Self::Array,
                    Self::UnsignedBigInt63,
                    Self::OtherUnsignedBigInt64,
                    Self::NegativeBigInt63,
                    Self::OtherBigInt,
                    Self::WasmObject,
                    Self::SandboxedPointer,
                    Self::Machine,
                    Self::Hole,
                    Self::StringWrapper,
                    Self::TypedArray,
                    Self::Signed31,
                    Self::Signed32,
                    Self::Negative32,
                    Self::Unsigned31,
                    Self::Unsigned32,
                    Self::BigInt,
                    Self::String,
                    Self::UniqueName,
                    Self::Name,
                ];
                // clang-format on

                let mut bits_mut = bits;
                let mut is_first = true;
                write!(os, "(")?;
                for i in (0..NAMED_BITSETS.len()).rev() {
                    let subset = NAMED_BITSETS[i];
                    if (bits_mut & subset) == subset {
                        if !is_first {
                            write!(os, " | ")?;
                        }
                        is_first = false;
                        write!(os, "{}", Self::Name(subset))?;
                        bits_mut -= subset;
                    }
                }
                assert_eq!(0, bits_mut);
                write!(os, ")")?;
                Ok(())
            }

            pub fn NumberBits(bits: bitset) -> bitset {
                bits & Self::kPlainNumber
            }
        }

        // -----------------------------------------------------------------------------
        // Superclass for non-bitset types (internal).
        pub struct TypeBase {
            kind_: Kind,
        }

        impl TypeBase {
            fn kind(&self) -> Kind {
                self.kind_
            }
            fn IsKind(type_: Type, kind: Kind) -> bool {
                if type_.IsBitset() {
                    return false;
                }
                let base = type_.ToTypeBase();
                base.kind() == kind
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Kind {
            kHeapConstant,
            kOtherNumberConstant,
            kTuple,
            kUnion,
            kRange,
            kWasm,
        }

        // -----------------------------------------------------------------------------
        // Range types.

        pub struct RangeType {
            bitset_: BitsetType::bitset,
            limits_: Limits,
            base: TypeBase,
        }

        impl RangeType {
            pub struct Limits {
                pub min: f64,
                pub max: f64,
            }

            impl Limits {
                pub fn IsEmpty(&self) -> bool {
                    self.min > self.max
                }
                pub fn Empty() -> Self {
                    Limits { min: 1.0, max: 0.0 }
                }
                pub fn Intersect(lhs: Limits, rhs: Limits) -> Limits {
                    DisallowGarbageCollection {};
                    let mut result = lhs;
                    if lhs.min < rhs.min {
                        result.min = rhs.min;
                    }
                    if lhs.max > rhs.max {
                        result.max = rhs.max;
                    }
                    return result;
                }
                pub fn Union(lhs: Limits, rhs: Limits) -> Limits {
                    DisallowGarbageCollection {};
                    if lhs.IsEmpty() {
                        return rhs;
                    }
                    if rhs.IsEmpty() {
                        return lhs;
                    }
                    let mut result = lhs;
                    if lhs.min > rhs.min {
                        result.min = rhs.min;
                    }
                    if lhs.max < rhs.max {
                        result.max = rhs.max;
                    }
                    return result;
                }
            }

            pub fn Min(&self) -> f64 {
                self.limits_.min
            }
            pub fn Max(&self) -> f64 {
                self.limits_.max
            }

            pub fn IsInteger(x: f64) -> bool {
                let nearby = x.round();
                nearby == x && !IsMinusZero(x) // Allows for infinities.
            }
        }
        pub trait Lubable {
            fn lub<T: Lubable>(map: T,
