// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// include/cppgc/explicit-management.h
pub mod explicit_management {
    use crate::heap::cppgc::heap_handle::HeapHandle;

    /// Frees an object allocated by the garbage collector explicitly.
    ///
    /// This API allows embedders to free garbage collected objects at will. This
    /// can be useful to integrate with existing APIs. It is the embedder's
    /// responsibility to ensure that no other garbage collected objects still
    /// reference this object. The garbage collector has no means of validating
    /// this.
    pub fn free_unreferenced_object(heap_handle: &mut HeapHandle, object: *mut std::ffi::c_void);

    /// Resizes an object allocated by the garbage collector explicitly.
    ///
    /// Similar to `free_unreferenced_object`, this gives embedders control over
    /// memory. This API is only supported for regular pages and will return
    /// `false` for large objects.
    pub fn resize_object(object: *mut std::ffi::c_void, new_object_size: usize) -> bool;
}

mod internal {
    use crate::heap::cppgc::base::BasePage;
    use crate::heap::cppgc::constants::kAllocationGranularity;
    use crate::heap::cppgc::free_list::FreeListEntry;
    use crate::heap::cppgc::heap_base::HeapBase;
    use crate::heap::cppgc::heap_handle::HeapHandle;
    use crate::heap::cppgc::heap_object_header::HeapObjectHeader;
    use crate::heap::cppgc::large_page::LargePage;
    use crate::heap::cppgc::linear_allocation_buffer::LinearAllocationBuffer;
    use crate::heap::cppgc::memory::{SetMemoryAccessible, SetMemoryInaccessible};
    use crate::heap::cppgc::normal_page::NormalPage;
    use crate::heap::cppgc::normal_page_space::NormalPageSpace;
    use crate::heap::cppgc::object_allocator::ObjectAllocator;
    use crate::heap::cppgc::object_view::ObjectView;
    use std::cmp::max;

    // Helper function to round up to the allocation granularity.
    fn round_up(size: usize) -> usize {
        ((size + kAllocationGranularity - 1) / kAllocationGranularity) * kAllocationGranularity
    }

    fn in_gc(heap_handle: &mut HeapHandle) -> bool {
        let heap = HeapBase::from(heap_handle);
        heap.in_atomic_pause() || heap.marker().is_some() || heap.sweeper().is_sweeping_in_progress()
    }

    pub struct ExplicitManagementImpl {}

    impl ExplicitManagementImpl {
        pub fn free_unreferenced_object(heap_handle: &mut HeapHandle, object: *mut std::ffi::c_void) {
            if in_gc(heap_handle) {
                return;
            }

            let header = unsafe { HeapObjectHeader::from_object(object) };
            header.finalize();

            let base_page = unsafe { BasePage::from_payload(object) };

            #[cfg(feature = "cppgc_young_generation")]
            {
                let object_size = ObjectView::new(header).size();

                if let Some(heap_base) = HeapBase::from(heap_handle).generational_gc_supported() {
                    heap_base.remembered_set().invalidate_remembered_slots_in_range(
                        object,
                        unsafe { (object as *mut u8).add(object_size) },
                    );
                    heap_base
                        .remembered_set()
                        .invalidate_remembered_source_object(header);
                    if header.is_marked() {
                        base_page.decrement_marked_bytes(if base_page.is_large() {
                            unsafe {
                                LargePage::from(base_page)
                                    .payload_size()
                            }
                        } else {
                            header.allocated_size()
                        });
                    }
                }
            }

            if base_page.is_large() {
                let large_page = unsafe { LargePage::from(base_page) };
                base_page.space().remove_page(base_page);
                base_page
                    .heap()
                    .stats_collector()
                    .notify_explicit_free(large_page.payload_size());
                unsafe { LargePage::destroy(large_page) };
            } else {
                let header_size = header.allocated_size();
                let normal_page = unsafe { NormalPage::from(base_page) };
                let normal_space = unsafe { &mut *(base_page.space() as *mut dyn crate::heap::cppgc::space::Space as *mut NormalPageSpace) };
                let lab = normal_space.linear_allocation_buffer();
                let payload_end = header.object_end();

                unsafe { SetMemoryInaccessible(header as *mut HeapObjectHeader as *mut std::ffi::c_void, header_size) };

                if payload_end == lab.start() {
                    lab.set(header as *mut HeapObjectHeader as *mut std::ffi::c_void, lab.size() + header_size);
                    normal_page.object_start_bitmap().clear_bit(lab.start());
                } else {
                    base_page
                        .heap()
                        .stats_collector()
                        .notify_explicit_free(header_size);
                    normal_space.free_list().add(FreeListEntry {
                        start: header as *mut HeapObjectHeader as *mut std::ffi::c_void,
                        size: header_size,
                    });
                    // No need to update the bitmap as the same bit is reused for the free
                    // list entry.
                }
            }
        }

