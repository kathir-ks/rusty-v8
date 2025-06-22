// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod register_configuration;
mod maglev_compilation_info;
mod maglev_graph_processor;
mod maglev_graph;
mod maglev_ir;

use std::cmp;
use std::collections::HashMap;
use std::mem;

use crate::maglev_graph::{BasicBlock, BlockProcessResult, Graph};
use crate::maglev_ir::{
    Input, InputLocation, Jump, JumpLoop, NodeBase, NodeIdT, NodeProperties,
    Phi, ProcessResult, ProcessingState, ValueNode, CheckpointedJump,
    kInvalidNodeId, kFirstValidNodeId, Identity, DeoptInfo, DeoptFrame,
    UnoptimizedFrameInfo, FastConstructStubFrameInfo, BuiltinContinuationFrameInfo,
};
use crate::maglev_compilation_info::MaglevCompilationInfo;
use crate::register_configuration::RegisterConfiguration;

const V8_COMPRESS_POINTERS: bool = cfg!(feature = "compress_pointers"); // Replace with actual conditional compilation check

/// Processes the graph to initialize temporaries and set value location constraints for each node.
pub struct ValueLocationConstraintProcessor {}

impl ValueLocationConstraintProcessor {
    /// Does nothing before processing the graph.
    pub fn pre_process_graph(&self, _graph: &mut Graph) {}
    /// Does nothing after processing the graph.
    pub fn post_process_graph(&self, _graph: &mut Graph) {}
    /// Does nothing after processing a basic block.
    pub fn post_process_basic_block(&self, _block: &mut BasicBlock) {}
    /// Does nothing before processing a basic block.
    pub fn pre_process_basic_block(&self, _block: &mut BasicBlock) -> BlockProcessResult {
        BlockProcessResult::Continue
    }
    /// Does nothing after phi processing.
    pub fn post_phi_processing(&self) {}

    /// Processes a node to initialize temporaries and set value location constraints.
    pub fn process<T: NodeBase>(&self, node: &mut T, _state: &ProcessingState) -> ProcessResult {
        node.init_temporaries();
        node.set_value_location_constraints();
        ProcessResult::Continue
    }
}

/// Marks tagged inputs as decompressing if pointer compression is enabled.
pub struct DecompressedUseMarkingProcessor {}

impl DecompressedUseMarkingProcessor {
    /// Does nothing before processing the graph.
    pub fn pre_process_graph(&self, _graph: &mut Graph) {}
    /// Does nothing after processing the graph.
    pub fn post_process_graph(&self, _graph: &mut Graph) {}
    /// Does nothing after processing a basic block.
    pub fn post_process_basic_block(&self, _block: &mut BasicBlock) {}
    /// Does nothing before processing a basic block.
    pub fn pre_process_basic_block(&self, _block: &mut BasicBlock) -> BlockProcessResult {
        BlockProcessResult::Continue
    }
     /// Does nothing after phi processing.
    pub fn post_phi_processing(&self) {}

    /// Processes a node to mark tagged inputs as decompressing.
    pub fn process<T: NodeBase>(&self, node: &mut T, _state: &ProcessingState) -> ProcessResult {
        if V8_COMPRESS_POINTERS {
            node.mark_tagged_inputs_as_decompressing();
        }
        ProcessResult::Continue
    }
}

/// Calculates the maximum call depth and deoptimized stack size.
pub struct MaxCallDepthProcessor {
    max_call_stack_args: i32,
    max_deopted_stack_size: i32,
    last_seen_unit: *const maglev_compilation_info::MaglevCompilationUnit, // Replace with proper type if accessible
}

impl MaxCallDepthProcessor {
    pub fn new() -> Self {
        MaxCallDepthProcessor {
            max_call_stack_args: 0,
            max_deopted_stack_size: 0,
            last_seen_unit: std::ptr::null(),
        }
    }

    /// Does nothing before processing the graph.
    pub fn pre_process_graph(&mut self, _graph: &mut Graph) {}

    /// Sets the maximum call stack arguments and deoptimized stack size on the graph.
    pub fn post_process_graph(&mut self, graph: &mut Graph) {
        graph.set_max_call_stack_args(self.max_call_stack_args);
        graph.set_max_deopted_stack_size(self.max_deopted_stack_size);
    }

    /// Does nothing after processing a basic block.
    pub fn post_process_basic_block(&mut self, _block: &mut BasicBlock) {}

    /// Does nothing before processing a basic block.
    pub fn pre_process_basic_block(&mut self, _block: &mut BasicBlock) -> BlockProcessResult {
        BlockProcessResult::Continue
    }
     /// Does nothing after phi processing.
    pub fn post_phi_processing(&mut self) {}

