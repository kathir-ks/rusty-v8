// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::mem;

/// UNIMPLEMENTED_ macro for PPC.
#[macro_export]
macro_rules! unimplemented_ppc {
    () => {
        #[cfg(debug_assertions)]
        {
            println!(
                "{}, \tline {}: \tfunction {} not implemented. \n",
                file!(),
                line!(),
                function!()
            );
        }
        #[cfg(not(debug_assertions))]
        {}
    };
}

pub const ABI_USES_FUNCTION_DESCRIPTORS: i32 = {
    if cfg!(all(
        target_arch = "powerpc64",
        any(target_os = "aix", all(target_arch = "powerpc64", target_endian = "big", not(defined(_CALL_ELF))))
    )) {
        1
    } else {
        0
    }
};

pub const ABI_PASSES_HANDLES_IN_REGS: i32 = {
    if cfg!(any(
        not(target_arch = "powerpc64"),
        target_os = "aix",
        target_arch = "powerpc64"
    )) {
        1
    } else {
        0
    }
};

pub const ABI_RETURNS_OBJECT_PAIRS_IN_REGS: i32 = {
    if cfg!(any(
        not(target_arch = "powerpc64"),
        not(target_arch = "powerpc64"),
        target_endian = "little",
        defined(_CALL_ELF) && _CALL_ELF == 2
    )) {
        1
    } else {
        0
    }
};

pub const ABI_CALL_VIA_IP: i32 = {
    if cfg!(any(
        not(target_arch = "powerpc64"),
        all(
            target_arch = "powerpc64",
            any(target_endian = "little", defined(_CALL_ELF) && _CALL_ELF == 2)
        )
    )) {
        1
    } else {
        0
    }
};

pub const ABI_TOC_REGISTER: i32 = {
    if cfg!(any(
        not(target_arch = "powerpc64"),
        target_os = "aix",
        target_arch = "powerpc64"
    )) {
        2
    } else {
        13
    }
};

pub mod v8 {
    pub mod internal {

        // TODO(sigurds): Change this value once we use relative jumps.
        pub const K_MAX_PC_RELATIVE_CODE_RANGE_IN_MB: usize = 0;

        // Used to encode a boolean value when emitting 32 bit
        // opcodes which will indicate the presence of function descriptors
        pub const K_HAS_FUNCTION_DESCRIPTOR_BIT_SHIFT: i32 = 4;
        pub const K_HAS_FUNCTION_DESCRIPTOR_BIT_MASK: i32 = 1 << K_HAS_FUNCTION_DESCRIPTOR_BIT_SHIFT;

        // Number of registers
        pub const K_NUM_REGISTERS: i32 = 32;

        // FP support.
        pub const K_NUM_DOUBLE_REGISTERS: i32 = 32;

        pub const K_NO_REGISTER: i32 = -1;

        // Used in embedded constant pool builder - max reach in bits for
        // various load instructions (one less due to unsigned)
        pub const K_LOAD_PTR_MAX_REACH_BITS: i32 = 15;
        pub const K_LOAD_DOUBLE_MAX_REACH_BITS: i32 = 15;

        // The actual value of the kRootRegister is offset from the IsolateData's start
        // to take advantage of negative displacement values.
        pub const K_ROOT_REGISTER_BIAS: i32 = 128;

        // sign-extend the least significant 5-bits of value <imm>
        #[inline]
        pub fn sign_ext_imm5(imm: i32) -> i32 {
            ((imm << 27) >> 27) as i32
        }

        // sign-extend the least significant 16-bits of value <imm>
        #[inline]
        pub fn sign_ext_imm16(imm: i32) -> i32 {
            ((imm << 16) >> 16) as i32
        }

        // sign-extend the least significant 14-bits of value <imm>
        #[inline]
        pub fn sign_ext_imm18(imm: i32) -> i32 {
            ((imm << 14) >> 14) as i32
        }

        // sign-extend the least significant 22-bits of value <imm>
        #[inline]
        pub fn sign_ext_imm22(imm: i32) -> i32 {
            ((imm << 10) >> 10) as i32
        }

        // sign-extend the least significant 26-bits of value <imm>
        #[inline]
        pub fn sign_ext_imm26(imm: i32) -> i32 {
            ((imm << 6) >> 6) as i32
        }

