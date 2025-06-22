// src/objects/swiss_name_dictionary.rs

// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file provides Rust bindings for the C++ SwissNameDictionary class.

use std::{
    mem,
    ops::{Deref, DerefMut},
    ptr,
};

// Assuming a simplified implementation for swiss_table
mod swiss_table {
    pub type ctrl_t = u8;

    pub const HASH_BITS: usize = 7;

    #[inline]
    pub fn H1(hash: u32) -> u32 {
        hash & ((1u32 << HASH_BITS) - 1)
    }

    #[inline]
    pub fn H2(hash: u32) -> u8 {
        ((hash >> HASH_BITS) & 0xFF) as u8 // Assuming 8 bits for H2
    }

    pub struct ProbeSequence<const WIDTH: usize> {
        hash: u32,
        mask: u32,
        index: u32,
    }

    impl<const WIDTH: usize> ProbeSequence<WIDTH> {
        pub fn new(hash: u32, mask: u32) -> Self {
            ProbeSequence {
                hash,
                mask,
                index: 0,
            }
        }

        pub fn offset(&self) -> usize {
            (self.hash as usize).wrapping_add((self.index * WIDTH as u32) as usize) & self.mask as usize
        }

        pub fn offset_i(&self, i: i32) -> usize {
            (self.hash as usize).wrapping_add((self.index * WIDTH as u32 + i as u32) as usize) & self.mask as usize
        }

        pub fn next(&mut self) {
            self.index = self.index.wrapping_add(1);
        }

        pub fn index(&self) -> u32 {
            self.index
        }
    }
}

mod base {
    pub mod bits {
        pub fn round_up_to_power_of_two_32(v: i32) -> i32 {
            let mut v = v as u32;
            v -= 1;
            v |= v >> 1;
            v |= v >> 2;
            v |= v >> 4;
            v |= v >> 8;
            v |= v >> 16;
            (v + 1) as i32
        }
    }
}

// Placeholder for external types and constants
type Isolate = (); // Replace with actual Isolate type
type Object = (); // Replace with actual Object type
type Name = (); // Replace with actual Name type
type ByteArray = Vec<u8>; // Replace with actual ByteArray type
type Tagged<T> = T; // Replace with actual Tagged<T> type
type DirectHandle<T> = T; // Replace with actual DirectHandle<T> type
type PropertyDetails = u8;
type HeapObject = (); // Replace with actual HeapObject type
type ReadOnlyRoots = ();
type PtrComprCageBase = ();
type Smi = i32;

const kTaggedSize: usize = 8; // Example value, adjust as necessary
const kOneByteSize: usize = 1;

const kDataTableEntryCount: usize = 2;

const kMetaTableElementCountFieldIndex: usize = 0;
const kMetaTableDeletedElementCountFieldIndex: usize = 1;
const kMetaTableEnumerationDataStartIndex: usize = 2;

const kInitialCapacity: i32 = 8;
const kMaxFixedArrayCapacity: i32 = 1024;
const kMax1ByteMetaTableCapacity: i32 = 255; // Example value
const kMax2ByteMetaTableCapacity: i32 = 65535; // Example value

const kDataTableKeyEntryIndex: usize = 0;
const kDataTableValueEntryIndex: usize = 1;

const kNoHashSentinel: i32 = -1;

macro_rules! OBJECT_CONSTRUCTORS_IMPL {
    ($struct_name:ident, $parent_type:ident) => {
        impl $struct_name {
            // Placeholder for constructors, adjust as needed
            pub fn new() -> Self {
                Self {}
            }
        }
    };
}

macro_rules! ACCESSORS_CHECKED2 {
    ($struct_name:ident, $field_name:ident, $field_type:ty, $offset:expr, $is_tagged:expr, $check:expr) => {
        impl $struct_name {
            pub fn $field_name(&self) -> &$field_type {
                // Placeholder for accessor implementation. Implement the actual logic to get field data.
                unimplemented!()
            }

            pub fn set_$field_name(&mut self, value: $field_type) {
                // Placeholder for setter implementation. Implement the actual logic to set the field.
                unimplemented!()
            }
        }
    };
}

#[repr(C)]
struct SwissNameDictionary {
    // Fields
}

impl SwissNameDictionary {
    fn field_address(&self, offset: usize) -> *mut u8 {
        (self as *const Self as *mut u8).wrapping_add(offset)
    }

    fn read_field<T>(&self, offset: usize) -> T {
        unsafe { ptr::read_volatile(self.field_address(offset) as *const T) }
    }

