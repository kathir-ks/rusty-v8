// Copyright 2015 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// TODO: Add Rust equivalents for missing V8 types and functions.
// This is a placeholder for the AstRawString type
//pub type AstRawString = String;
// This is a placeholder for the AstConsString type
//pub type AstConsString = String;
// This is a placeholder for the AstBigInt type
//pub type AstBigInt = String;
// This is a placeholder for the Scope type
//pub type Scope = i32;
// This is a placeholder for the Tagged<Smi> type
//pub type Smi = i32;
// This is a placeholder for the Tagged<Object> type
//pub type Object = i32;
// This is a placeholder for the Handle<Object> type
//pub type HandleObject = i32;
// This is a placeholder for the MaybeHandle<Object> type
//pub type MaybeHandleObject = i32;
// This is a placeholder for the TrustedFixedArray type
//pub type TrustedFixedArray = Vec<i32>;
// This is a placeholder for the Isolate type
//pub type Isolate = i32;
// This is a placeholder for the LocalIsolate type
//pub type LocalIsolate = i32;
// This is a placeholder for the Zone type, a memory arena
//pub type Zone = i32;

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::size_of;

// Define the OperandSize enum
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OperandSize {
    None,
    Byte,
    Short,
    Quad,
}

// Define the ConstantArrayBuilder module
pub mod constant_array_builder {
    use super::*;

    // Define the Entry struct
    #[derive(Debug, Clone, Copy)]
    pub struct Entry {
        tag_: Tag,
        smi_: Smi,
        heap_number_: f64,
        raw_string_: *const String,
        cons_string_: *const String,
        bigint_: *const String,
        scope_: *const i32, //Scope
        handle_: HandleObject,
    }

    impl Entry {
        pub fn smi(smi: Smi) -> Self {
            Entry {
                tag_: Tag::kSmi,
                smi_: smi,
                heap_number_: 0.0,
                raw_string_: std::ptr::null(),
                cons_string_: std::ptr::null(),
                bigint_: std::ptr::null(),
                scope_: std::ptr::null(),
                handle_: 0,
            }
        }

        pub fn raw_string(raw_string: *const String) -> Self {
            Entry {
                tag_: Tag::kRawString,
                smi_: 0,
                heap_number_: 0.0,
                raw_string_: raw_string,
                cons_string_: std::ptr::null(),
                bigint_: std::ptr::null(),
                scope_: std::ptr::null(),
                handle_: 0,
            }
        }

        pub fn cons_string(cons_string: *const String) -> Self {
            Entry {
                tag_: Tag::kConsString,
                smi_: 0,
                heap_number_: 0.0,
                raw_string_: std::ptr::null(),
                cons_string_: cons_string,
                bigint_: std::ptr::null(),
                scope_: std::ptr::null(),
                handle_: 0,
            }
        }

        pub fn bigint(bigint: *const String) -> Self {
            Entry {
                tag_: Tag::kBigInt,
                smi_: 0,
                heap_number_: 0.0,
                raw_string_: std::ptr::null(),
                cons_string_: std::ptr::null(),
                bigint_: bigint,
                scope_: std::ptr::null(),
                handle_: 0,
            }
        }

        pub fn scope(scope: *const i32) -> Self {
            Entry {
                tag_: Tag::kScope,
                smi_: 0,
                heap_number_: 0.0,
                raw_string_: std::ptr::null(),
                cons_string_: std::ptr::null(),
                bigint_: std::ptr::null(),
                scope_: scope,
                handle_: 0,
            }
        }
        pub fn heap_number(heap_number: f64) -> Self {
            Entry {
                tag_: Tag::kHeapNumber,
                smi_: 0,
                heap_number_: heap_number,
                raw_string_: std::ptr::null(),
                cons_string_: std::ptr::null(),
                bigint_: std::ptr::null(),
                scope_: std::ptr::null(),
                handle_: 0,
            }
        }
        pub fn deferred() -> Self {
            Entry {
                tag_: Tag::kDeferred,
                smi_: 0,
                heap_number_: 0.0,
                raw_string_: std::ptr::null(),
                cons_string_: std::ptr::null(),
                bigint_: std::ptr::null(),
                scope_: std::ptr::null(),
                handle_: 0,
            }
        }

