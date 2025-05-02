// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/heap/marking-state.h

pub mod marking_state {
    use crate::common::globals::*;
    use crate::heap::marking::*;
    use crate::objects::heap_object::*;

    pub struct MemoryChunkMetadata {} // Placeholder

    pub struct MutablePageMetadata {} // Placeholder

    #[derive(Clone, Copy)]
    pub struct PtrComprCageBase {} // Placeholder

    pub trait AccessModeTrait {
        const ATOMIC: bool;
    }

    pub struct AtomicAccessMode {}
    impl AccessModeTrait for AtomicAccessMode {
        const ATOMIC: bool = true;
    }

    pub struct NonAtomicAccessMode {}
    impl AccessModeTrait for NonAtomicAccessMode {
        const ATOMIC: bool = false;
    }

    pub struct MarkingStateBase<T, A: AccessModeTrait> {
        #[cfg(v8_compress_pointers)]
        cage_base_: PtrComprCageBase,
        _phantom_data: std::marker::PhantomData<(T, A)>,
    }

    impl<T, A: AccessModeTrait> MarkingStateBase<T, A> {
        pub fn new(cage_base: PtrComprCageBase) -> Self {
            MarkingStateBase {
                #[cfg(v8_compress_pointers)]
                cage_base_: cage_base,
                _phantom_data: std::marker::PhantomData,
            }
        }

        // The pointer compression cage base value used for decompression of all
        // tagged values except references to InstructionStream objects.
        #[inline]
        pub fn cage_base(&self) -> PtrComprCageBase {
            #[cfg(v8_compress_pointers)]
            {
                self.cage_base_
            }
            #[cfg(not(v8_compress_pointers))]
            {
                PtrComprCageBase {}
            }
        }

        #[inline]
        pub fn try_mark(&self, obj: TaggedHeapObject) -> bool {
            // Placeholder implementation
            println!("try_mark called");
            true
        }
        // Helper method for fully marking an object and accounting its live bytes.
        // Should be used to mark individual objects in one-off cases.
        #[inline]
        pub fn try_mark_and_account_live_bytes(&self, obj: TaggedHeapObject) -> bool {
            // Placeholder implementation
            println!("try_mark_and_account_live_bytes called");
            true
        }
        // Same, but does not require the object to be initialized.
        #[inline]
        pub fn try_mark_and_account_live_bytes_with_size(&self, obj: TaggedHeapObject, object_size: i32) -> bool {
            // Placeholder implementation
            println!("try_mark_and_account_live_bytes_with_size called");
            true
        }

        #[inline]
        pub fn is_marked(&self, obj: &TaggedHeapObject) -> bool {
            // Placeholder implementation
            println!("is_marked called");
            false
        }
        #[inline]
        pub fn is_unmarked(&self, obj: &TaggedHeapObject) -> bool {
            // Placeholder implementation
            println!("is_unmarked called");
            true
        }
    }

    // This is used by marking visitors.
    pub struct MarkingState {
        base: MarkingStateBase<MarkingState, AtomicAccessMode>,
    }

    impl MarkingState {
        pub fn new(cage_base: PtrComprCageBase) -> Self {
            MarkingState {
                base: MarkingStateBase::new(cage_base),
            }
        }

        #[inline]
        pub fn try_mark(&self, obj: TaggedHeapObject) -> bool {
            self.base.try_mark(obj)
        }
        // Helper method for fully marking an object and accounting its live bytes.
        // Should be used to mark individual objects in one-off cases.
        #[inline]
        pub fn try_mark_and_account_live_bytes(&self, obj: TaggedHeapObject) -> bool {
            self.base.try_mark_and_account_live_bytes(obj)
        }
        // Same, but does not require the object to be initialized.
        #[inline]
        pub fn try_mark_and_account_live_bytes_with_size(&self, obj: TaggedHeapObject, object_size: i32) -> bool {
            self.base.try_mark_and_account_live_bytes_with_size(obj, object_size)
        }
        #[inline]
        pub fn is_marked(&self, obj: &TaggedHeapObject) -> bool {
            self.base.is_marked(obj)
        }
        #[inline]
        pub fn is_unmarked(&self, obj: &TaggedHeapObject) -> bool {
            self.base.is_unmarked(obj)
        }
    }

    pub struct NonAtomicMarkingState {
        base: MarkingStateBase<NonAtomicMarkingState, NonAtomicAccessMode>,
    }

    impl NonAtomicMarkingState {
        pub fn new(cage_base: PtrComprCageBase) -> Self {
            NonAtomicMarkingState {
                base: MarkingStateBase::new(cage_base),
            }
        }
        #[inline]
        pub fn try_mark(&self, obj: TaggedHeapObject) -> bool {
            self.base.try_mark(obj)
        }
        // Helper method for fully marking an object and accounting its live bytes.
        // Should be used to mark individual objects in one-off cases.
        #[inline]
        pub fn try_mark_and_account_live_bytes(&self, obj: TaggedHeapObject) -> bool {
            self.base.try_mark_and_account_live_bytes(obj)
        }
        // Same, but does not require the object to be initialized.
        #[inline]
        pub fn try_mark_and_account_live_bytes_with_size(&self, obj: TaggedHeapObject, object_size: i32) -> bool {
            self.base.try_mark_and_account_live_bytes_with_size(obj, object_size)
        }
        #[inline]
        pub fn is_marked(&self, obj: &TaggedHeapObject) -> bool {
            self.base.is_marked(obj)
        }
        #[inline]
        pub fn is_unmarked(&self, obj: &TaggedHeapObject) -> bool {
            self.base.is_unmarked(obj)
        }
    }
}