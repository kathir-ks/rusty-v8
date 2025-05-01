// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/compiler/backend/move-optimizer.h (module definition)
mod move_optimizer {
    use std::cmp::Ordering;
    use std::collections::HashMap;
    use std::mem;

    use crate::codegen::register_configuration::RegisterConfiguration;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct InstructionOperand {
        kind: InstructionOperandKind,
    }

    impl InstructionOperand {
        fn compare(&self, other: &Self) -> Ordering {
            // Placeholder implementation
            Ordering::Equal
        }

        fn equals_canonicalized(&self, other: &Self) -> bool {
            // Placeholder implementation
            self == other
        }

        fn is_fp_register(&self) -> bool {
            match self.kind {
                InstructionOperandKind::Location(LocationOperand { kind: LocationOperandKind::Register, representation: MachineRepresentation::Float32, .. }) |
                InstructionOperandKind::Location(LocationOperand { kind: LocationOperandKind::Register, representation: MachineRepresentation::Float64, .. }) |
                InstructionOperandKind::Location(LocationOperand { kind: LocationOperandKind::Register, representation: MachineRepresentation::Simd128, .. }) => true,
                _ => false
            }
        }

        fn is_constant(&self) -> bool {
            match self.kind {
                InstructionOperandKind::Constant(_) => true,
                _ => false
            }
        }

        fn is_immediate(&self) -> bool {
            match self.kind {
                InstructionOperandKind::Immediate(_) => true,
                _ => false
            }
        }

        fn is_stack_slot(&self) -> bool {
            match self.kind {
                InstructionOperandKind::Location(LocationOperand { kind: LocationOperandKind::StackSlot, .. }) => true,
                _ => false,
            }
        }

