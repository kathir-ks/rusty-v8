// Converted from V8 C++ source files:
// Header: regexp-macro-assembler-arm.h
// Implementation: regexp-macro-assembler-arm.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod arm {
use std::sync::{Arc, Mutex, RwLock};

use crate::logging::code_events::InstructionStream;
use crate::regexp::ppc::{RegExpFlags, RegExp};
use crate::codegen::macro_assembler::CodeDesc;
use crate::heap::factory::Heap;
use crate::init::bootstrapper::JavaScript;
use crate::init::isolate::Isolate;
use crate::objects::code::CodeKind;
use crate::strings::uri::String;
use crate::codegen::macro_assembler::RelocInfo;
use crate::heap::factory::Factory;
use crate::objects::code::AbstractCode;
use crate::objects::code::Code;
use crate::codegen::assembler::NewAssemblerBuffer;
use crate::regexp::regexp_stack::RegExpStack;
use crate::objects::heap_object::HeapObject;
use crate::objects::fixed_array::FixedArray;
use crate::objects::object::Object;
use crate::baseline::arm::baseline_assembler_arm_inl::ExternalReference;

pub struct RegExpMacroAssemblerARM {
    isolate: *mut Isolate,
    zone: usize, // Zone*
    masm_: Box<MacroAssembler>,
    no_root_array_scope_: usize, // NoRootArrayScope
    mode_: Mode,
    num_registers_: i32,
    num_saved_registers_: i32,
    entry_label_: Label,
    start_label_: Label,
    success_label_: Label,
    backtrack_label_: Label,
    exit_label_: Label,
    check_preempt_label_: Label,
    stack_overflow_label_: Label,
    fallback_label_: Label,
    backtrack_limit_: i32,
    global_: bool,
    global_with_zero_length_check_: bool,
    can_fallback_: bool,
    registers_: Vec<i32>,
    string_start_minus_one_offset_: i32,
    has_backtrack_limit_: bool,
    global_unicode_: bool
}

impl RegExpMacroAssemblerARM {
    const K_REG_EXP_CODE_SIZE: i32 = 1024;
    const K_BACKTRACK_CONSTANT_POOL_SIZE: i32 = 4;
    
    pub fn new(isolate: *mut Isolate, zone: usize, mode: Mode, registers_to_save: i32, backtrack_limit: i32, global: bool, global_with_zero_length_check: bool, can_fallback: bool, registers: Vec<i32>, string_start_minus_one_offset: i32, has_backtrack_limit: bool, global_unicode: bool) -> RegExpMacroAssemblerARM {
        let mut masm_ = Box::new(MacroAssembler::new(isolate, CodeObjectRequired::kYes, NewAssemblerBuffer(RegExpMacroAssemblerARM::K_REG_EXP_CODE_SIZE)));
        masm_.jmp(&Label::new());
        RegExpMacroAssemblerARM {
            isolate,
            zone,
            masm_,
            no_root_array_scope_: 0,
            mode_: mode,
            num_registers_: registers_to_save,
            num_saved_registers_: registers_to_save,
            entry_label_: Label::new(),
            start_label_: Label::new(),
            success_label_: Label::new(),
            backtrack_label_: Label::new(),
            exit_label_: Label::new(),
            check_preempt_label_: Label::new(),
            stack_overflow_label_: Label::new(),
            fallback_label_: Label::new(),
            backtrack_limit,
            global_,
            global_with_zero_length_check,
            can_fallback,
            registers_,
            string_start_minus_one_offset,
            has_backtrack_limit,
            global_unicode
        }
    }

    pub fn aborted_code_generation(&mut self) {
        self.masm_.aborted_code_generation();
        self.entry_label_.unuse();
        self.start_label_.unuse();
        self.success_label_.unuse();
        self.backtrack_label_.unuse();
        self.exit_label_.unuse();
        self.check_preempt_label_.unuse();
        self.stack_overflow_label_.unuse();
        self.fallback_label_.unuse();
    }