    /// Processes a node to update the maximum call depth and deoptimized stack size.
    pub fn process<T: NodeBase>(&mut self, node: &mut T, _state: &ProcessingState) -> ProcessResult {
        if T::kProperties.is_call() || T::kProperties.needs_register_snapshot() {
            let mut node_stack_args = node.max_call_stack_args();
            if T::kProperties.needs_register_snapshot() {
                // Pessimistically assume that we'll push all registers in deferred
                // calls.
                node_stack_args +=
                    register_configuration::kAllocatableGeneralRegisterCount + register_configuration::kAllocatableDoubleRegisterCount;
            }
            self.max_call_stack_args = cmp::max(self.max_call_stack_args, node_stack_args);
        }
        if T::kProperties.can_eager_deopt() {
            self.update_max_deopted_stack_size(node.eager_deopt_info());
        }
        if T::kProperties.can_lazy_deopt() {
            self.update_max_deopted_stack_size(node.lazy_deopt_info());
        }
        ProcessResult::Continue
    }

    fn update_max_deopted_stack_size(&mut self, deopt_info: &DeoptInfo) {
        let mut deopt_frame = &deopt_info.top_frame();
        let mut frame_size = 0;

        if deopt_frame.frame_type() == DeoptFrame::FrameType::InterpretedFrame {
            let unit = deopt_frame.as_interpreted().unit();
            if unit as *const _ == self.last_seen_unit {
                return;
            }
            self.last_seen_unit = unit as *const _;
            frame_size = deopt_frame.as_interpreted().unit().max_arguments() * mem::size_of::<usize>() as i32;
        }

        loop {
            frame_size += self.conservative_frame_size(deopt_frame);
            if let Some(parent) = deopt_frame.parent() {
                deopt_frame = parent;
            } else {
                break;
            }
        }
        self.max_deopted_stack_size = cmp::max(frame_size, self.max_deopted_stack_size);
    }

    fn conservative_frame_size(&self, deopt_frame: &DeoptFrame) -> i32 {
        match deopt_frame.frame_type() {
            DeoptFrame::FrameType::InterpretedFrame => {
                let info = UnoptimizedFrameInfo::conservative(
                    deopt_frame.as_interpreted().unit().parameter_count(),
                    deopt_frame.as_interpreted().unit().register_count(),
                );
                info.frame_size_in_bytes()
            }
            DeoptFrame::FrameType::ConstructInvokeStubFrame => {
                FastConstructStubFrameInfo::conservative().frame_size_in_bytes()
            }
            DeoptFrame::FrameType::InlinedArgumentsFrame => {
                cmp::max(
                    0,
                    deopt_frame.as_inlined_arguments().arguments().len() as i32 -
                        deopt_frame.as_inlined_arguments().unit().parameter_count() *
                        mem::size_of::<usize>() as i32,
                )
            }
            DeoptFrame::FrameType::BuiltinContinuationFrame => {
                let config = RegisterConfiguration::default();
                let info = BuiltinContinuationFrameInfo::conservative(
                    deopt_frame.as_builtin_continuation().parameters().len() as i32,
                    deopt_frame.as_builtin_continuation().builtin_id(),
                    &config,
                );
                info.frame_size_in_bytes()
            }
        }
    }
}

/// Calculates live ranges and next use information for nodes in the graph.
pub struct LiveRangeAndNextUseProcessor<'a> {
    compilation_info: &'a mut MaglevCompilationInfo,
    next_node_id: NodeIdT,
    loop_used_nodes: Vec<LoopUsedNodes<'a>>,
}

impl<'a> LiveRangeAndNextUseProcessor<'a> {
    /// Creates a new `LiveRangeAndNextUseProcessor`.
    pub fn new(compilation_info: &'a mut MaglevCompilationInfo) -> Self {
        LiveRangeAndNextUseProcessor {
            compilation_info,
            next_node_id: kFirstValidNodeId,
            loop_used_nodes: Vec::new(),
        }
    }

    /// Does nothing before processing the graph.
    pub fn pre_process_graph(&mut self, _graph: &mut Graph) {}

    /// Asserts that there are no remaining loop used nodes after processing the graph.
    pub fn post_process_graph(&mut self, _graph: &mut Graph) {
        assert!(self.loop_used_nodes.is_empty());
    }

    /// Does nothing after processing a basic block.
    pub fn post_process_basic_block(&mut self, _block: &mut BasicBlock) {}

