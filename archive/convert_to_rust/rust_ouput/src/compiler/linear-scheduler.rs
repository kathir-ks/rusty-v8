// Converted from V8 C++ source files:
// Header: linear-scheduler.h
// Implementation: linear-scheduler.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr;
use std::rc::Rc;

//use crate::base::flags::Flag;
//use crate::common::globals::FLAG_trace_turbo;
//use crate::compiler::backend::code_generator::CodeGenerator;
//use crate::compiler::common_operator::Operator;
use crate::compiler::node::Node;
//use crate::compiler::node_properties::AllUses;
use crate::compiler::opcodes::IrOpcode;
//use crate::compiler::schedule::BasicBlock;
//use crate::compiler::scheduler::Scheduler;
use crate::compiler::turbofan_graph::Graph as TFGraph;
//use crate::compiler::turbofan_graph::Graph;
//use crate::execution::isolate::Isolate;
//use crate::objects::js_function::SharedFunctionInfo;
//use crate::objects::shared_function_info::SharedFunctionInfoRef;
use crate::zone::zone::Zone;
//use crate::zone::zone_containers::Deq;
use crate::compiler::node_properties::NodeProperties;

pub struct LinearScheduler<'z> {
    graph_: *mut TFGraph,
    control_level_: HashMap<*mut Node, i32>,
    early_schedule_position_: HashMap<*mut Node, *mut Node>,
    zone: &'z Zone,
}

impl<'z> LinearScheduler<'z> {
    pub fn new(zone: &'z Zone, graph: *mut TFGraph) -> Self {
        let mut scheduler = LinearScheduler {
            graph_: graph,
            control_level_: HashMap::new(),
            early_schedule_position_: HashMap::new(),
            zone,
        };
        scheduler.compute_control_level();
        scheduler
    }

    fn compute_control_level(&mut self) {
        unsafe {
            let start = (*self.graph_).start();
            self.set_control_level(start, 0);

            let mut queue: VecDeque<*mut Node> = VecDeque::new();
            queue.push_back(start);

            while !queue.is_empty() {
                let node = queue.pop_front().unwrap();
                let level = self.get_control_level(node);

                let node_ref = &*node;

                for edge in node_ref.use_edges() {
                    if !NodeProperties::is_control_edge(edge) {
                        continue;
                    }
                    let use_node = edge.from();
                    let use_node_ref = &*use_node;

                    if use_node_ref.opcode() == IrOpcode::kLoopExit && node_ref.opcode() == IrOpcode::kLoop {
                        continue;
                    }

                    if !self.control_level_.contains_key(&use_node) && use_node_ref.opcode() != IrOpcode::kEnd {
                        self.set_control_level(use_node, level + 1);
                        queue.push_back(use_node);
                    }
                }
            }
        }
    }

    fn get_control_level(&self, control: *mut Node) -> i32 {
        *self.control_level_.get(&control).unwrap()
    }

    fn set_control_level(&mut self, control: *mut Node, level: i32) {
        self.control_level_.insert(control, level);
    }

    fn set_early_schedule_position(&mut self, node: *mut Node, early_schedule_position: *mut Node) {
        self.early_schedule_position_.insert(node, early_schedule_position);
    }

    pub fn get_early_schedule_position(&mut self, node: *mut Node) -> *mut Node {
        unsafe {
            if NodeProperties::is_control(&(*node)) {
                return node;
            }

            if let Some(pos) = self.early_schedule_position_.get(&node) {
                return *pos;
            }

            #[derive(Debug)]
            struct NodeState {
                node: *mut Node,
                early_schedule_position: *mut Node,
                input_index: usize,
            }

            let mut stack: Vec<NodeState> = Vec::new();
            stack.push(NodeState {
                node,
                early_schedule_position: ptr::null_mut(),
                input_index: 0,
            });

            let mut early_schedule_position: *mut Node = ptr::null_mut();

            while !stack.is_empty() {
                let mut top = stack.pop().unwrap();
                let top_node = &*top.node;

                if NodeProperties::is_phi(top.node) {
                    early_schedule_position = NodeProperties::get_control_input(top.node);
                } else if top_node.input_count() == 0 {
                    early_schedule_position = (*self.graph_).start();
                } else {
                    if top.input_index == top_node.input_count() {
                        early_schedule_position = top.early_schedule_position;
                    } else {
                        let input = top_node.input_at(top.input_index);

                        let mut input_early_schedule_position: *mut Node = ptr::null_mut();

                        if NodeProperties::is_control(&*input) {
                            input_early_schedule_position = input;
                        } else if let Some(pos) = self.early_schedule_position_.get(&input) {
                            input_early_schedule_position = *pos;
                        }
                        if !input_early_schedule_position.is_null() {
                            if top.early_schedule_position.is_null()
                                || self.get_control_level(top.early_schedule_position)
                                    < self.get_control_level(input_early_schedule_position)
                            {
                                top.early_schedule_position = input_early_schedule_position;
                            }
                            top.input_index += 1;
                            stack.push(top);
                            continue;

                        } else {
                            top.input_index += 1;
                            stack.push(top);
                            stack.push(NodeState {
                                node: input,
                                early_schedule_position: ptr::null_mut(),
                                input_index: 0,
                            });
                            continue;
                        }
                    }
                }

                self.set_early_schedule_position(top.node, early_schedule_position);

                if !stack.is_empty() {
                    let mut use = stack.pop().unwrap();
                    if use.early_schedule_position.is_null()
                        || self.get_control_level(use.early_schedule_position) < self.get_control_level(early_schedule_position)
                    {
                        use.early_schedule_position = early_schedule_position;
                    }
                    stack.push(use);
                }
            }
            return early_schedule_position;
        }
    }

    pub fn same_basic_block(&mut self, node0: *mut Node, node1: *mut Node) -> bool {
        unsafe {
            let early_schedule_position0 = if NodeProperties::is_control(&(*node0)) {
                node0
            } else {
                self.get_early_schedule_position(node0)
            };
            let early_schedule_position1 = if NodeProperties::is_control(&(*node1)) {
                node1
            } else {
                self.get_early_schedule_position(node1)
            };
            early_schedule_position0 == early_schedule_position1
        }
    }
}
