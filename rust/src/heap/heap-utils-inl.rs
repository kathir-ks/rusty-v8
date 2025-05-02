// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod heap_utils {
    use crate::heap::memory_chunk::MemoryChunk;
    use crate::heap::Heap;
    use crate::objects::HeapObject;
    use crate::objects::Tagged;

    pub struct HeapUtils {}

    impl HeapUtils {
        /// Returns the owner heap of the given object.
        pub fn get_owner_heap(object: Tagged<HeapObject>) -> *mut Heap {
            MemoryChunk::from_heap_object(object).get_heap()
        }
    }
}

pub mod heap {
    pub struct Heap {}
}

pub mod objects {
    #[derive(Clone, Copy)]
    pub struct HeapObject {}

    #[derive(Clone, Copy)]
    pub struct Tagged<T> {
        _phantom: std::marker::PhantomData<T>,
    }
}

pub mod memory_chunk {
    use crate::heap::Heap;
    use crate::objects::HeapObject;
    use crate::objects::Tagged;

    pub struct MemoryChunk {}

    impl MemoryChunk {
        pub fn from_heap_object(_object: Tagged<HeapObject>) -> Self {
            MemoryChunk {}
        }

        pub fn get_heap(&self) -> *mut Heap {
            std::ptr::null_mut() // Returning null for now since Heap object is empty
        }
    }
}