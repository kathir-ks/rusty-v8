// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// Torque constants.
pub mod torque {

    /// Prefix for constexpr types.
    pub const CONSTEXPR_TYPE_PREFIX: &str = "constexpr ";
    /// String representation of the `never` type.
    pub const NEVER_TYPE_STRING: &str = "never";
    /// String representation of the `constexpr bool` type.
    pub const CONSTEXPR_BOOL_TYPE_STRING: &str = "constexpr bool";
    /// String representation of the `constexpr string` type.
    pub const CONSTEXPR_STRING_TYPE_STRING: &str = "constexpr string";
    /// String representation of the `constexpr intptr` type.
    pub const CONSTEXPR_INTPTR_TYPE_STRING: &str = "constexpr intptr";
    /// String representation of the `constexpr InstanceType` type.
    pub const CONSTEXPR_INSTANCE_TYPE_TYPE_STRING: &str = "constexpr InstanceType";
    /// String representation of the `bool` type.
    pub const BOOL_TYPE_STRING: &str = "bool";
    /// String representation of the `void` type.
    pub const VOID_TYPE_STRING: &str = "void";
    /// String representation of the `Arguments` type.
    pub const ARGUMENTS_TYPE_STRING: &str = "Arguments";
    /// String representation of the `Context` type.
    pub const CONTEXT_TYPE_STRING: &str = "Context";
    /// String representation of the `NoContext` type.
    pub const NO_CONTEXT_TYPE_STRING: &str = "NoContext";
    /// String representation of the `NativeContext` type.
    pub const NATIVE_CONTEXT_TYPE_STRING: &str = "NativeContext";
    /// String representation of the `JSFunction` type.
    pub const JS_FUNCTION_TYPE_STRING: &str = "JSFunction";
    /// String representation of the `Map` type.
    pub const MAP_TYPE_STRING: &str = "Map";
    /// String representation of the `Object` type.
    pub const OBJECT_TYPE_STRING: &str = "Object";
    /// String representation of the `HeapObject` type.
    pub const HEAP_OBJECT_TYPE_STRING: &str = "HeapObject";
    /// String representation of the `TaggedZeroPattern` type.
    pub const TAGGED_ZERO_PATTERN_TYPE_STRING: &str = "TaggedZeroPattern";
    /// String representation of the `JSAny` type.
    pub const JSANY_TYPE_STRING: &str = "JSAny";
    /// String representation of the `JSObject` type.
    pub const JSOBJECT_TYPE_STRING: &str = "JSObject";
    /// String representation of the `Smi` type.
    pub const SMI_TYPE_STRING: &str = "Smi";
    /// String representation of the `Tagged` type.
    pub const TAGGED_TYPE_STRING: &str = "Tagged";
    /// String representation of the `StrongTagged` type.
    pub const STRONG_TAGGED_TYPE_STRING: &str = "StrongTagged";
    /// String representation of the `Uninitialized` type.
    pub const UNINITIALIZED_TYPE_STRING: &str = "Uninitialized";
    /// String representation of the `UninitializedHeapObject` type.
    pub const UNINITIALIZED_HEAP_OBJECT_TYPE_STRING: &str = "UninitializedHeapObject";
    /// String representation of the `RawPtr` type.
    pub const RAWPTR_TYPE_STRING: &str = "RawPtr";
    /// String representation of the `ExternalPointer` type.
    pub const EXTERNALPTR_TYPE_STRING: &str = "ExternalPointer";
    /// String representation of the `CppHeapPointer` type.
    pub const CPPHEAPPTR_TYPE_STRING: &str = "CppHeapPointer";
    /// String representation of the `TrustedPointer` type.
    pub const TRUSTEDPTR_TYPE_STRING: &str = "TrustedPointer";
    /// String representation of the `ProtectedPointer` type.
    pub const PROTECTEDPTR_TYPE_STRING: &str = "ProtectedPointer";
    /// String representation of the `DispatchHandle` type.
    pub const DISPATCH_HANDLE_TYPE_STRING: &str = "DispatchHandle";
    /// String representation of the `constexpr string` type.
    pub const CONST_STRING_TYPE_STRING: &str = "constexpr string";
    /// String representation of the `String` type.
    pub const STRING_TYPE_STRING: &str = "String";
    /// String representation of the `Number` type.
    pub const NUMBER_TYPE_STRING: &str = "Number";
    /// String representation of the `BuiltinPtr` type.
    pub const BUILTIN_POINTER_TYPE_STRING: &str = "BuiltinPtr";
    /// String representation of the `intptr` type.
    pub const INTPTR_TYPE_STRING: &str = "intptr";
    /// String representation of the `uintptr` type.
    pub const UINTPTR_TYPE_STRING: &str = "uintptr";
    /// String representation of the `int64` type.
    pub const INT64_TYPE_STRING: &str = "int64";
    /// String representation of the `uint64` type.
    pub const UINT64_TYPE_STRING: &str = "uint64";
    /// String representation of the `int31` type.
    pub const INT31_TYPE_STRING: &str = "int31";
    /// String representation of the `int32` type.
    pub const INT32_TYPE_STRING: &str = "int32";
    /// String representation of the `uint31` type.
    pub const UINT31_TYPE_STRING: &str = "uint31";
    /// String representation of the `uint32` type.
    pub const UINT32_TYPE_STRING: &str = "uint32";
    /// String representation of the `int16` type.
    pub const INT16_TYPE_STRING: &str = "int16";
    /// String representation of the `uint16` type.
    pub const UINT16_TYPE_STRING: &str = "uint16";
    /// String representation of the `int8` type.
    pub const INT8_TYPE_STRING: &str = "int8";
    /// String representation of the `uint8` type.
    pub const UINT8_TYPE_STRING: &str = "uint8";
    /// String representation of the `bint` type.
    pub const BINT_TYPE_STRING: &str = "bint";
    /// String representation of the `char8` type.
    pub const CHAR8_TYPE_STRING: &str = "char8";
    /// String representation of the `char16` type.
    pub const CHAR16_TYPE_STRING: &str = "char16";
    /// String representation of the `float16_raw_bits` type.
    pub const FLOAT16_RAW_BITS_TYPE_STRING: &str = "float16_raw_bits";
    /// String representation of the `float32` type.
    pub const FLOAT32_TYPE_STRING: &str = "float32";
    /// String representation of the `float64` type.
    pub const FLOAT64_TYPE_STRING: &str = "float64";
    /// String representation of the `float64_or_undefined_or_hole` type.
    pub const FLOAT64_OR_UNDEFINED_OR_HOLE_TYPE_STRING: &str = "float64_or_undefined_or_hole";
    /// String representation of the `constexpr int31` type.
    pub const CONST_INT31_TYPE_STRING: &str = "constexpr int31";
    /// String representation of the `constexpr int32` type.
    pub const CONST_INT32_TYPE_STRING: &str = "constexpr int32";
    /// String representation of the `constexpr float64` type.
    pub const CONST_FLOAT64_TYPE_STRING: &str = "constexpr float64";
    /// String representation of the `constexpr IntegerLiteral` type.
    pub const INTEGER_LITERAL_TYPE_STRING: &str = "constexpr IntegerLiteral";
    /// String representation of the `torque_internal` namespace.
    pub const TORQUE_INTERNAL_NAMESPACE_STRING: &str = "torque_internal";
    /// String representation of the `MutableReference` type.
    pub const MUTABLE_REFERENCE_TYPE_STRING: &str = "MutableReference";
    /// String representation of the `ConstReference` type.
    pub const CONST_REFERENCE_TYPE_STRING: &str = "ConstReference";
    /// String representation of the `MutableSlice` type.
    pub const MUTABLE_SLICE_TYPE_STRING: &str = "MutableSlice";
    /// String representation of the `ConstSlice` type.
    pub const CONST_SLICE_TYPE_STRING: &str = "ConstSlice";
    /// String representation of the `Weak` type.
    pub const WEAK_TYPE_STRING: &str = "Weak";
    /// String representation of the `SmiTagged` type.
    pub const SMI_TAGGED_TYPE_STRING: &str = "SmiTagged";
    /// String representation of the `Lazy` type.
    pub const LAZY_TYPE_STRING: &str = "Lazy";
    /// String representation of the `UninitializedIterator` type.
    pub const UNINITIALIZED_ITERATOR_TYPE_STRING: &str = "UninitializedIterator";
    /// String representation of the `_generic_type_instantiation_namespace` namespace.
    pub const GENERIC_TYPE_INSTANTIATION_NAMESPACE_STRING: &str = "_generic_type_instantiation_namespace";
    /// String representation of the `FixedArrayBase` type.
    pub const FIXED_ARRAY_BASE_TYPE_STRING: &str = "FixedArrayBase";
    /// String representation of the `WeakHeapObject` type.
    pub const WEAK_HEAP_OBJECT: &str = "WeakHeapObject";
    /// String representation of the `StaticAssert` macro.
    pub const STATIC_ASSERT_MACRO_STRING: &str = "StaticAssert";