        // sign-extend the least significant 34-bits of prefix+suffix value <imm>
        #[inline]
        pub fn sign_ext_imm34(imm: i64) -> i64 {
            ((imm << 30) >> 30) as i64
        }

        // -----------------------------------------------------------------------------
        // Conditions.

        /// Defines constants and accessor classes to assemble, disassemble and
        /// simulate PPC instructions.
        ///
        /// Section references in the code refer to the "PowerPC Microprocessor
        /// Family: The Programmer.s Reference Guide" from 10/95
        /// https://www-01.ibm.com/chips/techlib/techlib.nsf/techdocs/852569B20050FF778525699600741775/$file/prg.pdf
        ///

        /// Constants for specific fields are defined in their respective named enums.
        /// General constants are in an anonymous enum in class Instr.
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        #[repr(i32)]
        pub enum Condition {
            kNoCondition = -1,
            eq = 0,         // Equal.
            ne = 1,         // Not equal.
            ge = 2,         // Greater or equal.
            lt = 3,         // Less than.
            gt = 4,         // Greater than.
            le = 5,         // Less then or equal
            unordered = 6,  // Floating-point unordered
            ordered = 7,
            overflow = 8,  // Summary overflow
            nooverflow = 9,
            al = 10,  // Always.

            // Unified cross-platform condition names/aliases.
            // Do not set unsigned constants equal to their signed variants.
            // We need to be able to differentiate between signed and unsigned enum
            // constants in order to emit the right instructions (i.e CmpS64 vs CmpU64).
            kEqual = eq as i32,
            kNotEqual = ne as i32,
            kLessThan = lt as i32,
            kGreaterThan = gt as i32,
            kLessThanEqual = le as i32,
            kGreaterThanEqual = ge as i32,
            kUnsignedLessThan = 11,
            kUnsignedGreaterThan = 12,
            kUnsignedLessThanEqual = 13,
            kUnsignedGreaterThanEqual = 14,
            kOverflow = overflow as i32,
            kNoOverflow = nooverflow as i32,
            kZero = 15,
            kNotZero = 16,
        }

        impl Condition {
            pub fn to_condition(self) -> Self {
                match self {
                    Condition::kUnsignedLessThan => Condition::lt,
                    Condition::kUnsignedGreaterThan => Condition::gt,
                    Condition::kUnsignedLessThanEqual => Condition::le,
                    Condition::kUnsignedGreaterThanEqual => Condition::ge,
                    Condition::kZero => Condition::eq,
                    Condition::kNotZero => Condition::ne,
                    _ => self,
                }
            }

            pub fn is_signed(self) -> bool {
                match self {
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
                        unreachable!();
                    }
                }
            }

            pub fn negate_condition(self) -> Self {
                assert_ne!(self, Condition::al);
                unsafe { mem::transmute(self as i32 ^ Condition::ne as i32) }
            }
        }

        // -----------------------------------------------------------------------------
        // Instructions encoding.

        /// Instr is merely used by the Assembler to distinguish 32bit integers
        /// representing instructions from usual 32 bit values.
        /// Instruction objects are pointers to 32bit values, and provide methods to
        /// access the various ISA fields.
        pub type Instr = u32;

        #[allow(clippy::enum_variant_names)]
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        #[repr(u32)]
        pub enum XX3_OPCODE_SCALAR {
            xsadddp = 0xF0000100,
            xsaddsp = 0xF0000000,
            xscmpodp = 0xF0000158,
            xscmpudp = 0xF0000118,
            xscpsgndp = 0xF0000580,
            xsdivdp = 0xF00001C0,
            xsdivsp = 0xF00000C0,
            xsmaddadp = 0xF0000108,
            xsmaddasp = 0xF0000008,
            xsmaddmdp = 0xF0000148,
            xsmaddmsp = 0xF0000048,
            xsmaxdp = 0xF0000500,
            xsmindp = 0xF0000540,
            xsmsubadp = 0xF0000188,
            xsmsubasp = 0xF0000088,
            xsmsubmdp = 0xF00001C8,
            xsmsubmsp = 0xF00000C8,
            xsmuldp = 0xF0000180,
            xsmulsp = 0xF0000080,
            xsnmaddadp = 0xF0000508,
            xsnmaddasp = 0xF0000408,
            xsnmaddmdp = 0xF0000548,
            xsnmaddmsp = 0xF0000448,
            xsnmsubadp = 0xF0000588,
            xsnmsubasp = 0xF0000488,
            xsnmsubmdp = 0xF00005C8,
            xsnmsubmsp = 0xF00004C8,
            xsredp = 0xF0000168,
            xssubdp = 0xF0000140,
            xssubsp = 0xF0000040,
            xstdivdp = 0xF00001E8,
        }

