// Copyright 2013 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// #![allow(dead_code)] // Suppress warnings about unused code
// #![allow(unused_variables)] // Suppress warnings about unused variables

use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::ops::{Index, IndexMut};
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

//use crate::base::iterator::*;
//use crate::builtins::profile_data_reader::*;
//use crate::codegen::tick_counter::*;
//use crate::compiler::common_operator::*;
//use crate::compiler::control_equivalence::*;
//use crate::compiler::node_marker::*;
//use crate::compiler::node_properties::*;
//use crate::compiler::node::*;
//use crate::compiler::turbofan_graph::*;
//use crate::utils::bit_vector::*;
//use crate::zone::zone_containers::*;

// Define a macro for tracing (similar to C++ TRACE)
macro_rules! trace {
    ($($arg:tt)*) => {
        if cfg!(feature = "trace_turbo_scheduler") {
            println!($($arg)*);
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(usize);

impl NodeId {
    pub fn to_usize(&self) -> usize {
        self.0
    }

    pub fn from_usize(id: usize) -> Self {
        NodeId(id)
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BasicBlockId(usize);

impl BasicBlockId {
    pub fn to_usize(&self) -> usize {
        self.0
    }

    pub fn from_usize(id: usize) -> Self {
        BasicBlockId(id)
    }

    pub fn to_int(&self) -> i32 {
        self.0 as i32
    }

    pub fn to_size(&self) -> usize {
        self.0
    }

    pub fn from_int(id: i32) -> Self {
        BasicBlockId(id as usize)
    }
}

impl fmt::Display for BasicBlockId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Dummy definitions for types not provided
pub struct Zone {}

impl Zone {
    pub fn new() -> Self {
        Zone {}
    }
    pub fn allocate_array<T>(&self, count: usize) -> Vec<T>
    where
        T: Default + Copy,
    {
        vec![T::default(); count]
    }

    pub fn new_in<T>(&self, value: T) -> Box<T> {
        Box::new(value)
    }
}

pub struct TFGraph {
    nodes: Vec<Node>,
    end_node: NodeId,
}

impl TFGraph {
    pub fn new(node_count: usize) -> Self {
        TFGraph {
            nodes: vec![Node::default(); node_count],
            end_node: NodeId(0),
        }
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn end(&self) -> &Node {
        &self.nodes[self.end_node.0]
    }

    pub fn clone_node(&self, node: &Node) -> Node {
        node.clone()
    }

    pub fn node(&self, id: NodeId) -> &Node {
        &self.nodes[id.0]
    }
}

pub struct Schedule {}
impl Schedule {
    pub fn new(zone: &Zone, node_count_hint: usize) -> Self {
        Schedule {}
    }

    pub fn end(&self) -> &BasicBlock {
        &BasicBlock::default()
    }
    pub fn start(&self) -> &BasicBlock {
        &BasicBlock::default()
    }
    pub fn block(&self, node: &Node) -> &BasicBlock {
        &BasicBlock::default()
    }
    pub fn new_basic_block(&self) -> BasicBlock {
        BasicBlock::default()
    }
    pub fn add_node(&self, block: &BasicBlock, node: &Node) {
        // Implementation here
    }
    pub fn rpo_order(&self) -> &Vec<BasicBlock> {
        &vec![]
    }
    pub fn basic_block_count(&self) -> usize {
        0
    }
    pub fn insert_branch(
        &self,
        component_start: &BasicBlock,
        component_end: &BasicBlock,
        branch: &Node,
        successor_blocks0: &BasicBlock,
        successor_blocks1: &BasicBlock,
    ) {
        // Implementation here
    }
    pub fn add_branch(
        &self,
        branch_block: &BasicBlock,
        branch: &Node,
        successor_blocks0: &BasicBlock,
        successor_blocks1: &BasicBlock,
    ) {
        // Implementation here
    }
    pub fn insert_switch(
        &self,
        component_start: &BasicBlock,
        component_end: &BasicBlock,
        sw: &Node,
        successor_blocks: &[BasicBlock],
        successor_count: usize,
    ) {
        // Implementation here
    }
    pub fn add_switch(
        &self,
        switch_block: &BasicBlock,
        sw: &Node,
        successor_blocks: &[BasicBlock],
        successor_count: usize,
    ) {
        // Implementation here
    }
    pub fn add_goto(&self, predecessor_block: &BasicBlock, block: &BasicBlock) {
        // Implementation here
    }
    pub fn add_tail_call(&self, call_block: &BasicBlock, call: &Node) {
        // Implementation here
    }
    pub fn add_return(&self, return_block: &BasicBlock, ret: &Node) {
        // Implementation here
    }
    pub fn add_deoptimize(&self, deoptimize_block: &BasicBlock, deopt: &Node) {
        // Implementation here
    }
    pub fn add_throw(&self, throw_block: &BasicBlock, thr: &Node) {
        // Implementation here
    }

    pub fn add_call(
        &self,
        call_block: &BasicBlock,
        call: &Node,
        successor_blocks0: &BasicBlock,
        successor_blocks1: &BasicBlock,
    ) {
        // Implementation here
    }

    pub fn plan_node(&self, block: &BasicBlock, node: &Node) {
        // Implementation here
    }

    pub fn set_block_for_node(&self, block: &BasicBlock, node: &Node) {
        // Implementation here
    }

    pub fn get_block_by_id(&self, id: BasicBlockId) -> &BasicBlock {
        &BasicBlock::default()
    }
    pub fn is_scheduled(&self, node: &Node) -> bool {
        false
    }
    pub fn all_blocks(&self) -> &Vec<BasicBlock> {
        &vec![]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrOpcode {
    Start,
    End,
    Loop,
    Merge,
    Terminate,
    Branch,
    Switch,
    Deoptimize,
    TailCall,
    Return,
    Throw,
    Call,
    FastApiCall,
    Parameter,
    OsrValue,
    Phi,
    EffectPhi,
    Projection,
    BeginRegion,
    FinishRegion,
}

impl IrOpcode {
    pub fn is_merge_opcode(self) -> bool {
        self == IrOpcode::Merge || self == IrOpcode::Loop
    }
}

#[derive(Debug, Default, Clone)]
pub struct Node {
    id: NodeId,
    opcode: IrOpcode,
}

impl Node {
    pub fn new(id: NodeId, opcode: IrOpcode) -> Self {
        Node { id, opcode }
    }

    pub fn id(&self) -> NodeId {
        self.id
    }

    pub fn opcode(&self) -> IrOpcode {
        self.opcode
    }

    pub fn input_at(&self, i: usize) -> &Node {
        &Node::default()
    }

    pub fn uses(&self) -> Vec<&Node> {
        vec![]
    }

    pub fn input_edges(&self) -> Vec<Edge> {
        vec![]
    }

    pub fn use_edges(&self) -> Vec<Edge> {
        vec![]
    }

    pub fn op(&self) -> &Operator {
        &Operator::default()
    }
}

#[derive(Debug, Default, Clone)]
pub struct Operator {
    properties: OperatorProperties,
}

impl Operator {
    pub fn has_property(&self, property: OperatorProperties) -> bool {
        self.properties == property
    }

    pub fn control_output_count(&self) -> usize {
        0
    }
    pub fn mnemonic(&self) -> String {
        "operator".to_string()
    }
    pub fn effect_input_count(&self) -> usize {
        0
    }
    pub fn effect_output_count(&self) -> usize {
        0
    }
    pub fn value_output_count(&self) -> usize {
        0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorProperties {
    kPure,
}
impl Default for OperatorProperties {
    fn default() -> Self {
        OperatorProperties::kPure
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum BranchHint {
    #[default]
    kNone,
    kTrue,
    kFalse,
}

pub fn branch_hint_of(operator: &Operator) -> BranchHint {
    BranchHint::kNone
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Edge {}
impl Edge {
    pub fn to(&self) -> &Node {
        &Node::default()
    }
    pub fn from(&self) -> &Node {
        &Node::default()
    }
    pub fn index(&self) -> usize {
        0
    }

    pub fn update_to(&self, use_node: &Node) {
        // Implementation here
    }
}

// Dummy definitions for enums and structs used in the code
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flags {
    kTempSchedule = 1 << 0,
    kSplitNodes = 1 << 1,
}

impl Flags {
    pub fn from_bits(bits: u32) -> Self {
        match bits {
            1 => Flags::kTempSchedule,
            2 => Flags::kSplitNodes,
            _ => panic!("Invalid flags value"),
        }
    }
}

pub struct TickCounter {}

impl TickCounter {
    pub fn tick_and_maybe_enter_safepoint(&self) {}
}

pub struct ProfileDataFromFile {}

impl ProfileDataFromFile {
    pub fn get_hint(&self, id1: usize, id2: usize) -> BranchHint {
        BranchHint::kNone
    }
    pub fn get_executed_count(&self, block_id: usize) -> u64 {
        0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Placement {
    Unknown,
    Fixed,
    Coupled,
    Schedulable,
    Scheduled,
}

#[derive(Debug, Clone, Copy)]
pub struct SchedulerData {
    minimum_block_: *const BasicBlock,
    unscheduled_count_: i32,
    placement_: Placement,
}

impl Default for SchedulerData {
    fn default() -> Self {
        SchedulerData {
            minimum_block_: std::ptr::null(),
            unscheduled_count_: 0,
            placement_: Placement::Unknown,
        }
    }
}

pub struct Scheduler {
    zone_: Zone,
    graph_: TFGraph,
    schedule_: Schedule,
    flags_: Flags,
    scheduled_nodes_: Vec<Option<Vec<Node>>>, //Vec<NodeVector>
    schedule_root_nodes_: Vec<Node>,
    schedule_queue_: VecDeque<Node>,
    node_data_: Vec<SchedulerData>,
    tick_counter_: TickCounter,
    profile_data_: Option<ProfileDataFromFile>,
    common_dominator_cache_: HashMap<usize, HashMap<usize, *const BasicBlock>>,
    equivalence_: ControlEquivalence,
    special_rpo_: SpecialRPONumberer,
    control_flow_builder_: CFGBuilder,
}

impl Scheduler {
    pub fn new(
        zone: Zone,
        graph: TFGraph,
        schedule: Schedule,
        flags: Flags,
        node_count_hint: usize,
        tick_counter: TickCounter,
        profile_data: Option<ProfileDataFromFile>,
    ) -> Self {
        let mut node_data_ = vec![SchedulerData::default(); graph.node_count()];
        node_data_.reserve(node_count_hint);

        Scheduler {
            zone_: zone,
            graph_: graph,
            schedule_: schedule,
            flags_: flags,
            scheduled_nodes_: vec![None; schedule.basic_block_count()],
            schedule_root_nodes_: Vec::new(),
            schedule_queue_: VecDeque::new(),
            node_data_: node_data_,
            tick_counter_: tick_counter,
            profile_data_: profile_data,
            common_dominator_cache_: HashMap::new(),
            equivalence_: ControlEquivalence::default(),
            special_rpo_: SpecialRPONumberer::default(),
            control_flow_builder_: CFGBuilder::default(),
        }
    }

    pub fn compute_schedule(
        zone: &Zone,
        graph: &TFGraph,
        flags: Flags,
        tick_counter: &TickCounter,
        profile_data: Option<&ProfileDataFromFile>,
    ) -> Schedule {
        //let schedule_zone = if flags & Flags::kTempSchedule != 0 {
        //   zone
        //} else {
        //   graph.zone()
        //};

        // Reserve 10% more space for nodes if node splitting is enabled to try to
        // avoid resizing the vector since that would triple its zone memory usage.
        let node_hint_multiplier = if flags & Flags::kSplitNodes != 0 {
            1.1
        } else {
            1.0
        };
        let node_count_hint = (node_hint_multiplier * graph.node_count() as f64) as usize;

        let schedule = Schedule::new(zone, node_count_hint);
        let mut scheduler = Scheduler::new(
            Zone::new(),
            TFGraph::new(graph.node_count()),
            schedule,
            flags,
            node_count_hint,
            TickCounter {},
            profile_data.map(|p| p.clone()),
        );

        scheduler.build_cfg();
        //scheduler.compute_special_rpo_numbering();
        //scheduler.generate_dominator_tree();

        //scheduler.prepare_uses();
        //scheduler.schedule_early();
        //scheduler.schedule_late();

        //scheduler.seal_final_schedule();

        Schedule::new(&Zone::new(), 0)
    }

    fn default_scheduler_data(&self) -> SchedulerData {
        SchedulerData {
            minimum_block_: &self.schedule_.start() as *const BasicBlock,
            unscheduled_count_: 0,
            placement_: Placement::Unknown,
        }
    }

    fn get_data(&mut self, node: &Node) -> &mut SchedulerData {
        &mut self.node_data_[node.id().0]
    }

    fn initialize_placement(&mut self, node: &Node) -> Placement {
        let data = self.get_data(node);
        if data.placement_ == Placement::Fixed {
            // Nothing to do for control nodes that have been already fixed in
            // the schedule.
            return data.placement_;
        }
        assert_eq!(Placement::Unknown, data.placement_);
        match node.opcode() {
            IrOpcode::Parameter | IrOpcode::OsrValue => {
                // Parameters and OSR values are always fixed to the start block.
                data.placement_ = Placement::Fixed;
            }
            IrOpcode::Phi | IrOpcode::EffectPhi => {
                // Phis and effect phis are fixed if their control inputs are, whereas
                // otherwise they are coupled to a floating control node.
                //let p = self.get_placement(NodeProperties::get_control_input(node));
                //data.placement_ = if p == Placement::Fixed {
                //    Placement::Fixed
                //} else {
                //    Placement::Coupled
                //};
                data.placement_ = Placement::Coupled;
            }
            _ => {
                // Control nodes that were not control-reachable from end may float.
                data.placement_ = Placement::Schedulable;
            }
        }
        data.placement_
    }

    fn get_placement(&self, node: &Node) -> Placement {
        self.get_data_const(node).placement_
    }

    fn get_data_const(&self, node: &Node) -> &SchedulerData {
        &self.node_data_[node.id().0]
    }

    fn is_live(&self, node: &Node) -> bool {
        self.get_placement(node) != Placement::Unknown
    }

    fn update_placement(&mut self, node: &Node, placement: Placement) {
        let data = self.get_data(node);
        if data.placement_ == Placement::Unknown {
            // We only update control nodes from {kUnknown} to {kFixed}.  Ideally, we
            // should check that {node} is a control node (including exceptional calls),
            // but that is expensive.
            assert_eq!(Placement::Fixed, placement);
            data.placement_ = placement;
            return;
        }

        match node.opcode() {
            IrOpcode::Parameter => {
                // Parameters are fixed once and for all.
                panic!("UNREACHABLE");
            }
            IrOpcode::Phi | IrOpcode::EffectPhi => {
                // Phis and effect phis are coupled to their respective blocks.
                assert_eq!(Placement::Coupled, data.placement_);
                assert_eq!(Placement::Fixed, placement);
                //let control = NodeProperties::get_control_input(node);
                //let block = self.schedule_.block(control);
                //self.schedule_.add_node(block, node);
            }
            IrOpcode::Branch
            | IrOpcode::Switch
            | IrOpcode::Deoptimize
            | IrOpcode::TailCall
            | IrOpcode::Return
            | IrOpcode::Throw
            | IrOpcode::Call
            | IrOpcode::FastApiCall => {
                // Control nodes force coupled uses to be placed.
                for use_node in node.uses() {
                    if self.get_placement(use_node) == Placement::Coupled {
                        //assert_eq!(node, NodeProperties::get_control_input(use_node));
                        self.update_placement(use_node, placement);
                    }
                }
            }
            _ => {
                assert_eq!(Placement::Schedulable, data.placement_);
                assert_eq!(Placement::Scheduled, placement);
            }
        }
        // Reduce the use count of the node's inputs to potentially make them
        // schedulable. If all the uses of a node have been scheduled, then the node
        // itself can be scheduled.
        let coupled_control_edge = self.get_coupled_control_edge(node);
        for edge in node.input_edges() {
            //assert_eq!(node, edge.from());
            if Some(edge.index()) != coupled_control_edge {
                self.decrement_unscheduled_use_count(edge.to(), node);
            }
        }
        data.placement_ = placement;
    }

    fn get_coupled_control_edge(&self, node: &Node) -> Option<usize> {
        if self.get_placement(node) == Placement::Coupled {
            //Some(NodeProperties::first_control_index(node))
            Some(0) // Placeholder
        } else {
            None
        }
    }

    fn increment_unscheduled_use_count(&mut self, node: &Node, from: &Node) {
        // Tracking use counts for fixed nodes is useless.
        if self.get_placement(node) == Placement::Fixed {
            return;
        }

        // Use count for coupled nodes is summed up on their control.
        let mut node = node;
        if self.get_placement(node) == Placement::Coupled {
            //node = NodeProperties::get_control_input(node);
            //assert_ne!(self.get_placement(node), Placement::Fixed);
            //assert_ne!(self.get_placement(node), Placement::Coupled);
        }

        let data = self.get_data(node);
        data.unscheduled_count_ += 1;
        if cfg!(feature = "trace_turbo_scheduler") {
            trace!(
                "  Use count of #{}:{:?} (used by #{}:{:?})++ = {}",
                node.id().0,
                node.opcode(),
                from.id().0,
                from.opcode(),
                data.unscheduled_count_
            );
        }
    }

    fn decrement_unscheduled_use_count(&mut self, node: &Node, from: &Node) {
        // Tracking use counts for fixed nodes is useless.
        if self.get_placement(node) == Placement::Fixed {
            return;
        }

        // Use count for coupled nodes is summed up on their control.
        let mut node = node;
        if self.get_placement(node) == Placement::Coupled {
            //node = NodeProperties::get_control_input(node);
            //assert_ne!(self.get_placement(node), Placement::Fixed);
            //assert_ne!(self.get_placement(node), Placement::Coupled);
        }

        let data = self.get_data(node);
        assert!(data.unscheduled_count_ > 0);
        data.unscheduled_count_ -= 1;
        if cfg!(feature = "trace_turbo_scheduler") {
            trace!(
                "  Use count of #{}:{:?} (used by #{}:{:?})-- = {}",
                node.id().0,
                node.opcode(),
                from.id().0,
                from.opcode(),
                data.unscheduled_count_
            );
        }
        if data.unscheduled_count_ == 0 {
            trace!("    newly eligible #{}:{:?}", node.id().0, node.opcode());
            self.schedule_queue_.push_back(node.clone());
        }
    }

    // Phase 1: Build control-flow graph.
    fn build_cfg(&mut self) {
        trace!("--- CREATING CFG -------------------------------------------");

        // Instantiate a new control equivalence algorithm for the graph.
        self.equivalence_ = ControlEquivalence::new(&self.zone_, &self.graph_);

        // Build a control-flow graph for the main control-connected component that
        // is being spanned by the graph's start and end nodes.
        self.control_flow_builder_ = CFGBuilder::new(&self.zone_, self);
        self.control_flow_builder_.run();

        // Initialize per-block data.
        // Reserve an extra 10% to avoid resizing vector when fusing floating control.
        //self.scheduled_nodes_.reserve((self.schedule_.basic_block_count() as f64 * 1.1) as usize);
        //self.scheduled_nodes_.resize(self.schedule_.basic_block_count(), None);
    }

    fn compute_special_rpo_numbering(&mut self) {
        trace!("--- COMPUTING SPECIAL RPO ----------------------------------");

        // Compute the special reverse-post-order for basic blocks.
        self.special_rpo_ = SpecialRPONumberer::new(&self.zone_, &self.schedule_);
        //self.special_rpo_.compute_special_rpo();
    }

    fn generate_dominator_tree(&mut self) {
        trace!("--- IMMEDIATE BLOCK DOMINATORS -----------------------------");
        //self.generate_dominator_tree_internal(&self.schedule_);
    }

    fn generate_dominator_tree_internal(&mut self, schedule: &Schedule) {
        // Seed start block to be the first dominator.
        //schedule.start().set_dominator_depth(0);

        // Build the block dominator tree resulting from the above seed.
        //self.propagate_immediate_dominators(schedule.start().rpo_next());
    }

    fn prepare_uses(&mut self) {
        trace!("--- PREPARE USES ------------------------------------------");

        // Count the uses of every node, which is used to ensure that all of a
        // node's uses are scheduled before the node itself.
        //PrepareUsesVisitor::new(self, &self.graph_, &self.zone_).run();
    }

    fn schedule_early(&mut self) {
        if !self.special_rpo_.has_loop_blocks() {
            trace!("--- NO LOOPS SO SKIPPING SCHEDULE EARLY --------------------");
            return;
        }

        trace!("--- SCHEDULE EARLY -----------------------------------------");
        if cfg!(feature = "trace_turbo_scheduler") {
            trace!("roots: ");
            for node in &self.schedule_root_nodes_ {
                trace!("#{}:{:?} ", node.id().0, node.opcode());
            }
            trace!("");
        }

        // Compute the minimum block for each node thereby determining the earliest
        // position each node could be placed within a valid schedule.
        //ScheduleEarlyNodeVisitor::new(&self.zone_, self).run(&self.schedule_root_nodes_);
    }

    fn schedule_late(&mut self) {
        trace!("--- SCHEDULE LATE -------------------------------------------");
        if cfg!(feature = "trace_turbo_scheduler") {
            trace!("roots: ");
            for node in &self.schedule_root_nodes_ {
                trace!("#{}:{:?} ", node.id().0, node.opcode());
            }
            trace!("");
        }

        // Schedule: Places nodes in dominator block of all their uses.
        //ScheduleLateNodeVisitor::new(&self.zone_, self).run(&self.schedule_root_nodes_);
    }

    fn seal_final_schedule(&mut self) {
        trace!("--- SEAL FINAL SCHEDULE -------------------------------------");

        // Serialize the assembly order and reverse-post-order numbering.
        //self.special_rpo_.serialize_rpo_into_schedule();
        //self.special_rpo_.print_and_verify_special_rpo();

        // Add collected nodes for basic blocks to their blocks in the right order.
        //let mut block_num = 0;
        //for nodes in &self.scheduled_nodes_ {
        //    let id = BasicBlockId::from_int(block_num);
        //    let block = self.schedule_.get_block_by_id(id);
        //    if let Some(nodes) = nodes {
        //        for node in nodes.iter().rev() {
        //            self.schedule_.add_node(block, node);
        //        }
        //    }
        //    block_num += 1;
        //}
    }

    fn fuse_floating_control(&mut self, block: &BasicBlock, node: &Node) {
        trace!("--- FUSE FLOATING CONTROL ----------------------------------");
        if cfg!(feature = "trace_turbo_scheduler") {
            //StdoutStream{}.write_str("Schedule before control flow fusion:\n");
            //StdoutStream{}.write_str(&format!("{:?}", self.schedule_));
        }

        // Iterate on phase 1: Build control-flow graph.
        self.control_flow_builder_.run_with_block(block, node);

        // Iterate on phase 2: Compute special RPO and dominator tree.
        //self.special_rpo_.update_special_rpo(block, self.schedule_.block(node));
        //// TODO(turbofan): Currently "iterate on" means "re-run". Fix that.
        //for b in block.rpo_next().iter() {
        //    //b.set_dominator_depth(-1);
        //    //b.set_dominator(None);
        //}
        //self.propagate_immediate_dominators(block.rpo_next());

        //// Iterate on phase 4: Schedule nodes early.
        //// TODO(turbofan): The following loop gathering the propagation roots is a
        //// temporary solution and should be merged into the rest of the scheduler as
        //// soon as the approach settled for all floating loops.
        //let mut propagation_roots: Vec<Node> = self.control_flow_builder_.control_.clone();
        //for control in &self.control_flow_builder_.control_ {
        //    for use_node in control.uses() {
        //        //if NodeProperties::is_phi(use_node) && self.is_live(use_node) {
        //        //    propagation_roots.push(use_node.clone());
        //        //}
        //    }
        //}
        //if cfg!(feature = "trace_turbo_scheduler") {
        //    trace!("propagation roots: ");
        //    for r in &propagation_roots {
        //        trace!("#{}:{:?} ", r.id().0, r.opcode());
        //    }
        //    trace!("");
        //}
        //ScheduleEarlyNodeVisitor::new(&self.zone_, self).run(&propagation_roots);

        //// Move previously planned nodes.
        //// TODO(turbofan): Improve that by supporting bulk moves.
        //self.scheduled_nodes_.resize(self.schedule_.basic_block_count(), None);
        //self.move_planned_nodes(block, self.schedule_.block(node));

        if cfg!(feature = "trace_turbo_scheduler") {
            //StdoutStream{}.write_str("Schedule after control flow fusion:\n");
            //StdoutStream{}.write_str(&format!("{:?}", self.schedule_));
        }
    }

    fn move_planned_nodes(&mut self, from: &BasicBlock, to: &BasicBlock) {
        trace!(
            "Move planned nodes from id:{} to id:{}",
            from.id,
            to.id
        );
        //let from_nodes = self.scheduled_nodes_[from.id.0].clone();
        //let to_nodes = self.scheduled_nodes_[to.id.0].clone();
        //if from_nodes.is_none() {
        //    return;
        //}

        //for node in from_nodes.unwrap() {
        //    self.schedule_.set_block_for_node(to, &node);
        //}

        //if to_nodes.is_some() {
        //    let mut to_nodes_unwrapped = to_nodes.unwrap();
        //    to_nodes_unwrapped.extend_from_slice(&from_nodes.unwrap());
        //    //to_nodes.unwrap().extend_from_slice(&from_nodes.unwrap());
        //    //to_nodes.unwrap().insert(to_nodes.unwrap().end(), from_nodes.unwrap().begin(), from_nodes.unwrap().end());
        //    self.scheduled_nodes_[to.id.0] = Some(to_nodes_unwrapped);
        //    self.scheduled_nodes_[from.id.0] = None;
        //} else {
        //    //std::mem::swap(&self.scheduled_nodes_[from.id.0], &self.scheduled_nodes_[to.id.0]);
        //    self.scheduled_nodes_[to.id.0] = self.scheduled_nodes_[from.id.0].take();
        //}
    }
}

// Dummy definitions for ControlEquivalence class
#[derive(Default)]
struct ControlEquivalence {}

impl ControlEquivalence {
    fn new(zone: &Zone, graph: &TFGraph) -> Self {
        ControlEquivalence {}
    }

    fn run(&mut self, exit: &Node) {}
    fn class_of(&self, entry: &Node) -> usize {
        0
    }
}

// Dummy definitions for BasicBlock class
#[derive(Debug, Default, Clone, Copy)]
pub struct BasicBlock {
    id: BasicBlockId,
    rpo_number: i32,
    rpo_next: *const BasicBlock,
    loop_number: i32,
    loop_end: *const BasicBlock,
    loop_header: *const BasicBlock,
    loop_depth: i32,
    dominator: *const BasicBlock,
    dominator_depth: i32,
    deferred: bool,
}

impl BasicBlock {
    pub fn new(id: BasicBlockId) -> Self {
        BasicBlock {
            id,
            rpo_number: -1,
            rpo_next: std::ptr::null(),
            loop_number: -1,
            loop_end: std::ptr::null(),
            loop_header: std::ptr::null(),
            loop_depth: 0,
            dominator: std::ptr::null(),
            dominator_depth: 0,
            deferred: false,
        }
    }
    pub fn id(&self) -> BasicBlockId {
        self.id
    }
    pub fn set_rpo_number(&mut self,