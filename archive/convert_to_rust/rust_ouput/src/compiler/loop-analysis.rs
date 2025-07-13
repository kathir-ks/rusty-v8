// Converted from V8 C++ source files:
// Header: loop-analysis.h
// Implementation: loop-analysis.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(unused_variables)]
use std::cell::RefCell;
use std::rc::Rc;
mod base {
    pub mod iterator;
}
mod common {
    pub mod globals;
}
mod compiler {
    pub mod compiler_source_position_table;
    pub mod node_marker;
    pub mod node_origin_table;
    pub mod node_properties;
    pub mod node;
    pub mod turbofan_graph;
    pub mod common_operator;
}
mod zone {
    pub mod zone_containers;
}
mod codegen {
    pub mod tick_counter;
}

use self::base::iterator::iterator_range;
use self::common::globals::*;
use self::compiler::compiler_source_position_table::SourcePositionTable;
use self::compiler::node_marker::NodeMarker;
use self::compiler::node_origin_table::NodeOriginTable;
use self::compiler::node_properties::NodeProperties;
use self::compiler::node::Node;
use self::compiler::turbofan_graph::TFGraph;
use self::zone::zone_containers::*;
use self::codegen::tick_counter::TickCounter;
use self::compiler::common_operator::IrOpcode;
use self::compiler::node::Inputs;
use self::compiler::node::Uses;
use self::compiler::operator::Operator;

pub struct V8_EXPORT_PRIVATE {}

pub const kAssumedLoopEntryIndex: i32 = 0;

pub type NodeRange = iterator_range<*mut Node>;

pub struct LoopTree {
    zone_: *mut Zone,
    outer_loops_: ZoneVector<*mut Loop>,
    all_loops_: ZoneVector<Loop>,
    node_to_loop_num_: ZoneVector<i32>,
    loop_nodes_: ZoneVector<*mut Node>,
}

impl LoopTree {
    pub fn new(num_nodes: usize, zone: *mut Zone) -> Self {
        LoopTree {
            zone_: zone,
            outer_loops_: ZoneVector::new(zone),
            all_loops_: ZoneVector::new(zone),
            node_to_loop_num_: ZoneVector::from_elem(num_nodes, -1, zone),
            loop_nodes_: ZoneVector::new(zone),
        }
    }

    pub fn containing_loop(&mut self, node: *mut Node) -> Option<*mut Loop> {
        unsafe {
            if (*node).id() >= self.node_to_loop_num_.len() {
                return None;
            }
            let num = self.node_to_loop_num_[(*node).id()];
            if num > 0 {
                Some(&mut self.all_loops_[num as usize - 1] as *mut Loop)
            } else {
                None
            }
        }
    }

    pub fn contains(&mut self, loop_: *mut Loop, node: *mut Node) -> bool {
        let mut c = self.containing_loop(node);
        while let Some(current_loop) = c {
            if current_loop == loop_ {
                return true;
            }
            unsafe {
                c = self.containing_loop((*current_loop).parent_ as *mut Node);
            }
        }
        return false;
    }

    pub fn outer_loops(&self) -> &ZoneVector<*mut Loop> {
        &self.outer_loops_
    }

    pub fn inner_loops(&self) -> ZoneVector<*const Loop> {
        unsafe {
            let zone = self.zone_;
            let mut inner_loops: ZoneVector<*const Loop> = ZoneVector::new(zone);
            for loop_ in &self.all_loops_ {
                if loop_.children_.is_empty() {
                    inner_loops.push(loop_ as *const Loop);
                }
            }
            inner_loops
        }
    }

    pub fn loop_num(&self, loop_: *const Loop) -> i32 {
        unsafe {
            1 + (loop_ as *const Loop as usize - &self.all_loops_[0] as *const Loop as usize) as i32
                / std::mem::size_of::<Loop>() as i32
        }
    }

    pub fn header_nodes(&self, loop_: *const Loop) -> NodeRange {
        unsafe {
            NodeRange {
                begin: self.loop_nodes_.as_ptr().add((*loop_).header_start_),
                end: self.loop_nodes_.as_ptr().add((*loop_).body_start_),
            }
        }
    }

