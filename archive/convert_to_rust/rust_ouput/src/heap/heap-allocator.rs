// Converted from V8 C++ source files:
// Header: heap-allocator.h
// Implementation: heap-allocator.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::cell::RefCell;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::null_mut;
use std::rc::Rc;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use crate::AllocationType;
use crate::Heap;
use crate::LocalHeap;
use crate::ReadOnlySpace;

#[derive(Debug, Clone, Copy)]
pub enum AllocationOrigin {
    kRuntime,
}

#[derive(Debug, Clone, Copy)]
pub enum AllocationAlignment {
    kTaggedAligned,
}

impl AllocationAlignment {
    const kTaggedAligned: AllocationAlignment = AllocationAlignment::kTaggedAligned;
}

pub struct AllocationResult {}

pub struct HeapAllocator {
    local_heap_: *mut LocalHeap,
    heap_: *mut Heap,
    spaces_: [*mut Space; 12],
    read_only_space_: *mut ReadOnlySpace,

    new_space_allocator_: Option<MainAllocator>,
    old_space_allocator_: Option<MainAllocator>,
    trusted_space_allocator_: Option<MainAllocator>,
    code_space_allocator_: Option<MainAllocator>,

    shared_space_allocator_: Option<MainAllocator>,
    shared_trusted_space_allocator_: Option<MainAllocator>,
    shared_lo_space_: *mut OldLargeObjectSpace,
    shared_trusted_lo_space_: *mut SharedTrustedLargeObjectSpace,

    allocation_timeout_: Option<i32>,
}

impl HeapAllocator {
    pub fn new(local_heap: *mut LocalHeap) -> Self {
        HeapAllocator {
            local_heap_: local_heap,
            heap_: unsafe { (*local_heap).heap() },
            spaces_: [null_mut(); 12],
            read_only_space_: null_mut(),
            new_space_allocator_: None,
            old_space_allocator_: None,
            trusted_space_allocator_: None,
            code_space_allocator_: None,
            shared_space_allocator_: None,
            shared_trusted_space_allocator_: None,
            shared_lo_space_: null_mut(),
            shared_trusted_lo_space_: null_mut(),
            allocation_timeout_: None,
        }
    }

    pub fn setup(
        &mut self,
        new_allocation_info: *mut LinearAllocationArea,
        old_allocation_info: *mut LinearAllocationArea,
    ) {
        for i in 0..12 {
            self.spaces_[i] = unsafe { (*self.heap_).space(i) };
        }

        // Mimicking the C++ flags for now with a simple boolean.
        let sticky_mark_bits = false;

        let local_heap = unsafe { &*self.local_heap_ };
        let heap = unsafe { &*self.heap_ };

        if (unsafe { (*self.heap_).new_space() } != null_mut() || sticky_mark_bits)
            && local_heap.is_main_thread()
        {
            let space = if sticky_mark_bits {
                unsafe { (*self.heap_).sticky_space() }
            } else {
                unsafe { (*self.heap_).new_space() }
            };
            self.new_space_allocator_ = Some(MainAllocator::new(
                self.local_heap_,
                space,
                MainAllocator::IsNewGeneration::kYes,
                new_allocation_info,
            ));
        }

        self.old_space_allocator_ = Some(MainAllocator::new(
            self.local_heap_,
            unsafe { (*self.heap_).old_space() },
            MainAllocator::IsNewGeneration::kNo,
            old_allocation_info,
        ));

        self.trusted_space_allocator_ = Some(MainAllocator::new(
            self.local_heap_,
            unsafe { (*self.heap_).trusted_space() },
            MainAllocator::IsNewGeneration::kNo,
            null_mut(),
        ));
        self.code_space_allocator_ = Some(MainAllocator::new(
            self.local_heap_,
            unsafe { (*self.heap_).code_space() },
            MainAllocator::IsNewGeneration::kNo,
            null_mut(),
        ));

        let isolate = unsafe { (*self.heap_).isolate() };
        if unsafe { (*isolate).has_shared_space() } {
            self.shared_space_allocator_ = Some(MainAllocator::new(
                self.local_heap_,
                unsafe { (*self.heap_).shared_allocation_space() },
                MainAllocator::IsNewGeneration::kNo,
                null_mut(),
            ));
            self.shared_lo_space_ = unsafe { (*self.heap_).shared_lo_allocation_space() };

            self.shared_trusted_space_allocator_ = Some(MainAllocator::new(
                self.local_heap_,
                unsafe { (*self.heap_).shared_trusted_allocation_space() },
                MainAllocator::IsNewGeneration::kNo,
                null_mut(),
            ));
            self.shared_trusted_lo_space_ =
                unsafe { (*self.heap_).shared_trusted_lo_allocation_space() };
        }
    }

