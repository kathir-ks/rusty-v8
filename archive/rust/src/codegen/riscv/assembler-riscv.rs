// Copyright (c) 1994-2006 Sun Microsystems Inc.
// All Rights Reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
// - Redistributions of source code must retain the above copyright notice,
// this list of conditions and the following disclaimer.
//
// - Redistribution in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
//
// - Neither the name of Sun Microsystems or the names of contributors may
// be used to endorse or promote products derived from this software without
// specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
// IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
// PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

// The original source code covered by the above license above has been
// modified significantly by Google Inc.
// Copyright 2021 the V8 project authors. All rights reserved.

// Note: This is a partial translation. Many parts of the C++ code rely on
// V8-specific data structures and functionalities, which are not fully
// replicable in a straightforward manner in Rust.  This translation focuses
// on the core logic and data structures where possible.

mod base;
mod codegen;
mod common;
mod deoptimizer;
mod diagnostics;
mod objects;

use std::mem;

//use crate::base::bits;
//use crate::base::cpu;
//use crate::codegen::assembler;
//use crate::codegen::safepoint_table;
//use crate::common::code_memory_access;
//use crate::deoptimizer::deoptimizer;
//use crate::diagnostics::disasm;
//use crate::diagnostics::disassembler;
//use crate::objects::heap_number;

