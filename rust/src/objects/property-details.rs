// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod property_details {
    use std::fmt;

    // Mimic v8::None, v8::ReadOnly, etc.  These should ideally come from a `v8` crate.
    pub const NONE: i32 = 0;
    pub const READ_ONLY: i32 = 1;
    pub const DONT_ENUM: i32 = 2;
    pub const DONT_DELETE: i32 = 4;

    /// ES6 6.1.7.1
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(i32)]
    pub enum PropertyAttributes {
        None = NONE,
        ReadOnly = READ_ONLY,
        DontEnum = DONT_ENUM,
        DontDelete = DONT_DELETE,
        // Note: Absent is intentionally excluded as it's not a storable attribute.
    }

    pub const ALL_ATTRIBUTES_MASK: i32 = READ_ONLY | DONT_ENUM | DONT_DELETE;
    pub const SEALED: i32 = DONT_DELETE;
    pub const FROZEN: i32 = SEALED | READ_ONLY;
    pub const ABSENT: i32 = 64; // Used in runtime to indicate a property is absent.

    pub fn property_attributes_from_int(value: i32) -> PropertyAttributes {
        assert_eq!(value & !ALL_ATTRIBUTES_MASK, 0);
        match value {
            NONE => PropertyAttributes::None,
            READ_ONLY => PropertyAttributes::ReadOnly,
            DONT_ENUM => PropertyAttributes::DontEnum,
            DONT_DELETE => PropertyAttributes::DontDelete,
            _ => panic!("Invalid PropertyAttributes value"),
        }
    }

    // Number of distinct bits in PropertyAttributes.
    pub const K_PROPERTY_ATTRIBUTES_BITS_COUNT: i32 = 3;

    pub const K_PROPERTY_ATTRIBUTES_COMBINATIONS_COUNT: i32 = 1 << K_PROPERTY_ATTRIBUTES_BITS_COUNT;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(i32)]
    pub enum PropertyFilter {
        AllProperties = 0,
        OnlyWritable = 1,
        OnlyEnumerable = 2,
        OnlyConfigurable = 4,
        SkipStrings = 8,
        SkipSymbols = 16,
        PrivateNamesOnly = 32,
        EnumerableStrings = ONLY_ENUMERABLE | SKIP_SYMBOLS,
    }

    // Enable fast comparisons of PropertyAttributes against PropertyFilters.
    // static_assert(ALL_PROPERTIES == static_cast<PropertyFilter>(NONE));
    // static_assert(ONLY_WRITABLE == static_cast<PropertyFilter>(READ_ONLY));
    // static_assert(ONLY_ENUMERABLE == static_cast<PropertyFilter>(DONT_ENUM));
    // static_assert(ONLY_CONFIGURABLE == static_cast<PropertyFilter>(DONT_DELETE));
    // static_assert(((SKIP_STRINGS | SKIP_SYMBOLS) & ALL_ATTRIBUTES_MASK) == 0);
    // static_assert(ALL_PROPERTIES == static_cast<PropertyFilter>(v8::PropertyFilter::ALL_PROPERTIES));
    // static_assert(ONLY_WRITABLE == static_cast<PropertyFilter>(v8::PropertyFilter::ONLY_WRITABLE));
    // static_assert(ONLY_ENUMERABLE == static_cast<PropertyFilter>(v8::PropertyFilter::ONLY_ENUMERABLE));
    // static_assert(ONLY_CONFIGURABLE == static_cast<PropertyFilter>(v8::PropertyFilter::ONLY_CONFIGURABLE));
    // static_assert(SKIP_STRINGS == static_cast<PropertyFilter>(v8::PropertyFilter::SKIP_STRINGS));
    // static_assert(SKIP_SYMBOLS == static_cast<PropertyFilter>(v8::PropertyFilter::SKIP_SYMBOLS));

    // Assert that kPropertyAttributesBitsCount value matches the definition of
    // ALL_ATTRIBUTES_MASK.
    // static_assert((ALL_ATTRIBUTES_MASK == (READ_ONLY | DONT_ENUM | DONT_DELETE)) ==
    //               (kPropertyAttributesBitsCount == 3));

    // Placeholder for Smi and TypeInfo.  Replace with actual definitions.
    pub struct Smi {}
    pub struct TypeInfo {}

    // Order of kinds is significant.
    // Must fit in the BitField PropertyDetails::KindField.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum PropertyKind {
        Data = 0,
        Accessor = 1,
    }

    // Order of modes is significant.
    // Must fit in the BitField PropertyDetails::LocationField.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum PropertyLocation {
        Field = 0,
        Descriptor = 1,
    }

    // Order of modes is significant.
    // Must fit in the BitField PropertyDetails::ConstnessField.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum PropertyConstness {
        Mutable = 0,
        Const = 1,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Representation {
        kind_: i8,
    }

    impl Representation {
        pub const K_NONE: i8 = 0;
        pub const K_SMI: i8 = 1;
        pub const K_DOUBLE: i8 = 2;
        pub const K_HEAP_OBJECT: i8 = 3;
        pub const K_TAGGED: i8 = 4;
        pub const K_WASM_VALUE: i8 = 5;
        pub const K_NUM_REPRESENTATIONS: i8 = 6;

        pub const fn new() -> Self {
            Representation { kind_: Self::K_NONE }
        }

        pub const fn none() -> Self {
            Representation { kind_: Self::K_NONE }
        }

        pub const fn tagged() -> Self {
            Representation { kind_: Self::K_TAGGED }
        }

        pub const fn smi() -> Self {
            Representation { kind_: Self::K_SMI }
        }

        pub const fn double() -> Self {
            Representation { kind_: Self::K_DOUBLE }
        }

        pub const fn heap_object() -> Self {
            Representation { kind_: Self::K_HEAP_OBJECT }
        }

        pub const fn wasm_value() -> Self {
            Representation { kind_: Self::K_WASM_VALUE }
        }

        pub const fn from_kind(kind: i8) -> Self {
            Representation { kind_: kind }
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

        pub fn might_cause_map_deprecation(&self) -> bool {
            if self.is_tagged() || self.is_heap_object() || self.is_double() || self.is_wasm_value() {
                return false;
            }
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

            if self.is_none() {
                return !other.is_double();
            }
            if !other.is_tagged() {
                return false;
            }
            assert!(self.is_smi() || self.is_double() || self.is_heap_object());
            true
        }

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
                return K_DOUBLE_SIZE;
            }
            assert!(self.is_tagged() || self.is_smi() || self.is_heap_object());
            K_TAGGED_SIZE
        }

        pub const fn kind(&self) -> i8 {
            self.kind_ as i8
        }
        pub const fn is_none(&self) -> bool {
            self.kind_ == Self::K_NONE
        }
        pub const fn is_wasm_value(&self) -> bool {
            self.kind_ == Self::K_WASM_VALUE
        }
        pub const fn is_tagged(&self) -> bool {
            self.kind_ == Self::K_TAGGED
        }
        pub const fn is_smi(&self) -> bool {
            self.kind_ == Self::K_SMI
        }
        pub const fn is_smi_or_tagged(&self) -> bool {
            self.is_smi() || self.is_tagged()
        }
        pub const fn is_double(&self) -> bool {
            self.kind_ == Self::K_DOUBLE
        }
        pub const fn is_heap_object(&self) -> bool {
            self.kind_ == Self::K_HEAP_OBJECT
        }

        pub fn mnemonic(&self) -> &'static str {
            match self.kind_ {
                Self::K_NONE => "v",
                Self::K_TAGGED => "t",
                Self::K_SMI => "s",
                Self::K_DOUBLE => "d",
                Self::K_HEAP_OBJECT => "h",
                Self::K_WASM_VALUE => "w",
                _ => unreachable!(),
            }
        }
    }

    pub const K_DESCRIPTOR_INDEX_BIT_COUNT: i32 = 10;
    pub const K_FIRST_INOBJECT_PROPERTY_OFFSET_BIT_COUNT: i32 = 7;

    // The maximum number of descriptors we want in a descriptor array.  It should
    // fit in a page and also the following should hold:
    // kMaxNumberOfDescriptors + kFieldsAdded <= PropertyArray::kMaxLength.
    pub const K_MAX_NUMBER_OF_DESCRIPTORS: i32 = (1 << K_DESCRIPTOR_INDEX_BIT_COUNT) - 4;
    pub const K_INVALID_ENUM_CACHE_SENTINEL: i32 = (1 << K_DESCRIPTOR_INDEX_BIT_COUNT) - 1;

    // A PropertyCell's property details contains a cell type that is meaningful if
    // the cell is still valid (does not hold the hole).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum PropertyCellType {
        Mutable = 0,       // Cell will no longer be tracked as constant.
        Undefined = 1,     // The PREMONOMORPHIC of property cells.
        Constant = 2,      // Cell has been assigned only once.
        ConstantType = 3,  // Cell has been assigned only one type.
        // Temporary value indicating an ongoing property cell state transition. Only
        // observable by a background thread.
        InTransition = 4,
        // Value for dictionaries not holding cells, must be 0:
        NoCell = Mutable as u8,
    }

    // PropertyDetails captures type and attributes for a property.
    // They are used both in property dictionaries and instance descriptors.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
                value_: KindField::encode(kind)
                    | LocationField::encode(PropertyLocation::Field)
                    | AttributesField::encode(attributes)
                    | ConstnessField::encode(PropertyConstness::Mutable)
                    | DictionaryStorageField::encode(dictionary_index as u32)
                    | PropertyCellTypeField::encode(cell_type),
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
                value_: KindField::encode(kind)
                    | LocationField::encode(PropertyLocation::Field)
                    | AttributesField::encode(attributes)
                    | ConstnessField::encode(constness)
                    | DictionaryStorageField::encode(dictionary_index as u32)
                    | PropertyCellTypeField::encode(PropertyCellType::NoCell),
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
                value_: KindField::encode(kind)
                    | AttributesField::encode(attributes)
                    | LocationField::encode(location)
                    | ConstnessField::encode(constness)
                    | RepresentationField::encode(Self::encode_representation(representation))
                    | FieldIndexField::encode(field_index as u32),
            }
        }

        pub const fn empty(cell_type: PropertyCellType) -> Self {
            PropertyDetails {
                value_: KindField::encode(PropertyKind::Data)
                    | LocationField::encode(PropertyLocation::Field) // Assuming Field is a reasonable default
                    | AttributesField::encode(PropertyAttributes::None)
                    | ConstnessField::encode(PropertyConstness::Mutable) // Default
                    | DictionaryStorageField::encode(0) //Default
                    | PropertyCellTypeField::encode(cell_type),
            }
        }

        pub fn pointer(&self) -> i32 {
            DescriptorPointer::decode(self.value_) as i32
        }

        pub fn set_pointer(&self, i: i32) -> Self {
            PropertyDetails {
                value_: DescriptorPointer::update(self.value_, i as u32),
            }
        }

        pub fn set_cell_type(&self, type_: PropertyCellType) -> Self {
            PropertyDetails {
                value_: PropertyCellTypeField::update(self.value_, type_),
            }
        }

        pub fn set_index(&self, index: i32) -> Self {
            PropertyDetails {
                value_: DictionaryStorageField::update(self.value_, index as u32),
            }
        }

        pub fn copy_with_representation(&self, representation: Representation) -> Self {
            PropertyDetails {
                value_: RepresentationField::update(
                    self.value_,
                    Self::encode_representation(representation),
                ),
            }
        }

        pub fn copy_with_constness(&self, constness: PropertyConstness) -> Self {
            PropertyDetails {
                value_: ConstnessField::update(self.value_, constness),
            }
        }

        pub fn copy_add_attributes(&self, new_attributes: PropertyAttributes) -> Self {
            let new_attributes_val = (self.attributes() as i32 | new_attributes as i32) as i32;
            let new_attributes_enum = match new_attributes_val {
                NONE => PropertyAttributes::None,
                READ_ONLY => PropertyAttributes::ReadOnly,
                DONT_ENUM => PropertyAttributes::DontEnum,
                DONT_DELETE => PropertyAttributes::DontDelete,
                _ => property_attributes_from_int(new_attributes_val),
            };

            PropertyDetails {
                value_: AttributesField::update(self.value_, new_attributes_enum),
            }
        }
        // Conversion for storing details as Object.
        //  explicit inline PropertyDetails(Tagged<Smi> smi);
        //  inline Tagged<Smi> AsSmi() const;
        // TODO Implement Tagged<Smi> and conversion if needed.

        pub const fn encode_representation(representation: Representation) -> u32 {
            representation.kind() as u32
        }

        pub fn decode_representation(bits: u32) -> Representation {
            Representation::from_kind(bits as i8)
        }

        pub fn kind(&self) -> PropertyKind {
            KindField::decode(self.value_)
        }
        pub fn location(&self) -> PropertyLocation {
            LocationField::decode(self.value_)
        }
        pub fn constness(&self) -> PropertyConstness {
            ConstnessField::decode(self.value_)
        }

        pub fn attributes(&self) -> PropertyAttributes {
            AttributesField::decode(self.value_)
        }

        pub fn has_kind_and_attributes(&self, kind: PropertyKind, attributes: PropertyAttributes) -> bool {
            (self.value_ & (KindField::k_mask | AttributesField::k_mask))
                == (KindField::encode(kind) | AttributesField::encode(attributes))
        }

        pub fn dictionary_index(&self) -> i32 {
            DictionaryStorageField::decode(self.value_) as i32
        }

        pub fn representation(&self) -> Representation {
            Self::decode_representation(RepresentationField::decode(self.value_))
        }

        pub fn field_index(&self) -> i32 {
            FieldIndexField::decode(self.value_) as i32
        }

        pub fn field_width_in_words(&self) -> i32 {
            todo!()
        }

        pub fn is_valid_index(index: i32) -> bool {
            DictionaryStorageField::is_valid(index as u32)
        }

        pub fn is_read_only(&self) -> bool {
            (self.attributes() as i32 & READ_ONLY) != 0
        }
        pub fn is_configurable(&self) -> bool {
            (self.attributes() as i32 & DONT_DELETE) == 0
        }
        pub fn is_dont_enum(&self) -> bool {
            (self.attributes() as i32 & DONT_ENUM) != 0
        }
        pub fn is_enumerable(&self) -> bool {
            !self.is_dont_enum()
        }
        pub fn cell_type(&self) -> PropertyCellType {
            PropertyCellTypeField::decode(self.value_)
        }

        pub const K_INITIAL_INDEX: i32 = 1;
    }

    impl PropertyDetails {
        fn new_with_value(value: u32) -> Self {
            PropertyDetails { value_: value }
        }

        fn new_with_value_pointer(value: u32, pointer: i32) -> Self {
            PropertyDetails {
                value_: DescriptorPointer::update(value, pointer as u32),
            }
        }

        fn new_with_value_representation(value: u32, representation: Representation) -> Self {
            PropertyDetails {
                value_: RepresentationField::update(
                    value,
                    Self::encode_representation(representation),
                ),
            }
        }

        fn new_with_value_constness(value: u32, constness: PropertyConstness) -> Self {
            PropertyDetails {
                value_: ConstnessField::update(value, constness),
            }
        }

        fn new_with_value_attributes(value: u32, attributes: PropertyAttributes) -> Self {
            PropertyDetails {
                value_: AttributesField::update(value, attributes),
            }
        }

        pub fn to_byte(&self) -> u8 {
            assert_eq!(PropertyLocation::Field, self.location());
            assert_eq!(PropertyCellType::NoCell, self.cell_type());
            assert_eq!(0, self.dictionary_index());
            self.value_ as u8
        }

        pub fn from_byte(encoded_details: u8) -> Self {
            let details = PropertyDetails::new_with_value(encoded_details as u32);
            assert_eq!(PropertyLocation::Field, details.location());
            assert_eq!(PropertyCellType::NoCell, details.cell_type());
            assert_eq!(0, details.dictionary_index());
            details
        }
    }

    pub mod base {
        pub struct BitField<T, const START: usize, const SIZE: usize>;

        impl<T, const START: usize, const SIZE: usize> BitField<T, START, SIZE> {
            pub const k_shift: u32 = START as u32;
            pub const k_size: u32 = SIZE as u32;
            pub const k_mask: u32 = ((1 << SIZE) - 1) << START;

            pub const fn encode(value: T) -> u32
            where
                T: Into<u32>,
            {
                value.into() << START
            }

            pub fn decode(bits: u32) -> T
            where
                T: From<u32>,
            {
                ((bits & Self::k_mask) >> START).into()
            }

            pub const fn update(bits: u32, value: T) -> u32
            where
                T: Into<u32>,
            {
                (bits & !Self::k_mask) | (value.into() << START)
            }

            pub type Next<U, const NEXT_SIZE: usize> = BitField<U, { START + SIZE }, NEXT_SIZE>;
        }
    }

    use base::BitField;

    impl From<PropertyKind> for u32 {
        fn from(kind: PropertyKind) -> Self {
            kind as u32
        }
    }

    impl From<PropertyAttributes> for u32 {
        fn from(attr: PropertyAttributes) -> Self {
            attr as u32
        }
    }

    impl From<PropertyLocation> for u32 {
        fn from(location: PropertyLocation) -> Self {
            location as u32
        }
    }

    impl From<PropertyConstness> for u32 {
        fn from(constness: PropertyConstness) -> Self {
            constness as u32
        }
    }

    impl From<PropertyCellType> for u32 {
        fn from(cell_type: PropertyCellType) -> Self {
            cell_type as u32
        }
    }

    impl From<u32> for PropertyKind {
        fn from(value: u32) -> Self {
            match value {
                0 => PropertyKind::Data,
                1 => PropertyKind::Accessor,
                _ => panic!("Invalid PropertyKind value"),
            }
        }
    }

    impl From<u32> for PropertyAttributes {
        fn from(value: u32) -> Self {
            match value {
                0 => PropertyAttributes::None,
                1 => PropertyAttributes::ReadOnly,
                2 => PropertyAttributes::DontEnum,
                4 => PropertyAttributes::DontDelete,
                _ => panic!("Invalid PropertyAttributes value"),
            }
        }
    }

    impl From<u32> for PropertyLocation {
        fn from(value: u32) -> Self {
            match value {
                0 => PropertyLocation::Field,
                1 => PropertyLocation::Descriptor,
                _ => panic!("Invalid PropertyLocation value"),
            }
        }
    }

    impl From<u32> for PropertyConstness {
        fn from(value: u32) -> Self {
            match value {
                0 => PropertyConstness::Mutable,
                1 => PropertyConstness::Const,
                _ => panic!("Invalid PropertyConstness value"),
            }
        }
    }

    impl From<u32> for PropertyCellType {
        fn from(value: u32) -> Self {
            match value {
                0 => PropertyCellType::Mutable,
                1 => PropertyCellType::Undefined,
                2 => PropertyCellType::Constant,
                3 => PropertyCellType::ConstantType,
                4 => PropertyCellType::InTransition,
                _ => panic!("Invalid PropertyCellType value"),
            }
        }
    }

    pub type KindField = BitField<PropertyKind, 0, 1>;
    pub type ConstnessField = KindField::Next<PropertyConstness, 1>;
    pub type AttributesField = ConstnessField::Next<PropertyAttributes, 3>;
    pub const K_ATTRIBUTES_READ_ONLY_MASK: u32 = (READ_ONLY as u32) << AttributesField::k_shift;
    pub const K_ATTRIBUTES_DONT_DELETE_MASK: u32 = (DONT_DELETE as u32) << AttributesField::k_shift;
    pub const K_ATTRIBUTES_DONT_ENUM_MASK: u32 = (DONT_ENUM as u32) << AttributesField::k_shift;

    // Bit fields for normalized/dictionary mode objects.
    pub type PropertyCellTypeField = AttributesField::Next<PropertyCellType, 3>;
    pub type DictionaryStorageField = PropertyCellTypeField::Next<u32, 23>;

    // Bit fields for fast objects.
    pub type LocationField = AttributesField::Next<PropertyLocation, 1>;
    pub type RepresentationField = LocationField::Next<u32, 3>;
    pub type DescriptorPointer = RepresentationField::Next<u32, K_DESCRIPTOR_INDEX_BIT_COUNT as usize>;
    pub type FieldIndexField = DescriptorPointer::Next<u32, K_DESCRIPTOR_INDEX_BIT_COUNT as usize>;

    // All bits for both fast and slow objects must fit in a smi.
    // static_assert(DictionaryStorageField::kLastUsedBit < 31);
    // static_assert(FieldIndexField::kLastUsedBit < 31);

    // DictionaryStorageField must be the last field, so that overflowing it
    // doesn't overwrite other fields.
    // static_assert(DictionaryStorageField::kLastUsedBit == 30);

    // All bits for non-global dictionary mode objects except enumeration index
    // must fit in a byte.
    // static_assert(KindField::kLastUsedBit < 8);
    // static_assert(ConstnessField::kLastUsedBit < 8);
    // static_assert(AttributesField::kLastUsedBit < 8);

    pub const V8_DICT_PROPERTY_CONST_TRACKING_BOOL: bool = false;
    pub const K_CONST_IF_DICT_CONSTNESS_TRACKING: PropertyConstness =
        if V8_DICT_PROPERTY_CONST_TRACKING_BOOL {
            PropertyConstness::Const
        } else {
            PropertyConstness::Mutable
        };

    pub enum PrintMode {
        PrintAttributes = 1 << 0,
        PrintFieldIndex = 1 << 1,
        PrintRepresentation = 1 << 2,
        PrintPointer = 1 << 3,

        ForProperties = PrintFieldIndex as isize | PrintAttributes as isize,
        ForTransitions = PrintAttributes as isize,
        PrintFull = -1,
    }

    impl fmt::Display for Representation {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.mnemonic())
        }
    }

    impl fmt::Display for PropertyAttributes {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                PropertyAttributes::None => write!(f, "None"),
                PropertyAttributes::ReadOnly => write!(f, "ReadOnly"),
                PropertyAttributes::DontEnum => write!(f, "DontEnum"),
                PropertyAttributes::DontDelete => write!(f, "DontDelete"),
            }
        }
    }

    impl fmt::Display for PropertyConstness {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                PropertyConstness::Const => write!(f, "Const"),
                PropertyConstness::Mutable => write!(f, "Mutable"),
            }
        }
    }

    impl fmt::Display for PropertyCellType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                PropertyCellType::Mutable => write!(f, "Mutable"),
                PropertyCellType::Undefined => write!(f, "Undefined"),
                PropertyCellType::Constant => write!(f, "Constant"),
                PropertyCellType::ConstantType => write!(f, "ConstantType"),
                PropertyCellType::InTransition => write!(f, "InTransition"),
                PropertyCellType::NoCell => write!(f, "NoCell"),
            }
        }
    }

    // kField location is more general than kDescriptor, kDescriptor generalizes
    // only to itself.
    pub fn is_generalizable_to(a: PropertyLocation, b: PropertyLocation) -> bool {
        b == PropertyLocation::Field || a == PropertyLocation::Descriptor
    }

    // PropertyConstness::kMutable constness is more general than
    // VariableMode::kConst, VariableMode::kConst generalizes only to itself.
    pub fn is_generalizable_to_constness(a: PropertyConstness, b: PropertyConstness) -> bool {
        b == PropertyConstness::Mutable || a == PropertyConstness::Const
    }

    pub fn generalize_constness(a: PropertyConstness, b: PropertyConstness) -> PropertyConstness {
        if a == PropertyConstness::Mutable {
            PropertyConstness::Mutable
        } else {
            b
        }
    }

    pub const K_DOUBLE_SIZE: i32 = 8;
    pub const K_TAGGED_SIZE: i32 = 4;
}