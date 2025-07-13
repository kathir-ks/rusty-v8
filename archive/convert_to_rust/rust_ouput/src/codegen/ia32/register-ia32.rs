// Converted from V8 C++ source files:
// Header: register-ia32.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod register_ia32 {
    use std::fmt;
    use std::marker::Copy;
    use std::sync::Arc;

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum AliasingKind {
        kNoOverlap,
        kOverlap,
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum RegisterCode {
        kRegCode_eax,
        kRegCode_ecx,
        kRegCode_edx,
        kRegCode_ebx,
        kRegCode_esp,
        kRegCode_ebp,
        kRegCode_esi,
        kRegCode_edi,
        kRegAfterLast,
    }

    impl fmt::Display for RegisterCode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                RegisterCode::kRegCode_eax => write!(f, "eax"),
                RegisterCode::kRegCode_ecx => write!(f, "ecx"),
                RegisterCode::kRegCode_edx => write!(f, "edx"),
                RegisterCode::kRegCode_ebx => write!(f, "ebx"),
                RegisterCode::kRegCode_esp => write!(f, "esp"),
                RegisterCode::kRegCode_ebp => write!(f, "ebp"),
                RegisterCode::kRegCode_esi => write!(f, "esi"),
                RegisterCode::kRegCode_edi => write!(f, "edi"),
                RegisterCode::kRegAfterLast => write!(f, "kRegAfterLast"),
            }
        }
    }

    pub trait RegisterBaseTrait<T, const N: usize>
    where
        T: Copy + Eq + PartialEq,
    {
        fn code(&self) -> i32;
        fn from_code(code: i32) -> T;
        fn no_reg() -> T;
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub struct Register {
        code: i32,
    }

    impl Register {
        pub fn is_byte_register(&self) -> bool {
            self.code() <= 3
        }

        pub const fn from_code(code: i32) -> Self {
            Register { code }
        }

        pub const fn no_reg() -> Self {
            Register { code: -1 }
        }

        pub fn code(&self) -> i32 {
            self.code
        }
    }

    impl fmt::Display for Register {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self.code() {
                0 => write!(f, "eax"),
                1 => write!(f, "ecx"),
                2 => write!(f, "edx"),
                3 => write!(f, "ebx"),
                4 => write!(f, "esp"),
                5 => write!(f, "ebp"),
                6 => write!(f, "esi"),
                7 => write!(f, "edi"),
                _ => write!(f, "no_reg"),
            }
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

    define_register!(eax);
    define_register!(ecx);
    define_register!(edx);
    define_register!(ebx);
    define_register!(esp);
    define_register!(ebp);
    define_register!(esi);
    define_register!(edi);

    pub const no_reg: Register = Register::no_reg();

    pub const fn argument_padding_slots(_argument_count: i32) -> i32 {
        0
    }

    pub const kFPAliasing: AliasingKind = AliasingKind::kOverlap;
    pub const kSimdMaskRegisters: bool = false;

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum DoubleCode {
        kDoubleCode_xmm0,
        kDoubleCode_xmm1,
        kDoubleCode_xmm2,
        kDoubleCode_xmm3,
        kDoubleCode_xmm4,
        kDoubleCode_xmm5,
        kDoubleCode_xmm6,
        kDoubleCode_xmm7,
        kDoubleAfterLast,
    }

    impl fmt::Display for DoubleCode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                DoubleCode::kDoubleCode_xmm0 => write!(f, "xmm0"),
                DoubleCode::kDoubleCode_xmm1 => write!(f, "xmm1"),
                DoubleCode::kDoubleCode_xmm2 => write!(f, "xmm2"),
                DoubleCode::kDoubleCode_xmm3 => write!(f, "xmm3"),
                DoubleCode::kDoubleCode_xmm4 => write!(f, "xmm4"),
                DoubleCode::kDoubleCode_xmm5 => write!(f, "xmm5"),
                DoubleCode::kDoubleCode_xmm6 => write!(f, "xmm6"),
                DoubleCode::kDoubleCode_xmm7 => write!(f, "xmm7"),
                DoubleCode::kDoubleAfterLast => write!(f, "kDoubleAfterLast"),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub struct XMMRegister {
        code: i32,
    }

    impl XMMRegister {
        pub const fn from_code(code: i32) -> Self {
            XMMRegister { code }
        }

        pub const fn no_reg() -> Self {
            XMMRegister { code: -1 }
        }

        pub fn code(&self) -> i32 {
            self.code
        }
    }

    impl fmt::Display for XMMRegister {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self.code() {
                0 => write!(f, "xmm0"),
                1 => write!(f, "xmm1"),
                2 => write!(f, "xmm2"),
                3 => write!(f, "xmm3"),
                4 => write!(f, "xmm4"),
                5 => write!(f, "xmm5"),
                6 => write!(f, "xmm6"),
                7 => write!(f, "xmm7"),
                _ => write!(f, "no_xmm"),
            }
        }
    }

    pub type FloatRegister = XMMRegister;
    pub type DoubleRegister = XMMRegister;
    pub type Simd128Register = XMMRegister;

    macro_rules! define_double_register {
        ($r:ident) => {
            pub const $r: DoubleRegister =
                DoubleRegister::from_code(DoubleCode::kDoubleCode_$r as i32);
        };
    }

    define_double_register!(xmm0);
    define_double_register!(xmm1);
    define_double_register!(xmm2);
    define_double_register!(xmm3);
    define_double_register!(xmm4);
    define_double_register!(xmm5);
    define_double_register!(xmm6);
    define_double_register!(xmm7);

    pub const no_dreg: DoubleRegister = DoubleRegister::no_reg();

    pub const kNumRegs: i32 = 8;

    macro_rules! define_register_names {
        ($reg_type:ident, $registers:ident) => {
            impl $reg_type {
                pub fn eax(&self) -> bool {
                    self.code == RegisterCode::kRegCode_eax as i32
                }
                pub fn ecx(&self) -> bool {
                    self.code == RegisterCode::kRegCode_ecx as i32
                }
                pub fn edx(&self) -> bool {
                    self.code == RegisterCode::kRegCode_edx as i32
                }
                pub fn ebx(&self) -> bool {
                    self.code == RegisterCode::kRegCode_ebx as i32
                }
                pub fn esp(&self) -> bool {
                    self.code == RegisterCode::kRegCode_esp as i32
                }
                pub fn ebp(&self) -> bool {
                    self.code == RegisterCode::kRegCode_ebp as i32
                }
                pub fn esi(&self) -> bool {
                    self.code == RegisterCode::kRegCode_esi as i32
                }
                pub fn edi(&self) -> bool {
                    self.code == RegisterCode::kRegCode_edi as i32
                }
                pub fn xmm0(&self) -> bool {
                    self.code == DoubleCode::kDoubleCode_xmm0 as i32
                }
                pub fn xmm1(&self) -> bool {
                    self.code == DoubleCode::kDoubleCode_xmm1 as i32
                }
                pub fn xmm2(&self) -> bool {
                    self.code == DoubleCode::kDoubleCode_xmm2 as i32
                }
                pub fn xmm3(&self) -> bool {
                    self.code == DoubleCode::kDoubleCode_xmm3 as i32
                }
                pub fn xmm4(&self) -> bool {
                    self.code == DoubleCode::kDoubleCode_xmm4 as i32
                }
                pub fn xmm5(&self) -> bool {
                    self.code == DoubleCode::kDoubleCode_xmm5 as i32
                }
                pub fn xmm6(&self) -> bool {
                    self.code == DoubleCode::kDoubleCode_xmm6 as i32
                }
                pub fn xmm7(&self) -> bool {
                    self.code == DoubleCode::kDoubleCode_xmm7 as i32
                }
            }
        };
    }
    define_register_names!(Register, GENERAL_REGISTERS);
    define_register_names!(XMMRegister, DOUBLE_REGISTERS);

    pub const kReturnRegister0: Register = eax;
    pub const kReturnRegister1: Register = edx;
    pub const kReturnRegister2: Register = edi;
    pub const kJSFunctionRegister: Register = edi;
    pub const kContextRegister: Register = esi;
    pub const kAllocateSizeRegister: Register = edx;
    pub const kInterpreterAccumulatorRegister: Register = eax;
    pub const kInterpreterBytecodeOffsetRegister: Register = edx;
    pub const kInterpreterBytecodeArrayRegister: Register = edi;
    pub const kInterpreterDispatchTableRegister: Register = esi;
    pub const kJavaScriptCallArgCountRegister: Register = eax;
    pub const kJavaScriptCallCodeStartRegister: Register = ecx;
    pub const kJavaScriptCallTargetRegister: Register = kJSFunctionRegister;
    pub const kJavaScriptCallNewTargetRegister: Register = edx;
    pub const kJavaScriptCallDispatchHandleRegister: Register = no_reg;
    pub const kJavaScriptCallExtraArg1Register: Register = ecx;
    pub const kRuntimeCallFunctionRegister: Register = edx;
    pub const kRuntimeCallArgCountRegister: Register = eax;
    pub const kRuntimeCallArgvRegister: Register = ecx;
    pub const kWasmImplicitArgRegister: Register = esi;
    pub const kWasmCompileLazyFuncIndexRegister: Register = edi;
    pub const kRootRegister: Register = ebx;
    pub const kFPReturnRegister0: DoubleRegister = xmm0;
    pub const kScratchDoubleReg: DoubleRegister = xmm7;
}
