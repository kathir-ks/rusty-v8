// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Note: This translation is incomplete as it relies on types/functions
//       not defined in the provided C++ code (e.g., Tagged<String>,
//       PtrComprCageBase, IsHeapObject, Cast<String>, ExternalString, etc.)
//       and external crates which implementation is impossible
//       without the full codebase.Placeholders are used for such cases.

use std::sync::atomic::{AtomicPtr, AtomicU32, Ordering};
//use std::mem::transmute;

//use crate::base::atomicops::*;
//use crate::common::globals::*;
//use crate::heap::safepoint::*;
//use crate::objects::name::*;
//use crate::objects::slots::*;
//use crate::objects::string_forwarding_table::*;
//use crate::objects::string::*;

const K_EXTERNAL_RESOURCE_IS_ONE_BYTE_TAG: usize = 1;
const K_EXTERNAL_RESOURCE_ENCODING_MASK: usize = 1;
const K_EXTERNAL_RESOURCE_ADDRESS_MASK: usize = !K_EXTERNAL_RESOURCE_ENCODING_MASK;
const K_HEAP_OBJECT_TAG: u32 = 1;
const K_HEAP_OBJECT_TAG_MASK: u32 = 1;

#[derive(Debug)]
pub struct StringForwardingTable {
    next_free_index_: std::sync::atomic::AtomicI32,
    blocks_: AtomicPtr<BlockVector>,
}

impl StringForwardingTable {
    pub fn size(&self) -> i32 {
        self.next_free_index_.load(Ordering::Relaxed)
    }
    pub fn empty(&self) -> bool {
        self.size() == 0
    }

    // static
    pub fn block_for_index(index: i32, index_in_block: &mut u32) -> u32 {
        debug_assert!(index >= 0);
        debug_assert!(index_in_block != &mut 0);

        const K_BITS_PER_INT: u32 = 32;
        const K_INITIAL_BLOCK_SIZE: u32 = 16;
        const K_INITIAL_BLOCK_SIZE_HIGHEST_BIT: u32 = 4;

        // The block is the leftmost set bit of the index, corrected by the size
        // of the first block.
        let block_index =
            K_BITS_PER_INT - (index as u32 + K_INITIAL_BLOCK_SIZE).leading_zeros()
            - K_INITIAL_BLOCK_SIZE_HIGHEST_BIT - 1;
        *index_in_block = Self::index_in_block(index, block_index);
        block_index
    }

    // static
    pub fn index_in_block(index: i32, block_index: u32) -> u32 {
        debug_assert!(index >= 0);
        const K_INITIAL_BLOCK_SIZE: u32 = 16;
        const K_INITIAL_BLOCK_SIZE_HIGHEST_BIT: u32 = 4;

        // Clear out the leftmost set bit (the block index) to get the index within
        // the block.
        ((index as u32 + K_INITIAL_BLOCK_SIZE) & !(1 << (block_index + K_INITIAL_BLOCK_SIZE_HIGHEST_BIT))) as u32
    }

    // static
    pub fn capacity_for_block(block_index: u32) -> u32 {
        const K_INITIAL_BLOCK_SIZE_HIGHEST_BIT: u32 = 4;
        1 << (block_index + K_INITIAL_BLOCK_SIZE_HIGHEST_BIT)
    }

    pub fn iterate_elements<F>(&self, mut callback: F)
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
            let block = blocks.load_block(block_index as usize);
            for index in 0..block.capacity() {
                let rec = block.record(index as usize);
                callback(rec);
            }
        }

        // Handle last block separately, as it is not filled to capacity.
        let max_index = Self::index_in_block(
            self.size() - 1,
            last_block_index,
        ) + 1;

        let block = blocks.load_block(last_block_index as usize);

        for index in 0..max_index {
            let rec = block.record(index as usize);
            callback(rec);
        }
    }
}

#[derive(Debug)]
pub struct Record {
    original_string_: AtomicPtr<Object>,
    forward_string_or_hash_: AtomicPtr<Object>,
    external_resource_: AtomicU32, // Address stored as u32
}

impl Record {
    pub fn original_string(&self, _cage_base: PtrComprCageBase) -> Tagged<String> {
        // TODO: Implement Cast<String> and OriginalStringObject
        let obj_ptr = self.OriginalStringObject(_cage_base);
        unsafe { Tagged::<String>::from_ptr(obj_ptr as *mut String) }
    }

    pub fn forward_string(&self, _cage_base: PtrComprCageBase) -> Tagged<String> {
        // TODO: Implement Cast<String> and ForwardStringObjectOrHash
        let obj_ptr = self.ForwardStringObjectOrHash(_cage_base);
        unsafe { Tagged::<String>::from_ptr(obj_ptr as *mut String) }
    }

