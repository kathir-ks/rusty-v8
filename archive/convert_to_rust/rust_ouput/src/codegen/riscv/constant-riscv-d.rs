// Converted from V8 C++ source files:
// Header: constant-riscv-d.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod constant_riscv_d {
pub use super::base_constants_riscv::*;

// RV32D Standard Extension
pub const RO_FLD: Opcode = Opcode::LOAD_FP; //| (0b011 << kFunct3Shift); // Assuming LOAD_FP covers the base opcode
pub const RO_FSD: Opcode = Opcode::STORE_FP; //| (0b011 << kFunct3Shift); // Assuming STORE_FP covers the base opcode
pub const RO_FMADD_D: Opcode = Opcode::MADD; //| (0b01 << kFunct2Shift);  // Assuming MADD covers the base opcode
pub const RO_FMSUB_D: Opcode = Opcode::MSUB; //| (0b01 << kFunct2Shift);  // Assuming MSUB covers the base opcode
pub const RO_FNMSUB_D: Opcode = Opcode::NMSUB; //| (0b01 << kFunct2Shift); // Assuming NMSUB covers the base opcode
pub const RO_FNMADD_D: Opcode = Opcode::NMADD; //| (0b01 << kFunct2Shift); // Assuming NMADD covers the base opcode
pub const RO_FADD_D: Opcode = Opcode::OP_FP; //| (0b0000001 << kFunct7Shift); // Assuming OP_FP covers the base opcode
pub const RO_FSUB_D: Opcode = Opcode::OP_FP; //| (0b0000101 << kFunct7Shift); // Assuming OP_FP covers the base opcode
pub const RO_FMUL_D: Opcode = Opcode::OP_FP; //| (0b0001001 << kFunct7Shift); // Assuming OP_FP covers the base opcode
pub const RO_FDIV_D: Opcode = Opcode::OP_FP; //| (0b0001101 << kFunct7Shift); // Assuming OP_FP covers the base opcode
pub const RO_FSQRT_D: Opcode = Opcode::OP_FP; //| (0b0101101 << kFunct7Shift) | (0b00000 << kRs2Shift); // Assuming OP_FP covers the base opcode
pub const RO_FSGNJ_D: Opcode = Opcode::OP_FP; //| (0b000 << kFunct3Shift) | (0b0010001 << kFunct7Shift); // Assuming OP_FP covers the base opcode
pub const RO_FSGNJN_D: Opcode = Opcode::OP_FP; //| (0b001 << kFunct3Shift) | (0b0010001 << kFunct7Shift); // Assuming OP_FP covers the base opcode
pub const RO_FSQNJX_D: Opcode = Opcode::OP_FP; //| (0b010 << kFunct3Shift) | (0b0010001 << kFunct7Shift); // Assuming OP_FP covers the base opcode
pub const RO_FMIN_D: Opcode = Opcode::OP_FP; //| (0b000 << kFunct3Shift) | (0b0010101 << kFunct7Shift); // Assuming OP_FP covers the base opcode
pub const RO_FMAX_D: Opcode = Opcode::OP_FP; //| (0b001 << kFunct3Shift) | (0b0010101 << kFunct7Shift); // Assuming OP_FP covers the base opcode
pub const RO_FCVT_S_D: Opcode = Opcode::OP_FP; //| (0b0100000 << kFunct7Shift) | (0b00001 << kRs2Shift); // Assuming OP_FP covers the base opcode
pub const RO_FCVT_D_S: Opcode = Opcode::OP_FP; //| (0b0100001 << kFunct7Shift) | (0b00000 << kRs2Shift); // Assuming OP_FP covers the base opcode
pub const RO_FEQ_D: Opcode = Opcode::OP_FP; //| (0b010 << kFunct3Shift) | (0b1010001 << kFunct7Shift); // Assuming OP_FP covers the base opcode
pub const RO_FLT_D: Opcode = Opcode::OP_FP; //| (0b001 << kFunct3Shift) | (0b1010001 << kFunct7Shift); // Assuming OP_FP covers the base opcode
pub const RO_FLE_D: Opcode = Opcode::OP_FP; //| (0b000 << kFunct3Shift) | (0b1010001 << kFunct7Shift); // Assuming OP_FP covers the base opcode
pub const RO_FCLASS_D: Opcode = Opcode::OP_FP; //| (0b001 << kFunct3Shift) | (0b1110001 << kFunct7Shift) | (0b00000 << kRs2Shift); // Assuming OP_FP covers the base opcode
pub const RO_FCVT_W_D: Opcode = Opcode::OP_FP; //| (0b1100001 << kFunct7Shift) | (0b00000 << kRs2Shift); // Assuming OP_FP covers the base opcode
pub const RO_FCVT_WU_D: Opcode = Opcode::OP_FP; //| (0b1100001 << kFunct7Shift) | (0b00001 << kRs2Shift); // Assuming OP_FP covers the base opcode
pub const RO_FCVT_D_W: Opcode = Opcode::OP_FP; //| (0b1101001 << kFunct7Shift) | (0b00000 << kRs2Shift); // Assuming OP_FP covers the base opcode
pub const RO_FCVT_D_WU: Opcode = Opcode::OP_FP; //| (0b1101001 << kFunct7Shift) | (0b00001 << kRs2Shift); // Assuming OP_FP covers the base opcode

#[cfg(target_arch = "riscv64")]
pub mod riscv64 {
    use super::*;
    // RV64D Standard Extension (in addition to RV32D)
    pub const RO_FCVT_L_D: Opcode = Opcode::OP_FP; //| (0b1100001 << kFunct7Shift) | (0b00010 << kRs2Shift); // Assuming OP_FP covers the base opcode
    pub const RO_FCVT_LU_D: Opcode = Opcode::OP_FP; //| (0b1100001 << kFunct7Shift) | (0b00011 << kRs2Shift); // Assuming OP_FP covers the base opcode
    pub const RO_FMV_X_D: Opcode = Opcode::OP_FP; //| (0b000 << kFunct3Shift) | (0b1110001 << kFunct7Shift) | (0b00000 << kRs2Shift); // Assuming OP_FP covers the base opcode
    pub const RO_FCVT_D_L: Opcode = Opcode::OP_FP; //| (0b1101001 << kFunct7Shift) | (0b00010 << kRs2Shift); // Assuming OP_FP covers the base opcode
    pub const RO_FCVT_D_LU: Opcode = Opcode::OP_FP; //| (0b1101001 << kFunct7Shift) | (0b00011 << kRs2Shift); // Assuming OP_FP covers the base opcode
    pub const RO_FMV_D_X: Opcode = Opcode::OP_FP; //| (0b000 << kFunct3Shift) | (0b1111001 << kFunct7Shift) | (0b00000 << kRs2Shift); // Assuming OP_FP covers the base opcode
}
}
