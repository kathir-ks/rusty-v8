// Converted from V8 C++ source files:
// Header: property-details.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(non_camel_case_types)]

use std::fmt;

//use crate::v8::PropertyFilter;
use crate::v8::READ_ONLY;
use crate::v8::DONT_ENUM;
use crate::v8::DONT_DELETE;
use crate::v8::HeapObject;
use crate::v8::internal::Tagged;
use crate::v8::internal::Smi;
use crate::v8::internal::Object;
use crate::v8::PropertyAttributes;

// ES6 6.1.7.1
//enum PropertyAttributes {
//  NONE = ::v8::None,
//  READ_ONLY = ::v8::ReadOnly,
//  DONT_ENUM = ::v8::DontEnum,
//  DONT_DELETE = ::v8::DontDelete,
//
//  ALL_ATTRIBUTES_MASK = READ_ONLY | DONT_ENUM | DONT_DELETE,
//
//  SEALED = DONT_DELETE,
//  FROZEN = SEALED | READ_ONLY,
//
//  ABSENT = 64,  // Used in runtime to indicate a property is absent.
//  // ABSENT can never be stored in or returned from a descriptor's attributes
//  // bitfield.  It is only used as a return value meaning the attributes of
//  // a non-existent property.
//}

#[allow(dead_code)]
const ALL_ATTRIBUTES_MASK: i32 = READ_ONLY as i32 | DONT_ENUM as i32 | DONT_DELETE as i32;
#[allow(dead_code)]
const SEALED: i32 = DONT_DELETE as i32;
#[allow(dead_code)]
const FROZEN: i32 = SEALED | READ_ONLY as i32;
#[allow(dead_code)]
const ABSENT: i32 = 64;

#[allow(dead_code)]
#[inline]
fn PropertyAttributesFromInt(value: i32) -> PropertyAttributes {
    assert_eq!(value & !ALL_ATTRIBUTES_MASK, 0);
    unsafe { std::mem::transmute(value as u8) }
}

// Number of distinct bits in PropertyAttributes.
const kPropertyAttributesBitsCount: i32 = 3;

const kPropertyAttributesCombinationsCount: i32 = 1 << kPropertyAttributesBitsCount;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum PropertyFilter {
    ALL_PROPERTIES = 0,
    ONLY_WRITABLE = 1,
    ONLY_ENUMERABLE = 2,
    ONLY_CONFIGURABLE = 4,
    SKIP_STRINGS = 8,
    SKIP_SYMBOLS = 16,
    PRIVATE_NAMES_ONLY = 32,
    ENUMERABLE_STRINGS = ONLY_ENUMERABLE as i32 | SKIP_SYMBOLS as i32,
}

// Enable fast comparisons of PropertyAttributes against PropertyFilters.
//static_assert(ALL_PROPERTIES == static_cast<PropertyFilter>(NONE));
//static_assert(ONLY_WRITABLE == static_cast<PropertyFilter>(READ_ONLY));
//static_assert(ONLY_ENUMERABLE == static_cast<PropertyFilter>(DONT_ENUM));
//static_assert(ONLY_CONFIGURABLE == static_cast<PropertyFilter>(DONT_DELETE));
//static_assert(((SKIP_STRINGS | SKIP_SYMBOLS) & ALL_ATTRIBUTES_MASK) == 0);
//static_assert(ALL_PROPERTIES ==
//              static_cast<PropertyFilter>(v8::PropertyFilter::ALL_PROPERTIES));
//static_assert(ONLY_WRITABLE ==
//              static_cast<PropertyFilter>(v8::PropertyFilter::ONLY_WRITABLE));
//static_assert(ONLY_ENUMERABLE ==
//              static_cast<PropertyFilter>(v8::PropertyFilter::ONLY_ENUMERABLE));
//static_assert(ONLY_CONFIGURABLE == static_cast<PropertyFilter>(
//                                       v8::PropertyFilter::ONLY_CONFIGURABLE));
//static_assert(SKIP_STRINGS ==
//              static_cast<PropertyFilter>(v8::PropertyFilter::SKIP_STRINGS));
//static_assert(SKIP_SYMBOLS ==
//              static_cast<PropertyFilter>(v8::PropertyFilter::SKIP_SYMBOLS));

