// Copyright (c) 1994-2006 Sun Microsystems Inc.
// All Rights Reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
// - Redistributions of source code must retain the above copyright notice,
// this list of conditions and the following disclaimer.
//
// - Redistribution in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
//
// - Neither the name of Sun Microsystems or the names of contributors may
// be used to endorse or promote products derived from this software without
// specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
// IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
// PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

// The original source code covered by the above license above has been
// modified significantly by Google Inc.
// Copyright 2021 the V8 project authors. All rights reserved.

// src/codegen/riscv/base-assembler-riscv.h
// This should define the public interface for the module
pub mod base_assembler_riscv {
    // Re-export types that are part of the public interface
    pub use super::AssemblerRiscvBase;
    pub use super::BaseOpcode;
    pub use super::Register;
    pub use super::FPURegister;
    pub use super::FPURoundingMode;
    pub use super::ControlStatusReg;
}

// src/codegen/riscv/base-assembler-riscv.cc

// use crate::base::cpu; // Assuming cpu is a module in 'base'
use std::convert::TryInto;

// Define constants and types
const kRdShift: usize = 7;
const kFunct3Shift: usize = 12;
const kRs1Shift: usize = 15;
const kRs2Shift: usize = 20;
const kFunct7Shift: usize = 25;
const kFunct2Shift: usize = 25;
const kRs3Shift: usize = 27;
const kRlShift: usize = 21;
const kAqShift: usize = 22;
const kFunct5Shift: usize = 27;
const kImm12Shift: usize = 20;
const kShamtShift: usize = 20;
const kFunct6Shift: usize = 26;
const kShamtWShift: usize = 20;
const kImm20Shift: usize = 12;
const kRvcRs2Shift: usize = 2;
const kRvcRdShift: usize = 7;
const kRvcFunct4Shift: usize = 12;
const kRvcRs2sShift: usize = 2;
const kRvcRs1sShift: usize = 7;
const kRvcFunct6Shift: usize = 10;
const kRvcFunct2Shift: usize = 12;

const kArithShiftShift: usize = 30;

type Instr = u32;
type ShortInstr = u16;

// Placeholder enums and structs - Replace with actual definitions
#[derive(Debug, Copy, Clone)]
pub enum BaseOpcode {
    OP = 0b0110011,
    LOAD = 0b0000011,
    STORE = 0b0100011,
    OP_IMM = 0b0010011,
    BRANCH = 0b1100011,
    SYSTEM = 0b1110011,
    OP_IMM_32 = 0b0011011,
    OP_32 = 0b0111011,
    LOAD_FP = 0b0000111,
    STORE_FP = 0b0100111,
    OP_FP = 0b1010011,
    AMO = 0b0101111,
    // Add more opcodes as needed
}

#[derive(Debug, Copy, Clone)]
pub enum Register {
    Zero = 0,
    Ra,
    Sp,
    Gp,
    Tp,
    T0,
    T1,
    T2,
    S0,
    S1,
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11,
    T3,
    T4,
    T5,
    T6,
    // Add other registers as needed
}

impl Register {
    pub fn code(&self) -> u32 {
        *self as u32
    }

    pub fn is_valid(&self) -> bool {
        true // Replace with actual validity check
    }
}

#[derive(Debug, Copy, Clone)]
pub enum FPURegister {
    F0 = 0,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    F26,
    F27,
    F28,
    F29,
    F30,
    F31,
    // Add other registers as needed
}

impl FPURegister {
    pub fn code(&self) -> u32 {
        *self as u32
    }

    pub fn is_valid(&self) -> bool {
        true // Replace with actual validity check
    }
}

#[derive(Debug, Copy, Clone)]
pub enum FPURoundingMode {
    RNE = 0,
    RTZ,
    RDN,
    RUP,
    RMM,
    // Add other rounding modes as needed
}

