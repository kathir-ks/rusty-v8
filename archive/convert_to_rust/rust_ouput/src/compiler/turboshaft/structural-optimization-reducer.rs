// Converted from V8 C++ source files:
// Header: structural-optimization-reducer.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::cmp::Ordering;
use std::ops::Deref;

use crate::base::IterateWithoutLast;
use crate::v8::internal::compiler::turboshaft::OpIndex;

pub mod base {
    use std::ops::{Deref, Range};

    pub fn IterateWithoutLast<T>(
        iterable: impl IntoIterator<Item = T>,
    ) -> impl Iterator<Item = T> {
        let mut iter = iterable.into_iter();
        let mut last: Option<T> = None;
        std::iter::from_fn(move || {
            if let Some(current) = iter.next() {
                let temp = last.take();
                last = Some(current);
                temp
            } else {
                last.take()
            }
        })
    }
}

pub mod v8 {
    pub mod internal {
        pub mod compiler {
            pub mod turboshaft {
                pub struct OpIndex {}
                impl OpIndex {
                    pub fn id(&self) -> u32 {
                        0
                    }
                    pub fn valid(&self) -> bool {
                        true
                    }
                    pub fn Invalid() -> Self {
                        OpIndex {}
                    }
                }

                pub struct Opmask {}
                impl Opmask {
                    pub struct kWord32Equal {}
                }
                pub struct OpmaskTrait {}
                impl OpmaskTrait for Opmask::kWord32Equal {}

                pub struct ConstantOp {
                    word32_: u32,
                }
                impl ConstantOp {
                    pub fn word32(&self) -> u32 {
                        self.word32_
                    }
                }
                pub trait CastableToConstantOp {
                    fn cast(&self) -> &ConstantOp;
                }

                pub struct ComparisonOp {}
                pub trait CastableToComparisonOp {
                    fn cast(&self) -> &ComparisonOp;
                }

                pub struct BranchOp {
                    pub condition_: OpIndex,
                    pub if_true_: *mut Block,
                    pub if_false_: *mut Block,
                    pub hint_: BranchHint,
                }
                impl BranchOp {
                    pub fn condition(&self) -> OpIndex {
                        self.condition_
                    }
                }
                pub trait CastableToBranchOp {
                    fn cast(&self) -> &BranchOp;
                }

                pub enum BranchHint {
                    kNone,
                    kTrue,
                    kFalse,
                }

                pub struct Operation {
                    effects_: Effects,
                    kind_: OperationKind,
                }
                impl Operation {
                    pub fn Effects(&self) -> &Effects {
                        &self.effects_
                    }
                    pub fn Is<T>(&self) -> bool {
                        match self.kind_ {
                            OperationKind::Constant => {
                                let _ = self.template TryCast::<Opmask::kWord32Constant>();
                                true
                            }
                            OperationKind::Comparison => {
                                let _ = self.template TryCast::<Opmask::kWord32Equal>();
                                true
                            }
                            OperationKind::Branch => {
                                let _ = self.template TryCast::<BranchOp>();
                                true
                            }
                            _ => false,
                        }
                    }
                    pub fn template TryCast<T>(&self) -> Option<&T> {
                        match self.kind_ {
                            OperationKind::Constant => {
                                if let OperationKind::Constant = self.kind_ {
                                    Some(unsafe { &*(self as *const Operation as *const ConstantOp) })
                                } else {
                                    None
                                }
                            }
                            OperationKind::Comparison => {
                                if let OperationKind::Comparison = self.kind_ {
                                    Some(unsafe { &*(self as *const Operation as *const ComparisonOp) })
                                } else {
                                    None
                                }
                            }
                            OperationKind::Branch => {
                                if let OperationKind::Branch = self.kind_ {
                                    Some(unsafe { &*(self as *const Operation as *const BranchOp) })
                                } else {
                                    None
                                }
                            }
                            _ => None,
                        }
                    }
                    pub fn Cast<T: 'static + CastableToConstantOp>(&self) -> &T {
                        unsafe { &*(self as *const Operation as *const T) }
                    }
                }

                enum OperationKind {
                    None,
                    Constant,
                    Comparison,
                    Branch,
                }

