// Converted from V8 C++ source files:
// Header: constants.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub struct Flags<T> {
        flags: u32,
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> Flags<T> {
        pub fn new() -> Self {
            Flags {
                flags: 0,
                _phantom: std::marker::PhantomData,
            }
        }

        pub fn has(&self, flag: T) -> bool
        where
            T: Into<u32>,
        {
            self.flags & flag.into() != 0
        }

        pub fn set(&mut self, flag: T)
        where
            T: Into<u32>,
        {
            self.flags |= flag.into();
        }

        pub fn clear(&mut self, flag: T)
        where
            T: Into<u32>,
        {
            self.flags &= !flag.into();
        }
    }
}

pub mod torque {
    use std::ffi::CStr;
    use std::os::raw::c_char;

    pub const CONSTEXPR_TYPE_PREFIX: &str = "constexpr ";
    pub const NEVER_TYPE_STRING: &str = "never";
    pub const CONSTEXPR_BOOL_TYPE_STRING: &str = "constexpr bool";
    pub const CONSTEXPR_STRING_TYPE_STRING: &str = "constexpr string";
    pub const CONSTEXPR_INTPTR_TYPE_STRING: &str = "constexpr intptr";
    pub const CONSTEXPR_INSTANCE_TYPE_TYPE_STRING: &str = "constexpr InstanceType";
    pub const BOOL_TYPE_STRING: &str = "bool";
    pub const VOID_TYPE_STRING: &str = "void";
    pub const ARGUMENTS_TYPE_STRING: &str = "Arguments";
    pub const CONTEXT_TYPE_STRING: &str = "Context";
    pub const NO_CONTEXT_TYPE_STRING: &str = "NoContext";
    pub const NATIVE_CONTEXT_TYPE_STRING: &str = "NativeContext";
    pub const JS_FUNCTION_TYPE_STRING: &str = "JSFunction";
    pub const MAP_TYPE_STRING: &str = "Map";
    pub const OBJECT_TYPE_STRING: &str = "Object";
    pub const HEAP_OBJECT_TYPE_STRING: &str = "HeapObject";
    pub const TAGGED_ZERO_PATTERN_TYPE_STRING: &str = "TaggedZeroPattern";
    pub const JSANY_TYPE_STRING: &str = "JSAny";
    pub const JSOBJECT_TYPE_STRING: &str = "JSObject";
    pub const SMI_TYPE_STRING: &str = "Smi";
    pub const TAGGED_TYPE_STRING: &str = "Tagged";
    pub const STRONG_TAGGED_TYPE_STRING: &str = "StrongTagged";
    pub const UNINITIALIZED_TYPE_STRING: &str = "Uninitialized";
    pub const UNINITIALIZED_HEAP_OBJECT_TYPE_STRING: &str = "UninitializedHeapObject";
    pub const RAWPTR_TYPE_STRING: &str = "RawPtr";
    pub const EXTERNALPTR_TYPE_STRING: &str = "ExternalPointer";
    pub const CPPHEAPPTR_TYPE_STRING: &str = "CppHeapPointer";
    pub const TRUSTEDPTR_TYPE_STRING: &str = "TrustedPointer";
    pub const PROTECTEDPTR_TYPE_STRING: &str = "ProtectedPointer";
    pub const DISPATCH_HANDLE_TYPE_STRING: &str = "DispatchHandle";
    pub const CONST_STRING_TYPE_STRING: &str = "constexpr string";
    pub const STRING_TYPE_STRING: &str = "String";
    pub const NUMBER_TYPE_STRING: &str = "Number";
    pub const BUILTIN_POINTER_TYPE_STRING: &str = "BuiltinPtr";
    pub const INTPTR_TYPE_STRING: &str = "intptr";
    pub const UINTPTR_TYPE_STRING: &str = "uintptr";
    pub const INT64_TYPE_STRING: &str = "int64";
    pub const UINT64_TYPE_STRING: &str = "uint64";
    pub const INT31_TYPE_STRING: &str = "int31";
    pub const INT32_TYPE_STRING: &str = "int32";
    pub const UINT31_TYPE_STRING: &str = "uint31";
    pub const UINT32_TYPE_STRING: &str = "uint32";
    pub const INT16_TYPE_STRING: &str = "int16";
    pub const UINT16_TYPE_STRING: &str = "uint16";
    pub const INT8_TYPE_STRING: &str = "int8";
    pub const UINT8_TYPE_STRING: &str = "uint8";
    pub const BINT_TYPE_STRING: &str = "bint";
    pub const CHAR8_TYPE_STRING: &str = "char8";
    pub const CHAR16_TYPE_STRING: &str = "char16";
    pub const FLOAT16_RAW_BITS_TYPE_STRING: &str = "float16_raw_bits";
    pub const FLOAT32_TYPE_STRING: &str = "float32";
    pub const FLOAT64_TYPE_STRING: &str = "float64";
    pub const FLOAT64_OR_UNDEFINED_OR_HOLE_TYPE_STRING: &str =
        "float64_or_undefined_or_hole";
    pub const CONST_INT31_TYPE_STRING: &str = "constexpr int31";
    pub const CONST_INT32_TYPE_STRING: &str = "constexpr int32";
    pub const CONST_FLOAT64_TYPE_STRING: &str = "constexpr float64";
    pub const INTEGER_LITERAL_TYPE_STRING: &str = "constexpr IntegerLiteral";
    pub const TORQUE_INTERNAL_NAMESPACE_STRING: &str = "torque_internal";
    pub const MUTABLE_REFERENCE_TYPE_STRING: &str = "MutableReference";
    pub const CONST_REFERENCE_TYPE_STRING: &str = "ConstReference";
    pub const MUTABLE_SLICE_TYPE_STRING: &str = "MutableSlice";
    pub const CONST_SLICE_TYPE_STRING: &str = "ConstSlice";
    pub const WEAK_TYPE_STRING: &str = "Weak";
    pub const SMI_TAGGED_TYPE_STRING: &str = "SmiTagged";
    pub const LAZY_TYPE_STRING: &str = "Lazy";
    pub const UNINITIALIZED_ITERATOR_TYPE_STRING: &str = "UninitializedIterator";
    pub const GENERIC_TYPE_INSTANTIATION_NAMESPACE_STRING: &str =
        "_generic_type_instantiation_namespace";
    pub const FIXED_ARRAY_BASE_TYPE_STRING: &str = "FixedArrayBase";
    pub const WEAK_HEAP_OBJECT: &str = "WeakHeapObject";
    pub const STATIC_ASSERT_MACRO_STRING: &str = "StaticAssert";

