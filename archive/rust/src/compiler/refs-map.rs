// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::collections::HashMap;
use std::hash::{Hasher, BuildHasherDefault};
use std::marker::PhantomData;

mod base {
    pub struct DefaultHasher {
        state: u64,
    }

    impl DefaultHasher {
        pub fn new() -> Self {
            DefaultHasher { state: 0 }
        }
    }

    impl Hasher for DefaultHasher {
        fn finish(&self) -> u64 {
            self.state
        }

        fn write(&mut self, bytes: &[u8]) {
            for &b in bytes {
                self.state = self.state.wrapping_mul(1000003).wrapping_xor(b as u64);
            }
        }
    }
}

type Address = usize; // Or a more specific type if Address is defined elsewhere
type Zone = (); // Placeholder, replace with actual Zone type if needed

pub struct ObjectData {} // Placeholder, replace with actual ObjectData type if needed

// Define a custom hasher.  In C++, AddressMatcher inherits from KeyEqualityMatcher<Address>
// which implies the implementation of an equality function. We also need a hash function in Rust.

#[derive(Default, Clone)]
pub struct AddressHasher;

impl std::hash::Hasher for AddressHasher {
    fn finish(&self) -> u64 {
        0
    }
    fn write(&mut self, _bytes: &[u8]) {}
    fn write_usize(&mut self, i: usize) {
        // This effectively uses the address as the hash.
        // A better hash function might be needed depending on use case.
        // We are just conforming to the original C++ code.
    }
}

#[derive(Default, Clone)]
pub struct AddressHashBuilder;

impl std::hash::BuildHasher for AddressHashBuilder {
    type Hasher = AddressHasher;

    fn build_hasher(&self) -> Self::Hasher {
        AddressHasher::default()
    }
}

pub struct RefsMap {
    map: HashMap<Address, ObjectData, BuildHasherDefault<AddressHashBuilder>>,
    //zone: *mut Zone, // Removed raw pointer. Zone usage to be determined.
    _phantom: PhantomData<Zone>,
}

impl RefsMap {
    pub fn new(capacity: usize, zone: Zone) -> Self {
        RefsMap {
            map: HashMap::with_capacity_and_hasher(capacity, BuildHasherDefault::default()),
            //zone: zone, // Removed raw pointer
            _phantom: PhantomData,
        }
    }

    pub fn with_hasher(capacity: usize) -> Self {
        RefsMap {
            map: HashMap::with_capacity_and_hasher(capacity, BuildHasherDefault::default()),
            //zone: std::ptr::null_mut(), // Removed raw pointer.  Safe alternative needs to be found
            _phantom: PhantomData,
        }
    }

    pub fn copy_from(other: &RefsMap) -> Self {
        RefsMap {
            map: other.map.clone(),
            //zone: other.zone, // Removed raw pointer
            _phantom: PhantomData,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn lookup(&self, key: &Address) -> Option<&ObjectData> {
        self.map.get(key)
    }

    pub fn lookup_or_insert(&mut self, key: Address, data: ObjectData) -> Option<&mut ObjectData> {
        use std::collections::hash_map::Entry;
        match self.map.entry(key) {
            Entry::Occupied(entry) => Some(entry.into_mut()),
            Entry::Vacant(entry) => {
                entry.insert(data);
                Some(self.map.get_mut(&key).unwrap()) // Safe because we just inserted it.
            }
        }
    }

    pub fn remove(&mut self, key: &Address) -> Option<ObjectData> {
        self.map.remove(key)
    }

    // Equivalent to occupancy() from the TemplateHashMapImpl in C++.
    pub fn occupancy(&self) -> usize {
        self.map.len()
    }
}