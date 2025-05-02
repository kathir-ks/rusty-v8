// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::mem::transmute;
use std::os::raw::{c_int, c_void};
use std::sync::atomic::{AtomicBool, Ordering};

#[cfg(target_os = "linux")]
use libc::ucontext;

#[cfg(target_os = "macos")]
use libc::ucontext_t as ucontext;

#[cfg(target_os = "freebsd")]
use libc::ucontext_t as ucontext;

#[cfg(target_os = "linux")]
use libc::siginfo_t;

#[cfg(target_os = "macos")]
use libc::siginfo as siginfo_t;

#[cfg(target_os = "freebsd")]
use libc::siginfo as siginfo_t;

// Placeholder for trap-handler-internal.h and trap-handler.h
mod trap_handler_internal {
    use std::sync::atomic::{AtomicBool, Ordering};
    pub static g_thread_in_wasm_code: AtomicBool = AtomicBool::new(false);

    extern "C" {
        pub fn RemoveTrapHandler();
    }

    pub fn is_thread_in_wasm_code() -> bool {
        g_thread_in_wasm_code.load(Ordering::Relaxed)
    }

    pub fn set_thread_in_wasm_code(value: bool) {
        g_thread_in_wasm_code.store(value, Ordering::Relaxed);
    }
}

mod trap_handler {
    extern "C" {
        pub fn IsAccessedMemoryCovered(access_addr: usize) -> bool;
        pub static mut gLandingPad: usize;
        pub fn IsFaultAddressCovered(fault_addr: usize) -> bool;
    }
}

// Placeholder for trap-handler-simulator.h
#[cfg(feature = "simulator")]
mod trap_handler_simulator {
    extern "C" {
        pub static mut probe_memory_continuation: usize;
        pub fn ProbeMemory();
    }
}

pub mod handler_inside_posix {
    use super::*;
    use libc::{pthread_sigmask, raise, sigaddset, sigemptyset, sigset_t, SIG_SETMASK, SIG_UNBLOCK};
    use std::mem;
    use std::os::raw::c_int;
    use trap_handler::gLandingPad;
    use trap_handler_internal::{set_thread_in_wasm_code, RemoveTrapHandler};

    #[cfg(target_os = "linux")]
    use libc::{SI_ASYNCIO, SI_MESGQ, SI_QUEUE, SI_TIMER, SI_USER};

    #[cfg(target_os = "macos")]
    const SI_USER: i32 = 0;
    #[cfg(target_os = "macos")]
    const SI_QUEUE: i32 = 0;
    #[cfg(target_os = "macos")]
    const SI_TIMER: i32 = 0;
    #[cfg(target_os = "macos")]
    const SI_ASYNCIO: i32 = 0;
    #[cfg(target_os = "macos")]
    const SI_MESGQ: i32 = 0;

    extern "C" {
        pub static kOobSignal: c_int;
    }

    // Implement CONTEXT_REG macro as a function
    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    unsafe fn context_reg(uc: *mut ucontext, reg: usize) -> *mut u64 {
        &mut (*uc).uc_mcontext.regs[reg] as *mut u64
    }

    #[cfg(all(target_os = "linux", any(target_arch = "loongarch64", target_arch = "riscv64")))]
    unsafe fn context_reg(uc: *mut ucontext, reg: usize) -> *mut i64 {
        &mut (*uc).uc_mcontext.__gregs[reg] as *mut i64
    }

    #[cfg(all(target_os = "linux", not(any(target_arch = "aarch64", target_arch = "loongarch64", target_arch = "riscv64"))))]
    unsafe fn context_reg(uc: *mut ucontext, reg: usize) -> *mut i64 {
        let reg_enum = match reg {
            0 => libc::REG_R8,
            1 => libc::REG_R9,
            2 => libc::REG_R10,
            3 => libc::REG_R11,
            4 => libc::REG_R12,
            5 => libc::REG_R13,
            6 => libc::REG_R14,
            7 => libc::REG_R15,
            8 => libc::REG_RDI,
            9 => libc::REG_RSI,
            10 => libc::REG_RBP,
            11 => libc::REG_RBX,
            12 => libc::REG_RDX,
            13 => libc::REG_RAX,
            14 => libc::REG_RCX,
            15 => libc::REG_RSP,
            16 => libc::REG_RIP,
            17 => libc::REG_EFL,
            18 => libc::REG_CSGSFS,
            19 => libc::REG_ERR,
            20 => libc::REG_TRAPNO,
            21 => libc::REG_OLDMASK,
            22 => libc::REG_CR2,
            _ => panic!("Invalid register index"),
        };
        &mut (*uc).uc_mcontext.gregs[reg_enum as usize] as *mut i64
    }

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    unsafe fn context_reg(uc: *mut ucontext, reg: usize) -> *mut u64 {
        &mut (*uc).uc_mcontext.__ss.__x[reg] as *mut u64
    }

