// Converted from V8 C++ source files:
// Header: ephemeron-remembered-set.h
// Implementation: ephemeron-remembered-set.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use crate::Address;
use crate::InternalIndex;
use crate::EphemeronHashTable;

mod base {
    pub mod worklist {
        pub struct Worklist<T, const N: usize> {
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T, const N: usize> Worklist<T, N> {
            pub fn new() -> Self {
                Worklist {
                    _phantom: std::marker::PhantomData,
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Tagged<T> {
    _phantom: std::marker::PhantomData<T>,
    address: usize,
}

impl<T> Tagged<T> {
    pub fn address(&self) -> usize {
        self.address
    }
}

impl<T> From<usize> for Tagged<T> {
    fn from(address: usize) -> Self {
        Tagged {
            _phantom: std::marker::PhantomData,
            address,
        }
    }
}

pub struct HeapObjectSlot(Address);

impl HeapObjectSlot {
    pub fn ToHeapObject(&self) -> Tagged<HeapObject> {
        Tagged::from(self.0)
    }
}

pub type Address = usize;

pub struct HeapLayout {}

impl HeapLayout {
    pub fn InYoungGeneration(_object: Tagged<HeapObject>) -> bool {
        true
    }
}

pub struct HeapObject {}

impl HeapObject {
}

impl EphemeronHashTable {
    pub fn SlotToIndex(_table_address: Address, slot: Address) -> i32 {
        slot as i32
    }
    pub fn IndexToEntry(index: i32) -> InternalIndex {
        InternalIndex {}
    }
}

pub struct Object {}

impl Object {
    pub struct Hasher;
}

impl Object::Hasher {
    pub fn new() -> Self {
        Object::Hasher
    }
}

impl std::hash::Hasher for Object::Hasher {
    fn finish(&self) -> u64 {
        0
    }

    fn write(&mut self, _bytes: &[u8]) {}
}

impl std::cmp::PartialEq for Object::Hasher {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl std::cmp::Eq for Object::Hasher {}

impl Default for Object::Hasher {
    fn default() -> Self {
        Object::Hasher::new()
    }
}

pub mod internal {
    use std::collections::{HashMap, HashSet};
    use std::sync::Mutex;
    use crate::{Address, EphemeronHashTable, Tagged, HeapLayout, HeapObjectSlot, InternalIndex};

    pub struct EphemeronRememberedSet {
        insertion_mutex_: Mutex<()>,
        tables_: HashMap<Tagged<EphemeronHashTable>, IndicesSet>,
    }

    impl EphemeronRememberedSet {
        pub const kEphemeronTableListSegmentSize: i32 = 128;
        pub type TableList = base::worklist::Worklist<Tagged<EphemeronHashTable>, {Self::kEphemeronTableListSegmentSize as usize}>;
        pub type IndicesSet = HashSet<i32>;
        pub type TableMap = HashMap<Tagged<EphemeronHashTable>, IndicesSet>;

        pub fn new() -> Self {
            EphemeronRememberedSet {
                insertion_mutex_: Mutex::new(()),
                tables_: HashMap::new(),
            }
        }

        #[allow(non_snake_case)]
        pub fn RecordEphemeronKeyWrite(&mut self, table: Tagged<EphemeronHashTable>, slot: Address) {
            if HeapLayout::InYoungGeneration(HeapObjectSlot(slot).ToHeapObject()) {
                let slot_index = EphemeronHashTable::SlotToIndex(table.address(), slot);
                let entry = EphemeronHashTable::IndexToEntry(slot_index);

                let _guard = self.insertion_mutex_.lock().unwrap();
                let entry_value = entry.as_int();
                self.tables_.entry(table).or_insert_with(HashSet::new).insert(entry_value);
            }
        }

        #[allow(non_snake_case)]
        pub fn RecordEphemeronKeyWrites(&mut self, table: Tagged<EphemeronHashTable>, indices: IndicesSet) {
            let _guard = self.insertion_mutex_.lock().unwrap();
            if let Some(existing_indices) = self.tables_.get_mut(&table) {
                existing_indices.extend(indices);
            } else {
                self.tables_.insert(table, indices);
            }
        }

        pub fn tables(&mut self) -> &mut TableMap {
            &mut self.tables_
        }
    }
}

trait AsInt {
    fn as_int(&self) -> i32;
}

impl AsInt for InternalIndex {
    fn as_int(&self) -> i32 {
        0
    }
}
