// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/objects/descriptor-array-inl.h

// This is a header file, so we define a module.
pub mod descriptor_array_inl {
    use std::cmp::Ordering;
    use std::ops::Deref;
    use std::ptr::NonNull;
    use crate::execution::isolate::Isolate;
    use crate::handles::maybe_handles_inl::MaybeHandle;
    use crate::heap::heap_write_barrier::HeapWriteBarrier;
    use crate::heap::heap::Heap;
    use crate::objects::api_callbacks::ApiCallbacks;
    use crate::objects::dictionary::Dictionary;
    use crate::objects::field_type::FieldType;
    use crate::objects::heap_object_inl::HeapObject;
    use crate::objects::lookup_cache_inl::LookupCache;
    use crate::objects::maybe_object_inl::MaybeObject;
    use crate::objects::property::PropertyDetails;
    use crate::objects::property::PropertyLocation;
    use crate::objects::struct_value::Struct;
    use crate::objects::tagged_field_inl::TaggedField;
    use crate::torque::runtime_macro_shims::RuntimeMacroShims;
    use crate::torque::runtime_support::RuntimeSupport;
    use crate::objects::name::Name;
    use crate::objects::smi::Smi;
    use crate::objects::map::Map;
    use crate::objects::object::Object;
    use crate::objects::descriptor_array::*;
    use crate::heap::safepoint::PtrComprCageBase;

    // Macro replacements (approximations)
    macro_rules! tq_object_constructors_impl {
        ($struct_name:ident) => {
            // Placeholder, as constructors are handled differently in Rust.
        };
    }

    macro_rules! relaxed_int16_accessors {
        ($struct_name:ident, $field_name:ident, $offset:expr) => {
            // Placeholder, using direct field access with interior mutability or methods.
        };
    }

    // Implementations for DescriptorArray
    impl DescriptorArray {
        pub fn number_of_slack_descriptors(&self) -> i16 {
            self.number_of_all_descriptors() - self.number_of_descriptors()
        }

        pub fn number_of_entries(&self) -> i32 {
            self.number_of_descriptors() as i32
        }

        pub fn copy_enum_cache_from(&mut self, array: &DescriptorArray) {
            self.set_enum_cache(array.enum_cache());
        }

        pub fn search(&self, name: TaggedName, valid_descriptors: i32, concurrent_search: bool) -> InternalIndex {
            if !name.is_unique_name() {
                panic!("Name must be unique"); //DCHECK
            }

            if valid_descriptors == 0 {
                return InternalIndex::NotFound();
            }

            const K_MAX_ELEMENTS_FOR_LINEAR_SEARCH: i32 = 8;

            if valid_descriptors <= K_MAX_ELEMENTS_FOR_LINEAR_SEARCH || concurrent_search {
                return self.linear_search(name, valid_descriptors);
            }

            self.binary_search(name, valid_descriptors)
        }

        fn binary_search(&self, name: TaggedName, valid_descriptors: i32) -> InternalIndex {
            let end = self.number_of_descriptors() as usize;
            let hash = name.hash();

            let number = (0..end).lower_bound(|&i| {
                let entry = self.get_sorted_key(i as i32);
                entry.hash().cmp(&hash) != Ordering::Less
            });

            for number in number..end {
                let index = InternalIndex::new(self.get_sorted_key_index(number as i32));
                let entry = self.get_key(index);

                if entry == name {
                    if index.as_int() >= valid_descriptors {
                        return InternalIndex::NotFound();
                    }
                    return index;
                }
                if entry.hash() != hash {
                    return InternalIndex::NotFound();
                }
            }
            InternalIndex::NotFound()
        }

        fn linear_search(&self, name: TaggedName, valid_descriptors: i32) -> InternalIndex {
            for i in 0..valid_descriptors {
                let index = InternalIndex::new(i);
                if self.get_key(index) == name {
                    return index;
                }
            }
            InternalIndex::NotFound()
        }

        pub fn search_map(&self, name: TaggedName, map: &Map, concurrent_search: bool) -> InternalIndex {
            if !name.is_unique_name() {
                panic!("Name must be unique"); //DCHECK
            }

            let number_of_own_descriptors = map.number_of_own_descriptors();
            if number_of_own_descriptors == 0 {
                return InternalIndex::NotFound();
            }
            self.search(name, number_of_own_descriptors, concurrent_search)
        }