    pub fn header_node(&self, loop_: *const Loop) -> *mut Node {
        unsafe {
            for node in self.header_nodes(loop_) {
                if (*node).opcode() == IrOpcode::kLoop {
                    return node;
                }
            }
            panic!("Header node not found");
        }
    }

    pub fn body_nodes(&self, loop_: *const Loop) -> NodeRange {
        unsafe {
            NodeRange {
                begin: self.loop_nodes_.as_ptr().add((*loop_).body_start_),
                end: self.loop_nodes_.as_ptr().add((*loop_).exits_start_),
            }
        }
    }

    pub fn exit_nodes(&self, loop_: *const Loop) -> NodeRange {
        unsafe {
            NodeRange {
                begin: self.loop_nodes_.as_ptr().add((*loop_).exits_start_),
                end: self.loop_nodes_.as_ptr().add((*loop_).exits_end_),
            }
        }
    }

    pub fn loop_nodes(&self, loop_: *const Loop) -> NodeRange {
        unsafe {
            NodeRange {
                begin: self.loop_nodes_.as_ptr().add((*loop_).header_start_),
                end: self.loop_nodes_.as_ptr().add((*loop_).exits_end_),
            }
        }
    }

    pub fn get_loop_control(&self, loop_: *const Loop) -> *mut Node {
        unsafe {
            for node in self.header_nodes(loop_) {
                if (*node).opcode() == IrOpcode::kLoop {
                    return node;
                }
            }
            panic!("Unreachable");
        }
    }

    pub fn zone(&self) -> *mut Zone {
        self.zone_
    }

    fn new_loop(&mut self) -> *mut Loop {
        unsafe {
            let zone = self.zone_;
            self.all_loops_.push(Loop::new(zone));
            &mut self.all_loops_.last_mut().unwrap() as *mut Loop
        }
    }

    fn set_parent(&mut self, parent: *mut Loop, child: *mut Loop) {
        unsafe {
            if !parent.is_null() {
                (*parent).children_.push(child);
                (*child).parent_ = parent;
                (*child).depth_ = (*parent).depth_ + 1;
            } else {
                self.outer_loops_.push(child);
            }
        }
    }
}

impl Drop for LoopTree {
    fn drop(&mut self) {
    }
}

impl LoopTree {
    pub struct Loop {
        parent_: *mut Loop,
        depth_: i32,
        children_: ZoneVector<*mut Loop>,
        header_start_: i32,
        body_start_: i32,
        exits_start_: i32,
        exits_end_: i32,
    }

    impl Loop {
        fn new(zone: *mut Zone) -> Self {
            Loop {
                parent_: std::ptr::null_mut(),
                depth_: 0,
                children_: ZoneVector::new(zone),
                header_start_: -1,
                body_start_: -1,
                exits_start_: -1,
                exits_end_: -1,
            }
        }

        pub fn parent(&self) -> *mut Loop {
            self.parent_
        }

        pub fn children(&self) -> &ZoneVector<*mut Loop> {
            &self.children_
        }

        pub fn header_size(&self) -> u32 {
            (self.body_start_ - self.header_start_) as u32
        }

        pub fn body_size(&self) -> u32 {
            (self.exits_start_ - self.body_start_) as u32
        }

        pub fn exits_size(&self) -> u32 {
            (self.exits_end_ - self.exits_start_) as u32
        }

        pub fn total_size(&self) -> u32 {
            (self.exits_end_ - self.header_start_) as u32
        }

        pub fn depth(&self) -> u32 {
            self.depth_ as u32
        }
    }
}

pub struct LoopFinder {}

impl LoopFinder {
    pub fn build_loop_tree(
        graph: *mut TFGraph,
        tick_counter: *mut TickCounter,
        zone: *mut Zone,
    ) -> *mut LoopTree {
        unsafe {
            let loop_tree =
                (*graph).zone().new::<LoopTree>(LoopTree::new((*graph).node_count(), (*graph).zone()));
            let mut finder = LoopFinderImpl::new(graph, loop_tree, tick_counter, zone);
            finder.run();
            if v8_flags.trace_turbo_loop {
            }
            loop_tree
        }
    }

