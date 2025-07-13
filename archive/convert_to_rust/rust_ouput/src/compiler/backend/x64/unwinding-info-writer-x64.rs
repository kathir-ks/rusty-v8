// Converted from V8 C++ source files:
// Header: unwinding-info-writer-x64.h
// Implementation: unwinding-info-writer-x64.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod diagnostics {
    pub mod eh_frame {
        pub struct EhFrameWriter {
        }

        impl EhFrameWriter {
            pub fn Initialize(&mut self) {}
            pub fn AdvanceLocation(&mut self, pc_offset: i32) {}
            pub fn IncreaseBaseAddressOffset(&mut self, base_delta: i32) {}
            pub fn SetBaseAddressRegisterAndOffset(&mut self, reg: Register, offset: i32) {}
            pub fn SetBaseAddressRegister(&mut self, reg: Register) {}
            pub fn SetBaseAddressOffset(&mut self, offset: i32) {}
            pub fn Finish(&mut self, code_size: i32) {}
            pub fn base_register(&self) -> Register { Register {} }
            pub fn base_offset(&self) -> i32 { 0 }
            pub fn RecordRegisterSavedToStack(&mut self, reg: Register, top_of_stack: i32) {}
        }
    }
}

pub mod flags {
    pub struct Flags {
        pub perf_prof_unwinding_info: bool,
    }

    impl Flags {
        pub fn new() -> Self {
            Flags {
                perf_prof_unwinding_info: false,
            }
        }
    }
    
    pub static mut v8_flags: Flags = Flags { perf_prof_unwinding_info: false };
}

use std::cell::RefCell;
use std::rc::Rc;
use crate::flags::v8_flags;

pub struct Zone {}

impl Zone {
    pub fn new() -> Self {
        Zone {}
    }

    pub fn New<T>(&self, value: T) -> Box<T> {
        Box::new(value)
    }
}

pub struct InstructionBlock {}

impl InstructionBlock {
    pub fn rpo_number(&self) -> RpoNumber {
        RpoNumber { number_: 0 }
    }

