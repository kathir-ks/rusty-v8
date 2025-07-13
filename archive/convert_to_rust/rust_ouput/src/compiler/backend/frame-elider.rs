// Converted from V8 C++ source files:
// Header: frame-elider.h
// Implementation: frame-elider.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub struct Reversed<'a, T>(&'a Vec<T>);

    impl<'a, T> Reversed<'a, T> {
        pub fn new(vec: &'a Vec<T>) -> Self {
            Reversed(vec)
        }
    }

    impl<'a, T> IntoIterator for Reversed<'a, T> {
        type Item = &'a T;
        type IntoIter = std::iter::Rev<std::slice::Iter<'a, T>>;

        fn into_iter(self) -> Self::IntoIter {
            self.0.iter().rev()
        }
    }

    impl<'a, T> Reversed<'a, T> {
        pub fn iter(&self) -> std::iter::Rev<std::slice::Iter<'a, T>> {
            self.0.iter().rev()
        }
    }
}

pub mod compiler {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct RpoNumber(usize);

    impl RpoNumber {
        pub fn from_usize(value: usize) -> Self {
            RpoNumber(value)
        }

        pub fn to_usize(&self) -> usize {
            self.0
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum ArchOpcode {
        kArchNop, // Add more opcodes as needed
        kArchStackPointerGreaterThan,
        kArchFramePointer,
        kArchStackSlot,
    }

    #[derive(Debug, Clone)]
    pub struct ImmediateOperand {
        value: i32,
    }

    impl ImmediateOperand {
        pub fn cast(input: &InstructionOperand) -> &Self {
            match input {
                InstructionOperand::Immediate(imm) => imm,
                _ => panic!("Expected ImmediateOperand"),
            }
        }

        pub fn to_i32(&self) -> i32 {
            self.value
        }
    }

    #[derive(Debug, Clone)]
    pub enum InstructionOperand {
        Immediate(ImmediateOperand),
        // Add more operand types as needed
    }

    impl InstructionOperand {
        pub fn is_immediate(&self) -> bool {
            match self {
                InstructionOperand::Immediate(_) => true,
                _ => false,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct Instruction {
        opcode: ArchOpcode,
        inputs: Vec<InstructionOperand>,
        // Add other instruction fields as needed
    }

    impl Instruction {
        pub fn new(opcode: ArchOpcode, inputs: Vec<InstructionOperand>) -> Self {
            Instruction { opcode, inputs }
        }

        pub fn arch_opcode(&self) -> ArchOpcode {
            self.opcode
        }

        pub fn input_at(&self, index: usize) -> &InstructionOperand {
            &self.inputs[index]
        }

        pub fn is_call(&self) -> bool {
            false
        }

        pub fn is_deoptimize_call(&self) -> bool {
            false
        }

        pub fn is_throw(&self) -> bool {
            false
        }

        pub fn is_tail_call(&self) -> bool {
            false
        }

        pub fn is_ret(&self) -> bool {
            false
        }

        pub fn is_jump(&self) -> bool {
            false
        }
    }

    #[derive(Debug, Clone)]
    pub struct InstructionBlock {
        rpo_number: RpoNumber,
        code_start: usize,
        code_end: usize,
        predecessors: Vec<RpoNumber>,
        successors: Vec<RpoNumber>,
        needs_frame: RefCell<bool>,
        must_construct_frame: RefCell<bool>,
        must_deconstruct_frame: RefCell<bool>,
        is_deferred: bool,
    }

    impl InstructionBlock {
        pub fn new(
            rpo_number: RpoNumber,
            code_start: usize,
            code_end: usize,
            predecessors: Vec<RpoNumber>,
            successors: Vec<RpoNumber>,
            is_deferred: bool,
        ) -> Self {
            InstructionBlock {
                rpo_number,
                code_start,
                code_end,
                predecessors,
                successors,
                needs_frame: RefCell::new(false),
                must_construct_frame: RefCell::new(false),
                must_deconstruct_frame: RefCell::new(false),
                is_deferred,
            }
        }

        pub fn rpo_number(&self) -> RpoNumber {
            self.rpo_number
        }

        pub fn code_start(&self) -> usize {
            self.code_start
        }

        pub fn code_end(&self) -> usize {
            self.code_end
        }

        pub fn predecessors(&self) -> &Vec<RpoNumber> {
            &self.predecessors
        }

        pub fn successors(&self) -> &Vec<RpoNumber> {
            &self.successors
        }

        pub fn successor_count(&self) -> usize {
            self.successors.len()
        }

        pub fn needs_frame(&self) -> bool {
            *self.needs_frame.borrow()
        }

        pub fn mark_needs_frame(&self) {
            *self.needs_frame.borrow_mut() = true;
        }

        pub fn must_construct_frame(&self) -> bool {
            *self.must_construct_frame.borrow()
        }

        pub fn mark_must_construct_frame(&self) {
            *self.must_construct_frame.borrow_mut() = true;
        }

        pub fn must_deconstruct_frame(&self) -> bool {
            *self.must_deconstruct_frame.borrow()
        }

        pub fn mark_must_deconstruct_frame(&self) {
            *self.must_deconstruct_frame.borrow_mut() = true;
        }

        pub fn is_deferred(&self) -> bool {
            self.is_deferred
        }

        pub fn predecessor_count(&self) -> usize {
            self.predecessors.len()
        }
    }

    #[derive(Debug, Clone)]
    pub struct InstructionBlocks {
        blocks: Vec<Rc<InstructionBlock>>,
    }

    impl InstructionBlocks {
        pub fn new(blocks: Vec<Rc<InstructionBlock>>) -> Self {
            InstructionBlocks { blocks }
        }

        pub fn iter(&self) -> std::slice::Iter<Rc<InstructionBlock>> {
            self.blocks.iter()
        }

        pub fn get(&self, index: usize) -> Option<&Rc<InstructionBlock>> {
            self.blocks.get(index)
        }
    }

    impl<'a> IntoIterator for &'a InstructionBlocks {
        type Item = &'a Rc<InstructionBlock>;
        type IntoIter = std::slice::Iter<'a, Rc<InstructionBlock>>;

        fn into_iter(self) -> Self::IntoIter {
            self.blocks.iter()
        }
    }

    #[derive(Debug)]
    pub struct InstructionSequence {
        instruction_blocks: InstructionBlocks,
        instructions: Vec<Instruction>,
        immediates: Vec<i32>, // Simplified, store immediates separately.
    }

    impl InstructionSequence {
        pub fn new(instruction_blocks: InstructionBlocks, instructions: Vec<Instruction>, immediates: Vec<i32>) -> Self {
            InstructionSequence {
                instruction_blocks,
                instructions,
                immediates,
            }
        }

        pub fn instruction_blocks(&self) -> &InstructionBlocks {
            &self.instruction_blocks
        }

        pub fn instruction_block_at(&self, rpo_number: RpoNumber) -> &Rc<InstructionBlock> {
            self.instruction_blocks
                .iter()
                .find(|block| block.rpo_number() == rpo_number)
                .expect("InstructionBlock not found")
        }

        pub fn instruction_at(&self, index: usize) -> &Instruction {
            &self.instructions[index]
        }

        pub fn get_immediate(&self, operand: &ImmediateOperand) -> i32 {
            operand.to_i32()
        }
    }

    pub struct FrameElider {
        code: Rc<InstructionSequence>,
        has_dummy_end_block: bool,
        is_wasm_to_js: bool,
    }

    impl FrameElider {
        pub fn new(
            code: Rc<InstructionSequence>,
            has_dummy_end_block: bool,
            is_wasm_to_js: bool,
        ) -> Self {
            FrameElider {
                code,
                has_dummy_end_block,
                is_wasm_to_js,
            }
        }

        pub fn run(&self) {
            self.mark_blocks();
            self.propagate_marks();
            self.mark_de_construction();
        }

        fn mark_blocks(&self) {
            for block in self.code.instruction_blocks().iter() {
                if block.needs_frame() {
                    continue;
                }
                for i in block.code_start()..block.code_end() {
                    let instr = self.code.instruction_at(i);
                    if instr.is_call()
                        || instr.is_deoptimize_call()
                        || instr.arch_opcode() == ArchOpcode::kArchStackPointerGreaterThan
                        || instr.arch_opcode() == ArchOpcode::kArchFramePointer
                    {
                        block.mark_needs_frame();
                        break;
                    }
                    if instr.arch_opcode() == ArchOpcode::kArchStackSlot
                        && match instr.input_at(0) {
                            InstructionOperand::Immediate(imm) => {
                                self.code.get_immediate(imm) > 0
                            }
                            _ => false,
                        } || self.is_wasm_to_js
                    {
                        block.mark_needs_frame();
                        break;
                    }
                }
            }
        }

        fn propagate_marks(&self) {
            while self.propagate_in_order() || self.propagate_reversed() {}
        }

        fn mark_de_construction(&self) {
            for block in self.code.instruction_blocks().iter() {
                if block.needs_frame() {
                    if block.predecessors().is_empty() {
                        block.mark_must_construct_frame();
                        if block.successor_count() == 0 {
                            let last = self.code.instruction_at(block.code_end() - 1);
                            if last.is_ret() || last.is_jump() {
                                block.mark_must_deconstruct_frame();
                            }
                        }
                    }

                    for succ_rpo in block.successors() {
                        let succ = self.instruction_block_at(*succ_rpo);
                        if !succ.needs_frame() {
                            if block.successor_count() != 1 {
                                continue;
                            }
                            let last = self.code.instruction_at(block.code_end() - 1);

                            if last.is_throw() || last.is_tail_call() || last.is_deoptimize_call() {
                                continue;
                            }
                            block.mark_must_deconstruct_frame();
                        }
                    }

                    if block.successor_count() == 0 {
                        let last = self.code.instruction_at(block.code_end() - 1);
                        if last.is_ret() || last.is_jump() {
                            block.mark_must_deconstruct_frame();
                        }
                    }
                } else {
                    for succ_rpo in block.successors() {
                        let succ = self.instruction_block_at(*succ_rpo);
                        if succ.needs_frame() {
                            if block.successor_count() == 1 {
                                continue;
                            }
                            succ.mark_must_construct_frame();
                        }
                    }
                }
            }
        }

        fn propagate_in_order(&self) -> bool {
            let mut changed = false;
            for block in self.code.instruction_blocks().iter() {
                changed |= self.propagate_into_block(block);
            }
            changed
        }

        fn propagate_reversed(&self) -> bool {
            use super::base::Reversed;

            let mut changed = false;
            for block in Reversed::new(self.code.instruction_blocks()).iter() {
                changed |= self.propagate_into_block(block);
            }
            changed
        }

        fn propagate_into_block(&self, block: &Rc<InstructionBlock>) -> bool {
            if block.needs_frame() {
                return false;
            }

            if self.has_dummy_end_block && block.successors().is_empty() {
                return false;
            }

            for pred_rpo in block.predecessors() {
                let pred = self.instruction_block_at(*pred_rpo);
                if pred.needs_frame() && (!pred.is_deferred() || block.is_deferred()) {
                    block.mark_needs_frame();
                    return true;
                }
            }

            let mut need_frame_successors = false;
            if block.successor_count() == 1 {
                need_frame_successors =
                    self.instruction_block_at(block.successors()[0]).needs_frame();
            } else {
                for succ_rpo in block.successors() {
                    let successor_block = self.instruction_block_at(*succ_rpo);
                    if successor_block.predecessor_count() != 1 {
                        continue;
                    }
                    if !successor_block.is_deferred() {
                        if successor_block.needs_frame() {
                            need_frame_successors = true;
                        } else {
                            return false;
                        }
                    }
                }
            }

            if need_frame_successors {
                block.mark_needs_frame();
                return true;
            } else {
                return false;
            }
        }

        fn instruction_blocks(&self) -> &InstructionBlocks {
            self.code.instruction_blocks()
        }

        fn instruction_block_at(&self, rpo_number: RpoNumber) -> &Rc<InstructionBlock> {
            self.code.instruction_block_at(rpo_number)
        }

        fn instruction_at(&self, index: usize) -> &Instruction {
            self.code.instruction_at(index)
        }
    }
}
