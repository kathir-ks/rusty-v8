// Converted from V8 C++ source files:
// Header: extension-riscv-c.h
// Implementation: extension-riscv-c.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/codegen/riscv/extension-riscv-c.h
pub mod extension_riscv_c {
    use crate::codegen::assembler::assembler::Assembler;
    use crate::codegen::riscv::base_assembler_riscv::BaseAssemblerRiscv;
    use crate::codegen::riscv::constant_riscv_c::*;
    use crate::codegen::riscv::register_riscv::*;
    use std::convert::TryInto;

    pub struct AssemblerRISCVC {
        base: BaseAssemblerRiscv,
    }

    impl AssemblerRISCVC {
        pub fn new(assembler: BaseAssemblerRiscv) -> Self {
            Self { base: assembler }
        }

        // RV64C Standard Extension
        pub fn c_nop(&mut self) {
            self.gen_instr_ci(0b000, C1, zero_reg, 0);
        }

        pub fn c_addi(&mut self, rd: Register, imm6: i8) {
            if rd != zero_reg && imm6 != 0 {
                self.gen_instr_ci(0b000, C1, rd, imm6);
            }
        }

        pub fn c_addi16sp(&mut self, imm10: i16) {
            if imm10 >= -512 && imm10 <= 511 && (imm10 & 0xf) == 0 {
                let uimm6 = ((imm10 & 0x200) >> 4) as u8
                    | (imm10 & 0x10) as u8
                    | ((imm10 & 0x40) >> 3) as u8
                    | ((imm10 & 0x180) >> 6) as u8
                    | ((imm10 & 0x20) >> 5) as u8;
                self.gen_instr_ciu(0b011, C1, sp, uimm6);
            }
        }

        pub fn c_addi4spn(&mut self, rd: Register, uimm10: i16) {
            if uimm10 >= 0 && uimm10 <= 1023 && uimm10 != 0 {
                let uimm8 = ((uimm10 & 0x4) >> 1) as u8
                    | ((uimm10 & 0x8) >> 3) as u8
                    | ((uimm10 & 0x30) << 2) as u8
                    | ((uimm10 & 0x3c0) >> 4) as u8;
                self.gen_instr_ciw(0b000, C0, rd, uimm8);
            }
        }

        pub fn c_li(&mut self, rd: Register, imm6: i8) {
            if rd != zero_reg {
                self.gen_instr_ci(0b010, C1, rd, imm6);
            }
        }

        pub fn c_lui(&mut self, rd: Register, imm6: i8) {
            if rd != zero_reg && rd != sp && imm6 != 0 {
                self.gen_instr_ci(0b011, C1, rd, imm6);
            }
        }

        pub fn c_slli(&mut self, rd: Register, shamt6: u8) {
            if rd != zero_reg && shamt6 != 0 {
                self.gen_instr_ciu(0b000, C2, rd, shamt6);
            }
        }

        pub fn c_lwsp(&mut self, rd: Register, uimm8: u16) {
            if rd != zero_reg && uimm8 <= 255 && (uimm8 & 0x3) == 0 {
                let uimm6 = ((uimm8 & 0x3c) | ((uimm8 & 0xc0) >> 6)) as u8;
                self.gen_instr_ciu(0b010, C2, rd, uimm6);
            }
        }

        pub fn c_jr(&mut self, rs1: Register) {
            if rs1 != zero_reg {
                self.gen_instr_cr(0b1000, C2, rs1, zero_reg);
                self.block_trampoline_pool_for(1);
            }
        }

        pub fn c_mv(&mut self, rd: Register, rs2: Register) {
            if rd != zero_reg && rs2 != zero_reg {
                self.gen_instr_cr(0b1000, C2, rd, rs2);
            }
        }

        pub fn c_ebreak(&mut self) {
            self.gen_instr_cr(0b1001, C2, zero_reg, zero_reg);
        }

        pub fn c_jalr(&mut self, rs1: Register) {
            if rs1 != zero_reg {
                self.gen_instr_cr(0b1001, C2, rs1, zero_reg);
                self.block_trampoline_pool_for(1);
            }
        }

