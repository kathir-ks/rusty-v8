// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file is a Rust translation of /home/kathirks_gc/v8_go/codebase/src/wasm/baseline/riscv/liftoff-assembler-riscv32-inl.h

pub mod liftoff_assembler_riscv32_inl {
    use crate::heap::mutable_page_metadata::MemoryChunk;
    use crate::wasm::baseline::liftoff_assembler::*;
    use crate::wasm::baseline::riscv::liftoff_assembler_riscv_inl::*;
    use crate::wasm::wasm_objects::*;
    //use crate::codegen::*;

    // Liftoff Frames.
    //
    //  slot      Frame
    //       +--------------------+---------------------------
    //  n+4  | optional padding slot to keep the stack 16 byte aligned.
    //  n+3  |   parameter n      |
    //  ...  |       ...          |
    //   4   |   parameter 1      | or parameter 2
    //   3   |   parameter 0      | or parameter 1
    //   2   |  (result address)  | or parameter 0
    //  -----+--------------------+---------------------------
    //   1   | return addr (ra)   |
    //   0   | previous frame (fp)|
    //  -----+--------------------+  <-- frame ptr (fp)
    //  -1   | StackFrame::WASM   |
    //  -2   |     instance       |
    //  -3   |     feedback vector|
    //  -----+--------------------+---------------------------
    //  -4   |     slot 0         |   ^
    //  -5   |     slot 1         |   |
    //       |                    | Frame slots
    //       |                    |   |
    //       |                    |   v
    //       | optional padding slot to keep the stack 16 byte aligned.
    //  -----+--------------------+  <-- stack ptr (sp)
    //

    #[cfg(target_endian = "big")]
    pub const K_LOW_WORD_OFFSET: i32 = 4;
    #[cfg(target_endian = "big")]
    pub const K_HIGH_WORD_OFFSET: i32 = 0;

    #[cfg(target_endian = "little")]
    pub const K_LOW_WORD_OFFSET: i32 = 0;
    #[cfg(target_endian = "little")]
    pub const K_HIGH_WORD_OFFSET: i32 = 4;

    pub fn get_half_stack_slot(offset: i32, half: RegPairHalf) -> MemOperand {
        let half_offset = if half == RegPairHalf::LowWord {
            0
        } else {
            LiftoffAssembler::K_STACK_SLOT_SIZE / 2
        };
        if offset > 0 {
            MemOperand::new(Register::Fp, -offset + half_offset) // Assuming Fp and Sp are Registers
        } else {
            MemOperand::new(Register::Sp, -offset + half_offset) // Assuming Fp and Sp are Registers
        }
    }

    pub fn get_mem_op(
        assm: &mut LiftoffAssembler,
        addr: Register,
        offset: Register,
        offset_imm: usize,
        shift_amount: u32,
    ) -> MemOperand {
        debug_assert_ne!(addr, Register::ScratchReg2);
        debug_assert_ne!(offset, Register::ScratchReg2);
        if offset_imm <= u32::MAX as usize {
            let offset_imm32 = offset_imm as i32;
            if offset == Register::NoReg {
                return MemOperand::new(addr, offset_imm32);
            }
            if shift_amount != 0 {
                assm.calc_scaled_address(Register::ScratchReg2, addr, offset, shift_amount);
            } else {
                assm.add_word(Register::ScratchReg2, offset, addr);
            }
            return MemOperand::new(Register::ScratchReg2, offset_imm32);
        }
        // Offset immediate does not fit in 31 bits.
        assm.li(Register::ScratchReg2, offset_imm as i64);
        assm.add_word(Register::ScratchReg2, Register::ScratchReg2, addr);
        if offset != Register::NoReg {
            if shift_amount != 0 {
                assm.calc_scaled_address(Register::ScratchReg2, Register::ScratchReg2, offset, shift_amount);
            } else {
                assm.add_word(Register::ScratchReg2, Register::ScratchReg2, offset);
            }
        }
        MemOperand::new(Register::ScratchReg2, 0)
    }

