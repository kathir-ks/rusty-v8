// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod turbofan_graph; // Assuming turbofan-graph.h is in src/compiler
use turbofan_graph::TFGraph;

use std::collections::HashSet;

/// Represents a collection of nodes reachable from a given starting point in a graph.
pub struct AllNodes<'a> {
    reachable: Vec<&'a Node>,
    is_reachable_: HashSet<usize>,
    only_inputs_: bool,
}

impl<'a> AllNodes<'a> {
    /// Creates a new `AllNodes` instance, marking all nodes reachable from the end node of the graph.
    pub fn new(local_zone: &mut Zone, graph: &'a TFGraph, only_inputs: bool) -> Self {
        let mut all_nodes = AllNodes {
            reachable: Vec::new(),
            is_reachable_: HashSet::new(),
            only_inputs_: only_inputs,
        };
        all_nodes.mark(local_zone, graph.end(), graph);
        all_nodes
    }

    /// Creates a new `AllNodes` instance, marking all nodes reachable from the given end node.
    pub fn with_end(local_zone: &mut Zone, end: &'a Node, graph: &'a TFGraph, only_inputs: bool) -> Self {
        let mut all_nodes = AllNodes {
            reachable: Vec::new(),
            is_reachable_: HashSet::new(),
            only_inputs_: only_inputs,
        };
        all_nodes.mark(local_zone, end, graph);
        all_nodes
    }

    /// Marks all nodes reachable from the given end node.
    fn mark(&mut self, _local_zone: &mut Zone, end: &'a Node, graph: &'a TFGraph) {
        debug_assert!(end.id() < graph.node_count());
        self.is_reachable_.insert(end.id());
        self.reachable.push(end);

        let mut i = 0;
        while i < self.reachable.len() {
            for input in self.reachable[i].inputs() {
                if input.is_none() {
                    // TODO(titzer): print a warning.
                    continue;
                }
                let input_node = input.unwrap();
                if !self.is_reachable_.contains(&input_node.id()) {
                    self.is_reachable_.insert(input_node.id());
                    self.reachable.push(input_node);
                }
            }

            if !self.only_inputs_ {
                for use_node in self.reachable[i].uses() {
                    if use_node.is_none() {
                        continue;
                    }
                    let use_node = use_node.unwrap();

                    if use_node.id() >= graph.node_count() {
                        continue;
                    }
                    if !self.is_reachable_.contains(&use_node.id()) {
                        self.is_reachable_.insert(use_node.id());
                        self.reachable.push(use_node);
                    }
                }
            }
            i += 1;
        }
    }
}

// Dummy implementations for types used in the original code, to allow compilation
// These need to be replaced with actual implementations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(usize);

impl NodeId {
    pub fn new(id: usize) -> Self {
        NodeId(id)
    }
}

#[derive(Debug)]
pub struct Node {
    id: usize,
    inputs: Vec<Option<&'a Node>>,
    uses: Vec<Option<&'a Node>>,
}

impl<'a> Node {
    pub fn new(id: usize, inputs: Vec<Option<&'a Node>>, uses: Vec<Option<&'a Node>>) -> Self {
        Node { id, inputs, uses }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn inputs(&self) -> &[Option<&'a Node>] {
        &self.inputs
    }

    pub fn uses(&self) -> &[Option<&'a Node>] {
        &self.uses
    }
}

#[derive(Debug)]
pub struct Zone {}

impl Zone {
    pub fn new() -> Self {
        Zone {}
    }
}