        pub fn c_j(&mut self, imm12: i16) {
            if imm12 >= -2048 && imm12 <= 2047 {
                let uimm11 = ((imm12 & 0x800) >> 1) as u16
                    | ((imm12 & 0x400) >> 4) as u16
                    | ((imm12 & 0x300) >> 1) as u16
                    | ((imm12 & 0x80) >> 3) as u16
                    | ((imm12 & 0x40) >> 1) as u16
                    | ((imm12 & 0x20) >> 5) as u16
                    | ((imm12 & 0x10) << 5) as u16
                    | (imm12 & 0xe) as u16;
                self.gen_instr_cj(0b101, C1, uimm11);
                self.block_trampoline_pool_for(1);
            }
        }

        pub fn c_add(&mut self, rd: Register, rs2: Register) {
            if rd != zero_reg && rs2 != zero_reg {
                self.gen_instr_cr(0b1001, C2, rd, rs2);
            }
        }

        // CA Instructions
        pub fn c_sub(&mut self, rd: Register, rs2: Register) {
            if ((rd.code() & 0b11000) == 0b01000) && ((rs2.code() & 0b11000) == 0b01000) {
                self.gen_instr_ca(0b100011, C1, rd, 0b00, rs2);
            }
        }

        pub fn c_and(&mut self, rd: Register, rs2: Register) {
            if ((rd.code() & 0b11000) == 0b01000) && ((rs2.code() & 0b11000) == 0b01000) {
                self.gen_instr_ca(0b100011, C1, rd, 0b11, rs2);
            }
        }

        pub fn c_xor(&mut self, rd: Register, rs2: Register) {
            if ((rd.code() & 0b11000) == 0b01000) && ((rs2.code() & 0b11000) == 0b01000) {
                self.gen_instr_ca(0b100011, C1, rd, 0b01, rs2);
            }
        }

        pub fn c_or(&mut self, rd: Register, rs2: Register) {
            if ((rd.code() & 0b11000) == 0b01000) && ((rs2.code() & 0b11000) == 0b01000) {
                self.gen_instr_ca(0b100011, C1, rd, 0b10, rs2);
            }
        }

        pub fn c_swsp(&mut self, rs2: Register, uimm8: u16) {
            if uimm8 <= 255 && (uimm8 & 0x3) == 0 {
                let uimm6 = ((uimm8 & 0x3c) | ((uimm8 & 0xc0) >> 6)) as u8;
                self.gen_instr_css(0b110, C2, rs2, uimm6);
            }
        }

        pub fn c_lw(&mut self, rd: Register, rs1: Register, uimm7: u16) {
            if ((rd.code() & 0b11000) == 0b01000)
                && ((rs1.code() & 0b11000) == 0b01000)
                && uimm7 <= 127
                && (uimm7 & 0x3) == 0
            {
                let uimm5 = (((uimm7 & 0x4) >> 1) | ((uimm7 & 0x40) >> 6) | ((uimm7 & 0x38) >> 1))
                    as u8;
                self.gen_instr_cl(0b010, C0, rd, rs1, uimm5);
            }
        }

        pub fn c_sw(&mut self, rs2: Register, rs1: Register, uimm7: u16) {
            if ((rs2.code() & 0b11000) == 0b01000)
                && ((rs1.code() & 0b11000) == 0b01000)
                && uimm7 <= 127
                && (uimm7 & 0x3) == 0
            {
                let uimm5 = (((uimm7 & 0x4) >> 1) | ((uimm7 & 0x40) >> 6) | ((uimm7 & 0x38) >> 1))
                    as u8;
                self.gen_instr_cs(0b110, C0, rs2, rs1, uimm5);
            }
        }