    #[cfg(all(target_os = "macos", not(target_arch = "aarch64")))]
    unsafe fn context_reg(uc: *mut ucontext, reg: &str) -> *mut i64 {
        match reg {
            "rax" => &mut (*uc).uc_mcontext.__ss.__rax as *mut i64,
            "rbx" => &mut (*uc).uc_mcontext.__ss.__rbx as *mut i64,
            "rcx" => &mut (*uc).uc_mcontext.__ss.__rcx as *mut i64,
            "rdx" => &mut (*uc).uc_mcontext.__ss.__rdx as *mut i64,
            "rdi" => &mut (*uc).uc_mcontext.__ss.__rdi as *mut i64,
            "rsi" => &mut (*uc).uc_mcontext.__ss.__rsi as *mut i64,
            "rbp" => &mut (*uc).uc_mcontext.__ss.__rbp as *mut i64,
            "rsp" => &mut (*uc).uc_mcontext.__ss.__rsp as *mut i64,
            "r8" => &mut (*uc).uc_mcontext.__ss.__r8 as *mut i64,
            "r9" => &mut (*uc).uc_mcontext.__ss.__r9 as *mut i64,
            "r10" => &mut (*uc).uc_mcontext.__ss.__r10 as *mut i64,
            "r11" => &mut (*uc).uc_mcontext.__ss.__r11 as *mut i64,
            "r12" => &mut (*uc).uc_mcontext.__ss.__r12 as *mut i64,
            "r13" => &mut (*uc).uc_mcontext.__ss.__r13 as *mut i64,
            "r14" => &mut (*uc).uc_mcontext.__ss.__r14 as *mut i64,
            "r15" => &mut (*uc).uc_mcontext.__ss.__r15 as *mut i64,
            "rip" => &mut (*uc).uc_mcontext.__ss.__rip as *mut i64,
            _ => panic!("Unsupported register"),
        }
    }

    #[cfg(target_os = "freebsd")]
    unsafe fn context_reg(uc: *mut ucontext, reg: &str) -> *mut i64 {
        match reg {
            "rax" => &mut (*uc).uc_mcontext.mc_rax as *mut i64,
            "rbx" => &mut (*uc).uc_mcontext.mc_rbx as *mut i64,
            "rcx" => &mut (*uc).uc_mcontext.mc_rcx as *mut i64,
            "rdx" => &mut (*uc).uc_mcontext.mc_rdx as *mut i64,
            "rdi" => &mut (*uc).uc_mcontext.mc_rdi as *mut i64,
            "rsi" => &mut (*uc).uc_mcontext.mc_rsi as *mut i64,
            "rbp" => &mut (*uc).uc_mcontext.mc_rbp as *mut i64,
            "rsp" => &mut (*uc).uc_mcontext.mc_rsp as *mut i64,
            "r8" => &mut (*uc).uc_mcontext.mc_r8 as *mut i64,
            "r9" => &mut (*uc).uc_mcontext.mc_r9 as *mut i64,
            "r10" => &mut (*uc).uc_mcontext.mc_r10 as *mut i64,
            "r11" => &mut (*uc).uc_mcontext.mc_r11 as *mut i64,
            "r12" => &mut (*uc).uc_mcontext.mc_r12 as *mut i64,
            "r13" => &mut (*uc).uc_mcontext.mc_r13 as *mut i64,
            "r14" => &mut (*uc).uc_mcontext.mc_r14 as *mut i64,
            "r15" => &mut (*uc).uc_mcontext.mc_r15 as *mut i64,
            "rip" => &mut (*uc).uc_mcontext.mc_rip as *mut i64,
            _ => panic!("Unsupported register"),
        }
    }

