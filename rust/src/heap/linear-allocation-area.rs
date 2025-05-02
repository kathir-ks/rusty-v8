// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This module represents the LinearAllocationArea from V8.

use std::mem;
use std::ptr::null_mut;

//use v8::internal::*; // Assuming v8::internal is translated to this crate.

// Placeholder for Address.  Using usize as a reasonable equivalent.
// Consider using a safer wrapper around usize for address manipulation.
pub type Address = usize;

const K_NULL_ADDRESS: Address = 0;

// Placeholder for kSystemPointerSize.  Assuming a 64-bit system.
const K_SYSTEM_POINTER_SIZE: usize = 8;

// Placeholder for kObjectAlignment. Assuming 8-byte alignment.
const K_OBJECT_ALIGNMENT: usize = 8;

// Placeholder for kObjectAlignment8GbHeap. Assuming 8-byte alignment.
const K_OBJECT_ALIGNMENT_8GB_HEAP: usize = 8;

// Placeholder for V8_COMPRESS_POINTERS_8GB_BOOL.  Default to false.
const V8_COMPRESS_POINTERS_8GB_BOOL: bool = false;

/// A linear allocation area to allocate objects from.
///
/// Invariant that must hold at all times:
///   start <= top <= limit
#[derive(Default, Debug)]
pub struct LinearAllocationArea {
    // The start of the LAB. Initially coincides with `top_`. As top is moved
    // ahead, the area [start_, top_[ denotes a range of new objects. This range
    // is reset with `ResetStart()`.
    start_: Address,
    // The top of the LAB that is used for allocation.
    top_: Address,
    // Limit of the LAB the denotes the end of the valid range for allocation.
    limit_: Address,
}

impl LinearAllocationArea {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_values(top: Address, limit: Address) -> Self {
        let mut area = LinearAllocationArea {
            start_: top,
            top_: top,
            limit_: limit,
        };
        area.verify();
        area
    }

    pub fn reset(&mut self, top: Address, limit: Address) {
        self.start_ = top;
        self.top_ = top;
        self.limit_ = limit;
        self.verify();
    }

    pub fn reset_start(&mut self) {
        self.start_ = self.top_;
    }

    #[inline]
    pub fn can_increment_top(&self, bytes: usize) -> bool {
        self.verify();
        (self.top_ + bytes) <= self.limit_
    }

    #[inline]
    pub fn increment_top(&mut self, bytes: usize) -> Address {
        let old_top = self.top_;
        self.top_ += bytes;
        self.verify();
        old_top
    }

    #[inline]
    pub fn decrement_top_if_adjacent(&mut self, new_top: Address, bytes: usize) -> bool {
        self.verify();
        if (new_top + bytes) == self.top_ {
            self.top_ = new_top;
            if self.start_ > self.top_ {
                self.reset_start();
            }
            self.verify();
            return true;
        }
        false
    }

    #[inline]
    pub fn merge_if_adjacent(&mut self, other: &mut LinearAllocationArea) -> bool {
        self.verify();
        other.verify();
        if self.top_ == other.limit_ {
            self.top_ = other.top_;
            self.start_ = other.start_;
            other.reset(K_NULL_ADDRESS, K_NULL_ADDRESS);
            self.verify();
            return true;
        }
        false
    }

    #[inline]
    pub fn set_limit(&mut self, limit: Address) {
        self.limit_ = limit;
        self.verify();
    }

    #[inline]
    pub fn start(&self) -> Address {
        self.verify();
        self.start_
    }

    #[inline]
    pub fn top(&self) -> Address {
        self.verify();
        self.top_
    }

    #[inline]
    pub fn limit(&self) -> Address {
        self.verify();
        self.limit_
    }

    #[inline]
    pub fn top_address(&self) -> *const Address {
        &self.top_ as *const Address
    }

    #[inline]
    pub fn top_address_mut(&mut self) -> *mut Address {
        &mut self.top_ as *mut Address
    }

    #[inline]
    pub fn limit_address(&self) -> *const Address {
        &self.limit_ as *const Address
    }

    #[inline]
    pub fn limit_address_mut(&mut self) -> *mut Address {
        &mut self.limit_ as *mut Address
    }

    #[cfg(debug_assertions)]
    fn verify(&self) {
        if self.start_ > self.top_ {
          panic!("start_ > top_");
        }
        if self.top_ > self.limit_ {
          panic!("top_ > limit_");
        }

        if V8_COMPRESS_POINTERS_8GB_BOOL {
            if self.top_ % K_OBJECT_ALIGNMENT_8GB_HEAP != 0 {
                panic!("top_ not aligned to K_OBJECT_ALIGNMENT_8GB_HEAP");
            }
        } else {
            if self.top_ % K_OBJECT_ALIGNMENT != 0 {
                panic!("top_ not aligned to K_OBJECT_ALIGNMENT");
            }
        }
    }

    #[cfg(not(debug_assertions))]
    fn verify(&self) {}


    pub const K_SIZE: usize = 3 * K_SYSTEM_POINTER_SIZE;
}

#[test]
fn test_linear_allocation_area_size() {
    assert_eq!(
        mem::size_of::<LinearAllocationArea>(),
        LinearAllocationArea::K_SIZE,
        "LinearAllocationArea's size must be small because it is included in IsolateData."
    );
}