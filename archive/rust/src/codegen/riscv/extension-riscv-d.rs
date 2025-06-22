// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod riscv_d {
    use crate::codegen::{
        assembler::Assembler,
        riscv::{
            base_assembler_riscv::AssemblerRiscvBase,
            constant_riscv_d::FPURoundingMode,
            register_riscv::{FPURegister, Register},
        },
    };

    const RNE: FPURoundingMode = FPURoundingMode::RNE; // Or whatever the correct default is

    pub struct AssemblerRISCVD<'a> {
        base: AssemblerRiscvBase<'a>,
    }

    impl<'a> AssemblerRISCVD<'a> {
        pub fn new(assembler: &'a Assembler) -> Self {
            AssemblerRISCVD {
                base: AssemblerRiscvBase::new(assembler),
            }
        }

        /// RV32D Standard Extension
        pub fn fld(&mut self, rd: FPURegister, rs1: Register, imm12: i16) {
            // TODO: Implement fld instruction emission
            println!(
                "fld: rd={}, rs1={}, imm12={}",
                rd.0, rs1.0, imm12
            ); // Placeholder
        }

        pub fn fsd(&mut self, source: FPURegister, base: Register, imm12: i16) {
            // TODO: Implement fsd instruction emission
            println!(
                "fsd: source={}, base={}, imm12={}",
                source.0, base.0, imm12
            ); // Placeholder
        }

        pub fn fmadd_d(
            &mut self,
            rd: FPURegister,
            rs1: FPURegister,
            rs2: FPURegister,
            rs3: FPURegister,
            frm: FPURoundingMode,
        ) {
            // TODO: Implement fmadd_d instruction emission
            println!(
                "fmadd_d: rd={}, rs1={}, rs2={}, rs3={}, frm={:?}",
                rd.0, rs1.0, rs2.0, rs3.0, frm
            ); // Placeholder
        }

        pub fn fmsub_d(
            &mut self,
            rd: FPURegister,
            rs1: FPURegister,
            rs2: FPURegister,
            rs3: FPURegister,
            frm: FPURoundingMode,
        ) {
            // TODO: Implement fmsub_d instruction emission
            println!(
                "fmsub_d: rd={}, rs1={}, rs2={}, rs3={}, frm={:?}",
                rd.0, rs1.0, rs2.0, rs3.0, frm
            ); // Placeholder
        }

        pub fn fnmsub_d(
            &mut self,
            rd: FPURegister,
            rs1: FPURegister,
            rs2: FPURegister,
            rs3: FPURegister,
            frm: FPURoundingMode,
        ) {
            // TODO: Implement fnmsub_d instruction emission
            println!(
                "fnmsub_d: rd={}, rs1={}, rs2={}, rs3={}, frm={:?}",
                rd.0, rs1.0, rs2.0, rs3.0, frm
            ); // Placeholder
        }

        pub fn fnmadd_d(
            &mut self,
            rd: FPURegister,
            rs1: FPURegister,
            rs2: FPURegister,
            rs3: FPURegister,
            frm: FPURoundingMode,
        ) {
            // TODO: Implement fnmadd_d instruction emission
            println!(
                "fnmadd_d: rd={}, rs1={}, rs2={}, rs3={}, frm={:?}",
                rd.0, rs1.0, rs2.0, rs3.0, frm
            ); // Placeholder
        }

        pub fn fadd_d(
            &mut self,
            rd: FPURegister,
            rs1: FPURegister,
            rs2: FPURegister,
            frm: FPURoundingMode,
        ) {
            // TODO: Implement fadd_d instruction emission
            println!(
                "fadd_d: rd={}, rs1={}, rs2={}, frm={:?}",
                rd.0, rs1.0, rs2.0, frm
            ); // Placeholder
        }

        pub fn fsub_d(
            &mut self,
            rd: FPURegister,
            rs1: FPURegister,
            rs2: FPURegister,
            frm: FPURoundingMode,
        ) {
            // TODO: Implement fsub_d instruction emission
            println!(
                "fsub_d: rd={}, rs1={}, rs2={}, frm={:?}",
                rd.0, rs1.0, rs2.0, frm
            ); // Placeholder
        }

        pub fn fmul_d(
            &mut self,
            rd: FPURegister,
            rs1: FPURegister,
            rs2: FPURegister,
            frm: FPURoundingMode,
        ) {
            // TODO: Implement fmul_d instruction emission
            println!(
                "fmul_d: rd={}, rs1={}, rs2={}, frm={:?}",
                rd.0, rs1.0, rs2.0, frm
            ); // Placeholder
        }

        pub fn fdiv_d(
            &mut self,
            rd: FPURegister,
            rs1: FPURegister,
            rs2: FPURegister,
            frm: FPURoundingMode,
        ) {
            // TODO: Implement fdiv_d instruction emission
            println!(
                "fdiv_d: rd={}, rs1={}, rs2={}, frm={:?}",
                rd.0, rs1.0, rs2.0, frm
            ); // Placeholder
        }

        pub fn fsqrt_d(
            &mut self,
            rd: FPURegister,
            rs1: FPURegister,
            frm: FPURoundingMode,
        ) {
            // TODO: Implement fsqrt_d instruction emission
            println!(
                "fsqrt_d: rd={}, rs1={}, frm={:?}",
                rd.0, rs1.0, frm
            ); // Placeholder
        }

        pub fn fsgnj_d(&mut self, rd: FPURegister, rs1: FPURegister, rs2: FPURegister) {
            // TODO: Implement fsgnj_d instruction emission
            println!("fsgnj_d: rd={}, rs1={}, rs2={}", rd.0, rs1.0, rs2.0); // Placeholder
        }

        pub fn fsgnjn_d(&mut self, rd: FPURegister, rs1: FPURegister, rs2: FPURegister) {
            // TODO: Implement fsgnjn_d instruction emission
            println!("fsgnjn_d: rd={}, rs1={}, rs2={}", rd.0, rs1.0, rs2.0); // Placeholder
        }

        pub fn fsgnjx_d(&mut self, rd: FPURegister, rs1: FPURegister, rs2: FPURegister) {
            // TODO: Implement fsgnjx_d instruction emission
            println!("fsgnjx_d: rd={}, rs1={}, rs2={}", rd.0, rs1.0, rs2.0); // Placeholder
        }

        pub fn fmin_d(&mut self, rd: FPURegister, rs1: FPURegister, rs2: FPURegister) {
            // TODO: Implement fmin_d instruction emission
            println!("fmin_d: rd={}, rs1={}, rs2={}", rd.0, rs1.0, rs2.0); // Placeholder
        }

        pub fn fmax_d(&mut self, rd: FPURegister, rs1: FPURegister, rs2: FPURegister) {
            // TODO: Implement fmax_d instruction emission
            println!("fmax_d: rd={}, rs1={}, rs2={}", rd.0, rs1.0, rs2.0); // Placeholder
        }

        pub fn fcvt_s_d(
            &mut self,
            rd: FPURegister,
            rs1: FPURegister,
            frm: FPURoundingMode,
        ) {
            // TODO: Implement fcvt_s_d instruction emission
            println!(
                "fcvt_s_d: rd={}, rs1={}, frm={:?}",
                rd.0, rs1.0, frm
            ); // Placeholder
        }

        pub fn fcvt_d_s(
            &mut self,
            rd: FPURegister,
            rs1: FPURegister,
            frm: FPURoundingMode,
        ) {
            // TODO: Implement fcvt_d_s instruction emission
            println!(
                "fcvt_d_s: rd={}, rs1={}, frm={:?}",
                rd.0, rs1.0, frm
            ); // Placeholder
        }

        pub fn feq_d(&mut self, rd: Register, rs1: FPURegister, rs2: FPURegister) {
            // TODO: Implement feq_d instruction emission
            println!("feq_d: rd={}, rs1={}, rs2={}", rd.0, rs1.0, rs2.0); // Placeholder
        }

        pub fn flt_d(&mut self, rd: Register, rs1: FPURegister, rs2: FPURegister) {
            // TODO: Implement flt_d instruction emission
            println!("flt_d: rd={}, rs1={}, rs2={}", rd.0, rs1.0, rs2.0); // Placeholder
        }

        pub fn fle_d(&mut self, rd: Register, rs1: FPURegister, rs2: FPURegister) {
            // TODO: Implement fle_d instruction emission
            println!("fle_d: rd={}, rs1={}, rs2={}", rd.0, rs1.0, rs2.0); // Placeholder
        }

        pub fn fclass_d(&mut self, rd: Register, rs1: FPURegister) {
            // TODO: Implement fclass_d instruction emission
            println!("fclass_d: rd={}, rs1={}", rd.0, rs1.0); // Placeholder
        }

        pub fn fcvt_w_d(
            &mut self,
            rd: Register,
            rs1: FPURegister,
            frm: FPURoundingMode,
        ) {
            // TODO: Implement fcvt_w_d instruction emission
            println!(
                "fcvt_w_d: rd={}, rs1={}, frm={:?}",
                rd.0, rs1.0, frm
            ); // Placeholder
        }

        pub fn fcvt_wu_d(
            &mut self,
            rd: Register,
            rs1: FPURegister,
            frm: FPURoundingMode,
        ) {
            // TODO: Implement fcvt_wu_d instruction emission
            println!(
                "fcvt_wu_d: rd={}, rs1={}, frm={:?}",
                rd.0, rs1.0, frm
            ); // Placeholder
        }

        pub fn fcvt_d_w(
            &mut self,
            rd: FPURegister,
            rs1: Register,
            frm: FPURoundingMode,
        ) {
            // TODO: Implement fcvt_d_w instruction emission
            println!(
                "fcvt_d_w: rd={}, rs1={}, frm={:?}",
                rd.0, rs1.0, frm
            ); // Placeholder
        }

        pub fn fcvt_d_wu(
            &mut self,
            rd: FPURegister,
            rs1: Register,
            frm: FPURoundingMode,
        ) {
            // TODO: Implement fcvt_d_wu instruction emission
            println!(
                "fcvt_d_wu: rd={}, rs1={}, frm={:?}",
                rd.0, rs1.0, frm
            ); // Placeholder
        }

        #[cfg(target_arch = "riscv64")]
        pub fn fcvt_l_d(
            &mut self,
            rd: Register,
            rs1: FPURegister,
            frm: FPURoundingMode,
        ) {
            // TODO: Implement fcvt_l_d instruction emission
            println!(
                "fcvt_l_d: rd={}, rs1={}, frm={:?}",
                rd.0, rs1.0, frm
            ); // Placeholder
        }

        #[cfg(target_arch = "riscv64")]
        pub fn fcvt_lu_d(
            &mut self,
            rd: Register,
            rs1: FPURegister,
            frm: FPURoundingMode,
        ) {
            // TODO: Implement fcvt_lu_d instruction emission
            println!(
                "fcvt_lu_d: rd={}, rs1={}, frm={:?}",
                rd.0, rs1.0, frm
            ); // Placeholder
        }

        #[cfg(target_arch = "riscv64")]
        pub fn fmv_x_d(&mut self, rd: Register, rs1: FPURegister) {
            // TODO: Implement fmv_x_d instruction emission
            println!("fmv_x_d: rd={}, rs1={}", rd.0, rs1.0); // Placeholder
        }

        #[cfg(target_arch = "riscv64")]
        pub fn fcvt_d_l(
            &mut self,
            rd: FPURegister,
            rs1: Register,
            frm: FPURoundingMode,
        ) {
            // TODO: Implement fcvt_d_l instruction emission
            println!(
                "fcvt_d_l: rd={}, rs1={}, frm={:?}",
                rd.0, rs1.0, frm
            ); // Placeholder
        }

        #[cfg(target_arch = "riscv64")]
        pub fn fcvt_d_lu(
            &mut self,
            rd: FPURegister,
            rs1: Register,
            frm: FPURoundingMode,
        ) {
            // TODO: Implement fcvt_d_lu instruction emission
            println!(
                "fcvt_d_lu: rd={}, rs1={}, frm={:?}",
                rd.0, rs1.0, frm
            ); // Placeholder
        }

        #[cfg(target_arch = "riscv64")]
        pub fn fmv_d_x(&mut self, rd: FPURegister, rs1: Register) {
            // TODO: Implement fmv_d_x instruction emission
            println!("fmv_d_x: rd={}, rs1={}", rd.0, rs1.0); // Placeholder
        }

        pub fn fmv_d(&mut self, rd: FPURegister, rs: FPURegister) {
            self.fsgnj_d(rd, rs, rs);
        }

        pub fn fabs_d(&mut self, rd: FPURegister, rs: FPURegister) {
            self.fsgnjx_d(rd, rs, rs);
        }

        pub fn fneg_d(&mut self, rd: FPURegister, rs: FPURegister) {
            self.fsgnjn_d(rd, rs, rs);
        }
    }
}