// Converted from V8 C++ source files:
// Header: regexp-macro-assembler-mips64.h
// Implementation: regexp-macro-assembler-mips64.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2011 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::rc::Rc;
use std::sync::Mutex;

use crate::codegen::macro_assembler::NewAssemblerBuffer;
use crate::codegen::macro_assembler::MacroAssembler;
use crate::codegen::mips64::assembler_mips64_inl::Assembler;
use crate::heap::factory::Factory;
use crate::logging::log::RegExpCodeCreateEvent;
use crate::objects::code_inl::Code;
use crate::regexp::regexp_macro_assembler::NativeRegExpMacroAssembler;
use crate::regexp::regexp_stack::RegExpStack;
use crate::snapshot::embedded::embedded_data_inl::EmbeddedData;
use crate::strings::string_inl::String;
use crate::strings::string_hasher_inl::StringHasher;
use crate::strings::unicode_inl::StringIterator;
use crate::strings::uri::Mode;
use crate::init::v8::Isolate;
use crate::regexp::arm64::regexp_macro_assembler_arm64::IrregexpImplementation;
use crate::regexp::arm64::regexp_macro_assembler_arm64::StackCheckFlag;
use crate::regexp::arm64::regexp_macro_assembler_arm64::StandardCharacterSet;
use crate::regexp::regexp_parser::RegExpFlags;
use crate::regexp::regexp_macro_assembler::CodeDesc;
use crate::regexp::regexp_macro_assembler::MemOperand;
use crate::regexp::regexp_macro_assembler::Operand;
use crate::regexp::regexp_macro_assembler::Condition;
use crate::regexp::regexp_macro_assembler::CallOrigin;
use crate::regexp::regexp_macro_assembler::RelocInfo;
use crate::regexp::regexp_macro_assembler::FAILURE;
use crate::regexp::regexp_macro_assembler::EXCEPTION;
use crate::regexp::regexp_macro_assembler::FALLBACK_TO_EXPERIMENTAL;
use crate::regexp::regexp_macro_assembler::SUCCESS;
use crate::codegen::register::RegList;
use crate::codegen::register::Register;
use crate::codegen::register::kIntSize;
use crate::codegen::register::kPointerSize;
use crate::codegen::register::kCArgsSlotsSize;
use crate::codegen::register::kTableSize;
use crate::codegen::register::zero_reg;
use crate::codegen::register::kScratchReg;
use crate::codegen::register::t1;
use crate::codegen::register::a0;
use crate::codegen::register::a1;
use crate::codegen::register::a2;
use crate::codegen::register::a3;
use crate::codegen::register::a4;
use crate::codegen::register::a5;
use crate::codegen::register::a6;
use crate::codegen::register::a7;
use crate::codegen::register::s3;
use crate::codegen::register::t9;
use crate::codegen::register::v0;
use crate::codegen::register::sp;
use crate::codegen::register::fp;
use crate::codegen::register::ra;

use crate::zone::zone::Zone;
use crate::objects::heap_object::HeapObject;
use crate::handles::handles::DirectHandle;
use crate::handles::handles::Handle;
use crate::objects::byte_array::ByteArray;
use crate::objects::abstract_code::AbstractCode;
use crate::objects::code::CodeKind;
use crate::roots::roots::Root;
use crate::codegen::macro_assembler::NoRootArrayScope;
use crate::flags::flags::v8_flags;
use crate::codegen::register::IsAligned;
use crate::base::bits;
use crate::codegen::frame_scope::FrameScope;
use crate::codegen::frame_scope::StackFrame;
use crate::init::builtins::Builtins;
use crate::regexp::regexp_macro_assembler::CharacterRange;
use crate::handles::indirect_handles::IndirectHandle;
use crate::codegen::assembler::AssemblerBase;
use crate::codegen::label::Label;
use crate::codegen::code_comments::ASM_CODE_COMMENT_STRING;
use crate::strings::string_stream::StringStream;

