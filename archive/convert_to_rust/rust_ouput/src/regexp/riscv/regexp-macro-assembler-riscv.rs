// Converted from V8 C++ source files:
// Header: regexp-macro-assembler-riscv.h
// Implementation: regexp-macro-assembler-riscv.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod regexp_macro_assembler_riscv {
use std::rc::Rc;
use std::sync::Mutex;

use crate::base::strings::base;
use crate::codegen::assembler_arch::CodeDesc;
use crate::codegen::macro_assembler::NewAssemblerBuffer;
use crate::logging::log::RegExpCodeCreateEvent;
use crate::objects::objects_inl::ByteArray;
use crate::regexp::regexp_macro_assembler::{
    IrregexpImplementation, NativeRegExpMacroAssembler, RegExpFlags,
};
use crate::regexp::regexp_stack::RegExpStack;
use crate::snapshot::embedded::embedded_data_inl::EmbeddedData;
use crate::strings::unicode::String;

use crate::strings::unicode::uc16;
use crate::v8::internal::{Address, HeapObject, Isolate};
use crate::zone::zone::Zone;

pub struct RegExpMacroAssemblerRISCV {
    base: NativeRegExpMacroAssembler,
    masm_: Rc<Mutex<MacroAssembler>>,
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

impl RegExpMacroAssemblerRISCV {
    pub fn new(
        isolate: *mut Isolate,
        zone: *mut Zone,
        mode: Mode,
        registers_to_save: i32,
    ) -> RegExpMacroAssemblerRISCV {
        let mut masm_ = Rc::new(Mutex::new(MacroAssembler::new(
            isolate,
            CodeObjectRequired::kYes,
            NewAssemblerBuffer::new(1024),
        )));
        let mut entry_label_ = Label::new();
        let mut internal_failure_label_ = Label::new();
        {
            let mut masm = masm_.lock().unwrap();
            masm.jmp(&mut entry_label_);
            masm.bind(&mut internal_failure_label_);
            masm.li(A0, Operand::new(FAILURE));
            masm.Ret();
        }
        RegExpMacroAssemblerRISCV {
            base: NativeRegExpMacroAssembler::new(isolate, zone),
            masm_: masm_,
            no_root_array_scope_: NoRootArrayScope {},
            mode_: mode,
            num_registers_: registers_to_save,
            num_saved_registers_: registers_to_save,
            entry_label_: entry_label_,
            start_label_: Label::new(),
            success_label_: Label::new(),
            backtrack_label_: Label::new(),
            exit_label_: Label::new(),
            check_preempt_label_: Label::new(),
            stack_overflow_label_: Label::new(),
            internal_failure_label_: internal_failure_label_,
            fallback_label_: Label::new(),
        }
    }

    pub fn stack_limit_slack_slot_count(&self) -> i32 {
        RegExpStack::kStackLimitSlackSlotCount
    }

    pub fn advance_current_position(&mut self, by: i32) {
        if by != 0 {
            let mut masm = self.masm_.lock().unwrap();
            masm.AddWord(
                CURRENT_INPUT_OFFSET,
                CURRENT_INPUT_OFFSET,
                Operand::new(by * self.char_size()),
            );
        }
    }

    pub fn advance_register(&mut self, reg: i32, by: i32) {
        assert!(reg >= 0);
        assert!(self.num_registers_ > reg);
        if by != 0 {
            let mut masm = self.masm_.lock().unwrap();
            masm.LoadWord(A0, self.register_location(reg));
            masm.AddWord(A0, A0, Operand::new(by));
            masm.StoreWord(A0, self.register_location(reg));
        }
    }

