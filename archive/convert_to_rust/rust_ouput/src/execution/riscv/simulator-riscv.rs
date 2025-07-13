// Converted from V8 C++ source files:
// Header: simulator-riscv.h
// Implementation: simulator-riscv.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_camel_case_types)]
#![allow(dead_code)]
use std::sync::Mutex;

pub struct Simulator {}
pub struct Isolate {}
pub struct Instruction {}
pub struct V8 {}
struct Redirection {}

// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Copyright(c) 2010 - 2017,
//     The Regents of the University of California(Regents).All Rights Reserved.
//
//     Redistribution and use in source and binary forms,
//     with or without modification,
//     are permitted provided that the following
//     conditions are met : 1. Redistributions of source code must retain the
//     above copyright notice, this list of conditions and the following
//     disclaimer.2. Redistributions in binary form must reproduce the above
//     copyright notice, this list of conditions and the following disclaimer in
//     the
//             documentation and /
//         or
//         other materials provided with the distribution.3. Neither the name of
//         the Regents nor the names of its contributors may be used to endorse
//         or
//         promote products derived from
//         this software without specific prior written permission.
//
//         IN NO EVENT SHALL REGENTS BE LIABLE TO ANY PARTY FOR DIRECT,
//     INDIRECT, SPECIAL,
//     INCIDENTAL, OR CONSEQUENTIAL DAMAGES, INCLUDING LOST PROFITS,
//     ARISING OUT OF THE USE OF THIS SOFTWARE AND ITS DOCUMENTATION,
//     EVEN IF REGENTS HAS BEEN ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
//     REGENTS SPECIFICALLY DISCLAIMS ANY WARRANTIES,
//     INCLUDING, BUT NOT LIMITED TO,
//     THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
//     PARTICULAR PURPOSE.THE SOFTWARE AND ACCOMPANYING DOCUMENTATION,
//     IF ANY,
//     PROVIDED HEREUNDER IS PROVIDED
//     "AS IS".REGENTS HAS NO OBLIGATION TO PROVIDE MAINTENANCE,
//     SUPPORT, UPDATES, ENHANCEMENTS,
//     OR MODIFICATIONS.

// The original source code covered by the above license above has been
// modified significantly by the v8 project authors.

// Declares a Simulator for RISC-V instructions if we are not generating a
// native RISC-V binary. This Simulator allows us to run and debug RISC-V code
// generation on regular desktop machines. V8 calls into generated code via the
// GeneratedCode wrapper, which will start execution in the Simulator or
// forwards to the real entry on a RISC-V HW platform.

// globals.h defines USE_SIMULATOR.

template <typename T>
int Compare(const T& a, const T& b) {
  if (a == b)
    return 0;
  else if (a < b)
    return -1;
  else
    return 1;
}

// Returns the negative absolute value of its argument.
template <typename T,
          typename = typename std::enable_if<std::is_signed<T>::value>::type>
T Nabs(T a) {
  return a < 0 ? a : -a;
}
const static xlen:i32 = 64;
pub mod globals {
    pub const USE_SIMULATOR: bool = true;
}
#[cfg(globals::USE_SIMULATOR)]
mod internal {
    use std::{sync::Mutex, vec::Vec, ffi::c_void, cell::Cell};
    use crate::V8;
    use super::*;

    pub type sreg_t = i64;
    pub type reg_t = u64;
    pub type freg_t = u64;
    pub type sfreg_t = i64;
    
    pub fn sext32(x: i32) -> sreg_t {
        x as sreg_t
    }

    pub fn zext32(x: u32) -> reg_t {
        x as reg_t
    }
    
    pub fn sext_xlen(x: i64) -> i64 {
        (x << (64 - xlen)) >> (64 - xlen)
    }

    pub fn zext_xlen(x: u64) -> u64 {
        (x << (64 - xlen)) >> (64 - xlen)
    }

    pub const BIT: i64 = 0x1LL;
    pub fn mulhu(a: u64, b: u64) -> u64 {
        let full_result = (a as u128) * (b as u128);
        (full_result >> 64) as u64
    }

    pub fn mulh(a: i64, b: i64) -> i64 {
        let full_result = (a as i128) * (b as i128);
        (full_result >> 64) as i64
    }

    pub fn mulhsu(a: i64, b: u64) -> i64 {
        let full_result = (a as i128) * (b as u128);
        (full_result >> 64) as i64
    }

    pub const F32_SIGN: u32 = 1 << 31;

    union U32F32 {
        u: u32,
        f: f32,
    }

    impl U32F32 {
        fn new(u: u32, f: f32) -> Self {
            U32F32 { u, f }
        }
    }

