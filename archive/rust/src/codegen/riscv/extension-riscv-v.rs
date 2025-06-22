// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/codegen/riscv/extension-riscv-v.h (Converted to Rust module)

/// Defines the AssemblerRISCVV struct and its methods for RISC-V Vector
/// extension instructions.
mod extension_riscv_v {
    use crate::codegen::riscv::{
        assembler::Assembler, constant_riscv_v::*, register_riscv::*,
    };

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum MaskType {
        NoMask,
        Mask,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum VSew {
        E8,
        E16,
        E32,
        E64,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Vlmul {
        LM1,
        LM2,
        LM4,
        LM8,
        MF2,
        MF4,
        MF8,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum TailAgnosticType {
        Agnostic,
        Undisturbed,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum MaskAgnosticType {
        Agnostic,
        Undisturbed,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Opcode {
        V,
        MVV,
        FVV,
        IVV,
        IVX,
        FVF,
        IVI,
        MVX,
        LOAD_FP,
        STORE_FP,
        AMO,
    }

    #[derive(Debug, Copy, Clone)]
    pub struct LoadStoreLaneParams {
        pub lane_idx: u8,
        pub element_size_bits: u32,
        pub max_lanes: u32,
    }

    impl LoadStoreLaneParams {
        pub fn new(lane_idx: u8, element_size_bits: u32, max_lanes: u32) -> Self {
            LoadStoreLaneParams {
                lane_idx,
                element_size_bits,
                max_lanes,
            }
        }
    }

    pub trait AssemblerRISCVVInterface {
        fn emit(&mut self, instr: u32);
    }

    /// The `AssemblerRISCVV` struct provides methods for generating RISC-V Vector
    /// extension instructions.
    pub struct AssemblerRISCVV<'a> {
        assembler: &'a mut Assembler,
    }

    impl<'a> AssemblerRISCVV<'a> {
        pub fn new(assembler: &'a mut Assembler) -> Self {
            AssemblerRISCVV { assembler }
        }

        fn emit(&mut self, instr: u32) {
            self.assembler.emit(instr);
        }

        /// Generates the ZIMM immediate value for `vsetvli` and `vsetivli` instructions.
        fn gen_zimm(&self, vsew: VSew, vlmul: Vlmul, tail: TailAgnosticType, mask: MaskAgnosticType) -> i32 {
            let vsew_bits = match vsew {
                VSew::E8 => 0b000,
                VSew::E16 => 0b001,
                VSew::E32 => 0b010,
                VSew::E64 => 0b011,
            };

            let vlmul_bits = match vlmul {
                Vlmul::MF8 => 0b111,
                Vlmul::MF4 => 0b110,
                Vlmul::MF2 => 0b101,
                Vlmul::LM1 => 0b000,
                Vlmul::LM2 => 0b001,
                Vlmul::LM4 => 0b010,
                Vlmul::LM8 => 0b011,
            };

            let ta_bit = match tail {
                TailAgnosticType::Agnostic => 0,
                TailAgnosticType::Undisturbed => 1,
            };

            let ma_bit = match mask {
                MaskAgnosticType::Agnostic => 0,
                MaskAgnosticType::Undisturbed => 1,
            };

            (ma_bit << 6 | ta_bit << 5 | vsew_bits << 2 | vlmul_bits) as i32
        }

        /// Generates the RVV instruction.
        fn gen_instr_v(
            &mut self,
            funct6: u8,
            opcode: Opcode,
            vd: VRegister,
            rs1: VRegister,
            vs2: VRegister,
            mask: MaskType,
        ) {
            assert!(
                opcode == Opcode::MVV || opcode == Opcode::FVV || opcode == Opcode::IVV,
                "Opcode {:?} not supported",
                opcode
            );
            let instr: u32 = (funct6 as u32) << kRvvFunct6Shift
                | match opcode {
                    Opcode::MVV => 0b1010111, // Replace with actual opcodes
                    Opcode::FVV => 0b1010111,
                    Opcode::IVV => 0b1010111,
                    _ => 0,
                }
                | (match mask {
                    MaskType::NoMask => 0,
                    MaskType::Mask => 1,
                } << kRvvVmShift) as u32
                | ((vd.code() & 0x1F) << kRvvVdShift) as u32
                | ((rs1.code() & 0x1F) << kRvvVs1Shift) as u32
                | ((vs2.code() & 0x1F) << kRvvVs2Shift) as u32;
            self.emit(instr);
        }
        fn gen_instr_v_imm(
            &mut self,
            funct6: u8,
            vd: VRegister,
            imm5: i8,
            vs2: VRegister,
            mask: MaskType,
        ) {
            let instr: u32 = (funct6 as u32) << kRvvFunct6Shift
                | 0b1010111 // OP_IVI Replace with actual opcodes
                | (match mask {
                    MaskType::NoMask => 0,
                    MaskType::Mask => 1,
                } << kRvvVmShift) as u32
                | ((vd.code() & 0x1F) << kRvvVdShift) as u32
                | (((imm5 as u32) << kRvvImm5Shift) & kRvvImm5Mask) as u32
                | ((vs2.code() & 0x1F) << kRvvVs2Shift) as u32;
            self.emit(instr);
        }

        fn gen_instr_v_reg(
            &mut self,
            funct6: u8,
            opcode: Opcode,
            rd: Register,
            vs1: VRegister,
            vs2: VRegister,
            mask: MaskType,
        ) {
            assert!(opcode == Opcode::MVV || opcode == Opcode::FVV);

            let instr: u32 = (funct6 as u32) << kRvvFunct6Shift
                | match opcode {
                    Opcode::MVV => 0b1010111,
                    Opcode::FVV => 0b1010111,
                    _ => 0,
                }
                | (match mask {
                    MaskType::NoMask => 0,
                    MaskType::Mask => 1,
                } << kRvvVmShift) as u32
                | ((rd.code() & 0x1F) << kRvvVdShift) as u32
                | ((vs1.code() & 0x1F) << kRvvVs1Shift) as u32
                | ((vs2.code() & 0x1F) << kRvvVs2Shift) as u32;
            self.emit(instr);
        }

        fn gen_instr_v_fpu(
            &mut self,
            funct6: u8,
            opcode: Opcode,
            fd: FPURegister,
            vs1: VRegister,
            vs2: VRegister,
            mask: MaskType,
        ) {
            assert_eq!(opcode, Opcode::FVV);
            let instr: u32 = (funct6 as u32) << kRvvFunct6Shift
                | 0b1010111 // Replace with actual opcodes for OP_FVV
                | (match mask {
                    MaskType::NoMask => 0,
                    MaskType::Mask => 1,
                } << kRvvVmShift) as u32
                | ((fd.code() & 0x1F) << kRvvVdShift) as u32
                | ((vs1.code() & 0x1F) << kRvvVs1Shift) as u32
                | ((vs2.code() & 0x1F) << kRvvVs2Shift) as u32;
            self.emit(instr);
        }

        fn gen_instr_v_reg_vreg(
            &mut self,
            funct6: u8,
            opcode: Opcode,
            vd: VRegister,
            rs1: Register,
            vs2: VRegister,
            mask: MaskType,
        ) {
            assert!(opcode == Opcode::IVX || opcode == Opcode::MVX);

            let instr: u32 = (funct6 as u32) << kRvvFunct6Shift
                | match opcode {
                    Opcode::IVX => 0b1010111,
                    Opcode::MVX => 0b1010111,
                    _ => 0,
                }
                | (match mask {
                    MaskType::NoMask => 0,
                    MaskType::Mask => 1,
                } << kRvvVmShift) as u32
                | ((vd.code() & 0x1F) << kRvvVdShift) as u32
                | ((rs1.code() & 0x1F) << kRvvRs1Shift) as u32
                | ((vs2.code() & 0x1F) << kRvvVs2Shift) as u32;
            self.emit(instr);
        }

        fn gen_instr_v_fpu_vreg(
            &mut self,
            funct6: u8,
            opcode: Opcode,
            vd: VRegister,
            fs1: FPURegister,
            vs2: VRegister,
            mask: MaskType,
        ) {
            assert_eq!(opcode, Opcode::FVF);
            let instr: u32 = (funct6 as u32) << kRvvFunct6Shift
                | 0b1010111 // Replace with actual opcodes for OP_FVF
                | (match mask {
                    MaskType::NoMask => 0,
                    MaskType::Mask => 1,
                } << kRvvVmShift) as u32
                | ((vd.code() & 0x1F) << kRvvVdShift) as u32
                | ((fs1.code() & 0x1F) << kRvvRs1Shift) as u32
                | ((vs2.code() & 0x1F) << kRvvVs2Shift) as u32;
            self.emit(instr);
        }

        fn gen_instr_v_reg_vreg_nomask(
            &mut self,
            funct6: u8,
            opcode: Opcode,
            rd: Register,
            rs1: Register,
            vs2: VRegister,
            mask: MaskType,
        ) {
            let instr: u32 = (funct6 as u32) << kRvvFunct6Shift
                | 0b1010111 // Replace with actual opcodes for OP_MVX
                | (match mask {
                    MaskType::NoMask => 0,
                    MaskType::Mask => 1,
                } << kRvvVmShift) as u32
                | ((rd.code() & 0x1F) << kRvvVdShift) as u32
                | ((rs1.code() & 0x1F) << kRvvRs1Shift) as u32
                | ((vs2.code() & 0x1F) << kRvvVs2Shift) as u32;
            self.emit(instr);
        }
        fn gen_instr_v_vreg(
            &mut self,
            base_opcode: BaseOpcode,
            width: u8,
            vd: VRegister,
            rs1: Register,
            umop: u8,
            mask: MaskType,
            is_mop: u8,
            is_mew: bool,
            nf: u8,
        ) {
            assert!(
                base_opcode == BaseOpcode::LOAD_FP || base_opcode == BaseOpcode::STORE_FP
            );
            let is_mew_int = if is_mew { 1 } else { 0 };
            let instr: u32 = base_opcode as u32
                | ((vd.code() << kRvvVdShift) & kRvvVdMask) as u32
                | ((width << kRvvWidthShift) & kRvvWidthMask) as u32
                | ((rs1.code() << kRvvRs1Shift) & kRvvRs1Mask) as u32
                | ((umop << kRvvRs2Shift) & kRvvRs2Mask) as u32
                | ((match mask {
                    MaskType::NoMask => 0,
                    MaskType::Mask => 1,
                } << kRvvVmShift) & kRvvVmMask) as u32
                | ((is_mop << kRvvMopShift) & kRvvMopMask) as u32
                | ((is_mew_int << kRvvMewShift) & kRvvMewMask) as u32
                | ((nf << kRvvNfShift) & kRvvNfMask) as u32;
            self.emit(instr);
        }

        fn gen_instr_v_vreg_reg(
            &mut self,
            base_opcode: BaseOpcode,
            width: u8,
            vd: VRegister,
            rs1: Register,
            rs2: Register,
            mask: MaskType,
            is_mop: u8,
            is_mew: bool,
            nf: u8,
        ) {
            assert!(
                base_opcode == BaseOpcode::LOAD_FP || base_opcode == BaseOpcode::STORE_FP
            );
            let is_mew_int = if is_mew { 1 } else { 0 };
            let instr: u32 = base_opcode as u32
                | ((vd.code() << kRvvVdShift) & kRvvVdMask) as u32
                | ((width << kRvvWidthShift) & kRvvWidthMask) as u32
                | ((rs1.code() << kRvvRs1Shift) & kRvvRs1Mask) as u32
                | ((rs2.code() << kRvvRs2Shift) & kRvvRs2Mask) as u32
                | ((match mask {
                    MaskType::NoMask => 0,
                    MaskType::Mask => 1,
                } << kRvvVmShift) & kRvvVmMask) as u32
                | ((is_mop << kRvvMopShift) & kRvvMopMask) as u32
                | ((is_mew_int << kRvvMewShift) & kRvvMewMask) as u32
                | ((nf << kRvvNfShift) & kRvvNfMask) as u32;
            self.emit(instr);
        }

        fn gen_instr_v_vreg_vsreg(
            &mut self,
            base_opcode: BaseOpcode,
            width: u8,
            vd: VRegister,
            rs1: Register,
            vs2: VRegister,
            mask: MaskType,
            is_mop: u8,
            is_mew: bool,
            nf: u8,
        ) {
            assert!(
                base_opcode == BaseOpcode::LOAD_FP
                    || base_opcode == BaseOpcode::STORE_FP
                    || base_opcode == BaseOpcode::AMO
            );
            let is_mew_int = if is_mew { 1 } else { 0 };
            let instr: u32 = base_opcode as u32
                | ((vd.code() << kRvvVdShift) & kRvvVdMask) as u32
                | ((width << kRvvWidthShift) & kRvvWidthMask) as u32
                | ((rs1.code() << kRvvRs1Shift) & kRvvRs1Mask) as u32
                | ((vs2.code() << kRvvRs2Shift) & kRvvRs2Mask) as u32
                | ((match mask {
                    MaskType::NoMask => 0,
                    MaskType::Mask => 1,
                } << kRvvVmShift) & kRvvVmMask) as u32
                | ((is_mop << kRvvMopShift) & kRvvMopMask) as u32
                | ((is_mew_int << kRvvMewShift) & kRvvMewMask) as u32
                | ((nf << kRvvNfShift) & kRvvNfMask) as u32;
            self.emit(instr);
        }

        fn gen_instr_v_reg_uint(
            &mut self,
            funct6: u8,
            opcode: Opcode,
            rd: Register,
            vs1: u8,
            vs2: VRegister,
            mask: MaskType,
        ) {
            assert_eq!(opcode, Opcode::MVV);
            let instr: u32 = (funct6 as u32) << kRvvFunct6Shift
                | 0b1010111 // Replace with actual opcodes for OP_MVV
                | (match mask {
                    MaskType::NoMask => 0,
                    MaskType::Mask => 1,
                } << kRvvVmShift) as u32
                | ((rd.code() & 0x1F) << kRvvVdShift) as u32
                | ((vs1 & 0x1F) << kRvvVs1Shift) as u32
                | ((vs2.code() & 0x1F) << kRvvVs2Shift) as u32;
            self.emit(instr);
        }

        /// Emits a `vredmaxu.vs` instruction.
        pub fn vredmaxu_vs(&mut self, vd: VRegister, vs2: VRegister, vs1: VRegister, mask: MaskType) {
            self.gen_instr_v(VREDMAXU_FUNCT6, Opcode::MVV, vd, vs1, vs2, mask);
        }

        /// Emits a `vredmax.vs` instruction.
        pub fn vredmax_vs(&mut self, vd: VRegister, vs2: VRegister, vs1: VRegister, mask: MaskType) {
            self.gen_instr_v(VREDMAX_FUNCT6, Opcode::MVV, vd, vs1, vs2, mask);
        }

        /// Emits a `vredmin.vs` instruction.
        pub fn vredmin_vs(&mut self, vd: VRegister, vs2: VRegister, vs1: VRegister, mask: MaskType) {
            self.gen_instr_v(VREDMIN_FUNCT6, Opcode::MVV, vd, vs1, vs2, mask);
        }

        /// Emits a `vredminu.vs` instruction.
        pub fn vredminu_vs(&mut self, vd: VRegister, vs2: VRegister, vs1: VRegister, mask: MaskType) {
            self.gen_instr_v(VREDMINU_FUNCT6, Opcode::MVV, vd, vs1, vs2, mask);
        }

        /// Emits a `vmv.vv` instruction.
        pub fn vmv_vv(&mut self, vd: VRegister, vs1: VRegister) {
            self.gen_instr_v(VMV_FUNCT6, Opcode::IVV, vd, vs1, VRegister::V0, MaskType::NoMask);
        }

        /// Emits a `vmv.vx` instruction.
        pub fn vmv_vx(&mut self, vd: VRegister, rs1: Register) {
            self.gen_instr_v(VMV_FUNCT6, Opcode::IVX, vd, Register::ZERO, VRegister::V0, MaskType::NoMask);
        }

        /// Emits a `vmv.vi` instruction.
        pub fn vmv_vi(&mut self, vd: VRegister, simm5: u8) {
            self.gen_instr_v_imm(VMV_FUNCT6, vd, simm5 as i8, VRegister::V0, MaskType::NoMask);
        }

        /// Emits a `vmv.xs` instruction.
        pub fn vmv_xs(&mut self, rd: Register, vs2: VRegister) {
            self.gen_instr_v_reg(
                VWXUNARY0_FUNCT6,
                Opcode::MVV,
                rd,
                VRegister::V0,
                vs2,
                MaskType::NoMask,
            );
        }

        /// Emits a `vmv.sx` instruction.
        pub fn vmv_sx(&mut self, vd: VRegister, rs1: Register) {
            self.gen_instr_v_reg_vreg(VRXUNARY0_FUNCT6, Opcode::MVX, vd, rs1, VRegister::V0, MaskType::NoMask);
        }

        /// Emits a `vmerge.vv` instruction.
        pub fn vmerge_vv(&mut self, vd: VRegister, vs1: VRegister, vs2: VRegister) {
            self.gen_instr_v(VMV_FUNCT6, Opcode::IVV, vd, vs1, vs2, MaskType::Mask);
        }

        /// Emits a `vmerge.vx` instruction.
        pub fn vmerge_vx(&mut self, vd: VRegister, rs1: Register, vs2: VRegister) {
            self.gen_instr_v_reg_vreg(VMV_FUNCT6, Opcode::IVX, vd, rs1, vs2, MaskType::Mask);
        }

        /// Emits a `vmerge.vi` instruction.
        pub fn vmerge_vi(&mut self, vd: VRegister, imm5: u8, vs2: VRegister) {
            self.gen_instr_v_imm(VMV_FUNCT6, vd, imm5 as i8, vs2, MaskType::Mask);
        }

        /// Emits a `vadc.vv` instruction.
        pub fn vadc_vv(&mut self, vd: VRegister, vs1: VRegister, vs2: VRegister) {
            self.gen_instr_v(VADC_FUNCT6, Opcode::IVV, vd, vs1, vs2, MaskType::Mask);
        }

        /// Emits a `vadc.vx` instruction.
        pub fn vadc_vx(&mut self, vd: VRegister, rs1: Register, vs2: VRegister) {
            self.gen_instr_v_reg_vreg(VADC_FUNCT6, Opcode::IVX, vd, rs1, vs2, MaskType::Mask);
        }

        /// Emits a `vadc.vi` instruction.
        pub fn vadc_vi(&mut self, vd: VRegister, imm5: u8, vs2: VRegister) {
            self.gen_instr_v_imm(VADC_FUNCT6, vd, imm5 as i8, vs2, MaskType::Mask);
        }

        /// Emits a `vmadc.vv` instruction.
        pub fn vmadc_vv(&mut self, vd: VRegister, vs1: VRegister, vs2: VRegister) {
            self.gen_instr_v(VMADC_FUNCT6, Opcode::IVV, vd, vs1, vs2, MaskType::Mask);
        }

        /// Emits a `vmadc.vx` instruction.
        pub fn vmadc_vx(&mut self, vd: VRegister, rs1: Register, vs2: VRegister) {
            self.gen_instr_v_reg_vreg(VMADC_FUNCT6, Opcode::IVX, vd, rs1, vs2, MaskType::Mask);
        }

        /// Emits a `vmadc.vi` instruction.
        pub fn vmadc_vi(&mut self, vd: VRegister, imm5: u8, vs2: VRegister) {
            self.gen_instr_v_imm(VMADC_FUNCT6, vd, imm5 as i8, vs2, MaskType::Mask);
        }

        /// Emits a `vrgather.vv` instruction.
        pub fn vrgather_vv(&mut self, vd: VRegister, vs2: VRegister, vs1: VRegister, mask: MaskType) {
            assert_ne!(vd, vs1);
            assert_ne!(vd, vs2);
            self.gen_instr_v(VRGATHER_FUNCT6, Opcode::IVV, vd, vs1, vs2, mask);
        }

        /// Emits a `vrgather.vi` instruction.
        pub fn vrgather_vi(&mut self, vd: VRegister, vs2: VRegister, imm5: i8, mask: MaskType) {
            assert_ne!(vd, vs2);
            self.gen_instr_v_imm(VRGATHER_FUNCT6, vd, imm5, vs2, mask);
        }

        /// Emits a `vrgather.vx` instruction.
        pub fn vrgather_vx(&mut self, vd: VRegister, vs2: VRegister, rs1: Register, mask: MaskType) {
            assert_ne!(vd, vs2);
            self.gen_instr_v_reg_vreg(VRGATHER_FUNCT6, Opcode::IVX, vd, rs1, vs2, mask);
        }

        /// Emits a `vwaddu.wx` instruction.
        pub fn vwaddu_wx(&mut self, vd: VRegister, vs2: VRegister, rs1: Register, mask: MaskType) {
            self.gen_instr_v_reg_vreg(VWADDUW_FUNCT6, Opcode::MVX, vd, rs1, vs2, mask);
        }

        /// Emits a `vid.v` instruction.
        pub fn vid_v(&mut self, vd: VRegister, mask: MaskType) {
            self.gen_instr_v(VMUNARY0_FUNCT6, Opcode::MVV, vd, VID_V, VRegister::V0, mask);
        }

        /// Emits a `vfmv.vf` instruction.
        pub fn vfmv_vf(&mut self, vd: VRegister, fs1: FPURegister) {
            self.gen_instr_v_fpu_vreg(VMV_FUNCT6, Opcode::FVF, vd, fs1, VRegister::V0, MaskType::NoMask);
        }

        /// Emits a `vfmv.fs` instruction.
        pub fn vfmv_fs(&mut self, fd: FPURegister, vs2: VRegister) {
            self.gen_instr_v_fpu(VWFUNARY0_FUNCT6, Opcode::FVV, fd, VRegister::V0, vs2, MaskType::NoMask);
        }

        /// Emits a `vfmv.sf` instruction.
        pub fn vfmv_sf(&mut self, vd: VRegister, fs: FPURegister) {
            self.gen_instr_v_fpu_vreg(VRFUNARY0_FUNCT6, Opcode::FVF, vd, fs, VRegister::V0, MaskType::NoMask);
        }

        /// Emits a `vfmerge.vf` instruction.
        pub fn vfmerge_vf(&mut self, vd: VRegister, fs1: FPURegister, vs2: VRegister) {
            self.gen_instr_v_fpu_vreg(VMV_FUNCT6, Opcode::FVF, vd, fs1, vs2, MaskType::Mask);
        }
        /// Emits a `vadd.vv` instruction.
        pub fn vadd_vv(&mut self, vd: VRegister, vs2: VRegister, vs1: VRegister, mask: MaskType) {
            self.gen_instr_v(VADD_FUNCT6, Opcode::IVV, vd, vs1, vs2, mask);
        }
        /// Emits a `vadd.vx` instruction.
        pub fn vadd_vx(&mut self, vd: VRegister, vs2: Register, vs1: VRegister, mask: MaskType) {
            self.gen_instr_v_reg_vreg(VADD_FUNCT6, Opcode::IVX, vd, vs2, vs1, mask);
        }
        /// Emits a `vadd.vi` instruction.
        pub fn vadd_vi(&mut self, vd: VRegister, vs2: VRegister, imm5: i8, mask: MaskType) {
            self.gen_instr_v_imm(VADD_FUNCT6, vd, imm5, vs2, mask);
        }
        /// Emits a `vsub.vv` instruction.
        pub fn vsub_vv(&mut self, vd: VRegister, vs2: VRegister, vs1: VRegister, mask: MaskType) {
            self.gen_instr_v(VSUB_FUNCT6, Opcode::IVV, vd, vs1, vs2, mask);
        }
        /// Emits a `vsub.vx` instruction.
        pub fn vsub_vx(&mut self, vd: VRegister, vs2: Register, vs1: VRegister, mask: MaskType) {
            self.gen_instr_v_reg_vreg(VSUB_FUNCT6, Opcode::IVX, vd, vs2, vs1, mask);
        }

        /// Emits a `vdiv.vx` instruction.
        pub fn vdiv_vx(&mut self, vd: VRegister, vs2: VRegister, rs1: Register, mask: MaskType) {
            self.gen_instr_v_reg_vreg(VDIV_FUNCT6, Opcode::MVX, vd, rs1, vs2, mask);
        }

        /// Emits a `vdivu.vx` instruction.
        pub fn vdivu_vx(&mut self, vd: VRegister, vs2: VRegister, rs1: Register, mask: MaskType) {
            self.gen_instr_v_reg_vreg(VDIVU_FUNCT6, Opcode::MVX, vd, rs1, vs2, mask);
        }

        /// Emits a `vmul.vx` instruction.
        pub fn vmul_vx(&mut self, vd: VRegister, vs2: VRegister, rs1: Register, mask: MaskType) {
            self.gen_instr_v_reg_vreg(VMUL_FUNCT6, Opcode::MVX, vd, rs1, vs2, mask);
        }

        /// Emits a `vmulhu.vx` instruction.
        pub fn vmulhu_vx(&mut self, vd: VRegister, vs2: VRegister, rs1: Register, mask: MaskType) {
            self.gen_instr_v_reg_vreg(VMULHU_FUNCT6, Opcode::MVX, vd, rs1, vs2, mask);
        }

        /// Emits a `vmulhsu.vx` instruction.
        pub fn vmulhsu_vx(&mut self, vd: VRegister, vs2: VRegister, rs1: Register, mask: MaskType) {
            self.gen_instr_v_reg_vreg(VMULHSU_FUNCT6, Opcode::MVX, vd, rs1, vs2, mask);
        }

        /// Emits a `vmulh.vx` instruction.
        pub fn vmulh_vx(&mut self, vd: VRegister, vs2: VRegister, rs1: Register, mask: MaskType) {
            self.gen_instr_v_reg_vreg(VMULH_FUNCT6, Opcode::MVX, vd, rs1, vs2, mask);
        }

        /// Emits a `vdiv.vv` instruction.
        pub fn vdiv_vv(&mut self, vd: VRegister, vs2: VRegister, vs1: VRegister, mask: MaskType) {
            self.gen_instr_v(VDIV_FUNCT6, Opcode::MVV, vd, vs1, vs2, mask);
        }

        /// Emits a `vdivu.vv` instruction.
        pub fn vdivu_vv(&mut self, vd: VRegister, vs2: VRegister, vs1: VRegister, mask: MaskType) {
            self.gen_instr_v(VDIVU_FUNCT6, Opcode