    pub fn stack_limit_slack_slot_count(&self) -> i32 {
        RegExpStack::K_STACK_LIMIT_SLACK_SLOT_COUNT
    }

    pub fn advance_current_position(&mut self, by: i32) {
        if by != 0 {
            self.masm_.add(self.registers_[6], self.registers_[6], by * self.char_size());
        }
    }

    pub fn advance_register(&mut self, reg: i32, by: i32) {
        if by != 0 {
            self.masm_.ldr(self.registers_[0], self.register_location(reg));
            self.masm_.add(self.registers_[0], self.registers_[0], by);
            self.masm_.str(self.registers_[0], self.register_location(reg));
        }
    }

    pub fn backtrack(&mut self) {
      self.check_preemption();
      if self.has_backtrack_limit_ {
        let mut next = Label::new();
        self.masm_.ldr(self.registers_[0], MemOperand(self.registers_[11], self.k_backtrack_count_offset()));
        self.masm_.add(self.registers_[0], self.registers_[0], 1);
        self.masm_.str(self.registers_[0], MemOperand(self.registers_[11], self.k_backtrack_count_offset()));
        self.masm_.cmp(self.registers_[0], self.backtrack_limit_);
        self.masm_.b(Condition::Ne, &mut next);
    
        // Backtrack limit exceeded.
        if self.can_fallback() {
          self.masm_.jmp(&mut self.fallback_label_);
        } else {
          // Can't fallback, so we treat it as a failed match.
          self.fail();
        }
    
        self.masm_.bind(&mut next);
      }
      // Pop InstructionStream offset from backtrack stack, add InstructionStream
      // and jump to location.
      self.pop(self.registers_[0]);
      self.masm_.add(15, self.registers_[0], self.masm_.code_object());
    }

    pub fn bind(&mut self, label: &mut Label) {
        self.masm_.bind(label);
    }

    pub fn check_character(&mut self, c: u32, on_equal: &mut Label) {
        self.masm_.cmp(self.registers_[7], c);
        self.branch_or_backtrack(Condition::Eq, on_equal);
    }

    pub fn check_character_gt(&mut self, limit: u16, on_greater: &mut Label) {
        self.masm_.cmp(self.registers_[7], limit);
        self.branch_or_backtrack(Condition::Gt, on_greater);
    }

    pub fn check_at_start(&mut self, cp_offset: i32, on_at_start: &mut Label) {
        self.masm_.ldr(self.registers_[1], MemOperand(self.registers_[11], self.string_start_minus_one_offset_));
        self.masm_.add(self.registers_[0], self.registers_[6], -self.char_size() + cp_offset * self.char_size());
        self.masm_.cmp(self.registers_[0], self.registers_[1]);
        self.branch_or_backtrack(Condition::Eq, on_at_start);
    }

    pub fn check_not_at_start(&mut self, cp_offset: i32, on_not_at_start: &mut Label) {
        self.masm_.ldr(self.registers_[1], MemOperand(self.registers_[11], self.string_start_minus_one_offset_));
        self.masm_.add(self.registers_[0], self.registers_[6], -self.char_size() + cp_offset * self.char_size());
        self.masm_.cmp(self.registers_[0], self.registers_[1]);
        self.branch_or_backtrack(Condition::Ne, on_not_at_start);
    }

    pub fn check_character_lt(&mut self, limit: u16, on_less: &mut Label) {
        self.masm_.cmp(self.registers_[7], limit);
        self.branch_or_backtrack(Condition::Lt, on_less);
    }

    pub fn check_greedy_loop(&mut self, on_equal: &mut Label) {
        self.masm_.ldr(self.registers_[0], MemOperand(self.registers_[8], 0));
        self.masm_.cmp(self.registers_[6], self.registers_[0]);
        self.masm_.add(self.registers_[8], self.registers_[8], 8, LeaveCC::LeaveCC, Condition::Eq);
        self.branch_or_backtrack(Condition::Eq, on_equal);
    }

