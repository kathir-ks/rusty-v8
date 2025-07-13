// Converted from V8 C++ source files:
// Header: live-object-range-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::ops::{Add, Deref, DerefMut};
use std::ptr::null_mut;

use crate::heap::safepoint::V8;
use crate::heap::scavenger::iterator;

//use crate::heap::heap::Heap;
use crate::heap::page_metadata_inl::PageMetadata;
use crate::objects::instance_type_inl::InstanceTypeChecker;
use crate::objects::map::MapWord;
use crate::heap::{Heap, MemoryChunk};
use crate::base::bits;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Address(pub usize);

impl Address {
    pub fn is_null(&self) -> bool {
        self.0 == 0
    }
}

impl Add<usize> for Address {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Address(self.0 + rhs)
    }
}

pub struct HeapObject {
    address: Address,
}

impl HeapObject {
    pub fn FromAddress(address: Address) -> Self {
        HeapObject { address }
    }

    pub fn address(&self) -> Address {
        self.address
    }

    pub fn map(&self, cage_base: *mut V8, load_type: i32) -> *mut Map {
        unsafe {
            (self.address.0 as *mut Map)
        }
    }

    pub fn SizeFromMap(&self, map: *mut Map) -> usize {
       16
    }

    pub fn is_null(&self) -> bool {
        self.address.is_null()
    }
}

// Dummy Map struct
pub struct Map {}

pub struct LiveObjectRange {
    page: *const PageMetadata,
}

impl LiveObjectRange {
    pub fn begin(&self) -> LiveObjectRangeIterator {
        LiveObjectRangeIterator::new(unsafe { self.page.as_ref().unwrap() })
    }

    pub fn end(&self) -> LiveObjectRangeIterator {
        LiveObjectRangeIterator::default()
    }
}

pub struct LiveObjectRangeIterator {
    page_: *const PageMetadata,
    cells_: *mut u8,
    cage_base_: *mut V8,
    current_cell_index_: usize,
    current_cell_: u8,
    current_object_: HeapObject,
    current_map_: *mut Map,
    current_size_: usize,
}

const K_TAGGED_SIZE: usize = 8;
const MARKING_BITMAP_K_CELLS_COUNT: usize = 512;

impl LiveObjectRangeIterator {
    pub fn new(page: &PageMetadata) -> Self {
        let cells_ptr = page.marking_bitmap_cells();
        let area_start = page.area_start();
        let isolate = page.heap_isolate();

        let current_cell_index_ = MarkingBitmap::address_to_index(area_start) / 8;

        let current_cell_ = unsafe { *cells_ptr.add(current_cell_index_) };

        let mut iterator = LiveObjectRangeIterator {
            page_: page,
            cells_: cells_ptr,
            cage_base_: isolate,
            current_cell_index_: current_cell_index_,
            current_cell_: current_cell_,
            current_object_: HeapObject { address: Address(0)},
            current_map_: null_mut(),
            current_size_: 0,
        };
        iterator.advance_to_next_valid_object();
        iterator
    }

    fn advance_to_next_valid_object(&mut self) {
        while self.advance_to_next_marked_object() && InstanceTypeChecker::IsFreeSpaceOrFiller(self.current_map_) {}
    }

    fn advance_to_next_marked_object(&mut self) -> bool {
        if !self.current_object_.is_null() {
            let next_object = self.current_object_.address() + self.current_size_;
            self.current_object_ = HeapObject { address: Address(0)};

            if (next_object.0 % 8) == 0 {
                return false;
            }

            let page = unsafe { self.page_.as_ref().unwrap() };
            if next_object.0 > page.area_end().0 {
                return false;
            }

            let next_markbit_index = MarkingBitmap::address_to_index(next_object);
            if (next_markbit_index / 8) < self.current_cell_index_ {
               return false;
            }
            self.current_cell_index_ = next_markbit_index / 8;

            let mask = MarkingBitmap::index_in_cell_mask(next_markbit_index);
            unsafe {
                self.current_cell_ = *self.cells_.add(self.current_cell_index_) & !((mask - 1) as u8);
            }
        }

        let page = unsafe { self.page_.as_ref().unwrap() };
        let chunk = page.chunk();

        loop {
            if self.current_cell_ != 0 {
                let trailing_zeros = bits::CountTrailingZeros(self.current_cell_ as u32);
                let current_cell_base = chunk.address() + (self.current_cell_index_ * 8);
                let object_address = current_cell_base + (trailing_zeros as usize * K_TAGGED_SIZE);

                self.current_object_ = HeapObject::FromAddress(Address(object_address));

                let cage_base = unsafe { self.cage_base_.as_mut().unwrap() };
                self.current_map_ = self.current_object_.map(cage_base, 0);
                self.current_size_ = align_to_allocation_alignment(self.current_object_.SizeFromMap(self.current_map_));
                if (object_address + self.current_size_) > page.area_end().0 {
                   return false;
                }
                return true;
            }

            self.current_cell_index_ += 1;
            if self.current_cell_index_ >= MARKING_BITMAP_K_CELLS_COUNT {
                break;
            }

            unsafe {
                self.current_cell_ = *self.cells_.add(self.current_cell_index_);
            }
        }
        return false;
    }
}

impl Default for LiveObjectRangeIterator {
    fn default() -> Self {
        LiveObjectRangeIterator {
            page_: null_mut(),
            cells_: null_mut(),
            cage_base_: null_mut(),
            current_cell_index_: 0,
            current_cell_: 0,
            current_object_: HeapObject { address: Address(0)},
            current_map_: null_mut(),
            current_size_: 0,
        }
    }
}

impl Iterator for LiveObjectRangeIterator {
    type Item = HeapObject;

    fn next(&mut self) -> Option<Self::Item> {
        if self.advance_to_next_marked_object() {
            Some(self.current_object_)
        } else {
            None
        }
    }
}

fn align_to_allocation_alignment(size: usize) -> usize {
    (size + 7) & !7
}

pub struct MarkingBitmap {}

impl MarkingBitmap {
    pub fn address_to_index(address: Address) -> usize {
        address.0 / 8
    }

    pub fn index_in_cell_mask(index: usize) -> usize {
        1 << (index % 8)
    }
}

impl PageMetadata {
    pub fn marking_bitmap_cells(&self) -> *mut u8 {
        self.marking_bitmap_cells_internal()
    }
    pub fn heap_isolate(&self) -> *mut V8 {
        self.heap_isolate_internal()
    }

    pub fn area_start(&self) -> Address {
        self.area_start_internal()
    }

    pub fn area_end(&self) -> Address {
        self.area_end_internal()
    }

    pub fn chunk(&self) -> &MemoryChunk {
        self.chunk_internal()
    }
}

trait PageMetadataAccess {
    fn marking_bitmap_cells_internal(&self) -> *mut u8;
    fn heap_isolate_internal(&self) -> *mut V8;
    fn area_start_internal(&self) -> Address;
    fn area_end_internal(&self) -> Address;
    fn chunk_internal(&self) -> &MemoryChunk;
}
