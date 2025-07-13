// Converted from V8 C++ source files:
// Header: code-range.h
// Implementation: code-range.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod base;

use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard, atomic::{AtomicPtr, Ordering}};
use crate::VirtualMemoryCage;
use crate::Address;

pub struct CodeRangeAddressHint {
    mutex_: Mutex<()>,
    recently_freed_: Mutex<HashMap<usize, Vec<Address>>>,
}

impl CodeRangeAddressHint {
    pub fn new() -> Self {
        CodeRangeAddressHint {
            mutex_: Mutex::new(()),
            recently_freed_: Mutex::new(HashMap::new()),
        }
    }

    pub fn get_address_hint(&self, code_range_size: usize, alignment: usize) -> Address {
        let _guard = self.mutex_.lock().unwrap();

        let mut recently_freed = self.recently_freed_.lock().unwrap();

        if let Some(addresses) = recently_freed.get_mut(&code_range_size) {
            if !addresses.is_empty() {
                let result = addresses.pop().unwrap();
                if result % alignment as u64 != 0 {
                    return result;
                }
                return result;
            }
        }
        0
    }

    pub fn notify_freed_code_range(&self, code_range_start: Address, code_range_size: usize) {
        let _guard = self.mutex_.lock().unwrap();
        let mut recently_freed = self.recently_freed_.lock().unwrap();
        recently_freed.entry(code_range_size).or_insert(Vec::new()).push(code_range_start);
    }
}

impl Default for CodeRangeAddressHint {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct CodeRange {
    virtual_memory_cage: VirtualMemoryCage,
    embedded_blob_code_copy_: AtomicPtr<u8>,
    remap_embedded_builtins_mutex_: Mutex<()>,
}

impl CodeRange {
    pub fn new() -> Self {
        CodeRange {
            virtual_memory_cage: VirtualMemoryCage::new(),
            embedded_blob_code_copy_: AtomicPtr::new(std::ptr::null_mut()),
            remap_embedded_builtins_mutex_: Mutex::new(()),
        }
    }

    pub fn get_writable_reserved_area_size() -> usize {
        kReservedCodeRangePages * MutablePageMetadata::kPageSize as usize
    }

    pub fn embedded_blob_code_copy(&self) -> *mut u8 {
        self.embedded_blob_code_copy_.load(Ordering::Acquire)
    }

    pub fn init_reservation(&mut self, page_allocator: &mut v8::PageAllocator, requested: usize, immutable: bool) -> bool {
        if requested == 0 {
            return false;
        }

        let k_page_size = MutablePageMetadata::kPageSize as usize;
        if k_page_size % page_allocator.allocate_page_size() != 0 {
            return false;
        }

        let base_alignment = if V8_EXTERNAL_CODE_SPACE_BOOL {
            requested.next_power_of_two()
        } else {
            k_page_size
        };

        if kPlatformRequiresCodeRange && requested > kMaximalCodeRangeSize {
            return false;
        }

        let mut params = VirtualMemoryCage::ReservationParams {
            page_allocator,
            reservation_size: requested,
            page_size: k_page_size,
            permissions: if v8_flags.jitless {
                base::PageAllocator::Permission::kNoAccess
            } else {
                base::PageAllocator::Permission::kNoAccessWillJitLater
            },
            page_initialization_mode: if v8_flags.jitless {
                base::PageInitializationMode::kAllocatedPagesCanBeUninitialized
            } else {
                base::PageInitializationMode::kRecommitOnly
            },
            page_freeing_mode: if v8_flags.jitless {
                base::PageFreeingMode::kMakeInaccessible
            } else {
                base::PageFreeingMode::kDiscard
            },
            base_alignment: 0,
            requested_start_hint: 0
        };
    
        let preferred_region = Self::get_preferred_region(kMaxPCRelativeCodeRangeInMB as usize, k_page_size);

        let address_hint = GetCodeRangeAddressHint().get_address_hint(requested, page_allocator.allocate_page_size());

        params.base_alignment = base_alignment;
        params.requested_start_hint = address_hint;

        if !self.virtual_memory_cage.init_reservation(params) {
            params.requested_start_hint = 0;
            if !self.virtual_memory_cage.init_reservation(params) {
                return false;
            }
        }

        let reserved_area = Self::get_writable_reserved_area_size();
        if reserved_area > 0 {
            if reserved_area > k_page_size {
                return false;
            }
            
        }
        true
    }

