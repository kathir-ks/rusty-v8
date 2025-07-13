// Converted from V8 C++ source files:
// Header: parallel-move-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod parallel_move_inl {
    use crate::wasm::baseline::liftoff_assembler::LiftoffAssembler;
    use crate::wasm::baseline::parallel_move::ParallelMove;

    impl<'a> ParallelMove<'a> {
        pub fn new(wasm_asm: &'a mut LiftoffAssembler) -> Self {
            let last_spill_offset_ = wasm_asm.top_spill_offset();
            ParallelMove {
                asm_: wasm_asm,
                last_spill_offset_: last_spill_offset_,
            }
        }
    }
}