    pub fn raw_hash(&self, cage_base: PtrComprCageBase) -> u32 {
        let hash_or_string = self.ForwardStringObjectOrHash(cage_base);
        let raw_hash: u32;
        if is_heap_object(hash_or_string) {
            raw_hash = unsafe { (hash_or_string as *mut Object as *mut String).read_unaligned() }.raw_hash();
        } else {
            raw_hash = hash_or_string as u32;
        }
        debug_assert!(is_hash_field_computed(raw_hash));
        raw_hash
    }

    pub fn external_resource(&self, is_one_byte: &mut bool) -> *mut v8::String::ExternalStringResourceBase {
        let address = self.external_resource_address();
        *is_one_byte = (address as usize & K_EXTERNAL_RESOURCE_ENCODING_MASK) == K_EXTERNAL_RESOURCE_IS_ONE_BYTE_TAG;
        let address = address as usize & K_EXTERNAL_RESOURCE_ADDRESS_MASK;
        address as *mut v8::String::ExternalStringResourceBase
    }

    pub fn OriginalStringObject(&self, cage_base: PtrComprCageBase) -> *mut Object {
        //TODO: Implement OriginalStringSlot().Acquire_Load(cage_base)
        self.original_string_.load(Ordering::Acquire)
    }

    pub fn ForwardStringObjectOrHash(&self, cage_base: PtrComprCageBase) -> *mut Object {
        //TODO: Implement ForwardStringOrHashSlot().Acquire_Load(cage_base)
        self.forward_string_or_hash_.load(Ordering::Acquire)
    }

    pub fn external_resource_address(&self) -> u32 {
        self.external_resource_.load(Ordering::Acquire)
    }

    pub fn set_original_string(&self, object: *mut Object) {
        //TODO: Implement OriginalStringSlot().Release_Store(object)
        self.original_string_.store(object, Ordering::Release);
    }

    pub fn set_forward_string(&self, object: *mut Object) {
        //TODO: Implement ForwardStringOrHashSlot().Release_Store(object)
        self.forward_string_or_hash_.store(object, Ordering::Release);
    }

    pub fn set_raw_hash_if_empty(&self, raw_hash: u32) {
        // Assert that computed hash values don't overlap with heap object tag.
        assert!((K_HEAP_OBJECT_TAG & (!0)) != 0);
        debug_assert!(is_hash_field_computed(raw_hash));
        debug_assert_ne!(raw_hash & K_HEAP_OBJECT_TAG_MASK, K_HEAP_OBJECT_TAG);

        //AsAtomicTagged::Release_CompareAndSwap(&self.forward_string_or_hash_, unused_element().value(), raw_hash);
        let current = self.forward_string_or_hash_.load(Ordering::Acquire);

        if current.is_null() {
            self.forward_string_or_hash_.compare_and_swap(current, raw_hash as *mut Object, Ordering::Release);
        }
    }

    pub fn set_external_resource(&self, resource: *mut v8::String::ExternalStringResourceBase, is_one_byte: bool) {
        debug_assert!(!resource.is_null());
        let mut address = resource as u32;
        if is_one_byte && address != 0 {
            address |= K_EXTERNAL_RESOURCE_IS_ONE_BYTE_TAG as u32;
        }
        self.set_external_resource_address(address);
    }

    pub fn set_external_resource_address(&self, address: u32) {
        self.external_resource_.store(address, Ordering::Release);
    }

    pub fn SetInternalized(&self, string: Tagged<String>, forward_to: Tagged<String>) {
        self.set_original_string(string.ptr() as *mut Object);
        self.set_forward_string(forward_to.ptr() as *mut Object);
        self.set_external_resource_address(0); //kNullExternalPointer
    }

    pub fn SetExternal(&self, string: Tagged<String>, resource: *mut v8::String::ExternalStringResourceBase, is_one_byte: bool, raw_hash: u32) {
        self.set_original_string(string.ptr() as *mut Object);
        self.set_raw_hash_if_empty(raw_hash);
        self.set_external_resource(resource, is_one_byte);
    }

    pub fn TryUpdateExternalResource(&self, resource: *mut v8::String::ExternalStringResourceBase, is_one_byte: bool) -> bool {
        debug_assert!(!resource.is_null());
        let mut address = resource as u32;
        if (is_one_byte && address != 0) {
            address |= K_EXTERNAL_RESOURCE_IS_ONE_BYTE_TAG as u32;
        }
        self.TryUpdateExternalResourceAddress(address)
    }

    pub fn TryUpdateExternalResourceAddress(&self, address: u32) -> bool {
        assert_eq!(0, 0); //kNullAddress == kNullExternalPointer
                            // Don't set the external resource if another one is already stored. If we
                            // would simply overwrite the resource, the previously stored one would be
                            // leaked.
        self.external_resource_.compare_and_swap(0, address, Ordering::AcquireRelease) == 0
    }

    pub fn DisposeExternalResource(&self) {
        let mut is_one_byte = false;
        let resource = self.external_resource(&mut is_one_byte);
        debug_assert!(!resource.is_null());
        unsafe { (*resource).Dispose(); }
    }