        pub fn c_bnez(&mut self, rs1: Register, imm9: i16) {
            if ((rs1.code() & 0b11000) == 0b01000) && imm9 >= -256 && imm9 <= 255 {
                let uimm8 = ((imm9 & 0x20) >> 5) as u8
                    | (imm9 & 0x6) as u8
                    | ((imm9 & 0xc0) >> 3) as u8
                    | ((imm9 & 0x18) << 2) as u8
                    | ((imm9 & 0x100) >> 1) as u8;
                self.gen_instr_cb(0b111, C1, rs1, uimm8);
            }
        }

        pub fn c_beqz(&mut self, rs1: Register, imm9: i16) {
            if ((rs1.code() & 0b11000) == 0b01000) && imm9 >= -256 && imm9 <= 255 {
                let uimm8 = ((imm9 & 0x20) >> 5) as u8
                    | (imm9 & 0x6) as u8
                    | ((imm9 & 0xc0) >> 3) as u8
                    | ((imm9 & 0x18) << 2) as u8
                    | ((imm9 & 0x100) >> 1) as u8;
                self.gen_instr_cb(0b110, C1, rs1, uimm8);
            }
        }

        pub fn c_srli(&mut self, rs1: Register, shamt6: i8) {
            if ((rs1.code() & 0b11000) == 0b01000) && shamt6 >= -32 && shamt6 <= 31 {
                self.gen_instr_cba(0b100, 0b00, C1, rs1, shamt6);
            }
        }

        pub fn c_srai(&mut self, rs1: Register, shamt6: i8) {
            if ((rs1.code() & 0b11000) == 0b01000) && shamt6 >= -32 && shamt6 <= 31 {
                self.gen_instr_cba(0b100, 0b01, C1, rs1, shamt6);
            }
        }

        pub fn c_andi(&mut self, rs1: Register, imm6: i8) {
            if ((rs1.code() & 0b11000) == 0b01000) && imm6 >= -32 && imm6 <= 31 {
                self.gen_instr_cba(0b100, 0b10, C1, rs1, imm6);
            }
        }

        pub fn c_fld(&mut self, rd: FPURegister, rs1: Register, uimm8: u16) {
            if ((rd.code() & 0b11000) == 0b01000)
                && ((rs1.code() & 0b11000) == 0b01000)
                && uimm8 <= 255
                && (uimm8 & 0x7) == 0
            {
                let uimm5 = (((uimm8 & 0x38) >> 1) | ((uimm8 & 0xc0) >> 6)) as u8;
                self.gen_instr_cl(0b001, C0, rd, rs1, uimm5);
            }
        }

        pub fn c_fsd(&mut self, rs2: FPURegister, rs1: Register, uimm8: u16) {
            if ((rs2.code() & 0b11000) == 0b01000)
                && ((rs1.code() & 0b11000) == 0b01000)
                && uimm8 <= 255
                && (uimm8 & 0x7) == 0
            {
                let uimm5 = (((uimm8 & 0x38) >> 1) | ((uimm8 & 0xc0) >> 6)) as u8;
                self.gen_instr_cs(0b101, C0, rs2, rs1, uimm5);
            }
        }

        pub fn c_fldsp(&mut self, rd: FPURegister, uimm9: u16) {
            if uimm9 <= 511 && (uimm9 & 0x7) == 0 {
                let uimm6 = ((uimm9 & 0x38) | ((uimm9 & 0x1c0) >> 6)) as u8;
                self.gen_instr_ciu(0b001, C2, rd, uimm6);
            }
        }

        pub fn c_fsdsp(&mut self, rs2: FPURegister, uimm9: u16) {
            if uimm9 <= 511 && (uimm9 & 0x7) == 0 {
                let uimm6 = ((uimm9 & 0x38) | ((uimm9 & 0x1c0) >> 6)) as u8;
                self.gen_instr_css(0b101, C2, rs2, uimm6);
            }
        }

        #[cfg(target_arch = "riscv64")]
        pub fn c_ld(&mut self, rd: Register, rs1: Register, uimm8: u16) {
            if ((rd.code() & 0b11000) == 0b01000)
                && ((rs1.code() & 0b11000) == 0b01000)
                && uimm8 <= 255
                && (uimm8 & 0x7) == 0
            {
                let uimm5 = (((uimm8 & 0x38) >> 1) | ((uimm8 & 0xc0) >> 6)) as u8;
                self.gen_instr_cl(0b011, C0, rd, rs1, uimm5);
            }
        }

