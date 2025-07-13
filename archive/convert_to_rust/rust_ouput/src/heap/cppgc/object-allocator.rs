// Converted from V8 C++ source files:
// Header: object-allocator.h
// Implementation: object-allocator.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod cppgc {
pub mod internal {
use std::alloc::Layout;
use std::mem;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::cppgc::EmbedderStackState;
use crate::cppgc::Heap;
use crate::cppgc::internal::GarbageCollector;
use crate::cppgc::internal::RawHeap;
use crate::cppgc::internal::StatsCollector;
use crate::cppgc::internal::Sweeper;
use crate::Address;
use crate::JSPluralRules;
use crate::MutablePageMetadata;
use crate::ReadOnlyPageMetadata;
use crate::TurboshaftGraph;
use crate::api_constants;
use crate::v8::base::TimeDelta;
pub struct ObjectAllocator {}
pub struct PreFinalizerHandler {}
}  // namespace internal

pub struct AllocationHandle {}

pub mod internal {

pub struct StatsCollector {}
pub struct PageBackend {}
pub struct GarbageCollector {}

#[derive(Debug)]
pub struct ObjectAllocator {
    raw_heap_: *mut RawHeap,
    page_backend_: *mut PageBackend,
    stats_collector_: *mut StatsCollector,
    prefinalizer_handler_: *mut PreFinalizerHandler,
    oom_handler_: FatalOutOfMemoryHandler,
    garbage_collector_: *mut GarbageCollector,
    #[cfg(v8_enable_allocation_timeout)]
    allocation_timeout_: Option<i32>,
}

impl ObjectAllocator {
    pub const KSMALLEST_SPACE_SIZE: usize = 32;
    const K_ALLOCATION_GRANULARITY: usize = 8;
    const K_LARGE_OBJECT_SIZE_THRESHOLD: usize = 2048;
    const K_ALLOCATION_MASK: usize = Self::K_ALLOCATION_GRANULARITY - 1;
    pub fn new(
        raw_heap_: *mut RawHeap,
        page_backend_: *mut PageBackend,
        stats_collector_: *mut StatsCollector,
        prefinalizer_handler_: *mut PreFinalizerHandler,
        oom_handler_: FatalOutOfMemoryHandler,
        garbage_collector_: *mut GarbageCollector,
    ) -> Self {
        ObjectAllocator {
            raw_heap_: raw_heap_,
            page_backend_: page_backend_,
            stats_collector_: stats_collector_,
            prefinalizer_handler_: prefinalizer_handler_,
            oom_handler_: oom_handler_,
            garbage_collector_: garbage_collector_,
            #[cfg(v8_enable_allocation_timeout)]
            allocation_timeout_: None,
        }
    }
    #[inline]
    pub fn allocate_object(&mut self, size: usize, gcinfo: GCInfoIndex) -> *mut u8 {
        if self.in_disallow_gc_scope() {
            eprintln!("FATAL: Attempt to allocate object during GC forbidden scope.");
            std::process::abort(); // Replace with appropriate error handling
        }
        #[cfg(v8_enable_allocation_timeout)]
        self.trigger_gc_on_allocation_timeout_if_needed();
        let allocation_size = round_up::<Self::K_ALLOCATION_GRANULARITY>(
            size + mem::size_of::<HeapObjectHeader>(),
        );
        let type_ = Self::get_initial_space_index_for_size(allocation_size);
		unsafe {
			let space = NormalPageSpace::from((*self.raw_heap_).space(type_));
			self.allocate_object_on_space(space, allocation_size, gcinfo)
		}
    }

    #[inline]
    pub fn allocate_object_aligned(
        &mut self,
        size: usize,
        alignment: AlignVal,
        gcinfo: GCInfoIndex,
    ) -> *mut u8 {
        if self.in_disallow_gc_scope() {
            eprintln!("FATAL: Attempt to allocate object during GC forbidden scope.");
            std::process::abort();
        }
        #[cfg(v8_enable_allocation_timeout)]
        self.trigger_gc_on_allocation_timeout_if_needed();
        let allocation_size = round_up::<Self::K_ALLOCATION_GRANULARITY>(
            size + mem::size_of::<HeapObjectHeader>(),
        );
        let type_ = Self::get_initial_space_index_for_size(allocation_size);
		unsafe {
			let space = NormalPageSpace::from((*self.raw_heap_).space(type_));
			self.allocate_object_on_space_aligned(space, allocation_size, alignment, gcinfo)
		}
    }

