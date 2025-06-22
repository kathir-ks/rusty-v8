// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file defines architecture-specific register lists and constants for the
// Liftoff assembler in the V8 WebAssembly engine.

// Note: The `codegen` and `reglist` modules would ideally be defined elsewhere
// to mirror the C++ project structure. For simplicity, they are included here.

mod codegen {
    pub type Register = u32; // Placeholder.  Should be architecture-specific.
    pub type DoubleRegister = u32; // Placeholder. Should be architecture-specific.

    //Placeholder for RootRegister
    pub const kRootRegister: Register = 1000;
    pub const kWasmImplicitArgRegister: Register = 1001;

    #[cfg(feature = "compress_pointers")]
    pub const kPtrComprCageBaseRegister: Register = 1002;

}

mod reglist {
    use std::collections::HashSet;

    use crate::codegen::{Register, DoubleRegister};

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct RegList(HashSet<Register>);

    impl RegList {
        pub fn new() -> Self {
            RegList(HashSet::new())
        }

        pub fn from_bits(bits: u32) -> Self {
            // Placeholder.  Needs proper implementation for bitmask -> register list conversion
            // based on architecture.  This is a dummy implementation.
            let mut reg_list = RegList::new();
            for i in 0..32 {
                if (bits >> i) & 1 != 0 {
                    reg_list.0.insert(i as Register); // Assuming registers are numbered 0-31
                }
            }
            reg_list
        }

        pub fn contains(&self, reg: Register) -> bool {
            self.0.contains(&reg)
        }

        pub fn insert(&mut self, reg: Register) {
            self.0.insert(reg);
        }
    }

    impl Default for RegList {
        fn default() -> Self {
            Self::new()
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct DoubleRegList(HashSet<DoubleRegister>);

    impl DoubleRegList {
        pub fn new() -> Self {
            DoubleRegList(HashSet::new())
        }

        pub fn from_bits(bits: u32) -> Self {
            // Placeholder.  Needs proper implementation for bitmask -> register list conversion
            // based on architecture.  This is a dummy implementation.
            let mut reg_list = DoubleRegList::new();
            for i in 0..32 {
                if (bits >> i) & 1 != 0 {
                    reg_list.0.insert(i as DoubleRegister); // Assuming registers are numbered 0-31
                }
            }
            reg_list
        }

        pub fn contains(&self, reg: DoubleRegister) -> bool {
            self.0.contains(&reg)
        }

        pub fn insert(&mut self, reg: DoubleRegister) {
            self.0.insert(reg);
        }
    }

    impl Default for DoubleRegList {
        fn default() -> Self {
            Self::new()
        }
    }
}

pub mod wasm {
    use crate::codegen::{Register, DoubleRegister, kWasmImplicitArgRegister, kRootRegister};
    use crate::reglist::{RegList, DoubleRegList};

    #[cfg(all(target_arch = "x86", target_pointer_width = "32"))]
    pub mod arch_ia32 {
        use super::*;

        pub const eax: Register = 0;
        pub const ecx: Register = 1;
        pub const edx: Register = 2;
        pub const esi: Register = 3;
        pub const edi: Register = 4;

        pub const xmm0: DoubleRegister = 0;
        pub const xmm1: DoubleRegister = 1;
        pub const xmm2: DoubleRegister = 2;
        pub const xmm3: DoubleRegister = 3;
        pub const xmm4: DoubleRegister = 4;
        pub const xmm5: DoubleRegister = 5;
        pub const xmm6: DoubleRegister = 6;
        pub const xmm7: DoubleRegister = 7;


        pub const K_LIFTOFF_ASSEMBLER_GP_CACHE_REGS: RegList = {
            let mut reg_list = RegList::new();
            reg_list.insert(eax);
            reg_list.insert(ecx);
            reg_list.insert(edx);
            reg_list.insert(esi);
            reg_list.insert(edi);
            reg_list
        };

        pub const K_LIFTOFF_ASSEMBLER_FP_CACHE_REGS: DoubleRegList = {
            let mut reg_list = DoubleRegList::new();
            reg_list.insert(xmm1);
            reg_list.insert(xmm2);
            reg_list.insert(xmm3);
            reg_list.insert(xmm4);
            reg_list.insert(xmm5);
            reg_list.insert(xmm6);
            reg_list
        };

