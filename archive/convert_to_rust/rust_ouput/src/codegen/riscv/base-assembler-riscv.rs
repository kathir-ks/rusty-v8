// Converted from V8 C++ source files:
// Header: base-assembler-riscv.h
// Implementation: base-assembler-riscv.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_camel_case_types)]

use std::fmt;

pub struct SafepointTableBuilder {}

pub struct Label {}

impl Label {
    pub fn is_bound(&self) -> bool {
        true
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BaseOpcode {
    LOAD,
    STORE,
    OP,
    OP_IMM,
    LUI,
    AUIPC,
    JAL,
    JALR,
    BRANCH,
    SYSTEM,
    AMO,
    OP_FP,
    LOAD_FP,
    STORE_FP,
    OP_32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FPURoundingMode {
    kNearestTiesToEven,
    kZero,
    kUp,
    kDown,
    kNearestTiesToAway,
    kInvalid,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ControlStatusReg {
    FCSR,
    FFLAGS,
    FRM,
}

impl From<ControlStatusReg> for i16 {
    fn from(csr: ControlStatusReg) -> Self {
        match csr {
            ControlStatusReg::FCSR => 0x000,
            ControlStatusReg::FFLAGS => 0x001,
            ControlStatusReg::FRM => 0x002,
        }
    }
}

pub trait IsValid {
    fn is_valid(&self) -> bool;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Register {
    code: u8,
}

impl Register {
    pub fn from_code(code: u8) -> Self {
        Register { code }
    }

    pub fn code(&self) -> u8 {
        self.code
    }
}

impl IsValid for Register {
    fn is_valid(&self) -> bool {
        self.code < 32
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x{}", self.code)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FPURegister {
    code: u8,
}

impl FPURegister {
    pub fn from_code(code: u8) -> Self {
        FPURegister { code }
    }

    pub fn code(&self) -> u8 {
        self.code
    }
}

impl IsValid for FPURegister {
    fn is_valid(&self) -> bool {
        self.code < 32
    }
}

impl fmt::Display for FPURegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "f{}", self.code)
    }
}

fn ToRegister(imm5: u8) -> Register {
    Register { code: imm5 }
}

type Instr = u32;
type ShortInstr = u16;

const kRdShift: u32 = 7;
const kFunct3Shift: u32 = 12;
const kRs1Shift: u32 = 15;
const kRs2Shift: u32 = 20;
const kFunct7Shift: u32 = 25;
const kImm12Shift: u32 = 20;
const kShamtShift: u32 = 20;
const kShamtWShift: u32 = 20;
const kImm20Shift: u32 = 12;

const kRlShift: u32 = 26;
const kAqShift: u32 = 27;
const kFunct5Shift: u32 = 27;
const kFunct2Shift: u32 = 25;
const kRs3Shift: u32 = 27;
const kFunct6Shift: u32 = 26;
const kArithShiftShift: u32 = kFunct6Shift;

const kRvcRs2Shift: u32 = 2;
const kRvcRdShift: u32 = 7;
const kRvcFunct4Shift: u32 = 12;
const kRvcRs2sShift: u32 = 2;
const kRvcRs1sShift: u32 = 7;
const kRvcFunct6Shift: u32 = 10;
const kRvcFunct2Shift: u32 = 12;
const kRvcFunct3Shift: u32 = 13;

const AMO: Instr = 0x02F00000 | 0x0000000F;

fn is_uint2(x: u8) -> bool {
    x < 4
}

fn is_uint3(x: u8) -> bool {
    x < 8
}

fn is_uint4(x: u8) -> bool {
    x < 16
}

fn is_uint5(x: u8) -> bool {
    x < 32
}

fn is_uint6(x: u8) -> bool {
    x < 64
}

fn is_uint7(x: u8) -> bool {
    x < 128
}

fn is_uint8(x: u8) -> bool {
    x < 256
}

fn is_uint11(x: u16) -> bool {
    x < 2048
}

fn is_uint12(x: i16) -> bool {
    x >= 0 && x < 4096
}

fn is_uint20(x: i32) -> bool {
    x >= 0 && x < (1 << 20)
}

fn is_int6(x: i8) -> bool {
    x >= -32 && x < 32
}

fn is_int8(x: i8) -> bool {
    x >= -128 && x < 128
}

fn is_int12(x: i16) -> bool {
    x >= -2048 && x < 2048
}

fn is_int13(x: i16) -> bool {
    x >= -4096 && x < 4096
}

fn is_int20(x: i32) -> bool {
    x >= -(1 << 19) && x < (1 << 19)
}

fn is_int21(x: i32) -> bool {
    x >= -(1 << 20) && x < (1 << 20)
}

pub trait AssemblerRiscvBaseTrait {
    fn branch_offset_helper(&mut self, l: &Label, bits: OffsetSize) -> i32;
    fn emit(&mut self, x: Instr);
    fn emit_short(&mut self, x: ShortInstr);
    fn emit_u64(&mut self, x: u64);
    fn clear_vectorunit(&mut self);

    fn gen_instr_r(&mut self, funct7: u8, funct3: u8, opcode: BaseOpcode, rd: Register, rs1: Register, rs2: Register);
    fn gen_instr_r_fpu(&mut self, funct7: u8, funct3: u8, opcode: BaseOpcode, rd: FPURegister, rs1: FPURegister, rs2: FPURegister);
    fn gen_instr_r_reg_fpu_reg(&mut self, funct7: u8, funct3: u8, opcode: BaseOpcode, rd: Register, rs1: FPURegister, rs2: Register);
    fn gen_instr_r_fpu_reg_reg(&mut self, funct7: u8, funct3: u8, opcode: BaseOpcode, rd: FPURegister, rs1: Register, rs2: Register);
    fn gen_instr_r_fpu_fpu_reg(&mut self, funct7: u8, funct3: u8, opcode: BaseOpcode, rd: FPURegister, rs1: FPURegister, rs2: Register);
    fn gen_instr_r_reg_fpu_fpu(&mut self, funct7: u8, funct3: u8, opcode: BaseOpcode, rd: Register, rs1: FPURegister, rs2: FPURegister);
    fn gen_instr_r4(&mut self, funct2: u8, opcode: BaseOpcode, rd: Register, rs1: Register, rs2: Register, rs3: Register, frm: FPURoundingMode);
    fn gen_instr_r4_fpu(&mut self, funct2: u8, opcode: BaseOpcode, rd: FPURegister, rs1: FPURegister, rs2: FPURegister, rs3: FPURegister, frm: FPURoundingMode);
    fn gen_instr_ratomic(&mut self, funct5: u8, aq: bool, rl: bool, funct3: u8, rd: Register, rs1: Register, rs2: Register);
    fn gen_instr_rfrm(&mut self, funct7: u8, opcode: BaseOpcode, rd: Register, rs1: Register, rs2: Register, frm: FPURoundingMode);
    fn gen_instr_i(&mut self, funct3: u8, opcode: BaseOpcode, rd: Register, rs1: Register, imm12: i16);
    fn gen_instr_i_fpu(&mut self, funct3: u8, opcode: BaseOpcode, rd: FPURegister, rs1: Register, imm12: i16);
    fn gen_instr_ishift(&mut self, funct7: u8, funct3: u8, opcode: BaseOpcode, rd: Register, rs1: Register, shamt: u8);
    fn gen_instr_ishiftw(&mut self, funct7: u8, funct3: u8, opcode: BaseOpcode, rd: Register, rs1: Register, shamt: u8);
    fn gen_instr_s(&mut self, funct3: u8, opcode: BaseOpcode, rs1: Register, rs2: Register, imm12: i16);
    fn gen_instr_s_fpu(&mut self, funct3: u8, opcode: BaseOpcode, rs1: Register, rs2: FPURegister, imm12: i16);
    fn gen_instr_b(&mut self, funct3: u8, opcode: BaseOpcode, rs1: Register, rs2: Register, imm12: i16);
    fn gen_instr_u(&mut self, opcode: BaseOpcode, rd: Register, imm20: i32);
    fn gen_instr_j(&mut self, opcode: BaseOpcode, rd: Register, imm20: i32);
    fn gen_instr_cr(&mut self, funct4: u8, opcode: BaseOpcode, rd: Register, rs2: Register);
    fn gen_instr_ca(&mut self, funct6: u8, opcode: BaseOpcode, rd: Register, funct: u8, rs2: Register);
    fn gen_instr_ci(&mut self, funct3: u8, opcode: BaseOpcode, rd: Register, imm6: i8);
    fn gen_instr_ciu(&mut self, funct3: u8, opcode: BaseOpcode, rd: Register, uimm6: u8);
    fn gen_instr_ciu_fpu(&mut self, funct3: u8, opcode: BaseOpcode, rd: FPURegister, uimm6: u8);
    fn gen_instr_ciw(&mut self, funct3: u8, opcode: BaseOpcode, rd: Register, uimm8: u8);
    fn gen_instr_css_reg(&mut self, funct3: u8, opcode: BaseOpcode, rs2: Register, uimm6: u8);
    fn gen_instr_css_fpu(&mut self, funct3: u8, opcode: BaseOpcode, rs2: FPURegister, uimm6: u8);
    fn gen_instr_cl_reg(&mut self, funct3: u8, opcode: BaseOpcode, rd: Register, rs1: Register, uimm5: u8);
    fn gen_instr_cl_fpu(&mut self, funct3: u8, opcode: BaseOpcode, rd: FPURegister, rs1: Register, uimm5: u8);
    fn gen_instr_cs_reg(&mut self, funct3: u8, opcode: BaseOpcode, rs2: Register, rs1: Register, uimm5: u8);
    fn gen_instr_cs_fpu(&mut self, funct3: u8, opcode: BaseOpcode, rs2: FPURegister, rs1: Register, uimm5: u8);
    fn gen_instr_cj(&mut self, funct3: u8, opcode: BaseOpcode, uint11: u16);
    fn gen_instr_cb(&mut self, funct3: u8, opcode: BaseOpcode, rs1: Register, uimm8: u8);
    fn gen_instr_cba(&mut self, funct3: u8, funct2: u8, opcode: BaseOpcode, rs1: Register, imm6: i8);

    fn gen_instr_branchcc_rri(&mut self, funct3: u8, rs1: Register, rs2: Register, imm12: i16);
    fn gen_instr_load_ri(&mut self, funct3: u8, rd: Register, rs1: Register, imm12: i16);
    fn gen_instr_store_rri(&mut self, funct3: u8, rs1: Register, rs2: Register, imm12: i16);
    fn gen_instr_alu_ri(&mut self, funct3: u8, rd: Register, rs1: Register, imm12: i16);
    fn gen_instr_shift_ri(&mut self, arithshift: bool, funct3: u8, rd: Register, rs1: Register, shamt: u8);
    fn gen_instr_alu_rr(&mut self, funct7: u8, funct3: u8, rd: Register, rs1: Register, rs2: Register);
    fn gen_instr_csr_ir(&mut self, funct3: u8, rd: Register, csr: ControlStatusReg, rs1: Register);
    fn gen_instr_csr_ii(&mut self, funct3: u8, rd: Register, csr: ControlStatusReg, rs1: u8);
    fn gen_instr_shiftw_ri(&mut self, arithshift: bool, funct3: u8, rd: Register, rs1: Register, shamt: u8);
    fn gen_instr_aluw_rr(&mut self, funct7: u8, funct3: u8, rd: Register, rs1: Register, rs2: Register);
    fn gen_instr_priv(&mut self, funct7: u8, rs1: Register, rs2: Register);
    fn gen_instr_loadfp_ri(&mut self, funct3: u8, rd: FPURegister, rs1: Register, imm12: i16);
    fn gen_instr_storefp_rri(&mut self, funct3: u8, rs1: Register, rs2: FPURegister, imm12: i16);
    fn gen_instr_alufp_rr_fpu(&mut self, funct7: u8, funct3: u8, rd: FPURegister, rs1: FPURegister, rs2: FPURegister);
    fn gen_instr_alufp_rr_reg(&mut self, funct7: u8, funct3: u8, rd: FPURegister, rs1: Register, rs2: Register);
    fn gen_instr_alufp_rr_fpu_reg(&mut self, funct7: u8, funct3: u8, rd: FPURegister, rs1: FPURegister, rs2: Register);
    fn gen_instr_alufp_rr_reg_fpu(&mut self, funct7: u8, funct3: u8, rd: Register, rs1: FPURegister, rs2: Register);
    fn gen_instr_alufp_rr_reg_fpu_fpu(&mut self, funct7: u8, funct3: u8, rd: Register, rs1: FPURegister, rs2: FPURegister);

    fn block_trampoline_pool_for(&mut self, instructions: i32);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OffsetSize {
    kOffset21 = 21,
    kOffset12 = 12,
    kOffset20 = 20,
    kOffset13 = 13,
    kOffset32 = 32,
    kOffset11 = 11,
    kOffset9 = 9,
}

pub struct AssemblerRiscvBase {}

impl AssemblerRiscvBase {
    pub fn new() -> Self {
        AssemblerRiscvBase {}
    }
}

impl AssemblerRiscvBaseTrait for AssemblerRiscvBase {
    fn branch_offset_helper(&mut self, _l: &Label, _bits: OffsetSize) -> i32 {
        0
    }

    fn emit(&mut self, _x: Instr) {}

    fn emit_short(&mut self, _x: ShortInstr) {}

    fn emit_u64(&mut self, _x: u64) {}

    fn clear_vectorunit(&mut self) {}

    fn gen_instr_r(&mut self, funct7: u8, funct3: u8, opcode: BaseOpcode, rd: Register, rs1: Register, rs2: Register) {
        assert!(is_uint7(funct7) && is_uint3(funct3) && rd.is_valid() && rs1.is_valid() && rs2.is_valid());
        let instr: Instr = opcode as Instr | (rd.code() as Instr << kRdShift) | (funct3 as Instr << kFunct3Shift) |
            (rs1.code() as Instr << kRs1Shift) | (rs2.code() as Instr << kRs2Shift) |
            (funct7 as Instr << kFunct7Shift);
        self.emit(instr);
    }

    fn gen_instr_r_fpu(&mut self, funct7: u8, funct3: u8, opcode: BaseOpcode, rd: FPURegister, rs1: FPURegister, rs2: FPURegister) {
        assert!(is_uint7(funct7) && is_uint3(funct3) && rd.is_valid() && rs1.is_valid() && rs2.is_valid());
        let instr: Instr = opcode as Instr | (rd.code() as Instr << kRdShift) | (funct3 as Instr << kFunct3Shift) |
            (rs1.code() as Instr << kRs1Shift) | (rs2.code() as Instr << kRs2Shift) |
            (funct7 as Instr << kFunct7Shift);
        self.emit(instr);
    }

    fn gen_instr_r_reg_fpu_reg(&mut self, funct7: u8, funct3: u8, opcode: BaseOpcode, rd: Register, rs1: FPURegister, rs2: Register) {
        assert!(is_uint7(funct7) && is_uint3(funct3) && rd.is_valid() && rs1.is_valid() && rs2.is_valid());
        let instr: Instr = opcode as Instr | (rd.code() as Instr << kRdShift) | (funct3 as Instr << kFunct3Shift) |
            (rs1.code() as Instr << kRs1Shift) | (rs2.code() as Instr << kRs2Shift) |
            (funct7 as Instr << kFunct7Shift);
        self.emit(instr);
    }

    fn gen_instr_r_fpu_reg_reg(&mut self, funct7: u8, funct3: u8, opcode: BaseOpcode, rd: FPURegister, rs1: Register, rs2: Register) {
        assert!(is_uint7(funct7) && is_uint3(funct3) && rd.is_valid() && rs1.is_valid() && rs2.is_valid());
        let instr: Instr = opcode as Instr | (rd.code() as Instr << kRdShift) | (funct3 as Instr << kFunct3Shift) |
            (rs1.code() as Instr << kRs1Shift) | (rs2.code() as Instr << kRs2Shift) |
            (funct7 as Instr << kFunct7Shift);
        self.emit(instr);
    }

    fn gen_instr_r_fpu_fpu_reg(&mut self, funct7: u8, funct3: u8, opcode: BaseOpcode, rd: FPURegister, rs1: FPURegister, rs2: Register) {
        assert!(is_uint7(funct7) && is_uint3(funct3) && rd.is_valid() && rs1.is_valid() && rs2.is_valid());
        let instr: Instr = opcode as Instr | (rd.code() as Instr << kRdShift) | (funct3 as Instr << kFunct3Shift) |
            (rs1.code() as Instr << kRs1Shift) | (rs2.code() as Instr << kRs2Shift) |
            (funct7 as Instr << kFunct7Shift);
        self.emit(instr);
    }

    fn gen_instr_r_reg_fpu_fpu(&mut self, funct7: u8, funct3: u8, opcode: BaseOpcode, rd: Register, rs1: FPURegister, rs2: FPURegister) {
        assert!(is_uint7(funct7) && is_uint3(funct3) && rd.is_valid() && rs1.is_valid() && rs2.is_valid());
        let instr: Instr = opcode as Instr | (rd.code() as Instr << kRdShift) | (funct3 as Instr << kFunct3Shift) |
            (rs1.code() as Instr << kRs1Shift) | (rs2.code() as Instr << kRs2Shift) |
            (funct7 as Instr << kFunct7Shift);
        self.emit(instr);
    }

    fn gen_instr_r4(&mut self, funct2: u8, opcode: BaseOpcode, rd: Register, rs1: Register, rs2: Register, rs3: Register, frm: FPURoundingMode) {
        assert!(is_uint2(funct2) && rd.is_valid() && rs1.is_valid() && rs2.is_valid() && rs3.is_valid() && is_uint3(frm as u8));
        let instr: Instr = opcode as Instr | (rd.code() as Instr << kRdShift) | ((frm as u8) as Instr << kFunct3Shift) |
            (rs1.code() as Instr << kRs1Shift) | (rs2.code() as Instr << kRs2Shift) |
            (funct2 as Instr << kFunct2Shift) | (rs3.code() as Instr << kRs3Shift);
        self.emit(instr);
    }

    fn gen_instr_r4_fpu(&mut self, funct2: u8, opcode: BaseOpcode, rd: FPURegister, rs1: FPURegister, rs2: FPURegister, rs3: FPURegister, frm: FPURoundingMode) {
        assert!(is_uint2(funct2) && rd.is_valid() && rs1.is_valid() && rs2.is_valid() && rs3.is_valid() && is_uint3(frm as u8));
        let instr: Instr = opcode as Instr | (rd.code() as Instr << kRdShift) | ((frm as u8) as Instr << kFunct3Shift) |
            (rs1.code() as Instr << kRs1Shift) | (rs2.code() as Instr << kRs2Shift) |
            (funct2 as Instr << kFunct2Shift) | (rs3.code() as Instr << kRs3Shift);
        self.emit(instr);
    }

    fn gen_instr_ratomic(&mut self, funct5: u8, aq: bool, rl: bool, funct3: u8, rd: Register, rs1: Register, rs2: Register) {
        assert!(is_uint5(funct5) && is_uint3(funct3) && rd.is_valid() && rs1.is_valid() && rs2.is_valid());
        let instr: Instr = AMO | (rd.code() as Instr << kRdShift) | (funct3 as Instr << kFunct3Shift) |
            (rs1.code() as Instr << kRs1Shift) | (rs2.code() as Instr << kRs2Shift) |
            ((rl as u8) as Instr << kRlShift) | ((aq as u8) as Instr << kAqShift) | (funct5 as Instr << kFunct5Shift);
        self.emit(instr);
    }

    fn gen_instr_rfrm(&mut self, funct7: u8, opcode: BaseOpcode, rd: Register, rs1: Register, rs2: Register, frm: FPURoundingMode) {
        assert!(rd.is_valid() && rs1.is_valid() && rs2.is_valid() && is_uint3(frm as u8));
        let instr: Instr = opcode as Instr | (rd.code() as Instr << kRdShift) | ((frm as u8) as Instr << kFunct3Shift) |
            (rs1.code() as Instr << kRs1Shift) | (rs2.code() as Instr << kRs2Shift) |
            (funct7 as Instr << kFunct7Shift);
        self.emit(instr);
    }

    fn gen_instr_i(&mut self, funct3: u8, opcode: BaseOpcode, rd: Register, rs1: Register, imm12: i16) {
        assert!(is_uint3(funct3) && rd.is_valid() && rs1.is_valid() && (is_uint12(imm12) || is_int12(imm12)));
        let instr: Instr = opcode as Instr | (rd.code() as Instr << kRdShift) | (funct3 as Instr << kFunct3Shift) |
            (rs1.code() as Instr << kRs1Shift) | ((imm12 as i32) as Instr << kImm12Shift);
        self.emit(instr);
    }

    fn gen_instr_i_fpu(&mut self, funct3: u8, opcode: BaseOpcode, rd: FPURegister, rs1: Register, imm12: i16) {
        assert!(is_uint3(funct3) && rd.is_valid() && rs1.is_valid() && (is_uint12(imm12) || is_int12(imm12)));
        let instr: Instr = opcode as Instr | (rd.code() as Instr << kRdShift) | (funct3 as Instr << kFunct3Shift) |
            (rs1.code() as Instr << kRs1Shift) | ((imm12 as i32) as Instr << kImm12Shift);
        self.emit(instr);
    }

    fn gen_instr_ishift(&mut self, funct7: u8, funct3: u8, opcode: BaseOpcode, rd: Register, rs1: Register, shamt: u8) {
        assert!(is_uint3(funct3) && rd.is_valid() && rs1.is_valid() && is_uint6(shamt));
        let instr: Instr = opcode as Instr | (rd.code() as Instr << kRdShift) | (funct3 as Instr << kFunct3Shift) |
            (rs1.code() as Instr << kRs1Shift) | ((shamt as Instr) << kShamtShift) |
            ((funct7 as Instr) << kFunct6Shift);
        self.emit(instr);
    }

    fn gen_instr_ishiftw(&mut self, funct7: u8, funct3: u8, opcode: BaseOpcode, rd: Register, rs1: Register, shamt: u8) {
        assert!(is_uint3(funct3) && rd.is_valid() && rs1.is_valid() && is_uint5(shamt));
        let instr: Instr = opcode as Instr | (rd.code() as Instr << kRdShift) | (funct3 as Instr << kFunct3Shift) |
            (rs1.code() as Instr << kRs1Shift) | ((shamt as Instr) << kShamtWShift) |
            ((funct7 as Instr) << kFunct7Shift);
        self.emit(instr);
    }

    fn gen_instr_s(&mut self, funct3: u8, opcode: BaseOpcode, rs1: Register, rs2: Register, imm12: i16) {
        assert!(is_uint3(funct3) && rs1.is_valid() && rs2.is_valid() && is_int12(imm12));
        let instr: Instr = opcode as Instr | (((imm12 & 0x1f) as i32) << 7) as Instr |  // bits  4-0
            ((funct3 as Instr) << kFunct3Shift) | (rs1.code() as Instr << kRs1Shift) |
            (rs2.code() as Instr << kRs2Shift) |
            (((imm12 & 0xfe0) as i32) << 20) as Instr; // bits 11-5
        self.emit(instr);
    }

    fn gen_instr_s_fpu(&mut self, funct3: u8, opcode: BaseOpcode, rs1: Register, rs2: FPURegister, imm12: i16) {
        assert!(is_uint3(funct3) && rs1.is_valid() && rs2.is_valid() && is_int12(imm12));
        let instr: Instr = opcode as Instr | (((imm12 & 0x1f) as i32) << 7) as Instr |  // bits  4-0
            ((funct3 as Instr) << kFunct3Shift) | (rs1.code() as Instr << kRs1Shift) |
            (rs2.code() as Instr << kRs2Shift) |
            (((imm12 & 0xfe0) as i32) << 20) as Instr; // bits 11-5
        self.emit(instr);
    }

    fn gen_instr_b(&mut self, funct3: u8, opcode: BaseOpcode, rs1: Register, rs2: Register, imm12: i16) {
        assert!(is_uint3(funct3) && rs1.is_valid() && rs2.is_valid() &&
            is_int13(imm12) && ((imm12 & 1) == 0));
        let imm13 = imm12 as i32;
        let instr: Instr = opcode as Instr | (((imm13 & 0x800) >> 4) as Instr) |  // bit  11
            (((imm13 & 0x1e) << 7) as Instr) |            // bits 4-1
            ((funct3 as Instr) << kFunct3Shift) | (rs1.code() as Instr << kRs1Shift) |
            (rs2.code() as Instr << kRs2Shift) |
            (((imm13 & 0x7e0) << 20) as Instr) |  // bits 10-5
            (((imm13 & 0x1000) << 19) as Instr); // bit 12
        self.emit(instr);
    }

    fn gen_instr_u(&mut self, opcode: BaseOpcode, rd: Register, imm20: i32) {
        assert!(rd.is_valid() && (is_int20(imm20) || is_uint20(imm20)));
        let instr: Instr = opcode as Instr | (rd.code() as Instr << kRdShift) | ((imm20 as Instr) << kImm20Shift);
        self.emit(instr);
    }

    fn gen_instr_j(&mut self, opcode: BaseOpcode, rd: Register, imm20: i32) {
        assert!(rd.is_valid() && is_int21(imm20) && ((imm20 & 1) == 0));
        let instr: Instr = opcode as Instr | (rd.code() as Instr << kRdShift) |
            ((imm20 & 0xff000) as Instr) |          // bits 19-12
            (((imm20 & 0x800) << 9) as Instr) |     // bit  11
            (((imm20 & 0x7fe) << 20) as Instr) |    // bits 10-1
            (((imm20 & 0x100000) << 11) as Instr); // bit  20
        self.emit(instr);
    }

    fn gen_instr_cr(&mut self, funct4: u8, opcode: BaseOpcode, rd: Register, rs2: Register) {
        assert!(is_uint4(funct4) && rd.is_valid() && rs2.is_valid());
        let instr: ShortInstr = opcode as ShortInstr | ((rs2.code() as ShortInstr) << kRvcRs2Shift) |
            ((rd.code() as ShortInstr) << kRvcRdShift) | ((funct4 as ShortInstr) << kRvcFunct4Shift);
        self.emit_short(instr);
    }

    fn gen_instr_ca(&mut self, funct6: u8, opcode: BaseOp
