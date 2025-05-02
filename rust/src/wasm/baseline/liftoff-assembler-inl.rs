// src/wasm/baseline/liftoff_assembler_inl.rs

// Placeholder for architecture specific includes.  Since these are inlines,
// they'll need to be feature gated correctly and live elsewhere.
// For now, implement a default version that compiles.  The intention
// is to make the file compile for all platforms, even if some functionality
// is missing.

use crate::wasm::baseline::liftoff_assembler::*;
use crate::wasm::wasm_opcodes::ValueKind;
// use std::mem::size_of;

// Helper function to determine slot size for a given type
pub fn slot_size_for_type(kind: ValueKind) -> i32 {
    match kind {
        ValueKind::I32 | ValueKind::F32 => 4,
        ValueKind::I64 | ValueKind::F64 => 8,
        ValueKind::Ref => 8, // Assuming refs are pointers
    }
}

pub fn needs_alignment(kind: ValueKind) -> bool {
    match kind {
        ValueKind::I64 | ValueKind::F64 | ValueKind::Ref => true,
        _ => false,
    }
}

pub fn round_up(value: i32, alignment: i32) -> i32 {
    ((value + alignment - 1) / alignment) * alignment
}

impl LiftoffAssembler {
    pub fn next_spill_offset_with_offset(kind: ValueKind, top_spill_offset: i32) -> i32 {
        let mut offset = top_spill_offset + slot_size_for_type(kind);
        if needs_alignment(kind) {
            offset = round_up(offset, slot_size_for_type(kind));
        }
        offset
    }

    pub fn next_spill_offset(&self, kind: ValueKind) -> i32 {
        Self::next_spill_offset_with_offset(kind, self.top_spill_offset())
    }

    pub fn top_spill_offset(&self) -> i32 {
        if self.cache_state_.stack_state.is_empty() {
            self.static_stack_frame_size()
        } else {
            self.cache_state_.stack_state.last().map(|x| x.offset).unwrap_or(0)
        }
    }

    pub fn push_register(&mut self, kind: ValueKind, reg: LiftoffRegister) {
        assert_eq!(reg_class_for(kind), reg.reg_class());
        self.cache_state_.inc_used(reg);
        self.cache_state_.stack_state.push(VarState::reg(kind, reg, self.next_spill_offset(kind)));
    }

    // Assumes that the exception is in {kReturnRegister0}. This is where the
    // exception is stored by the unwinder after a throwing call.
    pub fn push_exception(&mut self) {
        let reg = LiftoffRegister { code: kReturnRegister0 };
        // This is used after a call, so {kReturnRegister0} is not used yet.
        assert!(self.cache_state_.is_free(reg));
        self.cache_state_.inc_used(reg);
        self.cache_state_.stack_state.push(VarState::reg(ValueKind::Ref, reg, self.next_spill_offset(ValueKind::Ref)));
    }

    pub fn push_constant(&mut self, kind: ValueKind, i32_const: i32) {
        assert!(kind == ValueKind::I32 || kind == ValueKind::I64);
        self.cache_state_.stack_state.push(VarState::constant(kind, i32_const, self.next_spill_offset(kind)));
    }

    pub fn push_stack(&mut self, kind: ValueKind) {
        self.cache_state_.stack_state.push(VarState::stack(kind, self.next_spill_offset(kind)));
    }

    pub fn load_to_fixed_register(&mut self, slot: VarState, reg: LiftoffRegister) {
        assert!(slot.is_const() || slot.is_stack());
        if slot.is_const() {
            self.load_constant(reg, slot.constant());
        } else {
            self.fill(reg, slot.offset(), slot.kind());
        }
    }

