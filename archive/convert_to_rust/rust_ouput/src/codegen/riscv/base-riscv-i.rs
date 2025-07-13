// Converted from V8 C++ source files:
// Header: base-riscv-i.h
// Implementation: base-riscv-i.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_camel_case_types)]

use std::convert::TryInto;

pub struct AssemblerRISCVI {
    assembler_riscv_base: AssemblerRiscvBase,
}

impl AssemblerRISCVI {
    pub fn new(assembler_riscv_base: AssemblerRiscvBase) -> Self {
        Self {
            assembler_riscv_base,
        }
    }

    fn gen_instr_u(&mut self, opcode: u32, rd: Register, imm20: i32) {
        let instruction: u32 = ((imm20 as u32) & 0xfffff000) | ((rd.code() as u32) << 7) | opcode;
        self.assembler_riscv_base.emit(instruction);
    }

    fn gen_instr_j(&mut self, opcode: u32, rd: Register, imm21: i32) {
        let imm21_u32 = imm21 as u32;
        let imm20 = (imm21_u32 >> 20) & 0x1;
        let imm10_1 = (imm21_u32 >> 1) & 0x3ff;
        let imm11 = (imm21_u32 >> 11) & 0x1;
        let imm19_12 = (imm21_u32 >> 12) & 0xff;

        let instruction: u32 = (imm19_12 << 12) | (imm11 << 20) | (imm10_1 << 21) | (imm20 << 31) | ((rd.code() as u32) << 7) | opcode;

        self.assembler_riscv_base.emit(instruction);
    }

    fn gen_instr_i(&mut self, funct3: u32, opcode: u32, rd: Register, rs1: Register, imm12: i16) {
        let instruction: u32 = ((imm12 as u32) << 20) | ((rs1.code() as u32) << 15) | ((funct3 as u32) << 12) | ((rd.code() as u32) << 7) | opcode;
        self.assembler_riscv_base.emit(instruction);
    }

    fn gen_instr_branch_cc_rri(&mut self, funct3: u32, rs1: Register, rs2: Register, imm13: i16) {
        let imm13_u32 = imm13 as u32;

        let imm12 = (imm13_u32 >> 12) & 0x1;
        let imm10_5 = (imm13_u32 >> 5) & 0x3f;
        let imm4_1 = (imm13_u32 >> 1) & 0xf;
        let imm11 = (imm13_u32 >> 11) & 0x1;
        
        let instruction: u32 = (imm12 << 31) | (imm10_5 << 25) | ((rs2.code() as u32) << 20) | ((rs1.code() as u32) << 15) | (funct3 << 12) | (imm4_1 << 8) | (imm11 << 7) | BRANCH;
        self.assembler_riscv_base.emit(instruction);
    }

    fn gen_instr_load_ri(&mut self, funct3: u32, rd: Register, rs1: Register, imm12: i16) {
        let instruction: u32 = ((imm12 as u32) << 20) | ((rs1.code() as u32) << 15) | (funct3 << 12) | ((rd.code() as u32) << 7) | LOAD;
        self.assembler_riscv_base.emit(instruction);
    }

    fn gen_instr_store_rri(&mut self, funct3: u32, base: Register, source: Register, imm12: i16) {
        let imm12_u32 = imm12 as u32;
        let imm11_5 = (imm12_u32 >> 5) & 0x7f;
        let imm4_0 = imm12_u32 & 0x1f;

        let instruction: u32 = (imm11_5 << 25) | ((source.code() as u32) << 20) | ((base.code() as u32) << 15) | (funct3 << 12) | (imm4_0 << 7) | STORE;
        self.assembler_riscv_base.emit(instruction);
    }

    fn gen_instr_alu_ri(&mut self, funct3: u32, rd: Register, rs1: Register, imm12: i16) {
        let instruction: u32 = ((imm12 as u32) << 20) | ((rs1.code() as u32) << 15) | (funct3 << 12) | ((rd.code() as u32) << 7) | OP_IMM;
        self.assembler_riscv_base.emit(instruction);
    }

    fn gen_instr_shift_ri(&mut self, funct7: u32, funct3: u32, rd: Register, rs1: Register, shamt: u8) {
        let instruction: u32 = ((funct7 as u32) << 30) | ((shamt as u32) << 20) | ((rs1.code() as u32) << 15) | (funct3 << 12) | ((rd.code() as u32) << 7) | OP_IMM;
        self.assembler_riscv_base.emit(instruction);
    }

