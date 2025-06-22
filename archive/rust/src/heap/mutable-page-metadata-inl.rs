// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/heap/mutable-page-metadata-inl.h

use crate::heap::{
    memory_chunk_metadata::MemoryChunkMetadata,
    mutable_page_metadata::MutablePageMetadata,
    spaces::AllocationSpace,
    spaces::Space,
};
use std::convert::TryInto;
use std::ptr::NonNull;

impl MutablePageMetadata {
    /// Returns the `MutablePageMetadata` associated with the given address.
    pub fn from_address(a: usize) -> *mut MutablePageMetadata {
        MemoryChunkMetadata::from_address(a) as *mut MutablePageMetadata
    }

    /// Returns the `MutablePageMetadata` associated with the given `HeapObject`.
    pub fn from_heap_object(o: usize) -> *mut MutablePageMetadata {
        MemoryChunkMetadata::from_heap_object(o) as *mut MutablePageMetadata
    }

    /// Increments the external backing store bytes of the given type by the given
    /// amount.
    pub fn increment_external_backing_store_bytes(
        &mut self,
        type_: ExternalBackingStoreType,
        amount: usize,
    ) {
        self.external_backing_store_bytes_[type_ as usize] = self.external_backing_store_bytes_[type_ as usize].wrapping_add(amount);
        self.owner().increment_external_backing_store_bytes(type_, amount);
    }

    /// Decrements the external backing store bytes of the given type by the given
    /// amount.
    pub fn decrement_external_backing_store_bytes(
        &mut self,
        type_: ExternalBackingStoreType,
        amount: usize,
    ) {
        self.external_backing_store_bytes_[type_ as usize] = self.external_backing_store_bytes_[type_ as usize].wrapping_sub(amount);
        self.owner().decrement_external_backing_store_bytes(type_, amount);
    }

    /// Moves external backing store bytes from one `MutablePageMetadata` to
    /// another.
    pub fn move_external_backing_store_bytes(
        type_: ExternalBackingStoreType,
        from: &mut MutablePageMetadata,
        to: &mut MutablePageMetadata,
        amount: usize,
    ) {
        assert!(from.owner().owner().is_some());
        assert!(to.owner().owner().is_some());
        from.external_backing_store_bytes_[type_ as usize] = from.external_backing_store_bytes_[type_ as usize].wrapping_sub(amount);
        to.external_backing_store_bytes_[type_ as usize] = to.external_backing_store_bytes_[type_ as usize].wrapping_add(amount);
        Space::move_external_backing_store_bytes(type_, from.owner(), to.owner(), amount);
    }

    /// Returns the allocation space identity of the owner.
    pub fn owner_identity(&self) -> AllocationSpace {
        assert_eq!(self.owner().owner().is_none(), self.chunk().in_read_only_space());
        if self.owner().owner().is_none() {
            AllocationSpace::RoSpace
        } else {
            self.owner().identity()
        }
    }

    /// Sets old generation page flags.
    pub fn set_old_generation_page_flags(&mut self, marking_mode: MarkingMode) {
        self.chunk_mut()
            .set_old_generation_page_flags(marking_mode, self.owner_identity());
    }

    /// Clears liveness bits.
    pub fn clear_liveness<const MODE: AccessMode>(&mut self) {
        self.marking_bitmap().clear::<MODE>();
        self.set_live_bytes(0);
    }

    fn chunk(&self) -> &MemoryChunkMetadata {
        unsafe { &*(self as *const Self as *const MemoryChunkMetadata) }
    }
    fn chunk_mut(&mut self) -> &mut MemoryChunkMetadata {
        unsafe { &mut *(self as *mut Self as *mut MemoryChunkMetadata) }
    }
}

// Placeholder enum
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ExternalBackingStoreType {
    Malloced,
    MarkingBitmap,
}

// Placeholder enum
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MarkingMode {
    kConcurrentPage,
    kIncrementalPage,
}

// Placeholder enum
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AccessMode {
    NonAtomic,
    Atomic,
}