    pub fn backtrack(&mut self) {
        self.check_preemption();
        if self.base.has_backtrack_limit() {
            let mut masm = self.masm_.lock().unwrap();
            let mut next = Label::new();
            masm.LoadWord(A0, MemOperand::new(FP, K_BACKTRACK_COUNT_OFFSET));
            masm.AddWord(A0, A0, Operand::new(1));
            masm.StoreWord(A0, MemOperand::new(FP, K_BACKTRACK_COUNT_OFFSET));
            masm.BranchShort(&mut next, Condition::ne, A0, Operand::new(self.base.backtrack_limit()));

            if self.base.can_fallback() {
                masm.jmp(&mut self.fallback_label_);
            } else {
                self.fail();
            }
            masm.bind(&mut next);
        }

        let mut masm = self.masm_.lock().unwrap();
        self.pop(A0);
        masm.AddWord(A0, A0, CODE_POINTER);
        masm.Jump(A0);
    }

    pub fn bind(&mut self, label: &mut Label) {
        let mut masm = self.masm_.lock().unwrap();
        masm.bind(label);
    }

    pub fn check_character(&mut self, c: u32, on_equal: &mut Label) {
        self.branch_or_backtrack(
            on_equal,
            Condition::eq,
            CURRENT_CHARACTER,
            Operand::new(c as i32),
        );
    }

    pub fn check_character_gt(&mut self, limit: uc16, on_greater: &mut Label) {
        self.branch_or_backtrack(
            on_greater,
            Condition::gt,
            CURRENT_CHARACTER,
            Operand::new(limit as i32),
        );
    }

    pub fn check_at_start(&mut self, cp_offset: i32, on_at_start: &mut Label) {
        let mut masm = self.masm_.lock().unwrap();
        masm.LoadWord(A1, MemOperand::new(FP, K_STRING_START_MINUS_ONE_OFFSET));
        masm.AddWord(
            A0,
            CURRENT_INPUT_OFFSET,
            Operand::new(-self.char_size() + cp_offset * self.char_size()),
        );
        self.branch_or_backtrack(on_at_start, Condition::eq, A0, Operand::new(A1));
    }

    pub fn check_not_at_start(&mut self, cp_offset: i32, on_not_at_start: &mut Label) {
        let mut masm = self.masm_.lock().unwrap();
        masm.LoadWord(A1, MemOperand::new(FP, K_STRING_START_MINUS_ONE_OFFSET));
        masm.AddWord(
            A0,
            CURRENT_INPUT_OFFSET,
            Operand::new(-self.char_size() + cp_offset * self.char_size()),
        );
        self.branch_or_backtrack(on_not_at_start, Condition::ne, A0, Operand::new(A1));
    }

    pub fn check_character_lt(&mut self, limit: uc16, on_less: &mut Label) {
        self.branch_or_backtrack(
            on_less,
            Condition::lt,
            CURRENT_CHARACTER,
            Operand::new(limit as i32),
        );
    }

    pub fn check_greedy_loop(&mut self, on_equal: &mut Label) {
        let mut masm = self.masm_.lock().unwrap();
        let mut backtrack_non_equal = Label::new();
        masm.Lw(
            A0,
            MemOperand::new(BACKTRACK_STACKPOINTER, 0),
        );
        masm.BranchShort(
            &mut backtrack_non_equal,
            Condition::ne,
            CURRENT_INPUT_OFFSET,
            Operand::new(A0),
        );
        masm.AddWord(
            BACKTRACK_STACKPOINTER,
            BACKTRACK_STACKPOINTER,
            Operand::new(K_INT_SIZE),
        );

        masm.bind(&mut backtrack_non_equal);
        self.branch_or_backtrack(
            on_equal,
            Condition::eq,
            CURRENT_INPUT_OFFSET,
            Operand::new(A0),
        );
    }

    fn call_is_character_in_range_array(&mut self, ranges: &ZoneList<CharacterRange>) {
        let num_arguments = 3;
        let mut masm = self.masm_.lock().unwrap();
        masm.PrepareCallCFunction(num_arguments, A0);
        masm.mv(A0, CURRENT_CHARACTER);
        masm.li(A1, Operand::new(self.base.get_or_add_range_array(ranges)));
        masm.li(A2, Operand::new(ExternalReference::isolate_address(self.base.isolate_)));
        {
            let mut scope = FrameScope::new(&mut masm, StackFrame::MANUAL);
            self.call_c_function_from_irregexp_code(
                ExternalReference::re_is_character_in_range_array(),
                num_arguments,
            );
        }
        masm.li(CODE_POINTER, Operand::new(masm.CodeObject()));
    }

