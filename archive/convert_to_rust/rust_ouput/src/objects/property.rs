// Converted from V8 C++ source files:
// Header: property.h
// Implementation: property.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod property {
    use std::fmt;
    use crate::PropertyAttributes;
    use crate::PropertyConstness;
    use crate::PropertyKind;
    use crate::PropertyLocation;
    use crate::Representation;
    use crate::v8::internal::PtrComprCageBase;
    use crate::v8::internal::GetPtrComprCageBase;
    use crate::v8::internal::IsUniqueName;
    use crate::v8::internal::Name;
    use crate::v8::internal::Object;
    use crate::v8::internal::Smi;
    use crate::v8::internal::IsSmi;
    use crate::v8::internal::IsWeak;
    use crate::v8::internal::FieldType;
    use crate::v8::internal::Isolate;
    use crate::v8::internal::TaggedField;
    use crate::v8::internal::Tagged;
    use crate::v8::internal::Code;

    pub struct Descriptor {
        key_: DirectHandle<Name>,
        value_: MaybeObjectDirectHandle,
        details_: PropertyDetails,
    }

    impl Descriptor {
        pub fn new() -> Self {
            Descriptor {
                key_: DirectHandle::empty(),
                value_: MaybeObjectDirectHandle::empty(),
                details_: PropertyDetails::new(),
            }
        }

        pub fn GetKey(&self) -> DirectHandle<Name> {
            self.key_.clone()
        }

        pub fn GetValue(&self) -> MaybeObjectDirectHandle {
            self.value_.clone()
        }

        pub fn GetDetails(&self) -> PropertyDetails {
            self.details_.clone()
        }

        pub fn SetSortedKeyIndex(&mut self, index: i32) {
            self.details_.set_pointer(index);
        }

        pub fn DataField(isolate: *mut Isolate, key: DirectHandle<Name>,
                             field_index: i32, attributes: PropertyAttributes,
                             representation: Representation) -> Self {
            Self::DataField_full(key, field_index, attributes, PropertyConstness::kMutable,
                                 representation,
                                 MaybeObjectDirectHandle::empty()) //TODO: fix MaybeObjectDirectHandle(FieldType::Any(isolate))
        }

        pub fn DataField_full(
            key: DirectHandle<Name>, field_index: i32, attributes: PropertyAttributes,
            constness: PropertyConstness, representation: Representation,
            wrapped_field_type: MaybeObjectDirectHandle) -> Self {
            PropertyDetails::assert_smi_or_weak(&wrapped_field_type);
            let details = PropertyDetails::new_data_field(attributes, constness, representation, field_index);
            Descriptor::new_with_details(key, wrapped_field_type, details)
        }

        pub fn DataConstant(key: DirectHandle<Name>,
                                    value: DirectHandle<Object>,
                                    attributes: PropertyAttributes) -> Self {
            let cage_base: PtrComprCageBase = GetPtrComprCageBase(*key); //TODO: fix *key
            Descriptor::new_with_all(key, MaybeObjectDirectHandle::empty(), //TODO: fix MaybeObjectDirectHandle(value)
                                     PropertyKind::kData,
                                     attributes, PropertyLocation::kDescriptor,
                                     PropertyConstness::kConst,
                                     Representation::kTagged(), 0) //TODO: fix Object::OptimalRepresentation(*value, cage_base)
        }

         pub fn DataConstant_with_isolate(isolate: *mut Isolate, key: DirectHandle<Name>,
                                            field_index: i32, value: DirectHandle<Object>,
                                            attributes: PropertyAttributes) -> Self {
            let any_type = MaybeObjectDirectHandle::empty(); //TODO: fix MaybeObjectDirectHandle(FieldType::Any(), isolate);
            Descriptor::DataField_full(key, field_index, attributes, PropertyConstness::kConst,
                                        Representation::kTagged(), any_type)
        }

        pub fn AccessorConstant(key: DirectHandle<Name>,
                                        foreign: DirectHandle<Object>,
                                        attributes: PropertyAttributes) -> Self {
            Descriptor::new_with_all(key, MaybeObjectDirectHandle::empty(), //TODO: fix MaybeObjectDirectHandle(foreign)
                                     PropertyKind::kAccessor, attributes,
                                     PropertyLocation::kDescriptor, PropertyConstness::kConst,
                                     Representation::kTagged(), 0)
        }

        fn new_with_details(key: DirectHandle<Name>, value: MaybeObjectDirectHandle, details: PropertyDetails) -> Self {
            assert_unique_name(&key);
            Descriptor {
                key_: key,
                value_: value,
                details_: details,
            }
        }

        fn new_with_all(key: DirectHandle<Name>, value: MaybeObjectDirectHandle,
                            kind: PropertyKind, attributes: PropertyAttributes,
                            location: PropertyLocation, constness: PropertyConstness,
                            representation: Representation, field_index: i32) -> Self {
            assert_unique_name(&key);
            Descriptor {
                key_: key,
                value_: value,
                details_: PropertyDetails::new_all(kind, attributes, location, constness, representation, field_index),
            }
        }
    }

    fn assert_unique_name(key: &DirectHandle<Name>) {
         if let DirectHandle::Some(name) = key{
             assert!(IsUniqueName(*name));
         }
    }

    #[derive(Clone, Copy)]
    pub struct PropertyDetails {
        kind: PropertyKind,
        attributes: PropertyAttributes,
        location: PropertyLocation,
        constness: PropertyConstness,
        representation: Representation,
        field_index: i32,
        pointer: i32,
    }

    impl PropertyDetails {
        pub fn new() -> Self {
            PropertyDetails {
                kind: PropertyKind::kData,
                attributes: PropertyAttributes::NONE,
                location: PropertyLocation::kField,
                constness: PropertyConstness::kMutable,
                representation: Representation::kTagged(),
                field_index: 0,
                pointer: 0,
            }
        }

        pub fn new_data_field(attributes: PropertyAttributes, constness: PropertyConstness, representation: Representation, field_index: i32) -> Self {
            PropertyDetails {
                kind: PropertyKind::kData,
                attributes: attributes,
                location: PropertyLocation::kField,
                constness: constness,
                representation: representation,
                field_index: field_index,
                pointer: 0,
            }
        }

        pub fn new_all(kind: PropertyKind, attributes: PropertyAttributes,
                           location: PropertyLocation, constness: PropertyConstness,
                           representation: Representation, field_index: i32) -> Self {
            PropertyDetails {
                kind: kind,
                attributes: attributes,
                location: location,
                constness: constness,
                representation: representation,
                field_index: field_index,
                pointer: 0,
            }
        }

        pub fn kind(&self) -> PropertyKind {
            self.kind
        }

        pub fn attributes(&self) -> PropertyAttributes {
            self.attributes
        }

        pub fn location(&self) -> PropertyLocation {
            self.location
        }

        pub fn constness(&self) -> PropertyConstness {
            self.constness
        }

        pub fn representation(&self) -> Representation {
            self.representation
        }

        pub fn field_index(&self) -> i32 {
            self.field_index
        }

        pub fn pointer(&self) -> i32 {
            self.pointer
        }

        pub fn dictionary_index(&self) -> i32 {
            self.pointer
        }

        pub fn set_pointer(&mut self, pointer: i32) -> PropertyDetails {
             self.pointer = pointer;
             *self
        }

        pub fn IsEnumerable(&self) -> bool {
            (self.attributes & PropertyAttributes::DONT_ENUM) == PropertyAttributes::NONE
        }

        fn assert_smi_or_weak(wrapped_field_type: &MaybeObjectDirectHandle) {
             //TODO: fix this
           // assert!(IsSmi(*wrapped_field_type) || IsWeak(*wrapped_field_type));
        }
    }

    impl PropertyDetails {
        pub fn PrintAsSlowTo(&self, os: &mut std::fmt::Formatter, print_dict_index: bool) -> fmt::Result {
            write!(os, "(")?;
            if self.constness() == PropertyConstness::kConst {
                write!(os, "const ")?;
            }
            write!(os, "{}", if self.kind() == PropertyKind::kData { "data" } else { "accessor" })?;
            if print_dict_index {
                write!(os, ", dict_index: {}", self.dictionary_index())?;
            }
            write!(os, ", attrs: {:?})", self.attributes())
        }

        pub fn PrintAsFastTo(&self, os: &mut std::fmt::Formatter, mode: PrintMode) -> fmt::Result {
            write!(os, "(")?;
            if self.constness() == PropertyConstness::kConst {
                write!(os, "const ")?;
            }
            write!(os, "{}", if self.kind() == PropertyKind::kData { "data" } else { "accessor" })?;
            if self.location() == PropertyLocation::kField {
                write!(os, " field")?;
                if mode.contains(PrintMode::kPrintFieldIndex) {
                    write!(os, " {}", self.field_index())?;
                }
                if mode.contains(PrintMode::kPrintRepresentation) {
                    write!(os, ":{}", self.representation().Mnemonic())?;
                }
            } else {
                write!(os, " descriptor")?;
            }
            if mode.contains(PrintMode::kPrintPointer) {
                write!(os, ", p: {}", self.pointer())?;
            }
            if mode.contains(PrintMode::kPrintAttributes) {
                write!(os, ", attrs: {:?}", self.attributes())?;
            }
            write!(os, ")")
        }
    }

    impl fmt::Debug for PropertyDetails {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.PrintAsFastTo(f, PrintMode::kPrintFull)
        }
    }

    bitflags::bitflags! {
        #[derive(Clone, Copy, Debug)]
        pub struct PrintMode: i32 {
            const kPrintNone = 0;
            const kPrintFieldIndex = 1 << 0;
            const kPrintRepresentation = 1 << 1;
            const kPrintPointer = 1 << 2;
            const kPrintAttributes = 1 << 3;
            const kPrintFull = Self::kPrintFieldIndex.bits | Self::kPrintRepresentation.bits | Self::kPrintPointer.bits | Self::kPrintAttributes.bits;
        }
    }

    impl Representation {
        pub fn Mnemonic(&self) -> &'static str {
            match self {
                Representation::kNone => "none",
                Representation::kSmi => "smi",
                Representation::kDouble => "double",
                Representation::kHeapObject => "heap-object",
                Representation::kTagged => "tagged",
                Representation::kWasmValue => "wasm-value",
                Representation::kNumRepresentations => "num-representations",
            }
        }
    }

    impl fmt::Display for Representation {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.Mnemonic())
        }
    }

    impl fmt::Display for PropertyAttributes {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "[")?;
            write!(f, "{}", if !self.contains(PropertyAttributes::READ_ONLY) { "W" } else { "_" })?;
            write!(f, "{}", if !self.contains(PropertyAttributes::DONT_ENUM) { "E" } else { "_" })?;
            write!(f, "{}", if !self.contains(PropertyAttributes::DONT_DELETE) { "C" } else { "_" })?;
            write!(f, "]")
        }
    }

    impl fmt::Display for PropertyConstness {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                PropertyConstness::kMutable => write!(f, "mutable"),
                PropertyConstness::kConst => write!(f, "const"),
            }
        }
    }

    #[derive(Clone)]
    pub enum DirectHandle<T> {
        Some(T),
        None,
    }

    impl<T> DirectHandle<T> {
        pub fn empty() -> Self {
            DirectHandle::None
        }
    }

    #[derive(Clone)]
    pub enum MaybeObjectDirectHandle {
        Some(Object),
        None,
    }

    impl MaybeObjectDirectHandle {
        pub fn empty() -> Self {
            MaybeObjectDirectHandle::None
        }
    }
}
