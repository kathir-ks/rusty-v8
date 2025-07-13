// Converted from V8 C++ source files:
// Header: register-arm.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod register_arm {
    use std::fmt;
    use std::marker::Copy;
    use std::ops::Not;

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum AliasingKind {
        kCombine,
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum RegisterCode {
        kRegCode_r0,
        kRegCode_r1,
        kRegCode_r2,
        kRegCode_r3,
        kRegCode_r4,
        kRegCode_r5,
        kRegCode_r6,
        kRegCode_r7,
        kRegCode_r8,
        kRegCode_r9,
        kRegCode_r10,
        kRegCode_fp,
        kRegCode_ip,
        kRegCode_sp,
        kRegCode_lr,
        kRegCode_pc,
        kRegAfterLast,
    }

    impl fmt::Display for RegisterCode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub struct Register {
        code: i32,
    }

    impl Register {
        pub const fn from_code(code: RegisterCode) -> Self {
            Register { code: code as i32 }
        }

        pub const fn code(&self) -> i32 {
            self.code
        }

        pub const fn is_valid(&self) -> bool {
            self.code >= RegisterCode::kRegCode_r0 as i32 && self.code < RegisterCode::kRegAfterLast as i32
        }

        pub const fn no_reg() -> Self {
            Register { code: -1 }
        }

        pub const fn is(&self, other: &Self) -> bool {
            self.code == other.code
        }

        pub const fn eq(&self, other: &Self) -> bool {
            self.code == other.code
        }
    }

    impl fmt::Display for Register {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self.code {
                x if x == Register::from_code(RegisterCode::kRegCode_r0).code() => write!(f, "r0"),
                x if x == Register::from_code(RegisterCode::kRegCode_r1).code() => write!(f, "r1"),
                x if x == Register::from_code(RegisterCode::kRegCode_r2).code() => write!(f, "r2"),
                x if x == Register::from_code(RegisterCode::kRegCode_r3).code() => write!(f, "r3"),
                x if x == Register::from_code(RegisterCode::kRegCode_r4).code() => write!(f, "r4"),
                x if x == Register::from_code(RegisterCode::kRegCode_r5).code() => write!(f, "r5"),
                x if x == Register::from_code(RegisterCode::kRegCode_r6).code() => write!(f, "r6"),
                x if x == Register::from_code(RegisterCode::kRegCode_r7).code() => write!(f, "r7"),
                x if x == Register::from_code(RegisterCode::kRegCode_r8).code() => write!(f, "r8"),
                x if x == Register::from_code(RegisterCode::kRegCode_r9).code() => write!(f, "r9"),
                x if x == Register::from_code(RegisterCode::kRegCode_r10).code() => write!(f, "r10"),
                x if x == Register::from_code(RegisterCode::kRegCode_fp).code() => write!(f, "fp"),
                x if x == Register::from_code(RegisterCode::kRegCode_ip).code() => write!(f, "ip"),
                x if x == Register::from_code(RegisterCode::kRegCode_sp).code() => write!(f, "sp"),
                x if x == Register::from_code(RegisterCode::kRegCode_lr).code() => write!(f, "lr"),
                x if x == Register::from_code(RegisterCode::kRegCode_pc).code() => write!(f, "pc"),
                _ => write!(f, "UnknownRegister"),
            }
        }
    }

    pub fn reassign_register(source: &mut Register) -> Register {
        let result = *source;
        *source = Register::no_reg();
        result
    }

    macro_rules! declare_register {
        ($r:ident) => {
            pub const $r: Register = Register::from_code(RegisterCode::kRegCode_$r);
        };
    }

    declare_register!(r0);
    declare_register!(r1);
    declare_register!(r2);
    declare_register!(r3);
    declare_register!(r4);
    declare_register!(r5);
    declare_register!(r6);
    declare_register!(r7);
    declare_register!(r8);
    declare_register!(r9);
    declare_register!(r10);
    declare_register!(fp);
    declare_register!(ip);
    declare_register!(sp);
    declare_register!(lr);
    declare_register!(pc);

    pub const no_reg: Register = Register::no_reg();

    pub const K_CARG_REGS: [Register; 4] = [r0, r1, r2, r3];
    pub const K_REGISTER_PASSED_ARGUMENTS: usize = K_CARG_REGS.len();
    pub const K_DOUBLE_REGISTER_PASSED_ARGUMENTS: usize = 8;

    pub const fn argument_padding_slots(_argument_count: i32) -> i32 {
        0
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum SwVfpRegisterCode {
        kSwVfpCode_s0,
        kSwVfpCode_s1,
        kSwVfpCode_s2,
        kSwVfpCode_s3,
        kSwVfpCode_s4,
        kSwVfpCode_s5,
        kSwVfpCode_s6,
        kSwVfpCode_s7,
        kSwVfpCode_s8,
        kSwVfpCode_s9,
        kSwVfpCode_s10,
        kSwVfpCode_s11,
        kSwVfpCode_s12,
        kSwVfpCode_s13,
        kSwVfpCode_s14,
        kSwVfpCode_s15,
        kSwVfpCode_s16,
        kSwVfpCode_s17,
        kSwVfpCode_s18,
        kSwVfpCode_s19,
        kSwVfpCode_s20,
        kSwVfpCode_s21,
        kSwVfpCode_s22,
        kSwVfpCode_s23,
        kSwVfpCode_s24,
        kSwVfpCode_s25,
        kSwVfpCode_s26,
        kSwVfpCode_s27,
        kSwVfpCode_s28,
        kSwVfpCode_s29,
        kSwVfpCode_s30,
        kSwVfpCode_s31,
        kSwVfpAfterLast,
    }

    impl fmt::Display for SwVfpRegisterCode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    pub type VfpRegList = u64;

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub struct SwVfpRegister {
        code: i32,
    }

    impl SwVfpRegister {
        pub const K_SIZE_IN_BYTES: i32 = 4;

        pub const fn from_code(code: SwVfpRegisterCode) -> Self {
            SwVfpRegister { code: code as i32 }
        }

        pub const fn code(&self) -> i32 {
            self.code
        }

        pub fn split_code(&self, vm: &mut i32, m: &mut i32) {
            Self::split_code_static(self.code(), vm, m);
        }

        pub fn split_code_static(reg_code: i32, vm: &mut i32, m: &mut i32) {
            assert!(Self::from_code_static(reg_code).is_valid());
            *m = reg_code & 0x1;
            *vm = reg_code >> 1;
        }

        pub const fn from_code_static(code: i32) -> Self {
            SwVfpRegister { code }
        }

        pub const fn is_valid(&self) -> bool {
             self.code >= SwVfpRegisterCode::kSwVfpCode_s0 as i32 && self.code < SwVfpRegisterCode::kSwVfpAfterLast as i32
        }

        pub fn to_vfp_reg_list(&self) -> VfpRegList {
            1u64 << self.code()
        }
    }

    impl fmt::Display for SwVfpRegister {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self.code {
                x if x == SwVfpRegister::from_code_static(SwVfpRegisterCode::kSwVfpCode_s0 as i32).code() => write!(f, "s0"),
                x if x == SwVfpRegister::from_code_static(SwVfpRegisterCode::kSwVfpCode_s1 as i32).code() => write!(f, "s1"),
                x if x == SwVfpRegister::from_code_static(SwVfpRegisterCode::kSwVfpCode_s2 as i32).code() => write!(f, "s2"),
                x if x == SwVfpRegister::from_code_static(SwVfpRegisterCode::kSwVfpCode_s3 as i32).code() => write!(f, "s3"),
                x if x == SwVfpRegister::from_code_static(SwVfpRegisterCode::kSwVfpCode_s4 as i32).code() => write!(f, "s4"),
                x if x == SwVfpRegister::from_code_static(SwVfpRegisterCode::kSwVfpCode_s5 as i32).code() => write!(f, "s5"),
                x if x == SwVfpRegister::from_code_static(SwVfpRegisterCode::kSwVfpCode_s6 as i32).code() => write!(f, "s6"),
                x if x == SwVfpRegister::from_code_static(SwVfpRegisterCode::kSwVfpCode_s7 as i32).code() => write!(f, "s7"),
                x if x == SwVfpRegister::from_code_static(SwVfpRegisterCode::kSwVfpCode_s8 as i32).code() => write!(f, "s8"),
                x if x == SwVfpRegister::from_code_static(SwVfpRegisterCode::kSwVfpCode_s9 as i32).code() => write!(f, "s9"),
                x if x == SwVfpRegister::from_code_static(SwVfpRegisterCode::kSwVfpCode_s10 as i32).code() => write!(f, "s10"),
                x if x == SwVfpRegister::from_code_static(SwVfpRegisterCode::kSwVfpCode_s11 as i32).code() => write!(f, "s11"),
                x if x == SwVfpRegister::from_code_static(SwVfpRegisterCode::kSwVfpCode_s12 as i32).code() => write!(f, "s12"),
                x if x == SwVfpRegister::from_code_static(SwVfpRegisterCode::kSwVfpCode_s13 as i32).code() => write!(f, "s13"),
                x if x == SwVfpRegister::from_code_static(SwVfpRegisterCode::kSwVfpCode_s14 as i32).code() => write!(f, "s14"),
                x if x == SwVfpRegister::from_code_static(SwVfpRegisterCode::kSwVfpCode_s15 as i32).code() => write!(f, "s15"),
                x if x == SwVfpRegister::from_code_static(SwVfpRegisterCode::kSwVfpCode_s16 as i32).code() => write!(f, "s16"),
                x if x == SwVfpRegister::from_code_static(SwVfpRegisterCode::kSwVfpCode_s17 as i32).code() => write!(f, "s17"),
                x if x == SwVfpRegister::from_code_static(SwVfpRegisterCode::kSwVfpCode_s18 as i32).code() => write!(f, "s18"),
                x if x == SwVfpRegister::from_code_static(SwVfpRegisterCode::kSwVfpCode_s19 as i32).code() => write!(f, "s19"),
                x if x == SwVfpRegister::from_code_static(SwVfpRegisterCode::kSwVfpCode_s20 as i32).code() => write!(f, "s20"),
                x if x == SwVfpRegister::from_code_static(SwVfpRegisterCode::kSwVfpCode_s21 as i32).code() => write!(f, "s21"),
                x if x == SwVfpRegister::from_code_static(SwVfpRegisterCode::kSwVfpCode_s22 as i32).code() => write!(f, "s22"),
                x if x == SwVfpRegister::from_code_static(SwVfpRegisterCode::kSwVfpCode_s23 as i32).code() => write!(f, "s23"),
                x if x == SwVfpRegister::from_code_static(SwVfpRegisterCode::kSwVfpCode_s24 as i32).code() => write!(f, "s24"),
                x if x == SwVfpRegister::from_code_static(SwVfpRegisterCode::kSwVfpCode_s25 as i32).code() => write!(f, "s25"),
                x if x == SwVfpRegister::from_code_static(SwVfpRegisterCode::kSwVfpCode_s26 as i32).code() => write!(f, "s26"),
                x if x == SwVfpRegister::from_code_static(SwVfpRegisterCode::kSwVfpCode_s27 as i32).code() => write!(f, "s27"),
                x if x == SwVfpRegister::from_code_static(SwVfpRegisterCode::kSwVfpCode_s28 as i32).code() => write!(f, "s28"),
                x if x == SwVfpRegister::from_code_static(SwVfpRegisterCode::kSwVfpCode_s29 as i32).code() => write!(f, "s29"),
                x if x == SwVfpRegister::from_code_static(SwVfpRegisterCode::kSwVfpCode_s30 as i32).code() => write!(f, "s30"),
                x if x == SwVfpRegister::from_code_static(SwVfpRegisterCode::kSwVfpCode_s31 as i32).code() => write!(f, "s31"),
                _ => write!(f, "UnknownSwVfpRegister"),
            }
        }
    }

    pub type FloatRegister = SwVfpRegister;

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum DoubleRegisterCode {
        kDoubleCode_d0,
        kDoubleCode_d1,
        kDoubleCode_d2,
        kDoubleCode_d3,
        kDoubleCode_d4,
        kDoubleCode_d5,
        kDoubleCode_d6,
        kDoubleCode_d7,
        kDoubleCode_d8,
        kDoubleCode_d9,
        kDoubleCode_d10,
        kDoubleCode_d11,
        kDoubleCode_d12,
        kDoubleCode_d13,
        kDoubleCode_d14,
        kDoubleCode_d15,
        kDoubleCode_d16,
        kDoubleCode_d17,
        kDoubleCode_d18,
        kDoubleCode_d19,
        kDoubleCode_d20,
        kDoubleCode_d21,
        kDoubleCode_d22,
        kDoubleCode_d23,
        kDoubleCode_d24,
        kDoubleCode_d25,
        kDoubleCode_d26,
        kDoubleCode_d27,
        kDoubleCode_d28,
        kDoubleCode_d29,
        kDoubleCode_d30,
        kDoubleCode_d31,
        kDoubleAfterLast,
    }

    impl fmt::Display for DoubleRegisterCode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub struct DwVfpRegister {
        code: i32,
    }

    impl DwVfpRegister {
        pub const K_SIZE_IN_BYTES: i32 = 8;

        pub const fn from_code(code: DoubleRegisterCode) -> Self {
            DwVfpRegister { code: code as i32 }
        }

        pub const fn code(&self) -> i32 {
            self.code
        }

        pub fn split_code(&self, vm: &mut i32, m: &mut i32) {
            Self::split_code_static(self.code(), vm, m);
        }

        pub fn split_code_static(reg_code: i32, vm: &mut i32, m: &mut i32) {
            assert!(Self::from_code_static(reg_code).is_valid());
            *m = (reg_code & 0x10) >> 4;
            *vm = reg_code & 0x0F;
        }

        pub const fn from_code_static(code: i32) -> Self {
            DwVfpRegister { code }
        }

        pub const fn is_valid(&self) -> bool {
            self.code >= DoubleRegisterCode::kDoubleCode_d0 as i32 && self.code < DoubleRegisterCode::kDoubleAfterLast as i32
        }

        pub fn to_vfp_reg_list(&self) -> VfpRegList {
            3u64 << (self.code() * 2)
        }

        pub fn supported_register_count() -> i32 {
            32 // Assuming full support for all 32 double registers
        }

        pub const fn no_reg() -> Self {
            DwVfpRegister { code: -1 }
        }
    }

    impl fmt::Display for DwVfpRegister {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self.code {
                 x if x == DwVfpRegister::from_code_static(DoubleRegisterCode::kDoubleCode_d0 as i32).code() => write!(f, "d0"),
                 x if x == DwVfpRegister::from_code_static(DoubleRegisterCode::kDoubleCode_d1 as i32).code() => write!(f, "d1"),
                 x if x == DwVfpRegister::from_code_static(DoubleRegisterCode::kDoubleCode_d2 as i32).code() => write!(f, "d2"),
                 x if x == DwVfpRegister::from_code_static(DoubleRegisterCode::kDoubleCode_d3 as i32).code() => write!(f, "d3"),
                 x if x == DwVfpRegister::from_code_static(DoubleRegisterCode::kDoubleCode_d4 as i32).code() => write!(f, "d4"),
                 x if x == DwVfpRegister::from_code_static(DoubleRegisterCode::kDoubleCode_d5 as i32).code() => write!(f, "d5"),
                 x if x == DwVfpRegister::from_code_static(DoubleRegisterCode::kDoubleCode_d6 as i32).code() => write!(f, "d6"),
                 x if x == DwVfpRegister::from_code_static(DoubleRegisterCode::kDoubleCode_d7 as i32).code() => write!(f, "d7"),
                 x if x == DwVfpRegister::from_code_static(DoubleRegisterCode::kDoubleCode_d8 as i32).code() => write!(f, "d8"),
                 x if x == DwVfpRegister::from_code_static(DoubleRegisterCode::kDoubleCode_d9 as i32).code() => write!(f, "d9"),
                 x if x == DwVfpRegister::from_code_static(DoubleRegisterCode::kDoubleCode_d10 as i32).code() => write!(f, "d10"),
                 x if x == DwVfpRegister::from_code_static(DoubleRegisterCode::kDoubleCode_d11 as i32).code() => write!(f, "d11"),
                 x if x == DwVfpRegister::from_code_static(DoubleRegisterCode::kDoubleCode_d12 as i32).code() => write!(f, "d12"),
                 x if x == DwVfpRegister::from_code_static(DoubleRegisterCode::kDoubleCode_d13 as i32).code() => write!(f, "d13"),
                 x if x == DwVfpRegister::from_code_static(DoubleRegisterCode::kDoubleCode_d14 as i32).code() => write!(f, "d14"),
                 x if x == DwVfpRegister::from_code_static(DoubleRegisterCode::kDoubleCode_d15 as i32).code() => write!(f, "d15"),
                 x if x == DwVfpRegister::from_code_static(DoubleRegisterCode::kDoubleCode_d16 as i32).code() => write!(f, "d16"),
                 x if x == DwVfpRegister::from_code_static(DoubleRegisterCode::kDoubleCode_d17 as i32).code() => write!(f, "d17"),
                 x if x == DwVfpRegister::from_code_static(DoubleRegisterCode::kDoubleCode_d18 as i32).code() => write!(f, "d18"),
                 x if x == DwVfpRegister::from_code_static(DoubleRegisterCode::kDoubleCode_d19 as i32).code() => write!(f, "d19"),
                 x if x == DwVfpRegister::from_code_static(DoubleRegisterCode::kDoubleCode_d20 as i32).code() => write!(f, "d20"),
                 x if x == DwVfpRegister::from_code_static(DoubleRegisterCode::kDoubleCode_d21 as i32).code() => write!(f, "d21"),
                 x if x == DwVfpRegister::from_code_static(DoubleRegisterCode::kDoubleCode_d22 as i32).code() => write!(f, "d22"),
                 x if x == DwVfpRegister::from_code_static(DoubleRegisterCode::kDoubleCode_d23 as i32).code() => write!(f, "d23"),
                 x if x == DwVfpRegister::from_code_static(DoubleRegisterCode::kDoubleCode_d24 as i32).code() => write!(f, "d24"),
                 x if x == DwVfpRegister::from_code_static(DoubleRegisterCode::kDoubleCode_d25 as i32).code() => write!(f, "d25"),
                 x if x == DwVfpRegister::from_code_static(DoubleRegisterCode::kDoubleCode_d26 as i32).code() => write!(f, "d26"),
                 x if x == DwVfpRegister::from_code_static(DoubleRegisterCode::kDoubleCode_d27 as i32).code() => write!(f, "d27"),
                 x if x == DwVfpRegister::from_code_static(DoubleRegisterCode::kDoubleCode_d28 as i32).code() => write!(f, "d28"),
                 x if x == DwVfpRegister::from_code_static(DoubleRegisterCode::kDoubleCode_d29 as i32).code() => write!(f, "d29"),
                 x if x == DwVfpRegister::from_code_static(DoubleRegisterCode::kDoubleCode_d30 as i32).code() => write!(f, "d30"),
                 x if x == DwVfpRegister::from_code_static(DoubleRegisterCode::kDoubleCode_d31 as i32).code() => write!(f, "d31"),
                _ => write!(f, "UnknownDwVfpRegister"),
            }
        }
    }

    pub type DoubleRegister = DwVfpRegister;

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub struct LowDwVfpRegister {
        code: i32,
    }

    impl LowDwVfpRegister {
        pub const fn from_code(code: DoubleRegisterCode) -> Self {
            LowDwVfpRegister { code: code as i32 }
        }

        pub const fn code(&self) -> i32 {
            self.code
        }

        pub const fn into_dwvfp_register(self) -> DwVfpRegister {
            DwVfpRegister { code: self.code }
        }

        pub const fn low(&self) -> SwVfpRegister {
            SwVfpRegister::from_code_static((self.code() * 2) as i32)
        }

        pub const fn high(&self) -> SwVfpRegister {
            SwVfpRegister::from_code_static((self.code() * 2 + 1) as i32)
        }

        pub fn to_vfp_reg_list(&self) -> VfpRegList {
            3u64 << (self.code() * 2)
        }
    }

    impl fmt::Display for LowDwVfpRegister {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
             match self.code {
                 x if x == LowDwVfpRegister::from_code(DoubleRegisterCode::kDoubleCode_d0).code() => write!(f, "d0"),
                 x if x == LowDwVfpRegister::from_code(DoubleRegisterCode::kDoubleCode_d1).code() => write!(f, "d1"),
                 x if x == LowDwVfpRegister::from_code(DoubleRegisterCode::kDoubleCode_d2).code() => write!(f, "d2"),
                 x if x == LowDwVfpRegister::from_code(DoubleRegisterCode::kDoubleCode_d3).code() => write!(f, "d3"),
                 x if x == LowDwVfpRegister::from_code(DoubleRegisterCode::kDoubleCode_d4).code() => write!(f, "d4"),
                 x if x == LowDwVfpRegister::from_code(DoubleRegisterCode::kDoubleCode_d5).code() => write!(f, "d5"),
                 x if x == LowDwVfpRegister::from_code(DoubleRegisterCode::kDoubleCode_d6).code() => write!(f, "d6"),
                 x if x == LowDwVfpRegister::from_code(DoubleRegisterCode::kDoubleCode_d7).code() => write!(f, "d7"),
                 x if x == LowDwVfpRegister::from_code(DoubleRegisterCode::kDoubleCode_d8).code() => write!(f, "d8"),
                 x if x == LowDwVfpRegister::from_code(DoubleRegisterCode::kDoubleCode_d9).code() => write!(f, "d9"),
                 x if x == LowDwVfpRegister::from_code(DoubleRegisterCode::kDoubleCode_d10).code() => write!(f, "d10"),
                 x if x == LowDwVfpRegister::from_code(DoubleRegisterCode::kDoubleCode_d11).code() => write!(f, "d11"),
                 x if x == LowDwVfpRegister::from_code(DoubleRegisterCode::kDoubleCode_d12).code() => write!(f, "d12"),
                 x if x == LowDwVfpRegister::from_code(DoubleRegisterCode::kDoubleCode_d13).code() => write!(f, "d13"),
                 x if x == LowDwVfpRegister::from_code(DoubleRegisterCode::kDoubleCode_d14).code() => write!(f, "d14"),
                 x if x == LowDwVfpRegister::from_code(DoubleRegisterCode::kDoubleCode_d15).code() => write!(f, "d15"),
                 _ => write!(f, "UnknownLowDwVfpRegister"),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum Simd128RegisterCode {
        kSimd128Code_q0,
        kSimd128Code_q1,
        kSimd128Code_q2,
        kSimd128Code_q3,
        kSimd128Code_q4,
        kSimd128Code_q5,
        kSimd128Code_q6,
        kSimd128Code_q7,
        kSimd128Code_q8,
        kSimd128Code_q9,
        kSimd128Code_q10,
        kSimd128Code_q11,
        kSimd128Code_q12,
        kSimd128Code_q13,
        kSimd128Code_q14,
        kSimd128Code_q15,
        kSimd128AfterLast,
    }

    impl fmt::Display for Simd128RegisterCode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub struct QwNeonRegister {
        code: i32,
    }

    impl QwNeonRegister {
        pub const fn from_code(code: Simd128RegisterCode) -> Self {
            QwNeonRegister { code: code as i32 }
        }

        pub const fn code(&self) -> i32 {
            self.code
        }

        pub fn split_code(&self, vm: &mut i32, m: &mut i32) {
            Self::split_code_static(self.code(), vm, m);
        }

        pub fn split_code_static(reg_code: i32, vm: &mut i32, m: &mut i32) {
            assert!(reg_code >= 0 && reg_code < Self::k_num_registers());
            let encoded_code = reg_code << 1;
            *m = (encoded_code & 0x10) >> 4;
            *vm = encoded_code & 0x0F;
        }

        pub const fn low(&self) -> DwVfpRegister {
            DwVfpRegister::from_code_static((self.code() * 2) as i32)
        }

        pub const fn high(&self) -> DwVfpRegister {
            DwVfpRegister::from_code_static((self.code() * 2 + 1) as i32)
        }

        pub fn to_vfp_reg_list(&self) -> VfpRegList {
            0xf << (self.code() * 4)
        }

        const fn k_num_registers() -> i32 {
            16
        }
    }

    impl fmt::Display for QwNeonRegister {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
           match self.code {
                 x if x == QwNeonRegister::from_code(Simd128RegisterCode::kSimd128Code_q0).code() => write!(f, "q0"),
                 x if x == QwNeonRegister::from_code(Simd128RegisterCode::kSimd128Code_q1).code() => write!(f, "q1"),
                 x if x == QwNeonRegister::from_code(Simd128RegisterCode::kSimd128Code_q2).code() => write!(f, "q2"),
                 x if x == QwNeonRegister::from_code(Simd128RegisterCode::kSimd128Code_q3).code() => write!(f, "q3"),
                 x if x == Q
