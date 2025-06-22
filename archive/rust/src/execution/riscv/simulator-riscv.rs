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
// #include "src/common/globals.h"

#[cfg(feature = "simulator")]
pub mod simulator_riscv {
    // use std::cmp::Ordering;
    use std::mem::transmute;
    use std::{mem, ops};
    // use crate::base::hashmap::HashMap;
    // use crate::codegen::assembler::Assembler;
    // use crate::codegen::constants_arch::*;
    // use crate::execution::simulator_base::SimulatorBase;
    // use crate::utils::allocation::Address;
    // use crate::utils::boxed_float::*;
    // use crate::heap::base::StackVisitor;
    // use std::os::raw::c_char;

    /// Compares two values of the same type.
    pub fn compare<T: PartialOrd>(a: &T, b: &T) -> i32 {
        if a == b {
            0
        } else if a < b {
            -1
        } else {
            1
        }
    }

    /// Returns the negative absolute value of its argument.
    pub fn nabs<T>(a: T) -> T
    where
        T: ops::Neg<Output = T> + PartialOrd,
    {
        if a < unsafe { mem::zeroed() } {
            a
        } else {
            -a
        }
    }

    #[cfg(target_arch = "x86_64")]
    type Int128 = i128;
    #[cfg(target_arch = "x86_64")]
    type UInt128 = u128;

    // #[cfg(feature = "simulator")]
    // Running with a simulator.

    // -----------------------------------------------------------------------------
    // Utility types and functions for RISCV
    #[cfg(target_arch = "riscv32")]
    pub type SRegT = i32;
    #[cfg(target_arch = "riscv32")]
    pub type RegT = u32;
    #[cfg(target_arch = "riscv32")]
    pub type FRegT = u64;
    #[cfg(target_arch = "riscv32")]
    pub type SFRegT = i64;

    #[cfg(target_arch = "riscv64")]
    pub type SRegT = i64;
    #[cfg(target_arch = "riscv64")]
    pub type RegT = u64;
    #[cfg(target_arch = "riscv64")]
    pub type FRegT = u64;
    #[cfg(target_arch = "riscv64")]
    pub type SFRegT = i64;

    // TODO: Add error checking for riscv32 or riscv64
    // #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
    // compile_error!("Cannot detect Riscv's bitwidth");

    macro_rules! sext32 {
        ($x:expr) => {
            (($x as i32) as SRegT)
        };
    }
    pub(crate) use sext32;

    macro_rules! zext32 {
        ($x:expr) => {
            (($x as u32) as RegT)
        };
    }
    pub(crate) use zext32;

    // TODO: Find xlen
    // extern crate compile_error;
    // #[cfg(target_arch = "riscv64")]
    // pub const XLEN: i32 = 64;
    // #[cfg(target_arch = "riscv32")]
    // pub const XLEN: i32 = 32;

    // macro_rules! sext_xlen {
    //     ($x:expr) => {
    //         (($x as SRegT) << (64 - XLEN) ) >> (64 - XLEN)
    //     };
    // }

    // macro_rules! zext_xlen {
    //     ($x:expr) => {
    //         (($x as RegT) << (64 - XLEN)) >> (64 - XLEN)
    //     };
    // }

    // pub(crate) use sext_xlen;
    // pub(crate) use zext_xlen;

    macro_rules! bit {
        ($n:expr) => {
            0x1_u64 << $n
        };
    }
    pub(crate) use bit;

    // macro_rules! quiet_bit_s {
    //     ($nan:expr) => {
    //         transmute::<f32, i32>($nan) & bit!(22) as i32
    //     };
    // }
    // pub(crate) use quiet_bit_s;

    // macro_rules! quiet_bit_d {
    //     ($nan:expr) => {
    //         transmute::<f64, i64>($nan) & bit!(51) as i64
    //     };
    // }
    // pub(crate) use quiet_bit_d;

