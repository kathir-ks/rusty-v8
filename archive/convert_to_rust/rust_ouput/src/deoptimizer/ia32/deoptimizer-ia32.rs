// Converted from V8 C++ source files:
// Header: N/A
// Implementation: deoptimizer-ia32.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]

use std::mem;

pub struct Float32(f32);

impl Float32 {
    pub fn new(value: f32) -> Self {
        Float32(value)
    }
}

pub struct Float64(f64);

impl Float64 {
    pub fn new(value: f64) -> Self {
        Float64(value)
    }
}

pub type Address = usize;
pub struct RegisterValues {
    simd128_registers_: [u8; 16 * 16], // Assuming 16 registers, each 16 bytes
}

impl RegisterValues {
    pub fn new() -> Self {
        RegisterValues {
            simd128_registers_: [0; 16 * 16],
        }
    }

    pub fn get_float_register(&self, n: usize) -> Float32 {
        assert!(n < 16);
        let start = n * 16;
        let bytes: [u8; 4] = [
            self.simd128_registers_[start],
            self.simd128_registers_[start + 1],
            self.simd128_registers_[start + 2],
            self.simd128_registers_[start + 3],
        ];
        Float32(f32::from_ne_bytes(bytes))
    }

    pub fn get_double_register(&self, n: usize) -> Float64 {
        assert!(n < 16);
        let start = n * 16;
        let bytes: [u8; 8] = [
            self.simd128_registers_[start],
            self.simd128_registers_[start + 1],
            self.simd128_registers_[start + 2],
            self.simd128_registers_[start + 3],
            self.simd128_registers_[start + 4],
            self.simd128_registers_[start + 5],
            self.simd128_registers_[start + 6],
            self.simd128_registers_[start + 7],
        ];
        Float64(f64::from_ne_bytes(bytes))
    }

    pub fn set_double_register(&mut self, n: usize, value: Float64) {
        assert!(n < 16);
        let start = n * 16;
        let bytes = value.0.to_ne_bytes();
        for i in 0..8 {
            self.simd128_registers_[start + i] = bytes[i];
        }
    }
}

pub struct FrameDescription {
    pc_: usize,
    frame_: Vec<i64>, // Using a Vec to represent frame slots
}

impl FrameDescription {
    pub fn new(size: usize) -> Self {
        FrameDescription {
            pc_: 0,
            frame_: vec![0; size],
        }
    }

    pub fn set_caller_pc(&mut self, offset: usize, value: i64) {
        self.set_frame_slot(offset, value);
    }

    pub fn set_caller_fp(&mut self, offset: usize, value: i64) {
        self.set_frame_slot(offset, value);
    }

    pub fn set_caller_constant_pool(&mut self, _offset: usize, _value: i64) {
        panic!("No embedded constant pool support.");
    }

    pub fn set_pc(&mut self, pc: usize) {
        self.pc_ = pc;
    }

    fn set_frame_slot(&mut self, offset: usize, value: i64) {
        if offset >= self.frame_.len() {
            self.frame_.resize(offset + 1, 0);
        }
        self.frame_[offset] = value;
    }

    pub fn get_pc(&self) -> usize {
        self.pc_
    }

    pub fn get_frame_slot(&self, offset: usize) -> i64 {
        if offset >= self.frame_.len() {
            0
        } else {
            self.frame_[offset]
        }
    }
}

pub struct Deoptimizer {}

impl Deoptimizer {
    pub const KEAGER_DEOPT_EXIT_SIZE: i32 = 5;
    pub const KLAZY_DEOPT_EXIT_SIZE: i32 = 5;
    pub const KADAPT_SHADOW_STACK_OFFSET_TO_SUBTRACT: i32 = 0;

    pub fn patch_to_jump(_pc: Address, _new_pc: Address) {
        panic!("UNREACHABLE");
    }
}
