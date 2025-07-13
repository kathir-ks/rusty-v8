// Converted from V8 C++ source files:
// Header: macro-assembler-s390.h
// Implementation: macro-assembler-s390.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
use std::sync::Arc;
use std::{result, string::String};

use crate::base::platform::platform::MemoryChunk;
use crate::codegen::bailout_reason::AbortReason;
use crate::codegen::macro_assembler_base::MacroAssemblerBase;
use crate::codegen::s390::assembler_s390::*;
use crate::codegen::s390::constants_s390::*;
use crate::common::globals::*;
use crate::execution::frame_constants::*;
use crate::execution::isolate_data::*;
use crate::objects::contexts::*;

pub enum class StackLimitKind {
    kInterruptStackLimit,
    kRealStackLimit,
}

#[inline]
pub fn FieldMemOperand(object: Register, offset: i32) -> MemOperand {
    MemOperand { base: object, disp: offset - kHeapObjectTag }
}

#[inline]
pub fn FieldMemOperand_indexed(object: Register, index: Register, offset: i32) -> MemOperand {
    MemOperand { base: object, index: Some((index, 1)), disp: offset - kHeapObjectTag }
}

pub enum LinkRegisterStatus {
    kLRHasNotBeenSaved,
    kLRHasBeenSaved,
}

pub fn GetRegisterThatIsNotOneOf(
    reg1: Register,
    reg2: Option<Register>,
    reg3: Option<Register>,
    reg4: Option<Register>,
    reg5: Option<Register>,
    reg6: Option<Register>,
) -> Register {
    todo!()
}

pub struct MacroAssembler {
    base: MacroAssemblerBase,
}

impl MacroAssembler {
    pub fn new(base: MacroAssemblerBase) -> MacroAssembler {
        MacroAssembler { base }
    }

