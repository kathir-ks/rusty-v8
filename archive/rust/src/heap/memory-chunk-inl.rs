// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod memory_chunk_inl {
    use crate::heap::memory_chunk::{MemoryChunk, MemoryChunkConstants};
    use crate::heap::memory_chunk_metadata::MemoryChunkMetadata;
    use crate::sandbox::check::sbxcheck_eq;
    use crate::isolate::isolate_group::IsolateGroup;
    use crate::heap::heap::Heap;

    impl MemoryChunk {
        /// Returns a pointer to the MemoryChunkMetadata associated with this MemoryChunk.
        pub fn metadata(&mut self) -> *mut MemoryChunkMetadata {
            // If this changes, we also need to update
            // CodeStubAssembler::PageMetadataFromMemoryChunk
            #[cfg(feature = "v8_enable_sandbox")]
            {
                debug_assert!(self.metadata_index < MemoryChunkConstants::kMetadataPointerTableSizeMask as usize);
                let isolate_group = IsolateGroup::current();
                let metadata_pointer_table = isolate_group.metadata_pointer_table();
                let metadata = metadata_pointer_table[(self.metadata_index & MemoryChunkConstants::kMetadataPointerTableSizeMask as usize) as usize];
                // Check that the Metadata belongs to this Chunk, since an attacker with write
                // inside the sandbox could've swapped the index.

                //Assuming SBXCHECK_EQ(metadata->Chunk(), this); means metadata.chunk == self
                unsafe {
                    sbxcheck_eq((*metadata).chunk, self);
                }

                metadata
            }
            #[cfg(not(feature = "v8_enable_sandbox"))]
            {
                self.metadata_
            }
        }

        /// Returns a const pointer to the MemoryChunkMetadata associated with this MemoryChunk.
        pub fn metadata_const(&self) -> *const MemoryChunkMetadata {
            self.metadata() as *const MemoryChunkMetadata
        }

        /// Returns a pointer to the Heap associated with this MemoryChunk.
        pub fn get_heap(&mut self) -> *mut Heap {
            unsafe {
                (*self.metadata()).heap
            }
        }
    }
}