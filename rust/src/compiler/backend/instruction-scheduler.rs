// Copyright 2015 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::cmp::max;
use std::collections::HashMap;
use std::iter::Rev;
use std::vec::IntoIter;

//use crate::base::iterator::Reversed; // Assuming a similar structure exists in Rust.
//use crate::base::utils::random_number_generator::RandomNumberGenerator; // Assuming a similar structure exists in Rust.
//use crate::compiler::backend::instruction_codes::*; // Assuming these definitions are available in Rust.
//use crate::compiler::instruction::*; // Assuming Instruction struct definition
//use crate::compiler::instruction_sequence::*; // Assuming InstructionSequence struct definition
//use crate::compiler::instruction_operand::*; // Assuming InstructionOperand, UnallocatedOperand, ConstantOperand definitions

//TODO: Replace with proper implementations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RpoNumber(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InstructionOperand {}

impl InstructionOperand {
    pub fn is_unallocated(&self) -> bool {
        false
    }
    pub fn is_constant(&self) -> bool {
        false
    }
}

pub struct UnallocatedOperand {}

impl UnallocatedOperand {
    pub fn cast(_op: &InstructionOperand) -> &UnallocatedOperand {
        panic!()
    }
    pub fn virtual_register(&self) -> i32 {
        0
    }
}

pub struct ConstantOperand {}