    /// Processes a basic block to track loop used nodes.
    pub fn pre_process_basic_block(&mut self, block: &mut BasicBlock) -> BlockProcessResult {
        if !block.has_state() {
            return BlockProcessResult::Continue;
        }
        if block.state().is_loop() {
            self.loop_used_nodes.push(LoopUsedNodes {
                used_nodes: HashMap::new(),
                first_call: kInvalidNodeId,
                last_call: kInvalidNodeId,
                header: block,
            });
        }
        BlockProcessResult::Continue
    }
     /// Does nothing after phi processing.
    pub fn post_phi_processing(&mut self) {}

    /// Processes a node to set its ID, track loop used nodes, and mark input uses.
    pub fn process<T: NodeBase>(&mut self, node: &mut T, state: &ProcessingState) -> ProcessResult {
        node.set_id(self.next_node_id);
        self.next_node_id += 1;

        let loop_used_nodes = self.get_current_loop_used_nodes_mut();
        if let Some(loop_used_nodes) = loop_used_nodes {
            if node.properties().is_call() && loop_used_nodes.header.has_state() {
                if loop_used_nodes.first_call == kInvalidNodeId {
                    loop_used_nodes.first_call = node.id();
                }
                loop_used_nodes.last_call = node.id();
            }
        }

        self.mark_input_uses(node, state);
        ProcessResult::Continue
    }

    fn mark_input_uses<T: NodeBase>(&mut self, node: &mut T, state: &ProcessingState) {
        let loop_used_nodes = self.get_current_loop_used_nodes_mut();

        // Mark input uses in the same order as inputs are assigned in the register
        // allocator (see StraightForwardRegisterAllocator::AssignInputs).
        node.for_all_inputs_in_regalloc_assignment_order(
            |policy, input| {
                self.mark_use(input.node(), node.id(), input, loop_used_nodes);
            },
        );
        if T::kProperties.can_eager_deopt() {
            self.mark_checkpoint_nodes(node, node.eager_deopt_info(), loop_used_nodes, state);
        }
        if T::kProperties.can_lazy_deopt() {
            self.mark_checkpoint_nodes(node, node.lazy_deopt_info(), loop_used_nodes, state);
        }
    }

    /// Specialization for Phi nodes (empty implementation).
    pub fn mark_input_uses_phi(&mut self, _node: &mut Phi, _state: &ProcessingState) {
        // Don't mark Phi uses when visiting the node, because of loop phis.
        // Instead, they'll be visited while processing Jump/JumpLoop.
    }

    /// Specialization for JumpLoop nodes.
    pub fn mark_input_uses_jump_loop(&mut self, node: &mut JumpLoop, state: &ProcessingState) {
        let predecessor_id = state.block().predecessor_id();
        let target = node.target();
        let use_id = node.id();

        assert!(!self.loop_used_nodes.is_empty());
        let mut loop_used_nodes = self.loop_used_nodes.pop().unwrap();

        let outer_loop_used_nodes = self.get_current_loop_used_nodes_mut();

        if target.has_phi() {
            for phi in target.phis().iter_mut() {
                assert!(phi.is_used());
                let input = phi.input(predecessor_id);
                self.mark_use(input.node(), use_id, &mut phi.input(predecessor_id), outer_loop_used_nodes);
            }
        }

        assert_eq!(loop_used_nodes.header, target);
        if !loop_used_nodes.used_nodes.is_empty() {
            // Try to avoid unnecessary reloads or spills across the back-edge based
            // on use positions and calls inside the loop.
            let reload_hints = &mut loop_used_nodes.header.reload_hints();
            let spill_hints = &mut loop_used_nodes.header.spill_hints();
            for (used_node, info) in loop_used_nodes.used_nodes.iter() {
                // If the node is used before the first call and after the last call,
                // keep it in a register across the back-edge.
                if info.first_register_use != kInvalidNodeId &&
                    (loop_used_nodes.first_call == kInvalidNodeId ||
                     (info.first_register_use <= loop_used_nodes.first_call &&
                      info.last_register_use > loop_used_nodes.last_call)) {
                    reload_hints.push(*used_node);
                }
                // If the node is not used, or used after the first call and before the
                // last call, keep it spilled across the back-edge.
                if info.first_register_use == kInvalidNodeId ||
                    (loop_used_nodes.first_call != kInvalidNodeId &&
                     info.first_register_use > loop_used_nodes.first_call &&
                     info.last_register_use <= loop_used_nodes.last_call) {
                    spill_hints.push(*used_node);
                }
            }

            // Uses of nodes in this loop may need to propagate to an outer loop, so
            // that they're lifetime is extended there too.
            // TODO(leszeks): We only need to extend the lifetime in one outermost
            // loop, allow nodes to be "moved" between lifetime extensions.
            let used_node_inputs: Vec<Input> = loop_used_nodes.used_nodes.iter().map(|(used_node, _)| Input::new(*used_node)).collect();

            node.set_used_nodes(used_node_inputs);

            for used_node in node.used_nodes().iter() {
                self.mark_use(used_node.node(), use_id, used_node, outer_loop_used_nodes);
            }
        }
    }

