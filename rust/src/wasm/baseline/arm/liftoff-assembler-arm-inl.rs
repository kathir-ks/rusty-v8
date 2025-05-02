// This conversion is incomplete and represents a best-effort translation
// based on the provided C++ header file. Due to the lack of context
// and implementation details, some parts may be stubbed or require
// further refinement.

// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(clippy::missing_safety_doc)]

// Placeholder modules for external dependencies.  These would need
// to be replaced with actual crate dependencies or implementations.

mod codegen_arm {
    pub mod assembler_arm_inl;
    pub mod register_arm;
}

mod codegen {
    pub mod interface_descriptors_inl;
}

mod common {
    pub mod globals;
}

mod heap {
    pub mod mutable_page_metadata;
}

mod wasm {
    pub mod baseline {
        pub mod liftoff_assembler;
        pub mod liftoff_register;
        pub mod parallel_move_inl;
        pub mod arm {
            pub mod liftoff_assembler_arm_inl;
        }
    }
    pub mod object_access;
    pub mod wasm_linkage;
    pub mod wasm_objects;
}

mod base {
    pub mod bits;
}

use std::convert::TryInto;

pub mod liftoff_assembler_arm_inl {
    use super::*;
    use codegen_arm::assembler_arm_inl::*;
    use codegen_arm::register_arm::*;
    use common::globals::*;
    use std::mem;
    use wasm::baseline::liftoff_assembler::*;
    use wasm::baseline::liftoff_register::*;

    pub const K_HALF_STACK_SLOT_SIZE: i32 = LiftoffAssembler::K_STACK_SLOT_SIZE as i32 / 2;

    #[inline]
    pub fn get_stack_slot(offset: i32) -> MemOperand {
        MemOperand {
            rn: FP, // Assuming FP is a register
            offset: -offset,
            rm: NO_REG,
            shift: ShiftType::LSL,
            shift_amount: 0,
        }
    }

    #[inline]
    pub fn get_half_stack_slot(offset: i32, half: RegPairHalf) -> MemOperand {
        let half_offset = match half {
            RegPairHalf::KLowWord => 0,
            RegPairHalf::KHighWord => LiftoffAssembler::K_STACK_SLOT_SIZE as i32 / 2,
        };
        let base_reg = if offset > 0 { FP } else { SP }; // Assuming FP and SP are registers
        MemOperand {
            rn: base_reg,
            offset: -offset + half_offset,
            rm: NO_REG,
            shift: ShiftType::LSL,
            shift_amount: 0,
        }
    }

    #[inline]
    pub fn get_instance_data_operand() -> MemOperand {
        get_stack_slot(WasmLiftoffFrameConstants::K_INSTANCE_DATA_OFFSET as i32)
    }

    #[inline]
    pub fn get_mem_op(
        assm: &mut LiftoffAssembler,
        temps: &mut UseScratchRegisterScope,
        addr: Register,
        offset: Register,
        offset_imm: i32,
        shift_amount: u32,
    ) -> MemOperand {
        if offset != NO_REG {
            if offset_imm == 0 {
                return MemOperand {
                    rn: addr,
                    offset: 0, //offset as i32, // Cannot directly use a register as offset here.
                    rm: offset,
                    shift: ShiftType::LSL,
                    shift_amount,
                };
            }
            let tmp = temps.acquire();
            if shift_amount == 0 {
                assm.add(tmp, offset, Operand::Imm(offset_imm));
            } else {
                assm.lsl(tmp, offset, Operand::Imm(shift_amount));
                assm.add(tmp, tmp, Operand::Imm(offset_imm));
            }
            return MemOperand {
                rn: addr,
                offset: 0, //tmp as i32, // Cannot directly use a register as offset here.
                rm: tmp,
                shift: ShiftType::LSL,
                shift_amount: 0,
            };
        }
        MemOperand {
            rn: addr,
            offset: offset_imm,
            rm: NO_REG,
            shift: ShiftType::LSL,
            shift_amount: 0,
        }
    }

