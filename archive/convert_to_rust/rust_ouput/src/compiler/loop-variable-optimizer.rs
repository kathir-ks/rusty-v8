// Converted from V8 C++ source files:
// Header: loop-variable-optimizer.h
// Implementation: loop-variable-optimizer.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use std::ptr::null_mut;
use std::rc::Rc;

struct ZoneObject {}

struct Node {
    id: i32,
    opcode: IrOpcode,
}

impl Node {
    fn new(id: i32, opcode: IrOpcode) -> Node {
        Node { id, opcode }
    }
    fn id(&self) -> i32 {
        self.id
    }
    fn opcode(&self) -> IrOpcode {
        self.opcode
    }
    fn input_at(&self, index: usize) -> *mut Node {
        null_mut()
    }
    fn insert_input(&mut self, zone: &Zone, index: usize, node: *mut Node) {
    }
    fn input_count(&self) -> usize {
        0
    }
    fn trim_input_count(&mut self, count: usize) {}
    fn replace_input(&mut self, index: usize, node: *mut Node) {}
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node(id: {})", self.id)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum IrOpcode {
    kMerge,
    kLoop,
    kIfFalse,
    kIfTrue,
    kStart,
    kLoopExit,
    kJSLessThan,
    kNumberLessThan,
    kSpeculativeNumberLessThan,
    kJSGreaterThan,
    kJSLessThanOrEqual,
    kNumberLessThanOrEqual,
    kSpeculativeNumberLessThanOrEqual,
    kJSGreaterThanOrEqual,
    kPhi,
    kJSAdd,
    kNumberAdd,
    kSpeculativeNumberAdd,
    kSpeculativeAdditiveSafeIntegerAdd,
    kSpeculativeAdditiveSafeIntegerSubtract,
    kSpeculativeSmallIntegerAdd,
    kJSSubtract,
    kNumberSubtract,
    kSpeculativeNumberSubtract,
    kSpeculativeSmallIntegerSubtract,
    kSpeculativeToNumber,
    kJSToNumber,
    kJSToNumberConvertBigInt,
    kEffectPhi,
    kTypeGuard,
    kInductionVariablePhi,
}

struct Edge {}
impl Edge {
    fn from(&self) -> *mut Node {
        null_mut()
    }
    fn index(&self) -> i32 {
        0
    }
}

struct Inputs {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MachineRepresentation {
    kTagged,
}

struct Operator {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CallDescriptorKind {}

struct Type {}
impl Type {
    fn is(&self, other: Type) -> bool {
        true
    }
}

struct TFGraph {
    node_count: i32,
    start_node: *mut Node,
}

impl TFGraph {
    fn new(node_count: i32, start_node: *mut Node) -> TFGraph {
        TFGraph {
            node_count,
            start_node,
        }
    }

    fn node_count(&self) -> i32 {
        self.node_count
    }
    fn start(&self) -> *mut Node {
        self.start_node
    }
    fn new_node(&self, operator: &Operator, n1: *mut Node, n2: *mut Node, n3: *mut Node) -> *mut Node {
        null_mut()
    }
    fn zone(&self) -> &Zone {
        unsafe { &*(0 as *const Zone) }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum LoopVariableOptimizerError {}

struct CommonOperatorBuilder {}
impl CommonOperatorBuilder {
    fn induction_variable_phi(&self, input_count: usize) -> Operator {
        Operator {}
    }
    fn phi(&self, rep: MachineRepresentation, value_count: i32) -> Operator {
        Operator {}
    }
    fn type_guard(&self, phi_type: Type) -> Operator {
        Operator {}
    }
}

struct Zone {
    name: String,
}

impl Zone {
    fn new(name: String) -> Zone {
        Zone { name }
    }
    fn allocate<T>(&self, value: T) -> Box<T> {
        Box::new(value)
    }
}

impl fmt::Display for Zone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Zone(name: {})", self.name)
    }
}

struct NodeAuxData<T> {
    data: Vec<T>,
    zone: *mut Zone,
}

impl<T: Clone> NodeAuxData<T> {
    fn new(size: i32, zone: *mut Zone, default_value: T) -> NodeAuxData<T> {
        NodeAuxData {
            data: vec![default_value; size as usize],
            zone,
        }
    }

    fn get(&self, node: *mut Node) -> T {
        unsafe {
            self.data[( (*node).id() as usize)].clone()
        }
    }