    fn gen_instr_alu_rr(&mut self, funct7: u32, funct3: u32, rd: Register, rs1: Register, rs2: Register) {
        let instruction: u32 = (funct7 << 25) | ((rs2.code() as u32) << 20) | ((rs1.code() as u32) << 15) | (funct3 << 12) | ((rd.code() as u32) << 7) | OP;
        self.assembler_riscv_base.emit(instruction);
    }
    fn clear_vectorunit(&mut self) {}
    fn block_trampoline_pool_for(&mut self, _i: i32) {}

    pub fn lui(&mut self, rd: Register, imm20: i32) {
        self.gen_instr_u(LUI, rd, imm20);
    }

    pub fn auipc(&mut self, rd: Register, imm20: i32) {
        self.gen_instr_u(AUIPC, rd, imm20);
    }

    // Jumps
    pub fn jal(&mut self, rd: Register, imm21: i32) {
        self.gen_instr_j(JAL, rd, imm21);
        self.clear_vectorunit();
        self.block_trampoline_pool_for(1);
    }

    pub fn jalr(&mut self, rd: Register, rs1: Register, imm12: i16) {
        self.gen_instr_i(0b000, JALR, rd, rs1, imm12);
        self.clear_vectorunit();
        self.block_trampoline_pool_for(1);
    }

    // Branches
    pub fn beq(&mut self, rs1: Register, rs2: Register, imm13: i16) {
        self.gen_instr_branch_cc_rri(0b000, rs1, rs2, imm13);
        self.clear_vectorunit();
    }

    pub fn bne(&mut self, rs1: Register, rs2: Register, imm13: i16) {
        self.gen_instr_branch_cc_rri(0b001, rs1, rs2, imm13);
        self.clear_vectorunit();
    }

    pub fn blt(&mut self, rs1: Register, rs2: Register, imm13: i16) {
        self.gen_instr_branch_cc_rri(0b100, rs1, rs2, imm13);
        self.clear_vectorunit();
    }

    pub fn bge(&mut self, rs1: Register, rs2: Register, imm13: i16) {
        self.gen_instr_branch_cc_rri(0b101, rs1, rs2, imm13);
        self.clear_vectorunit();
    }

    pub fn bltu(&mut self, rs1: Register, rs2: Register, imm13: i16) {
        self.gen_instr_branch_cc_rri(0b110, rs1, rs2, imm13);
        self.clear_vectorunit();
    }

    pub fn bgeu(&mut self, rs1: Register, rs2: Register, imm13: i16) {
        self.gen_instr_branch_cc_rri(0b111, rs1, rs2, imm13);
        self.clear_vectorunit();
    }

    // Loads
    pub fn lb(&mut self, rd: Register, rs1: Register, imm12: i16) {
        self.gen_instr_load_ri(0b000, rd, rs1, imm12);
    }

    pub fn lh(&mut self, rd: Register, rs1: Register, imm12: i16) {
        self.gen_instr_load_ri(0b001, rd, rs1, imm12);
    }

    pub fn lw(&mut self, rd: Register, rs1: Register, imm12: i16) {
        self.gen_instr_load_ri(0b010, rd, rs1, imm12);
    }

    pub fn lbu(&mut self, rd: Register, rs1: Register, imm12: i16) {
        self.gen_instr_load_ri(0b100, rd, rs1, imm12);
    }

    pub fn lhu(&mut self, rd: Register, rs1: Register, imm12: i16) {
        self.gen_instr_load_ri(0b101, rd, rs1, imm12);
    }

    // Stores
    pub fn sb(&mut self, source: Register, base: Register, imm12: i16) {
        self.gen_instr_store_rri(0b000, base, source, imm12);
    }

    pub fn sh(&mut self, source: Register, base: Register, imm12: i16) {
        self.gen_instr_store_rri(0b001, base, source, imm12);
    }

    pub fn sw(&mut self, source: Register, base: Register, imm12: i16) {
        self.gen_instr_store_rri(0b010, base, source, imm12);
    }

    // Arithmetic with immediate
    pub fn addi(&mut self, rd: Register, rs1: Register, imm12: i16) {
        self.gen_instr_alu_ri(0b000, rd, rs1, imm12);
    }

    pub fn slti(&mut self, rd: Register, rs1: Register, imm12: i16) {
        self.gen_instr_alu_ri(0b010, rd, rs1, imm12);
    }

    pub fn sltiu(&mut self, rd: Register, rs1: Register, imm12: i16) {
        self.gen_instr_alu_ri(0b011, rd, rs1, imm12);
    }

