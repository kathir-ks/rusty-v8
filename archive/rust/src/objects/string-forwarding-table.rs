// src/objects/string_forwarding_table.rs

use std::sync::{Mutex, atomic::{AtomicUsize, Ordering}, Arc};
use std::mem;
use std::ptr;
use std::collections::HashSet;
use crate::objects::slots::{OffHeapObjectSlot};
use crate::objects::objects::{HeapObject, String, Object};
use crate::heap::heap::{Heap, HeapObjectIterator};
use crate::base::atomicops;
use crate::common::globals;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct MapWord {
    value: usize,
}

impl MapWord {
    const FORWARDING_ADDRESS_MASK: usize = 1;

    fn is_forwarding_address(&self) -> bool {
        (self.value & Self::FORWARDING_ADDRESS_MASK) != 0
    }

    fn to_forwarding_address<'a>(&self, object: &'a HeapObject) -> &'a HeapObject {
        unsafe {
            let address = (self.value & !Self::FORWARDING_ADDRESS_MASK) as *const HeapObject;
            &*address
        }
    }
}

trait PtrComprCageBase {
    fn compress_ptr(&self, ptr: usize) -> usize;
    fn decompress_ptr(&self, compressed: usize) -> usize;
}

struct Isolate {
    heap: Heap,
    string_forwarding_table: StringForwardingTable,
}

impl Isolate {
    fn string_forwarding_table(&self) -> &StringForwardingTable {
        &self.string_forwarding_table
    }
}

// Dummy implementation
#[derive(Clone, Copy)]
struct Address(usize);

const K_NULL_ADDRESS: Address = Address(0);

impl Address {
    fn is_null(&self) -> bool {
        self.0 == 0
    }
}

struct Flags {
    always_use_string_forwarding_table: bool,
    minor_ms: bool
}

mod v8_flags {
    use super::Flags;
    pub static mut FLAGS: Flags = Flags {
        always_use_string_forwarding_table: false,
        minor_ms: false
    };
}

const K_INITIAL_BLOCK_SIZE: usize = 16;
const K_INITIAL_BLOCK_VECTOR_CAPACITY: usize = 4;
const K_TAGGED_SIZE: usize = 8;
const K_SYSTEM_POINTER_SIZE: usize = 8; // Assuming 64-bit architecture

const UNUSED_ELEMENT_PTR: usize = 0;

fn unused_element() -> Address {
    Address(UNUSED_ELEMENT_PTR)
}

fn is_heap_object(_object: &Object) -> bool {
    true
}

fn cast<T>(object: &Object) -> &HeapObject {
    unsafe {
        &*(object as *const Object as *const HeapObject)
    }
}

fn aligned_alloc_with_retry(size: usize, alignment: usize) -> *mut u8 {
    unsafe {
        let layout = std::alloc::Layout::from_size_align(size, alignment).unwrap();
        std::alloc::alloc(layout)
    }
}

fn aligned_free(ptr: *mut u8) {
    unsafe {
        if !ptr.is_null() {
            let layout = std::alloc::Layout::from_size_align(0, K_SYSTEM_POINTER_SIZE).unwrap(); //This is incorrect and needs to store alignment for each allocation
            std::alloc::dealloc(ptr, layout);
        }
    }
}

/// Represents a record in the StringForwardingTable.
#[repr(C)]
#[derive(Debug)]
struct Record {
    original_string_: OffHeapObjectSlot,
    forward_string_or_hash_: OffHeapObjectSlot,
    external_resource_: usize,
    is_one_byte_: bool,
    padding_: [u8; 3],
    raw_hash_: u32,
}

impl Record {
    fn new() -> Self {
        Record {
            original_string_: OffHeapObjectSlot::new(Address(0)),
            forward_string_or_hash_: OffHeapObjectSlot::new(Address(0)),
            external_resource_: 0,
            is_one_byte_: false,
            padding_: [0; 3],
            raw_hash_: 0,
        }
    }

