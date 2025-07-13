// Converted from V8 C++ source files:
// Header: register-mips64.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod constants_mips64 {
    pub const kInvalidFPUControlRegister: i32 = -1;
    pub const kFCSRRegister: i32 = 31;
    pub const kInvalidMSAControlRegister: i32 = -1;
    pub const kMSAIRRegister: i32 = 62;
    pub const kMSACSRRegister: i32 = 63;
}

use std::sync::Arc;

pub enum AliasingKind {
    kNoAlias,
    kMayAlias,
    kOverlap,
}

pub struct RegisterBase<T, const N: usize> {
    code: i32,
}

impl<T, const N: usize> RegisterBase<T, const NN: usize> {
    const fn new(code: i32) -> Self {
        Self { code }
    }

    fn code(&self) -> i32 {
        self.code
    }

    fn from_code(code: i32) -> T {
        // Assuming T has a constructor that takes an i32.
        // This is a placeholder; the actual implementation depends on the
        // specific type T.
        unsafe { std::mem::transmute::<i32, T>(code) }
    }

    fn no_reg() -> T {
        // Assuming T has a default or a way to represent "no register".
        // This is a placeholder; the actual implementation depends on the
        // specific type T.
        unsafe { std::mem::transmute::<i32, T>(-1) }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum RegisterCode {
    kRegCode_zero_reg,
    kRegCode_at,
    kRegCode_v0,
    kRegCode_v1,
    kRegCode_a0,
    kRegCode_a1,
    kRegCode_a2,
    kRegCode_a3,
    kRegCode_a4,
    kRegCode_a5,
    kRegCode_a6,
    kRegCode_a7,
    kRegCode_t0,
    kRegCode_t1,
    kRegCode_t2,
    kRegCode_t3,
    kRegCode_s0,
    kRegCode_s1,
    kRegCode_s2,
    kRegCode_s3,
    kRegCode_s4,
    kRegCode_s5,
    kRegCode_s6,
    kRegCode_s7,
    kRegCode_t8,
    kRegCode_t9,
    kRegCode_k0,
    kRegCode_k1,
    kRegCode_gp,
    kRegCode_sp,
    kRegCode_fp,
    kRegCode_ra,
    kRegAfterLast,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Register {
    code: i32,
}

impl Register {
    pub const fn from_code(code: i32) -> Self {
        Self { code }
    }

    pub const fn no_reg() -> Self {
        Self { code: -1 }
    }

    pub fn code(&self) -> i32 {
        self.code
    }

    pub fn is_valid(&self) -> bool {
        self.code >= 0 && self.code < RegisterCode::kRegAfterLast as i32
    }

    pub fn is(&self, other: Register) -> bool {
        self.code == other.code
    }

    pub fn name(&self) -> &'static str {
        match self.code {
            0 => "zero_reg",
            1 => "at",
            2 => "v0",
            3 => "v1",
            4 => "a0",
            5 => "a1",
            6 => "a2",
            7 => "a3",
            8 => "a4",
            9 => "a5",
            10 => "a6",
            11 => "a7",
            12 => "t0",
            13 => "t1",
            14 => "t2",
            15 => "t3",
            16 => "s0",
            17 => "s1",
            18 => "s2",
            19 => "s3",
            20 => "s4",
            21 => "s5",
            22 => "s6",
            23 => "s7",
            24 => "t8",
            25 => "t9",
            26 => "k0",
            27 => "k1",
            28 => "gp",
            29 => "sp",
            30 => "fp",
            31 => "ra",
            _ => "invalid",
        }
    }

    pub const kMantissaOffset: i32 = 0;
    pub const kExponentOffset: i32 = 4;
}

macro_rules! declare_register {
    ($r:ident) => {
        pub const $r: Register = Register::from_code(RegisterCode::kRegCode_$r as i32);
    };
}

declare_register!(zero_reg);
declare_register!(at);
declare_register!(v0);
declare_register!(v1);
declare_register!(a0);
declare_register!(a1);
declare_register!(a2);
declare_register!(a3);
declare_register!(a4);
declare_register!(a5);
declare_register!(a6);
declare_register!(a7);
declare_register!(t0);
declare_register!(t1);
declare_register!(t2);
declare_register!(t3);
declare_register!(s0);
declare_register!(s1);
declare_register!(s2);
declare_register!(s3);
declare_register!(s4);
declare_register!(s5);
declare_register!(s6);
declare_register!(s7);
declare_register!(t8);
declare_register!(t9);
declare_register!(k0);
declare_register!(k1);
declare_register!(gp);
declare_register!(sp);
declare_register!(fp);
declare_register!(ra);

pub fn to_number(reg: Register) -> i32 {
    reg.code()
}

pub fn to_register(num: i32) -> Register {
    Register::from_code(num)
}

pub fn reassign_register(source: &mut Register) -> Register {
    let result = *source;
    *source = Register::no_reg();
    result
}

pub const fn argument_padding_slots(_argument_count: i32) -> i32 {
    0
}

pub const kFPAliasing: AliasingKind = AliasingKind::kOverlap;
pub const kSimdMaskRegisters: bool = false;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum MSARegisterCode {
    kMsaCode_w0,
    kMsaCode_w1,
    kMsaCode_w2,
    kMsaCode_w3,
    kMsaCode_w4,
    kMsaCode_w5,
    kMsaCode_w6,
    kMsaCode_w7,
    kMsaCode_w8,
    kMsaCode_w9,
    kMsaCode_w10,
    kMsaCode_w11,
    kMsaCode_w12,
    kMsaCode_w13,
    kMsaCode_w14,
    kMsaCode_w15,
    kMsaCode_w16,
    kMsaCode_w17,
    kMsaCode_w18,
    kMsaCode_w19,
    kMsaCode_w20,
    kMsaCode_w21,
    kMsaCode_w22,
    kMsaCode_w23,
    kMsaCode_w24,
    kMsaCode_w25,
    kMsaCode_w26,
    kMsaCode_w27,
    kMsaCode_w28,
    kMsaCode_w29,
    kMsaCode_w30,
    kMsaCode_w31,
    kMsaAfterLast,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct MSARegister {
    code: i32,
}

impl MSARegister {
    pub const fn from_code(code: i32) -> Self {
        Self { code }
    }

    pub const fn no_reg() -> Self {
        Self { code: -1 }
    }

    pub fn code(&self) -> i32 {
        self.code
    }

    pub fn is_valid(&self) -> bool {
        self.code >= 0 && self.code < MSARegisterCode::kMsaAfterLast as i32
    }

    pub fn is(&self, other: MSARegister) -> bool {
        self.code == other.code
    }

    pub fn name(&self) -> &'static str {
        match self.code {
             0 => "w0",
             1 => "w1",
             2 => "w2",
             3 => "w3",
             4 => "w4",
             5 => "w5",
             6 => "w6",
             7 => "w7",
             8 => "w8",
             9 => "w9",
            10 => "w10",
            11 => "w11",
            12 => "w12",
            13 => "w13",
            14 => "w14",
            15 => "w15",
            16 => "w16",
            17 => "w17",
            18 => "w18",
            19 => "w19",
            20 => "w20",
            21 => "w21",
            22 => "w22",
            23 => "w23",
            24 => "w24",
            25 => "w25",
            26 => "w26",
            27 => "w27",
            28 => "w28",
            29 => "w29",
            30 => "w30",
            31 => "w31",
            _ => "invalid",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum DoubleRegisterCode {
    kDoubleCode_f0,
    kDoubleCode_f1,
    kDoubleCode_f2,
    kDoubleCode_f3,
    kDoubleCode_f4,
    kDoubleCode_f5,
    kDoubleCode_f6,
    kDoubleCode_f7,
    kDoubleCode_f8,
    kDoubleCode_f9,
    kDoubleCode_f10,
    kDoubleCode_f11,
    kDoubleCode_f12,
    kDoubleCode_f13,
    kDoubleCode_f14,
    kDoubleCode_f15,
    kDoubleCode_f16,
    kDoubleCode_f17,
    kDoubleCode_f18,
    kDoubleCode_f19,
    kDoubleCode_f20,
    kDoubleCode_f21,
    kDoubleCode_f22,
    kDoubleCode_f23,
    kDoubleCode_f24,
    kDoubleCode_f25,
    kDoubleCode_f26,
    kDoubleCode_f27,
    kDoubleCode_f28,
    kDoubleCode_f29,
    kDoubleCode_f30,
    kDoubleCode_f31,
    kDoubleAfterLast,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct FPURegister {
    code: i32,
}

impl FPURegister {
    pub const fn from_code(code: i32) -> Self {
        Self { code }
    }

    pub const fn no_reg() -> Self {
        Self { code: -1 }
    }

    pub fn code(&self) -> i32 {
        self.code
    }

    pub fn is_valid(&self) -> bool {
        self.code >= 0 && self.code < DoubleRegisterCode::kDoubleAfterLast as i32
    }

    pub fn is(&self, other: FPURegister) -> bool {
        self.code == other.code
    }

    pub fn name(&self) -> &'static str {
        match self.code {
             0 => "f0",
             1 => "f1",
             2 => "f2",
             3 => "f3",
             4 => "f4",
             5 => "f5",
             6 => "f6",
             7 => "f7",
             8 => "f8",
             9 => "f9",
            10 => "f10",
            11 => "f11",
            12 => "f12",
            13 => "f13",
            14 => "f14",
            15 => "f15",
            16 => "f16",
            17 => "f17",
            18 => "f18",
            19 => "f19",
            20 => "f20",
            21 => "f21",
            22 => "f22",
            23 => "f23",
            24 => "f24",
            25 => "f25",
            26 => "f26",
            27 => "f27",
            28 => "f28",
            29 => "f29",
            30 => "f30",
            31 => "f31",
            _ => "invalid",
        }
    }

    pub fn low(&self) -> Self {
        assert_eq!(self.code % 2, 0);
        FPURegister::from_code(self.code)
    }

    pub fn high(&self) -> Self {
        assert_eq!(self.code % 2, 0);
        FPURegister::from_code(self.code + 1)
    }

    pub fn to_w(&self) -> MSARegister {
        MSARegister::from_code(self.code)
    }
}

pub type FloatRegister = FPURegister;
pub type DoubleRegister = FPURegister;

macro_rules! declare_double_register {
    ($r:ident) => {
        pub const $r: DoubleRegister = DoubleRegister::from_code(DoubleRegisterCode::kDoubleCode_$r as i32);
    };
}

declare_double_register!(f0);
declare_double_register!(f1);
declare_double_register!(f2);
declare_double_register!(f3);
declare_double_register!(f4);
declare_double_register!(f5);
declare_double_register!(f6);
declare_double_register!(f7);
declare_double_register!(f8);
declare_double_register!(f9);
declare_double_register!(f10);
declare_double_register!(f11);
declare_double_register!(f12);
declare_double_register!(f13);
declare_double_register!(f14);
declare_double_register!(f15);
declare_double_register!(f16);
declare_double_register!(f17);
declare_double_register!(f18);
declare_double_register!(f19);
declare_double_register!(f20);
declare_double_register!(f21);
declare_double_register!(f22);
declare_double_register!(f23);
declare_double_register!(f24);
declare_double_register!(f25);
declare_double_register!(f26);
declare_double_register!(f27);
declare_double_register!(f28);
declare_double_register!(f29);
declare_double_register!(f30);
declare_double_register!(f31);

pub const no_dreg: DoubleRegister = DoubleRegister::no_reg();

pub type Simd128Register = MSARegister;

macro_rules! declare_simd128_register {
    ($r:ident) => {
        pub const $r: Simd128Register = Simd128Register::from_code(MSARegisterCode::kMsaCode_$r as i32);
    };
}

declare_simd128_register!(w0);
declare_simd128_register!(w1);
declare_simd128_register!(w2);
declare_simd128_register!(w3);
declare_simd128_register!(w4);
declare_simd128_register!(w5);
declare_simd128_register!(w6);
declare_simd128_register!(w7);
declare_simd128_register!(w8);
declare_simd128_register!(w9);
declare_simd128_register!(w10);
declare_simd128_register!(w11);
declare_simd128_register!(w12);
declare_simd128_register!(w13);
declare_simd128_register!(w14);
declare_simd128_register!(w15);
declare_simd128_register!(w16);
declare_simd128_register!(w17);
declare_simd128_register!(w18);
declare_simd128_register!(w19);
declare_simd128_register!(w20);
declare_simd128_register!(w21);
declare_simd128_register!(w22);
declare_simd128_register!(w23);
declare_simd128_register!(w24);
declare_simd128_register!(w25);
declare_simd128_register!(w26);
declare_simd128_register!(w27);
declare_simd128_register!(w28);
declare_simd128_register!(w29);
declare_simd128_register!(w30);
declare_simd128_register!(w31);

pub const no_msareg: Simd128Register = Simd128Register::no_reg();

pub const kRootRegister: Register = s6;
pub const cp: Register = s7;
pub const kScratchReg: Register = s3;
pub const kScratchReg2: Register = s4;
pub const kScratchDoubleReg: DoubleRegister = f30;
pub const kScratchDoubleReg2: DoubleRegister = f31;
pub const kDoubleRegZero: DoubleRegister = f28;
pub const kDoubleCompareReg: DoubleRegister = f23;
pub const kSimd128RegZero: Simd128Register = w28;
pub const kSimd128ScratchReg: Simd128Register = w30;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct FPUControlRegister {
    reg_code: i32,
}

impl FPUControlRegister {
    pub fn is_valid(&self) -> bool {
        self.reg_code == constants_mips64::kFCSRRegister
    }

    pub fn is(&self, creg: FPUControlRegister) -> bool {
        self.reg_code == creg.reg_code
    }

    pub fn code(&self) -> i32 {
        assert!(self.is_valid());
        self.reg_code
    }

    pub fn bit(&self) -> i32 {
        assert!(self.is_valid());
        1 << self.reg_code
    }

    pub fn setcode(&mut self, f: i32) {
        self.reg_code = f;
        assert!(self.is_valid());
    }
}

pub const no_fpucreg: FPUControlRegister = FPUControlRegister { reg_code: constants_mips64::kInvalidFPUControlRegister };
pub const FCSR: FPUControlRegister = FPUControlRegister { reg_code: constants_mips64::kFCSRRegister };

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct MSAControlRegister {
    reg_code: i32,
}

impl MSAControlRegister {
    pub fn is_valid(&self) -> bool {
        self.reg_code == constants_mips64::kMSAIRRegister || self.reg_code == constants_mips64::kMSACSRRegister
    }

    pub fn is(&self, creg: MSAControlRegister) -> bool {
        self.reg_code == creg.reg_code
    }

    pub fn code(&self) -> i32 {
        assert!(self.is_valid());
        self.reg_code
    }

    pub fn bit(&self) -> i32 {
        assert!(self.is_valid());
        1 << self.reg_code
    }

    pub fn setcode(&mut self, f: i32) {
        self.reg_code = f;
        assert!(self.is_valid());
    }
}

pub const no_msacreg: MSAControlRegister = MSAControlRegister { reg_code: constants_mips64::kInvalidMSAControlRegister };
pub const MSAIR: MSAControlRegister = MSAControlRegister { reg_code: constants_mips64::kMSAIRRegister };
pub const MSACSR: MSAControlRegister = MSAControlRegister { reg_code: constants_mips64::kMSACSRRegister };

trait RegisterName {
    fn name(&self) -> &'static str;
}

impl RegisterName for Register {
    fn name(&self) -> &'static str {
        match self.code {
            0 => "zero_reg",
            1 => "at",
            2 => "v0",
            3 => "v1",
            4 => "a0",
            5 => "a1",
            6 => "a2",
            7 => "a3",
            8 => "a4",
            9 => "a5",
            10 => "a6",
            11 => "a7",
            12 => "t0",
            13 => "t1",
            14 => "t2",
            15 => "t3",
            16 => "s0",
            17 => "s1",
            18 => "s2",
            19 => "s3",
            20 => "s4",
            21 => "s5",
            22 => "s6",
            23 => "s7",
            24 => "t8",
            25 => "t9",
            26 => "k0",
            27 => "k1",
            28 => "gp",
            29 => "sp",
            30 => "fp",
            31 => "ra",
            _ => "invalid",
        }
    }
}

impl RegisterName for FPURegister {
    fn name(&self) -> &'static str {
        match self.code {
             0 => "f0",
             1 => "f1",
             2 => "f2",
             3 => "f3",
             4 => "f4",
             5 => "f5",
             6 => "f6",
             7 => "f7",
             8 => "f8",
             9 => "f9",
            10 => "f10",
            11 => "f11",
            12 => "f12",
            13 => "f13",
            14 => "f14",
            15 => "f15",
            16 => "f16",
            17 => "f17",
            18 => "f18",
            19 => "f19",
            20 => "f20",
            21 => "f21",
            22 => "f22",
            23 => "f23",
            24 => "f24",
            25 => "f25",
            26 => "f26",
            27 => "f27",
            28 => "f28",
            29 => "f29",
            30 => "f30",
            31 => "f31",
            _ => "invalid",
        }
    }
}

impl RegisterName for MSARegister {
    fn name(&self) -> &'static str {
        match self.code {
             0 => "w0",
             1 => "w1",
             2 => "w2",
             3 => "w3",
             4 => "w4",
             5 => "w5",
             6 => "w6",
             7 => "w7",
             8 => "w8",
             9 => "w9",
            10 => "w10",
            11 => "w11",
            12 => "w12",
            13 => "w13",
            14 => "w14",
            15 => "w15",
            16 => "w16",
            17 => "w17",
            18 => "w18",
            19 => "w19",
            20 => "w20",
            21 => "w21",
            22 => "w22",
            23 => "w23",
            24 => "w24",
            25 => "w25",
            26 => "w26",
            27 => "w27",
            28 => "w28",
            29 => "w29",
            30 => "w30",
            31 => "w31",
            _ => "invalid",
        }
    }
}

pub const kCArgRegs: [Register; 8] = [a0, a1, a2, a3, a4, a5, a6, a7];
pub const kRegisterPassedArguments: usize = kCArgRegs.len();
pub const kFPRegisterPassedArguments: i32 = 8;

pub const kReturnRegister0: Register = v0;
pub const kReturnRegister1: Register = v1;
pub const kReturnRegister2: Register = a0;
pub const kJSFunctionRegister: Register = a1;
pub const kContextRegister: Register = s7;
pub const kAllocateSizeRegister: Register = a0;
pub const kInterpreterAccumulatorRegister: Register = v0;
pub const kInterpreterBytecodeOffsetRegister: Register = t0;
pub const kInterpreterBytecodeArrayRegister: Register = t1;
pub const kInterpreterDispatchTableRegister: Register = t2;
pub const kJavaScriptCallArgCountRegister: Register = a0;
pub const kJavaScriptCallCodeStartRegister: Register = a2;
pub const kJavaScriptCallTargetRegister: Register = kJSFunctionRegister;
pub const kJavaScriptCallNewTargetRegister: Register = a3;
pub const kJavaScriptCallExtraArg1Register: Register = a2;
pub const kJavaScriptCallDispatchHandleRegister: Register = a4;

pub const kRuntimeCallFunctionRegister: Register = a1;
pub const kRuntimeCallArgCountRegister: Register = a0;
pub const kRuntimeCallArgvRegister: Register = a2;
pub const kWasmImplicitArgRegister: Register = a0;
pub const kWasmCompileLazyFuncIndexRegister: Register = t0;

pub const kFPReturnRegister0: DoubleRegister = f0;
