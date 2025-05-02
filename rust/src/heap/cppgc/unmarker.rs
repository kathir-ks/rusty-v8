// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod unmarker {
    use crate::heap::cppgc::{heap_object_header::HeapObjectHeader, heap_visitor::HeapVisitor, normal_page::NormalPage, large_page::LargePage};
    use crate::heap::cppgc::raw_heap::RawHeap;

    pub mod internal {

        use crate::heap::cppgc::unmarker::SequentialUnmarker;
        use crate::heap::cppgc::{heap_object_header::HeapObjectHeader, normal_page::NormalPage, large_page::LargePage};
        use crate::heap::cppgc::raw_heap::RawHeap;

        /// Unmarks all marked `HeapObjectHeader`s in a heap.
        pub struct SequentialUnmarker {}

        impl SequentialUnmarker {
            pub fn new(heap: &mut RawHeap) {
                SequentialUnmarker::traverse(heap);
            }

            fn visit_normal_page(page: &mut NormalPage) -> bool {
                page.reset_marked_bytes();
                false
            }

            fn visit_large_page(page: &mut LargePage) -> bool {
                page.reset_marked_bytes();
                false
            }
        }

        impl HeapVisitor for SequentialUnmarker {
            fn visit_heap_object_header(header: &mut HeapObjectHeader) -> bool {
                if header.is_marked() {
                    header.unmark();
                }
                true
            }

             fn visit_normal_page_hv(page: &mut NormalPage) -> bool {
                SequentialUnmarker::visit_normal_page(page)
            }

            fn visit_large_page_hv(page: &mut LargePage) -> bool {
                SequentialUnmarker::visit_large_page(page)
            }

            fn traverse(heap: &mut RawHeap) {
                 for page in &mut heap.normal_pages {
                     let _ = SequentialUnmarker::visit_normal_page_hv(page);
                 }

                 for page in &mut heap.large_pages {
                     let _ = SequentialUnmarker::visit_large_page_hv(page);
                 }

                 for object in &mut heap.objects {
                     let _ = SequentialUnmarker::visit_heap_object_header(object);
                 }
            }
        }
    }
}