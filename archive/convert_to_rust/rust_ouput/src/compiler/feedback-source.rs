// Converted from V8 C++ source files:
// Header: feedback-source.h
// Implementation: feedback-source.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

use std::fmt;
use std::hash::{Hash, Hasher};

pub struct FeedbackSource {
    vector: Option<IndirectHandle<FeedbackVector>>,
    slot: FeedbackSlot,
}

impl FeedbackSource {
    pub fn new() -> Self {
        FeedbackSource {
            vector: None,
            slot: FeedbackSlot::new_invalid(),
        }
    }

    pub fn new_with_indirect_handle(vector_: IndirectHandle<FeedbackVector>, slot_: FeedbackSlot) -> Self {
        assert!(!slot_.is_invalid());
        FeedbackSource {
            vector: Some(vector_),
            slot: slot_,
        }
    }

    pub fn new_with_feedback_vector_ref(vector_: FeedbackVectorRef, slot_: FeedbackSlot) -> Self {
        FeedbackSource::new_with_indirect_handle(vector_.object(), slot_)
    }

    pub fn is_valid(&self) -> bool {
        match &self.vector {
            Some(vector) => !vector.is_null() && !self.slot.is_invalid(),
            None => false,
        }
    }

    pub fn index(&self) -> usize {
        assert!(self.is_valid());
        FeedbackVector::get_index(&self.slot)
    }
}

impl Hash for FeedbackSource {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match &self.vector {
            Some(vector) => {
                vector.address().hash(state);
            }
            None => {
                0.hash(state); // Or some other sentinel value
            }
        }
        self.slot.hash(state);
    }
}

impl PartialEq for FeedbackSource {
    fn eq(&self, other: &Self) -> bool {
        match (&self.vector, &other.vector) {
            (Some(lhs_vector), Some(rhs_vector)) => lhs_vector.equals(rhs_vector) && self.slot == other.slot,
            (None, None) => self.slot == other.slot, // Consider two invalid FeedbackSources equal
            _ => false, // If only one is valid, they are not equal
        }
    }
}

impl Eq for FeedbackSource {}

impl fmt::Display for FeedbackSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_valid() {
            write!(f, "FeedbackSource({})", self.slot)
        } else {
            write!(f, "FeedbackSource(INVALID)")
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FeedbackSlot {
    index: usize,
    is_invalid: bool,
}

impl FeedbackSlot {
    pub fn new(index: usize) -> Self {
        FeedbackSlot {
            index,
            is_invalid: false,
        }
    }

    pub fn new_invalid() -> Self {
        FeedbackSlot {
            index: 0, // Or any default value
            is_invalid: true,
        }
    }

    pub fn is_invalid(&self) -> bool {
        self.is_invalid
    }

    // You can add methods to access the index if needed
    pub fn index(&self) -> usize {
        self.index
    }
}

impl fmt::Display for FeedbackSlot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FeedbackSlot(index: {}, invalid: {})", self.index, self.is_invalid)
    }
}

#[derive(Debug, Clone)]
pub struct IndirectHandle<T> {
    ptr: *mut T,
}

impl<T> IndirectHandle<T> {
    pub fn new(ptr: *mut T) -> Self {
        IndirectHandle { ptr }
    }

    pub fn address(&self) -> *mut T {
        self.ptr
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    pub fn equals(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }

    pub fn object(&self) -> Self {
        IndirectHandle{ptr: self.ptr}
    }
}

// Dummy FeedbackVector and FeedbackVectorRef for compilation
pub struct FeedbackVector {}

impl FeedbackVector {
    pub fn get_index(slot: &FeedbackSlot) -> usize {
        slot.index()
    }
}

pub struct FeedbackVectorRef {
    object: IndirectHandle<FeedbackVector>,
}

impl FeedbackVectorRef {
    pub fn object(&self) -> IndirectHandle<FeedbackVector> {
        self.object.clone()
    }
}