    pub fn free(&mut self) {
        if self.virtual_memory_cage.is_reserved() {
            GetCodeRangeAddressHint().notify_freed_code_range(
                self.virtual_memory_cage.reservation().region().begin(),
                self.virtual_memory_cage.reservation().region().size(),
            );
            self.virtual_memory_cage.free();
        }
    }

    pub fn remap_embedded_builtins(
        &self,
        isolate: *mut Isolate,
        embedded_blob_code: *const u8,
        embedded_blob_code_size: usize,
    ) -> *mut u8 {
        let _guard = self.remap_embedded_builtins_mutex_.lock().unwrap();

        let page_allocator = self.virtual_memory_cage.page_allocator();
        let code_region = base::AddressRegion {
            begin: page_allocator.begin(),
            size: page_allocator.size(),
        };

        let embedded_blob_code_copy =
            self.embedded_blob_code_copy_.load(Ordering::Acquire);

        if !embedded_blob_code_copy.is_null() {
            if !code_region.contains(
                embedded_blob_code_copy as Address,
                embedded_blob_code_size,
            ) {
                unsafe {
                    V8::FatalProcessOutOfMemory(
                        isolate,
                        "Embedded blob code copy is not within code region",
                    );
                }
            }
            return embedded_blob_code_copy;
        }
        let k_allocate_page_size = page_allocator.allocate_page_size();
        let allocate_code_size =
            (embedded_blob_code_size + k_allocate_page_size - 1) / k_allocate_page_size
                * k_allocate_page_size;
        let max_pc_relative_code_range = kMaxPCRelativeCodeRangeInMB as usize * MB;
        let hint_offset = std::cmp::min(max_pc_relative_code_range, code_region.size)
            - allocate_code_size;
        let hint = code_region.begin + hint_offset;
        unsafe {
            let embedded_blob_code_copy = page_allocator.allocate_pages(
                hint as *mut std::ffi::c_void,
                allocate_code_size,
                k_allocate_page_size,
                PageAllocator::kNoAccessWillJitLater,
            ) as *mut u8;
            if embedded_blob_code_copy.is_null() {
                V8::FatalProcessOutOfMemory(
                    isolate,
                    "Can't allocate space for re-embedded builtins",
                );
            }

            self.embedded_blob_code_copy_
                .store(embedded_blob_code_copy, Ordering::Release);
            embedded_blob_code_copy
        }
    }

    fn get_preferred_region(radius_in_megabytes: usize, allocate_page_size: usize) -> base::AddressRegion {
        base::AddressRegion { begin: 0, size: 0 }
    }
}

impl Default for CodeRange {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for CodeRange {
    fn drop(&mut self) {
        self.free();
    }
}

const kReservedCodeRangePages: usize = 1;

struct MutablePageMetadata {}
impl MutablePageMetadata {
    const kPageSize: u32 = 4096; 
}

pub mod v8 {
    pub struct PageAllocator {
    }

    impl PageAllocator {
        pub fn allocate_page_size(&mut self) -> usize {
            4096
        }
    
        pub fn commit_page_size(&mut self) -> usize {
            4096
        }

        pub fn begin(&self) -> u64{
            0
        }

        pub fn size(&self) -> usize{
            0
        }

        pub unsafe fn allocate_pages(&mut self, hint: *mut std::ffi::c_void, size: usize, alignment: usize, permissions: base::PageAllocator::Permission) -> *mut std::ffi::c_void{
            hint
        }

         pub unsafe fn set_permissions(&mut self, address: *mut u8, size: usize, permissions: base::PageAllocator::Permission) -> bool{
            true
         }

         pub unsafe fn recommit_pages(&mut self, address: *mut u8, size: usize, permissions: base::PageAllocator::Permission) -> bool{
            true
         }

