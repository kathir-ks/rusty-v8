// src/objects/objects-body-descriptors-inl.rs

//use std::cmp;
//use std::mem;
//use std::ptr;
//use std::rc::Rc;

//use crate::base::logging::*; // Assuming a similar logging system
//use crate::codegen::reloc_info::*; // Assuming similar structures
//use crate::common::globals::*; // Assuming similar globals
//use crate::heap::heap_layout_inl::*; // Assuming similar structures
//use crate::ic::handler_configuration::*; // Assuming similar structures
//use crate::objects::arguments_inl::*; // Assuming similar structures
//use crate::objects::bigint::*; // Assuming similar structures
//use crate::objects::call_site_info_inl::*; // Assuming similar structures
//use crate::objects::call_site_info::*; // Assuming similar structures
//use crate::objects::cell::*; // Assuming similar structures
//use crate::objects::data_handler::*; // Assuming similar structures
//use crate::objects::embedder_data_array_inl::*; // Assuming similar structures
//use crate::objects::fixed_array::*; // Assuming similar structures
//use crate::objects::foreign_inl::*; // Assuming similar structures
//use crate::objects::free_space_inl::*; // Assuming similar structures
//use crate::objects::hash_table::*; // Assuming similar structures
//use crate::objects::heap_number::*; // Assuming similar structures
//use crate::objects::instance_type::*; // Assuming similar structures
//use crate::objects::js_array_buffer::*; // Assuming similar structures
//use crate::objects::js_atomics_synchronization_inl::*; // Assuming similar structures
//use crate::objects::js_collection::*; // Assuming similar structures
//use crate::objects::js_objects::*; // Assuming similar structures
//use crate::objects::js_weak_refs::*; // Assuming similar structures
//use crate::objects::literal_objects::*; // Assuming similar structures
//use crate::objects::megadom_handler_inl::*; // Assuming similar structures
//use crate::objects::objects_body_descriptors::*; // Assuming similar structures
//use crate::objects::ordered_hash_table_inl::*; // Assuming similar structures
//use crate::objects::property_descriptor_object::*; // Assuming similar structures
//use crate::objects::source_text_module::*; // Assuming similar structures
//use crate::objects::swiss_name_dictionary_inl::*; // Assuming similar structures
//use crate::objects::synthetic_module::*; // Assuming similar structures
//use crate::objects::tagged_field::*; // Assuming similar structures
//use crate::objects::template_objects_inl::*; // Assuming similar structures
//use crate::objects::torque_defined_classes_inl::*; // Assuming similar structures
//use crate::objects::transitions::*; // Assuming similar structures
//use crate::objects::turbofan_types_inl::*; // Assuming similar structures
//use crate::objects::turboshaft_types_inl::*; // Assuming similar structures
//
//#[cfg(feature = "V8_ENABLE_WEBASSEMBLY")]
//use crate::wasm::wasm_objects_inl::*; // Assuming similar structures

//mod v8_internal {
//    // Re-export the necessary types and constants
//    pub use crate::objects::objects_body_descriptors::*;
//}

//use v8_internal::*;

pub struct FlexibleBodyDescriptor<const START_OFFSET: usize>;

impl<const START_OFFSET: usize> FlexibleBodyDescriptor<START_OFFSET> {
    pub fn size_of(map: &Map, object: &HeapObject) -> usize {
        object.size_from_map(map)
    }
}

pub struct FlexibleWeakBodyDescriptor<const START_OFFSET: usize>;

impl<const START_OFFSET: usize> FlexibleWeakBodyDescriptor<START_OFFSET> {
    pub fn size_of(map: &Map, object: &HeapObject) -> usize {
        object.size_from_map(map)
    }
}

pub struct BodyDescriptorBase;

