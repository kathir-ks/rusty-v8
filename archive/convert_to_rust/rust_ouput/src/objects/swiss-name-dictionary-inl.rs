// Converted from V8 C++ source files:
// Header: swiss-name-dictionary-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::{mem::size_of, num::NonZeroU32, ops::Deref, optional::Option, ptr};

use crate::base::macros;
use crate::execution::isolate_utils_inl;
use crate::heap::heap;
use crate::objects::fixed_array_inl;
use crate::objects::fixed_array::FixedArray;
use crate::objects::instance_type_inl;
use crate::objects::js_collection_iterator;
use crate::objects::objects_inl;
use crate::objects::slots_inl;
use crate::objects::smi::Smi;
use crate::objects::swiss_name_dictionary::{Ctrl, Group, SwissNameDictionary};
use crate::objects::object_macros;

use std::convert::TryInto;

use crate::objects::heap_object::HeapObject;
use crate::objects::name::Name;
use crate::objects::property_details::PropertyDetails;
use crate::objects::swiss_name_dictionary::kMetaTableEnumerationDataStartIndex;

use crate::objects::internal_index::InternalIndex;
use crate::objects::object::Object;

use crate::objects::property_array::PropertyArray;
use crate::objects::heap_object::Tagged;

use crate::objects::byte_array::ByteArray;
use crate::objects::read_only_roots::ReadOnlyRoots;
use crate::objects::tagged_field::TaggedField;
use crate::objects::js_date_time_format_inl::Isolate;
use crate::objects::heap_object::PtrComprCageBase;

#[repr(C)]
pub struct SwissNameDictionaryImpl {
    heap_object: HeapObject,
}

impl SwissNameDictionaryImpl {
    pub fn CtrlTable(&self) -> *mut u8 {
        let capacity = self.Capacity();
        let offset = self.CtrlTableStartOffset(capacity);
        unsafe { self.heap_object.address().add(offset).cast_mut() }
    }

    pub fn PropertyDetailsTable(&self) -> *mut u8 {
        let capacity = self.Capacity();
        let offset = self.PropertyDetailsTableStartOffset(capacity);
        unsafe { self.heap_object.address().add(offset).cast_mut() }
    }

    pub fn Capacity(&self) -> i32 {
        let offset = self.CapacityOffset();
        unsafe { (self.heap_object.address().add(offset) as *const i32).read() }
    }

    pub fn SetCapacity(&mut self, capacity: i32) {
        assert!(self.IsValidCapacity(capacity));
        let offset = self.CapacityOffset();
        unsafe {
            (self.heap_object.address().add(offset) as *mut i32).write(capacity);
        }
    }

    pub fn NumberOfElements(&self) -> i32 {
        self.GetMetaTableField(SwissNameDictionary::kMetaTableElementCountFieldIndex)
    }

    pub fn NumberOfDeletedElements(&self) -> i32 {
        self.GetMetaTableField(SwissNameDictionary::kMetaTableDeletedElementCountFieldIndex)
    }

    pub fn SetNumberOfElements(&mut self, elements: i32) {
        self.SetMetaTableField(SwissNameDictionary::kMetaTableElementCountFieldIndex, elements);
    }

    pub fn SetNumberOfDeletedElements(&mut self, deleted_elements: i32) {
        self.SetMetaTableField(
            SwissNameDictionary::kMetaTableDeletedElementCountFieldIndex,
            deleted_elements,
        );
    }

    pub fn UsedCapacity(&self) -> i32 {
        self.NumberOfElements() + self.NumberOfDeletedElements()
    }

    const kInitialCapacity: i32 = 8;

    pub fn IsValidCapacity(&self, capacity: i32) -> bool {
        capacity == 0 || (capacity >= Self::kInitialCapacity && (capacity & (capacity - 1)) == 0)
    }

    pub fn DataTableSize(&self, capacity: i32) -> usize {
        (capacity as usize) * size_of::<usize>() * SwissNameDictionary::kDataTableEntryCount as usize
    }