pub struct RegExpMacroAssemblerMIPS {
    isolate: *mut Isolate, // Raw pointer to Isolate
    zone: *mut Zone,       // Raw pointer to Zone
    masm_: Box<MacroAssembler>,
    no_root_array_scope_: NoRootArrayScope,
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
    internal_failure_label_: Label,
    fallback_label_: Label,
}

impl RegExpMacroAssemblerMIPS {
    pub fn new(isolate: *mut Isolate, zone: *mut Zone, mode: Mode, registers_to_save: i32) -> Self {
        let mut masm_ = Box::new(MacroAssembler::new(
            isolate,
            true,
            NewAssemblerBuffer::new(1024),
        ));

        let mut result = Self {
            isolate: isolate,
            zone: zone,
            masm_: masm_,
            no_root_array_scope_: NoRootArrayScope::new(),
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
            internal_failure_label_: Label::new(),
            fallback_label_: Label::new(),
        };
        // let masm_ref = &mut result.masm_;
        // let mut assembler = Assembler::new(masm_ref);
        result.masm_.jmp(&mut result.entry_label_);
        result.masm_.bind(&mut result.internal_failure_label_);
        result.masm_.li(v0, Operand::new(FAILURE));
        result.masm_.Ret();
        result.masm_.bind(&mut result.start_label_);
        result
    }

    fn isolate(&self) -> *mut Isolate {
        self.masm_.isolate()
    }
    fn has_exception(&self) -> bool{
        self.masm_.has_exception()
    }
    fn bind_to(&mut self, label: &mut Label, position: i32){
        self.masm_.bind_to(label, position)
    }
    fn jmp(&mut self, label: &mut Label){
        self.masm_.jmp(label)
    }
    fn li(&mut self, dst: Register, value: Operand){
        self.masm_.li(dst, value)
    }
    fn Ret(&mut self){
        self.masm_.Ret()
    }
    fn bind(&mut self, label: &mut Label){
        self.masm_.bind(label)
    }
    fn Daddu(&mut self, rd: Register, rs: Register, rt: Operand){
        self.masm_.Daddu(rd, rs, rt)
    }
    fn Branch(&mut self, label: &mut Label, cond: Condition, rs: Register, rt: Operand){
        self.masm_.Branch(label, cond, rs, rt)
    }
    fn MultiPush(&mut self, reg_list: RegList){
        self.masm_.MultiPush(reg_list)
    }
    fn mov(&mut self, rd: Register, rs: Register){
        self.masm_.mov(rd, rs)
    }
    fn li_const_size(&mut self, rd: Register, value: Operand, _constant_size: i32){
        self.masm_.li(rd, value)
    }
    fn push(&mut self, reg: Register){
        self.masm_.push(reg)
    }
    fn Ld(&mut self, rd: Register, mem: MemOperand){
        self.masm_.Ld(rd, mem)
    }
    fn Dsubu(&mut self, rd: Register, rs: Register, rt: Operand){
        self.masm_.Dsubu(rd, rs, rt)
    }
    fn dsll(&mut self, rd: Register, rs: Register, shift: i32){
        self.masm_.dsll(rd, rs, shift)
    }
    fn Sd(&mut self, rd: Register, mem: MemOperand){
        self.masm_.Sd(rd, mem)
    }
    fn sw(&mut self, rd: Register, mem: MemOperand){
        self.masm_.Sw(rd, mem)
    }
    fn Lwu(&mut self, rd: Register, mem: MemOperand){
        self.masm_.Lwu(rd, mem)
    }
    fn Jump(&mut self, target: Register){
        self.masm_.Jump(target)
    }
    fn and(&mut self, rd: Register, rs: Register, imm: i32){
        self.masm_.and(rd, rs, imm)
    }
    fn lbu(&mut self, rd: Register, mem: MemOperand){
        self.masm_.lbu(rd, mem)
    }
    fn or(&mut self, rd: Register, rs: Register, imm: i32){
        self.masm_.or(rd, rs, imm)
    }
    fn dsubu(&mut self, rd: Register, rs: Register, rt: Register){
        self.masm_.Dsubu(rd, rs, Operand::new(rt))
    }
    fn BranchAndLink(&mut self, label: &mut Label, cond: Condition, rs: Register, rt: Operand){
        self.masm_.BranchAndLink(label, cond, rs, rt)
    }
    fn PrepareCallCFunction(&mut self, num_arguments: i32, temp_reg: Register){
        self.masm_.PrepareCallCFunction(num_arguments, temp_reg)
    }
    fn daddiu(&mut self, rd: Register, rs: Register, imm: i32){
        self.masm_.daddiu(rd, rs, imm)
    }
    fn lhu(&mut self, rd: Register, mem: MemOperand){
        self.masm_.lhu(rd, mem)
    }
    fn dsra(&mut self, rd: Register, rs: Register, shift: i32){
        self.masm_.dsra(rd, rs, shift)
    }
    fn GetCode(&mut self, isolate: *mut Isolate, code_desc: &mut CodeDesc){
        self.masm_.GetCode(isolate, code_desc)
    }

