// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// include/cppgc/object-size-trait.h (Rust module definition)
pub mod object_size_trait {
    /// Trait for getting the size of an object.
    pub trait ObjectSizeTrait {
        /// Returns the size of the object.
        fn get_object_size(&self) -> usize;
    }
}

mod heap {
    pub mod cppgc {
        pub mod heap_object_header;
        pub mod heap_page;
        pub mod object_view;

        pub(crate) mod internal {
            use super::heap_object_header::HeapObjectHeader;
            use super::heap_page::BasePage;
            use super::object_view::{AccessMode, ObjectView};

            /// Base implementation for object size trait.
            pub struct BaseObjectSizeTrait;

            impl BaseObjectSizeTrait {
                /// Returns the size of the object pointed to by `object`.
                pub fn get_object_size_for_garbage_collected(object: *const std::ffi::c_void) -> usize {
                    let header = unsafe { HeapObjectHeader::from_object(object) };
                    ObjectView::<AccessMode>::new(header).size()
                }

                /// Returns the size of the object whose header is at `address`.
                pub fn get_object_size_for_garbage_collected_mixin(address: *const std::ffi::c_void) -> usize {
                    // `address` is guaranteed to be on a normal page because large object mixins
                    // are not supported.
                    let page = unsafe { BasePage::from_payload(address) };
                    let header = unsafe { page.object_header_from_inner_address::<AccessMode>(address) };
                    // TODO(Rust): Add assert that the object is not a large object
                    // DCHECK(!header.IsLargeObject<AccessMode::kAtomic>());
                    header.object_size::<AccessMode>()
                }
            }
        }
    }
}