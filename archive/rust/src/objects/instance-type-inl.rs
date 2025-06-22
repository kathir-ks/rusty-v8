// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

mod base {
    pub mod bounds {
        #[inline]
        pub const fn is_in_range<T: PartialOrd>(value: T, lower: T, upper: T) -> bool {
            value >= lower && value <= upper
        }
    }

    pub mod bits {
        #[inline]
        pub const fn count_trailing_zeros_non_zero(x: usize) -> u32 {
            x.trailing_zeros()
        }
    }
}

mod execution {
    pub mod isolate_utils {
        // This module would contain utilities related to isolates,
        // which are not directly translatable without more context.
    }
}

mod objects {
    pub mod instance_type {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        #[repr(u16)]
        pub enum InstanceType {
            // TODO: Define all instance types here based on src/objects/instance-type.h
            // Example:
            // JS_OBJECT_TYPE = 0,
            // MAP_TYPE = 1,
            FREE_SPACE_TYPE = 2,
            FILLER_TYPE = 3,
            PROPERTY_DICTIONARY_TYPE = 4,
            JS_MESSAGE_OBJECT_TYPE = 5,
            JS_EXTERNAL_OBJECT_TYPE = 6,
            HEAP_NUMBER_TYPE = 7,
            BIGINT_TYPE = 8,
            ODDBALL_TYPE = 9,
            CODE_TYPE = 10,
            BYTECODE_ARRAY_TYPE = 11,
            ALLOCATION_SITE_TYPE = 12,
            STRING_TYPE = 13,
            INTERNALIZED_STRING_TYPE = 14,
            SEQ_STRING_TYPE = 15,
            EXTERNAL_STRING_TYPE = 16,
            UNCACHED_EXTERNAL_STRING_TYPE = 17,
            CONS_STRING_TYPE = 18,
            SLICED_STRING_TYPE = 19,
            THIN_STRING_TYPE = 20,
            FIRST_STRING_TYPE = 21,
            LAST_STRING_TYPE = 22,
            NAME_TYPE = 23,
            SYMBOL_TYPE = 24,
            FIRST_NAME_TYPE = 25,
            LAST_NAME_TYPE = 26,
            FIRST_SMALL_ORDERED_HASH_TABLE_TYPE = 27,
            LAST_SMALL_ORDERED_HASH_TABLE_TYPE = 28,
            FIRST_ABSTRACT_INTERNAL_CLASS_TYPE = 29,
            LAST_ABSTRACT_INTERNAL_CLASS_TYPE = 30,
            FIRST_TURBOFAN_TYPE_TYPE = 31,
            LAST_TURBOFAN_TYPE_TYPE = 32,
            CONTEXT_TYPE = 33,
            JS_RECEIVER_TYPE = 34,
            ALWAYS_SHARED_SPACE_JS_OBJECT_TYPE = 35,
            WASM_OBJECT_TYPE = 36,
            FIRST_TYPE = 37,
            LAST_TYPE = 38,
        }

        pub const kNotInternalizedTag: u16 = 0x8000;
        pub const kIsNotStringMask: u16 = 0x4000;
        pub const kIsNotInternalizedMask: u16 = 0x2000;
        pub const kStringTag: u16 = 0x1000;
        pub const kInternalizedTag: u16 = 0x0800;
        pub const kStringRepresentationMask: u16 = 0x0700;
        pub const kSeqStringTag: u16 = 0x0100;
        pub const kExternalStringTag: u16 = 0x0200;
        pub const kUncachedExternalStringMask: u16 = 0x0400;
        pub const kConsStringTag: u16 = 0x0300;
        pub const kSlicedStringTag: u16 = 0x0500;
        pub const kThinStringTag: u16 = 0x0600;
        pub const kStringEncodingMask: u16 = 0x0080;
        pub const kOneByteStringTag: u16 = 0x0000;
        pub const kTwoByteStringTag: u16 = 0x0080;

        impl InstanceType {
            pub fn is_string(self) -> bool {
                (self as u16 & kIsNotStringMask) == 0
            }

            pub fn is_bigint(self) -> bool {
                self == InstanceType::BIGINT_TYPE
            }

            pub fn is_code(self) -> bool {
                self == InstanceType::CODE_TYPE
            }

            pub fn is_bytecode_array(self) -> bool {
                self == InstanceType::BYTECODE_ARRAY_TYPE
            }

