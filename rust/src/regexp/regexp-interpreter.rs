// Copyright 2011 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// A simple interpreter for the Irregexp byte code.

// #![allow(dead_code)]  // Suppress warnings about unused code

use std::cmp;
use std::convert::TryInto;
use std::mem;
//use std::os::raw::c_char;
use std::isize;
use std::ptr;
//use std::rc::Rc;

//use crate::base::small_vector::SmallVector;
//use crate::base::strings::Vector;
//use crate::execution::isolate::Isolate;
//use crate::logging::counters::Counters;
//use crate::objects::js_regexp::JSRegExp;
//use crate::objects::string::String;
//use crate::regexp::regexp_bytecodes::*;
//use crate::regexp::regexp_macro_assembler::RegExpMacroAssembler;
//use crate::regexp::regexp_stack::RegExpStack;
//use crate::regexp::regexp_utils::RegExpUtils;
//use crate::regexp::regexp::RegExp;
//use crate::strings::unicode;
//use crate::utils::memcopy::MemCopy;
//use crate::utils::utils::AllowGarbageCollection;

//use icu::uchar;  // Replace "unicode/uchar.h" with ICU

// Use token threaded dispatch iff the compiler supports computed gotos and the
// build argument v8_enable_regexp_interpreter_threaded_dispatch was set.
// #![cfg(all(V8_HAS_COMPUTED_GOTO, V8_ENABLE_REGEXP_INTERPRETER_THREADED_DISPATCH))]
// const V8_USE_COMPUTED_GOTO: bool =
//    cfg!(all(V8_HAS_COMPUTED_GOTO, V8_ENABLE_REGEXP_INTERPRETER_THREADED_DISPATCH));

// Mock implementations for necessary structures and functions
#[allow(unused_variables)]
mod mock {
    pub type Address = usize;
    pub struct Isolate {}
    impl Isolate {
        pub fn stack_overflow(&self) {}
        pub fn stack_guard(&self) -> StackGuard {
            StackGuard {}
        }
    }
    pub struct StackGuard {}
    impl StackGuard {
        pub fn interrupt_requested(&self) -> bool {
            false
        }
        pub fn handle_interrupts(&self) -> Result<Object, ()> {
            Ok(Object {})
        }
    }
    pub struct TrustedByteArray {}
    impl TrustedByteArray {
        pub fn begin(&self) -> *const u8 {
            std::ptr::null()
        }
    }

    pub struct String {}

    impl String {
        pub fn is_flat(&self) -> bool {
            true
        }
        pub fn get_flat_content(&self) -> FlatContent {
            FlatContent {}
        }
        pub fn is_one_byte_representation_underneath(&self) -> bool {
            true
        }
        pub fn len(&self) -> usize {
            0
        }
    }

    pub struct FlatContent {}

    impl FlatContent {
        pub fn is_one_byte(&self) -> bool {
            true
        }
        pub fn to_one_byte_vector(&self) -> Vec<u8> {
            Vec::new()
        }

        pub fn to_uc16_vector(&self) -> Vec<u16> {
            Vec::new()
        }

        pub fn unsafe_disable_checksum_verification(&mut self) {}

        pub fn is_two_byte(&self) -> bool {
            false
        }
    }

    pub struct RegExpFlags {}
    pub struct IrRegExpData {}
    impl IrRegExpData {
        pub fn flags(&self) -> RegExpFlags {
            RegExpFlags {}
        }
        pub fn bytecode(&self, is_one_byte: bool) -> TrustedByteArray {
            TrustedByteArray {}
        }
        pub fn max_register_count(&self) -> i32 {
            0
        }
        pub fn backtrack_limit(&self) -> i32 {
            0
        }
        pub fn marked_for_tier_up(&self) -> bool {
            false
        }
        pub fn tier_up_tick(&self) {}
        pub fn capture_count(&self) -> i32 {
            0
        }
    }
    pub struct RegExp {}

    impl RegExp {
        pub enum CallOrigin {
            kFromRuntime,
            kFromJs,
        }
    }

    pub struct Object {}

    pub fn is_exception(result: Result<Object, ()>) -> bool {
        result.is_err()
    }
    pub fn memcopy(dest: *mut i32, src: *const i32, count: usize) {
        unsafe {
            std::ptr::copy_nonoverlapping(src, dest, count);
        }
    }
}