    pub fn has_marked_exits(loop_tree: *mut LoopTree, loop_: *const LoopTree::Loop) -> bool {
        unsafe {
            let loop_node = (*loop_tree).get_loop_control(loop_);
            for node in (*loop_tree).loop_nodes(loop_) {
                for use in (*node).uses() {
                    if !(*loop_tree).contains(loop_ as *mut LoopTree::Loop, use.from()) {
                        let unmarked_exit = match (*node).opcode() {
                            IrOpcode::kLoopExit => (*node).input_at(1) != loop_node,
                            IrOpcode::kLoopExitValue | IrOpcode::kLoopExitEffect => {
                                (*node).input_at(1).input_at(1) != loop_node
                            }
                            _ => (*use.from()).opcode() != IrOpcode::kTerminate,
                        };
                        if unmarked_exit {
                            return false;
                        }
                    }
                }
            }
            true
        }
    }

    #[cfg(V8_ENABLE_WEBASSEMBLY)]
    pub fn find_small_innermost_loop_from_header(
        loop_header: *mut Node,
        all_nodes: &mut AllNodes,
        zone: *mut Zone,
        max_size: usize,
        purpose: Purpose,
    ) -> Option<*mut ZoneUnorderedSet<*mut Node>> {
        unsafe {
            let visited = (*zone).new::<ZoneUnorderedSet<*mut Node>>(ZoneUnorderedSet::new(zone));
            let mut queue: Vec<*mut Node> = Vec::new();

            if (*loop_header).opcode() != IrOpcode::kLoop {
                return None;
            }

            queue.push(loop_header);
            visited.insert(loop_header);

            macro_rules! enqueue_uses {
                ($use_name:ident, $condition:expr) => {
                    for $use_name in (*node).uses() {
                        if $condition && !visited.contains($use_name.from()) {
                            visited.insert($use_name.from());
                            queue.push($use_name.from());
                        }
                    }
                };
            }

            let mut has_instruction_worth_peeling = false;
            while !queue.is_empty() {
                let mut node = queue.pop().unwrap();
                if (*node).opcode() == IrOpcode::kEnd {
                    visited.remove(node);
                    continue;
                }
                if visited.len() > max_size {
                    return None;
                }
                match (*node).opcode() {
                    IrOpcode::kLoop => {
                        if node != loop_header {
                            return None;
                        }
                        enqueue_uses!(use, true);
                    }
                    IrOpcode::kLoopExit => {
                        if (*node).input_at(1) != loop_header {
                            return None;
                        }
                        enqueue_uses!(use, ((*use.from()).opcode() == IrOpcode::kLoopExitEffect
                            || (*use.from()).opcode() == IrOpcode::kLoopExitValue));
                    }
                    IrOpcode::kLoopExitEffect | IrOpcode::kLoopExitValue => {
                        if (*(*NodeProperties::get_control_input(node)).input_at(1)) != loop_header {
                            return None;
                        }
                    }
                    IrOpcode::kTailCall | IrOpcode::kJSWasmCall | IrOpcode::kJSCall => {
                        if purpose == Purpose::kLoopUnrolling {
                            return None;
                        }
                        enqueue_uses!(use, true);
                    }
                    IrOpcode::kCall => {
                        if purpose == Purpose::kLoopPeeling {
                            enqueue_uses!(use, true);
                            break;
                        }
                        let callee = (*node).input_at(0);
                        if (*callee).opcode() != IrOpcode::kRelocatableInt32Constant
                            && (*callee).opcode() != IrOpcode::kRelocatableInt64Constant
                        {
                            return None;
                        }

                        enqueue_uses!(use, true);
                        break;
                    }
                    IrOpcode::kWasmStructGet => {
                        let object = (*node).input_at(0);
                        if (*object).opcode() == IrOpcode::kWasmStructGet && visited.contains(object) {
                            has_instruction_worth_peeling = true;
                        }
                        enqueue_uses!(use, true);
                        break;
                    }
                    IrOpcode::kWasmArrayGet => {
                        has_instruction_worth_peeling = true;
                    }
                    IrOpcode::kStringPrepareForGetCodeunit => {
                        has_instruction_worth_peeling = true;
                        enqueue_uses!(use, true);
                        break;
                    }
                    _ => {
                        enqueue_uses!(use, true);
                        break;
                    }
                }
            }

            for node in &*visited {
                if node == &loop_header {
                    continue;
                }

                if !all_nodes.is_live(*node) {
                    continue;
                }

                for edge in (*node).input_edges() {
                    let input = edge.to();
                    if NodeProperties::is_control_edge(&edge) && !visited.contains(input)
                        && (*input).opcode() != IrOpcode::kStart
                    {
                        panic!(
                            "Floating control detected in wasm turbofan graph: Node #{} is \
                             inside loop headed by #{}, but its control dependency #{} is outside",
                            (*node).id(),
                            (*loop_header).id(),
                            (*input).id()
                        );
                    }
                }
            }

            if purpose == Purpose::kLoopPeeling && !has_instruction_worth_peeling {
                return None;
            }
            Some(visited)
        }
    }
}

