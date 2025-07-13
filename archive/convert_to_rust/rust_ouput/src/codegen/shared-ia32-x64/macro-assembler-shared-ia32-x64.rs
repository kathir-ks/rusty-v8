// Converted from V8 C++ source files:
// Header: macro-assembler-shared-ia32-x64.h
// Implementation: macro-assembler-shared-ia32-x64.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_camel_case_types)]

use std::optional::Optional;

use crate::base::macros::truncate_to_int3;
use crate::codegen::cpu_features::{CpuFeature, CpuFeatures, CpuFeatureScope};
use crate::codegen::external_reference::ExternalReference;
use crate::codegen::macro_assembler_base::MacroAssemblerBase;

#[cfg(target_arch = "x86")]
use crate::codegen::ia32::register_ia32::{Register, XMMRegister, FPURegister};

#[cfg(target_arch = "x86_64")]
use crate::codegen::x64::register_x64::{Register, XMMRegister, FPURegister};

use crate::codegen::assembler::Assembler;

#[cfg(target_arch = "x86")]
use crate::codegen::ia32::assembler_ia32::Immediate;

#[cfg(target_arch = "x86_64")]
use crate::codegen::x64::assembler_x64::Immediate;

use crate::codegen::assembler::Operand;

const kDoubleSize: i32 = 8;

#[cfg(target_arch = "x86")]
const kStackSavedSavedFPSize : i32 = kDoubleSize;
#[cfg(target_arch = "x86_64")]
const kStackSavedSavedFPSize : i32 = kDoubleSize;

#[derive(Debug)]
pub struct SharedMacroAssemblerBase {
    base: MacroAssemblerBase,
}

impl SharedMacroAssemblerBase {
    pub fn new(assembler: Assembler) -> Self {
        Self {
            base: MacroAssemblerBase::new(assembler),
        }
    }

    pub fn move_register(&mut self, dst: Register, src: u32) {
        #[cfg(target_arch = "x86")]
        self.base.assembler.borrow_mut().mov(dst, Immediate(src));

        #[cfg(target_arch = "x86_64")]
        self.base.assembler.borrow_mut().movl(dst, Immediate(src));
    }

   pub fn move_register_to_register(&mut self, dst: Register, src: Register) {
        if dst != src {
            #[cfg(target_arch = "x86")]
            self.base.assembler.borrow_mut().mov(dst, src);

            #[cfg(target_arch = "x86_64")]
            self.base.assembler.borrow_mut().movq(dst, src);
        }
    }

     pub fn add_register_immediate(&mut self, dst: Register, src: Immediate) {
        #[cfg(target_arch = "x86")]
        self.base.assembler.borrow_mut().add(dst, src);

        #[cfg(target_arch = "x86_64")]
        self.base.assembler.borrow_mut().addq(dst, src);
    }

    pub fn and_register_immediate(&mut self, dst: Register, src: Immediate) {
        #[cfg(target_arch = "x86")]
        self.base.assembler.borrow_mut().and_(dst, src);

        #[cfg(target_arch = "x86_64")]
        {
            if src.value() <= u32::MAX as i64 {
                self.base.assembler.borrow_mut().andl(dst, src);
            } else {
                self.base.assembler.borrow_mut().andq(dst, src);
            }
        }
    }

    pub fn movhps(&mut self, dst: XMMRegister, src1: XMMRegister, src2: Operand) {
        if CpuFeatures::is_supported(CpuFeature::AVX) {
            let mut avx_scope = CpuFeatureScope::new(&mut self.base.assembler, CpuFeature::AVX);
            self.base.assembler.borrow_mut().vmovhps(dst, src1, src2);
        } else {
            if dst != src1 {
                self.base.assembler.borrow_mut().movaps(dst, src1);
            }
            self.base.assembler.borrow_mut().movhps(dst, src2);
        }
    }

    pub fn movlps(&mut self, dst: XMMRegister, src1: XMMRegister, src2: Operand) {
        if CpuFeatures::is_supported(CpuFeature::AVX) {
            let mut avx_scope = CpuFeatureScope::new(&mut self.base.assembler, CpuFeature::AVX);
            self.base.assembler.borrow_mut().vmovlps(dst, src1, src2);
        } else {
            if dst != src1 {
                self.base.assembler.borrow_mut().movaps(dst, src1);
            }
            self.base.assembler.borrow_mut().movlps(dst, src2);
        }
    }