    pub fn xori(&mut self, rd: Register, rs1: Register, imm12: i16) {
        self.gen_instr_alu_ri(0b100, rd, rs1, imm12);
    }

    pub fn ori(&mut self, rd: Register, rs1: Register, imm12: i16) {
        self.gen_instr_alu_ri(0b110, rd, rs1, imm12);
    }

    pub fn andi(&mut self, rd: Register, rs1: Register, imm12: i16) {
        self.gen_instr_alu_ri(0b111, rd, rs1, imm12);
    }

    pub fn slli(&mut self, rd: Register, rs1: Register, shamt: u8) {
        self.gen_instr_shift_ri(0, 0b001, rd, rs1, shamt & 0x3f);
    }

    pub fn srli(&mut self, rd: Register, rs1: Register, shamt: u8) {
        self.gen_instr_shift_ri(0, 0b101, rd, rs1, shamt & 0x3f);
    }

    pub fn srai(&mut self, rd: Register, rs1: Register, shamt: u8) {
        self.gen_instr_shift_ri(1, 0b101, rd, rs1, shamt & 0x3f);
    }

    // Arithmetic
    pub fn add(&mut self, rd: Register, rs1: Register, rs2: Register) {
        self.gen_instr_alu_rr(0b0000000, 0b000, rd, rs1, rs2);
    }

    pub fn sub(&mut self, rd: Register, rs1: Register, rs2: Register) {
        self.gen_instr_alu_rr(0b0100000, 0b000, rd, rs1, rs2);
    }

    pub fn sll(&mut self, rd: Register, rs1: Register, rs2: Register) {
        self.gen_instr_alu_rr(0b0000000, 0b001, rd, rs1, rs2);
    }

    pub fn slt(&mut self, rd: Register, rs1: Register, rs2: Register) {
        self.gen_instr_alu_rr(0b0000000, 0b010, rd, rs1, rs2);
    }

    pub fn sltu(&mut self, rd: Register, rs1: Register, rs2: Register) {
        self.gen_instr_alu_rr(0b0000000, 0b011, rd, rs1, rs2);
    }

    pub fn xor_(&mut self, rd: Register, rs1: Register, rs2: Register) {
        self.gen_instr_alu_rr(0b0000000, 0b100, rd, rs1, rs2);
    }

    pub fn srl(&mut self, rd: Register, rs1: Register, rs2: Register) {
        self.gen_instr_alu_rr(0b0000000, 0b101, rd, rs1, rs2);
    }

    pub fn sra(&mut self, rd: Register, rs1: Register, rs2: Register) {
        self.gen_instr_alu_rr(0b0100000, 0b101, rd, rs1, rs2);
    }

    pub fn or_(&mut self, rd: Register, rs1: Register, rs2: Register) {
        self.gen_instr_alu_rr(0b0000000, 0b110, rd, rs1, rs2);
    }

    pub fn and_(&mut self, rd: Register, rs1: Register, rs2: Register) {
        self.gen_instr_alu_rr(0b0000000, 0b111, rd, rs1, rs2);
    }

    // Other pseudo instructions that are not part of RISCV pseudo assemly
    pub fn nor(&mut self, rd: Register, rs: Register, rt: Register) {
        self.or_(rd, rs, rt);
        self.not_(rd, rd);
    }

    // Memory fences
    pub fn fence(&mut self, pred: u8, succ: u8) {
        assert!(pred <= 0b1111 && succ <= 0b1111);
        let imm12: u16 = (succ as u16) | ((pred as u16) << 4) | (0b0000 << 8);
        self.gen_instr_i(0b000, MISC_MEM, zero_reg, zero_reg, imm12 as i16);
    }

    pub fn fence_tso(&mut self) {
        let imm12: u16 = (0b0011) | (0b0011 << 4) | (0b1000 << 8);
        self.gen_instr_i(0b000, MISC_MEM, zero_reg, zero_reg, imm12 as i16);
    }

    // Environment call / break
    pub fn ecall(&mut self) {
        self.gen_instr_i(0b000, SYSTEM, zero_reg, zero_reg, 0);
    }

    pub fn ebreak(&mut self) {
        self.gen_instr_i(0b000, SYSTEM, zero_reg, zero_reg, 1);
    }

    pub fn sync(&mut self) {
        self.fence(0b1111, 0b1111);
    }

