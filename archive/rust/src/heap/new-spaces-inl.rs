// src/heap/new_spaces_inl.rs

//use crate::base::sanitizer::msan; // Placeholder: msan crate not available
use crate::common::globals::*;
use crate::heap::heap::*;
use crate::heap::new_spaces::*;
use crate::heap::paged_spaces_inl::*;
use crate::heap::spaces_inl::*;
use crate::objects::objects_inl::*;
use crate::objects::tagged_impl::*;
use crate::objects::tagged::*;

/// Represents a semi-space in the heap.
impl SemiSpace {
    /// Checks if the semi-space contains the given heap object.
    pub fn contains(&self, o: Tagged<HeapObject>) -> bool {
        let memory_chunk = MemoryChunk::from_heap_object(o);
        if memory_chunk.is_large_page() {
            return false;
        }
        if self.id_ == kToSpace {
            memory_chunk.is_to_page()
        } else {
            memory_chunk.is_from_page()
        }
    }

    /// Checks if the semi-space contains the given object.
    pub fn contains_object(&self, o: Tagged<Object>) -> bool {
        o.is_heap_object() && self.contains(o.cast::<HeapObject>())
    }

    /// Checks if the semi-space contains the given object (generic version).
    pub fn contains_generic<T>(&self, o: Tagged<T>) -> bool {
        //static_assert(kTaggedCanConvertToRawObjects); // Assuming Tagged<T> can be converted to raw objects
        self.contains_object(*o)
    }

    /// Checks if the semi-space contains the given address (slow version).
    pub fn contains_slow(&self, a: Address) -> bool {
        for p in self.iter() {
            if p == MemoryChunkMetadata::from_address(a) {
                return true;
            }
        }
        false
    }
}

impl NewSpace {
    /// Checks if the new space contains the given object.
    pub fn contains_object(&self, o: Tagged<Object>) -> bool {
        o.is_heap_object() && self.contains(o.cast::<HeapObject>())
    }

    /// Checks if the new space contains the given heap object.
    pub fn contains(&self, o: Tagged<HeapObject>) -> bool {
        MemoryChunk::from_heap_object(o).in_new_space()
    }
}

/// Iterator for objects in a semi-space.
impl SemiSpaceObjectIterator {
    /// Creates a new semi-space object iterator.
    pub fn new(space: &SemiSpaceNewSpace) -> Self {
        SemiSpaceObjectIterator {
            current_: space.first_allocatable_address(),
        }
    }

    /// Returns the next heap object in the semi-space.
    pub fn next(&mut self) -> Option<Tagged<HeapObject>> {
        loop {
            if PageMetadata::is_aligned_to_page_size(self.current_) {
                let page = PageMetadata::from_allocation_area_address(self.current_);
                let page = page.next_page();
                match page {
                    None => return None,
                    Some(p) => self.current_ = p.area_start(),
                }
            }
            let object = HeapObject::from_address(self.current_);
            self.current_ += align_to_allocation_alignment(object.size());
            if !is_free_space_or_filler(object) {
                return Some(object);
            }
        }
    }
}

impl SemiSpaceNewSpace {
    /// Increments the allocation top.
    pub fn increment_allocation_top(&mut self, new_top: Address) {
        debug_assert!(self.allocation_top_ <= new_top);
        debug_assert_eq!(
            PageMetadata::from_allocation_area_address(self.allocation_top_),
            PageMetadata::from_allocation_area_address(new_top)
        );
        self.allocation_top_ = new_top;
    }

    /// Decrements the allocation top.
    pub fn decrement_allocation_top(&mut self, new_top: Address) {
        debug_assert!(new_top <= self.allocation_top_);
        debug_assert_eq!(
            PageMetadata::from_allocation_area_address(self.allocation_top_),
            PageMetadata::from_allocation_area_address(new_top)
        );
        self.allocation_top_ = new_top;
    }
}