    pub fn CallBuiltin(&mut self, builtin: Builtin, cond: Condition) {}
    pub fn TailCallBuiltin(&mut self, builtin: Builtin, cond: Condition) {}
    pub fn AtomicCmpExchangeHelper(
        &mut self,
        addr: Register,
        output: Register,
        old_value: Register,
        new_value: Register,
        start: i32,
        end: i32,
        shift_amount: i32,
        offset: i32,
        temp0: Register,
        temp1: Register,
    ) {
    }
    pub fn AtomicCmpExchangeU8(
        &mut self,
        addr: Register,
        output: Register,
        old_value: Register,
        new_value: Register,
        temp0: Register,
        temp1: Register,
    ) {
    }
    pub fn AtomicCmpExchangeU16(
        &mut self,
        addr: Register,
        output: Register,
        old_value: Register,
        new_value: Register,
        temp0: Register,
        temp1: Register,
    ) {
    }
    pub fn AtomicExchangeHelper(
        &mut self,
        addr: Register,
        value: Register,
        output: Register,
        start: i32,
        end: i32,
        shift_amount: i32,
        offset: i32,
        scratch: Register,
    ) {
    }
    pub fn AtomicExchangeU8(
        &mut self,
        addr: Register,
        value: Register,
        output: Register,
        scratch: Register,
    ) {
    }
    pub fn AtomicExchangeU16(
        &mut self,
        addr: Register,
        value: Register,
        output: Register,
        scratch: Register,
    ) {
    }
    pub fn DoubleMax(
        &mut self,
        result_reg: DoubleRegister,
        left_reg: DoubleRegister,
        right_reg: DoubleRegister,
    ) {
    }
    pub fn DoubleMin(
        &mut self,
        result_reg: DoubleRegister,
        left_reg: DoubleRegister,
        right_reg: DoubleRegister,
    ) {
    }
    pub fn FloatMax(
        &mut self,
        result_reg: DoubleRegister,
        left_reg: DoubleRegister,
        right_reg: DoubleRegister,
    ) {
    }
    pub fn FloatMin(
        &mut self,
        result_reg: DoubleRegister,
        left_reg: DoubleRegister,
        right_reg: DoubleRegister,
    ) {
    }
    pub fn CeilF32(&mut self, dst: DoubleRegister, src: DoubleRegister) {}
    pub fn CeilF64(&mut self, dst: DoubleRegister, src: DoubleRegister) {}
    pub fn FloorF32(&mut self, dst: DoubleRegister, src: DoubleRegister) {}
    pub fn FloorF64(&mut self, dst: DoubleRegister, src: DoubleRegister) {}
    pub fn TruncF32(&mut self, dst: DoubleRegister, src: DoubleRegister) {}
    pub fn TruncF64(&mut self, dst: DoubleRegister, src: DoubleRegister) {}
    pub fn NearestIntF32(&mut self, dst: DoubleRegister, src: DoubleRegister) {}
    pub fn NearestIntF64(&mut self, dst: DoubleRegister, src: DoubleRegister) {}
    pub fn LoadFromConstantsTable(&mut self, destination: Register, constant_index: i32) {}
    pub fn LoadRootRegisterOffset(&mut self, destination: Register, offset: i64) {}
    pub fn LoadRootRelative(&mut self, destination: Register, offset: i32) {}
    pub fn StoreRootRelative(&mut self, offset: i32, value: Register) {}
    pub fn ExternalReferenceAsOperand(&mut self, reference: ExternalReference, scratch: Register) -> MemOperand { todo!() }
    pub fn Jump(&mut self, target: Register, cond: Condition) {}
    pub fn Jump(&mut self, target: Address, rmode: RelocInfo::Mode, cond: Condition) {}
    pub fn Jump(&mut self, code: Handle<Code>, rmode: RelocInfo::Mode, cond: Condition) {}
    pub fn Jump(&mut self, reference: &ExternalReference) {}
    pub fn JumpIfSmi(&mut self, value: Register, smi_label: &mut Label) {}
    pub fn CheckSmi(&mut self, src: Register) -> Condition { todo!() }
    pub fn JumpIfEqual(&mut self, x: Register, y: i32, dest: &mut Label) {}
    pub fn JumpIfLessThan(&mut self, x: Register, y: i32, dest: &mut Label) {}
    pub fn Switch(&mut self, scrach: Register, reg: Register, case_base_value: i32, labels: &mut [*mut Label], num_labels: i32) {}
    pub fn JumpIfCodeIsMarkedForDeoptimization(&mut self, code: Register, scratch: Register, if_marked_for_deoptimization: &mut Label) {}
    pub fn JumpIfCodeIsTurbofanned(&mut self, code: Register, scratch: Register, if_turbofanned: &mut Label) {}
    pub fn LoadMap(&mut self, destination: Register, object: Register) {}
    pub fn LoadCompressedMap(&mut self, destination: Register, object: Register) {}
    pub fn LoadFeedbackVector(&mut self, dst: Register, closure: Register, scratch: Register, fbv_undef: &mut Label) {}
    pub fn Call(&mut self, target: Register) {}
    pub fn Call(&mut self, target: Address, rmode: RelocInfo::Mode, cond: Condition) {}
    pub fn Call(&mut self, code: Handle<Code>, rmode: RelocInfo::Mode, cond: Condition) {}
    pub fn Ret(&mut self) {}
    pub fn Ret_Cond(&mut self, cond: Condition) {}
    pub fn BailoutIfDeoptimized(&mut self, scratch: Register) {}
    pub fn CallForDeoptimization(&mut self, target: Builtin, deopt_id: i32, exit: &mut Label, kind: DeoptimizeKind, ret: &mut Label, jump_deoptimization_entry_label: &mut Label) {}
    pub fn Drop(&mut self, count: i32) {}
    pub fn Drop_register(&mut self, count: Register, scratch: Register) {}
    pub fn Ret_drop(&mut self, drop: i32) {}
    pub fn Call(&mut self, target: &mut Label) {}
    pub fn GetLabelAddress(&mut self, dst: Register, target: &mut Label) {}
    pub fn LoadEntryFromBuiltinIndex(&mut self, builtin_index: Register, target: Register) {}
    pub fn LoadEntryFromBuiltin(&mut self, builtin: Builtin, destination: Register) {}
    pub fn EntryFromBuiltinAsOperand(&mut self, builtin: Builtin) -> MemOperand { todo!() }
    pub fn LoadCodeInstructionStart(&mut self, destination: Register, code_object: Register, tag: CodeEntrypointTag) {}
    pub fn CallCodeObject(&mut self, code_object: Register) {}
    pub fn JumpCodeObject(&mut self, code_object: Register, jump_mode: JumpMode) {}
    pub fn CallBuiltinByIndex(&mut self, builtin_index: Register, target: Register) {}
    pub fn Move(&mut self, dst: Register, smi: Tagged<Smi>) {}
    pub fn Move(&mut self, dst: Register, value: Handle<HeapObject>, rmode: RelocInfo::Mode) {}
    pub fn Move(&mut self, dst: Register, reference: ExternalReference) {}
    pub fn LoadIsolateField(&mut self, dst: Register, id: IsolateFieldId) {}
    pub fn Move_MemOperand(&mut self, dst: Register, src: MemOperand) {}
    pub fn Move_Register_Register(&mut self, dst: Register, src: Register, cond: Condition) {}
    pub fn Move(&mut self, dst: DoubleRegister, src: DoubleRegister) {}
    pub fn MoveChar(&mut self, opnd1: MemOperand, opnd2: MemOperand, length: Operand) {}
    pub fn CompareLogicalChar(&mut self, opnd1: MemOperand, opnd2: MemOperand, length: Operand) {}
    pub fn ExclusiveOrChar(&mut self, opnd1: MemOperand, opnd2: MemOperand, length: Operand) {}
    pub fn RotateInsertSelectBits(&mut self, dst: Register, src: Register, startBit: Operand, endBit: Operand, shiftAmt: Operand, zeroBits: bool) {}
    pub fn BranchRelativeOnIdxHighP(&mut self, dst: Register, inc: Register, L: &mut Label) {}
    pub fn MaybeSaveRegisters(&mut self, registers: RegList) {}
    pub fn MaybeRestoreRegisters(&mut self, registers: RegList) {}
    pub fn CallEphemeronKeyBarrier(&mut self, object: Register, slot_address: Register, fp_mode: SaveFPRegsMode) {}
    pub fn CallRecordWriteStubSaveRegisters(&mut self, object: Register, slot_address: Register, fp_mode: SaveFPRegsMode, mode: StubCallMode) {}
    pub fn CallRecordWriteStub(&mut self, object: Register, slot_address: Register, fp_mode: SaveFPRegsMode, mode: StubCallMode) {}
    pub fn MultiPush(&mut self, regs: RegList, location: Register) {}
    pub fn MultiPop(&mut self, regs: RegList, location: Register) {}
    pub fn MultiPushDoubles(&mut self, dregs: DoubleRegList, location: Register) {}
    pub fn MultiPopDoubles(&mut self, dregs: DoubleRegList, location: Register) {}
    pub fn MultiPushV128(&mut self, dregs: DoubleRegList, scratch: Register, location: Register) {}
    pub fn MultiPopV128(&mut self, dregs: DoubleRegList, scratch: Register, location: Register) {}
    pub fn MultiPushF64OrV128(&mut self, dregs: DoubleRegList, scratch: Register, location: Register) {}
    pub fn MultiPopF64OrV128(&mut self, dregs: DoubleRegList, scratch: Register, location: Register) {}
    pub fn PushAll(&mut self, registers: RegList) {}
    pub fn PopAll(&mut self, registers: RegList) {}
    pub fn PushAll_doubles(&mut self, registers: DoubleRegList, stack_slot_size: i32) {}
    pub fn PopAll_doubles(&mut self, registers: DoubleRegList, stack_slot_size: i32) {}
    pub fn RequiredStackSizeForCallerSaved(&mut self, fp_mode: SaveFPRegsMode, exclusion1: Register, exclusion2: Register, exclusion3: Register) -> i32 { todo!() }
    pub fn PushCallerSaved(&mut self, fp_mode: SaveFPRegsMode, scratch: Register, exclusion1: Register, exclusion2: Register, exclusion3: Register) -> i32 { todo!() }
    pub fn PopCallerSaved(&mut self, fp_mode: SaveFPRegsMode, scratch: Register, exclusion1: Register, exclusion2: Register, exclusion3: Register) -> i32 { todo!() }
    pub fn LoadRoot(&mut self, destination: Register, index: RootIndex) {}
    pub fn LoadRoot_cond(&mut self, destination: Register, index: RootIndex, cond: Condition) {}
    pub fn LoadTaggedRoot(&mut self, destination: Register, index: RootIndex) {}
    pub fn AddS32(&mut self, dst: Register, imm: Operand) {}
    pub fn AddS64(&mut self, dst: Register, imm: Operand) {}
    pub fn AddS32_register_operand(&mut self, dst: Register, src: Register, imm: Operand) {}
    pub fn AddS64_register_operand(&mut self, dst: Register, src: Register, imm: Operand) {}
    pub fn AddS32_register_immediate(&mut self, dst: Register, src: Register, imm: i32) {}
    pub fn AddS64_register_immediate(&mut self, dst: Register, src: Register, imm: i32) {}
    pub fn AddS32_register_register(&mut self, dst: Register, src: Register) {}
    pub fn AddS64_register_register(&mut self, dst: Register, src: Register) {}
    pub fn AddS32_register_register_register(&mut self, dst: Register, src1: Register, src2: Register) {}
    pub fn AddS64_register_register_register(&mut self, dst: Register, src1: Register, src2: Register) {}
    pub fn AddS32_register_memoperand(&mut self, dst: Register, opnd: MemOperand) {}
    pub fn AddS64_register_memoperand(&mut self, dst: Register, opnd: MemOperand) {}
    pub fn AddS32_memoperand_operand(&mut self, opnd: MemOperand, imm: Operand) {}
    pub fn AddS64_memoperand_operand(&mut self, opnd: MemOperand, imm: Operand) {}
    pub fn AddU32(&mut self, dst: Register, imm: Operand) {}
    pub fn AddU64(&mut self, dst: Operand) {}
    pub fn AddU64_register_immediate(&mut self, dst: Register, imm: i32) {}
    pub fn AddU64_register_register(&mut self, dst: Register, src1: Register, src2: Register) {}
    pub fn AddU64_register_register_algr(&mut self, dst: Register, src: Register) {}
    pub fn AddU32_register_memoperand(&mut self, dst: Register, opnd: MemOperand) {}
    pub fn AddU64_register_memoperand(&mut self, dst: Register, opnd: MemOperand) {}
    pub fn SubS32(&mut self, dst: Register, imm: Operand) {}
    pub fn SubS64(&mut self, dst: Register, imm: Operand) {}
    pub fn SubS32_register_operand(&mut self, dst: Register, src: Register, imm: Operand) {}
    pub fn SubS64_register_operand(&mut self, dst: Register, src: Register, imm: Operand) {}
    pub fn SubS32_register_immediate(&mut self, dst: Register, src: Register, imm: i32) {}
    pub fn SubS64_register_immediate(&mut self, dst: Register, src: Register, imm: i32) {}
    pub fn SubS32_register_register(&mut self, dst: Register, src: Register) {}
    pub fn SubS64_register_register(&mut self, dst: Register, src: Register) {}
    pub fn SubS32_register_register_register(&mut self, dst: Register, src1: Register, src2: Register) {}
    pub fn SubS64_register_register_register(&mut self, dst: Register, src1: Register, src2: Register) {}
    pub fn SubS32_register_memoperand(&mut self, dst: Register, opnd: MemOperand) {}
    pub fn SubS64_register_memoperand(&mut self, dst: Register, opnd: MemOperand) {}
    pub fn LoadAndSub32(&mut self, dst: Register, src: Register, opnd: MemOperand) {}
    pub fn LoadAndSub64(&mut self, dst: Register, src: Register, opnd: MemOperand) {}
    pub fn SubU32_register_memoperand(&mut self, dst: Register, opnd: MemOperand) {}
    pub fn SubU64_register_memoperand(&mut self, dst: Register, opnd: MemOperand) {}
    pub fn MulS64(&mut self, dst: Register, opnd: Operand) {}
    pub fn MulS64_register(&mut self, dst: Register, src: Register) {}
    pub fn MulS64_register_memoperand(&mut self, dst: Register, opnd: MemOperand) {}
    pub fn Sqrt(&mut self, result: DoubleRegister, input: DoubleRegister) {}
    pub fn Sqrt_MemOperand(&mut self, result: DoubleRegister, input: MemOperand) {}
    pub fn CmpS32(&mut self, src1: Register, src2: Register) {}
    pub fn CmpS64(&mut self, src1: Register, src2: Register) {}
    pub fn CmpS32_register_operand(&mut self, dst: Register, opnd: Operand) {}
    pub fn CmpS64_register_operand(&mut self, dst: Register, opnd: Operand) {}
    pub fn CmpS32_register_memoperand(&mut self, dst: Register, opnd: MemOperand) {}
    pub fn CmpS64_register_memoperand(&mut self, dst: Register, opnd: MemOperand) {}
    pub fn CmpAndSwap(&mut self, old_val: Register, new_val: Register, opnd: MemOperand) {}
    pub fn CmpAndSwap64(&mut self, old_val: Register, new_val: Register, opnd: MemOperand) {}
    pub fn CmpU32(&mut self, src1: Register, src2: Register) {}
    pub fn CmpU64(&mut self, src1: Register, src2: Register) {}
    pub fn CmpU32_register_operand(&mut self, src1: Register, opnd: Operand) {}
    pub fn CmpU64_register_operand(&mut self, src1: Register, opnd: Operand) {}
    pub fn CmpU32_register_memoperand(&mut self, dst: Register, opnd: MemOperand) {}
    pub fn CmpU64_register_memoperand(&mut self, dst: Register, opnd: MemOperand) {}
    pub fn CmpF32(&mut self, src1: DoubleRegister, src2: DoubleRegister) {}
    pub fn CmpF64(&mut self, src1: DoubleRegister, src2: DoubleRegister) {}
    pub fn CmpF32_register_memoperand(&mut self, src1: DoubleRegister, src2: MemOperand) {}
    pub fn CmpF64_register_memoperand(&mut self, src1: DoubleRegister, src2: MemOperand) {}
    pub fn LoadU64(&mut self, dst: Register, mem: MemOperand, scratch: Register) {}
    pub fn LoadS32(&mut self, dst: Register, mem: MemOperand, scratch: Register) {}
    pub fn LoadS32_register(&mut self, dst: Register, src: Register) {}
    pub fn LoadU32(&mut self, dst: Register, mem: MemOperand, scratch: Register) {}
    pub fn LoadU32_register(&mut self, dst: Register, src: Register) {}
    pub fn LoadU16(&mut self, dst: Register, mem: MemOperand) {}
    pub fn LoadU16_register(&mut self, dst: Register, src: Register) {}
    pub fn LoadS16(&mut self, dst: Register, src: Register) {}
    pub fn LoadS16_register_memoperand(&mut self, dst: Register, mem: MemOperand, scratch: Register) {}
    pub fn LoadS8(&mut self, dst: Register, mem: MemOperand) {}
    pub fn LoadS8_register(&mut self, dst: Register, src: Register) {}
    pub fn LoadU8(&mut self, dst: Register, mem: MemOperand) {}
    pub fn LoadU8_register(&mut self, dst: Register, src: Register) {}
    pub fn LoadV128(&mut self, dst: Simd128Register, mem: MemOperand, scratch: Register) {}
    pub fn LoadF64(&mut self, dst: DoubleRegister, mem: MemOperand) {}
    pub fn LoadF32(&mut self, dst: DoubleRegister, mem: MemOperand) {}
    pub fn LoadU64LE(&mut self, dst: Register, mem: MemOperand, scratch: Register) {}
    pub fn LoadS32LE(&mut self, dst: Register, opnd: MemOperand, scratch: Register) {}
    pub fn LoadU32LE(&mut self, dst: Register, opnd: MemOperand, scratch: Register) {}
    pub fn LoadU16LE(&mut self, dst: Register, opnd: MemOperand) {}
    pub fn LoadS16LE(&mut self, dst: Register, opnd: MemOperand) {}
    pub fn LoadV128LE(&mut self, dst: DoubleRegister, mem: MemOperand, scratch0: Register, scratch1: Register) {}
    pub fn LoadF64LE(&mut self, dst: DoubleRegister, opnd: MemOperand, scratch: Register) {}
    pub fn LoadF32LE(&mut self, dst: DoubleRegister, opnd: MemOperand, scratch: Register) {}
    pub fn StoreU64(&mut self, src: Register, mem: MemOperand, scratch: Register) {}
    pub fn StoreU32(&mut self, src: Register, mem: MemOperand, scratch: Register) {}
    pub fn StoreU16(&mut self, src: Register, mem: MemOperand, scratch: Register) {}
    pub fn StoreU8(&mut self, src: Register, mem: MemOperand, scratch: Register) {}
    pub fn StoreF64(&mut self, dst: DoubleRegister, opnd: MemOperand) {}
    pub fn StoreF32(&mut self, dst: DoubleRegister, opnd: MemOperand) {}
    pub fn StoreV128(&mut self, src: Simd128Register, mem: MemOperand, scratch: Register) {}
    pub fn StoreU64LE(&mut self, src: Register, mem: MemOperand, scratch: Register) {}
    pub fn StoreU32LE(&mut self, src: Register, mem: MemOperand, scratch: Register) {}
    pub fn StoreU16LE(&mut self, src: Register, mem: MemOperand, scratch: Register) {}
    pub fn StoreF64LE(&mut self, src: DoubleRegister, opnd: MemOperand, scratch: Register) {}
    pub fn StoreF32LE(&mut self, src: DoubleRegister, opnd: MemOperand, scratch: Register) {}
    pub fn AddF32(&mut self, dst: DoubleRegister, lhs: DoubleRegister, rhs: DoubleRegister) {}
    pub fn SubF32(&mut self, dst: DoubleRegister, lhs: DoubleRegister, rhs: DoubleRegister) {}
    pub fn MulF32(&mut self, dst: DoubleRegister, lhs: DoubleRegister, rhs: DoubleRegister) {}
    pub fn DivF32(&mut self, dst: DoubleRegister, lhs: DoubleRegister, rhs: DoubleRegister) {}
    pub fn AddF64(&mut self, dst: DoubleRegister, lhs: DoubleRegister, rhs: DoubleRegister) {}
    pub fn SubF64(&mut self, dst: DoubleRegister, lhs: DoubleRegister, rhs: DoubleRegister) {}
    pub fn MulF64(&mut self, dst: DoubleRegister, lhs: DoubleRegister, rhs: DoubleRegister) {}
    pub fn DivF64(&mut self, dst: DoubleRegister, lhs: DoubleRegister, rhs: DoubleRegister) {}
    pub fn AddFloat32(&mut self, dst: DoubleRegister, opnd: MemOperand, scratch: DoubleRegister) {}
    pub fn AddFloat64(&mut self, dst: DoubleRegister, opnd: MemOperand, scratch: DoubleRegister) {}
    pub fn SubFloat32(&mut self, dst: DoubleRegister, opnd: MemOperand, scratch: DoubleRegister) {}
    pub fn SubFloat64(&mut self, dst: DoubleRegister, opnd: MemOperand, scratch: DoubleRegister) {}
    pub fn MulFloat32(&mut self, dst: DoubleRegister, opnd: MemOperand, scratch: DoubleRegister) {}
    pub fn MulFloat64(&mut self, dst: DoubleRegister, opnd: MemOperand, scratch: DoubleRegister) {}
    pub fn DivFloat32(&mut self, dst: DoubleRegister, opnd: MemOperand, scratch: DoubleRegister) {}
    pub fn DivFloat64(&mut self, dst: DoubleRegister, opnd: MemOperand, scratch: DoubleRegister) {}
    pub fn LoadF32AsF64(&mut self, dst: DoubleRegister, opnd: MemOperand) {}
    pub fn LoadOnConditionP(&mut self, cond: Condition, dst: Register, src: Register) {}
    pub fn LoadPositiveP(&mut self, result: Register, input: Register) {}
    pub fn LoadPositive32(&mut self, result: Register, input: Register) {}
    pub fn Branch(&mut self, c: Condition, opnd: Operand) {}
    pub fn BranchOnCount(&mut self, r1: Register, l: &mut Label) {}
    pub fn ShiftLeftU32(&mut self, dst: Register, src: Register, val: Register, val2: Operand) {}
    pub fn ShiftLeftU32_withOperand(&mut self, dst: Register, src: Register, val: Operand) {}
    pub fn ShiftLeftU64(&mut self, dst: Register, src: Register, val: Register, val2: Operand) {}
    pub fn ShiftLeftU64_withOperand(&mut self, dst: Register, src: Register, val: Operand) {}
    pub fn ShiftRightU32(&mut self, dst: Register, src: Register, val: Register, val2: Operand) {}
    pub fn ShiftRightU32_withOperand(&mut self, dst: Register, src: Register, val: Operand) {}
    pub fn ShiftRightU64(&mut self, dst: Register, src: Register, val: Register, val2: Operand) {}
    pub fn ShiftRightU64_withOperand(&mut self, dst: Register, src: Register, val: Operand) {}
    pub fn ShiftRightS32(&mut self, dst: Register, src: Register, shift: Register, val2: Operand) {}
    pub fn ShiftRightS32_withOperand(&mut self, dst: Register, src: Register, val: Operand) {}
    pub fn ShiftRightS64(&mut self, dst: Register, src: Register, shift: Register, val2: Operand) {}
    pub fn ShiftRightS64_withOperand(&mut self, dst: Register, src: Register, val: Operand) {}
    pub fn ClearRightImm(&mut self, dst: Register, src: Register, val: Operand) {}
    pub fn And(&mut self, dst: Register, src: Register) {}
    pub fn AndP(&mut self, dst: Register, src: Register) {}
    pub fn And_dst_src1_src2(&mut self, dst: Register, src1: Register, src2: Register) {}
    pub fn AndP_dst_src1_src2(&mut self, dst: Register, src1: Register, src2: Register) {}
    pub fn And_register_memoperand(&mut self, dst: Register, opnd: MemOperand) {}
    pub fn AndP_register_memoperand(&mut self, dst: Register, opnd: MemOperand) {}
    pub fn And_register_operand(&mut self, dst: Register, opnd: Operand) {}
    pub fn AndP_register_operand(&mut self, dst: Register, opnd: Operand) {}
    pub fn And_register_src_operand(&mut self, dst: Register, src: Register, opnd: Operand) {}
    pub fn AndP_register_src_operand(&mut self, dst: Register, src: Register, opnd: Operand) {}
    pub fn Or(&mut self, dst: Register, src: Register) {}
    pub fn OrP(&mut self, dst: Register, src: Register) {}
    pub fn Or_register_register_register(&mut self, dst: Register, src1: Register, src2: Register) {}
    pub fn OrP_register_register_register(&mut self, dst: Register, src1: Register, src2: Register) {}
    pub fn Or_register_memoperand(&mut self, dst: Register, opnd: MemOperand) {}
    pub fn OrP_register_memoperand(&mut self, dst: Register, opnd: MemOperand) {}
    pub fn Or_register_operand(&mut self, dst: Register, opnd: Operand) {}
    pub fn OrP_register_operand(&mut self, dst: Register, opnd: Operand) {}
    pub fn Or_register_src_operand(&mut self, dst: Register, src: Register, opnd: Operand) {}
    pub fn OrP_register_src_operand(&mut self, dst: Register, src: Register, opnd: Operand) {}
    pub fn Xor(&mut self, dst: Register, src: Register) {}
    pub fn XorP(&mut self, dst: Register, src: Register) {}
    pub fn Xor_register_register_register(&mut self, dst: Register, src1: Register, src2: Register) {}
    pub fn XorP_register_register_register(&mut self, dst: Register, src1: Register, src2: Register) {}
    pub fn Xor_register_memoperand(&mut self, dst: Register, opnd: MemOperand) {}
    pub fn XorP_register_memoperand(&mut self, dst: Register, opnd: MemOperand) {}
    pub fn Xor_register_operand(&mut self, dst: Register, opnd: Operand) {}
    pub fn XorP_register_operand(&mut self, dst: Register, opnd: Operand) {}
    pub fn Xor_register_src_operand(&mut self, dst: Register, src: Register, opnd: Operand) {}
    pub fn XorP_register_src_operand(&mut self, dst: Register, src: Register, opnd: Operand) {}
    pub fn Popcnt32(&mut self, dst: Register, src: Register) {}
    pub fn Not32(&mut self, dst: Register, src: Register) {}
    pub fn Not64(&mut self, dst: Register, src: Register) {}
    pub fn NotP(&mut self, dst: Register, src: Register) {}
    pub fn Popcnt64(&mut self, dst: Register, src: Register) {}
}