impl BodyDescriptorBase {
    // Assuming ObjectVisitor is a trait
    pub fn iterate_js_object_body_impl<OV: ObjectVisitor>(
        map: &Map,
        obj: &HeapObject,
        start_offset: usize,
        end_offset: usize,
        v: &mut OV,
    ) {
        //#ifdef V8_COMPRESS_POINTERS
        //    static_assert(kEmbedderDataSlotSize == 2 * kTaggedSize);
        let header_end_offset = JSObject::get_header_size(map);
        let inobject_fields_start_offset = map.get_in_object_property_offset(0);
        // We are always requested to process header and embedder fields.
        //DCHECK_LE(inobject_fields_start_offset, end_offset);
        assert!(inobject_fields_start_offset <= end_offset);

        // Embedder fields are located between header and inobject properties.
        if header_end_offset < inobject_fields_start_offset {
            // There are embedder fields.
            //DCHECK_EQ(header_end_offset, JSObject::GetEmbedderFieldsStartOffset(map));
            assert_eq!(header_end_offset, JSObject::get_embedder_fields_start_offset(map));
            Self::iterate_pointers(obj, start_offset, header_end_offset, v);

            //for (int offset = header_end_offset; offset < inobject_fields_start_offset; offset += kEmbedderDataSlotSize) {
            let mut offset = header_end_offset;
            while offset < inobject_fields_start_offset {
                //IteratePointer(obj, offset + EmbedderDataSlot::kTaggedPayloadOffset, v);
                Self::iterate_pointer(obj, offset + EmbedderDataSlot::k_TAGGED_PAYLOAD_OFFSET, v);

                //v->VisitExternalPointer(obj, obj->RawExternalPointerField(offset + EmbedderDataSlot::kExternalPointerOffset, kEmbedderDataSlotPayloadTag));
                v.visit_external_pointer(
                    obj,
                    obj.raw_external_pointer_field(
                        offset + EmbedderDataSlot::k_EXTERNAL_POINTER_OFFSET,
                        EmbedderDataSlot::K_EMBEDDER_DATA_SLOT_PAYLOAD_TAG,
                    ),
                );
                offset += EmbedderDataSlot::K_EMBEDDER_DATA_SLOT_SIZE;
            }

            // Proceed processing inobject properties.
            //start_offset = inobject_fields_start_offset;
            Self::iterate_pointers(obj, inobject_fields_start_offset, end_offset, v);
        } else {
            Self::iterate_pointers(obj, start_offset, end_offset, v);
        }
    }

    // static
    pub fn iterate_js_object_body_without_embedder_fields_impl<OV: ObjectVisitor>(
        map: &Map,
        obj: &HeapObject,
        start_offset: usize,
        end_offset: usize,
        v: &mut OV,
    ) {
        // This body iteration assumes that there's no embedder fields.
        //DCHECK_IMPLIES(JSObject::MayHaveEmbedderFields(map),UncheckedCast<JSObject>(obj)->GetEmbedderFieldCount(map) == 0);
        //assert!(!JSObject::may_have_embedder_fields(map) || (unsafe { (obj as *const HeapObject).cast::<JSObject>().as_ref().unwrap().get_embedder_field_count(map) == 0 }));
        Self::iterate_pointers(obj, start_offset, end_offset, v);
    }
    // Assuming ObjectVisitor is a trait
    pub fn iterate_pointers<OV: ObjectVisitor>(
        obj: &HeapObject,
        start_offset: usize,
        end_offset: usize,
        v: &mut OV,
    ) {
        if start_offset == HeapObject::K_MAP_OFFSET {
            v.visit_map_pointer(obj);
            //start_offset += kTaggedSize;
        }
        //v->VisitPointers(obj, obj->RawField(start_offset), obj->RawField(end_offset));
        v.visit_pointers(obj, obj.raw_field(start_offset), obj.raw_field(end_offset));
    }

    // Assuming ObjectVisitor is a trait
    pub fn iterate_pointer<OV: ObjectVisitor>(obj: &HeapObject, offset: usize, v: &mut OV) {
        //DCHECK_NE(offset, HeapObject::kMapOffset);
        assert_ne!(offset, HeapObject::K_MAP_OFFSET);
        //v->VisitPointer(obj, obj->RawField(offset));
        v.visit_pointer(obj, obj.raw_field(offset));
    }

    // Assuming ObjectVisitor is a trait
    pub fn iterate_maybe_weak_pointers<OV: ObjectVisitor>(
        obj: &HeapObject,
        start_offset: usize,
        end_offset: usize,
        v: &mut OV,
    ) {
        //v->VisitPointers(obj, obj->RawMaybeWeakField(start_offset), obj->RawMaybeWeakField(end_offset));
        v.visit_pointers(obj, obj.raw_maybe_weak_field(start_offset), obj.raw_maybe_weak_field(end_offset));
    }

