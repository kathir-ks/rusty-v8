// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//use std::arch::riscv64::*;
//use std::mem::transmute;

pub mod deoptimizer {
    pub const K_EAGER_DEOPT_EXIT_SIZE: i32 = 2 * K_INSTR_SIZE;
    pub const K_LAZY_DEOPT_EXIT_SIZE: i32 = 2 * K_INSTR_SIZE;

    pub const K_ADAPT_SHADOW_STACK_OFFSET_TO_SUBTRACT: i32 = 0;

    const K_INSTR_SIZE: i32 = 4; // Assuming instruction size is 4 bytes
    //static
    pub fn patch_to_jump(_pc: usize, _new_pc: usize) {
        unimplemented!();
    }

    #[derive(Clone, Copy)]
    pub struct Float32 {
      bits: u32,
    }

    impl Float32 {
      pub fn from_bits(bits: u32) -> Self {
        Float32 { bits }
      }
    }

    #[derive(Clone, Copy)]
    pub struct Float64 {
      bits: u64,
    }

    impl Float64 {
      pub fn from_bits(bits: u64) -> Self {
        Float64 { bits }
      }
    }

    pub struct RegisterValues {
        double_registers_: [Float64; 32],
    }

    impl RegisterValues {
        pub fn new() -> Self {
            RegisterValues {
                double_registers_: [Float64 { bits: 0 }; 32],
            }
        }

        pub fn get_float_register(&self, n: usize) -> Float32 {
            Float32::from_bits(self.double_registers_[n].bits as u32)
        }
        pub fn get_double_register(&self, n: usize) -> Float64 {
            self.double_registers_[n]
        }

        pub fn set_double_register(&mut self, n: usize, value: Float64) {
            self.double_registers_[n] = value;
        }
    }

    pub struct FrameDescription {
        pc_: usize,
        frame_: Vec<i64>,
    }

    impl FrameDescription {
        pub fn new(size: usize) -> Self {
            FrameDescription {
                pc_: 0,
                frame_: vec![0; size],
            }
        }
        fn set_frame_slot(&mut self, offset: usize, value: i64) {
            self.frame_[offset] = value;
        }

        pub fn set_caller_pc(&mut self, offset: usize, value: i64) {
            self.set_frame_slot(offset, value);
        }

        pub fn set_caller_fp(&mut self, offset: usize, value: i64) {
            self.set_frame_slot(offset, value);
        }

        pub fn set_caller_constant_pool(&mut self, _offset: usize, _value: i64) {
          // No embedded constant pool support.
          unimplemented!();
        }

        pub fn set_pc(&mut self, pc: usize) {
            self.pc_ = pc;
        }
    }
}