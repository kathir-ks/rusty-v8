// Converted from V8 C++ source files:
// Header: N/A
// Implementation: deoptimizer-x64.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
// use std::os::raw::c_void;

use crate::deoptimizer::deoptimizer::IsolateData;
use crate::deoptimizer::frame_description::{Address, Float32, Float64};

const kSystemPointerSize: usize = 8;

pub struct AssemblerOptions {}

pub struct ExternalAssemblerBuffer {
    pc: *mut u8,
    size: usize,
}

impl ExternalAssemblerBuffer {
    pub fn new(pc: *mut u8, size: usize) -> Self {
        ExternalAssemblerBuffer { pc, size }
    }
}

pub struct Assembler {
    options: AssemblerOptions,
    buffer: ExternalAssemblerBuffer,
}

impl Assembler {
    pub fn new(options: AssemblerOptions, buffer: ExternalAssemblerBuffer) -> Self {
        Assembler { options, buffer }
    }
    pub fn jmp_rel(&mut self, offset: i32) {
        unsafe {
            let pc = self.buffer.pc;
            let mut instr = vec![0u8; 5];
            instr[0] = 0xE9;
            instr[1..5].copy_from_slice(&offset.to_le_bytes());
            std::ptr::copy_nonoverlapping(instr.as_ptr(), pc, 5);
        }
    }

    pub fn is_nop(pc: Address) -> bool {
        unsafe {
            let byte = *(pc as *const u8);
            byte == 0x90 // NOP instruction
        }
    }

    pub fn is_jmp_rel(pc: Address) -> bool {
        unsafe {
            let byte = *(pc as *const u8);
            byte == 0xE9
        }
    }
}

pub struct RwxMemoryWriteScope {}

impl RwxMemoryWriteScope {
    pub fn new(_message: &str) -> Self {
        RwxMemoryWriteScope {}
    }
}

pub struct FlushInstructionCache {}

impl FlushInstructionCache {
    pub fn flush(_pc: Address, _size: usize) {}
}

pub struct Deoptimizer {}

impl Deoptimizer {
    pub const kEagerDeoptExitSize: i32 = 4;
    #[cfg(not(V8_ENABLE_CET_IBT))]
    pub const kLazyDeoptExitSize: i32 = 4;
    #[cfg(V8_ENABLE_CET_IBT)]
    pub const kLazyDeoptExitSize: i32 = 8;
    #[cfg(not(V8_ENABLE_CET_SHADOW_STACK))]
    pub const kAdaptShadowStackOffsetToSubtract: i32 = 0;
    #[cfg(V8_ENABLE_CET_SHADOW_STACK)]
    pub const kAdaptShadowStackOffsetToSubtract: i32 = 7;

    pub fn patch_to_jump(pc: Address, new_pc: Address) {
        if !Assembler::is_nop(pc) {
            if !Assembler::is_jmp_rel(pc) {
                eprintln!("Not a NOP or JMP instruction at {:p}", pc);
            }
            return;
        }

        let _rwx_write_scope = RwxMemoryWriteScope::new("Patch jump to deopt trampoline");
        let displacement = new_pc as isize - (pc as isize + 5);
        if displacement > i32::MAX as isize || displacement < i32::MIN as isize {
            eprintln!("Displacement out of i32 range: {}", displacement);
            return;
        }

        const KSIZE: usize = 32;
        let mut masm = Assembler::new(
            AssemblerOptions {},
            ExternalAssemblerBuffer::new(pc as *mut u8, KSIZE),
        );
        let offset = (new_pc as usize).wrapping_sub(pc as usize) as i32;
        masm.jmp_rel(offset);
        FlushInstructionCache::flush(pc, KSIZE);
    }
}

pub struct RegisterValues {
    simd128_registers_: [u128; 32],
}

impl RegisterValues {
    pub fn get_float_register(&self, n: u32) -> Float32 {
        unsafe {
            let ptr = &self.simd128_registers_[n as usize] as *const u128 as *const Float32;
            *ptr
        }
    }

    pub fn get_double_register(&self, n: u32) -> Float64 {
        unsafe {
            let ptr = &self.simd128_registers_[n as usize] as *const u128 as *const Float64;
            *ptr
        }
    }

    pub fn set_double_register(&mut self, n: u32, value: Float64) {
        unsafe {
            let ptr = &mut self.simd128_registers_[n as usize] as *mut u128 as *mut Float64;
            *ptr = value;
        }
    }
}

pub struct FrameDescription {
    frame_: Vec<i64>,
    caller_pc_: i64,
    pc_: i64,
}

impl FrameDescription {
    pub fn new(size: usize) -> Self {
        FrameDescription {
            frame_: vec![0; size],
            caller_pc_: 0,
            pc_: 0,
        }
    }
    fn set_frame_slot(&mut self, offset: usize, value: i64) {
        if offset < self.frame_.len() {
            self.frame_[offset] = value;
        } else {
            eprintln!("Offset out of bounds: {}", offset);
        }
    }
    pub fn set_caller_pc(&mut self, offset: usize, value: i64) {
        self.set_frame_slot(offset, value);
        self.caller_pc_ = value;
    }

    pub fn set_caller_fp(&mut self, offset: usize, value: i64) {
        self.set_frame_slot(offset, value);
    }
    pub fn set_caller_constant_pool(&mut self, _offset: usize, _value: i64) {
        panic!("UNREACHABLE");
    }

    pub fn set_pc(&mut self, pc: i64) {
        self.pc_ = pc;
    }
}

pub struct Builtins {}

impl Builtins {
    pub const kDeoptimizationEntry_Eager: i32 = 0;
    pub const kDeoptimizationEntry_Lazy: i32 = 1;

    pub fn to_int(builtin_name: i32) -> i32 {
        builtin_name
    }
}