impl ConstantOperand {
    pub fn cast(_op: &InstructionOperand) -> &ConstantOperand {
        panic!()
    }
    pub fn virtual_register(&self) -> i32 {
        0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ArchOpcode(usize);

const kArchNop: ArchOpcode = ArchOpcode(0);
const kArchStackCheckOffset: ArchOpcode = ArchOpcode(1);
const kArchFramePointer: ArchOpcode = ArchOpcode(2);
const kArchParentFramePointer: ArchOpcode = ArchOpcode(3);
const kArchStackSlot: ArchOpcode = ArchOpcode(4);
const kArchComment: ArchOpcode = ArchOpcode(5);
const kArchDeoptimize: ArchOpcode = ArchOpcode(6);
const kArchJmp: ArchOpcode = ArchOpcode(7);
const kArchBinarySearchSwitch: ArchOpcode = ArchOpcode(8);
const kArchRet: ArchOpcode = ArchOpcode(9);
const kArchTableSwitch: ArchOpcode = ArchOpcode(10);
const kArchThrowTerminator: ArchOpcode = ArchOpcode(11);
const kArchTruncateDoubleToI: ArchOpcode = ArchOpcode(12);
const kIeee754Float64Acos: ArchOpcode = ArchOpcode(13);
const kIeee754Float64Acosh: ArchOpcode = ArchOpcode(14);
const kIeee754Float64Asin: ArchOpcode = ArchOpcode(15);
const kIeee754Float64Asinh: ArchOpcode = ArchOpcode(16);
const kIeee754Float64Atan: ArchOpcode = ArchOpcode(17);
const kIeee754Float64Atanh: ArchOpcode = ArchOpcode(18);
const kIeee754Float64Atan2: ArchOpcode = ArchOpcode(19);
const kIeee754Float64Cbrt: ArchOpcode = ArchOpcode(20);
const kIeee754Float64Cos: ArchOpcode = ArchOpcode(21);
const kIeee754Float64Cosh: ArchOpcode = ArchOpcode(22);
const kIeee754Float64Exp: ArchOpcode = ArchOpcode(23);
const kIeee754Float64Expm1: ArchOpcode = ArchOpcode(24);
const kIeee754Float64Log: ArchOpcode = ArchOpcode(25);
const kIeee754Float64Log1p: ArchOpcode = ArchOpcode(26);
const kIeee754Float64Log10: ArchOpcode = ArchOpcode(27);
const kIeee754Float64Log2: ArchOpcode = ArchOpcode(28);
const kIeee754Float64Pow: ArchOpcode = ArchOpcode(29);
const kIeee754Float64Sin: ArchOpcode = ArchOpcode(30);
const kIeee754Float64Sinh: ArchOpcode = ArchOpcode(31);
const kIeee754Float64Tan: ArchOpcode = ArchOpcode(32);
const kIeee754Float64Tanh: ArchOpcode = ArchOpcode(33);
const kArchStackPointerGreaterThan: ArchOpcode = ArchOpcode(34);
const kArchStackPointer: ArchOpcode = ArchOpcode(35);
const kArchSetStackPointer: ArchOpcode = ArchOpcode(36);
const kArchPrepareCallCFunction: ArchOpcode = ArchOpcode(37);
const kArchPrepareTailCall: ArchOpcode = ArchOpcode(38);
const kArchTailCallCodeObject: ArchOpcode = ArchOpcode(39);
const kArchTailCallAddress: ArchOpcode = ArchOpcode(40);
const kArchAbortCSADcheck: ArchOpcode = ArchOpcode(41);
const kArchDebugBreak: ArchOpcode = ArchOpcode(42);
const kArchSaveCallerRegisters: ArchOpcode = ArchOpcode(43);
const kArchRestoreCallerRegisters: ArchOpcode = ArchOpcode(44);
const kArchCallCFunction: ArchOpcode = ArchOpcode(45);
const kArchCallCFunctionWithFrameState: ArchOpcode = ArchOpcode(46);
const kArchCallCodeObject: ArchOpcode = ArchOpcode(47);
const kArchCallJSFunction: ArchOpcode = ArchOpcode(48);
const kArchCallBuiltinPointer: ArchOpcode = ArchOpcode(49);
const kArchStoreWithWriteBarrier: ArchOpcode = ArchOpcode(50);
const kArchAtomicStoreWithWriteBarrier: ArchOpcode = ArchOpcode(51);
const kArchStoreIndirectWithWriteBarrier: ArchOpcode = ArchOpcode(52);
const kAtomicLoadInt8: ArchOpcode = ArchOpcode(53);
const kAtomicLoadUint8: ArchOpcode = ArchOpcode(54);
const kAtomicLoadInt16: ArchOpcode = ArchOpcode(55);
const kAtomicLoadUint16: ArchOpcode = ArchOpcode(56);
const kAtomicLoadWord32: ArchOpcode = ArchOpcode(57);
const kAtomicStoreWord8: ArchOpcode = ArchOpcode(58);
const kAtomicStoreWord16: ArchOpcode = ArchOpcode(59);
const kAtomicStoreWord32: ArchOpcode = ArchOpcode(60);
const kAtomicExchangeInt8: ArchOpcode = ArchOpcode(61);
const kAtomicExchangeUint8: ArchOpcode = ArchOpcode(62);
const kAtomicExchangeInt16: ArchOpcode = ArchOpcode(63);
const kAtomicExchangeUint16: ArchOpcode = ArchOpcode(64);
const kAtomicExchangeWord32: ArchOpcode = ArchOpcode(65);
const kAtomicCompareExchangeInt8: ArchOpcode = ArchOpcode(66);
const kAtomicCompareExchangeUint8: ArchOpcode = ArchOpcode(67);
const kAtomicCompareExchangeInt16: ArchOpcode = ArchOpcode(68);
const kAtomicCompareExchangeUint16: ArchOpcode = ArchOpcode(69);
const kAtomicCompareExchangeWord32: ArchOpcode = ArchOpcode(70);
const kAtomicAddInt8: ArchOpcode = ArchOpcode(71);
const kAtomicAddUint8: ArchOpcode = ArchOpcode(72);
const kAtomicAddInt16: ArchOpcode = ArchOpcode(73);
const kAtomicAddUint16: ArchOpcode = ArchOpcode(74);
const kAtomicAddWord32: ArchOpcode = ArchOpcode(75);
const kAtomicSubInt8: ArchOpcode = ArchOpcode(76);
const kAtomicSubUint8: ArchOpcode = ArchOpcode(77);
const kAtomicSubInt16: ArchOpcode = ArchOpcode(78);
const kAtomicSubUint16: ArchOpcode = ArchOpcode(79);
const kAtomicSubWord32: ArchOpcode = ArchOpcode(80);
const kAtomicAndInt8: ArchOpcode = ArchOpcode(81);
const kAtomicAndUint8: ArchOpcode = ArchOpcode(82);
const kAtomicAndInt16: ArchOpcode = ArchOpcode(83);
const kAtomicAndUint16: ArchOpcode = ArchOpcode(84);
const kAtomicAndWord32: ArchOpcode = ArchOpcode(85);
const kAtomicOrInt8: ArchOpcode = ArchOpcode(86);
const kAtomicOrUint8: ArchOpcode = ArchOpcode(87);
const kAtomicOrInt16: ArchOpcode = ArchOpcode(88);
const kAtomicOrUint16: ArchOpcode = ArchOpcode(89);
const kAtomicOrWord32: ArchOpcode = ArchOpcode(90);
const kAtomicXorInt8: ArchOpcode = ArchOpcode(91);
const kAtomicXorUint8: ArchOpcode = ArchOpcode(92);
const kAtomicXorInt16: ArchOpcode = ArchOpcode(93);
const kAtomicXorUint16: ArchOpcode = ArchOpcode(94);
const kAtomicXorWord32: ArchOpcode = ArchOpcode(95);

const kNoOpcodeFlags: i32 = 0;
const kIsLoadOperation: i32 = 1;
const kHasSideEffect: i32 = 2;
const kIsBarrier: i32 = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FlagsMode(usize);

const kFlags_branch: FlagsMode = FlagsMode(0);

struct V8Flags {
    turbo_stress_instruction_scheduling: bool,
    random_seed: u64,
}

impl V8Flags {
    fn new() -> Self {
        V8Flags {
            turbo_stress_instruction_scheduling: false,
            random_seed: 0,
        }
    }
}

static mut v8_flags: V8Flags = V8Flags::new();

fn init_flags() {
    unsafe {
        v8_flags = V8Flags::new(); // Initialize with default values
    }
}

fn set_turbo_stress_instruction_scheduling(value: bool) {
    unsafe {
        v8_flags.turbo_stress_instruction_scheduling = value;
    }
}

fn set_random_seed(seed: u64) {
    unsafe {
        v8_flags.random_seed = seed;
    }
}

pub struct Instruction {
    arch_opcode: ArchOpcode,
    flags_mode: FlagsMode,
    inputs: Vec<InstructionOperand>,
    outputs: Vec<InstructionOperand>,
}

impl Instruction {
    pub fn new(arch_opcode: ArchOpcode, flags_mode: FlagsMode) -> Self {
        Instruction {
            arch_opcode,
            flags_mode,
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }
    pub fn arch_opcode(&self) -> ArchOpcode {
        self.arch_opcode
    }
    pub fn flags_mode(&self) -> FlagsMode {
        self.flags_mode
    }
    pub fn input_count(&self) -> usize {
        self.inputs.len()
    }
    pub fn output_count(&self) -> usize {
        self.outputs.len()
    }
    pub fn input_at(&self, index: usize) -> &InstructionOperand {
        &self.inputs[index]
    }
    pub fn output_at(&self, index: usize) -> &InstructionOperand {
        &self.outputs[index]
    }
    pub fn is_deoptimize_call(&self) -> bool {
        false
    }
}

pub struct InstructionSequence {}

impl InstructionSequence {
    pub fn start_block(&mut self, _rpo: RpoNumber) {}
    pub fn end_block(&mut self, _rpo: RpoNumber) {}
    pub fn add_instruction(&mut self, _instr: &Instruction) {}
}

pub struct Zone {}

impl Zone {
    pub fn new() -> Self {
        Zone {}
    }
    pub fn allocate<T>(&self, value: T) -> Box<T> {
        Box::new(value)
    }
}

/// A trait for scheduler queues.
trait SchedulingQueue {
    fn add_node(&mut self, node: &mut ScheduleGraphNode);
    fn pop_best_candidate(&mut self, cycle: int32_t) -> Option<&mut ScheduleGraphNode>;
    fn is_empty(&self) -> bool;
}

/// Base class for scheduling queues.
struct SchedulingQueueBase<'a> {
    nodes_: Vec<&'a mut ScheduleGraphNode>,
}

impl<'a> SchedulingQueueBase<'a> {
    fn new() -> Self {
        SchedulingQueueBase { nodes_: Vec::new() }
    }
}

impl<'a> SchedulingQueueBase<'a> {
    fn add_node(&mut self, node: &mut ScheduleGraphNode) {
        // We keep the ready list sorted by total latency so that we can quickly find
        // the next best candidate to schedule.
        let mut it = self.nodes_.iter();
        let mut insert_pos = 0;
        while let Some(existing_node) = it.next() {
            if existing_node.total_latency() < node.total_latency() {
                insert_pos += 1;
            } else {
                break;
            }
        }
        self.nodes_.insert(insert_pos, node);
    }
}

/// A scheduling queue that prioritizes nodes on the critical path.
struct CriticalPathFirstQueue<'a> {
    base: SchedulingQueueBase<'a>,
}

impl<'a> CriticalPathFirstQueue<'a> {
    fn new() -> Self {
        CriticalPathFirstQueue {
            base: SchedulingQueueBase::new(),
        }
    }
}

impl<'a> SchedulingQueue for CriticalPathFirstQueue<'a> {
    fn add_node(&mut self, node: &mut ScheduleGraphNode) {
        self.base.add_node(node);
    }

