// Converted from V8 C++ source files:
// Header: allocation.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod cppgc {
pub struct AllocationHandle {}

pub mod internal {
    use std::alloc::{self, Layout};
    use std::sync::atomic::{AtomicU16, Ordering};
    use crate::cppgc::AllocationHandle;

    pub type AlignVal = alloc::Alignment;

    pub struct MakeGarbageCollectedTraitInternal {}

    impl MakeGarbageCollectedTraitInternal {
        pub(crate) fn mark_object_as_fully_constructed(payload: *const std::ffi::c_void) {
            // See api_constants for an explanation of the constants.
            unsafe {
                let atomic_mutable_bitfield = (payload as *mut u8)
                    .offset(-(api_constants::kFullyConstructedBitFieldOffsetFromPayload as isize))
                    as *mut AtomicU16;
                // It's safe to split use load+store here (instead of a read-modify-write
                // operation), since it's guaranteed that this 16-bit bitfield is only
                // modified by a single thread. This is cheaper in terms of code bloat (on
                // ARM) and performance.
                let mut value = (*atomic_mutable_bitfield).load(Ordering::Relaxed);
                value |= api_constants::kFullyConstructedBitMask;
                (*atomic_mutable_bitfield).store(value, Ordering::Release);
            }
        }
    }

    pub struct AllocationDispatcher {}

    impl AllocationDispatcher {
        pub fn invoke<GCInfoType, CustomSpace, const ALIGNMENT: usize>(
            handle: &mut AllocationHandle,
            size: usize,
        ) -> *mut std::ffi::c_void {
            // This is a dummy implementation as full type information is not available.
            // A real implementation would dispatch based on the type parameters.
            // The static asserts from the C++ code are not enforced here.
            unsafe {
                MakeGarbageCollectedTraitInternal::allocate_default(handle, size)
            }
        }
    }
    impl MakeGarbageCollectedTraitInternal {
        #[allow(unused_variables)]
        pub(crate) unsafe fn allocate_default(
            handle: &mut AllocationHandle,
            size: usize,
        ) -> *mut std::ffi::c_void {
            let layout = Layout::from_size_align(size, api_constants::kDefaultAlignment)
                .expect("Failed to create layout");
            alloc::alloc(layout) as *mut std::ffi::c_void
        }

        #[allow(unused_variables)]
        pub(crate) unsafe fn allocate_aligned(
            handle: &mut AllocationHandle,
            size: usize,
            align: AlignVal,
        ) -> *mut std::ffi::c_void {
            let layout = Layout::from_size_align(size, align.as_usize()).expect("Failed to create layout");
            alloc::alloc(layout) as *mut std::ffi::c_void
        }

        #[allow(unused_variables)]
        pub(crate) unsafe fn allocate_custom_space(
            handle: &mut AllocationHandle,
            size: usize,
            space_index: CustomSpaceIndex,
        ) -> *mut std::ffi::c_void {
            let layout = Layout::from_size_align(size, api_constants::kDefaultAlignment)
                .expect("Failed to create layout");
            alloc::alloc(layout) as *mut std::ffi::c_void
        }

        #[allow(unused_variables)]
        pub(crate) unsafe fn allocate_aligned_custom_space(
            handle: &mut AllocationHandle,
            size: usize,
            align: AlignVal,
            space_index: CustomSpaceIndex,
        ) -> *mut std::ffi::c_void {
            let layout = Layout::from_size_align(size, align.as_usize()).expect("Failed to create layout");
            alloc::alloc(layout) as *mut std::ffi::c_void
        }

        pub(crate) unsafe fn allocate(
            handle: &mut AllocationHandle,
            size: usize,
            gc_info_index: GCInfoIndex,
        ) -> *mut std::ffi::c_void {
            let layout = Layout::from_size_align(size, api_constants::kDefaultAlignment)
                .expect("Failed to create layout");
            alloc::alloc(layout) as *mut std::ffi::c_void
        }

        pub(crate) unsafe fn allocate_aligned_gcinfo(
            handle: &mut AllocationHandle,
            size: usize,
            align: AlignVal,
            gc_info_index: GCInfoIndex,
        ) -> *mut std::ffi::c_void {
            let layout = Layout::from_size_align(size, align.as_usize()).expect("Failed to create layout");
            alloc::alloc(layout) as *mut std::ffi::c_void
        }

        pub(crate) unsafe fn allocate_gcinfo_customspace(
            handle: &mut AllocationHandle,
            size: usize,
            gc_info_index: GCInfoIndex,
            custom_space_index: CustomSpaceIndex,
        ) -> *mut std::ffi::c_void {
            let layout = Layout::from_size_align(size, api_constants::kDefaultAlignment)
                .expect("Failed to create layout");
            alloc::alloc(layout) as *mut std::ffi::c_void
        }

        pub(crate) unsafe fn allocate_aligned_gcinfo_customspace(
            handle: &mut AllocationHandle,
            size: usize,
            align: AlignVal,
            gc_info_index: GCInfoIndex,
            custom_space_index: CustomSpaceIndex,
        ) -> *mut std::ffi::c_void {
            let layout = Layout::from_size_align(size, align.as_usize()).expect("Failed to create layout");
            alloc::alloc(layout) as *mut std::ffi::c_void
        }
    }

    #[derive(Clone, Copy)]
    pub struct GCInfoIndex(usize);
    #[derive(Clone, Copy)]
    pub struct CustomSpaceIndex(usize);

