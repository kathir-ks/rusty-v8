// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::fmt;
use std::mem;

// Get the standard printf format macros for C99 stdint types.
// Included implicitly by inttypes.h

mod base {
    pub mod logging {
        #[macro_export]
        macro_rules! CHECK {
            ($x:expr) => {
                if !$x {
                    panic!("Check failed: {}", stringify!($x));
                }
            };
        }
    }

    pub mod macros {
        #[macro_export]
        macro_rules! UNREACHABLE {
            () => {
                panic!("Unreachable code reached!");
            };
        }
    }
}

mod common {
    pub mod code_memory_access {}
    pub mod globals {
        pub const kMaxInt: i64 = i64::MAX;
    }
}

#[cfg(debug_assertions)]
macro_rules! UNIMPLEMENTED_S390 {
    () => {
        println!(
            "{}, \tline {}: \tfunction {} not implemented. \n",
            file!(),
            line!(),
            function!()
        );
    };
}

#[cfg(not(debug_assertions))]
macro_rules! UNIMPLEMENTED_S390 {
    () => {};
}

#[cfg(target_os = "zos")]
const ABI_USES_FUNCTION_DESCRIPTORS: i32 = 1;
#[cfg(target_os = "zos")]
const ABI_PASSES_HANDLES_IN_REGS: i32 = 1;
#[cfg(target_os = "zos")]
const ABI_RETURNS_OBJECTPAIR_IN_REGS: i32 = 1;

#[cfg(not(target_os = "zos"))]
const ABI_USES_FUNCTION_DESCRIPTORS: i32 = 0;
#[cfg(not(target_os = "zos"))]
const ABI_PASSES_HANDLES_IN_REGS: i32 = 1;
#[cfg(not(target_os = "zos"))]
const ABI_RETURNS_OBJECTPAIR_IN_REGS: i32 = 0;

const ABI_CALL_VIA_IP: i32 = 1;

pub mod internal {

    // The maximum size of the code range s.t. pc-relative calls are possible
    // between all Code objects in the range.
    pub const kMaxPCRelativeCodeRangeInMB: usize = 4096;

    #[cfg(target_os = "zos")]
    pub const kHasFunctionDescriptorBitShift: i32 = 4;
    #[cfg(target_os = "zos")]
    pub const kHasFunctionDescriptorBitMask: i32 = 1 << kHasFunctionDescriptorBitShift;

    // Number of registers
    pub const kNumRegisters: i32 = 16;

    // FP support.
    pub const kNumDoubleRegisters: i32 = 16;

    pub const kNoRegister: i32 = -1;

    // The actual value of the kRootRegister is offset from the IsolateData's start
    // to take advantage of negative displacement values.
    pub const kRootRegisterBias: i32 = 128;

    // sign-extend the least significant 16-bits of value <imm>
    macro_rules! SIGN_EXT_IMM16 {
        ($imm:expr) => {
            (($imm as i32) << 16) >> 16
        };
    }
    pub(crate) use SIGN_EXT_IMM16;

    // sign-extend the least significant 26-bits of value <imm>
    macro_rules! SIGN_EXT_IMM26 {
        ($imm:expr) => {
            (($imm as i32) << 6) >> 6
        };
    }
    pub(crate) use SIGN_EXT_IMM26;

    // -----------------------------------------------------------------------------
    // Conditions.

    // Defines constants and accessor classes to assemble, disassemble and
    // simulate z/Architecture instructions.
    //
    // Section references in the code refer to the "z/Architecture Principles
    // Of Operation" http://publibfi.boulder.ibm.com/epubs/pdf/dz9zr009.pdf
    //

