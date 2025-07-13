// Converted from V8 C++ source files:
// Header: control-equivalence.h
// Implementation: control-equivalence.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
pub mod compiler_specific {

}
}

pub mod common {
pub mod globals {

}
}

pub mod compiler {
pub mod node {
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Node {
        id_: usize,
    }

    impl Node {
        pub fn id(&self) -> usize {
            self.id_
        }

        pub fn op(&self) -> &Operator {
            &Operator{}
        }

         pub fn input_edges(&self) -> InputEdges {
            InputEdges {
                edges: vec![],
            }
        }

        pub fn use_edges(&self) -> UseEdges {
            UseEdges {
                edges: vec![],
            }
        }

        pub fn InputAt(&self, _i: i32) -> *mut Node {
            std::ptr::null_mut()
        }
}

pub struct InputEdges {
    edges: Vec<Edge>,
}

impl InputEdges {
    pub fn begin(&self) -> std::vec::IntoIter<Edge> {
        self.edges.clone().into_iter()
    }

    pub fn end(&self) -> std::vec::IntoIter<Edge> {
       self.edges.clone().into_iter()
    }
}

pub struct UseEdges {
    edges: Vec<Edge>,
}

impl UseEdges {
    pub fn begin(&self) -> std::vec::IntoIter<Edge> {
        self.edges.clone().into_iter()
    }

     pub fn end(&self) -> std::vec::IntoIter<Edge> {
       self.edges.clone().into_iter()
    }
}

#[derive(Clone)]
pub struct Edge {
    to_: *mut Node,
    from_: *mut Node,
}

impl Edge {
    pub fn to(&self) -> *mut Node {
        self.to_
    }

    pub fn from(&self) -> *mut Node {
        self.from_
    }
}
}
pub mod turbofan_graph {
use super::node::Node;

#[derive(Debug)]
pub struct TFGraph {
        node_count: usize,
        end: Node,
    }

    impl TFGraph {
        pub fn new(node_count: usize) -> Self {
            TFGraph {
                node_count,
                end: Node{ id_: 0 },
            }
        }

        pub fn NodeCount(&self) -> usize {
            self.node_count
        }

        pub fn end(&self) -> &Node {
            &self.end
        }
    }
}
pub mod operator {
    #[derive(Debug)]
    pub struct Operator {}

    impl Operator {
        pub fn mnemonic(&self) -> &'static str {
            "Operator"
        }
    }
}
pub mod node_properties {
    use super::node::Edge;

    pub fn IsControlEdge(_edge: Edge) -> bool {
        true
    }

    pub fn FirstControlIndex(_node: *mut super::node::Node) -> i32 {
        0
    }

    pub fn PastControlIndex(_node: *mut super::node::Node) -> i32 {
        1
    }
}
pub mod control_equivalence {
use std::cell::RefCell;
use std::rc::Rc;
use crate::base::compiler_specific::*;
use crate::common::globals::*;
use crate::compiler::node::*;
use crate::compiler::turbofan_graph::*;
use crate::zone::zone_containers::*;
use crate::compiler::operator::Operator;
use crate::compiler::node_properties;

#[derive(Debug)]
pub struct ControlEquivalence {
    zone_: *mut Zone,
    graph_: *mut TFGraph,
    dfs_number_: i32,
    class_number_: i32,
    node_data_: Vec<*mut NodeData>,
}

impl ControlEquivalence {
    pub fn new(zone: *mut Zone, graph: *mut TFGraph) -> Self {
        let graph_ref = unsafe { &*graph };
        let node_count = graph_ref.NodeCount();
        ControlEquivalence {
            zone_: zone,
            graph_: graph,
            dfs_number_: 0,
            class_number_: 1,
            node_data_: vec![std::ptr::null_mut(); node_count],
        }
    }

    pub fn run(&mut self, exit: *mut Node) {
        let exit_ref = unsafe { &*exit };
        if !self.participates(exit) || self.get_class(exit) == Self::K_INVALID_CLASS {
            self.determine_participation(exit);
            self.run_undirected_dfs(exit);
        }
    }

    pub fn class_of(&self, node: *mut Node) -> usize {
        let node_ref = unsafe { &*node };
        assert_ne!(Self::K_INVALID_CLASS, self.get_class(node));
        self.get_class(node)
    }