    #[inline]
    pub fn allocate_object_custom_space(
        &mut self,
        size: usize,
        gcinfo: GCInfoIndex,
        space_index: CustomSpaceIndex,
    ) -> *mut u8 {
        if self.in_disallow_gc_scope() {
            eprintln!("FATAL: Attempt to allocate object during GC forbidden scope.");
            std::process::abort();
        }

        #[cfg(v8_enable_allocation_timeout)]
        self.trigger_gc_on_allocation_timeout_if_needed();

        let allocation_size = round_up::<Self::K_ALLOCATION_GRANULARITY>(
            size + mem::size_of::<HeapObjectHeader>(),
        );
		unsafe {
			let space = NormalPageSpace::from((*self.raw_heap_).custom_space(space_index));
			self.allocate_object_on_space(space, allocation_size, gcinfo)
		}
    }

    #[inline]
    pub fn allocate_object_aligned_custom_space(
        &mut self,
        size: usize,
        alignment: AlignVal,
        gcinfo: GCInfoIndex,
        space_index: CustomSpaceIndex,
    ) -> *mut u8 {
        if self.in_disallow_gc_scope() {
            eprintln!("FATAL: Attempt to allocate object during GC forbidden scope.");
            std::process::abort();
        }

        #[cfg(v8_enable_allocation_timeout)]
        self.trigger_gc_on_allocation_timeout_if_needed();

        let allocation_size = round_up::<Self::K_ALLOCATION_GRANULARITY>(
            size + mem::size_of::<HeapObjectHeader>(),
        );
		unsafe {
			let space = NormalPageSpace::from((*self.raw_heap_).custom_space(space_index));
			self.allocate_object_on_space_aligned(space, allocation_size, alignment, gcinfo)
		}
    }

    pub fn reset_linear_allocation_buffers(&mut self) {
        struct Resetter {
            stats_collector_: *mut StatsCollector,
        }

        impl Resetter {
            fn new(stats: *mut StatsCollector) -> Self {
                Resetter {
                    stats_collector_: stats,
                }
            }
        }

        trait HeapVisitorTrait {
            fn visit_large_page_space(&mut self, _: &mut LargePageSpace) -> bool;
            fn visit_normal_page_space(&mut self, space: &mut NormalPageSpace) -> bool;
        }

        impl HeapVisitorTrait for Resetter {
            fn visit_large_page_space(&mut self, _: &mut LargePageSpace) -> bool {
                true
            }

            fn visit_normal_page_space(&mut self, space: &mut NormalPageSpace) -> bool {
				unsafe {
					self.replace_linear_allocation_buffer(space,  nullptr, 0);
				}
                true
            }
        }

        struct VisitorWrapper<'a> {
            inner: &'a mut Resetter,
        }

        impl<'a> VisitorWrapper<'a> {
            fn new(inner: &'a mut Resetter) -> Self {
                VisitorWrapper { inner }
            }

            fn traverse(&mut self, raw_heap: *mut RawHeap) {
				unsafe {
					let mut i = 0;
					while i < (*raw_heap).spaces_.len() {
						let space = &mut (*raw_heap).spaces_[i];
						match space {
							HeapSpace::Normal(normal_space) => {
								self.inner.visit_normal_page_space(normal_space);
							}
							HeapSpace::Large(large_space) => {
								self.inner.visit_large_page_space(large_space);
							}
						}
						i += 1;
					}
				}
            }
        }
		unsafe {
			let mut reseter = Resetter::new(self.stats_collector_);
			let mut wrapper = VisitorWrapper::new(&mut reseter);
			wrapper.traverse(self.raw_heap_);
		}
    }

