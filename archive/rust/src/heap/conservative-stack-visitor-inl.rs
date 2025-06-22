// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/heap/conservative-stack-visitor.h conversion is assumed to be in conservative_stack_visitor module

mod conservative_stack_visitor_inl {
    use crate::heap::conservative_stack_visitor::*;
    use crate::common::globals::*;
    use crate::execution::isolate::*;
    use crate::heap::heap_layout::*;
    use crate::heap::marking::*;
    use crate::heap::memory_chunk_metadata::*;
    use crate::heap::memory_chunk::*;
    use crate::objects::objects::*;
    use crate::objects::tagged::*;
    use crate::objects::visitors::*;

    // Assume V8_COMPRESS_POINTERS, V8_EXTERNAL_CODE_SPACE, V8_ENABLE_SANDBOX are defined elsewhere
    #[cfg(feature = "v8_compress_pointers")]
    use crate::common::ptr_compr::*;

    pub struct ConservativeStackVisitorBase<ConcreteVisitor> {
        cage_base_: PtrComprCageBase,
        #[cfg(feature = "v8_external_code_space")]
        code_cage_base_: PtrComprCageBase,
        #[cfg(feature = "v8_external_code_space")]
        code_address_region_: AddressRegion,
        #[cfg(feature = "v8_enable_sandbox")]
        trusted_cage_base_: Address,
        root_visitor_: *mut RootVisitor, // Using raw pointer because RootVisitor seems complex, might need refactoring to a trait
        allocator_: *mut MemoryAllocator, // Using raw pointer because MemoryAllocator seems complex, might need refactoring to a trait
        _phantom: std::marker::PhantomData<ConcreteVisitor>,
    }

    impl<ConcreteVisitor> ConservativeStackVisitorBase<ConcreteVisitor> {
        pub fn new(isolate: &mut Isolate, root_visitor: *mut RootVisitor) -> Self {
            let heap = isolate.heap();
            ConservativeStackVisitorBase {
                cage_base_: PtrComprCageBase::from_address(isolate.address()),
                #[cfg(feature = "v8_external_code_space")]
                code_cage_base_: PtrComprCageBase::from_address(isolate.code_cage_base()),
                #[cfg(feature = "v8_external_code_space")]
                code_address_region_: heap.code_region().clone(),
                #[cfg(feature = "v8_enable_sandbox")]
                trusted_cage_base_: isolate.isolate_data().trusted_cage_base_address(),
                root_visitor_: root_visitor,
                allocator_: heap.memory_allocator(),
                _phantom: std::marker::PhantomData,
            }
        }

        #[cfg(feature = "v8_compress_pointers")]
        fn is_interesting_cage(&self, cage_base: PtrComprCageBase) -> bool {
            if cage_base == self.cage_base_ {
                return true;
            }
            #[cfg(feature = "v8_external_code_space")]
            if cage_base == self.code_cage_base_ {
                return true;
            }
            #[cfg(feature = "v8_enable_sandbox")]
            if cage_base == PtrComprCageBase::from_address(self.trusted_cage_base_) {
                return true;
            }
            false
        }

