// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/runtime/runtime-utils.h (Placeholder, no direct equivalent needed as the Rust module structure defines the public interface)

#[cfg(feature = "v8_enable_webassembly")]
mod wasm_utils {
    use std::sync::atomic::{AtomicBool, Ordering};

    // Placeholder for logging (src/base/logging.h).  Rust's standard logging can be used.
    macro_rules! dcheck {
        ($x:expr) => {
            if !$x {
                panic!("DCHECK failed: {}", stringify!($x));
            }
        };
    }

    // Placeholder for isolate.  Needs more details for a proper Rust definition.
    pub struct Isolate {
        has_exception: AtomicBool,
    }

    impl Isolate {
        pub fn new() -> Self {
            Isolate {
                has_exception: AtomicBool::new(false),
            }
        }

        pub fn has_exception(&self) -> bool {
            self.has_exception.load(Ordering::Relaxed)
        }

        pub fn set_exception(&self) {
            self.has_exception.store(true, Ordering::Relaxed)
        }
    }

    // Placeholder for trap handler (src/trap-handler/trap-handler.h).
    // Using a simple atomic boolean for demonstration.  Requires more context.
    static TRAP_HANDLER_ENABLED: AtomicBool = AtomicBool::new(false);
    static THREAD_IN_WASM: AtomicBool = AtomicBool::new(false);

    pub fn is_trap_handler_enabled() -> bool {
        TRAP_HANDLER_ENABLED.load(Ordering::Relaxed)
    }

    pub fn set_trap_handler_enabled(enabled: bool) {
        TRAP_HANDLER_ENABLED.store(enabled, Ordering::Relaxed);
    }

    pub fn is_thread_in_wasm() -> bool {
        THREAD_IN_WASM.load(Ordering::Relaxed)
    }

    pub fn set_thread_in_wasm() {
        THREAD_IN_WASM.store(true, Ordering::Relaxed);
    }

    pub fn clear_thread_in_wasm() {
        THREAD_IN_WASM.store(false, Ordering::Relaxed);
    }

    /// RAII guard to save and clear the thread-in-wasm flag.
    pub struct SaveAndClearThreadInWasmFlag<'a> {
        isolate: &'a Isolate,
        thread_was_in_wasm: bool,
    }

    impl<'a> SaveAndClearThreadInWasmFlag<'a> {
        pub fn new(isolate: &'a Isolate) -> Self {
            dcheck!(isolate as *const _ != std::ptr::null());
            let thread_was_in_wasm =
                is_trap_handler_enabled() && is_thread_in_wasm();
            if thread_was_in_wasm {
                clear_thread_in_wasm();
            }
            SaveAndClearThreadInWasmFlag {
                isolate,
                thread_was_in_wasm,
            }
        }
    }

    impl<'a> Drop for SaveAndClearThreadInWasmFlag<'a> {
        fn drop(&mut self) {
            if self.thread_was_in_wasm && !self.isolate.has_exception() {
                set_thread_in_wasm();
            }
        }
    }
}

pub mod internal {
    #[cfg(feature = "v8_enable_webassembly")]
    pub use super::wasm_utils::SaveAndClearThreadInWasmFlag;

    #[cfg(feature = "v8_enable_webassembly")]
    pub use super::wasm_utils::Isolate;
}