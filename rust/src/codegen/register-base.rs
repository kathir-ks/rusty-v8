// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//use std::fmt;
//use std::marker::PhantomData;
//use std::convert::TryFrom;
//use std::convert::TryInto;

pub mod register_base {
    use std::fmt;
    use std::marker::PhantomData;
    use std::convert::TryFrom;

    /// Base type for CPU Registers.
    ///
    /// 1) We would prefer to use an enum for registers, but enum values are
    /// assignment-compatible with int, which has caused code-generation bugs.
    ///
    /// 2) By not using an enum, we are possibly preventing the compiler from
    /// doing certain constant folds, which may significantly reduce the
    /// code generated for some assembly instructions (because they boil down
    /// to a few constants). If this is a problem, we could change the code
    /// such that we use an enum in optimized mode, and the class in debug
    /// mode. This way we get the compile-time error checking in debug mode
    /// and best performance in optimized code.
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct RegisterBase<SubType, const K_AFTER_LAST_REGISTER: usize> {
        reg_code_: i8,
        _phantom: PhantomData<SubType>,
    }

    impl<SubType, const K_AFTER_LAST_REGISTER: usize> RegisterBase<SubType, K_AFTER_LAST_REGISTER> {
        pub const K_CODE_NO_REG: i8 = -1;
        pub const K_NUM_REGISTERS: usize = K_AFTER_LAST_REGISTER;

        pub const fn no_reg() -> RegisterBase<SubType, K_AFTER_LAST_REGISTER> {
            RegisterBase { reg_code_: Self::K_CODE_NO_REG, _phantom: PhantomData }
        }

        pub const fn from_code(code: i8) -> RegisterBase<SubType, K_AFTER_LAST_REGISTER> {
            assert!(code >= 0 && (code as usize) < Self::K_NUM_REGISTERS);
            RegisterBase { reg_code_: code, _phantom: PhantomData }
        }

        pub const fn is_valid(&self) -> bool {
            self.reg_code_ != Self::K_CODE_NO_REG
        }

        pub const fn code(&self) -> i8 {
            #[cfg(target_arch = "aarch64")]
            {
                // Arm64 uses kSPRegInternalCode which is > kNumRegisters.
                assert!(self.reg_code_ >= 0);
            }
            #[cfg(not(target_arch = "aarch64"))]
            {
                assert!(self.reg_code_ >= 0 && (self.reg_code_ as usize) < Self::K_NUM_REGISTERS);
            }
            self.reg_code_
        }

        // Used to print the name of some special registers.
        pub fn get_special_register_name(_code: i32) -> &'static str {
            "UNKNOWN"
        }
    }

    impl<SubType, const K_AFTER_LAST_REGISTER: usize> RegisterBase<SubType, K_AFTER_LAST_REGISTER> {
        pub const fn new(code: i8) -> Self {
          RegisterBase{reg_code_:code, _phantom: PhantomData}
        }
    }

    const K_MAX_INT8: usize = 127; //Rust does not expose MAX for prims, const needed for static_assert
    static_assertions::const_assert!(K_AFTER_LAST_REGISTER <= K_MAX_INT8);

    // Helper macros to define a {RegisterName} method based on a macro list
    // containing all names.

    #[macro_export]
    macro_rules! define_register_names {
        ($reg_type:ty, $list:ident) => {
            pub fn register_name(reg: $reg_type) -> &'static str {
                const NAMES: [&'static str; <$reg_type>::K_NUM_REGISTERS] = $list!(define_register_names_name);
                if reg.is_valid() {
                    NAMES[reg.code() as usize]
                } else {
                    "invalid"
                }
            }
        };
    }

    #[macro_export]
    macro_rules! define_register_names_name {
        ($name:ident) => {
            stringify!($name)
        };
    }

    pub trait RegisterName {
        fn register_name(&self) -> &'static str;
    }

    impl<SubType, const K_AFTER_LAST_REGISTER: usize> fmt::Display for RegisterBase<SubType, K_AFTER_LAST_REGISTER> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
           // write!(f, "{}", self.register_name()) //Cannot call trait method "register_name" without specifying the implementing type
            write!(f, "RegisterBase")
        }
    }
}