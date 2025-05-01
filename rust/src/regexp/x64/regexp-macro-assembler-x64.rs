// use std::arch::x86_64::*;
use std::convert::TryInto;
use std::mem;
use std::ptr;
use std::sync::atomic::{AtomicUsize, Ordering};

const V8_TARGET_ARCH_X64: bool = true;

const V8_TARGET_OS_WIN: bool = cfg!(target_os = "windows");

macro_rules! ACCESS_MASM {
    ($masm:expr) => {
        $masm
    };
}

macro_rules! DCHECK_EQ {
    ($left:expr, $right:expr) => {
        debug_assert_eq!($left, $right);
    };
}

macro_rules! DCHECK_LE {
    ($left:expr, $right:expr) => {
        debug_assert!($left <= $right);
    };
}

macro_rules! DCHECK_GT {
    ($left:expr, $right:expr) => {
        debug_assert!($left > $right);
    };
}

macro_rules! DCHECK {
    ($condition:expr) => {
        debug_assert!($condition);
    };
}

macro_rules! Move {
    ($dst:expr, $src:expr) => {
        // Placeholder for move instruction
        $dst = $src;
    };
}

macro_rules! LoadAddress {
    ($dst:expr, $src:expr) => {
        // Placeholder for load address instruction
        $dst = $src;
    };
}

macro_rules! FieldOperand {
    ($base:expr, $index:expr, $times:expr, $offset:expr) => {
        // Placeholder for field operand
        unsafe { $base.offset(($index as isize) * $times as isize + $offset as isize) }
    };
}

mod regexp {
    pub mod regexp_stack {
        pub const K_STACK_LIMIT_SLACK_SLOT_COUNT: i32 = 12;
    }
}

mod logging {
    pub mod log {
        #[macro_export]
        macro_rules! PROFILE {
            ($isolate:expr, $event:expr) => {
                // Placeholder for profiling events
                // log::isolate_event($isolate, $event);
            };
        }
    }
}

mod objects {
    pub mod code_inl {
        // Define constants or structures that would be found in code-inl.h
    }
}

mod heap {
    pub mod factory {
        // Define functions or structures that would be found in factory.h
        pub struct CodeBuilder {}
        impl CodeBuilder {
            pub fn set_self_reference(self, _arg: usize) -> Self {
                self
            }
            pub fn set_empty_source_position_table(self) -> Self {
                self
            }
            pub fn Build(self) -> usize {
                0
            }
        }
    }
}

mod codegen {
    pub mod code_desc {
        // Define structs or enums if necessary
    }
    pub mod macro_assembler {
        // Implementations for MacroAssembler, AssemblerBuffer etc.

        pub struct MacroAssembler {}

        impl MacroAssembler {
            pub fn GetCode(&self, _isolate: *mut Isolate, _code_desc: &mut usize) {}
            pub fn CodeObject(&self) -> usize {
                0
            }
            pub fn long_at(&self, _patch_position: i32) -> i32 {
                0
            }
            pub fn long_at_put(&self, _patch_position: i32, _value: i32) {}
        }
    }
}

mod base {
    pub type uc16 = u16;
}

mod strings {
    pub const K_MAX_UTF16_CODE_UNIT: u32 = 0x10FFFF;
    pub const K_MAX_ONE_BYTE_CHAR_CODE: u32 = 255;
}

mod flags {
    pub struct FlagList {
        pub regexp_simd: bool,
    }
}

lazy_static::lazy_static! {
    pub static ref v8_flags: flags::FlagList = flags::FlagList {
        regexp_simd: true,
    };
}

const WORD_CHARACTER_MAP_SIZE: usize = 256;
pub static word_character_map: [u8; WORD_CHARACTER_MAP_SIZE] = [0; WORD_CHARACTER_MAP_SIZE];

