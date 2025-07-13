// Converted from V8 C++ source files:
// Header: read-only-heap-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use crate::heap::read_only_heap::ReadOnlyHeap;
use crate::roots::roots_inl::GetHeapFromWritableObject;
use crate::heap::scavenger::ReadOnlyRoots;
use crate::execution::isolate_utils_inl::IsolateGroup;
use crate::objects::tagged::Tagged;
use crate::objects::heap_object::HeapObject;

impl ReadOnlyHeap {
    // static
    pub fn early_get_read_only_roots(object: Tagged<HeapObject>) -> ReadOnlyRoots {
        let isolate_group = IsolateGroup::current();
        if let Some(shared_ro_heap) = isolate_group.shared_read_only_heap() {
            if shared_ro_heap.roots_init_complete() {
                return ReadOnlyRoots {
                    read_only_roots_: shared_ro_heap.read_only_roots_.clone()
                };
            }
        }
        ReadOnlyRoots {
            read_only_roots_: GetHeapFromWritableObject(object)
        }
    }
}