impl FPURoundingMode {
    pub fn code(&self) -> u32 {
        *self as u32
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ControlStatusReg {
    // Example CSR
    Fflags = 0x001,
    // Add more CSRs as needed
}

impl From<ControlStatusReg> for i16 {
    fn from(csr: ControlStatusReg) -> Self {
        csr as i16
    }
}

#[derive(Debug)]
pub struct AssemblerRiscvBase {
    buffer: Vec<u8>, // Internal buffer to hold the generated code
}

impl AssemblerRiscvBase {
    pub fn new() -> Self {
        AssemblerRiscvBase { buffer: Vec::new() }
    }

    fn emit(&mut self, instr: Instr) {
        self.buffer.extend_from_slice(&instr.to_le_bytes());
    }

    fn emit(&mut self, instr: ShortInstr) {
        self.buffer.extend_from_slice(&instr.to_le_bytes());
    }

    fn is_uint7(value: u8) -> bool {
        value < 128
    }

    fn is_uint6(value: u8) -> bool {
        value < 64
    }

    fn is_uint5(value: u8) -> bool {
        value < 32
    }

    fn is_uint4(value: u8) -> bool {
        value < 16
    }

    fn is_uint3(value: u8) -> bool {
        value < 8
    }

    fn is_uint2(value: u8) -> bool {
        value < 4
    }

    fn is_uint8(value: u8) -> bool {
        true
    }

    fn is_uint11(value: u16) -> bool {
        value < 2048
    }

    fn is_uint12(value: i16) -> bool {
        value >= 0 && value < 4096
    }

    fn is_uint20(value: i32) -> bool {
        value >= 0 && value < 1048576
    }

    fn is_int6(value: i8) -> bool {
        value >= -32 && value < 32
    }

    fn is_int8(value: i8) -> bool {
        true
    }

    fn is_int12(value: i16) -> bool {
        value >= -2048 && value < 2048
    }

    fn is_int13(value: i16) -> bool {
        value >= -4096 && value < 4096
    }

    fn is_int20(value: i32) -> bool {
        value >= -524288 && value < 524288
    }

    fn is_int21(value: i32) -> bool {
        value >= -1048576 && value < 1048576
    }

    pub fn gen_instr_r(
        &mut self,
        funct7: u8,
        funct3: u8,
        opcode: BaseOpcode,
        rd: Register,
        rs1: Register,
        rs2: Register,
    ) {
        debug_assert!(
            Self::is_uint7(funct7)
                && Self::is_uint3(funct3)
                && rd.is_valid()
                && rs1.is_valid()
                && rs2.is_valid()
        );
        let instr = opcode as u32
            | (rd.code() << kRdShift)
            | ((funct3 as u32) << kFunct3Shift)
            | (rs1.code() << kRs1Shift)
            | (rs2.code() << kRs2Shift)
            | ((funct7 as u32) << kFunct7Shift);
        self.emit(instr);
    }

    pub fn gen_instr_r_fpu(
        &mut self,
        funct7: u8,
        funct3: u8,
        opcode: BaseOpcode,
        rd: FPURegister,
        rs1: FPURegister,
        rs2: FPURegister,
    ) {
        debug_assert!(
            Self::is_uint7(funct7)
                && Self::is_uint3(funct3)
                && rd.is_valid()
                && rs1.is_valid()
                && rs2.is_valid()
        );
        let instr = opcode as u32
            | (rd.code() << kRdShift)
            | ((funct3 as u32) << kFunct3Shift)
            | (rs1.code() << kRs1Shift)
            | (rs2.code() << kRs2Shift)
            | ((funct7 as u32) << kFunct7Shift);
        self.emit(instr);
    }

    pub fn gen_instr_r_fpu_r(
        &mut self,
        funct7: u8,
        funct3: u8,
        opcode: BaseOpcode,
        rd: Register,
        rs1: FPURegister,
        rs2: Register,
    ) {
        debug_assert!(
            Self::is_uint7(funct7)
                && Self::is_uint3(funct3)
                && rd.is_valid()
                && rs1.is_valid()
                && rs2.is_valid()
        );
        let instr = opcode as u32
            | (rd.code() << kRdShift)
            | ((funct3 as u32) << kFunct3Shift)
            | (rs1.code() << kRs1Shift)
            | (rs2.code() << kRs2Shift)
            | ((funct7 as u32) << kFunct7Shift);
        self.emit(instr);
    }

    pub fn gen_instr_r_r_fpu(
        &mut self,
        funct7: u8,
        funct3: u8,
        opcode: BaseOpcode,
        rd: FPURegister,
        rs1: Register,
        rs2: Register,
    ) {
        debug_assert!(
            Self::is_uint7(funct7)
                && Self::is_uint3(funct3)
                && rd.is_valid()
                && rs1.is_valid()
                && rs2.is_valid()
        );
        let instr = opcode as u32
            | (rd.code() << kRdShift)
            | ((funct3 as u32) << kFunct3Shift)
            | (rs1.code() << kRs1Shift)
            | (rs2.code() << kRs2Shift)
            | ((funct7 as u32) << kFunct7Shift);
        self.emit(instr);
    }

    pub fn gen_instr_r_fpu_r_fpu(
        &mut self,
        funct7: u8,
        funct3: u8,
        opcode: BaseOpcode,
        rd: FPURegister,
        rs1: FPURegister,
        rs2: Register,
    ) {
        debug_assert!(
            Self::is_uint7(funct7)
                && Self::is_uint3(funct3)
                && rd.is_valid()
                && rs1.is_valid()
                && rs2.is_valid()
        );
        let instr = opcode as u32
            | (rd.code() << kRdShift)
            | ((funct3 as u32) << kFunct3Shift)
            | (rs1.code() << kRs1Shift)
            | (rs2.code() << kRs2Shift)
            | ((funct7 as u32) << kFunct7Shift);
        self.emit(instr);
    }

    pub fn gen_instr_r_r_fpu_fpu(
        &mut self,
        funct7: u8,
        funct3: u8,
        opcode: BaseOpcode,
        rd: Register,
        rs1: FPURegister,
        rs2: FPURegister,
    ) {
        debug_assert!(
            Self::is_uint7(funct7)
                && Self::is_uint3(funct3)
                && rd.is_valid()
                && rs1.is_valid()
                && rs2.is_valid()
        );
        let instr = opcode as u32
            | (rd.code() << kRdShift)
            | ((funct3 as u32) << kFunct3Shift)
            | (rs1.code() << kRs1Shift)
            | (rs2.code() << kRs2Shift)
            | ((funct7 as u32) << kFunct7Shift);
        self.emit(instr);
    }

    pub fn gen_instr_r4(
        &mut self,
        funct2: u8,
        opcode: BaseOpcode,
        rd: Register,
        rs1: Register,
        rs2: Register,
        rs3: Register,
        frm: FPURoundingMode,
    ) {
        debug_assert!(
            Self::is_uint2(funct2)
                && rd.is_valid()
                && rs1.is_valid()
                && rs2.is_valid()
                && rs3.is_valid()
                && Self::is_uint3(frm.code() as u8)
        );
        let instr = opcode as u32
            | (rd.code() << kRdShift)
            | ((frm.code() as u32) << kFunct3Shift)
            | (rs1.code() << kRs1Shift)
            | (rs2.code() << kRs2Shift)
            | ((funct2 as u32) << kFunct2Shift)
            | (rs3.code() << kRs3Shift);
        self.emit(instr);
    }

    pub fn gen_instr_r4_fpu(
        &mut self,
        funct2: u8,
        opcode: BaseOpcode,
        rd: FPURegister,
        rs1: FPURegister,
        rs2: FPURegister,
        rs3: FPURegister,
        frm: FPURoundingMode,
    ) {
        debug_assert!(
            Self::is_uint2(funct2)
                && rd.is_valid()
                && rs1.is_valid()
                && rs2.is_valid()
                && rs3.is_valid()
                && Self::is_uint3(frm.code() as u8)
        );
        let instr = opcode as u32
            | (rd.code() << kRdShift)
            | ((frm.code() as u32) << kFunct3Shift)
            | (rs1.code() << kRs1Shift)
            | (rs2.code() << kRs2Shift)
            | ((funct2 as u32) << kFunct2Shift)
            | (rs3.code() << kRs3Shift);
        self.emit(instr);
    }

    pub fn gen_instr_r_atomic(
        &mut self,
        funct5: u8,
        aq: bool,
        rl: bool,
        funct3: u8,
        rd: Register,
        rs1: Register,
        rs2: Register,
    ) {
        debug_assert!(
            Self::is_uint5(funct5)
                && Self::is_uint3(funct3)
                && rd.is_valid()
                && rs1.is_valid()
                && rs2.is_valid()
        );
        let instr = BaseOpcode::AMO as u32
            | (rd.code() << kRdShift)
            | ((funct3 as u32) << kFunct3Shift)
            | (rs1.code() << kRs1Shift)
            | (rs2.code() << kRs2Shift)
            | ((rl as u32) << kRlShift)
            | ((aq as u32) << kAqShift)
            | ((funct5 as u32) << kFunct5Shift);
        self.emit(instr);
    }

    pub fn gen_instr_r_frm(
        &mut self,
        funct7: u8,
        opcode: BaseOpcode,
        rd: Register,
        rs1: Register,
        rs2: Register,
        frm: FPURoundingMode,
    ) {
        debug_assert!(
            rd.is_valid()
                && rs1.is_valid()
                && rs2.is_valid()
                && Self::is_uint3(frm.code() as u8)
        );
        let instr = opcode as u32
            | (rd.code() << kRdShift)
            | ((frm.code() as u32) << kFunct3Shift)
            | (rs1.code() << kRs1Shift)
            | (rs2.code() << kRs2Shift)
            | ((funct7 as u32) << kFunct7Shift);
        self.emit(instr);
    }

    pub fn gen_instr_i(
        &mut self,
        funct3: u8,
        opcode: BaseOpcode,
        rd: Register,
        rs1: Register,
        imm12: i16,
    ) {
        debug_assert!(
            Self::is_uint3(funct3)
                && rd.is_valid()
                && rs1.is_valid()
                && (Self::is_uint12(imm12) || Self::is_int12(imm12))
        );
        let instr = opcode as u32
            | (rd.code() << kRdShift)
            | ((funct3 as u32) << kFunct3Shift)
            | (rs1.code() << kRs1Shift)
            | ((imm12 as i32) << kImm12Shift) as u32;
        self.emit(instr);
    }

    pub fn gen_instr_i_fpu(
        &mut self,
        funct3: u8,
        opcode: BaseOpcode,
        rd: FPURegister,
        rs1: Register,
        imm12: i16,
    ) {
        debug_assert!(
            Self::is_uint3(funct3)
                && rd.is_valid()
                && rs1.is_valid()
                && (Self::is_uint12(imm12) || Self::is_int12(imm12))
        );
        let instr = opcode as u32
            | (rd.code() << kRdShift)
            | ((funct3 as u32) << kFunct3Shift)
            | (rs1.code() << kRs1Shift)
            | ((imm12 as i32) << kImm12Shift) as u32;
        self.emit(instr);
    }

    pub fn gen_instr_i_shift(
        &mut self,
        funct6: u8,
        funct3: u8,
        opcode: BaseOpcode,
        rd: Register,
        rs1: Register,
        shamt: u8,
    ) {
        debug_assert!(
            Self::is_uint3(funct3) && rd.is_valid() && rs1.is_valid() && Self::is_uint6(shamt)
        );
        let instr = opcode as u32
            | (rd.code() << kRdShift)
            | ((funct3 as u32) << kFunct3Shift)
            | (rs1.code() << kRs1Shift)
            | ((shamt as u32) << kShamtShift)
            | ((funct6 as u32) << kFunct6Shift);
        self.emit(instr);
    }

    pub fn gen_instr_i_shift_w(
        &mut self,
        funct7: u8,
        funct3: u8,
        opcode: BaseOpcode,
        rd: Register,
        rs1: Register,
        shamt: u8,
    ) {
        debug_assert!(
            Self::is_uint3(funct3) && rd.is_valid() && rs1.is_valid() && Self::is_uint5(shamt)
        );
        let instr = opcode as u32
            | (rd.code() << kRdShift)
            | ((funct3 as u32) << kFunct3Shift)
            | (rs1.code() << kRs1Shift)
            | ((shamt as u32) << kShamtWShift)
            | ((funct7 as u32) << kFunct7Shift);
        self.emit(instr);
    }

    pub fn gen_instr_s(
        &mut self,
        funct3: u8,
        opcode: BaseOpcode,
        rs1: Register,
        rs2: Register,
        imm12: i16,
    ) {
        debug_assert!(
            Self::is_uint3(funct3) && rs1.is_valid() && rs2.is_valid() && Self::is_int12(imm12)
        );
        let instr = opcode as u32
            | (((imm12 & 0x1f) as i32) << 7) as u32
            | ((funct3 as u32) << kFunct3Shift)
            | (rs1.code() << kRs1Shift)
            | (rs2.code() << kRs2Shift)
            | (((imm12 & 0xfe0) as i32) << 20) as u32;
        self.emit(instr);
    }

    pub fn gen_instr_s_fpu(
        &mut self,
        funct3: u8,
        opcode: BaseOpcode,
        rs1: Register,
        rs2: FPURegister,
        imm12: i16,
    ) {
        debug_assert!(
            Self::is_uint3(funct3) && rs1.is_valid() && rs2.is_valid() && Self::is_int12(imm12)
        );
        let instr = opcode as u32
            | (((imm12 & 0x1f) as i32) << 7) as u32
            | ((funct3 as u32) << kFunct3Shift)
            | (rs1.code() << kRs1Shift)
            | (rs2.code() << kRs2Shift)
            | (((imm12 & 0xfe0) as i32) << 20) as u32;
        self.emit(instr);
    }

    pub fn gen_instr_b(
        &mut self,
        funct3: u8,
        opcode: BaseOpcode,
        rs1: Register,
        rs2: Register,
        imm13: i16,
    ) {
        debug_assert!(
            Self::is_uint3(funct3) && rs1.is_valid() && rs2.is_valid() && Self::is_int13(imm13) && ((imm13 & 1) == 0)
        );
        let instr = opcode as u32
            | (((imm13 & 0x800) as i32 >> 4) as u32)
            | (((imm13 & 0x1e) as i32 << 7) as u32)
            | ((funct3 as u32) << kFunct3Shift)
            | (rs1.code() << kRs1Shift)
            | (rs2.code() << kRs2Shift)
            | (((imm13 & 0x7e0) as i32 << 20) as u32)
            | (((imm13 & 0x1000) as i32 << 19) as u32);
        self.emit(instr);
    }

    pub fn gen_instr_u(&mut self, opcode: BaseOpcode, rd: Register, imm20: i32) {
        debug_assert!(rd.is_valid() && (Self::is_int20(imm20) || Self::is_uint20(imm20)));
        let instr = opcode as u32 | (rd.code() << kRdShift) | ((imm20 as u32) << kImm20Shift);
        self.emit(instr);
    }

    pub fn gen_instr_j(&mut self, opcode: BaseOpcode, rd: Register, imm21: i32) {
        debug_assert!(rd.is_valid() && Self::is_int21(imm21) && ((imm21 & 1) == 0));
        let instr = opcode as u32
            | (rd.code() << kRdShift)
            | ((imm21 & 0xff000) as u32)
            | (((imm21 & 0x800) as i32 << 9) as u32)
            | (((imm21 & 0x7fe) as i32 << 20) as u32)
            | (((imm21 & 0x100000) as i32 << 11) as u32);
        self.emit(instr);
    }

    pub fn gen_instr_cr(&mut self, funct4: u8, opcode: BaseOpcode, rd: Register, rs2: Register) {
        debug_assert!(Self::is_uint4(funct4) && rd.is_valid() && rs2.is_valid());
        let instr = opcode as u16
            | ((rs2.code() as u16) << kRvcRs2Shift)
            | ((rd.code() as u16) << kRvcRdShift)
            | ((funct4 as u16) << kRvcFunct4Shift);
        self.emit(instr);
    }

    pub fn gen_instr_ca(
        &mut self,
        funct6: u8,
        opcode: BaseOpcode,
        rd: Register,
        funct: u8,
        rs2: Register,
    ) {
        debug_assert!(
            Self::is_uint6(funct6) && rd.is_valid() && rs2.is_valid() && Self::is_uint2(funct)
        );
        let instr = opcode as u16
            | (((rs2.code() & 0x7) as u16) << kRvcRs2sShift)
            | (((rd.code() & 0x7) as u16) << kRvcRs1sShift)
            | ((funct6 as u16) << kRvcFunct6Shift)
            | ((funct as u16) << kRvcFunct2Shift);
        self.emit(instr);
    }

    pub fn gen_instr_ci(&mut self, funct3: u8, opcode: BaseOpcode, rd: Register, imm6: i8) {
        debug_assert!(Self::is_uint3(funct3) && rd.is_valid() && Self::is_int6(imm6));
        let instr = opcode as u16
            | (((imm6 & 0x1f) as u16) << 2)
            | ((rd.code() as u16) << kRvcRdShift)
            | (((imm6 & 0x20) as u16) << 7)
            | ((funct3 as u16) << kRvcFunct3Shift);
        self.emit(instr);
    }

    pub fn gen_instr_ciu(&mut self, funct3: u8, opcode: BaseOpcode, rd: Register, uimm6: u8) {
        debug_assert!(Self::is_uint3(funct3) && rd.is_valid() && Self::is_uint6(uimm6));
        let instr = opcode as u16
            | (((uimm6 & 0x1f) as u16) << 2)
            | ((rd.code() as u16) << kRvcRdShift)
            | (((uimm6 & 0x20) as u16) << 7)
            | ((funct3 as u16) << kRvcFunct3Shift);
        self.emit(instr);
    }

    pub fn gen_instr_ciu_fpu(&mut self, funct3: u8, opcode: BaseOpcode, rd: FPURegister, uimm6: u8) {
        debug_assert!(Self::is_uint3(funct3) && rd.is_valid() && Self::is_uint6(uimm6));
        let instr = opcode as u16
            | (((uimm6 & 0x1f) as u16) << 2)
            | ((rd.code() as u16) << kRvcRdShift)
            | (((uimm6 & 0x20) as u16) << 7)
            | ((funct3 as u16) << kRvcFunct3Shift);
        self.emit(instr);
    }

    pub fn gen_instr_ciw(&mut self, funct3: u8, opcode: BaseOpcode, rd: Register, uimm8: u8) {
        debug_assert!(Self::is_uint3(funct3) && rd.is_valid() && Self::is_uint8(uimm8));
        let instr = opcode as u16
            | ((uimm8 as u16) << 5)
            | (((rd.code() & 0x7) as u16) << kRvcRs2sShift)
            | ((funct3 as u16) << kRvcFunct3Shift);
        self.emit(instr);
    }

    pub fn gen_instr_css(&mut self, funct3: u8, opcode: BaseOpcode, rs2: Register, uimm6: u8) {
        debug_assert!(Self::is_uint3(funct3) && rs2.is_valid() && Self::is_uint6(uimm6));
        let instr = opcode as u16
            | ((uimm6 as u16) << 7)
            | ((rs2.code() as u16) << kRvcRs2Shift)
            | ((funct3 as u16) << kRvcFunct3Shift);
        self.emit(instr);
    }

    pub fn gen_instr_css_fpu(&mut self, funct3: u8, opcode: BaseOpcode, rs2: FPURegister, uimm6: u8) {
        debug_assert!(Self::is_uint3(funct3) && rs2.is_valid() && Self::is_uint6(uimm6));