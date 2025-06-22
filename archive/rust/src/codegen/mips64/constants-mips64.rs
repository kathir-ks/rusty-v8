// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::fmt;
use std::mem;

macro_rules! unimplemented_mips {
    () => {
        #[cfg(debug_assertions)]
        {
            println!("{}, line {}: function {} not implemented.", file!(), line!(), function!());
        }
        #[cfg(not(debug_assertions))]
        {}
    };
}

macro_rules! unsupported_mips {
    () => {
        println!("Unsupported instruction.");
    };
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ArchVariants {
    kMips64r2,
    kMips64r6,
}

#[cfg(all(target_arch = "mips64", target_feature = "mips64r2"))]
static kArchVariant: ArchVariants = ArchVariants::kMips64r2;
#[cfg(all(target_arch = "mips64", target_feature = "mips64r6"))]
static kArchVariant: ArchVariants = ArchVariants::kMips64r6;
#[cfg(not(any(all(target_arch = "mips64", target_feature = "mips64r2"), all(target_arch = "mips64", target_feature = "mips64r6"))))]
static kArchVariant: ArchVariants = ArchVariants::kMips64r2;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Endianness {
    kLittle,
    kBig,
}

#[cfg(target_endian = "little")]
static kArchEndian: Endianness = Endianness::kLittle;
#[cfg(target_endian = "big")]
static kArchEndian: Endianness = Endianness::kBig;

// TODO: Determine the correct way to check for soft-float ABI
// For now, assume soft-float ABI unless explicitly specified
#[cfg(feature = "hardfloat")]
const IS_MIPS_SOFT_FLOAT_ABI: bool = false;
#[cfg(not(feature = "hardfloat"))]
const IS_MIPS_SOFT_FLOAT_ABI: bool = true;

#[cfg(target_endian = "little")]
const K_MIPS_LWR_OFFSET: u32 = 0;
#[cfg(target_endian = "little")]
const K_MIPS_LWL_OFFSET: u32 = 3;
#[cfg(target_endian = "little")]
const K_MIPS_SWR_OFFSET: u32 = 0;
#[cfg(target_endian = "little")]
const K_MIPS_SWL_OFFSET: u32 = 3;
#[cfg(target_endian = "little")]
const K_MIPS_LDR_OFFSET: u32 = 0;
#[cfg(target_endian = "little")]
const K_MIPS_LDL_OFFSET: u32 = 7;
#[cfg(target_endian = "little")]
const K_MIPS_SDR_OFFSET: u32 = 0;
#[cfg(target_endian = "little")]
const K_MIPS_SDL_OFFSET: u32 = 7;

#[cfg(target_endian = "big")]
const K_MIPS_LWR_OFFSET: u32 = 3;
#[cfg(target_endian = "big")]
const K_MIPS_LWL_OFFSET: u32 = 0;
#[cfg(target_endian = "big")]
const K_MIPS_SWR_OFFSET: u32 = 3;
#[cfg(target_endian = "big")]
const K_MIPS_SWL_OFFSET: u32 = 0;
#[cfg(target_endian = "big")]
const K_MIPS_LDR_OFFSET: u32 = 7;
#[cfg(target_endian = "big")]
const K_MIPS_LDL_OFFSET: u32 = 0;
#[cfg(target_endian = "big")]
const K_MIPS_SDR_OFFSET: u32 = 7;
#[cfg(target_endian = "big")]
const K_MIPS_SDL_OFFSET: u32 = 0;

#[cfg(target_endian = "little")]
const K_LEAST_SIGNIFICANT_BYTE_IN_INT32_OFFSET: u32 = 0;
#[cfg(target_endian = "little")]
const K_LESS_SIGNIFICANT_WORD_IN_DOUBLEWORD_OFFSET: u32 = 0;

#[cfg(target_endian = "big")]
const K_LEAST_SIGNIFICANT_BYTE_IN_INT32_OFFSET: u32 = 3;
#[cfg(target_endian = "big")]
const K_LESS_SIGNIFICANT_WORD_IN_DOUBLEWORD_OFFSET: u32 = 4;

pub mod v8 {
    pub mod internal {

        // Represents a writable JIT allocation. This is a placeholder.
        pub struct WritableJitAllocation {}

        // Implement Display trait for Opcode
        impl fmt::Display for Opcode {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:?}", self)
            }
        }

        // Implement Display trait for SecondaryField
        impl fmt::Display for SecondaryField {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:?}", self)
            }
        }

        // Implement Display trait for Condition
        impl fmt::Display for Condition {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:?}", self)
            }
        }

        // Implement Display trait for MSABranchCondition
        impl fmt::Display for MSABranchCondition {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:?}", self)
            }
        }

        // Implement Display trait for FPUCondition
        impl fmt::Display for FPUCondition {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:?}", self)
            }
        }

        // Implement Display trait for FPURoundingMode
        impl fmt::Display for FPURoundingMode {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:?}", self)
            }
        }

        // Implement Display trait for Hint
        impl fmt::Display for Hint {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:?}", self)
            }
        }

        /// Maximum range for PC-relative code in MB.
        pub const K_MAX_PC_RELATIVE_CODE_RANGE_IN_MB: usize = 0;

        /// Number of general-purpose registers.
        pub const K_NUM_REGISTERS: i32 = 32;
        pub const K_INVALID_REGISTER: i32 = -1;

        /// Number of registers with HI, LO, and pc.
        pub const K_NUM_SIMU_REGISTERS: i32 = 35;

        /// In the simulator, the PC register is simulated as the 34th register.
        pub const K_PC_REGISTER: i32 = 34;

        /// Number of coprocessor registers.
        pub const K_NUM_FPU_REGISTERS: i32 = 32;
        pub const K_INVALID_FPU_REGISTER: i32 = -1;

        /// Number of MSA registers
        pub const K_NUM_MSA_REGISTERS: i32 = 32;
        pub const K_INVALID_MSA_REGISTER: i32 = -1;

        pub const K_INVALID_MSA_CONTROL_REGISTER: i32 = -1;
        pub const K_MSA_IR_REGISTER: i32 = 0;
        pub const K_MSA_CSR_REGISTER: i32 = 1;
        pub const K_MSA_REG_SIZE: i32 = 128;
        pub const K_MSA_LANES_BYTE: i32 = K_MSA_REG_SIZE / 8;
        pub const K_MSA_LANES_HALF: i32 = K_MSA_REG_SIZE / 16;
        pub const K_MSA_LANES_WORD: i32 = K_MSA_REG_SIZE / 32;
        pub const K_MSA_LANES_DWORD: i32 = K_MSA_REG_SIZE / 64;

        /// FPU (coprocessor 1) control registers. Currently only FCSR is implemented.
        pub const K_FCSR_REGISTER: i32 = 31;
        pub const K_INVALID_FPU_CONTROL_REGISTER: i32 = -1;
        pub const K_FPU_INVALID_RESULT: u32 = (1u32 << 31) - 1;
        pub const K_FPU_INVALID_RESULT_NEGATIVE: i32 = 1i32 << 31;
        pub const K_FPU64_INVALID_RESULT: u64 = (1u64 << 63) - 1;
        pub const K_FPU64_INVALID_RESULT_NEGATIVE: i64 = 1i64 << 63;

        /// FCSR constants.
        pub const K_FCSR_INEXACT_FLAG_BIT: u32 = 2;
        pub const K_FCSR_UNDERFLOW_FLAG_BIT: u32 = 3;
        pub const K_FCSR_OVERFLOW_FLAG_BIT: u32 = 4;
        pub const K_FCSR_DIVIDE_BY_ZERO_FLAG_BIT: u32 = 5;
        pub const K_FCSR_INVALID_OP_FLAG_BIT: u32 = 6;
        pub const K_FCSR_NAN2008_FLAG_BIT: u32 = 18;

        pub const K_FCSR_INEXACT_FLAG_MASK: u32 = 1 << K_FCSR_INEXACT_FLAG_BIT;
        pub const K_FCSR_UNDERFLOW_FLAG_MASK: u32 = 1 << K_FCSR_UNDERFLOW_FLAG_BIT;
        pub const K_FCSR_OVERFLOW_FLAG_MASK: u32 = 1 << K_FCSR_OVERFLOW_FLAG_BIT;
        pub const K_FCSR_DIVIDE_BY_ZERO_FLAG_MASK: u32 = 1 << K_FCSR_DIVIDE_BY_ZERO_FLAG_BIT;
        pub const K_FCSR_INVALID_OP_FLAG_MASK: u32 = 1 << K_FCSR_INVALID_OP_FLAG_BIT;
        pub const K_FCSR_NAN2008_FLAG_MASK: u32 = 1 << K_FCSR_NAN2008_FLAG_BIT;

        pub const K_FCSR_FLAG_MASK: u32 = K_FCSR_INEXACT_FLAG_MASK
            | K_FCSR_UNDERFLOW_FLAG_MASK
            | K_FCSR_OVERFLOW_FLAG_MASK
            | K_FCSR_DIVIDE_BY_ZERO_FLAG_MASK
            | K_FCSR_INVALID_OP_FLAG_MASK;

        pub const K_FCSR_EXCEPTION_FLAG_MASK: u32 = K_FCSR_FLAG_MASK ^ K_FCSR_INEXACT_FLAG_MASK;

        pub const K_FCSR_INEXACT_CAUSE_BIT: u32 = 12;
        pub const K_FCSR_UNDERFLOW_CAUSE_BIT: u32 = 13;
        pub const K_FCSR_OVERFLOW_CAUSE_BIT: u32 = 14;
        pub const K_FCSR_DIVIDE_BY_ZERO_CAUSE_BIT: u32 = 15;
        pub const K_FCSR_INVALID_OP_CAUSE_BIT: u32 = 16;
        pub const K_FCSR_UNIMPLEMENTED_OP_CAUSE_BIT: u32 = 17;

        pub const K_FCSR_INEXACT_CAUSE_MASK: u32 = 1 << K_FCSR_INEXACT_CAUSE_BIT;
        pub const K_FCSR_UNDERFLOW_CAUSE_MASK: u32 = 1 << K_FCSR_UNDERFLOW_CAUSE_BIT;
        pub const K_FCSR_OVERFLOW_CAUSE_MASK: u32 = 1 << K_FCSR_OVERFLOW_CAUSE_BIT;
        pub const K_FCSR_DIVIDE_BY_ZERO_CAUSE_MASK: u32 = 1 << K_FCSR_DIVIDE_BY_ZERO_CAUSE_BIT;
        pub const K_FCSR_INVALID_OP_CAUSE_MASK: u32 = 1 << K_FCSR_INVALID_OP_CAUSE_BIT;
        pub const K_FCSR_UNIMPLEMENTED_OP_CAUSE_MASK: u32 =
            1 << K_FCSR_UNIMPLEMENTED_OP_CAUSE_BIT;

        pub const K_FCSR_CAUSE_MASK: u32 = K_FCSR_INEXACT_CAUSE_MASK
            | K_FCSR_UNDERFLOW_CAUSE_MASK
            | K_FCSR_OVERFLOW_CAUSE_MASK
            | K_FCSR_DIVIDE_BY_ZERO_CAUSE_MASK
            | K_FCSR_INVALID_OP_CAUSE_MASK
            | K_FCSR_UNIMPLEMENTED_OP_CAUSE_BIT;

        /// 'pref' instruction hints
        pub const K_PREF_HINT_LOAD: i32 = 0;
        pub const K_PREF_HINT_STORE: i32 = 1;
        pub const K_PREF_HINT_LOAD_STREAMED: i32 = 4;
        pub const K_PREF_HINT_STORE_STREAMED: i32 = 5;
        pub const K_PREF_HINT_LOAD_RETAINED: i32 = 6;
        pub const K_PREF_HINT_STORE_RETAINED: i32 = 7;
        pub const K_PREF_HINT_WRITEBACK_INVALIDATE: i32 = 25;
        pub const K_PREF_HINT_PREPARE_FOR_STORE: i32 = 30;

        /// Actual value of root register is offset from the root array's start
        /// to take advantage of negative displacement values.
        pub const K_ROOT_REGISTER_BIAS: i32 = 256;

        /// Helper functions for converting between register numbers and names.
        pub struct Registers {}

        impl Registers {
            /// Return the name of the register.
            pub fn name(reg: i32) -> &'static str {
                if reg >= 0 && reg < K_NUM_SIMU_REGISTERS {
                    Self::NAMES[reg as usize]
                } else {
                    "invalid"
                }
            }

            /// Lookup the register number for the name provided.
            pub fn number(name: &str) -> i32 {
                for alias in Self::ALIASES {
                    if alias.name == name {
                        return alias.reg;
                    }
                }
                K_INVALID_REGISTER
            }

            pub const K_MAX_VALUE: i64 = 0x7fffffffffffffffl;
            pub const K_MIN_VALUE: i64 = 0x8000000000000000l;

            const NAMES: [&'static str; K_NUM_SIMU_REGISTERS as usize] = [
                "r0", "r1", "r2", "r3", "r4", "r5", "r6", "r7", "r8", "r9", "r10", "r11", "r12",
                "r13", "r14", "r15", "r16", "r17", "r18", "r19", "r20", "r21", "r22", "r23",
                "r24", "r25", "r26", "r27", "r28", "r29", "r30", "r31", "hi", "lo", "pc",
            ];

            const ALIASES: [RegisterAlias; 33] = [
                RegisterAlias { reg: 0, name: "zero" },
                RegisterAlias { reg: 1, name: "at" },
                RegisterAlias { reg: 2, name: "v0" },
                RegisterAlias { reg: 3, name: "v1" },
                RegisterAlias { reg: 4, name: "a0" },
                RegisterAlias { reg: 5, name: "a1" },
                RegisterAlias { reg: 6, name: "a2" },
                RegisterAlias { reg: 7, name: "a3" },
                RegisterAlias { reg: 8, name: "t0" },
                RegisterAlias { reg: 9, name: "t1" },
                RegisterAlias { reg: 10, name: "t2" },
                RegisterAlias { reg: 11, name: "t3" },
                RegisterAlias { reg: 12, name: "t4" },
                RegisterAlias { reg: 13, name: "t5" },
                RegisterAlias { reg: 14, name: "t6" },
                RegisterAlias { reg: 15, name: "t7" },
                RegisterAlias { reg: 16, name: "s0" },
                RegisterAlias { reg: 17, name: "s1" },
                RegisterAlias { reg: 18, name: "s2" },
                RegisterAlias { reg: 19, name: "s3" },
                RegisterAlias { reg: 20, name: "s4" },
                RegisterAlias { reg: 21, name: "s5" },
                RegisterAlias { reg: 22, name: "s6" },
                RegisterAlias { reg: 23, name: "s7" },
                RegisterAlias { reg: 24, name: "t8" },
                RegisterAlias { reg: 25, name: "t9" },
                RegisterAlias { reg: 26, name: "k0" },
                RegisterAlias { reg: 27, name: "k1" },
                RegisterAlias { reg: 28, name: "gp" },
                RegisterAlias { reg: 29, name: "sp" },
                RegisterAlias { reg: 30, name: "fp" },
                RegisterAlias { reg: 31, name: "ra" },
                RegisterAlias { reg: 30, name: "s8" },
            ];
        }

        #[derive(Copy, Clone)]
        pub struct RegisterAlias {
            pub reg: i32,
            pub name: &'static str,
        }

        /// Helper functions for converting between register numbers and names.
        pub struct FPURegisters {}

        impl FPURegisters {
            /// Return the name of the register.
            pub fn name(reg: i32) -> &'static str {
                if reg >= 0 && reg < K_NUM_FPU_REGISTERS {
                    Self::NAMES[reg as usize]
                } else {
                    "invalid"
                }
            }

            /// Lookup the register number for the name provided.
            pub fn number(name: &str) -> i32 {
                for alias in Self::ALIASES {
                    if alias.name == name {
                        return alias.creg;
                    }
                }
                K_INVALID_FPU_REGISTER
            }

            const NAMES: [&'static str; K_NUM_FPU_REGISTERS as usize] = [
                "f0", "f1", "f2", "f3", "f4", "f5", "f6", "f7", "f8", "f9", "f10", "f11", "f12",
                "f13", "f14", "f15", "f16", "f17", "f18", "f19", "f20", "f21", "f22", "f23",
                "f24", "f25", "f26", "f27", "f28", "f29", "f30", "f31",
            ];

            const ALIASES: [RegisterAlias; 32] = [
                RegisterAlias { creg: 0, name: "f0" },
                RegisterAlias { creg: 1, name: "f1" },
                RegisterAlias { creg: 2, name: "f2" },
                RegisterAlias { creg: 3, name: "f3" },
                RegisterAlias { creg: 4, name: "f4" },
                RegisterAlias { creg: 5, name: "f5" },
                RegisterAlias { creg: 6, name: "f6" },
                RegisterAlias { creg: 7, name: "f7" },
                RegisterAlias { creg: 8, name: "f8" },
                RegisterAlias { creg: 9, name: "f9" },
                RegisterAlias { creg: 10, name: "f10" },
                RegisterAlias { creg: 11, name: "f11" },
                RegisterAlias { creg: 12, name: "f12" },
                RegisterAlias { creg: 13, name: "f13" },
                RegisterAlias { creg: 14, name: "f14" },
                RegisterAlias { creg: 15, name: "f15" },
                RegisterAlias { creg: 16, name: "f16" },
                RegisterAlias { creg: 17, name: "f17" },
                RegisterAlias { creg: 18, name: "f18" },
                RegisterAlias { creg: 19, name: "f19" },
                RegisterAlias { creg: 20, name: "f20" },
                RegisterAlias { creg: 21, name: "f21" },
                RegisterAlias { creg: 22, name: "f22" },
                RegisterAlias { creg: 23, name: "f23" },
                RegisterAlias { creg: 24, name: "f24" },
                RegisterAlias { creg: 25, name: "f25" },
                RegisterAlias { creg: 26, name: "f26" },
                RegisterAlias { creg: 27, name: "f27" },
                RegisterAlias { creg: 28, name: "f28" },
                RegisterAlias { creg: 29, name: "f29" },
                RegisterAlias { creg: 30, name: "f30" },
                RegisterAlias { creg: 31, name: "f31" },
            ];
        }

        /// Helper functions for converting between register numbers and names.
        pub struct MSARegisters {}

        impl MSARegisters {
            /// Return the name of the register.
            pub fn name(reg: i32) -> &'static str {
                if reg >= 0 && reg < K_NUM_MSA_REGISTERS {
                    Self::NAMES[reg as usize]
                } else {
                    "invalid"
                }
            }

            /// Lookup the register number for the name provided.
            pub fn number(name: &str) -> i32 {
                for alias in Self::ALIASES {
                    if alias.name == name {
                        return alias.creg;
                    }
                }
                K_INVALID_MSA_REGISTER
            }

            const NAMES: [&'static str; K_NUM_MSA_REGISTERS as usize] = [
                "w0", "w1", "w2", "w3", "w4", "w5", "w6", "w7", "w8", "w9", "w10", "w11", "w12",
                "w13", "w14", "w15", "w16", "w17", "w18", "w19", "w20", "w21", "w22", "w23",
                "w24", "w25", "w26", "w27", "w28", "w29", "w30", "w31",
            ];

            const ALIASES: [RegisterAlias; 32] = [
                RegisterAlias { creg: 0, name: "w0" },
                RegisterAlias { creg: 1, name: "w1" },
                RegisterAlias { creg: 2, name: "w2" },
                RegisterAlias { creg: 3, name: "w3" },
                RegisterAlias { creg: 4, name: "w4" },
                RegisterAlias { creg: 5, name: "w5" },
                RegisterAlias { creg: 6, name: "w6" },
                RegisterAlias { creg: 7, name: "w7" },
                RegisterAlias { creg: 8, name: "w8" },
                RegisterAlias { creg: 9, name: "w9" },
                RegisterAlias { creg: 10, name: "w10" },
                RegisterAlias { creg: 11, name: "w11" },
                RegisterAlias { creg: 12, name: "w12" },
                RegisterAlias { creg: 13, name: "w13" },
                RegisterAlias { creg: 14, name: "w14" },
                RegisterAlias { creg: 15, name: "w15" },
                RegisterAlias { creg: 16, name: "w16" },
                RegisterAlias { creg: 17, name: "w17" },
                RegisterAlias { creg: 18, name: "w18" },
                RegisterAlias { creg: 19, name: "w19" },
                RegisterAlias { creg: 20, name: "w20" },
                RegisterAlias { creg: 21, name: "w21" },
                RegisterAlias { creg: 22, name: "w22" },
                RegisterAlias { creg: 23, name: "w23" },
                RegisterAlias { creg: 24, name: "w24" },
                RegisterAlias { creg: 25, name: "w25" },
                RegisterAlias { creg: 26, name: "w26" },
                RegisterAlias { creg: 27, name: "w27" },
                RegisterAlias { creg: 28, name: "w28" },
                RegisterAlias { creg: 29, name: "w29" },
                RegisterAlias { creg: 30, name: "w30" },
                RegisterAlias { creg: 31, name: "w31" },
            ];
        }

        #[derive(Copy, Clone)]
        pub struct RegisterAlias {
            pub creg: i32,
            pub name: &'static str,
        }

        /// MSA sizes.
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum MSASize {
            MSA_B = 0x0,
            MSA_H = 0x1,
            MSA_W = 0x2,
            MSA_D = 0x3,
        }

        /// MSA data type, top bit set for unsigned data types.
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum MSADataType {
            MSAS8 = 0,
            MSAS16 = 1,
            MSAS32 = 2,
            MSAS64 = 3,
            MSAU8 = 4,
            MSAU16 = 5,
            MSAU32 = 6,
            MSAU64 = 7,
        }

        /// On MIPS all instructions are 32 bits.
        pub type Instr = i32;

        /// Special Software Interrupt codes when used in the presence of the MIPS
        /// simulator.
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum SoftwareInterruptCodes {
            /// Transition to C code.
            call_rt_redirected = 0xfffff,
        }

        /// On MIPS Simulator breakpoints can have different codes:
        /// - Breaks between 0 and K_MAX_WATCHPOINT_CODE are treated as simple watchpoints,
        ///   the simulator will run through them and print the registers.
        /// - Breaks between K_MAX_WATCHPOINT_CODE and K_MAX_STOP_CODE are treated as stop()
        ///   instructions (see Assembler::stop()).
        /// - Breaks larger than K_MAX_STOP_CODE are simple breaks, dropping you into the
        ///   debugger.
        pub const K_MAX_WATCHPOINT_CODE: u32 = 31;
        pub const K_MAX_STOP_CODE: u32 = 127;

        /// ----- Fields offset and length.
        pub const K_OPCODE_SHIFT: i32 = 26;
        pub const K_OPCODE_BITS: i32 = 6;
        pub const K_RS_SHIFT: i32 = 21;
        pub const K_RS_BITS: i32 = 5;
        pub const K_RT_SHIFT: i32 = 16;
        pub const K_RT_BITS: i32 = 5;
        pub const K_RD_SHIFT: i32 = 11;
        pub const K_RD_BITS: i32 = 5;
        pub const K_SA_SHIFT: i32 = 6;
        pub const K_SA_BITS: i32 = 5;
        pub const K_LSA_SA_BITS: i32 = 2;
        pub const K_FUNCTION_SHIFT: i32 = 0;
        pub const K_FUNCTION_BITS: i32 = 6;
        pub const K_LUI_SHIFT: i32 = 16;
        pub const K_BP2_SHIFT: i32 = 6;
        pub const K_BP2_BITS: i32 = 2;
        pub const K_BP3_SHIFT: i32 = 6;
        pub const K_BP3_BITS: i32 = 3;
        pub const K_BASE_SHIFT: i32 = 21;
        pub const K_BASE_BITS: i32 = 5;
        pub const K_BIT6_SHIFT: i32 = 6;
        pub const K_BIT6_BITS: i32 = 1;

        pub const K_IMM9_SHIFT: i32 = 7;
        pub const K_IMM9_BITS: i32 = 9;
        pub const K_IMM16_SHIFT: i32 = 0;
        pub const K_IMM16_BITS: i32 = 16;
        pub const K_IMM18_SHIFT: i32 = 0;
        pub const K_IMM18_BITS: i32 = 18;
        pub const K_IMM19_SHIFT: i32 = 0;
        pub const K_IMM19_BITS: i32 = 19;
        pub const K_IMM21_SHIFT: i32 = 0;
        pub const K_IMM21_BITS: i32 = 21;
        pub const K_IMM26_SHIFT: i32 = 0;
        pub const K_IMM26_BITS: i32 = 26;
        pub const K_IMM28_SHIFT: i32 = 0;
        pub const K_IMM28_BITS: i32 = 28;
        pub const K_IMM32_SHIFT: i32 = 0;
        pub const K_IMM32_BITS: i32 = 32;
        pub const K_MSA_IMM8_SHIFT: i32 = 16;
        pub const K_MSA_IMM8_BITS: i32 = 8;
        pub const K_MSA_IMM5_SHIFT: i32 = 16;
        pub const K_MSA_IMM5_BITS: i32 = 5;
        pub const K_MSA_IMM10_SHIFT: i32 = 11;
        pub const K_MSA_IMM10_BITS: i32 = 10;
        pub const K_MSA_IMM_MI10_SHIFT: i32 = 16;
        pub const K_MSA_IMM_MI10_BITS: i32 = 10;

        /// In branches and jumps immediate fields point to words, not bytes,
        /// and are therefore shifted by 2.
        pub const K_IMM_FIELD_SHIFT: i32 = 2;

        pub const K_FR_BITS: i32 = 5;
        pub const K_FR_SHIFT: i32 = 21;
        pub const K_FS_SHIFT: i32 = 11;
        pub const K_FS_BITS: i32 = 5;
        pub const K_FT_SHIFT: i32 = 16;
        pub const K_FT_BITS: i32 = 5;
        pub const K_FD_SHIFT: i32 = 6;
        pub const K_FD_BITS: i32 = 5;
        pub const K_FCC_SHIFT: i32 = 8;
        pub const K_FCC_BITS: i3