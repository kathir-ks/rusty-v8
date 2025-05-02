// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/parsing/literal-buffer.h (Rust module definition)
pub mod literal_buffer {
    use std::cmp;
    use std::mem;
    use std::ptr;
    use std::slice;
    use v8::base;
    use v8::execution::{Isolate, LocalIsolate};
    use v8::heap::Factory;
    use v8::strings::String;
    use v8::unibrow;
    use v8::utils;

    const K_INITIAL_CAPACITY: usize = 16;
    const K_GROWTH_FACTOR: usize = 2;
    const K_MAX_GROWTH: usize = 4096;

    /// A buffer for building string literals.
    pub struct LiteralBuffer {
        backing_store: Vec<u8>,
        position_: usize,
        is_one_byte_: bool,
    }

    impl LiteralBuffer {
        /// Creates a new empty LiteralBuffer.
        pub fn new() -> Self {
            LiteralBuffer {
                backing_store: Vec::new(),
                position_: 0,
                is_one_byte_: true,
            }
        }

        /// Creates a new empty LiteralBuffer with initial capacity.
        pub fn with_capacity(capacity: usize) -> Self {
            LiteralBuffer {
                backing_store: Vec::with_capacity(capacity),
                position_: 0,
                is_one_byte_: true,
            }
        }

        /// Returns true if the buffer only contains one-byte characters.
        pub fn is_one_byte(&self) -> bool {
            self.is_one_byte_
        }

        /// Returns the one-byte literal as a slice. Panics if not one-byte.
        pub fn one_byte_literal(&self) -> &[u8] {
            assert!(self.is_one_byte_);
            &self.backing_store[..self.position_]
        }

        /// Returns the two-byte literal as a slice. Panics if one-byte.
        pub fn two_byte_literal(&self) -> &[u16] {
            assert!(!self.is_one_byte_);
            unsafe {
                slice::from_raw_parts(self.backing_store.as_ptr() as *const u16, self.position_ / 2)
            }
        }

        /// Returns the current length of the literal in bytes.
        pub fn length(&self) -> usize {
            self.position_
        }

        /// Adds a one-byte character to the buffer.
        pub fn add_char(&mut self, c: u8) {
            if !self.is_one_byte_ {
                panic!("Cannot add one-byte character to two-byte buffer");
            }
            if self.position_ >= self.backing_store.len() {
                self.expand_buffer();
            }
            self.backing_store.push(c);
            self.position_ += 1;
        }

        /// Clears the buffer.
        pub fn clear(&mut self) {
            self.backing_store.clear();
            self.position_ = 0;
            self.is_one_byte_ = true;
        }
    }

    impl LiteralBuffer {
        pub fn internalize<T>(&self, isolate: &T) -> Result<String, String>
        where
            T: InternalizeTrait,
        {
            isolate.internalize(self)
        }

        fn new_capacity(min_capacity: usize) -> usize {
            if min_capacity < (K_MAX_GROWTH / (K_GROWTH_FACTOR - 1)) {
                min_capacity * K_GROWTH_FACTOR
            } else {
                min_capacity + K_MAX_GROWTH
            }
        }

        fn expand_buffer(&mut self) {
            let min_capacity = cmp::max(K_INITIAL_CAPACITY, self.backing_store.len());
            let new_capacity = Self::new_capacity(min_capacity);
            let mut new_store = Vec::with_capacity(new_capacity);
            if self.position_ > 0 {
                unsafe {
                    let src = self.backing_store.as_ptr();
                    let dst = new_store.as_mut_ptr();
                    ptr::copy_nonoverlapping(src, dst, self.position_);
                }
                new_store.set_len(self.position_);
            }
            self.backing_store = new_store;
        }

        fn convert_to_two_byte(&mut self) {
            assert!(self.is_one_byte_);
            let new_content_size = self.position_ * base::K_UC16_SIZE;
            let mut new_store: Vec<u8>;

            if new_content_size >= self.backing_store.capacity() {
                new_store = Vec::with_capacity(Self::new_capacity(new_content_size));
            } else {
                new_store = Vec::with_capacity(self.backing_store.capacity());
                unsafe {
                    new_store.set_len(self.backing_store.len());
                    ptr::copy_nonoverlapping(self.backing_store.as_ptr(), new_store.as_mut_ptr(), self.backing_store.len());
                }
            }
            
            unsafe {
                let src = self.backing_store.as_ptr();
                let dst = new_store.as_mut_ptr() as *mut u16;
                for i in 0..self.position_ {
                    *dst.add(i) = *src.add(i) as u16;
                }
            }

            self.backing_store = new_store;
            self.position_ = new_content_size;
            self.is_one_byte_ = false;
        }