            pub fn is_context(self) -> bool {
                self == InstanceType::CONTEXT_TYPE
            }

            pub fn is_js_receiver(self) -> bool {
                self == InstanceType::JS_RECEIVER_TYPE
            }

            pub fn is_always_shared_space_js_object(self) -> bool {
                self == InstanceType::ALWAYS_SHARED_SPACE_JS_OBJECT_TYPE
            }

            pub fn is_wasm_object(self) -> bool {
                self == InstanceType::WASM_OBJECT_TYPE
            }

            pub fn is_string_map(self) -> bool {
                self.is_string()
            }
        }
    }

    pub mod map {
        use super::instance_type::InstanceType;

        // Placeholder for Tagged<Map> type.  Needs more context.
        #[derive(Debug, Copy, Clone)]
        pub struct Map {
            instance_type: InstanceType,
        }

        impl Map {
            pub const kSize: usize = 8; // Example size

            pub fn instance_type(&self) -> InstanceType {
                self.instance_type
            }

            pub fn try_get_map_root_idx_for(_type: InstanceType) -> Option<RootIndex> {
                // Placeholder for TryGetMapRootIdxFor function.  Needs more context.
                None
            }
        }
    }

    pub mod instance_type_checker {
        use super::instance_type::InstanceType;
        use super::instance_type::*;
        use super::map::Map;
        use std::marker::PhantomData;
        use std::option::Option;

        #[derive(Debug, Copy, Clone)]
        pub struct Tagged<T> {
            ptr: *mut T,
            _phantom: PhantomData<T>,
        }

        impl<T> Tagged<T> {
            pub fn ptr(&self) -> *mut T {
                self.ptr
            }
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum RootIndex {
            kAllocationSiteWithWeakNextMap,
            kAllocationSiteWithoutWeakNextMap,
            kSeqTwoByteStringMap,
            kSymbolMap,
            kUndefinedMap,
            kBooleanMap,
            kHeapNumberMap,
            kBigIntMap,
            kSmallOrderedHashMapMap,
            kSmallOrderedNameDictionaryMap,
            kAbstractInternalClassSubclass1Map,
            kAbstractInternalClassSubclass2Map,
            kTurbofanBitsetTypeMap,
            kTurbofanOtherNumberConstantTypeMap,
            kInternalizedOneByteStringMap,
            kInternalizedTwoByteStringMap,
            kUncachedExternalInternalizedOneByteStringMap,
            kExternalInternalizedTwoByteStringMap,
            kSharedExternalOneByteStringMap,
            kSharedUncachedExternalOneByteStringMap,
            kUncachedExternalInternalizedTwoByteStringMap,
            kConsTwoByteStringMap,
            kConsOneByteStringMap,
            kSlicedTwoByteStringMap,
            kSlicedOneByteStringMap,
            kThinTwoByteStringMap,
            kThinOneByteStringMap,
            kSeqOneByteStringMap,
        }

        pub type TaggedAddress = usize;

        pub mod InstanceTypeTraits {
            pub struct AllocationSite;
            pub struct Map;
            pub struct Oddball;
            pub struct HeapNumber;
            pub struct BigInt;
            pub struct Code;
            pub struct BytecodeArray;
            pub struct JSFunction;
            pub struct JSArray;
            //TODO Add other traits
        }

        pub const kStringMapLowerBound: TaggedAddress = 0x1000; // Placeholder
        pub const kStringMapUpperBound: TaggedAddress = 0x2000; // Placeholder

        pub fn unique_map_of_instance_type_check<T>() -> Option<RootIndex> {
            None
        }

        macro_rules! instance_type_map {
            ($v:ident, $rootIndexName:ident, $rootAccessorName:ident, $class_name:ident) => {
                pub fn unique_map_of_instance_type_check_for_$class_name() -> Option<RootIndex> {
                    Some(RootIndex::$rootIndexName)
                }
            };
        }

        macro_rules! unique_instance_type_map_list_generator {
            ($instance_type_map:ident, $_:tt) => {
                // Example Usage
                // $instance_type_map!(V, AllocationSiteMap, allocation_site_map, AllocationSite);
                // $instance_type_map!(V, MapMap, map_map, Map);
            };
        }

        //This requires a lot of root index information, which I do not have.
        //unique_instance_type_map_list_generator!(instance_type_map, _);

