// Converted from V8 C++ source files:
// Header: handler-inside-posix.h
// Implementation: handler-inside-posix.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod trap_handler_internal;
mod trap_handler;

use std::os::raw::c_int;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{mem, ptr};

#[cfg(target_os = "linux")]
const kOobSignal: c_int = libc::SIGSEGV;
#[cfg(target_os = "freebsd")]
const kOobSignal: c_int = libc::SIGSEGV;
#[cfg(target_os = "macos")]
const kOobSignal: c_int = libc::SIGBUS;

extern "C" {
    static mut g_thread_in_wasm_code: bool;
    static mut gLandingPad: usize;
}

pub fn handle_signal(signum: c_int, info: *mut libc::siginfo_t, context: *mut libc::c_void) {
    if !try_handle_signal(signum, info, context) {
        unsafe {
            trap_handler::remove_trap_handler();
            if (*info).si_code <= 0 || (*info).si_code == libc::SI_USER || (*info).si_code == libc::SI_QUEUE
               || (*info).si_code == libc::SI_TIMER || (*info).si_code == libc::SI_ASYNCIO
               || (*info).si_code == libc::SI_MESGQ
            {
                libc::raise(signum);
            }
        }
    }
}

#[cfg(target_arch = "x86_64")]
unsafe fn get_context_ip(context: *mut libc::c_void) -> *mut u64 {
    let uc = context as *mut libc::ucontext_t;
    &mut (*uc).uc_mcontext.gregs[libc::REG_RIP as usize] as *mut i64 as *mut u64
}

#[cfg(target_arch = "aarch64")]
unsafe fn get_context_ip(context: *mut libc::c_void) -> *mut u64 {
    let uc = context as *mut libc::ucontext_t;
    &mut (*uc).uc_mcontext.pc as *mut u64
}

#[cfg(target_arch = "loongarch64")]
unsafe fn get_context_ip(context: *mut libc::c_void) -> *mut u64 {
    let uc = context as *mut libc::ucontext_t;
    &mut (*uc).uc_mcontext.__pc as *mut u64
}

#[cfg(target_arch = "riscv64")]
unsafe fn get_context_ip(context: *mut libc::c_void) -> *mut u64 {
    let uc = context as *mut libc::ucontext_t;
    &mut (*uc).uc_mcontext.__gregs[libc::REG_PC as usize] as *mut i64 as *mut u64
}

#[cfg(target_arch = "x86_64")]
unsafe fn get_context_reg_r10(context: *mut libc::c_void) -> *mut u64 {
    let uc = context as *mut libc::ucontext_t;
    &mut (*uc).uc_mcontext.gregs[libc::REG_R10 as usize] as *mut i64 as *mut u64
}

#[cfg(target_arch = "aarch64")]
unsafe fn get_context_reg_x16(context: *mut libc::c_void) -> *mut u64 {
    let uc = context as *mut libc::ucontext_t;
    let offset = 16;
    &mut (*uc).uc_mcontext.__ss.__x[offset] as *mut u64
}

#[cfg(target_arch = "loongarch64")]
unsafe fn get_context_reg_t6(context: *mut libc::c_void) -> *mut u64 {
    let uc = context as *mut libc::ucontext_t;
    let offset = 18;
    &mut (*uc).uc_mcontext.__gregs[offset] as *mut i64 as *mut u64
}

#[cfg(target_arch = "riscv64")]
unsafe fn get_context_reg_t6(context: *mut libc::c_void) -> *mut u64 {
    let uc = context as *mut libc::ucontext_t;
    let offset = 18;
    &mut (*uc).uc_mcontext.__gregs[offset] as *mut i64 as *mut u64
}

#[cfg(target_arch = "x86_64")]
unsafe fn get_context_reg_rax(context: *mut libc::c_void) -> *mut u64 {
    let uc = context as *mut libc::ucontext_t;
    &mut (*uc).uc_mcontext.gregs[libc::REG_RAX as usize] as *mut i64 as *mut u64
}

#[cfg(target_arch = "x86_64")]
unsafe fn get_context_reg_rsi(context: *mut libc::c_void) -> *mut u64 {
    let uc = context as *mut libc::ucontext_t;
    &mut (*uc).uc_mcontext.gregs[libc::REG_RSI as usize] as *mut i64 as *mut u64
}