    const K_INVALID_CLASS: usize = usize::MAX;

    fn visit_pre(&self, node: *mut Node) {
        let node_ref = unsafe { &*node };
        println!("CEQ: Pre-visit of #{}:{}", node_ref.id(), node_ref.op().mnemonic());
    }

    fn visit_mid(&mut self, node: *mut Node, direction: DFSDirection) {
        let node_ref = unsafe { &*node };
        println!("CEQ: Mid-visit of #{}:{}", node_ref.id(), node_ref.op().mnemonic());
        let blist = self.get_bracket_list(node);

        self.bracket_list_delete(blist, node, direction);

        if blist.is_empty() {
            assert_eq!(DFSDirection::KInputDirection, direction);
            self.visit_backedge(node, unsafe { &*self.graph_ }.end(), DFSDirection::KInputDirection);
        }

        self.bracket_list_trace(blist);
        let recent = blist.last().unwrap();
        let mut_recent = blist.last_mut().unwrap();
        if recent.recent_size != blist.len() {
            mut_recent.recent_size = blist.len();
            mut_recent.recent_class = self.new_class_number() as usize;
        }

        self.set_class(node, recent.recent_class);
        println!("  Assigned class number is {}", self.get_class(node));
    }

    fn visit_post(&mut self, node: *mut Node, parent_node: *mut Node, direction: DFSDirection) {
        let node_ref = unsafe { &*node };
        println!("CEQ: Post-visit of #{}:{}", node_ref.id(), node_ref.op().mnemonic());
        let blist = self.get_bracket_list(node);

        self.bracket_list_delete(blist, node, direction);

        if !parent_node.is_null() {
            let parent_blist = self.get_bracket_list(parent_node);
            let mut parent_blist_mut = unsafe { &mut *parent_blist };
            for bracket in blist.iter() {
                parent_blist_mut.push(bracket.clone());
            }
        }
    }

    fn visit_backedge(&mut self, from: *mut Node, to: &Node, direction: DFSDirection) {
        let from_ref = unsafe { &*from };
        println!(
            "CEQ: Backedge from #{}:{} to #{}:{}",
            from_ref.id(),
            from_ref.op().mnemonic(),
            to.id(),
            Operator{}.mnemonic()
        );

        let bracket = Bracket {
            direction,
            recent_class: Self::K_INVALID_CLASS,
            recent_size: 0,
            from,
            to: unsafe { &*to },
        };
        let blist = self.get_bracket_list(from);
        let mut blist_mut = unsafe { &mut *blist };
        blist_mut.push(bracket);
    }

    fn run_undirected_dfs(&mut self, exit: *mut Node) {
        let zone = unsafe { &mut *self.zone_ };
        let mut stack: ZoneStack<DFSStackEntry> = ZoneStack::new(zone);

        self.dfs_push(&mut stack, exit, std::ptr::null_mut(), DFSDirection::KInputDirection);
        self.visit_pre(exit);

        while !stack.is_empty() {
            let mut entry = stack.pop().unwrap();
            let node = entry.node;
            let node_ref = unsafe { &*node };

            if entry.direction == DFSDirection::KInputDirection {
                if entry.input_index < node_ref.input_edges().edges.len() {
                   let edge = node_ref.input_edges().edges[entry.input_index].clone();
                    let input = edge.to();
                    entry.input_index += 1;
                     stack.push(entry); 

                    if node_properties::IsControlEdge(edge) {
                        if !self.participates(input) {
                            continue;
                        }
                        let input_data = self.get_data(input);
                        let input_data_ref = unsafe { &*input_data };
                        if input_data_ref.visited {
                            continue;
                        }
                         if input_data_ref.on_stack {
                            if input != entry.parent_node {
                                self.visit_backedge(node, unsafe { &*input }, DFSDirection::KInputDirection);
                            }
                         } else {
                             self.dfs_push(&mut stack, input, node, DFSDirection::KInputDirection);
                             self.visit_pre(input);
                         }
                    }
                    continue;
                }
                 if entry.use_index < node_ref.use_edges().edges.len() {
                    entry.direction = DFSDirection::KUseDirection;
                    self.visit_mid(node, DFSDirection::KInputDirection);
                    stack.push(entry); 
                    continue;
                }
            }

            if entry.direction == DFSDirection::KUseDirection {
                if entry.use_index < node_ref.use_edges().edges.len() {
                   let edge = node_ref.use_edges().edges[entry.use_index].clone();
                    let use = edge.from();
                    entry.use_index += 1;
                     stack.push(entry); 

                     if node_properties::IsControlEdge(edge) {
                         if !self.participates(use) {
                            continue;
                         }
                        let use_data = self.get_data(use);
                        let use_data_ref = unsafe { &*use_data };
                        if use_data_ref.visited {
                            continue;
                         }
                         if use_data_ref.on_stack {
                            if use != entry.parent_node {
                                self.visit_backedge(node, unsafe { &*use }, DFSDirection::KUseDirection);
                            }
                         } else {
                             self.dfs_push(&mut stack, use, node, DFSDirection::KUseDirection);
                             self.visit_pre(use);
                         }
                    }
                    continue;
                }
                 if entry.input_index < node_ref.input_edges().edges.len() {
                    entry.direction = DFSDirection::KInputDirection;
                    self.visit_mid(node, DFSDirection::KUseDirection);
                    stack.push(entry); 
                    continue;
                }
            }

            self.visit_post(node, entry.parent_node, entry.direction);
             self.dfs_pop(&mut stack, node);
        }
    }

