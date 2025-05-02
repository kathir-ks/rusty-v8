// Copyright 2025 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/maglev/maglev-inlining.h

use std::any::Any;

use crate::base::logging::*;
use crate::compiler::heap_refs::*;
use crate::compiler::js_heap_broker::*;
use crate::execution::local_isolate::*;
use crate::maglev::maglev_basic_block::*;
use crate::maglev::maglev_compilation_info::*;
use crate::maglev::maglev_compilation_unit::*;
use crate::maglev::maglev_deopt_frame_visitor::*;
use crate::maglev::maglev_graph_builder::*;
use crate::maglev::maglev_graph_processor::*;
use crate::maglev::maglev_ir::*;
use crate::objects::shared_function_info::*;
use crate::v8_flags;
use std::fmt::Debug;

pub mod maglev_inlining {

    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::ops::{Deref, DerefMut};
    use crate::{debug_assert, debug_assert_ne, debug_assert_eq, debug_assert_not_null};
    use crate::maglev::maglev_graph_processor::{GraphProcessor, ProcessingState, ProcessResult, BlockProcessResult};
    use crate::maglev::maglev_graph_builder::ReduceResult;
    use crate::maglev::maglev_compilation_unit::MaglevCompilationUnit;
    use crate::compiler::bytecode_array_ref::BytecodeArrayRef;
    use crate::zone::Zone;
    use crate::compiler::shared_function_info_ref::SharedFunctionInfoRef;
    use crate::objects::shared_function_info::SharedFunctionInfo;
    use crate::maglev::maglev_ir::ValueRepresentation;
    use crate::maglev::maglev_graph_builder::MaglevGraphBuilder;
    use std::collections::HashSet;