    pub fn set_read_only_space(&mut self, read_only_space: *mut ReadOnlySpace) {
        self.read_only_space_ = read_only_space;
    }

    pub fn allocate_raw(
        &self,
        size_in_bytes: i32,
        allocation: AllocationType,
        origin: AllocationOrigin,
        alignment: AllocationAlignment,
    ) -> AllocationResult {
        if size_in_bytes > unsafe { (*self.heap_).max_regular_heap_object_size(allocation) } {
            return self.allocate_raw_large_internal(size_in_bytes, allocation, origin, alignment);
        }

        match allocation {
            AllocationType::kYoung => {
                if let Some(new_space_allocator) = &self.new_space_allocator_ {
                    new_space_allocator.allocate_raw(size_in_bytes)
                } else {
                    AllocationResult {}
                }
            }
            AllocationType::kOld => {
                if let Some(old_space_allocator) = &self.old_space_allocator_ {
                    old_space_allocator.allocate_raw(size_in_bytes)
                } else {
                    AllocationResult {}
                }
            }
            AllocationType::kCode => {
                if let Some(code_space_allocator) = &self.code_space_allocator_ {
                    code_space_allocator.allocate_raw(size_in_bytes)
                } else {
                    AllocationResult {}
                }
            }
            AllocationType::kMap => {
                if let Some(old_space_allocator) = &self.old_space_allocator_ {
                    old_space_allocator.allocate_raw(size_in_bytes)
                } else {
                    AllocationResult {}
                }
            }
            AllocationType::kReadOnly => {
                AllocationResult {}
            }
            AllocationType::kSharedOld => {
                if let Some(shared_space_allocator) = &self.shared_space_allocator_ {
                    shared_space_allocator.allocate_raw(size_in_bytes)
                } else {
                    AllocationResult {}
                }
            }
            AllocationType::kTrusted => {
                if let Some(trusted_space_allocator) = &self.trusted_space_allocator_ {
                    trusted_space_allocator.allocate_raw(size_in_bytes)
                } else {
                    AllocationResult {}
                }
            }
            AllocationType::kSharedMap => {
                if let Some(shared_space_allocator) = &self.shared_space_allocator_ {
                    shared_space_allocator.allocate_raw(size_in_bytes)
                } else {
                    AllocationResult {}
                }
            }
            AllocationType::kSharedTrusted => {
                if let Some(shared_trusted_space_allocator) =
                    &self.shared_trusted_space_allocator_
                {
                    shared_trusted_space_allocator.allocate_raw(size_in_bytes)
                } else {
                    AllocationResult {}
                }
            }
        }
    }

    pub fn allocate_raw_with<const MODE: AllocationRetryMode>(
        &self,
        size: i32,
        allocation: AllocationType,
        origin: AllocationOrigin,
        alignment: AllocationAlignment,
    ) -> Tagged<HeapObject> {
        match MODE {
            AllocationRetryMode::kLightRetry => {
                let result =
                    self.allocate_raw_with_light_retry_slow_path(size, allocation, origin, alignment);
                Tagged {
                    _phantom: PhantomData,
                }
            }
            AllocationRetryMode::kRetryOrFail => {
                let result =
                    self.allocate_raw_with_retry_or_fail_slow_path(size, allocation, origin, alignment);
                Tagged {
                    _phantom: PhantomData,
                }
            }
        }
    }

    pub fn can_allocate_in_read_only_space(&self) -> bool {
        self.read_only_space_ != null_mut()
    }