    fn determine_participation_enqueue(&mut self, queue: &mut ZoneQueue<*mut Node>, node: *mut Node) {
        if !self.participates(node) {
            self.allocate_data(node);
            queue.push(node);
        }
    }

    fn determine_participation(&mut self, exit: *mut Node) {
        let zone = unsafe { &mut *self.zone_ };
        let mut queue: ZoneQueue<*mut Node> = ZoneQueue::new(zone);
        self.determine_participation_enqueue(&mut queue, exit);

        while !queue.is_empty() {
            let node = queue.pop().unwrap();
            let max = node_properties::PastControlIndex(node);
            for i in node_properties::FirstControlIndex(node)..max {
                self.determine_participation_enqueue(&mut queue, unsafe { &*node }.InputAt(i));
            }
        }
    }

    fn dfs_push(&mut self, stack: &mut ZoneStack<DFSStackEntry>, node: *mut Node, from: *mut Node, dir: DFSDirection) {
        let node_ref = unsafe { &*node };
        assert!(self.participates(node));
        assert!(!unsafe { &*self.get_data(node) }.visited);
         let node_data = self.get_data(node);
         unsafe { &mut *node_data }.on_stack = true;
        let input_edges = node_ref.input_edges();
        let use_edges = node_ref.use_edges();
        stack.push(DFSStackEntry {
            direction: dir,
            input_index: 0,
            use_index: 0,
            parent_node: from,
            node,
        });
    }

    fn dfs_pop(&mut self, stack: &mut ZoneStack<DFSStackEntry>, node: *mut Node) {
         let top_entry = stack.last().unwrap();
         assert_eq!(top_entry.node, node);
         let node_data = self.get_data(node);
         unsafe { &mut *node_data }.on_stack = false;
         unsafe { &mut *node_data }.visited = true;
        stack.pop();
    }

    fn bracket_list_delete(&mut self, blist: *mut Vec<Bracket>, to: *mut Node, direction: DFSDirection) {
        let mut i = 0;
        let blist_ref = unsafe { &mut *blist };

        while i < blist_ref.len() {
            if blist_ref[i].to as *const _ == to as *const _ && blist_ref[i].direction != direction {
                let from_id = unsafe { &*blist_ref[i].from }.id();
                let to_id = unsafe { &*blist_ref[i].to }.id();
                println!("  BList erased: {{{}->{}}}\n", from_id, to_id);
                blist_ref.remove(i);
            } else {
                i += 1;
            }
        }
    }

    fn bracket_list_trace(&self, blist: *mut Vec<Bracket>) {
          if true {
            println!("  BList: ");
             let blist_ref = unsafe { &*blist };
            for bracket in blist_ref {
                let from_id = unsafe { &*bracket.from }.id();
                let to_id = unsafe { &*bracket.to }.id();
                println!("{{{0}->{1}}} ", from_id, to_id);
            }
            println!("\n");
        }
    }

