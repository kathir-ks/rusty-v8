// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Note: v8-internal.h and objects-definitions.h are V8 internal headers and are not directly translatable.
// They contain internal data structures and configurations.  The Rust equivalent would involve defining
// similar structures and constants based on the requirements.
// Torque-generated files are auto-generated code, so their direct translation depends on the Torque definitions.

// src/objects/object-macros.h is used for defining object accessors and is not directly translatable.
// The Rust equivalent would involve defining similar accessors using methods on the struct.

// This translation provides a basic structure mimicking the original C++ header.
// Further refinement would depend on the actual implementation of V8 internals.

pub mod instance_type {
    /// We use the full 16 bits of the instance_type field to encode heap object
    /// instance types. All the high-order bits (bits 7-15) are cleared if the object
    /// is a string, and contain set bits if it is not a string.
    pub const IS_NOT_STRING_MASK: u32 = !((1 << 7) - 1);
    pub const STRING_TAG: u32 = 0x0;

    /// For strings, bits 0-2 indicate the representation of the string. In
    /// particular, bit 0 indicates whether the string is direct or indirect.
    pub const STRING_REPRESENTATION_MASK: u32 = (1 << 3) - 1;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[repr(u32)]
    pub enum StringRepresentationTag {
        SeqStringTag = 0x0,
        ConsStringTag = 0x1,
        ExternalStringTag = 0x2,
        SlicedStringTag = 0x3,
        ThinStringTag = 0x5,
    }

    pub const IS_INDIRECT_STRING_MASK: u32 = 1 << 0;
    pub const IS_INDIRECT_STRING_TAG: u32 = 1 << 0;

    const _: () = assert!(
        (StringRepresentationTag::SeqStringTag as u32 & IS_INDIRECT_STRING_MASK) == 0
    );
    const _: () = assert!(
        (StringRepresentationTag::ExternalStringTag as u32 & IS_INDIRECT_STRING_MASK) == 0
    );
    const _: () = assert!(
        (StringRepresentationTag::ConsStringTag as u32 & IS_INDIRECT_STRING_MASK)
            == IS_INDIRECT_STRING_TAG
    );
    const _: () = assert!(
        (StringRepresentationTag::SlicedStringTag as u32 & IS_INDIRECT_STRING_MASK)
            == IS_INDIRECT_STRING_TAG
    );
    const _: () = assert!(
        (StringRepresentationTag::ThinStringTag as u32 & IS_INDIRECT_STRING_MASK)
            == IS_INDIRECT_STRING_TAG
    );

    pub const THIN_STRING_TAG_BIT: u32 = 1 << 2;

    const _: () = assert!((StringRepresentationTag::SeqStringTag as u32 & THIN_STRING_TAG_BIT) == 0);
    const _: () = assert!((StringRepresentationTag::ConsStringTag as u32 & THIN_STRING_TAG_BIT) == 0);
    const _: () = assert!((StringRepresentationTag::ExternalStringTag as u32 & THIN_STRING_TAG_BIT) == 0);
    const _: () = assert!((StringRepresentationTag::SlicedStringTag as u32 & THIN_STRING_TAG_BIT) == 0);
    const _: () = assert!((StringRepresentationTag::ThinStringTag as u32 & THIN_STRING_TAG_BIT) == THIN_STRING_TAG_BIT);

    /// For strings, bit 3 indicates whether the string consists of two-byte
    /// characters or one-byte characters.
    pub const STRING_ENCODING_MASK: u32 = 1 << 3;
    pub const TWO_BYTE_STRING_TAG: u32 = 0;
    pub const ONE_BYTE_STRING_TAG: u32 = 1 << 3;