    pub fn pop_to_fixed_register(&mut self, reg: LiftoffRegister) {
        assert!(!self.cache_state_.stack_state.is_empty());
        let slot = self.cache_state_.stack_state.pop().unwrap();
        if slot.is_reg() {
            let slot_reg = slot.reg();
            self.cache_state_.dec_used(slot_reg);
            if slot_reg == reg {
                return;
            }
            if self.cache_state_.is_used(reg) {
                self.spill_register(reg);
            }
            self.move_(reg, slot_reg, slot.kind());
            return;
        }
        if self.cache_state_.is_used(reg) {
            self.spill_register(reg);
        }
        self.load_to_fixed_register(slot, reg);
    }
    // The definition of FixedArray and its length_ member are not available.
    // Therefore, the offset cannot be determined, and this method remains unimplemented.
    pub fn load_fixed_array_length_as_int32(
        &mut self,
        _dst: LiftoffRegister,
        _array: Register,
        _pinned: LiftoffRegList,
    ) {
        todo!()
        //let offset = offsetof(FixedArray, length_) - kHeapObjectTag;
        //self.LoadSmiAsInt32(dst, array, offset);
    }

    // Similarly, the implementation depends on the architecture-specific details
    // of Smi representation, so it cannot be directly translated.
    pub fn load_smi_as_int32(&mut self, _dst: LiftoffRegister, _src_addr: Register, _offset: i32) {
        todo!()
        //if constexpr (SmiValuesAre32Bits()) {
        //#if V8_TARGET_LITTLE_ENDIAN
        //  DCHECK_EQ(kSmiShiftSize + kSmiTagSize, 4 * kBitsPerByte);
        //  offset += 4;
        //#endif
        //  Load(dst, src_addr, no_reg, offset, LoadType::kI32Load);
        //} else {
        //  DCHECK(SmiValuesAre31Bits());
        //  Load(dst, src_addr, no_reg, offset, LoadType::kI32Load);
        //  emit_i32_sari(dst.gp(), dst.gp(), kSmiTagSize);
        //}
    }

    pub fn load_code_pointer(&mut self, dst: Register, src_addr: Register, offset_imm: i32) {
        self.load(
            LiftoffRegister { code: dst as u8 },
            src_addr,
            NoReg,
            offset_imm,
            LoadType::kI32Load,
        );
    }

    pub fn emit_ptrsize_add(&mut self, dst: Register, lhs: Register, rhs: Register) {
        if kSystemPointerSize == 8 {
            self.emit_i64_add(
                LiftoffRegister { code: dst as u8 },
                LiftoffRegister { code: lhs as u8 },
                LiftoffRegister { code: rhs as u8 },
            );
        } else {
            self.emit_i32_add(dst, lhs, rhs);
        }
    }

    pub fn emit_ptrsize_sub(&mut self, dst: Register, lhs: Register, rhs: Register) {
        if kSystemPointerSize == 8 {
            self.emit_i64_sub(
                LiftoffRegister { code: dst as u8 },
                LiftoffRegister { code: lhs as u8 },
                LiftoffRegister { code: rhs as u8 },
            );
        } else {
            self.emit_i32_sub(dst, lhs, rhs);
        }
    }

    pub fn emit_ptrsize_and(&mut self, dst: Register, lhs: Register, rhs: Register) {
        if kSystemPointerSize == 8 {
            self.emit_i64_and(
                LiftoffRegister { code: dst as u8 },
                LiftoffRegister { code: lhs as u8 },
                LiftoffRegister { code: rhs as u8 },
            );
        } else {
            self.emit_i32_and(dst, lhs, rhs);
        }
    }

    pub fn emit_ptrsize_shri(&mut self, dst: Register, src: Register, amount: i32) {
        if kSystemPointerSize == 8 {
            self.emit_i64_shri(
                LiftoffRegister { code: dst as u8 },
                LiftoffRegister { code: src as u8 },
                amount,
            );
        } else {
            self.emit_i32_shri(dst, src, amount);
        }
    }

    pub fn emit_ptrsize_addi(&mut self, dst: Register, lhs: Register, imm: i64) {
        if kSystemPointerSize == 8 {
            self.emit_i64_addi(
                LiftoffRegister { code: dst as u8 },
                LiftoffRegister { code: lhs as u8 },
                imm,
            );
        } else {
            self.emit_i32_addi(dst, lhs, imm as i32);
        }
    }

    pub fn emit_ptrsize_muli(&mut self, dst: Register, lhs: Register, imm: i32) {
        if kSystemPointerSize == 8 {
            self.emit_i64_muli(
                LiftoffRegister { code: dst as u8 },
                LiftoffRegister { code: lhs as u8 },
                imm,
            );
        } else {
            self.emit_i32_muli(dst, lhs, imm);
        }
    }

