// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This header should only be included if WebAssembly is enabled.
// In V8, this check is done using a preprocessor macro.
// In Rust, we can use a cfg attribute:
// #[cfg(feature = "webassembly")]

// In V8, V8_ENABLE_WEBASSEMBLY is used. Here it's assumed webassembly is enabled if the module is compiled.

/// All the tiers of Wasm execution.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ExecutionTier {
    None,
    #[cfg(feature = "drumbrake")]
    Interpreter,
    Liftoff,
    Turbofan,
}

impl ExecutionTier {
    pub fn to_string(self) -> &'static str {
        match self {
            ExecutionTier::Turbofan => "turbofan",
            ExecutionTier::Liftoff => "liftoff",
            #[cfg(feature = "drumbrake")]
            ExecutionTier::Interpreter => "interpreter",
            ExecutionTier::None => "none",
        }
    }
}

/// {ForDebugging} is used for default tiered-down code, {kWithBreakpoints} if
/// the code also contains breakpoints, and {kForStepping} for code that is
/// flooded with breakpoints.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ForDebugging {
    NotForDebugging = 0,
    ForDebugging,
    WithBreakpoints,
    ForStepping,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DebugState {
    NotDebugging = false as isize != 0,
    Debugging = true as isize != 0,
}