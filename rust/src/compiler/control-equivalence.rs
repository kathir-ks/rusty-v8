// Copyright 2015 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// TODO: Add equivalent of v8_flags.trace_turbo_ceq.  Using a global static
// or thread_local variable might be appropriate, depending on the use case.

// src/compiler/control-equivalence.h (Partial definition)
pub mod control_equivalence {
    use std::cell::RefCell;
    use std::collections::{VecDeque, LinkedList};
    use std::rc::Rc;

    use crate::compiler::node::{Node, Edge, NodeProperties};
    use crate::compiler::zone::Zone;
    use crate::compiler::data_flow::DFSDirection;

    pub struct ControlEquivalence<'a> {
        zone_: &'a Zone,
        // TODO: Replace this with a proper data structure, potentially a HashMap.
        data_: RefCell<Vec<Option<NodeData>>>, // Vec<Option<NodeData>>
        next_class_number_: RefCell<usize>,
        graph_: &'a Graph, // Assuming Graph is accessible here.
    }

    impl<'a> ControlEquivalence<'a> {
        pub fn new(zone: &'a Zone, graph: &'a Graph) -> Self {
            ControlEquivalence {
                zone_: zone,
                data_: RefCell::new(Vec::new()),
                next_class_number_: RefCell::new(1), // Start from 1, 0 is invalid.
                graph_: graph,
            }
        }

        pub fn run(&self, exit: &Node) {
            if !self.participates(exit) || self.get_class(exit) == Self::K_INVALID_CLASS {
                self.determine_participation(exit);
                self.run_undirected_dfs(exit);
            }
        }

        const K_INVALID_CLASS: usize = 0;

        fn visit_pre(&self, node: &Node) {
            trace!("CEQ: Pre-visit of #{}:{}\n", node.id(), node.op().mnemonic());
        }

        fn visit_mid(&self, node: &Node, direction: DFSDirection) {
            trace!("CEQ: Mid-visit of #{}:{}\n", node.id(), node.op().mnemonic());
            let mut blist = self.get_bracket_list_mut(node);

            // Remove brackets pointing to this node [line:19].
            self.bracket_list_delete(&mut blist, node, direction);

            // Potentially introduce artificial dependency from start to end.
            if blist.is_empty() {
                debug_assert_eq!(DFSDirection::Input, direction);
                self.visit_backedge(node, self.graph_.end(), DFSDirection::Input);
            }

            // Potentially start a new equivalence class [line:37].
            self.bracket_list_trace(&blist);
            if let Some(recent) = blist.last_mut() {
                if recent.recent_size != blist.len() {
                    recent.recent_size = blist.len();
                    recent.recent_class = self.new_class_number();
                }

                // Assign equivalence class to node.
                self.set_class(node, recent.recent_class);
                trace!("  Assigned class number is {}\n", self.get_class(node));
            }
        }

        fn visit_post(&self, node: &Node, parent_node: Option<&Node>, direction: DFSDirection) {
            trace!("CEQ: Post-visit of #{}:{}\n", node.id(), node.op().mnemonic());
            let mut blist = self.get_bracket_list_mut(node);

            // Remove brackets pointing to this node [line:19].
            self.bracket_list_delete(&mut blist, node, direction);

            // Propagate bracket list up the DFS tree [line:13].
            if let Some(parent_node) = parent_node {
                let mut parent_blist = self.get_bracket_list_mut(parent_node);
                parent_blist.append(&mut blist);
            }
        }

        fn visit_backedge(&self, from: &Node, to: &Node, direction: DFSDirection) {
            trace!("CEQ: Backedge from #{}:{} to #{}:{}\n", from.id(), from.op().mnemonic(), to.id(), to.op().mnemonic());

            // Push backedge onto the bracket list [line:25].
            let bracket = Bracket {
                direction,
                recent_class: Self::K_INVALID_CLASS,
                recent_size: 0,
                from: from as *const Node,
                to: to as *const Node,
            };
            self.get_bracket_list_mut(from).push_back(bracket);
        }

        fn run_undirected_dfs(&self, exit: &Node) {
            let mut stack: Vec<DFSStackEntry> = Vec::new();
            self.dfs_push(&mut stack, exit, None, DFSDirection::Input);
            self.visit_pre(exit);

            while !stack.is_empty() {
                // Undirected depth-first backwards traversal.
                let entry = stack.last().unwrap();
                let node = entry.node;

                if entry.direction == DFSDirection::Input {
                    let mut input_iter = unsafe { node.input_edges().iter() };
                    if let Some(edge) = input_iter.next() {
                        let input = edge.to();
                        let mut new_entry = stack.pop().unwrap();
                        new_entry.input_iter = input_iter;
                        stack.push(new_entry);

                        if NodeProperties::is_control_edge(&edge) {
                            // Visit next control input.
                            if !self.participates(input) { continue; }
                            if self.get_data(input).visited { continue; }
                            if self.get_data(input).on_stack {
                                // Found backedge if input is on stack.
                                if input as *const Node != entry.parent_node.map_or(std::ptr::null(), |n| n as *const Node) {
                                    self.visit_backedge(node, input, DFSDirection::Input);
                                }
                            } else {
                                // Push input onto stack.
                                self.dfs_push(&mut stack, input, Some(node), DFSDirection::Input);
                                self.visit_pre(input);
                            }
                        }
                        continue;
                    }

                    let mut use_iter = unsafe {node.use_edges().iter()};
                    if let Some(_edge) = use_iter.next() {
                        // Switch direction to uses.
                        let mut new_entry = stack.pop().unwrap();
                        new_entry.direction = DFSDirection::Use;
                        new_entry.use_iter = use_iter;

                        stack.push(new_entry);
                        self.visit_mid(node, DFSDirection::Input);
                        continue;
                    }
                }

                if entry.direction == DFSDirection::Use {
                    let mut use_iter = unsafe { node.use_edges().iter() };

                    if let Some(edge) = use_iter.next() {
                        let use = edge.from();

                        let mut new_entry = stack.pop().unwrap();
                        new_entry.use_iter = use_iter;
                        stack.push(new_entry);

                        if NodeProperties::is_control_edge(&edge) {
                            // Visit next control use.
                            if !self.participates(use) { continue; }
                            if self.get_data(use).visited { continue; }
                            if self.get_data(use).on_stack {
                                // Found backedge if use is on stack.
                                if use as *const Node != entry.parent_node.map_or(std::ptr::null(), |n| n as *const Node) {
                                    self.visit_backedge(node, use, DFSDirection::Use);
                                }
                            } else {
                                // Push use onto stack.
                                self.dfs_push(&mut stack, use, Some(node), DFSDirection::Use);
                                self.visit_pre(use);
                            }
                        }
                        continue;
                    }

                    let mut input_iter = unsafe { node.input_edges().iter() };
                    if let Some(_edge) = input_iter.next() {
                        // Switch direction to inputs.
                        let mut new_entry = stack.pop().unwrap();
                        new_entry.direction = DFSDirection::Input;
                        new_entry.input_iter = input_iter;
                        stack.push(new_entry);

                        self.visit_mid(node, DFSDirection::Use);
                        continue;
                    }
                }

                // Pop node from stack when done with all inputs and uses.
                let entry = stack.last().unwrap();
                debug_assert!(unsafe { entry.node.input_edges().is_empty() });
                debug_assert!(unsafe { entry.node.use_edges().is_empty() });
                let node = entry.node;

                let parent_node = entry.parent_node;
                let direction = entry.direction;

                self.visit_post(node, parent_node, direction);
                self.dfs_pop(&mut stack, node);
            }
        }

        fn determine_participation_enqueue(&self, queue: &mut VecDeque<&Node>, node: &Node) {
            if !self.participates(node) {
                self.allocate_data(node);
                queue.push_back(node);
            }
        }

        fn determine_participation(&self, exit: &Node) {
            let mut queue: VecDeque<&Node> = VecDeque::new();
            self.determine_participation_enqueue(&mut queue, exit);
            while let Some(node) = queue.pop_front() {
                let max = NodeProperties::past_control_index(node);
                for i in NodeProperties::first_control_index(node)..max {
                    self.determine_participation_enqueue(&mut queue, unsafe { node.input_at(i) });
                }
            }
        }

        fn dfs_push(&self, stack: &mut Vec<DFSStackEntry>, node: &Node, from: Option<&Node>, dir: DFSDirection) {
            debug_assert!(self.participates(node));
            debug_assert!(!self.get_data(node).visited);
            self.get_data_mut(node).on_stack = true;

            let input_iter = unsafe{node.input_edges().iter()};
            let use_iter = unsafe{node.use_edges().iter()};

            stack.push(DFSStackEntry {
                direction: dir,
                input_iter,
                use_iter,
                parent_node: from,
                node,
            });
        }

        fn dfs_pop(&self, stack: &mut Vec<DFSStackEntry>, node: &Node) {
            debug_assert_eq!(stack.last().map(|x| x.node as *const Node), Some(node as *const Node));
            self.get_data_mut(node).on_stack = false;
            self.get_data_mut(node).visited = true;
            stack.pop();
        }

        fn bracket_list_delete(&self, blist: &mut LinkedList<Bracket>, to: &Node, direction: DFSDirection) {
            blist.retain(|i| {
                if i.to == to as *const Node && i.direction != direction {
                    trace!("  BList erased: {{{}->{}}}\n", unsafe { (*i.from).id() }, unsafe { (*i.to).id() });
                    false
                } else {
                    true
                }
            });
        }

        fn bracket_list_trace(&self, blist: &LinkedList<Bracket>) {
            if false { // TODO: Replace v8_flags.trace_turbo_ceq
                trace!("  BList: ");
                for bracket in blist {
                    trace!("{{{}->{}}} ", unsafe { (*bracket.from).id() }, unsafe { (*bracket.to).id() });
                }
                trace!("\n");
            }
        }

        fn new_class_number(&self) -> usize {
            let mut next_class_number = self.next_class_number_.borrow_mut();
            let result = *next_class_number;
            *next_class_number += 1;
            result
        }

        fn participates(&self, node: &Node) -> bool {
            let data = self.data_.borrow();
            if node.id() >= data.len() {
                return false;
            }
            data[node.id()].is_some()
        }

        fn allocate_data(&self, node: &Node) {
            let mut data = self.data_.borrow_mut();
            while data.len() <= node.id() {
                data.push(None);
            }
            data[node.id()] = Some(NodeData {
                visited: false,
                on_stack: false,
                class: Self::K_INVALID_CLASS,
                bracket_list: LinkedList::new()
            });
        }

        fn get_data(&self, node: &Node) -> &NodeData {
            let data = self.data_.borrow();
            data[node.id()].as_ref().unwrap()
        }

        fn get_data_mut(&self, node: &Node) -> &mut NodeData {
            let mut data = self.data_.borrow_mut();
            data[node.id()].as_mut().unwrap()
        }

        fn set_class(&self, node: &Node, class: usize) {
            self.get_data_mut(node).class = class;
        }

        fn get_class(&self, node: &Node) -> usize {
            self.get_data(node).class
        }

        fn get_bracket_list_mut(&self, node: &Node) -> &mut LinkedList<Bracket> {
            &mut self.get_data_mut(node).bracket_list
        }
    }

    #[derive(Debug)]
    struct Bracket {
        direction: DFSDirection,
        recent_class: usize,
        recent_size: usize,
        from: *const Node,
        to: *const Node,
    }

    #[derive(Debug)]
    struct NodeData {
        visited: bool,
        on_stack: bool,
        class: usize,
        bracket_list: LinkedList<Bracket>,
    }

    #[derive(Debug)]
    struct DFSStackEntry {
        direction: DFSDirection,
        input_iter: std::slice::Iter<'static, Edge>,
        use_iter: std::slice::Iter<'static, Edge>,
        parent_node: Option<&'static Node>,
        node: &'static Node,
    }

    #[derive(Debug)]
    pub struct Graph {
        end_: Box<Node>,
    }

    impl Graph {
        pub fn new(end_: Box<Node>) -> Self {
            Graph { end_ }
        }

        pub fn end(&self) -> &Node {
            &self.end_
        }
    }

}

