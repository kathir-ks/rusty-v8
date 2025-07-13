// Converted from V8 C++ source files:
// Header: backing-store.h
// Implementation: backing-store.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod backing_store {
    // === HEADER CONTENT ===
    // Copyright 2019 the V8 project authors. All rights reserved.
    // Use of this source code is governed by a BSD-style license that can be
    // found in the LICENSE file.

    #![allow(dead_code)]
    use std::sync::{Mutex, MutexGuard, PoisonError, atomic::{AtomicU32, AtomicUsize, Ordering}, Arc};
    use std::mem::MaybeUninit;
    use std::ops::{BitAnd, BitOr, BitAndAssign, BitOrAssign, Sub};
    use crate::ArrayBufferAllocator;
    use crate::WasmMemoryObject;
    use crate::Isolate;
    use crate::SharedFunctionInfo;
    use crate::SharedFlag;
    use crate::InitializedFlag;
    use crate::ResizableFlag;
    use crate::base;
    use std::collections::HashMap;
    use std::sync::Weak;
    use std::ptr::null_mut;
    use crate::FixedArray;
    use crate::JSArrayBuffer;
    use crate::WeakArrayList;
    use crate::HandleScope;
    use crate::HeapObject;
    use crate::Factory;
    use crate::WasmMemoryFlag;
    use crate::PageAllocator;
    use crate::Counters;
    use crate::MemoryPressureLevel;
    use crate::sandbox::sandbox::Sandbox;
    use crate::isolate::isolate_group::IsolateGroup;
    use crate::wasm::wasm_engine::WasmEngine;
    use crate::trap_handler;
    use crate::wasm::wasm_limits;
    use crate::wasm::wasm_constants;
    use crate::handles::global_handles::GlobalHandles;
    use crate::wasm::wasm_objects_inl::WasmMemoryObjectExtensions;
    use crate::stack_guard::StackGuard;
    use crate::Heap;
    use crate::Tagged;
    use crate::v8;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum AllocationStatus {
        kSuccess,
        kSuccessAfterRetry,
        kAddressSpaceLimitReachedFailure,
        kOtherFailure,
    }

    const MB: usize = 1024 * 1024;
    const GB: usize = 1024 * MB;

    const KFULL_GUARD_SIZE32: usize = 8 * GB;

    // Whether this is Wasm memory, and if 32 or 64 bit.
    // Whether the backing store is shared or not.
    // Whether the backing store is resizable or not.
    // Whether the backing store memory is initialied to zero or not.
    // Internal information for shared wasm memories. E.g. contains
    // a list of all memory objects (across all isolates) that share this
    // backing store.
    pub struct SharedWasmMemoryData {
        pub isolates_: Vec<*mut Isolate>,
    }

    // The {BackingStore} data structure stores all the low-level details about the
    // backing store of an array buffer or Wasm memory, including its base address
    // and length, whether it is shared, provided by the embedder, has guard
    // regions, etc. Instances of this classes *own* the underlying memory
    // when they are created through one of the {Allocate()} methods below,
    // and the destructor frees the memory (and page allocation if necessary).
    #[derive(Debug)]
    pub struct BackingStore {
        buffer_start_: *mut std::ffi::c_void,
        byte_length_: AtomicUsize,
        max_byte_length_: usize,
        byte_capacity_: usize,
        id_: u32,
        page_allocator_: *mut PageAllocator,
        type_specific_data_: TypeSpecificData,
        flags_: AtomicEnumSet<Flag, u16>,
    }

    impl Drop for BackingStore {
        fn drop(&mut self) {
            unsafe {
                GlobalBackingStoreRegistry::Unregister(self);
            }

            struct ClearSharedAllocator<'a> {
                bs: &'a BackingStore,
            }

            impl<'a> Drop for ClearSharedAllocator<'a> {
                fn drop(&mut self) {
                    if !self.bs.holds_shared_ptr_to_allocator() {
                        return;
                    }
                    //TODO: Implement the drop for the shared_ptr.
                    //self.bs.type_specific_data_.v8_api_array_buffer_allocator_shared
                    //    .std::shared_ptr::<v8::ArrayBuffer::Allocator>::~shared_ptr();
                }
            }

            let clear_shared_allocator = ClearSharedAllocator { bs: self };

            if self.buffer_start_.is_null() {
                return;
            }

            let free_resizable_memory = || {
                unsafe {
                    assert!(!self.custom_deleter());
                    assert!(self.is_resizable_by_js() || self.is_wasm_memory());
                    let region = GetReservedRegion(self.has_guard_regions(), self.is_wasm_memory64(),
                                                    self.buffer_start_, self.byte_capacity_);
                    if !region.is_empty() {
                        if self.page_allocator_.is_null(){
                            println!("page_allocator_ is NULL at drop");
                            return;
                        }
                        if self.buffer_start_.is_null(){
                            println!("buffer_start is NULL at drop");
                            return;
                        }
                        FreePages(self.page_allocator_, region.begin() as *mut std::ffi::c_void, region.size());
                    }
                }
            };

            if self.is_wasm_memory() {
                unsafe {
                    assert!(!self.is_resizable_by_js());
                    let reservation_size = GetReservationSize(self.has_guard_regions(), self.byte_capacity_, self.is_wasm_memory64());
                    //println!(
                    //    "BSw:free  bs={:p} mem={:p} (length={}, capacity={}, reservation={})",
                    //    self, self.buffer_start_, self.byte_length(), self.byte_capacity_, reservation_size
                    //);
                    if self.is_shared() {
                        let shared_data = self.get_shared_wasm_memory_data();
                        drop(shared_data);
                    }
                    free_resizable_memory();
                    return;
                }
            }

            if self.is_resizable_by_js() {
                free_resizable_memory();
                return;
            }

            if self.custom_deleter() {
                unsafe {
                    //println!(
                    //    "BS:custom deleter bs={:p} mem={:p} (length={}, capacity={})",
                    //    self, self.buffer_start_, self.byte_length(), self.byte_capacity_
                    //);
                    (self.type_specific_data_.deleter.callback)(self.buffer_start_, self.byte_length() as usize,
                                                                self.type_specific_data_.deleter.data);
                    return;
                }
            }

            // JSArrayBuffer backing store. Deallocate through the embedder's allocator.
            unsafe {
                let allocator = self.get_v8_api_array_buffer_allocator();
                //println!(
                //    "BS:free   bs={:p} mem={:p} (length={}, capacity={})",
                //    self, self.buffer_start_, self.byte_length(), self.byte_capacity_
                //);
                allocator.Free(self.buffer_start_, self.byte_length() as usize);
            }
        }
    }

    impl BackingStore {
        pub fn new(page_allocator: *mut PageAllocator, buffer_start: *mut std::ffi::c_void,
               byte_length: usize, max_byte_length: usize, byte_capacity: usize,
               shared: SharedFlag, resizable: ResizableFlag, is_wasm_memory: bool,
               is_wasm_memory64: bool, has_guard_regions: bool,
               custom_deleter: bool, empty_deleter: bool) -> Self {
            BackingStore {
                buffer_start_: buffer_start,
                byte_length_: AtomicUsize::new(byte_length),
                max_byte_length_: max_byte_length,
                byte_capacity_: byte_capacity,
                //TODO: Implement atomic type here
                id_: NEXT_BACKING_STORE_ID.fetch_add(1, Ordering::Relaxed),
                page_allocator_: page_allocator,
                type_specific_data_: TypeSpecificData::new(),
                flags_: AtomicEnumSet::new(
                    shared == SharedFlag::kShared,
                    resizable == ResizableFlag::kResizable,
                    is_wasm_memory,
                    is_wasm_memory64,
                    has_guard_regions,
                    custom_deleter,
                    empty_deleter,
                ),
            }
        }
        // Allocate an array buffer backing store using the default method,
        // which currently is the embedder-provided array buffer allocator.
        pub fn Allocate(isolate: *mut Isolate,
                        byte_length: usize,
                        shared: SharedFlag,
                        initialized: InitializedFlag) -> Result<Box<BackingStore>, String> {
            unsafe {
                let allocator = (*isolate).array_buffer_allocator();
                if allocator.is_null() {
                    return Err("Allocator is null".to_string());
                }

                if byte_length > (*allocator).MaxAllocationSize() {
                    return Err("Byte length exceeds maximum allocation size".to_string());
                }

                let mut buffer_start: *mut std::ffi::c_void = null_mut();
                if byte_length > 0 {
                    let counters = (*isolate).counters();
                    let mb_length = (byte_length / MB) as i32;
                    if mb_length > 0 {
                        (*counters).array_buffer_big_allocations().AddSample(mb_length);
                    }
                    if shared == SharedFlag::kShared {
                        (*counters).shared_array_allocations().AddSample(mb_length);
                    }

                    let allocate_buffer = |byte_length: usize| -> *mut std::ffi::c_void {
                        if initialized == InitializedFlag::kUninitialized {
                            (*allocator).AllocateUninitialized(byte_length)
                        } else {
                            (*allocator).Allocate(byte_length)
                        }
                    };

                     buffer_start = (*(*isolate).heap()).AllocateExternalBackingStore(
                        allocate_buffer, byte_length);

                    if buffer_start.is_null() {
                        (*counters).array_buffer_new_size_failures().AddSample(mb_length);
                        return Err("Allocation failed".to_string());
                    }
                }

                let page_allocator = (*(*isolate).isolate_group()).GetBackingStorePageAllocator();
                let result = Box::new(BackingStore {
                        buffer_start_: buffer_start,
                        byte_length_: AtomicUsize::new(byte_length),
                        max_byte_length_: byte_length,
                        byte_capacity_: byte_length,
                        id_: NEXT_BACKING_STORE_ID.fetch_add(1, Ordering::Relaxed),
                        page_allocator_: page_allocator,
                        type_specific_data_: TypeSpecificData::new(),
                        flags_: AtomicEnumSet::new(
                            shared == SharedFlag::kShared,
                            false,
                            false,
                            false,
                            false,
                            false,
                            false,
                            ),
                    });

                result.SetAllocatorFromIsolate(isolate);
                //println!(
                //    "BS:alloc  bs={:p} mem={:p} (length={})",
                //    &result, result.buffer_start(), byte_length
                //);
                Ok(result)
            }
        }

        #[cfg(V8_ENABLE_WEBASSEMBLY)]
        pub fn AllocateWasmMemory(
            isolate: *mut Isolate,
            initial_pages: usize,
            maximum_pages: usize,
            wasm_memory: WasmMemoryFlag,
            shared: SharedFlag,
        ) -> Result<Box<BackingStore>, String> {
            // Wasm pages must be a multiple of the allocation page size.
            assert_eq!(0, wasm_constants::kWasmPageSize % AllocatePageSize());
            assert!(initial_pages <= maximum_pages);
            assert!(maximum_pages
                <= match wasm_memory {
                    WasmMemoryFlag::kWasmMemory32 => wasm_constants::kV8MaxWasmMemory32Pages,
                    WasmMemoryFlag::kWasmMemory64 => wasm_constants::kV8MaxWasmMemory64Pages,
                    _ => 0,
                } as usize);

            assert!(wasm_memory == WasmMemoryFlag::kWasmMemory32 || wasm_memory == WasmMemoryFlag::kWasmMemory64);

            let is_wasm_memory64 = wasm_memory == WasmMemoryFlag::kWasmMemory64;
            let has_guard_regions = unsafe {
                trap_handler::IsTrapHandlerEnabled()
                    && (wasm_memory == WasmMemoryFlag::kWasmMemory32
                        || (is_wasm_memory64 && v8::flags::wasm_memory64_trap_handling))
            };

            let TryAllocate = |isolate: *mut Isolate, maximum_pages: usize, wasm_memory: WasmMemoryFlag, shared: SharedFlag, has_guard_regions: bool| -> Result<Box<BackingStore>, String> {
                let result = Self::TryAllocateAndPartiallyCommitMemory(
                    isolate,
                    initial_pages * wasm_constants::kWasmPageSize,
                    maximum_pages * wasm_constants::kWasmPageSize,
                    wasm_constants::kWasmPageSize,
                    initial_pages,
                    maximum_pages,
                    wasm_memory,
                    shared,
                    has_guard_regions,
                );
                match result {
                    Ok(mut r) => {
                        if shared == SharedFlag::kShared {
                           unsafe { (*r).type_specific_data_.shared_wasm_memory_data =
                                Box::into_raw(Box::new(SharedWasmMemoryData { isolates_: Vec::new() })) };
                        }
                        Ok(r)
                    }
                    Err(e) => Err(e),
                }
            };
            let backing_store = TryAllocate(isolate, maximum_pages, wasm_memory, shared, has_guard_regions);

            match backing_store {
                Ok(_) => {
                    return backing_store;
                }
                Err(_) => {
                   println!("Failed backing_store TryAllocate");
                }
            }


            if !has_guard_regions && maximum_pages - initial_pages >= 4 {
                let mut delta = (maximum_pages - initial_pages) / 4;
                let sizes: [usize; 4] = [
                    maximum_pages - delta,
                    maximum_pages - 2 * delta,
                    maximum_pages - 3 * delta,
                    initial_pages,
                ];

                for reduced_maximum_pages in sizes.iter() {
                   let backing_store = TryAllocate(isolate, *reduced_maximum_pages, wasm_memory, shared, has_guard_regions);
                    match backing_store {
                        Ok(_) => {
                            return backing_store;
                        }
                        Err(_) => {
                           println!("Failed backing_store TryAllocate reduced pages");
                        }
                    }
                }
            }
            Err("Failed to allocate wasm memory".to_string())
        }

        // Tries to allocate `maximum_pages` of memory and commit `initial_pages`.
        //
        // If {isolate} is not null, initial failure to allocate the backing store may
        // trigger GC, after which the allocation is retried. If {isolate} is null, no
        // GC will be triggered.
        pub fn TryAllocateAndPartiallyCommitMemory(
            isolate: *mut Isolate,
            byte_length: usize,
            max_byte_length: usize,
            page_size: usize,
            initial_pages: usize,
            maximum_pages: usize,
            wasm_memory: WasmMemoryFlag,
            shared: SharedFlag,
            has_guard_regions: bool,
        ) -> Result<Box<BackingStore>, String> {
            // Enforce engine limitation on the maximum number of pages.
            if maximum_pages > usize::MAX / page_size {
                return Err("Maximum pages exceeds limit".to_string());
            }

            // Cannot reserve 0 pages on some OSes.
            let mut maximum_pages = maximum_pages;
            if maximum_pages == 0 {
                maximum_pages = 1;
            }

            let is_wasm_memory = wasm_memory != WasmMemoryFlag::kNotWasm;
            let is_wasm_memory64 = wasm_memory == WasmMemoryFlag::kWasmMemory64;
            assert!(!(has_guard_regions && !is_wasm_memory));

            // For accounting purposes, whether a GC was necessary.
            let mut did_retry = false;

            // A helper to try running a function up to 3 times, executing a GC
            // if the first and second attempts failed.
            let mut gc_retry = |mut fn_: impl FnMut() -> bool| -> bool {
                for _i in 0..3 {
                    if fn_() {
                        return true;
                    }
                    // Collect garbage and retry.
                    did_retry = true;
                    if !isolate.is_null() {
                        unsafe {
                            (*isolate).heap().MemoryPressureNotification(
                                MemoryPressureLevel::kCritical,
                                true,
                            );
                        }
                    }
                }
                false
            };

            let byte_capacity = maximum_pages * page_size;
            let reservation_size = GetReservationSize(has_guard_regions, byte_capacity, is_wasm_memory64);

            //--------------------------------------------------------------------------
            // Allocate pages (inaccessible by default).
            //--------------------------------------------------------------------------
            let mut allocation_base: *mut std::ffi::c_void = null_mut();

            let page_allocator = unsafe {
                if !isolate.is_null(){
                 (*(*isolate).isolate_group()).GetBackingStorePageAllocator()
                }
                else{
                     GetArrayBufferPageAllocator()
                }
            };
            let allocate_pages = || -> bool {
                unsafe {
                    allocation_base = AllocatePages(
                       page_allocator,
                        null_mut(),
                        reservation_size,
                        page_size,
                        PageAllocator::kNoAccess,
                    );
                }

                !allocation_base.is_null()
            };
            if !gc_retry(allocate_pages) {
                // Page allocator could not reserve enough pages.
                unsafe {
                    if !isolate.is_null() {
                        RecordStatus(isolate, AllocationStatus::kOtherFailure);
                    }
                }
                //println!("TryAllocateAndPartiallyCommitMemory failed to allocate pages");
                return Err("Failed to allocate pages".to_string());
            }

            let mut buffer_start = allocation_base as *mut u8;

            //--------------------------------------------------------------------------
            // Commit the initial pages (allow read/write).
            //--------------------------------------------------------------------------
            let committed_byte_length = initial_pages * page_size;
            let commit_memory = || -> bool {
                if committed_byte_length == 0 {
                    return true;
                }
                unsafe {
                    SetPermissions(
                       page_allocator,
                        buffer_start as *mut std::ffi::c_void,
                        committed_byte_length,
                        PageAllocator::kReadWrite,
                    )
                }
            };
            if !gc_retry(commit_memory) {
               // println!("TryAllocateAndPartiallyCommitMemory failed to set permissions");
                unsafe {
                    FreePages(page_allocator, allocation_base, reservation_size);
                }

                return Err("Failed to set permissions".to_string());
            }

            unsafe {
                if !isolate.is_null() {
                    RecordStatus(
                        isolate,
                        if did_retry {
                            AllocationStatus::kSuccessAfterRetry
                        } else {
                            AllocationStatus::kSuccess
                        },
                    );
                }
            }

            let resizable = if is_wasm_memory {
                ResizableFlag::kNotResizable
            } else {
                ResizableFlag::kResizable
            };

            let result = Box::new(BackingStore {
                buffer_start_: buffer_start as *mut std::ffi::c_void,
                byte_length_: AtomicUsize::new(byte_length),
                max_byte_length_: max_byte_length,
                byte_capacity_: byte_capacity,
                id_: NEXT_BACKING_STORE_ID.fetch_add(1, Ordering::Relaxed),
                page_allocator_: page_allocator,
                type_specific_data_: TypeSpecificData::new(),
                flags_: AtomicEnumSet::new(
                    shared == SharedFlag::kShared,
                    resizable == ResizableFlag::kResizable,
                    is_wasm_memory,
                    is_wasm_memory64,
                    has_guard_regions,
                    false,
                    false,
                    ),
            });
            //println!(
            //    "BSw:alloc bs={:p} mem={:p} (length={}, capacity={}, reservation={})",
            //    &result, result.buffer_start(), byte_length, byte_capacity, reservation_size
            //);

            Ok(result)
        }

        // Create a backing store that wraps existing allocated memory.
        pub fn WrapAllocation(
            allocation_base: *mut std::ffi::c_void,
            allocation_length: usize,
            deleter: unsafe extern "C" fn(data: *mut std::ffi::c_void, byte_length: usize, deleter_data: *mut std::ffi::c_void),
            deleter_data: *mut std::ffi::c_void,
            shared: SharedFlag,
        ) -> Result<Box<BackingStore>, String> {
            let is_empty_deleter =
                deleter as usize == v8::ArrayBuffer::EmptyDeleter as usize;
            let result = Box::new(BackingStore {
                buffer_start_: allocation_base,
                byte_length_: AtomicUsize::new(allocation_length),
                max_byte_length_: allocation_length,
                byte_capacity_: allocation_length,
                id_: NEXT_BACKING_STORE_ID.fetch_add(1, Ordering::Relaxed),
                page_allocator_: null_mut(),
                type_specific_data_: TypeSpecificData::CustomDeleter {
                    callback: deleter,
                    data: deleter_data,
                },
                flags_: AtomicEnumSet::new(
                    shared == SharedFlag::kShared,
                    false,
                    false,
                    false,
                    false,
                    true,
                    is_empty_deleter,
                ),
            });
            //println!(
            //    "BS:wrap   bs={:p} mem={:p} (length={})",
            //    &result, result.buffer_start(), result.byte_length()
            //);
            Ok(result)
        }

        // Create an empty backing store.
        pub fn EmptyBackingStore(shared: SharedFlag) -> Box<BackingStore> {
            Box::new(BackingStore {
                buffer_start_: null_mut(),
                byte_length_: AtomicUsize::new(0),
                max_byte_length_: 0,
                byte_capacity_: 0,
                id_: NEXT_BACKING_STORE_ID.fetch_add(1, Ordering::Relaxed),
                page_allocator_: null_mut(),
                type_specific_data_: TypeSpecificData::new(),
                flags_: AtomicEnumSet::new(
                    shared == SharedFlag::kShared,
                    false,
                    false,
                    false,
                    false,
                    false,
                    false,
                ),
            })
        }

        // Accessors.
        // Internally, we treat nullptr as the empty buffer value. However,
        // externally, we should use the EmptyBackingStoreBuffer() constant for that
        // purpose as the buffer pointer should always point into the sandbox. As
        // such, this is the place where we convert between these two.
        pub fn buffer_start(&self) -> *mut std::ffi::c_void {
            if !self.buffer_start_.is_null() {
                self.buffer_start_
            } else {
                unsafe { EmptyBackingStoreBuffer() }
            }
        }
        pub fn byte_length(&self, memory_order: Ordering) -> usize {
            self.byte_length_.load(memory_order)
        }
        pub fn max_byte_length(&self) -> usize {
            self.max_byte_length_
        }
        pub fn byte_capacity(&self) -> usize {
            self.byte_capacity_
        }
        pub fn is_shared(&self) -> bool {
            self.has_flag(Flag::kIsShared)
        }
        pub fn is_resizable_by_js(&self) -> bool {
            self.has_flag(Flag::kIsResizableByJs)
        }
        pub fn is_wasm_memory(&self) -> bool {
            self.has_flag(Flag::kIsWasmMemory)
        }
        pub fn is_wasm_memory64(&self) -> bool {
            self.has_flag(Flag::kIsWasmMemory64)
        }
        pub fn has_guard_regions(&self) -> bool {
            self.has_flag(Flag::kHasGuardRegions)
        }

        pub fn IsEmpty(&self) -> bool {
            assert!(self.byte_capacity_ >= self.byte_length(Ordering::Relaxed));
            self.byte_capacity_ == 0
        }

        pub fn ResizeInPlace(&mut self, isolate: *mut Isolate, new_byte_length: usize) -> ResizeOrGrowResult {
           unsafe{
            let page_size = AllocatePageSize();
            let mut new_committed_pages = 0;

            let round_return_value = RoundUpToPageSize(
                new_byte_length,
                page_size,
                crate::JSArrayBuffer::kMaxByteLength,
                &mut new_committed_pages,
            );
            assert!(round_return_value);

            let new_committed_length = new_committed_pages * page_size;
            assert!(new_byte_length <= new_committed_length);
            assert!(!self.is_shared());

            if new_byte_length < self.byte_length(Ordering::Relaxed) {
                // Zero the memory so that in case the buffer is grown later, we have
                // zeroed the contents already. This is especially needed for the portion of
                // the memory we're not going to decommit below (since it belongs to a
                // committed page). In addition, we don't rely on all platforms always
                // zeroing decommitted-then-recommitted memory, but zero the memory
                // explicitly here.
                let base_ptr = self.buffer_start_ as *mut u8;
                let start_ptr = base_ptr.add(new_byte_length);
                let len = self.byte_length(Ordering::Relaxed) - new_byte_length;
                std::ptr::write_bytes(start_ptr, 0, len);

                // Check if we can un-commit some pages.
                let mut old_committed_pages = 0;
                let round_return_value = RoundUpToPageSize(
                    self.byte_length(Ordering::Relaxed),
                    page_size,
                    crate::JSArrayBuffer::kMaxByteLength,
                    &mut old_committed_pages,
                );
                assert!(round_return_value);
                assert!(new_committed_pages <= old_committed_pages);

                if new_committed_pages < old_committed_pages {
                    let old_committed_length = old_committed_pages * page_size;
                    if !SetPermissions(
                        self.page_allocator_,
                        (self.buffer_start_ as *mut u8).add(new_committed_length) as *mut std::ffi::c_void,
                        old_committed_length - new_committed_length,
                        PageAllocator::kNoAccess,
                    ) {
                        return ResizeOrGrowResult::kFailure;
                    }
                }

                // Changing the byte length wouldn't strictly speaking be needed, since
                // the JSArrayBuffer already stores the updated length. This is to keep
                // the BackingStore and JSArrayBuffer in sync.
                self.byte_length_.store(new_byte_length, Ordering::SeqCst);
                return ResizeOrGrowResult::kSuccess;
            }
            if new_byte_length == self.byte_length(Ordering::Relaxed) {
                // i::SetPermissions with size 0 fails on some platforms, so special
                // handling for the case byte_length_ == new_byte_length == 0 is
                // required.
                return ResizeOrGrowResult::kSuccess;
            }

            // Try to adjust the permissions on the memory.
            if !SetPermissions(
               self.page_allocator_,
                self.buffer_start_,
                new_committed_length,
                PageAllocator::kReadWrite,
            ) {
                return ResizeOrGrowResult::kFailure;
            }

            self.byte_length_.store(new_byte_length, Ordering::SeqCst);
            ResizeOrGrowResult::kSuccess
           }
        }

        // Commit already reserved memory (for GSAB backing stores (shared)).
        pub fn GrowInPlace(&mut self, isolate: *mut Isolate, new_byte_length: usize) -> ResizeOrGrowResult {
            unsafe{
            let page_size = AllocatePageSize();
            let mut new_committed_pages = 0;
            let round_return_value = RoundUpToPageSize(
                new_byte_length,
                page_size,
                crate::JSArrayBuffer::kMaxByteLength,
                &mut new_committed_pages,
            );
            assert!(round_return_value);

            let new_committed_length = new_committed_pages * page_size;
            assert!(new_byte_length <= new_committed_length);
            assert!(self.is_shared());
            // See comment in GrowWasmMemoryInPlace.
            // GrowableSharedArrayBuffer.prototype.grow can be called from several
            // threads. If two threads try to grow() in a racy way, the spec allows the
            // larger grow to throw also if the smaller grow succeeds first. The
            // implementation below doesn't throw in that case - instead, it retries and
            // succeeds. If the larger grow finishes first though, the smaller grow must
            // throw.
            let mut old_byte_length = self.byte_length_.load(Ordering::SeqCst);
            loop {
                if new_byte_length < old_byte_length {
                    // The caller checks for the new_byte_length < old_byte_length_ case. This
                    // can only happen if another thread grew the memory after that.
                    return ResizeOrGrowResult::kRace;
                }
                if new_byte_length == old_byte_length {
                    // i::SetPermissions with size 0 fails on some platforms, so special
                    // handling for the case old_byte_length == new_byte_length == 0 is
                    // required.
                    return ResizeOrGrowResult::kSuccess;
                }

                // Try to adjust the permissions on the memory.
                if !SetPermissions(
                    self.page_allocator_,
                    self.buffer_start_,
                    new_committed_length,
                    PageAllocator::kReadWrite,
                ) {
                    return ResizeOrGrowResult::kFailure;
                }

                // compare_exchange_weak updates old_byte_length.
                match self.byte_length_.compare_exchange_weak(
                    old_byte_length,
                    new_byte_length,
                    Ordering::SeqCst,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => {
                        // Successfully updated both the length and permissions.
                        break;
                    }
                    Err(value) => {
                        old_byte_length = value;
                    }
                }
            }
            ResizeOrGrowResult::kSuccess
            }
        }

        #[cfg(V8_ENABLE_WEBASSEMBLY)]
        pub fn CopyWasmMemory(
            &self,
            isolate: *mut Isolate,
            new_pages: usize,
            max_pages: usize,
            wasm_memory: WasmMemoryFlag,
        ) -> Result<Box<BackingStore>, String> {
            let new_backing_store = BackingStore::AllocateWasmMemory(
                isolate,
                new_pages,
                max_pages,
                wasm_memory,
                if self.is_shared() {
                    SharedFlag::kShared
                } else {
                    SharedFlag::kNotShared
                },
            );

            match new_backing_store {
                Ok(mut new_backing_store) => {
                   unsafe{
                    if new_backing_store.has_guard_regions() != self.has_guard_regions() {
                        return Err("Guard region mismatch".to_string());
                    }

                    if self.byte_length(Ordering::Relaxed) > 0 {
                        assert!(new_pages * wasm_constants::kWasmPageSize >= self.byte_length(Ordering::Relaxed));
                        let src_ptr = self.buffer_start_ as *const u8;
                        let dst_ptr = new_backing_store.buffer_start() as *mut u8;
                        std::ptr::copy_nonoverlapping(src_ptr, dst_ptr, self.byte_length(Ordering::Relaxed));
                    }

                    Ok(new_backing_store)
                   }
                }
                Err(msg) => {
                    return Err(msg);
                }
            }
        }

        #[cfg(V8_ENABLE_WEBASSEMBLY)]
        pub fn GrowWasmMemoryInPlace(
            &self,
            isolate: *mut Isolate,
            delta_pages: usize,
            max_pages: usize,
        ) -> Result<usize, String> {
           unsafe {
                assert!(self.is_wasm_memory());
                let mut max_