const BYTECODE_SHIFT: i32 = 8;
const BYTECODE_MASK: i32 = 255;
const kBitsPerByteLog2: usize = 3;
const kBitsPerByte: usize = 8;

const BC_BREAK: i32 = 0;
const BC_PUSH_CP: i32 = 1;
const BC_PUSH_BT: i32 = 2;
const BC_PUSH_REGISTER: i32 = 3;
const BC_SET_REGISTER: i32 = 4;
const BC_ADVANCE_REGISTER: i32 = 5;
const BC_SET_REGISTER_TO_CP: i32 = 6;
const BC_SET_CP_TO_REGISTER: i32 = 7;
const BC_SET_REGISTER_TO_SP: i32 = 8;
const BC_SET_SP_TO_REGISTER: i32 = 9;
const BC_POP_CP: i32 = 10;
const BC_POP_BT: i32 = 11;
const BC_POP_REGISTER: i32 = 12;
const BC_FAIL: i32 = 13;
const BC_SUCCEED: i32 = 14;
const BC_ADVANCE_CP: i32 = 15;
const BC_GOTO: i32 = 16;
const BC_ADVANCE_CP_AND_GOTO: i32 = 17;
const BC_CHECK_GREEDY: i32 = 18;
const BC_LOAD_CURRENT_CHAR: i32 = 19;
const BC_LOAD_CURRENT_CHAR_UNCHECKED: i32 = 20;
const BC_LOAD_2_CURRENT_CHARS: i32 = 21;
const BC_LOAD_2_CURRENT_CHARS_UNCHECKED: i32 = 22;
const BC_LOAD_4_CURRENT_CHARS: i32 = 23;
const BC_LOAD_4_CURRENT_CHARS_UNCHECKED: i32 = 24;
const BC_CHECK_4_CHARS: i32 = 25;
const BC_CHECK_CHAR: i32 = 26;
const BC_CHECK_NOT_4_CHARS: i32 = 27;
const BC_CHECK_NOT_CHAR: i32 = 28;
const BC_AND_CHECK_4_CHARS: i32 = 29;
const BC_AND_CHECK_CHAR: i32 = 30;
const BC_AND_CHECK_NOT_4_CHARS: i32 = 31;
const BC_AND_CHECK_NOT_CHAR: i32 = 32;
const BC_MINUS_AND_CHECK_NOT_CHAR: i32 = 33;
const BC_CHECK_CHAR_IN_RANGE: i32 = 34;
const BC_CHECK_CHAR_NOT_IN_RANGE: i32 = 35;
const BC_CHECK_BIT_IN_TABLE: i32 = 36;
const BC_CHECK_LT: i32 = 37;
const BC_CHECK_GT: i32 = 38;
const BC_CHECK_REGISTER_LT: i32 = 39;
const BC_CHECK_REGISTER_GE: i32 = 40;
const BC_CHECK_REGISTER_EQ_POS: i32 = 41;
const BC_CHECK_NOT_REGS_EQUAL: i32 = 42;
const BC_CHECK_NOT_BACK_REF: i32 = 43;
const BC_CHECK_NOT_BACK_REF_BACKWARD: i32 = 44;
const BC_CHECK_NOT_BACK_REF_NO_CASE_UNICODE: i32 = 45;
const BC_CHECK_NOT_BACK_REF_NO_CASE: i32 = 46;
const BC_CHECK_NOT_BACK_REF_NO_CASE_UNICODE_BACKWARD: i32 = 47;
const BC_CHECK_NOT_BACK_REF_NO_CASE_BACKWARD: i32 = 48;
const BC_CHECK_AT_START: i32 = 49;
const BC_CHECK_NOT_AT_START: i32 = 50;
const BC_SET_CURRENT_POSITION_FROM_END: i32 = 51;
const BC_CHECK_CURRENT_POSITION: i32 = 52;
const BC_SKIP_UNTIL_CHAR: i32 = 53;
const BC_SKIP_UNTIL_CHAR_AND: i32 = 54;
const BC_SKIP_UNTIL_CHAR_POS_CHECKED: i32 = 55;
const BC_SKIP_UNTIL_BIT_IN_TABLE: i32 = 56;
const BC_SKIP_UNTIL_GT_OR_NOT_BIT_IN_TABLE: i32 = 57;
const BC_SKIP_UNTIL_CHAR_OR_CHAR: i32 = 58;

