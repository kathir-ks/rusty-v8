// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::cmp::Ordering;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use std::ops::{Deref, DerefMut, Index, IndexMut};

const kTaggedSize: usize = std::mem::size_of::<usize>(); // Assuming Tagged_t is usize

/// Represents a raw memory address.
pub type Address = usize;

/// Represents a tagged value.  Assuming Tagged_t is usize for simplicity.
pub type Tagged_t = usize;

const kNullAddress: Address = 0;

/// A wrapper for atomic access to slots in memory.
pub struct AtomicSlot {
    address: Address,
}

impl AtomicSlot {
    /// Creates a new `AtomicSlot` with a null address.
    pub fn new() -> Self {
        AtomicSlot { address: kNullAddress }
    }

    /// Creates a new `AtomicSlot` with the given address.
    pub fn with_address(address: Address) -> Self {
        AtomicSlot { address }
    }

    /// Gets the address of this slot.
    pub fn address(&self) -> Address {
        self.address
    }

    /// Creates a `Reference` to the value at the slot's address.
    pub fn get(&self) -> Reference {
        Reference {
            address: self.address as *mut Tagged_t,
        }
    }

    /// Gets a `Reference` to the element at the given offset.
    pub fn offset(&self, i: isize) -> Reference {
        Reference {
            address: (self.address as isize + i * kTaggedSize as isize) as *mut Tagged_t,
        }
    }

}

impl Index<isize> for AtomicSlot {
    type Output = Tagged_t;

    fn index(&self, index: isize) -> &Self::Output {
        unsafe {
            let ptr = (self.address as isize + index * kTaggedSize as isize) as *mut Tagged_t;
            &*ptr
        }
    }
}

impl IndexMut<isize> for AtomicSlot {
    fn index_mut(&mut self, index: isize) -> &mut Self::Output {
        unsafe {
            let ptr = (self.address as isize + index * kTaggedSize as isize) as *mut Tagged_t;
            &mut *ptr
        }
    }
}

/// A reference to a tagged value with atomic access.
pub struct Reference {
    address: *mut Tagged_t,
}

impl Reference {
    /// Reads the value from the address using relaxed ordering.
    pub fn load(&self) -> Tagged_t {
        unsafe {
            let atomic_ptr = self.address as *mut AtomicUsize;
            (*atomic_ptr).load(Relaxed) as Tagged_t
        }
    }

    /// Stores the value to the address using relaxed ordering.
    pub fn store(&mut self, value: Tagged_t) {
        unsafe {
            let atomic_ptr = self.address as *mut AtomicUsize;
            (*atomic_ptr).store(value as usize, Relaxed);
        }
    }

    /// Swaps the value at this reference with the value at another reference.
    pub fn swap(&mut self, other: &mut Reference) {
        let tmp = self.load();
        self.store(other.load());
        other.store(tmp);
    }
}

impl From<&Reference> for Tagged_t {
    fn from(reference: &Reference) -> Self {
        reference.load()
    }
}

impl PartialEq for Reference {
    fn eq(&self, other: &Self) -> bool {
        self.load() == other.load()
    }
}

impl Eq for Reference {}

impl PartialOrd for Reference {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.load().partial_cmp(&other.load())
    }
}

impl Ord for Reference {
    fn cmp(&self, other: &Self) -> Ordering {
        self.load().cmp(&other.load())
    }
}