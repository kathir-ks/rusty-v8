// Copyright 2013 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::fmt;
use std::ops::{BitXor, Neg};

//use crate::base::bits; // Assuming 'src/base/bits.h' functionalities
//use crate::codegen::arm64::assembler_arm64; // Assuming 'src/codegen/arm64/assembler-arm64.h' functionalities
//use crate::codegen::bailout_reason; // Assuming 'src/codegen/bailout-reason.h' functionalities
//use crate::common::globals; // Assuming 'src/common/globals.h' functionalities
//use crate::objects::tagged_index; // Assuming 'src/objects/tagged-index.h' functionalities

// Simulator specific helpers.
#[cfg(all(feature = "use_simulator", debug_assertions))]
macro_rules! asm_location {
    ($message:expr) => {
        println!("LOCATION: {} {}", $message, line!());
    };
}

#[cfg(all(feature = "use_simulator", not(debug_assertions)))]
macro_rules! asm_location {
    ($message:expr) => {};
}

#[cfg(not(feature = "use_simulator"))]
macro_rules! asm_location {
    ($message:expr) => {};
}

#[cfg(all(feature = "use_simulator", debug_assertions))]
macro_rules! asm_location_in_assembler {
    ($message:expr) => {
        println!("LOCATION: {} {}", $message, line!());
    };
}

#[cfg(all(feature = "use_simulator", not(debug_assertions)))]
macro_rules! asm_location_in_assembler {
    ($message:expr) => {};
}

#[cfg(not(feature = "use_simulator"))]
macro_rules! asm_location_in_assembler {
    ($message:expr) => {};
}

mod wasm {
    pub struct JumpTableAssembler {} // Placeholder
}

macro_rules! ls_macro_list {
    ($V:ident) => {
        $V!(Ldrb, Register, rt, LDRB_w);
        $V!(Strb, Register, rt, STRB_w);
        $V!(Ldrsb, Register, rt, LDRSB_x); // Assuming rt.Is64Bits() ? LDRSB_x : LDRSB_w
        $V!(Ldrh, Register, rt, LDRH_w);
        $V!(Strh, Register, rt, STRH_w);
        $V!(Ldrsh, Register, rt, LDRSH_x); // Assuming rt.Is64Bits() ? LDRSH_x : LDRSH_w
        $V!(Ldr, CPURegister, rt, LoadOpFor(rt)); // Assuming LoadOpFor function exists
        $V!(Str, CPURegister, rt, StoreOpFor(rt)); // Assuming StoreOpFor function exists
        $V!(Ldrsw, Register, rt, LDRSW_x);
    };
}

macro_rules! lspair_macro_list {
    ($V:ident) => {
        $V!(Ldp, CPURegister, rt, rt2, LoadPairOpFor(rt, rt2)); // Assuming LoadPairOpFor function exists
        $V!(Stp, CPURegister, rt, rt2, StorePairOpFor(rt, rt2)); // Assuming StorePairOpFor function exists
        $V!(Ldpsw, CPURegister, rt, rt2, LDPSW_x);
    };
}

macro_rules! lda_stl_macro_list {
    ($V:ident) => {
        $V!(Ldarb, ldarb);
        $V!(Ldarh, ldarh);
        $V!(Ldar, ldar);
        $V!(Ldaxrb, ldaxrb);
        $V!(Ldaxrh, ldaxrh);
        $V!(Ldaxr, ldaxr);
        $V!(Stlrb, stlrb);
        $V!(Stlrh, stlrh);
        $V!(Stlr, stlr);
    };
}

macro_rules! stlx_macro_list {
    ($V:ident) => {
        $V!(Stlxrb, stlxrb);
        $V!(Stlxrh, stlxrh);
        $V!(Stlxr, stlxr);
    };
}

macro_rules! cas_single_macro_list {
    ($V:ident) => {
        $V!(Cas, cas);
        $V!(Casa, casa);
        $V!(Casl, casl);
        $V!(Casal, casal);
        $V!(Casb, casb);
        $V!(Casab, casab);
        $V!(Caslb, caslb);
        $V!(Casalb, casalb);
        $V!(Cash, cash);
        $V!(Casah, casah);
        $V!(Caslh, caslh);
        $V!(Casalh, casalh);
    };
}