    pub fn load(assm: &mut LiftoffAssembler, dst: LiftoffRegister, base: Register, offset: i32, kind: ValueKind) {
        let src = MemOperand::new(base, offset);

        match kind {
            ValueKind::I32 | ValueKind::Ref | ValueKind::RefNull => {
                assm.lw(dst.gp(), src);
            }
            ValueKind::I64 => {
                assm.lw(
                    dst.low_gp(),
                    MemOperand::new(base, offset + K_LOW_WORD_OFFSET),
                );
                assm.lw(
                    dst.high_gp(),
                    MemOperand::new(base, offset + K_HIGH_WORD_OFFSET),
                );
            }
            ValueKind::F32 => {
                assm.load_float(dst.fp(), src);
            }
            ValueKind::F64 => {
                assm.load_double(dst.fp(), src);
            }
            ValueKind::S128 => {
               // assm.VU.set(Register::ScratchReg, E8, m1); // TODO: implement `VU`
                let src_reg = if src.offset() == 0 {
                    src.rm()
                } else {
                    Register::ScratchReg
                };
                if src.offset() != 0 {
                    assm.add_word(Register::ScratchReg, src.rm(), src.offset());
                }
                assm.vl(dst.fp().to_v(), src_reg, 0, VSew::E8);
            }
            _ => unreachable!(),
        }
    }

    pub fn store(
        assm: &mut LiftoffAssembler,
        base: Register,
        offset: i32,
        src: LiftoffRegister,
        kind: ValueKind,
    ) {
        let dst = MemOperand::new(base, offset);
        match kind {
            ValueKind::I32 | ValueKind::RefNull | ValueKind::Ref => {
                assm.sw(src.gp(), dst);
            }
            ValueKind::I64 => {
                assm.sw(
                    src.low_gp(),
                    MemOperand::new(base, offset + K_LOW_WORD_OFFSET),
                );
                assm.sw(
                    src.high_gp(),
                    MemOperand::new(base, offset + K_HIGH_WORD_OFFSET),
                );
            }
            ValueKind::F32 => {
                assm.store_float(src.fp(), dst);
            }
            ValueKind::F64 => {
                assm.store_double(src.fp(), dst);
            }
            ValueKind::S128 => {
                //assm.VU.set(Register::ScratchReg, E8, m1);
                let dst_reg = if dst.offset() == 0 {
                    dst.rm()
                } else {
                    Register::ScratchReg
                };
                if dst.offset() != 0 {
                    assm.add_word(Register::ScratchReg, dst.rm(), dst.offset());
                }
                assm.vs(src.fp().to_v(), dst_reg, 0, VSew::E8);
            }
            _ => unreachable!(),
        }
    }

    pub fn push(assm: &mut LiftoffAssembler, reg: LiftoffRegister, kind: ValueKind) {
        match kind {
            ValueKind::I32 | ValueKind::RefNull | ValueKind::Ref => {
                assm.addi(Register::Sp, Register::Sp, -(k_system_pointer_size as i32));
                assm.sw(reg.gp(), MemOperand::new(Register::Sp, 0));
            }
            ValueKind::I64 => {
                assm.push(reg.high_gp(), reg.low_gp());
            }
            ValueKind::F32 => {
                assm.addi(Register::Sp, Register::Sp, -(k_system_pointer_size as i32));
                assm.store_float(reg.fp(), MemOperand::new(Register::Sp, 0));
            }
            ValueKind::F64 => {
                assm.addi(Register::Sp, Register::Sp, -(k_double_size as i32));
                assm.store_double(reg.fp(), MemOperand::new(Register::Sp, 0));
            }
            ValueKind::S128 => {
               // assm.VU.set(Register::ScratchReg, E8, m1);
                assm.addi(
                    Register::Sp,
                    Register::Sp,
                    -(k_system_pointer_size as i32) * 4,
                );
                assm.vs(reg.fp().to_v(), Register::Sp, 0, VSew::E8);
            }
            _ => unreachable!(),
        }
    }