    pub fn mark_all_pages_as_young(&mut self) {
        struct YoungMarker {}

        impl YoungMarker {
            fn new() -> Self {
                YoungMarker {}
            }
        }

        trait HeapVisitorTrait {
            fn visit_normal_page(&mut self, page: &mut NormalPage) -> bool;
            fn visit_large_page(&mut self, page: &mut LargePage) -> bool;
        }

        impl HeapVisitorTrait for YoungMarker {
            fn visit_normal_page(&mut self, page: &mut NormalPage) -> bool {
				unsafe {
					self.mark_range_as_young(page, page.payload_start(), page.payload_end());
				}
                true
            }

            fn visit_large_page(&mut self, page: &mut LargePage) -> bool {
				unsafe {
					self.mark_range_as_young(page, page.payload_start(), page.payload_end());
				}
                true
            }
        }

        struct VisitorWrapper<'a> {
            inner: &'a mut YoungMarker,
        }

        impl<'a> VisitorWrapper<'a> {
            fn new(inner: &'a mut YoungMarker) -> Self {
                VisitorWrapper { inner }
            }

            fn traverse(&mut self, raw_heap: *mut RawHeap) {
				unsafe {
					let mut i = 0;
					while i < (*raw_heap).spaces_.len() {
						let space = &mut (*raw_heap).spaces_[i];
						match space {
							HeapSpace::Normal(normal_space) => {
								for page in &mut normal_space.pages {
									self.inner.visit_normal_page(page);
								}
							}
							HeapSpace::Large(large_space) => {
								for page in &mut large_space.pages {
									self.inner.visit_large_page(page);
								}
							}
						}
						i += 1;
					}
				}
            }
        }

