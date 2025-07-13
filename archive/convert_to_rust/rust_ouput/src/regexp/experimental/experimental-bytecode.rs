// Converted from V8 C++ source files:
// Header: experimental-bytecode.h
// Implementation: experimental-bytecode.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod experimental_bytecode {
    // Copyright 2020 the V8 project authors. All rights reserved.
    // Use of this source code is governed by a BSD-style license that can be
    // found in the LICENSE file.

    #![allow(non_camel_case_types)]

    use crate::regexp::regexp_ast::RegExpAssertion;
    use crate::regexp::regexp_ast::RegExpLookaround;
    use std::fmt;

    // Bytecode format.
    // Currently very simple fixed-size: The opcode is encoded in the first 4
    // bytes, the payload takes another 4 bytes.
    #[derive(Debug, Clone, Copy)]
    pub struct RegExpInstruction {
        pub opcode: Opcode,
        pub payload: Payload,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Opcode {
        ACCEPT,
        ASSERTION,
        CLEAR_REGISTER,
        CONSUME_RANGE,
        RANGE_COUNT,
        FORK,
        JMP,
        SET_REGISTER_TO_CP,
        SET_QUANTIFIER_TO_CLOCK,
        FILTER_QUANTIFIER,
        FILTER_GROUP,
        FILTER_LOOKAROUND,
        FILTER_CHILD,
        BEGIN_LOOP,
        END_LOOP,
        START_LOOKAROUND,
        END_LOOKAROUND,
        WRITE_LOOKAROUND_TABLE,
        READ_LOOKAROUND_TABLE,
    }

    #[derive(Debug, Clone, Copy)]
    pub union Payload {
        pub consume_range: Uc16Range,
        pub num_ranges: i32,
        pub pc: i32,
        pub register_index: i32,
        pub assertion_type: RegExpAssertion::Type,
        pub quantifier_id: i32,
        pub group_id: i32,
        pub lookaround_id: i32,
        pub lookaround: LookaroundPayload,
    }

    #[derive(Debug, Clone, Copy)]
    #[repr(C)]
    pub struct Uc16Range {
        pub min: u16, // Inclusive.
        pub max: u16, // Inclusive.
    }

    #[derive(Debug, Clone, Copy)]
    pub struct LookaroundPayload {
        payload_: u32,
    }

    impl LookaroundPayload {
        pub fn new() -> Self {
            LookaroundPayload { payload_: 0 }
        }
        pub fn new_with_params(
            lookaround_index: u32,
            is_positive: bool,
            type_: RegExpLookaround::Type,
        ) -> Self {
            let mut payload = 0;
            payload = Self::IsPositive::update(
                Self::LookaroundIndex::encode(lookaround_index),
                is_positive,
            );
            payload = Self::Type::update(payload, type_);
            LookaroundPayload { payload_: payload }
        }

        pub fn index(&self) -> u32 {
            Self::LookaroundIndex::decode(self.payload_)
        }
        pub fn is_positive(&self) -> bool {
            Self::IsPositive::decode(self.payload_)
        }
        pub fn type_(&self) -> RegExpLookaround::Type {
            Self::Type::decode(self.payload_)
        }

        type IsPositive = BitField<bool, 0, 1>;
        type Type = Self::IsPositive::Next<RegExpLookaround::Type, 1>;
        type LookaroundIndex = Self::Type::Next<u32, 30>;
    }

    impl RegExpInstruction {
        pub fn consume_range(min: u16, max: u16) -> Self {
            RegExpInstruction {
                opcode: Opcode::CONSUME_RANGE,
                payload: Payload {
                    consume_range: Uc16Range { min, max },
                },
            }
        }

        pub fn consume_any_char() -> Self {
            RegExpInstruction::consume_range(0x0000, 0xFFFF)
        }

        pub fn fail() -> Self {
            // This is encoded as the empty CONSUME_RANGE of characters 0xFFFF <= c <=
            // 0x0000.
            RegExpInstruction::consume_range(0xFFFF, 0x0000)
        }

        pub fn range_count(num_ranges: i32) -> Self {
            RegExpInstruction {
                opcode: Opcode::RANGE_COUNT,
                payload: Payload { num_ranges },
            }
        }

        pub fn fork(alt_index: i32) -> Self {
            RegExpInstruction {
                opcode: Opcode::FORK,
                payload: Payload { pc: alt_index },
            }
        }

        pub fn jmp(alt_index: i32) -> Self {
            RegExpInstruction {
                opcode: Opcode::JMP,
                payload: Payload { pc: alt_index },
            }
        }

        pub fn accept() -> Self {
            RegExpInstruction {
                opcode: Opcode::ACCEPT,
                payload: Payload { num_ranges: 0 }, // Dummy value, not used
            }
        }

        pub fn set_register_to_cp(register_index: i32) -> Self {
            RegExpInstruction {
                opcode: Opcode::SET_REGISTER_TO_CP,
                payload: Payload { register_index },
            }
        }

        pub fn assertion(t: RegExpAssertion::Type) -> Self {
            RegExpInstruction {
                opcode: Opcode::ASSERTION,
                payload: Payload {
                    assertion_type: t,
                },
            }
        }

        pub fn clear_register(register_index: i32) -> Self {
            RegExpInstruction {
                opcode: Opcode::CLEAR_REGISTER,
                payload: Payload { register_index },
            }
        }

        pub fn set_quantifier_to_clock(quantifier_id: i32) -> Self {
            RegExpInstruction {
                opcode: Opcode::SET_QUANTIFIER_TO_CLOCK,
                payload: Payload { quantifier_id },
            }
        }

        pub fn filter_quantifier(quantifier_id: i32) -> Self {
            RegExpInstruction {
                opcode: Opcode::FILTER_QUANTIFIER,
                payload: Payload { quantifier_id },
            }
        }

        pub fn filter_group(group_id: i32) -> Self {
            RegExpInstruction {
                opcode: Opcode::FILTER_GROUP,
                payload: Payload { group_id },
            }
        }

        pub fn filter_lookaround(lookaround_id: i32) -> Self {
            RegExpInstruction {
                opcode: Opcode::FILTER_LOOKAROUND,
                payload: Payload { lookaround_id },
            }
        }

        pub fn filter_child(pc: i32) -> Self {
            RegExpInstruction {
                opcode: Opcode::FILTER_CHILD,
                payload: Payload { pc },
            }
        }

        pub fn begin_loop() -> Self {
            RegExpInstruction {
                opcode: Opcode::BEGIN_LOOP,
                payload: Payload { num_ranges: 0 }, // Dummy value, not used
            }
        }

        pub fn end_loop() -> Self {
            RegExpInstruction {
                opcode: Opcode::END_LOOP,
                payload: Payload { num_ranges: 0 }, // Dummy value, not used
            }
        }

        pub fn start_lookaround(
            lookaround_index: i32,
            is_positive: bool,
            type_: RegExpLookaround::Type,
        ) -> Self {
            RegExpInstruction {
                opcode: Opcode::START_LOOKAROUND,
                payload: Payload {
                    lookaround: LookaroundPayload::new_with_params(
                        lookaround_index as u32,
                        is_positive,
                        type_,
                    ),
                },
            }
        }

        pub fn end_lookaround() -> Self {
            RegExpInstruction {
                opcode: Opcode::END_LOOKAROUND,
                payload: Payload { num_ranges: 0 }, // Dummy value, not used
            }
        }

        pub fn write_look_table(index: i32) -> Self {
            RegExpInstruction {
                opcode: Opcode::WRITE_LOOKAROUND_TABLE,
                payload: Payload { lookaround_id: index },
            }
        }

        pub fn read_look_table(
            index: i32,
            is_positive: bool,
            type_: RegExpLookaround::Type,
        ) -> Self {
            RegExpInstruction {
                opcode: Opcode::READ_LOOKAROUND_TABLE,
                payload: Payload {
                    lookaround: LookaroundPayload::new_with_params(
                        index as u32,
                        is_positive,
                        type_,
                    ),
                },
            }
        }

        // Returns whether an instruction is `FILTER_GROUP`, `FILTER_QUANTIFIER` or
        // `FILTER_CHILD`.
        pub fn is_filter(&self) -> bool {
            self.opcode == Opcode::FILTER_GROUP
                || self.opcode == Opcode::FILTER_QUANTIFIER
                || self.opcode == Opcode::FILTER_CHILD
        }
    }

    #[derive(Debug)]
    struct BitField<T, const OFFSET: usize, const SIZE: usize> {
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T, const OFFSET: usize, const SIZE: usize> BitField<T, OFFSET, SIZE> {
        const MASK: u32 = (1 << SIZE) - 1;

        fn encode(value: u32) -> u32 {
            (value & Self::MASK) << OFFSET
        }

        fn decode(payload: u32) -> T
        where
            T: From<u32>,
        {
            ((payload >> OFFSET) & Self::MASK).into()
        }

        type Next<U, const NEXT_SIZE: usize> = BitField<U, { OFFSET + SIZE }, NEXT_SIZE>;

        fn update(payload: u32, value: bool) -> u32 {
            let encoded_value = if value { 1 } else { 0 };
            (payload & !(Self::MASK << OFFSET)) | (encoded_value << OFFSET)
        }

        fn update_enum<E: Into<u32>>(payload: u32, value: E) -> u32 {
            let encoded_value: u32 = value.into();
            (payload & !(Self::MASK << OFFSET)) | ((encoded_value & Self::MASK) << OFFSET)
        }
    }

    impl From<u32> for RegExpLookaround::Type {
        fn from(value: u32) -> Self {
            match value {
                0 => RegExpLookaround::Type::LOOKAHEAD,
                1 => RegExpLookaround::Type::LOOKBEHIND,
                _ => panic!("Invalid RegExpLookaround::Type value"),
            }
        }
    }

    impl From<RegExpLookaround::Type> for u32 {
        fn from(value: RegExpLookaround::Type) -> Self {
            match value {
                RegExpLookaround::Type::LOOKAHEAD => 0,
                RegExpLookaround::Type::LOOKBEHIND => 1,
            }
        }
    }

    impl fmt::Display for RegExpInstruction {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self.opcode {
                Opcode::CONSUME_RANGE => {
                    write!(
                        f,
                        "CONSUME_RANGE [{:?}, {:?}]",
                        self.payload.consume_range.min, self.payload.consume_range.max
                    )
                }
                Opcode::RANGE_COUNT => {
                    write!(f, "RANGE_COUNT {}", unsafe { self.payload.num_ranges })
                }
                Opcode::ASSERTION => {
                    write!(f, "ASSERTION ")?;
                    match unsafe { self.payload.assertion_type } {
                        RegExpAssertion::Type::START_OF_INPUT => write!(f, "START_OF_INPUT"),
                        RegExpAssertion::Type::END_OF_INPUT => write!(f, "END_OF_INPUT"),
                        RegExpAssertion::Type::START_OF_LINE => write!(f, "START_OF_LINE"),
                        RegExpAssertion::Type::END_OF_LINE => write!(f, "END_OF_LINE"),
                        RegExpAssertion::Type::BOUNDARY => write!(f, "BOUNDARY"),
                        RegExpAssertion::Type::NON_BOUNDARY => write!(f, "NON_BOUNDARY"),
                    }
                }
                Opcode::FORK => write!(f, "FORK {}", unsafe { self.payload.pc }),
                Opcode::JMP => write!(f, "JMP {}", unsafe { self.payload.pc }),
                Opcode::ACCEPT => write!(f, "ACCEPT"),
                Opcode::SET_REGISTER_TO_CP => write!(
                    f,
                    "SET_REGISTER_TO_CP {}",
                    unsafe { self.payload.register_index }
                ),
                Opcode::CLEAR_REGISTER => write!(
                    f,
                    "CLEAR_REGISTER {}",
                    unsafe { self.payload.register_index }
                ),
                Opcode::SET_QUANTIFIER_TO_CLOCK => write!(
                    f,
                    "SET_QUANTIFIER_TO_CLOCK {}",
                    unsafe { self.payload.quantifier_id }
                ),
                Opcode::FILTER_QUANTIFIER => write!(
                    f,
                    "FILTER_QUANTIFIER {}",
                    unsafe { self.payload.quantifier_id }
                ),
                Opcode::FILTER_GROUP => {
                    write!(f, "FILTER_GROUP {}", unsafe { self.payload.group_id })
                }
                Opcode::FILTER_LOOKAROUND => write!(
                    f,
                    "FILTER_LOOKAROUND {}",
                    unsafe { self.payload.lookaround_id }
                ),
                Opcode::FILTER_CHILD => write!(f, "FILTER_CHILD {}", unsafe { self.payload.pc }),
                Opcode::BEGIN_LOOP => write!(f, "BEGIN_LOOP"),
                Opcode::END_LOOP => write!(f, "END_LOOP"),
                Opcode::START_LOOKAROUND => {
                    write!(f, "START_LOOKAROUND {}", unsafe { self.payload.lookaround })
                }
                Opcode::END_LOOKAROUND => write!(f, "END_LOOKAROUND"),
                Opcode::WRITE_LOOKAROUND_TABLE => write!(
                    f,
                    "WRITE_LOOKAROUND_TABLE {}",
                    unsafe { self.payload.lookaround_id }
                ),
                Opcode::READ_LOOKAROUND_TABLE => write!(
                    f,
                    "READ_LOOKAROUND_TABLE {}",
                    unsafe { self.payload.lookaround }
                ),
            }
        }
    }

    impl fmt::Display for LookaroundPayload {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{} ({:?}, {})",
                self.index(),
                self.type_(),
                if self.is_positive() { "positive" } else { "negative" }
            )
        }
    }

    pub mod printing {
        use super::*;
        use std::fmt;

        fn print_ascii_or_hex(os: &mut fmt::Formatter<'_>, c: u16) -> fmt::Result {
            if c < 128 && c.is_ascii_graphic() {
                write!(os, "{}", c as char)
            } else {
                write!(os, "0x{:x}", c)
            }
        }

        // The maximum number of digits required to display a non-negative number < n
        // in base 10.
        fn digits_required_below(n: usize) -> usize {
            assert!(n >= 0);

            let mut result = 1;
            let mut i = 10;
            while i < n {
                result += 1;
                i *= 10;
            }
            result
        }

        pub fn print_instructions(
            os: &mut fmt::Formatter<'_>,
            insts: &[RegExpInstruction],
        ) -> fmt::Result {
            let inst_num = insts.len();
            let line_digit_num = digits_required_below(inst_num);

            for (i, inst) in insts.iter().enumerate() {
                write!(os, "{:0>width$}: {}\n", i, inst, width = line_digit_num)?;
            }
            Ok(())
        }
    }
}