    fn original_string_slot(&self) -> OffHeapObjectSlot {
        self.original_string_
    }

    fn forward_string_or_hash_slot(&self) -> OffHeapObjectSlot {
        self.forward_string_or_hash_
    }

    fn set_internalized(&mut self, string: &String, forward_to: &String) {
        self.original_string_.release_store(string as *const String as usize);
        self.forward_string_or_hash_.release_store(forward_to as *const String as usize);
    }

    fn set_forward_string(&mut self, forward_to: &String) {
        self.forward_string_or_hash_.release_store(forward_to as *const String as usize);
    }

    fn set_external<T>(&mut self, string: &String, resource: *mut T, is_one_byte: bool, raw_hash: u32) {
        self.original_string_.release_store(string as *const String as usize);
        self.external_resource_ = resource as usize;
        self.is_one_byte_ = is_one_byte;
        self.raw_hash_ = raw_hash;
        // self.forward_string_or_hash_.release_store(raw_hash as usize); //This is stored at a different place
    }

    fn try_update_external_resource<T>(&mut self, resource: *mut T, is_one_byte: bool) -> bool {
        if self.is_one_byte_ == is_one_byte && self.external_resource_ != 0 {
            self.external_resource_ = resource as usize;
            true
        } else {
            false
        }
    }

    fn forward_string<'a>(&self, cage_base: &dyn PtrComprCageBase) -> &'a String {
        unsafe {
            let address = self.forward_string_or_hash_.acquire_load(cage_base);
            &*(address as *const String)
        }
    }

    fn forward_string_object_or_hash<'a>(&self, cage_base: &dyn PtrComprCageBase) -> &'a Object {
        unsafe {
            let address = self.forward_string_or_hash_.acquire_load(cage_base);
            &*(address as *const Object)
        }
    }

    fn raw_hash(&self, cage_base: &dyn PtrComprCageBase) -> u32 {
         unsafe {
            let hash = self.forward_string_or_hash_.acquire_load(cage_base);
            hash as u32
         }
    }

    fn external_resource(&self, is_one_byte: &mut bool) -> *mut dyn v8::String::ExternalStringResourceBase {
        *is_one_byte = self.is_one_byte_;
        self.external_resource_ as *mut dyn v8::String::ExternalStringResourceBase
    }

    fn external_resource_address(&self) -> Address {
        Address(self.external_resource_)
    }

    fn dispose_external_resource(&mut self) {
        unsafe {
            if self.external_resource_ != 0 {
                let resource = self.external_resource_ as *mut dyn v8::String::ExternalStringResourceBase;
                if !resource.is_null() {
                    // TODO: Implement the Dispose method for ExternalStringResourceBase
                    // (*resource).Dispose();
                    self.external_resource_ = 0;
                }
            }
        }
    }

    fn original_string_object<'a>(&self, isolate: &Isolate) -> &'a Object {
        unsafe {
            let address = self.original_string_.acquire_load(isolate);
            &*(address as *const Object)
        }
    }
}

/// A block of records in the StringForwardingTable.
struct Block {
    capacity_: usize,
    elements_: [Record; 0], // This is a flexible array member in C++
}

impl Block {
    fn new(capacity: usize) -> Box<Self> {
        let size = std::mem::size_of::<Block>() + capacity * std::mem::size_of::<Record>();
        unsafe {
            let layout = std::alloc::Layout::from_size_align(size, K_SYSTEM_POINTER_SIZE).unwrap();
            let ptr = std::alloc::alloc(layout) as *mut Block;
            if ptr.is_null() {
                panic!("Allocation failed");
            }

            // Initialize the Block fields manually.
            (*ptr).capacity_ = capacity;
            //(*ptr).elements_.fill(Record::new());

            // Zero out the memory for elements_.  This is crucial!
            let elements_ptr = &mut (*ptr).elements_ as *mut [Record] as *mut u8;
            let elements_size = capacity * std::mem::size_of::<Record>();
            std::ptr::write_bytes(elements_ptr, 0, elements_size);

            Box::from_raw(ptr)
        }
    }

