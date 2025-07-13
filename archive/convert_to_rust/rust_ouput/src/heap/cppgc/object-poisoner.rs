// Converted from V8 C++ source files:
// Header: object-poisoner.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod sanitizer {
        pub mod asan {
            pub fn ASAN_POISON_MEMORY_REGION(_ptr: *mut u8, _size: usize) {}
        }
    }
}

pub mod heap {
    pub mod cppgc {
        pub mod object_poisoner {
            use super::{
                heap_object_header::HeapObjectHeader, heap_visitor::HeapVisitor, object_view::ObjectView,
            };

            #[cfg(feature = "v8_use_address_sanitizer")]
            pub struct UnmarkedObjectsPoisoner {}

            #[cfg(feature = "v8_use_address_sanitizer")]
            impl UnmarkedObjectsPoisoner {
                pub fn new() -> Self {
                    UnmarkedObjectsPoisoner {}
                }

                fn visit_heap_object_header(&mut self, header: &mut HeapObjectHeader) -> bool {
                    if header.is_free() || header.is_marked() {
                        return true;
                    }

                    unsafe {
                        let ptr = header.object_start() as *mut u8;
                        let size = ObjectView::from(header).size();
                        crate::base::sanitizer::asan::ASAN_POISON_MEMORY_REGION(ptr, size);
                    }
                    true
                }
            }

            #[cfg(feature = "v8_use_address_sanitizer")]
            impl HeapVisitor for UnmarkedObjectsPoisoner {
                 fn visit(&mut self, header: &mut HeapObjectHeader) -> bool{
                    self.visit_heap_object_header(header)
                 }
            }
        }

        pub mod heap_object_header {
            #[derive(Debug)]
            pub struct HeapObjectHeader {
                marked: bool,
                free: bool,
                object_start: usize,
                size: usize,
            }

            impl HeapObjectHeader {
                pub fn new(marked: bool, free: bool, object_start: usize, size: usize) -> Self {
                    HeapObjectHeader {
                        marked,
                        free,
                        object_start,
                        size,
                    }
                }

                pub fn is_marked(&self) -> bool {
                    self.marked
                }

                pub fn is_free(&self) -> bool {
                    self.free
                }

                pub fn object_start(&self) -> usize {
                    self.object_start
                }
                pub fn size(&self) -> usize {
                    self.size
                }

            }
        }

        pub mod heap_page {
            pub struct HeapPage {}
        }

        pub mod heap_visitor {
            use super::heap_object_header::HeapObjectHeader;
            pub trait HeapVisitor{
                fn visit(&mut self, header: &mut HeapObjectHeader) -> bool;
            }
        }

        pub mod object_view {
            use super::heap_object_header::HeapObjectHeader;

            pub struct ObjectView<'a> {
                header: &'a HeapObjectHeader,
            }

            impl<'a> ObjectView<'a> {
                pub fn from(header: &'a HeapObjectHeader) -> Self {
                    ObjectView { header }
                }

                pub fn size(&self) -> usize {
                    self.header.size()
                }
            }
        }
    }
}
