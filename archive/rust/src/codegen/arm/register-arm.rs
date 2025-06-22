// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod register_arm {
    use std::mem;

    //use crate::codegen::register_base::*; // Assuming register-base.h is in codegen module

    macro_rules! general_registers {
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
            $V!(sp);
            $V!(lr);
            $V!(pc);
        };
    }

    macro_rules! allocatable_general_registers {
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
        };
    }

    macro_rules! float_registers {
        ($V:ident) => {
            $V!(s0);
            $V!(s1);
            $V!(s2);
            $V!(s3);
            $V!(s4);
            $V!(s5);
            $V!(s6);
            $V!(s7);
            $V!(s8);
            $V!(s9);
            $V!(s10);
            $V!(s11);
            $V!(s12);
            $V!(s13);
            $V!(s14);
            $V!(s15);
            $V!(s16);
            $V!(s17);
            $V!(s18);
            $V!(s19);
            $V!(s20);
            $V!(s21);
            $V!(s22);
            $V!(s23);
            $V!(s24);
            $V!(s25);
            $V!(s26);
            $V!(s27);
            $V!(s28);
            $V!(s29);
            $V!(s30);
            $V!(s31);
        };
    }

    macro_rules! low_double_registers {
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

    macro_rules! non_low_double_registers {
        ($V:ident) => {
            $V!(d16);
            $V!(d17);
            $V!(d18);
            $V!(d19);
            $V!(d20);
            $V!(d21);
            $V!(d22);
            $V!(d23);
            $V!(d24);
            $V!(d25);
            $V!(d26);
            $V!(d27);
            $V!(d28);
            $V!(d29);
            $V!(d30);
            $V!(d31);
        };
    }

    macro_rules! double_registers {
        ($V:ident) => {
            low_double_registers!($V);
            non_low_double_registers!($V);
        };
    }

    macro_rules! simd128_registers {
        ($V:ident) => {
            $V!(q0);
            $V!(q1);
            $V!(q2);
            $V!(q3);
            $V!(q4);
            $V!(q5);
            $V!(q6);
            $V!(q7);
            $V!(q8);
            $V!(q9);
            $V!(q10);
            $V!(q11);
            $V!(q12);
            $V!(q13);
            $V!(q14);
            $V!(q15);
        };
    }

    macro_rules! allocatable_double_registers {
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
            $V!(d16);
            $V!(d17);
            $V!(d18);
            $V!(d19);
            $V!(d20);
            $V!(d21);
            $V!(d22);
            $V!(d23);
            $V!(d24);
            $V!(d25);
            $V!(d26);
            $V!(d27);
            $V!(d28);
            $V!(d29);
            $V!(d30);
            $V!(d31);
        };
    }

    macro_rules! allocatable_no_vfp32_double_registers {
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
            $V!(d15);
        };
    }

    macro_rules! c_registers {
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

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(u32)]
    pub enum RegisterCode {
        #[allow(non_camel_case_types)]
        r0 = 0,
        #[allow(non_camel_case_types)]
        r1,
        #[allow(non_camel_case_types)]
        r2,
        #[allow(non_camel_case_types)]
        r3,
        #[allow(non_camel_case_types)]
        r4,
        #[allow(non_camel_case_types)]
        r5,
        #[allow(non_camel_case_types)]
        r6,
        #[allow(non_camel_case_types)]
        r7,
        #[allow(non_camel_case_types)]
        r8,
        #[allow(non_camel_case_types)]
        r9,
        #[allow(non_camel_case_types)]
        r10,
        #[allow(non_camel_case_types)]
        fp,
        #[allow(non_camel_case_types)]
        ip,
        #[allow(non_camel_case_types)]
        sp,
        #[allow(non_camel_case_types)]
        lr,
        #[allow(non_camel_case_types)]
        pc,
        kRegAfterLast,
    }

    //impl From<u32> for RegisterCode {
    //    fn from(code: u32) -> Self {
    //        match code {
    //            0 => RegisterCode::r0,
    //            1 => RegisterCode::r1,
    //            2 => RegisterCode::r2,
    //            3 => RegisterCode::r3,
    //            4 => RegisterCode::r4,
    //            5 => RegisterCode::r5,
    //            6 => RegisterCode::r6,
    //            7 => RegisterCode::r7,
    //            8 => RegisterCode::r8,
    //            9 => RegisterCode::r9,
    //            10 => RegisterCode::r10,
    //            11 => RegisterCode::fp,
    //            12 => RegisterCode::ip,
    //            13 => RegisterCode::sp,
    //            14 => RegisterCode::lr,
    //            15 => RegisterCode::pc,
    //            _ => RegisterCode::kRegAfterLast
    //        }
    //    }
    //}
    impl RegisterCode {
        pub fn from_code(code: u32) -> Self {
            match code {
                0 => RegisterCode::r0,
                1 => RegisterCode::r1,
                2 => RegisterCode::r2,
                3 => RegisterCode::r3,
                4 => RegisterCode::r4,
                5 => RegisterCode::r5,
                6 => RegisterCode::r6,
                7 => RegisterCode::r7,
                8 => RegisterCode::r8,
                9 => RegisterCode::r9,
                10 => RegisterCode::r10,
                11 => RegisterCode::fp,
                12 => RegisterCode::ip,
                13 => RegisterCode::sp,
                14 => RegisterCode::lr,
                15 => RegisterCode::pc,
                _ => RegisterCode::kRegAfterLast,
            }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Register {
        code: RegisterCode,
    }

    impl Register {
        pub const fn new(code: RegisterCode) -> Self {
            Register { code }
        }

        pub const fn from_code(code: RegisterCode) -> Self {
            Register { code }
        }

        pub const fn code(&self) -> RegisterCode {
            self.code
        }

        pub fn is_valid(&self) -> bool {
            self.code != RegisterCode::kRegAfterLast
        }

        pub const fn no_reg() -> Self {
            Register {
                code: RegisterCode::kRegAfterLast,
            }
        }

        pub fn names() -> [&'static str; 16] {
            ["r0", "r1", "r2", "r3", "r4", "r5", "r6", "r7", "r8", "r9", "r10", "fp", "ip", "sp", "lr", "pc"]
        }
    }

    #[allow(dead_code)]
    pub fn reassign_register(source: &mut Register) -> Register {
        let result = *source;
        *source = Register::no_reg();
        result
    }

    macro_rules! declare_register {
        ($R:ident) => {
            #[allow(non_upper_case_globals)]
            pub const $R: Register = Register::from_code(RegisterCode::$R);
        };
    }

    general_registers!(declare_register);

    #[allow(non_upper_case_globals)]
    pub const no_reg: Register = Register::no_reg();

    #[allow(non_upper_case_globals)]
    pub const k_c_arg_regs: [Register; 4] = [r0, r1, r2, r3];
    #[allow(dead_code)]
    pub const K_REGISTER_PASSED_ARGUMENTS: usize = k_c_arg_regs.len();
    #[allow(dead_code)]
    pub const K_DOUBLE_REGISTER_PASSED_ARGUMENTS: usize = 8;

    #[allow(dead_code)]
    pub const fn argument_padding_slots(_argument_count: i32) -> i32 {
        0 // No argument padding required.
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum AliasingKind {
        kCombine,
    }

    #[allow(dead_code)]
    pub const K_FP_ALIASING: AliasingKind = AliasingKind::kCombine;
    #[allow(dead_code)]
    pub const K_SIMD_MASK_REGISTERS: bool = false;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(u32)]
    pub enum SwVfpRegisterCode {
        #[allow(non_camel_case_types)]
        s0 = 0,
        #[allow(non_camel_case_types)]
        s1,
        #[allow(non_camel_case_types)]
        s2,
        #[allow(non_camel_case_types)]
        s3,
        #[allow(non_camel_case_types)]
        s4,
        #[allow(non_camel_case_types)]
        s5,
        #[allow(non_camel_case_types)]
        s6,
        #[allow(non_camel_case_types)]
        s7,
        #[allow(non_camel_case_types)]
        s8,
        #[allow(non_camel_case_types)]
        s9,
        #[allow(non_camel_case_types)]
        s10,
        #[allow(non_camel_case_types)]
        s11,
        #[allow(non_camel_case_types)]
        s12,
        #[allow(non_camel_case_types)]
        s13,
        #[allow(non_camel_case_types)]
        s14,
        #[allow(non_camel_case_types)]
        s15,
        #[allow(non_camel_case_types)]
        s16,
        #[allow(non_camel_case_types)]
        s17,
        #[allow(non_camel_case_types)]
        s18,
        #[allow(non_camel_case_types)]
        s19,
        #[allow(non_camel_case_types)]
        s20,
        #[allow(non_camel_case_types)]
        s21,
        #[allow(non_camel_case_types)]
        s22,
        #[allow(non_camel_case_types)]
        s23,
        #[allow(non_camel_case_types)]
        s24,
        #[allow(non_camel_case_types)]
        s25,
        #[allow(non_camel_case_types)]
        s26,
        #[allow(non_camel_case_types)]
        s27,
        #[allow(non_camel_case_types)]
        s28,
        #[allow(non_camel_case_types)]
        s29,
        #[allow(non_camel_case_types)]
        s30,
        #[allow(non_camel_case_types)]
        s31,
        kSwVfpAfterLast,
    }

    impl SwVfpRegisterCode {
        pub fn from_code(code: u32) -> Self {
            match code {
                0 => SwVfpRegisterCode::s0,
                1 => SwVfpRegisterCode::s1,
                2 => SwVfpRegisterCode::s2,
                3 => SwVfpRegisterCode::s3,
                4 => SwVfpRegisterCode::s4,
                5 => SwVfpRegisterCode::s5,
                6 => SwVfpRegisterCode::s6,
                7 => SwVfpRegisterCode::s7,
                8 => SwVfpRegisterCode::s8,
                9 => SwVfpRegisterCode::s9,
                10 => SwVfpRegisterCode::s10,
                11 => SwVfpRegisterCode::s11,
                12 => SwVfpRegisterCode::s12,
                13 => SwVfpRegisterCode::s13,
                14 => SwVfpRegisterCode::s14,
                15 => SwVfpRegisterCode::s15,
                16 => SwVfpRegisterCode::s16,
                17 => SwVfpRegisterCode::s17,
                18 => SwVfpRegisterCode::s18,
                19 => SwVfpRegisterCode::s19,
                20 => SwVfpRegisterCode::s20,
                21 => SwVfpRegisterCode::s21,
                22 => SwVfpRegisterCode::s22,
                23 => SwVfpRegisterCode::s23,
                24 => SwVfpRegisterCode::s24,
                25 => SwVfpRegisterCode::s25,
                26 => SwVfpRegisterCode::s26,
                27 => SwVfpRegisterCode::s27,
                28 => SwVfpRegisterCode::s28,
                29 => SwVfpRegisterCode::s29,
                30 => SwVfpRegisterCode::s30,
                31 => SwVfpRegisterCode::s31,
                _ => SwVfpRegisterCode::kSwVfpAfterLast,
            }
        }
    }

    pub type VfpRegList = u64;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct SwVfpRegister {
        code: SwVfpRegisterCode,
    }

    impl SwVfpRegister {
        pub const K_SIZE_IN_BYTES: i32 = 4;

        pub fn from_code(code: SwVfpRegisterCode) -> Self {
            SwVfpRegister { code }
        }

        pub fn code(&self) -> SwVfpRegisterCode {
            self.code
        }

        pub fn is_valid(&self) -> bool {
            self.code != SwVfpRegisterCode::kSwVfpAfterLast
        }

        pub fn split_code(reg_code: i32, vm: &mut i32, m: &mut i32) {
            let reg = SwVfpRegister::from_code(SwVfpRegisterCode::from_code(reg_code as u32));
            assert!(reg.is_valid());
            *m = reg_code & 0x1;
            *vm = reg_code >> 1;
        }
        pub fn to_vfp_reg_list(&self) -> VfpRegList {
            1u64 << (self.code as u32)
        }

    }

    pub type FloatRegister = SwVfpRegister;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(u32)]
    pub enum DoubleRegisterCode {
        #[allow(non_camel_case_types)]
        d0 = 0,
        #[allow(non_camel_case_types)]
        d1,
        #[allow(non_camel_case_types)]
        d2,
        #[allow(non_camel_case_types)]
        d3,
        #[allow(non_camel_case_types)]
        d4,
        #[allow(non_camel_case_types)]
        d5,
        #[allow(non_camel_case_types)]
        d6,
        #[allow(non_camel_case_types)]
        d7,
        #[allow(non_camel_case_types)]
        d8,
        #[allow(non_camel_case_types)]
        d9,
        #[allow(non_camel_case_types)]
        d10,
        #[allow(non_camel_case_types)]
        d11,
        #[allow(non_camel_case_types)]
        d12,
        #[allow(non_camel_case_types)]
        d13,
        #[allow(non_camel_case_types)]
        d14,
        #[allow(non_camel_case_types)]
        d15,
        #[allow(non_camel_case_types)]
        d16,
        #[allow(non_camel_case_types)]
        d17,
        #[allow(non_camel_case_types)]
        d18,
        #[allow(non_camel_case_types)]
        d19,
        #[allow(non_camel_case_types)]
        d20,
        #[allow(non_camel_case_types)]
        d21,
        #[allow(non_camel_case_types)]
        d22,
        #[allow(non_camel_case_types)]
        d23,
        #[allow(non_camel_case_types)]
        d24,
        #[allow(non_camel_case_types)]
        d25,
        #[allow(non_camel_case_types)]
        d26,
        #[allow(non_camel_case_types)]
        d27,
        #[allow(non_camel_case_types)]
        d28,
        #[allow(non_camel_case_types)]
        d29,
        #[allow(non_camel_case_types)]
        d30,
        #[allow(non_camel_case_types)]
        d31,
        kDoubleAfterLast,
    }

    impl DoubleRegisterCode {
        pub fn from_code(code: u32) -> Self {
            match code {
                0 => DoubleRegisterCode::d0,
                1 => DoubleRegisterCode::d1,
                2 => DoubleRegisterCode::d2,
                3 => DoubleRegisterCode::d3,
                4 => DoubleRegisterCode::d4,
                5 => DoubleRegisterCode::d5,
                6 => DoubleRegisterCode::d6,
                7 => DoubleRegisterCode::d7,
                8 => DoubleRegisterCode::d8,
                9 => DoubleRegisterCode::d9,
                10 => DoubleRegisterCode::d10,
                11 => DoubleRegisterCode::d11,
                12 => DoubleRegisterCode::d12,
                13 => DoubleRegisterCode::d13,
                14 => DoubleRegisterCode::d14,
                15 => DoubleRegisterCode::d15,
                16 => DoubleRegisterCode::d16,
                17 => DoubleRegisterCode::d17,
                18 => DoubleRegisterCode::d18,
                19 => DoubleRegisterCode::d19,
                20 => DoubleRegisterCode::d20,
                21 => DoubleRegisterCode::d21,
                22 => DoubleRegisterCode::d22,
                23 => DoubleRegisterCode::d23,
                24 => DoubleRegisterCode::d24,
                25 => DoubleRegisterCode::d25,
                26 => DoubleRegisterCode::d26,
                27 => DoubleRegisterCode::d27,
                28 => DoubleRegisterCode::d28,
                29 => DoubleRegisterCode::d29,
                30 => DoubleRegisterCode::d30,
                31 => DoubleRegisterCode::d31,
                _ => DoubleRegisterCode::kDoubleAfterLast,
            }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct DwVfpRegister {
        code: DoubleRegisterCode,
    }

    impl DwVfpRegister {
        pub const K_SIZE_IN_BYTES: i32 = 8;

        pub fn from_code(code: DoubleRegisterCode) -> Self {
            DwVfpRegister { code }
        }

        pub fn code(&self) -> DoubleRegisterCode {
            self.code
        }

        pub fn is_valid(&self) -> bool {
            self.code != DoubleRegisterCode::kDoubleAfterLast
        }

        #[allow(dead_code)]
        pub fn supported_register_count() -> i32 {
            // This function differs from kNumRegisters by returning the number of double
            // registers supported by the current CPU, while kNumRegisters always returns
            // 32.
            todo!()
        }

        pub fn split_code(reg_code: i32, vm: &mut i32, m: &mut i32) {
            let reg = DwVfpRegister::from_code(DoubleRegisterCode::from_code(reg_code as u32));
            assert!(reg.is_valid());
            *m = (reg_code & 0x10) >> 4;
            *vm = reg_code & 0x0F;
        }

        pub fn to_vfp_reg_list(&self) -> VfpRegList {
            3u64 << ((self.code as u32) * 2)
        }
    }

    pub type DoubleRegister = DwVfpRegister;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct LowDwVfpRegister {
        code: DoubleRegisterCode,
    }

    impl LowDwVfpRegister {
        pub const fn from_code(code: DoubleRegisterCode) -> Self {
            LowDwVfpRegister { code }
        }

        pub fn code(&self) -> DoubleRegisterCode {
            self.code
        }

        pub fn is_valid(&self) -> bool {
            (self.code as u32) < DoubleRegisterCode::d16 as u32
        }

        pub const fn into_dw_vfp_register(self) -> DwVfpRegister {
            DwVfpRegister { code: self.code }
        }

        pub fn low(&self) -> SwVfpRegister {
            SwVfpRegister::from_code(SwVfpRegisterCode::from_code((self.code as u32 * 2) as u32))
        }

        pub fn high(&self) -> SwVfpRegister {
            SwVfpRegister::from_code(SwVfpRegisterCode::from_code((self.code as u32 * 2 + 1) as u32))
        }

        pub fn to_vfp_reg_list(&self) -> VfpRegList {
            3u64 << ((self.code as u32) * 2)
        }
    }

    impl From<LowDwVfpRegister> for DwVfpRegister {
        fn from(low_reg: LowDwVfpRegister) -> Self {
            DwVfpRegister { code: low_reg.code }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(u32)]
    pub enum Simd128RegisterCode {
        #[allow(non_camel_case_types)]
        q0 = 0,
        #[allow(non_camel_case_types)]
        q1,
        #[allow(non_camel_case_types)]
        q2,
        #[allow(non_camel_case_types)]
        q3,
        #[allow(non_camel_case_types)]
        q4,
        #[allow(non_camel_case_types)]
        q5,
        #[allow(non_camel_case_types)]
        q6,
        #[allow(non_camel_case_types)]
        q7,
        #[allow(non_camel_case_types)]
        q8,
        #[allow(non_camel_case_types)]
        q9,
        #[allow(non_camel_case_types)]
        q10,
        #[allow(non_camel_case_types)]
        q11,
        #[allow(non_camel_case_types)]
        q12,
        #[allow(non_camel_case_types)]
        q13,
        #[allow(non_camel_case_types)]
        q14,
        #[allow(non_camel_case_types)]
        q15,
        kSimd128AfterLast,
    }

    impl Simd128RegisterCode {
        pub fn from_code(code: u32) -> Self {
            match code {
                0 => Simd128RegisterCode::q0,
                1 => Simd128RegisterCode::q1,
                2 => Simd128RegisterCode::q2,
                3 => Simd128RegisterCode::q3,
                4 => Simd128RegisterCode::q4,
                5 => Simd128RegisterCode::q5,
                6 => Simd128RegisterCode::q6,
                7 => Simd128RegisterCode::q7,
                8 => Simd128RegisterCode::q8,
                9 => Simd128RegisterCode::q9,
                10 => Simd128RegisterCode::q10,
                11 => Simd128RegisterCode::q11,
                12 => Simd128RegisterCode::q12,
                13 => Simd128RegisterCode::q13,
                14 => Simd128RegisterCode::q14,
                15 => Simd128RegisterCode::q15,
                _ => Simd128RegisterCode::kSimd128AfterLast,
            }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct QwNeonRegister {
        code: Simd128RegisterCode,
    }

    impl QwNeonRegister {
        pub fn from_code(code: Simd128RegisterCode) -> Self {
            QwNeonRegister { code }
        }

        pub fn code(&self) -> Simd128RegisterCode {
            self.code
        }

        pub fn is_valid(&self) -> bool {
            self.code != Simd128RegisterCode::kSimd128AfterLast
        }

        pub fn split_code(reg_code: i32, vm: &mut i32, m: &mut i32) {
            assert!(reg_code >= 0 && reg_code < Self::k_num_registers() as i32);
            let encoded_code = reg_code << 1;
            *m = (encoded_code & 0x10) >> 4;
            *vm = encoded_code & 0x0F;
        }

        pub fn low(&self) -> DwVfpRegister {
            DwVfpRegister::from_code(DoubleRegisterCode::from_code((self.code as u32 * 2) as u32))
        }

        pub fn high(&self) -> DwVfpRegister {
            DwVfpRegister::from_code(DoubleRegisterCode::from_code((self.code as u32 * 2 + 1) as u32))
        }

        pub fn to_vfp_reg_list(&self) -> VfpRegList {
            0xfu64 << ((self.code as u32) * 4)
        }

        pub const fn k_num_registers() -> usize { 16 }
        pub fn names() -> [&'static str; 16] {
            ["q0", "q1", "q2", "q3", "q4", "q5", "q6", "q7", "q8", "q9", "q10", "q11", "q12", "q13", "q14", "q15"]
        }
    }

    pub type QuadRegister = QwNeonRegister;
    pub type Simd128Register = QwNeonRegister;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(u32)]
    pub enum CRegisterCode {
        #[allow(non_camel_case_types)]
        cr0 = 0,
        #[allow(non_camel_case_types)]
        cr1,
        #[allow(non_camel_case_types)]
        cr2,
        #[allow(non_camel_case_types)]
        cr3,
        #[allow(non_camel_case_types)]
        cr4,
        #[allow(non_camel_case_types)]
        cr5,
        #[allow(non_camel_case_types)]
        cr6,
        #[allow(non_camel_case_types)]
        cr7,
        #[allow(non_camel_case_types)]
        cr8,
        #[allow(non_camel_case_types)]
        cr9,
        #[allow(non_camel_case_types)]
        cr10,
        #[allow(non_camel_case_types)]
        cr11,
        #[allow(non_camel_case_types)]
        cr12,
        #[allow(non_camel_case_types)]
        cr15,
        kCAfterLast,
    }

    impl CRegisterCode {
        pub fn from_code(code: u32) -> Self {
            match code {
                0 => CRegisterCode::cr0,
                1 => CRegisterCode::cr1,
                2 => CRegisterCode::cr2,
                3 => CRegisterCode::cr3,
                4 => CRegisterCode::cr4,
                5 => CRegisterCode::cr5,
                6 => CRegisterCode::cr6,
                7 => CRegisterCode::cr7,
                8 => CRegisterCode::cr8,