    fn write_field<T>(&mut self, offset: usize, value: T) {
        unsafe { ptr::write_volatile(self.field_address(offset) as *mut T, value) }
    }

    fn raw_field(&self, offset: usize) -> *mut u8 {
        self.field_address(offset)
    }

    fn ctrl_table(&self) -> *mut swiss_table::ctrl_t {
        self.field_address(Self::ctrl_table_start_offset(self.capacity())) as *mut swiss_table::ctrl_t
    }

    fn property_details_table(&self) -> *mut u8 {
        self.field_address(Self::property_details_table_start_offset(self.capacity())) as *mut u8
    }

    fn capacity(&self) -> i32 {
        self.read_field::<i32>(Self::capacity_offset())
    }

    fn set_capacity(&mut self, capacity: i32) {
        debug_assert!(Self::is_valid_capacity(capacity));
        self.write_field(Self::capacity_offset(), capacity);
    }

    fn number_of_elements(&self) -> i32 {
        self.get_meta_table_field(kMetaTableElementCountFieldIndex)
    }

    fn number_of_deleted_elements(&self) -> i32 {
        self.get_meta_table_field(kMetaTableDeletedElementCountFieldIndex)
    }

    fn set_number_of_elements(&mut self, elements: i32) {
        self.set_meta_table_field(kMetaTableElementCountFieldIndex, elements);
    }

    fn set_number_of_deleted_elements(&mut self, deleted_elements: i32) {
        self.set_meta_table_field(kMetaTableDeletedElementCountFieldIndex, deleted_elements);
    }

    fn used_capacity(&self) -> i32 {
        self.number_of_elements() + self.number_of_deleted_elements()
    }

    const fn is_valid_capacity(capacity: i32) -> bool {
        capacity == 0 || (capacity >= kInitialCapacity && (capacity & (capacity - 1)) == 0)
    }

    const fn data_table_size(capacity: i32) -> usize {
        (capacity as usize) * kTaggedSize * kDataTableEntryCount
    }

    const fn ctrl_table_size(capacity: i32) -> usize {
        ((capacity as usize) + Group::kWidth) * kOneByteSize
    }

    const fn size_for(capacity: i32) -> usize {
        debug_assert!(Self::is_valid_capacity(capacity));
        Self::property_details_table_start_offset(capacity) + (capacity as usize)
    }

    const fn max_usable_capacity(capacity: i32) -> i32 {
        debug_assert!(Self::is_valid_capacity(capacity));

        if Group::kWidth == 8 && capacity == 4 {
            return 3;
        }
        capacity - capacity / 8
    }

    fn capacity_for(at_least_space_for: i32) -> i32 {
        if at_least_space_for <= 4 {
            if at_least_space_for == 0 {
                return 0;
            } else if at_least_space_for < 4 {
                return 4;
            } else if Group::kWidth == 16 {
                debug_assert_eq!(4, at_least_space_for);
                return 4;
            } else if Group::kWidth == 8 {
                debug_assert_eq!(4, at_least_space_for);
                return 8;
            }
        }

        let non_normalized = at_least_space_for + at_least_space_for / 7;
        base::bits::round_up_to_power_of_two_32(non_normalized)
    }

    fn entry_for_enumeration_index(&self, enumeration_index: i32) -> i32 {
        debug_assert!(enumeration_index < self.used_capacity());
        self.get_meta_table_field(kMetaTableEnumerationDataStartIndex + (enumeration_index as usize))
    }

    fn set_entry_for_enumeration_index(&mut self, enumeration_index: i32, entry: i32) {
        debug_assert!(enumeration_index < self.used_capacity());
        debug_assert!((entry as u32) < (self.capacity() as u32));
        debug_assert!(self.is_full(self.get_ctrl(entry)));

        self.set_meta_table_field(kMetaTableEnumerationDataStartIndex + (enumeration_index as usize), entry);
    }

    fn find_entry(&self, isolate: &Isolate, key: Tagged<Object>) -> InternalIndex {
        let name: Tagged<Name> = key; // Assuming implicit cast is valid
        debug_assert!(false); // IsUniqueName(name));
        let hash: u32 = 0; // name.hash();

        let ctrl = self.ctrl_table();
        let mut seq = Self::probe(hash, self.capacity());

        loop {
            let g = Group {
                ctrl: unsafe { ctrl.add(seq.offset()) },
            };

            for i in g.match_h2(swiss_table::H2(hash)) {
                let candidate_entry = seq.offset_i(i as i32);
                let candidate_key = self.key_at(candidate_entry);
                if false { // candidate_key == key { // This key matching is SwissNameDictionary specific!
                    return InternalIndex(candidate_entry as i32);
                }
            }

            if g.match_empty() {
                return InternalIndex::not_found();
            }

            seq.next();
            debug_assert!(seq.index() < self.capacity() as u32);
        }
    }

