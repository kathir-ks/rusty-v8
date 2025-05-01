// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod allocation {
    use std::alloc::{Layout, LayoutError};
    use std::any::Any;
    use std::marker::PhantomData;
    use std::mem;
    use std::sync::atomic::{AtomicU16, Ordering};
    use std::ptr::NonNull;

    //use crate::base::export::V8_EXPORT; // Assuming this is in a separate module
    use crate::cppgc::custom_space::{CustomSpaceBase, CustomSpaceIndex};
    use crate::cppgc::gc_info::{GCInfoFolding, GCInfoIndex, GCInfoTrait};
    use crate::cppgc::type_traits::{IsGarbageCollectedType, IsGarbageCollectedWithMixinTypeV};
    use crate::cppgc::space_trait::SpaceTrait;

    pub mod api_constants {
        pub const K_DEFAULT_ALIGNMENT: usize = 8;
        pub const K_FULLY_CONSTRUCTED_BIT_MASK: u16 = 1;
        pub const K_FULLY_CONSTRUCTED_BIT_FIELD_OFFSET_FROM_PAYLOAD: isize = 2;
        pub const K_LARGE_OBJECT_SIZE_THRESHOLD: usize = 2048;
        pub const K_MAX_SUPPORTED_ALIGNMENT: usize = 128; //alignof(std::max_align_t) = 16 on x86_64, 8 on 32bit. Adjusted to 128 as a safe bet.
    }

    /// AllocationHandle is used to allocate garbage-collected objects.
    pub struct AllocationHandle;

    mod internal {
        use super::*;

        pub type AlignVal = usize;

        pub struct MakeGarbageCollectedTraitInternal;

        impl MakeGarbageCollectedTraitInternal {
            pub fn mark_object_as_fully_constructed(payload: *const ()) {
                // See api_constants for an explanation of the constants.
                let atomic_mutable_bitfield = unsafe {
                    let payload_ptr = payload as *const u8;
                    let bitfield_ptr = payload_ptr.offset(-api_constants::K_FULLY_CONSTRUCTED_BIT_FIELD_OFFSET_FROM_PAYLOAD) as *mut AtomicU16;
                    &mut *bitfield_ptr
                };
                // It's safe to split use load+store here (instead of a read-modify-write
                // operation), since it's guaranteed that this 16-bit bitfield is only
                // modified by a single thread. This is cheaper in terms of code bloat (on
                // ARM) and performance.
                let mut value = atomic_mutable_bitfield.load(Ordering::Relaxed);
                value |= api_constants::K_FULLY_CONSTRUCTED_BIT_MASK;
                atomic_mutable_bitfield.store(value, Ordering::Release);
            }
        }

        // Dispatch based on compile-time information.
        //
        // Default implementation is for a custom space with >`kDefaultAlignment` byte
        // alignment.
        pub struct AllocationDispatcher<GCInfoType, CustomSpace, const ALIGNMENT: usize> {
            _gc_info: PhantomData<GCInfoType>,
            _custom_space: PhantomData<CustomSpace>,
        }

        impl<GCInfoType, CustomSpace, const ALIGNMENT: usize> AllocationDispatcher<GCInfoType, CustomSpace, ALIGNMENT> {
            pub fn invoke(handle: &mut AllocationHandle, size: usize) -> *mut ()
            where
                CustomSpace: CustomSpaceBase,
            {
                assert!(ALIGNMENT > api_constants::K_DEFAULT_ALIGNMENT);
                assert!(CustomSpace::supports_compaction() == false,
                           "Custom spaces that support compaction do not support allocating objects with non-default (i.e. word-sized) alignment.");
                MakeGarbageCollectedTraitInternal::allocate(
                    handle,
                    size,
                    ALIGNMENT,
                    GCInfoTrait::<GCInfoType>::index(),
                    CustomSpace::space_index(),
                )
            }
        }

        // Fast path for regular allocations for the default space with
        // `kDefaultAlignment` byte alignment.
        impl<GCInfoType> AllocationDispatcher<GCInfoType, (), { api_constants::K_DEFAULT_ALIGNMENT }> {
            pub fn invoke(handle: &mut AllocationHandle, size: usize) -> *mut () {
                MakeGarbageCollectedTraitInternal::allocate_default(handle, size, GCInfoTrait::<GCInfoType>::index())
            }
        }

        // Default space with >`kDefaultAlignment` byte alignment.
        impl<GCInfoType, const ALIGNMENT: usize> AllocationDispatcher<GCInfoType, (), ALIGNMENT> {
            pub fn invoke(handle: &mut AllocationHandle, size: usize) -> *mut () {
                assert!(ALIGNMENT > api_constants::K_DEFAULT_ALIGNMENT);
                MakeGarbageCollectedTraitInternal::allocate_aligned(handle, size, ALIGNMENT, GCInfoTrait::<GCInfoType>::index())
            }
        }

        // Custom space with `kDefaultAlignment` byte alignment.
        impl<GCInfoType, CustomSpace> AllocationDispatcher<GCInfoType, CustomSpace, { api_constants::K_DEFAULT_ALIGNMENT }> {
            pub fn invoke(handle: &mut AllocationHandle, size: usize) -> *mut ()
            where
                CustomSpace: CustomSpaceBase,
            {
                assert!(CustomSpace::supports_compaction() == false,
                           "Custom spaces that support compaction do not support allocating objects with non-default (i.e. word-sized) alignment.");
                MakeGarbageCollectedTraitInternal::allocate_custom(
                    handle,
                    size,
                    GCInfoTrait::<GCInfoType>::index(),
                    CustomSpace::space_index(),
                )
            }
        }

        impl MakeGarbageCollectedTraitInternal {
            // These functions are placeholders. Implement the actual allocation logic
            // based on the V8's memory management.

            // #[V8_EXPORT]
            pub fn allocate_default(
                _handle: &mut AllocationHandle,
                size: usize,
                _gc_info_index: GCInfoIndex,
            ) -> *mut () {
                // Placeholder: Implement default allocation logic here.
                unsafe {
                    let layout = Layout::from_size_align(size, api_constants::K_DEFAULT_ALIGNMENT).unwrap();
                    std::alloc::alloc(layout) as *mut ()
                }
            }

            // #[V8_EXPORT]
            pub fn allocate_aligned(
                _handle: &mut AllocationHandle,
                size: usize,
                alignment: usize,
                _gc_info_index: GCInfoIndex,
            ) -> *mut () {
                // Placeholder: Implement aligned allocation logic here.
                unsafe {
                    let layout = Layout::from_size_align(size, alignment).unwrap();
                    std::alloc::alloc(layout) as *mut ()
                }
            }

            // #[V8_EXPORT]
            pub fn allocate_custom(
                _handle: &mut AllocationHandle,
                size: usize,
                _gc_info_index: GCInfoIndex,
                _custom_space_index: CustomSpaceIndex,
            ) -> *mut () {
                // Placeholder: Implement custom space allocation logic here.
                unsafe {
                    let layout = Layout::from_size_align(size, api_constants::K_DEFAULT_ALIGNMENT).unwrap();
                    std::alloc::alloc(layout) as *mut ()
                }
            }

            // #[V8_EXPORT]
            pub fn allocate(
                _handle: &mut AllocationHandle,
                size: usize,
                alignment: usize,
                _gc_info_index: GCInfoIndex,
                _custom_space_index: CustomSpaceIndex,
            ) -> *mut () {
                // Placeholder: Implement custom space allocation logic here.
                unsafe {
                    let layout = Layout::from_size_align(size, alignment).unwrap();
                    std::alloc::alloc(layout) as *mut ()
                }
            }
        }
    }

    /// Base trait that provides utilities for advancers users that have custom
    /// allocation needs (e.g., overriding size). It's expected that users override
    /// MakeGarbageCollectedTrait (see below) and inherit from
    /// MakeGarbageCollectedTraitBase and make use of the low-level primitives
    /// offered to allocate and construct an object.
    pub struct MakeGarbageCollectedTraitBase<T> {
        _phantom: PhantomData<T>,
    }

    impl<T> MakeGarbageCollectedTraitBase<T>
    where
        T: 'static,
    {
        /// Allocates memory for an object of type T.
        ///
        /// # Arguments
        ///
        /// * `handle` - AllocationHandle identifying the heap to allocate the object on.
        /// * `size` - The size that should be reserved for the object.
        ///
        /// # Returns
        ///
        /// The memory to construct an object of type T on.
        #[inline]
        pub fn allocate(handle: &mut AllocationHandle, size: usize) -> *mut ()
        where
            T: SpaceTrait + IsGarbageCollectedType,
            <T as SpaceTrait>::Space: CustomSpaceBase,
        {
            assert!(std::mem::size_of::<T>() <= api_constants::K_LARGE_OBJECT_SIZE_THRESHOLD || !IsGarbageCollectedWithMixinTypeV::<T>);
            assert!(<T as SpaceTrait>::alignment() <= api_constants::K_MAX_SUPPORTED_ALIGNMENT,
                "Requested alignment larger than alignof(std::max_align_t) bytes. Please file a bug to possibly get this restriction lifted.");

            <internal::AllocationDispatcher<
                typename::GCInfoFolding<
                   T,
                   <T as SpaceTrait>::ParentMostGarbageCollectedType,
                >::ResultType,
                <T as SpaceTrait>::Space,
                {<T as SpaceTrait>::alignment()}
            > as Default>::default().invoke(handle, size)
        }

        /// Marks an object as fully constructed, resulting in precise handling by the
        /// garbage collector.
        ///
        /// # Arguments
        ///
        /// * `payload` - The base pointer the object is allocated at.
        #[inline]
        pub fn mark_object_as_fully_constructed(payload: *const ()) {
            internal::MakeGarbageCollectedTraitInternal::mark_object_as_fully_constructed(
                payload,
            );
        }
    }

    /// Passed to MakeGarbageCollected to specify how many bytes should be appended
    /// to the allocated object.
    ///
    /// Example:
    /// ```ignore
    /// class InlinedArray final : public GarbageCollected<InlinedArray> {
    ///  public:
    ///   explicit InlinedArray(size_t bytes) : size(bytes), byte_array(this + 1) {}
    ///   void Trace(Visitor*) const {}
    ///
    ///   size_t size;
    ///   char* byte_array;
    /// };
    ///
    /// auto* inlined_array = MakeGarbageCollected<InlinedArray(
    ///    GetAllocationHandle(), AdditionalBytes(4), 4);
    /// for (size_t i = 0; i < 4; i++) {
    ///   Process(inlined_array->byte_array[i]);
    /// }
    /// ```
    #[derive(Copy, Clone)]
    pub struct AdditionalBytes {
        pub value: usize,
    }

    impl AdditionalBytes {
        pub const fn new(bytes: usize) -> Self {
            Self { value: bytes }
        }
    }

    /// Default trait class that specifies how to construct an object of type T.
    /// Advanced users may override how an object is constructed using the utilities
    /// that are provided through MakeGarbageCollectedTraitBase.
    ///
    /// Any trait overriding construction must
    /// - allocate through `MakeGarbageCollectedTraitBase<T>::Allocate`;
    /// - mark the object as fully constructed using
    ///   `MakeGarbageCollectedTraitBase<T>::MarkObjectAsFullyConstructed`;
    pub struct MakeGarbageCollectedTrait<T> {
        _phantom: PhantomData<T>,
    }

    impl<T> MakeGarbageCollectedTrait<T>
    where
        T: 'static + SpaceTrait + IsGarbageCollectedType,
        <T as SpaceTrait>::Space: CustomSpaceBase,
    {
        pub fn call<Args>(handle: &mut AllocationHandle, args: Args) -> *mut T
        where
            Args: Construct<T>,
        {
            let memory = MakeGarbageCollectedTraitBase::<T>::allocate(
                handle,
                mem::size_of::<T>(),
            );
            let object = unsafe {
                let ptr = memory as *mut T;
                args.construct(ptr);
                ptr
            };
            MakeGarbageCollectedTraitBase::<T>::mark_object_as_fully_constructed(
                object as *const T as *const (),
            );
            object
        }

        pub fn call_with_additional_bytes<Args>(
            handle: &mut AllocationHandle,
            additional_bytes: AdditionalBytes,
            args: Args,
        ) -> *mut T
        where
            Args: Construct<T>,
        {
            let memory = MakeGarbageCollectedTraitBase::<T>::allocate(
                handle,
                mem::size_of::<T>() + additional_bytes.value,
            );
            let object = unsafe {
                let ptr = memory as *mut T;
                args.construct(ptr);
                ptr
            };
            MakeGarbageCollectedTraitBase::<T>::mark_object_as_fully_constructed(
                object as *const T as *const (),
            );
            object
        }
    }

    /// Allows users to specify a post-construction callback for specific types. The
    /// callback is invoked on the instance of type T right after it has been
    /// constructed. This can be useful when the callback requires a
    /// fully-constructed object to be able to dispatch to virtual methods.
    pub trait PostConstructionCallbackTrait<T> {
        fn call(object: *mut T);
    }

    impl<T> PostConstructionCallbackTrait<T> for () {
        default fn call(_object: *mut T) {}
    }

    /// Constructs a managed object of type T where T transitively inherits from
    /// GarbageCollected.
    ///
    /// # Arguments
    ///
    /// * `args` - List of arguments with which an instance of T will be constructed.
    ///
    /// # Returns
    ///
    /// An instance of type T.
    #[inline]
    pub fn make_garbage_collected<T, Args>(
        handle: &mut AllocationHandle,
        args: Args,
    ) -> *mut T
    where
        T: 'static + SpaceTrait + IsGarbageCollectedType,
        <T as SpaceTrait>::Space: CustomSpaceBase,
        Args: Construct<T>,
        (): PostConstructionCallbackTrait<T>,
    {
        let object = MakeGarbageCollectedTrait::<T>::call(handle, args);
        <() as PostConstructionCallbackTrait<T>>::call(object);
        object
    }

    /// Constructs a managed object of type T where T transitively inherits from
    /// GarbageCollected. Created objects will have additional bytes appended to
    /// it. Allocated memory would suffice for `sizeof(T) + additional_bytes`.
    ///
    /// # Arguments
    ///
    /// * `additional_bytes` - Denotes how many bytes to append to T.
    /// * `args` - List of arguments with which an instance of T will be constructed.
    ///
    /// # Returns
    ///
    /// An instance of type T.
    #[inline]
    pub fn make_garbage_collected_with_additional_bytes<T, Args>(
        handle: &mut AllocationHandle,
        additional_bytes: AdditionalBytes,
        args: Args,
    ) -> *mut T
    where
        T: 'static + SpaceTrait + IsGarbageCollectedType,
        <T as SpaceTrait>::Space: CustomSpaceBase,
        Args: Construct<T>,
        (): PostConstructionCallbackTrait<T>,
    {
        let object = MakeGarbageCollectedTrait::<T>::call_with_additional_bytes(
            handle,
            additional_bytes,
            args,
        );
        <() as PostConstructionCallbackTrait<T>>::call(object);
        object
    }

    pub trait Construct<T> {
        unsafe fn construct(self, ptr: *mut T);
    }

    impl<T, F: FnOnce(*mut T)> Construct<T> for F {
        unsafe fn construct(self, ptr: *mut T) {
            self(ptr)
        }
    }
}