    pub fn free_linear_allocation_areas(&mut self) {
        if let Some(new_space_allocator) = &mut self.new_space_allocator_ {
            new_space_allocator.free_linear_allocation_area();
        }
        if let Some(old_space_allocator) = &mut self.old_space_allocator_ {
            old_space_allocator.free_linear_allocation_area();
        }
        if let Some(trusted_space_allocator) = &mut self.trusted_space_allocator_ {
            trusted_space_allocator.free_linear_allocation_area();
        }
        if let Some(code_space_allocator) = &mut self.code_space_allocator_ {
            code_space_allocator.free_linear_allocation_area();
        }

        if let Some(shared_space_allocator) = &mut self.shared_space_allocator_ {
            shared_space_allocator.free_linear_allocation_area();
        }

        if let Some(shared_trusted_space_allocator) = &mut self.shared_trusted_space_allocator_ {
            shared_trusted_space_allocator.free_linear_allocation_area();
        }
    }

    pub fn make_linear_allocation_areas_iterable(&mut self) {
        if let Some(new_space_allocator) = &mut self.new_space_allocator_ {
            new_space_allocator.make_linear_allocation_area_iterable();
        }
        if let Some(old_space_allocator) = &mut self.old_space_allocator_ {
            old_space_allocator.make_linear_allocation_area_iterable();
        }
        if let Some(trusted_space_allocator) = &mut self.trusted_space_allocator_ {
            trusted_space_allocator.make_linear_allocation_area_iterable();
        }
        if let Some(code_space_allocator) = &mut self.code_space_allocator_ {
            code_space_allocator.make_linear_allocation_area_iterable();
        }
        if let Some(shared_space_allocator) = &mut self.shared_space_allocator_ {
            shared_space_allocator.make_linear_allocation_area_iterable();
        }

        if let Some(shared_trusted_space_allocator) = &mut self.shared_trusted_space_allocator_ {
            shared_trusted_space_allocator.make_linear_allocation_area_iterable();
        }
    }

    pub fn mark_linear_allocation_areas_black(&mut self) {
        if let Some(old_space_allocator) = &mut self.old_space_allocator_ {
            old_space_allocator.mark_linear_allocation_area_black();
        }
        if let Some(trusted_space_allocator) = &mut self.trusted_space_allocator_ {
            trusted_space_allocator.mark_linear_allocation_area_black();
        }
        if let Some(code_space_allocator) = &mut self.code_space_allocator_ {
            code_space_allocator.mark_linear_allocation_area_black();
        }
    }

    pub fn unmark_linear_allocations_area(&mut self) {
        if let Some(old_space_allocator) = &mut self.old_space_allocator_ {
            old_space_allocator.unmark_linear_allocation_area();
        }
        if let Some(trusted_space_allocator) = &mut self.trusted_space_allocator_ {
            trusted_space_allocator.unmark_linear_allocation_area();
        }
        if let Some(code_space_allocator) = &mut self.code_space_allocator_ {
            code_space_allocator.unmark_linear_allocation_area();
        }
    }

    pub fn mark_shared_linear_allocation_areas_black(&mut self) {
        if let Some(shared_space_allocator) = &mut self.shared_space_allocator_ {
            shared_space_allocator.mark_linear_allocation_area_black();
        }
        if let Some(shared_trusted_space_allocator) = &mut self.shared_trusted_space_allocator_ {
            shared_trusted_space_allocator.mark_linear_allocation_area_black();
        }
    }

    pub fn unmark_shared_linear_allocation_areas(&mut self) {
        if let Some(shared_space_allocator) = &mut self.shared_space_allocator_ {
            shared_space_allocator.unmark_linear_allocation_area();
        }
        if let Some(shared_trusted_space_allocator) = &mut self.shared_trusted_space_allocator_ {
            shared_trusted_space_allocator.unmark_linear_allocation_area();
        }
    }

    pub fn free_linear_allocation_areas_and_reset_free_lists(&mut self) {
        if let Some(old_space_allocator) = &mut self.old_space_allocator_ {
            old_space_allocator.free_linear_allocation_area_and_reset_free_list();
        }
        if let Some(trusted_space_allocator) = &mut self.trusted_space_allocator_ {
            trusted_space_allocator.free_linear_allocation_area_and_reset_free_list();
        }
        if let Some(code_space_allocator) = &mut self.code_space_allocator_ {
            code_space_allocator.free_linear_allocation_area_and_reset_free_list();
        }
    }