    fn load_from_data_table(&self, entry: usize, data_offset: usize) -> Tagged<Object> {
        self.load_from_data_table_inner((), entry, data_offset)
    }

    fn load_from_data_table_inner(&self, cage_base: PtrComprCageBase, entry: usize, data_offset: usize) -> Tagged<Object> {
        debug_assert!((entry as u32) < (self.capacity() as u32));
        let offset = Self::data_table_start_offset() + (entry * kDataTableEntryCount + data_offset) * kTaggedSize;
        unsafe { ptr::read_volatile(self.field_address(offset) as *const Object) } // TaggedField<Object>::Relaxed_Load
    }

    fn store_to_data_table(&mut self, entry: usize, data_offset: usize, data: Tagged<Object>) {
        debug_assert!((entry as u32) < (self.capacity() as u32));

        let offset = Self::data_table_start_offset() + (entry * kDataTableEntryCount + data_offset) * kTaggedSize;

        unsafe { ptr::write_volatile(self.field_address(offset) as *mut Object, data); } // RELAXED_WRITE_FIELD
        // WRITE_BARRIER
    }

    fn store_to_data_table_no_barrier(&mut self, entry: usize, data_offset: usize, data: Tagged<Object>) {
        debug_assert!((entry as u32) < (self.capacity() as u32));

        let offset = Self::data_table_start_offset() + (entry * kDataTableEntryCount + data_offset) * kTaggedSize;

        unsafe { ptr::write_volatile(self.field_address(offset) as *mut Object, data); } // RELAXED_WRITE_FIELD
    }

    fn clear_data_table_entry(&mut self, isolate: &Isolate, entry: usize) {
        let roots: ReadOnlyRoots = (); //ReadOnlyRoots(isolate);

        self.store_to_data_table(entry, kDataTableKeyEntryIndex, ()); //roots.the_hole_value());
        self.store_to_data_table(entry, kDataTableValueEntryIndex, ()); //roots.the_hole_value());
    }

    fn value_at_put(&mut self, entry: usize, value: Tagged<Object>) {
        debug_assert!(false); // !IsTheHole(value));
        self.store_to_data_table(entry, kDataTableValueEntryIndex, value);
    }

    fn set_key(&mut self, entry: usize, key: Tagged<Object>) {
        debug_assert!(false); // !IsTheHole(key));
        self.store_to_data_table(entry, kDataTableKeyEntryIndex, key);
    }

    fn details_at_put(&mut self, entry: usize, details: PropertyDetails) {
        debug_assert!((entry as u32) < (self.capacity() as u32));
        let encoded_details = details; // details.ToByte();
        unsafe { *self.property_details_table().add(entry) = encoded_details };
    }

    fn key_at(&self, entry: usize) -> Tagged<Object> {
        self.load_from_data_table(entry, kDataTableKeyEntryIndex)
    }

    fn name_at(&self, entry: InternalIndex) -> Tagged<Name> {
        self.key_at(entry.0 as usize) //Cast<Name>(KeyAt(entry));
    }

    fn value_at_raw(&self, entry: usize) -> Tagged<Object> {
        self.load_from_data_table(entry, kDataTableValueEntryIndex)
    }

    fn value_at(&self, entry: InternalIndex) -> Tagged<Object> {
        debug_assert!(self.is_full(self.get_ctrl(entry.0)));
        self.value_at_raw(entry.0 as usize)
    }

    fn try_value_at(&self, entry: InternalIndex) -> Option<Tagged<Object>> {
        if (entry.0 as u32) >= (self.capacity() as u32) {
            return None;
        }
        Some(self.value_at_raw(entry.0 as usize))
    }

    fn details_at(&self, entry: usize) -> PropertyDetails {
        debug_assert!(self.is_full(self.get_ctrl(entry as i32)));

        let encoded_details = unsafe { *self.property_details_table().add(entry) };
        encoded_details // PropertyDetails::FromByte(encoded_details)
    }

