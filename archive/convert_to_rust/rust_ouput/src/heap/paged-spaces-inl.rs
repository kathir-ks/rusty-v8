// Converted from V8 C++ source files:
// Header: paged-spaces-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
use std::ops::{Deref, DerefMut};

use crate::v8::internal::PagedSpaceBase;
use crate::v8::internal::Heap;
use crate::v8::internal::HeapObject;
use crate::v8::internal::Object;
use crate::v8::internal::Tagged;
use crate::v8::internal::WritableFreeSpace;

const kNullAddress: usize = 0;

mod page_metadata {
    use super::*;
    pub struct PageMetadata {}

    impl PageMetadata {
        pub fn FromAddress(addr: usize) -> &'static PageMetadata {
            todo!()
        }
        pub fn FromHeapObject(obj: &HeapObject) -> &'static PageMetadata {
            todo!()
        }
        pub fn owner(&self) -> *const PagedSpaceBase {
            todo!()
        }
        pub fn heap(&self) -> &'static Heap {
            todo!()
        }
        pub fn area_start(&self) -> usize {
            todo!()
        }
        pub fn area_end(&self) -> usize {
            todo!()
        }
        pub fn owner_identity(&self) -> i32 {
            todo!()
        }
    }
}

mod writable_jit_page {
    use super::*;
    pub struct WritableJitPage {
        start: usize,
        size_in_bytes: usize,
    }
    impl WritableJitPage {
        pub fn new(start: usize, size_in_bytes: usize) -> Self {
            WritableJitPage {
                start,
                size_in_bytes,
            }
        }
        pub fn FreeRange(&self, start: usize, size_in_bytes: usize) -> WritableFreeSpace {
            WritableFreeSpace {}
        }
    }
}

mod free_list {
    use super::*;
    pub struct FreeList {}
    impl FreeList {
        pub fn Free(
            &mut self,
            free_space: WritableFreeSpace,
            link_category: i32,
        ) -> usize {
            0
        }
        pub fn increase_wasted_bytes(&mut self, wasted: usize) {}
    }
}

mod accounting_stats {
    use super::*;
    use crate::page_metadata::PageMetadata;
    pub struct AccountingStats {}
    impl AccountingStats {
        pub fn DecreaseAllocatedBytes(&mut self, size_in_bytes: usize, page: &PageMetadata) {}
    }
}

pub mod internal {
    use super::*;
    use crate::page_metadata::PageMetadata;
    use crate::writable_jit_page::WritableJitPage;
    use std::marker::PhantomData;

    const CODE_SPACE: i32 = 1;

    pub struct HeapObjectRange {
        page_: *const PageMetadata,
    }

    impl HeapObjectRange {
        pub fn new(page: *const PageMetadata) -> Self {
            HeapObjectRange { page_: page }
        }

        pub fn begin(&self) -> iterator {
            iterator::new(self.page_)
        }

        pub fn end(&self) -> iterator {
            iterator::new(std::ptr::null())
        }

        pub struct iterator {
            cage_base_: usize,
            cur_addr_: usize,
            cur_end_: usize,
            cur_size_: usize,
        }

        impl iterator {
            fn new(page: *const PageMetadata) -> Self {
                if page.is_null() {
                    iterator {
                        cage_base_: kNullAddress,
                        cur_addr_: kNullAddress,
                        cur_end_: kNullAddress,
                        cur_size_: 0,
                    }
                } else {
                    let page_ref = unsafe { &*page };
                    iterator {
                        cage_base_: page_ref.heap().isolate(),
                        cur_addr_: page_ref.area_start(),
                        cur_end_: page_ref.area_end(),
                        cur_size_: 0,
                    }
                }
            }

            pub fn advance_to_next_object(&mut self) {
                if self.cur_addr_ == kNullAddress {
                    return;
                }

                while self.cur_addr_ != self.cur_end_ {
                    if self.cur_addr_ > self.cur_end_ {
                        panic!("cur_addr_ > cur_end_");
                    }

                    let obj = HeapObject::FromAddress(self.cur_addr_);
                    self.cur_size_ =
                        align_to_allocation_alignment(obj.Size(self.cage_base_));

                    if self.cur_addr_ + self.cur_size_ > self.cur_end_ {
                        panic!("cur_addr_ + cur_size_ > cur_end_");
                    }

                    if is_free_space_or_filler(&obj, self.cage_base_) {
                        self.cur_addr_ += self.cur_size_;
                    } else {
                        if is_instruction_stream(&obj, self.cage_base_) {
                            assert_eq!(
                                PageMetadata::FromHeapObject(&obj).owner_identity(),
                                CODE_SPACE
                            );
                            assert_codeobject_size(self.cur_size_);
                        } else {
                            assert_object_size(self.cur_size_);
                        }
                        return;
                    }
                }

                self.cur_addr_ = kNullAddress;
            }
        }

        impl Iterator for iterator {
            type Item = HeapObject;

            fn next(&mut self) -> Option<Self::Item> {
                if self.cage_base_ == kNullAddress {
                    return None;
                }
                if self.cur_addr_ == kNullAddress {
                    return None;
                }
                let current_object = HeapObject::FromAddress(self.cur_addr_);
                self.cur_addr_ += self.cur_size_;
                self.advance_to_next_object();

                Some(current_object)
            }
        }
    }

    fn align_to_allocation_alignment(size: usize) -> usize {
        (size + 8 - 1) & !(8 - 1)
    }

