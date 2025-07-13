// Converted from V8 C++ source files:
// Header: string-forwarding-table-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(non_snake_case)]
use crate::base::atomicops;
use crate::common::globals::*;
use crate::heap::safepoint::Safepoint;
use crate::objects::name_inl::*;
use crate::objects::slots_inl::*;
use crate::objects::slots::OffHeapObjectSlot;
use crate::objects::string_forwarding_table::*;
use crate::objects::string_inl::*;
use std::sync::atomic::{AtomicPtr, AtomicU32, Ordering};
use std::ptr;

pub struct StringForwardingTable {
    next_free_index_: i32,
    blocks_: AtomicPtr<BlockVector>,
    mutex_: Box<std::sync::Mutex<()>>,
}

impl StringForwardingTable {
    pub fn new() -> Self {
        let initial_block_size = kInitialBlockSize as usize;
        let initial_block = Block::New(initial_block_size);

        let mut initial_block_vector = BlockVector::new(kInitialBlockVectorSize as usize);
        initial_block_vector.AddBlock(initial_block);

        StringForwardingTable {
            next_free_index_: 0,
            blocks_: AtomicPtr::new(Box::into_raw(Box::new(initial_block_vector))),
            mutex_: Box::new(std::sync::Mutex::new(())),
        }
    }
    
    pub fn size(&self) -> i32 {
        self.next_free_index_
    }

    pub fn empty(&self) -> bool {
        self.size() == 0
    }

    pub fn BlockForIndex(index: i32, index_in_block: *mut u32) -> u32 {
        assert!(index >= 0);
        assert!(!index_in_block.is_null());

        let block_index =
            kBitsPerInt as u32
            - (index as u32 + kInitialBlockSize as u32).leading_zeros()
            - kInitialBlockSizeHighestBit as u32
            - 1;

        unsafe {
            *index_in_block = Self::IndexInBlock(index, block_index);
        }
        
        block_index
    }

    pub fn IndexInBlock(index: i32, block_index: u32) -> u32 {
        assert!(index >= 0);

        (index as u32 + kInitialBlockSize as u32) & !(1u32 << (block_index + kInitialBlockSizeHighestBit as u32))
    }

    pub fn CapacityForBlock(block_index: u32) -> u32 {
        1u32 << (block_index + kInitialBlockSizeHighestBit as u32)
    }

    pub fn IterateElements<F>(&self, mut callback: F)
    where
        F: FnMut(&Record),
    {
        if self.empty() {
            return;
        }
    
        let blocks_ptr = self.blocks_.load(Ordering::Relaxed);
        let blocks: &BlockVector = unsafe { &*blocks_ptr };
        let last_block_index = (blocks.size() - 1) as u32;
    
        for block_index in 0..last_block_index {
            let block = blocks.LoadBlock(block_index as usize);
            for index in 0..block.capacity() {
                let rec = block.record(index);
                callback(rec);
            }
        }
    
        let max_index = Self::IndexInBlock(self.size() - 1, last_block_index) + 1;
        let block = blocks.LoadBlock(last_block_index as usize);
        for index in 0..max_index {
            let rec = block.record(index as usize);
            callback(rec);
        }
    }
}

impl StringForwardingTable {
    pub const kInitialBlockSize: i32 = 4;
    pub const kInitialBlockSizeHighestBit: i32 = 2;
    pub const kInitialBlockVectorSize: i32 = 4;
}

pub struct Record {
    original_string_: Tagged_t,
    forward_string_or_hash_: Tagged_t,
    external_resource_: Address,
}

impl Record {
    pub fn original_string(&self, cage_base: PtrComprCageBase) -> Tagged<String> {
        unsafe { std::mem::transmute::<Tagged<Object>, Tagged<String>>(self.OriginalStringObject(cage_base)) }
    }

    pub fn forward_string(&self, cage_base: PtrComprCageBase) -> Tagged<String> {
        unsafe { std::mem::transmute::<Tagged<Object>, Tagged<String>>(self.ForwardStringObjectOrHash(cage_base)) }
    }

    pub fn raw_hash(&self, cage_base: PtrComprCageBase) -> u32 {
        let hash_or_string = self.ForwardStringObjectOrHash(cage_base);
        let raw_hash: u32;
        if IsHeapObject(hash_or_string) {
            raw_hash = unsafe { std::mem::transmute::<Tagged<Object>, Tagged<String>>(hash_or_string).RawHash() };
        } else {
            raw_hash = hash_or_string.ptr() as u32;
        }
        assert!(Name::IsHashFieldComputed(raw_hash));
        raw_hash
    }

