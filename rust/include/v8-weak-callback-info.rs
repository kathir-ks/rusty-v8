// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Equivalent of INCLUDE_V8_WEAK_CALLBACK_INFO_H_

pub mod weak_callback_info {
    // Replaces cppgc::internal::ConditionalStackAllocatedBase.  In a real
    // implementation, this might involve a more complex strategy for stack
    // allocation or heap allocation based on size/conditions. For now, it is just T.
    // use cppgc;

    //Placeholder
    pub struct Isolate;

    pub mod api_internal {
        pub fn internal_field_out_of_bounds(index: i32) {
            panic!("Internal field out of bounds: {}", index);
        }
    }

    pub const K_INTERNAL_FIELDS_IN_WEAK_CALLBACK: usize = 2;
    pub const K_EMBEDDER_FIELDS_IN_WEAK_CALLBACK: usize = 2;

    pub struct WeakCallbackInfo<T> {
        isolate_: *mut Isolate,
        parameter_: *mut T,
        callback_: *mut Callback<T>,
        embedder_fields_: [*mut std::ffi::c_void; K_EMBEDDER_FIELDS_IN_WEAK_CALLBACK],
    }

    pub type Callback<T> = extern "C" fn(&WeakCallbackInfo<T>);

    impl<T> WeakCallbackInfo<T> {
        pub fn new(
            isolate: *mut Isolate,
            parameter: *mut T,
            embedder_fields: [*mut std::ffi::c_void; K_EMBEDDER_FIELDS_IN_WEAK_CALLBACK],
            callback: *mut Callback<T>,
        ) -> Self {
            WeakCallbackInfo {
                isolate_: isolate,
                parameter_: parameter,
                callback_: callback,
                embedder_fields_: embedder_fields,
            }
        }

        #[inline]
        pub fn get_isolate(&self) -> *mut Isolate {
            self.isolate_
        }

        #[inline]
        pub fn get_parameter(&self) -> *mut T {
            self.parameter_
        }

        #[inline]
        pub fn get_internal_field(&self, index: usize) -> *mut std::ffi::c_void {
            #[cfg(debug_assertions)]
            if index >= K_EMBEDDER_FIELDS_IN_WEAK_CALLBACK {
                api_internal::internal_field_out_of_bounds(index as i32);
            }

            self.embedder_fields_[index]
        }

        /// When a weak callback is first invoked the embedders _must_ Reset() the
        /// handle which triggered the callback. The handle itself is unusable for
        /// anything else. No other V8 API calls may be called in the first callback.
        /// Additional work requires scheduling a second invocation via
        /// `SetSecondPassCallback()` which will be called some time after all the
        /// initial callbacks are processed.
        ///
        /// The second pass callback is prohibited from executing JavaScript. Embedders
        /// should schedule another callback in case this is required.
        pub fn set_second_pass_callback(&self, callback: Callback<T>) {
            unsafe {
                *self.callback_ = callback;
            }
        }
    }

    /// Weakness type for weak handles.
    #[derive(Clone, Copy)]
    pub enum WeakCallbackType {
        /// Passes a user-defined void* parameter back to the callback.
        KParameter,
        /// Passes the first two internal fields of the object back to the callback.
        KInternalFields,
    }
}