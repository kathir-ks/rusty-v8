// Converted from V8 C++ source files:
// Header: dictionary-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::optional::Optional;

use crate::execution::isolate_utils_inl::*;
use crate::numbers::hash_seed_inl::*;
use crate::objects::dictionary::*;
use crate::objects::hash_table_inl::*;
use crate::objects::objects_inl::*;
use crate::objects::oddball::*;
use crate::objects::property_cell_inl::*;
use crate::objects::object_macros::*;
use crate::objects::visitors::*;
use crate::objects::internal_index::*;
use crate::objects::property_array_inl::*;
use crate::objects::objects::*;
use crate::objects::tagged_impl_inl::*;
use crate::objects::tagged_field::*;
use crate::objects::simd::*;
use crate::runtime::runtime_wasm::*;
use crate::objects::js_date_time_format::*;
use crate::runtime::runtime_symbol::*;
use crate::objects::call_site_info::*;
use crate::objects::js_function_inl::*;
use crate::objects::casting_inl::*;
use v8::Handle;

trait DictionaryTrait<Derived, Shape> {
    fn value_at(&self, entry: InternalIndex) -> Tagged<Object>;
    fn value_at_with_cage(&self, cage_base: PtrComprCageBase, entry: InternalIndex) -> Tagged<Object>;
    fn value_at_with_tag(&self, entry: InternalIndex, tag: SeqCstAccessTag) -> Tagged<Object>;
    fn value_at_with_cage_and_tag(&self, cage_base: PtrComprCageBase, entry: InternalIndex, tag: SeqCstAccessTag) -> Tagged<Object>;
    fn try_value_at(&self, entry: InternalIndex) -> Optional<Tagged<Object>>;
    fn value_at_put(&mut self, entry: InternalIndex, value: Tagged<Object>);
    fn value_at_put_with_tag(&mut self, entry: InternalIndex, value: Tagged<Object>, tag: SeqCstAccessTag);
    fn value_at_swap(&mut self, entry: InternalIndex, value: Tagged<Object>, tag: SeqCstAccessTag) -> Tagged<Object>;
    fn value_at_compare_and_swap(&mut self, entry: InternalIndex, expected: Tagged<Object>, value: Tagged<Object>, tag: SeqCstAccessTag) -> Tagged<Object>;
    fn details_at(&self, entry: InternalIndex) -> PropertyDetails;
    fn details_at_put(&mut self, entry: InternalIndex, value: PropertyDetails);
    fn clear_entry(&mut self, entry: InternalIndex);
    fn set_entry(&mut self, entry: InternalIndex, key: Tagged<Object>, value: Tagged<Object>, details: PropertyDetails);
    fn raw_field_of_value_at(&self, entry: InternalIndex) -> ObjectSlot;
}

impl<Derived, Shape> DictionaryTrait<Derived, Shape> for Dictionary<Derived, Shape> {
    fn value_at(&self, entry: InternalIndex) -> Tagged<Object> {
        let cage_base = GetPtrComprCageBase();
        self.value_at_with_cage(cage_base, entry)
    }

    fn value_at_with_cage(&self, cage_base: PtrComprCageBase, entry: InternalIndex) -> Tagged<Object> {
        let index = DerivedHashTable::entry_to_index(entry) + Derived::kEntryValueIndex;
        self.get(index)
    }

    fn value_at_with_tag(&self, entry: InternalIndex, tag: SeqCstAccessTag) -> Tagged<Object> {
        let cage_base = GetPtrComprCageBase();
        self.value_at_with_cage_and_tag(cage_base, entry, tag)
    }

    fn value_at_with_cage_and_tag(&self, cage_base: PtrComprCageBase, entry: InternalIndex, tag: SeqCstAccessTag) -> Tagged<Object> {
        let index = DerivedHashTable::entry_to_index(entry) + Derived::kEntryValueIndex;
        self.get_with_tag(index, tag)
    }

