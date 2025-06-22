// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Note: The original C++ code includes headers that are part of the V8
// codebase.  Since a complete translation would require porting the entire
// V8 project, this Rust code focuses on the core logic and omits the
// V8-specific dependencies.  Assumed definitions and structures are used.

// Assume v8_flags is a global configuration object from V8
// For demonstration, we use a simple struct
struct V8Flags {
    riscv_c_extension: bool,
}

// Placeholder for global v8_flags instance
static V8_FLAGS: V8Flags = V8Flags {
    riscv_c_extension: false,
};

mod constants_riscv {
    pub const kNumSimuRegisters: usize = 34;
    pub const kNumFPURegisters: usize = 32;
    pub const kNumVRegisters: usize = 32;
    pub const kInvalidRegister: i32 = -1;
    pub const kInvalidFPURegister: i32 = -1;
    pub const kInvalidVRegister: i32 = -1;

    //Compressed Instruction Masks and Shifts
    pub const kRvcOpcodeMask: u32 = 0x03;
    pub const kRvcRdShift: u32 = 7;
    pub const kRvcRdBits: u32 = 5;
    pub const kRvcRs2Shift: u32 = 2;
    pub const kRvcRs2Bits: u32 = 5;
    pub const kRvcRs1sShift: u32 = 7;
    pub const kRvcRs1sBits: u32 = 3;
    pub const kRvcRs2sShift: u32 = 2;
    pub const kRvcRs2sBits: u32 = 3;
    pub const kRvcFunct6Shift: u32 = 10;
    pub const kRvcFunct6Bits: u32 = 6;
    pub const kRvcFunct4Shift: u32 = 12;
    pub const kRvcFunct4Bits: u32 = 4;
    pub const kRvcFunct3Shift: u32 = 13;
    pub const kRvcFunct3Bits: u32 = 3;
    pub const kRvcFunct2Shift: u32 = 5;
    pub const kRvcFunct2Bits: u32 = 2;
    pub const kRvcFunct2BShift: u32 = 10;

    pub const C2: u8 = 2;

    //Opcode Encodings
    pub const LOAD: u32 = 0x00000003;
    pub const LOAD_FP: u32 = 0x00000007;
    pub const MISC_MEM: u32 = 0x0000000F;
    pub const OP_IMM: u32 = 0x00000013;
    pub const AUIPC: u32 = 0x00000017;
    pub const OP_IMM_32: u32 = 0x0000001B;
    pub const STORE: u32 = 0x00000023;
    pub const STORE_FP: u32 = 0x00000027;
    pub const AMO: u32 = 0x0000002F;
    pub const OP: u32 = 0x00000033;
    pub const LUI: u32 = 0x00000037;
    pub const OP_32: u32 = 0x0000003B;
    pub const MADD: u32 = 0x00000043;
    pub const MSUB: u32 = 0x00000047;
    pub const NMSUB: u32 = 0x0000004B;
    pub const NMADD: u32 = 0x0000004F;
    pub const OP_FP: u32 = 0x00000053;
    pub const BRANCH: u32 = 0x00000063;
    pub const JALR: u32 = 0x00000067;
    pub const JAL: u32 = 0x0000006F;
    pub const SYSTEM: u32 = 0x00000073;
    pub const OP_V: u32 = 0x00000057;

    //Compressed Instruction Encodings
    pub const RO_C_ADDI4SPN: u32 = 0x00;
    pub const RO_C_FLD: u32 = 0x02;
    pub const RO_C_LW: u32 = 0x04;
    #[cfg(target_arch = "riscv64")]
    pub const RO_C_LD: u32 = 0x06;
    pub const RO_C_FSD: u32 = 0x03;
    pub const RO_C_SW: u32 = 0x05;
    #[cfg(target_arch = "riscv64")]
    pub const RO_C_SD: u32 = 0x07;
    pub const RO_C_NOP_ADDI: u32 = 0x00;
    pub const RO_C_LI: u32 = 0x01;
    #[cfg(target_arch = "riscv64")]
    pub const RO_C_ADDIW: u32 = 0x03;
    pub const RO_C_LUI_ADD: u32 = 0x02;
    pub const RO_C_MISC_ALU: u32 = 0x00;
    pub const RO_C_J: u32 = 0x01;
    pub const RO_C_BEQZ: u32 = 0x02;
    pub const RO_C_BNEZ: u32 = 0x03;
    pub const RO_C_SLLI: u32 = 0x00;
    pub const RO_C_FLDSP: u32 = 0x02;
    pub const RO_C_LWSP: u32 = 0x04;
    #[cfg(target_arch = "riscv64")]
    pub const RO_C_LDSP: u32 = 0x06;
    pub const RO_C_JR_MV_ADD: u32 = 0x00;
    pub const RO_C_FSDSP: u32 = 0x02;
    pub const RO_C_SWSP: u32 = 0x04;
    #[cfg(target_arch = "riscv64")]
    pub const RO_C_SDSP: u32 = 0x06;
    pub const RO_LB: u32 = 0x0;
    pub const RO_LBU: u32 = 0x1;
    pub const RO_LH: u32 = 0x2;
    pub const RO_LHU: u32 = 0x3;
    pub const RO_LW: u32 = 0x0;