    #[inline]
    pub fn calculate_actual_address(
        assm: &mut LiftoffAssembler,
        temps: &mut UseScratchRegisterScope,
        addr_reg: Register,
        offset_reg: Register,
        offset_imm: usize,
        result_reg: Register,
    ) -> Register {
        if offset_reg == NO_REG && offset_imm == 0 {
            if result_reg == addr_reg || result_reg == NO_REG {
                return addr_reg;
            }
            assm.mov(result_reg, Operand::Reg(addr_reg));
            return result_reg;
        }
        let result_reg = if result_reg == NO_REG {
            temps.acquire()
        } else {
            result_reg
        };
        if offset_reg == NO_REG {
            assm.add(result_reg, addr_reg, Operand::Imm(offset_imm as i32));
        } else {
            assm.add(result_reg, addr_reg, Operand::Reg(offset_reg));
            if offset_imm != 0 {
                assm.add(result_reg, result_reg, Operand::Imm(offset_imm as i32));
            }
        }
        result_reg
    }

    #[inline]
    pub fn make_unsigned(cond: Condition) -> Condition {
        match cond {
            Condition::LessThan => Condition::UnsignedLessThan,
            Condition::LessThanEqual => Condition::UnsignedLessThanEqual,
            Condition::GreaterThan => Condition::UnsignedGreaterThan,
            Condition::GreaterThanEqual => Condition::UnsignedGreaterThanEqual,
            Condition::Equal => Condition::Equal,
            Condition::NotEqual => Condition::NotEqual,
            Condition::UnsignedLessThan => Condition::UnsignedLessThan,
            Condition::UnsignedLessThanEqual => Condition::UnsignedLessThanEqual,
            Condition::UnsignedGreaterThan => Condition::UnsignedGreaterThan,
            Condition::UnsignedGreaterThanEqual => Condition::UnsignedGreaterThanEqual,
            _ => panic!("UNREACHABLE"),
        }
    }

    pub fn i64_binop<
        F: Fn(&mut Assembler, Register, Register, SBit, Condition),
        F2: Fn(&mut Assembler, Register, Register, &Operand, SBit, Condition),
    >(
        assm: &mut LiftoffAssembler,
        dst: LiftoffRegister,
        lhs: LiftoffRegister,
        rhs: LiftoffRegister,
        op: F,
        op_with_carry: F2,
    ) {
        let mut dst_low = dst.low_gp();
        if dst_low == lhs.high_gp() || dst_low == rhs.high_gp() {
            dst_low = assm
                .get_unused_register(
                    RegClass::GpReg,
                    LiftoffRegList::from_regs(&[lhs, rhs, dst.high_gp()]),
                )
                .gp();
        }
        op(&mut assm.assembler, dst_low, lhs.low_gp(), SBit::SetCC, Condition::AL);
        op_with_carry(
            &mut assm.assembler,
            dst.high_gp(),
            lhs.high_gp(),
            &Operand::Reg(rhs.high_gp()),
            SBit::LeaveCC,
            Condition::AL,
        );
        if dst_low != dst.low_gp() {
            assm.mov(dst.low_gp(), Operand::Reg(dst_low));
        }
    }

    pub fn i64_binop_i<
        F: Fn(&mut Assembler, Register, Register, &Operand, SBit, Condition),
        F2: Fn(&mut Assembler, Register, Register, &Operand, SBit, Condition),
    >(
        assm: &mut LiftoffAssembler,
        dst: LiftoffRegister,
        lhs: LiftoffRegister,
        imm: i64,
        op: F,
        op_with_carry: F2,
    ) {
        assert_ne!(dst.low_gp(), lhs.high_gp());
        let imm_low_word = imm as i32;
        let imm_high_word = (imm >> 32) as i32;
        op(
            &mut assm.assembler,
            dst.low_gp(),
            lhs.low_gp(),
            &Operand::Imm(imm_low_word),
            SBit::SetCC,
            Condition::AL,
        );
        op_with_carry(
            &mut assm.assembler,
            dst.high_gp(),
            lhs.high_gp(),
            &Operand::Imm(imm_high_word),
            SBit::LeaveCC,
            Condition::AL,
        );
    }

