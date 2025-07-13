// Converted from V8 C++ source files:
// Header: allocation-stats.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct MemoryChunkMetadata {}

const V8_COMPRESS_POINTERS_8GB_BOOL: bool = false;
const kObjectAlignment8GbHeap: usize = 8;

fn IsAligned(value: usize, alignment: usize) -> bool {
    value % alignment == 0
}

#[derive(Default)]
pub struct AllocationStats {
    capacity_: AtomicUsize,
    max_capacity_: usize,
    size_: AtomicUsize,
    #[cfg(debug_assertions)]
    allocated_on_page_: HashMap<MemoryChunkMetadata, usize>,
}

impl AllocationStats {
    pub fn new() -> Self {
        AllocationStats {
            capacity_: AtomicUsize::new(0),
            max_capacity_: 0,
            size_: AtomicUsize::new(0),
            #[cfg(debug_assertions)]
            allocated_on_page_: HashMap::new(),
        }
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
    pub fn allocated_on_page(&self, page: &MemoryChunkMetadata) -> usize {
        *self.allocated_on_page_.get(page).unwrap_or(&0)
    }

    pub fn increase_allocated_bytes(&self, bytes: usize, page: &MemoryChunkMetadata) {
        if V8_COMPRESS_POINTERS_8GB_BOOL {
            assert!(IsAligned(bytes, kObjectAlignment8GbHeap));
        }

        let size = self.size_.load(Ordering::Relaxed);

        self.size_.fetch_add(bytes, Ordering::Relaxed);

        #[cfg(debug_assertions)]
        {
            let mut map = self.allocated_on_page_.lock().unwrap();
            *map.entry(page.clone()).or_insert(0) += bytes;
        }
    }

    pub fn decrease_allocated_bytes(&self, bytes: usize, page: &MemoryChunkMetadata) {
        let size = self.size_.load(Ordering::Relaxed);
        assert!(size >= bytes);
        self.size_.fetch_sub(bytes, Ordering::Relaxed);

        #[cfg(debug_assertions)]
        {
           *self.allocated_on_page_.get_mut(page).unwrap() -= bytes;
        }
    }

    pub fn decrease_capacity(&mut self, bytes: usize) {
        let capacity = self.capacity_.load(Ordering::Relaxed);
        assert!(capacity >= bytes);
        assert!(capacity >= self.size());
        self.capacity_.fetch_sub(bytes, Ordering::Relaxed);
    }

    pub fn increase_capacity(&mut self, bytes: usize) {
        let capacity = self.capacity_.load(Ordering::Relaxed);

        self.capacity_.fetch_add(bytes, Ordering::Relaxed);
        let new_capacity = self.capacity_.load(Ordering::Relaxed);
        if new_capacity > self.max_capacity_ {
            self.max_capacity_ = new_capacity;
        }
    }
}