    #[cfg(target_arch = "riscv64")]
    pub const RO_LD: u32 = 0x3;
    #[cfg(target_arch = "riscv64")]
    pub const RO_LWU: u32 = 0x4;

    pub const RO_SB: u32 = 0x0;
    pub const RO_SH: u32 = 0x1;
    pub const RO_SW: u32 = 0x2;

    #[cfg(target_arch = "riscv64")]
    pub const RO_SD: u32 = 0x3;

    //Masks and shifts for Vector instructions
    pub const kBaseOpcodeMask: u32 = 0x7f;
    pub const kFunct3Mask: u32 = 0x7000;
    pub const RO_V_VSETVLI: u32 = 0x05707;
    pub const RO_V_VSETIVLI: u32 = 0x05747;
    pub const kRvvZimmMask: u32 = 0x1f8000;
    pub const kRvvZimmShift: u32 = 15;
    pub const kRvvUimmMask: u32 = 0xff000;
    pub const kRvvUimmShift: u32 = 12;

}

mod base_constants_riscv {
    use super::constants_riscv::*;
    use std::ffi::CStr;
    use std::os::raw::c_char;
    use std::str;

    /// Represents RISC-V registers.
    pub struct Registers {}

    impl Registers {
        const NAMES: [&'static str; kNumSimuRegisters] = [
            "zero_reg", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "fp", "s1", "a0", "a1", "a2",
            "a3", "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10",
            "s11", "t3", "t4", "t5", "t6", "pc",
        ];

        #[derive(Clone, Copy)]
        pub struct RegisterAlias {
            pub reg: i32,
            pub name: &'static str,
        }

        const ALIASES: [RegisterAlias; 4] = [
            RegisterAlias { reg: 0, name: "zero" },
            RegisterAlias { reg: 33, name: "pc" },
            RegisterAlias { reg: 8, name: "s0" },
            RegisterAlias { reg: 8, name: "s0_fp" },
            // Sentinel
        ];

        /// Returns the name of a register.
        pub fn name(reg: i32) -> &'static str {
            if (0 <= reg) && (reg < kNumSimuRegisters as i32) {
                Registers::NAMES[reg as usize]
            } else {
                "noreg"
            }
        }