    fn record(&self, index: usize) -> &mut Record {
        if index >= self.capacity_ {
            panic!("Index out of bounds");
        }

        unsafe {
            let base_ptr = self.elements_.as_ptr() as *mut Record;
            &mut *(base_ptr.add(index))
        }
    }

    fn update_after_young_evacuation(&mut self, cage_base: &dyn PtrComprCageBase) {
        self.update_after_young_evacuation_up_to(cage_base, self.capacity_);
    }

    fn update_after_full_evacuation(&mut self, cage_base: &dyn PtrComprCageBase) {
        self.update_after_full_evacuation_up_to(cage_base, self.capacity_);
    }

    fn update_after_young_evacuation_up_to(&mut self, cage_base: &dyn PtrComprCageBase, up_to_index: usize) {
        for index in 0..up_to_index {
            let slot = self.record(index).original_string_slot();
            let original = slot.acquire_load(cage_base);

            // if !is_heap_object(&original) {
            //     continue;
            // }

            // let object = cast(&original);

            // if Heap::in_from_page(object) {
            //     if !object.map_word(Ordering::Relaxed).is_forwarding_address() {
            //         // Object died in young space
            //         self.record(index).original_string_.release_store(UNUSED_ELEMENT_PTR);
            //     }
            // }
             //The above is incomplete due to access of methods that are not available
        }
    }

     fn update_after_full_evacuation_up_to(&mut self, cage_base: &dyn PtrComprCageBase, up_to_index: usize) {
        for index in 0..up_to_index {
            let original_slot = self.record(index).original_string_slot();
            let original = original_slot.acquire_load(cage_base);

            // if !is_heap_object(&original) {
            //     continue;
            // }

            // let object = cast(&original);

            // let forward_slot = self.record(index).forward_string_or_hash_slot();
            // let forward = forward_slot.acquire_load(cage_base);

            // if !is_heap_object(&forward) {
            //     continue;
            // }
            // let forward_object = cast(&forward);
        }
    }
}

/// A vector of blocks in the StringForwardingTable.
struct BlockVector {
    allocator_: std::alloc::Global, // Dummy allocator
    capacity_: usize,
    size_: usize,
    begin_: *mut *mut Block,
}

impl BlockVector {
    fn new(capacity: usize) -> Self {
        let layout = std::alloc::Layout::array::<*mut Block>(capacity).unwrap();
        let begin_ = unsafe { std::alloc::alloc(layout) as *mut *mut Block };
        if begin_.is_null() {
            panic!("Allocation failed");
        }
        BlockVector {
            allocator_: std::alloc::Global,
            capacity_: capacity,
            size_: 0,
            begin_: begin_,
        }
    }

    fn grow(data: &mut BlockVector, capacity: usize, mutex: &Mutex<()>) -> Self {
        let _guard = mutex.lock().unwrap(); // Hold the mutex during grow
        let mut new_data = BlockVector::new(capacity);

        // Copy pointers to blocks from the old to the new vector.
        for i in 0..data.size() {
            unsafe {
                *new_data.begin_.add(i) = data.load_block(i);
            }
        }
        new_data.size_ = data.size();

        new_data
    }

    fn add_block(&mut self, block: Box<Block>) {
        if self.size_ >= self.capacity_ {
            panic!("BlockVector is full");
        }
        unsafe {
            *self.begin_.add(self.size_) = Box::into_raw(block);
        }
        self.size_ += 1;
    }

    fn load_block(&self, index: usize) -> *mut Block {
        self.load_block_internal(index, Ordering::Relaxed)
    }