    pub fn ensure_no_alias(
        assm: &mut Assembler,
        reg: Register,
        must_not_alias: LiftoffRegister,
        temps: &mut UseScratchRegisterScope,
    ) -> Register {
        if reg != must_not_alias.low_gp() && reg != must_not_alias.high_gp() {
            return reg;
        }
        let tmp = temps.acquire();
        debug_assert_ne!(must_not_alias.low_gp(), tmp);
        debug_assert_ne!(must_not_alias.high_gp(), tmp);
        assm.mv(tmp, reg);
        tmp
    }
}

impl LiftoffAssembler {
    pub fn load_constant(&mut self, reg: LiftoffRegister, value: WasmValue) {
        match value.value_type().kind() {
            ValueKind::I32 => {
                self.li(reg.gp(), value.to_i32() as i64);
            }
            ValueKind::I64 => {
                let low_word = value.to_i64() as i32;
                let high_word = (value.to_i64() >> 32) as i32;
                self.li(reg.low_gp(), low_word as i64);
                self.li(reg.high_gp(), high_word as i64);
            }
            ValueKind::F32 => {
                self.load_fpr_immediate(reg.fp(), f32::from_bits(value.to_f32_boxed().get_bits()) as f64);
            }
            ValueKind::F64 => {
                self.load_fpr_immediate(reg.fp(), f64::from_bits(value.to_f64_boxed().get_bits()));
            }
            _ => unreachable!(),
        }
    }

    pub fn load_tagged_pointer(
        &mut self,
        dst: Register,
        src_addr: Register,
        offset_reg: Register,
        offset_imm: i32,
        protected_load_pc: &mut u32,
        needs_shift: bool,
    ) {
        assert_eq!(k_tagged_size, k_system_pointer_size);
        self.load(
            LiftoffRegister::new(dst),
            src_addr,
            offset_reg,
            offset_imm as usize,
            LoadType::I32Load,
            protected_load_pc,
            false,
            false,
            needs_shift,
        );
    }

    pub fn load_protected_pointer(&mut self, dst: Register, src_addr: Register, offset: i32) {
        assert!(!V8_ENABLE_SANDBOX_BOOL);
        self.load_tagged_pointer(dst, src_addr, Register::NoReg, offset, &mut 0, false);
    }

    pub fn load_full_pointer(&mut self, dst: Register, src_addr: Register, offset_imm: i32) {
        let src_op = MemOperand::new(src_addr, offset_imm);
        self.load_word(dst, src_op);
    }

    pub fn store_tagged_pointer(
        &mut self,
        dst_addr: Register,
        offset_reg: Register,
        offset_imm: i32,
        src: Register,
        pinned: LiftoffRegList,
        protected_store_pc: &mut u32,
        skip_write_barrier: SkipWriteBarrier,
    ) {
        assert_eq!(k_tagged_size, k_int32_size);
        let mut temps = UseScratchRegisterScope::new(self);
        let mut actual_offset_reg = offset_reg;
        if offset_reg != Register::NoReg && offset_imm != 0 {
            if self.cache_state().is_used(LiftoffRegister::new(offset_reg)) {
                actual_offset_reg = temps.acquire();
            }
            self.add32(actual_offset_reg, offset_reg, offset_imm as i64);
        }
        let mut dst_op = MemOperand::new(Register::ScratchReg, 0);
        if actual_offset_reg == Register::NoReg {
            dst_op = MemOperand::new(dst_addr, offset_imm);
        } else {
            self.add_word(Register::ScratchReg, dst_addr, actual_offset_reg);
            dst_op = MemOperand::new(Register::ScratchReg, 0);
        }

        let trapper = |offset: i32| {
            if protected_store_pc != &mut 0 {
                *protected_store_pc = offset as u32;
            }
        };

        self.store_word(src, dst_op, trapper);

        if protected_store_pc != &mut 0 {
            //DCHECK(InstructionAt(*protected_store_pc)->IsStore());
        }

        if skip_write_barrier == SkipWriteBarrier::Enabled || v8_flags.disable_write_barriers {
            return;
        }

        // The write barrier.
        let exit = self.new_label();
        self.check_page_flag(
            dst_addr,
            MemoryChunk::k_pointers_from_here_are_interesting_mask,
            ComparisonResult::Zero,
            &exit,
        );
        self.jump_if_smi(src, &exit);
        self.check_page_flag(
            src,
            MemoryChunk::k_pointers_to_here_are_interesting_mask,
            ComparisonResult::Equal,
            &exit,
        );
        self.call_record_write_stub_save_registers(
            dst_addr,
            if actual_offset_reg == Register::NoReg {
                Operand::Immediate(offset_imm as i64)
            } else {
                Operand::Register(actual_offset_reg)
            },
            SaveFPRegsMode::kSave,
            StubCallMode::kCallWasmRuntimeStub,
        );
        self.bind(&exit);
    }