    // inline bool isSnan(float fp) { return !QUIET_BIT_S(fp); }
    // inline bool isSnan(double fp) { return !QUIET_BIT_D(fp); }

    // #[inline]
    // fn is_snan_f32(fp: f32) -> bool {
    //     quiet_bit_s!(fp) == 0
    // }

    // #[inline]
    // fn is_snan_f64(fp: f64) -> bool {
    //     quiet_bit_d!(fp) == 0
    // }

    #[cfg(target_arch = "riscv64")]
    #[inline]
    fn mulhu(a: u64, b: u64) -> u64 {
        let full_result: u128 = (a as u128) * (b as u128);
        (full_result >> 64) as u64
    }

    #[cfg(target_arch = "riscv64")]
    #[inline]
    fn mulh(a: i64, b: i64) -> i64 {
        let full_result: i128 = (a as i128) * (b as i128);
        (full_result >> 64) as i64
    }

    #[cfg(target_arch = "riscv64")]
    #[inline]
    fn mulhsu(a: i64, b: u64) -> i64 {
        let full_result: i128 = (a as i128) * (b as u128);
        (full_result >> 64) as i64
    }

    #[cfg(target_arch = "riscv32")]
    #[inline]
    fn mulhu(a: u32, b: u32) -> u32 {
        let full_result: u64 = (a as u64) * (b as u64);
        let upper_part: u64 = full_result >> 32;
        upper_part as u32
    }

    #[cfg(target_arch = "riscv32")]
    #[inline]
    fn mulh(a: i32, b: i32) -> i32 {
        let full_result: i64 = (a as i64) * (b as i64);
        let upper_part: i64 = full_result >> 32;
        upper_part as i32
    }

    #[cfg(target_arch = "riscv32")]
    #[inline]
    fn mulhsu(a: i32, b: u32) -> i32 {
        let full_result: i64 = (a as i64) * (b as u64);
        let upper_part: i64 = full_result >> 32;
        upper_part as i32
    }

    // Floating point helpers
    const F32_SIGN: u32 = 1 << 31;

    union U32F32 {
        u: u32,
        f: f32,
    }

    #[inline]
    fn fsgnj32(rs1: f32, rs2: f32, n: bool, x: bool) -> f32 {
        unsafe {
            let a: U32F32 = U32F32 { f: rs1 };
            let b: U32F32 = U32F32 { f: rs2 };
            let mut res: U32F32;
            res.u = (a.u & !F32_SIGN) | (if x { a.u } else { if n { F32_SIGN } else { 0 } } ^ b.u) & F32_SIGN;
            res.f
        }
    }

    // struct Float32 {
    //   bits_: u32,
    // }

    // impl Float32 {
    //   pub fn from_bits(bits: u32) -> Self {
    //     Float32 { bits_: bits }
    //   }
    //   pub fn get_bits(&self) -> u32 {
    //     self.bits_
    //   }
    // }
    // inline Float32 fsgnj32(Float32 rs1, Float32 rs2, bool n, bool x) {
    //   u32_f32 a = {.u = rs1.get_bits()}, b = {.u = rs2.get_bits()};
    //   u32_f32 res;
    //   if (x) {  // RO_FSQNJX_S
    //     res.u = (a.u & ~F32_SIGN) | ((a.u ^ b.u) & F32_SIGN);
    //   } else {
    //     if (n) {  // RO_FSGNJN_S
    //       res.u = (a.u & ~F32_SIGN) | ((F32_SIGN ^ b.u) & F32_SIGN);
    //     } else {  // RO_FSGNJ_S
    //       res.u = (a.u & ~F32_SIGN) | ((0 ^ b.u) & F32_SIGN);
    //     }
    //   }
    //   return Float32::FromBits(res.u);
    // }

    const F64_SIGN: u64 = 1 << 63;

    union U64F64 {
        u: u64,
        d: f64,
    }