    pub fn CtrlTableSize(&self, capacity: i32) -> usize {
        ((capacity as usize) + SwissNameDictionary::kGroupWidth as usize) * size_of::<u8>()
    }

    pub fn SizeFor(&self, capacity: i32) -> usize {
        assert!(self.IsValidCapacity(capacity));
        self.PropertyDetailsTableStartOffset(capacity) + (capacity as usize)
    }

    pub fn MaxUsableCapacity(&self, capacity: i32) -> i32 {
        assert!(self.IsValidCapacity(capacity));
        if Group::kWidth == 8 && capacity == 4 {
            return 3;
        }
        capacity - capacity / 8
    }

    pub fn CapacityFor(&self, at_least_space_for: i32) -> i32 {
        if at_least_space_for <= 4 {
            if at_least_space_for == 0 {
                return 0;
            } else if at_least_space_for < 4 {
                return 4;
            } else if Group::kWidth == 16 {
                assert_eq!(4, at_least_space_for);
                return 4;
            } else if Group::kWidth == 8 {
                assert_eq!(4, at_least_space_for);
                return 8;
            }
        }

        let non_normalized = at_least_space_for + at_least_space_for / 7;
        let rounded = non_normalized.next_power_of_two();
        rounded as i32
    }

    pub fn EntryForEnumerationIndex(&self, enumeration_index: i32) -> i32 {
        assert!(enumeration_index < self.UsedCapacity());
        self.GetMetaTableField(kMetaTableEnumerationDataStartIndex + enumeration_index)
    }

    pub fn SetEntryForEnumerationIndex(&mut self, enumeration_index: i32, entry: i32) {
        assert!(enumeration_index < self.UsedCapacity());
        assert!(entry as u32 <= self.Capacity() as u32);
        assert!(self.IsFull(self.GetCtrl(entry)));

        self.SetMetaTableField(kMetaTableEnumerationDataStartIndex + enumeration_index, entry);
    }

    pub fn FindEntry(&self, isolate: &Isolate, key: Tagged<Object>) -> InternalIndex {
        let name = unsafe { key.unchecked_cast::<Name>() };
        assert!(self.IsUniqueName(name));
        let hash = name.hash();

        let ctrl = self.CtrlTable();
        let mut seq = self.probe(hash, self.Capacity());

        loop {
            let g = Group {
                ctrl: unsafe { ctrl.add(seq.offset()) },
            };

            for i in g.Match(hash) {
                let candidate_entry = seq.offset(i);
                let candidate_key = self.KeyAt(candidate_entry);
                if candidate_key == key {
                    return InternalIndex(candidate_entry);
                }
            }

            if g.MatchEmpty() {
                return InternalIndex::NotFound();
            }

            seq.next();

            assert!(seq.index() < self.Capacity() as usize);
        }
    }

    pub fn LoadFromDataTable(&self, entry: i32, data_offset: i32) -> Tagged<Object> {
        let cage_base = PtrComprCageBase {};
        self.LoadFromDataTable_base(cage_base, entry, data_offset)
    }

    pub fn LoadFromDataTable_base(&self, cage_base: PtrComprCageBase, entry: i32, data_offset: i32) -> Tagged<Object> {
        assert!(entry as u32 <= self.Capacity() as u32);

        let offset = self.DataTableStartOffset() +
            ((entry as usize) * (SwissNameDictionary::kDataTableEntryCount as usize) + (data_offset as usize)) * size_of::<usize>();

        unsafe {
            TaggedField::<Object, 0>::Relaxed_Load(cage_base, *self, offset)
        }
    }

    pub fn StoreToDataTable(&mut self, entry: i32, data_offset: i32, data: Tagged<Object>) {
        assert!(entry as u32 <= self.Capacity() as u32);

        let offset = self.DataTableStartOffset() +
            ((entry as usize) * (SwissNameDictionary::kDataTableEntryCount as usize) + (data_offset as usize)) * size_of::<usize>();

        unsafe {
            let addr = self.heap_object.address().add(offset).cast_mut::<usize>();
            addr.write(data.ptr() as usize);
        }

    }

