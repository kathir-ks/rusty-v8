// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod register_x64 {
    use crate::codegen::register_base::RegisterBase;

    macro_rules! general_registers {
        ($V:ident) => {
            $V!(rax);
            $V!(rcx);
            $V!(rdx);
            $V!(rbx);
            $V!(rsp);
            $V!(rbp);
            $V!(rsi);
            $V!(rdi);
            $V!(r8);
            $V!(r9);
            $V!(r10);
            $V!(r11);
            $V!(r12);
            $V!(r13);
            $V!(r14);
            $V!(r15);
        };
    }

    macro_rules! always_allocatable_general_registers {
        ($V:ident) => {
            $V!(rax);
            $V!(rbx);
            $V!(rdx);
            $V!(rcx);
            $V!(rsi);
            $V!(rdi);
            $V!(r8);
            $V!(r9);
            $V!(r11);
            $V!(r12);
            $V!(r15);
        };
    }

    macro_rules! maybe_allocatable_general_registers {
        ($V:ident) => {
            #[cfg(not(v8_compress_pointers))]
            $V!(r14);
        };
    }

    macro_rules! allocatable_general_registers {
        ($V:ident) => {
            always_allocatable_general_registers!($V);
            maybe_allocatable_general_registers!($V);
        };
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(u8)]
    pub enum RegisterCode {
        #[allow(non_camel_case_types)]
        kRegCode_rax,
        #[allow(non_camel_case_types)]
        kRegCode_rcx,
        #[allow(non_camel_case_types)]
        kRegCode_rdx,
        #[allow(non_camel_case_types)]
        kRegCode_rbx,
        #[allow(non_camel_case_types)]
        kRegCode_rsp,
        #[allow(non_camel_case_types)]
        kRegCode_rbp,
        #[allow(non_camel_case_types)]
        kRegCode_rsi,
        #[allow(non_camel_case_types)]
        kRegCode_rdi,
        #[allow(non_camel_case_types)]
        kRegCode_r8,
        #[allow(non_camel_case_types)]
        kRegCode_r9,
        #[allow(non_camel_case_types)]
        kRegCode_r10,
        #[allow(non_camel_case_types)]
        kRegCode_r11,
        #[allow(non_camel_case_types)]
        kRegCode_r12,
        #[allow(non_camel_case_types)]
        kRegCode_r13,
        #[allow(non_camel_case_types)]
        kRegCode_r14,
        #[allow(non_camel_case_types)]
        kRegCode_r15,
        kRegAfterLast,
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct Register {
        base: RegisterBase<Register, { RegisterCode::kRegAfterLast as usize }>,
    }

    impl Register {
        pub const fn is_byte_register(&self) -> bool {
            self.code() <= 3
        }

        // Return the high bit of the register code as a 0 or 1.  Used often
        // when constructing the REX prefix byte.
        pub const fn high_bit(&self) -> u8 {
            (self.code() >> 3) as u8
        }

        // Return the 3 low bits of the register code.  Used when encoding registers
        // in modR/M, SIB, and opcode bytes.
        pub const fn low_bits(&self) -> u8 {
            (self.code() & 0x7) as u8
        }

        pub const fn code(&self) -> u8 {
            self.base.code()
        }
    }

    impl RegisterBase<Register, { RegisterCode::kRegAfterLast as usize }> {
        const NO_REG_CODE: u8 = 255; // Choose a value outside the valid range

        pub const fn no_reg() -> Register {
            Register {
                base: RegisterBase::new(Self::NO_REG_CODE),
            }
        }
    }

    impl Register {
        pub const fn from_code(code: RegisterCode) -> Self {
            Register {
                base: RegisterBase::new(code as u8),
            }
        }
    }

    // Register that store tagged value. Tagged value is in compressed form when
    // pointer compression is enabled.
    #[derive(Copy, Clone, Debug)]
    pub struct TaggedRegister {
        reg_: Register,
    }

    impl TaggedRegister {
        pub const fn new(reg: Register) -> Self {
            TaggedRegister { reg_ }
        }
        pub const fn reg(&self) -> Register {
            self.reg_
        }
    }

    // Assign |source| value to |no_reg| and return the |source|'s previous value.
    pub fn reassign_register(source: &mut Register) -> Register {
        let result = *source;
        *source = Register::no_reg();
        result
    }

    macro_rules! declare_register {
        ($R:ident) => {
            #[allow(non_upper_case_globals)]
            pub const $R: Register = Register::from_code(RegisterCode::kRegCode_$R);
        };
    }
    general_registers!(declare_register);

    #[allow(non_upper_case_globals)]
    pub const no_reg: Register = Register::no_reg();

    pub const kNumRegs: usize = 16;

    #[cfg(target_os = "windows")]
    #[allow(non_upper_case_globals)]
    pub const kCArgRegs: [Register; 4] = [rcx, rdx, r8, r9];

    #[cfg(not(target_os = "windows"))]
    #[allow(non_upper_case_globals)]
    pub const kCArgRegs: [Register; 6] = [rdi, rsi, rdx, rcx, r8, r9];

    #[allow(non_upper_case_globals)]
    pub const kRegisterPassedArguments: usize = kCArgRegs.len();

    macro_rules! double_registers {
        ($V:ident) => {
            $V!(xmm0);
            $V!(xmm1);
            $V!(xmm2);
            $V!(xmm3);
            $V!(xmm4);
            $V!(xmm5);
            $V!(xmm6);
            $V!(xmm7);
            $V!(xmm8);
            $V!(xmm9);
            $V!(xmm10);
            $V!(xmm11);
            $V!(xmm12);
            $V!(xmm13);
            $V!(xmm14);
            $V!(xmm15);
        };
    }

    macro_rules! float_registers {
        ($V:ident) => {
            double_registers!($V);
        };
    }

    macro_rules! simd128_registers {
        ($V:ident) => {
            double_registers!($V);
        };
    }

    macro_rules! allocatable_double_registers {
        ($V:ident) => {
            $V!(xmm0);
            $V!(xmm1);
            $V!(xmm2);
            $V!(xmm3);
            $V!(xmm4);
            $V!(xmm5);
            $V!(xmm6);
            $V!(xmm7);
            $V!(xmm8);
            $V!(xmm9);
            $V!(xmm10);
            $V!(xmm11);
            $V!(xmm12);
            $V!(xmm13);
            $V!(xmm14);
        };
    }

    macro_rules! ymm_registers {
        ($V:ident) => {
            $V!(ymm0);
            $V!(ymm1);
            $V!(ymm2);
            $V!(ymm3);
            $V!(ymm4);
            $V!(ymm5);
            $V!(ymm6);
            $V!(ymm7);
            $V!(ymm8);
            $V!(ymm9);
            $V!(ymm10);
            $V!(ymm11);
            $V!(ymm12);
            $V!(ymm13);
            $V!(ymm14);
            $V!(ymm15);
        };
    }

    // Returns the number of padding slots needed for stack pointer alignment.
    pub const fn argument_padding_slots(_argument_count: i32) -> i32 {
        // No argument padding required.
        0
    }

    #[derive(PartialEq, Eq)]
    pub enum AliasingKind {
        kOverlap,
    }

    pub const kFPAliasing: AliasingKind = AliasingKind::kOverlap;
    pub const kSimdMaskRegisters: bool = false;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(u8)]
    pub enum DoubleRegisterCode {
        #[allow(non_camel_case_types)]
        kDoubleCode_xmm0,
        #[allow(non_camel_case_types)]
        kDoubleCode_xmm1,
        #[allow(non_camel_case_types)]
        kDoubleCode_xmm2,
        #[allow(non_camel_case_types)]
        kDoubleCode_xmm3,
        #[allow(non_camel_case_types)]
        kDoubleCode_xmm4,
        #[allow(non_camel_case_types)]
        kDoubleCode_xmm5,
        #[allow(non_camel_case_types)]
        kDoubleCode_xmm6,
        #[allow(non_camel_case_types)]
        kDoubleCode_xmm7,
        #[allow(non_camel_case_types)]
        kDoubleCode_xmm8,
        #[allow(non_camel_case_types)]
        kDoubleCode_xmm9,
        #[allow(non_camel_case_types)]
        kDoubleCode_xmm10,
        #[allow(non_camel_case_types)]
        kDoubleCode_xmm11,
        #[allow(non_camel_case_types)]
        kDoubleCode_xmm12,
        #[allow(non_camel_case_types)]
        kDoubleCode_xmm13,
        #[allow(non_camel_case_types)]
        kDoubleCode_xmm14,
        #[allow(non_camel_case_types)]
        kDoubleCode_xmm15,
        kDoubleAfterLast,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(u8)]
    pub enum YMMRegisterCode {
        #[allow(non_camel_case_types)]
        kYMMCode_ymm0,
        #[allow(non_camel_case_types)]
        kYMMCode_ymm1,
        #[allow(non_camel_case_types)]
        kYMMCode_ymm2,
        #[allow(non_camel_case_types)]
        kYMMCode_ymm3,
        #[allow(non_camel_case_types)]
        kYMMCode_ymm4,
        #[allow(non_camel_case_types)]
        kYMMCode_ymm5,
        #[allow(non_camel_case_types)]
        kYMMCode_ymm6,
        #[allow(non_camel_case_types)]
        kYMMCode_ymm7,
        #[allow(non_camel_case_types)]
        kYMMCode_ymm8,
        #[allow(non_camel_case_types)]
        kYMMCode_ymm9,
        #[allow(non_camel_case_types)]
        kYMMCode_ymm10,
        #[allow(non_camel_case_types)]
        kYMMCode_ymm11,
        #[allow(non_camel_case_types)]
        kYMMCode_ymm12,
        #[allow(non_camel_case_types)]
        kYMMCode_ymm13,
        #[allow(non_camel_case_types)]
        kYMMCode_ymm14,
        #[allow(non_camel_case_types)]
        kYMMCode_ymm15,
        kYMMAfterLast,
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct XMMRegister {
        base: RegisterBase<XMMRegister, { DoubleRegisterCode::kDoubleAfterLast as usize }>,
    }

    impl XMMRegister {
        // Return the high bit of the register code as a 0 or 1.  Used often
        // when constructing the REX prefix byte.
        pub const fn high_bit(&self) -> u8 {
            (self.code() >> 3) as u8
        }

        // Return the 3 low bits of the register code.  Used when encoding registers
        // in modR/M, SIB, and opcode bytes.
        pub const fn low_bits(&self) -> u8 {
            (self.code() & 0x7) as u8
        }

        pub const fn code(&self) -> u8 {
            self.base.code()
        }
    }

    impl RegisterBase<XMMRegister, { DoubleRegisterCode::kDoubleAfterLast as usize }> {
        const NO_REG_CODE: u8 = 255; // Choose a value outside the valid range

        pub const fn no_reg() -> XMMRegister {
            XMMRegister {
                base: RegisterBase::new(Self::NO_REG_CODE),
            }
        }
    }

    impl XMMRegister {
        pub const fn from_code(code: DoubleRegisterCode) -> Self {
            XMMRegister {
                base: RegisterBase::new(code as u8),
            }
        }
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct YMMRegister {
        base: XMMRegister,
    }

    impl YMMRegister {
        pub const fn from_code(code: YMMRegisterCode) -> Self {
            YMMRegister {
                base: XMMRegister::from_code(unsafe { std::mem::transmute(code) }),
            }
        }

        pub const fn from_xmm(xmm: XMMRegister) -> Self {
            YMMRegister { base: xmm }
        }

        pub const fn code(&self) -> u8 {
            self.base.code()
        }
    }

    pub type FloatRegister = XMMRegister;

    pub type DoubleRegister = XMMRegister;

    pub type Simd128Register = XMMRegister;

    pub type Simd256Register = YMMRegister;

    macro_rules! declare_double_register {
        ($R:ident) => {
            #[allow(non_upper_case_globals)]
            pub const $R: DoubleRegister =
                DoubleRegister::from_code(DoubleRegisterCode::kDoubleCode_$R);
        };
    }
    double_registers!(declare_double_register);
    #[allow(non_upper_case_globals)]
    pub const no_dreg: DoubleRegister = DoubleRegister::no_reg();

    macro_rules! declare_ymm_register {
        ($R:ident) => {
            #[allow(non_upper_case_globals)]
            pub const $R: YMMRegister = YMMRegister::from_code(YMMRegisterCode::kYMMCode_$R);
        };
    }
    ymm_registers!(declare_ymm_register);

    macro_rules! define_register_names {
        ($RegType:ident, $RegisterMacro:ident) => {
            macro_rules! define_name {
                ($name:ident) => {
                    impl $RegType {
                        #[allow(dead_code)]
                        pub const fn to_string(&self) -> &'static str {
                            stringify!($name)
                        }
                    }
                };
            }
            $RegisterMacro!(define_name);
        };
    }

    define_register_names!(Register, general_registers);
    define_register_names!(XMMRegister, double_registers);
    define_register_names!(YMMRegister, ymm_registers);

    // Give alias names to registers for calling conventions.
    #[allow(non_upper_case_globals)]
    pub const kStackPointerRegister: Register = rsp;
    #[allow(non_upper_case_globals)]
    pub const kReturnRegister0: Register = rax;
    #[allow(non_upper_case_globals)]
    pub const kReturnRegister1: Register = rdx;
    #[allow(non_upper_case_globals)]
    pub const kReturnRegister2: Register = r8;
    #[allow(non_upper_case_globals)]
    pub const kJSFunctionRegister: Register = rdi;
    #[allow(non_upper_case_globals)]
    pub const kContextRegister: Register = rsi;
    #[allow(non_upper_case_globals)]
    pub const kAllocateSizeRegister: Register = rdx;
    #[allow(non_upper_case_globals)]
    pub const kInterpreterAccumulatorRegister: Register = rax;
    #[allow(non_upper_case_globals)]
    pub const kInterpreterBytecodeOffsetRegister: Register = r9;
    #[allow(non_upper_case_globals)]
    pub const kInterpreterBytecodeArrayRegister: Register = r12;
    #[allow(non_upper_case_globals)]
    pub const kInterpreterDispatchTableRegister: Register = r15;

    #[allow(non_upper_case_globals)]
    pub const kJavaScriptCallArgCountRegister: Register = rax;
    #[allow(non_upper_case_globals)]
    pub const kJavaScriptCallCodeStartRegister: Register = rcx;
    #[allow(non_upper_case_globals)]
    pub const kJavaScriptCallTargetRegister: Register = kJSFunctionRegister;
    #[allow(non_upper_case_globals)]
    pub const kJavaScriptCallNewTargetRegister: Register = rdx;
    #[allow(non_upper_case_globals)]
    pub const kJavaScriptCallExtraArg1Register: Register = rbx;
    #[allow(non_upper_case_globals)]
    pub const kJavaScriptCallDispatchHandleRegister: Register = r15;

    #[allow(non_upper_case_globals)]
    pub const kRuntimeCallFunctionRegister: Register = rbx;
    #[allow(non_upper_case_globals)]
    pub const kRuntimeCallArgCountRegister: Register = rax;
    #[allow(non_upper_case_globals)]
    pub const kRuntimeCallArgvRegister: Register = r15;
    #[allow(non_upper_case_globals)]
    pub const kWasmImplicitArgRegister: Register = rsi;
    #[allow(non_upper_case_globals)]
    pub const kWasmTrapHandlerFaultAddressRegister: Register = r10;

    // Default scratch register used by MacroAssembler (and other code that needs
    // a spare register). The register isn't callee save, and not used by the
    // function calling convention.
    #[allow(non_upper_case_globals)]
    pub const kScratchRegister: Register = r10;
    #[allow(non_upper_case_globals)]
    pub const kScratchDoubleReg: XMMRegister = xmm15;
    #[allow(non_upper_case_globals)]
    pub const kScratchSimd256Reg: YMMRegister = ymm15;
    #[allow(non_upper_case_globals)]
    pub const kRootRegister: Register = r13; // callee save
    #[allow(non_upper_case_globals)]
    #[cfg(v8_compress_pointers)]
    pub const kPtrComprCageBaseRegister: Register = r14; // callee save
    #[allow(non_upper_case_globals)]
    #[cfg(not(v8_compress_pointers))]
    pub const kPtrComprCageBaseRegister: Register = no_reg;

    #[allow(non_upper_case_globals)]
    pub const kFPReturnRegister0: DoubleRegister = xmm0;
}

pub mod codegen {
    pub mod register_base {
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        pub struct RegisterBase<T, const N: usize> {
            code: u8,
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T, const N: usize> RegisterBase<T, N> {
            pub const fn new(code: u8) -> Self {
                RegisterBase {
                    code,
                    _phantom: std::marker::PhantomData,
                }
            }
            pub const fn code(&self) -> u8 {
                self.code
            }
        }
    }
}