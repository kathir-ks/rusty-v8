// Converted from V8 C++ source files:
// Header: register-allocator-verifier.h
// Implementation: register-allocator-verifier.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod utils;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::rc::Rc;

use crate::compiler::backend::instruction::InstructionOperand;
use crate::compiler::backend::instruction::RpoNumber;
use crate::compiler::backend::instruction_selector::MachineRepresentation;
use crate::execution::local_isolate::AccountingAllocator;
use crate::execution::v8threads::MaybeHandle;
use crate::execution::v8threads::ThreadId;
use crate::utils::bit_vector::BitVector;
use crate::v8::internal::compiler::Frame;
use crate::v8::internal::compiler::RegisterConfiguration;
use crate::v8::internal::handles::Handle;
use crate::v8::internal::Object;
use crate::v8::Local;

pub struct V8 {}

pub struct Zone {}

impl Zone {
    pub fn new() -> Zone {
        Zone {}
    }

    pub fn allocate_array<T>(&self, count: usize) -> Vec<T> {
        Vec::with_capacity(count)
    }

    pub fn new_final_assessment(&self, virtual_register: i32) -> Box<Assessment> {
        Box::new(Assessment::Final(FinalAssessment::new(virtual_register)))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Constant {
    pub virtual_register: i32,
}

impl Constant {
    pub fn new(virtual_register: i32) -> Constant {
        Constant { virtual_register }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InstructionOperandKind {
    Constant(Constant),
    Immediate(i32),
    Unallocated,
    Register,
    StackSlot,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LocationOperand {
    pub kind: LocationOperandKind,
    pub register_code: i32,
    pub index: i32,
    pub representation: MachineRepresentation,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LocationOperandKind {
    REGISTER,
    STACK_SLOT,
}

impl LocationOperand {
    pub fn cast(op: &InstructionOperand) -> &Self {
        match op {
            InstructionOperand::Register(reg) => reg,
            InstructionOperand::StackSlot(slot) => slot,
            _ => panic!("Invalid cast to LocationOperand"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ImmediateOperand {
    pub value: i32,
}

impl ImmediateOperand {
    pub fn cast(op: &InstructionOperand) -> &Self {
        match op {
            InstructionOperand::Immediate(imm) => imm,
            _ => panic!("Invalid cast to ImmediateOperand"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnallocatedOperand {
    pub virtual_register: i32,
}

impl UnallocatedOperand {
    pub fn cast(op: &InstructionOperand) -> &Self {
        match op {
            InstructionOperand::Unallocated(unalloc) => unalloc,
            _ => panic!("Invalid cast to UnallocatedOperand"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AllocatedOperand {
    pub kind: LocationOperandKind,
    pub register_code: i32,
    pub index: i32,
    pub representation: MachineRepresentation,
}

impl AllocatedOperand {
    pub fn cast(op: &InstructionOperand) -> &Self {
        match op {
            InstructionOperand::Register(reg) => reg,
            InstructionOperand::StackSlot(slot) => slot,
            _ => panic!("Invalid cast to AllocatedOperand"),
        }
    }
    pub fn new(
        zone: &Zone,
        kind: LocationOperandKind,
        representation: MachineRepresentation,
        index: i32,
    ) -> Self {
        AllocatedOperand {
            kind,
            register_code: 0,
            index,
            representation,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum RegisterRepresentation {
    Word32,
    Word64,
    Simd128,
    Float32,
    Float64,
}

pub struct StdoutStream {}

impl StdoutStream {
    pub fn new() -> Self {
        StdoutStream {}
    }
}

impl fmt::Write for StdoutStream {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        print!("{}", s);
        Ok(())
    }
}

impl std::io::Write for StdoutStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match std::str::from_utf8(buf) {
            Ok(s) => {
                print!("{}", s);
                Ok(buf.len())
            }
            Err(_) => Ok(0),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub struct ZoneVector<T> {
    data: Vec<T>,
    zone: Rc<RefCell<Zone>>,
}

impl<T> ZoneVector<T> {
    pub fn new(zone: Rc<RefCell<Zone>>) -> Self {
        ZoneVector {
            data: Vec::new(),
            zone,
        }
    }

    pub fn push_back(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn reserve(&mut self, capacity: usize) {
        self.data.reserve(capacity);
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.data.iter()
    }
}

pub struct ZoneSet<T, Less> {
    data: HashSet<T>,
    _less: std::marker::PhantomData<Less>,
}

impl<T, Less> ZoneSet<T, Less>
where
    T: Eq + std::hash::Hash + Copy,
{
    pub fn new(zone: &Zone) -> Self {
        ZoneSet {
            data: HashSet::new(),
            _less: std::marker::PhantomData,
        }
    }

    pub fn insert(&mut self, value: T) {
        self.data.insert(value);
    }

    pub fn count(&self, value: T) -> usize {
        if self.data.contains(&value) {
            1
        } else {
            0
        }
    }

    pub fn erase(&mut self, value: T) {
        self.data.remove(&value);
    }

    pub fn begin(&self) -> std::collections::hash_set::Iter<'_, T> {
        self.data.iter()
    }

    pub fn end(&self) -> std::collections::hash_set::Iter<'_, T> {
        self.data.iter()
    }
}

impl<T: Eq + std::hash::Hash + Copy, Less> Extend<T> for ZoneSet<T, Less> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for elem in iter {
            self.insert(elem);
        }
    }
}

pub struct OperandAsKeyLess;

impl OperandAsKeyLess {
    pub fn compare_canonicalized(a: &InstructionOperand, b: &InstructionOperand) -> bool {
        a == b
    }
}

pub struct ZoneMap<K, V, Less> {
    data: HashMap<K, V>,
    _less: std::marker::PhantomData<Less>,
}

impl<K, V, Less> ZoneMap<K, V, Less>
where
    K: Eq + std::hash::Hash + Copy,
{
    pub fn new(zone: &Zone) -> Self {
        ZoneMap {
            data: HashMap::new(),
            _less: std::marker::PhantomData,
        }
    }

    pub fn insert(&mut self, pair: (K, V)) {
        self.data.insert(pair.0, pair.1);
    }

    pub fn find(&self, key: K) -> Option<&V> {
        self.data.get(&key)
    }

    pub fn erase(&mut self, key: K) {
        self.data.remove(&key);
    }

    pub fn begin(&self) -> std::collections::hash_map::Iter<'_, K, V> {
        self.data.iter()
    }

    pub fn end(&self) -> std::collections::hash_map::Iter<'_, K, V> {
        self.data.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn clear(&mut self) {
        self.data.clear()
    }
}

impl<K: Eq + std::hash::Hash + Copy, V, Less> Extend<(K, V)> for ZoneMap<K, V, Less> {
    fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I) {
        for (key, value) in iter {
            self.insert((key, value));
        }
    }
}

pub struct ZoneQueue<T> {
    data: Vec<T>,
    zone: *mut Zone,
}

impl<T> ZoneQueue<T> {
    pub fn new(zone: *mut Zone) -> Self {
        ZoneQueue {
            data: Vec::new(),
            zone,
        }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            None
        } else {
            Some(self.data.remove(0))
        }
    }

    pub fn empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssessmentKind {
    Final,
    Pending,
}

pub enum Assessment {
    Final(FinalAssessment),
    Pending(PendingAssessment),
}

impl Assessment {
    pub fn kind(&self) -> AssessmentKind {
        match self {
            Assessment::Final(_) => AssessmentKind::Final,
            Assessment::Pending(_) => AssessmentKind::Pending,
        }
    }
}

pub struct PendingAssessment {
    origin_: *const InstructionBlock,
    operand_: InstructionOperand,
    aliases_: ZoneSet<i32, OperandAsKeyLess>,
}

impl PendingAssessment {
    pub fn new(zone: &Zone, origin: *const InstructionBlock, operand: InstructionOperand) -> Self {
        PendingAssessment {
            origin_: origin,
            operand_: operand,
            aliases_: ZoneSet::new(zone),
        }
    }

    pub fn cast<'a>(assessment: &'a Assessment) -> &'a Self {
        match assessment {
            Assessment::Pending(pending) => pending,
            _ => panic!("Invalid cast to PendingAssessment"),
        }
    }

    pub fn origin(&self) -> *const InstructionBlock {
        self.origin_
    }

    pub fn operand(&self) -> InstructionOperand {
        self.operand_
    }

    pub fn is_alias_of(&self, vreg: i32) -> bool {
        self.aliases_.count(vreg) > 0
    }

    pub fn add_alias(&mut self, vreg: i32) {
        self.aliases_.insert(vreg);
    }
}

pub struct FinalAssessment {
    virtual_register_: i32,
}

impl FinalAssessment {
    pub fn new(virtual_register: i32) -> Self {
        FinalAssessment {
            virtual_register_: virtual_register,
        }
    }

    pub fn virtual_register(&self) -> i32 {
        self.virtual_register_
    }

    pub fn cast<'a>(assessment: &'a Assessment) -> &'a Self {
        match assessment {
            Assessment::Final(final_assessment) => final_assessment,
            _ => panic!("Invalid cast to FinalAssessment"),
        }
    }
}

pub struct BlockAssessments {
    map_: HashMap<InstructionOperand, Box<Assessment>>,
    map_for_moves_: HashMap<InstructionOperand, Box<Assessment>>,
    stale_ref_stack_slots_: HashSet<InstructionOperand>,
    spill_slot_delta_: i32,
    zone_: Rc<RefCell<Zone>>,
    sequence_: *const InstructionSequence,
}

impl BlockAssessments {
    pub fn new(
        zone: Rc<RefCell<Zone>>,
        spill_slot_delta: i32,
        sequence: *const InstructionSequence,
    ) -> Self {
        BlockAssessments {
            map_: HashMap::new(),
            map_for_moves_: HashMap::new(),
            stale_ref_stack_slots_: HashSet::new(),
            spill_slot_delta_: spill_slot_delta,
            zone_: zone,
            sequence_: sequence,
        }
    }

    pub fn drop(&mut self, operand: InstructionOperand) {
        self.map_.remove(&operand);
        self.stale_ref_stack_slots_.remove(&operand);
    }

    pub fn drop_registers(&mut self) {
        self.map_.retain(|op, _| !op.is_any_register());
    }

    pub fn add_definition(&mut self, operand: InstructionOperand, virtual_register: i32) {
        if self.map_.contains_key(&operand) {
            self.map_.remove(&operand);
            self.stale_ref_stack_slots_.remove(&operand);
        }
        let zone = self.zone_.borrow();
        self.map_.insert(
            operand,
            zone.new_final_assessment(virtual_register),
        );
    }

    pub fn perform_moves(&mut self, instruction: &Instruction) {
        if let Some(first) = instruction.get_parallel_move(Instruction::GapPosition::START) {
            self.perform_parallel_moves(first);
        }
        if let Some(last) = instruction.get_parallel_move(Instruction::GapPosition::END) {
            self.perform_parallel_moves(last);
        }
    }

    pub fn perform_parallel_moves(&mut self, moves: &ParallelMove) {
        if moves.moves.is_empty() {
            return;
        }

        self.map_for_moves_.clear();
        for move_operands in &moves.moves {
            if move_operands.is_eliminated() || move_operands.is_redundant() {
                continue;
            }
            if let Some(it) = self.map_.get(&move_operands.source()) {
                if self.map_for_moves_.get(&move_operands.destination()).is_none() {
                    if !self.is_stale_reference_stack_slot(move_operands.source(), None) {
                        self.map_for_moves_.insert(move_operands.destination(), it.clone());
                    }
                }
            }
        }
        for (op, assessment) in self.map_for_moves_.drain() {
            self.map_.remove(&op);
            self.map_.insert(op, assessment);
            self.stale_ref_stack_slots_.remove(&op);
        }
    }

    pub fn copy_from(&mut self, other: &BlockAssessments) {
        if !self.map_.is_empty() {
            return;
        }
        if !self.stale_ref_stack_slots_.is_empty() {
            return;
        }
        for (key, value) in other.map_.iter() {
            self.map_.insert(*key, value.clone());
        }
        self.stale_ref_stack_slots_
            .extend(other.stale_ref_stack_slots_.iter());
    }

    pub fn check_reference_map(&mut self, reference_map: &ReferenceMap) {
        for (op, _pair) in self.map_.iter() {
            if op.is_stack_slot() {
                if let InstructionOperand::StackSlot(loc_op) = op {
                    if Self::can_be_tagged_or_compressed_pointer(loc_op.representation)
                        && loc_op.index >= self.spill_slot_delta_
                    {
                        self.stale_ref_stack_slots_.insert(*op);
                    }
                }
            }
        }

        for ref_map_operand in reference_map.reference_operands() {
            if ref_map_operand.is_stack_slot() {
                if let Some(pair) = self.map_.get(&ref_map_operand) {
                    self.stale_ref_stack_slots_.remove(&ref_map_operand);
                }
            }
        }
    }

    pub fn is_stale_reference_stack_slot(
        &self,
        op: InstructionOperand,
        vreg: Option<i32>,
    ) -> bool {
        if !op.is_stack_slot() {
            return false;
        }
        let sequence = unsafe { &*self.sequence_ };
        if vreg.is_some() && !sequence.is_reference(vreg.unwrap()) {
            return false;
        }

        if let InstructionOperand::StackSlot(loc_op) = &op {
            if Self::can_be_tagged_or_compressed_pointer(loc_op.representation)
                && self.stale_ref_stack_slots_.contains(&op)
            {
                return true;
            }
        }
        false
    }

    fn can_be_tagged_or_compressed_pointer(rep: MachineRepresentation) -> bool {
        true
    }

    pub fn print(&self) {
        let mut os = StdoutStream::new();
        for (op, assessment) in &self.map_ {
            write!(os, "{:?} : ", op).unwrap();
            match assessment {
                Assessment::Final(final_assessment) => {
                    write!(os, "v{}", final_assessment.virtual_register()).unwrap();
                }
                Assessment::Pending(_) => {
                    write!(os, "P").unwrap();
                }
            }
            if self.stale_ref_stack_slots_.contains(op) {
                write!(os, " (stale reference)").unwrap();
            }
            writeln!(os).unwrap();
        }
        writeln!(os).unwrap();
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ConstraintType {
    kConstant,
    kImmediate,
    kRegister,
    kFixedRegister,
    kFPRegister,
    kFixedFPRegister,
    kSlot,
    kFixedSlot,
    kRegisterOrSlot,
    kRegisterOrSlotFP,
    kRegisterOrSlotOrConstant,
    kSameAsInput,
    kRegisterAndSlot,
}

#[derive(Debug, Clone, Copy)]
pub struct OperandConstraint {
    type_: ConstraintType,
    value_: i32,
    spilled_slot_: i32,
    virtual_register_: i32,
}

impl OperandConstraint {
    pub fn new() -> Self {
        OperandConstraint {
            type_: ConstraintType::kConstant,
            value_: i32::MIN,
            spilled_slot_: 0,
            virtual_register_: InstructionOperand::kInvalidVirtualRegister,
        }
    }
}

pub struct InstructionConstraint {
    instruction_: *const Instruction,
    operand_constaints_size_: usize,
    operand_constraints_: Vec<OperandConstraint>,
}

impl InstructionConstraint {
    pub fn new(instruction_: *const Instruction, operand_constaints_size_: usize, operand_constraints_: Vec<OperandConstraint>) -> Self {
        InstructionConstraint {
            instruction_,
            operand_constaints_size_,
            operand_constraints_,
        }
    }
}

pub struct DelayedAssessments {
    map_: HashMap<InstructionOperand, i32>,
}

impl DelayedAssessments {
    pub fn new(zone: &Zone) -> Self {
        DelayedAssessments {
            map_: HashMap::new(),
        }
    }

    pub fn add_delayed_assessment(&mut self, op: InstructionOperand, vreg: i32) {
        if self.map_.contains_key(&op) {
            if self.map_.get(&op).unwrap() != &vreg {
                panic!("CHECK_EQ(it->second, vreg) failed");
            }
        } else {
            self.map_.insert(op, vreg);
        }
    }
}

pub struct RegisterAllocatorVerifier {
    zone_: Rc<RefCell<Zone>>,
    config_: *const RegisterConfiguration,
    sequence_: *const InstructionSequence,
    constraints_: Vec<InstructionConstraint>,
    assessments_: HashMap<RpoNumber, Rc<RefCell<BlockAssessments>>>,
    outstanding_assessments_: HashMap<RpoNumber, Rc<RefCell<DelayedAssessments>>>,
    spill_slot_delta_: i32,
    caller_info_: String,
}

impl RegisterAllocatorVerifier {
    pub fn new(
        zone: Rc<RefCell<Zone>>,
        config: *const RegisterConfiguration,
        sequence: *const InstructionSequence,
        frame: *const Frame,
    ) -> Self {
        let spill_slot_delta_ = unsafe {
            (*frame).get_total_frame_slot_count() - (*frame).get_spill_slot_count()
        };
        let mut verifier = RegisterAllocatorVerifier {
            zone_: zone.clone(),
            config_: config,
            sequence_: sequence,
            constraints_: Vec::new(),
            assessments_: HashMap::new(),
            outstanding_assessments_: HashMap::new(),
            spill_slot_delta_: spill_slot_delta_,
            caller_info_: String::new(),
        };

        let sequence_ref = unsafe { &*sequence };
        verifier.constraints_.reserve(sequence_ref.instructions().len());

        for instr in sequence_ref.instructions() {
            Self::verify_empty_gaps(instr);
            let operand_count = Self::operand_count(instr);
            let mut op_constraints = Vec::with_capacity(operand_count);
            for _ in 0..operand_count {
                op_constraints.push(OperandConstraint::new());
            }

            let mut count = 0;
            for i in 0..instr.input_count() {
                let op = instr.input_at(i);
                verifier.build_constraint(op, &mut op_constraints[count]);
                Self::verify_input(&op_constraints[count]);
                count += 1;
            }

            for i in 0..instr.temp_count() {
                let op = instr.temp_at(i);
                verifier.build_constraint(op, &mut op_constraints[count]);
                Self::verify_temp(&op_constraints[count]);
                count += 1;
            }

            for i in 0..instr.output_count() {
                let op = instr.output_at(i);
                verifier.build_constraint(op, &mut op_constraints[count]);
                if op_constraints[count].type_ == ConstraintType::kSameAsInput {
                    let input_index = op_constraints[count].value_;
                    if input_index >= instr.input_count() as i32 {
                        panic!("CHECK_LT(input_index, instr->InputCount()) failed");
                    }
                    let input_index = input_index as usize;
                    op_constraints[count].type_ = op_constraints[input_index].type_;
                    op_constraints[count].value_ = op_constraints[input_index].value_;
                }
                Self::verify_output(&op_constraints[count]);
                count += 1;
            }

            let instr_constraint = InstructionConstraint::new(
                instr,
                operand_count,
                op_constraints,
            );
            verifier.constraints_.push(instr_constraint);
        }

        verifier
    }

    fn verify_input(constraint: &OperandConstraint) {
        if constraint.type_ != ConstraintType::kImmediate {
            if constraint.virtual_register_ == InstructionOperand::kInvalidVirtualRegister {
                panic!("CHECK_NE(InstructionOperand::kInvalidVirtualRegister, constraint.virtual_register_) failed");
            }
        }
    }

    fn verify_temp(constraint: &OperandConstraint) {
        if constraint.type_ == ConstraintType::kImmediate {
            panic!("CHECK_NE(kImmediate, constraint.type_) failed");
        }
        if constraint.type_ == ConstraintType::kConstant {
            panic!("CHECK_NE(kConstant, constraint.type_) failed");
        }
    }

    fn verify_output(constraint: &OperandConstraint) {
        if constraint.type_ == ConstraintType::kImmediate {
            panic!("CHECK_NE(kImmediate, constraint.type_) failed");
        }
        if constraint.virtual_register_ == InstructionOperand::kInvalidVirtualRegister {
            panic!("CHECK_NE(InstructionOperand::kInvalidVirtualRegister, constraint.virtual_register_) failed");
        }
    }

    pub fn verify_assignment(&mut self, caller_info: String) {
        self.caller_info_ = caller_info;
        let sequence_ref = unsafe { &*self.sequence_ };
        if sequence_ref.instructions().len() != self.constraints_.len() {
            panic!("CHECK(sequence()->instructions().size() == constraints()->size()) failed");
        }
        let mut instr_it = sequence_ref.instructions().iter();
        for instr_constraint in &self.constraints_ {
            let instr = instr_constraint.instruction_;

            Self::verify_allocated_gaps(instr, &self.caller_info_);
            let operand_count = instr_constraint.operand_constaints_size_;
            let op_constraints = &instr_constraint.operand_constraints_;

            let instr_it_next = instr_it.next().unwrap();
            if instr != instr_it_next {
                panic!("CHECK_EQ(instr, *instr_it) failed");
            }
            if operand_count != Self::operand_count(instr) {
                panic!("CHECK(operand_count == OperandCount(instr)) failed");
            }

            let mut count = 0;
            for i in 0..unsafe { (*instr).input_count() } {
                self.check_constraint(unsafe { (*instr).input_at(i) }, &op_constraints[count]);
                count += 1;
            }
            for i in 0..unsafe { (*instr).temp_count() } {
                self.check_constraint(unsafe { (*instr).temp_at(i) }, &op_constraints[count]);
                count += 1;
            }
            for i in 0..unsafe { (*instr).output_count() } {
                self.check_constraint(unsafe { (*instr).output_at(i) }, &op_constraints[count]);
                count += 1;
            }
        }
    }

    fn build_constraint(&self, op: &InstructionOperand, constraint: &mut OperandConstraint) {
        constraint.value_ = i32::MIN;
        constraint.virtual_register_ = InstructionOperand::kInvalidVirtualRegister;
        if op.is_constant() {
            constraint.type_ = ConstraintType::kConstant;
            constraint.value_ = op.get_virtual_register();
            constraint.virtual_register_ = constraint.value_;
        } else if op.is_immediate() {
            constraint.type_ = ConstraintType::kImmediate;
            constraint.value_ = Self::get_value(op);
        } else {
            if !op.is_unallocated() {
                panic!("CHECK(op->IsUnallocated()) failed");
            }
            let vreg = op.get_virtual_register();
            constraint.virtual_register_ = vreg;
            if op.basic_policy() == InstructionOperand::FIXED_SLOT {
                constraint.type_ = ConstraintType::kFixedSlot;
                constraint.value_ = op.fixed_slot_index();
            } else {
                match op.extended_policy() {
                    InstructionOperand::REGISTER_OR_SLOT | InstructionOperand::NONE => {
                        let sequence_ref = unsafe { &*self.sequence_ };
                        if sequence_ref.is_fp(vreg) {
                            constraint.type_ = ConstraintType::kRegisterOrSlotFP;
                        } else {
                            constraint.type_ = ConstraintType::kRegisterOrSlot;
                        }
                    }
                    InstructionOperand::REGISTER_OR_SLOT_OR_CONSTANT => {
                        let sequence_ref = unsafe { &*self.sequence_ };
                        if sequence_ref.is_fp(vreg) {
                            panic!("DCHECK(!sequence()->IsFP(vreg)) failed");
                        }
                        constraint.type_ = ConstraintType::kRegisterOrSlotOrConstant;
                    }
                    InstructionOperand::FIXED_REGISTER => {
                        if op.has_secondary_storage() {
                            constraint.type_ = ConstraintType::kRegisterAndSlot;
                            constraint.spilled_slot_ = op.get_secondary_storage();
                        } else {
                            constraint.type_ = ConstraintType::kFixedRegister;
                        }
                        constraint.value_ = op.fixed_register_index();
                    }
                    InstructionOperand::FIXED_FP_REGISTER => {
                        constraint.type_ = ConstraintType::kFixedFPRegister;
                        constraint.value_ = op.fixed_register_index();
                    }
                    InstructionOperand::MUST_HAVE_REGISTER => {
                        let sequence_ref = unsafe { &*self.sequence_ };
                        if sequence_ref.is_fp(vreg) {
                            constraint.type_ = ConstraintType::kFPRegister;
                        } else {
                            constraint.type_ = ConstraintType::kRegister;
                        }
                    }
                    InstructionOperand::MUST_HAVE_SLOT => {
                        constraint.type_ = ConstraintType::kSlot;
                        let sequence_ref = unsafe { &*self.sequence_ };
                        constraint.value_ =
                            Self::element_size_log2_of(sequence_ref.get_representation(vreg));
                    }
                    InstructionOperand::SAME_AS_INPUT => {
                        constraint.type_ = ConstraintType::kSameAsInput;
                        constraint.value_ = op.input_index();
                    }
                }
            }
        }
    }

    fn check_constraint(&self, op: &InstructionOperand, constraint: &OperandConstraint) {
        match constraint.type_ {
            ConstraintType::kConstant => {
                if !op.is_constant() {
                    panic!("CHECK_WITH_MSG(op->IsConstant(), caller_info_) failed");
                }
                if op.get_virtual_register() != constraint.value_ {
                    panic!("CHECK_EQ(ConstantOperand::cast(op)->virtual_register(), constraint->value_) failed");
                }
            }
            ConstraintType::kImmediate => {
                if !op.is_immediate() {
                    panic!("CHECK_WITH_MSG(op->IsImmediate(), caller_info_) failed");
                }
                let value = Self::get_value(op);
                if value != constraint.value_ {
                    panic!("CHECK_EQ(value, constraint->value_) failed");
                }
            }
            ConstraintType::kRegister => {
                if !op.is_register() {
                    panic!("CHECK_WITH_MSG(op->IsRegister(), caller_info_) failed");
                }
            }
            ConstraintType::kFPRegister => {
                if !op.is_fp_register() {
                    panic!("CHECK_WITH_MSG(op->IsFPRegister(), caller_info_) failed");
                }
            }
            ConstraintType::kFixedRegister | ConstraintType::kRegisterAndSlot => {
                if !op.is_register() {
                    panic!("CHECK_WITH_MSG(op->IsRegister(), caller_info_) failed");
                }
                if op.register_code() != constraint.value_ {
                    panic!("CHECK_EQ(LocationOperand::cast(op)->register_code(), constraint->value_) failed");
                }
            }
            ConstraintType::kFixedFPRegister => {
                if !op.is_fp_register() {
                    panic!("CHECK_WITH_MSG(op->IsFPRegister(), caller_info_) failed");
                }
                if op.register_code() != constraint.value_ {
                    panic!("CHECK_EQ(LocationOperand::cast(op)->register_code(), constraint->value_) failed");
                }
            }
            ConstraintType::kFixedSlot => {
                if !op.is_stack_slot() && !op.is_fp_stack_slot() {
                    panic!("CHECK_WITH_MSG(op->IsStackSlot() || op->IsFPStackSlot(), caller_info_) failed");
                }
                if op.index() != constraint.value_ {
                    panic!("CHECK_EQ(LocationOperand::cast(op)->index(), constraint->value_) failed");
                }
            }
            ConstraintType::kSlot => {
                if !op.is_stack_slot() && !op.is_fp_stack_slot() {
                    panic!("CHECK_WITH_MSG(op->IsStackSlot() || op->IsFPStackSlot(), caller_info_) failed");
                }
                if Self::element_size_log2_of(op.representation()) != constraint.value_ {
                    panic!("CHECK_EQ(ElementSizeLog2Of(LocationOperand::cast(op)->representation()), constraint->value_) failed");
                }
            }
            ConstraintType::kRegisterOrSlot => {
                if !op.is_register() && !op.is_stack_slot() {
                    panic!("CHECK_WITH_MSG(op->IsRegister() || op->IsStackSlot(), caller_info_) failed");
                }
            }
            ConstraintType::kRegisterOrSlotFP => {
                if