    pub fn i64_shiftop<F: Fn(&mut MacroAssembler, Register, Register, Register, Register, Register)>(
        assm: &mut LiftoffAssembler,
        dst: LiftoffRegister,
        src: LiftoffRegister,
        amount: Register,
        op: F,
        is_left_shift: bool,
    ) {
        let src_low = src.low_gp();
        let src_high = src.high_gp();
        let dst_low = dst.low_gp();
        let dst_high = dst.high_gp();

        let clobbered_dst_reg = if is_left_shift { dst_high } else { dst_low };
        let mut pinned = LiftoffRegList::from_regs(&[LiftoffRegister::from_gp(clobbered_dst_reg), src]);

        let amount_capped = pinned.set(assm.get_unused_register(RegClass::GpReg, pinned)).gp();

        assm.and_(amount_capped, amount, Operand::Imm(0x3F));

        let later_src_reg = if is_left_shift { src_low } else { src_high };
        let mut later_src_reg = later_src_reg;

        if later_src_reg == clobbered_dst_reg {
            later_src_reg = assm.get_unused_register(RegClass::GpReg, pinned).gp();
            assm.assembler.move_(later_src_reg, clobbered_dst_reg);
        }
        op(
            &mut assm.assembler.masm,
            dst_low,
            dst_high,
            src_low,
            src_high,
            amount_capped,
        );
    }

    #[inline]
    pub fn get_float_register(reg: DoubleRegister) -> FloatRegister {
        assert!(reg.code() < DoubleRegister::D16.code());
        LowDwVfpRegister::from_code(reg.code()).low()
    }

    #[inline]
    pub fn get_simd128_register(reg: DoubleRegister) -> Simd128Register {
        QwNeonRegister::from_code(reg.code() / 2)
    }

    #[inline]
    pub fn get_simd128_register_liftoff(reg: LiftoffRegister) -> Simd128Register {
        get_simd128_register(reg.low_fp())
    }

    #[derive(PartialEq, Eq)]
    pub enum MinOrMax {
        KMin,
        KMax,
    }

    pub fn emit_float_min_or_max<RegisterType: Copy>(
        assm: &mut LiftoffAssembler,
        dst: RegisterType,
        lhs: RegisterType,
        rhs: RegisterType,
        min_or_max: MinOrMax,
    ) where RegisterType: RegisterTrait {
        assert!(RegisterType::k_size_in_bytes() == 4 || RegisterType::k_size_in_bytes() == 8);
        if lhs == rhs {
            assm.assembler.masm.move_(dst, lhs);
            return;
        }

        let mut done = Label::new();
        let mut is_nan = Label::new();

        if min_or_max == MinOrMax::KMin {
            assm.assembler.masm.float_min(dst, lhs, rhs, &mut is_nan);
        } else {
            assm.assembler.masm.float_max(dst, lhs, rhs, &mut is_nan);
        }
        assm.b(&mut done);
        assm.bind(&mut is_nan);

        assm.vadd(dst, lhs, rhs);
        assm.bind(&mut done);
    }

    #[inline]
    pub fn ensure_no_alias(
        assm: &mut Assembler,
        reg: Register,
        must_not_alias: Register,
        temps: &mut UseScratchRegisterScope,
    ) -> Register {
        if reg != must_not_alias {
            return reg;
        }
        let tmp = temps.acquire();
        assert_ne!(reg, tmp);
        assm.move_(tmp, reg);
        tmp
    }

    pub fn s128_narrow_op(
        assm: &mut LiftoffAssembler,
        dt: NeonDataType,
        sdt: NeonDataType,
        dst: LiftoffRegister,
        lhs: LiftoffRegister,
        rhs: LiftoffRegister,
    ) {
        if dst == lhs {
            assm.vqmovn(
                dt,
                sdt,
                dst.low_fp(),
                get_simd128_register_liftoff(lhs),
            );
            assm.vqmovn(
                dt,
                sdt,
                dst.high_fp(),
                get_simd128_register_liftoff(rhs),
            );
        } else {
            assm.vqmovn(
                dt,
                sdt,
                dst.high_fp(),
                get_simd128_register_liftoff(rhs),
            );
            assm.vqmovn(
                dt,
                sdt,
                dst.low_fp(),
                get_simd128_register_liftoff(lhs),
            );
        }
    }