    pub fn StoreToDataTableNoBarrier(&mut self, entry: i32, data_offset: i32, data: Tagged<Object>) {
        assert!(entry as u32 <= self.Capacity() as u32);
        let offset = self.DataTableStartOffset() +
            ((entry as usize) * (SwissNameDictionary::kDataTableEntryCount as usize) + (data_offset as usize)) * size_of::<usize>();

        unsafe {
            let addr = self.heap_object.address().add(offset).cast_mut::<usize>();
            addr.write(data.ptr() as usize);
        }
    }

    pub fn ClearDataTableEntry(&mut self, isolate: &Isolate, entry: i32) {
        let roots = ReadOnlyRoots {};

        self.StoreToDataTable(entry, SwissNameDictionary::kDataTableKeyEntryIndex, roots.the_hole_value());
        self.StoreToDataTable(entry, SwissNameDictionary::kDataTableValueEntryIndex, roots.the_hole_value());
    }

    pub fn ValueAtPut(&mut self, entry: i32, value: Tagged<Object>) {
        assert!(!self.IsTheHole(value));
        self.StoreToDataTable(entry, SwissNameDictionary::kDataTableValueEntryIndex, value);
    }

    pub fn SetKey(&mut self, entry: i32, key: Tagged<Object>) {
        assert!(!self.IsTheHole(key));
        self.StoreToDataTable(entry, SwissNameDictionary::kDataTableKeyEntryIndex, key);
    }

    pub fn DetailsAtPut(&mut self, entry: i32, details: PropertyDetails) {
        assert!(entry as u32 <= self.Capacity() as u32);
        let encoded_details = details.ToByte();
        unsafe {
            self.PropertyDetailsTable().add(entry as usize).write(encoded_details);
        }
    }

    pub fn KeyAt(&self, entry: i32) -> Tagged<Object> {
        self.LoadFromDataTable(entry, SwissNameDictionary::kDataTableKeyEntryIndex)
    }

    pub fn ValueAtRaw(&self, entry: i32) -> Tagged<Object> {
        self.LoadFromDataTable(entry, SwissNameDictionary::kDataTableValueEntryIndex)
    }

    pub fn ValueAt(&self, entry: InternalIndex) -> Tagged<Object> {
        assert!(self.IsFull(self.GetCtrl(entry.as_int())));
        self.ValueAtRaw(entry.as_int())
    }

    pub fn TryValueAt(&self, entry: InternalIndex) -> Option<Tagged<Object>> {
        if entry.as_int() as u32 >= self.Capacity() as u32 {
            return None;
        }
        Some(self.ValueAtRaw(entry.as_int()))
    }

    pub fn DetailsAt(&self, entry: i32) -> PropertyDetails {
        assert!(self.IsFull(self.GetCtrl(entry)));
        let encoded_details = unsafe { self.PropertyDetailsTable().add(entry as usize).read() };
        PropertyDetails::FromByte(encoded_details)
    }

    pub fn GetCtrl(&self, entry: i32) -> u8 {
        assert!(entry as u32 <= self.Capacity() as u32);
        unsafe { *self.CtrlTable().add(entry as usize) }
    }

    pub fn SetCtrl(&mut self, entry: i32, h: u8) {
        let capacity = self.Capacity();
        assert!(entry as u32 <= capacity as u32);

        let ctrl = self.CtrlTable();
        unsafe {
            ctrl.add(entry as usize).write(h);

            let mask = capacity - 1;
            let copy_entry =
                ((entry - Group::kWidth as i32) & mask) + 1 + ((Group::kWidth as i32 - 1) & mask);

            ctrl.add(copy_entry as usize).write(h);
        }
    }

    pub fn FindFirstEmpty(&self, hash: u32) -> i32 {
        let mut seq = self.probe(hash, self.Capacity());
        loop {
            let g = Group {
                ctrl: unsafe { self.CtrlTable().add(seq.offset()) },
            };
            let mask = g.MatchEmpty();
            if mask != 0 {
                return seq.offset(mask.trailing_zeros() as usize);
            }
            seq.next();
            assert!(seq.index() < self.Capacity() as usize);
        }
    }