        pub const K_LIFTOFF_FRAME_SETUP_FUNCTION_REG: Register = edi;
    }

    #[cfg(all(target_arch = "x86_64", target_pointer_width = "64"))]
    pub mod arch_x64 {
        use super::*;

        pub const rax: Register = 0;
        pub const rcx: Register = 1;
        pub const rdx: Register = 2;
        pub const rbx: Register = 3;
        pub const rsi: Register = 4;
        pub const rdi: Register = 5;
        pub const r8: Register = 8;
        pub const r9: Register = 9;
        pub const r12: Register = 12;
        pub const r15: Register = 15;

        pub const xmm0: DoubleRegister = 0;
        pub const xmm1: DoubleRegister = 1;
        pub const xmm2: DoubleRegister = 2;
        pub const xmm3: DoubleRegister = 3;
        pub const xmm4: DoubleRegister = 4;
        pub const xmm5: DoubleRegister = 5;
        pub const xmm6: DoubleRegister = 6;
        pub const xmm7: DoubleRegister = 7;

        pub const K_LIFTOFF_ASSEMBLER_GP_CACHE_REGS: RegList = {
            let mut reg_list = RegList::new();
            reg_list.insert(rax);
            reg_list.insert(rcx);
            reg_list.insert(rdx);
            reg_list.insert(rbx);
            reg_list.insert(rsi);
            reg_list.insert(rdi);
            reg_list.insert(r8);
            reg_list.insert(r9);
            reg_list.insert(r12);
            reg_list.insert(r15);
            reg_list
        };

        pub const K_LIFTOFF_ASSEMBLER_FP_CACHE_REGS: DoubleRegList = {
            let mut reg_list = DoubleRegList::new();
            reg_list.insert(xmm0);
            reg_list.insert(xmm1);
            reg_list.insert(xmm2);
            reg_list.insert(xmm3);
            reg_list.insert(xmm4);
            reg_list.insert(xmm5);
            reg_list.insert(xmm6);
            reg_list.insert(xmm7);
            reg_list
        };

        pub const K_LIFTOFF_FRAME_SETUP_FUNCTION_REG: Register = r12;
    }

    #[cfg(target_arch = "mips")]
    pub mod arch_mips {
        use super::*;

        pub const a0: Register = 4;
        pub const a1: Register = 5;
        pub const a2: Register = 6;
        pub const a3: Register = 7;
        pub const t0: Register = 8;
        pub const t1: Register = 9;
        pub const t2: Register = 10;
        pub const t3: Register = 11;
        pub const t4: Register = 12;
        pub const t5: Register = 13;
        pub const t6: Register = 14;
        pub const s7: Register = 23;
        pub const v0: Register = 2;
        pub const v1: Register = 3;

        pub const f0: DoubleRegister = 0;
        pub const f2: DoubleRegister = 2;
        pub const f4: DoubleRegister = 4;
        pub const f6: DoubleRegister = 6;
        pub const f8: DoubleRegister = 8;
        pub const f10: DoubleRegister = 10;
        pub const f12: DoubleRegister = 12;
        pub const f14: DoubleRegister = 14;
        pub const f16: DoubleRegister = 16;
        pub const f18: DoubleRegister = 18;
        pub const f20: DoubleRegister = 20;
        pub const f22: DoubleRegister = 22;
        pub const f24: DoubleRegister = 24;


        pub const K_LIFTOFF_ASSEMBLER_GP_CACHE_REGS: RegList = {
            let mut reg_list = RegList::new();
            reg_list.insert(a0);
            reg_list.insert(a1);
            reg_list.insert(a2);
            reg_list.insert(a3);
            reg_list.insert(t0);
            reg_list.insert(t1);
            reg_list.insert(t2);
            reg_list.insert(t3);
            reg_list.insert(t4);
            reg_list.insert(t5);
            reg_list.insert(t6);
            reg_list.insert(s7);
            reg_list.insert(v0);
            reg_list.insert(v1);
            reg_list
        };

        pub const K_LIFTOFF_ASSEMBLER_FP_CACHE_REGS: DoubleRegList = {
            let mut reg_list = DoubleRegList::new();
            reg_list.insert(f0);
            reg_list.insert(f2);
            reg_list.insert(f4);
            reg_list.insert(f6);
            reg_list.insert(f8);
            reg_list.insert(f10);
            reg_list.insert(f12);
            reg_list.insert(f14);
            reg_list.insert(f16);
            reg_list.insert(f18);
            reg_list.insert(f20);
            reg_list.insert(f22);
            reg_list.insert(f24);
            reg_list
        };
    }