    fn get_code_object(&self) -> Option<Root> {
        Some(Root{})
    }

    fn has_backtrack_limit(&self) -> bool{
        true
    }

    fn backtrack_limit(&self) -> i32{
        50000
    }

    fn can_fallback(&self) -> bool{
        true
    }

    fn global(&self) -> bool{
        true
    }

    fn global_with_zero_length_check(&self) -> bool{
        true
    }

    fn global_unicode(&self) -> bool{
        true
    }

    fn check_not_in_surrogate_pair(_offset: i32, _on_not_surrogate: &mut Label){

    }
    fn emit(&mut self, _value: i32){}
    fn label_at_put(&mut self, _label: &mut Label, _offset: i32){}

    pub fn stack_limit_slack_slot_count(&self) -> i32 {
        RegExpStack::kStackLimitSlackSlotCount
    }

    pub fn advance_current_position(&mut self, by: i32) {
        if by != 0 {
            self.Daddu(RegExpMacroAssemblerMIPS::current_input_offset(), RegExpMacroAssemblerMIPS::current_input_offset(), Operand::new(by * self.char_size()));
        }
    }

    pub fn advance_register(&mut self, reg: i32, by: i32) {
        assert!(reg >= 0);
        assert!(self.num_registers_ > reg);
        if by != 0 {
            self.Ld(a0, self.register_location(reg));
            self.Daddu(a0, a0, Operand::new(by));
            self.Sd(a0, self.register_location(reg));
        }
    }

    pub fn backtrack(&mut self) {
        self.CheckPreemption();
        if self.has_backtrack_limit() {
            let mut next = Label::new();
            self.Ld(a0, MemOperand::new(fp, Self::kBacktrackCountOffset));
            self.Daddu(a0, a0, Operand::new(1));
            self.Sd(a0, MemOperand::new(fp, Self::kBacktrackCountOffset));
            self.Branch(&mut next, Condition::ne, a0, Operand::new(self.backtrack_limit()));

            if self.can_fallback() {
                self.jmp(&mut self.fallback_label_);
            } else {
                self.Fail();
            }

            self.bind(&mut next);
        }

        self.Pop(a0);
        self.Daddu(a0, a0, self.masm_.CodeObject());
        self.Jump(a0);
    }

    pub fn bind_label(&mut self, label: &mut Label) {
        self.bind(label);
    }

    pub fn check_character(&mut self, c: u32, on_equal: &mut Label) {
        self.BranchOrBacktrack(on_equal, Condition::eq, Self::current_character(), Operand::new(c as i32));
    }