    fn try_value_at(&self, entry: InternalIndex) -> Optional<Tagged<Object>> {
        if DerivedHashTable::entry_to_index(entry) + Derived::kEntryValueIndex >= self.length() {
            return Optional::None;
        }
        Optional::Some(self.value_at(entry))
    }

    fn value_at_put(&mut self, entry: InternalIndex, value: Tagged<Object>) {
        let index = DerivedHashTable::entry_to_index(entry) + Derived::kEntryValueIndex;
        self.set(index, value);
    }

    fn value_at_put_with_tag(&mut self, entry: InternalIndex, value: Tagged<Object>, tag: SeqCstAccessTag) {
        let index = DerivedHashTable::entry_to_index(entry) + Derived::kEntryValueIndex;
        self.set_with_tag(index, value, tag);
    }

    fn value_at_swap(&mut self, entry: InternalIndex, value: Tagged<Object>, tag: SeqCstAccessTag) -> Tagged<Object> {
        let index = DerivedHashTable::entry_to_index(entry) + Derived::kEntryValueIndex;
        self.swap(index, value, tag)
    }

    fn value_at_compare_and_swap(&mut self, entry: InternalIndex, expected: Tagged<Object>, value: Tagged<Object>, tag: SeqCstAccessTag) -> Tagged<Object> {
        let index = DerivedHashTable::entry_to_index(entry) + Derived::kEntryValueIndex;
        self.compare_and_swap(index, expected, value, tag)
    }

    fn details_at(&self, entry: InternalIndex) -> PropertyDetails {
        Shape::details_at(self.derived(), entry)
    }

    fn details_at_put(&mut self, entry: InternalIndex, value: PropertyDetails) {
        Shape::details_at_put(self.derived(), entry, value);
    }

    fn clear_entry(&mut self, entry: InternalIndex) {
        let the_hole = GetReadOnlyRoots().the_hole_value();
        let details = PropertyDetails::Empty();
        self.set_entry(entry, the_hole, the_hole, details);
    }

    fn set_entry(&mut self, entry: InternalIndex, key: Tagged<Object>, value: Tagged<Object>, details: PropertyDetails) {
        let index = DerivedHashTable::entry_to_index(entry);
        let mode = self.get_write_barrier_mode();
        self.set(index + Derived::kEntryKeyIndex, key);
        self.set(index + Derived::kEntryValueIndex, value);
        if Shape::kHasDetails {
            self.details_at_put(entry, details);
        }
    }

    fn raw_field_of_value_at(&self, entry: InternalIndex) -> ObjectSlot {
        self.raw_field_of_element_at(DerivedHashTable::entry_to_index(entry) + Derived::kEntryValueIndex)
    }
}

trait BaseNameDictionaryTrait<Derived, Shape> {
    fn set_next_enumeration_index(&mut self, index: i32);
    fn next_enumeration_index(&self) -> i32;
    fn set_hash(&mut self, hash: i32);
    fn hash(&self) -> i32;
}

impl<Derived, Shape> BaseNameDictionaryTrait<Derived, Shape> for BaseNameDictionary<Derived, Shape> {
    fn set_next_enumeration_index(&mut self, index: i32) {
        self.set(Self::kNextEnumerationIndexIndex, Smi::from_int(index));
    }

    fn next_enumeration_index(&self) -> i32 {
        Smi::to_int(self.get(Self::kNextEnumerationIndexIndex))
    }

    fn set_hash(&mut self, hash: i32) {
        self.set(Self::kObjectHashIndex, Smi::from_int(hash));
    }

    fn hash(&self) -> i32 {
        let hash_obj = self.get(Self::kObjectHashIndex);
        let hash = Smi::to_int(hash_obj);
        hash
    }
}

impl NumberDictionary {
    fn requires_slow_elements(&self) -> bool {
        let max_index_object = self.get(Self::kMaxNumberKeyIndex);
        if !is_smi(max_index_object) {
            return false;
        }
        0 != (Smi::to_int(max_index_object) & Self::kRequiresSlowElementsMask)
    }

