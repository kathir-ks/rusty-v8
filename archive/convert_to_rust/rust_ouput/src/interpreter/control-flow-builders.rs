// Converted from V8 C++ source files:
// Header: control-flow-builders.h
// Implementation: control-flow-builders.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod interpreter {
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::ast::AstNode;
use crate::ast::BreakableStatement;
use crate::ast::CaseClause;
use crate::ast::ConditionalChain;
use crate::ast::IfStatement;
use crate::ast::SourceRangeKind;
use crate::ast::SwitchStatement;
use crate::ast::TryCatchStatement;
use crate::ast::TryFinallyStatement;
use crate::interpreter::BytecodeArrayBuilder;
use crate::interpreter::BytecodeLabel;
use crate::interpreter::BytecodeLabels;
use crate::interpreter::BytecodeJumpTable;
use crate::interpreter::BlockCoverageBuilder;
use crate::interpreter::Bytecode;
use crate::interpreter::Register;
use crate::interpreter::ToBooleanMode;
use crate::interpreter::FeedbackVectorSpec;
use crate::objects::FeedbackVector;

pub struct ControlFlowBuilder<'a> {
    builder_: &'a mut BytecodeArrayBuilder,
}

impl<'a> ControlFlowBuilder<'a> {
    pub fn new(builder: &'a mut BytecodeArrayBuilder) -> Self {
        ControlFlowBuilder { builder_: builder }
    }

    pub fn builder(&self) -> &mut BytecodeArrayBuilder {
        self.builder_
    }
}

pub struct BreakableControlFlowBuilder<'a> {
    base: ControlFlowBuilder<'a>,
    break_labels_: BytecodeLabels,
    node_: *mut AstNode,
    block_coverage_builder_: *mut BlockCoverageBuilder,
}

impl<'a> BreakableControlFlowBuilder<'a> {
    pub fn new(
        builder: &'a mut BytecodeArrayBuilder,
        block_coverage_builder: *mut BlockCoverageBuilder,
        node: *mut AstNode,
    ) -> Self {
        BreakableControlFlowBuilder {
            base: ControlFlowBuilder::new(builder),
            break_labels_: BytecodeLabels::new(builder.zone()),
            node_: node,
            block_coverage_builder_: block_coverage_builder,
        }
    }

    pub fn Break(&mut self) {
        self.EmitJump(&mut self.break_labels_);
    }

    pub fn BreakIfTrue(&mut self, mode: ToBooleanMode) {
        self.EmitJumpIfTrue(mode, &mut self.break_labels_);
    }

    pub fn BreakIfForInDone(&mut self, index: Register, cache_length: Register) {
        self.EmitJumpIfForInDone(&mut self.break_labels_, index, cache_length);
    }

    pub fn break_labels(&mut self) -> &mut BytecodeLabels {
        &mut self.break_labels_
    }

    fn EmitJump(&mut self, labels: &mut BytecodeLabels) {
        self.base.builder().Jump(labels.New()).unwrap();
    }

    fn EmitJumpIfTrue(&mut self, mode: ToBooleanMode, labels: &mut BytecodeLabels) {
        self.base.builder().JumpIfTrue(mode, labels.New()).unwrap();
    }

    fn EmitJumpIfFalse(&mut self, mode: ToBooleanMode, labels: &mut BytecodeLabels) {
        self.base.builder().JumpIfFalse(mode, labels.New()).unwrap();
    }

    fn EmitJumpIfUndefined(&mut self, labels: &mut BytecodeLabels) {
        self.base.builder().JumpIfUndefined(labels.New()).unwrap();
    }

    fn EmitJumpIfForInDone(
        &mut self,
        labels: &mut BytecodeLabels,
        index: Register,
        cache_length: Register,
    ) {
        self.base.builder().JumpIfForInDone(labels.New(), index, cache_length).unwrap();
    }

    fn BindBreakTarget(&mut self) {
        self.break_labels_.Bind(self.base.builder());
    }
}

