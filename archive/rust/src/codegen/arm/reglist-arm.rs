// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod reglist_arm {
    use crate::codegen::register_arch::Register;
    use crate::codegen::reglist_base::RegListBase;

    pub type RegList = RegListBase<Register>;
    pub type DoubleRegList = RegListBase<Register>; // Assuming DoubleRegister is also Register

    // Caller-saved/arguments registers
    pub const K_JS_CALLER_SAVED: RegList = RegList {
        bits: (1 << 0) | (1 << 1) | (1 << 2) | (1 << 3),
    };

    pub const K_NUM_JS_CALLER_SAVED: usize = 4;

    // Callee-saved registers preserved when switching from C to JavaScript
    pub const K_CALLEE_SAVED: RegList = RegList {
        bits: (1 << 4) | (1 << 5) | (1 << 6) | (1 << 7) | (1 << 8) | (1 << 9) | (1 << 10) | (1 << 11),
    };

    // When calling into C++ (only for C++ calls that can't cause a GC).
    // The call code will take care of lr, fp, etc.
    pub const K_CALLER_SAVED: RegList = RegList {
        bits: (1 << 0) | (1 << 1) | (1 << 2) | (1 << 3) | (1 << 9),
    };

    pub const K_NUM_CALLEE_SAVED: usize = 8;

    // Double registers d8 to d15 are callee-saved.
    pub const K_NUM_DOUBLE_CALLEE_SAVED: usize = 8;
}

pub mod register_arch {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct Register(pub usize);
}

pub mod reglist_base {
    use std::marker::Copy;

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct RegListBase<T: Copy> {
        pub bits: u32,
    }

    impl<T: Copy> RegListBase<T> {
        pub fn is_empty(&self) -> bool {
            self.bits == 0
        }
    }
}