        pub fn resize(object: *mut std::ffi::c_void, new_object_size: usize) -> bool {
            let base_page = unsafe { BasePage::from_payload(object) };

            if in_gc(base_page.heap()) {
                return false;
            }

            if base_page.is_large() {
                return false;
            }

            let new_size = round_up(std::mem::size_of::<HeapObjectHeader>() + new_object_size);
            let header = unsafe { HeapObjectHeader::from_object(object) };
            let old_size = header.allocated_size();

            if new_size > old_size {
                Self::grow(header, base_page, new_size, new_size - old_size)
            } else if old_size > new_size {
                Self::shrink(header, base_page, new_size, old_size - new_size)
            } else {
                true
            }
        }

        fn grow(mut header: HeapObjectHeader, base_page: &mut BasePage, new_size: usize, size_delta: usize) -> bool {
            if new_size < header.allocated_size() + kAllocationGranularity {
                panic!("new_size must be greater than or equal to header.allocated_size() + kAllocationGranularity");
            }
            if size_delta < kAllocationGranularity {
                panic!("size_delta must be greater than or equal to kAllocationGranularity");
            }
            if base_page.is_large() {
                panic!("base_page must not be large");
            }

            let normal_space = unsafe { &mut *(base_page.space() as *mut dyn crate::heap::cppgc::space::Space as *mut NormalPageSpace) };
            let lab = normal_space.linear_allocation_buffer();

            if header.object_end() == lab.start() && lab.size() >= size_delta {
                let delta_start = lab.allocate(size_delta);
                unsafe { SetMemoryAccessible(delta_start, size_delta) };
                header.set_allocated_size(new_size);

                #[cfg(feature = "cppgc_young_generation")]
                {
                    if let Some(heap_base) = normal_space.raw_heap().heap().generational_gc_supported() {
                        if header.is_marked() {
                            base_page.increment_marked_bytes(header.allocated_size());
                        }
                    }
                }
                true
            } else {
                false
            }
        }

        fn shrink(
            mut header: HeapObjectHeader,
            base_page: &mut BasePage,
            new_size: usize,
            size_delta: usize,
        ) -> bool {
            if header.allocated_size() < new_size + kAllocationGranularity {
                panic!("header.allocated_size() must be greater than or equal to new_size + kAllocationGranularity");
            }
            if size_delta < kAllocationGranularity {
                panic!("size_delta must be greater than or equal to kAllocationGranularity");
            }
            if base_page.is_large() {
                panic!("base_page must not be large");
            }

            let normal_space = unsafe { &mut *(base_page.space() as *mut dyn crate::heap::cppgc::space::Space as *mut NormalPageSpace) };
            let lab = normal_space.linear_allocation_buffer();
            let free_start = header.object_end() - size_delta;

            if header.object_end() == lab.start() {
                if free_start != lab.start() - size_delta {
                    panic!("free_start must be equal to lab.start() - size_delta");
                }
                lab.set(free_start, lab.size() + size_delta);
                unsafe { SetMemoryInaccessible(lab.start(), size_delta) };
                header.set_allocated_size(new_size);
            } else if size_delta >= ObjectAllocator::kSmallestSpaceSize {
                unsafe { SetMemoryInaccessible(free_start, size_delta) };
                base_page.heap().stats_collector().notify_explicit_free(size_delta);
                normal_space.free_list().add(FreeListEntry {
                    start: free_start,
                    size: size_delta,
                });
                unsafe { NormalPage::from(base_page).object_start_bitmap().set_bit(free_start) };
                header.set_allocated_size(new_size);
            }

            #[cfg(feature = "cppgc_young_generation")]
            {
                let heap = base_page.heap();
                if let Some(heap_base) = heap.generational_gc_supported() {
                    heap_base.remembered_set().invalidate_remembered_slots_in_range(
                        free_start,
                        free_start + size_delta,
                    );
                    if header.is_marked() {
                        base_page.decrement_marked_bytes(header.allocated_size());
                    }
                }
            }
            true
        }
    }
}

// Implementation of the public API.
impl explicit_management::explicit_management {
    use crate::heap::cppgc::internal::ExplicitManagementImpl;
    use std::ffi::c_void;
    use crate::heap::cppgc::heap_handle::HeapHandle;

    pub fn free_unreferenced_object(heap_handle: &mut HeapHandle, object: *mut c_void) {
        ExplicitManagementImpl::free_unreferenced_object(heap_handle, object);
    }

    pub fn resize_object(object: *mut c_void, new_object_size: usize) -> bool {
        ExplicitManagementImpl::resize(object, new_object_size)
    }
}