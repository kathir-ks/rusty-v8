// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/heap/cppgc-js/cross-heap-remembered-set.h

use std::cell::RefCell;
use std::rc::Rc;

pub(crate) mod cppgc {
    pub(crate) mod internal {
        pub struct HeapBase {} // Placeholder
    }
}

pub mod internal {
    pub struct JSObject {} // Placeholder

    // Placeholder for Tagged<JSObject>
    pub struct TaggedJSObject {}

    impl TaggedJSObject {
        pub fn new() -> Self {
            TaggedJSObject {}
        }
    }

    pub struct Isolate {} // Placeholder

    impl Isolate {
        pub fn new() -> Self {
            Isolate {}
        }
    }

    // Placeholder for IndirectHandle<JSObject>
    pub struct IndirectHandle<T> {
        pub(crate) obj: Rc<RefCell<T>>,
    }

    impl<T> IndirectHandle<T> {
        pub fn new(obj: Rc<RefCell<T>>) -> Self {
            IndirectHandle { obj }
        }
        pub fn get(&self) -> Rc<RefCell<T>> {
            self.obj.clone()
        }
    }

    /// The class is used to remember V8 to Oilpan references.
    pub struct CrossHeapRememberedSet {
        heap_base: *mut cppgc::internal::HeapBase, // Raw pointer to HeapBase
        // The vector keeps handles to remembered V8 objects that have outgoing
        // references to the cppgc heap. Please note that the handles are global.
        remembered_v8_to_cppgc_references: Vec<IndirectHandle<JSObject>>,
    }

    impl CrossHeapRememberedSet {
        pub fn new(heap_base: &mut cppgc::internal::HeapBase) -> Self {
            CrossHeapRememberedSet {
                heap_base: heap_base,
                remembered_v8_to_cppgc_references: Vec::new(),
            }
        }

        // Disabled copy and move constructors
        // CrossHeapRememberedSet(const CrossHeapRememberedSet&) = delete;
        // CrossHeapRememberedSet(CrossHeapRememberedSet&&) = delete;

        pub fn remember_reference_if_needed(
            &mut self,
            _isolate: &mut Isolate,
            host_obj: TaggedJSObject,
            _cppgc_object: *mut std::ffi::c_void,
        ) {
            // For now, create a dummy JSObject and store it
            let js_obj = Rc::new(RefCell::new(JSObject {}));
            self.remembered_v8_to_cppgc_references
                .push(IndirectHandle::new(js_obj));
        }

        pub fn reset(&mut self, _isolate: &mut Isolate) {
            self.remembered_v8_to_cppgc_references.clear();
        }

        pub fn visit<F>(&self, _isolate: &mut Isolate, mut f: F)
        where
            F: FnMut(&JSObject),
        {
            for obj in &self.remembered_v8_to_cppgc_references {
                f(&obj.obj.borrow());
            }
        }

        pub fn is_empty(&self) -> bool {
            self.remembered_v8_to_cppgc_references.is_empty()
        }
    }
}