    fn pop_best_candidate(&mut self, cycle: int32_t) -> Option<&mut ScheduleGraphNode> {
        if self.is_empty() {
            panic!("DCHECK(!IsEmpty()) failed");
        }
        let mut candidate: Option<usize> = None;
        for (index, node) in self.base.nodes_.iter().enumerate() {
            // We only consider instructions that have all their operands ready.
            if cycle >= node.start_cycle() {
                candidate = Some(index);
                break;
            }
        }

        if let Some(index) = candidate {
            let result = self.base.nodes_.remove(index);
            Some(result)
        } else {
            None
        }
    }
    fn is_empty(&self) -> bool {
        self.base.nodes_.is_empty()
    }
}

/// A scheduling queue that chooses a random element from the ready list.
struct StressSchedulerQueue<'a> {
    base: SchedulingQueueBase<'a>,
    //random_number_generator: &'a mut RandomNumberGenerator, // Assuming RandomNumberGenerator exists
}

impl<'a> StressSchedulerQueue<'a> {
    fn new() -> Self {
        StressSchedulerQueue {
            base: SchedulingQueueBase::new(),
            //random_number_generator: rng,
        }
    }
}

impl<'a> SchedulingQueue for StressSchedulerQueue<'a> {
    fn add_node(&mut self, node: &mut ScheduleGraphNode) {
        self.base.add_node(node);
    }