    // Constants for specific fields are defined in their respective named enums.
    // General constants are in an anonymous enum in class Instr.
    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum Condition {
        kNoCondition = -1,
        eq = 0x8,  // Equal.
        ne = 0x7,  // Not equal.
        ge = 0xa,  // Greater or equal.
        lt = 0x4,  // Less than.
        gt = 0x2,  // Greater than.
        le = 0xc,  // Less then or equal
        al = 0xf,  // Always.

        CC_NOP = 0x0,           // S390 NOP
        CC_EQ = 0x08,           // S390 condition code 0b1000
        CC_LT = 0x04,           // S390 condition code 0b0100
        CC_LE = CC_EQ | CC_LT,  // S390 condition code 0b1100
        CC_GT = 0x02,           // S390 condition code 0b0010
        CC_GE = CC_EQ | CC_GT,  // S390 condition code 0b1010
        CC_OF = 0x01,           // S390 condition code 0b0001
        CC_NOF = 0x0E,          // S390 condition code 0b1110
        CC_ALWAYS = 0x0F,       // S390 always taken branch
        unordered = CC_OF,      // Floating-point unordered
        ordered = CC_NOF,       // floating-point ordered
        overflow = CC_OF,       // Summary overflow
        nooverflow = CC_NOF,

        mask0x0 = 0,  // no jumps
        mask0x1 = 1,
        mask0x2 = 2,
        mask0x3 = 3,
        mask0x4 = 4,
        mask0x5 = 5,
        mask0x6 = 6,
        mask0x7 = 7,
        mask0x8 = 8,
        mask0x9 = 9,
        mask0xA = 10,
        mask0xB = 11,
        mask0xC = 12,
        mask0xD = 13,
        mask0xE = 14,
        mask0xF = 15,

        // Unified cross-platform condition names/aliases.
        // Do not set unsigned constants equal to their signed variants.
        // We need to be able to differentiate between signed and unsigned enum
        // constants in order to emit the right instructions (i.e CmpS64 vs CmpU64).
        kEqual = eq,
        kNotEqual = ne,
        kLessThan = lt,
        kGreaterThan = gt,
        kLessThanEqual = le,
        kGreaterThanEqual = ge,
        kUnsignedLessThan = 16,
        kUnsignedGreaterThan = 17,
        kUnsignedLessThanEqual = 18,
        kUnsignedGreaterThanEqual = 19,
        kOverflow = overflow,
        kNoOverflow = nooverflow,
        kZero = 20,
        kNotZero = 21,
    }

    impl fmt::Display for Condition {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    #[inline]
    pub fn to_condition(cond: Condition) -> Condition {
        match cond {
            Condition::kUnsignedLessThan => Condition::lt,
            Condition::kUnsignedGreaterThan => Condition::gt,
            Condition::kUnsignedLessThanEqual => Condition::le,
            Condition::kUnsignedGreaterThanEqual => Condition::ge,
            Condition::kZero => Condition::eq,
            Condition::kNotZero => Condition::ne,
            _ => cond,
        }
    }

    #[inline]
    pub fn is_signed(cond: Condition) -> bool {
        match cond {
            Condition::kEqual
            | Condition::kNotEqual
            | Condition::kLessThan
            | Condition::kGreaterThan
            | Condition::kLessThanEqual
            | Condition::kGreaterThanEqual
            | Condition::kOverflow
            | Condition::kNoOverflow
            | Condition::kZero
            | Condition::kNotZero => true,

            Condition::kUnsignedLessThan
            | Condition::kUnsignedGreaterThan
            | Condition::kUnsignedLessThanEqual
            | Condition::kUnsignedGreaterThanEqual => false,

            _ => {
                crate::base::macros::UNREACHABLE!();
                false // Added to satisfy the compiler, unreachable
            }
        }
    }

    #[inline]
    pub fn negate_condition(cond: Condition) -> Condition {
        assert_ne!(cond, Condition::al);
        match cond {
            Condition::eq => Condition::ne,
            Condition::ne => Condition::eq,
            Condition::ge => Condition::lt,
            Condition::gt => Condition::le,
            Condition::le => Condition::gt,
            Condition::lt => Condition::ge,
            Condition::lt => Condition::ge,
            Condition::CC_OF => Condition::CC_NOF,
            Condition::kUnsignedLessThan => Condition::kUnsignedGreaterThanEqual,
            Condition::kUnsignedGreaterThan => Condition::kUnsignedLessThanEqual,
            Condition::kUnsignedLessThanEqual => Condition::kUnsignedGreaterThan,
            Condition::kUnsignedGreaterThanEqual => Condition::kUnsignedLessThan,

            _ => {
                // The original C++ code had a CHECK(false), which translates to an assert in debug builds
                // and nothing in release builds. To mimic that behavior and ensure that we catch
                // this in debug mode, but don't pay the cost in release, we use an assert.
                assert!(false);
                Condition::al // Provide a default to satisfy the compiler's return type requirement.
            }
        }
    }

    // -----------------------------------------------------------------------------
    // Instructions encoding.