    pub const ANNOTATION_ABSTRACT: &str = "@abstract";
    pub const ANNOTATION_HAS_SAME_INSTANCE_TYPE_AS_PARENT: &str =
        "@hasSameInstanceTypeAsParent";
    pub const ANNOTATION_DO_NOT_GENERATE_CPP_CLASS: &str = "@doNotGenerateCppClass";
    pub const ANNOTATION_CUSTOM_MAP: &str = "@customMap";
    pub const ANNOTATION_CUSTOM_CPP_CLASS: &str = "@customCppClass";
    pub const ANNOTATION_HIGHEST_INSTANCE_TYPE_WITHIN_PARENT: &str =
        "@highestInstanceTypeWithinParentClassRange";
    pub const ANNOTATION_LOWEST_INSTANCE_TYPE_WITHIN_PARENT: &str =
        "@lowestInstanceTypeWithinParentClassRange";
    pub const ANNOTATION_RESERVE_BITS_IN_INSTANCE_TYPE: &str =
        "@reserveBitsInInstanceType";
    pub const ANNOTATION_INSTANCE_TYPE_VALUE: &str = "@apiExposedInstanceTypeValue";
    pub const ANNOTATION_IF: &str = "@if";
    pub const ANNOTATION_IFNOT: &str = "@ifnot";
    pub const ANNOTATION_GENERATE_BODY_DESCRIPTOR: &str = "@generateBodyDescriptor";
    pub const ANNOTATION_GENERATE_UNIQUE_MAP: &str = "@generateUniqueMap";
    pub const ANNOTATION_GENERATE_FACTORY_FUNCTION: &str =
        "@generateFactoryFunction";
    pub const ANNOTATION_EXPORT: &str = "@export";
    pub const ANNOTATION_DO_NOT_GENERATE_CAST: &str = "@doNotGenerateCast";
    pub const ANNOTATION_USE_PARENT_TYPE_CHECKER: &str = "@useParentTypeChecker";
    pub const ANNOTATION_CPP_OBJECT_DEFINITION: &str = "@cppObjectDefinition";
    pub const ANNOTATION_CPP_OBJECT_LAYOUT_DEFINITION: &str =
        "@cppObjectLayoutDefinition";
    pub const ANNOTATION_SAME_ENUM_VALUE_AS: &str = "@sameEnumValueAs";
    // Generate C++ accessors with relaxed store semantics.
    // Weak<T> and Tagged<MaybeObject> fields always use relaxed store.
    pub const ANNOTATION_CPP_RELAXED_STORE: &str = "@cppRelaxedStore";
    // Generate C++ accessors with relaxed load semantics.
    pub const ANNOTATION_CPP_RELAXED_LOAD: &str = "@cppRelaxedLoad";
    // Generate C++ accessors with release store semantics.
    pub const ANNOTATION_CPP_RELEASE_STORE: &str = "@cppReleaseStore";
    // Generate C++ accessors with acquire load semantics.
    pub const ANNOTATION_CPP_ACQUIRE_LOAD: &str = "@cppAcquireLoad";
    // Generate BodyDescriptor using IterateCustomWeakPointers.
    pub const ANNOTATION_CUSTOM_WEAK_MARKING: &str = "@customWeakMarking";
    // Do not generate an interface descriptor for this builtin.
    pub const ANNOTATION_CUSTOM_INTERFACE_DESCRIPTOR: &str =
        "@customInterfaceDescriptor";
    // Automatically generates a call to IncrementUseCounter at the start of a
    // builtin.
    pub const ANNOTATION_INCREMENT_USE_COUNTER: &str = "@incrementUseCounter";

