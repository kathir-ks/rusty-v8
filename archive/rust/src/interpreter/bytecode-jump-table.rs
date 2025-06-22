// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/interpreter/bytecode-jump-table.rs

use std::cell::RefCell;
use std::rc::Rc;

// Mock Zone and ZoneObject
pub struct Zone {}

impl Zone {
    pub fn new() -> Self {
        Zone {}
    }
}

pub trait ZoneObject {}

// Mock BitVector (replace with a suitable Rust bit vector implementation if needed)
#[cfg(debug_assertions)]
pub struct BitVector {
    bits: Vec<bool>,
}

#[cfg(debug_assertions)]
impl BitVector {
    pub fn new(size: usize) -> Self {
        BitVector { bits: vec![false; size] }
    }

    pub fn add(&mut self, index: usize) {
        self.bits[index] = true;
    }

    pub fn contains(&self, index: usize) -> bool {
        self.bits[index]
    }
}

/// A jump table for a set of targets in a bytecode array. When an entry in the
/// table is bound, it represents a known position in the bytecode array. If no
/// entries match, the switch falls through.
pub struct BytecodeJumpTable {
    #[cfg(debug_assertions)]
    bound_: BitVector,
    constant_pool_index_: usize,
    switch_bytecode_offset_: usize,
    size_: i32,
    case_value_base_: i32,
}

impl ZoneObject for BytecodeJumpTable {}

impl BytecodeJumpTable {
    const K_INVALID_INDEX: usize = usize::MAX;
    const K_INVALID_OFFSET: usize = usize::MAX;

    /// Constructs a new BytecodeJumpTable starting at |constant_pool_index|, with
    /// the given |size|, where the case values of the table start at
    /// |case_value_base|.
    pub fn new(constant_pool_index: usize, size: i32, case_value_base: i32, _zone: &Zone) -> Self {
        BytecodeJumpTable {
            #[cfg(debug_assertions)]
            bound_: BitVector::new(size as usize),
            constant_pool_index_: constant_pool_index,
            switch_bytecode_offset_: Self::K_INVALID_OFFSET,
            size_: size,
            case_value_base_: case_value_base,
        }
    }

    pub fn constant_pool_index(&self) -> usize {
        self.constant_pool_index_
    }

    pub fn switch_bytecode_offset(&self) -> usize {
        self.switch_bytecode_offset_
    }

    pub fn case_value_base(&self) -> i32 {
        self.case_value_base_
    }

    pub fn size(&self) -> i32 {
        self.size_
    }

    #[cfg(debug_assertions)]
    pub fn is_bound(&self, case_value: i32) -> bool {
        debug_assert!(case_value >= self.case_value_base_);
        debug_assert!(case_value < self.case_value_base_ + self.size());
        self.bound_.contains((case_value - self.case_value_base_) as usize)
    }

    pub fn constant_pool_entry_for(&self, case_value: i32) -> usize {
        debug_assert!(case_value >= self.case_value_base_);
        self.constant_pool_index_ + (case_value - self.case_value_base_) as usize
    }

    fn mark_bound(&mut self, case_value: i32) {
        #[cfg(debug_assertions)]
        {
            debug_assert!(case_value >= self.case_value_base_);
            debug_assert!(case_value < self.case_value_base_ + self.size());
            self.bound_.add((case_value - self.case_value_base_) as usize);
        }
    }

    fn set_switch_bytecode_offset(&mut self, offset: usize) {
        debug_assert_eq!(self.switch_bytecode_offset_, Self::K_INVALID_OFFSET);
        self.switch_bytecode_offset_ = offset;
    }
}

// This struct requires `BytecodeArrayWriter` for full conversion.
// struct BytecodeArrayWriter {
// }