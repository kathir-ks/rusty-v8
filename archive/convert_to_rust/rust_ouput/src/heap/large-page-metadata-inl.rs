// Converted from V8 C++ source files:
// Header: large-page-metadata-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
use crate::heap::mutable_page_metadata_inl::MutablePageMetadata;
use crate::v8::internal::HeapObject;
use crate::v8::internal::Tagged;

pub struct LargePageMetadata {}

impl LargePageMetadata {
    // static
    pub fn from_heap_object(o: Tagged<HeapObject>) -> *mut LargePageMetadata {
        MutablePageMetadata::from_heap_object(o) as *mut LargePageMetadata
    }
}