        pub fn uninitialized_jump_table_smi() -> Self {
            Entry {
                tag_: Tag::kUninitializedJumpTableSmi,
                smi_: 0,
                heap_number_: 0.0,
                raw_string_: std::ptr::null(),
                cons_string_: std::ptr::null(),
                bigint_: std::ptr::null(),
                scope_: std::ptr::null(),
                handle_: 0,
            }
        }

        pub fn is_deferred(&self) -> bool {
            self.tag_ == Tag::kDeferred
        }

        pub fn set_deferred(&mut self, object: HandleObject) {
            self.tag_ = Tag::kHandle;
            self.handle_ = object;
        }
        pub fn set_jump_table_smi(&mut self, smi: Smi) {
            self.tag_ = Tag::kJumpTableSmi;
            self.smi_ = smi;
        }

        // TODO: Implement ToHandle using Isolate
        //pub fn to_handle(&self, isolate: &Isolate) -> HandleObject {
        //  match self.tag_ {
        //      Tag::kDeferred => panic!("We shouldn't have any deferred entries by now."),
        //      Tag::kHandle => self.handle_,
        //      Tag::kSmi | Tag::kJumpTableSmi => todo!(), //Handle::from_raw(self.smi_ as *mut Smi) , //Smi::new(self.smi_).into(),
        //      Tag::kUninitializedJumpTableSmi => todo!(),//isolate.factory().the_hole_value(),
        //      Tag::kRawString => unsafe { *(self.raw_string_) },
        //      Tag::kConsString => unsafe { *(self.cons_string_) },
        //      Tag::kHeapNumber => self.heap_number_ as i32,
        //      Tag::kBigInt => unsafe { *(self.bigint_) },
        //      Tag::kScope => unsafe { *(self.scope_) },
        //      _ => panic!("Unreachable"),
        //  }
        //}
    }

    // Define the Tag enum
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Tag {
        kDeferred,
        kHandle,
        kSmi,
        kJumpTableSmi,
        kUninitializedJumpTableSmi,
        kRawString,
        kConsString,
        kHeapNumber,
        kBigInt,
        kScope,
        kUndefined,
        kNull,
        kTrue,
        kFalse,
        kTheHole,
    }

    impl fmt::Display for Tag {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    impl Entry {
        pub fn undefined() -> Self {
            Entry {
                tag_: Tag::kUndefined,
                smi_: 0,
                heap_number_: 0.0,
                raw_string_: std::ptr::null(),
                cons_string_: std::ptr::null(),
                bigint_: std::ptr::null(),
                scope_: std::ptr::null(),
                handle_: 0,
            }
        }
        pub fn null() -> Self {
            Entry {
                tag_: Tag::kNull,
                smi_: 0,
                heap_number_: 0.0,
                raw_string_: std::ptr::null(),
                cons_string_: std::ptr::null(),
                bigint_: std::ptr::null(),
                scope_: std::ptr::null(),
                handle_: 0,
            }
        }
        pub fn true_val() -> Self {
            Entry {
                tag_: Tag::kTrue,
                smi_: 0,
                heap_number_: 0.0,
                raw_string_: std::ptr::null(),
                cons_string_: std::ptr::null(),
                bigint_: std::ptr::null(),
                scope_: std::ptr::null(),
                handle_: 0,
            }
        }
        pub fn false_val() -> Self {
            Entry {
                tag_: Tag::kFalse,
                smi_: 0,
                heap_number_: 0.0,
                raw_string_: std::ptr::null(),
                cons_string_: std::ptr::null(),
                bigint_: std::ptr::null(),
                scope_: std::ptr::null(),
                handle_: 0,
            }
        }
        pub fn the_hole() -> Self {
            Entry {
                tag_: Tag::kTheHole,
                smi_: 0,
                heap_number_: 0.0,
                raw_string_: std::ptr::null(),
                cons_string_: std::ptr::null(),
                bigint_: std::ptr::null(),
                scope_: std::ptr::null(),
                handle_: 0,
            }
        }
    }

