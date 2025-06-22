// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/objects/js-struct.h (Partial - Interface definitions only as needed)
mod js_struct {
    use crate::objects::*;
    use crate::base::*;
    use crate::heap::*;
    use crate::isolate::*;
    use crate::objects::descriptor::*;
    use crate::objects::property_descriptor::*;
    use crate::objects::number_dictionary::*;
    use crate::objects::name::*;
    use crate::objects::map::*;
    use crate::objects::lookup::*;
    use crate::objects::object::*;

    use std::collections::HashSet;
    use std::sync::Mutex;

    pub struct AlwaysSharedSpaceJSObject {}

    impl AlwaysSharedSpaceJSObject {
        pub fn prepare_map_no_enumerable_properties_no_descriptors(map: &mut Map);
        pub fn prepare_map_no_enumerable_properties(
            isolate: &mut Isolate,
            map: &mut Map,
            descriptors: &DescriptorArray,
        );
        pub fn prepare_map_with_enumerable_properties(
            isolate: &mut Isolate,
            map: &mut Map,
            descriptors: &DescriptorArray,
            enum_length: i32,
        );
        pub fn define_own_property(
            isolate: &mut Isolate,
            shared_obj: &mut AlwaysSharedSpaceJSObject,
            key: &mut Object,
            desc: &mut PropertyDescriptor,
            should_throw: Option<ShouldThrow>,
        ) -> Result<bool, Error>;
        pub fn has_instance(
            isolate: &mut Isolate,
            constructor: &mut JSFunction,
            object: &mut Object,
        ) -> Result<bool, Error>;
    }

    pub struct JSSharedStruct {}

    impl JSSharedStruct {
        pub fn create_instance_map(
            isolate: &mut Isolate,
            field_names: &[&mut Name],
            element_names: &HashSet<u32>,
            maybe_registry_key: Option<&mut String>,
        ) -> Map;
        pub fn get_registry_key(isolate: &mut Isolate, instance_map: &Map) -> Result<String, Error>;
        pub fn is_registry_key_descriptor(
            isolate: &mut Isolate,
            instance_map: &Map,
            i: InternalIndex,
        ) -> bool;
        pub fn get_elements_template(
            isolate: &mut Isolate,
            instance_map: &Map,
        ) -> Result<NumberDictionary, Error>;
        pub fn is_elements_template_descriptor(
            isolate: &mut Isolate,
            instance_map: &Map,
            i: InternalIndex,
        ) -> bool;
    }
    
    pub struct SharedStructTypeRegistry {
        data_: Box<SharedStructTypeRegistryData>,
        data_mutex_: Mutex<()>,
    }

    impl SharedStructTypeRegistry {
        pub fn new() -> Self;
        pub fn register_no_throw(
            isolate: &mut Isolate,
            key: &mut String,
            field_names: &[&mut Name],
            element_names: &HashSet<u32>,
        ) -> Result<Map, Error>;
        pub fn register(
            isolate: &mut Isolate,
            key: &mut String,
            field_names: &[&mut Name],
            element_names: &HashSet<u32>,
        ) -> Result<Map, Error>;
        pub fn iterate_elements(isolate: &mut Isolate, visitor: &mut RootVisitor);
        pub fn notify_elements_removed(count: i32);
        fn ensure_capacity(&mut self, cage_base: PtrComprCageBase, additional_elements: i32);
    }

    struct SharedStructTypeRegistryData {} // Placeholder.  Implementation depends on OffHeapHashTableBase
}

// src/objects/js-struct.cc
use v8_go::base::*;
use v8_go::heap::*;
use v8_go::isolate::*;
use v8_go::objects::descriptor::*;
use v8_go::objects::js_struct::*;
use v8_go::objects::lookup::*;
use v8_go::objects::map::*;
use v8_go::objects::name::*;
use v8_go::objects::object::*;
use v8_go::objects::property_descriptor::*;
use v8_go::objects::number_dictionary::*;

use std::cmp;
use std::collections::HashSet;
use std::sync::Mutex;

// C++ anonymous namespace translated to Rust module
mod internal {
    use super::*;
    use v8_go::objects::map::Map;
    use v8_go::objects::symbol::Symbol;

    fn prepare_map_common(map: &mut Map) {
        debug_assert!(is_always_shared_space_js_object_map(map));
        // Shared objects have fixed layout ahead of time, so there's no slack.
        map.set_in_object_unused_property_fields(0);
        // Shared objects are not extensible and have a null prototype.
        map.set_is_extensible(false);
        // Shared space objects are not optimizable as prototypes because it is
        // not threadsafe.
        map.set_prototype_validity_cell(Map::kPrototypeChainValidSmi, kRelaxedStore, SkipWriteBarrier);
    }
    