    fn get_ctrl(&self, entry: i32) -> swiss_table::ctrl_t {
        debug_assert!((entry as u32) < (self.capacity() as u32));

        unsafe { *self.ctrl_table().add(entry as usize) }
    }

    fn set_ctrl(&mut self, entry: usize, h: swiss_table::ctrl_t) {
        let capacity = self.capacity() as usize;
        debug_assert!((entry as u32) < (capacity as u32));

        let ctrl = self.ctrl_table();
        unsafe { *ctrl.add(entry) = h };

        let mask = capacity - 1;
        let copy_entry = ((entry - Group::kWidth) & mask) + 1 + ((Group::kWidth - 1) & mask);
        debug_assert!((entry < Group::kWidth) == (copy_entry == capacity + entry));
        debug_assert!((entry >= Group::kWidth) == (copy_entry == entry));
        unsafe { *ctrl.add(copy_entry) = h };
    }

    fn find_first_empty(&self, hash: u32) -> usize {
        let mut seq = Self::probe(hash, self.capacity());
        loop {
            let g = Group {
                ctrl: unsafe { self.ctrl_table().add(seq.offset()) },
            };
            let mask = g.match_empty_mask();
            if mask != 0 {
                return seq.offset_i(mask.trailing_zeros() as i32) as usize;
            }
            seq.next();
            debug_assert!(seq.index() < self.capacity() as u32);
        }
    }

    fn set_meta_table_field(&mut self, field_index: usize, value: i32) {
        let capacity = self.capacity();
        let meta_table = self.meta_table();
        if capacity <= kMax1ByteMetaTableCapacity {
            Self::set_meta_table_field_inner::<u8>(meta_table, field_index, value);
        } else if capacity <= kMax2ByteMetaTableCapacity {
            Self::set_meta_table_field_inner::<u16>(meta_table, field_index, value);
        } else {
            Self::set_meta_table_field_inner::<u32>(meta_table, field_index, value);
        }
    }

    fn get_meta_table_field(&self, field_index: usize) -> i32 {
        let capacity = self.capacity();
        let meta_table = self.meta_table();
        if capacity <= kMax1ByteMetaTableCapacity {
            Self::get_meta_table_field_inner::<u8>(meta_table, field_index)
        } else if capacity <= kMax2ByteMetaTableCapacity {
            Self::get_meta_table_field_inner::<u16>(meta_table, field_index)
        } else {
            Self::get_meta_table_field_inner::<u32>(meta_table, field_index)
        }
    }

    fn set_meta_table_field_inner<T: Sized + Copy>(meta_table: &ByteArray, field_index: usize, value: i32) {
        debug_assert!(value <= std::mem::size_of::<T>() as i32);
        debug_assert!(meta_table.len() > field_index * std::mem::size_of::<T>());

        let raw_data = meta_table.as_ptr() as *mut T;
        unsafe {
            *raw_data.add(field_index) = value as T;
        }
    }

    fn get_meta_table_field_inner<T: Sized + Copy>(meta_table: &ByteArray, field_index: usize) -> i32 {
        debug_assert!(meta_table.len() > field_index * std::mem::size_of::<T>());
        let raw_data = meta_table.as_ptr() as *const T;
        unsafe { *raw_data.add(field_index) as i32 }
    }

    const fn meta_table_size_per_entry_for(capacity: i32) -> usize {
        debug_assert!(Self::is_valid_capacity(capacity));

        if capacity <= kMax1ByteMetaTableCapacity {
            std::mem::size_of::<u8>()
        } else if capacity <= kMax2ByteMetaTableCapacity {
            std::mem::size_of::<u16>()
        } else {
            std::mem::size_of::<u32>()
        }
    }

    fn meta_table_size_for(capacity: i32) -> usize {
        debug_assert!(Self::is_valid_capacity(capacity));

        let per_entry_size = Self::meta_table_size_per_entry_for(capacity);
        per_entry_size * (Self::max_usable_capacity(capacity) as usize + 2)
    }

    fn is_key(&self, roots: ReadOnlyRoots, key_candidate: Tagged<Object>) -> bool {
       true // key_candidate != roots.the_hole_value() // Placeholder, needs actual comparison
    }

    fn to_key(&self, roots: ReadOnlyRoots, entry: usize, out_key: &mut Tagged<Object>) -> bool {
        let k = self.key_at(entry);
        if !self.is_key(roots, k) {
            return false;
        }
        *out_key = k;
        true
    }

