// Converted from V8 C++ source files:
// Header: control-path-state.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod functional_list;
pub mod graph_reducer;
pub mod node_aux_data;
pub mod node_properties;
pub mod node;
pub mod persistent_map;
pub mod turbofan_graph;
pub mod zone;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Mutex;

use crate::compiler::functional_list::FunctionalList;
use crate::compiler::graph_reducer::AdvancedReducer;
use crate::compiler::node_aux_data::NodeAuxData;
use crate::compiler::node::Node;
use crate::compiler::persistent_map::PersistentMap;
use crate::compiler::turbofan_graph::Graph as TFGraph;
use crate::compiler::zone::Zone;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum NodeUniqueness {
    kUniqueInstance,
    kMultipleInstances,
}

pub trait IsSet {
    fn is_set(&self) -> bool;
}

pub trait NodeStateTrait {
    fn node(&self) -> *mut Node;
    fn is_set(&self) -> bool;
}

#[derive(Clone)]
pub struct ControlPathState<NodeState, const NODE_UNIQUENESS: NodeUniqueness>
    where
        NodeState: NodeStateTrait + Clone + PartialEq + Debug,
{
    blocks_: FunctionalList<FunctionalList<NodeState>>,
    states_: PersistentMap<(usize, usize), NodeState>,
    _phantom: PhantomData<NodeState>,
}

impl<NodeState, const NODE_UNIQUENESS: NodeUniqueness> Debug for ControlPathState<NodeState, NODE_UNIQUENESS>
    where
        NodeState: NodeStateTrait + Clone + PartialEq + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ControlPathState")
            .field("blocks_", &self.blocks_)
            .field("states_", &self.states_)
            .finish()
    }
}

impl<NodeState, const NODE_UNIQUENESS: NodeUniqueness> ControlPathState<NodeState, NODE_UNIQUENESS>
    where
        NodeState: NodeStateTrait + Clone + PartialEq + Debug,
{
    pub fn new(zone: &Zone) -> Self {
        ControlPathState {
            blocks_: FunctionalList::new(),
            states_: PersistentMap::new(),
            _phantom: PhantomData,
        }
    }

    pub fn lookup_state(&self, node: *mut Node) -> NodeState {
        if NODE_UNIQUENESS == NodeUniqueness::kUniqueInstance {
            self.states_.get(&(node as usize, 0)).cloned().unwrap_or_else(|| {
                let default_state: NodeState = unsafe { std::mem::zeroed() };
                default_state
            })
        } else {
            for depth in (1..=self.blocks_.size()).rev() {
                if let Some(state) = self.states_.get(&(node as usize, depth)) {
                    return state.clone();
                }
            }
            let default_state: NodeState = unsafe { std::mem::zeroed() };
            default_state
        }
    }

    pub fn add_state(&mut self, zone: &Zone, node: *mut Node, state: NodeState, hint: &ControlPathState<NodeState, NODE_UNIQUENESS>) {
        let previous_state = self.lookup_state(node);
        let should_add = if NODE_UNIQUENESS == NodeUniqueness::kUniqueInstance {
            !previous_state.is_set()
        } else {
            previous_state != state
        };

        if !should_add {
            return;
        }

        let mut prev_front = self.blocks_.front();
        if hint.blocks_.size() > 0 {
            prev_front = prev_front.push_front(state.clone(), zone, &hint.blocks_.front());
        } else {
            prev_front = prev_front.push_front(state.clone(), zone, &FunctionalList::new());
        }
        self.blocks_.drop_front();
        self.blocks_.push_front(prev_front, zone);

        self.states_.insert((node as usize, self.depth(self.blocks_.size())), state);

        // self.blocks_and_states_invariant();
    }

    pub fn add_state_in_new_block(&mut self, zone: &Zone, node: *mut Node, state: NodeState) {
        let mut new_block = FunctionalList::new();
        let previous_state = self.lookup_state(node);
        let should_add = if NODE_UNIQUENESS == NodeUniqueness::kUniqueInstance {
            !previous_state.is_set()
        } else {
            previous_state != state
        };

        if should_add {
            new_block = new_block.push_front(state.clone(), zone, &FunctionalList::new());
            self.states_.insert((node as usize, self.depth(self.blocks_.size() + 1)), state.clone());
        }

        self.blocks_.push_front(new_block, zone);

        //self.blocks_and_states_invariant();
    }

    pub fn reset_to_common_ancestor(&mut self, mut other: ControlPathState<NodeState, NODE_UNIQUENESS>) {
        while other.blocks_.size() > self.blocks_.size() {
            other.blocks_.drop_front();
        }

        while self.blocks_.size() > other.blocks_.size() {
            if let Some(block) = self.blocks_.front_option() {
                for state in block.iter() {
                    self.states_.remove(&(state.node() as usize, self.depth(self.blocks_.size())));
                }
            }
            self.blocks_.drop_front();
        }

        while self.blocks_ != other.blocks_ {
            if let Some(block) = self.blocks_.front_option() {
                for state in block.iter() {
                    self.states_.remove(&(state.node() as usize, self.depth(self.blocks_.size())));
                }
            }
            self.blocks_.drop_front();
            other.blocks_.drop_front();
        }
        //self.blocks_and_states_invariant();
    }

    pub fn is_empty(&self) -> bool {
        self.blocks_.size() == 0
    }

    fn depth(&self, depth_if_multiple_instances: usize) -> usize {
        if NODE_UNIQUENESS == NodeUniqueness::kMultipleInstances {
            depth_if_multiple_instances
        } else {
            0
        }
    }

    // #[cfg(debug_assertions)]
    // fn blocks_and_states_invariant(&self) -> bool {
    //     let mut states_copy = self.states_.clone();
    //     let mut current_depth = self.blocks_.size();
    //
    //     for block in self.blocks_.iter() {
    //         let mut seen_this_block = std::collections::HashSet::new();
    //         for state in block.iter() {
    //             if !seen_this_block.contains(&state.node()) {
    //                 if states_copy.get(&(state.node() as usize, self.depth(current_depth))) != Some(state) {
    //                     return false;
    //                 }
    //                 states_copy.remove(&(state.node() as usize, self.depth(current_depth)));
    //                 seen_this_block.insert(state.node());
    //             }
    //         }
    //         current_depth -= 1;
    //     }
    //     states_copy.is_empty()
    // }
}