    pub fn check_not_back_reference_ignore_case(&mut self, start_reg: i32, read_backward: bool, unicode: bool, on_no_match: &mut Label) {
        let mut fallthrough = Label::new();
        self.masm_.ldr(self.registers_[0], self.register_location(start_reg));
        self.masm_.ldr(self.registers_[1], self.register_location(start_reg + 1));
        self.masm_.sub(self.registers_[1], self.registers_[1], self.registers_[0], SetCC::SetCC);
    
        self.masm_.b(Condition::Eq, &mut fallthrough);
    
        if read_backward {
            self.masm_.ldr(self.registers_[3], MemOperand(self.registers_[11], self.string_start_minus_one_offset_));
            self.masm_.add(self.registers_[3], self.registers_[3], self.registers_[1]);
            self.masm_.cmp(self.registers_[6], self.registers_[3]);
            self.branch_or_backtrack(Condition::Le, on_no_match);
        } else {
            self.masm_.cmn(self.registers_[1], self.registers_[6]);
            self.branch_or_backtrack(Condition::Gt, on_no_match);
        }
    
        if self.mode_ == Mode::Latin1 {
           
        } else {
            //DCHECK(self.mode_ == Mode::UC16);
        }
    
        self.masm_.bind(&mut fallthrough);
    }

    pub fn check_not_back_reference(&mut self, start_reg: i32, read_backward: bool, on_no_match: &mut Label) {
        let mut fallthrough = Label::new();
        self.masm_.ldr(self.registers_[0], self.register_location(start_reg));
        self.masm_.ldr(self.registers_[1], self.register_location(start_reg + 1));
        self.masm_.sub(self.registers_[1], self.registers_[1], self.registers_[0], SetCC::SetCC);
    
        self.masm_.b(Condition::Eq, &mut fallthrough);
    
        if read_backward {
            self.masm_.ldr(self.registers_[3], MemOperand(self.registers_[11], self.string_start_minus_one_offset_));
            self.masm_.add(self.registers_[3], self.registers_[3], self.registers_[1]);
            self.masm_.cmp(self.registers_[6], self.registers_[3]);
            self.branch_or_backtrack(Condition::Le, on_no_match);
        } else {
            self.masm_.cmn(self.registers_[1], self.registers_[6]);
            self.branch_or_backtrack(Condition::Gt, on_no_match);
        }
    
        self.masm_.bind(&mut fallthrough);
    }

    pub fn check_not_character(&mut self, c: u32, on_not_equal: &mut Label) {
        self.masm_.cmp(self.registers_[7], c);
        self.branch_or_backtrack(Condition::Ne, on_not_equal);
    }

    pub fn check_character_after_and(&mut self, c: u32, mask: u32, on_equal: &mut Label) {
        if c == 0 {
            self.masm_.tst(self.registers_[7], mask);
        } else {
            self.masm_.and_(self.registers_[0], self.registers_[7], mask);
            self.masm_.cmp(self.registers_[0], c);
        }
        self.branch_or_backtrack(Condition::Eq, on_equal);
    }

    pub fn check_not_character_after_and(&mut self, c: u32, mask: u32, on_not_equal: &mut Label) {
        if c == 0 {
            self.masm_.tst(self.registers_[7], mask);
        } else {
            self.masm_.and_(self.registers_[0], self.registers_[7], mask);
            self.masm_.cmp(self.registers_[0], c);
        }
        self.branch_or_backtrack(Condition::Ne, on_not_equal);
    }

    pub fn check_not_character_after_minus_and(&mut self, c: u16, minus: u16, mask: u16, on_not_equal: &mut Label) {
        self.masm_.sub(self.registers_[0], self.registers_[7], minus);
        self.masm_.and_(self.registers_[0], self.registers_[0], mask);
        self.masm_.cmp(self.registers_[0], c);
        self.branch_or_backtrack(Condition::Ne, on_not_equal);
    }

