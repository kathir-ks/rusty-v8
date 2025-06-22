// Copyright 2011 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/heap/spaces-inl.h

use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicUsize, Ordering};

//use crate::base::atomic_utils; // Assuming this will be a Rust equivalent
use crate::heap::heap::Heap;
use crate::heap::large_spaces::{LargePageIterator, LargeSpaces};
use crate::heap::main_allocator; // Assuming this will be a Rust equivalent
use crate::heap::mutable_page_metadata::{MutablePageMetadata, PageMetadata};
use crate::heap::new_spaces::NewSpace;
use crate::heap::paged_spaces::{PagedSpace, PageIterator};
use crate::heap::spaces::Space;

// Placeholder for the CheckedIncrement and CheckedDecrement functionality.
// This needs a proper Rust implementation based on atomic operations and error handling.
mod atomic_utils {
    pub fn checked_increment(atomic: &AtomicUsize, amount: usize) {
        atomic.fetch_add(amount, Ordering::Relaxed); // Basic version, needs overflow handling
    }

    pub fn checked_decrement(atomic: &AtomicUsize, amount: usize) {
        atomic.fetch_sub(amount, Ordering::Relaxed); // Basic version, needs underflow handling
    }
}

/// Iterator implementation for pages.
pub struct PageIteratorImpl<'a, PageType> {
    p_: *mut PageType,
    phantom: PhantomData<&'a mut PageType>,
}

impl<'a, PageType> PageIteratorImpl<'a, PageType> {
    pub fn new(page: *mut PageType) -> Self {
        PageIteratorImpl {
            p_: page,
            phantom: PhantomData,
        }
    }
}

impl<'a, PageType> Iterator for PageIteratorImpl<'a, PageType>
where
    PageType: PageTrait,
{
    type Item = &'a mut PageType;

    fn next(&mut self) -> Option<Self::Item> {
        if self.p_.is_null() {
            return None;
        }

        // The lifetime needs more thought
        let current = unsafe { &mut *self.p_ };
        self.p_ = current.next_page();
        Some(current)
    }
}

pub trait PageTrait {
    fn next_page(&self) -> *mut Self;
}

impl Space {
    /// Increments the external backing store bytes for a specific type.
    pub fn increment_external_backing_store_bytes(
        &self,
        type_: ExternalBackingStoreType,
        amount: usize,
    ) {
        atomic_utils::checked_increment(
            &self.external_backing_store_bytes_[type_ as usize],
            amount,
        );
        self.heap()
            .increment_external_backing_store_bytes(type_, amount);
    }

    /// Decrements the external backing store bytes for a specific type.
    pub fn decrement_external_backing_store_bytes(
        &self,
        type_: ExternalBackingStoreType,
        amount: usize,
    ) {
        atomic_utils::checked_decrement(
            &self.external_backing_store_bytes_[type_ as usize],
            amount,
        );
        self.heap()
            .decrement_external_backing_store_bytes(type_, amount);
    }

    /// Moves external backing store bytes from one space to another.
    pub fn move_external_backing_store_bytes(
        &self,
        type_: ExternalBackingStoreType,
        from: &Space,
        to: &Space,
        amount: usize,
    ) {
        if from as *const _ == to as *const _ {
            return;
        }

        atomic_utils::checked_decrement(
            &from.external_backing_store_bytes_[type_ as usize],
            amount,
        );
        atomic_utils::checked_increment(&to.external_backing_store_bytes_[type_ as usize], amount);
    }
}

#[derive(Debug)]
pub enum ExternalBackingStoreType {
    ArrayBuffer,
    WasmMemory,
    Other, // Add more types as needed
}

#[derive(Debug)]
pub struct PageRange<'a> {
    page: *mut PageMetadata,
    next_page: *mut PageMetadata,
    phantom: PhantomData<&'a mut PageMetadata>,
}

