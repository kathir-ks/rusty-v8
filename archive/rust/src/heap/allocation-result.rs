// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod internal {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum AllocationOrigin {
        GeneratedCode = 0,
        Runtime = 1,
        GC = 2,
    }

    impl AllocationOrigin {
        pub const FIRST: AllocationOrigin = AllocationOrigin::GeneratedCode;
        pub const LAST: AllocationOrigin = AllocationOrigin::GC;
        pub const NUMBER_OF_ORIGINS: usize = AllocationOrigin::LAST as usize + 1;
    }

    // Placeholder for Tagged<HeapObject>.  Needs proper definition based on the V8 object model.
    #[derive(Copy, Clone, Debug)]
    pub struct HeapObject {
        address: usize,
    }

    impl HeapObject {
        pub fn address(&self) -> usize {
            self.address
        }
        pub fn is_null(&self) -> bool {
            self.address == 0
        }
    }

    // Placeholder for Tagged<T>. Needs proper definition based on the V8 object model.
    #[derive(Copy, Clone, Debug)]
    pub struct Tagged<T> {
        object: T,
    }

    impl<T> Tagged<T> {
        pub fn new(object: T) -> Self {
            Tagged { object }
        }

        pub fn object(&self) -> &T {
            &self.object
        }

        pub fn is_null(&self) -> bool {
            //  This assumes the HeapObject is being Tagged.  May need to adapt if other types are Tagged.
            if let Some(heap_object) = (self.object() as *const T as *const HeapObject).as_ref() {
                heap_object.is_null()
            } else {
                true // Handle the case where casting fails (though this shouldn't normally happen if T is intended to be HeapObject)
            }

        }
    }

    pub struct AllocationResult {
        object_: Tagged<HeapObject>,
    }

    impl AllocationResult {
        pub fn failure() -> Self {
            AllocationResult {
                object_: Tagged::new(HeapObject { address: 0 }),
            }
        }

        pub fn from_object(heap_object: Tagged<HeapObject>) -> Self {
            AllocationResult { object_: heap_object }
        }

        pub fn is_failure(&self) -> bool {
            self.object_.is_null()
        }

        pub fn to<T>(&self) -> Option<Tagged<T>> {
            if self.is_failure() {
                return None;
            }

            // This is a very unsafe cast, relying on the C++ object model and Tagged assumptions.
            // Needs careful consideration for soundness.  In a real implementation, we'd have
            // proper type checks and casting mechanisms.
            unsafe {
                let ptr = &self.object_.object as *const HeapObject as *const T;
                let obj = ptr.read(); //Potentially UB if T and HeapObject aren't compatible
                Some(Tagged::new(obj))
            }
        }

        pub fn to_object_checked(&self) -> Tagged<HeapObject> {
            assert!(!self.is_failure());
            self.to_object()
        }

        pub fn to_object(&self) -> Tagged<HeapObject> {
            assert!(!self.is_failure());

            unsafe {
                let ptr = &self.object_.object as *const HeapObject;
                let obj = ptr.read();
                Tagged::new(obj)
            }
        }

        pub fn to_address(&self) -> usize {
            assert!(!self.is_failure());
            self.object_.object().address()
        }
    }

    impl From<AllocationResult> for bool {
        fn from(val: AllocationResult) -> Self {
            !val.is_failure()
        }
    }

    const SYSTEM_POINTER_SIZE: usize = std::mem::size_of::<usize>();
    const _: () = assert!(std::mem::size_of::<AllocationResult>() == SYSTEM_POINTER_SIZE);
}

pub use internal::*;