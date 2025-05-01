#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::os::raw::c_int;
//use std::ptr;

//use crate::assembler::Assembler;
//use crate::macro_assembler::MacroAssembler;
//use crate::objects::Object;
//use crate::regexp::regexp_stack::RegExpStack;
//use crate::strings::unicode::uc16;
//use crate::utils::PtrComprCageBase;

mod codegen {
    pub mod assembler_inl;
    pub mod macro_assembler;
}

mod logging {
    pub mod log;
}

mod objects {
    pub mod objects_inl;
}

mod regexp {
    pub mod regexp_macro_assembler;
    pub mod regexp_stack;
    pub mod ia32 {
        pub mod regexp_macro_assembler_ia32;
    }
}

mod strings {
    pub mod unicode;
}

const V8_TARGET_ARCH_IA32: bool = true;

#[cfg(target_arch = "x86")]
mod regexp_macro_assembler_ia32_rust {
    use super::*;
    use std::mem;
    use std::os::raw::c_int;
    //use crate::assembler::Assembler;
    //use crate::macro_assembler::MacroAssembler;
    //use crate::objects::Object;
    //use crate::regexp::regexp_macro_assembler::{NativeRegExpMacroAssembler, RegExpMacroAssembler};
    //use crate::regexp::regexp_stack::RegExpStack;
    //use crate::strings::unicode::uc16;
    //use crate::utils::PtrComprCageBase;
    use std::ptr::null_mut;

    // Placeholder types, replace with actual V8 types.
    pub struct Isolate {}
    pub struct Zone {}
    pub struct MacroAssembler {}
    pub struct RegExpFlags {}
    pub struct String {}
    pub struct HeapObject {}
    pub struct CodeDesc {}
    pub struct Handle<T> {}
    pub struct ByteArray {}
    pub struct ZoneList<T> {}
    pub struct CharacterRange {}
    pub enum StandardCharacterSet {
        kWhitespace,
        kNotWhitespace,
        kDigit,
        kNotDigit,
        kLineTerminator,
        kNotLineTerminator,
        kWord,
        kNotWord,
        kEverything,
    }

    impl Isolate {
        fn current() -> Self {
            Isolate {}
        }
    }

    impl<T> Handle<T> {
        fn new() -> Self {
            Handle {}
        }
    }
    impl ByteArray{
        
    }
    impl ZoneList<CharacterRange> {
        
    }

    //enum Branch {
    //    Backtrack,
    //    Label(Label),
    //}

    #[derive(PartialEq, Eq)]
    enum Mode {
        LATIN1,
        UC16,
    }

    const kSystemPointerSize: i32 = 4; // Assuming 32-bit architecture
    const kRegExpCodeSize: i32 = 1024; // Example size
    const FAILURE: i32 = 0;
    const SUCCESS: i32 = 1;
    const EXCEPTION: i32 = -1;
    const FALLBACK_TO_EXPERIMENTAL: i32 = -2;
    const kTableMask: i32 = 255;
    const kTableSize: i32 = 256;
    const kNumCalleeSaveRegisters: i32 = 3;

    const kFrameTypeOffset: i32 = -1 * kSystemPointerSize;
    const kLastCalleeSaveRegisterOffset: i32 = -4 * kSystemPointerSize;
    const kBackupEbxOffset: i32 = -4 * kSystemPointerSize;
    const kBackupEdiOffset: i32 = -3 * kSystemPointerSize;
    const kBackupEsiOffset: i32 = -2 * kSystemPointerSize;

    const kSuccessfulCapturesOffset: i32 = kLastCalleeSaveRegisterOffset - kSystemPointerSize;
    const kStringStartMinusOneOffset: i32 = kSuccessfulCapturesOffset - kSystemPointerSize;
    const kBacktrackCountOffset: i32 = kStringStartMinusOneOffset - kSystemPointerSize;
    const kRegExpStackBasePointerOffset: i32 = kBacktrackCountOffset - kSystemPointerSize;

