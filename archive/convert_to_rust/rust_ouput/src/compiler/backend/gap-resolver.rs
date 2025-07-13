// Converted from V8 C++ source files:
// Header: gap-resolver.h
// Implementation: gap-resolver.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/compiler/backend/gap-resolver.h
pub mod gap_resolver {
    use std::rc::Rc;
    use std::cell::RefCell;

    use crate::compiler::backend::instruction::instruction::InstructionOperand;
    use crate::compiler::backend::instruction_selector::AllocatedOperand;
    use crate::compiler::turboshaft::int64_lowering_reducer::MachineRepresentation;

    pub trait Assembler {
        fn assemble_move(&mut self, source: &InstructionOperand, destination: &InstructionOperand);
        fn assemble_swap(&mut self, source: &InstructionOperand, destination: &InstructionOperand);
        fn push(&mut self, src: &InstructionOperand) -> AllocatedOperand;
        fn pop(&mut self, dest: &InstructionOperand, rep: MachineRepresentation);
        fn pop_temp_stack_slots(&mut self);
        fn move_to_temp_location(&mut self, src: &InstructionOperand, rep: MachineRepresentation);
        fn move_temp_location_to(&mut self, dst: &InstructionOperand, rep: MachineRepresentation);
        fn set_pending_move(&mut self, move_: &mut MoveOperands);
    }

    pub struct MoveOperands {
        source: InstructionOperand,
        destination: InstructionOperand,
        is_pending: bool,
        is_eliminated: bool,
        is_redundant: bool,
    }

    impl MoveOperands {
        pub fn new(source: InstructionOperand, destination: InstructionOperand) -> Self {
            MoveOperands {
                source,
                destination,
                is_pending: false,
                is_eliminated: false,
                is_redundant: false,
            }
        }

        pub fn source(&self) -> &InstructionOperand {
            &self.source
        }

        pub fn destination(&self) -> &InstructionOperand {
            &self.destination
        }

         pub fn set_source(&mut self, source: AllocatedOperand) {
            // Assuming AllocatedOperand can be converted to InstructionOperand
            self.source = InstructionOperand::from(source);
        }

        pub fn set_destination(&mut self, destination: InstructionOperand) {
            self.destination = destination;
        }

        pub fn is_pending(&self) -> bool {
            self.is_pending
        }

        pub fn set_pending(&mut self) {
            self.is_pending = true;
        }

        pub fn is_eliminated(&self) -> bool {
            self.is_eliminated
        }

        pub fn eliminate(&mut self) {
            self.is_eliminated = true;
        }

        pub fn is_redundant(&self) -> bool {
            self.is_redundant
        }

        pub fn set_redundant(&mut self) {
            self.is_redundant = true;
        }
    }

    pub struct ParallelMove {
        moves: Vec<Rc<RefCell<MoveOperands>>>,
    }

    impl ParallelMove {
        pub fn new() -> Self {
            ParallelMove {
                moves: Vec::new(),
            }
        }

        pub fn push(&mut self, move_: Rc<RefCell<MoveOperands>>) {
            self.moves.push(move_);
        }

        pub fn size(&self) -> usize {
            self.moves.len()
        }

        pub fn resize(&mut self, new_size: usize) {
            self.moves.resize(new_size, Rc::new(RefCell::new(MoveOperands::new(InstructionOperand::Invalid, InstructionOperand::Invalid))));
        }

        pub fn get(&self, index: usize) -> Option<Rc<RefCell<MoveOperands>>> {
            if index < self.moves.len() {
                Some(self.moves[index].clone())
            } else {
                None
            }
        }
    }

    impl std::ops::Index<usize> for ParallelMove {
        type Output = Rc<RefCell<MoveOperands>>;

        fn index(&self, index: usize) -> &Self::Output {
            &self.moves[index]
        }
    }

