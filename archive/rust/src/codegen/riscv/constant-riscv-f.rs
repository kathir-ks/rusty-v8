// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod base_constants_riscv;

use base_constants_riscv::*;

/// RISC-V floating-point instruction constants
pub mod riscv_f {
    use super::*;

    // RV32F Standard Extension
    pub const RO_FLW: Opcode = LOAD_FP | (0b010 << kFunct3Shift);
    pub const RO_FSW: Opcode = STORE_FP | (0b010 << kFunct3Shift);
    pub const RO_FMADD_S: Opcode = MADD | (0b00 << kFunct2Shift);
    pub const RO_FMSUB_S: Opcode = MSUB | (0b00 << kFunct2Shift);
    pub const RO_FNMSUB_S: Opcode = NMSUB | (0b00 << kFunct2Shift);
    pub const RO_FNMADD_S: Opcode = NMADD | (0b00 << kFunct2Shift);
    pub const RO_FADD_S: Opcode = OP_FP | (0b0000000 << kFunct7Shift);
    pub const RO_FSUB_S: Opcode = OP_FP | (0b0000100 << kFunct7Shift);
    pub const RO_FMUL_S: Opcode = OP_FP | (0b0001000 << kFunct7Shift);
    pub const RO_FDIV_S: Opcode = OP_FP | (0b0001100 << kFunct7Shift);
    pub const RO_FSQRT_S: Opcode =
        OP_FP | (0b0101100 << kFunct7Shift) | (0b00000 << kRs2Shift);
    pub const RO_FSGNJ_S: Opcode =
        OP_FP | (0b000 << kFunct3Shift) | (0b0010000 << kFunct7Shift);
    pub const RO_FSGNJN_S: Opcode =
        OP_FP | (0b001 << kFunct3Shift) | (0b0010000 << kFunct7Shift);
    pub const RO_FSQNJX_S: Opcode =
        OP_FP | (0b010 << kFunct3Shift) | (0b0010000 << kFunct7Shift);
    pub const RO_FMIN_S: Opcode =
        OP_FP | (0b000 << kFunct3Shift) | (0b0010100 << kFunct7Shift);
    pub const RO_FMAX_S: Opcode =
        OP_FP | (0b001 << kFunct3Shift) | (0b0010100 << kFunct7Shift);
    pub const RO_FCVT_W_S: Opcode =
        OP_FP | (0b1100000 << kFunct7Shift) | (0b00000 << kRs2Shift);
    pub const RO_FCVT_WU_S: Opcode =
        OP_FP | (0b1100000 << kFunct7Shift) | (0b00001 << kRs2Shift);
    pub const RO_FMV: Opcode = OP_FP | (0b1110000 << kFunct7Shift) |
                              (0b000 << kFunct3Shift) | (0b00000 << kRs2Shift);
    pub const RO_FEQ_S: Opcode =
        OP_FP | (0b010 << kFunct3Shift) | (0b1010000 << kFunct7Shift);
    pub const RO_FLT_S: Opcode =
        OP_FP | (0b001 << kFunct3Shift) | (0b1010000 << kFunct7Shift);
    pub const RO_FLE_S: Opcode =
        OP_FP | (0b000 << kFunct3Shift) | (0b1010000 << kFunct7Shift);
    pub const RO_FCLASS_S: Opcode =
        OP_FP | (0b001 << kFunct3Shift) | (0b1110000 << kFunct7Shift);
    pub const RO_FCVT_S_W: Opcode =
        OP_FP | (0b1101000 << kFunct7Shift) | (0b00000 << kRs2Shift);
    pub const RO_FCVT_S_WU: Opcode =
        OP_FP | (0b1101000 << kFunct7Shift) | (0b00001 << kRs2Shift);
    pub const RO_FMV_W_X: Opcode =
        OP_FP | (0b000 << kFunct3Shift) | (0b1111000 << kFunct7Shift);

    //Conditional compilation based on target architecture
    #[cfg(target_arch = "riscv64")]
    pub mod riscv64f {
        use super::*;

        // RV64F Standard Extension (in addition to RV32F)
        pub const RO_FCVT_L_S: Opcode =
            OP_FP | (0b1100000 << kFunct7Shift) | (0b00010 << kRs2Shift);
        pub const RO_FCVT_LU_S: Opcode =
            OP_FP | (0b1100000 << kFunct7Shift) | (0b00011 << kRs2Shift);
        pub const RO_FCVT_S_L: Opcode =
            OP_FP | (0b1101000 << kFunct7Shift) | (0b00010 << kRs2Shift);
        pub const RO_FCVT_S_LU: Opcode =
            OP_FP | (0b1101000 << kFunct7Shift) | (0b00011 << kRs2Shift);
    }
}