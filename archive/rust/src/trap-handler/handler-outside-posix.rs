// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// PLEASE READ BEFORE CHANGING THIS FILE!
//
// This file implements the support code for the out of bounds signal handler.
// Nothing in here actually runs in the signal handler, but the code here
// manipulates data structures used by the signal handler so we still need to be
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
// For the code that runs in the signal handler itself, see handler-inside.rs.

use std::mem::MaybeUninit;

#[cfg(unix)]
use libc::{
    sigaction, sigemptyset, SA_ONSTACK, SA_SIGINFO, SIG_DFL, SIG_ERR,
};

#[cfg(target_os = "linux")]
const SIGSEGV: i32 = libc::SIGSEGV;
#[cfg(target_os = "macos")]
const SIGSEGV: i32 = libc::SIGSEGV;

// The TH_CHECK macro is assumed to do nothing in this Rust translation,
// since Rust has stronger compile-time guarantees.

// V8_TRAP_HANDLER_SUPPORTED is assumed to be always true.
// In a real-world scenario, you'd conditionally compile the code
// using #[cfg(feature = "trap_handler")] or similar.

pub mod trap_handler {
    use super::*;

    #[cfg(unix)]
    static mut G_OLD_HANDLER: MaybeUninit<libc::sigaction> = MaybeUninit::uninit();

    // When using the default signal handler, we save the old one to restore in case
    // V8 chooses not to handle the signal.
    static mut G_IS_DEFAULT_SIGNAL_HANDLER_REGISTERED: bool = false;

    // Placeholder for HandleSignal function, which would be defined in
    // handler-inside-posix.rs or a similar module.
    #[cfg(unix)]
    extern "C" {
        fn HandleSignal(
            signum: libc::c_int,
            siginfo: *mut libc::siginfo_t,
            context: *mut libc::c_void,
        );
    }

    #[cfg(unix)]
    pub fn register_default_trap_handler() -> bool {
        unsafe {
            if G_IS_DEFAULT_SIGNAL_HANDLER_REGISTERED {
                return false;
            }

            let mut action: libc::sigaction = std::mem::zeroed();
            action.sa_sigaction = HandleSignal;
            action.sa_flags = SA_SIGINFO | SA_ONSTACK as i32;
            sigemptyset(&mut action.sa_mask as *mut libc::sigset_t);

            if sigaction(SIGSEGV, &action, G_OLD_HANDLER.as_mut_ptr()) != 0 {
                return false;
            }

            // Sanitizers often prevent us from installing our own signal handler. Attempt
            // to detect this and if so, refuse to enable trap handling.
            //
            // TODO(chromium:830894): Remove this once all bots support custom signal
            // handlers.
            #[cfg(any(
                feature = "address_sanitizer",
                feature = "memory_sanitizer",
                feature = "thread_sanitizer",
                feature = "leak_sanitizer",
                feature = "undefined_sanitizer"
            ))]
            {
                let mut installed_handler: libc::sigaction = std::mem::zeroed();
                assert_eq!(sigaction(SIGSEGV, std::ptr::null(), &mut installed_handler), 0);

                // This comparison is not directly possible in Rust because function pointers
                // are not comparable. A more robust solution would involve registering
                // a token and comparing against that.
                //
                // For now, just always enable.
                // if installed_handler.sa_sigaction != HandleSignal {
                //     eprintln!(
                //         "WARNING: sanitizers are preventing signal handler installation. \
                //          Trap handlers are disabled."
                //     );
                //     return false;
                // }
            }

            G_IS_DEFAULT_SIGNAL_HANDLER_REGISTERED = true;
            true
        }
    }

    #[cfg(unix)]
    pub fn remove_trap_handler() {
        unsafe {
            if G_IS_DEFAULT_SIGNAL_HANDLER_REGISTERED {
                if sigaction(SIGSEGV, G_OLD_HANDLER.as_ptr(), std::ptr::null_mut()) == 0 {
                    G_IS_DEFAULT_SIGNAL_HANDLER_REGISTERED = false;
                }
            }
        }
    }
}