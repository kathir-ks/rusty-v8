// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod register_riscv {
    use std::convert::TryFrom;
    use std::fmt;

    /// Dummy RegisterBase struct, as the original C++ RegisterBase class
    /// is not provided.  Replace with the actual definition if available.
    pub struct RegisterBase<T, const N: usize> {
        code: usize,
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T, const N: usize> RegisterBase<T, N> {
        const fn new(code: usize) -> Self {
            Self {
                code,
                _phantom: std::marker::PhantomData,
            }
        }

        fn code(&self) -> usize {
            self.code
        }

        fn is_valid(&self) -> bool {
            self.code < N
        }

        fn no_reg() -> Self {
            Self {
                code: N,
                _phantom: std::marker::PhantomData,
            }
        }
    }

    /// Represents a RISC-V register.
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub struct Register {
        base: RegisterBase<Register, { RegisterCode::kRegAfterLast as usize }>,
    }

    impl Register {
        pub const MANTISSA_OFFSET: i32 = {
            #[cfg(target_endian = "little")]
            {
                0
            }
            #[cfg(target_endian = "big")]
            {
                4
            }
            #[cfg(not(any(target_endian = "little", target_endian = "big")))]
            {
                compile_error!("Unknown endianness");
            }
        };

        pub const EXPONENT_OFFSET: i32 = {
            #[cfg(target_endian = "little")]
            {
                4
            }
            #[cfg(target_endian = "big")]
            {
                0
            }
            #[cfg(not(any(target_endian = "little", target_endian = "big")))]
            {
                compile_error!("Unknown endianness");
            }
        };

        const fn from_code(code: usize) -> Self {
            Self {
                base: RegisterBase::new(code),
            }
        }

        pub fn code(&self) -> usize {
            self.base.code()
        }

        pub fn is_valid(&self) -> bool {
            self.base.is_valid()
        }

        pub fn no_reg() -> Self {
            Self {
                base: RegisterBase::no_reg(),
            }
        }
    }

    /// Represents a RISC-V floating-point register.
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub struct FPURegister {
        base: RegisterBase<FPURegister, { DoubleRegisterCode::kDoubleAfterLast as usize }>,
    }

    impl FPURegister {
        const fn from_code(code: usize) -> Self {
            Self {
                base: RegisterBase::new(code),
            }
        }

        pub fn code(&self) -> usize {
            self.base.code()
        }

        pub fn is_valid(&self) -> bool {
            self.base.is_valid()
        }

        pub fn no_reg() -> Self {
            Self {
                base: RegisterBase::no_reg(),
            }
        }

        pub fn low(&self) -> Self {
            FPURegister::from_code(self.code())
        }

        pub fn high(&self) -> Self {
            FPURegister::from_code(self.code() + 1)
        }

        // FIXME(riscv64): In Rvv, Vector regs is different from Float Regs. But in
        // this cl, in order to facilitate modification, it is assumed that the vector
        // register and floating point register are shared.
        pub fn to_v(&self) -> VRegister {
            assert!(self.code() < (VRegisterCode::kVRAfterLast as usize));
            VRegister::from_code(self.code())
        }
    }

    /// Represents a RISC-V vector register.
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub struct VRegister {
        base: RegisterBase<VRegister, { VRegisterCode::kVRAfterLast as usize }>,
    }

    impl VRegister {
        const fn from_code(code: usize) -> Self {
            Self {
                base: RegisterBase::new(code),
            }
        }

        pub fn code(&self) -> usize {
            self.base.code()
        }

        pub fn is_valid(&self) -> bool {
            self.base.is_valid()
        }

        pub fn no_reg() -> Self {
            Self {
                base: RegisterBase::no_reg(),
            }
        }
    }

    pub type FloatRegister = FPURegister;
    pub type DoubleRegister = FPURegister;
    pub type Simd128Register = VRegister;

    /// Defines the general-purpose registers.
    macro_rules! define_general_registers {
    ($macro:ident) => {
        $macro!(zero_reg);
        $macro!(ra);
        $macro!(sp);
        $macro!(gp);
        $macro!(tp);
        $macro!(t0);
        $macro!(t1);
        $macro!(t2);
        $macro!(fp);
        $macro!(s1);
        $macro!(a0);
        $macro!(a1);
        $macro!(a2);
        $macro!(a3);
        $macro!(a4);
        $macro!(a5);
        $macro!(a6);
        $macro!(a7);
        $macro!(s2);
        $macro!(s3);
        $macro!(s4);
        $macro!(s5);
        $macro!(s6);
        $macro!(s7);
        $macro!(s8);
        $macro!(s9);
        $macro!(s10);
        $macro!(s11);
        $macro!(t3);
        $macro!(t4);
        $macro!(t5);
        $macro!(t6);
    };
}

    /// Defines the always allocatable general-purpose registers.
    macro_rules! define_always_allocatable_general_registers {
    ($macro:ident) => {
        $macro!(a0);
        $macro!(a1);
        $macro!(a2);
        $macro!(a3);
        $macro!(a4);
        $macro!(a5);
        $macro!(a6);
        $macro!(a7);
        $macro!(t0);
        $macro!(t1);
        $macro!(t2);
        $macro!(t4);
        $macro!(s7);
        $macro!(s8);
        $macro!(s9);
        $macro!(s10);
    };
}

    /// Defines the maybe allocatable general-purpose registers based on pointer compression.
    macro_rules! define_maybe_allocatable_general_registers {
    ($macro:ident) => {
        #[cfg(not(feature = "v8_compress_pointers"))]
        $macro!(s11);
    };
}

    /// Defines all allocatable general-purpose registers.
    macro_rules! define_allocatable_general_registers {
    ($macro:ident) => {
        define_always_allocatable_general_registers!($macro);
        define_maybe_allocatable_general_registers!($macro);
    };
}

    /// Defines the double-precision floating-point registers.
    macro_rules! define_double_registers {
    ($macro:ident) => {
        $macro!(ft0);
        $macro!(ft1);
        $macro!(ft2);
        $macro!(ft3);
        $macro!(ft4);
        $macro!(ft5);
        $macro!(ft6);
        $macro!(ft7);
        $macro!(fs0);
        $macro!(fs1);
        $macro!(fa0);
        $macro!(fa1);
        $macro!(fa2);
        $macro!(fa3);
        $macro!(fa4);
        $macro!(fa5);
        $macro!(fa6);
        $macro!(fa7);
        $macro!(fs2);
        $macro!(fs3);
        $macro!(fs4);
        $macro!(fs5);
        $macro!(fs6);
        $macro!(fs7);
        $macro!(fs8);
        $macro!(fs9);
        $macro!(fs10);
        $macro!(fs11);
        $macro!(ft8);
        $macro!(ft9);
        $macro!(ft10);
        $macro!(ft11);
    };
}

    /// Defines the vector registers.
    macro_rules! define_vector_registers {
    ($macro:ident) => {
        $macro!(v0);
        $macro!(v1);
        $macro!(v2);
        $macro!(v3);
        $macro!(v4);
        $macro!(v5);
        $macro!(v6);
        $macro!(v7);
        $macro!(v8);
        $macro!(v9);
        $macro!(v10);
        $macro!(v11);
        $macro!(v12);
        $macro!(v13);
        $macro!(v14);
        $macro!(v15);
        $macro!(v16);
        $macro!(v17);
        $macro!(v18);
        $macro!(v19);
        $macro!(v20);
        $macro!(v21);
        $macro!(v22);
        $macro!(v23);
        $macro!(v24);
        $macro!(v25);
        $macro!(v26);
        $macro!(v27);
        $macro!(v28);
        $macro!(v29);
        $macro!(v30);
        $macro!(v31);
    };
}

    /// Defines the allocatable 128-bit SIMD registers.
    macro_rules! define_allocatable_simd128_registers {
    ($macro:ident) => {
        $macro!(v1);
        $macro!(v2);
        $macro!(v3);
        $macro!(v4);
        $macro!(v5);
        $macro!(v6);
        $macro!(v7);
        $macro!(v10);
        $macro!(v11);
        $macro!(v12);
        $macro!(v13);
        $macro!(v14);
        $macro!(v15);
        $macro!(v16);
        $macro!(v17);
        $macro!(v18);
        $macro!(v19);
        $macro!(v20);
        $macro!(v21);
        $macro!(v22);
        $macro!(v26);
        $macro!(v27);
        $macro!(v28);
        $macro!(v29);
        $macro!(v30);
        $macro!(v31);
    };
}

    /// Defines the allocatable double-precision floating-point registers.
    macro_rules! define_allocatable_double_registers {
    ($macro:ident) => {
        $macro!(ft1);
        $macro!(ft2);
        $macro!(ft3);
        $macro!(ft4);
        $macro!(ft5);
        $macro!(ft6);
        $macro!(ft7);
        $macro!(ft8);
        $macro!(ft9);
        $macro!(ft10);
        $macro!(ft11);
        $macro!(fa0);
        $macro!(fa1);
        $macro!(fa2);
        $macro!(fa3);
        $macro!(fa4);
        $macro!(fa5);
        $macro!(fa6);
        $macro!(fa7);
    };
}

    /// Returns the number of padding slots needed for stack pointer alignment.
    pub const fn argument_padding_slots(_argument_count: i32) -> i32 {
        // No argument padding required.
        0
    }

    /// Defines register codes.
    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum RegisterCode {
        #[allow(dead_code)]
        kRegCode_zero_reg,
        #[allow(dead_code)]
        kRegCode_ra,
        #[allow(dead_code)]
        kRegCode_sp,
        #[allow(dead_code)]
        kRegCode_gp,
        #[allow(dead_code)]
        kRegCode_tp,
        #[allow(dead_code)]
        kRegCode_t0,
        #[allow(dead_code)]
        kRegCode_t1,
        #[allow(dead_code)]
        kRegCode_t2,
        #[allow(dead_code)]
        kRegCode_fp,
        #[allow(dead_code)]
        kRegCode_s1,
        #[allow(dead_code)]
        kRegCode_a0,
        #[allow(dead_code)]
        kRegCode_a1,
        #[allow(dead_code)]
        kRegCode_a2,
        #[allow(dead_code)]
        kRegCode_a3,
        #[allow(dead_code)]
        kRegCode_a4,
        #[allow(dead_code)]
        kRegCode_a5,
        #[allow(dead_code)]
        kRegCode_a6,
        #[allow(dead_code)]
        kRegCode_a7,
        #[allow(dead_code)]
        kRegCode_s2,
        #[allow(dead_code)]
        kRegCode_s3,
        #[allow(dead_code)]
        kRegCode_s4,
        #[allow(dead_code)]
        kRegCode_s5,
        #[allow(dead_code)]
        kRegCode_s6,
        #[allow(dead_code)]
        kRegCode_s7,
        #[allow(dead_code)]
        kRegCode_s8,
        #[allow(dead_code)]
        kRegCode_s9,
        #[allow(dead_code)]
        kRegCode_s10,
        #[allow(dead_code)]
        kRegCode_s11,
        #[allow(dead_code)]
        kRegCode_t3,
        #[allow(dead_code)]
        kRegCode_t4,
        #[allow(dead_code)]
        kRegCode_t5,
        #[allow(dead_code)]
        kRegCode_t6,
        kRegAfterLast,
    }

    /// Defines double register codes.
    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum DoubleRegisterCode {
        #[allow(dead_code)]
        kDoubleCode_ft0,
        #[allow(dead_code)]
        kDoubleCode_ft1,
        #[allow(dead_code)]
        kDoubleCode_ft2,
        #[allow(dead_code)]
        kDoubleCode_ft3,
        #[allow(dead_code)]
        kDoubleCode_ft4,
        #[allow(dead_code)]
        kDoubleCode_ft5,
        #[allow(dead_code)]
        kDoubleCode_ft6,
        #[allow(dead_code)]
        kDoubleCode_ft7,
        #[allow(dead_code)]
        kDoubleCode_fs0,
        #[allow(dead_code)]
        kDoubleCode_fs1,
        #[allow(dead_code)]
        kDoubleCode_fa0,
        #[allow(dead_code)]
        kDoubleCode_fa1,
        #[allow(dead_code)]
        kDoubleCode_fa2,
        #[allow(dead_code)]
        kDoubleCode_fa3,
        #[allow(dead_code)]
        kDoubleCode_fa4,
        #[allow(dead_code)]
        kDoubleCode_fa5,
        #[allow(dead_code)]
        kDoubleCode_fa6,
        #[allow(dead_code)]
        kDoubleCode_fa7,
        #[allow(dead_code)]
        kDoubleCode_fs2,
        #[allow(dead_code)]
        kDoubleCode_fs3,
        #[allow(dead_code)]
        kDoubleCode_fs4,
        #[allow(dead_code)]
        kDoubleCode_fs5,
        #[allow(dead_code)]
        kDoubleCode_fs6,
        #[allow(dead_code)]
        kDoubleCode_fs7,
        #[allow(dead_code)]
        kDoubleCode_fs8,
        #[allow(dead_code)]
        kDoubleCode_fs9,
        #[allow(dead_code)]
        kDoubleCode_fs10,
        #[allow(dead_code)]
        kDoubleCode_fs11,
        #[allow(dead_code)]
        kDoubleCode_ft8,
        #[allow(dead_code)]
        kDoubleCode_ft9,
        #[allow(dead_code)]
        kDoubleCode_ft10,
        #[allow(dead_code)]
        kDoubleCode_ft11,
        kDoubleAfterLast,
    }

    /// Defines vector register codes.
    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum VRegisterCode {
        #[allow(dead_code)]
        kVRCode_v0,
        #[allow(dead_code)]
        kVRCode_v1,
        #[allow(dead_code)]
        kVRCode_v2,
        #[allow(dead_code)]
        kVRCode_v3,
        #[allow(dead_code)]
        kVRCode_v4,
        #[allow(dead_code)]
        kVRCode_v5,
        #[allow(dead_code)]
        kVRCode_v6,
        #[allow(dead_code)]
        kVRCode_v7,
        #[allow(dead_code)]
        kVRCode_v8,
        #[allow(dead_code)]
        kVRCode_v9,
        #[allow(dead_code)]
        kVRCode_v10,
        #[allow(dead_code)]
        kVRCode_v11,
        #[allow(dead_code)]
        kVRCode_v12,
        #[allow(dead_code)]
        kVRCode_v13,
        #[allow(dead_code)]
        kVRCode_v14,
        #[allow(dead_code)]
        kVRCode_v15,
        #[allow(dead_code)]
        kVRCode_v16,
        #[allow(dead_code)]
        kVRCode_v17,
        #[allow(dead_code)]
        kVRCode_v18,
        #[allow(dead_code)]
        kVRCode_v19,
        #[allow(dead_code)]
        kVRCode_v20,
        #[allow(dead_code)]
        kVRCode_v21,
        #[allow(dead_code)]
        kVRCode_v22,
        #[allow(dead_code)]
        kVRCode_v23,
        #[allow(dead_code)]
        kVRCode_v24,
        #[allow(dead_code)]
        kVRCode_v25,
        #[allow(dead_code)]
        kVRCode_v26,
        #[allow(dead_code)]
        kVRCode_v27,
        #[allow(dead_code)]
        kVRCode_v28,
        #[allow(dead_code)]
        kVRCode_v29,
        #[allow(dead_code)]
        kVRCode_v30,
        #[allow(dead_code)]
        kVRCode_v31,
        kVRAfterLast,
    }

    pub const K_NUM_REGS: usize = 32;
    pub const K_UNDEF_INDEX: i32 = -1;

    pub const K_SAFEPOINT_REGISTER_STACK_INDEX_MAP: [i32; K_NUM_REGS] = [
        K_UNDEF_INDEX, // zero_reg
        K_UNDEF_INDEX, // ra
        K_UNDEF_INDEX, // sp
        K_UNDEF_INDEX, // gp
        K_UNDEF_INDEX, // tp
        0,             // t0
        1,             // t1
        2,             // t2
        3,             // s0/fp
        4,             // s1
        5,             // a0
        6,             // a1
        7,             // a2
        8,             // a3
        9,             // a4
        10,            // a5
        11,            // a6
        12,            // a7
        13,            // s2
        14,            // s3
        15,            // s4
        16,            // s5
        17,            // s6
        18,            // s7
        19,            // s8
        10,            // s9
        21,            // s10
        22,            // s11
        K_UNDEF_INDEX, // t3
        23,            // t4
        K_UNDEF_INDEX, // t5
        K_UNDEF_INDEX, // t6
    ];

    impl TryFrom<i32> for Register {
        type Error = &'static str;

        fn try_from(num: i32) -> Result<Self, Self::Error> {
            if num >= 0 && num < K_NUM_REGS as i32 {
                Ok(Register::from_code(num as usize))
            } else {
                Err("Invalid register number")
            }
        }
    }

    pub fn to_number(reg: Register) -> i32 {
        reg.code() as i32
    }

    pub fn to_register(num: i32) -> Result<Register, &'static str> {
        Register::try_from(num)
    }

    pub const K_PAD_ARGUMENTS: bool = false;
    #[derive(PartialEq, Eq, Clone, Copy, Debug)]
    pub enum AliasingKind {
        kIndependent,
        // Add other aliasing kinds if necessary.
    }
    pub const K_FP_ALIASING: AliasingKind = AliasingKind::kIndependent;
    pub const K_SIMD_MASK_REGISTERS: bool = false;

    macro_rules! declare_register {
        ($reg:ident) => {
            #[allow(dead_code)]
            pub const $reg: Register = Register::from_code(RegisterCode::kRegCode_$reg as usize);
        };
    }

    define_general_registers!(declare_register);

    macro_rules! declare_double_register {
        ($reg:ident) => {
            #[allow(dead_code)]
            pub const $reg: DoubleRegister = DoubleRegister::from_code(DoubleRegisterCode::kDoubleCode_$reg as usize);
        };
    }

    define_double_registers!(declare_double_register);

    macro_rules! declare_vector_register {
        ($reg:ident) => {
            #[allow(dead_code)]
            pub const $reg: VRegister = VRegister::from_code(VRegisterCode::kVRCode_$reg as usize);
        };
    }

    define_vector_registers!(declare_vector_register);

    #[allow(dead_code)]
    pub const no_reg: Register = Register::no_reg();
    #[allow(dead_code)]
    pub const no_dreg: DoubleRegister = DoubleRegister::no_reg();
    #[allow(dead_code)]
    pub const no_msareg: VRegister = VRegister::no_reg();

    /// Assign |source| value to |no_reg| and return the |source|'s previous value.
    pub fn reassign_register(source: &mut Register) -> Register {
        let result = *source;
        *source = Register::no_reg();
        result
    }

    // Register aliases.
    // cp is assumed to be a callee saved register.
    #[allow(dead_code)]
    pub const K_ROOT_REGISTER: Register = s6;
    #[allow(dead_code)]
    pub const CP: Register = s7;
    #[allow(dead_code)]
    pub const K_SCRATCH_REG: Register = s3;
    #[allow(dead_code)]
    pub const K_SCRATCH_REG2: Register = s4;
    #[allow(dead_code)]
    pub const K_STACK_POINTER_REGISTER: Register = sp;
    #[allow(dead_code)]
    pub const PADREG: Register = t6;

    #[allow(dead_code)]
    pub const K_SCRATCH_DOUBLE_REG: DoubleRegister = ft0;

    #[allow(dead_code)]
    pub const K_DOUBLE_REG_ZERO: DoubleRegister = fs9;
    #[allow(dead_code)]
    pub const K_SINGLE_REG_ZERO: DoubleRegister = fs10;

    // Define {RegisterName} methods for the register types.
    macro_rules! define_register_names {
        ($type:ident, $macro:ident) => {
            impl $type {
                #[allow(dead_code)]
                pub fn register_name(&self) -> &'static str {
                    match self.code() {
                        #(
                            #[allow(dead_code)]
                            _ if self == & $type::from_code(RegisterCode::kRegCode_$macro as usize) => stringify!($macro),
                        )*
                        _ => "unknown",
                    }
                }
            }
        };
    }

    // Helper macro for defining registers.
    macro_rules! register_name {
        ($name:ident) => {
            $name
        }
    }

    // Define {RegisterName} methods for the register types.
    //define_register_names!(Register, define_general_registers);
    //define_register_names!(FPURegister, define_double_registers);
    //define_register_names!(VRegister, define_vector_registers);

    impl Register {
        #[allow(dead_code)]
        pub fn register_name(&self) -> &'static str {
            match self.code() {
                _ if self == &zero_reg => "zero_reg",
                _ if self == &ra => "ra",
                _ if self == &sp => "sp",
                _ if self == &gp => "gp",
                _ if self == &tp => "tp",
                _ if self == &t0 => "t0",
                _ if self == &t1 => "t1",
                _ if self == &t2 => "t2",
                _ if self == &fp => "fp",
                _ if self == &s1 => "s1",
                _ if self == &a0 => "a0",
                _ if self == &a1 => "a1",
                _ if self == &a2 => "a2",
                _ if self == &a3 => "a3",
                _ if self == &a4 => "a4",
                _ if self == &a5 => "a5",
                _ if self == &a6 => "a6",
                _ if self == &a7 => "a7",
                _ if self == &s2 => "s2",
                _ if self == &s3 => "s3",
                _ if self == &s4 => "s4",
                _ if self == &s5 => "s5",
                _ if self == &s6 => "s6",
                _ if self == &s7 => "s7",
                _ if self == &s8 => "s8",
                _ if self == &s9 => "s9",
                _ if self == &s10 => "s10",
                _ if self == &s11 => "s11",
                _ if self == &t3 => "t3",
                _ if self == &t4 => "t4",
                _ if self == &t5 => "t5",
                _ if self == &t6 => "t6",
                _ => "unknown",
            }
        }
    }

    impl FPURegister {
        #[allow(dead_code)]
        pub fn register_name(&self) -> &'static str {
            match self.code() {
                _ if self == &ft0 => "ft0",
                _ if self == &ft1 => "ft1",
                _ if self == &ft2 => "ft2",
                _ if self == &ft3 => "ft3",
                _ if self == &ft4 => "ft4",
                _ if self == &ft5 => "ft5",
                _ if self == &ft6 => "ft6",
                _ if self == &ft7 => "ft7",
                _ if self == &fs0 => "fs0",
                _ if self == &fs1 => "fs1",
                _ if self == &fa0 => "fa0",
                _ if self == &fa1 => "fa1",
                _ if self == &fa2 => "fa2",
                _ if self == &fa3 => "fa3",
                _ if self == &fa4 => "fa4",
                _ if self == &fa5 => "fa5",
                _ if self == &fa6 => "fa6",
                _ if self == &fa7 => "fa7",
                _ if self == &fs2 => "fs2",
                _ if self == &fs3 => "fs3",
                _ if self == &fs4 => "fs4",
                _ if self == &fs5 => "fs5",
                _ if self == &fs6 => "fs6",
                _ if self == &fs7 => "fs7",
                _ if self == &fs8 => "fs8",
                _ if self == &fs9 => "fs9",
                _ if self == &fs10 => "fs10",
                _ if self == &fs11 => "fs11",
                _ if self == &ft8 => "ft8",
                _ if self == &ft9 => "ft9",
                _ if self == &ft10 => "ft10",
                _ if self == &ft11 => "ft11",
                _ => "unknown",
            }
        }
    }

    impl VRegister {
        #[allow(dead_code)]
        pub fn register_name(&self) -> &'static str {
            match self.code() {
                _ if self == &v0 => "v0",
                _ if self == &v1 => "v1",
                _ if self == &v2 => "v2",
                _ if self == &v3 => "v3",
                _ if self == &v4 => "v4",
                _ if self == &v5 => "v5",
                _ if self == &v6 => "v6",
                _ if self == &v7 => "v7",
                _ if self == &v8 => "v8",
                _ if self == &v9 => "v9",
                _ if self == &v10 => "v10",
                _ if self == &v11 => "v11",
                _ if self == &v12 => "v12",
                _ if self == &v13 => "v13",
                _ if self == &v14 => "v14",
                _ if self == &v15 => "v15",
                _ if self == &v16 => "v16",
                _ if self == &v17 => "v17",
                _ if self == &v18 => "v18",
                _ if self == &v19 => "v19",
                _ if self == &v20 => "v20",
                _ if self == &v21 => "v21",
                _ if self == &v22 => "v22",
                _ if self == &v23 => "v23",
                _ if self == &v24 => "v24",
                _ if self == &v25 => "v25",
                _ if self == &v26 => "v26",
                _ if self == &v27 => "v27",
                _ if self == &v28 => "v28",
                _ if self == &v29 => "v29",
                _ if self == &v30 => "v30",
                _ if self == &v31 => "v31",
                _ => "unknown",
            }
        }
    }

    // Give alias names to registers for calling conventions.
    #[allow(dead_code)]
    pub const K_C_ARG_REGS: [Register; 8] = [a0, a1, a2, a3, a4, a5, a6, a7];
    #[allow(dead_code)]
    pub const K_REGISTER_PASSED_ARGUMENTS: usize = K_C_ARG_REGS.len();
    #[allow(dead_code)]
    pub const K_FP_REGISTER_PASSED_ARGUMENTS: i32 = 8;

    #[allow(dead_code)]
    pub const K_RETURN_REGISTER0: Register = a0;
    #[allow(dead_code)]
    pub const K_RETURN_REGISTER1: Register = a1;
    #[allow(dead_code)]
    pub const K_RETURN_REGISTER2: Register = a2;
    #[allow