    pub fn blendvpd(&mut self, dst: XMMRegister, src1: XMMRegister, src2: XMMRegister, mask: XMMRegister) {
        if CpuFeatures::is_supported(CpuFeature::AVX) {
            let mut avx_scope = CpuFeatureScope::new(&mut self.base.assembler, CpuFeature::AVX);
            self.base.assembler.borrow_mut().vblendvpd(dst, src1, src2, mask);
        } else {
            let mut sse4_1_scope = CpuFeatureScope::new(&mut self.base.assembler, CpuFeature::SSE4_1);
            assert_eq!(mask, XMMRegister::xmm0());
            assert_eq!(dst, src1);
            self.base.assembler.borrow_mut().blendvpd(dst, src2);
        }
    }

    pub fn blendvps(&mut self, dst: XMMRegister, src1: XMMRegister, src2: XMMRegister, mask: XMMRegister) {
        if CpuFeatures::is_supported(CpuFeature::AVX) {
            let mut avx_scope = CpuFeatureScope::new(&mut self.base.assembler, CpuFeature::AVX);
            self.base.assembler.borrow_mut().vblendvps(dst, src1, src2, mask);
        } else {
            let mut sse4_1_scope = CpuFeatureScope::new(&mut self.base.assembler, CpuFeature::SSE4_1);
            assert_eq!(mask, XMMRegister::xmm0());
            assert_eq!(dst, src1);
            self.base.assembler.borrow_mut().blendvps(dst, src2);
        }
    }

    pub fn pblendvb(&mut self, dst: XMMRegister, src1: XMMRegister, src2: XMMRegister, mask: XMMRegister) {
        if CpuFeatures::is_supported(CpuFeature::AVX) {
            let mut avx_scope = CpuFeatureScope::new(&mut self.base.assembler, CpuFeature::AVX);
            self.base.assembler.borrow_mut().vpblendvb(dst, src1, src2, mask);
        } else {
            let mut sse4_1_scope = CpuFeatureScope::new(&mut self.base.assembler, CpuFeature::SSE4_1);
            assert_eq!(mask, XMMRegister::xmm0());
            assert_eq!(dst, src1);
            self.base.assembler.borrow_mut().pblendvb(dst, src2);
        }
    }

   pub fn shufps(&mut self, dst: XMMRegister, src1: XMMRegister, src2: XMMRegister, imm8: u8) {
        if CpuFeatures::is_supported(CpuFeature::AVX) {
            let mut avx_scope = CpuFeatureScope::new(&mut self.base.assembler, CpuFeature::AVX);
            self.base.assembler.borrow_mut().vshufps(dst, src1, src2, imm8);
        } else {
            if dst != src1 {
                self.base.assembler.borrow_mut().movaps(dst, src1);
            }
            self.base.assembler.borrow_mut().shufps(dst, src2, imm8);
        }
    }
}

impl SharedMacroAssemblerBase {
    fn pinsr_helper<Op>(
        &mut self,
        avx: fn(&mut Assembler, XMMRegister, XMMRegister, Op, u8),
        noavx: fn(&mut Assembler, XMMRegister, Op, u8),
        dst: XMMRegister,
        src1: XMMRegister,
        src2: Op,
        imm8: u8,
        load_pc_offset: &mut Option<u32>,
        feature: Option<CpuFeature>,
    ) {
        if CpuFeatures::is_supported(CpuFeature::AVX) {
            let mut avx_scope = CpuFeatureScope::new(&mut self.base.assembler, CpuFeature::AVX);
             if let Some(offset) = load_pc_offset {
                *offset = self.base.assembler.borrow().pc_offset() as u32;
            }
            avx(&mut self.base.assembler.borrow_mut(), dst, src1, src2, imm8);
            return;
        }

        if dst != src1 {
            self.base.assembler.borrow_mut().movaps(dst, src1);
        }
        if let Some(offset) = load_pc_offset {
            *offset = self.base.assembler.borrow().pc_offset() as u32;
        }
        if let Some(feat) = feature {
            assert!(CpuFeatures::is_supported(feat));
            let mut scope = CpuFeatureScope::new(&mut self.base.assembler, feat);
            noavx(&mut self.base.assembler.borrow_mut(), dst, src2, imm8);
        } else {
            noavx(&mut self.base.assembler.borrow_mut(), dst, src2, imm8);
        }
    }

