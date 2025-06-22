// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file is a Rust translation of the C++ header file
// `src/wasm/memory-tracing.h` from the V8 JavaScript engine codebase.

// The following conditional compilation directive from the original C++ file
// #if !V8_ENABLE_WEBASSEMBLY
// #error This header should only be included if WebAssembly is enabled.
// #endif  // !V8_ENABLE_WEBASSEMBLY
// is not directly translatable in Rust. Instead, we'll assume that this file is
// only compiled when WebAssembly support is enabled.  In a real project, you
// might use a feature flag or similar mechanism to achieve the same effect.

use std::mem;

/// Represents the different machine types for memory representation.
/// This enum mimics the `MachineRepresentation` enum in the original C++ code.
/// The specific variants here are just placeholders to allow the rest of the
/// code to compile, real values are architecture-dependent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRepresentation {
    Word8,
    Word16,
    Word32,
    Word64,
    Float32,
    Float64,
    Simd128,
    Pointer,
}

/// Information about memory tracing.
///
/// This struct is created in generated code, hence use low-level types.
#[derive(Debug, Copy, Clone)]
pub struct MemoryTracingInfo {
    pub offset: usize,
    pub is_store: u8, // 0 or 1
    pub mem_rep: u8,
}

impl MemoryTracingInfo {
    /// Creates a new `MemoryTracingInfo` instance.
    pub fn new(offset: usize, is_store: bool, rep: MachineRepresentation) -> Self {
        Self {
            offset,
            is_store: if is_store { 1 } else { 0 },
            mem_rep: rep as u8,
        }
    }
}

const _ASSERT_MACHINE_REPRESENTATION_USES_U8: () = {
    assert!(mem::size_of::<MachineRepresentation>() <= 1, "MachineRepresentation uses uint8_t");
};

pub mod wasm_tier {
    //This module is just a placeholder for `src/wasm/wasm-tier.h`
    //The actual implementation would have more complex data structures related to wasm compilation tiers
}