    const K_SPECIAL_SLOTS: i32 = 2; // const
    
    fn get_special_slot_index(instance_map: &Map, special_slot_name: &Symbol) -> InternalIndex {
        debug_assert!(is_js_shared_struct_map(instance_map));
        debug_assert!(is_private_symbol(special_slot_name));
        let descriptors = instance_map.instance_descriptors();
        // Special slots are optional and start at descriptor number 0.
        let end = cmp::min(
            descriptors.number_of_all_descriptors() as i32,
            K_SPECIAL_SLOTS,
        );
        for i in 0..end {
            let idx = InternalIndex(i as usize);
            if descriptors.get_key(idx) == *special_slot_name {
                debug_assert_eq!(
                    PropertyLocation::kDescriptor,
                    descriptors.get_details(idx).location()
                );
                return idx;
            }
        }
        InternalIndex::not_found()
    }
    
    fn get_special_slot_value<T: ObjectTrait>(
        isolate: &mut Isolate,
        instance_map: &Map,
        special_slot_name: &Symbol,
    ) -> Result<T, Error> {
      
        let entry = get_special_slot_index(instance_map, special_slot_name);
        if entry.is_found() {
            // TODO: Implement Cast<T>
            if *special_slot_name
                == ReadOnlyRoots::shared_struct_map_registry_key_symbol(&isolate)
            {
                debug_assert_eq!(entry.as_int(), 0);
            }
          
            //  result =
            //    handle(Cast<T>(instance_map->instance_descriptors()->GetStrongValue(
            //               isolate, entry)),
            //         isolate);
            
            
            let descriptor_array = instance_map.instance_descriptors();
            let value = descriptor_array.get_strong_value(isolate, entry);
          
            
            //TODO : need to implement cast properly
            // let result = unsafe {value.assume_init() as T};
            // Ok(result)
            
            todo!("Implement properly with valid Casting to handle<T> from Tagged<Object>")
            
        } else {
            todo!("Implement MaybeHandle<T> as Result<T, Error> for the negative result")
        }
    }
}

impl AlwaysSharedSpaceJSObject {
    pub fn prepare_map_no_enumerable_properties_no_descriptors(map: &mut Map) {
        internal::prepare_map_common(map);
        map.set_enum_length(0);
    }

    pub fn prepare_map_no_enumerable_properties(
        isolate: &mut Isolate,
        map: &mut Map,
        descriptors: &DescriptorArray,
    ) {
        internal::prepare_map_common(map);
        map.initialize_descriptors(isolate, descriptors);
        debug_assert_eq!(0, map.number_of_enumerable_properties());
        map.set_enum_length(0);
    }

    pub fn prepare_map_with_enumerable_properties(
        isolate: &mut Isolate,
        map: &mut Map,
        descriptors: &DescriptorArray,
        enum_length: i32,
    ) {
        internal::prepare_map_common(map);
        // Shared objects with enumerable own properties need to pre-create the enum
        // cache, as creating it lazily is racy.
        map.initialize_descriptors(isolate, descriptors);
        FastKeyAccumulator::initialize_fast_property_enum_cache(
            isolate,
            map,
            enum_length,
            AllocationType::kSharedOld,
        );
        debug_assert_eq!(enum_length, map.enum_length());
    }

