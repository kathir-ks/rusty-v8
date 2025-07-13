// Converted from V8 C++ source files:
// Header: register-ppc.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod register_ppc {
    use std::sync::Arc;

    // From src/codegen/register-base.h
    pub struct RegisterBase<T, const N: usize> {
        code: i32,
    }

    impl<T, const N: usize> RegisterBase<T, N> {
        const fn new(code: i32) -> Self {
            Self { code }
        }

        pub const fn from_code(code: i32) -> Self {
            Self { code }
        }

        pub const fn code(&self) -> i32 {
            self.code
        }

        pub const fn is_valid(&self) -> bool {
            self.code >= 0 && self.code < N as i32
        }

        pub const fn is_no_reg(&self) -> bool {
            self.code == -1
        }

        pub const fn no_reg() -> Self {
            Self { code: -1 }
        }
    }

    // From src/codegen/ppc/register-ppc.h

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum RegisterCode {
        kRegCode_r0,
        kRegCode_sp,
        kRegCode_r2,
        kRegCode_r3,
        kRegCode_r4,
        kRegCode_r5,
        kRegCode_r6,
        kRegCode_r7,
        kRegCode_r8,
        kRegCode_r9,
        kRegCode_r10,
        kRegCode_r11,
        kRegCode_ip,
        kRegCode_r13,
        kRegCode_r14,
        kRegCode_r15,
        kRegCode_r16,
        kRegCode_r17,
        kRegCode_r18,
        kRegCode_r19,
        kRegCode_r20,
        kRegCode_r21,
        kRegCode_r22,
        kRegCode_r23,
        kRegCode_r24,
        kRegCode_r25,
        kRegCode_r26,
        kRegCode_r27,
        kRegCode_r28,
        kRegCode_r29,
        kRegCode_r30,
        kRegCode_fp,
        kRegAfterLast,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct Register {
        code: i32,
    }

    impl Register {
        pub const fn new(code: i32) -> Self {
            Self { code }
        }

        pub const fn from_code(code: i32) -> Self {
            Self { code }
        }

        pub const fn code(&self) -> i32 {
            self.code
        }

        pub const fn is_valid(&self) -> bool {
            self.code >= 0 && self.code < RegisterCode::kRegAfterLast as i32
        }

        pub const fn is_no_reg(&self) -> bool {
            self.code == -1
        }

        pub const fn no_reg() -> Self {
            Self { code: -1 }
        }

        pub const fn r0() -> Self {
            Self { code: RegisterCode::kRegCode_r0 as i32 }
        }

        pub const fn sp() -> Self {
            Self { code: RegisterCode::kRegCode_sp as i32 }
        }

        pub const fn r2() -> Self {
            Self { code: RegisterCode::kRegCode_r2 as i32 }
        }

        pub const fn r3() -> Self {
            Self { code: RegisterCode::kRegCode_r3 as i32 }
        }

        pub const fn r4() -> Self {
            Self { code: RegisterCode::kRegCode_r4 as i32 }
        }

        pub const fn r5() -> Self {
            Self { code: RegisterCode::kRegCode_r5 as i32 }
        }

        pub const fn r6() -> Self {
            Self { code: RegisterCode::kRegCode_r6 as i32 }
        }

        pub const fn r7() -> Self {
            Self { code: RegisterCode::kRegCode_r7 as i32 }
        }

        pub const fn r8() -> Self {
            Self { code: RegisterCode::kRegCode_r8 as i32 }
        }

        pub const fn r9() -> Self {
            Self { code: RegisterCode::kRegCode_r9 as i32 }
        }

        pub const fn r10() -> Self {
            Self { code: RegisterCode::kRegCode_r10 as i32 }
        }

        pub const fn r11() -> Self {
            Self { code: RegisterCode::kRegCode_r11 as i32 }
        }

        pub const fn ip() -> Self {
            Self { code: RegisterCode::kRegCode_ip as i32 }
        }

        pub const fn r13() -> Self {
            Self { code: RegisterCode::kRegCode_r13 as i32 }
        }

        pub const fn r14() -> Self {
            Self { code: RegisterCode::kRegCode_r14 as i32 }
        }

        pub const fn r15() -> Self {
            Self { code: RegisterCode::kRegCode_r15 as i32 }
        }

        pub const fn r16() -> Self {
            Self { code: RegisterCode::kRegCode_r16 as i32 }
        }

        pub const fn r17() -> Self {
            Self { code: RegisterCode::kRegCode_r17 as i32 }
        }

        pub const fn r18() -> Self {
            Self { code: RegisterCode::kRegCode_r18 as i32 }
        }

        pub const fn r19() -> Self {
            Self { code: RegisterCode::kRegCode_r19 as i32 }
        }

        pub const fn r20() -> Self {
            Self { code: RegisterCode::kRegCode_r20 as i32 }
        }

        pub const fn r21() -> Self {
            Self { code: RegisterCode::kRegCode_r21 as i32 }
        }

        pub const fn r22() -> Self {
            Self { code: RegisterCode::kRegCode_r22 as i32 }
        }

        pub const fn r23() -> Self {
            Self { code: RegisterCode::kRegCode_r23 as i32 }
        }

        pub const fn r24() -> Self {
            Self { code: RegisterCode::kRegCode_r24 as i32 }
        }

        pub const fn r25() -> Self {
            Self { code: RegisterCode::kRegCode_r25 as i32 }
        }

        pub const fn r26() -> Self {
            Self { code: RegisterCode::kRegCode_r26 as i32 }
        }

        pub const fn r27() -> Self {
            Self { code: RegisterCode::kRegCode_r27 as i32 }
        }

        pub const fn r28() -> Self {
            Self { code: RegisterCode::kRegCode_r28 as i32 }
        }

        pub const fn r29() -> Self {
            Self { code: RegisterCode::kRegCode_r29 as i32 }
        }

        pub const fn r30() -> Self {
            Self { code: RegisterCode::kRegCode_r30 as i32 }
        }

        pub const fn fp() -> Self {
            Self { code: RegisterCode::kRegCode_fp as i32 }
        }
    }

    pub fn reassign_register(source: &mut Register) -> Register {
        let result = *source;
        *source = Register::no_reg();
        result
    }

    macro_rules! define_register {
        ($r:ident) => {
            pub const $r: Register = Register::from_code(RegisterCode::kRegCode_$r as i32);
        };
    }

    define_register!(r0);
    define_register!(sp);
    define_register!(r2);
    define_register!(r3);
    define_register!(r4);
    define_register!(r5);
    define_register!(r6);
    define_register!(r7);
    define_register!(r8);
    define_register!(r9);
    define_register!(r10);
    define_register!(r11);
    define_register!(ip);
    define_register!(r13);
    define_register!(r14);
    define_register!(r15);
    define_register!(r16);
    define_register!(r17);
    define_register!(r18);
    define_register!(r19);
    define_register!(r20);
    define_register!(r21);
    define_register!(r22);
    define_register!(r23);
    define_register!(r24);
    define_register!(r25);
    define_register!(r26);
    define_register!(r27);
    define_register!(r28);
    define_register!(r29);
    define_register!(r30);
    define_register!(fp);

    pub const no_reg: Register = Register::no_reg();
    pub const kConstantPoolRegister: Register = r28;
    pub const kRootRegister: Register = r29;
    pub const cp: Register = r30;
    pub const kPtrComprCageBaseRegister: Register = r27;
    pub const kCArgRegs: [Register; 8] = [r3, r4, r5, r6, r7, r8, r9, r10];
    pub const kRegisterPassedArguments: usize = kCArgRegs.len();

    pub const fn argument_padding_slots(argument_count: i32) -> i32 {
        0
    }

    pub enum AliasingKind {
        kIndependent,
    }

    pub const kFPAliasing: AliasingKind = AliasingKind::kIndependent;
    pub const kSimdMaskRegisters: bool = false;

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum Simd128RegisterCode {
        kSimd128Code_v0,
        kSimd128Code_v1,
        kSimd128Code_v2,
        kSimd128Code_v3,
        kSimd128Code_v4,
        kSimd128Code_v5,
        kSimd128Code_v6,
        kSimd128Code_v7,
        kSimd128Code_v8,
        kSimd128Code_v9,
        kSimd128Code_v10,
        kSimd128Code_v11,
        kSimd128Code_v12,
        kSimd128Code_v13,
        kSimd128Code_v14,
        kSimd128Code_v15,
        kSimd128Code_v16,
        kSimd128Code_v17,
        kSimd128Code_v18,
        kSimd128Code_v19,
        kSimd128Code_v20,
        kSimd128Code_v21,
        kSimd128Code_v22,
        kSimd128Code_v23,
        kSimd128Code_v24,
        kSimd128Code_v25,
        kSimd128Code_v26,
        kSimd128Code_v27,
        kSimd128Code_v28,
        kSimd128Code_v29,
        kSimd128Code_v30,
        kSimd128Code_v31,
        kSimd128AfterLast,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct Simd128Register {
        code: i32,
    }

    impl Simd128Register {
        pub const fn new(code: i32) -> Self {
            Self { code }
        }

        pub const fn from_code(code: i32) -> Self {
            Self { code }
        }

        pub const fn code(&self) -> i32 {
            self.code
        }

        pub const fn is_valid(&self) -> bool {
            self.code >= 0 && self.code < Simd128RegisterCode::kSimd128AfterLast as i32
        }

        pub const fn is_no_reg(&self) -> bool {
            self.code == -1
        }

        pub const fn no_reg() -> Self {
            Self { code: -1 }
        }

        pub const fn v0() -> Self {
            Self { code: Simd128RegisterCode::kSimd128Code_v0 as i32 }
        }

        pub const fn v1() -> Self {
            Self { code: Simd128RegisterCode::kSimd128Code_v1 as i32 }
        }

        pub const fn v2() -> Self {
            Self { code: Simd128RegisterCode::kSimd128Code_v2 as i32 }
        }

        pub const fn v3() -> Self {
            Self { code: Simd128RegisterCode::kSimd128Code_v3 as i32 }
        }

        pub const fn v4() -> Self {
            Self { code: Simd128RegisterCode::kSimd128Code_v4 as i32 }
        }

        pub const fn v5() -> Self {
            Self { code: Simd128RegisterCode::kSimd128Code_v5 as i32 }
        }

        pub const fn v6() -> Self {
            Self { code: Simd128RegisterCode::kSimd128Code_v6 as i32 }
        }

        pub const fn v7() -> Self {
            Self { code: Simd128RegisterCode::kSimd128Code_v7 as i32 }
        }

        pub const fn v8() -> Self {
            Self { code: Simd128RegisterCode::kSimd128Code_v8 as i32 }
        }

        pub const fn v9() -> Self {
            Self { code: Simd128RegisterCode::kSimd128Code_v9 as i32 }
        }

        pub const fn v10() -> Self {
            Self { code: Simd128RegisterCode::kSimd128Code_v10 as i32 }
        }

        pub const fn v11() -> Self {
            Self { code: Simd128RegisterCode::kSimd128Code_v11 as i32 }
        }

        pub const fn v12() -> Self {
            Self { code: Simd128RegisterCode::kSimd128Code_v12 as i32 }
        }

        pub const fn v13() -> Self {
            Self { code: Simd128RegisterCode::kSimd128Code_v13 as i32 }
        }

        pub const fn v14() -> Self {
            Self { code: Simd128RegisterCode::kSimd128Code_v14 as i32 }
        }

        pub const fn v15() -> Self {
            Self { code: Simd128RegisterCode::kSimd128Code_v15 as i32 }
        }

        pub const fn v16() -> Self {
            Self { code: Simd128RegisterCode::kSimd128Code_v16 as i32 }
        }

        pub const fn v17() -> Self {
            Self { code: Simd128RegisterCode::kSimd128Code_v17 as i32 }
        }

        pub const fn v18() -> Self {
            Self { code: Simd128RegisterCode::kSimd128Code_v18 as i32 }
        }

        pub const fn v19() -> Self {
            Self { code: Simd128RegisterCode::kSimd128Code_v19 as i32 }
        }

        pub const fn v20() -> Self {
            Self { code: Simd128RegisterCode::kSimd128Code_v20 as i32 }
        }

        pub const fn v21() -> Self {
            Self { code: Simd128RegisterCode::kSimd128Code_v21 as i32 }
        }

        pub const fn v22() -> Self {
            Self { code: Simd128RegisterCode::kSimd128Code_v22 as i32 }
        }

        pub const fn v23() -> Self {
            Self { code: Simd128RegisterCode::kSimd128Code_v23 as i32 }
        }

        pub const fn v24() -> Self {
            Self { code: Simd128RegisterCode::kSimd128Code_v24 as i32 }
        }

        pub const fn v25() -> Self {
            Self { code: Simd128RegisterCode::kSimd128Code_v25 as i32 }
        }

        pub const fn v26() -> Self {
            Self { code: Simd128RegisterCode::kSimd128Code_v26 as i32 }
        }

        pub const fn v27() -> Self {
            Self { code: Simd128RegisterCode::kSimd128Code_v27 as i32 }
        }

        pub const fn v28() -> Self {
            Self { code: Simd128RegisterCode::kSimd128Code_v28 as i32 }
        }

        pub const fn v29() -> Self {
            Self { code: Simd128RegisterCode::kSimd128Code_v29 as i32 }
        }

        pub const fn v30() -> Self {
            Self { code: Simd128RegisterCode::kSimd128Code_v30 as i32 }
        }

        pub const fn v31() -> Self {
            Self { code: Simd128RegisterCode::kSimd128Code_v31 as i32 }
        }
    }

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

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DoubleRegister {
        code: i32,
    }

    impl DoubleRegister {
        pub const fn new(code: i32) -> Self {
            Self { code }
        }

        pub const fn from_code(code: i32) -> Self {
            Self { code }
        }

        pub const fn code(&self) -> i32 {
            self.code
        }

        pub const fn is_valid(&self) -> bool {
            self.code >= 0 && self.code < DoubleRegisterCode::kDoubleAfterLast as i32
        }

        pub const fn is_no_reg(&self) -> bool {
            self.code == -1
        }

        pub const fn no_reg() -> Self {
            Self { code: -1 }
        }

        pub const fn SupportedRegisterCount() -> i32 {
            32
        }

        pub fn to_simd(&self) -> Simd128Register {
            let reg_code = self.code();
            if reg_code >= 0 && reg_code < Simd128RegisterCode::kSimd128AfterLast as i32 {
                Simd128Register::from_code(reg_code)
            } else {
                panic!("Invalid register code for Simd128Register: {}", reg_code);
            }
        }

        pub const fn d0() -> Self {
            Self { code: DoubleRegisterCode::kDoubleCode_d0 as i32 }
        }

        pub const fn d1() -> Self {
            Self { code: DoubleRegisterCode::kDoubleCode_d1 as i32 }
        }

        pub const fn d2() -> Self {
            Self { code: DoubleRegisterCode::kDoubleCode_d2 as i32 }
        }

        pub const fn d3() -> Self {
            Self { code: DoubleRegisterCode::kDoubleCode_d3 as i32 }
        }

        pub const fn d4() -> Self {
            Self { code: DoubleRegisterCode::kDoubleCode_d4 as i32 }
        }

        pub const fn d5() -> Self {
            Self { code: DoubleRegisterCode::kDoubleCode_d5 as i32 }
        }

        pub const fn d6() -> Self {
            Self { code: DoubleRegisterCode::kDoubleCode_d6 as i32 }
        }

        pub const fn d7() -> Self {
            Self { code: DoubleRegisterCode::kDoubleCode_d7 as i32 }
        }

        pub const fn d8() -> Self {
            Self { code: DoubleRegisterCode::kDoubleCode_d8 as i32 }
        }

        pub const fn d9() -> Self {
            Self { code: DoubleRegisterCode::kDoubleCode_d9 as i32 }
        }

        pub const fn d10() -> Self {
            Self { code: DoubleRegisterCode::kDoubleCode_d10 as i32 }
        }

        pub const fn d11() -> Self {
            Self { code: DoubleRegisterCode::kDoubleCode_d11 as i32 }
        }

        pub const fn d12() -> Self {
            Self { code: DoubleRegisterCode::kDoubleCode_d12 as i32 }
        }

        pub const fn d13() -> Self {
            Self { code: DoubleRegisterCode::kDoubleCode_d13 as i32 }
        }

        pub const fn d14() -> Self {
            Self { code: DoubleRegisterCode::kDoubleCode_d14 as i32 }
        }

        pub const fn d15() -> Self {
            Self { code: DoubleRegisterCode::kDoubleCode_d15 as i32 }
        }

        pub const fn d16() -> Self {
            Self { code: DoubleRegisterCode::kDoubleCode_d16 as i32 }
        }

        pub const fn d17() -> Self {
            Self { code: DoubleRegisterCode::kDoubleCode_d17 as i32 }
        }

        pub const fn d18() -> Self {
            Self { code: DoubleRegisterCode::kDoubleCode_d18 as i32 }
        }

        pub const fn d19() -> Self {
            Self { code: DoubleRegisterCode::kDoubleCode_d19 as i32 }
        }

        pub const fn d20() -> Self {
            Self { code: DoubleRegisterCode::kDoubleCode_d20 as i32 }
        }

        pub const fn d21() -> Self {
            Self { code: DoubleRegisterCode::kDoubleCode_d21 as i32 }
        }

        pub const fn d22() -> Self {
            Self { code: DoubleRegisterCode::kDoubleCode_d22 as i32 }
        }

        pub const fn d23() -> Self {
            Self { code: DoubleRegisterCode::kDoubleCode_d23 as i32 }
        }

        pub const fn d24() -> Self {
            Self { code: DoubleRegisterCode::kDoubleCode_d24 as i32 }
        }

        pub const fn d25() -> Self {
            Self { code: DoubleRegisterCode::kDoubleCode_d25 as i32 }
        }

        pub const fn d26() -> Self {
            Self { code: DoubleRegisterCode::kDoubleCode_d26 as i32 }
        }

        pub const fn d27() -> Self {
            Self { code: DoubleRegisterCode::kDoubleCode_d27 as i32 }
        }

        pub const fn d28() -> Self {
            Self { code: DoubleRegisterCode::kDoubleCode_d28 as i32 }
        }

        pub const fn d29() -> Self {
            Self { code: DoubleRegisterCode::kDoubleCode_d29 as i32 }
        }

        pub const fn d30() -> Self {
            Self { code: DoubleRegisterCode::kDoubleCode_d30 as i32 }
        }

        pub const fn d31() -> Self {
            Self { code: DoubleRegisterCode::kDoubleCode_d31 as i32 }
        }
    }

    pub type FloatRegister = DoubleRegister;

    macro_rules! declare_simd128_register {
        ($r:ident) => {
            pub const $r: Simd128Register = Simd128Register::from_code(Simd128RegisterCode::kSimd128Code_$r as i32);
        };
    }

    declare_simd128_register!(v0);
    declare_simd128_register!(v1);
    declare_simd128_register!(v2);
    declare_simd128_register!(v3);
    declare_simd128_register!(v4);
    declare_simd128_register!(v5);
    declare_simd128_register!(v6);
    declare_simd128_register!(v7);
    declare_simd128_register!(v8);
    declare_simd128_register!(v9);
    declare_simd128_register!(v10);
    declare_simd128_register!(v11);
    declare_simd128_register!(v12);
    declare_simd128_register!(v13);
    declare_simd128_register!(v14);
    declare_simd128_register!(v15);
    declare_simd128_register!(v16);
    declare_simd128_register!(v17);
    declare_simd128_register!(v18);
    declare_simd128_register!(v19);
    declare_simd128_register!(v20);
    declare_simd128_register!(v21);
    declare_simd128_register!(v22);
    declare_simd128_register!(v23);
    declare_simd128_register!(v24);
    declare_simd128_register!(v25);
    declare_simd128_register!(v26);
    declare_simd128_register!(v27);
    declare_simd128_register!(v28);
    declare_simd128_register!(v29);
    declare_simd128_register!(v30);
    declare_simd128_register!(v31);
    pub const no_simdreg: Simd128Register = Simd128Register::no_reg();

    macro_rules! define_double_register {
        ($r:ident) => {
            pub const $r: DoubleRegister = DoubleRegister::from_code(DoubleRegisterCode::kDoubleCode_$r as i32);
        };
    }

    define_double_register!(d0);
    define_double_register!(d1);
    define_double_register!(d2);
    define_double_register!(d3);
    define_double_register!(d4);
    define_double_register!(d5);
    define_double_register!(d6);
    define_double_register!(d7);
    define_double_register!(d8);
    define_double_register!(d9);
    define_double_register!(d10);
    define_double_register!(d11);
    define_double_register!(d12);
    define_double_register!(d13);
    define_double_register!(d14);
    define_double_register!(d15);
    define_double_register!(d16);
    define_double_register!(d17);
    define_double_register!(d18);
    define_double_register!(d19);
    define_double_register!(d20);
    define_double_register!(d21);
    define_double_register!(d22);
    define_double_register!(d23);
    define_double_register!(d24);
    define_double_register!(d25);
    define_double_register!(d26);
    define_double_register!(d27);
    define_double_register!(d28);
    define_double_register!(d29);
    define_double_register!(d30);
    define_double_register!(d31);
    pub const no_dreg: DoubleRegister = DoubleRegister::no_reg();

    pub const kFirstCalleeSavedDoubleReg: DoubleRegister = d14;
    pub const kLastCalleeSavedDoubleReg: DoubleRegister = d31;
    pub const kDoubleRegZero: DoubleRegister = d14;
    pub const kScratchDoubleReg: DoubleRegister = d13;
    pub const kSimd128RegZero: Simd128Register = v14;
    pub const kScratchSimd128Reg: Simd128Register = v13;
    pub const kScratchSimd128Reg2: Simd128Register = v15;

    pub fn to_register(num: i32) -> Register {
        Register::from_code(num)
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum CRegisterCode {
        kCCode_cr0,
        kCCode_cr1,
        kCCode_cr2,
        kCCode_cr3,
        kCCode_cr4,
        kCCode_cr5,
        kCCode_cr6,
        kCCode_cr7,
        kCCode_cr8,
        kCCode_cr9,
        kCCode_cr10,
        kCCode_cr11,
        kCCode_cr12,
        kCCode_cr15,
        kCAfterLast,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct CRegister {
        code: i32,
    }

    impl CRegister {
        pub const fn new(code: i32) -> Self {
            Self { code }
        }

        pub const fn from_code(code: i32) -> Self {
            Self { code }
        }

        pub const fn code(&self) -> i32 {
            self.code
        }

        pub const fn is_valid(&self) -> bool {
            self.code >= 0 && self.code < CRegisterCode::kCAfterLast as i32
        }

        pub const fn is_no_reg(&self) -> bool {
            self.code == -1
        }

        pub const fn no_reg() -> Self {
            Self { code: -1 }
        }

        pub const fn cr0() -> Self {
            Self { code: CRegisterCode::kCCode_cr0 as i32 }
        }

        pub const fn cr1() -> Self {
            Self { code: CRegisterCode::
