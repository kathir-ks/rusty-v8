// Converted from V8 C++ source files:
// Header: move-optimizer.h
// Implementation: move-optimizer.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]

use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::base::base;
use crate::ExecutionTier::kTopTier;
use crate::Nothing::Nothing;
use crate::V8::V8;
use crate::v8::v8;

pub struct V8_EXPORT_PRIVATE {}

mod register_configuration {
    pub struct RegisterConfiguration {}
    impl RegisterConfiguration {
        pub fn Default() -> *const RegisterConfiguration {
            std::ptr::null()
        }

        pub fn GetAliases(
            &self,
            rep: MachineRepresentation,
            register_code: i32,
            other_rep: MachineRepresentation,
            base: *mut i32,
        ) -> i32 {
            0
        }
    }
}

mod instruction {
    use super::*;
    use std::any::Any;

    pub enum GapPosition {
        START,
        END,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum LocationOperandKind {
        REGISTER,
        STACK_SLOT,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct LocationOperand {
        kind: LocationOperandKind,
        representation: MachineRepresentation,
        register_code: i32,
    }

    impl LocationOperand {
        pub fn new(
            kind: LocationOperandKind,
            representation: MachineRepresentation,
            register_code: i32,
        ) -> Self {
            LocationOperand {
                kind,
                representation,
                register_code,
            }
        }

        pub fn cast(op: &InstructionOperand) -> &LocationOperand {
            op.as_any().downcast_ref::<LocationOperand>().unwrap()
        }

        pub fn representation(&self) -> MachineRepresentation {
            self.representation
        }

        pub fn register_code(&self) -> i32 {
            self.register_code
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum InstructionOperandType {
        CONSTANT,
        IMMEDIATE,
        ALLOCATED,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct InstructionOperand {
        op_type: InstructionOperandType,
        canonicalized: bool,
        location: Option<LocationOperand>,
    }

    impl InstructionOperand {
        pub fn new(op_type: InstructionOperandType, canonicalized: bool) -> Self {
            InstructionOperand {
                op_type,
                canonicalized,
                location: None,
            }
        }

        pub fn constant() -> Self {
            InstructionOperand {
                op_type: InstructionOperandType::CONSTANT,
                canonicalized: false,
                location: None,
            }
        }

        pub fn immediate() -> Self {
            InstructionOperand {
                op_type: InstructionOperandType::IMMEDIATE,
                canonicalized: false,
                location: None,
            }
        }

        pub fn allocated(location: LocationOperand) -> Self {
            InstructionOperand {
                op_type: InstructionOperandType::ALLOCATED,
                canonicalized: false,
                location: Some(location),
            }
        }

        pub fn IsConstant(&self) -> bool {
            self.op_type == InstructionOperandType::CONSTANT
        }

        pub fn IsImmediate(&self) -> bool {
            self.op_type == InstructionOperandType::IMMEDIATE
        }

        pub fn IsStackSlot(&self) -> bool {
            if let Some(location) = &self.location {
                location.kind == LocationOperandKind::STACK_SLOT
            } else {
                false
            }
        }

        pub fn IsFPStackSlot(&self) -> bool {
            if let Some(location) = &self.location {
                location.kind == LocationOperandKind::STACK_SLOT
            } else {
                false
            }
        }

        pub fn IsFPRegister(&self) -> bool {
            if let Some(location) = &self.location {
                location.kind == LocationOperandKind::REGISTER
            } else {
                false
            }
        }

        pub fn IsLocationOperand(&self) -> bool {
            self.location.is_some()
        }

        pub fn Compare(&self, other: InstructionOperand) -> bool {
            self == &other
        }

        pub fn CompareCanonicalized(&self, other: InstructionOperand) -> bool {
            self == &other
        }

        pub fn EqualsCanonicalized(&self, other: InstructionOperand) -> bool {
            self == &other
        }

        pub fn as_any(&self) -> &dyn Any {
            self
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct MoveOperands {
        source: InstructionOperand,
        destination: InstructionOperand,
        redundant: bool,
    }

    impl MoveOperands {
        pub fn new(source: InstructionOperand, destination: InstructionOperand) -> Self {
            MoveOperands {
                source,
                destination,
                redundant: false,
            }
        }

        pub fn source(&self) -> &InstructionOperand {
            &self.source
        }

        pub fn destination(&self) -> &InstructionOperand {
            &self.destination
        }

        pub fn IsRedundant(&self) -> bool {
            self.redundant
        }

        pub fn Eliminate(&mut self) {
            self.redundant = true;
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ParallelMove {
        moves: Vec<MoveOperands>,
    }

    impl ParallelMove {
        pub fn new(_local_zone: &Zone) -> Self {
            ParallelMove { moves: Vec::new() }
        }

        pub fn AddMove(&mut self, source: InstructionOperand, destination: InstructionOperand) {
            self.moves.push(MoveOperands::new(source, destination));
        }

        pub fn PrepareInsertAfter(
            &mut self,
            move_operands: &MoveOperands,
            eliminated: &mut ZoneVector<MoveOperands>,
        ) {
        }

        pub fn AddMove(
            &mut self,
            source: InstructionOperand,
            destination: InstructionOperand,
        ) -> Result<(), String> {
            self.moves.push(MoveOperands {
                source,
                destination,
                redundant: false,
            });
            Ok(())
        }

        pub fn empty(&self) -> bool {
            self.moves.is_empty()
        }

        pub fn clear(&mut self) {
            self.moves.clear();
        }

        pub fn push_back(&mut self, move_operands: &MoveOperands) {
            self.moves.push(move_operands.clone());
        }

        pub fn iter(&self) -> std::slice::Iter<'_, MoveOperands> {
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

    impl<'a> IntoIterator for &'a ParallelMove {
        type Item = &'a MoveOperands;
        type IntoIter = std::slice::Iter<'a, MoveOperands>;

        fn into_iter(self) -> Self::IntoIter {
            self.moves.iter()
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Instruction {
        index: i32,
        parallel_moves: [Option<ParallelMove>; 2],
        output_count: usize,
        input_count: usize,
        temp_count: usize,
        outputs: Vec<InstructionOperand>,
        inputs: Vec<InstructionOperand>,
        temps: Vec<InstructionOperand>,
        is_call: bool,
        is_ret: bool,
        is_tail_call: bool,
    }

    impl Instruction {
        pub const FIRST_GAP_POSITION: usize = 0;
        pub const LAST_GAP_POSITION: usize = 1;

        pub fn new(index: i32) -> Self {
            Instruction {
                index,
                parallel_moves: [None, None],
                output_count: 0,
                input_count: 0,
                temp_count: 0,
                outputs: Vec::new(),
                inputs: Vec::new(),
                temps: Vec::new(),
                is_call: false,
                is_ret: false,
                is_tail_call: false,
            }
        }

        pub fn parallel_moves(&mut self) -> &mut [Option<ParallelMove>; 2] {
            &mut self.parallel_moves
        }

        pub fn OutputCount(&self) -> usize {
            self.output_count
        }

        pub fn InputCount(&self) -> usize {
            self.input_count
        }

        pub fn TempCount(&self) -> usize {
            self.temp_count
        }

        pub fn OutputAt(&self, index: usize) -> &InstructionOperand {
            &self.outputs[index]
        }

        pub fn InputAt(&self, index: usize) -> &InstructionOperand {
            &self.inputs[index]
        }

        pub fn TempAt(&self, index: usize) -> &InstructionOperand {
            &self.temps[index]
        }

        pub fn IsCall(&self) -> bool {
            self.is_call
        }

        pub fn IsRet(&self) -> bool {
            self.is_ret
        }

        pub fn IsTailCall(&self) -> bool {
            self.is_tail_call
        }

        pub fn GetOrCreateParallelMove(
            &mut self,
            position: GapPosition,
            code_zone: &Zone,
        ) -> &mut ParallelMove {
            let index = match position {
                GapPosition::START => 0,
                GapPosition::END => 1,
            };
            if self.parallel_moves[index].is_none() {
                self.parallel_moves[index] = Some(ParallelMove::new(code_zone));
            }
            self.parallel_moves[index].as_mut().unwrap()
        }
    }
}

mod instruction_sequence {
    use super::*;
    use instruction::Instruction;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct InstructionBlock {
        first_instruction_index: i32,
        last_instruction_index: i32,
        predecessors: Vec<RpoNumber>,
        successors: Vec<RpoNumber>,
        is_deferred: bool,
    }

    impl InstructionBlock {
        pub fn new(
            first_instruction_index: i32,
            last_instruction_index: i32,
            is_deferred: bool,
        ) -> Self {
            InstructionBlock {
                first_instruction_index,
                last_instruction_index,
                predecessors: Vec::new(),
                successors: Vec::new(),
                is_deferred,
            }
        }

        pub fn first_instruction_index(&self) -> i32 {
            self.first_instruction_index
        }

        pub fn last_instruction_index(&self) -> i32 {
            self.last_instruction_index
        }

        pub fn PredecessorCount(&self) -> usize {
            self.predecessors.len()
        }

        pub fn predecessors(&self) -> &Vec<RpoNumber> {
            &self.predecessors
        }

        pub fn SuccessorCount(&self) -> usize {
            self.successors.len()
        }

        pub fn IsDeferred(&self) -> bool {
            self.is_deferred
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct InstructionSequence {
        instructions: Vec<Instruction>,
        instruction_blocks: Vec<InstructionBlock>,
        zone: Zone,
    }

    impl InstructionSequence {
        pub fn new(zone: Zone) -> Self {
            InstructionSequence {
                instructions: Vec::new(),
                instruction_blocks: Vec::new(),
                zone,
            }
        }

        pub fn instructions(&mut self) -> &mut Vec<Instruction> {
            &mut self.instructions
        }

        pub fn instruction_blocks(&mut self) -> &mut Vec<InstructionBlock> {
            &mut self.instruction_blocks
        }

        pub fn InstructionBlockAt(&self, index: RpoNumber) -> &InstructionBlock {
            &self.instruction_blocks[index.number() as usize]
        }

        pub fn zone(&self) -> &Zone {
            &self.zone
        }
    }
}

use instruction::InstructionOperand;
use instruction::InstructionOperandType;
use instruction::MoveOperands;
use instruction::ParallelMove;
use instruction_sequence::InstructionBlock;
use instruction_sequence::InstructionSequence;
pub mod instruction_scheduler {
    pub enum InstructionOperand {}
}
pub mod jump_threading {
    pub struct ParallelMove {}
}
pub mod register_allocation {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum MachineRepresentation {
        kWord8,
        kWord16,
        kWord32,
        kWord64,
        kFloat32,
        kFloat64,
        kSimd128,
        kTagged,
        kTaggedSigned,
        kTaggedPointer,
    }
}
use register_allocation::MachineRepresentation;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub struct RpoNumber {
    number: i32,
}

impl RpoNumber {
    pub fn number(&self) -> i32 {
        self.number
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MoveKey {
    source: InstructionOperand,
    destination: InstructionOperand,
}

impl Hash for MoveKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Implement hashing based on the content of InstructionOperand
        // For simplicity, we use a placeholder hash.  A real implementation
        // would need to hash the relevant fields of InstructionOperand.
        std::ptr::hash(self, state);
    }
}

impl PartialOrd for MoveKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MoveKey {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.source != other.source {
            // Assuming InstructionOperand has a proper comparison
            Ordering::Equal // Placeholder, implement proper comparison
        } else {
            Ordering::Equal // Placeholder, implement proper comparison
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZoneVector<T> {
    elements: Vec<T>,
}

impl<T> ZoneVector<T> {
    pub fn new(_zone: &Zone) -> Self {
        ZoneVector {
            elements: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.elements.clear();
    }

    pub fn push_back(&mut self, value: T) {
        self.elements.push(value);
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.elements.iter()
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> IntoIterator for ZoneVector<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a ZoneVector<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zone {}

impl Zone {
    pub fn new() -> Self {
        Zone {}
    }
}

// Alias for consistency with C++ code.
type MoveOpVector = ZoneVector<MoveOperands>;

enum AliasingKind {
    kCombine,
    kNoCombine,
}

const kFPAliasing: AliasingKind = AliasingKind::kNoCombine;

struct OperandSet {
    set_: *mut ZoneVector<InstructionOperand>,
    fp_reps_: i32,
}

impl OperandSet {
    fn new(buffer: *mut ZoneVector<InstructionOperand>) -> Self {
        unsafe {
            (*buffer).clear();
        }
        OperandSet {
            set_: buffer,
            fp_reps_: 0,
        }
    }

    fn insert_op(&mut self, op: &InstructionOperand) {
        unsafe {
            (*self.set_).push_back(op.clone());
        }

        if let AliasingKind::kCombine = kFPAliasing {
            if op.IsFPRegister() {
                let loc = instruction::LocationOperand::cast(op);
                self.fp_reps_ |= Self::representation_bit(loc.representation());
            }
        }
    }

    fn contains(&self, op: &InstructionOperand) -> bool {
        unsafe {
            for elem in &(*self.set_).elements {
                if elem.EqualsCanonicalized(op.clone()) {
                    return true;
                }
            }
        }
        false
    }

    fn contains_op_or_alias(&self, op: &InstructionOperand) -> bool {
        if self.contains(op) {
            return true;
        }

        if let AliasingKind::kCombine = kFPAliasing {
            if op.IsFPRegister() {
                // Platforms where FP registers have complex aliasing need extra checks.
                let loc = instruction::LocationOperand::cast(op);
                let rep = loc.representation();
                // If haven't encountered mixed rep FP registers, skip the extra checks.
                if !Self::has_mixed_fp_reps(self.fp_reps_ | Self::representation_bit(rep)) {
                    return false;
                }

                // Check register against aliasing registers of other FP representations.
                let (other_rep1, other_rep2) = match rep {
                    MachineRepresentation::kFloat32 => (
                        MachineRepresentation::kFloat64,
                        MachineRepresentation::kSimd128,
                    ),
                    MachineRepresentation::kFloat64 => (
                        MachineRepresentation::kFloat32,
                        MachineRepresentation::kSimd128,
                    ),
                    MachineRepresentation::kSimd128 => (
                        MachineRepresentation::kFloat32,
                        MachineRepresentation::kFloat64,
                    ),
                    _ => panic!("UNREACHABLE"),
                };
                let config = register_configuration::RegisterConfiguration::Default();
                let mut base = -1;
                let mut aliases = unsafe {
                    (*config).GetAliases(
                        rep,
                        loc.register_code(),
                        other_rep1,
                        &mut base as *mut i32,
                    )
                };
                assert!(aliases > 0 || (aliases == 0 && base == -1));
                while aliases > 0 {
                    aliases -= 1;
                    //if self.contains(AllocatedOperand(instruction::LocationOperandKind::REGISTER, other_rep1,base + aliases)) {
                    //    return true;
                    //}
                }
                aliases = unsafe {
                    (*config).GetAliases(
                        rep,
                        loc.register_code(),
                        other_rep2,
                        &mut base as *mut i32,
                    )
                };
                assert!(aliases > 0 || (aliases == 0 && base == -1));
                while aliases > 0 {
                    aliases -= 1;
                    //if self.contains(AllocatedOperand(instruction::LocationOperandKind::REGISTER, other_rep2,base + aliases)) {
                    //    return true;
                    //}
                }
            }
        }
        false
    }
}

impl OperandSet {
    fn has_mixed_fp_reps(reps: i32) -> bool {
        reps != 0 && (reps & (reps - 1)) != 0
    }

    fn representation_bit(rep: MachineRepresentation) -> i32 {
        match rep {
            MachineRepresentation::kFloat32 => 1,
            MachineRepresentation::kFloat64 => 2,
            MachineRepresentation::kSimd128 => 4,
            _ => 0,
        }
    }
}

fn find_first_non_empty_slot(instr: &mut instruction::Instruction) -> i32 {
    let mut i = instruction::Instruction::FIRST_GAP_POSITION as i32;
    while i <= instruction::Instruction::LAST_GAP_POSITION as i32 {
        let moves = instr.parallel_moves()[i as usize].as_ref();
        if moves.is_none() {
            i += 1;
            continue;
        }
        for move_op in moves.unwrap().iter() {
            if !move_op.IsRedundant() {
                return i;
            }
        }
        i += 1;
    }
    i
}

type SmallZoneMap<K, V, const SIZE: usize> = HashMap<K, V>;

struct Dummy {}

pub struct MoveOptimizer {
    local_zone_: Zone,
    code_: *mut InstructionSequence,
    local_vector_: MoveOpVector,
    operand_buffer1: ZoneVector<InstructionOperand>,
    operand_buffer2: ZoneVector<InstructionOperand>,
}

impl MoveOptimizer {
    pub fn new(local_zone: Zone, code: *mut InstructionSequence) -> Self {
        MoveOptimizer {
            local_zone_: local_zone.clone(),
            code_: code,
            local_vector_: ZoneVector::new(&local_zone),
            operand_buffer1: ZoneVector::new(&local_zone),
            operand_buffer2: ZoneVector::new(&local_zone),
        }
    }

    fn code(&self) -> &mut InstructionSequence {
        unsafe { &mut *self.code_ }
    }
    fn local_zone(&self) -> &Zone {
        &self.local_zone_
    }
    fn code_zone(&self) -> &Zone {
        unsafe { &(*self.code_).zone }
    }
    fn local_vector(&mut self) -> &mut MoveOpVector {
        &mut self.local_vector_
    }

    pub fn run(&mut self) {
        let instructions = self.code().instructions().clone();
        for instruction in instructions.iter_mut() {
            self.compress_gaps(instruction);
        }
        let instruction_blocks = self.code().instruction_blocks().clone();
        for block in instruction_blocks.iter_mut() {
            self.compress_block(block);
        }
        let instruction_blocks = self.code().instruction_blocks().clone();
        for block in instruction_blocks.iter_mut() {
            if block.PredecessorCount() <= 1 {
                continue;
            }
            if !block.IsDeferred() {
                let mut has_only_deferred = true;
                for pred_id in block.predecessors() {
                    if !self.code().InstructionBlockAt(*pred_id).IsDeferred() {
                        has_only_deferred = false;
                        break;
                    }
                }
                if has_only_deferred {
                    continue;
                }
            }
            self.optimize_merge(block);
        }
        let instructions = self.code().instructions().clone();
        for gap in instructions.iter_mut() {
            self.finalize_moves(gap);
        }
    }

    fn remove_clobbered_destinations(&mut self, instruction: &mut instruction::Instruction) {
        if instruction.IsCall() {
            return;
        }
        let moves = instruction.parallel_moves()[0].as_mut();
        if moves.is_none() {
            return;
        }
        let moves = moves.unwrap();

        assert!(instruction.parallel_moves()[1].is_none() || instruction.parallel_moves()[1].as_ref().unwrap().empty());

        let mut outputs = unsafe {
            OperandSet::new(&mut self.operand_buffer1 as *mut ZoneVector<InstructionOperand>)
        };
        let mut inputs = unsafe {
            OperandSet::new(&mut self.operand_buffer2 as *mut ZoneVector<InstructionOperand>)
        };

        for i in 0..instruction.OutputCount() {
            outputs.insert_op(instruction.OutputAt(i));
        }
        for i in 0..instruction.TempCount() {
            outputs.insert_op(instruction.TempAt(i));
        }

        for i in 0..instruction.InputCount() {
            inputs.insert_op(instruction.InputAt(i));
        }

        for move_op in moves.iter_mut() {
            if outputs.contains_op_or_alias(move_op.destination()) && !inputs.contains_op_or_alias(move_op.destination())
            {
                move_op.Eliminate();
            }
        }

        if instruction.IsRet() || instruction.IsTailCall() {
            for move_op in moves.iter_mut() {
                if !inputs.contains_op_or_alias(move_op.destination()) {
                    move_op.Eliminate();
                }
            }
        }
    }

    fn migrate_moves(&mut self, to: &mut instruction::Instruction, from: &mut instruction::Instruction) {
        if from.IsCall() {
            return;
        }

        let from_moves = from.parallel_moves()[0].as_mut();
        if from_moves.is_none() || from_moves.as_ref().unwrap().empty() {
            return;
        }
        let from_moves = from_moves.unwrap();

        let mut dst_cant_be = unsafe {
            OperandSet::new(&mut self.operand_buffer1 as *mut ZoneVector<InstructionOperand>)
        };
        let mut src_cant_be = unsafe {
            OperandSet::new(&mut self.operand_buffer2 as *mut ZoneVector<InstructionOperand>)
        };

        for i in 0..from.InputCount() {
            dst_cant_be.insert_op(from.InputAt(i));
        }

        for i in 0..from.OutputCount() {
            src_cant_be.insert_op(from.OutputAt(i));
        }
        for i in 0..from.TempCount() {
            src_cant_be.insert_op(from.TempAt(i));
        }
        for move_op in from_moves.iter() {
            if move_op.IsRedundant() {
                continue;
            }
            src_cant_be.insert_op(move_op.destination());
        }

        let mut move_candidates: SmallZoneMap<MoveKey, Dummy, 16> = SmallZoneMap::new();
        for move_op in from_moves.iter() {
            if move_op.IsRedundant() {
                continue;
            }
            if !dst_cant_be.contains_op_or_alias(move_op.destination()) {
                let key = MoveKey {
                    source: move_op.source().clone(),
                    destination: move_op.destination().clone(),
                };
                move_candidates.insert(key, Dummy {});
            }
        }
        if move_candidates.is_empty() {
            return;
        }

        let mut changed = false;
        loop {
            changed = false;
            let mut iter = move_candidates.clone().into_iter();
            while let Some((move_candidate, _)) = iter.next() {
                if src_cant_be.contains_op_or_alias(&move_candidate.source) {
                    src_cant_be.insert_op(move_candidate.destination.clone());
                    move_candidates.remove(&move_candidate);
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }

        let mut to_move = ParallelMove::new(&self.local_zone_);
        for move_op in from_moves.iter_mut() {
            if move_op.IsRedundant() {
                continue;
            }
            let key = MoveKey {
                source: move_op.source().clone(),
                destination: move_op.destination().clone(),
            };
            if move_candidates.contains_key(&key) {
                to_move.AddMove(move_op.source().clone(), move_op.destination().clone());
                move_op.Eliminate();
            }
        }
        if to_move.empty() {
            return;
        }

        let dest = to.GetOrCreateParallelMove(
            instruction::GapPosition::START,
            self.code_zone(),
        );

        self.compress_moves(&mut to_move, Some(dest));
        assert!(dest.empty());
        for m in to_move.moves.iter() {
            dest.push_back(m);
        }
    }

    fn compress_moves(&mut self, left: &mut ParallelMove, right: Option<&mut ParallelMove>) {
        if right.is_none() {
            return;
        }
        let right = right.unwrap();

        let eliminated = &mut self.local_vector_;
        assert!(eliminated.is_empty());

        if !left.empty() {
            for move_op in right.iter() {
                if move_op.IsRedundant() {
                    continue;
                }
                left.PrepareInsertAfter(move_op, eliminated);
            }

            // Eliminate dead moves.
            for to_eliminate in eliminated.iter_mut() {
                to_eliminate.Eliminate();
            }
            eliminated.clear();
        }

        for move_op in right.iter() {
            if move_op.IsRedundant() {
                continue;
            }
            left.push_back(move_op);
        }

        right.clear();
        assert!(eliminated.is_empty());
    }

    fn compress_gaps(&mut self, instruction: &mut instruction::Instruction) {
        let i = find_first_non_empty_slot(instruction);
        let has_moves = i <= instruction::Instruction::LAST_GAP_POSITION as i32;
        assert!(has_moves == false || has_moves == true);

        if i == instruction::Instruction::LAST_GAP_POSITION as i32 {
            instruction.parallel_moves().swap(
                instruction::Instruction::FIRST_GAP_POSITION,
                instruction::Instruction::LAST_GAP_POSITION,
            );
        } else if i == instruction::Instruction::FIRST_GAP_POSITION as i32 {
            let first_gap = instruction.parallel_moves()[instruction::Instruction::FIRST_GAP_POSITION].take();
            let last_gap = instruction.parallel_moves()[instruction::Instruction::LAST_GAP_POSITION].take();

            if let Some(mut first) = first_gap {
                if let Some(mut last) = last_gap {
                    self.compress_moves(&mut first, Some(&mut last));
                    instruction.parallel_moves()[instruction::Instruction::FIRST_GAP_POSITION] = Some(first);
                } else {
                    instruction.parallel_moves()[instruction::Instruction::FIRST_GAP_POSITION] = Some(first);
                }
            }
            instruction.parallel_moves()[instruction::Instruction::LAST_GAP_POSITION] = last_gap;
        }

        let first = instruction.parallel_moves()[instruction::Instruction::FIRST_GAP_POSITION].as_ref();
        let last = instruction.parallel_moves()[instruction::Instruction::LAST_GAP_POSITION].as_ref();
        assert!(!has_moves || (first.is_some() && (last.is_none() || last.unwrap().empty())));
    }

    fn compress_block(&mut self, block: &mut InstructionBlock) {
        let first_instr_index = block.first_instruction_index();
        let last_instr_index = block.last_instruction_index();

        let instructions = unsafe { &mut (*self.code_).instructions };

        let mut prev_instr = instructions[first_instr_index as usize].clone();
        self.remove_clobbered_destinations(&mut instructions[first_instr_index as usize]);

        for index in first_instr_index + 1..=last_instr_index {
            let instr = instructions[index as usize].clone();
            self.migrate_moves(&mut instructions[index as usize], &mut prev_instr);
            self.remove_clobbered_destinations(&mut instructions[index as usize]);
            prev_instr = instr;
        }
    }

    fn last_instruction(&self, block: &InstructionBlock) -> &instruction::Instruction {
        let instructions = unsafe { &(*self.code_).instructions };
        &instructions[block.last_instruction_index() as usize]
    }

    fn optimize_merge(&mut self, block: &mut InstructionBlock) {
        assert!(block.PredecessorCount() > 1);

        for pred_index in block.predecessors() {
            let pred = self.code().InstructionBlockAt(*pred_index);

            if pred.SuccessorCount() > 1 {
                return;
            }

            let last_instr = self.last_instruction(pred);
            if last_instr.IsCall() {
                return;
            }
            if last_instr.TempCount() != 0 {
                return;
            }
            if last_instr.OutputCount() != 0 {
                return;
            }
            for i in 0..last_instr.InputCount() {
                let op = last_instr