    pub fn check_character_in_range_array(
        &mut self,
        ranges: &ZoneList<CharacterRange>,
        on_in_range: &mut Label,
    ) -> bool {
        self.call_is_character_in_range_array(ranges);
        self.branch_or_backtrack(on_in_range, Condition::ne, A0, Operand::new(ZERO_REG));
        return true;
    }

    pub fn check_character_not_in_range_array(
        &mut self,
        ranges: &ZoneList<CharacterRange>,
        on_not_in_range: &mut Label,
    ) -> bool {
        self.call_is_character_in_range_array(ranges);
        self.branch_or_backtrack(
            on_not_in_range,
            Condition::eq,
            A0,
            Operand::new(ZERO_REG),
        );
        return true;
    }

    pub fn check_not_back_reference_ignore_case(
        &mut self,
        start_reg: i32,
        read_backward: bool,
        unicode: bool,
        on_no_match: &mut Label,
    ) {
        let mut masm = self.masm_.lock().unwrap();
        let mut fallthrough = Label::new();
        masm.LoadWord(A0, self.register_location(start_reg));
        masm.LoadWord(A1, self.register_location(start_reg + 1));
        masm.SubWord(A1, A1, A0);

        masm.BranchShort(&mut fallthrough, Condition::eq, A1, Operand::new(ZERO_REG));

        if read_backward {
            masm.LoadWord(T1, MemOperand::new(FP, K_STRING_START_MINUS_ONE_OFFSET));
            masm.AddWord(T1, T1, A1);
            self.branch_or_backtrack(on_no_match, Condition::le, CURRENT_INPUT_OFFSET, Operand::new(T1));
        } else {
            masm.AddWord(T1, A1, CURRENT_INPUT_OFFSET);
            self.branch_or_backtrack(on_no_match, Condition::gt, T1, Operand::new(ZERO_REG));
        }

        if self.mode_ == Mode::LATIN1 {
            let mut success = Label::new();
            let mut fail = Label::new();
            let mut loop_check = Label::new();
            masm.AddWord(A0, A0, Operand::new(END_OF_INPUT_ADDRESS));
            masm.AddWord(A2, END_OF_INPUT_ADDRESS, Operand::new(CURRENT_INPUT_OFFSET));

            if read_backward {
                masm.SubWord(A2, A2, Operand::new(A1));
            }

            masm.AddWord(A1, A0, Operand::new(A1));

            let mut loop_ = Label::new();
            masm.bind(&mut loop_);
            masm.Lbu(A3, MemOperand::new(A0, 0));
            masm.addi(A0, A0, self.char_size());
            masm.Lbu(A4, MemOperand::new(A2, 0));
            masm.addi(A2, A2, self.char_size());
            masm.BranchShort(&mut loop_check, Condition::eq, A4, Operand::new(A3));
            masm.Or(A3, A3, Operand::new(0x20));
            masm.Or(A4, A4, Operand::new(0x20));
            masm.BranchShort(&mut fail, Condition::ne, A4, Operand::new(A3));
            masm.SubWord(A3, A3, Operand::new('a' as i32));
            masm.BranchShort(&mut loop_check, Condition::Uless_equal, A3, Operand::new(('z' as i32) - ('a' as i32)));
            masm.SubWord(A3, A3, Operand::new((224 as i32) - ('a' as i32)));
            masm.BranchShort(&mut fail, Condition::Ugreater, A3, Operand::new((254 as i32) - (224 as i32)));
            masm.BranchShort(&mut fail, Condition::eq, A3, Operand::new((247 as i32) - (224 as i32)));

            masm.bind(&mut loop_check);
            masm.Branch(&mut loop_, Condition::lt, A0, Operand::new(A1));
            masm.jmp(&mut success);

            masm.bind(&mut fail);
            self.go_to(on_no_match);

            masm.bind(&mut success);
            masm.SubWord(CURRENT_INPUT_OFFSET, A2, END_OF_INPUT_ADDRESS);

            if read_backward {
                masm.LoadWord(T1, self.register_location(start_reg));
                masm.LoadWord(A2, self.register_location(start_reg + 1));
                masm.AddWord(CURRENT_INPUT_OFFSET, CURRENT_INPUT_OFFSET, Operand::new(T1));
                masm.SubWord(CURRENT_INPUT_OFFSET, CURRENT_INPUT_OFFSET, Operand::new(A2));
            }
        } else {
            assert!(self.mode_ == Mode::UC16);
            let argument_count = 4;
            masm.PrepareCallCFunction(argument_count, A2);
            masm.AddWord(A0, A0, Operand::new(END_OF_INPUT_ADDRESS));
            masm.mv(A2, A1);
            masm.mv(S3, A1);
            masm.AddWord(A1, CURRENT_INPUT_OFFSET, Operand::new(END_OF_INPUT_ADDRESS));

            if read_backward {
                masm.SubWord(A1, A1, Operand::new(S3));
            }
            masm.li(A3, Operand::new(ExternalReference::isolate_address(self.base.isolate_)));
             {
                let mut scope = AllowExternalCallThatCantCauseGC::new(&mut masm);
                let function = if unicode {
                    ExternalReference::re_case_insensitive_compare_unicode()
                } else {
                    ExternalReference::re_case_insensitive_compare_non_unicode()
                };
                self.call_c_function_from_irregexp_code(function, argument_count);
            }
            self.branch_or_backtrack(on_no_match, Condition::eq, A0, Operand::new(ZERO_REG));

            if read_backward {
                masm.SubWord(CURRENT_INPUT_OFFSET, CURRENT_INPUT_OFFSET, Operand::new(S3));
            } else {
                masm.AddWord(CURRENT_INPUT_OFFSET, CURRENT_INPUT_OFFSET, Operand::new(S3));
            }
        }
        masm.bind(&mut fallthrough);
    }