    pub fn check_character_in_range(&mut self, from: u16, to: u16, on_in_range: &mut Label) {
        self.masm_.sub(self.registers_[0], self.registers_[7], from);
        self.masm_.cmp(self.registers_[0], to - from);
        self.branch_or_backtrack(Condition::Ls, on_in_range);
    }

    pub fn check_character_not_in_range(&mut self, from: u16, to: u16, on_not_in_range: &mut Label) {
        self.masm_.sub(self.registers_[0], self.registers_[7], from);
        self.masm_.cmp(self.registers_[0], to - from);
        self.branch_or_backtrack(Condition::Hi, on_not_in_range);
    }

    pub fn get_code(&mut self, source: usize, flags: usize) -> Result<usize, String> {
        Ok(0)
    }
    
    pub fn go_to(&mut self, to: &mut Label) {
        self.branch_or_backtrack(Condition::Al, to);
    }
    
    pub fn if_register_ge(&mut self, reg: i32, comparand: i32, if_ge: &mut Label) {
        self.masm_.ldr(self.registers_[0], self.register_location(reg));
        self.masm_.cmp(self.registers_[0], comparand);
        self.branch_or_backtrack(Condition::Ge, if_ge);
    }
    
    pub fn if_register_lt(&mut self, reg: i32, comparand: i32, if_lt: &mut Label) {
        self.masm_.ldr(self.registers_[0], self.register_location(reg));
        self.masm_.cmp(self.registers_[0], comparand);
        self.branch_or_backtrack(Condition::Lt, if_lt);
    }
    
    pub fn if_register_eq_pos(&mut self, reg: i32, if_eq: &mut Label) {
        self.masm_.ldr(self.registers_[0], self.register_location(reg));
        self.masm_.cmp(self.registers_[0], self.registers_[6]);
        self.branch_or_backtrack(Condition::Eq, if_eq);
    }
    
    pub fn implementation(&self) -> IrregexpImplementation {
        IrregexpImplementation::kARMImplementation
    }
    
    pub fn load_current_character_unchecked(&mut self, cp_offset: i32, character_count: i32) {

    }
    
    pub fn pop_current_position(&mut self) {
        self.pop(self.registers_[6]);
    }
    
    pub fn pop_register(&mut self, register_index: i32) {
        self.pop(self.registers_[0]);
        self.masm_.str(self.registers_[0], self.register_location(register_index));
    }
    
    pub fn push_backtrack(&mut self, label: &mut Label) {
        //self.masm_.mov_label_offset(self.registers_[0], label);
        self.push(self.registers_[0]);
        self.check_stack_limit();
    }
    
    pub fn push_current_position(&mut self) {
        self.push(self.registers_[6]);
        self.check_stack_limit();
    }
    
    pub fn push_register(&mut self, register_index: i32, check_stack_limit: StackCheckFlag) {
        self.masm_.ldr(self.registers_[0], self.register_location(register_index));
        self.push(self.registers_[0]);
        if check_stack_limit == StackCheckFlag::kCheckStackLimit {
            self.check_stack_limit();
        }
    }
    
    pub fn read_current_position_from_register(&mut self, reg: i32) {
        self.masm_.ldr(self.registers_[6], self.register_location(reg));
    }
    
    pub fn read_stack_pointer_from_register(&mut self, reg: i32) {
       
    }
    
    pub fn set_current_position_from_end(&mut self, by: i32) {
        let mut after_position = Label::new();
        self.masm_.cmp(self.registers_[6], -by * self.char_size());
        self.masm_.b(Condition::Ge, &mut after_position);
        self.masm_.mov(self.registers_[6], -by * self.char_size());
        self.load_current_character_unchecked(-1, 1);
        self.masm_.bind(&mut after_position);
    }
    
