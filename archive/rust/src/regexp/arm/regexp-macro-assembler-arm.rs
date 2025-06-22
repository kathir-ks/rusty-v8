// This conversion is a best-effort attempt and might require adjustments
// to compile and function correctly.  Many details about the V8 engine are
// not available, requiring educated guesses and simplifications.

use std::mem;
// use std::os::raw::*;
// use std::ptr;

// Placeholder for V8's isolate.rs module
mod isolate {
    pub struct Isolate {}
}

// Placeholder for V8's zone.rs module
mod zone {
    pub struct Zone {}
}

// Placeholder for V8's assembler_buffer.rs module
mod assembler_buffer {
    pub struct AssemblerBuffer {}
}

// Placeholder for V8's macro_assembler.rs module
mod macro_assembler {
    use super::assembler_buffer::AssemblerBuffer;
    use super::isolate::Isolate;
    use super::objects::Code;

    pub struct MacroAssembler {}

    impl MacroAssembler {
        pub fn new(_isolate: &mut Isolate, _code_object_required: CodeObjectRequired, _buffer: AssemblerBuffer) -> Self {
            MacroAssembler {}
        }

        pub fn aborted_code_generation(&mut self) {}

        pub fn get_code(&mut self, _isolate: &mut Isolate, _code_desc: &mut CodeDesc) {}

        pub fn code_object(&self) -> Code {
            Code{}
        }

        pub fn options(&self) -> AssemblerOptions {
            AssemblerOptions {}
        }
    }

    #[derive(PartialEq)]
    pub enum CodeObjectRequired {
        kYes,
        kNo,
    }

    pub struct CodeDesc {}
    pub struct AssemblerOptions {}
}

// Placeholder for V8's factory.rs module
mod factory {
    use super::isolate::Isolate;
    use super::objects::{Code, AbstractCode};

    pub struct CodeBuilder {}

    impl CodeBuilder {
        pub fn new(_isolate: &mut Isolate, _code_desc: crate::macro_assembler::CodeDesc, _code_kind: CodeKind) -> Self {
            CodeBuilder {}
        }

        pub fn set_self_reference(self, _code: Code) -> Self {
            self
        }

        pub fn set_empty_source_position_table(self) -> Self {
            self
        }

        pub fn build(self) -> Code {
            Code{}
        }
    }

    pub fn code_builder(_isolate: &mut Isolate, _code_desc: crate::macro_assembler::CodeDesc, _code_kind: CodeKind) -> CodeBuilder {
        CodeBuilder {}
    }

    pub fn code_builder_return_handle(_isolate: &mut Isolate, _code_desc: crate::macro_assembler::CodeDesc, _code_kind: CodeKind) -> Code {
        Code{}
    }
}

// Placeholder for V8's logging.rs module
mod logging {
    use super::objects::AbstractCode;
    use super::string::String;
    use super::regexp::RegExpFlags;
    use super::isolate::Isolate;

    pub fn regexp_code_create_event(_isolate: &Isolate, _abstract_code: AbstractCode, _source: String, _flags: RegExpFlags) {}
}

// Placeholder for V8's code.rs module
mod objects {
    pub struct Code {}
    pub struct AbstractCode {}
    impl AbstractCode {

    }
}

mod string {
    pub struct String {}
}

mod regexp {
    pub struct RegExpFlags {}
}

// Placeholder for V8's regexp_stack.rs module
mod regexp_stack {
    pub struct RegExpStack {}
}

// Placeholder for V8's embedded_data.rs module
mod embedded_data {
    pub struct EmbeddedData {}
}

// Placeholder for V8's arm assembler.rs module
mod arm_assembler {
    pub struct AssemblerARM {}
}

// Placeholder for V8's reg_exp_macro_assembler
mod reg_exp_macro_assembler {
    use super::isolate::Isolate;
    use super::zone::Zone;
    use super::regexp::RegExpFlags;
    use super::objects::HeapObject;
    use super::string::String;

