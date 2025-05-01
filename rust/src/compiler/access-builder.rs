// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// #include "src/compiler/access-builder.h"

// #include "src/compiler/type-cache.h"
// #include "src/handles/handles-inl.h"
// #include "src/objects/arguments.h"
// #include "src/objects/contexts.h"
// #include "src/objects/heap-number.h"
// #include "src/objects/js-collection.h"
// #include "src/objects/js-generator.h"
// #include "src/objects/js-objects.h"
// #include "src/objects/objects-inl.h"
// #include "src/objects/ordered-hash-table.h"
// #include "src/objects/source-text-module.h"
// #include "src/objects/tagged-field.h"

// Placeholder for v8::internal namespace
pub mod internal {
    // Placeholder for v8::internal::compiler namespace
    pub mod compiler {

        use std::mem::size_of;

        // Placeholder types, replace with actual definitions
        #[derive(Debug, Clone, Copy)]
        pub enum BaseTaggedness {
            kUntaggedBase,
            kTaggedBase,
        }

        #[derive(Debug, Clone, Copy)]
        pub enum WriteBarrierKind {
            kNoWriteBarrier,
            kPointerWriteBarrier,
            kFullWriteBarrier,
            kIndirectPointerWriteBarrier,
        }

        #[derive(Debug, Clone)]
        pub struct TypeCache {
            pub kFloat64: Type,
            pub kInt32: Type,
            pub kUint32: Type,
            pub kInt16: Type,
            pub kUint16: Type,
            pub kInt8: Type,
            pub kUint8: Type,
            pub kJSArrayLengthType: Type,
            pub kFixedArrayLengthType: Type,
            pub kWeakFixedArrayLengthType: Type,
            pub kJSTypedArrayLengthType: Type,
            pub kBigUint64: Type,
            pub kStringLengthType: Type,
            pub kPositiveSafeInteger: Type,
            pub kJSArrayIteratorKindType: Type,
            pub kHoleySmi: Type,
            pub kNumberOrHole: Type,
            pub kFixedDoubleArrayLengthType: Type,
        }

        impl TypeCache {
            pub fn get() -> &'static TypeCache {
                // This is a placeholder, implement the actual singleton logic
                Box::leak(Box::new(TypeCache {
                    kFloat64: Type::Number,
                    kInt32: Type::Signed32,
                    kUint32: Type::Unsigned32,
                    kInt16: Type::Signed16,
                    kUint16: Type::Unsigned16,
                    kInt8: Type::Signed8,
                    kUint8: Type::Unsigned8,
                    kJSArrayLengthType: Type::Number,
                    kFixedArrayLengthType: Type::Number,
                    kWeakFixedArrayLengthType: Type::Number,
                    kJSTypedArrayLengthType: Type::Number,
                    kBigUint64: Type::Number,
                    kStringLengthType: Type::Number,
                    kPositiveSafeInteger: Type::Number,
                    kJSArrayIteratorKindType: Type::SignedSmall,
                    kHoleySmi: Type::SignedSmall,
                    kNumberOrHole: Type::Number,
                    kFixedDoubleArrayLengthType: Type::Number,
                }))
            }
        }
        #[derive(Debug, Clone)]
        pub struct Type {
            // Add type information here
            _private: (),
        }

        impl Type {
            pub const Any: Self = Type { _private: () };
            pub const OtherInternal: Self = Type { _private: () };
            pub const NonInternal: Self = Type { _private: () };
            pub const Internal: Self = Type { _private: () };
            pub const ExternalPointer: Self = Type { _private: () };
            pub const Callable: Self = Type { _private: () };
            pub const CallableFunction: Self = Type { _private: () };
            pub const OtherObject: Self = Type { _private: () };
            pub const SignedSmall: Self = Type { _private: () };
            pub const Receiver: Self = Type { _private: () };
            pub const String: Self = Type { _private: () };
            pub const Unsigned32: Self = Type { _private: () };
            pub const Number: Self = Type { _private: () };
            pub const Signed32: Self = Type { _private: () };
            pub const Unsigned16: Self = Type { _private: () };
            pub const Signed16: Self = Type { _private: () };
            pub const Signed8: Self = Type { _private: () };
            pub const Unsigned8: Self = Type { _private: () };
            pub const SignedBigInt64: Self = Type { _private: () };
            pub const UnsignedBigInt64: Self = Type { _private: () };
            pub const SandboxedPointer: Self = Type { _private: () };
            pub const NumberOrHole: Self = Type { _private: () };
            pub const InternalizedString: Self = Type{ _private: ()};
        }