// Define missing structs and enums
#[derive(Debug, PartialEq)]
enum StandardCharacterSet {
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

#[derive(Debug)]
struct ZoneList<T> {
    items: Vec<T>,
}

impl<T> ZoneList<T> {
    fn new() -> Self {
        ZoneList { items: Vec::new() }
    }
    fn push(&mut self, item: T) {
        self.items.push(item);
    }
}

#[derive(Debug)]
struct CharacterRange {}

enum CodeKind {
    REGEXP,
}

// Constants
const FAILURE: i32 = 0;
const SUCCESS: i32 = 1;
const EXCEPTION: i32 = -1;
const FALLBACK_TO_EXPERIMENTAL: i32 = -2;

// Stackframe enum (approximation)
enum StackFrame {
    IRREGEXP,
    MANUAL,
}

// Isolate struct (approximation)
struct Isolate {}

impl Isolate {
    fn current() -> *mut Isolate {
        // Stub implementation. In a real setting, access the current isolate.
        unsafe { &mut ISOLATE as *mut Isolate }
    }
}

static mut ISOLATE: Isolate = Isolate {};

// RegExpFlags struct (approximation)
#[derive(Debug)]
struct RegExpFlags {}

// Dummy struct for String
struct String {}

// Define constants for offsets.
const K_DIRECT_CALL_OFFSET: i32 = 0;
const K_ISOLATE_OFFSET: i32 = 8;
const K_INPUT_STRING_OFFSET: i32 = -16;
const K_START_INDEX_OFFSET: i32 = -24;
const K_INPUT_START_OFFSET: i32 = -32;
const K_INPUT_END_OFFSET: i32 = -40;
const K_REGISTER_OUTPUT_OFFSET: i32 = -48;
const K_NUM_OUTPUT_REGISTERS_OFFSET: i32 = -56;
const K_BACKUP_RBX_OFFSET: i32 = -64;
const K_BACKUP_RSI_OFFSET: i32 = -16;
const K_BACKUP_RDI_OFFSET: i32 = -24;
const K_SUCCESSFUL_CAPTURES_OFFSET: i32 = -72;
const K_STRING_START_MINUS_ONE_OFFSET: i32 = -80;
const K_BACKTRACK_COUNT_OFFSET: i32 = -88;
const K_REG_EXP_STACK_BASE_POINTER_OFFSET: i32 = -96;
const K_LAST_CALLEE_SAVE_REGISTER: i32 = -72;
const K_FRAME_TYPE_OFFSET: i32 = -8;
const K_REGISTER_ZERO_OFFSET: i32 = -104;

// Define a dummy struct and impl for ExternalReference (since its a V8 type).
#[derive(Debug, Copy, Clone)]
struct ExternalReference {}

impl ExternalReference {
    fn isolate_address(_isolate: *mut Isolate) -> Self {
        ExternalReference {}
    }
    fn re_check_stack_guard_state() -> Self {
        ExternalReference {}
    }
    fn address_of_jslimit(_isolate: *mut Isolate) -> Self {
        ExternalReference {}
    }
    fn re_grow_stack() -> Self {
        ExternalReference {}
    }
    fn address_of_regexp_stack_limit_address(_isolate: *mut Isolate) -> Self {
        ExternalReference {}
    }
    fn address_of_regexp_stack_stack_pointer(_isolate: *mut Isolate) -> Self {
        ExternalReference {}
    }
    fn address_of_regexp_stack_memory_top_address(_isolate: *mut Isolate) -> Self {
        ExternalReference {}
    }
    fn re_word_character_map() -> Self {
        ExternalReference {}
    }
    fn re_case_insensitive_compare_unicode() -> Self {
        ExternalReference {}
    }
    fn re_case_insensitive_compare_non_unicode() -> Self {
        ExternalReference {}
    }
    fn re_is_character_in_range_array() -> Self {
        ExternalReference {}
    }
}

// Implement the macro with dummy implementation
macro_rules! __ExternalReferenceAsOperand {
    ($reference:expr, $reg:expr) => {
        $reference // return reference as dummy
    };
}

// Helper function to create a dummy Immediate type
fn Immediate(_value: i32) -> i32 {
    _value
}

// Another helper function to create a dummy char_size.
fn CharSizeScaleFactor() -> i32 {
    0
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug, Copy, Clone)]
enum Mode {
    LATIN1,
    UC16,
}