     pub fn pinsrb<Op>(
        &mut self,
        dst: XMMRegister,
        src1: XMMRegister,
        src2: Op,
        imm8: u8,
        load_pc_offset: &mut Option<u32>,
    ) {
        self.pinsr_helper(
            Assembler::vpinsrb,
            Assembler::pinsrb,
            dst,
            src1,
            src2,
            imm8,
            load_pc_offset,
            Some(CpuFeature::SSE4_1),
        );
    }

    pub fn pinsrw<Op>(
        &mut self,
        dst: XMMRegister,
        src1: XMMRegister,
        src2: Op,
        imm8: u8,
        load_pc_offset: &mut Option<u32>,
    ) {
        self.pinsr_helper(
            Assembler::vpinsrw,
            Assembler::pinsrw,
            dst,
            src1,
            src2,
            imm8,
            load_pc_offset,
            None,
        );
    }

     pub fn pshufb<Op>(&mut self, dst: XMMRegister, src: XMMRegister, mask: Op) {
        if CpuFeatures::is_supported(CpuFeature::AVX) {
            let mut avx_scope = CpuFeatureScope::new(&mut self.base.assembler, CpuFeature::AVX);
            self.base.assembler.borrow_mut().vpshufb(dst, src, mask);
        } else {
            assert_ne!(dst, mask);
            if dst != src {
                self.base.assembler.borrow_mut().movaps(dst, src);
            }
             let mut ssse3_scope = CpuFeatureScope::new(&mut self.base.assembler, CpuFeature::SSSE3);
            self.base.assembler.borrow_mut().pshufb(dst, mask);
        }
    }

    pub fn pshufb_op<Op>(&mut self, dst: XMMRegister, mask: Op) {
        self.pshufb(dst, dst, mask);
    }

    pub fn f64x2_extract_lane(&mut self, dst: DoubleRegister, src: XMMRegister, lane: u8) {
      if lane == 0 {
        if dst != src {
            self.base.assembler.borrow_mut().movaps(dst, src);
        }
      } else {
        assert_eq!(1, lane);
        if CpuFeatures::is_supported(CpuFeature::AVX) {
             let mut avx_scope = CpuFeatureScope::new(&mut self.base.assembler, CpuFeature::AVX);
            self.base.assembler.borrow_mut().vmovhlps(dst, src, src);
        } else {
           self.base.assembler.borrow_mut().movhlps(dst, src);
        }
      }
    }

    pub fn f64x2_replace_lane(&mut self, dst: XMMRegister, src: XMMRegister, rep: DoubleRegister, lane: u8) {
         if CpuFeatures::is_supported(CpuFeature::AVX) {
             let mut avx_scope = CpuFeatureScope::new(&mut self.base.assembler, CpuFeature::AVX);
            if lane == 0 {
               self.base.assembler.borrow_mut().vmovsd(dst, src, rep);
            } else {
                self.base.assembler.borrow_mut().vmovlhps(dst, src, rep);
            }
        } else {
            let mut sse4_1_scope = CpuFeatureScope::new(&mut self.base.assembler, CpuFeature::SSE4_1);
            if dst != src {
                assert_ne!(dst, rep);
               self.base.assembler.borrow_mut().movaps(dst, src);
            }
            if lane == 0 {
                self.base.assembler.borrow_mut().movsd(dst, rep);
            } else {
                self.base.assembler.borrow_mut().movlhps(dst, rep);
            }
        }
    }