    pub fn check_character_gt(&mut self, limit: u16, on_greater: &mut Label) {
        self.BranchOrBacktrack(on_greater, Condition::gt, Self::current_character(), Operand::new(limit as i32));
    }

    pub fn check_at_start(&mut self, cp_offset: i32, on_at_start: &mut Label) {
        self.Ld(a1, MemOperand::new(fp, Self::kStringStartMinusOneOffset));
        self.Daddu(a0, Self::current_input_offset(), Operand::new(-self.char_size() + cp_offset * self.char_size()));
        self.BranchOrBacktrack(on_at_start, Condition::eq, a0, Operand::new(a1));
    }

    pub fn check_not_at_start(&mut self, cp_offset: i32, on_not_at_start: &mut Label) {
        self.Ld(a1, MemOperand::new(fp, Self::kStringStartMinusOneOffset));
        self.Daddu(a0, Self::current_input_offset(), Operand::new(-self.char_size() + cp_offset * self.char_size()));
        self.BranchOrBacktrack(on_not_at_start, Condition::ne, a0, Operand::new(a1));
    }

    pub fn check_character_lt(&mut self, limit: u16, on_less: &mut Label) {
        self.BranchOrBacktrack(on_less, Condition::lt, Self::current_character(), Operand::new(limit as i32));
    }

    pub fn check_greedy_loop(&mut self, on_equal: &mut Label) {
        let mut backtrack_non_equal = Label::new();
        self.Lw(a0, MemOperand::new(Self::backtrack_stackpointer(), 0));
        self.Branch(&mut backtrack_non_equal, Condition::ne, Self::current_input_offset(), Operand::new(a0));
        self.Daddu(Self::backtrack_stackpointer(), Self::backtrack_stackpointer(), Operand::new(kIntSize));
        self.bind(&mut backtrack_non_equal);
        self.BranchOrBacktrack(on_equal, Condition::eq, Self::current_input_offset(), Operand::new(a0));
    }

    pub fn check_not_back_reference_ignore_case(&mut self, _start_reg: i32, _read_backward: bool, _unicode: bool, _on_no_match: &mut Label){

    }

    pub fn check_not_back_reference(&mut self, start_reg: i32, read_backward: bool, on_no_match: &mut Label) {
        let mut fallthrough = Label::new();

        // Find length of back-referenced capture.
        self.Ld(a0, self.register_location(start_reg));
        self.Ld(a1, self.register_location(start_reg + 1));
        self.Dsubu(a1, a1, Operand::new(a0)); // Length to check.

        // At this point, the capture registers are either both set or both cleared.
        // If the capture length is zero, then the capture is either empty or cleared.
        // Fall through in both cases.
        self.Branch(&mut fallthrough, Condition::eq, a1, Operand::new(zero_reg));

        if read_backward {
            self.Ld(t1, MemOperand::new(fp, Self::kStringStartMinusOneOffset));
            self.Daddu(t1, t1, Operand::new(a1));
            self.BranchOrBacktrack(on_no_match, Condition::le, Self::current_input_offset(), Operand::new(t1));
        } else {
            self.Daddu(t1, a1, Operand::new(Self::current_input_offset()));
            // Check that there are enough characters left in the input.
            self.BranchOrBacktrack(on_no_match, Condition::gt, t1, Operand::new(zero_reg));
        }

        // Compute pointers to match string and capture string.
        self.Daddu(a0, a0, Operand::new(self.end_of_input_address()));
        self.Daddu(a2, self.end_of_input_address(), Operand::new(Self::current_input_offset()));
        if read_backward {
            self.Dsubu(a2, a2, Operand::new(a1));
        }
        self.Daddu(a1, a1, Operand::new(a0));

        let mut loop_label = Label::new();
        self.bind(&mut loop_label);
        if self.mode_ == Mode::LATIN1 {
            self.lbu(a3, MemOperand::new(a0, 0));
            self.daddiu(a0, a0, self.char_size());
            self.lbu(a4, MemOperand::new(a2, 0));
            self.daddiu(a2, a2, self.char_size());
        } else {
            assert!(self.mode_ == Mode::UC16);
            self.lhu(a3, MemOperand::new(a0, 0));
            self.daddiu(a0, a0, self.char_size());
            self.lhu(a4, MemOperand::new(a2, 0));
            self.daddiu(a2, a2, self.char_size());
        }
        self.BranchOrBacktrack(on_no_match, Condition::ne, a3, Operand::new(a4));
        self.Branch(&mut loop_label, Condition::lt, a0, Operand::new(a1));

        // Move current character position to position after match.
        self.Dsubu(Self::current_input_offset(), a2, self.end_of_input_address());
        if read_backward {
            self.Ld(t1, self.register_location(start_reg)); // Index of start of capture.
            self.Ld(a2, self.register_location(start_reg + 1)); // Index of end of capture.
            self.Daddu(Self::current_input_offset(), Self::current_input_offset(), Operand::new(t1));
            self.Dsubu(Self::current_input_offset(), Self::current_input_offset(), Operand::new(a2));
        }
        self.bind(&mut fallthrough);
    }

