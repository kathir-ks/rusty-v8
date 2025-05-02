// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod riscv_m {
    use super::base_constants_riscv::*;

    /// RV32M Standard Extension
    pub const RO_MUL: Opcode = OP | (0b000 << FUNCT3_SHIFT) | (0b0000001 << FUNCT7_SHIFT);
    pub const RO_MULH: Opcode = OP | (0b001 << FUNCT3_SHIFT) | (0b0000001 << FUNCT7_SHIFT);
    pub const RO_MULHSU: Opcode = OP | (0b010 << FUNCT3_SHIFT) | (0b0000001 << FUNCT7_SHIFT);
    pub const RO_MULHU: Opcode = OP | (0b011 << FUNCT3_SHIFT) | (0b0000001 << FUNCT7_SHIFT);
    pub const RO_DIV: Opcode = OP | (0b100 << FUNCT3_SHIFT) | (0b0000001 << FUNCT7_SHIFT);
    pub const RO_DIVU: Opcode = OP | (0b101 << FUNCT3_SHIFT) | (0b0000001 << FUNCT7_SHIFT);
    pub const RO_REM: Opcode = OP | (0b110 << FUNCT3_SHIFT) | (0b0000001 << FUNCT7_SHIFT);
    pub const RO_REMU: Opcode = OP | (0b111 << FUNCT3_SHIFT) | (0b0000001 << FUNCT7_SHIFT);

    #[cfg(target_arch = "riscv64")]
    pub mod riscv64 {
        use super::*;

        /// RV64M Standard Extension (in addition to RV32M)
        pub const RO_MULW: Opcode = OP_32 | (0b000 << FUNCT3_SHIFT) | (0b0000001 << FUNCT7_SHIFT);
        pub const RO_DIVW: Opcode = OP_32 | (0b100 << FUNCT3_SHIFT) | (0b0000001 << FUNCT7_SHIFT);
        pub const RO_DIVUW: Opcode = OP_32 | (0b101 << FUNCT3_SHIFT) | (0b0000001 << FUNCT7_SHIFT);
        pub const RO_REMW: Opcode = OP_32 | (0b110 << FUNCT3_SHIFT) | (0b0000001 << FUNCT7_SHIFT);
        pub const RO_REMUW: Opcode = OP_32 | (0b111 << FUNCT3_SHIFT) | (0b0000001 << FUNCT7_SHIFT);
    }
}

pub mod base_constants_riscv {
    pub type Opcode = u32;

    pub const OP: Opcode = 0; // Replace with the actual value
    pub const OP_32: Opcode = 1; // Replace with the actual value

    pub const FUNCT3_SHIFT: u32 = 0; // Replace with actual value
    pub const FUNCT7_SHIFT: u32 = 5; // Replace with actual value
}