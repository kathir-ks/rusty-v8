// Converted from V8 C++ source files:
// Header: graph-trimmer.h
// Implementation: graph-trimmer.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub struct GraphTrimmer {
    graph_: *mut TFGraph,
    is_live_: NodeMarker<bool>,
    live_: Vec<*mut Node>,
}

impl GraphTrimmer {
    pub fn new(zone: *mut Zone, graph: *mut TFGraph) -> Self {
        let node_count = unsafe { (*graph).NodeCount() };
        GraphTrimmer {
            graph_: graph,
            is_live_: NodeMarker::new(graph, 2),
            live_: Vec::with_capacity(node_count),
        }
    }

    pub fn trim_graph(&mut self) {
        // Mark end node as live.
        unsafe {
            self.mark_as_live((*self.graph_).end());
        }

        // Compute transitive closure of live nodes.
        for i in 0..self.live_.len() {
            let live = self.live_[i];
            let inputs = unsafe { (*live).inputs() };
            for input in inputs {
                unsafe {
                    self.mark_as_live(input);
                }
            }
        }

        // Remove dead->live edges.
        for live in &self.live_ {
            unsafe {
                assert!(self.is_live(*live));
                let use_edges = (*live).use_edges();
                for edge in use_edges {
                    let user = edge.from();
                    if !self.is_live(user) {
                        if v8_flags.trace_turbo_trimming {
                            StdoutStream {} << "DeadLink: " << *user << "(" << edge.index()
                                             << ") -> " << *live << std::endl;
                        }
                        edge.UpdateTo(std::ptr::null_mut());
                    }
                }
            }
        }
    }

    pub fn trim_graph_with_iter<ForwardIterator>(&mut self, mut begin: ForwardIterator, end: ForwardIterator)
        where ForwardIterator: Iterator<Item = *mut Node> {
        let mut current = begin;
        while let Some(node) = current.next() {
            if unsafe { !(*node).IsDead() } {
                unsafe { self.mark_as_live(node); }
            }
        }
        self.trim_graph();
    }

    #[inline]
    fn is_live(&self, node: *mut Node) -> bool {
        self.is_live_.get(node)
    }

    #[inline]
    unsafe fn mark_as_live(&mut self, node: *mut Node) {
        if (*node).IsDead() {
            return;
        }
        if !self.is_live(node) {
            self.is_live_.set(node, true);
            self.live_.push(node);
        }
    }

    fn graph(&self) -> *mut TFGraph {
        self.graph_
    }
}

impl Drop for GraphTrimmer {
    fn drop(&mut self) {}
}

struct NodeMarker<T> {
    map: std::collections::HashMap<*mut Node, T>,
}

impl<T: Copy> NodeMarker<T> {
    fn new(_graph: *mut TFGraph, _capacity: usize) -> Self {
        NodeMarker {
            map: std::collections::HashMap::new(),
        }
    }

    fn get(&self, node: *mut Node) -> bool
    where T: std::cmp::PartialEq {
        match self.map.get(&node) {
            Some(&value) => value == true,
            None => false,
        }
    }

    fn set(&mut self, node: *mut Node, value: T) {
        self.map.insert(node, value);
    }
}

struct NodeVector {
    nodes: Vec<*mut Node>,
}

impl NodeVector {
    fn with_capacity(capacity: usize) -> Self {
        NodeVector {
            nodes: Vec::with_capacity(capacity),
        }
    }

    fn reserve(&mut self, additional: usize) {
        self.nodes.reserve(additional);
    }

    fn size(&self) -> usize {
        self.nodes.len()
    }

    fn push(&mut self, node: *mut Node) {
        self.nodes.push(node);
    }

    fn get(&self, index: usize) -> *mut Node {
        self.nodes[index]
    }
}

struct Edge {
    from_: *mut Node,
    index_: usize,
}

impl Edge {
    fn from(&self) -> *mut Node {
        self.from_
    }

    fn index(&self) -> usize {
        self.index_
    }

    fn UpdateTo(&self, new_target: *mut Node) {
        unsafe {
            (*self.from_).ReplaceInput(self.index_ as i32, new_target);
        }
    }
}

struct TFGraph {}
impl TFGraph{
    unsafe fn end(&self) -> *mut Node {std::ptr::null_mut()}
    unsafe fn NodeCount(&self) -> usize {0}
}

struct Zone {}
struct Node {}
impl Node{
    unsafe fn IsDead(&self) -> bool{false}
    unsafe fn inputs(&self) -> Vec<*mut Node> {Vec::new()}
    unsafe fn use_edges(&self) -> Vec<Edge> {Vec::new()}
    unsafe fn ReplaceInput(&self, _i: i32, _n: *mut Node){}
}
struct StdoutStream {}
impl std::fmt::Write for StdoutStream {
    fn write_str(&mut self, _s: &str) -> std::fmt::Result {
        Ok(())
    }
}
impl Drop for StdoutStream {
    fn drop(&mut self) {}
}
impl StdoutStream {
    fn new() -> Self { Self {} }
}
impl std::ops::Shl<&str> for StdoutStream {
    type Output = Self;
    fn shl(self, _other: &str) -> Self {
        self
    }
}
impl std::ops::Shl<Node> for StdoutStream {
    type Output = Self;
    fn shl(self, _other: Node) -> Self {
        self
    }
}
impl std::ops::Shl<usize> for StdoutStream {
    type Output = Self;
    fn shl(self, _other: usize) -> Self {
        self
    }
}

struct Flags {
    trace_turbo_trimming: bool
}
static mut v8_flags: Flags = Flags{trace_turbo_trimming: false};