    fn max_number_key(&self) -> u32 {
        if self.requires_slow_elements() {
            panic!("Requires slow elements");
        }
        let max_index_object = self.get(Self::kMaxNumberKeyIndex);
        if !is_smi(max_index_object) {
            return 0;
        }
        let value = Smi::to_int(max_index_object) as u32;
        value >> Self::kRequiresSlowElementsTagSize
    }

    fn set_requires_slow_elements(&mut self) {
        self.set(Self::kMaxNumberKeyIndex, Smi::from_int(Self::kRequiresSlowElementsMask));
    }
}

impl GlobalDictionary {
    fn set_entry(&mut self, entry: InternalIndex, key: Tagged<Object>, value: Tagged<Object>, details: PropertyDetails) {
        self.set(Self::entry_to_index(entry) + Self::kEntryKeyIndex, value);
        self.details_at_put(entry, details);
    }

    fn clear_entry(&mut self, entry: InternalIndex) {
        let the_hole = GetReadOnlyRoots().the_hole_value();
        self.set(Self::entry_to_index(entry) + Self::kEntryKeyIndex, the_hole);
    }

    fn value_at_put(&mut self, entry: InternalIndex, value: Tagged<Object>) {
        self.set(Self::entry_to_index(entry), value);
    }
}

trait BaseDictionaryShapeTrait<Key> {
    fn is_match(key: &DirectHandle<Key>, other: Tagged<Object>) -> bool;
    fn hash(roots: ReadOnlyRoots, key: &DirectHandle<Key>) -> u32;
    fn hash_for_object(roots: ReadOnlyRoots, other: Tagged<Object>) -> u32;
    fn as_handle<const ALLOCATION: AllocationType>(isolate: &Isolate, key: &DirectHandle<Key>) -> DirectHandle<Object>;
    fn as_handle_local<const ALLOCATION: AllocationType>(isolate: &LocalIsolate, key: &DirectHandle<Key>) -> DirectHandle<Object>;
}

impl NumberDictionaryBaseShape {
    fn is_match(key: u32, other: Tagged<Object>) -> bool {
        key == Object::number_value(other.unchecked_cast::<Number>()) as u32
    }

    fn hash(roots: ReadOnlyRoots, key: u32) -> u32 {
        compute_seeded_hash(key, HashSeed::new(roots))
    }

    fn hash_for_object(roots: ReadOnlyRoots, other: Tagged<Object>) -> u32 {
        compute_seeded_hash(
            Object::number_value(other.unchecked_cast::<Number>()) as u32,
            HashSeed::new(roots),
        )
    }

    fn as_handle<const ALLOCATION: AllocationType>(isolate: &Isolate, key: u32) -> DirectHandle<Object> {
        isolate.factory().new_number_from_uint::<ALLOCATION>(key)
    }

    fn as_handle_local<const ALLOCATION: AllocationType>(isolate: &LocalIsolate, key: u32) -> DirectHandle<Object> {
        isolate.factory().new_number_from_uint::<ALLOCATION>(key)
    }
}

impl BaseNameDictionaryShape {
    fn is_match(key: &DirectHandle<Name>, other: Tagged<Object>) -> bool {
        *key == other
    }

    fn hash(roots: ReadOnlyRoots, key: &DirectHandle<Name>) -> u32 {
        key.hash()
    }

    fn hash_for_object(roots: ReadOnlyRoots, other: Tagged<Object>) -> u32 {
        other.unchecked_cast::<Name>().hash()
    }

    fn as_handle<const ALLOCATION: AllocationType>(isolate: &Isolate, key: &DirectHandle<Name>) -> DirectHandle<Object> {
        key.clone()
    }

    fn as_handle_local<const ALLOCATION: AllocationType>(isolate: &LocalIsolate, key: &DirectHandle<Name>) -> DirectHandle<Object> {
        key.clone()
    }
}

impl GlobalDictionaryShape {
    fn is_match(key: &DirectHandle<Name>, other: Tagged<Object>) -> bool {
        *key == Cast::<PropertyCell>(other).name()
    }