impl MacroAssembler {
    pub fn This(&self) -> This { This { dummy: 0 } }
}

pub enum This {
    r3,
    r2,
    r4,
    NoImmediate,
    None,
    NoRootListIndex,
    ThisRegister,
    NoCondition,
    LeaveRC,
    IgnoreCaching,
    NotStatic,
    Ok,
}

// Dummy Implementation for OpIndex
pub struct OpIndex(u32);

impl OpIndex {
    pub fn ToInt(&self) -> i32 {
        self.0 as i32
    }
}

// Dummy Implementation for Label
pub struct Label {
    target: Option<usize>,
    bound: bool,
    id: i32,
}

impl Label {
    fn new() -> Label {
        Label { target: None, bound: false, id: 0 }
    }
    fn LinkTo(&mut self, offset: usize) {
        self.target = Some(offset);
        self.bound = true;
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CompactBranchType {
    Near,
    Far,
}

impl Operand {
  fn Zero() -> Self {
      Operand {
          immediate: 0,
          rmode: RelocInfo::NO_INFO,
      }
  }
  fn new(immediate: i32) -> Self {
      Operand {
          immediate,
          rmode: RelocInfo::NO_INFO,
      }
  }
}

impl MemOperand {
  fn new(base: Register, disp: i32) -> Self {
      MemOperand {
          base: base,
          index: None,
          disp: disp,
      }
  }
  fn new_index(base: Register, index:Register, disp: i32) -> Self {
      MemOperand {
          base: base,
          index: Some((index, 1)),
          disp: disp,
      }
  }
}

#[derive(Clone, Copy, Debug)]
struct RelocInfo {}

impl RelocInfo {
    const NO_INFO: RelocInfoMode = RelocInfoMode::NONE;
    const RELATIVE_CODE_TARGET: RelocInfoMode = RelocInfoMode::RELATIVE_CODE_TARGET;
    const CODE_TARGET: RelocInfoMode = RelocInfoMode::CODE_TARGET;
    const FULL_EMBEDD