     #[cfg(target_arch = "mips64")]
    pub mod arch_mips64 {
        use super::*;

        pub const a0: Register = 4;
        pub const a1: Register = 5;
        pub const a2: Register = 6;
        pub const a3: Register = 7;
        pub const a4: Register = 8;
        pub const a5: Register = 9;
        pub const a6: Register = 10;
        pub const a7: Register = 11;
        pub const t0: Register = 8;
        pub const t1: Register = 9;
        pub const t2: Register = 10;
        pub const s7: Register = 23;
        pub const v0: Register = 2;
        pub const v1: Register = 3;

        pub const f0: DoubleRegister = 0;
        pub const f2: DoubleRegister = 2;
        pub const f4: DoubleRegister = 4;
        pub const f6: DoubleRegister = 6;
        pub const f8: DoubleRegister = 8;
        pub const f10: DoubleRegister = 10;
        pub const f12: DoubleRegister = 12;
        pub const f14: DoubleRegister = 14;
        pub const f16: DoubleRegister = 16;
        pub const f18: DoubleRegister = 18;
        pub const f20: DoubleRegister = 20;
        pub const f22: DoubleRegister = 22;
        pub const f24: DoubleRegister = 24;
        pub const f26: DoubleRegister = 26;


        pub const K_LIFTOFF_ASSEMBLER_GP_CACHE_REGS: RegList = {
            let mut reg_list = RegList::new();
            reg_list.insert(a0);
            reg_list.insert(a1);
            reg_list.insert(a2);
            reg_list.insert(a3);
            reg_list.insert(a4);
            reg_list.insert(a5);
            reg_list.insert(a6);
            reg_list.insert(a7);
            reg_list.insert(t0);
            reg_list.insert(t1);
            reg_list.insert(t2);
            reg_list.insert(s7);
            reg_list.insert(v0);
            reg_list.insert(v1);
            reg_list
        };

        pub const K_LIFTOFF_ASSEMBLER_FP_CACHE_REGS: DoubleRegList = {
            let mut reg_list = DoubleRegList::new();
            reg_list.insert(f0);
            reg_list.insert(f2);
            reg_list.insert(f4);
            reg_list.insert(f6);
            reg_list.insert(f8);
            reg_list.insert(f10);
            reg_list.insert(f12);
            reg_list.insert(f14);
            reg_list.insert(f16);
            reg_list.insert(f18);
            reg_list.insert(f20);
            reg_list.insert(f22);
            reg_list.insert(f24);
            reg_list.insert(f26);
            reg_list
        };

          pub const K_LIFTOFF_FRAME_SETUP_FUNCTION_REG: Register = t0;

    }

    #[cfg(target_arch = "loongarch64")]
    pub mod arch_loong64 {
        use super::*;

        pub const a0: Register = 4;
        pub const a1: Register = 5;
        pub const a2: Register = 6;
        pub const a3: Register = 7;
        pub const a4: Register = 8;
        pub const a5: Register = 9;
        pub const a6: Register = 10;
        pub const a7: Register = 11;
        pub const t0: Register = 8;
        pub const t1: Register = 9;
        pub const t2: Register = 10;
        pub const t3: Register = 11;
        pub const t4: Register = 12;
        pub const t5: Register = 13;
        pub const s0: Register = 16;
        pub const s1: Register = 17;
        pub const s2: Register = 18;
        pub const s5: Register = 21;
        pub const s7: Register = 23;

        pub const f0: DoubleRegister = 0;
        pub const f1: DoubleRegister = 1;
        pub const f2: DoubleRegister = 2;
        pub const f3: DoubleRegister = 3;
        pub const f4: DoubleRegister = 4;
        pub const f5: DoubleRegister = 5;
        pub const f6: DoubleRegister = 6;
        pub const f7: DoubleRegister = 7;
        pub const f8: DoubleRegister = 8;
        pub const f9: DoubleRegister = 9;
        pub const f10: DoubleRegister = 10;
        pub const f11: DoubleRegister = 11;
        pub const f12: DoubleRegister = 12;
        pub const f13: DoubleRegister = 13;
        pub const f14: DoubleRegister = 14;
        pub const f15: DoubleRegister = 15;
        pub const f16: DoubleRegister = 16;
        pub const f17: DoubleRegister = 17;
        pub const f18: DoubleRegister = 18;
        pub const f19: DoubleRegister = 19;
        pub const f20: DoubleRegister = 20;
        pub const f21: DoubleRegister = 21;
        pub const f22: DoubleRegister = 22;
        pub const f23: DoubleRegister = 23;
        pub const f24: DoubleRegister = 24;
        pub const f25: DoubleRegister = 25;
        pub const f26: DoubleRegister = 26;
        pub const f27: DoubleRegister = 27;
        pub const f28: DoubleRegister = 28;