    pub fn free_shared_linear_allocation_areas_and_reset_free_lists(&mut self) {
        if let Some(shared_space_allocator) = &mut self.shared_space_allocator_ {
            shared_space_allocator.free_linear_allocation_area_and_reset_free_list();
        }
        if let Some(shared_trusted_space_allocator) = &mut self.shared_trusted_space_allocator_ {
            shared_trusted_space_allocator.free_linear_allocation_area_and_reset_free_list();
        }
    }

    pub fn pause_allocation_observers(&mut self) {
        if let Some(new_space_allocator) = &mut self.new_space_allocator_ {
            new_space_allocator.pause_allocation_observers();
        }
        if let Some(old_space_allocator) = &mut self.old_space_allocator_ {
            old_space_allocator.pause_allocation_observers();
        }
        if let Some(trusted_space_allocator) = &mut self.trusted_space_allocator_ {
            trusted_space_allocator.pause_allocation_observers();
        }
        if let Some(code_space_allocator) = &mut self.code_space_allocator_ {
            code_space_allocator.pause_allocation_observers();
        }
    }

    pub fn resume_allocation_observers(&mut self) {
        if let Some(new_space_allocator) = &mut self.new_space_allocator_ {
            new_space_allocator.resume_allocation_observers();
        }
        if let Some(old_space_allocator) = &mut self.old_space_allocator_ {
            old_space_allocator.resume_allocation_observers();
        }
        if let Some(trusted_space_allocator) = &mut self.trusted_space_allocator_ {
            trusted_space_allocator.resume_allocation_observers();
        }
        if let Some(code_space_allocator) = &mut self.code_space_allocator_ {
            code_space_allocator.resume_allocation_observers();
        }
    }

    pub fn publish_pending_allocations(&mut self) {
        if let Some(new_space_allocator) = &mut self.new_space_allocator_ {
            new_space_allocator.move_original_top_forward();
        }

        if let Some(old_space_allocator) = &mut self.old_space_allocator_ {
            old_space_allocator.move_original_top_forward();
        }
        if let Some(trusted_space_allocator) = &mut self.trusted_space_allocator_ {
            trusted_space_allocator.move_original_top_forward();
        }
        if let Some(code_space_allocator) = &mut self.code_space_allocator_ {
            code_space_allocator.move_original_top_forward();
        }

        unsafe { (*self.lo_space()).reset_pending_object() };
        if unsafe { (*self.new_lo_space()).new_lo_space() } != null_mut() {
            unsafe { (*(*self.new_lo_space()).new_lo_space()).reset_pending_object() };
        }
        unsafe { (*self.code_lo_space()).reset_pending_object() };
        unsafe { (*self.trusted_lo_space()).reset_pending_object() };
    }

    pub fn add_allocation_observer(
        &mut self,
        observer: *mut AllocationObserver,
        new_space_observer: *mut AllocationObserver,
    ) {
        if let Some(new_space_allocator) = &mut self.new_space_allocator_ {
            new_space_allocator.add_allocation_observer(new_space_observer);
        }
        if unsafe { (*self.new_lo_space()).new_lo_space() } != null_mut() {
            unsafe {
                (*(*self.new_lo_space()).new_lo_space())
                    .add_allocation_observer(new_space_observer)
            };
        }
        if let Some(old_space_allocator) = &mut self.old_space_allocator_ {
            old_space_allocator.add_allocation_observer(observer);
        }
        unsafe { (*self.lo_space()).add_allocation_observer(observer) };
        if let Some(trusted_space_allocator) = &mut self.trusted_space_allocator_ {
            trusted_space_allocator.add_allocation_observer(observer);
        }
        unsafe { (*self.trusted_lo_space()).add_allocation_observer(observer) };
        if let Some(code_space_allocator) = &mut self.code_space_allocator_ {
            code_space_allocator.add_allocation_observer(observer);
        }
        unsafe { (*self.code_lo_space()).add_allocation_observer(observer) };
    }