        #[derive(Debug, Clone, Copy)]
        pub enum MachineType {
            IntPtr,
            MapInHeader,
            Float64,
            Int32,
            Uint32,
            Uint64,
            AnyTagged,
            TaggedPointer,
            Pointer,
            IndirectPointer,
            Uint8,
            Uint16,
            Float32,
            Int64,
            SandboxedPointer,
            TaggedSigned,
        }

        #[derive(Debug, Clone)]
        pub struct FieldAccess {
            pub taggedness: BaseTaggedness,
            pub offset: usize,
            pub name: String, // Placeholder for MaybeHandle<Name>
            pub optional_map_ref: OptionalMapRef,
            pub r#type: Type,
            pub machine_type: MachineType,
            pub write_barrier_kind: WriteBarrierKind,
            pub description: &'static str,
            pub const_field_info: ConstFieldInfo, // Add real type
            pub is_immutable: bool,
            pub indirect_pointer_tag: IndirectPointerTag,
            pub is_bounded_size_access: bool,
        }

        impl FieldAccess {
            pub fn new(
                taggedness: BaseTaggedness,
                offset: usize,
                name: String, // Placeholder for MaybeHandle<Name>
                optional_map_ref: OptionalMapRef,
                r#type: Type,
                machine_type: MachineType,
                write_barrier_kind: WriteBarrierKind,
                description: &'static str,
                const_field_info: ConstFieldInfo,
                is_immutable: bool,
                indirect_pointer_tag: IndirectPointerTag,
                is_bounded_size_access: bool,
            ) -> Self {
                FieldAccess {
                    taggedness,
                    offset,
                    name,
                    optional_map_ref,
                    r#type,
                    machine_type,
                    write_barrier_kind,
                    description,
                    const_field_info,
                    is_immutable,
                    indirect_pointer_tag,
                    is_bounded_size_access,
                }
            }
        }

        #[derive(Debug, Clone)]
        pub struct ElementAccess {
            pub taggedness: BaseTaggedness,
            pub header_size: usize,
            pub r#type: Type,
            pub machine_type: MachineType,
            pub write_barrier_kind: WriteBarrierKind,
        }

        impl ElementAccess {
            pub fn new(
                taggedness: BaseTaggedness,
                header_size: usize,
                r#type: Type,
                machine_type: MachineType,
                write_barrier_kind: WriteBarrierKind,
            ) -> Self {
                ElementAccess {
                    taggedness,
                    header_size,
                    r#type,
                    machine_type,
                    write_barrier_kind,
                }
            }
        }

        #[derive(Debug, Clone)]
        pub struct OptionalMapRef {
            // Add actual fields, if any.  Using a unit struct as a placeholder.
            _private: (),
        }

        impl OptionalMapRef {
            pub fn new() -> Self {
                OptionalMapRef { _private: () }
            }
            //Add method implementations here if they exist.
        }

        impl Default for OptionalMapRef {
            fn default() -> Self {
                Self::new()
            }
        }

        #[derive(Debug, Clone, Copy)]
        pub enum ConstFieldInfo {
            None(),
        }

        #[derive(Debug, Clone, Copy)]
        pub enum IndirectPointerTag {
            None,
            kCodeIndirectPointerTag,
            kRegExpDataIndirectPointerTag,
            kExternalObjectValueTag,
            kExternalStringResourceDataTag
        }

        pub const K_IEEE_DOUBLE_MANTISSA_WORD_OFFSET: usize = 0;
        pub const K_IEEE_DOUBLE_EXPONENT_WORD_OFFSET: usize = 4;
        pub const K_HEAP_OBJECT_TAG: usize = 1;
        pub const K_TAGGED_SIZE: usize = 8;

