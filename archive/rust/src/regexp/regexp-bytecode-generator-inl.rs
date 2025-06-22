// Copyright 2008-2009 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod regexp_bytecode_generator {
    use crate::regexp::regexp_bytecodes::*;

    const BYTECODE_SHIFT: u32 = 8;

    #[inline]
    fn is_uint24(value: u32) -> bool {
        value < (1 << 24)
    }

    #[inline]
    fn is_int24(value: i32) -> bool {
        value >= -(1 << 23) && value < (1 << 23)
    }

    pub struct RegExpBytecodeGenerator {
        buffer_: Vec<u8>,
        pc_: usize,
    }

    impl RegExpBytecodeGenerator {
        pub fn new(initial_buffer_size: usize) -> Self {
            RegExpBytecodeGenerator {
                buffer_: Vec::with_capacity(initial_buffer_size),
                pc_: 0,
            }
        }

        pub fn current_bytecode_offset(&self) -> usize {
            self.pc_
        }

        pub fn get_buffer(&self) -> &[u8] {
            &self.buffer_
        }

        fn expand_buffer(&mut self) {
            self.buffer_.resize(self.buffer_.len() * 2, 0);
        }

        pub fn emit(&mut self, byte: u32, twenty_four_bits: u32) {
            debug_assert!(is_uint24(twenty_four_bits));
            self.emit32((twenty_four_bits << BYTECODE_SHIFT) | byte);
        }

        pub fn emit_i32(&mut self, byte: u32, twenty_four_bits: i32) {
            debug_assert!(is_int24(twenty_four_bits));
            self.emit32(((twenty_four_bits as u32) << BYTECODE_SHIFT) | byte);
        }

        pub fn emit16(&mut self, word: u32) {
            debug_assert!(self.pc_ <= self.buffer_.len());
            if self.pc_ + 1 >= self.buffer_.len() {
                self.expand_buffer();
            }

            let bytes = word.to_le_bytes();
            self.buffer_[self.pc_..self.pc_ + 2].copy_from_slice(&bytes[..2]);
            self.pc_ += 2;
        }

        pub fn emit8(&mut self, word: u32) {
            debug_assert!(self.pc_ <= self.buffer_.len());
            if self.pc_ == self.buffer_.len() {
                self.expand_buffer();
            }
            self.buffer_[self.pc_] = word as u8;
            self.pc_ += 1;
        }

        pub fn emit32(&mut self, word: u32) {
            debug_assert!(self.pc_ <= self.buffer_.len());
            if self.pc_ + 3 >= self.buffer_.len() {
                self.expand_buffer();
            }
            let bytes = word.to_le_bytes();
            self.buffer_[self.pc_..self.pc_ + 4].copy_from_slice(&bytes);
            self.pc_ += 4;
        }
    }
}

pub mod regexp_bytecodes {
    // This module would contain definitions for regexp bytecodes,
    // but since the original C++ header only included it, and
    // didn't define any content, we leave this module empty for now.
}