    // Assuming ObjectVisitor is a trait
    pub fn iterate_maybe_weak_pointer<OV: ObjectVisitor>(
        obj: &HeapObject,
        offset: usize,
        v: &mut OV,
    ) {
        //DCHECK_NE(offset, HeapObject::kMapOffset);
        assert_ne!(offset, HeapObject::K_MAP_OFFSET);
        //v->VisitPointer(obj, obj->RawMaybeWeakField(offset));
        v.visit_pointer(obj, obj.raw_maybe_weak_field(offset));
    }

    // Assuming ObjectVisitor is a trait
    pub fn iterate_custom_weak_pointers<OV: ObjectVisitor>(
        obj: &HeapObject,
        start_offset: usize,
        end_offset: usize,
        v: &mut OV,
    ) {
        //v->VisitCustomWeakPointers(obj, obj->RawField(start_offset),obj->RawField(end_offset));
        v.visit_custom_weak_pointers(obj, obj.raw_field(start_offset), obj.raw_field(end_offset));
    }

    // Assuming ObjectVisitor is a trait
    pub fn iterate_ephemeron<OV: ObjectVisitor>(
        obj: &HeapObject,
        index: usize,
        key_offset: usize,
        value_offset: usize,
        v: &mut OV,
    ) {
        //v->VisitEphemeron(obj, index, obj->RawField(key_offset),obj->RawField(value_offset));
        v.visit_ephemeron(obj, index, obj.raw_field(key_offset), obj.raw_field(value_offset));
    }

    // Assuming ObjectVisitor is a trait
    pub fn iterate_custom_weak_pointer<OV: ObjectVisitor>(
        obj: &HeapObject,
        offset: usize,
        v: &mut OV,
    ) {
        //v->VisitCustomWeakPointer(obj, obj->RawField(offset));
        v.visit_custom_weak_pointer(obj, obj.raw_field(offset));
    }

    // Assuming ObjectVisitor is a trait
    pub fn iterate_trusted_pointer<OV: ObjectVisitor>(
        obj: &HeapObject,
        offset: usize,
        v: &mut OV,
        mode: IndirectPointerMode,
        tag: IndirectPointerTag,
    ) {
    //#ifdef V8_ENABLE_SANDBOX
    //  v->VisitIndirectPointer(obj, obj->RawIndirectPointerField(offset, tag), mode);
    //#else
        if mode == IndirectPointerMode::K_STRONG {
            Self::iterate_pointer(obj, offset, v);
        } else {
            Self::iterate_custom_weak_pointer(obj, offset, v);
        }
    //#endif
    }

    // Assuming ObjectVisitor is a trait
    pub fn iterate_code_pointer<OV: ObjectVisitor>(
        obj: &HeapObject,
        offset: usize,
        v: &mut OV,
        mode: IndirectPointerMode,
    ) {
        Self::iterate_trusted_pointer(obj, offset, v, mode, IndirectPointerTag::K_CODE_INDIRECT_POINTER_TAG);
    }

    // Assuming ObjectVisitor is a trait
    pub fn iterate_self_indirect_pointer<OV: ObjectVisitor>(
        obj: &HeapObject,
        tag: IndirectPointerTag,
        v: &mut OV,
    ) {
        //#ifdef V8_ENABLE_SANDBOX
        //v->VisitTrustedPointerTableEntry(obj, obj->RawIndirectPointerField(ExposedTrustedObject::kSelfIndirectPointerOffset, tag));
        //#endif
    }

    // Assuming ObjectVisitor is a trait
    pub fn iterate_protected_pointer<OV: ObjectVisitor>(
        obj: &HeapObject,
        offset: usize,
        v: &mut OV,
    ) {
        //DCHECK(IsTrustedObject(obj));
        //let host = Cast::<TrustedObject>(obj);
        //v->VisitProtectedPointer(host, host->RawProtectedPointerField(offset));
        //This code requires "Cast" and "IsTrustedObject" from the original C++,
        //which are not defined.
        //Skipping the actual implementation for now.
    }