    fn is_free_space_or_filler(obj: &HeapObject, cage_base: usize) -> bool {
        false
    }

    fn is_instruction_stream(obj: &HeapObject, cage_base: usize) -> bool {
        false
    }

    fn assert_codeobject_size(size: usize) {}

    fn assert_object_size(size: usize) {}

    pub struct PagedSpaceObjectIterator {
        current_page_index: usize,
        pages: Vec<*mut PageMetadata>,
        cur_: HeapObjectRange::iterator,
        end_: HeapObjectRange::iterator,
    }

    impl PagedSpaceObjectIterator {
        pub fn Next(&mut self) -> Tagged<HeapObject> {
            loop {
                if self.cur_.cage_base_ != kNullAddress {
                    let next_object = self.cur_.next();
                    match next_object {
                        Some(obj) => {
                            return Tagged::from(obj);
                        }
                        None => {}
                    }
                }
                if !self.AdvanceToNextPage() {
                    return Tagged::<HeapObject>::default();
                }
            }
        }
        fn AdvanceToNextPage(&mut self) -> bool {
            self.current_page_index += 1;
            if self.current_page_index < self.pages.len() {
                let next_page = self.pages[self.current_page_index];
                let range = HeapObjectRange::new(next_page);
                self.cur_ = range.begin();
                self.end_ = range.end();
                true
            } else {
                false
            }
        }
    }

    pub struct PagedSpaceBase {
        executable_: bool,
        free_list_: Box<free_list::FreeList>,
        heap_: *mut Heap,
        accounting_stats_: accounting_stats::AccountingStats,
    }

    impl PagedSpaceBase {
        pub fn new(executable: bool, heap: *mut Heap) -> Self {
            PagedSpaceBase {
                executable_: executable,
                free_list_: Box::new(free_list::FreeList {}),
                heap_: heap,
                accounting_stats_: accounting_stats::AccountingStats {},
            }
        }

        pub fn Contains(&self, addr: usize) -> bool {
            unsafe { PageMetadata::FromAddress(addr).owner() == self }
        }

        pub fn Contains_tagged(&self, o: Tagged<Object>) -> bool {
            if !o.is_heap_object() {
                return false;
            }
            unsafe { PageMetadata::FromAddress(o.ptr() as usize).owner() == self }
        }

        fn FreeInternal<const DURING_SWEEP: bool>(
            &mut self,
            start: usize,
            size_in_bytes: usize,
        ) -> usize {
            if size_in_bytes == 0 {
                return 0;
            }

            let mut wasted: usize;
            if self.executable_ {
                let jit_page = WritableJitPage::new(start, size_in_bytes);
                let free_space = jit_page.FreeRange(start, size_in_bytes);
                self.heap()
                    .CreateFillerObjectAtBackground(free_space);
                wasted = self.free_list_.Free(
                    free_space,
                    if DURING_SWEEP {
                        kDoNotLinkCategory
                    } else {
                        kLinkCategory
                    },
                );
            } else {
                let free_space = WritableFreeSpace::ForNonExecutableMemory(start, size_in_bytes);
                self.heap()
                    .CreateFillerObjectAtBackground(free_space);
                wasted = self.free_list_.Free(
                    free_space,
                    if DURING_SWEEP {
                        kDoNotLinkCategory
                    } else {
                        kLinkCategory
                    },
                );
            }
            if !DURING_SWEEP {
                let page = PageMetadata::FromAddress(start);
                self.accounting_stats_
                    .DecreaseAllocatedBytes(size_in_bytes, page);
                self.free_list().increase_wasted_bytes(wasted);
            }

            if size_in_bytes < wasted {
                panic!("size_in_bytes < wasted");
            }
            size_in_bytes - wasted
        }
        pub fn Free(&mut self, start: Address, size_in_bytes: usize) -> usize {
            self.FreeInternal::<false>(start, size_in_bytes)
        }
        pub fn FreeDuringSweep(&mut self, start: Address, size_in_bytes: usize) -> usize {
            self.FreeInternal::<true>(start, size_in_bytes)
        }
        fn heap(&mut self) -> &mut Heap {
            unsafe { &mut *(self.heap_) }
        }
        fn free_list(&mut self) -> &mut free_list::FreeList {
            &mut self.free_list_
        }
    }

    const kDoNotLinkCategory: i32 = 0;
    const kLinkCategory: i32 = 1;

    pub type Address = usize;
}

pub trait HeapObjectTrait {
    fn Size(&self, cage_base: usize) -> usize;
    fn FromAddress(address: usize) -> HeapObject;
}

impl HeapObjectTrait for HeapObject {
    fn Size(&self, cage_base: usize) -> usize {
        16
    }
    fn FromAddress(address: usize) -> HeapObject {
        HeapObject {}
    }
}

pub trait ObjectTrait {
    fn ptr(&self) -> *mut u8;
    fn is_heap_object(&self) -> bool;
}

impl ObjectTrait for Object {
    fn ptr(&self) -> *mut u8 {
        std::ptr::null_mut()
    }
    fn is_heap_object(&self) -> bool {
        false
    }
}

impl Tagged<Object> {
    fn is_heap_object(&self) -> bool {
        false
    }
    fn ptr(&self) -> *mut u8 {
        std::ptr::null_mut()
    }
}
impl Tagged<HeapObject> {
    fn default() -> Self {
        Tagged { _phantom: PhantomData }
    }
}
