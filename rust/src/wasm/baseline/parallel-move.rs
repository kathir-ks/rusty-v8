// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod parallel_move {
    use crate::wasm::baseline::liftoff_assembler::LiftoffAssembler;
    use crate::wasm::baseline::liftoff_register::LiftoffRegister;
    use crate::wasm::wasm_value::ValueKind;
    use bit_set::BitSet;
    use std::mem::MaybeUninit;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RegPairHalf {
        LowWord,
        HighWord,
    }

    struct RegisterMove {
        src: LiftoffRegister,
        kind: ValueKind,
    }

    impl RegisterMove {
        const fn new(src: LiftoffRegister, kind: ValueKind) -> Self {
            RegisterMove { src, kind }
        }
    }

    struct RegisterLoad {
        load_kind: LoadKind,
        kind: ValueKind,
        value: i32,
    }

    impl RegisterLoad {
        fn const_val(kind: ValueKind, constant: i32) -> Self {
            assert!(kind == ValueKind::I32 || kind == ValueKind::I64);
            RegisterLoad {
                load_kind: LoadKind::Constant,
                kind,
                value: constant,
            }
        }

        fn stack(offset: i32, kind: ValueKind) -> Self {
            RegisterLoad {
                load_kind: LoadKind::Stack,
                kind,
                value: offset,
            }
        }

        fn half_stack(offset: i32, half: RegPairHalf) -> Self {
            RegisterLoad {
                load_kind: match half {
                    RegPairHalf::LowWord => LoadKind::LowHalfStack,
                    RegPairHalf::HighWord => LoadKind::HighHalfStack,
                },
                kind: ValueKind::I32,
                value: offset,
            }
        }

        fn nop() -> Self {
            RegisterLoad {
                load_kind: LoadKind::Nop,
                kind: ValueKind::I32, // ValueKind does not matter.
                value: 0,
            }
        }

        fn new(load_kind: LoadKind, kind: ValueKind, value: i32) -> Self {
            RegisterLoad {
                load_kind,
                kind,
                value,
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum LoadKind {
        Nop,
        Constant,
        Stack,
        LowHalfStack,
        HighHalfStack,
    }

    const K_AFTER_MAX_LIFTOFF_REG_CODE: usize = 128; // Placeholder value
                                                      // needs to be replaced with the actual const

    /// `ParallelMove` is a utility class that encodes multiple moves from registers to
    /// registers (`RegisterMove`), constants to registers (`RegisterLoad` with
    /// `LoadKind::kConstant`), or stack slots to registers (other
    /// `RegisterLoad`s).
    /// It can handle cyclic moves, e.g., swaps between registers.
    /// The moves are typically prepared/encoded into an instance via the high-level
    /// entry point `Transfer`, which takes two Wasm value stack configurations
    /// (`VarState`) as input.
    /// Code is actually emitted to the underlying `LiftoffAssembler` only at the
    /// end via `Execute` or implicitly in the destructor.
    pub struct ParallelMove<'a> {
        register_moves: [MaybeUninit<RegisterMove>; K_AFTER_MAX_LIFTOFF_REG_CODE],
        register_loads: [MaybeUninit<RegisterLoad>; K_AFTER_MAX_LIFTOFF_REG_CODE],
        src_reg_use_count: [i32; K_AFTER_MAX_LIFTOFF_REG_CODE],
        move_dst_regs: BitSet,
        load_dst_regs: BitSet,
        asm: &'a mut LiftoffAssembler,
        last_spill_offset: i32,
    }

    impl<'a> ParallelMove<'a> {
        pub fn new(wasm_asm: &'a mut LiftoffAssembler) -> Self {
            ParallelMove {
                register_moves: unsafe { MaybeUninit::uninit().assume_init() },
                register_loads: unsafe { MaybeUninit::uninit().assume_init() },
                src_reg_use_count: [0; K_AFTER_MAX_LIFTOFF_REG_CODE],
                move_dst_regs: BitSet::new(),
                load_dst_regs: BitSet::new(),
                asm: wasm_asm,
                last_spill_offset: 0,
            }
        }

        /// Executes all pending moves and loads.
        pub fn execute(&mut self) {
            // First, execute register moves. Then load constants and stack values into
            // registers.
            if !self.move_dst_regs.is_empty() {
                self.execute_moves();
            }
            assert!(self.move_dst_regs.is_empty());
            if !self.load_dst_regs.is_empty() {
                self.execute_loads();
            }
            assert!(self.load_dst_regs.is_empty());
            // Tell the compiler that the ParallelMove is empty after this, so it
            // can eliminate a second {Execute} in the destructor.
            let all_done = self.move_dst_regs.is_empty() && self.load_dst_regs.is_empty();
            assert!(all_done);
        }

        /// Transfers a value from src to dst.
        pub fn transfer(&mut self, dst: &LiftoffAssembler::VarState, src: &LiftoffAssembler::VarState) {
            assert!(LiftoffAssembler::compatible_stack_slot_types(dst.kind(), src.kind()));
            if dst.is_stack() {
                if src.is_stack() && src.offset() == dst.offset() {
                   //do nothing, source and destination are the same
                } else {
                    self.transfer_to_stack(dst.offset(), src);
                }
            } else if dst.is_reg() {
                self.load_into_register(dst.reg(), src);
            } else {
                assert!(dst.is_const());
                assert_eq!(dst.i32_const(), src.i32_const());
            }
        }

        fn transfer_to_stack(&mut self, dst_offset: i32, src: &LiftoffAssembler::VarState) {
            todo!()
        }

        /// Loads a value from src into register dst.
        pub fn load_into_register(&mut self, dst: LiftoffRegister, src: &LiftoffAssembler::VarState) {
            if src.is_reg() {
                assert_eq!(dst.reg_class(), src.reg_class());
                if dst != src.reg() {
                    self.move_register(dst, src.reg(), src.kind());
                }
            } else if src.is_stack() {
                self.load_stack_slot(dst, src.offset(), src.kind());
            } else {
                assert!(src.is_const());
                self.load_constant(dst, src.kind(), src.i32_const());
            }
        }

        fn load_i64_half_into_register(
            &mut self,
            dst: LiftoffRegister,
            src: &LiftoffAssembler::VarState,
            half: RegPairHalf,
        ) {
            // Use CHECK such that the remaining code is statically dead if
            // {kNeedI64RegPair} is false.
            // CHECK(kNeedI64RegPair);
            const K_NEED_I64_REG_PAIR: bool = true; // Placeholder value, replace with actual const if applicable.
            assert!(K_NEED_I64_REG_PAIR);
            assert_eq!(ValueKind::I64, src.kind());

            match src.loc() {
                LiftoffAssembler::VarStateLoc::Stack => {
                    self.load_i64_half_stack_slot(dst, src.offset(), half);
                }
                LiftoffAssembler::VarStateLoc::Register => {
                    let src_half = match half {
                        RegPairHalf::LowWord => src.reg().low(),
                        RegPairHalf::HighWord => src.reg().high(),
                    };
                    if dst != src_half {
                        self.move_register(dst, src_half, ValueKind::I32);
                    }
                }
                LiftoffAssembler::VarStateLoc::IntConst => {
                    let mut value = src.i32_const();
                    // The high word is the sign extension of the low word.
                    if half == RegPairHalf::HighWord {
                        value = value >> 31;
                    }
                    self.load_constant(dst, ValueKind::I32, value);
                }
            }
        }

        /// Moves the value from register src to register dst.
        fn move_register(&mut self, dst: LiftoffRegister, src: LiftoffRegister, kind: ValueKind) {
            assert_ne!(dst, src);
            assert_eq!(dst.reg_class(), src.reg_class());
            assert_eq!(
                LiftoffAssembler::reg_class_for(kind),
                src.reg_class()
            );

            if src.is_gp_pair() {
                assert_eq!(ValueKind::I64, kind);
                if dst.low() != src.low() {
                    self.move_register(dst.low(), src.low(), ValueKind::I32);
                }
                if dst.high() != src.high() {
                    self.move_register(dst.high(), src.high(), ValueKind::I32);
                }
                return;
            }

            if src.is_fp_pair() {
                assert_eq!(ValueKind::S128, kind);
                if dst.low() != src.low() {
                    self.move_register(dst.low(), src.low(), ValueKind::F64);
                    self.move_register(dst.high(), src.high(), ValueKind::F64);
                }
                return;
            }

            if self.move_dst_regs.contains(dst.liftoff_code() as usize) {
                assert_eq!(self.register_move(dst).src, src);
                // Check for compatible value kinds.
                // - references can occur with mixed kRef / kRefNull kinds.
                // - FP registers can only occur with f32 / f64 / s128 kinds (mixed kinds
                //   only if they hold the initial zero value).
                // - others must match exactly.
                assert_eq!(
                    LiftoffAssembler::is_object_reference(self.register_move(dst).kind),
                    LiftoffAssembler::is_object_reference(kind)
                );
                assert_eq!(
                    dst.is_fp(),
                    self.register_move(dst).kind == ValueKind::F32
                        || self.register_move(dst).kind == ValueKind::F64
                        || self.register_move(dst).kind == ValueKind::S128
                );
                if !LiftoffAssembler::is_object_reference(kind) && !dst.is_fp() {
                    assert_eq!(self.register_move(dst).kind, kind);
                }
                // Potentially upgrade an existing `kF32` move to a `kF64` move.
                if kind == ValueKind::F64 {
                    self.register_move_mut(dst).kind = ValueKind::F64;
                }
                return;
            }

            self.move_dst_regs.insert(dst.liftoff_code() as usize);
            *self.src_reg_use_count_mut(src) += 1;
            *self.register_move_mut(dst) = RegisterMove::new(src, kind);
        }

        /// Loads a constant value into register dst.
        fn load_constant(&mut self, dst: LiftoffRegister, kind: ValueKind, constant: i32) {
            assert!(!self.load_dst_regs.contains(dst.liftoff_code() as usize));
            self.load_dst_regs.insert(dst.liftoff_code() as usize);

            if dst.is_gp_pair() {
                assert_eq!(ValueKind::I64, kind);
                *self.register_load_mut(dst.low()) = RegisterLoad::const_val(ValueKind::I32, constant);
                // The high word is either 0 or 0xffffffff.
                *self.register_load_mut(dst.high()) = RegisterLoad::const_val(ValueKind::I32, constant >> 31);
            } else {
                *self.register_load_mut(dst) = RegisterLoad::const_val(kind, constant);
            }
        }

        /// Loads a value from a stack slot into register dst.
        fn load_stack_slot(&mut self, dst: LiftoffRegister, stack_offset: i32, kind: ValueKind) {
            assert!(stack_offset > 0);
            if self.load_dst_regs.contains(dst.liftoff_code() as usize) {
                // It can happen that we spilled the same register to different stack
                // slots, and then we reload them later into the same dst register.
                // In that case, it is enough to load one of the stack slots.
                return;
            }
            self.load_dst_regs.insert(dst.liftoff_code() as usize);
            // Make sure that we only spill to positions after this stack offset to
            // avoid overwriting the content.
            if stack_offset > self.last_spill_offset {
                self.last_spill_offset = stack_offset;
            }
            if dst.is_gp_pair() {
                assert_eq!(ValueKind::I64, kind);
                *self.register_load_mut(dst.low()) =
                    RegisterLoad::half_stack(stack_offset, RegPairHalf::LowWord);
                *self.register_load_mut(dst.high()) =
                    RegisterLoad::half_stack(stack_offset, RegPairHalf::HighWord);
            } else if dst.is_fp_pair() {
                assert_eq!(ValueKind::S128, kind);
                // Only need register_load for low_gp since we load 128 bits at one go.
                // Both low and high need to be set in load_dst_regs_ but when iterating
                // over it, both low and high will be cleared, so we won't load twice.
                *self.register_load_mut(dst.low()) = RegisterLoad::stack(stack_offset, kind);
                *self.register_load_mut(dst.high()) = RegisterLoad::nop();
            } else {
                *self.register_load_mut(dst) = RegisterLoad::stack(stack_offset, kind);
            }
        }

        fn load_i64_half_stack_slot(&mut self, dst: LiftoffRegister, offset: i32, half: RegPairHalf) {
            if self.load_dst_regs.contains(dst.liftoff_code() as usize) {
                // It can happen that we spilled the same register to different stack
                // slots, and then we reload them later into the same dst register.
                // In that case, it is enough to load one of the stack slots.
                return;
            }
            self.load_dst_regs.insert(dst.liftoff_code() as usize);
            *self.register_load_mut(dst) = RegisterLoad::half_stack(offset, half);
        }

        fn register_move(&self, reg: LiftoffRegister) -> &RegisterMove {
            unsafe {
                self.register_moves[reg.liftoff_code() as usize]
                    .assume_init_ref()
            }
        }

        fn register_move_mut(&mut self, reg: LiftoffRegister) -> &mut RegisterMove {
            unsafe {
                self.register_moves[reg.liftoff_code() as usize]
                    .assume_init_mut()
            }
        }

        fn register_load(&self, reg: LiftoffRegister) -> &RegisterLoad {
            unsafe {
                self.register_loads[reg.liftoff_code() as usize]
                    .assume_init_ref()
            }
        }

        fn register_load_mut(&mut self, reg: LiftoffRegister) -> &mut RegisterLoad {
            unsafe {
                self.register_loads[reg.liftoff_code() as usize]
                    .assume_init_mut()
            }
        }

        fn src_reg_use_count(&self, reg: LiftoffRegister) -> &i32 {
            &self.src_reg_use_count[reg.liftoff_code() as usize]
        }

        fn src_reg_use_count_mut(&mut self, reg: LiftoffRegister) -> &mut i32 {
            &mut self.src_reg_use_count[reg.liftoff_code() as usize]
        }

        fn execute_move(&mut self, dst: LiftoffRegister) {
            let move_data = self.register_move(dst);
            assert_eq!(0, *self.src_reg_use_count(dst));
            self.asm.move_val(dst, move_data.src, move_data.kind);
            self.clear_executed_move(dst);
        }

        fn clear_executed_move(&mut self, dst: LiftoffRegister) {
            assert!(self.move_dst_regs.contains(dst.liftoff_code() as usize));
            self.move_dst_regs.remove(dst.liftoff_code() as usize);
            let move_data = self.register_move(dst);
            assert!(0 < *self.src_reg_use_count(move_data.src));
            *self.src_reg_use_count_mut(move_data.src) -= 1;
            if *self.src_reg_use_count(move_data.src) > 0 {
                return;
            }

            // src count dropped to zero. If this is a destination register, execute
            // that move now.
            if !self.move_dst_regs.contains(move_data.src.liftoff_code() as usize) {
                return;
            }
            self.execute_move(move_data.src);
        }

        #[inline(never)]
        fn execute_moves(&mut self) {
            // This is a placeholder implementation.  The original C++ code has
            // V8_NOINLINE and V8_PRESERVE_MOST attributes which are hints to the
            // compiler about inlining and code preservation. The Rust equivalent
            // may require specific attributes depending on the target compiler and optimization
            // goals.
            let mut dst_regs: Vec<LiftoffRegister> = self.move_dst_regs.iter().map(|i| LiftoffRegister::from_code(i as i32)).collect();
            for &dst in &dst_regs {
                if self.move_dst_regs.contains(dst.liftoff_code() as usize) {
                    self.execute_move(dst);
                }
            }
        }

        #[inline(never)]
        fn execute_loads(&mut self) {
           // This is a placeholder implementation.  The original C++ code has
            // V8_NOINLINE and V8_PRESERVE_MOST attributes which are hints to the
            // compiler about inlining and code preservation. The Rust equivalent
            // may require specific attributes depending on the target compiler and optimization
            // goals.
            let mut dst_regs: Vec<LiftoffRegister> = self.load_dst_regs.iter().map(|i| LiftoffRegister::from_code(i as i32)).collect();
            for &dst in &dst_regs {
                if self.load_dst_regs.contains(dst.liftoff_code() as usize) {
                    let load = self.register_load(dst);
                    match load.load_kind {
                        LoadKind::Constant => {
                            self.asm.load_constant(dst, load.kind, load.value);
                        }
                        LoadKind::Stack => {
                            self.asm.load_stack_slot(dst, load.value, load.kind);
                        }
                        LoadKind::LowHalfStack => {
                            self.asm.load_i64_half_stack_slot(dst, load.value, RegPairHalf::LowWord);
                        }
                        LoadKind::HighHalfStack => {
                            self.asm.load_i64_half_stack_slot(dst, load.value, RegPairHalf::HighWord);
                        }
                        LoadKind::Nop => {}
                    }
                    self.load_dst_regs.remove(dst.liftoff_code() as usize);
                }
            }
        }
    }

    impl<'a> Drop for ParallelMove<'a> {
        fn drop(&mut self) {
            self.execute();
        }
    }
}