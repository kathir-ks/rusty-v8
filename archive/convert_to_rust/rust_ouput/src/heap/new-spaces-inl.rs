// Converted from V8 C++ source files:
// Header: new-spaces-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::ptr::null_mut;

use crate::SemiSpace;
use crate::NewSpace;

//use crate::internal::Cast;
//use crate::internal::HeapObject;
//use crate::internal::Object;
//use crate::internal::Tagged;

pub struct HeapObject {}
pub struct Object {}
pub struct Tagged<T>(T);

impl<T> Tagged<T> {
    pub fn from_address(_address: usize) -> Self {
        Tagged(unsafe { std::mem::zeroed() })
    }
    pub fn Size(&self) -> usize {
        0
    }
}
//use crate::internal::MemoryChunk;
//use crate::internal::PageMetadata;

const kTaggedCanConvertToRawObjects: bool = true;

pub struct MemoryChunk {}

impl MemoryChunk {
    pub fn FromHeapObject(_o: Tagged<HeapObject>) -> *mut MemoryChunk {
        null_mut()
    }
    pub fn IsLargePage(&self) -> bool {
        false
    }
    pub fn IsToPage(&self) -> bool {
        false
    }
    pub fn IsFromPage(&self) -> bool {
        false
    }
    pub fn InNewSpace(&self) -> bool {
        false
    }
}

pub struct PageMetadata {}

impl PageMetadata {
    pub fn IsAlignedToPageSize(_address: usize) -> bool {
        false
    }
    pub fn FromAllocationAreaAddress(_address: usize) -> *mut PageMetadata {
        null_mut()
    }
    pub fn next_page(&self) -> *mut PageMetadata {
        null_mut()
    }
    pub fn area_start(&self) -> usize {
        0
    }
}

fn IsHeapObject(_o: Tagged<Object>) -> bool {
    false
}

fn Cast<T>(_o: Tagged<Object>) -> Tagged<HeapObject> {
    Tagged(HeapObject {})
}

fn ALIGN_TO_ALLOCATION_ALIGNMENT(size: usize) -> usize {
    size
}

fn IsFreeSpaceOrFiller(_object: Tagged<HeapObject>) -> bool {
    false
}

impl SemiSpace {
    fn id_(&self) -> i32 {
        0
    }
    fn kToSpace(&self) -> i32 {
        0
    }

    pub fn Contains(&self, o: Tagged<HeapObject>) -> bool {
        let memory_chunk = unsafe { MemoryChunk::FromHeapObject(o) };
        if memory_chunk.is_null() {
            return false;
        }
        let memory_chunk_ref = unsafe { &*memory_chunk };
        if memory_chunk_ref.IsLargePage() {
            return false;
        }
        return if self.id_() == self.kToSpace() {
            memory_chunk_ref.IsToPage()
        } else {
            memory_chunk_ref.IsFromPage()
        };
    }

    pub fn Contains_obj(&self, o: Tagged<Object>) -> bool {
        IsHeapObject(o) && self.Contains(Cast::<HeapObject>(o))
    }

    pub fn Contains_gen<T>(&self, o: Tagged<T>) -> bool {
        self.Contains_obj(Tagged::<Object>(unsafe{std::mem::transmute(o)}))
    }

    pub fn ContainsSlow(&self, a: usize) -> bool {
        false
    }
}

impl NewSpace {
    pub fn Contains(&self, o: Tagged<Object>) -> bool {
        IsHeapObject(o) && self.Contains_heapobject(Cast::<HeapObject>(o))
    }

    pub fn Contains_heapobject(&self, o: Tagged<HeapObject>) -> bool {
        unsafe { MemoryChunk::FromHeapObject(o) }.InNewSpace()
    }
}

pub struct SemiSpaceObjectIterator {
    current_: usize,
}

impl SemiSpaceObjectIterator {
    pub fn new(space: *const SemiSpaceNewSpace) -> SemiSpaceObjectIterator {
        SemiSpaceObjectIterator {
            current_: unsafe { (*space).first_allocatable_address() },
        }
    }

    pub fn Next(&mut self) -> Tagged<HeapObject> {
        loop {
            if PageMetadata::IsAlignedToPageSize(self.current_) {
                let page = unsafe { PageMetadata::FromAllocationAreaAddress(self.current_) };
                let page_ref = unsafe { &*page };
                let next_page = page_ref.next_page();
                if next_page.is_null() {
                    return Tagged(HeapObject {});
                }
                self.current_ = unsafe { (*next_page).area_start() };
            }
            let object = Tagged::<HeapObject>::from_address(self.current_);
            self.current_ += ALIGN_TO_ALLOCATION_ALIGNMENT(object.Size());
            if !IsFreeSpaceOrFiller(object) {
                return object;
            }
        }
    }
}

pub struct SemiSpaceNewSpace {
    allocation_top_: usize,
}

impl SemiSpaceNewSpace {
    pub fn first_allocatable_address(&self) -> usize {
        0
    }
    pub fn IncrementAllocationTop(&mut self, new_top: usize) {
        self.allocation_top_ = new_top;
    }

    pub fn DecrementAllocationTop(&mut self, new_top: usize) {
        self.allocation_top_ = new_top;
    }
}