    fn hash_for_object(roots: ReadOnlyRoots, other: Tagged<Object>) -> u32 {
        Cast::<PropertyCell>(other).name().hash()
    }
}

trait GlobalDictionaryShapeTrait<Dictionary> {
    fn details_at(dict: Tagged<Dictionary>, entry: InternalIndex) -> PropertyDetails;
    fn details_at_put(dict: Tagged<Dictionary>, entry: InternalIndex, value: PropertyDetails);
}

impl GlobalDictionaryShapeTrait<GlobalDictionary> for GlobalDictionaryShape {
    fn details_at(dict: Tagged<GlobalDictionary>, entry: InternalIndex) -> PropertyDetails {
        dict.cell_at(entry).property_details()
    }

    fn details_at_put(dict: Tagged<GlobalDictionary>, entry: InternalIndex, value: PropertyDetails) {
        dict.cell_at(entry).update_property_details_except_cell_type(value);
    }
}

impl NameDictionary {
    fn name_at(&self, entry: InternalIndex) -> Tagged<Name> {
        let cage_base = GetPtrComprCageBase();
        self.name_at_with_cage(cage_base, entry)
    }

    fn name_at_with_cage(&self, cage_base: PtrComprCageBase, entry: InternalIndex) -> Tagged<Name> {
        Cast::<Name>(self.key_at_with_cage(cage_base, entry))
    }

    fn flags(&self) -> u32 {
        Smi::to_int(self.get(Self::kFlagsIndex)) as u32
    }

    fn set_flags(&mut self, flags: u32) {
        self.set(Self::kFlagsIndex, Smi::from_int(flags as i32));
    }

    define_bit_field_accessors!(NameDictionary, flags, may_have_interesting_properties, NameDictionary::MayHaveInterestingPropertiesBit);
}

impl GlobalDictionary {
    fn cell_at(&self, entry: InternalIndex) -> Tagged<PropertyCell> {
        let cage_base = GetPtrComprCageBase();
        self.cell_at_with_cage(cage_base, entry)
    }

    fn cell_at_with_cage(&self, cage_base: PtrComprCageBase, entry: InternalIndex) -> Tagged<PropertyCell> {
        Cast::<PropertyCell>(self.key_at_with_cage(cage_base, entry))
    }

    fn name_at(&self, entry: InternalIndex) -> Tagged<Name> {
        let cage_base = GetPtrComprCageBase();
        self.name_at_with_cage(cage_base, entry)
    }

    fn name_at_with_cage(&self, cage_base: PtrComprCageBase, entry: InternalIndex) -> Tagged<Name> {
        self.cell_at_with_cage(cage_base, entry).name_with_cage(cage_base)
    }

    fn value_at(&self, entry: InternalIndex) -> Tagged<Object> {
        let cage_base = GetPtrComprCageBase();
        self.value_at_with_cage(cage_base, entry)
    }

    fn value_at_with_cage(&self, cage_base: PtrComprCageBase, entry: InternalIndex) -> Tagged<Object> {
        self.cell_at_with_cage(cage_base, entry).value_with_cage(cage_base)
    }
}

impl<Key> BaseDictionaryShape<Key> {
    fn details_at<Dictionary: HashTableTrait>(dict: Tagged<Dictionary>, entry: InternalIndex) -> PropertyDetails {
        let details = dict.get(Dictionary::entry_to_index(entry) + Dictionary::kEntryDetailsIndex);
        PropertyDetails::new(Cast::<Smi>(details))
    }

    fn details_at_put<Dictionary: HashTableTrait>(&mut self, dict: Tagged<Dictionary>, entry: InternalIndex, value: PropertyDetails) {
        let details = value.as_smi();
        dict.set(Dictionary::entry_to_index(entry) + Dictionary::kEntryDetailsIndex, details);
    }
}

trait HashTableTrait {
    fn get(&self, index: usize) -> Tagged<Object>;
    fn set(&mut self, index: usize, value: Tagged<Object>);
    fn kEntryDetailsIndex() -> usize;
    fn entry_to_index(entry: InternalIndex) -> usize;
}
