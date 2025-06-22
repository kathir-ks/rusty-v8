// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod riscv_i {
    use crate::codegen::assembler::Assembler;
    use crate::codegen::riscv::base_assembler_riscv::AssemblerRiscvBase;
    use crate::codegen::riscv::constant_riscv_i::Instr;
    use crate::codegen::riscv::register_riscv::Register;

    pub struct AssemblerRISCVI {
        base: AssemblerRiscvBase,
    }

    impl AssemblerRISCVI {
        pub fn new(assembler: AssemblerRiscvBase) -> Self {
            AssemblerRISCVI { base: assembler }
        }

        pub fn lui(&mut self, rd: Register, imm20: i32) {
            // Implementation details depend on the underlying assembler's instruction encoding.
            // Placeholder for now.
            println!("LUI rd: {:?}, imm20: {}", rd, imm20);
        }

        pub fn auipc(&mut self, rd: Register, imm20: i32) {
            println!("AUIPC rd: {:?}, imm20: {}", rd, imm20);
        }

        pub fn jal(&mut self, rd: Register, imm20: i32) {
            println!("JAL rd: {:?}, imm20: {}", rd, imm20);
        }

        pub fn jalr(&mut self, rd: Register, rs1: Register, imm12: i16) {
            println!("JALR rd: {:?}, rs1: {:?}, imm12: {}", rd, rs1, imm12);
        }

        pub fn beq(&mut self, rs1: Register, rs2: Register, imm12: i16) {
            println!("BEQ rs1: {:?}, rs2: {:?}, imm12: {}", rs1, rs2, imm12);
        }

        pub fn bne(&mut self, rs1: Register, rs2: Register, imm12: i16) {
            println!("BNE rs1: {:?}, rs2: {:?}, imm12: {}", rs1, rs2, imm12);
        }

        pub fn blt(&mut self, rs1: Register, rs2: Register, imm12: i16) {
            println!("BLT rs1: {:?}, rs2: {:?}, imm12: {}", rs1, rs2, imm12);
        }

        pub fn bge(&mut self, rs1: Register, rs2: Register, imm12: i16) {
            println!("BGE rs1: {:?}, rs2: {:?}, imm12: {}", rs1, rs2, imm12);
        }

        pub fn bltu(&mut self, rs1: Register, rs2: Register, imm12: i16) {
            println!("BLTU rs1: {:?}, rs2: {:?}, imm12: {}", rs1, rs2, imm12);
        }

        pub fn bgeu(&mut self, rs1: Register, rs2: Register, imm12: i16) {
            println!("BGEU rs1: {:?}, rs2: {:?}, imm12: {}", rs1, rs2, imm12);
        }

        pub fn lb(&mut self, rd: Register, rs1: Register, imm12: i16) {
            println!("LB rd: {:?}, rs1: {:?}, imm12: {}", rd, rs1, imm12);
        }

        pub fn lh(&mut self, rd: Register, rs1: Register, imm12: i16) {
            println!("LH rd: {:?}, rs1: {:?}, imm12: {}", rd, rs1, imm12);
        }

        pub fn lw(&mut self, rd: Register, rs1: Register, imm12: i16) {
            println!("LW rd: {:?}, rs1: {:?}, imm12: {}", rd, rs1, imm12);
        }

        pub fn lbu(&mut self, rd: Register, rs1: Register, imm12: i16) {
            println!("LBU rd: {:?}, rs1: {:?}, imm12: {}", rd, rs1, imm12);
        }

        pub fn lhu(&mut self, rd: Register, rs1: Register, imm12: i16) {
            println!("LHU rd: {:?}, rs1: {:?}, imm12: {}", rd, rs1, imm12);
        }

        pub fn sb(&mut self, source: Register, base: Register, imm12: i16) {
            println!("SB source: {:?}, base: {:?}, imm12: {}", source, base, imm12);
        }

        pub fn sh(&mut self, source: Register, base: Register, imm12: i16) {
            println!("SH source: {:?}, base: {:?}, imm12: {}", source, base, imm12);
        }

        pub fn sw(&mut self, source: Register, base: Register, imm12: i16) {
            println!("SW source: {:?}, base: {:?}, imm12: {}", source, base, imm12);
        }

        pub fn addi(&mut self, rd: Register, rs1: Register, imm12: i16) {
            println!("ADDI rd: {:?}, rs1: {:?}, imm12: {}", rd, rs1, imm12);
        }

        pub fn slti(&mut self, rd: Register, rs1: Register, imm12: i16) {
            println!("SLTI rd: {:?}, rs1: {:?}, imm12: {}", rd, rs1, imm12);
        }

        pub fn sltiu(&mut self, rd: Register, rs1: Register, imm12: i16) {
            println!("SLTIU rd: {:?}, rs1: {:?}, imm12: {}", rd, rs1, imm12);
        }

        pub fn xori(&mut self, rd: Register, rs1: Register, imm12: i16) {
            println!("XORI rd: {:?}, rs1: {:?}, imm12: {}", rd, rs1, imm12);
        }

        pub fn ori(&mut self, rd: Register, rs1: Register, imm12: i16) {
            println!("ORI rd: {:?}, rs1: {:?}, imm12: {}", rd, rs1, imm12);
        }

        pub fn andi(&mut self, rd: Register, rs1: Register, imm12: i16) {
            println!("ANDI rd: {:?}, rs1: {:?}, imm12: {}", rd, rs1, imm12);
        }

        pub fn slli(&mut self, rd: Register, rs1: Register, shamt: u8) {
            println!("SLLI rd: {:?}, rs1: {:?}, shamt: {}", rd, rs1, shamt);
        }

        pub fn srli(&mut self, rd: Register, rs1: Register, shamt: u8) {
            println!("SRLI rd: {:?}, rs1: {:?}, shamt: {}", rd, rs1, shamt);
        }

        pub fn srai(&mut self, rd: Register, rs1: Register, shamt: u8) {
            println!("SRAI rd: {:?}, rs1: {:?}, shamt: {}", rd, rs1, shamt);
        }

        pub fn add(&mut self, rd: Register, rs1: Register, rs2: Register) {
            println!("ADD rd: {:?}, rs1: {:?}, rs2: {:?}", rd, rs1, rs2);
        }

        pub fn sub(&mut self, rd: Register, rs1: Register, rs2: Register) {
            println!("SUB rd: {:?}, rs1: {:?}, rs2: {:?}", rd, rs1, rs2);
        }

        pub fn sll(&mut self, rd: Register, rs1: Register, rs2: Register) {
            println!("SLL rd: {:?}, rs1: {:?}, rs2: {:?}", rd, rs1, rs2);
        }

        pub fn slt(&mut self, rd: Register, rs1: Register, rs2: Register) {
            println!("SLT rd: {:?}, rs1: {:?}, rs2: {:?}", rd, rs1, rs2);
        }

        pub fn sltu(&mut self, rd: Register, rs1: Register, rs2: Register) {
            println!("SLTU rd: {:?}, rs1: {:?}, rs2: {:?}", rd, rs1, rs2);
        }

        pub fn xor_(&mut self, rd: Register, rs1: Register, rs2: Register) {
            println!("XOR rd: {:?}, rs1: {:?}, rs2: {:?}", rd, rs1, rs2);
        }

        pub fn srl(&mut self, rd: Register, rs1: Register, rs2: Register) {
            println!("SRL rd: {:?}, rs1: {:?}, rs2: {:?}", rd, rs1, rs2);
        }

        pub fn sra(&mut self, rd: Register, rs1: Register, rs2: Register) {
            println!("SRA rd: {:?}, rs1: {:?}, rs2: {:?}", rd, rs1, rs2);
        }

        pub fn or_(&mut self, rd: Register, rs1: Register, rs2: Register) {
            println!("OR rd: {:?}, rs1: {:?}, rs2: {:?}", rd, rs1, rs2);
        }

        pub fn and_(&mut self, rd: Register, rs1: Register, rs2: Register) {
            println!("AND rd: {:?}, rs1: {:?}, rs2: {:?}", rd, rs1, rs2);
        }

        pub fn nor(&mut self, rd: Register, rs: Register, rt: Register) {
            self.or_(rd, rs, rt);
            self.not_(rd, rd);
        }

        pub fn fence(&mut self, pred: u8, succ: u8) {
            println!("FENCE pred: {}, succ: {}", pred, succ);
        }

        pub fn fence_tso(&mut self) {
            //Placeholder, no direct equivalent
            println!("FENCE_TSO");
        }

        pub fn ecall(&mut self) {
            println!("ECALL");
        }

        pub fn ebreak(&mut self) {
            println!("EBREAK");
        }

        pub fn sync(&mut self) {
            self.fence(0b1111, 0b1111);
        }

        pub fn unimp(&mut self) {
            println!("UNIMP");
        }

        pub fn jump_offset(_instr: Instr) -> i32 {
            //Placeholder
            0
        }

        pub fn auipc_offset(_instr: Instr) -> i32 {
            //Placeholder
            0
        }

        pub fn jalr_offset(_instr: Instr) -> i32 {
            //Placeholder
            0
        }

        pub fn load_offset(_instr: Instr) -> i32 {
            //Placeholder
            0
        }

        pub fn is_branch(_instr: Instr) -> bool {
            //Placeholder
            false
        }

        pub fn is_nop(_instr: Instr) -> bool {
            //Placeholder
            false
        }

        pub fn is_jump(_instr: Instr) -> bool {
            //Placeholder
            false
        }

        pub fn is_jal(_instr: Instr) -> bool {
            //Placeholder
            false
        }

        pub fn is_jalr(_instr: Instr) -> bool {
            //Placeholder
            false
        }

        pub fn is_lui(_instr: Instr) -> bool {
            //Placeholder
            false
        }

        pub fn is_auipc(_instr: Instr) -> bool {
            //Placeholder
            false
        }

        pub fn is_addi(_instr: Instr) -> bool {
            //Placeholder
            false
        }

        pub fn is_ori(_instr: Instr) -> bool {
            //Placeholder
            false
        }

        pub fn is_slli(_instr: Instr) -> bool {
            //Placeholder
            false
        }

        pub fn is_lw(_instr: Instr) -> bool {
            //Placeholder
            false
        }

        fn branch_offset_helper(&self, l: &Label, offset_size: OffsetSize) -> i32 {
            // Placeholder implementation.  Needs proper assembler and label management.
            println!(
                "branch_offset_helper: Label: {:?}, OffsetSize: {:?}",
                l, offset_size
            );
            0
        }

        pub fn beq_label(&mut self, rs1: Register, rs2: Register, l: &Label) {
            self.beq(rs1, rs2, self.branch_offset(l) as i16);
        }
        pub fn bne_label(&mut self, rs1: Register, rs2: Register, l: &Label) {
            self.bne(rs1, rs2, self.branch_offset(l) as i16);
        }
        pub fn blt_label(&mut self, rs1: Register, rs2: Register, l: &Label) {
            self.blt(rs1, rs2, self.branch_offset(l) as i16);
        }
        pub fn bge_label(&mut self, rs1: Register, rs2: Register, l: &Label) {
            self.bge(rs1, rs2, self.branch_offset(l) as i16);
        }
        pub fn bltu_label(&mut self, rs1: Register, rs2: Register, l: &Label) {
            self.bltu(rs1, rs2, self.branch_offset(l) as i16);
        }
        pub fn bgeu_label(&mut self, rs1: Register, rs2: Register, l: &Label) {
            self.bgeu(rs1, rs2, self.branch_offset(l) as i16);
        }
        pub fn beqz(&mut self, rs: Register, imm13: i16) {
            self.beq(rs, zero_reg(), imm13);
        }

        pub fn beqz_label(&mut self, rs1: Register, l: &Label) {
            self.beqz(rs1, self.branch_offset(l) as i16);
        }

        pub fn bnez(&mut self, rs: Register, imm13: i16) {
            self.bne(rs, zero_reg(), imm13);
        }

        pub fn bnez_label(&mut self, rs1: Register, l: &Label) {
            self.bnez(rs1, self.branch_offset(l) as i16);
        }

        pub fn blez(&mut self, rs: Register, imm13: i16) {
            self.bge(zero_reg(), rs, imm13);
        }

        pub fn blez_label(&mut self, rs1: Register, l: &Label) {
            self.blez(rs1, self.branch_offset(l) as i16);
        }

        pub fn bgez(&mut self, rs: Register, imm13: i16) {
            self.bge(rs, zero_reg(), imm13);
        }

        pub fn bgez_label(&mut self, rs1: Register, l: &Label) {
            self.bgez(rs1, self.branch_offset(l) as i16);
        }

        pub fn bltz(&mut self, rs: Register, imm13: i16) {
            self.blt(rs, zero_reg(), imm13);
        }

        pub fn bltz_label(&mut self, rs1: Register, l: &Label) {
            self.bltz(rs1, self.branch_offset(l) as i16);
        }

        pub fn bgtz(&mut self, rs: Register, imm13: i16) {
            self.blt(zero_reg(), rs, imm13);
        }

        pub fn bgtz_label(&mut self, rs1: Register, l: &Label) {
            self.bgtz(rs1, self.branch_offset(l) as i16);
        }

        pub fn bgt(&mut self, rs1: Register, rs2: Register, imm13: i16) {
            self.blt(rs2, rs1, imm13);
        }

        pub fn bgt_label(&mut self, rs1: Register, rs2: Register, l: &Label) {
            self.bgt(rs1, rs2, self.branch_offset(l) as i16);
        }

        pub fn ble(&mut self, rs1: Register, rs2: Register, imm13: i16) {
            self.bge(rs2, rs1, imm13);
        }

        pub fn ble_label(&mut self, rs1: Register, rs2: Register, l: &Label) {
            self.ble(rs1, rs2, self.branch_offset(l) as i16);
        }

        pub fn bgtu(&mut self, rs1: Register, rs2: Register, imm13: i16) {
            self.bltu(rs2, rs1, imm13);
        }

        pub fn bgtu_label(&mut self, rs1: Register, rs2: Register, l: &Label) {
            self.bgtu(rs1, rs2, self.branch_offset(l) as i16);
        }

        pub fn bleu(&mut self, rs1: Register, rs2: Register, imm13: i16) {
            self.bgeu(rs2, rs1, imm13);
        }

        pub fn bleu_label(&mut self, rs1: Register, rs2: Register, l: &Label) {
            self.bleu(rs1, rs2, self.branch_offset(l) as i16);
        }

        pub fn j(&mut self, imm21: i32) {
            self.jal(zero_reg(), imm21);
        }

        pub fn j_label(&mut self, l: &Label) {
            self.j(self.jump_offset(l));
        }

        pub fn b_label(&mut self, l: &Label) {
            self.j_label(l);
        }

        pub fn jal_imm21(&mut self, imm21: i32) {
            self.jal(ra(), imm21);
        }

        pub fn jal_label(&mut self, l: &Label) {
            self.jal(self.jump_offset(l));
        }

        pub fn jr(&mut self, rs: Register) {
            self.jalr(zero_reg(), rs, 0);
        }

        pub fn jr_imm12(&mut self, rs: Register, imm12: i32) {
            self.jalr(zero_reg(), rs, imm12 as i16);
        }

        pub fn jalr_imm12(&mut self, rs: Register, imm12: i32) {
            self.jalr(ra(), rs, imm12 as i16);
        }

        pub fn jalr_reg(&mut self, rs: Register) {
            self.jalr(ra(), rs, 0);
        }

        pub fn ret(&mut self) {
            self.jalr(zero_reg(), ra(), 0);
        }

        pub fn call(&mut self, offset: i32) {
            self.auipc(
                ra(),
                (offset >> 12) + (((offset & 0x800) >> 11) as i32),
            );
            self.jalr(ra(), ra(), (offset << 20 >> 20) as i16);
        }

        pub fn mv(&mut self, rd: Register, rs: Register) {
            self.addi(rd, rs, 0);
        }

        pub fn not_(&mut self, rd: Register, rs: Register) {
            self.xori(rd, rs, -1);
        }

        pub fn neg(&mut self, rd: Register, rs: Register) {
            self.sub(rd, zero_reg(), rs);
        }

        pub fn seqz(&mut self, rd: Register, rs: Register) {
            self.sltiu(rd, rs, 1);
        }

        pub fn snez(&mut self, rd: Register, rs: Register) {
            self.sltu(rd, zero_reg(), rs);
        }

        pub fn sltz(&mut self, rd: Register, rs: Register) {
            self.slt(rd, rs, zero_reg());
        }

        pub fn sgtz(&mut self, rd: Register, rs: Register) {
            self.slt(rd, zero_reg(), rs);
        }

        #[cfg(target_arch = "riscv64")]
        pub fn lwu(&mut self, rd: Register, rs1: Register, imm12: i16) {
            println!("LWU rd: {:?}, rs1: {:?}, imm12: {}", rd, rs1, imm12);
        }

        #[cfg(target_arch = "riscv64")]
        pub fn ld(&mut self, rd: Register, rs1: Register, imm12: i16) {
            println!("LD rd: {:?}, rs1: {:?}, imm12: {}", rd, rs1, imm12);
        }

        #[cfg(target_arch = "riscv64")]
        pub fn sd(&mut self, source: Register, base: Register, imm12: i16) {
            println!(
                "SD source: {:?}, base: {:?}, imm12: {}",
                source, base, imm12
            );
        }

        #[cfg(target_arch = "riscv64")]
        pub fn addiw(&mut self, rd: Register, rs1: Register, imm12: i16) {
            println!("ADDIW rd: {:?}, rs1: {:?}, imm12: {}", rd, rs1, imm12);
        }

        #[cfg(target_arch = "riscv64")]
        pub fn slliw(&mut self, rd: Register, rs1: Register, shamt: u8) {
            println!("SLLIW rd: {:?}, rs1: {:?}, shamt: {}", rd, rs1, shamt);
        }

        #[cfg(target_arch = "riscv64")]
        pub fn srliw(&mut self, rd: Register, rs1: Register, shamt: u8) {
            println!("SRLIW rd: {:?}, rs1: {:?}, shamt: {}", rd, rs1, shamt);
        }

        #[cfg(target_arch = "riscv64")]
        pub fn sraiw(&mut self, rd: Register, rs1: Register, shamt: u8) {
            println!("SRAIW rd: {:?}, rs1: {:?}, shamt: {}", rd, rs1, shamt);
        }

        #[cfg(target_arch = "riscv64")]
        pub fn addw(&mut self, rd: Register, rs1: Register, rs2: Register) {
            println!("ADDW rd: {:?}, rs1: {:?}, rs2: {:?}", rd, rs1, rs2);
        }

        #[cfg(target_arch = "riscv64")]
        pub fn subw(&mut self, rd: Register, rs1: Register, rs2: Register) {
            println!("SUBW rd: {:?}, rs1: {:?}, rs2: {:?}", rd, rs1, rs2);
        }

        #[cfg(target_arch = "riscv64")]
        pub fn sllw(&mut self, rd: Register, rs1: Register, rs2: Register) {
            println!("SLLW rd: {:?}, rs1: {:?}, rs2: {:?}", rd, rs1, rs2);
        }

        #[cfg(target_arch = "riscv64")]
        pub fn srlw(&mut self, rd: Register, rs1: Register, rs2: Register) {
            println!("SRLW rd: {:?}, rs1: {:?}, rs2: {:?}", rd, rs1, rs2);
        }

        #[cfg(target_arch = "riscv64")]
        pub fn sraw(&mut self, rd: Register, rs1: Register, rs2: Register) {
            println!("SRAW rd: {:?}, rs1: {:?}, rs2: {:?}", rd, rs1, rs2);
        }

        #[cfg(target_arch = "riscv64")]
        pub fn negw(&mut self, rd: Register, rs: Register) {
            self.subw(rd, zero_reg(), rs);
        }

        #[cfg(target_arch = "riscv64")]
        pub fn sext_w(&mut self, rd: Register, rs: Register) {
            self.addiw(rd, rs, 0);
        }

        #[cfg(target_arch = "riscv64")]
        pub fn is_addiw(_instr: Instr) -> bool {
            //Placeholder
            false
        }

        #[cfg(target_arch = "riscv64")]
        pub fn is_ld(_instr: Instr) -> bool {
            //Placeholder
            false
        }

        fn branch_offset(&self, l: &Label) -> i32 {
            self.branch_offset_helper(l, OffsetSize::KOffset13)
        }

        fn jump_offset(&self, l: &Label) -> i32 {
            self.branch_offset_helper(l, OffsetSize::KOffset21)
        }
    }

    #[derive(Debug)]
    pub struct Label {
        // Add fields to represent a label in the assembler.
    }

    #[derive(Debug)]
    enum OffsetSize {
        KOffset13,
        KOffset21,
    }

    fn zero_reg() -> Register {
        Register { code: 0 }
    }

    fn ra() -> Register {
        Register { code: 1 }
    }
}

pub mod assembler {
    pub struct Assembler {}
}

pub mod riscv {
    pub mod base_assembler_riscv {
        pub struct AssemblerRiscvBase {}
    }

    pub mod constant_riscv_i {
        pub struct Instr {}
    }

    pub mod register_riscv {
        #[derive(Debug, Copy, Clone)]
        pub struct Register {
            pub code: u8,
        }
    }
}