    fn pop_best_candidate(&mut self, cycle: int32_t) -> Option<&mut ScheduleGraphNode> {
        if self.is_empty() {
            panic!("DCHECK(!IsEmpty()) failed");
        }
        // Choose a random element from the ready list.
        unsafe {
            if v8_flags.turbo_stress_instruction_scheduling {
                let candidate_index = (v8_flags.random_seed % self.base.nodes_.len() as u64) as usize;
                let result = self.base.nodes_.remove(candidate_index);
                return Some(result);
            } else {
                if let Some(result) = self.base.nodes_.pop() {
                    return Some(result);
                } else {
                    return None;
                }
            }
        }
    }
    fn is_empty(&self) -> bool {
        self.base.nodes_.is_empty()
    }
}

/// Represents a node in the schedule graph.
struct ScheduleGraphNode {
    instr_: *mut Instruction,
    successors_: Vec<*mut ScheduleGraphNode>,
    unscheduled_predecessors_count_: i32,
    latency_: int32_t,
    total_latency_: int32_t,
    start_cycle_: int32_t,
}

impl ScheduleGraphNode {
    fn new(instr: *mut Instruction) -> Self {
        ScheduleGraphNode {
            instr_: instr,
            successors_: Vec::new(),
            unscheduled_predecessors_count_: 0,
            latency_: 0, // TODO: Implement GetInstructionLatency
            total_latency_: -1,
            start_cycle_: -1,
        }
    }

    fn instruction(&mut self) -> &Instruction {
        unsafe { &*self.instr_ }
    }

    fn successors(&self) -> &Vec<*mut ScheduleGraphNode> {
        &self.successors_
    }

    fn has_unscheduled_predecessor(&self) -> bool {
        self.unscheduled_predecessors_count_ > 0
    }

    fn latency(&self) -> int32_t {
        self.latency_
    }

    fn total_latency(&self) -> int32_t {
        self.total_latency_
    }

    fn set_total_latency(&mut self, latency: int32_t) {
        self.total_latency_ = latency;
    }

    fn start_cycle(&self) -> int32_t {
        self.start_cycle_
    }

    fn set_start_cycle(&mut self, cycle: int32_t) {
        self.start_cycle_ = cycle;
    }

    fn add_successor(&mut self, node: *mut ScheduleGraphNode) {
        self.successors_.push(node);
        unsafe {
            (*node).unscheduled_predecessors_count_ += 1;
        }
    }