    /// A struct for updating node inputs during inlining.
    pub struct UpdateInputsProcessor<'a> {
        from_: &'a mut ValueNode,
        to_: &'a mut ValueNode,
    }

    impl<'a> UpdateInputsProcessor<'a> {
        /// Creates a new `UpdateInputsProcessor`.
        pub fn new(from_: &'a mut ValueNode, to_: &'a mut ValueNode) -> Self {
            UpdateInputsProcessor { from_, to_ }
        }
    }

    impl<'a> GraphProcessor for UpdateInputsProcessor<'a> {
        fn pre_process_graph(&mut self, _graph: &mut Graph) {}
        fn post_process_graph(&mut self, _graph: &mut Graph) {}
        fn post_process_basic_block(&mut self, _block: &mut BasicBlock) {}
        fn pre_process_basic_block(&mut self, _block: &mut BasicBlock) -> BlockProcessResult {
            BlockProcessResult::Continue
        }
        fn post_phi_processing(&mut self) {}

        fn process<T: Node>(&mut self, node: &mut T, _state: &ProcessingState) -> ProcessResult {
            for input in node.inputs_mut() {
                if input.node() as *const _ == self.from_ as *const _ {
                    input.set_node(self.to_);
                    self.to_.add_use();
                }
            }

            if T::kProperties().can_eager_deopt() {
                if let Some(eager_deopt_info) = node.eager_deopt_info_mut() {
                    eager_deopt_info.for_each_input(|node| {
                        if node as *const _ == self.from_ as *const _ {
                            *node = self.to_;
                            self.to_.add_use();
                        }
                    });
                }
            }

            if T::kProperties().can_lazy_deopt() {
                if let Some(lazy_deopt_info) = node.lazy_deopt_info_mut() {
                    lazy_deopt_info.for_each_input(|node| {
                        if node as *const _ == self.from_ as *const _ {
                            *node = self.to_;
                            self.to_.add_use();
                        }
                    });
                }
            }
            ProcessResult::Continue
        }
    }

    /// Manages inlining of functions within a Maglev graph.
    pub struct MaglevInliner<'a> {
        compilation_info_: &'a mut MaglevCompilationInfo,
        graph_: &'a mut Graph,
    }

    impl<'a> MaglevInliner<'a> {
        /// Creates a new `MaglevInliner`.
        pub fn new(compilation_info_: &'a mut MaglevCompilationInfo, graph_: &'a mut Graph) -> Self {
            MaglevInliner {
                compilation_info_,
                graph_,
            }
        }

        /// Runs the inlining process.
        pub fn run(&mut self, is_tracing_maglev_graphs_enabled: bool) {
            // TODO(victorgomes): Add some heuristics to choose which function to
            // inline.
            while !self.graph_.inlineable_calls().is_empty() {
                if self.graph_.total_inlined_bytecode_size() > v8_flags::max_maglev_inlined_bytecode_size_cumulative() {
                    // No more inlining.
                    break;
                }
                if let Some(mut call_site) = self.graph_.inlineable_calls_mut().pop() {
                    let result = self.build_inline_function(&mut call_site);
                    if let Some(mut result_node) = result {
                        if let Some(alloc) = result_node.try_cast_mut::<InlinedAllocation>() {
                            // TODO(victorgomes): Support eliding VOs.
                            alloc.force_escaping();
                            #[cfg(debug_assertions)]
                            alloc.set_is_returned_value_from_inline_call();
                        }
                        let mut substitute_use = UpdateInputsProcessor::new(
                            call_site.generic_call_node_mut(),
                            &mut result_node,
                        );
                        substitute_use.process_graph(self.graph_);

                    }
                    // If --trace-maglev-inlining-verbose, we print the graph after each
                    // inlining step/call.
                    if is_tracing_maglev_graphs_enabled && v8_flags::print_maglev_graphs() &&
                        v8_flags::trace_maglev_inlining_verbose() {
                        println!("\nAfter inlining {}", call_site.generic_call_node().shared_function_info());
                        self.graph_.print_graph(self.compilation_info_);
                    }
                }
            }
            // Otherwise we print just once at the end.
            if is_tracing_maglev_graphs_enabled && v8_flags::print_maglev_graphs() &&
                !v8_flags::trace_maglev_inlining_verbose() {
                println!("\nAfter inlining");
                self.graph_.print_graph(self.compilation_info_);
            }
        }

        /// Gets the `JSHeapBroker`.
        fn broker(&self) -> &JSHeapBroker {
            self.compilation_info_.broker()
        }
        /// Gets the `Zone`.
        fn zone(&self) -> &Zone {
            self.compilation_info_.zone()
        }

        /// Builds the inlined function.
        fn build_inline_function(&mut self, call_site: &mut MaglevCallSiteInfo) -> Option<ValueNode> {
            let call_node = call_site.generic_call_node_mut();
            let caller_details = &mut call_site.caller_details;
            let caller_deopt_frame = caller_details.deopt_frame_mut();
            let caller_unit = caller_deopt_frame.get_compilation_unit_mut();
            let shared = call_node.shared_function_info();

            if v8_flags::trace_maglev_inlining() {
                println!("  non-eager inlining {}", shared);
            }

            // Create a new compilation unit.
            let inner_unit = MaglevCompilationUnit::new_inner(
                self.zone(),
                caller_unit,
                shared,
                call_site.feedback_cell_mut(),
            );

            let bytecode = shared.get_bytecode_array(self.broker());
            self.graph_.add_inlined_bytecode_size(bytecode.length());

            // Create a new graph builder for the inlined function.
            let local_isolate = self.broker().local_isolate_or_isolate();
            let mut inner_graph_builder = MaglevGraphBuilder::new(
                local_isolate,
                inner_unit,
                self.graph_,
                caller_details
            );

            // Update caller deopt frame with inlined arguments.
            caller_details.deopt_frame = inner_graph_builder.add_inlined_arguments_to_deopt_frame(
                caller_deopt_frame,
                inner_unit,
                call_node.closure_mut().node_mut(),
                caller_details.arguments_mut(),
            );

            let call_block = call_node.owner_mut();

            // We truncate the graph to build the function in-place, preserving the
            // invariant that all jumps move forward (except JumpLoop).
            let saved_bb = self.truncate_graph_at(call_block);

            // Truncate the basic block and remove the generic call node.
            let control_node = call_block.reset_control_node_mut();
            let rem_nodes_in_call_block = call_block.split(call_node, self.zone());

            // Set the inner graph builder to build in the truncated call block.
            inner_graph_builder.set_current_block(call_block);

            let result = inner_graph_builder.build_inline_function(
                caller_deopt_frame.get_source_position(),
                call_node.context_mut().node_mut(),
                call_node.closure_mut().node_mut(),
                call_node.new_target_mut().node_mut(),
            );

            if result.is_done_with_abort() {
                // Restore the rest of the graph.
                for bb in saved_bb.into_iter() {
                    self.graph_.add(bb);
                }
                self.remove_predecessor_following(control_node, call_block);
                // TODO(victorgomes): We probably don't need to iterate all the graph to
                // remove unreachable blocks, but only the successors of control_node in
                // saved_bbs.
                self.remove_unreachable_blocks();
                return None;
            }

            debug_assert!(result.is_done_with_value());
            let mut returned_value = self.ensure_tagged(&mut inner_graph_builder, result.value_mut());

            // Resume execution using the final block of the inner builder.

            // Add remaining nodes to the final block and use the control flow of the
            // old call block.
            let final_block = inner_graph_builder.finish_inlined_block_for_caller(
                control_node,
                rem_nodes_in_call_block,
            );
            debug_assert_not_null!(final_block);

            // Update the predecessor of the successors of the {final_block}, that were
            // previously pointing to {call_block}.
            let final_block_ptr = final_block as *mut BasicBlock;
            final_block.for_each_successor(|successor| {
                Self::update_predecessors_of(successor, call_block, final_block_ptr);
            });

            // Restore the rest of the graph.
            for bb in saved_bb.into_iter() {
                self.graph_.add(bb);
            }

            return Some(returned_value);
        }

        /// Truncates the graph at the given basic block `block`.  All blocks
        /// following `block` (exclusive) are removed from the graph and returned.
        /// `block` itself is removed from the graph and not returned.
        fn truncate_graph_at(&mut self, block: &mut BasicBlock) -> Vec<BasicBlock> {
            // TODO(victorgomes): Consider using a linked list of basic blocks in Maglev
            // instead of a vector.
            let it = self.graph_.blocks().iter().position(|b| b as *const _ == block as *const _);
            debug_assert!(it.is_some());
            let index = it.unwrap();
            let saved_bb: Vec<BasicBlock> = self.graph_.blocks().drain(index + 1..).collect();
            saved_bb
        }

        /// Adds a node to the end of a basic block using the graph builder.
        fn add_node_at_block_end<Node: Node + 'static>(&mut self, builder: &mut MaglevGraphBuilder, inputs: Vec<&mut ValueNode>, args: Vec<Box<dyn Any>>) -> ValueNode {
            let node = NodeBase::new::<Node>(self.zone(), inputs, args);
             debug_assert!(!node.properties().can_eager_deopt());
             debug_assert!(!node.properties().can_lazy_deopt());
            builder.node_buffer_mut().push(node);
            self.register_node(builder, &mut builder.node_buffer_mut().last_mut().unwrap() as &mut dyn Node);
            let node_ptr = builder.node_buffer_mut().last_mut().unwrap() as *mut dyn Node as *mut ValueNode;
            unsafe {
                std::ptr::read(node_ptr)
            }
        }

        /// Registers a node with the graph labeller, if present.
        fn register_node(&mut self, builder: &mut MaglevGraphBuilder, node: &mut dyn Node) {
            if builder.has_graph_labeller() {
                builder.graph_labeller().register_node(node);
            }
        }

        /// Ensures that the value node is tagged.
        fn ensure_tagged(&mut self, builder: &mut MaglevGraphBuilder, node: &mut ValueNode) -> ValueNode {
            // TODO(victorgomes): Use KNA to create better conversion nodes?
            match node.value_representation() {
                ValueRepresentation::kInt32 => {
                    self.add_node_at_block_end::<Int32ToNumber>(builder, vec![node], vec![])
                }
                ValueRepresentation::kUint32 => {
                    self.add_node_at_block_end::<Uint32ToNumber>(builder, vec![node], vec![])
                }
                ValueRepresentation::kFloat64 => {
                    self.add_node_at_block_end::<Float64ToTagged>(
                        builder,
                        vec![node],
                        vec![Box::new(Float64ToTaggedConversionMode::kForceHeapNumber) as Box<dyn Any>],
                    )
                }
                ValueRepresentation::kHoleyFloat64 => {
                    self.add_node_at_block_end::<HoleyFloat64ToTagged>(
                        builder,
                        vec![node],
                        vec![Box::new(HoleyFloat64ToTaggedConversionMode::kForceHeapNumber) as Box<dyn Any>],
                    )
                }
                ValueRepresentation::kIntPtr => {
                    self.add_node_at_block_end::<IntPtrToNumber>(builder, vec![node], vec![])
                }
                ValueRepresentation::kTagged => {
                    let node_ptr = node as *mut ValueNode;
                    unsafe {
                        std::ptr::read(node_ptr)
                    }
                }
            }
        }

        /// Updates the predecessors of a basic block.
        fn update_predecessors_of(block: &mut BasicBlock, prev_pred: &mut BasicBlock, new_pred: *mut BasicBlock) {
            unsafe {
                if !block.has_state() {
                    debug_assert_eq!(block.predecessor() as *const _, prev_pred as *const _);
                    block.set_predecessor(new_pred);
                    return;
                }
                for i in 0..block.predecessor_count() {
                    if block.predecessor_at(i) as *const _ == prev_pred as *const _ {
                        block.state_mut().set_predecessor_at(i, new_pred);
                        break;
                    }
                }
            }
        }

        /// Removes the predecessor following a control node.
        fn remove_predecessor_following(&mut self, control: &mut ControlNode, call_block: &mut BasicBlock) {
            BasicBlock::for_each_successor_following(control, |succ| {
                if !succ.has_state() {
                    return;
                }
                if succ.is_loop() && succ.backedge_predecessor() as *const _ == call_block as *const _ {
                    succ.state_mut().turn_loop_into_regular_block();
                    return;
                }
                for i in (0..succ.predecessor_count()).rev() {
                    if succ.predecessor_at(i) as *const _ == call_block as *const _ {
                        succ.state_mut().remove_predecessor_at(i);
                    }
                }
            });
        }

        /// Removes unreachable blocks from the graph.
        fn remove_unreachable_blocks(&mut self) {
            let mut reachable_blocks: HashSet<*mut BasicBlock> = HashSet::new();
            let mut loop_headers_unreachable_by_backegde: HashSet<*mut BasicBlock> = HashSet::new();
            let mut worklist: Vec<*mut BasicBlock> = Vec::new();

            debug_assert!(!self.graph_.blocks().is_empty());
            let initial_bb = self.graph_.blocks().first_mut().unwrap();
            worklist.push(initial_bb as *mut BasicBlock);
            reachable_blocks.insert(initial_bb as *mut BasicBlock);
            debug_assert!(!initial_bb.is_loop());

            // Add all exception handler blocks to the worklist.
            // TODO(victorgomes): A catch block could still be unreachable, if no
            // bbs in its try-block are unreachables, or its nodes cannot throw.
            for bb in self.graph_.blocks_mut().iter_mut() {
                if bb.is_exception_handler_block() {
                    worklist.push(bb as *mut BasicBlock);
                    reachable_blocks.insert(bb as *mut BasicBlock);
                }
            }

            while let Some(current) = worklist.pop() {
                if unsafe { (*current).is_loop() } {
                    loop_headers_unreachable_by_backegde.insert(current);
                }

                unsafe {
                    (*current).for_each_successor(|succ| {
                        let succ_ptr = succ as *mut BasicBlock;
                        if reachable_blocks.contains(&succ_ptr) {
                            // We have already added this block to the worklist, check only if
                            // that's a reachable loop header.
                            if succ.is_loop() {
                                // This must be the loop back edge.
                                debug_assert!(succ.is_loop());
                                debug_assert_eq!(succ.backedge_predecessor() as *const _, current as *const _);
                                debug_assert!(loop_headers_unreachable_by_backegde.contains(&succ_ptr));
                                loop_headers_unreachable_by_backegde.remove(&succ_ptr);
                            }
                        } else {
                            reachable_blocks.insert(succ_ptr);
                            worklist.push(succ_ptr);
                        }
                    });
                }
            }

            for bb in loop_headers_unreachable_by_backegde {
                unsafe {
                    debug_assert!((*bb).has_state());
                    (*bb).state_mut().turn_loop_into_regular_block();
                }
            }

            let mut new_blocks: Vec<BasicBlock> = Vec::new();
            let blocks = self.graph_.blocks_mut();
            let mut i = 0;
            while i < blocks.len() {
                let bb = &mut blocks[i];
                let bb_ptr = bb as *mut BasicBlock;

                if reachable_blocks.contains(&bb_ptr) {
                    new_blocks.push(unsafe { std::ptr::read(bb_ptr) });

                    // Remove unreachable predecessors.
                    // If block doesn't have a merge state, it has only one predecessor, so
                    // it must be a reachable one.
                    if !bb.has_state() {
                        i += 1;
                        continue;
                    }

                    let predecessor_count = bb.predecessor_count();
                    for j in (0..predecessor_count).rev() {
                        if !reachable_blocks.contains(&bb.predecessor_at(j)) {
                            bb.state_mut().remove_predecessor_at(j);
                        }
                    }
                }
                blocks.remove(i);

            }
            self.graph_.set_blocks(new_blocks);
        }
    }
}