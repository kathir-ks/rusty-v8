// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod diagnostics {
    pub mod unwinder {
        /// Represents the state of registers.
        pub struct RegisterState {}

        /// Retrieves the callee-saved registers from an entry frame.
        ///
        /// # Arguments
        ///
        /// * `fp` - A pointer to the frame pointer.
        /// * `register_state` - A mutable reference to the `RegisterState` struct where the registers will be stored.
        pub fn get_callee_saved_registers_from_entry_frame(
            _fp: *mut std::ffi::c_void,
            _register_state: &mut RegisterState,
        ) {
            // This function is currently empty in the C++ implementation.
        }
    }
}