    // Define the ConstantArraySlice struct
    pub struct ConstantArraySlice {
        start_index_: usize,
        capacity_: usize,
        reserved_: usize,
        operand_size_: OperandSize,
        constants_: Vec<Entry>,
    }

    impl ConstantArraySlice {
        pub fn new(zone: Zone, start_index: usize, capacity: usize, operand_size: OperandSize) -> Self {
            ConstantArraySlice {
                start_index_: start_index,
                capacity_: capacity,
                reserved_: 0,
                operand_size_: operand_size,
                constants_: Vec::new(),
            }
        }

        pub fn reserve(&mut self) {
            assert!(self.available() > 0);
            self.reserved_ += 1;
            assert!(self.reserved_ <= self.capacity() - self.constants_.len());
        }

        pub fn unreserve(&mut self) {
            assert!(self.reserved_ > 0);
            self.reserved_ -= 1;
        }

        pub fn allocate(&mut self, entry: Entry, count: usize) -> usize {
            assert!(self.available() >= count);
            let index = self.constants_.len();
            assert!(index < self.capacity());
            for _ in 0..count {
                self.constants_.push(entry);
            }
            index + self.start_index()
        }

        pub fn at(&mut self, index: usize) -> &mut Entry {
            assert!(index >= self.start_index());
            assert!(index < self.start_index() + self.size());
            &mut self.constants_[index - self.start_index()]
        }

        pub fn at_const(&self, index: usize) -> &Entry {
            assert!(index >= self.start_index());
            assert!(index < self.start_index() + self.size());
            &self.constants_[index - self.start_index()]
        }

        pub fn start_index(&self) -> usize {
            self.start_index_
        }

        pub fn capacity(&self) -> usize {
            self.capacity_
        }

        pub fn size(&self) -> usize {
            self.constants_.len()
        }

        pub fn available(&self) -> usize {
            self.capacity() - self.size() - self.reserved_
        }

        pub fn reserved(&self) -> usize {
            self.reserved_
        }
        pub fn max_index(&self) -> usize {
            self.start_index_ + self.capacity_
        }
        pub fn operand_size(&self) -> OperandSize {
            self.operand_size_
        }

        // TODO: Implement CheckAllElementsAreUnique using Isolate
        //#[cfg(debug_assertions)]
        //pub fn check_all_elements_are_unique(&self, isolate: &Isolate) {
        //    let mut smis: HashSet<Smi> = HashSet::new();
        //    let mut heap_numbers: HashSet<f64> = HashSet::new();
        //    let mut strings: HashSet<*const AstRawString> = HashSet::new();
        //    let mut cons_strings: HashSet<*const AstConsString> = HashSet::new();
        //    let mut bigints: HashSet<*const String> = HashSet::new();
        //    let mut scopes: HashSet<*const Scope> = HashSet::new();
        //    //let mut deferred_objects: HashSet<Object> = HashSet::new();
        //
        //    for entry in &self.constants_ {
        //        let mut duplicate = false;
        //        match entry.tag_ {
        //            Tag::kSmi => {
        //                duplicate = !smis.insert(entry.smi_);
        //            }
        //            Tag::kHeapNumber => {
        //                duplicate = !heap_numbers.insert(entry.heap_number_);
        //            }
        //            Tag::kRawString => {
        //                duplicate = !strings.insert(entry.raw_string_);
        //            }
        //            Tag::kConsString => {
        //                duplicate = !cons_strings.insert(entry.cons_string_);
        //            }
        //            Tag::kBigInt => {
        //                unsafe { duplicate = !bigints.insert(entry.bigint_) };
        //            }
        //            Tag::kScope => {
        //                duplicate = !scopes.insert(entry.scope_);
        //            }
        //            //Tag::kHandle => {
        //            //    duplicate = !deferred_objects.insert(entry.handle_);
        //            //}
        //            Tag::kDeferred => panic!("Should be kHandle at this point."),
        //            Tag::kJumpTableSmi | Tag::kUninitializedJumpTableSmi => {
        //                // TODO(leszeks): Ignore jump tables because they have to be contiguous,
        //                // so they can contain duplicates.
        //            }
        //            Tag::kUndefined | Tag::kNull | Tag::kTrue | Tag::kFalse | Tag::kTheHole => {
        //                // Singletons are non-duplicated by definition.
        //            }
        //            _ => (),
        //        }
        //
        //        if duplicate {
        //            let mut os = String::new();
        //            os.push_str("Duplicate constant found: ");
        //            //os.push_str(entry.to_handle(isolate).to_string().as_str());
        //            os.push_str("\n");
        //
        //            let mut i = self.start_index();
        //            for prev_entry in &self.constants_ {
        //                //os.push_str(format!("{}: {}\n", i, prev_entry.to_handle(isolate)).as_str());
        //                i += 1;
        //            }
        //            panic!("{}", os);
        //        }
        //    }
        //}
    }

