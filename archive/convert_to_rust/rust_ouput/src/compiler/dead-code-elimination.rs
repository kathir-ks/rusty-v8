// Converted from V8 C++ source files:
// Header: dead-code-elimination.h
// Implementation: dead-code-elimination.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod dead_code_elimination {
use crate::base::compiler_specific::NON_EXPORTED_BASE;
use crate::codegen::machine_type::MachineRepresentation;
use crate::compiler::common_operator::CommonOperatorBuilder;
use crate::compiler::graph_reducer::AdvancedReducer;
use crate::compiler::node_properties::NodeProperties;
use crate::compiler::operator_properties::OperatorProperties;
use crate::compiler::turbofan_graph::TFGraph;
use crate::compiler::wasm_gc_operator_reducer::{v8, Reduction};
use crate::compiler::Node;
use crate::compiler::Operator;
use crate::compiler::Zone;
use std::cell::RefCell;
use std::rc::Rc;

pub struct DeadCodeElimination {
    advanced_reducer: AdvancedReducer,
    graph_: *mut TFGraph,
    common_: *mut CommonOperatorBuilder,
    dead_: *mut Node,
    zone_: *mut Zone,
}

impl DeadCodeElimination {
    pub fn new(editor: &mut Editor, graph: *mut TFGraph, common: *mut CommonOperatorBuilder, temp_zone: *mut Zone) -> Self {
        unsafe {
        let dead_node = (*graph).NewNode((*common).Dead());
        NodeProperties::SetType(dead_node, Type::None());

        DeadCodeElimination {
            advanced_reducer: AdvancedReducer::new(editor),
            graph_: graph,
            common_: common,
            dead_: dead_node,
            zone_: temp_zone,
        }
        }
    }

    pub fn reducer_name(&self) -> &str {
        "DeadCodeElimination"
    }

    pub fn reduce(&mut self, node: *mut Node) -> Reduction {
        unsafe {
        match (*node).opcode() {
            IrOpcode::kEnd => self.reduce_end(node),
            IrOpcode::kLoop | IrOpcode::kMerge => self.reduce_loop_or_merge(node),
            IrOpcode::kLoopExit => self.reduce_loop_exit(node),
            IrOpcode::kUnreachable | IrOpcode::kIfException => {
                self.reduce_unreachable_or_if_exception(node)
            }
            IrOpcode::kPhi => self.reduce_phi(node),
            IrOpcode::kEffectPhi => self.reduce_effect_phi(node),
            IrOpcode::kDeoptimize | IrOpcode::kReturn | IrOpcode::kTerminate | IrOpcode::kTailCall =>
                self.reduce_deoptimize_or_return_or_terminate_or_tail_call(node),
            IrOpcode::kThrow => self.propagate_dead_control(node),
            IrOpcode::kBranch | IrOpcode::kSwitch => self.reduce_branch_or_switch(node),
            _ => self.reduce_node(node),
        }
        }
    }

    fn propagate_dead_control(&mut self, node: *mut Node) -> Reduction {
        unsafe {
        if (*(*node).op()).ControlInputCount() != 1 {
            panic!("DCHECK_EQ(1, node->op()->ControlInputCount()) failed");
        }
        let control = NodeProperties::GetControlInput(node);
        if (*control).opcode() == IrOpcode::kDead {
            return Reduction::Replace(control);
        }
        Reduction::NoChange
        }
    }

    fn reduce_end(&mut self, node: *mut Node) -> Reduction {
        unsafe {
        if (*node).opcode() != IrOpcode::kEnd {
            panic!("DCHECK_EQ(IrOpcode::kEnd, node->opcode()) failed");
        }
        let inputs = (*node).inputs();
        if inputs.count() < 1 {
            panic!("DCHECK_LE(1, inputs.count()) failed");
        }
        let mut live_input_count = 0;
        for i in 0..inputs.count() {
            let input = inputs[i];
            if (*input).opcode() == IrOpcode::kDead {
                continue;
            }
            if i != live_input_count {
                (*node).ReplaceInput(live_input_count, input);
            }
            live_input_count += 1;
        }

        if live_input_count == 0 {
            return Reduction::Replace(self.dead());
        } else if live_input_count < inputs.count() {
            (*node).TrimInputCount(live_input_count);
            NodeProperties::ChangeOp(node, (*self.common()).End(live_input_count));
            return Reduction::Changed(node);
        }
        if inputs.count() != live_input_count {
            panic!("DCHECK_EQ(inputs.count(), live_input_count) failed");
        }
        Reduction::NoChange
        }
    }

