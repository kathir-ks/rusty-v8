// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file is a placeholder for s390 unwinder functionality.
// The original C++ implementation is architecture-specific.

mod unwinder {
    pub struct RegisterState {}

    /// Placeholder function for retrieving callee-saved registers from an entry frame.
    ///
    /// This function's implementation would be architecture-specific and is currently empty.
    pub fn get_callee_saved_registers_from_entry_frame(
        _fp: *mut std::ffi::c_void,
        _register_state: &mut RegisterState,
    ) {
        // Architecture-specific implementation would go here.
        // Currently, it does nothing.
    }
}