        pub fn unique_map_of_instance_type(type_: InstanceType) -> Option<RootIndex> {
            macro_rules! instance_type_check {
                ($it:ident, $forinstancetype:expr) => {
                    if type_ == $forinstancetype {
                        return unique_map_of_instance_type_check::<InstanceTypeTraits::$it>();
                    }
                };
            }

            // TODO: Fill in all InstanceType variants
            // Example Usage
            // instance_type_check!(Map, InstanceType::MAP_TYPE);

            //instance_type_check!(AllocationSite, InstanceType::ALLOCATION_SITE_TYPE);

            Map::try_get_map_root_idx_for(type_)
        }

        pub type InstanceTypeRange = (InstanceType, InstanceType);
        pub type TaggedAddressRange = (TaggedAddress, TaggedAddress);

        #[cfg(feature = "V8_STATIC_ROOTS_BOOL")]
        pub mod static_roots {
            use super::*;

            pub const kUniqueMapRangeOfInstanceTypeRangeList: [((InstanceType, InstanceType), (TaggedAddress, TaggedAddress)); 9] = [
                ((InstanceType::ALLOCATION_SITE_TYPE, InstanceType::ALLOCATION_SITE_TYPE),
                 (0, 1)),
                 ((InstanceType::FIRST_STRING_TYPE, InstanceType::LAST_STRING_TYPE), (0, 1)),
                 ((InstanceType::FIRST_NAME_TYPE, InstanceType::LAST_NAME_TYPE), (0, 1)),
                 ((InstanceType::ODDBALL_TYPE, InstanceType::ODDBALL_TYPE), (0, 1)),
                 ((InstanceType::HEAP_NUMBER_TYPE, InstanceType::ODDBALL_TYPE), (0, 1)),
                 ((InstanceType::BIGINT_TYPE, InstanceType::HEAP_NUMBER_TYPE), (0, 1)),
                 ((InstanceType::FIRST_SMALL_ORDERED_HASH_TABLE_TYPE, InstanceType::LAST_SMALL_ORDERED_HASH_TABLE_TYPE), (0, 1)),
                 ((InstanceType::FIRST_ABSTRACT_INTERNAL_CLASS_TYPE, InstanceType::LAST_ABSTRACT_INTERNAL_CLASS_TYPE), (0, 1)),
                 ((InstanceType::FIRST_TURBOFAN_TYPE_TYPE, InstanceType::LAST_TURBOFAN_TYPE_TYPE), (0, 1)),
            ];

            pub struct KUniqueMapRangeOfStringType;

            impl KUniqueMapRangeOfStringType {
                pub const kSeqString: TaggedAddressRange = (0, 1);
                pub const kInternalizedString: TaggedAddressRange = (0, 1);
                pub const kExternalString: TaggedAddressRange = (0, 1);
                pub const kUncachedExternalString: TaggedAddressRange = (0, 1);
                pub const kConsString: TaggedAddressRange = (0, 1);
                pub const kSlicedString: TaggedAddressRange = (0, 1);
                pub const kThinString: TaggedAddressRange = (0, 1);
            }

            pub const kStringMapEncodingMask: i32 = 1 << base::bits::count_trailing_zeros_non_zero(Map::kSize) as i32;
            pub const kOneByteStringMapBit: i32 = 1 & kStringMapEncodingMask;
            pub const kTwoByteStringMapBit: i32 = 1 & kStringMapEncodingMask;

            pub const fn unique_map_range_of_instance_type_range(first: InstanceType, last: InstanceType) -> Option<TaggedAddressRange> {
                let mut i = 0;
                while i < kUniqueMapRangeOfInstanceTypeRangeList.len() {
                    let (range, address_range) = kUniqueMapRangeOfInstanceTypeRangeList[i];
                    if range.0 == first && range.1 == last {
                        return Some(address_range);
                    }
                    i += 1;
                }
                None
            }

            pub const fn unique_map_range_of_instance_type(type_: InstanceType) -> Option<TaggedAddressRange> {
                unique_map_range_of_instance_type_range(type_, type_)
            }

            pub fn may_have_map_check_fast_case(type_: InstanceType) -> bool {
                if unique_map_of_instance_type(type_).is_some() {
                    return true;
                }

                for &(range, _) in &kUniqueMapRangeOfInstanceTypeRangeList {
                    if range.0 <= type_ && type_ <= range.1 {
                        return true;
                    }
                }

                false
            }