//#[allow(dead_code)]
//#[derive(Debug, Copy, Clone, PartialEq, Eq)]
//pub enum Register {
//    Zero,
//    Ra,
//    Sp,
//    Gp,
//    Tp,
//    T0,
//    T1,
//    T2,
//    S0, // Fp
//    S1,
//    A0,
//    A1,
//    A2,
//    A3,
//    A4,
//    A5,
//    A6,
//    A7,
//    S2,
//    S3,
//    S4,
//    S5,
//    S6,
//    S7,
//    S8,
//    S9,
//    S10,
//    S11,
//    T3,
//    T4,
//    T5,
//    T6,
//}
//
//impl Register {
//    pub fn code(&self) -> usize {
//        match self {
//            Register::Zero => 0,
//            Register::Ra => 1,
//            Register::Sp => 2,
//            Register::Gp => 3,
//            Register::Tp => 4,
//            Register::T0 => 5,
//            Register::T1 => 6,
//            Register::T2 => 7,
//            Register::S0 => 8,
//            Register::S1 => 9,
//            Register::A0 => 10,
//            Register::A1 => 11,
//            Register::A2 => 12,
//            Register::A3 => 13,
//            Register::A4 => 14,
//            Register::A5 => 15,
//            Register::A6 => 16,
//            Register::A7 => 17,
//            Register::S2 => 18,
//            Register::S3 => 19,
//            Register::S4 => 20,
//            Register::S5 => 21,
//            Register::S6 => 22,
//            Register::S7 => 23,
//            Register::S8 => 24,
//            Register::S9 => 25,
//            Register::S10 => 26,
//            Register::S11 => 27,
//            Register::T3 => 28,
//            Register::T4 => 29,
//            Register::T5 => 30,
//            Register::T6 => 31,
//        }
//    }
//
//    pub fn is_valid(&self) -> bool {
//        true // Always valid in this simplified version
//    }
//}
//
//pub const zero_reg: Register = Register::Zero;
//pub const ra: Register = Register::Ra;
//pub const sp: Register = Register::Sp;
//pub const gp: Register = Register::Gp;
//pub const tp: Register = Register::Tp;
//pub const t0: Register = Register::T0;
//pub const t1: Register = Register::T1;
//pub const t2: Register = Register::T2;
//pub const fp: Register = Register::S0;
//pub const s1: Register = Register::S1;
//pub const a0: Register = Register::A0;
//pub const a1: Register = Register::A1;
//pub const a2: Register = Register::A2;
//pub const a3: Register = Register::A3;
//pub const a4: Register = Register::A4;
//pub const a5: Register = Register::A5;
//pub const a6: Register = Register::A6;
//pub const a7: Register = Register::A7;
//pub const s2: Register = Register::S2;
//pub const s3: Register = Register::S3;
//pub const s4: Register = Register::S4;
//pub const s5: Register = Register::S5;
//pub const s6: Register = Register::S6;
//pub const s7: Register = Register::S7;
//pub const s8: Register = Register::S8;
//pub const s9: Register = Register::S9;
//pub const s10: Register = Register::S10;
//pub const s11: Register = Register::S11;
//pub const t3: Register = Register::T3;
//pub const t4: Register = Register::T4;
//pub const t5: Register = Register::T5;
//pub const t6: Register = Register::T6;
//
//pub const kNumRegisters: usize = 32;
//
//
//#[derive(Debug, Copy, Clone, PartialEq, Eq)]
//pub enum FRegister {
//  F0,
//  F1,
//  F2,
//  F3,
//  F4,
//  F5,
//  F6,
//  F7,
//  F8,
//  F9,
//  F10,
//  F11,
//  F12,
//  F13,
//  F14,
//  F15,
//  F16,
//  F17,
//  F18,
//  F19,
//  F20,
//  F21,
//  F22,
//  F23,
//  F24,
//  F25,
//  F26,
//  F27,
//  F28,
//  F29,
//  F30,
//  F31,
//}
//
//pub const ft0: FRegister = FRegister::F0;
//pub const ft1: FRegister = FRegister::F1;
//pub const ft2: FRegister = FRegister::F2;
//pub const ft3: FRegister = FRegister::F3;
//pub const ft4: FRegister = FRegister::F4;
//pub const ft5: FRegister = FRegister::F5;
//pub const ft6: FRegister = FRegister::F6;
//pub const ft7: FRegister = FRegister::F7;
//pub const fs0: FRegister = FRegister::F8;
//pub const fs1: FRegister = FRegister::F9;
//pub const fa0: FRegister = FRegister::F10;
//pub const fa1: FRegister = FRegister::F11;
//pub const fa2: FRegister = FRegister::F12;
//pub const fa3: FRegister = FRegister::F13;
//pub const fa4: FRegister = FRegister::F14;
//pub const fa5: FRegister = FRegister::F15;
//pub const fa6: FRegister = FRegister::F16;
//pub const fa7: FRegister = FRegister::F17;
//pub const fs2: FRegister = FRegister::F18;
//pub const fs3: FRegister = FRegister::F19;
//pub const fs4: FRegister = FRegister::F20;
//pub const fs5: FRegister = FRegister::F21;
//pub const fs6: FRegister = FRegister::F22;
//pub const fs7: FRegister = FRegister::F23;
//pub const fs8: FRegister = FRegister::F24;
//pub const fs9: FRegister = FRegister::F25;
//pub const fs10: FRegister = FRegister::F26;
//pub const fs11: FRegister = FRegister::F27;
//pub const ft8: FRegister = FRegister::F28;
//pub const ft9: FRegister = FRegister::F29;
//pub const ft10: FRegister = FRegister::F30;
//pub const ft11: FRegister = FRegister::F31;
//
//
//#[derive(Debug, Copy, Clone, PartialEq, Eq)]
//pub enum VRegister {
//  V0,
//  V1,
//  V2,
//  V3,
//  V4,
//  V5,
//  V6,
//  V7,
//  V8,
//  V9,
//  V10,
//  V11,
//  V12,
//  V13,
//  V14,
//  V15,
//  V16,
//  V17,
//  V18,
//  V19,
//  V20,
//  V21,
//  V22,
//  V23,
//  V24,
//  V25,
//  V26,
//  V27,
//  V28,
//  V29,
//  V30,
//  V31,
//}
//
//pub const vt0: VRegister = VRegister::V0;
//pub const vt1: VRegister = VRegister::V1;
//pub const vt2: VRegister = VRegister::V2;
//pub const vt3: VRegister = VRegister::V3;
//pub const vt4: VRegister = VRegister::V4;
//pub const vt5: VRegister = VRegister::V5;
//pub const vt6: VRegister = VRegister::V6;
//pub const vt7: VRegister = VRegister::V7;
//pub const vs2: VRegister = VRegister::V8;
//pub const vs3: VRegister = VRegister::V9;
//pub const vs4: VRegister = VRegister::V10;
//pub const vs5: VRegister = VRegister::V11;
//pub const vs6: VRegister = VRegister::V12;
//pub const vs7: VRegister = VRegister::V13;
//pub const vs8: VRegister = VRegister::V14;
//pub const vs9: VRegister = VRegister::V15;
//pub const vs10: VRegister = VRegister::V16;
//pub const vs11: VRegister = VRegister::V17;
//pub const vt8: VRegister = VRegister::V18;
//pub const vt9: VRegister = VRegister::V19;
//pub const vt10: VRegister = VRegister::V20;
//pub const vt11: VRegister = VRegister::V21;
//pub const va0: VRegister = VRegister::V22;
//pub const va1: VRegister = VRegister::V23;
//pub const va2: VRegister = VRegister::V24;
//pub const va3: VRegister = VRegister::V25;
//pub const va4: VRegister = VRegister::V26;
//pub const va5: VRegister = VRegister::V27;
//pub const va6: VRegister = VRegister::V28;
//pub const va7: VRegister = VRegister::V29;
//pub const vsp: VRegister = VRegister::V30;
//pub const vzero: VRegister = VRegister::V31;
//
//
//#[derive(Debug, Copy, Clone, PartialEq, Eq)]
//pub enum Extension {
//  None,
//  M,
//  A,
//  F,
//  D,
//  C,
//  Zba,
//  Zbb,
//  Zbs,
//  Zicond,
//  Svnapot,
//  Sfencei,
//}

