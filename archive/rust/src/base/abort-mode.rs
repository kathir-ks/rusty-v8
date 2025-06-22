// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//! This module describes the way aborts are handled in OS::Abort and the way
//! DCHECKs are working.

/// Defines the different modes for handling aborts and assertions.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum AbortMode {
    /// Used when controlled crashes are harmless, such as for fuzzing or sandboxing.
    ///
    /// - `DCHECKs` are turned into no-ops, allowing execution to continue.
    /// - `CHECKs`, `FATAL`, etc., are turned into regular exits.
    /// - The exit code signals success (0) or failure (non-zero).
    kExitWithSuccessAndIgnoreDcheckFailures,
    /// Like kExitWithSuccessAndIgnoreDcheckFailures, but exits with failure
    kExitWithFailureAndIgnoreDcheckFailures,

    /// `DCHECKs`, `CHECKs`, etc., use IMMEDIATE_CRASH() to signal abnormal program termination.
    kImmediateCrash,

    /// `CHECKs`, `DCHECKs`, etc., use `abort()` to signal abnormal program termination.
    kDefault,
}

// This is a global mutable variable.  In C++, it's often better to use a
// compile-time constant, thread-local variable, or a singleton. In Rust,
// a static mutable variable should be used with caution.  Here, we use a
// lazy_static to initialize it.
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    /// Global abort mode.
    pub static ref G_ABORT_MODE: Mutex<AbortMode> = Mutex::new(AbortMode::kDefault);
}

/// Returns true if controlled crashes are harmless in the current abort mode.
#[inline]
pub fn controlled_crashes_are_harmless() -> bool {
    let abort_mode = *G_ABORT_MODE.lock().unwrap();
    abort_mode == AbortMode::kExitWithSuccessAndIgnoreDcheckFailures ||
        abort_mode == AbortMode::kExitWithFailureAndIgnoreDcheckFailures
}

/// Returns true if Dcheck failures are ignored in the current abort mode.
#[inline]
pub fn dcheck_failures_are_ignored() -> bool {
    let abort_mode = *G_ABORT_MODE.lock().unwrap();
    abort_mode == AbortMode::kExitWithSuccessAndIgnoreDcheckFailures ||
        abort_mode == AbortMode::kExitWithFailureAndIgnoreDcheckFailures
}