    //#ifdef V8_ENABLE_LEAPTIERING
    //Assuming ObjectVisitor is a trait
    pub fn iterate_js_dispatch_entry<OV: ObjectVisitor>(
        obj: &HeapObject,
        offset: usize,
        v: &mut OV,
    ) {
    //    JSDispatchHandle handle(obj->Relaxed_ReadField<JSDispatchHandle::underlying_type>(offset));
    //    v->VisitJSDispatchTableEntry(obj, handle);
    }
    //#endif  // V8_ENABLE_LEAPTIERING
}

// This is a BodyDescriptor helper for usage within JSAPIObjectWithEmbedderSlots
// and JSSpecialObject. The class hierarchies are separate but
// `kCppHeapWrappableOffset` is the same for both.
pub struct JSAPIObjectWithEmbedderSlotsOrJSSpecialObjectBodyDescriptor;

impl JSAPIObjectWithEmbedderSlotsOrJSSpecialObjectBodyDescriptor {
    // Assuming ObjectVisitor is a trait
    pub fn iterate_js_api_object_with_embedder_slots_header<OV: ObjectVisitor>(
        map: &Map,
        obj: &HeapObject,
        _object_size: usize, // unused
        v: &mut OV,
    ) {
        // Visit JSObject header.
        BodyDescriptorBase::iterate_pointers(
            obj,
            JSObject::K_PROPERTIES_OR_HASH_OFFSET,
            JSObject::K_END_OF_STRONG_FIELDS_OFFSET,
            v,
        );

        // Visit JSAPIObjectWithEmbedderSlots or JSSpecialObject header.
        //static_assert(JSObject::kEndOfStrongFieldsOffset == JSAPIObjectWithEmbedderSlots::kCppHeapWrappableOffset);
        assert_eq!(
            JSObject::K_END_OF_STRONG_FIELDS_OFFSET,
            JSAPIObjectWithEmbedderSlots::K_CPP_HEAP_WRAPPABLE_OFFSET
        );
        //static_assert(JSAPIObjectWithEmbedderSlots::kCppHeapWrappableOffset == JSSpecialObject::kCppHeapWrappableOffset);
        assert_eq!(
            JSAPIObjectWithEmbedderSlots::K_CPP_HEAP_WRAPPABLE_OFFSET,
            JSSpecialObject::K_CPP_HEAP_WRAPPABLE_OFFSET
        );
        //static_assert(JSAPIObjectWithEmbedderSlots::kCppHeapWrappableOffsetEnd + 1 == JSAPIObjectWithEmbedderSlots::kHeaderSize);
        //v->VisitCppHeapPointer(obj, obj->RawCppHeapPointerField(JSAPIObjectWithEmbedderSlots::kCppHeapWrappableOffset));
        v.visit_cpp_heap_pointer(
            obj,
            obj.raw_cpp_heap_pointer_field(JSAPIObjectWithEmbedderSlots::K_CPP_HEAP_WRAPPABLE_OFFSET),
        );
    }

    // Assuming ObjectVisitor is a trait
    pub fn iterate_js_api_object_with_embedder_slots_tail<
        ConcreteType,
        OV: ObjectVisitor,
    >(
        map: &Map,
        obj: &HeapObject,
        object_size: usize,
        v: &mut OV,
    ) {
        // Visit the tail of JSObject with possible embedder fields and in-object
        // properties. Note that embedder fields are processed in the JSObject base
        // class as there's other object hierarchies that contain embedder fields as
        // well.
        BodyDescriptorBase::iterate_js_object_body_impl(
            map,
            obj,
            ConcreteType::K_HEADER_SIZE,
            object_size,
            v,
        );
    }

    // Assuming ObjectVisitor is a trait
    pub fn iterate_js_api_object_without_embedder_slots_tail<
        ConcreteType,
        OV: ObjectVisitor,
    >(
        map: &Map,
        obj: &HeapObject,
        object_size: usize,
        v: &mut OV,
    ) {
        BodyDescriptorBase::iterate_js_object_body_without_embedder_fields_impl(
            map,
            obj,
            ConcreteType::K_HEADER_SIZE,
            object_size,
            v,
        );
    }