    pub fn define_own_property(
        isolate: &mut Isolate,
        shared_obj: &mut AlwaysSharedSpaceJSObject,
        key: &mut Object,
        desc: &mut PropertyDescriptor,
        should_throw: Option<ShouldThrow>,
    ) -> Result<bool, Error> {
        // Shared objects are designed to have fixed layout, i.e. their maps are
        // effectively immutable. They are constructed seal, but the semantics of
        // ordinary ECMAScript objects allow writable properties to be upgraded to
        // non-writable properties. This upgrade violates the fixed layout invariant
        // and is disallowed.
        
        //TODO: Add Name and Number checks
        //debug_assert!(IsName(*key) || IsNumber(*key));  // |key| is a PropertyKey.
        let lookup_key = PropertyKey::new(isolate, key);
        let mut it = LookupIterator::new(isolate, shared_obj, lookup_key, LookupIterator::OWN);
        let mut current = PropertyDescriptor::default();
        let get_own_property_result = get_own_property_descriptor(&mut it, &mut current);

        match get_own_property_result {
            Ok(false) => return Err(Error::Generic("GetOwnPropertyDescriptor failed".to_string())),
            Err(e) => return Err(e),
            _ => {}
        }
      
        // The only redefinition allowed is to set the value if all attributes match.
        if !it.is_found()
            || PropertyDescriptor::is_data_descriptor(desc)
                != PropertyDescriptor::is_data_descriptor(&current)
            || desc.to_attributes() != current.to_attributes()
        {
            debug_assert!(!shared_obj.map().is_extensible());
            // TODO: Implement Error handling
            //RETURN_FAILURE(isolate, GetShouldThrow(isolate, should_throw),
            //             NewTypeError(MessageTemplate::kDefineDisallowedFixedLayout,
            //                         it.GetName()));
            todo!("Implement NewTypeError for kDefineDisallowedFixedLayout");
            //return Err(Error::Generic("DefineDisallowedFixedLayout".to_string()));
        }
        debug_assert!(it.property_attributes() == desc.to_attributes());
        if desc.has_value() {
            return Object::set_data_property(&mut it, desc.value());
        }
        Ok(true)
    }

    pub fn has_instance(
        isolate: &mut Isolate,
        constructor: &mut JSFunction,
        object: &mut Object,
    ) -> Result<bool, Error> {
        if !constructor.has_prototype_slot() || !constructor.has_initial_map() || !object.is_js_receiver()
        {
            return Ok(false);
        }
        let constructor_map = constructor.initial_map();
        let mut iter = PrototypeIterator::new(isolate, object.to_js_receiver().unwrap(), kStartAtReceiver);
        
        loop {
            let current_map = iter.get_current().unwrap().map();

            if current_map == constructor_map {
                return Ok(true);
            }
            if !iter.advance_following_proxies(){
                todo!("Implement Maybe<bool> as Result<bool, Error> for the negative result")
            }
            if iter.is_at_end() {
                return Ok(false);
            }
        }
    }
}