    pub fn fsgnj32(rs1: f32, rs2: f32, n: bool, x: bool) -> f32 {
        let a = U32F32::new(rs1.to_bits(), rs1);
        let b = U32F32::new(rs2.to_bits(), rs2);
        let res_u = (a.u & !F32_SIGN) | (if x { a.u } else { if n { F32_SIGN } else { 0 } } ^ b.u) & F32_SIGN;
        f32::from_bits(res_u)
    }

    pub const F64_SIGN: u64 = 1 << 63;

    union U64F64 {
        u: u64,
        d: f64,
    }

    impl U64F64 {
        fn new(u: u64, d: f64) -> Self {
            U64F64 { u, d }
        }
    }

    pub fn fsgnj64(rs1: f64, rs2: f64, n: bool, x: bool) -> f64 {
        let a = U64F64::new(rs1.to_bits(), rs1);
        let b = U64F64::new(rs2.to_bits(), rs2);
        let res_u = (a.u & !F64_SIGN) | (if x { a.u } else { if n { F64_SIGN } else { 0 } } ^ b.u) & F64_SIGN;
        f64::from_bits(res_u)
    }

    pub fn box_float(v: f32) -> i64 {
        (0xFFFFFFFF00000000 | v.to_bits() as i64) as i64
    }

    pub struct CachePage {}
    pub struct SimInstructionBase {}
    pub struct SimInstruction {}
    pub struct SimulatorData {}

    // #[derive(Debug)]
    pub struct Simulator {
        registers_: [i64; kNumSimuRegisters as usize],
        FPUregisters_: [i64; kNumFPURegisters as usize],
        FCSR_: u32,
        stack_: usize,
        stack_limit_: usize,
        pc_modified_: bool,
        icount_: i64,
        last_debugger_input_: Vec<u8>,
        i_cache: Option<()>, // Placeholder for now
        isolate_: *mut Isolate,
        breakpoints_: Vec<Breakpoint>,
    }

    impl Simulator {
        pub const kNumSimuRegisters: i32 = 33;
        pub const kNumFPURegisters: i32 = 32;

        pub fn current(isolate: *mut Isolate) -> *mut Simulator {
            //FIXME
            std::ptr::null_mut()
        }
    
        pub fn set_register(&mut self, reg: usize, value: i64) {
            if reg < Self::kNumSimuRegisters as usize {
                self.registers_[reg] = value;
            }
        }
    
        pub fn get_register(&self, reg: usize) -> i64 {
            if reg < Self::kNumSimuRegisters as usize {
                self.registers_[reg]
            } else {
                0
            }
        }
        fn signal_exception(&self){}
        fn list_breakpoints(&self){}
        fn set_breakpoint(&mut self, _breakpoint: i32, _is_tbreak: bool){}
    }

    pub struct RiscvDebugger {
        sim_: *mut Simulator,
    }
    impl RiscvDebugger {
        fn debug(&self){}
    }

    impl Default for Simulator {
        fn default() -> Self {
            Simulator {
                registers_: [0; Simulator::kNumSimuRegisters as usize],
                FPUregisters_: [0; Simulator::kNumFPURegisters as usize],
                FCSR_: 0,
                stack_: 0,
                stack_limit_: 0,
                pc_modified_: false,
                icount_: 0,
                last_debugger_input_: Vec::new(),
                i_cache: None,
                isolate_: std::ptr::null_mut(),
                breakpoints_:Vec::new(),
            }
        }
    }
    pub const kMaxStopCode: i32 = 1000;
    pub const kSimulatorBreakArgument:i32 = 0;
    pub struct Builtins{}
    impl Builtins{
        fn Lookup(&self, _pc:i32) -> *const i8 {std::ptr::null()}
        fn code(&self, _b:i32)-> BuiltinCode {
            BuiltinCode{}
        }
        fn name(&self, _id:i32)-> *const i8 {
            std::ptr::null()
        }
    }
    pub struct BuiltinCode{}

    // Temporary workaround
    pub const kSmiTagMask: i64 = 0xffffffff00000000;
    pub struct Breakpoint{
        location: i32,
        enabled: bool,
        is_tbreak: bool,
    }
    pub const kArchEndian:i32 = 1;
    pub struct Address {}
    pub struct RootVisitor {}
}

#[cfg(globals::USE_SIMULATOR)]
use internal::*;

#[cfg(not(globals::USE_SIMULATOR))]
mod internal {
    pub type sreg_t = i64;
    pub type reg_t = u64;
    pub type freg_t = u64;
    pub type sfreg_t = i64;
}

