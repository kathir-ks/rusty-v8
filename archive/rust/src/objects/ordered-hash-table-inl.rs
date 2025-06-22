// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/objects/ordered-hash-table-inl.h

mod ordered_hash_table {
    use crate::heap::Heap;
    use crate::objects::fixed_array::FixedArray;
    use crate::objects::js_collection_iterator::JsCollectionIterator;
    use crate::objects::object::Object;
    use crate::objects::property_details::PropertyDetails;
    use crate::objects::slots::Slot;
    use crate::roots::ReadOnlyRoots;
    use crate::roots::RootsTable;
    use crate::tagged::{Tagged, TaggedPtr};
    use crate::objects::name::Name;
    use std::marker::PhantomData;

    // Placeholder for torque-generated code
    // include "torque-generated/src/objects/ordered-hash-table-tq-inl.inc"

    pub trait OrderedHashTableTrait<T> {
        fn is_key(&self, roots: &ReadOnlyRoots, k: Tagged<Object>) -> bool;
        fn key_at(&self, entry: InternalIndex) -> Tagged<Object>;
        fn data_entry(&self, entry: usize, relative_index: usize) -> Tagged<Object>;
        fn to_key(&self, roots: &ReadOnlyRoots, entry: InternalIndex, out_key: &mut Tagged<Object>) -> bool;
    }

    pub struct OrderedHashTable<Derived, const ENTRY_SIZE: usize> {
        // Fields specific to OrderedHashTable if any
        _phantom: PhantomData<Derived>,
    }

    impl<Derived, const ENTRY_SIZE: usize> OrderedHashTable<Derived, ENTRY_SIZE> {
        pub fn new() -> Self {
            OrderedHashTable {
                _phantom: PhantomData,
            }
        }
    }

    impl<Derived, const ENTRY_SIZE: usize> OrderedHashTableTrait<Derived> for OrderedHashTable<Derived, ENTRY_SIZE> {
        fn is_key(&self, roots: &ReadOnlyRoots, k: Tagged<Object>) -> bool {
            k != roots.the_hole_value()
        }

        fn key_at(&self, _entry: InternalIndex) -> Tagged<Object> {
            unimplemented!("key_at")
        }

        fn data_entry(&self, _entry: usize, _relative_index: usize) -> Tagged<Object> {
            unimplemented!("data_entry")
        }

        fn to_key(&self, roots: &ReadOnlyRoots, entry: InternalIndex, out_key: &mut Tagged<Object>) -> bool {
            let k = self.key_at(entry);
            if !self.is_key(roots, k) {
                return false;
            }
            *out_key = k;
            true
        }
    }

    pub struct SmallOrderedHashTable<Derived> {
        ptr: TaggedPtr<Object>, // Representing HeapObject(ptr)
        _phantom: PhantomData<Derived>,
    }

    impl<Derived> SmallOrderedHashTable<Derived> {
        pub fn new(ptr: TaggedPtr<Object>) -> Self {
            SmallOrderedHashTable {
                ptr,
                _phantom: PhantomData,
            }
        }

        pub fn key_at(&self, entry: InternalIndex) -> Tagged<Object> {
            //DCHECK_LT(entry.as_int(), Capacity());
            //Offset entry_offset = GetDataEntryOffset(entry.as_int(), Derived::kKeyIndex);
            //return TaggedField<Object>::load(*this, entry_offset);
            unimplemented!()
        }

         pub fn data_entry(&self, entry: usize, relative_index: usize) -> Tagged<Object> {
            //DCHECK_LT(entry, Capacity());
            //DCHECK_LE(static_cast<unsigned>(relative_index), Derived::kEntrySize);
            //Offset entry_offset = GetDataEntryOffset(entry, relative_index);
            //return TaggedField<Object>::load(*this, entry_offset);
            unimplemented!()
        }

         pub fn set_data_entry(&self, entry: usize, relative_index: usize, value: Tagged<Object>) {
            //DCHECK_NE(kNotFound, entry);
            //int entry_offset = GetDataEntryOffset(entry, relative_index);
            //RELAXED_WRITE_FIELD(*this, entry_offset, value);
            //WRITE_BARRIER(*this, entry_offset, value);
            unimplemented!()
        }
    }

    //OBJECT_CONSTRUCTORS_IMPL(SmallOrderedHashSet, SmallOrderedHashTable<SmallOrderedHashSet>)
    //OBJECT_CONSTRUCTORS_IMPL(SmallOrderedHashMap, SmallOrderedHashTable<SmallOrderedHashMap>)
    //OBJECT_CONSTRUCTORS_IMPL(SmallOrderedNameDictionary, SmallOrderedHashTable<SmallOrderedNameDictionary>)

