// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::sync::atomic::{AtomicBool, Ordering};
use std::collections::HashMap;

/// Represents the state of an observable node.
pub struct ObservableNodeState<'a> {
    id: u32,
    op: &'a Operator,
    type_: Type,
}

impl<'a> ObservableNodeState<'a> {
    /// Creates a new `ObservableNodeState`.
    pub fn new(node: &Node, zone: &Zone) -> Self {
        ObservableNodeState {
            id: node.id, // Assuming Node has a public field 'id'
            op: &node.op, // Assuming Node has a public field 'op' of type Operator
            type_: node.type_, // Assuming Node has a public field 'type_' of type Type
        }
    }

    /// Returns the ID of the node.
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Returns the operator of the node.
    pub fn op(&self) -> &Operator {
        self.op
    }

    /// Returns the opcode of the operator.
    pub fn opcode(&self) -> i16 {
        self.op.opcode()
    }

    /// Returns the type of the node.
    pub fn type_(&self) -> Type {
        self.type_
    }
}

impl PartialEq for ObservableNodeState<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id() && self.op() == other.op() && self.type_() == other.type_()
    }
}

impl Eq for ObservableNodeState<'_> {}

impl std::fmt::Debug for ObservableNodeState<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ObservableNodeState")
            .field("id", &self.id)
            .field("op", &self.op.opcode()) // Assuming Operator implements Debug or has a way to represent it
            .field("type_", &self.type_) // Assuming Type implements Debug
            .finish()
    }
}

/// Represents an observer for nodes in the graph.
pub trait NodeObserver {
    /// Represents the result of an observation.
    fn on_node_created(&mut self, node: &Node) -> Observation {
        Observation::Continue
    }

    /// Called when a node is changed.
    fn on_node_changed(
        &mut self,
        reducer_name: &str,
        node: &Node,
        old_state: &ObservableNodeState,
    ) -> Observation {
        Observation::Continue
    }

    /// Sets the flag indicating that changes have been observed.
    fn set_has_observed_changes(&mut self);

    /// Returns whether changes have been observed.
    fn has_observed_changes(&self) -> bool;
}

pub enum Observation {
    Continue,
    Stop,
}

pub trait ZoneObject {} // Add blanket implementation for all types inside zone if needed

pub struct DefaultNodeObserver {
    has_observed_changes: AtomicBool,
}

impl DefaultNodeObserver {
    pub fn new() -> Self {
        DefaultNodeObserver {
            has_observed_changes: AtomicBool::new(false),
        }
    }
}

impl NodeObserver for DefaultNodeObserver {
    fn set_has_observed_changes(&mut self) {
        self.has_observed_changes.store(true, Ordering::Relaxed);
    }
    fn has_observed_changes(&self) -> bool {
        self.has_observed_changes.load(Ordering::Relaxed)
    }
}

impl ZoneObject for DefaultNodeObserver {}

/// Stores an observation for a node.
#[derive(Debug)]
pub struct NodeObservation<'a> {
    observer: Box<dyn NodeObserver + 'a>,
    state: ObservableNodeState<'a>,
}

impl<'a> NodeObservation<'a> {
    /// Creates a new `NodeObservation`.
    pub fn new(node_observer: Box<dyn NodeObserver + 'a>, node: &Node, zone: &Zone) -> Self {
        NodeObservation {
            observer: node_observer,
            state: ObservableNodeState::new(node, zone),
        }
    }
}

type NodeId = u32; // Assuming NodeId is a u32

/// Manages node observations.
pub struct ObserveNodeManager<'a> {
    zone: &'a Zone,
    observations: HashMap<NodeId, NodeObservation<'a>>,
}

impl<'a> ObserveNodeManager<'a> {
    /// Creates a new `ObserveNodeManager`.
    pub fn new(zone: &'a Zone) -> Self {
        ObserveNodeManager {
            zone,
            observations: HashMap::new(),
        }
    }

    /// Starts observing a node.
    pub fn start_observing(&mut self, node: &mut Node, observer: Box<dyn NodeObserver + 'a>) {
        let observation = NodeObservation::new(observer, node, self.zone);
        self.observations.insert(node.id, observation); // Assuming Node has a public field 'id'
    }

    /// Called when a node is changed.
    pub fn on_node_changed(&mut self, reducer_name: &str, old_node: &Node, new_node: &Node) {
        if let Some(observation) = self.observations.get_mut(&old_node.id) { // Assuming Node has a public field 'id'
            let old_state = &observation.state;
            observation.observer.on_node_changed(reducer_name, new_node, old_state);
        }
    }
}

impl<'a> ZoneObject for ObserveNodeManager<'a> {}

/// Stores information about node observation.
pub struct ObserveNodeInfo<'a> {
    observe_node_manager: Option<&'a mut ObserveNodeManager<'a>>,
    node_observer: Option<Box<dyn NodeObserver + 'a>>,
}

impl<'a> ObserveNodeInfo<'a> {
    /// Creates a new `ObserveNodeInfo`.
    pub fn new() -> Self {
        ObserveNodeInfo {
            observe_node_manager: None,
            node_observer: None,
        }
    }

    /// Creates a new `ObserveNodeInfo` with a manager and observer.
    pub fn with_manager_and_observer(manager: &'a mut ObserveNodeManager<'a>, observer: Box<dyn NodeObserver + 'a>) -> Self {
        ObserveNodeInfo {
            observe_node_manager: Some(manager),
            node_observer: Some(observer),
        }
    }

    /// Starts observing a node.
    pub fn start_observing(&mut self, node: &mut Node) {
        if let Some(manager) = self.observe_node_manager.as_mut() {
            if let Some(observer) = self.node_observer.take() {
                manager.start_observing(node, observer);
                //Store the observer back so that we can observe new nodes
                //This also prevents observer from being dropped at the end of this function
                self.node_observer = Some(manager.observations.get_mut(&node.id).unwrap().observer); // Assuming Node has a public field 'id'
            }
        }
    }
}

/// Represents a node in the graph.
#[derive(Debug)]
pub struct Node {
    id: NodeId,
    op: Operator,
    type_: Type,
}

impl Node {
    pub fn new(id: NodeId, op: Operator, type_: Type) -> Self {
        Node { id, op, type_ }
    }
}

/// Represents an operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Operator {
    opcode: i16,
}

impl Operator {
    pub fn new(opcode: i16) -> Self {
        Operator { opcode }
    }
    pub fn opcode(&self) -> i16 {
        self.opcode
    }
}

/// Represents a type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Type {
    kind: i32,
}

impl Type {
    pub fn new(kind: i32) -> Self {
        Type { kind }
    }
}

/// Represents a zone.
pub struct Zone {}

impl Zone {
    pub fn new() -> Self {
        Zone {}
    }
}