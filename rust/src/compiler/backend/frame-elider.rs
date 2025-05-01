// Copyright 2015 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod frame_elider {
    use std::rc::Rc;

    // Placeholder for InstructionSequence
    pub struct InstructionSequence {}

    // Placeholder for InstructionBlock
    pub struct InstructionBlock {}

    // Placeholder for InstructionBlocks
    pub struct InstructionBlocks {}

    // Placeholder for Instruction
    pub struct Instruction {}

    // Placeholder for RpoNumber
    #[derive(Clone, Copy)]
    pub struct RpoNumber {}

    /// Determines which instruction blocks need a frame and where frames must be
    /// constructed/deconstructed.
    pub struct FrameElider {
        code: Rc<InstructionSequence>,
        has_dummy_end_block: bool,
        is_wasm_to_js: bool,
    }

    impl FrameElider {
        /// Creates a new `FrameElider`.
        pub fn new(code: Rc<InstructionSequence>, has_dummy_end_block: bool, is_wasm_to_js: bool) -> Self {
            FrameElider {
                code,
                has_dummy_end_block,
                is_wasm_to_js,
            }
        }

        /// Runs the frame elision process.
        pub fn run(&mut self) {
            self.mark_blocks();
            self.propagate_marks();
            self.mark_de_construction();
        }

        fn mark_blocks(&mut self) {}

        fn propagate_marks(&mut self) {}

        fn mark_de_construction(&mut self) {}

        fn propagate_in_order(&mut self) -> bool {
            true // dummy return
        }

        fn propagate_reversed(&mut self) -> bool {
            true // dummy return
        }

        fn propagate_into_block(&mut self, _block: &mut InstructionBlock) -> bool {
            true // dummy return
        }

        fn instruction_blocks(&self) -> &InstructionBlocks {
            todo!() // unimplemented
        }

        fn instruction_block_at(&self, _rpo_number: RpoNumber) -> Option<&InstructionBlock> {
            None //dummy return
        }

        fn instruction_at(&self, _index: usize) -> Option<&Instruction> {
            None //dummy return
        }
    }
}