    // Implement CONTEXT_PC macro as a function
    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    unsafe fn context_pc(uc: *mut ucontext) -> *mut u64 {
        &mut (*uc).uc_mcontext.pc as *mut u64
    }

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    unsafe fn context_pc(uc: *mut ucontext) -> *mut u64 {
        &mut (*uc).uc_mcontext.__ss.__pc as *mut u64
    }

    #[cfg(all(target_os = "linux", target_arch = "loongarch64"))]
    unsafe fn context_pc(uc: *mut ucontext) -> *mut i64 {
        &mut (*uc).uc_mcontext.__pc as *mut i64
    }

    #[cfg(all(target_os = "linux", target_arch = "riscv64"))]
    unsafe fn context_pc(uc: *mut ucontext) -> *mut i64 {
        &mut (*uc).uc_mcontext.__gregs[libc::REG_PC as usize] as *mut i64
    }

    pub fn is_kernel_generated_signal(info: *mut siginfo_t) -> bool {
        unsafe {
            // On macOS, only `info->si_code > 0` is relevant, because macOS leaves
            // si_code at its default of 0 for signals that don’t originate in hardware.
            // The other conditions are only relevant for Linux.
            #[cfg(target_os = "macos")]
            return (*info).si_code > 0;

            #[cfg(target_os = "linux")]
            return (*info).si_code > 0
                && (*info).si_code != SI_USER
                && (*info).si_code != SI_QUEUE
                && (*info).si_code != SI_TIMER
                && (*info).si_code != SI_ASYNCIO
                && (*info).si_code != SI_MESGQ;

            #[cfg(target_os = "freebsd")]
            return (*info).si_code > 0
                && (*info).si_code != SI_USER
                && (*info).si_code != SI_QUEUE
                && (*info).si_code != SI_TIMER
                && (*info).si_code != SI_ASYNCIO
                && (*info).si_code != SI_MESGQ;
        }
    }

    struct UnmaskOobSignalScope {
        old_mask_: sigset_t,
    }

    impl UnmaskOobSignalScope {
        pub fn new() -> Self {
            let mut sigs: sigset_t = unsafe { mem::zeroed() };
            // Fortunately, sigemptyset and sigaddset are async-signal-safe according to
            // the POSIX standard.
            unsafe {
                sigemptyset(&mut sigs);
                sigaddset(&mut sigs, kOobSignal);
            }
            let mut old_mask_: sigset_t = unsafe { mem::zeroed() };
            unsafe {
                pthread_sigmask(SIG_UNBLOCK, &sigs, &mut old_mask_);
            }
            Self { old_mask_ }
        }
    }

    impl Drop for UnmaskOobSignalScope {
        fn drop(&mut self) {
            unsafe {
                pthread_sigmask(SIG_SETMASK, &self.old_mask_, std::ptr::null_mut());
            }
        }
    }

    #[cfg(feature = "simulator")]
    extern "C" {
        pub static mut probe_memory_continuation: usize;
        pub fn ProbeMemory();
    }