pub mod riscv {
    use std::{fmt, mem};
    use std::fmt::Formatter;

    //use crate::base::bits;
    //use crate::base::cpu;
    //use crate::codegen::assembler;
    //use crate::codegen::safepoint_table;
    //use crate::common::code_memory_access;
    //use crate::deoptimizer::deoptimizer;
    //use crate::diagnostics::disasm;
    //use crate::diagnostics::disassembler;
    //use crate::objects::heap_number;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Register {
        Zero,
        Ra,
        Sp,
        Gp,
        Tp,
        T0,
        T1,
        T2,
        S0, // Fp
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
    }

    impl Register {
        pub fn code(&self) -> usize {
            match self {
                Register::Zero => 0,
                Register::Ra => 1,
                Register::Sp => 2,
                Register::Gp => 3,
                Register::Tp => 4,
                Register::T0 => 5,
                Register::T1 => 6,
                Register::T2 => 7,
                Register::S0 => 8,
                Register::S1 => 9,
                Register::A0 => 10,
                Register::A1 => 11,
                Register::A2 => 12,
                Register::A3 => 13,
                Register::A4 => 14,
                Register::A5 => 15,
                Register::A6 => 16,
                Register::A7 => 17,
                Register::S2 => 18,
                Register::S3 => 19,
                Register::S4 => 20,
                Register::S5 => 21,
                Register::S6 => 22,
                Register::S7 => 23,
                Register::S8 => 24,
                Register::S9 => 25,
                Register::S10 => 26,
                Register::S11 => 27,
                Register::T3 => 28,
                Register::T4 => 29,
                Register::T5 => 30,
                Register::T6 => 31,
            }
        }

        pub fn is_valid(&self) -> bool {
            true // Always valid in this simplified version
        }
    }

    impl fmt::Display for Register {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    pub const zero_reg: Register = Register::Zero;
    pub const ra: Register = Register::Ra;
    pub const sp: Register = Register::Sp;
    pub const gp: Register = Register::Gp;
    pub const tp: Register = Register::Tp;
    pub const t0: Register = Register::T0;
    pub const t1: Register = Register::T1;
    pub const t2: Register = Register::T2;
    pub const fp: Register = Register::S0;
    pub const s1: Register = Register::S1;
    pub const a0: Register = Register::A0;
    pub const a1: Register = Register::A1;
    pub const a2: Register = Register::A2;
    pub const a3: Register = Register::A3;
    pub const a4: Register = Register::A4;
    pub const a5: Register = Register::A5;
    pub const a6: Register = Register::A6;
    pub const a7: Register = Register::A7;
    pub const s2: Register = Register::S2;
    pub const s3: Register = Register::S3;
    pub const s4: Register = Register::S4;
    pub const s5: Register = Register::S5;
    pub const s6: Register = Register::S6;
    pub const s7: Register = Register::S7;
    pub const s8: Register = Register::S8;
    pub const s9: Register = Register::S9;
    pub const s10: Register = Register::S10;
    pub const s11: Register = Register::S11;
    pub const t3: Register = Register::T3;
    pub const t4: Register = Register::T4;
    pub const t5: Register = Register::T5;
    pub const t6: Register = Register::T6;

    pub const K_NUM_REGISTERS: usize = 32;


    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum FRegister {
        F0,
        F1,
        F2,
        F3,
        F4,
        F5,
        F6,
        F7,
        F8,
        F9,
        F10,
        F11,
        F12,
        F13,
        F14,
        F15,
        F16,
        F17,
        F18,
        F19,
        F20,
        F21,
        F22,
        F23,
        F24,
        F25,
        F26,
        F27,
        F28,
        F29,
        F30,
        F31,
    }

    impl fmt::Display for FRegister {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    pub const ft0: FRegister = FRegister::F0;
    pub const ft1: FRegister = FRegister::F1;
    pub const ft2: FRegister = FRegister::F2;
    pub const ft3: FRegister = FRegister::F3;
    pub const ft4: FRegister = FRegister::F4;
    pub const ft5: FRegister = FRegister::F5;
    pub const ft6: FRegister = FRegister::F6;
    pub const ft7: FRegister = FRegister::F7;
    pub const fs0: FRegister = FRegister::F8;
    pub const fs1: FRegister = FRegister::F9;
    pub const fa0: FRegister = FRegister::F10;
    pub const fa1: FRegister = FRegister::F11;
    pub const fa2: FRegister = FRegister::F12;
    pub const fa3: FRegister = FRegister::F13;
    pub const fa4: FRegister = FRegister::F14;
    pub const fa5: FRegister = FRegister::F15;
    pub const fa6: FRegister = FRegister::F16;
    pub const fa7: FRegister = FRegister::F17;
    pub const fs2: FRegister = FRegister::F18;
    pub const fs3: FRegister = FRegister::F19;
    pub const fs4: FRegister = FRegister::F20;
    pub const fs5: FRegister = FRegister::F21;
    pub const fs6: FRegister = FRegister::F22;
    pub const fs7: FRegister = FRegister::F23;
    pub const fs8: FRegister = FRegister::F24;
    pub const fs9: FRegister = FRegister::F25;
    pub const fs10: FRegister = FRegister::F26;
    pub const fs11: FRegister = FRegister::F27;
    pub const ft8: FRegister = FRegister::F28;
    pub const ft9: FRegister = FRegister::F29;
    pub const ft10: FRegister = FRegister::F30;
    pub const ft11: FRegister = FRegister::F31;


    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
    }

    impl fmt::Display for VRegister {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    pub const vt0: VRegister = VRegister::V0;
    pub const vt1: VRegister = VRegister::V1;
    pub const vt2: VRegister = VRegister::V2;
    pub const vt3: VRegister = VRegister::V3;
    pub const vt4: VRegister = VRegister::V4;
    pub const vt5: VRegister = VRegister::V5;
    pub const vt6: VRegister = VRegister::V6;
    pub const vt7: VRegister = VRegister::V7;
    pub const vs2: VRegister = VRegister::V8;
    pub const vs3: VRegister = VRegister::V9;
    pub const vs4: VRegister = VRegister::V10;
    pub const vs5: VRegister = VRegister::V11;
    pub const vs6: VRegister = VRegister::V12;
    pub const vs7: VRegister = VRegister::V13;
    pub const vs8: VRegister = VRegister::V14;
    pub const vs9: VRegister = VRegister::V15;
    pub const vs10: VRegister = VRegister::V16;
    pub const vs11: VRegister = VRegister::V17;
    pub const vt8: VRegister = VRegister::V18;
    pub const vt9: VRegister = VRegister::V19;
    pub const vt10: VRegister = VRegister::V20;
    pub const vt11: VRegister = VRegister::V21;
    pub const va0: VRegister = VRegister::V22;
    pub const va1: VRegister = VRegister::V23;
    pub const va2: VRegister = VRegister::V24;
    pub const va3: VRegister = VRegister::V25;
    pub const va4: VRegister = VRegister::V26;
    pub const va5: VRegister = VRegister::V27;
    pub const va6: VRegister = VRegister::V28;
    pub const va7: VRegister = VRegister::V29;
    pub const vsp: VRegister = VRegister::V30;
    pub const vzero: VRegister = VRegister::V31;


    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Extension {
        None,
        M,
        A,
        F,
        D,
        C,
        Zba,
        Zbb,
        Zbs,
        Zicond,
        Svnapot,
        Sfencei,
    }


    /// CPU Features
    #[derive(Debug, Default)]
    pub struct CpuFeatures {
        supported_: u32,
        supports_wasm_simd_128_: bool,
    }

    impl CpuFeatures {
        pub fn new() -> Self {
            CpuFeatures {
                supported_: 0,
                supports_wasm_simd_128_: false,
            }
        }

        pub fn supports_wasm_simd_128(&self) -> bool {
            self.is_supported(CpuFeature::RISCV_SIMD)
        }

        pub fn is_supported(&self, feature: CpuFeature) -> bool {
            (self.supported_ & (1 << feature as u32)) != 0
        }

        // TODO: Implement ProbeImpl, PrintTarget, PrintFeatures based on Rust's
        // CPU feature detection crates.
        pub fn probe_impl(&mut self, cross_compile: bool) {
            self.supported_ |= Self::cpu_features_implied_by_compiler();

            #[cfg(target_arch = "riscv64")]
            {
                self.supported_ |= Self::simulator_features();
            }

            if cross_compile {
                return;
            }

            // TODO: Add runtime probing with a suitable Rust crate for CPU
            // feature detection, e.g., "cpuid".
            // For now, we leave this unimplemented.

            self.supports_wasm_simd_128_ = self.supports_wasm_simd_128();
        }

        fn cpu_features_implied_by_compiler() -> u32 {
            let mut answer = 0;
            #[cfg(all(riscv_f, riscv_d))]
            {
                answer |= 1 << CpuFeature::FPU as u32;
            }
            #[cfg(all(riscv_vector, riscv_v >= 1000000))]
            {
                answer |= 1 << CpuFeature::RISCV_SIMD as u32;
            }
            #[cfg(riscv_zba)]
            {
                answer |= 1 << CpuFeature::ZBA as u32;
            }
            #[cfg(riscv_zbb)]
            {
                answer |= 1 << CpuFeature::ZBB as u32;
            }
            #[cfg(riscv_zbs)]
            {
                answer |= 1 << CpuFeature::ZBS as u32;
            }
            #[cfg(_riscv_zicond)]
            {
                answer |= 1 << CpuFeature::ZICOND as u32;
            }
            answer
        }

        #[cfg(target_arch = "riscv64")]
        fn simulator_features() -> u32 {
            let mut answer = 0;
            answer |= 1 << CpuFeature::RISCV_SIMD as u32;
            answer |= 1 << CpuFeature::ZBA as u32;
            answer |= 1 << CpuFeature::ZBB as u32;
            answer |= 1 << CpuFeature::ZBS as u32;
            answer |= 1 << CpuFeature::ZICOND as u32;
            answer |= 1 << CpuFeature::FPU as u32;
            answer
        }

        pub fn print_target(&self) {}
        pub fn print_features(&self) {
            println!(
                "supports_wasm_simd_128={}",
                self.supports_wasm_simd_128()
            );
            println!(
                "RISC-V Extension zba={},zbb={},zbs={},ZICOND={}",
                self.is_supported(CpuFeature::ZBA),
                self.is_supported(CpuFeature::ZBB),
                self.is_supported(CpuFeature::ZBS),
                self.is_supported(CpuFeature::ZICOND)
            );
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive, ToPrimitive)]
    pub enum CpuFeature {
        FPU,
        RISCV_SIMD,
        ZBA,
        ZBB,
        ZBS,
        ZICOND,
    }
    /// Converts a Register enum to a number.
    pub fn register_to_number(reg: Register) -> i32 {
        match reg {
            Register::Zero => 0,
            Register::Ra => 1,
            Register::Sp => 2,
            Register::Gp => 3,
            Register::Tp => 4,
            Register::T0 => 5,
            Register::T1 => 6,
            Register::T2 => 7,
            Register::S0 => 8,
            Register::S1 => 9,
            Register::A0 => 10,
            Register::A1 => 11,
            Register::A2 => 12,
            Register::A3 => 13,
            Register::A4 => 14,
            Register::A5 => 15,
            Register::A6 => 16,
            Register::A7 => 17,
            Register::S2 => 18,
            Register::S3 => 19,
            Register::S4 => 20,
            Register::S5 => 21,
            Register::S6 => 22,
            Register::S7 => 23,
            Register::S8 => 24,
            Register::S9 => 25,
            Register::S10 => 26,
            Register::S11 => 27,
            Register::T3 => 28,
            Register::T4 => 29,
            Register::T5 => 30,
            Register::T6 => 31,
        } as i32
    }

    /// Converts a number to a Register enum.
    pub fn number_to_register(num: i32) -> Register {
        match num {
            0 => Register::Zero,
            1 => Register::Ra,
            2 => Register::Sp,
            3 => Register::Gp,
            4 => Register::Tp,
            5 => Register::T0,
            6 => Register::T1,
            7 => Register::T2,
            8 => Register::S0,
            9 => Register::S1,
            10 => Register::A0,
            11 => Register::A1,
            12 => Register::A2,
            13 => Register::A3,
            14 => Register::A4,
            15 => Register::A5,
            16 => Register::A6,
            17 => Register::A7,
            18 => Register::S2,
            19 => Register::S3,
            20 => Register::S4,
            21 => Register::S5,
            22 => Register::S6,
            23 => Register::S7,
            24 => Register::S8,
            25 => Register::S9,
            26 => Register::S10,
            27 => Register::S11,
            28 => Register::T3,
            29 => Register::T4,
            30 => Register::T5,
            31 => Register::T6,
            _ => Register::Zero, // Or handle the error case appropriately
        }
    }

    /// Relocation information modes.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum RelocMode {
        NoInfo,
        InternalReference,
        InternalReferenceEncoded,
        NearBuiltinEntry,
        RelativeCodeTarget,
        WasmCall,
        WasmStubCall,
        EmbeddedObject,
        FullEmbeddedObject,
        ConstPool,
    }

    impl RelocMode {
        pub fn mode_mask(mode: RelocMode) -> i32 {
            match mode {
                RelocMode::InternalReference => 1 << 0,
                RelocMode::InternalReferenceEncoded => 1 << 1,
                RelocMode::NearBuiltinEntry => 1 << 2,
                RelocMode::RelativeCodeTarget => 1 << 3,
                _ => 0, // Other modes are not masked in the original code
            }
        }

        pub fn is_embedded_object_mode(