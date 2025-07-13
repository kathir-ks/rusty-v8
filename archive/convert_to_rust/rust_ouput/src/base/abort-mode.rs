// Converted from V8 C++ source files:
// Header: abort-mode.h
// Implementation: abort-mode.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub enum AbortMode {
        kExitWithSuccessAndIgnoreDcheckFailures,
        kExitWithFailureAndIgnoreDcheckFailures,
        kImmediateCrash,
        kDefault,
    }

    pub static mut g_abort_mode: AbortMode = AbortMode::kDefault;

    pub fn controlled_crashes_are_harmless() -> bool {
        unsafe {
            g_abort_mode == AbortMode::kExitWithSuccessAndIgnoreDcheckFailures
                || g_abort_mode == AbortMode::kExitWithFailureAndIgnoreDcheckFailures
        }
    }

    pub fn dcheck_failures_are_ignored() -> bool {
        unsafe {
            g_abort_mode == AbortMode::kExitWithSuccessAndIgnoreDcheckFailures
                || g_abort_mode == AbortMode::kExitWithFailureAndIgnoreDcheckFailures
        }
    }
}