    pub fn f32x4_splat(&mut self, dst: XMMRegister, src: DoubleRegister) {
    if CpuFeatures::is_supported(CpuFeature::AVX2) {
      let mut avx2_scope = CpuFeatureScope::new(&mut self.base.assembler, CpuFeature::AVX2);
      self.base.assembler.borrow_mut().vbroadcastss(dst, src);
    } else if CpuFeatures::is_supported(CpuFeature::AVX)) {
         let mut avx_scope = CpuFeatureScope::new(&mut self.base.assembler, CpuFeature::AVX);
      self.base.assembler.borrow_mut().vshufps(dst, src, src, 0);
    } else {
      if dst == src {
        // 1 byte shorter than pshufd.
           self.base.assembler.borrow_mut().shufps(dst, src, 0);
      } else {
           self.base.assembler.borrow_mut().pshufd(dst, src, 0);
      }
    }
  }

     pub fn f32x4_extract_lane(&mut self, dst: FloatRegister, src: XMMRegister, lane: u8) {
    assert!(lane < 4);
    // These instructions are shorter than insertps, but will leave junk in
    // the top lanes of dst.
    if lane == 0 {
      if dst != src {
         self.base.assembler.borrow_mut().movaps(dst, src);
      }
    } else if lane == 1 {
         self.base.assembler.borrow_mut().movshdup(dst, src);
    } else if lane == 2 && dst == src {
      // Check dst == src to avoid false dependency on dst.
         self.base.assembler.borrow_mut().movhlps(dst, src);
    } else if dst == src {
        self.base.assembler.borrow_mut().shufps(dst, src, lane);
    } else {
         self.base.assembler.borrow_mut().pshufd(dst, src, lane);
    }
  }

   pub fn f32x4_min(&mut self, dst: XMMRegister, lhs: XMMRegister, rhs: XMMRegister, scratch: XMMRegister) {
  // The minps instruction doesn't propagate NaNs and +0's in its first
  // operand. Perform minps in both orders, merge the results, and adjust.
  if CpuFeatures::is_supported(CpuFeature::AVX)) {
     let mut avx_scope = CpuFeatureScope::new(&mut self.base.assembler, CpuFeature::AVX);
    self.base.assembler.borrow_mut().vminps(scratch, lhs, rhs);
    self.base.assembler.borrow_mut().vminps(dst, rhs, lhs);
  } else if dst == lhs || dst == rhs {
    let src = if dst == lhs { rhs } else { lhs };
    self.base.assembler.borrow_mut().movaps(scratch, src);
    self.base.assembler.borrow_mut().minps(scratch, dst);
    self.base.assembler.borrow_mut().minps(dst, src);
  } else {
     self.base.assembler.borrow_mut().movaps(scratch, lhs);
    self.base.assembler.borrow_mut().minps(scratch, rhs);
    self.base.assembler.borrow_mut().movaps(dst, rhs);
    self.base.assembler.borrow_mut().minps(dst, lhs);
  }
  // Propagate -0's and NaNs, which may be non-canonical.
  self.xorps_avx(scratch, dst);
  // Canonicalize NaNs by quieting and clearing the payload.
  self.cmpunordps_avx(dst, dst, scratch);
  self.xorps_avx(scratch, dst);
  self.base.assembler.borrow_mut().psrld(dst, 10);
  self.andnps_avx(dst, dst, scratch);
}

   pub fn f32x4_max(&mut self, dst: XMMRegister, lhs: XMMRegister, rhs: XMMRegister, scratch: XMMRegister) {
  // The maxps instruction doesn't propagate NaNs and +0's in its first
  // operand. Perform maxps in both orders, merge the results, and adjust.
  if CpuFeatures::is_supported(CpuFeature::AVX)) {
     let mut avx_scope = CpuFeatureScope::new(&mut self.base.assembler, CpuFeature::AVX);
    self.base.assembler.borrow_mut().vmaxps(scratch, lhs, rhs);
    self.base.assembler.borrow_mut().vmaxps(dst, rhs, lhs);
  } else if dst == lhs || dst == rhs {
    let src = if dst == lhs { rhs } else { lhs };
    self.base.assembler.borrow_mut().movaps(scratch, src);
    self.base.assembler.borrow_mut().maxps(scratch, dst);
    self.base.assembler.borrow_mut().maxps(dst, src);
  } else {
     self.base.assembler.borrow_mut().movaps(scratch, lhs);
    self.base.assembler.borrow_mut().maxps(scratch, rhs);
    self.base.assembler.borrow_mut().movaps(dst, rhs);
    self.base.assembler.borrow_mut().maxps(dst, lhs);
  }
  // Find discrepancies.
  self.xorps_avx(dst, dst, scratch);
  // Propagate NaNs, which may be non-canonical.
  self.xorps_avx(scratch, scratch, dst);
  // Propagate sign discrepancy and (subtle) quiet NaNs.
  self.base.assembler.borrow_mut().subps(scratch, dst);
  // Canonicalize NaNs by clearing the payload. Sign is non-deterministic.
  self.cmpunordps_avx(dst, dst, scratch);
  self.base.assembler.borrow_mut().psrld(dst, 10);
  self.andnps_avx(dst, dst, scratch);
}

    pub fn f64x2_min(&mut self, dst: XMMRegister, lhs: XMMRegister, rhs: XMMRegister, scratch: XMMRegister) {
  ASM_CODE_COMMENT(self);
  if CpuFeatures::is_supported(CpuFeature::AVX)) {
     let mut avx_scope = CpuFeatureScope::new(&mut self.base.assembler, CpuFeature::AVX);
    // The minpd instruction doesn't propagate NaNs and +0's in its first
    // operand. Perform minpd in both orders, merge the resuls, and adjust.
    self.base.assembler.borrow_mut().vminpd(scratch, lhs, rhs);
    self.base.assembler.borrow_mut().vminpd(dst, rhs, lhs);
    // propagate -0's and NaNs, which may be non-canonical.
    self.xorps_avx(scratch, scratch, dst);
    // Canonicalize NaNs by quieting and clearing the payload.
    self.cmpunordpd_avx(dst, dst, scratch);
    self.xorps_avx(scratch, scratch, dst);
    self.base.assembler.borrow_mut().psrlq(dst, 13);
    self.andnps_avx(dst, dst, scratch);
  } else {
    // Compare lhs with rhs, and rhs with lhs, and have the results in scratch
    // and dst. If dst overlaps with lhs or rhs, we can save a move.
    if dst == lhs || dst == rhs) {
      let src = if dst == lhs { rhs } else { lhs };
      self.base.assembler.borrow_mut().movaps(scratch, src);
      self.base.assembler.borrow_mut().minpd(scratch, dst);
      self.base.assembler.borrow_mut().minpd(dst, src);
    } else {
        self.base.assembler.borrow_mut().movaps(scratch, lhs);
        self.base.assembler.borrow_mut().movaps(dst, rhs);
        self.base.assembler.borrow_mut().minpd(scratch, rhs);
        self.base.assembler.borrow_mut().minpd(dst, lhs);
    }
    self.xorps_avx(scratch, scratch, dst);
    self.cmpunordpd_avx(dst, scratch, scratch);
    self.xorps_avx(scratch, scratch, dst);
    self.base.assembler.borrow_mut().psrlq(dst, 13);
    self.andnps_avx(dst, dst, scratch);
  }
}

   pub fn f64x2_max(&mut self, dst: XMMRegister, lhs: XMMRegister, rhs: XMMRegister, scratch: XMMRegister) {
  // The maxpd instruction doesn't propagate NaNs and +0's in its first
  // operand. Perform maxpd in both orders, merge the resuls, and adjust.
  if CpuFeatures::is_supported(CpuFeature::AVX)) {
     let mut avx_scope = CpuFeatureScope::new(&mut self.base.assembler, CpuFeature::AVX);
    self.base.assembler.borrow_mut().vmaxpd(scratch, lhs, rhs);
    self.base.assembler.borrow_mut().vmaxpd(dst, rhs, lhs);
    // Find discrepancies.
    self.xorps_avx(dst, dst, scratch);
    // Propagate NaNs, which may be non-canonical.
    self.xorps_avx(scratch, scratch, dst);
    // Propagate sign discrepancy and (subtle) quiet NaNs.
    self.base.assembler.borrow_mut().subpd(scratch, dst);
    // Canonicalize NaNs by clearing the payload. Sign is non-deterministic.
    self.cmpunordpd_avx(dst, dst, scratch);
    self.base.assembler.borrow_mut().psrlq(dst, 13);
    self.andnps_avx(dst, dst, scratch);
  } else {
    if (dst == lhs || dst == rhs) {
      let src = if dst == lhs { rhs } else { lhs };
      self.base.assembler.borrow_mut().movaps(scratch, src);
      self.base.assembler.borrow_mut().maxpd(scratch, dst);
      self.base.assembler.borrow_mut().maxpd(dst, src);
    } else {
        self.base.assembler.borrow_mut().movaps(scratch, lhs);
        self.base.assembler.borrow_mut().movaps(dst, rhs);
        self.base.assembler.borrow_mut().maxpd(scratch, rhs);
        self.base.assembler.borrow_mut().maxpd(dst, lhs);
    }
     self.xorps_avx(dst, dst, scratch);
    self.xorps_avx(scratch, scratch, dst);
    self.base.assembler.borrow_mut().subpd(scratch, dst);
    self.cmpunordpd_avx(dst, scratch, scratch);
    self.base.assembler.borrow_mut().psrlq(dst, 13);
    self.andnps_avx(dst, dst, scratch);
  }
}

   pub fn s128_store32_lane(&mut self, dst: Operand, src: XMMRegister, laneidx: u8) {
    if laneidx == 0 {
      self.base.assembler.borrow_mut().movss(dst, src);
    } else {
        assert!(laneidx <= 3);
        self.base.assembler.borrow_mut().extractps(dst, src, laneidx);
    }
  }

   pub fn i8x16_splat(&mut self, dst: XMMRegister, src: Register, scratch: XMMRegister) {
      if CpuFeatures::is_supported(CpuFeature::AVX2) {
         let mut avx2_scope = CpuFeatureScope::new(&mut self.base.assembler, CpuFeature::AVX2);
         self.base.assembler.borrow_mut().movd(scratch, src);
         self.base.assembler.borrow_mut().vpbroadcastb(dst, scratch);
      } else {
          let _ssse3_scope = CpuFeatureScope::new(&mut self.base.assembler, CpuFeature::SSSE3);
          self.i8x16_splat_pre_avx2(dst, src, scratch);
      }
  }

  fn i8x16_splat_pre_avx2<Op>(&mut self, dst: XMMRegister, src: Op, scratch: XMMRegister) {
     self.base.assembler.borrow_mut().movd(dst, src);
        self.xorps_avx(scratch, scratch, scratch);
        let _ssse3_scope = CpuFeatureScope::new(&mut self.base.assembler, CpuFeature::SSSE3);
         self.base.assembler.borrow_mut().pshufb(dst, scratch);
  }

 pub fn i8x16_shl(&mut self, dst: XMMRegister, src1: XMMRegister, src2: u8, tmp1: Register, tmp2: XMMRegister) {
  assert_ne!(dst, tmp2);
  if !CpuFeatures::is_supported(CpuFeature::AVX) && (dst != src1) {
     self.base.assembler.borrow_mut().movaps(dst, src1);
  }

  let shift = truncate_to_int3(src2);
  self.base.assembler.borrow_mut().psllw(dst, shift);

  let bmask = 0xff << shift;
  let mask = bmask << 24 | bmask << 16 | bmask << 8 | bmask;
  self.move_register(tmp1, mask as u32);
  self.base.assembler.borrow_mut().movd(tmp2, tmp1);
  self.base.assembler.borrow_mut().pshufd(tmp2, tmp2, 0);
  self.base.assembler.borrow_mut().pand(dst, tmp2);
}