    fn load_block_internal(&self, index: usize, _ordering: Ordering) -> *mut Block {
        if index >= self.size_ {
            panic!("Index out of bounds");
        }
        unsafe {
            *self.begin_.add(index)
        }
    }

    fn size(&self) -> usize {
        self.size_
    }

    fn capacity(&self) -> usize {
        self.capacity_
    }
}

impl Drop for BlockVector {
    fn drop(&mut self) {
        // Deallocate the blocks.
        for i in 0..self.size_ {
            unsafe {
                let block_ptr = *self.begin_.add(i);
                if !block_ptr.is_null() {
                    let _ = Box::from_raw(block_ptr); //Reclaims ownership and deallocates
                }
            }
        }

        // Deallocate the begin_ array.
        let layout = std::alloc::Layout::array::<*mut Block>(self.capacity_).unwrap();
        unsafe {
            std::alloc::dealloc(self.begin_ as *mut u8, layout);
        }
    }
}

/// The StringForwardingTable.
pub struct StringForwardingTable {
    isolate_: *mut Isolate, //Raw pointer is unavoidable here as it creates a circular dependency
    next_free_index_: AtomicUsize,
    blocks_: AtomicUsize, // Points to BlockVector
    block_vector_storage_: Mutex<Vec<Box<BlockVector>>>,
    grow_mutex_: Mutex<()>,
}

impl StringForwardingTable {
    /// Creates a new StringForwardingTable.
    pub fn new(isolate: *mut Isolate) -> Self {
        let table = StringForwardingTable {
            isolate_: isolate,
            next_free_index_: AtomicUsize::new(0),
            blocks_: AtomicUsize::new(0),
            block_vector_storage_: Mutex::new(Vec::new()),
            grow_mutex_: Mutex::new(()),
        };
        table.initialize_block_vector();
        table
    }

    fn initialize_block_vector(&self) {
        let mut block_vector_storage = self.block_vector_storage_.lock().unwrap();
        let mut blocks = BlockVector::new(K_INITIAL_BLOCK_VECTOR_CAPACITY);
        blocks.add_block(Block::new(K_INITIAL_BLOCK_SIZE));
        let blocks_ptr = Box::into_raw(Box::new(blocks));
        block_vector_storage.push(unsafe {Box::from_raw(blocks_ptr)}); //Ownership transferred to vector

        self.blocks_.store(blocks_ptr as usize, Ordering::Relaxed);
    }

    fn ensure_capacity(&self, block_index: u32) -> *mut BlockVector {
        let blocks_ptr = self.blocks_.load(Ordering::Acquire) as *mut BlockVector;
        unsafe {
            if (block_index as usize) >= (*blocks_ptr).size() {
                let _guard = self.grow_mutex_.lock().unwrap();
                let blocks_ptr = self.blocks_.load(Ordering::Relaxed) as *mut BlockVector; // Reload
                if (block_index as usize) >= (*blocks_ptr).size() {
                    let mut block_vector_storage = self.block_vector_storage_.lock().unwrap();
                    if (block_index as usize) >= (*blocks_ptr).capacity() {
                        let mut new_blocks = BlockVector::grow(
                            &mut **block_vector_storage.last_mut().unwrap(),
                            (*blocks_ptr).capacity() * 2,
                            &self.grow_mutex_,
                        );
                         let new_blocks_ptr = Box::into_raw(Box::new(new_blocks));
                         block_vector_storage.push(unsafe {Box::from_raw(new_blocks_ptr)}); //Ownership transferred to vector

                        self.blocks_.store(new_blocks_ptr as usize, Ordering::Release);
                    }
                    let capacity = self.capacity_for_block(block_index);
                    let new_block = Block::new(capacity as usize);

                   let blocks_ptr = self.blocks_.load(Ordering::Relaxed) as *mut BlockVector; // Reload
                    (**block_vector_storage.last_mut().unwrap()).add_block(new_block);
                }
            }

            self.blocks_.load(Ordering::Acquire) as *mut BlockVector
        }
    }

