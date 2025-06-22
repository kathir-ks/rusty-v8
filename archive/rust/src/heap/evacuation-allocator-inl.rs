// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Corresponds to V8_HEAP_EVACUATION_ALLOCATOR_INL_H_
// src/heap/evacuation-allocator-inl.h

use crate::common::globals::*;
use crate::heap::evacuation_allocator::*;
//use crate::heap::spaces_inl::*; // Assuming this contains inline methods,
//  we might need to move implementation to the main spaces.rs file or another
// appropriate location.
use crate::heap::spaces::SpaceAllocator;

impl EvacuationAllocator {
    /// Allocates memory in the specified space with the given size and alignment.
    pub fn allocate(
        &mut self,
        space: AllocationSpace,
        object_size: usize,
        alignment: AllocationAlignment,
    ) -> AllocationResult {
        if !self.shared_space_allocator.is_null() {
            debug_assert!(space != AllocationSpace::SharedSpace);
        }

        let object_size = align_to_allocation_alignment(object_size);

        match space {
            AllocationSpace::NewSpace => self.new_space_allocator.allocate_raw(object_size, alignment, AllocationOrigin::GC),
            AllocationSpace::OldSpace => self.old_space_allocator.allocate_raw(object_size, alignment, AllocationOrigin::GC),
            AllocationSpace::CodeSpace => self.code_space_allocator.allocate_raw(object_size, alignment, AllocationOrigin::GC),
            AllocationSpace::SharedSpace => unsafe { self.shared_space_allocator.as_mut().expect("Shared space allocator was unexpectedly null.").allocate_raw(object_size, alignment, AllocationOrigin::GC)},
            AllocationSpace::TrustedSpace => self.trusted_space_allocator.allocate_raw(object_size, alignment, AllocationOrigin::GC),
            _ => unreachable!(),
        }
    }
}

fn align_to_allocation_alignment(size: usize) -> usize {
    (size + ALLOCATION_ALIGNMENT - 1) & !(ALLOCATION_ALIGNMENT - 1)
}