    fn reduce_loop_or_merge(&mut self, node: *mut Node) -> Reduction {
        unsafe {
        if !IrOpcode::IsMergeOpcode((*node).opcode()) {
            panic!("DCHECK(IrOpcode::IsMergeOpcode(node->opcode())) failed");
        }

        let inputs = (*node).inputs();
        if inputs.count() < 1 {
            panic!("DCHECK_LE(1, inputs.count()) failed");
        }

        let mut live_input_count = 0;
        if (*node).opcode() != IrOpcode::kLoop || (*(*node).InputAt(0)).opcode() != IrOpcode::kDead {
            for i in 0..inputs.count() {
                let input = inputs[i];
                if (*input).opcode() == IrOpcode::kDead {
                    continue;
                }
                if live_input_count != i {
                    (*node).ReplaceInput(live_input_count, input);
                    for use in (*node).uses() {
                        if NodeProperties::IsPhi(use) {
                            if inputs.count() + 1 != (*use).InputCount() {
                                panic!("DCHECK_EQ(inputs.count() + 1, use->InputCount()) failed");
                            }
                            (*use).ReplaceInput(live_input_count, (*use).InputAt(i));
                        }
                    }
                }
                live_input_count += 1;
            }
        }

        if live_input_count == 0 {
            return Reduction::Replace(self.dead());
        } else if live_input_count == 1 {
            let mut loop_exits: Vec<*mut Node> = Vec::new();
            for use in (*node).uses() {
                if NodeProperties::IsPhi(use) {
                    Reduction::Replace(use, (*use).InputAt(0));
                } else if (*use).opcode() == IrOpcode::kLoopExit && (*use).InputAt(1) == node {
                    loop_exits.push(use);
                } else if (*use).opcode() == IrOpcode::kTerminate {
                    if (*node).opcode() != IrOpcode::kLoop {
                        panic!("DCHECK_EQ(IrOpcode::kLoop, node->opcode()) failed");
                    }
                    Reduction::Replace(use, self.dead());
                }
            }

            for loop_exit in loop_exits {
                (*loop_exit).ReplaceInput(1, self.dead());
                self.advanced_reducer.Revisit(loop_exit);
            }
            return Reduction::Replace((*node).InputAt(0));
        }

        if live_input_count < 2 {
            panic!("DCHECK_LE(2, live_input_count) failed");
        }
        if live_input_count > inputs.count() {
            panic!("DCHECK_LE(live_input_count, inputs.count()) failed");
        }

        if live_input_count < inputs.count() {
            for use in (*node).uses() {
                if NodeProperties::IsPhi(use) {
                    (*use).ReplaceInput(live_input_count, node);
                    self.trim_merge_or_phi(use, live_input_count);
                    self.advanced_reducer.Revisit(use);
                }
            }
            self.trim_merge_or_phi(node, live_input_count);
            return Reduction::Changed(node);
        }
        Reduction::NoChange
        }
    }

    fn remove_loop_exit(&mut self, node: *mut Node) -> Reduction {
        unsafe {
        if (*node).opcode() != IrOpcode::kLoopExit {
            panic!("DCHECK_EQ(IrOpcode::kLoopExit, node->opcode()) failed");
        }

        for use in (*node).uses() {
            if (*use).opcode() == IrOpcode::kLoopExitValue || (*use).opcode() == IrOpcode::kLoopExitEffect {
                Reduction::Replace(use, (*use).InputAt(0));
            }
        }

        let control = NodeProperties::GetControlInput(node, 0);
        Reduction::Replace(node, control);
        Reduction::Replace(control)
        }
    }

    fn reduce_node(&mut self, node: *mut Node) -> Reduction {
        unsafe {
        if IrOpcode::IsGraphTerminator((*node).opcode()) {
            panic!("DCHECK(!IrOpcode::IsGraphTerminator(node->opcode())) failed");
        }
        let effect_input_count = (*(*node).op()).EffectInputCount();
        let control_input_count = (*(*node).op()).ControlInputCount();
        if control_input_count > 1 {
            panic!("DCHECK_LE(control_input_count, 1) failed");
        }

        if control_input_count == 1 {
            let reduction = self.propagate_dead_control(node);
            if reduction.Changed() {
                return reduction;
            }
        }

        if effect_input_count == 0 && (control_input_count == 0 || (*(*node).op()).ControlOutputCount() == 0) {
            return self.reduce_pure_node(node);
        }

        if effect_input_count > 0 {
            return self.reduce_effect_node(node);
        }

        Reduction::NoChange
        }
    }