    pub fn f64x2_compare(
        assm: &mut LiftoffAssembler,
        dst: LiftoffRegister,
        lhs: LiftoffRegister,
        rhs: LiftoffRegister,
        cond: Condition,
    ) {
        assert!(
            cond == Condition::Equal
                || cond == Condition::NotEqual
                || cond == Condition::LessThan
                || cond == Condition::LessThanEqual
        );

        let dest = get_simd128_register_liftoff(dst);
        let left = get_simd128_register_liftoff(lhs);
        let right = get_simd128_register_liftoff(rhs);

        let mut temps = UseScratchRegisterScope::new(assm);
        let scratch = temps.acquire();

        assm.mov(scratch, Operand::Imm(0));
        assm.vfp_compare_and_set_flags(left.low(), right.low());
        assm.mov(scratch, Operand::Imm(-1), LeaveCC, cond);
        if cond == Condition::LessThan || cond == Condition::LessThanEqual {
            // Check for NaN.
            assm.mov(scratch, Operand::Imm(0), LeaveCC, Condition::VS);
        }
        assm.vmov(dest.low(), scratch, scratch);

        assm.mov(scratch, Operand::Imm(0));
        assm.vfp_compare_and_set_flags(left.high(), right.high());
        assm.mov(scratch, Operand::Imm(-1), LeaveCC, cond);
        if cond == Condition::LessThan || cond == Condition::LessThanEqual {
            // Check for NaN.
            assm.mov(scratch, Operand::Imm(0), LeaveCC, Condition::VS);
        }
        assm.vmov(dest.high(), scratch, scratch);
    }

    pub fn store(assm: &mut LiftoffAssembler, src: LiftoffRegister, dst: MemOperand, kind: ValueKind) {
        // The {str} instruction needs a temp register when the immediate in the
        // provided MemOperand does not fit into 12 bits. This happens for large stack
        // frames. This assert checks that the temp register is available when needed.
        assert!(UseScratchRegisterScope::new(assm).can_acquire());

        match kind {
            ValueKind::I16 => assm.strh(src.gp(), dst),
            ValueKind::I32 | ValueKind::RefNull | ValueKind::Ref => assm.str(src.gp(), dst),
            ValueKind::I64 => {
                // Positive offsets should be lowered to kI32.
                assm.str(src.low_gp(), MemOperand::new(dst.rn, dst.offset));
                assm.str(
                    src.high_gp(),
                    MemOperand::new(dst.rn, dst.offset + K_HALF_STACK_SLOT_SIZE),
                );
            }
            ValueKind::F32 => assm.vstr(get_float_register(src.fp()), dst),
            ValueKind::F64 => assm.vstr(src.fp(), dst),
            ValueKind::S128 => {
                let mut temps = UseScratchRegisterScope::new(assm);
                let addr = calculate_actual_address(assm, &mut temps, dst.rn, NO_REG, dst.offset as usize, NO_REG);
                assm.vst1(
                    Neon8,
                    NeonListOperand::new(src.low_fp(), 2),
                    NeonMemOperand::new(addr),
                );
            }
            _ => panic!("UNREACHABLE"),
        }
    }

    pub fn load(assm: &mut LiftoffAssembler, dst: LiftoffRegister, src: MemOperand, kind: ValueKind) {
        match kind {
            ValueKind::I16 => assm.ldrh(dst.gp(), src),
            ValueKind::I32 | ValueKind::RefNull | ValueKind::Ref => assm.ldr(dst.gp(), src),
            ValueKind::I64 => {
                assm.ldr(dst.low_gp(), MemOperand::new(src.rn, src.offset));
                assm.ldr(
                    dst.high_gp(),
                    MemOperand::new(src.rn, src.offset + K_HALF_STACK_SLOT_SIZE),
                );
            }
            ValueKind::F32 => assm.vldr(get_float_register(dst.fp()), src),
            ValueKind::F64 => assm.vldr(dst.fp(), src),
            ValueKind::S128 => {
                let mut temps = UseScratchRegisterScope::new(assm);
                let addr = calculate_actual_address(assm, &mut temps, src.rn, NO_REG, src.offset as usize, NO_REG);
                assm.vld1(
                    Neon8,
                    NeonListOperand::new(dst.low_fp(), 2),
                    NeonMemOperand::new(addr),
                );
            }
            _ => panic!("UNREACHABLE"),
        }
    }

