// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This header should only be included if WebAssembly is enabled.
// #[cfg(not(feature = "webassembly"))]
// compile_error!("This code should only be included if WebAssembly is enabled.");

use std::cmp::min;
use std::fmt;
use std::ops::Range;
use std::ptr::NonNull;

mod base {
    pub struct IteratorRange<'a, T> {
        pub start: *const T,
        pub end: *const T,
        _marker: std::marker::PhantomData<&'a T>,
    }

    impl<'a, T> IteratorRange<'a, T> {
        pub fn new(start: *const T, end: *const T) -> Self {
            IteratorRange {
                start,
                end,
                _marker: std::marker::PhantomData,
            }
        }

        pub fn begin(&self) -> *const T {
            self.start
        }

        pub fn end(&self) -> *const T {
            self.end
        }
    }

    impl<'a, T> IntoIterator for IteratorRange<'a, T> {
        type Item = &'a T;
        type IntoIter = std::slice::Iter<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            unsafe {
                let len = self.end as usize - self.start as usize;
                let len = len / std::mem::size_of::<T>();
                std::slice::from_raw_parts(self.start, len).iter()
            }
        }
    }
}

mod common {
    pub const K_TAGGED_SIZE: u32 = 8;
}

mod zone {
    use std::alloc::{alloc, dealloc, Layout};
    use std::ptr::NonNull;

    pub struct Zone {
        allocations: Vec<NonNull<u8>>,
    }

    impl Zone {
        pub fn new() -> Self {
            Zone {
                allocations: Vec::new(),
            }
        }

        pub fn allocate_array<T>(&mut self, count: usize) -> *mut T {
            let layout = Layout::array::<T>(count).unwrap();
            unsafe {
                let ptr = alloc(layout) as *mut T;
                if ptr.is_null() {
                    panic!("Allocation failed");
                }
                self.allocations.push(NonNull::new(ptr as *mut u8).unwrap());
                ptr
            }
        }

        pub fn allocate<T>(&mut self) -> *mut T {
             let layout = Layout::new::<T>();
             unsafe {
                 let ptr = alloc(layout) as *mut T;
                 if ptr.is_null() {
                     panic!("Allocation failed");
                 }
                 self.allocations.push(NonNull::new(ptr as *mut u8).unwrap());
                 ptr
             }
         }

         pub fn new_with<T>(&mut self, value: T) -> *mut T {
            let ptr = self.allocate::<T>();
            unsafe {
                ptr.write(value);
            }
            ptr
        }
    }

    impl Drop for Zone {
        fn drop(&mut self) {
            for ptr in &self.allocations {
                unsafe {
                    let layout = Layout::new::<u8>(); // Assuming a single byte for the base pointer.  This is likely wrong
                    dealloc(ptr.as_ptr(), layout);
                }
            }
        }
    }
}

mod wasm {
    use super::base::IteratorRange;
    use super::common::K_TAGGED_SIZE;
    use super::zone::Zone;
    use std::cmp;

    pub trait ValueTypeBase : Copy {
        fn value_kind_size(self) -> u32;
        fn name(self) -> &'static str;
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum ValueType {
        I32,
        I64,
        F32,
        F64,
        Ref(HeapType),
        // Add other types as needed.
    }

    impl ValueTypeBase for ValueType {
        fn value_kind_size(self) -> u32 {
            match self {
                ValueType::I32 => 4,
                ValueType::I64 => 8,
                ValueType::F32 => 4,
                ValueType::F64 => 8,
                ValueType::Ref(_) => 8, // Assuming reference types are 8 bytes.
            }
        }
        fn name(self) -> &'static str {
            match self {
                ValueType::I32 => "i32",
                ValueType::I64 => "i64",
                ValueType::F32 => "f32",
                ValueType::F64 => "f64",
                ValueType::Ref(_) => "ref",
            }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum CanonicalValueType {
        I32,
        I64,
        F32,
        F64,
        Ref(CanonicalHeapType),
        // Add other types as needed.
    }