        pub mod access_builder {
            use super::*;

            pub struct AccessBuilder;

            impl AccessBuilder {
                pub fn for_external_int_ptr() -> FieldAccess {
                    FieldAccess::new(
                        BaseTaggedness::kUntaggedBase,
                        0,
                        String::new(),
                        OptionalMapRef::new(),
                        Type::Any,
                        MachineType::IntPtr,
                        WriteBarrierKind::kNoWriteBarrier,
                        "ExternalIntPtr",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::None,
                        false
                    )
                }

                pub fn for_map(write_barrier: WriteBarrierKind) -> FieldAccess {
                    FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        size_of::<usize>(), // Placeholder for HeapObject::kMapOffset
                        String::new(),
                        OptionalMapRef::new(),
                        Type::OtherInternal,
                        MachineType::MapInHeader,
                        write_barrier,
                        "Map",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::None,
                        false
                    )
                }

                pub fn for_heap_number_value() -> FieldAccess {
                    FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        0, // Placeholder for offsetof(HeapNumber, value_)
                        String::new(),
                        OptionalMapRef::new(),
                        TypeCache::get().kFloat64.clone(),
                        MachineType::Float64,
                        WriteBarrierKind::kNoWriteBarrier,
                        "HeapNumberValue",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::None,
                        false
                    )
                }