        #[allow(clippy::enum_variant_names)]
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        #[repr(u32)]
        pub enum XX3_OPCODE_VECTOR_A_FORM {
            xvcmpeqsp = 0xF0000218,
            xvcmpeqdp = 0xF0000318,
        }

        #[allow(clippy::enum_variant_names)]
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        #[repr(u32)]
        pub enum XX3_OPCODE_VECTOR_B_FORM {
            xvadddp = 0xF0000300,
            xvaddsp = 0xF0000200,
            xvcmpeqdpx = 0xF0000718,
            xvcmpeqspx = 0xF0000618,
            xvcmpgedp = 0xF0000398,
            xvcmpgedpx = 0xF0000798,
            xvcmpgesp = 0xF0000298,
            xvcmpgespx = 0xF0000698,
            xvcmpgtdp = 0xF0000358,
            xvcmpgtdpx = 0xF0000758,
            xvcmpgtsp = 0xF0000258,
            xvcmpgtspx = 0xF0000658,
            xvcpsgndp = 0xF0000780,
            xvcpsgnsp = 0xF0000680,
            xvdivdp = 0xF00003C0,
            xvdivsp = 0xF00002C0,
            xvmaddadp = 0xF0000308,
            xvmaddasp = 0xF0000208,
            xvmaddmdp = 0xF0000348,
            xvmaddmsp = 0xF0000248,
            xvmaxdp = 0xF0000700,
            xvmaxsp = 0xF0000600,
            xvmindp = 0xF0000740,
            xvminsp = 0xF0000640,
            xvmsubadp = 0xF0000388,
            xvmsubasp = 0xF0000288,
            xvmsubmdp = 0xF00003C8,
            xvmsubmsp = 0xF00002C8,
            xvmuldp = 0xF0000380,
            xvmulsp = 0xF0000280,
            xvnmaddadp = 0xF0000708,
            xvnmaddasp = 0xF0000608,
            xvnmaddmdp = 0xF0000748,
            xvnmaddmsp = 0xF0000648,
            xvnmsubadp = 0xF0000788,
            xvnmsubasp = 0xF0000688,
            xvnmsubmdp = 0xF00007C8,
            xvnmsubmsp = 0xF00006C8,
            xvredp = 0xF0000368,
            xvsubdp = 0xF0000340,
            xvsubsp = 0xF0000240,
            xvtdivdp = 0xF00003E8,
            xvtdivsp = 0xF00002E8,
            xxland = 0xF0000410,
            xxlandc = 0xF0000450,
            xxleqv = 0xF00005D0,
            xxlnand = 0xF0000590,
            xxlnor = 0xF0000510,
            xxlor = 0xF0000490,
            xxlorc = 0xF0000550,
            xxlxor = 0xF00004D0,
            xxmrghw = 0xF0000090,
            xxmrglw = 0xF0000190,
            xxpermdi = 0xF0000050,
            xxsldwi = 0xF0000010,
            xxspltw = 0xF0000290,
        }

        #[allow(clippy::enum_variant_names)]
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        #[repr(u32)]
        pub enum Z23_OPCODE {
            dqua = 0xEC000006,
            dquai = 0xEC000086,
            dquaiq = 0xFC000086,
            dquaq = 0xFC000006,
            drintn = 0xEC0001C6,
            drintnq = 0xFC0001C6,
            drintx = 0xEC0000C6,
            drintxq = 0xFC0000C6,
            drrnd = 0xEC000046,
            drrndq = 0xFC000046,
        }

        #[allow(clippy::enum_variant_names)]
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        #[repr(u32)]
        pub enum Z22_OPCODE {
            dscli = 0xEC000084,
            dscliq = 0xFC000084,
            dscri = 0xEC0000C4,
            dscriq = 0xFC0000C4,
            dtstdc = 0xEC000184,
            dtstdcq = 0xFC000184,
            dtstdg = 0xEC0001C4,
            dtstdgq = 0xFC0001C4,
        }