    pub fn SetMetaTableField(&mut self, field_index: i32, value: i32) {
        let capacity = self.Capacity();
        let meta_table = self.meta_table();
        if capacity <= SwissNameDictionary::kMax1ByteMetaTableCapacity {
            Self::SetMetaTableField_u8(meta_table, field_index, value);
        } else if capacity <= SwissNameDictionary::kMax2ByteMetaTableCapacity {
            Self::SetMetaTableField_u16(meta_table, field_index, value);
        } else {
            Self::SetMetaTableField_u32(meta_table, field_index, value);
        }
    }

    fn SetMetaTableField_u8(meta_table: Tagged<ByteArray>, field_index: i32, value: i32) {
        assert!(value <= u8::MAX as i32);
        let raw_data = unsafe { meta_table.ptr().add(ByteArray::kHeaderSize).cast::<u8>() };
        unsafe { raw_data.add(field_index as usize).write(value as u8) }
    }

    fn SetMetaTableField_u16(meta_table: Tagged<ByteArray>, field_index: i32, value: i32) {
        assert!(value <= u16::MAX as i32);
        let raw_data = unsafe { meta_table.ptr().add(ByteArray::kHeaderSize).cast::<u16>() };
        unsafe { raw_data.add(field_index as usize).write(value as u16) }
    }

    fn SetMetaTableField_u32(meta_table: Tagged<ByteArray>, field_index: i32, value: i32) {
        let raw_data = unsafe { meta_table.ptr().add(ByteArray::kHeaderSize).cast::<u32>() };
        unsafe { raw_data.add(field_index as usize).write(value as u32) }
    }

    pub fn GetMetaTableField(&self, field_index: i32) -> i32 {
        let capacity = self.Capacity();
        let meta_table = self.meta_table();
        if capacity <= SwissNameDictionary::kMax1ByteMetaTableCapacity {
            Self::GetMetaTableField_u8(meta_table, field_index)
        } else if capacity <= SwissNameDictionary::kMax2ByteMetaTableCapacity {
            Self::GetMetaTableField_u16(meta_table, field_index)
        } else {
            Self::GetMetaTableField_u32(meta_table, field_index)
        }
    }

    fn GetMetaTableField_u8(meta_table: Tagged<ByteArray>, field_index: i32) -> i32 {
        let raw_data = unsafe { meta_table.ptr().add(ByteArray::kHeaderSize).cast::<u8>() };
        unsafe { *raw_data.add(field_index as usize) as i32 }
    }

    fn GetMetaTableField_u16(meta_table: Tagged<ByteArray>, field_index: i32) -> i32 {
        let raw_data = unsafe { meta_table.ptr().add(ByteArray::kHeaderSize).cast::<u16>() };
        unsafe { *raw_data.add(field_index as usize) as i32 }
    }

    fn GetMetaTableField_u32(meta_table: Tagged<ByteArray>, field_index: i32) -> i32 {
        let raw_data = unsafe { meta_table.ptr().add(ByteArray::kHeaderSize).cast::<u32>() };
        unsafe { *raw_data.add(field_index as usize) as i32 }
    }

    pub fn MetaTableSizePerEntryFor(&self, capacity: i32) -> usize {
        assert!(self.IsValidCapacity(capacity));
        if capacity <= SwissNameDictionary::kMax1ByteMetaTableCapacity {
            size_of::<u8>()
        } else if capacity <= SwissNameDictionary::kMax2ByteMetaTableCapacity {
            size_of::<u16>()
        } else {
            size_of::<u32>()
        }
    }

    pub fn MetaTableSizeFor(&self, capacity: i32) -> usize {
        assert!(self.IsValidCapacity(capacity));
        let per_entry_size = self.MetaTableSizePerEntryFor(capacity);
        (self.MaxUsableCapacity(capacity) + 2) as usize * per_entry_size
    }

    pub fn IsKey(&self, roots: ReadOnlyRoots, key_candidate: Tagged<Object>) -> bool {
        key_candidate != roots.the_hole_value()
    }