    pub fn set_register(&mut self, register_index: i32, to: i32) {
        self.masm_.mov(self.registers_[0], to);
        self.masm_.str(self.registers_[0], self.register_location(register_index));
    }
    
    pub fn succeed(&mut self) -> bool {
        self.masm_.jmp(&mut self.success_label_);
        self.global_
    }
    
    pub fn write_current_position_to_register(&mut self, reg: i32, cp_offset: i32) {
        if cp_offset == 0 {
            self.masm_.str(self.registers_[6], self.register_location(reg));
        } else {
            self.masm_.add(self.registers_[0], self.registers_[6], cp_offset * self.char_size());
            self.masm_.str(self.registers_[0], self.register_location(reg));
        }
    }
    
    pub fn clear_registers(&mut self, reg_from: i32, reg_to: i32) {
        self.masm_.ldr(self.registers_[0], MemOperand(self.registers_[11], self.string_start_minus_one_offset_));
        for reg in reg_from..=reg_to {
            self.masm_.str(self.registers_[0], self.register_location(reg));
        }
    }
    
    pub fn write_stack_pointer_to_register(&mut self, reg: i32) {

    }
    
    fn register_location(&self, register_index: i32) -> MemOperand {
        MemOperand(self.registers_[11], -1*k_register_zero_offset - register_index * 4)
    }

    fn char_size(&self) -> i32 {
        match self.mode_ {
            Mode::Latin1 => 1,
            Mode::UC16 => 2,
        }
    }

    fn branch_or_backtrack(&mut self, condition: Condition, to: &mut Label) {
        if condition == Condition::Al {
            if to.is_unused() {
                self.backtrack();
                return;
            }
            self.masm_.jmp(to);
            return;
        }

        if to.is_unused() {
            self.masm_.b(condition, &mut self.backtrack_label_);
            return;
        }

        self.masm_.b(condition, to);
    }

    fn push(&mut self, source: i32) {
        self.masm_.str(source, MemOperand(self.registers_[8], -4, MemOperandType::kPreIndex));
    }

    fn pop(&mut self, target: i32) {
        self.masm_.ldr(target, MemOperand(self.registers_[8], 4, MemOperandType::kPostIndex));
    }

    fn call_c_function_from_irregexp_code(&mut self, function: ExternalReference, num_arguments: i32) {
        self.masm_.call_c_function(function, num_arguments, SetIsolateDataSlots::kNo);
    }

    fn check_preemption(&mut self) {
       
    }

    fn check_stack_limit(&mut self) {
      
    }

    fn fail(&mut self) {
        self.masm_.mov(self.registers_[0], -1);
        self.masm_.jmp(&mut self.exit_label_);
    }

    fn has_backtrack_limit(&self) -> bool {
      self.has_backtrack_limit_
    }

    fn backtrack_limit(&self) -> i32 {
      self.backtrack_limit_
    }

    fn can_fallback(&self) -> bool {
        self.can_fallback_
    }

    fn global(&self) -> bool {
        self.global_
    }

    fn global_with_zero_length_check(&self) -> bool {
        self.global_with_zero_length_check_
    }