impl<'a> Drop for BreakableControlFlowBuilder<'a> {
    fn drop(&mut self) {
        self.BindBreakTarget();
        if !self.break_labels_.empty() && !self.break_labels_.is_bound() {
            panic!("Break labels should be bound or empty");
        }

        // This part is hard to convert perfectly due to raw pointers
        // and the block_coverage_builder's IncrementBlockCounter API.
        // Here's a simplified approach.

        if self.block_coverage_builder_.is_null() == false {
            let block_coverage_builder = unsafe { &mut *self.block_coverage_builder_ };
                block_coverage_builder.IncrementBlockCounter(unsafe {&mut *self.node_}, SourceRangeKind::kContinuation);
        }
    }
}

pub struct BlockBuilder<'a> {
    base: BreakableControlFlowBuilder<'a>,
}

impl<'a> BlockBuilder<'a> {
    pub fn new(
        builder: &'a mut BytecodeArrayBuilder,
        block_coverage_builder: *mut BlockCoverageBuilder,
        statement: *mut BreakableStatement,
    ) -> Self {
        BlockBuilder {
            base: BreakableControlFlowBuilder::new(
                builder,
                block_coverage_builder,
                statement as *mut AstNode,
            ),
        }
    }
}

pub struct LoopBuilder<'a> {
    base: BreakableControlFlowBuilder<'a>,
    continue_labels_: BytecodeLabels,
    end_labels_: BytecodeLabels,
    block_coverage_body_slot_: i32,
    source_position_: i32,
    feedback_vector_spec_: *mut FeedbackVectorSpec,
    loop_header_: BytecodeLoopHeader,
}

#[derive(Default)]
struct BytecodeLoopHeader {
    offset_: usize
}

impl BytecodeLoopHeader {
    pub fn offset(&self) -> usize {
        self.offset_
    }
    pub fn new() -> BytecodeLoopHeader {
        BytecodeLoopHeader{
            offset_: 0
        }
    }
}

const kNoSourcePosition: i32 = -1;

impl<'a> LoopBuilder<'a> {
    pub fn new(
        builder: &'a mut BytecodeArrayBuilder,
        block_coverage_builder: *mut BlockCoverageBuilder,
        node: *mut AstNode,
        feedback_vector_spec: *mut FeedbackVectorSpec,
    ) -> Self {
        let mut block_coverage_body_slot_ = 0;
        if block_coverage_builder.is_null() == false {
            let block_coverage_builder_ref = unsafe {&mut *block_coverage_builder};
            let node_ref = unsafe {&mut *node};
                block_coverage_body_slot_ =
                block_coverage_builder_ref.AllocateBlockCoverageSlot(
                    node_ref,
                    SourceRangeKind::kBody,
                );
        }
        let source_position_ = if node.is_null() {
            kNoSourcePosition
        } else {
            unsafe { (*node).position() }
        };

        LoopBuilder {
            base: BreakableControlFlowBuilder::new(builder, block_coverage_builder, node),
            continue_labels_: BytecodeLabels::new(builder.zone()),
            end_labels_: BytecodeLabels::new(builder.zone()),
            block_coverage_body_slot_: block_coverage_body_slot_,
            source_position_: source_position_,
            feedback_vector_spec_: feedback_vector_spec,
            loop_header_: BytecodeLoopHeader::new(),
        }
    }

    pub fn LoopHeader(&mut self) {
        if !self.base.break_labels_.empty()
            || !self.base.break_labels_.empty()
            || !self.end_labels_.empty()
        {
            println!("DCHECK(break_labels_.empty() && continue_labels_.empty() && end_labels_.empty())");
        }

        self.base.builder().Bind(&mut self.loop_header_).unwrap();
    }

    pub fn LoopBody(&mut self) {
        if !self.base.block_coverage_builder_.is_null() {
            unsafe {
            let block_coverage_builder = &mut *self.base.block_coverage_builder_;
            block_coverage_builder.IncrementBlockCounter(self.block_coverage_body_slot_);
            }
        }
    }