const K_TABLE_MASK: u32 = strings::K_MAX_ONE_BYTE_CHAR_CODE;

// Define enum Condition for jump conditions
#[derive(Debug, Copy, Clone)]
enum Condition {
    equal,
    not_equal,
    less,
    greater,
    less_equal,
    greater_equal,
    above,
    below_equal,
    zero,
    not_zero,
}

// Placeholder for RegExpStack structure
struct RegExpStack {}

impl RegExpStack {
    const K_STACK_LIMIT_SLACK_SIZE: i32 = 1024;
}

// Define a struct to represent the Assembler
struct Assembler {}

impl Assembler {
    fn AllocateStackSpace(&mut self, _size: i32) {}
    fn EnterFrame(&mut self, _frame: StackFrame) {}
    fn LeaveFrame(&mut self, _frame: StackFrame) {}
}

// RootArrayScope struct
struct NoRootArrayScope<'a> {
    masm_: &'a mut codegen::macro_assembler::MacroAssembler,
}

impl<'a> NoRootArrayScope<'a> {
    fn new(masm: &'a mut codegen::macro_assembler::MacroAssembler) -> Self {
        NoRootArrayScope { masm_: masm }
    }
}

// Implement NativeRegExpMacroAssembler methods needed
trait RegExpMacroAssemblerInterface {
    fn isolate(&self) -> *mut Isolate;
    fn global(&self) -> bool;
    fn global_with_zero_length_check(&self) -> bool;
    fn global_unicode(&self) -> bool;
    fn has_backtrack_limit(&self) -> bool;
    fn backtrack_limit(&self) -> i32;
    fn can_fallback(&self) -> bool;
    fn char_size(&self) -> i32;
    fn CheckNotInSurrogatePair(&self, _cp_offset: i32, _advance: &Label);
}

// Implement NativeRegExpMacroAssembler methods needed
trait NativeRegExpMacroAssemblerInterface {
    fn CheckStackGuardState(
        return_address: *mut Address,
        raw_code: Address,
        re_frame: Address,
        extra_space: usize,
    ) -> i32;
}

// Dummy Address type
type Address = usize;

struct NativeRegExpMacroAssembler {}

impl NativeRegExpMacroAssemblerInterface for NativeRegExpMacroAssembler {
    fn CheckStackGuardState(
        _return_address: *mut Address,
        _raw_code: Address,
        _re_frame: Address,
        _extra_space: usize,
    ) -> i32 {
        0
    }
}

pub struct RegExpMacroAssemblerX64 {
    masm_: codegen::macro_assembler::MacroAssembler,
    no_root_array_scope_: NoRootArrayScope<'static>,
    code_relative_fixup_positions_: ZoneList<i32>,
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
    isolate_: *mut Isolate,
    global_: bool,
    global_with_zero_length_check_: bool,
    global_unicode_: bool,
    has_backtrack_limit_: bool,
    backtrack_limit_: i32,
    can_fallback_: bool,
}

const K_REG_EXP_CODE_SIZE: i32 = 4096;

impl RegExpMacroAssemblerX64 {
    pub const kRegExpCodeSize: i32 = K_REG_EXP_CODE_SIZE;

    pub fn new(
        isolate: *mut Isolate,
        zone: *mut usize,
        mode: Mode,
        registers_to_save: i32,
    ) -> Self {
        DCHECK_EQ!(0, registers_to_save % 2);

        let mut masm = codegen::macro_assembler::MacroAssembler {}; // Replace with actual initialization
        let mut no_root_array_scope_ = NoRootArrayScope::new(&mut masm);

        let mut result = Self {
            masm_: masm,
            no_root_array_scope_: no_root_array_scope_,
            code_relative_fixup_positions_: ZoneList::new(),
            mode_: mode,
            num_registers_: registers_to_save,
            num_saved_registers_: registers_to_save,
            entry_label_: Label::new("entry_label"),
            start_label_: Label::new("start_label"),
            success_label_: Label::new("success_label"),
            backtrack_label_: Label::new("backtrack_label"),
            exit_label_: Label::new("exit_label"),
            check_preempt_label_: Label::new("check_preempt_label"),
            stack_overflow_label_: Label::new("stack_overflow_label"),
            fallback_label_: Label::new("fallback_label"),
            isolate_: isolate, // Use provided isolate
            global_: false,
            global_with_zero_length_check_: false,
            global_unicode_: false,
            has_backtrack_limit_: false,
            backtrack_limit_: 0,
            can_fallback_: false,
        };

        ACCESS_MASM!(&mut result.masm_).CodeEntry();
        ACCESS_MASM!(&mut result.masm_).jmp(&result.entry_label_); // We'll write the entry code when we know more.
        ACCESS_MASM!(&mut result.masm_).bind(&result.start_label_); // And then continue from here.

        result
    }

