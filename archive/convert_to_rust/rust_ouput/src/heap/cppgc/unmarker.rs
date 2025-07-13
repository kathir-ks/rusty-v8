// Converted from V8 C++ source files:
// Header: unmarker.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod cppgc {
  pub mod internal {
    use crate::cppgc::heap_object_header::HeapObjectHeader;
    use crate::cppgc::heap_visitor::HeapVisitor;
    use crate::cppgc::page::{LargePage, NormalPage};
    use crate::cppgc::raw_heap::RawHeap;

    pub struct SequentialUnmarker {
      heap: *mut RawHeap,
    }

    impl SequentialUnmarker {
      pub fn new(heap: &mut RawHeap) -> Self {
        let mut unmarker = SequentialUnmarker { heap };
        unmarker.traverse(heap);
        unmarker
      }

      fn traverse(&mut RawHeap) {
          todo!()
      }

      pub fn visit_normal_page(&mut self, page: &mut NormalPage) -> bool {
        page.reset_marked_bytes();
        false
      }

      pub fn visit_large_page(&mut self, page: &mut LargePage) -> bool {
        page.reset_marked_bytes();
        false
      }

      fn visit_heap_object_header(&mut self, header: &mut HeapObjectHeader) -> bool {
        if header.is_marked() {
          header.unmark();
        }
        true
      }
    }
  }
}

pub mod cppgc {
    pub mod heap_object_header {
        #[derive(Debug)]
        pub struct HeapObjectHeader {
            marked: bool,
        }

        impl HeapObjectHeader {
            pub fn new() -> Self {
                HeapObjectHeader { marked: false }
            }

            pub fn is_marked(&self) -> bool {
                self.marked
            }

            pub fn mark(&mut self) {
                self.marked = true;
            }

            pub fn unmark(&mut self) {
                self.marked = false;
            }
        }
    }

    pub mod heap_visitor {
        pub trait HeapVisitor<T> {
           // fn traverse(&mut self, heap: &mut RawHeap);
        }
    }

    pub mod page {
        pub struct NormalPage {}
        impl NormalPage{
            pub fn reset_marked_bytes(&mut self) {}
        }
        pub struct LargePage {}
        impl LargePage{
            pub fn reset_marked_bytes(&mut self) {}
        }
    }

    pub mod raw_heap {
        pub struct RawHeap{}
    }
}