const kRegExpBytecodeCount: usize = 59;

const BC_BREAK_FILLER_1: i32 = 59;
const BC_BREAK_FILLER_2: i32 = 60;
const BC_BREAK_FILLER_3: i32 = 61;
const BC_BREAK_FILLER_4: i32 = 62;
const BC_BREAK_FILLER_5: i32 = 63;

const kRegExpBytecodeFillerCount: usize = 5;

const kRegExpPaddedBytecodeCount: usize = 64;

fn regexp_bytecode_length(bytecode: i32) -> usize {
    match bytecode {
        BC_BREAK => 4,
        BC_PUSH_CP => 4,
        BC_PUSH_BT => 8,
        BC_PUSH_REGISTER => 4,
        BC_SET_REGISTER => 8,
        BC_ADVANCE_REGISTER => 8,
        BC_SET_REGISTER_TO_CP => 8,
        BC_SET_CP_TO_REGISTER => 4,
        BC_SET_REGISTER_TO_SP => 4,
        BC_SET_SP_TO_REGISTER => 4,
        BC_POP_CP => 4,
        BC_POP_BT => 4,
        BC_POP_REGISTER => 4,
        BC_FAIL => 4,
        BC_SUCCEED => 4,
        BC_ADVANCE_CP => 4,
        BC_GOTO => 8,
        BC_ADVANCE_CP_AND_GOTO => 8,
        BC_CHECK_GREEDY => 8,
        BC_LOAD_CURRENT_CHAR => 8,
        BC_LOAD_CURRENT_CHAR_UNCHECKED => 4,
        BC_LOAD_2_CURRENT_CHARS => 8,
        BC_LOAD_2_CURRENT_CHARS_UNCHECKED => 4,
        BC_LOAD_4_CURRENT_CHARS => 8,
        BC_LOAD_4_CURRENT_CHARS_UNCHECKED => 4,
        BC_CHECK_4_CHARS => 12,
        BC_CHECK_CHAR => 8,
        BC_CHECK_NOT_4_CHARS => 12,
        BC_CHECK_NOT_CHAR => 8,
        BC_AND_CHECK_4_CHARS => 16,
        BC_AND_CHECK_CHAR => 12,
        BC_AND_CHECK_NOT_4_CHARS => 16,
        BC_AND_CHECK_NOT_CHAR => 12,
        BC_MINUS_AND_CHECK_NOT_CHAR => 12,
        BC_CHECK_CHAR_IN_RANGE => 12,
        BC_CHECK_CHAR_NOT_IN_RANGE => 12,
        BC_CHECK_BIT_IN_TABLE => 28,
        BC_CHECK_LT => 8,
        BC_CHECK_GT => 8,
        BC_CHECK_REGISTER_LT => 12,
        BC_CHECK_REGISTER_GE => 12,
        BC_CHECK_REGISTER_EQ_POS => 8,
        BC_CHECK_NOT_REGS_EQUAL => 12,
        BC_CHECK_NOT_BACK_REF => 8,
        BC_CHECK_NOT_BACK_REF_BACKWARD => 8,
        BC_CHECK_NOT_BACK_REF_NO_CASE_UNICODE => 8,
        BC_CHECK_NOT_BACK_REF_NO_CASE => 8,
        BC_CHECK_NOT_BACK_REF_NO_CASE_UNICODE_BACKWARD => 8,
        BC_CHECK_NOT_BACK_REF_NO_CASE_BACKWARD => 8,
        BC_CHECK_AT_START => 8,
        BC_CHECK_NOT_AT_START => 8,
        BC_SET_CURRENT_POSITION_FROM_END => 4,
        BC_CHECK_CURRENT_POSITION => 8,
        BC_SKIP_UNTIL_CHAR => 16,
        BC_SKIP_UNTIL_CHAR_AND => 24,
        BC_SKIP_UNTIL_CHAR_POS_CHECKED => 20,
        BC_SKIP_UNTIL_BIT_IN_TABLE => 32,
        BC_SKIP_UNTIL_GT_OR_NOT_BIT_IN_TABLE => 32,
        BC_SKIP_UNTIL_CHAR_OR_CHAR => 20,
        _ => panic!("Unknown bytecode {}", bytecode),
    }
}

