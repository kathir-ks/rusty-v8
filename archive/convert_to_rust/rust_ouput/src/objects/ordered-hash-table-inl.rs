// Converted from V8 C++ source files:
// Header: ordered-hash-table-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::marker::PhantomData;

use crate::heap::heap::Heap;
use crate::objects::fixed_array::FixedArray;
use crate::objects::js_collection_iterator::JSCollectionIterator;
use crate::objects::lookup_inl::V8;
use crate::objects::object::HeapObject;
use crate::objects::object::Object;
use crate::objects::slots::Address;
use crate::objects::slots::CppHeapPointerHandle;
use crate::objects::slots::CppHeapPointerTag;
use crate::objects::slots::TaggedField;
use crate::objects::slots::Tagged;

use crate::objects::map::Map;
use crate::objects::name::Name;
use crate::objects::property_descriptor::InternalIndex;
use crate::objects::property_descriptor::PropertyDetails;
use crate::objects::property_array::PropertyArray;
use crate::roots::roots::ReadOnlyRoots;
use crate::roots::roots::RootsTable;
use crate::torque_generated::src::objects::ordered_hash_table_tq_inl::IsHashTableHole;
use crate::handles::handles::DirectHandle;
use crate::objects::module::Module;
use crate::objects::smi::Smi;
use crate::codegen::reglist::RegisterT;

// torque-generated/src/objects/ordered-hash-table-tq-inl.inc should be handled by torque

trait OrderedHashTableMethods<Derived, const ENTRYSIZE: usize> {
    fn is_key(&self, roots: &ReadOnlyRoots, k: Tagged<Object>) -> bool;
    fn to_key(&self, roots: ReadOnlyRoots, entry: InternalIndex, out_key: &mut Tagged<Object>) -> bool;
}

struct OrderedHashTable<Derived, const ENTRYSIZE: usize> {
    ptr: Address,
    _derived: PhantomData<Derived>,
}

impl<Derived, const ENTRYSIZE: usize> OrderedHashTable<Derived, const ENTRYSIZE> {
    fn key_at(&self, entry: InternalIndex) -> Tagged<Object> {
        todo!()
    }
}

impl<Derived, const ENTRYSIZE: usize> OrderedHashTableMethods<Derived, const ENTRYSIZE> for OrderedHashTable<Derived, const ENTRYSIZE> {
    fn is_key(&self, roots: &ReadOnlyRoots, k: Tagged<Object>) -> bool {
        k != roots.the_hole_value()
    }

    fn to_key(&self, roots: ReadOnlyRoots, entry: InternalIndex, out_key: &mut Tagged<Object>) -> bool {
        let k = self.key_at(entry);
        if !self.is_key(&roots, k) {
            return false;
        }
        *out_key = k;
        true
    }
}

struct SmallOrderedHashTable<Derived> {
    ptr: Address,
    _derived: PhantomData<Derived>,
}

impl<Derived> SmallOrderedHashTable<Derived> {
    fn new(ptr: Address) -> Self {
        SmallOrderedHashTable { ptr, _derived: PhantomData }
    }

    fn key_at(&self, entry: InternalIndex) -> Tagged<Object> {
        let capacity = self.capacity();
        assert!(entry.as_int() < capacity);
        let entry_offset = self.get_data_entry_offset(entry.as_int(), Derived::k_key_index());
        TaggedField::<Object>::load_from_instance(self.ptr, entry_offset)
    }

    fn get_data_entry(&self, entry: i32, relative_index: i32) -> Tagged<Object> {
        let capacity = self.capacity();
        assert!(entry < capacity);
        assert!(relative_index as u32 <= Derived::k_entry_size() as u32);
        let entry_offset = self.get_data_entry_offset(entry, relative_index);
        TaggedField::<Object>::load_from_instance(self.ptr, entry_offset)
    }

    fn capacity(&self) -> i32 {
        todo!()
    }

    fn get_data_entry_offset(&self, entry: i32, relative_index: i32) -> usize {
        todo!()
    }

    fn set_data_entry(&self, entry: i32, relative_index: i32, value: Tagged<Object>) {
        let entry_offset = self.get_data_entry_offset(entry, relative_index) as usize;
        unsafe {
            (self.ptr as *mut Tagged<Object>).add(entry_offset).write(value);
        }
    }
}

trait SmallOrderedHashTableMethods<Derived> {
    const k_key_index: i32;
    const k_entry_size: i32;
}

struct SmallOrderedHashSet {
    table: SmallOrderedHashTable<SmallOrderedHashSet>,
}

impl SmallOrderedHashTableMethods<SmallOrderedHashSet> for SmallOrderedHashSet {
    const k_key_index: i32 = 0;
    const k_entry_size: i32 = 1;
}

impl SmallOrderedHashSet {
    fn get_map(roots: &RootsTable) -> DirectHandle<Map> {
        roots.small_ordered_hash_set_map()
    }

    fn is(table: DirectHandle<HeapObject>) -> bool {
        todo!()
    }
}

