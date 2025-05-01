// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file is a Rust translation of the C++ header file:
// /home/kathirks_gc/v8_go/codebase/src/sandbox/indirect-pointer-inl.h

pub mod indirect_pointer_inl {
    use std::sync::atomic::{AtomicU32, Ordering};

    use crate::sandbox::indirect_pointer::{IndirectPointerHandle, IndirectPointerTag, kCodeIndirectPointerTag, kUnknownIndirectPointerTag, kNullIndirectPointerHandle};
    use crate::sandbox::code_pointer_table_inl::CodePointerTable;
    use crate::sandbox::trusted_pointer_table_inl::TrustedPointerTable;
    use crate::sandbox::isolate_inl::IsolateForSandbox;
    use crate::sandbox::trusted_pointer_table_inl::TrustedPointerPublishingScope;
    use crate::base::atomic_utils;
    use crate::include::v8_internal::Address;
    use crate::include::v8_internal::Tagged;

    #[cfg(feature = "sandbox")]
    pub fn init_self_indirect_pointer_field(
        field_address: Address,
        isolate: IsolateForSandbox,
        host: Tagged<crate::include::v8_internal::HeapObject>,
        tag: IndirectPointerTag,
        opt_publishing_scope: Option<&mut TrustedPointerPublishingScope>,
    ) {
        if cfg!(feature = "sandbox") {
            assert_ne!(tag, kUnknownIndirectPointerTag);
            // TODO(saelo): in the future, we might want to CHECK here or in
            // AllocateAndInitializeEntry that the host lives in trusted space.

            let handle: IndirectPointerHandle;
            if tag == kCodeIndirectPointerTag {
                let space = isolate.get_code_pointer_table_space_for(field_address);
                // TODO: Translate IsolateGroup::current()
                let isolate_group = crate::sandbox::isolate_inl::get_current_isolate_group();
                let table = isolate_group.code_pointer_table();
                handle = table.allocate_and_initialize_entry(space, host.address(), 0, crate::sandbox::code_pointer_table_inl::kDefaultCodeEntrypointTag);
            } else {
                let space = isolate.get_trusted_pointer_table_space_for(tag);
                let table = isolate.get_trusted_pointer_table_for(tag);
                handle = table.allocate_and_initialize_entry(space, host.ptr(), tag, opt_publishing_scope);
            }

            // Use a Release_Store to ensure that the store of the pointer into the table
            // is not reordered after the store of the handle. Otherwise, other threads
            // may access an uninitialized table entry and crash.
            let location = field_address as *mut IndirectPointerHandle;
            atomic_utils::release_store(location, handle);
        } else {
            unreachable!();
        }
    }

    #[cfg(feature = "sandbox")]
    fn resolve_trusted_pointer_handle<const TAG: IndirectPointerTag>(
        handle: IndirectPointerHandle,
        isolate: IsolateForSandbox,
    ) -> Tagged<crate::include::v8_internal::Object> {
        let table = isolate.get_trusted_pointer_table_for(TAG);
        Tagged::from(table.get(handle, TAG))
    }

    #[cfg(feature = "sandbox")]
    fn resolve_code_pointer_handle(handle: IndirectPointerHandle) -> Tagged<crate::include::v8_internal::Object> {
        // TODO: Translate IsolateGroup::current()
        let isolate_group = crate::sandbox::isolate_inl::get_current_isolate_group();
        let table = isolate_group.code_pointer_table();
        Tagged::from(table.get_code_object(handle))
    }

    #[cfg(feature = "sandbox")]
    pub fn read_indirect_pointer_field<const TAG: IndirectPointerTag>(
        field_address: Address,
        isolate: IsolateForSandbox,
        _acquire_load_tag: atomic_utils::AcquireLoadTag,
    ) -> Tagged<crate::include::v8_internal::Object> {
        if cfg!(feature = "sandbox") {
            // Load the indirect pointer handle from the object.
            // Technically, we could use memory_order_consume here as the loads are
            // dependent, but that appears to be deprecated in favor of acquire ordering.
            let location = field_address as *mut IndirectPointerHandle;
            let handle = atomic_utils::acquire_load(location);

            // Resolve the handle. The tag implies the pointer table to use.
            if TAG == kUnknownIndirectPointerTag {
                // In this case we need to check if the handle is a code pointer handle and
                // select the appropriate table based on that.
                if handle & crate::sandbox::code_pointer_table_inl::kCodePointerHandleMarker != 0 {
                    resolve_code_pointer_handle(handle)
                } else {
                    // TODO(saelo): once we have type tagging for entries in the trusted
                    // pointer table, we could ASSUME that the top bits of the tag match the
                    // instance type, which might allow the compiler to optimize subsequent
                    // instance type checks.
                    resolve_trusted_pointer_handle::<{TAG}>(handle, isolate)
                }
            } else if TAG == kCodeIndirectPointerTag {
                resolve_code_pointer_handle(handle)
            } else {
                resolve_trusted_pointer_handle::<{TAG}>(handle, isolate)
            }
        } else {
            unreachable!();
        }
    }

    #[cfg(feature = "sandbox")]
    pub fn write_indirect_pointer_field<const TAG: IndirectPointerTag>(
        field_address: Address,
        value: Tagged<crate::sandbox::trusted_pointer_table_inl::ExposedTrustedObject>,
        _release_store_tag: atomic_utils::ReleaseStoreTag,
    ) {
        if cfg!(feature = "sandbox") {
            assert_ne!(TAG, 0); // kIndirectPointerNullTag
            let handle = value.self_indirect_pointer_handle();
            assert_ne!(handle, kNullIndirectPointerHandle);
            let location = field_address as *mut IndirectPointerHandle;
            atomic_utils::release_store(location, handle);
        } else {
            unreachable!();
        }
    }
}