    pub fn check_not_back_reference(
        &mut self,
        start_reg: i32,
        read_backward: bool,
        on_no_match: &mut Label,
    ) {
        let mut fallthrough = Label::new();
        let mut masm = self.masm_.lock().unwrap();

        masm.LoadWord(A0, self.register_location(start_reg));
        masm.LoadWord(A1, self.register_location(start_reg + 1));
        masm.SubWord(A1, A1, A0);

        masm.BranchShort(&mut fallthrough, Condition::eq, A1, Operand::new(ZERO_REG));

        if read_backward {
            masm.LoadWord(T1, MemOperand::new(FP, K_STRING_START_MINUS_ONE_OFFSET));
            masm.AddWord(T1, T1, A1);
            self.branch_or_backtrack(on_no_match, Condition::le, CURRENT_INPUT_OFFSET, Operand::new(T1));
        } else {
            masm.AddWord(T1, A1, CURRENT_INPUT_OFFSET);
            self.branch_or_backtrack(on_no_match, Condition::gt, T1, Operand::new(ZERO_REG));
        }
        masm.AddWord(A0, A0, Operand::new(END_OF_INPUT_ADDRESS));
        masm.AddWord(A2, END_OF_INPUT_ADDRESS, Operand::new(CURRENT_INPUT_OFFSET));
        if read_backward {
            masm.SubWord(A2, A2, Operand::new(A1));
        }
        masm.AddWord(A1, A1, Operand::new(A0));

        let mut loop_ = Label::new();
        masm.bind(&mut loop_);

        if self.mode_ == Mode::LATIN1 {
            masm.Lbu(A3, MemOperand::new(A0, 0));
            masm.addi(A0, A0, self.char_size());
            masm.Lbu(A4, MemOperand::new(A2, 0));
            masm.addi(A2, A2, self.char_size());
        } else {
            assert!(self.mode_ == Mode::UC16);
            masm.Lhu(A3, MemOperand::new(A0, 0));
            masm.addi(A0, A0, self.char_size());
            masm.Lhu(A4, MemOperand::new(A2, 0));
            masm.addi(A2, A2, self.char_size());
        }
        self.branch_or_backtrack(on_no_match, Condition::ne, A3, Operand::new(A4));
        masm.Branch(&mut loop_, Condition::lt, A0, Operand::new(A1));

        masm.SubWord(CURRENT_INPUT_OFFSET, A2, END_OF_INPUT_ADDRESS);
        if read_backward {
            masm.LoadWord(T1, self.register_location(start_reg));
            masm.LoadWord(A2, self.register_location(start_reg + 1));
            masm.AddWord(CURRENT_INPUT_OFFSET, CURRENT_INPUT_OFFSET, Operand::new(T1));
            masm.SubWord(CURRENT_INPUT_OFFSET, CURRENT_INPUT_OFFSET, Operand::new(A2));
        }
        masm.bind(&mut fallthrough);
    }

