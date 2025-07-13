// Converted from V8 C++ source files:
// Header: regexp-macro-assembler-ppc.h
// Implementation: regexp-macro-assembler-ppc.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::sync::Mutex;

pub struct RegExpMacroAssemblerPPC {
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

const K_REG_EXP_CODE_SIZE: i32 = 1024;

impl RegExpMacroAssemblerPPC {
    pub fn new(isolate: *mut Isolate, zone: *mut Zone, mode: Mode, registers_to_save: i32) -> RegExpMacroAssemblerPPC {
        assert_eq!(0, registers_to_save % 2);

        let mut masm_ = Box::new(MacroAssembler::new(
            isolate,
            CodeObjectRequired::kYes,
        ));
        let mut assembler = unsafe { masm_.as_mut() };

        let no_root_array_scope_ = NoRootArrayScope {
        };
        let mut result = RegExpMacroAssemblerPPC {
            masm_: masm_,
            no_root_array_scope_: no_root_array_scope_,
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

        let internal_failure_label = &mut result.internal_failure_label_;
        assembler.b(&result.entry_label_);
        assembler.bind(internal_failure_label);
        assembler.li(Register{}, Operand::new(FAILURE));
        assembler.Ret();
        assembler.bind(&result.start_label_);
        result
    }

    fn isolate(&self) -> *mut Isolate {
        self.masm_.isolate()
    }

    pub fn lr(&mut self) -> Register{
       Register{}
    }
    pub fn stack_limit_slack_slot_count(&self) -> i32 {
        RegExpStack::kStackLimitSlackSlotCount
    }

    pub fn advance_current_position(&mut self, by: i32) {
        if by != 0 {
            let char_size_val = self.char_size();
            if (by * char_size_val) < i16::MAX as i32 && (by * char_size_val) > i16::MIN as i32 {
                self.masm_.addi(
                    self.current_input_offset(),
                    self.current_input_offset(),
                    Operand::new(by * char_size_val),
                );
            } else {
                self.masm_.mov(Register{}, Operand::new(by * char_size_val));
                self.masm_.add(
                    self.current_input_offset(),
                    Register{},
                    self.current_input_offset(),
                );
            }
        }
    }

    pub fn advance_register(&mut self, reg: i32, by: i32) {
        assert!(reg >= 0);
        assert!(self.num_registers_ > reg);

        if by != 0 {
            self.masm_.LoadU64(Register{}, self.register_location(reg), Register{});
            self.masm_.mov(Register{}, Operand::new(by));
            self.masm_.add(Register{}, Register{}, Register{});
            self.masm_.StoreU64(Register{}, self.register_location(reg), Register{});
        }
    }

    pub fn backtrack(&mut self) {
        self.check_preemption();
        if self.has_backtrack_limit() {
            let next = Label::new();
            self.masm_.LoadU64(Register{}, MemOperand{});
            self.masm_.addi(Register{}, Register{}, Operand::new(1));
            self.masm_.StoreU64(Register{}, MemOperand{});
            self.masm_.mov(Register{}, Operand::new(self.backtrack_limit()));
            self.masm_.CmpS64(Register{}, Register{});
            self.masm_.bne(&next);

            if self.can_fallback() {
                self.masm_.b(&self.fallback_label_);
            } else {
                self.fail();
            }
            self.masm_.bind(&next);
        }

        self.masm_.Pop(Register{});
        self.masm_.add(Register{}, Register{}, self.code_pointer());
        self.masm_.Jump(Register{});
    }

    pub fn bind(&mut self, label: &mut Label) {
        self.masm_.bind(label);
    }

    pub fn check_character(&mut self, c: u32, on_equal: &mut Label) {
        self.masm_.CmpU64(self.current_character(), Operand::new(c), Register{});
        self.branch_or_backtrack(Condition::eq, on_equal);
    }

    pub fn check_character_gt(&mut self, limit: base::uc16, on_greater: &mut Label) {
        self.masm_.CmpU64(self.current_character(), Operand::new(limit as u32), Register{});
        self.branch_or_backtrack(Condition::gt, on_greater);
    }

    pub fn check_at_start(&mut self, cp_offset: i32, on_at_start: &mut Label) {
        self.masm_.LoadU64(Register{}, MemOperand{});
        self.masm_.addi(
            Register{},
            self.current_input_offset(),
            Operand::new(-self.char_size() + cp_offset * self.char_size()),
        );
        self.masm_.CmpS64(Register{}, Register{});
        self.branch_or_backtrack(Condition::eq, on_at_start);
    }

    pub fn check_not_at_start(&mut self, cp_offset: i32, on_not_at_start: &mut Label) {
        self.masm_.LoadU64(Register{}, MemOperand{});
        self.masm_.addi(
            Register{},
            self.current_input_offset(),
            Operand::new(-self.char_size() + cp_offset * self.char_size()),
        );
        self.masm_.CmpS64(Register{}, Register{});
        self.branch_or_backtrack(Condition::ne, on_not_at_start);
    }

    pub fn check_character_lt(&mut self, limit: base::uc16, on_less: &mut Label) {
        self.masm_.CmpU64(self.current_character(), Operand::new(limit as u32), Register{});
        self.branch_or_backtrack(Condition::lt, on_less);
    }

    pub fn check_greedy_loop(&mut self, on_equal: &mut Label) {
        let backtrack_non_equal = Label::new();
        self.masm_.LoadU64(Register{}, MemOperand{});
        self.masm_.CmpS64(self.current_input_offset(), Register{});
        self.masm_.bne(&backtrack_non_equal);
        self.masm_.addi(
            self.backtrack_stackpointer(),
            self.backtrack_stackpointer(),
            Operand::new(kSystemPointerSize),
        );

        self.masm_.bind(&backtrack_non_equal);
        self.branch_or_backtrack(Condition::eq, on_equal);
    }

    pub fn check_not_back_reference_ignore_case(
        &mut self,
        start_reg: i32,
        read_backward: bool,
        unicode: bool,
        on_no_match: &mut Label,
    ) {
        let fallthrough = Label::new();
        self.masm_.LoadU64(Register{}, self.register_location(start_reg), Register{});
        self.masm_.LoadU64(Register{}, self.register_location(start_reg + 1), Register{});
        self.masm_.sub(Register{}, Register{}, Register{}, LeaveOE::LeaveOE, SetRC::SetRC);

        self.masm_.beq(&fallthrough, Cr0::cr0);

        if read_backward {
            self.masm_.LoadU64(Register{}, MemOperand{});
            self.masm_.add(Register{}, Register{}, Register{});
            self.masm_.CmpS64(self.current_input_offset(), Register{});
            self.branch_or_backtrack(Condition::le, on_no_match);
        } else {
            self.masm_.add(Register{}, Register{}, self.current_input_offset(), LeaveOE::LeaveOE, SetRC::SetRC);
            self.branch_or_backtrack(Condition::gt, on_no_match, Cr0::cr0);
        }

        if self.mode_ == Mode::LATIN1 {
            let success = Label::new();
            let fail = Label::new();
            let loop_check = Label::new();

            self.masm_.add(Register{}, Register{}, self.end_of_input_address());
            self.masm_.add(Register{}, self.end_of_input_address(), self.current_input_offset());
            if read_backward {
                self.masm_.sub(Register{}, Register{}, Register{});
            }
            self.masm_.add(Register{}, Register{}, Register{});

            let loop_label = Label::new();
            self.masm_.bind(&loop_label);
            self.masm_.lbz(Register{}, MemOperand{});
            self.masm_.addi(Register{}, Register{}, Operand::new(self.char_size()));
            self.masm_.lbz(Register{}, MemOperand{});
            self.masm_.addi(Register{}, Register{}, Operand::new(self.char_size()));
            self.masm_.CmpS64(Register{}, Register{});
            self.masm_.beq(&loop_check);

            self.masm_.ori(Register{}, Register{}, Operand::new(0x20));
            self.masm_.ori(Register{}, Register{}, Operand::new(0x20));
            self.masm_.CmpS64(Register{}, Register{});
            self.masm_.bne(&fail);
            self.masm_.subi(Register{}, Register{}, Operand::new('a' as i32));
            self.masm_.cmpli(Register{}, Operand::new(('z' as i32) - ('a' as i32)));
            self.masm_.ble(&loop_check);
            self.masm_.subi(Register{}, Register{}, Operand::new(224 - ('a' as i32)));
            self.masm_.cmpli(Register{}, Operand::new(254 - 224));
            self.masm_.bgt(&fail);
            self.masm_.cmpi(Register{}, Operand::new(247 - 224));
            self.masm_.beq(&fail);

            self.masm_.bind(&loop_check);
            self.masm_.CmpS64(Register{}, Register{});
            self.masm_.blt(&loop_label);
            self.masm_.b(&success);

            self.masm_.bind(&fail);
            self.branch_or_backtrack(Condition::al, on_no_match);

            self.masm_.bind(&success);
            self.masm_.sub(self.current_input_offset(), Register{}, self.end_of_input_address());
            if read_backward {
                self.masm_.LoadU64(Register{}, self.register_location(start_reg), Register{});
                self.masm_.LoadU64(Register{}, self.register_location(start_reg + 1), Register{});
                self.masm_.add(self.current_input_offset(), self.current_input_offset(), Register{});
                self.masm_.sub(self.current_input_offset(), self.current_input_offset(), Register{});
            }
        } else {
            assert!(self.mode_ == Mode::UC16);
            let argument_count = 4;
            self.masm_.PrepareCallCFunction(argument_count, Register{});

            self.masm_.add(Register{}, Register{}, self.end_of_input_address());
            self.masm_.mr(Register{}, Register{});
            self.masm_.mr(Register{}, Register{});
            self.masm_.add(self.current_input_offset(), self.current_input_offset(), self.end_of_input_address());
            if read_backward {
                self.masm_.sub(self.current_input_offset(), self.current_input_offset(), Register{});
            }
            self.masm_.mov(Register{}, Operand::new(ExternalReference::isolate_address(self.isolate())));

            {
               // AllowExternalCallThatCantCauseGC scope(self.masm_.get());
                let function = if unicode {
                  //  ExternalReference::re_case_insensitive_compare_unicode()
                   ExternalReference {}
                } else {
                   // ExternalReference::re_case_insensitive_compare_non_unicode()
                   ExternalReference{}
                };
               // self.call_c_function_from_irregexp_code(function, argument_count);
            }
            self.masm_.cmpi(Register{}, Operand::Zero());
            self.branch_or_backtrack(Condition::eq, on_no_match);
            if read_backward {
                self.masm_.sub(self.current_input_offset(), self.current_input_offset(), Register{});
            } else {
                self.masm_.add(self.current_input_offset(), self.current_input_offset(), Register{});
            }
        }

        self.masm_.bind(&fallthrough);
    }
     pub fn check_not_back_reference(
        &mut self,
        start_reg: i32,
        read_backward: bool,
        on_no_match: &mut Label,
    ) {
        let fallthrough = Label::new();

        self.masm_.LoadU64(Register{}, self.register_location(start_reg), Register{});
        self.masm_.LoadU64(Register{}, self.register_location(start_reg + 1), Register{});
        self.masm_.sub(Register{}, Register{}, Register{}, LeaveOE::LeaveOE, SetRC::SetRC);

        self.masm_.beq(&fallthrough, Cr0::cr0);
        if read_backward {
            self.masm_.LoadU64(Register{}, MemOperand{});
            self.masm_.add(Register{}, Register{}, Register{});
            self.masm_.CmpS64(self.current_input_offset(), Register{});
            self.branch_or_backtrack(Condition::le, on_no_match);
        } else {
            self.masm_.add(Register{}, Register{}, self.current_input_offset(), LeaveOE::LeaveOE, SetRC::SetRC);
            self.branch_or_backtrack(Condition::gt, on_no_match, Cr0::cr0);
        }

        self.masm_.add(Register{}, Register{}, self.end_of_input_address());
        self.masm_.add(Register{}, self.end_of_input_address(), self.current_input_offset());
        if read_backward {
            self.masm_.sub(Register{}, Register{}, Register{});
        }
        self.masm_.add(Register{}, Register{}, Register{});

        let loop_label = Label::new();
        self.masm_.bind(&loop_label);
        if self.mode_ == Mode::LATIN1 {
            self.masm_.lbz(Register{}, MemOperand{});
            self.masm_.addi(Register{}, Register{}, Operand::new(self.char_size()));
            self.masm_.lbz(Register{}, MemOperand{});
            self.masm_.addi(Register{}, Register{}, Operand::new(self.char_size()));
        } else {
            assert!(self.mode_ == Mode::UC16);
            self.masm_.lhz(Register{}, MemOperand{});
            self.masm_.addi(Register{}, Register{}, Operand::new(self.char_size()));
            self.masm_.lhz(Register{}, MemOperand{});
            self.masm_.addi(Register{}, Register{}, Operand::new(self.char_size()));
        }
        self.masm_.CmpS64(Register{}, Register{});
        self.branch_or_backtrack(Condition::ne, on_no_match);
        self.masm_.CmpS64(Register{}, Register{});
        self.masm_.blt(&loop_label);

        self.masm_.sub(self.current_input_offset(), Register{}, self.end_of_input_address());
        if read_backward {
            self.masm_.LoadU64(Register{}, self.register_location(start_reg), Register{});
            self.masm_.LoadU64(Register{}, self.register_location(start_reg + 1), Register{});
            self.masm_.add(self.current_input_offset(), self.current_input_offset(), Register{});
            self.masm_.sub(self.current_input_offset(), self.current_input_offset(), Register{});
        }

        self.masm_.bind(&fallthrough);
    }
    pub fn check_not_character(&mut self, c: u32, on_not_equal: &mut Label) {
        self.masm_.CmpU64(self.current_character(), Operand::new(c), Register{});
        self.branch_or_backtrack(Condition::ne, on_not_equal);
    }

    pub fn check_character_after_and(&mut self, c: u32, mask: u32, on_equal: &mut Label) {
        self.masm_.mov(Register{}, Operand::new(mask));
        if c == 0 {
            self.masm_.and_(Register{}, self.current_character(), Register{}, SetRC::SetRC);
        } else {
            self.masm_.and_(Register{}, self.current_character(), Register{});
            self.masm_.CmpU64(Register{}, Operand::new(c), Register{}, Cr0::cr0);
        }
        self.branch_or_backtrack(Condition::eq, on_equal, Cr0::cr0);
    }

    pub fn check_not_character_after_and(&mut self, c: u32, mask: u32, on_not_equal: &mut Label) {
        self.masm_.mov(Register{}, Operand::new(mask));
        if c == 0 {
            self.masm_.and_(Register{}, self.current_character(), Register{}, SetRC::SetRC);
        } else {
            self.masm_.and_(Register{}, self.current_character(), Register{});
            self.masm_.CmpU64(Register{}, Operand::new(c), Register{}, Cr0::cr0);
        }
        self.branch_or_backtrack(Condition::ne, on_not_equal, Cr0::cr0);
    }

    pub fn check_not_character_after_minus_and(
        &mut self,
        c: base::uc16,
        minus: base::uc16,
        mask: base::uc16,
        on_not_equal: &mut Label,
    ) {
        assert!(String::kMaxUtf16CodeUnit > minus);
        self.masm_.subi(Register{}, self.current_character(), Operand::new(minus as i32));
        self.masm_.mov(Register{}, Operand::new(mask as u32));
        self.masm_.and_(Register{}, Register{}, Register{});
        self.masm_.CmpU64(Register{}, Operand::new(c as u32), Register{});
        self.branch_or_backtrack(Condition::ne, on_not_equal);
    }

    pub fn check_character_in_range(&mut self, from: base::uc16, to: base::uc16, on_in_range: &mut Label) {
        self.masm_.mov(Register{}, Operand::new(from as u32));
        self.masm_.sub(Register{}, self.current_character(), Register{});
        self.masm_.CmpU64(Register{}, Operand::new((to - from) as u32), Register{});
        self.branch_or_backtrack(Condition::le, on_in_range);
    }

    pub fn check_character_not_in_range(
        &mut self,
        from: base::uc16,
        to: base::uc16,
        on_not_in_range: &mut Label,
    ) {
        self.masm_.mov(Register{}, Operand::new(from as u32));
        self.masm_.sub(Register{}, self.current_character(), Register{});
        self.masm_.CmpU64(Register{}, Operand::new((to - from) as u32), Register{});
        self.branch_or_backtrack(Condition::gt, on_not_in_range);
    }
   
    fn call_is_character_in_range_array(&mut self, ranges: *const ZoneList<CharacterRange>) {
      
    }

    pub fn check_character_in_range_array(
        &mut self,
        ranges: *const ZoneList<CharacterRange>,
        on_in_range: &mut Label,
    ) -> bool {
       // self.call_is_character_in_range_array(ranges);
       self.masm_.cmpi(Register{}, Operand::Zero());
       self.branch_or_backtrack(Condition::ne, on_in_range);
       return true;
    }
    pub fn check_character_not_in_range_array(
        &mut self,
        ranges: *const ZoneList<CharacterRange>,
        on_not_in_range: &mut Label,
    ) -> bool {
         // self.call_is_character_in_range_array(ranges);
       self.masm_.cmpi(Register{}, Operand::Zero());
       self.branch_or_backtrack(Condition::eq, on_not_in_range);
       return true;
    }

    pub fn check_bit_in_table(&mut self, table: Handle<ByteArray>, on_bit_set: &mut Label) {
        self.masm_.mov(Register{}, Operand::new(table.location));
        if self.mode_ != Mode::LATIN1 || kTableMask != String::kMaxOneByteCharCode {
            self.masm_.andi(Register{}, self.current_character(), Operand::new(kTableSize - 1));
            self.masm_.addi(
                Register{},
                Register{},
                Operand::new(OFFSET_OF_DATA_START(ByteArray) as i32 - kHeapObjectTag),
            );
        } else {
            self.masm_.addi(
                Register{},
                self.current_character(),
                Operand::new(OFFSET_OF_DATA_START(ByteArray) as i32 - kHeapObjectTag),
            );
        }
        self.masm_.lbzx(Register{}, MemOperand{});
        self.masm_.cmpi(Register{}, Operand::Zero());
        self.branch_or_backtrack(Condition::ne, on_bit_set);
    }
    pub fn skip_until_bit_in_table(
        &mut self,
        cp_offset: i32,
        table: Handle<ByteArray>,
        nibble_table: Handle<ByteArray>,
        advance_by: i32,
    ) {
       let cont = Label::new();
       let again = Label::new();
       self.bind(&again);
       //self.LoadCurrentCharacter(cp_offset, &cont, true);
       self.check_bit_in_table(table, &cont);
       self.advance_current_position(advance_by);
       self.go_to(&again);
       self.bind(&cont);
    }

    pub fn check_special_class_ranges(&mut self, type_: StandardCharacterSet, on_no_match: &mut Label) -> bool {
        match type_ {
            StandardCharacterSet::kWhitespace => {
                if self.mode_ == Mode::LATIN1 {
                    let success = Label::new();
                    self.masm_.cmpi(self.current_character(), Operand::new(' ' as u32));
                    self.masm_.beq(&success);

                    self.masm_.subi(Register{}, self.current_character(), Operand::new('\t' as i32));
                    self.masm_.cmpli(Register{}, Operand::new(('\r' as i32) - ('\t' as i32)));
                    self.masm_.ble(&success);

                    self.masm_.cmpi(Register{}, Operand::new(0x00A0 - ('\t' as i32)));
                    self.branch_or_backtrack(Condition::ne, on_no_match);

                    self.masm_.bind(&success);
                    return true;
                }
                false
            }
            StandardCharacterSet::kNotWhitespace => false,
            StandardCharacterSet::kDigit => {
                self.masm_.subi(Register{}, self.current_character(), Operand::new('0' as i32));
                self.masm_.cmpli(Register{}, Operand::new(('9' as i32) - ('0' as i32)));
                self.branch_or_backtrack(Condition::gt, on_no_match);
                true
            }
            StandardCharacterSet::kNotDigit => {
                self.masm_.subi(Register{}, self.current_character(), Operand::new('0' as i32));
                self.masm_.cmpli(Register{}, Operand::new(('9' as i32) - ('0' as i32)));
                self.branch_or_backtrack(Condition::le, on_no_match);
                true
            }
            StandardCharacterSet::kNotLineTerminator => {
                self.masm_.xori(Register{}, self.current_character(), Operand::new(0x01));
                self.masm_.subi(Register{}, Register{}, Operand::new(0x0B));
                self.masm_.cmpli(Register{}, Operand::new(0x0C - 0x0B));
                self.branch_or_backtrack(Condition::le, on_no_match);

                if self.mode_ == Mode::UC16 {
                    self.masm_.subi(Register{}, Register{}, Operand::new(0x2028 - 0x0B));
                    self.masm_.cmpli(Register{}, Operand::new(1));
                    self.branch_or_backtrack(Condition::le, on_no_match);
                }
                true
            }
            StandardCharacterSet::kLineTerminator => {
                self.masm_.xori(Register{}, self.current_character(), Operand::new(0x01));
                self.masm_.subi(Register{}, Register{}, Operand::new(0x0B));
                self.masm_.cmpli(Register{}, Operand::new(0x0C - 0x0B));
                if self.mode_ == Mode::LATIN1 {
                    self.branch_or_backtrack(Condition::gt, on_no_match);
                } else {
                    let done = Label::new();
                    self.masm_.ble(&done);
                    self.masm_.subi(Register{}, Register{}, Operand::new(0x2028 - 0x0B));
                    self.masm_.cmpli(Register{}, Operand::new(1));
                    self.branch_or_backtrack(Condition::gt, on_no_match);
                    self.masm_.bind(&done);
                }
                true
            }
            StandardCharacterSet::kWord => {
                if self.mode_ != Mode::LATIN1 {
                    self.masm_.cmpi(self.current_character(), Operand::new('z' as u32));
                    self.branch_or_backtrack(Condition::gt, on_no_match);
                }

                let map = ExternalReference::re_word_character_map();
                self.masm_.mov(Register{}, Operand::new(map.location));
                self.masm_.lbzx(Register{}, MemOperand{});
                self.masm_.cmpli(Register{}, Operand::Zero());
                self.branch_or_backtrack(Condition::eq, on_no_match);
                true
            }
            StandardCharacterSet::kNotWord => {
                let done = Label::new();
                if self.mode_ != Mode::LATIN1 {
                    self.masm_.cmpli(self.current_character(), Operand::new('z' as u32));
                    self.masm_.bgt(&done);
                }

                let map = ExternalReference::re_word_character_map();
                self.masm_.mov(Register{}, Operand::new(map.location));
                self.masm_.lbzx(Register{}, MemOperand{});
                self.masm_.cmpli(Register{}, Operand::Zero());
                self.branch_or_backtrack(Condition::ne, on_no_match);
                if self.mode_ != Mode::LATIN1 {
                    self.masm_.bind(&done);
                }
                true
            }
            StandardCharacterSet::kEverything => true,
        }
    }
        pub fn set_register(&mut self, register_index: i32, to: i32) {
        assert!(register_index >= self.num_saved_registers_);
        self.masm_.mov(Register{}, Operand::new(to));
        self.masm_.StoreU64(Register{}, self.register_location(register_index), Register{});
    }

    pub fn go_to(&mut self, to: &mut Label) {
        self.branch_or_backtrack(Condition::al, to);
    }
    pub fn implementation(&self) -> IrregexpImplementation {
          IrregexpImplementation::kPPCImplementation
    }
    pub fn read_current_position_from_register(&mut self, reg: i32) {
       self.masm_.LoadU64(self.current_input_offset(), self.register_location(reg), Register{});
    }
    pub fn clear_registers(&mut self, reg_from: i32, reg_to: i32) {
          assert!(reg_from <= reg_to);
        self.masm_.LoadU64(Register{}, MemOperand{});
        for reg in reg_from..=reg_to {
           self.masm_.StoreU64(Register{}, self.register_location(reg), Register{});
        }
    }
    fn fail(&mut self) {
      self.masm_.li(Register{}, Operand::new(FAILURE));
      self.masm_.b(&self.exit_label_);
    }
    pub fn set_current_position_from_end(&mut self, by: i32) {
     let after_position = Label::new();
     self.masm_.CmpS64(self.current_input_offset(), Operand::new(-by * self.char_size()), Register{});
     self.masm_.bge(&after_position);
     self.masm_.mov(self.current_input_offset(), Operand::new(-by * self.char_size()));
    // self.LoadCurrentCharacterUnchecked(-1,1);
     self.masm_.bind(&after_position);

    }
   
    pub fn push_backtrack(&mut self, label: &mut Label) {
        self.masm_.mov_label_offset(Register{}, label);
        self.push(Register{});
        self.check_stack_limit();
    }
     pub fn write_current_position_to_register(&mut self, reg: i32, cp_offset: i32) {
        if cp_offset == 0 {
           self.masm_.StoreU64(self.current_input_offset(), self.register_location(reg), Register{});
        } else {
           self.masm_.mov(Register{}, Operand::new(cp_offset * self.char_size()));
           self.masm_.add(Register{}, self.current_input_offset(), Register{});
            self.masm_.StoreU64(Register{}, self.register_location(reg), Register{});
        }
    }
    pub fn succeed(&mut self) -> bool {
      self.masm_.b(&self.success_label_);
      return self.global();
    }

    fn load_current_character_unchecked(&mut self, cp_offset: i32, character_count: i32){

    }
     fn load_current_character(&mut self, cp_offset: i32, on_end_of_input: &mut Label, check_bounds: bool, characters: i32, eats_at_least: i32) {}

    fn check_preemption(&mut self) {}
    fn check_stack_limit(&mut self) {}

    fn register_location(&self, register_index: i3