    // Define the ConstantArrayBuilder struct
    pub struct ConstantArrayBuilder {
        constants_map_: HashMap<i64, usize>,
        smi_map_: HashMap<Smi, usize>,
        smi_pairs_: Vec<(Smi, Smi)>,
        heap_number_map_: HashMap<f64, usize>,
        idx_slice_: [ConstantArraySlice; 3],
        undefined_: isize,
        null_: isize,
        true_value_: isize,
        false_value_: isize,
        the_hole_: isize,
    }

    impl ConstantArrayBuilder {
        pub const K8_BIT_CAPACITY: usize = 256;
        pub const K16_BIT_CAPACITY: usize = 65536 - ConstantArrayBuilder::K8_BIT_CAPACITY;
        pub const K32_BIT_CAPACITY: usize = 16777216 - ConstantArrayBuilder::K8_BIT_CAPACITY - ConstantArrayBuilder::K16_BIT_CAPACITY;

        pub fn new(zone: Zone) -> Self {
            let slice0 = ConstantArraySlice::new(zone, 0, ConstantArrayBuilder::K8_BIT_CAPACITY, OperandSize::Byte);
            let slice1 = ConstantArraySlice::new(
                zone,
                ConstantArrayBuilder::K8_BIT_CAPACITY,
                ConstantArrayBuilder::K16_BIT_CAPACITY,
                OperandSize::Short,
            );
            let slice2 = ConstantArraySlice::new(
                zone,
                ConstantArrayBuilder::K8_BIT_CAPACITY + ConstantArrayBuilder::K16_BIT_CAPACITY,
                ConstantArrayBuilder::K32_BIT_CAPACITY,
                OperandSize::Quad,
            );

            ConstantArrayBuilder {
                constants_map_: HashMap::new(),
                smi_map_: HashMap::new(),
                smi_pairs_: Vec::new(),
                heap_number_map_: HashMap::new(),
                idx_slice_: [slice0, slice1, slice2],
                undefined_: -1,
                null_: -1,
                true_value_: -1,
                false_value_: -1,
                the_hole_: -1,
            }
        }

        pub fn size(&self) -> usize {
            let mut i = self.idx_slice_.len();
            while i > 0 {
                i -= 1;
                let slice = &self.idx_slice_[i];
                if slice.size() > 0 {
                    return slice.start_index() + slice.size();
                }
            }
            self.idx_slice_[0].size()
        }

        fn index_to_slice(&self, index: usize) -> &ConstantArraySlice {
            for slice in &self.idx_slice_ {
                if index <= slice.max_index() {
                    return slice;
                }
            }
            panic!("UNREACHABLE");
        }

