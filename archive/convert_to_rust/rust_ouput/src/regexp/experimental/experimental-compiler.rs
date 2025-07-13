// Converted from V8 C++ source files:
// Header: experimental-compiler.h
// Implementation: experimental-compiler.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod experimental_compiler {
    use crate::regexp::experimental::experimental_bytecode::RegExpInstruction;
    use crate::regexp::regexp_ast::RegExpTree;
    use crate::regexp::regexp_flags::RegExpFlags;
    use crate::zone::zone_list::ZoneList;
    use crate::regexp::regexp_flags::RegExpFlag;
    use crate::flags::flags::v8_flags;
    use crate::zone::zone::Zone;
    use crate::regexp::regexp_ast::RegExpDisjunction;
    use crate::regexp::regexp_ast::RegExpAlternative;
    use crate::regexp::regexp_ast::RegExpClassRanges;
    use crate::regexp::regexp_ast::RegExpClassSetOperand;
    use crate::regexp::regexp_ast::RegExpClassSetExpression;
    use crate::regexp::regexp_ast::RegExpAssertion;
    use crate::regexp::regexp_ast::RegExpAtom;
    use crate::regexp::regexp_ast::RegExpText;
    use crate::regexp::regexp_ast::RegExpQuantifier;
    use crate::regexp::regexp_ast::RegExpCapture;
    use crate::regexp::regexp_ast::RegExpGroup;
    use crate::regexp::regexp_ast::RegExpLookaround;
    use crate::regexp::regexp_ast::RegExpBackReference;
    use crate::regexp::regexp_ast::RegExpEmpty;
    use crate::regexp::regexp_ast::TextElement;
    use crate::regexp::regexp_ast::CharacterRange;
    use crate::regexp::regexp_ast::RegExpNode;
    use crate::base::uc32;
    use std::collections::LinkedList;
    use std::collections::HashMap;
    use std::ops::Deref;

    pub struct ExperimentalRegExpCompiler {}

    impl ExperimentalRegExpCompiler {
        pub fn can_be_handled(tree: *mut RegExpTree, flags: RegExpFlags, capture_count: i32) -> bool {
            CanBeHandledVisitor::check(unsafe { &mut *tree }, flags, capture_count)
        }

        pub fn compile(tree: *mut RegExpTree, flags: RegExpFlags, zone: *mut Zone) -> ZoneList<RegExpInstruction> {
            CompileVisitor::compile(unsafe { &mut *tree }, flags, unsafe { &mut *zone })
        }
    }

    const K_MAX_SUPPORTED_CODEPOINT: uc32 = 0xFFFFu;

    struct CanBeHandledVisitor {
        flags: RegExpFlags,
        replication_factor: i32,
        result: bool,
    }

    impl CanBeHandledVisitor {
        fn check(tree: &mut RegExpTree, flags: RegExpFlags, capture_count: i32) -> bool {
            if !Self::are_suitable_flags(flags) {
                return false;
            }
            let mut visitor = CanBeHandledVisitor {
                flags,
                replication_factor: 1,
                result: true,
            };
            tree.accept(&mut visitor);
            visitor.result
        }

        fn are_suitable_flags(flags: RegExpFlags) -> bool {
            const K_ALLOWED_FLAGS: RegExpFlags = RegExpFlag::K_GLOBAL as i32
                | RegExpFlag::K_STICKY as i32
                | RegExpFlag::K_MULTILINE as i32
                | RegExpFlag::K_DOT_ALL as i32
                | RegExpFlag::K_LINEAR as i32;

            (flags & !K_ALLOWED_FLAGS) == 0
        }
    }

    trait RegExpVisitor {
        fn visit_disjunction(&mut self, node: &mut RegExpDisjunction) {
            for alt in node.alternatives.iter_mut() {
                alt.accept(self);
                if !self.get_result() {
                    return;
                }
            }
        }

        fn visit_alternative(&mut self, node: &mut RegExpAlternative) {
            for child in node.nodes.iter_mut() {
                child.accept(self);
                if !self.get_result() {
                    return;
                }
            }
        }

        fn visit_class_ranges(&mut self, _node: &mut RegExpClassRanges) {}

        fn visit_class_set_operand(&mut self, node: &mut RegExpClassSetOperand) {
            self.set_result(!node.has_strings);
        }

        fn visit_class_set_expression(&mut self, _node: &mut RegExpClassSetExpression) {
            self.set_result(false);
        }

        fn visit_assertion(&mut self, _node: &mut RegExpAssertion) {}

        fn visit_atom(&mut self, _node: &mut RegExpAtom) {}

        fn visit_text(&mut self, node: &mut RegExpText) {
            for el in node.elements.iter_mut() {
                el.tree.accept(self);
                if !self.get_result() {
                    return;
                }
            }
        }

        fn visit_quantifier(&mut self, node: &mut RegExpQuantifier) {
            const K_MAX_REPLICATION_FACTOR: i32 = 16;

            if node.min > K_MAX_REPLICATION_FACTOR
                || (node.max != RegExpTree::K_INFINITY && node.max > K_MAX_REPLICATION_FACTOR)
            {
                self.set_result(false);
                return;
            }

            let before_replication_factor = self.get_replication_factor();

            let local_replication = if node.max == RegExpTree::K_INFINITY {
                if node.min > 0 && node.min_match > 0 {
                    std::cmp::max(node.min, 1)
                } else {
                    node.min + 1
                }
            } else {
                node.max
            };

            self.set_replication_factor(before_replication_factor * local_replication);
            if self.get_replication_factor() > K_MAX_REPLICATION_FACTOR {
                self.set_result(false);
                return;
            }

            match node.quantifier_type {
                regexp_ast::QuantifierType::GREEDY | regexp_ast::QuantifierType::NON_GREEDY => {}
                regexp_ast::QuantifierType::POSSESSIVE => {
                    self.set_result(false);
                    return;
                }
            }

            node.body.accept(self);
            self.set_replication_factor(before_replication_factor);
        }

        fn visit_capture(&mut self, node: &mut RegExpCapture) {
            node.body.accept(self);
        }

        fn visit_group(&mut self, node: &mut RegExpGroup) {
            if self.get_flags() != node.flags {
                if !CanBeHandledVisitor::are_suitable_flags(node.flags) {
                    self.set_result(false);
                    return;
                }
            }
            node.body.accept(self);
        }

        fn visit_lookaround(&mut self, node: &mut RegExpLookaround) {
            if is_global(self.get_flags()) || is_sticky(self.get_flags()) {
                self.set_result(false);
                return;
            }

            if !v8_flags.experimental_regexp_engine_capture_group_opt
                && (node.lookaround_type == regexp_ast::LookaroundType::LOOKAHEAD
                    || node.capture_count > 0)
            {
                self.set_result(false);
                return;
            }

            node.body.accept(self);
        }

        fn visit_back_reference(&mut self, _node: &mut RegExpBackReference) {
            self.set_result(false);
        }

        fn visit_empty(&mut self, _node: &mut RegExpEmpty) {}

        fn get_flags(&self) -> RegExpFlags;
        fn get_replication_factor(&self) -> i32;
        fn set_replication_factor(&mut self, factor: i32);
        fn get_result(&self) -> bool;
        fn set_result(&mut self, result: bool);
    }

    impl RegExpVisitor for CanBeHandledVisitor {
        fn get_flags(&self) -> RegExpFlags {
            self.flags
        }

        fn get_replication_factor(&self) -> i32 {
            self.replication_factor
        }

        fn set_replication_factor(&mut self, factor: i32) {
            self.replication_factor = factor;
        }

        fn get_result(&self) -> bool {
            self.result
        }

        fn set_result(&mut self, result: bool) {
            self.result = result;
        }

         fn visit_disjunction(&mut self, node: &mut RegExpDisjunction) {
            for alt in node.alternatives.iter_mut() {
                alt.accept(self);
                if !self.result {
                    return;
                }
            }
        }

        fn visit_alternative(&mut self, node: &mut RegExpAlternative) {
            for child in node.nodes.iter_mut() {
                child.accept(self);
                if !self.result {
                    return;
                }
            }
        }

        fn visit_class_set_operand(&mut self, node: &mut RegExpClassSetOperand) {
            self.result = !node.has_strings();
        }

        fn visit_back_reference(&mut self, _node: &mut RegExpBackReference) {
            self.result = false;
        }
    }

    fn is_global(flags: RegExpFlags) -> bool {
        (flags & RegExpFlag::K_GLOBAL as i32) != 0
    }

    fn is_sticky(flags: RegExpFlags) -> bool {
        (flags & RegExpFlag::K_STICKY as i32) != 0
    }

    #[derive(Default)]
    struct Label {
        state: LabelState,
        payload: i32,
    }

    #[derive(PartialEq, Eq)]
    enum LabelState {
        Unbound,
        Bound,
    }

    impl Default for LabelState {
        fn default() -> Self {
            LabelState::Unbound
        }
    }

    struct BytecodeAssembler<'a> {
        zone: &'a mut Zone,
        code: ZoneList<'a, RegExpInstruction>,
    }

    impl<'a> BytecodeAssembler<'a> {
        fn new(zone: &'a mut Zone) -> Self {
            BytecodeAssembler {
                zone,
                code: ZoneList::new(zone),
            }
        }

        fn into_code(self) -> ZoneList<'a, RegExpInstruction> {
            self.code
        }

        fn accept(&mut self) {
            self.code.add(RegExpInstruction::accept(), self.zone);
        }

        fn assertion(&mut self, t: regexp_ast::AssertionType) {
            self.code.add(RegExpInstruction::assertion(t), self.zone);
        }

        fn clear_register(&mut self, register_index: i32) {
            self.code.add(RegExpInstruction::clear_register(register_index), self.zone);
        }

        fn consume_range(&mut self, from: uc16, to: uc16) {
            self.code.add(RegExpInstruction::consume_range(from, to), self.zone);
        }

        fn consume_any_char(&mut self) {
            self.code.add(RegExpInstruction::consume_any_char(), self.zone);
        }

        fn range_count(&mut self, num_ranges: i32) {
            self.code.add(RegExpInstruction::range_count(num_ranges), self.zone);
        }

        fn fork(&mut self, target: &mut Label) {
            self.labelled_instr_impl(RegExpInstruction::Opcode::FORK, target);
        }

        fn jmp(&mut self, target: &mut Label) {
            self.labelled_instr_impl(RegExpInstruction::Opcode::JMP, target);
        }

        fn set_register_to_cp(&mut self, register_index: i32) {
            self.code.add(RegExpInstruction::set_register_to_cp(register_index), self.zone);
        }

        fn begin_loop(&mut self) {
            self.code.add(RegExpInstruction::begin_loop(), self.zone);
        }

        fn end_loop(&mut self) {
            self.code.add(RegExpInstruction::end_loop(), self.zone);
        }

        fn start_lookaround(&mut self, lookaround_index: i32, is_positive: bool, type_: regexp_ast::LookaroundType) {
            self.code.add(
                RegExpInstruction::start_lookaround(lookaround_index, is_positive, type_),
                self.zone,
            );
        }

        fn end_lookaround(&mut self) {
            self.code.add(RegExpInstruction::end_lookaround(), self.zone);
        }

        fn write_lookaround_table(&mut self, index: i32) {
            self.code.add(RegExpInstruction::write_look_table(index), self.zone);
        }

        fn read_lookaround_table(&mut self, index: i32, is_positive: bool, type_: regexp_ast::LookaroundType) {
            self.code.add(
                RegExpInstruction::read_look_table(index, is_positive, type_),
                self.zone,
            );
        }

        fn set_quantifier_to_clock(&mut self, quantifier_id: i32) {
            self.code.add(RegExpInstruction::set_quantifier_to_clock(quantifier_id), self.zone);
        }

        fn filter_quantifier(&mut self, quantifier_id: i32) {
            self.code.add(RegExpInstruction::filter_quantifier(quantifier_id), self.zone);
        }

        fn filter_group(&mut self, group_id: i32) {
            self.code.add(RegExpInstruction::filter_group(group_id), self.zone);
        }

        fn filter_lookaround(&mut self, lookaround_id: i32) {
            self.code.add(RegExpInstruction::filter_lookaround(lookaround_id), self.zone);
        }

        fn filter_child(&mut self, target: &mut Label) {
            self.labelled_instr_impl(RegExpInstruction::Opcode::FILTER_CHILD, target);
        }

        fn bind(&mut self, target: &mut Label) {
            assert_eq!(target.state, LabelState::Unbound);

            let index = self.code.length();

            // Patch all previous jumps to this label.
            // This code is inefficient, as it iterates over the code multiple times.
            // However, the number of jumps to a label is expected to be small.
            let mut i = 0;
            while i < self.code.length() {
                let mut inst = &mut self.code[i];
                match inst.opcode {
                    RegExpInstruction::Opcode::FORK | RegExpInstruction::Opcode::JMP | RegExpInstruction::Opcode::FILTER_CHILD => {
                        if inst.payload.pc == -1 {
                          inst.payload.pc = index;
                        }
                    },
                    _ => {}
                }
                i = i + 1;
            }

            target.state = LabelState::Bound;
            target.payload = index;
        }

        fn fail(&mut self) {
            self.code.add(RegExpInstruction::fail(), self.zone);
        }

        fn labelled_instr_impl(&mut self, op: RegExpInstruction::Opcode, target: &mut Label) {
            let mut result = RegExpInstruction {
                opcode: op,
                payload: RegExpInstruction::Payload { pc: 0 },
            };

            if target.state == LabelState::Bound {
                result.payload.pc = target.payload;
            } else {
                result.payload.pc = -1;
            }

            self.code.add(result, self.zone);
        }
    }

    type uc16 = u16;

    struct FilterGroupsCompileVisitor<'a> {
        zone: &'a mut Zone,
        assembler: &'a mut BytecodeAssembler<'a>,
        nodes: LinkedList<BFEntry<'a>>,
        can_compile_node: bool,
        quantifier_id_remapping: HashMap<i32, i32>,
        lookaround_id_remapping: HashMap<i32, i32>,
    }

    impl<'a> FilterGroupsCompileVisitor<'a> {
        fn compile_filter(
            zone: &'a mut Zone,
            tree: &mut RegExpTree,
            assembler: &'a mut BytecodeAssembler<'a>,
            quantifier_id_remapping: &HashMap<i32, i32>,
            lookaround_id_remapping: &HashMap<i32, i32>,
        ) {
            let mut visitor = FilterGroupsCompileVisitor {
                zone,
                assembler,
                nodes: LinkedList::new(),
                can_compile_node: false,
                quantifier_id_remapping: quantifier_id_remapping.clone(),
                lookaround_id_remapping: lookaround_id_remapping.clone(),
            };

            tree.accept(&mut visitor);

            while !visitor.nodes.is_empty() {
                let mut entry = visitor.nodes.pop_front().unwrap();

                visitor.assembler.bind(&mut entry.label);
                visitor.can_compile_node = true;
                entry.node.accept(&mut visitor);
            }
        }
    }

    impl<'a> RegExpVisitor for FilterGroupsCompileVisitor<'a> {
        fn visit_disjunction(&mut self, node: &mut RegExpDisjunction) {
            for alt in node.alternatives.iter_mut() {
                alt.accept(self);
            }
        }

        fn visit_alternative(&mut self, node: &mut RegExpAlternative) {
            for alt in node.nodes.iter_mut() {
                alt.accept(self);
            }
        }

        fn visit_class_ranges(&mut self, _node: &mut RegExpClassRanges) {}

        fn visit_class_set_operand(&mut self, _node: &mut RegExpClassSetOperand) {}

        fn visit_class_set_expression(&mut self, _node: &mut RegExpClassSetExpression) {}

        fn visit_assertion(&mut self, _node: &mut RegExpAssertion) {}

        fn visit_atom(&mut self, _node: &mut RegExpAtom) {}

        fn visit_text(&mut self, _node: &mut RegExpText) {}

        fn visit_quantifier(&mut self, node: &mut RegExpQuantifier) {
            if self.can_compile_node {
                let id = *self.quantifier_id_remapping.get(&node.index).unwrap();
                self.assembler.filter_quantifier(id);
                self.can_compile_node = false;
                node.body.accept(self);
            } else {
                if node.capture_registers.is_empty() {
                    return;
                }

                let mut bf_entry = BFEntry::new(node);
                self.assembler.filter_child(&mut bf_entry.label);
                self.nodes.push_back(bf_entry);
            }
        }

        fn visit_capture(&mut self, node: &mut RegExpCapture) {
            if self.can_compile_node {
                self.assembler.filter_group(node.index);
                self.can_compile_node = false;
                node.body.accept(self);
            } else {
                let mut bf_entry = BFEntry::new(node);
                self.assembler.filter_child(&mut bf_entry.label);
                self.nodes.push_back(bf_entry);
            }
        }

        fn visit_group(&mut self, node: &mut RegExpGroup) {
            node.body.accept(self);
        }

        fn visit_lookaround(&mut self, node: &mut RegExpLookaround) {
            if self.can_compile_node {
                 let id = *self.lookaround_id_remapping.get(&node.index).unwrap();
                self.assembler.filter_lookaround(id);
                self.can_compile_node = false;
                node.body.accept(self);
            } else {
                if node.capture_registers.is_empty() {
                    return;
                }
               let mut bf_entry = BFEntry::new(node);
                self.assembler.filter_child(&mut bf_entry.label);
                self.nodes.push_back(bf_entry);
            }
        }

        fn visit_back_reference(&mut self, _node: &mut RegExpBackReference) {}

        fn visit_empty(&mut self, _node: &mut RegExpEmpty) {}

        fn get_flags(&self) -> RegExpFlags {
           0
        }

        fn get_replication_factor(&self) -> i32 {
            0
        }

        fn set_replication_factor(&mut self, _factor: i32) {}

        fn get_result(&self) -> bool {
            true
        }

        fn set_result(&mut self, _result: bool) {}
    }

    struct BFEntry<'a> {
        label: Label,
        node: &'a mut RegExpTree,
    }

    impl<'a> BFEntry<'a> {
        fn new(node: &'a mut RegExpTree) -> Self {
            BFEntry {
                label: Label::default(),
                node,
            }
        }
    }

    struct CompileVisitor<'a> {
        zone: &'a mut Zone,
        lookarounds: LinkedList<&'a mut RegExpLookaround>,
        quantifier_id_remapping: HashMap<i32, i32>,
        lookaround_id_remapping: HashMap<i32, i32>,
        assembler: BytecodeAssembler<'a>,
        reverse: bool,
        ignore_captures: bool,
        ignore_lookarounds: bool,
    }

    impl<'a> CompileVisitor<'a> {
        fn compile(tree: &mut RegExpTree, flags: RegExpFlags, zone: &mut Zone) -> ZoneList<RegExpInstruction> {
            let mut compiler = CompileVisitor {
                zone,
                lookarounds: LinkedList::new(),
                quantifier_id_remapping: HashMap::new(),
                lookaround_id_remapping: HashMap::new(),
                assembler: BytecodeAssembler::new(zone),
                reverse: false,
                ignore_captures: false,
                ignore_lookarounds: false,
            };

            if !is_sticky(flags) && !tree.is_anchored_at_start() {
                compiler.compile_non_greedy_star(|| compiler.assembler.consume_any_char());
            }

            compiler.assembler.set_register_to_cp(0);
            tree.accept(&mut compiler);
            compiler.assembler.set_register_to_cp(1);
            compiler.assembler.accept();

            while !compiler.lookarounds.is_empty() {
                let node = compiler.lookarounds.pop_front().unwrap();
                compiler.compile_lookaround(node);
            }

            if v8_flags.experimental_regexp_engine_capture_group_opt {
                 FilterGroupsCompileVisitor::compile_filter(
                        compiler.zone,
                        tree,
                        &mut compiler.assembler,
                        &compiler.quantifier_id_remapping,
                        &compiler.lookaround_id_remapping,
                    );
            }
             compiler.assembler.into_code()
        }

        fn compile_lookaround(&mut self, lookaround: &mut RegExpLookaround) {
            self.assembler.start_lookaround(
                self.remap_lookaround(lookaround.index),
                lookaround.is_positive,
                lookaround.lookaround_type,
            );

            if (lookaround.lookaround_type == regexp_ast::LookaroundType::LOOKAHEAD
                && !lookaround.body.is_anchored_at_end())
                || (lookaround.lookaround_type == regexp_ast::LookaroundType::LOOKBEHIND
                    && !lookaround.body.is_anchored_at_start())
            {
                self.compile_non_greedy_star(|| self.assembler.consume_any_char());
            }

            self.reverse = lookaround.lookaround_type == regexp_ast::LookaroundType::LOOKAHEAD;

            self.ignore_captures = true;
            lookaround.body.accept(self);
            self.ignore_captures = false;

            self.assembler.write_lookaround_table(self.remap_lookaround(lookaround.index));

            if lookaround.capture_count > 0 && lookaround.is_positive {
                self.reverse = lookaround.lookaround_type == regexp_ast::LookaroundType::LOOKBEHIND;

                self.ignore_lookarounds = true;
                lookaround.body.accept(self);
                self.ignore_lookarounds = false;
            }

            self.assembler.end_lookaround();
        }

        fn compile_disjunction<F>(&mut self, alt_num: usize, mut gen_alt: F)
        where
            F: FnMut(usize),
        {
            if alt_num == 0 {
                self.assembler.fail();
                return;
            }

            let mut end = Label::default();

            for i in 0..alt_num - 1 {
                let mut tail = Label::default();
                self.assembler.fork(&mut tail);
                gen_alt(i);
                self.assembler.jmp(&mut end);
                self.assembler.bind(&mut tail);
            }

            gen_alt(alt_num - 1);

            self.assembler.bind(&mut end);
        }

        fn compile_character_ranges(&mut self, ranges: &mut Vec<CharacterRange>, negated: bool) {
           
            if negated {
                let mut negated_ranges = Vec::with_capacity(ranges.len() + 1);
                CharacterRange::negate(ranges, &mut negated_ranges, self.zone);
                CharacterRange::canonicalize(&mut negated_ranges);
                self.compile_character_ranges_inner(&mut negated_ranges);
            } else {
                CharacterRange::canonicalize(ranges);
                self.compile_character_ranges_inner(ranges);
            }
        }

        fn compile_character_ranges_inner(&mut self, ranges: &mut Vec<CharacterRange>) {
            if ranges.is_empty() {
                self.assembler.fail();
                return;
            }

            if ranges.len() > 1 {
                self.assembler.range_count(ranges.len() as i32);
            }

            for range in ranges.iter() {
                let from = range.from;
                assert!(from <= K_MAX_SUPPORTED_CODEPOINT);
                let from_uc16 = from as uc16;

                let to = range.to;
                let to_uc16 = std::cmp::min(to, K_MAX_SUPPORTED_CODEPOINT) as uc16;

                self.assembler.consume_range(from_uc16, to_uc16);
            }
        }

        fn clear_registers(&mut self, indices: Interval) {
            if indices.is_empty() {
                return;
            }
            assert_eq!(indices.from % 2, 0);
            assert_eq!(indices.to % 2, 1);
            for i in (indices.from..=indices.to).step_by(2) {
                self.assembler.clear_register(i);
            }
        }

        fn compile_greedy_star<F>(&mut self, mut emit_body: F)
        where
            F: FnMut(),
        {
            let mut begin = Label::default();
            let mut end = Label::default();

            self.assembler.bind(&mut begin);
            self.assembler.fork(&mut end);
            self.assembler.begin_loop();
            emit_body();
            self.assembler.end_loop();
            self.assembler.jmp(&mut begin);

            self.assembler.bind(&mut end);
        }

        fn compile_non_greedy_star<F>(&mut self, mut emit_body: F)
        where
            F: FnMut(),
        {
            let mut body = Label::default();
            let mut end = Label::default();

            self.assembler.fork(&mut body);
            self.assembler.jmp(&mut end);

            self.assembler.bind(&mut body);
            self.assembler.begin_loop();
            emit_body();
            self.assembler.end_loop();
            self.assembler.fork(&mut body);

            self.assembler.bind(&mut end);
        }

        fn compile_greedy_repetition<F>(&mut self, mut emit_body: F, max_repetition_num: i32)
        where
            F: FnMut(),
        {
            let mut end = Label::default();
            for _ in 0..max_repetition_num {
                self.assembler.fork(&mut end);
                self.assembler.begin_loop();
                emit_body();
                self.assembler.end_loop();
            }
            self.assembler.bind(&mut end);
        }

        fn compile_non_greedy_repetition<F>(&mut self, mut emit_body: F, max_repetition_num: i32)
        where
            F: FnMut(),
        {
            let mut end = Label::default();
            for _ in 0..max_repetition_num {
                let mut body = Label::default();
                self.assembler.fork(&mut body);
                self.assembler.jmp(&mut end);

                self.assembler.bind(&mut body);
                self.assembler.begin_loop();
                emit_body();
                self.assembler.end_loop();
            }
            self.assembler.bind(&mut end);
        }

        fn compile_non_nullable_greedy_plus<F>(&mut self, mut emit_body: F)
        where
            F: FnMut(),
        {
            let mut begin = Label::default();
            let mut end = Label::default();

            self.assembler.bind(&mut begin);
            emit_body();

            self.assembler.fork(&mut end);
            self.assembler.jmp(&mut begin);
            self.assembler.bind(&mut end);
        }

        fn compile_non_nullable_non_greedy_plus<F>(&mut self, mut emit_body: F)
        where
            F: FnMut(),
        {
            let mut begin = Label::default();

            self.assembler.bind(&mut begin);
            emit_body();

            self.assembler.fork(&mut begin);
        }

        fn remap_quantifier(&mut self, id: i32) -> i32 {
           
            if !self.quantifier_id_remapping.contains_key(&id) {
                let len = self.quantifier_id_remapping.len() as i32;
                self.quantifier_id_remapping.insert(id, len);
            }

            *self.quantifier_id_remapping.get(&id).unwrap()
        }

        fn remap_lookaround(&mut self, id: i32) -> i32 {
           
            if !self.lookaround_id_remapping.contains_key(&id) {
                 let len = self.lookaround_id_remapping.len() as i32;
                self.lookaround_id_remapping.insert(id, len);
            }
            *self.lookaround_id_remapping.get(&id).unwrap()
        }
    }

    impl<'a> RegExpVisitor for CompileVisitor<'a> {
        fn visit_disjunction(&mut self, node: &mut RegExpDisjunction) {
           
            let alts = &mut node.alternatives;
            let len = alts.len();

            self.compile_disjunction(len, |i| {
                let alt = &mut alts[i];
                alt.accept(self);
            });
        }

        fn visit_alternative(&mut self, node: &mut RegExpAlternative) {
             if self.reverse {
                let children = &mut node.nodes;
                for i in (0..children.len()).rev() {
                    let child = &mut children[i];
                    child.accept(self);
                }
            } else {
                for child in node.nodes.iter_mut() {
                    child.accept(self);
                }
            }
        }

        fn visit_assertion(&mut self, node: &mut RegExpAssertion) {
            self.assembler.assertion(node.assertion_type);
        }

        fn visit_class_ranges(&mut self, node: &mut RegExpClassRanges) {
            let mut ranges = node.ranges.clone();
             self.compile_character_ranges(&mut ranges, node.is_negated);
        }

        fn visit_class_set_operand(&mut self, node: &mut RegExpClassSetOperand) {
            assert!(!node.has_strings);
            let mut ranges = node.ranges.clone();
           self.compile_character_ranges(&mut ranges, false);
        }

        fn visit_class_set_expression(&mut self, _node: &mut RegExpClassSetExpression) {
            unreachable!();
        }

        fn visit_atom(&mut self, node: &mut RegExpAtom) {
             if self.reverse {
                let data = &node.data;
                for i in (0..data.len()).rev() {
                    self.assembler.consume_range(data[i], data[i]);
                }
            } else {
                for c in &node.data {
                    self.assembler.consume_range(*c, *c);
                }
            