    pub fn external_resource(&self, is_one_byte: *mut bool) -> *mut v8::String::ExternalStringResourceBase {
        let address = self.ExternalResourceAddress();
        unsafe {
            *is_one_byte = (address & Self::kExternalResourceEncodingMask) == Self::kExternalResourceIsOneByteTag;
        }
        let address = address & Self::kExternalResourceAddressMask;
        address as *mut v8::String::ExternalStringResourceBase
    }

    pub fn OriginalStringObject(&self, cage_base: PtrComprCageBase) -> Tagged<Object> {
        self.OriginalStringSlot().Acquire_Load(cage_base)
    }

    pub fn ForwardStringObjectOrHash(&self, cage_base: PtrComprCageBase) -> Tagged<Object> {
        self.ForwardStringOrHashSlot().Acquire_Load(cage_base)
    }

    pub fn ExternalResourceAddress(&self) -> Address {
        atomicops::Acquire_Load(&self.external_resource_)
    }

    pub fn set_original_string(&mut self, object: Tagged<Object>) {
        self.OriginalStringSlot().Release_Store(object);
    }

    pub fn set_forward_string(&mut self, object: Tagged<Object>) {
        self.ForwardStringOrHashSlot().Release_Store(object);
    }

    pub fn set_raw_hash_if_empty(&mut self, raw_hash: u32) {
        assert!((kHeapObjectTag & Name::kHashNotComputedMask) != 0);
        assert!(Name::IsHashFieldComputed(raw_hash));
        assert_ne!(raw_hash & kHeapObjectTagMask, kHeapObjectTag);

        let current_value = unsafe {
            atomicops::CompareAndSwap(&mut self.forward_string_or_hash_, unused_element().value() as Address, raw_hash as Address)
        };
    }

    pub fn set_external_resource(
        &mut self,
        resource: *mut v8::String::ExternalStringResourceBase,
        is_one_byte: bool,
    ) {
        assert!(!resource.is_null());
        let mut address = resource as Address;
        if is_one_byte && address != kNullAddress {
            address |= Self::kExternalResourceIsOneByteTag;
        }
        self.set_external_resource_address(address);
    }

    fn set_external_resource_address(&mut self, address: Address) {
        atomicops::Release_Store(&mut self.external_resource_, address);
    }

    pub fn SetInternalized(&mut self, string: Tagged<String>, forward_to: Tagged<String>) {
        self.set_original_string(unsafe { std::mem::transmute::<Tagged<String>, Tagged<Object>>(string) });
        self.set_forward_string(unsafe { std::mem::transmute::<Tagged<String>, Tagged<Object>>(forward_to) });
        self.set_external_resource_address(kNullExternalPointer);
    }

    pub fn SetExternal(
        &mut self,
        string: Tagged<String>,
        resource: *mut v8::String::ExternalStringResourceBase,
        is_one_byte: bool,
        raw_hash: u32,
    ) {
        self.set_original_string(unsafe { std::mem::transmute::<Tagged<String>, Tagged<Object>>(string) });
        self.set_raw_hash_if_empty(raw_hash);
        self.set_external_resource(resource, is_one_byte);
    }

    pub fn TryUpdateExternalResource(
        &mut self,
        resource: *mut v8::String::ExternalStringResourceBase,
        is_one_byte: bool,
    ) -> bool {
        assert!(!resource.is_null());
        let mut address = resource as Address;
        if (is_one_byte && address != kNullAddress) {
            address |= Self::kExternalResourceIsOneByteTag;
        }
        self.TryUpdateExternalResourceAddress(address)
    }

    pub fn TryUpdateExternalResourceAddress(&mut self, address: Address) -> bool {
        assert_eq!(kNullAddress, kNullExternalPointer);
    
        unsafe {
            atomicops::CompareAndSwap(&mut self.external_resource_, kNullAddress, address) == kNullAddress
        }
    }

    pub fn DisposeExternalResource(&self) {
        let mut is_one_byte = false;
        let resource = self.external_resource(&mut is_one_byte);
        assert!(!resource.is_null());
        unsafe {
            (*resource).Dispose();
        }
    }

    pub fn DisposeUnusedExternalResource(&self, isolate: *mut Isolate, original: Tagged<String>) {
        if !IsExternalString(original) {
            return;
        }

        let original_resource = unsafe { std::mem::transmute::<Tagged<String>, Tagged<ExternalString>>(original).resource_as_address() };
        let mut is_one_byte = false;
        let resource = self.external_resource(&mut is_one_byte);
    
        if !resource.is_null() && (resource as Address) != original_resource {
            unsafe {
                (*resource).Dispose();
            }
        }
    }

    fn OriginalStringSlot(&self) -> OffHeapObjectSlot {
        OffHeapObjectSlot(&self.original_string_)
    }

    fn ForwardStringOrHashSlot(&self) -> OffHeapObjectSlot {
        OffHeapObjectSlot(&self.forward_string_or_hash_)
    }

