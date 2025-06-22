// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod regexp_nodes {
    use std::cell::Cell;
    use std::rc::Rc;

    // Placeholder for Label, RegExpCompiler, Trace, Zone, Isolate, BoyerMooreLookahead,
    // AlternativeGenerationList, GreedyLoopState, QuickCheckDetails, RegExpFlags,
    // Interval, ZoneList, CharacterRange, TextElement, RegExpClassRanges,
    // RegExpMacroAssembler
    struct Label {}
    struct RegExpCompiler {}
    struct Trace {}
    struct Zone {}
    struct Isolate {}
    struct BoyerMooreLookahead {}
    struct AlternativeGenerationList {}
    struct GreedyLoopState {}
    struct QuickCheckDetails {}
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct RegExpFlags {
        flags: i32,
    }
    impl RegExpFlags {
        fn new(flags: i32) -> Self {
            RegExpFlags { flags }
        }
    }
    struct Interval {}
    struct ZoneList<T> {
        items: Vec<T>,
    }
    impl<T> ZoneList<T> {
        fn new(capacity: usize, _zone: &Zone) -> Self {
            ZoneList { items: Vec::with_capacity(capacity) }
        }
        fn add(&mut self, item: T, _zone: &Zone) {
            self.items.push(item);
        }
        fn at(&self, index: usize) -> &T {
            &self.items[index]
        }
        fn items(&self) -> &Vec<T> {
            &self.items
        }
    }
    struct CharacterRange {}
    struct TextElement {}
    impl TextElement {
        fn class_ranges(_that: &RegExpClassRanges) -> Self {
            TextElement {}
        }
    }
    struct RegExpClassRanges {}
    struct RegExpMacroAssembler {}

    const K_MIN_INT: i32 = i32::MIN;

    macro_rules! for_each_node_type {
        ($visit:ident) => {
            $visit!(End);
            $visit!(Action);
            $visit!(Choice);
            $visit!(LoopChoice);
            $visit!(NegativeLookaroundChoice);
            $visit!(BackReference);
            $visit!(Assertion);
            $visit!(Text);
        };
    }

    #[derive(Default)]
    struct NodeInfo {
        being_analyzed: bool,
        been_analyzed: bool,
        follows_word_interest: bool,
        follows_newline_interest: bool,
        follows_start_interest: bool,
        at_end: bool,
        visited: bool,
        replacement_calculated: bool,
    }

    impl NodeInfo {
        // Returns true if the interests and assumptions of this node
        // matches the given one.
        fn matches(&self, that: &NodeInfo) -> bool {
            (self.at_end == that.at_end)
                && (self.follows_word_interest == that.follows_word_interest)
                && (self.follows_newline_interest == that.follows_newline_interest)
                && (self.follows_start_interest == that.follows_start_interest)
        }

        // Updates the interests of this node given the interests of the
        // node preceding it.
        fn add_from_preceding(&mut self, that: &NodeInfo) {
            self.at_end |= that.at_end;
            self.follows_word_interest |= that.follows_word_interest;
            self.follows_newline_interest |= that.follows_newline_interest;
            self.follows_start_interest |= that.follows_start_interest;
        }

        fn has_lookbehind(&self) -> bool {
            self.follows_word_interest
                || self.follows_newline_interest
                || self.follows_start_interest
        }

        // Sets the interests of this node to include the interests of the
        // following node.
        fn add_from_following(&mut self, that: &NodeInfo) {
            self.follows_word_interest |= that.follows_word_interest;
            self.follows_newline_interest |= that.follows_newline_interest;
            self.follows_start_interest |= that.follows_start_interest;
        }

        fn reset_compilation_state(&mut self) {
            self.being_analyzed = false;
            self.been_analyzed = false;
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    struct EatsAtLeastInfo {
        eats_at_least_from_possibly_start: u8,
        eats_at_least_from_not_start: u8,
    }

    impl EatsAtLeastInfo {
        fn new(eats: u8) -> Self {
            EatsAtLeastInfo {
                eats_at_least_from_possibly_start: eats,
                eats_at_least_from_not_start: eats,
            }
        }

        fn set_min(&mut self, other: &EatsAtLeastInfo) {
            self.eats_at_least_from_possibly_start = std::cmp::min(
                self.eats_at_least_from_possibly_start,
                other.eats_at_least_from_possibly_start,
            );
            self.eats_at_least_from_not_start = std::cmp::min(
                self.eats_at_least_from_not_start,
                other.eats_at_least_from_not_start,
            );
        }

        fn is_zero(&self) -> bool {
            self.eats_at_least_from_possibly_start == 0
                && self.eats_at_least_from_not_start == 0
        }
    }

    trait RegExpNodeTrait {
        fn accept(&mut self, visitor: &mut dyn NodeVisitor);
        fn emit(&mut self, compiler: &mut RegExpCompiler, trace: &mut Trace);
        fn eats_at_least(&self, not_at_start: bool) -> u32;
        fn eats_at_least_from_loop_entry(&self) -> EatsAtLeastInfo;
        fn emit_quick_check(
            &mut self,
            compiler: &mut RegExpCompiler,
            bounds_check_trace: &mut Trace,
            trace: &mut Trace,
            preload_has_checked_bounds: bool,
            on_possible_success: &mut Label,
            details_return: &mut QuickCheckDetails,
            fall_through_on_failure: bool,
            predecessor: Option<&mut ChoiceNode>,
        ) -> bool;
        fn get_quick_check_details(
            &mut self,
            details: &mut QuickCheckDetails,
            compiler: &mut RegExpCompiler,
            characters_filled_in: i32,
            not_at_start: bool,
        );
        fn get_quick_check_details_from_loop_entry(
            &mut self,
            details: &mut QuickCheckDetails,
            compiler: &mut RegExpCompiler,
            characters_filled_in: i32,
            not_at_start: bool,
        );
        fn greedy_loop_text_length(&self) -> i32;
        fn get_successor_of_omnivorous_text_node(&mut self, compiler: &mut RegExpCompiler) -> Option<&mut RegExpNode>;
        fn keep_recursing(&self, compiler: &mut RegExpCompiler) -> bool;
        fn fill_in_bm_info(&mut self, isolate: &mut Isolate, offset: i32, budget: i32, bm: &mut BoyerMooreLookahead, not_at_start: bool);
        fn filter_one_byte(&mut self, depth: i32, compiler: &mut RegExpCompiler) -> &mut RegExpNode;
        fn replacement(&self) -> Option<&RegExpNode>;
        fn set_replacement(&mut self, replacement: Option<&mut RegExpNode>) -> Option<&mut RegExpNode>;
        fn save_bm_info(&mut self, bm: &mut BoyerMooreLookahead, not_at_start: bool, offset: i32);
        fn label(&mut self) -> &mut Label;
        fn on_work_list(&self) -> bool;
        fn set_on_work_list(&mut self, value: bool);
        fn info(&mut self) -> &mut NodeInfo;
        fn eats_at_least_info(&self) -> &EatsAtLeastInfo;
        fn set_eats_at_least_info(&mut self, eats_at_least: EatsAtLeastInfo);
        fn set_do_not_inline(&mut self);
        fn bm_info(&self, not_at_start: bool) -> Option<&BoyerMooreLookahead>;

        fn as_end_node(&mut self) -> Option<&mut EndNode> { None }
        fn as_action_node(&mut self) -> Option<&mut ActionNode> { None }
        fn as_choice_node(&mut self) -> Option<&mut ChoiceNode> { None }
        fn as_loop_choice_node(&mut self) -> Option<&mut LoopChoiceNode> { None }
        fn as_negative_lookaround_choice_node(&mut self) -> Option<&mut NegativeLookaroundChoiceNode> { None }
        fn as_back_reference_node(&mut self) -> Option<&mut BackReferenceNode> { None }
        fn as_assertion_node(&mut self) -> Option<&mut AssertionNode> { None }
        fn as_text_node(&mut self) -> Option<&mut TextNode> { None }
        fn as_seq_reg_exp_node(&mut self) -> Option<&mut SeqRegExpNode> { None }

        fn zone(&self) -> &Zone;
        fn limit_versions(&mut self, compiler: &mut RegExpCompiler, trace: &mut Trace) -> LimitResult;

    }

    enum LimitResult {
        DONE,
        CONTINUE,
    }

    struct RegExpNodeBase {
        replacement_: Cell<Option<Box<dyn RegExpNodeTrait>>>,
        on_work_list_: Cell<bool>,
        trace_count_: Cell<i32>,
        zone_: Zone,
        bm_info_: [Cell<Option<Box<BoyerMooreLookahead>>>; 2],
        label_: Label,
        info_: NodeInfo,
        eats_at_least_: EatsAtLeastInfo,
    }

    impl RegExpNodeBase {
        fn new(zone: Zone) -> Self {
            RegExpNodeBase {
                replacement_: Cell::new(None),
                on_work_list_: Cell::new(false),
                trace_count_: Cell::new(0),
                zone_: zone,
                bm_info_: [Cell::new(None), Cell::new(None)],
                label_: Label {},
                info_: NodeInfo::default(),
                eats_at_least_: EatsAtLeastInfo::default(),
            }
        }
    }

    struct RegExpNode {
        base: RegExpNodeBase,
        vtable: &'static RegExpNodeVTable,
    }

    trait RegExpNodeMethods {
        fn accept(&mut self, visitor: &mut dyn NodeVisitor);
        fn emit(&mut self, compiler: &mut RegExpCompiler, trace: &mut Trace);
        fn get_quick_check_details(
            &mut self,
            details: &mut QuickCheckDetails,
            compiler: &mut RegExpCompiler,
            characters_filled_in: i32,
            not_at_start: bool,
        );
    }

    impl RegExpNode {
        fn new<T>(zone: Zone, vtable: &'static RegExpNodeVTable) -> Self {
            RegExpNode {
                base: RegExpNodeBase::new(zone),
                vtable,
            }
        }

        fn replacement(&self) -> Option<&RegExpNode> {
            self.base.replacement_.borrow().as_ref().map(|node| node.as_any().downcast_ref::<RegExpNode>().unwrap())
        }

        fn set_replacement(&mut self, replacement: Option<&mut RegExpNode>) -> Option<&mut RegExpNode> {
            self.base.info_.replacement_calculated = true;
            let replacement = replacement.map(|node| unsafe { &mut *(node as *mut RegExpNode) });
            self.base.replacement_.replace(replacement.map(|node| Box::new(node)));
            replacement
        }
    }

    impl RegExpNodeTrait for RegExpNode {
        fn accept(&mut self, visitor: &mut dyn NodeVisitor) {
            todo!()
        }

        fn emit(&mut self, compiler: &mut RegExpCompiler, trace: &mut Trace) {
            todo!()
        }

        fn eats_at_least(&self, not_at_start: bool) -> u32 {
            if self.base.info_.been_analyzed {
                if not_at_start {
                    self.base.eats_at_least_.eats_at_least_from_not_start as u32
                } else {
                    self.base.eats_at_least_.eats_at_least_from_possibly_start as u32
                }
            } else {
                0
            }
        }

        fn eats_at_least_from_loop_entry(&self) -> EatsAtLeastInfo {
            EatsAtLeastInfo::default()
        }

        fn emit_quick_check(
            &mut self,
            compiler: &mut RegExpCompiler,
            bounds_check_trace: &mut Trace,
            trace: &mut Trace,
            preload_has_checked_bounds: bool,
            on_possible_success: &mut Label,
            details_return: &mut QuickCheckDetails,
            fall_through_on_failure: bool,
            predecessor: Option<&mut ChoiceNode>,
        ) -> bool {
            false
        }

        fn get_quick_check_details(
            &mut self,
            details: &mut QuickCheckDetails,
            compiler: &mut RegExpCompiler,
            characters_filled_in: i32,
            not_at_start: bool,
        ) {
            todo!()
        }

        fn get_quick_check_details_from_loop_entry(
            &mut self,
            details: &mut QuickCheckDetails,
            compiler: &mut RegExpCompiler,
            characters_filled_in: i32,
            not_at_start: bool,
        ) {
            todo!()
        }

        fn greedy_loop_text_length(&self) -> i32 {
            K_MIN_INT
        }

        fn get_successor_of_omnivorous_text_node(&mut self, compiler: &mut RegExpCompiler) -> Option<&mut RegExpNode> {
            None
        }

        fn keep_recursing(&self, compiler: &mut RegExpCompiler) -> bool {
            self.base.trace_count_.get() < Self::K_RECURSION_BUDGET
        }

        fn fill_in_bm_info(&mut self, isolate: &mut Isolate, offset: i32, budget: i32, bm: &mut BoyerMooreLookahead, not_at_start: bool) {
            todo!()
        }

        fn filter_one_byte(&mut self, depth: i32, compiler: &mut RegExpCompiler) -> &mut RegExpNode {
            unsafe { &mut *(self as *mut RegExpNode) }
        }

        fn replacement(&self) -> Option<&RegExpNode> {
            if self.base.info_.replacement_calculated {
                self.base.replacement_.borrow().as_ref().map(|node| node.as_any().downcast_ref::<RegExpNode>().unwrap())
            } else {
                None
            }
        }

        fn set_replacement(&mut self, replacement: Option<&mut RegExpNode>) -> Option<&mut RegExpNode> {
            self.base.info_.replacement_calculated = true;
            let replacement = replacement.map(|node| unsafe { &mut *(node as *mut RegExpNode) });
            self.base.replacement_.replace(replacement.map(|node| Box::new(node)));
            replacement
        }

        fn save_bm_info(&mut self, bm: &mut BoyerMooreLookahead, not_at_start: bool, offset: i32) {
            if offset == 0 {
                self.set_bm_info(not_at_start, bm);
            }
        }

        fn label(&mut self) -> &mut Label {
            &mut self.base.label_
        }

        fn on_work_list(&self) -> bool {
            self.base.on_work_list_.get()
        }

        fn set_on_work_list(&mut self, value: bool) {
            self.base.on_work_list_.set(value);
        }

        fn info(&mut self) -> &mut NodeInfo {
            &mut self.base.info_
        }

        fn eats_at_least_info(&self) -> &EatsAtLeastInfo {
            &self.base.eats_at_least_
        }

        fn set_eats_at_least_info(&mut self, eats_at_least: EatsAtLeastInfo) {
            self.base.eats_at_least_ = eats_at_least;
        }

        fn set_do_not_inline(&mut self) {
            self.base.trace_count_.set(Self::K_MAX_COPIES_CODE_GENERATED);
        }

        fn bm_info(&self, not_at_start: bool) -> Option<&BoyerMooreLookahead> {
            self.base.bm_info_[if not_at_start { 1 } else { 0 }].borrow().as_ref().map(|node| node.as_ref())
        }

        fn zone(&self) -> &Zone {
            &self.base.zone_
        }

        fn limit_versions(&mut self, compiler: &mut RegExpCompiler, trace: &mut Trace) -> LimitResult {
            if self.base.trace_count_.get() >= Self::K_MAX_COPIES_CODE_GENERATED {
                return LimitResult::DONE;
            }
            self.base.trace_count_.set(self.base.trace_count_.get() + 1);
            LimitResult::CONTINUE
        }
    }

    impl RegExpNode {
        const K_RECURSION_BUDGET: i32 = 200;
        const K_MAX_COPIES_CODE_GENERATED: i32 = 10;
        const K_FIRST_CHAR_BUDGET: i32 = 10;

        fn set_bm_info(&mut self, not_at_start: bool, bm: &mut BoyerMooreLookahead) {
            self.base.bm_info_[if not_at_start { 1 } else { 0 }].replace(Some(Box::new(BoyerMooreLookahead{})));
        }
    }
    // The size of the vtable can be reduced by adding the methods of
    //  RegExpNodeMethods to RegExpNodeTrait and implementing them using a pointer to self.
    struct RegExpNodeVTable {
        accept: fn(&mut RegExpNode, &mut dyn NodeVisitor),
        emit: fn(&mut RegExpNode, &mut RegExpCompiler, &mut Trace),
        get_quick_check_details: fn(
            &mut RegExpNode,
            &mut QuickCheckDetails,
            &mut RegExpCompiler,
            i32,
            bool,
        ),
    }

    impl dyn RegExpNodeTrait {
        const K_RECURSION_BUDGET: i32 = 200;
    }

    trait AsAny {
        fn as_any(&self) -> &dyn std::any::Any;
    }

    impl AsAny for RegExpNode {
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }
    trait SeqRegExpNodeTrait {
        fn on_success(&self) -> &RegExpNode;
        fn set_on_success(&mut self, node: &mut RegExpNode);
        fn filter_one_byte(&mut self, depth: i32, compiler: &mut RegExpCompiler) -> &mut RegExpNode;
        fn fill_in_bm_info(&mut self, isolate: &mut Isolate, offset: i32, budget: i32, bm: &mut BoyerMooreLookahead, not_at_start: bool);
    }

    struct SeqRegExpNode {
        base: RegExpNodeBase,
        on_success_: Cell<Option<Box<dyn RegExpNodeTrait>>>,
    }

    impl SeqRegExpNode {
        fn new(on_success: &mut RegExpNode) -> Self {
            SeqRegExpNode {
                base: RegExpNodeBase::new(on_success.base.zone_),
                on_success_: Cell::new(Some(Box::new(unsafe { &mut *(on_success as *mut RegExpNode) } ))),
            }
        }
    }

    impl RegExpNodeTrait for SeqRegExpNode {
        fn accept(&mut self, visitor: &mut dyn NodeVisitor) {
            todo!()
        }

        fn emit(&mut self, compiler: &mut RegExpCompiler, trace: &mut Trace) {
            todo!()
        }

        fn eats_at_least(&self, not_at_start: bool) -> u32 {
            todo!()
        }

        fn eats_at_least_from_loop_entry(&self) -> EatsAtLeastInfo {
            todo!()
        }

        fn emit_quick_check(
            &mut self,
            compiler: &mut RegExpCompiler,
            bounds_check_trace: &mut Trace,
            trace: &mut Trace,
            preload_has_checked_bounds: bool,
            on_possible_success: &mut Label,
            details_return: &mut QuickCheckDetails,
            fall_through_on_failure: bool,
            predecessor: Option<&mut ChoiceNode>,
        ) -> bool {
            todo!()
        }

        fn get_quick_check_details(
            &mut self,
            details: &mut QuickCheckDetails,
            compiler: &mut RegExpCompiler,
            characters_filled_in: i32,
            not_at_start: bool,
        ) {
            todo!()
        }

        fn get_quick_check_details_from_loop_entry(
            &mut self,
            details: &mut QuickCheckDetails,
            compiler: &mut RegExpCompiler,
            characters_filled_in: i32,
            not_at_start: bool,
        ) {
            todo!()
        }

        fn greedy_loop_text_length(&self) -> i32 {
            todo!()
        }

        fn get_successor_of_omnivorous_text_node(&mut self, compiler: &mut RegExpCompiler) -> Option<&mut RegExpNode> {
            todo!()
        }

        fn keep_recursing(&self, compiler: &mut RegExpCompiler) -> bool {
            todo!()
        }

        fn fill_in_bm_info(&mut self, isolate: &mut Isolate, offset: i32, budget: i32, bm: &mut BoyerMooreLookahead, not_at_start: bool) {
            if let Some(on_success) = self.on_success_.borrow_mut().as_mut() {
                on_success.fill_in_bm_info(isolate, offset, budget - 1, bm, not_at_start);
            }
            if offset == 0 {
                self.save_bm_info(bm, not_at_start, offset);
            }
        }

        fn filter_one_byte(&mut self, depth: i32, compiler: &mut RegExpCompiler) -> &mut RegExpNode {
            if let Some(on_success) = self.on_success_.borrow_mut().as_mut() {
                on_success.filter_one_byte(depth, compiler);
            }
            unsafe { &mut *(self as *mut SeqRegExpNode as *mut RegExpNode)}
        }

        fn replacement(&self) -> Option<&RegExpNode> {
            todo!()
        }

        fn set_replacement(&mut self, replacement: Option<&mut RegExpNode>) -> Option<&mut RegExpNode> {
            todo!()
        }

        fn save_bm_info(&mut self, bm: &mut BoyerMooreLookahead, not_at_start: bool, offset: i32) {
            todo!()
        }

        fn label(&mut self) -> &mut Label {
            todo!()
        }

        fn on_work_list(&self) -> bool {
            todo!()
        }

        fn set_on_work_list(&mut self, value: bool) {
            todo!()
        }

        fn info(&mut self) -> &mut NodeInfo {
            todo!()
        }

        fn eats_at_least_info(&self) -> &EatsAtLeastInfo {
            todo!()
        }

        fn set_eats_at_least_info(&mut self, eats_at_least: EatsAtLeastInfo) {
            todo!()
        }

        fn set_do_not_inline(&mut self) {
            todo!()
        }

        fn bm_info(&self, not_at_start: bool) -> Option<&BoyerMooreLookahead> {
            todo!()
        }

        fn zone(&self) -> &Zone {
            &self.base.zone_
        }

        fn limit_versions(&mut self, compiler: &mut RegExpCompiler, trace: &mut Trace) -> LimitResult {
            todo!()
        }
    }

    impl SeqRegExpNodeTrait for SeqRegExpNode {
        fn on_success(&self) -> &RegExpNode {
            self.on_success_.borrow().as_ref().map(|node| node.as_any().downcast_ref::<RegExpNode>().unwrap()).unwrap()
        }

        fn set_on_success(&mut self, node: &mut RegExpNode) {
            self.on_success_.replace(Some(Box::new(unsafe { &mut *(node as *mut RegExpNode) } )));
        }

        fn filter_one_byte(&mut self, depth: i32, compiler: &mut RegExpCompiler) -> &mut RegExpNode {
            todo!()
        }

        fn fill_in_bm_info(&mut self, isolate: &mut Isolate, offset: i32, budget: i32, bm: &mut BoyerMooreLookahead, not_at_start: bool) {
            todo!()
        }
    }

    impl SeqRegExpNode {
        fn filter_successor(&mut self, depth: i32, compiler: &mut RegExpCompiler) -> &mut RegExpNode {
            if let Some(on_success) = self.on_success_.borrow_mut().as_mut() {
                 on_success.filter_one_byte(depth, compiler);
            }
            unsafe { &mut *(self as *mut SeqRegExpNode as *mut RegExpNode)}
        }
    }
    enum ActionType {
        SET_REGISTER_FOR_LOOP,
        INCREMENT_REGISTER,
        STORE_POSITION,
        BEGIN_POSITIVE_SUBMATCH,
        BEGIN_NEGATIVE_SUBMATCH,
        POSITIVE_SUBMATCH_SUCCESS,
        EMPTY_MATCH_CHECK,
        CLEAR_CAPTURES,
        MODIFY_FLAGS
    }

    union ActionData {
        u_store_register: StoreRegisterData,
        u_increment_register: IncrementRegisterData,
        u_position_register: PositionRegisterData,
        u_submatch: SubmatchData,
        u_empty_match_check: EmptyMatchCheckData,
        u_clear_captures: ClearCapturesData,
        u_modify_flags: ModifyFlagsData,
    }

    #[derive(Clone, Copy)]
    struct StoreRegisterData {
        reg: i32,
        value: i32,
    }

    #[derive(Clone, Copy)]
    struct IncrementRegisterData {
        reg: i32,
    }

    #[derive(Clone, Copy)]
    struct PositionRegisterData {
        reg: i32,
        is_capture: bool,
    }

    #[derive(Clone, Copy)]
    struct SubmatchData {
        stack_pointer_register: i32,
        current_position_register: i32,
        clear_register_count: i32,
        clear_register_from: i32,
        success_node: *mut ActionNode,  // Only used for positive submatch.
    }

    #[derive(Clone, Copy)]
    struct EmptyMatchCheckData {
        start_register: i32,
        repetition_register: i32,
        repetition_limit: i32,
    }

    #[derive(Clone, Copy)]
    struct ClearCapturesData {
        range_from: i32,
        range_to: i32,
    }

    #[derive(Clone, Copy)]
    struct ModifyFlagsData {
        flags: i32,
    }

    struct ActionNode {
        base: SeqRegExpNode,
        action_type_: ActionType,
        data_: ActionData,
    }

    impl ActionNode {
        fn set_register_for_loop(reg: i32, val: i32, on_success: &mut RegExpNode) -> Self {
            let mut node = ActionNode {
                base: SeqRegExpNode::new(on_success),
                action_type_: ActionType::SET_REGISTER_FOR_LOOP,
                data_: ActionData {
                    u_store_register: StoreRegisterData { reg, value: val },
                },
            };
            unsafe {
                node.data_.u_store_register.reg = reg;
                node.data_.u_store_register.value = val;
            }
            node
        }

        fn increment_register(reg: i32, on_success: &mut RegExpNode) -> Self {
            let mut node = ActionNode {
                base: SeqRegExpNode::new(on_success),
                action_type_: ActionType::INCREMENT_REGISTER,
                data_: ActionData {
                    u_increment_register: IncrementRegisterData { reg },
                },
            };
            unsafe {
                node.data_.u_increment_register.reg = reg;
            }
            node
        }

        fn store_position(reg: i32, is_capture: bool, on_success: &mut RegExpNode) -> Self {
            let mut node = ActionNode {
                base: SeqRegExpNode::new(on_success),
                action_type_: ActionType::STORE_POSITION,
                data_: ActionData {
                    u_position_register: PositionRegisterData { reg, is_capture },
                },
            };
            unsafe {
                node.data_.u_position_register.reg = reg;
                node.data_.u_position_register.is_capture = is_capture;
            }
            node
        }

        fn clear_captures(range: Interval, on_success: &mut RegExpNode) -> Self {
            let mut node = ActionNode {
                base: SeqRegExpNode::new(on_success),
                action_type_: ActionType::CLEAR_CAPTURES,
                data_: ActionData {
                    u_clear_captures: ClearCapturesData { range_from: 0, range_to: 0 }, // fixme
                },
            };
             unsafe {
                node.data_.u_clear_captures.range_from = 0;
                node.data_.u_clear_captures.range_to = 0;
            }
            node
        }

        fn begin_positive_submatch(stack_pointer_reg: i32, current_position_reg: i32, body: &mut RegExpNode, success_node: &mut ActionNode) -> Self {
            let mut node = ActionNode {
                base: SeqRegExpNode::new(body),
                action_type_: ActionType::BEGIN_POSITIVE_SUBMATCH,
                data_: ActionData {
                    u_submatch: SubmatchData {
                        stack_pointer_register: stack_pointer_reg,
                        current_position_register: current_position_reg,
                        clear_register_count: 0,
                        clear_register_from: 0,
                        success_node: success_node as *mut ActionNode,
                    },
                },
            };
            unsafe {
                node.data_.u_submatch.stack_pointer_register = stack_pointer_reg;
                node.data_.u_submatch.current_position_register = current_position_reg;
                node.data_.u_submatch.clear_register_count = 0;
                node.data_.u_submatch.clear_register_from = 0;
                node.data_.u_submatch.success_node = success_node as *mut ActionNode;
            }
            node
        }

        fn begin_negative_submatch(stack_pointer_reg: i32, current_position_reg: i32, on_success: &mut RegExpNode) -> Self {
            let mut node = ActionNode {
                base: SeqRegExpNode::new(on_success),
                action_type_: ActionType::BEGIN_NEGATIVE_SUBMATCH,
                data_: ActionData {
                    u_submatch: SubmatchData {
                        stack_pointer_register: stack_pointer_reg,
                        current_position_register: current_position_reg,
                        clear_register_count: 0,
                        clear_register_from: 0,
                        success_node: std::ptr::null_mut(),
                    },
                },
            };
            unsafe {
                node.data_.u_submatch.stack_pointer_register = stack_pointer_reg;
                node.data_.u_submatch.current_position_register = current_position_reg;
                node.data_.u_submatch.clear_register_count = 0;
                node.data_.u_submatch.clear_register_from = 0;
                node.data_.u_submatch.success_node = std::ptr::null_mut();
            }
            node
        }

        fn positive_submatch_success(stack_pointer_reg: i32, restore_reg: i32, clear_capture_count: i32, clear_capture_from: i32, on_success: &mut RegExp