        pub const K_LIFTOFF_ASSEMBLER_GP_CACHE_REGS: RegList = {
            let mut reg_list = RegList::new();
            reg_list.insert(a0);
            reg_list.insert(a1);
            reg_list.insert(a2);
            reg_list.insert(a3);
            reg_list.insert(a4);
            reg_list.insert(a5);
            reg_list.insert(a6);
            reg_list.insert(a7);
            reg_list.insert(t0);
            reg_list.insert(t1);
            reg_list.insert(t2);
            reg_list.insert(t3);
            reg_list.insert(t4);
            reg_list.insert(t5);
            reg_list.insert(s0);
            reg_list.insert(s1);
            reg_list.insert(s2);
            reg_list.insert(s5);
            reg_list.insert(s7);
            reg_list
        };

        pub const K_LIFTOFF_ASSEMBLER_FP_CACHE_REGS: DoubleRegList = {
            let mut reg_list = DoubleRegList::new();
            reg_list.insert(f0);
            reg_list.insert(f1);
            reg_list.insert(f2);
            reg_list.insert(f3);
            reg_list.insert(f4);
            reg_list.insert(f5);
            reg_list.insert(f6);
            reg_list.insert(f7);
            reg_list.insert(f8);
            reg_list.insert(f9);
            reg_list.insert(f10);
            reg_list.insert(f11);
            reg_list.insert(f12);
            reg_list.insert(f13);
            reg_list.insert(f14);
            reg_list.insert(f15);
            reg_list.insert(f16);
            reg_list.insert(f17);
            reg_list.insert(f18);
            reg_list.insert(f19);
            reg_list.insert(f20);
            reg_list.insert(f21);
            reg_list.insert(f22);
            reg_list.insert(f23);
            reg_list.insert(f24);
            reg_list.insert(f25);
            reg_list.insert(f26);
            reg_list.insert(f27);
            reg_list.insert(f28);
            reg_list
        };

          pub const K_LIFTOFF_FRAME_SETUP_FUNCTION_REG: Register = t0;

    }

    #[cfg(target_arch = "arm")]
    pub mod arch_arm {
        use super::*;

        pub const r0: Register = 0;
        pub const r1: Register = 1;
        pub const r2: Register = 2;
        pub const r3: Register = 3;
        pub const r4: Register = 4;
        pub const r5: Register = 5;
        pub const r6: Register = 6;
        pub const r7: Register = 7;
        pub const r8: Register = 8;
        pub const r9: Register = 9;

        pub const d0: DoubleRegister = 0;
        pub const d1: DoubleRegister = 1;
        pub const d2: DoubleRegister = 2;
        pub const d3: DoubleRegister = 3;
        pub const d4: DoubleRegister = 4;
        pub const d5: DoubleRegister = 5;
        pub const d6: DoubleRegister = 6;
        pub const d7: DoubleRegister = 7;
        pub const d8: DoubleRegister = 8;
        pub const d9: DoubleRegister = 9;
        pub const d10: DoubleRegister = 10;
        pub const d11: DoubleRegister = 11;
        pub const d12: DoubleRegister = 12;


        pub const K_LIFTOFF_ASSEMBLER_GP_CACHE_REGS: RegList = {
            let mut reg_list = RegList::new();
            reg_list.insert(r0);
            reg_list.insert(r1);
            reg_list.insert(r2);
            reg_list.insert(r3);
            reg_list.insert(r4);
            reg_list.insert(r5);
            reg_list.insert(r6);
            reg_list.insert(r7);
            reg_list.insert(r8);
            reg_list.insert(r9);
            reg_list
        };

