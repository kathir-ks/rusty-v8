// Copyright 2015 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/interpreter/bytecode-register.h (Inferred module definition)

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Register {
    index: i32, // Assuming index is an i32 based on context.  Adjust if needed.
}

impl Register {
    pub fn new(index: i32) -> Self {
        Register { index }
    }

    pub fn index(&self) -> i32 {
        self.index
    }

    pub fn is_current_context(&self) -> bool {
        self.index == -1 // Placeholder: Define based on V8's semantics
    }

    pub fn is_function_closure(&self) -> bool {
        self.index == -2 // Placeholder: Define based on V8's semantics
    }

    pub fn virtual_accumulator() -> Self {
       Register { index: -3 } // Placeholder: Define based on V8's semantics
    }

    pub fn is_parameter(&self) -> bool {
        self.index >= 0 && self.index < 1000 // Placeholder: Define range based on V8's semantics
    }

    pub fn to_parameter_index(&self) -> i32 {
        self.index
    }
}

impl Register {
    /// Converts the Register to a String representation.
    pub fn to_string(&self) -> String {
        if self.is_current_context() {
            "<context>".to_string()
        } else if self.is_function_closure() {
            "<closure>".to_string()
        } else if *self == Register::virtual_accumulator() {
            "<accumulator>".to_string()
        } else if self.is_parameter() {
            let parameter_index = self.to_parameter_index();
            if parameter_index == 0 {
                "<this>".to_string()
            } else {
                format!("a{}", parameter_index - 1)
            }
        } else {
            format!("r{}", self.index())
        }
    }
}