    fn reduce_phi(&mut self, node: *mut Node) -> Reduction {
        unsafe {
        if (*node).opcode() != IrOpcode::kPhi {
            panic!("DCHECK_EQ(IrOpcode::kPhi, node->opcode()) failed");
        }
        let reduction = self.propagate_dead_control(node);
        if reduction.Changed() {
            return reduction;
        }
        let rep = PhiRepresentationOf((*node).op());
        if rep == MachineRepresentation::kNone || NodeProperties::GetTypeOrAny(node).IsNone() {
            return Reduction::Replace(self.dead_value(node, rep));
        }

        let input_count = (*(*node).op()).ValueInputCount();
        for i in 0..input_count {
            let input = NodeProperties::GetValueInput(node, i);
            if (*input).opcode() == IrOpcode::kDeadValue && DeadValueRepresentationOf((*input).op()) != rep {
                NodeProperties::ReplaceValueInput(node, self.dead_value(input, rep), i);
            }
        }

        Reduction::NoChange
        }
    }

    fn reduce_effect_phi(&mut self, node: *mut Node) -> Reduction {
        unsafe {
        if (*node).opcode() != IrOpcode::kEffectPhi {
            panic!("DCHECK_EQ(IrOpcode::kEffectPhi, node->opcode()) failed");
        }
        let reduction = self.propagate_dead_control(node);
        if reduction.Changed() {
            return reduction;
        }

        let merge = NodeProperties::GetControlInput(node);
        if (*merge).opcode() != IrOpcode::kMerge && (*merge).opcode() != IrOpcode::kLoop {
            panic!("DCHECK(merge->opcode() == IrOpcode::kMerge || merge->opcode() == IrOpcode::kLoop) failed");
        }

        let input_count = (*(*node).op()).EffectInputCount();
        for i in 0..input_count {
            let effect = NodeProperties::GetEffectInput(node, i);
            if (*effect).opcode() == IrOpcode::kUnreachable {
                let control = NodeProperties::GetControlInput(merge, i);
                let throw_node = (*self.graph_).NewNode((*self.common_).Throw(), effect, control);
                self.merge_control_to_end(throw_node);
                NodeProperties::ReplaceEffectInput(node, self.dead_, i);
                NodeProperties::ReplaceControlInput(merge, self.dead_, i);
                self.advanced_reducer.Revisit(merge);
                reduction = Reduction::Changed(node);
                return reduction;
            }
        }

        Reduction::NoChange
        }
    }

    fn reduce_pure_node(&mut self, node: *mut Node) -> Reduction {
        unsafe {
        if (*(*node).op()).EffectInputCount() != 0 {
            panic!("DCHECK_EQ(0, node->op()->EffectInputCount()) failed");
        }
        if (*node).opcode() == IrOpcode::kDeadValue {
            return Reduction::NoChange;
        }
        if let Some(input) = self.find_dead_input(node) {
            return Reduction::Replace(self.dead_value(input));
        }
        Reduction::NoChange
        }
    }

    fn reduce_unreachable_or_if_exception(&mut self, node: *mut Node) -> Reduction {
        unsafe {
        if (*node).opcode() != IrOpcode::kUnreachable && (*node).opcode() != IrOpcode::kIfException {
            panic!("DCHECK(node->opcode() == IrOpcode::kUnreachable || node->opcode() == IrOpcode::kIfException) failed");
        }
        let reduction = self.propagate_dead_control(node);
        if reduction.Changed() {
            return reduction;
        }

        let effect = NodeProperties::GetEffectInput(node, 0);
        if (*effect).opcode() == IrOpcode::kDead {
            return Reduction::Replace(effect);
        }
        if (*effect).opcode() == IrOpcode::kUnreachable {
            return Reduction::Replace(effect);
        }

        Reduction::NoChange
        }
    }