        pub const K_LIFTOFF_ASSEMBLER_FP_CACHE_REGS: DoubleRegList = {
            let mut reg_list = DoubleRegList::new();
            reg_list.insert(d0);
            reg_list.insert(d1);
            reg_list.insert(d2);
            reg_list.insert(d3);
            reg_list.insert(d4);
            reg_list.insert(d5);
            reg_list.insert(d6);
            reg_list.insert(d7);
            reg_list.insert(d8);
            reg_list.insert(d9);
            reg_list.insert(d10);
            reg_list.insert(d11);
            reg_list.insert(d12);
            reg_list
        };

        pub const K_LIFTOFF_FRAME_SETUP_FUNCTION_REG: Register = r4;
    }

    #[cfg(target_arch = "aarch64")]
    pub mod arch_arm64 {
        use super::*;

        pub const x0: Register = 0;
        pub const x1: Register = 1;
        pub const x2: Register = 2;
        pub const x3: Register = 3;
        pub const x4: Register = 4;
        pub const x5: Register = 5;
        pub const x6: Register = 6;
        pub const x7: Register = 7;
        pub const x8: Register = 8;
        pub const x9: Register = 9;
        pub const x10: Register = 10;
        pub const x11: Register = 11;
        pub const x12: Register = 12;
        pub const x13: Register = 13;
        pub const x14: Register = 14;
        pub const x15: Register = 15;
        pub const x19: Register = 19;
        pub const x20: Register = 20;
        pub const x21: Register = 21;
        pub const x22: Register = 22;
        pub const x23: Register = 23;
        pub const x24: Register = 24;
        pub const x25: Register = 25;
        pub const x27: Register = 27;

        pub const d0: DoubleRegister = 0;
        pub const d1: DoubleRegister = 1;
        pub const d2: DoubleRegister = 2;
        pub const d3: DoubleRegister = 3;
        pub const d4: DoubleRegister = 4;
        pub const d5: DoubleRegister = 5;
        pub const d6: DoubleRegister = 6;
        pub const d7: DoubleRegister = 7;
        pub const d8: DoubleRegister = 8;
        pub const d9: DoubleRegister = 9;
        pub const d10: DoubleRegister = 10;
        pub const d11: DoubleRegister = 11;
        pub const d12: DoubleRegister = 12;
        pub const d13: DoubleRegister = 13;
        pub const d14: DoubleRegister = 14;
        pub const d16: DoubleRegister = 16;
        pub const d17: DoubleRegister = 17;
        pub const d18: DoubleRegister = 18;
        pub const d19: DoubleRegister = 19;
        pub const d20: DoubleRegister = 20;
        pub const d21: DoubleRegister = 21;
        pub const d22: DoubleRegister = 22;
        pub const d23: DoubleRegister = 23;
        pub const d24: DoubleRegister = 24;
        pub const d25: DoubleRegister = 25;
        pub const d26: DoubleRegister = 26;
        pub const d27: DoubleRegister = 27;

        pub const K_LIFTOFF_ASSEMBLER_GP_CACHE_REGS: RegList = {
            let mut reg_list = RegList::new();
            reg_list.insert(x0);
            reg_list.insert(x1);
            reg_list.insert(x2);
            reg_list.insert(x3);
            reg_list.insert(x4);
            reg_list.insert(x5);
            reg_list.insert(x6);
            reg_list.insert(x7);
            reg_list.insert(x8);
            reg_list.insert(x9);
            reg_list.insert(x10);
            reg_list.insert(x11);
            reg_list.insert(x12);
            reg_list.insert(x13);
            reg_list.insert(x14);
            reg_list.insert(x15);
            reg_list.insert(x19);
            reg_list.insert(x20);
            reg_list.insert(x21);
            reg_list.insert(x22);
            reg_list.insert(x23);
            reg_list.insert(x24);
            reg_list.insert(x25);
            reg_list.insert(x27);
            reg_list
        };