    pub fn check_not_character(&mut self, c: u32, on_not_equal: &mut Label) {
        self.branch_or_backtrack(
            on_not_equal,
            Condition::ne,
            CURRENT_CHARACTER,
            Operand::new(c as i32),
        );
    }

    pub fn check_character_after_and(
        &mut self,
        c: u32,
        mask: u32,
        on_equal: &mut Label,
    ) {
        let mut masm = self.masm_.lock().unwrap();
        masm.And(A0, CURRENT_CHARACTER, Operand::new(mask as i32));
        let rhs = if c == 0 {
            Operand::new(ZERO_REG)
        } else {
            Operand::new(c as i32)
        };
        self.branch_or_backtrack(on_equal, Condition::eq, A0, rhs);
    }

    pub fn check_not_character_after_and(
        &mut self,
        c: u32,
        mask: u32,
        on_not_equal: &mut Label,
    ) {
        let mut masm = self.masm_.lock().unwrap();
        masm.And(A0, CURRENT_CHARACTER, Operand::new(mask as i32));
        let rhs = if c == 0 {
            Operand::new(ZERO_REG)
        } else {
            Operand::new(c as i32)
        };
        self.branch_or_backtrack(on_not_equal, Condition::ne, A0, rhs);
    }

    pub fn check_not_character_after_minus_and(
        &mut self,
        c: uc16,
        minus: uc16,
        mask: uc16,
        on_not_equal: &mut Label,
    ) {
        assert!(minus < String::K_MAX_UTF16_CODE_UNIT);
        let mut masm = self.masm_.lock().unwrap();
        masm.SubWord(A0, CURRENT_CHARACTER, Operand::new(minus as i32));
        masm.And(A0, A0, Operand::new(mask as i32));
        self.branch_or_backtrack(on_not_equal, Condition::ne, A0, Operand::new(c as i32));
    }

    pub fn check_character_in_range(
        &mut self,
        from: uc16,
        to: uc16,
        on_in_range: &mut Label,
    ) {
        let mut masm = self.masm_.lock().unwrap();
        masm.SubWord(A0, CURRENT_CHARACTER, Operand::new(from as i32));
        self.branch_or_backtrack(
            on_in_range,
            Condition::Uless_equal,
            A0,
            Operand::new((to - from) as i32),
        );
    }

    pub fn check_character_not_in_range(
        &mut self,
        from: uc16,
        to: uc16,
        on_not_in_range: &mut Label,
    ) {
        let mut masm = self.masm_.lock().unwrap();
        masm.SubWord(A0, CURRENT_CHARACTER, Operand::new(from as i32));
        self.branch_or_backtrack(
            on_not_in_range,
            Condition::Ugreater,
            A0,
            Operand::new((to - from) as i32),
        );
    }