    /// Annotation for abstract types.
    pub const ANNOTATION_ABSTRACT: &str = "@abstract";
    /// Annotation for types with the same instance type as their parent.
    pub const ANNOTATION_HAS_SAME_INSTANCE_TYPE_AS_PARENT: &str =
        "@hasSameInstanceTypeAsParent";
    /// Annotation to prevent C++ class generation.
    pub const ANNOTATION_DO_NOT_GENERATE_CPP_CLASS: &str = "@doNotGenerateCppClass";
    /// Annotation for custom maps.
    pub const ANNOTATION_CUSTOM_MAP: &str = "@customMap";
    /// Annotation for custom C++ classes.
    pub const ANNOTATION_CUSTOM_CPP_CLASS: &str = "@customCppClass";
    /// Annotation for the highest instance type within a parent class range.
    pub const ANNOTATION_HIGHEST_INSTANCE_TYPE_WITHIN_PARENT: &str =
        "@highestInstanceTypeWithinParentClassRange";
    /// Annotation for the lowest instance type within a parent class range.
    pub const ANNOTATION_LOWEST_INSTANCE_TYPE_WITHIN_PARENT: &str =
        "@lowestInstanceTypeWithinParentClassRange";
    /// Annotation to reserve bits in the instance type.
    pub const ANNOTATION_RESERVE_BITS_IN_INSTANCE_TYPE: &str =
        "@reserveBitsInInstanceType";
    /// Annotation for instance type values exposed to the API.
    pub const ANNOTATION_INSTANCE_TYPE_VALUE: &str = "@apiExposedInstanceTypeValue";
    /// Annotation for conditional compilation (`if`).
    pub const ANNOTATION_IF: &str = "@if";
    /// Annotation for conditional compilation (`ifnot`).
    pub const ANNOTATION_IFNOT: &str = "@ifnot";
    /// Annotation to generate a body descriptor.
    pub const ANNOTATION_GENERATE_BODY_DESCRIPTOR: &str = "@generateBodyDescriptor";
    /// Annotation to generate a unique map.
    pub const ANNOTATION_GENERATE_UNIQUE_MAP: &str = "@generateUniqueMap";
    /// Annotation to generate a factory function.
    pub const ANNOTATION_GENERATE_FACTORY_FUNCTION: &str = "@generateFactoryFunction";
    /// Annotation to export a type or function.
    pub const ANNOTATION_EXPORT: &str = "@export";
    /// Annotation to prevent cast generation.
    pub const ANNOTATION_DO_NOT_GENERATE_CAST: &str = "@doNotGenerateCast";
    /// Annotation to use the parent's type checker.
    pub const ANNOTATION_USE_PARENT_TYPE_CHECKER: &str = "@useParentTypeChecker";
    /// Annotation for C++ object definitions.
    pub const ANNOTATION_CPP_OBJECT_DEFINITION: &str = "@cppObjectDefinition";
    /// Annotation for C++ object layout definitions.
    pub const ANNOTATION_CPP_OBJECT_LAYOUT_DEFINITION: &str = "@cppObjectLayoutDefinition";
    /// Annotation to have the same enum value as another enum.
    pub const ANNOTATION_SAME_ENUM_VALUE_AS: &str = "@sameEnumValueAs";
    /// Annotation to generate C++ accessors with relaxed store semantics.
    pub const ANNOTATION_CPP_RELAXED_STORE: &str = "@cppRelaxedStore";
    /// Annotation to generate C++ accessors with relaxed load semantics.
    pub const ANNOTATION_CPP_RELAXED_LOAD: &str = "@cppRelaxedLoad";
    /// Annotation to generate C++ accessors with release store semantics.
    pub const ANNOTATION_CPP_RELEASE_STORE: &str = "@cppReleaseStore";
    /// Annotation to generate C++ accessors with acquire load semantics.
    pub const ANNOTATION_CPP_ACQUIRE_LOAD: &str = "@cppAcquireLoad";
    /// Annotation to generate BodyDescriptor using IterateCustomWeakPointers.
    pub const ANNOTATION_CUSTOM_WEAK_MARKING: &str = "@customWeakMarking";
    /// Do not generate an interface descriptor for this builtin.
    pub const ANNOTATION_CUSTOM_INTERFACE_DESCRIPTOR: &str = "@customInterfaceDescriptor";
    /// Automatically generates a call to IncrementUseCounter at the start of a builtin.
    pub const ANNOTATION_INCREMENT_USE_COUNTER: &str = "@incrementUseCounter";