// Assert that kPropertyAttributesBitsCount value matches the definition of
// ALL_ATTRIBUTES_MASK.
//static_assert((ALL_ATTRIBUTES_MASK == (READ_ONLY | DONT_ENUM | DONT_DELETE)) ==
//              (kPropertyAttributesBitsCount == 3));

//class Smi;
//class TypeInfo;

// Order of kinds is significant.
// Must fit in the BitField PropertyDetails::KindField.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum PropertyKind {
    kData = 0,
    kAccessor = 1,
}

// Order of modes is significant.
// Must fit in the BitField PropertyDetails::LocationField.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum PropertyLocation {
    kField = 0,
    kDescriptor = 1,
}

// Order of modes is significant.
// Must fit in the BitField PropertyDetails::ConstnessField.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum PropertyConstness {
    kMutable = 0,
    kConst = 1,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Representation {
    kind_: i8,
}

impl Representation {
    pub enum Kind {
        kNone,
        kSmi,
        kDouble,
        kHeapObject,
        kTagged,
        // This representation is used for WasmObject fields and basically means
        // that the actual field type information must be taken from the Wasm RTT
        // associated with the map.
        kWasmValue,
        kNumRepresentations
    }

    pub const fn new() -> Self {
        Representation { kind_: Representation::Kind::kNone as i8 }
    }

    pub const fn none() -> Self {
        Representation { kind_: Representation::Kind::kNone as i8 }
    }

    pub const fn tagged() -> Self {
        Representation { kind_: Representation::Kind::kTagged as i8 }
    }

    pub const fn smi() -> Self {
        Representation { kind_: Representation::Kind::kSmi as i8 }
    }

    pub const fn double() -> Self {
        Representation { kind_: Representation::Kind::kDouble as i8 }
    }

    pub const fn heap_object() -> Self {
        Representation { kind_: Representation::Kind::kHeapObject as i8 }
    }

    pub const fn wasm_value() -> Self {
        Representation { kind_: Representation::Kind::kWasmValue as i8 }
    }

    pub const fn from_kind(kind: Representation::Kind) -> Self {
        Representation { kind_: kind as i8 }
    }

    pub fn equals(&self, other: &Representation) -> bool {
        self.kind_ == other.kind_
    }

    pub fn is_compatible_for_load(&self, other: &Representation) -> bool {
        self.is_double() == other.is_double()
    }

    pub fn is_compatible_for_store(&self, other: &Representation) -> bool {
        self.equals(other)
    }

    // Returns true if a change from this representation to a more general one
    // might cause a map deprecation.
    pub fn might_cause_map_deprecation(&self) -> bool {
        // HeapObject to tagged representation change can be done in-place.
        // Boxed double to tagged transition is always done in-place.
        // Note that WasmValue is not supposed to be changed at all (the only
        // representation it fits into is WasmValue), so for the sake of predicate
        // correctness we treat it as in-place "changeable".
        if self.is_tagged() || self.is_heap_object() || self.is_double() || self.is_wasm_value() {
            return false;
        }
        // None to double and smi to double representation changes require
        // deprecation, because doubles might require box allocation, see
        // CanBeInPlaceChangedTo().
        assert!(self.is_none() || self.is_smi());
        true
    }

    pub fn can_be_in_place_changed_to(&self, other: &Representation) -> bool {
        if self.equals(other) {
            return true;
        }
        if self.is_wasm_value() || other.is_wasm_value() {
            return false;
        }
        // If it's just a representation generalization case (i.e. property kind and
        // attributes stays unchanged) it's fine to transition from None to anything
        // but double without any modification to the object, because the default
        // uninitialized value for representation None can be overwritten by both
        // smi and tagged values. Doubles, however, would require a box allocation.
        if self.is_none() {
            return !other.is_double();
        }
        if !other.is_tagged() {
            return false;
        }
        assert!(self.is_smi() || self.is_double() || self.is_heap_object());
        true
    }