        pub fn search_field_index(&self, field_index: i32, valid_descriptors: i32) -> InternalIndex {
            for desc_index in field_index..valid_descriptors {
                let details = self.get_details(InternalIndex::new(desc_index));
                if details.location() != PropertyLocation::kField {
                    continue;
                }
                if field_index == details.field_index() {
                    return InternalIndex::new(desc_index);
                }
                if details.field_index() >= field_index {
                    panic!("details.field_index() should be less than field_index"); //DCHECK_LT
                }
            }
            InternalIndex::NotFound()
        }

        pub fn search_field_index_map(&self, field_index: i32, map: &Map) -> InternalIndex {
            let number_of_own_descriptors = map.number_of_own_descriptors();
            if number_of_own_descriptors == 0 {
                return InternalIndex::NotFound();
            }
            self.search_field_index(field_index, number_of_own_descriptors)
        }

        pub fn search_with_cache(&self, isolate: &mut Isolate, name: TaggedName, map: &Map) -> InternalIndex {
            if !name.is_unique_name() {
                panic!("Name must be unique"); //DCHECK
            }

            let number_of_own_descriptors = map.number_of_own_descriptors();
            if number_of_own_descriptors == 0 {
                return InternalIndex::NotFound();
            }

            let cache = isolate.descriptor_lookup_cache();
            let mut cache_borrow = cache.borrow_mut();
            let number = cache_borrow.lookup(map, name);

            let result = if number == DescriptorLookupCache::K_ABSENT {
                let result = self.search(name, number_of_own_descriptors, false);
                let number = if result.is_found() {
                    result.as_int()
                } else {
                    DescriptorArray::K_NOT_FOUND
                };
                cache_borrow.update(map, name, number);
                result
            } else if number == DescriptorArray::K_NOT_FOUND {
                InternalIndex::NotFound()
            } else {
                InternalIndex::new(number)
            };

            result
        }

        pub fn get_first_pointer_slot(&self) -> ObjectSlot {
            //static_assert!(kEndOfStrongFieldsOffset == kStartOfWeakFieldsOffset,
            //    "Weak and strong fields are continuous.");
            //static_assert!(kEndOfWeakFieldsOffset == kHeaderSize,
            //    "Weak fields extend up to the end of the header.");
            self.raw_field(DescriptorArray::K_START_OF_STRONG_FIELDS_OFFSET)
        }

        pub fn get_descriptor_slot(&self, descriptor: i32) -> ObjectSlot {
            if descriptor > self.number_of_all_descriptors() as i32 {
                panic!("descriptor must be less than or equal to number_of_all_descriptors");
            }
            self.raw_field(self.offset_of_descriptor_at(descriptor))
        }

        pub fn is_initialized_descriptor(&self, descriptor_number: InternalIndex) -> bool {
            if descriptor_number.as_int() >= self.number_of_descriptors() {
                panic!("descriptor_number must be less than number_of_descriptors");
            }

            let entry_offset = self.offset_of_descriptor_at(descriptor_number.as_int());
            let cage_base = self.get_ptr_compr_cage_base();
            let maybe_name = EntryKeyField::relaxed_load(cage_base, self, entry_offset);
            let is_initialized = maybe_name.is_undefined();

            if is_initialized {
                let details = EntryDetailsField::relaxed_load(self, entry_offset);
                if !details.is_smi() {
                    panic!("details must be a smi");
                }
            }

            !is_initialized
        }

        pub fn get_key(&self, descriptor_number: InternalIndex) -> TaggedName {
            let cage_base = self.get_ptr_compr_cage_base();
            self.get_key_with_cage_base(cage_base, descriptor_number)
        }

        pub fn get_key_with_cage_base(&self, cage_base: PtrComprCageBase, descriptor_number: InternalIndex) -> TaggedName {
            if descriptor_number.as_int() >= self.number_of_descriptors() {
                panic!("descriptor_number must be less than number_of_descriptors");
            }

            let entry_offset = self.offset_of_descriptor_at(descriptor_number.as_int());
            EntryKeyField::relaxed_load(cage_base, self, entry_offset).cast()
        }

