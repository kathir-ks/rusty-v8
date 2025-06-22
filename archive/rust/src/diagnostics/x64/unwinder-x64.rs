// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod diagnostics {
    pub mod unwinder {
        /// Represents the state of registers.  The actual contents
        /// are architecture specific.
        pub struct RegisterState {}

        /// Retrieves callee-saved registers from the entry frame.
        ///
        /// # Arguments
        ///
        /// * `fp`: Frame pointer.
        /// * `register_state`: Mutable reference to a `RegisterState` struct
        ///   where the register values will be stored.
        pub fn get_callee_saved_registers_from_entry_frame(
            fp: *mut std::ffi::c_void,
            register_state: &mut RegisterState,
        ) {
            // This function is intentionally empty in the C++ version.
        }
    }
}