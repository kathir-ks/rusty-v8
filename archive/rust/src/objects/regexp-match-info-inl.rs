// src/objects/regexp_match_info.rs

// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//use crate::objects::fixed_array::FixedArray; // Assuming FixedArray is defined here
use crate::objects::object::Object;
use crate::objects::string::String;
use crate::objects::smi::Smi;
use std::sync::atomic::{AtomicI32, Ordering};

/// Represents the write barrier mode, mirroring the C++ enum.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WriteBarrierMode {
    //DEFAULT, // Removed: Since WriteBarrierMode is likely used in a generational GC, it's an implementation detail.
    INCREMENTAL, // Added based on common use cases
    SIMPLIFIED, // Added based on common use cases
}

/// Rust representation of v8::internal::RegExpMatchInfo
#[derive(Debug)]
pub struct RegExpMatchInfo {
    number_of_capture_registers: AtomicI32,
    last_subject: std::sync::Mutex<Option<String>>, // Use Option to represent nullable String
    last_input: std::sync::Mutex<Option<Object>>,   // Use Option to represent nullable Object
    // captures: FixedArray, // Assuming FixedArray holds the captures; needs conversion if FixedArray isn't directly translatable.
    captures: Vec<i32> // placeholder for captures
}

impl RegExpMatchInfo {
    /// Creates a new `RegExpMatchInfo`.
    pub fn new(capacity: usize) -> Self {
        RegExpMatchInfo {
            number_of_capture_registers: AtomicI32::new(0),
            last_subject: std::sync::Mutex::new(None),
            last_input: std::sync::Mutex::new(None),
            captures: vec![0; capacity]
        }
    }

    /// Gets the number of capture registers.
    pub fn number_of_capture_registers(&self) -> i32 {
        self.number_of_capture_registers.load(Ordering::Relaxed)
    }

    /// Sets the number of capture registers.
    pub fn set_number_of_capture_registers(&self, value: i32) {
        self.number_of_capture_registers.store(value, Ordering::Relaxed);
    }

    /// Gets the last subject.
    pub fn last_subject(&self) -> Option<String> {
        self.last_subject.lock().unwrap().clone()
    }

    /// Sets the last subject.
    pub fn set_last_subject(&self, value: String, _mode: WriteBarrierMode) {
        *self.last_subject.lock().unwrap() = Some(value);
    }

    /// Gets the last input.
    pub fn last_input(&self) -> Option<Object> {
        self.last_input.lock().unwrap().clone()
    }

    /// Sets the last input.
    pub fn set_last_input(&self, value: Object, _mode: WriteBarrierMode) {
        *self.last_input.lock().unwrap() = Some(value);
    }

    /// Gets a capture at the given index.
    pub fn capture(&self, index: usize) -> i32 {
        self.get(index)
    }

    /// Sets a capture at the given index.
    pub fn set_capture(&self, index: usize, value: i32) {
        self.set(index, value);
    }

    /// Gets the value at the given index.
    fn get(&self, index: usize) -> i32 {
        self.captures[index] // Assuming `get` accesses the FixedArray; needs adaptation based on FixedArray's actual implementation.
    }

    /// Sets the value at the given index.
    fn set(&mut self, index: usize, value: i32) {
        self.captures[index] = value; // Assuming `set` mutates the FixedArray; needs adaptation based on FixedArray's actual implementation.
    }
}