    // Instr is merely used by the Assembler to distinguish 32bit integers
    // representing instructions from usual 32 bit values.
    // Instruction objects are pointers to 32bit values, and provide methods to
    // access the various ISA fields.
    pub type Instr = i32;
    pub type TwoByteInstr = u16;
    pub type FourByteInstr = u32;
    pub type SixByteInstr = u64;

    macro_rules! define_opcodes {
        ($macro:ident, $($name:ident, $enum_name:ident, $value:expr),*) => {
            #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
            #[allow(non_camel_case_types)]
            pub enum Opcode {
                $($enum_name = $value,)*
            }

            impl Opcode {
                pub fn value(&self) -> u32 {
                    *self as u32
                }
            }

            impl std::fmt::Display for Opcode {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{:?}", self)
                }
            }

            pub fn $macro<F>(mut f: F) where F: FnMut(Opcode) {
                $(f(Opcode::$enum_name);)*
            }
        };
    }

    define_opcodes!(
        S390_RSY_A_OPCODE_LIST,
        lmg, LMG, 0xEB04,
        srag, SRAG, 0xEB0A,
        slag, SLAG, 0xEB0B,
        srlg, SRLG, 0xEB0C,
        sllg, SLLG, 0xEB0D,
        tracg, TRACG, 0xEB0F,
        csy, CSY, 0xEB14,
        rllg, RLLG, 0xEB1C,
        rll, RLL, 0xEB1D,
        stmg, STMG, 0xEB24,
        stctg, STCTG, 0xEB25,
        stmh, STMH, 0xEB26,
        lctlg, LCTLG, 0xEB2F,
        csg, CSG, 0xEB30,
        cdsy, CDSY, 0xEB31,
        cdsg, CDSG, 0xEB3E,
        bxhg, BXHG, 0xEB44,
        bxleg, BXLEG, 0xEB45,
        ecag, ECAG, 0xEB4C,
        mvclu, MVCLU, 0xEB8E,
        clclu, CLCLU, 0xEB8F,
        stmy, STMY, 0xEB90,
        lmh, LMH, 0xEB96,
        lmy, LMY, 0xEB98,
        lamy, LAMY, 0xEB9A,
        stamy, STAMY, 0xEB9B,
        srak, SRAK, 0xEBDC,
        slak, SLAK, 0xEBDD,
        srlk, SRLK, 0xEBDE,
        sllk, SLLK, 0xEBDF,
        lang, LANG, 0xEBE4,
        laog, LAOG, 0xEBE6,
        laxg, LAXG, 0xEBE7,
        laag, LAAG, 0xEBE8,
        laalg, LAALG, 0xEBEA,
        lan, LAN, 0xEBF4,
        lao, LAO, 0xEBF6,
        lax, LAX, 0xEBF7,
        laa, LAA, 0xEBF8,
        laal, LAAL, 0xEBFA
    );

    define_opcodes!(
        S390_RSY_B_OPCODE_LIST,
        clmh, CLMH, 0xEB20,
        clmy, CLMY, 0xEB21,
        clt, CLT, 0xEB23,
        clgt, CLGT, 0xEB2B,
        stcmh, STCMH, 0xEB2C,
        stcmy, STCMY, 0xEB2D,
        icmh, ICMH, 0xEB80,
        icmy, ICMY, 0xEB81,
        locfh, LOCFH, 0xEBE0,
        stocfh, STOCFH, 0xEBE1,
        locg, LOCG, 0xEBE2,
        stocg, STOCG, 0xEBE3,
        loc, LOC, 0xEBF2,
        stoc, STOC, 0xEBF3
    );

    define_opcodes!(
        S390_RXE_OPCODE_LIST,
        lcbb, LCBB, 0xE727,
        ldeb, LDEB, 0xED04,
        lxdb, LXDB, 0xED05,
        lxeb, LXEB, 0xED06,
        mxdb, MXDB, 0xED07,
        keb, KEB, 0xED08,
        ceb, CEB, 0xED09,
        aeb, AEB, 0xED0A,
        seb, SEB, 0xED0B,
        mdeb, MDEB, 0xED0C,
        deb, DEB, 0xED0D,
        tceb, TCEB, 0xED10,
        tcdb, TCDB, 0xED11,
        tcxb, TCXB, 0xED12,
        sqeb, SQEB, 0xED14,
        sqdb, SQDB, 0xED15,
        meeb, MEEB, 0xED17,
        kdb, KDB, 0xED18,
        cdb, CDB, 0xED19,
        adb, ADB, 0xED1A,
        sdb, SDB, 0xED1B,
        mdb, MDB, 0xED1C,
        ddb, DDB, 0xED1D,
        lde, LDE, 0xED24,
        lxd, LXD, 0xED25,
        lxe, LXE, 0xED26,
        sqe, SQE, 0xED34,
        sqd, SQD, 0xED35,
        mee, MEE, 0xED37,
        tdcet, TDCET, 0xED50,
        tdget, TDGET, 0xED51,
        tdcdt, TDCDT, 0xED54,
        tdgdt, TDGDT, 0xED55,
        tdcxt, TDCXT, 0xED58,
        tdgxt, TDGXT, 0xED59
    );

