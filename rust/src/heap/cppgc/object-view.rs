// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod internal {
    use std::marker::PhantomData;
    //use crate::heap::cppgc::globals::*; // Assuming globals.h functionality is elsewhere
    use crate::heap::cppgc::heap_object_header::HeapObjectHeader;
    use crate::heap::cppgc::heap_page::{BasePage, LargePage};

    /// Dummy enum for AccessMode.  Need to figure out actual usage later.
    #[derive(Debug, Copy, Clone)]
    pub enum AccessMode {
        kNonAtomic,
    }

    /// ObjectView allows accessing a header within the bounds of the actual object.
    /// It is not exposed externally and does not keep the underlying object alive.
    #[derive(Debug)]
    pub struct ObjectView<'a, A: 'static = AccessMode> {
        header_: &'a HeapObjectHeader,
        base_page_: *mut BasePage,
        is_large_object_: bool,
        _phantom: PhantomData<A>,
    }

    impl<'a, A: 'static> ObjectView<'a, A> {
        /// Constructor for ObjectView
        pub fn new(header: &'a HeapObjectHeader) -> Self {
            let base_page_ptr = BasePage::from_payload_ptr(header as *const HeapObjectHeader as *mut HeapObjectHeader);
            let is_large_object_ = header.is_large_object::<A>();

            let object_view = ObjectView {
                header_: header,
                base_page_: base_page_ptr,
                is_large_object_: is_large_object_,
                _phantom: PhantomData,
            };
            assert_eq!(object_view.start() + object_view.size(), object_view.end() as usize);

            object_view
        }

        /// Returns the start address of the object.
        pub fn start(&self) -> usize {
            self.header_.object_start()
        }

        /// Returns the end address of the object.
        pub fn end(&self) -> usize {
            if self.is_large_object_ {
                unsafe { LargePage::from(self.base_page_).payload_end() }
            } else {
                self.header_.object_end::<A>()
            }
        }

        /// Returns the size of the object.
        pub fn size(&self) -> usize {
            if self.is_large_object_ {
                unsafe { LargePage::from(self.base_page_).object_size() }
            } else {
                self.header_.object_size::<A>()
            }
        }
    }
} // namespace internal