        pub fn add_two_byte_char(&mut self, code_unit: base::Uc32) {
            assert!(!self.is_one_byte_);
            if self.position_ >= self.backing_store.len() {
                self.expand_buffer();
            }
            if code_unit <= unibrow::utf16::K_MAX_NON_SURROGATE_CHAR_CODE as u32 {
                unsafe {
                    let ptr = self.backing_store.as_mut_ptr().add(self.position_) as *mut u16;
                    *ptr = code_unit as u16;
                }
                self.position_ += base::K_UC16_SIZE;
            } else {
                unsafe {
                    let ptr = self.backing_store.as_mut_ptr().add(self.position_) as *mut u16;
                    *ptr = unibrow::utf16::lead_surrogate(code_unit);
                }
                self.position_ += base::K_UC16_SIZE;
                if self.position_ >= self.backing_store.len() {
                    self.expand_buffer();
                }
                unsafe {
                    let ptr = self.backing_store.as_mut_ptr().add(self.position_) as *mut u16;
                    *ptr = unibrow::utf16::trail_surrogate(code_unit);
                }
                self.position_ += base::K_UC16_SIZE;
            }
        }

        pub fn add_code_unit(&mut self, code_unit: base::Uc32) {
            if self.is_one_byte_ {
                if code_unit > 0xFF {
                    self.convert_to_two_byte();
                } else {
                    if self.position_ >= self.backing_store.len() {
                        self.expand_buffer();
                    }
                    self.backing_store.push(code_unit as u8);
                    self.position_ += 1;
                    return;
                }
            }
            self.add_two_byte_char(code_unit);
        }

    }

    pub trait InternalizeTrait {
        fn internalize(&self, literal_buffer: &LiteralBuffer) -> Result<String, String>;
    }

    impl InternalizeTrait for Isolate {
        fn internalize(&self, literal_buffer: &LiteralBuffer) -> Result<String, String> {
            if literal_buffer.is_one_byte() {
                self.factory().internalize_string(literal_buffer.one_byte_literal())
            } else {
                self.factory().internalize_string_two_byte(literal_buffer.two_byte_literal())
            }
        }
    }

    impl InternalizeTrait for LocalIsolate {
        fn internalize(&self, literal_buffer: &LiteralBuffer) -> Result<String, String> {
            if literal_buffer.is_one_byte() {
                self.factory().internalize_string(literal_buffer.one_byte_literal())
            } else {
                self.factory().internalize_string_two_byte(literal_buffer.two_byte_literal())
            }
        }
    }
}

pub mod v8 {
    pub mod base {
        pub type Uc32 = u32;
        pub const K_UC16_SIZE: usize = 2;
    }

    pub mod execution {
        use super::heap::Factory;
        use super::strings::String;
        pub trait IsolateInterface {
            fn factory(&self) -> &Factory;
        }
        pub struct Isolate {}
        impl Isolate {
            pub fn factory(&self) -> &Factory {
                todo!()
            }
        }
        pub struct LocalIsolate {}

        impl LocalIsolate {
             pub fn factory(&self) -> &Factory {
                todo!()
            }
        }
    }

    pub mod heap {
        use super::strings::String;
        pub struct Factory {}
        impl Factory {
            pub fn internalize_string(&self, literal: &[u8]) -> Result<String, String> {
                //  Factory::InternalizeString()
                todo!()
            }

            pub fn internalize_string_two_byte(&self, literal: &[u16]) -> Result<String, String> {
                // Factory::InternalizeString()
                todo!()
            }
        }
    }

    pub mod strings {
        #[derive(Debug)]
        pub struct String {}
    }

    pub mod unibrow {
        pub mod utf16 {
            pub const K_MAX_NON_SURROGATE_CHAR_CODE: i32 = 0xD7FF;

            pub fn lead_surrogate(code_unit: super::super::base::Uc32) -> u16 {
                (((code_unit - 0x10000) >> 10) + 0xD800) as u16
            }

            pub fn trail_surrogate(code_unit: super::super::base::Uc32) -> u16 {
                ((code_unit - 0x10000) & 0x3FF + 0xDC00) as u16
            }
        }
    }
    pub mod utils{}
}