        #[allow(clippy::enum_variant_names)]
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        #[repr(u32)]
        pub enum XX2_OPCODE_VECTOR_A_FORM {
            xvabsdp = 0xF0000764,
            xvnegdp = 0xF00007E4,
            xvsqrtdp = 0xF000032C,
            xvabssp = 0xF0000664,
            xvnegsp = 0xF00006E4,
            xvresp = 0xF0000268,
            xvrsqrtesp = 0xF0000228,
            xvsqrtsp = 0xF000022C,
            xvcvspsxws = 0xF0000260,
            xvcvspuxws = 0xF0000220,
            xvcvsxwsp = 0xF00002E0,
            xvcvuxwsp = 0xF00002A0,
            xvrdpip = 0xF00003A4,
            xvrdpim = 0xF00003E4,
            xvrdpiz = 0xF0000364,
            xvrdpi = 0xF0000324,
            xvrspip = 0xF00002A4,
            xvrspim = 0xF00002E4,
            xvrspiz = 0xF0000264,
            xvrspi = 0xF0000224,
            xvcvsxddp = 0xF00007E0,
            xvcvuxddp = 0xF00007A0,
            xvcvspdp = 0xF0000724,
            xvcvdpsp = 0xF0000624,
            xvcvdpsxws = 0xF0000360,
            xvcvdpuxws = 0xF0000320,
        }

        #[allow(clippy::enum_variant_names)]
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        #[repr(u32)]
        pub enum XX2_OPCODE_SCALAR_A_FORM {
            xscvdpspn = 0xF000042C,
            xscvspdpn = 0xF000052C,
        }

        #[allow(clippy::enum_variant_names)]
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        #[repr(u32)]
        pub enum XX2_OPCODE_B_FORM {
            xxbrq = 0xF01F076C,
            xxbrd = 0xF017076C,
            xxbrw = 0xF00F076C,
            xxbrh = 0xF007076C,
        }

        #[allow(clippy::enum_variant_names)]
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        #[repr(u32)]
        pub enum XX2_OPCODE_UNUSED {
            xssqrtdp = 0xF000012C,
            xsresp = 0xF0000068,
            xsrsqrtesp = 0xF0000028,
            xssqrtsp = 0xF000002C,
            xsabsdp = 0xF0000564,
            xscvdpsp = 0xF0000424,
            xscvdpsxds = 0xF0000560,
            xscvdpsxws = 0xF0000160,
            xscvdpuxds = 0xF0000520,
            xscvdpuxws = 0xF0000120,
            xscvspdp = 0xF0000524,
            xscvsxddp = 0xF00005E0,
            xscvsxdsp = 0xF00004E0,
            xscvuxddp = 0xF00005A0,
            xscvuxdsp = 0xF00004A0,
            xsnabsdp = 0xF00005A4,
            xsnegdp = 0xF00005E4,
            xsrdpi = 0xF0000124,
            xsrdpic = 0xF00001AC,
            xsrdpim = 0xF00001E4,
            xsrdpip = 0xF00001A4,
            xsrdpiz = 0xF0000164,
            xsrsp = 0xF0000464,
            xsrsqrtedp = 0xF0000128,
            xstsqrtdp = 0xF00001A8,
            xvcvdpsxds = 0xF0000760,
            xvcvdpuxds = 0xF0000720,
            xvcvspsxds = 0xF0000660,
            xvcvspuxds = 0xF0000620,
            xvcvsxdsp = 0xF00006E0,
            xvcvsxwdp = 0xF00003E0,
            xvcvuxdsp = 0xF00006A0,
            xvcvuxwdp = 0xF00003A0,
            xvnabsdp = 0xF00007A4,
            xvnabssp = 0xF00006A4,
            xvrdpic = 0xF00003AC,
            xvrspic = 0xF00002AC,
            xvrsqrtedp = 0xF0000328,
            xvtsqrtdp = 0xF00003A8,
            xvtsqrtsp = 0xF00002A8,
            xxspltib = 0xF00002D0,
        }

