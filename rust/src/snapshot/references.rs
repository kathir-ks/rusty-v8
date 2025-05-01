// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/snapshot/references.h

mod base {
    pub mod bit_field {
        pub struct BitField<T, const START: usize, const SIZE: usize>;

        impl<T, const START: usize, const SIZE: usize> BitField<T, START, SIZE> {
            pub const kSize: usize = SIZE;

            pub fn encode(_value: T) -> u32 {
                // Dummy implementation
                0
            }

            pub fn decode(_encoded: u32) -> T {
                // Dummy implementation
                unsafe { std::mem::transmute(0u8) } // Requires T to be Copy
            }

            pub fn Next<U, const TOTAL_SIZE: usize>() -> BitField<U, START + SIZE, TOTAL_SIZE - SIZE> {
                BitField
            }
        }
    }
}

mod execution {
    pub struct Isolate;
}

mod utils {
    use std::collections::HashMap;

    pub struct IdentityMap<K, V> {
        map: HashMap<K, V>,
    }

    impl<K, V> IdentityMap<K, V>
    where
        K: std::hash::Hash + Eq + Copy,
        V: Copy,
    {
        pub fn new() -> Self {
            IdentityMap { map: HashMap::new() }
        }

        pub fn Find(&self, key: K) -> Option<&V> {
            self.map.get(&key)
        }

        pub fn Insert(&mut self, key: K, value: V) {
            self.map.insert(key, value);
        }
    }

    pub struct DefaultAllocationPolicy;

}

pub mod internal {
    use crate::base::bit_field::BitField;
    use crate::execution::Isolate;
    use crate::utils::IdentityMap;
    use std::collections::HashMap;

    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum SnapshotSpace {
        ReadOnlyHeap = 0,
        Old = 1,
        Code = 2,
        Trusted = 3,
    }
    pub const K_NUMBER_OF_SNAPSHOT_SPACES: usize = 4;

    #[derive(Debug, Copy, Clone)]
    pub struct SerializerReference {
        bit_field_: u32,
    }

    impl SerializerReference {
        fn new(type_: SpecialValueType, value: u32) -> Self {
            SerializerReference {
                bit_field_: TypeBits::encode(type_) | ValueBits::encode(value),
            }
        }

        pub fn BackReference(index: u32) -> Self {
            SerializerReference::new(SpecialValueType::BackReference, index)
        }

        pub fn OffHeapBackingStoreReference(index: u32) -> Self {
            SerializerReference::new(SpecialValueType::OffHeapBackingStore, index)
        }

        pub fn AttachedReference(index: u32) -> Self {
            SerializerReference::new(SpecialValueType::AttachedReference, index)
        }

        pub fn BuiltinReference(index: u32) -> Self {
            SerializerReference::new(SpecialValueType::BuiltinReference, index)
        }

        pub fn is_back_reference(&self) -> bool {
            TypeBits::decode(self.bit_field_) == SpecialValueType::BackReference
        }

        pub fn back_ref_index(&self) -> u32 {
            debug_assert!(self.is_back_reference());
            ValueBits::decode(self.bit_field_)
        }

        pub fn is_off_heap_backing_store_reference(&self) -> bool {
            TypeBits::decode(self.bit_field_) == SpecialValueType::OffHeapBackingStore
        }

        pub fn off_heap_backing_store_index(&self) -> u32 {
            debug_assert!(self.is_off_heap_backing_store_reference());
            ValueBits::decode(self.bit_field_)
        }

        pub fn is_attached_reference(&self) -> bool {
            TypeBits::decode(self.bit_field_) == SpecialValueType::AttachedReference
        }

        pub fn attached_reference_index(&self) -> u32 {
            debug_assert!(self.is_attached_reference());
            ValueBits::decode(self.bit_field_)
        }

        pub fn is_builtin_reference(&self) -> bool {
            TypeBits::decode(self.bit_field_) == SpecialValueType::BuiltinReference
        }

        pub fn builtin_index(&self) -> u32 {
            debug_assert!(self.is_builtin_reference());
            ValueBits::decode(self.bit_field_)
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(u8)]
    enum SpecialValueType {
        BackReference,
        AttachedReference,
        OffHeapBackingStore,
        BuiltinReference,
    }

    type TypeBits = BitField<SpecialValueType, 0, 2>;
    type ValueBits = TypeBits::Next<u32, 30>;

    //This assert does not translate directly, it verifies a memory alignment that Rust handles automatically
    //static_assert!(std::mem::size_of::<SerializerReference>() <= std::mem::size_of::<*mut std::ffi::c_void>());

    #[derive(Debug)]
    pub struct SerializerReferenceMap {
        map_: IdentityMap<TaggedHeapObject, SerializerReference>,
        backing_store_map_: HashMap<*mut std::ffi::c_void, SerializerReference>,
        attached_reference_index_: i32,
    }

    impl SerializerReferenceMap {
        pub fn new(_isolate: &mut Isolate) -> Self {
            SerializerReferenceMap {
                map_: IdentityMap::new(),
                backing_store_map_: HashMap::new(),
                attached_reference_index_: 0,
            }
        }

        pub fn LookupReference(&self, object: TaggedHeapObject) -> Option<&SerializerReference> {
            self.map_.Find(object)
        }

        pub fn LookupReference_handle(&self, object: DirectHandleHeapObject) -> Option<&SerializerReference> {
            self.map_.Find(object.object)
        }

        pub fn LookupBackingStore(&self, backing_store: *mut std::ffi::c_void) -> Option<&SerializerReference> {
            self.backing_store_map_.get(&backing_store)
        }

        pub fn Add(&mut self, object: TaggedHeapObject, reference: SerializerReference) {
            debug_assert!(self.LookupReference(object).is_none());
            self.map_.Insert(object, reference);
        }

        pub fn AddBackingStore(&mut self, backing_store: *mut std::ffi::c_void, reference: SerializerReference) {
            debug_assert!(!self.backing_store_map_.contains_key(&backing_store));
            self.backing_store_map_.insert(backing_store, reference);
        }

        pub fn AddAttachedReference(&mut self, object: TaggedHeapObject) -> SerializerReference {
            let reference =
                SerializerReference::AttachedReference(self.attached_reference_index_ as u32);
            self.map_.Insert(object, reference);
            self.attached_reference_index_ += 1;
            reference
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct TaggedHeapObject {
        address: usize,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DirectHandleHeapObject {
        object: TaggedHeapObject,
    }
}