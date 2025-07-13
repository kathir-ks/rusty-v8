// Converted from V8 C++ source files:
// Header: constants-s390.h
// Implementation: constants-s390.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_camel_case_types)]

#[cfg(all(target_arch = "s390x"))]
pub mod constants_s390 {
    use std::{
        fmt::{self, Debug, Display, Formatter},
        sync::Mutex,
    };

    // Get the standard printf format macros for C99 stdint types.
    // Already handled by inttypes and stdint imports

    use crate::base::macros::UNREACHABLE;

    // UNIMPLEMENTED_ macro for S390.
    #[macro_export]
    macro_rules! unimplemented_s390 {
        () => {
            #[cfg(debug_assertions)]
            {
                println!(
                    "{}, line {}: function {} not implemented.",
                    file!(),
                    line!(),
                    function!()
                );
            }
        };
    }

    #[cfg(target_os = "zos")]
    pub const ABI_USES_FUNCTION_DESCRIPTORS: i32 = 1;
    #[cfg(target_os = "zos")]
    pub const ABI_PASSES_HANDLES_IN_REGS: i32 = 1;
    #[cfg(target_os = "zos")]
    pub const ABI_RETURNS_OBJECTPAIR_IN_REGS: i32 = 1;
    #[cfg(target_os = "zos")]
    mod zos_conflicts {
        #[allow(unused)]
        pub use std::{
            convert::TryFrom,
            ffi::{CStr, CString},
        };
    }

    #[cfg(not(target_os = "zos"))]
    pub const ABI_USES_FUNCTION_DESCRIPTORS: i32 = 0;
    #[cfg(not(target_os = "zos"))]
    pub const ABI_PASSES_HANDLES_IN_REGS: i32 = 1;
    #[cfg(not(target_os = "zos"))]
    pub const ABI_RETURNS_OBJECTPAIR_IN_REGS: i32 = 0;

    pub const ABI_CALL_VIA_IP: i32 = 1;

    // The maximum size of the code range s.t. pc-relative calls are possible
    // between all Code objects in the range.
    pub const K_MAX_PC_RELATIVE_CODE_RANGE_IN_MB: usize = 4096;

    #[cfg(target_os = "zos")]
    pub const K_HAS_FUNCTION_DESCRIPTOR_BIT_SHIFT: i32 = 4;
    #[cfg(target_os = "zos")]
    pub const K_HAS_FUNCTION_DESCRIPTOR_BIT_MASK: i32 =
        1 << K_HAS_FUNCTION_DESCRIPTOR_BIT_SHIFT;

    // Number of registers
    pub const K_NUM_REGISTERS: i32 = 16;

    // FP support.
    pub const K_NUM_DOUBLE_REGISTERS: i32 = 16;

    pub const K_NO_REGISTER: i32 = -1;

    // The actual value of the kRootRegister is offset from the IsolateData's start
    // to take advantage of negative displacement values.
    pub const K_ROOT_REGISTER_BIAS: i32 = 128;

    // sign-extend the least significant 16-bits of value <imm>
    macro_rules! sign_ext_imm16 {
        ($imm:expr) => {
            (($imm as i32) << 16 >> 16) as i32
        };
    }