        /// Returns the register number for a given name.
        pub fn number(name: &str) -> i32 {
            // Look through the canonical names.
            for (i, &reg_name) in Registers::NAMES.iter().enumerate() {
                if reg_name == name {
                    return i as i32;
                }
            }

            // Look through the alias names.
            for alias in Registers::ALIASES.iter() {
                if alias.name == name {
                    return alias.reg;
                }
            }

            // No register with the requested name found.
            kInvalidRegister
        }
    }

    /// Represents RISC-V floating-point registers.
    pub struct FPURegisters {}

    impl FPURegisters {
        //        const NAMES: [&'static str; kNumFPURegisters] = [
        //            "f0", "f1", "f2", "f3", "f4", "f5", "f6", "f7", "f8", "f9", "f10", "f11", "f12",
        //            "f13", "f14", "f15", "f16", "f17", "f18", "f19", "f20", "f21", "f22", "f23", "f24",
        //            "f25", "f26", "f27", "f28", "f29", "f30", "f31",
        //        ];

        const NAMES: [&'static str; kNumFPURegisters] = [
            "ft0", "ft1", "ft2", "ft3", "ft4", "ft5", "ft6", "ft7", "fs0", "fs1", "fa0", "fa1",
            "fa2", "fa3", "fa4", "fa5", "fa6", "fa7", "fs2", "fs3", "fs4", "fs5", "fs6", "fs7",
            "fs8", "fs9", "fs10", "fs11", "ft8", "ft9", "ft10", "ft11",
        ];

        #[derive(Clone, Copy)]
        pub struct RegisterAlias {
            pub creg: i32,
            pub name: &'static str,
        }

        const ALIASES: [RegisterAlias; 1] = [
            // Sentinel
            RegisterAlias {
                creg: kInvalidRegister,
                name: std::ptr::null() as *const str,
            },
        ];

        /// Returns the name of a floating-point register.
        pub fn name(creg: i32) -> &'static str {
            if (0 <= creg) && (creg < kNumFPURegisters as i32) {
                FPURegisters::NAMES[creg as usize]
            } else {
                "nocreg"
            }
        }

        /// Returns the register number for a given name.
        pub fn number(name: &str) -> i32 {
            // Look through the canonical names.
            for (i, &reg_name) in FPURegisters::NAMES.iter().enumerate() {
                if reg_name == name {
                    return i as i32;
                }
            }

            // Look through the alias names.
            for alias in FPURegisters::ALIASES.iter() {
                if alias.name != std::ptr::null() as *const str {
                  //  let c_str = unsafe { CStr::from_ptr(alias.name) };
                  //  let alias_name = c_str.to_str().unwrap();
                    if alias.name == name {
                        return alias.creg;
                    }
                } else {
                    break;
                }
            }

            // No register with the requested name found.
            kInvalidFPURegister
        }
    }

    /// Represents RISC-V vector registers.
    pub struct VRegisters {}

    impl VRegisters {
        const NAMES: [&'static str; kNumVRegisters] = [
            "v0", "v1", "v2", "v3", "v4", "v5", "v6", "v7", "v8", "v9", "v10", "v11", "v12", "v13",
            "v14", "v15", "v16", "v17", "v18", "v19", "v20", "v21", "v22", "v23", "v24", "v25",
            "v26", "v27", "v28", "v29", "v30", "v31",
        ];

        #[derive(Clone, Copy)]
        pub struct RegisterAlias {
            pub creg: i32,
            pub name: &'static str,
        }

        const ALIASES: [RegisterAlias; 1] = [
            // Sentinel
            RegisterAlias {
                creg: kInvalidRegister,
                name: std::ptr::null() as *const str,
            },
        ];

        /// Returns the name of a vector register.
        pub fn name(creg: i32) -> &'static str {
            if (0 <= creg) && (creg < kNumVRegisters as i32) {
                VRegisters::NAMES[creg as usize]
            } else {
                "nocreg"
            }
        }

        /// Returns the register number for a given name.
        pub fn number(name: &str) -> i32 {
            // Look through the canonical names.
            for (i, &reg_name) in VRegisters::NAMES.iter().enumerate() {
                if reg_name == name {
                    return i as i32;
                }
            }

            // Look through the alias names.
            for alias in VRegisters::ALIASES.iter() {
                if alias.name != std::ptr::null() as *const str {
                  //  let c_str = unsafe { CStr::from_ptr(alias.name) };
                  //  let alias_name = c_str.to_str().unwrap();
                    if alias.name == name {
                        return alias.creg;
                    }
                } else {
                    break;
                }
            }

            // No register with the requested name found.
            kInvalidVRegister
        }
    }
}

mod codegen_riscv {
    use super::base_constants_riscv::*;
    use super::base_constants_riscv::Registers;
    use super::base_constants_riscv::FPURegisters;
    use super::base_constants_riscv::VRegisters;
    use super::constants_riscv::*;
    use super::V8_FLAGS;

    pub struct InstructionBase {
        instruction_bits: u32,
    }

    impl InstructionBase {
        pub fn new(instruction_bits: u32) -> Self {
            InstructionBase { instruction_bits }
        }

        pub fn instruction_bits(&self) -> u32 {
            self.instruction_bits
        }

        pub fn is_short_instruction(&self) -> bool {
            let first_byte = (self.instruction_bits as u8) & 0x03;
            first_byte <= C2
        }

