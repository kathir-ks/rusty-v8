// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod copying_phase {
    use std::cell::RefCell;
    use std::collections::HashSet;
    use std::fmt;
    use std::fmt::Display;
    use std::marker::PhantomData;
    use std::ops::Deref;
    use std::rc::Rc;

    use crate::base::{
        iterator::Reversed,
        logging::DCHECK,
        small_vector::SmallVector,
        vector::{base, Vector},
    };
    use crate::codegen::{
        optimized_compilation_info::OptimizedCompilationInfo,
        source_position::SourcePosition,
    };
    use crate::compiler::{
        node_origin_table::NodeOriginTable,
        turboshaft::{
            assembler::Assembler,
            graph::Graph,
            index::OpIndex,
            operations::{
                BlockTerminator, BranchOp, CallOp, CallTarget, CheckExceptionOp, DidntThrowOp,
                FrameStateOp, GotoOp, MayThrow, Operation, OperationPrintStyle, Opcode,
                ParameterOp, PendingLoopPhiOp, PhiOp, SwitchOp, TupleOp,
            },
            phase::Phase,
            reducer_traits::CanBeUsedAsInput,
            representations::{
                MaybeRegisterRepresentation, MaybeVariable, RegisterRepresentation, V,
            },
            snapshot_table::SnapshotTable,
            variable_reducer::VariableReducer,
        },
    };
    use crate::zone::zone_containers::{FixedBlockSidetable, FixedOpIndexSidetable, ZoneVector};

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct PaddingSpace {
        pub spaces: i32,
    }

    impl Display for PaddingSpace {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for _ in 0..self.spaces {
                write!(f, " ")?;
            }
            Ok(())
        }
    }

    pub fn count_decimal_digits(value: u32) -> i32 {
        if value < 10 {
            1
        } else if value < 100 {
            2
        } else if value < 1000 {
            3
        } else if value < 10000 {
            4
        } else if value < 100000 {
            5
        } else if value < 1000000 {
            6
        } else if value < 10000000 {
            7
        } else if value < 100000000 {
            8
        } else if value < 1000000000 {
            9
        } else {
            10
        }
    }

    pub trait ReducerList {}

    pub struct OutputGraphAssembler<Derived, Base> {
        base: Base,
        derived_marker: PhantomData<Derived>,
    }

    impl<Derived, Base> OutputGraphAssembler<Derived, Base>
    where
        Derived: OutputGraphAssemblerTrait<Base = Base>,
        Base: AssemblerTrait,
    {
        pub fn new(base: Base) -> Self {
            OutputGraphAssembler {
                base,
                derived_marker: PhantomData,
            }
        }

        fn derived_this(&self) -> &Derived {
            unsafe { &*(self as *const Self as *const Derived) }
        }

        fn assembler(&self) -> &Assembler<<Base as AssemblerTrait>::ReducerList> {
            self.derived_this().asm()
        }

        fn map(&self, index: OpIndex) -> OpIndex {
            self.derived_this().map_to_new_graph(index)
        }

        fn map_optional(&self, index: Option<OpIndex>) -> Option<OpIndex> {
            self.derived_this().map_to_new_graph_optional(index)
        }

        fn map_vector<const N: usize>(
            &self,
            indices: Vector<OpIndex>,
        ) -> SmallVector<OpIndex, N> {
            self.derived_this().map_to_new_graph_vector::<N>(indices)
        }

        // The following functions are auto-generated from the
        // `TURBOSHAFT_OPERATION_LIST` macro in C++.  Since we don't
        // have direct access to the macro, we'll need to manually
        // implement these functions based on the list of operations.

        // Example:
        // OpIndex AssembleOutputGraphGoto(const GotoOp& op) {
        //   return op.Explode(
        //       [a = assembler()](auto... args) {
        //         return a->ReduceGoto(args...);
        //       },
        //       *this);
        // }

        pub fn assemble_output_graph_goto(&self, op: &GotoOp) -> OpIndex {
            let assembler = self.assembler();
            op.explode(|destination, is_backedge| {
                assembler.reduce_goto(destination, is_backedge)
            }, self.derived_this())
        }

        pub fn assemble_output_graph_branch(&self, op: &BranchOp) -> OpIndex {
            let assembler = self.assembler();
            op.explode(
                |condition, if_true, if_false, hint| {
                    assembler.reduce_branch(condition, if_true, if_false, hint)
                },
                self.derived_this(),
            )
        }

        pub fn assemble_output_graph_switch(&self, op: &SwitchOp) -> OpIndex {
            let assembler = self.assembler();
            op.explode(
                |input, cases, default_case, default_hint| {
                    assembler.reduce_switch(input, cases, default_case, default_hint)
                },
                self.derived_this(),
            )
        }

        pub fn assemble_output_graph_phi(&self, op: &PhiOp) -> OpIndex {
            let assembler = self.assembler();
            op.explode(|inputs, rep| assembler.reduce_phi(inputs, rep), self.derived_this())
        }

        pub fn assemble_output_graph_pending_loop_phi(&self, op: &PendingLoopPhiOp) -> OpIndex {
            //  UNREACHABLE();
            panic!("UNREACHABLE");
        }

        pub fn assemble_output_graph_frame_state(&self, op: &FrameStateOp) -> OpIndex {
            let assembler = self.assembler();
            op.explode(|inputs, inlined, data| {
                assembler.reduce_frame_state(inputs, inlined, data)
            }, self.derived_this())
        }

        pub fn assemble_output_graph_call(&self, op: &CallOp) -> OpIndex {
            let assembler = self.assembler();
            op.explode(
                |callee, frame_state, arguments, descriptor, effects| {
                    assembler.reduce_call(callee, frame_state, arguments, descriptor, effects)
                },
                self.derived_this(),
            )
        }

        pub fn assemble_output_graph_didnt_throw(&self, op: &DidntThrowOp) -> OpIndex {
            let assembler = self.assembler();
            op.explode(|throwing_operation| {
                //  assembler.reduce_input_graph_name(throwing_operation)
                assembler.reduce_didnt_throw(throwing_operation)
            }, self.derived_this())
        }

        pub fn assemble_output_graph_check_exception(&self, op: &CheckExceptionOp) -> V<None> {
            // Graph::OpIndexIterator it(op.didnt_throw_block->begin(),
            //                               &Asm().input_graph());
            //   Graph::OpIndexIterator end(op.didnt_throw_block->end(),
            //                                &Asm().input_graph());
            //     // To translate `CheckException` to the new graph, we reduce the throwing
            //     // operation (actually it's `DidntThrow` operation, but that triggers the
            //     // actual reduction) with a catch scope. If the reduction replaces the
            //     // throwing operation with other throwing operations, all of them will be
            //     // connected to the provided catch block. The reduction should automatically
            //     // bind a block that represents non-throwing control flow of the original
            //     // operation, so we can inline the rest of the `didnt_throw` block.
            //     {
            //       CatchScope scope(Asm(), MapToNewGraph(op.catch_block));
            //       DCHECK(Asm().input_graph().Get(*it).template Is<DidntThrowOp>());
            //       if (!Asm().InlineOp(*it, op.didnt_throw_block)) {
            //         return V<None>::Invalid();
            //       }
            //       ++it;
            //     }
            //     for (; it != end; ++it) {
            //       // Using `InlineOp` requires that the inlined operation is not emitted
            //       // multiple times. This is the case here because we just removed the
            //       // single predecessor of `didnt_throw_block`.
            //       if (!Asm().InlineOp(*it, op.didnt_throw_block)) {
            //         break;
            //       }
            //     }
            //     return V<None>::Invalid();
            todo!()
        }

        pub fn assemble_output_graph_parameter(&self, param: &ParameterOp) -> OpIndex {
            let assembler = self.assembler();
            // Calling the AssemblerOpInterface rather than the first Reduce method
            // in order to make use of the Parameter cache.
            assembler.parameter(param.parameter_index, param.rep, param.debug_name)
        }
    }

    trait OutputGraphAssemblerTrait {
        type Base: AssemblerTrait;
        fn asm(&self) -> &Assembler<<Self::Base as AssemblerTrait>::ReducerList>;
        fn map_to_new_graph(&self, index: OpIndex) -> OpIndex;
        fn map_to_new_graph_optional(&self, index: Option<OpIndex>) -> Option<OpIndex>;
        fn map_to_new_graph_vector<const N: usize>(
            &self,
            indices: Vector<OpIndex>,
        ) -> SmallVector<OpIndex, N>;
    }

    trait AssemblerTrait {
        type ReducerList: ReducerList;
        fn reduce_goto(&self, destination: OpIndex, is_backedge: bool) -> OpIndex;
        fn reduce_branch(
            &self,
            condition: OpIndex,
            if_true: OpIndex,
            if_false: OpIndex,
            hint: BranchOp::Hint,
        ) -> OpIndex;
        fn reduce_switch(
            &self,
            input: OpIndex,
            cases: Vector<SwitchOp::Case>,
            default_case: OpIndex,
            default_hint: SwitchOp::DefaultHint,
        ) -> OpIndex;
        fn reduce_phi(&self, inputs: Vector<OpIndex>, rep: RegisterRepresentation) -> OpIndex;
        fn reduce_frame_state(
            &self,
            inputs: Vector<OpIndex>,
            inlined: bool,
            data: FrameStateOp::Data,
        ) -> OpIndex;
        fn reduce_call(
            &self,
            callee: V<CallTarget>,
            frame_state: Option<V<FrameStateOp>>,
            arguments: Vector<OpIndex>,
            descriptor: CallOp::Descriptor,
            effects: CallOp::Effects,
        ) -> OpIndex;
        fn reduce_didnt_throw(&self, throwing_operation: OpIndex) -> OpIndex;
        fn parameter(
            &self,
            parameter_index: i32,
            rep: RegisterRepresentation,
            debug_name: String,
        ) -> OpIndex;
    }

    impl<R: ReducerList> AssemblerTrait for Assembler<R> {
        type ReducerList = R;

        fn reduce_goto(&self, destination: OpIndex, is_backedge: bool) -> OpIndex {
            self.reduce_goto(destination, is_backedge)
        }

        fn reduce_branch(
            &self,
            condition: OpIndex,
            if_true: OpIndex,
            if_false: OpIndex,
            hint: BranchOp::Hint,
        ) -> OpIndex {
            self.reduce_branch(condition, if_true, if_false, hint)
        }

        fn reduce_switch(
            &self,
            input: OpIndex,
            cases: Vector<SwitchOp::Case>,
            default_case: OpIndex,
            default_hint: SwitchOp::DefaultHint,
        ) -> OpIndex {
            self.reduce_switch(input, cases, default_case, default_hint)
        }

        fn reduce_phi(&self, inputs: Vector<OpIndex>, rep: RegisterRepresentation) -> OpIndex {
            self.reduce_phi(inputs, rep)
        }

        fn reduce_frame_state(
            &self,
            inputs: Vector<OpIndex>,
            inlined: bool,
            data: FrameStateOp::Data,
        ) -> OpIndex {
            self.reduce_frame_state(inputs, inlined, data)
        }

        fn reduce_call(
            &self,
            callee: V<CallTarget>,
            frame_state: Option<V<FrameStateOp>>,
            arguments: Vector<OpIndex>,
            descriptor: CallOp::Descriptor,
            effects: CallOp::Effects,
        ) -> OpIndex {
            self.reduce_call(callee, frame_state, arguments, descriptor, effects)
        }

        fn reduce_didnt_throw(&self, throwing_operation: OpIndex) -> OpIndex {
            self.reduce_didnt_throw(throwing_operation)
        }

        fn parameter(
            &self,
            parameter_index: i32,
            rep: RegisterRepresentation,
            debug_name: String,
        ) -> OpIndex {
            self.parameter(parameter_index, rep, debug_name)
        }
    }

    pub struct GraphVisitor<AfterNext> {
        next: VariableReducer<AfterNext>,
        input_graph_: Rc<RefCell<Graph>>,
        current_input_block_: Option<*const crate::compiler::turboshaft::graph::Block>, // *const Block,
        op_mapping_: FixedOpIndexSidetable<OpIndex>,
        block_mapping_: FixedBlockSidetable<*mut crate::compiler::turboshaft::graph::Block>, // FixedBlockSidetable<Block*>,
        blocks_needing_variables_: BitVector,
        old_opindex_to_variables: FixedOpIndexSidetable<MaybeVariable>,
        blocks_to_clone_: ZoneVector<BlockToClone>,
        turn_loop_without_backedge_into_merge_: bool,
        current_block_needs_variables_: bool,
        // info_: Option<*mut OptimizedCompilationInfo>, // OptimizedCompilationInfo*,
        _tick_counter: (), // TickCounter*,
        block_to_inline_now_: Option<*mut crate::compiler::turboshaft::graph::Block>, // Block*,
        is_in_recursive_inlining_: bool,

        _marker: PhantomData<AfterNext>,
    }

    impl<AfterNext> OutputGraphAssemblerTrait for GraphVisitor<AfterNext> {
        type Base = VariableReducer<AfterNext>;

        fn asm(&self) -> &Assembler<<Self::Base as AssemblerTrait>::ReducerList> {
            &self.next.asm()
        }

        fn map_to_new_graph(&self, index: OpIndex) -> OpIndex {
            self.map_to_new_graph_internal(index, -1)
        }

        fn map_to_new_graph_optional(&self, index: Option<OpIndex>) -> Option<OpIndex> {
            match index {
                Some(idx) => Some(self.map_to_new_graph_internal(idx, -1)),
                None => None,
            }
        }

        fn map_to_new_graph_vector<const N: usize>(
            &self,
            indices: Vector<OpIndex>,
        ) -> SmallVector<OpIndex, N> {
            let mut result = SmallVector::<OpIndex, N>::new();
            for input in indices.iter() {
                result.push(self.map_to_new_graph(*input));
            }
            result
        }
    }

    impl<AfterNext> GraphVisitor<AfterNext> {
        pub fn new(asm: Assembler<AfterNext>) -> Self {
            // let input_graph_ = asm.modifiable_input_graph();
            //  let current_input_block_ = None;
            // let op_mapping_ = FixedOpIndexSidetable::new(asm.input_graph().op_id_count(), OpIndex::Invalid(), asm.phase_zone(), &asm.input_graph());
            //  let block_mapping_ = FixedBlockSidetable::new(asm.input_graph().block_count(), None, asm.phase_zone());
            // let blocks_needing_variables_ = BitVector::new(asm.input_graph().block_count(), asm.phase_zone());
            // let old_opindex_to_variables = FixedOpIndexSidetable::new(asm.input_graph().op_id_count(), asm.phase_zone(), &asm.input_graph());
            // let blocks_to_clone_ = ZoneVector::new(asm.phase_zone());
            //  Asm().output_graph().Reset();

            //let info_ = asm.data().info();
            // let tick_counter_ = info_ ? &info_->tick_counter() : nullptr;
            let input_graph_rc = Rc::clone(&asm.input_graph());

            GraphVisitor {
                next: VariableReducer::new(asm),
                input_graph_: input_graph_rc, //  asm.modifiable_input_graph().clone(),
                current_input_block_: None,
                op_mapping_: FixedOpIndexSidetable::new(
                    0, // Replace with actual op_id_count from graph
                    OpIndex::invalid(),
                ), // Replace with actual zone and graph
                block_mapping_: FixedBlockSidetable::new(0, std::ptr::null_mut()), // Replace with actual block_count and zone
                blocks_needing_variables_: BitVector::new(0), // Replace with actual block_count and zone
                old_opindex_to_variables: FixedOpIndexSidetable::new(0, None), // Replace with actual zone and graph
                blocks_to_clone_: ZoneVector::new(),       // Replace with actual zone
                turn_loop_without_backedge_into_merge_: true,
                current_block_needs_variables_: false,
                //  info_: None,
                _tick_counter: (),
                block_to_inline_now_: None,
                is_in_recursive_inlining_: false,
                _marker: PhantomData,
            }
        }

        //TURBOSHAFT_REDUCER_BOILERPLATE(CopyingPhase)

        // `trace_reduction` is a template parameter to avoid paying for tracing at
        // runtime.
        pub fn visit_graph<const TRACE_REDUCTION: bool>(&mut self) {
            self.next.asm().analyze();

            // Creating initial old-to-new Block mapping.
            let blocks = self.next.asm().modifiable_input_graph().borrow().blocks().clone(); //TODO remove clone
            for input_block in blocks {
                self.block_mapping_.add(self.next.asm().output_graph().new_block(
                    if input_block.is_loop() {
                        crate::compiler::turboshaft::graph::Block::Kind::LoopHeader
                    } else {
                        crate::compiler::turboshaft::graph::Block::Kind::Merge
                    },
                    Some(&input_block),
                ));
            }

            // Visiting the graph.
            self.visit_all_blocks::<TRACE_REDUCTION>();

            self.finalize();
        }

        pub fn bind(&mut self, block: *mut crate::compiler::turboshaft::graph::Block) {
            //  Next::Bind(block);
            self.next.bind(block);
            unsafe {
                if let Some(current_input_block) = self.current_input_block_ {
                    (*block).set_origin(current_input_block);
                }
            }
        }

        pub fn finalize(&mut self) {
            // Updating the source_positions.
            //TODO Fix
            /*
            if !self.Asm().input_graph().source_positions().empty() {
                for index in self.Asm().output_graph().AllOperationIndices() {
                    let origin = self.Asm().output_graph().operation_origins()[index];
                    self.Asm().output_graph().source_positions()[index] = origin.valid() ? self.Asm().input_graph().source_positions()[origin] : SourcePosition::Unknown();
                }
            }
            // Updating the operation origins.
            let origins = self.Asm().data().node_origins();
            if origins {
                for index in self.Asm().output_graph().AllOperationIndices() {
                    let origin = self.Asm().output_graph().operation_origins()[index];
                    if origin.valid() {
                        origins.SetNodeOrigin(index.id(), origin.id());
                    }
                }
            }

            self.input_graph_.SwapWithCompanion();
            */
            todo!()
        }

        pub fn current_input_block(&self) -> Option<*const crate::compiler::turboshaft::graph::Block> {
            self.current_input_block_
        }

        pub fn turn_loop_without_backedge_into_merge(&mut self) -> &mut bool {
            &mut self.turn_loop_without_backedge_into_merge_
        }

        // Emits a Goto to a cloned version of {input_block}, assuming that the only
        // predecessor of this cloned copy will be the current block. {input_block} is
        // not cloned right away (because this would recursively call VisitBlockBody,
        // which could cause stack overflows), and is instead added to the
        // {blocks_to_clone_} stack, whose blocks will be cloned once the current
        // block has been fully visited.
        pub fn clone_block_and_goto(&mut self, input_block: *const crate::compiler::turboshaft::graph::Block) {
            //  let new_block = self.Asm().output_graph().NewBlock(unsafe { (*input_block).kind() }, input_block);

            // Computing which input of Phi operations to use when visiting
            // {input_block} (since {input_block} doesn't really have predecessors
            // anymore).
            // TODO fix
            //  int added_block_phi_input = input_block->GetPredecessorIndex(
            //      Asm().current_block()->OriginForBlockEnd());

            // There is no guarantees that {input_block} will be entirely removed just
            // because it's cloned/inlined, since it's possible that it has predecessors
            // for which this optimization didn't apply. As a result, we add it to
            // {blocks_needing_variables_}, so that if it's ever generated
            // normally, Variables are used when emitting its content, so that
            // they can later be merged when control flow merges with the current
            // version of {input_block} that we just cloned.
            unsafe {
                if let Some(block) = unsafe {
                    self.next
                        .asm()
                        .current_block()
                } {
                    if let Some(origin) = (*block).origin_for_block_end() {
                        //  let added_block_phi_input = (*input_block).get_predecessor_index(origin);
                        self.blocks_needing_variables_.add((*input_block).index().id() as usize);
                        //  self.Asm().Goto(new_block);
                        //  self.blocks_to_clone_.push_back({input_block, added_block_phi_input, new_block});
                        todo!()
                    }
                }
            }
            todo!()
        }

        // Visits and emits {input_block} right now (ie, in the current block). This
        // should not be called recursively in order to avoid stack overflow (ie,
        // processing {input_block} should never lead to calling CloneAndInlingBlock).
        pub fn clone_and_inline_block(&mut self, input_block: *const crate::compiler::turboshaft::graph::Block) {
            if self.next.asm().generating_unreachable_operations() {
                return;
            }

            // Making sure that we didn't call CloneAndInlineBlock recursively.
            DCHECK(!self.is_in_recursive_inlining_);
            //  ScopedModification<bool> recursive_guard(&is_in_recursive_inlining_, true);

            // Computing which input of Phi operations to use when visiting
            // {input_block} (since {input_block} doesn't really have predecessors
            // anymore).
            // TODO fix
            //  int added_block_phi_input = input_block->GetPredecessorIndex(
            //      Asm().current_block()->OriginForBlockEnd());

            // There is no guarantees that {input_block} will be entirely removed just
            // because it's cloned/inlined, since it's possible that it has predecessors
            // for which this optimization didn't apply. As a result, we add it to
            // {blocks_needing_variables_}, so that if it's ever generated
            // normally, Variables are used when emitting its content, so that
            // they can later be merged when control flow merges with the current
            // version of {input_block} that we just cloned.
            unsafe {
                if let Some(block) = unsafe {
                    self.next
                        .asm()
                        .current_block()
                } {
                    if let Some(origin) = (*block).origin_for_block_end() {
                        //let added_block_phi_input = (*input_block).get_predecessor_index(origin);
                        self.blocks_needing_variables_.add((*input_block).index().id() as usize);
                        //  ScopedModification<bool> set_true(&current_block_needs_variables_, true);
                        //self.VisitBlockBody::<CanHavePhis::kYes, ForCloning::kYes, false>(input_block, added_block_phi_input);
                        todo!()
                    }
                }
            }
            todo!()
        }

        // {InlineOp} introduces two limitations unlike {CloneAndInlineBlock}:
        // 1. The input operation must not be emitted anymore as part of its
        // regular input block;
        // 2. {InlineOp} must not be used multiple times for the same input op.
        pub fn inline_op(&mut self, index: OpIndex, input_block: *const crate::compiler::turboshaft::graph::Block) -> bool {
            self.visit_op_and_update_mapping::<false>(index, input_block)
        }

        fn map_to_new_graph_internal(&self, old_index: OpIndex, predecessor_index: i32) -> OpIndex {
            DCHECK(old_index.is_valid());
            //  OpIndex result = self.op_mapping_[old_index];
            let result = self.op_mapping_.get(old_index.id() as usize);

            match result {
                Some(result) => *result,
                None => {
                    // {op_mapping} doesn't have a mapping for {old_index}. The
                    // VariableReducer should provide the mapping.
                    let var = self.get_variable_for(old_index);
                    match var {
                        Some(var) => {
                            if predecessor_index == -1 {
                                self.next.asm().get_variable(var)
                            } else {
                                self.next.asm().get_predecessor_value(var, predecessor_index)
                            }
                        }
                        None => {
                            // if constexpr (can_be_invalid) {
                            //  return OpIndex::Invalid();
                            // }
                            panic!("DCHECK(var.has_value())");
                            //  OpIndex::Invalid()
                        }
                    }
                }
            }

            // DCHECK_IMPLIES(!can_be_invalid, result.valid());
            //  result
        }

        //  template <bool can_be_invalid = false, typename T>
        //  V<T> MapToNewGraph(V<T> old_index, int predecessor_index = -1) {
        //    return V<T>::Cast(MapToNewGraph<can_be_invalid>(
        //        static_cast<OpIndex>(old_index), predecessor_index));
        //  }

        //  Block* MapToNewGraph(const Block* block) const {
        //    Block* new_block = block_mapping_[block->index()];
        //    DCHECK_NOT_NULL(new_block);
        //    return new_block;
        //  }

        // template <typename FunctionType>
        // OpIndex ResolvePhi(const PhiOp& op, FunctionType&& map,
        //                   RegisterRepresentation rep) {
        fn resolve_phi<F>(&self, op: &PhiOp, map: F, rep: RegisterRepresentation) -> OpIndex
        where
            F: Fn(OpIndex, i32, i32) -> OpIndex,
        {
            if op.input_count == 1 {
                // If, in the previous CopyingPhase, a loop header was turned into a
                // regular blocks, its PendingLoopPhis became Phis with a single input. We
                // can now just get rid of these Phis.
                return map(op.input(0), -1, 0);
            }

            //TODO FIX
            /*  OpIndex ig_index = self.Asm().input_graph().Index(op);
            if (self.Asm().current_block().IsLoop()) {
                DCHECK_EQ(op.input_count, 2);
                OpIndex og_index = map(op.input(0), -1);
                if (ig_index == op.input(PhiOp::kLoopPhiBackEdgeIndex)) {
                    // Avoid emitting a Loop Phi which points to itself, instead
                    // emit it's 0'th input.
                    return og_index;
                }
                return self.Asm().PendingLoopPhi(og_index, rep);
            }

            base::Vector<OpIndex> old_inputs = op.inputs();
            base::SmallVector<OpIndex, 64> new_inputs;
            int predecessor_count = self.Asm().current_block().PredecessorCount();
            Block* old_pred = self.current_input_block_->LastPredecessor();
            Block* new_pred = self.Asm().current_block().LastPredecessor();
            // Control predecessors might be missing after the optimization phase. So we
            // need to skip phi inputs that belong to control predecessors that have no
            // equivalent in the new graph.

            // We first assume that the order if the predecessors of the current block
            // did not change. If it did, {new_pred} won't be nullptr at the end of this
            // loop, and we'll instead fall back to the slower code below to compute the
            // inputs of the Phi.
            int predecessor_index = predecessor_count - 1;
            int old_index = static_cast<int>(old_inputs.size()) - 1;
            for (OpIndex input : base::Reversed(old_inputs)) {
                if (new_pred && new_pred->OriginForBlockEnd() == old_pred) {
                    // Phis inputs have to come from predecessors. We thus have to
                    // MapToNewGraph with {predecessor_index} so that we get an OpIndex that
                    // is from a predecessor rather than one that comes from a Variable
                    // merged in the current block.
                    new_inputs.push_back(map(input, predecessor_index, old_index));
                    new_pred = new_pred->NeighboringPredecessor();
                    predecessor_index--;
                }
                old_pred = old_pred->NeighboringPredecessor();
                old_index--;
            }
            DCHECK_IMPLIES(new_pred == nullptr, old_pred == nullptr);

            if (new_pred != nullptr) {
                // If {new_pred} is not nullptr, then the order of the predecessors
                // changed. This should only happen with blocks that were introduced in
                // the previous graph. For instance, consider this (partial) dominator
                // tree:
                //
                //     ╠ 7
                //     ║ ╠ 8
                //     ║ ╚ 10
                //     ╠ 9
                //     ╚ 11
                //
                // Where the predecessors of block 11 are blocks 9 and 10 (in that order).
                // In dominator visit order, block 10 will be visited before block 9.
                // Since blocks are added to predecessors when the predecessors are
                // visited, it means that in the new graph, the predecessors of block 11
                // are [10, 9] rather than [9, 10].
                // To account for this, we reorder the inputs of the Phi, and get rid of
                // inputs from blocks that vanished.

                // DEBUG
                // To check that indices are set properly, we zap them in debug builds.
                //for (auto& block : self.Asm().modifiable_input_graph().blocks()) {
                //    block.clear_custom_data();
                //}
                //
                //uint32_t pos = current_input_block_->PredecessorCount() - 1;
                //for (old_pred = current_input_block_->LastPredecessor();
                //     old_pred != nullptr; old_pred = old_pred->NeighboringPredecessor()) {
                //    // Store the current index of the {old_pred}.
                //    old_pred->set_custom_data(pos--,