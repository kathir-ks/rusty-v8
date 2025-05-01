// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod duplication_optimization_reducer {
    use crate::compiler::turboshaft::assembler::*;
    use crate::compiler::turboshaft::graph::*;
    use crate::compiler::turboshaft::index::*;
    use crate::compiler::turboshaft::operations::*;
    use crate::compiler::turboshaft::value_numbering_reducer::*;
    use std::marker::PhantomData;

    // macro_rules! LABEL_BLOCK {
    //     ($name:ident, $block:block) => {
    //         let mut $name = || $block;
    //     };
    // }

    macro_rules! no_change_label_block {
        ($next_operation:expr) => {
            return $next_operation;
        };
    }

    pub struct DuplicationOptimizationReducer<Next> {
        next: Next,
    }

    impl<Next> DuplicationOptimizationReducer<Next> {
        pub fn new(next: Next) -> Self {
            DuplicationOptimizationReducer { next }
        }
    }

    impl<Next: Reducer> DuplicationOptimizationReducer<Next>
    where
        Next: ValueNumberingReducerMethods,
    {
        fn should_skip_optimization_step(&self) -> bool {
            // Placeholder for the actual logic from the original C++ code.
            false
        }

        fn maybe_duplicate_cond(
            &self,
            cond: &Operation,
            input_idx: OpIndex,
            input_graph: &InputGraph,
            new_graph: &mut OutputGraph,
        ) -> Option<V<Word32>> {
            if cond.saturated_use_count.is_one() {
                return None;
            }

            match cond.opcode {
                Opcode::kComparison => {
                    let comparison_op = cond.cast::<ComparisonOp>();
                    self.maybe_duplicate_comparison(
                        comparison_op,
                        input_idx,
                        input_graph,
                        new_graph,
                    )
                }
                Opcode::kWordBinop => {
                    let word_binop_op = cond.cast::<WordBinopOp>();
                    self.maybe_duplicate_word_binop(
                        word_binop_op,
                        input_idx,
                        input_graph,
                        new_graph,
                    )
                    .map(|idx| V::from(idx))
                }
                Opcode::kShift => {
                    let shift_op = cond.cast::<ShiftOp>();
                    self.maybe_duplicate_shift(shift_op, input_idx, input_graph, new_graph)
                        .map(|idx| V::from(idx))
                }
                _ => None,
            }
        }

        fn maybe_can_duplicate_generic_binop(
            &self,
            input_idx: OpIndex,
            left: OpIndex,
            right: OpIndex,
            input_graph: &InputGraph,
            new_graph: &OutputGraph,
        ) -> bool {
            if input_graph.get(left).saturated_use_count.is_one()
                && input_graph.get(right).saturated_use_count.is_one()
            {
                // We don't duplicate binops when all of their inputs are used a single
                // time (this would increase register pressure by keeping 2 values alive
                // instead of 1).
                return false;
            }
            let binop_output_idx = self.map_to_new_graph(input_idx, input_graph, new_graph);
            if new_graph.get(binop_output_idx).saturated_use_count.is_zero() {
                // This is the 1st use of {binop} in the output graph, so there is no need
                // to duplicate it just yet.
                return false;
            }
            true
        }

        fn maybe_duplicate_word_binop(
            &self,
            binop: &WordBinopOp,
            input_idx: OpIndex,
            input_graph: &InputGraph,
            new_graph: &mut OutputGraph,
        ) -> Option<OpIndex> {
            if !self.maybe_can_duplicate_generic_binop(
                input_idx,
                binop.left(),
                binop.right(),
                input_graph,
                new_graph,
            ) {
                return None;
            }

            match binop.kind {
                WordBinopOp::Kind::kSignedDiv
                | WordBinopOp::Kind::kUnsignedDiv
                | WordBinopOp::Kind::kSignedMod
                | WordBinopOp::Kind::kUnsignedMod => {
                    // These operations are somewhat expensive, and duplicating them is
                    // probably not worth it.
                    None
                }
                _ => {
                    let _disable_gvn = DisableValueNumbering::new(self);
                    Some(new_graph.word_binop(
                        self.map_to_new_graph(binop.left(), input_graph, new_graph),
                        self.map_to_new_graph(binop.right(), input_graph, new_graph),
                        binop.kind,
                        binop.rep,
                    ))
                }
            }
        }

        fn maybe_duplicate_comparison(
            &self,
            comp: &ComparisonOp,
            input_idx: OpIndex,
            input_graph: &InputGraph,
            new_graph: &mut OutputGraph,
        ) -> Option<V<Word32>> {
            if !self.maybe_can_duplicate_generic_binop(
                input_idx,
                comp.left(),
                comp.right(),
                input_graph,
                new_graph,
            ) {
                return None;
            }

            let _disable_gvn = DisableValueNumbering::new(self);
            Some(V::from(new_graph.comparison(
                self.map_to_new_graph(comp.left(), input_graph, new_graph),
                self.map_to_new_graph(comp.right(), input_graph, new_graph),
                comp.kind,
                comp.rep,
            )))
        }

        fn maybe_duplicate_shift(
            &self,
            shift: &ShiftOp,
            input_idx: OpIndex,
            input_graph: &InputGraph,
            new_graph: &mut OutputGraph,
        ) -> Option<OpIndex> {
            if !self.maybe_can_duplicate_generic_binop(
                input_idx,
                shift.left(),
                shift.right(),
                input_graph,
                new_graph,
            ) {
                return None;
            }

            let _disable_gvn = DisableValueNumbering::new(self);
            Some(new_graph.shift(
                self.map_to_new_graph(shift.left(), input_graph, new_graph),
                self.map_to_new_graph(shift.right(), input_graph, new_graph),
                shift.kind,
                shift.rep,
            ))
        }

        fn maybe_duplicate_output_graph_shift(
            &self,
            index: OpIndex,
            input_graph: &InputGraph,
            new_graph: &mut OutputGraph,
        ) -> OpIndex {
            let mut shifted: V<Word> = V::invalid();
            let mut shifted_by: i32 = 0;
            let mut shift_kind: ShiftOp::Kind = ShiftOp::Kind::kLeft;
            let mut shift_rep: WordRepresentation = WordRepresentation::Word32;

            if self.matcher().match_constant_shift(
                index,
                &mut shifted,
                &mut shift_kind,
                &mut shift_rep,
                &mut shifted_by,
                input_graph,
            ) && !new_graph.get(index).saturated_use_count.is_zero()
            {
                // We don't check the use count of {shifted}, because it might have uses
                // in the future that haven't been emitted yet.
                let _disable_gvn = DisableValueNumbering::new(self);
                return new_graph.shift(
                    shifted,
                    V::from(new_graph.word32_constant(shifted_by)),
                    shift_kind,
                    shift_rep,
                );
            }
            index
        }

        fn map_to_new_graph(
            &self,
            index: OpIndex,
            input_graph: &InputGraph,
            new_graph: &OutputGraph,
        ) -> OpIndex {
            // Assuming Next has a method to perform the graph mapping
            self.next.map_to_new_graph(index, input_graph, new_graph)
        }

        fn matcher(&self) -> &Matcher<'_> {
            // Assuming Next has a getter for the matcher
            self.next.matcher()
        }
    }

    pub trait Reducer {
        fn reduce_input_graph_branch(
            &mut self,
            ig_index: V<None>,
            branch: &BranchOp,
            input_graph: &InputGraph,
            output_graph: &mut OutputGraph,
        ) -> V<None>;

        fn reduce_input_graph_select(
            &mut self,
            ig_index: V<Any>,
            select: &SelectOp,
            input_graph: &InputGraph,
            output_graph: &mut OutputGraph,
        ) -> V<Any>;

        fn reduce_load(
            &mut self,
            base: OpIndex,
            index: Option<OpIndex>,
            kind: LoadOp::Kind,
            loaded_rep: MemoryRepresentation,
            result_rep: RegisterRepresentation,
            offset: i32,
            element_size_log2: u8,
            input_graph: &InputGraph,
            output_graph: &mut OutputGraph,
        ) -> OpIndex;

        fn reduce_store(
            &mut self,
            base: OpIndex,
            index: Option<OpIndex>,
            value: OpIndex,
            kind: StoreOp::Kind,
            stored_rep: MemoryRepresentation,
            write_barrier: WriteBarrierKind,
            offset: i32,
            element_size_log2: u8,
            maybe_initializing_or_transitioning: bool,
            maybe_indirect_pointer_tag: IndirectPointerTag,
            input_graph: &InputGraph,
            output_graph: &mut OutputGraph,
        ) -> OpIndex;

        fn map_to_new_graph(
            &self,
            index: OpIndex,
            input_graph: &InputGraph,
            new_graph: &OutputGraph,
        ) -> OpIndex;

        fn matcher(&self) -> &Matcher<'_>;
    }

    impl<Next: Reducer> ValueNumberingReducerMethods for DuplicationOptimizationReducer<Next> {}

    pub trait ValueNumberingReducerMethods {
        fn map_to_new_graph(
            &self,
            index: OpIndex,
            input_graph: &InputGraph,
            new_graph: &OutputGraph,
        ) -> OpIndex {
            index // Default implementation, can be overridden
        }
    }

    impl<Next: Reducer> DuplicationOptimizationReducer<Next> {
        pub fn reduce_input_graph_branch(
            &mut self,
            ig_index: V<None>,
            branch: &BranchOp,
            input_graph: &InputGraph,
            output_graph: &mut OutputGraph,
        ) -> V<None> {
            // LABEL_BLOCK!(no_change, {
            //     return self.next.reduce_input_graph_branch(ig_index, branch);
            // });
            if self.should_skip_optimization_step() {
                no_change_label_block!(self.next.reduce_input_graph_branch(
                    ig_index,
                    branch,
                    input_graph,
                    output_graph
                ));
            }

            let cond = input_graph.get(branch.condition());
            let mut new_cond: Option<V<Word32>> = None;

            if let Some(cond_val) = self.maybe_duplicate_cond(
                cond,
                branch.condition(),
                input_graph,
                output_graph,
            ) {
                new_cond = Some(cond_val);
            } else {
                no_change_label_block!(self.next.reduce_input_graph_branch(
                    ig_index,
                    branch,
                    input_graph,
                    output_graph
                ));
            }

            if let Some(new_cond) = new_cond {
                debug_assert!(new_cond.valid());
                let if_true = self.map_to_new_graph(branch.if_true, input_graph, output_graph);
                let if_false =
                    self.map_to_new_graph(branch.if_false, input_graph, output_graph);

                output_graph.branch(new_cond, if_true, if_false, branch.hint);

                V::<None>::invalid()
            } else {
                unreachable!()
            }
        }

        pub fn reduce_input_graph_select(
            &mut self,
            ig_index: V<Any>,
            select: &SelectOp,
            input_graph: &InputGraph,
            output_graph: &mut OutputGraph,
        ) -> V<Any> {
            // LABEL_BLOCK!(no_change, {
            //     return self.next.reduce_input_graph_select(ig_index, select);
            // });
            if self.should_skip_optimization_step() {
                no_change_label_block!(self.next.reduce_input_graph_select(
                    ig_index,
                    select,
                    input_graph,
                    output_graph
                ));
            }

            let cond = input_graph.get(select.cond());
            let mut new_cond: Option<V<Word32>> = None;

            if let Some(cond_val) = self.maybe_duplicate_cond(
                cond,
                select.cond(),
                input_graph,
                output_graph,
            ) {
                new_cond = Some(cond_val);
            } else {
                no_change_label_block!(self.next.reduce_input_graph_select(
                    ig_index,
                    select,
                    input_graph,
                    output_graph
                ));
            }

            if let Some(new_cond) = new_cond {
                debug_assert!(new_cond.valid());
                let vtrue = self.map_to_new_graph(select.vtrue(), input_graph, output_graph);
                let vfalse = self.map_to_new_graph(select.vfalse(), input_graph, output_graph);

                V::from(output_graph.select(
                    new_cond,
                    vtrue,
                    vfalse,
                    select.rep,
                    select.hint,
                    select.implem,
                ))
            } else {
                unreachable!()
            }
        }

        #[cfg(target_arch = "arm64")]
        pub fn reduce_load(
            &mut self,
            base: OpIndex,
            index: Option<OpIndex>,
            kind: LoadOp::Kind,
            loaded_rep: MemoryRepresentation,
            result_rep: RegisterRepresentation,
            offset: i32,
            element_size_log2: u8,
            input_graph: &InputGraph,
            output_graph: &mut OutputGraph,
        ) -> OpIndex {
            let mut index = index;
            if offset == 0 && element_size_log2 == 0 && index.is_some() {
                index = Some(self.maybe_duplicate_output_graph_shift(
                    index.unwrap(),
                    input_graph,
                    output_graph,
                ));
            }
            self.next.reduce_load(
                base,
                index,
                kind,
                loaded_rep,
                result_rep,
                offset,
                element_size_log2,
                input_graph,
                output_graph,
            )
        }

        #[cfg(not(target_arch = "arm64"))]
        pub fn reduce_load(
            &mut self,
            base: OpIndex,
            index: Option<OpIndex>,
            kind: LoadOp::Kind,
            loaded_rep: MemoryRepresentation,
            result_rep: RegisterRepresentation,
            offset: i32,
            element_size_log2: u8,
            input_graph: &InputGraph,
            output_graph: &mut OutputGraph,
        ) -> OpIndex {
            self.next.reduce_load(
                base,
                index,
                kind,
                loaded_rep,
                result_rep,
                offset,
                element_size_log2,
                input_graph,
                output_graph,
            )
        }

        #[cfg(target_arch = "arm64")]
        pub fn reduce_store(
            &mut self,
            base: OpIndex,
            index: Option<OpIndex>,
            value: OpIndex,
            kind: StoreOp::Kind,
            stored_rep: MemoryRepresentation,
            write_barrier: WriteBarrierKind,
            offset: i32,
            element_size_log2: u8,
            maybe_initializing_or_transitioning: bool,
            maybe_indirect_pointer_tag: IndirectPointerTag,
            input_graph: &InputGraph,
            output_graph: &mut OutputGraph,
        ) -> OpIndex {
            let mut index = index;
            if offset == 0 && element_size_log2 == 0 && index.is_some() {
                index = Some(self.maybe_duplicate_output_graph_shift(
                    index.unwrap(),
                    input_graph,
                    output_graph,
                ));
            }
            self.next.reduce_store(
                base,
                index,
                value,
                kind,
                stored_rep,
                write_barrier,
                offset,
                element_size_log2,
                maybe_initializing_or_transitioning,
                maybe_indirect_pointer_tag,
                input_graph,
                output_graph,
            )
        }

        #[cfg(not(target_arch = "arm64"))]
        pub fn reduce_store(
            &mut self,
            base: OpIndex,
            index: Option<OpIndex>,
            value: OpIndex,
            kind: StoreOp::Kind,
            stored_rep: MemoryRepresentation,
            write_barrier: WriteBarrierKind,
            offset: i32,
            element_size_log2: u8,
            maybe_initializing_or_transitioning: bool,
            maybe_indirect_pointer_tag: IndirectPointerTag,
            input_graph: &InputGraph,
            output_graph: &mut OutputGraph,
        ) -> OpIndex {
            self.next.reduce_store(
                base,
                index,
                value,
                kind,
                stored_rep,
                write_barrier,
                offset,
                element_size_log2,
                maybe_initializing_or_transitioning,
                maybe_indirect_pointer_tag,
                input_graph,
                output_graph,
            )
        }
    }

    struct DisableValueNumbering<'a, T> {
        reducer: &'a T,
        _phantom: PhantomData<&'a T>,
    }

    impl<'a, T> DisableValueNumbering<'a, T> {
        fn new(reducer: &'a T) -> Self {
            DisableValueNumbering {
                reducer,
                _phantom: PhantomData,
            }
        }
    }
}