    pub const fn mask_from_neon_data_type(dt: NeonDataType) -> i32 {
        match dt {
            NeonDataType::NeonS8 | NeonDataType::NeonU8 => 7,
            NeonDataType::NeonS16 | NeonDataType::NeonU16 => 15,
            NeonDataType::NeonS32 | NeonDataType::NeonU32 => 31,
            NeonDataType::NeonS64 | NeonDataType::NeonU64 => 63,
            _ => panic!("UNREACHABLE"),
        }
    }

    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum ShiftDirection {
        KLeft,
        KRight,
    }

    pub fn emit_simd_shift<const DIR: ShiftDirection, const DT: NeonDataType, const SZ: NeonSize>(
        assm: &mut LiftoffAssembler,
        dst: LiftoffRegister,
        lhs: LiftoffRegister,
        rhs: LiftoffRegister,
    ) {
        let mask = mask_from_neon_data_type(DT);
        let mut temps = UseScratchRegisterScope::new(assm);
        let tmp = temps.acquire_q();
        let shift = temps.acquire();

        assm.and_(shift, rhs.gp(), Operand::Imm(mask));
        assm.vdup(SZ, tmp, shift);

        if DIR == ShiftDirection::KRight {
            assm.vneg(SZ, tmp, tmp);
        }

        assm.vshl(DT, get_simd128_register_liftoff(dst), get_simd128_register_liftoff(lhs), tmp);
    }

    pub fn emit_simd_shift_immediate<const DIR: ShiftDirection, const DT: NeonDataType>(
        assm: &mut LiftoffAssembler,
        dst: LiftoffRegister,
        lhs: LiftoffRegister,
        rhs: i32,
    ) {
        let shift = rhs & mask_from_neon_data_type(DT);
        if shift != 0 {
            if DIR == ShiftDirection::KLeft {
                assm.vshl(DT, get_simd128_register_liftoff(dst), get_simd128_register_liftoff(lhs), shift);
            } else {
                assm.vshr(DT, get_simd128_register_liftoff(dst), get_simd128_register_liftoff(lhs), shift);
            }
        } else if dst != lhs {
            assm.vmov(get_simd128_register_liftoff(dst), get_simd128_register_liftoff(lhs));
        }
    }

    pub fn emit_any_true(assm: &mut LiftoffAssembler, dst: LiftoffRegister, src: LiftoffRegister) {
        let mut temps = UseScratchRegisterScope::new(assm);
        let scratch = temps.acquire_d();
        assm.vpmax(NeonDataType::NeonU32, scratch, src.low_fp(), src.high_fp());
        assm.vpmax(NeonDataType::NeonU32, scratch, scratch, scratch);
        assm.extract_lane(dst.gp(), scratch, NeonDataType::NeonS32, 0);
        assm.cmp(dst.gp(), Operand::Imm(0));
        assm.mov(dst.gp(), Operand::Imm(1), LeaveCC, Condition::NE);
    }

    pub struct CacheStatePreservingTempRegisters<'a> {
        assm_: &'a mut LiftoffAssembler,
        pinned_: LiftoffRegList,
        must_pop_: RegList,
    }

    impl<'a> CacheStatePreservingTempRegisters<'a> {
        pub fn new(assm: &'a mut LiftoffAssembler, pinned: LiftoffRegList) -> Self {
            CacheStatePreservingTempRegisters {
                assm_: assm,
                pinned_: pinned,
                must_pop_: RegList::empty(),
            }
        }

        pub fn acquire(&mut self) -> Register {
            if self.assm_.cache_state_.has_unused_register(RegClass::GpReg, self.pinned_) {
                return self
                    .pinned_
                    .set(self.assm_.cache_state_.unused_register(RegClass::GpReg, self.pinned_))
                    .gp();
            }

            let available = K_LIFTOFF_ASSEMBLER_GP_CACHE_REGS - self.pinned_.get_gp_list() - self.must_pop_;
            assert!(!available.is_empty());
            let reg = available.iter().last().unwrap(); // Use last() to iterate forwards in the destructor
            self.assm_.push(reg);
            self.must_pop_.set(reg);
            reg
        }
    }

    impl<'a> Drop for CacheStatePreservingTempRegisters<'a> {
        fn drop(&mut self) {
            for reg in self.must_pop_.iter() {
                self.assm_.pop(reg);
            }
        }
    }
}

