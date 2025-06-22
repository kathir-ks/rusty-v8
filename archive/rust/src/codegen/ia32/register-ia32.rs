// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod register_base;

use register_base::RegisterBase;
use std::mem;

pub mod internal {
    use super::*;

    macro_rules! define_registers {
        ($vis:vis enum $name:ident { $($reg:ident,)* }) => {
            #[derive(Debug, Copy, Clone, PartialEq, Eq)]
            $vis enum $name {
                $($reg,)*
                AfterLast,
            }

            impl $name {
                pub const COUNT: usize = match $name::AfterLast {
                    $( $name::$reg => unreachable!(), )*
                    $name::AfterLast => {
                        let mut count = 0;
                        $( let _ = $name::$reg; count += 1; )*
                        count
                    }
                };
            }
        }
    }

    define_registers! {
        pub enum RegisterCode {
            eax,
            ecx,
            edx,
            ebx,
            esp,
            ebp,
            esi,
            edi,
        }
    }

    define_registers! {
        pub enum DoubleCode {
            xmm0,
            xmm1,
            xmm2,
            xmm3,
            xmm4,
            xmm5,
            xmm6,
            xmm7,
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Register(RegisterBase<Register, { RegisterCode::COUNT }>);

    impl Register {
        pub const fn from_code(code: RegisterCode) -> Self {
            Register(RegisterBase::new(code as usize))
        }

        pub fn code(&self) -> usize {
            self.0.code()
        }

        pub fn is_byte_register(&self) -> bool {
            self.code() <= 3
        }

        pub const fn no_reg() -> Self {
            Register(RegisterBase::no_reg())
        }
    }

    static_assertions::assert_eq_size!(Register, i32);
    static_assertions::assert_impl_all!(Register: Copy, Clone, Send, Sync, Unpin);

    // Assign |source| value to |no_reg| and return the |source|'s previous value.
    pub fn reassign_register(source: &mut Register) -> Register {
        let result = *source;
        *source = Register::no_reg();
        result
    }

    macro_rules! define_register_constants {
        ($($reg:ident),*) => {
            $(
                pub const $reg: Register = Register::from_code(RegisterCode::$reg);
            )*
        };
    }

    define_register_constants!(eax, ecx, edx, ebx, esp, ebp, esi, edi);

    pub const no_reg: Register = Register::no_reg();

    // Returns the number of padding slots needed for stack pointer alignment.
    pub const fn argument_padding_slots(_argument_count: i32) -> i32 {
        // No argument padding required.
        0
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum AliasingKind {
        kDisjoint,
        kOverlap,
    }

    pub const k_fp_aliasing: AliasingKind = AliasingKind::kOverlap;
    pub const k_simd_mask_registers: bool = false;


    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct XMMRegister(RegisterBase<XMMRegister, {DoubleCode::COUNT}>);

    impl XMMRegister {
        pub const fn from_code(code: DoubleCode) -> Self {
            XMMRegister(RegisterBase::new(code as usize))
        }

        pub const fn no_reg() -> Self {
            XMMRegister(RegisterBase::no_reg())
        }
    }

    pub type FloatRegister = XMMRegister;
    pub type DoubleRegister = XMMRegister;
    pub type Simd128Register = XMMRegister;

    macro_rules! define_double_register_constants {
        ($($reg:ident),*) => {
            $(
                pub const $reg: DoubleRegister = DoubleRegister::from_code(DoubleCode::$reg);
            )*
        };
    }

    define_double_register_constants!(xmm0, xmm1, xmm2, xmm3, xmm4, xmm5, xmm6, xmm7);

    pub const no_dreg: DoubleRegister = DoubleRegister::no_reg();

    // Note that the bit values must match those used in actual instruction encoding
    pub const K_NUM_REGS: i32 = 8;

    macro_rules! define_register_names {
        ($reg_type:ident, $($reg:ident),*) => {
            impl $reg_type {
                pub fn register_name(&self) -> &'static str {
                    match self.0.code() {
                        $(
                            {
                                let code = RegisterCode::$reg as usize;
                                if self.0.code() == code {
                                    stringify!($reg)
                                } else
                                {
                                    continue;
                                }
                            }
                        )*
                        _ => "invalid",
                    }
                }
            }
        };
    }
    //The following macro call requires GENERAL_REGISTERS and DOUBLE_REGISTERS which have been unrolled into the enum definitions.
    //In a real implementation these would be unified as associated types or constants of a trait.
    //define_register_names!(Register, GENERAL_REGISTERS);
    //define_register_names!(XMMRegister, DOUBLE_REGISTERS);

    impl Register {
        pub fn register_name(&self) -> &'static str {
            match self.0.code() {
                0 => "eax",
                1 => "ecx",
                2 => "edx",
                3 => "ebx",
                4 => "esp",
                5 => "ebp",
                6 => "esi",
                7 => "edi",
                _ => "invalid",
            }
        }
    }

    impl XMMRegister {
        pub fn register_name(&self) -> &'static str {
            match self.0.code() {
                0 => "xmm0",
                1 => "xmm1",
                2 => "xmm2",
                3 => "xmm3",
                4 => "xmm4",
                5 => "xmm5",
                6 => "xmm6",
                7 => "xmm7",
                _ => "invalid",
            }
        }
    }