    fn add_forward_string(&self, string: &String, forward_to: &String) -> usize {
        unsafe {
            if !v8_flags::FLAGS.always_use_string_forwarding_table {
                //DCHECK_IMPLIES(!v8_flags.always_use_string_forwarding_table,
                //   HeapLayout::InAnySharedSpace(string));
                //DCHECK_IMPLIES(!v8_flags.always_use_string_forwarding_table,
                //    HeapLayout::InAnySharedSpace(forward_to));
            }
        }
        let index = self.next_free_index_.fetch_add(1, Ordering::Relaxed);
        let mut index_in_block: u32 = 0;
        let block_index = self.block_for_index(index, &mut index_in_block);

        let blocks_ptr = self.ensure_capacity(block_index);

        unsafe {
            let block_ptr = (*blocks_ptr).load_block_internal(block_index as usize, Ordering::Acquire);
            (*block_ptr).record(index_in_block as usize).set_internalized(string, forward_to);
        }

        index
    }

    fn update_forward_string(&self, index: usize, forward_to: &String) {
        if index >= self.size() {
            panic!("Index out of bounds");
        }

        let mut index_in_block: u32 = 0;
        let block_index = self.block_for_index(index, &mut index_in_block);
        unsafe {
            let blocks_ptr = self.blocks_.load(Ordering::Acquire) as *mut BlockVector;
            let block_ptr = (*blocks_ptr).load_block_internal(block_index as usize, Ordering::Acquire);
            (*block_ptr).record(index_in_block as usize).set_forward_string(forward_to);
        }
    }

    fn add_external_resource_and_hash<T>(&self, string: &String, resource: *mut T, raw_hash: u32) -> usize {
        unsafe {
            if !v8_flags::FLAGS.always_use_string_forwarding_table {
                //DCHECK_IMPLIES(!v8_flags.always_use_string_forwarding_table,
                //   HeapLayout::InAnySharedSpace(string));
            }
        }
        let index = self.next_free_index_.fetch_add(1, Ordering::Relaxed);
        let mut index_in_block: u32 = 0;
        let block_index = self.block_for_index(index, &mut index_in_block);

        let blocks_ptr = self.ensure_capacity(block_index);
        unsafe {
            let block_ptr = (*blocks_ptr).load_block_internal(block_index as usize, Ordering::Acquire);
            let is_one_byte = std::any::TypeId::of::<T>() == std::any::TypeId::of::<v8::String::ExternalOneByteStringResource>();
            (*block_ptr).record(index_in_block as usize).set_external(string, resource, is_one_byte, raw_hash);
        }
        index
    }

    fn try_update_external_resource<T>(&self, index: usize, resource: *mut T) -> bool {
        if index >= self.size() {
            panic!("Index out of bounds");
        }

        let mut index_in_block: u32 = 0;
        let block_index = self.block_for_index(index, &mut index_in_block);
        unsafe {
            let blocks_ptr = self.blocks_.load(Ordering::Acquire) as *mut BlockVector;
            let block_ptr = (*blocks_ptr).load_block_internal(block_index as usize, Ordering::Acquire);
            let is_one_byte = std::any::TypeId::of::<T>() == std::any::TypeId::of::<v8::String::ExternalOneByteStringResource>();
            (*block_ptr).record(index_in_block as usize).try_update_external_resource(resource, is_one_byte)
        }
    }

    fn get_forward_string<'a>(&self, cage_base: &dyn PtrComprCageBase, index: usize) -> &'a String {
        if index >= self.size() {
            panic!("Index out of bounds");
        }

        let mut index_in_block: u32 = 0;
        let block_index = self.block_for_index(index, &mut index_in_block);

        unsafe {
            let blocks_ptr = self.blocks_.load(Ordering::Acquire) as *mut BlockVector;
            let block_ptr = (*blocks_ptr).load_block_internal(block_index as usize, Ordering::Acquire);
            (*block_ptr).record(index_in_block as usize).forward_string(cage_base)
        }
    }