    fn add_internal(&mut self, key: Tagged<Name>, value: Tagged<Object>, details: PropertyDetails) -> i32 {
        debug_assert!(false); // IsUniqueName(key));
        debug_assert!(self.used_capacity() <= Self::max_usable_capacity(self.capacity()));

        let hash = 0; // key.hash();

        let target = self.find_first_empty(hash);

        self.set_ctrl(target, swiss_table::H2(hash));
        self.set_key(target, key);
        self.value_at_put(target, value);
        self.details_at_put(target, details);

        target as i32
    }

    fn initialize(&mut self, isolate: &Isolate, meta_table: Tagged<ByteArray>, capacity: i32) {
        debug_assert!(Self::is_valid_capacity(capacity));

        self.set_capacity(capacity);
        self.set_hash(kNoHashSentinel);

        unsafe {
            ptr::write_bytes(self.ctrl_table(), Ctrl::kEmpty, Self::ctrl_table_size(capacity));
        }

        // MemsetTagged
        // TODO Implement this more efficiently
        for i in 0..(capacity * (kDataTableEntryCount as i32)) {
          self.store_to_data_table((i as usize),0 , ());
        }

        self.set_meta_table(meta_table);

        self.set_number_of_elements(0);
        self.set_number_of_deleted_elements(0);

        // We leave the enumeration table and PropertyDetails table uninitialized.
    }

    fn set_hash(&mut self, hash: i32) {
      self.write_field(Self::prefix_offset(), hash);
    }

    fn hash(&self) -> i32 {
      self.read_field(Self::prefix_offset())
    }

    const fn prefix_offset() -> usize {
      mem::size_of::<HeapObject>()
    }

    const fn capacity_offset() -> usize {
      Self::prefix_offset() + mem::size_of::<u32>()
    }

    const fn meta_table_pointer_offset() -> usize {
      Self::capacity_offset() + mem::size_of::<i32>()
    }

    const fn data_table_start_offset() -> usize {
      Self::meta_table_pointer_offset() + kTaggedSize
    }

    const fn data_table_end_offset(capacity: i32) -> usize {
      Self::ctrl_table_start_offset(capacity)
    }

    const fn ctrl_table_start_offset(capacity: i32) -> usize {
      Self::data_table_start_offset() + Self::data_table_size(capacity)
    }

    const fn property_details_table_start_offset(capacity: i32) -> usize {
      Self::ctrl_table_start_offset(capacity) + Self::ctrl_table_size(capacity)
    }

    const fn max_capacity() -> i32 {
        let k_const_size = Self::data_table_start_offset() + std::mem::size_of::<ByteArray>() + 2 * std::mem::size_of::<u32>();
        let k_per_entry_size = kDataTableEntryCount * kTaggedSize + kOneByteSize + kOneByteSize + std::mem::size_of::<u32>();
        let result = (kMaxFixedArrayCapacity as usize * kTaggedSize - k_const_size) / k_per_entry_size;

        result as i32
    }

    fn is_empty(c: swiss_table::ctrl_t) -> bool {
        c == Ctrl::kEmpty
    }

    fn is_full(c: swiss_table::ctrl_t) -> bool {
      c >= 0
    }

    fn is_deleted(c: swiss_table::ctrl_t) -> bool {
        c == Ctrl::kDeleted
    }

    fn is_empty_or_deleted(c: swiss_table::ctrl_t) -> bool {
        c < Ctrl::kSentinel
    }

    fn probe(hash: u32, capacity: i32) -> swiss_table::ProbeSequence<Group::kWidth> {
        let non_zero_capacity = if capacity == 0 { 1 } else { capacity };
        swiss_table::ProbeSequence::new(
            swiss_table::H1(hash),
            (non_zero_capacity - 1) as u32,
        )
    }

    fn ensure_growable(_isolate: &Isolate, table: Self) -> Self {
        let capacity = table.capacity();
        if table.used_capacity() < Self::max_usable_capacity(capacity) {
            return table;
        }

        let new_capacity = if capacity == 0 {
            kInitialCapacity
        } else {
            capacity * 2
        };
        Self::rehash(_isolate, table, new_capacity)
    }