impl<NodeState, const NODE_UNIQUENESS: NodeUniqueness> PartialEq for ControlPathState<NodeState, NODE_UNIQUENESS>
    where
        NodeState: NodeStateTrait + Clone + PartialEq + Debug,
{
    fn eq(&self, other: &Self) -> bool {
        self.blocks_ == other.blocks_
    }
}

impl<NodeState, const NODE_UNIQUENESS: NodeUniqueness> Eq for ControlPathState<NodeState, NODE_UNIQUENESS>
    where
        NodeState: NodeStateTrait + Clone + PartialEq + Debug,
{}

#[allow(dead_code)]
pub struct AdvancedReducerWithControlPathState<NodeState, const NODE_UNIQUENESS: NodeUniqueness>
    where
        NodeState: NodeStateTrait + Clone + PartialEq + Debug,
{
    advanced_reducer: AdvancedReducer,
    zone_: Zone,
    node_states_: NodeAuxData<ControlPathState<NodeState, NODE_UNIQUENESS>>,
    reduced_: NodeAuxData<bool>,
    _phantom: PhantomData<NodeState>,
}

impl<NodeState, const NODE_UNIQUENESS: NodeUniqueness>
AdvancedReducerWithControlPathState<NodeState, NODE_UNIQUENESS>
    where
        NodeState: NodeStateTrait + Clone + PartialEq + Debug,
{
    pub fn new(editor: *mut graph_reducer::Editor, zone: Zone, graph: &TFGraph) -> Self {
        AdvancedReducerWithControlPathState {
            advanced_reducer: AdvancedReducer { editor },
            zone_: zone.clone(),
            node_states_: NodeAuxData::new(graph.node_count(), &zone),
            reduced_: NodeAuxData::new(graph.node_count(), &zone),
            _phantom: PhantomData,
        }
    }

    pub fn take_states_from_first_control(&mut self, node: *mut Node) -> graph_reducer::Reduction {
        // We just propagate the information from the control input (ideally,
        // we would only revisit control uses if there is change).
        let input = unsafe { (&*node).control_input(0) }; //NodeProperties::GetControlInput(node, 0);
        if !self.reduced_.get(input) {
            return graph_reducer::Reduction::NoChange;
        }
        self.update_states(node, self.node_states_.get(input).clone())
    }

    pub fn update_states(
        &mut self,
        state_owner: *mut Node,
        new_state: ControlPathState<NodeState, NODE_UNIQUENESS>,
    ) -> graph_reducer::Reduction {
        // Only signal that the node has {Changed} if its state has changed.
        let reduced_changed = self.reduced_.set(state_owner, true);
        let node_states_changed = self.node_states_.set(state_owner, new_state.clone());
        if reduced_changed || node_states_changed {
            return graph_reducer::Reduction::Changed(state_owner);
        }
        graph_reducer::Reduction::NoChange
    }

    pub fn update_states_with_additional(
        &mut self,
        state_owner: *mut Node,
        mut prev_states: ControlPathState<NodeState, NODE_UNIQUENESS>,
        additional_node: *mut Node,
        additional_state: NodeState,
        in_new_block: bool,
    ) -> graph_reducer::Reduction {
        if in_new_block || prev_states.is_empty() {
            prev_states.add_state_in_new_block(&self.zone_, additional_node, additional_state);
        } else {
            let original = self.node_states_.get(state_owner).clone();
            prev_states.add_state(&self.zone_, additional_node, additional_state, &original);
        }
        self.update_states(state_owner, prev_states)
    }

    pub fn zone(&self) -> &Zone {
        &self.zone_
    }

    pub fn get_state(&self, node: *mut Node) -> ControlPathState<NodeState, NODE_UNIQUENESS> {
        self.node_states_.get(node).clone()
    }

    pub fn is_reduced(&self, node: *mut Node) -> bool {
        self.reduced_.get(node)
    }
}
