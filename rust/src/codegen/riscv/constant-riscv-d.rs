// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod constant_riscv_d {
    use crate::codegen::riscv::base_constants_riscv::*;

    // RV32D Standard Extension
    pub const RO_FLD: Opcode = LOAD_FP | (0b011 << kFunct3Shift);
    pub const RO_FSD: Opcode = STORE_FP | (0b011 << kFunct3Shift);
    pub const RO_FMADD_D: Opcode = MADD | (0b01 << kFunct2Shift);
    pub const RO_FMSUB_D: Opcode = MSUB | (0b01 << kFunct2Shift);
    pub const RO_FNMSUB_D: Opcode = NMSUB | (0b01 << kFunct2Shift);
    pub const RO_FNMADD_D: Opcode = NMADD | (0b01 << kFunct2Shift);
    pub const RO_FADD_D: Opcode = OP_FP | (0b0000001 << kFunct7Shift);
    pub const RO_FSUB_D: Opcode = OP_FP | (0b0000101 << kFunct7Shift);
    pub const RO_FMUL_D: Opcode = OP_FP | (0b0001001 << kFunct7Shift);
    pub const RO_FDIV_D: Opcode = OP_FP | (0b0001101 << kFunct7Shift);
    pub const RO_FSQRT_D: Opcode =
        OP_FP | (0b0101101 << kFunct7Shift) | (0b00000 << kRs2Shift);
    pub const RO_FSGNJ_D: Opcode =
        OP_FP | (0b000 << kFunct3Shift) | (0b0010001 << kFunct7Shift);
    pub const RO_FSGNJN_D: Opcode =
        OP_FP | (0b001 << kFunct3Shift) | (0b0010001 << kFunct7Shift);
    pub const RO_FSQNJX_D: Opcode =
        OP_FP | (0b010 << kFunct3Shift) | (0b0010001 << kFunct7Shift);
    pub const RO_FMIN_D: Opcode =
        OP_FP | (0b000 << kFunct3Shift) | (0b0010101 << kFunct7Shift);
    pub const RO_FMAX_D: Opcode =
        OP_FP | (0b001 << kFunct3Shift) | (0b0010101 << kFunct7Shift);
    pub const RO_FCVT_S_D: Opcode =
        OP_FP | (0b0100000 << kFunct7Shift) | (0b00001 << kRs2Shift);
    pub const RO_FCVT_D_S: Opcode =
        OP_FP | (0b0100001 << kFunct7Shift) | (0b00000 << kRs2Shift);
    pub const RO_FEQ_D: Opcode =
        OP_FP | (0b010 << kFunct3Shift) | (0b1010001 << kFunct7Shift);
    pub const RO_FLT_D: Opcode =
        OP_FP | (0b001 << kFunct3Shift) | (0b1010001 << kFunct7Shift);
    pub const RO_FLE_D: Opcode =
        OP_FP | (0b000 << kFunct3Shift) | (0b1010001 << kFunct7Shift);
    pub const RO_FCLASS_D: Opcode = OP_FP | (0b001 << kFunct3Shift) |
                                   (0b1110001 << kFunct7Shift) |
                                   (0b00000 << kRs2Shift);
    pub const RO_FCVT_W_D: Opcode =
        OP_FP | (0b1100001 << kFunct7Shift) | (0b00000 << kRs2Shift);
    pub const RO_FCVT_WU_D: Opcode =
        OP_FP | (0b1100001 << kFunct7Shift) | (0b00001 << kRs2Shift);
    pub const RO_FCVT_D_W: Opcode =
        OP_FP | (0b1101001 << kFunct7Shift) | (0b00000 << kRs2Shift);
    pub const RO_FCVT_D_WU: Opcode =
        OP_FP | (0b1101001 << kFunct7Shift) | (0b00001 << kRs2Shift);

    #[cfg(target_arch = "riscv64")]
    pub mod riscv64 {
        use crate::codegen::riscv::base_constants_riscv::*;
        // RV64D Standard Extension (in addition to RV32D)
        pub const RO_FCVT_L_D: Opcode =
            OP_FP | (0b1100001 << kFunct7Shift) | (0b00010 << kRs2Shift);
        pub const RO_FCVT_LU_D: Opcode =
            OP_FP | (0b1100001 << kFunct7Shift) | (0b00011 << kRs2Shift);
        pub const RO_FMV_X_D: Opcode = OP_FP | (0b000 << kFunct3Shift) |
                                      (0b1110001 << kFunct7Shift) |
                                      (0b00000 << kRs2Shift);
        pub const RO_FCVT_D_L: Opcode =
            OP_FP | (0b1101001 << kFunct7Shift) | (0b00010 << kRs2Shift);
        pub const RO_FCVT_D_LU: Opcode =
            OP_FP | (0b1101001 << kFunct7Shift) | (0b00011 << kRs2Shift);
        pub const RO_FMV_D_X: Opcode = OP_FP | (0b000 << kFunct3Shift) |
                                      (0b1111001 << kFunct7Shift) |
                                      (0b00000 << kRs2Shift);
    }

}