    fn add(
        isolate: &Isolate,
        original_table: Self,
        key: DirectHandle<Name>,
        value: DirectHandle<Object>,
        details: PropertyDetails,
        entry_out: Option<&mut InternalIndex>,
    ) -> Self {
        if false { // original_table.find_entry(isolate, key).is_not_found()
            unimplemented!()
        }

        let mut table = Self::ensure_growable(isolate, original_table);

        let nof = table.number_of_elements();
        let nod = table.number_of_deleted_elements();
        let new_enum_index = nof + nod;

        let new_entry = table.add_internal(key, value, details);

        table.set_number_of_elements(nof + 1);
        table.set_entry_for_enumeration_index(new_enum_index, new_entry);

        if let Some(entry_out) = entry_out {
            *entry_out = InternalIndex(new_entry);
        }

        table
    }

    fn rehash(_isolate: &Isolate, _table: Self, _new_capacity: i32) -> Self{
        unimplemented!()
    }

    ACCESSORS_CHECKED2!(SwissNameDictionary, meta_table, ByteArray, Self::meta_table_pointer_offset(), true, false)
}

OBJECT_CONSTRUCTORS_IMPL!(SwissNameDictionary, HeapObject);

// Placeholder implementation for Group.
struct Group {
    ctrl: *mut swiss_table::ctrl_t,
}

impl Group {
    const kWidth: usize = 8;

    fn match_h2(&self, h2: u8) -> [i32; Group::kWidth] {
        let mut result: [i32; Group::kWidth] = [0; Group::kWidth];
        let mut count = 0;
        for i in 0..Group::kWidth {
            unsafe {
                if *self.ctrl.add(i) == h2 {
                    result[count] = i as i32;
                    count += 1;
                }
            }
        }
        result
    }

    fn match_empty(&self) -> bool {
        for i in 0..Group::kWidth {
            unsafe {
                if *self.ctrl.add(i) == Ctrl::kEmpty {
                    return true;
                }
            }
        }
        false
    }

    fn match_empty_mask(&self) -> u8 {
        let mut mask: u8 = 0;
        for i in 0..Group::kWidth {
            unsafe {
                if *self.ctrl.add(i) == Ctrl::kEmpty {
                    mask |= 1 << i;
                }
            }
        }
        mask
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct InternalIndex(i32);

impl InternalIndex {
    const fn not_found() -> Self {
        InternalIndex(-1)
    }

    fn is_not_found(&self) -> bool {
        self.0 < 0
    }

    fn as_int(&self) -> i32 {
      self.0
    }
}

struct Ctrl {}

impl Ctrl {
    const kEmpty: swiss_table::ctrl_t = 255; //0xFF;
    const kDeleted: swiss_table::ctrl_t = 254; //0xFE;
    const kSentinel: swiss_table::ctrl_t = 128;
}

struct IndexIterator {
    enum_index_: i32,
    used_capacity_: i32,
    dict_: SwissNameDictionary,
}

impl IndexIterator {
    fn new(dict_: SwissNameDictionary, start: i32) -> Self {
        let used_capacity_ = dict_.used_capacity();

        IndexIterator {
          enum_index_: start,
          dict_: dict_,
          used_capacity_: used_capacity_,
        }
    }

    fn increment(&mut self) -> &mut Self {
        debug_assert!(self.enum_index_ < self.used_capacity_);
        self.enum_index_ += 1;
        self
    }

    fn equals(&self, b: &IndexIterator) -> bool {
        debug_assert!(self.enum_index_ <= self.used_capacity_);
        debug_assert!(b.enum_index_ <= self.used_capacity_);

        self.enum_index_ == b.enum_index_
    }

    fn deref(&self) -> InternalIndex {
      debug_assert!(self.enum_index_ <= self.used_capacity_);

      if self.enum_index_ == self.used_capacity_ {
        return InternalIndex::not_found();
      }

      InternalIndex(self.dict_.entry_for_enumeration_index(self.enum_index_))
    }
}

struct IndexIterable {
    dict_: SwissNameDictionary,
}

impl IndexIterable {
    fn new(dict_: SwissNameDictionary) -> Self {
      IndexIterable {
        dict_: dict_,
      }
    }

    fn begin(&self) -> IndexIterator {
        IndexIterator::new(self.dict_, 0)
    }

    fn end(&self) -> IndexIterator {
        IndexIterator::new(self.dict_, self.dict_.used_capacity())
    }
}

impl SwissNameDictionary {
    fn iterate_entries_ordered(&self) -> IndexIterable {
      if self.capacity() == 0 {
        return IndexIterable::new(SwissNameDictionary::new());
      }
      IndexIterable::new(*self)
    }

    fn iterate_entries(&self) -> IndexIterable {
        self.iterate_entries_ordered()
    }
}