    // Return the most generic representation that this representation can be
    // changed to in-place. If an in-place representation change is not allowed,
    // then this will return the current representation.
    pub fn most_generic_in_place_change(&self) -> Representation {
        if self.is_wasm_value() {
            return Representation::wasm_value();
        }
        Representation::tagged()
    }

    pub fn is_more_general_than(&self, other: &Representation) -> bool {
        if self.is_wasm_value() {
            return false;
        }
        if self.is_heap_object() {
            return other.is_none();
        }
        self.kind_ > other.kind_
    }

    pub fn fits_into(&self, other: &Representation) -> bool {
        other.is_more_general_than(self) || other.equals(self)
    }

    pub fn generalize(&self, other: Representation) -> Representation {
        if other.fits_into(*self)) {
            return *self;
        }
        if other.is_more_general_than(*self)) {
            return other;
        }
        Representation::tagged()
    }

    pub fn size(&self) -> i32 {
        assert!(!self.is_none());
        if self.is_double() {
            return kDoubleSize;
        }
        assert!(self.is_tagged() || self.is_smi() || self.is_heap_object());
        kTaggedSize
    }

    pub const fn kind(&self) -> Representation::Kind {
        unsafe { std::mem::transmute(self.kind_) }
    }

    pub const fn is_none(&self) -> bool {
        self.kind_ == Representation::Kind::kNone as i8
    }

    pub const fn is_wasm_value(&self) -> bool {
        self.kind_ == Representation::Kind::kWasmValue as i8
    }

    pub const fn is_tagged(&self) -> bool {
        self.kind_ == Representation::Kind::kTagged as i8
    }

    pub const fn is_smi(&self) -> bool {
        self.kind_ == Representation::Kind::kSmi as i8
    }

    pub const fn is_smi_or_tagged(&self) -> bool {
        self.is_smi() || self.is_tagged()
    }

    pub const fn is_double(&self) -> bool {
        self.kind_ == Representation::Kind::kDouble as i8
    }

    pub const fn is_heap_object(&self) -> bool {
        self.kind_ == Representation::Kind::kHeapObject as i8
    }

    pub fn mnemonic(&self) -> &'static str {
        match unsafe { std::mem::transmute::<i8, Representation::Kind>(self.kind_) } {
            Representation::Kind::kNone => "v",
            Representation::Kind::kTagged => "t",
            Representation::Kind::kSmi => "s",
            Representation::Kind::kDouble => "d",
            Representation::Kind::kHeapObject => "h",
            Representation::Kind::kWasmValue => "w",
            _ => unreachable!(),
        }
    }
}

const kDescriptorIndexBitCount: i32 = 10;
const kFirstInobjectPropertyOffsetBitCount: i32 = 7;
// The maximum number of descriptors we want in a descriptor array.  It should
// fit in a page and also the following should hold:
// kMaxNumberOfDescriptors + kFieldsAdded <= PropertyArray::kMaxLength.
const kMaxNumberOfDescriptors: i32 = (1 << kDescriptorIndexBitCount) - 4;
const kInvalidEnumCacheSentinel: i32 = (1 << kDescriptorIndexBitCount) - 1;

// A PropertyCell's property details contains a cell type that is meaningful if
// the cell is still valid (does not hold the hole).
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum PropertyCellType {
    kMutable,       // Cell will no longer be tracked as constant.
    kUndefined,     // The PREMONOMORPHIC of property cells.
    kConstant,      // Cell has been assigned only once.
    kConstantType,  // Cell has been assigned only one type.
    // Temporary value indicating an ongoing property cell state transition. Only
    // observable by a background thread.
    kInTransition,
    // Value for dictionaries not holding cells, must be 0:
    kNoCell = kMutable,
}

// PropertyDetails captures type and attributes for a property.
// They are used both in property dictionaries and instance descriptors.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PropertyDetails {
    value_: u32,
}