    pub fn check_not_character(&mut self, c: u32, on_not_equal: &mut Label) {
        self.BranchOrBacktrack(on_not_equal, Condition::ne, Self::current_character(), Operand::new(c as i32));
    }

    pub fn check_character_after_and(&mut self, c: u32, mask: u32, on_equal: &mut Label) {
        self.and(a0, Self::current_character(), mask as i32);
        let rhs = if c == 0 { Operand::new(zero_reg) } else { Operand::new(c as i32) };
        self.BranchOrBacktrack(on_equal, Condition::eq, a0, rhs);
    }

    pub fn check_not_character_after_and(&mut self, c: u32, mask: u32, on_not_equal: &mut Label) {
        self.and(a0, Self::current_character(), mask as i32);
        let rhs = if c == 0 { Operand::new(zero_reg) } else { Operand::new(c as i32) };
        self.BranchOrBacktrack(on_not_equal, Condition::ne, a0, rhs);
    }

    pub fn check_not_character_after_minus_and(&mut self, c: u16, minus: u16, mask: u16, on_not_equal: &mut Label) {
        assert!(String::kMaxUtf16CodeUnit > minus);
        self.Dsubu(a0, Self::current_character(), Operand::new(minus as i32));
        self.and(a0, a0, mask as i32);
        self.BranchOrBacktrack(on_not_equal, Condition::ne, a0, Operand::new(c as i32));
    }

    pub fn check_character_in_range(&mut self, from: u16, to: u16, on_in_range: &mut Label) {
        self.Dsubu(a0, Self::current_character(), Operand::new(from as i32));
        self.BranchOrBacktrack(on_in_range, Condition::ls, a0, Operand::new((to - from) as i32));
    }

    pub fn check_character_not_in_range(&mut self, from: u16, to: u16, on_not_in_range: &mut Label) {
        self.Dsubu(a0, Self::current_character(), Operand::new(from as i32));
        self.BranchOrBacktrack(on_not_in_range, Condition::hi, a0, Operand::new((to - from) as i32));
    }

    pub fn call_is_character_in_range_array(&mut self, ranges: &ZoneList<CharacterRange>){
        static const int kNumArguments = 3;
        self.PrepareCallCFunction(kNumArguments, a0);

        self.mov(a0, Self::current_character());
        self.li(a1, Operand::new(123));
        self.li(a2, Operand::new(123));
        //TODO(Error)
        //self.li(a2, Operand::new(ExternalReference::isolate_address(self.isolate())));

        {
            let mut scope = FrameScope::new(&mut self.masm_, StackFrame::MANUAL);
            //TODO(Error)
           //self.CallCFunctionFromIrregexpCode(ExternalReference::ReIsCharacterInRangeArray(), kNumArguments);
        }

        self.li(Self::code_pointer(), Operand::new(123));
    }