#[cfg(V8_ENABLE_WEBASSEMBLY)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Purpose {
    kLoopPeeling,
    kLoopUnrolling,
}

struct NodeInfo {
    node: *mut Node,
    next: *mut NodeInfo,
    backwards_visited: bool,
}

struct TempLoopInfo {
    header: *mut Node,
    header_list: *mut NodeInfo,
    exit_list: *mut NodeInfo,
    body_list: *mut NodeInfo,
    loop_: *mut LoopTree::Loop,
}

struct LoopFinderImpl {
    zone_: *mut Zone,
    end_: *mut Node,
    queue_: NodeDeque,
    queued_: NodeMarker<bool>,
    info_: ZoneVector<NodeInfo>,
    loops_: ZoneVector<TempLoopInfo>,
    loop_num_: ZoneVector<i32>,
    loop_tree_: *mut LoopTree,
    loops_found_: i32,
    width_: i32,
    backward_: *mut u32,
    forward_: *mut u32,
    tick_counter_: *mut TickCounter,
}

impl LoopFinderImpl {
    fn new(
        graph: *mut TFGraph,
        loop_tree: *mut LoopTree,
        tick_counter: *mut TickCounter,
        zone: *mut Zone,
    ) -> Self {
        unsafe {
            LoopFinderImpl {
                zone_: zone,
                end_: (*graph).end(),
                queue_: NodeDeque::new(zone),
                queued_: NodeMarker::new(graph, 2),
                info_: ZoneVector::from_fn((*graph).node_count(), || NodeInfo {
                    node: std::ptr::null_mut(),
                    next: std::ptr::null_mut(),
                    backwards_visited: false,
                }, zone),
                loops_: ZoneVector::new(zone),
                loop_num_: ZoneVector::from_elem((*graph).node_count(), -1, zone),
                loop_tree_: loop_tree,
                loops_found_: 0,
                width_: 0,
                backward_: std::ptr::null_mut(),
                forward_: std::ptr::null_mut(),
                tick_counter_: tick_counter,
            }
        }
    }

    fn run(&mut self) {
        self.propagate_backward();
        self.propagate_forward();
        self.finish_loop_tree();
    }

    fn num_nodes(&self) -> i32 {
        unsafe { (*self.loop_tree_).node_to_loop_num_.len() as i32 }
    }

    fn propagate_backward_marks(&mut self, from: *mut Node, to: *mut Node, loop_filter: i32) -> bool {
        unsafe {
            if from == to {
                return false;
            }
            let fp = self.backward_.add((*from).id() * self.width_ as usize);
            let tp = self.backward_.add((*to).id() * self.width_ as usize);
            let mut change = false;
            for i in 0..self.width_ {
                let mask: u32 = if i == INDEX(loop_filter) {
                    !BIT(loop_filter)
                } else {
                    0xFFFFFFFF
                };
                let prev = *tp.add(i as usize);
                let next = prev | (*fp.add(i as usize) & mask);
                *tp.add(i as usize) = next;
                if !change && (prev != next) {
                    change = true;
                }
            }
            change
        }
    }

