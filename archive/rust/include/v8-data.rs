// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// v8-data.h equivalent in Rust

pub mod v8 {
    // Placeholder for v8config.h contents.  In a real conversion,
    // this would be replaced with equivalent Rust configuration.
    // For now, we assume V8_ENABLE_CHECKS is always enabled.
    const V8_ENABLE_CHECKS: bool = true;

    // Placeholder for Local<T>. In a real conversion, this would
    // involve more sophisticated memory management (e.g., using Rc, Arc, or a custom arena).
    #[derive(Debug, Clone, Copy)]
    pub struct Local<'a, T> {
        pub ptr: &'a T,
    }

    // Placeholder for Context.
    #[derive(Debug, Clone, Copy)]
    pub struct Context {}

    /// The superclass of objects that can reside on V8's heap.
    pub trait Data {
        /// Returns true if this data is a |v8::Value|.
        fn is_value(&self) -> bool;

        /// Returns true if this data is a |v8::Module|.
        fn is_module(&self) -> bool;

        /// Returns true if this data is a |v8::ModuleRequest|.
        fn is_module_request(&self) -> bool;

        /// Returns true if this data is a |v8::FixedArray|
        fn is_fixed_array(&self) -> bool;

        /// Returns true if this data is a |v8::Private|.
        fn is_private(&self) -> bool;

        /// Returns true if this data is a |v8::ObjectTemplate|.
        fn is_object_template(&self) -> bool;

        /// Returns true if this data is a |v8::FunctionTemplate|.
        fn is_function_template(&self) -> bool;

        /// Returns true if this data is a |v8::Context|.
        fn is_context(&self) -> bool;
    }

    /// A fixed-sized array with elements of type Data.
    pub struct FixedArray {
        length: usize,
        data: Vec<Box<dyn Data>>, // Using Box to hold trait objects
    }

    impl FixedArray {
        pub fn new(length: usize) -> Self {
            FixedArray {
                length,
                data: Vec::with_capacity(length),
            }
        }

        pub fn with_data(data: Vec<Box<dyn Data>>) -> Self {
          let length = data.len();
          FixedArray {
            length,
            data
          }
        }
        pub fn length(&self) -> usize {
            self.length
        }

        pub fn get<'a>(&'a self, context: Local<'_, Context>, i: usize) -> Option<Local<'a, dyn Data>> {
            if i < self.length {
              self.data.get(i).map(|data| Local { ptr: data.as_ref() })
            } else {
              None
            }
        }

        pub fn cast(data: &dyn Data) -> Option<&FixedArray> {
            if data.is_fixed_array() {
                // Perform the cast. This is unsafe because we're relying on the
                // `is_fixed_array` check to ensure the type is correct.
                // In a real V8 implementation, the type information would be more
                // robustly managed.
                Some(unsafe { &*(data as *const dyn Data as *const FixedArray) })
            } else {
                None
            }
        }
    }

    impl Data for FixedArray {
        fn is_value(&self) -> bool {
            false
        }

        fn is_module(&self) -> bool {
            false
        }

        fn is_module_request(&self) -> bool {
            false
        }

        fn is_fixed_array(&self) -> bool {
            true
        }

        fn is_private(&self) -> bool {
            false
        }

        fn is_object_template(&self) -> bool {
            false
        }

        fn is_function_template(&self) -> bool {
            false
        }

        fn is_context(&self) -> bool {
            false
        }
    }
}