        pub fn set_key(&mut self, descriptor_number: InternalIndex, key: TaggedName) {
            if descriptor_number.as_int() >= self.number_of_descriptors() {
                panic!("descriptor_number must be less than number_of_descriptors");
            }

            let entry_offset = self.offset_of_descriptor_at(descriptor_number.as_int());
            EntryKeyField::relaxed_store(self, entry_offset, key);
            self.write_barrier(entry_offset + DescriptorArray::K_ENTRY_KEY_OFFSET, key.into());
        }

        pub fn get_sorted_key_index(&self, descriptor_number: i32) -> i32 {
            self.get_details(InternalIndex::new(descriptor_number)).pointer()
        }

        pub fn get_sorted_key(&self, descriptor_number: i32) -> TaggedName {
            let cage_base = self.get_ptr_compr_cage_base();
            self.get_sorted_key_with_cage_base(cage_base, descriptor_number)
        }

        pub fn get_sorted_key_with_cage_base(&self, cage_base: PtrComprCageBase, descriptor_number: i32) -> TaggedName {
            let index = self.get_sorted_key_index(descriptor_number);
            self.get_key_with_cage_base(cage_base, InternalIndex::new(index))
        }

        pub fn set_sorted_key(&mut self, descriptor_number: i32, pointer: i32) {
            let details = self.get_details(InternalIndex::new(descriptor_number));
            self.set_details(InternalIndex::new(descriptor_number), details.set_pointer(pointer));
        }

        pub fn get_strong_value(&self, descriptor_number: InternalIndex) -> TaggedObject {
            let cage_base = self.get_ptr_compr_cage_base();
            self.get_strong_value_with_cage_base(cage_base, descriptor_number)
        }

        pub fn get_strong_value_with_cage_base(&self, cage_base: PtrComprCageBase, descriptor_number: InternalIndex) -> TaggedObject {
            self.get_value_with_cage_base(cage_base, descriptor_number).cast()
        }

        pub fn set_value(&mut self, descriptor_number: InternalIndex, value: TaggedMaybeObject) {
            if descriptor_number.as_int() >= self.number_of_descriptors() {
                panic!("descriptor_number must be less than number_of_descriptors");
            }

            let entry_offset = self.offset_of_descriptor_at(descriptor_number.as_int());
            EntryValueField::relaxed_store(self, entry_offset, value);
            self.write_barrier(entry_offset + DescriptorArray::K_ENTRY_VALUE_OFFSET, value.into());
        }

        pub fn get_value(&self, descriptor_number: InternalIndex) -> TaggedMaybeObject {
            let cage_base = self.get_ptr_compr_cage_base();
            self.get_value_with_cage_base(cage_base, descriptor_number)
        }

        pub fn get_value_with_cage_base(&self, cage_base: PtrComprCageBase, descriptor_number: InternalIndex) -> TaggedMaybeObject {
            if descriptor_number.as_int() >= self.number_of_descriptors() {
                panic!("descriptor_number must be less than number_of_descriptors");
            }

            let entry_offset = self.offset_of_descriptor_at(descriptor_number.as_int());
            EntryValueField::relaxed_load(cage_base, self, entry_offset)
        }

        pub fn get_details(&self, descriptor_number: InternalIndex) -> PropertyDetails {
            if descriptor_number.as_int() >= self.number_of_descriptors() {
                panic!("descriptor_number must be less than number_of_descriptors");
            }

            let entry_offset = self.offset_of_descriptor_at(descriptor_number.as_int());
            let details = EntryDetailsField::relaxed_load(self, entry_offset);
            PropertyDetails::from_smi(details)
        }

        pub fn set_details(&mut self, descriptor_number: InternalIndex, details: PropertyDetails) {
            if descriptor_number.as_int() >= self.number_of_descriptors() {
                panic!("descriptor_number must be less than number_of_descriptors");
            }

            let entry_offset = self.offset_of_descriptor_at(descriptor_number.as_int());
            EntryDetailsField::relaxed_store(self, entry_offset, details.as_smi());
        }

        pub fn get_field_index(&self, descriptor_number: InternalIndex) -> i32 {
            if self.get_details(descriptor_number).location() != PropertyLocation::kField {
                panic!("location must be kField");
            }

            self.get_details(descriptor_number).field_index()
        }