    fn reduce_effect_node(&mut self, node: *mut Node) -> Reduction {
        unsafe {
        if (*(*node).op()).EffectInputCount() != 1 {
            panic!("DCHECK_EQ(1, node->op()->EffectInputCount()) failed");
        }
        let effect = NodeProperties::GetEffectInput(node, 0);
        if (*effect).opcode() == IrOpcode::kDead {
            return Reduction::Replace(effect);
        }

        if let Some(input) = self.find_dead_input(node) {
            if (*effect).opcode() == IrOpcode::kUnreachable {
                self.relax_effects_and_controls(node);
                return Reduction::Replace(self.dead_value(input));
            }

            let control = if (*(*node).op()).ControlInputCount() == 1 {
                NodeProperties::GetControlInput(node, 0)
            } else {
                (*self.graph_).start()
            };
            let unreachable = (*self.graph_).NewNode((*self.common_).Unreachable(), effect, control);
            NodeProperties::SetType(unreachable, Type::None());
            self.replace_with_value(node, self.dead_value(input), node, control);
            return Reduction::Replace(unreachable);
        }

        Reduction::NoChange
        }
    }

    fn reduce_deoptimize_or_return_or_terminate_or_tail_call(&mut self, node: *mut Node) -> Reduction {
        unsafe {
        if (*node).opcode() != IrOpcode::kDeoptimize
            && (*node).opcode() != IrOpcode::kReturn
            && (*node).opcode() != IrOpcode::kTerminate
            && (*node).opcode() != IrOpcode::kTailCall
        {
            panic!("DCHECK(node->opcode() == IrOpcode::kDeoptimize || node->opcode() == IrOpcode::kReturn || node->opcode() == IrOpcode::kTerminate || node->opcode() == IrOpcode::kTailCall) failed");
        }

        let reduction = self.propagate_dead_control(node);
        if reduction.Changed() {
            return reduction;
        }

        if (*node).opcode() != IrOpcode::kTerminate && self.find_dead_input(node).is_some() {
            let effect = NodeProperties::GetEffectInput(node, 0);
            let control = NodeProperties::GetControlInput(node, 0);
            let mut effect = effect;
            if (*effect).opcode() != IrOpcode::kUnreachable {
                effect = (*self.graph_).NewNode((*self.common_).Unreachable(), effect, control);
                NodeProperties::SetType(effect, Type::None());
            }

            (*node).TrimInputCount(2);
            (*node).ReplaceInput(0, effect);
            (*node).ReplaceInput(1, control);
            NodeProperties::ChangeOp(node, (*self.common_).Throw());
            return Reduction::Changed(node);
        }
        Reduction::NoChange
        }
    }

    fn reduce_loop_exit(&mut self, node: *mut Node) -> Reduction {
        unsafe {
        let control = NodeProperties::GetControlInput(node, 0);
        let loop_node = NodeProperties::GetControlInput(node, 1);
        if (*control).opcode() == IrOpcode::kDead || (*loop_node).opcode() == IrOpcode::kDead {
            return self.remove_loop_exit(node);
        }
        Reduction::NoChange
        }
    }

    fn reduce_branch_or_switch(&mut self, node: *mut Node) -> Reduction {
        unsafe {
        if (*node).opcode() != IrOpcode::kBranch && (*node).opcode() != IrOpcode::kSwitch {
            panic!("DCHECK(node->opcode() == IrOpcode::kBranch || node->opcode() == IrOpcode::kSwitch) failed");
        }

        let reduction = self.propagate_dead_control(node);
        if reduction.Changed() {
            return reduction;
        }

        let condition = NodeProperties::GetValueInput(node, 0);
        if (*condition).opcode() == IrOpcode::kDeadValue {
            let projection_cnt = (*(*node).op()).ControlOutputCount();
            let mut projections: Vec<*mut Node> = Vec::new();
            projections.resize(projection_cnt, std::ptr::null_mut());
            NodeProperties::CollectControlProjections(node, projections.as_mut_ptr(), projection_cnt);

            Reduction::Replace(projections[0], NodeProperties::GetControlInput(node));
            return Reduction::Replace(self.dead());
        }

        Reduction::NoChange
        }
    }

    fn trim_merge_or_phi(&mut self, node: *mut Node, size: i32) {
        unsafe {
        let op = (*self.common_).ResizeMergeOrPhi((*node).op(), size as usize);
        (*node).TrimInputCount(OperatorProperties::GetTotalInputCount(op));
        NodeProperties::ChangeOp(node, op);
        }
    }