    pub fn load(
        &mut self,
        dst: LiftoffRegister,
        src_addr: Register,
        offset_reg: Register,
        offset_imm: usize,
        type_: LoadType,
        protected_load_pc: &mut u32,
        _is_load_mem: bool,
        _i64_offset: bool,
        needs_shift: bool,
    ) {
        let shift_amount = if needs_shift { type_.size_log_2() } else { 0 };
        let src_op = liftoff_assembler_riscv32_inl::get_mem_op(
            self, src_addr, offset_reg, offset_imm, shift_amount,
        );
        let trapper = |offset: i32| {
            if protected_load_pc != &mut 0 {
                *protected_load_pc = offset as u32;
            }
        };
        match type_.value() {
            LoadTypeValue::I32Load8U => {
                self.lbu(dst.gp(), src_op, trapper);
            }
            LoadTypeValue::I64Load8U => {
                self.lbu(dst.low_gp(), src_op, trapper);
                self.mv(dst.high_gp(), Register::Zero);
            }
            LoadTypeValue::I32Load8S => {
                self.lb(dst.gp(), src_op, trapper);
            }
            LoadTypeValue::I64Load8S => {
                self.lb(dst.low_gp(), src_op, trapper);
                self.srai(dst.high_gp(), dst.low_gp(), 31);
            }
            LoadTypeValue::I32Load16U => {
                self.lhu(dst.gp(), src_op, trapper);
            }
            LoadTypeValue::I64Load16U => {
                self.lhu(dst.low_gp(), src_op, trapper);
                self.mv(dst.high_gp(), Register::Zero);
            }
            LoadTypeValue::I32Load16S => {
                self.lh(dst.gp(), src_op, trapper);
            }
            LoadTypeValue::I64Load16S => {
                self.lh(dst.low_gp(), src_op, trapper);
                self.srai(dst.high_gp(), dst.low_gp(), 31);
            }
            LoadTypeValue::I64Load32U => {
                self.lw(dst.low_gp(), src_op, trapper);
                self.mv(dst.high_gp(), Register::Zero);
            }
            LoadTypeValue::I64Load32S => {
                self.lw(dst.low_gp(), src_op, trapper);
                self.srai(dst.high_gp(), dst.low_gp(), 31);
            }
            LoadTypeValue::I32Load => {
                self.lw(dst.gp(), src_op, trapper);
            }
            LoadTypeValue::I64Load => {
                self.lw(dst.low_gp(), src_op, trapper);
                let src_op = liftoff_assembler_riscv32_inl::get_mem_op(
                    self,
                    src_addr,
                    offset_reg,
                    offset_imm + k_system_pointer_size as usize,
                    0,
                );
                self.lw(dst.high_gp(), src_op);
            }
            LoadTypeValue::F32Load => {
                self.load_float(dst.fp(), src_op, trapper);
            }
            LoadTypeValue::F64Load => {
                self.load_double(dst.fp(), src_op, trapper);
            }
            LoadTypeValue::S128Load => {
               // self.vu.set(Register::ScratchReg, E8, m1);
                let src_reg = if src_op.offset() == 0 {
                    src_op.rm()
                } else {
                    Register::ScratchReg
                };
                if src_op.offset() != 0 {
                    self.add_word(Register::ScratchReg, src_op.rm(), src_op.offset());
                }
                trapper(self.pc_offset());
                self.vl(dst.fp().to_v(), src_reg, 0, VSew::E8);
            }
            LoadTypeValue::F32LoadF16 => {
                unimplemented!();
            }
            _ => unreachable!(),
        }
        if protected_load_pc != &mut 0 {
            //DCHECK(InstructionAt(*protected_load_pc)->IsLoad());
        }

        // #[cfg(target_endian = "big")]
        // if is_load_mem {
        //     pinned.set(src_op.rm());
        //     liftoff::change_endianness_load(self, dst, type_, pinned);
        // }
    }

