// Copyright 2008 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/regexp/regexp-macro-assembler-tracer.h

pub mod regexp_macro_assembler_tracer {
    use std::rc::Rc;
    use crate::regexp::regexp_macro_assembler::RegExpMacroAssembler;
    use crate::regexp::regexp_macro_assembler::IrregexpImplementation;
    use crate::regexp::regexp_macro_assembler::StackCheckFlag;
    use crate::base::strings::uc16;
    use crate::base::strings::ByteArray;
    use crate::base::strings::ZoneList;
    use crate::base::strings::CharacterRange;
    use crate::base::strings::StandardCharacterSet;
    use crate::regexp::regexp_macro_assembler::RegExpFlags;
    use crate::regexp::regexp_macro_assembler::Label;
    use crate::isolate::isolate::Isolate;
    use crate::isolate::isolate::HeapObject;
    use crate::isolate::isolate::String;
    use crate::isolate::isolate::DirectHandle;

    /// Decorator on a RegExpMacroAssembler that writes all calls.
    pub struct RegExpMacroAssemblerTracer {
        assembler: Rc<dyn RegExpMacroAssembler>,
        isolate: *mut Isolate, // Needs a proper replacement to handle the isolate.
    }

    impl RegExpMacroAssemblerTracer {
        pub fn new(isolate: *mut Isolate, assembler: Rc<dyn RegExpMacroAssembler>) -> Self {
            RegExpMacroAssemblerTracer { assembler, isolate }
        }

        pub fn aborted_code_generation(&self) {
            self.assembler.aborted_code_generation();
        }

        pub fn stack_limit_slack_slot_count(&self) -> i32 {
            self.assembler.stack_limit_slack_slot_count()
        }

        pub fn can_read_unaligned(&self) -> bool {
            self.assembler.can_read_unaligned()
        }

        pub fn advance_current_position(&self, by: i32) {
            self.assembler.advance_current_position(by);
        }

        pub fn advance_register(&self, reg: i32, by: i32) {
            self.assembler.advance_register(reg, by);
        }

        pub fn backtrack(&self) {
            self.assembler.backtrack();
        }

        pub fn bind(&self, label: &mut Label) {
            self.assembler.bind(label);
        }

        pub fn check_character(&self, c: u32, on_equal: &mut Label) {
            self.assembler.check_character(c, on_equal);
        }

        pub fn check_character_after_and(&self, c: u32, and_with: u32, on_equal: &mut Label) {
            self.assembler.check_character_after_and(c, and_with, on_equal);
        }

        pub fn check_character_gt(&self, limit: uc16, on_greater: &mut Label) {
            self.assembler.check_character_gt(limit, on_greater);
        }

        pub fn check_character_lt(&self, limit: uc16, on_less: &mut Label) {
            self.assembler.check_character_lt(limit, on_less);
        }

        pub fn check_greedy_loop(&self, on_tos_equals_current_position: &mut Label) {
            self.assembler.check_greedy_loop(on_tos_equals_current_position);
        }

        pub fn check_at_start(&self, cp_offset: i32, on_at_start: &mut Label) {
            self.assembler.check_at_start(cp_offset, on_at_start);
        }

        pub fn check_not_at_start(&self, cp_offset: i32, on_not_at_start: &mut Label) {
            self.assembler.check_not_at_start(cp_offset, on_not_at_start);
        }

        pub fn check_not_back_reference(&self, start_reg: i32, read_backward: bool, on_no_match: &mut Label) {
            self.assembler.check_not_back_reference(start_reg, read_backward, on_no_match);
        }

        pub fn check_not_back_reference_ignore_case(&self, start_reg: i32, read_backward: bool, unicode: bool, on_no_match: &mut Label) {
            self.assembler.check_not_back_reference_ignore_case(start_reg, read_backward, unicode, on_no_match);
        }

        pub fn check_not_character(&self, c: u32, on_not_equal: &mut Label) {
            self.assembler.check_not_character(c, on_not_equal);
        }

        pub fn check_not_character_after_and(&self, c: u32, and_with: u32, on_not_equal: &mut Label) {
            self.assembler.check_not_character_after_and(c, and_with, on_not_equal);
        }

        pub fn check_not_character_after_minus_and(&self, c: uc16, minus: uc16, and_with: uc16, on_not_equal: &mut Label) {
            self.assembler.check_not_character_after_minus_and(c, minus, and_with, on_not_equal);
        }

        pub fn check_character_in_range(&self, from: uc16, to: uc16, on_in_range: &mut Label) {
            self.assembler.check_character_in_range(from, to, on_in_range);
        }

        pub fn check_character_not_in_range(&self, from: uc16, to: uc16, on_not_in_range: &mut Label) {
            self.assembler.check_character_not_in_range(from, to, on_not_in_range);
        }

