// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod unwinder {
    /// Represents the state of registers.
    pub struct RegisterState {}

    /// Retrieves callee-saved registers from the entry frame.
    ///
    /// # Arguments
    ///
    /// * `fp` - A void pointer representing the frame pointer.
    /// * `register_state` - A mutable reference to a `RegisterState` struct where the callee-saved registers will be stored.
    pub fn get_callee_saved_registers_from_entry_frame(fp: *mut std::ffi::c_void, register_state: &mut RegisterState) {
        // This function is intentionally empty in the original C++ code.
        // In a real implementation, it would populate the `register_state`
        // based on the frame pointer `fp`.  Since this is architecture specific
        // (ia32), it requires unsafe operations and knowledge of the stack frame.
    }
}

pub use unwinder::*;