    pub fn store(
        &mut self,
        dst_addr: Register,
        offset_reg: Register,
        offset_imm: usize,
        src: LiftoffRegister,
        type_: StoreType,
        pinned: LiftoffRegList,
        protected_store_pc: &mut u32,
        is_store_mem: bool,
        _i64_offset: bool,
    ) {
        let dst_op = liftoff_assembler_riscv32_inl::get_mem_op(self, dst_addr, offset_reg, offset_imm, 0);

        // #[cfg(target_endian = "big")]
        // if is_store_mem {
        //     pinned.set(dst_op.rm());
        //     let tmp = self.get_unused_register(src.reg_class(), pinned);
        //     // Save original value.
        //     self.move_(tmp, src, type_.value_type());

        //     let src = tmp;
        //     pinned.set(tmp);
        //     liftoff::change_endianness_store(self, src, type_, pinned);
        // }

        let trapper = |offset: i32| {
            if protected_store_pc != &mut 0 {
                *protected_store_pc = offset as u32;
            }
        };

        match type_.value() {
            StoreTypeValue::I32Store8 => {
                self.sb(src.gp(), dst_op, trapper);
            }
            StoreTypeValue::I64Store8 => {
                self.sb(src.low_gp(), dst_op, trapper);
            }
            StoreTypeValue::I32Store16 => {
                self.sh(src.gp(), dst_op, trapper);
            }
            StoreTypeValue::I64Store16 => {
                self.sh(src.low_gp(), dst_op, trapper);
            }
            StoreTypeValue::I32Store => {
                self.sw(src.gp(), dst_op, trapper);
            }
            StoreTypeValue::I64Store32 => {
                self.sw(src.low_gp(), dst_op, trapper);
            }
            StoreTypeValue::I64Store => {
                self.sw(src.low_gp(), dst_op, trapper);
                let dst_op = liftoff_assembler_riscv32_inl::get_mem_op(
                    self,
                    dst_addr,
                    offset_reg,
                    offset_imm + k_system_pointer_size as usize,
                    0,
                );
                self.sw(src.high_gp(), dst_op, trapper);
            }
            StoreTypeValue::F32Store => {
                self.store_float(src.fp(), dst_op, trapper);
            }
            StoreTypeValue::F64Store => {
                self.store_double(src.fp(), dst_op, trapper);
            }
            StoreTypeValue::S128Store => {
               // self.vu.set(Register::ScratchReg, E8, m1);
                let dst_reg = if dst_op.offset() == 0 {
                    dst_op.rm()
                } else {
                    Register::ScratchReg
                };
                if dst_op.offset() != 0 {
                    self.add_word(Register::ScratchReg, dst_op.rm(), dst_op.offset());
                }
                trapper(self.pc_offset());
                self.vs(src.fp().to_v(), dst_reg, 0, VSew::E8);
            }
            _ => unreachable!(),
        }
        if protected_store_pc != &mut 0 {
            //DCHECK(InstructionAt(*protected_store_pc)->IsStore());
        }
    }