    fn drop_unscheduled_predecessor(&mut self) {
        self.unscheduled_predecessors_count_ -= 1;
    }
}

type int32_t = i32;

/// The instruction scheduler.
pub struct InstructionScheduler {
    zone_: Zone,
    sequence_: *mut InstructionSequence,
    graph_: Vec<*mut ScheduleGraphNode>,
    last_side_effect_instr_: *mut ScheduleGraphNode,
    pending_loads_: Vec<*mut ScheduleGraphNode>,
    last_live_in_reg_marker_: *mut ScheduleGraphNode,
    last_deopt_or_trap_: *mut ScheduleGraphNode,
    operands_map_: HashMap<i32, *mut ScheduleGraphNode>,
    //random_number_generator_: Option<RandomNumberGenerator>,
}

impl InstructionScheduler {
    pub fn new(zone: Zone, sequence: *mut InstructionSequence) -> Self {
        // Initialize the flags (only for demonstration)
        init_flags();

        unsafe {
            if v8_flags.turbo_stress_instruction_scheduling {
                //random_number_generator_ = Some(RandomNumberGenerator::new(v8_flags.random_seed));
            }
        }

        InstructionScheduler {
            zone_: zone,
            sequence_: sequence,
            graph_: Vec::new(),
            last_side_effect_instr_: std::ptr::null_mut(),
            pending_loads_: Vec::new(),
            last_live_in_reg_marker_: std::ptr::null_mut(),
            last_deopt_or_trap_: std::ptr::null_mut(),
            operands_map_: HashMap::new(),
            //random_number_generator_: None,
        }
    }

    fn zone(&self) -> &Zone {
        &self.zone_
    }

    fn sequence(&mut self) -> &mut InstructionSequence {
        unsafe { &mut *self.sequence_ }
    }

    pub fn start_block(&mut self, rpo: RpoNumber) {
        //DCHECK(graph_.empty());
        //DCHECK_NULL(last_side_effect_instr_);
        //DCHECK(pending_loads_.empty());
        //DCHECK_NULL(last_live_in_reg_marker_);
        //DCHECK_NULL(last_deopt_or_trap_);
        //DCHECK(operands_map_.empty());
        self.sequence().start_block(rpo);
        self.graph_.clear();
        self.last_side_effect_instr_ = std::ptr::null_mut();
        self.pending_loads_.clear();
        self.last_live_in_reg_marker_ = std::ptr::null_mut();
        self.last_deopt_or_trap_ = std::ptr::null_mut();
        self.operands_map_.clear();
    }

    pub fn end_block(&mut self, rpo: RpoNumber) {
        unsafe {
            if v8_flags.turbo_stress_instruction_scheduling {
                self.schedule::<StressSchedulerQueue>();
            } else {
                self.schedule::<CriticalPathFirstQueue>();
            }
        }
        self.sequence().end_block(rpo);
    }

    pub fn add_terminator(&mut self, instr: *mut Instruction) {
        let new_node = self.zone().allocate(ScheduleGraphNode::new(instr));
        let new_node_ptr: *mut ScheduleGraphNode = Box::into_raw(new_node);

        // Make sure that basic block terminators are not moved by adding them
        // as successor of every instruction.
        for node in &mut self.graph_ {
            unsafe {
                (*node).add_successor(new_node_ptr);
            }
        }
        self.graph_.push(new_node_ptr);
    }