        #[allow(clippy::enum_variant_names)]
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        #[repr(u32)]
        pub enum EVX_OPCODE {
            evlddepx = 0x7C00063E,
            evstddepx = 0x7C00073E,
            brinc = 0x1000020F,
            evabs = 0x10000208,
            evaddiw = 0x10000202,
            evaddsmiaaw = 0x100004C9,
            evaddssiaaw = 0x100004C1,
            evaddumiaaw = 0x100004C8,
            evaddusiaaw = 0x100004C0,
            evaddw = 0x10000200,
            evand = 0x10000211,
            evandc = 0x10000212,
            evcmpeq = 0x10000234,
            evcmpgts = 0x10000231,
            evcmpgtu = 0x10000230,
            evcmplts = 0x10000233,
            evcmpltu = 0x10000232,
            evcntlsw = 0x1000020E,
            evcntlzw = 0x1000020D,
            evdivws = 0x100004C6,
            evdivwu = 0x100004C7,
            eveqv = 0x10000219,
            evextsb = 0x1000020A,
            evextsh = 0x1000020B,
            evldd = 0x10000301,
            evlddx = 0x10000300,
            evldh = 0x10000305,
            evldhx = 0x10000304,
            evldw = 0x10000303,
            evldwx = 0x10000302,
            evlhhesplat = 0x10000309,
            evlhhesplatx = 0x10000308,
            evlhhossplat = 0x1000030F,
            evlhhossplatx = 0x1000030E,
            evlhhousplat = 0x1000030D,
            evlhhousplatx = 0x1000030C,
            evlwhe = 0x10000311,
            evlwhos = 0x10000317,
            evlwhosx = 0x10000316,
            evlwhou = 0x10000315,
            evlwhoux = 0x10000314,
            evlwhsplat = 0x1000031D,
            evlwhsplatx = 0x1000031C,
            evlwwsplat = 0x10000319,
            evlwwsplatx = 0x10000318,
            evmergehi = 0x1000022C,
            evmergehilo = 0x1000022E,
            evmergelo = 0x1000022D,
            evmergelohi = 0x1000022F,
            evmhegsmfaa = 0x1000052B,
            evmhegsmfan = 0x100005AB,
            evmhegsmiaa = 0x10000529,
            evmhegsmian = 0x100005A9,
            evmhegumiaa = 0x10000528,
            evmhegumian = 0x100005A8,
            evmhesmf = 0x1000040B,
            evmhesmfa = 0x1000042B,
            evmhesmfaaw = 0x1000050B,
            evmhesmfanw = 0x1000058B,
            evmhesmi = 0x10000409,
            evmhesmia = 0x10000429,
            evmhesmiaaw = 0x10000509,
            evmhesmianw = 0x10000589,
            evmhessf = 0x10000403,
            evmhessfa = 0x10000423,
            evmhessfaaw = 0x10000503,
            evmhessfanw = 0x10000583,
            evmhessiaaw = 0x10000501,
            evmhessianw = 0x10000581,
            evmheumi = 0x10000408,
            evmheumia = 0x10000428,
            evmheumiaaw = 0x10000508,
            evmheumianw = 0x10000588,
            evmheusiaaw = 0x10000500,
            evmheusianw = 0x10000580,
            evmhogsmfaa = 0x1000052F,
            evmhogsmfan = 0x100005AF,
            evmhogsmiaa = 0x1000052D,
            evmhogsmian = 0x100005AD,
            evmhogumiaa = 0x1000052C,
            evmhogumian = 0x100005AC,
            evmhosmf = 0x1000040F,
            evmhosmfa = 0x1000042F,
            evmhosmfaaw = 0x1000050F,
            evmhosmfanw = 0x1000058F,
            evmhosmi = 0x1000040D,
            evmhosmia = 0x1000042D,
            evmhosmiaaw = 0x1000050D,
            evmhosmianw = 0x1000058D,
            evmhossf = 0x10000407,
            evmhossfa = 0x10000427,
            evmhossfaaw = 0x10000507,
            evmhossfanw = 0x10000587,
            evmhossiaaw = 0x10000505,
            evmhossianw = 0x10000585,
            evmhoumi = 0x1000040C,
            evmhoumia = 0x1000042C,
            evmhoumiaaw = 0x1000050C,
            evmhoumianw = 0x1000058C,
            evmhousiaaw = 0x10000504,
            evmhousianw = 0x10000584,
            evmra = 0x100004C4,
            evmwhsmf = 0x1000044F,
            evmwhsmfa = 0x1000046F,
            evmwhsmi = 0x1000044D,
            evmwhsmia = 0x1000046D,
            evmwhssf = 0x10