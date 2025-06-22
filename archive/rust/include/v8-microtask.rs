// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod microtask {
    // Opaque type representing the V8 Isolate.
    // Needs to be defined properly when binding to V8 API.
    pub struct Isolate {
        _private: (),
    }

    /// Callback type for microtasks completed notifications.
    pub type MicrotasksCompletedCallbackWithData =
        unsafe extern "C" fn(isolate: *mut Isolate, data: *mut std::ffi::c_void);

    /// Callback type for a microtask.
    pub type MicrotaskCallback = unsafe extern "C" fn(data: *mut std::ffi::c_void);

    /// Policy for running microtasks.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum MicrotasksPolicy {
        /// Microtasks are invoked with the `Isolate::PerformMicrotaskCheckpoint()` method.
        Explicit,
        /// Microtasks invocation is controlled by `MicrotasksScope` objects.
        Scoped,
        /// Microtasks are invoked when the script call depth decrements to zero.
        Auto,
    }
}