    fn stack_limit_slack_slot_count() -> i32 {
        regexp::regexp_stack::K_STACK_LIMIT_SLACK_SLOT_COUNT
    }

    fn AdvanceCurrentPosition(&mut self, by: i32) {
        if by != 0 {
            // ACCESS_MASM!((&mut self.masm_)).addq(rdi, Immediate(by * self.char_size()));
        }
    }

    fn AdvanceRegister(&mut self, reg: i32, by: i32) {
        DCHECK_LE!(0, reg);
        DCHECK_GT!(self.num_registers_, reg);
        if by != 0 {
            // ACCESS_MASM!((&mut self.masm_)).addq(self.register_location(reg), Immediate(by));
        }
    }

    fn Backtrack(&mut self) {
        self.CheckPreemption();
        if self.has_backtrack_limit() {
            let mut next = Label::new("next");
            // ACCESS_MASM!((&mut self.masm_)).incq(Operand(rbp, K_BACKTRACK_COUNT_OFFSET));
            // ACCESS_MASM!((&mut self.masm_)).cmpq(Operand(rbp, K_BACKTRACK_COUNT_OFFSET), Immediate(self.backtrack_limit()));
            // ACCESS_MASM!((&mut self.masm_)).j(not_equal, &next);

            // Backtrack limit exceeded.
            if self.can_fallback() {
                // ACCESS_MASM!((&mut self.masm_)).jmp(&self.fallback_label_);
            } else {
                // Can't fallback, so we treat it as a failed match.
                self.Fail();
            }

            // ACCESS_MASM!((&mut self.masm_)).bind(&next);
        }
        // Pop InstructionStream offset from backtrack stack, add InstructionStream
        // and jump to location.
        self.Pop(0); //rbx
        // ACCESS_MASM!((&mut self.masm_)).addq(rbx, self.code_object_pointer());

        // TODO(sroettger): This jump needs an endbr64 instruction but the code is
        // performance sensitive. Needs more thought how to do this in a fast way.
        // ACCESS_MASM!((&mut self.masm_)).jmp(rbx, /*notrack=*/true);
    }

    fn Bind(&mut self, label: &Label) {
        ACCESS_MASM!((&mut self.masm_)).bind(label);
    }

    fn CheckCharacter(&mut self, c: u32, on_equal: &mut Label) {
        // ACCESS_MASM!((&mut self.masm_)).cmpl(self.current_character(), Immediate(c as i32));
        self.BranchOrBacktrack(Condition::equal, on_equal);
    }

    fn CheckCharacterGT(&mut self, limit: base::uc16, on_greater: &mut Label) {
        // ACCESS_MASM!((&mut self.masm_)).cmpl(self.current_character(), Immediate(limit as i32));
        self.BranchOrBacktrack(Condition::greater, on_greater);
    }

    fn CheckAtStart(&mut self, cp_offset: i32, on_at_start: &mut Label) {
        // ACCESS_MASM!((&mut self.masm_)).leaq(rax, Operand(rdi, -self.char_size() + cp_offset * self.char_size()));
        // ACCESS_MASM!((&mut self.masm_)).cmpq(rax, Operand(rbp, K_STRING_START_MINUS_ONE_OFFSET));
        self.BranchOrBacktrack(Condition::equal, on_at_start);
    }