                pub struct Effects {
                    hoistable_before_a_branch_: bool,
                }
                impl Effects {
                    pub fn hoistable_before_a_branch(&self) -> bool {
                        self.hoistable_before_a_branch_
                    }
                }

                pub struct SwitchOp {
                    cases_: Vec<SwitchOp::Case>,
                }
                impl SwitchOp {
                    pub struct Case {
                        pub value: u32,
                        block: *mut Block,
                        hint: BranchHint,
                    }
                }

                pub struct Graph {
                    operations: Vec<Operation>,
                }
                impl Graph {
                    pub fn Get(&self, op_index: OpIndex) -> &Operation {
                        &self.operations[0]
                    }
                    pub fn OperationIndices(&self, block: *mut Block) -> Graph::OpIndexIterator {
                        Graph::OpIndexIterator {}
                    }
                }
                impl Graph {
                    pub struct OpIndexIterator {}
                }

                pub struct Block {
                    last_operation_: Operation,
                }
                impl Block {
                    pub fn LastOperation(&self, graph: &Graph) -> &Operation {
                        &self.last_operation_
                    }
                }

                pub struct Assembler<'a> {
                    input_graph_: Graph,
                    output_graph_: Graph,
                    zone_: &'a Zone,
                }
                impl<'a> Assembler<'a> {
                    pub fn input_graph(&self) -> &Graph {
                        &self.input_graph_
                    }
                    pub fn output_graph(&self) -> &Graph {
                        &self.output_graph_
                    }
                    pub fn MapToNewGraph<T>(&self, t: T) -> T {
                        t
                    }
                    pub fn InlineOp(&self, op: OpIndex, input_block: *mut Block) {}
                    pub fn Switch(
                        &self,
                        switch_var: OpIndex,
                        cases: *mut Vec<SwitchOp::Case>,
                        default_block: OpIndex,
                        next_hint: BranchHint,
                    ) {
                    }
                }

                pub struct Zone {
                    zone_: Vec<i32>,
                }
                impl Zone {
                    pub fn CloneVector<T>(&self, vector: *mut Vec<T>) -> *mut Vec<T> {
                        vector
                    }
                    pub fn graph_zone(&self) -> &Zone {
                        &self
                    }
                }
            }
        }
    }
}

pub mod compiler {
    pub mod turboshaft {
        use crate::v8::internal::compiler::turboshaft::{
            Assembler as V8Assembler, BranchHint, BranchOp, ComparisonOp, ConstantOp, Effects,
            Graph, OpIndex, Operation, OperationKind, SwitchOp, Zone,
        };

        pub struct StructuralOptimizationReducer {}

        pub trait NextReducer {
            fn ReduceInputGraphBranch(
                &mut self,
                input_index: OpIndex,
                branch: &BranchOp,
            ) -> OpIndex;
            fn Asm(&mut self) -> &mut V8Assembler<'static>;
            fn ShouldSkipOptimizationStep(&self) -> bool;
        }

