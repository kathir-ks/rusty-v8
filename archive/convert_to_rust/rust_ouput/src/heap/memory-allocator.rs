// Converted from V8 C++ source files:
// Header: memory-allocator.h
// Implementation: memory-allocator.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::sync::{Mutex, Arc, atomic::{AtomicUsize, AtomicPtr, Ordering}};
use std::collections::{HashSet, BTreeSet};
use std::hash::{Hash, Hasher};
use std::ptr;
use std::mem::size_of;

use crate::heap::code_range::VirtualMemoryCage;
use crate::sandbox::sandbox::Sandbox;
use crate::heap::spaces::Space;
use crate::heap::spaces::AllocationSpace;
use crate::heap::{MemoryChunk, PageMetadata, LargePageMetadata, ReadOnlyPageMetadata, Heap, BaseSpace, MemoryChunkMetadata};
use crate::heap::mutable_page_metadata::MutablePageMetadata;
use crate::heap::concurrent_marking::Address;
use crate::heap::local_heap::AllocationSpace::RO_SPACE;
use crate::heap::safepoint::IsolateSafepoint;
use crate::init::isolate_group::Isolate;

struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<'a, T> MutexGuard<'a, T> {
    fn new(mutex: &'a Mutex<T>) -> Self {
        MutexGuard { mutex }
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        // Releasing the lock is handled by the MutexGuard's Drop implementation.
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Executability {
  NOT_EXECUTABLE,
  EXECUTABLE
}

#[derive(Debug)]
pub struct VirtualMemory {
    address: usize,
    size: usize,
    executable: Executability
}

impl VirtualMemory {
    pub fn new(address: usize, size: usize, executable: Executability) -> Self {
        VirtualMemory { address, size, executable }
    }

    pub fn address(&self) -> Address {
        self.address as Address
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn set_permissions(&self, _address: Address, _size: usize, _permission: i32) -> bool {
        true
    }

    pub fn is_reserved(&self) -> bool {
        true
    }

    pub fn free(&mut self) {}
    pub fn recommit_pages(&self, start: Address, chunk_size: usize, kReadWriteExecute: i32) -> bool {
        true
    }

    pub fn release(&self, start_free: Address) -> usize {
        self.size
    }
}

pub struct PageAllocator {}

impl PageAllocator {
    pub fn allocate_page_size(&self) -> usize {
        4096
    }
}

pub struct HeapAllocator {}

const kZapValue: i32 = 0xDE;

fn heap_should_zap_garbage() -> bool {
    true
}

pub struct CodePageMemoryModificationScopeForDebugging {}
impl CodePageMemoryModificationScopeForDebugging {
    pub fn new(_heap: &Heap, _vm: &VirtualMemory, _base_address_region: base::AddressRegion) -> Self {
        CodePageMemoryModificationScopeForDebugging {}
    }
}

impl Drop for CodePageMemoryModificationScopeForDebugging {
    fn drop(&mut self) {}
}

mod base {
    pub struct AddressRegion {
        base: usize,
        size: usize,
    }

    impl AddressRegion {
        pub fn new(base: usize, size: usize) -> Self {
            AddressRegion { base, size }
        }
    }
}

mod utils {
    pub fn round_up(value: usize, alignment: usize) -> usize {
        (value + alignment - 1) & !(alignment - 1)
    }
}

mod thread_isolation {
    pub fn register_jit_page(_base: usize, _chunk_size: usize) {}
    pub fn unregister_jit_page(_address: Address, _size: usize) {}
}

mod discard_sealed_memory_scope {
    pub struct DiscardSealedMemoryScope {}
    impl DiscardSealedMemoryScope {
        pub fn new(_reason: &str) -> Self {
            DiscardSealedMemoryScope {}
        }
    }

    impl Drop for DiscardSealedMemoryScope {
        fn drop(&mut self) {}
    }
}

macro_rules! DCHECK {
    ($condition:expr) => {
        if !$condition {
            panic!("DCHECK failed: {}", stringify!($condition));
        }
    };
}

macro_rules! CHECK {
    ($condition:expr) => {
        if !$condition {
            panic!("CHECK failed: {}", stringify!($condition));
        }
    };
}

macro_rules! CHECK_EQ {
    ($left:expr, $right:expr) => {
        if $left != $right {
            panic!("CHECK_EQ failed: {} != {}", $left, $right);
        }
    };
}

macro_rules! CHECK_NE {
    ($left:expr, $right:expr) => {
        if $left == $right {
            panic!("CHECK_NE failed: {} == {}", $left, $right);
        }
    };
}

macro_rules! CHECK_NULL {
    ($ptr:expr) => {
        if $ptr != ptr::null_mut() {
            panic!("CHECK_NULL failed: ptr is not null");
        }
    };
}

macro_rules! CHECK_NOT_NULL {
    ($ptr:expr) => {
        if $ptr == ptr::null_mut() {
            panic!("CHECK_NOT_NULL failed: ptr is null");
        }
    };
}

macro_rules! DLOG {
    ($isolate:expr, $event:expr) => {
        // Dummy implementation
    };
}

macro_rules! LOG {
    ($isolate:expr, $event:expr) => {
        // Dummy implementation
    };
}

fn free_pages(_page_allocator: *mut PageAllocator, _address: *mut std::ffi::c_void, _size: usize) {}

fn aligned_address(address: usize, alignment: usize) -> usize {
    (address + alignment - 1) & !(alignment - 1)
}

fn is_aligned(address: usize, alignment: usize) -> bool {
    address & (alignment - 1) == 0
}

fn commit_page_size() -> usize {
    4096 // A reasonable default
}

mod v8_flags {
    pub static mut v8_os_page_size: i32 = 0;
    pub static mut black_allocated_pages: bool = false;
}

mod incremental_marking {
    pub fn black_allocation() -> bool {
        false
    }
}

mod heap {
    pub fn zap_block(_address: Address, _size: usize, _zap_value: i32) {}
}

pub mod gc_tracer {
  pub struct GCTracer {}
}

pub mod flags {
  pub static mut black_allocated_pages: bool = false;
}

pub mod sandbox {
    pub mod hardware_support {
        pub fn notify_read_only_page_created(_address: Address, _size: usize, _permission: i32) {}
    }
}

pub mod bits {
    pub fn is_power_of_two(n: usize) -> bool {
        (n != 0) && ((n & (n - 1)) == 0)
    }

    pub fn which_power_of_two(n: usize) -> usize {
        if !is_power_of_two(n) {
            panic!("Not a power of two");
        }
        n.trailing_zeros() as usize
    }
}

pub mod memory_chunk_layout {
    pub fn object_start_offset_in_memory_chunk(_space: AllocationSpace) -> Address {
        64
    }

    pub fn allocatable_memory_in_memory_chunk(_space: AllocationSpace) -> usize {
        4032
    }
}

pub mod gc {
    pub mod gc_tracer_inl {
       pub struct GCTracerInl {}
    }
}

pub fn is_any_shared_space(_space: AllocationSpace) -> bool {
    false
}

pub mod i {
    pub fn i_heap() {}
}

pub mod base_i {
    pub fn bits_i() {}
}

pub struct TestMemoryAllocatorScope {}

impl TestMemoryAllocatorScope {
    pub fn new() -> Self {
        TestMemoryAllocatorScope {}
    }
}

impl Drop for TestMemoryAllocatorScope {
    fn drop(&mut self) {}
}

pub mod memory_allocator {
    use super::*;

    pub struct Pool {
        allocator: *mut MemoryAllocator,
        pooled_chunks: Mutex<Vec<*mut MutablePageMetadata>>,
        mutex: Mutex<()>,
    }

    impl Pool {
        pub fn new(allocator: *mut MemoryAllocator) -> Self {
            Pool {
                allocator,
                pooled_chunks: Mutex::new(Vec::new()),
                mutex: Mutex::new(()),
            }
        }

        pub fn add(&self, chunk: *mut MutablePageMetadata) {
            unsafe {
                // This method is called only on the main thread and only during the
                // atomic pause so a lock is not needed.
                CHECK_NOT_NULL!(chunk);
                //DCHECK_EQ!(((*chunk).size) as i64, PageMetadata::kPageSize as i64);
                //DCHECK!(!(*chunk).is_large_page());
                //DCHECK!(!(*chunk).is_trusted());
                //DCHECK_NE!((*chunk).executable(), Executability::EXECUTABLE);
                //(*chunk).release_all_allocated_memory();
                let mut guard = self.pooled_chunks.lock().unwrap();
                guard.push(chunk);
            }
        }

        pub fn try_get_pooled(&self) -> Option<*mut MemoryChunkMetadata> {
            let _guard = self.mutex.lock().unwrap();
            let mut pooled_chunks = self.pooled_chunks.lock().unwrap();
            if pooled_chunks.is_empty() {
                return None;
            }
            let chunk = pooled_chunks.pop().unwrap();
            Some(chunk as *mut MemoryChunkMetadata)
        }

        pub fn release_pooled_chunks(&self) {
            let mut copied_pooled: Vec<*mut MutablePageMetadata> = Vec::new();
            {
                let mut guard = self.pooled_chunks.lock().unwrap();
                copied_pooled = guard.drain(..).collect();
            }
            for chunk_metadata in copied_pooled {
                unsafe {
                    CHECK_NOT_NULL!(chunk_metadata);
                    delete_memory_chunk(chunk_metadata);
                }
            }
        }

        pub fn number_of_committed_chunks(&self) -> usize {
            let guard = self.pooled_chunks.lock().unwrap();
            guard.len()
        }

        pub fn committed_buffered_memory(&self) -> usize {
            self.number_of_committed_chunks() * PageMetadata::kPageSize
        }
    }

    unsafe fn delete_memory_chunk(metadata: *mut MutablePageMetadata) {
        // The Metadata contains a VirtualMemory reservation and the destructor will
        // release the MemoryChunk.
        //DiscardSealedMemoryScope discard_scope("Deleting a memory chunk");
        if (*metadata).is_large_page() {
            //delete reinterpret_cast<LargePageMetadata*>(metadata);
            todo!()
        } else {
            //delete reinterpret_cast<PageMetadata*>(metadata);
            todo!()
        }
    }
}

#[allow(dead_code)]
pub struct MemoryAllocator {
  isolate_: *mut Isolate,
  data_page_allocator_: *mut PageAllocator,
  code_page_allocator_: *mut PageAllocator,
  trusted_page_allocator_: *mut PageAllocator,
  capacity_: usize,
  size_: AtomicUsize,
  size_executable_: AtomicUsize,
  lowest_not_executable_ever_allocated_: AtomicUsize,
  highest_not_executable_ever_allocated_: AtomicUsize,
  lowest_executable_ever_allocated_: AtomicUsize,
  highest_executable_ever_allocated_: AtomicUsize,
  reserved_chunk_at_virtual_memory_limit_: Option<VirtualMemory>,
  pool_: Mutex<memory_allocator::Pool>,
  queued_pages_to_be_freed_: Mutex<Vec<*mut MutablePageMetadata>>,

  normal_pages_: Mutex<HashSet<*const MemoryChunk>>,
  large_pages_: Mutex<BTreeSet<*const MemoryChunk>>,
  chunks_mutex_: Mutex<()>,
}

impl MemoryAllocator {
  pub fn new(
    isolate: *mut Isolate,
    code_page_allocator: *mut PageAllocator,
    trusted_page_allocator: *mut PageAllocator,
    capacity: usize,
  ) -> Self {
    let rounded_capacity = utils::round_up(capacity, PageMetadata::kPageSize);

    MemoryAllocator {
      isolate_: isolate,
      data_page_allocator_: unsafe { (*isolate).page_allocator() },
      code_page_allocator_: code_page_allocator,
      trusted_page_allocator_: trusted_page_allocator,
      capacity_: rounded_capacity,
      size_: AtomicUsize::new(0),
      size_executable_: AtomicUsize::new(0),
      lowest_not_executable_ever_allocated_: AtomicUsize::new(usize::MAX),
      highest_not_executable_ever_allocated_: AtomicUsize::new(0),
      lowest_executable_ever_allocated_: AtomicUsize::new(usize::MAX),
      highest_executable_ever_allocated_: AtomicUsize::new(0),
      reserved_chunk_at_virtual_memory_limit_: None,
      pool_: Mutex::new(memory_allocator::Pool::new(unsafe { ptr::NonNull::new_unchecked(self as *mut Self) }.as_ptr())),
      queued_pages_to_be_freed_: Mutex::new(Vec::new()),
      normal_pages_: Mutex::new(HashSet::new()),
      large_pages_: Mutex::new(BTreeSet::new()),
      chunks_mutex_: Mutex::new(()),
    }
  }

  pub fn tear_down(&mut self) {
    self.pool().lock().unwrap().release_pooled_chunks();

    CHECK_EQ!(self.size_.load(Ordering::Relaxed), 0);
    self.capacity_ = 0;

    if self.reserved_chunk_at_virtual_memory_limit_.is_some() {
      self.reserved_chunk_at_virtual_memory_limit_
        .as_mut()
        .unwrap()
        .free();
    }

    self.code_page_allocator_ = ptr::null_mut();
    self.data_page_allocator_ = ptr::null_mut();
    self.trusted_page_allocator_ = ptr::null_mut();
  }

  pub fn pool(&self) -> &Mutex<memory_allocator::Pool> {
    &self.pool_
  }

  pub fn allocate_page(
    &self,
    alloc_mode: AllocationMode,
    space: *mut Space,
    executable: Executability,
  ) -> *mut PageMetadata {
    let size = unsafe {
        memory_chunk_layout::allocatable_memory_in_memory_chunk((*space).identity())
    };
    let mut chunk_info = if alloc_mode == AllocationMode::kUsePool {
      CHECK_EQ!(executable, Executability::NOT_EXECUTABLE);
      self.allocate_uninitialized_page_from_pool(space)
    } else {
      None
    };

    if chunk_info.is_none() {
      chunk_info = self.allocate_uninitialized_chunk(space, size, executable, PageSize::kRegular);
    }

    if chunk_info.is_none() {
      return ptr::null_mut();
    }

    let chunk_info = chunk_info.unwrap();
    let metadata;
    if chunk_info.optional_metadata.is_some() {
        //metadata = new (chunk_info.optional_metadata) PageMetadata(
        //    isolate_->heap(), space, chunk_info.size, chunk_info.area_start,
        //    chunk_info.area_end, std::move(chunk_info.reservation));
        todo!()
    } else {
        unsafe {
            metadata = Box::into_raw(Box::new(PageMetadata::new((*self.isolate_).heap(), space, chunk_info.size, chunk_info.area_start, chunk_info.area_end, chunk_info.reservation)));
        }
    }
    unsafe {
        let chunk;
        let flags = (*metadata).initial_flags(executable);
        if executable == Executability::EXECUTABLE {
            //RwxMemoryWriteScope scope("Initialize a new MemoryChunk.");
            //chunk = new (chunk_info.chunk) MemoryChunk(flags, metadata);
            chunk = Box::into_raw(Box::new(MemoryChunk::new(flags, metadata))) as *mut MemoryChunk;
        } else {
            //chunk = new (chunk_info.chunk) MemoryChunk(flags, metadata);
            chunk = Box::into_raw(Box::new(MemoryChunk::new(flags, metadata))) as *mut MemoryChunk;
        }
        (*metadata).set_chunk(chunk);
        self.record_memory_chunk_created(&(*chunk));
        return metadata;
    }
  }

    pub fn allocate_large_page(
        &self,
        space: *mut LargeObjectSpace,
        object_size: usize,
        executable: Executability,
    ) -> *mut LargePageMetadata {
        let chunk_info = self.allocate_uninitialized_chunk(space, object_size, executable, PageSize::kLarge);

        if chunk_info.is_none() {
            return ptr::null_mut();
        }

        let chunk_info = chunk_info.unwrap();
        unsafe {
            let metadata = Box::into_raw(Box::new(LargePageMetadata::new((*self.isolate_).heap(), space, chunk_info.size, chunk_info.area_start, chunk_info.area_end, chunk_info.reservation, executable)));
            let chunk;
            let flags = (*metadata).initial_flags(executable);
            if executable == Executability::EXECUTABLE {
                //RwxMemoryWriteScope scope("Initialize a new MemoryChunk.");
                //chunk = new (chunk_info.chunk) MemoryChunk(flags, metadata);
                chunk = Box::into_raw(Box::new(MemoryChunk::new(flags, metadata))) as *mut MemoryChunk;
            } else {
                //chunk = new (chunk_info.chunk) MemoryChunk(flags, metadata);
                chunk = Box::into_raw(Box::new(MemoryChunk::new(flags, metadata))) as *mut MemoryChunk;
            }

            (*metadata).set_chunk(chunk);
            self.record_memory_chunk_created(&(*chunk));
            return metadata;
        }
    }

    pub fn allocate_read_only_page(
        &self,
        space: *mut ReadOnlySpace,
        hint: Address,
    ) -> *mut ReadOnlyPageMetadata {
        unsafe {
            CHECK_EQ!((*space).identity(), RO_SPACE);
            let size = memory_chunk_layout::allocatable_memory_in_memory_chunk(RO_SPACE);
            let chunk_info = self.allocate_uninitialized_chunk_at(space, size, Executability::NOT_EXECUTABLE, hint, PageSize::kRegular);
            if chunk_info.is_none() {
                return ptr::null_mut();
            }

            let chunk_info = chunk_info.unwrap();
            CHECK_NULL!(chunk_info.optional_metadata);

            let metadata = Box::into_raw(Box::new(ReadOnlyPageMetadata::new((*self.isolate_).heap(), space, chunk_info.size, chunk_info.area_start, chunk_info.area_end, chunk_info.reservation)));
            let chunk = Box::into_raw(Box::new(MemoryChunk::new((*metadata).initial_flags(), metadata)));
            (*metadata).set_chunk(chunk);

            sandbox::hardware_support::notify_read_only_page_created((*chunk).address(), (*metadata).size(), 0);

            return metadata;
        }
    }

  fn allocate_uninitialized_chunk(
    &self,
    space: *mut dyn BaseSpace,
    area_size: usize,
    executable: Executability,
    page_size: PageSize,
  ) -> Option<MemoryChunkAllocationResult> {
    self.allocate_uninitialized_chunk_at(space, area_size, executable, 0, page_size)
  }

  fn allocate_uninitialized_chunk_at(
      &self,
      space: *mut dyn BaseSpace,
      area_size: usize,
      executable: Executability,
      hint: Address,
      page_size: PageSize,
  ) -> Option<MemoryChunkAllocationResult> {
    unsafe {
        // When pointer compression is enabled, spaces are expected to be at a
        // predictable address (see mkgrokdump) so we don't supply a hint and rely on
        // the deterministic behaviour of the BoundedPageAllocator.
        let hint = if hint == 0 {
            aligned_address(((*self.isolate_).heap()).get_random_mmap_addr() as usize, MemoryChunk::get_alignment_for_allocation()) as Address
        } else {
            hint
        };
        let mut controller = VirtualMemory { address: 0, size: 0, executable };

        let chunk_size = compute_chunk_size(area_size, (*space).identity());
        CHECK_EQ!(chunk_size % commit_page_size(), 0);

        let base = self.allocate_aligned_memory(chunk_size, area_size, MemoryChunk::get_alignment_for_allocation(), (*space).identity(), executable, hint as *mut std::ffi::c_void, &mut controller);
        if base == 0 {
            return None;
        }
        self.size_.fetch_add(controller.size, Ordering::Relaxed);

        if executable == Executability::EXECUTABLE {
            self.size_executable_.fetch_add(controller.size, Ordering::Relaxed);
        }
        if heap_should_zap_garbage() {
            if executable == Executability::EXECUTABLE {
                //CodePageMemoryModificationScopeForDebugging memory_write_scope(
                //    isolate_->heap(), &reservation,
                //    base::AddressRegion(base, chunk_size));
                //heap::ZapBlock(base, chunk_size, kZapValue);
                todo!()
            } else {
                DCHECK_EQ!(executable, Executability::NOT_EXECUTABLE);
                // Zap both page header and object area at once. No guard page in-between.
                //heap::ZapBlock(base, chunk_size, kZapValue);
                todo!()
            }
        }
        DLOG!(self.isolate_, ("MemoryChunk", base, chunk_size));

        let area_start = base + memory_chunk_layout::object_start_offset_in_memory_chunk((*space).identity());
        let area_end = area_start + area_size;

        Some(MemoryChunkAllocationResult {
            chunk: base as *mut std::ffi::c_void,
            optional_metadata: None,
            size: chunk_size,
            area_start,
            area_end,
            reservation: controller,
        })
    }
  }

  fn allocate_aligned_memory(
    &self,
    chunk_size: usize,
    area_size: usize,
    alignment: usize,
    space: AllocationSpace,
    executable: Executability,
    hint: *mut std::ffi::c_void,
    controller: &mut VirtualMemory,
  ) -> Address {
    unsafe {
        CHECK_EQ!((space == AllocationSpace::CODE_SPACE || space == AllocationSpace::CODE_LO_SPACE), executable == Executability::EXECUTABLE);
        let page_allocator = self.page_allocator(space);
        //DCHECK_LT!(area_size as i64, chunk_size as i64);

        //PageAllocator::Permission permissions =
        //    executable == EXECUTABLE
        //    ? MutablePageMetadata::GetCodeModificationPermission()
        //    : PageAllocator::kReadWrite;

        //VirtualMemory reservation(page_allocator, chunk_size, hint, alignment,
        //                            permissions);
        let mut reservation = VirtualMemory::new(hint as usize, chunk_size, executable);
        if !reservation.is_reserved() {
            return self.handle_allocation_failure(executable);
        }

        if (reservation.address() + chunk_size as Address) == 0 {
            //CHECK!(!self.reserved_chunk_at_virtual_memory_limit_);
            //self.reserved_chunk_at_virtual_memory_limit_ = std::move(reservation);
            //CHECK!(self.reserved_chunk_at_virtual_memory_limit_);

            // Retry reserve virtual memory.
            //reservation =
            //    VirtualMemory(page_allocator, chunk_size, hint, alignment, permissions);
            //if (!reservation.IsReserved()) return HandleAllocationFailure(executable);
            todo!()
        }

        let base = reservation.address();

        if executable == Executability::EXECUTABLE {
            thread_isolation::register_jit_page(base as usize, chunk_size);
        }

        self.update_allocated_space_limits(base, base + chunk_size as Address, executable);

        *controller = reservation;
        return base;
    }
  }

  fn handle_allocation_failure(&self, executable: Executability) -> Address {
    unsafe {
        let heap = (*self.isolate_).heap();
        if !(*heap).deserialization_complete() {
            //heap->FatalProcessOutOfMemory(
            //    executable == EXECUTABLE
            //    ? "Executable MemoryChunk allocation failed during deserialization."
            //    : "MemoryChunk allocation failed during deserialization.");
            todo!()
        }
        0
    }
  }

  fn update_allocated_space_limits(&self, low: Address, high: Address, executable: Executability) {
    // The use of atomic primitives does not guarantee correctness (wrt.
    // desired semantics) by default. The loop here ensures that we update the
    // values only if they did not change in between.
    match executable {
      Executability::NOT_EXECUTABLE => {
        loop {
          let ptr = self.lowest_not_executable_ever_allocated_.load(Ordering::Relaxed);
          if low < ptr as Address && self.lowest_not_executable_ever_allocated_.compare_exchange_weak(ptr, low as usize, Ordering::AcqRel, Ordering::Relaxed).is_ok() {
            break;
          }
        }
        loop {
          let ptr = self.highest_not_executable_ever_allocated_.load(Ordering::Relaxed);
          if high > ptr as Address && self.highest_not_executable_ever_allocated_.compare_exchange_weak(ptr, high as usize, Ordering::AcqRel, Ordering::Relaxed).is_ok() {
            break;
          }
        }
      }
      Executability::EXECUTABLE => {
        loop {
          let ptr = self.lowest_executable_ever_allocated_.load(Ordering::Relaxed);
          if low < ptr as Address && self.lowest_executable_ever_allocated_.compare_exchange_weak(ptr, low as usize, Ordering::AcqRel, Ordering::Relaxed).is_ok() {
            break;
          }
        }
        loop {
          let ptr = self.highest_executable_ever_allocated_.load(Ordering::Relaxed);
          if high > ptr as Address && self.highest_executable_ever_allocated_.compare_exchange_weak(ptr, high as usize, Ordering::AcqRel, Ordering::Relaxed).is_ok() {
            break;
          }
        }
      }
    }
  }

  fn allocate_uninitialized_page_from_pool(&self, space: *mut Space) -> Option<MemoryChunkAllocationResult> {
    let chunk_metadata = self.pool().lock().unwrap().try_get_pooled();
    if chunk_metadata.is_none() {
      return None;
    }

    let chunk_metadata = chunk_metadata.unwrap();
    let size = MutablePageMetadata::kPageSize;
    unsafe {
        let start = (*(chunk_metadata as *mut MemoryChunkMetadata)).chunk_address();
        let area_start = start + memory_chunk_layout::object_start_offset_in_memory_chunk((*space).identity());
        let area_end = start + size as Address;
        // Pooled pages are always regular data pages.
        DCHECK_NE!(AllocationSpace::CODE_SPACE, (*space).identity());
        let mut reservation = VirtualMemory::new(start as usize, size, Executability::NOT_EXECUTABLE);
        if heap::should_zap_garbage() {
            //heap::ZapBlock(start, size, kZapValue);
            todo!()
        }
        self.size_.fetch_add(size, Ordering::Relaxed);
        return Some(MemoryChunkAllocationResult {
            chunk: (*(chunk_metadata as *mut MemoryChunkMetadata)).chunk() as *mut std::ffi::c_void,
            optional_metadata: Some(chunk_metadata as *mut MemoryChunkMetadata as *mut std::ffi::c_void),
            size,
            area_start,
            area_end,
            reservation,
        });
    }
  }

  pub fn page_allocator(&self, space: AllocationSpace) -> *mut PageAllocator {
    match space {
      AllocationSpace::CODE_SPACE | AllocationSpace::CODE_LO_SPACE => self.code_page_allocator_,
      AllocationSpace::TRUSTED_SPACE | AllocationSpace::SHARED_TRUSTED_SPACE | AllocationSpace::TRUSTED_LO_SPACE | AllocationSpace::SHARED_TRUSTED_LO_SPACE => self.trusted_page_allocator_,
      _ => self.data_page_allocator_,
    }
  }

    pub fn free(
        &self,
        mode: FreeMode,
        chunk_metadata: *mut MutablePageMetadata,
    ) {
        unsafe {
            let chunk = (*chunk_metadata).chunk();
            self.record_memory_chunk_destroyed(chunk);
            match mode {
                FreeMode::kImmediately => {
                    self.pre_free_memory(chunk_metadata);
                    self.perform_free_memory(chunk_metadata);
                }
                FreeMode::kPostpone => {
                    self.pre_free_memory(chunk_metadata);
                    let mut guard = self.queued_pages_to_be_freed_.lock().unwrap();
                    guard.push(chunk_metadata);
                }
                FreeMode::kPool => {
                    //DCHECK_EQ!(chunk_metadata->size(), static_cast<size_t>(MutablePageMetadata::kPageSize));
                    //DCHECK_EQ!(chunk->executable(), NOT_EXECUTABLE);
                    self.pre_free_memory(chunk_metadata);
                    let pool = self.pool().lock().unwrap();
                    pool.add(chunk_metadata);
                }
            }
        }
    }

    fn pre_free_memory(&self, chunk_metadata: *mut MutablePageMetadata) {
        unsafe {
            let chunk = (*chunk_metadata).chunk();
            //DCHECK!(!chunk->IsFlagSet(MemoryChunk::PRE_FREED));
            DLOG!(self.isolate_, ("MemoryChunk", chunk_metadata));
            self.unregister_mutable_memory_chunk(chunk_metadata);
            ((*self.isolate_).heap()).remember_unmapped_page(
                chunk_metadata as Address,
                chunk.is_evacuation_candidate(),
            );
            chunk.set_flag_slow(MemoryChunk::PRE_FREED);
        }
    }

    fn perform_free_memory(&self, chunk_metadata: *mut MutablePageMetadata) {
        unsafe {
            //DCHECK!(chunk_metadata->chunk()->IsFlagSet(MemoryChunk::UNREGISTERED));
            //DCHECK!(chunk_metadata->chunk()->IsFlagSet(MemoryChunk::PRE_FREED));
            //DCHECK!(!chunk_metadata->chunk()->InReadOnlySpace());

            //chunk_metadata->ReleaseAllAllocatedMemory();

            //delete_memory_chunk(chunk_metadata);
            todo!()
        }
    }

    fn unregister_mutable_memory_chunk(&self, chunk_metadata: *mut MutablePageMetadata) {
        unsafe {
            self.unregister_memory_chunk(chunk_metadata, (*(*chunk_metadata).chunk()).executable());
        }
    }

    fn unregister_memory_chunk(&self, chunk_metadata: *mut MemoryChunkMetadata, executable: Executability) {
        unsafe {
            let chunk = (*chunk_metadata).chunk();
            //DCHECK!(!chunk->IsFlagSet(MemoryChunk::UNREGISTERED));
            let size = if (*chunk_metadata).reserved_memory().is_reserved() {
                (*chunk_metadata).reserved_memory().size()
            } else {
                (*chunk_metadata).size()
            };

            self.size_.fetch_sub(size, Ordering::Relaxed);
            if executable == Executability::EXECUTABLE {
                self.size_executable_.fetch_sub(size, Ordering::Relaxed);
            }
            chunk.set_flag_slow(MemoryChunk::UNREGISTERED);
        }
    }

    fn record_memory_chunk_created(&self, chunk: &MemoryChunk) {
        let _guard = self.chunks_mutex_.lock().unwrap();
        if chunk.is_large_page() {
            let mut large_pages = self.large_pages_.lock().unwrap();
            let result = large_pages.insert(chunk);
            DCHECK!(result);
        } else {
            let mut normal_pages = self.normal_pages_.lock().unwrap();
            let result = normal_pages.insert(chunk as *const MemoryChunk);
            DCHECK!(result);
        }