    #[inline]
    fn fsgnj64(rs1: f64, rs2: f64, n: bool, x: bool) -> f64 {
        unsafe {
            let a: U64F64 = U64F64 { d: rs1 };
            let b: U64F64 = U64F64 { d: rs2 };
            let mut res: U64F64;
            res.u = (a.u & !F64_SIGN) | (if x { a.u } else { if n { F64_SIGN } else { 0 } } ^ b.u) & F64_SIGN;
            res.d
        }
    }

    // struct Float64 {
    //   bits_: u64,
    // }

    // impl Float64 {
    //   pub fn from_bits(bits: u64) -> Self {
    //     Float64 { bits_: bits }
    //   }
    //   pub fn get_bits(&self) -> u64 {
    //     self.bits_
    //   }
    // }

    // inline Float64 fsgnj64(Float64 rs1, Float64 rs2, bool n, bool x) {
    //   u64_f64 a = {.u = rs1.get_bits()}, b = {.u = rs2.get_bits()};
    //   u64_f64 res;
    //   if (x) {  // RO_FSQNJX_D
    //     res.u = (a.u & ~F64_SIGN) | ((a.u ^ b.u) & F64_SIGN);
    //   } else {
    //     if (n) {  // RO_FSGNJN_D
    //       res.u = (a.u & ~F64_SIGN) | ((F64_SIGN ^ b.u) & F64_SIGN);
    //     } else {  // RO_FSGNJ_D
    //       res.u = (a.u & ~F64_SIGN) | ((0 ^ b.u) & F64_SIGN);
    //     }
    //   }
    //   return Float64::FromBits(res.u);
    // }

    // inline bool is_boxed_float(int64_t v) { return (uint32_t)((v >> 32) + 1) == 0; }
    // inline int64_t box_float(float v) {
    //   return (0xFFFFFFFF00000000 | base::bit_cast<int32_t>(v));
    // }

    // inline uint64_t box_float(uint32_t v) { return (0xFFFFFFFF00000000 | v); }

    /// Represents a cache page.
    pub struct CachePage {
        data_: [u8; Self::K_PAGE_SIZE],
        validity_map_: [u8; Self::K_VALIDITY_MAP_SIZE],
    }

    impl CachePage {
        pub const LINE_VALID: i32 = 0;
        pub const LINE_INVALID: i32 = 1;

        pub const K_PAGE_SHIFT: usize = 12;
        pub const K_PAGE_SIZE: usize = 1 << Self::K_PAGE_SHIFT;
        pub const K_PAGE_MASK: usize = Self::K_PAGE_SIZE - 1;
        pub const K_LINE_SHIFT: usize = 2; // The cache line is only 4 bytes right now.
        pub const K_LINE_LENGTH: usize = 1 << Self::K_LINE_SHIFT;
        pub const K_LINE_MASK: usize = Self::K_LINE_LENGTH - 1;

        const K_VALIDITY_MAP_SIZE: usize = Self::K_PAGE_SIZE >> Self::K_LINE_SHIFT;

        /// Creates a new `CachePage` with all lines marked as invalid.
        pub fn new() -> Self {
            CachePage {
                data_: [0u8; Self::K_PAGE_SIZE],
                validity_map_: [Self::LINE_INVALID as u8; Self::K_VALIDITY_MAP_SIZE],
            }
        }

        /// Returns a mutable pointer to the validity byte for a given offset.
        pub fn validity_byte(&mut self, offset: usize) -> &mut u8 {
            &mut self.validity_map_[offset >> Self::K_LINE_SHIFT]
        }

        /// Returns a mutable pointer to the cached data for a given offset.
        pub fn cached_data(&mut self, offset: usize) -> &mut u8 {
            &mut self.data_[offset]
        }
    }

    // InstructionBase is missing
    // pub struct SimInstructionBase {}

    // impl SimInstructionBase {
    //   pub fn instruction_type(&self) -> Type {
    //     self.type_
    //   }
    //   pub fn instr(&self) -> &Instruction {
    //     &self.instr_
    //   }
    //   pub fn operand(&self) -> i32 {
    //     self.operand_
    //   }