    pub fn check_character_in_range_array(&mut self, ranges: &ZoneList<CharacterRange>, on_in_range: &mut Label) -> bool {
        self.call_is_character_in_range_array(ranges);
        self.BranchOrBacktrack(on_in_range, Condition::ne, v0, Operand::new(zero_reg));
        return true;
    }

    pub fn check_character_not_in_range_array(&mut self, ranges: &ZoneList<CharacterRange>, on_not_in_range: &mut Label) -> bool {
        self.call_is_character_in_range_array(ranges);
        self.BranchOrBacktrack(on_not_in_range, Condition::eq, v0, Operand::new(zero_reg));
        return true;
    }

    pub fn check_bit_in_table(&mut self, table: Handle<ByteArray>, on_bit_set: &mut Label) {
        self.li(a0, Operand::new(123));

        if self.mode_ != Mode::LATIN1 || kTableSize != String::kMaxOneByteCharCode {
            self.and(a1, Self::current_character(), (kTableSize - 1) as i32);
            self.Daddu(a0, a0, a1);
        } else {
            self.Daddu(a0, a0, Self::current_character());
        }

        self.lbu(a0, MemOperand::new(a0, 123));
        self.BranchOrBacktrack(on_bit_set, Condition::ne, a0, Operand::new(zero_reg));
    }

    pub fn skip_until_bit_in_table(&mut self, _cp_offset: i32, _table: Handle<ByteArray>, _nibble_table: Handle<ByteArray>, _advance_by: i32){

    }

    pub fn check_special_class_ranges(&mut self, type_: StandardCharacterSet, on_no_match: &mut Label) -> bool {
       false
    }

    pub fn fail(&mut self) {
        self.li(v0, Operand::new(FAILURE));
        self.jmp(&mut self.exit_label_);
    }

    pub fn load_regexp_stack_pointer_from_memory(&mut self, dst: Register) {
        let ref_ = ExternalReference::address_of_regexp_stack_stack_pointer(self.isolate());
        self.li(dst, Operand::new(123));
        self.Ld(dst, MemOperand::new(dst,123));
    }

    pub fn store_regexp_stack_pointer_to_memory(&mut self, src: Register, scratch: Register) {
        let ref_ = ExternalReference::address_of_regexp_stack_stack_pointer(self.isolate());
        self.li(scratch, Operand::new(123));
        self.Sd(src, MemOperand::new(scratch, 123));
    }

    pub fn push_regexp_base_pointer(&mut self, stack_pointer: Register, scratch: Register) {
        let ref_ = ExternalReference::address_of_regexp_stack_memory_top_address(self.isolate());
        self.li(scratch, Operand::new(ref_ as i32));
        self.Ld(scratch, MemOperand::new(scratch, 123));
        self.Dsubu(scratch, stack_pointer, Operand::new(scratch));
        self.Sd(scratch, MemOperand::new(fp, Self::kRegExpStackBasePointerOffset));
    }

    pub fn pop_regexp_base_pointer(&mut self, stack_pointer_out: Register, scratch: Register) {
        let ref_ = ExternalReference::address_of_regexp_stack_memory_top_address(self.isolate());
        self.Ld(stack_pointer_out, MemOperand::new(fp, Self::kRegExpStackBasePointerOffset));
        self.li(scratch, Operand::new(123));
        self.Ld(scratch, MemOperand::new(scratch, 123));
        self.Daddu(stack_pointer_out, stack_pointer_out, Operand::new(scratch));
        self.store_regexp_stack_pointer_to_memory(stack_pointer_out, scratch);
    }

