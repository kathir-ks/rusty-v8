// Converted from V8 C++ source files:
// Header: register-base.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod bits {
        pub const fn countr_zero(x: u64) -> u32 {
            x.trailing_zeros()
        }
    }
    pub mod bounds {
        pub const kMaxInt8: i32 = i8::MAX as i32;
    }
}

pub mod common {
    pub mod globals {
        pub struct FlagList {}
    }
}

pub mod internal {
    use std::marker::PhantomData;
    use std::fmt;
    use std::fmt::Display;

    pub trait RegisterTrait: Copy + Clone + PartialEq + Eq + Display {
        fn code(self) -> i8;
        fn is_valid(self) -> bool;
        fn from_code(code: i8) -> Self;
        fn no_reg() -> Self;
        fn get_special_register_name(code: i32) -> &'static str;
        const K_NUM_REGISTERS: i8;
    }

    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct RegisterBase<SubType, const K_AFTER_LAST_REGISTER: i32> {
        reg_code_: i8,
        _phantom: PhantomData<SubType>,
    }

    impl<SubType, const K_AFTER_LAST_REGISTER: i32> RegisterBase<SubType, K_AFTER_LAST_REGISTER> {
        pub const K_CODE_NO_REG: i8 = -1;
        pub const K_NUM_REGISTERS: i8 = K_AFTER_LAST_REGISTER as i8;

        pub const fn no_reg() -> SubType
        where
            SubType: From<RegisterBase<SubType, K_AFTER_LAST_REGISTER>>,
        {
            RegisterBase {
                reg_code_: Self::K_CODE_NO_REG,
                _phantom: PhantomData,
            }.into()
        }

        pub const fn from_code(code: i8) -> SubType
        where
            SubType: From<RegisterBase<SubType, K_AFTER_LAST_REGISTER>>,
        {
            if code < 0 || code >= Self::K_NUM_REGISTERS {
                panic!("Code out of range");
            }
            RegisterBase {
                reg_code_: code,
                _phantom: PhantomData,
            }.into()
        }

        pub const fn is_valid(&self) -> bool {
            self.reg_code_ != Self::K_CODE_NO_REG
        }

        pub const fn code(&self) -> i8 {
             if cfg!(target_arch = "arm64") {
                // Arm64 uses kSPRegInternalCode which is > kNumRegisters.
                assert!(self.reg_code_ >= 0);
            } else {
                assert!(self.reg_code_ >= 0 && self.reg_code_ < Self::K_NUM_REGISTERS);
            }
            self.reg_code_
        }

        pub const fn new(code: i8) -> Self {
            Self {
                reg_code_: code,
                _phantom: PhantomData,
            }
        }

        pub fn get_special_register_name(code: i32) -> &'static str {
            "UNKNOWN"
        }
    }
    
    impl<SubType, const K_AFTER_LAST_REGISTER: i32> From<i8> for RegisterBase<SubType, K_AFTER_LAST_REGISTER> {
        fn from(code: i8) -> Self {
             if code < 0 || code >= K_AFTER_LAST_REGISTER as i8 {
                panic!("Code out of range");
            }
            RegisterBase {
                reg_code_: code,
                _phantom: PhantomData,
            }
        }
    }

    impl<SubType, const K_AFTER_LAST_REGISTER: i32> Display for RegisterBase<SubType, K_AFTER_LAST_REGISTER> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "RegisterBase(code: {})", self.reg_code_)
        }
    }
}