    fn get_raw_hash(&self, cage_base: &dyn PtrComprCageBase, index: usize) -> u32 {
         if index >= self.size() {
            panic!("Index out of bounds");
        }

        let mut index_in_block: u32 = 0;
        let block_index = self.block_for_index(index, &mut index_in_block);

        unsafe {
            let blocks_ptr = self.blocks_.load(Ordering::Acquire) as *mut BlockVector;
            let block_ptr = (*blocks_ptr).load_block_internal(block_index as usize, Ordering::Acquire);
            (*block_ptr).record(index_in_block as usize).raw_hash(cage_base)
        }
    }

     fn get_raw_hash_static(isolate: &Isolate, index: usize) -> u32 {
        isolate.string_forwarding_table().get_raw_hash(isolate, index)
    }

    fn get_external_resource(&self, index: usize, is_one_byte: &mut bool) -> *mut dyn v8::String::ExternalStringResourceBase {
         if index >= self.size() {
            panic!("Index out of bounds");
        }

        let mut index_in_block: u32 = 0;
        let block_index = self.block_for_index(index, &mut index_in_block);

        unsafe {
            let blocks_ptr = self.blocks_.load(Ordering::Acquire) as *mut BlockVector;
            let block_ptr = (*blocks_ptr).load_block_internal(block_index as usize, Ordering::Acquire);
            (*block_ptr).record(index_in_block as usize).external_resource(is_one_byte)
        }
    }

    fn tear_down(&self) {
        let mut disposed_resources: HashSet<usize> = HashSet::new();
        self.iterate_elements(|record| {
            unsafe {
                let isolate = self.isolate_.as_ref().unwrap();
                if record.original_string_object(isolate) != cast(&Object::new(UNUSED_ELEMENT_PTR)) {
                     let resource = record.external_resource_address().0;
                    if resource != 0 && !disposed_resources.contains(&resource) {
                        // record.dispose_external_resource(); //Needs implementaion
                        disposed_resources.insert(resource);
                    }
                }
            }
        });
        self.reset();
    }

    fn reset(&self) {
        unsafe {
            let isolate = self.isolate_.as_mut().unwrap();
            //isolate.heap.safepoint().assert_active();
            //DCHECK_NE(isolate.heap.gc_state(), Heap::NOT_IN_GC);

            let blocks_ptr = self.blocks_.load(Ordering::Relaxed) as *mut BlockVector;
            let block_vector_storage = self.block_vector_storage_.lock().unwrap();

            //Deallocate all blocks
            for i in 0..(**block_vector_storage.last().unwrap()).size(){
                let block_ptr = (**block_vector_storage.last().unwrap()).load_block(i);
                let _ = Box::from_raw(block_ptr); //Retakes ownership and deallocates
            }
            
            let mut block_vector_storage_locked = self.block_vector_storage_.lock().unwrap();
            block_vector_storage_locked.clear();
        }

        self.initialize_block_vector();
        self.next_free_index_.store(0, Ordering::Relaxed);
    }

    fn update_after_young_evacuation(&self) {
        unsafe {
            // This is only used for the Scavenger.
            if !v8_flags::FLAGS.minor_ms && v8_flags::FLAGS.always_use_string_forwarding_table {
                if self.is_empty() {
                    return;
                }

                let blocks_ptr = self.blocks_.load(Ordering::Relaxed) as *mut BlockVector;
                 let block_vector_storage = self.block_vector_storage_.lock().unwrap();
                let last_block_index = (**block_vector_storage.last().unwrap()).size() - 1;

                for block_index in 0..last_block_index {
                    let block_ptr = (**block_vector_storage.last().unwrap()).load_block_internal(block_index as usize, Ordering::Acquire);
                    let isolate = self.isolate_.as_mut().unwrap();
                   // (*block_ptr).update_after_young_evacuation(&isolate.heap); //CageBase not implemented
                }

                let max_index = self.index_in_block(self.size() - 1, last_block_index as usize) + 1;
                  let isolate = self.isolate_.as_mut().unwrap();
               // (**block_vector_storage.last().unwrap()).load_block_internal(last_block_index as usize, Ordering::Acquire).update_after_young_evacuation_up_to(&isolate.heap, max_index); //CageBase not implemented
            }
        }
    }