    pub struct GCInfoTrait<T> {
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> GCInfoTrait<T> {
        pub fn index() -> GCInfoIndex {
            GCInfoIndex(0) // Dummy implementation
        }
    }

    pub struct CustomSpaceBase {}

    pub struct SpaceTrait<T> {
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> SpaceTrait<T> {
        pub type Space = Void;
    }
    pub enum Void {}

    pub struct GCInfoFolding<T, U> {
        _phantom: std::marker::PhantomData<(T, U)>,
    }

    impl<T, U> GCInfoFolding<T, U> {
        pub type ResultType = T;
    }

    pub mod api_constants {
        pub const kDefaultAlignment: usize = 8;
        pub const kMaxSupportedAlignment: usize = 16;
        pub const kFullyConstructedBitFieldOffsetFromPayload: usize = 2;
        pub const kFullyConstructedBitMask: u16 = 1;
        pub const kLargeObjectSizeThreshold: usize = 2048;
    }

    pub trait IsGarbageCollectedType<T> {
        const VALUE: bool;
    }

    impl<T> IsGarbageCollectedType<T> for T {
        const VALUE: bool = true;
    }
}

pub struct MakeGarbageCollectedTraitBase<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> MakeGarbageCollectedTraitBase<T> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
    #[allow(unused_variables)]
    #[inline]
    pub fn allocate(handle: &mut AllocationHandle, size: usize) -> *mut std::ffi::c_void {
        let _ = <T as internal::IsGarbageCollectedType<T>>::VALUE;
        let _ = size;
        let alignment = std::mem::align_of::<T>();
        let wanted_alignment = if alignment < internal::api_constants::kDefaultAlignment {
            internal::api_constants::kDefaultAlignment
        } else {
            alignment
        };
        if wanted_alignment > internal::api_constants::kMaxSupportedAlignment {
            panic!("Requested alignment larger than maximum supported alignment.");
        }

        internal::AllocationDispatcher::invoke::<
            typename<<T as ParentMostGarbageCollectedType>::ParentMostGarbageCollectedType as internal::GCInfoFolding<T, <T as ParentMostGarbageCollectedType>::ParentMostGarbageCollectedType>>::ResultType,
            typename<<T as ParentMostGarbageCollectedType>::ParentMostGarbageCollectedType as internal::SpaceTrait<<T as ParentMostGarbageCollectedType>::ParentMostGarbageCollectedType>>::Space,
            { wanted_alignment }>(handle, size)
    }

    #[inline]
    pub fn mark_object_as_fully_constructed(payload: *const std::ffi::c_void) {
        internal::MakeGarbageCollectedTraitInternal::mark_object_as_fully_constructed(payload);
    }
}

pub struct AdditionalBytes {
    pub value: usize,
}

impl AdditionalBytes {
    pub const fn new(bytes: usize) -> Self {
        Self { value: bytes }
    }
}

pub struct MakeGarbageCollectedTrait<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> MakeGarbageCollectedTrait<T> {
    #[allow(unused_variables)]
    pub fn call<Args>(handle: &mut AllocationHandle, args: Args) -> *mut T {
        let memory =
            MakeGarbageCollectedTraitBase::<T>::allocate(handle, std::mem::size_of::<T>());
        let object = unsafe {
            let ptr = memory as *mut T;
            ptr.write(std::mem::zeroed()); // Initialize the memory
            ptr
        };
        MakeGarbageCollectedTraitBase::<T>::mark_object_as_fully_constructed(object as *const T as *const std::ffi::c_void);
        object
    }

    #[allow(unused_variables, clippy::too_many_arguments)]
    pub fn call_with_additional_bytes<Args>(
        handle: &mut AllocationHandle,
        additional_bytes: AdditionalBytes,
        args: Args,
    ) -> *mut T {
        let memory = MakeGarbageCollectedTraitBase::<T>::allocate(
            handle,
            std::mem::size_of::<T>() + additional_bytes.value,
        );
        let object = unsafe {
            let ptr = memory as *mut T;
            ptr.write(std::mem::zeroed()); // Initialize the memory
            ptr
        };
        MakeGarbageCollectedTraitBase::<T>::mark_object_as_fully_constructed(object as *const T as *const std::ffi::c_void);
        object
    }
}

pub struct PostConstructionCallbackTrait<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> PostConstructionCallbackTrait<T> {
    pub fn call(_object: *mut T) {}
}

pub unsafe fn make_garbage_collected<T>(handle: &mut AllocationHandle) -> *mut T {
    let object = MakeGarbageCollectedTrait::<T>::call(handle, ());
    PostConstructionCallbackTrait::<T>::call(object);
    object
}

pub unsafe fn make_garbage_collected_with_additional_bytes<T>(
    handle: &mut AllocationHandle,
    additional_bytes: AdditionalBytes,
) -> *mut T {
    let object =
        MakeGarbageCollectedTrait::<T>::call_with_additional_bytes(handle, additional_bytes, ());
    PostConstructionCallbackTrait::<T>::call(object);
    object
}

pub trait ParentMostGarbageCollectedType {
    type ParentMostGarbageCollectedType;
}

pub trait IsGarbageCollectedWithMixinType {
    const VALUE: bool;
}

impl<T> IsGarbageCollectedWithMixinType for T {
    const VALUE: bool = false;
}
pub type IsGarbageCollectedWithMixinTypeV<T> = <T as IsGarbageCollectedWithMixinType>::VALUE;
}
