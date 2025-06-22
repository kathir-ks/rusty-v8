// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// Module for inspector test interface.
pub mod test_interface {
    // Represents the V8Inspector (opaque type).  Since we don't have the
    // full definition, we use a raw pointer.  This would ideally be a more
    // managed type like Box or Arc if we owned it.
    pub type V8InspectorPtr = *mut std::ffi::c_void;

    /// Sets the maximum number of asynchronous task stacks for testing.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it operates on a raw pointer
    /// to a V8Inspector and modifies internal state. The caller must ensure
    /// that the pointer is valid and that the operation is safe in the
    /// context of the V8 engine.
    extern "C" {
        pub fn SetMaxAsyncTaskStacksForTest(inspector: V8InspectorPtr, limit: i32);
    }

    /// Dumps the state of asynchronous task stacks for testing.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it operates on a raw pointer
    /// to a V8Inspector and may access internal state. The caller must ensure
    /// that the pointer is valid and that the operation is safe in the
    /// context of the V8 engine.
    extern "C" {
        pub fn DumpAsyncTaskStacksStateForTest(inspector: V8InspectorPtr);
    }
}