    const kExternalResourceIsOneByteTag: Address = 1;
    const kExternalResourceEncodingMask: Address = 1;
    const kExternalResourceAddressMask: Address = !Self::kExternalResourceEncodingMask;
}

pub struct Block {
    capacity_: i32,
    elements_: [Record; 1],
}

impl Block {
    pub fn New(capacity: i32) -> Box<Self> {
        let mut block = unsafe {
            let layout = std::alloc::Layout::new::<Block>()
                .extend(std::alloc::Layout::array::<Record>(capacity as usize).unwrap())
                .unwrap()
                .0
                .pad_to_align();

            let ptr = std::alloc::alloc(layout);

            if ptr.is_null() {
                std::alloc::handle_alloc_error(layout);
            }
            let block = ptr as *mut Block;
            
            std::ptr::write(block, Block {
                capacity_: capacity,
                elements_: [Record {
                    original_string_: Tagged_t { ptr_: 0 },
                    forward_string_or_hash_: Tagged_t { ptr_: 0 },
                    external_resource_: 0,
                }; 1],
            });

            block
        };
        unsafe { Box::from_raw(block) }
    }

    pub fn new(capacity: i32) -> Self {
        Block {
            capacity_: capacity,
            elements_: [Record {
                original_string_: Tagged_t { ptr_: 0 },
                forward_string_or_hash_: Tagged_t { ptr_: 0 },
                external_resource_: 0,
            }; 1],
        }
    }

    pub fn capacity(&self) -> i32 {
        self.capacity_
    }

    pub fn record(&self, index: usize) -> &Record {
        assert!(index < self.capacity() as usize);
        unsafe {
            let elements_ptr = self.elements_.as_ptr();
            &*elements_ptr.add(index)
        }
    }

    pub fn record_mut(&mut self, index: usize) -> &mut Record {
        assert!(index < self.capacity() as usize);
        unsafe {
            let elements_ptr = self.elements_.as_mut_ptr();
            &mut *elements_ptr.add(index)
        }
    }

    pub fn UpdateAfterYoungEvacuation(&mut self, cage_base: PtrComprCageBase) {
        for index in 0..self.capacity() as usize {
            self.record_mut(index);
        }
    }

    pub fn UpdateAfterYoungEvacuationToIndex(&mut self, cage_base: PtrComprCageBase, up_to_index: i32) {
        for index in 0..up_to_index as usize {
            self.record_mut(index);
        }
    }

    pub fn UpdateAfterFullEvacuation(&mut self, cage_base: PtrComprCageBase) {
        for index in 0..self.capacity() as usize {
            self.record_mut(index);
        }
    }

    pub fn UpdateAfterFullEvacuationToIndex(&mut self, cage_base: PtrComprCageBase, up_to_index: i32) {
        for index in 0..up_to_index as usize {
            self.record_mut(index);
        }
    }
}

pub struct BlockVector {
    allocator_: std::alloc::Allocator,
    capacity_: usize,
    size_: AtomicUsize,
    begin_: Vec<*mut Block>,
}

impl BlockVector {
    pub fn new(capacity: usize) -> Self {
        let mut begin_: Vec<*mut Block> = Vec::with_capacity(capacity);
        unsafe {
            begin_.set_len(capacity);
        }
        BlockVector {
            allocator_: std::alloc::Allocator::default(),
            capacity_: capacity,
            size_: AtomicUsize::new(0),
            begin_: begin_,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity_
    }

    pub fn LoadBlock(&self, index: usize) -> &mut Block {
        assert!(index < self.size());
        unsafe { &mut *self.begin_[index] }
    }

    pub fn AddBlock(&mut self, block: Box<Block>) {
        assert!(self.size() < self.capacity());
        let block_ptr = Box::into_raw(block);
        self.begin_[self.size()] = block_ptr;
        self.size_.fetch_add(1, Ordering::Release);
    }

    pub fn Grow(data: *mut BlockVector, capacity: usize, mutex: &std::sync::Mutex<()>) -> Box<BlockVector> {
        let _guard = mutex.lock().unwrap();

        let old_data = unsafe { &mut *data };
        let mut new_data = BlockVector::new(capacity);

        for i in 0..old_data.size() {
            new_data.begin_[i] = old_data.begin_[i];
        }
        new_data.size_.store(old_data.size(), Ordering::Relaxed);

        let _ = unsafe { Box::from_raw(data) };
        Box::new(new_data)
    }

    pub fn size(&self) -> usize {
        self.size_.load(Ordering::Relaxed)
    }
}

impl Drop for BlockVector {
    fn drop(&mut self) {
        for i in 0..self.size() {
            unsafe {
                let _ = Box::from_raw(self.begin_[i]);
            }
        }
    }
}