    pub fn add_instruction(&mut self, instr: *mut Instruction) {
        unsafe {
            if self.is_barrier(&(*instr)) {
                if v8_flags.turbo_stress_instruction_scheduling {
                    self.schedule::<StressSchedulerQueue>();
                } else {
                    self.schedule::<CriticalPathFirstQueue>();
                }
                self.sequence().add_instruction(&(*instr));
                return;
            }

            let new_node = self.zone().allocate(ScheduleGraphNode::new(instr));
            let new_node_ptr: *mut ScheduleGraphNode = Box::into_raw(new_node);

            // We should not have branches in the middle of a block.
            if (*instr).flags_mode() == kFlags_branch {
                panic!("DCHECK_NE(instr->flags_mode(), kFlags_branch) failed");
            }

            if self.is_fixed_register_parameter(&(*instr)) {
                if !self.last_live_in_reg_marker_.is_null() {
                    (*self.last_live_in_reg_marker_).add_successor(new_node_ptr);
                }
                self.last_live_in_reg_marker_ = new_node_ptr;
            } else {
                if !self.last_live_in_reg_marker_.is_null() {
                    (*self.last_live_in_reg_marker_).add_successor(new_node_ptr);
                }

                // Make sure that instructions are not scheduled before the last
                // deoptimization or trap point when they depend on it.
                if (!self.last_deopt_or_trap_.is_null()) && self.depends_on_deopt_or_trap(&(*instr)) {
                    (*self.last_deopt_or_trap_).add_successor(new_node_ptr);
                }

                // Instructions with side effects and memory operations can't be
                // reordered with respect to each other.
                if self.has_side_effect(&(*instr)) {
                    if !self.last_side_effect_instr_.is_null() {
                        (*self.last_side_effect_instr_).add_successor(new_node_ptr);
                    }
                    for load in &mut self.pending_loads_ {
                        (*load).add_successor(new_node_ptr);
                    }
                    self.pending_loads_.clear();
                    self.last_side_effect_instr_ = new_node_ptr;
                } else if self.is_load_operation(&(*instr)) {
                    // Load operations can't be reordered with side effects instructions but
                    // independent loads can be reordered with respect to each other.
                    if !self.last_side_effect_instr_.is_null() {
                        (*self.last_side_effect_instr_).add_successor(new_node_ptr);
                    }
                    self.pending_loads_.push(new_node_ptr);
                } else if (*instr).is_deoptimize_call() || self.can_trap(&(*instr)) {
                    // Ensure that deopts or traps are not reordered with respect to
                    // side-effect instructions.
                    if !self.last_side_effect_instr_.is_null() {
                        (*self.last_side_effect_instr_).add_successor(new_node_ptr);
                    }
                }

                // Update last deoptimization or trap point.
                if (*instr).is_deoptimize_call() || self.can_trap(&(*instr)) {
                    self.last_deopt_or_trap_ = new_node_ptr;
                }

                // Look for operand dependencies.
                for i in 0..(*instr).input_count() {
                    let input = (*instr).input_at(i);
                    if input.is_unallocated() {
                        let vreg = UnallocatedOperand::cast(input).virtual_register();
                        if let Some(predecessor) = self.operands_map_.get(&vreg) {
                            (*predecessor).add_successor(new_node_ptr);
                        }
                    }
                }

                // Record the virtual registers defined by this instruction.
                for i in 0..(*instr).output_count() {
                    let output = (*instr).output_at(i);
                    if output.is_unallocated() {
                        self.operands_map_.insert(
                            UnallocatedOperand::cast(output).virtual_register(),
                            new_node_ptr,
                        );
                    } else if output.is_constant() {
                        self.operands_map_.insert(
                            ConstantOperand::cast(output).virtual_register(),
                            new_node_ptr,
                        );
                    }
                }
            }

            self.graph_.push(new_node_ptr);
        }
    }

    fn schedule<Q>(&mut self)
    where
        Q: for<'a> SchedulingQueue + 'static,
    {
        let mut ready_list: Q = <Q>::new(); // Create an instance of the queue type.

        // Compute total latencies so that we can schedule the critical path first.
        self.compute_total_latencies();

        // Add nodes which don't have dependencies to the ready list.
        for node in &mut self.graph_ {
            unsafe {
                if !(*node).has_unscheduled_predecessor() {
                    ready_list.add_node(*node);
                }
            }
        }

        // Go through the ready list and schedule the instructions.
        let mut cycle: int32_t = 0;
        while !ready_list.is_empty() {
            let candidate = ready_list.pop_best_candidate(cycle);

            if let Some(mut candidate_node) = candidate {
                unsafe {
                    self.sequence().add_instruction(candidate_node.instruction());

                    for successor_ptr in candidate_node.successors() {
                        (*successor_ptr).drop_unscheduled_predecessor();
                        (*successor_ptr).set_start_cycle(max(
                            (*successor_ptr).start_cycle(),
                            cycle + candidate_node.latency(),
                        ));

                        if !(*successor_ptr).has_unscheduled_predecessor() {
                            ready_list.add_node(*successor_ptr);
                        }
                    }
                }
            }

            cycle += 1;
        }

        // Reset own state.
        self.graph_.clear();
        self.operands_map_.clear();
        self.pending_loads_.clear();
        self.last_deopt_or_trap_ = std::ptr::null_mut();
        self.last_live_in_reg_marker_ = std::ptr::null_mut();
        self.last_side_effect_instr_ = std::ptr::null_mut();
    }