        pub fn check_character_in_range_array(&self, ranges: &ZoneList<CharacterRange>, on_in_range: &mut Label) -> bool {
            self.assembler.check_character_in_range_array(ranges, on_in_range)
        }

        pub fn check_character_not_in_range_array(&self, ranges: &ZoneList<CharacterRange>, on_not_in_range: &mut Label) -> bool {
            self.assembler.check_character_not_in_range_array(ranges, on_not_in_range)
        }

        pub fn check_bit_in_table(&self, table: &Handle<ByteArray>, on_bit_set: &mut Label) {
            self.assembler.check_bit_in_table(table, on_bit_set);
        }

        pub fn skip_until_bit_in_table_use_simd(&self, advance_by: i32) -> bool {
            self.assembler.skip_until_bit_in_table_use_simd(advance_by)
        }

        pub fn skip_until_bit_in_table(&self, cp_offset: i32, table: &Handle<ByteArray>, nibble_table: &Handle<ByteArray>, advance_by: i32) {
            self.assembler.skip_until_bit_in_table(cp_offset, table, nibble_table, advance_by);
        }

        pub fn check_position(&self, cp_offset: i32, on_outside_input: &mut Label) {
            self.assembler.check_position(cp_offset, on_outside_input);
        }

        pub fn check_special_class_ranges(&self, type_: StandardCharacterSet, on_no_match: &mut Label) -> bool {
            self.assembler.check_special_class_ranges(type_, on_no_match)
        }

        pub fn fail(&self) {
            self.assembler.fail();
        }

        pub fn get_code(&self, source: &DirectHandle<String>, flags: RegExpFlags) -> DirectHandle<HeapObject> {
           self.assembler.get_code(source, flags)
        }

        pub fn go_to(&self, label: &mut Label) {
            self.assembler.go_to(label);
        }

        pub fn if_register_ge(&self, reg: i32, comparand: i32, if_ge: &mut Label) {
            self.assembler.if_register_ge(reg, comparand, if_ge);
        }

        pub fn if_register_lt(&self, reg: i32, comparand: i32, if_lt: &mut Label) {
            self.assembler.if_register_lt(reg, comparand, if_lt);
        }

        pub fn if_register_eq_pos(&self, reg: i32, if_eq: &mut Label) {
            self.assembler.if_register_eq_pos(reg, if_eq);
        }

        pub fn implementation(&self) -> IrregexpImplementation {
            self.assembler.implementation()
        }

        pub fn load_current_character_impl(&self, cp_offset: i32, on_end_of_input: &mut Label, check_bounds: bool, characters: i32, eats_at_least: i32) {
            self.assembler.load_current_character_impl(cp_offset, on_end_of_input, check_bounds, characters, eats_at_least);
        }

        pub fn pop_current_position(&self) {
            self.assembler.pop_current_position();
        }

        pub fn pop_register(&self, register_index: i32) {
            self.assembler.pop_register(register_index);
        }

        pub fn push_backtrack(&self, label: &mut Label) {
            self.assembler.push_backtrack(label);
        }

        pub fn push_current_position(&self) {
            self.assembler.push_current_position();
        }

        pub fn push_register(&self, register_index: i32, check_stack_limit: StackCheckFlag) {
            self.assembler.push_register(register_index, check_stack_limit);
        }

        pub fn read_current_position_from_register(&self, reg: i32) {
            self.assembler.read_current_position_from_register(reg);
        }

        pub fn read_stack_pointer_from_register(&self, reg: i32) {
            self.assembler.read_stack_pointer_from_register(reg);
        }

        pub fn set_current_position_from_end(&self, by: i32) {
            self.assembler.set_current_position_from_end(by);
        }

        pub fn set_register(&self, register_index: i32, to: i32) {
            self.assembler.set_register(register_index, to);
        }

        pub fn succeed(&self) -> bool {
            self.assembler.succeed()
        }

        pub fn write_current_position_to_register(&self, reg: i32, cp_offset: i32) {
            self.assembler.write_current_position_to_register(reg, cp_offset);
        }

        pub fn clear_registers(&self, reg_from: i32, reg_to: i32) {
            self.assembler.clear_registers(reg_from, reg_to);
        }

        pub fn write_stack_pointer_to_register(&self, reg: i32) {
            self.assembler.write_stack_pointer_to_register(reg);
        }
    }

    impl Drop for RegExpMacroAssemblerTracer {
        fn drop(&mut self) {
            // No specific action needed as assembler_ is managed externally.
        }
    }
}

pub mod base {
    pub mod strings {
        pub type uc16 = u16;

        pub struct ByteArray {}
        pub struct ZoneList<T> {}
        pub struct CharacterRange {}
        pub enum StandardCharacterSet {}
    }
}

