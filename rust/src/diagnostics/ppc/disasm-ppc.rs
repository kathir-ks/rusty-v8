// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// A Disassembler object is used to disassemble a block of code instruction by
// instruction. The default implementation of the NameConverter object can be
// overriden to modify register names or to do symbol lookup on addresses.
//
// The example below will disassemble a block of code and print it to stdout.
//
//   NameConverter converter;
//   Disassembler d(converter);
//   for (uint8_t* pc = begin; pc < end;) {
//     v8::base::EmbeddedVector<char, 256> buffer;
//     uint8_t* prev_pc = pc;
//     pc += d.InstructionDecode(buffer, pc);
//     printf("%p    %08x      %s\n",
//            prev_pc, *reinterpret_cast<int32_t*>(prev_pc), buffer);
//   }
//
// The Disassembler class also has a convenience method to disassemble a block
// of code into a FILE*, meaning that the above functionality could also be
// achieved by just calling Disassembler::Disassemble(stdout, begin, end);

#[cfg(target_arch = "powerpc64")]
mod ppc64_disasm {
    use std::fmt;
    use std::mem;
    use std::ptr;
    use std::slice;
    use std::str;
    use libc::{c_char, FILE};

    //use crate::base::platform::platform::*; // Assuming a corresponding Rust module
    //use crate::base::strings::*; // Assuming a corresponding Rust module
    //use crate::base::vector::*; // Assuming a corresponding Rust module
    //use crate::codegen::macro_assembler::*; // Assuming a corresponding Rust module
    //use crate::codegen::ppc::constants_ppc::*; // Assuming a corresponding Rust module
    //use crate::codegen::register_configuration::*; // Assuming a corresponding Rust module
    //use crate::diagnostics::disasm::*; // Assuming a corresponding Rust module

    const K_INSTR_SIZE: usize = 4; // Size of an instruction in bytes

    // Placeholder types and constants. Need actual definitions.
    type Instruction = u32;
    type SoftwareInterruptCodes = i32;
    const kCallRtRedirected: SoftwareInterruptCodes = 1;
    const kBreakpoint: SoftwareInterruptCodes = 2;
    const kStopCode: SoftwareInterruptCodes = 3;
    const kStopCodeMask: SoftwareInterruptCodes = 0x3F;
    type CRBit = u32;
    const CRWIDTH: u32 = 4;
    const CR_EQ: CRBit = 0;
    const CR_GT: CRBit = 1;
    const CR_LT: CRBit = 2;
    const CR_SO: CRBit = 3;

    const DCBNZF: u32 = 0;
    const DCBEZF: u32 = 1;
    const BF: u32 = 2;
    const DCBNZT: u32 = 3;
    const DCBEZT: u32 = 4;
    const BT: u32 = 5;
    const DCBNZ: u32 = 6;
    const DCBEZ: u32 = 7;
    const BA: u32 = 8;

    const EXTP: u32 = 0;
    const EXT0: u32 = 1;
    const EXT1: u32 = 2;
    const EXT2: u32 = 3;
    const EXT3: u32 = 4;
    const EXT4: u32 = 5;
    const EXT5: u32 = 6;
    const EXT6: u32 = 7;

    const PLOAD_STORE_8LS: u32 = 0;
    const PLOAD_STORE_MLS: u32 = 1;
    const ADDI: u32 = 2;
    const LBZ: u32 = 3;
    const LHZ: u32 = 4;
    const LHA: u32 = 5;
    const LWZ: u32 = 6;
    const PPLWA: u32 = 7;
    const PPLD: u32 = 8;
    const LFS: u32 = 9;
    const LFD: u32 = 10;
    const STB: u32 = 11;
    const STH: u32 = 12;
    const STW: u32 = 13;
    const PPSTD: u32 = 14;
    const STFS: u32 = 15;
    const STFD: u32 = 16;

    const ABI_USES_FUNCTION_DESCRIPTORS: bool = false; // Placeholder

    const SRADIX: u32 = 0;

    // Placeholder PPC_VX opcodes
    macro_rules! define_ppc_opcodes {
        ($($name:ident = $value:expr,)*) => {
            $(const $name: u32 = $value;)*
        }
    }