            pub fn check_instance_map(expected: RootIndex, _map: Tagged<Map>) -> bool {
                // Placeholder for CheckInstanceMap function.  Needs more context on
                // V8HeapCompressionScheme and StaticReadOnlyRootsPointerTable.
                //V8HeapCompressionScheme::CompressObject(map.ptr()) ==
                // StaticReadOnlyRootsPointerTable[static_cast<size_t>(expected)]
                false
            }

            pub fn check_instance_map_range(expected: TaggedAddressRange, _map: Tagged<Map>) -> bool {
                // Placeholder for CheckInstanceMapRange function.  Needs more context on
                // V8HeapCompressionScheme.
                //Tagged_t ptr = V8HeapCompressionScheme::CompressObject(map.ptr());
                //base::IsInRange(ptr, expected.first, expected.second)
                false
            }
        }

        #[cfg(not(feature = "V8_STATIC_ROOTS_BOOL"))]
        pub mod static_roots {
            use super::*;
            pub fn may_have_map_check_fast_case(_type: InstanceType) -> bool {
                false
            }
        }

        macro_rules! instance_type_checker1 {
            ($type:ident, $forinstancetype:expr) => {
                pub const fn is_$type(instance_type: InstanceType) -> bool {
                    instance_type == $forinstancetype
                }
            };
        }

        #[cfg(feature = "V8_STATIC_ROOTS_BOOL")]
        macro_rules! instance_type_checker2 {
            ($type:ident, $forinstancetype_:expr) => {
                pub fn is_$type(map_object: Tagged<Map>) -> bool {
                    const FOR_INSTANCE_TYPE: InstanceType =
                        unsafe { std::mem::transmute($forinstancetype_ as u16) };
                    if let Some(index) = unique_map_of_instance_type(FOR_INSTANCE_TYPE) {
                        return static_roots::check_instance_map(index, map_object);
                    }
                    if let Some(map_range) =
                        static_roots::unique_map_range_of_instance_type(FOR_INSTANCE_TYPE)
                    {
                        return static_roots::check_instance_map_range(map_range, map_object);
                    }
                    is_$type(unsafe { (*map_object.ptr()).instance_type() })
                }
            };
        }

        #[cfg(not(feature = "V8_STATIC_ROOTS_BOOL"))]
        macro_rules! instance_type_checker2 {
            ($type:ident, $forinstancetype:expr) => {
                pub fn is_$type(map_object: Tagged<Map>) -> bool {
                    is_$type(unsafe { (*map_object.ptr()).instance_type() })
                }
            };
        }

        macro_rules! instance_type_checkers_single {
            ($macro:ident) => {
                $macro!(Map, InstanceType::MAP_TYPE);
                $macro!(Oddball, InstanceType::ODDBALL_TYPE);
                $macro!(HeapNumber, InstanceType::HEAP_NUMBER_TYPE);
                $macro!(BigInt, InstanceType::BIGINT_TYPE);
                $macro!(Code, InstanceType::CODE_TYPE);
                $macro!(BytecodeArray, InstanceType::BYTECODE_ARRAY_TYPE);
                //TODO: add all instance types.
            };
        }

        instance_type_checkers_single!(instance_type_checker1);
        instance_type_checkers_single!(instance_type_checker2);

        pub struct InstanceRangeChecker<const LOWER_LIMIT: InstanceType, const UPPER_LIMIT: InstanceType>;

        impl<const LOWER_LIMIT: InstanceType, const UPPER_LIMIT: InstanceType> InstanceRangeChecker<LOWER_LIMIT, UPPER_LIMIT> {
            pub const fn check(value: InstanceType) -> bool {
                base::bounds::is_in_range(value as u16, LOWER_LIMIT as u16, UPPER_LIMIT as u16)
            }
        }

        impl<const UPPER_LIMIT: InstanceType> InstanceRangeChecker<{ InstanceType::FIRST_TYPE }, UPPER_LIMIT> {
            pub const fn check(value: InstanceType) -> bool {
                assert!(InstanceType::FIRST_TYPE <= value);
                value <= UPPER_LIMIT
            }
        }

        impl<const LOWER_LIMIT: InstanceType> InstanceRangeChecker<LOWER_LIMIT, { InstanceType::LAST_TYPE }> {
            pub const fn check(value: InstanceType) -> bool {
                assert!(InstanceType::LAST_TYPE >= value);
                value >= LOWER_LIMIT
            }
        }

