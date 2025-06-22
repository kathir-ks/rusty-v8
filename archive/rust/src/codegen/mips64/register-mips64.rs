// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod register_mips64 {
    use std::fmt;

    /// Alias for i32 representing a register code.
    pub type RegisterCode = i32;

    /// Enum-like struct representing a general-purpose register.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct Register {
        code: RegisterCode,
    }

    /// Enum-like struct representing a floating-point register.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct FPURegister {
        code: RegisterCode,
    }

    /// Enum-like struct representing a SIMD128 register (MSA).
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct MSARegister {
        code: RegisterCode,
    }

    /// Enum-like struct representing a double-precision floating-point register.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DoubleRegister {
        code: RegisterCode,
    }

    /// Represents FPU control register
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct FPUControlRegister {
        reg_code: i32,
    }

    /// Represents MSA control register
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct MSAControlRegister {
        reg_code: i32,
    }

    impl Register {
        pub const fn from_code(code: RegisterCode) -> Self {
            Self { code }
        }

        pub const fn no_reg() -> Self {
            Self { code: -1 } // Assuming -1 represents no_reg
        }

        pub const fn code(&self) -> RegisterCode {
            self.code
        }
    }

    impl FPURegister {
        pub const fn from_code(code: RegisterCode) -> Self {
            Self { code }
        }

        pub const fn no_reg() -> Self {
            Self { code: -1 } // Assuming -1 represents no_reg
        }

        pub const fn code(&self) -> RegisterCode {
            self.code
        }

        pub fn low(&self) -> Self {
            assert_eq!(self.code() % 2, 0);
            FPURegister::from_code(self.code())
        }

        pub fn high(&self) -> Self {
            assert_eq!(self.code() % 2, 0);
            FPURegister::from_code(self.code() + 1)
        }

        pub fn to_w(&self) -> MSARegister {
            MSARegister::from_code(self.code())
        }
    }

    impl MSARegister {
        pub const fn from_code(code: RegisterCode) -> Self {
            Self { code }
        }

        pub const fn no_reg() -> Self {
            Self { code: -1 } // Assuming -1 represents no_reg
        }

        pub const fn code(&self) -> RegisterCode {
            self.code
        }
    }

    impl DoubleRegister {
        pub const fn from_code(code: RegisterCode) -> Self {
            Self { code }
        }

        pub const fn no_reg() -> Self {
            Self { code: -1 } // Assuming -1 represents no_reg
        }

        pub const fn code(&self) -> RegisterCode {
            self.code
        }
    }

    impl FPUControlRegister {
        pub const fn new(code: i32) -> Self {
            FPUControlRegister { reg_code: code }
        }

        pub fn is_valid(&self) -> bool {
            self.reg_code == K_FCSR_REGISTER
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

    impl MSAControlRegister {
        pub const fn new(code: i32) -> Self {
            MSAControlRegister { reg_code: code }
        }

        pub fn is_valid(&self) -> bool {
            (self.reg_code == K_MSAIR_REGISTER) || (self.reg_code == K_MSACSR_REGISTER)
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

    impl fmt::Display for Register {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", register_name(*self))
        }
    }

    impl fmt::Display for FPURegister {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", fpu_register_name(*self))
        }
    }

    impl fmt::Display for MSARegister {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", msa_register_name(*self))
        }
    }

    // Could be a `const` but not used in const context in this code.
    pub fn to_number(reg: Register) -> i32 {
        reg.code()
    }

    // Could be a `const` but not used in const context in this code.
    pub fn to_register(num: i32) -> Register {
        Register::from_code(num)
    }

    // Could be a `const` but not used in const context in this code.
    pub fn reassign_register(source: &mut Register) -> Register {
        let result = *source;
        *source = Register::no_reg();
        result
    }

    pub const fn argument_padding_slots(_argument_count: i32) -> i32 {
        0
    }

    pub const K_FP_ALIASING: AliasingKind = AliasingKind::Overlap;
    pub const K_SIMD_MASK_REGISTERS: bool = false;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum AliasingKind {
        Disjoint,
        Overlap,
    }

    macro_rules! define_registers {
        ($reg_type:ident, $($reg:ident),*) => {
            $(
                #[allow(non_upper_case_globals)]
                pub const $reg: $reg_type = $reg_type { code: k_reg_code::$reg as i32 };
            )*
        };
    }

    macro_rules! define_register_names {
        ($reg_type:ident, $func_name:ident, $($reg:ident),*) => {
            fn $func_name(reg: $reg_type) -> &'static str {
                match reg.code() {
                    $(
                        k_reg_code::$reg as i32 => stringify!($reg),
                    )*
                    _ => "invalid",
                }
            }
        };
    }

    // Define all the registers.
    pub mod k_reg_code {
        #![allow(non_upper_case_globals)]
        pub const zero_reg: u32 = 0;
        pub const at: u32 = 1;
        pub const v0: u32 = 2;
        pub const v1: u32 = 3;
        pub const a0: u32 = 4;
        pub const a1: u32 = 5;
        pub const a2: u32 = 6;
        pub const a3: u32 = 7;
        pub const a4: u32 = 8;
        pub const a5: u32 = 9;
        pub const a6: u32 = 10;
        pub const a7: u32 = 11;
        pub const t0: u32 = 12;
        pub const t1: u32 = 13;
        pub const t2: u32 = 14;
        pub const t3: u32 = 15;
        pub const s0: u32 = 16;
        pub const s1: u32 = 17;
        pub const s2: u32 = 18;
        pub const s3: u32 = 19;
        pub const s4: u32 = 20;
        pub const s5: u32 = 21;
        pub const s6: u32 = 22;
        pub const s7: u32 = 23;
        pub const t8: u32 = 24;
        pub const t9: u32 = 25;
        pub const k0: u32 = 26;
        pub const k1: u32 = 27;
        pub const gp: u32 = 28;
        pub const sp: u32 = 29;
        pub const fp: u32 = 30;
        pub const ra: u32 = 31;

        pub const f0: u32 = 0;
        pub const f1: u32 = 1;
        pub const f2: u32 = 2;
        pub const f3: u32 = 3;
        pub const f4: u32 = 4;
        pub const f5: u32 = 5;
        pub const f6: u32 = 6;
        pub const f7: u32 = 7;
        pub const f8: u32 = 8;
        pub const f9: u32 = 9;
        pub const f10: u32 = 10;
        pub const f11: u32 = 11;
        pub const f12: u32 = 12;
        pub const f13: u32 = 13;
        pub const f14: u32 = 14;
        pub const f15: u32 = 15;
        pub const f16: u32 = 16;
        pub const f17: u32 = 17;
        pub const f18: u32 = 18;
        pub const f19: u32 = 19;
        pub const f20: u32 = 20;
        pub const f21: u32 = 21;
        pub const f22: u32 = 22;
        pub const f23: u32 = 23;
        pub const f24: u32 = 24;
        pub const f25: u32 = 25;
        pub const f26: u32 = 26;
        pub const f27: u32 = 27;
        pub const f28: u32 = 28;
        pub const f29: u32 = 29;
        pub const f30: u32 = 30;
        pub const f31: u32 = 31;

        pub const w0: u32 = 0;
        pub const w1: u32 = 1;
        pub const w2: u32 = 2;
        pub const w3: u32 = 3;
        pub const w4: u32 = 4;
        pub const w5: u32 = 5;
        pub const w6: u32 = 6;
        pub const w7: u32 = 7;
        pub const w8: u32 = 8;
        pub const w9: u32 = 9;
        pub const w10: u32 = 10;
        pub const w11: u32 = 11;
        pub const w12: u32 = 12;
        pub const w13: u32 = 13;
        pub const w14: u32 = 14;
        pub const w15: u32 = 15;
        pub const w16: u32 = 16;
        pub const w17: u32 = 17;
        pub const w18: u32 = 18;
        pub const w19: u32 = 19;
        pub const w20: u32 = 20;
        pub const w21: u32 = 21;
        pub const w22: u32 = 22;
        pub const w23: u32 = 23;
        pub const w24: u32 = 24;
        pub const w25: u32 = 25;
        pub const w26: u32 = 26;
        pub const w27: u32 = 27;
        pub const w28: u32 = 28;
        pub const w29: u32 = 29;
        pub const w30: u32 = 30;
        pub const w31: u32 = 31;
    }

    define_registers!(
        Register, zero_reg, at, v0, v1, a0, a1, a2, a3, a4, a5, a6, a7, t0, t1, t2, t3, s0, s1, s2,
        s3, s4, s5, s6, s7, t8, t9, k0, k1, gp, sp, fp, ra
    );

    define_registers!(
        DoubleRegister, f0, f1, f2, f3, f4, f5, f6, f7, f8, f9, f10, f11, f12, f13, f14, f15, f16,
        f17, f18, f19, f20, f21, f22, f23, f24, f25, f26, f27, f28, f29, f30, f31
    );

    define_registers!(
        MSARegister, w0, w1, w2, w3, w4, w5, w6, w7, w8, w9, w10, w11, w12, w13, w14, w15, w16,
        w17, w18, w19, w20, w21, w22, w23, w24, w25, w26, w27, w28, w29, w30, w31
    );

    define_register_names!(
        Register,
        register_name,
        zero_reg,
        at,
        v0,
        v1,
        a0,
        a1,
        a2,
        a3,
        a4,
        a5,
        a6,
        a7,
        t0,
        t1,
        t2,
        t3,
        s0,
        s1,
        s2,
        s3,
        s4,
        s5,
        s6,
        s7,
        t8,
        t9,
        k0,
        k1,
        gp,
        sp,
        fp,
        ra
    );

    define_register_names!(
        FPURegister,
        fpu_register_name,
        f0,
        f1,
        f2,
        f3,
        f4,
        f5,
        f6,
        f7,
        f8,
        f9,
        f10,
        f11,
        f12,
        f13,
        f14,
        f15,
        f16,
        f17,
        f18,
        f19,
        f20,
        f21,
        f22,
        f23,
        f24,
        f25,
        f26,
        f27,
        f28,
        f29,
        f30,
        f31
    );

    define_register_names!(
        MSARegister,
        msa_register_name,
        w0,
        w1,
        w2,
        w3,
        w4,
        w5,
        w6,
        w7,
        w8,
        w9,
        w10,
        w11,
        w12,
        w13,
        w14,
        w15,
        w16,
        w17,
        w18,
        w19,
        w20,
        w21,
        w22,
        w23,
        w24,
        w25,
        w26,
        w27,
        w28,
        w29,
        w30,
        w31
    );

    pub const NO_REG: Register = Register::no_reg();
    pub const NO_DREG: DoubleRegister = DoubleRegister::no_reg();
    pub const NO_MSAREG: MSARegister = MSARegister::no_reg();

    pub const K_C_ARG_REGS: [Register; 8] = [a0, a1, a2, a3, a4, a5, a6, a7];
    pub const K_REGISTER_PASSED_ARGUMENTS: usize = K_C_ARG_REGS.len();
    pub const K_FP_REGISTER_PASSED_ARGUMENTS: i32 = 8;

    pub const K_RETURN_REGISTER0: Register = v0;
    pub const K_RETURN_REGISTER1: Register = v1;
    pub const K_RETURN_REGISTER2: Register = a0;
    pub const K_JS_FUNCTION_REGISTER: Register = a1;
    pub const K_CONTEXT_REGISTER: Register = s7;
    pub const K_ALLOCATE_SIZE_REGISTER: Register = a0;
    pub const K_INTERPRETER_ACCUMULATOR_REGISTER: Register = v0;
    pub const K_INTERPRETER_BYTECODE_OFFSET_REGISTER: Register = t0;
    pub const K_INTERPRETER_BYTECODE_ARRAY_REGISTER: Register = t1;
    pub const K_INTERPRETER_DISPATCH_TABLE_REGISTER: Register = t2;
    pub const K_JAVA_SCRIPT_CALL_ARG_COUNT_REGISTER: Register = a0;
    pub const K_JAVA_SCRIPT_CALL_CODE_START_REGISTER: Register = a2;
    pub const K_JAVA_SCRIPT_CALL_TARGET_REGISTER: Register = K_JS_FUNCTION_REGISTER;
    pub const K_JAVA_SCRIPT_CALL_NEW_TARGET_REGISTER: Register = a3;
    pub const K_JAVA_SCRIPT_CALL_EXTRA_ARG1_REGISTER: Register = a2;
    pub const K_JAVA_SCRIPT_CALL_DISPATCH_HANDLE_REGISTER: Register = a4;

    pub const K_RUNTIME_CALL_FUNCTION_REGISTER: Register = a1;
    pub const K_RUNTIME_CALL_ARG_COUNT_REGISTER: Register = a0;
    pub const K_RUNTIME_CALL_ARGV_REGISTER: Register = a2;
    pub const K_WASM_IMPLICIT_ARG_REGISTER: Register = a0;
    pub const K_WASM_COMPILE_LAZY_FUNC_INDEX_REGISTER: Register = t0;

    pub const K_FP_RETURN_REGISTER0: DoubleRegister = f0;

    pub const K_ROOT_REGISTER: Register = s6;
    pub const CP: Register = s7;
    pub const K_SCRATCH_REG: Register = s3;
    pub const K_SCRATCH_REG2: Register = s4;
    pub const K_SCRATCH_DOUBLE_REG: DoubleRegister = f30;
    pub const K_SCRATCH_DOUBLE_REG2: DoubleRegister = f31;
    pub const K_DOUBLE_REG_ZERO: DoubleRegister = f28;
    pub const K_DOUBLE_COMPARE_REG: DoubleRegister = f23;
    pub const K_SIMD128_REG_ZERO: MSARegister = w28;
    pub const K_SIMD128_SCRATCH_REG: MSARegister = w30;

    pub const K_INVALID_FPU_CONTROL_REGISTER: i32 = -1;
    pub const K_FCSR_REGISTER: i32 = 31;
    pub const NO_FPUCREG: FPUControlRegister = FPUControlRegister::new(K_INVALID_FPU_CONTROL_REGISTER);
    pub const FCSR: FPUControlRegister = FPUControlRegister::new(K_FCSR_REGISTER);

    pub const K_INVALID_MSA_CONTROL_REGISTER: i32 = -1;
    pub const K_MSAIR_REGISTER: i32 = 0;
    pub const K_MSACSR_REGISTER: i32 = 1;
    pub const NO_MSACREG: MSAControlRegister = MSAControlRegister::new(K_INVALID_MSA_CONTROL_REGISTER);
    pub const MSAIR: MSAControlRegister = MSAControlRegister::new(K_MSAIR_REGISTER);
    pub const MSACSR: MSAControlRegister = MSAControlRegister::new(K_MSACSR_REGISTER);

    pub type FloatRegister = FPURegister;
}