    // Implementations for specific types
    pub struct SmallOrderedHashSet {
        table: SmallOrderedHashTable<SmallOrderedHashSet>,
    }
    impl SmallOrderedHashSet {
        pub fn get_map(roots: &RootsTable) -> TaggedPtr<Object> {
            roots.small_ordered_hash_set_map()
        }
        pub fn is(table: TaggedPtr<Object>) -> bool {
            //IsSmallOrderedHashSet(*table)
            unimplemented!()
        }
    }

    pub struct SmallOrderedHashMap {
        table: SmallOrderedHashTable<SmallOrderedHashMap>,
    }
    impl SmallOrderedHashMap {
        pub fn get_map(roots: &RootsTable) -> TaggedPtr<Object> {
            roots.small_ordered_hash_map_map()
        }
        pub fn is(table: TaggedPtr<Object>) -> bool {
            //IsSmallOrderedHashMap(*table);
            unimplemented!()
        }
    }

    pub struct SmallOrderedNameDictionary {
        table: SmallOrderedHashTable<SmallOrderedNameDictionary>,
    }
    impl SmallOrderedNameDictionary {
         pub fn get_map(roots: &RootsTable) -> TaggedPtr<Object> {
            roots.small_ordered_name_dictionary_map()
        }
        pub fn is(table: TaggedPtr<Object>) -> bool {
            //IsSmallOrderedNameDictionary(*table);
            unimplemented!()
        }

        // Returns the property details for the property at entry.
        pub fn details_at(&self, entry: InternalIndex) -> PropertyDetails {
            // TODO(gsathya): Optimize the cast away. And store this in the data table.
            //return PropertyDetails(
            //    Cast<Smi>(this->GetDataEntry(entry.as_int(), kPropertyDetailsIndex)));
            unimplemented!()
        }

        // Set the details for entry.
        pub fn details_at_put(&self, entry: InternalIndex, value: PropertyDetails) {
            // TODO(gsathya): Optimize the cast away. And store this in the data table.
            //this->SetDataEntry(entry.as_int(), kPropertyDetailsIndex, value.AsSmi());
            unimplemented!()
        }

        pub fn value_at(&self, entry: InternalIndex) -> Tagged<Object> {
            //return this->GetDataEntry(entry.as_int(), kValueIndex);
            unimplemented!()
        }

        // Set the value for entry.
        pub fn value_at_put(&self, entry: InternalIndex, value: Tagged<Object>) {
            //this->SetDataEntry(entry.as_int(), kValueIndex, value);
            unimplemented!()
        }

         pub fn set_hash(&self, hash: i32) {
            //DCHECK(PropertyArray::HashField::is_valid(hash));
            //WriteField<int>(PrefixOffset(), hash);
            unimplemented!()
        }

        pub fn hash(&self) -> i32 {
            //int hash = ReadField<int>(PrefixOffset());
            //DCHECK(PropertyArray::HashField::is_valid(hash));
            //return hash;
            unimplemented!()
        }
    }


    pub struct OrderedHashSet {}
    impl OrderedHashSet {
        pub fn get_map(roots: &mut RootsTable) -> TaggedPtr<Object> {
            roots.ordered_hash_set_map()
        }
        pub fn is(table: TaggedPtr<Object>) -> bool {
            //IsOrderedHashSet(*table);
            unimplemented!()
        }
    }

    pub struct OrderedHashMap {}
    impl OrderedHashMap {
        pub fn get_map(roots: &mut RootsTable) -> TaggedPtr<Object> {
            roots.ordered_hash_map_map()
        }
        pub fn is(table: TaggedPtr<Object>) -> bool {
            //IsOrderedHashMap(*table);
            unimplemented!()
        }

        pub fn value_at(&self, entry: InternalIndex) -> Tagged<Object> {
            //DCHECK_LT(entry.as_int(), UsedCapacity());
            //return get(EntryToIndex(entry) + kValueOffset);
            unimplemented!()
        }
    }

    pub struct OrderedNameDictionary {}
    impl OrderedNameDictionary {
        pub fn get_map(roots: &mut RootsTable) -> TaggedPtr<Object> {
            roots.ordered_name_dictionary_map()
        }
        pub fn is(table: TaggedPtr<Object>) -> bool {
            //IsOrderedNameDictionary(*table);
            unimplemented!()
        }

        pub fn value_at(&self, entry: InternalIndex) -> Tagged<Object> {
            //DCHECK_LT(entry.as_int(), UsedCapacity());
            //return get(EntryToIndex(entry) + kValueOffset);
            unimplemented!()
        }