    pub fn JumpToHeader(&mut self, loop_depth: i32, parent_loop: *mut LoopBuilder) {
        self.BindLoopEnd();
        if !parent_loop.is_null() &&
            self.loop_header_.offset() == unsafe { (*parent_loop).loop_header_.offset() } {
            unsafe { (*parent_loop).JumpToLoopEnd() };
        } else {
            let slot_index;
            unsafe {
                let feedback_vector_spec = &mut *self.feedback_vector_spec_;
                slot_index = feedback_vector_spec.AddJumpLoopSlot().ToInt();
            }

            self.base.builder().JumpLoop(
                &mut self.loop_header_,
                std::cmp::min(loop_depth, FeedbackVector::kMaxOsrUrgency as i32 - 1),
                self.source_position_,
                slot_index,
            ).unwrap();
        }
    }

    fn JumpToLoopEnd(&mut self) {
        self.EmitJump(&mut self.end_labels_);
    }

    pub fn BindContinueTarget(&mut self) {
        self.continue_labels_.Bind(self.base.builder());
    }

    fn BindLoopEnd(&mut self) {
        self.end_labels_.Bind(self.base.builder());
    }

    pub fn Continue(&mut self) {
        self.EmitJump(&mut self.continue_labels_);
    }

    pub fn ContinueIfUndefined(&mut self) {
        self.EmitJumpIfUndefined(&mut self.continue_labels_);
    }

    fn EmitJump(&mut self, labels: &mut BytecodeLabels) {
        self.base.builder().Jump(labels.New()).unwrap();
    }

    fn EmitJumpIfUndefined(&mut self, labels: &mut BytecodeLabels) {
        self.base.builder().JumpIfUndefined(labels.New()).unwrap();
    }
}

impl<'a> Drop for LoopBuilder<'a> {
    fn drop(&mut self) {
        if !self.continue_labels_.empty() && !self.continue_labels_.is_bound() {
            panic!("Continue labels should be bound or empty");
        }
        if !self.end_labels_.empty() && !self.end_labels_.is_bound() {
            panic!("End labels should be bound or empty");
        }
    }
}

pub struct SwitchBuilder<'a> {
    base: BreakableControlFlowBuilder<'a>,
    case_sites_: Vec<BytecodeLabel>,
    default_: BytecodeLabels,
    fall_through_: BytecodeLabels,
    jump_table_: *mut BytecodeJumpTable,
}

impl<'a> SwitchBuilder<'a> {
    pub fn new(
        builder: &'a mut BytecodeArrayBuilder,
        block_coverage_builder: *mut BlockCoverageBuilder,
        statement: *mut SwitchStatement,
        number_of_cases: i32,
        jump_table: *mut BytecodeJumpTable,
    ) -> Self {
        SwitchBuilder {
            base: BreakableControlFlowBuilder::new(
                builder,
                block_coverage_builder,
                statement as *mut AstNode,
            ),
            case_sites_: (0..number_of_cases).map(|_| BytecodeLabel::new()).collect(),
            default_: BytecodeLabels::new(builder.zone()),
            fall_through_: BytecodeLabels::new(builder.zone()),
            jump_table_: jump_table,
        }
    }

    pub fn BindCaseTargetForJumpTable(&mut self, case_value: i32, clause: *mut CaseClause) {
        unsafe {
        self.base.builder().Bind(
            &mut *self.jump_table_,
            case_value,
        ).unwrap();
        }
        self.BuildBlockCoverage(clause);
    }

    pub fn BindCaseTargetForCompareJump(&mut self, index: usize, clause: *mut CaseClause) {
        self.base.builder().Bind(&mut self.case_sites_[index]).unwrap();
        self.BuildBlockCoverage(clause);
    }

    pub fn JumpToCaseIfTrue(&mut self, mode: ToBooleanMode, index: usize) {
        self.base.builder().JumpIfTrue(mode, &self.case_sites_[index]).unwrap();
    }