    pub fn successors(&self) -> Vec<RpoNumber> {
        Vec::new()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Register {}

const kInt64Size: i32 = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RpoNumber {
    number_: i32,
}

impl RpoNumber {
    pub fn ToInt(&self) -> i32 {
        self.number_
    }
}

pub struct UnwindingInfoWriter {
    zone_: Rc<RefCell<Zone>>,
    eh_frame_writer_: diagnostics::eh_frame::EhFrameWriter,
    tracking_fp_: bool,
    block_will_exit_: bool,
    block_initial_states_: Vec<Option<Box<BlockInitialState>>>,
}

impl UnwindingInfoWriter {
    pub fn new(zone: Rc<RefCell<Zone>>) -> Self {
        let mut writer = UnwindingInfoWriter {
            zone_: zone.clone(),
            eh_frame_writer_: diagnostics::eh_frame::EhFrameWriter {},
            tracking_fp_: false,
            block_will_exit_: false,
            block_initial_states_: Vec::new(),
        };
        if writer.enabled() {
            writer.eh_frame_writer_.Initialize();
        }
        writer
    }

    pub fn MaybeIncreaseBaseOffsetAt(&mut self, pc_offset: i32, base_delta: i32) {
        if self.enabled() && !self.tracking_fp_ {
            self.eh_frame_writer_.AdvanceLocation(pc_offset);
            self.eh_frame_writer_.IncreaseBaseAddressOffset(base_delta);
        }
    }

    pub fn SetNumberOfInstructionBlocks(&mut self, number: i32) {
        if self.enabled() {
            self.block_initial_states_.resize(number as usize, None);
        }
    }

    pub fn BeginInstructionBlock(&mut self, pc_offset: i32, block: &InstructionBlock) {
        if !self.enabled() {
            return;
        }

        self.block_will_exit_ = false;

        if block.rpo_number().ToInt() as usize >= self.block_initial_states_.len() {
            return;
        }
        
        let initial_state = &self.block_initial_states_[block.rpo_number().ToInt() as usize];
        if initial_state.is_none() {
            return;
        }
        let initial_state = initial_state.as_ref().unwrap();
        
        if initial_state.register_ != self.eh_frame_writer_.base_register() &&
           initial_state.offset_ != self.eh_frame_writer_.base_offset() {
            self.eh_frame_writer_.AdvanceLocation(pc_offset);
            self.eh_frame_writer_.SetBaseAddressRegisterAndOffset(initial_state.register_, initial_state.offset_);
        } else if initial_state.register_ != self.eh_frame_writer_.base_register() {
            self.eh_frame_writer_.AdvanceLocation(pc_offset);
            self.eh_frame_writer_.SetBaseAddressRegister(initial_state.register_);
        } else if initial_state.offset_ != self.eh_frame_writer_.base_offset() {
            self.eh_frame_writer_.AdvanceLocation(pc_offset);
            self.eh_frame_writer_.SetBaseAddressOffset(initial_state.offset_);
        }

        self.tracking_fp_ = initial_state.tracking_fp_;
    }

    pub fn EndInstructionBlock(&mut self, block: &InstructionBlock) {
        if !self.enabled() || self.block_will_exit_ {
            return;
        }

        for successor in block.successors() {
            let successor_index = successor.ToInt() as usize;
            if successor_index >= self.block_initial_states_.len() {
                continue;
            }

            let existing_state = &self.block_initial_states_[successor_index];

            if let Some(existing_state) = existing_state {
                if existing_state.register_ != self.eh_frame_writer_.base_register() {
                   continue;
                }
                if existing_state.offset_ != self.eh_frame_writer_.base_offset() {
                    continue;
                }
                if existing_state.tracking_fp_ != self.tracking_fp_ {
                    continue;
                }
            } else {
                let new_state = BlockInitialState::new(
                    self.eh_frame_writer_.base_register(),
                    self.eh_frame_writer_.base_offset(),
                    self.tracking_fp_,
                );
                
                self.block_initial_states_[successor_index] = Some(Box::new(new_state));
            }
        }
    }

    pub fn MarkFrameConstructed(&mut self, pc_base: i32) {
        if !self.enabled() {
            return;
        }

        self.eh_frame_writer_.AdvanceLocation(pc_base + 1);
        self.eh_frame_writer_.IncreaseBaseAddressOffset(kInt64Size);

        let top_of_stack = -self.eh_frame_writer_.base_offset();
        self.eh_frame_writer_.RecordRegisterSavedToStack(rbp, top_of_stack);

        self.eh_frame_writer_.AdvanceLocation(pc_base + 4);
        self.eh_frame_writer_.SetBaseAddressRegister(rbp);

        self.tracking_fp_ = true;
    }

    pub fn MarkFrameDeconstructed(&mut self, pc_base: i32) {
        if !self.enabled() {
            return;
        }

        self.eh_frame_writer_.AdvanceLocation(pc_base + 3);
        self.eh_frame_writer_.SetBaseAddressRegister(rsp);

        self.eh_frame_writer_.AdvanceLocation(pc_base + 4);
        self.eh_frame_writer_.IncreaseBaseAddressOffset(-kInt64Size);

        self.tracking_fp_ = false;
    }

    pub fn Finish(&mut self, code_size: i32) {
        if self.enabled() {
            self.eh_frame_writer_.Finish(code_size);
        }
    }

    pub fn eh_frame_writer(&mut self) -> Option<&mut diagnostics::eh_frame::EhFrameWriter> {
        if self.enabled() {
            Some(&mut self.eh_frame_writer_)
        } else {
            None
        }
    }

    fn enabled(&self) -> bool {
        unsafe { v8_flags.perf_prof_unwinding_info }
    }
}

struct BlockInitialState {
    register_: Register,
    offset_: i32,
    tracking_fp_: bool,
}

impl BlockInitialState {
    fn new(register_: Register, offset_: i32, tracking_fp_: bool) -> Self {
        BlockInitialState {
            register_,
            offset_,
            tracking_fp_,
        }
    }
}

const rbp: Register = Register {};
const rsp: Register = Register {};
