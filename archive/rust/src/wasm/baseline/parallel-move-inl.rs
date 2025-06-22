// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/wasm/baseline/parallel-move-inl.h

use crate::wasm::baseline::liftoff_assembler::LiftoffAssembler;
use crate::wasm::baseline::parallel_move::ParallelMove as OriginalParallelMove;

pub mod parallel_move_wrapper {
    use super::*;

    pub struct ParallelMove<'a> {
        asm: &'a mut LiftoffAssembler, // Assuming LiftoffAssembler needs mutability
        last_spill_offset: i32,
    }

    impl<'a> ParallelMove<'a> {
        pub fn new(wasm_asm: &'a mut LiftoffAssembler) -> Self {
            ParallelMove {
                asm: wasm_asm,
                last_spill_offset: wasm_asm.top_spill_offset(),
            }
        }
    }
}