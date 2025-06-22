// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//! This module provides platform implementations for the d8 tool.
//!
//! It includes implementations for predictable and delayed task platforms,
//! useful for testing and debugging.

/// A type representing a v8::Platform implementation.
pub trait Platform {}

/// Placeholder type for Isolate.  This would ideally be defined within the v8 module
/// but that requires significant refactoring of the larger codebase.
pub struct Isolate {}

/// Makes a predictable `Platform` implementation.
///
/// Worker threads are disabled, idle tasks are disallowed, and the time reported
/// by monotonically increasing time is deterministic.
pub fn make_predictable_platform<T: Platform + 'static>(
    platform: Box<T>,
) -> Box<dyn Platform> {
    // This is a placeholder; in a full implementation, this would wrap the
    // provided platform with logic to make it predictable.
    platform
}

/// Makes a `Platform` implementation which randomly delays tasks for stress-testing.
///
/// If `random_seed` is 0, a random seed is chosen.
pub fn make_delayed_tasks_platform<T: Platform + 'static>(
    platform: Box<T>,
    random_seed: i64,
) -> Box<dyn Platform> {
    // This is a placeholder; in a full implementation, this would wrap the
    // provided platform with logic to delay tasks.
    platform
}

/// A constant representing the process global predictable platform worker task queue.
///
/// Currently, `nullptr` is a valid value for the isolate in C++.  In Rust, we
/// use `Option<&Isolate>` to represent this.
pub const PROCESS_GLOBAL_PREDICTABLE_PLATFORM_WORKER_TASK_QUEUE: Option<&Isolate> = None;