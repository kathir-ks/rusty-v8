// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod control_path_state {
    use std::cell::RefCell;
    use std::collections::{HashMap, HashSet};
    use std::hash::{Hash, Hasher};
    use std::rc::Rc;

    // Placeholder for Zone functionality.  In C++, Zone provides a custom memory allocator.
    // In Rust, we can use a standard allocator, or explore custom allocators if performance
    // becomes critical.  Using Rc<RefCell<>> for mutability and shared ownership.
    pub type Zone = Rc<RefCell<()>>;

    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum NodeUniqueness {
        UniqueInstance,
        MultipleInstances,
    }

    // Placeholder for Node.  Replace with actual Node definition.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct Node {
        id: usize,
    }

    impl Node {
        pub fn new(id: usize) -> Self {
            Node { id }
        }
    }

    // Placeholder for FunctionalList. Using Vec for simplicity.
    pub type FunctionalList<T> = Vec<T>;

    // Placeholder for PersistentMap. Using HashMap for simplicity.
    pub type PersistentMap<K, V> = HashMap<K, V>;

    // Placeholder for GraphReducer.  Minimal implementation for now.
    pub trait GraphReducer {
        fn reduce(&mut self, node: Node) -> Reduction;
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum Reduction {
        Changed(Node),
        NoChange,
    }

    // Placeholder for Editor.  Minimal implementation for now.
    pub trait Editor {
        fn replace_input(&mut self, node: Node, index: usize, new_input: Node);
    }

    // Placeholder for TFGraph.  Minimal implementation for now.
    pub struct TFGraph {
        node_count: usize,
    }

    impl TFGraph {
        pub fn new(node_count: usize) -> Self {
            TFGraph { node_count }
        }
        pub fn NodeCount(&self) -> usize {
            self.node_count
        }
    }

    // Placeholder for NodeProperties.  Minimal implementation for now.
    pub mod NodeProperties {
        use super::Node;
        pub fn GetControlInput(node: Node, index: usize) -> Node {
            // Dummy implementation
            Node::new(node.id + index)
        }
    }

    pub trait IsSet {
        fn is_set(&self) -> bool;
    }

    // Trait for accessing the `node` field.
    pub trait HasNode {
        fn node(&self) -> Node;
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct NodeState {
        node: Node,
        value: i32, // Example state. Replace with actual data.
        is_set: bool,
    }

    impl NodeState {
        pub fn new(node: Node, value: i32) -> Self {
            NodeState {
                node,
                value,
                is_set: true,
            }
        }

        pub fn unset(node: Node) -> Self {
            NodeState {
                node,
                value: 0,
                is_set: false,
            }
        }
    }

    impl Default for NodeState {
        fn default() -> Self {
            NodeState {
                node: Node::new(0), // Default node
                value: 0,            // Default value
                is_set: false,       // Not set by default
            }
        }
    }

    impl IsSet for NodeState {
        fn is_set(&self) -> bool {
            self.is_set
        }
    }

    impl HasNode for NodeState {
        fn node(&self) -> Node {
            self.node
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct NodeWithPathDepth {
        node: Node,
        depth: usize,
    }

    impl Hash for NodeWithPathDepth {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.node.hash(state);
            self.depth.hash(state);
        }
    }

    impl NodeWithPathDepth {
        pub fn new(node: Node, depth: usize) -> Self {
            NodeWithPathDepth { node, depth }
        }
    }

    // Class for tracking information about path state. It is represented as a
    // linked list of {NodeState} blocks, each of which corresponds to a block of
    // code between an IfTrue/IfFalse and a Merge. Each block is in turn represented
    // as a linked list of {NodeState}s.
    // If {node_uniqueness} is {kMultipleInstances}, different states can be
    // assigned to the same node. The most recent state always takes precedence.
    // States still belong to a block and will be removed if the block gets merged.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ControlPathState<NodeState, const NODE_UNIQUENESS: NodeUniqueness> {
        blocks_: FunctionalList<FunctionalList<NodeState>>,
        states_: PersistentMap<NodeWithPathDepth, NodeState>,
    }

    impl<T: Clone + PartialEq + IsSet + HasNode + std::fmt::Debug, const NODE_UNIQUENESS: NodeUniqueness>
        ControlPathState<T, NODE_UNIQUENESS>
    {
        pub fn new(zone: &Zone) -> Self {
            ControlPathState {
                blocks_: Vec::new(),
                states_: HashMap::new(),
            }
        }

        // Returns the {NodeState} assigned to node, or the default value
        // {NodeState()} if it is not assigned.
        pub fn lookup_state(&self, node: Node) -> T {
            if NODE_UNIQUENESS == NodeUniqueness::UniqueInstance {
                return self
                    .states_
                    .get(&NodeWithPathDepth::new(node, 0))
                    .cloned()
                    .unwrap_or_default();
            }

            for depth in (1..=self.blocks_.len()).rev() {
                if let Some(state) = self.states_.get(&NodeWithPathDepth::new(node, depth)) {
                    if state.is_set() {
                        return state.clone();
                    }
                }
            }
            T::default()
        }

        // Adds a state in the current code block, or a new block if the block list is
        // empty.
        pub fn add_state(
            &mut self,
            zone: &Zone,
            node: Node,
            state: T,
            hint: &ControlPathState<T, NODE_UNIQUENESS>,
        ) {
            let previous_state = self.lookup_state(node);
            if NODE_UNIQUENESS == NodeUniqueness::UniqueInstance {
                if previous_state.is_set() {
                    return;
                }
            } else {
                if previous_state == state {
                    return;
                }
            }

            let mut prev_front = self.blocks_.first().cloned().unwrap_or_default();

            if !hint.blocks_.is_empty() {
                // PushFront takes ownership so we need to clone
                prev_front.insert(0, state.clone());
            } else {
                prev_front.insert(0, state.clone());
            }

            if !self.blocks_.is_empty() {
                self.blocks_.remove(0);
            }
            self.blocks_.insert(0, prev_front);

            self.states_.insert(
                NodeWithPathDepth::new(node, self.depth(self.blocks_.len())),
                state,
            );
            if cfg!(debug_assertions) {
                self.blocks_and_states_invariant();
            }
        }

        // Adds a state in a new block.
        pub fn add_state_in_new_block(&mut self, zone: &Zone, node: Node, state: T) {
            let mut new_block: FunctionalList<T> = Vec::new();
            let previous_state = self.lookup_state(node);

            if NODE_UNIQUENESS == NodeUniqueness::UniqueInstance {
                if !previous_state.is_set() {
                    new_block.push(state.clone());
                    self.states_.insert(
                        NodeWithPathDepth::new(node, self.depth(self.blocks_.len() + 1)),
                        state,
                    );
                }
            } else {
                if previous_state != state {
                    new_block.push(state.clone());
                    self.states_.insert(
                        NodeWithPathDepth::new(node, self.depth(self.blocks_.len() + 1)),
                        state,
                    );
                }
            }

            self.blocks_.insert(0, new_block);
            if cfg!(debug_assertions) {
                self.blocks_and_states_invariant();
            }
        }

        // Reset this {ControlPathState} to its longest prefix that is common with
        // {other}.
        pub fn reset_to_common_ancestor(&mut self, mut other: ControlPathState<T, NODE_UNIQUENESS>) {
            while other.blocks_.len() > self.blocks_.len() {
                other.blocks_.remove(0);
            }
            while self.blocks_.len() > other.blocks_.len() {
                for state in self.blocks_.first().cloned().unwrap_or_default() {
                    self.states_.insert(
                        NodeWithPathDepth::new(state.node(), self.depth(self.blocks_.len())),
                        T::default(),
                    );
                }
                self.blocks_.remove(0);
            }

            while self.blocks_ != other.blocks_ {
                for state in self.blocks_.first().cloned().unwrap_or_default() {
                    self.states_.insert(
                        NodeWithPathDepth::new(state.node(), self.depth(self.blocks_.len())),
                        T::default(),
                    );
                }
                self.blocks_.remove(0);
                other.blocks_.remove(0);
            }
            if cfg!(debug_assertions) {
                self.blocks_and_states_invariant();
            }
        }

        pub fn is_empty(&self) -> bool {
            self.blocks_.is_empty()
        }

        fn depth(&self, depth_if_multiple_instances: usize) -> usize {
            match NODE_UNIQUENESS {
                NodeUniqueness::MultipleInstances => depth_if_multiple_instances,
                NodeUniqueness::UniqueInstance => 0,
            }
        }

        #[cfg(debug_assertions)]
        fn blocks_and_states_invariant(&self) -> bool {
            let mut states_copy = self.states_.clone();
            let mut current_depth = self.blocks_.len();

            for block in &self.blocks_ {
                let mut seen_this_block: HashSet<Node> = HashSet::new();
                for state in block {
                    // Every element of blocks_ has to be in states_.
                    if !seen_this_block.contains(&state.node()) {
                        let expected_state = states_copy.get(&NodeWithPathDepth::new(
                            state.node(),
                            self.depth(current_depth),
                        ));
                        if expected_state != Some(state) {
                            return false;
                        }
                        states_copy.insert(
                            NodeWithPathDepth::new(state.node(), self.depth(current_depth)),
                            T::default(),
                        );
                        seen_this_block.insert(state.node());
                    }
                }
                current_depth -= 1;
            }

            // Every element of {states_} has to be in {blocks_}. We removed all
            // elements of blocks_ from states_copy, so if it is not empty, the
            // invariant fails.
            states_copy.is_empty()
        }
    }

    pub struct AdvancedReducerWithControlPathState<
        NodeState: Clone + PartialEq + IsSet + HasNode + std::fmt::Debug,
        const NODE_UNIQUENESS: NodeUniqueness,
        R: GraphReducer,
        E: Editor,
    > {
        editor: E,
        zone_: Zone,
        node_states_: NodeAuxData<ControlPathState<NodeState, NODE_UNIQUENESS>>,
        reduced_: NodeAuxData<bool>,
        reducer: R,
        graph: TFGraph,
    }

    impl<
            NodeState: Clone + PartialEq + IsSet + HasNode + std::fmt::Debug,
            const NODE_UNIQUENESS: NodeUniqueness,
            R: GraphReducer,
            E: Editor,
        > AdvancedReducerWithControlPathState<NodeState, NODE_UNIQUENESS, R, E>
    {
        pub fn new(editor: E, zone: Zone, graph: TFGraph, reducer: R) -> Self {
            AdvancedReducerWithControlPathState {
                editor,
                zone_: zone.clone(),
                node_states_: NodeAuxData::new(graph.NodeCount(), &zone),
                reduced_: NodeAuxData::new(graph.NodeCount(), &zone),
                reducer,
                graph,
            }
        }

        fn take_states_from_first_control(&mut self, node: Node) -> Reduction {
            // We just propagate the information from the control input (ideally,
            // we would only revisit control uses if there is change).
            let input = NodeProperties::GetControlInput(node, 0);
            if !self.reduced_.get(input) {
                return Reduction::NoChange;
            }
            self.update_states(node, self.node_states_.get(input).clone())
        }

        // Update the state of {state_owner} to {new_state}.
        fn update_states(
            &mut self,
            state_owner: Node,
            new_state: ControlPathState<NodeState, NODE_UNIQUENESS>,
        ) -> Reduction {
            // Only signal that the node has {Changed} if its state has changed.
            let reduced_changed = self.reduced_.set(state_owner, true);
            let node_states_changed = self.node_states_.set(state_owner, new_state.clone());

            if reduced_changed || node_states_changed {
                Reduction::Changed(state_owner)
            } else {
                Reduction::NoChange
            }
        }

        // Update the state of {state_owner} to {prev_states}, plus {additional_state}
        // assigned to {additional_node}. Force the new state in a new block if
        // {in_new_block}.
        fn update_states_with_additional(
            &mut self,
            state_owner: Node,
            mut prev_states: ControlPathState<NodeState, NODE_UNIQUENESS>,
            additional_node: Node,
            additional_state: NodeState,
            in_new_block: bool,
        ) -> Reduction {
            if in_new_block || prev_states.is_empty() {
                prev_states.add_state_in_new_block(&self.zone_, additional_node, additional_state);
            } else {
                let original = self.node_states_.get(state_owner).clone();
                prev_states.add_state(&self.zone_, additional_node, additional_state, &original);
            }
            self.update_states(state_owner, prev_states)
        }

        fn zone(&self) -> &Zone {
            &self.zone_
        }

        fn get_state(&self, node: Node) -> ControlPathState<NodeState, NODE_UNIQUENESS> {
            self.node_states_.get(node).clone()
        }

        fn is_reduced(&self, node: Node) -> bool {
            self.reduced_.get(node)
        }
    }

    impl<
            NodeState: Clone + PartialEq + IsSet + HasNode + std::fmt::Debug,
            const NODE_UNIQUENESS: NodeUniqueness,
            R: GraphReducer,
            E: Editor,
        > GraphReducer for AdvancedReducerWithControlPathState<NodeState, NODE_UNIQUENESS, R, E>
    {
        fn reduce(&mut self, node: Node) -> Reduction {
            self.reducer.reduce(node)
        }
    }

    // Placeholder for NodeAuxData. Using HashMap for simplicity.
    // NodeAuxData stores auxiliary data associated with nodes.
    pub struct NodeAuxData<T: Clone> {
        data: HashMap<Node, T>,
        default: T,
    }

    impl<T: Clone> NodeAuxData<T> {
        pub fn new(node_count: usize, zone: &Zone) -> Self {
            // `node_count` is unused, but it's kept for API parity with C++.
            NodeAuxData {
                data: HashMap::new(),
                default: T::clone(&Self::default_value()),
            }
        }

        fn default_value() -> T
        where
            T: Default,
        {
            T::default()
        }

        pub fn get(&self, node: Node) -> &T {
            self.data.get(&node).unwrap_or(&self.default)
        }

        pub fn set(&mut self, node: Node, value: T) -> bool {
            if self.data.get(&node) == Some(&value) {
                return false;
            }
            self.data.insert(node, value);
            true
        }
    }
}