    pub enum IrregexpImplementation {
        kARMImplementation,
    }

    pub struct NativeRegExpMacroAssembler {}

    impl NativeRegExpMacroAssembler {
        pub fn check_stack_guard_state(_isolate: &mut Isolate, _start_index: i32, _direct_call: i32, _return_address: *mut usize, _code: HeapObject, _input_string: *mut usize, _input_start: *const u8, _input_end: *const u8, _extra_space: usize) -> i32 {
            0
        }

        pub fn implementation() -> IrregexpImplementation {
            IrregexpImplementation::kARMImplementation
        }
    }

    pub trait RegExpMacroAssembler {
        fn get_code(&mut self, source: &String, flags: RegExpFlags) -> HeapObject;
        fn implementation() -> IrregexpImplementation;
    }
}

// Define constants for register names and offsets, replicating the C++ code.
const FAILURE: i32 = -1;
const SUCCESS: i32 = 0;
const EXCEPTION: i32 = -2;
const FALLBACK_TO_EXPERIMENTAL: i32 = -3;

const kSystemPointerSize: i32 = 4; // Assuming 32-bit ARM
const kHeapObjectTag: i32 = 0;
const kFrameTypeOffset: i32 = 52;
const kFramePointerOffset: i32 = 28;
const kInputEndOffset: i32 = 48;
const kInputStartOffset: i32 = 44;
const kStartIndexOffset: i32 = 40;
const kInputStringOffset: i32 = 36;
const kSuccessfulCapturesOffset: i32 = 32;
const kStringStartMinusOneOffset: i32 = 28;
const kBacktrackCountOffset: i32 = 24;
const kRegExpStackBasePointerOffset: i32 = 20;
const kNumOutputRegistersOffset: i32 = 40;
const kRegisterOutputOffset: i32 = 36;
const kRegisterZeroOffset: i32 = -4;

const kTableSize: i32 = 256;
const kTableMask: i32 = 255; // Assuming LATIN1

// Define enum for mode
#[derive(PartialEq, Copy, Clone)]
enum Mode {
    LATIN1,
    UC16,
}

#[allow(dead_code)]
mod arm {
    use super::*;
    use super::macro_assembler::{MacroAssembler, CodeDesc};
    use super::isolate::Isolate;
    use super::zone::Zone;
    use super::regexp::RegExpFlags;
    use super::objects::{HeapObject, Code};
    use super::string::String;
    use super::reg_exp_macro_assembler::{RegExpMacroAssembler, IrregexpImplementation};
    use std::rc::Rc;

    const V8_TARGET_ARCH_ARM: bool = true; // Assume ARM target

    pub struct RegExpMacroAssemblerARM {
        isolate: *mut Isolate,
        zone: *mut Zone,
        masm_: Rc<MacroAssembler>, // Using Rc for shared ownership
        no_root_array_scope_: Rc<MacroAssembler>, // Using Rc for shared ownership
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
        global_: bool,
        backtrack_limit_: i32,
        has_backtrack_limit_: bool,
        global_with_zero_length_check_: bool,
        global_unicode_: bool,
    }

    // Dummy struct, replace with actual label type when available
    #[derive(Default)]
    pub struct Label {
        linked: bool,
    }

    impl Label {
        pub fn is_linked(&self) -> bool {
            self.linked
        }

        pub fn unuse(&mut self) {
            self.linked = false;
        }
    }

    impl RegExpMacroAssemblerARM {
        pub const kRegExpCodeSize: i32 = 2048;