    // }
    // struct Instruction{}
    // #[derive(Debug, Copy, Clone)]
    // pub enum Type {
    //   Unsupported
    // }
    // pub struct InstructionBase{}
    // impl InstructionBase {
    //   pub fn InstructionType() -> Type {
    //     Type::Unsupported
    //   }
    // }

    // pub struct SimInstruction {
    //   operand_: i32,
    //   instr_: Instruction,
    //   type_: Type,
    // }
    // impl SimInstruction {
    //   pub fn new() -> Self {
    //     SimInstruction {
    //       operand_: -1,
    //       instr_: Instruction{},
    //       type_: Type::Unsupported,
    //     }
    //   }
    //   pub fn from_instruction(instr: &Instruction) -> Self {
    //     SimInstruction {
    //       operand_: 0,
    //       instr_: Instruction{},
    //       type_: Type::Unsupported,
    //     }
    //   }

    //   pub fn rs1_value(&self) -> i32 {
    //     0
    //   }
    //   pub fn rs2_value(&self) -> i32 {
    //     0
    //   }
    //   pub fn rs3_value(&self) -> i32 {
    //     0
    //   }
    //   pub fn rd_value(&self) -> i32 {
    //     0
    //   }
    //   pub fn rvc_rs1_value(&self) -> i32 {
    //     0
    //   }
    //   pub fn rvc_rs2_value(&self) -> i32 {
    //     0
    //   }
    //   pub fn rvc_rs1s_value(&self) -> i32 {
    //     0
    //   }
    //   pub fn rvc_rs2s_value(&self) -> i32 {
    //     0
    //   }
    //   pub fn rvc_rd_value(&self) -> i32 {
    //     0
    //   }
    //   pub fn branch_offset(&self) -> i16 {
    //     0
    //   }
    //   pub fn imm12_value(&self) -> i16 {
    //     0
    //   }
    //   pub fn imm20j_value(&self) -> i32 {
    //     0
    //   }
    //   pub fn csr_value(&self) -> i16 {
    //     0
    //   }
    //   pub fn rvc_imm6_value(&self) -> i16 {
    //     0
    //   }
    //   pub fn rvc_imm6_addi16sp_value(&self) -> i16 {
    //     0
    //   }
    //   pub fn rvc_imm8_addi4spn_value(&self) -> i16 {
    //     0
    //   }
    //   pub fn rvc_imm6_lwsp_value(&self) -> i16 {
    //     0
    //   }
    //   pub fn rvc_imm6_ldsp_value(&self) -> i16 {
    //     0
    //   }
    //   pub fn rvc_imm6_swsp_value(&self) -> i16 {
    //     0
    //   }
    //   pub fn rvc_imm6_sdsp_value(&self) -> i16 {
    //     0
    //   }
    //   pub fn rvc_imm5_w_value(&self) -> i16 {
    //     0
    //   }
    //   pub fn rvc_imm5_d_value(&self) -> i16 {
    //     0
    //   }
    //   pub fn rvc_imm8_b_value(&self) -> i16 {
    //     0
    //   }
    //   pub fn store_offset(&self) -> i32 {
    //     0
    //   }
    //   pub fn imm20u_value(&self) -> i32 {
    //     0
    //   }
    //   pub fn rvc_imm6_value_uimm(&self) -> i32 {
    //     0
    //   }
    // }

    // impl ops::Deref for SimInstruction {
    //   type Target = SimInstructionBase;
    //   fn deref(&self) -> &Self::Target {
    //     unimplemented!()
    //   }
    // }

    /// Represents the RISC-V simulator.
    pub struct Simulator {}

    impl Simulator {
        // Registers are declared in order. See SMRL chapter 2.
        #[allow(dead_code)]
        pub enum Register {
            NoReg = -1,
            ZeroReg = 0,
            Ra,
            Sp,
            Gp,
            Tp,
            T0,
            T1,
            T2,
            S0,
            S1,
            A0,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            S2,
            S3,
            S4,
            S5,
            S6,
            S7,
            S8,
            S9,
            S10,
            S11,
            T3,
            T4,
            T5,
            T6,
            Pc, // pc must be the last register.
            KNumSimuRegisters,
            // aliases
            Fp = Self::S0 as isize,
        }

