// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod reglist_s390 {
    use crate::codegen::register_arch::Register;
    use crate::codegen::reglist_base::RegListBase;

    pub type RegList = RegListBase<Register>;
    pub type DoubleRegList = RegListBase<DoubleRegister>;

    // Assuming DoubleRegister is defined somewhere in the codebase
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DoubleRegister {
        pub code: u8,
    }

    impl DoubleRegister {
        pub const fn new(code: u8) -> Self {
            DoubleRegister { code }
        }
    }

    // Define constants for registers
    pub const R0: Register = Register { code: 0 };
    pub const R1: Register = Register { code: 1 };
    pub const R2: Register = Register { code: 2 };
    pub const R3: Register = Register { code: 3 };
    pub const R4: Register = Register { code: 4 };
    pub const R5: Register = Register { code: 5 };
    pub const R6: Register = Register { code: 6 };
    pub const R7: Register = Register { code: 7 };
    pub const R8: Register = Register { code: 8 };
    pub const R9: Register = Register { code: 9 };
    pub const R10: Register = Register { code: 10 };
    pub const FP: Register = Register { code: 11 }; // r11
    pub const IP: Register = Register { code: 12 }; // r12
    pub const R13: Register = Register { code: 13 };
    pub const R14: Register = Register { code: 14 };
    pub const SP: Register = Register { code: 15 }; // r15

    pub const D0: DoubleRegister = DoubleRegister { code: 0 };
    pub const D1: DoubleRegister = DoubleRegister { code: 1 };
    pub const D2: DoubleRegister = DoubleRegister { code: 2 };
    pub const D3: DoubleRegister = DoubleRegister { code: 3 };
    pub const D4: DoubleRegister = DoubleRegister { code: 4 };
    pub const D5: DoubleRegister = DoubleRegister { code: 5 };
    pub const D6: DoubleRegister = DoubleRegister { code: 6 };
    pub const D7: DoubleRegister = DoubleRegister { code: 7 };
    pub const D8: DoubleRegister = DoubleRegister { code: 8 };
    pub const D9: DoubleRegister = DoubleRegister { code: 9 };
    pub const D10: DoubleRegister = DoubleRegister { code: 10 };
    pub const D11: DoubleRegister = DoubleRegister { code: 11 };
    pub const D12: DoubleRegister = DoubleRegister { code: 12 };
    pub const D13: DoubleRegister = DoubleRegister { code: 13 };
    pub const D14: DoubleRegister = DoubleRegister { code: 14 };
    pub const D15: DoubleRegister = DoubleRegister { code: 15 };

    // Caller-saved/arguments registers
    pub const K_JS_CALLER_SAVED: RegList = RegList {
        registers: vec![R1, R2, R3, R4, R5],
    };

    pub const K_NUM_JS_CALLER_SAVED: usize = 5;

    // Callee-saved registers preserved when switching from C to JavaScript
    pub const K_CALLEE_SAVED: RegList = RegList {
        registers: vec![R6, R7, R8, R9, R10, FP, IP, R13],
    };

    pub const K_NUM_CALLEE_SAVED: usize = 8;

    pub const K_CALLER_SAVED_DOUBLES: DoubleRegList = DoubleRegList {
        registers: vec![D0, D1, D2, D3, D4, D5, D6, D7],
    };

    pub const K_NUM_CALLER_SAVED_DOUBLES: usize = 8;

    pub const K_CALLEE_SAVED_DOUBLES: DoubleRegList = DoubleRegList {
        registers: vec![D8, D9, D10, D11, D12, D13, D14, D15],
    };

    pub const K_NUM_CALLEE_SAVED_DOUBLES: usize = 8;
}

pub mod codegen {
    pub mod register_arch {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub struct Register {
            pub code: u8,
        }

        impl Register {
            pub const fn new(code: u8) -> Self {
                Register { code }
            }
        }
    }

    pub mod reglist_base {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct RegListBase<T> {
            pub registers: Vec<T>,
        }

        impl<T> RegListBase<T> {
            pub fn new(registers: Vec<T>) -> Self {
                RegListBase { registers }
            }
        }
    }
}