    pub fn DisposeUnusedExternalResource(&self, isolate: *mut Isolate, original_string: Tagged<String>) {
        //TODO: Implement IsExternalString
        //TODO: Implement Cast<ExternalString>
        //TODO: Implement original.resource_as_address()
        //TODO: Implement stored_original
        if !is_external_string(original_string) {
            return;
        }

        let original_resource = unsafe { (original_string.ptr() as *mut ExternalString).read().resource_as_address() };

        let mut is_one_byte = false;
        let resource = self.external_resource(&mut is_one_byte);

        if !resource.is_null() && (resource as u32) != original_resource {
            unsafe { (*resource).Dispose(); }
        }
    }
}

#[derive(Debug)]
pub struct Block {
    capacity_: i32,
    elements_: Vec<Record>,
}

impl Block {
    pub fn new(capacity: i32) -> Self {
        Block {
            capacity_: capacity,
            elements_: (0..capacity).map(|_| Record {
                original_string_: AtomicPtr::new(std::ptr::null_mut()),
                forward_string_or_hash_: AtomicPtr::new(std::ptr::null_mut()),
                external_resource_: AtomicU32::new(0),
            }).collect(),
        }
    }

    pub fn capacity(&self) -> i32 {
        self.capacity_
    }

    pub fn record(&self, index: usize) -> &Record {
        debug_assert!(index < self.capacity() as usize);
        &self.elements_[index]
    }

    pub fn record_mut(&mut self, index: usize) -> &mut Record {
        debug_assert!(index < self.capacity() as usize);
        &mut self.elements_[index]
    }

    pub fn UpdateAfterYoungEvacuation(&self, _cage_base: PtrComprCageBase) {
        //TODO: Implement UpdateAfterYoungEvacuation
    }
    pub fn UpdateAfterYoungEvacuation_up_to_index(&self, _cage_base: PtrComprCageBase, _up_to_index: i32) {
        //TODO: Implement UpdateAfterYoungEvacuation
    }
    pub fn UpdateAfterFullEvacuation(&self, _cage_base: PtrComprCageBase) {
        //TODO: Implement UpdateAfterFullEvacuation
    }
    pub fn UpdateAfterFullEvacuation_up_to_index(&self, _cage_base: PtrComprCageBase, _up_to_index: i32) {
        //TODO: Implement UpdateAfterFullEvacuation
    }
}

#[derive(Debug)]
pub struct BlockVector {
    capacity_: usize,
    size_: AtomicUsize,
    begin_: Vec<AtomicPtr<Block>>,
}

impl BlockVector {
    pub fn new(capacity: usize) -> Self {
        BlockVector {
            capacity_: capacity,
            size_: AtomicUsize::new(0),
            begin_: (0..capacity).map(|_| AtomicPtr::new(std::ptr::null_mut())).collect(),
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity_
    }

    pub fn size(&self) -> usize {
        self.size_.load(Ordering::Relaxed)
    }

    pub fn load_block(&self, index: usize) -> &Block {
        debug_assert!(index < self.size());
        let ptr = self.begin_[index].load(Ordering::Acquire);
        unsafe { &*ptr }
    }

    pub fn add_block(&self, block: Box<Block>) {
        debug_assert!(self.size() < self.capacity());
        let block_ptr = Box::into_raw(block);
        self.begin_[self.size()].store(block_ptr, Ordering::Release);
        self.size_.fetch_add(1, Ordering::Release);
    }
}

use std::sync::atomic::AtomicUsize;

// Placeholder types and functions

#[derive(Debug, Copy, Clone)]
struct Tagged<T> {
    ptr: *mut T,
}

impl<T> Tagged<T> {
    unsafe fn from_ptr(ptr: *mut T) -> Self {
        Tagged { ptr }
    }
    fn ptr(&self) -> *mut T {
        self.ptr
    }
}

#[derive(Debug, Copy, Clone)]
struct PtrComprCageBase;

#[derive(Debug, Copy, Clone)]
struct Isolate;

#[derive(Debug, Copy, Clone)]
struct String;
impl String {
    fn raw_hash(&self) -> u32 {
        0
    }
}

#[derive(Debug, Copy, Clone)]
struct ExternalString;
impl ExternalString {
    fn resource_as_address(&self) -> u32 {
        0
    }
}

#[derive(Debug, Copy, Clone)]
struct ThinString;

#[derive(Debug, Copy, Clone)]
struct Object;

mod v8 {
    pub mod String {
        #[derive(Debug, Copy, Clone)]
        pub struct ExternalStringResourceBase;

        impl ExternalStringResourceBase {
            pub fn Dispose(&mut self) {}
        }
    }
}

fn is_heap_object(_obj: *mut Object) -> bool {
    false
}
fn is_hash_field_computed(_hash: u32) -> bool {
    false
}
fn is_external_string(_string: Tagged<String>) -> bool {
    false
}
fn is_thin_string(_string: Tagged<String>) -> bool {
    false
}

const K_INITIAL_BLOCK_SIZE: u32 = 16;
const K_INITIAL_BLOCK_SIZE_HIGHEST_BIT: u32 = 4;