        // Coprocessor registers.
        // Generated code will always use doubles. So we will only use even registers.
        #[allow(dead_code)]
        pub enum FPURegister {
            Ft0,
            Ft1,
            Ft2,
            Ft3,
            Ft4,
            Ft5,
            Ft6,
            Ft7,
            Fs0,
            Fs1,
            Fa0,
            Fa1,
            Fa2,
            Fa3,
            Fa4,
            Fa5,
            Fa6,
            Fa7,
            Fs2,
            Fs3,
            Fs4,
            Fs5,
            Fs6,
            Fs7,
            Fs8,
            Fs9,
            Fs10,
            Fs11,
            Ft8,
            Ft9,
            Ft10,
            Ft11,
            KNumFPURegisters,
        }

        #[allow(dead_code)]
        pub enum VRegister {
            V0,
            V1,
            V2,
            V3,
            V4,
            V5,
            V6,
            V7,
            V8,
            V9,
            V10,
            V11,
            V12,
            V13,
            V14,
            V15,
            V16,
            V17,
            V18,
            V19,
            V20,
            V21,
            V22,
            V23,
            V24,
            V25,
            V26,
            V27,
            V28,
            V29,
            V30,
            V31,
            KNumVRegisters,
        }

        /// Creates a new `Simulator`.
        pub fn new() -> Self {
            Simulator {}
        }

        /// Sets the value of a register.
        pub fn set_register(&mut self, reg: i32, value: SRegT) {
            // Implementation goes here
            println!("Setting register {} to {}", reg, value);
        }

        /// Sets the value of a register, interpreting the value as an i32.
        pub fn set_register_word(&mut self, reg: i32, value: i32) {
            // Implementation goes here
            println!("Setting register word {} to {}", reg, value);
        }

        /// Gets the value of a register.
        pub fn get_register(&self, reg: i32) -> SRegT {
            // Implementation goes here
            println!("Getting register {}", reg);
            0
        }

        pub fn get_double_from_register_pair(&self, reg: i32) -> f64 {
            println!("getting double from register pair {}", reg);
            0.0
        }

        /// Sets the value of an FPU register.
        pub fn set_fpu_register(&mut self, fpureg: i32, value: i64) {
            // Implementation goes here
            println!("Setting FPU register {} to {}", fpureg, value);
        }

        /// Sets the value of an FPU register, interpreting the value as an i32.
        pub fn set_fpu_register_word(&mut self, fpureg: i32, value: i32) {
            // Implementation goes here
            println!("Setting FPU register word {} to {}", fpureg, value);
        }

        /// Sets the high word of an FPU register, interpreting the value as an i32.
        pub fn set_fpu_register_hi_word(&mut self, fpureg: i32, value: i32) {
            // Implementation goes here
            println!("Setting FPU register hi word {} to {}", fpureg, value);
        }

        /// Sets the value of an FPU register, interpreting the value as a float.
        pub fn set_fpu_register_float(&mut self, fpureg: i32, value: f32) {
            // Implementation goes here
            println!("Setting FPU register float {} to {}", fpureg, value);
        }

        // /// Sets the value of an FPU register, interpreting the value as a Float32.
        // pub fn set_fpu_register_float32(&mut self, fpureg: i32, value: Float32) {
        //     // Implementation goes here
        //     println!("Setting FPU register float {} to {:?}", fpureg, value);
        // }

        /// Sets the value of an FPU register, interpreting the value as a double.
        pub fn set_fpu_register_double(&mut self, fpureg: i32, value: f64) {
            // Implementation goes here
            println!("Setting FPU register double {} to {}", fpureg, value);
        }

