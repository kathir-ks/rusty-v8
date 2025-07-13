// Converted from V8 C++ source files:
// Header: instance-type-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod instance_type_inl {
use std::{
    array,
    marker::PhantomData,
    mem::MaybeUninit,
    option::Option,
    sync::{Arc, Mutex, RwLock},
};

use crate::{
    base::bounds,
    compiler::map_inference::InstanceTypeChecker,
    execution::isolate_utils_inl,
    objects::{
        allocation_site::InstanceType, instance_type::*, map_inl::*,
        object_macros::*, off_heap_hash_table::Tagged_t, primitive_heap_object_inl::String,
    },
};

pub mod instance_type_checker {
    use std::option::Option;

    use crate::{
        base::bounds,
        objects::{
            allocation_site::InstanceType, instance_type::*, map_inl::*,
            object_macros::*, off_heap_hash_table::Tagged_t, primitive_heap_object_inl::String,
        },
    };

    pub mod instance_type_traits {
        pub struct AllocationSite;
        pub struct ArgumentsAdaptorFrameInfo;
        pub struct ArrayBoilerplateDescription;
        pub struct AsmWasmData;
        pub struct BigInt;
        pub struct BitField;
        pub struct BlockContext;
        pub struct BoilerplateDescription;
        pub struct Boolean;
        pub struct BreakPointInfo;
        pub struct BytecodeArray;
        pub struct CallHandlerInfo;
        pub struct CallSiteInfo;
        pub struct Cell;
        pub struct ClassBoilerplateDescription;
        pub struct Code;
        pub struct Context;
        pub struct ContextExtension;
        pub struct CoverageInfo;
        pub struct DebugInfo;
        pub struct DescriptorArray;
        pub struct Dictionary;
        pub struct ElementTransition;
        pub struct EnumCache;
        pub struct Error;
        pub struct ExportedSubclassBase;
        pub struct ExternalInternalizedString;
        pub struct ExternalOneByteString;
        pub struct ExternalString;
        pub struct ExternalTwoByteString;
        pub struct FixedArray;
        pub struct FixedArrayExact;
        pub struct FixedDoubleArray;
        pub struct FixedFloat32Array;
        pub struct FixedSet;
        pub struct FreeSpace;
        pub struct FunctionContext;
        pub struct HeapNumber;
        pub struct InterceptorInfo;
        pub struct InterpreterData;
        pub struct JSArgumentsObject;
        pub struct JSArray;
        pub struct JSArrayBuffer;
        pub struct JSAsyncFunctionObject;
        pub struct JSBoundFunction;
        pub struct JSCollectionIterator;
        pub struct JSDataView;
        pub struct JSDate;
        pub struct JSError;
        pub struct JSFinalizationGroup;
        pub struct JSFinalizationGroupCleanupIterator;
        pub struct JSFunction;
        pub struct JSGeneratorObject;
        pub struct JSGlobalObject;
        pub struct JSGlobalProxy;
        pub struct JSIntl;
        pub struct JSMap;
        pub struct JSMessageObject;
        pub struct JSModule;
        pub struct JSObject;
        pub struct JSPrimitiveWrapper;
        pub struct JSPromise;
        pub struct JSProxy;
        pub struct JSRegExp;
        pub struct JSSet;
        pub struct JSTypedArray;
        pub struct JSWeakMap;
        pub struct JSWeakRef;
        pub struct JSWeakSet;
        pub struct Map;
        pub struct MaybeSharedFunctionName;
        pub struct ModuleContext;
        pub struct NameDictionary;
        pub struct NativeContext;
        pub struct Null;
        pub struct Number;
        pub struct Oddball;
        pub struct OrderedHashMap;
        pub struct OrderedHashSet;
        pub struct OrderedNameDictionary;
        pub struct PromiseReactionJobTask;
        pub struct PropertyArray;
        pub struct PropertyCell;
        pub struct PrototypeInfo;
        pub struct Script;
        pub struct ScopeInfo;
        pub struct SeqOneByteString;
        pub struct SeqString;
        pub struct SeqTwoByteString;
        pub struct SharedFunctionInfo;
        pub struct SourceTextModule;
        pub struct String;
        pub struct StringTable;
        pub struct Symbol;
        pub struct SyntheticModule;
        pub struct TemplateInfo;
        pub struct ThinString;
        pub struct турbоfапBіtѕеtТурe;
        pub struct турbоfапЕquаlіtуТурe;
        pub struct турbоfапІнtеrvаlТурe;
        pub struct турbоfапОthеrNumbеrСоnѕtапtТурe;
        pub struct UсімРrоmіѕеСарturеdВеhаvіоr;
        pub struct Undefined;
        pub struct UncompiledData;
        pub struct UncompiledDataWithoutPreparseData;
        pub struct WasmCompiledModule;
        pub struct WasmExportedFunctionData;
        pub struct WasmInstanceObject;
        pub struct WasmMemoryObject;
        pub struct WasmModuleObject;
        pub struct WeakArrayList;
    }