        fn find_base_ptr(&self, maybe_inner_ptr: Address, cage_base: PtrComprCageBase) -> Address {
            #[cfg(feature = "v8_compress_pointers")]
            debug_assert!(self.is_interesting_cage(cage_base));

            // Check if the pointer is contained by a normal or large page owned by this
            // heap. Bail out if it is not.
            // TODO(379788114): Consider introducing a bloom filter for pages.
            let allocator = unsafe { &*self.allocator_ };
            let chunk = allocator.lookup_chunk_containing_address_in_safepoint(maybe_inner_ptr);

            if chunk.is_null() {
                return kNullAddress;
            }

            let chunk = unsafe { &*chunk };
            let chunk_metadata = chunk.metadata();
            debug_assert!(chunk_metadata.contains(maybe_inner_ptr));

            if !ConcreteVisitor::filter_page(chunk) {
                return kNullAddress;
            }

            // If it is contained in a large page, we want to mark the only object on it.
            if chunk.is_large_page() {
                // This could be simplified if we could guarantee that there are no free
                // space or filler objects in large pages. A few cctests violate this now.
                let obj = unsafe {
                    Tagged::<HeapObject>::from_address(
                        (*chunk_metadata.as_large_page_metadata()).get_object() as Address
                    )
                };
                let map_word = obj.map_word(cage_base, kRelaxedLoad);
                return if !ConcreteVisitor::filter_large_object(obj, map_word) ||
                       InstanceTypeChecker::is_free_space_or_filler(map_word.to_map()) {
                    kNullAddress
                } else {
                    obj.address()
                };
            }

            // Otherwise, we have a pointer inside a normal page.
            let page = chunk_metadata.as_page_metadata();
            // Try to find the address of a previous valid object on this page.
            let base_ptr = MarkingBitmap::find_previous_valid_object(page, maybe_inner_ptr);
            // Iterate through the objects in the page forwards, until we find the object
            // containing maybe_inner_ptr.
            debug_assert!(base_ptr <= maybe_inner_ptr);
            let bitmap = unsafe { &mut *(page.marking_bitmap() as *const _ as *mut MarkingBitmap) }; //Unsafe cast to mutable required for marking bitmap access in marking
            let page_area_end = page.area_end();
            let mut current_base_ptr = base_ptr;
            loop {
                let obj = unsafe { Tagged::<HeapObject>::from_address(current_base_ptr) };
                let map_word = obj.map_word(cage_base, kRelaxedLoad);
                if !ConcreteVisitor::filter_normal_object(obj, map_word, bitmap) {
                    return kNullAddress;
                }
                let size = obj.size_from_map(map_word.to_map());
                debug_assert!(0 < size);
                if maybe_inner_ptr < current_base_ptr + size {
                    ConcreteVisitor::handle_object_found(obj, size, bitmap);
                    return if is_free_space_or_filler(obj, cage_base) {
                        kNullAddress
                    } else {
                        current_base_ptr
                    };
                }
                current_base_ptr += ALIGN_TO_ALLOCATION_ALIGNMENT(size);
                debug_assert!(current_base_ptr < page_area_end);
            }
        }

        pub fn visit_pointer(&mut self, pointer: *const std::ffi::c_void) {
            let address = pointer as Address;
            #[cfg(feature = "v8_compress_pointers")]
            {
                let cage_base = self.cage_base_;
                V8HeapCompressionScheme::process_intermediate_pointers(
                    cage_base,
                    address,
                    |ptr| self.visit_conservatively_if_pointer(ptr, cage_base),
                );
                if ConcreteVisitor::K_ONLY_VISIT_MAIN_V8_CAGE {
                    return;
                }
                #[cfg(feature = "v8_external_code_space")]
                {
                    let code_cage_base = self.code_cage_base_;
                    ExternalCodeCompressionScheme::process_intermediate_pointers(
                        code_cage_base,
                        address,
                        |ptr| self.visit_conservatively_if_pointer(ptr, code_cage_base),
                    );
                }
                #[cfg(feature = "v8_enable_sandbox")]
                {
                    let trusted_cage_base = PtrComprCageBase::from_address(self.trusted_cage_base_);
                    TrustedSpaceCompressionScheme::process_intermediate_pointers(
                        trusted_cage_base,
                        address,
                        |ptr| self.visit_conservatively_if_pointer(ptr, trusted_cage_base),
                    );
                }
            }
            #[cfg(not(feature = "v8_compress_pointers"))]
            self.visit_conservatively_if_pointer(address, self.cage_base_);
        }

