// Converted from V8 C++ source files:
// Header: register-loong64.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod register_loong64 {
    use crate::codegen::register_base::RegisterBase;
    use std::sync::Arc;

    // clang-format off
    // macro_rules! general_registers {
    //     ($V:ident) => {
    //         $V!(zero_reg); $V!(ra); $V!(tp); $V!(sp);
    //         $V!(a0); $V!(a1); $V!(a2); $V!(a3); $V!(a4); $V!(a5); $V!(a6); $V!(a7);
    //         $V!(t0); $V!(t1); $V!(t2); $V!(t3); $V!(t4); $V!(t5); $V!(t6); $V!(t7); $V!(t8);
    //         $V!(x_reg); $V!(fp);
    //         $V!(s0); $V!(s1); $V!(s2); $V!(s3); $V!(s4); $V!(s5); $V!(s6); $V!(s7); $V!(s8);
    //     }
    // }

    // macro_rules! always_allocatable_general_registers {
    //     ($V:ident) => {
    //         $V!(a0); $V!(a1); $V!(a2); $V!(a3); $V!(a4); $V!(a5); $V!(a6); $V!(a7);
    //         $V!(t0); $V!(t1); $V!(t2); $V!(t3); $V!(t4); $V!(t5);
    //         $V!(s0); $V!(s1); $V!(s2); $V!(s3); $V!(s4); $V!(s5); $V!(s7);
    //     }
    // }

    // macro_rules! maybe_allocatable_general_registers {
    //     ($V:ident) => {
    //         #[cfg(not(v8_compress_pointers))]
    //         $V!(s8);
    //     }
    // }

    // macro_rules! allocatable_general_registers {
    //     ($V:ident) => {
    //         always_allocatable_general_registers!($V);
    //         maybe_allocatable_general_registers!($V);
    //     }
    // }

    // macro_rules! double_registers {
    //     ($V:ident) => {
    //         $V!(f0); $V!(f1); $V!(f2); $V!(f3); $V!(f4); $V!(f5); $V!(f6); $V!(f7);
    //         $V!(f8); $V!(f9); $V!(f10); $V!(f11); $V!(f12); $V!(f13); $V!(f14); $V!(f15);
    //         $V!(f16); $V!(f17); $V!(f18); $V!(f19); $V!(f20); $V!(f21); $V!(f22); $V!(f23);
    //         $V!(f24); $V!(f25); $V!(f26); $V!(f27); $V!(f28); $V!(f29); $V!(f30); $V!(f31);
    //     }
    // }

    // macro_rules! float_registers {
    //     ($V:ident) => {
    //         double_registers!($V);
    //     }
    // }

    // macro_rules! simd128_registers {
    //     ($V:ident) => {
    //         $V!(w0); $V!(w1); $V!(w2); $V!(w3); $V!(w4); $V!(w5); $V!(w6); $V!(w7);
    //         $V!(w8); $V!(w9); $V!(w10); $V!(w11); $V!(w12); $V!(w13); $V!(w14); $V!(w15);
    //         $V!(w16); $V!(w17); $V!(w18); $V!(w19); $V!(w20); $V!(w21); $V!(w22); $V!(w23);
    //         $V!(w24); $V!(w25); $V!(w26); $V!(w27); $V!(w28); $V!(w29); $V!(w30); $V!(w31);
    //     }
    // }

    // macro_rules! allocatable_double_registers {
    //     ($V:ident) => {
    //         $V!(f0); $V!(f1); $V!(f2); $V!(f3); $V!(f4); $V!(f5); $V!(f6); $V!(f7);
    //         $V!(f8); $V!(f9); $V!(f10); $V!(f11); $V!(f12); $V!(f13); $V!(f14); $V!(f15);
    //         $V!(f16); $V!(f17); $V!(f18); $V!(f19); $V!(f20); $V!(f21); $V!(f22); $V!(f23);
    //         $V!(f24); $V!(f25); $V!(f26); $V!(f27); $V!(f28);
    //     }
    // }
    // clang-format on

    // Note that the bit values must match those used in actual instruction
    // encoding.
    pub const K_NUM_REGS: usize = 32;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum RegisterCode {
        kRegCode_zero_reg,
        kRegCode_ra,
        kRegCode_tp,
        kRegCode_sp,
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
        kRegCode_t4,
        kRegCode_t5,
        kRegCode_t6,
        kRegCode_t7,
        kRegCode_t8,
        kRegCode_x_reg,
        kRegCode_fp,
        kRegCode_s0,
        kRegCode_s1,
        kRegCode_s2,
        kRegCode_s3,
        kRegCode_s4,
        kRegCode_s5,
        kRegCode_s6,
        kRegCode_s7,
        kRegCode_s8,
        kRegAfterLast,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Register {
        code: i32,
    }

    impl Register {
        pub const K_MANTISSA_OFFSET: usize = 0;
        pub const K_EXPONENT_OFFSET: usize = 4;

        pub fn from_code(code: RegisterCode) -> Self {
            Register { code: code as i32 }
        }

        pub const fn no_reg() -> Self {
            Register { code: -1 }
        }

        pub fn is_valid(&self) -> bool {
            self.code >= 0 && (self.code as usize) < RegisterCode::kRegAfterLast as usize
        }

        pub fn code(&self) -> i32 {
            self.code
        }
    }
    
    // s7: context register
    // s3: scratch register
    // s4: scratch register 2
    pub const zero_reg: Register = Register { code: RegisterCode::kRegCode_zero_reg as i32 };
    pub const ra: Register = Register { code: RegisterCode::kRegCode_ra as i32 };
    pub const tp: Register = Register { code: RegisterCode::kRegCode_tp as i32 };
    pub const sp: Register = Register { code: RegisterCode::kRegCode_sp as i32 };
    pub const a0: Register = Register { code: RegisterCode::kRegCode_a0 as i32 };
    pub const a1: Register = Register { code: RegisterCode::kRegCode_a1 as i32 };
    pub const a2: Register = Register { code: RegisterCode::kRegCode_a2 as i32 };
    pub const a3: Register = Register { code: RegisterCode::kRegCode_a3 as i32 };
    pub const a4: Register = Register { code: RegisterCode::kRegCode_a4 as i32 };
    pub const a5: Register = Register { code: RegisterCode::kRegCode_a5 as i32 };
    pub const a6: Register = Register { code: RegisterCode::kRegCode_a6 as i32 };
    pub const a7: Register = Register { code: RegisterCode::kRegCode_a7 as i32 };
    pub const t0: Register = Register { code: RegisterCode::kRegCode_t0 as i32 };
    pub const t1: Register = Register { code: RegisterCode::kRegCode_t1 as i32 };
    pub const t2: Register = Register { code: RegisterCode::kRegCode_t2 as i32 };
    pub const t3: Register = Register { code: RegisterCode::kRegCode_t3 as i32 };
    pub const t4: Register = Register { code: RegisterCode::kRegCode_t4 as i32 };
    pub const t5: Register = Register { code: RegisterCode::kRegCode_t5 as i32 };
    pub const t6: Register = Register { code: RegisterCode::kRegCode_t6 as i32 };
    pub const t7: Register = Register { code: RegisterCode::kRegCode_t7 as i32 };
    pub const t8: Register = Register { code: RegisterCode::kRegCode_t8 as i32 };
    pub const x_reg: Register = Register { code: RegisterCode::kRegCode_x_reg as i32 };
    pub const fp: Register = Register { code: RegisterCode::kRegCode_fp as i32 };
    pub const s0: Register = Register { code: RegisterCode::kRegCode_s0 as i32 };
    pub const s1: Register = Register { code: RegisterCode::kRegCode_s1 as i32 };
    pub const s2: Register = Register { code: RegisterCode::kRegCode_s2 as i32 };
    pub const s3: Register = Register { code: RegisterCode::kRegCode_s3 as i32 };
    pub const s4: Register = Register { code: RegisterCode::kRegCode_s4 as i32 };
    pub const s5: Register = Register { code: RegisterCode::kRegCode_s5 as i32 };
    pub const s6: Register = Register { code: RegisterCode::kRegCode_s6 as i32 };
    pub const s7: Register = Register { code: RegisterCode::kRegCode_s7 as i32 };
    pub const s8: Register = Register { code: RegisterCode::kRegCode_s8 as i32 };

    pub fn to_number(reg: Register) -> i32 {
        reg.code()
    }

    pub fn to_register(num: i32) -> Register {
        if num < 0 || num >= RegisterCode::kRegAfterLast as i32 {
            Register::no_reg()
        } else {
            unsafe {
                let code: RegisterCode = std::mem::transmute(num as u8);
                Register::from_code(code)
            }
        }
    }

    // Assign |source| value to |no_reg| and return the |source|'s previous value.
    pub fn reassign_register(source: &mut Register) -> Register {
        let result = *source;
        *source = Register::no_reg();
        result
    }

    // Returns the number of padding slots needed for stack pointer alignment.
    pub const fn argument_padding_slots(argument_count: i32) -> i32 {
        // No argument padding required.
        0
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum AliasingKind {
        kOverlap,
    }

    pub const K_FP_ALIASING: AliasingKind = AliasingKind::kOverlap;
    pub const K_SIMD_MASK_REGISTERS: bool = false;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct FPURegister {
        code: i32,
    }

    impl FPURegister {
        pub fn from_code(code: DoubleRegisterCode) -> Self {
            FPURegister { code: code as i32 }
        }

        pub const fn no_reg() -> Self {
            FPURegister { code: -1 }
        }
        
        pub fn code(&self) -> i32 {
            self.code
        }

        pub fn low(&self) -> FPURegister {
            FPURegister::from_code(unsafe { std::mem::transmute(self.code() as u8)})
        }
    }

    // Condition Flag Register
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

    pub const f0: DoubleRegister = DoubleRegister { code: DoubleRegisterCode::kDoubleCode_f0 as i32 };
    pub const f1: DoubleRegister = DoubleRegister { code: DoubleRegisterCode::kDoubleCode_f1 as i32 };
    pub const f2: DoubleRegister = DoubleRegister { code: DoubleRegisterCode::kDoubleCode_f2 as i32 };
    pub const f3: DoubleRegister = DoubleRegister { code: DoubleRegisterCode::kDoubleCode_f3 as i32 };
    pub const f4: DoubleRegister = DoubleRegister { code: DoubleRegisterCode::kDoubleCode_f4 as i32 };
    pub const f5: DoubleRegister = DoubleRegister { code: DoubleRegisterCode::kDoubleCode_f5 as i32 };
    pub const f6: DoubleRegister = DoubleRegister { code: DoubleRegisterCode::kDoubleCode_f6 as i32 };
    pub const f7: DoubleRegister = DoubleRegister { code: DoubleRegisterCode::kDoubleCode_f7 as i32 };
    pub const f8: DoubleRegister = DoubleRegister { code: DoubleRegisterCode::kDoubleCode_f8 as i32 };
    pub const f9: DoubleRegister = DoubleRegister { code: DoubleRegisterCode::kDoubleCode_f9 as i32 };
    pub const f10: DoubleRegister = DoubleRegister { code: DoubleRegisterCode::kDoubleCode_f10 as i32 };
    pub const f11: DoubleRegister = DoubleRegister { code: DoubleRegisterCode::kDoubleCode_f11 as i32 };
    pub const f12: DoubleRegister = DoubleRegister { code: DoubleRegisterCode::kDoubleCode_f12 as i32 };
    pub const f13: DoubleRegister = DoubleRegister { code: DoubleRegisterCode::kDoubleCode_f13 as i32 };
    pub const f14: DoubleRegister = DoubleRegister { code: DoubleRegisterCode::kDoubleCode_f14 as i32 };
    pub const f15: DoubleRegister = DoubleRegister { code: DoubleRegisterCode::kDoubleCode_f15 as i32 };
    pub const f16: DoubleRegister = DoubleRegister { code: DoubleRegisterCode::kDoubleCode_f16 as i32 };
    pub const f17: DoubleRegister = DoubleRegister { code: DoubleRegisterCode::kDoubleCode_f17 as i32 };
    pub const f18: DoubleRegister = DoubleRegister { code: DoubleRegisterCode::kDoubleCode_f18 as i32 };
    pub const f19: DoubleRegister = DoubleRegister { code: DoubleRegisterCode::kDoubleCode_f19 as i32 };
    pub const f20: DoubleRegister = DoubleRegister { code: DoubleRegisterCode::kDoubleCode_f20 as i32 };
    pub const f21: DoubleRegister = DoubleRegister { code: DoubleRegisterCode::kDoubleCode_f21 as i32 };
    pub const f22: DoubleRegister = DoubleRegister { code: DoubleRegisterCode::kDoubleCode_f22 as i32 };
    pub const f23: DoubleRegister = DoubleRegister { code: DoubleRegisterCode::kDoubleCode_f23 as i32 };
    pub const f24: DoubleRegister = DoubleRegister { code: DoubleRegisterCode::kDoubleCode_f24 as i32 };
    pub const f25: DoubleRegister = DoubleRegister { code: DoubleRegisterCode::kDoubleCode_f25 as i32 };
    pub const f26: DoubleRegister = DoubleRegister { code: DoubleRegisterCode::kDoubleCode_f26 as i32 };
    pub const f27: DoubleRegister = DoubleRegister { code: DoubleRegisterCode::kDoubleCode_f27 as i32 };
    pub const f28: DoubleRegister = DoubleRegister { code: DoubleRegisterCode::kDoubleCode_f28 as i32 };
    pub const f29: DoubleRegister = DoubleRegister { code: DoubleRegisterCode::kDoubleCode_f29 as i32 };
    pub const f30: DoubleRegister = DoubleRegister { code: DoubleRegisterCode::kDoubleCode_f30 as i32 };
    pub const f31: DoubleRegister = DoubleRegister { code: DoubleRegisterCode::kDoubleCode_f31 as i32 };

    pub const no_dreg: DoubleRegister = DoubleRegister::no_reg();

    // Register aliases.
    // cp is assumed to be a callee saved register.
    pub const kRootRegister: Register = s6;
    pub const cp: Register = s7;
    pub const kScratchReg: Register = s3;
    pub const kScratchReg2: Register = s4;
    pub const kScratchDoubleReg: DoubleRegister = f30;
    pub const kScratchDoubleReg2: DoubleRegister = f31;
    // FPU zero reg is often used to hold 0.0, but it's not hardwired to 0.0.
    pub const kDoubleRegZero: DoubleRegister = f29;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct FPUControlRegister {
        pub reg_code: i32,
    }

    impl FPUControlRegister {
        pub fn is_valid(&self) -> bool {
             (self.reg_code >> 2) == 0
        }
        pub fn is(&self, creg: FPUControlRegister) -> bool {
             self.reg_code == creg.reg_code
        }
        pub fn code(&self) -> i32 {
            //DCHECK(is_valid());
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

    pub const kInvalidFPUControlRegister: i32 = -1;
    pub const kFCSRRegister: i32 = 0;

    pub const no_fpucreg: FPUControlRegister = FPUControlRegister { reg_code: kInvalidFPUControlRegister };
    pub const FCSR: FPUControlRegister = FPUControlRegister { reg_code: kFCSRRegister };
    pub const FCSR0: FPUControlRegister = FPUControlRegister { reg_code: kFCSRRegister };
    pub const FCSR1: FPUControlRegister = FPUControlRegister { reg_code: kFCSRRegister + 1 };
    pub const FCSR2: FPUControlRegister = FPUControlRegister { reg_code: kFCSRRegister + 2 };
    pub const FCSR3: FPUControlRegister = FPUControlRegister { reg_code: kFCSRRegister + 3 };

    // Define {RegisterName} methods for the register types.
    //DEFINE_REGISTER_NAMES(Register, GENERAL_REGISTERS)
    //DEFINE_REGISTER_NAMES(FPURegister, DOUBLE_REGISTERS)

    // LoongArch64 calling convention.
    pub const K_C_ARG_REGS: [Register; 8] = [a0, a1, a2, a3, a4, a5, a6, a7];
    pub const K_REGISTER_PASSED_ARGUMENTS: usize = K_C_ARG_REGS.len();
    pub const K_FP_REGISTER_PASSED_ARGUMENTS: i32 = 8;

    pub const K_RETURN_REGISTER_0: Register = a0;
    pub const K_RETURN_REGISTER_1: Register = a1;
    pub const K_RETURN_REGISTER_2: Register = a2;
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
    pub const K_JAVASCRIPT_CALL_EXTRA_ARG_1_REGISTER: Register = a2;
    pub const K_JAVASCRIPT_CALL_DISPATCH_HANDLE_REGISTER: Register = a4;

    pub const K_RUNTIME_CALL_FUNCTION_REGISTER: Register = a1;
    pub const K_RUNTIME_CALL_ARG_COUNT_REGISTER: Register = a0;
    pub const K_RUNTIME_CALL_ARGV_REGISTER: Register = a2;
    pub const K_WASM_IMPLICIT_ARG_REGISTER: Register = a7;
    pub const K_WASM_COMPILE_LAZY_FUNC_INDEX_REGISTER: Register = t0;
    pub const K_WASM_TRAP_HANDLER_FAULT_ADDRESS_REGISTER: Register = t6;

    #[cfg(v8_compress_pointers)]
    pub const K_PTR_COMPR_CAGE_BASE_REGISTER: Register = s8;
    #[cfg(not(v8_compress_pointers))]
    pub const K_PTR_COMPR_CAGE_BASE_REGISTER: Register = Register::no_reg();

    pub const K_FP_RETURN_REGISTER_0: DoubleRegister = f0;
}