        macro_rules! instance_type_checker_range1 {
            ($type:ident, $first_instance_type:expr, $last_instance_type:expr) => {
                pub const fn is_$type(instance_type: InstanceType) -> bool {
                    InstanceRangeChecker::<{$first_instance_type}, {$last_instance_type}>::check(
                        instance_type,
                    )
                }
            };
        }

        #[cfg(feature = "V8_STATIC_ROOTS_BOOL")]
        macro_rules! instance_type_checker_range2 {
            ($type:ident, $first_instance_type:expr, $last_instance_type:expr) => {
                pub fn is_$type(map_object: Tagged<Map>) -> bool {
                    if let Some(maybe_range) = static_roots::unique_map_range_of_instance_type_range(
                        unsafe { std::mem::transmute($first_instance_type as u16) },
                        unsafe { std::mem::transmute($last_instance_type as u16) },
                    ) {
                        return static_roots::check_instance_map_range(maybe_range, map_object);
                    }
                    is_$type(unsafe { (*map_object.ptr()).instance_type() })
                }
            };
        }

        #[cfg(not(feature = "V8_STATIC_ROOTS_BOOL"))]
        macro_rules! instance_type_checker_range2 {
            ($type:ident, $first_instance_type:expr, $last_instance_type:expr) => {
                pub fn is_$type(map_object: Tagged<Map>) -> bool {
                    is_$type(unsafe { (*map_object.ptr()).instance_type() })
                }
            };
        }

        macro_rules! instance_type_checkers_range {
            ($macro:ident) => {
                $macro!(String, InstanceType::FIRST_STRING_TYPE, InstanceType::LAST_STRING_TYPE);
                $macro!(Name, InstanceType::FIRST_NAME_TYPE, InstanceType::LAST_NAME_TYPE);
            };
        }

        instance_type_checkers_range!(instance_type_checker_range1);
        instance_type_checkers_range!(instance_type_checker_range2);

        pub const fn is_heap_object(_instance_type: InstanceType) -> bool {
            true
        }

        pub const fn is_internalized_string(instance_type: InstanceType) -> bool {
            assert_ne!(kNotInternalizedTag, 0);
            (instance_type as u16 & (kIsNotStringMask | kIsNotInternalizedMask))
                == (kStringTag | kInternalizedTag)
        }

        pub fn is_internalized_string_map(map_object: Tagged<Map>) -> bool {
            #[cfg(feature = "V8_STATIC_ROOTS_BOOL")]
            {
                static_roots::check_instance_map_range(KUniqueMapRangeOfStringType::kInternalizedString, map_object)
            }
            #[cfg(not(feature = "V8_STATIC_ROOTS_BOOL"))]
            {
                is_internalized_string(unsafe { (*map_object.ptr()).instance_type() })
            }
        }

        pub const fn is_seq_string(instance_type: InstanceType) -> bool {
            (instance_type as u16 & (kIsNotStringMask | kStringRepresentationMask)) == kSeqStringTag
        }

        pub fn is_seq_string_map(map_object: Tagged<Map>) -> bool {
            #[cfg(feature = "V8_STATIC_ROOTS_BOOL")]
            {
                static_roots::check_instance_map_range(KUniqueMapRangeOfStringType::kSeqString, map_object)
            }
            #[cfg(not(feature = "V8_STATIC_ROOTS_BOOL"))]
            {
                is_seq_string(unsafe { (*map_object.ptr()).instance_type() })
            }
        }

        pub const fn is_external_string(instance_type: InstanceType) -> bool {
            (instance_type as u16 & (kIsNotStringMask | kStringRepresentationMask)) == kExternalStringTag
        }

        pub fn is_external_string_map(map_object: Tagged<Map>) -> bool {
            #[cfg(feature = "V8_STATIC_ROOTS_BOOL")]
            {
                static_roots::check_instance_map_range(KUniqueMapRangeOfStringType::kExternalString, map_object)
            }
            #[cfg(not(feature = "V8_STATIC_ROOTS_BOOL"))]
            {
                is_external_string(unsafe { (*map_object.ptr()).instance_type() })
            }
        }

        pub const fn is_uncached_external_string(instance_type: InstanceType) -> bool {
            (instance_type as u16 & (kIsNotStringMask | kUncachedExternalStringMask | kStringRepresentationMask))
                == (kExternalStringTag | kUncachedExternalStringTag)
        }