    pub const K_HEADER_SIZE: usize = JSSpecialObject::K_HEADER_SIZE;

    //static_assert(JSAPIObjectWithEmbedderSlots::kHeaderSize == JSSpecialObject::kHeaderSize);
    //static_assert(Internals::kJSAPIObjectWithEmbedderSlotsHeaderSize == JSSpecialObject::kHeaderSize);
}

pub struct JSAPIObjectWithEmbedderSlots;

impl JSAPIObjectWithEmbedderSlots {
    pub struct BodyDescriptor;

    impl BodyDescriptor {
        // Assuming ObjectVisitor is a trait
        pub fn iterate_body<OV: ObjectVisitor>(
            map: &Map,
            obj: &HeapObject,
            object_size: usize,
            v: &mut OV,
        ) {
            JSAPIObjectWithEmbedderSlotsOrJSSpecialObjectBodyDescriptor::iterate_js_api_object_with_embedder_slots_header(
                map, obj, object_size, v,
            );
            JSAPIObjectWithEmbedderSlotsOrJSSpecialObjectBodyDescriptor::iterate_js_api_object_with_embedder_slots_tail::<
                JSAPIObjectWithEmbedderSlotsOrJSSpecialObjectBodyDescriptor,
                OV,
            >(map, obj, object_size, v);
        }

        pub fn size_of(map: &Map, object: &HeapObject) -> usize {
            map.instance_size()
        }
    }

    const K_CPP_HEAP_WRAPPABLE_OFFSET: usize = 0; // Placeholder
}

pub struct HeapNumber;

impl HeapNumber {
    pub struct BodyDescriptor;

    impl BodyDescriptor {
        pub const fn size_of(_map: &Map, _object: &HeapObject) -> usize {
            std::mem::size_of::<HeapNumber>()
        }
    }
}

pub struct FreeSpaceFillerBodyDescriptor;

impl FreeSpaceFillerBodyDescriptor {
    pub fn size_of(map: &Map, raw_object: &HeapObject) -> usize {
        map.instance_size()
    }
}

pub struct FreeSpace;

impl FreeSpace {
    pub struct BodyDescriptor;

    impl BodyDescriptor {
        pub fn size_of(map: &Map, raw_object: &HeapObject) -> usize {
            unsafe { (raw_object as *const HeapObject).cast::<FreeSpace>().as_ref().unwrap().size() }
        }
    }

    unsafe fn size(&self) -> usize { 0 }
}

pub struct JSObject;

impl JSObject {
    pub struct BodyDescriptor;

    impl BodyDescriptor {
        pub const K_START_OFFSET: usize = JSReceiver::K_PROPERTIES_OR_HASH_OFFSET;

        // Assuming ObjectVisitor is a trait
        pub fn iterate_body<OV: ObjectVisitor>(
            map: &Map,
            obj: &HeapObject,
            object_size: usize,
            v: &mut OV,
        ) {
            BodyDescriptorBase::iterate_js_object_body_impl(
                map,
                obj,
                Self::K_START_OFFSET,
                object_size,
                v,
            );
        }

        pub fn size_of(map: &Map, object: &HeapObject) -> usize {
            map.instance_size()
        }
    }

    pub struct FastBodyDescriptor;

    impl FastBodyDescriptor {
        pub const K_START_OFFSET: usize = JSReceiver::K_PROPERTIES_OR_HASH_OFFSET;

        // Assuming ObjectVisitor is a trait
        pub fn iterate_body<OV: ObjectVisitor>(
            map: &Map,
            obj: &HeapObject,
            object_size: usize,
            v: &mut OV,
        ) {
            BodyDescriptorBase::iterate_pointers(
                obj,
                Self::K_START_OFFSET,
                object_size,
                v,
            );
        }

        pub fn size_of(map: &Map, object: &HeapObject) -> usize {
            map.instance_size()
        }
    }

    fn get_header_size(_map: &Map) -> usize { 0 }
    fn get_embedder_fields_start_offset(_map: &Map) -> usize { 0 }
    fn may_have_embedder_fields(_map: &Map) -> bool { false }
}

pub struct JSDate;

impl JSDate {
    pub struct BodyDescriptor;