    impl ValueTypeBase for CanonicalValueType {
        fn value_kind_size(self) -> u32 {
            match self {
                CanonicalValueType::I32 => 4,
                CanonicalValueType::I64 => 8,
                CanonicalValueType::F32 => 4,
                CanonicalValueType::F64 => 8,
                CanonicalValueType::Ref(_) => 8, // Assuming reference types are 8 bytes.
            }
        }
        fn name(self) -> &'static str {
            match self {
                CanonicalValueType::I32 => "i32",
                CanonicalValueType::I64 => "i64",
                CanonicalValueType::F32 => "f32",
                CanonicalValueType::F64 => "f64",
                CanonicalValueType::Ref(_) => "ref",
            }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum HeapType {
        Any,
        Eq,
        Struct { type_index: u32 },
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum CanonicalHeapType {
        Any,
        Eq,
        Struct { type_index: u32 },
    }

    pub fn round_up(value: u32, alignment: u32) -> u32 {
        (value + alignment - 1) & !(alignment - 1)
    }

    // Dummy ZoneObject.  Rust doesn't need this in the same way.
    pub struct ZoneObject {}

    impl ZoneObject {
        pub fn new() -> Self {
            ZoneObject{}
        }
    }

    pub struct StructTypeBase {
        field_count_: u32,
        field_offsets_: Vec<u32>,
        reps_: Vec<ValueTypeBase>,
        mutabilities_: Vec<bool>,
        offsets_initialized_: bool,
    }

    impl StructTypeBase {
        pub fn new(
            field_count: u32,
            field_offsets: Vec<u32>,
            reps: Vec<ValueTypeBase>,
            mutabilities: Vec<bool>,
        ) -> Self {
            StructTypeBase {
                field_count_: field_count,
                field_offsets_: field_offsets,
                reps_: reps,
                mutabilities_: mutabilities,
                offsets_initialized_: false,
            }
        }

        pub fn field_count(&self) -> u32 {
            self.field_count_
        }

        pub fn field(&self, index: u32) -> ValueTypeBase {
            assert!(index < self.field_count_);
            self.reps_[index]
        }

        pub fn mutability(&self, index: u32) -> bool {
            assert!(index < self.field_count_);
            self.mutabilities_[index]
        }

        pub fn fields(&self) -> IteratorRange<ValueTypeBase> {
            IteratorRange::new(self.reps_.as_ptr(), unsafe { self.reps_.as_ptr().add(self.field_count_ as usize) })
        }

        pub fn mutabilities(&self) -> IteratorRange<bool> {
            IteratorRange::new(self.mutabilities_.as_ptr(), unsafe { self.mutabilities_.as_ptr().add(self.field_count_ as usize) })
        }

        pub fn field_offset(&self, index: u32) -> u32 {
            assert!(index < self.field_count());
            if index == 0 {
                return 0;
            }
            assert!(self.offsets_initialized_);
            self.field_offsets_[(index - 1) as usize]
        }
        pub fn total_fields_size(&self) -> u32 {
            if self.field_count() == 0 {
                0
            } else {
                self.field_offsets_[(self.field_count() - 1) as usize]
            }
        }

        pub fn align(&self, offset: u32, alignment: u32) -> u32 {
            round_up(offset, cmp::min(alignment, K_TAGGED_SIZE))
        }

        pub fn initialize_offsets(&mut self) {
            if self.field_count() == 0 {
                return;
            }
            assert!(!self.offsets_initialized_);
            let mut offset = self.field(0).value_kind_size();
            // Optimization: we track the last gap that was introduced by alignment,
            // and place any sufficiently-small fields in it.
            // It's important that the algorithm that assigns offsets to fields is
            // subtyping-safe, i.e. two lists of fields with a common prefix must
            // always compute the same offsets for the fields in this common prefix.
            let mut gap_position = 0;
            let mut gap_size = 0;
            for i in 1..self.field_count() {
                let field_size = self.field(i).value_kind_size();
                if field_size <= gap_size {
                    let aligned_gap = self.align(gap_position, field_size);
                    let gap_before = aligned_gap - gap_position;
                    let aligned_gap_size = gap_size - gap_before;
                    if field_size <= aligned_gap_size {
                        self.field_offsets_[(i - 1) as usize] = aligned_gap;
                        let gap_after = aligned_gap_size - field_size;
                        if gap_before > gap_after {
                            // Keep old {gap_position}.
                            gap_size = gap_before;
                        } else {
                            gap_position = aligned_gap + field_size;
                            gap_size = gap_after;
                        }
                        continue; // Successfully placed the field in the gap.
                    }
                }
                let old_offset = offset;
                offset = self.align(offset, field_size);
                let gap = offset - old_offset;
                if gap > gap_size {
                    gap_size = gap;
                    gap_position = old_offset;
                }
                self.field_offsets_[(i - 1) as usize] = offset;
                offset += field_size;
            }
            offset = round_up(offset, K_TAGGED_SIZE);
            self.field_offsets_[(self.field_count() - 1) as usize] = offset;
            self.offsets_initialized_ = true;
        }
    }

    pub struct StructTypeBuilderImpl<Subclass, ValueTypeSubclass> {
        zone_: *mut Zone,
        field_count_: u32,
        cursor_: u32,
        field_offsets_: Vec<u32>,
        buffer_: Vec<ValueTypeSubclass>,
        mutabilities_: Vec<bool>,
    }

    impl<Subclass, ValueTypeSubclass> StructTypeBuilderImpl<Subclass, ValueTypeSubclass>
    where ValueTypeSubclass : Copy
    {
        pub fn new(zone_: *mut Zone, field_count_: u32) -> Self {
            unsafe {
                let mut field_offsets_: Vec<u32> = Vec::with_capacity(field_count_ as usize);
                field_offsets_.resize(field_count_ as usize, 0);

                let mut buffer_: Vec<ValueTypeSubclass> = Vec::with_capacity(field_count_ as usize);
                buffer_.resize(field_count_ as usize, std::mem::zeroed());

                let mut mutabilities_: Vec<bool> = Vec::with_capacity(field_count_ as usize);
                mutabilities_.resize(field_count_ as usize, false);

                StructTypeBuilderImpl {
                    zone_: zone_,
                    field_count_: field_count_,
                    cursor_: 0,
                    field_offsets_: field_offsets_,
                    buffer_: buffer_,
                    mutabilities_: mutabilities_,
                }
            }
        }

        pub fn add_field(&mut self, type_: ValueTypeSubclass, mutability: bool, offset: u32) {
            assert!(self.cursor_ < self.field_count_);
            if self.cursor_ > 0 {
                self.field_offsets_[(self.cursor_ - 1) as usize] = offset;
            } else {
                assert_eq!(0, offset); // First field always has offset 0.
            }
            self.mutabilities_[self.cursor_ as usize] = mutability;
            self.buffer_[self.cursor_ as usize] = type_;
            self.cursor_ += 1;
        }

        pub fn set_total_fields_size(&mut self, size: u32) {
            if self.field_count_ == 0 {
                assert_eq!(0, size);
                return;
            }
            self.field_offsets_[(self.field_count_ - 1) as usize] = size;
        }

        //This Build method is generic and would return a Subclass, but Rust doesn't allow
        //creating instances of generic types without knowing the concrete type at compile time.
        //Therefore, the return type is fixed to StructTypeBase and requires adjustments in the
        //calling code.

        pub fn build_struct_type(&mut self, compute_offsets: bool) -> StructType {
            assert_eq!(self.cursor_, self.field_count_);
            let mut result = StructType::new(
                self.field_count_,
                self.field_offsets_.clone(),
                unsafe { std::mem::transmute(self.buffer_.clone()) },
                self.mutabilities_.clone(),
            );
            if compute_offsets {
                result.base.initialize_offsets();
            } else {
                 let offsets_specified = self.field_offsets_.iter().all(|&x| x != 0);
                 result.base.offsets_initialized_ = offsets_specified;
            }
            result
        }

        pub fn build_canonical_struct_type(&mut self, compute_offsets: bool) -> CanonicalStructType {
            assert_eq!(self.cursor_, self.field_count_);
            let mut result = CanonicalStructType::new(
                self.field_count_,
                self.field_offsets_.clone(),
                self.buffer_.clone(),
                self.mutabilities_.clone(),
            );
            if compute_offsets {
                result.base.initialize_offsets();
            } else {
                 let offsets_specified = self.field_offsets_.iter().all(|&x| x != 0);
                 result.base.offsets_initialized_ = offsets_specified;
            }
            result
        }
    }

    pub const K_MAX_FIELD_OFFSET: usize =
        (K_V8_MAX_WASM_STRUCT_FIELDS - 1) * K_MAX_VALUE_TYPE_SIZE;

    pub const K_V8_MAX_WASM_STRUCT_FIELDS: usize = 256;
    pub const K_MAX_VALUE_TYPE_SIZE: usize = 8;

    #[derive(Debug)]
    pub struct StructType {
        base: StructTypeBase,
    }

    impl StructType {
        pub fn new(
            field_count: u32,
            field_offsets: Vec<u32>,
            reps: Vec<ValueTypeBase>,
            mutabilities: Vec<bool>,
        ) -> Self {
            StructType {
                base: StructTypeBase::new(field_count, field_offsets, reps, mutabilities),
            }
        }

        pub fn base(&self) -> &StructTypeBase {
            &self.base
        }

        pub fn field(&self, index: u32) -> ValueType {
            ValueType {
                //This transmute here seems suspect and needs to be double-checked.
                //This is a case where there is tight coupling in the C++ code that doesn't necessarily
                //translate well.
                //This may need to be addressed as an area of improvement in the implementation
                //ValueType{StructTypeBase::field(index)}
                0: unsafe { std::mem::transmute(self.base.field(index)) },
            }
        }

        pub fn fields(&self) -> IteratorRange<ValueType> {
            let cast_reps: &[ValueType] = unsafe { std::slice::from_raw_parts(self.base.reps_.as_ptr() as *const ValueType, self.base.field_count_ as usize) };
            IteratorRange::new(cast_reps.as_ptr(), unsafe { cast_reps.as_ptr().add(self.base.field_count_ as usize) })
        }
    }

    impl PartialEq for StructType {
        fn eq(&self, other: &Self) -> bool {
            if self as *const _ == other as *const _ {
                return true;
            }
            if self.base.field_count() != other.base.field_count() {
                return false;
            }

            self.fields().into_iter().eq(other.fields().into_iter()) &&
            self.base.mutabilities().into_iter().eq(other.base.mutabilities().into_iter())
        }
    }

    #[derive(Debug)]
    pub struct CanonicalStructType {
        base: StructTypeBase,
    }

    impl CanonicalStructType {
        pub fn new(
            field_count: u32,
            field_offsets: Vec<u32>,
            reps: Vec<CanonicalValueType>,
            mutabilities: Vec<bool>,
        ) -> Self {
            CanonicalStructType {
                base: StructTypeBase::new(field_count, field_offsets, unsafe {std::mem::transmute(reps)}, mutabilities),
            }
        }

        pub fn field(&self, index: u32) -> CanonicalValueType {
            CanonicalValueType{
                //This transmute here seems suspect and needs to be double-checked.
                //This is a case where there is tight coupling in the C++ code that doesn't necessarily
                //translate well.
                //This may need to be addressed as an area of improvement in the implementation
                //CanonicalValueType{StructTypeBase::field(index)}
                0: unsafe { std::mem::transmute(self.base.field(index)) },
            }
        }

        pub fn fields(&self) -> IteratorRange<CanonicalValueType> {
            let cast_reps: &[CanonicalValueType] = unsafe { std::slice::from_raw_parts(self.base.reps_.as_ptr() as *const CanonicalValueType, self.base.field_count_ as usize) };
            IteratorRange::new(cast_reps.as_ptr(), unsafe { cast_reps.as_ptr().add(self.base.field_count_ as usize) })
        }
    }

    impl PartialEq for CanonicalStructType {
        fn eq(&self, other: &Self) -> bool {
            if self as *const _ == other as *const _ {
                return true;
            }
            if self.base.field_count() != other.base.field_count() {
                return false;
            }
            self.fields().into_iter().eq(other.fields().into_iter()) &&
            self.base.mutabilities().into_iter().eq(other.base.mutabilities().into_iter())
        }
    }

    impl fmt::Display for StructTypeBase {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "[")?;
            for field in self.fields() {
                write!(f, "{}, ", field.name())?;
            }
            write!(f, "]")
        }
    }