    pub fn get_code(&mut self, source: DirectHandle<String>, flags: RegExpFlags) -> DirectHandle<HeapObject> {
        let mut return_v0 = Label::new();

        if self.has_exception() {
            self.bind_to(&mut self.entry_label_, self.internal_failure_label_.pos());
        } else {
            self.bind(&mut self.entry_label_);
            let mut scope = FrameScope::new(&mut self.masm_, StackFrame::MANUAL);
            let registers_to_retain = RegList::new().with(s0).with(s1).with(s2).with(s3).with(s4).with(s5).with(s6).with(s7).with(fp);
            self.MultiPush(registers_to_retain.with(ra));
            self.mov(fp, sp);
            let argument_registers = RegList::new().with(a0).with(a1).with(a2).with(a3);
            self.li(kScratchReg, Operand::new(123));
            self.MultiPush(argument_registers.with(kScratchReg));
            self.mov(a0, zero_reg);
            self.push(a0);
            self.push(a0);
            self.push(a0);
            self.push(a0);

            self.load_regexp_stack_pointer_from_memory(Self::backtrack_stackpointer());
            self.PushRegExpBasePointer(Self::backtrack_stackpointer(), a1);
            self.jmp(&mut self.start_label_);
            self.bind(&mut self.exit_label_);
            if self.global() {
                self.Ld(v0, MemOperand::new(fp, Self::kSuccessfulCapturesOffset));
            }

            self.bind(&mut return_v0);
            self.PopRegExpBasePointer(Self::backtrack_stackpointer(), a1);

            self.mov(sp, fp);
            self.MultiPop(registers_to_retain.with(ra));
            self.Ret();
            if self.backtrack_label_.is_linked(){
                self.bind(&mut self.backtrack_label_);
                self.backtrack();
            }
            let mut exit_with_exception = Label::new();
            if self.check_preempt_label_.is_linked(){
                let _scope_ = self;
                self.bind(&mut self.check_preempt_label_);
            }
            if self.stack_overflow_label_.is_linked(){
                let _scope_ = self;
                self.bind(&mut self.stack_overflow_label_);
            }
             if self.fallback_label_.is_linked(){
                let _scope_ = self;
                self.bind(&mut self.fallback_label_);
                self.li(v0, Operand::new(FALLBACK_TO_EXPERIMENTAL));
                self.jmp(&mut return_v0);
            }
        }

        let mut code_desc = CodeDesc::new();
        self.GetCode(self.isolate(), &mut code_desc);
        let code = Factory::CodeBuilder(self.isolate(), code_desc, CodeKind::REGEXP)
            .set_self_reference(self.masm_.CodeObject())
            .set_empty_source_position_table()
            .Build();

        todo!()
    }

    pub fn go_to(&mut self, to: &mut Label) {
        if to == None {
            self.backtrack();
            return;
        }
        self.jmp(to);
    }

    pub fn if_register_ge(&mut self, reg: i32, comparand: i32, if_ge: &mut Label) {
        self.Ld(a0, self.register_location(reg));
        self.BranchOrBacktrack(if_ge, Condition::ge, a0, Operand::new(comparand));
    }

    pub fn if_register_lt(&mut self, reg: i32, comparand: i32, if_lt: &mut Label) {
        self.Ld(a0, self.register_location(reg));
        self.BranchOrBacktrack(if_lt, Condition::lt, a0, Operand::new(comparand));
    }

    pub fn if_register_eq_pos(&mut self, reg: i32, if_eq: &mut Label) {
        self.Ld(a0, self.register_location(reg));
        self.BranchOrBacktrack(if_eq, Condition::eq, a0, Operand::new(Self::current_input_offset()));
    }

    pub fn implementation(&self) -> IrregexpImplementation {
        IrregexpImplementation::IrregexpImplementation
    }

    pub fn load_current_character_unchecked(&mut self, cp_offset: i32, character_count: i32) {
        let mut offset = Self::current_input_offset();
        if cp_offset != 0 {
            self.Daddu(t1, Self::current_input_offset(), Operand::new(cp_offset * self.char_size()));
            offset = t1;
        }
        assert_eq!(1, character_count);
        self.Daddu(