    /// Combined tags for convenience (add more if needed).
    pub const STRING_REPRESENTATION_AND_ENCODING_MASK: u32 =
        STRING_REPRESENTATION_MASK | STRING_ENCODING_MASK;
    pub const SEQ_ONE_BYTE_STRING_TAG: u32 =
        StringRepresentationTag::SeqStringTag as u32 | ONE_BYTE_STRING_TAG;
    pub const SEQ_TWO_BYTE_STRING_TAG: u32 =
        StringRepresentationTag::SeqStringTag as u32 | TWO_BYTE_STRING_TAG;
    pub const EXTERNAL_ONE_BYTE_STRING_TAG: u32 =
        StringRepresentationTag::ExternalStringTag as u32 | ONE_BYTE_STRING_TAG;
    pub const EXTERNAL_TWO_BYTE_STRING_TAG: u32 =
        StringRepresentationTag::ExternalStringTag as u32 | TWO_BYTE_STRING_TAG;

    /// For strings, bit 4 indicates whether the data pointer of an external string
    /// is cached. Note that the string representation is expected to be
    /// `StringRepresentationTag::ExternalStringTag`.
    pub const UNCACHED_EXTERNAL_STRING_MASK: u32 = 1 << 4;
    pub const UNCACHED_EXTERNAL_STRING_TAG: u32 = 1 << 4;

    /// For strings, bit 5 indicates that the string is internalized (if not set) or
    /// isn't (if set).
    pub const IS_NOT_INTERNALIZED_MASK: u32 = 1 << 5;
    pub const NOT_INTERNALIZED_TAG: u32 = 1 << 5;
    pub const INTERNALIZED_TAG: u32 = 0;

    /// For strings, bit 6 indicates that the string is accessible by more than one
    /// thread. Note that a string that is allocated in the shared heap is not
    /// accessible by more than one thread until it is explicitly shared (e.g. by
    /// postMessage).
    ///
    /// Runtime code that shares strings with other threads directly need to manually
    /// set this bit.
    ///
    /// TODO(v8:12007): External strings cannot be shared yet.
    ///
    /// TODO(v8:12007): This bit is currently ignored on internalized strings, which
    /// are either always shared or always not shared depending on
    /// v8_flags.shared_string_table. This will be hardcoded once
    /// v8_flags.shared_string_table is removed.
    pub const SHARED_STRING_MASK: u32 = 1 << 6;
    pub const SHARED_STRING_TAG: u32 = 1 << 6;

    pub const STRING_REPRESENTATION_ENCODING_AND_SHARED_MASK: u32 =
        STRING_REPRESENTATION_AND_ENCODING_MASK | SHARED_STRING_MASK;

    /// A ConsString with an empty string as the right side is a candidate
    /// for being shortcut by the garbage collector. We don't allocate any
    /// non-flat internalized strings, so we do not shortcut them thereby
    /// avoiding turning internalized strings into strings. The bit-masks
    /// below contain the internalized bit as additional safety.
    /// See heap.cc, mark-compact.cc and heap-visitor.cc.
    pub const SHORTCUT_TYPE_MASK: u32 =
        IS_NOT_STRING_MASK | IS_NOT_INTERNALIZED_MASK | STRING_REPRESENTATION_MASK;
    pub const SHORTCUT_TYPE_TAG: u32 =
        StringRepresentationTag::ConsStringTag as u32 | NOT_INTERNALIZED_TAG;

