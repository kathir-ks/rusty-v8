// Converted from V8 C++ source files:
// Header: node-marker.h
// Implementation: node-marker.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/compiler/node-marker.rs

use std::cell::RefCell;
use std::rc::Rc;

pub struct Node {
    mark_: u32,
}

impl Node {
    pub fn mark(&self) -> u32 {
        self.mark_
    }

    pub fn set_mark(&mut self, mark: u32) {
        self.mark_ = mark;
    }
}

pub struct TFGraph {
    pub mark_max_: u32,
}

impl TFGraph {
    pub fn new() -> Self {
        TFGraph { mark_max_: 0 }
    }
}

pub type Mark = u32;

pub struct NodeMarkerBase<'a> {
    mark_min_: Mark,
    mark_max_: Mark,
    graph: &'a mut TFGraph,
}

impl<'a> NodeMarkerBase<'a> {
    pub fn new(graph: &'a mut TFGraph, num_states: u32) -> Self {
        let mark_min_ = graph.mark_max_;
        graph.mark_max_ += num_states;
        let mark_max_ = graph.mark_max_;

        assert_ne!(0, num_states);
        assert!(mark_min_ < mark_max_);

        NodeMarkerBase {
            mark_min_,
            mark_max_,
            graph,
        }
    }

    #[inline]
    pub fn get(&self, node: &Node) -> Mark {
        let mark = node.mark();
        if mark < self.mark_min_ {
            return 0;
        }
        assert!(mark < self.mark_max_);
        mark - self.mark_min_
    }

    #[inline]
    pub fn set(&self, node: &mut Node, mark: Mark) {
        assert!(mark < self.mark_max_ - self.mark_min_);
        assert!(node.mark() < self.mark_max_);
        node.set_mark(mark + self.mark_min_);
    }
}

pub struct NodeMarker<'a, State> {
    base: NodeMarkerBase<'a>,
    _phantom: std::marker::PhantomData<State>,
}

impl<'a, State: From<Mark> + Into<Mark> + Copy> NodeMarker<'a, State> {
    #[inline]
    pub fn new(graph: &'a mut TFGraph, num_states: u32) -> Self {
        NodeMarker {
            base: NodeMarkerBase::new(graph, num_states),
            _phantom: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn get(&self, node: &Node) -> State {
        State::from(self.base.get(node))
    }

    #[inline]
    pub fn set(&self, node: &mut Node, state: State) {
        self.base.set(node, state.into());
    }
}