impl JSSharedStruct {
    pub fn create_instance_map(
        isolate: &mut Isolate,
        field_names: &[&mut Name],
        element_names: &HashSet<u32>,
        maybe_registry_key: Option<&mut String>,
    ) -> Map {
        let factory = isolate.factory();

        let mut num_fields = 0;
        let mut num_elements = 0;

        let mut num_descriptors = field_names.len() as i32;
        // If there are elements, an template NumberDictionary is created and stored
        // as a data constant on a descriptor.
        if !element_names.is_empty() {
            num_descriptors += 1;
        }
        // If this is a registered map, the key is stored as a data constant on a
        // descriptor because the registry stores the maps weakly. Storing the key in
        // the map simplifies the weakness handling in the GC.
        if maybe_registry_key.is_some() {
            num_descriptors += 1;
        }

        // Create the DescriptorArray if there are fields or elements.
        let mut descriptors: Option<DescriptorArray> = None;
        if num_descriptors != 0 {
            descriptors = Some(factory.new_descriptor_array(num_descriptors as usize, 0, AllocationType::kSharedOld));

            let mut special_slots = 0;

            // Store the registry key if the map is registered. This must be the first
            // slot if present. The registry depends on this for rehashing.
            if let Some(registry_key) = maybe_registry_key {
                let d = Descriptor::data_constant(
                    factory.shared_struct_map_registry_key_symbol(),
                    registry_key,
                    ALL_ATTRIBUTES_MASK,
                );
               // descriptors.as_mut().unwrap().set(InternalIndex(special_slots), &d);
                descriptors.as_mut().unwrap().set(InternalIndex(special_slots), &d);
                special_slots += 1;
            }

            // Elements in shared structs are only supported as a dictionary. Create the
            // template NumberDictionary if needed.
            if !element_names.is_empty() {
                let mut elements_template: Option<NumberDictionary> = None;
                num_elements = element_names.len() as i32;
                elements_template = Some(NumberDictionary::new(isolate, num_elements as usize, AllocationType::kSharedOld));
                let elements_template_mut = elements_template.as_mut().unwrap();

                for index in element_names {
                    let details = PropertyDetails::new(
                        PropertyKind::kData,
                        SEALED,
                        PropertyConstness::kMutable,
                        0,
                    );
                    NumberDictionary::unchecked_add::<Isolate, &mut NumberDictionary, AllocationType::kSharedOld>(
                        isolate,
                        elements_template_mut,
                        *index,
                        factory.undefined_value(),
                        details,
                    );
                }
                elements_template_mut.set_initial_number_of_elements(num_elements as usize);
                debug_assert!(HeapLayout::in_any_shared_space(elements_template_mut));

                let d = Descriptor::data_constant(
                    factory.shared_struct_map_elements_template_symbol(),
                    elements_template_mut,
                    ALL_ATTRIBUTES_MASK,
                );
                descriptors.as_mut().unwrap().set(InternalIndex(special_slots), &d);
                special_slots += 1;
            }

            debug_assert!(special_slots <= internal::K_SPECIAL_SLOTS);

            for field_name in field_names {
                // Shared structs' fields need to be aligned, so make it all tagged.
                let details = PropertyDetails::new(
                    PropertyKind::kData,
                    SEALED,
                    PropertyLocation::kField,
                    PropertyConstness::kMutable,
                    Representation::Tagged,
                    num_fields,
                );

                let index = InternalIndex(special_slots + num_fields as usize);
                let field_name_local = field_name; // Avoid moving field_name
                
                let descriptors_unwrapped = descriptors.as_mut().unwrap(); // Unwrap here

                descriptors_unwrapped.set(index, *field_name_local, FieldType::Any, details);
                num_fields += 1;
            }

            descriptors.as_mut().unwrap().sort();
        }

        // Calculate the size for instances and create the map.
        let mut instance_size = 0;
        let mut in_object_properties = 0;
        JSFunction::calculate_instance_size_helper(
            JSType::JS_SHARED_STRUCT_TYPE,
            false,
            0,
            num_fields,
            &mut instance_size,
            &mut in_object_properties,
        );
        let mut instance_map = factory.new_contextless_map(
            JSType::JS_SHARED_STRUCT_TYPE,
            instance_size,
            ElementsKind::DICTIONARY_ELEMENTS,
            in_object_properties,
            AllocationType::kSharedMap,
        );

        // Prepare the enum cache if necessary.
        if num_descriptors == 0 {
            debug_assert_eq!(0, num_fields);
            // No properties at all.
            AlwaysSharedSpaceJSObject::prepare_map_no_enumerable_properties_no_descriptors(&mut instance_map);
        } else if num_fields == 0 {
            // Have descriptors, but no enumerable fields.
            AlwaysSharedSpaceJSObject::prepare_map_no_enumerable_properties(
                isolate,
                &mut instance_map,
                descriptors.as_ref().unwrap(),
            );
        } else {
            // Have enumerable fields.
            AlwaysSharedSpaceJSObject::prepare_map_with_enumerable_properties(
                isolate,
                &mut instance_map,
                descriptors.as_ref().unwrap(),
                num_fields,
            );
        }

        // Structs have fixed layout ahead of time, so there's no slack.
        let out_of_object_properties = num_fields - in_object_properties;
        if out_of_object_properties != 0 {
            instance_map.set_out_of_object_unused_property_fields(0);
        }

        instance_map
    }

    pub fn get_registry_key(isolate: &mut Isolate, instance_map: &Map) -> Result<String, Error> {
        // internal::get_special_slot_value::<String>(
        //     isolate,
        //     instance_map,
        //     ReadOnlyRoots::shared_struct_map_registry_key_symbol(isolate),
        // )
        todo!("Fixing the typing here");
    }

    pub fn is_registry_key_descriptor(
        isolate: &mut Isolate,
        instance_map: &Map,
        i: InternalIndex,
    ) -> bool {
        debug_assert!(is_js_shared_struct_map(instance_map));
        instance_map.instance_descriptors().get_key(i)
            == ReadOnlyRoots::shared_struct_map_registry_key_symbol(isolate)
    }

    pub fn get_elements_template(
        isolate: &mut Isolate,
        instance_map: &Map,
    ) -> Result<NumberDictionary, Error> {
        // internal::get_special_slot_value::<NumberDictionary>(
        //     isolate,
        //     instance_map,
        //     ReadOnlyRoots::shared_struct_map_elements_template_symbol(isolate),
        // )
        todo!("Fixing the typing here");
    }

    pub fn is_elements_template_descriptor(
        isolate: &mut Isolate,
        instance_map: &Map,
        i: InternalIndex,
    ) -> bool {
        debug_assert!(is_js_shared_struct_map(instance_map));
        instance_map.instance_descriptors().get_key(i)
            == ReadOnlyRoots::shared_struct_map_elements_template_symbol(isolate)
    }
}

// Hash table mapping string keys to shared struct maps.
//class SharedStructTypeRegistry::Data : public OffHeapHashTableBase<Data> {
struct SharedStructTypeRegistryData {
   //TODO: OffHeapHashTableBase Implementation
}