        pub fn is_illegal_instruction(&self) -> bool {
            // Placeholder implementation for illegal instruction check.  The
            // original C++ code might rely on V8-specific state.
            false
        }

        pub fn instruction_type(&self) -> InstructionType {
            if self.is_illegal_instruction() {
                return InstructionType::Unsupported;
            }

            if V8_FLAGS.riscv_c_extension && self.is_short_instruction() {
                match self.instruction_bits() & kRvcOpcodeMask {
                    RO_C_ADDI4SPN => InstructionType::CIWType,
                    RO_C_FLD | RO_C_LW => InstructionType::CLType,
                    #[cfg(target_arch = "riscv64")]
                    RO_C_LD => InstructionType::CLType,
                    RO_C_FSD | RO_C_SW => InstructionType::CSType,
                    #[cfg(target_arch = "riscv64")]
                    RO_C_SD => InstructionType::CSType,
                    RO_C_NOP_ADDI | RO_C_LI => InstructionType::CIType,
                    #[cfg(target_arch = "riscv64")]
                    RO_C_ADDIW => InstructionType::CIType,
                    RO_C_LUI_ADD => InstructionType::CIType,
                    RO_C_MISC_ALU => {
                        if self.bits(11, 10) != 0b11 {
                            InstructionType::CBType
                        } else {
                            InstructionType::CAType
                        }
                    }
                    RO_C_J => InstructionType::CJType,
                    RO_C_BEQZ | RO_C_BNEZ => InstructionType::CBType,
                    RO_C_SLLI | RO_C_FLDSP | RO_C_LWSP => InstructionType::CIType,
                    #[cfg(target_arch = "riscv64")]
                    RO_C_LDSP => InstructionType::CIType,
                    RO_C_JR_MV_ADD => InstructionType::CRType,
                    RO_C_FSDSP | RO_C_SWSP => InstructionType::CSSType,
                    #[cfg(target_arch = "riscv64")]
                    RO_C_SDSP => InstructionType::CSSType,
                    _ => InstructionType::Unsupported,
                }
            } else {
                match self.instruction_bits() & kBaseOpcodeMask {
                    LOAD => InstructionType::IType,
                    LOAD_FP => InstructionType::IType,
                    MISC_MEM => InstructionType::IType,
                    OP_IMM => InstructionType::IType,
                    AUIPC => InstructionType::UType,
                    OP_IMM_32 => InstructionType::IType,
                    STORE => InstructionType::SType,
                    STORE_FP => InstructionType::SType,
                    AMO => InstructionType::RType,
                    OP => InstructionType::RType,
                    LUI => InstructionType::UType,
                    OP_32 => InstructionType::RType,
                    MADD | MSUB | NMSUB | NMADD => InstructionType::R4Type,
                    OP_FP => InstructionType::RType,
                    BRANCH => InstructionType::BType,
                    JALR => InstructionType::IType,
                    JAL => InstructionType::JType,
                    SYSTEM => InstructionType::IType,
                    OP_V => InstructionType::VType,
                    _ => InstructionType::Unsupported,
                }
            }
        }

