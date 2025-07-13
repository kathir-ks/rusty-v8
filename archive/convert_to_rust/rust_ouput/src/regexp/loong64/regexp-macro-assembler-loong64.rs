// Converted from V8 C++ source files:
// Header: regexp-macro-assembler-loong64.h
// Implementation: regexp-macro-assembler-loong64.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::sync::{Arc, Mutex, RwLock};

use crate::ast::If;
use crate::logging::code_events::InstructionStream;
use crate::regexp::experimental::experimental_interpreter::{Address, Label, RegExpFlags, String};
use crate::regexp::experimental::experimental::DirectHandle;
use crate::regexp::regexp_parser::CharacterRange;
use crate::snapshot::deserializer::Local;
use crate::init::v8::Mode;
use crate::codegen::macro_assembler::NewAssemblerBuffer;
use crate::regexp::regexp_macro_assembler::NativeRegExpMacroAssembler;
use crate::codegen::macro_assembler::MacroAssembler;
use crate::objects::code_inl::CodeDesc;
use crate::heap::factory::Factory;
use crate::objects::code_inl::Code;
use crate::logging::log::RegExpCodeCreateEvent;
use crate::ast::ast::CallType;
use crate::init::v8::V8;
use crate::strings::uri::v8;
use crate::regexp::experimental::experimental_interpreter::RegExp;
use crate::regexp::regexp_parser::void;