    pub fn ToKey(&self, roots: ReadOnlyRoots, entry: i32, out_key: &mut Tagged<Object>) -> bool {
        let k = self.KeyAt(entry);
        if !self.IsKey(roots, k) {
            return false;
        }
        *out_key = k;
        true
    }

    pub fn AddInternal(&mut self, key: Tagged<Name>, value: Tagged<Object>, details: PropertyDetails) -> i32 {
        assert!(self.IsUniqueName(key));
        assert!(self.UsedCapacity() <= self.MaxUsableCapacity(self.Capacity()));

        let hash = key.hash();

        let target = self.FindFirstEmpty(hash);

        self.SetCtrl(target, SwissNameDictionary::H2(hash));
        self.SetKey(target, key);
        self.ValueAtPut(target, value);
        self.DetailsAtPut(target, details);

        target
    }

    pub fn Initialize(&mut self, isolate: &Isolate, meta_table: Tagged<ByteArray>, capacity: i32) {
        assert!(self.IsValidCapacity(capacity));

        let roots = ReadOnlyRoots {};

        self.SetCapacity(capacity);
        self.SetHash(PropertyArray::kNoHashSentinel);

        unsafe {
            ptr::write_bytes(self.CtrlTable(), Ctrl::kEmpty, self.CtrlTableSize(capacity));
        }

        let hole = roots.the_hole_value();

        for i in 0..capacity {
            self.StoreToDataTable(i, SwissNameDictionary::kDataTableKeyEntryIndex, hole);
            self.StoreToDataTable(i, SwissNameDictionary::kDataTableValueEntryIndex, hole);
        }

        self.set_meta_table(meta_table);

        self.SetNumberOfElements(0);
        self.SetNumberOfDeletedElements(0);
    }

    pub fn SetHash(&mut self, hash: i32) {
        let offset = self.PrefixOffset();
        unsafe { (self.heap_object.address().add(offset) as *mut i32).write(hash) };
    }

    pub fn Hash(&self) -> i32 {
        let offset = self.PrefixOffset();
        unsafe { (self.heap_object.address().add(offset) as *const i32).read() }
    }

    pub const fn PrefixOffset(&self) -> usize {
        HeapObject::kHeaderSize
    }

    pub const fn CapacityOffset(&self) -> usize {
        self.PrefixOffset() + size_of::<u32>()
    }

    pub const fn MetaTablePointerOffset(&self) -> usize {
        self.CapacityOffset() + size_of::<i32>()
    }

    pub const fn DataTableStartOffset(&self) -> usize {
        self.MetaTablePointerOffset() + size_of::<usize>()
    }

    pub fn DataTableEndOffset(&self, capacity: i32) -> usize {
        self.CtrlTableStartOffset(capacity)
    }

    pub fn CtrlTableStartOffset(&self, capacity: i32) -> usize {
        self.DataTableStartOffset() + self.DataTableSize(capacity)
    }

    pub fn PropertyDetailsTableStartOffset(&self, capacity: i32) -> usize {
        self.CtrlTableStartOffset(capacity) + self.CtrlTableSize(capacity)
    }

    pub const fn MaxCapacity(&self) -> i32 {
        let kConstSize = self.DataTableStartOffset() + ByteArray::kHeaderSize + 2 * size_of::<u32>();
        let kPerEntrySize =
            SwissNameDictionary::kDataTableEntryCount as usize * size_of::<usize>() + size_of::<u8>() + size_of::<u8>() + size_of::<u32>();

        let result = (FixedArray::kMaxRegularHeapObjectSize - kConstSize) / kPerEntrySize;
        assert!(Smi::kMaxValue as usize >= result);

        result as i32
    }

    pub fn IsEmpty(&self, c: u8) -> bool {
        c == Ctrl::kEmpty
    }

    pub fn IsFull(&self, c: u8) -> bool {
        c >= 0x80
    }

    pub fn IsDeleted(&self, c: u8) -> bool {
        c == Ctrl::kDeleted
    }

