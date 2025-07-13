// Converted from V8 C++ source files:
// Header: regexp-bytecodes.h
// Implementation: regexp-bytecodes.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/regexp/regexp-bytecodes.rs
#![allow(non_camel_case_types)]

use std::fmt;

const kRegExpPaddedBytecodeCount: usize = 1 << 6;
const BYTECODE_MASK: usize = kRegExpPaddedBytecodeCount - 1;
const MAX_FIRST_ARG: u32 = 0x7fffffu;
const BYTECODE_SHIFT: i32 = 8;

// NOTE: The `uc16` type is assumed to be `u16` as it's used for characters.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegExpBytecode {
    BREAK,
    PUSH_CP,
    PUSH_BT,
    PUSH_REGISTER,
    SET_REGISTER_TO_CP,
    SET_CP_TO_REGISTER,
    SET_REGISTER_TO_SP,
    SET_SP_TO_REGISTER,
    SET_REGISTER,
    ADVANCE_REGISTER,
    POP_CP,
    POP_BT,
    POP_REGISTER,
    FAIL,
    SUCCEED,
    ADVANCE_CP,
    GOTO,
    LOAD_CURRENT_CHAR,
    LOAD_CURRENT_CHAR_UNCHECKED,
    LOAD_2_CURRENT_CHARS,
    LOAD_2_CURRENT_CHARS_UNCHECKED,
    LOAD_4_CURRENT_CHARS,
    LOAD_4_CURRENT_CHARS_UNCHECKED,
    CHECK_4_CHARS,
    CHECK_CHAR,
    CHECK_NOT_4_CHARS,
    CHECK_NOT_CHAR,
    AND_CHECK_4_CHARS,
    AND_CHECK_CHAR,
    AND_CHECK_NOT_4_CHARS,
    AND_CHECK_NOT_CHAR,
    MINUS_AND_CHECK_NOT_CHAR,
    CHECK_CHAR_IN_RANGE,
    CHECK_CHAR_NOT_IN_RANGE,
    CHECK_BIT_IN_TABLE,
    CHECK_LT,
    CHECK_GT,
    CHECK_NOT_BACK_REF,
    CHECK_NOT_BACK_REF_NO_CASE,
    CHECK_NOT_BACK_REF_NO_CASE_UNICODE,
    CHECK_NOT_BACK_REF_BACKWARD,
    CHECK_NOT_BACK_REF_NO_CASE_BACKWARD,
    CHECK_NOT_BACK_REF_NO_CASE_UNICODE_BACKWARD,
    CHECK_NOT_REGS_EQUAL,
    CHECK_REGISTER_LT,
    CHECK_REGISTER_GE,
    CHECK_REGISTER_EQ_POS,
    CHECK_AT_START,
    CHECK_NOT_AT_START,
    CHECK_GREEDY,
    ADVANCE_CP_AND_GOTO,
    SET_CURRENT_POSITION_FROM_END,
    CHECK_CURRENT_POSITION,
    SKIP_UNTIL_BIT_IN_TABLE,
    SKIP_UNTIL_CHAR_AND,
    SKIP_UNTIL_CHAR,
    SKIP_UNTIL_CHAR_POS_CHECKED,
    SKIP_UNTIL_CHAR_OR_CHAR,
    SKIP_UNTIL_GT_OR_NOT_BIT_IN_TABLE,
}