        pub fn new(
            isolate: *mut Isolate,
            zone: *mut Zone,
            mode: Mode,
            registers_to_save: i32,
        ) -> Self {
            assert_eq!(0, registers_to_save % 2);

            let mut assembler = MacroAssembler::new(unsafe { &mut *isolate }, macro_assembler::CodeObjectRequired::kYes, assembler_buffer::AssemblerBuffer {});
            let masm_rc = Rc::new(assembler);
            let masm_rc_clone = Rc::clone(&masm_rc);

            let mut result = RegExpMacroAssemblerARM {
                isolate,
                zone,
                masm_: masm_rc,
                no_root_array_scope_: masm_rc_clone,
                mode_: mode,
                num_registers_: registers_to_save,
                num_saved_registers_: registers_to_save,
                entry_label_: Label::default(),
                start_label_: Label::default(),
                success_label_: Label::default(),
                backtrack_label_: Label::default(),
                exit_label_: Label::default(),
                check_preempt_label_: Label::default(),
                stack_overflow_label_: Label::default(),
                fallback_label_: Label::default(),
                global_: false,
                backtrack_limit_: 0,
                has_backtrack_limit_: false,
                global_with_zero_length_check_: false,
                global_unicode_: false,
            };

            // Here would be where the assembler instructions would be called,
            // but they have been removed for brevity.
            // __ jmp(&entry_label_);   // We'll write the entry code later.
            // __ bind(&start_label_);  // And then continue from here.

            result
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

        pub fn stack_limit_slack_slot_count() -> i32 {
            // RegExpStack::kStackLimitSlackSlotCount
            2048 // Placeholder value
        }

        pub fn advance_current_position(&mut self, by: i32) {
            if by != 0 {
                // __ add(current_input_offset(),
                //    current_input_offset(), Operand(by * char_size()));
            }
        }

        pub fn advance_register(&mut self, reg: i32, by: i32) {
            assert!(reg >= 0);
            assert!(self.num_registers_ > reg);
            if by != 0 {
                // __ ldr(r0, register_location(reg));
                // __ add(r0, r0, Operand(by));
                // __ str(r0, register_location(reg));
            }
        }

        pub fn backtrack(&mut self) {
            self.check_preemption();
            if self.has_backtrack_limit_() {
                // Label next;
                // __ ldr(r0, MemOperand(frame_pointer(), kBacktrackCountOffset));
                // __ add(r0, r0, Operand(1));
                // __ str(r0, MemOperand(frame_pointer(), kBacktrackCountOffset));
                // __ cmp(r0, Operand(backtrack_limit()));
                // __ b(ne, &next);

                // Backtrack limit exceeded.
                if self.can_fallback() {
                    // __ jmp(&fallback_label_);
                } else {
                    // Can't fallback, so we treat it as a failed match.
                    self.fail();
                }

                // __ bind(&next);
            }
            // Pop InstructionStream offset from backtrack stack, add InstructionStream
            // and jump to location.
            // Pop(r0);
            // __ add(pc, r0, Operand(code_pointer()));
        }

        pub fn bind(&mut self, label: &mut Label) {
            // __ bind(label);
            label.linked = true;
        }

        pub fn check_character(&mut self, c: u32, on_equal: &mut Label) {
            // __ cmp(current_character(), Operand(c));
            self.branch_or_backtrack(Condition::eq, on_equal);
        }

        pub fn check_character_gt(&mut self, limit: u16, on_greater: &mut Label) {
            // __ cmp(current_character(), Operand(limit));
            self.branch_or_backtrack(Condition::gt, on_greater);
        }

        pub fn check_at_start(&mut self, cp_offset: i32, on_at_start: &mut Label) {
            // __ ldr(r1, MemOperand(frame_pointer(), kStringStartMinusOneOffset));
            // __ add(r0, current_input_offset(),
            //    Operand(-char_size() + cp_offset * char_size()));
            // __ cmp(r0, r1);
            self.branch_or_backtrack(Condition::eq, on_at_start);
        }

        pub fn check_not_at_start(&mut self, cp_offset: i32, on_not_at_start: &mut Label) {
            // __ ldr(r1, MemOperand(frame_pointer(), kStringStartMinusOneOffset));
            // __ add(r0, current_input_offset(),
            //    Operand(-char_size() + cp_offset * char_size()));
            // __ cmp(r0, r1);
            self.branch_or_backtrack(Condition::ne, on_not_at_start);
        }

        pub fn check_character_lt(&mut self, limit: u16, on_less: &mut Label) {
            // __ cmp(current_character(), Operand(limit));
            self.branch_or_backtrack(Condition::lt, on_less);
        }

        pub fn check_greedy_loop(&mut self, on_equal: &mut Label) {
            // __ ldr(r0, MemOperand(backtrack_stackpointer(), 0));
            // __ cmp(current_input_offset(), r0);
            // __ add(backtrack_stackpointer(), backtrack_stackpointer(),
            //    Operand(kSystemPointerSize), LeaveCC, eq);
            self.branch_or_backtrack(Condition::eq, on_equal);
        }

        pub fn check_not_back_reference_ignore_case(
            &mut self,
            start_reg: i32,
            read_backward: bool,
            unicode: bool,
            on_no_match: &mut Label,
        ) {
            // Label fallthrough;
            // __ ldr(r0, register_location(start_reg));  // Index of start of capture
            // __ ldr(r1, register_location(start_reg + 1));  // Index of end of capture
            // __ sub(r1, r1, r0, SetCC);  // Length of capture.

            // // At this point, the capture registers are either both set or both cleared.
            // // If the capture length is zero, then the capture is either empty or cleared.
            // // Fall through in both cases.
            // __ b(eq, &fallthrough);

            // // Check that there are enough characters left in the input.
            // if read_backward {
            //     __ ldr(r3, MemOperand(frame_pointer(), kStringStartMinusOneOffset));
            //     __ add(r3, r3, r1);
            //     __ cmp(current_input_offset(), r3);
            //     BranchOrBacktrack(le, on_no_match);
            // } else {
            //     __ cmn(r1, Operand(current_input_offset()));
            //     BranchOrBacktrack(gt, on_no_match);
            // }

            // if mode_ == LATIN1 {
            //     Label success;
            //     Label fail;
            //     Label loop_check;

            //     // r0 - offset of start of capture
            //     // r1 - length of capture
            //     __ add(r0, r0, end_of_input_address());
            //     __ add(r2, end_of_input_address(), current_input_offset());
            //     if (read_backward) {
            //         __ sub(r2, r2, r1);  // Offset by length when matching backwards.
            //     }
            //     __ add(r1, r0, r1);

            //     // r0 - Address of start of capture.
            //     // r1 - Address of end of capture
            //     // r2 - Address of current input position.

            //     Label loop;
            //     __ bind(&loop);
            //     __ ldrb(r3, MemOperand(r0, char_size(), PostIndex));
            //     __ ldrb(r4, MemOperand(r2, char_size(), PostIndex));
            //     __ cmp(r4, r3);
            //     __ b(eq, &loop_check);

            //     // Mismatch, try case-insensitive match (converting letters to lower-case).
            //     __ orr(r3, r3, Operand(0x20));  // Convert capture character to lower-case.
            //     __ orr(r4, r4, Operand(0x20));  // Also convert input character.
            //     __ cmp(r4, r3);
            //     __ b(ne, &fail);
            //     __ sub(r3, r3, Operand('a'));
            //     __ cmp(r3, Operand('z' - 'a'));  // Is r3 a lowercase letter?
            //     __ b(ls, &loop_check);  // In range 'a'-'z'.
            //     // Latin-1: Check for values in range [224,254] but not 247.
            //     __ sub(r3, r3, Operand(224 - 'a'));
            //     __ cmp(r3, Operand(254 - 224));
            //     __ b(hi, &fail);  // Weren't Latin-1 letters.
            //     __ cmp(r3, Operand(247 - 224));  // Check for 247.
            //     __ b(eq, &fail);

            //     __ bind(&loop_check);
            //     __ cmp(r0, r1);
            //     __ b(lt, &loop);
            //     __ jmp(&success);

            //     __ bind(&fail);
            //     BranchOrBacktrack(al, on_no_match);

            //     __ bind(&success);
            //     // Compute new value of character position after the matched part.
            //     __ sub(current_input_offset(), r2, end_of_input_address());
            //     if (read_backward) {
            //         __ ldr(r0, register_location(start_reg));  // Index of start of capture
            //         __ ldr(r1, register_location(start_reg + 1));  // Index of end of capture
            //         __ add(current_input_offset(), current_input_offset(), r0);
            //         __ sub(current_input_offset(), current_input_offset(), r1);
            //     }
            // } else {
            //     DCHECK(mode_ == UC16);
            //     int argument_count = 4;
            //     __ PrepareCallCFunction(argument_count);

            //     // r0 - offset of start of capture
            //     // r1 - length of capture

            //     // Put arguments into arguments registers.
            //     // Parameters are
            //     //   r0: Address byte_offset1 - Address captured substring's start.
            //     //   r1: Address byte_offset2 - Address of current character position.
            //     //   r2: size_t byte_length - length of capture in bytes(!)
            //     //   r3: Isolate* isolate.

            //     // Address of start of capture.
            //     __ add(r0, r0, Operand(end_of_input_address()));
            //     // Length of capture.
            //     __ mov(r2, Operand(r1));
            //     // Save length in callee-save register for use on return.
            //     __ mov(r4, Operand(r1));
            //     // Address of current input position.
            //     __ add(r1, current_input_offset(), end_of_input_address());
            //     if (read_backward) {
            //         __ sub(r1, r1, r4);
            //     }
            //     // Isolate.
            //     __ mov(r3, Operand(ExternalReference::isolate_address(isolate())));

            //     {
            //         AllowExternalCallThatCantCauseGC scope(masm_.get());
            //         ExternalReference function =
            //             unicode
            //                 ? ExternalReference::re_case_insensitive_compare_unicode()
            //                 : ExternalReference::re_case_insensitive_compare_non_unicode();
            //         CallCFunctionFromIrregexpCode(function, argument_count);
            //     }

            //     // Check if function returned non-zero for success or zero for failure.
            //     __ cmp(r0, Operand::Zero());
            //     BranchOrBacktrack(eq, on_no_match);

            //     // On success, advance position by length of capture.
            //     if (read_backward) {
            //         __ sub(current_input_offset(), current_input_offset(), r4);
            //     } else {
            //         __ add(current_input_offset(), current_input_offset(), r4);
            //     }
            // }

            // __ bind(&fallthrough);
            todo!()
        }

        pub fn check_not_back_reference(
            &mut self,
            start_reg: i32,
            read_backward: bool,
            on_no_match: &mut Label,
        ) {
            // Label fallthrough;

            // // Find length of back-referenced capture.
            // __ ldr(r0, register_location(start_reg));
            // __ ldr(r1, register_location(start_reg + 1));
            // __ sub(r1, r1, r0, SetCC);  // Length to check.

            // // At this point, the capture registers are either both set or both cleared.
            // // If the capture length is zero, then the capture is either empty or cleared.
            // // Fall through in both cases.
            // __ b(eq, &fallthrough);

            // // Check that there are enough characters left in the input.
            // if read_backward {
            //     __ ldr(r3, MemOperand(frame_pointer(), kStringStartMinusOneOffset));
            //     __ add(r3, r3, r1);
            //     __ cmp(current_input_offset(), r3);
            //     BranchOrBacktrack(le, on_no_match);
            // } else {
            //     __ cmn(r1, Operand(current_input_offset()));
            //     BranchOrBacktrack(gt, on_no_match);
            // }

            // // r0 - offset of start of capture
            // // r1 - length of capture
            // __ add(r0, r0, end_of_input_address());
            // __ add(r2, end_of_input_address(), current_input_offset());
            // if (read_backward) {
            //     __ sub(r2, r2, r1);  // Offset by length when matching backwards.
            // }
            // __ add(r1, r0, r1);

            // Label loop;
            // __ bind(&loop);
            // if (mode_ == LATIN1) {
            //     __ ldrb(r3, MemOperand(r0, char_size(), PostIndex));
            //     __ ldrb(r4, MemOperand(r2, char_size(), PostIndex));
            // } else {
            //     DCHECK(mode_ == UC16);
            //     __ ldrh(r3, MemOperand(r0, char_size(), PostIndex));
            //     __ ldrh(r4, MemOperand(r2, char_size(), PostIndex));
            // }
            // __ cmp(r3, r4);
            // BranchOrBacktrack(ne, on_no_match);
            // __ cmp(r0, r1);
            // __ b(lt, &loop);

            // // Move current character position to position after match.
            // __ sub(current_input_offset(), r2, end_of_input_address());
            // if (read_backward) {
            //     __ ldr(r0, register_location(start_reg));      // Index of start of capture
            //     __ ldr(r1, register_location(start_reg + 1));  // Index of end of capture
            //     __ add(current_input_offset(), current_input_offset(), r0);
            //     __ sub(current_input_offset(), current_input_offset(), r1);
            // }

            // __ bind(&fallthrough);
            todo!()
        }

        pub fn check_not_character(&mut self, c: u32, on_not_equal: &mut Label) {
            // __ cmp(current_character(), Operand(c));
            self.branch_or_backtrack(Condition::ne, on_not_equal);
        }

        pub fn check_character_after_and(
            &mut self,
            c: u32,
            mask: u32,
            on_equal: &mut Label,
        ) {
            // if (c == 0) {
            //     __ tst(current_character(), Operand(mask));
            // } else {
            //     __ and_(r0, current_character(), Operand(mask));
            //     __ cmp(r0, Operand(c));
            // }
            self.branch_or_backtrack(Condition::eq, on_equal);
        }

        pub fn check_not_character_after_and(
            &mut self,
            c: u32,
            mask: u32,
            on_not_equal: &mut Label,
        ) {
            // if (c == 0) {
            //     __ tst(current_character(), Operand(mask));
            // } else {
            //     __ and_(r0, current_character(), Operand(mask));
            //     __ cmp(r0, Operand(c));
            // }
            self.branch_or_backtrack(Condition::ne, on_not_equal);
        }

        pub fn check_not_character_after_minus_and(
            &mut self,
            c: u16,
            minus: u16,
            mask: u16,
            on_not_equal: &mut Label,
        ) {
            assert!(String::kMaxUtf16CodeUnit > minus);
            // __ sub(r0, current_character(), Operand(minus));
            // __ and_(r0, r0, Operand(mask));
            // __ cmp(r0, Operand(c));
            self.branch_or_backtrack(Condition::ne, on_not_equal);
        }

        pub fn check_character_in_range(
            &mut self,
            from: u16,
            to: u16,
            on_in_range: &mut Label,
        ) {
            // __ sub(r0, current_character(), Operand(from));
            // __ cmp(r0, Operand(to - from));
            self.branch_or_backtrack(Condition::ls, on_in_range); // Unsigned lower-or-same condition.
        }

        pub fn check_character_not_in_range(
            &mut self,
            from: u16,
            to: u16,
            on_not_in_range: &mut Label,
        ) {
            // __ sub(r0, current_character(), Operand(from));
            // __ cmp(r0, Operand(to - from));
            self.branch_or_backtrack(Condition::hi, on_not_in_range); // Unsigned higher condition.
        }

        pub fn call_is_character_in_range_array(&mut self, ranges: *const ZoneList<CharacterRange>) {
            // static const int kNumArguments = 2;
            // __ PrepareCallCFunction(kNumArguments);

            // __ mov(r0, current_character());
            // __ mov(r1, Operand(GetOrAddRangeArray(ranges)));

            // {
            //     // We have a frame (set up in GetCode), but the assembler doesn't know.
            //     FrameScope scope(masm_.get(), StackFrame::MANUAL);
            //     CallCFunctionFromIrregexpCode(
            //         ExternalReference::re_is_character_in_range_array(), kNumArguments);
            // }

            // __ mov(code_pointer(), Operand(masm_->CodeObject()));
            todo!()
        }

        pub fn check_character_in_range_array(
            &mut self,
            ranges: *const ZoneList<CharacterRange>,
            on_in_range: &mut Label,
        ) -> bool {
            self.call_is_character_in_range_array(ranges);
            // __ cmp(r0, Operand::Zero());
            self.branch_or_backtrack(Condition::ne, on_in_range);
            true
        }

        pub fn check_character_not_in_range_array(
            &mut self,
            ranges: *const ZoneList<CharacterRange>,
            on_not_in_range: &mut Label,
        ) -> bool {
            self.call_is_character_in_range_array(ranges);
            // __ cmp(r0, Operand::Zero());
            self.branch_or_backtrack(Condition::eq, on_not_in_range);
            true
        }

        pub fn check_bit_in_table(&mut self, table: *const ByteArray, on_bit_set: &mut Label) {
            // __ mov(r0, Operand(table));
            // if (mode_ != LATIN1 || kTableMask != String::kMaxOneByteCharCode) {
            //     __ and_(r1, current_character(), Operand(kTableSize - 1));
            //     __ add(r1, r1, Operand(OFFSET_OF_DATA_START(ByteArray) - kHeapObjectTag));
            // } else {
            //     __ add(r1, current_character(),
            //         Operand(OFFSET_OF_DATA_START(ByteArray) - kHeapObjectTag));
            // }
            // __ ldrb(r0, MemOperand(r0, r1));
            // __ cmp(r0, Operand::Zero());
            self.branch_or_backtrack(Condition::ne, on_bit_set);
        }

        pub fn skip_until_bit_in_table(
            &mut self,
            cp_offset: i32,
            table: *const ByteArray,
            nibble_table: *const ByteArray,
            advance_by: i32,
        ) {
            // // TODO(pthier): Optimize. Table can be loaded outside of the loop.
            // Label cont, again;
            // Bind(&again);
            // LoadCurrentCharacter(cp_offset, &cont, true);
            // CheckBitInTable(table, &cont);
            // AdvanceCurrentPosition(advance_by);
            // GoTo(&again);
            // Bind(&cont);
            todo!()
        }

        pub fn check_special_class_ranges(
            &mut self,
            type_: StandardCharacterSet,
            on_no_match: &mut Label,
        ) -> bool {
            // // Range checks (c in min..max) are generally implemented by an unsigned
            // // (c - min) <= (max - min) check
            // // TODO(jgruber): No custom implementation (yet): s(UC16), S(UC16).
            match type_ {
                StandardCharacterSet::kWhitespace => {
                    // // Match space-characters.
                    if self.mode_ == Mode::LATIN1 {
                        // // One byte space characters are '\t'..'\r', ' ' and \u00a0.
                        // Label success;
                        // __ cmp(current_character(), Operand(' '));
                        // __ b(eq, &success);
                        // // Check range 0x09..0x0D.
                        // __ sub(r0, current_character(), Operand('\t'));
                        // __ cmp(r0, Operand('\r' - '\t'));
                        // __ b(ls, &success);
                        // // \u00a0 (NBSP).
                        // __ cmp(r0, Operand(0x00A0 - '\t'));
                        // BranchOrBacktrack(ne, on_no_match);
                        // __ bind(&success);
                        return true;
                    }
                    false
                }
                StandardCharacterSet::kNotWhitespace => {
                    // // The emitted code for generic character classes is good enough.
                    false
                }
                StandardCharacterSet::kDigit => {
