// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::sync::{Mutex, MutexGuard};
//use std::result::Result;

// TODO: Add logging crate
// use log::{debug, info, warn};
//use crate::base::platform::mutex::Mutex;
//use crate::heap::cppgc::heap_page::HeapPage; // Assuming this is in a separate module
//use crate::heap::cppgc::object_start_bitmap::ObjectStartBitmap; // Assuming this is in a separate module

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum PageType {
    kNormal,
    kLarge,
}

pub struct BaseSpace {
    heap_: *mut RawHeap, // TODO: Replace with a proper lifetime and ownership model
    index_: usize,
    type_: PageType,
    is_compactable_: bool,
    pages_: Vec<*mut BasePage>, // TODO: Replace with a proper lifetime and ownership model (Box<>)
    pages_mutex_: Mutex<()>,
}

impl BaseSpace {
    pub fn new(heap: *mut RawHeap, index: usize, type_: PageType, is_compactable: bool) -> Self {
        BaseSpace {
            heap_: heap,
            index_: index,
            type_: type_,
            is_compactable_: is_compactable,
            pages_: Vec::new(),
            pages_mutex_: Mutex::new(()),
        }
    }

    // This simulates the USE() macro
    #[allow(dead_code)]
    fn use_is_compactable(&self) {
        let _ = self.is_compactable_;
    }

    pub fn add_page(&mut self, page: *mut BasePage) {
        let _lock = self.pages_mutex_.lock().unwrap();
        assert!(!self.pages_.iter().any(|&p| p == page));
        self.pages_.push(page);
    }

    pub fn remove_page(&mut self, page: *mut BasePage) {
        let _lock = self.pages_mutex_.lock().unwrap();
        let index = self.pages_.iter().position(|&p| p == page).unwrap();
        self.pages_.remove(index);
    }

    pub fn remove_all_pages(&mut self) -> Vec<*mut BasePage> {
        let mut pages = Vec::new();
        std::mem::swap(&mut pages, &mut self.pages_);
        pages
    }
}

pub struct NormalPageSpace {
    base: BaseSpace,
}

impl NormalPageSpace {
    pub fn new(heap: *mut RawHeap, index: usize, is_compactable: bool) -> Self {
        NormalPageSpace {
            base: BaseSpace::new(heap, index, PageType::kNormal, is_compactable),
        }
    }
}

pub struct LargePageSpace {
    base: BaseSpace,
}

impl LargePageSpace {
    pub fn new(heap: *mut RawHeap, index: usize) -> Self {
        LargePageSpace {
            base: BaseSpace::new(heap, index, PageType::kLarge, false),
        }
    }
}

// Dummy types to allow compilation.  These must be replaced with the
// correct types from the V8 codebase.
pub struct RawHeap {}
pub struct BasePage {}