        fn bits(&self, msb: u32, lsb: u32) -> u32 {
            let mask = ((1 << (msb - lsb + 1)) - 1) << lsb;
            (self.instruction_bits & mask) >> lsb
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum InstructionType {
        RType,
        IType,
        SType,
        BType,
        UType,
        JType,
        R4Type,
        VType,
        CIWType,
        CLType,
        CSType,
        CIType,
        CBType,
        CAType,
        CJType,
        CRType,
        CSSType,
        Unsupported,
    }

    pub struct InstructionGetters<T> {
        instruction: T,
    }

    impl<T> InstructionGetters<T>
    where
        T: AsRef<InstructionBase>,
    {
        pub fn new(instruction: T) -> Self {
            InstructionGetters { instruction }
        }

        fn is_short_instruction(&self) -> bool {
            self.instruction.as_ref().is_short_instruction()
        }

        fn bits(&self, msb: u32, lsb: u32) -> u32 {
            self.instruction.as_ref().bits(msb, lsb)
        }

        fn instruction_bits(&self) -> u32 {
            self.instruction.as_ref().instruction_bits()
        }

        fn operand_funct3(&self) -> u32 {
            (self.instruction_bits() >> 12) & 0x7
        }

        fn base_opcode(&self) -> u32 {
            self.instruction_bits() & 0x7f
        }

        pub fn rvc_rd_value(&self) -> i32 {
            assert!(self.is_short_instruction());
            self.bits(kRvcRdShift + kRvcRdBits - 1, kRvcRdShift) as i32
        }

        pub fn rvc_rs2_value(&self) -> i32 {
            assert!(self.is_short_instruction());
            self.bits(kRvcRs2Shift + kRvcRs2Bits - 1, kRvcRs2Shift) as i32
        }

        pub fn rvc_rs1s_value(&self) -> i32 {
            assert!(self.is_short_instruction());
            0b1000 + self.bits(kRvcRs1sShift + kRvcRs1sBits - 1, kRvcRs1sShift) as i32
        }

        pub fn rvc_rs2s_value(&self) -> i32 {
            assert!(self.is_short_instruction());
            0b1000 + self.bits(kRvcRs2sShift + kRvcRs2sBits - 1, kRvcRs2sShift) as i32
        }

        #[inline]
        pub fn rvc_funct6_value(&self) -> i32 {
            assert!(self.is_short_instruction());
            self.bits(kRvcFunct6Shift + kRvcFunct6Bits - 1, kRvcFunct6Shift) as i32
        }

        #[inline]
        pub fn rvc_funct4_value(&self) -> i32 {
            assert!(self.is_short_instruction());
            self.bits(kRvcFunct4Shift + kRvcFunct4Bits - 1, kRvcFunct4Shift) as i32
        }

        #[inline]
        pub fn rvc_funct3_value(&self) -> i32 {
            assert!(self.is_short_instruction());
            self.bits(kRvcFunct3Shift + kRvcFunct3Bits - 1, kRvcFunct3Shift) as i32
        }

        #[inline]
        pub fn rvc_funct2_value(&self) -> i32 {
            assert!(self.is_short_instruction());
            self.bits(kRvcFunct2Shift + kRvcFunct2Bits - 1, kRvcFunct2Shift) as i32
        }

        #[inline]
        pub fn rvc_funct2b_value(&self) -> i32 {
            assert!(self.is_short_instruction());
            self.bits(kRvcFunct2BShift + kRvcFunct2Bits - 1, kRvcFunct2BShift) as i32
        }

        pub fn rvvzimm(&self) -> u32 {
            if (self.instruction_bits() & (kBaseOpcodeMask | kFunct3Mask | 0x80000000))
                == RO_V_VSETVLI
            {
                let bits = self.instruction_bits();
                let zimm = bits & kRvvZimmMask;
                zimm >> kRvvZimmShift
            } else {
                assert_eq!(
                    self.instruction_bits() & (kBaseOpcodeMask | kFunct3Mask | 0xC0000000),
                    RO_V_VSETIVLI
                );
                let bits = self.instruction_bits();
                let zimm = bits & kRvvZimmMask;
                (zimm >> kRvvZimmShift) & 0x3FF
            }
        }

        pub fn rvvuimm(&self) -> u32 {
            assert_eq!(
                self.instruction_bits() & (kBaseOpcodeMask | kFunct3Mask | 0xC0000000),
                RO_V_VSETIVLI
            );
            let bits = self.instruction_bits();
            let uimm = bits & kRvvUimmMask;
            uimm >> kRvvUimmShift
        }

        pub fn is_load(&self) -> bool {
            match self.operand_funct3() {
                RO_LB | RO_LBU | RO_LH | RO_LHU | RO_LW => true,
                #[cfg(target_arch = "riscv64")]
                RO_LD | RO_LWU => true,
                _ => {
                    if V8_FLAGS.riscv_c_extension && self.is_short_instruction(){
                         match self.operand_funct3(){
                            constants_riscv::RO_C_LW => true,
                            constants_riscv::RO_C_LWSP => true,
                            #[cfg(target_arch = "riscv64")]
                            constants_riscv::RO_C_LD => true,
                            #[cfg(target_arch = "riscv64")]
                            constants_riscv::RO_C_LDSP => true,
                            _ => {
                                self.base_opcode() == LOAD_FP
                            }
                        }
                    }else{
                        self.base_opcode() == LOAD_FP
                    }
                }
            }
        }

        pub fn is_store(&self) -> bool {
            match self.operand_funct3() {
                RO_SB | RO_SH | RO_SW => true,
                #[cfg(target_arch = "riscv64")]
                RO_SD => true,
                _ => {
                    if V8_FLAGS.riscv_c_extension && self.is_short_instruction(){
                         match self.operand_funct3(){
                            constants_riscv::RO_C_SW => true,
                            constants_riscv::RO_C_SWSP => true,
                            #[cfg(target_arch = "riscv64")]
                            constants_riscv::RO_C_SD => true,
                            #[cfg(target_arch = "riscv64")]
                            constants_riscv::RO_C_SDSP => true,
                            _ => {
                                self.base_opcode() == STORE_FP
                            }
                         }
                    }else{
                        self.base_opcode() == STORE_FP
                    }
                }
            }
        }
    }

    impl AsRef<InstructionBase> for InstructionBase {
        fn as_ref(&self) -> &InstructionBase {
            self
        }
    }

    #[cfg(feature = "simulator")]
    pub struct SimInstructionBase {
        instruction_bits: u32,
    }

    #[cfg(feature = "simulator")]
    impl SimInstructionBase {
        pub fn new(instruction_bits: u32) -> Self {
            SimInstructionBase { instruction_bits }
        }
    }

    #[cfg(feature = "simulator")]
    impl AsRef<InstructionBase> for SimInstructionBase {
        fn as_ref(&self) -> &InstructionBase {
            //This is just a placeholder - we need to access the fields of SimInstructionBase
            //as if it were an InstructionBase, but Rust does not have inheritance.
            //For the sake of demonstrating the port, we construct a new InstructionBase
            //here. In a real port, we would need to refactor the code to avoid this.
            unsafe {
                std::mem::transmute::<&Self, &InstructionBase>(self)
            }
        }
    }

    #[test]
    fn test_instruction_type() {
        let instr = InstructionBase::new(0x00000003);
        assert_eq!(instr.instruction_type(), InstructionType::IType);

        let instr = InstructionBase::new(0x00000007);
        assert_eq!(instr.instruction_type(), InstructionType::IType);

        let instr = InstructionBase::new(0x0000000F);
        assert_eq!(instr.instruction_type(), InstructionType::IType);

        let instr = InstructionBase::new(0x00000013);
        assert_eq!(instr.instruction_type(), InstructionType::IType);

        let instr = InstructionBase::new(0x00000017);
        assert_eq!(instr.instruction_type(), InstructionType::UType);

        let instr = InstructionBase::new(0x0000001B);
        assert_eq!(instr.instruction_type(), InstructionType::IType);

        let instr = InstructionBase::new(0x00000023);
        assert_eq!(instr.instruction_type(), InstructionType::SType);

        let instr = InstructionBase::new(0x00000027);
        assert_eq!(instr.instruction_type(), InstructionType::SType);

        let instr = InstructionBase::new(0x0000002F);
        assert_eq!(instr.instruction_type(), InstructionType::RType);

        let instr = InstructionBase::new(0x00000033);
        assert_eq!(instr.instruction_type(), InstructionType::RType);

        let instr = InstructionBase::new(0x00000037);
        assert_eq!(instr.instruction_type(), InstructionType::UType);

        let instr = InstructionBase::new(0x0000003B);
        assert_eq!(instr.instruction_type(), InstructionType::RType);

        let instr = InstructionBase::new(0x00000043);
        assert_eq!(instr.instruction_type(), InstructionType::R4Type);

        let instr = InstructionBase::new(0x00000047);
        assert_eq!(instr.instruction_type(), InstructionType::R4Type);

        let instr = InstructionBase::new(0x0000004B);
        assert_eq!(instr.instruction_type(), InstructionType::R4Type);

        let instr = InstructionBase::new(0x0000004F);
        assert_eq!(instr.instruction_type(), InstructionType::R4Type);

        let instr = InstructionBase::new(0x00000053);
        assert_eq!(instr.instruction_type(), InstructionType::RType);

        let instr = InstructionBase::new(0x00000063);
        assert_eq!(instr.instruction_type(), InstructionType::BType);

        let instr = InstructionBase::new(0x00000067);
        assert_eq!(instr.instruction_type(), InstructionType::IType);

        let instr = InstructionBase::new(0x0000006F);
        assert_eq!(instr.instruction_type(), InstructionType::