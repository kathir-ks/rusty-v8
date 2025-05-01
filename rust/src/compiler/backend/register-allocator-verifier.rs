// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/compiler/backend/register-allocator-verifier.h
mod register_allocator_verifier {
    use std::cell::RefCell;
    use std::collections::{HashMap, HashSet};
    use std::fmt;
    use std::rc::Rc;

    // Placeholder for Instruction
    #[derive(Debug)]
    pub struct Instruction {
        inputs: Vec<InstructionOperand>,
        temps: Vec<InstructionOperand>,
        outputs: Vec<InstructionOperand>,
        parallel_move_start: Option<ParallelMove>,
        parallel_move_end: Option<ParallelMove>,
        reference_map: Option<ReferenceMap>,
    }

    impl Instruction {
        pub const FIRST_GAP_POSITION: usize = 0;
        pub const LAST_GAP_POSITION: usize = 1;

        pub fn input_count(&self) -> usize {
            self.inputs.len()
        }

        pub fn temp_count(&self) -> usize {
            self.temps.len()
        }

        pub fn output_count(&self) -> usize {
            self.outputs.len()
        }

        pub fn input_at(&self, index: usize) -> &InstructionOperand {
            &self.inputs[index]
        }

        pub fn temp_at(&self, index: usize) -> &InstructionOperand {
            &self.temps[index]
        }

        pub fn output_at(&self, index: usize) -> &InstructionOperand {
            &self.outputs[index]
        }

        pub fn get_parallel_move(&self, pos: usize) -> &Option<ParallelMove> {
            match pos {
                0 => &self.parallel_move_start,
                1 => &self.parallel_move_end,
                _ => panic!("Invalid GapPosition"),
            }
        }

        pub fn is_call(&self) -> bool {
            // Placeholder implementation
            false
        }

        pub fn has_reference_map(&self) -> bool {
            self.reference_map.is_some()
        }

        pub fn reference_map(&self) -> &Option<ReferenceMap> {
            &self.reference_map
        }
    }

    // Placeholder for InstructionOperand
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum InstructionOperand {
        Constant(ConstantOperand),
        Immediate(ImmediateOperand),
        Unallocated(UnallocatedOperand),
        Allocated(AllocatedOperand),
        InvalidVirtualRegister,
    }

    impl InstructionOperand {
        pub fn is_constant(&self) -> bool {
            matches!(self, InstructionOperand::Constant(_))
        }
        pub fn is_immediate(&self) -> bool {
            matches!(self, InstructionOperand::Immediate(_))
        }
        pub fn is_unallocated(&self) -> bool {
            matches!(self, InstructionOperand::Unallocated(_))
        }
        pub fn is_register(&self) -> bool {
            matches!(self, InstructionOperand::Allocated(AllocatedOperand{location_kind: LocationOperand::LocationKind::REGISTER, ..}))
        }
        pub fn is_fp_register(&self) -> bool {
            matches!(self, InstructionOperand::Allocated(AllocatedOperand{location_kind: LocationOperand::LocationKind::FP_REGISTER, ..}))
        }
        pub fn is_stack_slot(&self) -> bool {
            matches!(self, InstructionOperand::Allocated(AllocatedOperand{location_kind: LocationOperand::LocationKind::STACK_SLOT, ..}))
        }
        pub fn is_fp_stack_slot(&self) -> bool {
            matches!(self, InstructionOperand::Allocated(AllocatedOperand{location_kind: LocationOperand::LocationKind::FP_STACK_SLOT, ..}))
        }
        pub fn is_any_register(&self) -> bool {
            self.is_register() || self.is_fp_register()
        }
    }

    // Placeholder for ConstantOperand
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct ConstantOperand {
        virtual_register: i32,
    }

    impl ConstantOperand {
        pub fn cast(op: &InstructionOperand) -> &ConstantOperand {
            match op {
                InstructionOperand::Constant(operand) => operand,
                _ => panic!("Invalid cast to ConstantOperand"),
            }
        }

        pub fn virtual_register(&self) -> i32 {
            self.virtual_register
        }
    }