    pub fn atomic_load(
        &mut self,
        dst: LiftoffRegister,
        src_addr: Register,
        offset_reg: Register,
        offset_imm: usize,
        type_: LoadType,
        _pinned: LiftoffRegList,
        _i64_offset: bool,
    ) {
        let mut temps = UseScratchRegisterScope::new(self);
        let src_reg = liftoff_assembler_riscv32_inl::calculate_actual_address(
            self,
            &mut temps,
            src_addr,
            offset_reg,
            offset_imm,
            Register::NoReg,
        );
        let mut dst_reg = Register::NoReg;
        match type_.value() {
            LoadTypeValue::I32Load8U | LoadTypeValue::I32Load16U | LoadTypeValue::I32Load => {
                dst_reg = dst.gp();
            }
            LoadTypeValue::I64Load8U | LoadTypeValue::I64Load16U | LoadTypeValue::I64Load32U => {
                dst_reg = dst.low_gp();
                self.load_constant(dst.high(), WasmValue::from_i32(0));
            }
            _ => {}
        }
        match type_.value() {
            LoadTypeValue::I32Load8U | LoadTypeValue::I64Load8U => {
                self.fence(FenceFlag::PSR | FenceFlag::PSW, FenceFlag::PSR | FenceFlag::PSW);
                self.lbu(dst_reg, MemOperand::new(src_reg, 0), |_| {});
                self.fence(FenceFlag::PSR, FenceFlag::PSR | FenceFlag::PSW);
                return;
            }
            LoadTypeValue::I32Load16U | LoadTypeValue::I64Load16U => {
                self.fence(FenceFlag::PSR | FenceFlag::PSW, FenceFlag::PSR | FenceFlag::PSW);
                self.lhu(dst_reg, MemOperand::new(src_reg, 0), |_| {});
                self.fence(FenceFlag::PSR, FenceFlag::PSR | FenceFlag::PSW);
                return;
            }
            LoadTypeValue::I32Load | LoadTypeValue::I64Load32U => {
                self.fence(FenceFlag::PSR | FenceFlag::PSW, FenceFlag::PSR | FenceFlag::PSW);
                self.lw(dst_reg, MemOperand::new(src_reg, 0), |_| {});
                self.fence(FenceFlag::PSR, FenceFlag::PSR | FenceFlag::PSW);
                return;
            }
            LoadTypeValue::I64Load => {
                self.fence(FenceFlag::PSR | FenceFlag::PSW, FenceFlag::PSR | FenceFlag::PSW);
                self.lw(
                    dst.low_gp(),
                    MemOperand::new(src_reg, liftoff_assembler_riscv32_inl::K_LOW_WORD_OFFSET),
                    |_| {},
                );
                self.lw(
                    dst.high_gp(),
                    MemOperand::new(src_reg, liftoff_assembler_riscv32_inl::K_HIGH_WORD_OFFSET),
                    |_| {},
                );
                self.fence(FenceFlag::PSR, FenceFlag::PSR | FenceFlag::PSW);
                return;
            }
            _ => unreachable!(),
        }
    }