    fn CheckNotAtStart(&mut self, cp_offset: i32, on_not_at_start: &mut Label) {
        // ACCESS_MASM!((&mut self.masm_)).leaq(rax, Operand(rdi, -self.char_size() + cp_offset * self.char_size()));
        // ACCESS_MASM!((&mut self.masm_)).cmpq(rax, Operand(rbp, K_STRING_START_MINUS_ONE_OFFSET));
        self.BranchOrBacktrack(Condition::not_equal, on_not_at_start);
    }

    fn CheckCharacterLT(&mut self, limit: base::uc16, on_less: &mut Label) {
        // ACCESS_MASM!((&mut self.masm_)).cmpl(self.current_character(), Immediate(limit as i32));
        self.BranchOrBacktrack(Condition::less, on_less);
    }

    fn CheckGreedyLoop(&mut self, on_equal: &mut Label) {
        let mut fallthrough = Label::new("fallthrough");
        // ACCESS_MASM!((&mut self.masm_)).cmpl(rdi, Operand(self.backtrack_stackpointer(), 0));
        // ACCESS_MASM!((&mut self.masm_)).j(not_equal, &fallthrough);
        self.Drop();
        self.BranchOrBacktrack(Condition::equal, on_equal);
        // ACCESS_MASM!((&mut self.masm_)).bind(&fallthrough);
    }

    fn CallCFunctionFromIrregexpCode(&mut self, _function: ExternalReference, _num_arguments: i32) {
        // Irregexp code must not set fast_c_call_caller_fp and fast_c_call_caller_pc
        // since
        //
        // 1. it may itself have been called using CallCFunction and nested calls are
        //    unsupported, and
        // 2. it may itself have been called directly from C where the frame pointer
        //    might not be set (-fomit-frame-pointer), and thus frame iteration would
        //    fail.
        //
        // See also: crbug.com/v8/12670#c17.
        // ACCESS_MASM!((&mut self.masm_)).CallCFunction(function, num_arguments, SetIsolateDataSlots::kNo);
    }

    // Push (pop) caller-saved registers used by irregexp.
    fn PushCallerSavedRegisters(&mut self) {
        // ACCESS_MASM!((&mut self.masm_)).pushq(rsi);
        // ACCESS_MASM!((&mut self.masm_)).pushq(rdi);
        // ACCESS_MASM!((&mut self.masm_)).pushq(rcx);
    }

    fn PopCallerSavedRegisters(&mut self) {
        // ACCESS_MASM!((&mut self.masm_)).popq(rcx);
        // ACCESS_MASM!((&mut self.masm_)).popq(rdi);
        // ACCESS_MASM!((&mut self.masm_)).popq(rsi);
    }

