// Converted from V8 C++ source files:
// Header: N/A
// Implementation: allocation.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod cppgc {
    pub mod internal {
        const kLargeObjectSizeThreshold: usize = 2048;

        #[repr(C)]
        pub struct AlignVal {
            alignment: usize,
        }

        impl AlignVal {
            pub fn new(alignment: usize) -> Self {
                AlignVal { alignment }
            }

            pub fn alignment(&self) -> usize {
                self.alignment
            }
        }

        pub trait AllocationHandle {
            fn allocate_object(&mut self, size: usize, index: GCInfoIndex) -> *mut u8;
            fn allocate_object_aligned(
                &mut self,
                size: usize,
                alignment: AlignVal,
                index: GCInfoIndex,
            ) -> *mut u8;
            fn allocate_object_custom_space(
                &mut self,
                size: usize,
                index: GCInfoIndex,
                space_index: CustomSpaceIndex,
            ) -> *mut u8;
            fn allocate_object_aligned_custom_space(
                &mut self,
                size: usize,
                alignment: AlignVal,
                index: GCInfoIndex,
                space_index: CustomSpaceIndex,
            ) -> *mut u8;
        }

        pub struct ObjectAllocator {}

        impl ObjectAllocator {
            pub fn new() -> Self {
                ObjectAllocator {}
            }

            pub fn allocate_object(&mut self, size: usize, index: GCInfoIndex) -> *mut u8 {
                let layout = std::alloc::Layout::from_size_align(size, 8).unwrap();
                unsafe { std::alloc::alloc(layout) }
            }

            pub fn allocate_object_aligned(
                &mut self,
                size: usize,
                alignment: AlignVal,
                index: GCInfoIndex,
            ) -> *mut u8 {
                let layout =
                    std::alloc::Layout::from_size_align(size, alignment.alignment()).unwrap();
                unsafe { std::alloc::alloc(layout) }
            }

            pub fn allocate_object_custom_space(
                &mut self,
                size: usize,
                index: GCInfoIndex,
                space_index: CustomSpaceIndex,
            ) -> *mut u8 {
                let layout = std::alloc::Layout::from_size_align(size, 8).unwrap();
                unsafe { std::alloc::alloc(layout) }
            }

            pub fn allocate_object_aligned_custom_space(
                &mut self,
                size: usize,
                alignment: AlignVal,
                index: GCInfoIndex,
                space_index: CustomSpaceIndex,
            ) -> *mut u8 {
                let layout =
                    std::alloc::Layout::from_size_align(size, alignment.alignment()).unwrap();
                unsafe { std::alloc::alloc(layout) }
            }
        }

        impl AllocationHandle for ObjectAllocator {
            fn allocate_object(&mut self, size: usize, index: GCInfoIndex) -> *mut u8 {
                ObjectAllocator::allocate_object(self, size, index)
            }

            fn allocate_object_aligned(
                &mut self,
                size: usize,
                alignment: AlignVal,
                index: GCInfoIndex,
            ) -> *mut u8 {
                ObjectAllocator::allocate_object_aligned(self, size, alignment, index)
            }

            fn allocate_object_custom_space(
                &mut self,
                size: usize,
                index: GCInfoIndex,
                space_index: CustomSpaceIndex,
            ) -> *mut u8 {
                ObjectAllocator::allocate_object_custom_space(self, size, index, space_index)
            }

            fn allocate_object_aligned_custom_space(
                &mut self,
                size: usize,
                alignment: AlignVal,
                index: GCInfoIndex,
                space_index: CustomSpaceIndex,
            ) -> *mut u8 {
                ObjectAllocator::allocate_object_aligned_custom_space(
                    self,
                    size,
                    alignment,
                    index,
                    space_index,
                )
            }
        }

        pub struct MakeGarbageCollectedTraitInternal {}

        impl MakeGarbageCollectedTraitInternal {
            pub fn allocate(
                handle: &mut dyn AllocationHandle,
                size: usize,
                index: GCInfoIndex,
            ) -> *mut u8 {
                handle.allocate_object(size, index)
            }

            pub fn allocate_aligned(
                handle: &mut dyn AllocationHandle,
                size: usize,
                alignment: AlignVal,
                index: GCInfoIndex,
            ) -> *mut u8 {
                handle.allocate_object_aligned(size, alignment, index)
            }

            pub fn allocate_custom_space(
                handle: &mut dyn AllocationHandle,
                size: usize,
                index: GCInfoIndex,
                space_index: CustomSpaceIndex,
            ) -> *mut u8 {
                handle.allocate_object_custom_space(size, index, space_index)
            }

            pub fn allocate_aligned_custom_space(
                handle: &mut dyn AllocationHandle,
                size: usize,
                alignment: AlignVal,
                index: GCInfoIndex,
                space_index: CustomSpaceIndex,
            ) -> *mut u8 {
                handle.allocate_object_aligned_custom_space(
                    size,
                    alignment,
                    index,
                    space_index,
                )
            }
        }
    }

    pub struct GCInfoIndex {}

    pub struct CustomSpaceIndex {}

    pub trait AllocationHandle {
        fn allocate_object(&mut self, size: usize, index: GCInfoIndex) -> *mut u8;
        fn allocate_object_aligned(
            &mut self,
            size: usize,
            alignment: internal::AlignVal,
            index: GCInfoIndex,
        ) -> *mut u8;
        fn allocate_object_custom_space(
            &mut self,
            size: usize,
            index: GCInfoIndex,
            space_index: CustomSpaceIndex,
        ) -> *mut u8;
        fn allocate_object_aligned_custom_space(
            &mut self,
            size: usize,
            alignment: internal::AlignVal,
            index: GCInfoIndex,
            space_index: CustomSpaceIndex,
        ) -> *mut u8;
    }
}