impl PropertyDetails {
    // Property details for global dictionary properties.
    pub const fn new_global_dictionary(
        kind: PropertyKind,
        attributes: PropertyAttributes,
        cell_type: PropertyCellType,
        dictionary_index: i32,
    ) -> Self {
        PropertyDetails {
            value_: Self::KindField::encode(kind)
                | Self::LocationField::encode(PropertyLocation::kField)
                | Self::AttributesField::encode(attributes)
                // We track PropertyCell constness via PropertyCellTypeField,
                // so we set ConstnessField to kMutable to simplify DCHECKs
                // related to non-global property constness tracking.
                | Self::ConstnessField::encode(PropertyConstness::kMutable)
                | Self::DictionaryStorageField::encode(dictionary_index as u32)
                | Self::PropertyCellTypeField::encode(cell_type),
        }
    }

    // Property details for dictionary mode properties/elements.
    pub const fn new_dictionary(
        kind: PropertyKind,
        attributes: PropertyAttributes,
        constness: PropertyConstness,
        dictionary_index: i32,
    ) -> Self {
        PropertyDetails {
            value_: Self::KindField::encode(kind)
                | Self::LocationField::encode(PropertyLocation::kField)
                | Self::AttributesField::encode(attributes)
                | Self::ConstnessField::encode(constness)
                | Self::DictionaryStorageField::encode(dictionary_index as u32)
                | Self::PropertyCellTypeField::encode(PropertyCellType::kNoCell),
        }
    }

    // Property details for fast mode properties.
    pub const fn new_fast(
        kind: PropertyKind,
        attributes: PropertyAttributes,
        location: PropertyLocation,
        constness: PropertyConstness,
        representation: Representation,
        field_index: i32,
    ) -> Self {
        PropertyDetails {
            value_: Self::KindField::encode(kind)
                | Self::AttributesField::encode(attributes)
                | Self::LocationField::encode(location)
                | Self::ConstnessField::encode(constness)
                | Self::RepresentationField::encode(Self::encode_representation(representation))
                | Self::FieldIndexField::encode(field_index as u32),
        }
    }

    pub const fn empty(cell_type: PropertyCellType) -> Self {
        PropertyDetails::new_global_dictionary(PropertyKind::kData, unsafe { std::mem::transmute(0) }, cell_type, 0)
    }

    pub fn pointer(&self) -> i32 {
        Self::DescriptorPointer::decode(self.value_) as i32
    }

    pub fn set_pointer(&self, i: i32) -> PropertyDetails {
        PropertyDetails {
            value_: Self::DescriptorPointer::update(self.value_, i as u32),
        }
    }

    pub fn set_cell_type(&self, type_: PropertyCellType) -> PropertyDetails {
        PropertyDetails {
            value_: Self::PropertyCellTypeField::update(self.value_, type_),
        }
    }

    pub fn set_index(&self, index: i32) -> PropertyDetails {
        PropertyDetails {
            value_: Self::DictionaryStorageField::update(self.value_, index as u32),
        }
    }

    pub fn copy_with_representation(&self, representation: Representation) -> PropertyDetails {
        PropertyDetails {
            value_: Self::RepresentationField::update(self.value_, Self::encode_representation(representation)),
        }
    }

    pub fn copy_with_constness(&self, constness: PropertyConstness) -> PropertyDetails {
        PropertyDetails {
            value_: Self::ConstnessField::update(self.value_, constness),
        }
    }

    pub fn copy_add_attributes(&self, new_attributes: PropertyAttributes) -> PropertyDetails {
        let new_attributes_ =
            unsafe { std::mem::transmute((self.attributes() as i32 | new_attributes as i32) as u8) };
        PropertyDetails {
            value_: Self::AttributesField::update(self.value_, new_attributes_),
        }
    }

    // Conversion for storing details as Object.
    pub fn new_smi(smi: Tagged<Smi>) -> Self {
        PropertyDetails { value_: smi.ptr() as u32 }
    }

    pub fn as_smi(&self) -> Tagged<Smi> {
        unsafe { Tagged::<Smi>::from_ptr(self.value_ as usize) }
    }

    pub const fn encode_representation(representation: Representation) -> u32 {
        representation.kind() as u32
    }

    pub fn decode_representation(bits: u32) -> Representation {
        Representation::from_kind(unsafe { std::mem::transmute(bits as i8) })
    }

