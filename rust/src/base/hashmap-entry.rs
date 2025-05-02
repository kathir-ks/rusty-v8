// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod base {

  // Marker type for hashmaps without a value (i.e. hashsets). These won't
  // allocate space for the value in the entry.
  pub struct NoHashMapValue {}

  // HashMap entries are (key, value, hash) triplets, with a boolean indicating if
  // they are an empty entry. Some clients may not need to use the value slot
  // (e.g. implementers of sets, where the key is the value), in which case they
  // should use NoHashMapValue.
  #[derive(Debug)]
  pub struct TemplateHashMapEntry<Key, Value> {
    key: Key,
    value: Value,
    hash: u32, // The full hash value for key
    exists: bool,
  }

  impl<Key, Value> TemplateHashMapEntry<Key, Value>
  where
    Value: std::marker::Sized,
  {
    pub fn new(key: Key, value: Value, hash: u32) -> Self {
      TemplateHashMapEntry {
        key,
        value,
        hash,
        exists: true,
      }
    }

    pub fn exists(&self) -> bool {
      self.exists
    }

    pub fn clear(&mut self) {
      self.exists = false;
    }
  }

  // Specialization for pointer-valued keys
  #[derive(Debug)]
  pub struct TemplateHashMapEntryPtr<Key, Value> {
    key: *mut Key,
    value: Value,
    hash: u32, // The full hash value for key
  }

  impl<Key, Value> TemplateHashMapEntryPtr<Key, Value>
  where
    Value: std::marker::Sized,
  {
    pub fn new(key: *mut Key, value: Value, hash: u32) -> Self {
      TemplateHashMapEntryPtr { key, value, hash }
    }

    pub fn exists(&self) -> bool {
      !self.key.is_null()
    }

    pub fn clear(&mut self) {
      self.key = std::ptr::null_mut();
    }
  }

  // Address type alias to u64.  This could potentially be a pointer, but since it's just used in hashmap
  // context, using u64 directly should be fine
  pub type Address = u64;

  // Specialization for Address-valued keys
  #[derive(Debug)]
  pub struct TemplateHashMapEntryAddress<Value> {
    key: Address,
    value: Value,
    hash: u32, // The full hash value for key
  }

  impl<Value> TemplateHashMapEntryAddress<Value>
  where
    Value: std::marker::Sized,
  {
    pub fn new(key: Address, value: Value, hash: u32) -> Self {
      TemplateHashMapEntryAddress { key, value, hash }
    }

    pub fn exists(&self) -> bool {
      self.key != u64::MAX
    }

    pub fn clear(&mut self) {
      self.key = u64::MAX;
    }
  }

  // Specialization for no value.
  #[derive(Debug)]
  pub struct TemplateHashMapEntryNoValue<Key> {
    key: Key,
    hash: u32, // The full hash value for key
    exists: bool,
  }

  impl<Key> TemplateHashMapEntryNoValue<Key> {
    pub fn new(key: Key, hash: u32) -> Self {
      TemplateHashMapEntryNoValue {
        key,
        hash,
        exists: true,
      }
    }

    pub fn exists(&self) -> bool {
      self.exists
    }

    pub fn clear(&mut self) {
      self.exists = false;
    }
  }

  // Specialization for pointer-valued keys and no value.
  #[derive(Debug)]
  pub struct TemplateHashMapEntryPtrNoValue<Key> {
    key: *mut Key,
    hash: u32, // The full hash value for key
  }

  impl<Key> TemplateHashMapEntryPtrNoValue<Key> {
    pub fn new(key: *mut Key, hash: u32) -> Self {
      TemplateHashMapEntryPtrNoValue { key, hash }
    }

    pub fn exists(&self) -> bool {
      !self.key.is_null()
    }

    pub fn clear(&mut self) {
      self.key = std::ptr::null_mut();
    }
  }
}