          pub unsafe fn allocate_pages_at(&mut self, address: u64, size: usize, permissions: base::PageAllocator::Permission) -> bool{
            true
         }
    }
}

const V8_EXTERNAL_CODE_SPACE_BOOL: bool = false;
const COMPRESS_POINTERS_IN_SHARED_CAGE_BOOL: bool = false;
const kPlatformRequiresCodeRange: bool = false;
const kMinimumCodeRangeSize: usize = 16 * 1024 * 1024;
const kMaximalCodeRangeSize: usize = 2 * 1024 * 1024 * 1024;
const kMaxPCRelativeCodeRangeInMB: u32 = 2048;
const V8_ENABLE_NEAR_CODE_RANGE_BOOL: bool = false;
const MB: usize = 1024 * 1024;
const GB: usize = 1024 * 1024 * 1024;
const kPtrComprCageReservationSize: usize = 4 * 1024 * 1024 * 1024;

struct V8 {}
impl V8 {
     unsafe fn FatalProcessOutOfMemory(_isolate: *mut Isolate, _message: &str) -> ! {
        panic!("{}", _message);
    }
}
struct Isolate {}

struct base {}
impl base {
    struct AddressRegion {
        begin: u64,
        size: usize,
    }
}

impl base::AddressRegion {
    fn contains(&self, address: Address, size: usize) -> bool {
        address >= self.begin && address + size as u64 <= self.begin + self.size as u64
    }

    fn is_empty(&self) -> bool{
        true
    }
}

impl VirtualMemoryCage {
    fn page_allocator(&self) -> &VirtualMemoryCage{
        self
    }

    fn reservation(&self) -> &VirtualMemoryCage{
        self
    }
    fn init_reservation(&mut self, params: VirtualMemoryCage::ReservationParams) -> bool{
        true
    }

    fn is_reserved(&self) -> bool{
        false
    }
}

fn GetCodeRangeAddressHint() -> &'static CodeRangeAddressHint {
    use std::sync::Once;
    use std::mem::MaybeUninit;

    static mut HINT: MaybeUninit<CodeRangeAddressHint> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();

    ONCE.call_once(|| {
        unsafe {
            HINT.as_mut_ptr().write(CodeRangeAddressHint::new());
        }
    });

    unsafe { &*HINT.as_ptr() }
}

pub mod flags {
    pub static jitless: bool = false;
    pub static trace_code_range_allocation: bool = false;
    pub static abort_on_far_code_range: bool = false;
    pub static better_code_range_allocation: bool = false;
}
mod ThreadIsolation {
    pub fn Enabled() -> bool{
        false
    }
}

mod PageAllocator {
     #[allow(dead_code)]
    #[derive(Debug, Copy, Clone)]
    pub enum Permission {
        kNoAccess,
        kReadWrite,
        kReadExecute,
        kReadWriteExecute,
        kNoAccessWillJitLater,
    }
}

mod base_compiler{
    pub enum BranchHint {
        kNone,
    }
}

mod base_codegen{
     #[derive(PartialEq, Eq, Debug, Copy, Clone)]
    pub enum AllocationType{
        kOld,
        kYoung
    }
}

fn FUNCTION_ADDR<T>(_func: T) -> u64{
    0
}

mod win64_unwindinfo {
    pub fn CanRegisterUnwindInfoForNonABICompliantCodeRange() -> bool {
        false
    }

    pub fn RegisterNonABICompliantCodeRange(_address: *mut std::ffi::c_void, _size: usize) {}

     pub fn UnregisterNonABICompliantCodeRange(_address: *mut std::ffi::c_void) {}
}

pub fn GetPlatformPageAllocator() -> &'static mut v8::PageAllocator{
    use std::sync::Once;
    use std::mem::MaybeUninit;

    static mut ALLOCATOR: MaybeUninit<v8::PageAllocator> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();

    ONCE.call_once(|| {
        unsafe {
            ALLOCATOR.as_mut_ptr().write(v8::PageAllocator{});
        }
    });

    unsafe { &mut *ALLOCATOR.as_mut_ptr() }
}

mod RwxMemoryWriteScope {
    pub struct RwxMemoryWriteScope<'a>(&'a str);

    impl<'a> RwxMemoryWriteScope<'a> {
        pub fn new(_s: &'a str) -> Self {
            RwxMemoryWriteScope(_s)
        }
    }
}

mod DiscardSealedMemoryScope{
    pub struct DiscardSealedMemoryScope<'a>(&'a str);

    impl<'a> DiscardSealedMemoryScope<'a> {
        pub fn new(_s: &'a str) -> Self {
            DiscardSealedMemoryScope(_s)
        }
    }
}
