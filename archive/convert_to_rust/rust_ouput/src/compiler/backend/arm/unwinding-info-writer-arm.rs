// Converted from V8 C++ source files:
// Header: unwinding-info-writer-arm.h
// Implementation: unwinding-info-writer-arm.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod eh_frame {
    pub struct EhFrameWriter {
        initialized: bool,
    }

    impl EhFrameWriter {
        pub fn new() -> Self {
            EhFrameWriter { initialized: false }
        }

        pub fn Initialize(&mut self) {
            self.initialized = true;
        }

        pub fn AdvanceLocation(&mut self, _pc_offset: i32) {}
        pub fn RecordRegisterSavedToStack(&mut self, _reg: i32, _size: i32) {}
        pub fn RecordRegisterFollowsInitialRule(&mut self, _reg: i32) {}
        pub fn SetBaseAddressRegisterAndOffset(&mut self, _reg: i32, _offset: i32) {}
        pub fn Finish(&mut self, _code_size: i32) {}

        pub fn is_initialized(&self) -> bool {
            self.initialized
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

pub struct UnwindingInfoWriter {
    zone_: Rc<RefCell<Zone>>,
    eh_frame_writer_: eh_frame::EhFrameWriter,
    saved_lr_: bool,
    block_will_exit_: bool,
    block_initial_states_: Vec<Option<Rc<BlockInitialState>>>,
}

pub struct Zone {}

impl Zone {
    pub fn new() -> Self {
        Zone {}
    }

    pub fn New<T>(&self, saved_lr: bool) -> Rc<BlockInitialState> {
        Rc::new(BlockInitialState::new(saved_lr))
    }
}

#[derive(Debug)]
pub enum UnwindingInfoWriterError {}

const lr: i32 = 14;
const sp: i32 = 13;
const fp: i32 = 11;
const kSystemPointerSize: i32 = 4;

impl UnwindingInfoWriter {
    pub fn new(zone: Rc<RefCell<Zone>>) -> Self {
        let mut writer = UnwindingInfoWriter {
            zone_: zone,
            eh_frame_writer_: eh_frame::EhFrameWriter::new(),
            saved_lr_: false,
            block_will_exit_: false,
            block_initial_states_: Vec::new(),
        };

        if writer.enabled() {
            writer.eh_frame_writer_.Initialize();
        }

        writer
    }

    fn enabled(&self) -> bool {
        v8_flags.perf_prof_unwinding_info
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

        if block.rpo_number.0 as usize >= self.block_initial_states_.len() {
            return;
        }

        let initial_state = &self.block_initial_states_[block.rpo_number.0 as usize];
        if let Some(initial_state) = initial_state {
            if initial_state.saved_lr_ != self.saved_lr_ {
                self.eh_frame_writer_.AdvanceLocation(pc_offset);
                if initial_state.saved_lr_ {
                    self.eh_frame_writer_.RecordRegisterSavedToStack(lr, kSystemPointerSize);
                } else {
                    self.eh_frame_writer_.RecordRegisterFollowsInitialRule(lr);
                }
                self.saved_lr_ = initial_state.saved_lr_;
            }
        }
    }

    pub fn EndInstructionBlock(&mut self, block: &InstructionBlock) {
        if !self.enabled() || self.block_will_exit_ {
            return;
        }

        for successor in &block.successors {
            let successor_index = successor.0 as usize;
            if successor_index >= self.block_initial_states_.len() {
                continue;
            }

            let existing_state = &self.block_initial_states_[successor_index];

            match existing_state {
                Some(existing_state) => {
                    assert_eq!(existing_state.saved_lr_, self.saved_lr_);
                }
                None => {
                    let initial_state = self.zone_.borrow().New(self.saved_lr_);
                    self.block_initial_states_[successor_index] = Some(initial_state);
                }
            }
        }
    }

    pub fn MarkFrameConstructed(&mut self, at_pc: i32) {
        if !self.enabled() {
            return;
        }

        self.eh_frame_writer_.AdvanceLocation(at_pc);
        self.eh_frame_writer_.RecordRegisterSavedToStack(lr, kSystemPointerSize);
        self.saved_lr_ = true;
    }

    pub fn MarkFrameDeconstructed(&mut self, at_pc: i32) {
        if !self.enabled() {
            return;
        }

        self.eh_frame_writer_.AdvanceLocation(at_pc);
        self.eh_frame_writer_.RecordRegisterFollowsInitialRule(lr);
        self.saved_lr_ = false;
    }

    pub fn MarkLinkRegisterOnTopOfStack(&mut self, pc_offset: i32) {
        if !self.enabled() {
            return;
        }

        self.eh_frame_writer_.AdvanceLocation(pc_offset);
        self.eh_frame_writer_.SetBaseAddressRegisterAndOffset(sp, 0);
        self.eh_frame_writer_.RecordRegisterSavedToStack(lr, 0);
    }

    pub fn MarkPopLinkRegisterFromTopOfStack(&mut self, pc_offset: i32) {
        if !self.enabled() {
            return;
        }

        self.eh_frame_writer_.AdvanceLocation(pc_offset);
        self.eh_frame_writer_.SetBaseAddressRegisterAndOffset(fp, 0);
        self.eh_frame_writer_.RecordRegisterFollowsInitialRule(lr);
    }

    pub fn MarkBlockWillExit(&mut self) {
        self.block_will_exit_ = true;
    }

    pub fn Finish(&mut self, code_size: i32) {
        if self.enabled() {
            self.eh_frame_writer_.Finish(code_size);
        }
    }

    pub fn eh_frame_writer(&mut self) -> Option<&mut eh_frame::EhFrameWriter> {
        if self.enabled() {
            Some(&mut self.eh_frame_writer_)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub struct BlockInitialState {
    pub saved_lr_: bool,
}

impl BlockInitialState {
    pub fn new(saved_lr: bool) -> Self {
        BlockInitialState { saved_lr_: saved_lr }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RpoNumber(pub i32);

impl RpoNumber {
    pub fn ToInt(&self) -> i32 {
        self.0
    }
}

pub struct InstructionBlock {
    pub rpo_number: RpoNumber,
    pub successors: Vec<RpoNumber>,
}

impl InstructionBlock {
    pub fn new(rpo_number: RpoNumber, successors: Vec<RpoNumber>) -> Self {
        InstructionBlock {
            rpo_number,
            successors,
        }
    }
}

pub struct Flags {
    pub perf_prof_unwinding_info: bool,
}

static mut v8_flags: Flags = Flags {
    perf_prof_unwinding_info: false,
};