        pub fn get_field_type(&self, descriptor_number: InternalIndex) -> TaggedFieldType {
            let cage_base = self.get_ptr_compr_cage_base();
            self.get_field_type_with_cage_base(cage_base, descriptor_number)
        }

        pub fn get_field_type_with_cage_base(&self, cage_base: PtrComprCageBase, descriptor_number: InternalIndex) -> TaggedFieldType {
            if self.get_details(descriptor_number).location() != PropertyLocation::kField {
                panic!("location must be kField");
            }

            let wrapped_type = self.get_value_with_cage_base(cage_base, descriptor_number);
            Map::unwrap_field_type(wrapped_type)
        }

        pub fn set(
            &mut self,
            descriptor_number: InternalIndex,
            key: TaggedName,
            value: TaggedMaybeObject,
            details: PropertyDetails,
        ) {
            if descriptor_number.as_int() >= self.number_of_descriptors() {
                panic!("descriptor_number must be less than number_of_descriptors");
            }

            self.set_key(descriptor_number, key);
            self.set_details(descriptor_number, details);
            self.set_value(descriptor_number, value);
        }

        pub fn set_descriptor(&mut self, descriptor_number: InternalIndex, desc: &Descriptor) {
            let key = *desc.get_key();
            let value = *desc.get_value();
            self.set(descriptor_number, key, value, desc.get_details());
        }

        pub fn append(&mut self, desc: &Descriptor) {
            let mut descriptor_number = self.number_of_descriptors() as i32;
            if descriptor_number + 1 > self.number_of_all_descriptors() as i32 {
                panic!("descriptor_number + 1 must be less than or equal to number_of_all_descriptors");
            }

            self.set_number_of_descriptors(descriptor_number + 1);
            self.set_descriptor(InternalIndex::new(descriptor_number), desc);

            let desc_hash = desc.get_key().hash();
            let mut collision_hash: u32 = 0;

            let mut insertion = descriptor_number;

            while insertion > 0 {
                let key = self.get_sorted_key(insertion - 1);
                collision_hash = key.hash();
                if collision_hash <= desc_hash {
                    break;
                }
                self.set_sorted_key(insertion, self.get_sorted_key_index(insertion - 1));
                insertion -= 1;
            }

            self.set_sorted_key(insertion, descriptor_number);

            if collision_hash != desc_hash {
                return;
            }

            self.check_name_collision_during_insertion(desc, desc_hash, insertion);
        }

        pub fn swap_sorted_keys(&mut self, first: i32, second: i32) {
            let first_key = self.get_sorted_key_index(first);
            self.set_sorted_key(first, self.get_sorted_key_index(second));
            self.set_sorted_key(second, first_key);
        }

        fn raw_field(&self, offset: usize) -> ObjectSlot {
            // Placeholder for actual raw field access.
            ObjectSlot {}
        }

        fn offset_of_descriptor_at(&self, descriptor: i32) -> usize {
            // Placeholder for offset calculation.
            0 // Replace with actual offset calculation based on `descriptor`.
        }

        fn write_barrier(&mut self, offset: usize, value: TaggedObject) {
            // Placeholder for write barrier.
        }

        fn get_ptr_compr_cage_base(&self) -> PtrComprCageBase {
            // Placeholder for PtrComprCageBase retrieval.
            PtrComprCageBase {}
        }

        fn set_number_of_descriptors(&mut self, value: i32) {
            // Placeholder for setting the number of descriptors.
        }

        fn number_of_descriptors(&self) -> i32 {
            // Placeholder for getting the number of descriptors.
            0
        }

        fn number_of_all_descriptors(&self) -> i16 {
            // Placeholder for getting the number of all descriptors.
            0
        }

        fn enum_cache(&self) -> EnumCache {
            // Placeholder for enum_cache.
            EnumCache {}
        }

        fn set_enum_cache(&mut self, _cache: EnumCache) {
            // Placeholder for setting enum cache.
        }

        fn check_name_collision_during_insertion(&self, _desc: &Descriptor, _hash: u32, _index: i32) {
            // Placeholder for name collision check
        }
    }

    // Implementations for DescriptorArrayMarkingState
    pub struct DescriptorArrayMarkingState {}