        #[cfg(defined(cppgc_young_generation))]
        unsafe {
			let mut young_marker = YoungMarker::new();
			let mut wrapper = VisitorWrapper::new(&mut young_marker);
			wrapper.traverse(self.raw_heap_);
        }
    }

    #[inline]
    fn in_disallow_gc_scope(&self) -> bool {
		unsafe {
			(*(*self.raw_heap_).heap_).is_gc_forbidden()
		}
    }

    #[inline]
    fn get_initial_space_index_for_size(size: usize) -> RawHeap::RegularSpaceType {
        assert_eq!(
            Self::KSMALLEST_SPACE_SIZE,
            32,
            "should be half the next larger size"
        );
        if size < 64 {
            if size < Self::KSMALLEST_SPACE_SIZE {
                return RawHeap::RegularSpaceType::kNormal1;
            }
            return RawHeap::RegularSpaceType::kNormal2;
        }
        if size < 128 {
            return RawHeap::RegularSpaceType::kNormal3;
        }
        RawHeap::RegularSpaceType::kNormal4
    }

    fn out_of_line_allocate(
        &mut self,
        space: &mut NormalPageSpace,
        size: usize,
        alignment: AlignVal,
        gcinfo: GCInfoIndex,
    ) -> *mut u8 {
        let object = self.out_of_line_allocate_gc_safe_point(space, size, alignment, gcinfo);
        object
    }

    fn allocate_object_on_space_aligned(
        &mut self,
        space: &mut NormalPageSpace,
        size: usize,
        alignment: AlignVal,
        gcinfo: GCInfoIndex,
    ) -> *mut u8 {
        static_assert!(
            2 * Self::K_ALLOCATION_GRANULARITY == api_constants::kMaxSupportedAlignment
        );
        static_assert!(
            Self::K_ALLOCATION_GRANULARITY == mem::size_of::<HeapObjectHeader>()
        );
        static_assert!(
            Self::K_ALLOCATION_GRANULARITY == api_constants::kAllocationGranularity
        );
        assert_eq!(
            2 * mem::size_of::<HeapObjectHeader>(),
            alignment.0,
            "Alignment mismatch"
        );

        const K_ALIGNMENT: usize = 2 * Self::K_ALLOCATION_GRANULARITY;
        const K_ALIGNMENT_MASK: usize = K_ALIGNMENT - 1;
        const K_PADDING_SIZE: usize = K_ALIGNMENT - mem::size_of::<HeapObjectHeader>();

        let current_lab = &mut space.linear_allocation_buffer;
        let current_lab_size = current_lab.size;

        let lab_start_aligned = (current_lab.start as usize + mem::size_of::<HeapObjectHeader>()) & K_ALIGNMENT_MASK == 0;
        let lab_allocation_will_succeed = current_lab_size >= size && lab_start_aligned;

        let lab_fits_extended_request = current_lab_size >= (size + K_PADDING_SIZE);
        let lab_allocation_will_succeed = if !lab_allocation_will_succeed && lab_fits_extended_request {
			unsafe {
				let filler_memory = current_lab.allocate(K_PADDING_SIZE);
				let filler = Filler::create_at(filler_memory, K_PADDING_SIZE);
				let base_page = BasePage::from_payload(&filler as *const _ as *mut _);
				let normal_page = NormalPage::from(base_page);

				normal_page.object_start_bitmap().set_bit::<AccessMode::kAtomic>(filler as *const _ as usize);
				true
			}
        } else {
            lab_allocation_will_succeed
        };

        if !lab_allocation_will_succeed {
            return self.out_of_line_allocate(space, size, alignment, gcinfo);
        }

        let object = self.allocate_object_on_space(space, size, gcinfo);
        assert_ne!(object as usize & K_ALIGNMENT_MASK, 0);
        object
    }

    fn allocate_object_on_space(
        &mut self,
        space: &mut NormalPageSpace,
        size: usize,
        gcinfo: GCInfoIndex,
    ) -> *mut u8 {
        assert_ne!(gcinfo.0, 0);

        let current_lab = &mut space.linear_allocation_buffer;
        if current_lab.size < size {
            return self.out_of_line_allocate(
                space,
                size,
                AlignVal(Self::K_ALLOCATION_GRANULARITY as u32),
                gcinfo,
            );
        }

        unsafe {
			let raw = current_lab.allocate(size);

			#[cfg(not(any(v8_use_memory_sanitizer, v8_use_address_sanitizer)))]
			#[cfg(debug_assertions)]
			set_memory_accessible(raw.add(mem::size_of::<HeapObjectHeader>()) as *mut u8, size - mem::size_of::<HeapObjectHeader>());

			#[cfg(any(v8_use_memory_sanitizer, v8_use_address_sanitizer, not(debug_assertions)))]
			set_memory_accessible(raw, size);

			let header = HeapObjectHeader::new(size, gcinfo);
			let header_ptr = raw as *mut HeapObjectHeader;
			header_ptr.write(header);

			let base_page = BasePage::from_payload(header_ptr as *mut _ as *mut u8);
			let normal_page = NormalPage::from(base_page);

			normal_page.object_start_bitmap().set_bit::<AccessMode::kAtomic>(header_ptr as usize);

			(header_ptr as *mut HeapObjectHeader).add(1) as *mut u8
		}
    }

    fn out_of_line_allocate_gc_safe_point(
        &mut self,
        space: &mut NormalPageSpace,
        size: usize,
        alignment: AlignVal,
        gcinfo: GCInfoIndex,
    ) -> *mut u8 {
        let object = self.out_of_line_allocate_impl(space, size, alignment, gcinfo);

		unsafe {
			(*self.stats_collector_).notify_safe_point_for_conservative_collection();
		}
        if self.is_invoking_pre_finalizers() {
			unsafe {
				HeapObjectHeader::from_object(object).mark_non_atomic();
				self.replace_linear_allocation_buffer(space,  nullptr, 0);
				self.notify_allocation_in_prefinalizer(size);
			}
        }
        object
    }

	unsafe fn replace_linear_allocation_buffer(
        &mut self,
        space: &mut NormalPageSpace,
        new_buffer: *mut u8,
        new_size: usize,
    ) {
        let lab = &mut space.linear_allocation_buffer;
        if lab.size != 0 {
            self.add_to_free_list(space, lab.start, lab.size);
			(*self.stats_collector_).notify_explicit_free(lab.size);
        }

        lab.set(new_buffer, new_size);
        if new_size != 0 {
            assert!(!new_buffer.is_null());
			(*self.stats_collector_).notify_allocation(new_size);
            let page = NormalPage::from(BasePage::from_payload(new_buffer));
            page.object_start_bitmap().clear_bit::<AccessMode::kAtomic>(new_buffer as usize);
            self.mark_range_as_young_page(page, new_buffer as usize, new_buffer as usize + new_size);
        }
    }

	unsafe fn add_to_free_list(&mut self, space: &mut NormalPageSpace, start: *mut u8, size: usize) {
        space.free_list.add(FreeListBlock {
            address: start,
            size: size,
        });
        let page = NormalPage::from(BasePage::from_payload(start));
        page.object_start_bitmap().set_bit::<AccessMode::kAtomic>(start as usize);
    }

	unsafe fn mark_range_as_young_page(&mut self, page: &mut NormalPage, begin: usize, end: usize) {
		self.mark_range_as_young(page, begin as *mut u8, end as *mut u8);
	}

	unsafe fn mark_range_as_young(&mut self, page: &mut NormalPage, begin: *mut u8, end: *mut u8) {
		#[cfg(defined(cppgc_young_generation))]
		{
			if !page.heap().generational_gc_supported() {
				return;
			}

			let new_page = begin as usize == page.payload_start() as usize && end as usize == page.payload_end() as usize;

			let age_table = &mut (*(*self.raw_heap_).heap_).local_data.age_table;

			age_table.set_age_for_range(begin as usize, end as usize, AgeTable::Age::KYoung, if new_page { AgeTable::AdjacentCardsPolicy::KIgnore } else { AgeTable::AdjacentCardsPolicy::KConsider });
			page.set_as_containing_young_objects(true);
		}
	}
	

    fn out_of_line_allocate_impl(
        &mut self,
        space: &mut NormalPageSpace,
        size: usize,
        alignment: AlignVal,
        gcinfo: GCInfoIndex,
    ) -> *mut u8 {
        assert_eq!(size & Self::K_ALLOCATION_MASK, 0);
        assert!(size >= kFreeListEntrySize);
        assert!(!self.in_disallow_gc_scope());

        if size >= Self::K_LARGE_OBJECT_SIZE_THRESHOLD {
            let large_space = unsafe {
				LargePageSpace::from((*self.raw_heap_).space(RawHeap::RegularSpaceType::kLarge))
			};
            let result = unsafe {
				self.try_allocate_large_object(large_space, size, gcinfo)
			};
            if !result.is_null() {
                return result;
            }

            for _ in 0..2 {
                let config = kOnAllocationFailureGCConfig;
				unsafe {
					(*self.garbage_collector_).collect_garbage(config);
				}
                let result = unsafe {
					self.try_allocate_large_object(large_space, size, gcinfo)
				};
                if !result.is_null() {
                    return result;
                }
            }
			unsafe {
				let message = "Oilpan: Large allocation.".to_string();
				(self.oom_handler_)(message.as_ptr() as *const i8);
			}
            
            return std::ptr::null_mut();
        }

        let mut request_size = size;
        let dynamic_alignment = alignment.0 as usize;
        if dynamic_alignment != Self::K_ALLOCATION_GRANULARITY {
            assert_eq!(
                2 * mem::size_of::<HeapObjectHeader>(),
                dynamic_alignment
            );
            request_size += Self::K_ALLOCATION_GRANULARITY;
        }

        let success = self.try_refill_linear_allocation_buffer(space, request_size);
        if !success {
            for _ in 0..2 {
                let config = kOnAllocationFailureGCConfig;
				unsafe {
					(*self.garbage_collector_).collect_garbage(config);
				}
                let success = self.try_refill_linear_allocation_buffer(space, request_size);
                if success {
                    break;
                }
            }
            if !success {
				unsafe {
					let message = "Oilpan: Normal allocation.".to_string();
					(self.oom_handler_)(message.as_ptr() as *const i8);
				}
                return std::ptr::null_mut();
            }
        }

        if dynamic_alignment == Self::K_ALLOCATION_GRANULARITY {
            self.allocate_object_on_space(space, size, gcinfo)
        } else {
            self.allocate_object_on_space_aligned(space, size, alignment, gcinfo)
        }
    }
	

    unsafe fn try_allocate_large_object(
        &mut self,
        space: &mut LargePageSpace,
        size: usize,
        gcinfo: GCInfoIndex,
    ) -> *mut u8 {
        let page = self.try_allocate_large_object_impl(space, size);
        if page.is_null() {
            return std::ptr::null_mut();
        }

        space.add_page(page);

        let header_ptr = (*page).object_header() as *mut HeapObjectHeader;
        let header = HeapObjectHeader::new(HeapObjectHeader::K_LARGE_OBJECT_SIZE_IN_HEADER, gcinfo);
		header_ptr.write(header);

		(*self.stats_collector_).notify_allocation(size);
        self.mark_range_as_young_page(page, (*page).payload_start() as usize, (*page).payload_end() as usize);

		(header_ptr as *mut HeapObjectHeader).add(1) as *mut u8
    }
	

    unsafe fn try_allocate_large_object_impl(
        &mut self,
        space: &mut LargePageSpace,
        size: usize,
    ) -> *mut NormalPage {
		let page = LargePage::try_create((*self.page_backend_), space, size);
		if !page.is_null() {
			return page as *mut NormalPage;
		}

        let sweeper = &mut (*(*space.raw_heap_).heap_).sweeper;

        if sweeper.sweep_for_allocation_if_running(
            space,
            size,
            TimeDelta::from_microseconds(500),
        ) && (LargePage::try_create((*self.page_backend_), space, size) as *mut NormalPage) != std::ptr::null_mut()
        {
			let page = LargePage::try_create((*self.page_backend_), space, size);
			if !page.is_null() {
				return page as *mut NormalPage;
			}
        }

        if sweeper.sweep_for_allocation_if_running(
            space,
            size,
            TimeDelta::max(),
        ) && (LargePage::try_create((*self.page_backend_), space, size) as *mut NormalPage) != std::ptr::null_mut()
        {
			let page = LargePage::try_create((*self.page_backend_), space, size);
			if !page.is_null() {
				return page as *mut NormalPage;
			}
        }

        if sweeper.finish_if_running() && (LargePage::try_create((*self.page_backend_), space, size) as *mut NormalPage) != std::ptr::null_mut() {
			let page = LargePage::try_create((*self.page_backend_), space, size);
			if !page.is_null() {
				return page as *mut NormalPage;
			}
        }

        std::ptr::null_mut()
    }
	

    fn try_expand_and_refill_linear_allocation_buffer(
        &mut self,
        space: &mut NormalPageSpace,
    ) -> bool {
		unsafe {
			let new_page = NormalPage::try_create((*self.page_backend_), space);
			if new_page.is_null() {
				return false;
			}

			space.add_page(new_page);
			self.replace_linear_allocation_buffer(space, (*new_page).payload_start() as *mut u8, (*new_page).payload_size());
		}
        true
    }

    fn try_refill_linear_allocation_buffer(
        &mut self,
        space: &mut NormalPageSpace,
        size: usize,
    ) -> bool {
        if self.try_refill_linear_allocation_buffer_from_free_list(space, size) {
            return true;
        }

		unsafe {
			let sweeper = &mut (*(*(*self.raw_heap_).heap_).sweeper);

			if sweeper.sweep_for_allocation_if_running(
				space,
				size,
				TimeDelta::from_microseconds(500),
			) && self.try_refill_linear_allocation_buffer_from_free_list(space, size)
			{
				return true;
			}

			if self.try_expand_and_refill_linear_allocation_buffer(space) {
				return true;
			}

			if sweeper.sweep_for_allocation_if_running(
				space,
				size,
				TimeDelta::max(),
			) && self.try_refill_linear_allocation_buffer_from_free_list(space, size)
			{
				return true;
			}

			if sweeper.finish_if_running() {
				if self.try_refill_linear_allocation_buffer_from_free_list(space, size) {
					return true;
				}
				if self.try_expand_and_refill_linear_allocation_buffer(space) {
					return true;
				}
			}
		}
        false
    }
	

    fn try_refill_linear_allocation_buffer_from_free_list(
        &mut self,
        space: &mut NormalPageSpace,
        size: usize,
    ) -> bool {
        let entry = space.free_list.allocate(size);
        if entry.address.is_null() {
            return false;
        }

        unsafe {
			let page = NormalPage::from(BasePage::from_payload(entry.address));
			if page.discarded_memory != 0 {
				(*self.stats_collector_).decrement_discarded_memory(page.discarded_memory);
				page.discarded_memory = 0;
			}

			self.replace_linear_allocation_buffer(space, entry.address, entry.size);
		}
        true
    }

    #[cfg(v8_enable_allocation_timeout)]
    fn update_allocation_timeout(&mut self) {
		unsafe {
			self.allocation_timeout_ = (*self.garbage_collector_).update_allocation_timeout();
		}
    }

    #[cfg(v8_enable_allocation_timeout)]
    fn trigger_gc_on_allocation_timeout_if_needed(&mut self) {
        if self.allocation_timeout_.is_none() {
            return;
        }
        if self.allocation_timeout_.unwrap() <= 0 {
            eprintln!("FATAL: Invalid allocation timeout value.");
            std::process::abort();
        }

        self.allocation_timeout_.replace(self.allocation_timeout_.unwrap() - 1);
        if self.allocation_timeout_.unwrap() == 0 {
            let config = kOnAllocationFailureGCConfig;
			unsafe {
				(*self.garbage_collector_).collect_garbage(config);
				self.allocation_timeout_ = (*self.garbage_collector_).update_allocation_timeout();
			}
            if self.allocation_timeout_.is_none() {
                eprintln!("FATAL: Allocation timeout should not be None after GC.");
                std::process::abort();
            }
            if self.allocation_timeout_.unwrap() <= 0 {
                eprintln!("FATAL: Invalid allocation timeout value after GC.");
                std::process::abort();
            }
        }
    }

    fn is_invoking_pre_finalizers(&self) -> bool {
		unsafe {
			(*self.prefinalizer_handler_).is_invoking_pre_finalizers()
		}
    }

    fn notify_allocation_in_prefinalizer(&mut self, size: usize) {
		unsafe {
			(*self.prefinalizer_handler_).notify_allocation_in_prefinalizer(size)
		}
    }
}
#[derive(Clone, Copy, Debug)]
pub struct AlignVal(pub u32);

