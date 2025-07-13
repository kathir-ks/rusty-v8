// Converted from V8 C++ source files:
// Header: bytecode-array-random-iterator.h
// Implementation: bytecode-array-random-iterator.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/interpreter/bytecode-array-random-iterator.h
use std::rc::Rc;

use crate::deoptimizer::Deoptimizer::BytecodeArrayIterator;

pub struct BytecodeArray {
    length: usize,
}

impl BytecodeArray {
    pub fn length(&self) -> usize {
        self.length
    }
}

pub struct BytecodeArrayRandomIterator {
    bytecode_array: Rc<BytecodeArray>,
    offsets: Vec<i32>,
    current_index: i32,
    current_offset: i32,
}

impl BytecodeArrayRandomIterator {
    pub fn new(bytecode_array: Rc<BytecodeArray>, zone: &Zone) -> BytecodeArrayRandomIterator {
        let mut iterator = BytecodeArrayRandomIterator {
            bytecode_array: bytecode_array.clone(),
            offsets: Vec::with_capacity(bytecode_array.length() / 2),
            current_index: 0,
            current_offset: 0,
        };
        iterator.initialize();
        iterator
    }

    fn initialize(&mut self) {
        let mut offset = 0;
        let bytecode_array = self.bytecode_array.clone();
        while offset < bytecode_array.length() {
            self.offsets.push(offset as i32);
            offset += 1; // Assuming each bytecode is 1 byte for simplicity
        }
        self.go_to_start();
    }

    pub fn current_index(&self) -> i32 {
        self.current_index
    }

    pub fn size(&self) -> usize {
        self.offsets.len()
    }

    pub fn go_to_index(&mut self, index: i32) {
        self.current_index = index;
        self.update_offset_from_index();
    }

    pub fn go_to_start(&mut self) {
        self.go_to_index(0);
    }

    pub fn go_to_end(&mut self) {
        self.go_to_index((self.size() - 1) as i32);
    }

    pub fn is_valid(&self) -> bool {
        self.current_index >= 0 && (self.current_index as usize) < self.offsets.len()
    }

    fn update_offset_from_index(&mut self) {
        if self.is_valid() {
            self.current_offset = self.offsets[self.current_index as usize];
        }
    }
    pub fn current_offset(&self) -> i32 {
        self.current_offset
    }
}

impl BytecodeArrayRandomIterator {
    pub fn increment(&mut self) -> &mut Self {
        self.current_index += 1;
        self.update_offset_from_index();
        self
    }

    pub fn decrement(&mut self) -> &mut Self {
        self.current_index -= 1;
        self.update_offset_from_index();
        self
    }

    pub fn add_assign(&mut self, offset: i32) -> &mut Self {
        self.current_index += offset;
        self.update_offset_from_index();
        self
    }

    pub fn sub_assign(&mut self, offset: i32) -> &mut Self {
        self.current_index -= offset;
        self.update_offset_from_index();
        self
    }
}

// Dummy Zone struct for compilation
pub struct Zone {}

impl Zone {
    pub fn new() -> Zone {
        Zone {}
    }
}