    pub fn is_constexpr_name(name: &str) -> bool {
        name.starts_with(CONSTEXPR_TYPE_PREFIX)
    }

    pub fn get_non_constexpr_name(name: &str) -> String {
        if !is_constexpr_name(name) {
            return name.to_string();
        }
        name[CONSTEXPR_TYPE_PREFIX.len()..].to_string()
    }

    pub fn get_constexpr_name(name: &str) -> String {
        if is_constexpr_name(name) {
            return name.to_string();
        }
        format!("{}{}", CONSTEXPR_TYPE_PREFIX, name)
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum AbstractTypeFlag {
        kNone = 0,
        kTransient = 1 << 0,
        kConstexpr = 1 << 1,
        kUseParentTypeChecker = 1 << 2,
    }

    impl From<AbstractTypeFlag> for u32 {
        fn from(flag: AbstractTypeFlag) -> Self {
            flag as u32
        }
    }

    pub type AbstractTypeFlags = base::Flags<AbstractTypeFlag>;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ClassFlag {
        kNone = 0,
        kExtern = 1 << 0,
        kTransient = 1 << 1,
        kAbstract = 1 << 2,
        kIsShape = 1 << 3,
        kHasSameInstanceTypeAsParent = 1 << 4,
        kGenerateCppClassDefinitions = 1 << 5,
        kHighestInstanceTypeWithinParent = 1 << 6,
        kLowestInstanceTypeWithinParent = 1 << 7,
        kUndefinedLayout = 1 << 8,
        kGenerateBodyDescriptor = 1 << 9,
        kExport = 1 << 10,
        kDoNotGenerateCast = 1 << 11,
        kGenerateUniqueMap = 1 << 12,
        kGenerateFactoryFunction = 1 << 13,
        kCppObjectDefinition = 1 << 14,
        kCppObjectLayoutDefinition = 1 << 15,
    }

    impl From<ClassFlag> for u32 {
        fn from(flag: ClassFlag) -> Self {
            flag as u32
        }
    }

    pub type ClassFlags = base::Flags<ClassFlag>;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum StructFlag {
        kNone = 0,
        kExport = 1 << 0,
    }

    impl From<StructFlag> for u32 {
        fn from(flag: StructFlag) -> Self {
            flag as u32
        }
    }

    pub type StructFlags = base::Flags<StructFlag>;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum FieldSynchronization {
        kNone,
        kRelaxed,
        kAcquireRelease,
    }
}
