// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Used for building without embedded data.

#[no_mangle]
pub static v8_Default_embedded_blob_code_: [u8; 1] = [0];

#[no_mangle]
pub static v8_Default_embedded_blob_code_size_: u32 = 0;

#[no_mangle]
pub static v8_Default_embedded_blob_data_: [u8; 1] = [0];

#[no_mangle]
pub static v8_Default_embedded_blob_data_size_: u32 = 0;

// V8_ENABLE_DRUMBRAKE functionality is not directly translatable to Rust
// because it involves function pointers and a macro that creates extern
// declarations and initializes them to null. A Rust equivalent would
// require unsafe code and a more complex structure.  The following is
// a placeholder for the intended functionality.

#[cfg(feature = "drumbrake")]
mod drumbrake {
    // Placeholder for FOREACH_LOAD_STORE_INSTR_HANDLER
    // In C++, this was a macro that declared function pointers.
    // Rust doesn't directly translate to this pattern cleanly.
    // A possible implementation might involve a trait and a struct
    // to hold the function pointers, but without the actual
    // definition of FOREACH_LOAD_STORE_INSTR_HANDLER, it's
    // impossible to provide a complete and correct translation.

    // Example of how one might represent a single function pointer:
    // #[no_mangle]
    // pub static mut Builtins_LoadInt8: Option<unsafe extern "C" fn()> = None;
}
