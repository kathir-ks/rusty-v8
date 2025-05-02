// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// Atomic memory orders supported by the compiler.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AtomicMemoryOrder {
    /// Acquire Release
    AcqRel,
    /// Sequentially Consistent
    SeqCst,
}

impl AtomicMemoryOrder {
    /// Returns a hash value for the AtomicMemoryOrder.
    pub fn hash_value(&self) -> usize {
        *self as u8 as usize
    }
}

impl std::fmt::Display for AtomicMemoryOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AtomicMemoryOrder::AcqRel => write!(f, "kAcqRel"),
            AtomicMemoryOrder::SeqCst => write!(f, "kSeqCst"),
        }
    }
}