    fn update_after_full_evacuation(&self) {
        if self.is_empty() {
            return;
        }
        unsafe {
            let blocks_ptr = self.blocks_.load(Ordering::Relaxed) as *mut BlockVector;
             let block_vector_storage = self.block_vector_storage_.lock().unwrap();
            let last_block_index = (**block_vector_storage.last().unwrap()).size() - 1;

            for block_index in 0..last_block_index {
                 let isolate = self.isolate_.as_mut().unwrap();
                let block_ptr = (**block_vector_storage.last().unwrap()).load_block_internal(block_index as usize, Ordering::Acquire);
                 //(*block_ptr).update_after_full_evacuation(&isolate.heap); //CageBase not implemented
            }

            let max_index = self.index_in_block(self.size() - 1, last_block_index as usize) + 1;
            let isolate = self.isolate_.as_mut().unwrap();
            //(**block_vector_storage.last().unwrap()).load_block_internal(last_block_index as usize, Ordering::Acquire).update_after_full_evacuation_up_to(&isolate.heap, max_index); //CageBase not implemented
        }
    }

    fn size(&self) -> usize {
        self.next_free_index_.load(Ordering::Relaxed)
    }

    fn is_empty(&self) -> bool {
        self.size() == 0
    }

    fn block_for_index(&self, index: usize, index_in_block: &mut u32) -> u32 {
        let block_index = (index / K_INITIAL_BLOCK_SIZE) as u32;
        *index_in_block = (index % K_INITIAL_BLOCK_SIZE) as u32;
        block_index
    }

    fn capacity_for_block(&self, block_index: u32) -> u32 {
        K_INITIAL_BLOCK_SIZE as u32
    }

    fn index_in_block(&self, index: usize, last_block_index: usize) -> usize {
        index % ((last_block_index + 1) * K_INITIAL_BLOCK_SIZE)
    }

    fn iterate_elements<F>(&self, mut f: F)
    where
        F: FnMut(&mut Record),
    {
        unsafe {
            let blocks_ptr = self.blocks_.load(Ordering::Relaxed) as *mut BlockVector;
            let block_vector_storage = self.block_vector_storage_.lock().unwrap();

            for block_index in 0..(**block_vector_storage.last().unwrap()).size() {
                let block_ptr = (**block_vector_storage.last().unwrap()).load_block_internal(block_index as usize, Ordering::Acquire);
                for index_in_block in 0..K_INITIAL_BLOCK_SIZE {
                    f((*block_ptr).record(index_in_block as usize));
                }
            }
        }
    }
}

impl Drop for StringForwardingTable {
    fn drop(&mut self) {
        let blocks_ptr = self.blocks_.load(Ordering::Relaxed) as *mut BlockVector;
        unsafe {
             let block_vector_storage = self.block_vector_storage_.lock().unwrap();
            for block_index in 0..(**block_vector_storage.last().unwrap()).size() {
                let block_ptr = (**block_vector_storage.last().unwrap()).load_block(block_index as usize);
                let _ = Box::from_raw(block_ptr); //Retake ownership to deallocate
            }
        }
    }
}

mod v8 {
    pub mod String {
        pub trait ExternalStringResourceBase {}

        pub struct ExternalOneByteStringResource {}
        impl ExternalStringResourceBase for ExternalOneByteStringResource {}

        pub struct ExternalStringResource {}
         impl ExternalStringResourceBase for ExternalStringResource {}
    }
}