    /// Specialization for Jump nodes.
    pub fn mark_input_uses_jump(&mut self, node: &mut Jump, state: &ProcessingState) {
        self.mark_jump_input_uses(node.id(), node.target(), state);
    }

    /// Specialization for CheckpointedJump nodes.
    pub fn mark_input_uses_checkpointed_jump(&mut self, node: &mut CheckpointedJump, state: &ProcessingState) {
        self.mark_jump_input_uses(node.id(), node.target(), state);
    }

    fn mark_jump_input_uses(&mut self, use_id: u32, target: &mut BasicBlock, state: &ProcessingState) {
        let i = state.block().predecessor_id();
        if !target.has_phi() {
            return;
        }

        let loop_used_nodes = self.get_current_loop_used_nodes_mut();

        let phis = &mut target.phis_mut();
        let mut it = 0;
        while it < phis.len() {
            let phi = &mut phis[it];
            if !phi.is_used() {
                phis.remove(it);
                // Removed the current element, so don't increment `it`.
            } else {
                let input = phi.input(i);
                self.mark_use(input.node(), use_id, &mut phi.input(i), loop_used_nodes);
                it += 1;
            }
        }
    }

    fn get_current_loop_used_nodes_mut(&mut self) -> Option<&mut LoopUsedNodes<'a>> {
        self.loop_used_nodes.last_mut()
    }

    fn mark_use(&mut self, node: *mut ValueNode, use_id: u32, input: &mut InputLocation, loop_used_nodes: Option<&mut LoopUsedNodes<'a>>) {
        if unsafe { node.is::<Identity>() } {
            return;
        }

        unsafe {
            (*node).record_next_use(use_id, input);
        }

        // If we are in a loop, loop_used_nodes is non-null. In this case, check if
        // the incoming node is from outside the loop, and make sure to extend its
        // lifetime to the loop end if yes.
        if let Some(loop_used_nodes) = loop_used_nodes {
            // If the node's id is smaller than the smallest id inside the loop, then
            // it must have been created before the loop. This means that it's alive
            // on loop entry, and therefore has to be alive across the loop back edge
            // too.
            let node_id = unsafe { (*node).id() };
            if node_id < loop_used_nodes.header.first_id() {
                let entry = loop_used_nodes.used_nodes.entry(unsafe {&mut *node}).or_insert(NodeUse {
                    first_register_use: kInvalidNodeId,
                    last_register_use: kInvalidNodeId,
                });

                if input.operand().is_unallocated() {
                    let operand = input.operand();

                    if operand.has_register_policy() || operand.has_fixed_register_policy() || operand.has_fixed_fp_register_policy() {
                        if entry.first_register_use == kInvalidNodeId {
                            entry.first_register_use = use_id;
                        }
                        entry.last_register_use = use_id;
                    }
                }
            }
        }
    }

    fn mark_checkpoint_nodes<DeoptInfoT: crate::maglev_ir::ForEachInput>(&mut self, node: &mut NodeBase, deopt_info: &mut DeoptInfoT, loop_used_nodes: Option<&mut LoopUsedNodes<'a>>, state: &ProcessingState) {
        let use_id = node.id();
        if !deopt_info.has_input_locations() {
            let mut count = 0;
            deopt_info.for_each_input(|_| count += 1);
            deopt_info.initialize_input_locations(self.compilation_info.zone_mut(), count);
        }
        let mut input = deopt_info.input_locations();
        deopt_info.for_each_input(|value_node| {
            self.mark_use(value_node, use_id, input, loop_used_nodes);
            input = unsafe { input.add(1) }; // Increment the pointer
        });
    }
}

/// Stores first and last register use IDs.
#[derive(Clone, Copy, Debug)]
struct NodeUse {
    first_register_use: NodeIdT,
    last_register_use: NodeIdT,
}

/// Stores information about nodes used inside a loop.
struct LoopUsedNodes<'a> {
    used_nodes: HashMap<*mut ValueNode, NodeUse>,
    first_call: NodeIdT,
    last_call: NodeIdT,
    header: &'a mut BasicBlock,
}