    pub fn kind(&self) -> PropertyKind {
        Self::KindField::decode(self.value_)
    }

    pub fn location(&self) -> PropertyLocation {
        Self::LocationField::decode(self.value_)
    }

    pub fn constness(&self) -> PropertyConstness {
        Self::ConstnessField::decode(self.value_)
    }

    pub fn attributes(&self) -> PropertyAttributes {
        Self::AttributesField::decode(self.value_)
    }

    pub fn has_kind_and_attributes(&self, kind: PropertyKind, attributes: PropertyAttributes) -> bool {
        (self.value_ & (Self::KindField::kMask | Self::AttributesField::kMask))
            == (Self::KindField::encode(kind) | Self::AttributesField::encode(attributes))
    }

    pub fn dictionary_index(&self) -> i32 {
        Self::DictionaryStorageField::decode(self.value_) as i32
    }

    pub fn representation(&self) -> Representation {
        Self::decode_representation(Self::RepresentationField::decode(self.value_))
    }

    pub fn field_index(&self) -> i32 {
        Self::FieldIndexField::decode(self.value_) as i32
    }

    pub fn field_width_in_words(&self) -> i32 {
        1 // Placeholder
    }

    pub fn is_valid_index(index: i32) -> bool {
        Self::DictionaryStorageField::is_valid(index as u32)
    }

    pub fn is_read_only(&self) -> bool {
        (self.attributes() as i32 & READ_ONLY as i32) != 0
    }

    pub fn is_configurable(&self) -> bool {
        (self.attributes() as i32 & DONT_DELETE as i32) == 0
    }

    pub fn is_dont_enum(&self) -> bool {
        (self.attributes() as i32 & DONT_ENUM as i32) != 0
    }

    pub fn is_enumerable(&self) -> bool {
        !self.is_dont_enum()
    }

    pub fn cell_type(&self) -> PropertyCellType {
        Self::PropertyCellTypeField::decode(self.value_)
    }

    const kInitialIndex: i32 = 1;

    const kConstIfDictConstnessTracking: PropertyConstness = if true {
        PropertyConstness::kConst
    } else {
        PropertyConstness::kMutable
    };

    pub fn to_byte(&self) -> u8 {
        // We only care about the value of KindField, ConstnessField, and
        // AttributesField. We've statically asserted earlier that these fields fit
        // into a byte together.

        assert_eq!(PropertyLocation::kField, self.location());
        assert_eq!(PropertyCellType::kNoCell, self.cell_type());

        // Only to be used when the enum index isn't actually maintained
        // by the PropertyDetails:
        assert_eq!(0, self.dictionary_index());

        self.value_ as u8
    }

    // Only to be used for bytes obtained by ToByte. In particular, only used for
    // non-global dictionary properties.
    pub fn from_byte(encoded_details: u8) -> PropertyDetails {
        // The 0-extension to 32bit sets PropertyLocation to kField,
        // PropertyCellType to kNoCell, and enumeration index to 0, as intended.
        // Everything else is obtained from |encoded_details|.
        let details = PropertyDetails { value_: encoded_details as u32 };
        assert_eq!(PropertyLocation::kField, details.location());
        assert_eq!(PropertyCellType::kNoCell, details.cell_type());
        assert_eq!(0, details.dictionary_index());
        details
    }
}

impl PropertyDetails {
    // Bit fields in value_ (type, shift, size). Must be public so the
    // constants can be embedded in generated code.
    #[allow(dead_code)]
    pub struct KindField {}
    impl KindField {
        const kShift: u32 = 0;
        const kSize: u32 = 1;
        const kMask: u32 = ((1u32 << Self::kSize) - 1) << Self::kShift;

        #[allow(dead_code)]
        fn encode(value: PropertyKind) -> u32 {
            (value as u32) << Self::kShift
        }
        #[allow(dead_code)]
        fn decode(value: u32) -> PropertyKind {
            unsafe { std::mem::transmute((value >> Self::kShift) as u8) }
        }
        const kLastUsedBit: u32 = Self::kShift + Self::kSize -1;
    }

