// Converted from V8 C++ source files:
// Header: read-only-spaces.h
// Implementation: read-only-spaces.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod read_only_spaces {
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;
use crate::heap::stress_scavenge_observer::V8;
use crate::heap::stress_scavenge_observer::code;
use crate::heap::stress_scavenge_observer::v8;
use crate::heap::scavenger::CopyAndForwardResult;
use crate::snapshot::snapshot::ReadOnlyHeap;
use crate::heap::scavenger::ReadOnlyPageMetadata;
use crate::init::isolate_group::MemoryChunkMetadata;
use crate::heap::marking_worklist::Address;
use crate::zone::accounting_allocator::VirtualMemory;
use crate::heap::read_only_heap::ReadOnlySpace;
use crate::heap::marking_inl::Isolate;
use crate::heap::marking_inl::Heap;
use crate::heap::local_heap::AllocationResult;
use crate::heap::local_heap::AllocationAlignment;
use crate::heap::scavenger::ReadOnlyRoots;
use crate::objects::objects::If;
use crate::codegen::code_stub_assembler::isolate;

use std::{
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use v8::PageAllocator;

#[derive(Debug)]
struct AllocationStats {
    size: usize,
    capacity: usize,
    allocated_on_page: std::collections::HashMap<*const ReadOnlyPageMetadata, usize>,
}

impl AllocationStats {
    fn new() -> Self {
        AllocationStats {
            size: 0,
            capacity: 0,
            allocated_on_page: std::collections::HashMap::new(),
        }
    }

    fn increase_capacity(&mut self, size: usize) {
        self.capacity += size;
    }

    fn decrease_capacity(&mut self, size: i64) {
        self.capacity = (self.capacity as i64 - size) as usize;
    }

    fn increase_allocated_bytes(&mut self, size: usize, chunk: *const ReadOnlyPageMetadata) {
        self.size += size;
        *self.allocated_on_page.entry(chunk).or_insert(0) += size;
    }

    fn allocated_on_page(&self, page: *const ReadOnlyPageMetadata) -> usize {
        self.allocated_on_page.get(&page).copied().unwrap_or(0)
    }

    fn clear(&mut self) {
        self.size = 0;
        self.capacity = 0;
        self.allocated_on_page.clear();
    }

    fn size(&self) -> usize {
        self.size
    }

    fn capacity(&self) -> usize {
        self.capacity
    }
}

pub struct ReadOnlyArtifacts {
    pages_: Vec<*mut ReadOnlyPageMetadata>,
    stats_: AllocationStats,
    shared_read_only_space_: Option<Box<SharedReadOnlySpace>>,
    read_only_heap_: Option<Box<ReadOnlyHeap>>,
    initial_next_unique_sfi_id_: u32,
    external_pointer_registry_: Vec<ExternalPointerRegistryEntry>,
    page_allocator_: *mut PageAllocator, // Assuming v8::PageAllocator is FFI safe
}

impl ReadOnlyArtifacts {
    pub fn new() -> Self {
        ReadOnlyArtifacts {
            pages_: Vec::new(),
            stats_: AllocationStats::new(),
            shared_read_only_space_: None,
            read_only_heap_: None,
            initial_next_unique_sfi_id_: 0,
            external_pointer_registry_: Vec::new(),
            page_allocator_: std::ptr::null_mut(),
        }
    }

    pub fn initialize(&mut self, isolate: &mut Isolate, pages: Vec<*mut ReadOnlyPageMetadata>, stats: AllocationStats) {
        self.page_allocator_ = isolate.isolate_group.page_allocator;
        self.pages_ = pages;
        self.stats_ = stats;
        self.shared_read_only_space_ = Some(Box::new(SharedReadOnlySpace::new(isolate.heap.as_mut().unwrap(), self)));
    }

    pub fn reinstall_read_only_space(&mut self, isolate: &mut Isolate) {
        isolate.heap.as_mut().unwrap().replace_read_only_space(self.shared_read_only_space_.as_mut().map(|space| &mut **space));
    }

    pub fn verify_heap_and_space_relationships(&self, isolate: &Isolate) {
        if let Some(read_only_heap) = &self.read_only_heap_ {
            if let Some(shared_read_only_space) = &self.shared_read_only_space_ {
                assert_eq!(read_only_heap.read_only_space(), Some(shared_read_only_space.as_ref()));
            }

            assert_eq!(self.read_only_heap_ .as_ref().map(|x| &**x as *const ReadOnlyHeap), isolate.read_only_heap);
            assert_eq!(self.shared_read_only_space_.as_ref().map(|x| &**x as *const SharedReadOnlySpace), isolate.heap.as_ref().unwrap().read_only_space.map(|x| x as *const SharedReadOnlySpace));
        }
    }

    pub fn pages(&mut self) -> &mut Vec<*mut ReadOnlyPageMetadata> {
        &mut self.pages_
    }

    pub fn accounting_stats(&self) -> &AllocationStats {
        &self.stats_
    }

    pub fn shared_read_only_space(&mut self) -> Option<&mut SharedReadOnlySpace> {
        self.shared_read_only_space_.as_mut().map(|space| &mut **space)
    }

    pub fn set_read_only_heap(&mut self, read_only_heap: Box<ReadOnlyHeap>) {
        self.read_only_heap_ = Some(read_only_heap);
    }

    pub fn read_only_heap(&self) -> Option<&ReadOnlyHeap> {
        self.read_only_heap_.as_ref().map(|heap| &**heap)
    }

    pub fn set_initial_next_unique_sfi_id(&mut self, id: u32) {
        self.initial_next_unique_sfi_id_ = id;
    }

    pub fn initial_next_unique_sfi_id(&self) -> u32 {
        self.initial_next_unique_sfi_id_
    }

    pub fn set_external_pointer_registry(
        &mut self,
        registry: Vec<ExternalPointerRegistryEntry>,
    ) {
        assert!(self.external_pointer_registry_.is_empty());
        self.external_pointer_registry_ = registry;
    }
    pub fn external_pointer_registry(&self) -> &Vec<ExternalPointerRegistryEntry> {
        &self.external_pointer_registry_
    }

    pub fn initialize_checksum(&mut self, read_only_snapshot_data: &SnapshotData) {
        self.read_only_blob_checksum_ = Some(checksum(read_only_snapshot_data.payload.clone()));
    }
    pub fn verify_checksum(&self, read_only_snapshot_data: &SnapshotData, read_only_heap_created: bool) {
        if let Some(read_only_blob_checksum_) = self.read_only_blob_checksum_ {
            let snapshot_checksum = checksum(read_only_snapshot_data.payload.clone());
            assert!(snapshot_checksum != 0, "Attempt to create the read-only heap after already creating from a snapshot.");
            if !v8::V8_FLAGS.stress_snapshot {
                assert_eq!(read_only_blob_checksum_, snapshot_checksum);
            }
        } else {
            assert!(read_only_heap_created);
        }
    }
}

impl Drop for ReadOnlyArtifacts {
    fn drop(&mut self) {
        if let Some(shared_read_only_space_) = &mut self.shared_read_only_space_ {
            shared_read_only_space_.tear_down(std::ptr::null_mut());
        }
        let page_allocator = self.page_allocator_;

        for metadata in &self.pages_ {
            let chunk_address = unsafe { (*(*metadata)).chunk_address() } as *mut std::ffi::c_void;
            let size = unsafe { crate::heap::paged_spaces::RoundUp((*metadata).size(), unsafe { (*page_allocator).AllocatePageSize() }) };
            unsafe {
                assert!((&*page_allocator).FreePages(chunk_address, size));
            }
            drop(metadata);
        }
    }
}

pub struct ExternalPointerRegistryEntry {
    handle: ExternalPointerHandle,
    value: Address,
    tag: ExternalPointerTag,
}

impl ExternalPointerRegistryEntry {
    pub fn new(handle: ExternalPointerHandle, value: Address, tag: ExternalPointerTag) -> Self {
        ExternalPointerRegistryEntry { handle, value, tag }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ExternalPointerTag {
    kExternalPointerApi,
}

pub struct ExternalPointerHandle {}

struct SnapshotData {
    payload: Vec<u8>,
}

fn checksum(payload: Vec<u8>) -> u32 {
    let mut sum: u32 = 0;
    for byte in payload {
        sum = sum.wrapping_add(byte as u32);
    }
    sum
}

#[derive(Debug)]
pub struct IsolateGroup {
    page_allocator: *mut PageAllocator,
}

#[derive(Debug)]
pub struct HeapImpl {
    pub read_only_space: Option<*mut SharedReadOnlySpace>,
}

impl HeapImpl {
    pub fn replace_read_only_space(&mut self, space: Option<&mut SharedReadOnlySpace>) {
        self.read_only_space = space.map(|s| s as *mut SharedReadOnlySpace);
    }
}

#[derive(Debug)]
pub struct Isolate {
    pub heap: Option<Box<HeapImpl>>,
    pub read_only_heap: *const ReadOnlyHeap,
    pub isolate_group: IsolateGroup,
}

impl Isolate {
    pub fn new(page_allocator: *mut PageAllocator) -> Self {
        Isolate {
            heap: Some(Box::new(HeapImpl {
                read_only_space: None,
            })),
            read_only_heap: std::ptr::null(),
            isolate_group: IsolateGroup {
                page_allocator: page_allocator,
            },
        }
    }
}

pub struct SharedReadOnlySpace {
    base: ReadOnlySpace,
}

impl SharedReadOnlySpace {
    pub fn new(heap: &mut HeapImpl, artifacts: &mut ReadOnlyArtifacts) -> Self {
        let accounting_stats_ = artifacts.accounting_stats().clone();
        let pages_ = artifacts.pages().clone();
        let mut shared_space = SharedReadOnlySpace {
            base: ReadOnlySpace {
                heap_: None,
                space_type_: v8::internal::SpaceType::RO_SPACE,
                is_marked_read_only_: true,
                accounting_stats_: accounting_stats_,
                pages_: pages_,
                top_: Address { address: 0 },
                limit_: Address { address: 0 },
                capacity_: 0,
            },
        };
        shared_space.base.heap_ = Some(heap);
        shared_space
    }

    fn tear_down(&mut self, _memory_allocator: *mut std::ffi::c_void) {
        self.base.pages_.clear();
        self.base.accounting_stats_.clear();
    }
}

impl SharedReadOnlySpace {
    fn read_only_space(&self) -> Option<&SharedReadOnlySpace>{
        Some(self)
    }
}
}