        pub const K_LIFTOFF_ASSEMBLER_FP_CACHE_REGS: DoubleRegList = {
            let mut reg_list = DoubleRegList::new();
            reg_list.insert(d0);
            reg_list.insert(d1);
            reg_list.insert(d2);
            reg_list.insert(d3);
            reg_list.insert(d4);
            reg_list.insert(d5);
            reg_list.insert(d6);
            reg_list.insert(d7);
            reg_list.insert(d8);
            reg_list.insert(d9);
            reg_list.insert(d10);
            reg_list.insert(d11);
            reg_list.insert(d12);
            reg_list.insert(d13);
            reg_list.insert(d14);
            reg_list.insert(d16);
            reg_list.insert(d17);
            reg_list.insert(d18);
            reg_list.insert(d19);
            reg_list.insert(d20);
            reg_list.insert(d21);
            reg_list.insert(d22);
            reg_list.insert(d23);
            reg_list.insert(d24);
            reg_list.insert(d25);
            reg_list.insert(d26);
            reg_list.insert(d27);
            reg_list
        };

        pub const K_LIFTOFF_FRAME_SETUP_FUNCTION_REG: Register = x8;
    }

    #[cfg(target_arch = "s390x")]
    pub mod arch_s390x {
        use super::*;

        pub const r2: Register = 2;
        pub const r3: Register = 3;
        pub const r4: Register = 4;
        pub const r5: Register = 5;
        pub const r6: Register = 6;
        pub const r7: Register = 7;
        pub const r8: Register = 8;
        pub const cp: Register = 13;

        pub const d0: DoubleRegister = 0;
        pub const d1: DoubleRegister = 1;
        pub const d2: DoubleRegister = 2;
        pub const d3: DoubleRegister = 3;
        pub const d4: DoubleRegister = 4;
        pub const d5: DoubleRegister = 5;
        pub const d6: DoubleRegister = 6;
        pub const d7: DoubleRegister = 7;
        pub const d8: DoubleRegister = 8;
        pub const d9: DoubleRegister = 9;
        pub const d10: DoubleRegister = 10;
        pub const d11: DoubleRegister = 11;
        pub const d12: DoubleRegister = 12;


        pub const K_LIFTOFF_ASSEMBLER_GP_CACHE_REGS: RegList = {
            let mut reg_list = RegList::new();
            reg_list.insert(r2);
            reg_list.insert(r3);
            reg_list.insert(r4);
            reg_list.insert(r5);
            reg_list.insert(r6);
            reg_list.insert(r7);
            reg_list.insert(r8);
            reg_list.insert(cp);
            reg_list
        };

        pub const K_LIFTOFF_ASSEMBLER_FP_CACHE_REGS: DoubleRegList = {
            let mut reg_list = DoubleRegList::new();
            reg_list.insert(d0);
            reg_list.insert(d1);
            reg_list.insert(d2);
            reg_list.insert(d3);
            reg_list.insert(d4);
            reg_list.insert(d5);
            reg_list.insert(d6);
            reg_list.insert(d7);
            reg_list.insert(d8);
            reg_list.insert(d9);
            reg_list.insert(d10);
            reg_list.insert(d11);
            reg_list.insert(d12);
            reg_list
        };

        pub const K_LIFTOFF_FRAME_SETUP_FUNCTION_REG: Register = r7;
    }

    #[cfg(target_arch = "powerpc64")]
    pub mod arch_ppc64 {
        use super::*;

        pub const r3: Register = 3;
        pub const r4: Register = 4;
        pub const r5: Register = 5;
        pub const r6: Register = 6;
        pub const r7: Register = 7;
        pub const r8: Register = 8;
        pub const r9: Register = 9;
        pub const r10: Register = 10;
        pub const r11: Register = 11;
        pub const r15: Register = 15;
        pub const cp: Register = 30; //Context Pointer

        pub const d0: DoubleRegister = 0;
        pub const d1: DoubleRegister = 1;
        pub const d2: DoubleRegister = 2;
        pub const d3: DoubleRegister = 3;
        pub const d4: DoubleRegister = 4;
        pub const d5: DoubleRegister = 5;
        pub const d6: DoubleRegister = 6;
        pub const d7: DoubleRegister = 7;
        pub const d8: DoubleRegister = 8;
        pub const d9: DoubleRegister = 9;
        pub const d10: DoubleRegister = 10;
        pub const d11: DoubleRegister = 11;
        pub const d12: DoubleRegister = 12;


        pub const K_LIFTOFF_ASSEMBLER_GP_CACHE_REGS: RegList = {
            let mut reg_list = RegList::new();
            reg_list.insert(r3);
            reg_list.insert(r4);
            reg_list.insert(r5);
            reg_list.insert(r6);
            reg_list.insert(r7);
            reg_list.insert(r8);
            reg_list.insert(r9);
            reg_list.insert(r10);
