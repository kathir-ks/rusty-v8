// Converted from V8 C++ source files:
// Header: trusted-object.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
use crate::objects::heap_object::HeapObject;
use crate::objects::tagged_field::Tagged;

pub struct TrustedObject {
    heap_object: HeapObject,
}

impl TrustedObject {
    pub fn verify(_object: &TrustedObject) {}

    pub fn read_protected_pointer_field(&self, _offset: i32) -> Tagged<TrustedObject> {
        Tagged {}
    }

    pub fn read_protected_pointer_field_acquire_load_tag(
        &self,
        _offset: i32,
        _tag: AcquireLoadTag,
    ) -> Tagged<TrustedObject> {
        Tagged {}
    }

    pub fn write_protected_pointer_field(&mut self, _offset: i32, _value: Tagged<TrustedObject>) {}

    pub fn write_protected_pointer_field_release_store_tag(
        &mut self,
        _offset: i32,
        _value: Tagged<TrustedObject>,
        _tag: ReleaseStoreTag,
    ) {
    }

    pub fn is_protected_pointer_field_empty(&self, _offset: i32) -> bool {
        false
    }

    pub fn is_protected_pointer_field_empty_acquire_load_tag(
        &self,
        _offset: i32,
        _tag: AcquireLoadTag,
    ) -> bool {
        false
    }

    pub fn clear_protected_pointer_field(&mut self, _offset: i32) {}

    pub fn clear_protected_pointer_field_release_store_tag(
        &mut self,
        _offset: i32,
        _tag: ReleaseStoreTag,
    ) {
    }

    pub fn raw_protected_pointer_field(&self, _byte_offset: i32) -> ProtectedPointerSlot {
        ProtectedPointerSlot {}
    }

    pub fn raw_protected_maybe_object_field(&self, _byte_offset: i32) -> ProtectedMaybeObjectSlot {
        ProtectedMaybeObjectSlot {}
    }

    #[cfg(feature = "verify_heap")]
    pub fn verify_protected_pointer_field(&self, _isolate: &Isolate, _offset: i32) {}

    pub const K_HEADER_SIZE: i32 = HeapObject::K_HEADER_SIZE;

    pub fn new(_heap_object: HeapObject) -> Self {
        TrustedObject { heap_object: _heap_object }
    }
}

pub struct TrustedObjectLayout {
    heap_object_layout: HeapObjectLayout,
}

impl TrustedObjectLayout {
    pub fn verify(_object: &TrustedObjectLayout) {}
}

pub struct ExposedTrustedObject {
    trusted_object: TrustedObject,
}

impl ExposedTrustedObject {
    pub fn init_self_indirect_pointer(&mut self, _isolate: &Isolate) {}
    pub fn init_self_indirect_pointer_local(&mut self, _isolate: &LocalIsolate) {}
    pub fn self_indirect_pointer_handle(&self) -> IndirectPointerHandle {
        IndirectPointerHandle {}
    }

    pub fn verify(_object: &ExposedTrustedObject) {}

    #[cfg(feature = "v8_enable_sandbox")]
    pub const K_SELF_INDIRECT_POINTER_OFFSET: i32 = 0;
    #[cfg(feature = "v8_enable_sandbox")]
    pub const K_UNALIGNED_HEADER_SIZE: i32 = 0;
    #[cfg(feature = "v8_enable_sandbox")]
    pub const K_HEADER_SIZE: i32 = 0;
    #[cfg(feature = "v8_enable_sandbox")]
    pub const K_SIZE: i32 = 0;

    #[cfg(not(feature = "v8_enable_sandbox"))]
    pub const K_HEADER_SIZE: i32 = TrustedObject::K_HEADER_SIZE;

    pub fn new(_trusted_object: TrustedObject) -> Self {
        ExposedTrustedObject { trusted_object: _trusted_object }
    }
}

pub struct AcquireLoadTag {}
pub struct ReleaseStoreTag {}
pub struct Isolate {}
pub struct LocalIsolate {}
pub struct IndirectPointerHandle {}

pub struct ProtectedPointerSlot {}
pub struct ProtectedMaybeObjectSlot {}

pub struct HeapObjectLayout {}
