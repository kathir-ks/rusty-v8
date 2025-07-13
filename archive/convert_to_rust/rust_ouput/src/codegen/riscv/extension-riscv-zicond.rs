// Converted from V8 C++ source files:
// Header: extension-riscv-zicond.h
// Implementation: extension-riscv-zicond.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/codegen/riscv/extension-riscv-zicond.h
pub mod extension_riscv_zicond {
    use crate::codegen::assembler::Assembler;
    use crate::codegen::riscv::base_assembler_riscv::AssemblerRiscvBase;
    use crate::codegen::riscv::register_riscv::Register;

    pub struct AssemblerRISCVZicond {
        base: AssemblerRiscvBase,
    }

    impl AssemblerRISCVZicond {
        pub fn new(assembler: Assembler) -> Self {
            AssemblerRISCVZicond {
                base: AssemblerRiscvBase::new(assembler),
            }
        }

        pub fn czero_eqz(&mut self, rd: Register, rs1: Register, rs2: Register) {
            self.gen_instr_alu_rr(0b0000111, 0b101, rd, rs1, rs2);
        }

        pub fn czero_nez(&mut self, rd: Register, rs1: Register, rs2: Register) {
            self.gen_instr_alu_rr(0b0000111, 0b111, rd, rs1, rs2);
        }

        fn gen_instr_alu_rr(&mut self, opcode: u8, funct3: u8, rd: Register, rs1: Register, rs2: Register) {
           self.base.assembler().emit(opcode as i32 | ((funct3 as i32) << 3) | ((rd.code() as i32) << 7) | ((rs1.code() as i32) << 15) | ((rs2.code() as i32) << 20) | (0b0110011 << 25) as i32)
        }

        pub fn base(&self) -> &AssemblerRiscvBase {
            &self.base
        }

        pub fn base_mut(&mut self) -> &mut AssemblerRiscvBase {
            &mut self.base
        }
    }
}

// src/codegen/riscv/extension-riscv-zicond.cc
