// Converted from V8 C++ source files:
// Header: snapshot-table-opindex.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

use std::collections::HashMap;
use std::hash::Hash;
use std::option::Option;

pub struct SnapshotTable<Value, KeyData> {
    keys: Vec<Key>,
    values: Vec<Value>,
    key_data: Vec<KeyData>,
}

impl<Value, KeyData> SnapshotTable<Value, KeyData>
where
    Value: Default + Clone,
    KeyData: Default + Clone,
{
    pub fn new() -> Self {
        SnapshotTable {
            keys: Vec::new(),
            values: Vec::new(),
            key_data: Vec::new(),
        }
    }

    pub fn get(&self, key: Key) -> Value {
        if key.index < self.values.len() {
            self.values[key.index].clone()
        } else {
            Value::default()
        }
    }

    pub fn get_predecessor_value(&self, key: Key, _predecessor_index: usize) -> Value {
        self.get(key)
    }

    pub fn set(&mut self, key: Key, new_value: Value) -> bool {
        if key.index < self.values.len() {
            self.values[key.index] = new_value;
            true
        } else {
            false
        }
    }

    pub fn new_key(&mut self, data: KeyData, initial_value: Value) -> Key {
        let key = Key {
            index: self.keys.len(),
        };
        self.keys.push(key);
        self.values.push(initial_value);
        self.key_data.push(data);
        key
    }

    pub fn new_key_default(&mut self) -> Key {
        let key = Key {
            index: self.keys.len(),
        };
        self.keys.push(key);
        self.values.push(Value::default());
        self.key_data.push(KeyData::default());
        key
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Key {
    index: usize,
}

pub struct SparseOpIndexSnapshotTable<Value, KeyData = NoKeyData>
where
    Value: Default + Clone,
    KeyData: Default + Clone,
{
    base: SnapshotTable<Value, KeyData>,
    indices_to_keys_: HashMap<OpIndex, Key>,
}

impl<Value, KeyData> SparseOpIndexSnapshotTable<Value, KeyData>
where
    Value: Default + Clone,
    KeyData: Default + Clone,
{
    pub fn new() -> Self {
        SparseOpIndexSnapshotTable {
            base: SnapshotTable::new(),
            indices_to_keys_: HashMap::new(),
        }
    }

    pub fn get(&self, idx: OpIndex) -> Value {
        match self.indices_to_keys_.get(&idx) {
            Some(key) => self.base.get(*key),
            None => Value::default(),
        }
    }

    pub fn get_predecessor_value(&self, idx: OpIndex, predecessor_index: i32) -> Value {
        match self.indices_to_keys_.get(&idx) {
            Some(key) => self.base.get_predecessor_value(*key, predecessor_index as usize),
            None => Value::default(),
        }
    }

    pub fn set(&mut self, idx: OpIndex, new_value: Value) -> bool {
        let key = self.get_or_create_key(idx);
        self.base.set(key, new_value)
    }

    pub fn new_key_with_data(&mut self, idx: OpIndex, data: KeyData, initial_value: Value) {
        if self.indices_to_keys_.contains_key(&idx) {
            panic!("Key already exists for this OpIndex");
        }

        let key = self.base.new_key(data, initial_value);
        self.indices_to_keys_.insert(idx, key);
    }

    pub fn new_key(&mut self, idx: OpIndex, initial_value: Value) {
        self.new_key_with_data(idx, KeyData::default(), initial_value);
    }

    pub fn has_key_for(&self, idx: OpIndex) -> bool {
        self.indices_to_keys_.contains_key(&idx)
    }

    pub fn try_get_key_for(&self, idx: OpIndex) -> Option<Key> {
        self.indices_to_keys_.get(&idx).copied()
    }

    fn get_or_create_key(&mut self, idx: OpIndex) -> Key {
        if let Some(key) = self.indices_to_keys_.get(&idx) {
            return *key;
        }

        let key = self.base.new_key_default();
        self.indices_to_keys_.insert(idx, key);
        key
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OpIndex {
    index: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct NoKeyData {}
