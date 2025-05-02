// Copyright 2009 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#[cfg(target_arch = "arm")]
mod constants_arm {
    //use std::ffi::CString;
    //use std::os::raw::c_char;
    use std::mem::transmute;
    use std::cmp::PartialEq;

    // Placeholder type for Instr.  Should be a concrete type based on ARM architecture.
    pub type Instr = u32;

    // Placeholder for JitAllocation, needs further definition
    pub struct WritableJitAllocation {}

    impl WritableJitAllocation {
        pub fn write_value<T>(&mut self, _address: usize, _value: T) {
            // Placeholder implementation, needs a more concrete implementation.
        }
    }

    /// Represents an ARM instruction.
    pub struct Instruction {}

    impl Instruction {
        /// Sets the instruction bits.
        pub fn set_instruction_bits(&self, value: Instr, jit_allocation: Option<&mut WritableJitAllocation>) {
            match jit_allocation {
                Some(alloc) => {
                    let address = self as *const Self as usize;
                    alloc.write_value(address, value);
                }
                None => {
                    unsafe {
                        let ptr = self as *const Self as *mut Instr;
                        *ptr = value;
                    }
                }
            }
        }

        /// Reconstructs a double from the immediate encoded in the vmov instruction.
        pub fn double_immed_vmov(&self) -> f64 {
            // instruction: [xxxxxxxx,xxxxabcd,xxxxxxxx,xxxxefgh]
            // double: [aBbbbbbb,bbcdefgh,00000000,00000000,
            //          00000000,00000000,00000000,00000000]
            // where B = ~b. Only the high 16 bits are affected.

            let instruction_ptr = self as *const Self as *const u32;
            let instruction: u32;

            unsafe {
                instruction = *instruction_ptr;
            }

            let high16: u64 = {
                let mut high16: u64 = ((instruction >> 16 & 0x3) as u64) << 4 | (instruction & 0xF) as u64; // xxxxxxxx,xxcdefgh
                high16 |= (((instruction >> 18) & 0x1) as u64) * 0xFF << 6; // xxbbbbbb,bbxxxxxx
                high16 |= ((instruction >> 18) & 0x1 ^ 1) as u64 << 14; // xBxxxxxx,xxxxxxxx
                high16 |= ((instruction >> 19) & 0x1) as u64 << 15; // axxxxxxx,xxxxxxxx
                high16
            };

            let imm: u64 = high16 << 48;
            unsafe { transmute(imm) }
        }
    }

    pub const K_NUM_REGISTERS: usize = 16;
    pub const K_NO_REGISTER: i32 = -1;

    /// Represents ARM registers.
    pub struct Registers {}

    impl Registers {
        pub const NAMES: [&'static str; K_NUM_REGISTERS] = [
            "r0", "r1", "r2", "r3", "r4", "r5", "r6", "r7", "r8", "r9", "r10", "fp", "ip", "sp", "lr", "pc",
        ];

        #[derive(Clone, Copy)]
        pub struct RegisterAlias {
            pub reg: i32,
            pub name: &'static str,
        }

        pub const ALIASES: [RegisterAlias; 5] = [
            RegisterAlias { reg: 10, name: "sl" },
            RegisterAlias { reg: 11, name: "r11" },
            RegisterAlias { reg: 12, name: "r12" },
            RegisterAlias { reg: 13, name: "r13" },
            RegisterAlias { reg: 14, name: "r14" },
           // RegisterAlias { reg: 15, name: "r15" }, // This line caused problems, likely a bug.
            // The 'static lifetime cannot be inferred within this context
        ];

        pub fn number(name: &str) -> i32 {
            // Look through the canonical names.
            for (i, &reg_name) in Registers::NAMES.iter().enumerate() {
                if reg_name == name {
                    return i as i32;
                }
            }

            // Look through the alias names.
            for alias in &Registers::ALIASES {
                if alias.name == name {
                    return alias.reg;
                }
            }

            // No register with the requested name found.
            K_NO_REGISTER
        }
    }

    pub const K_NUM_VFP_SINGLE_REGISTERS: usize = 32;
    pub const K_NUM_VFP_REGISTERS: usize = 64;

    /// Represents VFP registers.
    pub struct VFPRegisters {}

    impl VFPRegisters {
        pub const NAMES: [&'static str; K_NUM_VFP_REGISTERS] = [
            "s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "s12", "s13", "s14", "s15",
            "s16", "s17", "s18", "s19", "s20", "s21", "s22", "s23", "s24", "s25", "s26", "s27", "s28", "s29", "s30", "s31",
            "d0", "d1", "d2", "d3", "d4", "d5", "d6", "d7", "d8", "d9", "d10", "d11", "d12", "d13", "d14", "d15",
            "d16", "d17", "d18", "d19", "d20", "d21", "d22", "d23", "d24", "d25", "d26", "d27", "d28", "d29", "d30", "d31",
        ];

        pub fn name(reg: i32, is_double: bool) -> &'static str {
            assert!((0 <= reg) && (reg < K_NUM_VFP_REGISTERS as i32));
            VFPRegisters::NAMES[reg as usize + if is_double { K_NUM_VFP_SINGLE_REGISTERS } else { 0 }]
        }

        pub fn number(name: &str, is_double: &mut bool) -> i32 {
            for (i, &reg_name) in VFPRegisters::NAMES.iter().enumerate() {
                if reg_name == name {
                    if i < K_NUM_VFP_SINGLE_REGISTERS {
                        *is_double = false;
                        return i as i32;
                    } else {
                        *is_double = true;
                        return (i - K_NUM_VFP_SINGLE_REGISTERS) as i32;
                    }
                }
            }

            // No register with the requested name found.
            K_NO_REGISTER
        }
    }

    impl PartialEq for Registers::RegisterAlias{
        fn eq(&self, other: &Self) -> bool {
            self.reg == other.reg && self.name == other.name
        }
    }

}

#[cfg(target_arch = "arm")]
pub use constants_arm::*;