    pub fn EmitJumpTableIfExists(
        &mut self,
        min_case: i32,
        max_case: i32,
        covered_cases: &mut HashMap<i32, *mut CaseClause>,
    ) {
        unsafe{
            self.base.builder().SwitchOnSmiNoFeedback(&mut *self.jump_table_).unwrap();
        }
        self.fall_through_.Bind(self.base.builder());
        for j in min_case..=max_case {
            if covered_cases.get(&j).is_none() {
                self.BindCaseTargetForJumpTable(j, std::ptr::null_mut());
            }
        }
    }

    pub fn BindDefault(&mut self, clause: *mut CaseClause) {
        self.default_.Bind(self.base.builder());
        self.BuildBlockCoverage(clause);
    }

    pub fn JumpToDefault(&mut self) {
        self.EmitJump(&mut self.default_);
    }

    pub fn JumpToFallThroughIfFalse(&mut self) {
        self.EmitJumpIfFalse(ToBooleanMode::kAlreadyBoolean, &mut self.fall_through_);
    }

    fn BuildBlockCoverage(&mut self, clause: *mut CaseClause) {
        if !self.base.block_coverage_builder_.is_null() && !clause.is_null() {
            let block_coverage_builder_ref = unsafe {&mut *self.base.block_coverage_builder_};
            let clause_ref = unsafe { &mut *clause };
            block_coverage_builder_ref.IncrementBlockCounter(clause_ref, SourceRangeKind::kBody);
        }
    }

    fn EmitJump(&mut self, labels: &mut BytecodeLabels) {
        self.base.builder().Jump(labels.New()).unwrap();
    }

    fn EmitJumpIfFalse(&mut self, mode: ToBooleanMode, labels: &mut BytecodeLabels) {
        self.base.builder().JumpIfFalse(mode, labels.New()).unwrap();
    }
}

impl<'a> Drop for SwitchBuilder<'a> {
    fn drop(&mut self) {
        for site in &self.case_sites_ {
            if site.has_referrer_jump() && !site.is_bound() {
                panic!("Case sites should be bound if they have a referrer jump");
            }
        }
    }
}

pub struct TryCatchBuilder<'a> {
    base: ControlFlowBuilder<'a>,
    handler_id_: i32,
    catch_prediction_: HandlerTable::CatchPrediction,
    exit_: BytecodeLabel,
    block_coverage_builder_: *mut BlockCoverageBuilder,
    statement_: *mut TryCatchStatement,
}

impl<'a> TryCatchBuilder<'a> {
    pub fn new(
        builder: &'a mut BytecodeArrayBuilder,
        block_coverage_builder: *mut BlockCoverageBuilder,
        statement: *mut TryCatchStatement,
        catch_prediction: HandlerTable::CatchPrediction,
    ) -> Self {
        let handler_id_ = builder.NewHandlerEntry();
        TryCatchBuilder {
            base: ControlFlowBuilder::new(builder),
            handler_id_: handler_id_,
            catch_prediction_: catch_prediction,
            exit_: BytecodeLabel::new(),
            block_coverage_builder_: block_coverage_builder,
            statement_: statement,
        }
    }

    pub fn BeginTry(&mut self, context: Register) {
        self.base.builder().MarkTryBegin(self.handler_id_, context).unwrap();
    }

    pub fn EndTry(&mut self) {
        self.base.builder().MarkTryEnd(self.handler_id_).unwrap();
        self.base.builder().Jump(&mut self.exit_).unwrap();
        self.base.builder().MarkHandler(self.handler_id_, self.catch_prediction_).unwrap();
        if !self.block_coverage_builder_.is_null() {
            unsafe {
            let block_coverage_builder = &mut *self.block_coverage_builder_;
            block_coverage_builder.IncrementBlockCounter(&mut *self.statement_, SourceRangeKind::kCatch);
            }
        }
    }

    pub fn EndCatch(&mut self) {
        self.base.builder().Bind(&mut self.exit_).unwrap();
    }
}