    fn set_backward_mark(&mut self, to: *mut Node, loop_num: i32) -> bool {
        unsafe {
            let tp = self.backward_.add((*to).id() * self.width_ as usize + INDEX(loop_num) as usize);
            let prev = *tp;
            let next = prev | BIT(loop_num);
            *tp = next;
            next != prev
        }
    }

    fn set_forward_mark(&mut self, to: *mut Node, loop_num: i32) -> bool {
        unsafe {
            let tp = self.forward_.add((*to).id() * self.width_ as usize + INDEX(loop_num) as usize);
            let prev = *tp;
            let next = prev | BIT(loop_num);
            *tp = next;
            next != prev
        }
    }

    fn propagate_forward_marks(&mut self, from: *mut Node, to: *mut Node) -> bool {
        unsafe {
            if from == to {
                return false;
            }
            let mut change = false;
            let findex = (*from).id() * self.width_ as usize;
            let tindex = (*to).id() * self.width_ as usize;
            for i in 0..self.width_ {
                let marks = *self.backward_.add(tindex + i as usize) & *self.forward_.add(findex + i as usize);
                let prev = *self.forward_.add(tindex + i as usize);
                let next = prev | marks;
                *self.forward_.add(tindex + i as usize) = next;
                if !change && (prev != next) {
                    change = true;
                }
            }
            change
        }
    }

    fn is_in_loop(&mut self, node: *mut Node, loop_num: i32) -> bool {
        unsafe {
            let offset = (*node).id() * self.width_ as usize + INDEX(loop_num) as usize;
            *self.backward_.add(offset) & *self.forward_.add(offset) & BIT(loop_num)
        }
    }

    fn propagate_backward(&mut self) {
        self.resize_backward_marks();
        self.set_backward_mark(self.end_, 0);
        self.queue(self.end_);

        unsafe {
            while !self.queue_.is_empty() {
                (*self.tick_counter_).tick_and_maybe_enter_safepoint();
                let node = self.queue_.pop_front().unwrap();
                self.info(node).backwards_visited = true;
                self.queued_.set(node, false);

                let mut loop_num = -1;
                if (*node).opcode() == IrOpcode::kLoop {
                    loop_num = self.create_loop_info(node);
                } else if NodeProperties::is_phi(node) {
                    let merge = (*node).input_at((*node).input_count() - 1);
                    if (*merge).opcode() == IrOpcode::kLoop {
                        loop_num = self.create_loop_info(merge);
                    }
                } else if (*node).opcode() == IrOpcode::kLoopExit {
                    self.create_loop_info((*node).input_at(1));
                } else if (*node).opcode() == IrOpcode::kLoopExitValue ||
                    (*node).opcode() == IrOpcode::kLoopExitEffect {
                    let loop_exit = NodeProperties::get_control_input(node);
                    self.create_loop_info((*loop_exit).input_at(1));
                }

                for i in 0..(*node).input_count() {
                    let input = (*node).input_at(i);
                    if self.is_backedge(node, i as i32) {
                        if self.set_backward_mark(input, loop_num) ||
                            !self.info(input).backwards_visited {
                            self.queue(input);
                        }
                    } else {
                        if self.propagate_backward_marks(node, input, loop_num) ||
                            !self.info(input).backwards_visited {
                            self.queue(input);
                        }
                    }
                }
            }
        }
    }