    // sign-extend the least significant 26-bits of value <imm>
    macro_rules! sign_ext_imm26 {
        ($imm:expr) => {
            (($imm as i32) << 6 >> 6) as i32
        };
    }

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
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Condition {
        kNoCondition = -1,
        eq = 0x8,   // Equal.
        ne = 0x7,   // Not equal.
        ge = 0xa,   // Greater or equal.
        lt = 0x4,   // Less than.
        gt = 0x2,   // Greater than.
        le = 0xc,   // Less then or equal
        al = 0xf,   // Always.
        CC_NOP = 0x0,    // S390 NOP
        CC_EQ = 0x08,    // S390 condition code 0b1000
        CC_LT = 0x04,    // S390 condition code 0b0100
        CC_LE = CC_EQ | CC_LT, // S390 condition code 0b1100
        CC_GT = 0x02,    // S390 condition code 0b0010
        CC_GE = CC_EQ | CC_GT, // S390 condition code 0b1010
        CC_OF = 0x01,    // S390 condition code 0b0001
        CC_NOF = 0x0E,   // S390 condition code 0b1110
        CC_ALWAYS = 0x0F, // S390 always taken branch
        unordered = CC_OF, // Floating-point unordered
        ordered = CC_NOF,  // floating-point ordered
        overflow = CC_OF,  // Summary overflow
        nooverflow = CC_NOF,
        mask0x0 = 0,   // no jumps
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

    impl Display for Condition {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            Debug::fmt(self, f)
        }
    }

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
                unimplemented_s390!();
                false
            }
        }
    }

    pub fn negate_condition(cond: Condition) -> Condition {
        if cond == Condition::al {
            return Condition::al;
        }
        match cond {
            Condition::eq => Condition::ne,
            Condition::ne => Condition::eq,
            Condition::ge => Condition::lt,
            Condition::gt => Condition::le,
            Condition::le => Condition::gt,
            Condition::lt => Condition::ge,
            Condition::lt | Condition::gt => Condition::eq,
            Condition::le | Condition::ge => Condition::CC_OF,
            Condition::CC_OF => Condition::CC_NOF,
            Condition::kUnsignedLessThan => Condition::kUnsignedGreaterThanEqual,
            Condition::kUnsignedGreaterThan => Condition::kUnsignedLessThanEqual,
            Condition::kUnsignedLessThanEqual => Condition::kUnsignedGreaterThan,
            Condition::kUnsignedGreaterThanEqual => Condition::kUnsignedLessThan,
            _ => {
                unimplemented_s390!();
                Condition::al
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

    macro_rules! declare_s390_rsy_a_opcodelist {
        ($(#[$attr:meta])* $vis:vis enum $name:ident {
            $($(#[$v_attr:meta])* $v_name:ident = $v_value:expr,)*
        }) => {
            $(#[$attr])*
            $vis enum $name {
                $($(#[$v_attr])* $v_name = $v_value,)*
            }
            impl $name {
                $vis fn from_value(value: i32) -> Option<Self> {
                    match value {
                        $($v_value => Some(Self::$v_name),)*
                        _ => None,
                    }
                }
            }
        }
    }

    macro_rules! define_opcodes {
        ($vis:vis enum $name:ident {
            $($(#[$attr:meta])* $field:ident = $value:expr,)*
        }) => {
            #[derive(Debug, Copy, Clone, PartialEq, Eq)]
            $vis enum $name {
                $($(#[$attr])* $field = $value,)*
            }
        }
    }

    define_opcodes! {
        pub enum Opcode {
            BKPT = 0x0001,  // GDB Software Breakpoint
            DUMY = 0xE352 , // Special dummy opcode

            lmg = 0xEB04 ,
            SRAG = 0xEB0A,
            SLAG = 0xEB0B,
            SRLG = 0xEB0C,
            SLLG = 0xEB0D,
            TRACG = 0xEB0F,
            CSY = 0xEB14,
            RLLG = 0xEB1C,
            RLL = 0xEB1D,
            STMG = 0xEB24,
            STCTG = 0xEB25,
            STMH = 0xEB26,
            LCTLG = 0xEB2F,
            CSG = 0xEB30,
            CDSY = 0xEB31,
            CDSG = 0xEB3E,
            BXHG = 0xEB44,
            BXLEG = 0xEB45,
            ECAG = 0xEB4C,
            MVCLU = 0xEB8E,
            CLCLU = 0xEB8F,
            STMY = 0xEB90,
            LMH = 0xEB96,
            LMY = 0xEB98,
            LAMY = 0xEB9A,
            STAMY = 0xEB9B,
            SRAK = 0xEBDC,
            SLAK = 0xEBDD,
            SRLK = 0xEBDE,
            SLLK = 0xEBDF,
            LANG = 0xEBE4,
            LAOG = 0xEBE6,
            LAXG = 0xEBE7,
            LAAG = 0xEBE8,
            LAALG = 0xEBEA,
            LAN = 0xEBF4,
            LAO = 0xEBF6,
            LAX = 0xEBF7,
            LAA = 0xEBF8,
            LAAL = 0xEBFA,
            CLMH = 0xEB20,
            CLMY = 0xEB21,
            CLT = 0xEB23,
            CLGT = 0xEB2B,
            STCMH = 0xEB2C,
            STCMY = 0xEB2D,
            ICMH = 0xEB80,
            ICMY = 0xEB81,
            LOCFH = 0xEBE0,
            STOCFH = 0xEBE1,
            LOCG = 0xEBE2,
            STOCG = 0xEBE3,
            LOC = 0xEBF2,
            STOC = 0xEBF3,
            LCBB = 0xE727,
            LDEB = 0xED04,
            LXDB = 0xED05,
            LXEB = 0xED06,
            MXDB = 0xED07,
            KEB = 0xED08,
            CEB = 0xED09,
            AEB = 0xED0A,
            SEB = 0xED0B,
            MDEB = 0xED0C,
            DEB = 0xED0D,
            TCEB = 0xED10,
            TCDB = 0xED11,
            TCXB = 0xED12,
            SQEB = 0xED14,
            SQDB = 0xED15,
            MEEB = 0xED17,
            KDB = 0xED18,
            CDB = 0xED19,
            ADB = 0xED1A,
            SDB = 0xED1B,
            MDB = 0xED1C,
            DDB = 0xED1D,
            LDE = 0xED24,
            LXD = 0xED25,
            LXE = 0xED26,
            SQE = 0xED34,
            SQD = 0xED35,
            MEE = 0xED37,
            TDCET = 0xED50,
            TDGET = 0xED51,
            TDCDT = 0xED54,
            TDGDT = 0xED55,
            TDCXT = 0xED58,
            TDGXT = 0xED59,
            IPTE = 0xB221,
            MDTRA = 0xB3D0,
            DDTRA = 0xB3D1,
            ADTRA = 0xB3D2,
            SDTRA = 0xB3D3,
            MXTRA = 0xB3D8,
            MSRKC = 0xB9FD,
            MSGRKC = 0xB9ED,
            DXTRA = 0xB3D9,
            AXTRA = 0xB3DA,
            SXTRA = 0xB3DB,
            AHHHR = 0xB9C8,
            SHHHR = 0xB9C9,
            ALHHHR = 0xB9CA,
            SLHHHR = 0xB9CB,
            AHHLR = 0xB9D8,
            SHHLR = 0xB9D9,
            ALHHLR = 0xB9DA,
            SLHHLR = 0xB9DB,
            NGRK = 0xB9E4,
            OGRK = 0xB9E6,
            XGRK = 0xB9E7,
            AGRK = 0xB9E8,
            SGRK = 0xB9E9,
            MGRK = 0xB9EC,
            ALGRK = 0xB9EA,
            SLGRK = 0xB9EB,
            NRK = 0xB9F4,
            ORK = 0xB9F6,
            XRK = 0xB9F7,
            ARK = 0xB9F8,
            SRK = 0xB9F9,
            ALRK = 0xB9FA,
            SLRK = 0xB9FB,
            MAEB = 0xED0E,
            MSEB = 0xED0F,
            MADB = 0xED1E,
            MSDB = 0xED1F,
            MAE = 0xED2E,
            MSE = 0xED2F,
            MAYL = 0xED38,
            MYL = 0xED39,
            MAY = 0xED3A,
            MY = 0xED3B,
            MAYH = 0xED3C,
            MYH = 0xED3D,
            MAD = 0xED3E,
            MSD = 0xED3F,
            SLDT = 0xED40,
            SRDT = 0xED41,
            SLXT = 0xED48,
            SRXT = 0xED49,
            NIAI = 0xB2FA,
            DIEBR = 0xB353,
            DIDBR = 0xB35B,
            CPSDR = 0xB372,
            QADTR = 0xB3F5,
            IEDTR = 0xB3F6,
            RRDTR = 0xB3F7,
            QAXTR = 0xB3FD,
            IEXTR = 0xB3FE,
            RRXTR = 0xB3FF,
            KMCTR = 0xB92D,
            IDTE = 0xB98E,
            CRDTE = 0xB98F,
            LPTEA = 0xB9AA,
            SSKE = 0xB22B,
            CU21 = 0xB2A6,
            CU12 = 0xB2A7,
            PPA = 0xB2E8,
            CGRT = 0xB960,
            CLGRT = 0xB961,
            CRT = 0xB972,
            CLRT = 0xB973,
            TRTT = 0xB990,
            TRTO = 0xB991,
            TROT = 0xB992,
            TROO = 0xB993,
            CU14 = 0xB9B0,
            CU24 = 0xB9B1,
            TRTRE = 0xB9BD,
            TRTE = 0xB9BF,
            LOCFHR = 0xB9E0,
            LOCGR = 0xB9E2,
            LOCR = 0xB9F2,
            BPRP = 0xC5,
            LDETR = 0xB3D4,
            LXDTR = 0xB3DC,
            CSDTR = 0xB3E3,
            CSXTR = 0xB3EB,
            LEDBRA = 0xB344,
            LDXBRA = 0xB345,
            LEXBRA = 0xB346,
            FIXBRA = 0xB347,
            TBEDR = 0xB350,
            TBDR = 0xB351,
            FIEBRA = 0xB357,
            FIDBRA = 0xB35F,
            CELFBR = 0xB390,
            CDLFBR = 0xB391,
            CXLFBR = 0xB392,
            CEFBRA = 0xB394,
            CDFBRA = 0xB395,
            CXFBRA = 0xB396,
            CFEBRA = 0xB398,
            CFDBRA = 0xB399,
            CFXBRA = 0xB39A,
            CLFEBR = 0xB39C,
            CLFDBR = 0xB39D,
            CLFXBR = 0xB39E,
            CELGBR = 0xB3A0,
            CDLGBR = 0xB3A1,
            CXLGBR = 0xB3A2,
            CEGBRA = 0xB3A4,
            CDGBRA = 0xB3A5,
            CXGBRA = 0xB3A6,
            CGEBRA = 0xB3A8,
            CGDBRA = 0xB3A9,
            CGXBRA = 0xB3AA,
            CLGEBR = 0xB3AC,
            CLGDBR = 0xB3AD,
            CLGXBR = 0xB3AE,
            CFER = 0xB3B8,
            CFDR = 0xB3B9,
            CFXR = 0xB3BA,
            LEDTR = 0xB3D5,
            FIDTR = 0xB3D7,
            LDXTR = 0xB3DD,
            FIXTR = 0xB3DF,
            CGDTRA = 0xB3E1,
            CGXTRA = 0xB3E9,
            CDGTRA = 0xB3F1,
            CXGTRA = 0xB3F9,
            CFDTR = 0xB941,
            CLGDTR = 0xB942,
            CLFDTR = 0xB943,
            CFXTR = 0xB949,
            CLGXTR = 0xB94A,
            CLFXTR = 0xB94B,
            CDLGTR = 0xB952,
            CDLFTR = 0xB953,
            CXLGTR = 0xB95A,
            CXLFTR = 0xB95B,
            VPOPCT = 0xE750,
            VCTZ = 0xE752,
            VCLZ = 0xE753,
            VLR = 0xE756,
            VISTR = 0xE75C,
            VSEG = 0xE75F,
            VCLGD = 0xE7C0,
            VCDLG = 0xE7C1,
            VCGD = 0xE7C2,
            VCDG = 0xE7C3,
            VLDE = 0xE7C4,
            VLED = 0xE7C5,
            VFI = 0xE7C7,
            WFK = 0xE7CA,
            WFC = 0xE7CB,
            VFPSO = 0xE7CC,
            VFSQ = 0xE7CE,
            VUPLL = 0xE7D4,
            VUPLH = 0xE7D5,
            VUPL = 0xE7D6,
            VUPH = 0xE7D7,
            VTM = 0xE7D8,
            VECL = 0xE7D9,
            VEC = 0xE7DB,
            VLC = 0xE7DE,
            VLP = 0xE7DF,
            VFEE = 0xE780,
            VFENE = 0xE781,
            VFAE = 0xE782,
            VPKLS = 0xE795,
            VPKS = 0xE797,
            VCEQ = 0xE7F8,
            VCHL = 0xE7F9,
            VCH = 0xE7FB,
            VMRL = 0xE760,
            VMRH = 0xE761,
            VSUM = 0xE764,
            VSUMG = 0xE765,
            VCKSM = 0xE766,
            VSUMQ = 0xE767,
            VN = 0xE768,
            VNC = 0xE769,
            VO = 0xE76A,
            VNO = 0xE76B,
            VX = 0xE76D,
            VESLV = 0xE770,
            VERLLV = 0xE773,
            VSL = 0xE774,
            VSLB = 0xE775,
            VESRLV = 0xE778,
            VESRAV = 0xE77A,
            VSRL = 0xE77C,
            VSRLB = 0xE77D,
            VSRA = 0xE77E,
            VSRAB = 0xE77F,
            VPDI = 0xE784,
            VPK = 0xE794,
            VMLH = 0xE7A1,
            VML = 0xE7A2,
            VMH = 0xE7A3,
            VMLE = 0xE7A4,
            VMLO = 0xE7A5,
            VME = 0xE7A6,
            VMO = 0xE7A7,
            VGFM = 0xE7B4,
            VFS = 0xE7E2,
            VFA = 0xE7E3,
            VFD = 0xE7E5,
            VFM = 0xE7E7,
            VFCE = 0xE7E8,
            VFCHE = 0xE7EA,
            VFCH = 0xE7EB,
            VFMAX = 0xE7EF,
            VFMIN = 0xE7EE,
            VAVGL = 0xE7F0,
            VACC = 0xE7F1,
            VAVG = 0xE7F2,
            VA = 0xE7F3,
            VSCBI = 0xE7F5,
            VS = 0xE7F7,
            VMNL = 0xE7FC,
            VMXL = 0xE7FD,
            VMN = 0xE7FE,
            VMX = 0xE7FF,
            VBPERM = 0xE785,
            VLEIB = 0xE740,
            VLEIH = 0xE741,
            VLEIG = 0xE742,
            VLEIF = 0xE743,
            VGBM = 0xE744,
            VREPI = 0xE745,
            vstrc = 0xE78A,
            vmalh = 0xE7A9,
            vmal = 0xE7AA,
            vmah = 0xE7AB,
            vmale = 0xE7AC,
            vmalo = 0xE7AD,
            vmae = 0xE7AE,
            vmao = 0xE7AF,
            vaccc = 0xE7B9,
            vac = 0xE7BB,
            vgfma = 0xE7BC,
            vsbcbi = 0xE7BD,
            vsbi = 0xE7BF,
            vgm = 0xE746,
            VPERM = 0xE78C,
            VSEL = 0xE78D,
            VFMS = 0xE78E,
            VFNMS = 0xE79E,
            VFMA = 0xE78F,
            VREP = 0xE74D,
            VERIM = 0xE772,
            VSLDB = 0xE777,
            VLVG = 0xE721,
            VLL = 0xE737,
            VSTL = 0xE73F,
            BRCL = 0xC04,
            PFDRL = 0xC62,
            VLVGP = 0xE762,
            IIHH = 0xA50,
            IIHL = 0xA51,
            IILH = 0xA52,
            IILL = 0xA53,
            NIHH = 0xA54,
            NIHL = 0xA55,
            NILH = 0xA56,
            NILL = 0xA57,
            OIHH = 0xA58,
            OIHL = 0xA59,
            OILH = 0xA5A,
            OILL = 0xA5B,
            LLIHH = 0xA5C,
            LLIHL = 0xA5D,
            LLILH = 0xA5E,
            LLILL = 0xA5F,
            TMLH = 0xA70,
            TMLL = 0xA71,
            TMHH = 0xA72,
            TMHL = 0xA73,
            LHI = 0xA78,
            LGHI = 0xA79,
            AHI = 0xA7A,
            AGHI = 0xA7B,
            MHI = 0xA7C,
            MGHI = 0xA7D,
            CHI = 0xA7E,
            CGHI = 0xA7F,
            BRXH = 0x84,
            BRXLE = 0x85,
            BRAS = 0xA75,
            BRCT = 0xA76,
            BRCTG = 0xA77,
            BRC = 0xA74,
            BPP = 0xC7,
            LTG = 0xE302,
            LRAG = 0xE303,
            LG = 0xE304,
            CVBY = 0xE306,
            AG = 0xE308,
            SG = 0xE309,
            ALG = 0xE30A,
            SLG = 0xE30B,
            MSG = 0xE30C,
            DSG = 0xE30D,
            CVBG = 0xE30E,
            LRVG = 0xE30F,
            LT = 0xE312,
            LRAY = 0xE313,
            LGF = 0xE314,
            LGH = 0xE315,
            LLGF = 0xE316,
            LLGT = 0xE317,
            AGF = 0xE318,
            SGF = 0xE319,
            ALGF = 0xE31A,
            SLGF = 0xE31B,
            MSGF = 0xE31C,
            DSGF = 0xE31D,
            LRV = 0xE31E,
            LRVH = 0xE31F,
            CG = 0xE320,
            CLG = 0xE321,
            STG = 0xE324,
            NTSTG = 0xE325,
            CVDY = 0xE326,
            LZRG = 0xE32A,
            CVDG = 0xE32E,
            STRVG = 0xE32F,
            CGF = 0xE330,
            CLGF = 0xE331,
            LTGF = 0xE332,
            CGH = 0xE334,
            LLZRGF = 0xE33A,
            LZRF = 0xE33B,
            STRV = 0xE33E,
            STRVH = 0xE33F,
            BCTG = 0xE346,
            STY = 0xE350,
            MSY = 0xE351,
            NY = 0xE354,
            CLY = 0xE355,
            OY = 0xE356,
            XY = 0xE357,
            LY = 0xE358,
            CY = 0xE359,
            AY = 0xE35A,
            SY = 0xE35B,
            MFY = 0xE35C,
            MG = 0xE384,
            ALY = 0xE35E,
            SLY = 0xE35F,
            STHY = 0xE370,
            LAY = 0xE371,
            STCY = 0xE372,
            ICY = 0xE373,
            LAEY = 0xE375,
            LB = 0xE376,
