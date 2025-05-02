// Copyright 2013 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![cfg(target_arch = "aarch64")]

mod arm64 {
    pub mod instructions_arm64 {
        use std::mem::size_of;

        //use crate::codegen::arm64::assembler_arm64_inl::*; // Assuming a Rust equivalent
        //use crate::common::code_memory_access_inl::*; // Assuming a Rust equivalent

        // Placeholder types for Assembler and WritableJitAllocation
        pub struct WritableJitAllocation {}
        pub struct AssemblerOptions {}
        pub struct Zone {}

        pub type Instr = u32;
        pub type Address = usize;

        // Constants representing register sizes
        pub const KWREG_SIZE_IN_BITS: u32 = 32;
        pub const KXREG_SIZE_IN_BITS: u32 = 64;
        const KINSTR_SIZE: usize = 4;
        const KINSTR_SIZE_LOG2: usize = 2;
        const KLOAD_LITERAL_SCALE_LOG2: usize = 2;
        const KQREG_SIZE_LOG2: u32 = 4;
        const KXREG_SIZE_LOG2: u32 = 3;
        const KDREG_SIZE: u32 = 8;
        const KWREG_SIZE_LOG2: u32 = 2;
        const KSRegSize: u32 = 4;

        // Masks and fixed values.  These will need to be populated with
        // the correct values from the C++ code.
        const LOAD_STORE_ANY_FMASK: Instr = 0xFFFFFFFF;
        const LOAD_STORE_ANY_FIXED: Instr = 0x00000000;
        const LOAD_STORE_PAIR_ANY_FMASK: Instr = 0xFFFFFFFF;
        const LOAD_STORE_PAIR_ANY_FIXED: Instr = 0x00000000;
        const LOAD_STORE_PAIR_LBIT: Instr = 0x00000000;
        const LOAD_STORE_MASK: Instr = 0xFFFFFFFF;
        const IMM_PCREL_MASK: Instr = 0xFFFFFFFF;
        const IMM_LLITERAL_MASK: Instr = 0xFFFFFFFF;

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum LoadStoreOp {
            LDRB_w,
            LDRH_w,
            LDR_w,
            LDR_x,
            LDRSB_w,
            LDRSB_x,
            LDRSH_w,
            LDRSH_x,
            LDRSW_x,
            LDR_b,
            LDR_h,
            LDR_s,
            LDR_d,
            LDR_q,
            STRB_w,
            STRH_w,
            STR_w,
            STR_x,
            STR_b,
            STR_h,
            STR_s,
            STR_d,
            STR_q,
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum LoadStorePairOp {
            STP_q,
            LDP_q,
            STP_x,
            LDP_x,
            STP_d,
            LDP_d,
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum BranchType {
            UnknownBranchType,
            // Placeholder variants, add actual branch types as needed
            CondBranchType,
            UncondBranchType,
            CompareBranchType,
            TestBranchType,
        }

        // Placeholder consts. To be replaced with values from C++.
        const LSSIZE_OFFSET: u32 = 0;
        const LSSIZE_WIDTH: u32 = 0;
        const LSVECTOR_MASK: Instr = 0x00000000;
        const LSOPC_MASK: Instr = 0x00000000;
        const LSOPC_OFFSET: u32 = 0;
        #[derive(Debug)]
        pub struct Instruction {
            instruction_bits: Instr,
        }

        impl Instruction {
            pub fn new(instruction_bits: Instr) -> Self {
                Instruction { instruction_bits }
            }

            pub fn InstructionBits(&self) -> Instr {
                self.instruction_bits
            }

            pub fn SetInstructionBits(&mut self, new_instr: Instr, jit_allocation: Option<&mut WritableJitAllocation>) {
                // Usually this is aligned, but when de/serializing that's not guaranteed.
                if let Some(jit_allocation) = jit_allocation {
                    //jit_allocation.WriteUnalignedValue(self as *mut Self as Address, new_instr);
                    //Placeholder: Need a safe Rust equivalent for unaligned writes using jit_allocation
                    unsafe {
                        (self as *mut Self as *mut Instr).write_unaligned(new_instr);
                    }
                } else {
                    //base::WriteUnalignedValue(self as *mut Self as Address, new_instr);
                    //Placeholder: Need a safe Rust equivalent for unaligned writes using base
                    unsafe {
                         (self as *mut Self as *mut Instr).write_unaligned(new_instr);
                    }
                }
            }

            fn Mask(&self, mask: Instr) -> Instr {
                self.instruction_bits & mask
            }

            pub fn IsLoad(&self) -> bool {
                if self.Mask(LOAD_STORE_ANY_FMASK) != LOAD_STORE_ANY_FIXED {
                    return false;
                }

                if self.Mask(LOAD_STORE_PAIR_ANY_FMASK) == LOAD_STORE_PAIR_ANY_FIXED {
                    return (self.Mask(LOAD_STORE_PAIR_LBIT) != 0);
                } else {
                    let op = self.Mask(LOAD_STORE_MASK);
                    match op {
                        x if x == LoadStoreOp::LDRB_w as Instr => true,
                        x if x == LoadStoreOp::LDRH_w as Instr => true,
                        x if x == LoadStoreOp::LDR_w as Instr => true,
                        x if x == LoadStoreOp::LDR_x as Instr => true,
                        x if x == LoadStoreOp::LDRSB_w as Instr => true,
                        x if x == LoadStoreOp::LDRSB_x as Instr => true,
                        x if x == LoadStoreOp::LDRSH_w as Instr => true,
                        x if x == LoadStoreOp::LDRSH_x as Instr => true,
                        x if x == LoadStoreOp::LDRSW_x as Instr => true,
                        x if x == LoadStoreOp::LDR_b as Instr => true,
                        x if x == LoadStoreOp::LDR_h as Instr => true,
                        x if x == LoadStoreOp::LDR_s as Instr => true,
                        x if x == LoadStoreOp::LDR_d as Instr => true,
                        x if x == LoadStoreOp::LDR_q as Instr => true,
                        _ => false,
                    }
                }
            }

            pub fn IsStore(&self) -> bool {
                if self.Mask(LOAD_STORE_ANY_FMASK) != LOAD_STORE_ANY_FIXED {
                    return false;
                }

                if self.Mask(LOAD_STORE_PAIR_ANY_FMASK) == LOAD_STORE_PAIR_ANY_FIXED {
                    return self.Mask(LOAD_STORE_PAIR_LBIT) == 0;
                } else {
                    let op = self.Mask(LOAD_STORE_MASK);
                    match op {
                        x if x == LoadStoreOp::STRB_w as Instr => true,
                        x if x == LoadStoreOp::STRH_w as Instr => true,
                        x if x == LoadStoreOp::STR_w as Instr => true,
                        x if x == LoadStoreOp::STR_x as Instr => true,
                        x if x == LoadStoreOp::STR_b as Instr => true,
                        x if x == LoadStoreOp::STR_h as Instr => true,
                        x if x == LoadStoreOp::STR_s as Instr => true,
                        x if x == LoadStoreOp::STR_d as Instr => true,
                        x if x == LoadStoreOp::STR_q as Instr => true,
                        _ => false,
                    }
                }
            }
            fn BitN(&self) -> i32 {
                0 // Placeholder
            }

            fn ImmSetBits(&self) -> i32 {
                0 // Placeholder
            }

            fn ImmRotate(&self) -> i32 {
                0 // Placeholder
            }

            fn SixtyFourBits(&self) -> bool {
                true // Placeholder
            }

            pub fn ImmLogical(&self) -> u64 {
                let reg_size = if self.SixtyFourBits() {
                    KXREG_SIZE_IN_BITS
                } else {
                    KWREG_SIZE_IN_BITS
                };
                let n = self.BitN();
                let imm_s = self.ImmSetBits();
                let imm_r = self.ImmRotate();

                if n == 1 {
                    if imm_s == 0x3F {
                        return 0;
                    }
                    let bits = (1u64 << (imm_s + 1)) - 1;
                    return rotate_right(bits, imm_r as u32, 64);
                } else {
                    if (imm_s >> 1) == 0x1F {
                        return 0;
                    }
                    let mut width = 0x20;
                    while width >= 0x2 {
                        if (imm_s & width) == 0 {
                            let mask = width - 1;
                            if (imm_s & mask) == mask {
                                return 0;
                            }
                            let bits = (1u64 << ((imm_s & mask) + 1)) - 1;
                            return repeat_bits_across_reg(
                                reg_size,
                                rotate_right(bits, (imm_r & mask) as u32, width),
                                width,
                            );
                        }
                        width >>= 1;
                    }
                }
                panic!("UNREACHABLE");
            }

            fn ImmNEONabc(&self) -> u32 {
                0 // Placeholder
            }

            fn ImmNEONdefgh(&self) -> u32 {
                0 // Placeholder
            }

            pub fn ImmNEONabcdefgh(&self) -> u32 {
                (self.ImmNEONabc() << 5) | self.ImmNEONdefgh()
            }

            fn ImmFP(&self) -> u32 {
                0 // Placeholder
            }

            pub fn ImmFP32(&self) -> f32 {
                imm8_to_fp32(self.ImmFP())
            }

            pub fn ImmFP64(&self) -> f64 {
                imm8_to_fp64(self.ImmFP())
            }

            pub fn ImmNEONFP32(&self) -> f32 {
                imm8_to_fp32(self.ImmNEONabcdefgh())
            }

            pub fn ImmNEONFP64(&self) -> f64 {
                imm8_to_fp64(self.ImmNEONabcdefgh())
            }

            fn IsPCRelAddressing(&self) -> bool {
                false // Placeholder
            }

            fn BranchType(&self) -> BranchType {
                BranchType::UnknownBranchType // Placeholder
            }

            fn IsUnresolvedInternalReference(&self) -> bool {
                false // Placeholder
            }

            fn IsLdrLiteral(&self) -> bool {
                false // Placeholder
            }

            fn ImmPCRel(&self) -> i64 {
                0 // Placeholder
            }

            fn ImmBranch(&self) -> i64 {
                0 // Placeholder
            }

            fn ImmUnresolvedInternalReference(&self) -> i64 {
                0 // Placeholder
            }

            fn ImmLLiteral(&self) -> i64 {
                0 // Placeholder
            }

            pub fn ImmPCOffset(&self) -> i64 {
                let offset: i64;
                if self.IsPCRelAddressing() {
                    // PC-relative addressing. Only ADR is supported.
                    offset = self.ImmPCRel();
                } else if self.BranchType() != BranchType::UnknownBranchType {
                    // All PC-relative branches.
                    // Relative branch offsets are instruction-size-aligned.
                    offset = self.ImmBranch() * KINSTR_SIZE as i64;
                } else if self.IsUnresolvedInternalReference() {
                    // Internal references are always word-aligned.
                    offset = self.ImmUnresolvedInternalReference() * KINSTR_SIZE as i64;
                } else {
                    // Load literal (offset from PC).
                    assert!(self.IsLdrLiteral());
                    // The offset is always shifted by 2 bits, even for loads to 64-bits
                    // registers.
                    offset = self.ImmLLiteral() * KINSTR_SIZE as i64;
                }
                offset
            }

            pub fn ImmPCOffsetTarget(&self) -> *mut Instruction {
                self.InstructionAtOffset(self.ImmPCOffset())
            }

            fn InstructionAtOffset(&self, offset: i64) -> *mut Instruction {
                ((self as *const Self as usize).wrapping_add(offset as usize)) as *mut Instruction
            }

            pub fn IsTargetInImmPCOffsetRange(&self, target: *mut Instruction) -> bool {
                self.IsValidImmPCOffset(self.BranchType(), self.DistanceTo(target))
            }

            fn DistanceTo(&self, target: *mut Instruction) -> i64 {
                (target as usize).wrapping_sub(self as *const Self as usize) as i64
            }

            fn IsValidImmPCOffset(&self, branch_type: BranchType, distance: i64) -> bool {
                // Placeholder implementation, replace with actual validity check
                true
            }

            fn IsCondBranchImm(&self) -> bool {
                self.BranchType() == BranchType::CondBranchType
            }

            fn IsUncondBranchImm(&self) -> bool {
                self.BranchType() == BranchType::UncondBranchType
            }

            fn IsCompareBranch(&self) -> bool {
                self.BranchType() == BranchType::CompareBranchType
            }

            fn IsTestBranch(&self) -> bool {
                self.BranchType() == BranchType::TestBranchType
            }
            pub fn SetImmPCOffsetTarget(&mut self, zone: &mut Zone, options: AssemblerOptions, target: *mut Instruction) {
                if self.IsPCRelAddressing() {
                    self.SetPCRelImmTarget(zone, options, target);
                } else if self.IsCondBranchImm() {
                    self.SetBranchImmTarget::<BranchType::CondBranchType>(target);
                } else if self.IsUncondBranchImm() {
                    self.SetBranchImmTarget::<BranchType::UncondBranchType>(target);
                } else if self.IsCompareBranch() {
                    self.SetBranchImmTarget::<BranchType::CompareBranchType>(target);
                } else if self.IsTestBranch() {
                    self.SetBranchImmTarget::<BranchType::TestBranchType>(target);
                } else if self.IsUnresolvedInternalReference() {
                    self.SetUnresolvedInternalReferenceImmTarget(zone, options, target);
                } else {
                    // Load literal (offset from PC).
                    self.SetImmLLiteral(target);
                }
            }

            fn SetBranchImmTarget<T: Copy>(&mut self, target: *mut Instruction) {
                // Placeholder implementation, replace with actual implementation
            }

            fn SetUnresolvedInternalReferenceImmTarget(&mut self, zone: &mut Zone, options: AssemblerOptions, target: *mut Instruction) {
                assert!(self.IsUnresolvedInternalReference());
                assert!(is_aligned(self.DistanceTo(target) as usize, KINSTR_SIZE));
                assert!(is_int32(self.DistanceTo(target) >> KINSTR_SIZE_LOG2 as i64));

                let target_offset = (self.DistanceTo(target) >> KINSTR_SIZE_LOG2 as i64) as i32;
                let high16 = unsigned_bitextract_32(31, 16, target_offset as u32);
                let low16 = unsigned_bitextract_32(15, 0, target_offset as u32);

                let mut patcher = PatchingAssembler::new(zone, options, self as *mut Self as *mut u8, 2);
                patcher.brk(high16);
                patcher.brk(low16);

            }
            fn SetImmLLiteral(&mut self, source: *mut Instruction) {
                assert!(self.IsLdrLiteral());
                assert!(is_aligned(self.DistanceTo(source) as usize, KINSTR_SIZE));
                assert!(Assembler::IsImmLLiteral(self.DistanceTo(source)));
                let imm = Assembler::ImmLLiteral((self.DistanceTo(source) >> KLOAD_LITERAL_SCALE_LOG2 as i64) as i32);
                let mask = IMM_LLITERAL_MASK;

                self.SetInstructionBits(self.Mask(!mask) | imm as Instr, None);

            }

            fn IsAdr(&self) -> bool {
                false // Placeholder
            }

            fn IsValidPCRelOffset(offset: i64) -> bool {
                true // Placeholder
            }

            fn SetPCRelImmTarget(&mut self, zone: &mut Zone, options: AssemblerOptions, target: *mut Instruction) {
                assert!(self.IsAdr());

                let target_offset = self.DistanceTo(target);
                let imm: Instr;

                if Instruction::IsValidPCRelOffset(target_offset) {
                    imm = Assembler::ImmPCRelAddress(target_offset as i32) as Instr;
                    self.SetInstructionBits(self.Mask(!IMM_PCREL_MASK) | imm, None);

                } else {
                    let mut patcher = PatchingAssembler::new(zone, options, self as *mut u8, PatchingAssembler::kAdrFarPatchableNInstrs);
                    patcher.PatchAdrFar(target_offset);
                }
            }
        }

        fn rotate_right(value: u64, rotate: u32, width: u32) -> u64 {
            assert!(width <= 64);
            let rotate = rotate & 63;
            if rotate == 0 {
                return value;
            }
            ((value & ((1u64 << rotate) - 1)) << (width - rotate)) | (value >> rotate)
        }

        fn repeat_bits_across_reg(reg_size: u32, value: u64, width: u32) -> u64 {
            assert!((width == 2) || (width == 4) || (width == 8) || (width == 16) || (width == 32));
            assert!((reg_size == KWREG_SIZE_IN_BITS) || (reg_size == KXREG_SIZE_IN_BITS));
            let mut result = value & ((1u64 << width) - 1);
            let mut i = width;
            while i < reg_size {
                result |= result << i;
                i *= 2;
            }
            result
        }

        fn imm8_to_fp32(imm8: u32) -> f32 {
            // Placeholder implementation
            0.0
        }

        fn imm8_to_fp64(imm8: u32) -> f64 {
            // Placeholder implementation
            0.0
        }

        fn is_aligned(value: usize, alignment: usize) -> bool {
            value & (alignment - 1) == 0
        }

        fn is_int32(value: i64) -> bool {
            value >= i32::MIN as i64 && value <= i32::MAX as i64
        }

        fn unsigned_bitextract_32(high: u32, low: u32, value: u32) -> u32 {
            (value >> low) & ((1 << (high - low + 1)) - 1)
        }
        // Placeholder struct for PatchingAssembler
        struct PatchingAssembler<'a> {
            zone: &'a mut Zone,
            options: AssemblerOptions,
            buffer: *mut u8,
            size: usize,
        }

        impl <'a> PatchingAssembler<'a> {
            const kAdrFarPatchableNInstrs: usize = 2;

            fn new(zone: &'a mut Zone, options: AssemblerOptions, buffer: *mut u8, size: usize) -> Self {
                PatchingAssembler {
                    zone,
                    options,
                    buffer,
                    size
                }
            }

            fn PatchAdrFar(&mut self, target_offset: i64) {
                // Placeholder implementation
            }

            fn brk(&mut self, value: u32) {
                // Placeholder implementation
            }
        }
        // Placeholder struct for Assembler
        struct Assembler {}

        impl Assembler {
            fn IsImmLLiteral(distance_to_source: i64) -> bool {
                true // Placeholder implementation
            }
            fn ImmLLiteral(offset: i32) -> i32 {
                0 // Placeholder
            }
            fn ImmPCRelAddress(offset: i32) -> i32 {
                0 // Placeholder
            }
        }

        //----------------------------------------------------------------------
        // NEONFormatDecoder
        //----------------------------------------------------------------------
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum VectorFormat {
            kFormatUndefined,
            kFormat8B,
            kFormat16B,
            kFormat4H,
            kFormat8H,
            kFormat2S,
            kFormat4S,
            kFormat1D,
            kFormat2D,
            kFormatB,
            kFormatH,
            kFormatS,
            kFormatD,
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum NEONFormat {
            NF_UNDEF,
            NF_8B,
            NF_16B,
            NF_4H,
            NF_8H,
            NF_2S,
            NF_4S,
            NF_1D,
            NF_2D,
            NF_B,
            NF_H,
            NF_S,
            NF_D,
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum SubstitutionMode {
            kPlaceholder,
            kFormat,
        }

        #[derive(Debug)]
        pub struct NEONFormatMap {
            map: [NEONFormat; 8], // Assuming max 8 entries based on PickBits logic
            bits: [[u8; 8]; 1],
        }

        impl NEONFormatMap {
            pub fn new(map: [NEONFormat; 8], bits: [[u8; 8]; 1]) -> Self {
                NEONFormatMap { map, bits }
            }
        }

        #[derive(Debug)]
        pub struct NEONFormatDecoder<'a> {
            instrbits_: Instr,
            formats_: [&'a NEONFormatMap; 4],
            form_buffer_: [u8; 256], // Example size, adjust as needed
            mne_buffer_: [u8; 64], // Example size, adjust as needed
        }

        const NEON_Q: Instr = 0x00000000; // Placeholder for the NEON_Q bit
        const KNEONFORMATMAXBITS: usize = 8;
        impl<'a> NEONFormatDecoder<'a> {
            pub fn new(instr: &Instruction, format: Option<&'a NEONFormatMap>, format1: Option<&'a NEONFormatMap>, format2: Option<&'a NEONFormatMap>) -> Self {
                let mut decoder = NEONFormatDecoder {
                    instrbits_: instr.InstructionBits(),
                    formats_: [&NEONFormatMap { map: [NEONFormat::NF_UNDEF; 8], bits: [[0u8; 8]; 1] }; 4],
                    form_buffer_: [0u8; 256],
                    mne_buffer_: [0u8; 64],
                };

                if let Some(f) = format {
                    if let Some(f1) = format1{
                         if let Some(f2) = format2{
                                 decoder.SetFormatMaps(f, f1, f2);
                            } else {
                                 decoder.SetFormatMaps(f, f1, f1);
                            }
                    } else {
                        decoder.SetFormatMaps(f, f, f);
                    }

                }
                else {
                    decoder.SetFormatMaps(&NEONFormatMap { map: [NEONFormat::NF_UNDEF; 8], bits: [[0u8; 8]; 1] },&NEONFormatMap { map: [NEONFormat::NF_UNDEF; 8], bits: [[0u8; 8]; 1] },&NEONFormatMap { map: [NEONFormat::NF_UNDEF; 8], bits: [[0u8; 8]; 1] });
                }

                decoder
            }

            pub fn new_with_instr(instr: &Instruction) -> Self {
                let mut decoder = NEONFormatDecoder {
                    instrbits_: instr.InstructionBits(),
                    formats_: [&NEONFormatMap { map: [NEONFormat::NF_UNDEF; 8], bits: [[0u8; 8]; 1] }; 4],
                    form_buffer_: [0u8; 256],
                    mne_buffer_: [0u8; 64],
                };
                decoder.SetFormatMaps(decoder.IntegerFormatMap(), decoder.IntegerFormatMap(),decoder.IntegerFormatMap());
                decoder
            }

            fn IntegerFormatMap<'b>(&self) -> &'b NEONFormatMap {
                 &NEONFormatMap { map: [NEONFormat::NF_UNDEF; 8], bits: [[0u8; 8]; 1] }
            }

            fn SetFormatMaps(&mut self, format0: &'a NEONFormatMap, format1: &'a NEONFormatMap, format2: &'a NEONFormatMap) {
                self.formats_[0] = format0;
                self.formats_[1] = if format1 as *const _ == format0 as *const _ {
                    self.formats_[0]
                } else {
                    format1
                };
                self.formats_[2] = if format2 as *const _ == format1 as *const _ {
                    self.formats_[1]
                } else {
                    format2
                };
                self.formats_[3] = self.formats_[2];
            }

            fn SetFormatMap(&mut self, index: usize, format: &'a NEONFormatMap) {
                assert!(index < self.formats_.len());
                self.formats_[index] = format;
            }

            pub fn SubstitutePlaceholders(&self, string: &str) -> String {
                self.Substitute(
                    string,
                    SubstitutionMode::kPlaceholder,
                    SubstitutionMode::kPlaceholder,
                    SubstitutionMode::kPlaceholder,
                    SubstitutionMode::kPlaceholder,
                )
            }

            pub fn Substitute(
                &self,
                string: &str,
                mode0: SubstitutionMode,
                mode1: SubstitutionMode,
                mode2: SubstitutionMode,
                mode3: SubstitutionMode,
            ) -> String {
                let arg0 = self.GetSubstitute(0, mode0);
                let arg1 = self.GetSubstitute(1, mode1);
                let arg2 = self.GetSubstitute(2, mode2);
                let arg3 = self.GetSubstitute(3, mode3);

                format!(string, arg0 = arg0, arg1 = arg1, arg2 = arg2, arg3 = arg3)
            }

            pub fn Mnemonic(&self, mnemonic: &str) -> String {
                if (self.instrbits_ & NEON_Q) != 0 {
                    format!("{}2", mnemonic)
                } else {
                    mnemonic.to_string()
                }
            }

            pub fn GetVectorFormat(&self, format_index: usize) -> VectorFormat {
                self.GetVectorFormatFromMap(self.formats_[format_index])
            }

            fn GetVectorFormatFromMap(&self, format_map: &'a NEONFormatMap) -> VectorFormat {
                let vform: [VectorFormat; 13] = [
                    VectorFormat::kFormatUndefined,
                    VectorFormat::kFormat8B,
                    VectorFormat::kFormat16B,
                    VectorFormat::kFormat4H,
                    VectorFormat::kFormat8H,
                    VectorFormat::kFormat2S,
                    VectorFormat::kFormat4S,
                    VectorFormat::kFormat1D,
                    VectorFormat::kFormat2D,
                    VectorFormat::kFormatB,
                    VectorFormat::kFormatH,
                    VectorFormat::kFormatS,
                    VectorFormat::kFormatD,
                ];
                let neon_format = self.GetNEONFormat(format_map);
                assert!(neon_format as usize < vform.len());
                vform[neon_format as usize]
            }

            fn GetSubstitute(&self, index: usize, mode: SubstitutionMode) -> &'static str {
                match mode {
                    SubstitutionMode::kFormat => self.NEONFormatAsString(self.GetNEONFormat(self.formats_[index])),
                    SubstitutionMode::kPlaceholder => self.NEONFormatAsPlaceholder(self.GetNEONFormat(self.formats_[index])),
                }
            }

            fn GetNEONFormat(&self, format_map: &'a NEONFormatMap) -> NEONFormat {
                format_map.map[self.PickBits(format_map.bits[0].as_ref()) as usize]
            }

            fn NEONFormatAsString(&self, format: NEONFormat) -> &'static str {
                let formats: [&str; 13] = [
                    "undefined", "8b", "16b", "4h", "8h", "2s", "4s", "1d", "2d", "b", "h", "s", "d",
                ];
                assert!(format as usize < formats.len());
                formats[format as usize]
            }

            fn NEONFormatAsPlaceholder(&self, format: NEONFormat) -> &'static str {
                assert!(
                    format == NEONFormat::NF_B
                        || format == NEONFormat::NF_H
                        || format == NEONFormat::NF_S
                        || format == NEONFormat::NF_D
                        || format == NEONFormat::NF_UNDEF
                );
                let formats: [&str; 13] = [
                    "undefined",
                    "undefined",
                    "undefined",
                    "undefined",
                    "undefined",
                    "undefined",
                    "undefined",
                    "undefined",
                    "undefined",
                    "'B",
                    "'H",
                    "'S",
                    "'D",
                ];
                formats[format as usize]
            }

            fn PickBits(&self, bits: &[u8]) -> u8 {
                let mut result: u8 = 0;
                for b in 0..KNEONFORMATMAXBITS {
                    if bits[b] == 0 {
                        break;
                    }
                    result <<= 1;
                    result |= if (self.instrbits_ & (1 << bits[b])) == 0 { 0 } else { 1 };
                }
                result
            }
        }
    }
}