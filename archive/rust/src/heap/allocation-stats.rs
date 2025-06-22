// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::HashMap;
use std::hash::{Hasher, BuildHasherDefault};

//use crate::base::hashing::Hasher; // Assuming a custom hasher, replace with a suitable alternative if needed.
use crate::base::macros::*; // Assuming macros are defined in this module
use crate::heap::memory_chunk_metadata::MemoryChunkMetadata;

// Placeholder for a custom hasher, replace with a suitable alternative if needed.
#[derive(Default)]
struct IdentityHasher(usize);

impl Hasher for IdentityHasher {
    fn finish(&self) -> u64 {
        self.0 as u64
    }

    fn write(&mut self, bytes: &[u8]) {
        self.0 = usize::from_ne_bytes(bytes.try_into().unwrap_or([0; std::mem::size_of::<usize>()]));
    }
}

type IdentityBuildHasher = BuildHasherDefault<IdentityHasher>;

mod base {
    pub mod hashing {
        pub struct Hasher {}
    }
    pub mod macros {
        #[macro_export]
        macro_rules! DCHECK_IMPLIES {
            ($cond1:expr, $cond2:expr) => {
                debug_assert!(!$cond1 || $cond2);
            };
        }
        #[macro_export]
        macro_rules! V8_NOEXCEPT {
            () => {
            };
        }

        pub const V8_COMPRESS_POINTERS_8GB_BOOL: bool = false;
    }
}

mod heap {
    pub mod memory_chunk_metadata {
        #[derive(PartialEq, Eq, Hash, Copy, Clone)]
        pub struct MemoryChunkMetadata {}
    }
}

const kObjectAlignment8GbHeap: usize = 8;

fn is_aligned(value: usize, alignment: usize) -> bool {
    value % alignment == 0
}

/// An abstraction of the accounting statistics of a page-structured space.
///
/// The stats are only set by functions that ensure they stay balanced. These
/// functions increase or decrease one of the non-capacity stats in conjunction
/// with capacity, or else they always balance increases and decreases to the
/// non-capacity stats.
pub struct AllocationStats {
    capacity_: AtomicUsize,
    max_capacity_: usize,
    size_: AtomicUsize,
    #[cfg(debug_assertions)]
    allocated_on_page_: HashMap<heap::memory_chunk_metadata::MemoryChunkMetadata, usize>,
}

impl AllocationStats {
    pub fn new() -> Self {
        let mut stats = Self {
            capacity_: AtomicUsize::new(0),
            max_capacity_: 0,
            size_: AtomicUsize::new(0),
            #[cfg(debug_assertions)]
            allocated_on_page_: HashMap::new(),
        };
        stats.clear();
        stats
    }

    pub fn clear(&mut self) {
        self.capacity_.store(0, Ordering::Relaxed);
        self.max_capacity_ = 0;
        self.clear_size();
    }

    pub fn clear_size(&mut self) {
        self.size_.store(0, Ordering::Relaxed);
        #[cfg(debug_assertions)]
        self.allocated_on_page_.clear();
    }

    /// Accessors for the allocation statistics.
    pub fn capacity(&self) -> usize {
        self.capacity_.load(Ordering::Relaxed)
    }

    pub fn max_capacity(&self) -> usize {
        self.max_capacity_
    }

    pub fn size(&self) -> usize {
        self.size_.load(Ordering::Relaxed)
    }

    #[cfg(debug_assertions)]
    pub fn allocated_on_page(&self, page: &heap::memory_chunk_metadata::MemoryChunkMetadata) -> usize {
        *self.allocated_on_page_.get(page).unwrap()
    }

    pub fn increase_allocated_bytes(&mut self, bytes: usize, page: &heap::memory_chunk_metadata::MemoryChunkMetadata) {
        DCHECK_IMPLIES!(base::macros::V8_COMPRESS_POINTERS_8GB_BOOL,
                           is_aligned(bytes, kObjectAlignment8GbHeap));
        #[cfg(debug_assertions)]
        {
            let size = self.size_.load(Ordering::Relaxed);
            debug_assert!(size.wrapping_add(bytes) >= size);
        }

        self.size_.fetch_add(bytes, Ordering::Relaxed);

        #[cfg(debug_assertions)]
        {
            *self.allocated_on_page_.entry(*page).or_insert(0) += bytes;
        }
    }

    pub fn decrease_allocated_bytes(&mut self, bytes: usize, page: &heap::memory_chunk_metadata::MemoryChunkMetadata) {
        let size = self.size_.load(Ordering::Relaxed);
        debug_assert!(size >= bytes);
        self.size_.fetch_sub(bytes, Ordering::Relaxed);

        #[cfg(debug_assertions)]
        {
            let allocated = self.allocated_on_page_.get(page).copied().unwrap_or(0);
            debug_assert!(allocated >= bytes);
             *self.allocated_on_page_.entry(*page).or_insert(0) -= bytes;
        }
    }

    pub fn decrease_capacity(&mut self, bytes: usize) {
        let capacity = self.capacity_.load(Ordering::Relaxed);
        debug_assert!(capacity >= bytes);
        debug_assert!(capacity - bytes >= self.size_.load(Ordering::Relaxed));
        self.capacity_.fetch_sub(bytes, Ordering::Relaxed);
    }

    pub fn increase_capacity(&mut self, bytes: usize) {
        let capacity = self.capacity_.load(Ordering::Relaxed);
        debug_assert!(capacity.wrapping_add(bytes) >= capacity);
        self.capacity_.fetch_add(bytes, Ordering::Relaxed);

        let new_capacity = self.capacity_.load(Ordering::Relaxed);
        if new_capacity > self.max_capacity_ {
            self.max_capacity_ = new_capacity;
        }
    }
}

impl Clone for AllocationStats {
    fn clone(&self) -> Self {
        AllocationStats {
            capacity_: AtomicUsize::new(self.capacity_.load(Ordering::Relaxed)),
            max_capacity_: self.max_capacity_,
            size_: AtomicUsize::new(self.size_.load(Ordering::Relaxed)),
            #[cfg(debug_assertions)]
            allocated_on_page_: self.allocated_on_page_.clone(),
        }
    }
}