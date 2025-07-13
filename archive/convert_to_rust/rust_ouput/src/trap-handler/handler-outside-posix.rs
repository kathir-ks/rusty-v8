// Converted from V8 C++ source files:
// Header: N/A
// Implementation: handler-outside-posix.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

use std::sync::Mutex;
use std::sync::Arc;
use std::os::raw::c_int;

extern "C" {
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sigset_t {
    __val: [u64; 16],
}

#[repr(C)]
struct sigaction {
    sa_sigaction: extern "C" fn(c_int, *mut siginfo_t, *mut std::ffi::c_void),
    sa_mask: sigset_t,
    sa_flags: c_int,
    _padding: [c_int; 29],
}

#[repr(C)]
struct siginfo_t {
  si_signo: c_int,
  si_errno: c_int,
  si_code: c_int,
  _pad: [c_int; 29],
}

pub mod trap_handler {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use lazy_static::lazy_static;

    const SA_SIGINFO: i32 = 4;
    const SA_ONSTACK: i32 = 0x08000000;
    const kOobSignal: i32 = 11; // Assuming SIGSEGV is 11, replace if different
    
    lazy_static! {
        static ref OLD_HANDLER: Mutex<sigaction> = Mutex::new(sigaction {
            sa_sigaction: dummy_handler,
            sa_mask: sigset_t { __val: [0; 16] },
            sa_flags: 0,
            _padding: [0; 29],
        });
        static ref IS_DEFAULT_SIGNAL_HANDLER_REGISTERED: AtomicBool = AtomicBool::new(false);
    }

    extern "C" fn dummy_handler(_: c_int, _: *mut siginfo_t, _: *mut std::ffi::c_void) {}

    extern "C" fn handle_signal(
        signum: c_int,
        siginfo: *mut siginfo_t,
        context: *mut std::ffi::c_void,
    ) {
        println!("Signal {} received", signum);
    }

    pub fn register_default_trap_handler() -> Result<(), String> {
        if IS_DEFAULT_SIGNAL_HANDLER_REGISTERED.load(Ordering::SeqCst) {
            return Err("Default signal handler already registered".to_string());
        }

        let mut action = sigaction {
            sa_sigaction: handle_signal,
            sa_mask: sigset_t { __val: [0; 16] },
            sa_flags: SA_SIGINFO | SA_ONSTACK,
            _padding: [0; 29],
        };

        unsafe {
            if sigemptyset(&mut action.sa_mask) != 0 {
                return Err("sigemptyset failed".to_string());
            }

            let mut old_handler = OLD_HANDLER.lock().unwrap();
            if sigaction(kOobSignal, &action, &mut *old_handler) != 0 {
                return Err("sigaction failed".to_string());
            }

            #[cfg(any(
                target_os = "linux",
                target_os = "android",
                target_os = "freebsd"
            ))]
            {
                let mut installed_handler = sigaction {
                    sa_sigaction: dummy_handler,
                    sa_mask: sigset_t { __val: [0; 16] },
                    sa_flags: 0,
                    _padding: [0; 29],
                };
                if sigaction(kOobSignal, std::ptr::null(), &mut installed_handler) != 0 {
                    return Err("sigaction(NULL) failed".to_string());
                }

                if installed_handler.sa_sigaction as usize != handle_signal as usize {
                    println!(
                        "WARNING: sanitizers are preventing signal handler installation. \
                         Trap handlers are disabled."
                    );
                    return Ok(());
                }
            }

            IS_DEFAULT_SIGNAL_HANDLER_REGISTERED.store(true, Ordering::SeqCst);
            Ok(())
        }
    }

    pub fn remove_trap_handler() -> Result<(), String> {
        if IS_DEFAULT_SIGNAL_HANDLER_REGISTERED.load(Ordering::SeqCst) {
            unsafe {
                let old_handler = OLD_HANDLER.lock().unwrap();
                if sigaction(kOobSignal, &*old_handler, std::ptr::null_mut()) != 0 {
                    return Err("sigaction failed to restore old handler".to_string());
                }
                IS_DEFAULT_SIGNAL_HANDLER_REGISTERED.store(false, Ordering::SeqCst);
                Ok(())
            }
        } else {
            Ok(())
        }
    }
}