pub fn i8x16_shr_s(&mut self, dst: XMMRegister, src1: XMMRegister, src2: u8, tmp: XMMRegister) {
  // Unpack bytes into words, do word (16-bit) shifts, and repack.
  assert_ne!(dst, tmp);
  let shift = truncate_to_int3(src2) + 8;

  self.base.assembler.borrow_mut().punpckhbw(tmp, src1);
  self.base.assembler.borrow_mut().punpcklbw(dst, src1);
   self.base.assembler.borrow_mut().psraw(tmp, shift);
  self.base.assembler.borrow_mut().psraw(dst, shift);
   self.base.assembler.borrow_mut().packsswb(dst, tmp);
}

fn xorps_avx(&mut self, dst: XMMRegister, src: XMMRegister) {
        if CpuFeatures::is_supported(CpuFeature::AVX) {
            let mut avx_scope = CpuFeatureScope::new(&mut self.base.assembler, CpuFeature::AVX);
            self.base.assembler.borrow_mut().vxorps(dst, dst, src);
        } else {
            self.base.assembler.borrow_mut().xorps(dst, src);
        }
    }

   fn cmpleps_avx(&mut self, scratch: XMMRegister, src: XMMRegister) {
        if CpuFeatures::is_supported(CpuFeature::AVX) {
            let mut avx_scope = CpuFeatureScope::new(&mut self.base.assembler, CpuFeature::AVX);
           self.base.assembler.borrow_mut().vcmpgeps(scratch, src, ExternalReferenceAsOperand(ExternalReference::address_of_wasm_int32_overflow_as_float(), Register::rax()));
        } else {
            self.base.assembler.borrow_mut().cmpleps(scratch, src);
        }
    }

    fn cmpunordps_avx(&mut self, dst: XMMRegister, dst2: XMMRegister, scratch:XMMRegister) {
        if CpuFeatures::is_supported(CpuFeature::AVX)) {
            let mut avx_scope = CpuFeatureScope::new(&mut self.base.assembler, CpuFeature::AVX);
            self.base.assembler.borrow_mut().vcmpeqps(dst, dst2, scratch);
        } else {
            self.base.assembler.borrow_mut().cmpunordps(dst, scratch);
        }
    }

    fn subps_avx(&mut self, dst: XMMRegister, scratch: XMMRegister) {
        if CpuFeatures::is_supported(CpuFeature::AVX)) {
            let mut avx_scope = CpuFeatureScope::new(&mut self.base.assembler, CpuFeature::AVX);
            self.base.assembler.borrow_mut().vsubps(scratch, scratch, dst);
        } else {
             self.base.assembler.borrow_mut().subps(scratch, dst);
        }
    }

    fn andnps_avx(&mut self, dst: XMMRegister, dst2: XMMRegister, scratch: XMMRegister) {
        if CpuFeatures::is_supported(CpuFeature::AVX)) {
            let mut avx_scope = CpuFeatureScope::new(&mut self.base.assembler, CpuFeature::AVX);
            self.base.assembler.borrow_mut().vandnps(dst, dst2, scratch);
        } else {
             self.base.assembler.borrow_mut().andnps(dst, scratch);
        }
    }
}

