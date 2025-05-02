// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod riscv_zifencei {
    use crate::codegen::riscv::base_constants_riscv::*;

    /// Represents the RO_FENCE_I opcode.
    pub const RO_FENCE_I: u32 = MISC_MEM | (0b001 << kFunct3Shift);
}