    pub const fn unique_map_of_instance_type_check<T>() -> Option<RootIndex> {
        None
    }

    macro_rules! instance_type_map {
        ($V:ident, $rootIndexName:ident, $rootAccessorName:ident, $class_name:ident) => {
            impl crate::objects::instance_type_inl::instance_type_checker::InstanceTypeTraits::$class_name {
                pub const fn unique_map_of_instance_type_check() -> Option<RootIndex> {
                    Some(RootIndex::k##rootIndexName)
                }
            }
        };
    }

    macro_rules! unique_instance_type_map_list_generator {
        ($instance_type_map:ident, _) => {};
    }

    unique_instance_type_map_list_generator!(instance_type_map, _);

    pub const fn try_get_map_root_idx_for(type_: InstanceType) -> Option<RootIndex> {
        None
    }

    pub const fn unique_map_of_instance_type(type_: InstanceType) -> Option<RootIndex> {
        macro_rules! instance_type_check {
            ($it:ident, $forinstancetype:ident) => {
                if type_ == InstanceType::$forinstancetype {
                    return Self::unique_map_of_instance_type_check::<
                        crate::objects::instance_type_inl::instance_type_checker::InstanceTypeTraits::$it,
                    >();
                }
            };
        }
        macro_rules! instance_type_checkers_single {
            ($instance_type_check:ident) => {};
        }

        instance_type_checkers_single!(instance_type_check);

        crate::objects::map_inl::Map::try_get_map_root_idx_for(type_)
    }

    pub type InstanceTypeRange = (InstanceType, InstanceType);
    pub type TaggedAddressRange = (Tagged_t, Tagged_t);

    #[cfg(v8_static_roots_bool)]
    static K_UNIQUE_MAP_RANGE_OF_INSTANCE_TYPE_RANGE_LIST: [((InstanceType, InstanceType),
        (Tagged_t, Tagged_t)); 9] = [
        ((InstanceType::ALLOCATION_SITE_TYPE, InstanceType::ALLOCATION_SITE_TYPE),
            (StaticReadOnlyRoot::kAllocationSiteWithWeakNextMap as Tagged_t,
             StaticReadOnlyRoot::kAllocationSiteWithoutWeakNextMap as Tagged_t)),
        ((InstanceType::FIRST_STRING_TYPE, InstanceType::LAST_STRING_TYPE),
            (kStringMapLowerBound, kStringMapUpperBound)),
        ((InstanceType::FIRST_NAME_TYPE, InstanceType::LAST_NAME_TYPE),
            (StaticReadOnlyRoot::kSeqTwoByteStringMap as Tagged_t,
             StaticReadOnlyRoot::kSymbolMap as Tagged_t)),
        ((InstanceType::ODDBALL_TYPE, InstanceType::ODDBALL_TYPE),
            (StaticReadOnlyRoot::kUndefinedMap as Tagged_t,
             StaticReadOnlyRoot::kBooleanMap as Tagged_t)),
        ((InstanceType::HEAP_NUMBER_TYPE, InstanceType::ODDBALL_TYPE),
            (StaticReadOnlyRoot::kUndefinedMap as Tagged_t,
             StaticReadOnlyRoot::kHeapNumberMap as Tagged_t)),
        ((InstanceType::BIGINT_TYPE, InstanceType::HEAP_NUMBER_TYPE),
            (StaticReadOnlyRoot::kHeapNumberMap as Tagged_t,
             StaticReadOnlyRoot::kBigIntMap as Tagged_t)),
        ((InstanceType::FIRST_SMALL_ORDERED_HASH_TABLE_TYPE,
          InstanceType::LAST_SMALL_ORDERED_HASH_TABLE_TYPE),
            (StaticReadOnlyRoot::kSmallOrderedHashMapMap as Tagged_t,
             StaticReadOnlyRoot::kSmallOrderedNameDictionaryMap as Tagged_t)),
        ((InstanceType::FIRST_ABSTRACT_INTERNAL_CLASS_TYPE,
          InstanceType::LAST_ABSTRACT_INTERNAL_CLASS_TYPE),
            (StaticReadOnlyRoot::kAbstractInternalClassSubclass1Map as Tagged_t,
             StaticReadOnlyRoot::kAbstractInternalClassSubclass2Map as Tagged_t)),
        ((InstanceType::FIRST_TURBOFAN_TYPE_TYPE, InstanceType::LAST_TURBOFAN_TYPE_TYPE),
            (StaticReadOnlyRoot::kTurbofanBitsetTypeMap as Tagged_t,
             StaticReadOnlyRoot::kTurbofanOtherNumberConstantTypeMap as Tagged_t)),
    ];

    pub struct KUniqueMapRangeOfStringType {}
    impl KUniqueMapRangeOfStringType {
        pub const K_SEQ_STRING: TaggedAddressRange =
            (kStringMapLowerBound, StaticReadOnlyRoot::kInternalizedOneByteStringMap as Tagged_t);
        pub const K_INTERNALIZED_STRING: TaggedAddressRange = (
            StaticReadOnlyRoot::kInternalizedTwoByteStringMap as Tagged_t,
            StaticReadOnlyRoot::kUncachedExternalInternalizedOneByteStringMap as Tagged_t,
        );
        pub const K_EXTERNAL_STRING: TaggedAddressRange = (
            StaticReadOnlyRoot::kExternalInternalizedTwoByteStringMap as Tagged_t,
            StaticReadOnlyRoot::kSharedExternalOneByteStringMap as Tagged_t,
        );
        pub const K_UNCACHED_EXTERNAL_STRING: TaggedAddressRange = (
            StaticReadOnlyRoot::kUncachedExternalInternalizedTwoByteStringMap as Tagged_t,
            StaticReadOnlyRoot::kSharedUncachedExternalOneByteStringMap as Tagged_t,
        );
        pub const K_CONS_STRING: TaggedAddressRange = (
            StaticReadOnlyRoot::kConsTwoByteStringMap as Tagged_t,
            StaticReadOnlyRoot::kConsOneByteStringMap as Tagged_t,
        );
        pub const K_SLICED_STRING: TaggedAddressRange = (
            StaticReadOnlyRoot::kSlicedTwoByteStringMap as Tagged_t,
            StaticReadOnlyRoot::kSlicedOneByteStringMap as Tagged_t,
        );
        pub const K_THIN_STRING: TaggedAddressRange = (
            StaticReadOnlyRoot::kThinTwoByteStringMap as Tagged_t,
            StaticReadOnlyRoot::kThinOneByteStringMap as Tagged_t,
        );
    }

    pub const K_STRING_MAP_ENCODING_MASK: i32 =
        1 << crate::base::bounds::count_trailing_zeros_nonzero(Map::K_SIZE);
    pub const K_ONE_BYTE_STRING_MAP_BIT: i32 =
        StaticReadOnlyRoot::kSeqOneByteStringMap as i32 & K_STRING_MAP_ENCODING_MASK;
    pub const K_TWO_BYTE_STRING_MAP_BIT: i32 =
        StaticReadOnlyRoot::kSeqTwoByteStringMap as i32 & K_STRING_MAP_ENCODING_MASK;

    pub const fn unique_map_range_of_instance_type_range(
        first: InstanceType,
        last: InstanceType,
    ) -> Option<TaggedAddressRange> {
        let k_unique_map_range_of_instance_type_range_list = K_UNIQUE_MAP_RANGE_OF_INSTANCE_TYPE_RANGE_LIST;
        let mut i: usize = 0;
        while i < k_unique_map_range_of_instance_type_range_list.len() {
            if k_unique_map_range_of_instance_type_range_list[i].0 .0 == first
                && k_unique_map_range_of_instance_type_range_list[i].0 .1 == last
            {
                return Some(k_unique_map_range_of_instance_type_range_list[i].1);
            }
            i += 1;
        }
        None
    }

    pub const fn unique_map_range_of_instance_type(type_: InstanceType) -> Option<TaggedAddressRange> {
        Self::unique_map_range_of_instance_type_range(type_, type_)
    }

    pub const fn may_have_map_check_fast_case(type_: InstanceType) -> bool {
        if unique_map_of_instance_type(type_).is_some() {
            return true;
        }
        for el in &K_UNIQUE_MAP_RANGE_OF_INSTANCE_TYPE_RANGE_LIST {
            if el.0 .0 <= type_ && type_ <= el.0 .1 {
                return true;
            }
        }
        false
    }

    pub fn check_instance_map(expected: RootIndex, map: Tagged<Map>) -> bool {
        crate::objects::map_inl::V8HeapCompressionScheme::compress_object(map.ptr())
            == crate::objects::map_inl::StaticReadOnlyRootsPointerTable[expected as usize]
    }

    pub fn check_instance_map_range(expected: TaggedAddressRange, map: Tagged<Map>) -> bool {
        let ptr = crate::objects::map_inl::V8HeapCompressionScheme::compress_object(map.ptr());
        crate::base::bounds::is_in_range(ptr, expected.0, expected.1)
    }

    #[cfg(not(v8_static_roots_bool))]
    pub const fn may_have_map_check_fast_case(type_: InstanceType) -> bool {
        false
    }

    macro_rules! instance_type_checker1 {
        ($type:ident, $forinstancetype:ident) => {
            impl crate::objects::instance_type_inl::instance_type_checker::InstanceTypeTraits::$type {
                pub const fn is_##type(instance_type: InstanceType) -> bool {
                    instance_type == InstanceType::$forinstancetype
                }
            }
        };
    }