    pub fn is_shortcut_candidate(type_: i32) -> bool {
        ((type_ as u32 & SHORTCUT_TYPE_MASK) == SHORTCUT_TYPE_TAG)
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[repr(u16)]
    pub enum InstanceType {
        // String types.
        InternalizedTwoByteStringType =
            TWO_BYTE_STRING_TAG as u16 | StringRepresentationTag::SeqStringTag as u16 | INTERNALIZED_TAG as u16,
        InternalizedOneByteStringType =
            ONE_BYTE_STRING_TAG as u16 | StringRepresentationTag::SeqStringTag as u16 | INTERNALIZED_TAG as u16,
        ExternalInternalizedTwoByteStringType =
            TWO_BYTE_STRING_TAG as u16 | StringRepresentationTag::ExternalStringTag as u16 | INTERNALIZED_TAG as u16,
        ExternalInternalizedOneByteStringType =
            ONE_BYTE_STRING_TAG as u16 | StringRepresentationTag::ExternalStringTag as u16 | INTERNALIZED_TAG as u16,
        UncachedExternalInternalizedTwoByteStringType =
            ExternalInternalizedTwoByteStringType as u16 | UNCACHED_EXTERNAL_STRING_TAG as u16,
        UncachedExternalInternalizedOneByteStringType =
            ExternalInternalizedOneByteStringType as u16 | UNCACHED_EXTERNAL_STRING_TAG as u16,
        SeqTwoByteStringType =
            InternalizedTwoByteStringType as u16 | NOT_INTERNALIZED_TAG as u16,
        SeqOneByteStringType =
            InternalizedOneByteStringType as u16 | NOT_INTERNALIZED_TAG as u16,
        ConsTwoByteStringType =
            TWO_BYTE_STRING_TAG as u16 | StringRepresentationTag::ConsStringTag as u16 | NOT_INTERNALIZED_TAG as u16,
        ConsOneByteStringType =
            ONE_BYTE_STRING_TAG as u16 | StringRepresentationTag::ConsStringTag as u16 | NOT_INTERNALIZED_TAG as u16,
        SlicedTwoByteStringType =
            TWO_BYTE_STRING_TAG as u16 | StringRepresentationTag::SlicedStringTag as u16 | NOT_INTERNALIZED_TAG as u16,
        SlicedOneByteStringType =
            ONE_BYTE_STRING_TAG as u16 | StringRepresentationTag::SlicedStringTag as u16 | NOT_INTERNALIZED_TAG as u16,
        ExternalTwoByteStringType =
            ExternalInternalizedTwoByteStringType as u16 | NOT_INTERNALIZED_TAG as u16,
        ExternalOneByteStringType =
            ExternalInternalizedOneByteStringType as u16 | NOT_INTERNALIZED_TAG as u16,
        UncachedExternalTwoByteStringType =
            UncachedExternalInternalizedTwoByteStringType as u16 | NOT_INTERNALIZED_TAG as u16,
        UncachedExternalOneByteStringType =
            UncachedExternalInternalizedOneByteStringType as u16 | NOT_INTERNALIZED_TAG as u16,
        ThinTwoByteStringType =
            TWO_BYTE_STRING_TAG as u16 | StringRepresentationTag::ThinStringTag as u16 | NOT_INTERNALIZED_TAG as u16,
        ThinOneByteStringType =
            ONE_BYTE_STRING_TAG as u16 | StringRepresentationTag::ThinStringTag as u16 | NOT_INTERNALIZED_TAG as u16,
        SharedSeqTwoByteStringType = SeqTwoByteStringType as u16 | SHARED_STRING_TAG as u16,
        SharedSeqOneByteStringType = SeqOneByteStringType as u16 | SHARED_STRING_TAG as u16,
        SharedExternalTwoByteStringType = ExternalTwoByteStringType as u16 | SHARED_STRING_TAG as u16,
        SharedExternalOneByteStringType = ExternalOneByteStringType as u16 | SHARED_STRING_TAG as u16,
        SharedUncachedExternalTwoByteStringType =
            UncachedExternalTwoByteStringType as u16 | SHARED_STRING_TAG as u16,
        SharedUncachedExternalOneByteStringType =
            UncachedExternalOneByteStringType as u16 | SHARED_STRING_TAG as u16,

        // Most instance types are defined in Torque, with the exception of the string
        // types above. They are ordered by inheritance hierarchy so that we can easily
        // use range checks to determine whether an object is an instance of a subclass
        // of any type. There are a few more constraints specified in the Torque type
        // definitions:
        // - Some instance types are exposed in v8.h, so they are locked to specific
        //   values to not unnecessarily change the ABI.
        // - JSSpecialObject and JSCustomElementsObject are aligned with the beginning
        //   of the JSObject range, so that we can use a larger range check from
        //   FIRST_JS_RECEIVER_TYPE to the end of those ranges and include JSProxy too.

        // Placeholder for Torque-assigned instance types
        // The actual values will depend on the Torque-generated code
        // For example:
        // MapType = 32,
        // FixedArrayType = 33,
        // ...

        // Pseudo-types
        FirstUniqueNameType = InternalizedTwoByteStringType,
        LastUniqueNameType = SymbolType, // Placeholder
        FirstNonStringType = SymbolType,    // Placeholder
        // Callable JS Functions are all JS Functions except class constructors.
        FirstCallableJsFunctionType = FirstJsFunctionType, // Placeholder
        LastCallableJsFunctionType = JsClassConstructorType as u16 - 1, // Placeholder
        // Boundary for testing JSReceivers that need special property lookup handling
        LastSpecialReceiverType = LastJsSpecialObjectType, // Placeholder
        // Boundary case for testing JSReceivers that may have elements while having
        // an empty fixed array as elements backing store. This is true for string
        // wrappers.
        LastCustomElementsReceiver = LastJsCustomElementsObjectType, // Placeholder

        // Convenient names for things where the generated name is awkward:
        FirstType = FirstHeapObjectType,   // Placeholder
        LastType = LastHeapObjectType,     // Placeholder
        BigintType = BigIntBaseType, // Placeholder

        // TODO(ishell): define a dedicated instance type for DependentCode to
        // simplify CodeSerializer.
        DependentCodeType = WeakArrayListType, // Placeholder
        JsObjectType = 64,
        MapType = 65,
        FixedArrayType = 66,
        SymbolType = 67,
        FirstJsFunctionType = 68,
        JsClassConstructorType = 69,
        FirstHeapObjectType = 70,
        LastHeapObjectType = 71,
        BigIntBaseType = 72,
        WeakArrayListType = 73,
        FirstJsReceiverType = 74,
        LastJsSpecialObjectType = 75,
        LastJsCustomElementsObjectType = 76,
        JsProxyType = 77,
        JsGlobalObjectType = 78,
        JsGlobalProxyType = 79,
        JsModuleNamespaceType = 80,
        JsSpecialApiObjectType = 81,
        JsPrimitiveWrapperType = 82,
        OddballType = 83,
        ForeignType = 84,

    }

    /// This constant is defined outside of the `InstanceType` enum because the
    /// string instance types are sparse and there's no such string instance type.
    /// But it's still useful for range checks to have such a value.
    pub const LAST_STRING_TYPE: InstanceType =
        unsafe { std::mem::transmute(InstanceType::FirstNonStringType as u16 - 1) };

    const _: () = assert!((InstanceType::FirstNonStringType as u16 as u32 & IS_NOT_STRING_MASK) != STRING_TAG);

    //  These assertions cannot be directly translated without access to internal structs and constants:
    //  static_assert(JS_OBJECT_TYPE == Internals::kJSObjectType);
    //  static_assert(FIRST_JS_API_OBJECT_TYPE == Internals::kFirstJSApiObjectType);
    //  static_assert(LAST_JS_API_OBJECT_TYPE == Internals::kLastJSApiObjectType);
    //  static_assert(JS_SPECIAL_API_OBJECT_TYPE == Internals::kJSSpecialApiObjectType);
    //  static_assert(FIRST_NONSTRING_TYPE == Internals::kFirstNonstringType);
    //  static_assert(ODDBALL_TYPE == Internals::kOddballType);
    //  static_assert(FOREIGN_TYPE == Internals::kForeignType);

    // Verify that string types are all less than other types.
    macro_rules! check_string_range {
        ($type:ident) => {
            const _: () = assert!(
                InstanceType::$type as u16  < InstanceType::FirstNonStringType as u16
            );
        };
    }

    check_string_range!(InternalizedTwoByteStringType);
    check_string_range!(InternalizedOneByteStringType);
    check_string_range!(ExternalInternalizedTwoByteStringType);
    check_string_range!(ExternalInternalizedOneByteStringType);
    check_string_range!(UncachedExternalInternalizedTwoByteStringType);
    check_string_range!(UncachedExternalInternalizedOneByteStringType);
    check_string_range!(SeqTwoByteStringType);
    check_string_range!(SeqOneByteStringType);
    check_string_range!(ConsTwoByteStringType);
    check_string_range!(ConsOneByteStringType);
    check_string_range!(SlicedTwoByteStringType);
    check_string_range!(SlicedOneByteStringType);
    check_string_range!(ExternalTwoByteStringType);
    check_string_range!(ExternalOneByteStringType);
    check_string_range!(UncachedExternalTwoByteStringType);
    check_string_range!(UncachedExternalOneByteStringType);
    check_string_range!(ThinTwoByteStringType);
    check_string_range!(ThinOneByteStringType);
    check_string_range!(SharedSeqTwoByteStringType);
    check_string_range!(SharedSeqOneByteStringType);
    check_string_range!(SharedExternalTwoByteStringType);
    check_string_range!(SharedExternalOneByteStringType);
    check_string_range!(SharedUncachedExternalTwoByteStringType);
    check_string_range!(SharedUncachedExternalOneByteStringType);

    macro_rules! check_nonstring_range {
        ($type:ident) => {
            const _: () = assert!(
                InstanceType::$type as u16 >= InstanceType::FirstNonStringType as u16
            );
        };
    }

    check_nonstring_range!(JsObjectType);
    check_nonstring_range!(MapType);
    check_nonstring_range!(FixedArrayType);
    check_nonstring_range!(SymbolType);
    check_nonstring_range!(FirstJsFunctionType);
    check_nonstring_range!(JsClassConstructorType);
    check_nonstring_range!(FirstHeapObjectType);
    check_nonstring_range!(LastHeapObjectType);
    check_nonstring_range!(BigIntBaseType);
    check_nonstring_range!(WeakArrayListType);
    check_nonstring_range!(FirstJsReceiverType);
    check_nonstring_range!(LastJsSpecialObjectType);
    check_nonstring_range!(LastJsCustomElementsObjectType);
    check_nonstring_range!(JsProxyType);
    check_nonstring_range!(JsGlobalObjectType);
    check_nonstring_range!(JsGlobalProxyType);
    check_nonstring_range!(JsModuleNamespaceType);
    check_nonstring_range!(JsSpecialApiObjectType);
    check_nonstring_range!(JsPrimitiveWrapperType);
    check_nonstring_range!(OddballType);
    check_nonstring_range!(ForeignType);

    // classConstructor type has to be the last one in the JS Function type range.
    const _: () = assert!(InstanceType::JsClassConstructorType as u16 == InstanceType::FirstJsFunctionType as u16);
    const _: () = assert!(
        (InstanceType::JsClassConstructorType as u16) < (InstanceType::FirstCallableJsFunctionType as u16)
            || (InstanceType::JsClassConstructorType as u16) > (InstanceType::LastCallableJsFunctionType as u16),
        "JS_CLASS_CONSTRUCTOR_TYPE must not be in the callable JS function type range"
    );

    // Two ranges don't cleanly follow the inheritance hierarchy. Here we ensure
    // that only expected types fall within these ranges.
    // - From FIRST_JS_RECEIVER_TYPE to LAST_SPECIAL_RECEIVER_TYPE should correspond
    //   to the union type JSProxy | JSSpecialObject.
    // - From FIRST_JS_RECEIVER_TYPE to LAST_CUSTOM_ELEMENTS_RECEIVER should
    //   correspond to the union type JSProxy | JSCustomElementsObject.
    // Note in particular that these ranges include all subclasses of JSReceiver
    // that are not also subclasses of JSObject (currently only JSProxy).
    // clang-format off
    macro_rules! check_instance_type {
        ($type:ident) => {
            const _: () = assert!(
                ((InstanceType::$type as u16) >= (InstanceType::FirstJsReceiverType as u16)
                    && (InstanceType::$type as u16) <= (InstanceType::LastSpecialReceiverType as u16))
                    == ((InstanceType::$type == InstanceType::JsProxyType) || (InstanceType::$type == InstanceType::JsGlobalObjectType) ||
                    (InstanceType::$type == InstanceType::JsGlobalProxyType) ||
                    (InstanceType::$type == InstanceType::JsModuleNamespaceType) ||
                    (InstanceType::$type == InstanceType::JsSpecialApiObjectType))
            );
            const _: () = assert!(
                ((InstanceType::$type as u16) >= (InstanceType::FirstJsReceiverType as u16)
                    && (InstanceType::$type as u16) <= (InstanceType::LastCustomElementsReceiver as u16))
                    == ((InstanceType::$type == InstanceType::JsProxyType) || (InstanceType::$type == InstanceType::JsGlobalObjectType) ||
                    (InstanceType::$type == InstanceType::JsGlobalProxyType) ||
                    (InstanceType::$type == InstanceType::JsModuleNamespaceType) ||
                    (InstanceType::$type == InstanceType::JsSpecialApiObjectType) ||
                    (InstanceType::$type == InstanceType::JsPrimitiveWrapperType))
            );
        };
    }

    check_instance_type!(JsObjectType);
    check_instance_type!(MapType);
    check_instance_type!(FixedArrayType);
    check_instance_type!(SymbolType);
    check_instance_type!(FirstJsFunctionType);
    check_instance_type!(JsClassConstructorType);
    check_instance_type!(FirstHeapObjectType);
    check_instance_type!(LastHeapObjectType);
    check_instance_type!(BigIntBaseType);
    check_instance_type!(WeakArrayListType);
    check_instance_type!(FirstJsReceiverType);
    check_instance_type!(LastJsSpecialObjectType);
    check_instance_type!(LastJsCustomElementsObjectType);
    check_instance_type!(JsProxyType);
    check_instance_type!(JsGlobalObjectType);
    check_instance_type!(JsGlobalProxyType);
    check_instance_type!(JsModuleNamespaceType);
    check_instance_type!(JsSpecialApiObjectType);
    check_instance_type!(JsPrimitiveWrapperType);
    check_instance_type!(OddballType);
    check_instance_type!(ForeignType);

    // clang-format on

    // Make sure it doesn't matter whether we sign-extend or zero-extend these
    // values, because Torque treats InstanceType as signed.
    const _: () = assert!((InstanceType::LastType as u16) < (1 << 15));

    impl std::fmt::Display for InstanceType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    pub fn to_string(instance_type: InstanceType) -> String {
        format!("{}", instance_type)
    }

    // These macros are used to generate lists of maps associated with instance types.
    // Since we don't have the actual map types, we'll just define empty macros.
    // In a real implementation, these macros would generate code to access the maps.

    macro_rules! unique_leaf_instance_type_map_list_generator {
        ($v:ident, $($(_:tt, $map_name:ident, $field_name:ident, $object_type:ident),*)*) => {
            // No code generated here.
        };
    }

    macro_rules! unique_instance_type_map_list_generator {
        ($v:ident, $($(_:tt, $map_name:ident, $field_name:ident, $object_type:ident),*)*) => {
            // No code generated here.
        };
    }

    pub(crate) const PROPERTY_DICTIONARY_TYPE: InstanceType = InstanceType::NameDictionaryType; // Placeholder

    #[allow(dead_code)]
    const NAME_DICTIONARY_TYPE: InstanceType = InstanceType::NameDictionaryType; // Placeholder

    pub mod instance_type_checker {
        use super::InstanceType;

        // Placeholder functions for checking instance types.
        pub fn is_seq_string(_map: u32) -> bool {
            false
        }

        pub fn is_cons_string(_map: u32) -> bool {
            false
        }

        pub fn is_sliced_string(_map: u32) -> bool {
            false
        }

        pub fn is_thin_string(_map: u32) -> bool {
            false
        }

        pub fn is_one_byte_string(_map: u32) -> bool {
            false
        }

        pub fn is_two_byte_string(_map: u32) -> bool {
            false
        }
    }
}