impl<'a> Drop for TryCatchBuilder<'a> {
    fn drop(&mut self) {
        if !self.block_coverage_builder_.is_null() {
            unsafe {
            let block_coverage_builder = &mut *self.block_coverage_builder_;
            block_coverage_builder.IncrementBlockCounter(&mut *self.statement_, SourceRangeKind::kContinuation);
            }
        }
    }
}

pub struct TryFinallyBuilder<'a> {
    base: ControlFlowBuilder<'a>,
    handler_id_: i32,
    catch_prediction_: HandlerTable::CatchPrediction,
    handler_: BytecodeLabel,
    finalization_sites_: BytecodeLabels,
    block_coverage_builder_: *mut BlockCoverageBuilder,
    statement_: *mut TryFinallyStatement,
}

impl<'a> TryFinallyBuilder<'a> {
    pub fn new(
        builder: &'a mut BytecodeArrayBuilder,
        block_coverage_builder: *mut BlockCoverageBuilder,
        statement: *mut TryFinallyStatement,
        catch_prediction: HandlerTable::CatchPrediction,
    ) -> Self {
        let handler_id_ = builder.NewHandlerEntry();
        TryFinallyBuilder {
            base: ControlFlowBuilder::new(builder),
            handler_id_: handler_id_,
            catch_prediction_: catch_prediction,
            handler_: BytecodeLabel::new(),
            finalization_sites_: BytecodeLabels::new(builder.zone()),
            block_coverage_builder_: block_coverage_builder,
            statement_: statement,
        }
    }

    pub fn BeginTry(&mut self, context: Register) {
        self.base.builder().MarkTryBegin(self.handler_id_, context).unwrap();
    }

    pub fn LeaveTry(&mut self) {
        self.base.builder().Jump(self.finalization_sites_.New()).unwrap();
    }

    pub fn EndTry(&mut self) {
        self.base.builder().MarkTryEnd(self.handler_id_).unwrap();
    }

    pub fn BeginHandler(&mut self) {
        self.base.builder().Bind(&mut self.handler_).unwrap();
        self.base.builder().MarkHandler(self.handler_id_, self.catch_prediction_).unwrap();
    }

    pub fn BeginFinally(&mut self) {
        self.finalization_sites_.Bind(self.base.builder());
        if !self.block_coverage_builder_.is_null() {
             unsafe {
            let block_coverage_builder = &mut *self.block_coverage_builder_;
            block_coverage_builder.IncrementBlockCounter(&mut *self.statement_, SourceRangeKind::kFinally);
             }
        }
    }

    pub fn EndFinally(&mut self) {}
}

impl<'a> Drop for TryFinallyBuilder<'a> {
    fn drop(&mut self) {
        if !self.block_coverage_builder_.is_null() {
             unsafe {
            let block_coverage_builder = &mut *self.block_coverage_builder_;
            block_coverage_builder.IncrementBlockCounter(&mut *self.statement_, SourceRangeKind::kContinuation);
             }
        }
    }
}

pub struct ConditionalChainControlFlowBuilder<'a> {
    base: ControlFlowBuilder<'a>,
    end_labels_: BytecodeLabels,
    then_count_: usize,
    then_labels_list_: Vec<BytecodeLabels>,
    else_labels_list_: Vec<BytecodeLabels>,
    block_coverage_then_slots_: Vec<i32>,
    block_coverage_else_slots_: Vec<i32>,
    block_coverage_builder_: *mut BlockCoverageBuilder,
}

