// Converted from V8 C++ source files:
// Header: loop-unrolling.h
// Implementation: loop-unrolling.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(unused_variables)]

mod compiler {
    use std::cmp;
    use std::error::Error;
    use std::fmt;
    use std::ops::Range;
    use std::rc::Rc;

    //use crate::v8::internal::base;
    //use crate::v8::internal::codegen;
    //use crate::v8::internal::compiler::CommonOperatorBuilder;
    //use crate::v8::internal::compiler::NodeProperties;
    //use crate::v8::internal::compiler::SourcePositionTable;
    //use crate::v8::internal::compiler::TFGraph;
    //use crate::v8::internal::compiler::Zone;
    //use crate::v8::internal::compiler::NodeOriginTable;
    //use crate::v8::internal::compiler::IrOpcode;

    pub struct V8 {}

    pub struct Node {}
    impl Node {
        pub fn opcode(&self) -> IrOpcode {
            IrOpcode::kLoop
        }
        pub fn InputCount(&self) -> i32{
            2
        }
        pub fn InputAt(&self, index: usize) -> *mut Node{
            std::ptr::null_mut()
        }

        pub fn uses(&self) -> Vec<*mut Node> {
            Vec::new()
        }

        pub fn ReplaceInput(&self, index: usize, input: *mut Node) {}
        pub fn RemoveInput(&self, index: usize) {}
    }
    pub struct Uses {}

    pub struct TFGraph {}
    impl TFGraph {
        pub fn NewNode(&self, op: &Operator) -> *mut Node {
            std::ptr::null_mut()
        }
    }

    pub struct CommonOperatorBuilder {}
    impl CommonOperatorBuilder{
        pub fn Dead(&self) -> Operator {
            Operator{}
        }
        pub fn Merge(&self, count: usize) -> Operator {
            Operator{}
        }

        pub fn EffectPhi(&self, iteration_count: u32) -> Operator {
            Operator{}
        }

        pub fn Phi(&self, rep: LoopExitValueRepresentation, iteration_count: u32) -> Operator {
            Operator{}
        }

        pub fn Int32Constant(&self, value: i32) -> Operator {
            Operator{}
        }
        pub fn ResizeMergeOrPhi(&self, op: &Operator, count: i32) -> Operator {
            Operator{}
        }
    }

    pub struct Zone {}
    impl Zone{
        pub fn AllocateArray<T>(&self, count: usize) -> *mut T {
            std::ptr::null_mut()
        }
    }

    pub struct SourcePositionTable {}
    impl SourcePositionTable{
        pub fn AddDecorator(&self) {}
        pub fn RemoveDecorator(&self) {}
    }

    pub struct NodeOriginTable {}

    pub struct NodeVector {
        nodes: Vec<*mut Node>,
    }

    impl NodeVector {
        pub fn new(zone: &Zone) -> Self {
            NodeVector { nodes: Vec::new() }
        }
    }

