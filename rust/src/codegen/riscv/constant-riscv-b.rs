// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod constant_riscv_b {
    use super::base_constants_riscv::*;

    // Assuming that V8_TARGET_ARCH_RISCV64, V8_TARGET_ARCH_RISCV32 are defined elsewhere
    // as compile-time constants or feature flags.

    // Zba
    #[cfg(target_arch = "riscv64")]
    pub const RO_ADDUW: Opcode =
        OP_32 | (0b000 << FUNCT3_SHIFT) | (0b0000100 << FUNCT7_SHIFT);
    #[cfg(target_arch = "riscv64")]
    pub const RO_SH1ADDUW: Opcode =
        OP_32 | (0b010 << FUNCT3_SHIFT) | (0b0010000 << FUNCT7_SHIFT);
    #[cfg(target_arch = "riscv64")]
    pub const RO_SH2ADDUW: Opcode =
        OP_32 | (0b100 << FUNCT3_SHIFT) | (0b0010000 << FUNCT7_SHIFT);
    #[cfg(target_arch = "riscv64")]
    pub const RO_SH3ADDUW: Opcode =
        OP_32 | (0b110 << FUNCT3_SHIFT) | (0b0010000 << FUNCT7_SHIFT);
    #[cfg(target_arch = "riscv64")]
    pub const RO_SLLIUW: Opcode =
        OP_IMM_32 | (0b001 << FUNCT3_SHIFT) | (0b000010 << FUNCT6_SHIFT);

    pub const RO_SH1ADD: Opcode =
        OP | (0b010 << FUNCT3_SHIFT) | (0b0010000 << FUNCT7_SHIFT);
    pub const RO_SH2ADD: Opcode =
        OP | (0b100 << FUNCT3_SHIFT) | (0b0010000 << FUNCT7_SHIFT);
    pub const RO_SH3ADD: Opcode =
        OP | (0b110 << FUNCT3_SHIFT) | (0b0010000 << FUNCT7_SHIFT);

    // Zbb
    pub const RO_ANDN: Opcode =
        OP | (0b111 << FUNCT3_SHIFT) | (0b0100000 << FUNCT7_SHIFT);
    pub const RO_ORN: Opcode =
        OP | (0b110 << FUNCT3_SHIFT) | (0b0100000 << FUNCT7_SHIFT);
    pub const RO_XNOR: Opcode =
        OP | (0b100 << FUNCT3_SHIFT) | (0b0100000 << FUNCT7_SHIFT);

    pub const OP_COUNT: Opcode =
        OP_IMM | (0b001 << FUNCT3_SHIFT) | (0b0110000 << FUNCT7_SHIFT);
    pub const RO_CLZ: Opcode = OP_COUNT | (0b00000 << SHAMT_SHIFT);
    pub const RO_CTZ: Opcode = OP_COUNT | (0b00001 << SHAMT_SHIFT);
    pub const RO_CPOP: Opcode = OP_COUNT | (0b00010 << SHAMT_SHIFT);

    #[cfg(target_arch = "riscv64")]
    pub const OP_COUNTW: Opcode =
        OP_IMM_32 | (0b001 << FUNCT3_SHIFT) | (0b0110000 << FUNCT7_SHIFT);
    #[cfg(target_arch = "riscv64")]
    pub const RO_CLZW: Opcode = OP_COUNTW | (0b00000 << SHAMT_SHIFT);
    #[cfg(target_arch = "riscv64")]
    pub const RO_CTZW: Opcode = OP_COUNTW | (0b00001 << SHAMT_SHIFT);
    #[cfg(target_arch = "riscv64")]
    pub const RO_CPOPW: Opcode = OP_COUNTW | (0b00010 << SHAMT_SHIFT);

    pub const RO_MAX: Opcode =
        OP | (0b110 << FUNCT3_SHIFT) | (0b0000101 << FUNCT7_SHIFT);
    pub const RO_MAXU: Opcode =
        OP | (0b111 << FUNCT3_SHIFT) | (0b0000101 << FUNCT7_SHIFT);

    pub const RO_MIN: Opcode =
        OP | (0b100 << FUNCT3_SHIFT) | (0b0000101 << FUNCT7_SHIFT);
    pub const RO_MINU: Opcode =
        OP | (0b101 << FUNCT3_SHIFT) | (0b0000101 << FUNCT7_SHIFT);

    pub const RO_SEXTB: Opcode = OP_IMM | (0b001 << FUNCT3_SHIFT)
        | (0b0110000 << FUNCT7_SHIFT)
        | (0b00100 << SHAMT_SHIFT);
    pub const RO_SEXTH: Opcode = OP_IMM | (0b001 << FUNCT3_SHIFT)
        | (0b0110000 << FUNCT7_SHIFT)
        | (0b00101 << SHAMT_SHIFT);

    #[cfg(target_arch = "riscv64")]
    pub const RO_ZEXTH: Opcode = OP_32 | (0b100 << FUNCT3_SHIFT)
        | (0b0000100 << FUNCT7_SHIFT)
        | (0b00000 << SHAMT_SHIFT);

    #[cfg(target_arch = "riscv32")]
    pub const RO_ZEXTH: Opcode = OP | (0b100 << FUNCT3_SHIFT)
        | (0b0000100 << FUNCT7_SHIFT)
        | (0b00000 << SHAMT_SHIFT);

    // Zbb: bitwise rotation
    pub const RO_ROL: Opcode =
        OP | (0b001 << FUNCT3_SHIFT) | (0b0110000 << FUNCT7_SHIFT);
    pub const RO_ROR: Opcode =
        OP | (0b101 << FUNCT3_SHIFT) | (0b0110000 << FUNCT7_SHIFT);
    pub const RO_ORCB: Opcode =
        OP_IMM | (0b101 << FUNCT3_SHIFT) | (0b001010000111 << IMM12_SHIFT);

    #[cfg(target_arch = "riscv64")]
    pub const RO_RORI: Opcode =
        OP_IMM | (0b101 << FUNCT3_SHIFT) | (0b011000 << FUNCT6_SHIFT);

    #[cfg(target_arch = "riscv32")]
    pub const RO_RORI: Opcode =
        OP_IMM | (0b101 << FUNCT3_SHIFT) | (0b0110000 << FUNCT7_SHIFT);

    #[cfg(target_arch = "riscv64")]
    pub const RO_ROLW: Opcode =
        OP_32 | (0b001 << FUNCT3_SHIFT) | (0b0110000 << FUNCT7_SHIFT);
    #[cfg(target_arch = "riscv64")]
    pub const RO_RORIW: Opcode =
        OP_IMM_32 | (0b101 << FUNCT3_SHIFT) | (0b0110000 << FUNCT7_SHIFT);
    #[cfg(target_arch = "riscv64")]
    pub const RO_RORW: Opcode =
        OP_32 | (0b101 << FUNCT3_SHIFT) | (0b0110000 << FUNCT7_SHIFT);

    pub const RO_REV8: Opcode =
        OP_IMM | (0b101 << FUNCT3_SHIFT) | (0b011010 << FUNCT6_SHIFT);

    #[cfg(target_arch = "riscv64")]
    pub const RO_REV8_IMM12: Opcode = 0b011010111000;

    #[cfg(target_arch = "riscv32")]
    pub const RO_REV8_IMM12: Opcode = 0b011010011000;

    // Zbs
    pub const RO_BCLR: Opcode =
        OP | (0b001 << FUNCT3_SHIFT) | (0b0100100 << FUNCT7_SHIFT);
    pub const RO_BCLRI: Opcode =
        OP_IMM | (0b001 << FUNCT3_SHIFT) | (0b010010 << FUNCT6_SHIFT);

    pub const RO_BEXT: Opcode =
        OP | (0b101 << FUNCT3_SHIFT) | (0b0100100 << FUNCT7_SHIFT);
    pub const RO_BEXTI: Opcode =
        OP_IMM | (0b101 << FUNCT3_SHIFT) | (0b010010 << FUNCT6_SHIFT);

    pub const RO_BINV: Opcode =
        OP | (0b001 << FUNCT3_SHIFT) | (0b0110100 << FUNCT7_SHIFT);
    pub const RO_BINVI: Opcode =
        OP_IMM | (0b001 << FUNCT3_SHIFT) | (0b011010 << FUNCT6_SHIFT);

    pub const RO_BSET: Opcode =
        OP | (0b001 << FUNCT3_SHIFT) | (0b0010100 << FUNCT7_SHIFT);
    pub const RO_BSETI: Opcode =
        OP_IMM | (0b001 << FUNCT3_SHIFT) | (0b0010100 << FUNCT7_SHIFT);
}