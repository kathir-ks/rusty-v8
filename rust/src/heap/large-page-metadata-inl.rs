// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod large_page_metadata {
    use crate::heap::large_page_metadata::LargePageMetadata;
    use crate::heap::mutable_page_metadata::MutablePageMetadata;
    use crate::objects::heap_object::HeapObject;

    impl LargePageMetadata {
        /// Returns the LargePageMetadata associated with the given HeapObject.
        pub fn from_heap_object(o: HeapObject) -> *mut LargePageMetadata {
            MutablePageMetadata::from_heap_object(o) as *mut LargePageMetadata
        }
    }
}

pub mod large_page_metadata_impl {
    // This module serves as a placeholder for the implementation
    // of the LargePageMetadata struct, which is assumed to be defined
    // elsewhere (e.g., in src/heap/large-page-metadata.h).
    // Since we only have the inline implementation file, we cannot provide
    // the full struct definition here.
}

pub mod mutable_page_metadata {
    use crate::objects::heap_object::HeapObject;
    
    pub struct MutablePageMetadata {}

    impl MutablePageMetadata {
        pub fn from_heap_object(_o: HeapObject) -> *mut MutablePageMetadata {
            //This should return a pointer to a MutablePageMetadata object
            //For the sake of compiling we are returning null.
            std::ptr::null_mut()
        }
    }
}

pub mod objects {
    pub mod heap_object {
        #[derive(Clone, Copy)]
        pub struct HeapObject {
        }
    }
}