    #[allow(dead_code)]
    pub struct ConstnessField {}
    impl ConstnessField {
        const kShift: u32 = KindField::kShift + KindField::kSize;
        const kSize: u32 = 1;
        const kMask: u32 = ((1u32 << Self::kSize) - 1) << Self::kShift;
        #[allow(dead_code)]
        fn encode(value: PropertyConstness) -> u32 {
            (value as u32) << Self::kShift
        }
        #[allow(dead_code)]
        fn decode(value: u32) -> PropertyConstness {
             unsafe { std::mem::transmute((value >> Self::kShift) as u8) }
        }
        const kLastUsedBit: u32 = Self::kShift + Self::kSize -1;
    }

    #[allow(dead_code)]
    pub struct AttributesField {}
    impl AttributesField{
        const kShift: u32 = ConstnessField::kShift + ConstnessField::kSize;
        const kSize: u32 = 3;
        const kMask: u32 = ((1u32 << Self::kSize) - 1) << Self::kShift;
        #[allow(dead_code)]
        fn encode(value: PropertyAttributes) -> u32 {
            (value as u32) << Self::kShift
        }
        #[allow(dead_code)]
        fn decode(value: u32) -> PropertyAttributes {
            unsafe { std::mem::transmute((value >> Self::kShift) as u8) }
        }
        const kLastUsedBit: u32 = Self::kShift + Self::kSize -1;
    }

    const kAttributesReadOnlyMask: i32 = (READ_ONLY as i32) << AttributesField::kShift;
    const kAttributesDontDeleteMask: i32 = (DONT_DELETE as i32) << AttributesField::kShift;
    const kAttributesDontEnumMask: i32 = (DONT_ENUM as i32) << AttributesField::kShift;

    // Bit fields for normalized/dictionary mode objects.
    #[allow(dead_code)]
    pub struct PropertyCellTypeField {}
    impl PropertyCellTypeField{
        const kShift: u32 = AttributesField::kShift + AttributesField::kSize;
        const kSize: u32 = 3;
        const kMask: u32 = ((1u32 << Self::kSize) - 1) << Self::kShift;
        #[allow(dead_code)]
        fn encode(value: PropertyCellType) -> u32 {
            (value as u32) << Self::kShift
        }
         #[allow(dead_code)]
        fn decode(value: u32) -> PropertyCellType {
            unsafe { std::mem::transmute((value >> Self::kShift) as u8) }
        }

         #[allow(dead_code)]
        fn update(value: u32, new_value: PropertyCellType) -> u32 {
            (value & !Self::kMask) | Self::encode(new_value)
        }
        const kLastUsedBit: u32 = Self::kShift + Self::kSize -1;

    }

    #[allow(dead_code)]
    pub struct DictionaryStorageField {}
    impl DictionaryStorageField{
        const kShift: u32 = PropertyCellTypeField::kShift + PropertyCellTypeField::kSize;
        const kSize: u32 = 23;
        const kMask: u32 = ((1u32 << Self::kSize) - 1) << Self::kShift;
        #[allow(dead_code)]
        fn encode(value: u32) -> u32 {
            (value as u32) << Self::kShift
        }
         #[allow(dead_code)]
        fn decode(value: u32) -> u32 {
            ((value >> Self::kShift) as u32)
        }
        #[allow(dead_code)]
        fn update(value: u32, new_value: u32) -> u32 {
            (value & !Self::kMask) | Self::encode(new_value)
        }
        const kLastUsedBit: u32 = Self::kShift + Self::kSize -1;

        fn is_valid(index: u32) -> bool {
            index <= ((1 << Self::kSize) - 1)
        }

    }

    // Bit fields for fast objects.
    #[allow(dead_code)]
    pub struct LocationField {}
    impl LocationField {
        const kShift: u32 = AttributesField::kShift + AttributesField::kSize;
        const kSize: u32 = 1;
        const kMask: u32 = ((1u32 << Self::kSize) - 1) << Self::kShift;
        #[allow(dead_code)]
        fn encode(value: PropertyLocation) -> u32 {
            (value as u32) << Self::kShift
        }
         #[allow(dead_code)]
        fn decode(value: u32) -> PropertyLocation {
            unsafe { std::mem::transmute((value >> Self::kShift) as u8) }
        }