        fn is_fp_stack_slot(&self) -> bool {
            match self.kind {
                InstructionOperandKind::Location(LocationOperand { kind: LocationOperandKind::FPStackSlot, .. }) => true,
                _ => false,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    enum InstructionOperandKind {
        Invalid,
        Constant(i64),
        Immediate(i64),
        // Represent register/stack slot locations
        Location(LocationOperand),
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct LocationOperand {
        kind: LocationOperandKind,
        representation: MachineRepresentation,
        register_code: i32, // Consider wrapping this with a RegisterCode enum
    }

    impl LocationOperand {
        fn cast(op: &InstructionOperand) -> &LocationOperand {
            match &op.kind {
                InstructionOperandKind::Location(loc) => loc,
                _ => panic!("Expected LocationOperand, found {:?}", op),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    enum LocationOperandKind {
        Register,
        StackSlot,
        FPStackSlot,
        // Add other location operand kinds as necessary
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum MachineRepresentation {
        None,
         ওয়ার্ড8,
        Word16,
        Word32,
        Word64,
        Float32,
        Float64,
        Simd128,
        Tagged,
        TaggedSigned,
        TaggedPointer,
        TaggedSmallSmi,
    }

    // Constants for GapPositions
    const FIRST_GAP_POSITION: usize = 0;
    const LAST_GAP_POSITION: usize = 1;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct ParallelMove {
        moves: Vec<MoveOperands>,
    }

    impl ParallelMove {
        fn new(zone: &Zone) -> Self {
            ParallelMove { moves: Vec::new() }
        }

        fn add_move(&mut self, source: InstructionOperand, destination: InstructionOperand) {
            let move_operands = MoveOperands::new(source, destination);
            self.moves.push(move_operands);
        }

        fn prepare_insert_after(&self, move_operands: &MoveOperands, eliminated: &mut Vec<&MoveOperands>) {
            // Placeholder implementation.  Needs to modify moves if necessary, and
            // potentially add to eliminated.
        }

        fn is_empty(&self) -> bool {
            self.moves.is_empty()
        }

        fn clear(&mut self) {
            self.moves.clear();
        }

        fn iter(&self) -> std::slice::Iter<MoveOperands> {
            self.moves.iter()
        }
    }

    impl<'a> IntoIterator for &'a ParallelMove {
        type Item = &'a MoveOperands;
        type IntoIter = std::slice::Iter<'a, MoveOperands>;

        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }

    type MoveOpVector = Vec<MoveOperands>;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct MoveOperands {
        source: InstructionOperand,
        destination: InstructionOperand,
        redundant: bool, // Added for tracking elimination
    }

    impl MoveOperands {
        fn new(source: InstructionOperand, destination: InstructionOperand) -> Self {
            MoveOperands {
                source,
                destination,
                redundant: false,
            }
        }

        fn is_redundant(&self) -> bool {
            self.redundant
        }

        fn eliminate(&mut self) {
            self.redundant = true;
        }

        fn source(&self) -> &InstructionOperand {
            &self.source
        }

        fn destination(&self) -> &InstructionOperand {
            &self.destination
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct RpoNumber(usize);

    #[derive(Debug, Clone)]
    struct InstructionBlock {
        id: RpoNumber,
        predecessors: Vec<RpoNumber>,
        successors: Vec<RpoNumber>,
        first_instruction_index: usize,
        last_instruction_index: usize,
        deferred: bool,
    }

    impl InstructionBlock {
        fn predecessor_count(&self) -> usize {
            self.predecessors.len()
        }
        fn successor_count(&self) -> usize {
            self.successors.len()
        }
        fn is_deferred(&self) -> bool {
            self.deferred
        }
    }

    #[derive(Debug, Clone)]
    struct Instruction {
        parallel_moves: [Option<ParallelMove>; 2],
        inputs: Vec<InstructionOperand>,
        outputs: Vec<InstructionOperand>,
        temps: Vec<InstructionOperand>,
        opcode: InstructionOpcode,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum InstructionOpcode {
        Nop,
        Call,
        Ret,
        TailCall,
        Load,
        Store,
        Other, // Add more opcodes as needed
    }

    impl Instruction {
        fn new(zone: &Zone) -> Self {
            Instruction {
                parallel_moves: [None, None],
                inputs: Vec::new(),
                outputs: Vec::new(),
                temps: Vec::new(),
                opcode: InstructionOpcode::Nop, // Or some default
            }
        }

        fn parallel_moves(&mut self) -> &mut [Option<ParallelMove>; 2] {
            &mut self.parallel_moves
        }

        fn input_count(&self) -> usize {
            self.inputs.len()
        }

        fn output_count(&self) -> usize {
            self.outputs.len()
        }

        fn temp_count(&self) -> usize {
            self.temps.len()
        }

        fn input_at(&self, index: usize) -> &InstructionOperand {
            &self.inputs[index]
        }

        fn output_at(&self, index: usize) -> &InstructionOperand {
            &self.outputs[index]
        }

        fn temp_at(&self, index: usize) -> &InstructionOperand {
            &self.temps[index]
        }

        fn is_call(&self) -> bool {
            self.opcode == InstructionOpcode::Call
        }

        fn is_ret(&self) -> bool {
            self.opcode == InstructionOpcode::Ret
        }

        fn is_tail_call(&self) -> bool {
            self.opcode == InstructionOpcode::TailCall
        }

        fn get_or_create_parallel_move(
            &mut self,
            position: usize,
            zone: &Zone,
        ) -> &mut ParallelMove {
            if self.parallel_moves[position].is_none() {
                self.parallel_moves[position] = Some(ParallelMove::new(zone));
            }
            self.parallel_moves[position].as_mut().unwrap()
        }
    }

    #[derive(Debug, Clone)]
    struct InstructionSequence {
        instructions: Vec<Instruction>,
        instruction_blocks: Vec<InstructionBlock>,
    }

    impl InstructionSequence {
        fn new() -> Self {
            InstructionSequence {
                instructions: Vec::new(),
                instruction_blocks: Vec::new(),
            }
        }

        fn instructions(&mut self) -> &mut Vec<Instruction> {
            &mut self.instructions
        }

        fn instruction_blocks(&mut self) -> &mut Vec<InstructionBlock> {
            &mut self.instruction_blocks
        }

        fn instruction_block_at(&self, id: RpoNumber) -> &InstructionBlock {
            &self.instruction_blocks[id.0] // Assuming RpoNumber is an index
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct MoveKey {
        source: InstructionOperand,
        destination: InstructionOperand,
    }

    impl MoveKey {
        fn new(source: InstructionOperand, destination: InstructionOperand) -> Self {
            MoveKey { source, destination }
        }
    }

    impl std::cmp::PartialOrd for MoveKey {
        fn partial_cmp(&self, other: Option<&Self>) -> Option<Ordering> {
            match other {
                Some(other) => {
                    if self.source != other.source {
                        return Some(self.source.compare(&other.source));
                    }
                    Some(self.destination.compare(&other.destination))
                }
                None => Some(Ordering::Greater),
            }
        }

        fn le(&self, other: &Self) -> bool {
            if self.source != other.source {
                return self.source.compare(&other.source) == Ordering::Less;
            }
            self.destination.compare(&other.destination) == Ordering::Less
        }

        fn ge(&self, other: &Self) -> bool {
            if self.source != other.source {
                return self.source.compare(&other.source) == Ordering::Greater;
            }
            self.destination.compare(&other.destination) == Ordering::Greater
        }

        fn gt(&self, other: &Self) -> bool {
            if self.source != other.source {
                return self.source.compare(&other.source) == Ordering::Greater;
            }
            self.destination.compare(&other.destination) == Ordering::Greater
        }

        fn lt(&self, other: &Self) -> bool {
            if self.source != other.source {
                return self.source.compare(&other.source) == Ordering::Less;
            }
            self.destination.compare(&other.destination) == Ordering::Less
        }
    }

    //enum class AliasingKind { kCombine, kIgnore };
    #[derive(PartialEq, Eq)]
    enum AliasingKind {
        kCombine,
        kIgnore,
    }

    const K_FP_ALIASING: AliasingKind = AliasingKind::kCombine;

    struct OperandSet {
        set: Vec<InstructionOperand>,
        fp_reps: i32,
    }

    impl OperandSet {
        fn new(buffer: &mut Vec<InstructionOperand>) -> Self {
            buffer.clear();
            OperandSet {
                set: mem::take(buffer),
                fp_reps: 0,
            }
        }

        fn insert_op(&mut self, op: &InstructionOperand) {
            self.set.push(op.clone());

            if K_FP_ALIASING == AliasingKind::kCombine && op.is_fp_register() {
                let loc = LocationOperand::cast(op);
                self.fp_reps |= Self::representation_bit(loc.representation);
            }
        }

        fn contains(&self, op: &InstructionOperand) -> bool {
            for elem in &self.set {
                if elem.equals_canonicalized(op) {
                    return true;
                }
            }
            false
        }

        fn contains_op_or_alias(&self, op: &InstructionOperand) -> bool {
            if self.contains(op) {
                return true;
            }

            if K_FP_ALIASING == AliasingKind::kCombine && op.is_fp_register() {
                // Platforms where FP registers have complex aliasing need extra checks.
                let loc = LocationOperand::cast(op);
                let rep = loc.representation;
                // If haven't encountered mixed rep FP registers, skip the extra checks.
                if !Self::has_mixed_fp_reps(self.fp_reps | Self::representation_bit(rep)) {
                    return false;
                }

                // Check register against aliasing registers of other FP representations.
                let (other_rep1, other_rep2) = match rep {
                    MachineRepresentation::Float32 => (
                        MachineRepresentation::Float64,
                        MachineRepresentation::Simd128,
                    ),
                    MachineRepresentation::Float64 => (
                        MachineRepresentation::Float32,
                        MachineRepresentation::Simd128,
                    ),
                    MachineRepresentation::Simd128 => (
                        MachineRepresentation::Float32,
                        MachineRepresentation::Float64,
                    ),
                    _ => panic!("UNREACHABLE"),
                };
                let config = RegisterConfiguration::default();
                let mut base = -1;
                let mut aliases = config.get_aliases(
                    rep,
                    loc.register_code,
                    other_rep1,
                    &mut base,
                );
                assert!(aliases > 0 || (aliases == 0 && base == -1));
                while aliases > 0 {
                    if self.contains(&Self::allocated_operand(
                        LocationOperandKind::Register,
                        other_rep1,
                        base + aliases,
                    )) {
                        return true;
                    }
                    aliases -= 1;
                }
                aliases = config.get_aliases(rep, loc.register_code, other_rep2, &mut base);
                assert!(aliases > 0 || (aliases == 0 && base == -1));
                while aliases > 0 {
                    if self.contains(&Self::allocated_operand(
                        LocationOperandKind::Register,
                        other_rep2,
                        base + aliases,
                    )) {
                        return true;
                    }
                    aliases -= 1;
                }
            }
            false
        }

        fn allocated_operand(kind: LocationOperandKind, representation: MachineRepresentation, register_code: i32) -> InstructionOperand {
            InstructionOperand {
                kind: InstructionOperandKind::Location(LocationOperand {
                    kind,
                    representation,
                    register_code,
                }),
            }
        }

        fn representation_bit(rep: MachineRepresentation) -> i32 {
            match rep {
                MachineRepresentation::Float32 => 1,
                MachineRepresentation::Float64 => 2,
                MachineRepresentation::Simd128 => 4,
                _ => 0, // Or handle other cases as needed
            }
        }

        fn has_mixed_fp_reps(reps: i32) -> bool {
            reps != 0 && !reps.is_power_of_two()
        }
    }

    struct Zone {
        // A simple zone allocator; can be replaced with a more sophisticated one.
    }

    impl Zone {
        fn new() -> Self {
            Zone {}
        }
    }

    struct MoveOptimizer<'a> {
        local_zone_: &'a Zone,
        code_: &'a mut InstructionSequence,
        local_vector_: Vec<MoveOperands>,
        operand_buffer1: Vec<InstructionOperand>,
        operand_buffer2: Vec<InstructionOperand>,
    }

    impl<'a> MoveOptimizer<'a> {
        fn new(local_zone_: &'a Zone, code_: &'a mut InstructionSequence) -> Self {
            MoveOptimizer {
                local_zone_: local_zone_,
                code_: code_,
                local_vector_: Vec::new(),
                operand_buffer1: Vec::new(),
                operand_buffer2: Vec::new(),
            }
        }

        fn run(&mut self) {
            for instruction in self.code_.instructions().iter_mut() {
                self.compress_gaps(instruction);
            }
            for block in self.code_.instruction_blocks().iter_mut() {
                self.compress_block(block);
            }
            for block in self.code_.instruction_blocks().iter_mut() {
                if block.predecessor_count() <= 1 {
                    continue;
                }
                if !block.is_deferred() {
                    let mut has_only_deferred = true;
                    for pred_id in block.predecessors.iter() {
                        if !self.code_.instruction_block_at(*pred_id).is_deferred() {
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
            for gap in self.code_.instructions().iter_mut() {
                self.finalize_moves(gap);
            }
        }

        fn remove_clobbered_destinations(&mut self, instruction: &mut Instruction) {
            if instruction.is_call() {
                return;
            }
            let moves = instruction.parallel_moves[0].as_mut();
            if moves.is_none() {
                return;
            }
            let moves = moves.unwrap();

            assert!(
                instruction.parallel_moves[1].is_none()
                    || instruction.parallel_moves[1].as_ref().unwrap().is_empty()
            );

            let mut outputs = OperandSet::new(&mut self.operand_buffer1);
            let mut inputs = OperandSet::new(&mut self.operand_buffer2);

            // Outputs and temps are treated together as potentially clobbering a
            // destination operand.
            for i in 0..instruction.output_count() {
                outputs.insert_op(instruction.output_at(i));
            }
            for i in 0..instruction.temp_count() {
                outputs.insert_op(instruction.temp_at(i));
            }

            // Input operands block elisions.
            for i in 0..instruction.input_count() {
                inputs.insert_op(instruction.input_at(i));
            }

            // Elide moves made redundant by the instruction.
            for move_op in moves.iter_mut() {
                if outputs.contains_op_or_alias(move_op.destination()) && !inputs.contains_op_or_alias(move_op.destination())
                {
                    move_op.eliminate();
                }
            }

            // The ret instruction makes any assignment before it unnecessary, except for
            // the one for its input.
            if instruction.is_ret() || instruction.is_tail_call() {
                for move_op in moves.iter_mut() {
                    if !inputs.contains_op_or_alias(move_op.destination()) {
                        move_op.eliminate();
                    }
                }
            }
        }

        fn migrate_moves(&mut self, to: &mut Instruction, from: &mut Instruction) {
            if from.is_call() {
                return;
            }

            let from_moves = from.parallel_moves[0].as_mut();
            if from_moves.is_none() || from_moves.as_ref().unwrap().is_empty() {
                return;
            }
            let from_moves = from_moves.unwrap();

            let mut dst_cant_be = OperandSet::new(&mut self.operand_buffer1);
            let mut src_cant_be = OperandSet::new(&mut self.operand_buffer2);

            // If an operand is an input to the instruction, we cannot move assignments
            // where it appears on the LHS.
            for i in 0..from.input_count() {
                dst_cant_be.insert_op(from.input_at(i));
            }
            // If an operand is output to the instruction, we cannot move assignments
            // where it appears on the RHS, because we would lose its value before the
            // instruction.
            // Same for temp operands.
            // The output can't appear on the LHS because we performed
            // RemoveClobberedDestinations for the "from" instruction.
            for i in 0..from.output_count() {
                src_cant_be.insert_op(from.output_at(i));
            }
            for i in 0..from.temp_count() {
                src_cant_be.insert_op(from.temp_at(i));
            }
            for move_op in from_moves.iter() {
                if move_op.is_redundant() {
                    continue;
                }
                // Assume dest has a value "V". If we have a "dest = y" move, then we can't
                // move "z = dest", because z would become y rather than "V".
                // We assume CompressMoves has happened before this, which means we don't
                // have more than one assignment to dest.
                src_cant_be.insert_op(move_op.destination());
            }

            // SmallZoneMap equivalent in Rust
            let mut move_candidates: HashMap<MoveKey, ()> = HashMap::new();

            // We start with all the moves that don't have conflicting source or
            // destination operands are eligible for being moved down.
            for move_op in from_moves.iter() {
                if move_op.is_redundant() {
                    continue;
                }
                if !dst_cant_be.contains_op_or_alias(move_op.destination()) {
                    let key = MoveKey::new(move_op.source().clone(), move_op.destination().clone());
                    move_candidates.insert(key, ());
                }
            }
            if move_candidates.is_empty() {
                return;
            }

            // Stabilize the candidate set.
            let mut changed = true;
            while changed {
                changed = false;
                let mut to_remove = Vec::new();
                for (move_key, _) in move_candidates.iter() {
                    if src_cant_be.contains_op_or_alias(&move_key.source) {
                        src_cant_be.insert_op(move_key.destination.clone());
                        to_remove.push(move_key.clone());
                        changed = true;
                    }
                }
                for key in to_remove {
                    move_candidates.remove(&key);
                }
            }

            let mut to_move = ParallelMove::new(self.local_zone_);
            for move_op in from_moves.iter_mut() {
                if move_op.is_redundant() {
                    continue;
                }
                let key = MoveKey::new(move_op.source().clone(), move_op.destination().clone());
                if move_candidates.contains_key(&key) {
                    to_move.add_move(move_op.source().clone(), move_op.destination().clone());
                    move_op.eliminate();
                }
            }
            if to_move.is_empty() {
                return;
            }

            let dest = to.get_or_create_parallel_move(FIRST_GAP_POSITION, self.local_zone_);

            self.compress_moves(&mut to_move, &mut dest.moves);
            assert!(dest.is_empty());

            dest.moves = to_move.moves; // Move the moves to dest
        }

        fn compress_moves(&mut self, left: &mut ParallelMove, right: &mut MoveOpVector) {
            if right.is_empty() {
                return;
            }

            let eliminated = &mut self.local_vector_;
            eliminated.clear();

            if !left.is_empty() {
                // Modify the right moves in place and collect moves that will be killed by
                // merging the two gaps.
                for move_op in right.iter_mut() {
                    if move_op.is_redundant() {
                        continue;
                    }
                    left.prepare_insert_after(move_op, eliminated);
                }
                // Eliminate dead moves.
                for to_eliminate in eliminated.iter() {
                    let index = right.iter().position(|x| x == to_eliminate).unwrap();
                    right[index].eliminate();
                }
                eliminated.clear();
            }
            // Add all possibly modified moves from right side.
            for move_op in right.iter_mut() {
                if move_op.is_redundant() {
                    continue;
                }
                left.moves.push(move_op.clone());
            }
            // Nuke right.
            right.clear();
            assert!(eliminated.is_empty());
        }

        fn compress_gaps(&mut self, instruction: &mut Instruction) {
            let i = self.find_first_non_empty_slot(instruction);
            let has_moves = i <= LAST_GAP_POSITION;

            if i == LAST_GAP_POSITION {
                instruction.parallel_moves.swap(FIRST_GAP_POSITION, LAST_GAP_POSITION);
            } else if i == FIRST_GAP_POSITION {
                let first = instruction.parallel_moves[FIRST_GAP_POSITION].take();
                let last = instruction.parallel_moves[LAST_GAP_POSITION].take();

                if let Some(mut first_moves) = first {
                    if let Some(mut last_moves) = last {
                        self.compress_moves(&mut first_moves, &mut last_moves.moves);
                    }
                    instruction.parallel_moves[FIRST_GAP_POSITION] = Some(first_moves);
                }
            }

            let first = &instruction.parallel_moves[FIRST_GAP_POSITION];
            let last = &instruction.parallel_moves[LAST_GAP_POSITION];
        }

        fn compress_block(&mut self, block: &mut InstructionBlock) {
            let first_instr_index = block.first_instruction_index;
            let last_instr_index = block.last_instruction_index;

            // Start by removing gap assignments where the output of the subsequent
            // instruction appears on LHS, as long as they are not needed by its input.
            let mut prev_instr = &mut self.code_.instructions[first_instr_index];
            self.remove_clobbered_destinations(prev_instr);

            for index in (first_instr_index + 1)..=last_instr_index {
                let instr = &mut self.code_.instructions[index];
                // Migrate to the gap of prev_instr eligible moves from instr.
                self.migrate_moves(instr, prev_instr);
                // Remove gap assignments clobbered by instr's output.
                self.remove_clobbered_destinations(instr);
                prev_instr = instr;
            }
        }

        fn last_instruction(&self, block: &InstructionBlock) -> &Instruction {
            &self.code_.instructions[block.last_instruction_index]
        }

        fn optimize_merge(&mut self, block: &mut InstructionBlock) {
            assert!(block.predecessor_count() > 1);

            // Ensure that the last instruction in all incoming blocks don't contain
            // things that would prevent moving gap moves across them.
            for pred_index in block.predecessors.iter() {
                let pred = self.code_.instruction_block_at(*pred_index);

                // If the predecessor has more than one successor, we shouldn't attempt to
                // move down to this block (one of the successors) any of the gap moves,
                // because their effect may be necessary to the other successors.
                if pred.successor_count() > 1 {
                    return;
                }

                let last_instr = &self.code_.instructions[pred.last_instruction_index];
                if last_instr.is_call() {
                    return;
                }
                if last_instr.temp_count() != 0 {
                    return;
                }
                if last_instr.output_count() != 0 {
                    return;
                }
                for i in 0..last_instr.input_count() {
                    let op = last_instr.input_at(i);
                    if !op.is_constant() && !op.is_immediate() {
                        return;
                    }
                }
            }

            // SmallZoneMap equivalent in Rust
            let mut move_map: HashMap<MoveKey, usize> = HashMap::new();
            let mut correct_counts = 0;

            // Accumulate set of shared moves.
            for pred_index in block.predecessors.iter() {
                let pred = self.code_.instruction_block_at(*pred_index);
                let instr = self.last_instruction(pred);
                let parallel_moves = &instr.parallel_moves[0];

                if parallel_moves.is_none() || parallel_moves.as_ref().unwrap().is_empty() {
                    return;
                }

                let parallel_moves = parallel_moves.as_ref().unwrap();

                for move_op in parallel_moves.iter() {
                    if move_op.is_redundant() {
                        continue;
                    }
                    let src = move_op.source().clone();
                    let dst = move_op.destination().clone();
                    let key = MoveKey::new(src, dst);
                    let count = move_map.entry(key).or_insert(0);
                    *count += 1;
                    if *count == block.predecessor_count() {
                        correct_counts += 1;
                    }
                }
            }

            if move_map.is_empty() || correct_counts == 0 {
                return;
            }

            // Find insertion point.
            let instr = &mut self.code_.instructions[block.first_instruction_index];

            if correct_counts != move_map.len() {
                // Moves that are unique to each predecessor won't be pushed to the common
                // successor.
                let mut conflicting_srcs = OperandSet::new(&mut self.operand_buffer1);
                let mut to_remove = Vec::new();
                for (move_key, count) in move_map.iter() {
                    if *count != block.predecessor_count() {
                        // Not all the moves in all the gaps are the same. Maybe some are. If
                        // there are such moves, we could move them, but the destination of the
                        // moves staying behind can't appear as a source of a common move,
                        // because the move staying behind will clobber this destination.
                        conflicting_srcs.insert_op(&move_key.destination);
                        to_remove.push(move_key.clone());
                    }
                }
                for key in to_remove {
                    move_map.remove(&key);
                }

                let mut changed = true;
                while changed {
                    // If a common move can't be pushed to the common successor, then its
                    // destination also can't appear as source to any move being pushed.
                    changed = false;
                    let mut to_remove = Vec::new();
                    for (move_key, count) in move_map.iter() {
                        assert_eq!(*count, block.predecessor_count());
                        if conflicting_srcs.contains_op_or_alias(&move_key.source) {
                            conflicting_srcs.insert_op(&move_key.destination);
                            to_remove.push(move_key.clone());
                            changed = true;
                        }
                    }
                    for key in to_remove {
                        move_map.remove(&key);
                    }
                }
            }

            if move_map.is_empty() {
                return;
            }

            let mut gap_initialized = true;
            if instr.parallel_moves[0].is_some() && !instr.parallel_moves[0].as_ref().unwrap().is_empty() {
                // Will compress after insertion.
                gap_initialized = false;
                instr.parallel_moves.swap(0, 1);
            }

            let moves = instr.get_or_create_parallel_move(FIRST_GAP_POSITION, self.local_zone_);
            // Delete relevant entries in predecessors and move everything to block.
            let mut first_iteration = true;
            for pred_index in block.predecessors.iter() {
                let pred = self.code_.instruction_block_at(*pred_index);
                let last_instr = self.last_instruction(pred);
                let parallel_moves = &mut last_instr.parallel_moves[0].as_mut().unwrap().moves;

                for move_op in parallel_moves.iter_mut() {
                    if move_op.is_redundant() {
                        continue;
                    }
                    let key = MoveKey::new(move_op.source().clone(), move_op.destination().clone());
                    if move_map.contains_key(&key) {
                        if first_iteration {
                            moves.add_move(move_op.source().clone(), move_op.destination().clone());
                        