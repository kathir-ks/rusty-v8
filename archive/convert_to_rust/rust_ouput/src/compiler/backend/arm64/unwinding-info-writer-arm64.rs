// Converted from V8 C++ source files:
// Header: unwinding-info-writer-arm64.h
// Implementation: unwinding-info-writer-arm64.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::cell::RefCell;
use std::rc::Rc;
use std::vec::Vec;

mod eh_frame {
    pub struct EhFrameWriter { }
    impl EhFrameWriter {
        pub fn Initialize(&mut self) {}
        pub fn AdvanceLocation(&mut self, pc_offset: i32) {}
        pub fn RecordRegisterSavedToStack(&mut self, reg: i32, offset: i32) {}
        pub fn RecordRegisterFollowsInitialRule(&mut self, reg: i32) {}
        pub fn SetBaseAddressRegisterAndOffset(&mut self, reg: i32, offset: i32) {}
        pub fn Finish(&mut self, code_size: i32) {}
    }
}

mod flags {
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

    lazy_static::lazy_static! {
        pub static ref v8_flags: Flags = Flags::new();
    }
}

use flags::v8_flags;

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
        RpoNumber { value: 0 }
    }

    pub fn successors(&self) -> Vec<RpoNumber> {
        Vec::new()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RpoNumber {
    value: i32,
}

impl RpoNumber {
    pub fn ToInt(&self) -> i32 {
        self.value
    }
}

pub struct Register {}

const kSystemPointerSize: i32 = 8;
const lr: i32 = 30;
const fp: i32 = 29;

pub struct UnwindingInfoWriter {
    zone_: Rc<Zone>,
    eh_frame_writer_: RefCell<eh_frame::EhFrameWriter>,
    saved_lr_: RefCell<bool>,
    block_will_exit_: RefCell<bool>,
    block_initial_states_: RefCell<Vec<Option<Box<BlockInitialState>>>>,
}

impl UnwindingInfoWriter {
    pub fn new(zone: Rc<Zone>) -> Self {
        let mut writer = UnwindingInfoWriter {
            zone_: zone.clone(),
            eh_frame_writer_: RefCell::new(eh_frame::EhFrameWriter {}),
            saved_lr_: RefCell::new(false),
            block_will_exit_: RefCell::new(false),
            block_initial_states_: RefCell::new(Vec::new()),
        };

        if writer.enabled() {
            writer.eh_frame_writer_.borrow_mut().Initialize();
        }

        writer
    }

    pub fn SetNumberOfInstructionBlocks(&self, number: i32) {
        if self.enabled() {
            self.block_initial_states_.borrow_mut().resize(number as usize, None);
        }
    }

    pub fn BeginInstructionBlock(&self, pc_offset: i32, block: &InstructionBlock) {
        if !self.enabled() {
            return;
        }

        *self.block_will_exit_.borrow_mut() = false;

        let rpo_number = block.rpo_number().ToInt();
        if rpo_number as usize >= self.block_initial_states_.borrow().len() {
            return;
        }

        let initial_state = self.block_initial_states_.borrow()[rpo_number as usize].as_ref();
        if initial_state.is_none() {
            return;
        }

        let initial_state = initial_state.unwrap();
        if initial_state.saved_lr_ != *self.saved_lr_.borrow() {
            self.eh_frame_writer_.borrow_mut().AdvanceLocation(pc_offset);
            if initial_state.saved_lr_ {
                self.eh_frame_writer_.borrow_mut().RecordRegisterSavedToStack(lr, kSystemPointerSize);
                self.eh_frame_writer_.borrow_mut().RecordRegisterSavedToStack(fp, 0);
            } else {
                self.eh_frame_writer_.borrow_mut().RecordRegisterFollowsInitialRule(lr);
            }
            *self.saved_lr_.borrow_mut() = initial_state.saved_lr_;
        }
    }

    pub fn EndInstructionBlock(&self, block: &InstructionBlock) {
        if !self.enabled() || *self.block_will_exit_.borrow() {
            return;
        }

        for successor in block.successors() {
            let successor_index = successor.ToInt();
            if successor_index as usize >= self.block_initial_states_.borrow().len() {
                continue;
            }

            let mut initial_states = self.block_initial_states_.borrow_mut();
            let existing_state = &initial_states[successor_index as usize];

            if let Some(existing_state) = existing_state {
                assert_eq!(existing_state.saved_lr_, *self.saved_lr_.borrow());
            } else {
                let new_state = self.zone_.New(BlockInitialState { saved_lr_: *self.saved_lr_.borrow() });
                initial_states[successor_index as usize] = Some(new_state);
            }
        }
    }

    pub fn MarkFrameConstructed(&self, at_pc: i32) {
        if !self.enabled() {
            return;
        }

        self.eh_frame_writer_.borrow_mut().AdvanceLocation(at_pc);
        self.eh_frame_writer_.borrow_mut().RecordRegisterSavedToStack(lr, kSystemPointerSize);
        self.eh_frame_writer_.borrow_mut().RecordRegisterSavedToStack(fp, 0);
        *self.saved_lr_.borrow_mut() = true;
    }

    pub fn MarkFrameDeconstructed(&self, at_pc: i32) {
        if !self.enabled() {
            return;
        }

        self.eh_frame_writer_.borrow_mut().AdvanceLocation(at_pc);
        self.eh_frame_writer_.borrow_mut().RecordRegisterFollowsInitialRule(lr);
        *self.saved_lr_.borrow_mut() = false;
    }

    pub fn MarkLinkRegisterOnTopOfStack(&self, pc_offset: i32, sp: &Register) {
        if !self.enabled() {
            return;
        }

        self.eh_frame_writer_.borrow_mut().AdvanceLocation(pc_offset);
        self.eh_frame_writer_.borrow_mut().SetBaseAddressRegisterAndOffset(0, 0);
        self.eh_frame_writer_.borrow_mut().RecordRegisterSavedToStack(lr, 0);
    }

    pub fn MarkPopLinkRegisterFromTopOfStack(&self, pc_offset: i32) {
        if !self.enabled() {
            return;
        }

        self.eh_frame_writer_.borrow_mut().AdvanceLocation(pc_offset);
        self.eh_frame_writer_.borrow_mut().SetBaseAddressRegisterAndOffset(fp, 0);
        self.eh_frame_writer_.borrow_mut().RecordRegisterFollowsInitialRule(lr);
    }

    pub fn Finish(&self, code_size: i32) {
        if self.enabled()) {
            self.eh_frame_writer_.borrow_mut().Finish(code_size);
        }
    }

    pub fn eh_frame_writer(&self) -> Option<std::cell::RefMut<'_, eh_frame::EhFrameWriter>> {
        if self.enabled() {
            Some(self.eh_frame_writer_.borrow_mut())
        } else {
            None
        }
    }

    fn enabled(&self) -> bool {
        v8_flags.perf_prof_unwinding_info
    }

    pub fn MarkBlockWillExit(&self) {
        *self.block_will_exit_.borrow_mut() = true;
    }
}

pub struct BlockInitialState {
    saved_lr_: bool,
}

