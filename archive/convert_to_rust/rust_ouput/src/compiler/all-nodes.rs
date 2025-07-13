// Converted from V8 C++ source files:
// Header: all-nodes.h
// Implementation: all-nodes.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod compiler {
    use crate::utils::bit_vector::BitVector;

    pub struct AllNodes {
        reachable: Vec<*mut Node>,
        is_reachable_: BitVector,
        only_inputs_: bool,
    }

    impl AllNodes {
        pub fn new(local_zone: &mut Zone, graph: &TFGraph, only_inputs: bool) -> Self {
            let mut all_nodes = AllNodes {
                reachable: Vec::new(),
                is_reachable_: BitVector::new(graph.NodeCount() as usize, local_zone),
                only_inputs_: only_inputs,
            };
            all_nodes.mark(local_zone, graph.end, graph);
            all_nodes
        }

        pub fn new_with_end(local_zone: &mut Zone, end: *mut Node, graph: &TFGraph, only_inputs: bool) -> Self {
             let mut all_nodes = AllNodes {
                reachable: Vec::new(),
                is_reachable_: BitVector::new(graph.NodeCount() as usize, local_zone),
                only_inputs_: only_inputs,
            };
            all_nodes.mark(local_zone, end, graph);
            all_nodes
        }

        pub fn is_live(&self, node: *const Node) -> bool {
            assert!(self.only_inputs_);
            self.is_reachable(node)
        }

        pub fn is_reachable(&self, node: *const Node) -> bool {
            if node.is_null() {
                return false;
            }
            unsafe {
                let node = &*node;
                let id = node.id();
                id < self.is_reachable_.length() && self.is_reachable_.contains(id)
            }
        }

        fn mark(&mut self, local_zone: &mut Zone, end: *mut Node, graph: &TFGraph) {
            unsafe {
                let end_node = &mut *end;
                assert!(end_node.id() < graph.NodeCount() as usize);
                self.is_reachable_.add(end_node.id());
                self.reachable.push(end);

                for i in 0..self.reachable.len() {
                    let current_node = &mut *self.reachable[i];
                    for input_node in current_node.inputs() {
                        if input_node.is_null() {
                             //println!("Warning: Null input node encountered.");
                            continue;
                        }

                        let input = &mut *input_node;

                        if !self.is_reachable_.contains(input.id()) {
                            self.is_reachable_.add(input.id());
                            self.reachable.push(input_node);
                        }
                    }

                    if !self.only_inputs_ {
                        for use_node in current_node.uses() {
                            if use_node.is_null() {
                                continue;
                            }
                            let use = &mut *use_node;

                            if use.id() >= graph.NodeCount() as usize{
                                continue;
                            }

                            if !self.is_reachable_.contains(use.id()) {
                                self.is_reachable_.add(use.id());
                                self.reachable.push(use_node);
                            }
                        }
                    }
                }
            }
        }
    }

    // Dummy implementations for dependencies.  These need to be replaced with real implementations
    // from the V8 codebase for the code to function correctly.
    pub struct Zone {}

    impl Zone {
        pub fn new() -> Self {
            Zone {}
        }
    }

    pub struct TFGraph {
        node_count: usize,
        end: *mut Node
    }

    impl TFGraph {
        pub fn new(node_count: usize, end: *mut Node) -> Self {
            TFGraph {
                node_count: node_count,
                end: end
            }
        }
        pub fn NodeCount(&self) -> usize {
            self.node_count
        }
    }

    #[derive(Debug)]
    pub struct Node {
        id_: usize,
        inputs_: Vec<*mut Node>,
        uses_: Vec<*mut Node>
    }

    impl Node {
        pub fn new(id: usize) -> Self {
            Node { id_: id, inputs_: Vec::new(), uses_: Vec::new() }
        }

        pub fn id(&self) -> usize {
            self.id_
        }

        pub fn inputs(&mut self) -> &Vec<*mut Node> {
            &self.inputs_
        }
        
        pub fn add_input(&mut self, input: *mut Node) {
            self.inputs_.push(input);
        }

        pub fn uses(&mut self) -> &Vec<*mut Node> {
            &self.uses_
        }

        pub fn add_use(&mut self, use_node: *mut Node) {
            self.uses_.push(use_node);
        }
    }
}