        pub fn is_uncached_external_string_map(map_object: Tagged<Map>) -> bool {
            #[cfg(feature = "V8_STATIC_ROOTS_BOOL")]
            {
                static_roots::check_instance_map_range(
                    KUniqueMapRangeOfStringType::kUncachedExternalString,
                    map_object,
                )
            }
            #[cfg(not(feature = "V8_STATIC_ROOTS_BOOL"))]
            {
                is_uncached_external_string(unsafe { (*map_object.ptr()).instance_type() })
            }
        }

        pub const fn is_cons_string(instance_type: InstanceType) -> bool {
            (instance_type as u16 & kStringRepresentationMask) == kConsStringTag
        }

        pub fn is_cons_string_map(map_object: Tagged<Map>) -> bool {
            #[cfg(feature = "V8_STATIC_ROOTS_BOOL")]
            {
                static_roots::check_instance_map_range(KUniqueMapRangeOfStringType::kConsString, map_object)
            }
            #[cfg(not(feature = "V8_STATIC_ROOTS_BOOL"))]
            {
                is_cons_string(unsafe { (*map_object.ptr()).instance_type() })
            }
        }

        pub const fn is_sliced_string(instance_type: InstanceType) -> bool {
            (instance_type as u16 & kStringRepresentationMask) == kSlicedStringTag
        }

        pub fn is_sliced_string_map(map_object: Tagged<Map>) -> bool {
            #[cfg(feature = "V8_STATIC_ROOTS_BOOL")]
            {
                static_roots::check_instance_map_range(KUniqueMapRangeOfStringType::kSlicedString, map_object)
            }
            #[cfg(not(feature = "V8_STATIC_ROOTS_BOOL"))]
            {
                is_sliced_string(unsafe { (*map_object.ptr()).instance_type() })
            }
        }

        pub const fn is_thin_string(instance_type: InstanceType) -> bool {
            (instance_type as u16 & kStringRepresentationMask) == kThinStringTag
        }

        pub fn is_thin_string_map(map_object: Tagged<Map>) -> bool {
            #[cfg(feature = "V8_STATIC_ROOTS_BOOL")]
            {
                static_roots::check_instance_map_range(KUniqueMapRangeOfStringType::kThinString, map_object)
            }
            #[cfg(not(feature = "V8_STATIC_ROOTS_BOOL"))]
            {
                is_thin_string(unsafe { (*map_object.ptr()).instance_type() })
            }
        }

        pub const fn is_one_byte_string(instance_type: InstanceType) -> bool {
            assert!(instance_type.is_string());
            (instance_type as u16 & kStringEncodingMask) == kOneByteStringTag
        }

        pub fn is_one_byte_string_map(map_object: Tagged<Map>) -> bool {
            #[cfg(feature = "V8_STATIC_ROOTS_BOOL")]
            {
                assert!(unsafe { (*map_object.ptr()).instance_type() }.is_string_map());

                //Tagged_t ptr = V8HeapCompressionScheme::CompressObject(map_object.ptr());
                //return (ptr & kStringMapEncodingMask) == kOneByteStringMapBit;
                false
            }
            #[cfg(not(feature = "V8_STATIC_ROOTS_BOOL"))]
            {
                is_one_byte_string(unsafe { (*map_object.ptr()).instance_type() })
            }
        }

        pub const fn is_two_byte_string(instance_type: InstanceType) -> bool {
            assert!(instance_type.is_string());
            (instance_type as u16 & kStringEncodingMask) == kTwoByteStringTag
        }

        pub fn is_two_byte_string_map(map_object: Tagged<Map>) -> bool {
            #[cfg(feature = "V8_STATIC_ROOTS_BOOL")]
            {
                assert!(unsafe { (*map_object.ptr()).instance_type() }.is_string_map());

                //Tagged_t ptr = V8HeapCompressionScheme::CompressObject(map_object.ptr());
                //return (ptr & kStringMapEncodingMask) == kTwoByteStringMapBit;
                false
            }
            #[cfg(not(feature = "V8_STATIC_ROOTS_BOOL"))]
            {
                is_two_byte_string(unsafe { (*map_object.ptr()).instance_type() })
            }
        }

        pub const fn is_reference_comparable(instance_type: InstanceType) -> bool {
            !instance_type.is_string() && !instance_type.is_bigint() && instance_type != InstanceType::HEAP_NUMBER_TYPE
        }

