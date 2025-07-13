// Converted from V8 C++ source files:
// Header: extension-riscv-d.h
// Implementation: extension-riscv-d.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_camel_case_types)]

use std::error::Error;
use std::fmt;

//use crate::codegen::riscv::register_riscv::FPURoundingMode;
//use crate::codegen::riscv::register_riscv::FPURegister;
//use crate::codegen::riscv::register_riscv::Register;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FPURoundingMode {
    RNE,
    RTZ,
    RDN,
    RUP,
    RMM,
    DYN,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FPURegister {
    code_: u8,
}

impl FPURegister {
    pub fn from_code(code: u8) -> Self {
        FPURegister { code_: code }
    }

    pub fn code(&self) -> u8 {
        self.code_
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Register {
    code_: u8,
}

impl Register {
    pub fn from_code(code: u8) -> Self {
        Register { code_: code }
    }

    pub fn code(&self) -> u8 {
        self.code_
    }
}

pub struct AssemblerRISCVD {}

impl AssemblerRISCVD {
    pub fn new() -> Self {
        AssemblerRISCVD {}
    }

    pub fn fld(&mut self, rd: FPURegister, rs1: Register, imm12: i16) -> Result<(), AssemblerError> {
        self.gen_instr_load_fp_ri(0b011, rd, rs1, imm12)
    }

    pub fn fsd(&mut self, source: FPURegister, base: Register, imm12: i16) -> Result<(), AssemblerError> {
        self.gen_instr_store_fp_rri(0b011, base, source, imm12)
    }

    pub fn fmadd_d(
        &mut self,
        rd: FPURegister,
        rs1: FPURegister,
        rs2: FPURegister,
        rs3: FPURegister,
        frm: FPURoundingMode,
    ) -> Result<(), AssemblerError> {
        self.gen_instr_r4(0b01, MADD, rd, rs1, rs2, rs3, frm)
    }

    pub fn fmsub_d(
        &mut self,
        rd: FPURegister,
        rs1: FPURegister,
        rs2: FPURegister,
        rs3: FPURegister,
        frm: FPURoundingMode,
    ) -> Result<(), AssemblerError> {
        self.gen_instr_r4(0b01, MSUB, rd, rs1, rs2, rs3, frm)
    }

    pub fn fnmsub_d(
        &mut self,
        rd: FPURegister,
        rs1: FPURegister,
        rs2: FPURegister,
        rs3: FPURegister,
        frm: FPURoundingMode,
    ) -> Result<(), AssemblerError> {
        self.gen_instr_r4(0b01, NMSUB, rd, rs1, rs2, rs3, frm)
    }

    pub fn fnmadd_d(
        &mut self,
        rd: FPURegister,
        rs1: FPURegister,
        rs2: FPURegister,
        rs3: FPURegister,
        frm: FPURoundingMode,
    ) -> Result<(), AssemblerError> {
        self.gen_instr_r4(0b01, NMADD, rd, rs1, rs2, rs3, frm)
    }

    pub fn fadd_d(
        &mut self,
        rd: FPURegister,
        rs1: FPURegister,
        rs2: FPURegister,
        frm: FPURoundingMode,
    ) -> Result<(), AssemblerError> {
        self.gen_instr_alu_fp_rr(0b0000001, frm, rd, rs1, rs2)
    }

    pub fn fsub_d(
        &mut self,
        rd: FPURegister,
        rs1: FPURegister,
        rs2: FPURegister,
        frm: FPURoundingMode,
    ) -> Result<(), AssemblerError> {
        self.gen_instr_alu_fp_rr(0b0000101, frm, rd, rs1, rs2)
    }

    pub fn fmul_d(
        &mut self,
        rd: FPURegister,
        rs1: FPURegister,
        rs2: FPURegister,
        frm: FPURoundingMode,
    ) -> Result<(), AssemblerError> {
        self.gen_instr_alu_fp_rr(0b0001001, frm, rd, rs1, rs2)
    }

    pub fn fdiv_d(
        &mut self,
        rd: FPURegister,
        rs1: FPURegister,
        rs2: FPURegister,
        frm: FPURoundingMode,
    ) -> Result<(), AssemblerError> {
        self.gen_instr_alu_fp_rr(0b0001101, frm, rd, rs1, rs2)
    }

    pub fn fsqrt_d(&mut self, rd: FPURegister, rs1: FPURegister, frm: FPURoundingMode) -> Result<(), AssemblerError> {
        self.gen_instr_alu_fp_rr(0b0101101, frm, rd, rs1, ZERO_REG)
    }

    pub fn fsgnj_d(&mut self, rd: FPURegister, rs1: FPURegister, rs2: FPURegister) -> Result<(), AssemblerError> {
        self.gen_instr_alu_fp_rr(0b0010001, 0b000, rd, rs1, rs2)
    }

    pub fn fsgnjn_d(&mut self, rd: FPURegister, rs1: FPURegister, rs2: FPURegister) -> Result<(), AssemblerError> {
        self.gen_instr_alu_fp_rr(0b0010001, 0b001, rd, rs1, rs2)
    }

    pub fn fsgnjx_d(&mut self, rd: FPURegister, rs1: FPURegister, rs2: FPURegister) -> Result<(), AssemblerError> {
        self.gen_instr_alu_fp_rr(0b0010001, 0b010, rd, rs1, rs2)
    }

    pub fn fmin_d(&mut self, rd: FPURegister, rs1: FPURegister, rs2: FPURegister) -> Result<(), AssemblerError> {
        self.gen_instr_alu_fp_rr(0b0010101, 0b000, rd, rs1, rs2)
    }

    pub fn fmax_d(&mut self, rd: FPURegister, rs1: FPURegister, rs2: FPURegister) -> Result<(), AssemblerError> {
        self.gen_instr_alu_fp_rr(0b0010101, 0b001, rd, rs1, rs2)
    }

    pub fn fcvt_s_d(&mut self, rd: FPURegister, rs1: FPURegister, frm: FPURoundingMode) -> Result<(), AssemblerError> {
        self.gen_instr_alu_fp_rr(0b0100000, frm, rd, rs1, FTOREGISTER_1)
    }

    pub fn fcvt_d_s(&mut self, rd: FPURegister, rs1: FPURegister, frm: FPURoundingMode) -> Result<(), AssemblerError> {
        self.gen_instr_alu_fp_rr(0b0100001, frm, rd, rs1, ZERO_REG)
    }

    pub fn feq_d(&mut self, rd: Register, rs1: FPURegister, rs2: FPURegister) -> Result<(), AssemblerError> {
        self.gen_instr_alu_fp_rr(0b1010001, 0b010, rd, rs1, rs2)
    }

    pub fn flt_d(&mut self, rd: Register, rs1: FPURegister, rs2: FPURegister) -> Result<(), AssemblerError> {
        self.gen_instr_alu_fp_rr(0b1010001, 0b001, rd, rs1, rs2)
    }

    pub fn fle_d(&mut self, rd: Register, rs1: FPURegister, rs2: FPURegister) -> Result<(), AssemblerError> {
        self.gen_instr_alu_fp_rr(0b1010001, 0b000, rd, rs1, rs2)
    }

    pub fn fclass_d(&mut self, rd: Register, rs1: FPURegister) -> Result<(), AssemblerError> {
        self.gen_instr_alu_fp_rr(0b1110001, 0b001, rd, rs1, ZERO_REG)
    }

    pub fn fcvt_w_d(&mut self, rd: Register, rs1: FPURegister, frm: FPURoundingMode) -> Result<(), AssemblerError> {
        self.gen_instr_alu_fp_rr(0b1100001, frm, rd, rs1, ZERO_REG)
    }

    pub fn fcvt_wu_d(&mut self, rd: Register, rs1: FPURegister, frm: FPURoundingMode) -> Result<(), AssemblerError> {
        self.gen_instr_alu_fp_rr(0b1100001, frm, rd, rs1, FTOREGISTER_1)
    }

    pub fn fcvt_d_w(&mut self, rd: FPURegister, rs1: Register, frm: FPURoundingMode) -> Result<(), AssemblerError> {
        self.gen_instr_alu_fp_rr(0b1101001, frm, rd, rs1, ZERO_REG)
    }

    pub fn fcvt_d_wu(&mut self, rd: FPURegister, rs1: Register, frm: FPURoundingMode) -> Result<(), AssemblerError> {
        self.gen_instr_alu_fp_rr(0b1101001, frm, rd, rs1, FTOREGISTER_1)
    }

    #[cfg(target_arch = "riscv64")]
    pub fn fcvt_l_d(&mut self, rd: Register, rs1: FPURegister, frm: FPURoundingMode) -> Result<(), AssemblerError> {
        self.gen_instr_alu_fp_rr(0b1100001, frm, rd, rs1, FTOREGISTER_2)
    }

    #[cfg(target_arch = "riscv64")]
    pub fn fcvt_lu_d(&mut self, rd: Register, rs1: FPURegister, frm: FPURoundingMode) -> Result<(), AssemblerError> {
        self.gen_instr_alu_fp_rr(0b1100001, frm, rd, rs1, FTOREGISTER_3)
    }

    #[cfg(target_arch = "riscv64")]
    pub fn fmv_x_d(&mut self, rd: Register, rs1: FPURegister) -> Result<(), AssemblerError> {
        self.gen_instr_alu_fp_rr(0b1110001, 0b000, rd, rs1, ZERO_REG)
    }

    #[cfg(target_arch = "riscv64")]
    pub fn fcvt_d_l(&mut self, rd: FPURegister, rs1: Register, frm: FPURoundingMode) -> Result<(), AssemblerError> {
        self.gen_instr_alu_fp_rr(0b1101001, frm, rd, rs1, FTOREGISTER_2)
    }

    #[cfg(target_arch = "riscv64")]
    pub fn fcvt_d_lu(&mut self, rd: FPURegister, rs1: Register, frm: FPURoundingMode) -> Result<(), AssemblerError> {
        self.gen_instr_alu_fp_rr(0b1101001, frm, rd, rs1, FTOREGISTER_3)
    }

    #[cfg(target_arch = "riscv64")]
    pub fn fmv_d_x(&mut self, rd: FPURegister, rs1: Register) -> Result<(), AssemblerError> {
        self.gen_instr_alu_fp_rr(0b1111001, 0b000, rd, rs1, ZERO_REG)
    }

    pub fn fmv_d(&mut self, rd: FPURegister, rs: FPURegister) -> Result<(), AssemblerError> {
        self.fsgnj_d(rd, rs, rs)
    }

    pub fn fabs_d(&mut self, rd: FPURegister, rs: FPURegister) -> Result<(), AssemblerError> {
        self.fsgnjx_d(rd, rs, rs)
    }

    pub fn fneg_d(&mut self, rd: FPURegister, rs: FPURegister) -> Result<(), AssemblerError> {
        self.fsgnjn_d(rd, rs, rs)
    }

    fn gen_instr_load_fp_ri(&mut self, opcode: u8, rd: FPURegister, rs1: Register, imm12: i16) -> Result<(), AssemblerError> {
        // Placeholder implementation
        println!(
            "Generating load FP instruction: opcode={}, rd={}, rs1={}, imm12={}",
            opcode, rd.code(), rs1.code(), imm12
        );
        Ok(())
    }

    fn gen_instr_store_fp_rri(&mut self, opcode: u8, base: Register, source: FPURegister, imm12: i16) -> Result<(), AssemblerError> {
        // Placeholder implementation
        println!(
            "Generating store FP instruction: opcode={}, base={}, source={}, imm12={}",
            opcode, base.code(), source.code(), imm12
        );
        Ok(())
    }

    fn gen_instr_r4(
        &mut self,
        opcode: u8,
        funct3: Funct3,
        rd: FPURegister,
        rs1: FPURegister,
        rs2: FPURegister,
        rs3: FPURegister,
        frm: FPURoundingMode,
    ) -> Result<(), AssemblerError> {
        // Placeholder implementation
        println!(
            "Generating R4 instruction: opcode={}, funct3={:?}, rd={}, rs1={}, rs2={}, rs3={}, frm={:?}",
            opcode, funct3, rd.code(), rs1.code(), rs2.code(), rs3.code(), frm
        );
        Ok(())
    }

    fn gen_instr_alu_fp_rr(
        &mut self,
        funct7: u8,
        frm: FPURoundingMode,
        rd: FPURegister,
        rs1: FPURegister,
        rs2: FPURegister,
    ) -> Result<(), AssemblerError> {
        // Placeholder implementation
        println!(
            "Generating ALU FP instruction: funct7={}, frm={:?}, rd={}, rs1={}, rs2={}",
            funct7, frm, rd.code(), rs1.code(), rs2.code()
        );
        Ok(())
    }
    fn to_register(&mut self, value: i32) -> Register {
        Register::from_code(value as u8)
    }
}

#[derive(Debug)]
pub enum AssemblerError {
    GenericError,
}

impl fmt::Display for AssemblerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AssemblerError")
    }
}

impl Error for AssemblerError {}

#[derive(Debug, Copy, Clone)]
enum Funct3 {
    MADD,
    MSUB,
    NMSUB,
    NMADD,
}

const MADD: Funct3 = Funct3::MADD;
const MSUB: Funct3 = Funct3::MSUB;
const NMSUB: Funct3 = Funct3::NMSUB;
const NMADD: Funct3 = Funct3::NMADD;

const RNE: FPURoundingMode = FPURoundingMode::RNE;

// Dummy implementation for ToRegister
trait ToRegisterTrait {
    fn to_register(self) -> Register;
}

impl ToRegisterTrait for i32 {
    fn to_register(self) -> Register {
        Register::from_code(self as u8)
    }
}

fn ToRegister(value: i32) -> Register {
    value.to_register()
}

const ZERO_REG: FPURegister = FPURegister { code_: 0 };
const FTOREGISTER_1: Register = Register { code_: 1 };
const FTOREGISTER_2: Register = Register { code_: 2 };
const FTOREGISTER_3: Register = Register { code_: 3 };
