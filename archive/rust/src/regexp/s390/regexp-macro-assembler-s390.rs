// Copyright 2015 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_macros)]
//#[cfg(target_arch = "s390x")]

//use crate::init::v8::V8; // Assuming v8.h functionality is in v8.rs
//use crate::codegen::macro_assembler::MacroAssembler;
//use crate::codegen::s390::assembler_s390_inl::{ACCESS_MASM, is_int8}; // Assuming assembler_s390_inl.h functionality is in assembler_s390_inl.rs
//use crate::heap::factory::Factory;
//use crate::logging::log::Log;
//use crate::objects::code_inl::Code;
//use crate::regexp::regexp_stack::RegExpStack;
//use crate::regexp::s390::regexp_macro_assembler_s390::{FAILURE, SUCCESS, EXCEPTION, FALLBACK_TO_EXPERIMENTAL, RegExpMacroAssemblerS390, NativeRegExpMacroAssembler};
//use crate::snapshot::embedded::embedded_data_inl::EmbeddedData;

//use v8::base::uc16;
//use v8::internal::wasm::asm::Assembler;

const V8_TARGET_ARCH_S390X: bool = true; // Replace with actual feature detection

#[cfg(feature = "s390x")]
mod regexp_macro_assembler_s390_mod {
    use std::rc::Rc;
    use std::cell::RefCell;
    use crate::regexp::regexp_macro_assembler::RegExpMacroAssembler;
    //use crate::init::v8::Isolate;
    //use crate::codegen::macro_assembler::MacroAssembler;
    //use crate::codegen::s390::assembler_s390_inl::{ACCESS_MASM, is_int8};
    //use crate::objects::code_inl::Code;
    //use crate::regexp::regexp_stack::RegExpStack;
    //use crate::snapshot::embedded::embedded_data_inl::EmbeddedData;

    //use v8::base::uc16;
    //use v8::internal::wasm::asm::Assembler;
    //use std::convert::TryInto;

    // Dummy definitions to allow compilation
    pub struct Isolate {}
    impl Isolate {
        pub fn new() -> Isolate { Isolate {} }
        pub fn IsGeneratingEmbeddedBuiltins(&self) -> bool {false}
    }
    pub struct Zone {}
    impl Zone {
        pub fn new() -> Zone { Zone {} }
    }
    pub struct MacroAssembler {}
    impl MacroAssembler {
        pub fn new() -> MacroAssembler { MacroAssembler {} }
        pub fn GetCode(&self, _isolate : &Isolate, _code_desc : &CodeDesc) {}
        pub fn CodeObject(&self) -> u64 {0}
        pub fn isolate(&self) -> &Isolate {&Isolate::new()}
        pub fn options(&self) -> AssemblerOptions { AssemblerOptions{} }
        //pub fn options(&self) -> AssemblerOptions { AssemblerOptions{} }
    }
    pub struct CodeDesc {}
    pub struct AssemblerBuffer {}
    impl AssemblerBuffer {
        pub fn new(_size: usize) -> AssemblerBuffer {AssemblerBuffer{}}
    }

    pub struct AssemblerOptions {}

    pub struct ByteArray {}
    pub struct Handle<T> {}
    impl<T> Handle<T> {
        pub fn new() -> Self {Self{}}
    }
    pub struct Code {}
    pub struct RegExpFlags {}
    pub struct String {}
    // Constants
    pub const FAILURE: i32 = -1;
    pub const SUCCESS: i32 = 1;
    pub const EXCEPTION: i32 = -2;
    pub const FALLBACK_TO_EXPERIMENTAL: i32 = -3;

    macro_rules! ACCESS_MASM {
        ($masm:expr) => {
           $masm
        };
    }

    macro_rules! is_int8 {
        ($x:expr) => {
            ($x >= -128) && ($x <= 127)
        };
    }