    pub fn remove_allocation_observer(
        &mut self,
        observer: *mut AllocationObserver,
        new_space_observer: *mut AllocationObserver,
    ) {
        if let Some(new_space_allocator) = &mut self.new_space_allocator_ {
            new_space_allocator.remove_allocation_observer(new_space_observer);
        }
        if unsafe { (*self.new_lo_space()).new_lo_space() } != null_mut() {
            unsafe {
                (*(*self.new_lo_space()).new_lo_space())
                    .remove_allocation_observer(new_space_observer)
            };
        }
        if let Some(old_space_allocator) = &mut self.old_space_allocator_ {
            old_space_allocator.remove_allocation_observer(observer);
        }
        unsafe { (*self.lo_space()).remove_allocation_observer(observer) };
        if let Some(trusted_space_allocator) = &mut self.trusted_space_allocator_ {
            trusted_space_allocator.remove_allocation_observer(observer);
        }
        unsafe { (*self.trusted_lo_space()).remove_allocation_observer(observer) };
        if let Some(code_space_allocator) = &mut self.code_space_allocator_ {
            code_space_allocator.remove_allocation_observer(observer);
        }
        unsafe { (*self.code_lo_space()).remove_allocation_observer(observer) };
    }

    pub fn new_space_allocator(&mut self) -> &mut MainAllocator {
        self.new_space_allocator_.as_mut().unwrap()
    }

    pub fn old_space_allocator(&mut self) -> &mut MainAllocator {
        self.old_space_allocator_.as_mut().unwrap()
    }

    pub fn trusted_space_allocator(&mut self) -> &mut MainAllocator {
        self.trusted_space_allocator_.as_mut().unwrap()
    }

    pub fn code_space_allocator(&mut self) -> &mut MainAllocator {
        self.code_space_allocator_.as_mut().unwrap()
    }

    pub fn shared_space_allocator(&mut self) -> &mut MainAllocator {
        self.shared_space_allocator_.as_mut().unwrap()
    }

    fn code_space(&self) -> *mut PagedSpace {
        unsafe { (*self.heap_).code_space() }
    }
    fn code_lo_space(&self) -> *mut CodeLargeObjectSpace {
        unsafe { (*self.heap_).code_lo_space() }
    }
    fn new_space(&self) -> *mut NewSpace {
        unsafe { (*self.heap_).new_space() }
    }
    fn new_lo_space(&self) -> *mut NewLargeObjectSpace {
        unsafe { (*self.heap_).new_lo_space() }
    }
    fn lo_space(&self) -> *mut OldLargeObjectSpace {
        unsafe { (*self.heap_).lo_space() }
    }
    fn shared_lo_space(&self) -> *mut OldLargeObjectSpace {
        self.shared_lo_space_
    }
    fn shared_trusted_lo_space(&self) -> *mut SharedTrustedLargeObjectSpace {
        self.shared_trusted_lo_space_
    }
    fn old_space(&self) -> *mut PagedSpace {
        unsafe { (*self.heap_).old_space() }
    }
    fn read_only_space(&self) -> *mut ReadOnlySpace {
        self.read_only_space_
    }
    fn trusted_space(&self) -> *mut PagedSpace {
        unsafe { (*self.heap_).trusted_space() }
    }
    fn trusted_lo_space(&self) -> *mut OldLargeObjectSpace {
        unsafe { (*self.heap_).trusted_lo_space() }
    }

    fn allocate_raw_large_internal(
        &self,
        size_in_bytes: i32,
        allocation: AllocationType,
        origin: AllocationOrigin,
        alignment: AllocationAlignment,
    ) -> AllocationResult {
        match allocation {
            AllocationType::kYoung => {
                unsafe { (*self.new_lo_space()).allocate_raw(self.local_heap_, size_in_bytes) }
            }
            AllocationType::kOld => {
                unsafe { (*self.lo_space()).allocate_raw(self.local_heap_, size_in_bytes) }
            }
            AllocationType::kCode => {
                unsafe { (*self.code_lo_space()).allocate_raw(self.local_heap_, size_in_bytes) }
            }
            AllocationType::kSharedOld => {
                unsafe { (*self.shared_lo_space()).allocate_raw(self.local_heap_, size_in_bytes) }
            }
            AllocationType::kTrusted => {
                unsafe { (*self.trusted_lo_space()).allocate_raw(self.local_heap_, size_in_bytes) }
            }
            AllocationType::kSharedTrusted => unsafe {
                (*self.shared_trusted_lo_space()).allocate_raw(self.local_heap_, size_in_bytes)
            },
            AllocationType::kMap => AllocationResult {},
            AllocationType::kReadOnly => AllocationResult {},
            AllocationType::kSharedMap => AllocationResult {},
        }
    }