    define_opcodes!(
        S390_RRF_A_OPCODE_LIST,
        ipte, IPTE, 0xB221,
        mdtra, MDTRA, 0xB3D0,
        ddtra, DDTRA, 0xB3D1,
        adtra, ADTRA, 0xB3D2,
        sdtra, SDTRA, 0xB3D3,
        mxtra, MXTRA, 0xB3D8,
        msrkc, MSRKC, 0xB9FD,
        msgrkc, MSGRKC, 0xB9ED,
        dxtra, DXTRA, 0xB3D9,
        axtra, AXTRA, 0xB3DA,
        sxtra, SXTRA, 0xB3DB,
        ahhhr, AHHHR, 0xB9C8,
        shhhr, SHHHR, 0xB9C9,
        alhhhr, ALHHHR, 0xB9CA,
        slhhhr, SLHHHR, 0xB9CB,
        ahhlr, AHHLR, 0xB9D8,
        shhlr, SHHLR, 0xB9D9,
        alhhlr, ALHHLR, 0xB9DA,
        slhhlr, SLHHLR, 0xB9DB,
        ngrk, NGRK, 0xB9E4,
        ogrk, OGRK, 0xB9E6,
        xgrk, XGRK, 0xB9E7,
        agrk, AGRK, 0xB9E8,
        sgrk, SGRK, 0xB9E9,
        mgrk, MGRK, 0xB9EC,
        algrk, ALGRK, 0xB9EA,
        slgrk, SLGRK, 0xB9EB,
        nrk, NRK, 0xB9F4,
        ork, ORK, 0xB9F6,
        xrk, XRK, 0xB9F7,
        ark, ARK, 0xB9F8,
        srk, SRK, 0xB9F9,
        alrk, ALRK, 0xB9FA,
        slrk, SLRK, 0xB9FB
    );

    define_opcodes!(
        S390_RXF_OPCODE_LIST,
        maeb, MAEB, 0xED0E,
        mseb, MSEB, 0xED0F,
        madb, MADB, 0xED1E,
        msdb, MSDB, 0xED1F,
        mae, MAE, 0xED2E,
        mse, MSE, 0xED2F,
        mayl, MAYL, 0xED38,
        myl, MYL, 0xED39,
        may, MAY, 0xED3A,
        my, MY, 0xED3B,
        mayh, MAYH, 0xED3C,
        myh, MYH, 0xED3D,
        mad, MAD, 0xED3E,
        msd, MSD, 0xED3F,
        sldt, SLDT, 0xED40,
        srdt, SRDT, 0xED41,
        slxt, SLXT, 0xED48,
        srxt, SRXT, 0xED49
    );

    define_opcodes!(S390_IE_OPCODE_LIST, niai, NIAI, 0xB2FA);

    define_opcodes!(
        S390_RRF_B_OPCODE_LIST,
        diebr, DIEBR, 0xB353,
        didbr, DIDBR, 0xB35B,
        cpsdr, CPSDR, 0xB372,
        qadtr, QADTR, 0xB3F5,
        iedtr, IEDTR, 0xB3F6,
        rrdtr, RRDTR, 0xB3F7,
        qaxtr, QAXTR, 0xB3FD,
        iextr, IEXTR, 0xB3FE,
        rrxtr, RRXTR, 0xB3FF,
        kmctr, KMCTR, 0xB92D,
        idte, IDTE, 0xB98E,
        crdte, CRDTE, 0xB98F,
        lptea, LPTEA, 0xB9AA
    );

