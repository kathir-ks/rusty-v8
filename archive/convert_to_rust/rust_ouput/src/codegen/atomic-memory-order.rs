// Converted from V8 C++ source files:
// Header: atomic-memory-order.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AtomicMemoryOrder {
    kAcqRel,
    kSeqCst,
}

impl AtomicMemoryOrder {
    pub fn hash_value(&self) -> usize {
        *self as u8 as usize
    }
}

impl std::fmt::Display for AtomicMemoryOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AtomicMemoryOrder::kAcqRel => write!(f, "kAcqRel"),
            AtomicMemoryOrder::kSeqCst => write!(f, "kSeqCst"),
        }
    }
}