        pub const fn is_gc_safe_code(instance_type: InstanceType) -> bool {
            instance_type.is_code()
        }

        pub fn is_gc_safe_code_map(map_object: Tagged<Map>) -> bool {
            is_code(map_object)
        }

        pub const fn is_abstract_code(instance_type: InstanceType) -> bool {
            instance_type.is_bytecode_array() || instance_type.is_code()
        }

        pub fn is_abstract_code_map(map_object: Tagged<Map>) -> bool {
            is_abstract_code(unsafe { (*map_object.ptr()).instance_type() })
        }

        pub const fn is_free_space_or_filler(instance_type: InstanceType) -> bool {
            instance_type == InstanceType::FREE_SPACE_TYPE || instance_type == InstanceType::FILLER_TYPE
        }

        pub fn is_free_space_or_filler_map(map_object: Tagged<Map>) -> bool {
            is_free_space_or_filler(unsafe { (*map_object.ptr()).instance_type() })
        }

        pub const fn is_maybe_read_only_js_object(instance_type: InstanceType) -> bool {
            is_js_external_object(instance_type) || is_js_message_object(instance_type)
        }

        pub fn is_maybe_read_only_js_object_map(map_object: Tagged<Map>) -> bool {
            is_maybe_read_only_js_object(unsafe { (*map_object.ptr()).instance_type() })
        }

        pub const fn is_property_dictionary(instance_type: InstanceType) -> bool {
            instance_type == InstanceType::PROPERTY_DICTIONARY_TYPE
        }

        pub fn is_property_dictionary_map(map_object: Tagged<Map>) -> bool {
            is_property_dictionary(unsafe { (*map_object.ptr()).instance_type() })
        }

        pub const fn is_native_context_specific(instance_type: InstanceType) -> bool {
            if instance_type.is_context() {
                return true;
            }
            if !instance_type.is_js_receiver() {
                return false;
            }

            if instance_type == InstanceType::JS_MESSAGE_OBJECT_TYPE || instance_type == InstanceType::JS_EXTERNAL_OBJECT_TYPE {
                return false;
            } else if is_always_shared_space_js_object(instance_type) {
                return false;
            }

            #[cfg(feature = "V8_ENABLE_WEBASSEMBLY")]
            {
                if is_wasm_object(instance_type) {
                    return false;
                }
            }
            true
        }

        pub fn is_native_context_specific_map(map_object: Tagged<Map>) -> bool {
            is_native_context_specific(unsafe { (*map_object.ptr()).instance_type() })
        }

        pub const fn is_always_shared_space_js_object(instance_type: InstanceType) -> bool {
            instance_type == InstanceType::ALWAYS_SHARED_SPACE_JS_OBJECT_TYPE
        }

        pub const fn is_js_external_object(instance_type: InstanceType) -> bool {
            instance_type == InstanceType::JS_EXTERNAL_OBJECT_TYPE
        }

        pub const fn is_js_message_object(instance_type: InstanceType) -> bool {
            instance_type == InstanceType::JS_MESSAGE_OBJECT_TYPE
        }

        #[cfg(feature = "V8_ENABLE_WEBASSEMBLY")]
        pub const fn is_wasm_object(instance_type: InstanceType) -> bool {
            instance_type == InstanceType::WASM_OBJECT_TYPE
        }

        macro_rules! type_checker {
            ($type:ident, $params:tt) => {
                pub fn is_$type_map(map: Tagged<Map>) -> bool {
                    is_$type(map)
                }
            };
        }

        macro_rules! instance_type_checkers {
            ($macro:ident) => {
                $macro!(Map, ());
                $macro!(Oddball, ());
                $macro!(HeapNumber, ());
                $macro!(BigInt, ());
                $macro!(Code, ());
                $macro!(BytecodeArray, ());
                $macro!(String, ());
            };
        }

        instance_type_checkers!(type_checker);
    }
}

mod v8_heap_compression_scheme {
    pub fn compress_object<T>(_ptr: *mut T) -> usize {
        // Placeholder for V8HeapCompressionScheme::CompressObject.  Needs more context.
        0
    }
}

mod static_read_only_roots_pointer_table {
    // Placeholder for StaticReadOnlyRootsPointerTable. Needs more context.
    pub static StaticReadOnlyRootsPointerTable: [usize; 1] = [0];
}