    fn get_data(&self, node: *mut Node) -> *mut NodeData {
        let index = unsafe { &*node }.id();
        if index >= self.node_data_.len() {
            return std::ptr::null_mut();
        }
        self.node_data_[index]
    }

    fn allocate_data(&mut self, node: *mut Node) {
        let index = unsafe { &*node }.id();
        if index >= self.node_data_.len() {
             self.node_data_.resize(index + 1, std::ptr::null_mut());
        }
        let zone = unsafe { &mut *self.zone_ };
        self.node_data_[index] = zone.new_node_data(zone);
    }

    fn new_class_number(&mut self) -> i32 {
        self.class_number_ += 1;
        self.class_number_ - 1
    }

    fn new_dfs_number(&mut self) -> i32 {
        self.dfs_number_ += 1;
        self.dfs_number_ - 1
    }

    fn participates(&self, node: *mut Node) -> bool {
        !self.get_data(node).is_null()
    }

    fn get_class(&self, node: *mut Node) -> usize {
        let data = self.get_data(node);
        if data.is_null() {
            return Self::K_INVALID_CLASS;
        }
        unsafe { &*data }.class_number
    }

    fn set_class(&mut self, node: *mut Node, number: usize) {
        assert!(self.participates(node));
         let data = self.get_data(node);
         unsafe { &mut *data }.class_number = number;
    }

    fn get_bracket_list(&self, node: *mut Node) -> *mut Vec<Bracket> {
       let data = self.get_data(node);
       unsafe { &mut (*data).blist }
    }

    fn set_bracket_list(&mut self, node: *mut Node, list: *mut Vec<Bracket>) {
        assert!(self.participates(node));
         let data = self.get_data(node);
         unsafe { &mut (*data).blist = unsafe { &mut *list };
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum DFSDirection {
    KInputDirection,
    KUseDirection,
}

#[derive(Debug, Clone)]
struct Bracket {
    direction: DFSDirection,
    recent_class: usize,
    recent_size: usize,
    from: *mut Node,
    to: *const Node,
}

#[derive(Debug)]
struct DFSStackEntry {
    direction: DFSDirection,
    input_index: usize,
    use_index: usize,
    parent_node: *mut Node,
    node: *mut Node,
}

#[derive(Debug)]
struct NodeData {
    class_number: usize,
    blist: Vec<Bracket>,
    visited: bool,
    on_stack: bool,
}

impl NodeData {
    fn new() -> Self {
        NodeData {
            class_number: ControlEquivalence::K_INVALID_CLASS,
            blist: Vec::new(),
            visited: false,
            on_stack: false,
        }
    }
}
}  // namespace compiler
}  // namespace internal
}  // namespace v8

pub mod zone {
pub mod zone_containers {
use std::cell::RefCell;
use std::rc::Rc;
use crate::compiler::control_equivalence::NodeData;

#[derive(Debug)]
pub struct Zone {}

impl Zone {
    pub fn new() -> Self {
        Zone {}
    }
     pub fn new_node_data(&mut self, zone: &Zone) -> *mut NodeData {
        Box::into_raw(Box::new(NodeData::new()))
    }
}

#[derive(Debug)]
pub struct ZoneQueue<T> {
    queue: Vec<T>,
    zone: *mut Zone,
}

impl<T> ZoneQueue<T> {
    pub fn new(zone: *mut Zone) -> Self {
        ZoneQueue {
            queue: Vec::new(),
            zone,
        }
    }

    pub fn push(&mut self, value: T) {
        self.queue.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.queue.is_empty() {
            return None;
        }
        Some(self.queue.remove(0))
    }

    pub fn empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

#[derive(Debug)]
pub struct ZoneStack<T> {
    stack: Vec<T>,
    zone: *mut Zone,
}

impl<T> ZoneStack<T> {
    pub fn new(zone: *mut Zone) -> Self {
        ZoneStack {
            stack: Vec::new(),
            zone,
        }
    }

    pub fn push(&mut self, value: T) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    pub fn top(&mut self) -> &mut T {
        self.stack.last_mut().unwrap()
    }

    pub fn empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

     pub fn last(&self) -> Option<&T> {
        self.stack.last()
    }
}
}  // namespace zone_containers
}  // namespace zone