    pub fn IsEmptyOrDeleted(&self, c: u8) -> bool {
        c < 0x01
    }

    pub fn probe(&self, hash: u32, capacity: i32) -> swiss_table::ProbeSequence< {SwissNameDictionary::kGroupWidth}> {
        let non_zero_capacity = if capacity == 0 { 1 } else { capacity };
        swiss_table::ProbeSequence::< {SwissNameDictionary::kGroupWidth}>::new(
            SwissNameDictionary::H1(hash),
            (non_zero_capacity - 1) as u32
        )
    }

    pub fn meta_table(&self) -> Tagged<ByteArray> {
        let offset = self.MetaTablePointerOffset();
         unsafe { TaggedField::<ByteArray, 0>::Relaxed_Load(PtrComprCageBase{}, *self, offset) }
    }

    pub fn set_meta_table(&mut self, value: Tagged<ByteArray>) {
        let offset = self.MetaTablePointerOffset();
        unsafe {
            let addr = self.heap_object.address().add(offset).cast_mut::<usize>();
            addr.write(value.ptr() as usize);
        }
    }

    fn H1(hash: u32) -> u32 {
        hash & 0x7fffffff
    }

    fn H2(hash: u32) -> u8 {
        (hash >> 24) as u8 | 0x80
    }

    fn IsUniqueName(&self, name: Tagged<Name>) -> bool {
        true
    }

    fn IsTheHole(&self, object: Tagged<Object>) -> bool {
        false
    }

}

pub mod swiss_table {
    use std::num::NonZeroU32;

    pub type ctrl_t = u8;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ProbeSequence<const GROUP_WIDTH: usize> {
        h1_: u32,
        i_: u32,
        modulo_mask_: u32,
    }

    impl<const GROUP_WIDTH: usize> ProbeSequence<GROUP_WIDTH> {
        pub fn new(h1: u32, modulo_mask: u32) -> Self {
            ProbeSequence {
                h1_: h1,
                i_: 0,
                modulo_mask_: modulo_mask,
            }
        }

        #[inline(always)]
        pub fn offset(&self, index: usize) -> usize {
            (self.h1_.wrapping_add((index as u32).wrapping_mul(self.i_)) & self.modulo_mask_) as usize
        }

        #[inline(always)]
        pub fn next(&mut self) {
            self.i_ = self.i_.wrapping_add(5);
        }

        #[inline(always)]
        pub fn index(&self) -> usize {
            self.h1_ as usize
        }
    }
}

impl SwissNameDictionary {
    pub const kDataTableKeyEntryIndex: i32 = 0;
    pub const kDataTableValueEntryIndex: i32 = 1;
    pub const kDataTableEntryCount: i32 = 2;
    pub const kMax1ByteMetaTableCapacity: i32 = 255;
    pub const kMax2ByteMetaTableCapacity: i32 = 65535;
    pub const kGroupWidth: usize = 8;

    fn H1(hash: u32) -> u32 {
        hash & 0x7fffffff
    }

    fn H2(hash: u32) -> u8 {
        (hash >> 24) as u8 | 0x80
    }
}

impl SwissNameDictionaryImpl {
    fn get_direct_handle(&self, isolate: &Isolate) -> DirectHandle<SwissNameDictionaryImpl> {
        DirectHandle {
            value: *self,
            isolate: std::marker::PhantomData,
        }
    }
}

#[derive(Clone, Copy)]
struct DirectHandle<T> {
    value: T,
    isolate: std::marker::PhantomData<Isolate>,
}

impl<T> DirectHandle<T> {
    fn null() -> Self {
        DirectHandle {
            value: unsafe { std::mem::zeroed() },
            isolate: std::marker::PhantomData,
        }
    }

    fn equals(&self, other: &DirectHandle<T>) -> bool {
        unsafe { std::mem::transmute::<&T, usize>(&self.value) == std::mem::transmute::<&T, usize>(&other.value) }
    }

    fn is_null(&self) -> bool {
        unsafe { std::mem::transmute::<&T, usize>(&self.value) == 0 }
    }
}