// src/compiler/node-properties.h (Partial definition)
pub mod node {
    use crate::compiler::data_flow::DFSDirection;

    #[derive(Debug, PartialEq, Eq)]
    pub struct Node {
        id_: usize,
        op_: Operation,
        inputs: Vec<Edge>,
        uses: Vec<Edge>,
    }

    impl Node {
        pub fn new(id: usize, op: Operation) -> Self {
            Node { id_: id, op_: op, inputs: Vec::new(), uses: Vec::new() }
        }

        pub fn id(&self) -> usize {
            self.id_
        }

        pub fn op(&self) -> &Operation {
            &self.op_
        }

        pub unsafe fn input_at(&self, index: usize) -> &Node {
            self.inputs[index].to()
        }

        pub fn input_edges(&self) -> &Vec<Edge> {
            &self.inputs
        }

        pub fn use_edges(&self) -> &Vec<Edge> {
            &self.uses
        }

        pub fn input_edges_mut(&mut self) -> &mut Vec<Edge> {
            &mut self.inputs
        }

        pub fn use_edges_mut(&mut self) -> &mut Vec<Edge> {
            &mut self.uses
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct Operation {
        mnemonic_: String
    }

    impl Operation {
        pub fn new(mnemonic: String) -> Self {
            Operation{mnemonic_: mnemonic}
        }

        pub fn mnemonic(&self) -> &str {
            &self.mnemonic_
        }
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub struct Edge {
        from_: *const Node,
        to_: *const Node,
    }

    impl Edge {
        pub fn new(from: *const Node, to: *const Node) -> Self {
            Edge { from_: from, to_: to }
        }

        pub fn from(&self) -> &Node {
            unsafe { &*self.from_ }
        }

        pub fn to(&self) -> &Node {
            unsafe { &*self.to_ }
        }
    }

    pub struct NodeProperties {}

    impl NodeProperties {
        pub fn is_control_edge(_edge: &Edge) -> bool {
            // In this simplified example, all edges are considered control edges.
            true
        }

        pub fn first_control_index(_node: &Node) -> usize {
            0
        }

        pub fn past_control_index(node: &Node) -> usize {
            node.inputs.len()
        }
    }
}

pub mod data_flow {
    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum DFSDirection {
        Input,
        Use,
    }
}

pub mod zone {
    #[derive(Debug)]
    pub struct Zone {}

    impl Zone {
        pub fn new() -> Self {
            Zone {}
        }
    }
}

macro_rules! trace {
    ($($arg:tt)*) => {
        if false { // TODO: Implement trace flag
            println!($($arg)*);
        }
    };
}

#[cfg(debug_assertions)]
macro_rules! debug_assert_eq {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right);
    };
}

#[cfg(not(debug_assertions))]
macro_rules! debug_assert_eq {
    ($left:expr, $right:expr) => {
        {}
    };
}

fn main() {
    use crate::control_equivalence::*;
    use crate::node::*;
    use crate::zone::*;

    let zone = Zone::new();
    let op_end = Operation::new("End".to_string());
    let end_node = Node::new(0, op_end);
    let graph = Graph::new(Box::new(end_node));

    let ceq = ControlEquivalence::new(&zone, &graph);
    let op_start = Operation::new("Start".to_string());
    let mut start_node = Node::new(1, op_start);
    let end_edge = Edge::new(&start_node, graph.end());
    start_node.input_edges_mut().push(end_edge);

    ceq.run(&start_node);
}