struct SmallOrderedHashMap {
    table: SmallOrderedHashTable<SmallOrderedHashMap>,
}

impl SmallOrderedHashTableMethods<SmallOrderedHashMap> for SmallOrderedHashMap {
    const k_key_index: i32 = 0;
    const k_entry_size: i32 = 2;
}

impl SmallOrderedHashMap {
     fn get_map(roots: &RootsTable) -> DirectHandle<Map> {
        roots.small_ordered_hash_map_map()
    }

    fn is(table: DirectHandle<HeapObject>) -> bool {
        todo!()
    }
}

struct SmallOrderedNameDictionary {
    table: SmallOrderedHashTable<SmallOrderedNameDictionary>,
}

impl SmallOrderedHashTableMethods<SmallOrderedNameDictionary> for SmallOrderedNameDictionary {
    const k_key_index: i32 = 0;
    const k_entry_size: i32 = 3;
}

impl SmallOrderedNameDictionary {
    const k_value_index: i32 = 1;
    const k_property_details_index: i32 = 2;

    fn get_map(roots: &RootsTable) -> DirectHandle<Map> {
        roots.small_ordered_name_dictionary_map()
    }

    fn value_at(&self, entry: InternalIndex) -> Tagged<Object> {
        self.table.get_data_entry(entry.as_int(), Self::k_value_index)
    }

    fn value_at_put(&self, entry: InternalIndex, value: Tagged<Object>) {
        self.table.set_data_entry(entry.as_int(), Self::k_value_index, value);
    }

    fn details_at(&self, entry: InternalIndex) -> PropertyDetails {
        PropertyDetails(Smi::from_int(self.table.get_data_entry(entry.as_int(), Self::k_property_details_index).ptr as i32))
    }

    fn details_at_put(&self, entry: InternalIndex, value: PropertyDetails) {
        self.table.set_data_entry(entry.as_int(), Self::k_property_details_index, Tagged{ptr: value.as_smi().ptr});
    }

    fn is(table: DirectHandle<HeapObject>) -> bool {
        todo!()
    }

    fn set_hash(&self, hash: i32) {
        todo!()
    }

    fn hash(&self) -> i32 {
        todo!()
    }
}

struct OrderedHashSet {
    table: OrderedHashTable<OrderedHashSet, 1>,
}

impl OrderedHashSet {
     fn get_map(roots: &RootsTable) -> Handle<Map> {
        roots.ordered_hash_set_map()
    }

    fn is(table: DirectHandle<HeapObject>) -> bool {
        todo!()
    }
}

struct OrderedHashMap {
    table: OrderedHashTable<OrderedHashMap, 2>,
}

impl OrderedHashMap {
    fn get_map(roots: &RootsTable) -> Handle<Map> {
        roots.ordered_hash_map_map()
    }

    fn value_at(&self, entry: InternalIndex) -> Tagged<Object> {
        todo!()
    }

    fn is(table: DirectHandle<HeapObject>) -> bool {
        todo!()
    }
}

struct OrderedNameDictionary {
    table: OrderedHashTable<OrderedNameDictionary, 3>,
}

impl OrderedNameDictionary {
     fn get_map(roots: &RootsTable) -> Handle<Map> {
        roots.ordered_name_dictionary_map()
    }

    fn value_at(&self, entry: InternalIndex) -> Tagged<Object> {
        todo!()
    }

    fn name_at(&self, entry: InternalIndex) -> Tagged<Name> {
        todo!()
    }

    fn value_at_put(&self, entry: InternalIndex, value: Tagged<Object>) {
        todo!()
    }

    fn details_at(&self, entry: InternalIndex) -> PropertyDetails {
        todo!()
    }

    fn details_at_put(&self, entry: InternalIndex, value: PropertyDetails) {
        todo!()
    }

    fn is(table: DirectHandle<HeapObject>) -> bool {
        todo!()
    }

    const k_value_offset: i32 = 1;
    const k_property_details_offset: i32 = 2;

    fn entry_to_index(&self, entry: InternalIndex) -> i32 {
        todo!()
    }

    fn used_capacity(&self) -> i32 {
        todo!()
    }

    fn get(&self, index: i32) -> Tagged<Object> {
        todo!()
    }

    fn set(&self, index: i32, value: Tagged<Object>) {
        todo!()
    }

    const k_hash_index: i32 = 0;

    fn hash_index(&self) -> i32 {
        todo!()
    }

    fn set_hash(&self, hash: i32) {
        todo!()
    }

    fn hash(&self) -> i32 {
        todo!()
    }
}

struct OrderedHashTableIterator<Derived, TableType> {
    table_ptr: Address,
}

impl<Derived, TableType> OrderedHashTableIterator<Derived, TableType> {
    fn current_key(&self) -> Tagged<Object> {
        todo!()
    }

    fn table(&self) -> Address {
        todo!()
    }

    fn index(&self) -> Tagged<Object> {
        todo!()
    }
}