        // TODO: Implement At using Isolate
        //pub fn at(&self, index: usize, isolate: &Isolate) -> Option<HandleObject> {
        //    let slice = self.index_to_slice(index);
        //    assert!(index < slice.capacity());
        //    if index < slice.start_index() + slice.size() {
        //        let entry = slice.at_const(index);
        //        if !entry.is_deferred() {
        //            return Some(entry.to_handle(isolate));
        //        }
        //    }
        //    None
        //}

        // TODO: Implement ToFixedArray using Isolate
        //pub fn to_fixed_array(&mut self, isolate: &Isolate) -> TrustedFixedArray {
        //    let mut fixed_array: TrustedFixedArray = vec![0; self.size()]; //isolate.factory().new_trusted_fixed_array(self.size() as i32);
        //    //MemsetTagged(fixed_array.raw_field_of_first_element(), isolate.factory().the_hole_value(), self.size());
        //
        //    let mut array_index = 0;
        //    for slice in &mut self.idx_slice_ {
        //        assert_eq!(slice.reserved(), 0);
        //        assert!(array_index == 0 || array_index.is_power_of_two());
        //
        //        #[cfg(debug_assertions)]
        //        slice.check_all_elements_are_unique(isolate);
        //
        //        for i in 0..slice.size() {
        //            //let value = slice.at(slice.start_index() + i).to_handle(isolate);
        //            //fixed_array[array_index] = value; //fixed_array.set(array_index, value);
        //            array_index += 1;
        //        }
        //
        //        let padding = slice.capacity() - slice.size();
        //        if (fixed_array.len() - array_index) <= padding {
        //            break;
        //        }
        //        array_index += padding;
        //    }
        //
        //    assert!(array_index >= fixed_array.len());
        //    fixed_array
        //}

        pub fn insert(&mut self, smi: Smi) -> usize {
            if let Some(&index) = self.smi_map_.get(&smi) {
                return index;
            }
            self.allocate_reserved_entry(smi)
        }

        pub fn insert_number(&mut self, number: f64) -> usize {
            if number.is_nan() {
                return self.insert_nan();
            }
            if let Some(&index) = self.heap_number_map_.get(&number) {
                return index;
            }
            let index = self.allocate_index(Entry::heap_number(number)) as usize;
            self.heap_number_map_.insert(number, index);
            index
        }

        pub fn insert_raw_string(&mut self, raw_string: *const String) -> usize {
            let hash = unsafe { raw_string.hash(&mut std::collections::hash_map::DefaultHasher::new()) } as u32;
            *self
                .constants_map_
                .entry(raw_string as i64)
                .or_insert_with(|| self.allocate_index(Entry::raw_string(raw_string)) as usize)
        }

        pub fn insert_cons_string(&mut self, cons_string: *const String) -> usize {
            let hash = unsafe {
                let s = &*cons_string;
                s.hash(&mut std::collections::hash_map::DefaultHasher::new())
            } as u32;

            *self
                .constants_map_
                .entry(cons_string as i64)
                .or_insert_with(|| self.allocate_index(Entry::cons_string(cons_string)) as usize)
        }

        pub fn insert_bigint(&mut self, bigint: *const String) -> usize {
            let hash = unsafe {
                let s = &*bigint;
                s.hash(&mut std::collections::hash_map::DefaultHasher::new())
            } as u32;

            *self
                .constants_map_
                .entry(bigint as i64)
                .or_insert_with(|| self.allocate_index(Entry::bigint(bigint)) as usize)
        }

        pub fn insert_scope(&mut self, scope: *const i32) -> usize {
            let hash = unsafe {
                let s = &*scope;
                s.hash(&mut std::collections::hash_map::DefaultHasher::new())
            } as u32;

            *self
                .constants_map_
                .entry(scope as i64)
                .or_insert_with(|| self.allocate_index(Entry::scope(scope)) as usize)
        }

        pub fn insert_undefined(&mut self) -> usize {
            if self.undefined_ < 0 {
                self.undefined_ = self.allocate_index(Entry::undefined()) as isize;
            }
            self.undefined_ as usize
        }

        pub fn insert_null(&mut self) -> usize {
            if self.null_ < 0 {
                self.null_ = self.allocate_index(Entry::null()) as isize;
            }
            self.null_ as usize
        }