#[allow(dead_code)]
fn load32_aligned(pc: *const u8) -> i32 {
    assert_eq!(0, pc as usize & 3);
    unsafe { *(pc as *const i32) }
}

#[allow(dead_code)]
fn load16_aligned_unsigned(pc: *const u8) -> u32 {
    assert_eq!(0, pc as usize & 1);
    unsafe { *(pc as *const u16) as u32 }
}

#[allow(dead_code)]
fn load16_aligned_signed(pc: *const u8) -> i32 {
    assert_eq!(0, pc as usize & 1);
    unsafe { *(pc as *const i16) as i32 }
}

#[allow(dead_code)]
fn load_packed24_signed(bytecode_and_packed_arg: i32) -> i32 {
    bytecode_and_packed_arg >> BYTECODE_SHIFT
}

#[allow(dead_code)]
fn load_packed24_unsigned(bytecode_and_packed_arg: i32) -> u32 {
    (bytecode_and_packed_arg as u32) >> BYTECODE_SHIFT
}

struct BacktrackStack {
    data: Vec<i32>,
}

impl BacktrackStack {
    const K_MAX_SIZE: usize = 4096 / mem::size_of::<i32>(); //RegExpStack::kMaximumStackSize / sizeof(ValueT);

    fn new() -> Self {
        BacktrackStack { data: Vec::new() }
    }

    fn push(&mut self, v: i32) -> bool {
        self.data.push(v);
        self.data.len() <= Self::K_MAX_SIZE
    }

    fn peek(&self) -> i32 {
        *self.data.last().expect("Stack is empty")
    }

    fn pop(&mut self) -> i32 {
        self.data.pop().expect("Stack is empty")
    }

    fn sp(&self) -> usize {
        self.data.len()
    }

    fn set_sp(&mut self, new_sp: usize) {
        assert!(new_sp <= self.sp());
        self.data.resize(new_sp, 0); // Pad with 0, though it's technically unused.
    }
}

struct InterpreterRegisters {
    registers: Vec<i32>,
    output_registers: *mut i32,
    total_register_count: i32,
    output_register_count: i32,
}

impl InterpreterRegisters {
    fn new(total_register_count: i32, output_registers: *mut i32, output_register_count: i32) -> Self {
        assert!(output_register_count >= 2);
        assert!(total_register_count >= output_register_count);
        assert!(total_register_count <= 64); //RegExpMacroAssembler::kMaxRegisterCount);
        assert!(!output_registers.is_null());

        let mut registers = vec![0; total_register_count as usize];
        // Initialize the output register region to -1 signifying 'no match'.
        for i in 0..output_register_count as usize {
            registers[i] = -1;
        }

        InterpreterRegisters {
            registers,
            output_registers,
            total_register_count,
            output_register_count,
        }
    }

    fn get(&self, index: usize) -> &i32 {
        assert!(index < self.total_register_count as usize);
        &self.registers[index]
    }

    fn get_mut(&mut self, index: usize) -> &mut i32 {
        assert!(index < self.total_register_count as usize);
        &mut self.registers[index]
    }