    fn CheckNotBackReferenceIgnoreCase(
        &mut self,
        start_reg: i32,
        read_backward: bool,
        unicode: bool,
        on_no_match: &mut Label,
    ) {
        let mut fallthrough = Label::new("fallthrough");
        self.ReadPositionFromRegister(0, start_reg); // rdx
        self.ReadPositionFromRegister(1, start_reg + 1); // rbx
                                                          // ACCESS_MASM!((&mut self.masm_)).subq(rbx, rdx); // Length of capture.

        // -----------------------
        // rdx  = Start offset of capture.
        // rbx = Length of capture

        // At this point, the capture registers are either both set or both cleared.
        // If the capture length is zero, then the capture is either empty or cleared.
        // Fall through in both cases.
        // ACCESS_MASM!((&mut self.masm_)).j(equal, &fallthrough);

        // -----------------------
        // rdx - Start of capture
        // rbx - length of capture
        // Check that there are sufficient characters left in the input.
        if read_backward {
            // ACCESS_MASM!((&mut self.masm_)).movl(rax, Operand(rbp, K_STRING_START_MINUS_ONE_OFFSET));
            // ACCESS_MASM!((&mut self.masm_)).addl(rax, rbx);
            // ACCESS_MASM!((&mut self.masm_)).cmpl(rdi, rax);
            self.BranchOrBacktrack(Condition::less_equal, on_no_match);
        } else {
            // ACCESS_MASM!((&mut self.masm_)).movl(rax, rdi);
            // ACCESS_MASM!((&mut self.masm_)).addl(rax, rbx);
            self.BranchOrBacktrack(Condition::greater, on_no_match);
        }

        if self.mode_ == Mode::LATIN1 {
            let mut loop_increment = Label::new("loop_increment");
            let on_no_match = if on_no_match.name == "backtrack_label" {
                &mut self.backtrack_label_
            } else {
                on_no_match
            };

            // ACCESS_MASM!((&mut self.masm_)).leaq(r9, Operand(rsi, rdx, times_1, 0));
            // ACCESS_MASM!((&mut self.masm_)).leaq(r11, Operand(rsi, rdi, times_1, 0));
            if read_backward {
                // ACCESS_MASM!((&mut self.masm_)).subq(r11, rbx); // Offset by length when matching backwards.
            }
            // ACCESS_MASM!((&mut self.masm_)).addq(rbx, r9); // End of capture
                                                            // ---------------------
                                                            // r11 - current input character address
                                                            // r9 - current capture character address
                                                            // rbx - end of capture

            let mut loop_label = Label::new("loop");
            // ACCESS_MASM!((&mut self.masm_)).bind(&loop);
            // ACCESS_MASM!((&mut self.masm_)).movzxbl(rdx, Operand(r9, 0));
            // ACCESS_MASM!((&mut self.masm_)).movzxbl(rax, Operand(r11, 0));
            // al - input character
            // dl - capture character
            // ACCESS_MASM!((&mut self.masm_)).cmpb(rax, rdx);
            // ACCESS_MASM!((&mut self.masm_)).j(equal, &loop_increment);

            // Mismatch, try case-insensitive match (converting letters to lower-case).
            // I.e., if or-ing with 0x20 makes values equal and in range 'a'-'z', it's
            // a match.
            // ACCESS_MASM!((&mut self.masm_)).orq(rax, Immediate(0x20)); // Convert match character to lower-case.
            // ACCESS_MASM!((&mut self.masm_)).orq(rdx, Immediate(0x20)); // Convert capture character to lower-case.
            // ACCESS_MASM!((&mut self.masm_)).cmpb(rax, rdx);
            // ACCESS_MASM!((&mut self.masm_)).j(not_equal, on_no_match); // Definitely not equal.
            // ACCESS_MASM!((&mut self.masm_)).subb(rax, Immediate('a' as i32));
            // ACCESS_MASM!((&mut self.masm_)).cmpb(rax, Immediate(('z' as i32) - ('a' as i32)));
            // ACCESS_MASM!((&mut self.masm_)).j(below_equal, &loop_increment); // In range 'a'-'z'.
            // Latin-1: Check for values in range [224,254] but not 247.
            // ACCESS_MASM!((&mut self.masm_)).subb(rax, Immediate((224 - ('a' as i32))));
            // ACCESS_MASM!((&mut self.masm_)).cmpb(rax, Immediate(254 - 224));
            // ACCESS_MASM!((&mut self.masm_)).j(above, on_no_match); // Weren't Latin-1 letters.
            // ACCESS_MASM!((&mut self.masm_)).cmpb(rax, Immediate(247 - 224)); // Check for 247.
            // ACCESS_MASM!((&mut self.masm_)).j(equal, on_no_match);
            // ACCESS_MASM!((&mut self.masm_)).bind(&loop_increment);
            // Increment pointers into match and capture strings.
            // ACCESS_MASM!((&mut self.masm_)).addq(r11, Immediate(1));
            // ACCESS_MASM!((&mut self.masm_)).addq(r9, Immediate(1));
            // Compare to end of capture, and loop if not done.
            // ACCESS_MASM!((&mut self.masm_)).cmpq(r9, rbx);
            // ACCESS_MASM!((&mut self.masm_)).j(below, &loop);

            // Compute new value of character position after the matched part.
            // ACCESS_MASM!((&mut self.masm_)).movq(rdi, r11);
            // ACCESS_MASM!((&mut self.masm_)).subq(rdi, rsi);
            if read_backward {
                // Subtract match length if we matched backward.
                // ACCESS_MASM!((&mut self.masm_)).addq(rdi, self.register_location(start_reg));
                // ACCESS_MASM!((&mut self.masm_)).subq(rdi, self.register_location(start_reg + 1));
            }
        } else {
            DCHECK_EQ!(self.mode_, Mode::UC16);
            self.PushCallerSavedRegisters();

            //static const int num_arguments = 4;
            // ACCESS_MASM!((&mut self.masm_)).PrepareCallCFunction(num_arguments);

            // Put arguments into parameter registers. Parameters are
            //   Address byte_offset1 - Address captured substring's start.
            //   Address byte_offset2 - Address of current character position.
            //   size_t byte_length - length of capture in bytes(!)
            //   Isolate* isolate.
            //
            // Compute and set byte_offset1 (start of capture).
            // ACCESS_MASM!((&mut self.masm_)).leaq(rcx, Operand(rsi, rdx, times_1, 0));
            // Set byte_offset2.
            // ACCESS_MASM!((&mut self.masm_)).leaq(rdx, Operand(rsi, rdi, times_1, 0));
            if read_backward {
                // ACCESS_MASM!((&mut self.masm_)).subq(rdx, rbx);
            }
            // Set byte_length.
            // ACCESS_MASM!((&mut self.masm_)).movq(kCArgRegs[2], rbx);
            // Isolate.
            // ACCESS_MASM!((&mut self.masm_)).LoadAddress(kCArgRegs[3], ExternalReference::isolate_address(self.isolate()));

            {
                //AllowExternalCallThatCantCauseGC scope(&masm_);
                let compare = if unicode {
                    ExternalReference::re_case_insensitive_compare_unicode()
                } else {
                    ExternalReference::re_case_insensitive_compare_non_unicode()
                };
                self.CallCFunctionFromIrregexpCode(compare, 0); //num_arguments
            }

            // Restore original values before reacting on result value.
            // ACCESS_MASM!((&mut self.masm_)).Move(self.code_object_pointer(), self.masm_.CodeObject());
            self.PopCallerSavedRegisters();

            // Check if function returned non-zero for success or zero for failure.
            // ACCESS_MASM!((&mut self.masm_)).testq(rax, rax);
            self.BranchOrBacktrack(Condition::zero, on_no_match);
            // On success, advance position by length of capture.
            // Requires that rbx is callee save (true for both Win64 and AMD64 ABIs).
            if read_backward {
                // ACCESS_MASM!((&mut self.masm_)).subq(rdi, rbx);
            } else {
                // ACCESS_MASM!((&mut self.masm_)).addq(rdi, rbx);
            }
        }
        // ACCESS_MASM!((&mut self.masm_)).bind(&fallthrough);
    }