    impl BodyDescriptor {
        // Assuming ObjectVisitor is a trait
        pub fn iterate_body<OV: ObjectVisitor>(
            map: &Map,
            obj: &HeapObject,
            object_size: usize,
            v: &mut OV,
        ) {
            BodyDescriptorBase::iterate_pointers(
                obj,
                JSObject::BodyDescriptor::K_START_OFFSET,
                Self::K_VALUE_OFFSET,
                v,
            );
            BodyDescriptorBase::iterate_js_object_body_impl(
                map,
                obj,
                Self::K_START_OF_STRONG_FIELDS_OFFSET,
                object_size,
                v,
            );
        }

        pub fn size_of(map: &Map, object: &HeapObject) -> usize {
            map.instance_size()
        }
    }

    const K_VALUE_OFFSET: usize = 0;
    const K_START_OF_STRONG_FIELDS_OFFSET: usize = 0;
}

pub struct JSRegExp;

impl JSRegExp {
    pub struct BodyDescriptor;

    impl BodyDescriptor {
        pub const K_START_OFFSET: usize = JSReceiver::K_PROPERTIES_OR_HASH_OFFSET;

        // Assuming ObjectVisitor is a trait
        pub fn iterate_body<OV: ObjectVisitor>(
            map: &Map,
            obj: &HeapObject,
            object_size: usize,
            v: &mut OV,
        ) {
            BodyDescriptorBase::iterate_pointers(obj, JSReceiver::K_PROPERTIES_OR_HASH_OFFSET, JSObject::get_header_size(map), v);
            BodyDescriptorBase::iterate_trusted_pointer(obj, Self::K_DATA_OFFSET, v, IndirectPointerMode::K_STRONG, IndirectPointerTag::K_REG_EXP_DATA_INDIRECT_POINTER_TAG);
            BodyDescriptorBase::iterate_pointer(obj, Self::K_SOURCE_OFFSET, v);
            BodyDescriptorBase::iterate_js_object_body_impl(map, obj, JSObject::get_header_size(map), object_size, v);
        }

        pub fn size_of(map: &Map, object: &HeapObject) -> usize {
            map.instance_size()
        }
    }

    const K_DATA_OFFSET: usize = 0;
    const K_SOURCE_OFFSET: usize = 0;
}

pub struct RegExpData;

impl RegExpData {
    pub struct BodyDescriptor;

    impl BodyDescriptor {
        // Assuming ObjectVisitor is a trait
        pub fn iterate_body<OV: ObjectVisitor>(
            map: &Map,
            obj: &HeapObject,
            object_size: usize,
            v: &mut OV,
        ) {
            // If new pointers are added to RegExpData, make sure to also add them to
            // the subclasses descriptors (AtomRegExpData and IrRegExpData).
            // We don't directly call the base class IterateBody, as in the future
            // the subclasses will have a different indirect pointer tag from the base
            // class (once inheritance hierarchies are supported for indirect pointer
            // tags).
            BodyDescriptorBase::iterate_self_indirect_pointer(obj, IndirectPointerTag::K_REG_EXP_DATA_INDIRECT_POINTER_TAG, v);
            BodyDescriptorBase::iterate_pointer(obj, Self::K_SOURCE_OFFSET, v);
            BodyDescriptorBase::iterate_pointer(obj, Self::K_WRAPPER_OFFSET, v);
        }

        pub fn size_of(_map: &Map, _obj: &HeapObject) -> usize {
            Self::K_SIZE
        }
    }

    const K_SIZE: usize = 0;
    const K_SOURCE_OFFSET: usize = 0;
    const K_WRAPPER_OFFSET: usize = 0;
}

pub struct AtomRegExpData;

impl AtomRegExpData {
    pub struct BodyDescriptor;

    impl BodyDescriptor {
        // Assuming ObjectVisitor is a trait
        pub fn iterate_body<OV: ObjectVisitor>(
            map: &Map,
            obj: &HeapObject,
            object_size: usize,
            v: &mut OV,
        ) {
            BodyDescriptorBase::iterate_self_indirect_pointer(obj, IndirectPointerTag::K_REG_EXP_DATA_INDIRECT_POINTER_TAG, v);

            BodyDescriptorBase::iterate_pointer(obj, Self::K_SOURCE_OFFSET, v);
            BodyDescriptorBase::iterate_pointer(obj, Self::K_WRAPPER_OFFSET, v);

            BodyDescriptorBase::iterate_pointer(obj, Self::K_PATTERN_OFFSET, v);
        }