    // Placeholder for ImmediateOperand
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct ImmediateOperand {
        value: i64,
        operand_type: ImmediateOperandType,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum ImmediateOperandType {
        INLINE_INT32,
        INLINE_INT64,
        INDEXED_RPO,
        INDEXED_IMM,
    }

    impl ImmediateOperand {
        pub fn cast(op: &InstructionOperand) -> &ImmediateOperand {
            match op {
                InstructionOperand::Immediate(operand) => operand,
                _ => panic!("Invalid cast to ImmediateOperand"),
            }
        }

        pub fn inline_int32_value(&self) -> i32 {
            self.value as i32
        }
        pub fn inline_int64_value(&self) -> i64 {
            self.value
        }
        pub fn indexed_value(&self) -> i32 {
            self.value as i32
        }
        pub fn operand_type(&self) -> &ImmediateOperandType {
            &self.operand_type
        }
    }

    // Placeholder for UnallocatedOperand
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct UnallocatedOperand {
        virtual_register: i32,
        basic_policy: UnallocatedOperandBasicPolicy,
        extended_policy: UnallocatedOperandExtendedPolicy,
        fixed_slot_index: i32,
        fixed_register_index: i32,
        input_index: usize,
        has_secondary_storage: bool,
        secondary_storage: i32,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum UnallocatedOperandBasicPolicy {
        FIXED_SLOT,
        // Other policies can be added here as needed
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum UnallocatedOperandExtendedPolicy {
        REGISTER_OR_SLOT,
        NONE,
        REGISTER_OR_SLOT_OR_CONSTANT,
        FIXED_REGISTER,
        FIXED_FP_REGISTER,
        MUST_HAVE_REGISTER,
        MUST_HAVE_SLOT,
        SAME_AS_INPUT,
        // Other policies can be added here as needed
    }

    impl UnallocatedOperand {
        pub fn cast(op: &InstructionOperand) -> &UnallocatedOperand {
            match op {
                InstructionOperand::Unallocated(operand) => operand,
                _ => panic!("Invalid cast to UnallocatedOperand"),
            }
        }

        pub fn virtual_register(&self) -> i32 {
            self.virtual_register
        }
        pub fn basic_policy(&self) -> &UnallocatedOperandBasicPolicy {
            &self.basic_policy
        }
        pub fn extended_policy(&self) -> &UnallocatedOperandExtendedPolicy {
            &self.extended_policy
        }
        pub fn fixed_slot_index(&self) -> i32 {
            self.fixed_slot_index
        }
        pub fn fixed_register_index(&self) -> i32 {
            self.fixed_register_index
        }
        pub fn input_index(&self) -> usize {
            self.input_index
        }
        pub fn has_secondary_storage(&self) -> bool {
            self.has_secondary_storage
        }
        pub fn get_secondary_storage(&self) -> i32 {
            self.secondary_storage
        }
    }

    // Placeholder for AllocatedOperand
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct AllocatedOperand {
        location_kind: LocationOperand::LocationKind,
        representation: MachineRepresentation,
        index: i32,
        register_code: i32,
    }

    impl AllocatedOperand {
        pub fn cast(op: &InstructionOperand) -> &AllocatedOperand {
            match op {
                InstructionOperand::Allocated(operand) => operand,
                _ => panic!("Invalid cast to AllocatedOperand"),
            }
        }

        pub fn new(zone: &Zone, location_kind: LocationOperand::LocationKind, representation: MachineRepresentation, index: i32) -> Self {
            AllocatedOperand {
                location_kind,
                representation,
                index,
                register_code: 0, // Default value
            }
        }

        pub fn representation(&self) -> MachineRepresentation {
            self.representation
        }

    }

    // Placeholder for LocationOperand
    pub mod LocationOperand {
        use super::MachineRepresentation;

        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub enum LocationKind {
            REGISTER,
            FP_REGISTER,
            STACK_SLOT,
            FP_STACK_SLOT,
        }

        // Removed LocationOperand struct as its functionality is now integrated into AllocatedOperand

        pub fn cast(op: &super::InstructionOperand) -> &super::AllocatedOperand {
            super::AllocatedOperand::cast(op)
        }

        pub fn index(operand: &super::AllocatedOperand) -> i32 {
            operand.index
        }

        pub fn register_code(operand: &super::AllocatedOperand) -> i32 {
            operand.register_code
        }

        pub fn representation(operand: &super::AllocatedOperand) -> MachineRepresentation {
            operand.representation
        }
    }

    // Placeholder for ParallelMove
    #[derive(Debug)]
    pub struct ParallelMove {
        moves: Vec<MoveOperands>,
    }

    impl ParallelMove {
        pub fn iter(&self) -> std::slice::Iter<MoveOperands> {
            self.moves.iter()
        }
    }

    impl IntoIterator for ParallelMove {
        type Item = MoveOperands;
        type IntoIter = std::vec::IntoIter<Self::Item>;

        fn into_iter(self) -> Self::IntoIter {
            self.moves.into_iter()
        }
    }

    // Placeholder for MoveOperands
    #[derive(Debug, Clone)]
    pub struct MoveOperands {
        source: InstructionOperand,
        destination: InstructionOperand,
        is_redundant: bool,
        is_eliminated: bool,
    }

    impl MoveOperands {
        pub fn source(&self) -> &InstructionOperand {
            &self.source
        }
        pub fn destination(&self) -> &InstructionOperand {
            &self.destination
        }
        pub fn is_redundant(&self) -> bool {
            self.is_redundant
        }
        pub fn is_eliminated(&self) -> bool {
            self.is_eliminated
        }
    }

    // Placeholder for ReferenceMap
    #[derive(Debug)]
    pub struct ReferenceMap {
        reference_operands: Vec<InstructionOperand>,
    }

    impl ReferenceMap {
        pub fn reference_operands(&self) -> &Vec<InstructionOperand> {
            &self.reference_operands
        }
    }

    // Placeholder for InstructionSequence
    #[derive(Debug)]
    pub struct InstructionSequence {
        instructions: Vec<Instruction>,
        fp_virtual_registers: HashSet<i32>,
        instruction_blocks: Vec<InstructionBlock>,
        representations: HashMap<i32, MachineRepresentation>,
    }

    impl InstructionSequence {
        pub fn instructions(&self) -> &Vec<Instruction> {
            &self.instructions
        }
        pub fn is_fp(&self, vreg: i32) -> bool {
            self.fp_virtual_registers.contains(&vreg)
        }

        pub fn instruction_blocks(&self) -> &Vec<InstructionBlock> {
            &self.instruction_blocks
        }

        pub fn get_representation(&self, vreg: i32) -> MachineRepresentation {
            *self.representations.get(&vreg).unwrap()
        }

        pub fn is_reference(&self, vreg: i32) -> bool {
            // Placeholder Implementation
            true
        }
    }

    // Placeholder for InstructionBlock
    #[derive(Debug)]
    pub struct InstructionBlock {
        rpo_number: RpoNumber,
        predecessors: Vec<RpoNumber>,
        code_start: usize,
        code_end: usize,
        phis: Vec<PhiInstruction>,
        is_loop_header: bool,
    }

    impl InstructionBlock {
        pub fn rpo_number(&self) -> RpoNumber {
            self.rpo_number
        }
        pub fn predecessors(&self) -> &Vec<RpoNumber> {
            &self.predecessors
        }
        pub fn predecessor_count(&self) -> usize {
            self.predecessors.len()
        }
        pub fn code_start(&self) -> usize {
            self.code_start
        }
        pub fn code_end(&self) -> usize {
            self.code_end
        }
        pub fn phis(&self) -> &Vec<PhiInstruction> {
            &self.phis
        }
        pub fn is_loop_header(&self) -> bool {
            self.is_loop_header
        }
    }

    // Placeholder for RpoNumber
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct RpoNumber {
        number: i32,
    }

    impl RpoNumber {
        pub fn to_int(&self) -> i32 {
            self.number
        }
    }

    // Placeholder for PhiInstruction
    #[derive(Debug)]
    pub struct PhiInstruction {
        virtual_register: i32,
        operands: Vec<i32>,
    }

    impl PhiInstruction {
        pub fn virtual_register(&self) -> i32 {
            self.virtual_register
        }

        pub fn operands(&self) -> &Vec<i32> {
            &self.operands
        }
    }

    // Placeholder for RegisterConfiguration
    #[derive(Debug)]
    pub struct RegisterConfiguration {}

    // Placeholder for Frame
    #[derive(Debug)]
    pub struct Frame {
        total_frame_slot_count: i32,
        spill_slot_count: i32,
    }

    impl Frame {
        pub fn get_total_frame_slot_count(&self) -> i32 {
            self.total_frame_slot_count
        }
        pub fn get_spill_slot_count(&self) -> i32 {
            self.spill_slot_count
        }
    }

    // Placeholder for Zone
    #[derive(Debug)]
    pub struct Zone {
        allocator: ZoneAllocator,
    }

    impl Zone {
        pub fn new() -> Self {
            Zone { allocator: ZoneAllocator::new() }
        }

        pub fn allocator(&self) -> &ZoneAllocator {
            &self.allocator
        }

        pub fn allocate_array<T>(&self, count: usize) -> Vec<T>
        where
            T: Default + Clone,
        {
            vec![T::default(); count]
        }

        pub fn new_block_assessments(&self, spill_slot_delta: i32, sequence: &InstructionSequence) -> BlockAssessments {
            BlockAssessments::new(self, spill_slot_delta, sequence)
        }

        pub fn new_pending_assessment(&self, origin: &InstructionBlock, operand: InstructionOperand) -> PendingAssessment {
            PendingAssessment::new(self, origin, operand)
        }
    }

    // Placeholder for ZoneAllocator
    #[derive(Debug)]
    pub struct ZoneAllocator {}

    impl ZoneAllocator {
        pub fn new() -> Self {
            ZoneAllocator {}
        }
    }

    // OperandConstraint enum
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct OperandConstraint {
        pub type_: ConstraintType,
        pub value_: i32,
        pub virtual_register_: i32,
        pub spilled_slot_: i32,
    }

    impl Default for OperandConstraint {
        fn default() -> Self {
            OperandConstraint {
                type_: ConstraintType::kImmediate, // Default value
                value_: i32::MIN,
                virtual_register_: InstructionOperand::InvalidVirtualRegister as i32,
                spilled_slot_: 0,
            }
        }
    }

    // ConstraintType enum
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum ConstraintType {
        kConstant,
        kImmediate,
        kRegister,
        kFPRegister,
        kFixedRegister,
        kFixedFPRegister,
        kFixedSlot,
        kSlot,
        kRegisterOrSlot,
        kRegisterOrSlotFP,
        kRegisterOrSlotOrConstant,
        kSameAsInput,
        kRegisterAndSlot,
    }

    // InstructionConstraint struct
    #[derive(Debug)]
    pub struct InstructionConstraint {
        pub instruction_: *const Instruction,
        pub operand_constaints_size_: usize,
        pub operand_constraints_: *mut OperandConstraint,
    }

    // Struct for RegisterAllocatorVerifier
    pub struct RegisterAllocatorVerifier<'a> {
        zone_: &'a Zone,
        config_: *const RegisterConfiguration,
        sequence_: *const InstructionSequence,
        constraints_: Vec<InstructionConstraint>,
        assessments_: HashMap<RpoNumber, BlockAssessments>,
        outstanding_assessments_: HashMap<RpoNumber, DelayedAssessments>,
        spill_slot_delta_: i32,
        caller_info_: &'static str,
    }

    impl<'a> RegisterAllocatorVerifier<'a> {
        pub fn new(
            zone: &'a Zone,
            config: *const RegisterConfiguration,
            sequence: *const InstructionSequence,
            frame: *const Frame,
        ) -> Self {
            let sequence_ref = unsafe { &*sequence };
            let frame_ref = unsafe { &*frame };

            let mut verifier = RegisterAllocatorVerifier {
                zone_: zone,
                config_: config,
                sequence_: sequence,
                constraints_: Vec::new(),
                assessments_: HashMap::new(),
                outstanding_assessments_: HashMap::new(),
                spill_slot_delta_: unsafe { (*frame).get_total_frame_slot_count() - (*frame).get_spill_slot_count() },
                caller_info_: "",
            };

            verifier.constraints_.reserve(sequence_ref.instructions().len());

            for instr in sequence_ref.instructions() {
                verify_empty_gaps(instr);

                let operand_count = operand_count(instr);
                let mut op_constraints = verifier.zone_.allocate_array::<OperandConstraint>(operand_count);
                let mut count = 0;

                for i in 0..instr.input_count() {
                    verifier.build_constraint(instr.input_at(i), &mut op_constraints[count]);
                    verifier.verify_input(op_constraints[count].clone());
                    count += 1;
                }
                for i in 0..instr.temp_count() {
                    verifier.build_constraint(instr.temp_at(i), &mut op_constraints[count]);
                    verifier.verify_temp(op_constraints[count].clone());
                    count += 1;
                }
                for i in 0..instr.output_count() {
                    verifier.build_constraint(instr.output_at(i), &mut op_constraints[count]);
                    if op_constraints[count].type_ == ConstraintType::kSameAsInput {
                        let input_index = op_constraints[count].value_ as usize;
                        assert!(input_index < instr.input_count());
                        op_constraints[count].type_ = op_constraints[input_index].type_;
                        op_constraints[count].value_ = op_constraints[input_index].value_;
                    }
                    verifier.verify_output(op_constraints[count].clone());
                    count += 1;
                }

                let instr_constraint = InstructionConstraint {
                    instruction_: instr,
                    operand_constaints_size_: operand_count,
                    operand_constraints_: op_constraints.as_mut_ptr(),
                };

                verifier.constraints_.push(instr_constraint);
            }

            verifier
        }

        fn sequence(&self) -> &InstructionSequence {
            unsafe { &*self.sequence_ }
        }

        fn constraints(&self) -> &Vec<InstructionConstraint> {
            &self.constraints_
        }

        fn verify_input(&self, constraint: OperandConstraint) {
            assert_ne!(ConstraintType::kSameAsInput, constraint.type_);
            if constraint.type_ != ConstraintType::kImmediate {
                assert_ne!(
                    InstructionOperand::InvalidVirtualRegister as i32,
                    constraint.virtual_register_
                );
            }
        }

        fn verify_temp(&self, constraint: OperandConstraint) {
            assert_ne!(ConstraintType::kSameAsInput, constraint.type_);
            assert_ne!(ConstraintType::kImmediate, constraint.type_);
            assert_ne!(ConstraintType::kConstant, constraint.type_);
        }

        fn verify_output(&self, constraint: OperandConstraint) {
            assert_ne!(ConstraintType::kImmediate, constraint.type_);
            assert_ne!(
                InstructionOperand::InvalidVirtualRegister as i32,
                constraint.virtual_register_
            );
        }

        pub fn verify_assignment(&mut self, caller_info: &'static str) {
            self.caller_info_ = caller_info;
            assert_eq!(
                self.sequence().instructions().len(),
                self.constraints().len()
            );

            let mut instr_it = self.sequence().instructions().iter();
            for instr_constraint in self.constraints() {
                let instr = unsafe { &*instr_constraint.instruction_ };
                verify_allocated_gaps(instr, self.caller_info_);
                let operand_count = instr_constraint.operand_constaints_size_;
                let op_constraints =
                    unsafe { std::slice::from_raw_parts(instr_constraint.operand_constraints_, operand_count) };

                assert_eq!(instr, instr_it.next().unwrap());
                assert_eq!(operand_count, operand_count(instr));

                let mut count = 0;
                for i in 0..instr.input_count() {
                    self.check_constraint(instr.input_at(i), &op_constraints[count]);
                    count += 1;
                }
                for i in 0..instr.temp_count() {
                    self.check_constraint(instr.temp_at(i), &op_constraints[count]);
                    count += 1;
                }
                for i in 0..instr.output_count() {
                    self.check_constraint(instr.output_at(i), &op_constraints[count]);
                    count += 1;
                }
            }
        }

        fn build_constraint(
            &self,
            op: &InstructionOperand,
            constraint: &mut OperandConstraint,
        ) {
            constraint.value_ = i32::MIN;
            constraint.virtual_register_ = InstructionOperand::InvalidVirtualRegister as i32;
            if op.is_constant() {
                constraint.type_ = ConstraintType::kConstant;
                constraint.value_ = ConstantOperand::cast(op).virtual_register();
                constraint.virtual_register_ = constraint.value_;
            } else if op.is_immediate() {
                let imm = ImmediateOperand::cast(op);
                constraint.type_ = ConstraintType::kImmediate;
                constraint.value_ = get_value(imm);
            } else {
                assert!(op.is_unallocated());
                let unallocated = UnallocatedOperand::cast(op);
                let vreg = unallocated.virtual_register();
                constraint.virtual_register_ = vreg;
                match unallocated.basic_policy() {
                    UnallocatedOperandBasicPolicy::FIXED_SLOT => {
                        constraint.type_ = ConstraintType::kFixedSlot;
                        constraint.value_ = unallocated.fixed_slot_index();
                    }
                    // Handle other basic policies here as needed
                }
                match unallocated.extended_policy() {
                    UnallocatedOperandExtendedPolicy::REGISTER_OR_SLOT | UnallocatedOperandExtendedPolicy::NONE => {
                        if unsafe { &*self.sequence_ }.is_fp(vreg) {
                            constraint.type_ = ConstraintType::kRegisterOrSlotFP;
                        } else {
                            constraint.type_ = ConstraintType::kRegisterOrSlot;
                        }
                    }
                    UnallocatedOperandExtendedPolicy::REGISTER_OR_SLOT_OR_CONSTANT => {
                        assert!(!unsafe { &*self.sequence_ }.is_fp(vreg));
                        constraint.type_ = ConstraintType::kRegisterOrSlotOrConstant;
                    }
                    UnallocatedOperandExtendedPolicy::FIXED_REGISTER => {
                        if unallocated.has_secondary_storage() {
                            constraint.type_ = ConstraintType::kRegisterAndSlot;
                            constraint.spilled_slot_ = unallocated.get_secondary_storage();
                        } else {
                            constraint.type_ = ConstraintType::kFixedRegister;
                        }
                        constraint.value_ = unallocated.fixed_register_index();
                    }
                    UnallocatedOperandExtendedPolicy::FIXED_FP_REGISTER => {
                        constraint.type_ = ConstraintType::kFixedFPRegister;
                        constraint.value_ = unallocated.fixed_register_index();
                    }
                    UnallocatedOperandExtendedPolicy::MUST_HAVE_REGISTER => {
                        if unsafe { &*self.sequence_ }.is_fp(vreg) {
                            constraint.type_ = ConstraintType::kFPRegister;
                        } else {
                            constraint.type_ = ConstraintType::kRegister;
                        }
                    }
                    UnallocatedOperandExtendedPolicy::MUST_HAVE_SLOT => {
                        constraint.type_ = ConstraintType::kSlot;
                        constraint.value_ = element_size_log2_of(unsafe { &*self.sequence_ }.get_representation(vreg));
                    }
                    UnallocatedOperandExtendedPolicy::SAME_AS_INPUT => {
                        constraint.type_ = ConstraintType::kSameAsInput;
                        constraint.value_ = unallocated.input_index() as i32;
                    }
                }
            }
        }

        fn check_constraint(
            &self,
            op: &InstructionOperand,
            constraint: &OperandConstraint,
        ) {
            match constraint.type_ {
                ConstraintType::kConstant => {
                    assert!(op.is_constant());
                    assert_eq!(
                        ConstantOperand::cast(op).virtual_register(),
                        constraint.value_
                    );
                }
                ConstraintType::kImmediate => {
                    assert!(op.is_immediate());
                    let imm = ImmediateOperand::cast(op);
                    let value = get_value(imm);
                    assert_eq!(value, constraint.value_);
                }
                ConstraintType::kRegister => {
                    assert!(op.is_register());
                }
                ConstraintType::kFPRegister => {
                    assert!(op.is_fp_register());
                }
                ConstraintType::kFixedRegister | ConstraintType::kRegisterAndSlot => {
                    assert!(op.is_register());
                    assert_eq!(LocationOperand::cast(op).register_code, constraint.value_);
                }
                ConstraintType::kFixedFPRegister => {
                    assert!(op.is_fp_register());
                    assert_eq!(LocationOperand::cast(op).register_code, constraint.value_);
                }
                ConstraintType::kFixedSlot => {
                    assert!(op.is_stack_slot() || op.is_fp_stack_slot());
                    assert_eq!(LocationOperand::cast(op).index, constraint.value_);
                }
                ConstraintType::kSlot => {
                    assert!(op.is_stack_slot() || op.is_fp_stack_slot());
                    assert_eq!(
                        element_size_log2_of(LocationOperand::cast(op).representation()),
                        constraint.value_
                    );
                }
                ConstraintType::kRegisterOrSlot => {
                    assert!(op.is_register() || op.is_stack_slot());
                }
                ConstraintType::kRegisterOrSlotFP => {
                    assert!(op.is_fp_register() || op.is_fp_stack_slot());
                }
                ConstraintType::kRegisterOrSlotOrConstant => {
                    assert!(op.is_register() || op.is_stack_slot() || op.is_constant());
                }
                ConstraintType::kSameAsInput => {
                    assert!(false, "{}", self.caller_info_);
                }
            }
        }

        fn create_for_block(&self, block: &InstructionBlock) -> BlockAssessments {
            let current_block_id = block.rpo_number();
            let zone = self.zone_;

            let mut ret = BlockAssessments::new(zone, self.spill_slot_delta_, unsafe { &*self.sequence_ });

            if block.predecessor_count() == 0 {
                // Empty block assessments for the first block
            } else if block.predecessor_count() == 1 && block.phis().is_empty() {
                let prev_block = &self.assessments_[&block.predecessors()[0]];
                ret.copy_from(prev_block);
            } else {
                for pred_id in block.predecessors() {
                    let iterator = self.assessments_.get(&pred_id);
                    match iterator {
                        Some(pred_assessments) => {
                            for (operand, assessment) in pred_assessments.map().iter() {
                                if !ret.map().contains_key(operand) {
                                    let pending_assessment = self.zone_.new_pending_assessment(block, operand.clone());
                                    ret.map().insert(operand.clone(), pending_assessment);
                                }
                            }

                            ret.stale_ref_stack_slots().extend(pred_assessments.stale_ref_stack_slots().iter().cloned());
                        }
                        None => {
                            assert!(pred_id >= current_block_id);
                            assert!(block.is_loop_header());

                            let todo_iter = self.outstanding_assessments_.get(&pred_id);
                            match todo_iter {
                                Some(_todo) => {}
                                None => {
                                    self.outstanding_assessments_.insert(pred_id, DelayedAssessments::new(zone));
                                }
                            }
                        }
                    }
                }
            }
            ret
        }

        fn validate_pending_assessment(
            &self,
            block_id: RpoNumber,
            op: InstructionOperand,
            current_assessments: &BlockAssessments,
            assessment: &PendingAssessment,
            virtual_register: i32,
        ) {
            if assessment.is_alias_of(virtual_register) {
                return;
            }

            let local_zone = Zone::new();
            let mut worklist: Vec<(&PendingAssessment, i32)> = Vec::new();
            let mut seen: HashSet<RpoNumber> = HashSet::new();

            worklist.push((assessment, virtual_register));
            seen.insert(block_id);

            while let Some((current_assessment, current_virtual_register)) = worklist.pop() {
                let origin = current_assessment.origin();
                assert!(origin.predecessor_count() > 1 || !origin.phis().is_empty());

                let mut phi: Option<&PhiInstruction> = None;
                for candidate in origin.phis() {
                    if candidate.virtual_register() == current_virtual_register {
                        phi = Some(candidate);
                        break;
                    }
                }

                let mut op_index = 0;
                for pred in origin.predecessors() {
                    let expected = match &phi {
                        Some(phi) => phi.operands()[op_index],
                        None => current_virtual_register,
                    };

                    op_index += 1;
                    let pred_assignment = self.assessments_.get(&pred);

                    match pred_assignment {
                        Some(pred_assessments) => {
                            let found_contribution = pred_assessments.map().get(&current_assessment.operand());
                            match found_contribution {
                                Some(contribution) => {
                                    match contribution.kind() {
                                        AssessmentKind::Final => {
                                            let final_assessment = contribution.as_final().unwrap();
                                            assert_eq!(final_assessment.virtual_register(), expected);
                                        }
                                        AssessmentKind::Pending => {
                                            let next = contribution.as_pending().unwrap();
                                            if seen.insert(pred) {
                                                worklist.push((next, expected));
                                            }
                                        }
                                    }
                                }
                                None => unreachable!(),
                            }
                        }
                        None => {
                            assert!(origin.is_loop_header());
                            let todo_iter = self.outstanding_assessments_.get(&pred);
                            match todo_iter {
                                Some(todo) => {
                                    todo.add_delayed_assessment(current_assessment.operand().clone(), expected);
                                }
                                None => {
                                    self.outstanding_assessments_.insert(pred, DelayedAssessments::new(self.zone_));
                                    self.outstanding_assessments_.get(&pred).unwrap().add_delayed_assessment(current_assessment.operand().clone(), expected);
                                }
                            }
                        }
                    }
                }
            }
            assessment.add_alias(virtual_register);
        }

        fn validate_use(
            &self,
            block_id: RpoNumber,
            current_assessments: &BlockAssessments,
            op: InstructionOperand,
            virtual_register: i32,
        ) {
            let iterator = current_assessments.map().get(&op);
            assert!(iterator.is_some());
            let assessment