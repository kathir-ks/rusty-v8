// Converted from V8 C++ source files:
// Header: hashmap-entry.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub struct Address {}

    pub struct NoHashMapValue {}

    pub struct TemplateHashMapEntry<Key, Value> {
        pub key: Key,
        pub value: Value,
        pub hash: u32,
        exists_: bool,
    }

    impl<Key, Value> TemplateHashMapEntry<Key, Value> {
        pub fn new(key: Key, value: Value, hash: u32) -> Self
        where
            Value: std::cmp::PartialEq<NoHashMapValue>,
        {
            TemplateHashMapEntry {
                key,
                value,
                hash,
                exists_: true,
            }
        }

        pub fn exists(&self) -> bool {
            self.exists_
        }

        pub fn clear(&mut self) {
            self.exists_ = false;
        }
    }

    impl<Key, Value> TemplateHashMapEntry<Key, Value>
    where
        Value: std::cmp::PartialEq<NoHashMapValue>,
    {
    }

    impl<Key, Value> TemplateHashMapEntry<Key, Value> {
        pub fn create(key: Key, value: Value, hash: u32) -> Self {
            TemplateHashMapEntry {
                key,
                value,
                hash,
                exists_: true,
            }
        }
    }

    impl<Value> TemplateHashMapEntry<Address, Value> {
        pub fn new(key: Address, value: Value, hash: u32) -> Self {
            TemplateHashMapEntry {
                key,
                value,
                hash,
                exists_: true,
            }
        }

        pub fn exists(&self) -> bool {
            // Assuming Address can be compared to a default invalid value.
            // You might need to adjust this based on the actual Address type.
            true
        }

        pub fn clear(&mut self) {
        }
    }

    impl<Key> TemplateHashMapEntry<Key, NoHashMapValue> {
        pub fn new(key: Key, hash: u32) -> Self {
            TemplateHashMapEntry {
                key,
                value: NoHashMapValue {},
                hash,
                exists_: true,
            }
        }

        pub fn exists(&self) -> bool {
            self.exists_
        }

        pub fn clear(&mut self) {
            self.exists_ = false;
        }
    }

    impl<Key> TemplateHashMapEntry<*mut Key, NoHashMapValue> {
        pub fn new(key: *mut Key, hash: u32) -> Self {
            TemplateHashMapEntry {
                key,
                value: NoHashMapValue {},
                hash,
            }
        }

        pub fn exists(&self) -> bool {
            self.key != std::ptr::null_mut()
        }

        pub fn clear(&mut self) {
            self.key = std::ptr::null_mut();
        }
    }

    // Specialization for pointer-valued keys
    impl<Key, Value> TemplateHashMapEntry<*mut Key, Value> {
        pub key: *mut Key,
        pub value: Value,
        pub hash: u32, // The full hash value for key

        pub fn new(key: *mut Key, value: Value, hash: u32) -> Self {
            TemplateHashMapEntry { key, value, hash }
        }

        pub fn exists(&self) -> bool {
            self.key != std::ptr::null_mut()
        }

        pub fn clear(&mut self) {
            self.key = std::ptr::null_mut();
        }
    }

    // Specialization for Address-valued keys
    impl<Value> TemplateHashMapEntry<Address, Value> {
        pub key: Address,
        pub value: Value,
        pub hash: u32, // The full hash value for key

        pub fn new(key: Address, value: Value, hash: u32) -> Self {
            TemplateHashMapEntry { key, value, hash }
        }

        pub fn exists(&self) -> bool {
            true
        }

        pub fn clear(&mut self) {
        }
    }

    // Specialization for pointer-valued keys and no value.
    pub struct TemplateHashMapEntryPtrNoValue<Key> {
        pub key: *mut Key,
        pub value: NoHashMapValue, // Value in union with key to not take up space.
        pub hash: u32,              // The full hash value for key
    }

    impl<Key> TemplateHashMapEntryPtrNoValue<Key> {
        pub fn new(key: *mut Key, hash: u32) -> Self {
            TemplateHashMapEntryPtrNoValue {
                key,
                value: NoHashMapValue {},
                hash,
            }
        }

        pub fn exists(&self) -> bool {
            self.key != std::ptr::null_mut()
        }

        pub fn clear(&mut self) {
            self.key = std::ptr::null_mut();
        }
    }
}