                pub fn for_heap_int32_value() -> FieldAccess {
                    FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        K_IEEE_DOUBLE_MANTISSA_WORD_OFFSET,
                        String::new(),
                        OptionalMapRef::new(),
                        TypeCache::get().kInt32.clone(),
                        MachineType::Int32,
                        WriteBarrierKind::kNoWriteBarrier,
                        "HeapInt32Value",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::None,
                        false
                    )
                }

                pub fn for_heap_int32_upper_value() -> FieldAccess {
                    FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        K_IEEE_DOUBLE_EXPONENT_WORD_OFFSET,
                        String::new(),
                        OptionalMapRef::new(),
                        TypeCache::get().kInt32.clone(),
                        MachineType::Int32,
                        WriteBarrierKind::kNoWriteBarrier,
                        "HeapInt32ValueUpperValue",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::None,
                        false
                    )
                }

                pub fn for_heap_number_or_oddball_or_hole_value() -> FieldAccess {
                    // STATIC_ASSERT_FIELD_OFFSETS_EQUAL(offsetof(HeapNumber, value_),
                    //                                 offsetof(Oddball, to_number_raw_));
                    // STATIC_ASSERT_FIELD_OFFSETS_EQUAL(offsetof(HeapNumber, value_),
                    //                                 Hole::kRawNumericValueOffset);
                    Self::for_heap_number_value()
                }

                pub fn for_big_int_bitfield() -> FieldAccess {
                    FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        0, // Placeholder for offsetof(BigInt, bitfield_)
                        String::new(),
                        OptionalMapRef::new(),
                        TypeCache::get().kInt32.clone(),
                        MachineType::Uint32,
                        WriteBarrierKind::kNoWriteBarrier,
                        "BigIntBitfield",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::None,
                        false
                    )
                }

                #[cfg(feature = "bigint_needs_padding")]
                pub fn for_big_int_optional_padding() -> FieldAccess {
                    //static_assert(arraysize(BigInt::padding_) == sizeof(uint32_t));
                    FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        0, // Placeholder for offsetof(BigInt, padding_)
                        String::new(),
                        OptionalMapRef::new(),
                        TypeCache::get().kInt32.clone(),
                        MachineType::Uint32,
                        WriteBarrierKind::kNoWriteBarrier,
                        "BigIntOptionalPadding",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::None,
                        false
                    )
                }

                pub fn for_big_int_least_significant_digit64() -> FieldAccess {
                    //DCHECK_EQ(BigInt::SizeFor(1) - BigInt::SizeFor(0), 8);
                    FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        0, // Placeholder for OFFSET_OF_DATA_START(BigInt)
                        String::new(),
                        OptionalMapRef::new(),
                        TypeCache::get().kBigUint64.clone(),
                        MachineType::Uint64,
                        WriteBarrierKind::kNoWriteBarrier,
                        "BigIntLeastSignificantDigit64",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::None,
                        false
                    )
                }

                pub fn for_js_object_properties_or_hash() -> FieldAccess {
                    FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        0, // Placeholder for JSObject::kPropertiesOrHashOffset
                        String::new(),
                        OptionalMapRef::new(),
                        Type::Any,
                        MachineType::AnyTagged,
                        WriteBarrierKind::kFullWriteBarrier,
                        "JSObjectPropertiesOrHash",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::None,
                        false
                    )
                }

                pub fn for_js_object_properties_or_hash_known_pointer() -> FieldAccess {
                    FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        0, // Placeholder for JSObject::kPropertiesOrHashOffset
                        String::new(),
                        OptionalMapRef::new(),
                        Type::Any,
                        MachineType::TaggedPointer,
                        WriteBarrierKind::kPointerWriteBarrier,
                        "JSObjectPropertiesOrHashKnownPointer",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::None,
                        false
                    )
                }

                pub fn for_js_object_elements() -> FieldAccess {
                    FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        0, // Placeholder for JSObject::kElementsOffset
                        String::new(),
                        OptionalMapRef::new(),
                        Type::Internal,
                        MachineType::TaggedPointer,
                        WriteBarrierKind::kPointerWriteBarrier,
                        "JSObjectElements",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::None,
                        false
                    )
                }

                pub fn for_js_object_in_object_property(
                    _map: &MapRef,
                    index: i32,
                    machine_type: MachineType,
                ) -> FieldAccess {
                    let offset = Self::get_in_object_property_offset(_map, index);
                    FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        offset,
                        String::new(),
                        OptionalMapRef::new(),
                        Type::NonInternal,
                        machine_type,
                        WriteBarrierKind::kFullWriteBarrier,
                        "JSObjectInObjectProperty",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::None,
                        false
                    )
                }

                fn get_in_object_property_offset(_map: &MapRef, _index: i32) -> usize {
                    0 // Placeholder for map.GetInObjectPropertyOffset(index);
                }

                pub fn for_js_object_offset(
                    offset: i32,
                    write_barrier_kind: WriteBarrierKind,
                ) -> FieldAccess {
                    FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        offset as usize,
                        String::new(),
                        OptionalMapRef::new(),
                        Type::NonInternal,
                        MachineType::AnyTagged,
                        write_barrier_kind,
                        "JSObjectOffset",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::None,
                        false
                    )
                }

                pub fn for_js_collection_table() -> FieldAccess {
                    FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        0, // Placeholder for JSCollection::kTableOffset
                        String::new(),
                        OptionalMapRef::new(),
                        Type::OtherInternal,
                        MachineType::TaggedPointer,
                        WriteBarrierKind::kPointerWriteBarrier,
                        "JSCollectionTable",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::None,
                        false
                    )
                }

                pub fn for_js_collection_iterator_table() -> FieldAccess {
                    FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        0, // Placeholder for JSCollectionIterator::kTableOffset
                        String::new(),
                        OptionalMapRef::new(),
                        Type::OtherInternal,
                        MachineType::TaggedPointer,
                        WriteBarrierKind::kPointerWriteBarrier,
                        "JSCollectionIteratorTable",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::None,
                        false
                    )
                }

                pub fn for_js_collection_iterator_index() -> FieldAccess {
                    FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        0, // Placeholder for JSCollectionIterator::kIndexOffset
                        String::new(),
                        OptionalMapRef::new(),
                        TypeCache::get().kFixedArrayLengthType.clone(),
                        MachineType::TaggedSigned,
                        WriteBarrierKind::kNoWriteBarrier,
                        "JSCollectionIteratorIndex",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::None,
                        false
                    )
                }

                pub fn for_js_external_object_value() -> FieldAccess {
                    FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        0, // Placeholder for JSExternalObject::kValueOffset
                        String::new(),
                        OptionalMapRef::new(),
                        Type::ExternalPointer,
                        MachineType::Pointer,
                        WriteBarrierKind::kNoWriteBarrier,
                        "JSExternalObjectValue",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::kExternalObjectValueTag,
                        false
                    )
                }

                #[cfg(feature = "v8_enable_sandbox")]
                pub fn for_js_external_object_pointer_handle() -> FieldAccess {
                    FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        0, // Placeholder for JSExternalObject::kValueOffset
                        String::new(),
                        OptionalMapRef::new(),
                        TypeCache::get().kUint32.clone(),
                        MachineType::Uint32,
                        WriteBarrierKind::kNoWriteBarrier,
                        "JSExternalObjectPointerHandle",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::None,
                        false
                    )
                }

                pub fn for_js_function_prototype_or_initial_map() -> FieldAccess {
                    FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        0, // Placeholder for JSFunction::kPrototypeOrInitialMapOffset
                        String::new(),
                        OptionalMapRef::new(),
                        Type::Any,
                        MachineType::TaggedPointer,
                        WriteBarrierKind::kPointerWriteBarrier,
                        "JSFunctionPrototypeOrInitialMap",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::None,
                        false
                    )
                }

                pub fn for_js_function_context() -> FieldAccess {
                    FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        0, // Placeholder for JSFunction::kContextOffset
                        String::new(),
                        OptionalMapRef::new(),
                        Type::Internal,
                        MachineType::TaggedPointer,
                        WriteBarrierKind::kPointerWriteBarrier,
                        "JSFunctionContext",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::None,
                        false
                    )
                }

                pub fn for_js_function_shared_function_info() -> FieldAccess {
                    FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        0, // Placeholder for JSFunction::kSharedFunctionInfoOffset
                        String::new(),
                        OptionalMapRef::new(),
                        Type::OtherInternal,
                        MachineType::TaggedPointer,
                        WriteBarrierKind::kPointerWriteBarrier,
                        "JSFunctionSharedFunctionInfo",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::None,
                        false
                    )
                }

                pub fn for_js_function_feedback_cell() -> FieldAccess {
                    FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        0, // Placeholder for JSFunction::kFeedbackCellOffset
                        String::new(),
                        OptionalMapRef::new(),
                        Type::Internal,
                        MachineType::TaggedPointer,
                        WriteBarrierKind::kPointerWriteBarrier,
                        "JSFunctionFeedbackCell",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::None,
                        false
                    )
                }

                #[cfg(feature = "v8_enable_leap_tiering")]
                pub fn for_js_function_dispatch_handle_no_write_barrier() -> FieldAccess {
                    // We currently don't require write barriers when writing dispatch handles of
                    // JSFunctions because they are loaded from the function's FeedbackCell and
                    // so must already be reachable. If this ever changes, we'll need to
                    // implement write barrier support for dispatch handles in generated code.
                    FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        0, // Placeholder for JSFunction::kDispatchHandleOffset
                        String::new(),
                        OptionalMapRef::new(),
                        TypeCache::get().kInt32.clone(),
                        MachineType::Int32,
                        WriteBarrierKind::kNoWriteBarrier,
                        "JSFunctionDispatchHandle",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::None,
                        false
                    )
                }

                #[cfg(not(feature = "v8_enable_leap_tiering"))]
                pub fn for_js_function_code() -> FieldAccess {
                    #[cfg(feature = "v8_enable_sandbox")]
                    {
                        let mut access = FieldAccess::new(
                            BaseTaggedness::kTaggedBase,
                            0, // Placeholder for JSFunction::kCodeOffset
                            String::new(),
                            OptionalMapRef::new(),
                            Type::OtherInternal,
                            MachineType::IndirectPointer,
                            WriteBarrierKind::kIndirectPointerWriteBarrier,
                            "JSFunctionCode",
                            ConstFieldInfo::None(),
                            false,
                            IndirectPointerTag::kCodeIndirectPointerTag,
                            false
                        );
                        access.indirect_pointer_tag = IndirectPointerTag::kCodeIndirectPointerTag;
                        access
                    }
                    #[cfg(not(feature = "v8_enable_sandbox"))]
                    {
                        FieldAccess::new(
                            BaseTaggedness::kTaggedBase,
                            0, // Placeholder for JSFunction::kCodeOffset
                            String::new(),
                            OptionalMapRef::new(),
                            Type::OtherInternal,
                            MachineType::TaggedPointer,
                            WriteBarrierKind::kPointerWriteBarrier,
                            "JSFunctionCode",
                            ConstFieldInfo::None(),
                            false,
                            IndirectPointerTag::None,
                            false
                        )
                    }
                }

                pub fn for_js_bound_function_bound_target_function() -> FieldAccess {
                    FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        0, // Placeholder for JSBoundFunction::kBoundTargetFunctionOffset
                        String::new(),
                        OptionalMapRef::new(),
                        Type::Callable,
                        MachineType::TaggedPointer,
                        WriteBarrierKind::kPointerWriteBarrier,
                        "JSBoundFunctionBoundTargetFunction",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::None,
                        false
                    )
                }

                pub fn for_js_bound_function_bound_this() -> FieldAccess {
                    FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        0, // Placeholder for JSBoundFunction::kBoundThisOffset
                        String::new(),
                        OptionalMapRef::new(),
                        Type::NonInternal,
                        MachineType::AnyTagged,
                        WriteBarrierKind::kFullWriteBarrier,
                        "JSBoundFunctionBoundThis",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::None,
                        false
                    )
                }

                pub fn for_js_bound_function_bound_arguments() -> FieldAccess {
                    FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        0, // Placeholder for JSBoundFunction::kBoundArgumentsOffset
                        String::new(),
                        OptionalMapRef::new(),
                        Type::Internal,
                        MachineType::TaggedPointer,
                        WriteBarrierKind::kPointerWriteBarrier,
                        "JSBoundFunctionBoundArguments",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::None,
                        false
                    )
                }

                pub fn for_js_generator_object_context() -> FieldAccess {
                    FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        0, // Placeholder for JSGeneratorObject::kContextOffset
                        String::new(),
                        OptionalMapRef::new(),
                        Type::Internal,
                        MachineType::TaggedPointer,
                        WriteBarrierKind::kPointerWriteBarrier,
                        "JSGeneratorObjectContext",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::None,
                        false
                    )
                }

                pub fn for_js_generator_object_function() -> FieldAccess {
                    FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        0, // Placeholder for JSGeneratorObject::kFunctionOffset
                        String::new(),
                        OptionalMapRef::new(),
                        Type::CallableFunction,
                        MachineType::TaggedPointer,
                        WriteBarrierKind::kPointerWriteBarrier,
                        "JSGeneratorObjectFunction",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::None,
                        false
                    )
                }

                pub fn for_js_generator_object_receiver() -> FieldAccess {
                    FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        0, // Placeholder for JSGeneratorObject::kReceiverOffset
                        String::new(),
                        OptionalMapRef::new(),
                        Type::Internal,
                        MachineType::TaggedPointer,
                        WriteBarrierKind::kPointerWriteBarrier,
                        "JSGeneratorObjectReceiver",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::None,
                        false
                    )
                }

                pub fn for_js_generator_object_continuation() -> FieldAccess {
                    FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        0, // Placeholder for JSGeneratorObject::kContinuationOffset
                        String::new(),
                        OptionalMapRef::new(),
                        Type::SignedSmall,
                        MachineType::TaggedSigned,
                        WriteBarrierKind::kNoWriteBarrier,
                        "JSGeneratorObjectContinuation",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::None,
                        false
                    )
                }

                pub fn for_js_generator_object_input_or_debug_pos() -> FieldAccess {
                    FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        0, // Placeholder for JSGeneratorObject::kInputOrDebugPosOffset
                        String::new(),
                        OptionalMapRef::new(),
                        Type::NonInternal,
                        MachineType::AnyTagged,
                        WriteBarrierKind::kFullWriteBarrier,
                        "JSGeneratorObjectInputOrDebugPos",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::None,
                        false
                    )
                }

                pub fn for_js_generator_object_parameters_and_registers() -> FieldAccess {
                    FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        0, // Placeholder for JSGeneratorObject::kParametersAndRegistersOffset
                        String::new(),
                        OptionalMapRef::new(),
                        Type::Internal,
                        MachineType::TaggedPointer,
                        WriteBarrierKind::kPointerWriteBarrier,
                        "JSGeneratorObjectParametersAndRegisters",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::None,
                        false
                    )
                }

                pub fn for_js_generator_object_resume_mode() -> FieldAccess {
                    FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        0, // Placeholder for JSGeneratorObject::kResumeModeOffset
                        String::new(),
                        OptionalMapRef::new(),
                        Type::SignedSmall,
                        MachineType::TaggedSigned,
                        WriteBarrierKind::kNoWriteBarrier,
                        "JSGeneratorObjectResumeMode",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::None,
                        false
                    )
                }

                pub fn for_js_async_function_object_promise() -> FieldAccess {
                    FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        0, // Placeholder for JSAsyncFunctionObject::kPromiseOffset
                        String::new(),
                        OptionalMapRef::new(),
                        Type::OtherObject,
                        MachineType::TaggedPointer,
                        WriteBarrierKind::kPointerWriteBarrier,
                        "JSAsyncFunctionObjectPromise",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::None,
                        false
                    )
                }

                pub fn for_js_async_generator_object_queue() -> FieldAccess {
                    FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        0, // Placeholder for JSAsyncGeneratorObject::kQueueOffset
                        String::new(),
                        OptionalMapRef::new(),
                        Type::NonInternal,
                        MachineType::AnyTagged,
                        WriteBarrierKind::kFullWriteBarrier,
                        "JSAsyncGeneratorObjectQueue",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::None,
                        false
                    )
                }

                pub fn for_js_async_generator_object_is_awaiting() -> FieldAccess {
                    FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        0, // Placeholder for JSAsyncGeneratorObject::kIsAwaitingOffset
                        String::new(),
                        OptionalMapRef::new(),
                        Type::SignedSmall,
                        MachineType::TaggedSigned,
                        WriteBarrierKind::kNoWriteBarrier,
                        "JSAsyncGeneratorObjectIsAwaiting",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::None,
                        false
                    )
                }

                pub fn for_js_array_length(elements_kind: ElementsKind) -> FieldAccess {
                    let type_cache = TypeCache::get();
                    let mut access = FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        0, // Placeholder for JSArray::kLengthOffset
                        String::new(),
                        OptionalMapRef::new(),
                        type_cache.kJSArrayLengthType.clone(),
                        MachineType::AnyTagged,
                        WriteBarrierKind::kFullWriteBarrier,
                        "JSArrayLength",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::None,
                        false
                    );
                    if is_double_elements_kind(elements_kind) {
                        access.r#type = type_cache.kFixedDoubleArrayLengthType.clone();
                        access.machine_type = MachineType::TaggedSigned;
                        access.write_barrier_kind = WriteBarrierKind::kNoWriteBarrier;
                    } else if is_fast_elements_kind(elements_kind) {
                        access.r#type = type_cache.kFixedArrayLengthType.clone();
                        access.machine_type = MachineType::TaggedSigned;
                        access.write_barrier_kind = WriteBarrierKind::kNoWriteBarrier;
                    }
                    access
                }

                pub fn for_js_array_buffer_bit_field() -> FieldAccess {
                    FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        0, // Placeholder for JSArrayBuffer::kBitFieldOffset
                        String::new(),
                        OptionalMapRef::new(),
                        TypeCache::get().kUint8.clone(),
                        MachineType::Uint32,
                        WriteBarrierKind::kNoWriteBarrier,
                        "JSArrayBufferBitField",
                        ConstFieldInfo::None(),
                        false,
                        IndirectPointerTag::None,
                        false
                    )
                }

                pub fn for_js_array_buffer_byte_length() -> FieldAccess {
                    let mut access = FieldAccess::new(
                        BaseTaggedness::kTaggedBase,
                        0, // Placeholder for JSArrayBuffer::kRawByteLengthOffset
                        String::new(),
                        OptionalMapRef::new(),
                        TypeCache::get().kJSTypedArrayLengthType.