    fn CheckNotBackReference(
        &mut self,
        start_reg: i32,
        read_backward: bool,
        on_no_match: &mut Label,
    ) {
        let mut fallthrough = Label::new("fallthrough");

        // Find length of back-referenced capture.
        self.ReadPositionFromRegister(0, start_reg); // rdx
        self.ReadPositionFromRegister(2, start_reg + 1); // rax
                                                          // ACCESS_MASM!((&mut self.masm_)).subq(rax, rdx); // Length to check.

        // At this point, the capture registers are either both set or both cleared.
        // If the capture length is zero, then the capture is either empty or cleared.
        // Fall through in both cases.
        // ACCESS_MASM!((&mut self.masm_)).j(equal, &fallthrough);

        // -----------------------
        // rdx - Start of capture
        // rax - length of capture
        // Check that there are sufficient characters left in the input.
        if read_backward {
            // ACCESS_MASM!((&mut self.masm_)).movl(rbx, Operand(rbp, K_STRING_START_MINUS_ONE_OFFSET));
            // ACCESS_MASM!((&mut self.masm_)).addl(rbx, rax);
            // ACCESS_MASM!((&mut self.masm_)).cmpl(rdi, rbx);
            self.BranchOrBacktrack(Condition::less_equal, on_no_match);
        } else {
            // ACCESS_MASM!((&mut self.masm_)).movl(rbx, rdi);
            // ACCESS_MASM!((&mut self.masm