    #[cfg(v8_static_roots_bool)]
    macro_rules! instance_type_checker2 {
        ($type:ident, $forinstancetype_:ident) => {
            impl crate::objects::instance_type_inl::instance_type_checker::InstanceTypeTraits::$type {
                pub fn is_##type(map_object: Tagged<Map>) -> bool {
                    const FORINSTANCETYPE: InstanceType = InstanceType::$forinstancetype_;
                    if let Some(index) = unique_map_of_instance_type(FORINSTANCETYPE) {
                        return check_instance_map(index, map_object);
                    }
                    if let Some(map_range) = unique_map_range_of_instance_type(FORINSTANCETYPE) {
                        return check_instance_map_range(map_range, map_object);
                    }
                    return Self::is_##type(map_object.instance_type());
                }
            }
        };
    }

    #[cfg(not(v8_static_roots_bool))]
    macro_rules! instance_type_checker2 {
        ($type:ident, $forinstancetype:ident) => {
            impl crate::objects::instance_type_inl::instance_type_checker::InstanceTypeTraits::$type {
                pub fn is_##type(map_object: Tagged<Map>) -> bool {
                    Self::is_##type(map_object.instance_type())
                }
            }
        };
    }

    macro_rules! instance_type_checkers_single {
        ($instance_type_checker1:ident) => {};
    }

    instance_type_checkers_single!(instance_type_checker1);
    macro_rules! instance_type_checkers_single {
        ($instance_type_checker2:ident) => {};
    }

    instance_type_checkers_single!(instance_type_checker2);

    pub struct InstanceRangeChecker<const LOWER_LIMIT: InstanceType, const UPPER_LIMIT: InstanceType>;
    impl<const LOWER_LIMIT: InstanceType, const UPPER_LIMIT: InstanceType> InstanceRangeChecker<LOWER_LIMIT, UPPER_LIMIT> {
        pub const fn check(value: InstanceType) -> bool {
            crate::base::bounds::is_in_range(value, LOWER_LIMIT, UPPER_LIMIT)
        }
    }

    impl<const UPPER_LIMIT: InstanceType> InstanceRangeChecker<FIRST_TYPE, UPPER_LIMIT> {
        pub const fn check(value: InstanceType) -> bool {
            debug_assert!(FIRST_TYPE <= value);
            value <= UPPER_LIMIT
        }
    }

    impl<const LOWER_LIMIT: InstanceType> InstanceRangeChecker<LOWER_LIMIT, LAST_TYPE> {
        pub const fn check(value: InstanceType) -> bool {
            debug_assert!(LAST_TYPE >= value);
            value >= LOWER_LIMIT
        }
    }

    macro_rules! instance_type_checker_range1 {
        ($type:ident, $first_instance_type:ident, $last_instance_type:ident) => {
            impl crate::objects::instance_type_inl::instance_type_checker::InstanceTypeTraits::$type {
                pub const fn is_##type(instance_type: InstanceType) -> bool {
                    InstanceRangeChecker::<InstanceType::$first_instance_type, InstanceType::$last_instance_type>::check(instance_type)
                }
            }
        };
    }

    #[cfg(v8_static_roots_bool)]
    macro_rules! instance_type_checker_range2 {
        ($type:ident, $first_instance_type:ident, $last_instance_type:ident) => {
            impl crate::objects::instance_type_inl::instance_type_checker::InstanceTypeTraits::$type {
                pub fn is_##type(map_object: Tagged<Map>) -> bool {
                    if let Some(maybe_range) = unique_map_range_of_instance_type_range(
                        InstanceType::$first_instance_type,
                        InstanceType::$last_instance_type,
                    ) {
                        return check_instance_map_range(maybe_range, map_object);
                    }
                    Self::is_##type(map_object.instance_type())
                }
            }
        };
    }

    #[cfg(not(v8_static_roots_bool))]
    macro_rules! instance_type_checker_range2 {
        ($type:ident, $first_instance_type:ident, $last_instance_type:ident) => {
            impl crate::objects::instance_type_inl::instance_type_checker::InstanceTypeTraits::$type {
                pub fn is_##type(map_object: Tagged<Map>) -> bool {
                    Self::is_##type(map_object.instance_type())
                }
            }
        };
    }

    macro_rules! instance_type_checkers_range {
        ($instance_type_checker_range1:ident) => {};
    }

    instance_type_checkers_range!(instance_type_checker_range1);

    macro_rules! instance_type_checkers_range {
        ($instance_type_checker_range2:ident) => {};
    }

    instance_type_checkers_range!(instance_type_checker_range2);

    impl crate::objects::instance_type_inl::instance_type_checker::InstanceTypeTraits::HeapObject {
        pub const fn is_heap_object(instance_type: InstanceType) -> bool {
            true
        }
    }

    impl crate::objects::instance_type_inl::instance_type_checker::InstanceTypeTraits::InternalizedString {
        pub const fn is_internalized_string(instance_type: InstanceType) -> bool {
            assert_ne!(K_NOT_INTERNALIZED_TAG, 0);
            (instance_type as i32 & (K_IS_NOT_STRING_MASK | K_IS_NOT_INTERNALIZED_MASK))
                == (K_STRING_TAG | K_INTERNALIZED_TAG) as i32
        }
        pub fn is_internalized_string(map_object: Tagged<Map>) -> bool {
            #[cfg(v8_static_roots_bool)]
            return check_instance_map_range(
                KUniqueMapRangeOfStringType::K_INTERNALIZED_STRING,
                map_object,
            );
            #[cfg(not(v8_static_roots_bool))]
            return Self::is_internalized_string(map_object.instance_type());
        }
    }

    impl crate::objects::instance_type_inl::instance_type_checker::InstanceTypeTraits::SeqString {
        pub const fn is_seq_string(instance_type: InstanceType) -> bool {
            (instance_type as i32 & (K_IS_NOT_STRING_MASK | K_STRING_REPRESENTATION_MASK))
                == K_SEQ_STRING_TAG as i32
        }

        pub fn is_seq_string(map_object: Tagged<Map>) -> bool {
            #[cfg(v8_static_roots_bool)]
            return check_instance_map_range(KUniqueMapRangeOfStringType::K_SEQ_STRING, map_object);
            #[cfg(not(v8_static_roots_bool))]
            return Self::is_seq_string(map_object.instance_type());
        }
    }

    impl crate::objects::instance_type_inl::instance_type_checker::InstanceTypeTraits::ExternalString {
        pub const fn is_external_string(instance_type: InstanceType) -> bool {
            (instance_type as i32 & (K_IS_NOT_STRING_MASK | K_STRING_REPRESENTATION_MASK))
                == K_EXTERNAL_STRING_TAG as i32
        }
        pub fn is_external_string(map_object: Tagged<Map>) -> bool {
            #[cfg(v8_static_roots_bool)]
            return check_instance_map_range(
                KUniqueMapRangeOfStringType::K_EXTERNAL_STRING,
                map_object,
            );
            #[cfg(not(v8_static_roots_bool))]
            return Self::is_external_string(map_object.instance_type());
        }
    }

    impl crate::objects::instance_type_inl::instance_type_checker::InstanceTypeTraits::UncachedExternalString {
        pub const fn is_uncached_external_string(instance_type: InstanceType) -> bool {
            (instance_type as i32 & (K_IS_NOT_STRING_MASK | kUncachedExternalStringMask | K_STRING_REPRESENTATION_MASK))
                == (K_EXTERNAL_STRING_TAG | K_UNCACHED_EXTERNAL_STRING_TAG) as i32
        }

        pub fn is_uncached_external_string(map_object: Tagged<Map>) -> bool {
            #[cfg(v8_static_roots_bool)]
            return check_instance_map_range(
                KUniqueMapRangeOfStringType::K_UNCACHED_EXTERNAL_STRING,
                map_object,
            );
            #[cfg(not(v8_static_roots_bool))]
            return Self::is_uncached_external_string(map_object.instance_type());
        }
    }

    impl crate::objects::instance_type_inl::instance_type_checker::InstanceTypeTraits::ConsString {
        pub const fn is_cons_string(instance_type: InstanceType) -> bool {
            (instance_type as i32 & K_STRING_REPRESENTATION_MASK) == K_CONS_STRING_TAG as i32
        }
        pub fn is_cons_string(map_object: Tagged<Map>) -> bool {
            #[cfg(v8_static_roots_bool)]
            return check_instance_map_range(KUniqueMapRangeOfStringType::K_CONS_STRING, map_object);
            #[cfg(not(v8_static_roots_bool))]
            return Self::is_cons_string(map_object.instance_type());
        }
    }

    impl crate::objects::instance_type_inl::instance_type_checker::InstanceTypeTraits::SlicedString {
        pub const fn is_sliced_string(instance_type: InstanceType) -> bool {
            (instance_type as i32 & K_STRING_REPRESENTATION_MASK) == K_SLICED_STRING_TAG as i32
        }

        pub fn is_sliced_string(map_object: Tagged<Map>) -> bool {
            #[cfg(v8_static_roots_bool)]
            return check_instance_map_range(
                KUniqueMapRangeOfStringType::K_SLICED_STRING,
                map_object,
            );
            #[cfg(not(v8_static_roots_bool))]
            return Self::is_sliced_string(map_object.instance_type());
        }
    }

    impl crate::objects::instance_type_inl::instance_type_checker::InstanceTypeTraits::ThinString {
        pub const fn is_thin_string(instance_type: InstanceType) -> bool {
            (instance_type as i32 & K_STRING_REPRESENTATION_MASK) == K_THIN_STRING_TAG as i32
        }

        pub fn is_thin_string(map_object: Tagged<Map>) -> bool {
            #[cfg(v8_static_roots_bool)]
            return check_instance_map_range(KUniqueMapRangeOfStringType::K_THIN_STRING, map_object);
            #[cfg(not(v8_static_roots_bool))]
            return Self::is_thin_string(map_object.instance_type());
        }
    }

    impl crate::objects::instance_type_inl::instance_type_checker::InstanceTypeTraits::OneByteString {
        pub const fn is_one_byte_string(instance_type: InstanceType) -> bool {
            debug_assert!(Self::is_string(instance_type));
            (instance_type as i32 & K_STRING_ENCODING_MASK) == K_ONE_BYTE_STRING_TAG as i32
        }

        pub fn is_one_byte_string(map_object: Tagged<Map>) -> bool {
            #[cfg(v8_static_roots_bool)]
            {
                debug_assert!(is_string_map(map_object));

                let ptr = crate::objects::map_inl::V8HeapCompressionScheme::compress_object(map_object.ptr());
                return (ptr & K_STRING_MAP_ENCODING_MASK as Tagged_t) == K_ONE_BYTE_STRING_MAP_BIT as Tagged_t;
            }
            #[cfg(not(v8_static_roots_bool))]
            return Self::is_one_byte_string(map_object.instance_type());
        }
    }

    impl crate::objects::instance_type_inl::instance_type_checker::InstanceTypeTraits::TwoByteString {
        pub const fn is_two_byte_string(instance_type: InstanceType) -> bool {
            debug_assert!(Self::is_string(instance_type));
            (instance_type as i32 & K_STRING_ENCODING_MASK) == K_TWO_BYTE_STRING_TAG as i32
        }

        pub fn is_two_byte_string(map_object: Tagged<Map>) -> bool {
            #[cfg(v8_static_roots_bool)]
            {
                debug_assert!(is_string_map(map_object));
                let ptr = crate::objects::map_inl::V8HeapCompressionScheme::compress_object(map_object.ptr());
                return (ptr & K_STRING_MAP_ENCODING_MASK as Tagged_t) == K_TWO_BYTE_STRING_MAP_BIT as Tagged_t;
            }
            #[cfg(not(v8_static_roots_bool))]
            return Self::is_two_byte_string(map_object.instance_type());
        }
    }

    impl crate::objects::instance_type_inl::instance_type_checker::InstanceTypeTraits::ReferenceComparable {
        pub const fn is_reference_comparable(instance_type: InstanceType) -> bool {
            !Self::is_string(instance_type) && !Self::is_bigint(instance_type) && instance_type != InstanceType::HEAP_NUMBER_TYPE
        }
    }

    impl crate::objects::instance_type_inl::instance_type_checker::InstanceTypeTraits::GcSafeCode {
        pub const fn is_gc_safe_code(instance_type: InstanceType) -> bool {
            Self::is_code(instance_type)
        }
        pub fn is_gc_safe_code(map_object: Tagged<Map>) -> bool {
            Self::is_code(map_object)
        }
    }

    impl crate::objects::instance_type_inl::instance_type_checker::InstanceTypeTraits::AbstractCode {
        pub const fn is_abstract_code(instance_type: InstanceType) -> bool {
            Self::is_bytecode_array(instance_type) || Self::is_code(instance_type)
        }
        pub fn is_abstract_code(map_object: Tagged<Map>) -> bool {
            Self::is_abstract_code(map_object.instance_type())
        }
    }

    impl crate::objects::instance_type_inl::instance_type_checker::InstanceTypeTraits::FreeSpaceOrFiller {
        pub const fn is_free_space_or_filler(instance_type: InstanceType) -> bool {
            instance_type == InstanceType::FREE_SPACE_TYPE || instance_type == InstanceType::FILLER_TYPE
        }
        pub fn is_free_space_or_filler(map_object: Tagged<Map>) -> bool {
            Self::is_free_space_or_filler(map_object.instance_type())
        }
    }

    impl crate::objects::instance_type_inl::instance_type_checker::InstanceTypeTraits::MaybeReadOnlyJSObject {
        pub const fn is_maybe_read_only_js_object(instance_type: InstanceType) -> bool {
            Self::is_js_external_object(instance_type) || Self::is_js_message_object(instance_type)
        }
        pub fn is_maybe_read_only_js_object(map_object: Tagged<Map>) -> bool {
            Self::is_maybe_read_only_js_object(map_object.instance_type())
        }
    }

    impl crate::objects::instance_type_inl::instance_type_checker::InstanceTypeTraits::PropertyDictionary {
        pub const fn is_property_dictionary(instance_type: InstanceType) -> bool {
            instance_type == InstanceType::PROPERTY_DICTIONARY_TYPE
        }
        pub fn is_property_dictionary(map_object: Tagged<Map>) -> bool {
            Self::is_property_dictionary(map_object.instance_type())
        }
    }

    impl crate::objects::instance_type_inl::instance_type_checker::InstanceTypeTraits::NativeContextSpecific {
        pub const fn is_native_context_specific(instance_type: InstanceType) -> bool {
            if Self::is_context(instance_type) {
                return true;
            }
            if !Self::is_js_receiver(instance_type) {
                return false;
            }
            if instance_type == InstanceType::JS_MESSAGE_OBJECT_TYPE
                || instance_type == InstanceType::JS_EXTERNAL_OBJECT_TYPE
            {
                return false;
            } else if always_shared_space_js_object(instance_type) {
                return false;
            }

            #[cfg(v8_enable_webassembly)]
            {
                if Self::is_wasm_object(instance_type) {
                    return false;
                }
            }

            true
        }

        pub fn is_native_context_specific_map(map_object: Tagged<Map>) -> bool {
            Self::is_native_context_specific(map_object.instance_type())
        }
    }
}

macro_rules! type_checker {
    ($type:ident, $($tt:tt)*) => {
        impl crate::objects::map_inl::Tagged<crate::objects::map_inl::Map> {
            pub fn is_##type##_map(&self) -> bool {
                crate::objects::instance_type_inl::instance_type_checker::InstanceTypeTraits::$type::is_##type(self.clone())
            }
        }
    };
}
macro_rules! instance_type_checkers {
    ($type_checker:ident) => {};
}

instance_type_checkers!(type_checker);
}