         #[allow(dead_code)]
        fn update(value: u32, new_value: PropertyLocation) -> u32 {
            (value & !Self::kMask) | Self::encode(new_value)
        }
        const kLastUsedBit: u32 = Self::kShift + Self::kSize -1;
    }

    #[allow(dead_code)]
    pub struct RepresentationField {}
    impl RepresentationField {
        const kShift: u32 = LocationField::kShift + LocationField::kSize;
        const kSize: u32 = 3;
        const kMask: u32 = ((1u32 << Self::kSize) - 1) << Self::kShift;
        #[allow(dead_code)]
        fn encode(value: u32) -> u32 {
            (value as u32) << Self::kShift
        }
         #[allow(dead_code)]
        fn decode(value: u32) -> u32 {
            ((value >> Self::kShift) as u32)
        }

         #[allow(dead_code)]
        fn update(value: u32, new_value: u32) -> u32 {
            (value & !Self::kMask) | Self::encode(new_value)
        }
        const kLastUsedBit: u32 = Self::kShift + Self::kSize -1;
    }

    #[allow(dead_code)]
    pub struct DescriptorPointer {}
    impl DescriptorPointer{
        const kShift: u32 = RepresentationField::kShift + RepresentationField::kSize;
        const kSize: u32 = kDescriptorIndexBitCount as u32;
        const kMask: u32 = ((1u32 << Self::kSize) - 1) << Self::kShift;
        #[allow(dead_code)]
        fn encode(value: u32) -> u32 {
            (value as u32) << Self::kShift
        }
         #[allow(dead_code)]
        fn decode(value: u32) -> u32 {
            ((value >> Self::kShift) as u32)
        }

         #[allow(dead_code)]
        fn update(value: u32, new_value: u32) -> u32 {
            (value & !Self::kMask) | Self::encode(new_value)
        }
        const kLastUsedBit: u32 = Self::kShift + Self::kSize -1;
    }

    #[allow(dead_code)]
    pub struct FieldIndexField {}
    impl FieldIndexField{
        const kShift: u32 = DescriptorPointer::kShift + DescriptorPointer::kSize;
        const kSize: u32 = kDescriptorIndexBitCount as u32;
        const kMask: u32 = ((1u32 << Self::kSize) - 1) << Self::kShift;
        #[allow(dead_code)]
        fn encode(value: u32) -> u32 {
            (value as u32) << Self::kShift
        }
         #[allow(dead_code)]
        fn decode(value: u32) -> u32 {
            ((value >> Self::kShift) as u32)
        }

         #[allow(dead_code)]
        fn update(value: u32, new_value: u32) -> u32 {
            (value & !Self::kMask) | Self::encode(new_value)
        }
        const kLastUsedBit: u32 = Self::kShift + Self::kSize -1;
    }
}

// kField location is more general than kDescriptor, kDescriptor generalizes
// only to itself.
#[inline]
fn is_generalizable_to(a: PropertyLocation, b: PropertyLocation) -> bool {
    b == PropertyLocation::kField || a == PropertyLocation::kDescriptor
}

// PropertyConstness::kMutable constness is more general than
// VariableMode::kConst, VariableMode::kConst generalizes only to itself.
#[inline]
fn is_generalizable_to_constness(a: PropertyConstness, b: PropertyConstness) -> bool {
    b == PropertyConstness::kMutable || a == PropertyConstness::kConst
}

#[inline]
fn generalize_constness(a: PropertyConstness, b: PropertyConstness) -> PropertyConstness {
    if a == PropertyConstness::kMutable {
        PropertyConstness::kMutable
    } else {
        b
    }
}

impl fmt::Display for Representation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.mnemonic())
    }
}

impl fmt::Display for PropertyAttributes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut attrs = Vec::new();
        if (self as i32 & READ_ONLY as i32) != 0 {
            attrs.push("READ_ONLY");
        }
        if (self as i32 & DONT_ENUM as i32) != 0 {
            attrs.push("DONT_ENUM");
        
