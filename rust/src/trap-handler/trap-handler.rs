// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod trap_handler {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Mutex;

    // Placeholder for v8config.h and base/immediate-crash.h functionality
    // These would require a full understanding of V8's build system and crash reporting
    // which is out of scope.  For now, we'll use panic! as a stand-in for IMMEDIATE_CRASH.

    macro_rules! th_check {
        ($condition:expr) => {
            if !($condition) {
                panic!("TH_CHECK failed: {}", stringify!($condition));
            }
        };
    }

    #[cfg(debug_assertions)]
    macro_rules! th_dcheck {
        ($condition:expr) => {
            th_check!($condition);
        };
    }

    #[cfg(not(debug_assertions))]
    macro_rules! th_dcheck {
        ($condition:expr) => {
            let _ = $condition;
        };
    }

    // Placeholder for address sanitizer
    macro_rules! th_disable_asan {
        () => {};
    }

    #[derive(Debug, Copy, Clone)]
    pub struct ProtectedInstructionData {
        pub instr_offset: u32,
    }

    pub const K_INVALID_INDEX: i32 = -1;

    extern "C" {
        pub fn RegisterHandlerData(
            base: usize,
            size: usize,
            num_protected_instructions: usize,
            protected_instructions: *const ProtectedInstructionData,
        ) -> i32;

        pub fn ReleaseHandlerData(index: i32);

        pub fn RegisterV8Sandbox(base: usize, size: usize) -> bool;

        pub fn UnregisterV8Sandbox(base: usize, size: usize);
    }

    pub static mut G_IS_TRAP_HANDLER_ENABLED: bool = false;
    pub static G_CAN_ENABLE_TRAP_HANDLER: AtomicBool = AtomicBool::new(true);

    extern "C" {
        pub fn EnableTrapHandler(use_v8_handler: bool) -> bool;

        pub fn SetLandingPad(landing_pad: usize);

        pub fn GetThreadInWasmThreadLocalAddress() -> *mut i32;

        pub fn RemoveTrapHandler();

        pub fn GetRecoveredTrapCount() -> usize;

        pub fn AssertThreadNotInWasm();
    }

    thread_local! {
        pub static G_THREAD_IN_WASM_CODE: std::cell::RefCell<i32> = std::cell::RefCell::new(0);
    }

    pub fn is_trap_handler_enabled() -> bool {
        th_dcheck!(!unsafe { G_IS_TRAP_HANDLER_ENABLED } || cfg!(feature = "trap_handler_supported"));

        if G_CAN_ENABLE_TRAP_HANDLER.load(Ordering::Relaxed) {
            G_CAN_ENABLE_TRAP_HANDLER.store(false, Ordering::Relaxed);
        }

        unsafe { G_IS_TRAP_HANDLER_ENABLED }
    }

    #[th_disable_asan!()]
    pub fn is_thread_in_wasm() -> bool {
        G_THREAD_IN_WASM_CODE.with(|f| *f.borrow() != 0)
    }

    pub fn set_thread_in_wasm() {
        if is_trap_handler_enabled() {
            th_dcheck!(!is_thread_in_wasm());
            G_THREAD_IN_WASM_CODE.with(|f| *f.borrow_mut() = 1);
        }
    }

    pub fn clear_thread_in_wasm() {
        if is_trap_handler_enabled() {
            th_dcheck!(is_thread_in_wasm());
            G_THREAD_IN_WASM_CODE.with(|f| *f.borrow_mut() = 0);
        }
    }

    extern "C" {
        pub fn RegisterDefaultTrapHandler() -> bool;
    }
}