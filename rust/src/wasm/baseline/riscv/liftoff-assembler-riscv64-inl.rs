// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod liftoff_assembler_riscv64 {
    //use crate::codegen::interface_descriptors;
    //use crate::heap::mutable_page_metadata;
    //use crate::wasm::baseline::liftoff_assembler;
    //use crate::wasm::baseline::riscv::liftoff_assembler_riscv;
    //use crate::wasm::wasm_objects;

    // Missing C++ dependencies.  Defining empty structs for now.
    pub struct LiftoffAssembler {}
    pub struct MemOperand {}
    pub struct LiftoffRegister {}
    pub enum ValueKind {
        kI32,
        kI64,
        kRef,
        kRefNull,
        kF32,
        kF64,
        kS128,
        kVoid,
        kI8,
        kI16,
        kTop,
        kBottom,
        kF16,
    }
    pub struct Operand {}

    // These are some dummy implementations for types used in the original C++
    // code. They are here for the code to compile, but they are not complete
    // translations of the original.
    impl LiftoffAssembler {
        pub fn ExtractBits(&mut self, _reg: Register, _offset: Register, _start: i32, _length: i32, _sign_extend: bool) {}
        pub fn CalcScaledAddress(&mut self, _dst: Register, _base: Register, _index: Register, _scale: u32) {}
        pub fn Add64(&mut self, _dst: Register, _src1: Register, _src2: Register) {}
        pub fn li(&mut self, _reg: Register, _operand: Operand) {}
        pub fn Lw(&mut self, _dst: Register, _src: MemOperand) {}
        pub fn Ld(&mut self, _dst: Register, _src: MemOperand) {}
        pub fn LoadFloat(&mut self, _dst: DoubleRegister, _src: MemOperand) {}
        pub fn LoadDouble(&mut self, _dst: DoubleRegister, _src: MemOperand) {}
        pub fn VU(&mut self) -> VU { VU {} }
        pub fn vl(&mut self, _dst: VRegister, _src: Register, _offset: i32, _e8: VSew) {}
        pub fn Sw(&mut self, _src: Register, _dst: MemOperand) {}
        pub fn Sd(&mut self, _src: Register, _dst: MemOperand) {}
        pub fn StoreFloat(&mut self, _src: DoubleRegister, _dst: MemOperand) {}
        pub fn StoreDouble(&mut self, _src: DoubleRegister, _dst: MemOperand) {}
        pub fn vs(&mut self, _src: VRegister, _dst: Register, _offset: i32, _e8: VSew) {}
        pub fn addi(&mut self, _dst: Register, _src: Register, _imm: i32) {}
        pub fn push(&mut self, _reg: Register) {}
        pub fn StoreWord(&mut self, _src: Register, _dst: MemOperand) {}
        pub fn AddWord(&mut self, _dst: Register, _src1: Register, _src2: Operand) {}
        pub fn Lbu(&mut self, _dst: Register, _src: MemOperand, _trapper: Option<fn(i32)>) {}
        pub fn Lb(&mut self, _dst: Register, _src: MemOperand, _trapper: Option<fn(i32)>) {}
        pub fn Lhu(&mut self, _dst: Register, _src: MemOperand, _trapper: Option<fn(i32)>) {}
        pub fn Lh(&mut self, _dst: Register, _src: MemOperand, _trapper: Option<fn(i32)>) {}
        pub fn Lwu(&mut self, _dst: Register, _src: MemOperand, _trapper: Option<fn(i32)>) {}
        pub fn li_wasm_value(&mut self, _reg: Register, _value: WasmValue) {}
        pub fn BlockPoolsScope(&mut self, _i: i32) -> BlockPoolsScope { BlockPoolsScope {} }
        pub fn LoadTaggedField(&mut self, _dst: Register, _src: MemOperand, _trapper: Option<fn(i32)>) {}
        pub fn LoadProtectedPointerField(&mut self, _dst: Register, _src: MemOperand) {}
        pub fn LoadWord(&mut self, _dst: Register, _src: MemOperand) {}
        pub fn LoadCodeEntrypointViaCodePointer(&mut self, _dst: Register, _src: MemOperand, _tag: i32) {}
        pub fn StoreTaggedField(&mut self, _src: Register, _dst: MemOperand, _trapper: Option<fn(i32)>) {}
        pub fn CheckPageFlag(&mut self, _addr: Register, _mask: i32, _eq: Register, _exit: &Label) {}
        pub fn JumpIfSmi(&mut self, _src: Register, _exit: &Label) {}
        pub fn CallRecordWriteStubSaveRegisters(&mut self, _addr: Register, _offset: Operand, _save_fp_regs: SaveFPRegsMode, _stub_call_mode: StubCallMode) {}
        pub fn bind(&mut self, _label: &Label) {}
        pub fn Sb(&mut self, _src: Register, _dst: MemOperand, _trapper: Option<fn(i32)>) {}
        pub fn Sh(&mut self, _src: Register, _dst: MemOperand, _trapper: Option<fn(i32)>) {}
        pub fn synci(&mut self) {}
        pub fn ChangeEndiannessLoad(&mut self, _dst: LiftoffRegister, _type: LoadType, _pinned: LiftoffRegList) {}
        pub fn ChangeEndiannessStore(&mut self, _src: LiftoffRegister, _type: StoreType, _pinned: LiftoffRegList) {}
        pub fn sync(&mut self) {}
        pub fn lr_w(&mut self, _arg0: bool, _arg1: bool, _result_reg: Register, _actual_addr: Register) {}
        pub fn lr_d(&mut self, _arg0: bool, _arg1: bool, _result_reg: Register, _actual_addr: Register) {}
        pub fn sc_w(&mut self, _arg0: bool, _arg1: bool, _store_result: Register, _actual_addr: Register, _temp: Register) {}
        pub fn sc_d(&mut self, _arg0: bool, _arg1: bool, _store_result: Register, _actual_addr: Register, _temp: Register) {}
        pub fn bnez(&mut self, _store_result: Register, _retry: &Label) {}
        pub fn lbu(&mut self, _result_reg: Register, _actual_addr: Register, _i32: i32) {}
        pub fn lhu(&mut self, _result_reg: Register, _actual_addr: Register, _i32: i32) {}
        pub fn lw(&mut self, _result_reg: Register, _actual_addr: Register, _i32: i32) {}
        pub fn lwu(&mut self, _result_reg: Register, _actual_addr: Register, _i32: i32) {}
        pub fn ld(&mut self, _result_reg: Register, _actual_addr: Register, _i32: i32) {}
        pub fn sb(&mut self, _result_reg: Register, _actual_addr: Register, _i32: i32) {}
        pub fn sh(&mut self, _result_reg: Register, _actual_addr: Register, _i32: i32) {}
        pub fn AtomicBinop(&mut self, _dst_addr: Register, _offset_reg: Register, _offset_imm: usize, _value: LiftoffRegister, _result: LiftoffRegister, _type: StoreType, _op: Binop) {}
        pub fn li_i32(&mut self, _reg: Register, _i32: i32) {}
        pub fn Clz64(&mut self, _dst: Register, _src: Register) {}
        pub fn Ctz64(&mut self, _dst: Register, _src: Register) {}
        pub fn Popcnt64(&mut self, _dst: Register, _src: Register, _scratch: Register) {}
        pub fn Mul32(&mut self, _dst: Register, _lhs: Register, _rhs: Register) {}
        pub fn Branch(&mut self, _target: &Label, _condition: Condition, _op1: Register, _op2: Operand) {}
        pub fn CompareI(&mut self, _reg: Register, _op1: Register, _op2: Operand, _cond: Condition) {}
        pub fn Div32(&mut self, _dst: Register, _lhs: Register, _rhs: Register) {}
        pub fn Divu32(&mut self, _dst: Register, _lhs: Register, _rhs: Register) {}
        pub fn Mod32(&mut self, _dst: Register, _lhs: Register, _rhs: Register) {}
        pub fn Modu32(&mut self, _dst: Register, _lhs: Register, _rhs: Register) {}
        pub fn addw(&mut self, _dst: Register, _lhs: Register, _rhs: Register) {}
        pub fn subw(&mut self, _dst: Register, _lhs: Register, _rhs: Register) {}
        pub fn and_(&mut self, _dst: Register, _lhs: Register, _rhs: Register) {}
        pub fn or_(&mut self, _dst: Register, _lhs: Register, _rhs: Register) {}
        pub fn xor_(&mut self, _dst: Register, _lhs: Register, _rhs: Register) {}
        pub fn Add32(&mut self, _dst: Register, _lhs: Register, _imm: Operand) {}
        pub fn Sub32(&mut self, _dst: Register, _lhs: Register, _imm: Operand) {}
        pub fn And(&mut self, _dst: Register, _lhs: Register, _imm: Operand) {}
        pub fn Or(&mut self, _dst: Register, _lhs: Register, _imm: Operand) {}
        pub fn Xor(&mut self, _dst: Register, _lhs: Register, _imm: Operand) {}
        pub fn Clz32(&mut self, _dst: Register, _src: Register) {}
        pub fn Ctz32(&mut self, _dst: Register, _src: Register) {}
        pub fn Popcnt32(&mut self, _dst: Register, _src: Register, _scratch: Register) {}
        pub fn sllw(&mut self, _dst: Register, _src: Register, _amount: Register) {}
        pub fn sraw(&mut self, _dst: Register, _src: Register, _amount: Register) {}
        pub fn srlw(&mut self, _dst: Register, _src: Register, _amount: Register) {}
        pub fn slliw(&mut self, _dst: Register, _src: Register, _amount: i32) {}
        pub fn sraiw(&mut self, _dst: Register, _src: Register, _amount: i32) {}
        pub fn srliw(&mut self, _dst: Register, _src: Register, _amount: i32) {}
        pub fn Mul64(&mut self, _dst: Register, _lhs: Register, _rhs: Register) {}
        pub fn Div64(&mut self, _dst: Register, _lhs: Register, _rhs: Register) {}
        pub fn Divu64(&mut self, _dst: Register, _lhs: Register, _rhs: Register) {}
        pub fn Mod64(&mut self, _dst: Register, _lhs: Register, _rhs: Register) {}
        pub fn Modu64(&mut self, _dst: Register, _lhs: Register, _rhs: Register) {}
        pub fn add(&mut self, _dst: Register, _lhs: Register, _rhs: Register) {}
        pub fn sub(&mut self, _dst: Register, _lhs: Register, _rhs: Register) {}
        pub fn sll(&mut self, _dst: Register, _src: Register, _amount: Register) {}
        pub fn sra(&mut self, _dst: Register, _src: Register, _amount: Register) {}
        pub fn srl(&mut self, _dst: Register, _src: Register, _amount: Register) {}
        pub fn Sltu(&mut self, _dst: Register, _src: Register, _one: i32) {}
        pub fn AllocateStackSpace(&mut self, _i: i32) {}
        pub fn SignExtendWord(&mut self, _dst: Register, _src: Register) {}
        pub fn ZeroExtendWord(&mut self, _dst: Register, _src: Register) {}
        pub fn BranchShort(&mut self, _label: &Label, _ne: Condition, _temp2: Register, _operand: Operand) {}
        pub fn Branch(&mut self, _label: &Label) {}
        pub fn Jump(&mut self, _target: Register) {}
        pub fn CompareTaggedAndBranch(&mut self, _label: &Label, _cond: Condition, _lhs: Register, _operand: Operand) {}
        pub fn ExtractLowWordFromF64(&mut self, _dst: Register, _src: DoubleRegister) {}
        pub fn fmv_x_d(&mut self, _dst: Register, _src: DoubleRegister) {}
        pub fn Cvt_s_w(&mut self, _dst: DoubleRegister, _src: Register) {}
        pub fn Cvt_s_uw(&mut self, _dst: DoubleRegister, _src: Register) {}
        pub fn fmv_w_x(&mut self, _dst: DoubleRegister, _src: Register) {}
        pub fn Cvt_d_w(&mut self, _dst: DoubleRegister, _src: Register) {}
        pub fn Cvt_d_uw(&mut self, _dst: DoubleRegister, _src: Register) {}
        pub fn fcvt_d_s(&mut self, _dst: DoubleRegister, _src: DoubleRegister) {}
        pub fn fmv_d_x(&mut self, _dst: DoubleRegister, _src: Register) {}
        pub fn fcvt_w_s(&mut self, _dst: Register, _src: DoubleRegister, _rtz: RoundingMode) {}
        pub fn fcvt_wu_s(&mut self, _dst: Register, _src: DoubleRegister, _rtz: RoundingMode) {}
        pub fn fcvt_w_d(&mut self, _dst: Register, _src: DoubleRegister, _rtz: RoundingMode) {}
        pub fn fcvt_wu_d(&mut self, _dst: Register, _src: DoubleRegister, _rtz: RoundingMode) {}
        pub fn fcvt_l_s(&mut self, _dst: Register, _src: DoubleRegister, _rtz: RoundingMode) {}
        pub fn fcvt_lu_s(&mut self, _dst: Register, _src: DoubleRegister, _rtz: RoundingMode) {}
        pub fn fcvt_l_d(&mut self, _dst: Register, _src: DoubleRegister, _rtz: RoundingMode) {}
        pub fn fcvt_lu_d(&mut self, _dst: Register, _src: DoubleRegister, _rtz: RoundingMode) {}
        pub fn vslidedown_vi(&mut self, _dst: VRegister, _src: VRegister, _laneidx: u8) {}
        pub fn Clear_if_nan_s(&mut self, _dst: Register, _src: DoubleRegister) {}
        pub fn Clear_if_nan_d(&mut self, _dst: Register, _src: DoubleRegister) {}
        pub fn vmv_vx(&mut self, _dst: VRegister, _src: Register) {}
        pub fn vrgather_vv(&mut self, _dst: VRegister, _src: VRegister, _indices: VRegister) {}
        pub fn vwadd_vv(&mut self, _dst: VRegister, _op1: VRegister, _op2: VRegister) {}
        pub fn vwaddu_vv(&mut self, _dst: VRegister, _op1: VRegister, _op2: VRegister) {}
        pub fn vxor_vv(&mut self, _v: VRegister, _v1: VRegister, _v2: VRegister) {}
        pub fn vmerge_vx(&mut self, _dst: VRegister, _val: Register, _src: VRegister) {}
        pub fn vmv_xs(&mut self, _dst: Register, _src: VRegister) {}
        pub fn CallCWithStackBuffer(&mut self, _args: &std::initializer_list::InitializerList<VarState>, _rets: *const LiftoffRegister, _return_kind: ValueKind, _out_argument_kind: ValueKind, _stack_bytes: i32, _ext_ref: ExternalReference) {}
        pub fn PrepareCallCFunction(&mut self, _num_args: i32, _scratch: Register) {}
        pub fn CallCFunction(&mut self, _ext_ref: ExternalReference, _num_args: i32) {}
        pub fn ParallelMove(&mut self) -> ParallelMove { ParallelMove {} }
    }

    pub struct VU {}
    impl VU {
        pub fn set(&mut self, _scratch_reg: Register, _e8: VSew, _m1: m1) {}
    }

    pub struct UseScratchRegisterScope<'a> {
        _assm: &'a LiftoffAssembler,
    }

    impl<'a> UseScratchRegisterScope<'a> {
        pub fn Acquire(&self) -> Register {
            Register {}
        }
    }

    pub struct BlockPoolsScope {}

    pub enum m1 {}
    pub enum VSew { E8, E16, E32, E64 }
    pub struct Register {}
    pub struct DoubleRegister {}
    pub struct VRegister {}

    pub enum Condition {
        eq,
        ne,
    }

    pub enum SaveFPRegsMode {
        kSave,
    }

    pub enum StubCallMode {
        kCallWasmRuntimeStub,
    }

    pub struct Label {}

    pub struct WasmValue {}

    pub struct LoadType {}
    impl LoadType {
        pub fn value(&self) -> i32 { 0 }
        pub fn size_log_2(&self) -> u32 { 0 }
        pub fn mem_type(&self) -> MachineType {MachineType::Int32()}
    }

    pub struct StoreType {}
    impl StoreType {
      pub fn value(&self) -> i32 { 0 }
      pub fn mem_rep(&self) -> MachineRepresentation { MachineRepresentation::kWord32 }
    }

    pub struct LiftoffRegList {}
    impl LiftoffRegList {
        pub fn set(&mut self, _arg: Register) -> Register { Register {} }
    }

    pub enum Binop { kAdd, kSub, kAnd, kOr, kXor, kExchange }

    pub struct RoundingMode {}
    const RTZ: RoundingMode = RoundingMode {};

    pub struct ExternalReference {}

    pub enum LoadTransformationKind { kExtend, kZeroExtend, kSplat }

    pub enum MachineType {
      Int8,
      Int16,
      Int32,
      Int64,
      Uint8,
      Uint16,
      Uint32,
    }

    pub enum MachineRepresentation {
      kWord8,
      kWord16,
      kWord32,
      kWord64,
    }

    pub struct VarState {
        loc_: i32,
        kind_: ValueKind
    }

    impl VarState {
        pub fn is_const(&self) -> bool { self.loc_ == 0 }
        pub fn is_reg(&self) -> bool { self.loc_ == 1 }
        pub fn is_stack(&self) -> bool { self.loc_ == 2 }
        pub fn i32_const(&self) -> i32 { 0 }
        pub fn kind(&self) -> ValueKind { self.kind_ }
        pub fn reg(&self) -> LiftoffRegister { LiftoffRegister {} }
        pub fn offset(&self) -> i32 { 0 }
        pub fn loc(&self) -> i32 { self.loc_ }
    }

    pub struct FreezeCacheState {}

    const Mask: i32 = 0;

    pub mod liftoff {
        use super::*;

        pub fn GetMemOp(
            assm: &mut LiftoffAssembler,
            addr: Register,
            offset: Register,
            offset_imm: usize,
            i64_offset: bool,
            shift_amount: u32,
        ) -> MemOperand {
            MemOperand {}
        }

        pub fn Load(assm: &mut LiftoffAssembler, dst: LiftoffRegister, src: MemOperand, kind: ValueKind) {

        }

        pub fn Store(assm: &mut LiftoffAssembler, base: Register, offset: i32, src: LiftoffRegister, kind: ValueKind) {

        }

        pub fn GetStackSlot(offset: i32) -> MemOperand { MemOperand {} }

        pub fn StoreToMemory(assm: &mut LiftoffAssembler, dst: MemOperand, src: &LiftoffAssembler::VarState) {

        }

        pub fn ChangeEndiannessLoad(
            _assm: &mut LiftoffAssembler,
            _dst: LiftoffRegister,
            _type: LoadType,
            _pinned: LiftoffRegList,
        ) {
        }
        pub fn ChangeEndiannessStore(
            _assm: &mut LiftoffAssembler,
            _src: LiftoffRegister,
            _type: StoreType,
            _pinned: LiftoffRegList,
        ) {
        }

        pub fn CalculateActualAddress(
            _lasm: &mut LiftoffAssembler,
            _temps: &mut UseScratchRegisterScope,
            addr_reg: Register,
            offset_reg: Register,
            offset_imm: usize,
        ) -> Register {
            Register {}
        }
    }

    impl LiftoffAssembler {
        pub fn LoadConstant(&mut self, reg: LiftoffRegister, value: WasmValue) {}

        pub fn LoadTaggedPointer(
            &mut self,
            dst: Register,
            src_addr: Register,
            offset_reg: Register,
            offset_imm: i32,
            protected_load_pc: *mut u32,
            needs_shift: bool,
        ) {
        }

        pub fn LoadProtectedPointer(&mut self, dst: Register, src_addr: Register, offset_imm: i32) {}

        pub fn LoadFullPointer(&mut self, dst: Register, src_addr: Register, offset_imm: i32) {}

        #[cfg(V8_ENABLE_SANDBOX)]
        pub fn LoadCodeEntrypointViaCodePointer(
            &mut self,
            dst: Register,
            src_addr: Register,
            offset_imm: i32,
        ) {
        }

        pub fn StoreTaggedPointer(
            &mut self,
            dst_addr: Register,
            offset_reg: Register,
            offset_imm: i32,
            src: Register,
            pinned: LiftoffRegList,
            protected_store_pc: *mut u32,
            skip_write_barrier: SkipWriteBarrier,
        ) {
        }

        pub fn Load(
            &mut self,
            dst: LiftoffRegister,
            src_addr: Register,
            offset_reg: Register,
            offset_imm: usize,
            type_: LoadType,
            protected_load_pc: *mut u32,
            is_load_mem: bool,
            i64_offset: bool,
            needs_shift: bool,
        ) {
        }

        pub fn Store(
            &mut self,
            dst_addr: Register,
            offset_reg: Register,
            offset_imm: usize,
            src: LiftoffRegister,
            type_: StoreType,
            pinned: LiftoffRegList,
            protected_store_pc: *mut u32,
            is_store_mem: bool,
            i64_offset: bool,
        ) {
        }

        pub fn AtomicLoad(
            &mut self,
            dst: LiftoffRegister,
            src_addr: Register,
            offset_reg: Register,
            offset_imm: usize,
            type_: LoadType,
            pinned: LiftoffRegList,
            i64_offset: bool,
        ) {
        }

        pub fn AtomicStore(
            &mut self,
            dst_addr: Register,
            offset_reg: Register,
            offset_imm: usize,
            src: LiftoffRegister,
            type_: StoreType,
            pinned: LiftoffRegList,
            i64_offset: bool,
        ) {
        }

        pub fn AtomicAdd(
            &mut self,
            dst_addr: Register,
            offset_reg: Register,
            offset_imm: usize,
            value: LiftoffRegister,
            result: LiftoffRegister,
            type_: StoreType,
            i64_offset: bool,
        ) {
        }

        pub fn AtomicSub(
            &mut self,
            dst_addr: Register,
            offset_reg: Register,
            offset_imm: usize,
            value: LiftoffRegister,
            result: LiftoffRegister,
            type_: StoreType,
            i64_offset: bool,
        ) {
        }

        pub fn AtomicAnd(
            &mut self,
            dst_addr: Register,
            offset_reg: Register,
            offset_imm: usize,
            value: LiftoffRegister,
            result: LiftoffRegister,
            type_: StoreType,
            i64_offset: bool,
        ) {
        }

        pub fn AtomicOr(
            &mut self,
            dst_addr: Register,
            offset_reg: Register,
            offset_imm: usize,
            value: LiftoffRegister,
            result: LiftoffRegister,
            type_: StoreType,
            i64_offset: bool,
        ) {
        }

        pub fn AtomicXor(
            &mut self,
            dst_addr: Register,
            offset_reg: Register,
            offset_imm: usize,
            value: LiftoffRegister,
            result: LiftoffRegister,
            type_: StoreType,
            i64_offset: bool,
        ) {
        }

        pub fn AtomicExchange(
            &mut self,
            dst_addr: Register,
            offset_reg: Register,
            offset_imm: usize,
            value: LiftoffRegister,
            result: LiftoffRegister,
            type_: StoreType,
            i64_offset: bool,
        ) {
        }

        pub fn AtomicCompareExchange(
            &mut self,
            dst_addr: Register,
            offset_reg: Register,
            offset_imm: usize,
            expected: LiftoffRegister,
            new_value: LiftoffRegister,
            result: LiftoffRegister,
            type_: StoreType,
            i64_offset: bool,
        ) {
        }

        pub fn AtomicFence(&mut self) {}

        pub fn LoadCallerFrameSlot(
            &mut self,
            dst: LiftoffRegister,
            caller_slot_idx: u32,
            kind: ValueKind,
        ) {
        }

        pub fn StoreCallerFrameSlot(
            &mut self,
            src: LiftoffRegister,
            caller_slot_idx: u32,
            kind: ValueKind,
            frame_pointer: Register,
        ) {
        }

        pub fn LoadReturnStackSlot(&mut self, dst: LiftoffRegister, offset: i32, kind: ValueKind) {}

        pub fn MoveStackValue(&mut self, dst_offset: u32, src_offset: u32, kind: ValueKind) {}

        pub fn Move(&mut self, dst: Register, src: Register, kind: ValueKind) {}

        pub fn MoveDouble(&mut self, dst: DoubleRegister, src: DoubleRegister, kind: ValueKind) {}

        pub fn Spill(&mut self, offset: i32, reg: LiftoffRegister, kind: ValueKind) {}

        pub fn SpillValue(&mut self, offset: i32, value: WasmValue) {}

        pub fn Fill(&mut self, reg: LiftoffRegister, offset: i32, kind: ValueKind) {}

        pub fn FillI64Half(&mut self, _arg0: Register, _offset: i32, _arg2: RegPairHalf) {
        }

        pub fn FillStackSlotsWithZero(&mut self, start: i32, size: i32) {}

        pub fn emit_i64_clz(&mut self, dst: LiftoffRegister, src: LiftoffRegister) {}

        pub fn emit_i64_ctz(&mut self, dst: LiftoffRegister, src: LiftoffRegister) {}

        pub fn emit_i64_popcnt(&mut self, dst: LiftoffRegister, src: LiftoffRegister) -> bool {
            false
        }

        pub fn emit_i32_mul(&mut self, dst: Register, lhs: Register, rhs: Register) {}

        pub fn emit_i32_divs(
            &mut self,
            dst: Register,
            lhs: Register,
            rhs: Register,
            trap_div_by_zero: *mut Label,
            trap_div_unrepresentable: *mut Label,
        ) {
        }

        pub fn emit_i32_divu(
            &mut self,
            dst: Register,
            lhs: Register,
            rhs: Register,
            trap_div_by_zero: *mut Label,
        ) {
        }

        pub fn emit_i32_rems(
            &mut self,
            dst: Register,
            lhs: Register,
            rhs: Register,
            trap_div_by_zero: *mut Label,
        ) {
        }

        pub fn emit_i32_remu(
            &mut self,
            dst: Register,
            lhs: Register,
            rhs: Register,
            trap_div_by_zero: *mut Label,
        ) {
        }

        pub fn emit_i32_addi(&mut self, dst: Register, lhs: Register, imm: i32) {}
        pub fn emit_i32_subi(&mut self, dst: Register, lhs: Register, imm: i32) {}
        pub fn emit_i32_andi(&mut self, dst: Register, lhs: Register, imm: i32) {}
        pub fn emit_i32_ori(&mut self, dst: Register, lhs: Register, imm: i32) {}
        pub fn emit_i32_xori(&mut self, dst: Register, lhs: Register, imm: i32) {}

        pub fn emit_i32_add(&mut self, dst: Register, lhs: Register, rhs: Register) {}
        pub fn emit_i32_sub(&mut self, dst: Register, lhs: Register, rhs: Register) {}
        pub fn emit_i32_and(&mut self, dst: Register, lhs: Register, rhs: Register) {}
        pub fn emit_i32_or(&mut self, dst: Register, lhs: Register, rhs: Register) {}
        pub fn emit_i32_xor(&mut self, dst: Register, lhs: Register, rhs: Register) {}

        pub fn emit_i32_clz(&mut self, dst: Register, src: Register) {}

        pub fn emit_i32_ctz(&mut self, dst: Register, src: Register) {}

        pub fn emit_i32_popcnt(&mut self, dst: Register, src: Register) -> bool {
            false
        }

        pub fn emit_i32_shl(&mut self, dst: Register, src: Register, amount: Register) {}
        pub fn emit_i32_sar(&mut self, dst: Register, src: Register, amount: Register) {}
        pub fn emit_i32_shr(&mut self, dst: Register, src: Register, amount: Register) {}

        pub fn emit_i32_shli(&mut self, dst: Register, src: Register, amount: i32) {}
        pub fn emit_i32_sari(&mut self, dst: Register, src: Register, amount: i32) {}
        pub fn emit_i32_shri(&mut self, dst: Register, src: Register, amount: i32) {}

        pub fn emit_i64_mul(&mut self, dst: LiftoffRegister, lhs: LiftoffRegister, rhs: LiftoffRegister) {}

        pub fn emit_i64_muli(&mut self, dst: LiftoffRegister, lhs: LiftoffRegister, imm: i32) {}

        pub fn emit_i64_divs(
            &mut self,
            dst: LiftoffRegister,
            lhs: LiftoffRegister,
            rhs: LiftoffRegister,
            trap_div_by_zero: *mut Label,
            trap_div_unrepresentable: *mut Label,
        ) -> bool {
            false
        }

        pub fn emit_i64_divu(
            &mut self,
            dst: LiftoffRegister,
            lhs: LiftoffRegister,
            rhs: LiftoffRegister,
            trap_div_by_zero: *mut Label,
        ) -> bool {
            false
        }

        pub fn emit_i64_rems(
            &mut self,
            dst: LiftoffRegister,
            lhs: LiftoffRegister,
            rhs: LiftoffRegister,
            trap_div_by_zero: *mut Label,
        ) -> bool {
            false
        }

        pub fn emit_i64_remu(
            &mut self,
            dst: LiftoffRegister,
            lhs: LiftoffRegister,
            rhs: LiftoffRegister