    define_ppc_opcodes! {
        MCRF = 100,
        BCLRX = 101,
        BCCTRX = 102,
        CRNOR = 103,
        RFI = 104,
        CRANDC = 105,
        ISYNC = 106,
        CRXOR = 107,
        CRNAND = 108,
        CRAND = 109,
        CREQV = 110,
        CRORC = 111,
        CROR = 112,
        LVX = 113,
        STVX = 114,
        LXVD = 115,
        LXVX = 116,
        LXSDX = 117,
        LXSIBZX = 118,
        LXSIHZX = 119,
        LXSIWZX = 120,
        STXVD = 121,
        STXVX = 122,
        STXSDX = 123,
        STXSIBX = 124,
        STXSIHX = 125,
        STXSIWX = 126,
        SRWX = 127,
        SRDX = 128,
        SRAW = 129,
        SRAD = 130,
        SYNC = 131,
        MODSW = 132,
        MODUW = 133,
        MODSD = 134,
        MODUD = 135,
        SRAWIX = 136,
        EXTSH = 137,
        EXTSW = 138,
        EXTSB = 139,
        LFSX = 140,
        LFSUX = 141,
        LFDX = 142,
        LFDUX = 143,
        STFSX = 144,
        STFSUX = 145,
        STFDX = 146,
        STFDUX = 147,
        POPCNTW = 148,
        POPCNTD = 149,
        STBCX = 150,
        STHCX = 151,
        STWCX = 152,
        STDCX = 153,
        CMP = 154,
        SLWX = 155,
        SLDX = 156,
        SUBFCX = 157,
        SUBFEX = 158,
        ADDCX = 159,
        ADDEX = 160,
        CNTLZWX = 161,
        CNTLZDX = 162,
        CNTTZWX = 163,
        CNTTZDX = 164,
        BRH = 165,
        BRW = 166,
        BRD = 167,
        ANDX = 168,
        ANDCX = 169,
        CMPL = 170,
        NEGX = 171,
        NORX = 172,
        SUBFX = 173,
        MULHWX = 174,
        ADDZEX = 175,
        MULLW = 176,
        MULLD = 177,
        DIVW = 178,
        DIVWU = 179,
        DIVD = 180,
        ADDX = 181,
        XORX = 182,
        ORX = 183,
        MFSPR = 184,
        MTSPR = 185,
        MFCR = 186,
        STWX = 187,
        STWUX = 188,
        STBX = 189,
        STBUX = 190,
        STHX = 191,
        STHUX = 192,
        LWZX = 193,
        LWZUX = 194,
        LWAX = 195,
        LBZX = 196,
        LBZUX = 197,
        LHZX = 198,
        LHZUX = 199,
        LHAX = 200,
        LBARX = 201,
        LHARX = 202,
        LWARX = 203,
        LDX = 204,
        LDUX = 205,
        LDARX = 206,
        STDX = 207,
        STDUX = 208,
        MFVSRD = 209,
        MFVSRWZ = 210,
        MTVSRD = 211,
        MTVSRWA = 212,
        MTVSRWZ = 213,
        MTVSRDD = 214,
        LDBRX = 215,
        LHBRX = 216,
        LWBRX = 217,
        STDBRX = 218,
        STWBRX = 219,
        STHBRX = 220,
        MTCRF = 221,
        ISEL = 222,
        FDIV = 223,
        FSUB = 224,
        FADD = 225,
        FSQRT = 226,
        FSEL = 227,
        FMUL = 228,
        FMSUB = 229,
        FMADD = 230,
        FCMPU = 231,
        FRSP = 232,
        FCTID = 233,
        FCTIDZ = 234,
        FCTIDU = 235,
        FCTIDUZ = 236,
        FCTIW = 237,
        FCTIWZ = 238,
        FCTIWUZ = 239,
        FMR = 240,
        MTFSFI = 241,
        MFFS = 242,
        MTFSF = 243,
        FABS = 244,
        FRIN = 245,
        FRIZ = 246,
        FRIP = 247,
        FRIM = 248,
        FNEG = 249,
        FCPSGN = 250,
        MCRFS = 251,
        MTFSB0 = 252,
        MTFSB1 = 253,
        RLDICL = 254,
        RLDICR = 255,
        RLDIC = 256,
        RLDIMI = 257,
        RLDCL = 258,
        XXSPLTIB = 259,
        TWI = 260,
        MULLI = 261,
        SUBFIC = 262,
        CMPLI = 263,
        CMPI = 264,
        ADDIC = 265,
        ADDICx = 266,
        ADDI_OP = 267,
        ADDIS = 268,
        BCX = 269,
        SC = 270,
        BX = 271,
        RLWIMIX = 272,
        RLWINMX = 273,
        RLWNMX = 274,
        ORI = 275,
        ORIS = 276,
        XORI = 277,
        XORIS = 278,
        ANDIx = 279,
        ANDISx = 280,
        LWZ_OP = 281,
        LWZU_OP = 282,
        LBZ_OP = 283,
        LBZU_OP = 284,
        STW_OP = 285,
        STWU_OP = 286,
        STB_OP = 287,
        STBU_OP = 288,
        LHZ_OP = 289,
        LHZU_OP = 290,
        LHA_OP = 291,
        LHAU_OP = 292,
        STH_OP = 293,
        STHU_OP = 294,
        LMW = 295,
        STMW = 296,
        LFS_OP = 297,
        LFSU_OP = 298,
        LFD_OP = 299,
        LFDU_OP = 300,
        STFS_OP = 301,
        STFSU_OP = 302,
        STFD_OP = 303,
        STFDU_OP = 304,
        LD_OP = 305,
        STD_OP = 306,
    }

