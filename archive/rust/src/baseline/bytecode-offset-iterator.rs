// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod execution;
mod heap;
mod objects;

use crate::execution::isolate::Isolate;
use crate::heap::local_heap::LocalHeap;
use crate::objects::bytecode_array::BytecodeArray;
use crate::objects::trusted_byte_array::TrustedByteArray;
use std::cell::RefCell;
use std::rc::Rc;

pub mod baseline {

    use super::*;
    use std::ptr::NonNull;

    const K_FUNCTION_ENTRY_BYTECODE_OFFSET: usize = 0;

    // This struct represents the BytecodeOffsetIterator.
    pub struct BytecodeOffsetIterator {
        mapping_table_: Option<Rc<TrustedByteArray>>,
        data_start_address_: *const u8,
        data_length_: usize,
        current_index_: usize,
        bytecode_iterator_: BytecodeArrayIterator,
        local_heap_: Option<Rc<RefCell<LocalHeap>>>,
        bytecode_handle_storage_: BytecodeArray,
        no_gc_: Option<NoGarbageCollection>,
        current_pc_start_offset_: usize,
        current_pc_end_offset_: usize,
        current_bytecode_offset_: usize,
    }

    impl BytecodeOffsetIterator {
        /// Creates a new `BytecodeOffsetIterator` from a `TrustedByteArray` handle and a `BytecodeArray` handle.
        pub fn new(
            mapping_table: Rc<TrustedByteArray>,
            bytecodes: Rc<BytecodeArray>,
        ) -> Self {
            let local_heap = LocalHeap::current().map(Rc::clone).or_else(|| {
                Isolate::current().map(|isolate| isolate.main_thread_local_heap())
            });
            let mut iterator = BytecodeOffsetIterator {
                mapping_table_: Some(mapping_table.clone()),
                data_start_address_: mapping_table.begin(),
                data_length_: mapping_table.length(),
                current_index_: 0,
                bytecode_iterator_: BytecodeArrayIterator::new(bytecodes.clone()),
                local_heap_: local_heap.clone(),
                bytecode_handle_storage_: BytecodeArray::default(), // Dummy value, will be overwritten.
                no_gc_: None,
                current_pc_start_offset_: 0,
                current_pc_end_offset_: 0,
                current_bytecode_offset_: 0,
            };
            if let Some(heap) = &local_heap {
                let iterator_ptr = NonNull::from(&iterator);
                heap.borrow_mut()
                    .add_gc_epilogue_callback(update_pointers_callback, iterator_ptr);
            }

            iterator.initialize();
            iterator
        }

        /// Creates a new `BytecodeOffsetIterator` from a `TrustedByteArray` and a `BytecodeArray`.  No garbage collection is allowed.
        pub fn new_non_gc(mapping_table: &TrustedByteArray, bytecodes: &BytecodeArray) -> Self {
            let mut iterator = BytecodeOffsetIterator {
                mapping_table_: None,
                data_start_address_: mapping_table.begin(),
                data_length_: mapping_table.length(),
                current_index_: 0,
                bytecode_iterator_: BytecodeArrayIterator::new(Rc::new(bytecodes.clone())),
                local_heap_: None,
                bytecode_handle_storage_: bytecodes.clone(),
                no_gc_: Some(NoGarbageCollection::new()),
                current_pc_start_offset_: 0,
                current_pc_end_offset_: 0,
                current_bytecode_offset_: 0,
            };

            iterator.initialize();
            iterator
        }

        /// Destructor for `BytecodeOffsetIterator`.  Removes the GC epilogue callback if a local heap exists.
        pub fn destroy(&mut self) {
            if let Some(local_heap) = &self.local_heap_ {
                if let Some(mapping_table) = &self.mapping_table_ {
                     let iterator_ptr = NonNull::from(self);
                     local_heap.borrow_mut().remove_gc_epilogue_callback(
                       update_pointers_callback,
                       iterator_ptr,
                    );
                }

            }
        }

        fn initialize(&mut self) {
            // Initialize values for the prologue.
            // The first recorded position is at the start of the first bytecode.
            self.current_pc_start_offset_ = 0;
            self.current_pc_end_offset_ = self.read_position();
            self.current_bytecode_offset_ = K_FUNCTION_ENTRY_BYTECODE_OFFSET;
        }

        fn update_pointers(&mut self) {
            let _no_gc = NoGarbageCollection::new();
            if let Some(mapping_table) = &self.mapping_table_ {
                self.data_start_address_ = mapping_table.begin();
            }
        }

        fn read_position(&self) -> usize {
            // Dummy implementation
            0
        }
    }

    impl Drop for BytecodeOffsetIterator {
        fn drop(&mut self) {
            self.destroy();
        }
    }

    fn update_pointers_callback(iterator_ptr: NonNull<BytecodeOffsetIterator>) {
        unsafe {
            iterator_ptr.as_ptr().as_mut().unwrap().update_pointers();
        }
    }

    struct NoGarbageCollection {}

    impl NoGarbageCollection {
        fn new() -> Self {
            NoGarbageCollection {}
        }
    }

    struct BytecodeArrayIterator {
        bytecode_array: Rc<BytecodeArray>,
    }

    impl BytecodeArrayIterator {
        fn new(bytecode_array: Rc<BytecodeArray>) -> Self {
            BytecodeArrayIterator { bytecode_array }
        }
    }
} // namespace baseline