fn round_up<const GRANULARITY: usize>(size: usize) -> usize {
    (size + GRANULARITY - 1) & !(GRANULARITY - 1)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct GCInfoIndex(usize);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct CustomSpaceIndex(usize);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AccessMode {
    KNonAtomic,
    KAtomic,
}

#[derive(Debug)]
struct LinearAllocationBuffer {
    start: *mut u8,
    size: usize,
}

impl LinearAllocationBuffer {
    fn new() -> Self {
        LinearAllocationBuffer {
            start: std::ptr::null_mut(),
            size: 0,
        }
    }

    fn set(&mut self, start: *mut u8, size: usize) {
        self.start = start;
        self.size = size;
    }

    unsafe fn allocate(&mut self, size: usize) -> *mut u8 {
        if size > self.size {
            return std::ptr::null_mut();
        }
        let ptr = self.start;
        self.start = self.start.add(size);
        self.size -= size;
        ptr
    }
}

struct FreeListBlock {
    address: *mut u8,
    size: usize,
}

struct FreeList {
    blocks: Vec<FreeListBlock>,
}

impl FreeList {
    fn new() -> Self {
        FreeList { blocks: Vec::new() }
    }

    fn add(&mut self, block: FreeListBlock) {
        self.blocks.push(block);
    }

    fn allocate(&mut self, size: usize) -> FreeListBlock {
        if let Some(index) = self.blocks.iter().position(|b| b.size >= size) {
            let mut block = self.blocks.remove(index);
            if block.size > size {
                let remaining_size = block.size - size;
                let remaining_address = unsafe { block.address.add(size) };
                self.add(FreeListBlock {
                    address: remaining_address,
                    size: remaining_size,
                });
            }
            block.size = size;
            return block;
        }
        FreeListBlock {
            address: std::ptr::null_mut(),
            size: 0,
        }
    }
}

struct HeapObjectHeader {
    size: usize,
    gcinfo_index: GCInfoIndex,
}
impl HeapObjectHeader {
	const K_LARGE_OBJECT_SIZE_IN_HEADER: usize = usize::MAX;
	fn new(size: usize, gcinfo_index: GCInfoIndex) -> Self {
		HeapObjectHeader {
			size,
			gcinfo_index,
		}
	}

	
