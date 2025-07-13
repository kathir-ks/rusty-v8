// Converted from V8 C++ source files:
// Header: bytecode-jump-table.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod utils {
    pub struct BitVector {
        bits: Vec<u64>,
        size: usize,
    }

    impl BitVector {
        pub fn new(size: usize) -> Self {
            let num_words = (size + 63) / 64;
            BitVector {
                bits: vec![0; num_words],
                size,
            }
        }

        pub fn add(&mut self, index: usize) {
            if index >= self.size {
                panic!("Index out of bounds");
            }
            let word_index = index / 64;
            let bit_index = index % 64;
            self.bits[word_index] |= 1 << bit_index;
        }

        pub fn contains(&self, index: usize) -> bool {
            if index >= self.size {
                return false;
            }
            let word_index = index / 64;
            let bit_index = index % 64;
            (self.bits[word_index] & (1 << bit_index)) != 0
        }
    }
}

pub mod zone {
    pub struct Zone {}
    impl Zone {
        pub fn new() -> Self {
            Zone {}
        }
    }
}

use crate::utils::BitVector;
use crate::zone::Zone;

pub struct ConstantArrayBuilder {}

pub struct BytecodeJumpTable {
    #[cfg(debug_assertions)]
    bound_: BitVector,
    constant_pool_index_: usize,
    switch_bytecode_offset_: usize,
    size_: i32,
    case_value_base_: i32,
}

impl BytecodeJumpTable {
    pub fn new(
        constant_pool_index: usize,
        size: i32,
        case_value_base: i32,
        zone: &Zone,
    ) -> Self {
        BytecodeJumpTable {
            #[cfg(debug_assertions)]
            bound_: BitVector::new(size as usize),
            constant_pool_index_: constant_pool_index,
            switch_bytecode_offset_: Self::kInvalidOffset,
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
        assert!(case_value >= self.case_value_base_);
        assert!(case_value < self.case_value_base_ + self.size());
        self.bound_.contains((case_value - self.case_value_base_) as usize)
    }

    pub fn constant_pool_entry_for(&self, case_value: i32) -> usize {
        assert!(case_value >= self.case_value_base_);
        (self.constant_pool_index_ as i32 + case_value - self.case_value_base_) as usize
    }

    const kInvalidIndex: usize = usize::MAX;
    const kInvalidOffset: usize = usize::MAX;

    fn mark_bound(&mut self, case_value: i32) {
        #[cfg(debug_assertions)]
        {
            assert!(case_value >= self.case_value_base_);
            assert!(case_value < self.case_value_base_ + self.size());
            self.bound_.add((case_value - self.case_value_base_) as usize);
        }
    }

    fn set_switch_bytecode_offset(&mut self, offset: usize) {
        assert_eq!(self.switch_bytecode_offset_, Self::kInvalidOffset);
        self.switch_bytecode_offset_ = offset;
    }
}