    pub fn check_bit_in_table(&mut self, table: Handle<ByteArray>, on_bit_set: &mut Label) {
        let mut masm = self.masm_.lock().unwrap();
        masm.li(A0, Operand::new(table));

        if self.mode_ != Mode::LATIN1 || K_TABLE_MASK != String::K_MAX_ONE_BYTE_CHAR_CODE {
            masm.And(A1, CURRENT_CHARACTER, Operand::new(K_TABLE_SIZE - 1));
            masm.AddWord(A0, A0, A1);
        } else {
            masm.AddWord(A0, A0, CURRENT_CHARACTER);
        }

        masm.Lbu(A0, FieldMemOperand::new(A0, ByteArray::K_DATA_START_OFFSET));
        self.branch_or_backtrack(on_bit_set, Condition::ne, A0, Operand::new(ZERO_REG));
    }

    pub fn skip_until_bit_in_table(
        &mut self,
        cp_offset: i32,
        table: Handle<ByteArray>,
        nibble_table: Handle<ByteArray>,
        advance_by: i32,
    ) {
        let mut cont = Label::new();
        let mut again = Label::new();
        self.bind(&mut again);
        self.load_current_character(cp_offset, &mut cont, true);
        self.check_bit_in_table(table, &mut cont);
        self.advance_current_position(advance_by);
        self.go_to(&mut again);
        self.bind(&mut cont);
    }

    pub fn check_special_class_ranges(
        &mut self,
        type_: StandardCharacterSet,
        on_no_match: &mut Label,
    ) -> bool {
        match type_ {
            StandardCharacterSet::K_WHITESPACE => {
                if self.mode_ == Mode::LATIN1 {
                    let mut success = Label::new();
                    let mut masm = self.masm_.lock().unwrap();

                    masm.BranchShort(&mut success, Condition::eq, CURRENT_CHARACTER, Operand::new(' ' as i32));

                    masm.SubWord(A0, CURRENT_CHARACTER, Operand::new('\t' as i32));
                    masm.BranchShort(&mut success, Condition::Uless_equal, A0, Operand::new(('\r' as i32) - ('\t' as i32)));
                    self.branch_or_backtrack(on_no_match, Condition::ne, A0, Operand::new((0x00A0 as i32) - ('\t' as i32)));
                    masm.bind(&mut success);
                    return true;
                }
                return false;
            }
            StandardCharacterSet::K_NOT_WHITESPACE => {
                return false;
            }
            StandardCharacterSet::K_DIGIT => {
                let mut masm = self.masm_.lock().unwrap();
                masm.SubWord(A0, CURRENT_CHARACTER, Operand::new('0' as i32));
                self.branch_or_backtrack(
                    on_no_match,
                    Condition::Ugreater,
                    A0,
                    Operand::new(('9' as i32) - ('0' as i32)),
                );
                return true;
            }
            StandardCharacterSet::K_NOT_DIGIT => {
                let mut masm = self.masm_.lock().unwrap();
                masm.SubWord(A0, CURRENT_CHARACTER, Operand::new('0' as i32));
                self.branch_or_backtrack(
                    on_no_match,
                    Condition::Uless_equal,
                    A0,
                    Operand::new(('9' as i32) - ('0' as i32)),
                );
                return true;
            }
            StandardCharacterSet::K_NOT_LINE_TERMINATOR => {
                let mut masm = self.masm_.lock().unwrap();
                masm.Xor(A0, CURRENT_CHARACTER, Operand::new(0x01));
                masm.SubWord(A0, A0, Operand::new(0x0B));
                self.branch_or_backtrack(
                    on_no_match,
                    Condition::Uless_equal,
                    A0,
                    Operand::new(0x0C - 0x0B),
                );
                if self.mode_ == Mode::UC16 {
                    masm.SubWord(A0, A0, Operand::new(0x2028 - 0x0B));
                    self.branch_or_backtrack(on_no_match, Condition::Uless_equal, A0, Operand::new(1));
                }
                return true;
            }
            StandardCharacterSet::K_LINE_TERMINATOR => {
                let mut masm = self.masm_.lock().unwrap();
                masm.Xor(A0, CURRENT_CHARACTER, Operand::new(0x01));
                masm.SubWord(A0, A0, Operand::new(0x0B));
                if self.mode_ == Mode::LATIN1 {
                    self.branch_or_backtrack(
                        on_no_match,
                        Condition::Ugreater,
                        A0,
                        Operand::new(0x0C - 0x0B),
                    );
                } else {
                    let mut done = Label::new();
                    masm.BranchShort(&mut done, Condition::Uless_equal, A0, Operand::new(0x0C - 0x0B));
                    masm.SubWord(A0, A0, Operand::new(0x2028 - 0x0B));
                    self.branch_or_backtrack(on_no_match, Condition::Ugreater, A0, Operand::new(1));
                    masm.bind(&mut done);
                }
                return true;
            }
            StandardCharacterSet::K_WORD => {
                let mut masm = self.masm_.lock().unwrap();
                if self.mode_ != Mode::LATIN1 {
                    self.branch_or_backtrack(on_no_match, Condition::Ugreater, CURRENT_CHARACTER, Operand::new('z' as i32));
                }
                let map = ExternalReference::re_word_character_map();
                masm.li(A0, Operand::new(map));
                masm.AddWord(A0, A0, CURRENT_CHARACTER);
                masm.Lbu(A0, MemOperand::new(A0, 0));
                self.branch_or_backtrack(on_no_match, Condition::eq, A0, Operand::new(ZERO_REG));
                return true;
            }
            StandardCharacterSet::K_NOT_WORD => {
                 let mut done = Label::new();
                let mut masm = self.masm_.lock().unwrap();
                if self.mode_ != Mode::LATIN1 {
                    masm.BranchShort(&mut done, Condition::Ugreater, CURRENT_CHARACTER, Operand::new('z' as i32));
                }
                let map = ExternalReference::re_word_character_map();
                masm.li(A0, Operand::new(map));
                masm.AddWord(A0, A0, CURRENT_CHARACTER);
                masm.Lbu(A0, MemOperand::new(A0, 0));
                self.branch_or_backtrack(on_no_match, Condition::ne, A0, Operand::new(ZERO_REG));
                if self.mode_ != Mode::LATIN1 {
                    masm.bind(&mut done);
                }
                return true;
            }
            StandardCharacterSet::K_EVERYTHING => {
                return true;
            }
            _ => return false,
        }
    }

