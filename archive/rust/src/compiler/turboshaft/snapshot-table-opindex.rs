// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;

mod index {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct OpIndex(pub usize); // Assuming OpIndex is just a usize
}

mod snapshot_table {
    use super::*;

    pub trait KeyDataTrait: Default + Clone {}
    impl KeyDataTrait for () {}

    pub struct SnapshotTable<Value, KeyData = ()>
    where
        Value: Default + Clone,
        KeyData: KeyDataTrait,
    {
        table: Vec<Value>,
        key_data: Vec<KeyData>,
        phantom: PhantomData<Value>,
    }

    impl<Value, KeyData> SnapshotTable<Value, KeyData>
    where
        Value: Default + Clone,
        KeyData: KeyDataTrait,
    {
        pub fn new() -> Self {
            SnapshotTable {
                table: Vec::new(),
                key_data: Vec::new(),
                phantom: PhantomData,
            }
        }

        pub type Key = usize; // Assuming Key is an index into the table

        pub fn get(&self, key: Self::Key) -> Value
        where
            Value: Default + Clone,
        {
            self.table.get(key).cloned().unwrap_or_default()
        }

        pub fn get_predecessor_value(&self, key: Self::Key, _predecessor_index: usize) -> Value
        where
            Value: Default + Clone,
        {
            self.get(key) // Simplified: no actual predecessor logic
        }

        pub fn set(&mut self, key: Self::Key, new_value: Value) -> bool {
            if key < self.table.len() {
                self.table[key] = new_value;
                true
            } else {
                false // Or panic, or grow the vector if that's the intent
            }
        }

        pub fn new_key(&mut self) -> Self::Key {
            let key = self.table.len();
            self.table.push(Value::default());
            self.key_data.push(KeyData::default());
            key
        }

        pub fn new_key_with_data(&mut self, data: KeyData, initial_value: Value) -> Self::Key {
            let key = self.table.len();
            self.table.push(initial_value);
            self.key_data.push(data);
            key
        }
    }
}

pub mod turboshaft {
    use super::*;
    use index::OpIndex;
    use snapshot_table::SnapshotTable;

    pub struct SparseOpIndexSnapshotTable<Value, KeyData = ()>
    where
        Value: Default + Clone,
        KeyData: snapshot_table::KeyDataTrait,
    {
        base: SnapshotTable<Value, KeyData>,
        indices_to_keys: HashMap<OpIndex, usize>, // Using usize for Key
    }

    impl<Value, KeyData> SparseOpIndexSnapshotTable<Value, KeyData>
    where
        Value: Default + Clone,
        KeyData: snapshot_table::KeyDataTrait,
    {
        pub fn new() -> Self {
            SparseOpIndexSnapshotTable {
                base: SnapshotTable::new(),
                indices_to_keys: HashMap::new(),
            }
        }

        pub type Key = usize; // Key alias

        pub fn get(&self, idx: OpIndex) -> Value {
            self.indices_to_keys
                .get(&idx)
                .map(|&key| self.base.get(key))
                .unwrap_or_default()
        }

        pub fn get_predecessor_value(&self, idx: OpIndex, predecessor_index: usize) -> Value {
            self.indices_to_keys
                .get(&idx)
                .map(|&key| self.base.get_predecessor_value(key, predecessor_index))
                .unwrap_or_default()
        }

        pub fn set(&mut self, idx: OpIndex, new_value: Value) -> bool {
            let key = self.get_or_create_key(idx);
            self.base.set(key, new_value)
        }

        pub fn new_key(&mut self, idx: OpIndex, data: KeyData, initial_value: Value) {
            if self.indices_to_keys.contains_key(&idx) {
                panic!("Key already exists for OpIndex {:?}", idx); //DCHECK
            }
            let key = self.base.new_key_with_data(data, initial_value);
            self.indices_to_keys.insert(idx, key);
        }

        pub fn new_key_simple(&mut self, idx: OpIndex, initial_value: Value)
        where
            KeyData: Default + Clone,
        {
            self.new_key(idx, KeyData::default(), initial_value);
        }

        pub fn has_key_for(&self, idx: OpIndex) -> bool {
            self.indices_to_keys.contains_key(&idx)
        }

        pub fn try_get_key_for(&self, idx: OpIndex) -> Option<Key> {
            self.indices_to_keys.get(&idx).copied()
        }

        fn get_or_create_key(&mut self, idx: OpIndex) -> Key {
            if let Some(&key) = self.indices_to_keys.get(&idx) {
                key
            } else {
                let key = self.base.new_key();
                self.indices_to_keys.insert(idx, key);
                key
            }
        }
    }
}