    fn get_instruction_flags(&self, instr: &Instruction) -> i32 {
        match instr.arch_opcode() {
            kArchNop | kArchStackCheckOffset | kArchFramePointer | kArchParentFramePointer
            | kArchStackSlot | kArchComment | kArchDeoptimize | kArchJmp | kArchBinarySearchSwitch
            | kArchRet | kArchTableSwitch | kArchThrowTerminator => kNoOpcodeFlags,

            kArchTruncateDoubleToI | kIeee754Float64Acos | kIeee754Float64Acosh | kIeee754Float64Asin
            | kIeee754Float64Asinh | kIeee754Float64Atan | kIeee754Float64Atanh | kIeee754Float64Atan2
            | kIeee754Float64Cbrt | kIeee754Float64Cos | kIeee754Float64Cosh | kIeee754Float64Exp
            | kIeee754Float64Expm1 | kIeee754Float64Log | kIeee754Float64Log1p | kIeee754Float64Log10
            | kIeee754Float64Log2 | kIeee754Float64Pow | kIeee754Float64Sin | kIeee754Float64Sinh
            | kIeee754Float64Tan | kIeee754Float64Tanh => kNoOpcodeFlags,

            kArchStackPointerGreaterThan => kIsLoadOperation,

            kArchStackPointer | kArchSetStackPointer => kHasSideEffect,

            kArchPrepareCallCFunction | kArchPrepareTailCall | kArchTailCallCodeObject
            | kArchTailCallAddress | kArchAbortCSADcheck => kHasSideEffect,

            kArchDebugBreak => kIsBarrier,

            kArchSaveCallerRegisters | kArchRestoreCallerRegisters => kIsBarrier,

            kArchCallCFunction | kArchCallCFunctionWithFrameState | kArchCallCodeObject
            | kArchCallJSFunction | kArchCallBuiltinPointer => kIsBarrier,

            kArchStoreWithWriteBarrier | kArchAtomicStoreWithWriteBarrier
            | kArchStoreIndirectWithWriteBarrier => kHasSideEffect,

            kAtomicLoadInt8 | kAtomicLoadUint8 | kAtomicLoadInt16 | kAtomicLoadUint16 | kAtomicLoadWord32 => {
                kIsLoadOperation
            }

            kAtomicStoreWord8 | kAtomicStoreWord16 | kAtomicStoreWord32 => kHasSideEffect,

            kAtomicExchangeInt8 | kAtomicExchangeUint8 | kAtomicExchangeInt16 | kAtomicExchangeUint16
            | kAtomicExchangeWord32 | kAtomicCompareExchangeInt8 | kAtomicCompareExchangeUint8
            | kAtomicCompareExchangeInt16 | kAtomicCompareExchangeUint16 | kAtomicCompareExchangeWord32
            | kAtomicAddInt8 | kAtomicAddUint8 | kAtomicAddInt16 | kAtomicAddUint16 | kAtomicAddWord32
            | kAtomicSubInt8 | kAtomicSubUint8 | kAtomicSubInt16 | kAtomicSubUint16 | kAtomicSubWord32
            | kAtomicAndInt8 | kAtomicAndUint8 | kAtomicAndInt16 | kAtomicAndUint16 | kAtomicAndWord32
            | kAtomicOrInt8 | kAtomicOrUint8 | kAtomicOrInt16 | kAtomicOrUint16 | kAtomicOrWord32
            | kAtomicXorInt8 | kAtomicXorUint8 | kAtomicXorInt16 | kAtomicXorUint16 | kAtomicXorWord32 => {
                kHasSideEffect
            }

            _ => self.get_target_instruction_flags(instr),
        }
    }

    fn compute_total_latencies(&mut self) {
        for node_ptr in self.graph_.iter().rev() {
            let mut max_latency = 0;

            unsafe {
                for successor_ptr in &(*node_ptr).successors() {
                    if (**successor_ptr).total_latency() > max_latency {
                        max_latency = (**successor_ptr).total_latency();
                    }
                }

                (*node_ptr).set_total_latency(max