impl<'a> ConditionalChainControlFlowBuilder<'a> {
    pub fn new(
        builder: &'a mut BytecodeArrayBuilder,
        block_coverage_builder: *mut BlockCoverageBuilder,
        node: *mut AstNode,
        then_count: usize,
    ) -> Self {
        assert!(unsafe { (*node).IsConditionalChain() });

        let mut then_labels_list_ = Vec::new();
        let mut else_labels_list_ = Vec::new();

        for _ in 0..then_count {
            then_labels_list_.push(BytecodeLabels::new(builder.zone()));
            else_labels_list_.push(BytecodeLabels::new(builder.zone()));
        }

        let mut block_coverage_then_slots_ = Vec::new();
        let mut block_coverage_else_slots_ = Vec::new();

        if !block_coverage_builder.is_null() {
            unsafe {
                let conditional_chain = &mut *node;
                let conditional_chain_struct = conditional_chain.AsConditionalChain();
                let block_coverage_builder_ref = &mut *block_coverage_builder;

                for i in 0..then_count {
                    block_coverage_then_slots_.push(
                        block_coverage_builder_ref
                            .AllocateConditionalChainBlockCoverageSlot(
                                conditional_chain_struct,
                                SourceRangeKind::kThen,
                                i as usize,
                            ),
                    );
                    block_coverage_else_slots_.push(
                        block_coverage_builder_ref
                            .AllocateConditionalChainBlockCoverageSlot(
                                conditional_chain_struct,
                                SourceRangeKind::kElse,
                                i as usize,
                            ),
                    );
                }
            }
        }

        ConditionalChainControlFlowBuilder {
            base: ControlFlowBuilder::new(builder),
            end_labels_: BytecodeLabels::new(builder.zone()),
            then_count_: then_count,
            then_labels_list_: then_labels_list_,
            else_labels_list_: else_labels_list_,
            block_coverage_then_slots_: block_coverage_then_slots_,
            block_coverage_else_slots_: block_coverage_else_slots_,
            block_coverage_builder_: block_coverage_builder,
        }
    }

    pub fn then_labels_at(&mut self, index: usize) -> &mut BytecodeLabels {
        assert!(index < self.then_count_);
        &mut self.then_labels_list_[index]
    }

    pub fn else_labels_at(&mut self, index: usize) -> &mut BytecodeLabels {
        assert!(index < self.then_count_);
        &mut self.else_labels_list_[index]
    }

    pub fn block_coverage_then_slot_at(&self, index: usize) -> i32 {
        assert!(index < self.then_count_);
        self.block_coverage_then_slots_[index]
    }

    pub fn block_coverage_else_slot_at(&self, index: usize) -> i32 {
        assert!(index < self.then_count_);
        self.block_coverage_else_slots_[index]
    }

    pub fn ThenAt(&mut self, index: usize) {
        assert!(index < self.then_labels_list_.len());
        self.then_labels_at(index).Bind(self.base.builder());

        if !self.block_coverage_builder_.is_null() {
            unsafe {
            let block_coverage_builder = &mut *self.block_coverage_builder_;
            block_coverage_builder.IncrementBlockCounter(self.block_coverage_then_slot_at(index));
            }
        }
    }

    pub fn ElseAt(&mut self, index: usize) {
        assert!(index < self.else_labels_list_.len());
        self.else_labels_at(index).Bind(self.base.builder());
        if !self.block_coverage_builder_.is_null() {
             unsafe {
            let block_coverage_builder = &mut *self.block_coverage_builder_;
            block_coverage_builder.IncrementBlockCounter(self.block_coverage_else_slot_at(index));
             }
        }
    }

    pub fn JumpToEnd(&mut self) {
        self.base.builder().Jump(self.end_labels_.New()).unwrap();
    }
}

impl<'a> Drop for ConditionalChainControlFlowBuilder<'a> {
    fn drop(&mut self) {
        self.end_labels_.Bind(self.base.builder());

        assert!(self.end_labels_.empty() || self.end_labels_.is_bound());

        for label in &self.then_labels_list_ {
            assert!(label.empty() || label.is_bound());
        }

        for label in &self.else_labels_list_ {
            assert!(label.empty() || label.is_bound());
        }
    }
}

