// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod register_loong64 {
    use std::marker::PhantomData;

    pub mod constants_loong64 {
        // Placeholder for constants-loong64.h content.
        // Define constants specific to the LoongArch64 architecture here.
        pub const kInvalidFPUControlRegister: i32 = -1;
        pub const kFCSRRegister: i32 = 0;
    }

    pub mod register_base {
        use std::fmt;
        use std::ops::{Deref, DerefMut};

        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct RegisterBase<T, const N: usize> {
            code: i32,
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T, const N: usize> RegisterBase<T, const N> {
            pub const fn new(code: i32) -> Self {
                RegisterBase {
                    code,
                    _phantom: std::marker::PhantomData,
                }
            }

            pub const fn code(&self) -> i32 {
                self.code
            }

            pub const fn from_code(code: i32) -> Self {
                RegisterBase {
                    code,
                    _phantom: std::marker::PhantomData,
                }
            }

            pub const fn no_reg() -> Self {
                RegisterBase {
                    code: -1, // Or some other invalid code
                    _phantom: std::marker::PhantomData,
                }
            }

            pub fn is_valid(&self) -> bool {
                self.code >= 0 && self.code < N as i32
            }

            pub fn is(&self, other: &Self) -> bool {
                self.code == other.code
            }
        }

        impl<T: fmt::Debug, const N: usize> fmt::Debug for RegisterBase<T, N> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct("RegisterBase")
                    .field("code", &self.code)
                    .finish()
            }
        }

        // You can add more methods here based on the C++ RegisterBase class.
    }

    use self::constants_loong64::*;
    use self::register_base::*;

    // clang-format off
    macro_rules! general_registers {
        ($V:ident) => {
            $V!(zero_reg);   $V!(ra);  $V!(tp);  $V!(sp);
            $V!(a0);  $V!(a1);  $V!(a2);  $V!(a3); $V!(a4);  $V!(a5);  $V!(a6);  $V!(a7);
            $V!(t0);  $V!(t1);  $V!(t2);  $V!(t3); $V!(t4);  $V!(t5);  $V!(t6);  $V!(t7);  $V!(t8);
            $V!(x_reg);      $V!(fp);
            $V!(s0);  $V!(s1);  $V!(s2);  $V!(s3);  $V!(s4);  $V!(s5);  $V!(s6);  $V!(s7);  $V!(s8);
        };
    }

    macro_rules! always_allocatable_general_registers {
        ($V:ident) => {
            $V!(a0);  $V!(a1);  $V!(a2);  $V!(a3);  $V!(a4);  $V!(a5);  $V!(a6);  $V!(a7);
            $V!(t0);  $V!(t1);  $V!(t2);  $V!(t3);  $V!(t4);  $V!(t5);
            $V!(s0);  $V!(s1);  $V!(s2);  $V!(s3);  $V!(s4);  $V!(s5);  $V!(s7);
        };
    }

    macro_rules! maybe_allocatable_general_registers {
        ($V:ident) => {
            $V!(s8);
        };
    }

    macro_rules! allocatable_general_registers {
        ($V:ident) => {
            always_allocatable_general_registers!($V);
            maybe_allocatable_general_registers!($V);
        };
    }

    macro_rules! double_registers {
        ($V:ident) => {
            $V!(f0);  $V!(f1);  $V!(f2);  $V!(f3);  $V!(f4);  $V!(f5);  $V!(f6);  $V!(f7);
            $V!(f8);  $V!(f9);  $V!(f10); $V!(f11); $V!(f12); $V!(f13); $V!(f14); $V!(f15);
            $V!(f16); $V!(f17); $V!(f18); $V!(f19); $V!(f20); $V!(f21); $V!(f22); $V!(f23);
            $V!(f24); $V!(f25); $V!(f26); $V!(f27); $V!(f28); $V!(f29); $V!(f30); $V!(f31);
        };
    }

    macro_rules! float_registers {
        ($V:ident) => {
            double_registers!($V);
        };
    }

    macro_rules! simd128_registers {
        ($V:ident) => {
            $V!(w0);  $V!(w1);  $V!(w2);  $V!(w3);  $V!(w4);  $V!(w5);  $V!(w6);  $V!(w7);
            $V!(w8);  $V!(w9);  $V!(w10); $V!(w11); $V!(w12); $V!(w13); $V!(w14); $V!(w15);
            $V!(w16); $V!(w17); $V!(w18); $V!(w19); $V!(w20); $V!(w21); $V!(w22); $V!(w23);
            $V!(w24); $V!(w25); $V!(w26); $V!(w27); $V!(w28); $V!(w29); $V!(w30); $V!(w31);
        };
    }

    macro_rules! allocatable_double_registers {
        ($V:ident) => {
            $V!(f0);  $V!(f1);  $V!(f2);  $V!(f3);  $V!(f4);  $V!(f5);  $V!(f6);  $V!(f7);
            $V!(f8);  $V!(f9);  $V!(f10); $V!(f11); $V!(f12); $V!(f13); $V!(f14); $V!(f15);
            $V!(f16); $V!(f17); $V!(f18); $V!(f19); $V!(f20); $V!(f21); $V!(f22); $V!(f23);
            $V!(f24); $V!(f25); $V!(f26); $V!(f27); $V!(f28);
        };
    }
    // clang-format on

    // Note that the bit values must match those used in actual instruction
    // encoding.
    pub const NUM_REGS: usize = 32;

    // CPU Registers.
    //
    // 1) We would prefer to use an enum, but enum values are assignment-
    // compatible with int, which has caused code-generation bugs.
    //
    // 2) We would prefer to use a class instead of a struct but we don't like
    // the register initialization to depend on the particular initialization
    // order (which appears to be different on OS X, Linux, and Windows for the
    // installed versions of C++ we tried). Using a struct permits C-style
    // "initialization". Also, the Register objects cannot be const as this
    // forces initialization stubs in MSVC, making us dependent on initialization
    // order.
    //
    // 3) By not using an enum, we are possibly preventing the compiler from
    // doing certain constant folds, which may significantly reduce the
    // code generated for some assembly instructions (because they boil down
    // to a few constants). If this is a problem, we could change the code
    // such that we use an enum in optimized mode, and the struct in debug
    // mode. This way we get the compile-time error checking in debug mode
    // and best performance in optimized code.

    // -----------------------------------------------------------------------------
    // Implementation of Register and FPURegister.

    #[allow(non_camel_case_types)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum RegisterCode {
        zero_reg,
        ra,
        tp,
        sp,
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
        t4,
        t5,
        t6,
        t7,
        t8,
        x_reg,
        fp,
        s0,
        s1,
        s2,
        s3,
        s4,
        s5,
        s6,
        s7,
        s8,
        kRegAfterLast,
    }

    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Register {
        base: RegisterBase<Register, { RegisterCode::kRegAfterLast as usize }>,
    }

    impl Register {
        pub const K_MANTISSA_OFFSET: i32 = 0;
        pub const K_EXPONENT_OFFSET: i32 = 4;

        pub const fn from_code(code: RegisterCode) -> Self {
            Register {
                base: RegisterBase::new(code as i32),
            }
        }

        pub const fn no_reg() -> Self {
            Register {
                base: RegisterBase::no_reg(),
            }
        }
    }

    impl Deref for Register {
        type Target = RegisterBase<Register, { RegisterCode::kRegAfterLast as usize }>;

        fn deref(&self) -> &Self::Target {
            &self.base
        }
    }

    impl DerefMut for Register {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.base
        }
    }

    impl std::fmt::Debug for Register {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Register {{ code: {} }}", self.code())
        }
    }

    macro_rules! declare_register {
        ($R:ident) => {
            pub const $R: Register = Register::from_code(RegisterCode::$R);
        };
    }
    general_registers!(declare_register);
    undefine_macro!(declare_register);

    macro_rules! undefine_macro {
        ($macro:ident) => {
            #[allow(unused_macros)]
            macro_rules! $macro {
                () => {
                    ()
                };
            }
        };
    }

    pub const no_reg: Register = Register::no_reg();

    pub fn to_number(reg: Register) -> i32 {
        reg.code()
    }

    pub fn to_register(num: i32) -> Register {
        if num >= 0 && num < RegisterCode::kRegAfterLast as i32 {
            unsafe { std::mem::transmute(num as u8) } // TODO: revisit this
        } else {
            no_reg
        }
    }

    pub fn reassign_register(source: &mut Register) -> Register {
        let result = *source;
        *source = Register::no_reg();
        result
    }

    pub const fn argument_padding_slots(argument_count: i32) -> i32 {
        0
    }

    pub const K_FP_ALIASING: AliasingKind = AliasingKind::kOverlap;
    pub const K_SIMD_MASK_REGISTERS: bool = false;

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum AliasingKind {
        kNoAlias,
        kMayAlias,
        kOverlap,
    }

    #[allow(non_camel_case_types)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum DoubleRegisterCode {
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
        f31,
        kDoubleAfterLast,
    }

    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct FPURegister {
        base: RegisterBase<FPURegister, { DoubleRegisterCode::kDoubleAfterLast as usize }>,
    }

    impl FPURegister {
        pub const fn from_code(code: DoubleRegisterCode) -> Self {
            FPURegister {
                base: RegisterBase::new(code as i32),
            }
        }

        pub fn low(&self) -> FPURegister {
            FPURegister::from_code(unsafe { std::mem::transmute(self.code() as u8) })
        }

        pub const fn no_reg() -> Self {
            FPURegister {
                base: RegisterBase::no_reg(),
            }
        }
    }

    impl Deref for FPURegister {
        type Target = RegisterBase<FPURegister, { DoubleRegisterCode::kDoubleAfterLast as usize }>;

        fn deref(&self) -> &Self::Target {
            &self.base
        }
    }

    impl DerefMut for FPURegister {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.base
        }
    }

    impl std::fmt::Debug for FPURegister {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "FPURegister {{ code: {} }}", self.code())
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum CFRegister {
        FCC0,
        FCC1,
        FCC2,
        FCC3,
        FCC4,
        FCC5,
        FCC6,
        FCC7,
    }

    pub type FloatRegister = FPURegister;
    pub type DoubleRegister = FPURegister;
    pub type Simd128Register = FPURegister;

    macro_rules! declare_double_register {
        ($R:ident) => {
            pub const $R: DoubleRegister = DoubleRegister::from_code(DoubleRegisterCode::$R);
        };
    }
    double_registers!(declare_double_register);
    undefine_macro!(declare_double_register);

    pub const no_dreg: DoubleRegister = DoubleRegister::no_reg();

    // Register aliases.
    // cp is assumed to be a callee saved register.
    pub const k_root_register: Register = s6;
    pub const cp: Register = s7;
    pub const k_scratch_reg: Register = s3;
    pub const k_scratch_reg2: Register = s4;
    pub const k_scratch_double_reg: DoubleRegister = f30;
    pub const k_scratch_double_reg2: DoubleRegister = f31;
    // FPU zero reg is often used to hold 0.0, but it's not hardwired to 0.0.
    pub const k_double_reg_zero: DoubleRegister = f29;

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct FPUControlRegister {
        reg_code: i32,
    }

    impl FPUControlRegister {
        pub fn is_valid(&self) -> bool {
            (self.reg_code >> 2) == 0
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

    pub const no_fpucreg: FPUControlRegister = FPUControlRegister {
        reg_code: kInvalidFPUControlRegister,
    };
    pub const FCSR: FPUControlRegister = FPUControlRegister {
        reg_code: kFCSRRegister,
    };
    pub const FCSR0: FPUControlRegister = FPUControlRegister {
        reg_code: kFCSRRegister,
    };
    pub const FCSR1: FPUControlRegister = FPUControlRegister {
        reg_code: kFCSRRegister + 1,
    };
    pub const FCSR2: FPUControlRegister = FPUControlRegister {
        reg_code: kFCSRRegister + 2,
    };
    pub const FCSR3: FPUControlRegister = FPUControlRegister {
        reg_code: kFCSRRegister + 3,
    };

    // Define {RegisterName} methods for the register types.
    macro_rules! define_register_names {
        ($type:ident, $macro:ident) => {
            impl $type {
                pub fn register_name(&self) -> &'static str {
                    match self.code() {
                        $(
                            #[allow(non_camel_case_types)]
                            code if code == RegisterCode::$name as i32 => stringify!($name),
                        )*
                        _ => "unknown",
                    }
                }
            }
        };
    }

    // LoongArch64 calling convention.
    pub const K_C_ARG_REGS: [Register; 8] = [a0, a1, a2, a3, a4, a5, a6, a7];
    pub const K_REGISTER_PASSED_ARGUMENTS: usize = K_C_ARG_REGS.len();
    pub const K_FP_REGISTER_PASSED_ARGUMENTS: usize = 8;

    pub const K_RETURN_REGISTER0: Register = a0;
    pub const K_RETURN_REGISTER1: Register = a1;
    pub const K_RETURN_REGISTER2: Register = a2;
    pub const K_JS_FUNCTION_REGISTER: Register = a1;
    pub const K_CONTEXT_REGISTER: Register = s7;
    pub const K_ALLOCATE_SIZE_REGISTER: Register = a0;
    pub const K_INTERPRETER_ACCUMULATOR_REGISTER: Register = a0;
    pub const K_INTERPRETER_BYTECODE_OFFSET_REGISTER: Register = t0;
    pub const K_INTERPRETER_BYTECODE_ARRAY_REGISTER: Register = t1;
    pub const K_INTERPRETER_DISPATCH_TABLE_REGISTER: Register = t2;

    pub const K_JAVASCRIPT_CALL_ARG_COUNT_REGISTER: Register = a0;
    pub const K_JAVASCRIPT_CALL_CODE_START_REGISTER: Register = a2;
    pub const K_JAVASCRIPT_CALL_TARGET_REGISTER: Register = K_JS_FUNCTION_REGISTER;
    pub const K_JAVASCRIPT_CALL_NEW_TARGET_REGISTER: Register = a3;
    pub const K_JAVASCRIPT_CALL_EXTRA_ARG1_REGISTER: Register = a2;
    pub const K_JAVASCRIPT_CALL_DISPATCH_HANDLE_REGISTER: Register = a4;

    pub const K_RUNTIME_CALL_FUNCTION_REGISTER: Register = a1;
    pub const K_RUNTIME_CALL_ARG_COUNT_REGISTER: Register = a0;
    pub const K_RUNTIME_CALL_ARGV_REGISTER: Register = a2;
    pub const K_WASM_IMPLICIT_ARG_REGISTER: Register = a7;
    pub const K_WASM_COMPILE_LAZY_FUNC_INDEX_REGISTER: Register = t0;
    pub const K_WASM_TRAP_HANDLER_FAULT_ADDRESS_REGISTER: Register = t6;

    pub const K_PTR_COMPR_CAGE_BASE_REGISTER: Register = s8;

    pub const K_FP_RETURN_REGISTER0: DoubleRegister = f0;
}