        // /// Sets the value of an FPU register, interpreting the value as a Float64.
        // pub fn set_fpu_register_float64(&mut self, fpureg: i32, value: Float64) {
        //     // Implementation goes here
        //     println!("Setting FPU register double {} to {:?}", fpureg, value);
        // }

        /// Gets the value of an FPU register.
        pub fn get_fpu_register(&self, fpureg: i32) -> i64 {
            // Implementation goes here
            println!("Getting FPU register {}", fpureg);
            0
        }

        /// Gets the value of an FPU register, interpreting the value as an i32.
        pub fn get_fpu_register_word(&self, fpureg: i32) -> i32 {
            // Implementation goes here
            println!("Getting FPU register word {}", fpureg);
            0
        }

        pub fn get_fpu_register_signed_word(&self, fpureg: i32) -> i32 {
            // Implementation goes here
            println!("Getting FPU register signed word {}", fpureg);
            0
        }

        /// Gets the high word of an FPU register, interpreting the value as an i32.
        pub fn get_fpu_register_hi_word(&self, fpureg: i32) -> i32 {
            // Implementation goes here
            println!("Getting FPU register hi word {}", fpureg);
            0
        }

        /// Gets the value of an FPU register, interpreting the value as a float.
        pub fn get_fpu_register_float(&self, fpureg: i32) -> f32 {
            // Implementation goes here
            println!("Getting FPU register float {}", fpureg);
            0.0
        }

        // /// Gets the value of an FPU register, interpreting the value as a Float32.
        // pub fn get_fpu_register_float32(&self, fpureg: i32, check_nanbox: bool) -> Float32 {
        //     // Implementation goes here
        //     println!("Getting FPU register float {} check nanbox {}", fpureg, check_nanbox);
        //     Float32::from_bits(0)
        // }

        /// Gets the value of an FPU register, interpreting the value as a double.
        pub fn get_fpu_register_double(&self, fpureg: i32) -> f64 {
            // Implementation goes here
            println!("Getting FPU register double {}", fpureg);
            0.0
        }

        // /// Gets the value of an FPU register, interpreting the value as a Float64.
        // pub fn get_fpu_register_float64(&self, fpureg: i32) -> Float64 {
        //     // Implementation goes here
        //     println!("Getting FPU register double {}", fpureg);
        //     Float64::from_bits(0)
        // }

        /// Reads a value from a CSR (Control and Status Register).
        pub fn read_csr_value(&self, csr: u32) -> u32 {
            // Implementation goes here
            println!("Reading CSR value {}", csr);
            0
        }

        /// Writes a value to a CSR (Control and Status Register).
        pub fn write_csr_value(&mut self, csr: u32, value: RegT) {
            // Implementation goes here
            println!("Writing CSR value {} with {}", csr, value);
        }

        /// Sets bits in a CSR (Control and Status Register).
        pub fn set_csr_bits(&mut self, csr: u32, flags: RegT) {
            // Implementation goes here
            println!("Setting CSR bits {} with {}", csr, flags);
        }

        /// Clears bits in a CSR (Control and Status Register).
        pub fn clear_csr_bits(&mut self, csr: u32, flags: RegT) {
            // Implementation goes here
            println!("Clearing CSR bits {} with {}", csr, flags);
        }

        /// Sets floating-point flags.
        pub fn set_fflags(&mut self, flags: u32) {
            self.set_csr_bits(0, flags); // Assuming csr_fflags is 0
        }

        /// Clears floating-point flags.
        pub fn clear_fflags(&mut self, flags: i32) {
            self.clear_csr_bits(0, flags as RegT); // Assuming csr_fflags is 0
        }

        /// Special case of set_register and get_register to access the raw PC value.
        pub fn set_pc(&mut self, value: SRegT) {
            // Implementation goes here
            println!("Setting PC to {}", value);
        }

        /// Gets the raw PC value.
        pub fn get_pc(&self) -> SRegT {
            // Implementation goes here
            println!("Getting PC");
            0
        }
    }
}