        #[cfg(target_arch = "riscv64")]
        pub fn c_sd(&mut self, rs2: Register, rs1: Register, uimm8: u16) {
            if ((rs2.code() & 0b11000) == 0b01000)
                && ((rs1.code() & 0b11000) == 0b01000)
                && uimm8 <= 255
                && (uimm8 & 0x7) == 0
            {
                let uimm5 = (((uimm8 & 0x38) >> 1) | ((uimm8 & 0xc0) >> 6)) as u8;
                self.gen_instr_cs(0b111, C0, rs2, rs1, uimm5);
            }
        }

        #[cfg(target_arch = "riscv64")]
        pub fn c_subw(&mut self, rd: Register, rs2: Register) {
            if ((rd.code() & 0b11000) == 0b01000) && ((rs2.code() & 0b11000) == 0b01000) {
                self.gen_instr_ca(0b100111, C1, rd, 0b00, rs2);
            }
        }

        #[cfg(target_arch = "riscv64")]
        pub fn c_addw(&mut self, rd: Register, rs2: Register) {
            if ((rd.code() & 0b11000) == 0b01000) && ((rs2.code() & 0b11000) == 0b01000) {
                self.gen_instr_ca(0b100111, C1, rd, 0b01, rs2);
            }
        }

        #[cfg(target_arch = "riscv64")]
        pub fn c_addiw(&mut self, rd: Register, imm6: i8) {
            if rd != zero_reg {
                self.gen_instr_ci(0b001, C1, rd, imm6);
            }
        }

        #[cfg(target_arch = "riscv64")]
        pub fn c_ldsp(&mut self, rd: Register, uimm9: u16) {
            if rd != zero_reg && uimm9 <= 511 && (uimm9 & 0x7) == 0 {
                let uimm6 = ((uimm9 & 0x38) | ((uimm9 & 0x1c0) >> 6)) as u8;
                self.gen_instr_ciu(0b011, C2, rd, uimm6);
            }
        }

        #[cfg(target_arch = "riscv64")]
        pub fn c_sdsp(&mut self, rs2: Register, uimm9: u16) {
            if uimm9 <= 511 && (uimm9 & 0x7) == 0 {
                let uimm6 = ((uimm9 & 0x38) | ((uimm9 & 0x1c0) >> 6)) as u8;
                self.gen_instr_css(0b111, C2, rs2, uimm6);
            }
        }

        fn gen_instr_ci(&mut self, op: u8, c: u8, rd: Register, imm6: i8) {
            let instruction: u16 = ((op as u16) << 13)
                | ((rd.code() as u16) << 8)
                | ((imm6 as i16 & 0x3f) as u16) << 2
                | (c as u16);
            self.emit16(instruction);
        }

        fn gen_instr_ciu(&mut self, op: u8, c: u8, rd: Register, uimm6: u8) {
            let instruction: u16 = ((op as u16) << 13)
                | ((rd.code() as u16) << 8)
                | ((uimm6 as u16 & 0x3f) << 2)
                | (c as u16);
            self.emit16(instruction);
        }

        fn gen_instr_ciw(&mut self, op: u8, c: u8, rd: Register, uimm8: u8) {
            let instruction: u16 = ((op as u16) << 13)
                | ((rd.code() as u16 & 0x7) << 5)
                | ((uimm8 as u16 & 0xff) << 2)
                | (c as u16);
            self.emit16(instruction);
        }

        fn gen_instr_cr(&mut self, op: u8, c: u8, rd: Register, rs2: Register) {
            let instruction: u16 = ((op as u16) << 13)
                | ((rd.code() as u16) << 8)
                | ((rs2.code() as u16) << 3)
                | (c as u16);
            self.emit16(instruction);
        }