    const BT_SHIFT: u32 = 21;
    const BI_SHIFT: u32 = 16;
    const IMM_SHIFT: u32 = 0;

    macro_rules! SIGN_EXT_IMM34 {
        ($value:expr) => {
            (($value as i64) << (64 - 34)) >> (64 - 34)
        }
    }

    macro_rules! SIGN_EXT_IMM5 {
        ($value:expr) => {
            (($value as i32) << (32 - 5)) >> (32 - 5)
        }
    }

    macro_rules! SIGN_EXT_IMM16 {
        ($value:expr) => {
            (($value as i32) << (32 - 16)) >> (32 - 16)
        }
    }

    //------------------------------------------------------------------------------

    /// Decoder decodes and disassembles instructions into an output buffer.
    /// It uses the converter to convert register names and call destinations into
    /// more informative description.
    pub struct Decoder<'a> {
        converter: &'a dyn NameConverter,
        out_buffer: Vec<u8>,
        out_buffer_pos: usize,
        prefix_status: PrefixType,
        prefix_value: u64,
    }

    impl<'a> Decoder<'a> {
        pub fn new(converter: &'a dyn NameConverter, buffer_size: usize) -> Self {
            Decoder {
                converter,
                out_buffer: vec![0u8; buffer_size],
                out_buffer_pos: 0,
                prefix_status: PrefixType::NotPrefixed,
                prefix_value: 0,
            }
        }

        /// Writes one disassembled instruction into 'buffer' (0-terminated).
        /// Returns the length of the disassembled machine instruction in bytes.
        pub fn instruction_decode(&mut self, instruction: *mut u8) -> i32 {
            let instruction_bits: u32 = unsafe { *(instruction as *mut u32) };
            let instr = InstructionWrapper { instruction_bits, instruction };

            self.decode_instruction(&instr) as i32
        }

        fn decode_instruction(&mut self, instr: &InstructionWrapper) -> usize {
            let opcode = (instr.instruction_bits >> 26) & 0x3F;

            // Print raw instruction bytes.
            if opcode != EXTP {
                self.out_buffer_pos += self.format(&format!("{:08x}       ", instr.instruction_bits));
            } else {
                // Prefixed instructions have a 4-byte prefix and a 4-byte suffix. Print
                // both on the same line.
                let next_instr_ptr = unsafe { instr.instruction.add(K_INSTR_SIZE) };
                let next_instr_bits = unsafe { *(next_instr_ptr as *mut u32) };
                self.out_buffer_pos += self.format(&format!("{:08x}|{:08x} ", instr.instruction_bits, next_instr_bits));
            }

            if ABI_USES_FUNCTION_DESCRIPTORS && instr.instruction_bits == 0 {
                // The first field will be identified as a jump table entry.  We
                // emit the rest of the structure as zero, so just skip past them.
                self.format("constant");
                return K_INSTR_SIZE;
            }

            match opcode {
                _ => {
                    match opcode {
                        TWI => {
                            self.print_software_interrupt(self.get_svc_value(instr));
                        }
                        MULLI => {
                            self.unknown_format(instr, "mulli");
                        }
                        SUBFIC => {
                            self.format("subfic  'rt, 'ra, 'int16");
                        }
                        CMPLI => {
                            if (instr.instruction_bits >> 21) & 1 == 1 {
                                self.format("cmpli   'ra, 'uint16");
                            } else {
                                self.format("cmplwi  'ra, 'uint16");
                            }
                        }
                        CMPI => {
                            if (instr.instruction_bits >> 21) & 1 == 1 {
                                self.format("cmpi    'ra, 'int16");
                            } else {
                                self.format("cmpwi   'ra, 'int16");
                            }
                        }
                        ADDIC => {
                            self.format("addic   'rt, 'ra, 'int16");
                        }
                        ADDICx => {
                            self.unknown_format(instr, "addicx");
                        }
                        ADDI_OP => {
                            if self.get_ra_value(instr) == 0 {
                                // this is load immediate
                                self.format("li      'rt, 'int16");
                            } else {
                                self.format("addi    'rt, 'ra, 'int16");
                            }
                        }
                        ADDIS => {
                            if self.get_ra_value(instr) == 0 {
                                self.format("lis     'rt, 'int16");
                            } else {
                                self.format("addis   'rt, 'ra, 'int16");
                            }
                        }
                        BCX => {
                            let bo = (instr.instruction_bits >> 21) & 0x1F;
                            let bi = (instr.instruction_bits >> 16) & 0x1F;
                            let cond = bi; //static_cast<CRBit>(bi & (CRWIDTH - 1));
                            match bo {
                                _ => {
                                    match bo {
                                        5 => {  // Branch if condition true
                                            match cond {
                                                _ => {}
                                            }
                                        }
                                        2 => {  // Branch if condition false
                                            match cond {
                                                _ => {}
                                            }
                                        }
                                        6 => {  // Decrement CTR; branch if CTR != 0
                                            self.format("bdnz'l'a 'target16");
                                        }
                                        _ => {
                                            self.format("bc'l'a'cr 'target16");
                                        }
                                    }
                                }
                            }
                        }
                        SC => {
                            self.unknown_format(instr, "sc");
                        }
                        BX => {
                            self.format("b'l'a 'target26");
                        }
                        EXTP => {
                            self.decode_extp(instr);
                        }
                        EXT0 => {
                            self.decode_ext0(instr);
                        }
                        EXT1 => {
                            self.decode_ext1(instr);
                        }
                        RLWIMIX => {
                            self.format("rlwimi'. 'ra, 'rs, 'sh, 'me, 'mb");
                        }
                        RLWINMX => {
                            self.format("rlwinm'. 'ra, 'rs, 'sh, 'me, 'mb");
                        }
                        RLWNMX => {
                            self.format("rlwnm'.  'ra, 'rs, 'rb, 'me, 'mb");
                        }
                        ORI => {
                            self.format("ori     'ra, 'rs, 'uint16");
                        }
                        ORIS => {
                            self.format("oris    'ra, 'rs, 'uint16");
                        }
                        XORI => {
                            self.format("xori    'ra, 'rs, 'uint16");
                        }
                        XORIS => {
                            self.format("xoris   'ra, 'rs, 'uint16");
                        }
                        ANDIx => {
                            self.format("andi.   'ra, 'rs, 'uint16");
                        }
                        ANDISx => {
                            self.format("andis.  'ra, 'rs, 'uint16");
                        }
                        EXT2 => {
                            self.decode_ext2(instr);
                        }
                        LWZ_OP => {
                            self.format("lwz     'rt, 'int16('ra)");
                        }
                        LWZU_OP => {
                            self.format("lwzu    'rt, 'int16('ra)");
                        }
                        LBZ_OP => {
                            self.format("lbz     'rt, 'int16('ra)");
                        }
                        LBZU_OP => {
                            self.format("lbzu    'rt, 'int16('ra)");
                        }
                        STW_OP => {
                            self.format("stw     'rs, 'int16('ra)");
                        }
                        STWU_OP => {
                            self.format("stwu    'rs, 'int16('ra)");
                        }
                        STB_OP => {
                            self.format("stb     'rs, 'int16('ra)");
                        }
                        STBU_OP => {
                            self.format("stbu    'rs, 'int16('ra)");
                        }
                        LHZ_OP => {
                            self.format("lhz     'rt, 'int16('ra)");
                        }
                        LHZU_OP => {
                            self.format("lhzu    'rt, 'int16('ra)");
                        }
                        LHA_OP => {
                            self.format("lha     'rt, 'int16('ra)");
                        }
                        LHAU_OP => {
                            self.format("lhau    'rt, 'int16('ra)");
                        }
                        STH_OP => {
                            self.format("sth 'rs, 'int16('ra)");
                        }
                        STHU_OP => {
                            self.format("sthu 'rs, 'int16('ra)");
                        }
                        LMW => {
                            self.unknown_format(instr, "lmw");
                        }
                        STMW => {
                            self.unknown_format(instr, "stmw");
                        }
                        LFS_OP => {
                            self.format("lfs     'Dt, 'int16('ra)");
                        }
                        LFSU_OP => {
                            self.format("lfsu    'Dt, 'int16('ra)");
                        }
                        LFD_OP => {
                            self.format("lfd     'Dt, 'int16('ra)");
                        }
                        LFDU_OP => {
                            self.format("lfdu    'Dt, 'int16('ra)");
                        }
                        STFS_OP => {
                            self.format("stfs    'Dt, 'int16('ra)");
                        }
                        STFSU_OP => {
                            self.format("stfsu   'Dt, 'int16('ra)");
                        }
                        STFD_OP => {
                            self.format("stfd    'Dt, 'int16('ra)");
                        }
                        STFDU_OP => {
                            self.format("stfdu   'Dt, 'int16('ra)");
                        }
                        EXT3 => {
                            self.decode_ext3(instr);
                        }
                        EXT4 => {
                            self.decode_ext4(instr);
                        }
                        EXT5 => {
                            self.decode_ext5(instr);
                        }
                        EXT6 => {
                            self.decode_ext6(instr);
                        }
                        LD_OP => {
                            match instr.instruction_bits & 0x3 {
                                0 => self.format("ld      'rt, 'd('ra)"),
                                1 => self.format("ldu     'rt, 'd('ra)"),
                                2 => self.format("lwa     'rt, 'd('ra)"),
                                _ => {},
                            }
                        }
                        STD_OP => {  // could be STD or STDU
                            if (instr.instruction_bits & 0x1) == 0 {
                                self.format("std     'rs, 'd('ra)");
                            } else {
                                self.format("stdu    'rs, 'd('ra)");
                            }
                        }
                        _ => {
                            self.unknown(instr);
                        }
                    }
                }
            }

            if self.is_prefixed() {
                // The next instruction (suffix) should have already been decoded as part of
                // prefix decoding.
                self.reset_prefix();
                return 2 * K_INSTR_SIZE;
            }

            return K_INSTR_SIZE;
        }

        fn get_svc_value(&self, instr: &InstructionWrapper) -> SoftwareInterruptCodes {
            // Placeholder implementation.  Needs to actually extract the SVC value.
            1 // Dummy Value
        }

        fn get_ra_value(&self, instr: &InstructionWrapper) -> i32 {
            // Placeholder implementation.  Needs to actually extract the RA register value.
            0 // Dummy value
        }

        fn decode_extp(&mut self, instr: &InstructionWrapper) {
            match EXTP | ((instr.instruction_bits >> 25) & 0x1) as u32 {
                PLOAD_STORE_8LS | PLOAD_STORE_MLS => {
                    // TODO(miladfarca): Decode the R bit.
                    //DCHECK_NE(instr->Bit(20), 1);
                    if (instr.instruction_bits >> 20) & 0x1 == 1 {
                        //Todo: crash or return error.
                    }
                    // Read prefix.
                    self.set_as_prefixed(((instr.instruction_bits >> 0) & ((1 << 18) - 1)) as u64);
                    // Read suffix (next instruction).
                    let next_instr_ptr = unsafe { instr.instruction.add(K_INSTR_SIZE) };
                    let next_instr = InstructionWrapper {
                        instruction_bits: unsafe { *(next_instr_ptr as *mut u32) },
                        instruction: next_instr_ptr,
                    };

                    match (next_instr.instruction_bits >> 26) & 0x3F {
                        // Prefixed ADDI.
                        ADDI => {
                            if self.get_ra_value(&next_instr) == 0 {
                                // This is load immediate prefixed.
                                self.format("pli");
                                self.format("     'rt, ");
                            } else {
                                self.format("paddi");
                                self.format("   'rt, 'ra, ");
                            }
                            self.format("'int34");
                        }
                        // Prefixed LBZ.
                        LBZ => {
                            self.format("plbz    'rt, 'int34('ra)");
                        }
                        // Prefixed LHZ.
                        LHZ => {
                            self.format("plhz    'rt, 'int34('ra)");
                        }
                        // Prefixed LHA.
                        LHA => {
                            self.format("plha    'rt, 'int34('ra)");
                        }
                        // Prefixed LWZ.
                        LWZ => {
                            self.format("plwz    'rt, 'int34('ra)");
                        }
                        // Prefixed LWA.
                        PPLWA => {
                            self.format("plwa    'rt, 'int34('ra)");
                        }
                        // Prefixed LD.
                        PPLD => {
                            self.format("pld     'rt, 'int34('ra)");
                        }
                        // Prefixed LFS.
                        LFS => {
                            self.format("plfs    'Dt, 'int34('ra)");
                        }
                        // Prefixed LFD.
                        LFD => {
                            self.format("plfd    'Dt, 'int34('ra)");
                        }
                        // Prefixed STB.
                        STB => {
                            self.format("pstb    'rs, 'int34('ra)");
                        }
                        // Prefixed STH.
                        STH => {
                            self.format("psth    'rs, 'int34('ra)");
                        }
                        // Prefixed STW.
                        STW => {
                            self.format("pstw    'rs, 'int34('ra)");
                        }
                        // Prefixed STD.
                        PPSTD => {
                            self.format("pstd    'rs, 'int34('ra)");
                        }
                        // Prefixed STFS.
                        STFS => {
                            self.format("pstfs   'Dt, 'int34('ra)");
                        }
                        // Prefixed STFD.
                        STFD => {
                            self.format("pstfd   'Dt, 'int34('ra)");
                        }
                        _ => {
                            self.unknown(&next_instr);
                        }
                    }
                }
                _ => {
                    self.unknown(instr);
                }
            }
        }

        fn decode_ext0(&mut self, instr: &InstructionWrapper) {
            // Some encodings have integers hard coded in the middle, handle those first.
            match EXT0 | ((instr.instruction_bits >> 16) & 0x1F) | ((instr.instruction_bits >> 0) & ((1 << 11) - 1)) as u32 {
                _ => {}
            }
            // Some encodings are 5-0 bits, handle those first
            match EXT0 | ((instr.instruction_bits >> 0) & ((1 << 6) - 1)) as u32 {
                _ => {}
            }
            match EXT0 | ((instr.instruction_bits >> 0) & ((1 << 10) - 1)) as u32 {
                _ => {}
            }
            match EXT0 | ((instr.instruction_bits >> 0) & ((1 << 11) - 1)) as u32 {
                _ => {}
            }
        }

        fn decode_ext1(&mut self, instr: &InstructionWrapper) {
            match EXT1 | ((instr.instruction_bits >> 1) & ((1 << 10) - 1)) as u32 {
                MCRF => {
                    self.unknown_format(instr, "mcrf");  // not used by V8
                }
                BCLRX => {
                    let bo = (instr.instruction_bits >> 21) & 0x1F;
                    let bi = (instr.instruction_bits >> 16) & 0x1F;
                    //CRBit cond = static_cast<CRBit>(bi & (CRWIDTH - 1));
                    let cond = bi;
                    match bo {
                        DCBNZF => {
                            self.unknown_format(instr, "bclrx-dcbnzf");
                        }
                        DCBEZF => {
                            self.unknown_format(instr, "bclrx-dcbezf");
                        }
                        BF => {
                            match cond {
                                CR_EQ => {
                                    self.format("bnelr'l'cr");
                                }
                                CR_GT => {
                                    self.format("blelr'l'cr");
                                }
                                CR_LT => {
                                    self.format("bgelr'l'cr");
                                }
                                CR_SO => {
                                    self.format("bnsolr'l'cr");
                                }
                                _ => {},
                            }
                        }
                        DCBNZT => {
                            self.unknown_format(instr, "bclrx-dcbbzt");
                        }
                        DCBEZT => {
                            self.unknown_format(instr, "bclrx-dcbnezt");
                        }
                        BT => {
                            match cond {
                                CR_EQ => {
                                    self.format("beqlr'l'cr");
                                }
                                CR_GT => {
                                    self.format("bgtlr'l'cr");
                                }
                                CR_LT => {
                                    