impl<'a> PageRange<'a> {
    pub fn new(page: *mut PageMetadata) -> Self {
        let next_page = unsafe { (*page).next_page() };
        PageRange {
            page,
            next_page,
            phantom: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct ConstPageRange<'a> {
    page: *const PageMetadata,
    next_page: *const PageMetadata,
    phantom: PhantomData<&'a PageMetadata>,
}

impl<'a> ConstPageRange<'a> {
    pub fn new(page: *const PageMetadata) -> Self {
        let next_page = unsafe { (*page).next_page() as *const PageMetadata };
        ConstPageRange {
            page,
            next_page,
            phantom: PhantomData,
        }
    }
}

enum OldGenState {
    OldSpace,
    CodeSpace,
    LargeObjectSpace,
    CodeLargeObjectSpace,
    TrustedSpace,
    TrustedLargeObjectSpace,
    Finished,
}

pub struct OldGenerationMemoryChunkIterator<'a> {
    heap_: &'a Heap,
    state_: OldGenState,
    iterator_: OldGenIteratorVariants<'a>,
}

enum OldGenIteratorVariants<'a> {
    PageIterator(PageIterator<'a>),
    LargePageIterator(LargePageIterator<'a>),
}

impl<'a> OldGenerationMemoryChunkIterator<'a> {
    pub fn new(heap: &'a Heap) -> Self {
        OldGenerationMemoryChunkIterator {
            heap_: heap,
            state_: OldGenState::OldSpace,
            iterator_: OldGenIteratorVariants::PageIterator(heap.old_space().begin()),
        }
    }

    pub fn next(&mut self) -> Option<&'a mut MutablePageMetadata> {
        use OldGenIteratorVariants::*;
        match self.state_ {
            OldGenState::OldSpace => {
                if let PageIterator(ref mut iterator) = self.iterator_ {
                    if let Some(page) = iterator.next() {
                        return Some(page);
                    }
                    self.state_ = OldGenState::CodeSpace;
                    self.iterator_ =
                        PageIterator(self.heap_.code_space().begin());
                    return self.next();
                } else {
                    unreachable!(); // Should not happen based on state transition
                }
            }
            OldGenState::CodeSpace => {
                if let PageIterator(ref mut iterator) = self.iterator_ {
                    if let Some(page) = iterator.next() {
                        return Some(page);
                    }
                    self.state_ = OldGenState::LargeObjectSpace;
                    self.iterator_ =
                        LargePageIterator(self.heap_.lo_space().begin());
                    return self.next();
                } else {
                    unreachable!();
                }
            }
            OldGenState::LargeObjectSpace => {
                if let LargePageIterator(ref mut iterator) = self.iterator_ {
                    if let Some(page) = iterator.next() {
                        return Some(page);
                    }
                    self.state_ = OldGenState::CodeLargeObjectSpace;
                    self.iterator_ = LargePageIterator(self.heap_.code_lo_space().begin());
                    return self.next();
                } else {
                    unreachable!();
                }
            }
            OldGenState::CodeLargeObjectSpace => {
                if let LargePageIterator(ref mut iterator) = self.iterator_ {
                    if let Some(page) = iterator.next() {
                        return Some(page);
                    }
                    self.state_ = OldGenState::TrustedSpace;
                    self.iterator_ =
                        PageIterator(self.heap_.trusted_space().begin());
                    return self.next();
                } else {
                    unreachable!();
                }
            }
            OldGenState::TrustedSpace => {
                if let PageIterator(ref mut iterator) = self.iterator_ {
                    if let Some(page) = iterator.next() {
                        return Some(page);
                    }
                    self.state_ = OldGenState::TrustedLargeObjectSpace;
                    self.iterator_ =
                        LargePageIterator(self.heap_.trusted_lo_space().begin());
                    return self.next();
                } else {
                    unreachable!();
                }
            }
            OldGenState::TrustedLargeObjectSpace => {
                if let LargePageIterator(ref mut iterator) = self.iterator_ {
                    if let Some(page) = iterator.next() {
                        return Some(page);
                    }
                    self.state_ = OldGenState::Finished;
                    return None;
                } else {
                    unreachable!();
                }
            }
            OldGenState::Finished => None,
        }
    }
}

pub struct MemoryChunkIterator<'a> {
    space_iterator_: SpaceIterator<'a>,
    current_chunk_: *mut MutablePageMetadata,
}

impl<'a> MemoryChunkIterator<'a> {
    pub fn new(heap: &'a Heap) -> Self {
        MemoryChunkIterator {
            space_iterator_: SpaceIterator::new(heap),
            current_chunk_: std::ptr::null_mut(),
        }
    }

    pub fn has_next(&self) -> bool {
        if !self.current_chunk_.is_null() {
            return true;
        }

        let mut space_iterator = SpaceIterator {
            current_space_index: self.space_iterator_.current_space_index,
            spaces: self.space_iterator_.spaces,
        };

        while space_iterator.has_next() {
            let space = space_iterator.next();
            let first_page = space.first_page();
            if !first_page.is_null() {
                return true;
            }
        }

        return false;
    }

    pub fn next(&mut self) -> Option<&'a mut MutablePageMetadata> {
        if self.current_chunk_.is_null() && !self.has_next() {
            return None;
        }

        let chunk = self.current_chunk_;
        if !chunk.is_null() {
            self.current_chunk_ = unsafe { (*chunk).list_node().next() };
            return Some(unsafe { &mut *chunk });
        } else {
            // This should not happen because of the has_next check, but it's good to have
            return None;
        }
    }
}

struct SpaceIterator<'a> {
    current_space_index: usize,
    spaces: &'a [*mut Space],
}

impl<'a> SpaceIterator<'a> {
    fn new(heap: &'a Heap) -> Self {
        SpaceIterator {
            current_space_index: 0,
            spaces: &[
                heap.old_space(),
                heap.code_space(),
                heap.map_space(),
                heap.large_object_space(),
                heap.code_large_object_space(),
                heap.trusted_space(),
                heap.trusted_large_object_space(),
            ],
        }
    }

    fn has_next(&self) -> bool {
        self.current_space_index < self.spaces.len()
    }

    fn next(&mut self) -> &'a mut Space {
        let space = unsafe { &mut *self.spaces[self.current_space_index] };
        self.current_space_index += 1;
        space
    }
}

trait ListNodeTrait {
    fn next(&self) -> *mut MutablePageMetadata;
}