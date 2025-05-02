// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// PLEASE READ BEFORE CHANGING THIS FILE!
//
// This file implements the support code for the out of bounds trap handler.
// Nothing in here actually runs in the trap handler, but the code here
// manipulates data structures used by the trap handler so we still need to be
// careful. In order to minimize this risk, here are some rules to follow.
//
// 1. Avoid introducing new external dependencies. The files in src/trap-handler
//    should be as self-contained as possible to make it easy to audit the code.
//
// 2. Any changes must be reviewed by someone from the crash reporting
//    or security team. Se OWNERS for suggested reviewers.
//
// For more information, see https://goo.gl/yMeyUY.
//
// For the code that runs in the trap handler itself, see handler-inside.cc.

#[cfg(target_os = "windows")]
use winapi::um::errhandlingapi::AddVectoredExceptionHandler;
#[cfg(target_os = "windows")]
use winapi::um::errhandlingapi::RemoveVectoredExceptionHandler;
#[cfg(target_os = "windows")]
use winapi::shared::minwindef::{ULONG, TRUE};

#[cfg(target_os = "windows")]
mod handler_inside_win;
#[cfg(target_os = "windows")]
mod trap_handler;

#[cfg(target_os = "windows")]
pub mod trap_handler_outside {
    use super::*;

    // Re-export HandleWasmTrap for use in this module.
    use super::handler_inside_win::HandleWasmTrap;

    #[cfg(target_arch = "x86_64")] // Example, adjust as needed based on V8_TARGET_ARCH_X64
    const V8_TRAP_HANDLER_SUPPORTED: bool = true;
    #[cfg(not(target_arch = "x86_64"))]
    const V8_TRAP_HANDLER_SUPPORTED: bool = false;

    #[cfg(V8_TRAP_HANDLER_SUPPORTED)]
    mod inner {
        use super::*;
        use std::sync::Mutex;

        lazy_static::lazy_static! {
            static ref REGISTERED_HANDLER: Mutex<Option<*mut std::ffi::c_void>> = Mutex::new(None);
        }

        /// Registers the default trap handler using AddVectoredExceptionHandler.
        pub fn register_default_trap_handler() -> bool {
            let mut guard = REGISTERED_HANDLER.lock().unwrap();
            if guard.is_some() {
                return false; // Or panic, depending on how TH_CHECK behaves
            }

            let first: ULONG = TRUE;
            let handler = unsafe { AddVectoredExceptionHandler(first, Some(HandleWasmTrap)) };

            if handler.is_null() {
                return false;
            }

            *guard = Some(handler);
            true
        }

        /// Removes the registered trap handler using RemoveVectoredExceptionHandler.
        pub fn remove_trap_handler() {
            let mut guard = REGISTERED_HANDLER.lock().unwrap();
            if let Some(handler) = guard.take() {
                unsafe { RemoveVectoredExceptionHandler(handler) };
            }
        }
    }

    #[cfg(V8_TRAP_HANDLER_SUPPORTED)]
    pub use inner::{register_default_trap_handler, remove_trap_handler};

    #[cfg(not(V8_TRAP_HANDLER_SUPPORTED))]
    pub fn register_default_trap_handler() -> bool {
        false
    }

    #[cfg(not(V8_TRAP_HANDLER_SUPPORTED))]
    pub fn remove_trap_handler() {}
}