pub mod custom_space {
    use crate::cppgc::gc_info::GCInfoIndex;

    pub trait CustomSpaceBase {
        fn supports_compaction() -> bool;
        fn space_index() -> CustomSpaceIndex;
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct CustomSpaceIndex(pub usize);
}

pub mod gc_info {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct GCInfoIndex(pub usize);

    pub struct GCInfoTrait<T> {
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> GCInfoTrait<T> {
        pub const fn index() -> GCInfoIndex {
            GCInfoIndex(0) // Placeholder
        }
    }

    pub struct GCInfoFolding<T, U> {
        _phantom: std::marker::PhantomData<(T, U)>,
    }

    impl<T, U> GCInfoFolding<T, U> {
        pub type ResultType = T; // Placeholder
    }
}

pub mod type_traits {
    pub trait IsGarbageCollectedType: 'static {
    }

    pub type IsGarbageCollectedWithMixinTypeV<T> = false;
}

pub mod space_trait {
    use crate::cppgc::custom_space::CustomSpaceBase;
    use crate::cppgc::allocation::api_constants;

    pub trait SpaceTrait {
        type Space: CustomSpaceBase;
        type ParentMostGarbageCollectedType: SpaceTrait;

        const fn alignment() -> usize {
            api_constants::K_DEFAULT_ALIGNMENT
        }
    }
}