        pub fn size_of(_map: &Map, _obj: &HeapObject) -> usize {
            Self::K_SIZE
        }
    }

    const K_SIZE: usize = 0;
    const K_SOURCE_OFFSET: usize = 0;
    const K_WRAPPER_OFFSET: usize = 0;
    const K_PATTERN_OFFSET: usize = 0;
}

pub struct IrRegExpData;

impl IrRegExpData {
    pub struct BodyDescriptor;

    impl BodyDescriptor {
        // Assuming ObjectVisitor is a trait
        pub fn iterate_body<OV: ObjectVisitor>(
            map: &Map,
            obj: &HeapObject,
            object_size: usize,
            v: &mut OV,
        ) {
            BodyDescriptorBase::iterate_self_indirect_pointer(obj, IndirectPointerTag::K_REG_EXP_DATA_INDIRECT_POINTER_TAG, v);

            BodyDescriptorBase::iterate_pointer(obj, Self::K_SOURCE_OFFSET, v);
            BodyDescriptorBase::iterate_pointer(obj, Self::K_WRAPPER_OFFSET, v);

            BodyDescriptorBase::iterate_protected_pointer(obj, Self::K_LATIN1_BYTECODE_OFFSET, v);
            BodyDescriptorBase::iterate_protected_pointer(obj, Self::K_UC16_BYTECODE_OFFSET, v);
            BodyDescriptorBase::iterate_code_pointer(obj, Self::K_LATIN1_CODE_OFFSET, v, IndirectPointerMode::K_STRONG);
            BodyDescriptorBase::iterate_code_pointer(obj, Self::K_UC16_CODE_OFFSET, v, IndirectPointerMode::K_STRONG);
            BodyDescriptorBase::iterate_pointer(obj, Self::K_CAPTURE_NAME_MAP_OFFSET, v);
        }

        pub fn size_of(_map: &Map, _obj: &HeapObject) -> usize {
            Self::K_SIZE
        }
    }

    const K_SIZE: usize = 0;
    const K_SOURCE_OFFSET: usize = 0;
    const K_WRAPPER_OFFSET: usize = 0;
    const K_LATIN1_BYTECODE_OFFSET: usize = 0;
    const K_UC16_BYTECODE_OFFSET: usize = 0;
    const K_LATIN1_CODE_OFFSET: usize = 0;
    const K_UC16_CODE_OFFSET: usize = 0;
    const K_CAPTURE_NAME_MAP_OFFSET: usize = 0;
}

pub struct RegExpDataWrapper;

impl RegExpDataWrapper {
    pub struct BodyDescriptor;

    impl BodyDescriptor {
        // Assuming ObjectVisitor is a trait
        pub fn iterate_body<OV: ObjectVisitor>(
            map: &Map,
            obj: &HeapObject,
            object_size: usize,
            v: &mut OV,
        ) {
            BodyDescriptorBase::iterate_trusted_pointer(obj, Self::K_DATA_OFFSET, v, IndirectPointerMode::K_STRONG, IndirectPointerTag::K_REG_EXP_DATA_INDIRECT_POINTER_TAG);
        }

        pub fn size_of(_map: &Map, _obj: &HeapObject) -> usize {
            Self::K_SIZE
        }
    }

    const K_SIZE: usize = 0;
    const K_DATA_OFFSET: usize = 0;
}

pub struct WeakCell;

impl WeakCell {
    pub struct BodyDescriptor;

    impl BodyDescriptor {
        // Assuming ObjectVisitor is a trait
        pub fn iterate_body<OV: ObjectVisitor>(
            map: &Map,
            obj: &HeapObject,
            object_size: usize,
            v: &mut OV,
        ) {
            BodyDescriptorBase::iterate_pointers(obj, HeapObject::K_HEADER_SIZE, Self::K_TARGET_OFFSET, v);
            BodyDescriptorBase::iterate_custom_weak_pointer(obj, Self::K_TARGET_OFFSET, v);
            BodyDescriptorBase::iterate_custom_weak_pointer(obj, Self::K_UNREGISTER_TOKEN_OFFSET, v);
            BodyDescriptorBase::iterate_pointers(obj, Self::K_UNREGISTER_TOKEN_OFFSET + std::mem::size_of::<usize>(), object_size, v);
        }