impl RegExpBytecode {
    pub fn code(&self) -> i32 {
        match self {
            RegExpBytecode::BREAK => 0,
            RegExpBytecode::PUSH_CP => 1,
            RegExpBytecode::PUSH_BT => 2,
            RegExpBytecode::PUSH_REGISTER => 3,
            RegExpBytecode::SET_REGISTER_TO_CP => 4,
            RegExpBytecode::SET_CP_TO_REGISTER => 5,
            RegExpBytecode::SET_REGISTER_TO_SP => 6,
            RegExpBytecode::SET_SP_TO_REGISTER => 7,
            RegExpBytecode::SET_REGISTER => 8,
            RegExpBytecode::ADVANCE_REGISTER => 9,
            RegExpBytecode::POP_CP => 10,
            RegExpBytecode::POP_BT => 11,
            RegExpBytecode::POP_REGISTER => 12,
            RegExpBytecode::FAIL => 13,
            RegExpBytecode::SUCCEED => 14,
            RegExpBytecode::ADVANCE_CP => 15,
            RegExpBytecode::GOTO => 16,
            RegExpBytecode::LOAD_CURRENT_CHAR => 17,
            RegExpBytecode::LOAD_CURRENT_CHAR_UNCHECKED => 18,
            RegExpBytecode::LOAD_2_CURRENT_CHARS => 19,
            RegExpBytecode::LOAD_2_CURRENT_CHARS_UNCHECKED => 20,
            RegExpBytecode::LOAD_4_CURRENT_CHARS => 21,
            RegExpBytecode::LOAD_4_CURRENT_CHARS_UNCHECKED => 22,
            RegExpBytecode::CHECK_4_CHARS => 23,
            RegExpBytecode::CHECK_CHAR => 24,
            RegExpBytecode::CHECK_NOT_4_CHARS => 25,
            RegExpBytecode::CHECK_NOT_CHAR => 26,
            RegExpBytecode::AND_CHECK_4_CHARS => 27,
            RegExpBytecode::AND_CHECK_CHAR => 28,
            RegExpBytecode::AND_CHECK_NOT_4_CHARS => 29,
            RegExpBytecode::AND_CHECK_NOT_CHAR => 30,
            RegExpBytecode::MINUS_AND_CHECK_NOT_CHAR => 31,
            RegExpBytecode::CHECK_CHAR_IN_RANGE => 32,
            RegExpBytecode::CHECK_CHAR_NOT_IN_RANGE => 33,
            RegExpBytecode::CHECK_BIT_IN_TABLE => 34,
            RegExpBytecode::CHECK_LT => 35,
            RegExpBytecode::CHECK_GT => 36,
            RegExpBytecode::CHECK_NOT_BACK_REF => 37,
            RegExpBytecode::CHECK_NOT_BACK_REF_NO_CASE => 38,
            RegExpBytecode::CHECK_NOT_BACK_REF_NO_CASE_UNICODE => 39,
            RegExpBytecode::CHECK_NOT_BACK_REF_BACKWARD => 40,
            RegExpBytecode::CHECK_NOT_BACK_REF_NO_CASE_BACKWARD => 41,
            RegExpBytecode::CHECK_NOT_BACK_REF_NO_CASE_UNICODE_BACKWARD => 42,
            RegExpBytecode::CHECK_NOT_REGS_EQUAL => 43,
            RegExpBytecode::CHECK_REGISTER_LT => 44,
            RegExpBytecode::CHECK_REGISTER_GE => 45,
            RegExpBytecode::CHECK_REGISTER_EQ_POS => 46,
            RegExpBytecode::CHECK_AT_START => 47,
            RegExpBytecode::CHECK_NOT_AT_START => 48,
            RegExpBytecode::CHECK_GREEDY => 49,
            RegExpBytecode::ADVANCE_CP_AND_GOTO => 50,
            RegExpBytecode::SET_CURRENT_POSITION_FROM_END => 51,
            RegExpBytecode::CHECK_CURRENT_POSITION => 52,
            RegExpBytecode::SKIP_UNTIL_BIT_IN_TABLE => 53,
            RegExpBytecode::SKIP_UNTIL_CHAR_AND => 54,
            RegExpBytecode::SKIP_UNTIL_CHAR => 55,
            RegExpBytecode::SKIP_UNTIL_CHAR_POS_CHECKED => 56,
            RegExpBytecode::SKIP_UNTIL_CHAR_OR_CHAR => 57,
            RegExpBytecode::SKIP_UNTIL_GT_OR_NOT_BIT_IN_TABLE => 58,
        }
    }