    fn create_loop_info(&mut self, node: *mut Node) -> i32 {
        unsafe {
            if (*node).opcode() != IrOpcode::kLoop {
                return 0;
            }
            let mut loop_num = self.loop_num(node);
            if loop_num > 0 {
                return loop_num;
            }

            self.loops_found_ += 1;
            loop_num = self.loops_found_;
            if INDEX(loop_num) >= self.width_ {
                self.resize_backward_marks();
            }

            self.loops_.push(TempLoopInfo {
                header: node,
                header_list: std::ptr::null_mut(),
                exit_list: std::ptr::null_mut(),
                body_list: std::ptr::null_mut(),
                loop_: std::ptr::null_mut(),
            });
            (*self.loop_tree_).new_loop();
            self.set_loop_mark_for_loop_header(node, loop_num);
            loop_num
        }
    }

    fn set_loop_mark(&mut self, node: *mut Node, loop_num: i32) {
        unsafe {
            self.info(node);
            self.set_backward_mark(node, loop_num);
            (*self.loop_tree_).node_to_loop_num_[(*node).id()] = loop_num;
        }
    }

    fn set_loop_mark_for_loop_header(&mut self, node: *mut Node, loop_num: i32) {
        unsafe {
            if (*node).opcode() != IrOpcode::kLoop {
                return;
            }
            self.set_loop_mark(node, loop_num);
            for use in (*node).uses() {
                if NodeProperties::is_phi(use.from()) {
                    self.set_loop_mark(use.from(), loop_num);
                }

                if (*node).input_count() <= 1 {
                    continue;
                }

                if (*use.from()).opcode() == IrOpcode::kLoopExit {
                    self.set_loop_mark(use.from(), loop_num);
                    for exit_use in (*use.from()).uses() {
                        if (*exit_use.from()).opcode() == IrOpcode::kLoopExitValue ||
                            (*exit_use.from()).opcode() == IrOpcode::kLoopExitEffect {
                            self.set_loop_mark(exit_use.from(), loop_num);
                        }
                    }
                }
            }
        }
    }

    fn resize_backward_marks(&mut self) {
        unsafe {
            let new_width = self.width_ + 1;
            let max = self.num_nodes() as usize;
            let new_backward = (*self.zone_).allocate_array::<u32>((new_width * max) as usize);
            std::ptr::write_bytes(new_backward, 0, (new_width * max) as usize * std::mem::size_of::<u32>());
            if self.width_ > 0 {
                for i in 0..max {
                    let np = new_backward.add(i * new_width as usize);
                    let op = self.backward_.add(i * self.width_ as usize);
                    for j in 0..self.width_ {
                        *np.add(j as usize) = *op.add(j as usize);
                    }
                }
            }
            self.width_ = new_width;
            self.backward_ = new_backward;
        }
    }

    fn resize_forward_marks(&mut self) {
        unsafe {
            let max = self.num_nodes() as usize;
            self.forward_ = (*self.zone_).allocate_array::<u32>((self.width_ as usize) * max);
            std::ptr::write_bytes(self.forward_, 0, (self.width_ as usize) * max * std::mem::size_of::<u32>());
        }
    }

    fn propagate_forward(&mut self) {
        self.resize_forward_marks();
        unsafe {
            for li in &self.loops_ {
                self.set_forward_mark(li.header, self.loop_num(li.header));
                self.queue(li.header);
            }

            while !self.queue_.is_empty() {
                (*self.tick_counter_).tick_and_maybe_enter_safepoint();
                let node = self.queue_.pop_front().unwrap();
                self.queued_.set(node, false);
                for edge in (*node).use_edges() {
                    let use = edge.from();
                    if !self.is_backedge(use, edge.index()) {
                        if self.propagate_forward_marks(node, use) {
                            self.queue(use);
                        }
                    }
                }
            }
        }
    }

    fn is_loop_header_node(&self, node: *mut Node) -> bool {
        unsafe {
            (*node).opcode() == IrOpcode::kLoop || NodeProperties::is_phi(node)
        }
    }

    fn is_loop_exit_node(&self, node: *mut Node) -> bool {
        unsafe {
            (*node).opcode() == IrOpcode::kLoopExit ||
                (*node).opcode() == IrOpcode::kLoopExitValue ||
                (*node).opcode() == IrOpcode::kLoopExitEffect
        }
    }

