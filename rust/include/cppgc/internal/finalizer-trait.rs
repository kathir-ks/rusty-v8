// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod internal {
    use std::{
        any::Any,
        mem,
        ops::Drop,
        ptr::NonNull,
        marker::PhantomData,
    };

    pub type FinalizationCallback = unsafe fn(*mut dyn Any);

    // Helper trait to detect if a type has a `FinalizeGarbageCollectedObject` method.
    pub trait HasFinalizeGarbageCollectedObject {
        fn finalize_garbage_collected_object(&mut self);
    }

    // Blanket implementation for types that have the method.
    impl<T: FinalizeGarbageCollectedObject> HasFinalizeGarbageCollectedObject for T {
        fn finalize_garbage_collected_object(&mut self) {
            FinalizeGarbageCollectedObject::finalize_garbage_collected_object(self)
        }
    }

    pub trait FinalizeGarbageCollectedObject {
        fn finalize_garbage_collected_object(&mut self);
    }

    // The FinalizerTrait specifies how to finalize objects.
    pub trait FinalizerTrait {
        const HAS_FINALIZER: bool;
        const CALLBACK: Option<FinalizationCallback>;
    }

    // Implementation of FinalizerTrait for types with a finalizer.
    pub struct FinalizerTraitImpl<T> {
        _marker: PhantomData<T>,
    }

    impl<T> FinalizerTraitImpl<T> {
        unsafe fn call_custom(obj: *mut dyn Any) {
            let obj = (obj as *mut T).as_mut().unwrap();
            <T as FinalizeGarbageCollectedObject>::finalize_garbage_collected_object(obj);
        }

        unsafe fn call_destructor(obj: *mut dyn Any) {
            let obj_ptr = obj as *mut T;
            // Manually drop the object without deallocating the memory.
            // This emulates the C++ placement delete behavior.
            std::ptr::drop_in_place(obj_ptr);
        }
    }

    impl<T: 'static + FinalizeGarbageCollectedObject> FinalizerTrait for T {
        const HAS_FINALIZER: bool = true;
        const CALLBACK: Option<FinalizationCallback> = Some(|obj| unsafe {
            FinalizerTraitImpl::<T>::call_custom(obj);
        });
    }

    impl<T: 'static> FinalizerTrait for T
    where T: Drop,
          T: !FinalizeGarbageCollectedObject
    {
        const HAS_FINALIZER: bool = true;
        const CALLBACK: Option<FinalizationCallback> = Some(|obj| unsafe {
            FinalizerTraitImpl::<T>::call_destructor(obj);
        });
    }

    impl<T: 'static> FinalizerTrait for T
    where T: !Drop,
          T: !FinalizeGarbageCollectedObject
    {
        const HAS_FINALIZER: bool = false;
        const CALLBACK: Option<FinalizationCallback> = None;
    }

    pub fn finalize<T: 'static>(obj: *mut dyn Any) {
        if <T as FinalizerTrait>::HAS_FINALIZER {
            if let Some(callback) = <T as FinalizerTrait>::CALLBACK {
                 unsafe { callback(obj) };
            }
        }
    }
}