    fn copy_to_output_registers(&self) {
        unsafe {
            mock::memcopy(
                self.output_registers,
                self.registers.as_ptr(),
                self.output_register_count as usize,
            );
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum IrregexpInterpreterResult {
    SUCCESS,
    FAILURE,
    EXCEPTION,
    RETRY,
    FALLBACK_TO_EXPERIMENTAL, // Not directly convertible; placeholder.
}

#[allow(dead_code)]
fn throw_stack_overflow(isolate: &mock::Isolate, call_origin: mock::RegExp::CallOrigin) -> IrregexpInterpreterResult {
    assert!(call_origin == mock::RegExp::CallOrigin::kFromRuntime);
    // We abort interpreter execution after the stack overflow is thrown, and thus
    // allow allocation here despite the outer DisallowGarbageCollectionScope.
    isolate.stack_overflow();
    IrregexpInterpreterResult::EXCEPTION
}

#[allow(dead_code)]
fn maybe_throw_stack_overflow(isolate: &mock::Isolate, call_origin: mock::RegExp::CallOrigin) -> IrregexpInterpreterResult {
    if call_origin == mock::RegExp::CallOrigin::kFromRuntime {
        throw_stack_overflow(isolate, call_origin)
    } else {
        IrregexpInterpreterResult::EXCEPTION
    }
}

#[allow(dead_code)]
fn back_ref_matches_no_case<T: PartialEq>(
    isolate: &mock::Isolate,
    from: usize,
    current: usize,
    len: usize,
    subject: &Vec<T>,
    unicode: bool,
) -> bool {
    if unicode {
        //TODO implement RegExpMacroAssembler::CaseInsensitiveCompareUnicode
        true //mock::RegExpMacroAssembler::case_insensitive_compare_unicode(offset_a, offset_b, length, isolate) == 1
    } else {
        //TODO implement RegExpMacroAssembler::CaseInsensitiveCompareNonUnicode
        true //mock::RegExpMacroAssembler::case_insensitive_compare_non_unicode(offset_a, offset_b, length, isolate) == 1
    }
}

#[allow(dead_code)]
fn back_ref_matches_no_case_u8(
    isolate: &mock::Isolate,
    from: usize,
    current: usize,
    len: usize,
    subject: &Vec<u8>,
    unicode: bool,
) -> bool {
    // For Latin1 characters the unicode flag makes no difference.
    let mut from_idx = from;
    let mut current_idx = current;
    for _ in 0..len {
        let old_char = subject[from_idx];
        from_idx += 1;
        let new_char = subject[current_idx];
        current_idx += 1;
        if old_char == new_char {
            continue;
        }
        // Convert both characters to lower case.
        let mut old_char = old_char as u32;
        let mut new_char = new_char as u32;
        old_char |= 0x20;
        new_char |= 0x20;
        if old_char != new_char {
            return false;
        }
        // Not letters in the ASCII range and Latin-1 range.
        if !(old_char >= 'a' as u32 && old_char <= 'z' as u32)
            && !(old_char >= 224 && old_char <= 254 && old_char != 247)
        {
            return false;
        }
    }
    true
}

#[allow(dead_code)]
fn handle_interrupts<Char>(
    isolate: &mock::Isolate,
    call_origin: mock::RegExp::CallOrigin,
    code_array_out: &mut mock::TrustedByteArray,
    subject_string_out: &mut mock::String,
    code_base_out: &mut *const u8,
    subject_string_vector_out: &mut Vec<Char>,
    pc_out: &mut *const u8,
) -> IrregexpInterpreterResult {
    //DisallowGarbageCollection no_gc;

    //mock::StackLimitCheck check(isolate);
    //bool js_has_overflowed = check.JsHasOverflowed();
    let js_has_overflowed = false;

    if call_origin == mock::RegExp::CallOrigin::kFromJs {
        // Direct calls from JavaScript can be interrupted in two ways:
        // 1. A real stack overflow, in which case we let the caller throw the
        //    exception.
        // 2. The stack guard was used to interrupt execution for another purpose,
        //    forcing the call through the runtime system.
        if js_has_overflowed {
            return IrregexpInterpreterResult::EXCEPTION;
        } else if isolate.stack_guard().interrupt_requested() {
            return IrregexpInterpreterResult::RETRY;
        }
    } else {
        assert!(call_origin == mock::RegExp::CallOrigin::kFromRuntime);
        // Prepare for possible GC.
        //mock::HandleScope handles(isolate);
        //mock::DirectHandle<mock::TrustedByteArray> code_handle(*code_array_out, isolate);
        //mock::DirectHandle<mock::String> subject_handle(*subject_string_out, isolate);

        if js_has_overflowed {
            return throw_stack_overflow(isolate, call_origin);
        } else if isolate.stack_guard().interrupt_requested() {
            //const bool was_one_byte = mock::String::IsOneByteRepresentationUnderneath(*subject_string_out);
            let was_one_byte = true;

            let result = isolate.stack_guard().handle_interrupts();

            if mock::is_exception(result) {
                return IrregexpInterpreterResult::EXCEPTION;
            }

            // If we changed between a LATIN1 and a UC16 string, we need to
            // restart regexp matching with the appropriate template instantiation of
            // RawMatch.
            //if mock::String::IsOneByteRepresentationUnderneath(*subject_handle) != was_one_byte {
            if true != was_one_byte {
                return IrregexpInterpreterResult::RETRY;
            }
            // TODO: Implement update_code_and_subject_references if needed
            // UpdateCodeAndSubjectReferences(
            //     isolate, code_handle, subject_handle, code_array_out, code_base_out,
            //     pc_out, subject_string_out, subject_string_vector_out);
        }
    }

    IrregexpInterpreterResult::SUCCESS
}

#[allow(dead_code)]
fn check_bit_in_table(current_char: u32, table: *const u8) -> bool {
    let mask = 255; //RegExpMacroAssembler::kTableMask;
    let b = unsafe { *table.add((current_char & mask) as usize >> 3) }; //kBitsPerByteLog2) };
    let bit = (current_char & 7) as u8; //(kBitsPerByte - 1)) as u8;
    (b & (1 << bit)) != 0
}

#[allow(dead_code)]
fn index_is_in_bounds(index: i32, length: usize) -> bool {
    assert!(length >= 0);
    (index as usize) < length
}

macro_rules! advance {
    ($name:tt, $next_pc:ident, $pc:ident, $code_base:ident) => {
        $next_pc = $pc.add(regexp_bytecode_length($name) as usize);
    };
}

macro_rules! set_pc_from_offset {
    ($offset:expr, $next_pc:ident, $code_base:ident) => {
        $next_pc = $code_base.add($offset as usize);
    };
}

macro_rules! set_current_position {
    ($value:expr, $current:ident, $subject:ident) => {
        $current = $value;
        assert!($current >= 0 && $current <= $subject.len() as i32);
    };
}

macro_rules! advance_current_position {
    ($by:expr, $current:ident) => {
        $current += $by;
    };
}

#[allow(dead_code)]
fn raw_match<Char: PartialEq + Copy>(
    isolate: &mock::Isolate,
    code_array: &mut mock::TrustedByteArray,
    subject_string: &mut mock::String,
    subject: &Vec<Char>,
    output_registers: *mut i32,
    output_register_count: i32,
    total_register_count: i32,
    mut current: i32,
    mut current_char: u32,
    call_origin: mock::RegExp::CallOrigin,
    backtrack_limit: u32,
) -> IrregexpInterpreterResult {
    let code_base = unsafe { (*code_array).begin() };
    let mut pc = code_base;

    let mut registers = InterpreterRegisters::new(total_register_count, output_registers, output_register_count);
    let mut backtrack_stack = BacktrackStack::new();

    let mut backtrack_count: u32 = 0;

    // Define dispatch table and associated logic
    #[cfg(all(V8_HAS_COMPUTED_GOTO, V8_ENABLE_REGEXP_INTERPRETER_THREADED_DISPATCH))]
    {
        // Make sure every bytecode we get by using BYTECODE_MASK is well defined.
        assert!(kRegExpBytecodeCount <= kRegExpPaddedBytecodeCount);
        assert!(kRegExpBytecodeCount + kRegExpBytecodeFillerCount == kRegExpPaddedBytecodeCount);

        const DISPATCH_TABLE: [fn(); kRegExpPaddedBytecodeCount] = [
            || {}, // BC_BREAK,
            || {}, // BC_PUSH_CP,
            || {}, // BC_PUSH_BT,
            || {}, // BC_PUSH_REGISTER,
            || {}, // BC_SET_REGISTER,
            || {}, // BC_ADVANCE_REGISTER,
            || {}, // BC_SET_REGISTER_TO_CP,
            || {}, // BC_SET_CP_TO_REGISTER,
            || {}, // BC_SET_REGISTER_TO_SP,
            || {}, // BC_SET_SP_TO_REGISTER,
            || {}, // BC_POP_CP,
            || {}, // BC_POP_BT,
            || {}, // BC_POP_REGISTER,
            || {}, // BC_FAIL,
            || {}, // BC_SUCCEED,
            || {}, // BC_ADVANCE_CP,
            || {}, // BC_GOTO,
            || {}, // BC_ADVANCE_CP_AND_GOTO,
            || {}, // BC_CHECK_GREEDY,
            || {}, // BC_LOAD_CURRENT_CHAR,
            || {}, // BC_LOAD_CURRENT_CHAR_UNCHECKED,
            || {}, // BC_LOAD_2_CURRENT_CHARS,
            || {}, // BC_LOAD_2_CURRENT_CHARS_UNCHECKED,
            || {}, // BC_LOAD_4_CURRENT_CHARS,
            || {}, // BC_LOAD_4_CURRENT_CHARS_UNCHECKED,
            || {}, // BC_CHECK_4_CHARS,
            || {}, // BC_CHECK_CHAR,
            || {}, // BC_CHECK_NOT_4_CHARS,
            || {}, // BC_CHECK_NOT_CHAR,
            || {}, // BC_AND_CHECK_4_CHARS,
            || {}, // BC_AND_CHECK_CHAR,
            || {}, // BC_AND_CHECK_NOT_4_CHARS,
            || {}, // BC_AND_CHECK_NOT_CHAR,
            || {}, // BC_MINUS_AND_CHECK_NOT_CHAR,
            || {}, // BC_CHECK_CHAR_IN_RANGE,
            || {}, // BC_CHECK_CHAR_NOT_IN_RANGE,
            || {}, // BC_CHECK_BIT_IN_TABLE,
            || {}, // BC_CHECK_LT,
            || {}, // BC_CHECK_GT,
            || {}, // BC_CHECK_REGISTER_LT,
            || {}, // BC_CHECK_REGISTER_GE,
            || {}, // BC_CHECK_REGISTER_EQ_POS,
            || {}, // BC_CHECK_NOT_REGS_EQUAL,
            || {}, // BC_CHECK_NOT_BACK_REF,
            || {}, // BC_CHECK_NOT_BACK_REF_BACKWARD,
            || {}, // BC_CHECK_NOT_BACK_REF_NO_CASE_UNICODE,
            || {}, // BC_CHECK_NOT_BACK_REF_NO_CASE,
            || {}, // BC_CHECK_NOT_BACK_REF_NO_CASE_UNICODE_BACKWARD,
            || {}, // BC_CHECK_NOT_BACK_REF_NO_CASE_BACKWARD,
            || {}, // BC_CHECK_AT_START,
            || {}, // BC_CHECK_NOT_AT_START,
            || {}, // BC_SET_CURRENT_POSITION_FROM_END,
            || {}, // BC_CHECK_CURRENT_POSITION,
            || {}, // BC_SKIP_UNTIL_CHAR,
            || {}, // BC_SKIP_UNTIL_CHAR_AND,
            || {}, // BC_SKIP_UNTIL_CHAR_POS_CHECKED,
            || {}, // BC_SKIP_UNTIL_BIT_IN_TABLE,
            || {}, // BC_SKIP_UNTIL_GT_OR_NOT_BIT_IN_TABLE,
            || {}, // BC_SKIP_UNTIL_CHAR_OR_CHAR,
            || {}, // BC_BREAK (FILLER),
            || {}, // BC_BREAK (FILLER),
            || {}, // BC_BREAK (FILLER),
            || {}, // BC_BREAK (FILLER),
            || {}, // BC_BREAK (FILLER),
        ];
    }

    let mut next_pc: *const u8;
    let mut insn: i32;
    let mut next_insn: i32;

    loop {
        let pc_offset = unsafe { pc.offset_from(code_base) };
        let bytecode = unsafe { *(pc as *const i32) & BYTECODE_MASK };

        #[cfg(all(V8_HAS_COMPUTED_GOTO, V8_ENABLE_REGEXP_INTERPRETER_THREADED_DISPATCH))]
        {
            // Compute next instruction and handler address.
            next_insn = load32_aligned(next_pc);
            //TODO Implement dispatch_table
            //let next_handler_addr = dispatch_table[next_insn as usize & BYTECODE_MASK as usize];

            // Go to next handler
            pc = next_pc;
            insn = next_insn;
            //goto* next_handler_addr
        }

        #[cfg(not(all(V8_HAS_COMPUTED_GOTO, V8_ENABLE_REGEXP_INTERPRETER_THREADED_DISPATCH)))]
        {
            insn = load32_aligned(pc);
            match insn & BYTECODE_MASK {
                BC_BREAK => {
                    unreachable!();
                }
                BC_PUSH_CP => {
                    advance!(BC_PUSH_CP, next_pc, pc, code_base);
                    if !backtrack_stack.push(current) {
                        return maybe_throw_stack_overflow(isolate, call_origin);
                    }
                    pc = next_pc;
                    continue;
                }
                BC_PUSH_BT => {
                    advance!(BC_PUSH_BT, next_pc, pc, code_base);
                    let offset = load32_aligned(unsafe { pc.add(4) });
                    if !backtrack_stack.push(offset) {
                        return maybe_throw_stack_overflow(isolate, call_origin);
                    }
                    pc = next_pc;
                    continue;
