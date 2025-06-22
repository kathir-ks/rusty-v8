// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::fmt;

/// Represents source information for a bytecode.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct BytecodeSourceInfo {
    source_position: i32,
    is_statement: bool,
    is_valid: bool,
}

impl BytecodeSourceInfo {
    /// Creates a new `BytecodeSourceInfo`.
    pub fn new(source_position: i32, is_statement: bool, is_valid: bool) -> Self {
        BytecodeSourceInfo {
            source_position,
            is_statement,
            is_valid,
        }
    }

    /// Returns the source position.
    pub fn source_position(&self) -> i32 {
        self.source_position
    }

    /// Returns whether this info represents a statement.
    pub fn is_statement(&self) -> bool {
        self.is_statement
    }

    /// Returns whether this info is valid.
    pub fn is_valid(&self) -> bool {
        self.is_valid
    }
}

impl fmt::Display for BytecodeSourceInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_valid {
            let description = if self.is_statement { 'S' } else { 'E' };
            write!(f, "{} {}>", self.source_position, description)
        } else {
            Ok(())
        }
    }
}