    pub fn try_handle_signal(signum: c_int, info: *mut siginfo_t, context: *mut c_void) -> bool {
        // Ensure the faulting thread was actually running Wasm code. This should be
        // the first check in the trap handler to guarantee that the
        // g_thread_in_wasm_code flag is only set in wasm code. Otherwise a later
        // signal handler is executed with the flag set.
        use trap_handler_internal::g_thread_in_wasm_code;
        if !g_thread_in_wasm_code.load(Ordering::Relaxed) {
            return false;
        }

        // Clear g_thread_in_wasm_code, primarily to protect against nested faults.
        // The only path that resets the flag to true is if we find a landing pad (in
        // which case this function returns true). Otherwise we leave the flag unset
        // since we do not return to wasm code.
        g_thread_in_wasm_code.store(false, Ordering::Relaxed);

        // Bail out early in case we got called for the wrong kind of signal.
        if signum != unsafe { kOobSignal } {
            return false;
        }

        // Make sure the signal was generated by the kernel and not some other source.
        if !is_kernel_generated_signal(info) {
            return false;
        }

        // Check whether the fault should be handled based on the accessed address.
        // A fault caused by an access to an address that cannot belong to a Wasm
        // memory object should not be handled.
        let access_addr = unsafe { (*info).si_addr } as usize;
        if unsafe { !trap_handler::IsAccessedMemoryCovered(access_addr) } {
            return false;
        }

        // Unmask the oob signal, which is automatically masked during the execution
        // of this handler. This ensures that crashes generated in this function will
        // be handled by the crash reporter. Otherwise, the process might be killed
        // with the crash going unreported. The scope object makes sure to restore the
        // signal mask on return from this function. We put the scope object in a
        // separate block to ensure that we restore the signal mask before we restore
        // the g_thread_in_wasm_code flag.
        {
            let _unmask_oob_signal = UnmaskOobSignalScope::new();

            let uc = context as *mut ucontext;
            #[cfg(target_arch = "x86_64")]
            let context_ip = unsafe { context_reg(uc, libc::REG_RIP as usize) as *mut i64 };
            #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
            let context_ip = unsafe { context_pc(uc) as *mut u64 };
            #[cfg(all(target_os = "linux", target_arch = "loongarch64"))]
            let context_ip = unsafe { context_pc(uc) as *mut i64 };
            #[cfg(all(target_os = "linux", target_arch = "riscv64"))]
            let context_ip = unsafe { context_pc(uc) as *mut i64 };
            #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
            let context_ip = unsafe { context_pc(uc) as *mut u64 };

            let fault_addr = unsafe { *context_ip } as usize;

            #[cfg(feature = "simulator")]
            {
                // Only handle signals triggered by the load in {ProbeMemory}.
                if fault_addr != unsafe { trap_handler_simulator::ProbeMemory as usize } {
                    return false;
                }

                // The simulated ip will be in the second parameter register (%rsi).
                let simulated_ip_reg = unsafe { context_reg(uc, 1) as *mut i64 };
                if unsafe { !trap_handler::IsFaultAddressCovered(*simulated_ip_reg as usize) } {
                    return false;
                }
                assert!(unsafe { gLandingPad != 0 });

                unsafe {
                    let return_reg = context_reg(uc, 0) as *mut i64;
                    *return_reg = gLandingPad as i64;
                    // The fault_address that is set in non-simulator builds here is set in the
                    // simulator directly.
                    // Continue at the memory probing continuation.
                    *context_ip = trap_handler_simulator::probe_memory_continuation as usize as i64;
                }
            }

            #[cfg(not(feature = "simulator"))]
            {
                if unsafe { !trap_handler::IsFaultAddressCovered(fault_addr) } {
                    return false;
                }
                assert!(unsafe { gLandingPad != 0 });
                // Tell the caller to return to the landing pad.
                unsafe { *context_ip = gLandingPad as i64 };

                #[cfg(target_arch = "x86_64")]
                let fault_address_reg = unsafe { context_reg(uc, libc::REG_R10 as usize) as *mut i64 };
                #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
                let fault_address_reg = unsafe { context_reg(uc, 16) as *mut u64 };
                #[cfg(all(target_os = "linux", target_arch = "loongarch64"))]
                let fault_address_reg = unsafe { context_reg(uc, libc::REG_T6 as usize) as *mut i64 };
                #[cfg(all(target_os = "linux", target_arch = "riscv64"))]
                let fault_address_reg = unsafe { context_reg(uc, libc::REG_T6 as usize) as *mut i64 };

                unsafe { *fault_address_reg = fault_addr as i64 };
            }
        }
        // We will return to wasm code, so restore the g_thread_in_wasm_code flag.
        // This should only be done once the signal is blocked again (outside the
        // {UnmaskOobSignalScope}) to ensure that we do not catch a signal we raise
        // inside of the handler.
        set_thread_in_wasm_code(true);
        true
    }

    pub fn handle_signal(signum: c_int, info: *mut siginfo_t, context: *mut c_void) {
        if !try_handle_signal(signum, info, context) {
            // Since V8 didn't handle this signal, we want to re-raise the same signal.
            // For kernel-generated signals, we do this by restoring the original
            // handler and then returning. The fault will happen again and the usual
            // signal handling will happen.
            //
            // We handle user-generated signals by calling raise() instead. This is for
            // completeness. We should never actually see one of these, but just in
            // case, we do the right thing.
            unsafe {
                RemoveTrapHandler();
            }
            if !is_kernel_generated_signal(info) {
                unsafe {
                    raise(signum);
                }
            }
        }
        // TryHandleSignal modifies context to change where we return to.
    }
}