        fn gen_instr_cj(&mut self, op: u8, c: u8, imm11: u16) {
            let instruction: u16 = ((op as u16) << 13) | ((imm11 & 0x7ff) << 2) | (c as u16);
            self.emit16(instruction);
        }

        fn gen_instr_cb(&mut self, op: u8, c: u8, rs1: Register, uimm8: u8) {
            let instruction: u16 = ((op as u16) << 13)
                | ((rs1.code() as u16 & 0x7) << 9)
                | ((uimm8 as u16 & 0xff) << 2)
                | (c as u16);
            self.emit16(instruction);
        }

        fn gen_instr_css(&mut self, op: u8, c: u8, rs2: FPURegister, uimm6: u8) {
            let instruction: u16 = ((op as u16) << 13)
                | ((rs2.code() as u16) << 8)
                | ((uimm6 as u16 & 0x3f) << 2)
                | (c as u16);
            self.emit16(instruction);
        }

        fn gen_instr_cl(&mut self, op: u8, c: u8, rd: FPURegister, rs1: Register, uimm5: u8) {
            let instruction: u16 = ((op as u16) << 13)
                | ((rd.code() as u16) << 8)
                | ((rs1.code() as u16 & 0x7) << 5)
                | ((uimm5 as u16 & 0x1f) << 2)
                | (c as u16);
            self.emit16(instruction);
        }

        fn gen_instr_cs(&mut self, op: u8, c: u8, rs2: Register, rs1: Register, uimm5: u8) {
            let instruction: u16 = ((op as u16) << 13)
                | ((rs2.code() as u16 & 0x7) << 10)
                | ((rs1.code() as u16 & 0x7) << 7)
                | ((uimm5 as u16 & 0x1f) << 2)
                | (c as u16);
            self.emit16(instruction);
        }

        fn gen_instr_ca(&mut self, op: u8, c: u8, rd: Register, funct2: u8, rs2: Register) {
            let instruction: u16 = ((op as u16) << 9)
                | ((rd.code() as u16 & 0x7) << 4)
                | ((funct2 as u16) << 2)
                | ((rs2.code() as u16 & 0x7) << 7)
                | (c as u16);
            self.emit16(instruction);
        }

        fn gen_instr_cba(&mut self, op: u8, funct2: u8, c: u8, rs1: Register, imm6: i8) {
            let instruction: u16 = ((op as u16) << 13)
                | ((funct2 as u16) << 10)
                | ((rs1.code() as u16 & 0x7) << 7)
                | ((imm6 as i16 & 0x3f) << 2)
                | (c as u16);
            self.emit16(instruction);
        }

        fn emit16(&mut self, instruction: u16) {
            self.base.emit(instruction as i32);
        }

        fn block_trampoline_pool_for(&mut self, size: usize) {
            self.base.block_trampoline_pool_for(size);
        }

        fn branch_offset_helper(&self, l: *mut u8, offset_size: OffsetSize) -> i32 {
            self.base.branch_offset_helper(l, offset_size)
        }

        pub fn cjump_offset(&self, l: *mut u8) -> i16 {
            self.branch_offset_helper(l, OffsetSize::kOffset11) as i16
        }
        pub fn cbranch_offset(&self, l: *mut u8) -> i32 {
            self.branch_offset_helper(l, OffsetSize::kOffset9)
        }
        pub fn c_j_label(&mut self, l: *mut u8) {
            let offset = self.cjump_offset(l);
            self.c_j(offset)
        }
        pub fn c_bnez_label(&mut self, rs1: Register, l: *mut u8) {
            let offset = self.cbranch_offset(l);
            self.c_bnez(rs1, offset as i16)
        }
        pub fn c_beqz_label(&mut self, rs1: Register, l: *mut u8) {
            let offset = self.cbranch_offset(l);
            self.c_beqz(rs1, offset as i16)
        }

        pub fn cjump_offset_label(&self, l: *mut u8) -> i16 {
            self.branch_offset_helper(l, OffsetSize::kOffset11).try_into().unwrap()
        }

        pub fn cbranch_offset_label(&self, l: *mut u8) -> i32 {
            self.branch_offset_helper(l, OffsetSize::kOffset9)
        }

