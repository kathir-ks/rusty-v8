// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/compiler/backend/ppc/unwinding-info-writer-ppc.h (converted to Rust module definition)
mod unwinding_info_writer_ppc {
    use std::cell::RefCell;
    use std::rc::Rc;

    pub struct BlockInitialState {
        pub saved_lr_: bool,
    }

    pub struct UnwindingInfoWriter {
        enabled_: bool,
        saved_lr_: bool,
        block_will_exit_: bool,
        block_initial_states_: Vec<Option<Rc<BlockInitialState>>>,
        eh_frame_writer_: EhFrameWriter,
        zone_: Rc<RefCell<Zone>>, // Needs an actual zone allocator

    }

    impl UnwindingInfoWriter {
        pub fn new(enabled: bool, eh_frame_writer: EhFrameWriter, zone: Rc<RefCell<Zone>>) -> Self {
            UnwindingInfoWriter {
                enabled_: enabled,
                saved_lr_: false,
                block_will_exit_: false,
                block_initial_states_: Vec::new(),
                eh_frame_writer_: eh_frame_writer,
                zone_: zone,
            }
        }

        pub fn begin_instruction_block(&mut self, pc_offset: i32, block: &InstructionBlock) {
            if !self.enabled() {
                return;
            }

            self.block_will_exit_ = false;

            assert!(block.rpo_number.to_i32() < self.block_initial_states_.len() as i32);

            let initial_state = self.block_initial_states_[block.rpo_number.to_i32() as usize].clone();

            if initial_state.is_none() {
                return;
            }

            let initial_state = initial_state.unwrap();

            if initial_state.saved_lr_ != self.saved_lr_ {
                self.eh_frame_writer_.advance_location(pc_offset);
                if initial_state.saved_lr_ {
                    self.eh_frame_writer_.record_register_saved_to_stack(
                        K_LR_DWARF_CODE,
                        K_SYSTEM_POINTER_SIZE as i32,
                    );
                    self.eh_frame_writer_.record_register_saved_to_stack(FP, 0);
                } else {
                    self.eh_frame_writer_
                        .record_register_follows_initial_rule(K_LR_DWARF_CODE);
                }
                self.saved_lr_ = initial_state.saved_lr_;
            }
        }

        pub fn end_instruction_block(&mut self, block: &InstructionBlock) {
            if !self.enabled() || self.block_will_exit_ {
                return;
            }

            for successor in &block.successors {
                let successor_index = successor.to_i32();
                assert!(successor_index < self.block_initial_states_.len() as i32);

                let existing_state = self.block_initial_states_[successor_index as usize].clone();

                if let Some(existing_state) = existing_state {
                    assert_eq!(existing_state.saved_lr_, self.saved_lr_);
                } else {
                    let new_state = Rc::new(BlockInitialState { saved_lr_: self.saved_lr_ });
                    //Use the Zone allocator to allocate the memory
                    let mut zone = self.zone_.borrow_mut();
                    if (successor_index as usize) >= self.block_initial_states_.len() {
                        self.block_initial_states_.resize((successor_index + 1) as usize, None);
                    }
                    self.block_initial_states_[successor_index as usize] = Some(new_state);
                }
            }
        }

        pub fn mark_frame_constructed(&mut self, at_pc: i32) {
            if !self.enabled() {
                return;
            }

            self.eh_frame_writer_.advance_location(at_pc);
            self.eh_frame_writer_.record_register_saved_to_stack(
                K_LR_DWARF_CODE,
                K_SYSTEM_POINTER_SIZE as i32,
            );
            self.eh_frame_writer_.record_register_saved_to_stack(FP, 0);
            self.saved_lr_ = true;
        }

        pub fn mark_frame_deconstructed(&mut self, at_pc: i32) {
            if !self.enabled() {
                return;
            }

            self.eh_frame_writer_.advance_location(at_pc);
            self.eh_frame_writer_
                .record_register_follows_initial_rule(K_LR_DWARF_CODE);
            self.saved_lr_ = false;
        }

        pub fn mark_link_register_on_top_of_stack(&mut self, pc_offset: i32) {
            if !self.enabled() {
                return;
            }

            self.eh_frame_writer_.advance_location(pc_offset);
            self.eh_frame_writer_
                .set_base_address_register_and_offset(SP, 0);
            self.eh_frame_writer_
                .record_register_saved_to_stack(K_LR_DWARF_CODE, 0);
        }

        pub fn mark_pop_link_register_from_top_of_stack(&mut self, pc_offset: i32) {
            if !self.enabled() {
                return;
            }

            self.eh_frame_writer_.advance_location(pc_offset);
            self.eh_frame_writer_
                .set_base_address_register_and_offset(FP, 0);
            self.eh_frame_writer_
                .record_register_follows_initial_rule(K_LR_DWARF_CODE);
        }

        fn enabled(&self) -> bool {
            self.enabled_
        }
    }

    // Dummy structs and constants to satisfy the compiler.  These would be defined
    // elsewhere in the V8 codebase.
    pub struct InstructionBlock {
        pub rpo_number: RpoNumber,
        pub successors: Vec<RpoNumber>,
    }

    pub struct RpoNumber {
        number: i32,
    }

    impl RpoNumber {
        pub fn to_i32(&self) -> i32 {
            self.number
        }
    }

    pub struct EhFrameWriter {}

    impl EhFrameWriter {
        pub fn advance_location(&mut self, _pc_offset: i32) {}
        pub fn record_register_saved_to_stack(&mut self, _reg: i32, _offset: i32) {}
        pub fn record_register_follows_initial_rule(&mut self, _reg: i32) {}
        pub fn set_base_address_register_and_offset(&mut self, _reg: i32, _offset: i32) {}
    }

    const K_LR_DWARF_CODE: i32 = 1;
    const K_SYSTEM_POINTER_SIZE: usize = 8;
    const FP: i32 = 2;
    const SP: i32 = 3;

    // Placeholder for a Zone allocator. Requires an implementation.
    pub struct Zone {}

    impl Zone {
        pub fn new() -> Self {
            Zone {}
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;
use unwinding_info_writer_ppc::*;
