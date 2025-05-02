// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod riscv_a {

    /// Represents a RISC-V register.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Register(u8);

    impl Register {
        pub const ZERO: Register = Register(0);

        pub fn new(value: u8) -> Self {
            Register(value)
        }
    }

    pub struct AssemblerRISCVA {}

    impl AssemblerRISCVA {
        pub fn new() -> Self {
            AssemblerRISCVA {}
        }

        fn gen_instr_ratomic(
            &mut self,
            funct5: u8,
            aq: bool,
            rl: bool,
            funct3: u8,
            rd: Register,
            rs1: Register,
            rs2: Register,
        ) {
            // This function would actually generate the instruction encoding in a real assembler.
            // For this example, we'll just print the arguments.
            println!(
                "Atomic Instruction: funct5={}, aq={}, rl={}, funct3={}, rd={:?}, rs1={:?}, rs2={:?}",
                funct5, aq, rl, funct3, rd, rs1, rs2
            );
        }

        pub fn lr_w(&mut self, aq: bool, rl: bool, rd: Register, rs1: Register) {
            self.gen_instr_ratomic(0b00010, aq, rl, 0b010, rd, rs1, Register::ZERO);
        }

        pub fn sc_w(&mut self, aq: bool, rl: bool, rd: Register, rs1: Register, rs2: Register) {
            self.gen_instr_ratomic(0b00011, aq, rl, 0b010, rd, rs1, rs2);
        }

        pub fn amoswap_w(&mut self, aq: bool, rl: bool, rd: Register, rs1: Register, rs2: Register) {
            self.gen_instr_ratomic(0b00001, aq, rl, 0b010, rd, rs1, rs2);
        }

        pub fn amoadd_w(&mut self, aq: bool, rl: bool, rd: Register, rs1: Register, rs2: Register) {
            self.gen_instr_ratomic(0b00000, aq, rl, 0b010, rd, rs1, rs2);
        }

        pub fn amoxor_w(&mut self, aq: bool, rl: bool, rd: Register, rs1: Register, rs2: Register) {
            self.gen_instr_ratomic(0b00100, aq, rl, 0b010, rd, rs1, rs2);
        }

        pub fn amoand_w(&mut self, aq: bool, rl: bool, rd: Register, rs1: Register, rs2: Register) {
            self.gen_instr_ratomic(0b01100, aq, rl, 0b010, rd, rs1, rs2);
        }

        pub fn amoor_w(&mut self, aq: bool, rl: bool, rd: Register, rs1: Register, rs2: Register) {
            self.gen_instr_ratomic(0b01000, aq, rl, 0b010, rd, rs1, rs2);
        }

        pub fn amomin_w(&mut self, aq: bool, rl: bool, rd: Register, rs1: Register, rs2: Register) {
            self.gen_instr_ratomic(0b10000, aq, rl, 0b010, rd, rs1, rs2);
        }

        pub fn amomax_w(&mut self, aq: bool, rl: bool, rd: Register, rs1: Register, rs2: Register) {
            self.gen_instr_ratomic(0b10100, aq, rl, 0b010, rd, rs1, rs2);
        }

        pub fn amominu_w(&mut self, aq: bool, rl: bool, rd: Register, rs1: Register, rs2: Register) {
            self.gen_instr_ratomic(0b11000, aq, rl, 0b010, rd, rs1, rs2);
        }

        pub fn amomaxu_w(&mut self, aq: bool, rl: bool, rd: Register, rs1: Register, rs2: Register) {
            self.gen_instr_ratomic(0b11100, aq, rl, 0b010, rd, rs1, rs2);
        }

        #[cfg(target_arch = "riscv64")]
        pub fn lr_d(&mut self, aq: bool, rl: bool, rd: Register, rs1: Register) {
            self.gen_instr_ratomic(0b00010, aq, rl, 0b011, rd, rs1, Register::ZERO);
        }

        #[cfg(target_arch = "riscv64")]
        pub fn sc_d(&mut self, aq: bool, rl: bool, rd: Register, rs1: Register, rs2: Register) {
            self.gen_instr_ratomic(0b00011, aq, rl, 0b011, rd, rs1, rs2);
        }

        #[cfg(target_arch = "riscv64")]
        pub fn amoswap_d(&mut self, aq: bool, rl: bool, rd: Register, rs1: Register, rs2: Register) {
            self.gen_instr_ratomic(0b00001, aq, rl, 0b011, rd, rs1, rs2);
        }

        #[cfg(target_arch = "riscv64")]
        pub fn amoadd_d(&mut self, aq: bool, rl: bool, rd: Register, rs1: Register, rs2: Register) {
            self.gen_instr_ratomic(0b00000, aq, rl, 0b011, rd, rs1, rs2);
        }

        #[cfg(target_arch = "riscv64")]
        pub fn amoxor_d(&mut self, aq: bool, rl: bool, rd: Register, rs1: Register, rs2: Register) {
            self.gen_instr_ratomic(0b00100, aq, rl, 0b011, rd, rs1, rs2);
        }

        #[cfg(target_arch = "riscv64")]
        pub fn amoand_d(&mut self, aq: bool, rl: bool, rd: Register, rs1: Register, rs2: Register) {
            self.gen_instr_ratomic(0b01100, aq, rl, 0b011, rd, rs1, rs2);
        }

        #[cfg(target_arch = "riscv64")]
        pub fn amoor_d(&mut self, aq: bool, rl: bool, rd: Register, rs1: Register, rs2: Register) {
            self.gen_instr_ratomic(0b01000, aq, rl, 0b011, rd, rs1, rs2);
        }

        #[cfg(target_arch = "riscv64")]
        pub fn amomin_d(&mut self, aq: bool, rl: bool, rd: Register, rs1: Register, rs2: Register) {
            self.gen_instr_ratomic(0b10000, aq, rl, 0b011, rd, rs1, rs2);
        }

        #[cfg(target_arch = "riscv64")]
        pub fn amomax_d(&mut self, aq: bool, rl: bool, rd: Register, rs1: Register, rs2: Register) {
            self.gen_instr_ratomic(0b10100, aq, rl, 0b011, rd, rs1, rs2);
        }

        #[cfg(target_arch = "riscv64")]
        pub fn amominu_d(&mut self, aq: bool, rl: bool, rd: Register, rs1: Register, rs2: Register) {
            self.gen_instr_ratomic(0b11000, aq, rl, 0b011, rd, rs1, rs2);
        }

        #[cfg(target_arch = "riscv64")]
        pub fn amomaxu_d(&mut self, aq: bool, rl: bool, rd: Register, rs1: Register, rs2: Register) {
            self.gen_instr_ratomic(0b11100, aq, rl, 0b011, rd, rs1, rs2);
        }
    }
}

pub use riscv_a::*;