    // Give alias names to registers for calling conventions.
    pub const K_RETURN_REGISTER_0: Register = eax;
    pub const K_RETURN_REGISTER_1: Register = edx;
    pub const K_RETURN_REGISTER_2: Register = edi;
    pub const K_JS_FUNCTION_REGISTER: Register = edi;
    pub const K_CONTEXT_REGISTER: Register = esi;
    pub const K_ALLOCATE_SIZE_REGISTER: Register = edx;
    pub const K_INTERPRETER_ACCUMULATOR_REGISTER: Register = eax;
    pub const K_INTERPRETER_BYTECODE_OFFSET_REGISTER: Register = edx;
    pub const K_INTERPRETER_BYTECODE_ARRAY_REGISTER: Register = edi;
    pub const K_INTERPRETER_DISPATCH_TABLE_REGISTER: Register = esi;

    pub const K_JAVA_SCRIPT_CALL_ARG_COUNT_REGISTER: Register = eax;
    pub const K_JAVA_SCRIPT_CALL_CODE_START_REGISTER: Register = ecx;
    pub const K_JAVA_SCRIPT_CALL_TARGET_REGISTER: Register = K_JS_FUNCTION_REGISTER;
    pub const K_JAVA_SCRIPT_CALL_NEW_TARGET_REGISTER: Register = edx;
    // DispatchHandle is only needed for the sandbox which is not available on Ia32.
    pub const K_JAVA_SCRIPT_CALL_DISPATCH_HANDLE_REGISTER: Register = no_reg;

    // The ExtraArg1Register not part of the real JS calling convention and is
    // mostly there to simplify consistent interface descriptor definitions across
    // platforms. Note that on ia32 it aliases kJavaScriptCallCodeStartRegister.
    pub const K_JAVA_SCRIPT_CALL_EXTRA_ARG_1_REGISTER: Register = ecx;

    pub const K_RUNTIME_CALL_FUNCTION_REGISTER: Register = edx;
    pub const K_RUNTIME_CALL_ARG_COUNT_REGISTER: Register = eax;
    pub const K_RUNTIME_CALL_ARGV_REGISTER: Register = ecx;
    pub const K_WASM_IMPLICIT_ARG_REGISTER: Register = esi;
    pub const K_WASM_COMPILE_LAZY_FUNC_INDEX_REGISTER: Register = edi;

    pub const K_ROOT_REGISTER: Register = ebx;

    pub const K_FP_RETURN_REGISTER_0: DoubleRegister = xmm0;
    pub const K_SCRATCH_DOUBLE_REG: DoubleRegister = xmm7;
}