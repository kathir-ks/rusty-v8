// Converted from V8 C++ source files:
// Header: unwinding-info-writer-ppc.h
// Implementation: unwinding-info-writer-ppc.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod diagnostics {
    pub struct EhFrame {}
}
pub mod flags {
    pub struct flags {
        pub perf_prof_unwinding_info: bool,
    }
    pub static mut v8_flags: flags = flags {
        perf_prof_unwinding_info: false,
    };
}
use std::cell::RefCell;
use std::rc::Rc;
pub struct Zone {}
impl Zone {
    pub fn new() -> Zone {
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
        vec![]
    }
}
#[derive(Clone, Copy)]
pub struct RpoNumber {
    number_: i32,
}
impl RpoNumber {
    pub fn ToInt(&self) -> i32 {
        self.number_
    }
}
#[derive(Clone, Copy)]
pub struct DwarfCode(i32);
const kLrDwarfCode: DwarfCode = DwarfCode(1);
const fp: DwarfCode = DwarfCode(2);
const sp: DwarfCode = DwarfCode(3);
pub struct EhFrameWriter {
    zone_: *mut Zone,
    initialized: bool,
}
impl EhFrameWriter {
    pub fn new(zone: *mut Zone) -> EhFrameWriter {
        EhFrameWriter {
            zone_: zone,
            initialized: false,
        }
    }
    pub fn Initialize(&mut self) {
        self.initialized = true;
    }
    pub fn AdvanceLocation(&mut self, pc_offset: i32) {
        if !self.initialized {
            return;
        }
    }
    pub fn RecordRegisterSavedToStack(&mut self, register: DwarfCode, offset: i32) {
        if !self.initialized {
            return;
        }
    }
    pub fn RecordRegisterFollowsInitialRule(&mut self, register: DwarfCode) {
        if !self.initialized {
            return;
        }
    }
    pub fn SetBaseAddressRegisterAndOffset(&mut self, register: DwarfCode, offset: i32) {
        if !self.initialized {
            return;
        }
    }
    pub fn Finish(&mut self, code_size: i32) {
        if !self.initialized {
            return;
        }
    }
}
pub struct UnwindingInfoWriter {
    zone_: *mut Zone,
    eh_frame_writer_: EhFrameWriter,
    saved_lr_: bool,
    block_will_exit_: bool,
    block_initial_states_: Vec<Option<Box<BlockInitialState>>>,
}
impl UnwindingInfoWriter {
    pub fn new(zone: *mut Zone) -> UnwindingInfoWriter {
        let mut writer = UnwindingInfoWriter {
            zone_: zone,
            eh_frame_writer_: EhFrameWriter::new(zone),
            saved_lr_: false,
            block_will_exit_: false,
            block_initial_states_: Vec::new(),
        };
        unsafe {
            if flags::v8_flags.perf_prof_unwinding_info {
                writer.eh_frame_writer_.Initialize();
            }
        }
        writer
    }
    pub fn SetNumberOfInstructionBlocks(&mut self, number: i32) {
        unsafe {
            if flags::v8_flags.perf_prof_unwinding_info {
                self.block_initial_states_
                    .resize(number as usize, None);
            }
        }
    }
    pub fn BeginInstructionBlock(
        &mut self,
        pc_offset: i32,
        block: *const InstructionBlock,
    ) {
        unsafe {
            if !flags::v8_flags.perf_prof_unwinding_info {
                return;
            }
        }
        self.block_will_exit_ = false;
        let block = unsafe { &*block };
        let rpo_number = block.rpo_number().ToInt();
        if rpo_number as usize >= self.block_initial_states_.len() {
            return;
        }
        let initial_state = self.block_initial_states_[rpo_number as usize].as_ref();
        if initial_state.is_none() {
            return;
        }
        let initial_state = initial_state.unwrap();
        if initial_state.saved_lr_ != self.saved_lr_ {
            self.eh_frame_writer_.AdvanceLocation(pc_offset);
            if initial_state.saved_lr_ {
                self.eh_frame_writer_.RecordRegisterSavedToStack(
                    kLrDwarfCode,
                    kSystemPointerSize,
                );
                self.eh_frame_writer_.RecordRegisterSavedToStack(fp, 0);
            } else {
                self.eh_frame_writer_
                    .RecordRegisterFollowsInitialRule(kLrDwarfCode);
            }
            self.saved_lr_ = initial_state.saved_lr_;
        }
    }
    pub fn EndInstructionBlock(&mut self, block: *const InstructionBlock) {
        unsafe {
            if !flags::v8_flags.perf_prof_unwinding_info || self.block_will_exit_ {
                return;
            }
        }
        let block = unsafe { &*block };
        for successor in block.successors() {
            let successor_index = successor.ToInt();
            if successor_index as usize >= self.block_initial_states_.len() {
                continue;
            }
            let existing_state = self.block_initial_states_[successor_index as usize].as_ref();
            if let Some(existing_state) = existing_state {
                assert_eq!(existing_state.saved_lr_, self.saved_lr_);
            } else {
                let new_state = unsafe {
                    (*self.zone_).New(BlockInitialState {
                        saved_lr_: self.saved_lr_,
                    })
                };
                self.block_initial_states_[successor_index as usize] = Some(new_state);
            }
        }
    }
    pub fn MarkFrameConstructed(&mut self, at_pc: i32) {
        unsafe {
            if !flags::v8_flags.perf_prof_unwinding_info {
                return;
            }
        }
        self.eh_frame_writer_.AdvanceLocation(at_pc);
        self.eh_frame_writer_.RecordRegisterSavedToStack(
            kLrDwarfCode,
            kSystemPointerSize,
        );
        self.eh_frame_writer_.RecordRegisterSavedToStack(fp, 0);
        self.saved_lr_ = true;
    }
    pub fn MarkFrameDeconstructed(&mut self, at_pc: i32) {
        unsafe {
            if !flags::v8_flags.perf_prof_unwinding_info {
                return;
            }
        }
        self.eh_frame_writer_.AdvanceLocation(at_pc);
        self.eh_frame_writer_
            .RecordRegisterFollowsInitialRule(kLrDwarfCode);
        self.saved_lr_ = false;
    }
    pub fn MarkLinkRegisterOnTopOfStack(&mut self, pc_offset: i32) {
        unsafe {
            if !flags::v8_flags.perf_prof_unwinding_info {
                return;
            }
        }
        self.eh_frame_writer_.AdvanceLocation(pc_offset);
        self.eh_frame_writer_.SetBaseAddressRegisterAndOffset(sp, 0);
        self.eh_frame_writer_
            .RecordRegisterSavedToStack(kLrDwarfCode, 0);
    }
    pub fn MarkPopLinkRegisterFromTopOfStack(&mut self, pc_offset: i32) {
        unsafe {
            if !flags::v8_flags.perf_prof_unwinding_info {
                return;
            }
        }
        self.eh_frame_writer_.AdvanceLocation(pc_offset);
        self.eh_frame_writer_.SetBaseAddressRegisterAndOffset(fp, 0);
        self.eh_frame_writer_
            .RecordRegisterFollowsInitialRule(kLrDwarfCode);
    }
    pub fn MarkBlockWillExit(&mut self) {
        self.block_will_exit_ = true;
    }
    pub fn Finish(&mut self, code_size: i32) {
        unsafe {
            if flags::v8_flags.perf_prof_unwinding_info {
                self.eh_frame_writer_.Finish(code_size);
            }
        }
    }
    pub fn eh_frame_writer(&mut self) -> Option<&mut EhFrameWriter> {
        unsafe {
            if flags::v8_flags.perf_prof_unwinding_info {
                Some(&mut self.eh_frame_writer_)
            } else {
                None
            }
        }
    }
}
const kSystemPointerSize: i32 = 8;
pub struct BlockInitialState {
    pub saved_lr_: bool,
}
impl BlockInitialState {
    pub fn new(saved_lr: bool) -> BlockInitialState {
        BlockInitialState { saved_lr_: saved_lr }
    }
}
