// Converted from V8 C++ source files:
// Header: js-struct.h
// Implementation: js-struct.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod js_struct {
    // Copyright 2022 the V8 project authors. All rights reserved.
    // Use of this source code is governed by a BSD-style license that can be
    // found in the LICENSE file.

    #![allow(dead_code)]
    #![allow(non_snake_case)]
    use crate::objects::js_objects::*;
    use crate::objects::smi::*;
    use std::cell::RefCell;
    use std::collections::HashSet;
    use std::rc::Rc;
    use std::sync::Mutex;

    pub struct AlwaysSharedSpaceJSObject {}

    impl AlwaysSharedSpaceJSObject {
        // Prepare a Map to be used as the instance map for shared JS objects.
        pub fn PrepareMapNoEnumerableProperties(map: &mut Map) {
            Self::prepare_map_common(map);
            map.SetEnumLength(0);
        }
        pub fn PrepareMapNoEnumerableProperties_2(
            isolate: *mut Isolate,
            map: &mut Map,
            descriptors: &DescriptorArray,
        ) {
            Self::prepare_map_common(map);
            map.InitializeDescriptors(isolate, descriptors);
            assert_eq!(0, map.NumberOfEnumerableProperties());
            map.SetEnumLength(0);
        }

        pub fn PrepareMapWithEnumerableProperties(
            isolate: *mut Isolate,
            map: &mut Map,
            descriptors: &DescriptorArray,
            enum_length: i32,
        ) {
            Self::prepare_map_common(map);
            map.InitializeDescriptors(isolate, descriptors);
            //FastKeyAccumulator::InitializeFastPropertyEnumCache(
            //    isolate, map, enum_length, AllocationType::kSharedOld,
            //);
            map.SetEnumLength(enum_length);
        }

        pub fn DefineOwnProperty(
            isolate: *mut Isolate,
            shared_obj: &mut AlwaysSharedSpaceJSObject,
            key: &mut Object,
            desc: &mut PropertyDescriptor,
            should_throw: Option<ShouldThrow>,
        ) -> Result<bool, String> {
            if !isName(key) && !isNumber(key) {
                return Err("key is not Name or Number".to_string());
            }
            let lookup_key = PropertyKey { dummy: 0 }; //PropertyKey(isolate, key);
            let mut it = LookupIterator { dummy: 0 }; //LookupIterator(isolate, shared_obj, lookup_key, LookupIterator::OWN);
            let mut current = PropertyDescriptor { dummy: 0 };
            //MAYBE_RETURN(GetOwnPropertyDescriptor(&it, &current), Nothing<bool>());
            //if !it.IsFound() || PropertyDescriptor::IsDataDescriptor(desc) != PropertyDescriptor::IsDataDescriptor(&current) || desc.ToAttributes() != current.ToAttributes() {
            //    //DCHECK(!shared_obj->map()->is_extensible());
            //    //RETURN_FAILURE(isolate, GetShouldThrow(isolate, should_throw), NewTypeError(MessageTemplate::kDefineDisallowedFixedLayout, it.GetName()));
            //    return Err("DefineOwnProperty failed".to_string());
            //}
            //DCHECK(it.property_attributes() == desc->ToAttributes());
            //if desc.has_value() {
            //    return Object::SetDataProperty(&it, desc->value());
            //}
            Ok(true)
        }

        pub fn HasInstance(
            isolate: *mut Isolate,
            constructor: &mut JSFunction,
            object: &mut Object,
        ) -> Result<bool, String> {
            if !constructor.has_prototype_slot() || !constructor.has_initial_map() || !isJSReceiver(object)
            {
                return Ok(false);
            }
            //DirectHandle<Map> constructor_map(constructor->initial_map(), isolate);
            //PrototypeIterator iter(isolate, Cast<JSReceiver>(object), kStartAtReceiver);
            //DirectHandle<Map> current_map;
            //while (true) {
            //    current_map = direct_handle(PrototypeIterator::GetCurrent(iter)->map(), isolate);
            //    if (current_map.is_identical_to(constructor_map)) {
            //        return Just(true);
            //    }
            //    if (!iter.AdvanceFollowingProxies()) return Nothing<bool>();
            //    if (iter.IsAtEnd()) return Just(false);
            //}
            Ok(true)
        }

        fn prepare_map_common(map: &mut Map) {
            //DCHECK(IsAlwaysSharedSpaceJSObjectMap(map));
            //DisallowGarbageCollection no_gc;
            //// Shared objects have fixed layout ahead of time, so there's no slack.
            map.SetInObjectUnusedPropertyFields(0);
            //// Shared objects are not extensible and have a null prototype.
            map.set_is_extensible(false);
            //// Shared space objects are not optimizable as prototypes because it is
            //// not threadsafe.
            //map->set_prototype_validity_cell(Map::kPrototypeChainValidSmi, kRelaxedStore,
            //                                   SKIP_WRITE_BARRIER);
        }
    }

    pub struct JSSharedStruct {}

    impl JSSharedStruct {
        pub fn CreateInstanceMap(
            isolate: *mut Isolate,
            field_names: &Vec<&Name>,
            element_names: &HashSet<u32>,
            maybe_registry_key: Option<&String>,
        ) -> Result<Box<Map>, String> {
            //let factory = isolate.factory();

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

            let descriptors: Option<Box<DescriptorArray>>;
            if num_descriptors != 0 {
                descriptors = Some(Box::new(DescriptorArray { dummy: 0 }));
                //descriptors = factory.NewDescriptorArray(num_descriptors, 0,
                //                                          AllocationType::kSharedOld);

                let mut special_slots = 0;

                // Store the registry key if the map is registered. This must be the first
                // slot if present. The registry depends on this for rehashing.
                if let Some(registry_key) = maybe_registry_key {
                    //Descriptor d = Descriptor::DataConstant(
                    //    factory->shared_struct_map_registry_key_symbol(), registry_key,
                    //    ALL_ATTRIBUTES_MASK);
                    assert_eq!(0, special_slots);
                    //descriptors.Set(InternalIndex(special_slots++), &d);
                }

                // Elements in shared structs are only supported as a dictionary. Create the
                // template NumberDictionary if needed.
                if !element_names.is_empty() {
                    //DirectHandle<NumberDictionary> elements_template;
                    num_elements = element_names.len() as i32;
                    //elements_template = NumberDictionary::New(isolate, num_elements,
                    //                                            AllocationType::kSharedOld);
                    //for (uint32_t index : element_names) {
                    //    PropertyDetails details(PropertyKind::kData, SEALED,
                    //                            PropertyConstness::kMutable, 0);
                    //    NumberDictionary::UncheckedAdd<Isolate, DirectHandle,
                    //                                   AllocationType::kSharedOld>(
                    //        isolate, elements_template, index,
                    //        isolate->factory()->undefined_value(), details);
                    //}
                    //elements_template->SetInitialNumberOfElements(num_elements);
                    //DCHECK(HeapLayout::InAnySharedSpace(*elements_template));

                    //Descriptor d = Descriptor::DataConstant(
                    //    factory->shared_struct_map_elements_template_symbol(),
                    //    elements_template, ALL_ATTRIBUTES_MASK);
                    //descriptors.Set(InternalIndex(special_slots++), &d);
                }

                //DCHECK_LE(special_slots, kSpecialSlots);

                for field_name in field_names {
                    // Shared structs' fields need to be aligned, so make it all tagged.
                    //PropertyDetails details(
                    //    PropertyKind::kData, SEALED, PropertyLocation::kField,
                    //    PropertyConstness::kMutable, Representation::Tagged(), num_fields);
                    //descriptors.Set(InternalIndex(special_slots + num_fields), *field_name,
                    //                   FieldType::Any(), details);
                    num_fields += 1;
                }

                //descriptors.Sort();
            } else {
                descriptors = None;
            }

            // Calculate the size for instances and create the map.
            let instance_size: i32;
            let in_object_properties: i32;
            JSFunction::CalculateInstanceSizeHelper(
                JS_SHARED_STRUCT_TYPE,
                false,
                0,
                num_fields as i32,
                &mut instance_size,
                &mut in_object_properties,
            );
            //DirectHandle<Map> instance_map = factory->NewContextlessMap(
            //    JS_SHARED_STRUCT_TYPE, instance_size, DICTIONARY_ELEMENTS,
            //    in_object_properties, AllocationType::kSharedMap);
            let mut instance_map = Map { dummy: 0 };

            // Prepare the enum cache if necessary.
            if num_descriptors == 0 {
                assert_eq!(0, num_fields);
                // No properties at all.
                AlwaysSharedSpaceJSObject::PrepareMapNoEnumerableProperties(&mut instance_map);
            } else if num_fields == 0 {
                // Have descriptors, but no enumerable fields.
                //AlwaysSharedSpaceJSObject::PrepareMapNoEnumerableProperties(
                //    isolate, &mut instance_map, descriptors.unwrap().as_mut());
            } else {
                // Have enumerable fields.
                if let Some(desc) = descriptors.as_ref() {
                    AlwaysSharedSpaceJSObject::PrepareMapWithEnumerableProperties(
                        isolate,
                        &mut instance_map,
                        desc.as_ref(),
                        num_fields as i32,
                    );
                }
            }

            //// Structs have fixed layout ahead of time, so there's no slack.
            //let out_of_object_properties = num_fields - in_object_properties;
            //if out_of_object_properties != 0 {
            //    instance_map.SetOutOfObjectUnusedPropertyFields(0);
            //}

            Ok(Box::new(instance_map))
        }

        pub fn GetRegistryKey(isolate: *mut Isolate, instance_map: &Map) -> Result<String, String> {
            //GetSpecialSlotValue<String>(
            //    isolate, *instance_map,
            //    ReadOnlyRoots(isolate).shared_struct_map_registry_key_symbol());
            Ok("".to_string())
        }

        pub fn IsRegistryKeyDescriptor(
            isolate: *mut Isolate,
            instance_map: &Map,
            i: InternalIndex,
        ) -> bool {
            //DCHECK(IsJSSharedStructMap(instance_map));
            //return instance_map->instance_descriptors(isolate)->GetKey(i) ==
            //       ReadOnlyRoots(isolate).shared_struct_map_registry_key_symbol();
            false
        }

        pub fn GetElementsTemplate(
            isolate: *mut Isolate,
            instance_map: &Map,
        ) -> Result<NumberDictionary, String> {
            //GetSpecialSlotValue<NumberDictionary>(
            //    isolate, instance_map,
            //    ReadOnlyRoots(isolate).shared_struct_map_elements_template_symbol());
            Err("Not implemented".to_string())
        }

        pub fn IsElementsTemplateDescriptor(
            isolate: *mut Isolate,
            instance_map: &Map,
            i: InternalIndex,
        ) -> bool {
            //DCHECK(IsJSSharedStructMap(instance_map));
            //return instance_map->instance_descriptors(isolate)->GetKey(i) ==
            //       ReadOnlyRoots(isolate).shared_struct_map_elements_template_symbol();
            false
        }
    }

    // Hash table mapping string keys to shared struct maps.
    pub struct SharedStructTypeRegistry {
        data_: Box<Data>,
        data_mutex_: Mutex<()>,
    }

    impl SharedStructTypeRegistry {
        pub const fn deleted_element() -> Smi {
            Smi::FromInt(1)
        }

        pub fn new() -> Self {
            SharedStructTypeRegistry {
                data_: Data::New(Data::kMinCapacity),
                data_mutex_: Mutex::new(()),
            }
        }

        pub fn Register(
            &self,
            isolate: *mut Isolate,
            key: &String,
            field_names: &Vec<&Name>,
            element_names: &HashSet<u32>,
        ) -> Result<Box<Map>, String> {
            let canonical_map = self.RegisterNoThrow(isolate, key, field_names, element_names);
            match canonical_map {
                Ok(Some(map)) => Ok(map),
                _ => Err("SharedStructTypeRegistryMismatch".to_string()),
            }
        }

        fn CheckIfEntryMatches(
            &self,
            isolate: *mut Isolate,
            entry: InternalIndex,
            key: &String,
            field_names: &Vec<&Name>,
            element_names: &HashSet<u32>,
        ) -> Result<Option<Box<Map>>, String> {
            //Tagged<Map> existing_map = Cast<Map>(data_->GetKey(isolate, entry));
            //// A map is considered a match iff all of the following hold:
            //// - field names are the same element-wise (in order)
            //// - element indices are the same
            //// Registered types always have the key as the first descriptor.
            //DCHECK_EQ(
            //    *JSSharedStruct::GetRegistryKey(isolate, existing_map).ToHandleChecked(),
            //    *key);
            //int num_descriptors = static_cast<int>(field_names.size()) + 1;
            //if (!element_names.empty()) {
            //    if (JSSharedStruct::GetElementsTemplate(isolate, existing_map).is_null()) {
            //        return MaybeDirectHandle<Map>();
            //    }
            //    num_descriptors++;
            //}
            //if (num_descriptors != existing_map->NumberOfOwnDescriptors()) {
            //    return MaybeDirectHandle<Map>();
            //}
            //Tagged<DescriptorArray> existing_descriptors =
            //    existing_map->instance_descriptors(isolate);
            //auto field_names_iter = field_names.begin();
            //for (InternalIndex i : existing_map->IterateOwnDescriptors()) {
            //    if (JSSharedStruct::IsElementsTemplateDescriptor(isolate, existing_map,
            //                                                     i)) {
            //        DirectHandle<NumberDictionary> elements_template(
            //            Cast<NumberDictionary>(
            //                existing_map->instance_descriptors()->GetStrongValue(isolate, i)),
            //            isolate);
            //        if (static_cast<int>(element_names.size()) !=
            //            elements_template->NumberOfElements()) {
            //            return MaybeDirectHandle<Map>();
            //        }
            //        for (int element : element_names) {
            //            if (elements_template->FindEntry(isolate, element).is_not_found()) {
            //                return MaybeDirectHandle<Map>();
            //            }
            //        }
            //        continue;
            //    }
            //    if (JSSharedStruct::IsRegistryKeyDescriptor(isolate, existing_map, i)) {
            //        continue;
            //    }
            //    Tagged<Name> existing_name = existing_descriptors->GetKey(i);
            //    DCHECK(IsUniqueName(existing_name));
            //    Tagged<Name> name = **field_names_iter;
            //    DCHECK(IsUniqueName(name));
            //    if (name != existing_name) return MaybeDirectHandle<Map>();
            //    ++field_names_iter;
            //}
            //return direct_handle(existing_map, isolate);
            Ok(None)
        }

        fn RegisterNoThrow(
            &self,
            isolate: *mut Isolate,
            key: &String,
            field_names: &Vec<&Name>,
            element_names: &HashSet<u32>,
        ) -> Result<Option<Box<Map>>, String> {
            //key = isolate.factory().InternalizeString(key);
            //// To avoid deadlock with iteration during GC and modifying the table, no GC
            //// must occur under lock.
            //{
            //    NoGarbageCollectionMutexGuard data_guard(&data_mutex_);
            //    InternalIndex entry = data_->FindEntry(isolate, key, key->hash());
            //    if (entry.is_found()) {
            //        return CheckIfEntryMatches(isolate, entry, key, field_names,
            //                                     element_names);
            //    }
            //}
            //// We have a likely miss. Create a new instance map outside of the lock.
            //DirectHandle<Map> map = JSSharedStruct::CreateInstanceMap(
            //    isolate, field_names, element_names, key);
            //// Relookup to see if it's in fact a miss.
            //NoGarbageCollectionMutexGuard data_guard(&data_mutex_);
            //EnsureCapacity(isolate, 1);
            //InternalIndex entry =
            //    data_->FindEntryOrInsertionEntry(isolate, key, key->hash());
            //Tagged<Object> existing_key = data_->GetKey(isolate, entry);
            //if (existing_key == Data::empty_element()) {
            //    data_->AddAt(isolate, entry, *map);
            //    return map;
            //} else if (existing_key == Data::deleted_element()) {
            //    data_->OverwriteDeletedAt(isolate, entry, *map);
            //    return map;
            //} else {
            //    // An entry with the same key was inserted between the two locks.
            //    return CheckIfEntryMatches(isolate, entry, key, field_names, element_names);
            //}
            let map_result =
                JSSharedStruct::CreateInstanceMap(isolate, field_names, element_names, Some(key));
            match map_result {
                Ok(map) => Ok(Some(map)),
                Err(err) => Err(err),
            }
        }

        pub fn IterateElements(&self, isolate: *mut Isolate, visitor: &mut RootVisitor) {
            //base::MutexGuard data_guard(&data_mutex_);
            //data_->IterateElements(Root::kSharedStructTypeRegistry, visitor);
        }

        pub fn NotifyElementsRemoved(&self, count: i32) {
            self.data_.ElementsRemoved(count);
        }

        fn EnsureCapacity(&self, cage_base: PtrComprCageBase, additional_elements: i32) {
            let _data_guard = self.data_mutex_.lock().unwrap();
            let new_capacity: i32;
            if self.data_.ShouldResizeToAdd(additional_elements, &mut new_capacity) {
                let mut new_data = Data::New(new_capacity as i32);
                self.data_.RehashInto(cage_base, &mut new_data);
                self.data_ = new_data;
            }
        }
    }

    struct Data {
        table: Vec<Tagged<Object>>,
        capacity: i32,
        used: i32,
        deleted: i32,
    }

    impl Data {
        const kEntrySize: usize = 1;
        const kMaxEmptyFactor: i32 = 4;
        const kMinCapacity: i32 = 4;

        fn New(capacity: i32) -> Box<Data> {
            let table = vec![Tagged::<Object>::empty(); capacity as usize];
            Box::new(Data {
                table,
                capacity,
                used: 0,
                deleted: 0,
            })
        }

        fn Hash(_cage_base: PtrComprCageBase, key: &Object) -> u32 {
            0
        }

        fn ShouldResizeToAdd(&self, additional_elements: i32, new_capacity: &mut i32) -> bool {
            false
        }

        fn RehashInto(&mut self, _cage_base: PtrComprCageBase, _new_data: &mut Data) {}

        fn ElementsRemoved(&mut self, _count: i32) {}

        fn FindEntry(&self, isolate: *mut Isolate, key: &String, hash: u32) -> InternalIndex {
            InternalIndex { dummy: 0 }
        }
        fn FindEntryOrInsertionEntry(&self, isolate: *mut Isolate, key: &String, hash: u32) -> InternalIndex {
            InternalIndex { dummy: 0 }
        }

        fn AddAt(&mut self, isolate: *mut Isolate, entry: InternalIndex, map: &Map) {
            
        }
        fn OverwriteDeletedAt(&mut self, isolate: *mut Isolate, entry: InternalIndex, map: &Map) {
           
        }

        const fn empty_element() -> Tagged<Object> {
            Tagged::<Object>::empty()
        }
    }

    pub struct RootVisitor {}
    impl RootVisitor {
        
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct PtrComprCageBase {}
    pub fn isName(obj: &Object) -> bool {
        true
    }

    pub fn isNumber(obj: &Object) -> bool {
        true
    }

    pub fn isJSReceiver(obj: &Object) -> bool {
        true
    }

    pub const JS_SHARED_STRUCT_TYPE: i32 = 1;

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct PropertyKey {
        dummy: i32,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct LookupIterator {
        dummy: i32,
    }
    impl LookupIterator{

        pub fn IsFound(&self) -> bool {
            true
        }
        pub fn GetName(&self) -> String {
            "dummy".to_string()
        }

    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct ShouldThrow {}
}
