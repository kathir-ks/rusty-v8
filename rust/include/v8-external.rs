// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Corresponding C++ header: include/v8-external.h

//use v8_value::*; // Assuming v8-value.h is converted into v8_value module
//use v8config::*; // Assuming v8config.h is converted into v8config module

// Replace with actual crate imports once the other files are converted.
// For now using std::ptr::null_mut and other standard library features.
use std::ptr::null_mut;

pub mod v8 {
    pub struct Isolate {} // Stub Isolate type.  Needs to be properly defined from v8.h

    pub struct Value {} // Stub Value type.  Needs to be properly defined from v8-value.h

    /// A JavaScript value that wraps a C++ void*. This type of value is mainly used
    /// to associate C++ data structures with JavaScript objects.
    #[derive(Debug)]
    pub struct External {
        value: *mut std::ffi::c_void,
    }

    impl External {
        /// Creates a new External object.
        pub fn new(isolate: &mut Isolate, value: *mut std::ffi::c_void) -> Box<External> {
            //TODO:  Need to tie the lifetime of the External to the Isolate's heap.
            // For now, just allocate on the heap.  This is probably not correct.
            Box::new(External { value })
        }

        /// Casts a Value to an External.
        pub fn cast<'a>(value: &'a Value) -> &'a External {
            // Note: Since we don't have enough information about the Value type, we can't
            // perform a proper type check here.  This is just a blind cast.  In a real
            // implementation, we would need to verify that `value` is actually an `External`
            // before casting.  This will require more information from the original C++
            // Value class and its hierarchy.
            unsafe { &*(value as *const Value as *const External) }
        }
        /// Returns the wrapped C++ void*.
        pub fn value(&self) -> *mut std::ffi::c_void {
            self.value
        }
    }
}