// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod register_s390 {
    use std::fmt;

    /// Macro to define general purpose registers.
    macro_rules! define_general_registers {
        ($V:ident) => {
            $V!(r0);
            $V!(r1);
            $V!(r2);
            $V!(r3);
            $V!(r4);
            $V!(r5);
            $V!(r6);
            $V!(r7);
            $V!(r8);
            $V!(r9);
            $V!(r10);
            $V!(fp);
            $V!(ip);
            $V!(r13);
            $V!(r14);
            $V!(sp);
        };
    }

    /// Macro to define always allocatable general purpose registers.
    macro_rules! define_always_allocatable_general_registers {
        ($V:ident) => {
            $V!(r2);
            $V!(r3);
            $V!(r4);
            $V!(r5);
            $V!(r6);
            $V!(r7);
            $V!(r8);
            $V!(r13);
        };
    }

    /// Macro to define maybe allocatable general purpose registers.
    macro_rules! define_maybe_allocatable_general_registers {
        ($V:ident) => {
            $V!(r9);
        };
    }

    /// Macro to define allocatable general purpose registers.
    macro_rules! define_allocatable_general_registers {
        ($V:ident) => {
            define_always_allocatable_general_registers!($V);
            define_maybe_allocatable_general_registers!($V);
        };
    }

    /// Macro to define double registers.
    macro_rules! define_double_registers {
        ($V:ident) => {
            $V!(d0);
            $V!(d1);
            $V!(d2);
            $V!(d3);
            $V!(d4);
            $V!(d5);
            $V!(d6);
            $V!(d7);
            $V!(d8);
            $V!(d9);
            $V!(d10);
            $V!(d11);
            $V!(d12);
            $V!(d13);
            $V!(d14);
            $V!(d15);
        };
    }

    /// Macro to define float registers.
    macro_rules! define_float_registers {
        ($V:ident) => {
            define_double_registers!($V);
        };
    }

    /// Macro to define SIMD128 registers.
    macro_rules! define_simd128_registers {
        ($V:ident) => {
            define_double_registers!($V);
        };
    }

    /// Macro to define allocatable double registers.
    macro_rules! define_allocatable_double_registers {
        ($V:ident) => {
            $V!(d1);
            $V!(d2);
            $V!(d3);
            $V!(d4);
            $V!(d5);
            $V!(d6);
            $V!(d7);
            $V!(d8);
            $V!(d9);
            $V!(d10);
            $V!(d11);
            $V!(d12);
            $V!(d15);
            $V!(d0);
        };
    }

    /// Macro to define C registers.
    macro_rules! define_c_registers {
        ($V:ident) => {
            $V!(cr0);
            $V!(cr1);
            $V!(cr2);
            $V!(cr3);
            $V!(cr4);
            $V!(cr5);
            $V!(cr6);
            $V!(cr7);
            $V!(cr8);
            $V!(cr9);
            $V!(cr10);
            $V!(cr11);
            $V!(cr12);
            $V!(cr15);
        };
    }

    // The following constants describe the stack frame linkage area as
    // defined by the ABI.

    // z/OS XPLINK 64-bit frame shape (without the 2k stack bias):
    // [0] Backchain
    // [1] Environment
    // [2] Entry Point
    // [3] Return Address (XPLINK)
    // [4] GPR8
    // [5] GPR9
    // ...
    // [10] GPR14 / RA Slot
    // [11] GPR15 / SP Slot
    // [12] Reserved
    // [13] Reserved
    // [14] Debug Area
    // [15] Reserved
    // [16] Register Arg1
    // [17] Register Arg2
    // [18] Register Arg3
    // [19] Register Arg4
    // [20] Register Arg5

    // Since z/OS port of V8 follows the register assignment from Linux in the
    // JavaScript context, JS code will set up r2-r6 as parameter registers,
    // with 6th+ parameters passed on the stack, when calling C functions.
    // XPLINK allocates stack slots for all parameters regardless of whether
    // they are passed in registers. To ensure stack slots are available to
    // store register parameters back to the stack for XPLINK calls, we include
    // slots for the 5 "register" arguments (r2-r6 as noted above) as part of
    // the required stack frame slots. Additional params being passed on the
    // stack will continue to grow from slot 22 and beyond.
    //
    // The 2k stack bias for XPLINK will be adjusted from SP into r4 (system
    // stack pointer) by the CallCFunctionHelper and CEntryStub right before
    // the actual native call.
    // zLinux ABI requires caller frames to include sufficient space for
    // callee preserved register save area.
    pub const K_CALLEE_REGISTER_SAVE_AREA_SIZE: i32 = 160;

    pub const K_NUM_REQUIRED_STACK_FRAME_SLOTS: i32 = 20;
    pub const K_STACK_FRAME_SP_SLOT: i32 = 15;
    pub const K_STACK_FRAME_RA_SLOT: i32 = 14;
    pub const K_STACK_FRAME_EXTRA_PARAM_SLOT: i32 = 20;

    macro_rules! define_register_code_enum {
        ($enum_name:ident, $macro_name:ident) => {
            #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
            #[repr(u8)]
            pub enum $enum_name {
                #[allow(non_camel_case_types)]
                $macro_name(r0),
                #[allow(non_camel_case_types)]
                $macro_name(r1),
                #[allow(non_camel_case_types)]
                $macro_name(r2),
                #[allow(non_camel_case_types)]
                $macro_name(r3),
                #[allow(non_camel_case_types)]
                $macro_name(r4),
                #[allow(non_camel_case_types)]
                $macro_name(r5),
                #[allow(non_camel_case_types)]
                $macro_name(r6),
                #[allow(non_camel_case_types)]
                $macro_name(r7),
                #[allow(non_camel_case_types)]
                $macro_name(r8),
                #[allow(non_camel_case_types)]
                $macro_name(r9),
                #[allow(non_camel_case_types)]
                $macro_name(r10),
                #[allow(non_camel_case_types)]
                $macro_name(fp),
                #[allow(non_camel_case_types)]
                $macro_name(ip),
                #[allow(non_camel_case_types)]
                $macro_name(r13),
                #[allow(non_camel_case_types)]
                $macro_name(r14),
                #[allow(non_camel_case_types)]
                $macro_name(sp),
                kRegAfterLast,
            }
        };
    }

    define_register_code_enum!(RegisterCodeEnum, kRegCode_);

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct Register(RegisterCodeEnum);

    impl Register {
        pub const fn from_code(code: RegisterCodeEnum) -> Self {
            Register(code)
        }

        pub const fn code(&self) -> RegisterCodeEnum {
            self.0
        }

        pub const fn no_reg() -> Self {
            Register(RegisterCodeEnum::kRegAfterLast)
        }
    }

    impl fmt::Display for Register {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self.0 {
                RegisterCodeEnum::kRegCode_r0 => write!(f, "r0"),
                RegisterCodeEnum::kRegCode_r1 => write!(f, "r1"),
                RegisterCodeEnum::kRegCode_r2 => write!(f, "r2"),
                RegisterCodeEnum::kRegCode_r3 => write!(f, "r3"),
                RegisterCodeEnum::kRegCode_r4 => write!(f, "r4"),
                RegisterCodeEnum::kRegCode_r5 => write!(f, "r5"),
                RegisterCodeEnum::kRegCode_r6 => write!(f, "r6"),
                RegisterCodeEnum::kRegCode_r7 => write!(f, "r7"),
                RegisterCodeEnum::kRegCode_r8 => write!(f, "r8"),
                RegisterCodeEnum::kRegCode_r9 => write!(f, "r9"),
                RegisterCodeEnum::kRegCode_r10 => write!(f, "r10"),
                RegisterCodeEnum::kRegCode_fp => write!(f, "fp"),
                RegisterCodeEnum::kRegCode_ip => write!(f, "ip"),
                RegisterCodeEnum::kRegCode_r13 => write!(f, "r13"),
                RegisterCodeEnum::kRegCode_r14 => write!(f, "r14"),
                RegisterCodeEnum::kRegCode_sp => write!(f, "sp"),
                RegisterCodeEnum::kRegAfterLast => write!(f, "no_reg"),
            }
        }
    }

    pub fn reassign_register(source: &mut Register) -> Register {
        let result = *source;
        *source = Register::no_reg();
        result
    }

    macro_rules! define_registers {
        ($define_register:ident) => {
            const $define_register: Register = Register::from_code(RegisterCodeEnum::kRegCode_$define_register);
        };
    }

    define_general_registers!(define_registers);

    // Register aliases
    pub const K_ROOT_REGISTER: Register = r10; // Roots array pointer.
    pub const K_PTR_COMPR_CAGE_BASE_REGISTER: Register = K_ROOT_REGISTER; // callee save
    pub const CP: Register = r13; // JavaScript context pointer.

    // s390x calling convention
    pub const K_C_ARG_REGS: [Register; 5] = [r2, r3, r4, r5, r6];
    pub const K_REGISTER_PASSED_ARGUMENTS: usize = K_C_ARG_REGS.len();

    /// Returns the number of padding slots needed for stack pointer alignment.
    pub const fn argument_padding_slots(_argument_count: i32) -> i32 {
        // No argument padding required.
        0
    }

    pub const K_FP_ALIASING: AliasingKind = AliasingKind::Overlap;
    pub const K_SIMD_MASK_REGISTERS: bool = false;

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum AliasingKind {
        Overlap,
    }

    macro_rules! define_double_register_code_enum {
        ($enum_name:ident, $macro_name:ident) => {
            #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
            #[repr(u8)]
            pub enum $enum_name {
                #[allow(non_camel_case_types)]
                $macro_name(d0),
                #[allow(non_camel_case_types)]
                $macro_name(d1),
                #[allow(non_camel_case_types)]
                $macro_name(d2),
                #[allow(non_camel_case_types)]
                $macro_name(d3),
                #[allow(non_camel_case_types)]
                $macro_name(d4),
                #[allow(non_camel_case_types)]
                $macro_name(d5),
                #[allow(non_camel_case_types)]
                $macro_name(d6),
                #[allow(non_camel_case_types)]
                $macro_name(d7),
                #[allow(non_camel_case_types)]
                $macro_name(d8),
                #[allow(non_camel_case_types)]
                $macro_name(d9),
                #[allow(non_camel_case_types)]
                $macro_name(d10),
                #[allow(non_camel_case_types)]
                $macro_name(d11),
                #[allow(non_camel_case_types)]
                $macro_name(d12),
                #[allow(non_camel_case_types)]
                $macro_name(d13),
                #[allow(non_camel_case_types)]
                $macro_name(d14),
                #[allow(non_camel_case_types)]
                $macro_name(d15),
                kDoubleAfterLast,
            }
        };
    }

    define_double_register_code_enum!(DoubleRegisterCodeEnum, kDoubleCode_);

    /// Double word VFP register.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DoubleRegister(DoubleRegisterCodeEnum);

    impl DoubleRegister {
        pub const K_SIZE_IN_BYTES: i32 = 8;

        pub const fn from_code(code: DoubleRegisterCodeEnum) -> Self {
            DoubleRegister(code)
        }

        pub const fn code(&self) -> DoubleRegisterCodeEnum {
            self.0
        }

        pub const fn no_reg() -> Self {
            DoubleRegister(DoubleRegisterCodeEnum::kDoubleAfterLast)
        }

        /// This function differs from kNumRegisters by returning the number of double
        /// registers supported by the current CPU, while kNumRegisters always returns
        /// 32.
        #[inline]
        pub const fn supported_register_count() -> i32 {
            16 // Assuming all 16 are supported.  This might need CPU feature detection.
        }
    }

    impl fmt::Display for DoubleRegister {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self.0 {
                DoubleRegisterCodeEnum::kDoubleCode_d0 => write!(f, "d0"),
                DoubleRegisterCodeEnum::kDoubleCode_d1 => write!(f, "d1"),
                DoubleRegisterCodeEnum::kDoubleCode_d2 => write!(f, "d2"),
                DoubleRegisterCodeEnum::kDoubleCode_d3 => write!(f, "d3"),
                DoubleRegisterCodeEnum::kDoubleCode_d4 => write!(f, "d4"),
                DoubleRegisterCodeEnum::kDoubleCode_d5 => write!(f, "d5"),
                DoubleRegisterCodeEnum::kDoubleCode_d6 => write!(f, "d6"),
                DoubleRegisterCodeEnum::kDoubleCode_d7 => write!(f, "d7"),
                DoubleRegisterCodeEnum::kDoubleCode_d8 => write!(f, "d8"),
                DoubleRegisterCodeEnum::kDoubleCode_d9 => write!(f, "d9"),
                DoubleRegisterCodeEnum::kDoubleCode_d10 => write!(f, "d10"),
                DoubleRegisterCodeEnum::kDoubleCode_d11 => write!(f, "d11"),
                DoubleRegisterCodeEnum::kDoubleCode_d12 => write!(f, "d12"),
                DoubleRegisterCodeEnum::kDoubleCode_d13 => write!(f, "d13"),
                DoubleRegisterCodeEnum::kDoubleCode_d14 => write!(f, "d14"),
                DoubleRegisterCodeEnum::kDoubleCode_d15 => write!(f, "d15"),
                DoubleRegisterCodeEnum::kDoubleAfterLast => write!(f, "no_dreg"),
            }
        }
    }

    pub type FloatRegister = DoubleRegister;
    pub type Simd128Register = DoubleRegister;

    macro_rules! define_double_registers_const {
        ($define_register:ident) => {
            const $define_register: DoubleRegister = DoubleRegister::from_code(DoubleRegisterCodeEnum::kDoubleCode_$define_register);
        };
    }

    define_double_registers!(define_double_registers_const);

    pub const K_DOUBLE_REG_ZERO: DoubleRegister = d14;
    pub const K_SCRATCH_DOUBLE_REG: DoubleRegister = d13;

    // TODO(john.yan) Define SIMD registers.

    macro_rules! define_c_register_code_enum {
        ($enum_name:ident, $macro_name:ident) => {
            #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
            #[repr(u8)]
            pub enum $enum_name {
                #[allow(non_camel_case_types)]
                $macro_name(cr0),
                #[allow(non_camel_case_types)]
                $macro_name(cr1),
                #[allow(non_camel_case_types)]
                $macro_name(cr2),
                #[allow(non_camel_case_types)]
                $macro_name(cr3),
                #[allow(non_camel_case_types)]
                $macro_name(cr4),
                #[allow(non_camel_case_types)]
                $macro_name(cr5),
                #[allow(non_camel_case_types)]
                $macro_name(cr6),
                #[allow(non_camel_case_types)]
                $macro_name(cr7),
                #[allow(non_camel_case_types)]
                $macro_name(cr8),
                #[allow(non_camel_case_types)]
                $macro_name(cr9),
                #[allow(non_camel_case_types)]
                $macro_name(cr10),
                #[allow(non_camel_case_types)]
                $macro_name(cr11),
                #[allow(non_camel_case_types)]
                $macro_name(cr12),
                #[allow(non_camel_case_types)]
                $macro_name(cr15),
                kCAfterLast,
            }
        };
    }

    define_c_register_code_enum!(CRegisterCodeEnum, kCCode_);

    /// Coprocessor register
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct CRegister(CRegisterCodeEnum);

    impl CRegister {
        pub const fn from_code(code: CRegisterCodeEnum) -> Self {
            CRegister(code)
        }

        pub const fn code(&self) -> CRegisterCodeEnum {
            self.0
        }

        pub const fn no_reg() -> Self {
            CRegister(CRegisterCodeEnum::kCAfterLast)
        }
    }

    impl fmt::Display for CRegister {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self.0 {
                CRegisterCodeEnum::kCCode_cr0 => write!(f, "cr0"),
                CRegisterCodeEnum::kCCode_cr1 => write!(f, "cr1"),
                CRegisterCodeEnum::kCCode_cr2 => write!(f, "cr2"),
                CRegisterCodeEnum::kCCode_cr3 => write!(f, "cr3"),
                CRegisterCodeEnum::kCCode_cr4 => write!(f, "cr4"),
                CRegisterCodeEnum::kCCode_cr5 => write!(f, "cr5"),
                CRegisterCodeEnum::kCCode_cr6 => write!(f, "cr6"),
                CRegisterCodeEnum::kCCode_cr7 => write!(f, "cr7"),
                CRegisterCodeEnum::kCCode_cr8 => write!(f, "cr8"),
                CRegisterCodeEnum::kCCode_cr9 => write!(f, "cr9"),
                CRegisterCodeEnum::kCCode_cr10 => write!(f, "cr10"),
                CRegisterCodeEnum::kCCode_cr11 => write!(f, "cr11"),
                CRegisterCodeEnum::kCCode_cr12 => write!(f, "cr12"),
                CRegisterCodeEnum::kCCode_cr15 => write!(f, "cr15"),
                CRegisterCodeEnum::kCAfterLast => write!(f, "no_creg"),
            }
        }
    }

    macro_rules! define_c_registers_const {
        ($define_register:ident) => {
            const $define_register: CRegister = CRegister::from_code(CRegisterCodeEnum::kCCode_$define_register);
        };
    }

    define_c_registers!(define_c_registers_const);

    pub const NO_CREG: CRegister = CRegister::no_reg();

    pub fn to_register(num: i32) -> Register {
        match num {
            0 => r0,
            1 => r1,
            2 => r2,
            3 => r3,
            4 => r4,
            5 => r5,
            6 => r6,
            7 => r7,
            8 => r8,
            9 => r9,
            10 => r10,
            11 => fp,
            12 => ip,
            13 => r13,
            14 => r14,
            15 => sp,
            _ => Register::no_reg(),
        }
    }

    pub const K_STACK_POINTER_REGISTER: Register = sp;
    pub const K_RETURN_REGISTER_0: Register = r2;
    pub const K_RETURN_REGISTER_1: Register = r3;
    pub const K_RETURN_REGISTER_2: Register = r4;
    pub const K_JS_FUNCTION_REGISTER: Register = r3;
    pub const K_CONTEXT_REGISTER: Register = r13;
    pub const K_ALLOCATE_SIZE_REGISTER: Register = r3;
    pub const K_INTERPRETER_ACCUMULATOR_REGISTER: Register = r2;
    pub const K_INTERPRETER_BYTECODE_OFFSET_REGISTER: Register = r6;
    pub const K_INTERPRETER_BYTECODE_ARRAY_REGISTER: Register = r7;
    pub const K_INTERPRETER_DISPATCH_TABLE_REGISTER: Register = r8;

    pub const K_JAVASCRIPT_CALL_ARG_COUNT_REGISTER: Register = r2;
    pub const K_JAVASCRIPT_CALL_CODE_START_REGISTER: Register = r4;
    pub const K_JAVASCRIPT_CALL_TARGET_REGISTER: Register = K_JS_FUNCTION_REGISTER;
    pub const K_JAVASCRIPT_CALL_NEW_TARGET_REGISTER: Register = r5;
    pub const K_JAVASCRIPT_CALL_EXTRA_ARG_1_REGISTER: Register = r4;
    // DispatchHandle is only needed for the sandbox which is not available on
    // s390x.
    pub const K_JAVASCRIPT_CALL_DISPATCH_HANDLE_REGISTER: Register = Register::no_reg();

    pub const K_RUNTIME_CALL_FUNCTION_REGISTER: Register = r3;
    pub const K_RUNTIME_CALL_ARG_COUNT_REGISTER: Register = r2;
    pub const K_RUNTIME_CALL_ARGV_REGISTER: Register = r4;
    pub const K_WASM_IMPLICIT_ARG_REGISTER: Register = r6;
    pub const K_WASM_COMPILE_LAZY_FUNC_INDEX_REGISTER: Register = r7;

    pub const K_FP_RETURN_REGISTER_0: DoubleRegister = d0;

}