    pub fn byte_length(&self) -> usize {
        match self {
            RegExpBytecode::BREAK => 4,
            RegExpBytecode::PUSH_CP => 4,
            RegExpBytecode::PUSH_BT => 8,
            RegExpBytecode::PUSH_REGISTER => 4,
            RegExpBytecode::SET_REGISTER_TO_CP => 8,
            RegExpBytecode::SET_CP_TO_REGISTER => 4,
            RegExpBytecode::SET_REGISTER_TO_SP => 4,
            RegExpBytecode::SET_SP_TO_REGISTER => 4,
            RegExpBytecode::SET_REGISTER => 8,
            RegExpBytecode::ADVANCE_REGISTER => 8,
            RegExpBytecode::POP_CP => 4,
            RegExpBytecode::POP_BT => 4,
            RegExpBytecode::POP_REGISTER => 4,
            RegExpBytecode::FAIL => 4,
            RegExpBytecode::SUCCEED => 4,
            RegExpBytecode::ADVANCE_CP => 4,
            RegExpBytecode::GOTO => 8,
            RegExpBytecode::LOAD_CURRENT_CHAR => 8,
            RegExpBytecode::LOAD_CURRENT_CHAR_UNCHECKED => 4,
            RegExpBytecode::LOAD_2_CURRENT_CHARS => 8,
            RegExpBytecode::LOAD_2_CURRENT_CHARS_UNCHECKED => 4,
            RegExpBytecode::LOAD_4_CURRENT_CHARS => 8,
            RegExpBytecode::LOAD_4_CURRENT_CHARS_UNCHECKED => 4,
            RegExpBytecode::CHECK_4_CHARS => 12,
            RegExpBytecode::CHECK_CHAR => 8,
            RegExpBytecode::CHECK_NOT_4_CHARS => 12,
            RegExpBytecode::CHECK_NOT_CHAR => 8,
            RegExpBytecode::AND_CHECK_4_CHARS => 16,
            RegExpBytecode::AND_CHECK_CHAR => 12,
            RegExpBytecode::AND_CHECK_NOT_4_CHARS => 16,
            RegExpBytecode::AND_CHECK_NOT_CHAR => 12,
            RegExpBytecode::MINUS_AND_CHECK_NOT_CHAR => 12,
            RegExpBytecode::CHECK_CHAR_IN_RANGE => 12,
            RegExpBytecode::CHECK_CHAR_NOT_IN_RANGE => 12,
            RegExpBytecode::CHECK_BIT_IN_TABLE => 24,
            RegExpBytecode::CHECK_LT => 8,
            RegExpBytecode::CHECK_GT => 8,
            RegExpBytecode::CHECK_NOT_BACK_REF => 8,
            RegExpBytecode::CHECK_NOT_BACK_REF_NO_CASE => 8,
            RegExpBytecode::CHECK_NOT_BACK_REF_NO_CASE_UNICODE => 8,
            RegExpBytecode::CHECK_NOT_BACK_REF_BACKWARD => 8,
            RegExpBytecode::CHECK_NOT_BACK_REF_NO_CASE_BACKWARD => 8,
            RegExpBytecode::CHECK_NOT_BACK_REF_NO_CASE_UNICODE_BACKWARD => 8,
            RegExpBytecode::CHECK_NOT_REGS_EQUAL => 12,
            RegExpBytecode::CHECK_REGISTER_LT => 12,
            RegExpBytecode::CHECK_REGISTER_GE => 12,
            RegExpBytecode::CHECK_REGISTER_EQ_POS => 8,
            RegExpBytecode::CHECK_AT_START => 8,
            RegExpBytecode::CHECK_NOT_AT_START => 8,
            RegExpBytecode::CHECK_GREEDY => 8,
            RegExpBytecode::ADVANCE_CP_AND_GOTO => 8,
            RegExpBytecode::SET_CURRENT_POSITION_FROM_END => 4,
            RegExpBytecode::CHECK_CURRENT_POSITION => 8,
            RegExpBytecode::SKIP_UNTIL_BIT_IN_TABLE => 32,
            RegExpBytecode::SKIP_UNTIL_CHAR_AND => 24,
            RegExpBytecode::SKIP_UNTIL_CHAR => 16,
            RegExpBytecode::SKIP_UNTIL_CHAR_POS_CHECKED => 20,
            RegExpBytecode::SKIP_UNTIL_CHAR_OR_CHAR => 20,
            RegExpBytecode::SKIP_UNTIL_GT_OR_NOT_BIT_IN_TABLE => 32,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            RegExpBytecode::BREAK => "BREAK",
            RegExpBytecode::PUSH_CP => "PUSH_CP",
            RegExpBytecode::PUSH_BT => "PUSH_BT",
            RegExpBytecode::PUSH_REGISTER => "PUSH_REGISTER",
            RegExpBytecode::SET_REGISTER_TO_CP => "SET_REGISTER_TO_CP",
            RegExpBytecode::SET_CP_TO_REGISTER => "SET_CP_TO_REGISTER",
            RegExpBytecode::SET_REGISTER_TO_SP => "SET_REGISTER_TO_SP",
            RegExpBytecode::SET_SP_TO_REGISTER => "SET_SP_TO_REGISTER",
            RegExpBytecode::SET_REGISTER => "SET_REGISTER",
            RegExpBytecode::ADVANCE_REGISTER => "ADVANCE_REGISTER",
            RegExpBytecode::POP_CP => "POP_CP",
            RegExpBytecode::POP_BT => "POP_BT",
            RegExpBytecode::POP_REGISTER => "POP_REGISTER",
            RegExpBytecode::FAIL => "FAIL",
            RegExpBytecode::SUCCEED => "SUCCEED",
            RegExpBytecode::ADVANCE_CP => "ADVANCE_CP",
            RegExpBytecode::GOTO => "GOTO",
            RegExpBytecode::LOAD_CURRENT_CHAR => "LOAD_CURRENT_CHAR",
            RegExpBytecode::LOAD_CURRENT_CHAR_UNCHECKED => "LOAD_CURRENT_CHAR_UNCHECKED",
            RegExpBytecode::LOAD_2_CURRENT_CHARS => "LOAD_2_CURRENT_CHARS",
            RegExpBytecode::LOAD_2_CURRENT_CHARS_UNCHECKED => "LOAD_2_CURRENT_CHARS_UNCHECKED",
            RegExpBytecode::LOAD_4_CURRENT_CHARS => "LOAD_4_CURRENT_CHARS",
            RegExpBytecode::LOAD_4_CURRENT_CHARS_UNCHECKED => "LOAD_4_CURRENT_CHARS_UNCHECKED",
            RegExpBytecode::CHECK_4_CHARS => "CHECK_4_CHARS",
            RegExpBytecode::CHECK_CHAR => "CHECK_CHAR",
            RegExpBytecode::CHECK_NOT_4_CHARS => "CHECK_NOT_4_CHARS",
            RegExpBytecode::CHECK_NOT_CHAR => "CHECK_NOT_CHAR",
            RegExpBytecode::AND_CHECK_4_CHARS => "AND_CHECK_4_CHARS",
            RegExpBytecode::AND_CHECK_CHAR => "AND_CHECK_CHAR",
            RegExpBytecode::AND_CHECK_NOT_4_CHARS => "AND_CHECK_NOT_4_CHARS",
            RegExpBytecode::AND_CHECK_NOT_CHAR => "AND_CHECK_NOT_CHAR",
            RegExpBytecode::MINUS_AND_CHECK_NOT_CHAR => "MINUS_AND_CHECK_NOT_CHAR",
            RegExpBytecode::CHECK_CHAR_IN_RANGE => "CHECK_CHAR_IN_RANGE",
            RegExpBytecode::CHECK_CHAR_NOT_IN_RANGE => "CHECK_CHAR_NOT_IN_RANGE",
            RegExpBytecode::CHECK_BIT_IN_TABLE => "CHECK_BIT_IN_TABLE",
            RegExpBytecode::CHECK_LT => "CHECK_LT",
            RegExpBytecode::CHECK_GT => "CHECK_GT",
            RegExpBytecode::CHECK_NOT_BACK_REF => "CHECK_NOT_BACK_REF",
            RegExpBytecode::CHECK_NOT_BACK_REF_NO_CASE => "CHECK_NOT_BACK_REF_NO_CASE",
            RegExpBytecode::CHECK_NOT_BACK_REF_NO_CASE_UNICODE => "CHECK_NOT_BACK_REF_NO_CASE_UNICODE",
            RegExpBytecode::CHECK_NOT_BACK_REF_BACKWARD => "CHECK_NOT_BACK_REF_BACKWARD",
            RegExpBytecode::CHECK_NOT_BACK_REF_NO_CASE_BACKWARD => "CHECK_NOT_BACK_REF_NO_CASE_BACKWARD",
            RegExpBytecode::CHECK_NOT_BACK_REF_NO_CASE_UNICODE_BACKWARD => "CHECK_NOT_BACK_REF_NO_CASE_UNICODE_BACKWARD",
            RegExpBytecode::CHECK_NOT_REGS_EQUAL => "CHECK_NOT_REGS_EQUAL",
            RegExpBytecode::CHECK_REGISTER_LT => "CHECK_REGISTER_LT",
            RegExpBytecode::CHECK_REGISTER_GE => "CHECK_REGISTER_GE",
            RegExpBytecode::CHECK_REGISTER_EQ_POS => "CHECK_REGISTER_EQ_POS",
            RegExpBytecode::CHECK_AT_START => "CHECK_AT_START",
            RegExpBytecode::CHECK_NOT_AT_START => "CHECK_NOT_AT_START",
            RegExpBytecode::CHECK_GREEDY => "CHECK_GREEDY",
            RegExpBytecode::ADVANCE_CP_AND_GOTO => "ADVANCE_CP_AND_GOTO",
            RegExpBytecode::SET_CURRENT_POSITION_FROM_END => "SET_CURRENT_POSITION_FROM_END",
            RegExpBytecode::CHECK_CURRENT_POSITION => "CHECK_CURRENT_POSITION",
            RegExpBytecode::SKIP_UNTIL_BIT_IN_TABLE => "SKIP_UNTIL_BIT_IN_TABLE",
            RegExpBytecode::SKIP_UNTIL_CHAR_AND => "SKIP_UNTIL_CHAR_AND",
            RegExpBytecode::SKIP_UNTIL_CHAR => "SKIP_UNTIL_CHAR",
            RegExpBytecode::SKIP_UNTIL_CHAR_POS_CHECKED => "SKIP_UNTIL_CHAR_POS_CHECKED",
            RegExpBytecode::SKIP_UNTIL_CHAR_OR_CHAR => "SKIP_UNTIL_CHAR_OR_CHAR",
            RegExpBytecode::SKIP_UNTIL_GT_OR_NOT_BIT_IN_TABLE => "SKIP_UNTIL_GT_OR_NOT_BIT_IN_TABLE",
        }
    }
}

