// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod maybe_object_inl {
    use crate::common::ptr_compr_inl::PtrComprCageBase;
    use crate::objects::casting::*;
    use crate::objects::maybe_object::*;
    use crate::objects::smi_inl::*;
    use crate::objects::tagged_impl_inl::*;
    use crate::objects::tagged::*;

    //use std::mem;
    //use std::marker::PhantomData;

    /// Returns a tagged cleared weak value
    pub fn cleared_value(cage_base: PtrComprCageBase) -> Tagged<ClearedWeakValue> {
        // Construct cleared weak ref value.
        let value: usize;
        #[cfg(feature = "v8_compress_pointers")]
        {
            // This is necessary to make pointer decompression computation also
            // suitable for cleared weak references.
            value = V8HeapCompressionScheme::decompress_tagged(
                cage_base,
                kClearedWeakHeapObjectLower32 as usize,
            );
        }
        #[cfg(not(feature = "v8_compress_pointers"))]
        {
            value = kClearedWeakHeapObjectLower32 as usize;
        }
        // The rest of the code will check only the lower 32-bits.
        debug_assert_eq!(kClearedWeakHeapObjectLower32 as usize, value as u32 as usize);
        Tagged::<ClearedWeakValue>::new(value)
    }

    /// Returns a tagged cleared trusted weak value.
    pub fn cleared_trusted_value() -> Tagged<ClearedWeakValue> {
        #[cfg(feature = "v8_compress_pointers")]
        {
            Tagged::<ClearedWeakValue>::new(TrustedSpaceCompressionScheme::decompress_tagged(
                TrustedSpaceCompressionScheme::base(),
                kClearedWeakHeapObjectLower32 as usize,
            ))
        }
        #[cfg(not(feature = "v8_compress_pointers"))]
        {
            Tagged::<ClearedWeakValue>::new(kClearedWeakHeapObjectLower32 as usize)
        }
    }

    /// Updates a heap object reference slot.
    pub fn update_heap_object_reference_slot<THeapObjectSlot>(
        slot: &mut THeapObjectSlot,
        value: Tagged<HeapObject>,
    ) where
        THeapObjectSlot: HeapObjectSlotTrait,
    {
        // Static assertion for `THeapObjectSlot`
        // This is a limitation, Rust does not allow checking trait bounds at compile time.
        // This can be handled by trait objects in a `dyn Trait`
        // or by using enum and implementing logic for each enum type.
        //assert!(std::any::TypeId::of::<THeapObjectSlot>() == std::any::TypeId::of::<FullHeapObjectSlot>() ||
        //       std::any::TypeId::of::<THeapObjectSlot>() == std::any::TypeId::of::<HeapObjectSlot>());

        let old_value: usize = slot.ptr();
        debug_assert!(!has_smi_tag(old_value));
        let new_value: usize = value.ptr();
        debug_assert!(Internals::has_heap_object_tag(new_value));

        #[cfg(debug_assertions)]
        let weak_before: bool = has_weak_heap_object_tag(old_value);

        slot.store(Cast::<HeapObjectReference>::from(Tagged::<MaybeObject>::new(
            new_value | (old_value & kWeakHeapObjectMask as usize),
        )));

        #[cfg(debug_assertions)]
        let weak_after: bool = has_weak_heap_object_tag(slot.ptr());
        #[cfg(debug_assertions)]
        debug_assert_eq!(weak_before, weak_after);
    }

    // Trait for HeapObjectSlot-like types
    pub trait HeapObjectSlotTrait {
        fn ptr(&self) -> usize;
        fn store(&mut self, value: Cast<HeapObjectReference>);
    }

    impl HeapObjectSlotTrait for HeapObjectSlot {
        fn ptr(&self) -> usize {
            self.value
        }
        fn store(&mut self, value: Cast<HeapObjectReference>) {
            self.value = value.value
        }
    }

    impl HeapObjectSlotTrait for FullHeapObjectSlot {
        fn ptr(&self) -> usize {
            self.value
        }
        fn store(&mut self, value: Cast<HeapObjectReference>) {
            self.value = value.value
        }
    }
}