    pub fn emit_ptrsize_set_cond(
        &mut self,
        condition: Condition,
        dst: Register,
        lhs: LiftoffRegister,
        rhs: LiftoffRegister,
    ) {
        if kSystemPointerSize == 8 {
            self.emit_i64_set_cond(condition, dst, lhs, rhs);
        } else {
            self.emit_i32_set_cond(condition, dst, lhs.gp(), rhs.gp());
        }
    }

    pub fn bailout(&mut self, reason: LiftoffBailoutReason, detail: &'static str) {
        assert_ne!(LiftoffBailoutReason::kSuccess, reason);
        if self.bailout_reason_ != LiftoffBailoutReason::kSuccess {
            return;
        }
        self.abort_compilation();
        self.bailout_reason_ = reason;
        self.bailout_detail_ = detail;
    }
    // #ifdef V8_TARGET_ARCH_32_BIT

    pub fn emit_ptrsize_cond_jumpi(
        &mut self,
        _cond: Condition,
        _label: &mut Label,
        _lhs: Register,
        _imm: i32,
        _frozen: &FreezeCacheState,
    ) {
        todo!()
        //emit_i32_cond_jumpi(cond, label, lhs, imm, frozen);
    }
    //End of the partially platform-independent implementations of the
    //platform-dependent part.
    // =======================================================================
}

//const kSystemPointerSize: i32 = 8; // Assuming 64-bit architecture

pub mod liftoff {
    use super::*;

    // This function is heavily dependent on LiftoffAssembler internals,
    // so it's not directly translatable without a deep understanding of the
    // register allocation and instruction emission logic.
    pub fn emit_i64_independent_half_operation<
        F: Fn(&mut LiftoffAssembler, Register, Register, Register),
    >(
        assm: &mut LiftoffAssembler,
        dst: LiftoffRegister,
        lhs: LiftoffRegister,
        rhs: LiftoffRegister,
        op: F,
    ) {
        // If {dst.low_gp()} does not overlap with {lhs.high_gp()} or {rhs.high_gp()},
        // just first compute the lower half, then the upper half.
        if dst.low() != lhs.high() && dst.low() != rhs.high() {
            op(assm, dst.low_gp(), lhs.low_gp(), rhs.low_gp());
            op(assm, dst.high_gp(), lhs.high_gp(), rhs.high_gp());
            return;
        }
        // If {dst.high_gp()} does not overlap with {lhs.low_gp()} or {rhs.low_gp()},
        // we can compute this the other way around.
        if dst.high() != lhs.low() && dst.high() != rhs.low() {
            op(assm, dst.high_gp(), lhs.high_gp(), rhs.high_gp());
            op(assm, dst.low_gp(), lhs.low_gp(), rhs.low_gp());
            return;
        }
        // Otherwise, we need a temporary register.
        let tmp = assm
            .get_unused_register(kGpReg, LiftoffRegList::new(&[lhs, rhs]))
            .gp();
        op(assm, tmp, lhs.low_gp(), rhs.low_gp());
        op(assm, dst.high_gp(), lhs.high_gp(), rhs.high_gp());
        assm.move_(dst.low_gp(), tmp, ValueKind::I32);
    }

    pub fn emit_i64_independent_half_operation_imm<
        F: Fn(&mut LiftoffAssembler, Register, Register, i32),
    >(
        assm: &mut LiftoffAssembler,
        dst: LiftoffRegister,
        lhs: LiftoffRegister,
        imm: i64,
        op: F,
    ) {
        let low_word = imm as i32;
        let high_word = (imm >> 32) as i32;
        // If {dst.low_gp()} does not overlap with {lhs.high_gp()},
        // just first compute the lower half, then the upper half.
        if dst.low() != lhs.high() {
            op(assm, dst.low_gp(), lhs.low_gp(), low_word);
            op(assm, dst.high_gp(), lhs.high_gp(), high_word);
            return;
        }
        // If {dst.high_gp()} does not overlap with {lhs.low_gp()},
        // we can compute this the other way around.
        if dst.high() != lhs.low() {
            op(assm, dst.high_gp(), lhs.high_gp(), high_word);
            op(assm, dst.low_gp(), lhs.low_gp(), low_word);
            return;
        }
        // Otherwise, we need a temporary register.
        let tmp = assm.get_unused_register(kGpReg, LiftoffRegList::new(&[lhs])).gp();
        op(assm, tmp, lhs.low_gp(), low_word);
        op(assm, dst.high_gp(), lhs.high_gp(), high_word);
        assm.move_(dst.low_gp(), tmp, ValueKind::I32);
    }
}

