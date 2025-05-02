// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod liftoff_assembler; // Assuming this exists and is adjacent
use liftoff_assembler::*;

mod var_state; // Assuming this exists and is adjacent
use var_state::VarState;

mod liftoff_register;
use liftoff_register::*;

mod wasm_value;
use wasm_value::*;

const kInt32Size: usize = 4;

fn value_kind_size(kind: ValueKind) -> usize {
    match kind {
        ValueKind::I32 => 4,
        ValueKind::I64 => 8,
        ValueKind::F32 => 4,
        ValueKind::F64 => 8,
        ValueKind::S128 => 16,
        ValueKind::Ref => 8, // Assuming Ref is a pointer-sized type
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RegisterLoadKind {
    kNop,
    kConstant,
    kStack,
    kLowHalfStack,
    kHighHalfStack,
}

#[derive(Debug)]
struct RegisterLoad {
    load_kind: RegisterLoadKind,
    value: i32, // Or appropriate integer type
    kind: ValueKind,
}

struct RegisterMove {
    src: LiftoffRegister,
    kind: ValueKind,
}

struct ParallelMove<'a> {
    asm_: &'a mut LiftoffAssembler,
    move_dst_regs_: LiftoffRegisterSet,
    load_dst_regs_: LiftoffRegisterSet,
    src_reg_use_counts_: std::collections::HashMap<LiftoffRegister, u32>, // Replace LiftoffRegisterMap<uint32_t>
    register_moves_: std::collections::HashMap<LiftoffRegister, RegisterMove>, // Replace LiftoffRegisterMap<RegisterMove>
    register_loads_: std::collections::HashMap<LiftoffRegister, RegisterLoad>, // Replace LiftoffRegisterMap<RegisterLoad>
    last_spill_offset_: i32,
}

impl<'a> ParallelMove<'a> {
    pub fn new(asm_: &'a mut LiftoffAssembler) -> Self {
        ParallelMove {
            asm_,
            move_dst_regs_: LiftoffRegisterSet::new(),
            load_dst_regs_: LiftoffRegisterSet::new(),
            src_reg_use_counts_: std::collections::HashMap::new(),
            register_moves_: std::collections::HashMap::new(),
            register_loads_: std::collections::HashMap::new(),
            last_spill_offset_: 0,
        }
    }

    fn src_reg_use_count(&mut self, reg: LiftoffRegister) -> &mut u32 {
        self.src_reg_use_counts_.entry(reg).or_insert(0)
    }

    fn register_move(&self, dst: LiftoffRegister) -> &RegisterMove {
        self.register_moves_.get(&dst).expect("RegisterMove not found")
    }

    fn register_load(&self, dst: LiftoffRegister) -> &RegisterLoad {
        self.register_loads_.get(&dst).expect("RegisterLoad not found")
    }

    pub fn transfer_to_stack(&mut self, dst_offset: i32, src: &VarState) {
        match src.loc() {
            VarState::Location::Stack => {
                // Same offsets can happen even if we move values down in the value stack,
                // because of alignment.
                if src.offset() == dst_offset {
                    return;
                }
                #[cfg(debug_assertions)]
                {
                    // Check that the stack value at `dst_offset` is not used in a pending
                    // register load.
                    for reg in self.load_dst_regs_.iter() {
                        // Assuming that LiftoffRegisterSet can be iterated
                        if reg.is_pair() { continue; }
                        if let Some(load) = self.register_loads_.get(&reg) {
                            if load.load_kind == RegisterLoadKind::kStack ||
                               load.load_kind == RegisterLoadKind::kLowHalfStack {
                                // We overwrite the lower half of the stack value for sure.
                                assert_ne!(load.value, dst_offset);
                            } else if load.load_kind == RegisterLoadKind::kHighHalfStack &&
                                       value_kind_size(src.kind()) > kInt32Size {
                                // We overwrite the full stack slot, but we still need the higher half
                                // later.
                                assert_ne!(load.value, dst_offset);
                            }
                        }
                    }
                }
                self.asm_.move_stack_value(dst_offset, src.offset(), src.kind());
            }
            VarState::Location::Register => {
                self.asm_.spill(dst_offset, src.reg(), src.kind());
            }
            VarState::Location::IntConst => {
                self.asm_.spill_const(dst_offset, src.constant());
            }
        }
    }

    pub fn execute_moves(&mut self) {
        // Execute all moves whose {dst} is not being used as src in another move.
        // If any src count drops to zero, also (transitively) execute the
        // corresponding move to that register.
        let mut dsts_to_remove = Vec::new();
        for dst in self.move_dst_regs_.iter() {
            // Check if already handled via transitivity in {ClearExecutedMove}.
            if !self.move_dst_regs_.has(dst) {
                continue;
            }
            if *self.src_reg_use_count(dst) > 0 {
                continue;
            }
            self.execute_move(dst);
            dsts_to_remove.push(dst); //Avoid mutating while iterating
        }
        for dst in dsts_to_remove {
            self.move_dst_regs_.remove(dst);
        }

        // All remaining moves are parts of a cycle. Just spill the first one, then
        // process all remaining moves in that cycle. Repeat for all cycles.
        while !self.move_dst_regs_.is_empty() {
            // TODO(clemensb): Use an unused register if available.
            let dst = self.move_dst_regs_.get_first_reg_set();
            let move_ = self.register_move(dst);
            self.last_spill_offset_ += LiftoffAssembler::slot_size_for_type(move_.kind);
            let spill_reg = move_.src;
            self.asm_.spill(self.last_spill_offset_, spill_reg, move_.kind);
            // Remember to reload into the destination register later.
            self.load_stack_slot(dst, self.last_spill_offset_, move_.kind);
            self.clear_executed_move(dst);
        }
    }

    fn execute_move(&mut self, dst: LiftoffRegister) {
        let move_ = self.register_moves_.remove(&dst).expect("Register move missing");
        self.asm_.move_reg(dst, move_.src, move_.kind);
        self.clear_executed_move(dst);
    }

    fn clear_executed_move(&mut self, dst: LiftoffRegister) {
        if let Some(move_) = self.register_moves_.get(&dst) {
            let src = move_.src;
            let count = self.src_reg_use_counts_.get_mut(&src).expect("Src use count missing");
            *count -= 1;
        }
    }

    fn load_stack_slot(&mut self, dst: LiftoffRegister, offset: i32, kind: ValueKind) {
        self.load_dst_regs_.insert(dst);
        self.register_loads_.insert(dst, RegisterLoad {
            load_kind: RegisterLoadKind::kStack,
            value: offset,
            kind,
        });
    }

    pub fn execute_loads(&mut self) {
        let dsts_to_remove: Vec<LiftoffRegister> = self.load_dst_regs_.iter().collect();
        for dst in dsts_to_remove {
            if let Some(load) = self.register_loads_.get(&dst) {
                match load.load_kind {
                    RegisterLoadKind::kNop => {}
                    RegisterLoadKind::kConstant => {
                        self.asm_.load_constant(dst, if load.kind == ValueKind::I64 {
                            WasmValue::I64(load.value as i64)
                        } else {
                            WasmValue::I32(load.value)
                        });
                    }
                    RegisterLoadKind::kStack => {
                        if kNeedS128RegPair && load.kind == ValueKind::S128 {
                            let fp_pair = LiftoffRegister::for_fp_pair(dst.fp());
                            self.asm_.fill(fp_pair, load.value, load.kind);
                        } else {
                            self.asm_.fill(dst, load.value, load.kind);
                        }
                    }
                    RegisterLoadKind::kLowHalfStack => {
                        // Half of a register pair, {dst} must be a gp register.
                        self.asm_.fill_i64_half(dst.gp(), load.value, Half::LowWord);
                    }
                    RegisterLoadKind::kHighHalfStack => {
                        // Half of a register pair, {dst} must be a gp register.
                        self.asm_.fill_i64_half(dst.gp(), load.value, Half::HighWord);
                    }
                }
            }
            self.register_loads_.remove(&dst);
        }
        self.load_dst_regs_ = LiftoffRegisterSet::new();
    }

    pub fn queue_move(&mut self, dst: LiftoffRegister, src: LiftoffRegister, kind: ValueKind) {
        if dst == src {
            return;
        }
        if !self.move_dst_regs_.has(dst) {
            self.move_dst_regs_.insert(dst);
        }
        self.register_moves_.insert(dst, RegisterMove { src, kind });
        *self.src_reg_use_count(src) += 1;
    }

    pub fn queue_load_const(&mut self, dst: LiftoffRegister, value: i32, kind: ValueKind) {
        if !self.load_dst_regs_.has(dst) {
            self.load_dst_regs_.insert(dst);
        }
        self.register_loads_.insert(dst, RegisterLoad {
            load_kind: RegisterLoadKind::kConstant,
            value,
            kind,
        });
    }

    pub fn queue_load_stack(&mut self, dst: LiftoffRegister, offset: i32, kind: ValueKind) {
        if !self.load_dst_regs_.has(dst) {
            self.load_dst_regs_.insert(dst);
        }
        self.register_loads_.insert(dst, RegisterLoad {
            load_kind: RegisterLoadKind::kStack,
            value: offset,
            kind,
        });
    }

    pub fn queue_load_low_half_stack(&mut self, dst: LiftoffRegister, offset: i32) {
        if !self.load_dst_regs_.has(dst) {
            self.load_dst_regs_.insert(dst);
        }
        self.register_loads_.insert(dst, RegisterLoad {
            load_kind: RegisterLoadKind::kLowHalfStack,
            value: offset,
            kind: ValueKind::I64, // Assuming I64 for halves
        });
    }

    pub fn queue_load_high_half_stack(&mut self, dst: LiftoffRegister, offset: i32) {
        if !self.load_dst_regs_.has(dst) {
            self.load_dst_regs_.insert(dst);
        }
        self.register_loads_.insert(dst, RegisterLoad {
            load_kind: RegisterLoadKind::kHighHalfStack,
            value: offset,
            kind: ValueKind::I64, // Assuming I64 for halves
        });
    }
}