impl LiftoffAssembler {
    pub fn prepare_stack_frame(&mut self) -> i32 {
        if !CpuFeatures::is_supported(CpuFeature::Armv7) {
            self.bailout(BailoutReason::UnsupportedArchitecture, "Liftoff needs ARMv7");
            return 0;
        }
        let offset = self.pc_offset() as u32;
        for _ in 0..K_PATCH_INSTRUCTIONS_REQUIRED {
            self.nop();
        }
        assert_eq!(
            offset + (K_PATCH_INSTRUCTIONS_REQUIRED * kInstrSize) as u32,
            self.pc_offset() as u32
        );
        offset as i32
    }

    pub fn call_frame_setup_stub(&mut self, declared_function_index: i32) {
        // The standard library used by gcc tryjobs does not consider `std::find` to be
        // `constexpr`, so wrap it in a `#ifdef __clang__` block.
        // #ifdef __clang__
        // static_assert(std::find(std::begin(wasm::kGpParamRegisters),
        //                           std::end(wasm::kGpParamRegisters),
        //                           kLiftoffFrameSetupFunctionReg) ==
        //                std::end(wasm::kGpParamRegisters));
        // #endif

        // On ARM, we must push at least {lr} before calling the stub, otherwise
        // it would get clobbered with no possibility to recover it.
        let scratch = R7;
        self.mov(
            scratch,
            Operand::Imm(StackFrame::TypeToMarker(StackFrameType::WASM) as i32),
        );
        self.push_common_frame(scratch);
        self.load_constant(
            LiftoffRegister::from_gp(K_LIFTOFF_FRAME_SETUP_FUNCTION_REG),
            WasmValue::from_i32(declared_function_index),
        );
        self.call_builtin(Builtin::kWasmLiftoffFrameSetup);
    }

    pub fn prepare_tail_call(&mut self, num_callee_stack_params: i32, stack_param_delta: i32) {
        let mut temps = UseScratchRegisterScope::new(self);
        let scratch = temps.acquire();

        self.sub(SP, SP, Operand::Imm(8));
        self.ldr(scratch, MemOperand::new(FP, 4));
        self.str(scratch, MemOperand::new(SP, 4));
        self.ldr(scratch, MemOperand::new(FP, 0));
        self.str(scratch, MemOperand::new(SP, 0));

        let slot_count = num_callee_stack_params + 2;
        for i in (0..slot_count).rev() {
            self.ldr(scratch, MemOperand::new(SP, i * 4));
            self.str(scratch, MemOperand::new(FP, (i - stack_param_delta) * 4));
        }

        self.sub(SP, FP, Operand::Imm(stack_param_delta * 4));
        self.pop(LR, FP);
    }

    pub fn align_frame_size(&mut self) {}