    impl DescriptorArrayMarkingState {
        pub fn try_update_indices_to_mark(
            gc_epoch: u32,
            array: &DescriptorArray,
            index_to_mark: DescriptorIndex,
        ) -> bool {
            let current_epoch = gc_epoch & Epoch::K_MASK;
            loop {
                let raw_gc_state = array.raw_gc_state_relaxed_load();
                let epoch_from_state = Epoch::decode(raw_gc_state);
                let mut new_raw_gc_state: RawGCStateType = 0;

                if current_epoch != epoch_from_state {
                    if raw_gc_state != 0 {
                        if Epoch::decode(epoch_from_state + 1) != current_epoch {
                            panic!("Epoch mismatch");
                        }
                    }
                    new_raw_gc_state = Self::new_state(current_epoch, 0, index_to_mark);
                } else {
                    let already_marked = Marked::decode(raw_gc_state);
                    let delta = Delta::decode(raw_gc_state);
                    if (already_marked + delta) >= index_to_mark {
                        return false;
                    }
                    new_raw_gc_state = Self::new_state(current_epoch, already_marked, index_to_mark - already_marked);
                }

                if Self::swap_state(array, raw_gc_state, new_raw_gc_state) {
                    return true;
                }
            }
        }

        pub fn acquire_descriptor_range_to_mark(
            gc_epoch: u32,
            array: &DescriptorArray,
        ) -> (DescriptorIndex, DescriptorIndex) {
            let current_epoch = gc_epoch & Epoch::K_MASK;
            loop {
                let raw_gc_state = array.raw_gc_state_relaxed_load();
                let marked = Marked::decode(raw_gc_state);
                let delta = Delta::decode(raw_gc_state);

                if current_epoch != Epoch::decode(raw_gc_state) || (marked + delta) == 0 {
                    let number_of_descriptors = if array.number_of_descriptors() != 0 {
                        array.number_of_descriptors()
                    } else {
                        array.number_of_all_descriptors() as i32
                    };

                    if number_of_descriptors <= 0 {
                        panic!("Number of descriptors should be greater than 0");
                    }

                    if Self::swap_state(array, raw_gc_state, Self::new_state(current_epoch, number_of_descriptors, 0)) {
                        return (0, number_of_descriptors);
                    }
                    continue;
                }

                if delta == 0 {
                    return (marked, marked);
                }

                if Self::swap_state(array, raw_gc_state, Self::new_state(current_epoch, marked + delta, 0)) {
                    return (marked, marked + delta);
                }
            }
        }

        fn new_state(epoch: u32, marked: i32, delta: i32) -> RawGCStateType {
            Epoch::encode(epoch) | Marked::encode(marked) | Delta::encode(delta)
        }

        fn swap_state(array: &DescriptorArray, expected: RawGCStateType, new: RawGCStateType) -> bool {
            // Placeholder for actual atomic swap.
            true
        }
    }

    // --- Helper structs for bitfield manipulation ---

    struct Epoch {}
    impl Epoch {
        const K_SHIFT: i32 = 24;
        const K_MASK: u32 = 0x00FFFFFF;

        fn decode(raw_gc_state: RawGCStateType) -> u32 {
            (raw_gc_state >> Self::K_SHIFT) as u32 & Self::K_MASK
        }

        fn encode(epoch: u32) -> RawGCStateType {
            (epoch << Self::K_SHIFT) as RawGCStateType
        }
    }

    struct Marked {}
    impl Marked {
        const K_SHIFT: i32 = 12;
        const K_MASK: RawGCStateType = 0x0FFF000;

        fn decode(raw_gc_state: RawGCStateType) -> i32 {
            ((raw_gc_state & Self::K_MASK) >> Self::K_SHIFT) as i32
        }

        fn encode(marked: i32) -> RawGCStateType {
            ((marked << Self::K_SHIFT) & Self::K_MASK) as RawGCStateType
        }
    }

    struct Delta {}
    impl Delta {
        const K_SHIFT: i32 = 0;
        const K_MASK: RawGCStateType = 0x00000FFF;

        fn decode(raw_gc_state: RawGCStateType) -> i32 {
            (raw_gc_state & Self::K_MASK) as i32
        }

        fn encode(delta: i32) -> RawGCStateType {
            (delta & Self::K_MASK) as RawGCStateType
        }
    }

    // --- Type definitions ---

