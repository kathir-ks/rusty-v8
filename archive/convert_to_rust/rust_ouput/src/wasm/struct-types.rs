// Converted from V8 C++ source files:
// Header: struct-types.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::cmp::min;
use std::fmt;
use std::ops::Range;
use std::rc::Rc;

use crate::base::macros::RoundUp;
use crate::wasm::decoder::ValueType;
use crate::wasm::decoder::ValueTypeKind;
use crate::wasm::decoder::HeapTypeRepresentation;
use crate::torque::types::ModuleTypeIndex;
use crate::compiler::turboshaft::ArrayType;
use crate::torque::types::CanonicalTypeIndex;

const kV8MaxWasmStructFields: usize = 1024;
const kMaxValueTypeSize: usize = 8;
const kTaggedSize: usize = 8;

pub struct StructTypeBase {
    field_count_: u32,
    field_offsets_: Vec<u32>,
    reps_: Vec<ValueType>, // Changed from ValueTypeBase to ValueType
    mutabilities_: Vec<bool>,
    offsets_initialized_: bool,
}

impl StructTypeBase {
    pub fn new(
        field_count: u32,
        field_offsets: Vec<u32>,
        reps: Vec<ValueType>, // Changed from ValueTypeBase to ValueType
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

    pub fn field(&self, index: u32) -> ValueType {
        assert!(index < self.field_count_);
        self.reps_[index as usize]
    }

    pub fn mutability(&self, index: u32) -> bool {
        assert!(index < self.field_count_);
        self.mutabilities_[index as usize]
    }

    // Iteration support.
    pub fn fields(&self) -> Range<usize> {
        0..(self.field_count_ as usize)
    }
    pub fn mutabilities(&self) -> Range<usize> {
        0..(self.field_count_ as usize)
    }

    // Returns the offset of this field in the runtime representation of the
    // object, from the start of the object fields (disregarding the object
    // header).
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
        RoundUp(offset as usize, min(alignment as usize, kTaggedSize)) as u32
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
        offset = RoundUp(offset as usize, kTaggedSize) as u32;
        self.field_offsets_[(self.field_count() - 1) as usize] = offset;
        self.offsets_initialized_ = true;
    }
}

// Module-relative type indices.
#[derive(Debug)]
pub struct StructType {
    base: StructTypeBase,
}

impl StructType {
    pub fn new(
        field_count: u32,
        field_offsets: Vec<u32>,
        reps: Vec<ValueType>,
        mutabilities: Vec<bool>,
    ) -> Self {
        StructType {
            base: StructTypeBase::new(field_count, field_offsets, reps, mutabilities),
        }
    }

    pub fn field_count(&self) -> u32 {
        self.base.field_count()
    }

    pub fn field(&self, index: u32) -> ValueType {
        ValueType{
          kind: self.base.field(index).kind,
          nullable: self.base.field(index).nullable,
          heap_type_representation: self.base.field(index).heap_type_representation
        }
    }

    pub fn mutability(&self, index: u32) -> bool {
        self.base.mutability(index)
    }

    pub fn fields(&self) -> Range<usize> {
        self.base.fields()
    }

    pub fn initialize_offsets(&mut self) {
        self.base.initialize_offsets();
    }

    pub fn field_offset(&self, index: u32) -> u32 {
        self.base.field_offset(index)
    }
}

impl PartialEq for StructType {
    fn eq(&self, other: &Self) -> bool {
        if self as *const _ == other as *const _ {
            return true;
        }
        if self.field_count() != other.field_count() {
            return false;
        }

        for i in 0..self.field_count() as usize {
          if self.base.reps_[i].kind != other.base.reps_[i].kind ||
             self.base.mutabilities_[i] != other.base.mutabilities_[i] {
            return false;
          }
        }
        true
    }
}

// Canonicalized type indices.
pub struct CanonicalStructType {
    field_count_: u32,
    field_offsets_: Vec<u32>,
    reps_: Vec<ValueType>, // Changed from CanonicalValueType to ValueType
    mutabilities_: Vec<bool>,
    offsets_initialized_: bool,
}

impl CanonicalStructType {
    pub fn new(
        field_count: u32,
        field_offsets: Vec<u32>,
        reps: Vec<ValueType>, // Changed from CanonicalValueType to ValueType
        mutabilities: Vec<bool>,
    ) -> Self {
        CanonicalStructType {
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

    pub fn field(&self, index: u32) -> ValueType {
        assert!(index < self.field_count_);
        self.reps_[index as usize]
    }

    pub fn mutability(&self, index: u32) -> bool {
        assert!(index < self.field_count_);
        self.mutabilities_[index as usize]
    }

    // Iteration support.
    pub fn fields(&self) -> Range<usize> {
        0..(self.field_count_ as usize)
    }
    pub fn mutabilities(&self) -> Range<usize> {
        0..(self.field_count_ as usize)
    }

    // Returns the offset of this field in the runtime representation of the
    // object, from the start of the object fields (disregarding the object
    // header).
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
        RoundUp(offset as usize, min(alignment as usize, kTaggedSize)) as u32
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
        offset = RoundUp(offset as usize, kTaggedSize) as u32;
        self.field_offsets_[(self.field_count() - 1) as usize] = offset;
        self.offsets_initialized_ = true;
    }

}

impl PartialEq for CanonicalStructType {
    fn eq(&self, other: &Self) -> bool {
        if self as *const _ == other as *const _ {
            return true;
        }
        if self.field_count() != other.field_count() {
            return false;
        }
        for i in 0..self.field_count() as usize {
          if self.reps_[i].kind != other.reps_[i].kind ||
             self.mutabilities_[i] != other.mutabilities_[i] {
            return false;
          }
        }
        true
    }
}

impl fmt::Display for StructTypeBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for i in 0..self.field_count() as usize {
            let field = self.reps_[i];
            write!(f, "{:?}, ", field.kind)?; // Assuming ValueTypeBase has a name() method
        }
        write!(f, "]")
    }
}

pub struct ArrayTypeBase {
    mutability_: bool,
}

impl ArrayTypeBase {
    pub const fn new(mutability: bool) -> Self {
        ArrayTypeBase { mutability_: mutability }
    }

    pub fn mutability(&self) -> bool {
        self.mutability_
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ArrayTypeWrapper {
  pub rep_: ValueType,
  pub mutability_: bool,
}

impl ArrayTypeWrapper {
    pub fn new(rep_: ValueType, mutability_: bool) -> Self {
      ArrayTypeWrapper{
        rep_: rep_,
        mutability_: mutability_
      }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct CanonicalArrayTypeWrapper {
  pub rep_: ValueType,
  pub mutability_: bool,
}

impl CanonicalArrayTypeWrapper {
    pub fn new(rep_: ValueType, mutability_: bool) -> Self {
      CanonicalArrayTypeWrapper{
        rep_: rep_,
        mutability_: mutability_
      }
    }
}

#[derive(Debug)]
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

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct CanonicalContType {
    index_: CanonicalTypeIndex,
}

impl CanonicalContType {
    pub const fn new(idx: CanonicalTypeIndex) -> Self {
        CanonicalContType { index_: idx }
    }

    pub fn contfun_typeindex(&self) -> CanonicalTypeIndex {
        self.index_
    }
}