        pub fn c_j_label2(&mut self, l: *mut u8) {
            self.c_j(self.cjump_offset_label(l))
        }

        pub fn c_bnez_label2(&mut self, rs1: Register, l: *mut u8) {
            self.c_bnez(rs1, self.cbranch_offset_label(l) as i16)
        }

        pub fn c_beqz_label2(&mut self, rs1: Register, l: *mut u8) {
            self.c_beqz(rs1, self.cbranch_offset_label(l) as i16)
        }

        pub fn c_j_label3(&mut self, l: *mut u8) {
            self.c_j(self.cjump_offset(l))
        }
        pub fn c_bnez_label3(&mut self, rs1: Register, l: *mut u8) {
            self.c_bnez(rs1, self.cbranch_offset(l) as i16)
        }
        pub fn c_beqz_label3(&mut self, rs1: Register, l: *mut u8) {
            self.c_beqz(rs1, self.cbranch_offset(l) as i16)
        }

        pub fn cjump_offset_label4(&self, l: *mut u8) -> i16 {
            self.branch_offset_helper(l, OffsetSize::kOffset11) as i16
        }

        pub fn cbranch_offset_label4(&self, l: *mut u8) -> i32 {
            self.branch_offset_helper(l, OffsetSize::kOffset9)
        }

        pub fn c_j_label4(&mut self, l: *mut u8) {
            self.c_j(self.cjump_offset_label4(l))
        }
        pub fn c_bnez_label4(&mut self, rs1: Register, l: *mut u8) {
            self.c_bnez(rs1, self.cbranch_offset_label4(l) as i16)
        }
        pub fn c_beqz_label4(&mut self, rs1: Register, l: *mut u8) {
            self.c_beqz(rs1, self.cbranch_offset_label4(l) as i16)
        }

        pub fn cjump_offset_label5(&self, l: *mut u8) -> i16 {
            self.branch_offset_helper(l, OffsetSize::kOffset11) as i16
        }

        pub fn cbranch_offset_label5(&self, l: *mut u8) -> i32 {
            self.branch_offset_helper(l, OffsetSize::kOffset9)
        }
        pub fn c_j_label5(&mut self, l: *mut u8) {
            self.c_j(self.cjump_offset_label5(l))
        }

        pub fn c_bnez_label5(&mut self, rs1: Register, l: *mut u8) {
            self.c_bnez(rs1, self.cbranch_offset_label5(l) as i16)
        }

        pub fn c_beqz_label5(&mut self, rs1: Register, l: *mut u8) {
            self.c_beqz(rs1, self.cbranch_offset_label5(l) as i16)
        }
        pub fn cjump_offset_label6(&self, l: *mut u8) -> i16 {
            self.branch_offset_helper(l, OffsetSize::kOffset11) as i16
        }

        pub fn cbranch_offset_label6(&self, l: *mut u8) -> i32 {
            self.branch_offset_helper(l, OffsetSize::kOffset9)
        }

        pub fn c_j_label6(&mut self, l: *mut u8) {
            self.c_j(self.cjump_offset_label6(l))
        }

        pub fn c_bnez_label6(&mut self, rs1: Register, l: *mut u8) {
            self.c_bnez(rs1, self.cbranch_offset_label6(l) as i16)
        }

        pub fn c_beqz_label6(&mut self, rs1: Register, l: *mut u8) {
            self.c_beqz(rs1, self.cbranch_offset_label6(l) as i16)
        }
        pub fn CJumpOffset(&self, instr: i32) -> i32 {
            let imm12 = ((instr & 0x4) << 3)
                | ((instr & 0x38) >> 2)
                | ((instr & 0x40) << 1)
                | ((instr & 0x80) >> 1)
                | ((instr & 0x100) << 2)
                | ((instr & 0x600) >> 1)
                | ((instr & 0x800) >> 7)
                | ((instr & 0x1000) >> 1);
            let imm12 = imm12 << 20 >> 20;
            imm12
        }

        pub fn Is