    fn collect_garbage(&mut self, allocation: AllocationType) {
        if is_shared_allocation_type(allocation) {
            unsafe {
                (*self.heap_).collect_garbage_shared(
                    self.local_heap_,
                    GarbageCollectionReason::kAllocationFailure,
                )
            };
        } else if unsafe { (*self.local_heap_).is_main_thread() } {
            let space_to_gc = allocation_type_to_gcspace(allocation);
            unsafe {
                (*self.heap_).collect_garbage(
                    space_to_gc,
                    GarbageCollectionReason::kAllocationFailure,
                )
            };
        } else {
            unsafe { (*self.heap_).collect_garbage_from_any_thread(self.local_heap_) };
        }
    }

    fn allocate_raw_with_retry_or_fail_slow_path(
        &self,
        size: i32,
        allocation: AllocationType,
        origin: AllocationOrigin,
        alignment: AllocationAlignment,
    ) -> AllocationResult {
        let allocate = |allocation: AllocationType| {
            self.allocate_raw(size, allocation, origin, alignment)
        };
        let retry_allocate =
            |allocation: AllocationType| self.retry_allocate_raw(size, allocation, origin, alignment);
        self.allocate_raw_with_retry_or_fail_slow_path_internal(allocate, retry_allocate, allocation)
    }

    fn collect_all_available_garbage(&mut self, allocation: AllocationType) {
        if is_shared_allocation_type(allocation) {
            unsafe {
                (*self.heap_).collect_garbage_shared(
                    (*self.heap_).main_thread_local_heap(),
                    GarbageCollectionReason::kLastResort,
                )
            };
        } else if unsafe { (*self.local_heap_).is_main_thread() } {
            unsafe {
                (*self.heap_).collect_all_available_garbage(GarbageCollectionReason::kLastResort)
            };
        } else {
            unsafe { (*self.heap_).collect_garbage_from_any_thread(self.local_heap_) };
        }
    }

    fn retry_allocate_raw(
        &self,
        size_in_bytes: i32,
        allocation: AllocationType,
        origin: AllocationOrigin,
        alignment: AllocationAlignment,
    ) -> AllocationResult {
        unsafe {
            if (*self.local_heap_).is_retry_of_failed_allocation() {
                return AllocationResult {};
            }
            (*self.local_heap_).set_retry_of_failed_allocation(true);
        }
        let result = self.allocate_raw(size_in_bytes, allocation, origin, alignment);
        unsafe {
            (*self.local_heap_).set_retry_of_failed_allocation(false);
        }
        result
    }

    fn allocate_raw_with_light_retry_slow_path(
        &self,
        size: i32,
        allocation: AllocationType,
        origin: AllocationOrigin,
        alignment: AllocationAlignment,
    ) -> AllocationResult {
        let allocate = |allocation: AllocationType| {
            self.allocate_raw(size, allocation, origin, alignment)
        };
        let retry_allocate =
            |allocation: AllocationType| self.retry_allocate_raw(size, allocation, origin, alignment);
        self.allocate_raw_with_light_retry_slow_path_internal(allocate, retry_allocate, allocation)
    }

    fn allocate_raw_with_retry_or_fail_slow_path_internal<
        AllocateFunction,
        RetryFunction,
    >(
        &self,
        allocate: AllocateFunction,
        retry_allocate: RetryFunction,
        allocation: AllocationType,
    ) -> AllocationResult
    where
        AllocateFunction: Fn(AllocationType) -> AllocationResult,
        RetryFunction: Fn(AllocationType) -> AllocationResult,
    {
        AllocationResult {}
    }

