// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod riscv_i {
    use crate::codegen::riscv::base_constants_riscv::*;

    // Note use RO (RiscV Opcode) prefix
    // RV32I Base Instruction Set
    pub const RO_LUI: Opcode = LUI;
    pub const RO_AUIPC: Opcode = AUIPC;
    pub const RO_JAL: Opcode = JAL;
    pub const RO_JALR: Opcode = JALR | (0b000 << kFunct3Shift);
    pub const RO_BEQ: Opcode = BRANCH | (0b000 << kFunct3Shift);
    pub const RO_BNE: Opcode = BRANCH | (0b001 << kFunct3Shift);
    pub const RO_BLT: Opcode = BRANCH | (0b100 << kFunct3Shift);
    pub const RO_BGE: Opcode = BRANCH | (0b101 << kFunct3Shift);
    pub const RO_BLTU: Opcode = BRANCH | (0b110 << kFunct3Shift);
    pub const RO_BGEU: Opcode = BRANCH | (0b111 << kFunct3Shift);
    pub const RO_LB: Opcode = LOAD | (0b000 << kFunct3Shift);
    pub const RO_LH: Opcode = LOAD | (0b001 << kFunct3Shift);
    pub const RO_LW: Opcode = LOAD | (0b010 << kFunct3Shift);
    pub const RO_LBU: Opcode = LOAD | (0b100 << kFunct3Shift);
    pub const RO_LHU: Opcode = LOAD | (0b101 << kFunct3Shift);
    pub const RO_SB: Opcode = STORE | (0b000 << kFunct3Shift);
    pub const RO_SH: Opcode = STORE | (0b001 << kFunct3Shift);
    pub const RO_SW: Opcode = STORE | (0b010 << kFunct3Shift);

    pub const RO_ADDI: Opcode = OP_IMM | (0b000 << kFunct3Shift);
    pub const RO_SLTI: Opcode = OP_IMM | (0b010 << kFunct3Shift);
    pub const RO_SLTIU: Opcode = OP_IMM | (0b011 << kFunct3Shift);
    pub const RO_XORI: Opcode = OP_IMM | (0b100 << kFunct3Shift);
    pub const RO_ORI: Opcode = OP_IMM | (0b110 << kFunct3Shift);
    pub const RO_ANDI: Opcode = OP_IMM | (0b111 << kFunct3Shift);

    pub const OP_SHL: Opcode = OP_IMM | (0b001 << kFunct3Shift);
    pub const RO_SLLI: Opcode = OP_SHL | (0b000000 << kFunct6Shift);

    pub const OP_SHR: Opcode = OP_IMM | (0b101 << kFunct3Shift);
    pub const RO_SRLI: Opcode = OP_SHR | (0b000000 << kFunct6Shift);
    pub const RO_SRAI: Opcode = OP_SHR | (0b010000 << kFunct6Shift);

    pub const RO_ADD: Opcode =
        OP | (0b000 << kFunct3Shift) | (0b0000000 << kFunct7Shift);
    pub const RO_SUB: Opcode =
        OP | (0b000 << kFunct3Shift) | (0b0100000 << kFunct7Shift);
    pub const RO_SLL: Opcode =
        OP | (0b001 << kFunct3Shift) | (0b0000000 << kFunct7Shift);
    pub const RO_SLT: Opcode =
        OP | (0b010 << kFunct3Shift) | (0b0000000 << kFunct7Shift);
    pub const RO_SLTU: Opcode =
        OP | (0b011 << kFunct3Shift) | (0b0000000 << kFunct7Shift);
    pub const RO_XOR: Opcode =
        OP | (0b100 << kFunct3Shift) | (0b0000000 << kFunct7Shift);
    pub const RO_SRL: Opcode =
        OP | (0b101 << kFunct3Shift) | (0b0000000 << kFunct7Shift);
    pub const RO_SRA: Opcode =
        OP | (0b101 << kFunct3Shift) | (0b0100000 << kFunct7Shift);
    pub const RO_OR: Opcode =
        OP | (0b110 << kFunct3Shift) | (0b0000000 << kFunct7Shift);
    pub const RO_AND: Opcode =
        OP | (0b111 << kFunct3Shift) | (0b0000000 << kFunct7Shift);
    pub const RO_FENCE: Opcode = MISC_MEM | (0b000 << kFunct3Shift);
    pub const RO_ECALL: Opcode = SYSTEM | (0b000 << kFunct3Shift);
    // RO_EBREAK = SYSTEM | (0b000 << kFunct3Shift), // Same as ECALL, use imm12

    // This conditional compilation is based on the V8 build system.
    // The Rust equivalent would likely be a cfg attribute.
    #[cfg(target_arch = "riscv64")]
    pub mod riscv64 {
        use crate::codegen::riscv::base_constants_riscv::*;

        // RV64I Base Instruction Set (in addition to RV32I)
        pub const RO_LWU: Opcode = LOAD | (0b110 << kFunct3Shift);
        pub const RO_LD: Opcode = LOAD | (0b011 << kFunct3Shift);
        pub const RO_SD: Opcode = STORE | (0b011 << kFunct3Shift);
        pub const RO_ADDIW: Opcode = OP_IMM_32 | (0b000 << kFunct3Shift);

        pub const OP_SHLW: Opcode = OP_IMM_32 | (0b001 << kFunct3Shift);
        pub const RO_SLLIW: Opcode = OP_SHLW | (0b0000000 << kFunct7Shift);

        pub const OP_SHRW: Opcode = OP_IMM_32 | (0b101 << kFunct3Shift);
        pub const RO_SRLIW: Opcode = OP_SHRW | (0b0000000 << kFunct7Shift);
        pub const RO_SRAIW: Opcode = OP_SHRW | (0b0100000 << kFunct7Shift);

        pub const RO_ADDW: Opcode =
            OP_32 | (0b000 << kFunct3Shift) | (0b0000000 << kFunct7Shift);
        pub const RO_SUBW: Opcode =
            OP_32 | (0b000 << kFunct3Shift) | (0b0100000 << kFunct7Shift);
        pub const RO_SLLW: Opcode =
            OP_32 | (0b001 << kFunct3Shift) | (0b0000000 << kFunct7Shift);
        pub const RO_SRLW: Opcode =
            OP_32 | (0b101 << kFunct3Shift) | (0b0000000 << kFunct7Shift);
        pub const RO_SRAW: Opcode =
            OP_32 | (0b101 << kFunct3Shift) | (0b0100000 << kFunct7Shift);
    }
}