impl DirectHandle<SwissNameDictionaryImpl> {
    fn Add(
        isolate: &Isolate,
        original_table: DirectHandle<SwissNameDictionaryImpl>,
        key: DirectHandle<Name>,
        value: DirectHandle<Object>,
        details: PropertyDetails,
        entry_out: Option<&mut InternalIndex>,
    ) -> DirectHandle<SwissNameDictionaryImpl> {
        assert!(original_table.value.FindEntry(isolate, unsafe { *key }).is_not_found());

        let table = Self::EnsureGrowable(isolate, original_table);
        let mut raw_table = table.value;
        let nof = raw_table.NumberOfElements();
        let nod = raw_table.NumberOfDeletedElements();
        let new_enum_index = nof + nod;

        let new_entry = raw_table.AddInternal(unsafe { *key }, unsafe { *value }, details);

        raw_table.SetNumberOfElements(nof + 1);
        raw_table.SetEntryForEnumerationIndex(new_enum_index, new_entry);

        if let Some(entry_out) = entry_out {
            *entry_out = InternalIndex(new_entry);
        }

        table
    }

    fn EnsureGrowable(
        isolate: &Isolate,
        table: DirectHandle<SwissNameDictionaryImpl>,
    ) -> DirectHandle<SwissNameDictionaryImpl> {
        let capacity = table.value.Capacity();

        if table.value.UsedCapacity() < table.value.MaxUsableCapacity(capacity) {
            return table;
        }

        let new_capacity = if capacity == 0 {
            8
        } else {
            capacity * 2
        };
        Self::Rehash(isolate, table, new_capacity)
    }

    fn Rehash(
        isolate: &Isolate,
        table: DirectHandle<SwissNameDictionaryImpl>,
        new_capacity: i32,
    ) -> DirectHandle<SwissNameDictionaryImpl> {
        DirectHandle::null()
    }
}

struct DisallowGarbageCollection {}

struct RawField<const OFFSET: usize>;

struct MemsetTagged {}

impl SwissNameDictionaryImpl {
    fn IterateEntriesOrdered(&self) -> IndexIterable {
        if self.Capacity() == 0 {
            return IndexIterable {
                dict_: DirectHandle::null()
            };
        }

        IndexIterable {
            dict_: self.get_direct_handle(&Isolate{})
        }
    }
}

struct IndexIterable {
    dict_: DirectHandle<SwissNameDictionaryImpl>
}

struct IndexIterator {
    enum_index_: i32,
    dict_: DirectHandle<SwissNameDictionaryImpl>,
    used_capacity_: i32
}

impl IndexIterator {
    fn new(dict: DirectHandle<SwissNameDictionaryImpl>, start: i32) -> Self {
        let used_capacity_ = if dict.is_null() {
            0
        } else {
            dict.value.UsedCapacity()
        };

        IndexIterator {
            enum_index_: start,
            dict_: dict,
            used_capacity_: used_capacity_
        }
    }

    fn next(&mut self) -> &mut Self {
        assert!(self.enum_index_ < self.used_capacity_);
        self.enum_index_ += 1;
        self
    }

    fn equals(&self, other: &IndexIterator) -> bool {
        assert!(self.enum_index_ <= self.used_capacity_);
        assert!(other.enum_index_ <= self.used_capacity_);

        self.dict_.equals(&other.dict_) && self.enum_index_ == other.enum_index_
    }

    fn deref(&self) -> InternalIndex {
        assert!(self.enum_index_ <= self.used_capacity_);

        if self.enum_index_ == self.used_capacity_ {
            return InternalIndex::NotFound()
        }

        InternalIndex(self.dict_.value.EntryForEnumerationIndex(self.enum_index_))
    }
}

impl IndexIterable {
    fn begin(&self) -> IndexIterator {
        IndexIterator::new(self.dict_, 0)
    }

    fn end(&self) -> IndexIterator {
        if self.dict_.is_null() {
            IndexIterator::new(self.dict_, 0)
        } else {
            IndexIterator::new(self.dict_, self.dict_.value.UsedCapacity())
        }
    }
}
