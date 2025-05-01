// Copyright 2015 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod compiler {
    use std::cell::RefCell;
    use std::rc::Rc;

    pub trait Graph {
        fn end(&self) -> &Node;
    }

    pub trait NodeLike {
        fn is_dead(&self) -> bool;
        fn id(&self) -> usize;
    }

    // Dummy TFGraph and Node types for compilation.  Replace with actual
    // graph implementation.
    pub struct TFGraph {
        end: Node,
    }

    impl TFGraph {
        pub fn new(end: Node) -> Self {
            TFGraph { end }
        }
    }

    impl Graph for TFGraph {
        fn end(&self) -> &Node {
            &self.end
        }
    }

    #[derive(Clone)]
    pub struct Node {
        id: usize,
        dead: bool,
    }

    impl Node {
        pub fn new(id: usize, dead: bool) -> Self {
            Node { id, dead }
        }
    }

    impl NodeLike for Node {
        fn is_dead(&self) -> bool {
            self.dead
        }
        fn id(&self) -> usize {
            self.id
        }
    }

    pub struct NodeMarker<T> {
        data: RefCell<Vec<Option<T>>>,
    }

    impl<T: Copy> NodeMarker<T> {
        pub fn new(num_nodes: usize, default_value: T) -> Self {
            NodeMarker {
                data: RefCell::new(vec![Some(default_value); num_nodes]),
            }
        }

        pub fn get(&self, node: &Node) -> T {
            self.data.borrow()[node.id()].unwrap()
        }

        pub fn set(&self, node: &Node, value: T) {
            self.data.borrow_mut()[node.id()] = Some(value);
        }
    }

    pub type NodeVector = Vec<Rc<Node>>;

    /// Trims dead nodes from the node graph.
    pub struct GraphTrimmer<'a> {
        graph_: &'a TFGraph,
        is_live_: NodeMarker<bool>,
        live_: RefCell<NodeVector>,
    }

    impl<'a> GraphTrimmer<'a> {
        /// Creates a new `GraphTrimmer`.
        pub fn new(graph: &'a TFGraph, num_nodes: usize) -> Self {
            GraphTrimmer {
                graph_: graph,
                is_live_: NodeMarker::new(num_nodes, false),
                live_: RefCell::new(Vec::new()),
            }
        }

        /// Trim nodes in the {graph} that are not reachable from {graph->end()}.
        pub fn trim_graph(&self) {
            self.mark_as_live(self.graph().end());
            let mut live_nodes = self.live_.borrow_mut();
            while let Some(node) = live_nodes.pop() {
                // Iterate through the inputs of the live node and mark them as live.
                // This part depends on how graph is implemented.
                // Example: node.inputs().for_each(|input| self.mark_as_live(input));
            }
        }

        /// Trim nodes in the {graph} that are not reachable from either {graph->end()}
        /// or any of the roots in the sequence [{begin},{end}[.
        pub fn trim_graph_with_roots<I>(&self, begin: I, end: I)
        where
            I: Iterator<Item = Rc<Node>> + Clone,
        {
            for node in begin.clone().take(end.count()) {
                if !node.is_dead() {
                    self.mark_as_live(&node);
                }
            }
            self.trim_graph();
        }

        #[inline]
        fn is_live(&self, node: &Node) -> bool {
            self.is_live_.get(node)
        }

        #[inline]
        fn mark_as_live(&self, node: &Node) {
            if !node.is_dead() {
                if !self.is_live(node) {
                    self.is_live_.set(node, true);
                    self.live_.borrow_mut().push(Rc::new(node.clone()));
                }
            }
        }

        fn graph(&self) -> &TFGraph {
            self.graph_
        }
    }
}