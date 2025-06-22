// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Note: The `AllStatic` base class from C++ doesn't have a direct
// equivalent in Rust.  It's likely used to prevent instantiation
// and enforce static methods.  In Rust, we can achieve this by making
// the struct empty and using `impl` blocks for the methods.

/// Represents integer values that can be stored in 31 bits.
/// On 32-bit architectures, it's the same as Smi, but on 64-bit
/// architectures, it differs by having a 31-bit payload and always
/// being sign-extended.
pub struct TaggedIndex {}

impl TaggedIndex {
    const K_SMI_TAG_SIZE: usize = 1;
    const K_TAGGED_VALUE_SIZE: usize = 31;
    const K_UINT_PTR_ALL_BITS_SET: isize = -1; // Assuming this represents all bits set for an isize
    const K_MIN_VALUE: isize = Self::K_UINT_PTR_ALL_BITS_SET << (Self::K_TAGGED_VALUE_SIZE - 1);
    const K_MAX_VALUE: isize = -(Self::K_MIN_VALUE + 1);

    /// Converts a value to a TaggedIndex object.
    pub fn from_intptr(value: isize) -> Tagged<TaggedIndex> {
        debug_assert!(TaggedIndex::is_valid(value));
        let smi_tag: isize = 0b1; // Assuming kSmiTag is 1
        Tagged::new(((value << Self::K_SMI_TAG_SIZE) | smi_tag) as usize) // Casting to usize to align with Tagged<T>
    }

    /// Returns whether the value can be represented in a TaggedIndex.
    pub const fn is_valid(value: isize) -> bool {
        Self::K_MIN_VALUE <= value && value <= Self::K_MAX_VALUE
    }
}

// Dummy types for translation only.
#[derive(Debug, Clone, Copy)]
pub struct Tagged<T> {
    ptr_: usize,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Tagged<T> {
    pub fn new(ptr: usize) -> Self {
        Tagged {
            ptr_: ptr,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn ptr(&self) -> usize {
        self.ptr_
    }
}

pub struct Object {}
pub struct HeapObject {}

trait CastTraitsT<T> {
    fn allow_from(value: Tagged<Object>) -> bool;
    fn allow_from_heap_object(value: Tagged<HeapObject>) -> bool;
}

struct CastTraits {}

impl CastTraitsT<TaggedIndex> for CastTraits {
    fn allow_from(value: Tagged<Object>) -> bool {
        has_smi_tag(value.ptr())
    }
    fn allow_from_heap_object(value: Tagged<HeapObject>) -> bool {
        false
    }
}

const fn has_smi_tag(value: usize) -> bool {
    (value & 1) == 1
}

// Dummy values for translation only.
type Address = usize;