trait SharedMacroAssembler {
    fn abspd(&mut self, dst: XMMRegister, src: XMMRegister, tmp: Register);
    fn absps(&mut self, dst: XMMRegister, src: XMMRegister, tmp: Register);
    fn absph(&mut self, dst: XMMRegister, src: XMMRegister, tmp: Register);
    fn negpd(&mut self, dst: XMMRegister, src: XMMRegister, tmp: Register);
    fn negps(&mut self, dst: XMMRegister, src: XMMRegister, tmp: Register);
    fn negph(&mut self, dst: XMMRegister, src: XMMRegister, tmp: Register);
    fn pextrd(&mut self, dst: Register, src: XMMRegister, imm8: u8);
    fn pinsrd<Op>(&mut self, dst: XMMRegister, src1: XMMRegister, src2: Op, imm8: u8, load_pc_offset: &mut Option<u32>);
    fn pinsrd_op<Op>(&mut self, dst: XMMRegister, src: Op, imm8: u8, load_pc_offset: &mut Option<u32>);
    fn f64x2_convert_low_i32x4_u(&mut self, dst: XMMRegister, src: XMMRegister, scratch: Register);
    fn i32x4_sconvert_f32x4(&mut self, dst: XMMRegister, src: XMMRegister, tmp: XMMRegister, scratch: Register);
    fn i32x4_trunc_sat_f64x2_s_zero(&mut self, dst: XMMRegister, src: XMMRegister, scratch: XMMRegister, tmp: Register);
    fn i32x4_trunc_sat_f64x2_u_