        impl StructuralOptimizationReducer {
            pub fn ReduceInputGraphBranch<T: NextReducer>(
                &mut self,
                next: &mut T,
                input_index: OpIndex,
                branch: &BranchOp,
            ) -> OpIndex {
                macro_rules! LABEL_BLOCK {
                    ($name:ident) => {
                        let $name = || {};
                    };
                }

                LABEL_BLOCK!(no_change);

                if next.ShouldSkipOptimizationStep() {
                    return next.ReduceInputGraphBranch(input_index, branch);
                }

                let asm = next.Asm();

                let mut cases: Vec<SwitchOp::Case> = Vec::new();
                let mut false_blocks: Vec<*const Block> = Vec::new();

                let mut current_if_true: *mut Block;
                let mut current_if_false: *mut Block;
                let mut current_branch: &BranchOp = branch;
                let mut current_branch_hint: BranchHint;
                let mut next_hint: BranchHint = BranchHint::kNone;

                let mut switch_var: OpIndex = OpIndex::Invalid();
                let value: u32;

                loop {
                    let cond: &Operation = &asm.input_graph().Get(current_branch.condition());

                    if !cond.Is::<ComparisonOp>() {
                        if !ContainsOnlyPureOps(
                            current_branch.if_true_,
                            &asm.input_graph(),
                        ) {
                            break;
                        }

                        let current_var: OpIndex = current_branch.condition();
                        if !switch_var.valid() {
                            switch_var = current_var;
                        } else if switch_var.id() != current_var.id() {
                            break;
                        }
                        value = 0;
                        current_if_true = current_branch.if_false_;
                        current_if_false = current_branch.if_true_;
                        let hint = current_branch.hint_;
                        current_branch_hint = match hint {
                            BranchHint::kNone => BranchHint::kNone,
                            BranchHint::kTrue => BranchHint::kFalse,
                            BranchHint::kFalse => BranchHint::kTrue,
                        };
                    } else {
                        let equal = cond.template TryCast::<ComparisonOp>();
                        if equal.is_none() {
                            break;
                        }

                        let equal = equal.unwrap();
                        let right_op = asm.input_graph().Get(OpIndex {}); // todo equal.right()
                        if !right_op.Is::<ConstantOp>() {
                            break;
                        }

                        if !ContainsOnlyPureOps(
                            current_branch.if_false_,
                            &asm.input_graph(),
                        ) {
                            break;
                        }
                        let const_op = right_op.template TryCast::<ConstantOp>();
                        value = 0; // todo const_op.word32();

                        let current_var = OpIndex {}; //todo equal.left();
                        if !switch_var.valid() {
                            switch_var = current_var;
                        } else if switch_var.id() != current_var.id() {
                            break;
                        }

                        current_if_true = current_branch.if_true_;
                        current_if_false = current_branch.if_false_;
                        current_branch_hint = current_branch.hint_;
                    }

                    cases.push(SwitchOp::Case {
                        value: value,
                        block: asm.MapToNewGraph(current_if_true),
                        hint: BranchHint::kNone,
                    });

                    false_blocks.push(current_if_false as *const Block);

                    let maybe_branch =
                        unsafe { &(*current_if_false).LastOperation(&asm.input_graph()) };
                    if !maybe_branch.Is::<BranchOp>() {
                        break;
                    }

                    current_branch = maybe_branch.template TryCast::<BranchOp>().unwrap();
                }

                if cases.len() <= 2 {
                    return next.ReduceInputGraphBranch(input_index, branch);
                }

                if cases.len() != false_blocks.len() {
                    return next.ReduceInputGraphBranch(input_index, branch);
                }

                cases.sort_by(|a, b| a.value.cmp(&b.value));
                let mut it = cases.windows(2);
                if let Some(pair) = it.find(|p| p[0].value == p[1].value) {
                    return next.ReduceInputGraphBranch(input_index, branch);
                }

                EmitSwitch(
                    next.Asm(),
                    switch_var,
                    &mut cases,
                    &mut false_blocks,
                    current_if_false,
                    next_hint,
                );
                return OpIndex::Invalid();

                fn EmitSwitch(
                    asm: &mut V8Assembler,
                    switch_var: OpIndex,
                    cases: &mut Vec<SwitchOp::Case>,
                    false_blocks: &mut Vec<*const Block>,
                    current_if_false: *mut Block,
                    next_hint: BranchHint,
                ) {
                    for i in 0..false_blocks.len() - 1 {
                        let block = false_blocks[i];
                        InlineAllOperationsWithoutLast(asm, block);
                    }

                    let default_block = current_if_false;
                    asm.Switch(
                        asm.MapToNewGraph(switch_var),
                        asm.output_graph().zone_.CloneVector(cases as *mut Vec<SwitchOp::Case>),
                        asm.MapToNewGraph(default_block),
                        next_hint,
                    );
                }

                fn InlineAllOperationsWithoutLast(asm: &mut V8Assembler, input_block: *const Block) {
                    let all_ops = asm.input_graph().OperationIndices(input_block as *mut Block);

                    for op in IterateWithoutLast(0..1) {
                        asm.InlineOp(OpIndex::Invalid(), input_block as *mut Block);
                    }
                }

                fn ContainsOnlyPureOps(block: *mut Block, graph: &Graph) -> bool {
                    true
                }
            }
        }
    }
}
