// Copyright 2013 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(non_snake_case)]
#![allow(dead_code)]

mod base {
    pub mod bits {
        pub fn count_trailing_zeros(x: u64) -> usize {
            x.trailing_zeros() as usize
        }

        pub fn is_power_of_two(x: u64) -> bool {
            x != 0 && (x & (x - 1)) == 0
        }
    }
}

mod codegen {
    pub mod arm64 {
        pub mod assembler_arm64_inl;
        pub mod assembler_arm64;
    }
    pub mod macro_assembler;
}

mod common {
    pub mod globals;
}

mod execution {
    pub mod isolate_data;
}

use std::convert::TryInto;

const kHeapObjectTag: i32 = 0; // Replace with actual value
const kSystemPointerSize: i32 = 8; // Replace with actual value
const kSmiShift: u32 = 1; // Replace with actual value
const kSmiTagSize: i32 = 1;
const kSmiTag: i32 = 0;
const kSmiValueSize: u32 = 31;
const kBitsPerByte: i32 = 8;
const kStackPageSize: i64 = 4096;
const kXRegSize: i64 = 8;

const COMPRESS_POINTERS_BOOL: bool = false; // Replace with actual value

// Dummy definitions for types and enums used in the C++ code.
// These need to be replaced with actual Rust implementations.

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Register(u16);
impl Register {
    pub fn is_zero(&self) -> bool {
        self.0 == 31
    }
    pub fn w(&self) -> Self {
        Register(self.0)
    }
    pub fn is_sp(&self) -> bool {
        false
    }
    pub fn size_in_bits(&self) -> i32 {
        64
    }
    pub fn is_64bits(&self) -> bool {
      true
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct VRegister(u16);
impl VRegister {
    pub fn d(&self) -> Self {
        VRegister(self.0)
    }
    pub fn is_1s(&self) -> bool {
        false
    }
        pub fn is_2s(&self) -> bool {
        false
    }
        pub fn is_4s(&self) -> bool {
        false
    }
            pub fn is_1d(&self) -> bool {
        false
    }
        pub fn is_2d(&self) -> bool {
        false
    }

    pub fn is_scalar(&self) -> bool {
        false
    }

    pub fn is_64bits(&self) -> bool {
      true
    }
}

#[derive(Debug, Copy, Clone)]
pub struct MemOperand {
    base: Register,
    offset: i32,
    addrmode: i32,
    shift_amount: i32
}

impl MemOperand {
    pub fn new(base: Register, offset: i32) -> Self {
        MemOperand { base, offset, addrmode: 0, shift_amount:0 }
    }

    pub fn base(&self) -> Register {
        self.base
    }

    pub fn offset(&self) -> i32 {
        self.offset
    }

    pub fn addrmode(&self) -> i32 {
        self.addrmode
    }
    pub fn shift_amount(&self) -> i32 {
      self.shift_amount
    }

    pub fn is_immediate_offset(&self) -> bool {
      true
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Operand {
    immediate: i64,
    is_immediate: bool,
    reg: Register,
    is_shifted_register: bool,
    shift_amount: i32,
}

impl Operand {
    pub fn new(immediate: i64) -> Self {
        Operand { immediate, is_immediate: true, reg: Register(0), is_shifted_register: false, shift_amount: 0 }
    }

    pub fn immediate_value(&self) -> i64 {
        self.immediate
    }

    pub fn is_immediate(&self) -> bool {
        self.is_immediate
    }

    pub fn reg(&self) -> Register {
        self.reg
    }

    pub fn is_shifted_register(&self) -> bool {
        self.is_shifted_register
    }

    pub fn shift_amount(&self) -> i32 {
        self.shift_amount
    }

    pub fn to_w(&self) -> Self {
        *self
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Condition {
    Eq,
    Ne,
    Lt,
    Ge,
    Lo,
    Hi,
    Al,
    Nv,
}

#[derive(Debug, Copy, Clone)]
pub enum StatusFlags {}

#[derive(Debug, Copy, Clone)]
pub enum BarrierDomain {}

#[derive(Debug, Copy, Clone)]
pub enum BarrierType {}

#[derive(Debug, Copy, Clone)]
pub enum SystemHint {}

#[derive(Debug, Copy, Clone)]
pub enum SystemRegister {}

#[derive(Debug, Copy, Clone)]
pub enum BranchTargetIdentifier {}

impl BranchTargetIdentifier {
    const K_NONE: Self = BranchTargetIdentifier {};
    const K_PACIBSP: Self = BranchTargetIdentifier {};
    const K_BTI_JUMP: Self = BranchTargetIdentifier {};
    const K_BTI_CALL: Self = BranchTargetIdentifier {};
    const K_BTI_JUMP_CALL: Self = BranchTargetIdentifier {};
}

#[derive(Debug, Copy, Clone)]
pub enum LeaveFlags {}

#[derive(Debug, Copy, Clone)]
pub enum SetFlags {}

#[derive(Debug, Copy, Clone)]
pub enum Instr {}

#[derive(Debug, Copy, Clone)]
pub enum AbortReason {}

#[derive(Debug, Copy, Clone)]
pub struct CPURegister {}

#[derive(Debug, Copy, Clone)]
pub struct Label {}

#[derive(Debug, Copy, Clone)]
pub struct UseScratchRegisterScope<'a> {
  assembler: &'a mut MacroAssembler
}

impl<'a> UseScratchRegisterScope<'a> {
  fn acquire_x(&mut self) -> Register {
    Register(0) //Placeholder
  }

    fn acquire_w(&mut self) -> Register {
    Register(0) //Placeholder
  }

  fn acquire_same_size_as(&mut self, reg: VRegister) -> VRegister {
    VRegister(0) //Placeholder
  }
}

macro_rules! LS_MACRO_LIST {
    ($define_function:ident) => {
        $define_function!(Ldr, Register, rt, addr, ldr);
        $define_function!(Ldr, VRegister, rt, addr, ldr);
        $define_function!(Str, Register, rt, addr, str);
        $define_function!(Str, VRegister, rt, addr, str);
    };
}

macro_rules! LSPAIR_MACRO_LIST {
    ($define_function:ident) => {
        // Example, add more as needed
    };
}

macro_rules! LDA_STL_MACRO_LIST {
    ($define_function:ident) => {
        // Example, add more as needed
    };
}

macro_rules! STLX_MACRO_LIST {
    ($define_function:ident) => {
        // Example, add more as needed
    };
}

macro_rules! CAS_SINGLE_MACRO_LIST {
    ($define_function:ident) => {
        // Example, add more as needed
    };
}

macro_rules! CAS_PAIR_MACRO_LIST {
    ($define_function:ident) => {
        // Example, add more as needed
    };
}

macro_rules! ATOMIC_MEMORY_SIMPLE_MACRO_LIST {
    ($macro_modes:ident, $define_function:ident, $prefix:ident, $op:ident) => {
        $macro_modes!($define_function, $prefix, $op);
    };
}

macro_rules! ATOMIC_MEMORY_LOAD_MACRO_MODES {
    ($define_function:ident, $prefix:ident, $op:ident) => {
        $define_function!(Lda, lda);
    };
}

macro_rules! ATOMIC_MEMORY_STORE_MACRO_MODES {
    ($define_function:ident, $prefix:ident, $op:ident) => {
        $define_function!(Stl, stl);
    };
}

// Dummy definitions for functions used in the C++ code.
// These need to be replaced with actual Rust implementations.

fn is_imm_add_sub(imm: i64) -> bool {
    true
}

fn mask_to_bit(mask: u64) -> i32 {
  0
}

fn count_set_bits(pattern: u64, bits: i32) -> i32 {
  0
}

fn round_up(x: i64, multiple: i64) -> i64 {
    (x + multiple - 1) / multiple * multiple
}

// Define a struct to hold the state of the MacroAssembler.
pub struct MacroAssembler {
    // Add fields to represent the state of the assembler, such as:
    // - Current position in the code buffer
    // - List of labels and their corresponding positions
    // - Any other relevant state

    isolate: Isolate
}

struct Isolate {}

impl MacroAssembler {
    pub fn new(isolate: Isolate) -> Self {
        MacroAssembler {
          isolate: isolate
        }
    }

    fn allow_macro_instructions(&self) -> bool {
        true
    }

    fn appropriate_zero_reg_for(&self, rn: Register) -> Register {
        Register(31)
    }

    // Implementation of the FieldMemOperand function.
    pub fn field_mem_operand(object: Register, offset: i32) -> MemOperand {
        MemOperand::new(object, offset - kHeapObjectTag)
    }

    // Implementation of the ExitFrameStackSlotOperand function.
    pub fn exit_frame_stack_slot_operand(offset: i32) -> MemOperand {
        static SPOFFSET: i32 = 1 * kSystemPointerSize;
        MemOperand::new(sp, SPOFFSET + offset)
    }

    // Implementation of the ExitFrameCallerStackSlotOperand function.
    pub fn exit_frame_caller_stack_slot_operand(index: i32) -> MemOperand {
        const K_FIXED_SLOT_COUNT_ABOVE_FP: i32 = 0; // Replace with actual value
        MemOperand::new(fp, (K_FIXED_SLOT_COUNT_ABOVE_FP + index) * kSystemPointerSize)
    }

    // Implementation of the And function.
    pub fn and(&mut self, rd: &Register, rn: &Register, operand: &Operand) {
        assert!(self.allow_macro_instructions());
        assert!(!rd.is_zero());
        self.logical_macro(rd, rn, operand, LogicalOp::AND);
    }

    pub fn ands(&mut self, rd: &Register, rn: &Register, operand: &Operand) {
        assert!(self.allow_macro_instructions());
        assert!(!rd.is_zero());
        self.logical_macro(rd, rn, operand, LogicalOp::ANDS);
    }

    pub fn tst(&mut self, rn: &Register, operand: &Operand) {
        assert!(self.allow_macro_instructions());
        let zr = self.appropriate_zero_reg_for(*rn);
        self.logical_macro(&zr, rn, operand, LogicalOp::ANDS);
    }

    pub fn bic(&mut self, rd: &Register, rn: &Register, operand: &Operand) {
        assert!(self.allow_macro_instructions());
        assert!(!rd.is_zero());
        self.logical_macro(rd, rn, operand, LogicalOp::BIC);
    }

    pub fn bics(&mut self, rd: &Register, rn: &Register, operand: &Operand) {
        assert!(self.allow_macro_instructions());
        assert!(!rd.is_zero());
        self.logical_macro(rd, rn, operand, LogicalOp::BICS);
    }

    pub fn orr(&mut self, rd: &Register, rn: &Register, operand: &Operand) {
        assert!(self.allow_macro_instructions());
        assert!(!rd.is_zero());
        self.logical_macro(rd, rn, operand, LogicalOp::ORR);
    }

    pub fn orn(&mut self, rd: &Register, rn: &Register, operand: &Operand) {
        assert!(self.allow_macro_instructions());
        assert!(!rd.is_zero());
        self.logical_macro(rd, rn, operand, LogicalOp::ORN);
    }

    pub fn eor(&mut self, rd: &Register, rn: &Register, operand: &Operand) {
        assert!(self.allow_macro_instructions());
        assert!(!rd.is_zero());
        self.logical_macro(rd, rn, operand, LogicalOp::EOR);
    }

    pub fn eon(&mut self, rd: &Register, rn: &Register, operand: &Operand) {
        assert!(self.allow_macro_instructions());
        assert!(!rd.is_zero());
        self.logical_macro(rd, rn, operand, LogicalOp::EON);
    }

    pub fn ccmp(&mut self, rn: &Register, operand: &Operand, nzcv: StatusFlags, cond: Condition) {
      assert!(self.allow_macro_instructions());
      if operand.is_immediate() && (operand.immediate_value() < 0) {
        self.conditional_compare_macro(rn, -operand.immediate_value(), nzcv, cond, ConditionalCompareOp::CCMN);
      } else {
        self.conditional_compare_macro(rn, operand, nzcv, cond, ConditionalCompareOp::CCMP);
      }
    }

    pub fn ccmp_tagged(&mut self, rn: &Register, operand: &Operand, nzcv: StatusFlags, cond: Condition) {
      if COMPRESS_POINTERS_BOOL {
        self.ccmp(&rn.w(), &operand.to_w(), nzcv, cond);
      } else {
        self.ccmp(rn, operand, nzcv, cond);
      }
    }

    pub fn ccmn(&mut self, rn: &Register, operand: &Operand, nzcv: StatusFlags, cond: Condition) {
      assert!(self.allow_macro_instructions());
      if operand.is_immediate() && (operand.immediate_value() < 0) {
        self.conditional_compare_macro(rn, -operand.immediate_value(), nzcv, cond, ConditionalCompareOp::CCMP);
      } else {
        self.conditional_compare_macro(rn, operand, nzcv, cond, ConditionalCompareOp::CCMN);
      }
    }

    pub fn add(&mut self, rd: &Register, rn: &Register, operand: &Operand) {
        assert!(self.allow_macro_instructions());
        if operand.is_immediate() {
            let imm = operand.immediate_value();
            if (imm > 0) && is_imm_add_sub(imm) {
                self.data_proc_immediate(rd, rn, imm.try_into().unwrap(), DataProcOp::ADD);
                return;
            } else if (imm < 0) && is_imm_add_sub(-imm) {
                self.data_proc_immediate(rd, rn, (-imm).try_into().unwrap(), DataProcOp::SUB);
                return;
            }
        } else if operand.is_shifted_register() && (operand.shift_amount() == 0) {
            if !rd.is_sp() && !rn.is_sp() && !operand.reg().is_sp() && !operand.reg().is_zero() {
                self.data_proc_plain_register(rd, rn, operand.reg(), DataProcOp::ADD);
                return;
            }
        }
        self.add_sub_macro(rd, rn, operand, LeaveFlags {}, AddSubOp::ADD);
    }

    pub fn adds(&mut self, rd: &Register, rn: &Register, operand: &Operand) {
      assert!(self.allow_macro_instructions());
      if operand.is_immediate() && (operand.immediate_value() < 0) &&
         is_imm_add_sub(-operand.immediate_value()) {
          self.add_sub_macro(rd, rn, -operand.immediate_value(), SetFlags {}, AddSubOp::SUB);
      } else {
          self.add_sub_macro(rd, rn, operand, SetFlags {}, AddSubOp::ADD);
      }
    }

    pub fn sub(&mut self, rd: &Register, rn: &Register, operand: &Operand) {
      assert!(self.allow_macro_instructions());
      if operand.is_immediate() {
          let imm = operand.immediate_value();
          if (imm > 0) && is_imm_add_sub(imm) {
              self.data_proc_immediate(rd, rn, imm.try_into().unwrap(), DataProcOp::SUB);
              return;
          } else if (imm < 0) && is_imm_add_sub(-imm) {
              self.data_proc_immediate(rd, rn, (-imm).try_into().unwrap(), DataProcOp::ADD);
              return;
          }
      } else if operand.is_shifted_register() && (operand.shift_amount() == 0) {
          if !rd.is_sp() && !rn.is_sp() && !operand.reg().is_sp() && !operand.reg().is_zero() {
              self.data_proc_plain_register(rd, rn, operand.reg(), DataProcOp::SUB);
              return;
          }
      }
      self.add_sub_macro(rd, rn, operand, LeaveFlags {}, AddSubOp::SUB);
    }

    pub fn subs(&mut self, rd: &Register, rn: &Register, operand: &Operand) {
      assert!(self.allow_macro_instructions());
      if operand.is_immediate() && (operand.immediate_value() < 0) &&
         is_imm_add_sub(-operand.immediate_value()) {
          self.add_sub_macro(rd, rn, -operand.immediate_value(), SetFlags {}, AddSubOp::ADD);
      } else {
          self.add_sub_macro(rd, rn, operand, SetFlags {}, AddSubOp::SUB);
      }
    }

    pub fn cmn(&mut self, rn: &Register, operand: &Operand) {
        assert!(self.allow_macro_instructions());
        let zr = self.appropriate_zero_reg_for(*rn);
        self.adds(&zr, rn, operand);
    }

    pub fn cmp(&mut self, rn: &Register, operand: &Operand) {
      assert!(self.allow_macro_instructions());
      if operand.is_shifted_register() && operand.shift_amount() == 0 {
        if !rn.is_sp() && !operand.reg().is_sp() {
          self.cmp_plain_register(rn, operand.reg());
          return;
        }
      }
      let zr = self.appropriate_zero_reg_for(*rn);
      self.subs(&zr, rn, operand);
    }

    pub fn cmp_tagged(&mut self, rn: &Register, operand: &Operand) {
      if COMPRESS_POINTERS_BOOL {
        self.cmp(&rn.w(), &operand.to_w());
      } else {
        self.cmp(rn, operand);
      }
    }

    pub fn neg(&mut self, rd: &Register, operand: &Operand) {
      assert!(self.allow_macro_instructions());
      assert!(!rd.is_zero());
      if operand.is_immediate() {
        self.mov(rd, -operand.immediate_value());
      } else {
        let zr = self.appropriate_zero_reg_for(*rd);
        self.sub(rd, &zr, operand);
      }
    }

    pub fn negs(&mut self, rd: &Register, operand: &Operand) {
      assert!(self.allow_macro_instructions());
      let zr = self.appropriate_zero_reg_for(*rd);
      self.subs(rd, &zr, operand);
    }

    pub fn adc(&mut self, rd: &Register, rn: &Register, operand: &Operand) {
      assert!(self.allow_macro_instructions());
      assert!(!rd.is_zero());
      self.add_sub_with_carry_macro(rd, rn, operand, LeaveFlags {}, AddSubWithCarryOp::ADC);
    }

    pub fn adcs(&mut self, rd: &Register, rn: &Register, operand: &Operand) {
      assert!(self.allow_macro_instructions());
      assert!(!rd.is_zero());
      self.add_sub_with_carry_macro(rd, rn, operand, SetFlags {}, AddSubWithCarryOp::ADC);
    }

    pub fn sbc(&mut self, rd: &Register, rn: &Register, operand: &Operand) {
      assert!(self.allow_macro_instructions());
      assert!(!rd.is_zero());
      self.add_sub_with_carry_macro(rd, rn, operand, LeaveFlags {}, AddSubWithCarryOp::SBC);
    }

    pub fn sbcs(&mut self, rd: &Register, rn: &Register, operand: &Operand) {
      assert!(self.allow_macro_instructions());
      assert!(!rd.is_zero());
      self.add_sub_with_carry_macro(rd, rn, operand, SetFlags {}, AddSubWithCarryOp::SBC);
    }

    pub fn ngc(&mut self, rd: &Register, operand: &Operand) {
      assert!(self.allow_macro_instructions());
      assert!(!rd.is_zero());
      let zr = self.appropriate_zero_reg_for(*rd);
      self.sbc(rd, &zr, operand);
    }

    pub fn ngcs(&mut self, rd: &Register, operand: &Operand) {
      assert!(self.allow_macro_instructions());
      assert!(!rd.is_zero());
      let zr = self.appropriate_zero_reg_for(*rd);
      self.sbcs(rd, &zr, operand);
    }

    pub fn mvn(&mut self, rd: &Register, imm: u64) {
      assert!(self.allow_macro_instructions());
      assert!(!rd.is_zero());
      self.mov(rd, !imm as i64);
    }

    fn check_veneer_pool(&mut self, b: bool, b2: bool) {}
    fn logical_macro(&mut self, rd: &Register, rn: &Register, operand: &Operand, op: LogicalOp) {}
    fn add_sub_macro(&mut self, rd: &Register, rn: &Register, operand: &Operand, flags: LeaveFlags, op: AddSubOp) {}
    fn conditional_compare_macro(&mut self, rn: &Register, operand: i64, nzcv: StatusFlags, cond: Condition, op: ConditionalCompareOp) {}
    fn add_sub_with_carry_macro(&mut self, rd: &Register, rn: &Register, operand: &Operand, flags: LeaveFlags, op: AddSubWithCarryOp) {}
    fn load_store_macro(&mut self, reg: Register, addr: MemOperand, op: LoadStoreOp) {}
    fn load_store_pair_macro(&mut self, reg: Register, reg2: Register, addr: MemOperand, op: LoadStorePairOp) {}
    fn pacibsp(&mut self) {}
    fn autibsp(&mut self) {}

    fn data_proc_immediate(&mut self, rd: &Register, rn: &Register, imm: i32, op: DataProcOp) {}
    fn data_proc_plain_register(&mut self, rd: &Register, rn: &Register, rm: Register, op: DataProcOp) {}
    fn cmp_plain_register(&mut self, rn: &Register, rm: Register) {}
    fn mov(&mut self, rd: &Register, imm: i64) {}
    fn ldr(&mut self, rt: &CPURegister, operand: &Operand) {}
    fn str(&mut self, reg: VRegister, mem_operand: MemOperand) {}
    fn str(&mut self, reg: Register, mem_operand: MemOperand) {}
    fn movk(&mut self, rd: &Register, imm: u64, shift: i32) {}
    fn mrs(&mut self, rt: &Register, sysreg: SystemRegister) {}
    fn msr(&mut self, sysreg: SystemRegister, rt: &Register) {}
    fn rbit(&mut self, rd: &Register, rn: &Register) {}
    fn ret(&mut self, xn: &Register) {}
    fn rev(&mut self, rd: &Register, rn: &Register) {}
    fn rev16(&mut self, rd: &Register, rn: &Register) {}
    fn rev32(&mut self, rd: &Register, rn: &Register) {}
    fn asr(&mut self, rd: &Register, rn: &Register, shift: u32) {}
    fn asrv(&mut self, rd: &Register, rn: &Register, rm: Register) {}
    fn b(&mut self, label: &Label) {}
    fn b( &mut self, label: &Label, cond: Condition) {}
    fn bfi(&mut self, rd: &Register, rn: &Register, lsb: u32, width: u32) {}
    fn bfxil(&mut self, rd: &Register, rn: &Register, lsb: u32, width: u32) {}
    fn bind(&mut self, label: &Label) {}
    fn bti(&mut self, id: BranchTargetIdentifier) {}
    fn bl(&mut self, label: &Label) {}
    fn blr(&mut self, xn: &Register) {}
    fn br(&mut self, xn: &Register) {}
    fn brk(&mut self, code: i32) {}
    fn cinc(&mut self, rd: &Register, rn: &Register, cond: Condition) {}
    fn cinv(&mut self, rd: &Register, rn: &Register, cond: Condition) {}
    fn cls(&mut self, rd: &Register, rn: &Register) {}
    fn clz(&mut self, rd: &Register, rn: &Register) {}
    fn cneg(&mut self, rd: &Register, rn: &Register, cond: Condition) {}
    fn csel(&mut self, rd: &Register, rn: Register, rd1: &Register, cond: Condition) {}
    fn csdb(&mut self) {}
    fn cset(&mut self, rd: &Register, cond: Condition) {}
    fn csetm(&mut self, rd: &Register, cond: Condition) {}
    fn csinc(&mut self, rd: &Register, rn: &Register, rm: &Register, cond: Condition) {}
    fn csinv(&mut self, rd: &Register, rn: &Register, rm: &Register, cond: Condition) {}
    fn csneg(&mut self, rd: &Register, rn: &Register, rm: &Register, cond: Condition) {}
    fn dmb(&mut self, domain: BarrierDomain, type_: BarrierType) {}
    fn dsb(&mut self, domain: BarrierDomain, type_: BarrierType) {}
    fn debug(&mut self, message: &str, code: u32, params: Instr) {}
    fn extr(&mut self, rd: &Register, rn: &Register, rm: &Register, lsb: u32) {}
    fn fabs(&mut self, fd: &VRegister, fn_: &VRegister) {}
    fn fadd(&mut self, fd: &VRegister, fn_: &VRegister, fm: &VRegister) {}
    fn fccmp(&mut self, fn_: &VRegister, fm: &VRegister, nzcv: StatusFlags, cond: Condition) {}
    fn fcmp(&mut self, fn_: &VRegister, fm: &VRegister) {}
    fn fcmp(&mut self, fn_: &VRegister, value: f64) {}
    fn fcsel(&mut self, fd: &VRegister, fn_: &VRegister, fm: &VRegister, cond: Condition) {}
    fn fcvt(&mut self, fd: &VRegister, fn_: &VRegister) {}
    fn fcvtas(&mut self, rd: &Register, fn_: &VRegister) {}
    fn fcvtau(&mut self, rd: &Register, fn_: &VRegister) {}
    fn fcvtms(&mut self, rd: &Register, fn_: &VRegister) {}
    fn fcvtmu(&mut self, rd: &Register, fn_: &VRegister) {}
    fn fcvtns(&mut self, rd: &Register, fn_: &VRegister) {}
    fn fcvtnu(&mut self, rd: &Register, fn_: &VRegister) {}
    fn fcvtzs(&mut self, rd: &Register, fn_: &VRegister) {}
    fn fcvtzu(&mut self, rd: &Register, fn_: &VRegister) {}
    fn fdiv(&mut self, fd: &VRegister, fn_: &VRegister, fm: &VRegister) {}
    fn fmadd(&mut self, fd: &VRegister, fn_: &VRegister, fm: &VRegister, fa: &VRegister) {}
    fn fmax(&mut self, fd: &VRegister, fn_: &VRegister, fm: &VRegister) {}
    fn fmaxnm(&mut self, fd: &VRegister, fn_: &VRegister, fm: &VRegister) {}
    fn fmin(&mut self, fd: &VRegister, fn_: &VRegister, fm: &VRegister) {}
    fn fminnm(&mut self, fd: &VRegister, fn_: &VRegister, fm: &VRegister) {}
    fn fmov(&mut self, fd: VRegister, fn_: VRegister) {}
    fn fmov(&mut self, fd: VRegister, rn: Register) {}
    fn fmov(&mut self, vd: VRegister, imm: f64) {}
    fn fmov(&mut self, vd: VRegister, imm: f32) {}
    fn fmov(&mut self, rd: &Register, fn_: &VRegister) {}
    fn fmsub(&mut self, fd: &VRegister, fn_: &VRegister, fm: &VRegister, fa: &VRegister) {}
    fn fmul(&mut self, fd: &VRegister, fn_: &VRegister, fm: &VRegister) {}
    fn fnmadd(&mut self, fd: &VRegister, fn_: &VRegister, fm: &VRegister, fa: &VRegister) {}
    fn fnmsub(&mut self, fd: &VRegister, fn_: &VRegister, fm: &VRegister, fa: &VRegister) {}
    fn fsub(&mut self, fd: &VRegister, fn_: &VRegister, fm: &VRegister) {}
    fn hint(&mut self, code: SystemHint) {}
    fn hlt(&mut self, code: i32) {}
    fn isb(&mut self) {}
    fn lsl(&mut self, rd: &Register, rn: &Register, shift: u32) {}
    fn lslv(&mut self, rd: &Register, rn: &Register, rm: Register) {}
    fn lsr(&mut self, rd: &Register, rn: &Register, shift: u32) {}
    fn lsrv(&mut self, rd: &Register, rn: &Register, rm: Register) {}
    fn madd(&mut self, rd: &Register, rn: &Register, rm: &Register, ra: &Register) {}
    fn mneg(&mut self, rd: &Register, rn: &Register, rm: &Register) {}
    fn msub(&mut self, rd: &Register, rn: &Register, rm: &Register, ra: &Register) {}
    fn mul(&mut self, rd: &Register, rn: &Register, rm: &Register) {}
    fn ror(&mut self, rd: &Register, rs: &Register, shift: u32) {}
    fn rorv(&mut self, rd: &Register, rn: &Register, rm: &Register) {}
    fn sbfx(&mut self, rd: &Register, rn: &Register, lsb: u32, width: u32) {}
    fn scvtf(&mut self, fd: &VRegister, rn: &Register, fbits: u32) {}
    fn sdiv(&mut self, rd: &Register, rn: &Register, rm: &Register) {}
    fn smaddl(&mut self, rd: &Register, rn: &Register, rm: &Register, ra: &Register) {}
    fn smsubl(&mut self, rd: &Register, rn: &Register, rm: &Register, ra: &Register) {}
    fn smull(&mut self, rd: &Register, rn: &Register, rm: &Register) {}
    fn smulh(&mut self, rd: &Register, rn: &Register, rm: &Register) {}
    fn umull(&mut self, rd: &Register, rn: &Register, rm: &Register) {}
    fn umulh(&mut self, rd: &Register, rn: &Register, rm: &Register) {}
    fn sxtb(&mut self, rd: &Register,