    fn is_backedge(&self, use: *mut Node, index: i32) -> bool {
        unsafe {
            if self.loop_num(use) <= 0 {
                return false;
            }
            if NodeProperties::is_phi(use) {
                return index != NodeProperties::first_control_index(use) &&
                    index != kAssumedLoopEntryIndex;
            } else if (*use).opcode() == IrOpcode::kLoop {
                return index != kAssumedLoopEntryIndex;
            }
            if !self.is_loop_exit_node(use) {
                return false;
            }
            false
        }
    }

    fn loop_num(&self, node: *mut Node) -> i32 {
        unsafe {
             (*self.loop_tree_).node_to_loop_num_[(*node).id()]
        }
    }

    fn info(&mut self, node: *mut Node) -> &mut NodeInfo {
        unsafe {
            let i = &mut self.info_[(*node).id()];
            if i.node.is_null() {
                i.node = node;
            }
            i
        }
    }

    fn queue(&mut self, node: *mut Node) {
        unsafe {
            if !self.queued_.get(node) {
                self.queue_.push_back(node);
                self.queued_.set(node, true);
            }
        }
    }

    fn add_node_to_loop(&mut self, node_info: *mut NodeInfo, loop_: *mut TempLoopInfo, loop_num: i32) {
        unsafe {
            if self.loop_num((*node_info).node) == loop_num {
                if self.is_loop_header_node((*node_info).node) {
                    (*node_info).next = (*loop_).header_list;
                    (*loop_).header_list = node_info;
                } else {
                    if !self.is_loop_exit_node((*node_info).node) {
                       panic!("DCHECK(IsLoopExitNode(node_info->node));");
                    }
                    (*node_info).next = (*loop_).exit_list;
                    (*loop_).exit_list = node_info;
                }
            } else {
                (*node_info).next = (*loop_).body_list;
                (*loop_).body_list = node_info;
            }
        }
    }

    fn finish_loop_tree(&mut self) {
        unsafe {
            if self.loops_found_ != self.loops_.len() as i32 {
                panic!("DCHECK(loops_found_ == static_cast<int>(loops_.size()));");
            }
            if self.loops_found_ != (*self.loop_tree_).all_loops_.len() as i32 {
                panic!("DCHECK(loops_found_ == static_cast<int>(loop_tree_->all_loops_.size()));");
            }

            if self.loops_found_ == 0 {
                return;
            }
            if self.loops_found_ == 1 {
                self.finish_single_loop();
                return;
            }

            for i in 1..=self.loops_found_ {
                self.connect_loop_tree(i);
            }

            let mut count: usize = 0;
            for ni in &mut self.info_ {
                if ni.node.is_null() {
                    continue;
                }

                let mut innermost: *mut TempLoopInfo = std::ptr::null_mut();
                let mut innermost_index: i32 = 0;
                let pos = (*ni.node).id() * self.width_ as usize;
                for i in 0..self.width_ {
                    let marks = *self.backward_.add(pos + i as usize) & *self.forward_.add(pos + i as usize);

                    for j in 0..32 {
                        if marks & (1u32 << j) != 0 {
                            let loop_num = i * 32 + j as i32;
                            if loop_num == 0 {
                                continue;
                            }
                            let loop_ = &mut self.loops_[loop_num as usize - 1];
                            if innermost.is_null() || (*loop_.loop_).depth_ > unsafe {(*innermost).loop_}.depth_ {
                                innermost = loop_;
                                innermost_index = loop_num;
                            }
                        }
                    }
                }
                if innermost.is_null() {
                    continue;
                }

                if (*ni.node).opcode() == IrOpcode::kReturn {
                    panic!("CHECK(ni.node->opcode() != IrOpcode::kReturn);");
                }

                self.add_node_to_loop(ni, innermost, innermost_index);
                count += 1;
            }

            (*self.loop_tree_).loop_nodes_.reserve(count);
            for loop_ in &*(*self.loop_tree_).outer_loops_ {
                self.serialize_loop(*loop_);
            }
        }
    }

    fn finish_single_
