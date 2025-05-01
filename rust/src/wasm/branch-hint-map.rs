// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::collections::HashMap;

/// Represents a hint for a branch instruction.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BranchHint {
    /// No hint.
    kNone,
    // Add other branch hint types as needed based on `src/wasm/wasm-opcodes.h`
    // For example:
    // kUnlikely,
    // kLikely,
}

/// A map from offset to branch hint.
#[derive(Default, Debug)]
pub struct BranchHintMap {
    map_: HashMap<u32, BranchHint>,
}

impl BranchHintMap {
    /// Inserts a branch hint for the given offset.
    pub fn insert(&mut self, offset: u32, hint: BranchHint) {
        self.map_.insert(offset, hint);
    }

    /// Gets the branch hint for the given offset.
    pub fn get_hint_for(&self, offset: u32) -> BranchHint {
        self.map_.get(&offset).copied().unwrap_or(BranchHint::kNone)
    }

    /// Returns the number of hints in the map (for testing).
    pub fn num_hints_for_testing(&self) -> usize {
        self.map_.len()
    }
}

/// A map from offset to `BranchHintMap`.
pub type BranchHintInfo = HashMap<u32, BranchHintMap>;