        pub fn size_of(map: &Map, object: &HeapObject) -> usize {
            map.instance_size()
        }
    }

    const K_TARGET_OFFSET: usize = 0;
    const K_UNREGISTER_TOKEN_OFFSET: usize = 0;
}

pub struct JSWeakRef;

impl JSWeakRef {
    pub struct BodyDescriptor;

    impl BodyDescriptor {
        // Assuming ObjectVisitor is a trait
        pub fn iterate_body<OV: ObjectVisitor>(
            map: &Map,
            obj: &HeapObject,
            object_size: usize,
            v: &mut OV,
        ) {
            BodyDescriptorBase::iterate_pointers(obj, JSReceiver::K_PROPERTIES_OR_HASH_OFFSET, Self::K_TARGET_OFFSET, v);
            BodyDescriptorBase::iterate_custom_weak_pointer(obj, Self::K_TARGET_OFFSET, v);
            BodyDescriptorBase::iterate_js_object_body_impl(map, obj, Self::K_TARGET_OFFSET + std::mem::size_of::<usize>(), object_size, v);
        }

        pub fn size_of(map: &Map, object: &HeapObject) -> usize {
            map.instance_size()
        }
    }

    const K_TARGET_OFFSET: usize = 0;
}

pub struct JSFinalizationRegistry;

impl JSFinalizationRegistry {
    pub struct BodyDescriptor;

    impl BodyDescriptor {
        // Assuming ObjectVisitor is a trait
        pub fn iterate_body<OV: ObjectVisitor>(
            map: &Map,
            obj: &HeapObject,
            object_size: usize,
            v: &mut OV,
        ) {
            BodyDescriptorBase::iterate_pointers(
                obj,
                JSObject::BodyDescriptor::K_START_OFFSET,
                Self::K_NEXT_DIRTY_OFFSET,
                v,
            );
            BodyDescriptorBase::iterate_custom_weak_pointer(obj, Self::K_NEXT_DIRTY_OFFSET, v);
            BodyDescriptorBase::iterate_js_object_body_impl(map, obj, Self::K_NEXT_DIRTY_OFFSET + std::mem::size_of::<usize>(), object_size, v);
        }

        pub fn size_of(map: &Map, object: &HeapObject) -> usize {
            map.instance_size()
        }
    }

    const K_NEXT_DIRTY_OFFSET: usize = 0;
}

pub struct AllocationSite;

impl AllocationSite {
    pub struct BodyDescriptor;

    impl BodyDescriptor {
        //static_assert(AllocationSite::kCommonPointerFieldEndOffset == AllocationSite::kPretenureDataOffset);
        //static_assert(AllocationSite::kPretenureDataOffset + kInt32Size == AllocationSite::kPretenureCreateCountOffset);
        //static_assert(AllocationSite::kPretenureCreateCountOffset + kInt32Size == AllocationSite::kWeakNextOffset);

        // Assuming ObjectVisitor is a trait
        pub fn iterate_body<OV: ObjectVisitor>(
            map: &Map,
            obj: &HeapObject,
            object_size: usize,
            v: &mut OV,
        ) {
            // Iterate over all the common pointer fields
            BodyDescriptorBase::iterate_pointers(
                obj,
                Self::K_START_OFFSET,
                Self::K_COMMON_POINTER_FIELD_END_OFFSET,
                v,
            );
            // Skip PretenureDataOffset and PretenureCreateCount which are Int32 fields.
            // Visit weak_next only if it has weak_next field.
            if object_size == Self::K_SIZE_WITH_WEAK_NEXT {
                BodyDescriptorBase::iterate_custom_weak_pointers(
                    obj,
                    Self::K_WEAK_NEXT_OFFSET,
                    Self::K_SIZE_WITH_WEAK_NEXT,
                    v,
                );