    // This is a de facto standard (as set by GNU binutils) 32-bit unimplemented
    // instruction (i.e., it should always trap, if your implementation has
    // invalid instruction traps).
    pub fn unimp(&mut self) {
        self.gen_instr_i(0b001, SYSTEM, zero_reg, zero_reg, 0b110000000000);
    }

    pub fn jump_offset(&mut self, l: &Label) -> i32 {
        self.assembler_riscv_base.branch_offset_helper(l, OffsetSize::kOffset21)
    }

    pub fn branch_offset(&mut self, l: &Label) -> i32 {
        self.assembler_riscv_base.branch_offset_helper(l, OffsetSize::kOffset13)
    }

    // Branches
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
        self.beq(rs, zero_reg, imm13);
    }
    pub fn beqz_label(&mut self, rs1: Register, l: &Label) {
        self.beqz(rs1, self.branch_offset(l) as i16);
    }
    pub fn bnez(&mut self, rs: Register, imm13: i16) {
        self.bne(rs, zero_reg, imm13);
    }
    pub fn bnez_label(&mut self, rs1: Register, l: &Label) {
        self.bnez(rs1, self.branch_offset(l) as i16);
    }
    pub fn blez(&mut self, rs: Register, imm13: i16) {
        self.bge(zero_reg, rs, imm13);
    }
    pub fn blez_label(&mut self, rs1: Register, l: &Label) {
        self.blez(rs1, self.branch_offset(l) as i16);
    }
    pub fn bgez(&mut self, rs: Register, imm13: i16) {
        self.bge(rs, zero_reg, imm13);
    }
    pub fn bgez_label(&mut self, rs1: Register, l: &Label) {
        self.bgez(rs1, self.branch_offset(l) as i16);
    }
    pub fn bltz(&mut self, rs: Register, imm13: i16) {
        self.blt(rs, zero_reg, imm13);
    }
    pub fn bltz_label(&mut self, rs1: Register, l: &Label) {
        self.bltz(rs1, self.branch_offset(l) as i16);
    }
    pub fn bgtz(&mut self, rs: Register, imm13: i16) {
        self.blt(zero_reg, rs, imm13);
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
        self.jal(zero_reg, imm21);
    }
    pub fn j_label(&mut self, l: &Label) {
        self.j(self.jump_offset(l));
    }
    pub fn b(&mut self, l: &Label) {
        self.j_label(l);
    }
    pub fn jal_imm21(&mut self, imm21: i32) {
        self.jal(ra, imm21);
    }
    pub fn jal_label(&mut self, l: &Label) {
        self.jal(self.jump_offset(l));
    }
    pub fn jr(&mut self, rs: Register) {
        self.jalr(zero_reg, rs, 0);
    }
    pub fn jr_imm12(&mut self, rs: Register, imm12: i32) {
        self.jalr(zero_reg, rs, imm12 as i16);
    }
    pub fn jalr_imm12(&mut self, rs: Register, imm12: i32) {
        self.jalr(ra, rs, imm12 as i16);
    }
    pub fn jalr_reg(&mut self, rs: Register) {
        self.jalr(ra, rs, 0);
    }
    pub fn ret(&mut self) {
        self.jalr(zero_reg, ra, 0);
    }

    pub fn call(&mut self, offset: i32) {
        let offset_u32 = offset as u32;
        let imm20_upper = (offset_u32 >> 12) as i32 + (((offset_u32 & 0x800) >> 11) as i32);
        self.auipc(ra, imm20_upper);
        let imm20_lower = (offset_u32 << 20) as i32 >> 20;
        self.jalr(ra, ra, imm20_lower as i16);
    }

    pub fn mv(&mut self, rd: Register, rs: Register) {
        self.addi(rd, rs, 0);
    }
    pub fn not_(&mut self, rd: Register, rs: Register) {
        self.xori(rd, rs, -1);
    }
    pub fn neg(&mut self, rd: Register, rs: Register) {
        self.sub(rd, zero_reg, rs);
    }
    pub fn seqz(&mut self, rd: Register, rs: Register) {
        self.sltiu(rd, rs, 1);
    }
    pub fn snez(&mut self, rd: Register, rs: Register) {
        self.sltu(rd, zero_reg, rs);
    }
    pub fn sltz(&mut self, rd: Register, rs: Register) {
        self.slt(rd, rs, zero_reg);
    }
    pub fn sgtz(&mut self, rd: Register, rs: Register) {
        self.slt(rd, zero_reg, rs);
    }

    #[cfg(target_arch = "riscv64")]
    pub fn lwu(&mut self, rd: Register, rs1: Register, imm12: i16) {
        self.gen_instr_load_ri(0b110, rd, rs1, imm12);
    }

    #[cfg(target_arch = "riscv64")]
    pub fn ld(&mut self, rd: Register, rs1: Register, imm12: i16) {
        self.gen_instr_load_ri(0b011, rd, rs1, imm12);
    }

    #[cfg(target_arch = "riscv64")]
    pub fn sd(&mut self, source: Register, base: Register, imm12: i16) {
        self.gen_instr_store_rri(0b011, base, source, imm12);
    }

    #[cfg(target_arch = "riscv64")]
    pub fn addiw(&mut self, rd: Register, rs1: Register, imm12: i16) {
        self.gen_instr_i(0b000, OP_IMM_32, rd, rs1, imm12);
    }

    #[cfg(target_arch = "riscv64")]
    pub fn slliw(&mut self, rd: Register, rs1: Register, shamt: u8) {
        self.gen_instr_shiftw_ri(0, 0b001, rd, rs1, shamt & 0x1f);
    }

    #[cfg(target_arch = "riscv64")]
    pub fn srliw(&mut self, rd: Register, rs1: Register, shamt: u8) {
        self.gen_instr_shiftw_ri(0, 0b101, rd, rs1, shamt & 0x1f);
    }

    #[cfg(target_arch = "riscv64")]
    pub fn sraiw(&mut self, rd: Register, rs1: Register, shamt: u8) {
        self.gen_instr_shiftw_ri(1, 0b101, rd, rs1, shamt & 0x1f);
    }

    #[cfg(target_arch = "riscv64")]
    fn gen_instr_shiftw_ri(&mut self, funct7: u32, funct3: u32, rd: Register, rs1: Register, shamt: u8) {
        let instruction: u32 = ((funct7 as u32) << 30) | ((shamt as u32) << 20) | ((rs1.code() as u32) << 15) | (funct3 << 12) | ((rd.code() as u32) << 7) | OP_IMM_32;
        self.assembler_riscv_base.emit(instruction);
    }
    #[cfg(target_arch = "riscv64")]
    pub fn addw(&mut self, rd: Register, rs1: Register, rs2: Register) {
        self.gen_instr_aluw_rr(0b0000000, 0b000, rd, rs1, rs2);
    }
    #[cfg(target_arch = "riscv64")]
    fn gen_instr_aluw_rr(&mut self, funct7: u32, funct3: u32, rd: Register, rs1: Register, rs2: Register) {
        let instruction: u32 = (funct7 << 25) | ((rs2.code() as u32) << 20) | ((rs1.code() as u32) << 15) | (funct3 << 12) | ((rd.code() as u32) << 7) | OP_32;
        self.assembler_riscv_base.emit(instruction);
    }

    #[cfg(target_arch = "riscv64")]
    pub fn subw(&mut self, rd: Register, rs1: Register, rs2: Register) {
        self.gen_instr_aluw_rr(0b0100000, 0b000, rd, rs1, rs2);
    }
    #[cfg(target_arch = "riscv64")]
    pub fn sllw(&mut self, rd: Register, rs1: Register, rs2: Register) {
        self.gen_instr_aluw_rr(0b0000000, 0b001, rd, rs1, rs2);
    }
    #[cfg(target_arch = "riscv64")]
    pub fn srlw(&mut self, rd: Register, rs1: Register, rs2: Register) {
        self.gen_instr_aluw_rr(0b0000000, 0b101, rd, rs1, rs2);
    }
    #[cfg(target_arch = "riscv64")]
    pub fn sraw(&mut self, rd: Register, rs1: Register, rs2: Register) {
        self.gen_instr_aluw_rr(0b0100000, 0b101, rd, rs1, rs2);
    }
    #[cfg(target_arch = "riscv64")]
    pub fn negw(&mut self, rd: Register, rs: Register) {
        self.subw(rd, zero_reg, rs);
    }
    #[cfg(target_arch = "riscv64")]
    pub fn sext_w(&mut self, rd: Register, rs: Register) {
        self.addiw(rd, rs, 0);
    }

    pub fn is_branch(instr: Instr) -> bool {
        (instr & K_BASE_OPCODE_MASK) == BRANCH
    }

    pub fn is_jump(instr: Instr) -> bool {
        let op = instr & K_BASE_OPCODE_MASK;
        op == JAL || op == JALR
    }

    pub fn is_nop(instr: Instr) -> bool {
        instr == KNOP_BYTE
    }

    pub fn is_jal(instr: Instr) -> bool {
        (instr & K_