    fn global_unicode(&self) -> bool {
        self.global_unicode_
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Latin1,
    UC16,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IrregexpImplementation {
    kARMImplementation,
}

pub struct Label {
    is_linked: bool,
}

impl Label {
    pub fn new() -> Self {
        Label {
            is_linked: false,
        }
    }

    pub fn is_linked(&self) -> bool {
        self.is_linked
    }

    pub fn unuse(&mut self) {
        self.is_linked = false;
    }

    pub fn bind(&mut self) {
        self.is_linked = true;
    }
}

pub struct MacroAssembler {
    code_object: i32,
    isolate: *mut Isolate,
    options: AssemblerOptions
}

impl MacroAssembler {
    pub fn new(isolate: *mut Isolate, code_object_required: CodeObjectRequired, assembler_buffer: usize) -> MacroAssembler {
        MacroAssembler {
            code_object: 0,
            isolate,
            options: AssemblerOptions{}
        }
    }

    pub fn jmp(&mut self, target: &Label) {

    }

    pub fn bind(&mut self, label: &Label) {

    }

    pub fn ldr(&mut self, rd: i32, mem_operand: MemOperand) {

    }

    pub fn str(&mut self, rd: i32, mem_operand: MemOperand) {

    }

    pub fn add(&mut self, rd: i32, rn: i32, imm: i32) {

    }

    pub fn sub(&mut self, rd: i32, rn: i32, rm: i32, set_cc: SetCC) {

    }

    pub fn b(&mut self, condition: Condition, target: &Label) {

    }

    pub fn cmp(&mut self, rn: i32, operand: i32) {

    }

    pub fn mov(&mut self, rd: i32, operand: i32) {

    }

    pub fn tst(&mut self, rn: i32, mask: u32) {

    }

    pub fn and_(&mut self, rd: i32, rn: i32, mask: u32) {

    }

    pub fn orr(&mut self, rd: i32, rn: i32, imm: i32) {

    }

    pub fn eor(&mut self, rd: i32, rn: i32, imm: i32) {

    }

    pub fn ldrb(&mut self, rd: i32, mem_operand: MemOperand) {

    }

    pub fn ldrh(&mut self, rd: i32, mem_operand: MemOperand) {

    }

    pub fn bl(&mut self, target: &Label, cond: Condition) {

    }

    pub fn pop(&mut self, registers: i32) {

    }

    pub fn code_object(&self) -> i32 {
        self.code_object
    }

    pub fn aborted_code_generation(&mut self) {

    }
    pub fn call_c_function(&mut self, function: ExternalReference, num_arguments: i32, set_isolate_data_slots: SetIsolateDataSlots){

    }
    pub fn AllocateStackSpace(&mut self, arg: i32){

    }
    pub fn GetCode(&mut self, isolate: *mut Isolate, code_desc: &mut CodeDesc){

    }

    pub fn PrepareCallCFunction(&mut self, arg: i32){

    }

    pub fn mov_label_offset(&mut self, _r0: i32, _label: &Label){
    }

    pub fn options(&self) -> &AssemblerOptions {
        &self.options
    }
}

pub struct MemOperand {
    base: i32,
    offset: i32,
    operand_type: MemOperandType
}

impl MemOperand {
    pub fn new(base: i32, offset: i32, operand_type: MemOperandType) -> MemOperand {
        MemOperand {
            base,
            offset,
            operand_type
        }
    }
    pub fn new_from_register(base:i32, _offset: i32) -> Self {
        MemOperand{
            base: base,
            offset: 0,
            operand_type: MemOperandType::kOffset
        }
    }
    pub fn from_offset(_base: i32, _offset: i32) -> Self{
        MemOperand{
            base: 0,
            offset: 0,
            operand_type: MemOperandType::kOffset
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MemOperandType{
    kPreIndex,
    kPostIndex,
    kOffset
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Condition {
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    Al,
    Ls,
    Hi
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SetCC {
    SetCC,
    LeaveCC
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CodeObjectRequired {
    kYes,
    kNo
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StackCheckFlag {
    kCheckStackLimit,
    kNoCheckStackLimit
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LeaveCC{
    LeaveCC
}
impl LeaveCC {
    pub fn LeaveCC() -> Self {
        LeaveCC::LeaveCC
    }
}

pub struct AssemblerOptions{
    pub isolate_independent_code: bool
}
impl AssemblerOptions{
    pub fn new()-> Self{
        AssemblerOptions{
            isolate_independent_code: false
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SetIsolateDataSlots{
    kNo
}

impl SetIsolateDataSlots {
    pub fn kNo() -> Self {
        SetIsolateDataSlots::kNo
    }
}

static k_register_zero_offset: i32 = -36;
}