    /// Checks if a name is a constexpr name.
    pub fn is_constexpr_name(name: &str) -> bool {
        name.starts_with(CONSTEXPR_TYPE_PREFIX)
    }

    /// Gets the non-constexpr name from a constexpr name.
    pub fn get_non_constexpr_name(name: &str) -> String {
        if !is_constexpr_name(name) {
            return name.to_string();
        }
        name[CONSTEXPR_TYPE_PREFIX.len()..].to_string()
    }

    /// Gets the constexpr name from a name.
    pub fn get_constexpr_name(name: &str) -> String {
        if is_constexpr_name(name) {
            return name.to_string();
        }
        format!("{}{}", CONSTEXPR_TYPE_PREFIX, name)
    }

    bitflags::bitflags! {
        /// Flags for abstract types.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct AbstractTypeFlag: u32 {
            /// No flags.
            const NONE = 0;
            /// Transient flag.
            const TRANSIENT = 1 << 0;
            /// Constexpr flag.
            const CONSTEXPR = 1 << 1;
            /// Use parent type checker flag.
            const USE_PARENT_TYPE_CHECKER = 1 << 2;
        }
    }

    bitflags::bitflags! {
        /// Flags for classes.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct ClassFlag: u32 {
            /// No flags.
            const NONE = 0;
            /// Extern flag.
            const EXTERN = 1 << 0;
            /// Transient flag.
            const TRANSIENT = 1 << 1;
            /// Abstract flag.
            const ABSTRACT = 1 << 2;
            /// IsShape flag.
            const IS_SHAPE = 1 << 3;
            /// HasSameInstanceTypeAsParent flag.
            const HAS_SAME_INSTANCE_TYPE_AS_PARENT = 1 << 4;
            /// GenerateCppClassDefinitions flag.
            const GENERATE_CPP_CLASS_DEFINITIONS = 1 << 5;
            /// HighestInstanceTypeWithinParent flag.
            const HIGHEST_INSTANCE_TYPE_WITHIN_PARENT = 1 << 6;
            /// LowestInstanceTypeWithinParent flag.
            const LOWEST_INSTANCE_TYPE_WITHIN_PARENT = 1 << 7;
            /// UndefinedLayout flag.
            const UNDEFINED_LAYOUT = 1 << 8;
            /// GenerateBodyDescriptor flag.
            const GENERATE_BODY_DESCRIPTOR = 1 << 9;
            /// Export flag.
            const EXPORT = 1 << 10;
            /// DoNotGenerateCast flag.
            const DO_NOT_GENERATE_CAST = 1 << 11;
             /// GenerateUniqueMap flag.
            const GENERATE_UNIQUE_MAP = 1 << 12;
            /// GenerateFactoryFunction flag.
            const GENERATE_FACTORY_FUNCTION = 1 << 13;
            /// CppObjectDefinition flag.
            const CPP_OBJECT_DEFINITION = 1 << 14;
            /// CppObjectLayoutDefinition flag.
            const CPP_OBJECT_LAYOUT_DEFINITION = 1 << 15;
        }
    }

    bitflags::bitflags! {
        /// Flags for structs.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct StructFlag: u32 {
            /// No flags.
            const NONE = 0;
            /// Export flag.
            const EXPORT = 1 << 0;
        }
    }

    /// Enum for field synchronization.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FieldSynchronization {
        /// No synchronization.
        None,
        /// Relaxed synchronization.
        Relaxed,
        /// Acquire/Release synchronization.
        AcquireRelease,
    }
}