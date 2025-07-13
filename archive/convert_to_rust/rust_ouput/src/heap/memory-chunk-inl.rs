// Converted from V8 C++ source files:
// Header: memory-chunk-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
mod memory_chunk_metadata;
mod memory_chunk;
mod sandbox;
mod isolate_group;
mod heap;
use memory_chunk_metadata::*;
use memory_chunk::*;
use sandbox::*;
use isolate_group::*;
use heap::*;

use std::ptr::null_mut;
use std::mem::transmute;

impl MemoryChunk {
    pub fn metadata(&mut self) -> *mut MemoryChunkMetadata {
        #[cfg(feature = "V8_ENABLE_SANDBOX")]
        {
            if self.metadata_index_ as usize >= MemoryChunkConstants::kMetadataPointerTableSizeMask {
                panic!("Metadata index out of bounds");
            }

            let isolate_group = IsolateGroup::current();
            let metadata_pointer_table = unsafe { isolate_group.metadata_pointer_table(isolate_group) };

            let metadata = unsafe {
                *(*metadata_pointer_table).get_unchecked(
                    self.metadata_index_ as usize & MemoryChunkConstants::kMetadataPointerTableSizeMask
                )
            };

            if unsafe { (*metadata).chunk() != self } {
                panic!("Metadata does not belong to this chunk");
            }
            metadata
        }
        #[cfg(not(feature = "V8_ENABLE_SANDBOX"))]
        {
            self.metadata_
        }
    }

    pub fn metadata_const(&self) -> *const MemoryChunkMetadata {
        self.metadata() as *const MemoryChunkMetadata
    }

    pub fn get_heap(&mut self) -> *mut Heap {
        unsafe { (*self.metadata()).heap() }
    }
}