    fn allocate_raw_with_light_retry_slow_path_internal<
        AllocateFunction,
        RetryFunction,
    >(
        &self,
        allocate: AllocateFunction,
        retry_allocate: RetryFunction,
        allocation: AllocationType,
    ) -> AllocationResult
    where
        AllocateFunction: Fn(AllocationType) -> AllocationResult,
        RetryFunction: Fn(AllocationType) -> AllocationResult,
    {
        AllocationResult {}
    }

    fn reached_allocation_timeout(&mut self) -> bool {
        if self.allocation_timeout_.is_none() {
            return false;
        }

        if unsafe { (*self.heap_).always_allocate() }
            || unsafe { (*self.local_heap_).is_retry_of_failed_allocation() }
        {
            return false;
        }

        let allocation_timeout = self.allocation_timeout_.unwrap();
        self.allocation_timeout_ = Some(std::cmp::max(0, allocation_timeout - 1));
        self.allocation_timeout_.unwrap() <= 0
    }

    pub fn custom_allocate_with_retry_or_fail<Function>(
        &mut self,
        allocate: Function,
        allocation: AllocationType,
    ) -> i32
    where
        Function: FnOnce(AllocationType) -> i32,
    {
        let result = allocate(allocation);
        result
    }
}

fn is_shared_allocation_type(allocation: AllocationType) -> bool {
    matches!(
        allocation,
        AllocationType::kSharedOld
            | AllocationType::kSharedMap
            | AllocationType::kSharedTrusted
    )
}

fn allocation_type_to_gcspace(allocation: AllocationType) -> AllocationSpace {
    match allocation {
        AllocationType::kYoung => AllocationSpace::NEW_SPACE,
        AllocationType::kOld
        | AllocationType::kCode
        | AllocationType::kMap
        | AllocationType::kTrusted => AllocationSpace::OLD_SPACE,
        AllocationType::kReadOnly
        | AllocationType::kSharedMap
        | AllocationType::kSharedOld
        | AllocationType::kSharedTrusted => panic!("UNREACHABLE"),
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AllocationSpace {
    NEW_SPACE,
    OLD_SPACE,
}

pub struct LinearAllocationArea {}

pub struct Space {}

pub struct PagedSpace {}

pub struct CodeLargeObjectSpace {}

pub struct NewSpace {}

pub struct NewLargeObjectSpace {}

pub struct OldLargeObjectSpace {}

pub struct SharedTrustedLargeObjectSpace {}

pub struct AllocationObserver {}

pub struct MainAllocator {
    local_heap_: *mut LocalHeap,
    space_: *mut Space,
    is_new_generation_: IsNewGeneration,
    linear_allocation_area_: *mut LinearAllocationArea,
}

impl MainAllocator {
    pub fn new(
        local_heap: *mut LocalHeap,
        space: *mut Space,
        is_new_generation: IsNewGeneration,
        linear_allocation_area: *mut LinearAllocationArea,
    ) -> Self {
        MainAllocator {
            local_heap_: local_heap,
            space_: space,
            is_new_generation_: is_new_generation,
            linear_allocation_area_: linear_allocation_area,
        }
    }

    fn allocate_raw(&self, size_in_bytes: i32) -> AllocationResult {
        AllocationResult {}
    }

    fn make_linear_allocation_area_iterable(&mut self) {}

    fn verify(&const self) {}

    fn mark_linear_allocation_area_black(&mut self) {}

    fn unmark_linear_allocation_area(&mut self) {}

    fn free_linear_allocation_area_and_reset_free_list(&mut self) {}

    fn free_linear_allocation_area(&mut self) {}

    fn move_original_top_forward(&mut self) {}

    fn add_allocation_observer(&mut self, _observer: *mut AllocationObserver) {}

    fn remove_allocation_observer(&mut self, _observer: *mut AllocationObserver) {}

    fn pause_allocation_observers(&mut self) {}

    fn resume_allocation_observers(&mut self) {}
}

impl Drop for MainAllocator {
    fn drop(&mut self) {}
}

pub enum IsNewGeneration {
    kYes,
    kNo,
}

#[derive(Debug, Clone, Copy)]
pub enum GarbageCollectionReason {
    kAllocationFailure