    pub fn patch_prepare_stack_frame(
        &mut self,
        offset: i32,
        safepoint_table_builder: &mut SafepointTableBuilder,
        feedback_vector_slot: bool,
        stack_param_slots: usize,
    ) {
        let mut frame_size = self.get_total_frame_size() - 2 * kSystemPointerSize;

        if feedback_vector_slot {
            frame_size -= kSystemPointerSize;
        }

        let mut patching_assembler = PatchingAssembler::new(
            AssemblerOptions {},
            self.buffer_start_.as_mut_ptr().wrapping_add(offset as usize),
            liftoff_assembler_arm_inl::K_PATCH_INSTRUCTIONS_REQUIRED,
        );

        if frame_size < 4 * 1024 {
            patching_assembler.sub(SP, SP, Operand::Imm(frame_size));
            patching_assembler.pad_with_nops();
            return;
        }

        patching_assembler.b(
            (self.pc_offset() - offset - Instruction::K_PC_LOAD_DELTA as i32)
                .try_into()
                .unwrap(),
        );
        patching_assembler.pad_with_nops();

        let mut continuation = Label::new();

        if frame_size < *V8_FLAGS.stack_size * 1024 {
            let mut temps = UseScratchRegisterScope::new(self);
            let stack_limit = temps.acquire();
            self.load_stack_limit(stack_limit, StackLimitKind::KRealStackLimit);
            self.add(stack_limit, stack_limit, Operand::Imm(frame_size));
            self.cmp(SP, Operand::Reg(stack_limit));
            self.b(&mut continuation, Condition::CS);
        }

        if *V8_FLAGS.experimental_wasm_growable_stacks {
            let mut regs_to_save = LiftoffRegList::empty();
            regs_to_save.set(WasmHandleStackOverflowDescriptor::GapRegister());
            regs_to_save.set(WasmHandleStackOverflowDescriptor::FrameBaseRegister());
            for reg in kGpParamRegisters {
                regs_to_save.set(reg);
            }
            for reg in kFpParamRegisters {
                regs_to_save.set(reg);
            }
            self.push_registers(regs_to_save);
            self.mov(
                WasmHandleStackOverflowDescriptor::GapRegister(),
                Operand::Imm(frame_size),
            );
            self.add(
                WasmHandleStackOverflowDescriptor::FrameBaseRegister(),
                FP,
                Operand::Imm(
                    stack_param_slots as i32 * kStackSlotSize + CommonFrameConstants::K_FIXED_FRAME_SIZE_ABOVE_FP,
                ),
            );
            self.call_builtin(Builtin::kWasmHandleStackOverflow);
            self.pop_registers(regs_to_save);
        } else {
            self.call(
                Builtin::kWasmStackOverflow as *const u8,
                RelocInfo::WASM_STUB_CALL,
            );
            safepoint_table_builder.define_safepoint(self);
            if *V8_FLAGS.debug_code {
                self.stop();
            }
        }

        self.bind(&mut continuation);

        self.allocate_stack_space(frame_size);

        let func_start_offset =
            offset + liftoff_assembler_arm_inl::K_PATCH_INSTRUCTIONS_REQUIRED * kInstrSize;
        self.b(
            (func_start_offset as i32 - self.pc_offset() - Instruction::K_PC_LOAD_DELTA as i32)
                .try_into()
                .unwrap(),
        );
    }

    pub fn finish_code(&mut self) {
        self.check_const_pool(true, false);
    }

    pub fn abort_compilation(&mut self) {
        self.aborted_code_generation();
    }

    pub const fn static_stack_frame_size() -> i32 {
        WasmLiftoffFrameConstants::K_FEEDBACK_VECTOR_OFFSET as i32
    }

    pub fn slot_size_for_type(kind: ValueKind) -> i32 {
        match kind {
            ValueKind::S128 => value_kind_size(kind) as i32,
            _ => kStackSlotSize as i32,
        }
    }

    pub fn needs_alignment(kind: ValueKind) -> bool {
        kind == ValueKind::S128 || is_reference(kind)
    }

    pub fn check_tier_up(
        &mut self,
        declared_func_index: i32,
        budget_used: i32,
        ool_label: &mut Label,
        frozen: &FreezeCacheState,
    ) {
        {
            let mut temps = liftoff_assembler_arm_inl::CacheStatePreservingTempRegisters::new(self, LiftoffRegList::empty());
            let budget_array = temps.acquire();

            let mut instance_data = self.cache_state_.cached_instance_data;
            if instance_data == NO_REG {
                instance_data = budget_array; // Reuse the temp register.
                self.load_instance_data_from_frame(instance_data);
            }

            const K_ARRAY_OFFSET: i32 = wasm::object_access::to_tagged(
                WasmTrustedInstanceData::K_TIERING_BUDGET_ARRAY_OFFSET,
            ) as i32;
            self.ldr(budget_array, MemOperand::new(instance_data, K_ARRAY_OFFSET));

            let mut budget_arr_offset = kInt32Size as i32 * declared_func_index;
            if !self.immediate_fits_addr_mode2_