    fn set(&mut self, node: *mut Node, value: T) {
        unsafe {
            self.data[( (*node).id() as usize)] = value;
        }
    }
}

struct NodeProperties {}
impl NodeProperties {
    fn get_control_input(node: *mut Node, index: i32) -> *mut Node {
        null_mut()
    }
    fn is_control_edge(edge: Edge) -> bool {
        true
    }
    fn get_effect_input(effect_phi: *mut Node, index: i32) -> *mut Node {
        null_mut()
    }
    fn change_op(node: *mut Node, op: Operator) {}
    fn get_type(backedge_value: *mut Node) -> Type {
        Type {}
    }
}

struct ZoneQueue<T> {
    queue: Vec<T>,
    zone: *mut Zone,
}

impl<T> ZoneQueue<T> {
    fn new(zone: *mut Zone) -> ZoneQueue<T> {
        ZoneQueue {
            queue: Vec::new(),
            zone,
        }
    }

    fn push(&mut self, value: T) {
        self.queue.push(value);
    }

    fn pop(&mut self) -> Option<T> {
        self.queue.drain(0..1).next()
    }

    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

struct NodeMarker<T> {
    marker: Vec<T>,
    graph: *mut TFGraph,
}

impl<T: Copy + Clone> NodeMarker<T> {
    fn new(graph: *mut TFGraph, size: usize, initial_value: T) -> NodeMarker<T> {
        let capacity = unsafe { (*graph).node_count() as usize };
        NodeMarker {
            marker: vec![initial_value; capacity],
            graph,
        }
    }
    fn get(&self, node: *mut Node) -> T {
        unsafe {
            self.marker[((*node).id() as usize)]
        }
    }
    fn set(&mut self, node: *mut Node, value: T) {
        unsafe {
            self.marker[((*node).id() as usize)] = value;
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ConstraintKind {
    kStrict,
    kNonStrict,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Bound {
    bound: *mut Node,
    kind: ConstraintKind,
}

impl Bound {
    fn new(bound: *mut Node, kind: ConstraintKind) -> Bound {
        Bound { bound, kind }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum ArithmeticType {
    kAddition,
    kSubtraction,
}

struct InductionVariable {
    phi_: *mut Node,
    effect_phi_: *mut Node,
    arith_: *mut Node,
    increment_: *mut Node,
    init_value_: *mut Node,
    lower_bounds_: Vec<Bound>,
    upper_bounds_: Vec<Bound>,
    arithmeticType_: ArithmeticType,
    zone: *mut Zone,
}

impl InductionVariable {
    fn new(
        phi: *mut Node,
        effect_phi: *mut Node,
        arith: *mut Node,
        increment: *mut Node,
        init_value: *mut Node,
        zone: *mut Zone,
        arithmetic_type: ArithmeticType,
    ) -> InductionVariable {
        InductionVariable {
            phi_: phi,
            effect_phi_: effect_phi,
            arith_: arith,
            increment_: increment,
            init_value_: init_value,
            lower_bounds_: Vec::new(),
            upper_bounds_: Vec::new(),
            arithmeticType_: arithmetic_type,
            zone: zone,
        }
    }

    fn phi(&self) -> *mut Node {
        self.phi_
    }
    fn effect_phi(&self) -> *mut Node {
        self.effect_phi_
    }
    fn arith(&self) -> *mut Node {
        self.arith_
    }
    fn increment(&self) -> *mut Node {
        self.increment_
    }
    fn init_value(&self) -> *mut Node {
        self.init_value_
    }
    fn lower_bounds(&self) -> &Vec<Bound> {
        &self.lower_bounds_
    }
    fn upper_bounds(&self) -> &Vec<Bound> {
        &self.upper_bounds_
    }
    fn add_upper_bound(&mut self, bound: *mut Node, kind: ConstraintKind) {
        self.upper_bounds_.push(Bound::new(bound, kind));
    }
    fn add_lower_bound(&mut self, bound: *mut Node, kind: ConstraintKind) {
        self.lower_bounds_.push(Bound::new(bound, kind));
    }
    fn arithmetic_type(&self) -> ArithmeticType {
        self.arithmeticType_
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Constraint {
    left: *mut Node,
    kind: ConstraintKind,
    right: *mut Node,
}

type VariableLimits = Vec<Constraint>;

struct LoopVariableOptimizer {
    graph_: *mut TFGraph,
    common_: *mut CommonOperatorBuilder,
    zone_: *mut Zone,
    limits_: NodeAuxData<VariableLimits>,
    reduced_: NodeAuxData<bool>,
    induction_vars_: HashMap<i32, *mut InductionVariable>,
}

impl LoopVariableOptimizer {
    fn new(graph: *mut TFGraph, common: *mut CommonOperatorBuilder, zone: *mut Zone) -> LoopVariableOptimizer {
        let node_count = unsafe { (*graph).node_count() };
        LoopVariableOptimizer {
            graph_: graph,
            common_: common,
            zone_: zone,
            limits_: NodeAuxData::new(node_count, zone, Vec::new()),
            reduced_: NodeAuxData::new(node_count, zone, false),
            induction_vars_: HashMap::new(),
        }
    }

    fn run(&mut self) {
        let zone = self.zone_;
        let mut queue = ZoneQueue::new(zone);
        queue.push(unsafe { (*self.graph_).start() });

        let mut queued = NodeMarker::new(self.graph_, 2, false);

        while !queue.empty() {
            let node = queue.pop().unwrap();
            queued.set(node, false);
            if unsafe { self.reduced_.get(node) } {
                continue;
            }
            let all_inputs_visited = (0..self.get_inputs_end(node))
                .all(|i| unsafe { self.reduced_.get(NodeProperties::get_control_input(node, i)) });
            if !all_inputs_visited {
                continue;
            }

            self.visit_node(node);
            self.reduced_.set(node, true);

            for edge in self.get_use_edges(node) {
                if NodeProperties::is_control_edge(edge) && unsafe { (*(*edge.from()).op()).control_output_count() > 0 } {
                    let use = unsafe { edge.from() };
                    if unsafe { (*use).opcode() } == IrOpcode::kLoop && edge.index() != 0 {
                        self.visit_backedge(node, use);
                    } else if !queued.get(use) {
                        queue.push(use);
                        queued.set(use, true);
                    }
                }
            }
        }
    }
    fn induction_variables(&self) -> &HashMap<i32, *mut InductionVariable> {
        &self.induction_vars_
    }

    fn change_to_induction_variable_phis(&mut self) {
        for (&_, &induction_var_ptr) in &self.induction_vars_ {
            let induction_var = unsafe { &*induction_var_ptr };

            if induction_var.upper_bounds().is_empty() && induction_var.lower_bounds().is_empty() {
                continue;
            }

            let phi = unsafe { &mut *induction_var.phi() };
            let increment = induction_var.increment();
            phi.insert_input(unsafe { &*self.zone_ }, phi.input_count() - 1, increment);

            for bound in induction_var.lower_bounds() {
                phi.insert_input(unsafe { &*self.zone_ }, phi.input_count() - 1, bound.bound);
            }

            for bound in induction_var.upper_bounds() {
                phi.insert_input(unsafe { &*self.zone_ }, phi.input_count() - 1, bound.bound);
            }

            NodeProperties::change_op(
                induction_var.phi(),
                unsafe { (*self.common_).induction_variable_phi(phi.input_count() - 1) },
            );
        }
    }

    fn change_to_phis_and_insert_guards(&mut self) {
        for (&_, &induction_var_ptr) in &self.induction_vars_ {
            let induction_var = unsafe { &mut *induction_var_ptr };

            if unsafe { (*induction_var.phi()).opcode() } == IrOpcode::kInductionVariablePhi {
                let phi = unsafe { &mut *induction_var.phi() };

                let value_count = 2;
                let control = NodeProperties::get_control_input(induction_var.phi(), 0);
                phi.trim_input_count(value_count + 1);
                phi.replace_input(value_count, control);
                NodeProperties::change_op(
                    induction_var.phi(),
                    unsafe { (*self.common_).phi(MachineRepresentation::kTagged, value_count as i32) },
                );

                let backedge_value = unsafe { (*induction_var.phi()).input_at(1) };
                let backedge_type = NodeProperties::get_type(backedge_value);
                let phi_type = NodeProperties::get_type(induction_var.phi());

                if !backedge_type.is(phi_type) {
                    let loop_node = NodeProperties::get_control_input(induction_var.phi(), 0);
                    let backedge_control = unsafe { (*loop_node).input_at(1) };
                    let backedge_effect = NodeProperties::get_effect_input(induction_var.effect_phi(), 1);

                    let rename = unsafe {
                        (*self.graph_).new_node(
                            &(*self.common_).type_guard(phi_type),
                            backedge_value,
                            backedge_effect,
                            backedge_control,
                        )
                    };
                    unsafe { (*induction_var.effect_phi()).replace_input(1, rename) };
                    phi.replace_input(1, rename);
                }
            }
        }
    }

    fn visit_backedge(&mut self, from: *mut Node, loop_node: *mut Node) {
        if unsafe { (*loop_node).opcode() } == IrOpcode::kLoop && unsafe { (*(*loop_node).op()).control_input_count() != 2 } {
            return;
        }

        let limits = unsafe { self.limits_.get(from) };
        for constraint in limits {
            if unsafe { (*constraint.left).opcode() } == IrOpcode::kPhi
                && NodeProperties::get_control_input(constraint.left, 0) == loop_node
            {
                if let Some(&var) = self.induction_vars_.get(&(unsafe { (*constraint.left).id() })) {
                    unsafe {
                        (&mut *var).add_upper_bound(constraint.right, constraint.kind);
                    }
                }
            }
            if unsafe { (*constraint.right).opcode() } == IrOpcode::kPhi
                && NodeProperties::get_control_input(constraint.right, 0) == loop_node
            {
                if let Some(&var) = self.induction_vars_.get(&(unsafe { (*constraint.right).id() })) {
                    unsafe {
                        (&mut *var).add_lower_bound(constraint.left, constraint.kind);
                    }
                }
            }
        }
    }

    fn visit_node(&mut self, node: *mut Node) {
        match unsafe { (*node).opcode() } {
            IrOpcode::kMerge => self.visit_merge(node),
            IrOpcode::kLoop => self.visit_loop(node),
            IrOpcode::kIfFalse => self.visit_if(node, false),
            IrOpcode::kIfTrue => self.visit_if(node, true),
            IrOpcode::kStart => self.visit_start(node),
            IrOpcode::kLoopExit => self.visit_loop_exit(node),
            _ => self.visit_other_control(node),
        }
    }

    fn visit_merge(&mut self, node: *mut Node) {
        let mut merged = unsafe { self.limits_.get((*node).input_at(0)) };
        for i in 1..unsafe { (*node).input_count() } {
            let input = unsafe { (*node).input_at(i as usize) };
            self.reset_to_common_ancestor(&mut merged, unsafe { self.limits_.get(input) });
        }
        self.limits_.set(node, merged);
    }

    fn reset_to_common_ancestor(&mut self, merged: &mut VariableLimits, other: VariableLimits) {
        let mut new_merged: VariableLimits = Vec::new();
        for c1 in merged.iter() {
            if other.contains(c1) {
                new_merged.push(c1.clone());
            }
        }
        *merged = new_merged;
    }

    fn visit_loop(&mut self, node: *mut Node) {
        self.detect_induction_variables(node);
        self.take_conditions_from_first_control(node);
    }

    fn visit_if(&mut self, node: *mut Node, polarity: bool) {
        let branch = unsafe { (*node).input_at(0) };
        let cond = unsafe { (*(*branch).input_at(0)) };
        let mut limits = unsafe { self.limits_.get(branch) };

        match unsafe { (*cond).opcode() } {
            IrOpcode::kJSLessThan | IrOpcode::kNumberLessThan | IrOpcode::kSpeculativeNumberLessThan => {
                self.add_cmp_to_limits(&mut limits, cond, ConstraintKind::kStrict, polarity);
            }
            IrOpcode::kJSGreaterThan => {
                self.add_cmp_to_limits(&mut limits, cond, ConstraintKind::kNonStrict, !polarity);
            }
            IrOpcode::kJSLessThanOrEqual | IrOpcode::kNumberLessThanOrEqual
            | IrOpcode::kSpeculativeNumberLessThanOrEqual => {
                self.add_cmp_to_limits(&mut limits, cond, ConstraintKind::kNonStrict, polarity);
            }
            IrOpcode::kJSGreaterThanOrEqual => {
                self.add_cmp_to_limits(&mut limits, cond, ConstraintKind::kStrict, !polarity);
            }
            _ => {}
        }
        self.limits_.set(node, limits);
    }

    fn add_cmp_to_limits(
        &mut self,
        limits: &mut VariableLimits,
        node: *mut Node,
        kind: ConstraintKind,
        polarity: bool,
    ) {
        let left = unsafe { (*node).input_at(0) };
        let right = unsafe { (*node).input_at(1) };

        if self.find_induction_variable(left).is_some() || self.find_induction_variable(right).is_some() {
            if polarity {
                limits.push(Constraint { left, kind, right });
            } else {
                let new_kind = if kind == ConstraintKind::kStrict {
                    ConstraintKind::kNonStrict
                } else {
                    ConstraintKind::kStrict
                };
                limits.push(Constraint {
                    left: right,
                    kind: new_kind,
                    right: left,
                });
            }
        }
    }

    fn visit_start(&mut self, node: *mut Node) {
        self.limits_.set(node, Vec::new());
    }

    fn visit_loop_exit(&mut self, node: *mut Node) {
        self.take_conditions_from_first_control(node);
    }

    fn visit_other_control(&mut self, node: *mut Node) {
        self.take_conditions_from_first_control(node);
    }

    fn take_conditions_from_first_control(&mut self, node: *mut Node) {
        let control_input = NodeProperties::get_control_input(node, 0);
        let limits = unsafe { self.limits_.get(control_input) };
        self.limits_.set(node, limits);
    }

    fn find_induction_variable(&self, node: *mut Node) -> Option<*mut InductionVariable> {
        self.induction_vars_.get(&(unsafe { (*node).id() })).copied()
    }

    fn try_get_induction_variable(&mut self, phi: *mut Node) -> Option<*mut InductionVariable> {
        if unsafe { (*phi).opcode() } != IrOpcode::kPhi {
            return None;
        }
        if unsafe { (*(*phi).op()).value_input_count() != 2 } {
            return None;
        }

        let loop_node = NodeProperties::get_control_input(phi, 0);
        if unsafe { (*loop_node).opcode() } != IrOpcode::kLoop {
            return None;
        }

        let initial = unsafe { (*phi).input_at(0) };
        let arith = unsafe { (*phi).input_at(1) };

        let arithmetic_type = match unsafe { (*arith).opcode() } {
            IrOpcode::kJSAdd
            | IrOpcode::kNumberAdd
            | IrOpcode::kSpeculativeNumberAdd
            | IrOpcode::kSpeculativeAdditiveSafeIntegerAdd
            | IrOpcode::kSpeculativeAdditiveSafeIntegerSubtract
            | IrOpcode::kSpeculativeSmallIntegerAdd => ArithmeticType::kAddition,
            IrOpcode::kJSSubtract
            | IrOpcode::kNumberSubtract
            | IrOpcode::kSpeculativeNumberSubtract
            | IrOpcode::kSpeculativeSmallIntegerSubtract => ArithmeticType::kSubtraction,
            _ => return None,
        };
        let mut input = unsafe { (*arith).input_at(0) };
        if unsafe { (*input).opcode() } == IrOpcode::kSpeculativeToNumber
            || unsafe { (*input).opcode() } == IrOpcode::kJSToNumber
            || unsafe { (*input).opcode() } == IrOpcode::kJSToNumberConvertBigInt
        {
            input = unsafe { (*input).input_at(0) };
        }
        if input != phi {
            return None;
        }
        let mut effect_phi = None;
        for i in self.get_use_edges(loop_node) {
            let use = unsafe { i.from() };
            if unsafe { (*use).opcode() } == IrOpcode::kEffectPhi {
                if effect_phi.is_some() {
                    return None;
                }
                effect_phi = Some(use);
            }
        }
        if effect_phi.is_none() {
            return None;
        }
        let incr = unsafe { (*arith).input_at(1) };
        let induction_var = unsafe {
            &mut *Zone::allocate(
                &*self.zone_,
                InductionVariable::new(
                    phi,
                    effect_phi.unwrap(),
                    arith,
                    incr,
                    initial,
                    self.zone_,
                    arithmetic_type,
                ),
            )
        };
        Some(induction_var)
    }

    fn detect_induction_variables(&mut self, loop_node: *mut Node) {
        if unsafe { (*loop_node).opcode() } != IrOpcode::kLoop {
            return;
        }
        if unsafe { (*(*loop_node).op()).control_input_count() != 2 } {
            return;
        }

        for edge in self.get_use_edges(loop_node) {
            if NodeProperties::is_control_edge(edge) && unsafe { (*edge.from()).opcode() } == IrOpcode::kPhi {
                let phi = unsafe { edge.from() };
                if let Some(induction_var) = self.try_get_induction_variable(phi) {
                    self.induction_vars_.insert(unsafe { (*phi).id() }, induction_var);
                }
            }
        }
    }

    fn graph(&self) -> *mut TFGraph {
        self.graph_
    }
    fn common(&self) -> *mut CommonOperatorBuilder {
        self.common_
    }
    fn zone(&self) -> *mut Zone {
        self.zone_
    }
    fn get_inputs_end(&self, node: *mut Node) -> i32 {
        if unsafe { (*node).opcode() } == IrOpcode::kLoop {
            1
        } else {
            unsafe { (*(*node).op()).control_input_count() }
        }
    }
    fn get_use_edges(&self, node: *mut Node) -> Vec<Edge> {
        Vec::new()
    }
}
trait ControlOutputCount {
    fn control_output_count(&self) -> i32;
}

impl ControlOutputCount for Operator {
    fn control_output_count(&self) -> i32 {
        0
    }
}
trait ValueInputCount {
    fn value_input_count(&self) -> i32;
}

impl ValueInputCount for Operator {
    fn value_input_count(&self) -> i32 {
        0
    }
}
