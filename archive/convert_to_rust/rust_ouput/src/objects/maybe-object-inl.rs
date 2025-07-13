// Converted from V8 C++ source files:
// Header: maybe-object-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod maybe_object_inl {
    use std::mem::size_of;
    use std::marker::PhantomData;

    use crate::common::ptr_compr_inl::*;
    use crate::objects::casting::*;
    use crate::objects::maybe_object::*;
    use crate::objects::smi_inl::*;
    use crate::objects::tagged_impl_inl::*;
    use crate::objects::tagged::*;
    use crate::v8::internal::Address;

    pub struct ClearedWeakValue;

    pub fn cleared_value(cage_base: PtrComprCageBase) -> Tagged<ClearedWeakValue> {
        let value: Address;
        #[cfg(v8_compress_pointers)]
        {
            value = v8_heap_compression_scheme::decompress_tagged(
                cage_base,
                kClearedWeakHeapObjectLower32,
            );
        }
        #[cfg(not(v8_compress_pointers))]
        {
            value = kClearedWeakHeapObjectLower32;
        }

        debug_assert_eq!(
            kClearedWeakHeapObjectLower32,
            value as u32
        );
        Tagged::<ClearedWeakValue>(value)
    }

    pub fn cleared_trusted_value() -> Tagged<ClearedWeakValue> {
        #[cfg(v8_compress_pointers)]
        {
            Tagged::<ClearedWeakValue>(
                trusted_space_compression_scheme::decompress_tagged(
                    trusted_space_compression_scheme::base(),
                    kClearedWeakHeapObjectLower32,
                )
            )
        }
        #[cfg(not(v8_compress_pointers))]
        {
            Tagged::<ClearedWeakValue>(kClearedWeakHeapObjectLower32)
        }
    }

    pub trait HeapObjectSlotTrait {
        fn ptr(&self) -> Address;
        fn store(&self, value: HeapObjectReference);
    }

    pub struct FullHeapObjectSlot {
        address: Address,
    }

    impl FullHeapObjectSlot {
        pub fn new(address: Address) -> Self {
            FullHeapObjectSlot { address }
        }
    }

    impl HeapObjectSlotTrait for FullHeapObjectSlot {
        fn ptr(&self) -> Address {
            self.address
        }

        fn store(&self, value: HeapObjectReference) {
             unsafe {
                *(self.address as *mut HeapObjectReference) = value;
            }
        }
    }

    pub struct HeapObjectSlot {
        address: Address,
    }

    impl HeapObjectSlot {
        pub fn new(address: Address) -> Self {
            HeapObjectSlot { address }
        }
    }

    impl HeapObjectSlotTrait for HeapObjectSlot {
        fn ptr(&self) -> Address {
            self.address
        }

        fn store(&self, value: HeapObjectReference) {
            unsafe {
               *(self.address as *mut HeapObjectReference) = value;
           }
        }
    }

    pub fn update_heap_object_reference_slot<THeapObjectSlot>(
        slot: THeapObjectSlot,
        value: Tagged<HeapObject>,
    ) where THeapObjectSlot: HeapObjectSlotTrait {
        let old_value = slot.ptr();
        debug_assert!(!has_smi_tag(old_value));
        let new_value = value.ptr();
        debug_assert!(Internals::has_heap_object_tag(new_value));

        #[cfg(debug_assertions)]
        let weak_before = has_weak_heap_object_tag(old_value);

        slot.store(HeapObjectReference(
            (new_value | (old_value & kWeakHeapObjectMask)) as usize
        ));

        #[cfg(debug_assertions)]
        let weak_after = has_weak_heap_object_tag(slot.ptr());
        #[cfg(debug_assertions)]
        debug_assert_eq!(weak_before, weak_after);
    }

    pub struct Internals {}

    impl Internals {
        pub fn has_heap_object_tag(_address: Address) -> bool {
            true
        }
    }

}