    pub struct RegExpMacroAssemblerS390<'a> {
        isolate: &'a Isolate,
        zone: &'a Zone,
        masm_: Rc<RefCell<MacroAssembler>>,
        no_root_array_scope_: Rc<RefCell<MacroAssembler>>, // Assuming NoRootArrayScope is a wrapper around MacroAssembler
        mode_: Mode,
        num_registers_: i32,
        num_saved_registers_: i32,
        entry_label_: Label,
        start_label_: Label,
        success_label_: Label,
        backtrack_label_: Label,
        exit_label_: Label,
        internal_failure_label_: Label,
        fallback_label_: Label,
    }

    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum Mode {
        LATIN1,
        UC16,
    }

    pub enum StandardCharacterSet {
        kWhitespace,
        kNotWhitespace,
        kDigit,
        kNotDigit,
        kNotLineTerminator,
        kLineTerminator,
        kWord,
        kNotWord,
        kEverything,
    }

    #[derive(Default)]
    pub struct Label {
        pos: Option<usize>,
        is_bound: bool,
    }

    impl Label {
        pub fn new() -> Self {
            Label { pos: None, is_bound: false }
        }

        pub fn bind(&mut self) {
            self.is_bound = true;
        }

        pub fn pos(&self) -> usize {
            self.pos.unwrap_or(0)
        }

        pub fn is_linked(&self) -> bool {
            self.is_bound
        }

        pub fn Unuse(&mut self) {
            self.is_bound = false;
            self.pos = None;
        }
    }

    impl<'a> RegExpMacroAssemblerS390<'a> {
        const kRegExpCodeSize: usize = 4096; // Example size

        pub fn new(isolate: &'a Isolate, zone: &'a Zone, mode: Mode, registers_to_save: i32) -> Self {
            assert_eq!(0, registers_to_save % 2);

            let masm_ = Rc::new(RefCell::new(MacroAssembler::new()));
            let no_root_array_scope_ = Rc::new(RefCell::new(MacroAssembler::new()));
            let mut assembler = RegExpMacroAssemblerS390 {
                isolate,
                zone,
                masm_: masm_.clone(),
                no_root_array_scope_: no_root_array_scope_.clone(),
                mode_: mode,
                num_registers_: registers_to_save,
                num_saved_registers_: registers_to_save,
                entry_label_: Label::new(),
                start_label_: Label::new(),
                success_label_: Label::new(),
                backtrack_label_: Label::new(),
                exit_label_: Label::new(),
                internal_failure_label_: Label::new(),
                fallback_label_: Label::new(),
            };

            // Replace with actual assembly instructions
            let masm = &mut assembler.masm_.borrow_mut();
            //let __ = ACCESS_MASM!(masm);
            assembler.b(&mut assembler.entry_label_); // We'll write the entry code later.

            // If the code gets too big or corrupted, an internal exception will be
            // raised, and we will exit right away.
            assembler.bind(&mut assembler.internal_failure_label_);
            //__ mov(r2, Operand(FAILURE));
            //__ Ret();
            assembler.mov(2, FAILURE as u64);
            assembler.Ret();

            assembler.bind(&mut assembler.start_label_); // And then continue from here.
            assembler
        }

        pub fn mov(&mut self, _reg: i32, _op: u64) {}
        pub fn Ret(&mut self) {}
        pub fn b(&mut self, _label: &mut Label) {}
        pub fn bind(&mut self, label: &mut Label) {
            label.bind();
        }

        pub fn has_backtrack_limit(&self) -> bool {
            false
        }

        pub fn can_fallback(&self) -> bool {
            false
        }

        pub fn backtrack_limit(&self) -> u64 {
            0
        }
        pub fn BranchOrBacktrack(&mut self, _condition: Condition, _to: &mut Label) {}
        pub fn GetOrAddRangeArray(&mut self, _ranges: &ZoneList<CharacterRange>) -> u64 {0}
        pub fn end_of_input_address(&mut self) -> i32 {0}
        pub fn current_input_offset(&mut self) -> i32 {0}
        pub fn current_character(&mut self) -> i32 {0}
        pub fn backtrack_stackpointer(&mut self) -> i32 {0}
        pub fn code_pointer(&mut self) -> i32 {0}
        pub fn frame_pointer(&mut self) -> i32 {0}
        pub fn CharSize(&self) -> i32 {0}
        pub fn global(&self) -> bool {false}
        pub fn global_with_zero_length_check(&self) -> bool {false}
        pub fn global_unicode(&self) -> bool {false}
        pub fn CheckNotInSurrogatePair(&mut self, _offset: i32, _label: &mut Label){}
        pub fn CheckStackGuardState(&mut self, _return_address: *mut i64, _raw_code: u64, _re_frame: u64, _extra_space: u64) -> i32 {0}
    }

    impl<'a> Drop for RegExpMacroAssemblerS390<'a> {
        fn drop(&mut self) {
            // Unuse labels in case we throw away the assembler without calling GetCode.
            self.entry_label_.Unuse();
            self.start_label_.Unuse();
            self.success_label_.Unuse();
            self.backtrack_label_.Unuse();
            self.exit_label_.Unuse();
            //self.check_preempt_label_.Unuse();
            //self.stack_overflow_label_.Unuse();
            self.internal_failure_label_.Unuse();
            self.fallback_label_.Unuse();
        }
    }

    impl<'a> RegExpMacroAssemblerS390<'a> {
        pub fn stack_limit_slack_slot_count() -> i32 {
            //RegExpStack::kStackLimitSlackSlotCount
            0
        }

        pub fn AdvanceCurrentPosition(&mut self, by: i32) {
            if by != 0 {
                //__ AddS64(current_input_offset(), Operand(by * char_size()));
            }
        }

        pub fn AdvanceRegister(&mut self, reg: i32, by: i32) {
            assert!(reg >= 0);
            assert!(self.num_registers_ > reg);
            if by != 0 {
                //if CpuFeatures::IsSupported(GENERAL_INSTR_EXT) && is_int8(by) {
                //  __ agsi(register_location(reg), Operand(by));
                //} else {
                //  __ LoadU64(r2, register_location(reg), r0);
                //  __ mov(r0, Operand(by));
                //  __ agr(r2, r0);
                //  __ StoreU64(r2, register_location(reg));
                //}
            }
        }

        pub fn Backtrack(&mut self) {
            //CheckPreemption();
            if self.has_backtrack_limit() {
                //Label next;
                //__ LoadU64(r2, MemOperand(frame_pointer(), kBacktrackCountOffset), r0);
                //__ AddS64(r2, r2, Operand(1));
                //__ StoreU64(r2, MemOperand(frame_pointer(), kBacktrackCountOffset), r0);
                //__ CmpU64(r2, Operand(backtrack_limit()));
                //__ bne(&next);

                //// Backtrack limit exceeded.
                //if (can_fallback()) {
                //  __ jmp(&fallback_label_);
                //} else {
                //  // Can't fallback, so we treat it as a failed match.
                //  Fail();
                //}

                //__ bind(&next);
            }
            //// Pop InstructionStream offset from backtrack stack, add InstructionStream
            //// and jump to location.
            //Pop(r2);
            //__ AddS64(r2, code_pointer());
            //__ b(r2);
        }

        pub fn CheckCharacter(&mut self, c: u32, on_equal: &mut Label) {
            //__ CmpU64(current_character(), Operand(c));
            self.BranchOrBacktrack(Condition::EQ, on_equal);
        }

        pub fn CheckCharacterGT(&mut self, limit: u16, on_greater: &mut Label) {
            //__ CmpU64(current_character(), Operand(limit));
            self.BranchOrBacktrack(Condition::GT, on_greater);
        }

        pub fn CheckAtStart(&mut self, cp_offset: i32, on_at_start: &mut Label) {
            //__ LoadU64(r3, MemOperand(frame_pointer(), kStringStartMinusOneOffset));
            //__ AddS64(r2, current_input_offset(),
            //          Operand(-char_size() + cp_offset * char_size()));
            //__ CmpS64(r2, r3);
            self.BranchOrBacktrack(Condition::EQ, on_at_start);
        }

        pub fn CheckNotAtStart(&mut self, cp_offset: i32, on_not_at_start: &mut Label) {
            //__ LoadU64(r3, MemOperand(frame_pointer(), kStringStartMinusOneOffset));
            //__ AddS64(r2, current_input_offset(),
            //          Operand(-char_size() + cp_offset * char_size()));
            //__ CmpS64(r2, r3);
            self.BranchOrBacktrack(Condition::NE, on_not_at_start);
        }

        pub fn CheckCharacterLT(&mut self, limit: u16, on_less: &mut Label) {
            //__ CmpU64(current_character(), Operand(limit));
            self.BranchOrBacktrack(Condition::LT, on_less);
        }

        pub fn CheckGreedyLoop(&mut self, on_equal: &mut Label) {
            //Label backtrack_non_equal;
            //__ CmpS64(current_input_offset(), MemOperand(backtrack_stackpointer(), 0));
            //__ bne(&backtrack_non_equal);
            //__ AddS64(backtrack_stackpointer(), Operand(kSystemPointerSize));

            self.BranchOrBacktrack(Condition::AL, on_equal);
            //__ bind(&backtrack_non_equal);
        }

        pub fn CheckNotBackReferenceIgnoreCase(
            &mut self,
            start_reg: i32,
            read_backward: bool,
            unicode: bool,
            on_no_match: &mut Label,
        ) {
            //Label fallthrough;
            //__ LoadU64(r2, register_location(start_reg));      // Index of start of
            //                                                   // capture
            //__ LoadU64(r3, register_location(start_reg + 1));  // Index of end
            //__ SubS64(r3, r3, r2);

            //// At this point, the capture registers are either both set or both cleared.
            //// If the capture length is zero, then the capture is either empty or cleared.
            //// Fall through in both cases.
            //__ beq(&fallthrough);

            //// Check that there are enough characters left in the input.
            //if (read_backward) {
            //  __ LoadU64(r5, MemOperand(frame_pointer(), kStringStartMinusOneOffset));
            //  __ AddS64(r5, r5, r3);
            //  __ CmpS64(current_input_offset(), r5);
            //  BranchOrBacktrack(le, on_no_match);
            //} else {
            //  __ AddS64(r0, r3, current_input_offset());
            //  BranchOrBacktrack(gt, on_no_match);
            //}

            //if (mode_ == LATIN1) {
            //  Label success;
            //  Label fail;
            //  Label loop_check;

            //  // r2 - offset of start of capture
            //  // r3 - length of capture
            //  __ AddS64(r2, end_of_input_address());
            //  __ AddS64(r4, current_input_offset(), end_of_input_address());
            //  if (read_backward) {
            //    __ SubS64(r4, r4, r3);  // Offset by length when matching backwards.
            //  }
            //  __ mov(r1, Operand::Zero());

            //  // r1 - Loop index
            //  // r2 - Address of start of capture.
            //  // r4 - Address of current input position.

            //  Label loop;
            //  __ bind(&loop);
            //  __ LoadU8(r5, MemOperand(r2, r1));
            //  __ LoadU8(r6, MemOperand(r4, r1));

            //  __ CmpS64(r6, r5);
            //  __ beq(&loop_check);

            //  // Mismatch, try case-insensitive match (converting letters to lower-case).
            //  __ Or(r5, Operand(0x20));  // Convert capture character to lower-case.
            //  __ Or(r6, Operand(0x20));  // Also convert input character.
            //  __ CmpS64(r6, r5);
            //  __ bne(&fail);
            //  __ SubS64(r5, Operand('a'));
            //  __ CmpU64(r5, Operand('z' - 'a'));       // Is r5 a lowercase letter?
            //  __ ble(&loop_check);                     // In range 'a'-'z'.
            //  // Latin-1: Check for values in range [224,254] but not 247.
            //  __ SubS64(r5, Operand(224 - 'a'));
            //  __ CmpU64(r5, Operand(254 - 224));
            //  __ bgt(&fail);                           // Weren't Latin-1 letters.
            //  __ CmpU64(r5, Operand(247 - 224));       // Check for 247.
            //  __ beq(&fail);

            //  __ bind(&loop_check);
            //  __ la(r1, MemOperand(r1, char_size()));
            //  __ CmpS64(r1, r3);
            //  __ blt(&loop);
            //  __ b(&success);

            //  __ bind(&fail);
            //  BranchOrBacktrack(al, on_no_match);

            //  __ bind(&success);
            //  // Compute new value of character position after the matched part.
            //  __ SubS64(current_input_offset(), r4, end_of_input_address());
            //  if (read_backward) {
            //    __ LoadU64(r2,
            //               register_location(start_reg));  // Index of start of capture
            //    __ LoadU64(r3,
            //               register_location(start_reg + 1));  // Index of end of capture
            //    __ AddS64(current_input_offset(), current_input_offset(), r2);
            //    __ SubS64(current_input_offset(), current_input_offset(), r3);
            //  }
            //  __ AddS64(current_input_offset(), r1);
            //} else {
            //  DCHECK(mode_ == UC16);
            //  int argument_count = 4;
            //  __ PrepareCallCFunction(argument_count, r4);

            //  // r2 - offset of start of capture
            //  // r3 - length of capture

            //  // Put arguments into arguments registers.
            //  // Parameters are
            //  //   r2: Address byte_offset1 - Address captured substring's start.
            //  //   r3: Address byte_offset2 - Address of current character position.
            //  //   r4: size_t byte_length - length of capture in bytes(!)
            //  //   r5: Isolate* isolate.

            //  // Address of start of capture.
            //  __ AddS64(r2, end_of_input_address());
            //  // Length of capture.
            //  __ mov(r4, r3);
            //  // Save length in callee-save register for use on return.
            //  __ mov(r6, r3);
            //  // Address of current input position.
            //  __ AddS64(r3, current_input_offset(), end_of_input_address());
            //  if (read_backward) {
            //    __ SubS64(r3, r3, r6);
            //  }
            //// Isolate.
            //  __ mov(r5, Operand(ExternalReference::isolate_address(isolate())));

            //  {
            //    AllowExternalCallThatCantCauseGC scope(masm_.get());
            //    ExternalReference function =
            //        unicode
            //            ? ExternalReference::re_case_insensitive_compare_unicode()
            //            : ExternalReference::re_case_insensitive_compare_non_unicode();
            //    CallCFunctionFromIrregexpCode(function, argument_count);
            //  }

            //  // Check if function returned non-zero for success or zero for failure.
            //  __ CmpS64(r2, Operand::Zero());
            //  BranchOrBacktrack(eq, on_no_match);

            //  // On success, advance position by length of capture.
            //  if (read_backward) {
            //    __ SubS64(current_input_offset(), current_input_offset(), r6);
            //  } else {
            //    __ AddS64(current_input_offset(), current_input_offset(), r6);
            //  }
            //}

            //__ bind(&fallthrough);
        }

        pub fn CheckNotBackReference(
            &mut self,
            start_reg: i32,
            read_backward: bool,
            on_no_match: &mut Label,
        ) {
            //Label fallthrough;

            //// Find length of back-referenced capture.
            //__ LoadU64(r2, register_location(start_reg));
            //__ LoadU64(r3, register_location(start_reg + 1));
            //__ SubS64(r3, r3, r2);  // Length to check.

            //// At this point, the capture registers are either both set or both cleared.
            //// If the capture length is zero, then the capture is either empty or cleared.
            //// Fall through in both cases.
            //__ beq(&fallthrough);

            //// Check that there are enough characters left in the input.
            //if (read_backward) {
            //  __ LoadU64(r5, MemOperand(frame_pointer(), kStringStartMinusOneOffset));
            //  __ AddS64(r5, r5, r3);
            //  __ CmpS64(current_input_offset(), r5);
            //  BranchOrBacktrack(le, on_no_match);
            //} else {
            //  __ AddS64(r0, r3, current_input_offset());
            //  BranchOrBacktrack(gt, on_no_match, cr0);
            //}

            //// r2 - offset of start of capture
            //// r3 - length of capture
            //__ la(r2, MemOperand(r2, end_of_input_address()));
            //__ la(r4, MemOperand(current_input_offset(), end_of_input_address()));
            //if (read_backward) {
            //  __ SubS64(r4, r4, r3);  // Offset by length when matching backwards.
            //}
            //__ mov(r1, Operand::Zero());

            //Label loop;
            //__ bind(&loop);
            //if (mode_ == LATIN1) {
            //  __ LoadU8(r5, MemOperand(r2, r1));
            //  __ LoadU8(r6, MemOperand(r4, r1));
            //} else {
            //  DCHECK(mode_ == UC16);
            //  __ LoadU16(r5, MemOperand(r2, r1));
            //  __ LoadU16(r6, MemOperand(r4, r1));
            //}
            //__ la(r1, MemOperand(r1, char_size()));
            //__ CmpS64(r5, r6);
            //BranchOrBacktrack(ne, on_no_match);
            //__ CmpS64(r1, r3);
            //__ blt(&loop);

            //// Move current character position to position after match.
            //__ SubS64(current_input_offset(), r4, end_of_input_address());
            //if (read_backward) {
            //  __ LoadU64(r2, register_location(start_reg));  // Index of start of capture
            //  __ LoadU64(r3,
            //             register_location(start_reg + 1));  // Index of end of capture
            //  __ AddS64(current_input_offset(), current_input_offset(), r2);
            //  __ SubS64(current_input_offset(), current_input_offset(), r3);
            //}
            //__ AddS64(current_input_offset(), r1);

            //__ bind(&fallthrough);
        }

        pub fn CheckNotCharacter(&mut self, c: u32, on_not_equal: &mut Label) {
            //__ CmpU64(current_character(), Operand(c));
            self.BranchOrBacktrack(Condition::NE, on_not_equal);
        }

        pub fn CheckCharacterAfterAnd(
            &mut self,
            c: u32,
            mask: u32,
            on_equal: &mut Label,
        ) {
            //__ AndP(r2, current_character(), Operand(mask));
            //if (c != 0) {
            //  __ CmpU64(r2, Operand(c));
            //}
            self.BranchOrBacktrack(Condition::EQ, on_equal);
        }

        pub fn CheckNotCharacterAfterAnd(
            &mut self,
            c: u32,
            mask: u32,
            on_not_equal: &mut Label,
        ) {
            //__ AndP(r2, current_character(), Operand(mask));
            //if (c != 0) {
            //  __ CmpU64(r2, Operand(c));
            //}
            self.BranchOrBacktrack(Condition::NE, on_not_equal);
        }

        pub fn CheckNotCharacterAfterMinusAnd(
            &mut self,
            c: u16,
            minus: u16,
            mask: u16,
            on_not_equal: &mut Label,
        ) {
            assert!(String::kMaxUtf16CodeUnit > minus);
            //__ lay(r2, MemOperand(current_character(), -minus));
            //__ And(r2, Operand(mask));
            //if (c != 0) {
            //  __ CmpU64(r2, Operand(c));
            //}
            self.BranchOrBacktrack(Condition::NE, on_not_equal);
        }

        pub fn CheckCharacterInRange(
            &mut self,
            from: u16,
            to: u16,
            on_in_range: &mut Label,
        ) {
            //__ lay(r2, MemOperand(current_character(), -from));
            //__ CmpU64(r2, Operand(to - from));
            self.BranchOrBacktrack(Condition::LE, on_in_range); // Unsigned lower-or-same condition.
        }

        pub fn CheckCharacterNotInRange(
            &mut self,
            from: u16,
            to: u16,
            on_not_in_range: &mut Label,
        ) {
            //__ lay(r2, MemOperand(current_character(), -from));
            //__ CmpU64(r2, Operand(to - from));
            self.BranchOrBacktrack(Condition::GT, on_not_in_range); // Unsigned higher condition.
        }

        pub fn CallIsCharacterInRangeArray(&mut self, ranges: &ZoneList<CharacterRange>) {
            //static const int kNumArguments = 2;
            //__ PrepareCallCFunction(kNumArguments, r0);

            //__ mov(r2, current_character());
            //__ mov(r3, Operand(GetOrAddRangeArray(ranges)));

            //{
            //  // We have a frame (set up in GetCode), but the assembler doesn't know.
            //  FrameScope scope(masm_.get(), StackFrame::MANUAL);
            //  CallCFunctionFromIrregexpCode(
            //      ExternalReference::re_is_character_in_range_array(), kNumArguments);
            //}

            //__ mov(code_pointer(), Operand(masm_->CodeObject()));
        }

        pub fn CheckCharacterInRangeArray(
            &mut self,
            ranges: &ZoneList<CharacterRange>,
            on_in_range: &mut Label,
        ) -> bool {
            self.CallIsCharacterInRangeArray(ranges);
            //__ CmpS64(r2, Operand::Zero());
            self.BranchOrBacktrack(Condition::NE, on_in_range);
            true
        }

        pub fn CheckCharacterNotInRangeArray(
            &mut self,
            ranges: &ZoneList<CharacterRange>,
            on_not_in_range: &mut Label,
        ) -> bool {
            self.CallIsCharacterInRangeArray(ranges);
            //__ CmpS64(r2, Operand::Zero());
            self.BranchOrBacktrack(Condition::EQ, on_not_in_range);
            true
        }

        pub fn CheckBitInTable(&mut self, _table: &Handle<ByteArray>, on_bit_set: &mut Label) {
            //__ mov(r2, Operand(table));
            //Register index = current_character();
            //if (mode_ != LATIN1 || kTableMask != String::kMaxOneByteCharCode) {
            //  __ AndP(r3, current_character(), Operand(kTableSize - 1));
            //  index = r3;
            //}
            //__ LoadU8(r2, MemOperand(r2, index,
            //                         (OFFSET_OF_DATA_START(ByteArray) - kHeapObjectTag)));
            //__ CmpS64(r2, Operand::Zero());
            self.BranchOrBacktrack(Condition::NE, on_bit_set);
        }

        pub fn SkipUntilBitInTable(
            &mut self,
            cp_offset: i32,
            _table: &Handle<ByteArray>,
            _nibble_table: &Handle<ByteArray>,
            advance_by: i32,
        ) {
            //// TODO(pthier): Optimize. Table can be loaded outside of the loop.
            //Label cont, again;
            //Bind(&again);
            //LoadCurrentCharacter(cp_offset, &cont, true);
            //CheckBitInTable(table, &cont);
            //AdvanceCurrentPosition(advance_by);
            //GoTo(&again);
            //Bind(&cont);
        }

        pub fn CheckSpecialClassRanges(
            &mut self,
            type_: StandardCharacterSet,
            on_no_match: &mut Label,
        ) -> bool {
            //// Range checks (c in min..max) are generally implemented by an unsigned
            //// (c - min) <= (max - min) check
            //// TODO(jgruber): No custom implementation (yet): s(UC16), S(UC16).
            match type_ {
                StandardCharacterSet::kWhitespace => {
                    //// Match space-characters.
                    if self.mode_ == Mode::LATIN1 {
                        //// One byte space characters are '\t'..'\r', ' ' and \u00a0.
                        //Label success;
                        //__ CmpS64(current_character(), Operand(' '));
                        //__ beq(&success);
                        //// Check range 0x09..0x0D.
                        //__ SubS64(r2, current_character(), Operand('\t'));
                        //__ CmpU64(r2, Operand('\r' - '\t'));
                        //__ ble(&success);
                        //// \u00a0 (NBSP).
                        //__ CmpU64(r2, Operand