        pub fn insert_true(&mut self) -> usize {
            if self.true_value_ < 0 {
                self.true_value_ = self.allocate_index(Entry::true_val()) as isize;
            }
            self.true_value_ as usize
        }

        pub fn insert_false(&mut self) -> usize {
            if self.false_value_ < 0 {
                self.false_value_ = self.allocate_index(Entry::false_val()) as isize;
            }
            self.false_value_ as usize
        }

        pub fn insert_the_hole(&mut self) -> usize {
            if self.the_hole_ < 0 {
                self.the_hole_ = self.allocate_index(Entry::the_hole()) as isize;
            }
            self.the_hole_ as usize
        }

        fn allocate_index(&mut self, entry: Entry) -> usize {
            self.allocate_index_array(entry, 1)
        }

        fn allocate_index_array(&mut self, entry: Entry, count: usize) -> usize {
            for i in 0..self.idx_slice_.len() {
                if self.idx_slice_[i].available() >= count {
                    return self.idx_slice_[i].allocate(entry, count);
                }
            }
            panic!("UNREACHABLE");
        }

        fn operand_size_to_slice(&self, operand_size: OperandSize) -> &ConstantArraySlice {
            match operand_size {
                OperandSize::None => panic!("UNREACHABLE"),
                OperandSize::Byte => &self.idx_slice_[0],
                OperandSize::Short => &self.idx_slice_[1],
                OperandSize::Quad => &self.idx_slice_[2],
            }
        }

        pub fn insert_deferred(&mut self) -> usize {
            self.allocate_index(Entry::deferred())
        }

        pub fn insert_jump_table(&mut self, size: usize) -> usize {
            self.allocate_index_array(Entry::uninitialized_jump_table_smi(), size)
        }

        pub fn set_deferred_at(&mut self, index: usize, object: HandleObject) {
            let slice = self.index_to_slice(index);
            slice.at(index).set_deferred(object);
        }

        pub fn set_jump_table_smi(&mut self, index: usize, smi: Smi) {
            let slice = self.index_to_slice(index);
            // Allow others to reuse these Smis, but insert using emplace to avoid
            // overwriting existing values in the Smi map (which may have a smaller
            // operand size).
            self.smi_map_.insert(smi, index);
            slice.at(index).set_jump_table_smi(smi);
        }

        pub fn create_reserved_entry(&mut self, minimum_operand_size: OperandSize) -> OperandSize {
            for i in 0..self.idx_slice_.len() {
                if self.idx_slice_[i].available() > 0 && self.idx_slice_[i].operand_size() >= minimum_operand_size {
                    self.idx_slice_[i].reserve();
                    return self.idx_slice_[i].operand_size();
                }
            }
            panic!("UNREACHABLE");
        }

        fn allocate_reserved_entry(&mut self, value: Smi) -> usize {
            let index = self.allocate_index(Entry::smi(value));
            self.smi_map_.insert(value, index);
            index
        }

        pub fn commit_reserved_entry(&mut self, operand_size: OperandSize, value: Smi) -> usize {
            self.discard_reserved_entry(operand_size);
            let index;
            if let Some(&idx) = self.smi_map_.get(&value) {
                let slice = self.operand_size_to_slice(operand_size);
                index = idx;
                if index > slice.max_index() {
                    // The object is already in the constant array, but may have an
                    // index too big for the reserved operand_size. So, duplicate
                    // entry with the smaller operand size.
                    index = self.allocate_reserved_entry(value);
                }
                assert!(index <= slice.max_index());
            } else {
                index = self.allocate_reserved_entry(value);
            }
            index
        }

        pub fn discard_reserved_entry(&mut self, operand_size: OperandSize) {
            self.operand_size_to_slice(operand_size).unreserve();
        }

        fn insert_nan(&mut self) -> usize {
            let index = self.allocate_index(Entry::heap_number(f64::NAN));
            self.heap_number_map_.insert(f64::NAN, index);
            index
        }
    }
}