    impl std::ops::IndexMut<usize> for ParallelMove {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.moves[index]
        }
    }

    pub struct GapResolver<'a, T: Assembler> {
        assembler: &'a mut T,
    }

    impl<'a, T: Assembler> GapResolver<'a, T> {
        pub fn new(assembler: &'a mut T) -> Self {
            GapResolver {
                assembler,
            }
        }

        pub fn resolve(&mut self, parallel_move: &mut ParallelMove) {
            let mut source_kinds: std::collections::HashSet<MoveOperandKind> = std::collections::HashSet::new();
            let mut destination_kinds: std::collections::HashSet<MoveOperandKind> = std::collections::HashSet::new();

            let mut nmoves = parallel_move.size();
            let mut i = 0;

            while i < nmoves {
                let move_rc = parallel_move[i].clone();
                let mut move_ = move_rc.borrow_mut();

                if move_.is_redundant() {
                    nmoves -= 1;
                    if i < nmoves {
                        parallel_move[i] = parallel_move[nmoves].clone();
                    }
                } else {
                    source_kinds.insert(get_kind(&move_.source()));
                    destination_kinds.insert(get_kind(&move_.destination()));
                    i += 1;
                }
            }

            if nmoves != parallel_move.size() {
                parallel_move.resize(nmoves);
            }

            if source_kinds.intersection(&destination_kinds).count() == 0 || parallel_move.size() < 2 {
                // Fast path for non-conflicting parallel moves.
                for i in 0..parallel_move.size() {
                    let move_rc = parallel_move[i].clone();
                    let move_ = move_rc.borrow();
                    self.assembler.assemble_move(&move_.source(), &move_.destination());
                }
                return;
            }

            for i in 0..parallel_move.size() {
                let move_rc = parallel_move[i].clone();
                let mut move_ = move_rc.borrow_mut();
                if !move_.is_eliminated() {
                    self.perform_move(parallel_move, &mut move_);
                }
            }
            self.assembler.pop_temp_stack_slots();
        }

        fn perform_cycle(&mut self, cycle: &Vec<Rc<RefCell<MoveOperands>>>) {
            assert!(!cycle.is_empty());

            let move1_rc = cycle.last().unwrap().clone();
            let mut move1 = move1_rc.borrow_mut();

            if cycle.len() == 2 && is_swap(cycle.first().unwrap(), cycle.last().unwrap()) {
                let move2_rc = cycle.first().unwrap().clone();
                let mut move2 = move2_rc.borrow_mut();

                let mut source = &move1.source();
                let mut destination = &move1.destination();

                if source.is_any_stack_slot() {
                    std::mem::swap(&mut source, &mut destination);
                }

                self.assembler.assemble_swap(source, destination);
                move1.eliminate();
                move2.eliminate();
                return;
            }

            let rep = destination.location_operand().unwrap().representation();

            for i in 0..cycle.len() - 1 {
                let move_rc = cycle[i].clone();
                let mut move_ = move_rc.borrow_mut();
                self.assembler.set_pending_move(&mut move_);
            }

            self.assembler.move_to_temp_location(&move1.source(), rep);
            let destination = move1.destination();
            move1.eliminate();

            for i in 0..cycle.len() - 1 {
                 let move_rc = cycle[i].clone();
                let mut move_ = move_rc.borrow_mut();
                self.assembler.assemble_move(&move_.source(), &move_.destination());
                move_.eliminate();
            }

            self.assembler.move_temp_location_to(&destination, rep);
        }

        fn perform_move(&mut self, moves: &mut ParallelMove, move_: &mut MoveOperands) {
            let mut cycle: Vec<Rc<RefCell<MoveOperands>>> = Vec::new();
            while let Some(blocking_move) = self.perform_move_helper(moves, move_, &mut cycle) {
                let scratch = self.assembler.push(&blocking_move.borrow().source());
                let source = blocking_move.borrow().source();

                for i in 0..moves.size() {
                     let move_rc = moves[i].clone();
                     let mut m = move_rc.borrow_mut();
                    if m.source() == &source {
                        m.set_source(scratch);
                    }
                }
                cycle.clear();
            }
        }

        fn perform_move_helper(
            &mut self,
            moves: &mut ParallelMove,
            move_: &mut MoveOperands,
            cycle: &mut Vec<Rc<RefCell<MoveOperands>>>,
        ) -> Option<Rc<RefCell<MoveOperands>>> {
            if move_.is_pending() || move_.is_redundant() {
                return None;
            }

            let source = move_.source().clone();
            if source.is_invalid() {
                return None;
            }

            let destination = move_.destination().clone();
            move_.set_pending();

            let mut blocking_move: Option<Rc<RefCell<MoveOperands>>> = None;

            for i in 0..moves.size() {
                let other_rc = moves[i].clone();
                let mut other = other_rc.borrow_mut();

                if other.is_eliminated() || Rc::ptr_eq(&moves[i], &Rc::new(RefCell::new(*move_))) {
                    continue;
                }

                if other.source().interferes_with(&destination) {
                    if other.is_pending() {
                        if !cycle.is_empty() {
                            blocking_move = cycle.first().cloned();
                            break;
                        }
                        cycle.push(moves[i].clone());
                    } else {
                        let mut cycle_rec: Vec<Rc<RefCell<MoveOperands>>> = Vec::new();
                        let blocking_move_rec = self.perform_move_helper(moves, &mut other, &mut cycle_rec);

                        if let Some(blocking_move_rec) = blocking_move_rec {
                            blocking_move = Some(blocking_move_rec);
                            break;
                        }

                        if !cycle.is_empty() && !cycle_rec.is_empty() {
                            blocking_move = cycle_rec.first().cloned();
                            break;
                        }

                        if cycle.is_empty() && !cycle_rec.is_empty() {
                            *cycle = cycle_rec;
                        }
                    }
                }
            }

            move_.set_destination(destination);

            if blocking_move.is_some() {
                return blocking_move;
            }

            if !cycle.is_empty() {
                if Rc::ptr_eq(cycle.first().unwrap(), &Rc::new(RefCell::new(*move_))) {
                    self.perform_cycle(cycle);
                    cycle.clear();
                } else {
                     cycle.push(Rc::new(RefCell::new(*move_)));
                }
            } else {
                self.assembler.assemble_move(&source, &destination);
                move_.eliminate();
            }

            None
        }
    }

    #[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
    enum MoveOperandKind {
        Constant,
        GpReg,
        FpReg,
        Stack,
    }

    fn get_kind(move_: &InstructionOperand) -> MoveOperandKind {
        if move_.is_constant() {
            return MoveOperandKind::Constant;
        }
        if let Some(loc_op) = move_.location_operand() {
            if loc_op.location_kind() != crate::compiler::backend::instruction::instruction::LocationOperandKind::REGISTER {
                return MoveOperandKind::Stack;
            }
            if loc_op.representation().is_floating_point() {
                return MoveOperandKind::FpReg;
            } else {
                return MoveOperandKind::GpReg;
            }
        } else {
            MoveOperandKind::Stack // Or a better default
        }
    }

    fn is_swap(move1_rc: &Rc<RefCell<MoveOperands>>, move2_rc: &Rc<RefCell<MoveOperands>>) -> bool {
         let move1 = move1_rc.borrow();
         let move2 = move2_rc.borrow();
        move1.source() == move2.destination() && move2.source() == move1.destination()
    }
}

// src/compiler/backend/gap-resolver.cc
