// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod primitive_heap_object {
    use crate::objects::heap_object::HeapObject;
    use std::mem;
    use std::marker::PhantomData;

    /// An abstract superclass for classes representing JavaScript primitive values
    /// other than Smi. It doesn't carry any functionality but allows primitive
    /// classes to be identified in the type system.
    #[repr(C)]
    #[derive(Debug)]
    pub struct PrimitiveHeapObject {
        // This field is added only to ensure that the size is identical to HeapObjectLayout.
        // In reality, PrimitiveHeapObject inherits from HeapObjectLayout which in turns inherit from HeapObject.
        // This ensures that they have same layout and are interchangeable.
        _phantom: PhantomData<HeapObject>,
    }

    impl PrimitiveHeapObject {
        // TODO: Add DECL_VERIFIER macro functionality if verification is needed.
    }

    // Ensure that the size of PrimitiveHeapObject is the same as HeapObject.
    const _: () = assert!(mem::size_of::<PrimitiveHeapObject>() == mem::size_of::<HeapObject>());

    // Ensure that PrimitiveHeapObject is a subtype of HeapObject.
    // In rust terms, this means ensuring that PrimitiveHeapObject and HeapObject are interchangeable.
    // This is achieved through the `PhantomData` and layout compatibility of `repr(C)`.
    // TODO: Add static_assert is_subtype_v<PrimitiveHeapObject, HeapObject> functionality if needed.
    //  Note: We can achieve this through trait bounds, or phantom types.

    impl From<PrimitiveHeapObject> for HeapObject {
        fn from(obj: PrimitiveHeapObject) -> Self {
            // Safety: PrimitiveHeapObject and HeapObject have the same layout.
            unsafe { std::mem::transmute_copy(&obj) }
        }
    }

    impl From<&PrimitiveHeapObject> for &HeapObject {
        fn from(obj: &PrimitiveHeapObject) -> Self {
            // Safety: PrimitiveHeapObject and HeapObject have the same layout.
            unsafe { std::mem::transmute(obj) }
        }
    }
}