pub mod regexp {
    pub mod regexp_macro_assembler {
        use crate::base::strings::ByteArray;
        use crate::base::strings::ZoneList;
        use crate::base::strings::CharacterRange;
        use crate::base::strings::StandardCharacterSet;
        use crate::base::strings::uc16;
        use crate::isolate::isolate::{HeapObject, String};
        use crate::isolate::isolate::DirectHandle;

        pub enum IrregexpImplementation {}
        pub enum StackCheckFlag {}
        pub struct RegExpFlags {}
        pub struct Label {}

        pub trait RegExpMacroAssembler {
            fn aborted_code_generation(&self);
            fn stack_limit_slack_slot_count(&self) -> i32;
            fn can_read_unaligned(&self) -> bool;
            fn advance_current_position(&self, by: i32);
            fn advance_register(&self, reg: i32, by: i32);
            fn backtrack(&self);
            fn bind(&self, label: &mut Label);
            fn check_character(&self, c: u32, on_equal: &mut Label);
            fn check_character_after_and(&self, c: u32, and_with: u32, on_equal: &mut Label);
            fn check_character_gt(&self, limit: uc16, on_greater: &mut Label);
            fn check_character_lt(&self, limit: uc16, on_less: &mut Label);
            fn check_greedy_loop(&self, on_tos_equals_current_position: &mut Label);
            fn check_at_start(&self, cp_offset: i32, on_at_start: &mut Label);
            fn check_not_at_start(&self, cp_offset: i32, on_not_at_start: &mut Label);
            fn check_not_back_reference(&self, start_reg: i32, read_backward: bool, on_no_match: &mut Label);
            fn check_not_back_reference_ignore_case(&self, start_reg: i32, read_backward: bool, unicode: bool, on_no_match: &mut Label);
            fn check_not_character(&self, c: u32, on_not_equal: &mut Label);
            fn check_not_character_after_and(&self, c: u32, and_with: u32, on_not_equal: &mut Label);
            fn check_not_character_after_minus_and(&self, c: uc16, minus: uc16, and_with: uc16, on_not_equal: &mut Label);
            fn check_character_in_range(&self, from: uc16, to: uc16, on_in_range: &mut Label);
            fn check_character_not_in_range(&self, from: uc16, to: uc16, on_not_in_range: &mut Label);
            fn check_character_in_range_array(&self, ranges: &ZoneList<CharacterRange>, on_in_range: &mut Label) -> bool;
            fn check_character_not_in_range_array(&self, ranges: &ZoneList<CharacterRange>, on_not_in_range: &mut Label) -> bool;
            fn check_bit_in_table(&self, table: &Handle<ByteArray>, on_bit_set: &mut Label);
            fn skip_until_bit_in_table_use_simd(&self, advance_by: i32) -> bool;
            fn skip_until_bit_in_table(&self, cp_offset: i32, table: &Handle<ByteArray>, nibble_table: &Handle<ByteArray>, advance_by: i32);
            fn check_position(&self, cp_offset: i32, on_outside_input: &mut Label);
            fn check_special_class_ranges(&self, type_: StandardCharacterSet, on_no_match: &mut Label) -> bool;
            fn fail(&self);
            fn get_code(&self, source: &DirectHandle<String>, flags: RegExpFlags) -> DirectHandle<HeapObject>;
            fn go_to(&self, label: &mut Label);
            fn if_register_ge(&self, reg: i32, comparand: i32, if_ge: &mut Label);
            fn if_register_lt(&self, reg: i32, comparand: i32, if_lt: &mut Label);
            fn if_register_eq_pos(&self, reg: i32, if_eq: &mut Label);
            fn implementation(&self) -> IrregexpImplementation;
            fn load_current_character_impl(&self, cp_offset: i32, on_end_of_input: &mut Label, check_bounds: bool, characters: i32, eats_at_least: i32);
            fn pop_current_position(&self);
            fn pop_register(&self, register_index: i32);
            fn push_backtrack(&self, label: &mut Label);
            fn push_current_position(&self);
            fn push_register(&self, register_index: i32, check_stack_limit: StackCheckFlag);
            fn read_current_position_from_register(&self, reg: i32);
            fn read_stack_pointer_from_register(&self, reg: i32);
            fn set_current_position_from_end(&self, by: i32);
            fn set_register(&self, register_index: i32, to: i32);
            fn succeed(&self) -> bool;
            fn write_current_position_to_register(&self, reg: i32, cp_offset: i32);
            fn clear_registers(&self, reg_from: i32, reg_to: i32);
            fn write_stack_pointer_to_register(&self, reg: i32);
        }
    }
}

pub mod isolate {
    pub mod isolate {
        pub struct Isolate {}
        pub struct HeapObject {}
        pub struct String {}
        pub struct DirectHandle<T> {}
    }
}