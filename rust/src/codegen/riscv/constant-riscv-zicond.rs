// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod riscv_zicond {
    use crate::codegen::riscv::base_constants_riscv::*;

    // RV32/RV64 Zicond Standard Extension
    pub const RO_CZERO_EQZ: Opcode =
        OP | (0b101 << FUNCT3_SHIFT) | (0b0000111 << FUNCT7_SHIFT);
    pub const RO_CZERO_NEZ: Opcode =
        OP | (0b111 << FUNCT3_SHIFT) | (0b0000111 << FUNCT7_SHIFT);
}