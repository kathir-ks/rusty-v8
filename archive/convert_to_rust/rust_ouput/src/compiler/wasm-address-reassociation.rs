// Converted from V8 C++ source files:
// Header: wasm-address-reassociation.h
// Implementation: wasm-address-reassociation.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod wasm_address_reassociation {
use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::rc::Rc;

use crate::compiler::common_operator::CommonOperatorBuilder;
use crate::compiler::graph_reducer::V8;
use crate::compiler::js_graph::JSGraph;
use crate::compiler::machine_graph::MachineGraph;
use crate::compiler::machine_operator::MachineOperatorBuilder;
use crate::compiler::node::Node;
use crate::compiler::node_properties::NodeProperties;
use crate::compiler::opcodes::IrOpcode;
use crate::compiler::operator::Operator;
use crate::compiler::scheduler::NodeId;
use crate::compiler::string_builder_optimizer::TFGraph;
use crate::compiler::turbofan_types::v8;
use crate::zone::zone::{Zone, ZoneObject};
use crate::Int64BinopMatcher;
use crate::Smi;
use v8::internal::HeapObjectRef;
use v8::internal::RootIndex;
use v8::internal::Tagged;

pub struct WasmAddressReassociation<'a> {
    graph_: *mut TFGraph,
    common_: *mut CommonOperatorBuilder,
    machine_: *mut MachineOperatorBuilder,
    candidate_base_addrs_:
        RefCell<std::collections::HashMap<CandidateAddressKey, CandidateBaseAddr>>,
    candidates_: RefCell<std::collections::HashMap<CandidateAddressKey, CandidateMemOps<'a>>>,
    zone_: &'a Zone,
}

type CandidateAddressKey = (NodeId, NodeId, NodeId);

struct CandidateBaseAddr {
    base_reg_: *mut Node,
    offset_reg_: *mut Node,
}

impl CandidateBaseAddr {
    fn new(base: *mut Node, offset: *mut Node) -> Self {
        CandidateBaseAddr {
            base_reg_: base,
            offset_reg_: offset,
        }
    }
    fn base(&self) -> *mut Node {
        self.base_reg_
    }
    fn offset(&self) -> *mut Node {
        self.offset_reg_
    }
}

struct CandidateMemOps<'a> {
    mem_ops_: RefCell<Vec<*mut Node>>,
    imm_offsets_: RefCell<Vec<i64>>,
    zone_: &'a Zone,
}

impl<'a> CandidateMemOps<'a> {
    fn new(zone: &'a Zone) -> Self {
        CandidateMemOps {
            mem_ops_: RefCell::new(Vec::new()),
            imm_offsets_: RefCell::new(Vec::new()),
            zone_: zone,
        }
    }
    fn add_candidate(&self, mem_op: *mut Node, imm_offset: i64) {
        unsafe {
            if (*mem_op).opcode() != IrOpcode::kProtectedLoad
                && (*mem_op).opcode() != IrOpcode::kProtectedStore
            {
                panic!("Unexpected opcode");
            }
        }
        self.mem_ops_.borrow_mut().push(mem_op);
        self.imm_offsets_.borrow_mut().push(imm_offset);
    }

    fn get_num_nodes(&self) -> usize {
        let mem_ops = self.mem_ops_.borrow();
        let imm_offsets = self.imm_offsets_.borrow();
        if mem_ops.len() != imm_offsets.len() {
            panic!("mem_ops and imm_offsets size mismatch");
        }
        mem_ops.len()
    }

    fn mem_op(&self, i: usize) -> *mut Node {
        self.mem_ops_.borrow()[i]
    }

    fn imm_offset(&self, i: usize) -> i64 {
        self.imm_offsets_.borrow()[i]
    }
}

impl<'a> WasmAddressReassociation<'a> {
    pub fn new(jsgraph: *mut JSGraph, zone: &'a Zone) -> Self {
        unsafe {
            WasmAddressReassociation {
                graph_: (*jsgraph).graph(),
                common_: (*jsgraph).common(),
                machine_: (*jsgraph).machine(),
                candidate_base_addrs_: RefCell::new(std::collections::HashMap::new()),
                candidates_: RefCell::new(std::collections::HashMap::new()),
                zone_: zone,
            }
        }
    }

    pub fn optimize(&self) {
        let mut candidates = self.candidates_.borrow_mut();
        let keys: Vec<CandidateAddressKey> = candidates.keys().cloned().collect();

        for key in keys {
            if !self.should_try_optimize(&key) {
                continue;
            }

            let new_object = self.create_new_base(&key);

            let mut mem_ops = candidates.get_mut(&key).unwrap();
            let num_nodes = mem_ops.get_num_nodes();

            for i in 0..num_nodes {
                let mem_op = mem_ops.mem_op(i);
                unsafe {
                    let imm_offset_val = mem_ops.imm_offset(i);
                    let imm_offset = (*self.graph_).NewNode(
                        (*self.common_).Int64Constant(imm_offset_val),
                    );
                    self.replace_inputs(mem_op, new_object, imm_offset);
                }
            }
        }
    }