    const kRegisterOutputOffset: i32 = 5 * kSystemPointerSize;
    const kNumOutputRegistersOffset: i32 = 4 * kSystemPointerSize;
    const kDirectCallOffset: i32 = 2 * kSystemPointerSize;
    const kStartIndexOffset: i32 = 7 * kSystemPointerSize;
    const kInputEndOffset: i32 = 6 * kSystemPointerSize;
    const kInputStartOffset: i32 = 7 * kSystemPointerSize;
    const kInputStringOffset: i32 = 8 * kSystemPointerSize;

    const kRegisterZeroOffset: i32 = -4 * kSystemPointerSize;

    //const kBackTrackStackpointer: Register = ecx; // example. need the proper enum
    fn backtrack_stackpointer() -> Register {
        Register::ecx
    }
    
    fn current_character() -> Register {
        Register::edx
    }
    
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    enum Register {
        eax,
        ebx,
        ecx,
        edx,
        esi,
        edi,
        ebp,
        esp,
    }

    impl Register {
        // Convert the Register enum to a string representation for debugging
        fn as_str(&self) -> &'static str {
            match self {
                Register::eax => "eax",
                Register::ebx => "ebx",
                Register::ecx => "ecx",
                Register::edx => "edx",
                Register::esi => "esi",
                Register::edi => "edi",
                Register::ebp => "ebp",
                Register::esp => "esp",
            }
        }
    }

    // MacroAssembler Interface:
    impl MacroAssembler {
        fn CodeObject(&self) -> *mut std::ffi::c_void {
            null_mut() // Replace with actual implementation
        }
    }
    
    struct Immediate {
        value: i32,
    }

    impl Immediate {
        fn new(value: i32) -> Self {
            Immediate { value }
        }
        
        fn CodeRelativeOffset(label: *const Label) -> Self{
            Immediate {value: 0} //dummy
        }
    }

    struct Operand {
        base: Register,
        index: Option<(Register, i32)>,
        displacement: i32,
    }

    impl Operand {
        fn new(base: Register, displacement: i32) -> Self {
            Operand { base, index: None, displacement }
        }

        fn with_index(base: Register, index: Register, scale: i32, displacement: i32) -> Self {
            Operand {
                base,
                index: Some((index, scale)),
                displacement,
            }
        }
    }

    struct StaticVariable {
        address: usize,
    }

    impl StaticVariable {
        fn new(address: usize) -> Self {
            StaticVariable { address }
        }
    }

    enum Condition {
        equal,
        not_equal,
        less,
        greater,
        less_equal,
        greater_equal,
        zero,
        not_zero,
        above,
        below,
    }

    struct Label {
        // Placeholder, add necessary fields.
        linked: bool,
    }

    impl Label {
        fn new() -> Self {
            Label { linked: false }
        }

        fn is_linked(&self) -> bool {
            self.linked
        }

        fn Unuse(&mut self) {
            self.linked = false;
        }
    }

    struct FrameScope {
        // Placeholder, add necessary fields.
    }

    impl FrameScope {
        fn new() -> Self {
            FrameScope {}
        }
    }

    enum StackFrame {
        MANUAL,
        IRREGEXP,
    }

    enum CodeKind {
        REGEXP,
    }

    // Placeholder types, replace with actual V8 types.
    struct AllowExternalCallThatCantCauseGC {}

    impl AllowExternalCallThatCantCauseGC {
        fn new() -> Self {
            AllowExternalCallThatCantCauseGC {}
        }
    }

    struct SetIsolateDataSlots {}
    impl SetIsolateDataSlots {
        const kNo: i32 = 0;
    }

    struct Factory {}
    impl Factory {
        fn CodeBuilder(isolate: &Isolate, code_desc: CodeDesc, code_kind: CodeKind) -> CodeBuilder {
            CodeBuilder {}
        }
    }

    struct CodeBuilder {}
    impl CodeBuilder {
        fn set_self_reference(self, code_object: *mut std::ffi::c_void) -> Self {
            self
        }
        fn set_empty_source_position_table(self) -> Self {
            self
        }
        fn Build(self) -> Handle<HeapObject> {
            Handle::new()
        }
    }
    
    const kStackLimitSlackSlotCount: i32 = 100; // Example value

    pub struct RegExpMacroAssemblerIA32 {
        isolate_: *mut Isolate,
        zone_: *mut Zone,
        masm_: Box<MacroAssembler>,
        no_root_array_scope_: bool, //NoRootArrayScope, // Assuming this is just a boolean flag
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
        global_with_zero_length_check_: bool,
        global_unicode_: bool,
        backtrack_limit_: i32,
        can_fallback_: bool,
        has_backtrack_limit_: bool,
    }

    impl RegExpMacroAssemblerIA32 {
        pub fn new(isolate: *mut Isolate, zone: *mut Zone, mode: Mode, registers_to_save: i32) -> Self {
            assert_eq!(0, registers_to_save % 2);

            let mut masm_ = Box::new(MacroAssembler {});
            let mut entry_label_ = Label::new();
            //let mut start_label_ = Label::new();

            // Dummy implementation to match the original C++ code, needs a valid jmp instruction
            //__ jmp(&entry_label_);
            // Replace with the appropriate rust assembly instruction when available.
            // Placeholder for assembly instruction.
            //masm_.jmp(&mut entry_label_);

            RegExpMacroAssemblerIA32 {
                isolate_: isolate,
                zone_: zone,
                masm_: masm_,
                no_root_array_scope_: true,
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
                fallback_label_: Label::new(),
                global_with_zero_length_check_: false,
                global_unicode_: false,
                backtrack_limit_: 0,
                can_fallback_: false,
                has_backtrack_limit_: false,
            }
        }

        fn global(&self) -> bool {
            self.global_with_zero_length_check_ || self.global_unicode_
        }

        fn global_with_zero_length_check(&self) -> bool {
            self.global_with_zero_length_check_
        }

        fn global_unicode(&self) -> bool {
            self.global_unicode_
        }

        fn has_backtrack_limit(&self) -> bool {
            self.has_backtrack_limit_
        }
        
        fn can_fallback(&self) -> bool {
            self.can_fallback_
        }
        
        fn backtrack_limit(&self) -> i32 {
            self.backtrack_limit_
        }

        fn char_size(&self) -> i32 {
            match self.mode_ {
                Mode::LATIN1 => 1,
                Mode::UC16 => 2,
            }
        }
        
        fn stack_limit_slack_slot_count() -> i32 {
            kStackLimitSlackSlotCount
        }

        fn AdvanceCurrentPosition(&mut self, by: i32) {
            if by != 0 {
                //self.masm_.add(Register::edi, Immediate::new(by * self.char_size()));
            }
        }

        fn AdvanceRegister(&mut self, reg: i32, by: i32) {
            assert!(reg >= 0);
            assert!(self.num_registers_ > reg);
            if by != 0 {
                //self.masm_.add(self.register_location(reg), Immediate::new(by));
            }
        }

        fn Backtrack(&mut self) {
            self.CheckPreemption();
            if self.has_backtrack_limit() {
                let mut next = Label::new();
                //self.masm_.inc(Operand::new(Register::ebp, kBacktrackCountOffset));
                //self.masm_.cmp(Operand::new(Register::ebp, kBacktrackCountOffset), Immediate::new(self.backtrack_limit()));
                //self.masm_.j(Condition::not_equal, &next);

                if self.can_fallback() {
                    //self.masm_.jmp(&self.fallback_label_);
                } else {
                    self.Fail();
                }

                //self.masm_.bind(&next);
            }

            //self.Pop(Register::ebx);
            //self.masm_.add(Register::ebx, Immediate::new(self.masm_.CodeObject() as i32)); // This cast might be problematic
            //self.masm_.jmp(Register::ebx);
        }

        fn Bind(&mut self, label: &mut Label) {
            //self.masm_.bind(label);
        }

        fn CheckCharacter(&mut self, c: u32, on_equal: &mut Label) {
            //self.masm_.cmp(self.current_character(), c);
            self.BranchOrBacktrack(Condition::equal, on_equal);
        }

        fn CheckCharacterGT(&mut self, limit: u16, on_greater: &mut Label) {
            //self.masm_.cmp(self.current_character(), limit);
            self.BranchOrBacktrack(Condition::greater, on_greater);
        }

        fn CheckAtStart(&mut self, cp_offset: i32, on_at_start: &mut Label) {
            //self.masm_.lea(Register::eax, Operand::new(Register::edi, -self.char_size() + cp_offset * self.char_size()));
            //self.masm_.cmp(Register::eax, Operand::new(Register::ebp, kStringStartMinusOneOffset));
            self.BranchOrBacktrack(Condition::equal, on_at_start);
        }

        fn CheckNotAtStart(&mut self, cp_offset: i32, on_not_at_start: &mut Label) {
            //self.masm_.lea(Register::eax, Operand::new(Register::edi, -self.char_size() + cp_offset * self.char_size()));
            //self.masm_.cmp(Register::eax, Operand::new(Register::ebp, kStringStartMinusOneOffset));
            self.BranchOrBacktrack(Condition::not_equal, on_not_at_start);
        }

        fn CheckCharacterLT(&mut self, limit: u16, on_less: &mut Label) {
            //self.masm_.cmp(self.current_character(), limit);
            self.BranchOrBacktrack(Condition::less, on_less);
        }

        fn CheckGreedyLoop(&mut self, on_equal: &mut Label) {
            let mut fallthrough = Label::new();
            //self.masm_.cmp(Register::edi, Operand::new(self.backtrack_stackpointer(), 0));
            //self.masm_.j(Condition::not_equal, &fallthrough);
            //self.masm_.add(self.backtrack_stackpointer(), Immediate::new(kSystemPointerSize));
            self.BranchOrBacktrack(Condition::equal, on_equal);
            //self.masm_.bind(&fallthrough);
        }

        fn CallCFunctionFromIrregexpCode(&mut self, function: *const std::ffi::c_void, num_arguments: i32) {
            // Placeholder for call to external C function.
            //self.masm_.call_c_function(function, num_arguments, SetIsolateDataSlots::kNo);
        }

        fn PushCallerSavedRegisters(&mut self) {
            //self.masm_.push(backtrack_stackpointer());
            //self.masm_.push(current_character());
        }

        fn PopCallerSavedRegisters(&mut self) {
            //self.masm_.pop(current_character());
            //self.masm_.pop(backtrack_stackpointer());
        }

        fn CheckNotBackReferenceIgnoreCase(&mut self, start_reg: i32, read_backward: bool, unicode: bool, on_no_match: &mut Label) {
            let mut fallthrough = Label::new();
            //self.masm_.mov(current_character(), self.register_location(start_reg));
            //self.masm_.mov(Register::ebx, self.register_location(start_reg + 1));
            //self.masm_.sub(Register::ebx, current_character());

            //self.masm_.j(Condition::equal, &fallthrough);

            if read_backward {
                //self.masm_.mov(Register::eax, Operand::new(Register::ebp, kStringStartMinusOneOffset));
                //self.masm_.add(Register::eax, Register::ebx);
                //self.masm_.cmp(Register::edi, Register::eax);
                self.BranchOrBacktrack(Condition::less_equal, on_no_match);
            } else {
                //self.masm_.mov(Register::eax, Register::edi);
                //self.masm_.add(Register::eax, Register::ebx);
                self.BranchOrBacktrack(Condition::greater, on_no_match);
            }

            if self.mode_ == Mode::LATIN1 {
                let mut success = Label::new();
                let mut fail = Label::new();
                let mut loop_increment = Label::new();

                //self.masm_.push(Register::edi);
                //self.masm_.push(self.backtrack_stackpointer());

                //self.masm_.add(current_character(), Register::esi);
                //self.masm_.add(Register::edi, Register::esi);
                if read_backward {
                    //self.masm_.sub(Register::edi, Register::ebx);
                }
                //self.masm_.add(Register::ebx, Register::edi);

                let mut loop_label = Label::new();
                //self.masm_.bind(&loop_label);
                //self.masm_.movzx_b(Register::eax, Operand::new(Register::edi, 0));
                //self.masm_.cmpb_al(Operand::new(current_character(), 0));
                //self.masm_.j(Condition::equal, &loop_increment);

                //self.masm_.or_(Register::eax, 0x20);
                //self.masm_.lea(Register::ecx, Operand::new(Register::eax, -('a' as i32)));
                //self.masm_.cmp(Register::ecx, ('z' as i32) - ('a' as i32));
                let mut convert_capture = Label::new();
                //self.masm_.j(Condition::below_equal, &convert_capture);
                //self.masm_.sub(Register::ecx, Immediate::new(224 - ('a' as i32)));
                //self.masm_.cmp(Register::ecx, Immediate::new(254 - 224));
                //self.masm_.j(Condition::above, &fail);
                //self.masm_.cmp(Register::ecx, Immediate::new(247 - 224));
                //self.masm_.j(Condition::equal, &fail);

                //self.masm_.bind(&convert_capture);
                //self.masm_.movzx_b(Register::ecx, Operand::new(current_character(), 0));
                //self.masm_.or_(Register::ecx, 0x20);

                //self.masm_.cmp(Register::eax, Register::ecx);
                //self.masm_.j(Condition::not_equal, &fail);

                //self.masm_.bind(&loop_increment);
                //self.masm_.add(current_character(), Immediate::new(1));
                //self.masm_.add(Register::edi, Immediate::new(1));
                //self.masm_.cmp(Register::edi, Register::ebx);
                //self.masm_.j(Condition::below, &loop_label);
                //self.masm_.jmp(&success);

                //self.masm_.bind(&fail);
                //self.masm_.pop(self.backtrack_stackpointer());
                //self.masm_.pop(Register::edi);
                self.BranchOrBacktrack(Condition::equal, on_no_match);

                //self.masm_.bind(&success);
                //self.masm_.pop(self.backtrack_stackpointer());
                //self.masm_.add(Register::esp, Immediate::new(kSystemPointerSize));
                //self.masm_.sub(Register::edi, Register::esi);
                if read_backward {
                    //self.masm_.add(Register::edi, self.register_location(start_reg));
                    //self.masm_.sub(Register::edi, self.register_location(start_reg + 1));
                }
            } else {
                assert_eq!(self.mode_, Mode::UC16);

                //self.masm_.push(Register::esi);
                //self.masm_.push(Register::edi);
                //self.masm_.push(self.backtrack_stackpointer());
                //self.masm_.push(Register::ebx);

                //const argument_count: i32 = 4;
                //self.masm_.PrepareCallCFunction(argument_count, Register::ecx);

                // Set isolate.
                //self.masm_.mov(Operand::new(Register::esp, 3 * kSystemPointerSize), Immediate::new(self.isolate_ as i32)); //TODO check

                // Set byte_length.
                //self.masm_.mov(Operand::new(Register::esp, 2 * kSystemPointerSize), Register::ebx);

                // Set byte_offset2.
                //self.masm_.add(Register::edi, Register::esi);
                if read_backward {
                    //self.masm_.sub(Register::edi, Register::ebx);
                }
                //self.masm_.mov(Operand::new(Register::esp, 1 * kSystemPointerSize), Register::edi);

                // Set byte_offset1.
                //self.masm_.add(current_character(), Register::esi);
                //self.masm_.mov(Operand::new(Register::esp, 0 * kSystemPointerSize), current_character());

                //{
                //    let scope = AllowExternalCallThatCantCauseGC::new();
                //    let compare = if unicode {
                //        //ExternalReference::re_case_insensitive_compare_unicode()
                //        0 as *const std::ffi::c_void
                //    } else {
                //        //ExternalReference::re_case_insensitive_compare_non_unicode()
                //        0 as *const std::ffi::c_void
                //    };
                //    //self.CallCFunctionFromIrregexpCode(compare, argument_count);
                //}
                //self.masm_.pop(Register::ebx);
                //self.masm_.pop(self.backtrack_stackpointer());
                //self.masm_.pop(Register::edi);
                //self.masm_.pop(Register::esi);

                //self.masm_.or_(Register::eax, Register::eax);
                self.BranchOrBacktrack(Condition::zero, on_no_match);

                if read_backward {
                    //self.masm_.sub(Register::edi, Register::ebx);
                } else {
                    //self.masm_.add(Register::edi, Register::ebx);
                }
            }
            //self.masm_.bind(&fallthrough);
        }

        fn CheckNotBackReference(&mut self, start_reg: i32, read_backward: bool, on_no_match: &mut Label) {
            let mut fallthrough = Label::new();
            let mut success = Label::new();
            let mut fail = Label::new();

            //self.masm_.mov(current_character(), self.register_location(start_reg));
            //self.masm_.mov(Register::eax, self.register_location(start_reg + 1));
            //self.masm_.sub(Register::eax, current_character());

            //self.masm_.j(Condition::equal, &fallthrough);

            if read_backward {
                //self.masm_.mov(Register::ebx, Operand::new(Register::ebp, kStringStartMinusOneOffset));
                //self.masm_.add(Register::ebx, Register::eax);
                //self.masm_.cmp(Register::edi, Register::ebx);
                self.BranchOrBacktrack(Condition::less_equal, on_no_match);
            } else {
                //self.masm_.mov(Register::ebx, Register::edi);
                //self.masm_.add(Register::ebx, Register::eax);
                self.BranchOrBacktrack(Condition::greater, on_no_match);
            }

            //self.masm_.push(self.backtrack_stackpointer());

            //self.masm_.add(current_character(), Register::esi);
            //self.masm_.lea(Register::ebx, Operand::new(Register::esi, Register::edi, 1, 0));
            if read_backward {
                //self.masm_.sub(Register::ebx, Register::eax);
            }
            //self.masm_.lea(Register::ecx, Operand::new(Register::eax, Register::ebx, 1, 0));

            let mut loop_label = Label::new();
            //self.masm_.bind(&loop_label);
            if self.mode_ == Mode::LATIN1 {
                //self.masm_.movzx_b(Register::eax, Operand::new(current_character(), 0));
                //self.masm_.cmpb_al(Operand::new(Register::ebx, 0));
            } else {
                assert_eq!(self.mode_, Mode::UC16);
                //self.masm_.movzx_w(Register::eax, Operand::new(current_character(), 0));
                //self.masm_.cmpw_ax(Operand::new(Register::ebx, 0));
            }
            //self.masm_.j(Condition::not_equal, &fail);

            //self.masm_.add(current_character(), Immediate::new(self.char_size()));
            //self.masm_.add(Register::ebx, Immediate::new(self.char_size()));
            //self.masm_.cmp(Register::ebx, Register::ecx);
            //self.masm_.j(Condition::below, &loop_label);
            //self.masm_.jmp(&success);

            //self.masm_.bind(&fail);
            //self.masm_.pop(self.backtrack_stackpointer());
            self.BranchOrBacktrack(Condition::equal, on_no_match);

            //self.masm_.bind(&success);
            //self.masm_.mov(Register::edi, Register::ecx);
            //self.masm_.sub(Register::edi, Register::esi);
            if read_backward {
                //self.masm_.add(Register::edi, self.register_location(start_reg));
                //self.masm_.sub(Register::edi, self.register_location(start_reg + 1));
            }
            //self.masm_.pop(self.backtrack_stackpointer());

            //self.masm_.bind(&fallthrough);
        }

        fn CheckNotCharacter(&mut self, c: u32, on_not_equal: &mut Label) {
            //self.masm_.cmp(self.current_character(), c);
            self.BranchOrBacktrack(Condition::not_equal, on_not_equal);
        }

        fn CheckCharacterAfterAnd(&mut self, c: u32, mask: u32, on_equal: &mut Label) {
            if c == 0 {
                //self.masm_.test(self.current_character(), Immediate::new(mask as i32));
            } else {
                //self.masm_.mov(Register::eax, mask as i32);
                //self.masm_.and_(Register::eax, self.current_character());
                //self.masm_.cmp(Register::eax, c as i32);
            }
            self.BranchOrBacktrack(Condition::equal, on_equal);
        }

        fn CheckNotCharacterAfterAnd(&mut self, c: u32, mask: u32, on_not_equal: &mut Label) {
            if c == 0 {
                //self.masm_.test(self.current_character(), Immediate::new(mask as i32));
            } else {
                //self.masm_.mov(Register::eax, mask as i32);
                //self.masm_.and_(Register::eax, self.current_character());
                //self.masm_.cmp(Register::eax, c as i32);
            }
            self.BranchOrBacktrack(Condition::not_equal, on_not_equal);
        }

        fn CheckNotCharacterAfterMinusAnd(&mut self, c: u16, minus: u16, mask: u16, on_not_equal: &mut Label) {
            assert!(strings::unicode::uc16::MAX > minus);
            //self.masm_.lea(Register::eax, Operand::new(self.current_character(), -(minus as i32)));
            if c == 0 {
                //self.masm_.test(Register::eax, Immediate::new(mask as i32));
            } else {
                //self.masm_.and_(Register::eax, mask as i32);
                //self.masm_.cmp(Register::eax, c as i32);
            }
            self.BranchOrBacktrack(Condition::not_equal, on_not_equal);
        }

        fn CheckCharacterInRange(&mut self, from: u16, to: u16, on_in_range: &mut Label) {
            //self.masm_.lea(Register::eax, Operand::new(self.current_character(), -(from as i32)));
            //self.masm_.cmp(Register::eax, (to - from) as i32);
            self.BranchOrBacktrack(Condition::below_equal, on_in_range);
        }

        fn CheckCharacterNotInRange(&mut self, from: u16, to: u16, on_not_in_range: &mut Label) {
            //self.masm_.lea(Register::eax, Operand::new(self.current_character(), -(from as i32)));
            //self.masm_.cmp(Register::eax, (to - from) as i32);
            self.BranchOrBacktrack(Condition::above, on_not_in_range);
        }

        fn CallIsCharacterInRangeArray(&mut self, ranges: *const ZoneList<CharacterRange>) {
            self.PushCallerSavedRegisters();

            //const num_arguments: i32 = 2;
            //self.masm_.PrepareCallCFunction(num_arguments, Register::ecx);

            //self.masm_.mov(Operand::new(Register::esp, 0 * kSystemPointerSize), self.current_character());
            //self.masm_.mov(Operand::new(Register::esp, 1 * kSystemPointerSize), self.GetOrAddRangeArray(ranges));

            //{
            //    let scope = FrameScope::new(); //masm_.get(), StackFrame::MANUAL
            //    //self.CallCFunctionFromIrregexpCode(ExternalReference::re_is_character_in_range_array(), num_arguments);
            //}

            self.PopCallerSavedRegisters();
        }

        fn CheckCharacterInRangeArray(&mut self, ranges: *const ZoneList<CharacterRange>, on_in_range: &mut Label) -> bool {
            self.CallIsCharacterInRangeArray(ranges);
            