    pub fn atomic_store(
        &mut self,
        dst_addr: Register,
        offset_reg: Register,
        offset_imm: usize,
        src: LiftoffRegister,
        type_: StoreType,
        _pinned: LiftoffRegList,
        _i64_offset: bool,
    ) {
        let mut temps = UseScratchRegisterScope::new(self);
        let dst_reg = liftoff_assembler_riscv32_inl::calculate_actual_address(
            self,
            &mut temps,
            dst_addr,
            offset_reg,
            offset_imm,
            Register::NoReg,
        );
        let mut src_reg = Register::NoReg;
        match type_.value() {
            StoreTypeValue::I32Store8 | StoreTypeValue::I32Store16 | StoreTypeValue::I32Store => {
                src_reg = src.gp();
            }
            StoreTypeValue::I64Store8 | StoreTypeValue::I64Store16 | StoreTypeValue::I64Store32 => {
                src_reg = src.low_gp();
            }
            _ => {}
        }
        match type_.value() {
            StoreTypeValue::I64Store8 | StoreTypeValue::I32Store8 => {
                self.fence(FenceFlag::PSR | FenceFlag::PSW, FenceFlag::PSW);
                self.sb(src_reg, MemOperand::new(dst_reg, 0), |_| {});
                return;
            }
            StoreTypeValue::I64Store16 | StoreTypeValue::I32Store16 => {
                self.fence(FenceFlag::PSR | FenceFlag::PSW, FenceFlag::PSW);
                self.sh(src_reg, MemOperand::new(dst_reg, 0), |_| {});
                return;
            }
            StoreTypeValue::I64Store32 | StoreTypeValue::I32Store => {
                self.fence(FenceFlag::PSR | FenceFlag::PSW, FenceFlag::PSW);
                self.sw(src_reg, MemOperand::new(dst_reg, 0), |_| {});
                return;
            }
            StoreTypeValue::I64Store => {
                self.fence(FenceFlag::PSR | FenceFlag::PSW, FenceFlag::PSW);
                self.sw(
                    src.low_gp(),
                    MemOperand::new(dst_reg, liftoff_assembler_riscv32_inl::K_LOW_WORD_OFFSET),
                    |_| {},
                );
                self.sw(
                    src.high_gp(),
                    MemOperand::new(dst_reg, liftoff_assembler_riscv32_inl::K_HIGH_WORD_OFFSET),
                    |_| {},
                );
                return;
            }
            _ => unreachable!(),
        }
    }

    pub fn atomic_add(
        &mut self,
        dst_addr: Register,
        offset_reg: Register,
        offset_imm: u32,
        value: LiftoffRegister,
        result: LiftoffRegister,
        type_: StoreType,
        _i64_offset: bool,
    ) {
        if type_.value() == StoreTypeValue::I64Store {
            liftoff_assembler_riscv32_inl::atomic_binop64(
                self,
                dst_addr,
                offset_reg,
                offset_imm as usize,
                value,
                result,
                type_,
                Binop::Add,
            );
            return;
        }
        if type_.value() == StoreTypeValue::I32Store || type_.value() == StoreTypeValue::I64Store32
        {
            let mut temps = UseScratchRegisterScope::new(self);
            let actual_addr = liftoff_assembler_riscv32_inl::calculate_actual_address(
                self,
                &mut temps,
                dst_addr,
                offset_reg,
                offset_imm as usize,
                Register::NoReg,
            );
            if type_.value() == StoreTypeValue::I64Store32 {
                self.mv(result.high_gp(), Register::Zero); // High word of result is always 0.
                let result = result.low();
                let value = value.low();
            }
            self.amoadd_w(true, true, result.gp(), MemOperand::new(actual_addr, 0), value.gp());
            return;
        }

        liftoff_assembler_riscv32_inl::atomic_binop(
            self,
            dst_addr,
            offset_reg,
            offset_imm as usize,
            value,
            result,
            type_,
            Binop::Add,
        );
    }

    pub fn atomic_sub(
        &mut self,
        dst_addr: Register,
        offset_reg: Register,
        offset_imm: u32,
        value: LiftoffRegister,
        result: LiftoffRegister,
        type_: StoreType,
        _i64_offset: bool,
    ) {
        if type_.value() == StoreTypeValue::I64Store {
            liftoff_assembler_riscv32_inl::atomic_binop64(
                self,
                dst_addr,
                offset_reg,
                offset_imm as usize,
                value,
                result,
                type_,
                Binop::Sub,
            );
            return;
        }
        if type_.value() == StoreTypeValue::I32Store || type_.value() == StoreTypeValue::I64Store32
        {
            let mut temps = UseScratchRegisterScope::new(self);
            let actual_addr = liftoff_assembler_riscv32_inl::calculate_actual_address(
                self,
                &mut temps,
                dst_addr,
                offset_reg,
                offset_imm as usize,
                Register::NoReg,
            );
            if type_.value() == Store