    pub fn fail(&mut self) {
        let mut masm = self.masm_.lock().unwrap();
        masm.li(A0, Operand::new(FAILURE));
        masm.jmp(&mut self.exit_label_);
    }

    pub fn get_code(
        &mut self,
        source: DirectHandle<String>,
        flags: RegExpFlags,
    ) -> DirectHandle<HeapObject> {
        let mut return_a0 = Label::new();

        {
            let mut masm = self.masm_.lock().unwrap();
            if masm.has_exception() {
                masm.bind_to(&mut self.entry_label_, self.internal_failure_label_.pos());
            } else {
                masm.bind(&mut self.entry_label_);

                let mut scope = FrameScope::new(&mut masm, StackFrame::MANUAL);

                let registers_to_retain = RegList::new(vec![FP,S1, S2,S3,S4,S5,S6,S7, S8, S9,S10,S11]);

                let argument_registers = RegList::new(vec![A0, A1, A2, A3, A4, A5, A6, A7]);

                masm.MultiPush(RegList::new(vec![RA]).combine(&registers_to_retain));

                masm.AddWord(FP, SP, Operand::new(0));
                masm.li(
                    A0,
                    Operand::new(StackFrame::TypeToMarker(StackFrame::IRREGEXP)),
                );
                masm.push(A0);

                masm.MultiPush(argument_registers);

                masm.mv(A0, ZERO_REG);
                masm.push(A0);
                masm.push(A0);
                masm.push(A0);
                masm.push(A0);

                self.load_regexp_stack_pointer_from_memory(BACKTRACK_STACKPOINTER);
                self.push_regexp_base_pointer(BACKTRACK_STACKPOINTER, A1);
                let mut stack_limit_hit = Label::new();
                let mut stack_