    define_opcodes!(
        S390_RRF_C_OPCODE_LIST,
        sske, SSKE, 0xB22B,
        cu21, CU21, 0xB2A6,
        cu12, CU12, 0xB2A7,
        ppa, PPA, 0xB2E8,
        cgrt, CGRT, 0xB960,
        clgrt, CLGRT, 0xB961,
        crt, CRT, 0xB972,
        clrt, CLRT, 0xB973,
        trtt, TRTT, 0xB990,
        trto, TRTO, 0xB991,
        trot, TROT, 0xB992,
        troo, TROO, 0xB993,
        cu14, CU14, 0xB9B0,
        cu24, CU24, 0xB9B1,
        trtre, TRTRE, 0xB9BD,
        trte, TRTE, 0xB9BF,
        locfhr, LOCFHR, 0xB9E0,
        locgr, LOCGR, 0xB9E2,
        locr, LOCR, 0xB9F2
    );

    define_opcodes!(S390_MII_OPCODE_LIST, bprp, BPRP, 0xC5);

    define_opcodes!(
        S390_RRF_D_OPCODE_LIST,
        ldetr, LDETR, 0xB3D4,
        lxdtr, LXDTR, 0xB3DC,
        csdtr, CSDTR, 0xB3E3,
        csxtr, CSXTR, 0xB3EB
    );

    define_opcodes!(
        S390_RRF_E_OPCODE_LIST,
        ledbra, LEDBRA, 0xB344,
        ldxbra, LDXBRA, 0xB345,
        lexbra, LEXBRA, 0xB346,
        fixbra, FIXBRA, 0xB347,
        tbedr, TBEDR, 0xB350,
        tbdr, TBDR, 0xB351,
        fiebra, FIEBRA, 0xB357,
        fidbra, FIDBRA, 0xB35F,
        celfbr, CELFBR, 0xB390,
        cdlfbr, CDLFBR, 0xB391,
        cxlfbr, CXLFBR, 0xB392,
        cefbra, CEFBRA, 0xB394,
        cdfbra, CDFBRA, 0xB395,
        cxfbra, CXFBRA, 0xB396,
        cfebra, CFEBRA, 0xB398,
        cfdbra, CFDBRA, 0xB399,
        cfxbra, CFXBRA, 0xB39A,
        clfebr, CLFEBR, 0xB39C,
        clfdbr, CLFDBR, 0xB39D,
        clfxbr, CLFXBR, 0xB39E,
        celgbr, CELGBR, 0xB3A0,
        cdlgbr, CDLGBR, 0xB3A1,
        cxlgbr, CXLGBR, 0xB3A2,
        cegbra, CEGBRA, 0xB3A4,
        cdgbra, CDGBRA, 0xB3A5,
        cxgbra, CXGBRA, 0xB3A6,
        cgebra, CGEBRA, 0xB3A8,
        cgdbra, CGDBRA, 0xB3A9,
        cgxbra, CGXBRA, 0xB3AA,
        clgebr, CLGEBR, 0xB3AC,
        clgdbr, CLGDBR, 0xB3AD,
        clgxbr, CLGXBR, 0xB3AE,
        cfer, CFER, 0xB3B8,
        cfdr, CFDR, 0xB3B9,
        cfxr, CFXR, 0xB3BA,
        cger, CGER, 0xB3C8,
        cgdr, CGDR, 0xB3C9,
        cgxr, CGXR, 0xB3CA,
        ledtr, LEDTR, 0xB3D5,
        fidtr, FIDTR, 0xB3D7,
        ldxtr, LDXTR, 0xB3DD,
        fixtr, FIXTR, 0xB3DF,
        cgdtra, CGDTRA, 0xB3E1,
        cgxtra, CGXTRA, 0xB3E9,
        cdgtra, CDGTRA, 0xB3F1,
        cxgtra, CXGTRA, 0xB3F9,
        cfdtr, CFDTR, 0xB941,
        clgdtr, CLGDTR, 0xB942,
        clfdtr, CLFDTR, 0xB943,
        cfxtr, CFXTR, 0xB949,
        clgxtr, CLGXTR, 0xB94A,
        clfxtr, CLFXTR, 0xB94B,
        cdlgtr, CDLGTR, 0xB952,
        cdlftr, CDLFTR, 0xB953,
        cxlgtr, CXLGTR, 0xB95A,
        cxlftr, CXLFTR, 0xB95B
    );