const kRegExpBytecodeLengths: [usize; 59] = [
    4, 4, 8, 4, 8, 4, 4, 4, 8, 8, 4, 4, 4, 4, 4, 4, 8, 8, 4, 8, 4, 8, 4, 12, 8, 12, 8, 16, 12, 16,
    12, 12, 12, 24, 8, 8, 8, 8, 8, 8, 8, 12, 12, 12, 8, 8, 8, 8, 8, 4, 8, 32, 24, 16, 20, 20, 32,
];

pub fn regexp_bytecode_length(bytecode: i32) -> usize {
    if bytecode < 0 || bytecode >= kRegExpBytecodeCount as i32 {
        panic!("Bytecode out of range");
    }
    kRegExpBytecodeLengths[bytecode as usize]
}

const kRegExpBytecodeNames: [&str; 59] = [
    "BREAK",
    "PUSH_CP",
    "PUSH_BT",
    "PUSH_REGISTER",
    "SET_REGISTER_TO_CP",
    "SET_CP_TO_REGISTER",
    "SET_REGISTER_TO_SP",
    "SET_SP_TO_REGISTER",
    "SET_REGISTER",
    "ADVANCE_REGISTER",
    "POP_CP",
    "POP_BT",
    "POP_REGISTER",
    "FAIL",
    "SUCCEED",
    "ADVANCE_CP",
    "GOTO",
    "LOAD_CURRENT_CHAR",
    "LOAD_CURRENT_CHAR_UNCHECKED",
    "LOAD_2_CURRENT_CHARS",
    "LOAD_2_CURRENT_CHARS_UNCHECKED",
    "LOAD_4_CURRENT_CHARS",
    "LOAD_4_CURRENT_CHARS_UNCHECKED",
    "CHECK_4_CHARS",
    "CHECK_CHAR",
    "CHECK_NOT_4_CHARS",
    "CHECK_NOT_CHAR",
    "AND_CHECK_4_CHARS",
    "AND_CHECK_CHAR",
    "AND_CHECK_NOT_4_CHARS",
    "AND_CHECK_NOT_CHAR",
    "MINUS_AND_CHECK_NOT_CHAR",
    "CHECK_CHAR_IN_RANGE",
    "CHECK_CHAR_NOT_IN_RANGE",
    "CHECK_BIT_IN_TABLE",
    "CHECK_LT",
    "CHECK_GT",
    "CHECK_NOT_BACK_REF",
    "CHECK_NOT_BACK_REF_NO_CASE",
    "CHECK_NOT_BACK_REF_NO_CASE_UNICODE",
    "CHECK_NOT_BACK_REF_BACKWARD",
    "CHECK_NOT_BACK_REF_NO_CASE_BACKWARD",
    "CHECK_NOT_BACK_REF_NO_CASE_UNICODE_BACKWARD",
    "CHECK_NOT_REGS_EQUAL",
    "CHECK_REGISTER_LT",
    "CHECK_REGISTER_GE",
    "CHECK_REGISTER_EQ_POS",
    "CHECK_AT_START",
    "CHECK_NOT_AT_START",
    "CHECK_GREEDY",
    "ADVANCE_CP_AND_GOTO",
    "SET_CURRENT_POSITION_FROM_END",
    "CHECK_CURRENT_POSITION",
    "SKIP_UNTIL_BIT_IN_TABLE",
    "SKIP_UNTIL_CHAR_AND",
    "SKIP_UNTIL_CHAR",
    "SKIP_UNTIL_CHAR_POS_CHECKED",
    "SKIP_UNTIL_CHAR_OR_CHAR",
    "SKIP_UNTIL_GT_OR_NOT_BIT_IN_TABLE",
];

