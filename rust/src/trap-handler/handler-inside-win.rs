// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file corresponds to src/trap-handler/handler-inside-win.h

#![allow(non_camel_case_types)]

use winapi::shared::minwindef::LONG;
use winapi::um::winnt::EXCEPTION_POINTERS;

pub mod trap_handler {
    extern "C" {
        /// Handles a WebAssembly trap on Windows.
        pub fn HandleWasmTrap(exception: *mut EXCEPTION_POINTERS) -> LONG;

        /// Attempts to handle a WebAssembly trap on Windows, being careful to
        /// not access ASan shadow memory.
        pub fn TryHandleWasmTrap(exception: *mut EXCEPTION_POINTERS) -> bool;
    }
}