    pub struct NodeCopier<'a> {
        graph: &'a TFGraph,
        copied_size: u32,
        copies: &'a NodeVector,
        unrolling_count: u32,
    }

    impl<'a> NodeCopier<'a> {
        pub fn new(
            graph: &'a TFGraph,
            copied_size: u32,
            copies: &'a NodeVector,
            unrolling_count: u32,
        ) -> Self {
            NodeCopier {
                graph,
                copied_size,
                copies,
                unrolling_count,
            }
        }

        pub fn CopyNodes(
            &self,
            graph: &TFGraph,
            zone: &Zone,
            dead: *mut Node,
            nodes: Range<*mut Node>,
            source_positions: &SourcePositionTable,
            node_origins: &NodeOriginTable,
        ) {
        }

        pub fn map(&self, node: *mut Node, n: u32) -> *mut Node {
            std::ptr::null_mut()
        }
    }

    pub struct Operator {}
    impl Operator {
        pub fn ValueInputCount(&self) -> i32{
            1
        }
        pub fn EffectInputCount(&self) -> i32{
            1
        }
    }

    pub enum IrOpcode {
        kLoop,
        kBranch,
        kStackPointerGreaterThan,
        kLoopExit,
        kTerminate,
        kLoopExitEffect,
        kLoopExitValue,
        kPhi,
        kMerge
    }

    impl IrOpcode {
        pub fn IsGraphTerminator(opcode: IrOpcode) -> bool {
            match opcode {
                IrOpcode::kTerminate => true,
                _ => false,
            }
        }

        pub fn IsLoop(opcode: IrOpcode) -> bool {
            match opcode {
                IrOpcode::kLoop => true,
                _ => false,
            }
        }
    }

    pub enum LoopExitValueRepresentation {
        kNone
    }

    pub fn LoopExitValueRepresentationOf(op: &Operator) -> LoopExitValueRepresentation {
        LoopExitValueRepresentation::kNone
    }

    pub type NodeId = u32;

    static KMAXIMUM_UNNESTED_SIZE: u32 = 50;
    static KMAXIMUM_UNROLLING_COUNT: u32 = 5;

    pub fn unrolling_count_heuristic(size: u32, depth: u32) -> u32 {
        cmp::min((depth + 1) * KMAXIMUM_UNNESTED_SIZE / size, KMAXIMUM_UNROLLING_COUNT)
    }

    pub fn maximum_unrollable_size(depth: u32) -> u32 {
        (depth + 1) * KMAXIMUM_UNNESTED_SIZE
    }

    pub struct ZoneUnorderedSet<T> {
        set: std::collections::HashSet<T>,
    }

    impl<T: Eq + std::hash::Hash> ZoneUnorderedSet<T> {
        pub fn new() -> Self {
            ZoneUnorderedSet {
                set: std::collections::HashSet::new(),
            }
        }

        pub fn insert(&mut self, value: T) {
            self.set.insert(value);
        }

        pub fn contains(&self, value: &T) -> bool {
            self.set.contains(value)
        }

        pub fn size(&self) -> usize {
            self.set.len()
        }

        pub fn begin(&self) -> std::collections::hash_set::Iter<T> {
            self.set.iter()
        }

        pub fn end(&self) -> std::collections::hash_set::Iter<T> {
            self.set.iter()
        }

        pub fn count(&self, use: *mut Node) -> i32{
            1
        }
    }

    impl From<Vec<*mut Node>> for ZoneUnorderedSet<*mut Node>{
        fn from(vec: Vec<*mut Node>) -> Self {
            let mut set = ZoneUnorderedSet::new();
            for node in vec {
                set.insert(node);
            }
            set
        }
    }

    pub fn UnrollLoop(
        loop_node: *mut Node,
        loop_: &mut ZoneUnorderedSet<*mut Node>,
        depth: u32,
        graph: &TFGraph,
        common: &CommonOperatorBuilder,
        tmp_zone: &Zone,
        source_positions: &SourcePositionTable,
        node_origins: &NodeOriginTable,
    ) {
        let loop_node_ref = unsafe { &*loop_node };
        if loop_node_ref.opcode() != IrOpcode::kLoop {
            return;
        }
        // No back-jump to the loop header means this is not really a loop.
        if loop_node_ref.InputCount() < 2 {
            return;
        }

        let unrolling_count = unrolling_count_heuristic(loop_.size() as u32, depth);
        if unrolling_count == 0 {
            return;
        }

        let iteration_count = unrolling_count + 1;
        let copied_size = loop_.size() as u32 * iteration_count;

        let copies = NodeVector::new(tmp_zone);

        let copier = NodeCopier::new(graph, copied_size, &copies, unrolling_count);
        source_positions.AddDecorator();
        copier.CopyNodes(
            graph,
            tmp_zone,
            graph.NewNode(&common.Dead()),
            loop_.begin().copied()..loop_.end().copied(),
            source_positions,
            node_origins,
        );
        source_positions.RemoveDecorator();

        // The terminator nodes in the copies need to get connected to the graph's end
        // node, except Terminate nodes which will be deleted anyway.
        //for node in &copies.nodes {
        //    if IrOpcode::IsGraphTerminator(node.opcode())
        //        && node.opcode() != IrOpcode::kTerminate
        //        && node.UseCount() == 0
        //    {
        //        NodeProperties::MergeControlToEnd(graph, common, node);
        //    }
        //}

        for node in loop_node_ref.uses() {
            let node_ref = unsafe { &*node };
            match node_ref.opcode() {
                IrOpcode::kBranch => {
                    /*** Step 1: Remove stack checks from all but the first iteration of the
                         loop. ***/
                    //let stack_check = node.InputAt(0);
                    //if stack_check.opcode() != IrOpcode::kStackPointerGreaterThan {
                    //    break;
                    //}
                    //// Replace value uses of the stack check with {true}, and remove the
                    //// stack check from the effect chain.
                    //for i in 0..unrolling_count {
                    //    for use_edge in copier.map(stack_check, i).use_edges() {
                    //        if NodeProperties::IsValueEdge(use_edge) {
                    //            use_edge.UpdateTo(graph.NewNode(common.Int32Constant(1)));
                    //        } else if NodeProperties::IsEffectEdge(use_edge) {
                    //            use_edge.UpdateTo(
                    //                NodeProperties::GetEffectInput(copier.map(stack_check, i)),
                    //            );
                    //        } else {
                    //            unreachable!();
                    //        }
                    //    }
                    //}
                    break;
                }

                IrOpcode::kLoopExit => {
                    /*** Step 2: Create merges for loop exits. ***/
                    //if node.InputAt(1) == loop_node {
                    //    // Create a merge node from all iteration exits.
                    //    let merge_inputs: Vec<Node> = Vec::new();
                    //        //tmp_zone.AllocateArray<Node>(iteration_count);
                    //    //merge_inputs[0] = node;
                    //    //for i in 1..iteration_count {
                    //    //    merge_inputs[i] = copier.map(node, i - 1);
                    //    //}
                    //    //let merge_node = graph.NewNode(common.Merge(iteration_count), merge_inputs);
                    //    //// Replace all uses of the loop exit with the merge node.
                    //    //for use_edge in node.use_edges() {
                    //    //    let use = use_edge.from();
                    //    //    if loop_.contains(use) {
                    //    //        // Uses within the loop will be LoopExitEffects and
                    //    //        // LoopExitValues. We need to create a phi from all loop
                    //    //        // iterations. Its merge will be the merge node for LoopExits.
                    //    //        let phi_operator: &Operator;
                    //    //        if use.opcode() == IrOpcode::kLoopExitEffect {
                    //    //            phi_operator = &common.EffectPhi(iteration_count);
                    //    //        } else {
                    //    //            assert_eq!(use.opcode(), IrOpcode::kLoopExitValue);
                    //    //            phi_operator = &common.Phi(
                    //    //                LoopExitValueRepresentationOf(use.op()),
                    //    //                iteration_count,
                    //    //            );
                    //    //        }
                    //    //        let phi_inputs = tmp_zone.AllocateArray<Node>(iteration_count + 1);
                    //    //        phi_inputs[0] = use;
                    //    //        for i in 1..iteration_count {
                    //    //            phi_inputs[i] = copier.map(use, i - 1);
                    //    //        }
                    //    //        phi_inputs[iteration_count] = merge_node;
                    //    //        let phi = graph.NewNode(phi_operator, phi_inputs);
                    //    //        use.ReplaceUses(phi);
                    //    //        // Repair phi which we just broke.
                    //    //        phi.ReplaceInput(0, use);
                    //    //    } else if use != merge_node {
                    //    //        // For uses outside the loop, simply redirect them to the merge.
                    //    //        use.ReplaceInput(use_edge.index(), merge_node);
                    //    //    }
                    //    //}
                    //}
                    break;
                }

                IrOpcode::kTerminate => {
                    // We only need to keep the Terminate node for the loop header of the
                    // first iteration.
                    //for i in 0..unrolling_count {
                    //    copier.map(node, i).Kill();
                    //}
                    break;
                }

                _ => {
                    break;
                }
            }
        }

        /*** Step 3: Rewire the iterations of the loop. Each iteration should flow
             into the next one, and the last should flow into the first. ***/

        // 3a) Rewire control.

        // We start at index=1 assuming that index=0 is the (non-recursive) loop
        // entry.
        //for input_index in 1..loop_node.InputCount() {
        //    let last_iteration_input =
        //        copier.map(loop_node, unrolling_count - 1).InputAt(input_index);
        //    for copy_index in (1..unrolling_count).rev() {
        //        copier
        //            .map(loop_node, copy_index)
        //            .ReplaceInput(input_index, copier.map(loop_node, copy_index - 1).InputAt(input_index));
        //    }
        //    copier
        //        .map(loop_node, 0)
        //        .ReplaceInput(input_index, loop_node.InputAt(input_index));
        //    loop_node.ReplaceInput(input_index, last_iteration_input);
        //}
        //// The loop of each following iteration will become a merge. We need to remove
        //// its non-recursive input.
        //for i in 0..unrolling_count {
        //    copier.map(loop_node, i).RemoveInput(0);
        //    NodeProperties::ChangeOp(
        //        copier.map(loop_node, i),
        //        common.Merge(loop_node.InputCount() - 1),
        //    );
        //}

        // 3b) Rewire phis and loop exits.
        //for use in loop_node.uses() {
        //    if NodeProperties::IsPhi(use) {
        //        let count = if use.opcode() == IrOpcode::kPhi {
        //            use.op().ValueInputCount()
        //        } else {
        //            use.op().EffectInputCount()
        //        };
        //        // Phis depending on the loop header should take their input from the
        //        // previous iteration instead.
        //        for input_index in 1..count {
        //            let last_iteration_input =
        //                copier.map(use, unrolling_count - 1).InputAt(input_index);
        //            for copy_index in (1..unrolling_count).rev() {
        //                copier
        //                    .map(use, copy_index)
        //                    .ReplaceInput(input_index, copier.map(use, copy_index - 1).InputAt(input_index));
        //            }
        //            copier
        //                .map(use, 0)
        //                .ReplaceInput(input_index, use.InputAt(input_index));
        //            use.ReplaceInput(input_index, last_iteration_input);
        //        }

        //        // Phis in each following iteration should not depend on the
        //        // (non-recursive) entry to the loop. Remove their first input.
        //        for i in 0..unrolling_count {
        //            copier.map(use, i).RemoveInput(0);
        //            NodeProperties::ChangeOp(
        //                copier.map(use, i),
        //                common.ResizeMergeOrPhi(use.op(), count - 1),
        //            );
        //        }
        //    }

        //    // Loop exits should point to the loop header.
        //    if use.opcode() == IrOpcode::kLoopExit {
        //        for i in 0..unrolling_count {
        //            copier.map(use, i).ReplaceInput(1, loop_node);
        //        }
        //    }
        //}
    }
}