    fn should_try_optimize(&self, key: &CandidateAddressKey) -> bool {
        self.candidates_.borrow().get(key).unwrap().get_num_nodes() > 1
    }

    fn create_new_base(&self, key: &CandidateAddressKey) -> *mut Node {
        let candidate_base_addrs = self.candidate_base_addrs_.borrow();
        let candidate_base_addr = candidate_base_addrs.get(key).unwrap();
        let base = candidate_base_addr.base();
        let reg_offset = candidate_base_addr.offset();
        unsafe {
            (*self.graph_).NewNode((*self.machine_).Int64Add(), base, reg_offset)
        }
    }

    fn replace_inputs(&self, mem_op: *mut Node, base: *mut Node, offset: *mut Node) {
        unsafe {
            if (*mem_op).InputCount() <= 1 {
                panic!("Input count must be greater than 1");
            }

            if !NodeProperties::IsConstant(offset) {
                panic!("Offset must be constant");
            }

            (*mem_op).ReplaceInput(0, base);
            (*mem_op).ReplaceInput(1, offset);
        }
    }

    pub fn visit_protected_mem_op(&self, node: *mut Node, effect_chain: NodeId) {
        unsafe {
            if (*node).opcode() != IrOpcode::kProtectedLoad
                && (*node).opcode() != IrOpcode::kProtectedStore
            {
                panic!("Unexpected opcode");
            }

            let base = (*node).InputAt(0);
            let offset = (*node).InputAt(1);

            if (*base).opcode() == IrOpcode::kInt64Add && (*offset).opcode() == IrOpcode::kInt64Add {
                let base_add = Int64BinopMatcher::new(base);
                let offset_add = Int64BinopMatcher::new(offset);

                if base_add.right().HasResolvedValue()
                    && !base_add.left().HasResolvedValue()
                    && offset_add.right().HasResolvedValue()
                    && !offset_add.left().HasResolvedValue()
                {
                    let base_reg = base_add.left().node();
                    let reg_offset = offset_add.left().node();
                    let imm_offset =
                        base_add.right().ResolvedValue() + offset_add.right().ResolvedValue();
                    self.add_candidate(node, base_reg, reg_offset, imm_offset, effect_chain);
                    return;
                }
            }
            if (*base).opcode() == IrOpcode::kInt64Add {
                let base_add = Int64BinopMatcher::new(base);
                if base_add.right().HasResolvedValue() && !base_add.left().HasResolvedValue() {
                    let base_reg = base_add.left().node();
                    let reg_offset = (*node).InputAt(1);
                    let imm_offset = base_add.right().ResolvedValue();
                    self.add_candidate(node, base_reg, reg_offset, imm_offset, effect_chain);
                    return;
                }
            }
            if (*offset).opcode() == IrOpcode::kInt64Add {
                let offset_add = Int64BinopMatcher::new(offset);
                if offset_add.right().HasResolvedValue() && !offset_add.left().HasResolvedValue() {
                    let base_reg = (*node).InputAt(0);
                    let reg_offset = offset_add.left().node();
                    let imm_offset = offset_add.right().ResolvedValue();
                    self.add_candidate(node, base_reg, reg_offset, imm_offset, effect_chain);
                    return;
                }
            }
        }
    }

    fn add_candidate(
        &self,
        mem_op: *mut Node,
        base: *mut Node,
        reg_offset: *mut Node,
        imm_offset: i64,
        effect_chain: NodeId,
    ) {
        let mut base_local = base;
        let mut reg_offset_local = reg_offset;

        unsafe {
            if (*base_local).id() > (*reg_offset_local).id() {
                std::mem::swap(&mut base_local, &mut reg_offset_local);
            }

            let key = (
                (*base_local).id(),
                (*reg_offset_local).id(),
                effect_chain,
            );

            let mut candidate_base_addrs = self.candidate_base_addrs_.borrow_mut();
            let mut candidates = self.candidates_.borrow_mut();

            let is_new = match candidate_base_addrs.entry(key) {
                Entry::Vacant(entry) => {
                    entry.insert(CandidateBaseAddr::new(base_local, reg_offset_local));
                    true
                }
                Entry::Occupied(_) => false,
            };

            let entry = candidates.entry(key);

            match entry {
                Entry::Vacant(entry) => {
                    let mut mem_ops = CandidateMemOps::new(self.zone_);
                    mem_ops.add_candidate(mem_op, imm_offset);
                    entry.insert(mem_ops);
                }
                Entry::Occupied(mut entry) => {
                    entry.get_mut().add_candidate(mem_op, imm_offset);
                }
            };
        }
    }

    fn has_candidate_base_addr(&self, key: &CandidateAddressKey) -> bool {
        self.candidate_base_addrs_.borrow().contains_key(key)
    }
}
}
