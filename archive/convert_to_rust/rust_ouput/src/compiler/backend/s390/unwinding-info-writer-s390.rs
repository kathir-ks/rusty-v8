// Converted from V8 C++ source files:
// Header: unwinding-info-writer-s390.h
// Implementation: unwinding-info-writer-s390.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod eh_frame {
    pub struct EhFrameWriter {
    }
    impl EhFrameWriter {
        pub fn Initialize(&mut self) {}
        pub fn AdvanceLocation(&mut self, pc_offset: i32) {}
        pub fn RecordRegisterSavedToStack(&mut self, register: i32, offset: i32) {}
        pub fn RecordRegisterFollowsInitialRule(&mut self, register: i32) {}
        pub fn Finish(&mut self, code_size: i32) {}
        pub fn SetBaseAddressRegisterAndOffset(&mut self, register: i32, offset: i32) {}
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
}

use std::cell::RefCell;
use std::rc::Rc;

const kSystemPointerSize: i32 = 8;

thread_local! {
    pub static v8_flags: RefCell<flags::Flags> = RefCell::new(flags::Flags::new());
}

pub struct Zone {}

impl Zone {
    pub fn new() -> Self {
        Zone {}
    }
    pub fn New<T>(&mut self, value: bool) -> Box<BlockInitialState> {
        Box::new(BlockInitialState::new(value))
    }
}

pub struct InstructionBlock {
    rpo_number_: RpoNumber,
}

impl InstructionBlock {
    pub fn rpo_number(&self) -> RpoNumber {
        self.rpo_number_
    }
    pub fn successors(&self) -> Vec<RpoNumber> {
        vec![]
    }
}

pub struct RpoNumber {
    number: i32,
}

impl RpoNumber {
    pub fn ToInt(&self) -> i32 {
        self.number
    }
}

pub struct UnwindingInfoWriter {
    zone_: Rc<RefCell<Zone>>,
    eh_frame_writer_: eh_frame::EhFrameWriter,
    saved_lr_: bool,
    block_will_exit_: bool,
    block_initial_states_: Vec<Option<Box<BlockInitialState>>>,
}

const r14: i32 = 14;
const fp: i32 = 11;
const sp: i32 = 15;

impl UnwindingInfoWriter {
    pub fn new(zone: Rc<RefCell<Zone>>) -> Self {
        let mut writer = UnwindingInfoWriter {
            zone_: zone.clone(),
            eh_frame_writer_: eh_frame::EhFrameWriter {},
            saved_lr_: false,
            block_will_exit_: false,
            block_initial_states_: Vec::new(),
        };

        v8_flags.with(|flags| {
            if flags.borrow().perf_prof_unwinding_info {
                writer.eh_frame_writer_.Initialize();
            }
        });

        writer
    }

    pub fn SetNumberOfInstructionBlocks(&mut self, number: i32) {
        v8_flags.with(|flags| {
            if flags.borrow().perf_prof_unwinding_info {
                self.block_initial_states_.resize(number as usize, None);
            }
        });
    }

    pub fn BeginInstructionBlock(&mut self, pc_offset: i32, block: &InstructionBlock) {
        v8_flags.with(|flags| {
            if !flags.borrow().perf_prof_unwinding_info {
                return;
            }

            self.block_will_exit_ = false;

            let rpo_number = block.rpo_number().ToInt() as usize;

            if rpo_number < self.block_initial_states_.len() {
                if let Some(initial_state) = &self.block_initial_states_[rpo_number] {
                    if initial_state.saved_lr_ != self.saved_lr_ {
                        self.eh_frame_writer_.AdvanceLocation(pc_offset);
                        if initial_state.saved_lr_ {
                            self.eh_frame_writer_.RecordRegisterSavedToStack(r14, kSystemPointerSize);
                            self.eh_frame_writer_.RecordRegisterSavedToStack(fp, 0);
                        } else {
                            self.eh_frame_writer_.RecordRegisterFollowsInitialRule(r14);
                        }
                        self.saved_lr_ = initial_state.saved_lr_;
                    }
                }
            }
        });
    }

    pub fn EndInstructionBlock(&mut self, block: &InstructionBlock) {
        v8_flags.with(|flags| {
            if !flags.borrow().perf_prof_unwinding_info || self.block_will_exit_ {
                return;
            }

            for successor in block.successors() {
                let successor_index = successor.ToInt() as usize;

                if successor_index < self.block_initial_states_.len() {
                    let existing_state = &self.block_initial_states_[successor_index];

                    if let Some(state) = existing_state {
                        assert_eq!(state.saved_lr_, self.saved_lr_);
                    } else {
                        let mut zone = self.zone_.borrow_mut();
                        self.block_initial_states_[successor_index] = Some(zone.New(self.saved_lr_));
                    }
                }
            }
        });
    }

    pub fn MarkLinkRegisterOnTopOfStack(&mut self, pc_offset: i32) {
        v8_flags.with(|flags| {
            if !flags.borrow().perf_prof_unwinding_info {
                return;
            }

            self.eh_frame_writer_.AdvanceLocation(pc_offset);
            self.eh_frame_writer_.SetBaseAddressRegisterAndOffset(sp, 0);
            self.eh_frame_writer_.RecordRegisterSavedToStack(r14, 0);
        });
    }

    pub fn MarkPopLinkRegisterFromTopOfStack(&mut self, pc_offset: i32) {
        v8_flags.with(|flags| {
            if !flags.borrow().perf_prof_unwinding_info {
                return;
            }

            self.eh_frame_writer_.AdvanceLocation(pc_offset);
            self.eh_frame_writer_.SetBaseAddressRegisterAndOffset(fp, 0);
            self.eh_frame_writer_.RecordRegisterFollowsInitialRule(r14);
        });
    }

    pub fn MarkFrameConstructed(&mut self, at_pc: i32) {
        v8_flags.with(|flags| {
            if !flags.borrow().perf_prof_unwinding_info {
                return;
            }

            self.eh_frame_writer_.AdvanceLocation(at_pc);
            self.eh_frame_writer_.RecordRegisterSavedToStack(r14, kSystemPointerSize);
            self.eh_frame_writer_.RecordRegisterSavedToStack(fp, 0);
            self.saved_lr_ = true;
        });
    }

    pub fn MarkFrameDeconstructed(&mut self, at_pc: i32) {
        v8_flags.with(|flags| {
            if !flags.borrow().perf_prof_unwinding_info {
                return;
            }

            self.eh_frame_writer_.AdvanceLocation(at_pc);
            self.eh_frame_writer_.RecordRegisterFollowsInitialRule(r14);
            self.saved_lr_ = false;
        });
    }

    pub fn MarkBlockWillExit(&mut self) {
        self.block_will_exit_ = true;
    }

    pub fn Finish(&mut self, code_size: i32) {
        v8_flags.with(|flags| {
            if flags.borrow().perf_prof_unwinding_info {
                self.eh_frame_writer_.Finish(code_size);
            }
        });
    }

    pub fn eh_frame_writer(&mut self) -> Option<&mut eh_frame::EhFrameWriter> {
        v8_flags.with(|flags| {
            if flags.borrow().perf_prof_unwinding_info {
                Some(&mut self.eh_frame_writer_)
            } else {
                None
            }
        })
    }

    fn enabled(&self) -> bool {
        v8_flags.with(|flags| flags.borrow().perf_prof_unwinding_info)
    }
}

pub struct BlockInitialState {
    pub saved_lr_: bool,
}

impl BlockInitialState {
    pub fn new(saved_lr: bool) -> Self {
        BlockInitialState {
            saved_lr_: saved_lr,
        }
    }
}
