// Converted from V8 C++ source files:
// Header: regexp-compiler.h
// Implementation: regexp-compiler.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod regexp_compiler {
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::regexp::regexp_flags::{RegExpFlags, IsEitherUnicode, IsIgnoreCase};
    use crate::regexp::regexp_nodes::{RegExpNode, TextElement, EndNode, RegExpCapture, RegExpQuantifier, ChoiceNode, GuardedAlternative, TextNode as RegExpNodeTextNode, StandardCharacterSet, ActionNode};
    use crate::sandbox::code_pointer_table::Isolate;
    use crate::base::{Char16, Vector, Uc32};
    use crate::zone::zone::Zone;
    use crate::objects::{String, FixedArray};
    use crate::logging::code_events::RegExpFlags as LoggingRegExpFlags;
    use crate::ast::ast::String as AstString;
    use crate::base;
    use std::io;
    use crate::handles::handles::DirectHandle;
    use crate::execution::local_isolate::StackLimitCheck;
    use crate::compiler::turboshaft::operations::Number;
    use crate::compiler::backend::instruction_selector_adapter::ArchOpcode;
    use crate::compiler::common_operator::OpIndex;
    use crate::compiler::turboshaft::graph::Graph;
    use std::sync::Arc;
    use crate::tasks::cancelable_task::Cancelable;
    use crate::torque::declarable::VariableMode;
    use crate::compiler::backend::frame::Frame;
    use crate::compiler::loop_unrolling::Operator;
    use crate::ast::ast_value_factory::AstRawString;
    use crate::compiler::js_type_hint_lowering::Node;
    use crate::handles::handles::Handle;
    use crate::compiler::turboshaft::typer::WordType;
    use crate::compiler::turboshaft::typer::Bits;
    use crate::objects::heap_object::HeapObject;
    use crate::compiler::common_operator::AtomicMemoryOrder;
    use crate::compiler::backend::instruction::InstructionOperand;
    use crate::init::bootstrapper::Root;
    use crate::compiler::common_operator::CallFrequency;
    use crate::compiler::backend::machine_operator_builder::MachineOperatorBuilder;
    use crate::compiler::common_operator::Common;
    use crate::torque::cfg::Instruction;
    use crate::bigint::bigint_internal::Status;
    use crate::torque::types::Simple;
    use crate::torque::types::Type;
    use crate::torque::torque_code_generator::CfgAssembler;
    use crate::handles::handles::IndirectHandle;
    use crate::deoptimizer::deoptimizer::AbortReason;
    use crate::compiler::scheduler::BranchHint;
    use crate::base::small_vector::SmallVector;
    use crate::regexp::regexp_compiler_tonode::CharacterClassStrings;
    use crate::regexp::regexp_utils::base;
    use crate::regexp::regexp_compiler_tonode::RegExpCapture;
    use crate::objects::jsregexp::JSRegExp;
    use crate::compiler::backend::code_generator_impl::MachineType;
    use crate::torque::earley_parser::Symbol;
    use crate::torque::earley_parser;
    use crate::codegen::register::{Register, VfpRegister, DwVfpRegister, CPURegister, RegisterConfiguration};
    use std::fmt;

    pub struct Interval {
        from_: i32,
        to_: i32,
    }

    impl Interval {
        pub fn new(from_: i32, to_: i32) -> Self {
            Interval { from_: from_, to_: to_ }
        }

        pub fn from(&self) -> i32 {
            self.from_
        }

        pub fn to(&self) -> i32 {
            self.to_
        }

        pub fn size(&self) -> i32 {
            self.to_ - self.from_ + 1
        }

        pub fn contains(&self, value: i32) -> bool {
            self.from_ <= value && value <= self.to_
        }
    }

    pub mod regexp_compiler_constants {
        pub const kRangeEndMarker: i32 = 0x110000;
    }

    pub fn needs_unicode_case_equivalents(flags: RegExpFlags) -> bool {
        IsEitherUnicode(flags) && IsIgnoreCase(flags)
    }

    // Details of a quick mask-compare check that can look ahead in the
    // input stream.
    #[derive(Clone, Copy, Debug)]
    pub struct QuickCheckDetails {
        characters_: i32,
        positions_: [Position; 4],
        mask_: u32,
        value_: u32,
        cannot_match_: bool,
    }

    impl QuickCheckDetails {
        pub fn new() -> Self {
            QuickCheckDetails {
                characters_: 0,
                positions_: [Position::new(); 4],
                mask_: 0,
                value_: 0,
                cannot_match_: false,
            }
        }

        pub fn with_characters(characters: i32) -> Self {
            QuickCheckDetails {
                characters_: characters,
                positions_: [Position::new(); 4],
                mask_: 0,
                value_: 0,
                cannot_match_: false,
            }
        }

        pub fn rationalize(&mut self, one_byte: bool) -> bool {
            let mut found_useful_op = false;
            let char_mask = CharMask(one_byte);
            self.mask_ = 0;
            self.value_ = 0;
            let mut char_shift = 0;
            for i in 0..self.characters_ {
                let pos = &mut self.positions_[i as usize];
                if (pos.mask & String::kMaxOneByteCharCode as u32) != 0 {
                    found_useful_op = true;
                }
                self.mask_ |= (pos.mask & char_mask as u32) << char_shift;
                self.value_ |= (pos.value & char_mask as u32) << char_shift;
                char_shift += if one_byte { 8 } else { 16 };
            }
            found_useful_op
        }

        pub fn merge(&mut self, other: &QuickCheckDetails, from_index: i32) {
            if self.characters_ != other.characters_ {
                return;
            }
            if other.cannot_match_ {
                return;
            }
            if self.cannot_match_ {
                *self = *other;
                return;
            }
            for i in from_index..self.characters_ {
                let pos = &mut self.positions_[i as usize];
                let other_pos = &other.positions_[i as usize];
                if pos.mask != other_pos.mask || pos.value != other_pos.value || !other_pos.determines_perfectly {
                    pos.determines_perfectly = false;
                }
                pos.mask &= other_pos.mask;
                pos.value &= pos.mask;
            }
        }

        pub fn advance(&mut self, by: i32, one_byte: bool) {
            if by >= self.characters_ || by < 0 {
                if by < 0 {
                   
                }
                self.clear();
                return;
            }
            for i in 0..(self.characters_ - by) {
                self.positions_[i as usize] = self.positions_[(by + i) as usize];
            }
            for i in (self.characters_ - by)..self.characters_ {
                self.positions_[i as usize].mask = 0;
                self.positions_[i as usize].value = 0;
                self.positions_[i as usize].determines_perfectly = false;
            }
            self.characters_ -= by;
        }

        pub fn clear(&mut self) {
            for i in 0..self.characters_ {
                self.positions_[i as usize].mask = 0;
                self.positions_[i as usize].value = 0;
                self.positions_[i as usize].determines_perfectly = false;
            }
            self.characters_ = 0;
        }

        pub fn cannot_match(&self) -> bool {
            self.cannot_match_
        }

        pub fn set_cannot_match(&mut self) {
            self.cannot_match_ = true;
        }

        pub fn characters(&self) -> i32 {
            self.characters_
        }

        pub fn set_characters(&mut self, characters: i32) {
            self.characters_ = characters;
        }

        pub fn positions(&mut self, index: i32) -> &mut Position {
            &mut self.positions_[index as usize]
        }

        pub fn mask(&self) -> u32 {
            self.mask_
        }

        pub fn value(&self) -> u32 {
            self.value_
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct Position {
        mask: u32,
        value: u32,
        determines_perfectly: bool,
    }

    impl Position {
        pub fn new() -> Self {
            Position {
                mask: 0,
                value: 0,
                determines_perfectly: false,
            }
        }
    }

    fn char_mask(one_byte: bool) -> u32 {
        if one_byte { String::kMaxOneByteCharCodeU as u32 } else { String::kMaxUtf16CodeUnitU as u32 }
    }

    pub enum ContainedInLattice {
        kNotYet = 0,
        kLatticeIn = 1,
        kLatticeOut = 2,
        kLatticeUnknown = 3,
    }

    impl fmt::Debug for ContainedInLattice {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ContainedInLattice::kNotYet => write!(f, "kNotYet"),
                ContainedInLattice::kLatticeIn => write!(f, "kLatticeIn"),
                ContainedInLattice::kLatticeOut => write!(f, "kLatticeOut"),
                ContainedInLattice::kLatticeUnknown => write!(f, "kLatticeUnknown"),
            }
        }
    }

    pub fn combine(a: ContainedInLattice, b: ContainedInLattice) -> ContainedInLattice {
        match (a as u32 | b as u32) {
          0 => ContainedInLattice::kNotYet,
          1 => ContainedInLattice::kLatticeIn,
          2 => ContainedInLattice::kLatticeOut,
          _ => ContainedInLattice::kLatticeUnknown,
        }
    }
}
