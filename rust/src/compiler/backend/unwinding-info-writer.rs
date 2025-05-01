// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Placeholder for unsupported architectures.

// TODO: Add equivalent to logging crate
// use log::{debug, info, warn};

// TODO: Add equivalent to V8 flags.
// const PERF_PROF_UNWINDING_INFO: bool = false;

// static_assert!(
//     !PERF_PROF_UNWINDING_INFO,
//     "--perf-prof-unwinding-info should be statically disabled if not supported"
// );

pub mod compiler {

    pub struct InstructionBlock {}

    pub struct UnwindingInfoWriter {
        // Placeholder for zone, which we are skipping
    }

    impl UnwindingInfoWriter {
        pub fn new() -> Self {
            UnwindingInfoWriter {}
        }

        pub fn set_number_of_instruction_blocks(&mut self, _number: i32) {}

        pub fn begin_instruction_block(&mut self, _pc_offset: i32, _instruction_block: &InstructionBlock) {}

        pub fn end_instruction_block(&mut self, _instruction_block: &InstructionBlock) {}

        pub fn finish(&mut self, _code_size: i32) {}

        pub fn eh_frame_writer(&self) -> Option<()> {
            None
        }
    }
}