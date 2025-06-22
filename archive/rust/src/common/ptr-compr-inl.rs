// src/common/ptr_compr_inl.rs

// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod ptr_compr_inl {
    use crate::common::ptr_compr::ptr_compr::{kPtrComprCageBaseAlignment, PtrComprCageBase, V8HeapCompressionScheme};
    use crate::execution::isolate::isolate::Isolate;
    use crate::execution::local_isolate_inl::local_isolate_inl::LocalIsolate;
    use crate::include::v8_internal::*;
    use std::{mem, u32};
    use std::ops::BitAnd;
    use std::ptr::null_mut;
    use std::os::raw::c_int;

    #[cfg(feature = "V8_ENABLE_SANDBOX")]
    use crate::sandbox::sandbox::Sandbox;

    #[cfg(feature = "V8_COMPRESS_POINTERS")]
    impl PtrComprCageBase {
        pub fn new(isolate: *const Isolate) -> Self {
            unsafe {
                PtrComprCageBase {
                    address_: (*isolate).cage_base(),
                }
            }
        }

        pub fn new_local(isolate: *const LocalIsolate) -> Self {
            unsafe {
                PtrComprCageBase {
                    address_: (*isolate).cage_base(),
                }
            }
        }
    }

    #[cfg(feature = "V8_COMPRESS_POINTERS")]
    pub const K_PTR_COMPR_CAGE_BASE_MASK: Address = !(kPtrComprCageBaseAlignment - 1);

    #[cfg(feature = "V8_COMPRESS_POINTERS")]
    pub struct V8HeapCompressionSchemeImpl<Cage> {
        phantom: std::marker::PhantomData<Cage>,
    }

    #[cfg(feature = "V8_COMPRESS_POINTERS")]
    impl<Cage> V8HeapCompressionSchemeImpl<Cage> {
        pub const fn get_ptr_compr_cage_base_address(on_heap_addr: Address) -> Address {
            round_down::<kPtrComprCageBaseAlignment>(on_heap_addr)
        }

        pub fn get_ptr_compr_cage_base_address_cage_base(cage_base: PtrComprCageBase) -> Address {
            let mut base = cage_base.address();
            assert!((base & K_PTR_COMPR_CAGE_BASE_MASK) == base);
            base
        }

        pub fn init_base(base: Address) {
            assert_eq!(base, Self::get_ptr_compr_cage_base_address(base));
            // This part depends on global mutable state, and is therefore not directly translatable.
            // See original C++ for details.
            // #[cfg(all(feature = "USING_V8_SHARED_PRIVATE", feature = "V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES"))]
            // {
            //     Cage::set_base_non_inlined(base);
            // }
            // #[cfg(not(all(feature = "USING_V8_SHARED_PRIVATE", feature = "V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES")))]
            // {
            //     Cage::base_ = base;
            // }
        }

        pub fn base() -> Address {
            // This part depends on global mutable state, and is therefore not directly translatable.
            // See original C++ for details.
            // #[cfg(all(feature = "USING_V8_SHARED_PRIVATE", feature = "V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES"))]
            // {
            //     let base = Cage::base_non_inlined();
            // }
            // #[cfg(not(all(feature = "USING_V8_SHARED_PRIVATE", feature = "V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES")))]
            // {
            //     let base = Cage::base_;
            // }
            let base: Address = 0; //Dummy base because the real one depends on mutable state
            assert!((base & K_PTR_COMPR_CAGE_BASE_MASK) == base);
            base
        }

        pub fn compress_object(tagged: Address) -> Tagged_t {
            // This is used to help clang produce better code. Values which could be
            // invalid pointers need to be compressed with CompressAny.
            // #[cfg(feature = "V8_COMPRESS_POINTERS_IN_SHARED_CAGE")]
            // {
            //     assert!((tagged & K_PTR_COMPR_CAGE_BASE_MASK) == Self::base() || has_smi_tag(tagged));
            // }
            tagged as Tagged_t
        }

        pub const fn compress_any(tagged: Address) -> Tagged_t {
            tagged as Tagged_t
        }

        pub fn decompress_tagged_signed(raw_value: Tagged_t) -> Address {
            // For runtime code the upper 32-bits of the Smi value do not matter.
            raw_value as Address
        }

        pub fn decompress_tagged<TOnHeapAddress>(
            on_heap_addr: TOnHeapAddress,
            raw_value: Tagged_t,
        ) -> Address {
            #[cfg(feature = "V8_COMPRESS_POINTERS")]
            {
                let cage_base = Self::base();
                // #[cfg(feature = "V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES")]
                // {
                //     assert_ne!(cage_base, kNullAddress, "V8HeapCompressionSchemeImpl::base is not initialized for current thread");
                // }
            }
            #[cfg(not(feature = "V8_COMPRESS_POINTERS"))]
            {
                let cage_base = Self::get_ptr_compr_cage_base_address(on_heap_addr);
            }
            let cage_base = 0; //Dummy value to prevent conditional compilation
            let result = cage_base + raw_value as Address;
            assert_eq!(result as u32, raw_value as u32);
            result
        }

        pub fn process_intermediate_pointers<ProcessPointerCallback>(
            cage_base: PtrComprCageBase,
            raw_value: Address,
            callback: ProcessPointerCallback,
        ) where
            ProcessPointerCallback: Fn(Address),
        {
            // If pointer compression is enabled, we may have random compressed pointers
            // on the stack that may be used for subsequent operations.
            // Extract, decompress and trace both halfwords.
            let decompressed_low =
                V8HeapCompressionSchemeImpl::<Cage>::decompress_tagged(cage_base.address(), raw_value as Tagged_t);
            callback(decompressed_low);
            let decompressed_high = V8HeapCompressionSchemeImpl::<Cage>::decompress_tagged(
                cage_base.address(),
                (raw_value >> (mem::size_of::<Tagged_t>() * 8)) as Tagged_t,
            );
            callback(decompressed_high);
        }
    }

    #[cfg(feature = "V8_EXTERNAL_CODE_SPACE")]
    pub mod external_code_compression_scheme {
        use crate::common::ptr_compr::ptr_compr::{kPtrComprCageBaseAlignment, PtrComprCageBase};
        use crate::include::v8_internal::*;
        use std::{mem, os::raw::c_int};

        pub fn prepare_cage_base_address(on_heap_addr: Address) -> Address {
            round_down::<kPtrComprCageBaseAlignment>(on_heap_addr)
        }

        pub fn get_ptr_compr_cage_base_address(cage_base: PtrComprCageBase) -> Address {
            let mut base = cage_base.address();
            assert!((base & super::K_PTR_COMPR_CAGE_BASE_MASK) == base);
            base
        }

        pub fn init_base(base: Address) {
            assert_eq!(base, prepare_cage_base_address(base));
            // This part depends on global mutable state, and is therefore not directly translatable.
            // See original C++ for details.
            // #[cfg(all(feature = "USING_V8_SHARED_PRIVATE", feature = "V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES"))]
            // {
            //     set_base_non_inlined(base);
            // }
            // #[cfg(not(all(feature = "USING_V8_SHARED_PRIVATE", feature = "V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES")))]
            // {
            //     base_ = base;
            // }
        }

        pub fn base() -> Address {
            // This part depends on global mutable state, and is therefore not directly translatable.
            // See original C++ for details.
            // #[cfg(all(feature = "USING_V8_SHARED_PRIVATE", feature = "V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES"))]
            // {
            //     let base = base_non_inlined();
            // }
            // #[cfg(not(all(feature = "USING_V8_SHARED_PRIVATE", feature = "V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES")))]
            // {
            //     let base = base_;
            // }
            let base: Address = 0; //Dummy value to prevent conditional compilation

            assert!((base & super::K_PTR_COMPR_CAGE_BASE_MASK) == base);
            base
        }

        pub fn compress_object(tagged: Address) -> Tagged_t {
            // This is used to help clang produce better code. Values which could be
            // invalid pointers need to be compressed with CompressAny.
            // The DCHECK generated by this V8_ASSUME is also very helpful during
            // development when moving objects between pointer compression cages as it
            // quickly identifies any places where we still store a compressed pointer
            // slot with the wrong base.
            // #[cfg(feature = "V8_COMPRESS_POINTERS_IN_SHARED_CAGE")]
            // {
            //     assert!((tagged & super::K_PTR_COMPR_CAGE_BASE_MASK) == base() || has_smi_tag(tagged));
            // }
            tagged as Tagged_t
        }

        pub const fn compress_any(tagged: Address) -> Tagged_t {
            tagged as Tagged_t
        }

        pub fn decompress_tagged_signed(raw_value: Tagged_t) -> Address {
            // For runtime code the upper 32-bits of the Smi value do not matter.
            raw_value as Address
        }

        pub fn decompress_tagged<TOnHeapAddress>(
            on_heap_addr: TOnHeapAddress,
            raw_value: Tagged_t,
        ) -> Address {
            #[cfg(feature = "V8_COMPRESS_POINTERS")]
            {
                let cage_base = base();
                // #[cfg(feature = "V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES")]
                // {
                //     assert_ne!(cage_base, kNullAddress, "ExternalCodeCompressionScheme::base is not initialized for current thread");
                // }
            }
            #[cfg(not(feature = "V8_COMPRESS_POINTERS"))]
            {
                let cage_base = get_ptr_compr_cage_base_address(PtrComprCageBase{address_: on_heap_addr});
            }
            let cage_base = 0; //Dummy value to prevent conditional compilation
            let result = cage_base + raw_value as Address;
            assert_eq!(result as u32, raw_value as u32);
            result
        }

        pub fn process_intermediate_pointers<ProcessPointerCallback>(
            cage_base: PtrComprCageBase,
            raw_value: Address,
            callback: ProcessPointerCallback,
        ) where
            ProcessPointerCallback: Fn(Address),
        {
            // If pointer compression is enabled, we may have random compressed pointers
            // on the stack that may be used for subsequent operations.
            // Extract, decompress and trace both halfwords.
            let decompressed_low = external_code_compression_scheme::decompress_tagged(
                cage_base.address(),
                raw_value as Tagged_t,
            );
            callback(decompressed_low);
            let decompressed_high = external_code_compression_scheme::decompress_tagged(
                cage_base.address(),
                (raw_value >> (mem::size_of::<Tagged_t>() * 8)) as Tagged_t,
            );
            callback(decompressed_high);
        }
    }

    #[cfg(feature = "V8_COMPRESS_POINTERS")]
    pub fn get_ptr_compr_cage_base_from_on_heap_address(address: Address) -> PtrComprCageBase {
        PtrComprCageBase {
            address_: V8HeapCompressionScheme::get_ptr_compr_cage_base_address(address),
        }
    }

    #[cfg(feature = "V8_COMPRESS_POINTERS")]
    pub fn get_ptr_compr_cage_base() -> PtrComprCageBase {
        PtrComprCageBase {
            address_: V8HeapCompressionScheme::base(),
        }
    }

    #[cfg(not(feature = "V8_COMPRESS_POINTERS"))]
    pub const fn get_ptr_compr_cage_base_from_on_heap_address(address: Address) -> PtrComprCageBase {
        PtrComprCageBase { address_: 0 }
    }

    #[cfg(not(feature = "V8_COMPRESS_POINTERS"))]
    pub fn get_ptr_compr_cage_base() -> PtrComprCageBase {
        PtrComprCageBase { address_: 0 }
    }

    use crate::objects::heap_object::heap_object::HeapObject;
    use crate::objects::tagged::tagged::Tagged;

    pub fn get_ptr_compr_cage_base_tagged(object: Tagged<HeapObject>) -> PtrComprCageBase {
        get_ptr_compr_cage_base_from_on_heap_address(object.ptr())
    }

    fn round_down<const ALIGNMENT: usize>(addr: Address) -> Address {
        addr & !(ALIGNMENT as Address - 1)
    }

    #[cfg(feature = "V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES")]
    pub struct PtrComprCageAccessScope {
        cage_base_: Address,
        #[cfg(feature = "V8_EXTERNAL_CODE_SPACE")]
        code_cage_base_: Address,
        saved_current_isolate_group_: usize, //IsolateGroup
        #[cfg(feature = "V8_ENABLE_SANDBOX")]
        saved_current_sandbox_: usize, //Sandbox
    }

    #[cfg(feature = "V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES")]
    impl PtrComprCageAccessScope {
        pub fn new(isolate: *mut Isolate) -> Self {
            unsafe {
                let cage_base_ = V8HeapCompressionScheme::base();
                #[cfg(feature = "V8_EXTERNAL_CODE_SPACE")]
                let code_cage_base_ = external_code_compression_scheme::base();
                let isolate_group_ptr = (*isolate).isolate_group();
                let saved_current_isolate_group_ = isolate_group_ptr as usize;

                #[cfg(feature = "V8_ENABLE_SANDBOX")]
                let saved_current_sandbox_ = crate::sandbox::sandbox::Sandbox::current() as usize;

                V8HeapCompressionScheme::init_base((*isolate).cage_base());
                #[cfg(feature = "V8_EXTERNAL_CODE_SPACE")]
                external_code_compression_scheme::init_base((*isolate).code_cage_base());

                // TODO: Implement IsolateGroup::set_current
                // IsolateGroup::set_current(isolate->isolate_group());
                #[cfg(feature = "V8_ENABLE_SANDBOX")]
                {
                    // TODO: Implement Sandbox::set_current
                    // Sandbox::set_current(isolate->isolate_group()->sandbox());
                }

                PtrComprCageAccessScope {
                    cage_base_: cage_base_,
                    #[cfg(feature = "V8_EXTERNAL_CODE_SPACE")]
                    code_cage_base_: code_cage_base_,
                    saved_current_isolate_group_: saved_current_isolate_group_,
                    #[cfg(feature = "V8_ENABLE_SANDBOX")]
                    saved_current_sandbox_: saved_current_sandbox_,
                }
            }
        }
    }

    #[cfg(feature = "V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES")]
    impl Drop for PtrComprCageAccessScope {
        fn drop(&mut self) {
            V8HeapCompressionScheme::init_base(self.cage_base_);

            #[cfg(feature = "V8_EXTERNAL_CODE_SPACE")]
            external_code_compression_scheme::init_base(self.code_cage_base_);

            // TODO: Implement IsolateGroup::set_current
            // IsolateGroup::set_current(saved_current_isolate_group_);
            #[cfg(feature = "V8_ENABLE_SANDBOX")]
            {
                // TODO: Implement Sandbox::set_current
                // Sandbox::set_current(saved_current_sandbox_);
            }
        }
    }
}