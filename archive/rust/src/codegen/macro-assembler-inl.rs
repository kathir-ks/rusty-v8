// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file is a direct translation of the C++ header file
// `src/codegen/macro-assembler-inl.h` from the V8 JavaScript engine.

// Since this header file primarily includes other headers and has conditional compilation,
// the Rust translation reflects this structure.

// `assembler-inl.h` is included within `assembler.h` so it is implicitly included.
pub mod assembler {
    pub use crate::codegen::assembler::*;
}
pub mod macro_assembler {
    pub use crate::codegen::macro_assembler::*;
}

#[cfg(target_arch = "aarch64")]
pub mod arm64 {
    pub use crate::codegen::arm64::macro_assembler_arm64_inl::*;
}