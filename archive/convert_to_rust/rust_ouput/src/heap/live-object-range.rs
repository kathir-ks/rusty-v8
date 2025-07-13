// Converted from V8 C++ source files:
// Header: live-object-range.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod live_object_range {
    use std::marker::PhantomData;

    use crate::heap::marking::MarkBit;
    use crate::heap::new_spaces::PtrComprCageBase;
    use crate::heap::PageMetadata;
    use crate::objects::heap_object::HeapObject;
    use crate::objects::map::Map;

    pub struct LiveObjectRange {
        page_: *const PageMetadata,
    }

    impl LiveObjectRange {
        pub fn new(page: *const PageMetadata) -> Self {
            LiveObjectRange { page_: page }
        }

        pub fn begin(&self) -> iterator {
            iterator::new(self.page_)
        }

        pub fn end(&self) -> iterator {
            iterator::default()
        }
    }

    pub struct iterator {
        page_: *const PageMetadata,
        cells_: *const MarkBit::CellType,
        cage_base_: PtrComprCageBase,
        current_cell_index_: usize,
        current_cell_: MarkBit::CellType,
        current_object_: *mut HeapObject,
        current_map_: *mut Map,
        current_size_: usize,
        phantom: PhantomData<*const PageMetadata>,
    }

    impl iterator {
        pub fn new(page: *const PageMetadata) -> Self {
            let mut it = iterator {
                page_: page,
                cells_: std::ptr::null(),
                cage_base_: PtrComprCageBase {},
                current_cell_index_: 0,
                current_cell_: 0,
                current_object_: std::ptr::null_mut(),
                current_map_: std::ptr::null_mut(),
                current_size_: 0,
                phantom: PhantomData,
            };
            //TODO implement AdvanceToNextMarkedObject and AdvanceToNextValidObject
            //it.AdvanceToNextMarkedObject();
            //it.AdvanceToNextValidObject();
            it
        }

        fn advance_to_next_marked_object(&mut self) -> bool {
            //TODO implement
            false
        }

        fn advance_to_next_valid_object(&mut self) {
            //TODO implement
        }
    }

    impl Default for iterator {
        fn default() -> Self {
            iterator {
                page_: std::ptr::null(),
                cells_: std::ptr::null(),
                cage_base_: PtrComprCageBase {},
                current_cell_index_: 0,
                current_cell_: 0,
                current_object_: std::ptr::null_mut(),
                current_map_: std::ptr::null_mut(),
                current_size_: 0,
                phantom: PhantomData,
            }
        }
    }

    impl iterator {
        pub fn increment(&mut self) -> &mut Self {
            //TODO implement AdvanceToNextMarkedObject and AdvanceToNextValidObject
            //self.AdvanceToNextMarkedObject();
            //self.AdvanceToNextValidObject();
            self
        }
    }

    impl PartialEq for iterator {
        fn eq(&self, other: &Self) -> bool {
            self.current_object_ == other.current_object_
        }
    }

    impl Eq for iterator {}

    impl std::ops::Deref for iterator {
        type Target = (
            *mut HeapObject,
            usize,
        );

        fn deref(&self) -> &Self::Target {
            unsafe {
                & (
                    self.current_object_,
                    self.current_size_,
                )
            }
        }
    }

    impl Iterator for iterator {
        type Item = (*mut HeapObject, usize);

        fn next(&mut self) -> Option<Self::Item> {
            if self.current_object_.is_null() {
                return None;
            }

            let result = unsafe { (self.current_object_, self.current_size_) };
            self.increment();

            Some(result)
        }
    }
}