pub struct ConditionalControlFlowBuilder<'a> {
    base: ControlFlowBuilder<'a>,
    end_labels_: BytecodeLabels,
    then_labels_: BytecodeLabels,
    else_labels_: BytecodeLabels,
    node_: *mut AstNode,
    block_coverage_then_slot_: i32,
    block_coverage_else_slot_: i32,
    block_coverage_builder_: *mut BlockCoverageBuilder,
}

impl<'a> ConditionalControlFlowBuilder<'a> {
    pub fn new(
        builder: &'a mut BytecodeArrayBuilder,
        block_coverage_builder: *mut BlockCoverageBuilder,
        node: *mut AstNode,
    ) -> Self {
        assert!(unsafe { (*node).IsIfStatement() } || unsafe { (*node).IsConditional() });

        let mut block_coverage_then_slot_ = 0;
        let mut block_coverage_else_slot_ = 0;

        if !block_coverage_builder.is_null() {
             unsafe {
            let block_coverage_builder_ref = &mut *block_coverage_builder;
            let node_ref = &mut *node;
            block_coverage_then_slot_ =
                block_coverage_builder_ref.AllocateBlockCoverageSlot(
                    node_ref,
                    SourceRangeKind::kThen,
                );
            block_coverage_else_slot_ =
                block_coverage_builder_ref.AllocateBlockCoverageSlot(
                    node_ref,
                    SourceRangeKind::kElse,
                );
             }
        }

        ConditionalControlFlowBuilder {
            base: ControlFlowBuilder::new(builder),
            end_labels_: BytecodeLabels::new(builder.zone()),
            then_labels_: BytecodeLabels::new(builder.zone()),
            else_labels_: BytecodeLabels::new(builder.zone()),
            node_: node,
            block_coverage_then_slot_: block_coverage_then_slot_,
            block_coverage_else_slot_: block_coverage_else_slot_,
            block_coverage_builder_: block_coverage_builder,
        }
    }

    pub fn then_labels(&mut self) -> &mut BytecodeLabels {
        &mut self.then_labels_
    }

    pub fn else_labels(&mut self) -> &mut BytecodeLabels {
        &mut self.else_labels_
    }

    pub fn Then(&mut self) {
        self.then_labels().Bind(self.base.builder());
        if !self.block_coverage_builder_.is_null() {
             unsafe {
            let block_coverage_builder = &mut *self.block_coverage_builder_;
            block_coverage_builder.IncrementBlockCounter(self.block_coverage_then_slot_);
             }
        }
    }

    pub fn Else(&mut self) {
        self.else_labels().Bind(self.base.builder());
        if !self.block_coverage_builder_.is_null() {
             unsafe {
            let block_coverage_builder = &mut *self.block_coverage_builder_;
            block_coverage_builder.IncrementBlockCounter(self.block_coverage_else_slot_);
             }
        }
    }

    pub fn JumpToEnd(&mut self) {
        assert!(self.end_labels_.empty()); // May only be called once.
        self.base.builder().Jump(self.end_labels_.New()).unwrap();
    }
}

impl<'a> Drop for ConditionalControlFlowBuilder<'a> {
    fn drop(&mut self) {
        if !self.else_labels_.is_bound() {
            self.else_labels_.Bind(self.base.builder());
        }
        self.end_labels_.Bind(self.base.builder());

        assert!(self.end_labels_.empty() || self.end_labels_.is_bound());
        assert!(self.then_labels_.empty() || self.then_labels_.is_bound());
        assert!(self.else_labels_.empty() || self.else_labels_.is_bound());

        // IfStatement requires a continuation counter, Conditional does not (as it
        // can only contain expressions).
        if !self.block_coverage_builder_.is_null() && unsafe { (*self.node_).IsIfStatement() } {
             unsafe {
            let block_coverage_builder = &mut *self.block_coverage_builder_;
            block_coverage_builder.IncrementBlockCounter(&mut *self.node_, SourceRangeKind::kContinuation);
             }
        }
    }
}

pub mod HandlerTable {
    #[derive(Debug, Clone, Copy)]
    pub enum CatchPrediction {
        kUnknown,
    }
}
}  // namespace interpreter