        pub fn name_at(&self, entry: InternalIndex) -> Tagged<Name> {
            //return Cast<Name>(KeyAt(entry));
            unimplemented!()
        }

        // Returns the property details for the property at entry.
        pub fn details_at(&self, entry: InternalIndex) -> PropertyDetails {
            //DCHECK_LT(entry.as_int(), this->UsedCapacity());
            //// TODO(gsathya): Optimize the cast away.
            //return PropertyDetails(
            //    Cast<Smi>(get(EntryToIndex(entry) + kPropertyDetailsOffset)));
            unimplemented!()
        }

        pub fn details_at_put(&self, entry: InternalIndex, value: PropertyDetails) {
            //DCHECK_LT(entry.as_int(), this->UsedCapacity());
            //// TODO(gsathya): Optimize the cast away.
            //this->set(EntryToIndex(entry) + kPropertyDetailsOffset, value.AsSmi());
            unimplemented!()
        }

        // Set the value for entry.
        pub fn value_at_put(&self, entry: InternalIndex, value: Tagged<Object>) {
            //DCHECK_LT(entry.as_int(), UsedCapacity());
            //this->set(EntryToIndex(entry) + kValueOffset, value);
            unimplemented!()
        }

         pub fn set_hash(&self, hash: i32) {
            //DCHECK(PropertyArray::HashField::is_valid(hash));
            //this->set(HashIndex(), Smi::FromInt(hash));
            unimplemented!()
        }

        pub fn hash(&self) -> i32 {
            //Tagged<Object> hash_obj = this->get(HashIndex());
            //int hash = Smi::ToInt(hash_obj);
            //DCHECK(PropertyArray::HashField::is_valid(hash));
            //return hash;
            unimplemented!()
        }
    }

    pub struct OrderedHashTableIterator<Derived, TableType> {
        table: Tagged<Object>,
        index: Tagged<Object>,
        _phantom: PhantomData<(Derived, TableType)>,
    }

    impl<Derived, TableType> OrderedHashTableIterator<Derived, TableType> {
        pub fn current_key(&self) -> Tagged<Object> {
            //Tagged<TableType> table = Cast<TableType>(this->table());
            //int index = Smi::ToInt(this->index());
            //DCHECK_LE(0, index);
            //InternalIndex entry(index);
            //Tagged<Object> key = table->KeyAt(entry);
            //DCHECK(!IsHashTableHole(key));
            //return key;
            unimplemented!()
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct InternalIndex(pub usize);

    impl InternalIndex {
        pub fn as_int(&self) -> usize {
            self.0
        }
    }

}

mod heap {
    pub struct Heap {}
}

mod objects {
    pub mod fixed_array {
        pub struct FixedArray {}
    }

    pub mod js_collection_iterator {
        pub struct JsCollectionIterator {}
    }

    pub mod object {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
        pub struct Object {}
    }
    
    pub mod name {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
        pub struct Name {}
    }

    pub mod property_details {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
        pub struct PropertyDetails(pub i32);

        impl PropertyDetails {
            pub fn as_smi(&self) -> i32 {
                self.0
            }
        }
    }

    pub mod slots {
        pub struct Slot {}
    }
}

mod roots {
    use crate::objects::object::Object;
    use crate::tagged::TaggedPtr;

    pub struct ReadOnlyRoots {
        the_hole_value: crate::tagged::Tagged<crate::objects::object::Object>,
    }

    impl ReadOnlyRoots {
        pub fn the_hole_value(&self) -> crate::tagged::Tagged<crate::objects::object::Object> {
            self.the_hole_value
        }
    }

    pub struct RootsTable {}

    impl RootsTable {
        pub fn ordered_hash_set_map(&mut self) -> TaggedPtr<Object> {
            unimplemented!()
        }
        pub fn ordered_hash_map_map(&mut self) -> TaggedPtr<Object> {
            unimplemented!()
        }
        pub fn ordered_name_dictionary_map(&mut self) -> TaggedPtr<Object> {
            unimplemented!()
        }
        pub fn small_ordered_name_dictionary_map(&mut self) -> TaggedPtr<Object> {
            unimplemented!()
        }
        pub fn small_ordered_hash_map_map(&mut self) -> TaggedPtr<Object> {
            unimplemented!()
        }
        pub fn small_ordered_hash_set_map(&mut self) -> TaggedPtr<Object> {
            unimplemented!()
        }
    }
}

mod tagged {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Tagged<T> {
        _phantom: std::marker::PhantomData<T>,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct TaggedPtr<T> {
        _phantom: std::marker::PhantomData<T>,
    }
}