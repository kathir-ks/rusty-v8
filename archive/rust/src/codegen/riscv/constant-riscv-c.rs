// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod constant_riscv_c {
    use crate::codegen::riscv::base_constants_riscv::*;

    pub const RO_C_ADDI4SPN: Opcode = C0 | (0b000 << kRvcFunct3Shift);
    pub const RO_C_ADDI16SP: Opcode = C1 | (0b011 << kRvcFunct3Shift);
    pub const RO_C_LW: Opcode = C0 | (0b010 << kRvcFunct3Shift);
    pub const RO_C_SW: Opcode = C0 | (0b110 << kRvcFunct3Shift);
    pub const RO_C_NOP_ADDI: Opcode = C1 | (0b000 << kRvcFunct3Shift);
    pub const RO_C_LI: Opcode = C1 | (0b010 << kRvcFunct3Shift);
    pub const RO_C_SUB: Opcode =
        C1 | (0b100011 << kRvcFunct6Shift) | (FUNCT2_0 << kRvcFunct2Shift);
    pub const RO_C_XOR: Opcode =
        C1 | (0b100011 << kRvcFunct6Shift) | (FUNCT2_1 << kRvcFunct2Shift);
    pub const RO_C_OR: Opcode =
        C1 | (0b100011 << kRvcFunct6Shift) | (FUNCT2_2 << kRvcFunct2Shift);
    pub const RO_C_AND: Opcode =
        C1 | (0b100011 << kRvcFunct6Shift) | (FUNCT2_3 << kRvcFunct2Shift);
    pub const RO_C_LUI_ADD: Opcode = C1 | (0b011 << kRvcFunct3Shift);
    pub const RO_C_MISC_ALU: Opcode = C1 | (0b100 << kRvcFunct3Shift);
    pub const RO_C_J: Opcode = C1 | (0b101 << kRvcFunct3Shift);
    pub const RO_C_BEQZ: Opcode = C1 | (0b110 << kRvcFunct3Shift);
    pub const RO_C_BNEZ: Opcode = C1 | (0b111 << kRvcFunct3Shift);
    pub const RO_C_SLLI: Opcode = C2 | (0b000 << kRvcFunct3Shift);
    pub const RO_C_LWSP: Opcode = C2 | (0b010 << kRvcFunct3Shift);
    pub const RO_C_JR_MV_ADD: Opcode = C2 | (0b100 << kRvcFunct3Shift);
    pub const RO_C_JR: Opcode = C2 | (0b1000 << kRvcFunct4Shift);
    pub const RO_C_MV: Opcode = C2 | (0b1000 << kRvcFunct4Shift);
    pub const RO_C_EBREAK: Opcode = C2 | (0b1001 << kRvcFunct4Shift);
    pub const RO_C_JALR: Opcode = C2 | (0b1001 << kRvcFunct4Shift);
    pub const RO_C_ADD: Opcode = C2 | (0b1001 << kRvcFunct4Shift);
    pub const RO_C_SWSP: Opcode = C2 | (0b110 << kRvcFunct3Shift);

    pub const RO_C_FSD: Opcode = C0 | (0b101 << kRvcFunct3Shift);
    pub const RO_C_FLD: Opcode = C0 | (0b001 << kRvcFunct3Shift);
    pub const RO_C_FLDSP: Opcode = C2 | (0b001 << kRvcFunct3Shift);
    pub const RO_C_FSDSP: Opcode = C2 | (0b101 << kRvcFunct3Shift);

    #[cfg(target_arch = "riscv64")]
    pub mod riscv64 {
        use crate::codegen::riscv::base_constants_riscv::*;

        pub const RO_C_LD: Opcode = C0 | (0b011 << kRvcFunct3Shift);
        pub const RO_C_SD: Opcode = C0 | (0b111 << kRvcFunct3Shift);
        pub const RO_C_LDSP: Opcode = C2 | (0b011 << kRvcFunct3Shift);
        pub const RO_C_SDSP: Opcode = C2 | (0b111 << kRvcFunct3Shift);
        pub const RO_C_ADDIW: Opcode = C1 | (0b001 << kRvcFunct3Shift);
        pub const RO_C_SUBW: Opcode =
            C1 | (0b100111 << kRvcFunct6Shift) | (FUNCT2_0 << kRvcFunct2Shift);
        pub const RO_C_ADDW: Opcode =
            C1 | (0b100111 << kRvcFunct6Shift) | (FUNCT2_1 << kRvcFunct2Shift);
    }
    #[cfg(target_arch = "riscv32")]
    pub mod riscv32 {
        use crate::codegen::riscv::base_constants_riscv::*;

        pub const RO_C_FLWSP: Opcode = C2 | (0b011 << kRvcFunct3Shift);
        pub const RO_C_FSWSP: Opcode = C2 | (0b111 << kRvcFunct3Shift);
        pub const RO_C_FLW: Opcode = C0 | (0b011 << kRvcFunct3Shift);
        pub const RO_C_FSW: Opcode = C0 | (0b111 << kRvcFunct3Shift);
    }
}