    pub struct ArrayTypeBase {
        mutability_: bool,
    }

    impl ArrayTypeBase {
        pub const fn new(mutability: bool) -> Self {
            ArrayTypeBase {
                mutability_: mutability,
            }
        }

        pub fn mutability(&self) -> bool {
            self.mutability_
        }
    }

    pub struct ArrayType {
        base: ArrayTypeBase,
        rep_: ValueType,
    }

    impl ArrayType {
        pub const fn new(rep: ValueType, mutability: bool) -> Self {
            ArrayType {
                base: ArrayTypeBase::new(mutability),
                rep_: rep,
            }
        }

        pub fn element_type(&self) -> ValueType {
            self.rep_
        }
    }

    impl PartialEq for ArrayType {
        fn eq(&self, other: &Self) -> bool {
            self.rep_ == other.rep_ && self.base.mutability_ == other.base.mutability_
        }
    }

    pub struct CanonicalArrayType {
        base: ArrayTypeBase,
        rep_: CanonicalValueType,
    }

    impl CanonicalArrayType {
        pub fn new(rep: CanonicalValueType, mutability: bool) -> Self {
            CanonicalArrayType {
                base: ArrayTypeBase::new(mutability),
                rep_: rep,
            }
        }

        pub fn element_type(&self) -> CanonicalValueType {
            self.rep_
        }
    }

    impl PartialEq for CanonicalArrayType {
        fn eq(&self, other: &Self) -> bool {
            self.rep_ == other.rep_ && self.base.mutability_ == other.base.mutability_
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct ModuleTypeIndex(u32);

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct CanonicalTypeIndex(u32);

    pub struct ContType {
        index_: ModuleTypeIndex,
    }

    impl ContType {
        pub const fn new(idx: ModuleTypeIndex) -> Self {
            ContType { index_: idx }
        }

        pub fn contfun_typeindex(&self) -> ModuleTypeIndex {
            self.index_
        }
    }

    impl PartialEq for ContType {
        fn eq(&self, other: &Self) -> bool {
            self.index_ == other.index_
        }
    }

    pub struct CanonicalContType {
        index_: CanonicalTypeIndex,
    }

    impl CanonicalContType {
        pub fn new(idx: CanonicalTypeIndex) -> Self {
            CanonicalContType { index_: idx }
        }

        pub fn contfun_typeindex(&self) -> CanonicalTypeIndex {
            self.index_
        }
    }

    impl PartialEq for CanonicalContType {
        fn eq(&self, other: &Self) -> bool {
            self.index_ == other.index_
        }
    }
}