        fn visit_conservatively_if_pointer(&mut self, address: Address, cage_base: PtrComprCageBase) {
            // Bail out immediately if the pointer is not in the space managed by the
            // allocator.
            let allocator = unsafe { &*self.allocator_ };
            if allocator.is_outside_allocated_space(address) {
                debug_assert_eq!(
                    std::ptr::null(),
                    allocator.lookup_chunk_containing_address_in_safepoint(address)
                );
                return;
            }

            // Proceed with inner-pointer resolution.
            let base_ptr = self.find_base_ptr(address, cage_base);
            if base_ptr == kNullAddress {
                return;
            }

            let obj = unsafe { HeapObject::from_address(base_ptr) };
            let mut root = unsafe {std::mem::transmute::<HeapObject, Object>(obj)};

            //DCHECK_NOT_NULL(root_visitor_);
            let root_visitor = unsafe { &mut *self.root_visitor_ };
            root_visitor.visit_root_pointer(Root::kStackRoots, std::ptr::null_mut(), FullObjectSlot(&mut root));
            // Check that the root visitor did not modify the root slot.
            debug_assert_eq!(unsafe {std::mem::transmute::<Object, HeapObject>(root)}, obj);
        }

        #[cfg(feature = "v8_compress_pointers")]
        fn visit_conservatively_if_pointer(&mut self, address: Address) {
            if V8HeapCompressionScheme::get_ptr_compr_cage_base_address(address)
                == self.cage_base_.address()
            {
                self.visit_conservatively_if_pointer(address, self.cage_base_);
            } else if ConcreteVisitor::K_ONLY_VISIT_MAIN_V8_CAGE {
                return;
                #[cfg(feature = "v8_external_code_space")]
            } else if self.code_address_region_.contains(address) {
                self.visit_conservatively_if_pointer(address, self.code_cage_base_);
            }
            #[cfg(feature = "v8_enable_sandbox")]
            if TrustedSpaceCompressionScheme::get_ptr_compr_cage_base_address(address)
                == self.trusted_cage_base_
            {
                self.visit_conservatively_if_pointer(
                    address,
                    PtrComprCageBase::from_address(self.trusted_cage_base_),
                );
            }
        }
    }
    // Placeholder functions/structs/impls
    // Needs complete implementation from original C++ V8 code.
    struct V8HeapCompressionScheme {}
    impl V8HeapCompressionScheme {
        #[cfg(feature = "v8_compress_pointers")]
        fn process_intermediate_pointers<F>(cage_base: PtrComprCageBase, address: Address, mut f: F)
        where
            F: FnMut(Address),
        {
            // Placeholder implementation
            f(address);
        }
        #[cfg(feature = "v8_compress_pointers")]
        fn get_ptr_compr_cage_base_address(address: Address) -> Address {
            // Placeholder implementation
            address
        }
    }

    #[cfg(feature = "v8_external_code_space")]
    struct ExternalCodeCompressionScheme {}
    #[cfg(feature = "v8_external_code_space")]
    impl ExternalCodeCompressionScheme {
        fn process_intermediate_pointers<F>(cage_base: PtrComprCageBase, address: Address, mut f: F)
        where
            F: FnMut(Address),
        {
            // Placeholder implementation
            f(address);
        }
    }

    #[cfg(feature = "v8_enable_sandbox")]
    struct TrustedSpaceCompressionScheme {}
    #[cfg(feature = "v8_enable_sandbox")]
    impl TrustedSpaceCompressionScheme {
        fn process_intermediate_pointers<F>(cage_base: PtrComprCageBase, address: Address, mut f: F)
        where
            F: FnMut(Address),
        {
            // Placeholder implementation
            f(address);
        }
        fn get_ptr_compr_cage_base_address(address: Address) -> Address {
            // Placeholder implementation
            address
        }
    }

    trait InstanceTypeChecker {
        fn is_free_space_or_filler(map: Map) -> bool;
    }

    impl InstanceTypeChecker for Map {
        fn is_free_space_or_filler(_map: Map) -> bool {
            false
        }
    }
}