    define_opcodes!(
        S390_VRR_A_OPCODE_LIST,
        vpopct, VPOPCT, 0xE750,
        vctz, VCTZ, 0xE752,
        vclz, VCLZ, 0xE753,
        vlr, VLR, 0xE756,
        vistr, VISTR, 0xE75C,
        vseg, VSEG, 0xE75F,
        vclgd, VCLGD, 0xE7C0,
        vcdlg, VCDLG, 0xE7C1,
        vcgd, VCGD, 0xE7C2,
        vcdg, VCDG, 0xE7C3,
        vlde, VLDE, 0xE7C4,
        vled, VLED, 0xE7C5,
        vfi, VFI, 0xE7C7,
        wfk, WFK, 0xE7CA,
        wfc, WFC, 0xE7CB,
        vfpso, VFPSO, 0xE7CC,
        vfsq, VFSQ, 0xE7CE,
        vupll, VUPLL, 0xE7D4,
        vuplh, VUPLH, 0xE7D5,
        vupl, VUPL, 0xE7D6,
        vuph, VUPH, 0xE7D7,
        vtm, VTM, 0xE7D8,
        vecl, VECL, 0xE7D9,
        vec, VEC, 0xE7DB,
        vlc, VLC, 0xE7DE,
        vlp, VLP, 0xE7DF
    );

    define_opcodes!(
        S390_VRR_B_OPCODE_LIST,
        vfee, VFEE, 0xE780,
        vfene, VFENE, 0xE781,
        vfae, VFAE, 0xE782,
        vpkls, VPKLS, 0xE795,
        vpks, VPKS, 0xE797,
        vceq, VCEQ, 0xE7F8,
        vchl, VCHL, 0xE7F9,
        vch, VCH, 0xE7FB
    );

    define_opcodes!(
        S390_VRR_C_OPCODE_LIST,
        vmrl, VMRL, 0xE760,
        vmrh, VMRH, 0xE761,
        vsum, VSUM, 0xE764,
        vsumg, VSUMG, 0xE765,
        vcksm, VCKSM, 0xE766,
        vsumq, VSUMQ, 0xE767,
        vn, VN, 0xE768,
        vnc, VNC, 0xE769,
        vo, VO, 0xE76A,
        vno, VNO, 0xE76B,
        vx, VX, 0xE76D,
        veslv, VESLV, 0xE770,
        verllv, VERLLV, 0xE773,
        vsl, VSL, 0xE774,
        vslb, VSLB, 0xE775,
        vesrlv, VESRLV, 0xE778,
        vesrav, VESRAV, 0xE77A,
        vsrl, VSRL, 0xE77C,
        vsrlb, VSRLB, 0xE77D,
        vsra, VSRA, 0xE77E,
        vsrab, VSRAB, 0xE77F,
        vpdi, VPDI, 0xE784,
        vpk, VPK, 0xE794,
        vmlh, VMLH, 0xE7A1,
        vml, VML, 0xE7A2,
        vmh, VMH, 0xE7A3,
        vmle, VMLE, 0xE7A4,
        vmlo, VMLO, 0xE7A5,
        vme, VME, 0xE7A6,
        vmo, VMO, 0xE7A7,
        vgfm, VGFM, 0xE7B4,
        vfs, VFS, 0xE7E2,
        vfa, VFA, 0xE7E3,
        vfd, VFD, 0xE7E5,
        vfm, VFM, 0xE7E7,
        vfce, VFCE, 0xE7E8,
        vfche, VFCHE, 0xE7EA,
        vfch, VFCH, 0xE7EB,
        vfmax, VFMAX, 0xE7EF,
        vfmin, VFMIN, 0xE7EE,
        vavgl, VAVGL, 0xE7F0,
        vacc, VACC, 0xE7F1,
        vavg, VAVG, 0xE7F2,
        va, VA, 0xE7F3,
        vscbi, VSCBI, 0xE7F5,
        vs, VS, 0xE7F7,
        vmnl, VMNL, 0xE7FC,
        vmxl, VMXL, 0xE7FD,
        vmn, VMN, 0xE7FE,
        vmx, VMX, 0xE7FF,
        vbperm, VBPERM, 0xE785
    );

    define_opcodes!(
        S390_VRI_A_OPCODE_LIST,
        vleib, VLEIB, 0xE740,
        vleih, VLEIH, 0xE741,
        vleig, VLEIG, 0xE7