    type DescriptorIndex = i32;
    type RawGCStateType = u32;
    type TaggedName = Name; // Replace with the actual Tagged<Name> type if needed
    type TaggedObject = Object;
    type TaggedMaybeObject = MaybeObject;
    type TaggedFieldType = FieldType;
    struct ObjectSlot {} // Dummy type
    struct EnumCache {} //Dummy type
    struct Descriptor {} //Dummy type

    struct EntryKeyField {}
    impl EntryKeyField {
        fn relaxed_load(_cage_base: PtrComprCageBase, _descriptor_array: &DescriptorArray, _entry_offset: usize) -> TaggedMaybeObject {
            // Placeholder implementation
            MaybeObject::from_smi(Smi::zero())
        }

        fn relaxed_store(_descriptor_array: &DescriptorArray, _entry_offset: usize, _value: TaggedName) {
            // Placeholder implementation
        }
    }

    struct EntryDetailsField {}
    impl EntryDetailsField {
        fn relaxed_load(_descriptor_array: &DescriptorArray, _entry_offset: usize) -> Smi {
            // Placeholder implementation
            Smi::zero()
        }

        fn relaxed_store(_descriptor_array: &DescriptorArray, _entry_offset: usize, _details: Smi) {
            // Placeholder implementation
        }
    }

    struct EntryValueField {}
    impl EntryValueField {
        fn relaxed_load(_cage_base: PtrComprCageBase, _descriptor_array: &DescriptorArray, _entry_offset: usize) -> TaggedMaybeObject {
            // Placeholder implementation
            MaybeObject::from_smi(Smi::zero())
        }

        fn relaxed_store(_descriptor_array: &DescriptorArray, _entry_offset: usize, _value: TaggedMaybeObject) {
            // Placeholder implementation
        }
    }

    trait UniqueName {
        fn is_unique_name(&self) -> bool;
    }

    impl UniqueName for TaggedName {
        fn is_unique_name(&self) -> bool {
            // Placeholder for actual check
            true
        }
    }

    trait NameHash {
        fn hash(&self) -> u32;
    }

    impl NameHash for TaggedName {
        fn hash(&self) -> u32 {
            // Placeholder implementation
            0
        }
    }

    trait MaybeUndefined {
        fn is_undefined(&self) -> bool;
    }

    impl MaybeUndefined for TaggedMaybeObject {
        fn is_undefined(&self) -> bool {
            // Placeholder
            true
        }
    }

    trait ObjectCast {
        fn cast(self) -> Object;
    }

    impl ObjectCast for TaggedMaybeObject {
        fn cast(self) -> Object {
            //Placeholder
            Object::null()
        }
    }

    impl DescriptorArray {
        const K_NOT_FOUND: i32 = -1;
        const K_START_OF_STRONG_FIELDS_OFFSET: usize = 0;
        const K_ENTRY_KEY_OFFSET: usize = 0;
        const K_ENTRY_VALUE_OFFSET: usize = 0;

        fn raw_gc_state_relaxed_load(&self) -> RawGCStateType {
            0 // Placeholder
        }
    }

    trait SmiCheck {
        fn is_smi(&self) -> bool;
    }

    impl SmiCheck for Smi {
        fn is_smi(&self) -> bool {
            true
        }
    }

    trait MapUnwrapFieldType {
        fn unwrap_field_type(wrapped_type: TaggedMaybeObject) -> TaggedFieldType;
    }

    impl MapUnwrapFieldType for Map {
        fn unwrap_field_type(_wrapped_type: TaggedMaybeObject) -> TaggedFieldType {
            FieldType::any()
        }
    }

    trait NullObject {
        fn null() -> Self;
    }

    impl NullObject for Object {
        fn null() -> Self {
            Object{}
        }
    }

    // Binary search extension trait
    trait LowerBound<T> {
        fn lower_bound<F>(&self, func: F) -> usize
        where
            F: Fn(&T) -> bool;
    }

    impl<T> LowerBound<T> for std::ops::Range<usize> {
        fn lower_bound<F>(&self, func: F) -> usize
        where
            F: Fn(&T) -> bool,
        {
            let mut left = self.start;
            let mut right = self.end;

            while left < right {
                let mid = left + (right - left) / 2;
                let value = mid as T;

                if func(&value) {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }

            left
        }
    }
}