impl LiftoffAssembler {
    pub fn emit_i64_and(&mut self, dst: LiftoffRegister, lhs: LiftoffRegister, rhs: LiftoffRegister) {
        liftoff::emit_i64_independent_half_operation(self, dst, lhs, rhs, |assm, dst, lhs, rhs| {
            assm.emit_i32_and(dst, lhs, rhs);
        });
    }

    pub fn emit_i64_andi(&mut self, dst: LiftoffRegister, lhs: LiftoffRegister, imm: i32) {
        liftoff::emit_i64_independent_half_operation_imm(self, dst, lhs, imm as i64, |assm, dst, lhs, imm| {
            assm.emit_i32_andi(dst, lhs, imm);
        });
    }

    pub fn emit_i64_or(&mut self, dst: LiftoffRegister, lhs: LiftoffRegister, rhs: LiftoffRegister) {
        liftoff::emit_i64_independent_half_operation(self, dst, lhs, rhs, |assm, dst, lhs, rhs| {
            assm.emit_i32_or(dst, lhs, rhs);
        });
    }

    pub fn emit_i64_ori(&mut self, dst: LiftoffRegister, lhs: LiftoffRegister, imm: i32) {
        liftoff::emit_i64_independent_half_operation_imm(self, dst, lhs, imm as i64, |assm, dst, lhs, imm| {
            assm.emit_i32_ori(dst, lhs, imm);
        });
    }

    pub fn emit_i64_xor(&mut self, dst: LiftoffRegister, lhs: LiftoffRegister, rhs: LiftoffRegister) {
        liftoff::emit_i64_independent_half_operation(self, dst, lhs, rhs, |assm, dst, lhs, rhs| {
            assm.emit_i32_xor(dst, lhs, rhs);
        });
    }

    pub fn emit_i64_xori(&mut self, dst: LiftoffRegister, lhs: LiftoffRegister, imm: i32) {
        liftoff::emit_i64_independent_half_operation_imm(self, dst, lhs, imm as i64, |assm, dst, lhs, imm| {
            assm.emit_i32_xori(dst, lhs, imm);
        });
    }

    pub fn emit_u32_to_uintptr(&mut self, dst: Register, src: Register) {
        if dst != src {
            self.move_(
                LiftoffRegister { code: dst as u8 },
                LiftoffRegister { code: src as u8 },
                ValueKind::I32,
            );
        }
    }