fn is_kernel_generated_signal(info: *mut libc::siginfo_t) -> bool {
    unsafe {
        if cfg!(target_os = "macos") {
            (*info).si_code > 0
        } else {
            (*info).si_code > 0 && (*info).si_code != libc::SI_USER as i32 && (*info).si_code != libc::SI_QUEUE as i32 &&
            (*info).si_code != libc::SI_TIMER as i32 && (*info).si_code != libc::SI_ASYNCIO as i32 &&
            (*info).si_code != libc::SI_MESGQ as i32
        }
    }
}

struct UnmaskOobSignalScope {
    old_mask_: libc::sigset_t,
}

impl UnmaskOobSignalScope {
    fn new() -> Self {
        let mut sigs: libc::sigset_t = unsafe { mem::zeroed() };
        unsafe {
            libc::sigemptyset(&mut sigs);
            libc::sigaddset(&mut sigs, kOobSignal);
            libc::pthread_sigmask(libc::SIG_UNBLOCK, &sigs, &mut Self { old_mask_: unsafe { mem::zeroed() } }.old_mask_);
        }
        UnmaskOobSignalScope { old_mask_: sigs }
    }
}

impl Drop for UnmaskOobSignalScope {
    fn drop(&mut self) {
        unsafe {
            libc::pthread_sigmask(libc::SIG_SETMASK, &self.old_mask_, ptr::null_mut());
        }
    }
}

extern "C" {
    static probe_memory_continuation: [libc::c_char; 0];
}

fn try_handle_signal(signum: c_int, info: *mut libc::siginfo_t, context: *mut libc::c_void) -> bool {
    unsafe {
        if !g_thread_in_wasm_code {
            return false;
        }

        g_thread_in_wasm_code = false;

        if signum != kOobSignal {
            return false;
        }

        if !is_kernel_generated_signal(info) {
            return false;
        }

        let access_addr = (*info).si_addr as usize;
        if !trap_handler::is_accessed_memory_covered(access_addr) {
            return false;
        }

        {
            let _unmask_oob_signal = UnmaskOobSignalScope::new();

            let uc = context as *mut libc::ucontext_t;
            let context_ip = get_context_ip(context as *mut libc::c_void);
            let fault_addr = *context_ip;

            #[cfg(feature = "trap_handler_via_simulator")]
            {
                if fault_addr != probe_memory_continuation.as_ptr() as usize {
                    return false;
                }

                let simulated_ip_reg = get_context_reg_rsi(context as *mut libc::c_void);
                if !trap_handler::is_fault_address_covered(*simulated_ip_reg as usize) {
                    return false;
                }

                if gLandingPad == 0 {
                    panic!("gLandingPad is null");
                }

                let return_reg = get_context_reg_rax(context as *mut libc::c_void);
                *return_reg = gLandingPad as u64;

                *context_ip = &probe_memory_continuation as *const [libc::c_char; 0] as usize as u64;
            }

            #[cfg(not(feature = "trap_handler_via_simulator"))]
            {
                if !trap_handler::is_fault_address_covered(fault_addr as usize) {
                    return false;
                }

                if gLandingPad == 0 {
                    panic!("gLandingPad is null");
                }

                *context_ip = gLandingPad as u64;

                let fault_address_reg = if cfg!(target_arch = "x86_64") {
                    get_context_reg_r10(context as *mut libc::c_void)
                } else if cfg!(target_arch = "aarch64") {
                    get_context_reg_x16(context as *mut libc::c_void)
                } else if cfg!(target_arch = "loongarch64") {
                    get_context_reg_t6(context as *mut libc::c_void)
                } else if cfg!(target_arch = "riscv64") {
                    get_context_reg_t6(context as *mut libc::c_void)
                } else {
                    panic!("Unsupported architecture.");
                };
                *fault_address_reg = fault_addr;
            }
        }
        g_thread_in_wasm_code = true;
        return true;
    }
}

mod internal {
    pub mod trap_handler {
        pub fn is_accessed_memory_covered(access_addr: usize) -> bool {
            true
        }
        pub fn is_fault_address_covered(fault_addr: usize) -> bool {
            true
        }
    }
}
mod trap_handler {
    pub fn remove_trap_handler() {}
}
