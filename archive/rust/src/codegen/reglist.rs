// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file emulates the C++ reglist.h file, which includes different architecture-specific
// register lists based on preprocessor directives.  In Rust, we'll use a configuration system
// or feature flags to achieve a similar effect.  For simplicity, this example provides a
// basic structure and uses a placeholder for the architecture-specific includes.
//
// Note: The exact implementation of `Register` and `DoubleRegister` and the macros
// `ALLOCATABLE_GENERAL_REGISTERS` and `ALLOCATABLE_DOUBLE_REGISTERS` will depend on the
// architecture-specific details and are not included here.  This example shows how the
// overall structure of the header file can be translated.

// Placeholder for architecture-specific reglist includes.
// In a real implementation, this would be conditionally compiled
// based on target architecture.  For example:
// #[cfg(target_arch = "x86")]
// mod reglist_x86;
// #[cfg(target_arch = "x86_64")]
// mod reglist_x64;
// ... and so on.

pub mod internal {
    // Placeholder types for Register and DoubleRegister
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Register {
        id: u32,
    }

    impl Register {
        pub const fn no_reg() -> Self {
            Register { id: 0 }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct DoubleRegister {
        id: u32,
    }

    impl DoubleRegister {
        pub const fn no_reg() -> Self {
            DoubleRegister { id: 0 }
        }
    }

    pub type RegList = [Register; 32]; // Assuming a maximum size for example purposes
    pub type DoubleRegList = [DoubleRegister; 32]; // Assuming a maximum size for example purposes

    pub const EMPTY_REG_LIST: RegList = [Register::no_reg(); 32];
    pub const EMPTY_DOUBLE_REG_LIST: DoubleRegList = [DoubleRegister::no_reg(); 32];

    // Placeholder macro implementations.  In a real implementation,
    // these would be architecture-specific and define the allocatable
    // registers.
    macro_rules! allocatable_general_registers {
        ($v:ident) => {
            [
                Register { id: 1 },
                Register { id: 2 },
                Register { id: 3 },
                Register { id: 4 },
                Register { id: 5 },
                Register { id: 6 },
                Register { id: 7 },
                Register { id: 8 },
                Register { id: 9 },
                Register { id: 10 },
                Register { id: 11 },
                Register { id: 12 },
                Register { id: 13 },
                Register { id: 14 },
                Register { id: 15 },
                Register { id: 16 },
                Register { id: 17 },
                Register { id: 18 },
                Register { id: 19 },
                Register { id: 20 },
                Register { id: 21 },
                Register { id: 22 },
                Register { id: 23 },
                Register { id: 24 },
                Register { id: 25 },
                Register { id: 26 },
                Register { id: 27 },
                Register { id: 28 },
                Register { id: 29 },
                Register { id: 30 },
                Register { id: 31 },
                Register::no_reg(),
            ]
        };
    }

    macro_rules! allocatable_double_registers {
        ($v:ident) => {
            [
                DoubleRegister { id: 1 },
                DoubleRegister { id: 2 },
                DoubleRegister { id: 3 },
                DoubleRegister { id: 4 },
                DoubleRegister { id: 5 },
                DoubleRegister { id: 6 },
                DoubleRegister { id: 7 },
                DoubleRegister { id: 8 },
                DoubleRegister { id: 9 },
                DoubleRegister { id: 10 },
                DoubleRegister { id: 11 },
                DoubleRegister { id: 12 },
                DoubleRegister { id: 13 },
                DoubleRegister { id: 14 },
                DoubleRegister { id: 15 },
                DoubleRegister { id: 16 },
                DoubleRegister { id: 17 },
                DoubleRegister { id: 18 },
                DoubleRegister { id: 19 },
                DoubleRegister { id: 20 },
                DoubleRegister { id: 21 },
                DoubleRegister { id: 22 },
                DoubleRegister { id: 23 },
                DoubleRegister { id: 24 },
                DoubleRegister { id: 25 },
                DoubleRegister { id: 26 },
                DoubleRegister { id: 27 },
                DoubleRegister { id: 28 },
                DoubleRegister { id: 29 },
                DoubleRegister { id: 30 },
                DoubleRegister { id: 31 },
                DoubleRegister::no_reg(),
            ]
        };
    }

    pub const ALLOCATABLE_GENERAL_REGISTERS: RegList = allocatable_general_registers!(v);
    pub const ALLOCATABLE_DOUBLE_REGISTERS: DoubleRegList = allocatable_double_registers!(v);
}