    fn dead_value(&mut self, node: *mut Node, rep: MachineRepresentation) -> *mut Node {
        unsafe {
        let mut node = node;
        if (*node).opcode() == IrOpcode::kDeadValue {
            if DeadValueRepresentationOf((*node).op()) == rep {
                return node;
            }
            node = NodeProperties::GetValueInput(node, 0);
        }
        let dead_value = (*self.graph_).NewNode((*self.common_).DeadValue(rep), node);
        NodeProperties::SetType(dead_value, Type::None());
        dead_value
        }
    }

    fn graph(&self) -> *mut TFGraph {
        self.graph_
    }

    fn common(&self) -> *mut CommonOperatorBuilder {
        self.common_
    }

    fn dead(&self) -> *mut Node {
        self.dead_
    }

    fn find_dead_input(&self, node: *mut Node) -> Option<*mut Node> {
        unsafe {
        for input in (*node).inputs() {
            if self.no_return(input) {
                return Some(input);
            }
        }
        None
        }
    }

    fn no_return(&self, node: *mut Node) -> bool {
        unsafe {
         (*node).opcode() == IrOpcode::kDead
            || (*node).opcode() == IrOpcode::kUnreachable
            || (*node).opcode() == IrOpcode::kDeadValue
            || NodeProperties::GetTypeOrAny(node).IsNone()
        }
    }

    fn relax_effects_and_controls(&self, _node: *mut Node) {}

    fn replace_with_value(&self, _node: *mut Node, _dead_value: *mut Node, _node1: *mut Node, _control: *mut Node) {}

    fn merge_control_to_end(&mut self, throw_node: *mut Node) {
        unsafe {
        MergeControlToEnd(self.graph_, self.common_, throw_node);
        }
    }
}

pub struct NodeVector {
    nodes: Vec<*mut Node>,
}

impl NodeVector {
    pub fn new(zone: *mut Zone) -> NodeVector {
        NodeVector { nodes: Vec::new() }
    }

    pub fn push_back(&mut self, node: *mut Node) {
        self.nodes.push(node);
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum IrOpcode {
    kDead,
    kEnd,
    kLoop,
    kMerge,
    kLoopExit,
    kUnreachable,
    kIfException,
    kPhi,
    kEffectPhi,
    kDeoptimize,
    kReturn,
    kTerminate,
    kTailCall,
    kThrow,
    kBranch,
    kSwitch,
    kDeadValue,
    kLoopExitValue,
    kLoopExitEffect,
}

impl IrOpcode {
    pub fn IsMergeOpcode(op: IrOpcode) -> bool {
        op == IrOpcode::kLoop || op == IrOpcode::kMerge
    }
     pub fn IsGraphTerminator(op: IrOpcode) -> bool {
        op == IrOpcode::kEnd || op == IrOpcode::kReturn || op == IrOpcode::kTerminate || op == IrOpcode::kThrow || op == IrOpcode::kDeoptimize
    }
}

pub struct Inputs {
    inputs: Vec<*mut Node>,
}

impl Inputs {
    pub fn count(&self) -> usize {
        self.inputs.len()
    }

    pub fn get(&self, index: usize) -> *mut Node {
        self.inputs[index]
    }
}

pub struct Uses {
    uses: Vec<*mut Node>,
}

impl Uses {
    pub fn iter(&self) -> std::slice::Iter<*mut Node> {
        self.uses.iter()
    }
}

pub enum Type {
    NoneValue,
    Any,
}

impl Type {
    pub fn IsNone(&self) -> bool {
        match self {
            Type::NoneValue => true,
            _ => false,
        }
    }
}

pub fn PhiRepresentationOf(_op: &Operator) -> MachineRepresentation {
    MachineRepresentation::kNone
}

pub fn DeadValueRepresentationOf(_op: &Operator) -> MachineRepresentation {
    MachineRepresentation::kNone
}

pub fn MergeControlToEnd(graph_: *mut TFGraph, common_: *mut CommonOperatorBuilder, throw_node: *mut Node){
    unsafe{
    }
}

pub struct Editor{}

impl Editor {
    pub fn Revisit(&mut self, _node: *mut Node){
    }
}
}