impl SharedStructTypeRegistry {
    pub fn new() -> Self {
        SharedStructTypeRegistry {
            data_: Box::new(SharedStructTypeRegistryData{}), //Data::new(Data::kMinCapacity)),
            data_mutex_: Mutex::new(()),
        }
    }

    fn check_if_entry_matches(
        &self,
        isolate: &mut Isolate,
        entry: InternalIndex,
        key: &mut String,
        field_names: &[&mut Name],
        element_names: &HashSet<u32>,
    ) -> Result<Map, Error> {
        
        // A map is considered a match iff all of the following hold:
        // - field names are the same element-wise (in order)
        // - element indices are the same
        todo!("SharedStructTypeRegistryData::check_if_entry_matches")
    }

    pub fn register_no_throw(
        isolate: &mut Isolate,
        key: &mut String,
        field_names: &[&mut Name],
        element_names: &HashSet<u32>,
    ) -> Result<Map, Error> {
       // key = isolate.factory().internalize_string(key);
        todo!("Factory not created, internalize_string pending");

        // To avoid deadlock with iteration during GC and modifying the table, no GC
        // must occur under lock.

        // {
        //     let _data_guard = self.data_mutex_.lock().unwrap();
        //     let entry = self.data_.find_entry(isolate, key, key.hash());
        //     if entry.is_found() {
        //         return self.check_if_entry_matches(isolate, entry, key, field_names, element_names);
        //     }
        // }

        // // We have a likely miss. Create a new instance map outside of the lock.
        // let map = JSSharedStruct::create_instance_map(isolate, field_names, element_names, Some(key));

        // // Relookup to see if it's in fact a miss.
        // let _data_guard = self.data_mutex_.lock().unwrap();

        // self.ensure_capacity(isolate.cage_base(), 1);
        // let entry = self.data_.find_entry_or_insertion_entry(isolate, key, key.hash());
        // let existing_key = self.data_.get_key(isolate, entry);
        // if existing_key == Data::empty_element() {
        //     self.data_.add_at(isolate, entry, map);
        //     return Ok(map);
        // } else if existing_key == Data::deleted_element() {
        //     self.data_.overwrite_deleted_at(isolate, entry, map);
        //     return Ok(map);
        // } else {
        //     // An entry with the same key was inserted between the two locks.
        //     return self.check_if_entry_matches(isolate, entry, key, field_names, element_names);
        // }
        todo!("SharedStructTypeRegistryData::register_no_throw")
    }

    pub fn register(
        isolate: &mut Isolate,
        key: &mut String,
        field_names: &[&mut Name],
        element_names: &HashSet<u32>,
    ) -> Result<Map, Error> {
        let canonical_map = self.register_no_throw(isolate, key, field_names, element_names);
        // if canonical_map.is_null() {
        //     THROW_NEW_ERROR(
        //         isolate,
        //         NewTypeError(MessageTemplate::kSharedStructTypeRegistryMismatch, key),
        //     );
        // }
        // return canonical_map;
        todo!("SharedStructTypeRegistryData::register")
    }

    pub fn iterate_elements(isolate: &mut Isolate, visitor: &mut RootVisitor) {
        // Ideally this should only happen during a global safepoint, when all
        // workers and background threads are paused, so there would be no need to
        // take the data mutex. However, the array left trimming has a verifier
        // visitor that visits all roots (including weak ones), thus we take the
        // mutex.
        //
        // TODO(v8:12547): Figure out how to do
        // isolate->global_safepoint()->AssertActive() instead.
        // let _data_guard = self.data_mutex_.lock().unwrap();
        // self.data_.iterate_elements(Root::kSharedStructTypeRegistry, visitor);
        todo!("SharedStructTypeRegistryData::iterate_elements")
    }

    pub fn notify_elements_removed(count: i32) {
        // self.data_.elements_removed(count);
        todo!("SharedStructTypeRegistryData::notify_elements_removed")
    }

    fn ensure_capacity(&mut self, cage_base: PtrComprCageBase, additional_elements: i32) {
        // self.data_mutex_.assert_held();

        // let mut new_capacity;
        // if self.data_.should_resize_to_add(additional_elements, &mut new_capacity) {
        //     let new_data = Data::new(new_capacity);
        //     self.data_.rehash_into(cage_base, &mut new_data);
        //     self.data_ = new_data;
        // }
        todo!("SharedStructTypeRegistryData::ensure_capacity")
    }
}

trait ObjectTrait {
    // Define common methods required for objects
}