macro_rules! cas_pair_macro_list {
    ($V:ident) => {
        $V!(Casp, casp);
        $V!(Caspa, caspa);
        $V!(Caspl, caspl);
        $V!(Caspal, caspal);
    };
}

macro_rules! atomic_memory_simple_macro_list {
    ($V:ident, $DEF:ident, $MASM_PRE:ident, $ASM_PRE:ident) => {
        $V!($DEF, $MASM_PRE##add, $ASM_PRE##add);
        $V!($DEF, $MASM_PRE##clr, $ASM_PRE##clr);
        $V!($DEF, $MASM_PRE##eor, $ASM_PRE##eor);
        $V!($DEF, $MASM_PRE##set, $ASM_PRE##set);
        $V!($DEF, $MASM_PRE##smax, $ASM_PRE##smax);
        $V!($DEF, $MASM_PRE##smin, $ASM_PRE##smin);
        $V!($DEF, $MASM_PRE##umax, $ASM_PRE##umax);
        $V!($DEF, $MASM_PRE##umin, $ASM_PRE##umin);
    };
}

macro_rules! atomic_memory_store_macro_modes {
    ($V:ident, $MASM:ident, $ASM:ident) => {
        $V!($MASM, $ASM);
        $V!($MASM##l, $ASM##l);
        $V!($MASM##b, $ASM##b);
        $V!($MASM##lb, $ASM##lb);
        $V!($MASM##h, $ASM##h);
        $V!($MASM##lh, $ASM##lh);
    };
}

macro_rules! atomic_memory_load_macro_modes {
    ($V:ident, $MASM:ident, $ASM:ident) => {
        atomic_memory_store_macro_modes!($V, $MASM, $ASM);
        $V!($MASM##a, $ASM##a);
        $V!($MASM##al, $ASM##al);
        $V!($MASM##ab, $ASM##ab);
        $V!($MASM##alb, $ASM##alb);
        $V!($MASM##ah, $ASM##ah);
        $V!($MASM##alh, $ASM##alh);
    };
}

// Placeholder for MemOperand
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemOperand {}

// Placeholder
fn FieldMemOperand(_object: Register, _offset: i32) -> MemOperand {
    MemOperand {}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BranchType {
    IntegerEq,
    IntegerNe,
    IntegerHs,
    IntegerLo,
    IntegerMi,
    IntegerPl,
    IntegerVs,
    IntegerVc,
    IntegerHi,
    IntegerLs,
    IntegerGe,
    IntegerLt,
    IntegerGt,
    IntegerLe,
    IntegerAl,
    IntegerNv,
    Always,
    Never,
    RegZero,
    RegNotZero,
    RegBitClear,
    RegBitSet,

    KBranchTypeFirstCondition,
    KBranchTypeLastCondition,
    KBranchTypeFirstUsingReg,
    KBranchTypeFirstUsingBit,
}

impl BranchType {
    pub fn invert(&self) -> BranchType {
        match self {
            BranchType::IntegerEq => BranchType::IntegerNe,
            BranchType::IntegerNe => BranchType::IntegerEq,
            BranchType::IntegerHs => BranchType::IntegerLo,
            BranchType::IntegerLo => BranchType::IntegerHs,
            BranchType::IntegerMi => BranchType::IntegerPl,
            BranchType::IntegerPl => BranchType::IntegerMi,
            BranchType::IntegerVs => BranchType::IntegerVc,
            BranchType::IntegerVc => BranchType::IntegerVs,
            BranchType::IntegerHi => BranchType::IntegerLs,
            BranchType::IntegerLs => BranchType::IntegerHi,
            BranchType::IntegerGe => BranchType::IntegerLt,
            BranchType::IntegerLt => BranchType::IntegerGe,
            BranchType::IntegerGt => BranchType::IntegerLe,
            BranchType::IntegerLe => BranchType::IntegerGt,
            BranchType::IntegerAl => BranchType::IntegerNv,
            BranchType::IntegerNv => BranchType::IntegerAl,
            BranchType::Always => BranchType::Never,
            BranchType::Never => BranchType::Always,
            BranchType::RegZero => BranchType::RegNotZero,
            BranchType::RegNotZero => BranchType::RegZero,
            BranchType::RegBitClear => BranchType::RegBitSet,
            BranchType::RegBitSet => BranchType::RegBitClear,
            _ => *self, // Handle other cases or panic if unexpected
        }
    }
}

// Placeholder for NegateCondition
fn NegateCondition(_condition: Condition) -> Condition {
    Condition::AL // placeholder
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LinkRegisterStatus {
    kLRHasNotBeenSaved,
    kLRHasBeenSaved,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DiscardMoveMode {
    kDontDiscardForSameWReg,
    kDiscardForSameWReg,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PreShiftImmMode {
    kNoShift,
    kLimitShiftForSP,
    kAnyShift,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StackLimitKind {
    kInterruptStackLimit,
    kRealStackLimit,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Condition {
    EQ,
    NE,
    HS,
    LO,
    MI,
    PL,
    VS,
    VC,
    HI,
    LS,
    GE,
    LT,
    GT,
    LE,
    AL,
    NV,
    // added for testing/compilation
    Invalid,
    Always,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RelocInfoMode {
    EXTERNAL_REFERENCE,
    CODE_TARGET,
}

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StackFrameType {
    JAVA_SCRIPT
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum IsolateFieldId {}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AbortReason {
    kOperandIsNotASmi,
    kOperandIsASmi,
    kUnreachable,
}

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Instr {}
const BREAK: Instr = Instr {};

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CPURegister {}
const NoCPUReg: CPURegister = CPURegister {};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Register {}
const NoReg: Register = Register {};
const x16: Register = Register {};
const x17: Register = Register {};
const x10: Register = Register {};
const x11: Register = Register {};
const lr: Register = Register {};
const sp: Register = Register {};
const fp_zero: Register = Register {};

impl Register {
    fn Is64Bits(&self) -> bool {
        true // placeholder
    }

    fn IsZero(&self) -> bool {
        self == &NoReg
    }
}

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct VRegister {}
const NoVReg: VRegister = VRegister {};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CPURegList {}
impl CPURegList {
    fn IncludesAliasOf(&self, _r: Register) -> bool {
        false //placeholder
    }
}

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ExternalReference {}
impl ExternalReference {
    fn Create(_id: IsolateFieldId) -> Self {
        ExternalReference {} //placeholder
    }
}

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Tagged<T> {
    value: T,
}

impl Tagged<i64> {
  fn new(value: i64) -> Self {
    Tagged { value }
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Address {}

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Handle<T> {
    value: T,
}

impl<T> Handle<T> {
    fn new(_value: T) -> Self {
        Handle { } // placeholder
    }
}

// Placeholder
type Code = i32;

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CodeEntrypointTag {}

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BarrierDomain {}

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BarrierType {}

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StubCallMode {}

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum IndirectPointerTag {}

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum JSDispatchHandle {}

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum JumpMode {}

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CallJumpMode {}

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DeoptimizeKind {}

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SaveFPRegsMode {}

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LogicalOp {}

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FlagsUpdate {}

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AddSubOp {}

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BranchTargetIdentifier {}

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SystemRegister {}

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SystemHint {}

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Shift {}
const LSL: Shift = Shift {};

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StatusFlags {}

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SmiCheck {}

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ReadOnlyCheck {}

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SlotDescriptor {}

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RootIndex {}

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ComparisonMode {}

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InvokeType {}

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ArgumentAdaptionMode {}

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CodeKind {}

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ExternalPointerTagRange {}

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DoubleRegList {}

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct RegList {}

const kXRegSize: u64 = 8;
const kXRegSizeInBits: u32 = 64;
const kWRegSizeInBits: u32 = 32;
const kQRegSizeInBits: u32 = 128;
const kDRegSizeInBits: u32 = 64;
const kSRegSizeInBits: u32 = 32;
const kDoubleSize: i32 = 8;

const kCodeIndirectPointerTag: IndirectPointerTag = IndirectPointerTag {};

// Placeholder
#[derive(Debug, Clone)]
struct V8Flags {
    native_code_counters: bool,
}

static v8_flags: V8Flags = V8Flags {
    native_code_counters: false,
};

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InstanceType {}

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct StatsCounter {}

// Placeholder
struct Isolate {}
struct Zone {}

// Placeholder
struct Runtime {}

impl Runtime {
  fn FunctionForId(_fid: i32) -> &Self {
    &Runtime {} // placeholder
  }

  const nargs: i32 = 0;
}

// Placeholder
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SetIsolateDataSlots {
    kYes,
    kNo,
}

#[derive(Debug)]
pub struct MacroAssembler {
    assembler_base: MacroAssemblerBase,
    tmp_list_: CPURegList,
    fptmp_list_: CPURegList,
    allow_macro_instructions_: bool
}

impl MacroAssembler {
    pub fn new(assembler_base: MacroAssemblerBase) -> Self {
        MacroAssembler {
            assembler_base,
            tmp_list_: Self::DefaultTmpList(),
            fptmp_list_: Self::DefaultFPTmpList(),
            allow_macro_instructions_: true
        }
    }

    #[cfg(debug_assertions)]
    pub fn set_allow_macro_instructions(&mut self, value: bool) {
        self.allow_macro_instructions_ = value;
    }

    #[cfg(debug_assertions)]
    pub fn allow_macro_instructions(&self) -> bool {
        self.allow_macro_instructions_
    }

    pub fn CanUseNearCallOrJump(&self, rmode: RelocInfoMode) -> bool {
        rmode != RelocInfoMode::EXTERNAL_REFERENCE
    }

    pub fn IsNearCallOffset(offset: i64) -> bool {
        offset >= i64::min_value() && offset <= i64::max_value() //placeholder
    }

    pub fn EnterFrame(&mut self, _type: StackFrameType) {
        // Implementation details...
    }

    pub fn LeaveFrame(&mut self, _type: StackFrameType) {
        // Implementation details...
    }

    pub fn InitializeRootRegister(&mut self) {
        // Implementation details...
    }

    pub fn Mov(&mut self, rd: &Register, operand: &Operand, discard_mode: DiscardMoveMode) {
        // Implementation details...
    }

    pub fn Mov_imm(&mut self, rd: &Register, imm: u64) {
      // Implementation details...
    }

    pub fn Mov_ext_ref(&mut self, rd: &Register, reference: ExternalReference) {
      // Implementation details...
    }

    pub fn LoadIsolateField(&mut self, rd: &Register, id: IsolateFieldId) {
      // Implementation details...
    }

    pub fn Mov_vreg_index_vreg_index(&mut self, vd: &VRegister, vd_index: i32, vn: &VRegister, vn_index: i32) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn Mov_smi(&mut self, rd: &Register, smi: Tagged<i64>) {
      // Implementation details...
    }

    pub fn Mov_vreg_vreg_index(&mut self, vd: &VRegister, vn: &VRegister, index: i32) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn Mov_vreg_index_reg(&mut self, vd: &VRegister, vd_index: i32, rn: &Register) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn Mov_reg_vreg_index(&mut self, rd: &Register, vn: &VRegister, vn_index: i32) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn Move(&mut self, dst: Register, src: Tagged<i64>) {
        // Implementation details...
    }

    pub fn Move_mem(&mut self, dst: Register, src: MemOperand) {
        // Implementation details...
    }

    pub fn Move_reg(&mut self, dst: Register, src: Register) {
        // Implementation details...
    }

    pub fn MovePair(&mut self, dst0: Register, src0: Register, dst1: Register, src1: Register) {
        // Implementation details...
    }

    pub fn Swap(&mut self, lhs: Register, rhs: Register) {
        // Implementation details...
    }

    pub fn Swap_vreg(&mut self, lhs: VRegister, rhs: VRegister) {
        // Implementation details...
    }

    pub fn fmla(&mut self, vd: &VRegister, vn: &VRegister, vm: &VRegister, vm_index: i32) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn fmls(&mut self, vd: &VRegister, vn: &VRegister, vm: &VRegister, vm_index: i32) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn fmul(&mut self, vd: &VRegister, vn: &VRegister, vm: &VRegister, vm_index: i32) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn fmulx(&mut self, vd: &VRegister, vn: &VRegister, vm: &VRegister, vm_index: i32) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn mul(&mut self, vd: &VRegister, vn: &VRegister, vm: &VRegister, vm_index: i32) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn mla(&mut self, vd: &VRegister, vn: &VRegister, vm: &VRegister, vm_index: i32) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn mls(&mut self, vd: &VRegister, vn: &VRegister, vm: &VRegister, vm_index: i32) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn sqdmulh(&mut self, vd: &VRegister, vn: &VRegister, vm: &VRegister, vm_index: i32) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn sqrdmulh(&mut self, vd: &VRegister, vn: &VRegister, vm: &VRegister, vm_index: i32) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn sqdmull(&mut self, vd: &VRegister, vn: &VRegister, vm: &VRegister, vm_index: i32) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn sqdmull2(&mut self, vd: &VRegister, vn: &VRegister, vm: &VRegister, vm_index: i32) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn sqdmlal(&mut self, vd: &VRegister, vn: &VRegister, vm: &VRegister, vm_index: i32) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn sqdmlal2(&mut self, vd: &VRegister, vn: &VRegister, vm: &VRegister, vm_index: i32) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn sqdmlsl(&mut self, vd: &VRegister, vn: &VRegister, vm: &VRegister, vm_index: i32) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn sqdmlsl2(&mut self, vd: &VRegister, vn: &VRegister, vm: &VRegister, vm_index: i32) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn smull(&mut self, vd: &VRegister, vn: &VRegister, vm: &VRegister, vm_index: i32) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn smull2(&mut self, vd: &VRegister, vn: &VRegister, vm: &VRegister, vm_index: i32) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn smlal(&mut self, vd: &VRegister, vn: &VRegister, vm: &VRegister, vm_index: i32) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn smlal2(&mut self, vd: &VRegister, vn: &VRegister, vm: &VRegister, vm_index: i32) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn smlsl(&mut self, vd: &VRegister, vn: &VRegister, vm: &VRegister, vm_index: i32) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn smlsl2(&mut self, vd: &VRegister, vn: &VRegister, vm: &VRegister, vm_index: i32) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn umull(&mut self, vd: &VRegister, vn: &VRegister, vm: &VRegister, vm_index: i32) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn umull2(&mut self, vd: &VRegister, vn: &VRegister, vm: &VRegister, vm_index: i32) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn umlal(&mut self, vd: &VRegister, vn: &VRegister, vm: &VRegister, vm_index: i32) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn umlal2(&mut self, vd: &VRegister, vn: &VRegister, vm: &VRegister, vm_index: i32) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn umlsl(&mut self, vd: &VRegister, vn: &VRegister, vm: &VRegister, vm_index: i32) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn umlsl2(&mut self, vd: &VRegister, vn: &VRegister, vm: &VRegister, vm_index: i32) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn abs(&mut self, vd: &VRegister, vn: &VRegister) {
        if cfg!(debug_assertions) {
          assert!(self.allow_macro_instructions());
        }
    }

    pub fn addp(&mut self, vd: &VRegister, vn: &VRegister) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn addv(&mut self, vd: &VRegister, vn: &VRegister) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn cls(&mut self, vd: &VRegister, vn: &VRegister) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn clz(&mut self, vd: &VRegister, vn: &VRegister) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn cnt(&mut self, vd: &VRegister, vn: &VRegister) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn faddp(&mut self, vd: &VRegister, vn: &VRegister) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn fcvtas(&mut self, vd: &VRegister, vn: &VRegister) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn fcvtau(&mut self, vd: &VRegister, vn: &VRegister) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn fcvtl(&mut self, vd: &VRegister, vn: &VRegister) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn fcvtms(&mut self, vd: &VRegister, vn: &VRegister) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn fcvtmu(&mut self, vd: &VRegister, vn: &VRegister) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn fcvtn(&mut self, vd: &VRegister, vn: &VRegister) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn fcvtns(&mut self, vd: &VRegister, vn: &VRegister) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn fcvtnu(&mut self, vd: &VRegister, vn: &VRegister) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn fcvtps(&mut self, vd: &VRegister, vn: &VRegister) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn fcvtpu(&mut self, vd: &VRegister, vn: &VRegister) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn fmaxnmp(&mut self, vd: &VRegister, vn: &VRegister) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn fmaxnmv(&mut self, vd: &VRegister, vn: &VRegister) {
      if cfg!(debug_assertions) {
        assert!(self.allow_macro_instructions());
      }
    }

    pub fn fmaxp(&mut