pub fn regexp_bytecode_name(bytecode: i32) -> &'static str {
    if bytecode < 0 || bytecode >= kRegExpBytecodeCount as i32 {
        panic!("Bytecode out of range");
    }
    kRegExpBytecodeNames[bytecode as usize]
}

fn is_printable(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == 0x20 || (0x21..=0x7E).contains(&b)
}

pub fn regexp_bytecode_disassemble_single(code_base: &[u8], pc: &[u8]) {
    let bytecode = pc[0] as usize & BYTECODE_MASK;
	let regexp_bytecode = match bytecode {
		0 => RegExpBytecode::BREAK,
		1 => RegExpBytecode::PUSH_CP,
		2 => RegExpBytecode::PUSH_BT,
		3 => RegExpBytecode::PUSH_REGISTER,
		4 => RegExpBytecode::SET_REGISTER_TO_CP,
		5 => RegExpBytecode::SET_CP_TO_REGISTER,
		6 => RegExpBytecode::SET_REGISTER_TO_SP,
		7 => RegExpBytecode::SET_SP_TO_REGISTER,
		8 => RegExpBytecode::SET_REGISTER,
		9 => RegExpBytecode::ADVANCE_REGISTER,
		10 => RegExpBytecode::POP_CP,
		11 => RegExpBytecode::POP_BT,
		12 => RegExpBytecode::POP_REGISTER,
		13 => RegExpBytecode::FAIL,
		14 => RegExpBytecode::SUCCEED,
		15 => RegExpBytecode::ADVANCE_CP,
		16 => RegExpBytecode::GOTO,
		17 => RegExpBytecode::LOAD_CURRENT_CHAR,
		18 => RegExpBytecode::LOAD_CURRENT_CHAR_UNCHECKED,
		19 => RegExpBytecode::LOAD_2_CURRENT_CHARS,
		20 => RegExpBytecode::LOAD_2_CURRENT_CHARS_UNCHECKED,
		21 => RegExpBytecode::LOAD_4_CURRENT_CHARS,
		22 => RegExpBytecode::LOAD_4_CURRENT_CHARS_UNCHECKED,
		23 => RegExpBytecode::CHECK_4_CHARS,
		24 => RegExpBytecode::CHECK_CHAR,
		25 => RegExpBytecode::CHECK_NOT_4_CHARS,
		26 => RegExpBytecode::CHECK_NOT_CHAR,
		27 => RegExpBytecode::AND_CHECK_4_CHARS,
		28 => RegExpBytecode::AND_CHECK_CHAR,
		29 => RegExpBytecode::AND_CHECK_NOT_4_CHARS,
		30 => RegExpBytecode::AND_CHECK_NOT_CHAR,
		31 => RegExpBytecode::MINUS_AND_CHECK_NOT_CHAR,
		32 => RegExpBytecode::CHECK_CHAR_IN_RANGE,
		33 => RegExpBytecode::CHECK_CHAR_NOT_IN_RANGE,
		34 => RegExpBytecode::CHECK_BIT_IN_TABLE,
		35 => RegExpBytecode::CHECK_LT,
		36 => RegExpBytecode::CHECK_GT,
		37 => RegExpBytecode::CHECK_NOT_BACK_REF,
		38 => RegExpBytecode::CHECK_NOT_BACK_REF_NO_CASE,
		39 => RegExpBytecode::CHECK_NOT_BACK_REF_NO_CASE_UNICODE,
		40 => RegExpBytecode::CHECK_NOT_BACK_REF_BACKWARD,
		41 => RegExpBytecode::CHECK_NOT_BACK_REF_NO_CASE_BACKWARD,
		42 => RegExpBytecode::CHECK_NOT_BACK_REF_NO_CASE_UNICODE_BACKWARD,
		43 => RegExpBytecode::CHECK_NOT_REGS_EQUAL,
		44 => RegExpBytecode::CHECK_REGISTER_LT,
		45 => RegExpBytecode::CHECK_REGISTER_GE,
		46 => RegExpBytecode::CHECK_REGISTER_EQ_POS,
		47 => RegExpBytecode::CHECK_AT_START,
		48 => RegExpBytecode::CHECK_NOT_AT_START,
		49 => RegExpBytecode::CHECK_GREEDY,
		50 => RegExpBytecode::ADVANCE_CP_AND_GOTO,
		51 => RegExpBytecode::SET_CURRENT_POSITION_FROM_END,
		52 => RegExpBytecode::CHECK_CURRENT_POSITION,
		53 => RegExpBytecode::SKIP_UNTIL_BIT_IN_TABLE,
		54 => RegExpBytecode::SKIP_UNTIL_CHAR_AND,
		55 => RegExpBytecode::SKIP_UNTIL_CHAR,
		56 => RegExpBytecode::SKIP_UNTIL_CHAR_POS_CHECKED,
		57 => RegExpBytecode::SKIP_UNTIL_CHAR_OR_CHAR,
		58 => RegExpBytecode::SKIP_UNTIL_GT_OR_NOT_BIT_IN_TABLE,
		_ => panic!("Unknown bytecode"),
	};

	print!("{}", regexp_bytecode.name());

    for i in 0..regexp_bytecode.byte_length() {
        print!(", {:02x}", pc[i]);
    }
    print!(" ");

    for i in 1..regexp_bytecode.byte_length() {
        let b = pc[i];
        print!("{}", if is_printable(b) { b as char } else { '.' });
    }
    println!();
}

pub fn regexp_bytecode_disassemble(code_base: &[u8], length: usize, pattern: &str) {
    println!("[generated bytecode for regexp pattern: '{}']", pattern);

    let mut offset: usize = 0;

    while offset < length {
        let pc = &code_base[offset..];
        print!("{:p}  {:4}  ", pc.as_ptr(), offset);
        regexp_bytecode_disassemble_single(code_base, pc);
        let bytecode_value = code_base[offset] as usize & BYTECODE_MASK;
        offset += kRegExpBytecodeLengths[bytecode_value];
    }
}
