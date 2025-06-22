// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod type_inference_analysis {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    use crate::compiler::turboshaft::assembler::*;
    use crate::compiler::turboshaft::operations::*;
    use crate::compiler::turboshaft::representations::*;
    use crate::compiler::turboshaft::sidetable::*;
    use crate::compiler::turboshaft::snapshot_table::*;
    use crate::compiler::turboshaft::typer::*;
    use crate::compiler::turboshaft::types::*;

    pub struct TypeInferenceAnalysis<'a> {
        graph: &'a Graph,
        types: GrowingOpIndexSidetable<Type>,
        table: SnapshotTable<Type>,
        op_to_key_mapping: GrowingOpIndexSidetable<Option<Key>>,
        block_to_snapshot_mapping: GrowingBlockSidetable<Option<Snapshot>>,
        predecessors: RefCell<Vec<Snapshot>>,
        graph_zone: &'a Zone,

        #[cfg(debug_assertions)]
        block_refinements: RefCell<Option<GrowingBlockSidetable<Vec<(OpIndex, Type)>>>>,
    }

    impl<'a> TypeInferenceAnalysis<'a> {
        pub fn new(graph: &'a Graph, phase_zone: &'a Zone) -> Self {
            TypeInferenceAnalysis {
                graph,
                types: GrowingOpIndexSidetable::new(graph.op_id_count(), Type::None(), &graph),
                table: SnapshotTable::new(phase_zone),
                op_to_key_mapping: GrowingOpIndexSidetable::new_empty(&graph),
                block_to_snapshot_mapping: GrowingBlockSidetable::new(graph.block_count(), None, phase_zone),
                predecessors: RefCell::new(Vec::new()),
                graph_zone: graph.graph_zone(),
                #[cfg(debug_assertions)]
                block_refinements: RefCell::new(None),
            }
        }

        pub fn run(&self, block_refinements: Option<&mut GrowingBlockSidetable<Vec<(OpIndex, Type)>>>) -> GrowingOpIndexSidetable<Type> {
            #[cfg(debug_assertions)]
            {
                *self.block_refinements.borrow_mut() = block_refinements.map(|br| br.clone());
            }

            if cfg!(debug_assertions) && V8_FLAGS.turboshaft_trace_typing {
                println!("=== Running Type Inference Analysis ===");
            }
            let mut unprocessed_index: u32 = 0;
            while unprocessed_index < self.graph.block_count() {
                let block_index = BlockIndex::from(unprocessed_index);
                unprocessed_index += 1;
                let block = self.graph.get(block_index);

                if cfg!(debug_assertions) && V8_FLAGS.turboshaft_trace_typing {
                    let block_info = format!("{} {}", block.kind(), block.index().id());
                    println!("=== {} ===", block_info);
                }

                self.process_block::<false>(&block, &mut unprocessed_index);
            }

            if cfg!(debug_assertions) && V8_FLAGS.turboshaft_trace_typing {
                println!("=== Completed Type Inference Analysis ===");
            }
            self.types.clone()
        }

        fn process_block<const REVISIT_LOOP_HEADER: bool>(&self, block: &Block, unprocessed_index: &mut u32) {
            if REVISIT_LOOP_HEADER {
              assert!(block.is_loop());
            }

            if self.table.is_sealed() {
                assert!(self.current_block().is_none());
            } else {
                assert!(self.current_block().is_some());
                let current_block = self.current_block().unwrap();
                assert!(current_block.index().is_valid());
                self.block_to_snapshot_mapping.set(current_block.index(), Some(self.table.seal()));
                self.set_current_block(None);
            }

            let mut predecessors = self.predecessors.borrow_mut();
            predecessors.clear();
            for pred in block.predecessors_iterable() {
                if let Some(pred_snapshot) = self.block_to_snapshot_mapping.get(pred.index()) {
                    predecessors.push(pred_snapshot);
                } else {
                    assert!(block.is_loop() && pred == block.last_predecessor() && !REVISIT_LOOP_HEADER);
                }
            }
            predecessors.reverse();

            let merge_types = |_: Key, predecessors: Vec<Type>| -> Type {
                assert!(!predecessors.is_empty());
                let mut result_type = predecessors[0].clone();
                for i in 1..predecessors.len() {
                    result_type = Type::least_upper_bound(&result_type, &predecessors[i], self.graph_zone);
                }
                result_type
            };

            self.table.start_new_snapshot(predecessors.to_vec(), merge_types);

            if block.predecessor_count() == 1 {
                let predecessor = block.last_predecessor();
                let terminator = predecessor.last_operation(self.graph);
                if let Some(branch) = terminator.try_cast::<BranchOp>() {
                    assert!(branch.if_true() == block || branch.if_false() == block);
                    self.refine_types_after_branch(branch, block, branch.if_true() == block);
                }
            }
            self.set_current_block(Some(block));

            let mut loop_needs_revisit = false;
            let op_range = self.graph.operation_indices(block);
            for index in op_range {
                let op = self.graph.get(index);

                match op.opcode() {
                    Opcode::kBranch |
                    Opcode::kDeoptimize |
                    Opcode::kDeoptimizeIf |
                    Opcode::kFrameState |
                    Opcode::kReturn |
                    Opcode::kStore |
                    Opcode::kRetain |
                    Opcode::kUnreachable |
                    Opcode::kSwitch |
                    Opcode::kTuple |
                    Opcode::kStaticAssert |
                    Opcode::kDebugBreak |
                    Opcode::kDebugPrint |
                    #[cfg(V8_ENABLE_WEBASSEMBLY)]
                    Opcode::kGlobalSet |
                    #[cfg(V8_ENABLE_WEBASSEMBLY)]
                    Opcode::kTrapIf |
                    Opcode::kCheckException => {
                        assert_eq!(0, op.outputs_rep().len());
                    },
                    Opcode::kCheckTurboshaftTypeOf => {
                        self.process_check_turboshaft_type_of(index, op.cast::<CheckTurboshaftTypeOfOp>());
                    },
                    Opcode::kComparison => {
                        self.process_comparison(index, op.cast::<ComparisonOp>());
                    },
                    Opcode::kConstant => {
                        self.process_constant(index, op.cast::<ConstantOp>());
                    },
                    Opcode::kFloatBinop => {
                        self.process_float_binop(index, op.cast::<FloatBinopOp>());
                    },
                    Opcode::kOverflowCheckedBinop => {
                        self.process_overflow_checked_binop(index, op.cast::<OverflowCheckedBinopOp>());
                    },
                    Opcode::kProjection => {
                        self.process_projection(index, op.cast::<ProjectionOp>());
                    },
                    Opcode::kWordBinop => {
                        self.process_word_binop(V::<Word>::new(index), op.cast::<WordBinopOp>());
                    },
                    Opcode::kWord32PairBinop |
                    Opcode::kAtomicWord32Pair |
                    Opcode::kPendingLoopPhi => {
                        panic!("UNREACHABLE");
                    },
                    Opcode::kPhi => {
                        if REVISIT_LOOP_HEADER {
                            loop_needs_revisit = self.process_loop_phi(index, op.cast::<PhiOp>()) || loop_needs_revisit;
                        } else {
                            self.process_phi(index, op.cast::<PhiOp>());
                        }
                    },
                    Opcode::kGoto => {
                        let gto = op.cast::<GotoOp>();

                        if gto.destination().is_loop() {
                            if gto.destination().index() < self.current_block().unwrap().index() {
                                self.process_block::<true>(gto.destination(), unprocessed_index);
                            } else if gto.destination().index() == self.current_block().unwrap().index() {
                                if !REVISIT_LOOP_HEADER || loop_needs_revisit {
                                    self.process_block::<true>(gto.destination(), unprocessed_index);
                                }
                            }
                        }
                    },
                    _ => {
                        if op.outputs_rep().len() > 0 {
                            const ALLOW_NARROWING: bool = false;
                            const IS_FALLBACK_FOR_UNSUPPORTED_OPERATION: bool = true;
                            self.set_type(index, Typer::type_for_representation(op.outputs_rep(), self.graph_zone), ALLOW_NARROWING, IS_FALLBACK_FOR_UNSUPPORTED_OPERATION);
                        }
                    },
                    Opcode::kLoadRootRegister => {
                        self.set_type(index, Typer::type_for_representation(op.outputs_rep(), self.graph_zone));
                    },
                }
            }

            if REVISIT_LOOP_HEADER {
                if loop_needs_revisit {
                    *unprocessed_index = std::cmp::min(*unprocessed_index, block.index().id() + 1);
                }
            }
        }

        fn process_check_turboshaft_type_of(&self, index: OpIndex, check: &CheckTurboshaftTypeOfOp) {
            let input_type = self.get_type(check.input());

            if input_type.is_subtype_of(check.type()) {
                if cfg!(debug_assertions) && V8_FLAGS.turboshaft_trace_typing {
                    println!(
                        "CTOF {:3}:{:40}\n  P: {:3}:{:40} ~~> {}",
                        index.id(),
                        self.graph.get(index).to_string().chars().take(40).collect::<String>(),
                        check.input().id(),
                        self.graph.get(check.input()).to_string().chars().take(40).collect::<String>(),
                        input_type.to_string()
                    );
                }
            } else if check.successful() {
                panic!(
                    "Checking type {} of operation {}:{} failed after it passed in a previous phase",
                    check.type().to_string(),
                    check.input().id(),
                    self.graph.get(check.input()).to_string()
                );
            } else {
                if cfg!(debug_assertions) && V8_FLAGS.turboshaft_trace_typing {
                    println!(
                        "CTOF {:3}:{:40}\n  F: {:3}:{:40} ~~> {}",
                        index.id(),
                        self.graph.get(index).to_string().chars().take(40).collect::<String>(),
                        check.input().id(),
                        self.graph.get(check.input()).to_string().chars().take(40).collect::<String>(),
                        input_type.to_string()
                    );
                }
            }
        }

        fn process_comparison(&self, index: OpIndex, comparison: &ComparisonOp) {
            let left_type = self.get_type(comparison.left());
            let right_type = self.get_type(comparison.right());

            let result_type = Typer::type_comparison(
                &left_type,
                &right_type,
                comparison.rep(),
                comparison.kind(),
                self.graph_zone,
            );
            self.set_type(index, result_type);
        }

        fn process_constant(&self, index: OpIndex, constant: &ConstantOp) {
            if constant.kind() == ConstantOpKind::kFloat64 && constant.float64().is_hole_nan() {
                self.set_type(index, Type::Any(), false, false);
                return;
            }
            let type_ = Typer::type_constant(constant.kind(), constant.storage());
            self.set_type(index, type_);
        }

        fn process_float_binop(&self, index: OpIndex, binop: &FloatBinopOp) {
            let left_type = self.get_type(binop.left());
            let right_type = self.get_type(binop.right());

            let result_type = Typer::type_float_binop(
                &left_type,
                &right_type,
                binop.kind(),
                binop.rep(),
                self.graph_zone,
            );
            self.set_type(index, result_type);
        }

        fn process_loop_phi(&self, index: OpIndex, phi: &PhiOp) -> bool {
            let old_type = self.get_type_at_definition(index);
            let new_type = self.compute_type_for_phi(phi);

            if old_type.is_invalid() {
                self.set_type(index, new_type, false, false);
                return true;
            }

            if new_type.is_subtype_of(&old_type) {
                if cfg!(debug_assertions) && V8_FLAGS.turboshaft_trace_typing {
                    println!(
                        "LOOP {:3}:{:40} (FIXPOINT)\n  N:     {:40} ~~> {:40}",
                        index.id(),
                        self.graph.get(index).to_string().chars().take(40).collect::<String>(),
                        old_type.to_string(),
                        new_type.to_string()
                    );
                }

                const ALLOW_NARROWING: bool = true;
                self.set_type(index, new_type, ALLOW_NARROWING, false);
                return false;
            }

            if cfg!(debug_assertions) && V8_FLAGS.turboshaft_trace_typing {
                println!(
                    "LOOP {:3}:{:40} (REVISIT)\n  W:     {:40} ~~> {:40}",
                    index.id(),
                    self.graph.get(index).to_string().chars().take(40).collect::<String>(),
                    old_type.to_string(),
                    new_type.to_string()
                );
            }

            let mut new_type = new_type.clone();
            if !old_type.is_none() {
                new_type = self.widen(&old_type, &new_type);
            }
            self.set_type(index, new_type, false, false);
            true
        }

        fn process_overflow_checked_binop(&self, index: OpIndex, binop: &OverflowCheckedBinopOp) {
            let left_type = self.get_type(binop.left());
            let right_type = self.get_type(binop.right());

            let result_type = Typer::type_overflow_checked_binop(
                &left_type,
                &right_type,
                binop.kind(),
                binop.rep(),
                self.graph_zone,
            );
            self.set_type(index, result_type);
        }

        fn process_phi(&self, index: OpIndex, phi: &PhiOp) {
            let result_type = self.compute_type_for_phi(phi);
            self.set_type(index, result_type);
        }

        fn process_projection(&self, index: OpIndex, projection: &ProjectionOp) {
            let input_type = self.get_type(projection.input());

            let result_type: Type;
            if input_type.is_none() {
                result_type = Type::None();
            } else if input_type.is_tuple() {
                let tuple = input_type.as_tuple();
                assert!(projection.index() < tuple.size());
                result_type = tuple.element(projection.index()).clone();
                assert!(result_type.is_subtype_of(&Typer::type_for_representation(projection.rep(), self.graph_zone)));
            } else {
                result_type = Typer::type_for_representation(projection.rep(), self.graph_zone);
            }

            self.set_type(index, result_type);
        }

        fn process_word_binop(&self, index: V<Word>, binop: &WordBinopOp) {
            let left_type = self.get_type(binop.left());
            let right_type = self.get_type(binop.right());

            let result_type = Typer::type_word_binop(
                &left_type,
                &right_type,
                binop.kind(),
                binop.rep(),
                self.graph_zone,
            );
            self.set_type(index.into(), result_type);
        }

        fn compute_type_for_phi(&self, phi: &PhiOp) -> Type {
            let maybe_truncate = |t: Type| -> Type {
                if t.is_none() {
                    return t;
                }
                if phi.rep() == RegisterRepresentation::Word32() {
                    return Typer::truncate_word32_input(&t, true, self.graph_zone);
                }
                t
            };

            let mut result_type = maybe_truncate(self.get_type_or_default(phi.inputs()[0], Type::None()));
            for i in 1..phi.inputs().len() {
                let input_type = maybe_truncate(self.get_type_or_default(phi.inputs()[i], Type::None()));
                result_type = Type::least_upper_bound(&result_type, &input_type, self.graph_zone);
            }
            result_type
        }

        fn refine_types_after_branch(&self, branch: &BranchOp, new_block: &Block, then_branch: bool) {
            if cfg!(debug_assertions) && V8_FLAGS.turboshaft_trace_typing {
                println!(
                    "Br   {:3}:{:40}",
                    self.graph.index(branch).id(),
                    branch.to_string().chars().take(40).collect::<String>()
                );
            }

            let mut refinements = Typer::BranchRefinements::new(
                |index: OpIndex| -> Type { self.get_type(index) },
                |index: OpIndex, refined_type: &Type| {
                    self.refine_operation_type(new_block, index, refined_type, if then_branch { 'T' } else { 'F' });
                },
            );

            let condition = self.graph.get(branch.condition());
            refinements.refine_types(&condition, then_branch, self.graph_zone);
        }

        fn refine_operation_type(&self, new_block: &Block, op: OpIndex, type_: &Type, case_for_tracing: char) {
            assert!(op.is_valid());
            assert!(!type_.is_invalid());

            if cfg!(debug_assertions) && V8_FLAGS.turboshaft_trace_typing {
                println!(
                    "  {}: {:3}:{:40} ~~> {}",
                    case_for_tracing,
                    op.id(),
                    self.graph.get(op).to_string().chars().take(40).collect::<String>(),
                    type_.to_string()
                );
            }

            if let Some(key_opt) = self.op_to_key_mapping.get(op) {
                self.table.set(key_opt, type_.clone());
            } else {
                panic!("Key not found for OpIndex: {:?}", op);
            }

            #[cfg(debug_assertions)]
            {
                if let Some(ref block_refinements) = *self.block_refinements.borrow() {
                  block_refinements.get(new_block.index()).push((op, type_.clone()));
                }
            }
        }

        fn set_type(&self, index: OpIndex, result_type: Type, allow_narrowing: bool, is_fallback_for_unsupported_operation: bool) {
            assert!(!result_type.is_invalid());

            let key_opt = self.op_to_key_mapping.get(index);
            match key_opt {
                Some(key) => {
                    self.table.set(key, result_type.clone());
                    self.types.set(index, result_type.clone());
                },
                None => {
                    let key = self.table.new_key(Type::None());
                    self.op_to_key_mapping.set(index, Some(key));
                    self.table.set(key, result_type.clone());
                    self.types.set(index, result_type.clone());
                }
            }

            if !is_fallback_for_unsupported_operation {
                if cfg!(debug_assertions) && V8_FLAGS.turboshaft_trace_typing {
                    println!(
                        "Type {:3}:{:40} ==> {}",
                        index.id(),
                        self.graph.get(index).to_string().chars().take(40).collect::<String>(),
                        result_type.to_string()
                    );
                }
            } else {
                if cfg!(debug_assertions) && V8_FLAGS.turboshaft_trace_typing {
                    println!(
                        "TODO {:3}:{:40} ==> {}",
                        index.id(),
                        self.graph.get(index).to_string().chars().take(40).collect::<String>(),
                        result_type.to_string()
                    );
                }
            }
        }

        fn get_type_or_invalid(&self, index: OpIndex) -> Type {
            self.op_to_key_mapping.get(index).map_or(Type::Invalid(), |key| self.table.get(key).clone())
        }

        fn get_type_or_default(&self, index: OpIndex, default_type: Type) -> Type {
            let t = self.get_type_or_invalid(index);
            if t.is_invalid() {
                return default_type;
            }
            t
        }

        fn get_type(&self, index: OpIndex) -> Type {
            let t = self.get_type_or_invalid(index);
            if t.is_invalid() {
                let op = self.graph.get(index);
                return Typer::type_for_representation(op.outputs_rep(), self.graph_zone);
            }
            t
        }

        fn get_type_at_definition(&self, index: OpIndex) -> Type {
            self.types.get(index).clone()
        }

        fn widen(&self, old_type: &Type, new_type: &Type) -> Type {
            if new_type.is_any() {
                return new_type.clone();
            }
            assert_eq!(old_type.kind(), new_type.kind());

            match old_type.kind() {
                TypeKind::kWord32 => {
                    WordOperationTyper::<32>::widen_maximal(old_type.as_word32(), new_type.as_word32(), self.graph_zone)
                }
                TypeKind::kWord64 => {
                    WordOperationTyper::<64>::widen_maximal(old_type.as_word64(), new_type.as_word64(), self.graph_zone)
                }
                TypeKind::kFloat32 => {
                    Float32Type::any()
                }
                TypeKind::kFloat64 => {
                    Float64Type::any()
                }
                _ => {
                    panic!("UNREACHABLE");
                }
            }
        }
    
        //Helper functions for accessing private fields
        fn current_block(&self) -> Option<&Block> {
            self.table.current_block()
        }

        fn set_current_block(&self, block: Option<&Block>) {
            self.table.set_current_block(block)
        }

    }
}