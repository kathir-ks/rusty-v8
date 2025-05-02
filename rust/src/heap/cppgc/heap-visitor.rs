// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod cppgc {
    pub mod internal {

        /// Visitor for heap, which also implements the accept (traverse) interface.
        /// Implements preorder traversal of the heap. The order of traversal is defined.
        /// Implemented as a CRTP visitor to avoid virtual calls and support better
        /// inlining.
        pub trait HeapVisitor {
            fn visit_heap(&mut self, _heap: &RawHeap) -> bool {
                false
            }
            fn visit_normal_page_space(&mut self, _space: &NormalPageSpace) -> bool {
                false
            }
            fn visit_large_page_space(&mut self, _space: &LargePageSpace) -> bool {
                false
            }
            fn visit_normal_page(&mut self, _page: &NormalPage) -> bool {
                false
            }
            fn visit_large_page(&mut self, _page: &LargePage) -> bool {
                false
            }
            fn visit_heap_object_header(&mut self, _header: &HeapObjectHeader) -> bool {
                false
            }

            fn traverse(&mut self, heap: &mut RawHeap) {
                if self.visit_heap(heap) {
                    return;
                }
                for space in heap.spaces.iter_mut() {
                    self.traverse_space(space);
                }
            }

            fn traverse_space(&mut self, space: &mut BaseSpace) {
                let is_stopped = match space {
                    BaseSpace::Large(large_space) => self.visit_large_page_space(large_space),
                    BaseSpace::Normal(normal_space) => self.visit_normal_page_space(normal_space),
                };

                if is_stopped {
                    return;
                }
                for page in space.pages_mut().iter_mut() {
                    self.traverse_page(page);
                }
            }

            fn traverse_page(&mut self, page: &mut BasePage) {
                match page {
                    BasePage::Large(large_page) => {
                        if self.visit_large_page(large_page) {
                            return;
                        }
                        self.visit_heap_object_header(&large_page.object_header);
                    }
                    BasePage::Normal(normal_page) => {
                        if self.visit_normal_page(normal_page) {
                            return;
                        }
                        for header in normal_page.headers.iter() {
                            self.visit_heap_object_header(header);
                        }
                    }
                }
            }
        }

        // Dummy types representing the original C++ types.
        #[derive(Debug)]
        pub struct RawHeap {
            spaces: Vec<BaseSpace>,
        }

        impl RawHeap {
            pub fn new() -> Self {
                RawHeap { spaces: Vec::new() }
            }
        }

        #[derive(Debug)]
        pub enum BaseSpace {
            Normal(NormalPageSpace),
            Large(LargePageSpace),
        }

        impl BaseSpace {
            fn pages_mut(&mut self) -> &mut Vec<BasePage> {
                match self {
                    BaseSpace::Normal(space) => &mut space.pages,
                    BaseSpace::Large(space) => &mut space.pages,
                }
            }

        }
        #[derive(Debug)]
        pub struct NormalPageSpace {
            pages: Vec<BasePage>,
        }

        impl NormalPageSpace {
            pub fn new() -> Self {
                NormalPageSpace { pages: Vec::new() }
            }
        }

        #[derive(Debug)]
        pub struct LargePageSpace {
            pages: Vec<BasePage>,
        }
        impl LargePageSpace {
             pub fn new() -> Self {
                LargePageSpace { pages: Vec::new() }
            }
        }

        #[derive(Debug)]
        pub enum BasePage {
            Normal(NormalPage),
            Large(LargePage),
        }
        #[derive(Debug)]
        pub struct NormalPage {
            headers: Vec<HeapObjectHeader>,
        }

        impl NormalPage {
            pub fn new() -> Self {
                NormalPage { headers: Vec::new() }
            }
        }

        #[derive(Debug)]
        pub struct LargePage {
            object_header: HeapObjectHeader,
        }
        impl LargePage {
             pub fn new() -> Self {
                LargePage { object_header: HeapObjectHeader::new() }
            }
        }

        #[derive(Debug)]
        pub struct HeapObjectHeader { }

        impl HeapObjectHeader {
            pub fn new() -> Self {
                HeapObjectHeader { }
            }
        }
    }
}