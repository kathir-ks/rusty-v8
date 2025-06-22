// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod reglist_ia32 {
    use crate::codegen::register_arch::Register;
    use crate::codegen::reglist_base::RegListBase;

    pub type RegList = RegListBase<Register>;
    pub type DoubleRegList = RegListBase<f64>; // Assuming DoubleRegister is f64

    // Caller-saved registers
    pub const K_JS_CALLER_SAVED: RegList = RegList {
        registers: [
            Register::Eax,
            Register::Ecx,
            Register::Edx,
            Register::Ebx, // used as caller-saved register in JavaScript code
            Register::Edi,
        ], // callee function
        len: 5,
    };

    // Caller-saved registers according to the x86 ABI
    pub const K_CALLER_SAVED: RegList = RegList {
        registers: [Register::Eax, Register::Ecx, Register::Edx],
        len: 3,
    };

    pub const K_NUM_JS_CALLER_SAVED: usize = 5;
}

pub mod codegen {
    pub mod register_arch {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum Register {
            Eax,
            Ecx,
            Edx,
            Ebx,
            Esp,
            Ebp,
            Esi,
            Edi,
        }
    }
    pub mod reglist_base {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct RegListBase<T: Copy> {
            pub registers: [T; 8], // Assuming a max of 8 registers. Adjust as necessary.
            pub len: usize,
        }

        impl<T: Copy> RegListBase<T> {
            pub const fn new(registers: [T; 8], len: usize) -> Self {
                Self { registers, len }
            }
        }
    }
}