pub struct RegExpMacroAssemblerLOONG64 {
    base: NativeRegExpMacroAssembler,
    masm_: Box<MacroAssembler>,
    no_root_array_scope_: Box<NoRootArrayScope>,
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

struct NoRootArrayScope {
    _dummy: i32,
}

impl RegExpMacroAssemblerLOONG64 {
    pub fn new(isolate: &mut Isolate, zone: &mut Zone, mode: Mode, registers_to_save: i32) -> RegExpMacroAssemblerLOONG64 {
        if registers_to_save % 2 != 0 {
            panic!("registers_to_save must be even");
        }

        let mut masm_ = MacroAssembler::new(
            isolate,
            CodeObjectRequired::kYes,
            NewAssemblerBuffer::new(1024),
        );
        let no_root_array_scope_ = NoRootArrayScope { _dummy: 0 };

        let mut result = RegExpMacroAssemblerLOONG64 {
            base: NativeRegExpMacroAssembler::new(isolate, zone),
            masm_: masm_,
            no_root_array_scope_: Box::new(no_root_array_scope_),
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

         result.masm_.jmp(&result.entry_label_);
        result.masm_.bind(&result.internal_failure_label_);
        result.masm_.li(Register::A0, FAILURE);
        result.masm_.Ret();
        result.masm_.bind(&result.start_label_);
        result
    }

    pub fn stack_limit_slack_slot_count(&self) -> i32 {
        RegExpStack::kStackLimitSlackSlotCount
    }

    pub fn AdvanceCurrentPosition(&mut self, by: i32) {
        if by != 0 {
            self.masm_.Add_d(Register::S2, Register::S2, by * self.char_size());
        }
    }

    pub fn AdvanceRegister(&mut self, reg: i32, by: i32) {
        if reg < 0 || reg >= self.num_registers_ {
            panic!("Register index out of bounds");
        }
        if by != 0 {
            self.masm_.Ld_d(Register::A0, self.register_location(reg));
            self.masm_.Add_d(Register::A0, Register::A0, by);
            self.masm_.St_d(Register::A0, self.register_location(reg));
        }
    }

    pub fn Backtrack(&mut self) {
        self.CheckPreemption();
        if self.base.has_backtrack_limit() {
            self.BacktrackWithLimit();
        } else {
            self.BacktrackWithoutLimit();
        }
    }

    fn BacktrackWithLimit(&mut self) {
        let mut next = Label::new();
        self.masm_.Ld_d(Register::A0, self.mem_operand(kBacktrackCountOffset));
        self.masm_.Add_d(Register::A0, Register::A0, 1);
        self.masm_.St_d(Register::A0, self.mem_operand(kBacktrackCountOffset));
        self.masm_.Branch(&mut next, Condition::ne, Register::A0, self.backtrack_limit());

        if self.base.can_fallback() {
             self.masm_.jmp(&self.fallback_label_);
        } else {
           self.Fail();
        }
        self.masm_.bind(&next);
        self.PopAndJump();

    }
    fn BacktrackWithoutLimit(&mut self) {
        self.PopAndJump();
    }
    fn PopAndJump(&mut self) {
        self.masm_.Pop(Register::A0);
        self.masm_.Add_d(Register::A0, Register::A0, self.code_pointer());
        self.masm_.Jump(Register::A0);
    }

    pub fn Bind(&mut self, label: &mut Label) {
        self.masm_.bind(label);
    }

    pub fn CheckAtStart(&mut self, cp_offset: i32, on_at_start: &mut Label) {
        self.masm_.Ld_d(Register::A1, self.mem_operand(kStringStartMinusOneOffset));
        self.masm_.Add_d(Register::A0, self.current_input_offset(), -self.char_size() + cp_offset * self.char_size());
        self.BranchOrBacktrack(on_at_start, Condition::eq, Register::A0, Register::A1);
    }

    pub fn CheckCharacter(&mut self, c: u32, on_equal: &mut Label) {
        self.BranchOrBacktrack(on_equal, Condition::eq, self.current_character(), c);
    }

    pub fn CheckCharacterAfterAnd(&mut self, c: u32, mask: u32, on_equal: &mut Label) {
        self.masm_.And(Register::A0, self.current_character(), mask);
        let rhs = if c == 0 { Register::ZERO } else { c };
        self.BranchOrBacktrack(on_equal, Condition::eq, Register::A0, rhs);
    }

    pub fn CheckCharacterGT(&mut self, limit: u16, on_greater: &mut Label) {
        self.BranchOrBacktrack(on_greater, Condition::gt, self.current_character(), limit);
    }

    pub fn CheckCharacterLT(&mut self, limit: u16, on_less: &mut Label) {
        self.BranchOrBacktrack(on_less, Condition::lt, self.current_character(), limit);
    }

    pub fn CheckGreedyLoop(&mut self, on_tos_equals_current_position: &mut Label) {
        let mut backtrack_non_equal = Label::new();
        self.masm_.Ld_w(Register::A0, MemOperand(self.backtrack_stackpointer(), 0));
        self.masm_.Branch(&mut backtrack_non_equal, Condition::ne, self.current_input_offset(), Register::A0);
        self.masm_.Add_d(self.backtrack_stackpointer(), self.backtrack_stackpointer(), crate::codegen::macro_assembler::kIntSize);
        self.masm_.bind(&mut backtrack_non_equal);
        self.BranchOrBacktrack(on_tos_equals_current_position, Condition::eq, self.current_input_offset(), Register::A0);
    }

    pub fn CheckNotAtStart(&mut self, cp_offset: i32, on_not_at_start: &mut Label) {
        self.masm_.Ld_d(Register::A1, self.mem_operand(kStringStartMinusOneOffset));
        self.masm_.Add_d(Register::A0, self.current_input_offset(), -self.char_size() + cp_offset * self.char_size());
        self.BranchOrBacktrack(on_not_at_start, Condition::ne, Register::A0, Register::A1);
    }

   pub fn CheckNotBackReference(&mut self, start_reg: i32, read_backward: bool, on_no_match: &mut Label) {
        let mut fallthrough = Label::new();

        self.masm_.Ld_d(Register::A0, self.register_location(start_reg));
        self.masm_.Ld_d(Register::A1, self.register_location(start_reg + 1));
        self.masm_.Sub_d(Register::A1, Register::A1, Register::A0);

        self.masm_.Branch(&mut fallthrough, Condition::eq, Register::A1, Register::ZERO);

        if read_backward {
            self.masm_.Ld_d(Register::T1, self.mem_operand(kStringStartMinusOneOffset));
            self.masm_.Add_d(Register::T1, Register::T1, Register::A1);
            self.BranchOrBacktrack(on_no_match, Condition::le, self.current_input_offset(), Register::T1);
        } else {
            self.masm_.Add_d(Register::T1, Register::A1, self.current_input_offset());
            self.BranchOrBacktrack(on_no_match, Condition::gt, Register::T1, Register::ZERO);
        }
        self.AddCodeForCheckNotBackReference(start_reg,read_backward,on_no_match,fallthrough);

    }
    fn AddCodeForCheckNotBackReference(&mut self,start_reg: i32,
                                            read_backward: bool,
                                            on_no_match: &mut Label,
                                            mut fallthrough:Label){
            self.NotCheck(read_backward,on_no_match,fallthrough,false);
    }

   pub fn CheckNotBackReferenceIgnoreCase(&mut self, start_reg: i32, read_backward: bool, unicode: bool, on_no_match: &mut Label) {
       let mut fallthrough = Label::new();
        self.masm_.Ld_d(Register::A0, self.register_location(start_reg));
        self.masm_.Ld_d(Register::A1, self.register_location(start_reg + 1));
        self.masm_.Sub_d(Register::A1, Register::A1, Register::A0);

        self.masm_.Branch(&mut fallthrough, Condition::eq, Register::A1, Register::ZERO);

        if read_backward {
            self.masm_.Ld_d(Register::T1, self.mem_operand(kStringStartMinusOneOffset));
            self.masm_.Add_d(Register::T1, Register::T1, Register::A1);
            self.BranchOrBacktrack(on_no_match, Condition::le, self.current_input_offset(), Register::T1);
        } else {
            self.masm_.Add_d(Register::T1, Register::A1, self.current_input_offset());
            self.BranchOrBacktrack(on_no_match, Condition::gt, Register::T1, Register::ZERO);
        }
        self.AddCodeForCheckNotBackReferenceIgnoreCase(start_reg,read_backward,unicode,on_no_match,fallthrough);
    }
     fn AddCodeForCheckNotBackReferenceIgnoreCase(&mut self,start_reg: i32,
                                                    read_backward: bool,
                                                    unicode: bool,
                                                    on_no_match: &mut Label,
                                                    mut fallthrough:Label){
            self.NotCheck(read_backward,on_no_match,fallthrough,true);
     }
    fn NotCheck(&mut self, read_backward: bool, on_no_match: &mut Label, mut fallthrough: Label, is_ignore_case: bool) {
         if self.mode_ == Mode::LATIN1 {
            self.NotCheckLatin(on_no_match,read_backward);
        } else {
            DCHECK(self.mode_ == Mode::UC16);
            self.NotCheckUC16(on_no_match,read_backward,is_ignore_case);
        }
         self.masm_.bind(&mut fallthrough);
    }
    fn NotCheckUC16(&mut self, on_no_match: &mut Label, read_backward: bool,unicode: bool){
        let argument_count = 4;
        self.masm_.PrepareCallCFunction(argument_count, Register::A2);

        self.masm_.Add_d(Register::A0, Register::A0, self.end_of_input_address());
        self.masm_.mov(Register::A2, Register::A1);
        self.masm_.mov(Register::S3, Register::A1);
        self.masm_.Add_d(Register::A1, self.current_input_offset(), self.end_of_input_address());
        if read_backward {
            self.masm_.Sub_d(Register::A1, Register::A1, Register::S3);
        }

        let isolate_address = ExternalReference::isolate_address(self.masm_.isolate());
        self.masm_.li(Register::A3, isolate_address);

        {
            let scope = AllowExternalCallThatCantCauseGC(self.masm_.as_mut());
             let function = if unicode{
                ExternalReference::re_case_insensitive_compare_unicode()
            } else {
                  ExternalReference::re_case_insensitive_compare_non_unicode()
            };
            self.CallCFunctionFromIrregexpCode(function, argument_count);
        }
         self.BranchOrBacktrack(on_no_match,Condition::eq,Register::A0,Register::ZERO);
         if read_backward {
           self.masm_.Sub_d(self.current_input_offset(),self.current_input_offset(),Register::S3);
         } else {
          self.masm_.Add_d(self.current_input_offset(),self.current_input_offset(),Register::S3);
        }
    }
    fn NotCheckLatin(&mut self, on_no_match: &mut Label, read_backward: bool){
       let mut success = Label::new();
        let mut fail = Label::new();
        let mut loop_check = Label::new();

        self.masm_.Add_d(Register::A0, Register::A0, self.end_of_input_address());
        self.masm_.Add_d(Register::A2, self.end_of_input_address(), self.current_input_offset());
         if read_backward{
           self.masm_.Sub_d(Register::A2,Register::A2,Register::A1);
        }

        self.masm_.Add_d(Register::A1, Register::A0, Register::A1);
        let mut loop_start = Label::new();
        self.masm_.bind(&mut loop_start);
        self.masm_.Ld_bu(Register::A3,MemOperand(Register::A0,0));
        self.masm_.addi_d(Register::A0,Register::A0,self.char_size());
        self.masm_.Ld_bu(Register::A4, MemOperand(Register::A2,0));
        self.masm_.addi_d(Register::A2,Register::A2,self.char_size());

        self.masm_.Branch(&mut loop_check,Condition::eq,Register::A4,Register::A3);
        self.masm_.Or(Register::A3,Register::A3,0x20);
        self.masm_.Or(Register::A4,Register::A4,0x20);
        self.masm_.Branch(&mut fail, Condition::ne, Register::A4, Register::A3);
        self.masm_.Sub_d(Register::A3,Register::A3,'a');
        self.masm_.Branch(&mut loop_check, Condition::ls,Register::A3, 'z'-'a');
        self.masm_.Sub_d(Register::A3, Register::A3, 224-'a');
        self.masm_.Branch(&mut fail, Condition::hi, Register::A3, 254-224);
        self.masm_.Branch(&mut fail, Condition::eq, Register::A3, 247-224);

        self.masm_.bind(&mut loop_check);
         self.masm_.Branch(&mut loop_start,Condition::lt,Register::A0,Register::A1);
         self.masm_.jmp(&mut success);
         self.masm_.bind(&mut fail);
          self.GoTo(on_no_match);

          self.masm_.bind(&mut success);
         self.masm_.Sub_d(self.current_input_offset(), Register::A2, self.end_of_input_address());
           if read_backward {
           self.masm_.Ld_d(Register::T1, self.register_location(start_reg));
            self.masm_.Ld_d(Register::A2, self.register_location(start_reg+1));

            self.masm_.Add_d(self.current_input_offset(),self.current_input_offset(),Register::T1);
             self.masm_.Sub_d(self.current_input_offset(),self.current_input_offset(),Register::A2);

           }

    }

    pub fn CheckNotCharacter(&mut self, c: u32, on_not_equal: &mut Label) {
        self.BranchOrBacktrack(on_not_equal, Condition::ne, self.current_character(), c);
    }

    pub fn CheckNotCharacterAfterAnd(&mut self, c: u32, mask: u32, on_not_equal: &mut Label) {
        self.masm_.And(Register::A0, self.current_character(), mask);
        let rhs = if c == 0 { Register::ZERO } else { c };
        self.BranchOrBacktrack(on_not_equal, Condition::ne, Register::A0, rhs);
    }

    pub fn CheckNotCharacterAfterMinusAnd(&mut self, c: u16, minus: u16, mask: u16, on_not_equal: &mut Label) {
        self.masm_.Sub_d(Register::A0, self.current_character(), minus);
        self.masm_.And(Register::A0, Register::A0, mask);
        self.BranchOrBacktrack(on_not_equal, Condition::ne, Register::A0, c);
    }

    pub fn CheckCharacterInRange(&mut self, from: u16, to: u16, on_in_range: &mut Label) {
        self.masm_.Sub_d(Register::A0, self.current_character(), from);
        self.BranchOrBacktrack(on_in_range, Condition::ls, Register::A0, to - from);
    }

    pub fn CheckCharacterNotInRange(&mut self, from: u16, to: u16, on_not_in_range: &mut Label) {
        self.masm_.Sub_d(Register::A0, self.current_character(), from);
        self.BranchOrBacktrack(on_not_in_range, Condition::hi, Register::A0, to - from);
    }

    pub fn CheckCharacterInRangeArray(&mut self, ranges: &ZoneList<CharacterRange>, on_in_range: &mut Label) -> bool {
        self.CallIsCharacterInRangeArray(ranges);
        self.BranchOrBacktrack(on_in_range, Condition::ne, Register::A0, Register::ZERO);
        true
    }

    pub fn CheckCharacterNotInRangeArray(&mut self, ranges: &ZoneList<CharacterRange>, on_not_in_range: &mut Label) -> bool {
        self.CallIsCharacterInRangeArray(ranges);
        self.BranchOrBacktrack(on_not_in_range, Condition::eq, Register::A0, Register::ZERO);
        true
    }

    pub fn CheckBitInTable(&mut self, table: Handle<ByteArray>, on_bit_set: &mut Label) {
        self.masm_.li(Register::A0, table);
         self.AddCodeForBitInTable(on_bit_set);
    }
     fn AddCodeForBitInTable(&mut self,on_bit_set: &mut Label){
        if self.mode_ != Mode::LATIN1 || crate::codegen::macro_assembler::kTableMask != String::kMaxOneByteCharCode {
            self.masm_.And(Register::A1, self.current_character(), crate::codegen::macro_assembler::kTableSize - 1);
            self.masm_.Add_d(Register::A0, Register::A0, Register::A1);
        } else {
            self.masm_.Add_d(Register::A0, Register::A0, self.current_character());
        }

        self.masm_.Ld_bu(Register::A0, FieldMemOperand(Register::A0, OFFSET_OF_DATA_START(ByteArray)));
         self.BranchOrBacktrack(on_bit_set, Condition::ne, Register::A0, Register::ZERO);
     }

    pub fn SkipUntilBitInTable(&mut self, cp_offset: i32, table: Handle<ByteArray>, nibble_table: Handle<ByteArray>, advance_by: i32) {
        let mut cont = Label::new();
        let mut again = Label::new();
        self.Bind(&mut again);
        self.LoadCurrentCharacter(cp_offset, &mut cont, true);
        self.AddCodeForBitInTable(&mut cont);
        self.AdvanceCurrentPosition(advance_by);
         self.GoTo(&mut again);
          self.Bind(&mut cont);
    }

    pub fn CheckPosition(&mut self, cp_offset: i32, on_outside_input: &mut Label) {
        if cp_offset >= 0 {
            self.BranchOrBacktrack(on_outside_input, Condition::ge, self.current_input_offset(), -cp_offset * self.char_size());
        } else {
            self.masm_.Ld_d(Register::A1, self.mem_operand(kStringStartMinusOneOffset));
            self.masm_.Add_d(Register::A0, self.current_input_offset(), cp_offset * self.char_size());
            self.BranchOrBacktrack(on_outside_input, Condition::le, Register::A0, Register::A1);
        }
    }

    pub fn CheckSpecialClassRanges(&mut self, type_: StandardCharacterSet, on_no_match: &mut Label) -> bool {
        match type_ {
            StandardCharacterSet::kWhitespace => {
                if self.mode_ == Mode::LATIN1 {
                   self.Matchwhitespace(on_no_match);
                    return true;
                }
                return false;
            }
            StandardCharacterSet::kNotWhitespace => {
                return false;
            }
             StandardCharacterSet::kDigit => {

                 self.SubCheckForLatin("0",'9',Condition::hi,on_no_match);
                  return true;
              }
             StandardCharacterSet::kNotDigit => {
              self.SubCheckForLatin("0",'9',Condition::ls,on_no_match);
                  return true;
              }
             StandardCharacterSet::kNotLineTerminator => {
                 self.SubCodeForNLT(on_no_match);
                 return true;
              }
              StandardCharacterSet::kLineTerminator => {
                self.SubCodeForLineTerminator(on_no_match);
                 return true;
              }
             StandardCharacterSet::kWord => {
                 self.WordRange(on_no_match);
                 return true;
              }
            StandardCharacterSet::kNotWord => {
                 self.NotWordRange(on_no_match);
                 return true;
              }
             StandardCharacterSet::kEverything => {
               return true;
               }
             _ => {panic!("Invalid SpecialClass");}
         }
    }
    fn Matchwhitespace(&mut self,on_no_match: &mut Label){
        let mut success = Label::new();
        self.masm_.Branch(&mut success, Condition::eq, self.current_character(), ' ');
        self.masm_.Sub_d(Register::A0, self.current_character(), '\t');
        self.masm_.Branch(&mut success, Condition::ls, Register::A0, '\r' - '\t');
       self.BranchOrBacktrack(on_no_match,Condition::ne,Register::A0, 0x00A0 - '\t');
       self.masm_.bind(&mut success);

    }
     fn SubCheckForLatin(&mut self,char: &str,char2: &str,condition: Condition,on_no_match: &mut Label){
           self.masm_.Sub_d(Register::A0, self.current_character(), char);
        self.BranchOrBacktrack(on_no_match, condition, Register::A0, char2-char);
    }
    fn SubCodeForLineTerminator(&mut self,on_no_match: &mut Label){
        self.masm_.Xor(Register::A0,self.current_character(),0x01);
         self.masm_.Sub_d(Register::A0,Register::A0,0x0B);

         if self.mode_ == Mode::LATIN1{
              self.BranchOrBacktrack(on_no_match,Condition::hi,Register::A0,0x0C-0x0B);
         } else {
             let mut done = Label::new();
            self.masm_.Branch(&mut done,Condition::ls,Register::A0,0x0C-0x0B);
            self.masm_.Sub_d(Register::A0,Register::A0,0x2028-0x0B);
             self.BranchOrBacktrack(on_no_match,Condition::hi,Register::A0,1);
             self.masm_.bind(&mut done);
         }

    }
     fn SubCodeForNLT(&mut self,on_no_match: &mut Label){
           self.masm_.Xor(Register::A0,self.current_character(),0x01);
         self.masm_.Sub_d(Register::A0,Register::A0,0x0B);
         self.BranchOrBacktrack(on_no_match,Condition::ls,Register::A0,0x0C-0x0B);
          if self.mode_ == Mode::UC16 {
            self.masm_.Sub_d(Register::A0,Register::A0,0x2028-0x0B);
             self.BranchOrBacktrack(on_no_match,Condition::ls,Register::A0,1);
          }

    }
     fn WordRange(&mut self,on_no_match: &mut Label){
          if self.mode_ != Mode::LATIN1{
             self.BranchOrBacktrack(on_no_match,Condition::hi,self.current_character(),'z');
         }
          let map = ExternalReference::re_word_character_map();
         self.masm_.li(Register::A0,map);
         self.masm_.Add_d(Register::A0, Register::A0, self.current_character());
         self.masm_.Ld_bu(Register::A0,MemOperand(Register::A0,0));
          self.BranchOrBacktrack(on_no_match, Condition::eq, Register::A0, Register::ZERO);
     }
      fn NotWordRange(&mut self,on_no_match: &mut Label){
         let mut done = Label::new();
         if self.mode_ != Mode::LATIN1{
             self.masm_.Branch(&mut done,Condition::hi,self.current_character(),'z');
         }
          let map = ExternalReference::re_word_character_map();
         self.masm_.li(Register::A0,map);
         self.masm_.Add_d(Register::A0, Register::A0, self.current_character());
         self.masm_.Ld_bu(Register::A0,MemOperand(Register::A0,0));
          self.BranchOrBacktrack(on_no_match, Condition::ne, Register::A0, Register::ZERO);
           if self.mode_ != Mode::LATIN1{
              self.masm_.bind(&mut done);
          }
     }

    pub fn Fail(&mut self) {
        self.masm_.li(Register::A0, FAILURE);
        self.masm_.jmp(&self.exit_label_);
    }
    fn CodeDescCode(code_desc: &mut CodeDesc,masm_: & MacroAssembler)-> Code{
          let isolate = masm_.isolate();

          let mut code = Factory::CodeBuilder(isolate, code_desc.clone(), CodeKind::REGEXP)
              .set_self_reference(masm_.CodeObject())
              .set_empty_source_position_table()
               .Build();
             code
        }
    pub fn GetCode(&mut self, source: DirectHandle<String>, flags: RegExpFlags) -> DirectHandle<HeapObject> {
        let mut return_v0 = Label::new();
        if false {
           //Todo
        }else {

            self.masm_.bind(&self.entry_label_);
            let mut scope = FrameScope::new(self.masm_.as_mut(), StackFrame::MANUAL);
            let registers_to_retain: RegList = RegList::new( vec![Register::S0, Register::S1, Register::S2, Register::S3, Register::S4, Register::S5, Register::S6, Register::S7] );
            self.masm_.MultiPush( vec![Register::RA],  vec![Register::FP],registers_to_retain);
            self.masm_.mov(Register::FP,Register::SP);

           let argument_registers:RegList = RegList::new(vec![Register::A0,Register::A1,Register::A2,Register::A3,Register::A4,Register::A5,Register::A6,Register::A7]);
           self.masm_.li(Register::KScratchReg, StackFrame::TypeToMarker(StackFrame::IRREGEXP));
           self.masm_.MultiPush(argument_registers.to_vec() , Register::KScratchReg.to_vec(),RegList::new(vec![]));
              self.masm_.mov(Register::A0,Register::ZERO);
            self.masm_.Push(Register::A0);
            self.masm_.Push(Register::A0);
            self.masm_.Push(Register::A0);
            self.masm_.Push(Register::A0);

             self.LoadRegExpStackPointerFromMemory(Register::S7);

            self.PushRegExpBasePointer(Register::S7,Register::A1);

               let mut stack_limit_hit = Label::new();
               let mut stack_ok = Label::new();

            let stack_limit = ExternalReference::address_of_jslimit(self.masm_.isolate());
            self.masm_.li(Register::A0,stack_limit);
            self.masm_.Ld_d(Register::A0,MemOperand(Register::A0,0));
             self.masm_.Sub_d(Register::A0, Register::SP, Register::A0);
        self.masm_.Branch(&mut stack_limit_hit, Condition::le, Register::A0,Register::ZERO);
        self.masm_.Branch(&mut stack_ok, Condition::hs, Register::A0, self.num_registers_* 8);
        self.masm_.li(Register::A0, EXCEPTION);
         self.masm_.jmp(&return_v0);

          self.masm_.bind(&mut stack_limit_hit);
              self.CallCheckStackGuardState(Register::A0, self.num_registers_);
             self.masm_.Branch(&mut return_v0,Condition::ne,Register::A0,Register::ZERO);

             self.masm_.bind(&mut stack_ok);
               self.masm_.Sub_d(Register::SP, Register::SP, self.num_registers_* 8);
                self.masm_.Ld_d(Register::S6,MemOperand(Register::FP, kInputEndOffset));
                self.masm_.Ld_d(Register::A0, MemOperand(Register::FP, kInputStartOffset));
                self.masm_.Sub_d(Register::S2,Register::A0, Register::S6);
                   self.masm_.Ld_d(Register::A1,MemOperand(Register