    pub fn clear_i32_upper_half(&mut self, _dst: Register) {
        todo!(); //UNREACHABLE();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wasm::baseline::liftoff_assembler::{CacheState, LiftoffRegister, VarState};
    use crate::wasm::wasm_opcodes::ValueKind;
    const kGpReg: RegClass = RegClass::Gp;
    const kReturnRegister0: u8 = 0;
    const kSystemPointerSize: i32 = 8;

    #[test]
    fn test_next_spill_offset() {
        assert_eq!(LiftoffAssembler::next_spill_offset_with_offset(ValueKind::I32, 0), 4);
        assert_eq!(LiftoffAssembler::next_spill_offset_with_offset(ValueKind::I64, 0), 8);
        assert_eq!(LiftoffAssembler::next_spill_offset_with_offset(ValueKind::F64, 0), 8);
        assert_eq!(LiftoffAssembler::next_spill_offset_with_offset(ValueKind::I32, 4), 8);
        assert_eq!(LiftoffAssembler::next_spill_offset_with_offset(ValueKind::I64, 4), 8);
        assert_eq!(LiftoffAssembler::next_spill_offset_with_offset(ValueKind::F64, 4), 8);
        assert_eq!(LiftoffAssembler::next_spill_offset_with_offset(ValueKind::I32, 8), 12);
        assert_eq!(LiftoffAssembler::next_spill_offset_with_offset(ValueKind::I64, 8), 16);
        assert_eq!(LiftoffAssembler::next_spill_offset_with_offset(ValueKind::F64, 8), 16);
    }

    #[test]
    fn test_needs_alignment() {
        assert_eq!(needs_alignment(ValueKind::I32), false);
        assert_eq!(needs_alignment(ValueKind::I64), true);
        assert_eq!(needs_alignment(ValueKind::F32), false);
        assert_eq!(needs_alignment(ValueKind::F64), true);
    }

    #[test]
    fn test_round_up() {
        assert_eq!(round_up(0, 8), 0);
        assert_eq!(round_up(1, 8), 8);
        assert_eq!(round_up(7, 8), 8);
        assert_eq!(round_up(8, 8), 8);
        assert_eq!(round_up(9, 8), 16);
    }

    #[test]
    fn test_liftoff_assembler_top_spill_offset() {
        let mut assm = LiftoffAssembler {
            cache_state_: CacheState {
                stack_state: vec![],
                used: [0; 2],
            },
            static_stack_frame_size_: 16,
            bailout_reason_: LiftoffBailoutReason::kSuccess,
            bailout_detail_: "",
        };
        assert_eq!(assm.top_spill_offset(), 16);

        assm.cache_state_.stack_state.push(VarState::stack(ValueKind::I32, 20));
        assert_eq!(assm.top_spill_offset(), 20);
    }

    #[test]
    fn test_liftoff_push_register() {
        let mut assm = LiftoffAssembler {
            cache_state_: CacheState {
                stack_state: vec![],
                used: [0; 2],
            },
            static_stack_frame_size_: 16,
            bailout_reason_: LiftoffBailoutReason::kSuccess,
            bailout_detail_: "",
        };

        let reg = LiftoffRegister { code: 1 };
        assm.push_register(ValueKind::I32, reg);
        assert_eq!(assm.cache_state_.used[0], 1);
        assert_eq!(assm.cache_state_.stack_state.len(), 1);
        assert_eq!(assm.cache_state_.stack_state[0].offset, 16);
    }

    #[test]
    fn test_liftoff_push_exception() {
        let mut assm = LiftoffAssembler {
            cache_state_: CacheState {
                stack_state: vec![],
                used: [0; 2],
            },
            static_stack_frame_size_: 16,
            bailout_reason_: LiftoffBailoutReason::kSuccess,
            bailout_detail_: "",
        };

        assm.push_exception();
        assert_eq!(assm.cache_state_.used[0], 1);
        assert_eq!(assm.cache_state_.stack_state.len(), 1);
        assert_eq!(assm.cache_state_.stack_state[0].offset, 16);
    }

    #[test]
    fn test_liftoff_push_constant() {
        let mut assm = LiftoffAssembler {
            cache_state_: CacheState {
                stack_state: vec![],
                used: [0; 2],
            },
            static_stack_frame_size_: 16,
            bailout_reason_: LiftoffBailoutReason::kSuccess,
            bailout_detail_: "",
        };

        assm.push_constant(ValueKind::I32, 42);
        assert_eq!(assm.cache_state_.stack_state.len(), 1);
        assert_eq!(assm.cache_state_.stack_state[0].offset, 16);
        assert_eq!(assm.cache_state_.stack_state[0].constant, 42);
    }

    #[test]
    fn test_liftoff_push_stack() {
        let mut assm = LiftoffAssembler {
            cache_state_: CacheState {
                stack_state: vec![],
                used: [0; 2],
            },
            static_stack_frame_size_: 16,
            bailout_reason_: LiftoffBailoutReason::kSuccess,
            bailout_detail_: "",
        };

        assm.push_stack(ValueKind::I32);
        assert_eq!(assm.cache_state_.stack_state.len(), 1);
        assert_eq!(assm.cache_state_.stack_state[0].offset, 16);
    }
}